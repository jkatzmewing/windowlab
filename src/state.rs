use getopts::Matches;
use xcb;
use xcb::render::Color;
use xcb::{Atom, Cursor, Gc};

pub struct Style {
    pub string_gc: Gc,
    pub border_gc: Gc,
    pub text_gc: Gc,
    pub active_gc: Gc,
    pub depressed_gc: Gc,
    pub inactive_gc: Gc,
    pub menu_gc: Gc,
    pub selected_gc: Gc,
    pub empty_gc: Gc,

    pub string_col: Color,
    pub border_col: Color,
    pub text_col: Color,
    pub active_col: Color,
    pub depressed_col: Color,
    pub inactive_col: Color,
    pub menu_col: Color,
    pub selected_col: Color,
    pub empty_col: Color,

    pub resize_curs: Cursor,
}

impl Style {
    pub fn new(conn: &xcb::Connection, cmap: xcb::Colormap, matches: &Matches) -> Style {
        use xcb::alloc_named_color;

        std::unimplemented!();

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
}

pub struct WmState {
    pub state: Atom,
    pub change_state: Atom,
    pub protos: Atom,
    pub delete: Atom,
    pub cmapwins: Atom,
}

impl WmState {
    pub fn new(conn: &xcb::Connection) -> WmState {
        use xcb::intern_atom;

        let wm_state_cookie = intern_atom(&conn, false, "WM_STATE");
        let wm_change_state_cookie = intern_atom(&conn, false, "WM_CHANGE_STATE");
        let wm_protos_cookie = intern_atom(&conn, false, "WM_PROTOCOLS");
        let wm_delete_cookie = intern_atom(&conn, false, "WM_DELETE_WINDOW");
        let wm_cmapwins_cookie = intern_atom(&conn, false, "WM_COLORMAP_WINDOWS");

        let wmstate = WmState {
            state: wm_state_cookie.get_reply().unwrap().atom(),
            change_state: wm_change_state_cookie.get_reply().unwrap().atom(),
            protos: wm_protos_cookie.get_reply().unwrap().atom(),
            delete: wm_delete_cookie.get_reply().unwrap().atom(),
            cmapwins: wm_cmapwins_cookie.get_reply().unwrap().atom(),
        };

        wmstate
    }
}
