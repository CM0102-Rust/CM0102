use super::*;

/// Describes one state league.
///
/// It holds the vtable, the stages beyond the league (+0x2c: 1 in the round-robin ones with a knockout, 3 in the ones
/// with two rounds and a final; the vector [+0xc] = malloc of 4 × stages), the rules (thiscall, ret 0), the table
/// (thiscall, ret 0), the predicted positions (thiscall, ret 0), the source file and the error-box lines (rules, table,
/// fixtures, order, statistics).
///
/// [pt] Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct BraRegRioStateChampionship {
    pub(crate) vtabela: usize,
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) predicted: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The constructor of a Brazilian state league (the same 544-byte template in all twelve, checked instruction by
/// instruction: only the vtable, the stages, the three own routines, the source file and the line numbers change).
///
/// League::League (`exe::new_league`, thiscall, ret 0); +4 = the competition record; the vtable; the year (+0x40, word) = `ano`;
/// the discipline rule (+0x50) = 7. If the saved state loads (League::load_from_temp_file `exe::league_load_from_temp_file`, thiscall(this, 1),
/// ret 4; returns ≠ 0), returns this.
///
/// Otherwise (new career): +0x51 = 0xf (the importance), +0x44 = 0xff, +0x30 = −1, +0x2c = the stages and their array
/// (malloc `exe::crt_malloc`, unchecked); then, each with its own error box (returning this) if it returns 0: the rules, the
/// table, the fixtures (League::create_fixtures `create_fixtures`, thiscall, ret 0) and the sort (League::sort_table `sort_table`,
/// thiscall(this, 0), ret 4); the stats (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`,
/// thiscall(new, this), ret 4; the original's SEH frame covers only that new) at +8 (without them: the error box);
/// finally the predicted positions.
///
/// [pt] O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn bra_reg_rio_construct_state_league(this: League, year: u32, registro: usize, e: &BraRegRioStateChampionship, discipline: u8, predicted_yes: bool, ramo: &'static str) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), e.vtabela);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    crate::comparison::branch(ramo);
    this.set_importance(0xf);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    this.set_stages(v);
    if regras(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    if predicted_yes {
        previstas(this.addr());
    }
    this.addr()
}

/// Comparer for state league constructors: runs only the original and checks return, vtable, year, record, discipline
/// and, for a new league, stages, stage vector and importance.
///
/// [pt] O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn bra_reg_rio_compare_state_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraRegRioStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == discipline_rule::BRAZIL_REGIONAL;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

pub(crate) const CARIOCA_CHAMPIONSHIP: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x968608,
    stages: 3,
    rules: 0x43a600,
    table: 0x43a500,
    predicted: 0x43add0,
    file: 0x98b014,
    rows: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const CATARINENSE_STATE_CHAMPIONSHIP: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x9686bc,
    stages: 1,
    rules: 0x43bc70,
    table: 0x43bb70,
    predicted: 0x43c110,
    file: 0x98b050,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const PAULISTA_STATE_LEAGUE: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x968770,
    stages: 1,
    rules: 0x4322d0,
    table: 0x43cc40,
    predicted: 0x43d140,
    file: 0x98b090,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Rio state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegRio::BraRegRio` (0x439a40, bra_reg_rio.cpp, thiscall, ret 8).
///
/// [pt] { 0x439a40 } (thiscall, ret 8, bra_reg_rio.cpp): `BraRegRio::BraRegRio(ano, registro)`: o Carioca (três fases além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_rio(this: usize, year: u32, record: usize) -> usize {
    bra_reg_rio_construct_state_league(League(this), year, record, &CARIOCA_CHAMPIONSHIP, 7, true, "cqt_4 carioca novo") // cqt4: the discipline and expected positions of the Carioca
}

/// Competition reputation of a league (word +0x69 of the competition record at +4).
///
/// [pt] A reputação da competição de uma liga (+0x69 do registro +4, word).
pub(crate) unsafe fn league_comp_reputation(league: League) -> i32 {
    league.competition().reputation() as i32
}

/// Raises a club's attendance (+0x73) to a floor that rises with reputation `r` and factor `f`, and raises the record
/// (+0x7b) with it.
///
/// The floor (from the branches that push attendance up): with r < 5, 250 × r × f; otherwise 750 × (r − 4) × f + 1000
/// (the original routine compares with the integer part + 1000 and writes the value with 1000 added before truncation).
///
/// Below it, attendance goes up to it and the record (+0x7b) rises up to it.
///
/// [pt] O piso que sobe com a reputação `r` (dos ramos que puxam o público para cima): com r < 5, 250 × r × f; senão
/// 750 × (r − 4) × f + 1000 (a original compara com a parte inteira + 1000 e grava a conta com 1000 somado antes do
/// truncamento). Abaixo dele, o público vai a ele e o recorde (+0x7b) sobe até ele.
pub(crate) unsafe fn raise_attendance_floor(c: Club, r: i32, f: f64) {
    let lim = if r < 5 {
        let lim = ftol((r.wrapping_mul(250)) as f64 * f);
        if c.attendance() >= lim {
            return;
        }
        lim
    } else {
        let x = (r.wrapping_sub(4).wrapping_mul(750)) as f64 * f;
        if c.attendance() >= ftol(x).wrapping_add(1000) {
            return;
        }
        ftol(x + 1000.0)
    };
    c.set_attendance(lim);
    if lim > c.max_attendance() {
        c.set_max_attendance(lim);
    }
}

/// Caps club attendance (+0x73) at lim, lowering the minimum (+0x77) to it.
///
/// [pt] O teto `lim` do público: acima dele, o público cai a ele e o mínimo (+0x77) desce até ele.
pub(crate) unsafe fn cap_attendance(c: Club, lim: i32) {
    if c.attendance() > lim {
        c.set_attendance(lim);
        if lim < c.min_attendance() {
            c.set_min_attendance(lim);
        }
    }
}

/// `atualizar_media_de_publico(league)` (cdecl, game.cpp, unnamed in the map): updates the clubs' average attendance at
/// the end of each league season. Called by the seasons loop (0x5d5ae0, at 0x5d5cfb: push league; call; add esp, 4), at
/// the end of the season of each league of the nation (the top one and those below, through +0x20).
///
/// The club's average attendance (+0x73, Attendance) moves with the season result, between the minimum (+0x77) and the
/// record (+0x7b). The weight `f` comes from the competition's nation (the first dword of [+4]+0x5d): 0.1 in
/// [0x9cf510], [0x9cf2dc], [0x9cf3e4], [0x9cf224]; 0.25 in [0x9cf2c0], [0x9cf2fc], [0x9cf408], [0x9cf4a8], [0x9cf4f8];
/// 0.5 in [0x9cf45c]; 1 in the others. `r` = the competition's reputation (+0x69 of the record, word). The final table
/// comes from the statistics (Comp_Stats::get_sorted_team_table `exe::comp_stats_get_sorted_team_table`, thiscall(+8, 1, &n, 1000), ret 0xc: 0x76-byte
/// records, +0 the club id, +0x72 the predicted position, +0x73 the final one, +5 the home games [inference], +6 the
/// summed attendance [inference]).
///
/// For each club (the branches in the original's order, with the same calls to virtuals +0x98 "has promotion" and +0x9c
/// "has relegation"; d = predicted − final + 1, word):
///
/// - champion without promotion: + (r + max(d, 0)) × 200 × f (stadium owner, +0x6d) or × 100 × f;
///
/// - 2nd (or position 0) without promotion: + (r + max(d, 0)) × 125 × f or × 65 × f;
///
/// - promoted (position ≤ +0xbe): by the league above ([0xadadfc][+0x1c]; without it, nothing): v = the club's +0x80
/// (reputation, word) × max(r above − r, 1) × f × 0.1, at least 100; + v + draw(v) (owner) or + v; and the floor by the
/// reputation of the league above;
///
/// - promotion play-off (+0xbf ≠ 0 and position ≤ +0xbf): with d − 1 > 0: + draw(r(d − 1) × 75 × f) + r(d − 1) × 75 × f
/// (owner) or + draw(r(d − 1) × 25 × f) + r(d − 1) × 10 × f; otherwise + draw(r × 50 × f or r × 25 × f); with home
/// games, the mix with the real attendance: (summed attendance / games + 4 × average) × f × 0.2;
///
/// - relegated (position > +0x3e − +0xc1): by the league below (without it, nothing): v = +0x80 × min(r below − r, −1)
/// × f × 0.2, at most −100 (with no relegation in the league below, +0x20 = −1: v = 2v − 250 − draw(750)); + v +
/// draw(v); and the ceiling of 2000 (r below ≤ 7) or 3000 times r below, × f;
///
/// - the others: + draw(r × d × 5 × f), × 10 for the owner with d > 0 or with +0x80 above 500 × r; the mix with the
/// real attendance (without the weight f); the floor by reputation; and the ceiling of 2000 r (r ≤ 7), 3000 r (r ≤ 12)
/// or 5000 r.
///
/// At the end of each: at least 500, and within [+0x77, +0x7b]. The table is freed (free `exe::crt_free`).
/// Original: `atualizar_media_de_publico (sem nome no mapa)` (0x5d6280, game.cpp, cdecl).
///
/// [pt] { 0x5d6280 } (cdecl, game.cpp, sem nome no mapa): `atualizar_media_de_publico(liga)`, do laço das temporadas
/// (0x5d5ae0, em 0x5d5cfb: push liga; call; add esp, 4), no fim da temporada de cada liga da nação (a de cima e as de
/// baixo, pela +0x20). A média de público do clube (+0x73, Attendance) anda com o resultado da temporada, entre o
/// mínimo (+0x77) e o recorde (+0x7b).
/// O peso `f` vem da nação da competição (o primeiro dword de [+4]+0x5d): 0,1 em [0x9cf510], [0x9cf2dc], [0x9cf3e4],
/// [0x9cf224]; 0,25 em [0x9cf2c0], [0x9cf2fc], [0x9cf408], [0x9cf4a8], [0x9cf4f8]; 0,5 em [0x9cf45c]; 1 nas outras.
/// `r` = a reputação da competição (+0x69 do registro, word). A tabela final vem das estatísticas (Comp_Stats::
/// get_sorted_team_table 0x4a4b20, thiscall(+8, 1, &n, 1000), ret 0xc: registros de 0x76 bytes, +0 o id do clube,
/// +0x72 a posição prevista, +0x73 a final, +5 os jogos em casa [inferência], +6 o público somado [inferência]);
/// para cada clube (os ramos na ordem da original, com as mesmas chamadas às virtuais +0x98 "tem acesso" e +0x9c "tem
/// rebaixamento"; d = prevista − final + 1, word):
/// - campeão sem acesso: + (r + max(d, 0)) × 200 × f (dono do estádio, +0x6d) ou × 100 × f;
/// - 2º (ou posição 0) sem acesso: + (r + max(d, 0)) × 125 × f ou × 65 × f;
/// - subiu (posição ≤ +0xbe): pela liga de cima ([0xadadfc][+0x1c]; sem ela, nada): v = +0x80 do clube (a reputação,
///   word) × max(r de cima − r, 1) × f × 0,1, no mínimo 100; + v + exe::utils_random(v) (dono) ou + v; e o piso pela reputação
///   da liga de cima;
/// - play-off de acesso (+0xbf ≠ 0 e posição ≤ +0xbf): com d − 1 > 0: + exe::utils_random(r(d − 1) × 75 × f) + r(d − 1) × 75 × f
///   (dono) ou + exe::utils_random(r(d − 1) × 25 × f) + r(d − 1) × 10 × f; senão + exe::utils_random(r × 50 × f ou r × 25 × f); com
///   jogos em casa, a mistura com o público real: (público somado / jogos + 4 × média) × f × 0,2;
/// - caiu (posição > +0x3e − +0xc1): pela liga de baixo (sem ela, nada): v = +0x80 × min(r de baixo − r, −1) × f × 0,2,
///   no máximo −100 (sem rebaixamento na de baixo, +0x20 = −1: v = 2v − 250 − exe::utils_random(750)); + v + exe::utils_random(v); e o
///   teto de 2000 (r de baixo ≤ 7) ou 3000 vezes a r de baixo, × f;
/// - os outros: + exe::utils_random(r × d × 5 × f), × 10 para o dono com d > 0 ou com +0x80 acima de 500 × r; a mistura com o
///   público real (sem o peso f); o piso pela reputação; e o teto de 2000 r (r ≤ 7), 3000 r (r ≤ 12) ou 5000 r.
/// No fim de cada um: no mínimo 500, e dentro de [+0x77, +0x7b]. A tabela é liberada (free 0x9452ca).
pub(crate) unsafe extern "C" fn cqt4_update_average_attendance(league: usize) {
    let league = League(league);
    let table_c = read_usize(G_COMP_OBJECT_TABLE);
    let cima = if league.upper_division_id() == -1 { 0 } else { read_usize(table_c + 4 * (league.upper_division_id() as usize)) };
    let low = if (league.lower_division_id() as i32) == -1 { 0 } else { read_usize(table_c + 4 * league.lower_division_id()) };
    let mut f = 1.0f64;
    let reg = league.competition().addr();
    if reg != 0 && read_usize(reg + competition::NATION) != 0 {
        let n = read_usize(read_usize(reg + competition::NATION));
        let em = |gs: &[usize]| gs.iter().any(|&g| n == read_usize(g));
        if em(&[KEY_NATION_WALES, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_AUSTRALIA]) {
            f = f64::from_bits(0x3fb9_9999_9999_999a);
        } else if em(&[KEY_NATION_DENMARK, KEY_NATION_FINLAND, KEY_NATION_NORWAY, KEY_NATION_SWEDEN, KEY_NATION_UNITED_STATES]) {
            f = 0.25;
        } else if n == read_usize(KEY_NATION_SCOTLAND) {
            f = 0.5;
        }
    }
    let promotion = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(league.addr()) + 0x98));
        v(league.addr())
    };
    let relegation = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(league.addr()) + 0x9c));
        v(league.addr())
    };
    let mut n = 0i32;
    let t = exe::comp_stats_get_sorted_team_table(league.comp_stats(), 1, &mut n, 1000);
    if t == 0 {
        return;
    }
    let mut i = 0i32;
    while i < n {
        let e = CompStatsClub(t + 0x76 * i as usize);
        let c = club_at(read_usize(e.addr()) as isize);
        let pos = e.current_position();
        let prevista = e.final_position();
        let owner = c.own_stadium() != 0;
        let add = |v: i32| c.set_attendance(c.attendance().wrapping_add(v));
        let d = (prevista as i16).wrapping_sub(pos as i16).wrapping_add(1);
        if pos == 1 && promotion() == 0 {
            let k = league_comp_reputation(league) + d.max(0) as i32;
            let val = if owner { k.wrapping_mul(200) } else { k.wrapping_mul(100) };
            add(ftol(val as f64 * f));
        } else if promotion() == 0 && pos < 3 {
            let k = league_comp_reputation(league) + d.max(0) as i32;
            let val = if owner { k.wrapping_mul(125) } else { k.wrapping_mul(65) };
            add(ftol(val as f64 * f));
        } else if promotion() != 0 && pos as i32 <= (league.promoted_count() as i8) as i32 {
            if cima != 0 {
                let mut a = read_i16(read_usize(cima + 4) + competition::REPUTATION).wrapping_sub(league.competition().reputation());
                if a < 1 {
                    a = 1;
                }
                let v = ftol(c.reputation() as f64 * a as f64 * f * 0.1).max(100);
                if owner {
                    let r = exe::utils_random(v);
                    add(r.wrapping_add(v));
                } else {
                    add(v);
                }
                raise_attendance_floor(c, league_comp_reputation(League(cima)), f);
                crate::comparison::branch("cqt_4 attendance of promoted");
            }
        } else if (league.promotion_playoff_count() as i8) != 0 && pos as i32 <= (league.promotion_playoff_count() as i8) as i32 {
            let a = (prevista as i16).wrapping_sub(pos as i16);
            if a > 0 {
                let s = a as i32;
                if owner {
                    let v = ftol(league_comp_reputation(league).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    let r = exe::utils_random(v);
                    let w = ftol(league_comp_reputation(league).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    add(r.wrapping_add(w));
                } else {
                    let v = ftol(league_comp_reputation(league).wrapping_mul(s).wrapping_mul(25) as f64 * f);
                    let r = exe::utils_random(v);
                    let w = ftol(league_comp_reputation(league).wrapping_mul(s).wrapping_mul(10) as f64 * f);
                    add(r.wrapping_add(w));
                }
            } else {
                let r = league_comp_reputation(league);
                let val = if owner { r.wrapping_mul(50) } else { r.wrapping_mul(25) };
                let v = ftol(val as f64 * f);
                add(exe::utils_random(v));
            }
            let games = read_u8(e.addr() + 5);
            if games != 0 {
                let val = (read_i32(e.addr() + 6) / games as i32).wrapping_add(c.attendance().wrapping_mul(4));
                c.set_attendance(ftol(val as f64 * f * 0.2));
            }
        } else if relegation() != 0
            && pos as i32 > league.club_count() as i32 - (league.relegated_count() as i8) as i32
        {
            if low != 0 {
                let mut a = read_i16(read_usize(low + 4) + competition::REPUTATION).wrapping_sub(league.competition().reputation());
                if a > -1 {
                    a = -1;
                }
                let mut v = ftol(c.reputation() as f64 * a as f64 * f * 0.2).min(-100);
                if read_i32(low + league::LOWER_DIVISION_ID) == -1 {
                    let r = exe::utils_random(0x2ee);
                    v = v.wrapping_mul(2).wrapping_sub(0xfa).wrapping_sub(r);
                }
                let r = exe::utils_random(v);
                add(r.wrapping_add(v));
                let rb = league_comp_reputation(League(low));
                let lim = if rb > 7 { ftol(rb.wrapping_mul(3000) as f64 * f) } else { ftol(rb.wrapping_mul(2000) as f64 * f) };
                cap_attendance(c, lim);
                crate::comparison::branch("cqt_4 attendance of relegated");
            }
        } else {
            let r16 = league.competition().reputation();
            let r = r16 as i32;
            let val = if d <= 0 && c.reputation() <= r16.wrapping_mul(500) {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            } else if owner {
                r.wrapping_mul(d as i32).wrapping_mul(10)
            } else {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            };
            let v = ftol(val as f64 * f);
            add(exe::utils_random(v));
            let games = read_u8(e.addr() + 5);
            if games != 0 {
                let val = (read_i32(e.addr() + 6) / games as i32).wrapping_add(c.attendance().wrapping_mul(4));
                c.set_attendance(ftol(val as f64 * 0.2)); // cqt4: the mix with the real attendance
            }
            crate::comparison::branch("cqt_4 attendance of mid-table");
            raise_attendance_floor(c, league_comp_reputation(league), f);
            let r = league_comp_reputation(league);
            let ceiling = if r > 12 { r.wrapping_mul(5000) } else if r > 7 { r.wrapping_mul(3000) } else { r.wrapping_mul(2000) };
            cap_attendance(c, ceiling);
        }
        if c.attendance() < 500 {
            c.set_attendance(500);
        }
        let a = c.attendance();
        if a < c.min_attendance() {
            c.set_attendance(c.min_attendance());
        } else if a > c.max_attendance() {
            c.set_attendance(c.max_attendance());
        }
        i += 1;
    }
    exe::crt_free(t);
}

/// Creates half-season stage `k` of a two-half league (the create_half_season_stage slot; the same template in the
/// three state championships and in the J1, checked instruction by instruction: only the source file, the box line and
/// the constructor's 20th argument change, 0xb in the state championships and 0xf in the J1).
///
/// Called from each match's result (0x43af20 and its siblings: push dl = +0x30 + 1; mov ecx -> thiscall, ret 4; the
/// original reads the whole argument, uses the low byte and pushes it whole to the constructor).
///
/// 1) The clubs of the league table (+0xb1, rows of 0x3b bytes, the +0x3e rows) in a vector (malloc `exe::crt_malloc` of 4 ×
/// +0x3e; out of memory: the box and return).
///
/// 2) The stage: operator new `exe::crt_operator_new`(0xee) and the League_Stage constructor 0x68b100 (thiscall, ret 0x58, 22
/// arguments, in the original's order: the league, +0x3e (word), the vector, 1, +4, 0, 0, the points per win +0xc2
/// (`pv`), per draw +0xc3, the match category +0xc4, &+0xc5 (the tie-break criteria), &{1,0,0,0} (one top slot: the
/// half's champion), the year +0x40 (word), k, the id 0x439 (first half) or 0x43a (second), 0, 1, 0, 8, `vinte`, 0, 2;
/// the original pushes the words and bytes with garbage in the high bytes, which the constructor does not read: the
/// copy passes clean values; k goes whole, as it arrived). Out of memory for the stage: 0 in the slot, no box.
///
/// 3) [+0xc][k] = the stage, +0x30 = k (the signed byte) and the vector freed (free `exe::crt_free`). The original's SEH frame
/// only covers the constructor (no destructor on the normal path).
///
/// [pt] O turno `k` de uma liga de dois turnos (o slot create_half_season_stage; o mesmo molde nos três estaduais e na J1,
/// conferido instrução a instrução: só mudam o fonte, a linha da caixa e o 20º argumento do construtor, 0xb nos
/// estaduais e 0xf na J1). Chamado do resultado de cada jogo (0x43af20 e as irmãs: push dl = +0x30 + 1; mov ecx ->
/// thiscall, ret 4; a original lê o argumento inteiro, usa o byte baixo e o empurra inteiro ao construtor).
/// 1) Os clubes da tabela da liga (+0xb1, linhas de 0x3b bytes, as +0x3e linhas) num vetor (malloc 0x944e46 de
///    4 × +0x3e; sem memória: a caixa e volta).
/// 2) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, 1, +4, 0, 0, os pontos por vitória +0xc2 (`pv`), por empate +0xc3, a
///    categoria dos jogos +0xc4, &+0xc5 (os critérios de desempate), &{1,0,0,0} (uma vaga de cima: o campeão do
///    turno), o ano +0x40 (word), k, o id 0x439 (primeiro turno) ou 0x43a (segundo), 0, 1, 0, 8, `vinte`, 0, 2; a
///    original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores
///    limpos; o k vai inteiro, como chegou). Sem memória para a fase: 0 na vaga, sem caixa.
/// 3) [+0xc][k] = a fase, +0x30 = k (o byte com sinal) e o vetor liberado (free 0x9452ca). O quadro SEH da original só
///    cobre o construtor (sem destrutor no caminho normal).
pub(crate) unsafe fn create_half_season_stage(this: League, k: u32, file: usize, row: u32, twenty: u32, pv: usize, ramo: &'static str) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, u32, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let slots: [u8; 4] = [1, 0, 0, 0];
    let n = this.club_count();
    let clubs = exe::crt_malloc(((n as i32) << 2) as u32);
    if clubs == 0 {
        return show_error_at(file, row);
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        write_usize(clubs + 4 * i as usize, read_usize(this.table() + i as usize * league_table_row::SIZE));
        i += 1;
    }
    let id = 0x439 + (k as u8 != 0) as u32;
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), this.club_count() as u16 as u32, clubs, 1, this.competition().addr(), 0, 0,
                  read_u8(this.addr() + pv) as u32,
                  this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, slots.as_ptr(),
                  this.year() as u16 as u32, k, id, 0, 1, 0, 8, twenty, 0, 2)
    } else {
        0
    };
    let k = k as u8 as i8 as i32;
    write_usize(this.stages() + 4 * k as usize, stage);
    this.set_current_stage((k) as usize);
    crate::comparison::branch(ramo);
    exe::crt_free(clubs);
}

/// Comparer for a half-season stage: only the original runs; checks that slot k has the stage, +0x30 = k and the stage
/// has the league's clubs.
///
/// [pt] O comparador de um turno: só a original (simplification: aloca o vetor e a fase, que monta a tabela do turno), com o plano:
/// depois, a vaga k tem a fase (o operator new não falha na carreira), +0x30 = k e a fase tem os +0x3e clubes da liga.
pub(crate) unsafe fn compare_half_season_stage(name: &str, this: League, k: u32, original: impl FnOnce()) {
    original();
    let kk = k as u8 as i8 as i32;
    let stage = League(read_usize(this.stages() + 4 * kk as usize));
    let same = !stage.is_null() && this.current_stage() as i32 == kk && stage.club_count() == this.club_count();
    SCOREBOARD.record(same, || format!("{}({:#x}, {}) stage={:#x} +0x30={}", name, this.addr(), kk, stage.addr(), this.current_stage() as i32));
}

/// One half of the Rio state league: when the first game of a half is processed, it builds that half's round-robin
/// stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as
/// the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by
/// the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegRio::create_half_season_stage` (0x43afa0, bra_reg_rio.cpp, thiscall, ret 4).
///
/// [pt] { 0x43afa0 } (thiscall, ret 4, bra_reg_rio.cpp): `BraRegRio::create_half_season_stage(k)`, um turno do Carioca (os
/// doze clubes; a caixa da linha 0x3d5).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(League(this), k, 0x98b014, 0x3d5, 0xb, 0xc2, "cqr_1 turno do carioca") // cqr1: the points per win in the Carioca
}

