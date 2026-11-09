use super::*;

/// The play-off of a Brazilian Série (the same template in A and B, checked instruction by instruction: only the source
/// file and the box lines change): builds the 4-club list, creates the Cup_Stage in the first stage slot, sets up round
/// 0, frees the list and sets +0x30 = 0.
///
/// The list of 4 (malloc `exe::crt_malloc` of 0x10; out of memory: the box `linha_sem_lista` and return) with the clubs of the
/// table's `linhas`, in this order (the ties: 0 × 1, 2 × 3); the virtual +0x3c (create_round_table: thiscall(this,
/// stage 0, &rounds, &extra, 0), ret 0x10) builds the round table.
///
/// The new stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `construct_cup_stage`, thiscall, ret 0x3c: the league, 4, the
/// list, the rounds, +4, the table, the year +0x40, 0, 1, the extra, 0x14, 0, 0, 0 and no seed: the order is not by
/// seeds) goes to the first stage slot ([+0xc]) and the round table is freed (free `exe::crt_free`).
///
/// No stage: the box `linha_sem_fase`, the list freed and return. With it: Cup::set_up_round (`set_up_round`, thiscall(stage,
/// 0), ret 4), the list freed and +0x30 = 0. The original's SEH frame only covers the constructor (no destructor).
///
/// [pt] O play-off de uma Série (o mesmo molde na A e na B, conferido instrução a instrução: só mudam o fonte e as linhas
/// das caixas): a lista dos 4 (malloc 0x944e46 de 0x10; sem memória: a caixa `linha_sem_lista` e volta) com os clubes
/// das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3); a virtual +0x3c (create_round_table: thiscall(this,
/// fase 0, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new 0x944cf1 de 0xb2;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 0, 1,
/// o extra, 0x14, 0, 0, 0 e nenhuma semente: a ordem não é por sementes) vai para a primeira vaga das fases ([+0xc]) e a
/// tabela de rodadas é liberada (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`, a lista liberada e volta. Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a lista liberada e +0x30 = 0. O quadro SEH da original só
/// cobre o construtor (sem destrutor).
pub(crate) unsafe fn create_play_off_stage(this: League, rows: &[usize; 4], file: usize, row_without_list: u32, row_without_stage: u32, ramo: &'static str) {
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        core::ptr::write_unaligned((list + 4 * k) as *mut usize, bra_reg_bahia_table_row_club(this, l));
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in dx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 4, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned(this.stages() as *mut usize, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        show_error_at(file, row_without_stage);
        exe::crt_free(list);
        return;
    }
    crate::comparison::branch(ramo);
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(list);
    this.set_current_stage((0) as usize);
}

/// The next stage of a Série (`advance_stage`, slot 11, +0x28).
///
/// At the last stage (+0x30 ≥ +0x2c − 1, signed), the error box and nothing else; otherwise +0x30 goes up by one and,
/// on reaching 0 (the end of the round-robin phase), builds the play-off (`play_off`, direct call with the league's ecx
/// -> thiscall, ret 0).
///
/// [pt] A fase seguinte de uma Série (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o play-off
/// (`play_off`, call direto com o ecx da liga -> thiscall, ret 0).
pub(crate) unsafe fn advance_series_stage(this: League, file: usize, row: u32, play_off: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Series A has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0
/// and the play-off is built; past the last stage it is an error (the box)
/// Original: `BraNatFirst::advance_stage` (0x42a950, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a950 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::advance_stage()`, a fase seguinte da Série A: no fim
/// dos pontos corridos, o play-off (0x42a9e0); a caixa da linha 0x264.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_advance_stage(this: usize) {
    advance_series_stage(League(this), 0x98aa30, 0x264, 0x42a9e0);
}

