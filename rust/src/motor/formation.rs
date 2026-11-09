use super::*;

/// Returns the tactical grid cell code of formation slot 0..10 (word array +0x12d, or the second set at +0x14e when
/// `segunda` is set). Out-of-range slots go to the original routine (error box).
///
/// [pt] { 0x5b2ae0 } (ret 8, formation.cpp): a casa da vaga 0..10 (palavra +0x12d, ou +0x14e com
/// `segunda`). Vaga fora da faixa: a original (caixa de erro e 1). Devolve so `ax`: no caminho +0x12d
/// a original deixa em cima de `eax` o que o chamador tinha ali.
pub(crate) unsafe extern "thiscall" fn slot_code(this: usize, slot: i8, second: u32) -> u16 {
    if !(0..=10).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u16>(IDX_SLOT_CODE)(this, slot, second);
    }
    let base = if second != 0 { 0x14e } else { 0x12d };
    core::ptr::read_unaligned((this + base + 2 * slot as usize) as *const u16)
}

/// Returns a grammatical attribute of a competition name [inference: its gender]: from the stage name id in the low
/// word of `b` (or else of `a`) via the comp_util name routine `exe::comp_util_get_stage_name_gender`; without either, the byte +0x37 (when `tipo` is
/// 1) or +0x52 of the competition record; with no record, 1.
///
/// [pt] { 0x4bff70 } (comp_util.cpp): com a palavra baixa de `b` (ou, sem ela, a de `a`), o resultado
/// de 0x4ba8f0(palavra, 0); senao o byte +0x37 (`tipo` 1) ou +0x52 do registro; sem registro, 1.
pub(crate) unsafe extern "C" fn competition_criterion(record: usize, a: u32, b: u32, kind: u8) -> u8 {
    if b as u16 != 0 {
        return exe::comp_util_get_stage_name_gender(b, 0);
    }
    if a as u16 != 0 {
        return exe::comp_util_get_stage_name_gender(a, 0);
    }
    if record == 0 {
        return 1;
    }
    *((record + if kind == 1 { 0x37 } else { 0x52 }) as *const u8)
}

/// With attribute masking enabled and a valid person, resets that person's knowledge level (9-byte record) in each of
/// the 16 knowledge maps of `this` that exist and are active (+4), so every manager forgets him.
///
/// [pt] { 0x5afd00 } (ret 4, fog_of_war.cpp): com a nevoa ligada (0x9cc270) e a pessoa valida, zera o
/// nivel dela (registro de 9 bytes) nos 16 quadros de [this] que existem e estao ativos (+4).
pub(crate) unsafe extern "thiscall" fn forget_person(this: usize, person: usize) {
    let valida = || *(person as *const i32) < *(G_STAFF_COUNT as *const i32) && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0;
    if person == 0 || !valida() {
        return;
    }
    for k in 0..16usize {
        let q = *((this + 4 * k) as *const usize);
        if q != 0 && *((q + 4) as *const u32) != 0 && valida() {
            *((*(q as *const usize)).wrapping_add((*(person as *const i32) as usize).wrapping_mul(9)) as *mut u8) = 0;
        }
    }
}

/// Fog of war: returns the knowledge map of manager `tecnico` (pointer table indexed backwards from the staff count:
/// [this + (staff_count - id) x 4 - 4]); 0 with no manager.
///
/// [pt] { 0x5afde0 } (ret 4, fog_of_war.cpp): o quadro do tecnico, [this + (0xae2364 - id) x 4 - 4];
/// sem tecnico, 0.
pub(crate) unsafe extern "thiscall" fn manager_table(this: usize, manager: usize) -> usize {
    if manager == 0 {
        return 0;
    }
    let k = (*(G_STAFF_COUNT as *const i32)).wrapping_sub(*(manager as *const i32)) as usize;
    *(this.wrapping_add(k.wrapping_mul(4)).wrapping_sub(4) as *const usize)
}

/// Splits the position mask into sector and side (`exe::formation_split_position_code`), finds the first of the 11 formation slots (+0x12d) of that
/// sector flagged 0x200 and returns 0 if none; with no player list returns 1, otherwise 1 if any of the 20 players
/// (0x1be-byte records, slot in +0x19, or +0x1a when +0x19 is 0xfe) occupies that slot.
///
/// [pt] { 0x5b2e80 } (ret 8, formation.cpp): a primeira das 11 casas (+0x12d) com o setor da mascara
/// (0x5b73a0) e o bit 0x200; sem ela, 0. Sem `jogadores`, 1; senao 1 se algum dos 20 (+0x19, ou
/// +0x1a com 0xfe em +0x19) esta naquela vaga.
pub(crate) unsafe extern "thiscall" fn sector_slot_taken(this: usize, mask: u32, players: usize) -> u32 {
    let (mut setor, mut lado) = (0u16, 0u16);
    exe::formation_split_position_code(mask, &mut setor, &mut lado);
    let home = |k: usize| core::ptr::read_unaligned((this + 0x12d + 2 * k) as *const u16);
    let k = match (0..11usize).find(|&k| home(k) & setor != 0 && home(k) & 0x200 != 0) {
        Some(k) => k as u8,
        None => return 0,
    };
    if players == 0 {
        return 1;
    }
    (0..20usize).any(|i| {
        let p = players + 0x19 + i * 0x1be;
        let c = *(p as *const u8);
        c == k || (c == 0xfe && *((p + 1) as *const u8) == k)
    }) as u32
}

/// Formation: returns the slot (0..10) whose 16-bit position code among the 11 at +0x12d equals `codigo`, or 0xff if
/// none.
///
/// [pt] { 0x5b2f10 } (ret 4, formation.cpp): a vaga (0..10) da casa `codigo` entre as 11 de +0x12d; 0xff
/// se nao estiver.
pub(crate) unsafe extern "thiscall" fn slot_of_code(this: usize, code: u16) -> u32 {
    (0..11u32).find(|&k| core::ptr::read_unaligned((this + 0x12d + 2 * k as usize) as *const u16) == code).unwrap_or(0xff)
}

/// marks (+0x5b4) with 4 and 8 become 2; assigned records (+0x5bf) go back to −1; returns 1
/// Original: `clear_assigned` (0x5b5960, formation.cpp, thiscall).
///
/// [pt] { 0x5b5960 } (formation.cpp): as 11 marcas (+0x5b4) com 4 e 8 viram 2, e as fichas designadas
/// (+0x5bf) voltam a -1. Devolve 1.
pub(crate) unsafe extern "thiscall" fn clear_assigned(this: usize) -> u32 {
    for k in 0..11usize {
        let b = (this + 0x5b4 + k) as *mut u8;
        if *b & 4 != 0 && *b & 8 != 0 {
            *b = 2;
        }
        core::ptr::write_unaligned((this + 0x5bf + 4 * k) as *mut i32, -1);
    }
    1
}

/// +0x187b = OR of the bits of the 11 slots (`exe::formation_position_to_shape_bit`), with 11 masks swapped for 4 reference formations
/// Original: `formation_mask` (0x5b59a0, formation.cpp, thiscall).
///
/// [pt] { 0x5b59a0 } (formation.cpp): a mascara da formacao (+0x187b) = OU dos bits das 11 casas
/// (0x5b5a60), com onze mascaras trocadas por quatro formacoes de referencia. Devolve a mascara
/// antes da troca (o eax da original).
pub(crate) unsafe extern "thiscall" fn formation_mask(this: usize) -> u32 {
    let m = (this + 0x187b) as *mut u32;
    core::ptr::write_unaligned(m, 0);
    for k in 0..11usize {
        let c = core::ptr::read_unaligned((this + 0x12d + 2 * k) as *const i16);
        let v = core::ptr::read_unaligned(m) | exe::formation_position_to_shape_bit(c);
        core::ptr::write_unaligned(m, v);
    }
    let v = core::ptr::read_unaligned(m);
    let swap = match v {
        0x101176c1 | 0x101b26c1 => 0x101f06c1,
        0x38115381 | 0x3a242381 => 0x381b0381,
        0x280e07c1 | 0x091a07c1 | 0x2884ab81 | 0x280aab81 | 0x288c07c1 | 0x281507c1 => 0x280e8b81,
        0x288a8b81 => 0x281b2381,
        _ => v,
    };
    core::ptr::write_unaligned(m, swap);
    v
}

