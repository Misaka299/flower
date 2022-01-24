use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::{HDC, HWND, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{AdjustWindowRectEx, CreateWindowExW, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, DefWindowProcW, GetDC, GetSystemMetrics, IDC_ARROW, IDI_WINLOGO, LoadCursorW, LoadIconW, PostQuitMessage, RegisterClassW, ShowWindow, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WM_DESTROY, WM_LBUTTONDOWN, WNDCLASSW, WS_CAPTION, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_APPWINDOW, WS_EX_WINDOWEDGE, WS_MINIMIZEBOX, WS_POPUP, WS_SYSMENU, WS_VISIBLE};

use crate::event::Event;

pub static mut HWND_WINDOWS_MAP: Lazy<FxHashMap<HWND, Rc<RefCell<NativeWindow>>>> = Lazy::new(|| FxHashMap::default());

#[derive(Clone, Debug)]
pub struct NativeWindowSetting {
    window_title: String,
    width: i32,
    height: i32,
    scale: f32,
}

impl NativeWindowSetting {
    pub fn build() -> Self { NativeWindowSetting::default() }
    pub fn window_title(mut self, window_title: String) -> Self {
        self.window_title = window_title;
        self
    }
    pub fn width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for NativeWindowSetting {
    fn default() -> Self {
        NativeWindowSetting {
            window_title: "flower ui".to_string(),
            width: 800,
            height: 400,
            scale: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct NativeWindow {
    hwnd: HWND,
    dc: HDC,
    fullscreen: bool,
    width: i32,
    height: i32,
    scale: f32,
    native_event_proc: Option<Rc<Box<dyn Fn(Event)>>>,
}

impl Debug for NativeWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, )", self.hwnd as i32, self.dc as i32, self.fullscreen, self.width, self.height, self.scale)
    }
}

impl NativeWindow {
    pub fn css(&self) {
        println!("sss")
    }
}

impl NativeWindow {
    pub fn create(setting: NativeWindowSetting) -> Rc<RefCell<NativeWindow>> {
        unsafe {
            let rc_ref_window = Rc::new(RefCell::new(NativeWindow {
                hwnd: std::ptr::null_mut(),
                dc: std::ptr::null_mut(),
                fullscreen: false,
                width: setting.width,
                height: setting.height,
                scale: setting.scale,
                native_event_proc: None,
            }));
            let mut window = rc_ref_window.borrow_mut();
            let mut wnd_class_w: WNDCLASSW = std::mem::zeroed();

            wnd_class_w.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
            wnd_class_w.lpfnWndProc = Some(win32_wndproc);
            wnd_class_w.hInstance = GetModuleHandleW(NULL as _);
            wnd_class_w.hCursor = LoadCursorW(NULL as _, IDC_ARROW);
            wnd_class_w.hIcon = LoadIconW(NULL as _, IDI_WINLOGO);
            let class_name = "FLOWER_UI\0".encode_utf16().collect::<Vec<u16>>();
            wnd_class_w.lpszClassName = class_name.as_ptr() as _;
            RegisterClassW(&wnd_class_w);

            let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };

            let win_style: DWORD = get_win_style();
            if window.fullscreen {
                rect.right = GetSystemMetrics(SM_CXSCREEN);
                rect.bottom = GetSystemMetrics(SM_CYSCREEN);
            } else {
                rect.right = (window.width as f32 * window.scale) as _;
                rect.bottom = (window.height as f32 * window.scale) as _;
            }

            AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
            let win_width = rect.right - rect.left;
            let win_height = rect.bottom - rect.top;

            let class_name = "FLOWER_UI\0".encode_utf16().collect::<Vec<u16>>();
            let mut window_name = setting.window_title.encode_utf16().collect::<Vec<u16>>();
            window_name.push(0);
            window.hwnd = CreateWindowExW(
                win_ex_style,                // dwExStyle
                class_name.as_ptr(),         // lpClassName
                window_name.as_ptr(),        // lpWindowName
                win_style,                   // dwStyle
                CW_USEDEFAULT,               // X
                CW_USEDEFAULT,               // Y
                win_width,                   // nWidth
                win_height,                  // nHeight
                NULL as _,                   // hWndParent
                NULL as _,                   // hMenu
                GetModuleHandleW(NULL as _), // hInstance
                NULL as _,                   // lParam
            );


            assert!(window.hwnd.is_null() == false);
            HWND_WINDOWS_MAP.insert(window.hwnd.clone(), Rc::clone(&rc_ref_window));
            ShowWindow(window.hwnd, SW_SHOW);
            window.dc = GetDC(window.hwnd);
            assert!(window.dc.is_null() == false);
            // update_dimensions();

            // 自定义事件
            // PROC_MAP.insert(window.hwnd,  Box::as_ref(&window.proc_event));

            // Box::new(window)
            Rc::clone(&rc_ref_window)
        }
    }

    pub fn event_proc(&mut self, proc: Box<dyn Fn(Event)>) {
        proc(Event::LButtonUp);
        self.native_event_proc = Some(Rc::new(proc));
        // unsafe { HWND_WINDOWS_MAP.insert(self.hwnd, self.clone()); }
    }
}

unsafe extern "system" fn win32_wndproc(
    h_wnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let window = match HWND_WINDOWS_MAP.get(&h_wnd) {
        None => {
            println!("proc get windows is none");
            return DefWindowProcW(h_wnd, u_msg, w_param, l_param);
        }
        Some(win) => { Rc::clone(win) }
    };
    let proc = match window.try_borrow_mut() {
        Ok(win) => {
            match win.native_event_proc.as_ref() {
                None => {
                    println!("win {} proc is none.", h_wnd as i32);
                    return DefWindowProcW(h_wnd, u_msg, w_param, l_param);
                }
                Some(proc) => { Rc::clone(proc) }
            }
        }
        Err(_) => {
            println!("borrow fail.");
            return DefWindowProcW(h_wnd, u_msg, w_param, l_param);
        }
    };
    match u_msg {
        WM_LBUTTONDOWN => {
            let x_pos = GET_X_LPARAM(l_param);
            let y_pos = GET_Y_LPARAM(l_param);
            proc(Event::LButtonDown(x_pos, y_pos));
            println!("proc get WM_LBUTTONDOWN call {} event_proc", h_wnd as i32);
        }
        // todo()
        WM_DESTROY => {
            println!("quit {}", u_msg);
            PostQuitMessage(0);
            return 0;
        }
        _ => {
            // println!("other msg: {u_msg}")
        }
    }
    DefWindowProcW(h_wnd, u_msg, w_param, l_param)
}


// match u_msg {
//     WM_LBUTTONDOWN => {}
//     WM_RBUTTONDOWN => {}
//     WM_MBUTTONDOWN => {}
//     WM_LBUTTONUP => {}
//     WM_RBUTTONUP => {}
//     WM_MBUTTONUP => {}
//     _ => {
//         PROC_MAP.get(&h_wnd).unwrap()();
//     }
// }

unsafe fn get_win_style() -> DWORD {
    if false {
        WS_POPUP | WS_SYSMENU | WS_VISIBLE
    } else {
        let win_style: DWORD =
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

        // if _sapp.desc.window_resizable {
        //     win_style |= WS_MAXIMIZEBOX | WS_SIZEBOX;
        // }

        win_style
    }
}