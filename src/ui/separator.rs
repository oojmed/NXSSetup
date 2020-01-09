pub fn new() -> Box<Separator> {
    return Box::new(Separator {});
}

pub struct Separator {

}

impl crate::ui::Render for Separator {
    fn render(& mut self) {
        println!("");
    }
}