/// Formation: returns the person record designated for position `posicao` (index at +0x5bf + 4 x position into the
/// person table at 0xae23bc, 110 bytes each), or 0 when the index is negative.
///
/// [pt] { 0x5b58e0 } (formation.cpp): a pessoa designada para a posicao (indice em +0x5bf + 4 x posicao,
/// no vetor de pessoas de 0xae23bc, 110 bytes cada); indice negativo: 0.
pub(crate) unsafe extern "thiscall" fn designated_for_position(this: usize, position: i8) -> usize {
    let i = *((this as isize + 0x5bf + 4 * position as isize) as *const i32);
    if i > -1 {
        (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)
    } else {
        0
    }
}

/// Formation: sets one instruction bit `mascara` in the 64-bit instruction mask of position `posicao` (+0x188b + 8 x
/// position), first clearing the mutually exclusive bits of its group, and clears the changed flag (+0x187f). Returns
/// 1; position outside 0..=10 or unknown bit: clears the mouse-button global and returns 0 (the original also shows an
/// error box).
///
/// [pt] { 0x5b5290 } (formation.cpp): liga o bit `mascara` (um so) na mascara de casas da posicao
/// (+0x188b + 8 x posicao, 64 bits), apagando antes o grupo dele (bits que se excluem), e zera
/// +0x187f. Posicao fora de 0..=10 ou bit desconhecido: o original mostra uma caixa de erro (sem a
/// caixa aqui), zera 0xb67a34 e devolve 0.
pub(crate) unsafe extern "thiscall" fn set_square(this: usize, position: i8, mask: u64) -> u32 {
    let group: u64 = match mask {
        0x1 | 0x2 | 0x4 | 0x8 | 0x10 => 0x1f,
        0x20 | 0x40 | 0x80 | 0x100 => 0x1e0,
        0x200 | 0x400 | 0x800 | 0x1000 => 0x1e00,
        0x2000 | 0x4000 | 0x8000 => 0xe000,
        0x10000 | 0x20000 | 0x4000_0000 => 0x4003_0000,
        0x40000 | 0x80000 | 0x8000_0000 => 0x800c_0000,
        0x10_0000 | 0x20_0000 => 0x30_0000,
        0x40_0000 | 0x80_0000 => 0xc0_0000,
        0x100_0000 | 0x200_0000 => 0x300_0000,
        0x400_0000 | 0x800_0000 => 0xc00_0000,
        0x1000_0000 | 0x2000_0000 => 0x3000_0000,
        0x1_0000_0000 | 0x2_0000_0000 => 0x3_0000_0000,
        0x4_0000_0000 | 0x8_0000_0000 => 0xc_0000_0000,
        _ => 0,
    };
    if !(0..=10).contains(&position) || group == 0 {
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        return 0;
    }
    let home = (this + 0x188b + 8 * position as usize) as *mut u64;
    *home = (*home & !group) | mask;
    *((this + 0x187f) as *mut u32) = 0;
    1
}

/// Marks the standout players of side `lado` in the side's formation (instruction bit 0x800000): each active outfield
/// player scores 10 x (three person attributes +0x43, +0x33, +0x27) plus current form (+0x4d), adjusted by role
/// (defenders down, attacking roles up); those above 8500 are flagged. Goalkeepers are skipped.
///
/// [pt] { 0x6d10a0 } (match_eng.cpp): marca (bit 0x800000 da formacao do lado, 0x5b5290) os jogadores
/// ativos cuja nota passa de 8500: 10 x (+0x43 + +0x33 + +0x27 da pessoa) + forma (+0x4d), com o
/// papel (0x717410): goleiro fica de fora; 2 +500; 4 -1000 (-500 com 0x800 ou 0x80); 8 -300 (sem
/// 0x800 e 0x80); 0x10 +1000; 0x20 +2000.
pub(crate) unsafe extern "thiscall" fn mark_stars(this: usize, side: i8) {
    for k in 0..20 {
        let j = side_player_slot(this, side, k);
        if read_i8(j + match_player::POSITION) < 0 {
            continue;
        }
        let pd = *((j + match_player::PLAYER) as *const usize);
        let sum = 10 * (read_i8(pd + 0x43) as i32 + read_i8(pd + 0x33) as i32 + read_i8(pd + 0x27) as i32);
        let mut v = ftol(sum as f64 + read_f32(j + match_player::STAMINA) as f64);
        // simplification: the original calls 0x717410 on every test; it only reads the player
        let r = match_player_position(j);
        if r & 1 != 0 {
            continue;
        }
        if r & 2 != 0 {
            v += 500;
        } else if r & 4 != 0 {
            v -= if r & 0x800 != 0 || r & 0x80 != 0 { 500 } else { 1000 };
        } else if r & 8 != 0 {
            if r & 0x800 == 0 && r & 0x80 == 0 {
                v -= 300;
            }
        } else if r & 0x10 != 0 {
            v += 1000;
        } else if r & 0x20 != 0 {
            v += 2000;
        }
        if v as i16 > 8500 {
            let write: unsafe extern "thiscall" fn(usize, i8, u64) -> u32 = core::mem::transmute(FN_FORMATION_SET_PLAYER_INSTRUCTION);
            write((this as isize + 0x91e2 + 0x18e3 * side as isize) as usize, read_i8(j + match_player::POSITION), 0x80_0000);
        }
    }
}

/// Builds the player's strong-position mask from the position ratings and the sides rated 15; returns only `ax`.
///
/// (cdecl) The mask of the strong positions of player `p` from the position ratings (signed bytes): from 15, going down
/// to 10, the first level at which any rating passes gives the sector bits (+0xf goalkeeper 1, +0x10 sweeper 2, +0x11
/// centre-back 4, +0x12 defensive midfielder 8 or, without it, +0x14 midfielder 0x20 or +0x13 0x10, +0x15 striker
/// 0x40); the sides (+0x17 0x800, +0x18 0x80, +0x19 0x200) only count with a rating of 15 or more (the original repeats
/// the same test at each level). Returns only `ax` (the callers use 16 bits). Null `p`: as in the original (error box
/// and 0).
/// Original: `position_mask` (0x5b7d20, formation.cpp, cdecl).
///
/// [pt] { 0x5b7d20 } (cdecl, formation.cpp): a máscara das posições fortes do jogador `p` pelas notas de
/// posição (bytes com sinal): a partir de 15, descendo até 10, o primeiro nível em que alguma passa
/// dá os bits do setor (+0xf goleiro 1, +0x10 líbero 2, +0x11 zagueiro 4, +0x12 volante 8 ou, sem
/// ele, +0x14 meia 0x20 ou +0x13 0x10, +0x15 atacante 0x40); os lados (+0x17 0x800, +0x18 0x80,
/// +0x19 0x200) só contam com nota 15 ou mais (a original repete o mesmo teste a cada nível).
/// Devolve só `ax` (os chamadores usam 16 bits). `p` nulo: a original (caixa de erro e 0).
pub(crate) unsafe extern "C" fn position_mask(p: usize) -> u16 {
    if p == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_POSITION_MASK)(p);
    }
    let mut m = 0u16;
    let mut level = 15i8;
    loop {
        if read_i8(p + 0xf) >= level {
            m |= 1;
        }
        if read_i8(p + 0x11) >= level {
            m |= 4;
        }
        if read_i8(p + 0x10) >= level {
            m |= 2;
        }
        if read_i8(p + 0x12) >= level {
            m |= 8;
        } else if read_i8(p + 0x14) >= level {
            m |= 0x20;
        } else if read_i8(p + 0x13) >= level {
            m |= 0x10;
        }
        if read_i8(p + 0x15) >= level {
            m |= 0x40;
        }
        level -= 1;
        if level < 10 || m != 0 {
            break;
        }
    }
    if read_i8(p + 0x17) >= 15 {
        m |= 0x800;
    }
    if read_i8(p + 0x18) >= 15 {
        m |= 0x80;
    }
    if read_i8(p + 0x19) >= 15 {
        m |= 0x200;
    }
    m
}

