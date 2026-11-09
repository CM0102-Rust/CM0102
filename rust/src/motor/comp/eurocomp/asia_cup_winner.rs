use super::*;

/// The group each African Cup quarter-final slot comes from (−1 = group A, the cup's own table; 0..2 = the stages of
/// groups B to D) and the position: the keys are the pairs of the list.
///
/// [pt] O grupo de onde sai cada vaga das quartas da Copa Africana (−1 = o grupo A, a tabela da própria copa; 0..2 = as fases
/// dos grupos B a D) e a posição: as chaves são os pares da lista.
pub(crate) const AFRICAN_NATIONS_QUARTER_FINAL_SLOTS: [(i32, i32); 8] = [
    (-1, 0), (0, 1), // cqt13: 1º A × 2º B
    (1, 0), (2, 1), // cqt13: 1º C × 2º D
    (1, 1), (2, 0), // cqt13: 2º C × 1º D
    (0, 0), (-1, 1), // cqt13: 1º B × 2º A
];

/// Club at position p of group g (or the cup itself when g < 0) via League::get_table_row.
///
/// [pt] O clube da posição `p` do grupo `g` (pela 0x6826d0 League::get_table_row: push p; mov ecx, grupo -> thiscall, ret 4).
pub(crate) unsafe fn group_table_club(this: League, g: i32, p: i32) -> usize {
    let group = if g < 0 { this.addr() } else { read_usize(this.stages() + g as usize * 4) };
    read_usize(exe::league_get_table_row(group, p))
}

/// `AfricanNations::create_knockout_stage()` (`0x402090`, thiscall, ret 0, `african_nations.cpp`): builds the
/// quarter-finals of the African Nations Cup.
///
/// SEH frame only for the stage's `new`, with no destructor on the normal path; called by the next-stage routine
/// `0x401e10` on reaching stage 3.
///
/// The list of the 8 (`0x944e46(0x20)`: push; add esp, 4 -> cdecl; out of memory, the box of line `0x311` and return)
/// by the pairings of AFRICAN_NATIONS_QUARTER_FINAL_SLOTS, in call order; the round table via virtual `+0x3c` (push 0;
/// push &id; push &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); the stage (`0x944cf1(0xb2)` push; add esp, 4
/// -> cdecl; Cup_Stage::Cup_Stage `0x522e00`, thiscall, ret 0x3c: the cup, 8, the list, the rounds, `+4`, the table,
/// the year `+0x40`, 3, 4, the id, 0x14, 0, 0, 0 and no seeds; the original routine pushes the rounds, the id and the
/// year with garbage in the high 16 bits, which the constructor does not read) into [`+0xc`][3]; the list and the table
/// are freed (`0x9452ca`).
///
/// Without the stage: the box of line `0x32c` and return. With it: the round is set up (`0x51c800` Cup::set_up_round:
/// push 0; mov ecx, stage -> thiscall, ret 4) and `+0x30` = 3.
///
/// [pt] { 0x402090 } (thiscall, ret 0, african_nations.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AfricanNations::create_knockout_stage()`, as quartas da Copa Africana (da fase seguinte 0x401e10, ao chegar
/// a 3): a lista dos 8 (0x944e46(0x20): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x311 e volta) pelos
/// cruzamentos de AFRICAN_NATIONS_QUARTER_FINAL_SLOTS, na ordem das chamadas; a tabela das rodadas pela virtual +0x3c (push 0; push &id; push
/// &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 8, a lista, as rodadas, +4, a tabela, o ano +0x40, 3, 4, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][3]; a lista e a tabela liberadas (0x9452ca). Sem fase: a caixa da linha 0x32c e volta.
/// Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall, ret 4) e +0x30 = 3.
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_create_knockout_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(0x20);
    if list == 0 {
        return show_error_at(0x9870f8, 0x311);
    }
    crate::comparison::branch("cqt_13 african cup quarterfinals");
    for (k, &(g, p)) in AFRICAN_NATIONS_QUARTER_FINAL_SLOTS.iter().enumerate() {
        write_usize(list + k * 4, group_table_club(this, g, p));
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 3, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 8, list, rodadas as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 3, 4, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 0xc, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let f = read_usize(this.stages() + 0xc);
    if f == 0 {
        return show_error_at(0x9870f8, 0x32c);
    }
    exe::cup_set_up_round(f, 0);
    this.set_current_stage(3);
}

