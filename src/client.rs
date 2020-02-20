use xcb::{Colormap, FontDraw, Window};

pub struct Client {
    pub name: String,
    /* size: ??? - originally XSizeHint */
    pub window: Window,
    pub frame: Window,
    pub trans: Window,
    pub cmap: Colormap,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub ignore_unmap: bool,
    pub hidden: bool,
    pub was_hidden: bool,
    pub focus_order: bool,
    pub has_been_shaped: bool,
    pub xftdraw: FontDraw,
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct MenuItem {
    command: String,
    label: String,
    x: i32,
    width: i32,
}