/// Returns the pitch cell (column 0..26, row 0..35) of a position by attack and band.
///
/// (cdecl) The pitch cell (`out[0]` = column 0..26, `out[1]` = row 0..35) of position `mask` (split by `exe::formation_split_position_code` into
/// sector and side), with `attack` 0/1 and `band` = low word 0..2 (left, centre, right) and high word 0..3 (the height
/// in the formation). The row comes from the sector (tables of 4 by height; in attack, the full-backs 0x80/0x800 move
/// up 1, or 1/4 in sectors 2 and 4); the column, from the side (tables of 5); then the nudges (of 1 or 5) in the bands
/// and the limits. Arguments out of range: as in the original (error box and 0). Returns 1.
/// Original: `pitch_cell` (0x5b7e60, formation.cpp, cdecl).
///
/// [pt] { 0x5b7e60 } (cdecl, formation.cpp): a casa no campo (`saida[0]` = coluna 0..26, `saida[1]` =
/// linha 0..35) da posição `mascara` (separada pela 0x5b73a0 em setor e lado), com `ataque` 0/1 e
/// `faixa` = palavra baixa 0..2 (esquerda, meio, direita) e palavra alta 0..3 (a altura na
/// formação). A linha vem do setor (tabelas de 4 pela altura; no ataque, os laterais 0x80/0x800
/// sobem 1, ou 1/4 nos setores 2 e 4); a coluna, do lado (tabelas de 5); depois os encostos nas
/// faixas e os limites. Argumentos fora da faixa: a original (caixa de erro e 0). Devolve 1.
pub(crate) unsafe extern "C" fn pitch_cell(mask: u32, attack: i8, band: u32, out: *mut i16) -> u32 {
    let low = band as u16 as i16;
    let high = (band >> 16) as u16 as i16;
    if !(0..=1).contains(&attack) || !(0..=2).contains(&low) || !(0..=3).contains(&high) {
        return original_fn::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(IDX_PITCH_CELL)(mask, attack, band, out);
    }
    let (mut setor, mut lado) = (0u16, 0u16);
    exe::formation_split_position_code(mask, &mut setor, &mut lado);
    let a = high as usize;
    let throw_in = lado & 0x80 != 0 || lado & 0x800 != 0;
    let at = attack != 0;
    // a linha (saida[1])
    let row: i16 = if setor & 1 != 0 {
        if at { [0, 2, 4, 6][a] } else { [0, 1, 2, 3][a] }
    } else if setor & 2 != 0 {
        if !at {
            [1, 3, 5, 9][a]
        } else {
            let v = [2, 4, 13, 15][a];
            if throw_in { v + if high != 3 { 1 } else { 4 } } else { v }
        }
    } else if setor & 4 != 0 {
        if !at {
            [2, 7, 9, 13][a]
        } else {
            let v = [4, 9, 17, 17][a];
            if throw_in { v + if high != 3 { 1 } else { 4 } } else { v }
        }
    } else {
        let (sem, com): ([i16; 4], [i16; 4]) = if setor & 8 != 0 {
            ([4, 9, 13, 18], [7, 11, 20, 22])
        } else if setor & 0x10 != 0 {
            ([6, 11, 16, 22], [9, 14, 23, 26])
        } else if setor & 0x20 != 0 {
            ([9, 13, 19, 26], [12, 19, 27, 30])
        } else {
            ([13, 15, 23, 31], [15, 24, 31, 34])
        };
        if !at {
            sem[a]
        } else if throw_in {
            com[a] + 1
        } else {
            com[a]
        }
    };
    *out.add(1) = row;
    // the column (saida[0]), by side
    let l = if lado & 0x80 != 0 {
        0
    } else if lado & 0x100 != 0 {
        1
    } else if lado & 0x200 != 0 {
        2
    } else if lado & 0x400 != 0 {
        3
    } else {
        4
    };
    let table: [i16; 5] = if setor & 1 != 0 {
        [13; 5]
    } else if setor & 6 != 0 || setor & 0x38 != 0 {
        match (at, high == 0, setor & 6 != 0) {
            (false, true, _) => [9, 11, 13, 15, 17],
            (false, false, _) => [7, 11, 13, 15, 19],
            (true, true, true) => [6, 11, 13, 15, 20],
            (true, _, _) => [3, 10, 13, 16, 23],
        }
    } else if !at {
        [7, 11, 13, 15, 19]
    } else if high == 0 || high == 1 {
        [2, 8, 13, 18, 24]
    } else if high == 2 {
        [3, 10, 13, 16, 23]
    } else {
        [6, 11, 13, 15, 20]
    };
    *out = table[l];
    // the cushions: with `perto`, one step in the band; without, five (except the goalkeeper)
    let near = (setor & 1 != 0 && high > 0)
        || (setor & 6 != 0 && high == 0)
        || (setor & 0x40 != 0 && lado & 0x700 != 0 && high == 3 && attack == 1);
    let step_towards = if near { 1 } else if setor & 1 == 0 { 5 } else { 0 };
    if step_towards != 0 {
        let v = *out as i32;
        if low == 0 {
            *out = (v - step_towards).clamp(2, 0x11) as i16;
        } else if low == 2 {
            *out = (v + step_towards).clamp(9, 0x18) as i16;
        }
    }
    *out = (*out).clamp(0, 0x1a);
    *out.add(1) = (*out.add(1)).clamp(0, 0x23);
    1
}

/// Checks the 11 formation slots for conflicts: 0 if any slot with an open side is also a striker's, midfielder's,
/// defensive or attacking midfielder's; returns only `al`.
///
/// (thiscall) The 11 slots of formation `this` without conflict: returns 0 if any slot with an open side (0x880 in cell
/// +0x12d or +0x14e, see `exe::formation_get_slot_position`) is also a striker's (0x40, in both), a midfielder's (0x20 in +0x12d and in +0x14e
/// through `exe::formation_get_slot_position`) or a defensive midfielder's and attacking midfielder's through `exe::formation_get_slot_position` (8 and 0x10); otherwise 1.
/// Returns only `al`.
/// Original: `slots_without_conflict` (0x5b8960, formation.cpp, thiscall).
///
/// [pt] { 0x5b8960 } (thiscall, formation.cpp): as 11 vagas da formação `this` sem conflito: devolve 0
/// se alguma vaga com lado aberto (0x880 na casa +0x12d ou +0x14e, ver 0x5b2ae0) é também de
/// atacante (0x40, nas duas), de meia (0x20 na +0x12d e na +0x14e pela 0x5b2ae0) ou de volante e
/// meia-atacante pela 0x5b2ae0 (8 e 0x10); senão 1. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn slots_without_conflict(this: usize) -> u8 {
    for i in 0..=10u32 {
        let a = read_u16(this + (0x12d + 2 * i as usize));
        let b = read_u16(this + (0x14e + 2 * i as usize));
        if a & 0x880 != 0 && a & 0x40 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 0x40 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 0x20 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 0x20 != 0 {
            return 0;
        }
        if exe::formation_get_slot_position(this, i, 0) & 0x880 != 0 && exe::formation_get_slot_position(this, i, 0) & 8 != 0 {
            return 0;
        }
        if exe::formation_get_slot_position(this, i, 0) & 0x880 != 0 && exe::formation_get_slot_position(this, i, 0) & 0x10 != 0 {
            return 0;
        }
        if exe::formation_get_slot_position(this, i, 1) & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 0x10 != 0 {
            return 0;
        }
    }
    1
}

