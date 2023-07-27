// use glutin::{ContextWrapper, PossiblyCurrent};
// use glutin::platform::windows::WindowExtWindows;
// use glutin::window::Window;
// use windows::core::Type;
// use windows::Win32::Foundation::HWND;
// use windows::Win32::Graphics::Direct2D::{D2D1_FACTORY_TYPE_MULTI_THREADED, D2D1_FEATURE_LEVEL_DEFAULT, D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_PRESENT_OPTIONS_NONE, D2D1_RENDER_TARGET_PROPERTIES, D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_USAGE_NONE, D2D1_STROKE_STYLE_PROPERTIES, D2D1CreateFactory, ID2D1Factory, ID2D1HwndRenderTarget, ID2D1StrokeStyle};
// use windows::Win32::Graphics::Direct2D::Common::{D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_RECT_F, D2D_SIZE_U};
// use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
//
// use crate::graphics::{Color, DashStyle, Point, Rect, Render};
//
// pub struct Dx2DRenderer {
//     render: ID2D1HwndRenderTarget,
// }
//
// impl Render for Dx2DRenderer {
//     fn create(context_wrapper: &ContextWrapper<PossiblyCurrent, Window>) -> Self {
//         unsafe {
//             let d2d_factory = D2D1CreateFactory::<ID2D1Factory>(D2D1_FACTORY_TYPE_MULTI_THREADED, None).expect("Failed to create Direct2D factory");
//             let hwnd = HWND(context_wrapper.window().hwnd().abi());
//             let size = context_wrapper.window().inner_size();
//             let render = d2d_factory.CreateHwndRenderTarget(&D2D1_RENDER_TARGET_PROPERTIES {
//                 r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
//                 pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
//                 dpiX: 96.0,
//                 dpiY: 96.0,
//                 usage: D2D1_RENDER_TARGET_USAGE_NONE,
//                 minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
//             }, &D2D1_HWND_RENDER_TARGET_PROPERTIES {
//                 hwnd,
//                 pixelSize: D2D_SIZE_U { width: size.width, height: size.width },
//                 presentOptions: D2D1_PRESENT_OPTIONS_NONE,
//             }).expect("Failed to create Direct2D render target");
//
//             Self { render }
//         }
//     }
//
//     fn begin_paint(&self) {
//         unsafe { self.render.BeginDraw(); }
//     }
//
//     fn end_paint(&self) {
//         unsafe { self.render.EndDraw(None, None).expect("Fail EndDraw."); }
//     }
//
//     fn draw_line(&self, pt1: Point, pt2: Point, pen: &Color, stroke_width: f32, stroke_style: DashStyle) {
//         unsafe {
//             let brush = self.render.CreateSolidColorBrush(&D2D1_COLOR_F {
//                 r: 1.0,
//                 g: 0.0,
//                 b: 0.0,
//                 a: 1.0,
//             }, None).expect("Failed to create Direct2D solid color");
//             let st = ID2D1StrokeStyle::from();
//             self.render.DrawLine(pt1.into(), pt2.into(), &brush, stroke_width, st);
//         }
//     }
//
//     fn update_window_size() {
//         todo!()
//     }
// }
//
// impl Dx2DRenderer {
//     pub fn test(&self) {
//         unsafe {
//             let brush = self.render.CreateSolidColorBrush(&D2D1_COLOR_F {
//                 r: 1.0,
//                 g: 0.0,
//                 b: 0.0,
//                 a: 1.0,
//             }, None).expect("Failed to create Direct2D solid color");
//             self.render.BeginDraw();
//             self.render.Clear(Some(&D2D1_COLOR_F {
//                 r: 1.0,
//                 g: 1.0,
//                 b: 1.0,
//                 a: 1.0,
//             }));
//             self.render.FillRectangle(&D2D_RECT_F {
//                 left: 100.0,
//                 top: 100.0,
//                 right: 300.0,
//                 bottom: 200.0,
//             }, &brush);
//
//             self.render.EndDraw(None, None);
//         }
//     }
// }