use super::*;




/// Club k of the drawn list (+0xee pointers).
///
/// [pt] O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
pub(crate) unsafe fn conmebol_liber_drawn_club(this: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) // cqr10: o sorteado k
}

/// rule: the Libertadores has 32 clubs (those the South American rules put in it, shuffled) in 8 groups of 4, home and
/// away (3 points a win, 1 a draw; two qualify from each group, to the round of 16); the draw is on the start day (15
/// January of the cup year): group 1 stays in the cup's own table (drawn clubs 0 to 3, in order) and groups 2 to 8 are
/// stages (drawn clubs 4g + 4 to 4g + 7); on the draw day: the clubs (0x4c3f40 (the 32 clubs with the Libertadores as
/// their season competition, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages
/// and the predicted positions; then the league day
/// Original: `ConmebolLiber::update_for_date` (0x4c4410, conmebol_liber.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c4410 } (thiscall, ret 8, conmebol_liber.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Libertadores.
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c3f40:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c37a0, thiscall, ret 0; 0: a caixa da linha 0x365),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x36a), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x370), as fases dos grupos 0..6 (0x4c3b10: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e segue. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
pub(crate) unsafe extern "thiscall" fn conmebol_liber_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let clubs: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONMEBOLLIBER_SELECT_QUALIFIED_CLUBS);
    let first: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONMEBOLLIBER_BUILD_FIRST_GROUP_TABLE);
    let games: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let stage: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CONMEBOLLIBER_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let league: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if read_u32_as_usize(this + league::ACTIVE) == 0 && full != 0 {
        let day = read_i16(this + 0xb6) as u16; // cqr10: o dia do sorteio
        let year = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + league::YEAR) as u16);
        if year == core::ptr::read_unaligned((today + 2) as *const u16) && day == core::ptr::read_unaligned(today as *const u16) {
            crate::comparison::branch("cqr_10 sorteio da libertadores");
            clubs(this);
            if first(this) == 0 {
                show_error_at(0x9a7de4, 0x365);
            }
            if games(this) == 0 {
                show_error_at(0x9a7de4, 0x36a);
            }
            if sort(this, 0) == 0 {
                show_error_at(0x9a7de4, 0x370);
            }
            for g in 0..7i32 {
                stage(this, g);
            }
            let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
            predicted(this);
            soltar(read_u32_as_usize(this + 0xee));
            write_u32_from_usize(this + 0xee, 0);
            write_u32_from_usize(this + league::ACTIVE, 1);
        }
    }
    league(this, today, full)
}

// simplification: the clubs (shuffled), the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee zeroed, the first group’s table and the 7 stages
// created and the current stage (+0x30) the last of them.

