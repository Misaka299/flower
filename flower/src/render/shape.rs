use std::f32::consts;
use std::ops::{Deref, DerefMut};
use glow::{LINE_LOOP, LINE_STRIP, QUADS, TRIANGLE_FAN};

use crate::render::pixel_tool::PixelTool;

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    FIll,
    LINE,
}

#[derive(Debug, Clone)]
pub enum Shape {
    // 这里应该是顺时针12点，展开。不是就改程是。
    Sector { origin_x: u32, origin_y: u32, radiu_x: u32, radiu_y: u32, start_angle: f32, end_angle: f32, line_width: Option<u32> },
    Rect {
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        radiu_left_top: Option<u32>,
        radiu_left_bottom: Option<u32>,
        radiu_right_top: Option<u32>,
        radiu_right_bottom: Option<u32>,
        line_width: Option<u32>,
    },
    // 现在只是两阶曲线
    Bezier { points: Vec<(u32, u32)>, p: u32, line_width: Option<u32> },
    Line { start: (u32, u32), end: (u32, u32), line_width: Option<u32> },
    Other { vertex: Vec<f32>, line_width: Option<u32> },
}

impl Shape {
    ///
    /// create a custom shape.
    ///
    pub fn new_custom(vertex: Vec<f32>) -> Self {
        Self::Other {
            vertex,
            line_width: None,
        }
    }

    ///
    /// create a line.
    ///
    pub fn line(start: (u32, u32), end: (u32, u32)) -> Self {
        Self::Line {
            start,
            end,
            line_width: None,
        }
    }

    ///
    /// create a line of custom width..
    ///
    pub fn line_with_width(start: (u32, u32), end: (u32, u32), line_width: u32) -> Self {
        Self::Line {
            start,
            end,
            line_width: if line_width < 1 { None } else { Some(line_width) },
        }
    }


    ///
    /// create a rect shape.
    ///
    pub fn rect(left: u32, top: u32, width: u32, height: u32) -> Self {
        Self::Rect {
            left,
            top,
            width,
            height,
            radiu_left_top: None,
            radiu_left_bottom: None,
            radiu_right_top: None,
            radiu_right_bottom: None,
            line_width: None,
        }
    }

    ///
    /// create a rounded rectangle shape.
    ///
    pub fn rect_radiu(left: u32, top: u32, width: u32, height: u32, radiu: u32) -> Self {
        let radiu = if radiu == 0 { None } else { Some(radiu) };
        Self::Rect {
            left,
            top,
            width,
            height,
            radiu_left_top: radiu,
            radiu_left_bottom: radiu,
            radiu_right_top: radiu,
            radiu_right_bottom: radiu,
            line_width: None,
        }
    }

    ///
    /// create a circle.
    ///
    pub fn circle(origin_x: u32, origin_y: u32, radiu: u32) -> Self {
        Self::sector(origin_x, origin_y, radiu, 0., 360.)
    }

    ///
    /// create a circle use rectangle format param.
    /// If you pass a parameter of rectangular data, it will become an ellipse.
    ///
    pub fn circle_from_rect(left: u32, top: u32, width: u32, height: u32) -> Self {
        Self::sector_from_rect(left, top, width, height, 0., 360.)
    }

    ///
    /// create a sector.
    ///
    pub fn sector(origin_x: u32, origin_y: u32, radiu: u32, start_angle: f32, end_angle: f32) -> Self {
        Self::Sector {
            origin_x,
            origin_y,
            radiu_x: radiu,
            radiu_y: radiu,
            start_angle,
            end_angle,
            line_width: None,
        }
    }

    ///
    /// create a sector use rectangle format param.
    /// if you pass a parameter of rectangular data, it will become an ellipse.
    ///
    pub fn sector_from_rect(left: u32, top: u32, width: u32, height: u32, start_angle: f32, end_angle: f32) -> Self {
        let radiu_x = width / 2;
        let radiu_y = height / 2;
        Self::Sector {
            origin_x: left + radiu_x,
            origin_y: top + radiu_y,
            radiu_x,
            radiu_y,
            start_angle,
            end_angle,
            line_width: None,
        }
    }

