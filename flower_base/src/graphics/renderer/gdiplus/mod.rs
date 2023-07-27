use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr::null_mut;

use gdiplus_sys2::{ARGB, Color_Blue, GdipCreateBitmapFromScan0, GdipCreateFont, GdipCreateFontFamilyFromName, GdipCreateFromHWND, GdipCreatePen1, GdipCreateStringFormat, GdipDeleteGraphics, GdipDeletePen, GdipDrawImageI, GdipDrawImageRectI, GdipDrawRectangleI, GdipDrawString, GdipGetImageGraphicsContext, GdipGetPenBrushFill, GdiplusStartup, GdiplusStartupInput, GdiplusStartupOutput, GdipMeasureString, GdipSetTextRenderingHint, GpFont, GpFontFamily, GpGraphics, GpImage, GpPen, HWND, RectF, TextRenderingHint_TextRenderingHintAntiAliasGridFit, Unit_UnitPixel};
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::platform::windows::WindowExtWindows;
use glutin::window::Window;
use rustc_hash::FxHashMap;
use winapi::shared::ntdef::INT;

use crate::graphics::{DashStyle, Point, Render};
use crate::graphics::color::Color;
use crate::graphics::font::Font;
use crate::graphics::pen::Pen;
use crate::graphics::rect::Rect;
pub use crate::graphics::renderer::default::GdiPlusRenderer as Renderer;
use crate::graphics::renderer::gdiplus::stream::{GdipLoadImageFromStream, Stream};

pub mod stream;

#[macro_export]
macro_rules! wchar {
    ($str:expr) => {
        OsStr::new($str).encode_wide().chain(once(0)).collect::<Vec<u16>>().as_ptr()
    };
}

enum FontStyle {
    FontStyleRegular = 0,
    FontStyleBold = 1,
    FontStyleItalic = 2,
    FontStyleBoldItalic = 3,
    FontStyleUnderline = 4,
    FontStyleStrikeout = 8,
}

enum PixelFormat {
    Undefined = 0,
    Format16bppArgb1555 = 397319,
    Format1bppIndexed = 196865,
    Format4bppIndexed = 197634,
    Format8bppIndexed = 198659,
    Format16bppGrayScale = 1052676,
    Format24bppRgb = 137224,
    Format32bppRgb = 139273,
    Format32bppArgb = 2498570,
    Format32bppPArgb = 925707,
    Format48bppRgb = 1060876,
    Format64bppArgb = 3424269,
    Format64bppPArgb = 1851406,
    Indexed = 65536,
    Gdi = 131072,
    Alpha = 262144,
    PAlpha = 524288,
    Extended = 1048576,
    Canonical = 2097152,
    Max = 15,
}

pub struct GdiPlusRenderer {
    window_graphics: *mut GpGraphics,
    canvas_id: Option<i32>,
    canvas_graphics: FxHashMap<i32, *mut GpGraphics>,
    canvas_image: FxHashMap<i32, *mut GpImage>,
    hash_image: FxHashMap<u64, *mut GpImage>,
    pen: FxHashMap<Font, *mut GpPen>,
    font: FxHashMap<Font, *mut GpFont>,
    font_family: FxHashMap<Font, *mut GpFontFamily>,
}

impl Render for GdiPlusRenderer {
    fn create() -> Self {
        Self {
            window_graphics: null_mut(),
            canvas_id: None,
            canvas_graphics: Default::default(),
            canvas_image: Default::default(),
            hash_image: Default::default(),
            pen: Default::default(),
            font: Default::default(),
            font_family: Default::default(),
        }
    }

    fn begin_paint(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>) {
        unsafe {
            self.window_graphics = null_mut();
            let mut token = 0;
            GdiplusStartup(&mut token, &mut GdiplusStartupInput {
                GdiplusVersion: 1,
                DebugEventCallback: None,
                SuppressBackgroundThread: 0,
                SuppressExternalCodecs: 0,
            }, &mut GdiplusStartupOutput { NotificationHook: None, NotificationUnhook: None });
            GdipCreateFromHWND(window_context.window().hwnd() as HWND, &mut self.window_graphics);
        }
    }

    fn end_paint(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>) {
        unsafe {
            GdipDeleteGraphics(self.window_graphics);
        }
    }

    fn new_buffer_canvas(&mut self, id: i32, width: i32, height: i32) {
        unsafe {
            let mut bitmap = null_mut();
            GdipCreateBitmapFromScan0(width, height, 0, PixelFormat::Format32bppArgb as i32, null_mut(), &mut bitmap);
            let mut graphics = null_mut();
            GdipGetImageGraphicsContext(bitmap as *mut GpImage, &mut graphics);

            self.canvas_graphics.insert(id, graphics);
            self.canvas_image.insert(id, bitmap as *mut GpImage);
            self.canvas_id = Some(id);
        }
    }