/// The season rollover of a state league with two halves and a final (`start_new_season`, slot 3, `+0x08`): frees
/// table, stages and lists, archives statistics, allocates 3 stage slots, advances the year and rebuilds rules, table,
/// fixtures, order and officials.
///
/// The same template in the Carioca, the Mineiro and the Paranaense: only the source file and the table routine change,
/// checked instruction by instruction.
///
/// `+0x4c` = 0 and the table freed (League::free_table `0x687970`, thiscall(this, 0), ret 4); the list at `+0xba` freed
/// (free `0x9452ca`) and zeroed; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`,
/// thiscall(`+8`, 1), ret 4); with the current stage ≥ 0, the existing stages 0..=`+0x30` destroyed (virtual 0, the
/// destructor with 1: frees); the stage vector freed (free) and replaced by a new one with three slots (`+0x2c` = 3,
/// malloc `0x944e46` of 0xc, neither checked nor zeroed: the halves and the final fill it); the year (`+0x40`, word)
/// goes up by `ano` (the counter `ramo`) and `+0x30` = −1.
///
/// Then, each with its own box (lines `0x1d4`, `0x1db`, `0x1e1` and `0x1e8`) if it returns 0: the season rules (virtual
/// `+0x8c`), the table with the twelve clubs (`tabela`, thiscall, ret 0), the fixtures (League::create_fixtures
/// `0x6835c0`, thiscall, ret 0) and the table order (League::sort_table `0x6827d0`, thiscall(this, 0), ret 4); finally
/// the predicted positions (virtual `+0x5c`) and the renewal of the referees (OfficialsManager::renew_officials
/// `0x79cee0`, thiscall([`0xb74340`], `+4`), ret 4), whose eax it returns.
///
/// After a box, the copy returns 0 (the original routine returns the box's eax, which Brazil's rollover does not read).
///
/// [pt] A virada da temporada de um estadual de dois turnos e final (`start_new_season`, slot 3, +0x08; o mesmo molde no
/// Carioca, no Mineiro e no Paranaense: só mudam o fonte e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o vetor das fases liberado (free) e trocado por um novo de três vagas (+0x2c = 3, malloc 0x944e46 de 0xc, sem
/// conferir nem zerar: os turnos e a final o preenchem); o ano (+0x40, word) sobe `ano` (o contador `ramo`) e
/// +0x30 = −1; então, cada um com a sua caixa (linhas 0x1d4, 0x1db, 0x1e1 e 0x1e8) se devolver 0: as regras da
/// temporada (virtual +0x8c), a tabela com os doze clubes (`tabela`, thiscall, ret 0), os jogos (League::create_fixtures
/// 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as
/// posições previstas (virtual +0x5c) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0,
/// thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com caixa, a cópia devolve 0 (a original devolve o eax da
/// caixa, que a virada do Brasil não lê).
pub(crate) unsafe fn start_new_season_two_halves(this: League, table: usize, file: usize, year: i16, ramo: &'static str) -> u32 {
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
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
    exe::crt_free(this.stages());
    write_u32(this.addr() + league::STAGE_COUNT, 3);
    let v = exe::crt_malloc(0xc);
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    crate::comparison::branch(ramo);
    this.set_stages(v);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x8c));
    if rules(this.addr()) == 0 {
        show_error_at(file, 0x1d4);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x1db);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x1e1);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x1e8);
        return 0;
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x5c));
    predicted(this.addr());
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32 
}