    pub fn bezier(points: Vec<(u32, u32)>, p: u32) -> Self {
        Shape::Bezier {
            points,
            p,
            line_width: None,
        }
    }

    pub fn to_vertex(&self, mode: DrawMode, px: &PixelTool) -> (Vec<f32>, u32) {
        match *self {
            Shape::Sector { origin_x, origin_y, radiu_x, radiu_y, start_angle, end_angle, .. } => {
                let mut vertex = Vec::new();

                let p = (origin_x + origin_y) as f32 * consts::PI;

                let gl_x = px.to_gl_x(origin_x as f32);
                let gl_y = px.to_gl_y(origin_y as f32);

                let radiu_x = 1. + px.to_gl_x(radiu_x as f32);
                let radiu_y = 1. - px.to_gl_y(radiu_y as f32);

                let mut angle_start = (start_angle * 100.) as i32;
                let mut angle_end = (end_angle * 100.) as i32;

                // if angle_end - angle_start < p {
                //     vertex.push(gl_x);
                //     vertex.push(gl_y);
                // }
                for i in angle_start..angle_end {
                    use std::f32::consts;
                    let x = f32::cos(consts::PI / p as f32 * (2 * i) as f32) * radiu_x;
                    let y = f32::sin(consts::PI / p as f32 * (2 * i) as f32) * radiu_y;

                    vertex.push(x + gl_x);
                    vertex.push(y + gl_y);
                }
                (vertex, match mode {
                    DrawMode::FIll => { TRIANGLE_FAN }
                    DrawMode::LINE => { LINE_LOOP }
                })
            }
            Shape::Rect {
                left,
                top,
                width,
                height,
                radiu_left_top,
                radiu_left_bottom,
                radiu_right_top,
                radiu_right_bottom,
                line_width,
            } => {
                let mut vertex = Vec::new();
                let c = line_width.unwrap_or(0u32) / 2;
                if let Some(radiu) = radiu_left_bottom {
                    vertex.extend(Self::bezier(vec![
                        (left, top + height - radiu),
                        (left, top + height),
                        (left + radiu, top + height),
                    ], radiu * 2).to_vertex(mode, px).0.iter());
                } else {
                    vertex.push(px.to_gl_x((left - c) as f32));
                    vertex.push(px.to_gl_y((top + height) as f32));
                }

                if let Some(radiu) = radiu_right_bottom {
                    vertex.extend(Self::bezier(vec![
                        (left + width - radiu, top + height),
                        (left + width, top + height),
                        (left + width, top + height - radiu),
                    ], radiu as u32 * 2).to_vertex(mode, px).0.iter());
                } else {
                    vertex.push(px.to_gl_x((left + width + c) as f32));
                    vertex.push(px.to_gl_y((top + height) as f32));
                }


                if let Some(radiu) = radiu_right_top {
                    vertex.extend(Self::bezier(vec![
                        (left + width, top + radiu),
                        (left + width, top),
                        (left + width - radiu, top),
                    ], radiu as u32 * 2).to_vertex(mode, px).0.iter());
                } else {
                    vertex.push(px.to_gl_x(if c > 0 { left + width + c - 1 } else { left + width } as f32));
                    vertex.push(px.to_gl_y(top as f32));
                }

                if let Some(radiu) = radiu_left_top {
                    vertex.extend(Self::bezier(vec![
                        (left + radiu, top),
                        (left, top),
                        (left, top + radiu),
                    ], radiu as u32 * 2).to_vertex(mode, px).0.iter());
                } else {
                    println!("c === {}", c);
                    vertex.push(px.to_gl_x(if c > 0 { left - c - 1 } else { left } as f32));
                    vertex.push(px.to_gl_y((top) as f32));
                }

                (vertex, match mode {
                    DrawMode::FIll => { TRIANGLE_FAN }
                    DrawMode::LINE => { LINE_LOOP }
                })
            }
            Shape::Bezier { ref points, p, .. } => {
                let mut vertex = Vec::new();
                let tc = 1. / p as f32;
                let mut p = (0., 0.);
                let mut t = 0.;
                while t <= 1. {
                    let f1 = (1. - t) * (1. - t);
                    let f2 = 2. * (1. - t) * t;
                    let f3 = t * t;
                    p.0 = points[0].0 as f32 * f1 + points[1].0 as f32 * f2 + points[2].0 as f32 * f3;
                    p.1 = points[0].1 as f32 * f1 + points[1].1 as f32 * f2 + points[2].1 as f32 * f3;
                    vertex.push(px.to_gl_x(p.0));
                    vertex.push(px.to_gl_y(p.1));
                    t = t + tc;
                }
                (vertex, match mode {
                    DrawMode::FIll => { TRIANGLE_FAN }
                    DrawMode::LINE => { LINE_STRIP }
                })
            }
            Shape::Line { start, end, .. } => {
                let mut vertex = Vec::new();
                if start == end {
                    vertex.push(px.to_gl_x(start.0 as f32));
                    vertex.push(px.to_gl_y(start.1 as f32));
                    ((vertex, match mode {
                        DrawMode::FIll => { TRIANGLE_FAN }
                        DrawMode::LINE => { LINE_STRIP }
                    }))
                } else {
                    vertex.push(px.to_gl_x(start.0 as f32));
                    vertex.push(px.to_gl_y(start.1 as f32));
                    vertex.push(px.to_gl_x(end.0 as f32));
                    vertex.push(px.to_gl_y(end.1 as f32));
                    ((vertex, match mode {
                        DrawMode::FIll => { TRIANGLE_FAN }
                        DrawMode::LINE => { LINE_STRIP }
                    }))
                }
            }
            Shape::Other { ref vertex, .. } => {
                let mut vertex = vertex.clone();
                for i in 0..vertex.len() {
                    if i % 2 == 0 {
                        vertex[i] = px.to_gl_x(vertex[i]);
                    } else {
                        vertex[i] = px.to_gl_y(vertex[i]);
                    }
                }
                (vertex, match mode {
                    DrawMode::FIll => { TRIANGLE_FAN }
                    DrawMode::LINE => { LINE_LOOP }
                })
            }
        }
    }