/// Builds the table of the first group of the Copa Libertadores. Rule: group 1 has 4 clubs, the first 4 drawn, each in
/// the row of its order; human managers get the group draw news.
///
/// Ret 0: 4 teams (+0x3e), table +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 rows of 0x3b bytes; out of
/// memory: the box at line 0x16f and returns 0), each row i with drawn club i (+0xee) (0x687430 League::init_table_row:
/// push 0; push 0; push club; push i; mov ecx, this -> thiscall, ret 0x10), the group draw news (0x684230: mov ecx,
/// this -> thiscall, ret 0) and returns 1. The same body as 0x4019c0 of the African Nations Cup (only the file and the
/// box line change).
/// Original: `ConmebolLiber::build_first_group_table` (0x4c37a0, conmebol_liber.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c37a0 } (thiscall, ret 0, conmebol_liber.cpp): a tabela do primeiro grupo da Libertadores: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x16f e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
pub(crate) unsafe extern "thiscall" fn build_first_group_table(this: usize) -> u32 {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let row: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, 4);
    let t = new(0xec);
    write_u32_from_usize(this + league::TABLE, t);
    if t == 0 {
        show_error_at(0x9a7de4, 0x16f);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        crate::comparison::branch("cqr_10 linha do primeiro grupo");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        row(this, i as u8 as u32, conmebol_liber_drawn_club(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the drawn team i with position i (+4).

/// Creates a group stage of the Copa Libertadores. Rule: each of the other groups is a league stage with the next 4
/// drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get
/// the news.
///
/// Ret 4; SEH frame only for the stage's new (no destructor on the normal path): the stage of group g + 1 (g from 0 to
/// 6, from the draw day). The teams in a vector of +0x3e pointers (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; out
/// of memory: the box at line 0x218 and 0): drawn clubs 4g + 4 to 4g + 7 (+0xee); the round table through virtual +0x3c
/// (push 0; push &id; push &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the stage (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; the League_Stage constructor 0x68b100, thiscall, ret 0x58, 22 arguments: the cup, the teams (+0x3e,
/// word), the vector, +0x3c (word), +4, the round table, the rounds (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, the
/// year +0x40 (word), g, the id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; the original pushes the words and bytes
/// with garbage in the high bytes, which the constructor does not read: the copy passes clean values) in `[+0xc][g]`;
/// the vector and the round table are freed (0x9452ca). No stage: the box at line 0x22e and 0. With it: the group news
/// (0x684230 on the stage), +0x30 = g and 1.
/// Original: `ConmebolLiber::create_group_stage` (0x4c3b10, conmebol_liber.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c3b10 } (thiscall, ret 4, conmebol_liber.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 6, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x218 e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x22e e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
pub(crate) unsafe extern "thiscall" fn conmebol_liber_create_group_stage(this: usize, g: i32) -> u32 {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = alloc(((read_i16(this + league::CLUB_COUNT) as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0x9a7de4, 0x218);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparison::branch("cqr_10 time do grupo");
        write_u32_from_usize(teams + n * 4, conmebol_liber_drawn_club(this, k as usize)); // cqr10: o time na vaga do grupo
        k += 1;
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, g, &mut rodadas, &mut id, 0);
    let p = new(0xee);
    let stage = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construct(
            p,
            this,
            read_i16(this + league::CLUB_COUNT) as u16 as u32,
            teams,
            read_i16(this + league::LEGS) as u16 as u32,
            read_u32_as_usize(this + 4),
            table,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT,
            read_i16(this + league::YEAR) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            read_i16(this + league::LEAGUE_FLAGS) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4, stage);
    soltar(teams);
    soltar(table);
    let f = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4);
    if f == 0 {
        show_error_at(0x9a7de4, 0x22e);
        return 0;
    }
    noticia(f);
    write_u32_from_usize(this + league::CURRENT_STAGE, g as usize);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][g], +0x30 = g, the stage with the teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).

/// As regras de um grupo continental (a virtual +0x8c, set_up_rules, numa liga de grupo): os campos de byte e palavra
/// gravados (deslocamento, tamanho 1 ou 2, valor), a data de começo (0x549f70: dia, mês, ano da competição +0x40, dia da
/// semana; −1 = qualquer) e o fundo de prêmios (+0x28), quando há.
pub(crate) struct ContinentalGroupRules {
    pub(crate) campos: &'static [(usize, u8, u32)],
    pub(crate) dia: u32,
    pub(crate) mes: u32,
    pub(crate) semana: u32,
    pub(crate) premio: Option<u32>,
}

/// Writes the rule fields (win points from `vitoria`), start date word at +0xb6, +0xb8 = 0, prize, and the round table
/// at +0xba; returns 1.
///
/// [pt] Grava os campos de `g` (os pontos da vitória, +0xc2, vêm de `vitoria`); a data de começo numa Date local (0x549f70: push semana; push ano; push mês; push dia; lea
/// ecx, local -> thiscall, ret 0x10) e a primeira palavra dela em +0xb6, +0xb8 = 0; o prêmio; depois a tabela das
/// rodadas da virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba.
/// Devolve 1.
pub(crate) unsafe fn apply_cup_rules(this: usize, g: &ContinentalGroupRules, day: u32, win: u8) -> u32 {
    for &(o, n, v) in g.campos {
        let v = if o == 0xc2 { win as u32 } else { v };
        if n == 1 {
            *((this + o) as *mut u8) = v as u8;
        } else if n == 2 {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        } else {
            core::ptr::write_unaligned((this + o) as *mut u32, v);
        }
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    // simplification: the year goes in a register that may have garbage in the high 16 bits; 0x549f70 reads it as a word
    data(d.as_mut_ptr(), day, g.mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, g.semana);
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, core::ptr::read_unaligned(d.as_ptr() as *const u16));
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, 0);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + 0x28) as *mut u32, p);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = table(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// Comparer: runs the original and checks rule fields, date word, +0xb8, prize and return 1.
///
/// [pt] O plano: os campos de `g`, a palavra da data (calculada antes numa Date local pela 0x549f70, sem efeito fora dela),
/// +0xb8 = 0, o prêmio e a devolução 1.
pub(crate) unsafe fn check_cup_rules(nome: &str, this: usize, g: &ContinentalGroupRules, o: impl FnOnce() -> u32) -> u32 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    data(d.as_mut_ptr(), g.dia, g.mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, g.semana);
    let palavra = core::ptr::read_unaligned(d.as_ptr() as *const u16);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in g.campos {
        let lido = match n {
            1 => *((this + off) as *const u8) as u32,
            2 => core::ptr::read_unaligned((this + off) as *const u16) as u32,
            _ => core::ptr::read_unaligned((this + off) as *const u32),
        };
        let v = if n == 1 { v & 0xff } else if n == 2 { v & 0xffff } else { v };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = core::ptr::read_unaligned((this + 0xb6) as *const u16);
    let b8 = core::ptr::read_unaligned((this + 0xb8) as *const u16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + 0x28) as *const u32) == p);
    PLACAR.record(r == 1 && k == 0 && b6 == palavra && b8 == 0 && premio, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {} prêmio ok {}", nome, this, r, &errados[..k], b6, palavra, b8, premio));
    r
}