/// The Series A play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds: the list
/// order) [inference: two-legged semi-finals and final] that decides the Brazilian champion
/// Original: `BraNatFirst::create_play_off_stage` (0x42a9e0, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a9e0 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_play_off_stage()`, o play-off da Série A:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x286 (sem lista) e 0x29d (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_play_off_stage(this: usize) {
    create_play_off_stage(League(this), &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A
}

/// Comparer of the play-off creation: runs only the original routine and then checks the new stage against the plan.
///
/// Only the original runs (it allocates the list, the table and the stage and draws the round), with the plan made
/// beforehand: the clubs of the rows.
///
/// Afterwards, the stage in the first slot must exist, with +0x3e = 4, +4 = the league's +4 and the first 4 clubs of
/// its list (+0xa7, 6 bytes) equal to those of the plan (as a set: the round draw may reorder them), and +0x30 = 0.
///
/// [pt] O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = 4, +4 = o +4 da liga e os 4
/// primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada pode
/// reordenar), e +0x30 = 0.
pub(crate) unsafe fn compare_play_off_stage(name: &str, this: League, rows: &[usize; 4], original: impl FnOnce()) {
    let mut plan: Vec<usize> = rows.iter().map(|&l| bra_reg_bahia_table_row_club(this, l)).collect();
    original();
    let stage = read_usize(this.stages());
    if stage == 0 {
        SCOREBOARD.record(false, || format!("{}({:#x}) sem fase", name, this.addr()));
        return;
    }
    let list = read_usize(stage + cup::TEAM_TABLE);
    let mut found: Vec<usize> = (0..4).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && read_i16(stage + cup::TEAM_COUNT) == 4 && read_usize(stage + 4) == this.competition().addr() && this.current_stage() == 0;
    SCOREBOARD.record(same, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", name, this.addr(), pl, stage, ac, read_i16(stage + cup::TEAM_COUNT)));
}

/// Competition object whose id is in global `global` (table G_COMP_OBJECT_TABLE).
///
/// [pt] A competição de id [`global`] (a tabela das competições [0xadadfc]).
pub(crate) unsafe fn comp_by_global(global: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(global))
}

/// Calls the thiscall virtual at offset deslocamento of object c with no arguments → u32.
///
/// [pt] A virtual `deslocamento` de `c`, thiscall sem argumentos.
pub(crate) unsafe fn bra_nat_first_call_virtual_no_args(c: usize, deslocamento: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + deslocamento));
    f(c)
}

/// The common body of the season rollover of a Brazilian Série (the same as the state leagues' rollover, `cqp_7..9`):
/// frees and rebuilds everything; returns false after an error box.
///
/// The table is freed (League::free_table `0x687970`, thiscall(this, 0), ret 4); the list at `+0xba` is freed (free
/// `0x9452ca`) and zeroed; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`, thiscall(`+8`, 1),
/// ret 4); with the current stage ≥ 0, the existing stages 0..=`+0x30` are destroyed (virtual 0 with 1); the year
/// (`+0x40`, word) goes up by `ano` and `+0x30` = −1.
///
/// Then, each with its own box if it returns 0: the rules (virtual `+0x8c`), the table (`tabela`, thiscall, ret 0), the
/// fixtures (League::create_fixtures `0x6835c0`, thiscall, ret 0) and the table order (League::sort_table `0x6827d0`,
/// thiscall(this, 0), ret 4); finally the predicted positions (virtual `+0x5c`). Returns false after a box.
///
/// [pt] O corpo comum da virada de uma Série (o mesmo da virada dos estaduais, cqp_7..9): a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada (free 0x9452ca) e zerada; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0,
/// as fases 0..=+0x30 que existem destruídas (a virtual 0 com 1); o ano (+0x40, word) sobe `ano` e +0x30 = −1; então,
/// cada um com a sua caixa se devolver 0: as regras (virtual +0x8c), a tabela (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c). Devolve falso depois de uma caixa.
pub(crate) unsafe fn rebuild_season(this: League, table: usize, file: usize, rows: [u32; 4], year: i16) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        core::ptr::write_unaligned((this.addr() + league::ROUND_DATES) as *mut usize, 0);
    }
    let stats = read_usize(this.addr() + league::COMP_STATS);
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
    let a = core::ptr::read_unaligned((this.addr() + league::YEAR) as *const i16);
    core::ptr::write_unaligned((this.addr() + league::YEAR) as *mut i16, a.wrapping_add(year));
    core::ptr::write_unaligned((this.addr() + league::CURRENT_STAGE) as *mut i32, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), 0x8c) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), 0x5c);
    true
}

