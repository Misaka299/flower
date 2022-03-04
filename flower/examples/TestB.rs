use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::atomic::{AtomicI32, Ordering};

use log::debug;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

fn main() {}