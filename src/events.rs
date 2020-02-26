use crate::state::TotalState;

pub fn do_event_loop(conn: &xcb::Connection, root: xcb::Window, state: &mut TotalState) {
    loop {
        if let Some(ev) = conn.wait_for_event() {
            /* TODO menu items stuff goes here */

            match ev.response_type() {
                xcb::KEY_PRESS => {
                    let ev: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&ev) };
                    handle_key_press(&conn, root, state, ev);
                }

                xcb::BUTTON_PRESS => {
                    let ev: &xcb::ButtonPressEvent = unsafe { xcb::cast_event(&ev) };
                    handle_button_press(conn, root, state, ev);
                }

                xcb::CONFIGURE_REQUEST => {
                    let ev: &xcb::ConfigureRequestEvent = unsafe { xcb::cast_event(&ev) };
                    handle_configure_request(conn, root, state, ev);
                }

                xcb::MAP_REQUEST => {
                    let ev: &xcb::MapRequestEvent = unsafe { xcb::cast_event(&ev) };
                    handle_map_request(conn, root, state, ev);
                }

                xcb::UNMAP_NOTIFY => {
                    let ev: &xcb::UnmapNotifyEvent = unsafe { xcb::cast_event(&ev) };
                    handle_unmap_event(conn, root, state, ev);
                }

                xcb::DESTROY_NOTIFY => {
                    let ev: &xcb::DestroyNotifyEvent = unsafe { xcb::cast_event(&ev) };
                    handle_destroy_event(conn, root, state, ev);
                }

                xcb::CLIENT_MESSAGE => {
                    let ev: &xcb::ClientMessageEvent = unsafe { xcb::cast_event(&ev) };
                    handle_client_message(conn, root, state, ev);
                }

                xcb::COLORMAP_NOTIFY => {
                    let ev: &xcb::ColormapNotifyEvent = unsafe { xcb::cast_event(&ev) };
                    handle_colormap_change(conn, root, state, ev);
                }

                _ => {} // TODO other conditions
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

fn handle_configure_request(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::ConfigureRequestEvent,
) {
    std::unimplemented!();
}

fn handle_map_request(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::MapRequestEvent,
) {
    std::unimplemented!();
}

fn handle_unmap_event(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::UnmapNotifyEvent,
) {
    std::unimplemented!();
}

fn handle_destroy_event(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::DestroyNotifyEvent,
) {
    std::unimplemented!();
}

fn handle_client_message(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::ClientMessageEvent,
) {
    std::unimplemented!();
}

fn handle_colormap_change(
    conn: &xcb::Connection,
    root: xcb::Window,
    state: &mut TotalState,
    ev: &xcb::ColormapNotifyEvent,
) {
    std::unimplemented!();
}
