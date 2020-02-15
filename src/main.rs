extern crate getopts;
extern crate xcb;

use getopts::{Options, Matches};
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

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let (conn, root) = setup_display(&matches);
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

fn setup_display(matches: &Matches) -> (xcb::Connection, xcb::Window) {
    let (conn, screen_num) = xcb::Connection::connect(None)
        .expect("Could not connect to X server");
    let setup = conn.get_setup();
    let screen = setup
        .roots()
        .nth(screen_num as usize)
        .expect("Could not get default screen");

    let root = screen.root();
    let cmap = screen.default_colormap();

    setup_atoms(&conn);
    setup_colors(&conn, cmap, matches);

    (conn, root)
}

fn setup_atoms(conn: &xcb::Connection) {
    use xcb::intern_atom;

    let wm_state_cookie = intern_atom(&conn, false, "WM_STATE");
    let wm_change_state_cookie = intern_atom(&conn, false, "WM_CHANGE_STATE");
    let wm_protos_cookie = intern_atom(&conn, false, "WM_PROTOCOLS");
    let wm_delete_cookie = intern_atom(&conn, false, "WM_DELETE_WINDOW");
    let wm_cmapwins_cookie = intern_atom(&conn, false, "WM_COLORMAP_WINDOWS");

    let wm_state = wm_state_cookie.get_reply().unwrap();
    let wm_change_state = wm_change_state_cookie.get_reply().unwrap();
    let wm_protos = wm_protos_cookie.get_reply().unwrap();
    let wm_delete = wm_delete_cookie.get_reply().unwrap();
    let wm_cmapwins = wm_cmapwins_cookie.get_reply().unwrap();
}

fn setup_colors(conn: &xcb::Connection, cmap: xcb::Colormap, matches: &Matches) {
    use xcb::alloc_named_color;

    let border_cookie = alloc_named_color(conn, cmap, &matches.opt_str("border").unwrap());
    let text_cookie = alloc_named_color(conn, cmap, &matches.opt_str("text").unwrap());
    let active_cookie = alloc_named_color(conn, cmap, &matches.opt_str("active").unwrap());
    let inactive_cookie = alloc_named_color(conn, cmap, &matches.opt_str("inactive").unwrap());
    let menu_cookie = alloc_named_color(conn, cmap, &matches.opt_str("menu").unwrap());
    let selected_cookie = alloc_named_color(conn, cmap, &matches.opt_str("selected").unwrap());
    let empty_cookie = alloc_named_color(conn, cmap, &matches.opt_str("empty").unwrap());

    let border = border_cookie.get_reply();
    let text = text_cookie.get_reply();
    let active = active_cookie.get_reply();
    let inactive = inactive_cookie.get_reply();
    let menu = menu_cookie.get_reply();
    let selected = selected_cookie.get_reply();
    let empty = empty_cookie.get_reply();
}
