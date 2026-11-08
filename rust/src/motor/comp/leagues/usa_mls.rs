use super::*;



/// Row i of a league/stage table (+0xb1, 0x3b bytes): club and points (+0x19).
///
/// [pt] A linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`: o clube (+0) e os pontos (+0x19, word).
pub(crate) unsafe fn table_row_club_points(t: usize, i: usize) -> (usize, i16) {
    let l = read_usize(t + 0xb1) + i * 0x3b;
    (read_usize(l), read_i16(l + 0x19))
}

/// Writes the 6-byte (club, points) record k into a byte vector.
///
/// [pt] Grava o registro de 6 bytes `k` (clube, pontos) do vetor `r`.
pub(crate) fn put_club_points_record(r: &mut [u8], k: usize, (c, p): (usize, i16)) {
    r[6 * k..6 * k + 4].copy_from_slice(&(c as u32).to_le_bytes());
    r[6 * k + 4..6 * k + 6].copy_from_slice(&p.to_le_bytes());
}

/// Club of the 6-byte record k of a byte vector.
///
/// [pt] O clube do registro de 6 bytes `k` do vetor `r`.
pub(crate) fn club_of_record(r: &[u8], k: usize) -> usize {
    u32::from_le_bytes([r[6 * k], r[6 * k + 1], r[6 * k + 2], r[6 * k + 3]]) as usize
}

/// The MLS has three divisions of four (the league and stages 0 and 1, played together in the regular season), the
/// play-off (stage 2) and the final (stage 3): the next stage goes from stage 1 to the play-off and from the play-off
/// to the final, building each; past the last stage it is an error (the box)
/// Original: `UsaMls::advance_stage` (0x909400, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x909400 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::advance_stage()`, a fase seguinte da MLS (as fases 0 e 1 são
/// as duas outras divisões, que correm junto com a liga; a 2, o play-off; a 3, a final). Na última fase (+0x30 ≥
/// +0x2c − 1, com sinal): a caixa de erro da linha 0x55b e nada mais. Senão: da fase 1 vai à 2 e da 2 à 3 (de outra
/// fase, +0x30 fica); então, na 2, monta o play-off (0x9094c0) e, na 3, a final (0x9097d0), em call direto com o ecx da
/// liga.
pub(crate) unsafe extern "thiscall" fn usa_mls_advance_stage(this: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0xaa1fb4, 0x55b);
    }
    if fase == 1 {
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
    } else if fase == 2 {
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 3);
    }
    match read_usize(this + 0x30) {
        2 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLS_CREATE_PLAYOFF_STAGE);
            f(this);
        }
        3 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLS_CREATE_FINAL_STAGE);
            f(this);
        }
        _ => {}
    }
}

