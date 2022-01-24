use winapi::shared::ntdef::NULL;
use winapi::um::winuser::{DispatchMessageW, MSG, PeekMessageW, PM_REMOVE, TranslateMessage, WM_QUIT};

pub mod window;
pub mod event;
pub mod app;

pub fn run() {
    'proc: loop {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg as *mut _ as _, NULL as _, 0, 0, PM_REMOVE) != 0 {
                if WM_QUIT == msg.message {
                    break 'proc;
                } else {
                    TranslateMessage(&mut msg as *mut _ as _);
                    DispatchMessageW(&mut msg as *mut _ as _);
                }
            }
        }
    }
}