compare_value!(compare_slots_without_conflict, slots_without_conflict, IDX_SLOTS_WITHOUT_CONFLICT, () -> u8);

/// Stores the slot coordinate for the phase and ball zone and picks the general and extra positions closest to the
/// slot's 12 coordinates.
///
/// `Formation::set_slot_coordinate(cell, phase, band, coord)`: writes the coordinate (dword: x 0..0x1a, y 0..0x23) of
/// the cell for the phase and the ball band (+0x164, 11 × 2 × 3 × 4 dwords) and, for each phase, finds the positions of
/// table 0x9cc368 (words up to 0xffff) closest to the cell's 12 coordinates in that phase (|Δx·9/27| + |Δy·12/36|
/// against each position's cell through `pitch_cell`; doubled for positions with bit 2; ties go into the phase's list, up
/// to 11 contiguous at [esp+0x64]). The general position (+0x12d) stays if it is in one of the lists; otherwise it
/// becomes the first of the phase with the smallest distance (`formation_mask` redoes the mask, the cell gets the flag +0x143
/// and the other cells with the same position lose it). The extra position (+0x14e) comes from the other phase's list:
/// the same position, or one with the same side (`exe::formation_split_position_code`), or the first, or, with two, the middle of the two's bits.
/// Zeroes +0x187f. Arguments out of range: as in the original (box and 0). Returns 1.
/// Original: `Formation::set_slot_coordinate` (0x5b3560, formation.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5b3560 } (thiscall, ret 0x10, formation.cpp): `Formation::set_slot_coordinate(casa, fase, faixa, coord)`:
/// grava a coordenada (dword: x 0..0x1a, y 0..0x23) da casa na fase e na faixa da bola (+0x164, 11 × 2 × 3 × 4
/// dwords) e, para cada fase, acha as posições da tabela 0x9cc368 (palavras até 0xffff) mais perto das 12 coordenadas
/// da casa nessa fase (|Δx·9/27| + |Δy·12/36| contra a casa de cada posição pela 0x5b7e60; dobrada nas posições com o
/// bit 2; os empates entram na lista da fase, até 11 em [esp+0x64] contíguas). A posição geral (+0x12d) fica se
/// estiver numa das listas; senão vira a primeira da fase de menor distância (a 0x5b59a0 refaz a máscara, a casa
/// ganha a marca +0x143 e as outras casas com a mesma posição a perdem). A posição extra (+0x14e) sai da lista da
/// outra fase: a mesma posição, ou a de mesmo lado (0x5b73a0), ou a primeira, ou com duas o meio dos bits das duas.
/// Zera +0x187f. Argumentos fora da faixa: a original (caixa e 0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn set_slot_coordinate(this: usize, home: u32, stage: u32, band: u32, coord: u32) -> u32 {
    let (s, f) = (home as u8 as i8, stage as u8 as i8);
    let (zx, zy) = (band as u16 as i16, (band >> 16) as u16 as i16);
    let (cx, cy) = (coord as u16 as i16, (coord >> 16) as u16 as i16);
    if !(0..=10).contains(&s) || !(0..=1).contains(&f) || !(0..=2).contains(&zx) || !(0..=3).contains(&zy)
        || !(0..=0x1a).contains(&cx) || !(0..=0x23).contains(&cy)
    {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_SET_SLOT_COORDINATE)(this, home, stage, band, coord);
    }
    let su = s as usize;
    let cel = this + 0x164 + 4 * (zy as usize + 4 * (zx as usize + 3 * (f as usize + 2 * su)));
    crate::comparison::branch("cpu_11 coordinate recorded");
    core::ptr::write_unaligned(cel as *mut u32, coord); // cpu11: a coordenada
    // the output of 0x5b7e60 is the coord argument itself in the original (a single slot, for the whole function)
    let mut out: [i16; 2] = [cx, cy];
    // simplification: the lists of the two stages stay contiguous as at [esp+0x64]; indexes beyond 0xea do not happen
    let mut cand = [0u16; 0xea];
    let mut count = [0i8; 2]; // uninitialized in the original; the first position always zeroes it
    let mut best = [0i32; 2];
    let mut last_dy: i32 = 0; // [esp+0x60], read only by the indices outside the table at the end
    for p in 0..2usize {
        best[p] = 0x7fffffff;
        let mut list = 0x9cc368usize;
        while read_i16(list) != -1 {
            let pos = read_i16(list) as u16;
            let base = this + 0x164 + su * 0x60 + p * 0x30;
            let mut dist: i32 = 0;
            for zx in 0..3u32 {
                for zy in 0..4u32 {
                    // the mask goes with the band height in the upper 16 bits (the original’s ecx); 0x5b7e60 does not read them
                    exe::formation_get_default_coordinate(pos as u32 | zy << 16, p as i8, zx | zy << 16, out.as_mut_ptr());
                    let c = base + 4 * (zx as usize * 4 + zy as usize);
                    let dy = ((read_i16(c + 2) as i32 * 12) / 36 - (out[1] as i32 * 12) / 36).wrapping_abs();
                    let dx = ((read_i16(c) as i32 * 9) / 27 - (out[0] as i32 * 9) / 27).wrapping_abs();
                    last_dy = dy;
                    dist = dx.wrapping_add(dist).wrapping_add(dy);
                }
            }
            if pos & 2 != 0 {
                crate::comparison::branch("cpu_11 distance doubled");
                dist = dist.wrapping_add(dist); // cpu11: o dobro
            }
            if dist <= best[p] { // cpu11: the tie goes into the list
                if dist < best[p] {
                    count[p] = 0;
                } else {
                    crate::comparison::branch("cpu_11 tie in the list");
                }
                best[p] = dist;
                cand[(count[p] as i32 + p as i32 * 11) as usize] = pos;
                count[p] = count[p].wrapping_add(1);
            }
            list += 2;
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    let general = this + 0x12d + su * 2;
    let mut sf: i8 = (best[0] > best[1]) as i8;
    let mut si: i8 = 0;
    for q in 0..2i8 {
        let mut a: i8 = 0;
        while a < count[q as usize] {
            if cand[(a as i32 + q as i32 * 11) as usize] == read_i16(general) as u16 {
                sf = q;
                si = a;
                break;
            }
            a += 1;
        }
    }
    let new = cand[(sf as i32 * 11 + si as i32) as usize];
    if read_i16(general) as u16 != new {
        write_u16(general, new);
        exe::formation_update_shape_code(this);
        *((this + 0x143 + su) as *mut u8) = 1;
        for k in 0..11i8 {
            if k != s && read_i16(this + 0x12d + k as usize * 2) as u16 == new {
                *((this + 0x143 + k as usize) as *mut u8) = 0;
            }
        }
    }
    let extra = this + 0x14e + su * 2;
    let o = (1 - sf as i32) * 11; // the start of the other stage’s list
    let no = count[(1 - sf) as usize];
    let current = read_i16(general) as u16;
    for a in 0..no.max(0) as i32 {
        if cand[(o + a) as usize] == current {
            write_u16(extra, cand[(o + a) as usize]);
            return 1;
        }
    }
    // simplification: the outputs of 0x5b73a0 are uninitialized in the original if it fails (position without a side); here 0
    let (mut setor, mut lado_c, mut lado_a) = (0u16, 0u16, 0u16);
    for b in 0..no.max(0) as i32 {
        exe::formation_split_position_code(cand[(o + b) as usize] as u32, &mut setor, &mut lado_c);
        exe::formation_split_position_code(read_i16(general) as u16 as u32, &mut setor, &mut lado_a);
        if lado_c == lado_a {
            write_u16(extra, cand[(o + b) as usize]);
            return 1;
        }
    }
    if no != 2 {
        write_u16(extra, cand[o as usize]);
        return 1;
    }
    // two in the other stage: the middle of the sector bits (1..0x40) and side bits (0x80..0x800) of both. The frame words
    // from [esp+0x38]: the sides table and 0xffff, the band (3) and 0, the best-distance pointer (uninitialized
    // reproducible: 0), this, the sector table and 0xffff, the last |Δy| and the lists.
    let (c0, c1) = (cand[o as usize], cand[(o + 1) as usize]);
    let mut q = [0u16; 0x100];
    q[..6].copy_from_slice(&[0x80, 0x100, 0x200, 0x400, 0x800, 0xffff]);
    q[6] = 3;
    q[10] = this as u16;
    q[11] = (this >> 16) as u16;
    q[12..20].copy_from_slice(&[1, 2, 4, 8, 0x10, 0x20, 0x40, 0xffff]);
    q[20] = last_dy as u16;
    q[21] = (last_dy >> 16) as u16;
    q[22..].copy_from_slice(&cand[..0xea]);
    let mut v1a: i8 = count[0]; // [esp+0x1a]
    let mut v12: i8 = no; // [esp+0x12]
    for k in 0..7usize {
        if q[12 + k] & c0 != 0 {
            v1a = k as i8;
        }
        if q[12 + k] & c1 != 0 {
            v12 = k as i8;
        }
    }
    let mut v13: i8 = si; // [esp+0x13]
    let mut dl: i8 = v1a;
    for k in 0..5usize {
        if q[k] & c0 != 0 {
            v13 = k as i8;
        }
        if q[k] & c1 != 0 {
            dl = k as i8;
        }
    }
    let ib = (v13 as i32 + dl as i32) / 2;
    let ia = (v12 as i32 + v1a as i32) / 2;
    write_u16(extra, q[ib as usize & 0xff] | q[(12 + ia) as usize & 0xff]);
    1
}

/// Finds another slot with the same general position and sets the output when its flag is lower.
///
/// `Formation::check_duplicate_position(cell, *out)`: zeroes the output; the first other cell with the same general
/// position (+0x12d) decides: with a flag (+0x143, signed byte) lower than the cell's, output 1 and returns 1;
/// otherwise returns 0. With no duplicate, 1. Cell outside 0..10: as in the original (box and 0).
/// Original: `Formation::check_duplicate_position` (0x5b3460, formation.cpp, thiscall, ret 8).
///
/// [pt] { 0x5b3460 } (thiscall, ret 8, formation.cpp): `Formation::check_duplicate_position(casa, *saida)`: zera a saída;
/// a primeira outra casa com a mesma posição geral (+0x12d) decide: com marca (+0x143, byte com sinal) menor que a da
/// casa, saída 1 e devolve 1; senão devolve 0. Sem repetida, 1. Casa fora de 0..10: a original (caixa e 0).
pub(crate) unsafe extern "thiscall" fn check_duplicate_position(this: usize, home: u32, out: *mut u32) -> u32 {
    let s = home as u8 as i8;
    if !(0..=10).contains(&s) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(IDX_CHECK_DUPLICATE_POSITION)(this, home, out);
    }
    *out = 0;
    let su = s as usize;
    for k in 0..11usize {
        if k != su && read_i16(this + 0x12d + k * 2) == read_i16(this + 0x12d + su * 2) {
            if *((this + 0x143 + k) as *const i8) < *((this + 0x143 + su) as *const i8) {
                *out = 1;
                return 1;
            }
            return 0;
        }
    }
    1
}

