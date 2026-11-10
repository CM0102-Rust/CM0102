use super::*;

/// The J1 has two stages (0 and 1, each a round-robin league) and the final (stage 2): the next stage moves the current
/// stage on and, at the end of the second stage, builds the final; past the last stage it is an error (the box)
/// Original: `JapJ1::advance_stage` (0x6639c0, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x6639c0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::advance_stage()`, a fase seguinte da J1 (dois turnos, as fases 0
/// e 1, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x223 e nada mais. Senão
/// +0x30 sobe um e, ao chegar a 2, monta a final (0x663a50, call direto com o ecx da liga).
pub(crate) unsafe extern "thiscall" fn jap_j1_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9eabc8, 0x223);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 2 {
        exe::japj1_create_championship_final_stage(this);
    }
}

/// The J1 championship final: the first-stage winner against the second-stage winner (the club in the first row of each
/// stage's table), over two games [inference: home and away], with no seeds; unlike the Brazilian state leagues, it
/// never checks for the same club (the final is built anyway).
///
/// `JapJ1::create_championship_final_stage()`: the list of 2 (malloc `0x944e46` of 8; out of memory: the box of line
/// `0x245` and it returns); virtual `+0x3c` (create_round_table: thiscall(this, stage 2, &rounds, &extra, 0), ret
/// `0x10`); the new stage (operator new `0x944cf1` of `0xb2`; Cup_Stage::Cup_Stage `0x522e00`, thiscall, ret `0x3c`:
/// the league, 2, the list, the rounds, `+4`, the table, the year `+0x40`, 2, 1, the extra, `0x14`, 0, 0, 0 and no
/// seed) in the third stage slot (`[+0xc]+8`); the list and the table freed (free `0x9452ca`, in that order);
/// Cup::set_up_round (`0x51c800`, thiscall(stage, 0), ret 4) called with the third slot's stage even when null (as in
/// the original); with no stage, the box of line `0x25c`; with it, `+0x30` = 2. The original's SEH frame only covers
/// the constructor (no destructor).
/// Original: `JapJ1::create_championship_final_stage` (0x663a50, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x663a50 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_championship_final_stage()`, a final da J1 entre os
/// campeões dos dois turnos (o clube da primeira linha da tabela de cada um), sempre (sem conferir se é o mesmo clube):
/// a lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x245 e volta); a virtual +0x3c
/// (create_round_table: thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10); a fase nova (operator new 0x944cf1 de
/// 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano +0x40,
/// 2, 1, o extra, 0x14, 0, 0, 0 e nenhuma semente) na terceira vaga das fases ([+0xc]+8); a lista e a tabela liberadas
/// (free 0x9452ca, nessa ordem); Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4) chamada com a fase da terceira
/// vaga mesmo nula (como na original); sem fase, a caixa da linha 0x25c; com ela, +0x30 = 2. O quadro SEH da original
/// só cobre o construtor (sem destrutor).
pub(crate) unsafe extern "thiscall" fn create_championship_final_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x9eabc8, 0x245);
    }
    write_usize(list, half_champion(this, 0)); // cqq9: the first-stage champion
    write_usize(list + 4, half_champion(this, 1)); // cqq9: the second-stage champion
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
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 8, stage);
    exe::crt_free(list);
    exe::crt_free(t);
    exe::cup_set_up_round(read_usize(this.stages() + 8), 0);
    if read_usize(this.stages() + 8) == 0 {
        return show_error_at(0x9eabc8, 0x25c);
    }
    crate::comparison::branch("cqq_9 final");
    this.set_current_stage((2) as usize);
}

