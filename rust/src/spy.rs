//! COM wrappers that forward each DirectDraw method to the real object and log the call.
//!
//! Each object handed to the game (IDirectDraw, IDirectDrawSurface, IDirectDrawClipper) is an
//! `Embrulho` with our vtable. Arguments that are wrappers are swapped for the real object
//! before forwarding; outputs that create objects are wrapped.
//! The first calls of each method are logged with arguments; after that, only the count.
//!
//! [pt] Embrulhos COM que repassam cada metodo do DirectDraw ao objeto verdadeiro e registram a chamada.
//!
//! Cada objeto entregue ao jogo (IDirectDraw, IDirectDrawSurface, IDirectDrawClipper) e um
//! `Embrulho` com a nossa vtable. Argumentos que sejam embrulhos sao trocados pelo objeto
//! verdadeiro antes de repassar; saidas que criam objetos sao embrulhadas.
//! As primeiras chamadas de cada metodo saem com argumentos; depois, so a contagem.

use core::ffi::c_void;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::registro::linha;

/// How many calls of each method are logged in detail.
///
/// [pt] Quantas chamadas de cada metodo saem detalhadas no registro.
const DETAILED: u64 = 25;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    DirectDraw,
    Surface,
    Clipper,
}

#[repr(C)]
pub struct Wrapper {
    vtbl: *const usize,
    real: *mut c_void,
    kind: Kind,
    id: u32,
}

#[derive(Default)]
struct State {
    /// real object -> wrapper
    ///
    /// [pt] objeto verdadeiro -> embrulho
    by_real: HashMap<usize, usize>,
    /// wrapper -> real object
    ///
    /// [pt] embrulho -> objeto verdadeiro
    by_wrapper: HashMap<usize, usize>,
    counts: HashMap<&'static str, u64>,
    next_id: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut g = STATE.lock().unwrap_or_else(|e| e.into_inner());
    f(g.get_or_insert_with(State::default))
}

/// Counts the call; returns true if it should still be logged in detail. Marks 100, 1000, 10000... in the log.
///
/// [pt] Conta a chamada; devolve true se ainda deve sair detalhada. Marca 100, 1000, 10000... no registro.
fn count_call(name: &'static str) -> bool {
    let n = with_state(|e| {
        let c = e.counts.entry(name).or_insert(0);
        *c += 1;
        *c
    });
    if n >= 100 && 10u64.pow(n.ilog10()) == n {
        linha(&format!("  ({} chamadas de {})", n, name));
    }
    n <= DETAILED
}

pub unsafe fn wrap(real: *mut c_void, kind: Kind) -> *mut c_void {
    if real.is_null() {
        return real;
    }
    let new_entry = with_state(|e| {
        if let Some(&emb) = e.by_real.get(&(real as usize)) {
            return Err(emb);
        }
        e.next_id += 1;
        let emb = Box::into_raw(Box::new(Wrapper { vtbl: vtable(kind), real, kind, id: e.next_id })) as usize;
        e.by_real.insert(real as usize, emb);
        e.by_wrapper.insert(emb, real as usize);
        Ok((emb, e.next_id))
    });
    match new_entry {
        Err(emb) => emb as *mut c_void,
        Ok((emb, id)) => {
            linha(&format!("  novo {:?} #{} (real {:p})", kind, id, real));
            emb as *mut c_void
        }
    }
}

/// If `v` is one of our wrappers, returns the real object; otherwise, returns `v`.
///
/// [pt] Se `v` for um embrulho nosso, devolve o objeto verdadeiro; senao, devolve `v`.
fn unwrap_arg(v: usize) -> usize {
    if v == 0 {
        return 0;
    }
    with_state(|e| e.by_wrapper.get(&v).copied().unwrap_or(v))
}

unsafe fn method(real: *mut c_void, i: usize) -> usize {
    *(*(real as *const *const usize)).add(i)
}

macro_rules! usize_of {
    ($a:ident) => {
        usize
    };
}

