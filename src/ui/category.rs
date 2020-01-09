use crate::ui::group;
use crate::ui::title;
use crate::ui::text;

pub fn new<'a>(title_str: &'a str, description_str: &'a str, group_obj: Box<group::Group>) -> Box<Category<'a>> {
    return Box::new(Category { group: group_obj, title: title::new(title_str), description: text::new(description_str) });
}

pub struct Category<'a> {
    pub group: Box<group::Group>,
    pub title: Box<title::Title<'a>>,
    pub description: Box<text::Text<'a>>
}

impl Category<'_> {

}

impl crate::ui::Render for Category<'_> {
    fn render(& mut self) {
        self.title.render();
        self.description.render();

        println!("");

        self.group.render();
    }
}