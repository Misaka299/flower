extern crate nalgebra_glm as glm;

use std::ops::{Deref, DerefMut};

use glow::HasContext;

use crate::control::{Control, ControlState, ControlType};
use crate::rect::Rect;
use crate::render::render::Renderer;

pub struct Button {
    control_state: ControlState,
    text: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn from(name: String, text: String) -> Button {
        Button {
            control_state: ControlState::create(name, false, ControlType::Control),
            text,
            on_click: None,
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
        // gl.create_canvas(&Rect::new(self.base_left + self.left, self.base_top + self.top, self.width, self.height));

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


            gl.use_def_program();
            // gl::UniformMatrix4fv(transformLoc, 1, gl::FALSE, transform.as_ptr());
            gl.rect(&Rect::new(self.base_left + self.left, self.base_top + self.top, self.width, self.height),Option::None);
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
        }
        println!("button[{}] draw over focus {}", self.id(), self.focus);

        // match self.interactive_state {
        //     InteractiveState::Ordinary => {
        //         gl.fill(&self.rect, &Color::from_hex_str("#FFF").unwrap());
        //     }
        //     InteractiveState::Active => {
        //         gl.fill(&self.rect, &Color::from_hex_str("#efefef").unwrap());
        //     }
        //     InteractiveState::Pressed => {
        //         gl.fill(&self.rect, &Color::from_hex_str("#3c4043").unwrap());
        //     }
        //     InteractiveState::Disable => {
        //         gl.fill(&self.rect, &Color::from_hex_str("##eaeaea").unwrap());
        //     }
        // }
    }
}