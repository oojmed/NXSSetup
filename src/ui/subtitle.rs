use crate::ui::utils;

pub fn new(text: &str) -> Box<Subtitle> {
    return Box::new(Subtitle { text: text });
}

pub struct Subtitle<'a> {
    pub text: &'a str
}

impl crate::ui::Render for Subtitle<'_> {
    fn render(& mut self) {
        println!("{}", utils::underline(self.text));
    }
}