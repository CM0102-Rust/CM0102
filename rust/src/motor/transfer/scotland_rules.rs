use super::*;

// ---------- 268a leva: o período do empréstimo na Inglaterra e as regras de empréstimo da Escócia, da Itália, da Irlanda e da Espanha (england_rules.cpp, scotland_rules.cpp, italy_rules.cpp, ireland_rules.cpp, spain_rules.cpp) ----------



pub(crate) unsafe fn cqe6_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de um contrato: +4 é o índice em [0xae23b4] (registros de 0x245 bytes).
pub(crate) unsafe fn cqe6_clube_do_contrato(c: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
}

/// O contrato principal da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em
/// [0xae1080] como a original faz.
pub(crate) unsafe fn cqe6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), p);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
    c
}

/// O começo comum das regras escocesa, irlandesa e espanhola: sem contrato do jogador (ou sem clube do contrato),
/// `None` (pode); senão o clube dono pelo contrato consultado de novo (0 se a segunda consulta não achar).
pub(crate) unsafe fn cqe6_dono(jogador: usize) -> Option<usize> {
    let c = cqe6_contrato(jogador);
    if c == 0 || cqe6_clube_do_contrato(c) == 0 {
        return None;
    }
    let c2 = cqe6_contrato(jogador);
    Some(if c2 != 0 { cqe6_clube_do_contrato(c2) } else { 0 })
}

/// A recusa "fora da janela" (motivo 9), com o adjetivo da nação das regras (+0xa).
pub(crate) unsafe fn cqe6_fora_da_janela(this: usize, msg: usize, motivo: *mut u16) -> u32 {
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nação; add esp, 0x10 -> cdecl
        let adjetivo: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_NATION_GET_NATIONALITY_ADJECTIVE);
        // 0x66f4e0: push ...; push formato; push destino; add esp, 0x14 -> cdecl variádica
        let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut buf = [0u8; 100];
        adjetivo(read_usize(this + 0xa), buf.as_mut_ptr(), 100, 2);
        texto(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !motivo.is_null() {
        *motivo = 9;
    }
    0
}

/// Uma recusa com o texto `formato` montado com o byte +0x37 (duas vezes) e o nome em +4 do registro `r`.
pub(crate) unsafe fn cqe6_recusa(msg: usize, motivo: *mut u16, formato: usize, r: usize, m: u16) -> u32 {
    if msg != 0 {
        let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let g = read_u8(r + 0x37) as i8 as i32;
        texto(msg, formato, g, g, r + 4);
    }
    if !motivo.is_null() {
        *motivo = m;
    }
    0
}


/// refuses (0x1d) between two clubs of the same Scottish division
/// Original: `SCOTLAND_RULES::can_loan_player` (0x7f6650, scotland_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7f6650 } (thiscall, ret 0x10, scotland_rules.cpp): `SCOTLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: com contrato, o clube que pede e o dono na mesma divisão escocesa ([0x9cf5dc]) não podem (0x1d).
pub(crate) unsafe extern "thiscall" fn cqe6_pode_emprestar_escocia(_this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    let comp = read_usize(clube + 0x57);
    if comp == read_usize(dono + 0x57) && comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
        crate::comparacao::ramo("cqe_6 escocia mesma divisao");
        let r = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION).wrapping_mul(0x6b));
        return cqe6_recusa(msg, motivo, 0x9c4590, r, 0x1d);
    }
    1
}

/// starts next month; under 22 it ends in the season end month (this year or next), otherwise three months later
/// Original: `SCOTLAND_RULES::get_loan_period` (0x7f6530, scotland_rules.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x7f6530 } (thiscall, ret 0x1c, scotland_rules.cpp): `SCOTLAND_RULES::get_loan_period(a1, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`: começa no mês seguinte; o fim é o mês do fim de temporada da nação a1
/// (0x4d45f0) para quem tem menos de 22 anos (+0x18), neste ano se o começo vem antes dele, senão no ano seguinte;
/// para os outros, três meses depois do começo.
pub(crate) unsafe extern "thiscall" fn cqe6_periodo_escocia(_this: usize, a1: usize, _a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    *ini_mes = (mes(0xae2c90) as i8).wrapping_add(1);
    *ini_ano = cqe6_h(0xae2c92);
    if *ini_mes > 0xb {
        *ini_mes = (*ini_mes).wrapping_sub(0xc);
        *ini_ano = (*ini_ano).wrapping_add(1);
    }
    // 0x4d45f0: push a1; mov ecx, [0xae1050] -> thiscall, ret 4 (o mês do fim da temporada)
    let mes_do_fim: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_SEASON_END_MONTH);
    let e = mes_do_fim(read_usize(G_CONTRACT_MANAGER), a1) as i8;
    *fim_mes = e;
    if (read_u8(jogador + 0x18) as i8) < 0x16 {
        crate::comparacao::ramo("cqe_6 escocia ate o fim da temporada");
        if *ini_mes < e {
            *fim_ano = *ini_ano;
        } else {
            *fim_ano = cqe6_h(0xae2c92).wrapping_add(1);
        }
        return;
    }
    *fim_mes = (*ini_mes).wrapping_add(3);
    *fim_ano = *ini_ano;
    if *fim_mes > 0xb {
        *fim_mes = (*fim_mes).wrapping_sub(0xc);
        *fim_ano = (*fim_ano).wrapping_add(1);
    }
}

