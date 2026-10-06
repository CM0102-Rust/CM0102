//! Embrulhos COM que repassam cada metodo do DirectDraw ao objeto verdadeiro e registram a chamada.
//!
//! Cada objeto entregue ao jogo (IDirectDraw, IDirectDrawSurface, IDirectDrawClipper) e um
//! `Embrulho` com a nossa vtable. Argumentos que sejam embrulhos sao trocados pelo objeto
//! verdadeiro antes de repassar; saidas que criam objetos sao embrulhadas.
//! As primeiras chamadas de cada metodo saem com argumentos; depois, so a contagem.

use core::ffi::c_void;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::registro::linha;

/// Quantas chamadas de cada metodo saem detalhadas no registro.
const DETALHADAS: u64 = 25;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tipo {
    DirectDraw,
    Superficie,
    Clipper,
}

#[repr(C)]
pub struct Embrulho {
    vtbl: *const usize,
    real: *mut c_void,
    tipo: Tipo,
    id: u32,
}

#[derive(Default)]
struct Estado {
    /// objeto verdadeiro -> embrulho
    por_real: HashMap<usize, usize>,
    /// embrulho -> objeto verdadeiro
    por_embrulho: HashMap<usize, usize>,
    contagem: HashMap<&'static str, u64>,
    proximo_id: u32,
}

static ESTADO: Mutex<Option<Estado>> = Mutex::new(None);

fn com_estado<R>(f: impl FnOnce(&mut Estado) -> R) -> R {
    let mut g = ESTADO.lock().unwrap_or_else(|e| e.into_inner());
    f(g.get_or_insert_with(Estado::default))
}

/// Conta a chamada; devolve true se ainda deve sair detalhada. Marca 100, 1000, 10000... no registro.
fn chamada(nome: &'static str) -> bool {
    let n = com_estado(|e| {
        let c = e.contagem.entry(nome).or_insert(0);
        *c += 1;
        *c
    });
    if n >= 100 && 10u64.pow(n.ilog10()) == n {
        linha(&format!("  ({} chamadas de {})", n, nome));
    }
    n <= DETALHADAS
}

pub unsafe fn embrulhar(real: *mut c_void, tipo: Tipo) -> *mut c_void {
    if real.is_null() {
        return real;
    }
    let novo = com_estado(|e| {
        if let Some(&emb) = e.por_real.get(&(real as usize)) {
            return Err(emb);
        }
        e.proximo_id += 1;
        let emb = Box::into_raw(Box::new(Embrulho { vtbl: vtable(tipo), real, tipo, id: e.proximo_id })) as usize;
        e.por_real.insert(real as usize, emb);
        e.por_embrulho.insert(emb, real as usize);
        Ok((emb, e.proximo_id))
    });
    match novo {
        Err(emb) => emb as *mut c_void,
        Ok((emb, id)) => {
            linha(&format!("  novo {:?} #{} (real {:p})", tipo, id, real));
            emb as *mut c_void
        }
    }
}

/// Se `v` for um embrulho nosso, devolve o objeto verdadeiro; senao, devolve `v`.
fn desembrulhar(v: usize) -> usize {
    if v == 0 {
        return 0;
    }
    com_estado(|e| e.por_embrulho.get(&v).copied().unwrap_or(v))
}

unsafe fn metodo(real: *mut c_void, i: usize) -> usize {
    *(*(real as *const *const usize)).add(i)
}

macro_rules! usize_de {
    ($a:ident) => {
        usize
    };
}

/// Repasse generico: desembrulha todos os argumentos, chama o metodo `$i` do objeto verdadeiro.
macro_rules! repassa {
    ($f:ident, $i:expr, $nome:literal $(, $a:ident)*) => {
        unsafe extern "system" fn $f(this: *mut Embrulho $(, $a: usize)*) -> i32 {
            let real = (*this).real;
            let g: unsafe extern "system" fn(*mut c_void $(, usize_de!($a))*) -> i32 =
                core::mem::transmute(metodo(real, $i));
            let r = g(real $(, desembrulhar($a))*);
            if chamada($nome) {
                let mut s = format!("#{} {}(", (*this).id, $nome);
                $( s += &format!("{}={:#x} ", stringify!($a), $a); )*
                s += &format!(") -> {:#x}", r);
                linha(&s);
            }
            r
        }
    };
}