/// Comparer for the season change: only the original runs; checks year +1, +0x2c = 3 and +0x30 = −1.
///
/// [pt] O comparador da virada: só a original (simplification: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um, +0x2c = 3 e +0x30 = −1.
pub(crate) unsafe fn compare_new_season(name: &str, this: League, original: impl FnOnce() -> u32) -> u32 {
    let before = this.year();
    let r = original();
    let after = this.year();
    let same = after == before.wrapping_add(1) && this.stage_count() == 3 && this.current_stage() as i32 == -1;
    SCOREBOARD.record(same, || format!("{}({:#x}) year before={} after={} +0x2c={} +0x30={}", name, this.addr(), before, after, this.stage_count(), this.current_stage() as i32));
    r
}

/// The Rio state league turnover: archives the season in the statistics, clears the table, the schedule and the stages
/// (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the rules,
/// the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the referee
/// renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegRio::start_new_season` (0x43a6a0, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a6a0 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::start_new_season()`, a virada do Carioca (a tabela: 0x43a500).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(League(this), 0x43a500, 0x98b014, 1, "cqr_2 virada do carioca") // cqr2: the next year of the Carioca
}

/// The Rio de Janeiro state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final
/// (stage 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the
/// last stage it is an error (the box)
/// Original: `BraRegRio::advance_stage` (0x43a870, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a870 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::advance_stage()`, a fase seguinte do Carioca (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x43a900, call direto com o ecx da liga).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x98b014, 0x202);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 2 {
        exe::braregrio_create_final_stage(this); // cqo9: the final after the second stage
    }
}