/// The Japanese season turnover, led by the J1: first promotion and relegation with the J2; then the J1 is rebuilt with
/// three stages (the two halves and the final), next year, rules, the table and the fixtures; then the J2 turns over,
/// followed by the predicted positions, the board expectations and the referee renewal.
///
/// `JapJ1::start_new_season()`: `+0x4c` = 0; promotion and relegation (virtual `+0xb0`, thiscall(this, 1), ret 4); the
/// table freed (League::free_table `0x687970`, thiscall(this, 0), ret 4); the list at `+0xba` freed and cleared; with
/// statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`, thiscall(`+8`, 1), ret 4); with a current stage
/// ≥ 0, the existing stages 0..=`+0x30` destroyed (virtual 0 with 1); the stage array freed and a new one of 3 (malloc
/// `0x944e46` of `0xc`, `+0x2c` = 3; out of memory: the box of line `0x1b4`, the clean-up `0x5cc7d0`, `0x5a8f60` and
/// exit(−1) `0x945934`, all cdecl); the year (`+0x40`) goes up by one and `+0x30` = −1.
///
/// Then, each with its own box if it returns 0: the rules (virtual `+0x8c`, `0x1be`), the table (`0x6634e0`, `0x1c5`),
/// the fixtures (League::create_fixtures `0x6835c0`, `0x1cb`) and the order (League::sort_table `0x6827d0`,
/// thiscall(this, 0), `0x1d2`); then the J2 turnover (virtual `+8` of the competition with id `[0x9cf768]`), the
/// predicted positions (virtual `+0x5c`), the board exe::league_set_club_season_expectations(League::set_club_season_expectations `0x68aa80`) and
/// the referee renewal (OfficialsManager::renew_officials `0x79cee0`, thiscall(`[0xb74340]`, `+4`), ret 4), whose eax
/// it returns.
/// Original: `JapJ1::start_new_season` (0x6636d0, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x6636d0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::start_new_season()`, a virada do futebol japonês (a J1 comanda):
/// +0x4c = 0; o acesso e o rebaixamento (a virtual +0xb0, thiscall(this, 1), ret 4); a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada e zerada; com estatísticas (+8),
/// Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0, as fases 0..=+0x30
/// que existem destruídas (a virtual 0 com 1); o vetor das fases liberado e um novo de 3 (malloc 0x944e46 de 0xc,
/// +0x2c = 3; sem memória: a caixa da linha 0x1b4, a limpeza 0x5cc7d0, 0x5a8f60 e exit(−1) 0x945934, todas cdecl); o
/// ano (+0x40) sobe um e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras (virtual +0x8c, 0x1be), a
/// tabela (0x6634e0, 0x1c5), os jogos (League::create_fixtures 0x6835c0, 0x1cb) e a ordem (League::sort_table 0x6827d0,
/// thiscall(this, 0), 0x1d2); depois a virada da J2 (a virtual +8 da competição de id [0x9cf768]), as posições previstas
/// (virtual +0x5c), as expectativas das diretorias (League::set_club_season_expectations 0x68aa80) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
pub(crate) unsafe extern "thiscall" fn jap_j1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let virtual_ = |c: usize, desl: usize| -> u32 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + desl));
        f(c)
    };
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0xb0));
    cadeia(this.addr(), 1);
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
    this.set_stages(v);
    if v == 0 {
        show_error_at(0x9eabc8, 0x1b4);
        for f in [0x5cc7d0usize, 0x5a8f60] {
            let f: unsafe extern "C" fn() = core::mem::transmute(f);
            f();
        }
        exe::crt_exit(-1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // cqq9: the next year of the J1
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    crate::comparison::branch("cqq_9 j1 rollover");
    let error = if virtual_(this.addr(), 0x8c) == 0 {
        0x1be
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6634e0usize)(this.addr()) == 0 {
        0x1c5
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6835c0usize)(this.addr()) == 0 {
        0x1cb
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, usize) -> u32>(0x6827d0usize)(this.addr(), 0) == 0 {
        0x1d2
    } else {
        0
    };
    if error != 0 {
        show_error_at(0x9eabc8, error);
        return 0;
    }
    virtual_(read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2)), 8);
    virtual_(this.addr(), 0x5c);
    exe::league_set_club_season_expectations(this);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32 
}

