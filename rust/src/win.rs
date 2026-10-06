//! Declaracoes minimas da API do Windows usadas pelo nucleo (sem dependencias externas).

#![allow(non_snake_case, clippy::upper_case_acronyms)]

use core::ffi::c_void;

pub type HWND = *mut c_void;
pub type HDC = *mut c_void;

#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}

pub const SRCCOPY: u32 = 0x00CC_0020;
pub const BLACKNESS: u32 = 0x0000_0042;
pub const DIB_RGB_COLORS: u32 = 0;
pub const COLORONCOLOR: i32 = 3;
pub const HALFTONE: i32 = 4;
pub const GWL_WNDPROC: i32 = -4;
pub const GWL_STYLE: i32 = -16;
pub const GWL_EXSTYLE: i32 = -20;
pub const WS_THICKFRAME: u32 = 0x0004_0000;
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
pub const SW_MAXIMIZE: i32 = 3;
pub const SWP_NOZORDER: u32 = 0x0004;
pub const SWP_NOACTIVATE: u32 = 0x0010;
pub const SWP_FRAMECHANGED: u32 = 0x0020;
pub const PAGE_READWRITE: u32 = 0x04;

#[link(name = "user32")]
extern "system" {
    pub fn GetDC(hwnd: HWND) -> HDC;
    pub fn ReleaseDC(hwnd: HWND, hdc: HDC) -> i32;
    pub fn GetClientRect(hwnd: HWND, r: *mut RECT) -> i32;
    pub fn GetSystemMetrics(i: i32) -> i32;
    pub fn SetWindowLongA(hwnd: HWND, i: i32, v: isize) -> isize;
    pub fn CallWindowProcA(anterior: isize, hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize;
    pub fn DefWindowProcA(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize;
    pub fn GetWindowLongA(hwnd: HWND, i: i32) -> isize;
    pub fn GetWindowRect(hwnd: HWND, r: *mut RECT) -> i32;
    pub fn AdjustWindowRectEx(r: *mut RECT, estilo: u32, menu: i32, estilo_ex: u32) -> i32;
    pub fn IsZoomed(hwnd: HWND) -> i32;
    pub fn ShowWindow(hwnd: HWND, modo: i32) -> i32;
    pub fn SetWindowPos(hwnd: HWND, depois: HWND, x: i32, y: i32, w: i32, h: i32, flags: u32) -> i32;
}

#[link(name = "gdi32")]
extern "system" {
    pub fn StretchDIBits(
        hdc: HDC, xd: i32, yd: i32, wd: i32, hd: i32, xs: i32, ys: i32, ws: i32, hs: i32,
        bits: *const c_void, info: *const BITMAPINFOHEADER, uso: u32, rop: u32,
    ) -> i32;
    pub fn SetStretchBltMode(hdc: HDC, modo: i32) -> i32;
    pub fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, anterior: *mut c_void) -> i32;
    pub fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: u32) -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleW(nome: *const u16) -> *mut c_void;
    pub fn VirtualProtect(end: *mut c_void, tam: usize, nova: u32, antiga: *mut u32) -> i32;
    pub fn GetEnvironmentVariableW(nome: *const u16, buf: *mut u16, tam: u32) -> u32;
}

/// Variavel de ambiente definida e nao vazia?
pub fn ambiente(nome: &str) -> bool {
    let n: Vec<u16> = nome.encode_utf16().chain([0]).collect();
    let mut buf = [0u16; 8];
    unsafe { GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) > 0 }
}
