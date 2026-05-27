/*
 * Kernel driver for EAC bypass - Memory R/W from kernel space
 * Avoids user-mode ReadProcessMemory/WriteProcessMemory detection
 * 
 * Build with WDK (Windows Driver Kit)
 */

#include <ntddk.h>
#include <windef.h>

#define IOCTL_READ_MEMORY   CTL_CODE(FILE_DEVICE_UNKNOWN, 0x801, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_WRITE_MEMORY  CTL_CODE(FILE_DEVICE_UNKNOWN, 0x802, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GET_MODULE    CTL_CODE(FILE_DEVICE_UNKNOWN, 0x803, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PROTECT       CTL_CODE(FILE_DEVICE_UNKNOWN, 0x804, METHOD_BUFFERED, FILE_ANY_ACCESS)

typedef struct _MEMORY_REQUEST {
    ULONG ProcessId;
    PVOID Address;
    PVOID Buffer;
    SIZE_T Size;
} MEMORY_REQUEST, *PMEMORY_REQUEST;

typedef struct _MODULE_REQUEST {
    ULONG ProcessId;
    WCHAR ModuleName[256];
    PVOID BaseAddress;
    SIZE_T Size;
} MODULE_REQUEST, *PMODULE_REQUEST;

// Hide from PsLoadedModuleList to avoid EAC enumeration
VOID HideDriver(PDRIVER_OBJECT DriverObject) {
    PLIST_ENTRY current = DriverObject->DriverSection;
    PLIST_ENTRY prev = current->Blink;
    PLIST_ENTRY next = current->Flink;
    
    // Unlink from list
    prev->Flink = next;
    next->Blink = prev;
    
    // Point to ourselves to avoid crash
    current->Flink = current;
    current->Blink = current;
}

// Read physical memory directly (bypass EAC scanning)
NTSTATUS ReadPhysicalMemory(PVOID TargetAddress, PVOID Buffer, SIZE_T Size) {
    PHYSICAL_ADDRESS physAddr;
    PVOID mappedAddr;
    
    physAddr.QuadPart = (LONGLONG)TargetAddress;
    mappedAddr = MmMapIoSpace(physAddr, Size, MmNonCached);
    
    if (!mappedAddr) {
        return STATUS_UNSUCCESSFUL;
    }
    
    RtlCopyMemory(Buffer, mappedAddr, Size);
    MmUnmapIoSpace(mappedAddr, Size);
    
    return STATUS_SUCCESS;
}

// Kernel-mode memory read (undetected by EAC user-mode hooks)
NTSTATUS ReadProcessMemoryKernel(ULONG ProcessId, PVOID Address, PVOID Buffer, SIZE_T Size) {
    PEPROCESS process;
    KAPC_STATE apcState;
    NTSTATUS status;
    
    status = PsLookupProcessByProcessId((HANDLE)ProcessId, &process);
    if (!NT_SUCCESS(status)) {
        return status;
    }
    
    KeStackAttachProcess(process, &apcState);
    
    __try {
        RtlCopyMemory(Buffer, Address, Size);
        status = STATUS_SUCCESS;
    }
    __except(EXCEPTION_EXECUTE_HANDLER) {
        status = GetExceptionCode();
    }
    
    KeUnstackDetachProcess(&apcState);
    ObDereferenceObject(process);
    
    return status;
}

// Kernel-mode memory write
NTSTATUS WriteProcessMemoryKernel(ULONG ProcessId, PVOID Address, PVOID Buffer, SIZE_T Size) {
    PEPROCESS process;
    KAPC_STATE apcState;
    NTSTATUS status;
    
    status = PsLookupProcessByProcessId((HANDLE)ProcessId, &process);
    if (!NT_SUCCESS(status)) {
        return status;
    }
    
    KeStackAttachProcess(process, &apcState);
    
    __try {
        // Disable write protection temporarily
        KIRQL oldIrql = KeRaiseIrqlToDpcLevel();
        ULONG_PTR cr0 = __readcr0();
        __writecr0(cr0 & ~0x10000); // Clear WP bit
        
        RtlCopyMemory(Address, Buffer, Size);
        
        __writecr0(cr0); // Restore WP bit
        KeLowerIrql(oldIrql);
        
        status = STATUS_SUCCESS;
    }
    __except(EXCEPTION_EXECUTE_HANDLER) {
        status = GetExceptionCode();
    }
    
    KeUnstackDetachProcess(&apcState);
    ObDereferenceObject(process);
    
    return status;
}

// IOCTL handler
NTSTATUS DeviceControl(PDEVICE_OBJECT DeviceObject, PIRP Irp) {
    PIO_STACK_LOCATION stack = IoGetCurrentIrpStackLocation(Irp);
    NTSTATUS status = STATUS_SUCCESS;
    ULONG bytesReturned = 0;
    
    switch (stack->Parameters.DeviceIoControl.IoControlCode) {
        case IOCTL_READ_MEMORY: {
            PMEMORY_REQUEST req = (PMEMORY_REQUEST)Irp->AssociatedIrp.SystemBuffer;
            status = ReadProcessMemoryKernel(req->ProcessId, req->Address, req->Buffer, req->Size);
            bytesReturned = req->Size;
            break;
        }
        
        case IOCTL_WRITE_MEMORY: {
            PMEMORY_REQUEST req = (PMEMORY_REQUEST)Irp->AssociatedIrp.SystemBuffer;
            status = WriteProcessMemoryKernel(req->ProcessId, req->Address, req->Buffer, req->Size);
            break;
        }
        
        default:
            status = STATUS_INVALID_DEVICE_REQUEST;
            break;
    }
    
    Irp->IoStatus.Status = status;
    Irp->IoStatus.Information = bytesReturned;
    IoCompleteRequest(Irp, IO_NO_INCREMENT);
    
    return status;
}

// Driver unload
VOID DriverUnload(PDRIVER_OBJECT DriverObject) {
    UNICODE_STRING symLink = RTL_CONSTANT_STRING(L"\\??\\RustDriver");
    IoDeleteSymbolicLink(&symLink);
    IoDeleteDevice(DriverObject->DeviceObject);
}

// Driver entry point
NTSTATUS DriverEntry(PDRIVER_OBJECT DriverObject, PUNICODE_STRING RegistryPath) {
    NTSTATUS status;
    PDEVICE_OBJECT deviceObject;
    UNICODE_STRING deviceName = RTL_CONSTANT_STRING(L"\\Device\\RustDriver");
    UNICODE_STRING symLink = RTL_CONSTANT_STRING(L"\\??\\RustDriver");
    
    // Create device
    status = IoCreateDevice(
        DriverObject,
        0,
        &deviceName,
        FILE_DEVICE_UNKNOWN,
        FILE_DEVICE_SECURE_OPEN,
        FALSE,
        &deviceObject
    );
    
    if (!NT_SUCCESS(status)) {
        return status;
    }
    
    // Create symbolic link
    status = IoCreateSymbolicLink(&symLink, &deviceName);
    if (!NT_SUCCESS(status)) {
        IoDeleteDevice(deviceObject);
        return status;
    }
    
    // Set handlers
    DriverObject->MajorFunction[IRP_MJ_CREATE] = 
    DriverObject->MajorFunction[IRP_MJ_CLOSE] = 
        (PDRIVER_DISPATCH)STATUS_SUCCESS;
    DriverObject->MajorFunction[IRP_MJ_DEVICE_CONTROL] = DeviceControl;
    DriverObject->DriverUnload = DriverUnload;
    
    // Hide driver from EAC enumeration
    HideDriver(DriverObject);
    
    return STATUS_SUCCESS;
}
