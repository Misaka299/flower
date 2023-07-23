use windows::Win32::Graphics::Direct2D::Common::{D2D_POINT_2F, D2D_RECT_F};

use crate::graphics::{Point, Rect};
pub use crate::graphics::renderer::default::renderer::Dx2DRenderer as Renderer;

pub mod renderer;

impl Into<D2D_RECT_F> for Rect {
    fn into(self) -> D2D_RECT_F {
        D2D_RECT_F {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}

impl Into<D2D_POINT_2F> for Point {
    fn into(self) -> D2D_POINT_2F {
        D2D_POINT_2F{
            x: self.x,
            y: self.y,
        }
    }
}

// create_d2d_factory();
//
// let d2drtp = &D2D1_RENDER_TARGET_PROPERTIES {
//     r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
//     pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
//     dpiX: 96.0,
//     dpiY: 96.0,
//     usage: D2D1_RENDER_TARGET_USAGE_NONE,
//     minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
// } as *const D2D1_RENDER_TARGET_PROPERTIES;
//
// let d2d_factory = D2D1CreateFactory::<ID2D1Factory>(D2D1_FACTORY_TYPE::default(), None).unwrap();
// let dc = d2d_factory.CreateDCRenderTarget(d2drtp).unwrap();
// window.window().hwnd().abi();
// // d2d_factory.CreateHwndRenderTarget();
//
// let brush = dc.CreateSolidColorBrush(&D2D1_COLOR_F {
//     r: 1.0,
//     g: 0.0,
//     b: 0.0,
//     a: 1.0,
// } as *const D2D1_COLOR_F, None).unwrap();
//
// dc.BeginDraw();
//
// dc.Clear(Some(&D2D1_COLOR_F {
//     r: 1.0,
//     g: 1.0,
//     b: 1.0,
//     a: 1.0,
// } as *const D2D1_COLOR_F));
//
// dc.FillRectangle(&D2D_RECT_F {
//     left: 100.0,
//     top: 100.0,
//     right: 300.0,
//     bottom: 200.0,
// } as *const D2D_RECT_F, &brush);
// dc.EndDraw(None, None);