/// True if the club's division (+0x57) is non-null and its id equals the competition id in global g.
///
/// [pt] O clube é da divisão do global `g`: a divisão do clube (+0x57) não é nula e o id do registro (+0) é o do global.
pub(crate) unsafe fn is_in_division(club: Club, g: usize) -> bool {
    let comp = club.division().addr();
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// Sets +0x3e = n and allocates the league table (n x 0x3b) at +0xb1; error box on out of memory; returns the table.
///
/// [pt] +0x3e = `n` e a tabela (malloc 0x944e46 de n × 0x3b, em +0xb1; sem memória: a caixa do fonte `arquivo` na linha
/// `linha`). Devolve a tabela (0 sem memória).
pub(crate) unsafe fn alloc_league_table(this: League, n: i16, file: usize, row: u32) -> usize {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
    }
    t
}

/// League::init_table_row (`init_table_row`: thiscall(league, row, club, 0, 0), ret 0x10) for row (k + `girar`) % n with the
/// club.
///
/// `girar` is 0 (the original: row k); it exists only for each league's mutations. Simplification: the original routine
/// pushes the row in eax with spare upper bits; `init_table_row` only reads the byte.
///
/// [pt] League::init_table_row (0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10). `girar` é 0 (o original: a linha
/// k); existe só para as mutações de cada liga. simplification: a original empurra a linha no eax com bits de cima de sobra;
/// a 0x687430 só lê o byte.
pub(crate) unsafe fn jap_j1_init_table_row(this: League, k: u8, n: i16, girar: u8, club: usize) {
    exe::league_init_table_row(this, (k as u32 + girar as u32) % n as u32, club, 0, 0);
}