/// The Rio de Janeiro final: the winners of the first and second halves meet over two games [inference: home and away],
/// the first-half winner as seed 0; if the same club wins both halves, it is champion without a final.
///
/// `BraRegRio::create_final_stage()`, called from the next stage when the second half ends. The two half winners (the
/// club in the first row of each League_Stage's table) go into a list of 2 (malloc `exe::crt_malloc` of 8; out of memory: the
/// error box at line 0x226 and return).
///
/// - The same club in both halves: it is champion without a final. +0x30 = 1, +0x2c = 2 (the season ends with the two
/// stages), status 5 (+0x37, champion) in the first row of each half's table and in its row of the league table (+0xb1,
/// the +0x3e rows). The list is not freed (the original leaks it; so does the copy).
///
/// - Different winners: the final, over two games [inference: home and away]. Virtual +0x3c (create_round_table:
/// thiscall(this, stage 2, &rounds, &extra, 0), ret 0x10) builds the round table; the new stage (operator new `exe::crt_operator_new`
/// of 0xb2; Cup_Stage::Cup_Stage `construct_cup_stage`, thiscall, ret 0x3c: the league, 2, the list, the rounds, +4, the table, the
/// year +0x40, 2, 1, the extra, 0x14, 0, 1, 0 and the seeds {0, 1}: the first-half winner is seed 0) goes into the
/// third stage slot ([+0xc]+8; no stage: the error box at line 0x250 and return, without freeing). With it:
/// Cup::set_up_round (`set_up_round`, thiscall(stage, 0), ret 4), the table and the list freed (free `exe::crt_free`, in that order)
/// and +0x30 = 2. The original's SEH frame covers only the constructor (no destructor).
/// Original: `BraRegRio::create_final_stage` (0x43a900, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a900 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_final_stage(this: usize) {
    let this = League(this);
    let seeds: [u8; 4] = [0, 1, 0, 0];
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x98b014, 0x226);
    }
    let a = half_champion(this, 0);
    write_usize(list, a);
    let b = half_champion(this, 1);
    write_usize(list + 4, b);
    if a == b {
        crate::comparison::branch("cqo_9 same champion");
        this.set_current_stage((1) as usize);
        this.set_stage_count((2) as usize);
        for k in 0..2 {
            write_u8(read_usize(read_usize(this.stages() + 4 * k) + league::TABLE) + league_table_row::STATUS, 5); // cqo9: champion of both halves
        }
        let mut i: i16 = 0;
        while i < this.club_count() {
            let row = LeagueTableRow(this.table() + i as usize * league_table_row::SIZE);
            if read_usize(row.addr()) == read_usize(list) {
                row.set_status(table_status::CHAMPIONS);
            }
            i += 1;
        }
        // simplification: the 8-byte list is not freed, as in the original.
        return;
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 2, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in dx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, seeds.as_ptr())
    } else {
        0
    };
    write_usize(this.stages() + 8, stage);
    let stage = read_usize(this.stages() + 8);
    if stage == 0 {
        return show_error_at(0x98b014, 0x250);
    }
    crate::comparison::branch("cqo_9 final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(t);
    exe::crt_free(list);
    this.set_current_stage((2) as usize);
}

