use super::*;

/// Row i of a league/stage table (+0xb1, 0x3b bytes): club and points (+0x19).
///
/// [pt] A linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`: o clube (+0) e os pontos (+0x19, word).
pub(crate) unsafe fn table_row_club_points(t: League, i: usize) -> (usize, i16) {
    let l = t.table() + i * 0x3b;
    (read_usize(l), read_i16(l + 0x19))
}

/// Writes the 6-read_i8(club + points) record k into a byte vector.
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
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xaa1fb4, 0x55b);
    }
    if stage == 1 {
        this.set_current_stage((2) as usize);
    } else if stage == 2 {
        this.set_current_stage((3) as usize);
    }
    match this.current_stage() {
        2 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLS_CREATE_PLAYOFF_STAGE);
            f(this.addr());
        }
        3 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLS_CREATE_FINAL_STAGE);
            f(this.addr());
        }
        _ => {}
    }
}

/// Creates the MLS play-off with eight clubs: the three division winners, ordered by points, and the five best of the
/// rest; the winners go to the final.
///
/// `UsaMls::create_playoff_stage()`, the MLS play-off, from the next stage; SEH frame only for the new.
///
/// 1) A list of 8 (malloc `exe::crt_malloc` of 0x20; out of memory: the box for line 0x58d and it returns).
///
/// 2) The three division winners (the first of the league table, of stage 0 and of stage 1), in 6-byte records (the
/// club and the points, +0x19 of the row), sorted by points (qsort `exe::crt_qsort` with 0x90a130: more points first), go to
/// slots 1, 3 and 5 of the list.
///
/// 3) The others: the second, third and fourth of each table (in the order: league, stage 0, stage 1, row by row), nine
/// records sorted by points in the same way (ties with no rule of their own); the five best go to slots 7, 6, 4, 2 and
/// 0 (the best in 7). The play-off pairs: the best winner against the fifth best of the rest, the second winner against
/// the fourth, the third winner against the third, and the best of the rest against the second [inference: the pairs
/// follow the list].
///
/// 4) The round table through virtual +0x3c (thiscall(this, 2, &rounds, &extra, 0), ret 0x10); the stage (operator new
/// `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `construct_cup_stage`, thiscall, ret 0x3c: the league, 8, the list, the rounds, +4,
/// the table, the year +0x40, 2, 1, the extra, 0x12, 1, 0, 0 and null seeds; the port passes clean words) into
/// [+0xc][2] and the table freed (free `exe::crt_free`). Without a stage: the box for line 0x5c4. With it: Cup::set_up_round
/// (`set_up_round`, thiscall(stage, 0), ret 4). The list is not freed (the stage keeps it; the original leaves it so).
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
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let (a, b) = (read_usize(this.stages()), read_usize(this.stages() + 4));
    let list = exe::crt_malloc(0x20);
    if list == 0 {
        return show_error_at(0xaa1fb4, 0x58d);
    }
    let poe = |k: usize, c: usize| core::ptr::write_unaligned((list + 4 * k) as *mut usize, c);
    let mut r = [0u8; 54];
    put_club_points_record(&mut r, 0, table_row_club_points(this, 0));
    put_club_points_record(&mut r, 1, table_row_club_points(League(a), 0));
    put_club_points_record(&mut r, 2, table_row_club_points(League(b), 0));
    exe::crt_qsort((r.as_mut_ptr()) as usize, 3, 6, 0x90a130);
    poe(1, club_of_record(&r, 0)); // cqr4: the best division champion
    poe(3, club_of_record(&r, 1));
    poe(5, club_of_record(&r, 2));
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this.addr(), a, b] {
            put_club_points_record(&mut r, k, table_row_club_points(League(t), j));
            k += 1;
        }
    }
    exe::crt_qsort((r.as_mut_ptr()) as usize, 9, 6, 0x90a130);
    poe(7, club_of_record(&r, 0));
    poe(6, club_of_record(&r, 1));
    poe(4, club_of_record(&r, 2));
    poe(2, club_of_record(&r, 3));
    poe(0, club_of_record(&r, 4)); // cqr4: the fifth best of the others
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 2, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 8, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 2, 1,
                  extra as u16 as u32, 0x12, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((this.stages() + 8) as *mut usize, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 8);
    if stage == 0 {
        return show_error_at(0xaa1fb4, 0x5c4);
    }
    crate::comparison::branch("cqr_4 play-off");
    exe::cup_set_up_round(stage, 0);
}

