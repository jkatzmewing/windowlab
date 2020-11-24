use ::libc;
use x11::xlib::*;

extern "C" {
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn XShapeCombineRectangles(_: *mut Display, _: Window, _: libc::c_int,
                               _: libc::c_int, _: libc::c_int,
                               _: *mut XRectangle, _: libc::c_int,
                               _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn XShapeCombineShape(_: *mut Display, _: Window, _: libc::c_int,
                          _: libc::c_int, _: libc::c_int, _: Window,
                          _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn XShapeGetRectangles(_: *mut Display, _: Window, _: libc::c_int,
                           _: *mut libc::c_int, _: *mut libc::c_int)
     -> *mut XRectangle;
    #[no_mangle]
    static mut dsply: *mut Display;
    #[no_mangle]
    static mut root: Window;
    #[no_mangle]
    static mut head_client: *mut Client;
    #[no_mangle]
    static mut focused_client: *mut Client;
    #[no_mangle]
    static mut fullscreen_client: *mut Client;
    #[no_mangle]
    static mut focus_count: libc::c_uint;
    #[no_mangle]
    static mut font: *mut XFontStruct;
    #[no_mangle]
    static mut border_gc: GC;
    #[no_mangle]
    static mut text_gc: GC;
    #[no_mangle]
    static mut active_gc: GC;
    #[no_mangle]
    static mut inactive_gc: GC;
    #[no_mangle]
    static mut wm_state: Atom;
    #[no_mangle]
    fn redraw_taskbar();
    #[no_mangle]
    fn handle_xerror(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int;
    #[no_mangle]
    fn ignore_xerror(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int;
}
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}

pub type XErrorHandler
    =
    Option<unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent)
               -> libc::c_int>;
pub type CARD32 = libc::c_uint;

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
#[no_mangle]
pub unsafe extern "C" fn find_client(mut w: Window, mut mode: libc::c_int)
 -> *mut Client {
    let mut c: *mut Client = head_client;
    if mode == 1 as libc::c_int {
        while !c.is_null() { if (*c).frame == w { return c } c = (*c).next }
    } else {
        // WINDOW
        while !c.is_null() { if (*c).window == w { return c } c = (*c).next }
    }
    return 0 as *mut Client;
}
/* Attempt to follow the ICCCM by explicitly specifying 32 bits for
 * this property. Does this goof up on 64 bit systems? */
#[no_mangle]
pub unsafe extern "C" fn set_wm_state(mut c: *mut Client,
                                      mut state: libc::c_int) {
    let mut data: [CARD32; 2] =
        [0; 2]; //Icon? We don't need no steenking icon.
    data[0 as libc::c_int as usize] = state as CARD32;
    data[1 as libc::c_int as usize] = 0 as libc::c_long as CARD32;
    XChangeProperty(dsply, (*c).window, wm_state, wm_state, 32 as libc::c_int,
                    0 as libc::c_int, data.as_mut_ptr() as *mut libc::c_uchar,
                    2 as libc::c_int);
}
/* If we can't find a WM_STATE we're going to have to assume
 * Withdrawn. This is not exactly optimal, since we can't really
 * distinguish between the case where no WM has run yet and when the
 * state was explicitly removed (Clients are allowed to either set the
 * atom to Withdrawn or just remove it... yuck.) */
#[no_mangle]
pub unsafe extern "C" fn get_wm_state(mut c: *mut Client) -> libc::c_long {
    let mut real_type: Atom = 0;
    let mut real_format: libc::c_int = 0;
    let mut state: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut items_read: libc::c_ulong = 0;
    let mut items_left: libc::c_ulong = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if XGetWindowProperty(dsply, (*c).window, wm_state, 0 as libc::c_long,
                          2 as libc::c_long, 0 as libc::c_int, wm_state,
                          &mut real_type, &mut real_format, &mut items_read,
                          &mut items_left, &mut data) == 0 as libc::c_int &&
           items_read != 0 {
        state = *(data as *mut libc::c_long);
        XFree(data as *mut libc::c_void);
    }
    return state;
}
/* This will need to be called whenever we update our Client stuff.
 * Yeah, yeah, stop yelling at me about OO. */
#[no_mangle]
pub unsafe extern "C" fn send_config(mut c: *mut Client) {
    let mut ce: XConfigureEvent =
        XConfigureEvent{type_: 0,
                        serial: 0,
                        send_event: 0,
                        display: 0 as *mut Display,
                        event: 0,
                        window: 0,
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                        border_width: 0,
                        above: 0,
                        override_redirect: 0,};
    ce.type_ = 22 as libc::c_int;
    ce.event = (*c).window;
    ce.window = (*c).window;
    ce.x = (*c).x;
    ce.y = (*c).y;
    ce.width = (*c).width;
    ce.height = (*c).height;
    ce.border_width = 0 as libc::c_int;
    ce.above = 0 as libc::c_long as Window;
    ce.override_redirect = 0 as libc::c_int;
    XSendEvent(dsply, (*c).window, 0 as libc::c_int,
               (1 as libc::c_long) << 17 as libc::c_int,
               &mut ce as *mut XConfigureEvent as *mut XEvent);
}
/* After pulling my hair out trying to find some way to tell if a
 * window is still valid, I've decided to instead carefully ignore any
 * errors raised by this function. We know that the X calls are, and
 * we know the only reason why they could fail -- a window has removed
 * itself completely before the Unmap and Destroy events get through
 * the queue to us. It's not absolutely perfect, but it works.
 *
 * The 'withdrawing' argument specifies if the client is actually
 * (destroying itself||being destroyed by us) or if we are merely
 * cleaning up its data structures when we exit mid-session. */
#[no_mangle]
pub unsafe extern "C" fn remove_client(mut c: *mut Client,
                                       mut mode: libc::c_int) {
    let mut p: *mut Client = 0 as *mut Client;
    XGrabServer(dsply);
    XSetErrorHandler(Some(ignore_xerror as
                              unsafe extern "C" fn(_: *mut Display,
                                                   _: *mut XErrorEvent)
                                  -> libc::c_int));
    if mode == 0 as libc::c_int {
        set_wm_state(c, 0 as libc::c_int);
    } else {
        //REMAP
        XMapWindow(dsply, (*c).window);
    }
    gravitate(c, -(1 as libc::c_int));
    XReparentWindow(dsply, (*c).window, root, (*c).x, (*c).y);
    XSetWindowBorderWidth(dsply, (*c).window,
                          1 as libc::c_int as libc::c_uint);
    XRemoveFromSaveSet(dsply, (*c).window);
    XDestroyWindow(dsply, (*c).frame);
    if head_client == c {
        head_client = (*c).next
    } else {
        p = head_client;
        while !p.is_null() && !(*p).next.is_null() {
            if (*p).next == c { (*p).next = (*c).next }
            p = (*p).next
        }
    }
    if !(*c).name.is_null() { XFree((*c).name as *mut libc::c_void); }
    if !(*c).size.is_null() { XFree((*c).size as *mut libc::c_void); }
    if c == fullscreen_client { fullscreen_client = 0 as *mut Client }
    if c == focused_client {
        focused_client = 0 as *mut Client;
        check_focus(get_prev_focused());
    }
    free(c as *mut libc::c_void);
    XSync(dsply, 0 as libc::c_int);
    XSetErrorHandler(Some(handle_xerror as
                              unsafe extern "C" fn(_: *mut Display,
                                                   _: *mut XErrorEvent)
                                  -> libc::c_int));
    XUngrabServer(dsply);
    redraw_taskbar();
}
#[no_mangle]
pub unsafe extern "C" fn redraw(mut c: *mut Client) {
    if c == fullscreen_client { return }
    XDrawLine(dsply, (*c).frame, border_gc, 0 as libc::c_int,
              (*font).ascent + (*font).descent +
                  2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                  2 as libc::c_int + 2 as libc::c_int / 2 as libc::c_int,
              (*c).width,
              (*font).ascent + (*font).descent +
                  2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                  2 as libc::c_int + 2 as libc::c_int / 2 as libc::c_int);
    // clear text part of bar
    if c == focused_client {
        XFillRectangle(dsply, (*c).frame, active_gc, 0 as libc::c_int,
                       0 as libc::c_int,
                       ((*c).width -
                            ((*font).ascent + (*font).descent +
                                 2 as libc::c_int * 3 as libc::c_int +
                                 2 as libc::c_int - 2 as libc::c_int) *
                                3 as libc::c_int) as libc::c_uint,
                       ((*font).ascent + (*font).descent +
                            2 as libc::c_int * 3 as libc::c_int +
                            2 as libc::c_int - 2 as libc::c_int) as
                           libc::c_uint);
    } else {
        XFillRectangle(dsply, (*c).frame, inactive_gc, 0 as libc::c_int,
                       0 as libc::c_int,
                       ((*c).width -
                            ((*font).ascent + (*font).descent +
                                 2 as libc::c_int * 3 as libc::c_int +
                                 2 as libc::c_int - 2 as libc::c_int) *
                                3 as libc::c_int) as libc::c_uint,
                       ((*font).ascent + (*font).descent +
                            2 as libc::c_int * 3 as libc::c_int +
                            2 as libc::c_int - 2 as libc::c_int) as
                           libc::c_uint);
    }
    if (*c).trans == 0 && !(*c).name.is_null() {
        XDrawString(dsply, (*c).frame, text_gc, 3 as libc::c_int,
                    3 as libc::c_int + (*font).ascent, (*c).name,
                    strlen((*c).name) as libc::c_int);
    }
    if c == focused_client {
        draw_hide_button(c, &mut text_gc, &mut active_gc);
        draw_toggledepth_button(c, &mut text_gc, &mut active_gc);
        draw_close_button(c, &mut text_gc, &mut active_gc);
    } else {
        draw_hide_button(c, &mut text_gc, &mut inactive_gc);
        draw_toggledepth_button(c, &mut text_gc, &mut inactive_gc);
        draw_close_button(c, &mut text_gc, &mut inactive_gc);
    };
}
/* Window gravity is a mess to explain, but we don't need to do much
 * about it since we're using X borders. For NorthWest et al, the top
 * left corner of the window when there is no WM needs to match up
 * with the top left of our fram once we manage it, and likewise with
 * SouthWest and the bottom right (these are the only values I ever
 * use, but the others should be obvious). Our titlebar is on the top
 * so we only have to adjust in the first case. */
#[no_mangle]
pub unsafe extern "C" fn gravitate(mut c: *mut Client,
                                   mut multiplier: libc::c_int) {
    let mut dy: libc::c_int = 0 as libc::c_int;
    let mut gravity: libc::c_int =
        if (*(*c).size).flags & (1 as libc::c_long) << 9 as libc::c_int != 0 {
            (*(*c).size).win_gravity
        } else { 1 as libc::c_int };
    match gravity {
        1 | 3 | 2 => {
            dy =
                (*font).ascent + (*font).descent +
                    2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
        }
        5 => {
            dy =
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) /
                    2 as libc::c_int
        }
        _ => { }
    }
    (*c).y += multiplier * dy;
}
/* Well, the man pages for the shape extension say nothing, but I was
 * able to find a shape.PS.Z on the x.org FTP site. What we want to do
 * here is make the window shape be a boolean OR (or union, if you
 * prefer) of the client's shape and our titlebar. The titlebar
 * requires both a bound and a clip because it has a border -- the X
 * server will paint the border in the region between the two. (I knew
 * that using X borders would get me eventually... ;-)) */
#[no_mangle]
pub unsafe extern "C" fn set_shape(mut c: *mut Client) {
    let mut n: libc::c_int = 0;
    let mut order: libc::c_int = 0;
    let mut temp: XRectangle = XRectangle{x: 0, y: 0, width: 0, height: 0,};
    let mut dummy: *mut XRectangle = 0 as *mut XRectangle;
    dummy =
        XShapeGetRectangles(dsply, (*c).window, 0 as libc::c_int, &mut n,
                            &mut order);
    if n > 1 as libc::c_int {
        XShapeCombineShape(dsply, (*c).frame, 0 as libc::c_int,
                           0 as libc::c_int,
                           (*font).ascent + (*font).descent +
                               2 as libc::c_int * 3 as libc::c_int +
                               2 as libc::c_int, (*c).window,
                           0 as libc::c_int, 0 as libc::c_int);
        temp.x = -(2 as libc::c_int) as libc::c_short;
        temp.y = -(2 as libc::c_int) as libc::c_short;
        temp.width =
            ((*c).width + 2 as libc::c_int * 2 as libc::c_int) as
                libc::c_ushort;
        temp.height =
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int +
                 2 as libc::c_int) as libc::c_ushort;
        XShapeCombineRectangles(dsply, (*c).frame, 0 as libc::c_int,
                                0 as libc::c_int, 0 as libc::c_int, &mut temp,
                                1 as libc::c_int, 1 as libc::c_int,
                                3 as libc::c_int);
        temp.x = 0 as libc::c_int as libc::c_short;
        temp.y = 0 as libc::c_int as libc::c_short;
        temp.width = (*c).width as libc::c_ushort;
        temp.height =
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                 2 as libc::c_int) as libc::c_ushort;
        XShapeCombineRectangles(dsply, (*c).frame, 1 as libc::c_int,
                                0 as libc::c_int,
                                (*font).ascent + (*font).descent +
                                    2 as libc::c_int * 3 as libc::c_int +
                                    2 as libc::c_int, &mut temp,
                                1 as libc::c_int, 1 as libc::c_int,
                                3 as libc::c_int);
        (*c).has_been_shaped = 1 as libc::c_int
    } else if (*c).has_been_shaped != 0 {
        // I can't find a 'remove all shaping' function...
        temp.x = -(2 as libc::c_int) as libc::c_short; // 5 being ~half of 9
        temp.y = -(2 as libc::c_int) as libc::c_short; // 6 being ~half of 11
        temp.width =
            ((*c).width + 2 as libc::c_int * 2 as libc::c_int) as
                libc::c_ushort; // 5 being ~half of 9
        temp.height =
            ((*c).height +
                 ((*font).ascent + (*font).descent +
                      2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
                 + 2 as libc::c_int * 2 as libc::c_int) as libc::c_ushort;
        XShapeCombineRectangles(dsply, (*c).frame, 0 as libc::c_int,
                                0 as libc::c_int, 0 as libc::c_int, &mut temp,
                                1 as libc::c_int, 0 as libc::c_int,
                                3 as libc::c_int);
    }
    XFree(dummy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn check_focus(mut c: *mut Client) {
    if !c.is_null() {
        XSetInputFocus(dsply, (*c).window, 0 as libc::c_long as libc::c_int,
                       0 as libc::c_long as Time);
        XInstallColormap(dsply, (*c).cmap);
    }
    if c != focused_client {
        let mut old_focused: *mut Client = focused_client;
        focused_client = c;
        focus_count = focus_count.wrapping_add(1);
        if !c.is_null() { (*c).focus_order = focus_count; redraw(c); }
        if !old_focused.is_null() { redraw(old_focused); }
        redraw_taskbar();
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_prev_focused() -> *mut Client {
    let mut c: *mut Client = head_client;
    let mut prev_focused: *mut Client = 0 as *mut Client;
    let mut highest: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !c.is_null() {
        if (*c).hidden == 0 && (*c).focus_order > highest {
            highest = (*c).focus_order;
            prev_focused = c
        }
        c = (*c).next
    }
    return prev_focused;
}
#[no_mangle]
pub unsafe extern "C" fn draw_hide_button(mut c: *mut Client,
                                          mut detail_gc: *mut GC,
                                          mut background_gc: *mut GC) {
    let mut x: libc::c_int = 0;
    let mut topleft_offset: libc::c_int = 0;
    x =
        (*c).width -
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                 2 as libc::c_int) * 3 as libc::c_int;
    topleft_offset =
        ((*font).ascent + (*font).descent +
             2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) /
            2 as libc::c_int - 5 as libc::c_int;
    XFillRectangle(dsply, (*c).frame, *background_gc, x, 0 as libc::c_int,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 4 as libc::c_int,
              topleft_offset + 2 as libc::c_int,
              x + topleft_offset + 4 as libc::c_int,
              topleft_offset + 0 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 6 as libc::c_int,
              topleft_offset + 2 as libc::c_int,
              x + topleft_offset + 7 as libc::c_int,
              topleft_offset + 1 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 6 as libc::c_int,
              topleft_offset + 4 as libc::c_int,
              x + topleft_offset + 8 as libc::c_int,
              topleft_offset + 4 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 6 as libc::c_int,
              topleft_offset + 6 as libc::c_int,
              x + topleft_offset + 7 as libc::c_int,
              topleft_offset + 7 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 4 as libc::c_int,
              topleft_offset + 6 as libc::c_int,
              x + topleft_offset + 4 as libc::c_int,
              topleft_offset + 8 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 2 as libc::c_int,
              topleft_offset + 6 as libc::c_int,
              x + topleft_offset + 1 as libc::c_int,
              topleft_offset + 7 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 2 as libc::c_int,
              topleft_offset + 4 as libc::c_int,
              x + topleft_offset + 0 as libc::c_int,
              topleft_offset + 4 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 2 as libc::c_int,
              topleft_offset + 2 as libc::c_int,
              x + topleft_offset + 1 as libc::c_int,
              topleft_offset + 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn draw_toggledepth_button(mut c: *mut Client,
                                                 mut detail_gc: *mut GC,
                                                 mut background_gc: *mut GC) {
    let mut x: libc::c_int = 0;
    let mut topleft_offset: libc::c_int = 0;
    x =
        (*c).width -
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                 2 as libc::c_int) * 2 as libc::c_int;
    topleft_offset =
        ((*font).ascent + (*font).descent +
             2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) /
            2 as libc::c_int - 6 as libc::c_int;
    XFillRectangle(dsply, (*c).frame, *background_gc, x, 0 as libc::c_int,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint);
    XDrawRectangle(dsply, (*c).frame, *detail_gc, x + topleft_offset,
                   topleft_offset, 7 as libc::c_int as libc::c_uint,
                   7 as libc::c_int as libc::c_uint);
    XDrawRectangle(dsply, (*c).frame, *detail_gc,
                   x + topleft_offset + 3 as libc::c_int,
                   topleft_offset + 3 as libc::c_int,
                   7 as libc::c_int as libc::c_uint,
                   7 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn draw_close_button(mut c: *mut Client,
                                           mut detail_gc: *mut GC,
                                           mut background_gc: *mut GC) {
    let mut x: libc::c_int = 0;
    let mut topleft_offset: libc::c_int = 0;
    x =
        (*c).width -
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int -
                 2 as libc::c_int);
    topleft_offset =
        ((*font).ascent + (*font).descent +
             2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) /
            2 as libc::c_int - 5 as libc::c_int;
    XFillRectangle(dsply, (*c).frame, *background_gc, x, 0 as libc::c_int,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint,
                   ((*font).ascent + (*font).descent +
                        2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
                        - 2 as libc::c_int) as libc::c_uint);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 1 as libc::c_int, topleft_offset,
              x + topleft_offset + 8 as libc::c_int,
              topleft_offset + 7 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 1 as libc::c_int,
              topleft_offset + 1 as libc::c_int,
              x + topleft_offset + 7 as libc::c_int,
              topleft_offset + 7 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc, x + topleft_offset,
              topleft_offset + 1 as libc::c_int,
              x + topleft_offset + 7 as libc::c_int,
              topleft_offset + 8 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc, x + topleft_offset,
              topleft_offset + 7 as libc::c_int,
              x + topleft_offset + 7 as libc::c_int, topleft_offset);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 1 as libc::c_int,
              topleft_offset + 7 as libc::c_int,
              x + topleft_offset + 7 as libc::c_int,
              topleft_offset + 1 as libc::c_int);
    XDrawLine(dsply, (*c).frame, *detail_gc,
              x + topleft_offset + 1 as libc::c_int,
              topleft_offset + 8 as libc::c_int,
              x + topleft_offset + 8 as libc::c_int,
              topleft_offset + 1 as libc::c_int);
}
