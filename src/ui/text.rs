pub fn new(text: &str) -> Box<Text> {
    return Box::new(Text { text: text });
}

pub struct Text<'a> {
    pub text: &'a str
}

impl crate::ui::Render for Text<'_> {
    fn render(& mut self) {
        println!("{}", self.text);
    }
}