/// Sets one marking type of the slot (bits 0..2 or 3..4 of byte +0x5b4) and clears +0x187f.
///
/// `Formation::set_marking_type(cell, marking)`: markings 1, 2 and 4 replace bits 0..2 of the cell's byte +0x5b4; 8 and
/// 0x10, bits 3..4; it zeroes +0x187f and returns 1. Any other marking: as in the original (box and 0). The cell is not
/// checked.
/// Original: `Formation::set_marking_type` (0x5b57d0, formation.cpp, thiscall, ret 8).
///
/// [pt] { 0x5b57d0 } (thiscall, ret 8, formation.cpp): `Formation::set_marking_type(casa, marca)`: as marcas 1, 2 e 4
/// trocam os bits 0..2 do byte +0x5b4 da casa; 8 e 0x10, os bits 3..4; zera +0x187f e devolve 1. Outra marca: a
/// original (caixa e 0). A casa não é conferida.
pub(crate) unsafe extern "thiscall" fn set_marking_type(this: usize, home: u32, mark: u32) -> u32 {
    let m = mark as u8;
    let limpa = match m {
        1 | 2 | 4 => 0xf8u8,
        8 | 0x10 => 0xe7,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_MARKING_TYPE)(this, home, mark),
    };
    let b = (this as isize + home as u8 as i8 as isize + 0x5b4) as *mut u8;
    *b = *b & limpa | m;
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// Formation memory regions the three slot setters write: positions/marks/coordinates (+0x12d..+0x584), marking types
/// (+0x5b4) and the mask (+0x187b).
///
/// [pt] As regiões da formação que as três gravam: as posições, as marcas, as extras e as coordenadas (+0x12d..+0x584),
/// os tipos de marcação (+0x5b4, 11) e a máscara com +0x187f (+0x187b, 8).
pub(crate) fn formation_regions(this: usize) -> [(usize, usize); 3] {
    [(this + 0x12d, 0x584 - 0x12d), (this + 0x5b4, 11), (this + 0x187b, 8)]
}

