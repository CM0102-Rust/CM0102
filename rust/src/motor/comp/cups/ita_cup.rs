use super::*;

/// The table of group 1 of an Italian cup (`create_first_group_table`; the cup is itself the league of group 1).
///
/// Sets `+0x3e` = `n` teams; allocates the table (`0x944e46` of n × 0x3b, cdecl) into `+0xb1`; out of memory, the box
/// of line `linha` and 0.
///
/// Each of the first n entrants (`+0xee`, pointer vector) goes to row i of the table (League::init_table_row
/// `0x687430`: push 0; push 0; push club; push i (the original routine pushes the dword of the local byte, with stack
/// garbage above it; `0x687430` only reads the byte); mov ecx, cup -> thiscall, ret 0x10) and its slot in `+0xee` is
/// zeroed (it leaves the list of those waiting for the next stage); the index is a signed byte compared with the word
/// `+0x3e`.
///
/// Then the group draw news (League::send_group_draw_news `0x684230`, mov ecx -> thiscall, ret 0). Returns 1.
///
/// [pt] A tabela do grupo 1 de uma copa italiana (`create_first_group_table`; a copa é ela mesma a liga do grupo 1): +0x3e =
/// `n` times; a tabela (0x944e46 de n × 0x3b, cdecl) em +0xb1; sem memória, a caixa da linha `linha` e 0. Cada um dos n
/// primeiros inscritos (+0xee, vetor de ponteiros) vai à linha i da tabela (League::init_table_row 0x687430: push 0; push
/// 0; push clube; push i (a original empurra o dword do byte local, com lixo da pilha em cima; a 0x687430 só lê o byte);
/// mov ecx, copa -> thiscall, ret 0x10) e a vaga dele em +0xee é zerada (sai da lista dos que esperam a próxima fase); o
/// índice é um byte com sinal comparado à palavra +0x3e. Depois a notícia do sorteio do grupo (League::send_group_draw_news
/// 0x684230, mov ecx -> thiscall, ret 0). Devolve 1.
pub(crate) unsafe fn create_first_group_table(this: usize, n: i16, file: usize, row: u32, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, n);
    let t = exe::crt_malloc(n as i32 as u32 * 0x3b);
    core::ptr::write_unaligned((this + league::TABLE) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    crate::comparison::branch(ramo);
    let mut i: i8 = 0;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        let slot = read_usize(this + 0xee) + i as isize as usize * 4;
        exe::league_init_table_row(this, i as u8 as u32, read_usize(slot), 0, 0);
        core::ptr::write_unaligned((read_usize(this + 0xee) + i as isize as usize * 4) as *mut u32, 0); // cqr7: the group 1 slot leaves the list
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Runs only the original routine and checks the plan made beforehand.
///
/// Simplification: 0x687430 allocates the form and the head-to-head of each row and the news item goes to the human
/// players, so it cannot run twice.
///
/// The check: returns 1, +0x3e = n, the table at +0xb1 holds the first n entrants in its rows (of 0x3b bytes), in
/// order, and the first n slots of +0xee are zeroed.
///
/// [pt] Só a original (simplification: a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos; não
/// pode rodar duas vezes), conferindo o plano feito antes: devolve 1, +0x3e = n, a tabela em +0xb1 com os n primeiros
/// inscritos nas linhas (de 0x3b bytes), na ordem, e as n primeiras vagas de +0xee zeradas.
pub(crate) unsafe fn compare_first_group_table(name: &str, this: usize, n: i16, original: impl FnOnce() -> u32) -> u32 {
    let plan: Vec<usize> = (0..n as usize).map(|k| read_usize(read_usize(this + 0xee) + k * 4)).collect();
    let r = original();
    let t = read_usize(this + league::TABLE);
    let ok = r == 1 && read_i16(this + league::CLUB_COUNT) == n && t != 0
        && plan.iter().enumerate().all(|(k, &c)| read_usize(t + k * 0x3b) == c && read_usize(read_usize(this + 0xee) + k * 4) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}) devolveu {}; plano {:x?}", name, this, r, plan));
    r
}

/// ItaCup::create_first_group_table: creates group 1 of the Coppa Italia with 4 teams (via the shared
/// create_first_group_table, error box at line 0x19e of ita_cup.cpp); returns 1, or 0 when out of memory.
///
/// [pt] { 0x63f250 } (thiscall, ret 0, ita_cup.cpp; do construtor): `ItaCup::create_first_group_table()`: o grupo 1 da Copa
/// da Itália com 4 times (`create_first_group_table`; a caixa do ita_cup.cpp na linha 0x19e).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_first_group_table(this: usize) -> u32 {
    create_first_group_table(this, 4, 0x9ea23c, 0x19e, "cqr_7 grupo 1 da copa da italia")
}

/// ItaCCup::init_first_group(), called from the constructor: builds the table of group 1 of the Italian Serie C cup
/// with 5 teams (via create_first_group_table). Returns 1, or 0 when memory runs out.
///
/// [pt] { 0x63d5e0 } (thiscall, ret 0, ita_c_cup.cpp; do construtor): `ItaCCup::init_first_group()`: o grupo 1 da Copa da
/// Itália da Série C com 5 times (`create_first_group_table`; a caixa do ita_c_cup.cpp na linha 0x1a0).
pub(crate) unsafe extern "thiscall" fn init_first_group(this: usize) -> u32 {
    create_first_group_table(this, 5, 0x9ea204, 0x1a0, "cqr_7 grupo 1 da copa da serie c") // cqr7: 5 teams in group 1 of Serie C
}

/// Advances an Italian cup to the next stage (`advance_stage`, virtual +0x28).
///
/// On the last stage (+0x30 ≥ +0x2c − 1, signed), the error box and nothing else. Otherwise +0x30 goes up by one and,
/// on reaching `ultimo_grupo` + 1 (the end of the groups), it builds the knockout (`mata_mata`, direct call with the
/// cup's ecx -> thiscall, ret 0).
///
/// [pt] A fase seguinte de uma copa italiana (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal), a caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a `ultimo_grupo` + 1 (o fim dos grupos), monta o
/// mata-mata (`mata_mata`, call direto com o ecx da copa -> thiscall, ret 0).
pub(crate) unsafe fn advance_cup_stage(this: usize, file: usize, row: u32, end_of_groups: i32, knockout_knockout: usize) {
    let stage = read_usize(this + league::CURRENT_STAGE) as i32;
    if stage >= (read_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, stage);
    if stage == end_of_groups {
        crate::comparison::branch("cqr_7 mata-mata das copas italianas");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout_knockout);
        f(this); // cqr7: monta o mata-mata
    }
}

/// Comparer of advance_stage: original only, checking the stage (+0x30) goes up by one except on the last stage.
///
/// [pt] O comparador da fase seguinte: só a original (simplification: o mata-mata aloca a fase e sorteia; não pode rodar duas vezes),
/// conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
pub(crate) unsafe fn ita_cup_compare_advance_stage(name: &str, this: usize, original: impl FnOnce()) {
    let before = read_usize(this + league::CURRENT_STAGE) as i32;
    let last = before >= (read_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1);
    original();
    let after = read_usize(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("{}({:#x}) fase {} -> {}", name, this, before, after));
}

