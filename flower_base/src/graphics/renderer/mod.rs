use crate::graphics::Render;
use crate::graphics::renderer::gdiplus::GdiPlusRenderer;

pub mod gdiplus;

pub type Renderer = GdiPlusRenderer;

// pub enum RenderingEngine {
//     GdiPlus
// }
//
// pub struct Renderer<T: Render> {
//     engine: T,
// }
//
// impl<T: Render> Renderer<T> {
//     pub fn create_renderer(window_id: i32, rendering_engine: RenderingEngine) -> Self <T> {
//         let engine = match rendering_engine {
//             RenderingEngine::GdiPlus => {
//                 gdiplus::GdiPlusRenderer::create(window_id)
//             }
//         };
//         Self {
//             engine
//         }
//     }
// }
//
// impl<T: Render> Render for Renderer<T> {
//     fn create(window_id: i32) -> Self {
//         todo!()
//     }
//
//     #[inline]
//     fn get_window_id(&self) -> i32 {
//         self.engine.get_window_id()
//     }
//
//     #[inline]
//     fn begin_paint(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>) {
//         self.engine.begin_paint(window_context);
//     }
//
//     #[inline]
//     fn end_paint(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>) {
//         self.engine.end_paint(window_context);
//     }
//
//     #[inline]
//     fn new_canvas_buffer(&mut self, id: i32, width: i32, height: i32) {
//         self.engine.new_canvas_buffer(id, width, height);
//     }
//
//     #[inline]
//     fn refresh_to_buffer(&mut self, source_id: i32, target_id: i32, x: i32, y: i32) {
//         self.refresh_to_buffer(source_id, target_id, x, y);
//     }
//
//     #[inline]
//     fn refresh_canvas_to_window(&mut self) {
//         self.engine.refresh_canvas_to_window();
//     }
//
//     #[inline]
//     fn delete_canvas_buffer(&mut self, id: i32) {
//         self.engine.delete_canvas_buffer(id);
//     }
//
//     #[inline]
//     fn store(&mut self, rect: &Rect, pen: &Pen) {
//         self.engine.store(rect, pen);
//     }
//
//     #[inline]
//     fn draw_line(&self, pt1: Point, pt2: Point, color: &Color, stroke_width: f32, stroke_style: DashStyle) {
//         self.engine.draw_line(pt1, pt2, color, stroke_width, stroke_style);
//     }
//
//     #[inline]
//     fn draw_image(&mut self, image: Vec<u8>, rect_location: Rect) {
//         self.engine.draw_image(image, rect_location)
//     }
//
//     #[inline]
//     fn measure_text(&mut self, font: &Font, str: &impl AsRef<str>) -> Rect {
//         self.engine.measure_text(font, str)
//     }
//
//     #[inline]
//     fn draw_text_rect(&mut self, rect: &Rect, font: &Font, color: &Color, str: &impl AsRef<str>) {
//         self.engine.draw_text_rect(rect, font, color, str);
//     }
//
//     #[inline]
//     fn update_window_size() {
//         todo!()
//     }
// }