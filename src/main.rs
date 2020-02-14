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
        for w in reply.unwrap().children().iter() {
            if let attr_reply = xcb::get_window_attributes(
                conn,
                *w,
            ).get_reply() {
                let attr = attr_reply.unwrap();
                if !attr.override_redirect() &&
                    attr.map_state() == xcb::MAP_STATE_VIEWABLE
                        .try_into()
                        .unwrap() {
                    reparent::make_new_client(*w);
                }
            }
        }
    }
}

fn setup_display() -> (xcb::Connection, xcb::Window) {
    let (conn, screen_num) = xcb::Connection::connect(None)
        .expect("Could not connect to X server");
    let setup = conn.get_setup();
    let root = setup
        .roots()
        .nth(screen_num as usize)
        .expect("Could not get default screen")
        .root();

    let wm_state_cookie = xcb::intern_atom(&conn, false, "WM_STATE");
    let wm_change_state_cookie = xcb::intern_atom(&conn, false, "WM_CHANGE_STATE");
    let wm_protos_cookie = xcb::intern_atom(&conn, false, "WM_PROTOCOLS");
    let wm_delete_cookie = xcb::intern_atom(&conn, false, "WM_DELETE_WINDOW");
    let wm_cmapwins_cookie = xcb::intern_atom(&conn, false, "WM_COLORMAP_WINDOWS");

    let wm_state = wm_state_cookie.get_reply().unwrap();
    let wm_change_state = wm_change_state_cookie.get_reply().unwrap();
    let wm_protos = wm_protos_cookie.get_reply().unwrap();
    let wm_delete = wm_delete_cookie.get_reply().unwrap();
    let wm_cmapwins = wm_cmapwins_cookie.get_reply().unwrap();

    (conn, root)
}
