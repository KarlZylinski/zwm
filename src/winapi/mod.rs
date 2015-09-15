extern crate libc;
mod internal;
use math::*;
use self::internal::*;
use std::mem;

macro_rules! def(
    ($t:ident) => {
        $t { ..Default::default() }
    };
);

pub fn winstr(str: &[u8]) -> Winstr {
    return str.as_ptr() as Winstr;
}

pub type Winstr = *const i8;
pub const NULLSTR: Winstr = 0 as Winstr;
pub type WindowHandle = HWND;

pub fn create_window(name: &[u8], size: &Vector2) -> WindowHandle {
    let winstr_name = winstr(name);
    const STYLE: libc::types::os::arch::extra::DWORD = WS_POPUP | WS_VISIBLE;

    unsafe {
        let instance = GetModuleHandleA(NULLSTR);

        if instance == NULLPTR {
            panic!("Failed to create register window class.");
        }

        let wc = WNDCLASSEXA {
            hInstance: instance,
            lpszClassName: winstr_name,
            style: CS_OWNDC,
            ..Default::default()
        };

        let class_atom = RegisterClassExA(&wc) as Winstr;

        if class_atom == 0 as *const i8 {
            panic!("Failed to create register window class.");
        }

        let hwnd = CreateWindowExA(WS_EX_TOOLWINDOW, class_atom, winstr_name, STYLE, 0, 0, size.x, size.y,
                                   NULLPTR, NULLPTR, instance, NULLPTR);

        if hwnd == NULLPTR {
            panic!("Failed to create window.");
        }

        return hwnd;
    }
}

pub fn is_window(handle: WindowHandle) -> bool {
    unsafe {
        return IsWindow(handle) != 0;
    }
}

pub enum WindowVisibility {
    Visible,
    Hidden
}

pub fn find_window(class_name: &str, window_name: Option<&str>) -> Option<WindowHandle> {
    unsafe {
        let hwnd = FindWindowA(
            winstr(class_name.as_bytes()),
            window_name.map_or(NULLSTR, |name| winstr(name.as_bytes()))
        );

        return if hwnd == NULLPTR {
            None
        } else {
            Some(hwnd)
        }
    }
}

pub fn set_window_visibility(handle: WindowHandle, visibility: WindowVisibility) {
    unsafe {
        ShowWindow(handle, match visibility {
            WindowVisibility::Visible => SW_SHOW,
            WindowVisibility::Hidden => SW_HIDE
        });
    }
}

pub fn get_desktop_window() -> WindowHandle {
    unsafe {
        return GetDesktopWindow();
    }
}

pub fn get_window_rect(handle: WindowHandle) -> Rect {
    unsafe {
        let mut desktop_rect = def!(RECT);
        GetWindowRect(handle, &mut desktop_rect);

        return Rect {
            left: desktop_rect.left,
            right: desktop_rect.right,
            top: desktop_rect.top,
            bottom: desktop_rect.bottom
        }
    }
}

pub fn process_events() {
    unsafe {
        let mut msg = MSG {
            ..Default::default()
        };

        while PeekMessageA(&mut msg, NULLPTR, 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

pub fn set_desktop_work_area(rect: &Rect) {
    unsafe {
        let mut win_rect = RECT {
            left: rect.left,
            right: rect.right,
            top: rect.top,
            bottom: rect.bottom
        };

        SystemParametersInfoA(SPI_SETWORKAREA, 0, &mut win_rect as *mut _ as PVOID, SPIF_SENDCHANGE);
    }
}

pub fn set_window_position_and_size(window_handle: WindowHandle, position: &Vector2, size: &Vector2) {
    unsafe {
        SetWindowPos(window_handle, NULLPTR, position.x, position.y, size.x, size.y, 0);
    }
}

pub fn get_all_current_windows(all_windows: &mut Vec<WindowHandle>)
{
    unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let windows: &mut Vec<WindowHandle> = mem::transmute(lparam);

        if IsWindowVisible(hwnd) == 0 {
            return TRUE;
        }

        if GetWindowLongA(hwnd, GWL_EXSTYLE) as u32 & WS_EX_TOOLWINDOW as u32 != 0 {
            return TRUE;
        }

        let mut hwnd_try = GetAncestor(hwnd, GA_ROOTOWNER);
        let mut hwnd_walk = NULLPTR;

        loop {
            if hwnd_walk == hwnd_try {
                break;
            }

            hwnd_walk = hwnd_try;
            hwnd_try = GetLastActivePopup(hwnd_walk);

            if IsWindowVisible(hwnd_try) == TRUE {
                break;
            }
        }

        if hwnd_walk != hwnd {
            return TRUE;
        }

        windows.push(hwnd);
        return TRUE;
    };

    unsafe {
        let lparam: LPARAM = mem::transmute(all_windows);
        EnumWindows(Some(callback), lparam);
    }
}
