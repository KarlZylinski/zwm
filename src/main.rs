extern crate libc;
mod math;
use math::*;
mod winapi;
use winapi::*;

fn main() {
    hide_explorer();
    let desktop_rect = get_window_rect(get_desktop_window());
    set_desktop_work_area(&desktop_rect);
    let desktop_size = desktop_rect.size();
    let main_window_handle = create_window(b"ZWM", &desktop_size);
    let mut all_windows: Vec<WindowHandle> = Vec::new();
    get_all_current_windows(&mut all_windows);
    let primary_window_size = Vector2::new(desktop_size.x / 2, desktop_size.y);
    let mut auxiliary_windows = all_windows.to_vec();

    let primary_window = if all_windows.is_empty() {
        None
    } else {
        Some(auxiliary_windows.remove(0))
    };

    match primary_window {
        Some(window) => {
            remove_window_border(window);
            set_window_position_and_size(window, &Vector2::new(0, 0), &primary_window_size);
        }, _ => {}
    }
    
    let auxiliary_window_size = Vector2::new(desktop_size.x - primary_window_size.x, desktop_size.y / auxiliary_windows.len() as i32);

    for i in 0..auxiliary_windows.len()  {
        let window_handle = auxiliary_windows[i];
        set_window_position_and_size(window_handle, &Vector2::new(primary_window_size.x, auxiliary_window_size.y * i as i32), &auxiliary_window_size);
        remove_window_border(window_handle);
    }

    loop {
        std::thread::sleep_ms(10);

        if !is_window(main_window_handle) {
            break;
        }

        process_events();
    }
}

fn hide_explorer() {
    match find_window("Progman", Some("Program Manager")) {
        Some(handle) => set_window_visibility(handle, WindowVisibility::Hidden),
        _ => {}mod
    };

    match find_window("Shell_TrayWnd", None) {
        Some(handle) => set_window_visibility(handle, WindowVisibility::Hidden),
        _ => {}
    };
}