/// The common end of the season rollover: sets each club's board expectations (League::set_club_season_expectations
/// `set_club_season_expectations`, thiscall, ret 0) and renews the officials (OfficialsManager::renew_officials `renew_officials`,
/// thiscall([0xb74340], +4), ret 4), whose eax the rollover returns.
///
/// [pt] O fim comum: as expectativas da diretoria de cada clube (League::set_club_season_expectations 0x68aa80, thiscall,
/// ret 0) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4),
/// cujo eax a virada devolve.
pub(crate) unsafe fn set_expectations_and_renew_officials(this: League) -> u32 {
    let expectations: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectations(this.addr());
    (exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32) // cqq8: os árbitros
}

/// The season turnovers of the other fourteen competitions in Brazil, in the original routine's order.
///
/// Série B, Série C and the twelve state leagues (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste,
/// Paranaense, Pernambucano, Carioca, Catarinense and Paulista: the id globals 0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998,
/// 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4, 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4).
///
/// [pt] As viradas das outras catorze competições do Brasil, na ordem da original: a Série B, a Série C e os doze estaduais
/// (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste, Paranaense, Pernambucano, Carioca, Catarinense e
/// Paulista: os globais de id 0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998, 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4,
/// 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4).
pub(crate) const BRAZIL_TURNOVER_COMPS: [usize; 14] = [KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP];

/// The Brazilian turnover, led by Series A: first the promotion and relegation chain; then the Brazilian Cup entrants,
/// the 16 best clubs without a national division into Series C and the swaps between the state leagues and their lower
/// divisions; the division tier of the A and B clubs is capped; Series A is rebuilt (next year, rules, 28 clubs,
/// fixtures); then Series B, Series C and the twelve state leagues turn over (Bahia, Centre-West, Rio Grande do Sul,
/// Goias, Minas Gerais, North, Northeast, Parana, Pernambuco, Rio, Santa Catarina and Sao Paulo), with the board
/// expectations and the referee renewal
/// Original: `BraNatFirst::start_new_season` (0x42a640, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a640 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::start_new_season()`, a virada do futebol brasileiro
/// (a Série A comanda): +0x4c = 0; o acesso e o rebaixamento em cadeia (a virtual +0xb0, 0x42ae90, thiscall(this, 1),
/// ret 4); os inscritos da Copa do Brasil (0x42b760), a entrada de 16 clubes na Série C (0x42b520) e as trocas dos
/// estaduais com as divisões de baixo (0x42b130), as três com mov ecx, esi e sem push (thiscall, ret 0; o mapa as diz
/// cdecl(void): o ecx sobra, sem efeito); o nível da divisão dos clubes da Série A e da Série B limitado
/// (League::limit_club_division_tier 0x68a980, thiscall(liga, 1, 3, −3, 1), ret 0x10); a Série A refeita (o corpo comum,
/// a tabela 0x42a480, caixas nas linhas 0x223, 0x22a, 0x230 e 0x237); as viradas das outras catorze (a virtual +8);
/// as expectativas e os árbitros.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed((0) as usize);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0xb0));
    cadeia(this.addr(), 1);
    for f in [0x42b760usize, 0x42b520, 0x42b130] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    }
    exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION), 1, 3, -3, 1);
    if !rebuild_season(this, 0x42a480, 0x98aa30, [0x223, 0x22a, 0x230, 0x237], 1) {
        return 0;
    }
    for (k, g) in BRAZIL_TURNOVER_COMPS.iter().enumerate() {
        if k == 13 {
            crate::comparison::branch("cqq_8 paulista rollover");
        }
        bra_nat_first_call_virtual_no_args(comp_by_global(*g), 8); // cqq8: the rollover of each one
    }
    set_expectations_and_renew_officials(this)
}