/// The MLS play-off: eight clubs. The three division winners, ordered by points, and the five best of the rest (second
/// to fourth of each division, by points; ties with no own rule). The pairs: the best winner against the fifth best of
/// the rest, the second winner against the fourth, the third winner against the third and the best of the rest against
/// the second [inference: pairs follow the list]; the winners go to the final
/// Original: `UsaMls::create_playoff_stage` (0x9094c0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x9094c0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_playoff_stage()`, o play-off
/// da MLS, da fase seguinte.
/// 1) Uma lista de 8 (malloc 0x944e46 de 0x20; sem memória: a caixa da linha 0x58d e volta).
/// 2) Os três campeões de divisão (o primeiro da tabela da liga, da fase 0 e da fase 1), em registros de 6 bytes (o
///    clube e os pontos, +0x19 da linha), ordenados pelos pontos (qsort 0x945333 com 0x90a130: mais pontos primeiro),
///    vão às vagas 1, 3 e 5 da lista.
/// 3) Os outros: o segundo, o terceiro e o quarto de cada tabela (na ordem: liga, fase 0, fase 1, linha a linha), nove
///    registros ordenados pelos pontos do mesmo jeito; os cinco melhores vão às vagas 7, 6, 4, 2 e 0 (o melhor na 7).
///    Os pares do play-off: o melhor campeão contra o quinto melhor dos outros, o segundo campeão contra o quarto, o
///    terceiro campeão contra o terceiro, e o melhor dos outros contra o segundo [inferência: os pares da lista].
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 2, &rodadas, &extra, 0), ret 0x10); a fase (operator
///    new 0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 8, a lista, as rodadas, +4, a
///    tabela, o ano +0x40, 2, 1, o extra, 0x12, 1, 0, 0 e sementes nulas; a cópia passa as words limpas) em [+0xc][2]
///    e a tabela liberada (free 0x9452ca). Sem fase: a caixa da linha 0x5c4. Com ela: Cup::set_up_round (0x51c800,
///    thiscall(fase, 0), ret 4). A lista não é liberada (a fase fica com ela; a original a deixa assim).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_playoff_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let (a, b) = (read_usize(read_usize(this + 0xc)), read_usize(read_usize(this + 0xc) + 4));
    let lista = alocar(0x20);
    if lista == 0 {
        return show_error_at(0xaa1fb4, 0x58d);
    }
    let poe = |k: usize, c: usize| core::ptr::write_unaligned((lista + 4 * k) as *mut usize, c);
    let mut r = [0u8; 54];
    put_club_points_record(&mut r, 0, table_row_club_points(this, 0));
    put_club_points_record(&mut r, 1, table_row_club_points(a, 0));
    put_club_points_record(&mut r, 2, table_row_club_points(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    poe(1, club_of_record(&r, 0)); // cqr4: o melhor campeão de divisão
    poe(3, club_of_record(&r, 1));
    poe(5, club_of_record(&r, 2));
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            put_club_points_record(&mut r, k, table_row_club_points(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    poe(7, club_of_record(&r, 0));
    poe(6, club_of_record(&r, 1));
    poe(4, club_of_record(&r, 2));
    poe(2, club_of_record(&r, 3));
    poe(0, club_of_record(&r, 4)); // cqr4: o quinto melhor dos outros
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 8, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 2, 1,
                  extra as u16 as u32, 0x12, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 8) as *mut usize, fase);
    soltar(t);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    if fase == 0 {
        return show_error_at(0xaa1fb4, 0x5c4);
    }
    crate::comparacao::ramo("cqr_4 play-off");
    montar(fase, 0);
}

/// The MLS Cup final: the two play-off winners (the first two with the winner mark, in play-off list order) in a
/// two-club cup stage [inference: a single game]; the winner is the MLS champion
/// Original: `UsaMls::create_final_stage` (0x9097d0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x9097d0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_final_stage()`, a final da MLS
/// Cup, da fase seguinte: os dois primeiros times do play-off ([+0xc][2], os registros de 6 bytes de +0xa7, na ordem)
/// com a marca de vencedor (+5 = 1) numa lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x5e5 e
/// volta); a tabela das rodadas pela virtual +0x3c (thiscall(this, 3, &rodadas, &extra, 0), ret 0x10); a fase
/// (operator new 0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00: a liga, 2, a lista, as rodadas, +4, a tabela, o ano,
/// 3, 1, o extra, 0x14, 0, 0, 0 e sementes nulas) em [+0xc][3] e a tabela liberada. Sem fase: a caixa da linha 0x5fe e
/// volta (sem liberar a lista). Com ela: Cup::set_up_round (thiscall(fase, 0)) e a lista liberada (free 0x9452ca).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let po = read_usize(read_usize(this + 0xc) + 8);
    let lista = alocar(8);
    if lista == 0 {
        return show_error_at(0xaa1fb4, 0x5e5);
    }
    let mut n = 0i8;
    let mut i = 0i32;
    while i < read_i16(po + 0x3e) as i32 && n < 2 {
        let e = read_usize(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            core::ptr::write_unaligned((lista + 4 * n as usize) as *mut usize, read_usize(e)); // cqr4: os finalistas
            n += 1;
        }
        i += 1;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 3, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 3, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 0xc) as *mut usize, fase);
    soltar(t);
    let fase = read_usize(read_usize(this + 0xc) + 0xc);
    if fase == 0 {
        return show_error_at(0xaa1fb4, 0x5fe);
    }
    crate::comparacao::ramo("cqr_4 final");
    montar(fase, 0);
    soltar(lista);
}