/// Expands each formation slot's positions into the pitch cell tables and their mirrored copy.
///
/// (thiscall) The formation's cell map. For each of the 22 lines k (11 cells × 2 phases) and 3 × 4 block (a18, a14)
/// with the position (i = 12k + 4·a18 + a14) at +0x164 (column, word) and +0x166 (row), the 3 × 3 cells of the block
/// get the column × 9 / 27 in the low nibble and the row × 12 / 36 in the high nibble of table +0x5eb (12 per row, 9
/// rows per k); the mirror at +0xf9e (row and column inverted) receives (0xb0 − row × 16) with (0xf8 − column) & 0xf.
/// Returns 1.
/// Original: `Formation::build_position_lookup` (0x5b7160, formation.cpp, thiscall).
///
/// [pt] { 0x5b7160 } (thiscall, formation.cpp): o mapa de casas da formação. Para cada uma das 22 linhas
/// k (11 casas × 2 fases) e bloco 3 × 4 (a18, a14) com a posição (i = 12k + 4·a18 + a14) em +0x164
/// (coluna, palavra) e +0x166 (linha), as 3 × 3 células do bloco ganham a coluna × 9 / 27 no nibble
/// baixo e a linha × 12 / 36 no alto da tabela +0x5eb (12 por fila, 9 filas por k); o espelho em +0xf9e
/// (fila e coluna invertidas) recebe (0xb0 − linha × 16) com (0xf8 − coluna) & 0xf. Devolve 1.
pub(crate) unsafe extern "thiscall" fn build_position_lookup(this: usize) -> u32 {
    for k in 0..22i32 {
        for a18 in 0..3i32 {
            for a14 in 0..4i32 {
                let i = (12 * k + 4 * a18 + a14) as usize;
                for y in 3 * a18..3 * a18 + 3 {
                    for x in 3 * a14..3 * a14 + 3 {
                        let column = core::ptr::read_unaligned((this + i * 4 + 0x164) as *const i16) as i32;
                        let c = (column * 9 / 27) as u8;
                        let p = (this as isize + 0x5eb + ((9 * k + y) * 12 + x) as isize) as *mut u8;
                        *p = (*p & 0xf0) | (c & 0xf);
                        let row = core::ptr::read_unaligned((this + i * 4 + 0x166) as *const i16) as i32;
                        let d = ((row * 12 / 36) as u8) << 4;
                        *p = (c & 0xf) | d;
                        let q = (this as isize + 0xf9e + ((9 * k - y) * 12 - x) as isize) as *mut u8;
                        *q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho
                    }
                }
            }
        }
    }
    1
}

/// Record (42735 bytes in [this]) of the match the human manager (or the on-screen one) is watching: Ok(0) without
/// human, Err when the index is out of range.
///
/// [pt] O registro (42735 bytes em [this]) da partida que o `humano` (ou o da tela, 0xb74c26 + 0x300 ×
/// [0xb77c7a]) assiste: Ok(0) sem humano; Err se o índice (id − [0xae2364] + 0x10) sai de 0..[this+4].
pub(crate) unsafe fn formation_watched_match_record(this: usize, human: usize) -> Result<usize, ()> {
    let h = if human != 0 {
        human
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return Ok(0);
    }
    let i = (*(h as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(());
    }
    Ok((*(this as *const usize)).wrapping_add((i as usize).wrapping_mul(42735)))
}

/// Returns the stored cell of the slot by attack and band.
///
/// The stored cell of slot `slot` 0..10 with `attack` 0/1 and `band` (low word 0..2, high word 0..3): the dword +0x164
/// + 4×(high + 4×(low + 3×(attack + 2×slot))) goes to `*out`. Returns 1. Out of range: as in the original (error box).
/// Original: `stored_cell` (0x5b3c70, formation.cpp, thiscall).
///
/// [pt] { 0x5b3c70 } (thiscall, ret 0x10, formation.cpp): a casa guardada da vaga `vaga` 0..10 com
/// `ataque` 0/1 e `faixa` (palavra baixa 0..2, alta 0..3): o dword +0x164 + 4×(alta + 4×(baixa +
/// 3×(ataque + 2×vaga))) vai para `*saida`. Devolve 1. Fora das faixas: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn stored_cell(t: usize, slot: i8, attack: i8, band: u32, out: *mut u32) -> u32 {
    let low = band as u16 as i16;
    let high = (band >> 16) as u16 as i16;
    if !(0..=10).contains(&slot) || !(0..=1).contains(&attack) || !(0..=2).contains(&low) || !(0..=3).contains(&high) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(IDX_STORED_CELL)(t, slot, attack, band, out);
    }
    let i = high as usize + 4 * (low as usize + 3 * (attack as usize + 2 * slot as usize));
    *out = core::ptr::read_unaligned((t + 0x164 + 4 * i) as *const u32);
    1
}

/// Sets a formation flag in +0x584, clearing the others of the same group, and zeroes +0x187f.
///
/// Sets flag `m` in +0x584, first clearing those of the same group (1/2/4/8; 0x10/0x20/0x40; 0x80/0x100; 0x200/0x400;
/// 0x800/0x1000; 0x2000/0x4000; 0x8000/0x10000/0x20000) and zeroes +0x187f. Returns 1. Flag outside the groups: as in
/// the original (error box).
/// Original: `set_formation_flag` (0x5b50d0, formation.cpp, thiscall).
///
/// [pt] { 0x5b50d0 } (thiscall, ret 4, formation.cpp): liga a marca `m` em +0x584 apagando antes as do
/// mesmo grupo (1/2/4/8; 0x10/0x20/0x40; 0x80/0x100; 0x200/0x400; 0x800/0x1000; 0x2000/0x4000;
/// 0x8000/0x10000/0x20000) e zera +0x187f. Devolve 1. Marca fora dos grupos: a original (caixa de
/// erro).
pub(crate) unsafe extern "thiscall" fn set_formation_flag(t: usize, m: u32) -> u32 {
    let group: u32 = match m {
        1 | 2 | 4 | 8 => 0xf,
        0x10 | 0x20 | 0x40 => 0x70,
        0x80 | 0x100 => 0x180,
        0x200 | 0x400 => 0x600,
        0x800 | 0x1000 => 0x1800,
        0x2000 | 0x4000 => 0x6000,
        0x8000 | 0x10000 | 0x20000 => 0x38000,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_FORMATION_FLAG)(t, m),
    };
    let marks = (t + 0x584) as *mut u32;
    let v = core::ptr::read_unaligned(marks) & !group | m;
    core::ptr::write_unaligned((t + 0x187f) as *mut u32, 0);
    core::ptr::write_unaligned(marks, v);
    1
}