    fn refresh_canvas_to_window(&mut self, id: Option<i32>, x: i32, y: i32) {
        if let Some(id) = id {
            if let Some(image) = self.canvas_image.get(&id) {
                unsafe {
                    let a = GdipDrawImageI(self.window_graphics, *image, x, y);
                }
            }
            return;
        }
        if let Some(id) = self.canvas_id {
            if let Some(image) = self.canvas_image.get(&id) {
                // unsafe {
                //     let mut clsid_encoder = CLSID {
                //         Data1: 0x557cf406,
                //         Data2: 0x1a04,
                //         Data3: 0x11d3,
                //         Data4: [0x9a, 0x73, 0x00, 0x60, 0x97, 0xa0, 0x11, 0xa8],
                //     }; // CLSID for BMP format
                //     GdipSaveImageToFile(*image, wchar!("output.bmp"), &mut clsid_encoder, null_mut());
                // }
                unsafe {
                    GdipDrawImageI(self.window_graphics, *image, x, y);
                }
            }
            return;
        }
    }

    fn store(&mut self, rect: &Rect, pen: &Pen) {
        unsafe {
            // 填充颜色/渐变/图片

            //width,color
            let mut gp_pen = null_mut();
            GdipCreatePen1(pen.color.into(), pen.width, 2, &mut gp_pen);

            GdipDrawRectangleI(self.get_graphics(), gp_pen, rect.left as i32, rect.top as i32, rect.width as i32, rect.height as i32);

            GdipDeletePen(gp_pen);
        }
    }


    fn draw_line(&self, pt1: Point, pt2: Point, color: &Color, stroke_width: f32, stroke_style: DashStyle) {
        todo!()
    }

    fn draw_image(&mut self, image: Vec<u8>, rl: Rect) {
        unsafe {
            let gp_image = self.get_gp_image(image);
            GdipDrawImageRectI(self.window_graphics, gp_image, rl.left as INT, rl.top as INT, rl.width as INT, rl.height as INT);
        }
    }

    fn measure_text(&self, font: &Font, str: &impl AsRef<str>) -> Rect {
        unsafe {
            let font = self.get_font(font);

            let mut format = null_mut();
            GdipCreateStringFormat(0, 0, &mut format);

            let mut rect_f = RectF {
                X: 0.0,
                Y: 0.0,
                Width: 0.0,
                Height: 0.0,
            };
            // 必传字段
            let mut layout_rect_f = RectF {
                X: 0.0,
                Y: 0.0,
                Width: 0.0,
                Height: 0.0,
            };
            GdipMeasureString(self.get_graphics(), wchar!(str.as_ref()), -1, font, &mut layout_rect_f, format, &mut rect_f, null_mut(), null_mut());
            rect_f.into()
        }
    }

    fn draw_text_rect(&self, rect: &Rect, font: &Font, color: &Color, str: &impl AsRef<str>) {
        unsafe {
            let mut format = null_mut();
            GdipCreateStringFormat(0, 0, &mut format);
            let mut pen = null_mut();
            let s = (*color).into();
            GdipCreatePen1(s, 1.0, Unit_UnitPixel, &mut pen);
            let mut brush = null_mut();
            GdipGetPenBrushFill(pen, &mut brush);
            GdipSetTextRenderingHint(self.get_graphics(), TextRenderingHint_TextRenderingHintAntiAliasGridFit);
            GdipDrawString(self.get_graphics(), wchar!(str.as_ref()), -1, self.get_font(font), &(*rect).into(), format, brush);
        }
    }

    fn update_window_size() {
        todo!()
    }
}

impl GdiPlusRenderer {
    fn get_graphics(&self) -> *mut GpGraphics {
        if let Some(id) = self.canvas_id {
            if let Some(graphics) = self.canvas_graphics.get(&id) {
                return *graphics;
            }
        }
        self.window_graphics
    }

    fn get_font(&self, font: &Font) -> *const GpFont {
        match self.font.get(font) {
            None => {
                unsafe {
                    let mut ptr = null_mut();
                    let i = GdipCreateFont(self.get_font_family(font), 12., 0, Unit_UnitPixel, &mut ptr);
                    ptr
                }
            }
            Some(v) => { *v }
        }
    }

    fn get_font_family(&self, font: &Font) -> *const GpFontFamily {
        match self.font_family.get(font) {
            None => {
                unsafe {
                    let mut ptr = null_mut();
                    let i = GdipCreateFontFamilyFromName(wchar!(font.name.as_str()), null_mut(), &mut ptr);
                    ptr
                }
            }
            Some(v) => { *v }
        }
    }

    fn get_gp_image(&mut self, image: Vec<u8>) -> *mut GpImage {
        let mut hasher = DefaultHasher::new();
        image.hash(&mut hasher);
        let hash = hasher.finish();
        println!("hash {}", hash);
        *self.hash_image.entry(hash).or_insert_with(|| unsafe {
            println!("insert");
            let mut stream = Stream::create_from_u8(image.as_slice());
            let mut gp_image = null_mut();
            GdipLoadImageFromStream(stream.stream, &mut gp_image);
            gp_image
        })
    }
}

impl GdiPlusRenderer {
    pub fn test(&self) {
        unsafe {
            let mut pen = null_mut();
            GdipCreatePen1(Color_Blue as ARGB, 1.0, 2, &mut pen);

            GdipDrawRectangleI(self.get_graphics(), pen, 100, 100, 300, 200);

            GdipDeletePen(pen);
        }
    }
}