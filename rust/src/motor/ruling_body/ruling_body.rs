use super::*;



/// Matches of ban by foul code (9..12) and ruling body scale (+6); None when no ban.
///
/// [pt] Os jogos de suspensão pelo código da falta (9 a 12) e pela escala do órgão (+6), na tabela de saltos 0x7e9028;
/// None quando não há suspensão (escala abaixo de 3 nos códigos 9 e 10, código fora de 9..=12).
pub(crate) fn ban_matches(codigo: i32, e: i8) -> Option<u8> {
    let x = e as i32;
    Some(match codigo {
        9 if e < 3 => return None,
        9 if e < 8 => 1,
        9 => 2 + (e >= 0xf) as u8,
        10 if e < 3 => return None,
        10 if e < 5 => 1,
        10 if e < 9 => 2,
        10 if e < 0xd => 3,
        10 => 4 + (e >= 0x11) as u8,
        11 => ((x * 8) / 20 + 4) as u8,
        12 => ((x * 28) / 20 + 12) as u8,
        _ => return None,
    })
}

/// Days of ban (same steps in weeks) by foul code and scale; None when no ban.
///
/// [pt] Os dias de suspensão (tabela 0x7e9038): os mesmos degraus em semanas.
pub(crate) fn ban_days(codigo: i32, e: i8) -> Option<u16> {
    let x = e as i32;
    Some(match codigo {
        9 if e < 3 => return None,
        9 if e < 8 => 7,
        9 => if e >= 0xf { 21 } else { 14 },
        10 if e < 3 => return None,
        10 if e < 5 => 7,
        10 if e < 9 => 14,
        10 if e < 0xd => 21,
        10 => if e >= 0x11 { 35 } else { 28 },
        11 => (((x * 8) / 20 + 4) * 7) as u16,
        12 => (((x * 28) / 20 + 12) * 7) as u16,
        _ => return None,
    })
}

/// builds the red card ban: the matches or days from the offence code (9 to 12) and the body scale, today's date and
/// the player; 0 when there is no ban
/// Original: `RulingBody::make_points_ban` (0x7e8e10, ruling_body.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7e8e10 } (thiscall, ret 0xc, ruling_body.cpp): `RulingBody::make_points_ban(registro, jogador, por_jogos)`, pelos
/// órgãos de cada país (rb_*.cpp, ao processar o cartão vermelho de uma partida): o código da falta (+0x2e com +0x2c =
/// 0, +0x2f com +0x2c = 1; outro tipo: 0) dá, pela escala do órgão (+6), os jogos (por_jogos ≠ 0; os dias ficam
/// 0xffff) ou os dias (os jogos ficam 0xff) da suspensão. No registro: +0 jogos, +1 = 0, +2 dias, +4 a data de hoje
/// ([0xae2c90], 0x417c10), +0x10 = 3, +0x1c o id do jogador (+0), +0x6f = (+0x48 do jogador == 0). Devolve 1; sem
/// suspensão, 0 sem gravar nada.
pub(crate) unsafe extern "thiscall" fn make_points_ban(this: usize, r: usize, j: usize, por_jogos: u32) -> u32 {
    let codigo = match *((j + 0x2c) as *const u8) {
        0 => *((j + 0x2e) as *const i8) as i32, // cpu10: a falta do primeiro tipo
        1 => *((j + 0x2f) as *const i8) as i32,
        _ => return 0,
    };
    let escala = *((this + 6) as *const i8);
    let (jogos, dias) = if por_jogos != 0 {
        match ban_matches(codigo, escala) {
            Some(n) => (n, 0xffffu16),
            None => return 0,
        }
    } else {
        match ban_days(codigo, escala) {
            Some(n) => (0xffu8, n),
            None => return 0,
        }
    };
    crate::comparacao::ramo("cpu_10 punicao");
    *(r as *mut u8) = jogos; // cpu10: os jogos da suspensão
    *((r + 1) as *mut u8) = 0;
    *((r + 2) as *mut u16) = dias;
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut hoje = [0u32; 2];
    copiar(r + 4, hoje.as_mut_ptr() as usize, G_CURRENT_DATE);
    *((r + 0x10) as *mut u8) = 3;
    *((r + 0x1c) as *mut u32) = read_u32_as_usize(j) as u32;
    *((r + 0x6f) as *mut u8) = (*((j + 0x48) as *const u8) == 0) as u8;
    1 // cpu10: a punição feita
}