/// ItaCup::advance_stage: moves the Italian Cup to its next stage (shared `advance_cup_stage`, error box at ita_cup.cpp
/// line 0x23e); after the group stages (stage 7) it builds the knockout rounds (ItaCup::create_knockout_stage
/// 0x63f870).
///
/// [pt] { 0x63f5b0 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x28): `ItaCup::advance_stage()`: a fase seguinte
/// (`advance_cup_stage`; a caixa do ita_cup.cpp na linha 0x23e); depois da fase 6 (os 8 grupos: o grupo 1 e as fases
/// 0..6), o mata-mata da Copa da Itália (ItaCup::create_knockout_stage 0x63f870).
pub(crate) unsafe extern "thiscall" fn ita_cup_advance_stage(this: usize) {
    advance_cup_stage(this, 0x9ea23c, 0x23e, 7, 0x63f870) // cqr7: the Coppa Italia knockout in stage 7
}

/// ItaCCup::advance_stage() (virtual +0x28) for the Italian Serie C cup: moves the cup to its next stage (error box at
/// the last stage) and, once the 14 group stages are over, builds the knockout stage (ItaCCup::create_knockout_stage,
/// 0x63dc10).
///
/// [pt] { 0x63d950 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x28): `ItaCCup::advance_stage()`: a fase seguinte
/// (`advance_cup_stage`; a caixa do ita_c_cup.cpp na linha 0x240); depois da fase 14 (os 16 grupos), o mata-mata da Copa
/// da Série C (ItaCCup::create_knockout_stage 0x63dc10).
pub(crate) unsafe extern "thiscall" fn ita_cup_advance_stage_63d950(this: usize) {
    advance_cup_stage(this, 0x9ea204, 0x240, 0xf, 0x63dc10)
}

/// The stage of group `g` of an Italian cup (`create_group_stage(g)`): builds the League_Stage of group g with the next
/// entrants from +0xee, stores it in [+0xc][g], sends the draw news and sets +0x30 = g.
///
/// SEH frame only for the new: no destructor on the normal path. Shared template of the Coppa Italia and Serie C,
/// checked instruction by instruction: only the source file and the box lines change.
///
/// The round table via the cup's method +0x3c (push 0; push &id; push &rounds; push g; mov ecx -> thiscall, ret 0x10);
/// a vector of +0x3e pointers (0x944e46, cdecl; out of memory, the box of `linhas[0]` and 0) with the first +0x3e
/// entrants still in the list (+0xee, vector of pointers: the non-null ones, in order, each zeroed as it leaves).
///
/// The stage (0x944cf1(0xee), cdecl; the League_Stage constructor 0x68b100, thiscall, ret 0x58, 22 arguments: the cup,
/// the teams (word +0x3e), the vector, 1, +4, the round table, the rounds (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe,
/// the year +0x40 (word), g, the id (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; the original pushes the words and
/// bytes with garbage in the high bytes, which the constructor does not read: the copy passes clean values) is stored
/// in [+0xc][g]; the vector and the round table are freed (0x9452ca).
///
/// No stage: the box of `linhas[1]` and 0. With it: the group draw news (0x684230 on the stage), +0x30 = g and 1.
///
/// [pt] A fase do grupo `g` de uma copa italiana (`create_group_stage(g)`, quadro SEH só para o new: sem destrutor no caminho
/// normal; o molde da Copa da Itália e da Série C, conferido instrução a instrução: só mudam o fonte e as linhas das
/// caixas). A tabela das rodadas pelo método +0x3c da copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall,
/// ret 0x10); um vetor de +0x3e ponteiros (0x944e46, cdecl; sem memória, a caixa da `linhas[0]` e 0) com os primeiros
/// +0x3e inscritos ainda na lista (+0xee, vetor de ponteiros: os não nulos, na ordem, cada um zerado ao sair); a fase
/// (0x944cf1(0xee), cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (word
/// +0x3e), o vetor, 1, +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40
/// (word), g, o id (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da `linhas[1]` e 0. Com ela: a notícia do sorteio do grupo (0x684230
/// na fase), +0x30 = g e 1.
pub(crate) unsafe fn ita_cup_create_group_stage_cqr8_fase_do_grupo(this: usize, g: u32, file: usize, rows: [u32; 2], ramo: &'static str) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, g, &mut rodadas, &mut id, 0);
    let teams = exe::crt_malloc(((read_i16(this + league::CLUB_COUNT) as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    if read_i16(this + league::CLUB_COUNT) > 0 {
        let mut k = 0i32;
        let mut o = 0usize;
        loop {
            let c = read_usize(read_usize(this + 0xee) + o);
            if c != 0 {
                crate::comparison::branch(ramo);
                write_usize(teams + k as usize * 4, c); // cqr8: the team in slot k of the group
                write_usize(read_usize(this + 0xee) + o, 0);
                k += 1;
            }
            o += 4;
            if k >= read_i16(this + league::CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this,
            read_i16(this + league::CLUB_COUNT) as u16 as u32,
            teams,
            1,
            read_usize(this + 4),
            table,
            rodadas as u16 as u32,
            read_u8(this + league::WIN_POINTS) as u32,
            read_u8(this + league::DRAW_POINTS) as u32,
            read_u8(this + league::MATCH_CATEGORY) as u32,
            this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT,
            read_i16(this + league::YEAR) as u16 as u32,
            g,
            id as u16 as u32,
            read_u8(this + league::IMPORTANCE) as u32,
            1,
            0,
            read_i16(this + league::LEAGUE_FLAGS) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES) + g as usize * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES) + g as usize * 4);
    if f == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    exe::league_send_group_draw_news(f);
    write_usize(this + league::CURRENT_STAGE, g as usize);
    1
}

/// Runs only the original routine (it allocates the stage, which allocates its own table, and sends the news item; it
/// cannot run twice) and checks it against the plan made beforehand.
///
/// Expected: returns 1 with the stage stored in [+0xc][g] and +0x30 = g; the stage has the cup's team count (+0x3e) and
/// year (+0x40); the first +0x3e non-null entrants of +0xee are zeroed and, in the stage table (+0xb1, rows of 0x3b
/// bytes), those clubs are present (in any order: the stage sorts its own table [inference, as in the World Cup group
/// stage]).
///
/// [pt] Só a original (aloca a fase, que aloca a tabela dela, e manda a notícia; não pode rodar duas vezes), conferindo o plano
/// feito antes: devolve 1 com a fase em [+0xc][g] e +0x30 = g; a fase com os times (+0x3e) e o ano (+0x40) da copa; os
/// primeiros +0x3e inscritos não nulos de +0xee zerados e, na tabela da fase (+0xb1, linhas de 0x3b bytes), esses clubes
/// (em qualquer ordem: a fase ordena a tabela dela [inferência, como na fase dos grupos da Copa do Mundo]).
pub(crate) unsafe fn compare_group_stage(name: &str, this: usize, g: u32, original: impl FnOnce() -> u32) -> u32 {
    let n = read_i16(this + league::CLUB_COUNT).max(0) as usize;
    let mut plan = Vec::new();
    let mut slots = Vec::new();
    let mut o = 0usize;
    while plan.len() < n && o < 0x400 {
        let c = read_usize(read_usize(this + 0xee) + o);
        if c != 0 {
            plan.push(c);
            slots.push(o);
        }
        o += 4;
    }
    let r = original();
    let mut ok = r == 1;
    if ok {
        let f = read_usize(read_usize(this + league::STAGES) + g as usize * 4);
        let t = read_usize(f + league::TABLE);
        ok = f != 0 && read_usize(this + league::CURRENT_STAGE) == g as usize && read_i16(f + league::CLUB_COUNT) == read_i16(this + league::CLUB_COUNT) && read_i16(f + league::YEAR) == read_i16(this + league::YEAR)
            && slots.iter().all(|&o| read_usize(read_usize(this + 0xee) + o) == 0)
            && (t == 0 || {
                let mut a: Vec<usize> = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
                let mut b = plan.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}) devolveu {}; plano {:x?}", name, this, g, r, plan));
    r
}

/// ItaCup::create_group_stage(g): creates the league stage of group g of the Italian Cup from the entrants still
/// waiting, stores it in the cup's stage list and sends the group draw news. Returns 1, or 0 (after the original error
/// box) when allocation fails.
///
/// [pt] { 0x63f640 } (thiscall, ret 4, ita_cup.cpp): `ItaCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// (`ita_cup_create_group_stage_cqr8_fase_do_grupo`; as caixas do ita_cup.cpp nas linhas 0x263 e 0x27c).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_group_stage(this: usize, g: u32) -> u32 {
    ita_cup_create_group_stage_cqr8_fase_do_grupo(this, g, 0x9ea23c, [0x263, 0x27c], "cqr_8 time do grupo da copa da italia")
}

/// ItaCCup::create_group_stage(g): builds the stage of group `g` of the Serie C Italian Cup through the shared
/// group-stage builder (error boxes at ita_c_cup.cpp lines 0x265 and 0x27e). Returns the builder's result.
///
/// [pt] { 0x63d9e0 } (thiscall, ret 4, ita_c_cup.cpp): `ItaCCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// da Série C (`ita_cup_create_group_stage_cqr8_fase_do_grupo`; as caixas do ita_c_cup.cpp nas linhas 0x265 e 0x27e).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_group_stage_63d9e0(this: usize, g: u32) -> u32 {
    ita_cup_create_group_stage_cqr8_fase_do_grupo(this, g, 0x9ea204, [0x265, 0x27e], "cqr_8 time do grupo da copa da serie c")
}

/// League table (+0xb1, 0x3b-byte rows) of group i: group 0 is the cup itself, group i ≥ 1 is stage [+0xc][i - 1].
///
/// [pt] A tabela (+0xb1, linhas de 0x3b bytes) do grupo `i` de uma copa italiana: o grupo 0 é a própria copa; o grupo i ≥ 1 é
/// a fase [+0xc][i − 1].
pub(crate) unsafe fn group_table(this: usize, i: usize) -> usize {
    if i == 0 {
        read_usize(this + league::TABLE)
    } else {
        read_usize(read_usize(read_usize(this + league::STAGES) + (i - 1) * 4) + 0xb1)
    }
}

/// Copies the entrants still waiting in +0xee into the knockout team array from slot k up to fim, clearing each.
///
/// [pt] Os que entram direto no mata-mata: a partir da vaga `k` do vetor `times`, até `fim`, os inscritos ainda na lista (+0xee,
/// até +0xf2 vagas: os não nulos, na ordem), cada um zerado ao sair.
pub(crate) unsafe fn take_direct_entrants(this: usize, teams: usize, mut k: usize, end: usize) {
    let mut j = 0i32;
    if (read_usize(this + 0xf2) as i32) <= 0 {
        return;
    }
    loop {
        if k >= end {
            break;
        }
        let c = read_usize(read_usize(this + 0xee) + j as usize * 4);
        if c != 0 {
            write_usize(teams + k * 4, c);
            write_usize(read_usize(this + 0xee) + j as usize * 4, 0);
            k += 1;
        }
        j += 1;
        if j >= read_usize(this + 0xf2) as i32 {
            break;
        }
    }
}

/// Builds the knockout stage (Cup_Stage) with the `n` teams of `times`, frees the temporaries and draws the first
/// round.
///
/// The rounds come from the cup's method +0x3c (push 0; push &extra; push &rodadas; push fase; mov ecx -> thiscall, ret
/// 0x10). The stage (0x944cf1(0xb2), cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: the cup, n, the vector,
/// the rounds (word), +4, the round table, the year +0x40 (word), the stage, 1, the extra (word), 0x14, 0, 0, 0 and
/// `sementes`; the original routine pushes the words with garbage in the high 16 bits, which the constructor does not
/// read) goes into [+0xc][fase]; the vector and the round table are freed (0x9452ca, cdecl; in this order).
///
/// Without a stage, the error box for `linha`; with one, the draw of the first round (Cup::draw_round 0x51c410: push 0;
/// mov ecx, stage -> thiscall, ret 4).
///
/// [pt] O mata-mata (Cup_Stage) com os `n` times de `times`: as rodadas pelo método +0x3c da copa (push 0; push &extra; push
/// &rodadas; push fase; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2), cdecl; Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, n, o vetor, as rodadas (word), +4, a tabela das rodadas, o ano +0x40 (word), a fase, 1, o
/// extra (word), 0x14, 0, 0, 0 e `sementes`; a original empurra os words com lixo nos 16 bits altos, que o construtor não
/// lê) em [+0xc][fase]; o vetor e a tabela das rodadas liberados (0x9452ca, cdecl; nesta ordem). Sem fase, a caixa da
/// `linha`; com ela, o sorteio da primeira rodada (Cup::draw_round 0x51c410: push 0; mov ecx, fase -> thiscall, ret 4).
pub(crate) unsafe fn create_knockout_stage(this: usize, n: u32, teams: usize, stage: u32, seeds: *const u8, file: usize, row: u32) {
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this, stage, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n, teams, rodadas as u16 as u32, read_usize(this + 4), table, read_i16(this + league::YEAR) as u16 as u32, stage, 1, extra as u16 as u32, 0x14, 0, 0, 0, seeds)
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES) + stage as usize * 4, f);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES) + stage as usize * 4);
    if f == 0 {
        return show_error_at(file, row);
    }
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    sortear(f, 0);
}