/// The end of the Rio de Janeiro (Carioca) season: with a final, the honours come from the final; without one, from the
/// league, and the champion gets 675,000 and the runner-up 250,000 in prize money. Then, once the twelve state leagues
/// are over, the champions go into the Champions Cup.
///
/// `BraRegRio::record_season_honours()` (slot 13).
///
/// - With a final (+0x30 = 2): the history of the final (slot 13, +0x30, of stage [+0xc]+8) and then, as a tail jump,
/// `exe::bra_nat_first_fill_champions_cup_entrants` (`bra_nat_first_fill_champions_cup_entrants`, cdecl with no arguments: when the twelve state leagues are
/// over, it puts the champions into the Champions Cup); returns its eax.
///
/// - Without a final (the same champion in both halves): the league history (League::record_season_honours `league_record_season_honours`,
/// thiscall, ret 0); the prizes, 675,000 to the champion (first row of table +0xb1) and 250,000 to the runner-up
/// (second row), each one to the balance (`get_club_finance` on [0xae2a38] -> the club's finances, thiscall, ret 4; `add_prize_money`
/// Finance::add_prize_money, thiscall, ret 4) and in the prize money news (`send_prize_money_news` Comp::send_prize_money_news:
/// thiscall(this, club, 64-bit amount, position 1 or 2, round 0, match 0, type −2), ret 0x1c); then `exe::bra_nat_first_fill_champions_cup_entrants`, as above.
/// Original: `BraRegRio::record_season_honours` (0x43ad20, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43ad20 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::record_season_honours()` (slot 13), o fim da temporada do Carioca.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 675.000 ao campeão (primeira linha da tabela +0xb1) e 250.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_record_season_honours(this: usize) -> u32 {
    let this = League(this);
    if this.current_stage() == 2 {
        let f = read_usize(this.stages() + 8);
        let history: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + 0x30));
        history(f);
        crate::comparison::branch("cqo_9 final history");
        return exe::bra_nat_first_fill_champions_cup_entrants();
    }
    let history: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    history(this.addr());
    for (linha, valor, posicao) in [(0usize, 675000i32, 1i32), (0x3b, 250000, 2)] {
        let c = read_usize(this.table() + linha);
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), valor);
        exe::comp_send_prize_money_news(this, read_usize(this.table() + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparison::branch("cqo_9 prizes");
    exe::bra_nat_first_fill_champions_cup_entrants()
}

/// the Rio league has 12 clubs: those with it as their reserve division in the database (the field that holds the state
/// or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegRio::create_table` (0x43a500, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a500 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_table()`: o Carioca tem 12 clubes (divisão reserva +0x60 = [0x9cf9dc]).
/// Fonte 0x98b014, linha 0x169.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xc, 0x98b014, 0x169, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 0, "cqm_8 carioca cheio")
}

