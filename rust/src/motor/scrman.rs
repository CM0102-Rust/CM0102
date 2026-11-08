use super::*;

// ---------- 187a leva: as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título (match_screens.cpp, scrman.cpp, Setup.cpp) ----------

/// the screen manager has no next screen to go forward to
/// Original: `ScreenManager::is_forward_unavailable` (0x7febb0, scrman.cpp, thiscall).
///
/// [pt] { 0x7febb0 } (thiscall, scrman.cpp): o gerente de telas não tem para onde avançar? (o bloco de
/// parâmetros da tela atual sem a tela seguinte em +0x1f8). Sem bloco: a original (caixa de erro e saída).
pub(crate) unsafe extern "thiscall" fn is_forward_unavailable(gerente: usize) -> u32 {
    let vaga = *((gerente + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((gerente as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco == 0 {
        return orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IS_FORWARD_UNAVAILABLE)(gerente);
    }
    (*((bloco + 0x1f8) as *const usize) == 0) as u32
}


/// Copia o texto C de `origem` (com o zero) para o texto de trabalho 0xde1f64.
pub(crate) unsafe fn copiar_para_o_texto(origem: usize) {
    let mut i = 0;
    loop {
        let b = *((origem + i) as *const u8);
        *((0xde1f64 + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// O nome da competição do registro `c` (107 bytes) no texto 0xde1f64 do jeito do painel: "Friendlies"
/// e "Reserves" para as dos amistosos ([0x9cf668]) e reservas ([0x9cf684]) com mais de uma rodada
/// (+8 > +4 de `rodadas`); senão o nome (+0x38).
pub(crate) unsafe fn nome_da_competicao_no_painel(c: usize, rodadas: usize) {
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mais_de_uma = core::ptr::read_unaligned((rodadas + 8) as *const i32) > core::ptr::read_unaligned((rodadas + 4) as *const i32);
    let chave = *(c as *const usize);
    if chave == *(KEY_CLUB_COMP_FRIENDLY as *const usize) && mais_de_uma {
        texto(0xde1f64, 0xa1631c);
    } else if chave == *(KEY_CLUB_COMP_RESERVE as *const usize) && mais_de_uma {
        texto(0xde1f64, 0xa16310);
    } else {
        copiar_para_o_texto(c + 0x38);
    }
}


pub(crate) unsafe extern "thiscall" fn comparar_is_forward_unavailable(gerente: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IS_FORWARD_UNAVAILABLE)(gerente);
    let n = is_forward_unavailable(gerente);
    PLACAR.registrar(r == n, || format!("is_forward_unavailable original={} rust={}", r, n));
    r
}



