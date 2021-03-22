use ::libc;
use x11::xlib::*;
extern "C" {
    #[no_mangle]
    fn send_xmessage(_: Window, _: Atom, _: libc::c_long) -> libc::c_int;
    #[no_mangle]
    fn redraw_taskbar();
    #[no_mangle]
    static mut dsply: *mut Display;
    #[no_mangle]
    static mut root: Window;
    #[no_mangle]
    static mut screen: libc::c_int;
    #[no_mangle]
    static mut topmost_client: *mut Client;
    #[no_mangle]
    static mut fullscreen_client: *mut Client;
    #[no_mangle]
    static mut in_taskbar: libc::c_uint;
    #[no_mangle]
    static mut showing_taskbar: libc::c_uint;
    #[no_mangle]
    static mut fs_prevdims: Rect;
    #[no_mangle]
    static mut font: *mut XFontStruct;
    #[no_mangle]
    static mut text_gc: GC;
    #[no_mangle]
    static mut border_col: XColor;
    #[no_mangle]
    static mut active_col: XColor;
    #[no_mangle]
    static mut menu_col: XColor;
    #[no_mangle]
    static mut resize_curs: Cursor;
    #[no_mangle]
    static mut wm_protos: Atom;
    #[no_mangle]
    static mut wm_delete: Atom;
    #[no_mangle]
    fn find_client(_: Window, _: libc::c_int) -> *mut Client;
    #[no_mangle]
    fn set_wm_state(_: *mut Client, _: libc::c_int);
    #[no_mangle]
    fn send_config(_: *mut Client);
    #[no_mangle]
    fn redraw(_: *mut Client);
    #[no_mangle]
    fn check_focus(_: *mut Client);
    #[no_mangle]
    fn get_prev_focused() -> *mut Client;
    #[no_mangle]
    fn get_mouse_position(_: *mut libc::c_int, _: *mut libc::c_int);
    #[no_mangle]
    fn copy_dims(_: *mut Rect, _: *mut Rect);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                                   -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                              -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Client {
    pub next: *mut Client,
    pub name: *mut libc::c_char,
    pub size: *mut XSizeHints,
    pub window: Window,
    pub frame: Window,
    pub trans: Window,
    pub cmap: Colormap,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub ignore_unmap: libc::c_int,
    pub hidden: libc::c_uint,
    pub was_hidden: libc::c_uint,
    pub focus_order: libc::c_uint,
    pub has_been_shaped: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn raise_lower(mut c: *mut Client) {
    if !c.is_null() {
        if c == topmost_client {
            XLowerWindow(dsply, (*c).frame);
            topmost_client = 0 as *mut Client
            // lazy but amiwm does similar
        } else { XRaiseWindow(dsply, (*c).frame); topmost_client = c }
    };
}
/* increment ignore_unmap here and decrement it in handle_unmap_event in events.c */
#[no_mangle]
pub unsafe extern "C" fn hide(mut c: *mut Client) {
    if !c.is_null() {
        if (*c).hidden == 0 {
            (*c).ignore_unmap += 1;
            (*c).hidden = 1 as libc::c_int as libc::c_uint;
            if c == topmost_client { topmost_client = 0 as *mut Client }
            XUnmapWindow(dsply, (*c).frame);
            XUnmapWindow(dsply, (*c).window);
            set_wm_state(c, 3 as libc::c_int);
            check_focus(get_prev_focused());
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn unhide(mut c: *mut Client) {
    if !c.is_null() {
        if (*c).hidden != 0 {
            (*c).hidden = 0 as libc::c_int as libc::c_uint;
            topmost_client = c;
            XMapWindow(dsply, (*c).window);
            XMapRaised(dsply, (*c).frame);
            set_wm_state(c, 1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn toggle_fullscreen(mut c: *mut Client) {
    let mut xoffset: libc::c_int = 0;
    let mut yoffset: libc::c_int = 0;
    let mut maxwinwidth: libc::c_int = 0;
    let mut maxwinheight: libc::c_int = 0;
    if !c.is_null() && (*c).trans == 0 {
        if c == fullscreen_client {
            // reset to original size
            (*c).x = fs_prevdims.x;
            (*c).y = fs_prevdims.y;
            (*c).width = fs_prevdims.width;
            (*c).height = fs_prevdims.height;
            XMoveResizeWindow(dsply, (*c).frame, (*c).x,
                              (*c).y -
                                  ((*font).ascent + (*font).descent +
                                       2 as libc::c_int * 3 as libc::c_int +
                                       2 as libc::c_int),
                              (*c).width as libc::c_uint,
                              ((*c).height +
                                   ((*font).ascent + (*font).descent +
                                        2 as libc::c_int * 3 as libc::c_int +
                                        2 as libc::c_int)) as libc::c_uint);
            XMoveResizeWindow(dsply, (*c).window, 0 as libc::c_int,
                              (*font).ascent + (*font).descent +
                                  2 as libc::c_int * 3 as libc::c_int +
                                  2 as libc::c_int,
                              (*c).width as libc::c_uint,
                              (*c).height as libc::c_uint);
            send_config(c);
            fullscreen_client = 0 as *mut Client;
            showing_taskbar = 1 as libc::c_int as libc::c_uint
        } else {
            // make fullscreen
            yoffset = 0 as libc::c_int;
            xoffset = yoffset;
            maxwinwidth =
                (*(*(dsply as
                         _XPrivDisplay)).screens.offset(screen as
                                                            isize)).width;
            maxwinheight =
                (*(*(dsply as
                         _XPrivDisplay)).screens.offset(screen as
                                                            isize)).height -
                    ((*font).ascent + (*font).descent +
                         2 as libc::c_int * 3 as libc::c_int +
                         2 as libc::c_int);
            if !fullscreen_client.is_null() {
                // reset existing fullscreen window to original size
                (*fullscreen_client).x = fs_prevdims.x;
                (*fullscreen_client).y = fs_prevdims.y;
                (*fullscreen_client).width = fs_prevdims.width;
                (*fullscreen_client).height = fs_prevdims.height;
                XMoveResizeWindow(dsply, (*fullscreen_client).frame,
                                  (*fullscreen_client).x,
                                  (*fullscreen_client).y -
                                      ((*font).ascent + (*font).descent +
                                           2 as libc::c_int * 3 as libc::c_int
                                           + 2 as libc::c_int),
                                  (*fullscreen_client).width as libc::c_uint,
                                  ((*fullscreen_client).height +
                                       ((*font).ascent + (*font).descent +
                                            2 as libc::c_int *
                                                3 as libc::c_int +
                                            2 as libc::c_int)) as
                                      libc::c_uint);
                XMoveResizeWindow(dsply, (*fullscreen_client).window,
                                  0 as libc::c_int,
                                  (*font).ascent + (*font).descent +
                                      2 as libc::c_int * 3 as libc::c_int +
                                      2 as libc::c_int,
                                  (*fullscreen_client).width as libc::c_uint,
                                  (*fullscreen_client).height as
                                      libc::c_uint);
                send_config(fullscreen_client);
            }
            fs_prevdims.x = (*c).x;
            fs_prevdims.y = (*c).y;
            fs_prevdims.width = (*c).width;
            fs_prevdims.height = (*c).height;
            (*c).x = 0 as libc::c_int - 2 as libc::c_int;
            (*c).y =
                (*font).ascent + (*font).descent +
                    2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                    2 as libc::c_int;
            (*c).width = maxwinwidth;
            (*c).height = maxwinheight;
            if (*(*c).size).flags & (1 as libc::c_long) << 5 as libc::c_int !=
                   0 ||
                   (*(*c).size).flags &
                       (1 as libc::c_long) << 6 as libc::c_int != 0 {
                if (*(*c).size).flags &
                       (1 as libc::c_long) << 6 as libc::c_int != 0 {
                    let mut maxwinsize: Rect =
                        Rect{x: 0, y: 0, width: 0, height: 0,};
                    maxwinsize.x = xoffset;
                    maxwinsize.width = maxwinwidth;
                    maxwinsize.y = yoffset;
                    maxwinsize.height = maxwinheight;
                    get_incsize(c,
                                &mut (*(*c).size).max_width as
                                    *mut libc::c_int as *mut libc::c_uint,
                                &mut (*(*c).size).max_height as
                                    *mut libc::c_int as *mut libc::c_uint,
                                &mut maxwinsize, 0 as libc::c_int);
                }
                if (*(*c).size).max_width < maxwinwidth {
                    (*c).width = (*(*c).size).max_width;
                    xoffset = (maxwinwidth - (*c).width) / 2 as libc::c_int
                }
                if (*(*c).size).max_height < maxwinheight {
                    (*c).height = (*(*c).size).max_height;
                    yoffset = (maxwinheight - (*c).height) / 2 as libc::c_int
                }
            }
            XMoveResizeWindow(dsply, (*c).frame, (*c).x, (*c).y,
                              maxwinwidth as libc::c_uint,
                              maxwinheight as libc::c_uint);
            XMoveResizeWindow(dsply, (*c).window, xoffset, yoffset,
                              (*c).width as libc::c_uint,
                              (*c).height as libc::c_uint);
            send_config(c);
            fullscreen_client = c;
            showing_taskbar = in_taskbar
        }
        redraw_taskbar();
    };
}
/* The name of this function is a bit misleading: if the client
 * doesn't listen to WM_DELETE then we just terminate it with extreme
 * prejudice. */
#[no_mangle]
pub unsafe extern "C" fn send_wm_delete(mut c: *mut Client) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut protocols: *mut Atom = 0 as *mut Atom;
    if XGetWMProtocols(dsply, (*c).window, &mut protocols, &mut n) != 0 {
        i = 0 as libc::c_int;
        while i < n {
            if *protocols.offset(i as isize) == wm_delete { found += 1 }
            i += 1
        }
        XFree(protocols as *mut libc::c_void);
    }
    if found != 0 {
        send_xmessage((*c).window, wm_protos, wm_delete as libc::c_long);
    } else { XKillClient(dsply, (*c).window); };
}
#[export_name = "move"]
pub unsafe extern "C" fn move_0(mut c: *mut Client) {
    let mut ev: XEvent = XEvent{type_: 0,};
    let mut old_cx: libc::c_int = (*c).x;
    let mut old_cy: libc::c_int = (*c).y;
    let mut mousex: libc::c_int = 0;
    let mut mousey: libc::c_int = 0;
    let mut dw: libc::c_int = 0;
    let mut dh: libc::c_int = 0;
    let mut exposed_c: *mut Client = 0 as *mut Client;
    let mut bounddims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
    let mut constraint_win: Window = 0;
    let mut pattr: XSetWindowAttributes =
        XSetWindowAttributes{background_pixmap: 0,
                             background_pixel: 0,
                             border_pixmap: 0,
                             border_pixel: 0,
                             bit_gravity: 0,
                             win_gravity: 0,
                             backing_store: 0,
                             backing_planes: 0,
                             backing_pixel: 0,
                             save_under: 0,
                             event_mask: 0,
                             do_not_propagate_mask: 0,
                             override_redirect: 0,
                             colormap: 0,
                             cursor: 0,};
    dw = (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).width;
    dh =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).height;
    get_mouse_position(&mut mousex, &mut mousey);
    bounddims.x = mousex - (*c).x - 2 as libc::c_int;
    bounddims.width =
        dw - bounddims.x - ((*c).width - bounddims.x) + 1 as libc::c_int;
    bounddims.y = mousey - (*c).y;
    bounddims.height =
        dh - bounddims.y - ((*c).height - bounddims.y) + 1 as libc::c_int;
    bounddims.y +=
        ((*font).ascent + (*font).descent +
             2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
            2 as libc::c_int - 2 as libc::c_int;
    bounddims.height +=
        (*c).height -
            (((*font).ascent + (*font).descent +
                  2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
                 2 as libc::c_int - 2 as libc::c_int);
    constraint_win =
        XCreateWindow(dsply, root, bounddims.x, bounddims.y,
                      bounddims.width as libc::c_uint,
                      bounddims.height as libc::c_uint,
                      0 as libc::c_int as libc::c_uint,
                      0 as libc::c_long as libc::c_int,
                      2 as libc::c_int as libc::c_uint, 0 as *mut Visual,
                      0 as libc::c_int as libc::c_ulong, &mut pattr);
    XMapWindow(dsply, constraint_win);
    if !(XGrabPointer(dsply, root, 0 as libc::c_int,
                      ((1 as libc::c_long) << 2 as libc::c_int |
                           (1 as libc::c_long) << 3 as libc::c_int |
                           (1 as libc::c_long) << 6 as libc::c_int) as
                          libc::c_uint, 1 as libc::c_int, 1 as libc::c_int,
                      constraint_win, 0 as libc::c_long as Cursor,
                      0 as libc::c_long as Time) == 0 as libc::c_int) {
        XDestroyWindow(dsply, constraint_win);
        return
    }
    loop  {
        XMaskEvent(dsply,
                   (1 as libc::c_long) << 15 as libc::c_int |
                       ((1 as libc::c_long) << 2 as libc::c_int |
                            (1 as libc::c_long) << 3 as libc::c_int |
                            (1 as libc::c_long) << 6 as libc::c_int),
                   &mut ev);
        match ev.type_ {
            12 => {
                exposed_c = find_client(ev.expose.window, 1 as libc::c_int);
                if !exposed_c.is_null() { redraw(exposed_c); }
            }
            6 => {
                (*c).x = old_cx + (ev.motion.x - mousex);
                (*c).y = old_cy + (ev.motion.y - mousey);
                XMoveWindow(dsply, (*c).frame, (*c).x,
                            (*c).y -
                                ((*font).ascent + (*font).descent +
                                     2 as libc::c_int * 3 as libc::c_int +
                                     2 as libc::c_int));
                send_config(c);
            }
            _ => { }
        }
        if !(ev.type_ != 5 as libc::c_int) { break ; }
    }
    XUngrabPointer(dsply, 0 as libc::c_long as Time);
    XDestroyWindow(dsply, constraint_win);
}
#[no_mangle]
pub unsafe extern "C" fn resize(mut c: *mut Client, mut x: libc::c_int,
                                mut y: libc::c_int) {
    let mut ev: XEvent = XEvent{type_: 0,};
    let mut exposed_c: *mut Client = 0 as *mut Client;
    let mut newdims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
    let mut recalceddims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
    let mut bounddims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
    let mut dragging_outwards: libc::c_uint = 0;
    let mut dw: libc::c_uint = 0;
    let mut dh: libc::c_uint = 0;
    let mut constraint_win: Window = 0;
    let mut resize_win: Window = 0;
    let mut resizebar_win: Window = 0;
    let mut pattr: XSetWindowAttributes =
        XSetWindowAttributes{background_pixmap: 0,
                             background_pixel: 0,
                             border_pixmap: 0,
                             border_pixel: 0,
                             bit_gravity: 0,
                             win_gravity: 0,
                             backing_store: 0,
                             backing_planes: 0,
                             backing_pixel: 0,
                             save_under: 0,
                             event_mask: 0,
                             do_not_propagate_mask: 0,
                             override_redirect: 0,
                             colormap: 0,
                             cursor: 0,};
    let mut resize_pattr: XSetWindowAttributes =
        XSetWindowAttributes{background_pixmap: 0,
                             background_pixel: 0,
                             border_pixmap: 0,
                             border_pixel: 0,
                             bit_gravity: 0,
                             win_gravity: 0,
                             backing_store: 0,
                             backing_planes: 0,
                             backing_pixel: 0,
                             save_under: 0,
                             event_mask: 0,
                             do_not_propagate_mask: 0,
                             override_redirect: 0,
                             colormap: 0,
                             cursor: 0,};
    let mut resizebar_pattr: XSetWindowAttributes =
        XSetWindowAttributes{background_pixmap: 0,
                             background_pixel: 0,
                             border_pixmap: 0,
                             border_pixel: 0,
                             bit_gravity: 0,
                             win_gravity: 0,
                             backing_store: 0,
                             backing_planes: 0,
                             backing_pixel: 0,
                             save_under: 0,
                             event_mask: 0,
                             do_not_propagate_mask: 0,
                             override_redirect: 0,
                             colormap: 0,
                             cursor: 0,};
    if x > (*c).x + 2 as libc::c_int &&
           x < (*c).x + (*c).width - 2 as libc::c_int &&
           y >
               (*c).y -
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int +
                        2 as libc::c_int) + 2 as libc::c_int &&
           y < (*c).y + (*c).height - 2 as libc::c_int {
        // inside the window, dragging outwards
        dragging_outwards = 1 as libc::c_int as libc::c_uint
    } else {
        // outside the window, dragging inwards
        dragging_outwards = 0 as libc::c_int as libc::c_uint
    }
    dw =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).width
            as libc::c_uint;
    dh =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).height
            as libc::c_uint;
    bounddims.x = 0 as libc::c_int;
    bounddims.width = dw as libc::c_int;
    bounddims.y = 0 as libc::c_int;
    bounddims.height = dh as libc::c_int;
    constraint_win =
        XCreateWindow(dsply, root, bounddims.x, bounddims.y,
                      bounddims.width as libc::c_uint,
                      bounddims.height as libc::c_uint,
                      0 as libc::c_int as libc::c_uint,
                      0 as libc::c_long as libc::c_int,
                      2 as libc::c_int as libc::c_uint, 0 as *mut Visual,
                      0 as libc::c_int as libc::c_ulong, &mut pattr);
    XMapWindow(dsply, constraint_win);
    if !(XGrabPointer(dsply, root, 0 as libc::c_int,
                      ((1 as libc::c_long) << 2 as libc::c_int |
                           (1 as libc::c_long) << 3 as libc::c_int |
                           (1 as libc::c_long) << 6 as libc::c_int) as
                          libc::c_uint, 1 as libc::c_int, 1 as libc::c_int,
                      constraint_win, resize_curs, 0 as libc::c_long as Time)
             == 0 as libc::c_int) {
        XDestroyWindow(dsply, constraint_win);
        return
    }
    newdims.x = (*c).x;
    newdims.y =
        (*c).y -
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
    newdims.width = (*c).width;
    newdims.height =
        (*c).height +
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
    copy_dims(&mut newdims, &mut recalceddims);
    // create and map resize window
    resize_pattr.override_redirect = 1 as libc::c_int;
    resize_pattr.background_pixel = menu_col.pixel;
    resize_pattr.border_pixel = border_col.pixel;
    resize_pattr.event_mask =
        (1 as libc::c_long) << 20 as libc::c_int |
            (1 as libc::c_long) << 19 as libc::c_int |
            (1 as libc::c_long) << 2 as libc::c_int |
            (1 as libc::c_long) << 15 as libc::c_int |
            (1 as libc::c_long) << 4 as libc::c_int;
    resize_win =
        XCreateWindow(dsply, root, newdims.x, newdims.y,
                      newdims.width as libc::c_uint,
                      newdims.height as libc::c_uint,
                      2 as libc::c_int as libc::c_uint,
                      (*(*(dsply as
                               _XPrivDisplay)).screens.offset(screen as
                                                                  isize)).root_depth,
                      0 as libc::c_long as libc::c_uint,
                      (*(*(dsply as
                               _XPrivDisplay)).screens.offset(screen as
                                                                  isize)).root_visual,
                      ((1 as libc::c_long) << 9 as libc::c_int |
                           (1 as libc::c_long) << 1 as libc::c_int |
                           (1 as libc::c_long) << 3 as libc::c_int |
                           (1 as libc::c_long) << 11 as libc::c_int) as
                          libc::c_ulong, &mut resize_pattr);
    XMapRaised(dsply, resize_win);
    resizebar_pattr.override_redirect = 1 as libc::c_int;
    resizebar_pattr.background_pixel = active_col.pixel;
    resizebar_pattr.border_pixel = border_col.pixel;
    resizebar_pattr.event_mask =
        (1 as libc::c_long) << 20 as libc::c_int |
            (1 as libc::c_long) << 19 as libc::c_int |
            (1 as libc::c_long) << 2 as libc::c_int |
            (1 as libc::c_long) << 15 as libc::c_int |
            (1 as libc::c_long) << 4 as libc::c_int;
    resizebar_win =
        XCreateWindow(dsply, resize_win, -(2 as libc::c_int),
                      -(2 as libc::c_int), newdims.width as libc::c_uint,
                      ((*font).ascent + (*font).descent +
                           2 as libc::c_int * 3 as libc::c_int +
                           2 as libc::c_int - 2 as libc::c_int) as
                          libc::c_uint, 2 as libc::c_int as libc::c_uint,
                      (*(*(dsply as
                               _XPrivDisplay)).screens.offset(screen as
                                                                  isize)).root_depth,
                      0 as libc::c_long as libc::c_uint,
                      (*(*(dsply as
                               _XPrivDisplay)).screens.offset(screen as
                                                                  isize)).root_visual,
                      ((1 as libc::c_long) << 9 as libc::c_int |
                           (1 as libc::c_long) << 1 as libc::c_int |
                           (1 as libc::c_long) << 3 as libc::c_int |
                           (1 as libc::c_long) << 11 as libc::c_int) as
                          libc::c_ulong, &mut resizebar_pattr);
    XMapRaised(dsply, resizebar_win);
    // hide real window's frame
    XUnmapWindow(dsply, (*c).frame);
    loop  {
        XMaskEvent(dsply,
                   (1 as libc::c_long) << 15 as libc::c_int |
                       ((1 as libc::c_long) << 2 as libc::c_int |
                            (1 as libc::c_long) << 3 as libc::c_int |
                            (1 as libc::c_long) << 6 as libc::c_int),
                   &mut ev);
        match ev.type_ {
            12 => {
                if ev.expose.window == resizebar_win {
                    write_titletext(c, resizebar_win);
                } else {
                    exposed_c =
                        find_client(ev.expose.window, 1 as libc::c_int);
                    if !exposed_c.is_null() { redraw(exposed_c); }
                }
            }
            6 => {
                let mut in_taskbar_0: libc::c_uint =
                    1 as libc::c_int as libc::c_uint;
                let mut leftedge_changed: libc::c_uint =
                    0 as libc::c_int as libc::c_uint;
                let mut rightedge_changed: libc::c_uint =
                    0 as libc::c_int as libc::c_uint;
                let mut topedge_changed: libc::c_uint =
                    0 as libc::c_int as libc::c_uint;
                let mut bottomedge_changed: libc::c_uint =
                    0 as libc::c_int as libc::c_uint;
                let mut newwidth: libc::c_int = 0;
                let mut newheight: libc::c_int = 0;
                // warping the pointer is wrong - wait until it leaves the taskbar
                if ev.motion.y <
                       (*font).ascent + (*font).descent +
                           2 as libc::c_int * 3 as libc::c_int +
                           2 as libc::c_int {
                    in_taskbar_0 = 1 as libc::c_int as libc::c_uint
                } else {
                    if in_taskbar_0 == 1 as libc::c_int as libc::c_uint {
                        // first time outside taskbar
                        in_taskbar_0 = 0 as libc::c_int as libc::c_uint;
                        bounddims.x = 0 as libc::c_int;
                        bounddims.width = dw as libc::c_int;
                        bounddims.y =
                            (*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int;
                        bounddims.height =
                            dh.wrapping_sub(((*font).ascent + (*font).descent
                                                 +
                                                 2 as libc::c_int *
                                                     3 as libc::c_int +
                                                 2 as libc::c_int) as
                                                libc::c_uint) as libc::c_int;
                        XMoveResizeWindow(dsply, constraint_win, bounddims.x,
                                          bounddims.y,
                                          bounddims.width as libc::c_uint,
                                          bounddims.height as libc::c_uint);
                        in_taskbar_0 = 0 as libc::c_int as libc::c_uint
                    }
                    // inside the window, dragging outwards
                    if dragging_outwards != 0 {
                        if ev.motion.x < newdims.x + 2 as libc::c_int {
                            newdims.width +=
                                newdims.x + 2 as libc::c_int -
                                    ev.motion.x; // add 1 to allow window to be flush with edge of screen
                            newdims.x =
                                ev.motion.x -
                                    2 as
                                        libc::c_int; // add 1 to allow window to be flush with edge of screen
                            leftedge_changed =
                                1 as libc::c_int as libc::c_uint
                        } else if ev.motion.x >
                                      newdims.x + newdims.width +
                                          2 as libc::c_int {
                            newdims.width =
                                ev.motion.x - newdims.x - 2 as libc::c_int +
                                    1 as libc::c_int;
                            rightedge_changed =
                                1 as libc::c_int as libc::c_uint
                        }
                        if ev.motion.y < newdims.y + 2 as libc::c_int {
                            newdims.height +=
                                newdims.y + 2 as libc::c_int - ev.motion.y;
                            newdims.y = ev.motion.y - 2 as libc::c_int;
                            topedge_changed = 1 as libc::c_int as libc::c_uint
                        } else if ev.motion.y >
                                      newdims.y + newdims.height +
                                          2 as libc::c_int {
                            newdims.height =
                                ev.motion.y - newdims.y - 2 as libc::c_int +
                                    1 as libc::c_int;
                            bottomedge_changed =
                                1 as libc::c_int as libc::c_uint
                        }
                    } else {
                        // outside the window, dragging inwards
                        let mut above_win: libc::c_uint = 0;
                        let mut below_win: libc::c_uint = 0;
                        let mut leftof_win: libc::c_uint = 0;
                        let mut rightof_win: libc::c_uint = 0;
                        let mut in_win: libc::c_uint = 0;
                        above_win =
                            (ev.motion.y < newdims.y + 2 as libc::c_int) as
                                libc::c_int as libc::c_uint;
                        below_win =
                            (ev.motion.y >
                                 newdims.y + newdims.height +
                                     2 as libc::c_int) as libc::c_int as
                                libc::c_uint;
                        leftof_win =
                            (ev.motion.x < newdims.x + 2 as libc::c_int) as
                                libc::c_int as libc::c_uint;
                        rightof_win =
                            (ev.motion.x >
                                 newdims.x + newdims.width + 2 as libc::c_int)
                                as libc::c_int as libc::c_uint;
                        in_win =
                            (above_win == 0 && below_win == 0 &&
                                 leftof_win == 0 && rightof_win == 0) as
                                libc::c_int as libc::c_uint;
                        if in_win != 0 {
                            let mut from_left: libc::c_uint = 0;
                            let mut from_right: libc::c_uint = 0;
                            let mut from_top: libc::c_uint = 0;
                            let mut from_bottom: libc::c_uint = 0;
                            from_left =
                                (ev.motion.x - newdims.x - 2 as libc::c_int)
                                    as libc::c_uint;
                            from_right =
                                (newdims.x + newdims.width + 2 as libc::c_int
                                     - ev.motion.x) as libc::c_uint;
                            from_top =
                                (ev.motion.y - newdims.y - 2 as libc::c_int)
                                    as libc::c_uint;
                            from_bottom =
                                (newdims.y + newdims.height + 2 as libc::c_int
                                     - ev.motion.y) as libc::c_uint;
                            if from_left < from_right && from_left < from_top
                                   && from_left < from_bottom {
                                newdims.width -=
                                    ev.motion.x - newdims.x -
                                        2 as libc::c_int;
                                newdims.x = ev.motion.x - 2 as libc::c_int;
                                leftedge_changed =
                                    1 as libc::c_int as libc::c_uint
                            } else if from_right < from_top &&
                                          from_right < from_bottom {
                                newdims.width =
                                    ev.motion.x - newdims.x -
                                        2 as libc::c_int;
                                rightedge_changed =
                                    1 as libc::c_int as libc::c_uint
                            } else if from_top < from_bottom {
                                newdims.height -=
                                    ev.motion.y - newdims.y -
                                        2 as libc::c_int;
                                newdims.y = ev.motion.y - 2 as libc::c_int;
                                topedge_changed =
                                    1 as libc::c_int as libc::c_uint
                            } else {
                                newdims.height =
                                    ev.motion.y - newdims.y -
                                        2 as libc::c_int;
                                bottomedge_changed =
                                    1 as libc::c_int as libc::c_uint
                            }
                        }
                    }
                    // coords have changed
                    if leftedge_changed != 0 || rightedge_changed != 0 ||
                           topedge_changed != 0 || bottomedge_changed != 0 {
                        copy_dims(&mut newdims, &mut recalceddims);
                        recalceddims.height -=
                            (*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int;
                        if get_incsize(c,
                                       &mut newwidth as *mut libc::c_int as
                                           *mut libc::c_uint,
                                       &mut newheight as *mut libc::c_int as
                                           *mut libc::c_uint,
                                       &mut recalceddims, 0 as libc::c_int) !=
                               0 {
                            if leftedge_changed != 0 {
                                recalceddims.x =
                                    recalceddims.x + recalceddims.width -
                                        newwidth;
                                recalceddims.width = newwidth
                            } else if rightedge_changed != 0 {
                                recalceddims.width = newwidth
                            }
                            if topedge_changed != 0 {
                                recalceddims.y =
                                    recalceddims.y + recalceddims.height -
                                        newheight;
                                recalceddims.height = newheight
                            } else if bottomedge_changed != 0 {
                                recalceddims.height = newheight
                            }
                        }
                        recalceddims.height +=
                            (*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int;
                        limit_size(c, &mut recalceddims);
                        XMoveResizeWindow(dsply, resize_win, recalceddims.x,
                                          recalceddims.y,
                                          recalceddims.width as libc::c_uint,
                                          recalceddims.height as
                                              libc::c_uint);
                        XResizeWindow(dsply, resizebar_win,
                                      recalceddims.width as libc::c_uint,
                                      ((*font).ascent + (*font).descent +
                                           2 as libc::c_int * 3 as libc::c_int
                                           + 2 as libc::c_int -
                                           2 as libc::c_int) as libc::c_uint);
                    }
                }
            }
            _ => { }
        }
        if !(ev.type_ != 5 as libc::c_int) { break ; }
    }
    XUngrabServer(dsply);
    XUngrabPointer(dsply, 0 as libc::c_long as Time);
    (*c).x = recalceddims.x;
    (*c).y =
        recalceddims.y +
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
    (*c).width = recalceddims.width;
    (*c).height =
        recalceddims.height -
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
    XMoveResizeWindow(dsply, (*c).frame, (*c).x,
                      (*c).y -
                          ((*font).ascent + (*font).descent +
                               2 as libc::c_int * 3 as libc::c_int +
                               2 as libc::c_int), (*c).width as libc::c_uint,
                      ((*c).height +
                           ((*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int)) as libc::c_uint);
    XResizeWindow(dsply, (*c).window, (*c).width as libc::c_uint,
                  (*c).height as libc::c_uint);
    // unhide real window's frame
    XMapWindow(dsply, (*c).frame);
    XSetInputFocus(dsply, (*c).window, 0 as libc::c_long as libc::c_int,
                   0 as libc::c_long as Time);
    send_config(c);
    XDestroyWindow(dsply, constraint_win);
    XDestroyWindow(dsply, resizebar_win);
    XDestroyWindow(dsply, resize_win);
}
/* WindowLab - an X11 window manager
 * Copyright (c) 2001-2010 Nick Gravgaard
 * me at nickgravgaard.com
 * http://nickgravgaard.com/windowlab/
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301, USA.
 */
unsafe extern "C" fn limit_size(mut c: *mut Client, mut newdims: *mut Rect) {
    let mut dw: libc::c_int = 0;
    let mut dh: libc::c_int = 0;
    dw = (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).width;
    dh =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).height;
    if (*(*c).size).flags & (1 as libc::c_long) << 4 as libc::c_int != 0 {
        if (*newdims).width < (*(*c).size).min_width {
            (*newdims).width = (*(*c).size).min_width
        }
        if (*newdims).height < (*(*c).size).min_height {
            (*newdims).height = (*(*c).size).min_height
        }
    }
    if (*(*c).size).flags & (1 as libc::c_long) << 5 as libc::c_int != 0 {
        if (*newdims).width > (*(*c).size).max_width {
            (*newdims).width = (*(*c).size).max_width
        }
        if (*newdims).height > (*(*c).size).max_height {
            (*newdims).height = (*(*c).size).max_height
        }
    }
    if (*newdims).width <
           ((*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
               4 as libc::c_int {
        (*newdims).width =
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
                4 as libc::c_int
    }
    if (*newdims).height <
           ((*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
               4 as libc::c_int {
        (*newdims).height =
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
                4 as libc::c_int
    }
    if (*newdims).width > dw { (*newdims).width = dw }
    if (*newdims).height >
           dh -
               ((*font).ascent + (*font).descent +
                    2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) {
        (*newdims).height =
            dh -
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
    };
}
/* If the window in question has a ResizeInc int, then it wants to be
 * resized in multiples of some (x,y). Here we set x_ret and y_ret to
 * the number of multiples (if mode == INCREMENTS) or the correct size
 * in pixels for said multiples (if mode == PIXELS). */
unsafe extern "C" fn get_incsize(mut c: *mut Client,
                                 mut x_ret: *mut libc::c_uint,
                                 mut y_ret: *mut libc::c_uint,
                                 mut newdims: *mut Rect,
                                 mut mode: libc::c_int) -> libc::c_int {
    let mut basex: libc::c_int = 0;
    let mut basey: libc::c_int = 0;
    if (*(*c).size).flags & (1 as libc::c_long) << 6 as libc::c_int != 0 {
        basex =
            if (*(*c).size).flags & (1 as libc::c_long) << 8 as libc::c_int !=
                   0 {
                (*(*c).size).base_width
            } else if (*(*c).size).flags &
                          (1 as libc::c_long) << 4 as libc::c_int != 0 {
                (*(*c).size).min_width
            } else { 0 as libc::c_int };
        basey =
            if (*(*c).size).flags & (1 as libc::c_long) << 8 as libc::c_int !=
                   0 {
                (*(*c).size).base_height
            } else if (*(*c).size).flags &
                          (1 as libc::c_long) << 4 as libc::c_int != 0 {
                (*(*c).size).min_height
            } else { 0 as libc::c_int };
        // work around broken apps that set their resize increments to 0
        if mode == 0 as libc::c_int {
            if (*(*c).size).width_inc != 0 as libc::c_int {
                *x_ret =
                    ((*newdims).width -
                         ((*newdims).width - basex) % (*(*c).size).width_inc)
                        as libc::c_uint
            }
            if (*(*c).size).height_inc != 0 as libc::c_int {
                *y_ret =
                    ((*newdims).height -
                         ((*newdims).height - basey) %
                             (*(*c).size).height_inc) as libc::c_uint
            }
        } else {
            // INCREMENTS
            if (*(*c).size).width_inc != 0 as libc::c_int {
                *x_ret =
                    (((*newdims).width - basex) / (*(*c).size).width_inc) as
                        libc::c_uint
            }
            if (*(*c).size).height_inc != 0 as libc::c_int {
                *y_ret =
                    (((*newdims).height - basey) / (*(*c).size).height_inc) as
                        libc::c_uint
            }
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn write_titletext(mut c: *mut Client,
                                         mut bar_win: Window) {
    if (*c).trans == 0 && !(*c).name.is_null() {
        XDrawString(dsply, bar_win, text_gc, 3 as libc::c_int,
                    3 as libc::c_int + (*font).ascent, (*c).name,
                    strlen((*c).name) as libc::c_int);
    };
}