/// Generic forwarding: unwraps all the arguments, calls method `$i` of the real object.
///
/// [pt] Repasse generico: desembrulha todos os argumentos, chama o metodo `$i` do objeto verdadeiro.
macro_rules! forward {
    ($f:ident, $i:expr, $name:literal $(, $a:ident)*) => {
        unsafe extern "system" fn $f(this: *mut Wrapper $(, $a: usize)*) -> i32 {
            let real = (*this).real;
            let g: unsafe extern "system" fn(*mut c_void $(, usize_of!($a))*) -> i32 =
                core::mem::transmute(method(real, $i));
            let r = g(real $(, unwrap_arg($a))*);
            if count_call($name) {
                let mut s = format!("#{} {}(", (*this).id, $name);
                $( s += &format!("{}={:#x} ", stringify!($a), $a); )*
                s += &format!(") -> {:#x}", r);
                linha(&s);
            }
            r
        }
    };
}

/// Forwarding with a `**` output that creates an object (`$out` is the name of the pointer-to-pointer argument).
///
/// [pt] Repasse com uma saida `**` que cria objeto (`$saida` e o nome do argumento ponteiro-para-ponteiro).
macro_rules! forward_create {
    ($f:ident, $i:expr, $name:literal, $kind:expr, $out:ident $(, $a:ident)*) => {
        unsafe extern "system" fn $f(this: *mut Wrapper $(, $a: usize)*) -> i32 {
            let real = (*this).real;
            let g: unsafe extern "system" fn(*mut c_void $(, usize_of!($a))*) -> i32 =
                core::mem::transmute(method(real, $i));
            let r = g(real $(, unwrap_arg($a))*);
            let detailed = count_call($name);
            if detailed {
                let mut s = format!("#{} {}(", (*this).id, $name);
                $( s += &format!("{}={:#x} ", stringify!($a), $a); )*
                s += &format!(") -> {:#x}", r);
                linha(&s);
            }
            if r == 0 && $out != 0 {
                let p = $out as *mut *mut c_void;
                *p = wrap(*p, $kind);
            }
            r
        }
    };
}

// ---------- IUnknown (common to all three) ----------

unsafe extern "system" fn query_interface(this: *mut Wrapper, iid: *const [u8; 16], out: *mut *mut c_void) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, *const [u8; 16], *mut *mut c_void) -> i32 = core::mem::transmute(method(real, 0));
    let r = g(real, iid, out);
    let guid = if iid.is_null() { String::from("?") } else { (*iid).iter().map(|b| format!("{:02x}", b)).collect() };
    // For now the returned object is NOT wrapped: if the game uses it, the log shows it and we handle it.
    linha(&format!("#{} QueryInterface({}) -> {:#x}  [nao embrulhado]", (*this).id, guid, r));
    r
}

unsafe extern "system" fn add_ref(this: *mut Wrapper) -> u32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void) -> u32 = core::mem::transmute(method(real, 1));
    g(real)
}

unsafe extern "system" fn release(this: *mut Wrapper) -> u32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void) -> u32 = core::mem::transmute(method(real, 2));
    let r = g(real);
    if r == 0 {
        let (id, kind) = ((*this).id, (*this).kind);
        with_state(|e| {
            e.by_real.remove(&(real as usize));
            e.by_wrapper.remove(&(this as usize));
        });
        drop(Box::from_raw(this));
        linha(&format!("  liberado {:?} #{}", kind, id));
        if kind == Kind::DirectDraw {
            summary();
        }
    }
    r
}