/// Promotion and relegation in Brazil: records the final positions of Series A, Series B, Series C and the seven Series
/// C stages; then swaps the marked clubs between A and B, between B and C and between B and each C stage (who goes up
/// and down comes from the end-of-season marks, with no fixed number here)
/// Original: `BraNatFirst::process_promotion_relegation_chain` (0x42ae90, bra_nat_first.cpp, thiscall, ret 4).
///
/// [pt] { 0x42ae90 } (thiscall, ret 4, bra_nat_first.cpp): `BraNatFirst::process_promotion_relegation_chain(int)` (slot 45,
/// +0xb0; o argumento não é lido): as posições finais (a virtual +0xa4, record_final_positions, thiscall, ret 0) da
/// Série A, da Série B, da Série C e das sete fases da Série C ([C+0xc][0..7]); depois o acesso e o rebaixamento
/// (league_promote_and_relegate 0x689c80, thiscall(this, de cima, de baixo, 1, 1, −1, −1), ret 0x18: a original põe
/// o ecx da Série A e a rotina o guarda) entre A e B, entre B e C e entre a B e cada uma das sete fases da C. Devolve o
/// eax da última.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    let (a, b, c) = (KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION);
    bra_nat_first_call_virtual_no_args(comp_by_global(a), 0xa4);
    bra_nat_first_call_virtual_no_args(comp_by_global(b), 0xa4);
    bra_nat_first_call_virtual_no_args(comp_by_global(c), 0xa4);
    for k in 0..7 {
        bra_nat_first_call_virtual_no_args(read_usize(read_usize(comp_by_global(c) + 0xc) + 4 * k), 0xa4);
    }
    crate::comparison::branch("cqq_8 promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, -1, -1); // cqq8: A e B
    let mut r = exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c), 1, 1, -1, -1); // cqq8: B e C
    for k in 0..7 {
        r = exe::league_promote_and_relegate(this, comp_by_global(b), read_usize(read_usize(comp_by_global(c) + 0xc) + 4 * k), 1, 1, -1, -1);
    }
    r
}

