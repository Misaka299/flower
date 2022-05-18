use std::ops::{Deref, DerefMut};
use glm::ext::{scale, translate};
use glm::{mat2, Mat4};
use glow::Context;

use crate::Px;
use crate::rect::Rect;
use crate::render::border::Border;
use crate::render::draw::Draw;

pub trait Render {}

pub struct PaintSetting {
    radiu: Px,
    border: Border,
}

pub struct Renderer {
    // gl: Context,
    gl: Draw,
}

impl Renderer {
    pub(crate) fn set_window_height(&mut self, window_height: Px) {
        self.gl.window_height = window_height;
    }
}

impl Render for Renderer {}

impl Renderer {
    pub fn new(gl: Context, height: Px) -> Self {
        Self {
            gl: Draw::new(gl, height)
        }
    }
}

impl Renderer {
    ///
    /// 绘制矩形
    ///
    /// 支持如下属性
    /// radiu
    /// border
    ///
    fn rect(&mut self, rect: &Rect, border: Option<Border>) {
        mat2(1.0,1.0,1.0,1.0);
    }

    ///
    /// 填充矩形
    ///
    /// 支持如下属性
    /// radiu
    /// border
    ///
    fn fill(&mut self, rect: &Rect, border: Option<Border>) {}

    /// 绘制文字
    fn text(&mut self, text: String, setting: PaintSetting) {}

    /// 绘制线
    fn line(&mut self, line: (Px, Px)) {}

    // 贝塞尔曲线
    fn bezier(&mut self, line_start: (Px, Px), line_end: (Px, Px)) {}

    /// todo 参数未定义
    fn circle(&mut self) {}

    /// 缺失参数,绘制方式
    fn image(&mut self, image: Vec<u8>) {}
}

impl Deref for Renderer {
    type Target = Draw;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gl
    }
}