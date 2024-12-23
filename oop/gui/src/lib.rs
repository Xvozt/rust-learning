pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // trait object, the exact type is known at runtime using vtable
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub struct TextField {
    pub placeholder: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Implementation for drawing a button
        println!("Drawing a button: {}", self.label);
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a text field: {}", self.placeholder);
    }
}