/// Comparer for `usa_mls_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    let esperado = if ultima { antes } else if antes == 1 || antes == 2 { antes + 1 } else { antes };
    PLACAR.registrar(depois == esperado, || format!("usa_mls_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// Comparer for `usa_mls_create_playoff_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela cópia da escolha (os mesmos registros e o mesmo qsort): a fase da terceira vaga existe, com +0x3e = 8, +4
/// = o +4 da liga e os oito na lista dela (+0xa7, 6 bytes; como conjunto).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_create_playoff_stage(this: usize) {
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let (a, b) = (read_usize(read_usize(this + 0xc)), read_usize(read_usize(this + 0xc) + 4));
    let mut plano = [0usize; 8];
    let mut r = [0u8; 54];
    put_club_points_record(&mut r, 0, table_row_club_points(this, 0));
    put_club_points_record(&mut r, 1, table_row_club_points(a, 0));
    put_club_points_record(&mut r, 2, table_row_club_points(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    for k in 0..3 {
        plano[k] = club_of_record(&r, k);
    }
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            put_club_points_record(&mut r, k, table_row_club_points(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    for k in 0..5 {
        plano[3 + k] = club_of_record(&r, k);
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_CREATE_PLAYOFF_STAGE)(this);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    let igual = fase != 0 && read_i16(fase + 0x3e) == 8 && read_usize(fase + 4) == read_usize(this + 4) && {
        let mut nela = [0usize; 8];
        for (i, c) in nela.iter_mut().enumerate() {
            *c = read_usize(read_usize(fase + 0xa7) + 6 * i);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    PLACAR.registrar(igual, || format!("usa_mls_create_playoff_stage({:#x}) fase={:#x} plano={:x?}", this, fase, plano));
}

/// Comparer for `usa_mls_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os dois primeiros vencedores do play-off. Depois, a fase da quarta vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (como conjunto).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_create_final_stage(this: usize) {
    let po = read_usize(read_usize(this + 0xc) + 8);
    let mut v = [0usize; 2];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < read_i16(po + 0x3e) as i32 && n < 2 {
        let e = read_usize(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_CREATE_FINAL_STAGE)(this);
    let fase = read_usize(read_usize(this + 0xc) + 0xc);
    // ponytail: com menos de dois vencedores a original lê vagas da lista sem valor; aí só a fase é conferida.
    let igual = fase != 0 && n < 2 || fase != 0 && {
        let l = read_usize(fase + 0xa7);
        let (x, y) = (read_usize(l), read_usize(l + 6));
        read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (v[0], v[1]) || (x, y) == (v[1], v[0]))
    };
    PLACAR.registrar(igual, || format!("usa_mls_create_final_stage({:#x}) finalistas={:x?} n={} fase={:#x}", this, v, n, fase));
}




/// True if club c is in this league's competition and its MLS division code matches.
///
/// [pt] O código de divisão do clube `c` na MLS (usa_mls_get_club_division 0x909ec0: push c; call; add esp, 4 -> cdecl; a
/// palavra baixa do eax: 0x413, 0x47b ou 0x414), se ele é da competição da liga (+0x57 do clube = +4 da liga).
pub(crate) unsafe fn is_in_mls_division(this: usize, c: usize, codigo: i16) -> bool {
    let divisao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_USA_MLS_GET_CLUB_DIVISION);
    read_usize(c + 0x57) == read_usize(this + 4) && divisao(c) as u16 as i16 == codigo
}

/// The MLS table is the first division one: the four clubs of the competition with division code 0x413, in database
/// order; succeeds if it found all four
/// Original: `UsaMls::create_table` (0x908560, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x908560 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::create_table()`, a tabela da MLS (a primeira divisão, código
/// 0x413): +0x3e = 4; a tabela (malloc 0x944e46 de 0xec, 4 linhas de 0x3b; sem memória: a caixa da linha 0x2fe e 0)
/// em +0xb1; na ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes, contador i16), enquanto faltam linhas, cada
/// clube da competição com o código 0x413 vai à linha seguinte (League::init_table_row 0x687430: push 0; push 0; push
/// clube; push eax com a linha no byte baixo e o lixo do código em cima, que ela não lê; mov ecx -> thiscall, ret
/// 0x10). Devolve 1 se encheu as 4 linhas.
pub(crate) unsafe extern "thiscall" fn usa_mls_create_table(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = alocar(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(0xaa1fb4, 0x2fe);
        return 0;
    }
    let mut n: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < read_i16(this + 0x3e) {
        let c = base + i as u16 as usize * club::SIZE;
        if is_in_mls_division(this, c, 0x413) {
            iniciar(this, n as u8 as u32, c, 0, 0); // cqs1: a linha seguinte da primeira divisão
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqs_1 tabela da mls");
    (n == read_i16(this + 0x3e)) as u32
}

/// Creates an MLS second/third division stage: club vector, round table, League_Stage, stored in slot k with the
/// division code.
///
/// [pt] A fase de uma das outras divisões da MLS (`create_second_division_stage` e `create_third_division_stage`; o mesmo
/// molde, conferido instrução a instrução: só mudam o código da divisão, a vaga k, o fonte das caixas e as linhas).
/// 1) Um vetor de +0x3e clubes (malloc 0x944e46 de 4 × +0x3e; sem memória: a caixa da linha `linhas[0]` e volta) com os
///    clubes da competição com o `codigo`, na ordem do banco (contador i32), enquanto faltam vagas.
/// 2) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, −3, &rodadas, &extra, 0), ret 0x10).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, +0x3c (word, os turnos), +4, a tabela das rodadas, as rodadas
///    (word), os pontos +0xc2 (`pv`) e +0xc3, a categoria +0xc4, &+0xc5, &+0xbe (as vagas da liga), o ano +0x40 (word),
///    k, o extra (word), +0x51, 1, +0xab (word), +0xd9 (word), +0xa7 (word), 0, 2; a cópia passa os valores limpos).
///    Sem memória: 0.
/// 4) [+0xc][k] = a fase; o vetor e a tabela das rodadas liberados (free 0x9452ca, nessa ordem). Sem fase: a caixa da
///    linha `linhas[1]`. Com ela: +0x30 = `atual` (a original grava k) e +0x3a da fase = o código da divisão.
pub(crate) unsafe fn create_division_stage(this: usize, codigo: i16, k: u32, atual: u32, pv: usize, linhas: [u32; 2], ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let v = alocar(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, linhas[0]);
    }
    let mut n: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && n < read_i16(this + 0x3e) {
        let c = base + i as usize * club::SIZE;
        if is_in_mls_division(this, c, codigo) {
            core::ptr::write_unaligned((v + 4 * n as usize) as *mut usize, c);
            n += 1;
        }
        i += 1;
    }
    let tabela: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, -3, &mut rodadas, &mut extra, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, read_i16(this + 0x3e) as u16 as u32, v, read_i16(this + 0x3c) as u16 as u32, read_usize(this + 4), t, rodadas as u16 as u32,
                  read_u8(this + pv) as u32, read_u8(this + 0xc3) as u32, read_u8(this + 0xc4) as u32, this + 0xc5, this + 0xbe,
                  read_i16(this + 0x40) as u16 as u32, k, extra as u16 as u32, read_u8(this + 0x51) as u32, 1, read_i16(this + 0xab) as u16 as u32,
                  read_i16(this + 0xd9) as u16 as u32, read_i16(this + 0xa7) as u16 as u32, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    soltar(v);
    soltar(t);
    let fase = read_usize(read_usize(this + 0xc) + 4 * k as usize);
    if fase == 0 {
        return show_error_at(0xaa1fb4, linhas[1]);
    }
    core::ptr::write_unaligned((this + 0x30) as *mut u32, atual);
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((fase + 0x3a) as *mut i16, codigo);
}

/// The second MLS division (code 0x47b): a round-robin stage with its four clubs (in database order), the same rounds,
/// points, tie-breakers and places as the league, as stage 0; the three division winners and the five best of the rest
/// go to the play-off
/// Original: `UsaMls::create_second_division_stage` (0x908fc0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x908fc0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_second_division_stage()`, a
/// segunda divisão da MLS (código 0x47b) na fase 0; caixas nas linhas 0x4e6 e 0x506.
pub(crate) unsafe extern "thiscall" fn create_second_division_stage(this: usize) {
    create_division_stage(this, 0x47b, 0, 0, 0xc2, [0x4e6, 0x506], "cqs_1 segunda divisao") // cqs1: os pontos por vitória na segunda
}

/// The third MLS division (code 0x414): the same template as the second, as stage 1; the current stage becomes 1, from
/// which the next stage goes to the play-off
/// Original: `UsaMls::create_third_division_stage` (0x9091e0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x9091e0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_third_division_stage()`, a
/// terceira divisão da MLS (código 0x414) na fase 1; caixas nas linhas 0x524 e 0x544.
pub(crate) unsafe extern "thiscall" fn create_third_division_stage(this: usize) {
    create_division_stage(this, 0x414, 1, 1, 0xc2, [0x524, 0x544], "cqs_1 terceira divisao") // cqs1: a fase atual depois da terceira
}

/// Comparer for `usa_mls_create_table` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da tabela: só a original (ponytail: aloca a tabela e as listas das linhas), com o plano feito antes:
/// os clubes da competição com o código 0x413, na ordem do banco, até 4. Depois, a devolução, +0x3e = 4 e as linhas.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_create_table(this: usize) -> u32 {
    let mut plano = [0usize; 4];
    let mut n = 0usize;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 4 {
        let c = base + i as u16 as usize * club::SIZE;
        if is_in_mls_division(this, c, 0x413) {
            plano[n] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_CREATE_TABLE)(this);
    let t = read_usize(this + 0xb1);
    let igual = r == (n == 4) as u32 && read_i16(this + 0x3e) == 4 && t != 0 && (0..n).all(|k| read_usize(t + k * 0x3b) == plano[k]);
    PLACAR.registrar(igual, || format!("usa_mls_create_table({:#x}) r={} n={} plano={:x?}", this, r, n, plano));
    r
}

/// Comparer for an MLS division stage: runs only the original and checks slot k, code, count and clubs (as a set)
/// against the plan.
///
/// [pt] O comparador de uma divisão: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes:
/// os clubes do código, na ordem do banco. Depois, a fase na vaga k com +0x3e da liga, os clubes na tabela dela (como
/// conjunto), +0x3a = o código e +0x30 = k.
pub(crate) unsafe fn compare_division_stage(nome: &str, this: usize, codigo: i16, k: usize, original: impl FnOnce()) {
    let mut plano = [0usize; 8];
    let mut n = 0usize;
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && (n as i16) < read_i16(this + 0x3e) && n < 8 {
        let c = base + i as usize * club::SIZE;
        if is_in_mls_division(this, c, codigo) {
            plano[n] = c;
            n += 1;
        }
        i += 1;
    }
    original();
    let fase = read_usize(read_usize(this + 0xc) + 4 * k);
    let igual = fase != 0 && read_i16(fase + 0x3a) == codigo && read_usize(this + 0x30) == k && read_i16(fase + 0x3e) == read_i16(this + 0x3e) && {
        let mut nela = [0usize; 8];
        for (j, c) in nela.iter_mut().enumerate().take(n) {
            *c = read_usize(read_usize(fase + 0xb1) + j * 0x3b);
        }
        let (mut a, mut b) = (plano, nela);
        a[..n].sort_unstable();
        b[..n].sort_unstable();
        a[..n] == b[..n]
    };
    PLACAR.registrar(igual, || format!("{}({:#x}) fase={:#x} n={} plano={:x?}", nome, this, fase, n, plano));
}

/// Comparer for `create_second_division_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_second_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_SECOND_DIVISION_STAGE);
    compare_division_stage("create_second_division_stage", this, 0x47b, 0, || o(this))
}

/// Comparer for `create_third_division_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_third_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_THIRD_DIVISION_STAGE);
    compare_division_stage("create_third_division_stage", this, 0x414, 1, || o(this))
}




/// 0x908e00 UsaMls::start_new_season: frees table, calendar and stages, archives stats, bumps the year, rebuilds rules,
/// table, fixtures, order, predictions, expectations and renews officials.
///
/// [pt] { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::start_new_season()`, a virada da MLS: +0x4c = 0 e a tabela
/// liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o calendário (+0xba) liberado (free 0x9452ca) e
/// zerado; com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1); +0x30 = −1;
/// com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); o ano (+0x40, word)
/// sobe `ano` (o contador "cqs_2 virada da mls"); então, cada um com a sua caixa (linhas 0x4af, 0x4b6, 0x4bc e 0x4c3)
/// se devolver 0: as regras (virtual +0x8c), a tabela da primeira divisão (0x908560), os jogos da temporada
/// (UsaMls::create_season_fixtures 0x9086e0, que monta também as fases das outras duas divisões) e a ordem da tabela
/// (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as posições previstas (virtual +0x5c), as
/// expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com
/// caixa, a cópia devolve 0 (a original devolve o eax da caixa; a virada é chamada pelo laço das viradas, que não o lê).
/// O vetor das fases (+0xc) fica: as divisões e o play-off regravam as vagas.
pub(crate) unsafe fn mls_start_new_season(this: usize, ano: i16) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    if (read_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > read_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let estatisticas = read_usize(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        arquivar(estatisticas, 1);
    }
    let a = read_i16(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo("cqs_2 virada da mls");
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(0xaa1fb4, 0x4af);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_USAMLS_CREATE_TABLE);
    if tabela(this) == 0 {
        show_error_at(0xaa1fb4, 0x4b6);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_USAMLS_CREATE_SEASON_FIXTURES);
    if jogos(this) == 0 {
        show_error_at(0xaa1fb4, 0x4bc);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if ordenar(this, 0) == 0 {
        show_error_at(0xaa1fb4, 0x4c3);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    previstas(this);
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4)) // cqs2: a renovação dos árbitros da MLS
}

/// The MLS turnover: clears the table, the schedule and the stages (the other two divisions, the play-off and the
/// final), archives the season in the statistics, moves to the next year and rebuilds the rules, the first division,
/// the season fixtures (with the other two division stages) and the order; then the predicted positions, the board
/// expectations and the referee renewal
/// Original: `UsaMls::start_new_season` (0x908e00, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): a virada da MLS (acima), um ano adiante.
pub(crate) unsafe extern "thiscall" fn usa_mls_start_new_season(this: usize) -> u32 {
    mls_start_new_season(this, 1) // cqs2: o ano seguinte da MLS
}

/// The MLS predicted positions: in each division the four clubs are ordered by reputation and get positions 1, 4, 7 and
/// 10 (each division favourite is predicted 1st, the second 4th, the third 7th and the last 10th), one scale shared by
/// the three divisions
/// Original: `UsaMls::set_predicted_positions` (0x909bb0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x909bb0 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::set_predicted_positions()`, as posições previstas da MLS. Sem
/// estatísticas (+8): a caixa da linha 0x675 e volta. Um vetor de 3 × +0x3e clubes (malloc 0x944e46; sem memória: a
/// caixa da linha 0x67d e volta): os da tabela da liga, os da fase 0 e os da fase 1 (as três divisões, linha a linha),
/// cada bloco de +0x3e ordenado pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no fim). Os doze
/// primeiros do vetor ganham, em cada divisão, as posições 1, 4, 7 e 10 (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a ordem das doze chamadas é a do vetor); o vetor liberado (free 0x9452ca).
/// [inferência] As três divisões se misturam na previsão: o favorito de cada uma é previsto em 1º, o segundo em 4º...
pub(crate) unsafe extern "thiscall" fn usa_mls_set_predicted_positions(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let n3 = read_i16(this + 0x3e).wrapping_mul(3);
    if read_usize(this + 8) == 0 {
        return show_error_at(0xaa1fb4, 0x675);
    }
    let v = alocar(((n3 as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, 0x67d);
    }
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let n = read_i16(this + 0x3e) as i32;
        let k = i as usize;
        core::ptr::write_unaligned((v + 4 * k) as *mut usize, aus_nsl_table_row_club(this, k));
        core::ptr::write_unaligned((v + 4 * (k + n as usize)) as *mut usize, aus_nsl_table_row_club(read_usize(read_usize(this + 0xc)), k));
        core::ptr::write_unaligned((v + 4 * (k + 2 * n as usize)) as *mut usize, aus_nsl_table_row_club(read_usize(read_usize(this + 0xc) + 4), k));
        i += 1;
    }
    let n = read_i16(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = read_i16(this + 0x3e) as i32;
    ordenar(v + 4 * n as usize, n as u32, 4, 0x4b07d0);
    let n = read_i16(this + 0x3e) as i32;
    ordenar(v + 8 * n as usize, n as u32, 4, 0x4b07d0);
    let lugares: [u32; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10]; // cqs2: as posições previstas das três divisões
    for (k, p) in lugares.iter().enumerate() {
        posicao(read_usize(this + 8), read_usize(v + 4 * k), *p);
    }
    crate::comparacao::ramo("cqs_2 posicoes previstas");
    soltar(v);
}

/// The list of the twelve MLS clubs: the four of the first division, then the four of the second and the four of the
/// third, in table order [inference: used by the MLS All-Star game]
/// Original: `UsaMls::get_club_array` (0x909a40, usa_mls.cpp, thiscall, ret 8).
///
/// [pt] { 0x909a40 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::get_club_array(&lista, &n)`, a lista dos clubes da MLS: *n = 12
/// (word); *lista = malloc 0x944e46 de 0x30 (sem memória: a caixa da linha 0x64c, *n = 0 e volta); os +0x3e clubes da
/// tabela da liga, depois os da fase 0 (a partir de +0x3e) e os da fase 1 (a partir de 2 × +0x3e), linha a linha.
pub(crate) unsafe extern "thiscall" fn usa_mls_get_club_array(this: usize, lista: *mut usize, n: *mut i16) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    *n = 0xc;
    let v = alocar(0x30);
    *lista = v;
    if v == 0 {
        show_error_at(0xaa1fb4, 0x64c);
        *n = 0;
        return;
    }
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        core::ptr::write_unaligned((*lista + 4 * i as usize) as *mut usize, aus_nsl_table_row_club(this, i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let k = read_i16(this + 0x3e) as i32 + i as i32;
        core::ptr::write_unaligned((*lista + 4 * k as usize) as *mut usize, aus_nsl_table_row_club(read_usize(read_usize(this + 0xc)), i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let k = 2 * read_i16(this + 0x3e) as i32 + i as i32;
        core::ptr::write_unaligned((*lista + 4 * k as usize) as *mut usize, aus_nsl_table_row_club(read_usize(read_usize(this + 0xc) + 4), i as usize));
        i += 1;
    }
    crate::comparacao::ramo("cqs_2 clubes da mls");
}

/// Comparer for `usa_mls_start_new_season` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1 e as três divisões montadas
/// (+0x3e = 4 e as vagas 0 e 1 das fases).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_start_new_season(this: usize) -> u32 {
    let antes = read_i16(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_START_NEW_SEASON)(this);
    let depois = read_i16(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && read_i16(this + 0x3e) == 4 && read_usize(read_usize(this + 0xc)) != 0 && read_usize(read_usize(this + 0xc) + 4) != 0;
    PLACAR.registrar(igual, || format!("usa_mls_start_new_season({:#x}) ano antes={} depois={} +0x3e={}", this, antes, depois, read_i16(this + 0x3e)));
    r
}

/// Comparer for `usa_mls_set_predicted_positions` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador das posições previstas: só a original (ponytail: a 0x4a2540 acrescenta o registro de um clube que não
/// tem; não pode rodar duas vezes), com o plano feito antes: os doze clubes das três tabelas, cada bloco ordenado pela
/// mesma qsort numa cópia; depois, a posição (+0x72 do registro achado por 0x4a4850) de cada um.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_SET_PREDICTED_POSITIONS);
    let n = read_i16(this + 0x3e) as usize;
    if read_usize(this + 8) == 0 || n != 4 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut v = [0usize; 12];
    for k in 0..4 {
        v[k] = aus_nsl_table_row_club(this, k);
        v[4 + k] = aus_nsl_table_row_club(read_usize(read_usize(this + 0xc)), k);
        v[8 + k] = aus_nsl_table_row_club(read_usize(read_usize(this + 0xc) + 4), k);
    }
    for b in 0..3 {
        ordenar(v[4 * b..].as_mut_ptr(), 4, 4, 0x4b07d0);
    }
    o(this);
    const LUGARES: [u8; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10];
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(read_usize(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) != LUGARES[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("usa_mls_set_predicted_positions({:#x}): a posição da vaga {:?} não bate ({:x?})", this, errado, v));
}

/// Comparer for `usa_mls_get_club_array` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da lista: só a original (aloca a lista, que fica com quem chamou), com o plano: n = 12 e os doze clubes
/// das três tabelas na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_get_club_array(this: usize, lista: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_USA_MLS_GET_CLUB_ARRAY)(this, lista, n);
    let m = read_i16(this + 0x3e) as usize;
    let igual = *n == 0xc && *lista != 0 && (0..m).all(|i| {
        read_usize(*lista + 4 * i) == aus_nsl_table_row_club(this, i)
            && read_usize(*lista + 4 * (m + i)) == aus_nsl_table_row_club(read_usize(read_usize(this + 0xc)), i)
            && read_usize(*lista + 4 * (2 * m + i)) == aus_nsl_table_row_club(read_usize(read_usize(this + 0xc) + 4), i)
    });
    PLACAR.registrar(igual, || format!("usa_mls_get_club_array({:#x}) n={}", this, *n));
}




/// Staff_Records::add_failed_to_qualify_record (0x86c1d0) for a club.
///
/// [pt] 0x86c1d0 Staff_Records::add_failed_to_qualify_record (thiscall em [0xdd7f00], ret 0x10; já em Rust).
pub(crate) unsafe fn usa_mls_add_failed_to_qualify_record(clube: usize, comp: usize, fase: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FAILED_TO_QUALIFY_RECORD);
    f(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, fase, peso);
}

/// comp_util_scale_by_round: weight 50..70 by the match round.
///
/// [pt] 0x4bf850 comp_util_scale_by_round (cdecl): o peso de 50 a 70 pela rodada do jogo, em 2 rodadas.
pub(crate) unsafe fn weight_by_round(jogo: usize) -> u32 {
    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_SCALE_BY_ROUND);
    escala(0x32, 0x46, read_i16(jogo + 0x34) as u16 as u32, 2)
}

/// The MLS stage-end outcome (0x908200): qualified, failed or knocked-out records by situation and phase; dry run
/// without effects when `fazer` is false.
///
/// [pt] A MLS no fim das fases (0x908200). `fazer` = falso: o ensaio do comparador (só a devolução).
pub(crate) unsafe fn mls_stage_outcome(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> u32 {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    if fa < 2 {
        // a temporada (-1) e as duas primeiras rodadas dos play-offs
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqk_9 mls classificado");
                bra_nat_first_add_qualified_record(clube, comp, 0xa0, 0x78, 0x28); // cqk9: o classificado da MLS
            }
            return 0;
        }
        if st != 0xfe {
            return 0;
        }
        if fazer {
            let f = if fa == -1 { 0x413 } else if fa != 0 { 0x414 } else { 0x47b };
            usa_mls_add_failed_to_qualify_record(clube, comp, f, 0x28);
        }
        return u32::MAX;
    }
    if fa == 2 {
        // a final
        if st == 1 {
            if fazer {
                bra_nat_first_add_qualified_record(clube, comp, 0x417, 0x96, 0x50);
            }
        } else if st == 0 {
            if fazer {
                // ponytail: a original empurra eax/ecx/edx inteiros (lixo acima das palavras: o do chamador, a metade
                // de cima do this); a 0x4bf850 e a 0x86bdd0 só leem as palavras: vão as palavras.
                let peso = weight_by_round(jogo);
                bra_nat_first_add_qualified_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
            }
        } else if st == 0xff || st == 2 {
            if fazer {
                let peso = weight_by_round(jogo);
                let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
                elim(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
            }
        }
        return 0;
    }
    if st == 1 && fazer {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
        f(read_u32_as_usize(G_STAFF_RECORDS), clube, comp);
    }
    0
}

/// MLS: in the season and the first two play-off rounds, those who go through get the qualification record (stage 0xa0,
/// round 0x78, weight 0x28) and those left out the failed-to-qualify one; in the final, the winner gets the final
/// record (weight 0x50) and the others the round one, weighted 50 to 70; then the champion gets the champion record
/// Original: `UsaMls::update_staff_records_for_status` (0x908200, usa_mls.cpp, thiscall).
///
/// [pt] { 0x908200 } (thiscall, ret 0x18, usa_mls.cpp): `UsaMls::update_staff_records_for_status` (slot 18): na temporada e
/// nas fases 0 e 1, quem avança (6) ganha o registro de classificação (fase 0xa0, rodada 0x78) e quem fica de fora (-2) o
/// de não classificado (fase 0x413 na temporada, 0x47b na fase 0, 0x414 na 1) e -1; na fase 2, o vencedor (1) ganha
/// o registro da final (0x417, 0x96, peso 0x50), quem passa (0) o da rodada do jogo e os eliminados (-1, 2) o de
/// eliminação, com peso de 50 a 70 pela rodada; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
pub(crate) unsafe extern "thiscall" fn usa_mls_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mls_stage_outcome(this, clube, situacao, fase, jogo, true)
}

/// Comparer for `usa_mls_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com a
/// devolução prevista pelo ensaio.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let n = mls_stage_outcome(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_USA_MLS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, situacao, fase, a4, jogo, a6);
    PLACAR.registrar(r == n, || format!("usa_mls_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}

/// MLS: each game goes to the overall table and, in the regular season, also to the two conference tables (East and
/// West [inference]), without counting the statistics twice
/// Original: `UsaMls::add_result` (0x9099d0, usa_mls.cpp, thiscall).
///
/// [pt] { 0x9099d0 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::add_result(jogo, estatisticas)` (slot 2): o jogo vai para a
/// tabela geral (`League::add_result` 0x685d30) com a marca +0x42 do jogo em -1 quando abaixo de 2; no estágio 1 (a
/// temporada regular), vai também para as tabelas das duas conferências (os estágios 0 e 1, pelo slot 2 deles, com a
/// marca 0 e 1 e sem o bloco de estatísticas +8, guardado e devolvido). Devolve 1.
pub(crate) unsafe extern "thiscall" fn usa_mls_add_result(this: usize, jogo: usize, estat: u32) -> u32 {
    if (*((jogo + 0x42) as *const i8)) < 2 {
        *((jogo + 0x42) as *mut u8) = 0xff;
    }
    let geral: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_ADD_RESULT);
    geral(this, jogo, estat);
    if read_u32_as_usize(this + 0x30) != 1 {
        return 1;
    }
    let guardado = read_u32_as_usize(this + 8);
    *((this + 8) as *mut u32) = 0;
    *((jogo + 0x42) as *mut u8) = 0;
    let estagio = |k: usize| -> (usize, unsafe extern "thiscall" fn(usize, usize, u32) -> u32) {
        let e = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4 * k);
        (e, core::mem::transmute(read_u32_as_usize(read_u32_as_usize(e) + 4)))
    };
    let (e0, f0) = estagio(0);
    f0(e0, jogo, estat);
    *((jogo + 0x42) as *mut u8) = 1;
    let (e1, f1) = estagio(1);
    crate::comparacao::ramo("cqk_9 mls segunda conferencia");
    f1(e1, jogo, estat); // cqk9: o jogo na tabela da segunda conferência
    *((this + 8) as *mut u32) = guardado as u32;
    1
}

/// Comparer for `usa_mls_add_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (ponytail: o resultado entra nas tabelas; não roda duas vezes), com o plano: a devolução
/// 1, a marca +0x42 do jogo (1 no estágio 1; senão -1 se estava abaixo de 2) e o bloco +8 devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_add_result(this: usize, jogo: usize, estat: u32) -> u32 {
    let marca = *((jogo + 0x42) as *const i8);
    let bloco = read_u32_as_usize(this + 8);
    let esperada = if read_u32_as_usize(this + 0x30) == 1 { 1 } else if marca < 2 { -1 } else { marca };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_USA_MLS_ADD_RESULT)(this, jogo, estat);
    let (m, b) = (*((jogo + 0x42) as *const i8), read_u32_as_usize(this + 8));
    PLACAR.registrar(r == 1 && m == esperada && b == bloco, || format!("usa_mls_add_result({:x}, {:x}) devolveu {} marca {} x {} bloco {:x} x {:x}", this, jogo, r, m, esperada, b, bloco));
    r
}

/// MLS All-Star game: East (0x413) for New England, MetroStars, Tampa Bay, D.C. United, Columbus and Miami; West
/// (0x414) for Colorado, Dallas, Kansas City, Los Angeles, San Jose and Chicago; clubs outside the MLS, none
/// Original: `usa_mls_get_club_all_star_conference` (0x90a000, usa_mls.cpp, cdecl).
///
/// [pt] { 0x90a000 } (cdecl, usa_mls.cpp): `usa_mls_get_club_all_star_conference(clube)`: a conferência do clube no Jogo das
/// Estrelas: 0x413 (o Leste) para New England, MetroStars, Tampa Bay, D.C. United, Columbus e Miami (as chaves 0x9d0390,
/// 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380), 0x414 (o Oeste) para Colorado, Dallas, Kansas City, Los Angeles,
/// San Jose e Chicago (0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c); 0 para clube cuja liga (+0x57) não
/// é a MLS (chave 0x9cf590); clube da MLS fora dos doze: a caixa "Error" (a original).
pub(crate) unsafe extern "C" fn usa_mls_get_club_all_star_conference(clube: usize) -> u16 {
    let nacao = read_u32_as_usize(clube + club::DIVISION);
    if nacao == 0 || read_u32_as_usize(nacao) != read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return 0;
    }
    let id = read_u32_as_usize(clube);
    const LESTE: [usize; 6] = [0x9d0390, 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380];
    const OESTE: [usize; 6] = [0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c];
    if LESTE.iter().any(|&k| read_u32_as_usize(k) == id) {
        crate::comparacao::ramo("cqk_9 conferencia 0x413");
        return 0x413; // cqk9: a primeira conferência
    }
    if OESTE.iter().any(|&k| read_u32_as_usize(k) == id) {
        return 0x414;
    }
    original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_ALL_STAR_CONFERENCE)(clube)
}