    ///
    /// image_size 贝塞尔曲线时需要
    /// 贝塞尔传递的顶点标记为从图片截取的部分
    ///
    pub fn to_texture_coord(&self, image_size: Option<(u32, u32)>) -> Vec<f32> {
        match *self {
            Shape::Sector { .. } => {
                let mut coord = Vec::new();
                coord
            }
            Shape::Rect { left, top, width, height, radiu_left_top, radiu_left_bottom, radiu_right_top, radiu_right_bottom, line_width, } => {
                let mut coord = Vec::new();
                let image_size = Some((width, height));
                if let Some(radiu) = radiu_left_bottom {
                    coord.extend(Self::bezier(vec![
                        (0, 0 + height - radiu),
                        (0, 0 + height),
                        (0 + radiu, 0 + height),
                    ], radiu * 2).to_texture_coord(image_size).iter());
                } else {
                    coord.push((1. / width as f32) * (0 as f32));
                    coord.push((1. / height as f32) * (0 + height) as f32);
                }
                if let Some(radiu) = radiu_right_bottom {
                    coord.extend(Self::bezier(vec![
                        (0 + width - radiu, 0 + height),
                        (0 + width, 0 + height),
                        (0 + width, 0 + height - radiu),
                    ], radiu as u32 * 2).to_texture_coord(image_size).iter());
                } else {
                    coord.push((1. / width as f32) * (0 + width) as f32);
                    coord.push((1. / height as f32) * (0 + height) as f32);
                }
                if let Some(radiu) = radiu_right_top {
                    coord.extend(Self::bezier(vec![
                        (0 + width, 0 + radiu),
                        (0 + width, 0),
                        (0 + width - radiu, 0),
                    ], radiu as u32 * 2).to_texture_coord(image_size).iter());
                } else {
                    coord.push((1. / width as f32) * (0 + width) as f32);
                    coord.push((1. / height as f32) * 0 as f32);
                }
                if let Some(radiu) = radiu_left_top {
                    coord.extend(Self::bezier(vec![
                        (0 + radiu, 0),
                        (0, 0),
                        (0, 0 + radiu),
                    ], radiu as u32 * 2).to_texture_coord(image_size).iter());
                } else {
                    coord.push((1. / width as f32) * 0 as f32);
                    coord.push((1. / height as f32) * 0 as f32);
                }
                coord
            }
            Shape::Bezier { ref points, p, line_width } => {
                let mut coord = Vec::new();
                let tc = 1. / p as f32;
                let mut p = (0., 0.);
                let mut t = 0.;
                while t <= 1. {
                    let f1 = (1. - t) * (1. - t);
                    let f2 = 2. * (1. - t) * t;
                    let f3 = t * t;
                    p.0 = points[0].0 as f32 * f1 + points[1].0 as f32 * f2 + points[2].0 as f32 * f3;
                    p.1 = points[0].1 as f32 * f1 + points[1].1 as f32 * f2 + points[2].1 as f32 * f3;

                    coord.push((1. / image_size.unwrap().0 as f32) * p.0);
                    coord.push((1. / image_size.unwrap().1 as f32) * p.1);
                    t = t + tc;
                }
                coord
            }
            Shape::Line { .. } | Shape::Other { .. } => {
                let mut coord = Vec::new();

                coord.push(1.);//-1
                coord.push(1.);//-1
                coord.push(0.);//-1
                coord.push(1.);//1


                coord.push(0.);//1
                coord.push(0.);//1
                coord.push(1.);//-1
                coord.push(0.);//1
                coord
            }
        }
    }

