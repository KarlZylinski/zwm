/*
This is a layer between the internal winapi ffi (internal.rs) and the rest of the application. It exists to:
- Separate winapi unsafeness from the rest of the rust code.
- Create some commonly used utility for this application.

This is not a rust-winapi wrapper, it contains loads of stuff specific to this application.
*/

extern crate libc;
use libc::{c_uint, c_int, c_void};
mod internal;
pub mod key;
use self::key::*;
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

unsafe extern "system" fn window_proc(window_handle: WindowHandle, msg: c_uint, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    /*return match msg {
        WM_KEYDOWN => {
            if msg == VirtualKey::Kkk as u32 {
                println!("LOL");
                return 0;
            }

            return DefWindowProcA(window_handle, msg, wparam, lparam);
        }
        _ => {
            return DefWindowProcA(window_handle, msg, wparam, lparam);
        }
    }*/
    return DefWindowProcA(window_handle, msg, wparam, lparam);
}


// Move these things into a Window struct?
static mut WIN_KEY_DOWN: bool = false;
static mut LOW_LEVEL_KEYBOARD_HOOK: usize = 0;
//static mut HOTKEY_CALLBACK: Option<Fn(VirtualKey)> = None;

unsafe extern "system" fn low_level_keyboard_proc(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let default_retval = || -> LRESULT {
        return CallNextHookEx(LOW_LEVEL_KEYBOARD_HOOK as HHOOK, code, wparam, lparam);
    };

    let is_win_key = |virtual_key: u32| -> bool {
        return virtual_key == VirtualKey::LeftWin as u32 || virtual_key == VirtualKey::RightWin as u32
    };

    if code < 0 || code != HC_ACTION {
        return default_retval();
    }

    let keyboard_info: &mut KBDLLHOOKSTRUCT = mem::transmute(lparam);

    return match wparam {
        WM_KEYDOWN => {
            let is_win_key = is_win_key(keyboard_info.vkCode);

            if !is_win_key && WIN_KEY_DOWN {
                /*match HOTKEY_CALLBACK {
                    Some(callback) => {
                        /*let callback: &mut Fn(VirtualKey) = mem::transmute(callback_u64);
                        let virtual_key: VirtualKey = mem::transmute(keyboard_info.vkCode as u8);
                        callback(virtual_key);
                        return 1;*/
                        let virtual_key: VirtualKey = mem::transmute(keyboard_info.vkCode as u8);
                        callback(virtual_key);
                        return 1;
                    },
                    _ => {
                        return 1;
                    }
                };*/
                println!("Pressed hotkey {:?}", keyboard_info.vkCode);
                return 1;
            }

            if is_win_key {
                WIN_KEY_DOWN = true;
                return 1;
            }

            default_retval()
        },
        WM_KEYUP => {
            if is_win_key(keyboard_info.vkCode) {
                WIN_KEY_DOWN = false;
                return 1;
            }

            default_retval()
        },
        _ => { default_retval() }
    }
}

pub fn setup_keyboard_hook<F>(callback: &F) 
    where F: Fn(VirtualKey) {
    unsafe {
        if (LOW_LEVEL_KEYBOARD_HOOK != 0) {
            panic!("setup_keyboard_hook has already been run.");
        }

        LOW_LEVEL_KEYBOARD_HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), GetModuleHandleA(NULLSTR), 0) as usize;

        if (LOW_LEVEL_KEYBOARD_HOOK == 0) {
            panic!("failed setting up low level keyboard hook.");
        }

        //HOTKEY_CALLBACK = Some(callback);
    }
}

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
            lpfnWndProc: Some(window_proc),
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

pub fn remove_window_border(handle: WindowHandle) {
    unsafe {
        let current_style = GetWindowLongPtrA(handle, GWL_STYLE) as u32;
        let new_style = current_style & !(WS_CAPTION | WS_THICKFRAME | WS_MINIMIZE | WS_MAXIMIZE | WS_SYSMENU);
        SetWindowLongPtrA(handle, GWL_STYLE, new_style as i32);
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

pub fn register_hot_key(handle: Option<WindowHandle>, id: c_int, modifiers: c_uint, key: VirtualKey) -> bool {
    unsafe {
        return RegisterHotKey(handle.unwrap_or(NULLPTR), id, modifiers, key as u32) == TRUE;
    }
}