/// Reads a formation from a .pct file, fixes the instructions of old versions, redoes the positions of the 11 slots
/// from the coordinates and builds the coordinate map.
///
/// `Formation::load_from_disk(path, quiet, obfuscated)`, from the creation of the tactics (0x893dc0: the AI formations
/// and the predefined ones, 45 in a career) and from the tactics screens. Null path (line 0x8d8) or a file that does
/// not open (0x8e0): 0 (without `quiet`, the box). It reads the identifier (4 bytes; byte-swapped if the file's marker
/// `exe::utils_read_file_format_marker` is not the native one; minus 123456789 if obfuscated): outside 0x98ec54..0x98ec5e "not a formation"
/// (0x8f7), below 0x98ec59 "old version" (0x900). Then, in order: the password (+0x33, 50 bytes, each byte inverted,
/// 0xff stays), the creator (+0x65, 200), the mask (+0x187b, from 0x98ec5a; before that 0), the 11 general positions
/// (+0x12d, words) and the mask redone (`formation_mask`), the display order (+0x143, 11 bytes, from 0x98ec5c; before that
/// zeros), the 11 extra positions (+0x14e), the cells (+0x164, 0x420 bytes = 0x210 words), in versions
/// 0x98ec59/0x98ec5a 0x108 bytes discarded, the team instructions (+0x584, dword), the player instructions (below
/// 0x98ec5e 11 dwords, each with 10 alongside, at +0x188b every 8; after that 0x58 bytes at +0x188b) and the marking
/// types (+0x5b4, 11 bytes); the last three, if they fail, only show the box. The team instructions get 0x2000 (without
/// 0x4000) and, below 0x98ec5d, 0x20000. The 11 slots redo their positions from the stored cell (`stored_cell` and `set_slot_coordinate`
/// with phase 0 and band 0); the name (+0) is the file's (_splitpath `exe::crt_strtok`); the cell map (`build_position_lookup`); +0x187f = 1,
/// +0x1883 = password not empty, +0x1887 = 1. Closes and returns 1. Non-native files swap the byte order (`swap_bytes_16_array`
/// words, `swap_bytes_32_array` dwords, `exe::swap_bytes_64_array` qwords).
/// Original: `Formation::load_from_disk` (0x5b5d10, formation.cpp, thiscall).
///
/// [pt] { 0x5b5d10 } (thiscall, ret 0xc, formation.cpp): `Formation::load_from_disk(caminho, quieto, ofuscado)`, pela
/// criação das táticas (0x893dc0: as formações da IA e as pré-definidas, 45 na carreira) e pelas telas de tática.
/// Caminho nulo (linha 0x8d8) ou o arquivo que não abre (0x8e0): 0 (sem `quieto`, a caixa). Lê o identificador (4 bytes;
/// invertido se o marcador do arquivo 0x90c140 não é o nativo; menos 123456789 se ofuscado): fora de 0x98ec54..0x98ec5e
/// "não é formação" (0x8f7), abaixo de 0x98ec59 "versão antiga" (0x900). Depois, na ordem: a senha (+0x33, 50 bytes,
/// cada byte invertido, 0xff fica), o criador (+0x65, 200), a máscara (+0x187b, a partir de 0x98ec5a; antes 0), as 11
/// posições gerais (+0x12d, palavras) e a máscara refeita (0x5b59a0), a ordem de exibição (+0x143, 11 bytes, a partir de
/// 0x98ec5c; antes zeros), as 11 posições extras (+0x14e), as casas (+0x164, 0x420 bytes = 0x210 palavras), nas versões
/// 0x98ec59/0x98ec5a 0x108 bytes descartados, as instruções do time (+0x584, dword), as instruções dos jogadores (abaixo
/// de 0x98ec5e 11 dwords, cada um com 10 ao lado, em +0x188b de 8 em 8; depois 0x58 bytes em +0x188b) e os tipos de
/// marcação (+0x5b4, 11 bytes); as três últimas, se falham, só mostram a caixa. As instruções do time ganham 0x2000
/// (sem 0x4000) e, abaixo de 0x98ec5d, 0x20000. As 11 vagas refazem as posições pela casa guardada (0x5b3c70 e
/// 0x5b3560 com a fase 0 e a faixa 0); o nome (+0) é o do arquivo (_splitpath 0x944cff); o mapa das casas (0x5b7160);
/// +0x187f = 1, +0x1883 = senha não vazia, +0x1887 = 1. Fecha e devolve 1. Os arquivos não nativos trocam a ordem dos
/// bytes (0x43f2d0 palavras, 0x43f300 dwords, 0x43f350 qwords).
pub(crate) unsafe extern "thiscall" fn load_from_disk(this: usize, path: usize, quiet: u32, ofuscado: u32) -> u32 {
    if path == 0 {
        if quiet == 0 {
            formation_error_box(0x8d8);
        }
        return 0;
    }
    let f = exe::crt_fopen(path, 0x98f928);
    if f == 0 {
        if quiet == 0 {
            formation_error_box(0x8e0);
        }
        return 0;
    }
    let nativo = exe::utils_read_file_format_marker(f) != 0xf1ff_fff1;
    let mut id_ = [0u8; 4];
    if crt_fread_one(id_.as_mut_ptr() as usize, 4, f) != 1 {
        return fail_read(quiet, 0x8ea, f);
    }
    let mut id = u32::from_le_bytes(id_);
    if !nativo {
        id = swap_bytes_u32(id);
    }
    if ofuscado != 0 {
        id = id.wrapping_sub(0x75bcd15);
    }
    let id = id as i32;
    if !(0x98ec54..=0x98ec5e).contains(&id) {
        return fail_read(quiet, 0x8f7, f);
    }
    if id < 0x98ec59 {
        return fail_read(quiet, 0x900, f);
    }
    let senha = this + 0x33;
    if crt_fread_one(senha, 0x32, f) != 1 {
        return fail_read(quiet, 0x909, f);
    }
    let strlen = |a: usize| (0..).take_while(|&k| *((a + k) as *const u8) != 0).count();
    if strlen(senha) != 0 {
        let mut k = 0usize;
        loop {
            let b = *((senha + k) as *const u8);
            *((senha + k) as *mut u8) = if b == 0xff { 0xff } else { !b }; // cqg8: a senha invertida
            k += 1;
            if k >= strlen(senha) {
                break;
            }
        }
    }
    if crt_fread_one(this + 0x65, 200, f) != 1 {
        return fail_read(quiet, 0x915, f);
    }
    if id >= 0x98ec5a {
        if crt_fread_one(this + 0x187b, 4, f) != 1 {
            return fail_read(quiet, 0x920, f);
        }
        if !nativo {
            write_u32(this + 0x187b, swap_bytes_u32(read_u32(this + 0x187b)));
        }
    } else {
        write_u32(this + 0x187b, 0);
    }
    if crt_fread_one(this + 0x12d, 0x16, f) != 1 {
        return fail_read(quiet, 0x930, f);
    }
    if !nativo {
        exe::swap_bytes_16_array(this + 0x12d, 0xb);
    }
    exe::formation_update_shape_code(this);
    if id >= 0x98ec5c {
        if crt_fread_one(this + 0x143, 0xb, f) != 1 {
            return fail_read(quiet, 0x940, f);
        }
    } else {
        core::ptr::write_bytes((this + 0x143) as *mut u8, 0, 0xb);
    }
    if crt_fread_one(this + 0x14e, 0x16, f) != 1 {
        return fail_read(quiet, 0x94f, f);
    }
    if !nativo {
        exe::swap_bytes_16_array(this + 0x14e, 0xb);
    }
    if crt_fread_one(this + 0x164, 0x420, f) != 1 {
        return fail_read(quiet, 0x959, f);
    }
    if !nativo {
        exe::swap_bytes_16_array(this + 0x164, 0x210);
    }
    if id == 0x98ec59 || id == 0x98ec5a {
        let mut descarte = [0u8; 0x108];
        if crt_fread_one(descarte.as_mut_ptr() as usize, 0x108, f) != 1 {
            return fail_read(quiet, 0x968, f);
        }
        if !nativo {
            exe::swap_bytes_16_array(descarte.as_mut_ptr() as usize, 0x84);
        }
    }
    let team = this + 0x584;
    if crt_fread_one(team, 4, f) == 1 {
        if !nativo {
            write_u32(team, swap_bytes_u32(read_u32(team)));
        }
    } else if quiet == 0 {
        formation_error_box(0x971);
    }
    if id < 0x98ec5e {
        // simplification: the original copies the local array even when the read fails (stack leftover in a truncated file);
        // the copy starts with zeros
        let mut v = [0u32; 11];
        if crt_fread_one(v.as_mut_ptr() as usize, 0x2c, f) == 1 {
            if !nativo {
                exe::swap_bytes_32_array(v.as_mut_ptr() as usize, 0xb);
            }
        } else if quiet == 0 {
            formation_error_box(0x97b);
        }
        for (k, x) in v.iter().enumerate() {
            write_u32(this + 0x188b + 8 * k, *x);
            write_u32(this + 0x188b + 8 * k + 4, 0xa); // cqg8: the default instruction of the old versions
        }
        crate::comparison::branch("cqg_8 old instructions");
    } else if crt_fread_one(this + 0x188b, 0x58, f) == 1 {
        if !nativo {
            exe::swap_bytes_64_array(this + 0x188b, 0xb);
        }
    } else if quiet == 0 {
        formation_error_box(0x988);
    }
    if crt_fread_one(this + 0x5b4, 0xb, f) != 1 && quiet == 0 {
        formation_error_box(0x98f);
    }
    let mut v = read_u32(team);
    if v & 0x4000 == 0 {
        v |= 0x2000; // cqg8: instruction 0x2000 without 0x4000
        write_u32(team, v);
        crate::comparison::branch("cqg_8 instruction 0x2000");
    }
    if id < 0x98ec5d {
        write_u32(team, read_u32(team) | 0x20000);
    }
    let mut c = 0u32;
    for vaga in 0..0xbi32 {
        exe::formation_get_slot_coordinate(this, vaga as i8, 0, 0, &mut c); // cqg8: the stored home side of stage 0
        crate::comparison::branch("cqg_8 positions redone");
        exe::formation_set_slot_coordinate(this, vaga, 0, 0, c); // cqg8: the positions redone by home side
    }
    exe::crt_strtok(path, 0, 0, (this) as *mut u8, (0) as *mut u8);
    exe::formation_build_position_lookup(this);
    write_u32(this + 0x187f, 1);
    write_u32(this + 0x1883, (*((this + 0x33) as *const u8) != 0) as u32);
    write_u32(this + 0x1887, 1);
    cash_crt_fclose(f);
    1
}