/// refuses outside the first two windows (9) and, in the five listed divisions, with four players on loan from other
/// clubs (0x25)
/// Original: `ITALY_RULES::can_loan_player` (0x661770, italy_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x661770 } (thiscall, ret 0x10, italy_rules.cpp): `ITALY_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora das duas primeiras janelas (0x8e8c40 com [this+4] e [this+4] + 0xc): motivo 9. Um clube de uma das
/// cinco divisões ([0x9cf578] a [0x9cf588]) com quatro jogadores emprestados de outros clubes (ids abaixo de
/// [0xae2364], contrato de empréstimo 0x4cf6d0) não pode (0x25).
pub(crate) unsafe extern "thiscall" fn cqe6_pode_emprestar_italia(this: usize, clube: usize, _jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8c40: push janela; push 0xae2c90; ecx = this -> thiscall, ret 8
    let na_janela: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_WINDOW_CONTAINS_DATE);
    if na_janela(this, 0xae2c90, read_usize(this + 4)) == 0 && na_janela(this, 0xae2c90, read_usize(this + 4) + 0xc) == 0 {
        crate::comparacao::ramo("cqe_6 italia fora da janela");
        return cqe6_fora_da_janela(this, msg, motivo);
    }
    let comp = read_usize(clube + 0x57);
    if comp == 0 {
        return 1;
    }
    let id = read_usize(comp);
    if ![KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C].iter().any(|&g| read_usize(g) == id) {
        return 1;
    }
    // 0x4cf6d0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = read_usize(clube + 0xd7 + 4 * k);
        if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 || emprestimo(read_usize(G_CONTRACT_MANAGER), p) == 0 {
            continue;
        }
        let cp = cqe6_contrato(p);
        let de = if cp != 0 { cqe6_clube_do_contrato(cp) } else { 0 };
        if de != clube {
            n = n.wrapping_add(1);
        }
    }
    if n >= 4 {
        crate::comparacao::ramo("cqe_6 italia quatro emprestados");
        if msg != 0 {
            let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let g = read_u8(clube + 0x52) as i8 as i32;
            texto(msg, 0x9ea9dc, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x25;
        }
        return 0;
    }
    1
}

/// refuses with five loans this season (0x1e) and between clubs of the same division (0x1d)
/// Original: `IRELAND_RULES::can_loan_player` (0x63c3d0, ireland_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63c3d0 } (thiscall, ret 0x10, ireland_rules.cpp): `IRELAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: cinco empréstimos na temporada (a conta de 16 bytes em [[0xde1f54] + 0x91c]): 0x1e; com contrato, o
/// clube que pede e o dono na mesma divisão (+0x57, sem conferir o nulo, como a original): 0x1d, com o nome da nação.
pub(crate) unsafe extern "thiscall" fn cqe6_pode_emprestar_irlanda(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let conta = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x91c).wrapping_add(read_usize(clube).wrapping_mul(16));
    if read_u8(conta) as i8 >= 5 {
        crate::comparacao::ramo("cqe_6 irlanda cinco na temporada");
        if msg != 0 {
            let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let g = read_u8(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c4534, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x1e;
        }
        return 0;
    }
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if read_usize(clube + 0x57) == read_usize(dono + 0x57) {
        crate::comparacao::ramo("cqe_6 irlanda mesma divisao");
        return cqe6_recusa(msg, motivo, 0x9ea168, read_usize(this + 0xa), 0x1d);
    }
    1
}


/// O texto da mensagem (até o zero, no máximo 0x200 bytes).
pub(crate) unsafe fn cqe6_texto(msg: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if msg == 0 {
        return v;
    }
    for k in 0..0x200usize {
        let b = read_u8(msg + k);
        v.push(b);
        if b == 0 {
            break;
        }
    }
    v
}

/// Comparador comum (pode emprestar): lado a lado na devolução, em [0xae1080] e no motivo; a mensagem, que só é
/// escrita, é conferida pelo texto (antes da cópia o primeiro byte vai a zero, e no fim volta o texto da original).
pub(crate) unsafe fn cqe6_comparar_pode(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, nome: &str, this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || {
        let x = o(this, clube, jogador, msg, motivo);
        to = cqe6_texto(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, clube, jogador, msg, motivo);
        tr = cqe6_texto(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    PLACAR.registrar(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", nome, this, clube, jogador, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}


pub(crate) unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_escocia(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1590), cqe6_pode_emprestar_escocia, "cqe6_pode_emprestar_escocia", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado nas quatro saídas e em [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_cqe6_periodo_escocia(this: usize, a1: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(1591);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || cqe6_periodo_escocia(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("cqe6_periodo_escocia(regras {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, jogador, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_italia(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1592), cqe6_pode_emprestar_italia, "cqe6_pode_emprestar_italia", this, clube, jogador, msg, motivo)
}

pub(crate) unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_irlanda(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1593), cqe6_pode_emprestar_irlanda, "cqe6_pode_emprestar_irlanda", this, clube, jogador, msg, motivo)
}

/// the Scottish rule (21): one window, from 30 May (moved to the Saturday [inference]) to 30 March (moved to the
/// Thursday [inference]); dword +0x15 = 1
/// Original: `SCOTLAND_RULES::SCOTLAND_RULES` (0x7f6390, scotland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7f6390 } (thiscall, ret 0x14, scotland_rules.cpp, `SCOTLAND_RULES::SCOTLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Escócia (regra 21): a base (0x8e84a0), a vtable 0x96f838, 1 janela(s) de `CQK3_ESCOCIA` e o dword +0x15 = 1. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_escocia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_3 escocia");
    cqk3_construir(this, [a1, a2, a3, a4, a5], 0x96f838, 1, &CQK3_ESCOCIA, 0xa7b058, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_escocia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1684);
    let mut r = 0usize;
    cqk3_comparar("regras_escocia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_escocia(s, a1, a2, a3, a4, a5);
    });
    r
}
