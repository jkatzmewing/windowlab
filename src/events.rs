use crate::state::TotalState;

pub fn do_event_loop(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
) {
    loop {
        if let Some(ev) = conn.wait_for_event() {
            /* TODO menu items stuff goes here */

            match ev.response_type() {
                xcb::KEY_PRESS => {
                    let ev: &xcb::KeyPressEvent = unsafe {
                        xcb::cast_event(&ev)
                    };
                    handle_key_press(&conn, root, state, ev);
                },

                xcb::BUTTON_PRESS => {
                    let ev: &xcb::ButtonPressEvent = unsafe {
                        xcb::cast_event(&ev)
                    };
                    handle_button_press(conn, root, state, ev);
                },
                
                _ => {}, // TODO other conditions
            }
        }
    }
}

fn handle_key_press(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::KeyPressEvent,
) {
    std::unimplemented!();
}

fn handle_button_press(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::ButtonPressEvent,
) {
    std::unimplemented!();
}
