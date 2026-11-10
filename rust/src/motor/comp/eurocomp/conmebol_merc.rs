use super::*;

/// Club k of the drawn list (+0xee pointers).
///
/// [pt] O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
pub(crate) unsafe fn conmebol_merc_drawn_club(this: League, k: usize) -> usize {
    read_usize(read_usize(this.addr() + 0xee) + k * 4) // cqr11: o sorteado k
}

/// `update_for_date(today, full)` (virtual +0x14): the Copa Mercosur's day. Rule: the Mercosur has 20 invited clubs
/// (the clubs the South American rules invite, shuffled) in 5 groups of 4 [inference: six rounds on Tuesdays, from 29
/// July to 14 October]; the season turns on 1 June; on the draw day group 1 stays in the cup's own table (drawn clubs 0
/// to 3) and groups 2 to 5 are stages (4g + 4 to 4g + 7); the 5 winners and the 3 best runners-up go to the
/// quarter-finals.
///
/// First the turn of the season: on 1 June of today's year (the date from `exe::date_date_549f70`(1, 5, year, −1): lea ecx, local;
/// push −1; push year; push 5; push 1 -> thiscall, ret 0x10, pure: the day of the year from the month tables, the month
/// counted from 0), if the cup's year (+0x40, signed) is not today's, virtual +8 (the new season: mov ecx, this ->
/// thiscall, ret 0).
///
/// When the draw has not been done (+0x45 = 0) and with `full`, on the draw day (the date from 0x549ef0(+0xb6, +0xb8 +
/// +0x40): the day of the year and the relative year; compared with today's, +0 the day of the year and +2 the year,
/// unsigned): the clubs (`conmebol_merc_select_invited_clubs`, the 20 invited clubs shuffled: mov ecx, this -> thiscall, ret 0), the group 1 table
/// (`conmebol_merc_create_first_group_table`, thiscall, ret 0; 0: the box of line 0x396), the fixtures (`create_fixtures` League::create_fixtures, thiscall, ret
/// 0; 0: the box of line 0x39b), the sorted table (`sort_table` push 0; mov ecx -> thiscall, ret 4; 0: the box of line
/// 0x3a1), the stages of groups 0..3 (`conmebol_merc_create_group_stage`: push g; mov ecx -> thiscall, ret 4), the predicted positions (virtual
/// +0x5c, thiscall, ret 0), the list +0xee freed (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed, and +0x45 = 1. An
/// error gives the box of its line and goes on. Then the league day (`league_update_for_date`: push full; push today; mov ecx, this ->
/// thiscall, ret 8), whose return value it passes on.
/// Original: `ConmebolMerc::update_for_date` (0x4c5c90, conmebol_merc.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c5c90 } (thiscall, ret 8, conmebol_merc.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Mercosul.
/// Primeiro a virada: no dia 1º de junho do ano de hoje (a data de 0x549f70(1, 5, ano, −1): lea ecx, local; push −1;
/// push ano; push 5; push 1 -> thiscall, ret 0x10, pura: o dia do ano pelas tabelas dos meses, o mês contado de 0), se o ano da copa (+0x40,
/// com sinal) não é o de hoje, a virtual +8 (a temporada nova: mov ecx, this -> thiscall, ret 0).
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c5850:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c4e60, thiscall, ret 0; 0: a caixa da linha 0x396),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x39b), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x3a1), as fases dos grupos 0..3 (0x4c51d0: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e segue. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
pub(crate) unsafe extern "thiscall" fn conmebol_merc_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let year_of_today = core::ptr::read_unaligned((today + 2) as *const u16);
    let mut junho = [0u8; 8];
    exe::date_date_549f70((junho.as_mut_ptr()) as *mut u8 as *mut [u16; 4], 1, 5, year_of_today as u32, (-1) as i32 as u32);
    if core::ptr::read_unaligned(junho.as_ptr().add(2) as *const u16) == year_of_today
        && core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(today as *const u16)
        && this.year() as i32 != year_of_today as i32
    {
        let turned: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 8));
        turned(this.addr());
    }
    if this.active() == 0 && full != 0 {
        let day = this.next_draw_day() as u16; // cqr11: the draw day
        let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
        if year == core::ptr::read_unaligned((today + 2) as *const u16) && day == core::ptr::read_unaligned(today as *const u16) {
            crate::comparison::branch("cqr_11 mercosul draw");
            exe::conmebolmerc_select_invited_clubs(this);
            if exe::conmebolmerc_create_first_group_table(this) == 0 {
                show_error_at(0x9a7e5c, 0x396);
            }
            if exe::league_create_fixtures(this) == 0 {
                show_error_at(0x9a7e5c, 0x39b);
            }
            if exe::league_sort_table(this, 0) == 0 {
                show_error_at(0x9a7e5c, 0x3a1);
            }
            for g in 0..4i32 {
                exe::conmebolmerc_create_group_stage(this, g);
            }
            let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x5c));
            predicted(this.addr());
            exe::crt_free(read_usize(this.addr() + 0xee));
            write_usize(this.addr() + 0xee, 0);
            this.set_active(1);
        }
    }
    (exe::league_update_for_date(this, today, full) as u32)
}