/// ItaCup::create_knockout_stage(): builds the 24-team knockout stage (stage 7) of the Italian Cup from the 8 group
/// winners (top row of each group table) followed by the 16 seeded clubs that enter directly, with seed 3 for the group
/// winners and 2/1 for the direct entrants [inference: the draw pot of each slot]. Out of memory shows the error box
/// and returns.
///
/// [pt] { 0x63f870 } (thiscall, ret 0, ita_cup.cpp; da fase seguinte, depois da fase 6; quadro SEH só para o new, sem
/// destrutor): `ItaCup::create_knockout_stage()`: o mata-mata da Copa da Itália com 24 times: um vetor de 24 ponteiros
/// (0x944e46 de 0x60; sem memória, a caixa da linha 0x29c) com os 8 campeões dos grupos (a primeira linha da tabela de
/// cada grupo, `group_table`) e, nas vagas 8 a 23, os 16 cabeças de chave que entram direto (`take_direct_entrants`); a fase 7
/// (`create_knockout_stage`) com as sementes {3 × 8, 2 × 8, 1 × 8} (os campeões dos grupos, depois os cabeças de chave
/// [inferência: o pote de cada vaga no sorteio]) e a caixa da linha 0x2c0.
pub(crate) unsafe extern "thiscall" fn ita_cup_create_knockout_stage(this: usize) {
    let mut seeds = [0u8; 24];
    for (k, s) in seeds.iter_mut().enumerate() {
        *s = 3 - (k / 8) as u8;
    }
    let teams = exe::crt_malloc(0x60);
    if teams == 0 {
        return show_error_at(0x9ea23c, 0x29c);
    }
    crate::comparison::branch("cqr_9 mata-mata da copa da italia");
    for i in 0..8usize {
        write_usize(teams + i * 4, read_usize(group_table(this, i))); // cqr9: the winner of group i
    }
    take_direct_entrants(this, teams, 8, 0x18);
    create_knockout_stage(this, 0x18, teams, 7, seeds.as_ptr(), 0x9ea23c, 0x2c0);
}