/// True when the club's competition pointer at campo (+0x57 national division) is set and its id equals global g's.
///
/// [pt] O clube está na competição do global `g`: o ponteiro do clube em `campo` (+0x57 a divisão nacional) não é nulo e o
/// id do registro (+0) é o do global.
pub(crate) unsafe fn club_in_comp_global(club: usize, field: usize, g: usize) -> bool {
    let comp = read_usize(club + field);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// The common body of the Série A and Série B table setup (the same template as the state leagues): fills the table
/// with the division's clubs; returns 1 if it is full.
///
/// +0x3e = `n` clubs; the table (malloc `exe::crt_malloc` of n × 0x3b; out of memory: the error box of source `arquivo` at line
/// `linha` and returns 0); the clubs of [0xae23b4] (0x245 bytes, [0xae235c] of them) in the competition of global `g`,
/// in database order, each in the next row (League::init_table_row `init_table_row`: thiscall(league, row, club, 0, 0), ret
/// 0x10), until full.
///
/// `girar` is 0 (the original: row k gets the k-th club); it exists only for each division's mutations.
///
/// [pt] O corpo comum da Série A e da Série B (o mesmo molde dos estaduais): +0x3e = `n` clubes; a tabela (malloc 0x944e46
/// de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245
/// bytes, [0xae235c] deles) da competição do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada divisão.
pub(crate) unsafe fn create_division_table(this: League, n: i16, file: usize, row: u32, field: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if club_in_comp_global(club, field, g) {
            // simplification: the original pushes the row in eax with the high bits of the competition id; 0x687430 only reads
            // the byte. With girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            exe::league_init_table_row(this, l as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(0x245);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    crate::comparison::branch(ramo);
    1
}

/// Builds the Series A table: 28 clubs, those whose national division is Series A, in database order; when full, TV
/// pays 150,000 to each club (no prize fund).
///
/// `BraNatFirst::create_table()`: Series A has 28 clubs (those with national division +0x57 = `[0x9cf9ec]`); when full,
/// TV pays 150,000 to each club (League::pay_tv_rights `pay_tv_rights`: push 0; push 0x249f0; mov ecx, edi -> thiscall(league,
/// amount, filter), ret 8; it sends the TV news) and returns 1. Source 0x98aa30, line 0x1a1.
/// Original: `BraNatFirst::create_table` (0x42a480, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a480 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_table()`: a Série A tem 28 clubes (os de
/// divisão nacional +0x57 = [0x9cf9ec]); cheia, a TV paga 150.000 a cada clube (League::pay_tv_rights 0x683010:
/// push 0; push 0x249f0; mov ecx, edi -> thiscall(liga, valor, filtro), ret 8; manda a notícia da TV) e devolve 1.
/// Fonte 0x98aa30, linha 0x1a1.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_table(this: usize) -> u32 {
    if create_division_table(League(this), 0x1c, 0x98aa30, 0x1a1, 0x57, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, 0, "cql_7 serie a cheia") == 0 {
        return 0;
    }
    exe::league_pay_tv_rights(this, 0x249f0, 0);
    1
}

/// The clubs of global g's competition (pointer at campo) in database order, up to n.
///
/// [pt] Os clubes da competição do global `g` (ponteiro em `campo`), na ordem do banco, até `n`.
pub(crate) unsafe fn division_clubs_plan(n: usize, field: usize, g: usize) -> Vec<usize> {
    let mut plan = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && plan.len() < n {
        let club = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if club_in_comp_global(club, field, g) {
            plan.push(club);
        }
        i += 1;
    }
    plan
}

/// Comparer of the Série A and Série B table creation: runs only the original routine, then checks +0x3e, the return
/// value and the table rows against the planned clubs.
///
/// The original allocates the table (and in Série A pays the TV money with a news item), so only it runs. The plan,
/// made beforehand: the division's clubs in database order (up to `n`).
///
/// Afterwards: +0x3e = `n`, the return value = 1 if the plan filled up (otherwise 0) and the table rows (+0 the club,
/// 0x3b bytes) hold the planned clubs, in order.
///
/// [pt] O comparador da Série A e da Série B: a original aloca a tabela (e na A paga a TV com notícia): só ela. O plano,
/// feito antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano
/// encheu (senão 0) e as linhas da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn compare_division_table(name: &str, this: League, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let plan = division_clubs_plan(n as usize, 0x57, g);
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", name, this.addr(), r, plan, found));
    r
}

/// Creates the main stage schedule via vtable slot 16 (create_schedule, stage -1, outputs +0xa9/+0x3a) and stores the
/// pointer in +0xba.
///
/// [pt] O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x429b50, 0x42c140, 0x42d620): estágio -1, as saídas +0xa9 e +0x3a (palavras), sem a marca (0); o ponteiro
/// devolvido vai para +0xba.
pub(crate) unsafe fn create_main_schedule(o: League) {
    let vt = *(o.addr() as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o.addr(), -1, o.addr() + league::ROUND_COUNT, o.addr() + league::STAGE_ID, 0);
    write_u32_at(o.addr(), 0xba, r as u32);
}

