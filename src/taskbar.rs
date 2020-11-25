use ::libc;
use x11::xlib::*;

extern "C" {
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    #[no_mangle]
    static mut num_menuitems: libc::c_uint;
    #[no_mangle]
    static mut menuitems: *mut MenuItem;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut inactive_gc: GC;
    #[no_mangle]
    static mut dsply: *mut Display;
    #[no_mangle]
    static mut root: Window;
    #[no_mangle]
    static mut screen: libc::c_int;
    #[no_mangle]
    static mut head_client: *mut Client;
    #[no_mangle]
    static mut focused_client: *mut Client;
    #[no_mangle]
    static mut showing_taskbar: libc::c_uint;
    #[no_mangle]
    static mut font: *mut XFontStruct;
    #[no_mangle]
    static mut border_gc: GC;
    #[no_mangle]
    static mut text_gc: GC;
    #[no_mangle]
    static mut active_gc: GC;
    #[no_mangle]
    static mut menu_gc: GC;
    #[no_mangle]
    static mut selected_gc: GC;
    #[no_mangle]
    static mut border_col: XColor;
    #[no_mangle]
    static mut empty_col: XColor;
    #[no_mangle]
    fn find_client(_: Window, _: libc::c_int) -> *mut Client;
    #[no_mangle]
    fn redraw(_: *mut Client);
    #[no_mangle]
    fn check_focus(_: *mut Client);
    #[no_mangle]
    fn raise_lower(_: *mut Client);
    #[no_mangle]
    fn hide(_: *mut Client);
    #[no_mangle]
    fn unhide(_: *mut Client);
    #[no_mangle]
    fn fork_exec(_: *mut libc::c_char);
    #[no_mangle]
    fn get_mouse_position(_: *mut libc::c_int, _: *mut libc::c_int);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
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
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MenuItem {
    pub command: *mut libc::c_char,
    pub label: *mut libc::c_char,
    pub x: libc::c_int,
    pub width: libc::c_int,
}
#[no_mangle]
pub static mut taskbar: Window = 0;
#[no_mangle]
pub unsafe extern "C" fn make_taskbar() {
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
                             cursor: 0,}; // force initial highlight
    pattr.override_redirect = 1 as libc::c_int;
    pattr.background_pixel = empty_col.pixel;
    pattr.border_pixel = border_col.pixel;
    pattr.event_mask =
        (1 as libc::c_long) << 20 as libc::c_int |
            (1 as libc::c_long) << 19 as libc::c_int |
            (1 as libc::c_long) << 2 as libc::c_int |
            (1 as libc::c_long) << 15 as libc::c_int |
            (1 as libc::c_long) << 4 as libc::c_int;
    taskbar =
        XCreateWindow(dsply, root, 0 as libc::c_int - 2 as libc::c_int,
                      0 as libc::c_int - 2 as libc::c_int,
                      (*(*(dsply as
                               _XPrivDisplay)).screens.offset(screen as
                                                                  isize)).width
                          as libc::c_uint,
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
                          libc::c_ulong, &mut pattr);
    XMapWindow(dsply, taskbar);
}
#[no_mangle]
pub unsafe extern "C" fn remember_hidden() {
    let mut c: *mut Client = 0 as *mut Client;
    c = head_client;
    while !c.is_null() { (*c).was_hidden = (*c).hidden; c = (*c).next };
}
#[no_mangle]
pub unsafe extern "C" fn forget_hidden() {
    let mut c: *mut Client = 0 as *mut Client;
    c = head_client;
    while !c.is_null() {
        if c == focused_client {
            (*c).was_hidden = (*c).hidden
        } else { (*c).was_hidden = 0 as libc::c_int as libc::c_uint }
        c = (*c).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn lclick_taskbutton(mut old_c: *mut Client,
                                           mut c: *mut Client) {
    if !old_c.is_null() { if (*old_c).was_hidden != 0 { hide(old_c); } }
    if (*c).hidden != 0 {
        unhide(c);
    } else if (*c).was_hidden != 0 { hide(c); } else { raise_lower(c); }
    check_focus(c);
}
#[no_mangle]
pub unsafe extern "C" fn lclick_taskbar(mut x: libc::c_int) {
    let mut ev: XEvent = XEvent{type_: 0,};
    let mut mousex: libc::c_int = 0;
    let mut mousey: libc::c_int = 0;
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
    let mut button_width: libc::c_float = 0.;
    let mut button_clicked: libc::c_uint = 0;
    let mut old_button_clicked: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut c: *mut Client = 0 as *mut Client;
    let mut exposed_c: *mut Client = 0 as *mut Client;
    let mut old_c: *mut Client = 0 as *mut Client;
    if !head_client.is_null() {
        remember_hidden();
        get_mouse_position(&mut mousex, &mut mousey);
        bounddims.x = 0 as libc::c_int;
        bounddims.y = 0 as libc::c_int;
        bounddims.width =
            (*(*(dsply as
                     _XPrivDisplay)).screens.offset(screen as isize)).width;
        bounddims.height =
            (*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int;
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
                              libc::c_uint, 1 as libc::c_int,
                          1 as libc::c_int, constraint_win,
                          0 as libc::c_long as Cursor,
                          0 as libc::c_long as Time) == 0 as libc::c_int) {
            XDestroyWindow(dsply, constraint_win);
            return
        }
        button_width = get_button_width();
        button_clicked = (x as libc::c_float / button_width) as libc::c_uint;
        i = 0 as libc::c_int as libc::c_uint;
        c = head_client;
        while i < button_clicked { c = (*c).next; i = i.wrapping_add(1) }
        lclick_taskbutton(0 as *mut Client, c);
        loop  {
            XMaskEvent(dsply,
                       (1 as libc::c_long) << 15 as libc::c_int |
                           ((1 as libc::c_long) << 2 as libc::c_int |
                                (1 as libc::c_long) << 3 as libc::c_int |
                                (1 as libc::c_long) << 6 as libc::c_int) |
                           ((1 as libc::c_long) << 0 as libc::c_int |
                                (1 as libc::c_long) << 1 as libc::c_int),
                       &mut ev);
            match ev.type_ {
                12 => {
                    exposed_c =
                        find_client(ev.expose.window, 1 as libc::c_int);
                    if !exposed_c.is_null() { redraw(exposed_c); }
                }
                6 => {
                    old_button_clicked = button_clicked;
                    button_clicked =
                        (ev.motion.x as libc::c_float / button_width) as
                            libc::c_uint;
                    if button_clicked != old_button_clicked {
                        old_c = c;
                        i = 0 as libc::c_int as libc::c_uint;
                        c = head_client;
                        while i < button_clicked {
                            c = (*c).next;
                            i = i.wrapping_add(1)
                        }
                        lclick_taskbutton(old_c, c);
                    }
                }
                2 => { XPutBackEvent(dsply, &mut ev); }
                _ => { }
            }
            if !(ev.type_ != 4 as libc::c_int &&
                     ev.type_ != 5 as libc::c_int &&
                     ev.type_ != 2 as libc::c_int) {
                break ;
            }
        }
        XUnmapWindow(dsply, constraint_win);
        XDestroyWindow(dsply, constraint_win);
        XUngrabPointer(dsply, 0 as libc::c_long as Time);
        forget_hidden();
    };
}
#[no_mangle]
pub unsafe extern "C" fn rclick_taskbar(mut x: libc::c_int) {
    let mut ev: XEvent = XEvent{type_: 0,};
    let mut mousex: libc::c_int = 0;
    let mut mousey: libc::c_int = 0;
    let mut bounddims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
    let mut current_item: libc::c_uint =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
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
    get_mouse_position(&mut mousex, &mut mousey);
    bounddims.x = 0 as libc::c_int;
    bounddims.y = 0 as libc::c_int;
    bounddims.width =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).width;
    bounddims.height =
        (*font).ascent + (*font).descent + 2 as libc::c_int * 3 as libc::c_int
            + 2 as libc::c_int;
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
    draw_menubar();
    update_menuitem(2147483647 as libc::c_int);
    current_item = update_menuitem(x);
    loop  {
        XMaskEvent(dsply,
                   (1 as libc::c_long) << 2 as libc::c_int |
                       (1 as libc::c_long) << 3 as libc::c_int |
                       (1 as libc::c_long) << 6 as libc::c_int |
                       ((1 as libc::c_long) << 0 as libc::c_int |
                            (1 as libc::c_long) << 1 as libc::c_int),
                   &mut ev);
        match ev.type_ {
            6 => { current_item = update_menuitem(ev.motion.x) }
            5 => {
                if current_item !=
                       (2147483647 as libc::c_int as
                            libc::c_uint).wrapping_mul(2 as
                                                           libc::c_uint).wrapping_add(1
                                                                                          as
                                                                                          libc::c_uint)
                   {
                    fork_exec((*menuitems.offset(current_item as
                                                     isize)).command);
                }
            }
            2 => { XPutBackEvent(dsply, &mut ev); }
            _ => { }
        }
        if !(ev.type_ != 4 as libc::c_int && ev.type_ != 5 as libc::c_int &&
                 ev.type_ != 2 as libc::c_int) {
            break ;
        }
    }
    redraw_taskbar();
    XUnmapWindow(dsply, constraint_win);
    XDestroyWindow(dsply, constraint_win);
    XUngrabPointer(dsply, 0 as libc::c_long as Time);
}
#[no_mangle]
pub unsafe extern "C" fn rclick_root() {
    let mut ev: XEvent = XEvent{type_: 0,};
    if !(XGrabPointer(dsply, root, 0 as libc::c_int,
                      ((1 as libc::c_long) << 2 as libc::c_int |
                           (1 as libc::c_long) << 3 as libc::c_int |
                           (1 as libc::c_long) << 6 as libc::c_int) as
                          libc::c_uint, 1 as libc::c_int, 1 as libc::c_int,
                      0 as libc::c_long as Window,
                      0 as libc::c_long as Cursor, 0 as libc::c_long as Time)
             == 0 as libc::c_int) {
        return
    }
    draw_menubar();
    loop  {
        XMaskEvent(dsply,
                   (1 as libc::c_long) << 2 as libc::c_int |
                       (1 as libc::c_long) << 3 as libc::c_int |
                       (1 as libc::c_long) << 6 as libc::c_int |
                       ((1 as libc::c_long) << 0 as libc::c_int |
                            (1 as libc::c_long) << 1 as libc::c_int),
                   &mut ev);
        match ev.type_ {
            6 => {
                if ev.motion.y <
                       (*font).ascent + (*font).descent +
                           2 as libc::c_int * 3 as libc::c_int +
                           2 as libc::c_int {
                    XUngrabPointer(dsply, 0 as libc::c_long as Time);
                    rclick_taskbar(ev.motion.x);
                    return
                }
            }
            2 => { XPutBackEvent(dsply, &mut ev); }
            _ => { }
        }
        if !(ev.type_ != 5 as libc::c_int && ev.type_ != 2 as libc::c_int) {
            break ;
        }
    }
    redraw_taskbar();
    XUngrabPointer(dsply, 0 as libc::c_long as Time);
}
#[no_mangle]
pub unsafe extern "C" fn redraw_taskbar() {
    let mut i: libc::c_uint = 0;
    let mut button_startx: libc::c_int = 0;
    let mut button_iwidth: libc::c_int = 0;
    let mut button_width: libc::c_float = 0.;
    let mut c: *mut Client = 0 as *mut Client;
    button_width = get_button_width();
    XClearWindow(dsply, taskbar);
    if showing_taskbar == 0 as libc::c_int as libc::c_uint { return }
    c = head_client;
    i = 0 as libc::c_int as libc::c_uint;
    while !c.is_null() {
        button_startx = (i as libc::c_float * button_width) as libc::c_int;
        button_iwidth =
            (i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_float
                 * button_width - button_startx as libc::c_float) as
                libc::c_uint as libc::c_int;
        if button_startx != 0 as libc::c_int {
            XDrawLine(dsply, taskbar, border_gc,
                      button_startx - 1 as libc::c_int, 0 as libc::c_int,
                      button_startx - 1 as libc::c_int,
                      (*font).ascent + (*font).descent +
                          2 as libc::c_int * 3 as libc::c_int +
                          2 as libc::c_int - 2 as libc::c_int);
        }
        if c == focused_client {
            XFillRectangle(dsply, taskbar, active_gc, button_startx,
                           0 as libc::c_int, button_iwidth as libc::c_uint,
                           ((*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int - 2 as libc::c_int) as
                               libc::c_uint);
        } else {
            XFillRectangle(dsply, taskbar, inactive_gc, button_startx,
                           0 as libc::c_int, button_iwidth as libc::c_uint,
                           ((*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int - 2 as libc::c_int) as
                               libc::c_uint);
        }
        if (*c).trans == 0 && !(*c).name.is_null() {
            XDrawString(dsply, taskbar, text_gc,
                        button_startx + 3 as libc::c_int,
                        3 as libc::c_int + (*font).ascent, (*c).name,
                        strlen((*c).name) as libc::c_int);
        }
        c = (*c).next;
        i = i.wrapping_add(1)
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
unsafe extern "C" fn draw_menubar() {
    let mut i: libc::c_uint = 0; // retain value from last call
    let mut dw: libc::c_uint = 0;
    dw =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).width
            as libc::c_uint;
    XFillRectangle(dsply, taskbar, menu_gc, 0 as libc::c_int,
                   0 as libc::c_int, dw,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint);
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_menuitems {
        if !(*menuitems.offset(i as isize)).label.is_null() &&
               !(*menuitems.offset(i as isize)).command.is_null() {
            XDrawString(dsply, taskbar, text_gc,
                        (*menuitems.offset(i as isize)).x +
                            3 as libc::c_int * 2 as libc::c_int,
                        (*font).ascent + 3 as libc::c_int,
                        (*menuitems.offset(i as isize)).label,
                        strlen((*menuitems.offset(i as isize)).label) as
                            libc::c_int);
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn update_menuitem(mut mousex: libc::c_int)
 -> libc::c_uint {
    static mut last_item: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    if mousex == 2147483647 as libc::c_int {
        // entered function to set last_item
        last_item = num_menuitems;
        return (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_menuitems {
        if mousex >= (*menuitems.offset(i as isize)).x &&
               mousex <=
                   (*menuitems.offset(i as isize)).x +
                       (*menuitems.offset(i as isize)).width {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if i != last_item {
        // don't redraw if same
        if last_item != num_menuitems {
            draw_menuitem(last_item, 0 as libc::c_int as libc::c_uint);
        }
        if i != num_menuitems {
            draw_menuitem(i, 1 as libc::c_int as libc::c_uint);
        }
        last_item = i
        // set to new menu item
    }
    if i != num_menuitems {
        return i
    } else {
        // no item selected
        return (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
    };
}
unsafe extern "C" fn draw_menuitem(mut index: libc::c_uint,
                                   mut active: libc::c_uint) {
    if active != 0 {
        XFillRectangle(dsply, taskbar, selected_gc,
                       (*menuitems.offset(index as isize)).x,
                       0 as libc::c_int,
                       (*menuitems.offset(index as isize)).width as
                           libc::c_uint,
                       ((*font).ascent + (*font).descent +
                            2 as libc::c_int * 3 as libc::c_int +
                            2 as libc::c_int - 2 as libc::c_int) as
                           libc::c_uint);
    } else {
        XFillRectangle(dsply, taskbar, menu_gc,
                       (*menuitems.offset(index as isize)).x,
                       0 as libc::c_int,
                       (*menuitems.offset(index as isize)).width as
                           libc::c_uint,
                       ((*font).ascent + (*font).descent +
                            2 as libc::c_int * 3 as libc::c_int +
                            2 as libc::c_int - 2 as libc::c_int) as
                           libc::c_uint);
    }
    XDrawString(dsply, taskbar, text_gc,
                (*menuitems.offset(index as isize)).x +
                    3 as libc::c_int * 2 as libc::c_int,
                (*font).ascent + 3 as libc::c_int,
                (*menuitems.offset(index as isize)).label,
                strlen((*menuitems.offset(index as isize)).label) as
                    libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn get_button_width() -> libc::c_float {
    let mut nwins: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut c: *mut Client = head_client;
    while !c.is_null() { nwins = nwins.wrapping_add(1); c = (*c).next }
    return ((*(*(dsply as
                     _XPrivDisplay)).screens.offset(screen as isize)).width +
                2 as libc::c_int) as libc::c_float / nwins as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn cycle_previous() {
    let mut c: *mut Client = focused_client;
    let mut original_c: *mut Client = c;
    if !head_client.is_null() && !(*head_client).next.is_null() {
        // at least 2 windows exist
        if c.is_null() { c = head_client }
        if c == head_client { original_c = 0 as *mut Client }
        loop  {
            if (*c).next.is_null() { c = head_client } else { c = (*c).next }
            if !((*c).next != original_c) { break ; }
        }
        lclick_taskbutton(0 as *mut Client, c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cycle_next() {
    let mut c: *mut Client = focused_client;
    if !head_client.is_null() && !(*head_client).next.is_null() {
        // at least 2 windows exist
        if c.is_null() || (*c).next.is_null() {
            c = head_client
        } else { c = (*c).next }
        lclick_taskbutton(0 as *mut Client, c);
    };
}