/// Comparer of the knockout creation: after the original, checks the stage in [+0xc][fase] and the planned entrants
/// cleared from +0xee.
///
/// [pt] O plano dos dois mata-matas (feito antes): os campeões dos `grupos` grupos e os primeiros inscritos não nulos de +0xee
/// que cabem; o comparador confere, depois da original, a fase em [+0xc][fase] e esses inscritos zerados em +0xee.
pub(crate) unsafe fn ita_cup_compare_knockout_stage(name: &str, this: usize, stage: usize, cabem: usize, original: impl FnOnce()) {
    let mut slots = Vec::new();
    let mut j = 0usize;
    while slots.len() < cabem && (j as i32) < read_usize(this + 0xf2) as i32 {
        if read_usize(read_usize(this + 0xee) + j * 4) != 0 {
            slots.push(j);
        }
        j += 1;
    }
    original();
    let f = read_usize(read_usize(this + league::STAGES) + stage * 4);
    let ok = f != 0 && slots.iter().all(|&j| read_usize(read_usize(this + 0xee) + j * 4) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}) fase {:#x}, vagas tiradas {:?}", name, this, f, slots));
}

/// `ItaCCup::create_knockout_stage()` (`0x63dc10`, thiscall, ret 0, `ita_c_cup.cpp`): builds the 32-team knockout of
/// the Italian Serie C cup.
///
/// Called from the next-stage routine, after stage 14; SEH frame only for the two `new` calls, with no destructor on
/// the normal path.
///
/// 1) Ranking of the runners-up: a temporary league (`0x944cf1(0xee)`; League_Stage `0x68b100`, thiscall, ret 0x58: the
/// cup, 16, 0, 0, `+4`, 0, 0, `+0xc2`, `+0xc3`, 0, &{1, 2, 3, 0} (the tie-breakers), &`+0xbe`, the year `+0x40` (word),
/// −2, 0, 0, 1, 0, 2, −1, 0, 2) whose table receives, in row i, the second row (`+0x3b`) of group i's table (the 0x3b
/// bytes copied) with `+5` and `+9` zeroed, then sorted (League::sort_table `0x6827d0`: push 0; mov ecx -> thiscall,
/// ret 4).
///
/// 2) A vector of 32 pointers (`0x944e46` of 0x80; out of memory, the box of line `0x2ba`) with the 16 group winners
/// (the first row of each table) and, in slots 16 to 21, the 6 best runners-up (the first 6 rows of the ranking): each
/// one, found in its group's table (the first group in which it appears), marks the group (`+0xbe` = 2) and its row
/// (`+0x37` = 6).
///
/// 3) The temporary league is destroyed (virtual 0 with 1).
///
/// 4) In slots 22 to 31, the 10 seeded clubs that enter directly (`take_direct_entrants`).
///
/// 5) Stage 15 (`create_knockout_stage`), with no seeds, and the box of line `0x304`.
///
/// [pt] { 0x63dc10 } (thiscall, ret 0, ita_c_cup.cpp; da fase seguinte, depois da fase 14; quadro SEH só para os dois new, sem
/// destrutor no caminho normal): `ItaCCup::create_knockout_stage()`: o mata-mata da Copa da Itália da Série C com 32
/// times. 1) A classificação dos segundos colocados: uma liga provisória (0x944cf1(0xee); League_Stage 0x68b100,
/// thiscall, ret 0x58: a copa, 16, 0, 0, +4, 0, 0, +0xc2, +0xc3, 0, &{1, 2, 3, 0} (os critérios de desempate), &+0xbe, o
/// ano +0x40 (word), −2, 0, 0, 1, 0, 2, −1, 0, 2) cuja tabela recebe, na linha i, a segunda linha (+0x3b) da tabela do
/// grupo i (os 0x3b bytes copiados) com +5 e +9 zerados, ordenada (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall,
/// ret 4). 2) Um vetor de 32 ponteiros (0x944e46 de 0x80; sem memória, a caixa da linha 0x2ba) com os 16 campeões dos
/// grupos (a primeira linha de cada tabela) e, nas vagas 16 a 21, os 6 melhores segundos (as 6 primeiras linhas da
/// classificação): cada um, achado na tabela do seu grupo (o primeiro grupo em que aparece), marca o grupo (+0xbe = 2) e a
/// linha dele (+0x37 = 6). 3) A liga provisória destruída (a virtual 0 com 1). 4) Nas vagas 22 a 31, os 10 cabeças de chave
/// que entram direto (`take_direct_entrants`). 5) A fase 15 (`create_knockout_stage`), sem sementes, e a caixa da linha 0x304.
pub(crate) unsafe extern "thiscall" fn ita_cup_create_knockout_stage_63dc10(this: usize) {
    let desempate: [u8; 4] = [1, 2, 3, 0];
    let p = exe::crt_operator_new(0xee);
    let league = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        construct(p, this, 0x10, 0, 0, read_usize(this + 4), 0, 0, *((this + league::WIN_POINTS) as *const u8) as u32, *((this + league::DRAW_POINTS) as *const u8) as u32, 0,
            desempate.as_ptr() as usize, this + league::PROMOTED_COUNT, read_i16(this + league::YEAR) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for i in 0..16usize {
        let de = group_table(this, i) + 0x3b;
        let para = read_usize(league + league::TABLE) + i * 0x3b;
        core::ptr::copy_nonoverlapping(de as *const u8, para as *mut u8, 0x3b); // cqr9: the runner-up of group i
        write_usize(read_usize(league + league::TABLE) + i * 0x3b + 9, 0);
        write_usize(read_usize(league + league::TABLE) + i * 0x3b + 5, 0);
    }
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    sort(league, 0); // cqr9: the ranking of the runners-up
    let teams = exe::crt_malloc(0x80);
    if teams == 0 {
        return show_error_at(0x9ea204, 0x2ba);
    }
    crate::comparison::branch("cqr_9 mata-mata da copa da serie c");
    for i in 0..16usize {
        write_usize(teams + i * 4, read_usize(group_table(this, i)));
    }
    let mut l = 0usize;
    while l < 0x162 {
        let c = read_usize(read_usize(league + league::TABLE) + l);
        write_usize(teams + 0x40 + (l / 0x3b) * 4, c); // cqr9: os 6 melhores segundos
        let mut g = 0usize;
        while g < 0x10 {
            let group = if g == 0 { this } else { read_usize(read_usize(this + league::STAGES) + (g - 1) * 4) };
            let n = read_i16(group + league::CLUB_COUNT) as i32;
            let mut r = 0i32;
            while r < n {
                if read_usize(read_usize(group + league::TABLE) + r as usize * 0x3b) == read_usize(read_usize(league + league::TABLE) + l) {
                    *((group + league::PROMOTED_COUNT) as *mut u8) = 2;
                    *((read_usize(group + league::TABLE) + r as usize * 0x3b + 0x37) as *mut u8) = 6;
                    g = 0x10;
                    break;
                }
                r += 1;
            }
            g += 1;
        }
        l += 0x3b;
    }
    if league != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(league)));
        destroy(league, 1);
    }
    take_direct_entrants(this, teams, 0x16, 0x20);
    create_knockout_stage(this, 0x20, teams, 0xf, core::ptr::null(), 0x9ea204, 0x304);
}

/// Writes one group round (a 0x41-byte record, like the league rounds).
///
/// The date goes through 0x68a160 (day, month with 0 = January, weekday, code), with no years to add and value 0; and
/// match 0 is left empty (0x68a1c0 with −1, −1, −1 and value 0).
///
/// [pt] Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
pub(crate) type ItaCupGroupRound = (u32, u32, u32, u32);

