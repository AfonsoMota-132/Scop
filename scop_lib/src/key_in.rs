#[derive(Clone, Default)]
pub struct KeyIn {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub r_left: bool,
    pub r_right: bool,
}

impl KeyIn {
    pub fn new() -> Self {
        KeyIn {
            up: false,
            down: false,
            left: false,
            right: false,
            r_left: false,
            r_right: false,
        }
    }
}
