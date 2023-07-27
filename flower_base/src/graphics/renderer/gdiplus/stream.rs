use std::ptr::null_mut;

use gdiplus_sys2::{GpImage, GpStatus};
use winapi::shared::minwindef::{FALSE, HGLOBAL};
use winapi::um::combaseapi::CreateStreamOnHGlobal;
use winapi::um::objidlbase::IStream;
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};

extern "C" {
    #[link_name = "\u{1}GdipLoadImageFromStream"]
    pub fn GdipLoadImageFromStream(stream: *mut IStream, image: *mut *mut GpImage) -> GpStatus;
}

pub struct Stream {
    hglobal: HGLOBAL,
    pub stream: *mut IStream,
}

impl Stream {
    pub fn create_from_u8(data: &[u8]) -> Stream {
        unsafe {
            let mut stream = null_mut();
            let hglobal = GlobalAlloc(GMEM_MOVEABLE, data.len());
            let buffer = GlobalLock(hglobal) as *mut u8;
            buffer.copy_from_nonoverlapping(data.as_ptr(), data.len());

            let i = CreateStreamOnHGlobal(hglobal, FALSE, &mut stream);
            println!("CreateStreamOnHGlobal - {} stream - {:?}", i, stream);
            GlobalUnlock(hglobal);
            Self {
                hglobal,
                stream,
            }
        }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        unsafe {
            GlobalFree(self.hglobal);
        }
    }
}