use super::*;

// Unaligned memory access for the game's records, one function per type (stage 4 of the reorganisation:
// they replace the copies each batch item used to carry).
// [pt] Acesso à memória dos registros do jogo, sem alinhamento, uma função por tipo (etapa 4 da reorganização:
// replace the copies each batch item carried).

pub unsafe fn read_f64(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

pub unsafe fn read_i16(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

pub unsafe fn read_i16_as_i32(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

pub unsafe fn read_i32(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

pub unsafe fn read_i8(a: usize) -> i8 {
    core::ptr::read_unaligned(a as *const i8)
}

pub unsafe fn read_i8_as_i32(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i8) as i32
}

pub unsafe fn read_u16(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

pub unsafe fn read_u32(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

pub unsafe fn read_u32_as_usize(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

pub unsafe fn read_u8(a: usize) -> u8 {
    core::ptr::read_unaligned(a as *const u8)
}

pub unsafe fn read_usize(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

pub unsafe fn write_f64(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v);
}

pub unsafe fn write_i16(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

pub unsafe fn write_i8(a: usize, v: i8) {
    core::ptr::write_unaligned(a as *mut i8, v);
}

pub unsafe fn write_u16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

pub unsafe fn write_u16_at(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

pub unsafe fn write_u32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

pub unsafe fn write_u32_at(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

pub unsafe fn write_u32_from_usize(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32);
}

pub unsafe fn write_u8(a: usize, v: u8) {
    core::ptr::write_unaligned(a as *mut u8, v);
}

pub unsafe fn write_u8_at(o: usize, d: usize, v: u8) {
    core::ptr::write_unaligned((o + d) as *mut u8, v);
}

pub unsafe fn write_usize(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

pub unsafe fn read_f32(a: usize) -> f32 {
    core::ptr::read_unaligned(a as *const f32)
}

pub unsafe fn write_i32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}
