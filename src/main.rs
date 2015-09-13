extern crate libc;
mod winapi;
use winapi::*;

fn create_window() -> HWND {
    let name = winstr(b"ZWM");
    const STYLE: libc::types::os::arch::extra::DWORD = WS_POPUP | WS_VISIBLE;

    unsafe {
        let instance = GetModuleHandleA(NULLSTR);

        if instance == NULLPTR {
            panic!("Failed to create register window class.");
        }

        let wc = WNDCLASSEXA {
            hInstance: instance,
            lpszClassName: name,
            style: CS_OWNDC,
            ..Default::default()
        };

        let class_atom = RegisterClassExA(&wc) as Winstr;

        if class_atom == 0 as *const i8 {
            panic!("Failed to create register window class.");
        }

        let hwnd = CreateWindowExA(WS_EX_TOOLWINDOW, class_atom, name, STYLE, 0, 0, 800, 600,
                                   NULLPTR, NULLPTR, instance, NULLPTR);

        if hwnd == NULLPTR {
            panic!("Failed to create window.");
        }

        return hwnd;
    }
}

fn process_events() {
    unsafe {
        print!("");
        let mut msg = MSG {
            ..Default::default()
        };

        while PeekMessageA(&mut msg, NULLPTR, 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

fn is_window_open(hwnd: HWND) -> bool {
    unsafe {
        return IsWindow(hwnd) != 0;
    }
}

fn hide_window(class_name: &str, window_name: Option<&str>) {
    unsafe {
        let handle = FindWindowA(winstr(class_name.as_bytes()), winstr(window_name.unwrap_or("").as_bytes()));
        ShowWindow(handle, SW_HIDE);
    }
}

fn main() {
    hide_window("Progman", Some("Program Manager"));
    hide_window("Shell_TrayWnd", None);

    let window_handle = create_window();

    loop {
        std::thread::sleep_ms(10);

        if !is_window_open(window_handle) {
            break;
        }

        process_events();
    }
}
