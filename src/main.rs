extern crate getopts;
extern crate xcb;

use getopts::{Matches, Options};
use std::convert::TryInto;
use std::env;

mod events;
mod menufile;
mod reparent;
mod state;
mod taskbar;

use state::{Style, WmState};

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

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let (conn, root, style, wmstate) = setup_display(&matches);
    menufile::get_menuitems();
    taskbar::make_taskbar();
    scan_wins(&conn, root);
    events::do_event_loop();
}

fn scan_wins(conn: &xcb::Connection, root: xcb::Window) {
    let reply = xcb::query_tree(conn, root).get_reply();
    let mut attr: xcb::GetWindowAttributesReply;
    for w in reply.unwrap().children().iter() {
        let attr_reply = xcb::get_window_attributes(conn, *w).get_reply();
        attr = attr_reply.unwrap();
        if !attr.override_redirect()
            && attr.map_state() == xcb::MAP_STATE_VIEWABLE.try_into().unwrap()
        {
            reparent::make_new_client(*w);
        }
    }
}

fn setup_display(
    matches: &Matches,
) -> (xcb::Connection, xcb::Window, state::Style, state::WmState) {
    let (conn, screen_num) = xcb::Connection::connect(None).expect("Could not connect to X server");
    let setup = conn.get_setup();
    let screen = setup
        .roots()
        .nth(screen_num as usize)
        .expect("Could not get default screen");

    let root = screen.root();
    let cmap = screen.default_colormap();

    let wmstate = WmState::new(&conn);
    let style = Style::new(&conn, cmap, matches);

    (conn, root, style, wmstate)
}