/// The two stages of the Italian cups (virtual +0x3c, `create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret
/// 0x10; the stage is read as a signed byte): returns the round table or 0.
///
/// Below `limite` (−1 included), the groups: é_liga (if requested) = 1, rodadas = the number of group rounds, extra =
/// stage + `id` (16 bits); a table of 0x41 bytes per round (0x944e46; out of memory, the error box `linha_g` and
/// returns 0) with the dates from `grupos`.
///
/// At stage `limite`, the knockout: é_liga = 0, rodadas = the size of `mata`, extra = 0; a table of 0x68 bytes per
/// round (out of memory, the error box `linha_m`), the rounds written by `eng_cc_cup_write_cup_round` with the cup's
/// year +0x40 (signed).
///
/// Above `limite`: returns 0 without writing anything.
///
/// [pt] As duas fases das copas italianas (a virtual +0x3c, `create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret
/// 0x10; a fase lida como byte com sinal). Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1, rodadas
/// = o número de rodadas de grupo, extra = fase + `id` (16 bits); a tabela de 0x41 bytes por rodada (0x944e46; sem
/// memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0, rodadas =
/// o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as rodadas pela
/// `eng_cc_cup_write_cup_round` com o ano +0x40 da copa (com sinal). Acima de `limite`: devolve 0 sem gravar nada.
pub(crate) unsafe fn create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, id: i16, groups: &[ItaCupGroupRound],
    knockout: &[EngCcCupCupRound], file: usize, row_g: u32, row_m: u32, name_g: &'static str, name: &'static str) -> usize {
    let f = stage as u8 as i8;
    if f < limit {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = groups.len() as i16;
        *extra = (f as i16).wrapping_add(id);
        let t = exe::crt_malloc(groups.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(file, row_g);
            return 0;
        }
        crate::comparison::branch(name_g);
        // the year goes with garbage in the high bytes in the original (the previous register with the word +0x40 over it); the
        // 0x68a160 only uses the low 16 bits: the copy passes the clean year
        let year = core::ptr::read_unaligned((this + league::YEAR) as *const u16) as u32;
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
    let year = core::ptr::read_unaligned((this + league::YEAR) as *const i16) as i32 as u32;
    for (i, r) in knockout.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// The bytes the original writes in a group schedule table of n 0x41-byte rounds (date/code, first match, value), for
/// comparison.
///
/// [pt] Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
pub(crate) unsafe fn written_group_schedule_bytes(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Side-by-side comparer of create_schedule.
///
/// The original routine runs with the caller's outputs, the copy with its own outputs that start with the same values,
/// both from the same [0xb67a34]; the outputs and the bytes written to the two tables (group or knockout tables,
/// according to the copy's é_liga) must be equal; the copy's table is freed (0x9452ca).
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (de grupos ou de mata-mata, pelo é_liga
/// da cópia) têm de ser iguais; a tabela da cópia é liberada (0x9452ca).
pub(crate) unsafe fn ita_cup_compare_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize, limit: i8) -> usize {
    if stage as u8 as i8 > limit {
        // simplification: above the knockout neither of the two writes anything; the outputs may not even exist
        let a = o();
        SCOREBOARD.record(a == 0, || format!("{}(fase {}) devolveu {:#x}", name, stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, &mut e2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let groups = (stage as u8 as i8) < limit;
    let table = if a != 0 && b != 0 {
        if groups { written_group_schedule_bytes(a, *rounds) == written_group_schedule_bytes(b, r2) } else { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Copa da Itália, grupos: três rodadas em agosto (12 e 19, sábado [inferência: dia da semana 6], e 29, terça [2]).
pub(crate) const ITALIAN_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 3] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x1d, 7, 2, 2)];

/// A Copa da Itália, mata-mata: cinco rodadas em ida e volta, de setembro a março do ano seguinte.
pub(crate) const ITALIAN_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x1e, 7, 0, 3), (0x13, 8, 0, 2, 2, 0), 0xa, (0, 0x400, 3), 4, (0x10, 8, 0x10, 0), (0, 2, 0x23), (0, 0xffff_ffff, 0)), // cqo10: the 1st knockout round of the Coppa Italia
    cql10r((0x19, 9, 0, 3), (0xb, 0xa, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 4, (0x10, 8, 8, 0x10), (0, 2, 0x11), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 0xa, 0, 3), (0xc, 0xb, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0x1c), (0, 0xffff_ffff, 0)),
    cql10r((0xa, 0, 1, 3), (0x17, 0, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((7, 1, 1, 3), (6, 2, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 0x39), (0, 0xffff_ffff, 0)),
];

/// rule: the Coppa Italia starts with three-round groups in August (12 and 19, Saturday [inference: weekday 6], and 29,
/// Tuesday), each group a league stage (is_league = 1, id 0x3e9 + stage); then the knockout (stage 7) over two legs:
/// 1st round drawn on 30/8 and played on 19/9 (16 teams from the groups and 16 new), 2nd draw 25/10, match 11/11 (8
/// new), quarter-finals 29/11 and 12/12, semi-final 10/1 and 23/1, final draw 7/2 and legs on 6/3 and 57 days later;
/// category 4 up to the 2nd round and 6 after; no per-round prize (−1)
/// Original: `ItaCup::create_schedule` (0x63ec50, ita_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63ec50 } (thiscall, ret 0x10, ita_cup.cpp; a virtual +0x3c da Copa da Itália): `ItaCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (extra = fase + 0x3e9; a caixa da linha 0xc1), o mata-mata na
/// fase 7 (a caixa da linha 0xe1).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(this, stage, rounds, extra, league, 7, 0x3e9, &ITALIAN_CUP_GROUP_ROUNDS, &ITALIAN_CUP_KNOCKOUT_ROUNDS, 0x9ea23c, 0xc1, 0xe1,
        "cqo_10 grupos da copa da italia", "cqo_10 mata-mata da copa da italia")
}

/// A Copa da Itália da Série C, grupos: cinco rodadas em agosto (12, 19, 22, 26 e 29).
pub(crate) const SERIE_C_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 5] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x16, 7, 2, 2), (0x1a, 7, 6, 1), (0x1d, 7, 2, 2)]; // cqo10: the Serie C Cup groups

/// A Copa da Itália da Série C, mata-mata: cinco rodadas em ida e volta, de setembro a maio do ano seguinte (as
/// semifinais em abril).
pub(crate) const SERIE_C_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x1e, 7, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x14, (0, 0, 3), 4, (0x20, 0x10, 0x20, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 9, 0, 3), (1, 0xa, 0, 2, 2, 0), 0x1e, (0, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0xf, 0xa, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0xa, 0, 3), (0xb, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
];

/// A Supercopa da Itália: um jogo único em 19 de agosto (sorteio em 5 de julho), mando neutro [inferência: o +0x13 da
/// data fica 0], sem prêmios.
pub(crate) const ITALIAN_SUPER_CUP: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (0x13, 7, 0, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqo10: the Italian Super Cup
];

/// A Supercopa da Série C1: um jogo único em 21 de maio do ano seguinte (sorteio em 15 de maio), sem prêmios.
pub(crate) const SERIE_C1_SUPER_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((0xf, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)),
];

/// Clubs of G_CLUB_TABLE the pass looks at: G_CLUB_COUNT - 2 × G_NATION_COUNT (without national teams [inference]).
///
/// [pt] Os clubes da tabela de clubes ([0xae23b4], 0x245 bytes) que a passada olha: [0xae235c] − 2 × [0xae2350] (sem as
/// seleções [inferência]).
pub(crate) unsafe fn ita_cup_club_count_without_nations() -> i32 {
    (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2))
}