/// Comparer for `make_points_ban` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 0x70 bytes do registro (a rotina só grava nele).
pub(crate) unsafe extern "thiscall" fn comparar_make_points_ban(this: usize, r: usize, j: usize, por_jogos: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_MAKE_POINTS_BAN);
    let regioes = [(r, 0x70)];
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, r, j, por_jogos), || make_points_ban(this, r, j, por_jogos));
    PLACAR.registrar(igual && a == b, || format!("make_points_ban(jogador {:x}, por_jogos {}) original={} rust={} {:x?} x {:x?}", read_u32_as_usize(j), por_jogos, a, b, mo, mr));
    a
}

/// at the new season, releases bans 2 (or 4) years old, frees the entries without a ban, clears the bookings of the
/// others and compacts the table
/// Original: `RulingBody::start_new_season` (0x7e7ed0, ruling_body.cpp, thiscall).
///
/// [pt] { 0x7e7ed0 } (thiscall, ret 0, ruling_body.cpp): `RulingBody::start_new_season()`: para cada entrada disciplinar
/// ocupada (14 bytes em [this+0xb], conta +0x13; id −1 é vaga livre): com suspensão (+6, índice com sinal na tabela de
/// 20 bytes da disciplina [[0xae28f0]+4]) de 2 anos ou mais (4 se o órgão não tem +0x17 nem +0x1b; o ano da suspensão
/// em +6 contra o de hoje [0xae2c92]), solta a suspensão (0x556d00) e libera a entrada; sem suspensão, libera a
/// entrada; senão zera os cartões (+4, +5, +8 e os 6 bits baixos de +0xd). Liberar: a vaga guardada do jogador no
/// órgão (0x55cb60 pela vaga +4) com o mesmo +5 fica com −1 em +6; sem ela, grava a vaga (0x55cb20, tipo +5, −1); o id
/// da entrada vira −1. No fim compacta as entradas (0x7e9050, nesta leva).
pub(crate) unsafe extern "thiscall" fn ruling_body_start_new_season(this: usize) {
    let guardada: unsafe extern "thiscall" fn(usize, i32, i8) -> usize = core::mem::transmute(FN_DISCIPLINE_FIND_HELD_BAN);
    let gravar: unsafe extern "thiscall" fn(usize, i32, i8, u8, u32) = core::mem::transmute(FN_DISCIPLINE_SET_SLOT);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DISCIPLINE_RELEASE_BAN);
    crate::comparacao::ramo("cpu_10 nova temporada");
    let mut e = read_u32_as_usize(this + 0xb);
    let mut i = 0i32;
    while i < read_i32(this + 0x13) {
        let id = read_i32(e);
        if id != -1 {
            let s = *((e + 6) as *const i16);
            let liberar = if s == -1 {
                true
            } else {
                let ban = read_u32_as_usize(read_u32_as_usize(G_DISCIPLINE) + 4).wrapping_add((s as i32 as usize).wrapping_mul(20));
                let anos = if read_i32(this + 0x17) == -1 && read_i32(this + 0x1b) == -1 { 4 } else { 2 };
                let ano = *((ban + 6) as *const u16) as i32;
                if *(G_CURRENT_DATE_YEAR as *const u16) as i32 - ano >= anos {
                    crate::comparacao::ramo("cpu_10 suspensao velha");
                    // ponytail: a original empurra o índice com os 16 bits altos do edx de antes; a 0x556d00 lê só a palavra.
                    soltar(read_u32_as_usize(G_DISCIPLINE), id as u32, s as u16 as u32);
                    true
                } else {
                    false
                }
            };
            if liberar {
                let id = read_i32(e);
                let g = guardada(read_u32_as_usize(G_DISCIPLINE), id, *((this + 4) as *const i8));
                if g != 0 && *((g + 5) as *const u8) == *((this + 5) as *const u8) {
                    *((g + 6) as *mut u32) = 0xffff_ffff;
                } else {
                    gravar(read_u32_as_usize(G_DISCIPLINE), read_i32(e), *((this + 4) as *const i8), *((this + 5) as *const u8), 0xffff_ffff);
                }
                *(e as *mut u32) = 0xffff_ffff;
            } else {
                let f = *((e + 0xd) as *const u8);
                *((e + 4) as *mut u8) = 0;
                *((e + 5) as *mut u8) = 0;
                *((e + 8) as *mut u16) = 0;
                *((e + 0xd) as *mut u8) = f & 0xc0;
            }
        }
        i += 1;
        e += 0xe;
    }
    let compactar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_RULINGBODY_COMPACT_ENTRIES);
    compactar(this);
}

