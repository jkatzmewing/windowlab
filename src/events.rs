use ::libc;
use x11::xlib::*;

extern "C" {
    #[no_mangle]
    static mut do_menuitems: libc::c_int;
    #[no_mangle]
    fn free_menuitems();
    #[no_mangle]
    fn get_menuitems();
    #[no_mangle]
    fn toggle_fullscreen(_: *mut Client);
    #[no_mangle]
    fn resize(_: *mut Client, _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn lclick_taskbar(_: libc::c_int);
    #[no_mangle]
    fn rclick_taskbar(_: libc::c_int);
    #[no_mangle]
    fn cycle_previous();
    #[no_mangle]
    fn cycle_next();
    #[no_mangle]
    fn check_focus(_: *mut Client);
    #[no_mangle]
    fn draw_close_button(_: *mut Client, _: *mut GC, _: *mut GC);
    #[no_mangle]
    fn draw_toggledepth_button(_: *mut Client, _: *mut GC, _: *mut GC);
    #[no_mangle]
    fn draw_hide_button(_: *mut Client, _: *mut GC, _: *mut GC);
    #[no_mangle]
    fn send_wm_delete(_: *mut Client);
    #[no_mangle]
    fn raise_lower(_: *mut Client);
    #[link_name = "move"]
    fn move_0(_: *mut Client);
    #[no_mangle]
    fn rclick_root();
    #[no_mangle]
    fn refix_position(_: *mut Client, _: *mut XConfigureRequestEvent);
    #[no_mangle]
    fn gravitate(_: *mut Client, _: libc::c_int);
    #[no_mangle]
    fn send_config(_: *mut Client);
    #[no_mangle]
    fn unhide(_: *mut Client);
    #[no_mangle]
    fn make_new_client(_: Window);
    #[no_mangle]
    fn remove_client(_: *mut Client, _: libc::c_int);
    #[no_mangle]
    fn hide(_: *mut Client);
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut taskbar: Window;
    #[no_mangle]
    fn redraw_taskbar();
    #[no_mangle]
    fn redraw(_: *mut Client);
    #[no_mangle]
    fn set_shape(_: *mut Client);
    #[no_mangle]
    fn find_client(_: Window, _: libc::c_int) -> *mut Client;
    #[no_mangle]
    fn XUngrabPointer(_: *mut Display, _: Time) -> libc::c_int;
    #[no_mangle]
    fn XUngrabServer(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XGetWMNormalHints(_: *mut Display, _: Window, _: *mut XSizeHints,
                         _: *mut libc::c_long) -> libc::c_int;
    #[no_mangle]
    static mut dsply: *mut Display;
    #[no_mangle]
    static mut root: Window;
    #[no_mangle]
    static mut focused_client: *mut Client;
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
    static mut active_gc: GC;
    #[no_mangle]
    static mut depressed_gc: GC;
    #[no_mangle]
    static mut wm_change_state: Atom;
    #[no_mangle]
    static mut shape: libc::c_int;
    #[no_mangle]
    static mut shape_event: libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData)
                                 -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;

pub type Display = _XDisplay;
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
pub struct XMappingEvent {
    pub type_: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: libc::c_short,
    pub rbearing: libc::c_short,
    pub width: libc::c_short,
    pub ascent: libc::c_short,
    pub descent: libc::c_short,
    pub attributes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: libc::c_uint,
    pub min_char_or_byte2: libc::c_uint,
    pub max_char_or_byte2: libc::c_uint,
    pub min_byte1: libc::c_uint,
    pub max_byte1: libc::c_uint,
    pub all_chars_exist: libc::c_int,
    pub default_char: libc::c_uint,
    pub n_properties: libc::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: C2RustUnnamed_1,
    pub max_aspect: C2RustUnnamed_1,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XShapeEvent {
    pub type_: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub kind: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub time: Time,
    pub shaped: libc::c_int,
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
/* We may want to put in some sort of check for unknown events at some
 * point. TWM has an interesting and different way of doing this... */
#[no_mangle]
pub unsafe extern "C" fn do_event_loop() {
    let mut ev: XEvent = XEvent{type_: 0,};
    loop  {
        XNextEvent(dsply, &mut ev);
        /* check to see if menu rebuild has been requested */
        if do_menuitems != 0 { free_menuitems(); get_menuitems(); }
        match ev.type_ {
            2 => { handle_key_press(&mut ev.key); }
            4 => { handle_button_press(&mut ev.button); }
            23 => { handle_configure_request(&mut ev.configure_request); }
            20 => { handle_map_request(&mut ev.map_request); }
            18 => { handle_unmap_event(&mut ev.unmap); }
            17 => { handle_destroy_event(&mut ev.destroy_window); }
            33 => { handle_client_message(&mut ev.client_message); }
            32 => { handle_colormap_change(&mut ev.colormap); }
            28 => { handle_property_change(&mut ev.property); }
            7 => { handle_enter_event(&mut ev.crossing); }
            12 => { handle_expose_event(&mut ev.expose); }
            _ => {
                if shape != 0 && ev.type_ == shape_event {
                    handle_shape_change(&mut ev as *mut XEvent as
                                            *mut XShapeEvent);
                }
            }
        }
    };
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
unsafe extern "C" fn handle_key_press(mut e: *mut XKeyEvent) {
    let mut key: KeySym =
        XKeycodeToKeysym(dsply, (*e).keycode as KeyCode, 0 as libc::c_int);
    match key {
        65289 => { cycle_previous(); }
        113 => { cycle_next(); }
        65480 => { toggle_fullscreen(focused_client); }
        65481 => { raise_lower(focused_client); }
        _ => { }
    };
}
/* Someone clicked a button. If it was on the root, we get the click
 * by default. If it's on a window frame, we get it as well. If it's
 * on a client window, it may still fall through to us if the client
 * doesn't select for mouse-click events. */
unsafe extern "C" fn handle_button_press(mut e: *mut XButtonEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    if (*e).state & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
           != 0 {
        if !focused_client.is_null() && focused_client != fullscreen_client {
            resize(focused_client, (*e).x_root, (*e).y_root);
        } else {
            // pass event on
            XAllowEvents(dsply, 2 as libc::c_int, 0 as libc::c_long as Time);
        }
    } else if (*e).window == root {
        if (*e).button == 3 as libc::c_int as libc::c_uint { rclick_root(); }
    } else if (*e).window == taskbar {
        match (*e).button {
            1 => {
                // left mouse button
                lclick_taskbar((*e).x);
            }
            3 => {
                // right mouse button
                rclick_taskbar((*e).x);
            }
            4 => {
                // mouse wheel up
                cycle_previous();
            }
            5 => {
                // mouse wheel down
                cycle_next();
            }
            _ => { }
        }
    } else {
        // pass event on
        XAllowEvents(dsply, 2 as libc::c_int, 0 as libc::c_long as Time);
        if (*e).button == 1 as libc::c_int as libc::c_uint {
            c = find_client((*e).window, 1 as libc::c_int);
            if !c.is_null() {
                // click-to-focus
                check_focus(c);
                if (*e).y <
                       (*font).ascent + (*font).descent +
                           2 as libc::c_int * 3 as libc::c_int +
                           2 as libc::c_int && c != fullscreen_client {
                    handle_windowbar_click(e, c);
                }
            }
        } else if (*e).button == 3 as libc::c_int as libc::c_uint {
            rclick_root();
        }
    };
}
unsafe extern "C" fn handle_windowbar_click(mut e: *mut XButtonEvent,
                                            mut c: *mut Client) {
    static mut first_click_c: *mut Client = 0 as *const Client as *mut Client;
    static mut first_click_time: Time = 0;
    let mut in_box: libc::c_uint = 0;
    let mut in_box_down: libc::c_uint = 0;
    let mut in_box_up: libc::c_uint = 0;
    let mut win_ypos: libc::c_int = 0;
    let mut ev: XEvent = XEvent{type_: 0,};
    in_box_down = box_clicked(c, (*e).x);
    if in_box_down <= 2 as libc::c_int as libc::c_uint {
        if !(XGrabPointer(dsply, root, 0 as libc::c_int,
                          ((1 as libc::c_long) << 2 as libc::c_int |
                               (1 as libc::c_long) << 3 as libc::c_int |
                               (1 as libc::c_long) << 6 as libc::c_int) as
                              libc::c_uint, 1 as libc::c_int,
                          1 as libc::c_int, 0 as libc::c_long as Window,
                          0 as libc::c_long as Cursor,
                          0 as libc::c_long as Time) == 0 as libc::c_int) {
            return
        }
        XGrabServer(dsply);
        in_box = 1 as libc::c_int as libc::c_uint;
        draw_button(c, &mut text_gc, &mut depressed_gc, in_box_down);
        loop  {
            XMaskEvent(dsply,
                       (1 as libc::c_long) << 2 as libc::c_int |
                           (1 as libc::c_long) << 3 as libc::c_int |
                           (1 as libc::c_long) << 6 as libc::c_int, &mut ev);
            in_box_up =
                box_clicked(c, ev.button.x - ((*c).x + 2 as libc::c_int));
            win_ypos =
                ev.button.y - (*c).y +
                    ((*font).ascent + (*font).descent +
                         2 as libc::c_int * 3 as libc::c_int +
                         2 as libc::c_int);
            if ev.type_ == 6 as libc::c_int {
                if win_ypos <=
                       (*font).ascent + (*font).descent +
                           2 as libc::c_int * 3 as libc::c_int +
                           2 as libc::c_int && win_ypos >= 2 as libc::c_int &&
                       in_box_up == in_box_down {
                    in_box = 1 as libc::c_int as libc::c_uint;
                    draw_button(c, &mut text_gc, &mut depressed_gc,
                                in_box_down);
                } else {
                    in_box = 0 as libc::c_int as libc::c_uint;
                    draw_button(c, &mut text_gc, &mut active_gc, in_box_down);
                }
            }
            if !(ev.type_ != 5 as libc::c_int) { break ; }
        }
        draw_button(c, &mut text_gc, &mut active_gc, in_box_down);
        XUngrabServer(dsply);
        XUngrabPointer(dsply, 0 as libc::c_long as Time);
        if in_box != 0 {
            match in_box_up {
                0 => { send_wm_delete(c); }
                1 => { raise_lower(c); }
                2 => { hide(c); }
                _ => { }
            }
        }
    } else if in_box_down !=
                  (2147483647 as libc::c_int as
                       libc::c_uint).wrapping_mul(2 as
                                                      libc::c_uint).wrapping_add(1
                                                                                     as
                                                                                     libc::c_uint)
     {
        if first_click_c == c &&
               (*e).time.wrapping_sub(first_click_time) <
                   400 as libc::c_int as libc::c_ulong {
            raise_lower(c);
            first_click_c = 0 as *mut Client
            // prevent 3rd clicks counting as double clicks
        } else { first_click_c = c }
        first_click_time = (*e).time;
        move_0(c);
    };
}
/* Return which button was clicked - this is a multiple of BARHEIGHT()
 * from the right hand side. We only care about 0, 1 and 2. */
unsafe extern "C" fn box_clicked(mut c: *mut Client, mut x: libc::c_int)
 -> libc::c_uint {
    let mut pix_from_right: libc::c_int = (*c).width - x;
    if pix_from_right < 0 as libc::c_int {
        return (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
        // outside window
    } else {
        return (pix_from_right /
                    ((*font).ascent + (*font).descent +
                         2 as libc::c_int * 3 as libc::c_int +
                         2 as libc::c_int - 2 as libc::c_int)) as libc::c_uint
    };
}
unsafe extern "C" fn draw_button(mut c: *mut Client, mut detail_gc: *mut GC,
                                 mut background_gc: *mut GC,
                                 mut which_box: libc::c_uint) {
    match which_box {
        0 => { draw_close_button(c, detail_gc, background_gc); }
        1 => { draw_toggledepth_button(c, detail_gc, background_gc); }
        2 => { draw_hide_button(c, detail_gc, background_gc); }
        _ => { }
    };
}
/* Because we are redirecting the root window, we get ConfigureRequest
 * events from both clients we're handling and ones that we aren't.
 * For clients we manage, we need to fiddle with the frame and the
 * client window, and for unmanaged windows we have to pass along
 * everything unchanged. Thankfully, we can reuse (a) the
 * XWindowChanges struct and (b) the code to configure the client
 * window in both cases.
 *
 * Most of the assignments here are going to be garbage, but only the
 * ones that are masked in by e->value_mask will be looked at by the X
 * server.
 *
 * We ignore managed clients that want their z-order changed and
 * managed fullscreen clients that want their size and/or position
 * changed (except to update their size and/or position for when
 * fullscreen mode is toggled off). From what I can remember, clients
 * are supposed to have been written so that they are aware that their
 * requirements may not be met by the window manager. */
unsafe extern "C" fn handle_configure_request(mut e:
                                                  *mut XConfigureRequestEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    let mut wc: XWindowChanges =
        XWindowChanges{x: 0,
                       y: 0,
                       width: 0,
                       height: 0,
                       border_width: 0,
                       sibling: 0,
                       stack_mode: 0,};
    if !fullscreen_client.is_null() && c == fullscreen_client {
        if (*e).value_mask &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0
           {
            fs_prevdims.x = (*e).x
        }
        if (*e).value_mask &
               ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
           {
            fs_prevdims.y = (*e).y
        }
        if (*e).value_mask &
               ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0
           {
            fs_prevdims.width = (*e).width
        }
        if (*e).value_mask &
               ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
           {
            fs_prevdims.height = (*e).height
        }
        return
    }
    if !c.is_null() {
        gravitate(c, -(1 as libc::c_int));
        if (*e).value_mask &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0
           {
            (*c).x = (*e).x
        }
        if (*e).value_mask &
               ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
           {
            (*c).y = (*e).y
        }
        if (*e).value_mask &
               ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0
           {
            (*c).width = (*e).width
        }
        if (*e).value_mask &
               ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
           {
            (*c).height = (*e).height
        }
        refix_position(c, e);
        gravitate(c, 1 as libc::c_int);
        // configure the frame
        wc.x = (*c).x;
        wc.y =
            (*c).y -
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
        wc.width = (*c).width;
        wc.height =
            (*c).height +
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
        wc.border_width = 2 as libc::c_int;
        //wc.sibling = e->above;
		//wc.stack_mode = e->detail;
        XConfigureWindow(dsply, (*c).frame, (*e).value_mask as libc::c_uint,
                         &mut wc);
        if (*e).value_mask &
               ((1 as libc::c_int) << 2 as libc::c_int |
                    (1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
               != 0 {
            set_shape(c);
        }
        send_config(c);
        // start setting up the next call
        wc.x = 0 as libc::c_int;
        wc.y =
            (*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
    } else { wc.x = (*e).x; wc.y = (*e).y }
    wc.width = (*e).width;
    wc.height = (*e).height;
    //wc.sibling = e->above;
	//wc.stack_mode = e->detail;
    XConfigureWindow(dsply, (*e).window, (*e).value_mask as libc::c_uint,
                     &mut wc);
}
/* Two possibilities if a client is asking to be mapped. One is that
 * it's a new window, so we handle that if it isn't in our clients
 * list anywhere. The other is that it already exists and wants to
 * de-iconify, which is simple to take care of. */
unsafe extern "C" fn handle_map_request(mut e: *mut XMapRequestEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    if !c.is_null() { unhide(c); } else { make_new_client((*e).window); };
}
/* See windowlab.h for the intro to this one. If this is a window we
 * unmapped ourselves, decrement c->ignore_unmap and casually go on as
 * if nothing had happened. If the window unmapped itself from under
 * our feet, however, get rid of it.
 *
 * If you spend a lot of time with -DDEBUG on, you'll realize that
 * because most clients unmap and destroy themselves at once, they're
 * gone before we even get the Unmap event, never mind the Destroy
 * one. This will necessitate some extra caution in remove_client.
 *
 * Personally, I think that if Map events are intercepted, Unmap
 * events should be intercepted too. No use arguing with a standard
 * that's almost as old as I am though. :-( */
unsafe extern "C" fn handle_unmap_event(mut e: *mut XUnmapEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    if !c.is_null() {
        if (*c).ignore_unmap != 0 {
            (*c).ignore_unmap -= 1
        } else { remove_client(c, 0 as libc::c_int); }
    };
}
/* This happens when a window is iconified and destroys itself. An
 * Unmap event wouldn't happen in that case because the window is
 * already unmapped. */
unsafe extern "C" fn handle_destroy_event(mut e: *mut XDestroyWindowEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    if !c.is_null() { remove_client(c, 0 as libc::c_int); };
}
/* If a client wants to iconify itself (boo! hiss!) it must send a
 * special kind of ClientMessage. We might set up other handlers here
 * but there's nothing else required by the ICCCM. */
unsafe extern "C" fn handle_client_message(mut e: *mut XClientMessageEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    if !c.is_null() && (*e).message_type == wm_change_state &&
           (*e).format == 32 as libc::c_int &&
           (*e).data.as_longs()[0 as libc::c_int as usize] ==
               3 as libc::c_int as libc::c_long {
        hide(c);
    };
}
/* All that we have cached is the name and the size hints, so we only
 * have to check for those here. A change in the name means we have to
 * immediately wipe out the old name and redraw; size hints only get
 * used when we need them. */
unsafe extern "C" fn handle_property_change(mut e: *mut XPropertyEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    let mut dummy: libc::c_long = 0;
    if !c.is_null() {
        match (*e).atom {
            39 => {
                if !(*c).name.is_null() {
                    XFree((*c).name as *mut libc::c_void);
                    (*c).name = 0 as *mut libc::c_char
                }
                XFetchName(dsply, (*c).window, &mut (*c).name);
                redraw(c);
                redraw_taskbar();
            }
            40 => {
                XGetWMNormalHints(dsply, (*c).window, (*c).size, &mut dummy);
            }
            _ => { }
        }
    };
}
/* X's default focus policy is follows-mouse, but we have to set it
 * anyway because some sloppily written clients assume that (a) they
 * can set the focus whenever they want or (b) that they don't have
 * the focus unless the keyboard is grabbed to them. OTOH it does
 * allow us to keep the previous focus when pointing at the root,
 * which is nice.
 *
 * We also implement a colormap-follows-mouse policy here. That, on
 * the third hand, is *not* X's default. */
unsafe extern "C" fn handle_enter_event(mut e: *mut XCrossingEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    if (*e).window == taskbar {
        in_taskbar = 1 as libc::c_int as libc::c_uint;
        if showing_taskbar == 0 as libc::c_int as libc::c_uint {
            showing_taskbar = 1 as libc::c_int as libc::c_uint;
            redraw_taskbar();
        }
    } else {
        in_taskbar = 0 as libc::c_int as libc::c_uint;
        if !fullscreen_client.is_null() {
            if showing_taskbar == 1 as libc::c_int as libc::c_uint {
                showing_taskbar = 0 as libc::c_int as libc::c_uint;
                redraw_taskbar();
            }
        } else if showing_taskbar == 0 as libc::c_int as libc::c_uint {
            showing_taskbar = 1 as libc::c_int as libc::c_uint;
            redraw_taskbar();
        }
        c = find_client((*e).window, 1 as libc::c_int);
        if !c.is_null() {
            XGrabButton(dsply, 0 as libc::c_long as libc::c_uint,
                        ((1 as libc::c_int) << 15 as libc::c_int) as
                            libc::c_uint, (*c).frame, 0 as libc::c_int,
                        ((1 as libc::c_long) << 2 as libc::c_int |
                             (1 as libc::c_long) << 3 as libc::c_int) as
                            libc::c_uint, 0 as libc::c_int, 0 as libc::c_int,
                        0 as libc::c_long as Window,
                        0 as libc::c_long as Cursor);
        }
    };
}
// no fullscreen client
/* Here's part 2 of our colormap policy: when a client installs a new
 * colormap on itself, set the display's colormap to that. Arguably,
 * this is bad, because we should only set the colormap if that client
 * has the focus. However, clients don't usually set colormaps at
 * random when you're not interacting with them, so I think we're
 * safe. If you have an 8-bit display and this doesn't work for you,
 * by all means yell at me, but very few people have 8-bit displays
 * these days. */
unsafe extern "C" fn handle_colormap_change(mut e: *mut XColormapEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    //if (c != NULL && e->c_new) // use c_new for c++
    if !c.is_null() && (*e).new != 0 {
        (*c).cmap = (*e).colormap;
        XInstallColormap(dsply, (*c).cmap);
    };
}
/* If we were covered by multiple windows, we will usually get
 * multiple expose events, so ignore them unless e->count (the number
 * of outstanding exposes) is zero. */
unsafe extern "C" fn handle_expose_event(mut e: *mut XExposeEvent) {
    if (*e).window == taskbar {
        if (*e).count == 0 as libc::c_int { redraw_taskbar(); }
    } else {
        let mut c: *mut Client = find_client((*e).window, 1 as libc::c_int);
        if !c.is_null() && (*e).count == 0 as libc::c_int { redraw(c); }
    };
}
unsafe extern "C" fn handle_shape_change(mut e: *mut XShapeEvent) {
    let mut c: *mut Client = find_client((*e).window, 0 as libc::c_int);
    if !c.is_null() { set_shape(c); };
}