/// True when the club's nation (+0x53) is Italy (KEY_NATION_ITALY).
///
/// [pt] O clube é da Itália (+0x53 a nação; [0x9cf360] o id da Itália).
pub(crate) unsafe fn is_italian_club(c: usize) -> bool {
    comp_id_at(c, 0x53) == Some(read_usize(KEY_NATION_ITALY))
}

/// Club's previous-season league position (+0x5f, signed byte; 0 = champion [inference]).
///
/// [pt] A posição na temporada anterior (+0x5f, byte com sinal; 0 o campeão [inferência]).
pub(crate) unsafe fn last_season_position(c: usize) -> i8 {
    *((c + 0x5f) as *const i8)
}

/// Shuffles n pointers from v (CompUtil shuffle_pointer_array, cdecl).
///
/// [pt] Embaralha `n` ponteiros a partir de `v` (0x4afb40: push n; push v; add esp junto -> cdecl; já em Rust).
pub(crate) unsafe fn shuffle_pointers(v: usize, n: u32) {
    exe::comp_util_shuffle_pointer_array(v, n);
}

/// qsort of n pointers from v with comparison cmp (CRT qsort).
///
/// [pt] O qsort de `n` ponteiros a partir de `v` com a comparação `cmp` (0x945333: 4 pushes; add esp, 0x10 -> cdecl).
pub(crate) unsafe fn sort_pointers(v: usize, n: u32, cmp: usize) {
    exe::crt_qsort(v, n, 4, cmp);
}

/// Allocates the entrant pointer list (+0xee) with max entries in +0xf2; on failure shows the error box for linha and
/// returns None.
///
/// [pt] A lista de ponteiros (+0xee) com o máximo `max` em +0xf2 (dword): 0x944e46 de 4 × max; sem memória, a caixa `linha`.
pub(crate) unsafe fn alloc_entrant_list(this: usize, max: u32, file: usize, row: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, max);
    let l = exe::crt_malloc(max * 4);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(file, row);
        return None;
    }
    Some(l)
}

/// Appends club c to the entrant list (+0xee); true when the count reaches the max (+0xf2).
///
/// [pt] Põe `c` no fim da lista (+0xee) e devolve verdadeiro quando a conta chega ao máximo +0xf2 (dword).
pub(crate) unsafe fn push_entrant(this: usize, n: &mut i32, c: usize) -> bool {
    *n += 1;
    core::ptr::write_unaligned((read_usize(this + 0xee) + (*n as usize - 1) * 4) as *mut u32, c as u32);
    *n == read_usize(this + 0xf2) as i32
}

/// `ItaCup::select_entrants()` (0x63faf0, thiscall, ret 0, ita_cup.cpp): the 48 Coppa Italia entrants (+0xf2 = 0x30;
/// the list of 0xc0 bytes at +0xee; out of memory, the box for line 0x2e1 and 0). Returns 1.
///
/// The 32 for the groups go into a local vector: the Italian clubs of the season (single pass) that did not come from
/// Serie B ([0x9cf574]) among the top 3 nor from Serie A ([0x9cf570]) among the top 15: those in the current Serie B,
/// those that came from Serie B in 4th, 5th or below 17th, and those that came from C1 ([0x9cf578]/[0x9cf57c]) in 4th,
/// 5th or 6th; if short, those of C1 outside Serie B by position (6th, 7th...).
///
/// The vector goes to qsort (0x640590) and is split into 4 pots of 8: 8 passes, each takes one randomly drawn club from
/// each pot (0x90cfc0(8), moving to the next in the pot if it was already taken).
///
/// Then the top 3 of the previous Serie B and those of the previous Serie A from 10th to 15th (until reaching 48) and,
/// without checking first, the top 9 of the previous Serie A.
///
/// [pt] { 0x63faf0 } (thiscall, ret 0, ita_cup.cpp): `ItaCup::select_entrants()`: os 48 inscritos da Copa da Itália (+0xf2 =
/// 0x30; a lista de 0xc0 bytes em +0xee; sem memória, a caixa da linha 0x2e1 e 0). Os 32 dos grupos num vetor local:
/// os clubes italianos da temporada (passada única) que não vieram da Série B ([0x9cf574]) entre os 3 primeiros nem da
/// Série A ([0x9cf570]) entre os 15 primeiros: os da Série B atual, os que vieram da Série B em 4º, 5º ou abaixo do 17º,
/// e os que vieram da C1 ([0x9cf578]/[0x9cf57c]) em 4º, 5º ou 6º; faltando, os da C1 fora da Série B pela posição (6º,
/// 7º...). O vetor vai ao qsort (0x640590) e é repartido em 4 potes de 8: 8 voltas, cada uma tira um clube sorteado de
/// cada pote (0x90cfc0(8), seguindo para o próximo do pote se já saiu). Depois os 3 primeiros da Série B anterior e os
/// da Série A anterior do 10º ao 15º (até completar 48) e, sem conferir antes, os 9 primeiros da Série A anterior.
/// Devolve 1.
pub(crate) unsafe fn select_coppa_italia_entrants(this: usize) -> (i32, i32) {
    let l = match alloc_entrant_list(this, 0x30, 0x9ea23c, 0x2e1) {
        Some(l) => l,
        None => return (0, 0),
    };
    let (serie_a, serie_b, c1a, c1b) = (read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut v = [0usize; 32];
    let mut k = 0usize;
    let n = ita_cup_club_count_without_nations();
    'grupos: {
        let mut i = 0;
        while i < n {
            let c = jap_j_cup_club_by_index(i);
            i += 1;
            if !is_italian_club(c) {
                continue;
            }
            let before = comp_id_at(c, 0x5b);
            if before == Some(serie_b) && last_season_position(c) < 3 || before == Some(serie_a) && last_season_position(c) < 0xf {
                continue;
            }
            if comp_id_at(c, 0x57) == Some(serie_b) {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
                continue;
            }
            let p = last_season_position(c);
            let enters = match before {
                None => false,
                Some(x) if x == serie_b => p == 3 || p == 4 || p > 0x10,
                Some(x) => (x == c1a || x == c1b) && (p == 3 || p == 4 || p == 5),
            };
            if enters {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
            }
        }
        let mut place = 5i8;
        while k < 32 {
            let mut i = 0;
            while i < n {
                let c = jap_j_cup_club_by_index(i);
                i += 1;
                if !is_italian_club(c) {
                    continue;
                }
                let now = comp_id_at(c, 0x57);
                if now.is_none() || now == Some(serie_b) {
                    continue;
                }
                let before = comp_id_at(c, 0x5b);
                if (before == Some(c1a) || before == Some(c1b)) && last_season_position(c) == place {
                    crate::comparison::branch("cqo_11 completar com a c1");
                    v[k] = c;
                    k += 1;
                    if k == 32 {
                        break;
                    }
                }
            }
            place = place.wrapping_add(1);
        }
    }
    sort_pointers(v.as_mut_ptr() as usize, k as u32, 0x640590);
    let mut m = 0i32;
    for _ in 0..8 {
        let mut pote = 0usize;
        while pote < 32 {
            crate::comparison::branch("cqo_11 potes da copa da italia");
            let mut r = jap_j_cup_random(8) as i8 as i32; // cqo11: the club drawn from each pot
            while v[pote + r as usize] == 0 {
                r += 1;
                if r == 8 {
                    r = 0;
                }
            }
            m += 1;
            core::ptr::write_unaligned((l + (m as usize - 1) * 4) as *mut u32, v[pote + r as usize] as u32);
            v[pote + r as usize] = 0;
            pote += 8;
        }
    }
    'b: {
        let mut i = 0;
        while i < ita_cup_club_count_without_nations() {
            let c = jap_j_cup_club_by_index(i);
            i += 1;
            if !is_italian_club(c) {
                continue;
            }
            let before = comp_id_at(c, 0x5b);
            let p = last_season_position(c);
            if before == Some(serie_b) && p < 3 || before == Some(serie_a) && p > 8 && p < 0xf { // cqo11: those entering in the 2nd round
                if push_entrant(this, &mut m, c) {
                    break 'b;
                }
            }
        }
    }
    let mut i = 0;
    while i < ita_cup_club_count_without_nations() {
        let c = jap_j_cup_club_by_index(i);
        i += 1;
        if is_italian_club(c) && comp_id_at(c, 0x5b) == Some(serie_a) && last_season_position(c) < 9 {
            if push_entrant(this, &mut m, c) {
                break;
            }
        }
    }
    (1, m)
}