/// Japanese loop: every club of the division fills the next row while rows remain; the rest lose their division (+0x57
/// = 0); returns 1 if full.
///
/// [pt] O laço do Japão: percorre todos os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles); cada clube da divisão do
/// global `g`, na ordem do banco, entra na linha seguinte enquanto houver linha; os que sobram perdem a divisão
/// (+0x57 = 0). Devolve 1 se a tabela encheu.
pub(crate) unsafe fn fill_japanese_table(this: League, g: usize, girar: u8, ramo: &'static str) -> u32 {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if is_in_division(Club(club), g) {
            if (k as i8 as i16) < this.club_count() {
                jap_j1_init_table_row(this, k, this.club_count(), girar, club);
                k = k.wrapping_add(1);
            } else {
                write_usize(club + club::DIVISION, 0);
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    crate::comparison::branch(ramo);
    1
}

/// Creates the J1 table: 16 clubs, those of the division in database order; those beyond 16 lose their division (left
/// with no league); when full, the prize fund starts at 100,000 per place and TV pays 3,000,000 to each club.
///
/// `JapJ1::create_table()`: J1 has 16 clubs (`+0x57` = `[0x9cf65c]`; the rest lose the division); once full, the prize
/// fund with the 100,000 share (League::calc_prize_fund `0x682f70`: push `0x186a0`; mov ecx, edi -> thiscall, ret 4) at
/// `+0x28`, `+0xe1` = 1, the TV money of 3,000,000 to each club (League::pay_tv_rights `0x683010`: push 0; push
/// `0x2dc6c0`; mov ecx, edi -> thiscall, ret 8; it sends the TV news) and returns 1. Source `0x9eabc8`, line `0x148`.
/// Original: `JapJ1::create_table` (0x6634e0, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x6634e0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_table()`: a J1 tem 16 clubes (+0x57 = [0x9cf65c]; os que
/// sobram perdem a divisão); cheia, o fundo de prêmios com a cota 100.000 (League::calc_prize_fund 0x682f70: push
/// 0x186a0; mov ecx, edi -> thiscall, ret 4) em +0x28, +0xe1 = 1, a TV 3.000.000 a cada clube (League::pay_tv_rights
/// 0x683010: push 0; push 0x2dc6c0; mov ecx, edi -> thiscall, ret 8; manda a notícia da TV) e 1. Fonte 0x9eabc8, linha
/// 0x148.
pub(crate) unsafe extern "thiscall" fn jap_j1_create_table(this: usize) -> u32 {
    let this = League(this);
    if alloc_league_table(this, 0x10, 0x9eabc8, 0x148) == 0 {
        return 0;
    }
    if fill_japanese_table(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0, "cqm_9 j1 cheia") == 0 {
        return 0;
    }
    let f = exe::league_calc_prize_fund(this, 0x186a0);
    this.set_prize_fund(f);
    this.set_prize_mode(1);
    exe::league_pay_tv_rights(this, 0x2dc6c0, 0);
    1
}

/// The comparer of the three: the original routine allocates the table (and the J1 pays the TV money with a news item),
/// so only it runs.
///
/// The plan, made beforehand: the division's clubs in database order (all of them). Afterwards it checks: +0x3e = `n`,
/// the return value = 1 if the plan filled (otherwise 0), the table rows (+0 the club, 0x3b bytes) with the first `n`
/// of the plan, in order; in Japan (`limpar`), the leftovers of the plan with their division zeroed; in a full J1,
/// +0xe1 = 1 and +0x28 = the quota fund (prize fund).
///
/// [pt] O comparador dos três: a original aloca a tabela (e a J1 paga a TV com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (todos). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0), as
/// linhas da tabela (+0 o clube, 0x3b bytes) com os primeiros `n` do plano, na ordem; no Japão (`limpar`), os que
/// sobram do plano com a divisão zerada; na J1 cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn compare_japanese_table(name: &str, this: League, n: i16, g: usize, clear: bool, quota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let club = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if is_in_division(Club(club), g) {
            plan.push(club);
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let cabem = plan.len().min(n as usize);
    let expected = (plan.len() >= n as usize) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..cabem).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found[..] == plan[..cabem];
        if clear {
            same = same && plan[cabem..].iter().all(|&c| read_usize(c + club::DIVISION) == 0);
        }
        if r == 1 && quota != 0 {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, quota);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// J1: double round robin (the two stages of the year, created by add_result 0x663940 [inference]); win 3, draw 1;
/// tie-break by goal difference, goals scored and away goals; no top places in the overall table (rule word 8, no
/// champion band: the champion comes from the final between the stage winners [inference]); 2 go down to J2; no league
/// above; 5 bench substitutes and 3 substitutions; fixed squad numbers; the word +0xab = 0x82 copied to each match.
/// Then the schedule
/// Original: `JapJ1::set_up_rules` (0x663620, jap_j1.cpp, thiscall).
///
/// [pt] { 0x663620 } (thiscall, jap_j1.cpp): `JapJ1::set_up_rules`, as regras da J1 e o calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn jap_j1_set_up_rules(this: usize) -> u32 {
    j1_rules(this);
    crate::comparison::branch("cqj_7 j1 rules");
    create_main_schedule(League(this));
    1
}

/// One half of the J1: when the first game of a half is processed, it builds that half's round-robin stage (a league
/// with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and
/// one top place, the half winner's); the first half goes to stage 0 and the second (from round 16, by the match +0x34)
/// to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `JapJ1::create_half_season_stage` (0x663c20, jap_j1.cpp, thiscall, ret 4).
///
/// [pt] { 0x663c20 } (thiscall, ret 4, jap_j1.cpp): `JapJ1::create_half_season_stage(k)`, um turno da J1 (a caixa da linha
/// 0x278; o 20º argumento do construtor é 0xf).
pub(crate) unsafe extern "thiscall" fn jap_j1_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(League(this), k, 0x9eabc8, 0x278, 0xf, 0xc2, "cqr_1 turno da j1") // cqr1: the points per win in the J1
}