fn summary() {
    let mut v: Vec<(&'static str, u64)> = with_state(|e| e.counts.iter().map(|(k, v)| (*k, *v)).collect());
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    linha("== resumo de chamadas ==");
    for (name, n) in v {
        linha(&format!("  {:>10}  {}", n, name));
    }
}

// ---------- structure details ----------

unsafe fn dword(p: usize, off: usize) -> u32 {
    *((p + off) as *const u32)
}

/// DDSURFACEDESC (0x6c bytes): the fields that matter for the video.
///
/// [pt] DDSURFACEDESC (0x6c bytes): campos que importam para o video.
unsafe fn surface_desc(p: usize) -> String {
    if p == 0 {
        return "null".into();
    }
    format!(
        "flags={:#x} {}x{} pitch={} caps={:#x} fmt[flags={:#x} bpp={} r={:#x} g={:#x} b={:#x}] mem={:#x}",
        dword(p, 4), dword(p, 0xc), dword(p, 8), dword(p, 0x10) as i32, dword(p, 0x68),
        dword(p, 0x4c), dword(p, 0x54), dword(p, 0x58), dword(p, 0x5c), dword(p, 0x60), dword(p, 0x24)
    )
}

unsafe fn rect(p: usize) -> String {
    if p == 0 {
        return "inteira".into();
    }
    let r = p as *const i32;
    format!("({},{})-({},{})", *r, *r.add(1), *r.add(2), *r.add(3))
}

// ---------- IDirectDraw ----------

forward!(dd_compact, 3, "DD.Compact");
forward_create!(dd_create_clipper, 4, "DD.CreateClipper", Kind::Clipper, saida, flags, saida, externo);
forward!(dd_create_palette, 5, "DD.CreatePalette", flags, cores, saida, externo);

unsafe extern "system" fn dd_create_surface(this: *mut Wrapper, desc: usize, out: usize, outer: usize) -> i32 {
    let real = (*this).real;
    let requested = surface_desc(desc);
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize) -> i32 = core::mem::transmute(method(real, 6));
    let r = g(real, desc, out, outer);
    if count_call("DD.CreateSurface") {
        linha(&format!("#{} DD.CreateSurface({}) -> {:#x}", (*this).id, requested, r));
    }
    if r == 0 && out != 0 {
        let p = out as *mut *mut c_void;
        *p = wrap(*p, Kind::Surface);
    }
    r
}

forward_create!(dd_duplicate_surface, 7, "DD.DuplicateSurface", Kind::Surface, saida, origem, saida);
forward!(dd_enum_display_modes, 8, "DD.EnumDisplayModes", flags, desc, ctx, cb);
forward!(dd_enum_surfaces, 9, "DD.EnumSurfaces", flags, desc, ctx, cb);
forward!(dd_flip_to_gdi_surface, 10, "DD.FlipToGDISurface");
forward!(dd_get_caps, 11, "DD.GetCaps", hw, hel);
forward!(dd_get_display_mode, 12, "DD.GetDisplayMode", desc);
forward!(dd_get_fourcc_codes, 13, "DD.GetFourCCCodes", n, codigos);
forward_create!(dd_get_gdi_surface, 14, "DD.GetGDISurface", Kind::Surface, saida, saida);
forward!(dd_get_monitor_frequency, 15, "DD.GetMonitorFrequency", freq);
forward!(dd_get_scan_line, 16, "DD.GetScanLine", linha_);
forward!(dd_get_vertical_blank_status, 17, "DD.GetVerticalBlankStatus", status);
forward!(dd_initialize, 18, "DD.Initialize", guid);
forward!(dd_restore_display_mode, 19, "DD.RestoreDisplayMode");
forward!(dd_set_cooperative_level, 20, "DD.SetCooperativeLevel", hwnd, flags);
forward!(dd_set_display_mode, 21, "DD.SetDisplayMode", largura, altura, bpp);
forward!(dd_wait_for_vertical_blank, 22, "DD.WaitForVerticalBlank", flags, evento);

// ---------- IDirectDrawSurface ----------

forward!(s_add_attached_surface, 3, "S.AddAttachedSurface", sup);
forward!(s_add_overlay_dirty_rect, 4, "S.AddOverlayDirtyRect", ret);