/// Memory regions of the league object (0xee bytes) minus the schedule outputs (+0x3a, +0xa9, +0xba), for the comparer.
///
/// [pt] O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba), que o ensaio não chama: os
/// três calendários não gravam nada no objeto além delas (lido nas três rotinas: nenhum `mov [edi+..]`).
pub(crate) fn league_regions_without_schedule(o: usize) -> [(usize, usize); 4] {
    let o = League(o);
    [(o.addr(), 0x3a), (o.addr() + league::LEGS, 0xa9 - 0x3c), (o.addr() + league::MATCH_RULES, 0xba - 0xab), (o.addr() + league::PROMOTED_COUNT, 0xee - 0xbe)]
}

/// Writes the Série A rule fields (what `bra_nat_first_set_up_rules` writes before the schedule).
///
/// [pt] As regras da Série A (o que a 0x42a5a0 grava antes do calendário).
pub(crate) unsafe fn set_serie_a_rules(o: usize) {
    write_u16_at(o, 0x3c, 1); // um turno
    write_u8_at(o, 0xc2, 3); // cqi7: the Serie A points per win
    write_u8_at(o, 0xc3, 1); // pontos por empate
    write_u8_at(o, 0xc4, 2); // match category: league
    write_u8_at(o, 0x42, 1); // national club
    write_u8_at(o, 0xc5, 3); // tiebreakers: head-to-head, goal difference, none
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, 4); // top places
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 4); // rebaixados
    write_u32_at(o, 0x1c, 0xffff_ffff); // nenhuma liga acima
    write_u8_at(o, 0x4a, 3); // substitutions
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION as *const u32)); // relegated to Série B
    write_u16_at(o, 0xd9, 8); // bit 0x8: the top ones go to the next stage (no promotion, no champion band)
    write_u8_at(o, 0x52, 2); // fixed squad numbering
    write_u8_at(o, 0x49, 7); // substitutes on the bench
}

/// Writes the Série B rule fields (`bra_nat_second_set_up_rules`).
///
/// [pt] As regras da Série B (0x42cb70).
pub(crate) unsafe fn set_serie_b_rules(o: usize) {
    write_u16_at(o, 0x3c, 1);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1); // cqi7: the Serie B points per draw
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, 4); // quatro sobem
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 4); // quatro caem
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION as *const u32)); // goes up to Série A
    write_u16_at(o, 0xd9, 2); // bit 0x2: the top places are promotion places
    write_u8_at(o, 0x52, 2);
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION as *const u32)); // drops to Série C
    write_u8_at(o, 0x49, 7);
}

/// Writes the Série C rule fields (`bra_nat_third_set_up_rules`).
///
/// [pt] As regras da Série C (0x42de50).
pub(crate) unsafe fn set_serie_c_rules(o: usize) {
    write_u16_at(o, 0x3c, 2); // cqi7: the Serie C round-robin legs
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals for, away goals
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xbe, 2);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 2);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0x52, 2);
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION as *const u32)); // promoted to Série B
    write_u32_at(o, 0x20, 0xffff_ffff); // nenhuma liga abaixo
    write_u16_at(o, 0xd9, 8);
    write_u8_at(o, 0x49, 7);
}

/// Series A: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no league above, drops
/// to Series B; 4 top places that, by the rule word 8, lead to the next stage (the play-offs; no promotion and no
/// champion band: the champion comes from the knockout) [inference: the next stage is the slot 11 one]; 4 relegated; 7
/// substitutes on the bench and 3 substitutions; fixed squad numbers; category 2 (league) matches. Then the main stage
/// schedule
/// Original: `BraNatFirst::set_up_rules` (0x42a5a0, bra_nat_first.cpp, thiscall).
///
/// [pt] { 0x42a5a0 } (thiscall, bra_nat_first.cpp): `BraNatFirst::set_up_rules` (slot 36), as regras da Série A e o
/// calendário da fase principal; devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_set_up_rules(this: usize) -> u32 {
    set_serie_a_rules(this);
    crate::comparison::branch("cqi_7 serie a rules");
    create_main_schedule(League(this));
    1
}