/// rule: 48 Italian clubs enter the Coppa Italia. The 32 of the group stage: the season's Serie B clubs, those that
/// came from Serie B in 4th, 5th or from 18th down and those that came from C1 in 4th, 5th or 6th (positions counted
/// from 0 [inference]); if short, C1 clubs by position (6th, 7th...). The 32 are sorted (qsort 0x640590) and split into
/// 4 pots of 8, and each of the 8 rounds draws one club from each pot. In the 2nd round enter the top 3 of last
/// season's Serie B (the promoted) and the Serie A clubs from 10th to 15th; last, the top 9 of Serie A, up to 48 (the
/// rest are left out by table order)
/// Original: `ItaCup::select_entrants` (0x63faf0, ita_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Itália 48 clubes italianos. Os 32 da fase de grupos: os da Série B da temporada, os que
/// vieram da Série B em 4º, 5º ou do 18º para baixo e os que vieram da C1 em 4º, 5º ou 6º (posições contadas de 0
/// [inferência]); faltando, os da C1 pela posição (6º, 7º...). Os 32 são ordenados (qsort 0x640590) e repartidos em 4
/// potes de 8, e cada uma das 8 voltas sorteia um clube de cada pote. Na 2ª rodada entram os 3 primeiros da Série B
/// anterior (os que subiram) e os da Série A do 10º ao 15º; por último os 9 primeiros da Série A, até completar 48 (os
/// que passam disso ficam de fora pela ordem da tabela)
pub(crate) unsafe extern "thiscall" fn ita_cup_select_entrants(this: usize) -> i32 {
    select_coppa_italia_entrants(this).0
}

/// Side-by-side comparer for entrant selection routines that allocate +0xee and draw: compares +0xf2, 0xb67a34, RNG
/// state and written entrants; frees the copy's list.
///
/// [pt] Lado a lado para as rotinas que alocam a lista de ponteiros +0xee e sorteiam: o +0xf2, o [0xb67a34] e o estado do
/// sorteador a partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xee, a cópia faz
/// a sua, comparada nos inscritos que ela gravou e liberada.
pub(crate) unsafe fn compare_select_entrants(name: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + 0xf2, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), a_[0], a_[1]];
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(read_usize(this + 0xee));
        r
    }, || {
        let (r, n) = crate::rng::without_counting(c);
        lr.set(read_usize(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    let ver = |l: usize| if l == 0 { Vec::new() } else { core::slice::from_raw_parts(l as *const u32, k / 4).to_vec() };
    let (vo, vr) = (ver(lo), ver(lr));
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) devolveu {} x {} memória {} {:x?} x {:x?} lista {} ({} bytes) {:x?} x {:x?}", name, this, a, b, igual, mo, mr, list, k, vo, vr));
    a
}

/// `ItaCCup::select_entrants()` (0x63dfe0, thiscall, ret 0, ita_c_cup.cpp): the 90 entrants of the Serie C Cup from C1
/// and C2, reordered, shuffled and sorted by longitude into groups.
///
/// Sets +0xf2 = 0x5a and allocates the list of 0x168 bytes at +0xee (when out of memory, the error box for line 0x322
/// and 0). It takes the Italian clubs of C1 and C2 ([0x9cf578]..[0x9cf588]) in table order, up to 90.
///
/// Of the first 80, those relegated from Serie B ([0x9cf574]) and those from C1 that finished 4th, 5th or 6th go, one
/// by one, to slots 80, 81... (swapped with whatever was there).
///
/// The first 80 are shuffled (0x4afb40), sorted by longitude (qsort with 0x53fe50) and each of their 8 groups of ten is
/// shuffled (the regional groups [inference]); the last 10 slots stay as they are. Returns 1.
///
/// [pt] { 0x63dfe0 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::select_entrants()`: os 90 inscritos da Copa da Série C (+0xf2
/// = 0x5a; a lista de 0x168 bytes em +0xee; sem memória, a caixa da linha 0x322 e 0): os clubes italianos da C1 e da C2
/// ([0x9cf578]..[0x9cf588]) pela ordem da tabela, até 90. Dos 80 primeiros, os que vieram da Série B ([0x9cf574]) e os
/// que vieram da C1 em 4º, 5º ou 6º vão, um a um, para as vagas 80, 81... (trocados com o que estava lá). Os 80 primeiros
/// são embaralhados (0x4afb40), ordenados pela longitude (qsort com 0x53fe50) e cada uma das 8 dezenas deles
/// embaralhada (os grupos por região [inferência]); as 10 últimas vagas ficam como estão. Devolve 1.
pub(crate) unsafe fn select_serie_c_cup_entrants(this: usize) -> (i32, i32) {
    let l = match alloc_entrant_list(this, 0x5a, 0x9ea204, 0x322) {
        Some(l) => l,
        None => return (0, 0),
    };
    let divisoes = [read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_B), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_C)];
    let mut m = 0i32;
    let mut i = 0;
    while i < ita_cup_club_count_without_nations() {
        let c = jap_j_cup_club_by_index(i);
        i += 1;
        if !is_italian_club(c) {
            continue;
        }
        match comp_id_at(c, 0x57) {
            Some(d) if divisoes.contains(&d) => {
                if push_entrant(this, &mut m, c) {
                    break;
                }
            }
            _ => {}
        }
    }
    let (serie_b, c1a, c1b) = (read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut j = 0x50i32;
    for idx in 0..0x50usize {
        let c = read_usize(l + idx * 4);
        let before = match comp_id_at(c, 0x5b) {
            Some(x) => x,
            None => continue,
        };
        let p = last_season_position(c);
        if before == serie_b || (before == c1a || before == c1b) && (p == 3 || p == 4 || p == 5) {
            crate::comparison::branch("cqo_11 vagas do fim da copa da serie c");
            let e = l + (j as i16 as isize * 4) as usize;
            let t = read_usize(e);
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            j += 1;
            core::ptr::write_unaligned((l + idx * 4) as *mut u32, t as u32);
            if j as i16 as i32 >= read_usize(this + 0xf2) as i32 {
                break;
            }
        }
    }
    shuffle_pointers(l, 0x50);
    sort_pointers(l, 0x50, 0x53fe50);
    crate::comparison::branch("cqo_11 dezenas da copa da serie c");
    for k in 0..8usize {
        shuffle_pointers(l + k * 0x28, 0xa); // cqo11: the tens of the Serie C Cup
    }
    (1, m)
}