pub(crate) const LIBERTADORES_RULES: ContinentalGroupRules = ContinentalGroupRules {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7),
        (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    dia: 0xf, mes: 0, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

pub(crate) const MERCONORTE_RULES: ContinentalGroupRules = ContinentalGroupRules {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1), (0xbe, 1, 1),
        (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff)],
    dia: 1, mes: 7, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

pub(crate) const OCEANIA_RULES: ContinentalGroupRules = ContinentalGroupRules {
    campos: &[(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x49, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 8),
        (0xc5, 1, 1), (0xc7, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xd9, 2, 0x28)],
    dia: 0x14, mes: 7, semana: 5, premio: None,
};

/// rule: the Libertadores groups are home and away (double round robin), 3 points a win and 1 a draw, tiebreak by codes
/// 1, 2 and 3 [inference: goal difference, goals for, wins]; two qualifiers per group; clubs from several nations;
/// category 7 on each match; 7 substitutes on the bench and 3 changes; starts on day 15 of month 0 of the competition
/// year [inference: 15 January, months counted from 0]; prize fund of 2,000,000
/// Original: `ConmebolLiber::set_up_rules` (0x4c3890, conmebol_liber.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c3890 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `ConmebolLiber::set_up_rules()`: os grupos da Libertadores em dois turnos, 3 pontos a vitória e 1 o empate, os
/// desempates (1, 2, 3, 0), dois classificados por grupo, clubes internacionais (+0x42 = 2), categoria 7, 7 no banco e 3
/// substituições, começo no dia 15 do mês 0 do ano da competição, prêmios de 2.000.000; depois a tabela das rodadas.
pub(crate) unsafe extern "thiscall" fn conmebol_liber_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_11 regras da libertadores");
    let day = 0xf; // cqj11: o dia do começo da Libertadores
    let win = 3; // cqj11: a vitória nos grupos da Libertadores
    apply_cup_rules(this, &LIBERTADORES_RULES, day, win)
}

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.


// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.


// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.



