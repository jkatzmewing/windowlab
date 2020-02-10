extern crate getopts;

use getopts::Options;
use std::env;

mod events;
mod menufile;
mod taskbar;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optopt("", "font", "set font", "FONT");
    opts.optopt("", "border", "set border color", "COLOR");
    opts.optopt("", "text", "set text color", "COLOR");
    opts.optopt("", "active", "set active color", "COLOR");
    opts.optopt("", "inactive", "set inactive color", "COLOR");
    opts.optopt("", "menu", "set menu color", "COLOR");
    opts.optopt("", "selected", "set selected color", "COLOR");
    opts.optopt("", "empty", "set empty color", "COLOR");
    opts.optopt("", "display", "set X11 display", "DISPLAY");

    setup_display();
    menufile::get_menuitems();
    taskbar::make_taskbar();
    scan_wins();
    events::do_event_loop();
}

fn scan_wins() {
    std::unimplemented!();
}

fn setup_display() {
    std::unimplemented!();
}