/// Repasse com uma saida `**` que cria objeto (`$saida` e o nome do argumento ponteiro-para-ponteiro).
macro_rules! repassa_cria {
    ($f:ident, $i:expr, $nome:literal, $tipo:expr, $saida:ident $(, $a:ident)*) => {
        unsafe extern "system" fn $f(this: *mut Embrulho $(, $a: usize)*) -> i32 {
            let real = (*this).real;
            let g: unsafe extern "system" fn(*mut c_void $(, usize_de!($a))*) -> i32 =
                core::mem::transmute(metodo(real, $i));
            let r = g(real $(, desembrulhar($a))*);
            let detalhe = chamada($nome);
            if detalhe {
                let mut s = format!("#{} {}(", (*this).id, $nome);
                $( s += &format!("{}={:#x} ", stringify!($a), $a); )*
                s += &format!(") -> {:#x}", r);
                linha(&s);
            }
            if r == 0 && $saida != 0 {
                let p = $saida as *mut *mut c_void;
                *p = embrulhar(*p, $tipo);
            }
            r
        }
    };
}

// ---------- IUnknown (comum aos tres) ----------

unsafe extern "system" fn query_interface(this: *mut Embrulho, iid: *const [u8; 16], saida: *mut *mut c_void) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, *const [u8; 16], *mut *mut c_void) -> i32 = core::mem::transmute(metodo(real, 0));
    let r = g(real, iid, saida);
    let guid = if iid.is_null() { String::from("?") } else { (*iid).iter().map(|b| format!("{:02x}", b)).collect() };
    // Por ora o objeto devolvido NAO e embrulhado: se o jogo usar, o registro mostra e tratamos.
    linha(&format!("#{} QueryInterface({}) -> {:#x}  [nao embrulhado]", (*this).id, guid, r));
    r
}

unsafe extern "system" fn add_ref(this: *mut Embrulho) -> u32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void) -> u32 = core::mem::transmute(metodo(real, 1));
    g(real)
}

unsafe extern "system" fn release(this: *mut Embrulho) -> u32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void) -> u32 = core::mem::transmute(metodo(real, 2));
    let r = g(real);
    if r == 0 {
        let (id, tipo) = ((*this).id, (*this).tipo);
        com_estado(|e| {
            e.por_real.remove(&(real as usize));
            e.por_embrulho.remove(&(this as usize));
        });
        drop(Box::from_raw(this));
        linha(&format!("  liberado {:?} #{}", tipo, id));
        if tipo == Tipo::DirectDraw {
            resumo();
        }
    }
    r
}

fn resumo() {
    let mut v: Vec<(&'static str, u64)> = com_estado(|e| e.contagem.iter().map(|(k, v)| (*k, *v)).collect());
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    linha("== resumo de chamadas ==");
    for (nome, n) in v {
        linha(&format!("  {:>10}  {}", n, nome));
    }
}

// ---------- detalhes de estruturas ----------

unsafe fn dword(p: usize, off: usize) -> u32 {
    *((p + off) as *const u32)
}

/// DDSURFACEDESC (0x6c bytes): campos que importam para o video.
unsafe fn desc_superficie(p: usize) -> String {
    if p == 0 {
        return "null".into();
    }
    format!(
        "flags={:#x} {}x{} pitch={} caps={:#x} fmt[flags={:#x} bpp={} r={:#x} g={:#x} b={:#x}] mem={:#x}",
        dword(p, 4), dword(p, 0xc), dword(p, 8), dword(p, 0x10) as i32, dword(p, 0x68),
        dword(p, 0x4c), dword(p, 0x54), dword(p, 0x58), dword(p, 0x5c), dword(p, 0x60), dword(p, 0x24)
    )
}

unsafe fn retangulo(p: usize) -> String {
    if p == 0 {
        return "inteira".into();
    }
    let r = p as *const i32;
    format!("({},{})-({},{})", *r, *r.add(1), *r.add(2), *r.add(3))
}

// ---------- IDirectDraw ----------

repassa!(dd_compact, 3, "DD.Compact");
repassa_cria!(dd_create_clipper, 4, "DD.CreateClipper", Tipo::Clipper, saida, flags, saida, externo);
repassa!(dd_create_palette, 5, "DD.CreatePalette", flags, cores, saida, externo);

unsafe extern "system" fn dd_create_surface(this: *mut Embrulho, desc: usize, saida: usize, externo: usize) -> i32 {
    let real = (*this).real;
    let pedido = desc_superficie(desc);
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize) -> i32 = core::mem::transmute(metodo(real, 6));
    let r = g(real, desc, saida, externo);
    if chamada("DD.CreateSurface") {
        linha(&format!("#{} DD.CreateSurface({}) -> {:#x}", (*this).id, pedido, r));
    }
    if r == 0 && saida != 0 {
        let p = saida as *mut *mut c_void;
        *p = embrulhar(*p, Tipo::Superficie);
    }
    r
}

