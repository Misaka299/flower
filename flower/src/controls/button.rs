use std::ops::{Deref, DerefMut};

use glow::{HasContext, LINE_LOOP, LINE_STRIP, LINES};
use image::imageops::FilterType;

use crate::control::{Control, ControlState, ControlType, InteractiveState};
use crate::render::color::Color;
use crate::render::fragment::FragData;
use crate::render::image::{Align, Image, ZoomType};
use crate::render::render::Renderer;
use crate::render::shape::{DrawMode, Shape};

pub struct Button {
    control_state: ControlState,
    text: String,
    on_click: Option<Box<dyn Fn()>>,
    shape: Shape,
}

impl Button {
    pub fn from(name: String, text: String) -> Button {
        let state = ControlState::create(name, false, ControlType::Control);
        let shape = Shape::rect(state.left, state.top, state.width, state.height);
        Button {
            control_state: state,
            text,
            on_click: None,
            shape,
        }
    }
    pub fn on_click(&mut self, fn_on_click: Box<dyn Fn()>) -> &mut Self {
        self.on_click = Some(fn_on_click);
        self
    }
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Deref for Button {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.control_state
    }
}

impl Control for Button {
    fn on_draw(&mut self, gl: &mut Renderer) {
        println!("button[{}] draw rect {:?}", self.id(), &self.rect);

        unsafe {
            // let verts_mat4x2 = mat2x4(
            //     1.0f32, 1.0f32,
            //     1.0f32, -1.0f32,
            //     -1.0f32, -1.0f32,
            //     -1.0f32, 1.0f32,
            // );

            // let a_position = gl.get_attrib_location(gl.shader.unwrap(), "verts");
            //
            // gl.enable_vertex_attrib_array(a_position.unwrap());
            // gl.vertex_attrib_pointer_f32(a_position.unwrap(), 3, FLOAT, false, 0, 32 * 4);
            // gl.disable_vertex_attrib_array(a_position.unwrap());


            // gl.use_def_program();


            // gl.Circle(ShapeCoord::from_rect());

            // gl::UniformMatrix4fv(transformLoc, 1, gl::FALSE, transform.as_ptr());
            // gl.rect(&Rect::new(self.base_left + self.left, self.base_top + self.top, self.width, self.height),Option::None);
            // gl.uniform_4_f32(gl.get_uniform_location(gl.shader.unwrap(), "transform").as_ref(), matrix[0][0], matrix[0][1], matrix[0][2], matrix[0][3]);
            // gl.draw_arrays(glow::LINE_LOOP, 0, 4);
            // let verts = gl.get_uniform_location(gl.shader.unwrap(), "verts");
            //     gl.uniform_4(verts.as_ref(),);

            // gl.vertex_attrib_1_f32()
        }


        unsafe {
            // gl.clear_color(1.0, 1.0, 1.0, 1.0);
            // gl.clear(glow::COLOR_BUFFER_BIT);


            // let texture = gl.create_texture().unwrap();
            // gl.bind_texture(TEXTURE_2D, Some(texture));
            //
            // // 为当前绑定的纹理对象设置环绕、过滤方式
            // gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
            // gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
            // //
            // gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            // gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            //
            // // 加载并生成纹理
            // let image = image::open("flower/resource/test2.png").expect("Failed to load texture");
            //
            // let data = image.as_bytes();
            //
            // gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, image.width() as i32, image.height() as i32, 0, RGBA, UNSIGNED_BYTE, Some(data));
            //
            // gl.generate_mipmap(TEXTURE_2D);
            //
            //
            // gl.bind_texture(TEXTURE_2D, Some(texture));
            // gl.draw_arrays(glow::QUADS, 0, 4);
            //
            // let i = gl.get_error();
            // println!("-----  {}", i);
            // for x in gl.get_debug_message_log(i) {
            //     println!("x---- {:?}", x);
            // }


            // glow::SRC1_ALPHA;
            gl.blend_func(glow::SRC_ALPHA, glow::SRC_COLOR);
            gl.enable(glow::BLEND);
            // gl.enable(glow::LINE_SMOOTH);
            // gl.enable(glow::POLYGON_SMOOTH);
            gl.enable(glow::MULTISAMPLE);
        }
        println!("button[{}] draw over focus {}", self.id(), self.focus);
        let shape = Shape::rect(self.left, self.top, self.left + self.width, self.top + self.height);
        match self.interactive_state {
            InteractiveState::Ordinary => {
                gl.draw_shape(shape, DrawMode::FIll, vec![FragData::Color(Color::from_hex_str("#409eff").unwrap())]);
            }
            InteractiveState::Active => {
                gl.draw_shape(shape, DrawMode::FIll, vec![FragData::Color(Color::from_hex_str("#66b1ff").unwrap())]);
            }
            InteractiveState::Pressed => {
                gl.draw_shape(shape, DrawMode::FIll, vec![FragData::Color(Color::from_hex_str("#3a8ee6").unwrap())]);
            }
            InteractiveState::Disable => {
                gl.draw_shape(shape, DrawMode::FIll, vec![FragData::Color(Color::from_hex_str("#909399").unwrap())]);
            }
        }

        // 加载并生成纹理
        let mut image = image::open("flower/resource/test.jpg").expect("Failed to load texture");
        // image = image.fliph();
        // image = image.resize(40, 60, FilterType::Nearest);
        // let shape = Shape::sector(200., 200., 100, 0., 500);
        // gl.draw_shape(shape, glow::LINE_LOOP, Fill::Color(Color::from_hex_str("00ccff").unwrap()));
        // //
        // let shape = Shape::circle(400., 400., 100);
        // gl.draw_shape(shape, glow::QUADS, Fill::Color(Color::from_hex_str("00ccff").unwrap()));

        // let shape = Shape::rect_radiu(50, 50, 100, 100, 0.);
        let shape = Shape::rect_radiu(50, 50, 500, 500, 10);
        // unsafe {gl.line_width(5.)}
        // gl.draw_shape(shape, glow::LINE_LOOP, vec![FragData::Color(Color::rgba(0, 204, 255, 150))]);


        // gl.draw_shape(shape.clone(), glow::QUADS, Fill::Image(Image::new(image.clone(), Align::LeftBottom, ZoomType::Zoom, glow::NEAREST)));
        // gl.draw_shape(shape.clone(), glow::QUADS, Fill::Image(Image::new(
        //     image.clone(),
        //     Align::LeftBottom,
        //     ZoomType::Tile(self.width, self.height),
        //     glow::NEAREST,
        // )));
        // gl.draw_shape(shape.clone(), glow::QUADS, Fragment::with(FragType::Color(Color::rgba(0,204,255,50))));


        // gl.draw_shape(shape.clone(), DrawMode::FIll, vec![
        //     FragData::Color(Color::rgba(0, 204, 255, 50)),
        //     FragData::Image(Image::new(image.clone(), Align::Center, ZoomType::Zoom, glow::NEAREST)),
        // ]);


        // gl.draw_shape(Shape::Bezier { points: vec![(500, 500), (1000, 500), (1000, 1000), (500, 1000)], p: 250 },
        //               glow::LINE_LOOP,
        //               vec![FragData::Color(Color::rgba(0, 204, 255, 150))]);
        gl.draw_shape(
            Shape::Rect {
                left: 500,
                top: 500,
                width: 500,
                height: 500,
                radiu_left_top: Some(300),
                radiu_left_bottom: Some(30),
                radiu_right_top: Some(50),
                radiu_right_bottom: None,
                line_width: None,
            },
            DrawMode::FIll,
            vec![
                FragData::Color(Color::rgba(0, 204, 255, 150)),
                FragData::Image(Image::from_src(image.clone())),
            ]);
        // gl.draw_shape(Shape::Bezier { points: vec![(500, 500), (1000, 500), (1000, 1000)], p: 25, line_width: None },
        //               DrawMode::FIll,
        //               vec![
        //                   FragData::Color(Color::rgba(0, 204, 255, 150)),
        //                   FragData::Image(Image::from_src(image.clone())),
        //               ]);

        gl.draw_shape(Shape::line_with_width((44, 50), (44, 100), 1), DrawMode::LINE, vec![FragData::Color(Color::rgba(66, 255, 00, 150))]);
        gl.draw_shape(Shape::line_with_width((50, 44), (100, 44), 1), DrawMode::LINE, vec![FragData::Color(Color::rgba(66, 255, 00, 150))]);

        let x = 4 / 2;
        // 竖线
        gl.draw_shape(Shape::line_with_width((50, 50 - x), (50, 100 + x + 1), 5), DrawMode::LINE, vec![FragData::Color(Color::rgba(0, 204, 255, 150))]);
        //
        gl.draw_shape(Shape::line_with_width((100, 50 - x), (100, 100 + x + 1), 5), DrawMode::LINE, vec![FragData::Color(Color::rgba(0, 204, 255, 150))]);
        // 横线
        gl.draw_shape(Shape::line_with_width((50 - x - 1, 50), (100 + x, 50), 5), DrawMode::LINE, vec![FragData::Color(Color::rgba(0, 204, 0, 150))]);
        gl.draw_shape(Shape::line_with_width((50 - x - 1, 100), (100 + x, 100), 5), DrawMode::LINE, vec![FragData::Color(Color::rgba(0, 204, 0, 150))]);
        // unsafe { gl.clear_color(1.0, 1.0, 1.0, 1.0); }

        // gl.draw_shape(Shape::line_with_width((200, 200), (200, 400.0), 5.), DrawMode::LINE, vec![FragData::Color(Color::rgba(255, 255, 80, 100))]);
        // gl.draw_shape(Shape::line_with_width((200, 400.0), (400.0, 400.0), 5.), DrawMode::LINE, vec![FragData::Color(Color::rgba(255, 255, 80, 100))]);
        // gl.draw_shape(Shape::line_with_width( (400.0, 400.0), (400.0, 200), 5.), DrawMode::LINE, vec![FragData::Color(Color::rgba(255, 255, 80, 100))]);
        // gl.draw_shape(Shape::line_with_width((400.0, 200), (200, 200), 5.), DrawMode::LINE, vec![FragData::Color(Color::rgba(255, 255, 80, 100))]);

        // gl.draw_shape(
        //     Shape::Rect {
        //         left: 200,
        //         top: 200,
        //         width: 200,
        //         height: 200,
        //         radiu_left_top: Some(50),
        //         radiu_left_bottom: Some(50),
        //         radiu_right_top: Some(50),
        //         radiu_right_bottom: Some(50),
        //         line_width: Some(5),
        //     },
        //     DrawMode::LINE,
        //     vec![FragData::Color(Color::rgba(0, 204, 255, 100))]
        // );
    }
}