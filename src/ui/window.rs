use crate::ui::utils;
use crate::ui::checkbox;

extern crate termion;
extern crate crossterm;

pub fn new<'a>(title_str: &'a str, window_type: &'a str, checkboxes: Vec<Box<checkbox::Checkbox<'a>>>) -> Box<Window<'a>> {
    let padding: i32 = 2;

    let (o_term_width, o_term_height) = termion::terminal_size().unwrap();

    let term_width: i32 = o_term_width as i32;
    let term_height: i32 = o_term_height as i32;

    let mut width: i32 = title_str.chars().count() as i32;

    for c in &checkboxes {
        let this_width: i32 = c.text.chars().count() as i32 + 2;

        if this_width > width {
            width = this_width;
        }
    }

    width += padding * 2;

    if width < 30 {
        width = 30;
    }
    
    let height: i32 = checkboxes.len() as i32 + 2 + padding * 2;

    let term_center_x: i32 = term_width / 2;
    let term_center_y: i32 = term_height / 2;

    let x: i32 = term_center_x - width / 2;
    let y: i32 = term_center_y - height / 2;

    // println!("{}Window Debug: Width: {}, Height: {} | X: {}, Y: {} | Terminal Size: Width: {}, Height: {}", termion::cursor::Goto(1, 1), width, height, x, y, term_width, term_height);

    return Box::new(Window { checkboxes: checkboxes, title: title_str, window_type: window_type,
        x: x, y: y, width: width, height: height, padding: padding });
}

pub struct Window<'a> {
    pub checkboxes: Vec<Box<checkbox::Checkbox<'a>>>,
    pub title: &'a str,
    pub window_type: &'a str,
    
    pub x: i32,
    pub y: i32,

    pub width: i32,
    pub height: i32,

    pub padding: i32
}

impl Window<'_> {

}

pub static COLOR_BORDER: (i32, i32, i32) = (200, 200, 200);
pub static COLOR_TITLE: (i32, i32, i32) = (0, 0, 0);

pub static COLOR_BACKGROUND: (i32, i32, i32) = (50, 50, 50);
pub static COLOR_TEXT: (i32, i32, i32) = (200, 200, 200);

impl crate::ui::Render for Window<'_> {
    fn render(& mut self) {
        let mut actual_border_color: (i32, i32, i32) = COLOR_BORDER;
        let mut title = self.title;

        if self.window_type == "warning" {
            actual_border_color = (200, 200, 50);
        } else if self.window_type == "error" {
            actual_border_color = (200, 50, 50);
        }

        let CHAR_TOP_LEFT: String = utils::for_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, "█");
        let CHAR_TOP_RIGHT: String = utils::for_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, "█");

        let CHAR_SIDE: String = utils::for_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, "█");
        let CHAR_MIDDLE: String = utils::for_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, "█");
        let CHAR_EMPTY: String = utils::back_rgb(COLOR_BACKGROUND.0, COLOR_BACKGROUND.1, COLOR_BACKGROUND.2, " ");

        let CHAR_BOTTOM_LEFT: String = utils::for_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, "█");
        let CHAR_BOTTOM_RIGHT: String = utils::for_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, "█");

        let titlebar_border_count: f32 = ((self.width as usize - self.title.chars().count()) - 2) as f32;

        let titlebar = CHAR_TOP_LEFT.to_string() + &CHAR_MIDDLE.repeat((titlebar_border_count / 2.0).floor() as usize) + utils::for_rgb(COLOR_TITLE.0, COLOR_TITLE.1, COLOR_TITLE.2, utils::back_rgb(actual_border_color.0, actual_border_color.1, actual_border_color.2, self.title).as_str()).as_str() + &CHAR_MIDDLE.repeat((titlebar_border_count / 2.0).ceil() as usize) + CHAR_TOP_RIGHT.as_str();
        let empty = CHAR_SIDE.to_string() + &CHAR_EMPTY.repeat(self.width as usize - 2) + CHAR_SIDE.as_str();
        let end = CHAR_BOTTOM_LEFT.to_string() + &CHAR_MIDDLE.repeat(self.width as usize - 2) + CHAR_BOTTOM_RIGHT.as_str();

        let mut cursor = self.y;
        print!("{}{}", termion::cursor::Goto(self.x as u16, cursor as u16), titlebar);

        cursor += 1;

        for i in 0..self.padding {
            print!("{}{}", termion::cursor::Goto(self.x as u16, cursor as u16), empty);

            cursor += 1;
        }

        for i in 0..self.checkboxes.len() {
            let mut padding = (self.width as usize - self.checkboxes[i].text.chars().count()) as f64 - 2.0;
            if self.checkboxes[i].show_check {
                padding -= 2.0;
            }

            let padding_pre = CHAR_SIDE.to_string() + &CHAR_EMPTY.repeat((padding / 2.0).floor() as usize);
            let padding_end = CHAR_EMPTY.repeat((padding / 2.0).ceil() as usize) + CHAR_SIDE.as_str();

            print!("{}{}", termion::cursor::Goto(self.x as u16, cursor as u16), padding_pre);

            self.checkboxes[i].prerender(self.x + 1 + (padding / 2.0).floor() as i32, cursor);
            self.checkboxes[i].render();

            print!("{}", padding_end);

            cursor += 1;

            if i != self.checkboxes.len() - 1 {
                print!("{}{}", termion::cursor::Goto(self.x as u16, cursor as u16), empty);

                cursor += 1;
            }
        }

        for i in 0..self.padding {
            print!("{}{}", termion::cursor::Goto(self.x as u16, cursor as u16), empty);

            cursor += 1;
        }

        print!("{}{}", termion::cursor::Goto(self.x as u16, cursor as u16), end);
    }
}