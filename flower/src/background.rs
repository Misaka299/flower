use std::io::Read;

#[derive(Debug, Clone)]
pub enum Background {
    None,
    Image(Vec<u8>, ImageSize),
}

#[derive(Debug, Copy, Clone)]
pub enum ImageSize {
    // 指定大小 宽、高
    Size(i32, i32),
    // 自适应宽高覆盖组件
    Cover,
}