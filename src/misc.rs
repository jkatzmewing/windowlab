use ::libc;
use x11::xlib::*;

extern "C" {
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn XSendEvent(_: *mut Display, _: Window, _: libc::c_int, _: libc::c_long,
                  _: *mut XEvent) -> libc::c_int;
    #[no_mangle]
    fn free_menuitems();
    #[no_mangle]
    static mut do_menuitems: libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn remove_client(_: *mut Client, _: libc::c_int);
    #[no_mangle]
    fn find_client(_: Window, _: libc::c_int) -> *mut Client;
    #[no_mangle]
    static mut dsply: *mut Display;
    #[no_mangle]
    static mut root: Window;
    #[no_mangle]
    static mut screen: libc::c_int;
    #[no_mangle]
    static mut fullscreen_client: *mut Client;
    #[no_mangle]
    static mut font: *mut XFontStruct;
    #[no_mangle]
    static mut border_gc: GC;
    #[no_mangle]
    static mut text_gc: GC;
    #[no_mangle]
    static mut resize_curs: Cursor;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Cursor = XID;
pub type Colormap = XID;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
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
pub unsafe extern "C" fn fork_exec(mut cmd: *mut libc::c_char) {
    let mut envshell: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envshellname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pid: pid_t = fork();
    match pid {
        0 => {
            setsid();
            envshell =
                getenv(b"SHELL\x00" as *const u8 as *const libc::c_char);
            if envshell.is_null() {
                envshell =
                    b"/bin/sh\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            envshellname = strrchr(envshell, '/' as i32);
            if envshellname.is_null() {
                envshellname = envshell
            } else {
                /* move to the character after the slash */
                envshellname = envshellname.offset(1)
            }
            execlp(envshell, envshellname,
                   b"-c\x00" as *const u8 as *const libc::c_char, cmd,
                   0 as *mut libc::c_void);
            eprintln!("exec failed, cleaning up child");
            exit(1 as libc::c_int);
        }
        -1 => { eprintln!("can\'t fork"); }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sig_handler(mut signal: libc::c_int) {
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    match signal {
        2 | 15 => { quit_nicely(); }
        1 => { do_menuitems = 1 as libc::c_int }
        17 => {
            loop  {
                pid =
                    waitpid(-(1 as libc::c_int), &mut status,
                            1 as libc::c_int);
                if !(pid != 0 as libc::c_int) { break ; }
                if pid == -(1 as libc::c_int) &&
                       *__errno_location() != 4 as libc::c_int {
                    break ;
                }
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_xerror(mut dsply_0: *mut Display,
                                       mut e: *mut XErrorEvent)
 -> libc::c_int {
    let mut c: *mut Client = find_client((*e).resourceid, 0 as libc::c_int);
    if (*e).error_code as libc::c_int == 10 as libc::c_int &&
           (*e).resourceid == root {
        eprintln!("root window unavailable (maybe another wm is running?)");
        exit(1 as libc::c_int);
    } else {
        let mut msg: [libc::c_char; 255] = [0; 255];
        XGetErrorText(dsply_0, (*e).error_code as libc::c_int,
                      msg.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 255]>() as
                          libc::c_ulong as libc::c_int);
        eprintln!("X error ({}): {}", (*e).resourceid, msg.as_mut_ptr());
    }
    if !c.is_null() { remove_client(c, 0 as libc::c_int); }
    return 0 as libc::c_int;
}
/* Ick. Argh. You didn't see this function. */
#[no_mangle]
pub unsafe extern "C" fn ignore_xerror(mut _dsply_0: *mut Display,
                                       mut _e: *mut XErrorEvent)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/* Currently, only send_wm_delete uses this one... */
#[no_mangle]
pub unsafe extern "C" fn send_xmessage(mut w: Window, mut a: Atom,
                                       mut x: libc::c_long) -> libc::c_int {
    let mut d = ClientMessageData::new();
    d.set_byte(0, 0);
    d.set_byte(1, 20);

    let mut e: XClientMessageEvent =
        XClientMessageEvent{type_: 0,
                            serial: 0,
                            send_event: 0,
                            display: 0 as *mut Display,
                            window: 0,
                            message_type: 0,
                            format: 0,
                            data: d,};
    e.type_ = 33 as libc::c_int;
    e.window = w;
    e.message_type = a;
    e.format = 32 as libc::c_int;
    e.data.set_long(0, x);
    e.data.set_long(1, 0);
    return XSendEvent(dsply, w, 0 as libc::c_int, 0 as libc::c_long,
                      &mut e as *mut XClientMessageEvent as *mut XEvent);
}
#[no_mangle]
pub unsafe extern "C" fn get_mouse_position(mut x: *mut libc::c_int,
                                            mut y: *mut libc::c_int) {
    let mut mouse_root: Window = 0;
    let mut mouse_win: Window = 0;
    let mut win_x: libc::c_int = 0;
    let mut win_y: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    XQueryPointer(dsply, root, &mut mouse_root, &mut mouse_win, x, y,
                  &mut win_x, &mut win_y, &mut mask);
}
/* If this is the fullscreen client we don't take BARHEIGHT() into account
 * because the titlebar isn't being drawn on the window. */
#[no_mangle]
pub unsafe extern "C" fn fix_position(mut c: *mut Client) {
    let mut xmax: libc::c_int =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).width;
    let mut ymax: libc::c_int =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).height;
    let mut titlebarheight: libc::c_int = 0;
    titlebarheight =
        if fullscreen_client == c {
            0 as libc::c_int
        } else {
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int) + 2 as libc::c_int
        };
    if (*c).width <
           ((*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
               4 as libc::c_int {
        (*c).width =
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
                4 as libc::c_int
    }
    if (*c).height <
           ((*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
               4 as libc::c_int {
        (*c).height =
            ((*font).ascent + (*font).descent +
                 2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) *
                4 as libc::c_int
    }
    if (*c).width > xmax { (*c).width = xmax }
    if (*c).height +
           ((*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int +
                titlebarheight) > ymax {
        (*c).height =
            ymax -
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int +
                     titlebarheight)
    }
    if (*c).x < 0 as libc::c_int { (*c).x = 0 as libc::c_int }
    if (*c).y <
           (*font).ascent + (*font).descent +
               2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int {
        (*c).y =
            (*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int
    }
    if (*c).x + (*c).width + 2 as libc::c_int >= xmax {
        (*c).x = xmax - (*c).width
    }
    if (*c).y + (*c).height +
           ((*font).ascent + (*font).descent +
                2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) >=
           ymax {
        (*c).y =
            ymax - (*c).height -
                ((*font).ascent + (*font).descent +
                     2 as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
    }
    (*c).x -= 2 as libc::c_int;
    (*c).y -= 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn refix_position(mut c: *mut Client,
                                        mut e: *mut XConfigureRequestEvent) {
    let mut olddims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
    olddims.x = (*c).x - 2 as libc::c_int;
    olddims.y = (*c).y - 2 as libc::c_int;
    olddims.width = (*c).width;
    olddims.height = (*c).height;
    fix_position(c);
    if olddims.x != (*c).x {
        (*e).value_mask |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
    }
    if olddims.y != (*c).y {
        (*e).value_mask |=
            ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
    }
    if olddims.width != (*c).width {
        (*e).value_mask |=
            ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
    }
    if olddims.height != (*c).height {
        (*e).value_mask |=
            ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
    };
}
#[no_mangle]
pub unsafe extern "C" fn copy_dims(mut sourcedims: *mut Rect,
                                   mut destdims: *mut Rect) {
    (*destdims).x = (*sourcedims).x;
    (*destdims).y = (*sourcedims).y;
    (*destdims).width = (*sourcedims).width;
    (*destdims).height = (*sourcedims).height;
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
/* We use XQueryTree here to preserve the window stacking order,
 * since the order in our linked list is different. */
unsafe extern "C" fn quit_nicely() {
    let mut nwins: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut dummyw1: Window = 0;
    let mut dummyw2: Window = 0;
    let mut wins: *mut Window = 0 as *mut Window;
    let mut c: *mut Client = 0 as *mut Client;
    free_menuitems();
    XQueryTree(dsply, root, &mut dummyw1, &mut dummyw2, &mut wins,
               &mut nwins);
    i = 0 as libc::c_int as libc::c_uint;
    while i < nwins {
        c = find_client(*wins.offset(i as isize), 1 as libc::c_int);
        if !c.is_null() { remove_client(c, 1 as libc::c_int); }
        i = i.wrapping_add(1)
    }
    XFree(wins as *mut libc::c_void);
    if !font.is_null() { XFreeFont(dsply, font); }
    XFreeCursor(dsply, resize_curs);
    XFreeGC(dsply, border_gc);
    XFreeGC(dsply, text_gc);
    XInstallColormap(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap);
    XSetInputFocus(dsply, 1 as libc::c_long as Window,
                   0 as libc::c_long as libc::c_int,
                   0 as libc::c_long as Time);
    XCloseDisplay(dsply);
    exit(0 as libc::c_int);
}
