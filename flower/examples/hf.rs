fn main() {
    let hf = HasF {};
    let manipulator = Manipulator::Manipulator(hf);
    manipulator.obj.f();
    manipulator.manipulate();
}

struct HasF {}

impl HasF {
    pub fn f(self) {
        println!("HasF.f()")
    }
}

struct Manipulator<T> {
    obj: T,
}

impl<T> Manipulator<T> {
    pub fn Manipulator(x: T) -> Manipulator<T> {
        Manipulator { obj: x }
    }
    pub fn manipulate(self) { self.obj.f(); }
}