// simplification: the clubs (shuffled), the tables, the matches and the league day do not run twice. Original only; the plan
// on draw day (the copy’s condition): +0x45 = 1, the list +0xee zeroed, the first group’s table and the 4 stages
// created and the current stage (+0x30) the last of them.

/// Is today June 1 with the cup from another year (rollover day, draw not checked)?
///
/// [pt] O dia da virada (1º de junho do ano de hoje, com a copa de outro ano): o comparador não confere o sorteio nesse dia.
pub(crate) unsafe fn is_rollover_day(this: League, today: usize) -> bool {
    let year_of_today = core::ptr::read_unaligned((today + 2) as *const u16);
    let mut junho = [0u8; 8];
    exe::date_date_549f70((junho.as_mut_ptr()) as *mut u8 as *mut [u16; 4], 1, 5, year_of_today as u32, (-1) as i32 as u32);
    core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(today as *const u16) && this.year() as i32 != year_of_today as i32
}

/// Builds the table of group 1 of the Copa Mercosur. Rule: group 1 has 4 clubs, the first 4 drawn, each in the row of
/// its order; human managers get the group draw news.
///
/// 4 teams (+0x3e), the table +0xb1 (`exe::crt_malloc`(0xec): push 0xec; add esp, 4 -> cdecl; 4 rows of 0x3b bytes; without
/// memory, the box of line 0x155 and returns 0), each row i with drawn club i (+0xee) (`init_table_row` League::init_table_row:
/// push 0; push 0; push club; push i; mov ecx, this -> thiscall, ret 0x10), the group draw news (`send_group_draw_news`: mov ecx,
/// this -> thiscall, ret 0), and returns 1. The same body as `init_group_a_table` of the African Cup (only the file and the box
/// line change).
/// Original: `ConmebolMerc::create_first_group_table` (0x4c4e60, conmebol_merc.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c4e60 } (thiscall, ret 0, conmebol_merc.cpp): a tabela do primeiro grupo da Mercosul: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x155 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_first_group_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9a7e5c, 0x155);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        crate::comparison::branch("cqr_11 first group row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, conmebol_merc_drawn_club(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the drawn team i with position i (+4).

