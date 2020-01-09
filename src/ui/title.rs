use crate::ui::utils;

pub fn new(text: &str) -> Box<Title> {
    return Box::new(Title { text: text });
}

pub struct Title<'a> {
    pub text: &'a str
}

impl crate::ui::Render for Title<'_> {
    fn render(& mut self) {
        println!("{}", utils::title(self.text));
    }
}