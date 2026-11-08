use super::*;

// The comparers of the routines in ../match_events.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_events.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `record_events` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_record_events(e: usize, desc: *mut u32, human: u8) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, u8) -> u8>(IDX_RECORD_EVENTS)(e, desc, human)
}

/// Ler os eventos so le arquivo (sem efeito fora do registro): a original e a copia leem os mesmos
/// eventos (+6 quantos, +0x720 os eventos de 0xe bytes) e o resultado e conferido. Nasceu do erro
/// `Area.cpp 507` ao rever o relatorio de uma partida antiga (17/10/2026).
pub(crate) unsafe extern "thiscall" fn comparar_read_events(e: usize, desc: *const u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const u32) -> u8>(IDX_READ_EVENTS);
    if desc.is_null() {
        return o(e, desc);
    }
    let n = (*desc.add(1) as usize).min(0x10000);
    let reg = [(e + 6, 2), (e + 0x720, 0xe * n)];
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(e, desc), || read_events(e, desc));
    PLACAR.record(igual && r == rr, || format!("read_events(pos {} n {} numero {} humano {}) original={} rust={} {:x?} x {:x?}", *desc, *desc.add(1), *desc.add(2), *(desc.add(3) as *const u8), r, rr, mo, mr));
    r
}

/// Comparer for `read_events_cfg` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_read_events_cfg() {
    original_fn::<unsafe extern "C" fn()>(IDX_READ_EVENTS_CFG)()
}

/// Comparer for `event_text` (comparison mode: see the header of motor.rs).
///
/// [pt] O texto do evento so escreve nos buffers de texto ([0xb67ea0], [0xb67dd8]) e no `qual` do evento:
/// a original e a copia montam o mesmo texto e o resultado e conferido.
pub(crate) unsafe extern "stdcall" fn comparar_event_text(ev: usize, ctx: usize, comp: i32, variant: u32) -> usize {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i32, u32) -> usize>(IDX_EVENT_TEXT);
    if ev == 0 {
        return o(ev, ctx, comp, variant);
    }
    let reg = [(ev + 2, 2), (0xb67ea0, 0x400), (0xb67dd8, 0xc8)];
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(ev, ctx, comp, variant) as u32, || event_text(ev, ctx, comp, variant) as u32);
    PLACAR.record(igual && r == rr, || format!("event_text(codigo {:#x} variante {}) original={:#x} rust={:#x} {:x?} x {:x?}", *(ev as *const u16), variant, r, rr, mo, mr));
    r as usize
}

/// Comparer for `commentate_on_panel` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_commentate_on_panel(nar: usize, ev: usize, ctx: usize, comp: i32, painel: u32, width: i32, quebrar: u32, font: usize, mode: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, i32, u32, usize, u32) -> u32>(IDX_COMMENTATE_ON_PANEL)(nar, ev, ctx, comp, painel, width, quebrar, font, mode)
}

/// Comparer for `next_event` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_next_event(t: usize, out: *mut usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize) -> usize>(IDX_NEXT_EVENT);
    let mut reg = vec![(t, 0x2db4)];
    if !out.is_null() {
        reg.push((out as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, out), || next_event(t, out));
    PLACAR.record(igual && r == n, || format!("next_event({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", t, r, n, mo, mr));
    r
}

/// Comparer for `commentary_player_name` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn comparar_commentary_player_name(pt: usize, player: i8, side: i8, buf: *mut u8, size: u32, curto: u8, x: u32, longa: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i8, i8, *mut u8, u32, u8, u32, u32)>(IDX_COMMENTARY_PLAYER_NAME);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((buf as usize, size as usize));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(pt, player, side, buf, size, curto, x, longa), || crate::rng::without_counting(|| commentary_player_name(pt, player, side, buf, size, curto, x, longa)));
    PLACAR.record(igual, || format!("commentary_player_name({}, {}) {:x?} x {:x?}", player, side, mo, mr));
}

/// Comparer for `combine_with_previous` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_combine_with_previous(t: usize, e: usize, m: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i16>(IDX_COMBINE_WITH_PREVIOUS);
    if is_recording_top_level() {
        return o(t, e, m);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(e, 14), (t + 0x30, 12 * 14)]);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, e, m), || crate::rng::without_counting(|| combine_with_previous(t, e, m)));
    PLACAR.record(igual && r == n, || format!("combine_with_previous({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", core::ptr::read_unaligned(e as *const u16), r, n, mo, mr));
    r
}

/// Comparer for `event_at_front` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_event_at_front(t: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(IDX_EVENT_AT_FRONT)(t, a, b, c, d, e, f, g)
}

/// Comparer for `construct_commentary_queue` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_commentary_queue(t: usize, goal_crowd_reaction: u8) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u8) -> usize>(IDX_CONSTRUCT_COMMENTARY_QUEUE)(t, goal_crowd_reaction)
}

/// Comparer for `free_event_tables` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_free_event_tables() {
    original_fn::<unsafe extern "C" fn()>(IDX_FREE_EVENT_TABLES)()
}

/// Comparer for `sounds_finished` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_sounds_finished(e: usize, ativos: *const u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32) -> u8>(IDX_SOUNDS_FINISHED)(e, ativos)
}

/// Comparer for `event_variant` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "system" fn comparar_event_variant(e: *const i16, lados: u32, kind: u8) -> i16 {
    let o = original_fn::<unsafe extern "system" fn(*const i16, u32, u8) -> i16>(IDX_EVENT_VARIANT);
    if is_recording_top_level() {
        return o(e, lados, kind);
    }
    let reg = crate::rng::STATE_REGIONS.to_vec();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(e, lados, kind), || crate::rng::without_counting(|| event_variant(e, lados, kind)));
    PLACAR.record(igual && r == n, || format!("event_variant({:#x}, {:#x}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", *e, lados, kind, r, n, mo, mr));
    r
}