/// Creates the MLS Cup final: the two play-off winners in a two-club cup stage [inference: a single game]; the winner
/// is the MLS champion.
///
/// `UsaMls::create_final_stage()`, the MLS Cup final, from the next stage; SEH frame only for the new. The first two
/// play-off teams ([+0xc][2], the 6-byte records of +0xa7, in order) with the winner mark (+5 = 1) in a list of 2
/// (malloc `exe::crt_malloc` of 8; out of memory: the box for line 0x5e5 and it returns); the round table through virtual +0x3c
/// (thiscall(this, 3, &rounds, &extra, 0), ret 0x10); the stage (operator new `exe::crt_operator_new`(0xb2) and the Cup_Stage
/// constructor `construct_cup_stage`: the league, 2, the list, the rounds, +4, the table, the year, 3, 1, the extra, 0x14, 0, 0, 0
/// and null seeds) into [+0xc][3] and the table freed. Without a stage: the box for line 0x5fe and it returns (without
/// freeing the list). With it: Cup::set_up_round (thiscall(stage, 0)) and the list freed (free `exe::crt_free`).
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
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let po = Cup(read_usize(this.stages() + 8));
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xaa1fb4, 0x5e5);
    }
    let mut n = 0i8;
    let mut i = 0i32;
    while i < po.team_count() as i32 && n < 2 {
        let e = po.team_table() + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            core::ptr::write_unaligned((list + 4 * n as usize) as *mut usize, read_usize(e)); // cqr4: os finalistas
            n += 1;
        }
        i += 1;
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 3, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 3, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((this.stages() + 0xc) as *mut usize, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 0xc);
    if stage == 0 {
        return show_error_at(0xaa1fb4, 0x5fe);
    }
    crate::comparison::branch("cqr_4 final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(list);
}

/// True if club c is in this league's competition and its MLS division code matches.
///
/// [pt] O código de divisão do clube `c` na MLS (usa_mls_get_club_division 0x909ec0: push c; call; add esp, 4 -> cdecl; a
/// palavra baixa do eax: 0x413, 0x47b ou 0x414), se ele é da competição da liga (+0x57 do clube = +4 da liga).
pub(crate) unsafe fn is_in_mls_division(this: League, c: Club, code: i16) -> bool {
    c.division() == this.competition() && (exe::usa_mls_get_club_division(c) as u32) as u16 as i16 == code
}

/// Creates the MLS table (the first division, code 0x413): the four clubs of the competition with that division code,
/// in database order; succeeds if it found all four.
///
/// `UsaMls::create_table()`: +0x3e = 4; the table (malloc `exe::crt_malloc` of 0xec, 4 rows of 0x3b; out of memory: the box for
/// line 0x2fe and 0) at +0xb1; in database order ([0xae23b4], [0xae235c] clubs of 0x245 bytes, i16 counter), while rows
/// are missing, each club of the competition with code 0x413 goes to the next row (League::init_table_row `init_table_row`:
/// push 0; push 0; push club; push eax with the row in the low byte and the code's garbage above, which it does not
/// read; mov ecx -> thiscall, ret 0x10). Returns 1 if it filled the 4 rows.
/// Original: `UsaMls::create_table` (0x908560, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x908560 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::create_table()`, a tabela da MLS (a primeira divisão, código
/// 0x413): +0x3e = 4; a tabela (malloc 0x944e46 de 0xec, 4 linhas de 0x3b; sem memória: a caixa da linha 0x2fe e 0)
/// em +0xb1; na ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes, contador i16), enquanto faltam linhas, cada
/// clube da competição com o código 0x413 vai à linha seguinte (League::init_table_row 0x687430: push 0; push 0; push
/// clube; push eax com a linha no byte baixo e o lixo do código em cima, que ela não lê; mov ecx -> thiscall, ret
/// 0x10). Devolve 1 se encheu as 4 linhas.
pub(crate) unsafe extern "thiscall" fn usa_mls_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xaa1fb4, 0x2fe);
        return 0;
    }
    let mut n: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < this.club_count() {
        let c = base + i as u16 as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), 0x413) {
            exe::league_init_table_row(this, n as u8 as u32, c, 0, 0); // cqs1: the next row of the first division
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    crate::comparison::branch("cqs_1 mls fixtures");
    (n == this.club_count()) as u32
}

