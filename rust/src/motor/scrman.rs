use super::*;

/// the screen manager has no next screen to go forward to
/// Original: `ScreenManager::is_forward_unavailable` (0x7febb0, scrman.cpp, thiscall).
///
/// [pt] { 0x7febb0 } (thiscall, scrman.cpp): o gerente de telas não tem para onde avançar? (o bloco de
/// parâmetros da tela atual sem a tela seguinte em +0x1f8). Sem bloco: a original (caixa de erro e saída).
pub(crate) unsafe extern "thiscall" fn is_forward_unavailable(manager: usize) -> u32 {
    let slot = *((manager + 0x382a) as *const i16) as isize;
    let block = core::ptr::read_unaligned((manager as isize + 0x7e2 + slot * 0x300) as *const usize);
    if block == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IS_FORWARD_UNAVAILABLE)(manager);
    }
    (*((block + 0x1f8) as *const usize) == 0) as u32
}

/// Copies C string origem (with terminator) to the work text 0xde1f64.
///
/// [pt] Copia o texto C de `origem` (com o zero) para o texto de trabalho 0xde1f64.
pub(crate) unsafe fn copy_to_work_text(source: usize) {
    let mut i = 0;
    loop {
        let b = read_u8(source + i);
        write_u8(0xde1f64 + i, b);
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Writes the competition name to 0xde1f64 for the panel: "Friendlies"/"Reserves" for those with more than one round,
/// else the name (+0x38).
///
/// [pt] O nome da competição do registro `c` (107 bytes) no texto 0xde1f64 do jeito do painel: "Friendlies"
/// e "Reserves" para as dos amistosos ([0x9cf668]) e reservas ([0x9cf684]) com mais de uma rodada
/// (+8 > +4 de `rodadas`); senão o nome (+0x38).
pub(crate) unsafe fn panel_competition_name(c: Competition, rounds: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let more_of_one = read_i32(rounds + 8) > read_i32(rounds + 4);
    let key = read_usize(c.addr());
    if key == read_usize(KEY_CLUB_COMP_FRIENDLY) && more_of_one {
        text(0xde1f64, 0xa1631c);
    } else if key == read_usize(KEY_CLUB_COMP_RESERVE) && more_of_one {
        text(0xde1f64, 0xa16310);
    } else {
        copy_to_work_text(c.addr() + competition::SHORT_NAME);
    }
}