repassa_cria!(dd_duplicate_surface, 7, "DD.DuplicateSurface", Tipo::Superficie, saida, origem, saida);
repassa!(dd_enum_display_modes, 8, "DD.EnumDisplayModes", flags, desc, ctx, cb);
repassa!(dd_enum_surfaces, 9, "DD.EnumSurfaces", flags, desc, ctx, cb);
repassa!(dd_flip_to_gdi_surface, 10, "DD.FlipToGDISurface");
repassa!(dd_get_caps, 11, "DD.GetCaps", hw, hel);
repassa!(dd_get_display_mode, 12, "DD.GetDisplayMode", desc);
repassa!(dd_get_fourcc_codes, 13, "DD.GetFourCCCodes", n, codigos);
repassa_cria!(dd_get_gdi_surface, 14, "DD.GetGDISurface", Tipo::Superficie, saida, saida);
repassa!(dd_get_monitor_frequency, 15, "DD.GetMonitorFrequency", freq);
repassa!(dd_get_scan_line, 16, "DD.GetScanLine", linha_);
repassa!(dd_get_vertical_blank_status, 17, "DD.GetVerticalBlankStatus", status);
repassa!(dd_initialize, 18, "DD.Initialize", guid);
repassa!(dd_restore_display_mode, 19, "DD.RestoreDisplayMode");
repassa!(dd_set_cooperative_level, 20, "DD.SetCooperativeLevel", hwnd, flags);
repassa!(dd_set_display_mode, 21, "DD.SetDisplayMode", largura, altura, bpp);
repassa!(dd_wait_for_vertical_blank, 22, "DD.WaitForVerticalBlank", flags, evento);

// ---------- IDirectDrawSurface ----------

repassa!(s_add_attached_surface, 3, "S.AddAttachedSurface", sup);
repassa!(s_add_overlay_dirty_rect, 4, "S.AddOverlayDirtyRect", ret);

unsafe extern "system" fn s_blt(this: *mut Embrulho, dest: usize, origem: usize, ret_origem: usize, flags: usize, fx: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize, usize, usize) -> i32 = core::mem::transmute(metodo(real, 5));
    let r = g(real, dest, desembrulhar(origem), ret_origem, flags, fx);
    if chamada("S.Blt") {
        let id_origem = if origem == 0 { 0 } else { com_estado(|e| e.por_embrulho.contains_key(&origem)).then(|| (*(origem as *const Embrulho)).id).unwrap_or(0) };
        linha(&format!(
            "#{} S.Blt(dest={} origem=#{} {} flags={:#x} fx={:#x}) -> {:#x}",
            (*this).id, retangulo(dest), id_origem, retangulo(ret_origem), flags, fx, r
        ));
    }
    r
}

repassa!(s_blt_batch, 6, "S.BltBatch", lote, n, flags);

unsafe extern "system" fn s_blt_fast(this: *mut Embrulho, x: usize, y: usize, origem: usize, ret_origem: usize, trans: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize, usize, usize) -> i32 = core::mem::transmute(metodo(real, 7));
    let r = g(real, x, y, desembrulhar(origem), ret_origem, trans);
    if chamada("S.BltFast") {
        let id_origem = if origem == 0 { 0 } else { com_estado(|e| e.por_embrulho.contains_key(&origem)).then(|| (*(origem as *const Embrulho)).id).unwrap_or(0) };
        linha(&format!(
            "#{} S.BltFast(x={} y={} origem=#{} {} trans={:#x}) -> {:#x}",
            (*this).id, x as i32, y as i32, id_origem, retangulo(ret_origem), trans, r
        ));
    }
    r
}