/// Creates the stage of group g + 1 of the Copa Mercosur (g from 0 to 3, on the draw day). Rule: each of the other
/// groups is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's
/// round table for the group; human managers get the news.
///
/// SEH frame only for the stage's new: no destructor on the normal path. The teams go into a vector of +0x3e pointers
/// (`exe::crt_malloc`(+0x3e << 2): push; add esp, 4 -> cdecl; without memory, the box of line 0x1fb and 0): drawn clubs 4g + 4
/// to 4g + 7 (+0xee); the round table through virtual +0x3c (push 0; push &id; push &rounds; push g; mov ecx, this ->
/// thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xee) push; add esp, 4 -> cdecl; the League_Stage constructor 0x68b100,
/// thiscall, ret 0x58, 22 arguments: the cup, the teams (+0x3e, word), the vector, +0x3c (word), +4, the round table,
/// the rounds (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, the year +0x40 (word), g, the id (word), +0x51, 2, 0, +0xd9
/// (word), −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes, which the constructor does
/// not read: the port passes clean values) in [+0xc][g]; the vector and the round table freed (`exe::crt_free`). Without a
/// stage: the box of line 0x211 and 0. With it: the group news (`send_group_draw_news` on the stage), +0x30 = g and 1.
/// Original: `ConmebolMerc::create_group_stage` (0x4c51d0, conmebol_merc.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c51d0 } (thiscall, ret 4, conmebol_merc.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 3, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x1fb e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x211 e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0x9a7e5c, 0x1fb);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparison::branch("cqr_11 group team");
        write_usize(teams + n * 4, conmebol_merc_drawn_club(this, k as usize)); // cqr11: the team in the group slot
        k += 1;
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), g, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this.addr(),
            this.club_count() as u16 as u32,
            teams,
            this.legs() as u16 as u32,
            this.competition().addr(),
            table,
            rodadas as u16 as u32,
            (this.win_points() as u32),
            (this.draw_points() as u32),
            (this.match_category() as u32),
            this.addr() + league::TIEBREAKS,
            this.addr() + league::PROMOTED_COUNT,
            this.year() as u16 as u32,
            g as u32,
            id as u16 as u32,
            (this.importance() as u32),
            2,
            0,
            this.league_flags() as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(this.stages() + g as usize * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(this.stages() + g as usize * 4);
    if f == 0 {
        show_error_at(0x9a7e5c, 0x211);
        return 0;
    }
    exe::league_send_group_draw_news(f);
    this.set_current_stage(g as usize);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][g], +0x30 = g, the stage with the teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// drawn 4g + 4 to 4g + 7 (in any order: the constructor sorts the table).

/// Writes one group round (a 0x41-byte record, like the league rounds).
///
/// The date goes through `league_round_set_date` (day, month with 0 = January, weekday, code), with no years to add and value 0; and
/// match 0 is left empty (`store_round_match` with −1, −1, −1 and value 0).
///
/// [pt] Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
pub(crate) type ConmebolMercGroupRound = (u32, u32, u32, u32);

