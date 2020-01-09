pub static mut OPTIONS: Options = Options { color: true, styling: true };

pub struct Options {
    pub color: bool,
    pub styling: bool
}

pub fn modifyOptions(options: Options) {
    unsafe {
        OPTIONS = options;
    }
}