/// Creates the stage of one of the other MLS divisions (`create_second_division_stage` and
/// `create_third_division_stage`): club vector, round table, League_Stage, stored in slot k with the division code.
///
/// Same template in both, checked instruction by instruction: only the division code, slot k, the error boxes' source
/// file and the lines change.
///
/// 1) A vector of +0x3e clubs (malloc `exe::crt_malloc` of 4 × +0x3e; when out of memory: the error box for line `linhas[0]` and
/// return) with the clubs of the competition with `codigo`, in database order (i32 counter), while there are slots
/// left.
///
/// 2) The round table from virtual +0x3c (create_round_table: thiscall(this, −3, &rodadas, &extra, 0), ret 0x10).
///
/// 3) The stage: operator new `exe::crt_operator_new`(0xee) and the League_Stage constructor 0x68b100 (thiscall, ret 0x58, 22
/// arguments, in the original routine's order: the league, +0x3e (word), the vector, +0x3c (word, the rounds), +4, the
/// round table, the rounds (word), the points +0xc2 (`pv`) and +0xc3, the category +0xc4, &+0xc5, &+0xbe (the league's
/// places), the year +0x40 (word), k, the extra (word), +0x51, 1, +0xab (word), +0xd9 (word), +0xa7 (word), 0, 2; the
/// copy passes clean values). When out of memory: 0.
///
/// 4) [+0xc][k] = the stage; the vector and the round table freed (free `exe::crt_free`, in that order). Without a stage: the
/// error box for line `linhas[1]`. With one: +0x30 = `atual` (the original writes k) and the stage's +0x3a = the
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
pub(crate) unsafe fn create_division_stage(this: League, code: i16, k: u32, current: u32, pv: usize, rows: [u32; 2], ramo: &'static str) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let v = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, rows[0]);
    }
    let mut n: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && n < this.club_count() {
        let c = base + i as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), code) {
            core::ptr::write_unaligned((v + 4 * n as usize) as *mut usize, c);
            n += 1;
        }
        i += 1;
    }
    let table: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), -3, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), this.club_count() as u16 as u32, v, this.legs() as u16 as u32, this.competition().addr(), t, rodadas as u16 as u32,
                  read_u8(this.addr() + pv) as u32, this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT,
                  this.year() as u16 as u32, k, extra as u16 as u32, this.importance() as u32, 1, this.match_rules() as u16 as u32,
                  this.league_flags() as u16 as u32, this.games_per_club() as u16 as u32, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((this.stages() + 4 * k as usize) as *mut usize, stage);
    exe::crt_free(v);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 4 * k as usize);
    if stage == 0 {
        return show_error_at(0xaa1fb4, rows[1]);
    }
    this.set_current_stage((current) as usize);
    crate::comparison::branch(ramo);
    core::ptr::write_unaligned((stage + league::STAGE_ID) as *mut i16, code);
}

/// The second MLS division (code 0x47b): a round-robin stage with its four clubs (in database order), the same rounds,
/// points, tie-breakers and places as the league, as stage 0; the three division winners and the five best of the rest
/// go to the play-off
/// Original: `UsaMls::create_second_division_stage` (0x908fc0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x908fc0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_second_division_stage()`, a
/// segunda divisão da MLS (código 0x47b) na fase 0; caixas nas linhas 0x4e6 e 0x506.
pub(crate) unsafe extern "thiscall" fn create_second_division_stage(this: usize) {
    create_division_stage(League(this), 0x47b, 0, 0, 0xc2, [0x4e6, 0x506], "cqs_1 segunda divisao") // cqs1: the points per win in the second
}

