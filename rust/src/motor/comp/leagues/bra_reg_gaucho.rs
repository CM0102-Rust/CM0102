use super::*;

/// Builds a knockout stage from table rows (the same template in the semi-finals and in the state leagues' knockouts;
/// only the source file, the box lines and the arrangement change).
///
/// The list of the `linhas.len()` teams (malloc `0x944e46` of 4 × n; out of memory: the box `linha_sem_lista` and
/// nothing else) with the clubs of the table's `linhas`, in this order (the ties: 0 × 1, 2 × 3...); virtual `+0x3c`
/// (create_round_table: thiscall(this, stage 0, &rodadas, &extra, null), ret 0x10) builds the round table; the new
/// stage (operator new `0x944cf1` of 0xb2 bytes; Cup_Stage::Cup_Stage `0x522e00`, thiscall with 15 arguments, ret 0x3c:
/// the league, n, the list, the rounds, `+4`, the round table, the year `+0x40`, 0, 1, the virtual's extra, 0x14, 0, 1,
/// 0 and the seeds, one byte per team, which the constructor writes into `+4` of the stage's list: in the original
/// routine, each team's row) goes into the first stage slot ([`+0xc`]).
///
/// Without the stage: the box `linha_sem_fase` and return (without freeing). With it: Cup::set_up_round (`0x51c800`,
/// thiscall(stage, 0), ret 4), the round table and the list are freed (free `0x9452ca`, in that order) and `+0x30` = 0.
/// The original routine's SEH frame only covers the constructor (no destructor).
///
/// [pt] O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe fn create_knockout_from_table(this: League, rows: &[usize], seeds: &[u8], file: usize, row_without_list: u32, row_without_stage: u32, ramo: &'static str) {
    let n = rows.len();
    // the seeds: the original writes the bytes on the stack before allocating (same as the rows, but separate constants)
    let mut s = [0u8; 8];
    s[..seeds.len()].copy_from_slice(seeds);
    let list = exe::crt_malloc((n * 4) as u32);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        write_usize(list + 4 * k, bra_reg_bahia_table_row_club(this, l));
    }
    covered!(ramo);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in cx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), n as u32, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    write_usize(this.stages(), stage);
    let stage = read_usize(this.stages());
    if stage == 0 {
        return show_error_at(file, row_without_stage);
    }
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(t);
    exe::crt_free(list);
    this.set_current_stage((0) as usize);
}

/// Rio Grande do Sul: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed
/// is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegGaucho::create_knockout_stage` (0x4325b0, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x4325b0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98ae50, linhas 0x217 (sem lista) e 0x22b (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_create_knockout_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98ae50, 0x217, 0x22b, "cqn_9 gaucho"); // cqn9: 1º × 4º, 2º × 3º Gaúcho
}

/// End of a state league: records the honours of the first stage (the table) and calls `exe::bra_nat_first_fill_champions_cup_entrants`, which, once the twelve
/// state leagues are over, puts the champions into the Champions Cup.
///
/// `BraRegGaucho::record_season_honours` (slot 13 of nine state leagues: Baiano, Centro-Oeste, Gaúcho, Goiano, Norte,
/// Nordeste, Pernambucano, Catarinense and Paulista): the history of the first stage and then, as a tail jump, `exe::bra_nat_first_fill_champions_cup_entrants`
/// (`bra_nat_first_fill_champions_cup_entrants`, cdecl with no arguments: when the twelve state leagues are over, it
/// puts the champions into the Champions Cup); returns its eax.
/// Original: `BraRegGaucho::record_season_honours` (0x432970, bra_reg_gaucho.cpp, thiscall).
///
/// [pt] { 0x432970 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::record_season_honours` (slot 13 de nove estaduais: Baiano,
/// Centro-Oeste, Gaúcho, Goiano, Norte, Nordeste, Pernambucano, Catarinense e Paulista): o histórico do primeiro estágio
/// e, em salto final, a 0x42bba0 (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze
/// estaduais terminam, põe os campeões na Copa dos Campeões); devolve o eax dela.
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_record_season_honours(this: usize) -> u32 {
    record_first_stage_honours(League(this)); // cqj8: the state championship history
    covered!("cqj_8 state league history");
    exe::bra_nat_first_fill_champions_cup_entrants()
}

/// The Rio Grande do Sul state league at career creation: the year and the database competition, disciplinary rule 7
/// and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules,
/// the table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegGaucho::BraRegGaucho` (0x4317c0, bra_reg_gaucho.cpp, thiscall, ret 8).
///
/// [pt] { 0x4317c0 } (thiscall, ret 8, bra_reg_gaucho.cpp): `BraRegGaucho::BraRegGaucho(ano, registro)`: o Gaúcho (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_gaucho(this: usize, year: u32, record: usize) -> usize {
    bra_reg_bahia_construct_state_league(League(this), year, record, &GAUCHO_STATE_LEAGUE, 7, true, "cqt_1 gaucho novo") // cqt1: the discipline and expected positions of the Gaúcho
}

/// The Rio Grande do Sul turnover: archives the season in the statistics, clears the previous table and stages, moves
/// to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table
/// order; then the predicted positions and the renewal of the state referees
/// Original: `BraRegGaucho::start_new_season` (0x432370, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x432370 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::start_new_season()`, a virada da temporada do Gaúcho (a tabela:
/// 0x4321d0; caixas nas linhas 0x1c6, 0x1cd, 0x1d3 e 0x1da).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x4321d0, 0x98ae50, [0x1c6, 0x1cd, 0x1d3, 0x1da], 1, "cqp_7 virada do gaucho") // cqp7: the next year of the Gaúcho
}

/// The Rio Grande do Sul next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to
/// the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegGaucho::advance_stage` (0x432520, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x432520 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::advance_stage()`, a fase seguinte do Gaúcho: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4325b0); a caixa da linha 0x1f4.
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98ae50, 0x1f4, 0x4325b0);
}

/// the Rio Grande do Sul league has 17 clubs: those with it as their reserve division in the database (the field that
/// holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegGaucho::load_region_clubs` (0x4321d0, bra_reg_gaucho.cpp, thiscall, ret 0).
///
/// [pt] { 0x4321d0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::load_region_clubs()`: o Gaúcho tem 17 clubes (divisão reserva +0x60 = [0x9cf9c8]).
/// Fonte 0x98ae50, linha 0x163.
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_load_region_clubs(this: usize) -> u32 {
    fill_state_league_table(League(this), 0x11, 0x98ae50, 0x163, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 0, "cql_8 gaucho cheio")
}

/// Gaucho, Northeast Cup and Paulista (the same routine in the three vtables): single round robin; 4 top places (the
/// 4-club knockout); 2 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the
/// table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places
/// lead to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegGaucho::set_up_rules` (0x4322d0, bra_reg_gaucho.cpp, thiscall).
///
/// [pt] { 0x4322d0 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::set_up_rules`, nas vtables do Gaúcho (0x96811c), do
/// Nordeste (0x9683ec) e do Paulista (0x968770); devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_set_up_rules(this: usize) -> u32 {
    gaucho_rules(this);
    covered!("cqi_8 gaucho rules");
    create_main_schedule(League(this));
    1
}