// simplification: allocates the stage and sets up the round: does not run twice. Original only; the plan: the stage at [+0xc][3] with 8 teams,
// the cup’s +4 and the 8 cross-paired ones (read earlier through the tables, +0xb1 of each group) in its list (+0xa7, 6 bytes; as
// conjunto) e +0x30 = 3.

/// AsiaCupWinner::advance_stage(): moves the Asian Cup Winners' Cup to its next stage (+0x30); on reaching stage 1
/// (after both zones' knockouts) it creates the finals. Shows the original error box if already at the last stage.
///
/// [pt] { 0x40dfa0 } (thiscall, ret 0, asia_cup_winner.cpp; a virtual +0x28): `AsiaCupWinner::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x328; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), as finais (0x40e030: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_advance_stage(this: usize) {
    let f = read_usize(this + cup::CURRENT_STAGE) as i32;
    if f >= (read_usize(this + cup::STAGE_COUNT) as i32).wrapping_sub(1) {
        return show_error_at(0x987b60, 0x328);
    }
    let f = f + 1;
    write_usize(this + cup::CURRENT_STAGE, f as usize);
    if f == 1 {
        crate::comparison::branch("cqt_13 asian cup winners cup finals");
        exe::asiacupwinner_create_finals_stage(this);
    }
}

// simplification: the finals allocate and draw: they do not run twice. Original only; the plan: +0x30 goes up one (outside the last
// stage, which shows the box and changes nothing).

/// The 4 finalists of the Asian Cup Winners' Cup: the surviving entrants of the two zones' knockouts, with the 2nd and
/// 3rd swapped.
///
/// The survivors (+5 = 1 in the 6-byte table) of the first zone's knockout (the cup's table, +0xa7) and then those of
/// the second (table +0xa7 of stage [+0xc][0]), up to 4 in total, both searches going up to the cup's +0x3e (signed
/// word). The 2nd and 3rd swap places (the pairings: the brackets are the pairs of the list).
///
/// Slots left over stay 0 (the original routine leaves them with whatever malloc returned).
///
/// [pt] Os 4 finalistas da Recopa asiática: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da
/// copa, +0xa7) e depois os da segunda (a tabela +0xa7 da fase [+0xc][0]), até 4 ao todo, as duas buscas até o +0x3e da
/// copa (palavra com sinal); o 2º e o 3º trocam de lugar (os cruzamentos: as chaves são os pares da lista). As vagas
/// que sobrarem ficam 0 (a original as deixa com o que o malloc deu).
pub(crate) unsafe fn asian_cup_winners_finalists(this: usize) -> [usize; 4] {
    let mut v = [0usize; 4];
    let mut n = 0i16;
    let teams = read_i16(this + cup::TEAM_COUNT);
    for tabela in [read_usize(this + cup::TEAM_TABLE), read_usize(read_usize(read_usize(this + cup::STAGES)) + 0xa7)] {
        let mut i = 0i16;
        while i < teams && n < 4 {
            let e = tabela + i as usize * 6;
            if *((e + 5) as *const u8) == 1 {
                v[n as usize] = read_usize(e);
                n += 1;
            }
            i += 1;
        }
    }
    v.swap(1, 2); // cqt13: crosses the middle pair
    v
}