/// Clubs whose season continental competition (+0x1db) is this competition, in database order, up to max.
///
/// [pt] Os clubes da competição da Libertadores na ordem do banco ([0xae23b4], 0x245 bytes, [0xae235c] clubes): os que têm
/// a competição europeia/continental da temporada (+0x1db) igual ao id da competição ([+4]), até `max`.
pub(crate) unsafe fn libertadores_entrants(this: usize, max: i32, out: &mut [usize; 32]) -> i32 {
    let mut n = 0i32;
    let mut i = 0i16;
    let mut clube = read_u32_as_usize(G_CLUB_TABLE);
    if (read_u32_as_usize(G_CLUB_COUNT) as i32) > 0 {
        loop {
            if read_u32_as_usize(clube + club::EURO_FLAG) == read_u32_as_usize(read_u32_as_usize(this + 4)) && n < max {
                if (n as usize) < out.len() {
                    out[n as usize] = clube;
                }
                n += 1;
            }
            i = i.wrapping_add(1);
            clube = clube.wrapping_add(0x245);
            if i as i32 >= read_u32_as_usize(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    n
}

/// Selects the clubs qualified for the Libertadores. Rule: takes the 32 clubs marked with it as the season's
/// continental competition (the mark comes from league qualification), in database order, then shuffled (96 random
/// swaps) for the groups; any other count is an error (the box).
///
/// Ret 0: `ConmebolLiber::select_qualified_clubs()`: +0xf2 = 32 and the list +0xee (0x944e46(0x80): push 0x80; add esp,
/// 4 -> cdecl; out of memory: the box at line 0x285 and returns 0) with the clubs, in database order, whose season
/// competition (+0x1db) is the Libertadores, up to 32; fewer than 32: the box at line 0x297 and returns 0; otherwise
/// the list is shuffled (0x4afb40(list, 32): push n; push list; add esp, 8 -> cdecl; 3 × 32 random swaps) and returns
/// 1.
/// Original: `ConmebolLiber::select_qualified_clubs` (0x4c3f40, conmebol_liber.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c3f40 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::select_qualified_clubs()`: +0xf2 = 32 e a lista
/// +0xee (0x944e46(0x80): push 0x80; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x285 e devolve 0) com os
/// clubes, na ordem do banco, cuja competição da temporada (+0x1db) é a Libertadores, até 32; menos de 32, a caixa da
/// linha 0x297 e devolve 0; senão a lista embaralhada (0x4afb40(lista, 32): push n; push lista; add esp, 8 -> cdecl; 3 ×
/// 32 trocas sorteadas) e devolve 1.
pub(crate) unsafe extern "thiscall" fn conmebol_liber_select_qualified_clubs(this: usize) -> u32 {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    let list = new(0x80);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, list as u32);
    if list == 0 {
        show_error_at(0x9a7de4, 0x285);
        return 0;
    }
    let mut v = [0usize; 32];
    let n = libertadores_entrants(this, read_u32_as_usize(this + 0xf2) as i32, &mut v);
    for (k, &c) in v.iter().enumerate().take(n.min(32) as usize) {
        core::ptr::write_unaligned((list + k * 4) as *mut u32, c as u32); // cqk11: o clube na ordem do banco
    }
    if n != read_u32_as_usize(this + 0xf2) as i32 {
        show_error_at(0x9a7de4, 0x297);
        return 0;
    }
    crate::comparison::branch("cqk_11 classificados da libertadores");
    embaralhar(list, read_i16(this + 0xf2) as u16 as u32); // cqk11: os 32 da Libertadores embaralhados
    1
}


/// rule: in the seven groups (stages 0 to 6), a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 7), a club going through gets the
/// qualification by round (weight 50 to 100), one going out the knocked-out record; the winner gets the title record
/// and goes to the Intercontinental Cup, in the South American place (place 0 if a club of continent [0x9cfa1c] is
/// there, otherwise place 1), with the Intercontinental season archive and predicted positions; the runner-up gets the
/// runner-up record
/// Original: `ConmebolLiber::update_staff_records_for_status` (0x4c4080, conmebol_liber.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x4c4080 } (thiscall, ret 0x18, conmebol_liber.cpp; a virtual +0x44): `ConmebolLiber::
/// update_staff_records_for_status(clube, desfecho, fase, _, jogo, a6)`; devolve 0. Fase (byte com sinal) abaixo de 7,
/// os grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x14, peso 0x28); 0xfe, a não classificação (0x86c1d0:
/// 0x41c, 0x28). Fase 7, o mata-mata: peso 0x4bf850(0x32, 0x64, rodada do jogo +0x34, 4); 0xff, a eliminação (0x86c000:
/// +0x32, +0x30 do jogo, peso); 0, a classificação com +0x32 e o +0x6f da rodada do jogo na tabela de rodadas da fase 7
/// ([+0xc] + 0x1c, +0xa3, 0x68 bytes; os 16 bits altos que a original empurra são os do ponteiro da fase); 1, o registro
/// de campeão (0x86a800) e o clube vai ao Mundial Interclubes (o objeto [[0xadadfc] + 4 × [0x9cf6e4]]): na vaga 0 da
/// tabela +0xa7 dele se o clube que lá está é do continente [0x9cfa1c] (a nação +0x53, o continente +0x71 da nação, +0
/// o id), senão na vaga 1; depois o arquivo da temporada das estatísticas dele (0x4a1c50 em +8: push 1 -> thiscall, ret
/// 4) e as posições previstas (virtual +0x5c); 2, o registro de vice (0x86b0b0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_update_staff_records_for_status(this: usize, clube: usize, outcome: u32, stage: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    let final_ = |g: usize| {
        let h: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(g);
        h(eng_second_staff_records(), clube, jogo, a6);
    };
    if f < 7 {
        if d == 6 {
            european_cup_add_qualified_record(clube, comp, 0, 0x14, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x28);
        }
    } else if f == 7 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, read_i16(jogo + 0x34), 4);
                // simplification: +0x30 goes in a register with garbage in the high 16 bits; 0x86c000 only uses the word
                european_cup_add_knocked_out_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, read_i16(jogo + 0x34), 4);
                let knockout = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + 0x1c);
                let r = read_i16(jogo + 0x34) as isize;
                let round = (knockout as u32 & 0xffff_0000) | core::ptr::read_unaligned((read_u32_as_usize(knockout + 0xa3).wrapping_add((r * 0x68) as usize) + 0x6f) as *const u16) as u32;
                european_cup_add_qualified_record(clube, comp, core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, round, weight);
            }
            1 => {
                final_(0x86a800);
                let mundial = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP).wrapping_mul(4)));
                let t = read_u32_as_usize(mundial + 0xa7);
                let nacao = read_u32_as_usize(read_u32_as_usize(t) + 0x53);
                crate::comparison::branch("cqk_11 campeao da libertadores");
                if nacao != 0 && read_u32_as_usize(read_u32_as_usize(nacao + nation::CONTINENT)) == read_u32_as_usize(KEY_CONTINENT_SOUTH_AMERICA) {
                    core::ptr::write_unaligned(t as *mut u32, clube as u32); // cqk11: o campeão na vaga sul-americana
                } else {
                    core::ptr::write_unaligned((t + 6) as *mut u32, clube as u32);
                }
                let archive: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
                archive(read_u32_as_usize(mundial + 8), 1);
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(mundial) + 0x5c));
                prever(mundial);
            }
            2 => final_(0x86b0b0),
            _ => {}
        }
    }
    0
}

