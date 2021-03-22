use ::libc;
use x11::xlib::*;

extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn get_mouse_position(_: *mut libc::c_int, _: *mut libc::c_int);
    #[no_mangle]
    fn fix_position(_: *mut Client);
    #[no_mangle]
    fn XShapeSelectInput(_: *mut Display, _: Window, _: libc::c_ulong);
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
    static mut topmost_client: *mut Client;
    #[no_mangle]
    static mut font: *mut XFontStruct;
    #[no_mangle]
    static mut border_col: XColor;
    #[no_mangle]
    static mut empty_col: XColor;
    #[no_mangle]
    static mut shape: libc::c_int;
    #[no_mangle]
    fn set_wm_state(_: *mut Client, _: libc::c_int);
    #[no_mangle]
    fn get_wm_state(_: *mut Client) -> libc::c_long;
    #[no_mangle]
    fn send_config(_: *mut Client);
    #[no_mangle]
    fn gravitate(_: *mut Client, _: libc::c_int);
    #[no_mangle]
    fn set_shape(_: *mut Client);
    #[no_mangle]
    fn check_focus(_: *mut Client);
    #[no_mangle]
    fn redraw_taskbar();
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
pub struct C2RustUnnamed_0 {
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
/* Set up a client structure for the new (not-yet-mapped) window. The
 * confusing bit is that we have to ignore 2 unmap events if the
 * client was already mapped but has IconicState set (for instance,
 * when we are the second window manager in a session). That's
 * because there's one for the reparent (which happens on all viewable
 * windows) and then another for the unmapping itself. */
#[no_mangle]
pub unsafe extern "C" fn make_new_client(mut w: Window) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut p: *mut Client = 0 as *mut Client;
    let mut attr: XWindowAttributes =
        XWindowAttributes{x: 0,
                          y: 0,
                          width: 0,
                          height: 0,
                          border_width: 0,
                          depth: 0,
                          visual: 0 as *mut Visual,
                          root: 0,
                          class: 0,
                          bit_gravity: 0,
                          win_gravity: 0,
                          backing_store: 0,
                          backing_planes: 0,
                          backing_pixel: 0,
                          save_under: 0,
                          colormap: 0,
                          map_installed: 0,
                          map_state: 0,
                          all_event_masks: 0,
                          your_event_mask: 0,
                          do_not_propagate_mask: 0,
                          override_redirect: 0,
                          screen: 0 as *mut Screen,};
    let mut hints: *mut XWMHints = 0 as *mut XWMHints;
    let mut dummy: libc::c_long = 0;
    c =
        malloc(::std::mem::size_of::<Client>() as libc::c_ulong) as
            *mut Client;
    if head_client.is_null() {
        head_client = c
    } else {
        p = head_client;
        while !(*p).next.is_null() { p = (*p).next }
        (*p).next = c
    }
    (*c).next = 0 as *mut Client;
    XGrabServer(dsply);
    XGetTransientForHint(dsply, w, &mut (*c).trans);
    XFetchName(dsply, w, &mut (*c).name);
    XGetWindowAttributes(dsply, w, &mut attr);
    (*c).window = w;
    (*c).ignore_unmap = 0 as libc::c_int;
    (*c).hidden = 0 as libc::c_int as libc::c_uint;
    (*c).was_hidden = 0 as libc::c_int as libc::c_uint;
    (*c).has_been_shaped = 0 as libc::c_int;
    (*c).x = attr.x;
    (*c).y = attr.y;
    (*c).width = attr.width;
    (*c).height = attr.height;
    (*c).cmap = attr.colormap;
    (*c).size = XAllocSizeHints();
    XGetWMNormalHints(dsply, (*c).window, (*c).size, &mut dummy);
    // XReparentWindow seems to try an XUnmapWindow, regardless of whether the reparented window is mapped or not
    (*c).ignore_unmap += 1;
    if attr.map_state != 2 as libc::c_int {
        init_position(c);
        set_wm_state(c, 1 as libc::c_int);
        hints = XGetWMHints(dsply, w);
        if !hints.is_null() {
            if (*hints).flags & (1 as libc::c_long) << 1 as libc::c_int != 0 {
                set_wm_state(c, (*hints).initial_state);
            }
            XFree(hints as *mut libc::c_void);
        }
    }
    fix_position(c);
    gravitate(c, 1 as libc::c_int);
    reparent(c);
    if get_wm_state(c) != 3 as libc::c_int as libc::c_long {
        XMapWindow(dsply, (*c).window);
        XMapRaised(dsply, (*c).frame);
        topmost_client = c
    } else {
        (*c).hidden = 1 as libc::c_int as libc::c_uint;
        if attr.map_state == 2 as libc::c_int {
            (*c).ignore_unmap += 1;
            XUnmapWindow(dsply, (*c).window);
        }
    }
    // if no client has focus give focus to the new client
    if focused_client.is_null() { check_focus(c); focused_client = c }
    XSync(dsply, 0 as libc::c_int);
    XUngrabServer(dsply);
    redraw_taskbar();
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
/* This one does *not* free the data coming back from Xlib; it just
 * sends back the pointer to what was allocated. */
/* Figure out where to map the window. c->x, c->y, c->width, and
 * c->height actually start out with values in them (whatever the
 * client passed to XCreateWindow).
 *
 * The ICCM says that there are no position/size fields anymore and
 * the SetWMNormalHints says that they are obsolete, so we use the values
 * we got from the window attributes
 * We honour both program and user preferences
 *
 * If we can't find a reasonable position hint, we make up a position
 * using the relative mouse co-ordinates and window size. To account
 * for window gravity while doing this, we add BARHEIGHT() into the
 * calculation and then degravitate. Don't think about it too hard, or
 * your head will explode. */
unsafe extern "C" fn init_position(mut c: *mut Client) {
    let mut mousex: libc::c_int = 0;
    let mut mousey: libc::c_int = 0;
    // make sure it's big enough for the 3 buttons and a bit of bar
    if (*c).width <
           4 as libc::c_int *
               ((*font).ascent + (*font).descent +
                    2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) {
        (*c).width =
            4 as libc::c_int *
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
    }
    if (*c).height <
           (*font).ascent + (*font).descent +
               2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int {
        (*c).height =
            (*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
    }
    if (*c).x == 0 as libc::c_int && (*c).y == 0 as libc::c_int {
        get_mouse_position(&mut mousex, &mut mousey);
        (*c).x = mousex;
        (*c).y =
            mousey +
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int);
        gravitate(c, -(1 as libc::c_int));
    };
}
unsafe extern "C" fn reparent(mut c: *mut Client) {
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
    pattr.override_redirect = 1 as libc::c_int;
    pattr.background_pixel = empty_col.pixel;
    pattr.border_pixel = border_col.pixel;
    pattr.event_mask =
        (1 as libc::c_long) << 20 as libc::c_int |
            (1 as libc::c_long) << 19 as libc::c_int |
            (1 as libc::c_long) << 2 as libc::c_int |
            (1 as libc::c_long) << 15 as libc::c_int |
            (1 as libc::c_long) << 4 as libc::c_int;
    (*c).frame =
        XCreateWindow(dsply, root, (*c).x,
                      (*c).y -
                          ((*font).ascent + (*font).descent +
                               2 as libc::c_int * 3 as libc::c_int +
                               2 as libc::c_int), (*c).width as libc::c_uint,
                      ((*c).height +
                           ((*font).ascent + (*font).descent +
                                2 as libc::c_int * 3 as libc::c_int +
                                2 as libc::c_int)) as libc::c_uint,
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
                          libc::c_ulong, &mut pattr);
    if shape != 0 {
        XShapeSelectInput(dsply, (*c).window,
                          ((1 as libc::c_long) << 0 as libc::c_int) as
                              libc::c_ulong);
        set_shape(c);
    }
    XAddToSaveSet(dsply, (*c).window);
    XSelectInput(dsply, (*c).window,
                 (1 as libc::c_long) << 23 as libc::c_int |
                     (1 as libc::c_long) << 22 as libc::c_int);
    XSetWindowBorderWidth(dsply, (*c).window,
                          0 as libc::c_int as libc::c_uint);
    XResizeWindow(dsply, (*c).window, (*c).width as libc::c_uint,
                  (*c).height as libc::c_uint);
    XReparentWindow(dsply, (*c).window, (*c).frame, 0 as libc::c_int,
                    (*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int +
                        2 as libc::c_int);
    send_config(c);
}