/// Comparador: a original solta suspensões e realoca (não pode rodar duas vezes): só ela roda.
// ponytail: a cópia leva o contador "cpu_10 nova temporada" para provar no modo Rust; a compactação (comparada) confere o fim.
/// Comparer for `ruling_body_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ruling_body_start_new_season(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RULING_BODY_START_NEW_SEASON)(this);
}

/// removes the free entries from the body's discipline table, fixes the stored index of each moved player and
/// reallocates with 100 spare rows
/// Original: `RulingBody::compact_entries` (0x7e9050, ruling_body.cpp, thiscall).
///
/// [pt] { 0x7e9050 } (thiscall, ret 0, ruling_body.cpp): `RulingBody::compact_entries()`: tira as entradas livres (id −1)
/// da tabela de 14 bytes [this+0xb], trazendo as ocupadas para a frente; cada uma que muda de lugar acerta o índice
/// guardado do jogador (0x55cb60 pela vaga +4: com o mesmo +5, o novo índice em +6; sem ela, 0x55cb20 com o tipo +5 e
/// o índice). Conta +0x13 = n, capacidade +0xf = n + 100 e realoca (0x945667) com (n + 100) × 14 bytes; sem memória, a
/// caixa (linha 0x594) e segue.
pub(crate) unsafe extern "thiscall" fn compact_entries(this: usize) {
    let guardada: unsafe extern "thiscall" fn(usize, i32, i8) -> usize = core::mem::transmute(FN_DISCIPLINE_FIND_HELD_BAN);
    let gravar: unsafe extern "thiscall" fn(usize, i32, i8, u8, u32) = core::mem::transmute(FN_DISCIPLINE_SET_SLOT);
    let mut destino = read_u32_as_usize(this + 0xb);
    let mut origem = destino;
    let mut n = 0u32;
    let mut i = 0i32;
    while i < read_i32(this + 0x13) {
        if read_i32(origem) != -1 {
            if origem != destino {
                core::ptr::copy(origem as *const u8, destino as *mut u8, 0xe);
                let g = guardada(read_u32_as_usize(G_DISCIPLINE), read_i32(origem), *((this + 4) as *const i8));
                if g != 0 && *((g + 5) as *const u8) == *((this + 5) as *const u8) {
                    *((g + 6) as *mut u32) = n;
                } else {
                    gravar(read_u32_as_usize(G_DISCIPLINE), read_i32(origem), *((this + 4) as *const i8), *((this + 5) as *const u8), n);
                }
            }
            destino += 0xe;
            n += 1;
        }
        i += 1;
        origem += 0xe;
    }
    *((this + 0x13) as *mut u32) = n;
    *((this + 0xf) as *mut u32) = n + 100;
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let p = realocar(read_u32_as_usize(this + 0xb), (n + 100) * 14);
    *((this + 0xb) as *mut u32) = p as u32;
    if p == 0 {
        show_error_at(0xa7a858, 0x594);
    }
}

/// Comparer for `compact_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original realoca (não pode rodar duas vezes): as entradas ocupadas copiadas antes; roda só a
/// original e confere a conta, a capacidade e as entradas na nova ordem.
pub(crate) unsafe extern "thiscall" fn comparar_compact_entries(this: usize) {
    let mut esperado: Vec<u8> = Vec::new();
    let base = read_u32_as_usize(this + 0xb);
    for k in 0..read_i32(this + 0x13).max(0) as usize {
        let e = base + 0xe * k;
        if read_i32(e) != -1 {
            esperado.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xe));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_COMPACT_ENTRIES)(this);
    let n = (esperado.len() / 0xe) as u32;
    let p = read_u32_as_usize(this + 0xb);
    let iguais = p != 0 && core::slice::from_raw_parts(p as *const u8, esperado.len()) == &esperado[..];
    PLACAR.registrar(read_u32_as_usize(this + 0x13) as u32 == n && read_u32_as_usize(this + 0xf) as u32 == n + 100 && iguais, || {
        format!("compact_entries: conta {} (esperava {}), capacidade {}, entradas iguais {}", read_u32_as_usize(this + 0x13), n, read_u32_as_usize(this + 0xf), iguais)
    });
}

