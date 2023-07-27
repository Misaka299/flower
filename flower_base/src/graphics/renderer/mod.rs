#[cfg(all(target_family = "windows",feature = "renderer-native-direct2d"))]
pub use crate::graphics::renderer::direct2d as default;

#[cfg(all(target_family = "windows",feature = "renderer-native-gdi-plus"))]
pub use crate::graphics::renderer::gdiplus as default;

#[cfg(feature = "renderer-native-direct2d")]
pub mod direct2d;
#[cfg(feature = "renderer-native-gdi-plus")]
pub mod gdiplus;