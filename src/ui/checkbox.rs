use crate::ui::utils;
use crate::ui::collider;
use crate::ui::Render;
use crate::ui::window;

extern crate termion;
extern crate crossterm;

extern crate strip_ansi_escapes;

pub static mut CHOSEN: [String; 10] = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];

pub fn new(text: String, checked: bool, show_check: bool, interactable: bool) -> Box<Checkbox> {
    return Box::new(Checkbox { x: 0, y: 0, checked: checked, show_check: show_check, interactable: interactable, selected: false, text: text, collider: collider::new(1, 1, 2, 2) });
}

pub struct Checkbox {
    pub checked: bool,
    pub show_check: bool,
    pub interactable: bool,

    pub selected: bool,

    pub text: String,

    pub collider: Box<collider::Collider>,

    pub x: i32,
    pub y: i32
}

impl Checkbox {
    pub fn check(& mut self) {
        self.checked = true;
    }

    pub fn uncheck(& mut self) {
        self.checked = false;
    }
    
    pub fn toggle_checked(& mut self) {
        if self.checked {
            self.uncheck();
        } else {
            self.check();
        }
    }

    pub fn select(& mut self) {
        self.selected = true;
    }

    pub fn unselect(& mut self) {
        self.selected = false;
    }

    pub fn toggle_selected(& mut self) {
        if self.selected {
            self.unselect();
        } else {
            self.select();
        }
    }

    pub fn prerender(& mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;

        let mut after_x = self.x;

        if self.show_check {
            after_x += self.text.chars().count() as i32 + 2;
        } else {
            after_x += self.text.chars().count() as i32;
        }

        self.collider = collider::new(self.x, self.y, after_x, self.y);
    }
}

impl crate::ui::Render for Checkbox {
    fn render(& mut self) {
        let italic_text = utils::italic(self.text.as_str());

        if self.show_check {
            print!("{}", utils::back_rgb(window::COLOR_BACKGROUND.0, window::COLOR_BACKGROUND.1, window::COLOR_BACKGROUND.2,
                format!("{} {}", if self.checked {
                        utils::for_rgb(20, 200, 20, "✔")
                    } else {
                        utils::for_rgb(200, 20, 20, "✗")
                    },
                    if self.interactable { italic_text } else { self.text.clone() }).as_str()));
        } else {
            print!("{}", utils::back_rgb(window::COLOR_BACKGROUND.0, window::COLOR_BACKGROUND.1, window::COLOR_BACKGROUND.2,
                if self.interactable { italic_text.as_str() } else { self.text.as_str() }));
        }
    }
}

impl crate::ui::Interact for Checkbox {
    fn interact(& mut self) -> bool {
        if !self.interactable {
            return false;
        }

        self.toggle_checked();

        unsafe {
            for i in 0..CHOSEN.len() {
                if CHOSEN[i] == self.text && !self.checked {
                    CHOSEN[i] = "".to_string();
                } else if CHOSEN[i] == "" && self.checked {
                    CHOSEN[i] = String::from_utf8(strip_ansi_escapes::strip(self.text.clone()).unwrap()).unwrap();
                    break;
                }
            }

            print!("{}{:?}{}", termion::cursor::Goto(1, 7), CHOSEN, termion::cursor::Goto(self.collider.start_x as u16, self.collider.start_y as u16));
        }

        self.render();

        return true;
    }
}