/// Comparer of the three state league tables: runs only the original routine, then checks `n`, the return value and the
/// row clubs against the planned clubs.
///
/// The original allocates the table (it cannot run twice), so only it runs. The plan, made beforehand: the state
/// league's clubs in database order (up to `n`).
///
/// Afterwards: +0x3e = `n`, the return value = 1 if the plan filled up (otherwise 0) and the table rows (+0 the club,
/// 0x3b bytes) hold the planned clubs, in order.
///
/// [pt] O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn bra_reg_rio_compare_state_league_table(name: &str, this: League, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let club = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if is_in_state_league(Club(club), g) {
            plan.push(club);
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// Minas Gerais and Rio (the same routine in the two vtables): no top places in the overall table (the halves have
/// stages of their own, created by add_result 0x43af20 / 0x435130 [inference: they decide the title]); 2 relegated;
/// double round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no
/// league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to
/// the next stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the
/// main stage schedule
/// Original: `BraRegRio::set_up_rules` (0x43a600, bra_reg_rio.cpp, thiscall).
///
/// [pt] { 0x43a600 } (thiscall, bra_reg_rio.cpp): `BraRegRio::set_up_rules`, nas vtables do Mineiro (0x968284) e do
/// Carioca (0x968608); devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_set_up_rules(this: usize) -> u32 {
    carioca_rules(this);
    crate::comparison::branch("cqi_9 carioca rules");
    create_main_schedule(League(this));
    1
}

