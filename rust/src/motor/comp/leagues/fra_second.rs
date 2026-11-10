use super::*;

/// What changes from one division to another in the constructor.
///
/// [pt] O que muda de uma divisão para outra no construtor.
pub(crate) struct LeagueDivision {
    pub(crate) vtabela: usize,
    /// a regra de disciplina (+0x50): 0xc a francesa, 0x1a a espanhola
    pub(crate) discipline: u8,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    pub(crate) rows: [u32; 5],
    /// a Primera cria no fim a divisão de baixo espanhola (+0xee)
    pub(crate) low: bool,
}

/// The body of the four league constructors (thiscall(this, ano, registro), ret 8, SEH frame only for the operator new
/// calls): loads the saved state or builds a new league; for the Primera also the Spanish lower division at `+0xee`.
///
/// The base (League::League `0x682200`: mov ecx -> thiscall, ret 0), the record (`+4`), the vtable, the year (`+0x40`)
/// and the discipline (`+0x50`); tries to load the state (League::load_from_temp_file `0x687b10`: push 1; mov ecx ->
/// thiscall, ret 4). Loaded: done.
///
/// Otherwise, the new league, the league only: stage 0xff (`+0x44`), current stage −1 (`+0x30`), `+0x2c` = 0, `+0xc` =
/// 0; the rules, the table, the fixtures (League::create_fixtures `0x6835c0`), the order (League::sort_table
/// `0x6827d0`: push 0 -> thiscall, ret 4), the statistics (operator new `0x944cf1` of 0x5ce: push; add esp, 4 -> cdecl;
/// Comp_Stats `0x49ee70`: push league; mov ecx, new -> thiscall, ret 4) into `+8` and the predicted positions
/// (`0x68a850`: mov ecx -> thiscall, ret 0); each step that fails shows the box of its line and ends.
///
/// In the Primera, finally: if the competition's nation ([`+4`]+`0x5d`) does not have bit 4 of `+0x11c` [inference: the
/// playable lower division], the Spanish lower division (operator new of 0xee; SpaLower::SpaLower `0x851020`: push
/// year; mov ecx, new -> thiscall, ret 4) goes into `+0xee` (0 when out of memory); with the bit, `+0xee` = 0. Returns
/// this.
///
/// [pt] O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para os operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a disciplina
/// (+0x50); tenta carregar o estado (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4).
/// Carregou: pronto. Senão, a liga nova, só a liga: fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c = 0, +0xc = 0; as
/// regras, a tabela, os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 ->
/// thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats 0x49ee70:
/// push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0);
/// cada passo que falha mostra a caixa da sua linha e encerra. Na Primera, por fim: se a nação da competição
/// ([+4]+0x5d) não tem o bit 4 de +0x11c [inferência: a divisão de baixo jogável], a divisão de baixo espanhola
/// (operator new de 0xee; SpaLower::SpaLower 0x851020: push ano; mov ecx, novo -> thiscall, ret 4) vai para +0xee
/// (0 sem memória); com o bit, +0xee = 0. Devolve this.
pub(crate) unsafe fn construct_division_league(this: League, ano: u32, registro: usize, p: &LeagueDivision) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), p.vtabela);
    this.set_year((ano as u16) as i16);
    this.set_discipline_rule(p.discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    crate::comparison::branch("cqm_5 new division");
    this.set_stage_index(0xff);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_stages(0);
    if regras(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(p.file, p.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.file, p.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    if p.low {
        let nacao = this.competition().nation();
        if nacao.league_selected() & 4 == 0 { // cqm5: the Spanish lower division
            crate::comparison::branch("cqm_5 spanish lower division");
            let e = exe::crt_operator_new(0xee);
            // simplification: the original pushes the year in the low word of ecx, with the rest of the record pointer in
            // above; 0x851020 only reads the word (mov ax, word ptr [esp + 0x21c]).
            let b = if e != 0 { exe::new_spalower(e, (this.year() as u16) as u32) } else { 0 };
            write_usize(this.addr() + 0xee, b);
        } else {
            write_usize(this.addr() + 0xee, 0);
        }
    }
    this.addr()
}

pub(crate) const FRENCH_SECOND_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96ac2c,
    discipline: 0xc, // cqm5: the French discipline rule
    rules: 0x5bf0a0,
    table: 0x5bef10,
    file: 0x9ccc0c,
    rows: [0x43, 0x4a, 0x51, 0x5a, 0x61],
    low: false,
};
pub(crate) const FRENCH_THIRD_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96ad80,
    discipline: 0xc,
    rules: 0x5c0700,
    table: 0x5c05d0,
    file: 0x9ccc80,
    rows: [0x42, 0x49, 0x50, 0x59, 0x60],
    low: false,
};
pub(crate) const SPANISH_SEGUNDA: LeagueDivision = LeagueDivision {
    vtabela: 0x96fbd0,
    discipline: 0x1a, // cqm5: the Spanish discipline rule
    rules: 0x855cc0,
    table: 0x855b40,
    file: 0xa828bc,
    rows: [0x46, 0x4d, 0x54, 0x5b, 0x62],
    low: false,
};
pub(crate) const SPAIN_PRIMERA_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96fa68,
    discipline: 0x1a,
    rules: 0x84fdf0,
    table: 0x84fc70,
    file: 0xa82844,
    rows: [0x47, 0x4e, 0x55, 0x5c, 0x63],
    low: true,
};