unsafe extern "system" fn s_blt(this: *mut Wrapper, dest: usize, src: usize, src_rect: usize, flags: usize, fx: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize, usize, usize) -> i32 = core::mem::transmute(method(real, 5));
    let r = g(real, dest, unwrap_arg(src), src_rect, flags, fx);
    if count_call("S.Blt") {
        let src_id = if src == 0 { 0 } else { with_state(|e| e.by_wrapper.contains_key(&src)).then(|| (*(src as *const Wrapper)).id).unwrap_or(0) };
        linha(&format!(
            "#{} S.Blt(dest={} origem=#{} {} flags={:#x} fx={:#x}) -> {:#x}",
            (*this).id, rect(dest), src_id, rect(src_rect), flags, fx, r
        ));
    }
    r
}

forward!(s_blt_batch, 6, "S.BltBatch", lote, n, flags);

unsafe extern "system" fn s_blt_fast(this: *mut Wrapper, x: usize, y: usize, src: usize, src_rect: usize, trans: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize, usize, usize) -> i32 = core::mem::transmute(method(real, 7));
    let r = g(real, x, y, unwrap_arg(src), src_rect, trans);
    if count_call("S.BltFast") {
        let src_id = if src == 0 { 0 } else { with_state(|e| e.by_wrapper.contains_key(&src)).then(|| (*(src as *const Wrapper)).id).unwrap_or(0) };
        linha(&format!(
            "#{} S.BltFast(x={} y={} origem=#{} {} trans={:#x}) -> {:#x}",
            (*this).id, x as i32, y as i32, src_id, rect(src_rect), trans, r
        ));
    }
    r
}

forward!(s_delete_attached_surface, 8, "S.DeleteAttachedSurface", flags, sup);
forward!(s_enum_attached_surfaces, 9, "S.EnumAttachedSurfaces", ctx, cb);
forward!(s_enum_overlay_z_orders, 10, "S.EnumOverlayZOrders", flags, ctx, cb);
forward!(s_flip, 11, "S.Flip", alvo, flags);
forward_create!(s_get_attached_surface, 12, "S.GetAttachedSurface", Kind::Surface, saida, caps, saida);
forward!(s_get_blt_status, 13, "S.GetBltStatus", flags);
forward!(s_get_caps, 14, "S.GetCaps", caps);
forward_create!(s_get_clipper, 15, "S.GetClipper", Kind::Clipper, saida, saida);
forward!(s_get_color_key, 16, "S.GetColorKey", flags, chave);
forward!(s_get_dc, 17, "S.GetDC", hdc);
forward!(s_get_flip_status, 18, "S.GetFlipStatus", flags);
forward!(s_get_overlay_position, 19, "S.GetOverlayPosition", x, y);
forward!(s_get_palette, 20, "S.GetPalette", paleta);
forward!(s_get_pixel_format, 21, "S.GetPixelFormat", fmt);

unsafe extern "system" fn s_get_surface_desc(this: *mut Wrapper, desc: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize) -> i32 = core::mem::transmute(method(real, 22));
    let r = g(real, desc);
    if count_call("S.GetSurfaceDesc") {
        linha(&format!("#{} S.GetSurfaceDesc -> {:#x} {}", (*this).id, r, surface_desc(desc)));
    }
    r
}

forward!(s_initialize, 23, "S.Initialize", dd, desc);
forward!(s_is_lost, 24, "S.IsLost");

unsafe extern "system" fn s_lock(this: *mut Wrapper, ret: usize, desc: usize, flags: usize, evento: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize, usize) -> i32 = core::mem::transmute(method(real, 25));
    let r = g(real, ret, desc, flags, evento);
    if count_call("S.Lock") {
        linha(&format!("#{} S.Lock({} flags={:#x}) -> {:#x} {}", (*this).id, rect(ret), flags, r, surface_desc(desc)));
    }
    r
}

forward!(s_release_dc, 26, "S.ReleaseDC", hdc);
forward!(s_restore, 27, "S.Restore");
forward!(s_set_clipper, 28, "S.SetClipper", clipper);
forward!(s_set_color_key, 29, "S.SetColorKey", flags, chave);
forward!(s_set_overlay_position, 30, "S.SetOverlayPosition", x, y);
forward!(s_set_palette, 31, "S.SetPalette", paleta);
forward!(s_unlock, 32, "S.Unlock", ptr);
forward!(s_update_overlay, 33, "S.UpdateOverlay", ret_origem, dest, ret_dest, flags, fx);
forward!(s_update_overlay_display, 34, "S.UpdateOverlayDisplay", flags);
forward!(s_update_overlay_z_order, 35, "S.UpdateOverlayZOrder", flags, refer);