/// The third MLS division (code 0x414): the same template as the second, as stage 1; the current stage becomes 1, from
/// which the next stage goes to the play-off
/// Original: `UsaMls::create_third_division_stage` (0x9091e0, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x9091e0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_third_division_stage()`, a
/// terceira divisão da MLS (código 0x414) na fase 1; caixas nas linhas 0x524 e 0x544.
pub(crate) unsafe extern "thiscall" fn create_third_division_stage(this: usize) {
    create_division_stage(League(this), 0x414, 1, 1, 0xc2, [0x524, 0x544], "cqs_1 terceira divisao") // cqs1: the current stage after the third
}

/// The comparer of an MLS division stage: runs only the original (it allocates the array, the round table and the
/// stage), checked against the plan made beforehand.
///
/// The plan: the clubs with the code, in database order. Afterwards, the stage in slot k with the league's +0x3e, the
/// clubs in its table (as a set), +0x3a = the code and +0x30 = k.
///
/// [pt] O comparador de uma divisão: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes:
/// os clubes do código, na ordem do banco. Depois, a fase na vaga k com +0x3e da liga, os clubes na tabela dela (como
/// conjunto), +0x3a = o código e +0x30 = k.
pub(crate) unsafe fn compare_division_stage(name: &str, this: League, code: i16, k: usize, original: impl FnOnce()) {
    let mut plan = [0usize; 8];
    let mut n = 0usize;
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && (n as i16) < this.club_count() && n < 8 {
        let c = base + i as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), code) {
            plan[n] = c;
            n += 1;
        }
        i += 1;
    }
    original();
    let stage = League(read_usize(this.stages() + 4 * k));
    let same = !stage.is_null() && stage.stage_id() == code && this.current_stage() == k && stage.club_count() == this.club_count() && {
        let mut in_it = [0usize; 8];
        for (j, c) in in_it.iter_mut().enumerate().take(n) {
            *c = read_usize(stage.table() + j * 0x3b);
        }
        let (mut a, mut b) = (plan, in_it);
        a[..n].sort_unstable();
        b[..n].sort_unstable();
        a[..n] == b[..n]
    };
    SCOREBOARD.record(same, || format!("{}({:#x}) fase={:#x} n={} plano={:x?}", name, this.addr(), stage.addr(), n, plan));
}

/// `UsaMls::start_new_season()` (`0x908e00`, thiscall, ret 0, `usa_mls.cpp`): the MLS season rollover.
///
/// `+0x4c` = 0 and the table freed (League::free_table `0x687970`, thiscall(this, 0), ret 4); the schedule (`+0xba`)
/// freed (free `0x9452ca`) and zeroed; with the current stage ≥ 0, the existing stages 0..=`+0x30` destroyed (virtual
/// 0, the destructor with 1); `+0x30` = −1; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`,
/// thiscall(`+8`, 1), ret 4); the year (`+0x40`, word) goes up by `ano` (the counter "cqs_2 mls rollover").
///
/// Then, each with its own box (lines `0x4af`, `0x4b6`, `0x4bc` and `0x4c3`) if it returns 0: the rules (virtual
/// `+0x8c`), the first division's table (`0x908560`), the season's fixtures (UsaMls::create_season_fixtures `0x9086e0`,
/// which also builds the stages of the other two divisions) and the table order (League::sort_table `0x6827d0`,
/// thiscall(this, 0), ret 4); finally the predicted positions (virtual `+0x5c`), the boards' expectations
/// (League::set_club_season_expectations `0x68aa80`, thiscall, ret 0) and the renewal of the referees
/// (OfficialsManager::renew_officials `0x79cee0`, thiscall([`0xb74340`], `+4`), ret 4), whose eax it returns.
///
/// After a box, the copy returns 0 (the original routine returns the box's eax; the rollover is called by the rollover
/// loop, which does not read it). The stage vector (`+0xc`) stays: the divisions and the play-off rewrite the slots.
///
/// [pt] { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::start_new_season()`, a virada da MLS: +0x4c = 0 e a tabela
/// liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o calendário (+0xba) liberado (free 0x9452ca) e
/// zerado; com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1); +0x30 = −1;
/// com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); o ano (+0x40, word)
/// sobe `ano` (o contador "cqs_2 mls rollover"); então, cada um com a sua caixa (linhas 0x4af, 0x4b6, 0x4bc e 0x4c3)
/// se devolver 0: as regras (virtual +0x8c), a tabela da primeira divisão (0x908560), os jogos da temporada
/// (UsaMls::create_season_fixtures 0x9086e0, que monta também as fases das outras duas divisões) e a ordem da tabela
/// (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as posições previstas (virtual +0x5c), as
/// expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com
/// caixa, a cópia devolve 0 (a original devolve o eax da caixa; a virada é chamada pelo laço das viradas, que não o lê).
/// O vetor das fases (+0xc) fica: as divisões e o play-off regravam as vagas.
pub(crate) unsafe fn mls_start_new_season(this: League, year: i16) -> u32 {
    core::ptr::write_unaligned((this.addr() + league::SEASON_CLOSED) as *mut u32, 0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        core::ptr::write_unaligned((this.addr() + league::ROUND_DATES) as *mut usize, 0);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this.addr() + league::CURRENT_STAGE) as *mut i32, -1);
    let stats = read_usize(this.addr() + league::COMP_STATS);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    core::ptr::write_unaligned((this.addr() + league::YEAR) as *mut i16, a.wrapping_add(year));
    crate::comparison::branch("cqs_2 mls rollover");
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x8c));
    if rules(this.addr()) == 0 {
        show_error_at(0xaa1fb4, 0x4af);
        return 0;
    }
    if exe::usamls_create_table(this) == 0 {
        show_error_at(0xaa1fb4, 0x4b6);
        return 0;
    }
    if exe::usamls_create_season_fixtures(this) == 0 {
        show_error_at(0xaa1fb4, 0x4bc);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0xaa1fb4, 0x4c3);
        return 0;
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x5c));
    predicted(this.addr());
    exe::league_set_club_season_expectations(this);
    (exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), read_usize(this.addr() + league::COMPETITION)) as u32) // cqs2: the renewal of the MLS referees
}