    pub fn get_line_width(&self) -> Option<u32> {
        *match self {
            Shape::Sector { origin_x: _origin_x, origin_y: _origin_y, radiu_x: _radiu_x, radiu_y: _radiu_y, start_angle: _start_angle, end_angle: _end_angle, line_width } => { line_width }
            Shape::Rect { left: _left, top: _top, width: _width, height: _height, radiu_left_top: _radiu_left_top, radiu_left_bottom: _radiu_left_bottom, radiu_right_top: _radiu_right_top, radiu_right_bottom: _radiu_right_bottom, line_width } => { line_width }
            Shape::Bezier { points: _points, p: _p, line_width } => { line_width }
            Shape::Line { start: _start, end: _end, line_width } => { line_width }
            Shape::Other { vertex: _vertex, line_width } => { line_width }
        }
    }
}

// 三阶
// fn s (){
//     let tc = 1 / p;
//     let mut p = (0., 0.);
//     let mut t = 0.;
//     while t < 1. {
//         let a1 = u32::powf((1. - t), 3.);
//         let a2 = u32::powf((1. - t), 2.) * 3. * t;
//         let a3 = 3. * t * t * (1. - t);
//         let a4 = t * t * t;
//         p.x = a1.abs() * p1.0 + a2.abs() * p2.0 + a3 * p3.0 + a4 * p4.0;
//         p.y = a1.abs() * p1.1 + a2.abs() * p2.1 + a3 * p3.1 + a4 * p4.1;
//         vertex.push(px.to_gl_x(p.x));
//         vertex.push(px.to_gl_y(p.y));
//         t = t + 0.05;
//     }
// }