repassa!(s_delete_attached_surface, 8, "S.DeleteAttachedSurface", flags, sup);
repassa!(s_enum_attached_surfaces, 9, "S.EnumAttachedSurfaces", ctx, cb);
repassa!(s_enum_overlay_z_orders, 10, "S.EnumOverlayZOrders", flags, ctx, cb);
repassa!(s_flip, 11, "S.Flip", alvo, flags);
repassa_cria!(s_get_attached_surface, 12, "S.GetAttachedSurface", Tipo::Superficie, saida, caps, saida);
repassa!(s_get_blt_status, 13, "S.GetBltStatus", flags);
repassa!(s_get_caps, 14, "S.GetCaps", caps);
repassa_cria!(s_get_clipper, 15, "S.GetClipper", Tipo::Clipper, saida, saida);
repassa!(s_get_color_key, 16, "S.GetColorKey", flags, chave);
repassa!(s_get_dc, 17, "S.GetDC", hdc);
repassa!(s_get_flip_status, 18, "S.GetFlipStatus", flags);
repassa!(s_get_overlay_position, 19, "S.GetOverlayPosition", x, y);
repassa!(s_get_palette, 20, "S.GetPalette", paleta);
repassa!(s_get_pixel_format, 21, "S.GetPixelFormat", fmt);

unsafe extern "system" fn s_get_surface_desc(this: *mut Embrulho, desc: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize) -> i32 = core::mem::transmute(metodo(real, 22));
    let r = g(real, desc);
    if chamada("S.GetSurfaceDesc") {
        linha(&format!("#{} S.GetSurfaceDesc -> {:#x} {}", (*this).id, r, desc_superficie(desc)));
    }
    r
}

repassa!(s_initialize, 23, "S.Initialize", dd, desc);
repassa!(s_is_lost, 24, "S.IsLost");

unsafe extern "system" fn s_lock(this: *mut Embrulho, ret: usize, desc: usize, flags: usize, evento: usize) -> i32 {
    let real = (*this).real;
    let g: unsafe extern "system" fn(*mut c_void, usize, usize, usize, usize) -> i32 = core::mem::transmute(metodo(real, 25));
    let r = g(real, ret, desc, flags, evento);
    if chamada("S.Lock") {
        linha(&format!("#{} S.Lock({} flags={:#x}) -> {:#x} {}", (*this).id, retangulo(ret), flags, r, desc_superficie(desc)));
    }
    r
}

repassa!(s_release_dc, 26, "S.ReleaseDC", hdc);
repassa!(s_restore, 27, "S.Restore");
repassa!(s_set_clipper, 28, "S.SetClipper", clipper);
repassa!(s_set_color_key, 29, "S.SetColorKey", flags, chave);
repassa!(s_set_overlay_position, 30, "S.SetOverlayPosition", x, y);
repassa!(s_set_palette, 31, "S.SetPalette", paleta);
repassa!(s_unlock, 32, "S.Unlock", ptr);
repassa!(s_update_overlay, 33, "S.UpdateOverlay", ret_origem, dest, ret_dest, flags, fx);
repassa!(s_update_overlay_display, 34, "S.UpdateOverlayDisplay", flags);
repassa!(s_update_overlay_z_order, 35, "S.UpdateOverlayZOrder", flags, refer);

// ---------- IDirectDrawClipper ----------

repassa!(c_get_clip_list, 3, "C.GetClipList", ret, lista, tam);
repassa!(c_get_hwnd, 4, "C.GetHWnd", hwnd);
repassa!(c_initialize, 5, "C.Initialize", dd, flags);
repassa!(c_is_clip_list_changed, 6, "C.IsClipListChanged", mudou);
repassa!(c_set_clip_list, 7, "C.SetClipList", lista, flags);
repassa!(c_set_hwnd, 8, "C.SetHWnd", flags, hwnd);

fn vtable(tipo: Tipo) -> *const usize {
    static TABELAS: OnceLock<[Vec<usize>; 3]> = OnceLock::new();
    let t = TABELAS.get_or_init(|| {
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
    t[tipo as usize].as_ptr()
}