/// `AsiaCupWinner::create_finals_stage()` (`exe::asiacupwinner_create_finals_stage`, thiscall, ret 0, asia_cup_winner.cpp): builds the 4-team finals
/// stage of the Asian Cup Winners' Cup from its finalists, gets the round table from the cup's virtual +0x3c,
/// constructs the Cup_Stage in stage slot 1 (cup +0xc) and draws its round 0.
///
/// The SEH frame is only for the stage's new: no destructor on the normal path.
///
/// The list of 4 (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the error box for line 0x34b and return)
/// from asian_cup_winners_finalists; the round table via virtual +0x3c (push 0; push &id; push &rodadas; push 1; mov
/// ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage
/// `construct_cup_stage`, thiscall, ret 0x3c: the cup, 4, the list, the rounds, +4, the table, the year +0x40, 1, 2, the id, 0x14,
/// 0, 0, 0 and no seed; the original pushes the rounds, the id and the year with garbage in the high 16 bits, which the
/// constructor does not read) stored in [+0xc][1]; the table is freed (`exe::crt_free`).
///
/// No stage: the error box for line 0x375 and return (the list is not freed, as in the original). With it: the draw of
/// round 0 (`draw_round` Cup::draw_round: push 0; mov ecx, stage -> thiscall, ret 4) and the list is freed.
///
/// [pt] { 0x40e030 } (thiscall, ret 0, asia_cup_winner.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AsiaCupWinner::create_finals_stage()`: a lista dos 4 (0x944e46(0x10): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x34b e volta) de asian_cup_winners_finalists; a tabela das rodadas pela virtual +0x3c (push 0; push
/// &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 1, 2, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][1]; a tabela liberada (0x9452ca). Sem fase: a caixa da linha 0x375 e volta (a lista não
/// é liberada, como na original). Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round: push 0; mov ecx, fase ->
/// thiscall, ret 4) e a lista liberada.
pub(crate) unsafe extern "thiscall" fn create_finals_stage(this: usize) {
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0x987b60, 0x34b);
    }
    crate::comparison::branch("cqt_13 asian cup winners cup finalists");
    // simplification: with fewer than 4 alive the original leaves in the slots what malloc gave; the copy writes 0 (in this career there are 4)
    for (k, &c) in asian_cup_winners_finalists(this).iter().enumerate() {
        write_usize(list + k * 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, 1, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 4, list, rodadas as u16 as u32, read_usize(this + 4), table, read_i16(this + cup::YEAR) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(read_usize(this + cup::STAGES) + 4, stage);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + cup::STAGES) + 4);
    if f == 0 {
        return show_error_at(0x987b60, 0x375);
    }
    exe::cup_draw_round(f, 0);
    exe::crt_free(list);
}

// simplification: allocates the stage and draws the round: it does not run twice. Original only; the plan: the stage in [+0xc][1] with 4
// teams, the cup’s +4 and the 4 finalists (read earlier) in its list (+0xa7, 6 bytes; as a set).

/// rule: the Asian Cup Winners' Cup has, in each zone (stages −1 and 0, ids 0x480 and 0x481), three two-legged
/// qualifying rounds: 1st on 23/8, 2nd on 20/9, quarter-finals on 24 March of the following year (21 days between legs
/// in the 1st, 7 after); the final stage (stage 1) in May: semi-final on 24/5, third place and final on 26/5, single
/// matches; category 4 in the qualifiers and 6 after
/// Original: `AsiaCupWinner::create_round_table` (0x40cf90, asia_cup_winner.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x40cf90 } (thiscall, ret 0x10, asia_cup_winner.cpp; a virtual +0x3c): `AsiaCupWinner::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `asian_cup_winners_stage`.
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(this, asian_cup_winners_stage(stage), rounds, extra, league, 0x987b60)
}

/// rule: the Asian Cup Winners' Cup has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with
/// class and status 0
/// Original: `AsiaCupWinner::create_team_entries` (0x40d4d0, asia_cup_winner.cpp, thiscall, ret 0).
///
/// [pt] { 0x40d4d0 } (thiscall, ret 0, asia_cup_winner.cpp): `AsiaCupWinner::create_team_entries()`: os 16 times (a caixa
/// da linha 0x185 sem memória).
pub(crate) unsafe extern "thiscall" fn create_team_entries(this: usize) -> i32 {
    build_team_table(this, 0x987b60, 0x185, "cqq_12 times da recopa asiatica", |i| i) // cqq12: the Asian Cup Winners’ Cup teams
}

