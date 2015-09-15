use libc::{c_int, c_uint, c_void, c_char, c_long, c_ushort, wchar_t};
use libc::types::os::arch::extra::{HANDLE, DWORD};
use std::mem;
pub type HWND = HANDLE;
pub type WORD = c_ushort;
pub type HMENU = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type LPVOID = *mut c_void;
pub type PVOID = *mut c_void;
pub type LPCSTR = *const c_char;
pub type WPARAM = c_uint;
pub type LPARAM = i64;
pub type LRESULT = c_long;
pub type LONG = c_long;
pub type BOOL = c_int;
pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_POPUP: DWORD = 0x80000000;
pub const CS_OWNDC: DWORD = 0x0020;
pub const PM_REMOVE: c_uint = 0x0001;
pub const SW_HIDE: c_int = 0x0000;
pub const SW_SHOW: c_int = 0x0005;
pub const SPI_SETWORKAREA: c_uint = 0x002F;
pub const SPIF_SENDCHANGE: c_uint = 2;
pub const GWL_EXSTYLE: c_int = -20;
pub const NULLPTR: *mut c_void = 0 as *mut c_void;
pub const GA_ROOTOWNER: c_uint = 0x0003;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;
pub type WNDPROC = Option<unsafe extern "system" fn(
    HWND, c_uint, WPARAM,LPARAM,
) -> LRESULT>;
pub type WNDENUMPROC = Option<unsafe extern "system" fn(HWND, LPARAM) -> BOOL>;

#[repr(C)]
pub struct HICON_ {
    pub i: c_int,
}
pub type HICON = *mut HICON_;

#[repr(C)]
pub struct HCURSOR_ {
    pub i: c_int,
}
pub type HCURSOR = *mut HCURSOR_;

#[repr(C)]
pub struct HBRUSH_ {
    pub i: c_int,
}
pub type HBRUSH = *mut HBRUSH_;

#[repr(C)] #[derive(Copy)] #[allow(non_snake_case)]
pub struct WNDCLASSEXA {
    pub cbSize: c_uint,
    pub style: c_uint,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
    pub hIconSm: HICON
}

impl Default for WNDCLASSEXA {
    fn default () -> WNDCLASSEXA {
        WNDCLASSEXA {
            cbSize: mem::size_of::<WNDCLASSEXA>() as u32,
            style: 0,
            lpfnWndProc: Some(DefWindowProcA),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: 0 as HBRUSH,
            lpszMenuName: 0 as LPCSTR,
            lpszClassName: 0 as LPCSTR,
            hIconSm: 0 as HICON
        }
    }
}

impl Clone for WNDCLASSEXA { fn clone(&self) -> WNDCLASSEXA { *self } }

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POINT {
    pub x: c_long,
    pub y: c_long
}

#[repr(C)] #[derive(Clone, Copy, Debug)] #[allow(non_snake_case)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: c_uint,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

pub type LPMSG = *mut MSG;

impl Default for MSG {
    fn default () -> MSG {
        MSG {
            hwnd: NULLPTR,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 }
        }
    }
}

#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct RECT {
    pub left: c_long,
    pub top: c_long,
    pub right: c_long,
    pub bottom: c_long
}

pub type LPRECT = *mut RECT;

#[link(name = "user32")] #[allow(dead_code)]
extern "system" {
    pub fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR, dwStyle: DWORD,
        x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU,
        hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    pub fn RegisterClassExA(lpWndClass: *const WNDCLASSEXA) -> WORD;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HINSTANCE;
    pub fn DefWindowProcA(hWnd: HWND, Msg: c_uint, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn GetLastError() -> DWORD;
    pub fn PeekMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: c_uint, wMsgFilterMax: c_uint, wRemoveMsg: c_uint) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn DispatchMessageA(lpmsg: *const MSG) -> LRESULT;
    pub fn FindWindowA(lpClassName: LPCSTR, lpWindowName: LPCSTR) -> HWND;
    pub fn IsWindow(hWnd: HWND) -> BOOL;
    pub fn IsWindowVisible(hWnd: HWND) -> BOOL;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn GetDesktopWindow() -> HWND;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn SystemParametersInfoA(uiAction: c_uint, uiParam: c_uint, pvParam: PVOID, fWinIni: c_uint) -> BOOL;
    pub fn EnumWindows(lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn GetWindowLongA(hWnd: HWND, nIndex: c_int) -> LONG;
    pub fn SetWindowPos(hWnd: HWND, hWndInsertAfter: HWND, X: c_int, Y: c_int, cx: c_int, cy: c_int, uFlags: c_uint) -> BOOL;
    pub fn GetAncestor(hWnd: HWND, gaFlags: c_uint) -> HWND;
    pub fn GetLastActivePopup(hWnd: HWND) -> HWND;
}
