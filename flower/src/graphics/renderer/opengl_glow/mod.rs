pub mod shape;
pub mod enums;
pub mod color;
pub mod text;
pub mod draw;
pub mod pixel_tool;
pub mod render;
pub mod fragment;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
//
// pub fn new(context: impl Fn() -> Context, scene_size: (u32, u32)) -> Self {
//     let context = context();
//     unsafe {
//         print!("------{:?}", context.get_parameter_string(glow::VERSION));
//         // todo
//         context.enable(glow::MULTISAMPLE);
//     }
//     Draw {
//         render: Renderer::new(context, scene_size)
//     }
// }