/// The MLS turnover: clears the table, the schedule and the stages (the other two divisions, the play-off and the
/// final), archives the season in the statistics, moves to the next year and rebuilds the rules, the first division,
/// the season fixtures (with the other two division stages) and the order; then the predicted positions, the board
/// expectations and the referee renewal
/// Original: `UsaMls::start_new_season` (0x908e00, usa_mls.cpp, thiscall, ret 0).
///
/// [pt] { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): a virada da MLS (acima), um ano adiante.
pub(crate) unsafe extern "thiscall" fn usa_mls_start_new_season(this: usize) -> u32 {
    mls_start_new_season(League(this), 1) // cqs2: the next year of the MLS
}

/// Sets the MLS predicted positions: in each division the four clubs are ordered by reputation and get positions 1, 4,
/// 7 and 10, one scale shared by the three divisions.
///
/// `UsaMls::set_predicted_positions()`. Without statistics (+8): the box for line 0x675 and it returns. A vector of 3 ×
/// +0x3e clubs (malloc `exe::crt_malloc`; out of memory: the box for line 0x67d and it returns): those of the league table,
/// those of stage 0 and those of stage 1 (the three divisions, row by row), each block of +0x3e sorted by reputation
/// (qsort `exe::crt_qsort` with 0x4b07d0, highest first, nulls at the end). The first twelve of the vector get, in each
/// division, positions 1, 4, 7 and 10 (Comp_Stats::set_club_final_position `set_club_final_position`, thiscall(+8, club, position), ret
/// 8; the order of the twelve calls is the vector's); the vector is freed (free `exe::crt_free`). [inference] The three
/// divisions are mixed in the prediction: each division's favourite is predicted 1st, the second 4th, the third 7th and
/// the last 10th.
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
    let this = League(this);
    let n3 = this.club_count().wrapping_mul(3);
    if this.comp_stats() == 0 {
        return show_error_at(0xaa1fb4, 0x675);
    }
    let v = exe::crt_malloc(((n3 as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, 0x67d);
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let n = this.club_count() as i32;
        let k = i as usize;
        core::ptr::write_unaligned((v + 4 * k) as *mut usize, aus_nsl_table_row_club(this, k));
        core::ptr::write_unaligned((v + 4 * (k + n as usize)) as *mut usize, aus_nsl_table_row_club(League(read_usize(this.stages())), k));
        core::ptr::write_unaligned((v + 4 * (k + 2 * n as usize)) as *mut usize, aus_nsl_table_row_club(League(read_usize(this.stages() + 4)), k));
        i += 1;
    }
    let n = this.club_count() as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    let n = this.club_count() as i32;
    exe::crt_qsort(v + 4 * n as usize, n as u32, 4, 0x4b07d0);
    let n = this.club_count() as i32;
    exe::crt_qsort(v + 8 * n as usize, n as u32, 4, 0x4b07d0);
    let places: [u32; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10]; // cqs2: the predicted positions of the three divisions
    for (k, p) in places.iter().enumerate() {
        exe::comp_stats_set_club_final_position(this.comp_stats(), read_usize(v + 4 * k), *p);
    }
    crate::comparison::branch("cqs_2 expected positions");
    exe::crt_free(v);
}