// simplification: the records (history, call-ups), the statistics file and the predicted World Cup positions do not
// run twice. Original only; the plan: return 0 and, in the title (stage 7, outcome 1), the champion in the slot of the
// Club World Championship chosen by the rule (0 if a club from continent [0x9cfa1c] is there, otherwise 1).

/// rule: the Libertadores has seven group stages (0 to 6) and the knockout at stage 7 (the top two of each group, 16
/// clubs); going past the last stage is an error (the box)
/// Original: `ConmebolLiber::advance_stage` (0x4c3a80, conmebol_liber.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c3a80 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x28): `ConmebolLiber::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x1f6; senão +0x30 sobe e, chegando a 7 (o fim dos sete grupos), o
/// mata-mata (0x4c3d10: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_advance_stage(this: usize) {
    let f = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    if f >= (read_u32_as_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        show_error_at(0x9a7de4, 0x1f6);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, f);
    if f == 7 { // cqk11: o mata-mata depois dos sete grupos
        crate::comparison::branch("cqk_11 mata-mata da libertadores");
        let knockout: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONMEBOLLIBER_CREATE_KNOCKOUT_STAGE);
        knockout(this);
    }
}

// simplification: the knockout (0x4c3d10) allocates the stage and draws: does not run twice. Original only; the plan: +0x30 goes up one
// (outside the last stage, which gives the message box).

/// rule: the Copa Libertadores starts with six-round groups on Tuesdays [inference: weekday 2], from 25/2 to 1/4 (each
/// group a league stage, id 0x3e9 + stage, up to stage 6: 8 groups [inference]); then the knockout (stage 7) over two
/// legs: round of 16 drawn on 2/4 and played on 15/4 (16 teams, return 14 days later), quarter-finals 30/4 and 20/5,
/// semi-final 28/5 and 3/6, final draw 11/6 and match on 17/6 (return 7 days later); two-leg tie-break 1 and draw with
/// bit 4; category 7 in the round of 16 and 8 after; final prize 2 million to the winner and 800 thousand to the
/// runner-up
/// Original: `ConmebolLiber::create_schedule` (0x4c31c0, conmebol_liber.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c31c0 } (thiscall, ret 0x10, conmebol_liber.cpp; a virtual +0x3c da Libertadores): `ConmebolLiber::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (a caixa da linha 0x9f), o mata-mata na
/// fase 7 (a caixa da linha 0xc5).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    south_american_cup_schedule(this, stage, rounds, extra, league, 7, &LIBERTADORES_GROUP_ROUNDS, &LIBERTADORES_KNOCKOUT_ROUNDS, &[], 0x9a7de4, 0x9f, 0xc5,
        "cqo_13 grupos da libertadores", "cqo_13 mata-mata da libertadores")
}


/// rule: the Libertadores knockout has 16 clubs: the top 2 (+0xbe) of each of the 8 groups (group 1 in table order, the
/// others by position in each stage's table), in a cup stage (stage 7) with the cup's round table and year; the first
/// round is drawn at once
/// Original: `ConmebolLiber::create_knockout_stage` (0x4c3d10, conmebol_liber.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c3d10 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::create_knockout_stage()`, o mata-mata da
/// Libertadores: os 16 (os 2 primeiros do grupo 1 e das 7 fases de grupo) na fase 7; caixas nas linhas 0x24c (sem
/// lista) e 0x26a (sem fase).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_create_knockout_stage(this: usize) {
    build_knockout_stage(this, 0x10, 7, 7, 0x9a7de4, 0x24c, 0x26a, "cqs_10 classificados da libertadores");
}