/// Shared body of virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10; the stage read
/// as a signed byte) of the South American cups: group rounds below `limite`, knockout rounds at it, nothing above.
///
/// Below `limite` (−1 included), the groups: é_liga (if requested) = 1, rodadas = the number of group rounds, extra =
/// stage + 0x3e9 (16 bits); the table of 0x41 bytes per round (`exe::crt_malloc`; when out of memory, the error box `linha_g`
/// and returns 0) with the dates of `grupos`.
///
/// At stage `limite`, the knockout: é_liga = 0, rodadas = the size of `mata`, extra = 0; the table of 0x68 bytes per
/// round (when out of memory, the error box `linha_m`), the rounds written by `eng_cc_cup_write_cup_round` with the
/// cup's year +0x40 (signed) and the words of `a_mais` (round, offset, value).
///
/// Above `limite`: returns 0 without writing anything.
///
/// [pt] As duas fases da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10; a fase lida
/// como byte com sinal) das copas sul-americanas. Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1,
/// rodadas = o número de rodadas de grupo, extra = fase + 0x3e9 (16 bits); a tabela de 0x41 bytes por rodada (0x944e46;
/// sem memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0,
/// rodadas = o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as
/// rodadas pela `eng_cc_cup_write_cup_round` com o ano +0x40 da copa (com sinal) e as palavras de `a_mais` (rodada, deslocamento,
/// valor). Acima de `limite`: devolve 0 sem gravar nada.
pub(crate) unsafe fn south_american_cup_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, groups: &[ConmebolMercGroupRound],
    knockout: &[EngCcCupCupRound], more: &[(usize, usize, u16)], file: usize, row_g: u32, row_m: u32, name_g: &'static str, name: &'static str) -> usize {
    let f = stage as u8 as i8;
    if f < limit {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = groups.len() as i16;
        *extra = (f as i16).wrapping_add(0x3e9);
        let t = exe::crt_malloc(groups.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(file, row_g);
            return 0;
        }
        crate::comparison::branch(name_g);
        // the year goes with garbage in the high bytes in the original (the previous register with the word +0x40 over it); the
        // 0x68a160 only uses the low 16 bits: the copy passes the clean year
        let year = (this.year() as u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in groups.iter().enumerate() {
            exe::league_round_set_date(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, year, 0);
            exe::league_round_set_match_slot(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limit {
        return 0;
    }
    if !league.is_null() {
        *league = 0;
    }
    *rounds = knockout.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(knockout.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row_m);
        return 0;
    }
    crate::comparison::branch(name);
    let year = this.year() as i32 as u32;
    for (i, r) in knockout.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    for &(r, o, v) in more {
        core::ptr::write_unaligned((t + r * 0x68 + o) as *mut u16, v);
    }
    t
}

/// Side-by-side comparer for South American cup schedules.
///
/// The original runs with the caller's outputs, the copy with its own outputs that start with the same values, from the
/// same [0xb67a34]; the outputs and the bytes written in the two tables (groups or knockout, the latter including the
/// words of `a_mais`) must be equal; the copy's table is freed (`exe::crt_free`). Above the knockout stage, only the original
/// runs.
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (grupos ou mata-mata, este com as
/// palavras de `a_mais`) têm de ser iguais; a tabela da cópia é liberada (0x9452ca). Acima do mata-mata só a original.
pub(crate) unsafe fn compare_south_american_cup_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, more: &[(usize, usize, u16)],
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = stage as u8 as i8;
    if f > limit {
        // simplification: above the knockout neither of the two writes anything; the outputs may not even exist
        let a = o();
        SCOREBOARD.record(a == 0, || format!("{}(fase {}) devolveu {:#x}", name, f, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, &mut e2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let mais = |t: usize| more.iter().flat_map(|&(r, o, _)| core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2).to_vec()).collect::<Vec<u8>>();
    let table = if a != 0 && b != 0 {
        if f < limit {
            written_group_schedule_bytes(a, *rounds) == written_group_schedule_bytes(b, r2)
        } else {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) && mais(a) == mais(b)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", name, f, outs, *rounds, r2, table));
    a
}

/// A Libertadores, grupos: seis rodadas às terças [inferência: dia da semana 2], de 25 de fevereiro a 1º de abril.
pub(crate) const LIBERTADORES_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x19, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // cqo13: the Libertadores groups

/// The Copa Libertadores knockout: four two-legged rounds from April to June.
///
/// The second leg is 14 days after the first in the round of 16 and 7 in the other rounds.
///
/// [pt] A Libertadores, mata-mata: quatro rodadas em ida e volta, de abril a junho (o jogo de volta 14 dias depois nas
/// oitavas e 7 nas outras).
pub(crate) const LIBERTADORES_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 4] = [
    cql10r((2, 3, 0, 3), (0xf, 3, 0, 2, 2, 0), 0x14, (0, 4, 1), 7, (0x10, 8, 0x10, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo13: the Libertadores round of 16
    cql10r((0x1e, 3, 0, 3), (0x14, 4, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x1c, 4, 0, 3), (3, 5, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 5, 0, 3), (0x11, 5, 0, 2, 2, 0), 0x96, (0, 4, 1), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0xc3500)),
];

/// A Mercosul, grupos: seis rodadas às terças, de 29 de julho a 14 de outubro.
pub(crate) const MERCOSUR_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x1d, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // cqo13: the Mercosur groups

/// A Mercosul, mata-mata: quartas, semifinal e final em ida e volta, de outubro a dezembro (a volta 3 dias depois nas
/// quartas e na semifinal, 7 na final).
pub(crate) const MERCOSUR_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0xf, 9, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 8, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((5, 0xa, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x96, (0, 0x2000, 0x2000), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0)),
];

/// A final da Mercosul grava ainda a palavra +0x11 = 3 [inferência: o desempate da final, pênaltis].
pub(crate) const MERCOSUR_FINAL_EXTRA_WRITES: [(usize, usize, u16); 1] = [(2, 0x11, 3)];

/// rule: the Copa Mercosur starts with six-round groups on Tuesdays, from 29/7 to 14/10 (id 0x3e9 + stage, up to stage
/// 3: 5 groups [inference]); the knockout (stage 4) over two legs: quarter-finals drawn on 15/10 and played on 28/10 (8
/// teams, return 3 days later), semi-final 5/11 and 11/11 (return 3 days later), final draw 19/11 and match on 2/12
/// (return 7 days later, tie-break 0x2000 and the word +0x11 = 3 [inference: penalties]); category 8; prize of 2
/// million to the winner
/// Original: `ConmebolMerc::create_schedule` (0x4c4930, conmebol_merc.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c4930 } (thiscall, ret 0x10, conmebol_merc.cpp; a virtual +0x3c da Mercosul): `ConmebolMerc::create_schedule(
/// fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 4 (a caixa da linha 0x97), o mata-mata na fase 4 (a caixa
/// da linha 0xbd).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    south_american_cup_schedule(League(this), stage, rounds, extra, league, 4, &MERCOSUR_GROUP_ROUNDS, &MERCOSUR_KNOCKOUT_ROUNDS, &MERCOSUR_FINAL_EXTRA_WRITES, 0x9a7e5c, 0x97, 0xbd,
        "cqo_13 grupos da mercosul", "cqo_13 mata-mata da mercosul")
}

/// `ConmebolMerc::select_invited_clubs()` (`0x4c5850`, thiscall, ret 0, `conmebol_merc.cpp`): builds the shuffled
/// 20-slot list of clubs invited by the seeding rules.
///
/// Sets `+0xf2` = 20 and allocates the pointer list (`0x944e46` of 0x50 into `+0xee`; out of memory, the box of line
/// `0x2c6` and 0). Each club of the table ([`0xae23b4`], [`0xae235c`] − 2 × [`0xae2350`]) that the rules ([`0xae1000`],
/// `0x4c6890`: thiscall with the club) invite enters the list, without checking the space; the list is shuffled
/// (`0x4afb40` with `+0xf2`). Returns 1.
///
/// [pt] { 0x4c5850 } (thiscall, ret 0, conmebol_merc.cpp): `ConmebolMerc::select_invited_clubs()`: +0xf2 = 20 e a lista de
/// ponteiros (0x944e46 de 0x50 em +0xee; sem memória, a caixa da linha 0x2c6 e 0); cada clube da tabela ([0xae23b4],
/// [0xae235c] − 2 × [0xae2350]) que as regras ([0xae1000], 0x4c6890: thiscall com o clube) convidam entra na lista, sem
/// conferir o espaço; a lista é embaralhada (0x4afb40 com +0xf2). Devolve 1.
pub(crate) unsafe fn mercosur_select_invited_clubs(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x14);
    let l = exe::crt_malloc(0x50);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9a7e5c, 0x2c6);
        return (0, 0);
    }
    let clubs = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut k = 0i16;
    let mut i = 0i32;
    while i < clubs() {
        let c = read_usize(G_CLUB_TABLE).wrapping_add((i as isize * 0x245) as usize);
        if exe::conmebolseeding_is_mercosur_invited_club(read_usize(G_CONMEBOL_SEEDING), c) != 0 {
            core::ptr::write_unaligned((read_usize(this + 0xee) + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    crate::comparison::branch("cqo_13 mercosul guests");
    // simplification: the original pushes the word +0xf2 with garbage in the high 16 bits (the previous eax); 0x4afb40 reads the word
    exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), core::ptr::read_unaligned((this + 0xf2) as *const u16) as u32); // cqo13: embaralha os convidados
    (1, k as i32)
}

/// rule: the Copa Mercosur is invitation-only: the 20 clubs of the South American rules' fixed list (`is_mercosur_invited_club`), in club
/// table order, shuffled for the group draw
/// Original: `ConmebolMerc::select_invited_clubs` (0x4c5850, conmebol_merc.cpp, thiscall, ret 0).
///
/// [pt] regra: a Mercosul é só de convidados: os 20 clubes da lista fixa das regras sul-americanas (0x4c6890), na ordem da
/// tabela de clubes, embaralhados para o sorteio dos grupos
pub(crate) unsafe extern "thiscall" fn conmebol_merc_select_invited_clubs(this: usize) -> i32 {
    mercosur_select_invited_clubs(this).0
}

/// rule: the Mercosur moves on one stage when the current one ends; after the 4 group stages (on reaching 4) it builds
/// the quarter-finals; going past the last stage is an error
/// Original: `ConmebolMerc::advance_stage` (0x4c5140, conmebol_merc.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c5140 } (thiscall, ret 0, conmebol_merc.cpp; a virtual +0x28): `ConmebolMerc::advance_stage()`, a fase seguinte
/// da Mercosul: a caixa da linha 0x1d9 na última; chegando a 4 (depois dos 4 grupos-fase), as quartas (0x4c53d0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_advance_stage(this: usize) {
    con_merc_cup_advance_stage_cqs11_fase_seguinte(League(this), 0x9a7e5c, 0x1d9, 4, 0x4c53d0, "cqs_11 quartas da mercosul");
}