/// The list of the twelve MLS clubs: the four of the first division, then the four of the second and the four of the
/// third, in table order [inference: used by the MLS All-Star game]
/// Original: `UsaMls::get_club_array` (0x909a40, usa_mls.cpp, thiscall, ret 8).
///
/// [pt] { 0x909a40 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::get_club_array(&lista, &n)`, a lista dos clubes da MLS: *n = 12
/// (word); *lista = malloc 0x944e46 de 0x30 (sem memória: a caixa da linha 0x64c, *n = 0 e volta); os +0x3e clubes da
/// tabela da liga, depois os da fase 0 (a partir de +0x3e) e os da fase 1 (a partir de 2 × +0x3e), linha a linha.
pub(crate) unsafe extern "thiscall" fn usa_mls_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let this = League(this);
    *n = 0xc;
    let v = exe::crt_malloc(0x30);
    *list = v;
    if v == 0 {
        show_error_at(0xaa1fb4, 0x64c);
        *n = 0;
        return;
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        core::ptr::write_unaligned((*list + 4 * i as usize) as *mut usize, aus_nsl_table_row_club(this, i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let k = this.club_count() as i32 + i as i32;
        core::ptr::write_unaligned((*list + 4 * k as usize) as *mut usize, aus_nsl_table_row_club(League(read_usize(this.stages())), i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let k = 2 * this.club_count() as i32 + i as i32;
        core::ptr::write_unaligned((*list + 4 * k as usize) as *mut usize, aus_nsl_table_row_club(League(read_usize(this.stages() + 4)), i as usize));
        i += 1;
    }
    crate::comparison::branch("cqs_2 mls clubs");
}

/// Staff_Records::add_failed_to_qualify_record (`add_failed_to_qualify_record`) for a club.
///
/// [pt] 0x86c1d0 Staff_Records::add_failed_to_qualify_record (thiscall em [0xdd7f00], ret 0x10; já em Rust).
pub(crate) unsafe fn usa_mls_add_failed_to_qualify_record(club: usize, comp: usize, stage: u32, weight: u32) {
    exe::staff_records_add_failed_to_qualify_record(read_usize(G_STAFF_RECORDS), club, comp, stage, weight);
}

/// comp_util_scale_by_round: weight 50..70 by the match round.
///
/// [pt] 0x4bf850 comp_util_scale_by_round (cdecl): o peso de 50 a 70 pela rodada do jogo, em 2 rodadas.
pub(crate) unsafe fn weight_by_round(game: usize) -> u32 {
    (exe::comp_util_scale_by_round(0x32, 0x46, read_i16(game + 0x34) as u16 as u32, 2) as u32)
}

/// The MLS stage-end outcome (`usa_mls_update_staff_records_for_status`): qualified, failed or knocked-out records by situation and phase; dry run
/// without effects when `fazer` is false.
///
/// [pt] A MLS no fim das fases (0x908200). `fazer` = falso: o ensaio do comparador (só a devolução).
pub(crate) unsafe fn mls_stage_outcome(this: usize, club: usize, status: u32, stage: u32, game: usize, fazer: bool) -> u32 {
    let (st, fa) = (status as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if fa < 2 {
        // the season (-1) and the first two play-off rounds
        if st == 6 {
            if fazer {
                crate::comparison::branch("cqk_9 mls qualified");
                bra_nat_first_add_qualified_record(club, comp, 0xa0, 0x78, 0x28); // cqk9: the MLS qualifier
            }
            return 0;
        }
        if st != 0xfe {
            return 0;
        }
        if fazer {
            let f = if fa == -1 { 0x413 } else if fa != 0 { 0x414 } else { 0x47b };
            usa_mls_add_failed_to_qualify_record(club, comp, f, 0x28);
        }
        return u32::MAX;
    }
    if fa == 2 {
        // a final
        if st == 1 {
            if fazer {
                bra_nat_first_add_qualified_record(club, comp, 0x417, 0x96, 0x50);
            }
        } else if st == 0 {
            if fazer {
                // simplification: the original pushes whole eax/ecx/edx (garbage above the words: the caller’s, the half
                // from above this); 0x4bf850 and 0x86bdd0 only read the words: the words go.
                let weight = weight_by_round(game);
                bra_nat_first_add_qualified_record(club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
            }
        } else if st == 0xff || st == 2 {
            if fazer {
                let weight = weight_by_round(game);
                exe::staff_records_add_knocked_out_record(read_usize(G_STAFF_RECORDS), club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
            }
        }
        return 0;
    }
    if st == 1 && fazer {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
        f(read_usize(G_STAFF_RECORDS), club, comp);
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
pub(crate) unsafe extern "thiscall" fn usa_mls_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mls_stage_outcome(this, club, status, stage, game, true)
}

/// MLS result: each game goes to the overall table and, in the regular season, also to the two conference tables (East
/// and West [inference]), without counting the statistics twice.
///
/// `UsaMls::add_result(match, statistics)` (slot 2): the match goes to the overall table (`League::add_result`
/// `league_add_result`) with the match's mark +0x42 set to -1 when below 2; in stage 1 (the regular season), it also goes to the
/// tables of the two conferences (stages 0 and 1, through their slot 2, with mark 0 and 1 and without the statistics
/// block +8, which is saved and restored). Returns 1.
/// Original: `UsaMls::add_result` (0x9099d0, usa_mls.cpp, thiscall).
///
/// [pt] { 0x9099d0 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::add_result(jogo, estatisticas)` (slot 2): o jogo vai para a
/// tabela geral (`League::add_result` 0x685d30) com a marca +0x42 do jogo em -1 quando abaixo de 2; no estágio 1 (a
/// temporada regular), vai também para as tabelas das duas conferências (os estágios 0 e 1, pelo slot 2 deles, com a
/// marca 0 e 1 e sem o bloco de estatísticas +8, guardado e devolvido). Devolve 1.
pub(crate) unsafe extern "thiscall" fn usa_mls_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let this = League(this);
    if (*((game + 0x42) as *const i8)) < 2 {
        *((game + 0x42) as *mut u8) = 0xff;
    }
    exe::league_add_result(this, game, stats);
    if this.current_stage() != 1 {
        return 1;
    }
    let stored = this.comp_stats();
    this.set_comp_stats((0) as usize);
    *((game + 0x42) as *mut u8) = 0;
    let estagio = |k: usize| -> (usize, unsafe extern "thiscall" fn(usize, usize, u32) -> u32) {
        let e = read_usize(this.stages() + 4 * k);
        (e, core::mem::transmute(read_usize(read_usize(e) + 4)))
    };
    let (e0, f0) = estagio(0);
    f0(e0, game, stats);
    *((game + 0x42) as *mut u8) = 1;
    let (e1, f1) = estagio(1);
    crate::comparison::branch("cqk_9 mls second conference");
    f1(e1, game, stats); // cqk9: the match in the second conference table
    this.set_comp_stats((stored as u32) as usize);
    1
}

/// Returns the club's conference in the MLS All-Star game: East (0x413) or West (0x414); clubs outside the MLS, none.
///
/// `usa_mls_get_club_all_star_conference(club)` (cdecl): the club's conference in the All-Star game: 0x413 (East) for
/// New England, MetroStars, Tampa Bay, D.C. United, Columbus and Miami (the keys 0x9d0390, 0x9d0394, 0x9d0388,
/// 0x9d0384, 0x9d03a4, 0x9d0380), 0x414 (West) for Colorado, Dallas, Kansas City, Los Angeles, San Jose and Chicago
/// (0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c); 0 for a club whose league (+0x57) is not the MLS (key
/// 0x9cf590); an MLS club outside the twelve: the "Error" box (as in the original).
/// Original: `usa_mls_get_club_all_star_conference` (0x90a000, usa_mls.cpp, cdecl).
///
/// [pt] { 0x90a000 } (cdecl, usa_mls.cpp): `usa_mls_get_club_all_star_conference(clube)`: a conferência do clube no Jogo das
/// Estrelas: 0x413 (o Leste) para New England, MetroStars, Tampa Bay, D.C. United, Columbus e Miami (as chaves 0x9d0390,
/// 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380), 0x414 (o Oeste) para Colorado, Dallas, Kansas City, Los Angeles,
/// San Jose e Chicago (0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c); 0 para clube cuja liga (+0x57) não
/// é a MLS (chave 0x9cf590); clube da MLS fora dos doze: a caixa "Error" (a original).
pub(crate) unsafe extern "C" fn usa_mls_get_club_all_star_conference(club: usize) -> u16 {
    let club = Club(club);
    let nation = club.division().addr();
    if nation == 0 || read_usize(nation) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return 0;
    }
    let id = read_usize(club.addr());
    const LESTE: [usize; 6] = [0x9d0390, 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380];
    const OESTE: [usize; 6] = [0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c];
    if LESTE.iter().any(|&k| read_usize(k) == id) {
        crate::comparison::branch("cqk_9 conference 0x413");
        return 0x413; // cqk9: a primeira conferência
    }
    if OESTE.iter().any(|&k| read_usize(k) == id) {
        return 0x414;
    }
    original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_ALL_STAR_CONFERENCE)(club.addr())
}

/// MLS: 32 games per team (+0xa7; four round robins in +0x3c); win 3, draw 1; tie-break by goal difference, goals
/// scored and head-to-head; 1 top place in the overall table (rule word 0x48: 0x8 next stage, the play-offs; 0x40 fixed
/// round order); nobody goes up or down; the word +0xab = 0xc2 copied to each match. No schedule here: the games are
/// built by hand (`exe::usamls_create_season_fixtures`)
/// Original: `UsaMls::set_up_rules` (0x908660, usa_mls.cpp, thiscall).
///
/// [pt] { 0x908660 } (thiscall, usa_mls.cpp): `UsaMls::set_up_rules`, as regras da MLS; devolve 1 (o eax que ela usa como
/// valor).
pub(crate) unsafe extern "thiscall" fn usa_mls_set_up_rules(this: usize) -> u32 {
    set_mls_rules(this);
    crate::comparison::branch("cqj_8 mls rules");
    1
}

/// MLS: in the regular season and play-off stages 0 and 1, the current position 1, 2, 3, 4 becomes 1, 3, 7, 10, the
/// lowest 1 to 3 becomes 1 and 4 becomes 10, the highest 1 to 3 becomes 7 and 4 becomes 10 (positions on the
/// whole-league scale [inference]); in stage 2, each code becomes 2 × code − 1; after that, as they come
/// Original: `UsaMls::set_club_progress` (0x909dd0, usa_mls.cpp, thiscall).
///
/// [pt] { 0x909dd0 } (thiscall, ret 0x14, usa_mls.cpp): `UsaMls::set_club_progress(clube, fase, a, b, c)`: nas fases 0 e 1
/// e na temporada (-1), a: 1 → 1, 2 → 3, 3 → 7, 4 → 10; b: 1..3 → 1, 4 → 10; c: 1..3 → 7, 4 → 10; na fase 2, cada
/// código vira 2 × código − 1; depois, os códigos como vêm. Devolve a entrada.
pub(crate) unsafe extern "thiscall" fn usa_mls_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (fa, a, b, c) = (stage as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 2 {
        let (b, c) = (mls_progress_code(b, 1), mls_progress_code(c, 7));
        let a = match a {
            1 => 1,
            2 => {
                crate::comparison::branch("cqk_8 mls second");
                3 // cqk8: the MLS code 2
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