/// the French second division: discipline rule 0xc (the French one), only the league; with no saved state, the rules,
/// the table, the fixtures, the order, the stats and the predicted positions
/// Original: `FraSecond::FraSecond` (0x5be330, fra_second.cpp, thiscall, ret 8).
///
/// [pt] { 0x5be330 } (thiscall, ret 8, fra_second.cpp): `FraSecond::FraSecond(ano, registro)`: a segunda divisão francesa
/// (a Division 2), disciplina 0xc, só a liga.
pub(crate) unsafe extern "thiscall" fn construct_fra_second(this: usize, year: u32, record: usize) -> usize {
    construct_division_league(League(this), year, record, &FRENCH_SECOND_DIVISION)
}

/// Comparer for the four constructors: only the original runs; checks return, vtable, year, record, discipline, stage
/// fields and the lower division at +0xee.
///
/// [pt] O comparador dos quatro: alocam: só a original, conferindo a devolução, a vtable, o ano, o registro e a disciplina;
/// na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c = 0 e +0xc = 0; na Primera, a divisão de baixo em +0xee
/// conforme o bit 4 da nação.
pub(crate) unsafe fn compare_division_league_ctor(name: &str, this: League, year: u32, record: usize, p: &LeagueDivision, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == p.discipline;
    if this.stage_index() == 0xff && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == 0 && this.stages() == 0;
        if p.low {
            let nation = this.competition().nation();
            let has = read_usize(this.addr() + 0xee) != 0;
            ok = ok && has == (nation.league_selected() & 4 == 0);
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +8 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.comp_stats()));
    r
}

/// Sets up the rules of the French second division: 4 go up and 2 go down in 2001, then 3 up and 3 down; the rest as
/// the top division.
///
/// `FraSecond::set_up_rules()`: the common rules, 2 rounds, 3 points; in 2001, 4 promoted (`+0xbe`) and 2 relegated
/// (`+0xc1`), afterwards 3 up and 3 down; the first division above (`[0x9cf5cc]`), the third below (`[0x9cf5d4]`),
/// `+0x52` = 2 and 5 substitutes.
/// Original: `FraSecond::set_up_rules` (0x5bf0a0, fra_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bf0a0 } (thiscall, ret 0, fra_second.cpp): `FraSecond::set_up_rules()`. A segunda: as comuns, 2 turnos, 3
/// pontos; em 2001 sobem 4 (+0xbe) e caem 2 (+0xc1), depois sobem 3 e caem 3; a primeira acima ([0x9cf5cc]), a
/// terceira abaixo ([0x9cf5d4]), +0x52 = 2 e 5 reservas.
pub(crate) unsafe extern "thiscall" fn fra_second_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_5 second division rules");
    fra_first_apply_league_rules(League(this), &fra_first_second_division_rules(this))
}

/// the second has 20 clubs; no prize fund; TV pays by the division the club came from: 2,000,000 to those relegated
/// from the top flight and 1,500,000 to those who stayed and those promoted from the National
/// Original: `FraSecond::create_table` (0x5bef10, fra_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bef10 } (thiscall, ret 0, fra_second.cpp): `FraSecond::create_table()`: 20 clubes ([0x9cf5d0]); cheia, sem
/// fundo (+0x28 = 0, +0xe1 = 2: nada a pagar no fim) e a TV pela divisão de onde o clube veio (+0x5b): 2.000.000 aos
/// que caíram da primeira ([0x9cf5cc]), 1.500.000 aos que ficaram ([0x9cf5d0]) e 1.500.000 aos que subiram do
/// National ([0x9cf5d4]); 1. Fonte 0x9ccc0c, linha 0x171.
pub(crate) unsafe extern "thiscall" fn fra_second_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x14);
    if fra_first_fill_league_table(this, 0x9ccc0c, 0x171, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION) != Some(true) {
        return 0;
    }
    crate::comparison::branch("cqj_6 second division fixtures");
    this.set_prize_fund(0);
    this.set_prize_mode(2);
    eng_prm_pay_tv_rights(this.addr(), 0x1e8480, comp_record_of(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION)); // cqj6: the TV money of those relegated from the first division
    eng_prm_pay_tv_rights(this.addr(), 0x16e360, comp_record_of(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION));
    eng_prm_pay_tv_rights(this.addr(), 0x16e360, comp_record_of(KEY_CLUB_COMP_FRENCH_NATIONAL));
    1
}

/// the second division schedule: 38 rounds, from 31 July to 20 May; in 18 rounds one match goes to a TV slot (day 5,
/// slot 1) [inference: the televised match]
/// Original: `FraSecond::create_schedule` (0x5be620, fra_second.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5be620 } (thiscall, ret 0x10, fra_second.cpp, 2.275 bytes, método +0x3c): `FraSecond::create_schedule`: a segunda divisão,
/// 38 rodadas (`SECOND_DIVISION_ROUNDS`). Fonte 0x9ccc0c, linha 0xa7.
pub(crate) unsafe extern "thiscall" fn fra_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &FRENCH_SECOND_DIVISION_LEAGUE)
}