/// Checks the centre-backs for conflicts: 0 if any slot with an open side is a centre-back's or defensive midfielder's;
/// returns only `al`.
///
/// (thiscall) Like `slots_without_conflict`, for the centre-backs: returns 0 if any of the 11 slots with an open side (0x880) is a
/// centre-back's (4, in cell +0x12d or +0x14e) or a defensive midfielder's (8 in +0x12d; in +0x14e through `exe::formation_get_slot_position`);
/// otherwise 1. Returns only `al`.
/// Original: `centre_backs_without_conflict` (0x5b8dc0, formation.cpp, thiscall).
///
/// [pt] { 0x5b8dc0 } (thiscall, formation.cpp): como a 0x5b8960, para os zagueiros: devolve 0 se alguma
/// das 11 vagas com lado aberto (0x880) é de zagueiro (4, na casa +0x12d ou +0x14e) ou de volante
/// (8 na +0x12d; na +0x14e pela 0x5b2ae0); senão 1. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn centre_backs_without_conflict(this: usize) -> u8 {
    for i in 0..=10u32 {
        let a = read_u16(this + (0x12d + 2 * i as usize));
        let b = read_u16(this + (0x14e + 2 * i as usize));
        if a & 0x880 != 0 && a & 4 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 4 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 8 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 8 != 0 {
            return 0;
        }
    }
    1
}

/// Stores the general or extra position of the slot, the flags, and recomputes the pitch cells without and with the
/// ball (`pitch_cell`).
///
/// `Formation::set_slot_position(cell, position, extra)`: without `extra`, the cell's general position (+0x12d, words)
/// changes to `position` (if it changes: the mask `formation_mask`, the cell's flag +0x143 = 1 and that of the other cells with
/// the same position = 0); with or without it, the extra position (+0x14e) = `position`. Then, for each band 0..2 and
/// height 0..3 (the low and high word of `pitch_cell`'s argument), the pitch cells without and with the ball (+0x164 and
/// +0x194 + 4 × (height + 4 × band + 24 × cell)): with general = extra, both from the general one; otherwise, by
/// sectors (`exe::formation_split_position_code`): an extra in a sector below the general's gives the extra without the ball and the general with
/// the ball; otherwise the reverse. Finally +0x187f = 0. Returns 1. Cell outside 0..10: as in the original ("Error" box
/// and 0).
/// Original: `Formation::set_slot_position` (0x5b2ba0, formation.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5b2ba0 } (thiscall, ret 0xc, formation.cpp): `Formation::set_slot_position(casa, posicao, extra)`: sem `extra`,
/// a posição geral da casa (+0x12d, palavras) muda para `posicao` (se mudar: a máscara 0x5b59a0, a marca +0x143 da casa
/// = 1 e a das outras casas com a mesma posição = 0); com ou sem, a posição extra (+0x14e) = `posicao`. Depois, para
/// cada faixa 0..2 e altura 0..3 (a palavra baixa e a alta do argumento da 0x5b7e60), as casas no campo sem e com a
/// bola (+0x164 e +0x194 + 4 × (altura + 4 × faixa + 24 × casa)): com geral = extra, as duas da geral; senão, pelos
/// setores (0x5b73a0): extra de setor abaixo do da geral, sem a bola a extra e com a bola a geral; senão o contrário.
/// Por fim +0x187f = 0. Devolve 1. Casa fora de 0..10: a original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn set_slot_position(this: usize, home: u32, position: u32, extra: u32) -> u32 {
    let k = home as u8 as i8;
    if !(0..=10).contains(&k) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SET_SLOT_POSITION)(this, home, position, extra); // message box: original only
    }
    let s = k as usize;
    let p = position as u16 as i16;
    let general = this + 0x12d + 2 * s;
    let extra_of_home = this + 0x14e + 2 * s;
    if extra == 0 && read_i16(general) != p {
        core::ptr::write_unaligned(general as *mut i16, p);
        exe::formation_update_shape_code(this);
        *((this + 0x143 + s) as *mut u8) = 1;
        for j in 0..11usize {
            if j != s && read_i16(this + 0x12d + 2 * j) == p {
                *((this + 0x143 + j) as *mut u8) = 0; // the other square with the same position
            }
        }
    }
    core::ptr::write_unaligned(extra_of_home as *mut i16, p);
    // 0x5b7e60 and 0x5b73a0 only read the low word of the mask (the original pushes eax with whatever high part it has)
    let pitch_cell: unsafe extern "C" fn(u32, u32, u32, *mut u32) -> u32 = core::mem::transmute(FN_FORMATION_GET_DEFAULT_COORDINATE);
    // the outputs carry over from one iteration to the next, like the original’s locals (uninitialized only if 0x5b73a0 fails
    // right on the first one: zero here)
    let (mut setor_geral, mut setor_extra, mut lado, mut saida) = (0u16, 0u16, 0u16, 0u32);
    if read_i16(general) == read_i16(extra_of_home) {
        crate::comparison::branch("cpk_7 formation same position");
    } else {
        crate::comparison::branch("cpk_7 formation extra position");
    }
    for band in 0..3usize {
        for altura in 0..4usize {
            let arg = band as u32 | (altura as u32) << 16;
            let i = altura + 4 * (band + 6 * s);
            let (sem_bola, com_bola) = (this + 0x164 + 4 * i, this + 0x194 + 4 * i);
            let (g, e) = (read_i16(general) as u16 as u32, read_i16(extra_of_home) as u16 as u32);
            let (a, b) = if g == e {
                (g, g) // the same position with and without the ball
            } else {
                exe::formation_split_position_code(g, &mut setor_geral, &mut lado);
                exe::formation_split_position_code(e, &mut setor_extra, &mut lado);
                if (setor_extra as i16) < (setor_geral as i16) { // the rearmost extra
                    (e, g)
                } else {
                    (g, e)
                }
            };
            pitch_cell(a, 0, arg, &mut saida);
            core::ptr::write_unaligned(sem_bola as *mut u32, saida);
            pitch_cell(b, 1, arg, &mut saida);
            core::ptr::write_unaligned(com_bola as *mut u32, saida);
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// Formation: returns byte `i` of the per-position table at +0x5b4 [inference: the role flags of position `i`; bit 1
/// marks the goalkeeper].
/// Original: `position_byte` (0x5b57c0, formation.cpp).
///
/// [pt] { 0x5b57c0 }: byte `i` de uma tabela em +0x5b4 (por posicao; ver 0x6f4600).
pub(crate) unsafe extern "thiscall" fn position_byte(this: usize, i: i8) -> u8 {
    *((this as isize + 0x5b4 + i as isize) as *const u8)
}

