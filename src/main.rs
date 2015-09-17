extern crate libc;
mod math;
use math::*;
mod winapi;
use winapi::*;
use winapi::key::*;

fn resize_windows(primary_window: Option<WindowHandle>, auxiliary_windows: &Vec<WindowHandle>, primary_window_width_ratio: f32, desktop_size: &Vector2) {
    let primary_window_size = Vector2::new((desktop_size.x as f32 * primary_window_width_ratio) as i32, desktop_size.y);

    match primary_window {
        Some(window) => {
            println!("{:?}", window);
            set_window_position_and_size(window, &Vector2::new(0, 0), &primary_window_size);
        }, _ => {}
    }
    
    let auxiliary_window_size = Vector2::new(desktop_size.x - primary_window_size.x, desktop_size.y / auxiliary_windows.len() as i32);

    for i in 0..auxiliary_windows.len() {
        set_window_position_and_size(auxiliary_windows[i], &Vector2::new(primary_window_size.x, auxiliary_window_size.y * i as i32), &auxiliary_window_size);
    }
}

fn main() {
    hide_explorer();
    
    match get_console_window() {
        Some(handle) => set_window_visibility(handle, WindowVisibility::Hidden),
        _ => {}
    };

    let desktop_rect = get_window_rect(get_desktop_window());
    set_desktop_work_area(&desktop_rect);
    let desktop_size = desktop_rect.size();
    let main_window_handle = create_window(b"ZWM", &desktop_size);
    let mut all_windows: Vec<WindowHandle> = Vec::new();
    get_all_current_windows(&mut all_windows);

    for i in 0..all_windows.len() {
        let window = all_windows[i];

        if window == main_window_handle {
            all_windows.remove(i);
            break;
        }
    }

    let mut auxiliary_windows = all_windows.to_vec();
    static mut primary_window_width_ratio: f32 = 0.6;

    let primary_window = if all_windows.is_empty() {
        None
    } else {
        Some(auxiliary_windows.remove(0))
    };

    match primary_window {
        Some(window) => {
            remove_window_border(window);
        }, _ => {}
    }
    
    for i in 0..auxiliary_windows.len() {
        let window_handle = auxiliary_windows[i];
        remove_window_border(auxiliary_windows[i]);
    }

    let refresh_window_sizes = move || {
        unsafe {
            resize_windows(primary_window, &auxiliary_windows, primary_window_width_ratio, &desktop_size);
        }
    };

    refresh_window_sizes();

    let mut hotkey_handler: Box<Fn(VirtualKey)> = Box::new(move |key: VirtualKey| {
        unsafe {
            match key {
                VirtualKey::H => {
                    primary_window_width_ratio -= 0.05;
                    refresh_window_sizes();
                }
                VirtualKey::K => {
                    primary_window_width_ratio += 0.05;
                    refresh_window_sizes();
                }
                _ => {
                }
            }
        }
    });

    setup_keyboard_hook(hotkey_handler);

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
        _ => {}
    };

    match find_window("Shell_TrayWnd", None) {
        Some(handle) => set_window_visibility(handle, WindowVisibility::Hidden),
        _ => {}
    };
}
