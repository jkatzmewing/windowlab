use ::libc;
use x11::xlib::*;

extern "C" {
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char,
                __len: size_t) -> ssize_t;
    #[no_mangle]
    static mut font: *mut XFontStruct;
    #[no_mangle]
    fn err(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type ssize_t = __ssize_t;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type Font = XID;
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
pub struct MenuItem {
    pub command: *mut libc::c_char,
    pub label: *mut libc::c_char,
    pub x: libc::c_int,
    pub width: libc::c_int,
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
// semaphor activated by SIGHUP
#[no_mangle]
pub static mut do_menuitems: libc::c_int = 0;
#[no_mangle]
pub static mut menuitems: *mut MenuItem =
    0 as *const MenuItem as *mut MenuItem;
#[no_mangle]
pub static mut num_menuitems: libc::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn get_menuitems() {
    let mut i: libc::c_uint = 0;
    let mut button_startx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut menufile: *mut FILE = 0 as *mut FILE;
    let mut menurcpath: [libc::c_char; 4096] = [0; 4096];
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    menuitems =
        malloc((::std::mem::size_of::<MenuItem>() as
                    libc::c_ulong).wrapping_mul(24 as libc::c_int as
                                                    libc::c_ulong)) as
            *mut MenuItem;
    if menuitems.is_null() {
        err(b"Unable to allocate menu items array.\x00" as *const u8 as
                *const libc::c_char);
        return
    }
    memset(menuitems as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<MenuItem>() as
                libc::c_ulong).wrapping_mul(24 as libc::c_int as
                                                libc::c_ulong));
    snprintf(menurcpath.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
             b"%s/.windowlab/windowlab.menurc\x00" as *const u8 as
                 *const libc::c_char,
             getenv(b"HOME\x00" as *const u8 as *const libc::c_char));
    menufile =
        fopen(menurcpath.as_mut_ptr(),
              b"r\x00" as *const u8 as *const libc::c_char);
    if menufile.is_null() {
        let mut len: ssize_t = 0;
        // get location of the executable
        len =
            readlink(b"/proc/self/exe\x00" as *const u8 as
                         *const libc::c_char, menurcpath.as_mut_ptr(),
                     (4096 as libc::c_int - 1 as libc::c_int) as size_t);
        if len == -(1 as libc::c_int) as libc::c_long {
            err(b"readlink() /proc/self/exe failed: %s\n\x00" as *const u8 as
                    *const libc::c_char, strerror(*__errno_location()));
            menurcpath[0 as libc::c_int as usize] =
                '.' as i32 as libc::c_char;
            menurcpath[1 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        } else {
            // insert null to end the file path properly
            menurcpath[len as usize] = '\u{0}' as i32 as libc::c_char
        }
        c = strrchr(menurcpath.as_mut_ptr(), '/' as i32);
        if !c.is_null() { *c = '\u{0}' as i32 as libc::c_char }
        c = strrchr(menurcpath.as_mut_ptr(), '/' as i32);
        if !c.is_null() { *c = '\u{0}' as i32 as libc::c_char }
        strncat(menurcpath.as_mut_ptr(),
                b"/etc/windowlab.menurc\x00" as *const u8 as
                    *const libc::c_char,
                (4096 as libc::c_int as
                     libc::c_ulong).wrapping_sub(strlen(menurcpath.as_mut_ptr())).wrapping_sub(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong));
        menufile =
            fopen(menurcpath.as_mut_ptr(),
                  b"r\x00" as *const u8 as *const libc::c_char);
        if menufile.is_null() {
            menufile =
                fopen(b"/usr/local/etc/X11/windowlab/windowlab.menurc\x00" as
                          *const u8 as *const libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char)
        }
    }
    if !menufile.is_null() {
        num_menuitems = 0 as libc::c_int as libc::c_uint;
        while feof(menufile) == 0 && ferror(menufile) == 0 &&
                  num_menuitems < 24 as libc::c_int as libc::c_uint {
            let mut menustr: [libc::c_char; 128] =
                *::std::mem::transmute::<&[u8; 128],
                                         &mut [libc::c_char; 128]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
            fgets(menustr.as_mut_ptr(), 128 as libc::c_int, menufile);
            if strlen(menustr.as_mut_ptr()) !=
                   0 as libc::c_int as libc::c_ulong {
                let mut pmenustr: *mut libc::c_char = menustr.as_mut_ptr();
                while *pmenustr.offset(0 as libc::c_int as isize) as
                          libc::c_int == ' ' as i32 ||
                          *pmenustr.offset(0 as libc::c_int as isize) as
                              libc::c_int == '\t' as i32 {
                    pmenustr = pmenustr.offset(1)
                }
                if *pmenustr.offset(0 as libc::c_int as isize) as libc::c_int
                       != '#' as i32 {
                    let mut labelstr: [libc::c_char; 128] =
                        *::std::mem::transmute::<&[u8; 128],
                                                 &mut [libc::c_char; 128]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                    let mut commandstr: [libc::c_char; 128] =
                        *::std::mem::transmute::<&[u8; 128],
                                                 &mut [libc::c_char; 128]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                    if parseline(pmenustr, labelstr.as_mut_ptr(),
                                 commandstr.as_mut_ptr()) != 0 {
                        let ref mut fresh0 =
                            (*menuitems.offset(num_menuitems as isize)).label;
                        *fresh0 =
                            malloc(strlen(labelstr.as_mut_ptr()).wrapping_add(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong))
                                as *mut libc::c_char;
                        let ref mut fresh1 =
                            (*menuitems.offset(num_menuitems as
                                                   isize)).command;
                        *fresh1 =
                            malloc(strlen(commandstr.as_mut_ptr()).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                                as *mut libc::c_char;
                        strcpy((*menuitems.offset(num_menuitems as
                                                      isize)).label,
                               labelstr.as_mut_ptr());
                        strcpy((*menuitems.offset(num_menuitems as
                                                      isize)).command,
                               commandstr.as_mut_ptr());
                        num_menuitems = num_menuitems.wrapping_add(1)
                    }
                }
            }
        }
        fclose(menufile);
    } else {
        // one menu item - xterm
        err(b"can\'t find ~/.windowlab/windowlab.menurc, %s or %s\n\x00" as
                *const u8 as *const libc::c_char, menurcpath.as_mut_ptr(),
            b"/usr/local/etc/X11/windowlab/windowlab.menurc\x00" as *const u8
                as *const libc::c_char);
        let ref mut fresh2 =
            (*menuitems.offset(0 as libc::c_int as isize)).command;
        *fresh2 =
            malloc(strlen(b"xterm\x00" as *const u8 as
                              *const libc::c_char).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong))
                as *mut libc::c_char;
        strcpy((*menuitems.offset(0 as libc::c_int as isize)).command,
               b"xterm\x00" as *const u8 as *const libc::c_char);
        let ref mut fresh3 =
            (*menuitems.offset(0 as libc::c_int as isize)).label;
        *fresh3 =
            malloc(strlen(b"xterm\x00" as *const u8 as
                              *const libc::c_char).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong))
                as *mut libc::c_char;
        strcpy((*menuitems.offset(0 as libc::c_int as isize)).label,
               b"xterm\x00" as *const u8 as *const libc::c_char);
        num_menuitems = 1 as libc::c_int as libc::c_uint
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_menuitems {
        (*menuitems.offset(i as isize)).x = button_startx as libc::c_int;
        (*menuitems.offset(i as isize)).width =
            XTextWidth(font, (*menuitems.offset(i as isize)).label,
                       strlen((*menuitems.offset(i as isize)).label) as
                           libc::c_int) + 3 as libc::c_int * 4 as libc::c_int;
        button_startx =
            button_startx.wrapping_add(((*menuitems.offset(i as isize)).width
                                            + 1 as libc::c_int) as
                                           libc::c_uint);
        i = i.wrapping_add(1)
    }
    // menu items have been built
    do_menuitems = 0 as libc::c_int;
}
unsafe extern "C" fn parseline(mut menustr: *mut libc::c_char,
                               mut labelstr: *mut libc::c_char,
                               mut commandstr: *mut libc::c_char)
 -> libc::c_int {
    let mut success: libc::c_int = 0 as libc::c_int;
    let mut menustrlen: libc::c_int = strlen(menustr) as libc::c_int;
    let mut ptemp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut menustrcpy: *mut libc::c_char =
        malloc((menustrlen + 1 as libc::c_int) as libc::c_ulong) as
            *mut libc::c_char;
    if menustrcpy.is_null() { return 0 as libc::c_int }
    strcpy(menustrcpy, menustr);
    ptemp = strtok(menustrcpy, b":\x00" as *const u8 as *const libc::c_char);
    if !ptemp.is_null() {
        strcpy(labelstr, ptemp);
        ptemp =
            strtok(0 as *mut libc::c_char,
                   b"\n\x00" as *const u8 as *const libc::c_char);
        if !ptemp.is_null() {
            // right of ':' is not empty
            while *ptemp as libc::c_int == ' ' as i32 ||
                      *ptemp as libc::c_int == '\t' as i32 {
                ptemp = ptemp.offset(1)
            }
            if *ptemp as libc::c_int != '\u{0}' as i32 &&
                   *ptemp as libc::c_int != '\r' as i32 &&
                   *ptemp as libc::c_int != '\n' as i32 {
                strcpy(commandstr, ptemp);
                success = 1 as libc::c_int
            }
        }
    }
    if !menustrcpy.is_null() { free(menustrcpy as *mut libc::c_void); }
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn free_menuitems() {
    let mut i: libc::c_uint = 0;
    if !menuitems.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < num_menuitems {
            if !(*menuitems.offset(i as isize)).label.is_null() {
                free((*menuitems.offset(i as isize)).label as
                         *mut libc::c_void);
                let ref mut fresh4 = (*menuitems.offset(i as isize)).label;
                *fresh4 = 0 as *mut libc::c_char
            }
            if !(*menuitems.offset(i as isize)).command.is_null() {
                free((*menuitems.offset(i as isize)).command as
                         *mut libc::c_void);
                let ref mut fresh5 = (*menuitems.offset(i as isize)).command;
                *fresh5 = 0 as *mut libc::c_char
            }
            i = i.wrapping_add(1)
        }
        free(menuitems as *mut libc::c_void);
        menuitems = 0 as *mut MenuItem
    };
}