/// `ItaSuper::init_team_list()` (0x65fa60, thiscall, ret 0, ita_super.cpp): the two Supercoppa teams.
///
/// A two-entry list (+0x3e = 2, 0xc bytes at +0xa7; out of memory, the error box for line 0xc2 and −1). The Serie A
/// champion ([0x9cf570]; if missing, error box 0xcb and 0) and the Coppa Italia winner ([0x9cf71c]; if missing, 0xd4
/// and 0); if they are the same club, the second becomes the cup runner-up (if missing, 0xe0 and 0). Class and status 0
/// for both; returns 1.
///
/// [pt] { 0x65fa60 } (thiscall, ret 0, ita_super.cpp): `ItaSuper::init_team_list()`: a lista de dois (+0x3e = 2, 0xc bytes
/// em +0xa7; sem memória, a caixa da linha 0xc2 e −1). O campeão da Série A ([0x9cf570]; sem ele, a caixa 0xcb e 0) e o
/// da Copa da Itália ([0x9cf71c]; sem ele, 0xd4 e 0); se é o mesmo clube, o segundo vira o vice da Copa (sem ele, 0xe0
/// e 0). Classe e situação 0 nos dois; devolve 1.
pub(crate) unsafe fn init_supercoppa_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x9ea934;
    let l = match alloc_two_team_list(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_ITALIAN_SERIE_A, 0); // cqo11: the Serie A champion
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_ITALIAN_CUP, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparison::branch("cqo_11 dobradinha italiana");
        if !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_ITALIAN_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparison::branch("cqo_11 clubes da supercopa da italia");
    finish_two_team_list(l)
}

/// The rules that virtual `+0x8c` (set_up_rules) of a cup with a group stage writes into the group's league, before the
/// round table.
///
/// Tie-breakers: the four codes at `+0xc5`..`+0xc8` (`docs/.md` §2.7).
///
/// [pt] As regras que a virtual +0x8c (set_up_rules) de uma copa com fase de grupos grava na liga do grupo, antes da tabela
/// das rodadas. Desempates: os quatro códigos de +0xc5..+0xc8 (docs/.md §2.7).
#[derive(Clone, Copy)]
pub(crate) struct GroupStageRules {
    pub(crate) vitoria: u8,     // +0xc2: points per win
    pub(crate) empate: u8,      // +0xc3: pontos por empate
    pub(crate) categoria: u8,   // +0xc4: the category of each match (goes to the match’s +0x3a)
    pub(crate) desempate: [u8; 4], // +0xc5..+0xc8
    pub(crate) vagas: u8,       // +0xbe: top places (qualified per group)
    pub(crate) premio: Option<u32>, // +0x28: the prize fund (the Champions Cup does not write it)
    pub(crate) bits: Option<u16>,   // +0xd9: the rule bit word (only the Champions Cup writes it: 8)
}

pub(crate) const ITALIAN_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0xc3500), bits: None };
pub(crate) const SERIE_C_CUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0x493e0), bits: None };
pub(crate) const CHAMPIONS_CUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 0x17, desempate: [3, 1, 2, 4], vagas: 2, premio: None, bits: Some(8) };

/// Writes rules `g` into league `this` and builds its round table; returns 1.
///
/// Promotion/relegation targets set to none (+0x1c, +0x20 = −1), one leg (+0x3c = 1), type 1 (+0x42, clubs), no
/// play-offs and no relegation (+0xbf, +0xc0, +0xc1 = 0); then the round table from virtual +0x3c (push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) stored at +0xba.
///
/// [pt] Grava as regras `g` na liga `this`: sobe/cai para nenhuma (+0x1c, +0x20 = −1), um turno (+0x3c = 1), o tipo 1
/// (+0x42, clubes), sem play-offs nem rebaixamento (+0xbf, +0xc0, +0xc1 = 0); depois a tabela das rodadas da virtual
/// +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
pub(crate) unsafe fn set_cup_league_rules(this: usize, g: &GroupStageRules, turnos: i16) -> u32 {
    let b = |o: usize, v: u8| *((this + o) as *mut u8) = v;
    b(0xc2, g.vitoria);
    b(0xc3, g.empate);
    b(0xc4, g.categoria);
    for (k, &v) in g.desempate.iter().enumerate() {
        b(0xc5 + k, v);
    }
    b(0xbe, g.vagas);
    b(0xbf, 0);
    b(0xc0, 0);
    b(0xc1, 0);
    b(0x42, 1);
    core::ptr::write_unaligned((this + league::UPPER_DIVISION_ID) as *mut i32, -1);
    core::ptr::write_unaligned((this + league::LOWER_DIVISION_ID) as *mut i32, -1);
    core::ptr::write_unaligned((this + league::LEGS) as *mut i16, turnos);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + league::PRIZE_FUND) as *mut u32, p);
    }
    if let Some(w) = g.bits {
        core::ptr::write_unaligned((this + league::LEAGUE_FLAGS) as *mut u16, w);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = table(this, 0xffff_ffff, this + league::ROUND_COUNT, this + league::STAGE_ID, 0);
    core::ptr::write_unaligned((this + league::ROUND_DATES) as *mut u32, t as u32);
    1
}

/// Checks after the original that the fields set_cup_league_rules writes have the planned values.
///
/// [pt] O plano: os campos que `set_cup_league_rules` grava, lidos depois da original.
pub(crate) unsafe fn check_cup_league_rules(nome: &str, this: usize, g: &GroupStageRules, r: u32) {
    let lido = (read_u8(this + league::WIN_POINTS), read_u8(this + league::DRAW_POINTS), read_u8(this + league::MATCH_CATEGORY), [read_u8(this + league::TIEBREAKS), read_u8(this + 0xc6), read_u8(this + 0xc7), read_u8(this + 0xc8)], read_u8(this + league::PROMOTED_COUNT), [read_u8(this + league::PROMOTION_PLAYOFF_COUNT), read_u8(this + league::RELEGATION_PLAYOFF_COUNT), read_u8(this + league::RELEGATED_COUNT)], read_u8(this + league::CLUB_SIDES),
        core::ptr::read_unaligned((this + league::UPPER_DIVISION_ID) as *const i32), core::ptr::read_unaligned((this + league::LOWER_DIVISION_ID) as *const i32),
        core::ptr::read_unaligned((this + league::LEGS) as *const i16));
    let previsto = (g.vitoria, g.empate, g.categoria, g.desempate, g.vagas, [0u8; 3], 1u8, -1i32, -1i32, 1i16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + league::PRIZE_FUND) as *const u32) == p);
    let bits = g.bits.map_or(true, |w| core::ptr::read_unaligned((this + league::LEAGUE_FLAGS) as *const u16) == w);
    SCOREBOARD.record(r == 1 && lido == previsto && premio && bits, || format!("{}({:#x}) devolveu {} regras {:?} previstas {:?} prêmio ok {} bits ok {}", nome, this, r, lido, previsto, premio, bits));
}

/// rule: the Italian Cup groups are single round robin, 3 points a win and 1 a draw; tiebreak by codes 4, 1 and 2
/// [inference: head-to-head, goal difference, goals for]; one qualifier per group (the winner); category 4 (national
/// cup) on each match; no promotion or relegation; prize fund of 800,000
/// Original: `ItaCup::set_up_rules` (0x63f350, ita_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x63f350 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x8c): `ItaCup::set_up_rules()`: os grupos da Copa da Itália
/// em turno único, 3 pontos a vitória e 1 o empate, a categoria 4 (copa nacional), o desempate (4, 1, 2), um
/// classificado por grupo e o fundo de prêmios de 800.000; depois a tabela das rodadas.
pub(crate) unsafe extern "thiscall" fn ita_cup_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_12 regras da copa da italia");
    let mut g = ITALIAN_CUP_GROUP_RULES;
    g.vitoria = 3; // cqi12: the win in the Coppa Italia groups
    g.categoria = 4; // cqi12: the category of the Coppa Italia matches
    set_cup_league_rules(this, &g, 1)
}

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