/// Comparer for `usa_mls_get_club_all_star_conference` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só lê; o caso de erro fica com a original sozinha).
pub(crate) unsafe extern "C" fn comparar_usa_mls_get_club_all_star_conference(clube: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_ALL_STAR_CONFERENCE)(clube);
    let nacao = read_u32_as_usize(clube + club::DIVISION);
    if nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && r == 0 {
        return r; // o caso de erro (a caixa já saiu na original)
    }
    let n = usa_mls_get_club_all_star_conference(clube);
    PLACAR.registrar(r == n, || format!("usa_mls_get_club_all_star_conference({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// MLS: 32 games per team (+0xa7; four round robins in +0x3c); win 3, draw 1; tie-break by goal difference, goals
/// scored and head-to-head; 1 top place in the overall table (rule word 0x48: 0x8 next stage, the play-offs; 0x40 fixed
/// round order); nobody goes up or down; the word +0xab = 0xc2 copied to each match. No schedule here: the games are
/// built by hand (0x9086e0)
/// Original: `UsaMls::set_up_rules` (0x908660, usa_mls.cpp, thiscall).
///
/// [pt] { 0x908660 } (thiscall, usa_mls.cpp): `UsaMls::set_up_rules`, as regras da MLS; devolve 1 (o eax que ela usa como
/// valor).
pub(crate) unsafe extern "thiscall" fn usa_mls_set_up_rules(this: usize) -> u32 {
    set_mls_rules(this);
    crate::comparacao::ramo("cqj_8 regras da mls");
    1
}

/// Comparer for `usa_mls_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado sobre o objeto da liga (0xee; só grava nele); a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_set_up_rules(this: usize) -> u32 {
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xee)], || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_SET_UP_RULES)(this), || usa_mls_set_up_rules(this));
    PLACAR.registrar(igual && r == n, || format!("usa_mls_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// MLS: in the regular season and play-off stages 0 and 1, the current position 1, 2, 3, 4 becomes 1, 3, 7, 10, the
/// lowest 1 to 3 becomes 1 and 4 becomes 10, the highest 1 to 3 becomes 7 and 4 becomes 10 (positions on the
/// whole-league scale [inference]); in stage 2, each code becomes 2 × code − 1; after that, as they come
/// Original: `UsaMls::set_club_progress` (0x909dd0, usa_mls.cpp, thiscall).
///
/// [pt] { 0x909dd0 } (thiscall, ret 0x14, usa_mls.cpp): `UsaMls::set_club_progress(clube, fase, a, b, c)`: nas fases 0 e 1
/// e na temporada (-1), a: 1 → 1, 2 → 3, 3 → 7, 4 → 10; b: 1..3 → 1, 4 → 10; c: 1..3 → 7, 4 → 10; na fase 2, cada
/// código vira 2 × código − 1; depois, os códigos como vêm. Devolve a entrada.
pub(crate) unsafe extern "thiscall" fn usa_mls_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 2 {
        let (b, c) = (mls_progress_code(b, 1), mls_progress_code(c, 7));
        let a = match a {
            1 => 1,
            2 => {
                crate::comparacao::ramo("cqk_8 mls segundo");
                3 // cqk8: o código 2 da MLS
            }
            3 => 7,
            4 => 10,
            _ => a,
        };
        write_progress_codes(e, a, b, c);
    } else if fa == 2 {
        write_progress_codes(e, a.wrapping_mul(2).wrapping_sub(1), b.wrapping_mul(2).wrapping_sub(1), c.wrapping_mul(2).wrapping_sub(1));
    } else {
        write_progress_codes(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_USA_MLS_SET_CLUB_PROGRESS)(this, clube, fase, a, b, c), || usa_mls_set_club_progress(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("usa_mls_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}