// ---------- IDirectDrawClipper ----------

forward!(c_get_clip_list, 3, "C.GetClipList", ret, lista, tam);
forward!(c_get_hwnd, 4, "C.GetHWnd", hwnd);
forward!(c_initialize, 5, "C.Initialize", dd, flags);
forward!(c_is_clip_list_changed, 6, "C.IsClipListChanged", mudou);
forward!(c_set_clip_list, 7, "C.SetClipList", lista, flags);
forward!(c_set_hwnd, 8, "C.SetHWnd", flags, hwnd);

fn vtable(kind: Kind) -> *const usize {
    static TABLES: OnceLock<[Vec<usize>; 3]> = OnceLock::new();
    let t = TABLES.get_or_init(|| {
        let iu = [query_interface as *const () as usize, add_ref as *const () as usize, release as *const () as usize];
        let mut dd = iu.to_vec();
        dd.extend([
            dd_compact as *const () as usize, dd_create_clipper as *const () as usize, dd_create_palette as *const () as usize, dd_create_surface as *const () as usize,
            dd_duplicate_surface as *const () as usize, dd_enum_display_modes as *const () as usize, dd_enum_surfaces as *const () as usize,
            dd_flip_to_gdi_surface as *const () as usize, dd_get_caps as *const () as usize, dd_get_display_mode as *const () as usize,
            dd_get_fourcc_codes as *const () as usize, dd_get_gdi_surface as *const () as usize, dd_get_monitor_frequency as *const () as usize,
            dd_get_scan_line as *const () as usize, dd_get_vertical_blank_status as *const () as usize, dd_initialize as *const () as usize,
            dd_restore_display_mode as *const () as usize, dd_set_cooperative_level as *const () as usize, dd_set_display_mode as *const () as usize,
            dd_wait_for_vertical_blank as *const () as usize,
        ]);
        let mut s = iu.to_vec();
        s.extend([
            s_add_attached_surface as *const () as usize, s_add_overlay_dirty_rect as *const () as usize, s_blt as *const () as usize, s_blt_batch as *const () as usize,
            s_blt_fast as *const () as usize, s_delete_attached_surface as *const () as usize, s_enum_attached_surfaces as *const () as usize,
            s_enum_overlay_z_orders as *const () as usize, s_flip as *const () as usize, s_get_attached_surface as *const () as usize, s_get_blt_status as *const () as usize,
            s_get_caps as *const () as usize, s_get_clipper as *const () as usize, s_get_color_key as *const () as usize, s_get_dc as *const () as usize,
            s_get_flip_status as *const () as usize, s_get_overlay_position as *const () as usize, s_get_palette as *const () as usize,
            s_get_pixel_format as *const () as usize, s_get_surface_desc as *const () as usize, s_initialize as *const () as usize, s_is_lost as *const () as usize,
            s_lock as *const () as usize, s_release_dc as *const () as usize, s_restore as *const () as usize, s_set_clipper as *const () as usize, s_set_color_key as *const () as usize,
            s_set_overlay_position as *const () as usize, s_set_palette as *const () as usize, s_unlock as *const () as usize, s_update_overlay as *const () as usize,
            s_update_overlay_display as *const () as usize, s_update_overlay_z_order as *const () as usize,
        ]);
        let mut c = iu.to_vec();
        c.extend([
            c_get_clip_list as *const () as usize, c_get_hwnd as *const () as usize, c_initialize as *const () as usize, c_is_clip_list_changed as *const () as usize,
            c_set_clip_list as *const () as usize, c_set_hwnd as *const () as usize,
        ]);
        assert_eq!((dd.len(), s.len(), c.len()), (23, 36, 9));
        [dd, s, c]
    });
    t[kind as usize].as_ptr()
}
