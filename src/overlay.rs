// ESP overlay using transparent window (basic framework)
use winapi::um::winuser::*;
use winapi::um::wingdi::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use std::ptr;

pub struct Overlay {
    hwnd: HWND,
    target_hwnd: HWND,
}

impl Overlay {
    pub fn new(target_window_title: &str) -> Option<Self> {
        unsafe {
            // Find target window
            let title = std::ffi::CString::new(target_window_title).ok()?;
            let target_hwnd = FindWindowA(ptr::null(), title.as_ptr());
            
            if target_hwnd.is_null() {
                return None;
            }

            // Register window class
            let class_name = std::ffi::CString::new("OverlayClass").unwrap();
            
            let wc = WNDCLASSA {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: ptr::null_mut(),
                hIcon: ptr::null_mut(),
                hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };

            RegisterClassA(&wc);

            // Get target window rect
            let mut rect: RECT = std::mem::zeroed();
            GetWindowRect(target_hwnd, &mut rect);

            // Create overlay window
            let hwnd = CreateWindowExA(
                WS_EX_TOPMOST | WS_EX_TRANSPARENT | WS_EX_LAYERED,
                class_name.as_ptr(),
                std::ffi::CString::new("Overlay").unwrap().as_ptr(),
                WS_POPUP,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if hwnd.is_null() {
                return None;
            }

            // Make window transparent
            SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA);
            
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);

            Some(Overlay { hwnd, target_hwnd })
        }
    }

    pub fn update(&self) {
        unsafe {
            // Match target window position
            let mut rect: RECT = std::mem::zeroed();
            GetWindowRect(self.target_hwnd, &mut rect);
            
            SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                SWP_NOACTIVATE,
            );
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        unsafe {
            let hdc = GetDC(self.hwnd);
            let pen = CreatePen(PS_SOLID, 2, color);
            let old_pen = SelectObject(hdc, pen as *mut _);
            
            MoveToEx(hdc, x1, y1, ptr::null_mut());
            LineTo(hdc, x2, y2);
            
            SelectObject(hdc, old_pen);
            DeleteObject(pen as *mut _);
            ReleaseDC(self.hwnd, hdc);
        }
    }

    pub fn draw_rect(&self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        unsafe {
            let hdc = GetDC(self.hwnd);
            let pen = CreatePen(PS_SOLID, 2, color);
            let old_pen = SelectObject(hdc, pen as *mut _);
            let old_brush = SelectObject(hdc, GetStockObject(NULL_BRUSH as i32));
            
            Rectangle(hdc, x, y, x + w, y + h);
            
            SelectObject(hdc, old_pen);
            SelectObject(hdc, old_brush);
            DeleteObject(pen as *mut _);
            ReleaseDC(self.hwnd, hdc);
        }
    }

    pub fn draw_text(&self, x: i32, y: i32, text: &str, color: u32) {
        unsafe {
            let hdc = GetDC(self.hwnd);
            let c_text = std::ffi::CString::new(text).unwrap();
            
            SetTextColor(hdc, color);
            SetBkMode(hdc, TRANSPARENT as i32);
            TextOutA(hdc, x, y, c_text.as_ptr(), text.len() as i32);
            
            ReleaseDC(self.hwnd, hdc);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            EndPaint(hwnd, &ps);
            0
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}
