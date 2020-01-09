pub fn new(children: Vec<Box<dyn crate::ui::Render>>) -> Box<Group> {
    return Box::new(Group { children: children });
}

pub struct Group {
    pub children: Vec<Box<dyn crate::ui::Render>>
}

impl Group {

}

impl crate::ui::Render for Group {
    fn render(& mut self) {
        for i in 0..self.children.len() {
            self.children[i].render();
        }
    }
}