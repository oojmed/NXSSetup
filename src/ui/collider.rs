pub fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Box<Collider> {
    return Box::new(Collider { start_x: start_x, start_y: start_y, end_x: end_x, end_y: end_y })
}

pub struct Collider {
    pub start_x: i32,
    pub start_y: i32,

    pub end_x: i32,
    pub end_y: i32
}

impl Collider {
    pub fn check(&self, x: i32, y: i32) -> bool {
        if (x >= self.start_x && y >= self.start_y) && (x <= self.end_x && y <= self.end_y) {
            return true;
        }

        return false;
    }
}