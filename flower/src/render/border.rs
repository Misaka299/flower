use crate::render::color::Color;

pub struct Border {
    width: u16,
    r#type: BorderType,
    color: Color,
}

pub enum BorderType {
    /// 定义一个点线边框
    None,

    /// 定义一个虚线边框
    Dashed,

    /// 定义实线边框
    Solid,

    /// 定义两个边框。 两个边框的宽度和 border-width 的值相同
    Double,
}

fn s() {}