/// Staff_Records add_qualified_for_round_record (club, comp, stage, round, weight).
///
/// [pt] Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, todos já em Rust):
/// 0x86bdd0 classificou-se (clube, comp, fase, rodada, peso; ret 0x14).
pub(crate) unsafe fn bra_nat_first_add_qualified_record(club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    exe::staff_records_add_qualified_for_round_record(read_usize(G_STAFF_RECORDS), club, comp, stage, round, weight);
}

/// Staff_Records add_relegation_record (club, league).
///
/// [pt] 0x86a1c0 rebaixado (clube, liga; ret 8).
pub(crate) unsafe fn bra_nat_first_add_relegation_record(club: usize, comp: usize) {
    exe::staff_records_add_relegation_record(read_usize(G_STAFF_RECORDS), club, comp);
}

/// Staff_Records add_league_champions_record (club, league).
///
/// [pt] 0x868c50 campeão da liga (clube, liga; ret 8).
pub(crate) unsafe fn add_champions_record(club: usize, comp: usize) {
    exe::staff_records_add_league_champions_record(read_usize(G_STAFF_RECORDS), club, comp);
}

/// Staff_Records add_promotion_record (club, league, weight).
///
/// [pt] 0x869480 promovido (clube, liga, peso; ret 0xc).
pub(crate) unsafe fn bra_nat_first_add_promotion_record(club: usize, comp: usize, weight: u32) {
    exe::staff_records_add_promotion_record(read_usize(G_STAFF_RECORDS), club, comp, weight);
}

/// League::find_club_row(club, 0): the club's row in the league table.
///
/// [pt] 0x68ab70 `League::find_club_row(clube, 0)` (thiscall, ret 8): a linha do clube na tabela da liga.
pub(crate) unsafe fn bra_nat_first_find_club_row(league: usize, club: usize) -> usize {
    exe::league_find_club_row(league, club, 0)
}

/// Série A update_staff_records_for_status (`bra_nat_first_update_staff_records_for_status`); fazer = false is the comparer's dry run → (return, row written,
/// destination).
///
/// [pt] A Série A (0x42abc0). `fazer` = falso: o ensaio do comparador, sem os registros nem a gravação; devolve
/// (devolução, linha gravada ou 0, destino gravado).
pub(crate) unsafe fn serie_a_staff_records(this: usize, club: usize, status: u32, stage: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8);
    let comp = read_usize(this + 4);
    if fa == 0xff {
        // the round-robin stage
        if st == 6 {
            if fazer {
                crate::comparison::branch("cqj_9 serie a qualified");
                bra_nat_first_add_qualified_record(club, comp, 0x41c, 0, 0x28);
            }
        } else if st == 3 && fazer {
            bra_nat_first_add_relegation_record(club, comp);
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    // o mata-mata
    if st == 1 {
        let l = bra_nat_first_find_club_row(this, club);
        if fazer {
            crate::comparison::branch("cqj_9 serie a champion");
            add_champions_record(club, comp); // cqj9: the Serie A champion record
            *((l + 0x37) as *mut u8) = 5;
        }
        return (0, l, 5);
    }
    if st == 2 || st == 0xff {
        let l = bra_nat_first_find_club_row(this, club);
        if fazer {
            *((l + 0x37) as *mut u8) = 0xfe; // eliminado
        }
        return (0, l, 0xfe);
    }
    (0, 0, 0)
}

/// Série C update_staff_records_for_status (`bra_nat_third_update_staff_records_for_status`), like Série A's.
///
/// [pt] A Série C (0x42eb00); como a da Série A.
pub(crate) unsafe fn serie_c_staff_records(this: League, club: usize, status: u32, stage: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if fa < 7 {
        // the groups
        if st == 6 && fazer {
            bra_nat_first_add_qualified_record(club, comp, 0x14, 0, 0x28);
        }
        return (0, 0, 0);
    }
    if fa == 7 {
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            bra_nat_first_add_qualified_record(club, comp, 0, 0x1e, 0x28);
        }
        return (0, 0, 0);
    }
    if fa < 10 {
        // the final round-robin of four: the two promoted
        if st != 0 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            bra_nat_first_add_promotion_record(club, comp, 0x32);
            bra_nat_first_add_qualified_record(club, comp, 0, 0x96, 0x28);
        }
        // the club’s row in the league (k = -1) or in stages 0..6, the first that has the club, gets destination 0
        let mut k: i8 = -1;
        let mut achada = 0;
        while k < 7 {
            let league = League(if k == -1 { this.addr() } else { read_usize(this.stages() + 4 * k as i32 as usize) });
            let n = league.club_count();
            if n > 0 {
                let t = league.table();
                let mut i: i8 = 0;
                while (i as i16) < n {
                    let r = t.wrapping_add(0x3b * i as i32 as usize);
                    if read_usize(r) == club {
                        achada = r;
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if achada != 0 {
                    break;
                }
            }
            k += 1;
        }
        if achada != 0 && fazer {
            crate::comparison::branch("cqj_9 serie c promoted");
            *((achada + 0x37) as *mut u8) = 0; // cqj9: the destination of those promoted from Serie C
        }
        return (0, achada, 0);
    }
    if fa == 10 {
        // a final
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            add_champions_record(club, comp);
        }
    }
    (0, 0, 0)
}

