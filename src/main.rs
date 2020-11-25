#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]

extern crate x11;
use x11::xlib::*;
use ::c2rust_out;

extern "C" {
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn get_menuitems();
    #[no_mangle]
    fn make_taskbar();
    #[no_mangle]
    fn XShapeQueryExtension(_: *mut Display, _: *mut libc::c_int,
                            _: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn handle_xerror(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int;
    #[no_mangle]
    fn sig_handler(_: libc::c_int);
    #[no_mangle]
    fn err(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn make_new_client(_: Window);
    #[no_mangle]
    fn do_event_loop();
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub type _XPrivDisplay = *mut C2RustUnnamed_10;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub static mut dsply: *mut Display = 0 as *const Display as *mut Display;
#[no_mangle]
pub static mut root: Window = 0;
#[no_mangle]
pub static mut screen: libc::c_int = 0;
#[no_mangle]
pub static mut font: *mut XFontStruct =
    0 as *const XFontStruct as *mut XFontStruct;
#[no_mangle]
pub static mut string_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut border_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut text_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut active_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut depressed_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut inactive_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut menu_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut selected_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut empty_gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut border_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut text_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut active_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut depressed_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut inactive_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut menu_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut selected_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut empty_col: XColor =
    XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
#[no_mangle]
pub static mut resize_curs: Cursor = 0;
#[no_mangle]
pub static mut wm_state: Atom = 0;
#[no_mangle]
pub static mut wm_change_state: Atom = 0;
#[no_mangle]
pub static mut wm_protos: Atom = 0;
#[no_mangle]
pub static mut wm_delete: Atom = 0;
#[no_mangle]
pub static mut wm_cmapwins: Atom = 0;
#[no_mangle]
pub static mut head_client: *mut Client = 0 as *const Client as *mut Client;
#[no_mangle]
pub static mut focused_client: *mut Client =
    0 as *const Client as *mut Client;
#[no_mangle]
pub static mut topmost_client: *mut Client =
    0 as *const Client as *mut Client;
#[no_mangle]
pub static mut fullscreen_client: *mut Client =
    0 as *const Client as *mut Client;
#[no_mangle]
pub static mut in_taskbar: libc::c_uint = 0 as libc::c_int as libc::c_uint;
// actually, we don't know yet
#[no_mangle]
pub static mut showing_taskbar: libc::c_uint =
    1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut focus_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut fs_prevdims: Rect = Rect{x: 0, y: 0, width: 0, height: 0,};
#[no_mangle]
pub static mut opt_font: *mut libc::c_char =
    b"-b&h-lucida-medium-r-*-*-10-*-*-*-*-*-*-*\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_border: *mut libc::c_char =
    b"#000\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_text: *mut libc::c_char =
    b"#000\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_active: *mut libc::c_char =
    b"#fd0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_inactive: *mut libc::c_char =
    b"#aaa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_menu: *mut libc::c_char =
    b"#ddd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_selected: *mut libc::c_char =
    b"#aad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_empty: *mut libc::c_char =
    b"#000\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut opt_display: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut shape: libc::c_int = 0;
#[no_mangle]
pub static mut shape_event: libc::c_int = 0;
#[no_mangle]
pub static mut numlockmask: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut act: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_9{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-font\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_font = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-border\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_border = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-text\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_text = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-active\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_active = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-inactive\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_inactive = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-menu\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_menu = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-selected\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_selected = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-empty\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_empty = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-display\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int && (i + 1 as libc::c_int) < argc {
            i += 1;
            opt_display = *argv.offset(i as isize)
        } else {
            if strcmp(*argv.offset(i as isize),
                      b"-about\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                printf(b"WindowLab 1.40 (2010-04-04), Copyright (c) 2001-2009 Nick Gravgaard\nWindowLab comes with ABSOLUTELY NO WARRANTY.\nThis is free software, and you are welcome to redistribute it\nunder certain conditions; view the LICENCE file for details.\n\x00"
                           as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            // shouldn't get here; must be a bad option
            err(b"usage:\n  windowlab [options]\n\noptions are:\n  -font <font>\n  -border|-text|-active|-inactive|-menu|-selected|-empty <color>\n  -about\n  -display <display>\x00"
                    as *const u8 as *const libc::c_char);
            return 2 as libc::c_int
        }
        i += 1
    }
    act.__sigaction_handler.sa_handler =
        Some(sig_handler as unsafe extern "C" fn(_: libc::c_int) -> ());
    act.sa_flags = 0 as libc::c_int;
    sigaction(15 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(1 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(17 as libc::c_int, &mut act, 0 as *mut sigaction);
    setup_display();
    get_menuitems();
    make_taskbar();
    scan_wins();
    do_event_loop();
    return 1 as libc::c_int;
    // just another brick in the -Wall
}
unsafe extern "C" fn scan_wins() {
    let mut nwins: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut dummyw1: Window = 0;
    let mut dummyw2: Window = 0;
    let mut wins: *mut Window = 0 as *mut Window;
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
    XQueryTree(dsply, root, &mut dummyw1, &mut dummyw2, &mut wins,
               &mut nwins);
    i = 0 as libc::c_int as libc::c_uint;
    while i < nwins {
        XGetWindowAttributes(dsply, *wins.offset(i as isize), &mut attr);
        if attr.override_redirect == 0 && attr.map_state == 2 as libc::c_int {
            make_new_client(*wins.offset(i as isize));
        }
        i = i.wrapping_add(1)
    }
    XFree(wins as *mut libc::c_void);
}
unsafe extern "C" fn setup_display() {
    let mut dummyc: XColor =
        XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
    let mut gv: XGCValues =
        XGCValues{function: 0,
                  plane_mask: 0,
                  foreground: 0,
                  background: 0,
                  line_width: 0,
                  line_style: 0,
                  cap_style: 0,
                  join_style: 0,
                  fill_style: 0,
                  fill_rule: 0,
                  arc_mode: 0,
                  tile: 0,
                  stipple: 0,
                  ts_x_origin: 0,
                  ts_y_origin: 0,
                  font: 0,
                  subwindow_mode: 0,
                  graphics_exposures: 0,
                  clip_x_origin: 0,
                  clip_y_origin: 0,
                  clip_mask: 0,
                  dash_offset: 0,
                  dashes: 0,};
    let mut sattr: XSetWindowAttributes =
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
    let mut modmap: *mut XModifierKeymap = 0 as *mut XModifierKeymap;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    dsply = XOpenDisplay(opt_display);
    if dsply.is_null() {
        err(b"can\'t open display! check your DISPLAY variable.\x00" as
                *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    XSetErrorHandler(Some(handle_xerror as
                              unsafe extern "C" fn(_: *mut Display,
                                                   _: *mut XErrorEvent)
                                  -> libc::c_int));
    screen = (*(dsply as _XPrivDisplay)).default_screen;
    root =
        (*(*(dsply as _XPrivDisplay)).screens.offset(screen as isize)).root;
    wm_state =
        XInternAtom(dsply,
                    b"WM_STATE\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int);
    wm_change_state =
        XInternAtom(dsply,
                    b"WM_CHANGE_STATE\x00" as *const u8 as
                        *const libc::c_char, 0 as libc::c_int);
    wm_protos =
        XInternAtom(dsply,
                    b"WM_PROTOCOLS\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int);
    wm_delete =
        XInternAtom(dsply,
                    b"WM_DELETE_WINDOW\x00" as *const u8 as
                        *const libc::c_char, 0 as libc::c_int);
    wm_cmapwins =
        XInternAtom(dsply,
                    b"WM_COLORMAP_WINDOWS\x00" as *const u8 as
                        *const libc::c_char, 0 as libc::c_int);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_border, &mut border_col, &mut dummyc);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_text, &mut text_col, &mut dummyc);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_active, &mut active_col, &mut dummyc);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_inactive, &mut inactive_col, &mut dummyc);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_menu, &mut menu_col, &mut dummyc);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_selected, &mut selected_col, &mut dummyc);
    XAllocNamedColor(dsply,
                     (*(*(dsply as
                              _XPrivDisplay)).screens.offset(screen as
                                                                 isize)).cmap,
                     opt_empty, &mut empty_col, &mut dummyc);
    depressed_col.pixel = active_col.pixel;
    depressed_col.red =
        (active_col.red as libc::c_int - 0x2000 as libc::c_int) as
            libc::c_ushort;
    depressed_col.green =
        (active_col.green as libc::c_int - 0x2000 as libc::c_int) as
            libc::c_ushort;
    depressed_col.blue =
        (active_col.blue as libc::c_int - 0x2000 as libc::c_int) as
            libc::c_ushort;
    depressed_col.red =
        if depressed_col.red as libc::c_int <=
               32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int -
                   0x2000 as libc::c_int {
            depressed_col.red as libc::c_int
        } else { 0 as libc::c_int } as libc::c_ushort;
    depressed_col.green =
        if depressed_col.green as libc::c_int <=
               32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int -
                   0x2000 as libc::c_int {
            depressed_col.green as libc::c_int
        } else { 0 as libc::c_int } as libc::c_ushort;
    depressed_col.blue =
        if depressed_col.blue as libc::c_int <=
               32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int -
                   0x2000 as libc::c_int {
            depressed_col.blue as libc::c_int
        } else { 0 as libc::c_int } as libc::c_ushort;
    XAllocColor(dsply,
                (*(*(dsply as
                         _XPrivDisplay)).screens.offset(screen as
                                                            isize)).cmap,
                &mut depressed_col);
    font = XLoadQueryFont(dsply, opt_font);
    if font.is_null() {
        err(b"XLoadQueryFont(): font \'%s\' not found\x00" as *const u8 as
                *const libc::c_char, opt_font);
        exit(1 as libc::c_int);
    }
    shape = XShapeQueryExtension(dsply, &mut shape_event, &mut dummy);
    resize_curs = XCreateFontCursor(dsply, 52 as libc::c_int as libc::c_uint);
    /* find out which modifier is NumLock - we'll use this when grabbing every combination of modifiers we can think of */
    modmap = XGetModifierMapping(dsply);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*modmap).max_keypermod {
            if *(*modmap).modifiermap.offset((i * (*modmap).max_keypermod + j)
                                                 as isize) as libc::c_int ==
                   XKeysymToKeycode(dsply, 0xff7f as libc::c_int as KeySym) as
                       libc::c_int {
                numlockmask = ((1 as libc::c_int) << i) as libc::c_uint
            }
            j += 1
        }
        i += 1
    }
    XFree(modmap as *mut libc::c_void);
    gv.function = 0x3 as libc::c_int;
    gv.foreground = border_col.pixel;
    gv.line_width = 2 as libc::c_int;
    border_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int |
                       (1 as libc::c_long) << 4 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = text_col.pixel;
    gv.line_width = 1 as libc::c_int;
    gv.font = (*font).fid;
    text_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int |
                       (1 as libc::c_long) << 14 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = active_col.pixel;
    active_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = depressed_col.pixel;
    depressed_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = inactive_col.pixel;
    inactive_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = menu_col.pixel;
    menu_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = selected_col.pixel;
    selected_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    gv.foreground = empty_col.pixel;
    empty_gc =
        XCreateGC(dsply, root,
                  ((1 as libc::c_long) << 0 as libc::c_int |
                       (1 as libc::c_long) << 2 as libc::c_int) as
                      libc::c_ulong, &mut gv);
    sattr.event_mask =
        (1 as libc::c_long) << 20 as libc::c_int |
            (1 as libc::c_long) << 19 as libc::c_int |
            (1 as libc::c_long) << 23 as libc::c_int |
            ((1 as libc::c_long) << 2 as libc::c_int |
                 (1 as libc::c_long) << 3 as libc::c_int);
    XChangeWindowAttributes(dsply, root,
                            ((1 as libc::c_long) << 11 as libc::c_int) as
                                libc::c_ulong, &mut sattr);
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0xff09 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint, root,
             1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0xff09 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 1 as libc::c_int |
                  (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
             root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    if numlockmask != 0 {
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0xff09 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0xff09 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                     |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    }
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0x71 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint, root,
             1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0x71 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 1 as libc::c_int |
                  (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
             root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    if numlockmask != 0 {
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0x71 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0x71 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                     |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    }
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0xffc8 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint, root,
             1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0xffc8 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 1 as libc::c_int |
                  (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
             root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    if numlockmask != 0 {
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0xffc8 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0xffc8 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                     |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    }
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0xffc9 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint, root,
             1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    XGrabKey(dsply,
             XKeysymToKeycode(dsply, 0xffc9 as libc::c_int as KeySym) as
                 libc::c_int,
             ((1 as libc::c_int) << 1 as libc::c_int |
                  (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
             root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    if numlockmask != 0 {
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0xffc9 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
        XGrabKey(dsply,
                 XKeysymToKeycode(dsply, 0xffc9 as libc::c_int as KeySym) as
                     libc::c_int,
                 numlockmask |
                     ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                     |
                     ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                 root, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    };
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
