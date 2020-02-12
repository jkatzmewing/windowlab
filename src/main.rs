extern crate getopts;
extern crate xcb;

use getopts::Options;
use std::convert::TryInto;
use std::env;

mod events;
mod menufile;
mod reparent;
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

    let (conn, root) = setup_display();
    menufile::get_menuitems();
    taskbar::make_taskbar();
    scan_wins(&conn, root);
    events::do_event_loop();
}

fn scan_wins(conn: &xcb::Connection, root: xcb::Window) {
    if let reply = xcb::query_tree(conn, root).get_reply() {
        let children = reply.children();
        let n_children = reply.children_len();

        for n in 0..n_children {
            if let attr_reply = xcb::get_window_attributes(
                conn,
                children[n],
            ).get_reply() {
                let attr = attr_reply.unwrap();
                if attr.override_redirect() == 0 &&
                    attr.map_state() == xcb::MAP_STATE_VIEWABLE
                        .try_into()
                        .unwrap() {
                    reparent::make_new_client(children[n]);
                }
            }
        }
    }
}

fn setup_display() -> (xcb::Connection, xcb::Window) {
    std::unimplemented!();
}