/// Baiano update_staff_records_for_status (`bra_reg_bahia_update_staff_records_for_status`), like Série A's but the club row comes from the table (+0xb1).
///
/// [pt] O Baiano (0x430210); como a da Série A. O clube vem da tabela (+0xb1, +0x3e linhas), não da 0x68ab70.
pub(crate) unsafe fn baiano_staff_records(this: League, club: usize, status: u32, stage: u32, game: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8);
    let comp = this.competition().addr();
    if fa == 0xff {
        if st == 6 {
            if fazer {
                bra_nat_first_add_qualified_record(club, comp, 0, 0x82, 0x32);
            }
        } else if st == 3 && fazer {
            crate::comparison::branch("cqj_9 baiano relegated");
            bra_nat_first_add_relegation_record(club, comp); // cqj9: the club relegated from the Baiano
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let n = this.club_count() as i32;
    let mut i = 0i32;
    while i < n {
        if read_usize(this.table() + 0x3b * i as usize) == club {
            if st == 1 {
                if fazer {
                    add_champions_record(club, comp);
                }
                let l = this.table() + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (0, l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // simplification: the original pushes the whole registers (eax with the caller’s garbage above the
                    // word, ecx with the upper half of the table pointer); 0x4bf850 and 0x86c000 only read the
                    // words (0x4b0870 / `as u16`): the words go.
                    let weight = (exe::comp_util_scale_by_round(0x1e, 0x32, read_i16(game + 0x34) as u16 as u32, 2) as u32);
                    exe::staff_records_add_knocked_out_record(read_usize(G_STAFF_RECORDS), club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
                }
                let l = this.table() + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (0, l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0, 0)
}

/// Series A: at the end of the round robin, the clubs through to the knockout get the qualification record (stage
/// 0x41c, weight 0x28) and the relegated the relegation record; in the knockout, the champion gets the champion record
/// and fate 5 in the table, the eliminated fate -2
/// Original: `BraNatFirst::update_staff_records_for_status` (0x42abc0, bra_nat_first.cpp, thiscall).
///
/// [pt] { 0x42abc0 } (thiscall, ret 0x18, bra_nat_first.cpp): `BraNatFirst::update_staff_records_for_status` (slot 18):
/// na fase de pontos corridos, os 8 classificados (situação 6) ganham o registro de classificação e os rebaixados (3) o
/// de rebaixamento; no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5, os eliminados (2 ou
/// -1) o destino -2. Devolve 0.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    serie_a_staff_records(this, club, status, stage, true).0
}

