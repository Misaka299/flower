use std::any::Any;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::unsync::Lazy;
use rustc_hash::FxHashMap;

pub trait ThemeMap<K, V> {}

impl<K, V> ThemeMap<K, V> for FxHashMap<K, V> {}

impl<K, V> ThemeMap<K, V> for HashMap<K, V> {}

pub static mut THEME1: Lazy<Box<dyn ThemeMap<String, Theme>>> = Lazy::new(|| Box::new(FxHashMap::default()));


#[derive(Clone)]
pub struct Theme {}