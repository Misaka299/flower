use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Default)]
pub struct FontStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
}

impl Eq for FontStyle {}

impl PartialEq<Self> for FontStyle {
    fn eq(&self, other: &Self) -> bool {
        self.italic == other.italic && self.bold == other.bold
            && self.strikeout == other.strikeout && self.underline == other.underline
    }
}

impl Hash for FontStyle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.italic.hash(state);
        self.bold.hash(state);
        self.strikeout.hash(state);
        self.underline.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    pub name: String,
    pub width: i8,
    pub style: FontStyle,
}

impl Font {
    pub fn new(name: &str) -> Self {
        Font{
            name: name.to_string(),
            width: 1,
            style: Default::default(),
        }
    }
}

impl Eq for Font {}

impl PartialEq<Self> for Font {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Font {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.style.hash(state);
    }
}