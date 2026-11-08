use super::*;

/// who takes set piece `tipo` (0..4) on `lado` (0..1): the cell (dword) in the club's tactics row (0x8982b0) at +0x601
/// + row × 0x91 + 0x5e + (side + type × 2) × 4. Above [0xae2364] + 0x52d it becomes −1; negative, 0; a person (0x6e
/// bytes at [0xae23bc]) without +0x61 clears the cell; from the club (+0x39), in a national team (0x53b900) or with
/// link 0x775790 they are returned; otherwise it clears. Null club, type or side out of range: the original (error
/// box). Compared side by side (the cell)
/// Original: `stored_taker` (0x897990, tactics.cpp (544 bytes, called by the taker choice 0x6ceb90), thiscall, ret 0x10).
///
/// [pt] { 0x897990 } (ret 0x10, tactics.cpp): quem bate a bola parada `tipo` (0..=4) no lado `lado`
/// (0..=1) pelo clube: a celula (dword) na linha tatica do clube (0x8982b0 com `humano`) em +0x601 +
/// linha x 0x91 + 0x5e + (lado + tipo x 2) x 4. Acima de [0xae2364] + 0x52d a celula vira -1 (0);
/// negativa, 0. A pessoa (0x6e bytes em [0xae23bc]) sem +0x61 apaga a celula (0); do clube (+0x39),
/// numa selecao (0x53b900) ou com o vinculo das selecoes (0x775790 em [0xb74318]) e devolvida; senao
/// a celula vira -1 (0). Clube nulo, tipo ou lado fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn stored_taker(this: usize, club: usize, kind: u32, side: u32, human: u32) -> usize {
    let (t, l) = (kind as u8 as i8, side as u8 as i8);
    if club == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(IDX_STORED_TAKER)(this, club, kind, side, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    if k == -1 {
        crate::comparison::branch("batedor sem linha");
        return 0;
    }
    let cell = (*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91 + 0x5e + (l as i32 + t as i32 * 2) as usize * 4) as *mut i32;
    let v = *cell;
    if v > (*(G_STAFF_COUNT as *const i32)).wrapping_add(0x52d) {
        crate::comparison::branch("batedor alem das pessoas");
        *cell = -1;
        return 0;
    }
    if v <= -1 {
        crate::comparison::branch("batedor vazio");
        return 0;
    }
    let person = (*(G_STAFF_TABLE as *const usize)).wrapping_add(v as usize * staff::SIZE);
    if *((person + staff::PLAYER) as *const u32) == 0 {
        crate::comparison::branch("batedor sem ficha");
        *cell = -1;
        return 0;
    }
    if *((person + staff::CLUB_JOB) as *const usize) == club {
        crate::comparison::branch("batedor do clube");
        return person;
    }
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if national_team(club) != 0 || exe::nationalteams_is_national_or_all_star_team(*(G_NATIONAL_TEAMS as *const usize), club) != 0 {
        crate::comparison::branch("batedor de fora do clube");
        return person;
    }
    crate::comparison::branch("batedor saiu do clube");
    *cell = -1;
    0
}

/// Saves or restores the club's 20 instructions (dwords of the tactics row + 2) in one of 16 slots of 0x5c bytes
/// (+0x30; used count at +0x5f0), the current human's (`[0xb74c26 + [0xb77c7a] × 0x300]`; `[0xae2364]` − id − 1); when
/// saving, the instructions become −1. Compared side by side (the slots and the row).
///
/// Ret 0x10: saves (`restore` 0) or restores (`restore` != 0) the club's 20 instructions (dwords at +0x601 + row × 0x91
/// + 2, the row from 0x8982b0 with `person`) in one of the 16 spaces of 0x5c bytes starting at +0x30 (club +0x30,
/// person +0x34, used +0x38, instructions +0x3c; used count at +0x5f0). The space is that of the human whose turn it is
/// (`[0xb74c26 + [0xb77c7a] × 0x300]`): `[0xae2364]` - his id - 1, in 0..16. Saving (with `use`), marks the space and
/// copies; then the 20 instructions become -1. Restoring with `use`, the space of the same club and person goes back to
/// the table and is freed; without `use`, frees all the spaces of the club and person. Null club: the original routine
/// (error box); row -1: 0; otherwise 1.
/// Original: `store_instructions` (0x896fe0, tactics.cpp (624 bytes, called by the engine step 0x6b98c0 and others), thiscall, ret 0x10).
///
/// [pt] { 0x896fe0 } (ret 0x10, tactics.cpp): guarda (`volta` 0) ou devolve (`volta` != 0) as 20
/// instrucoes (dwords em +0x601 + linha x 0x91 + 2, linha da 0x8982b0 com `pessoa`) do clube num dos
/// 16 espacos de 0x5c bytes a partir de +0x30 (clube +0x30, pessoa +0x34, ocupado +0x38, instrucoes
/// +0x3c; ocupados em +0x5f0). O espaco e o do humano da vez ([0xb74c26 + [0xb77c7a] x 0x300]):
/// [0xae2364] - id dele - 1, em 0..16. Guardando (com `usar`), marca o espaco e copia; depois as 20
/// instrucoes viram -1. Devolvendo com `usar`, o espaco do mesmo clube e pessoa volta para a tabela
/// e e liberado; sem `usar`, libera todos os espacos do clube e pessoa. Clube nulo: a original (caixa
/// de erro); linha -1: 0; senao 1.
pub(crate) unsafe extern "thiscall" fn store_instructions(this: usize, club: usize, person: u32, usar: u32, volta: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_STORE_INSTRUCTIONS)(this, club, person, usar, volta);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, person);
    if k == -1 {
        return 0;
    }
    let instrucao = |i: usize| (*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91 + 2 + 4 * i) as *mut u32;
    let ocupados = (this + tactics_fields::UNDO_COUNT) as *mut u8;
    // the space of the current human, if any
    let espaco = || {
        let human = *((G_SCREEN_RECORDS_MANAGER + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize * 0x300) as *const usize);
        if human == 0 {
            return None;
        }
        let e = (*(G_STAFF_COUNT as *const i32)).wrapping_sub(*(human as *const i32)).wrapping_sub(1);
        if (0..16).contains(&e) { Some(this + e as usize * 0x5c) } else { None }
    };
    let u = |p: usize, o: usize| (p + o) as *mut u32;
    if volta == 0 {
        if usar != 0 {
            if let Some(s) = espaco() {
                crate::comparison::branch("instructions guardadas");
                if *u(s, 0x38) == 0 {
                    *ocupados = (*ocupados).wrapping_add(1);
                }
                *u(s, 0x30) = club as u32;
                *u(s, 0x34) = person;
                *u(s, 0x38) = 1;
                for i in 0..20 {
                    *u(s, 0x3c + 4 * i) = *instrucao(i);
                }
            }
        }
        crate::comparison::branch("instructions apagadas");
        for i in 0..20 {
            *instrucao(i) = u32::MAX;
        }
        return 1;
    }
    if usar != 0 {
        if let Some(s) = espaco() {
            if *u(s, 0x38) != 0 && *u(s, 0x30) == club as u32 && *u(s, 0x34) == person {
                crate::comparison::branch("instructions devolvidas");
                for i in 0..20 {
                    *instrucao(i) = *u(s, 0x3c + 4 * i);
                }
            }
            if *u(s, 0x38) != 0 {
                *ocupados = (*ocupados).wrapping_sub(1);
            }
            *u(s, 0x38) = 0;
        }
        return 1;
    }
    if *ocupados != 0 {
        for e in 0..16 {
            let s = this + e * 0x5c;
            if *u(s, 0x38) != 0 && *u(s, 0x30) == club as u32 && *u(s, 0x34) == person {
                crate::comparison::branch("instructions liberadas");
                *u(s, 0x38) = 0;
                *ocupados = (*ocupados).wrapping_sub(1);
            }
        }
    }
    1
}

/// copies the club's tactics row (0x91 bytes at +0x601, row from 0x8982b0) to the destination. Null club: the original
/// (error box); row -1: 0. Compared side by side
/// Original: `read_tactics_row` (0x8983f0, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8983f0 } (ret 0xc, tactics.cpp): copia a linha tatica do clube (0x91 bytes em +0x601 + linha
/// x 0x91, linha da 0x8982b0 com `humano`) para `destino`. Clube nulo: a original (caixa de erro);
/// linha -1: 0; senao 1.
pub(crate) unsafe extern "thiscall" fn read_tactics_row(this: usize, club: usize, target: *mut u8, human: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(IDX_READ_TACTICS_ROW)(this, club, target, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    if k == -1 {
        return 0;
    }
    crate::comparison::branch("linha tatica lida");
    core::ptr::copy((*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91) as *const u8, target, 0x91);
    1
}

/// the reverse, writes the 0x91 bytes into the row (row -1 or beyond the total +0x605: 0). Called by 0x6dbb10
/// (`store_tactic`). Compared side by side
/// Original: `write_tactics_row` (0x8984d0, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8984d0 } (ret 0xc, tactics.cpp): o inverso da 0x8983f0, grava `origem` (0x91 bytes) na linha
/// tatica do clube. Linha -1 ou alem do total (+0x605): 0.
pub(crate) unsafe extern "thiscall" fn write_tactics_row(this: usize, club: usize, source: *const u8, human: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(IDX_WRITE_TACTICS_ROW)(this, club, source, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    if k == -1 || k >= *((this + tactics_fields::ROW_COUNT) as *const i32) {
        return 0;
    }
    crate::comparison::branch("linha tatica gravada");
    core::ptr::copy(source, (*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91) as *mut u8, 0x91);
    1
}

/// Memory regions of the tactics copy routines: the source/destination (0x91 bytes) and the club's tactics row, if any,
/// plus the RNG global.
///
/// [pt] Regioes das duas: o destino/origem e a linha do clube (se houver).
pub(crate) unsafe fn tactics_row_regions(this: usize, club: usize, p: usize, human: u32) -> Vec<(usize, usize)> {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    let mut reg = vec![(0xb67a34, 4), (p, 0x91)];
    if k != -1 && k < *((this + tactics_fields::ROW_COUNT) as *const i32) {
        reg.push(((*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91), 0x91));
    }
    reg
}

/// `0x6beeb0` (match_eng.cpp, called by the engine step 0x6b98c0) with the calls that change things outside the engine
/// injected (0x6d6f70 commentary, 0x6d0980 injures; and those of the player exit 0x6cda20): the match simulated without
/// highlights, in periods of two rounds (counter +0x8ed0, period code +0x8ed4), including extra time, penalties and
/// ratings.
///
/// Per period: the teams' strength (0x6c0c60) with four pairs limited between half and double side 1, and the side that
/// starts (random(2)).
///
/// Per round, for the side in turn: three attacks (the third only if the side wins the strength draw) with a duel of
/// one active player (+0xc5 + 20 against the other's strength); the second duel (0x6c2880, +0xdd + 15) and the third
/// (+0xad + 15); the wear (0x6c2750, +0xb9 + 20); the other side's cards (sending-off 0x6c2310 mode 4, yellow mode 2,
/// the second yellow sends off; more than 4 sent off and the match is abandoned, 3 × 0); the incident type; the score
/// factor (difference, 1, 0.65 after 60 minutes or 0.75), the contest (0x6c06b0/0x6c07a0 or 0x6c0a70) and, with an
/// advantage, the shot (0x6c23f0, 0x6c0890/0x6c0980) and the goal (0x6c2050; goalkeepers 0x717410; comments 0x1fcc or
/// 0x1fcf, or 0x1fce on a missed penalty).
///
/// At minutes 45, 90, 105 and 120 it writes the scores (+0x45, +0x43, +0x47) and decides extra time, golden goal (+0x38
/// & 0x80) and penalties (+0x49/+0x4a, random(5)); the aggregate (+0x4b/+0x4c).
///
/// Afterwards: the home goal (random(4) with the score level), the injuries of the last side in turn (0x6d0980), the
/// ratings (friendly −2: adjustment by the club's reputation) and the counters +0xf5e2/+0xf618.
///
/// [pt] { 0x6beeb0 } (match_eng.cpp, chamada pelo passo do motor 0x6b98c0) com as chamadas que mexem fora
/// do motor injetadas (0x6d6f70 narra, 0x6d0980 lesiona; e as da saida do jogador 0x6cda20): a
/// partida simulada sem lances, em periodos de duas rodadas (contador +0x8ed0, codigo do periodo
/// +0x8ed4). Por periodo: a forca dos times (0x6c0c60) com quatro pares limitados entre metade e o
/// dobro do lado 1, e o lado que comeca (sorteio(2)). Por rodada, do lado da vez: tres ataques (o
/// terceiro so se o lado vencer o sorteio de forca) com duelo de um jogador ativo (+0xc5 + 20 contra
/// a forca do outro); o segundo duelo (0x6c2880, +0xdd + 15) e o terceiro (+0xad + 15); o desgaste
/// (0x6c2750, +0xb9 + 20); os cartoes do outro lado (expulsao 0x6c2310 modo 4, amarelo modo 2, o
/// segundo amarelo expulsa; mais de 4 expulsos e abandono, 3 x 0); o tipo do lance; o fator do placar
/// (diferenca, 1, 0,65 depois dos 60 ou 0,75), a disputa (0x6c06b0/0x6c07a0 ou 0x6c0a70) e, com
/// vantagem, o chute (0x6c23f0, 0x6c0890/0x6c0980) e o gol (0x6c2050; goleiros 0x717410; narra
/// 0x1fcc ou 0x1fcf, ou 0x1fce no erro do penalti). Nos minutos 45, 90, 105 e 120 grava os placares
/// (+0x45, +0x43, +0x47) e decide prorrogacao, gol de ouro (+0x38 & 0x80) e penaltis (+0x49/+0x4a,
/// sorteio(5)); o agregado (+0x4b/+0x4c). Depois: o gol do mando (sorteio(4) com placar igual), as
/// lesoes do ultimo lado da vez (0x6d0980), as notas (amistoso -2: ajuste pela reputacao do clube) e
/// os contadores +0xf5e2/+0xf618.
pub(crate) unsafe fn quick_simulation_with(m: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |o: usize| (m + o) as *mut i16;
    let b = |o: usize| (m + o) as *mut u8;
    let more = |p: *mut u8| *p = (*p).wrapping_add(1);
    let mais16 = |p: *mut i16| *p = (*p).wrapping_add(1);
    let word = |o: usize, side: usize| *((m + o + 2 * side) as *const i16) as i32;
    let jog = |side: usize, i: i8| (m as isize + 0x4796 + (i as isize + side as isize * 20) * 0x1be) as usize;
    let add = |p: usize, d: i16| {
        let q = (p + match_player::MATCH_RATING) as *mut i16;
        *q = (*q).wrapping_add(d);
    };
    let pt = || u(m, engine::MATCH);
    let pb = |o: usize| (u(m, engine::MATCH) + o) as *mut i8;
    let goals = |side: usize| *b(0xf5bd + side * 0x36) as i32;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, c: u8, side: usize| {
        call(FN_MATCH_EVENTS_ADD_EVENT, u(m, engine::COMMENTATOR), [code, m as u32, c as u32, side as u32, u32::MAX]);
    };
    // active goalkeepers (table 1) of the side concede the goal
    let goleiros = |side: usize| {
        for i in 0..20 {
            let q = jog(side, i);
            if byte(q, match_player::POSITION) >= 0 && player_table(q) == 1 {
                more((q + match_player::GOALS_CONCEDED) as *mut u8);
            }
        }
    };
    let mut end = false;
    let mut decides = false;
    let mut local_28 = 0u8;
    let mut side = 0usize;
    *w(engine::CLOCK) = 0;
    *w(engine::PERIOD_END) = 0x1ef;
    let penalties = 'periodo: loop {
        team_strength(m, 0);
        team_strength(m, 1);
        for o in [0x1da, 0x1de, 0x1e2, 0x1e6] {
            let (a, x) = (*w(o) as f64, *w(o + 2) as f64);
            let middle = x * read_f64_at(0x968968); // 0x968968 = 0.5
            if !(a >= middle) {
                *w(o) = ftol(middle) as i16;
            } else if a > x + x {
                *w(o) = ftol(x + x) as i16;
            }
        }
        let c7 = random(2) as u8;
        let mut round = 0u8;
        'rodadas: while *b(engine::GOLDEN_GOAL_SCORED) == 0 {
            side = if c7 != 0 { round as usize } else { (round == 0) as usize };
            let other = (side == 0) as usize;
            let strong = || random(word(0x1de, side) + word(0x1da, side)) > random(word(0x1da, other) + word(0x1de, other));
            // the three attacks (the third by strength)
            for k in 0..3 {
                if k >= 2 && !strong() {
                    continue;
                }
                mais16(w(0xf5e2 + side * 0x36));
                let mut t = 0u8;
                let i = loop {
                    let i = random(0x14) as i8;
                    t += 1;
                    if t >= 0x78 || byte(jog(side, i), 0x19) >= 0 {
                        break i;
                    }
                };
                if t >= 0x78 {
                    end = true;
                    break;
                }
                let p = jog(side, i);
                let f = (read_f32_at(p + match_player::PASSING) as f64 + read_f32_at(0x96afb0) as f64) as f32; // 0x96afb0 = 20
                let r = random((word(0x1da, other) + 200) / 19);
                if !(r as f64 >= f as f64) {
                    mais16(w(0xf5e4 + side * 0x36));
                    add(p, 0x28);
                } else {
                    add(p, -0x1e);
                }
            }
            if strong() {
                mais16(w(0xf5e6 + side * 0x36));
                let c = choose_by_role_b(m, side as u32);
                if c == 0xff {
                    end = true;
                    break 'rodadas;
                }
                let p = jog(side, c as i8);
                let f = (read_f32_at(p + match_player::TACKLING) as f64 + read_f32_at(0x96b020) as f64) as f32; // 0x96b020 = 15
                let r = random(word(0x1de, other) / 5);
                if !(r as f64 >= f as f64) {
                    add(p, 0x28);
                    mais16(w(0xf5e8 + side * 0x36));
                } else {
                    add(p, -10);
                }
            }
            if strong() {
                mais16(w(0xf5ea + side * 0x36));
                let mut t = 0u8;
                let i = loop {
                    let i = random(0x14) as i8;
                    t += 1;
                    if t >= 0x78 {
                        end = true;
                        break 'rodadas;
                    }
                    if byte(jog(side, i), 0x19) >= 0 {
                        break i;
                    }
                };
                let p = jog(side, i);
                let f = (read_f32_at(p + match_player::JUMPING) as f64 + read_f32_at(0x96b020) as f64) as f32; // 0x96b020 = 15
                let r = random((word(0x1f2, other) + 100) / 7);
                if !(r as f64 >= f as f64) {
                    mais16(w(0xf5ec + side * 0x36));
                    add(p, 0x14);
                } else {
                    add(p, -8);
                }
            }
            if random(10) == 0 {
                more(b(0xf5c8 + side * 0x36));
            }
            let p = jog(side, choose_by_role(m, side as u32) as i8);
            let v = ftol(read_f32_at(p + match_player::MOVEMENT) as f64 + read_f32_at(0x96afb0) as f64); // 0x96afb0 = 20
            if random(v) < random(2) {
                add(p, -0xf);
                more(b(0xf5ca + side * 0x36));
            }
            // the other side’s cards, or the type of the move
            local_28 = 0;
            let mut c20 = false;
            if random(word(0x1ea, other)) > random(0x640) {
                let mo = m + other * 0x36;
                more((mo + engine::SIDE_FOUL_TALLY) as *mut u8);
                let expulsar = |c: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
                    let p = jog(other, c as i8);
                    more((p + match_player::RED_CARDS) as *mut u8);
                    more((mo + engine::CARDS) as *mut u8);
                    player_left_pitch_with(m, p, 1, call);
                    team_strength(m, other as i8);
                    narrate(call, 0x1ff8, c, other);
                };
                if random(word(0x1ea, other)) > random(0x61a8) {
                    crate::comparison::branch("simulacao rapida expulsao");
                    let c = choose_by_condition(m, other as i8, 4);
                    add(jog(other, c as i8), -0xfa);
                    expulsar(c, call);
                } else if random(word(0x1ea, other)) > random(0x6a4) {
                    crate::comparison::branch("simulacao rapida give_yellow_card");
                    let c = choose_by_condition(m, other as i8, 2);
                    let p = jog(other, c as i8);
                    add(p, -0x32);
                    more((p + match_player::YELLOW_CARDS) as *mut u8);
                    more((mo + engine::FOULS) as *mut u8);
                    if byte(p, match_player::YELLOW_CARDS) == 2 {
                        crate::comparison::branch("simulacao rapida segundo give_yellow_card");
                        expulsar(c, call);
                    }
                    if *((mo + engine::CARDS) as *const u8) > 4 {
                        // abandonment: 3 x 0 for the side whose turn it is
                        crate::comparison::branch("simulacao rapida abandono");
                        end = true;
                        *b(0xf5bd + other * 0x36) = 0;
                        let g = b(0xf5bd + side * 0x36);
                        if *g <= 3 {
                            *g = 3;
                        }
                        let (x, y) = if side != 0 { (3, 0) } else { (0, 3) };
                        *pb(0x43) = x;
                        *pb(0x44) = y;
                        if *pb(0x4b) != -1 && *pb(0x43) as i32 + *pb(0x4b) as i32 == *pb(0x4c) as i32 + *pb(0x44) as i32 {
                            *pb(0x43) = (*pb(0x43)).wrapping_add(1);
                            more(b(engine::SCORE));
                        }
                        break 'rodadas;
                    }
                }
                if random(0x1e) != 0 {
                    c20 = true;
                    more((mo + engine::SET_PIECES) as *mut u8);
                } else {
                    local_28 = 1;
                }
            } else if random(word(0x1de, side)) > random(3000) {
                c20 = true;
                more(b(0xf5c7 + side * 0x36));
            }
            // the score factor
            let d = goals(side) - goals(other);
            let goal_difference = if side == 0 { *pb(0x4b) as i32 - *pb(0x4c) as i32 } else { *pb(0x4c) as i32 - *pb(0x4b) as i32 };
            let x = if goal_difference + d > 3 {
                d as f64
            } else if goals(side) + goals(other) != 0 {
                read_f32_at(0x968e70) as f64 // 0x968e70 = 1
            } else if *w(engine::CLOCK) > 0x3c {
                read_f32_at(0x96d320) as f64 // 0x96d320 = 0.6499999762
            } else {
                read_f32_at(0x96d248) as f64 // 0x96d248 = 0.75
            };
            let (v, repartir_outro): (f64, unsafe extern "thiscall" fn(usize, i8, i16)) = if c20 {
                (word(0x1f2, other) as f64 * x * read_f64_at(0x96d310), split_c) // 0x96d310 = 6.1
            } else {
                (word(0x1da, other) as f64 * x * read_f64_at(0x96d318), split_b) // 0x96d318 = 4.8
            };
            let r1 = random(ftol(v));
            let l24 = random(word(0x1de, side)).wrapping_sub(r1) as i16;
            if l24 > 0 {
                split_a(m, side as i8, 0x1c);
                repartir_outro(m, other as i8, -0x14);
            } else {
                split_a(m, side as i8, -6);
                repartir_outro(m, other as i8, 0x28);
            }
            // o chute
            if l24 > 0 && random(goals(1) + goals(0)) <= 3 {
                let ml = m + side * 0x36;
                if local_28 != 0 {
                    more((ml + engine::SIDE_SUMMARY) as *mut u8);
                }
                more((ml + 0xf5be) as *mut u8);
                let c8 = choose_by_role_e(m, side as u32, c20 as u32) as i8;
                let a = if c20 { random(word(0x1ee, side)) } else { random(word(0x1e2, side) * (local_28 as i32 + 1)) };
                let us = a as i16 as i32;
                if us + 0xa5 > random(0x172) + (0x168 - l24 as i32) {
                    more((ml + 0xf5bf) as *mut u8);
                    if c20 {
                        split_d(m, side as i8, a as i16);
                    } else {
                        split_e(m, side as i8, a as i16);
                    }
                    if us + 0x2d > random((7 - local_28 as i32 * 3) * word(0x1e6, other)) {
                        crate::comparison::branch("simulacao rapida gol");
                        let c9 = choose_by_role_c(m, side as u32, local_28 as u32);
                        let p = jog(side, c9 as i8);
                        more((p + match_player::GOALS) as *mut u8);
                        add(p, 0x1db);
                        goleiros(other);
                        if local_28 != 0 {
                            crate::comparison::branch("simulacao rapida gol de penalty");
                            narrate(call, 0x1fcf, c9, side);
                            more((ml + 0xf5c1) as *mut u8);
                            more((p + match_player::PENALTIES_TAKEN) as *mut u8);
                            more((p + match_player::PENALTIES_SCORED) as *mut u8);
                        } else {
                            narrate(call, 0x1fcc, c9, side);
                            if c20 {
                                let o = match random(4) {
                                    0 | 1 => 0xf5c3,
                                    2 => 0xf5c5,
                                    _ => 0xf5c4,
                                };
                                more((ml + o) as *mut u8);
                            }
                        }
                        if c8 > -1 {
                            let q = jog(side, c8);
                            more((q + match_player::ASSISTS) as *mut u8);
                            add(q, 0x15e);
                        }
                        more(b(0xf5c2 + other * 0x36));
                        more((ml + engine::SCORE) as *mut u8);
                        for (s, delta) in [(side, 0x4b), (other, -0x43)] {
                            for i in 0..20 {
                                let q = jog(s, i);
                                if byte(q, match_player::POSITION) >= 0 {
                                    add(q, delta);
                                }
                            }
                        }
                        if decides {
                            crate::comparison::branch("simulacao rapida gol de ouro");
                            *b(engine::GOLDEN_GOAL_SCORED) = 1;
                            break 'rodadas;
                        }
                    } else {
                        split_f(m, other as i8, 0xe1);
                        if local_28 != 0 {
                            crate::comparison::branch("simulacao rapida penalty perdido");
                            let c = choose_by_role_c(m, side as u32, local_28 as u32);
                            narrate(call, 0x1fce, c, side);
                            let p = jog(side, c as i8);
                            more((p + match_player::PENALTIES_TAKEN) as *mut u8);
                            add(p, -500);
                        }
                    }
                }
            }
            round += 1;
            if round >= 2 {
                break;
            }
        }
        // the end of the period
        mais16(w(engine::CLOCK));
        let t = *w(engine::CLOCK);
        let w38 = *((pt() + 0x38) as *const u16);
        let igual_no_agregado = |a: usize, x: usize| *pb(a) as i32 + *pb(0x4b) as i32 == *pb(0x4c) as i32 + *pb(x) as i32;
        if end {
            let (a, x) = if t <= 0x5a { (0x43, 0x44) } else { (0x47, 0x48) };
            *pb(a) = goals(0) as i8;
            *pb(x) = goals(1) as i8;
            let desempata = if *pb(0x4b) != -1 { igual_no_agregado(0x43, 0x44) } else { *pb(0x43) == *pb(0x44) && w38 & 0xa3 != 0 };
            if desempata {
                *pb(a) = (*pb(a)).wrapping_add(1);
            }
            break 'periodo false;
        }
        if *b(engine::GOLDEN_GOAL_SCORED) != 0 {
            *pb(0x47) = goals(0) as i8;
            *pb(0x48) = goals(1) as i8;
            break 'periodo false;
        }
        match t {
            0x2d => {
                *w(engine::PERIOD_END) = 0x3de;
                *pb(0x45) = goals(0) as i8;
                *pb(0x46) = goals(1) as i8;
            }
            0x5a => {
                *pb(0x43) = goals(0) as i8;
                *pb(0x44) = goals(1) as i8;
                *w(engine::PERIOD_END) = 0x483;
                let agregado = *pb(0x4b) != -1;
                if (agregado && !igual_no_agregado(0x43, 0x44)) || (!agregado && *pb(0x43) != *pb(0x44)) {
                    break 'periodo false;
                }
                if w38 & 0x80 != 0 {
                    crate::comparison::branch("simulacao rapida prorrogacao com gol de ouro");
                    decides = true;
                    continue 'periodo;
                }
                if w38 & 2 != 0 {
                    crate::comparison::branch("simulacao rapida prorrogacao");
                    continue 'periodo;
                }
                if agregado && w38 & 4 == 0 && *pb(0x44) != *pb(0x4b) && w38 & 0x4000 == 0 {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            0x69 => *w(engine::PERIOD_END) = 0x528,
            0x78 => {
                *pb(0x47) = goals(0) as i8;
                *pb(0x48) = goals(1) as i8;
                if *pb(0x4b) != -1 {
                    if !igual_no_agregado(0x47, 0x48) || (w38 & 0x4000 != 0 && *pb(0x48) != *pb(0x4b)) {
                        break 'periodo false;
                    }
                } else if *pb(0x47) != *pb(0x48) {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            _ => {}
        }
    };
    if penalties {
        crate::comparison::branch("simulacao rapida penalties");
        random(5);
        random(5);
        *pb(0x49) = random(5) as i8;
        random(5);
        random(5);
        *pb(0x4a) = random(5) as i8;
        if *pb(0x49) == *pb(0x4a) {
            let o = if random(2) != 0 { 0x49 } else { 0x4a };
            *pb(o) = (*pb(o)).wrapping_add(1);
        }
    }
    // o agregado
    if *pb(0x4b) != -1 {
        let (a, x) = if *pb(0x47) != -1 { (0x47, 0x48) } else { (0x43, 0x44) };
        *pb(0x4b) = (*pb(0x4b)).wrapping_add(*pb(a));
        *pb(0x4c) = (*pb(0x4c)).wrapping_add(*pb(x));
    }
    // the home goal
    let c1 = u(pt(), 0x1c);
    if c1 != 0 && u(pt(), 0x14) == u(c1, club::DIVISION) && (*pb(0x43) as i32 + *pb(0x44) as i32) < 5 && *pb(0x43) == *pb(0x44) && random(4) == 0 {
        let (r0, r1) = (*((c1 + club::REPUTATION) as *const i16) as i32, *((u(pt(), 0x20) + 0x80) as *const i16) as i32);
        let s = if r0 > r1 { Some(0) } else if r0 + 0xfa < r1 { Some(1) } else { None };
        if let Some(s) = s {
            crate::comparison::branch("simulacao rapida gol do mando");
            more(b(0xf5be + s * 0x36));
            let c = choose_by_role_c(m, s as u32, local_28 as u32);
            let p = jog(s, c as i8);
            more((p + match_player::GOALS) as *mut u8);
            add(p, 0x1db);
            goleiros(1 - s);
            narrate(call, 0x1fcc, c, s);
        }
    }
    // the injuries (two passes over the last side of the turn)
    if *(pt() as *const i32) != -2 {
        for _ in 0..2 {
            for i in 0..20 {
                let p = jog(side, i);
                let person = u(p, match_player::PLAYER);
                if byte(p, match_player::POSITION) < 0 || person == 0 {
                    continue;
                }
                let f = read_f32_at(p + match_player::STAMINA);
                let mut cl = (byte(person, player_sheet::INJURY_PRONENESS) as u8).wrapping_mul(2).wrapping_add(5);
                if !(f >= read_f32_at(0x96d30c)) { // 0x96d30c = 6750
                    cl = cl.wrapping_add(0xf);
                } else if !(f >= read_f32_at(0x96d308)) { // 0x96d308 = 8000
                    cl = cl.wrapping_add(5);
                }
                if random(cl as i8 as i32) < random(400) {
                    continue;
                }
                let comp = u(pt(), 0x14);
                let league = u(G_FRIENDLY, 0);
                let ceiling = if comp == 0 || (league != 0 && is_friendly(league, comp) != 0) { 0x5b } else { 0x66 };
                let g = random(ceiling) as u8;
                let kind = match random(10) {
                    0 => 1,
                    1..=3 => 3,
                    4..=6 => 4,
                    7..=9 => 5,
                    _ => continue,
                };
                crate::comparison::branch("simulacao rapida lesao");
                call(FN_CM3_MATCH_ENGINE_INJURE_PLAYER, m, [kind, g as u32, p as u32, 0, 0]);
            }
        }
    }
    // the ratings
    for s in 0..2 {
        for i in 0..20i8 {
            let p = jog(s, i);
            let q = (p + match_player::MATCH_RATING) as *mut i16;
            if *(pt() as *const i32) == -2 && i < 11 {
                crate::comparison::branch("simulacao rapida nota do amistoso");
                let club = if s != 0 { u(pt(), 0x20) } else { u(pt(), 0x1c) };
                let a = (*((p + match_player::EFFECTIVE_ABILITY) as *const i16) as f64 - read_f64_at(0x968e30)) * read_f64_at(0x968ff8); // 0x968e30 = 40, 0x968ff8 = 50
                let r = *((club + club::REPUTATION) as *const i16) as f64 * read_f64_at(0x969018) - read_f64_at(0x96d300); // 0x969018 = 1.6, 0x96d300 = 6000
                let v = ftol(((a - r) * read_f64_at(0x96d2f8) + read_f64_at(0x967890)) * *q as f64) as i16; // 0x96d2f8 = 2e-05, 0x967890 = 1
                *q = v.clamp(0xdac, 0x2ee0);
            }
            let n = (ftol((*q as i32 + 0x1770) as f64 * read_f64_at(0x96a608) + read_f64_at(0x96d2f0)) as i8).clamp(1, 10); // 0x96a608 = 0.0004, 0x96d2f0 = 1.83
            *((p + match_player::MATCH_GRADE) as *mut i8) = n;
            *((p + match_player::STAMINA_CEILING) as *mut u32) = *((p + match_player::STAMINA) as *const u32);
        }
    }
    for (a, x) in [(0xf5e2, 0xf5e4), (0xf618, 0xf61a)] {
        if *w(a) < 10 {
            crate::comparison::branch("simulacao rapida contador reiniciado");
            *w(a) = 0x14;
            *w(x) = (random(6) + 12) as i16;
        } else if *w(a) == *w(x) {
            crate::comparison::branch("simulacao rapida contador igual");
            let r = random(3);
            *w(x) = (*w(x)).wrapping_add((-1i32).wrapping_sub(r) as i16);
        }
    }
}

/// Club's tactics row index (0x8982b0) and its start in the table at +0x601 (0x91 bytes per row), or None for row −1.
///
/// [pt] Linha tatica do clube (0x8982b0) e o inicio dela na tabela em +0x601 (0x91 bytes por linha), ou
/// `None` na linha -1.
pub(crate) unsafe fn tactics_row_of_club(this: usize, club: usize, human: u32) -> Option<usize> {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(club, human);
    (l != -1).then(|| (*((this + tactics_fields::ROWS) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)))
}

/// Returns the squad slot (0..19) of the club's tactics row whose person id matches the id stored at +0x52 of the row
/// (the designated player, captain per the function map [inference]); none: -1 with `menos_um`, else 0; no row: 0. A
/// null club calls the original (error box).
///
/// [pt] { 0x897590 } (tactics.cpp): a vaga (0..20) cuja pessoa (indice em +2 + 4 x vaga, no vetor de
/// 0xae23bc) tem o id guardado em +0x52 da linha; sem nenhuma: -1 com `menos_um`, senao 0. Linha
/// -1: 0. Clube nulo: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn marked_slot(this: usize, club: usize, human: u32, minus_one: u32) -> i8 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(IDX_MARKED_SLOT)(this, club, human, minus_one);
    }
    let Some(linha) = tactics_row_of_club(this, club, human) else { return 0 };
    let persons = *(G_STAFF_TABLE as *const usize);
    for vaga in 0..20usize {
        let i = *((linha + tactics_row::SELECTION + 4 * vaga) as *const i32);
        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == *((linha + tactics_row::CAPTAIN) as *const i32) {
            return vaga as i8;
        }
    }
    -((minus_one != 0) as i8)
}

/// Sets the captain in the club's tactic: stores in the tactics row (+0x52) the person index of slot `vaga` (0..19).
/// Returns 1 if the slot was filled, 0 if empty (-1) or the row is missing; a null club or bad slot goes to the
/// original (error box).
///
/// [pt] { 0x897760 } (tactics.cpp): guarda em +0x52 da linha o indice da pessoa da vaga (0..20); 1 se
/// havia alguem (-1 = vazia). Linha -1: 0. Clube nulo ou vaga fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn mark_slot(this: usize, club: usize, slot: i8, human: u32) -> u32 {
    if club == 0 || !(0..20).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(IDX_MARK_SLOT)(this, club, slot, human);
    }
    let Some(linha) = tactics_row_of_club(this, club, human) else { return 0 };
    let i = *((linha + tactics_row::SELECTION + 4 * slot as usize) as *const i32);
    if i == -1 {
        return 0;
    }
    *((linha + tactics_row::CAPTAIN) as *mut i32) = i;
    1
}

/// Returns the person (staff table 0xae23bc, 110 bytes each) referenced by the club's tactics row at +0x56 + 4 x `qual`
/// (0 or 1); 0 when the index is negative or the club has no row. Null club or `qual` out of range go to the original
/// (error box).
///
/// [pt] { 0x897dc0 } (tactics.cpp): a pessoa (vetor de 0xae23bc, 110 bytes cada) do indice em +0x56 +
/// 4 x `qual` (0 ou 1) da linha; indice negativo ou linha -1: 0. Clube nulo ou `qual` fora: a
/// original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn row_person(this: usize, club: usize, which: i8, human: u32) -> usize {
    if club == 0 || !(0..2).contains(&which) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_ROW_PERSON)(this, club, which, human);
    }
    let Some(linha) = tactics_row_of_club(this, club, human) else { return 0 };
    let i = *((linha + tactics_row::PLAYMAKER + 4 * which as usize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)
}

/// Returns the club's row in the tactics table: for a human-managed club with `humano` set, -1 for a second national
/// team, else the human manager's row after the clubs (staff count - manager id - 1, +15 for a national team; out of
/// range: the original); in other cases the club id.
///
/// [pt] { 0x8982b0 } (tactics.cpp; o `this` nao e usado, ret 8): a linha do clube na tabela tatica.
/// Clube dirigido por humano (0x600a50 com 1, 1) e `humano` ligado: segunda selecao, -1; senao a
/// linha do tecnico, depois dos clubes (0xae235c): quantidade de pessoas (0xae2364) - id do tecnico
/// (+0xcf) - 1, ou + 15 numa selecao (0x53b900), fora da faixa: a original (caixa de erro). Nos
/// outros casos, o id do clube.
pub(crate) unsafe extern "stdcall" fn tactics_row(club: usize, human: u32) -> i32 {
    let and_human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if and_human(club, 1, 1, 0, 0) != 0 && human != 0 {
        if exe::club_is_national_team(club) != 0 {
            return -1;
        }
        let manager = *((club + club::MANAGER) as *const usize);
        if manager == 0 {
            return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(club, human);
        }
        let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        let clubs = *(G_CLUB_COUNT as *const i32);
        let base = (*(G_STAFF_COUNT as *const i32)).wrapping_sub(*(manager as *const i32));
        let i = base.wrapping_add(if is_national_team(club) != 0 { 0xf } else { -1 }).wrapping_add(clubs);
        if i < 0 || i >= clubs.wrapping_add(0x20) {
            return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(club, human);
        }
        return i;
    }
    *(club as *const i32)
}

/// Returns byte `pos` (0..10) of the club's tactics row (index from 0x8982b0, for the human manager when `humano`) in
/// the table at +0x601 (0x91 bytes per row, from +0x86); row -1: 0. Null club or position out of range: falls back to
/// the original (error box).
///
/// [pt] { 0x898040 } (tactics.cpp): byte `pos` (0..=10) da linha tatica do clube (0x8982b0) na tabela
/// em +0x601 (0x91 bytes por linha, a partir de +0x86); linha -1: 0. Clube nulo ou posicao fora:
/// a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn tactics_byte(this: usize, club: usize, pos: i8, human: u32) -> u8 {
    if club == 0 || !(0..=10).contains(&pos) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8>(IDX_TACTICS_BYTE)(this, club, pos, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(club, human);
    if l == -1 {
        return 0;
    }
    let table = *((this + tactics_fields::ROWS) as *const usize);
    *((table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x86 + pos as usize) as *const u8)
}

/// Row of the tactic statistics table ([this], 0x28 bytes per tactic) used by the club: word +0 of the club's 0x91-byte
/// tactics row.
///
/// [pt] A linha da tabela de estatísticas das táticas ([this], 0x28 bytes por tática) usada pelo clube: a palavra +0 do
/// registro de 0x91 bytes do clube (id em +0) na tabela tática (+0x601).
pub(crate) unsafe fn tactic_stats_row(this: usize, club: usize) -> usize {
    let id = read_usize(club);
    let k = read_i16(read_usize(this + tactics_fields::ROWS).wrapping_add(id.wrapping_mul(0x91))) as i32;
    read_usize(this).wrapping_add((k as isize * 0x28) as usize)
}

/// Returns x as f64, or 1.0 when x < 1.
///
/// [pt] x < 1 vale 1 (palavra com sinal), como as três comparações antes de cada `fild`.
pub(crate) fn at_least_one(x: i16) -> f64 {
    if x < 1 { 1.0 } else { x as f64 }
}

/// At the end of a match with no human, adds to each club's tactic row (0x28 bytes) the games, wins, draws, losses,
/// goals and points (3 per win, 1 per draw) and redoes the three double ratios (+0x10/+0x18/+0x20) that the AI
/// formation choice reads.
///
/// Ret 4: `Tactics::record_tactic_result(match)`, from the end of each match in the engine loop (0x6ba9a0, with the
/// aggregate score; fixture +0 ≠ −2). Null match: the box at line 0x1f9 and 0. Home side (+0x1c) or away side (+0x20)
/// managed by a human (0x600a50 with 0, 1, 0, 0; cdecl 5): 0. Otherwise, for each side (null club: the box at line
/// 0x20a and continues), the match record for the club (0x7e4750, into a local record that starts with the default date
/// 0x54a110) is added to the club's tactic row (tactic_stats_row): +2 matches, +4 wins (+3 points in +0xe), +6 draws
/// (+1 point), +8 defeats, +0xa goals for (+0x19 of the record, signed), +0xc against (+0x1a); with p = matches, a =
/// for, b = against and c = points (each at least 1), q = c/p, g = a/p, h = g·q², r = p/b: +0x10 = r·h·g, +0x18 =
/// r·g·q²·q, +0x20 = r·r·h (the order of the x87 operations). The computer's formation choice (0x6e0230) reads +2 and
/// the three ratios. Returns 1.
/// Original: `Tactics::record_tactic_result` (0x896290, tactics.cpp, thiscall, ret 4).
///
/// [pt] { 0x896290 } (thiscall, ret 4, tactics.cpp): `Tactics::record_tactic_result(jogo)`, do fim de cada partida no laço
/// do motor (0x6ba9a0, com o placar agregado; fixture +0 ≠ −2). Jogo nulo: a caixa da linha 0x1f9 e 0. Mandante (+0x1c)
/// ou visitante (+0x20) dirigido por humano (0x600a50 com 0, 1, 0, 0; cdecl 5): 0. Senão, para cada lado (clube nulo:
/// a caixa da linha 0x20a e segue), o registro da partida pelo clube (0x7e4750, num registro local que começa com a
/// data padrão 0x54a110) soma na linha da tática do clube (tactic_stats_row): +2 jogos, +4 vitórias (+3 pontos em +0xe), +6
/// empates (+1 ponto), +8 derrotas, +0xa gols pró (+0x19 do registro, com sinal), +0xc contra (+0x1a); com p = jogos,
/// a = pró, b = contra e c = pontos (cada um pelo menos 1), q = c/p, g = a/p, h = g·q², r = p/b: +0x10 = r·h·g, +0x18
/// = r·g·q²·q, +0x20 = r·r·h (a ordem das contas do x87). A escolha da formação do computador (0x6e0230) lê +2 e as
/// três razões. Devolve 1.
pub(crate) unsafe extern "thiscall" fn record_tactic_result(this: usize, game: usize) -> u32 {
    if game == 0 {
        show_error_at(0xa8b908, 0x1f9);
        return 0;
    }
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    for k in 0..2 {
        let c = read_usize(game + fixture::HOME_CLUB + 4 * k);
        if c != 0 && human(c, 0, 1, 0, 0) != 0 {
            return 0;
        }
    }
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let record: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_FILL_MATCH_RECORD_FROM_FIXTURE);
    let mut reg = [0u8; 0x20];
    let pr = reg.as_mut_ptr() as usize;
    data_default(pr);
    for k in 0..2 {
        let c = read_usize(game + fixture::HOME_CLUB + 4 * k);
        if c == 0 {
            show_error_at(0xa8b908, 0x20a);
            continue;
        }
        let l = tactic_stats_row(this, c);
        record(game, c, pr);
        let m = |o: usize| (l + o) as *mut i16;
        crate::comparison::branch("cpz_5 linha da tatica");
        *m(2) = (*m(2)).wrapping_add(1);
        let (pro, contra) = (reg[0x19] as i8, reg[0x1a] as i8);
        if pro > contra {
            *m(4) = (*m(4)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(3); // cpz5: a vitória vale 3 pontos
        } else if pro == contra {
            *m(6) = (*m(6)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(1);
        } else {
            *m(8) = (*m(8)).wrapping_add(1);
        }
        *m(0xa) = (*m(0xa)).wrapping_add(pro as i16); // cpz5: os gols pró
        *m(0xc) = (*m(0xc)).wrapping_add(contra as i16);
        let a = at_least_one(*m(0xa));
        let b = at_least_one(*m(0xc));
        let c = at_least_one(*m(0xe));
        let p = at_least_one(*m(2));
        let q = c / p;
        let q2 = q * q;
        let g = a / p;
        let h = g * q2;
        let r = p / b;
        let f = |o: usize| (l + o) as *mut f64;
        core::ptr::write_unaligned(f(0x10), r * h * g);
        core::ptr::write_unaligned(f(0x18), r * g * q2 * q); // cpz5: the ratio of the normal stance
        core::ptr::write_unaligned(f(0x20), r * r * h);
    }
    1
}

/// Returns the captain stored (+0x52) in the club and team row of the tactics table; no row, 0.
///
/// Ret 8: `Tactics::get_captain(club, team)`, from the tactic screens (0x8a7440). Null club: the box at line 0x527 and
/// 0. The club's row (0x8982b0, thiscall(this, club, team), ret 8; −1: 0); returns dword +0x52 of the row's 0x91-byte
/// record in the tactics table (+0x601).
/// Original: `Tactics::get_captain` (0x8976a0, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x8976a0 } (thiscall, ret 8, tactics.cpp): `Tactics::get_captain(clube, time)`, das telas de tática (0x8a7440). Clube
/// nulo: a caixa da linha 0x527 e 0. A linha do clube (0x8982b0, thiscall(this, clube, time), ret 8; −1: 0); devolve o
/// dword +0x52 do registro de 0x91 bytes da linha na tabela tática (+0x601).
pub(crate) unsafe extern "thiscall" fn get_captain(this: usize, club: usize, team: u32) -> u32 {
    if club == 0 {
        show_error_at(0xa8b908, 0x527);
        return 0;
    }
    let row: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(this, club, team);
    if k == -1 {
        return 0;
    }
    read_usize(read_usize(this + tactics_fields::ROWS).wrapping_add((k as isize * 0x91) as usize) + 0x52) as u32
}

/// Writes the captain (+0x52) in the club and team row of the tactics table and returns 1; no row, 0.
///
/// Ret 0xc: `Tactics::set_captain(club, &person, team)`, when applying the changes of the tactic screen (0x8a80d0).
/// Null club: the box at line 0x579; null pointer: that of 0x581; 0. The club's row (0x8982b0; −1: 0); writes the
/// pointed dword into +0x52 of the row's record (+0x601) and returns 1.
/// Original: `Tactics::set_captain` (0x897880, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x897880 } (thiscall, ret 0xc, tactics.cpp): `Tactics::set_captain(clube, &pessoa, time)`, ao aplicar as mudanças da
/// tela de tática (0x8a80d0). Clube nulo: a caixa da linha 0x579; ponteiro nulo: a da 0x581; 0. A linha do clube
/// (0x8982b0; −1: 0); grava o dword apontado em +0x52 do registro da linha (+0x601) e devolve 1.
pub(crate) unsafe extern "thiscall" fn set_captain(this: usize, club: usize, person: usize, team: u32) -> u32 {
    if club == 0 {
        show_error_at(0xa8b908, 0x579);
        return 0;
    }
    if person == 0 {
        show_error_at(0xa8b908, 0x581);
        return 0;
    }
    let row: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(this, club, team);
    if k == -1 {
        return 0;
    }
    let p = read_usize(this + tactics_fields::ROWS).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    core::ptr::write_unaligned(p as *mut u32, core::ptr::read_unaligned(person as *const u32));
    1
}

/// Club's row in the tactics table (FN_TACTICS_GET_CLUB_TACTICS_INDEX, stdcall(club, human)); −1 without one.
///
/// [pt] A linha do clube na tabela tática (0x8982b0, stdcall(clube, humano); −1 sem linha).
pub(crate) unsafe fn club_tactics_index(club: usize, human: u32) -> i32 {
    let f: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    f(club, human)
}

/// Start of tactics row l (0x91 bytes each, from [this+0x601]).
///
/// [pt] O começo da linha `l` (0x91 bytes cada, a partir de [this+0x601]).
pub(crate) unsafe fn tactics_row_start(this: usize, l: i32) -> usize {
    read_usize(this + tactics_fields::ROWS).wrapping_add(l.wrapping_mul(0x91) as usize)
}

/// Clears the undo copies (16 × 0x5c bytes at this+0x30) of the club and team and decrements the count at +0x5f0.
///
/// [pt] As 16 cópias para desfazer (0x5c bytes a partir de this+0x30: +0 clube, +4 time, +8 a cópia; conta em +0x5f0): a do
/// clube e do time some (+8 = 0) e a conta desce.
pub(crate) unsafe fn drop_undo_copies(this: usize, club: usize, team: u32) {
    if read_u8(this + tactics_fields::UNDO_COUNT) == 0 {
        return;
    }
    for k in 0..0x10usize {
        let e = this + tactics_fields::UNDO_SLOTS + k * 0x5c;
        if read_usize(e + undo_slot::IN_USE) != 0 && read_usize(e) == club && read_usize(e + undo_slot::TEAM) as u32 == team {
            write_u32(e + undo_slot::IN_USE, 0);
            *((this + tactics_fields::UNDO_COUNT) as *mut u8) = read_u8(this + tactics_fields::UNDO_COUNT).wrapping_sub(1);
        }
    }
}

/// Memory regions of the club's lineup routines: its tactics row (if any) and the undo copies with their count.
///
/// [pt] As regiões da escalação do clube: a linha dele na tabela tática (se houver) e as cópias para desfazer com a conta.
pub(crate) unsafe fn lineup_regions(this: usize, club: usize, team: u32) -> Vec<(usize, usize)> {
    let mut reg = vec![(this + tactics_fields::UNDO_SLOTS, 0x5c1)];
    let l = club_tactics_index(club, team);
    if l != -1 {
        reg.push((tactics_row_start(this, l), 0x91));
    }
    reg
}

/// Removes a player from the selection: empties (−1) the selection slots holding the person; with roles, also the two
/// extra slots and the set-piece takers; the undo copies of the club and team are dropped; 1 when removed from a slot
/// or set piece.
///
/// Ret 0x10: `Tactics::remove_player_from_selection(club, person, team, roles)`, from the contracts (change of place in
/// the squad, loan and its end), from leaving the squad, from the automatic line-up (0x6dbc50), from the national
/// sides, from retirement and from the transfers. No tactic row (0x8982b0): 0. The 20 line-up slots (+2 of the row,
/// ids) with the person (id at +0) become −1. With `roles`: the two extra slots (+0x56, +0x5a: 0x897dc0 and 0x897ee0)
/// and the takers of the five set pieces on both sides (0x897990 and 0x897bb0) that are that person are emptied. The
/// undo copies of the club and team are dropped. Returns 1 if it removed the person from a slot or set piece. Null club
/// or person: the original routine ("Error" box, 0).
/// Original: `Tactics::remove_player_from_selection` (0x896c60, tactics.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x896c60 } (thiscall, ret 0x10, tactics.cpp): `Tactics::remove_player_from_selection(clube, pessoa, time,
/// papeis)`, pelos contratos (troca de lugar no elenco, empréstimo e o fim dele), pelas saídas do elenco, pela escalação
/// automática (0x6dbc50), pelas seleções, pela aposentadoria e pelas transferências. Sem linha tática (0x8982b0): 0. As
/// 20 vagas da escalação (+2 da linha, ids) com a pessoa (id em +0) viram −1. Com `papeis`: as duas vagas extras (+0x56,
/// +0x5a: 0x897dc0 e 0x897ee0) e os batedores das cinco bolas paradas nos dois lados (0x897990 e 0x897bb0) que são ela
/// ficam vazios. As cópias para desfazer do clube e do time somem. Devolve 1 se a tirou de uma vaga ou de bola parada.
/// Clube ou pessoa nulos: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn remove_player_from_selection(this: usize, club: usize, person: usize, team: u32, papeis: u32) -> u32 {
    if club == 0 || person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_REMOVE_PLAYER_FROM_SELECTION)(this, club, person, team, papeis); // message box: original only
    }
    let l = club_tactics_index(club, team);
    if l == -1 {
        return 0;
    }
    let mut found = 0;
    for k in 0..20usize { // cpx9: the line-up slots
        let slot = tactics_row_start(this, l) + 2 + 4 * k;
        if read_i32(slot) == read_i32(person) {
            crate::comparison::branch(if k < 11 { "cpx_9 tirou titular" } else { "cpx_9 tirou reserva" });
            write_u32(slot, u32::MAX); // cpx9: a vaga fica vazia
            found = 1;
        }
    }
    if papeis != 0 {
        let of_row: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_PLAYMAKER);
        let extra: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_PLAYMAKER);
        // simplification: the original pushes the type and the side with leftovers in the high bytes (the row offset and the stack);
        // the calls read only the low byte, and the copy passes clean values
        let batedor: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_SET_PIECE_TAKER);
        for qual in 0..2u32 {
            if of_row(this, club, qual, team) == person {
                extra(this, club, qual, 0, team);
            }
        }
        for tipo in 0..5u32 {
            for lado in 0..2u32 {
                if batedor(this, club, tipo, lado, team) == person {
                    crate::comparison::branch("cpx_9 batedor tirado");
                    exe::tactics_set_set_piece_taker(this, club, tipo, lado, 0, team);
                    found = 1;
                }
            }
        }
    }
    drop_undo_copies(this, club, team);
    found
}

/// Swaps the ids of two selection slots (0..19) and drops the undo copies of the club and team.
///
/// Ret 0x10: `Tactics::swap_selection_slots(club, a, b, team)`, from the squad screen (0x46835e). Slots `a` and `b`
/// (signed bytes, 0..19): swaps the ids of the two line-up slots (+2 + 4 × slot of the tactic row, 0x90ba90 with 4
/// bytes) and the undo copies of the club and team are dropped; returns 1. No row (0x8982b0): 0. Null club or slot out
/// of range: the original routine ("Error" box, 0).
/// Original: `Tactics::swap_selection_slots` (0x8973e0, tactics.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8973e0 } (thiscall, ret 0x10, tactics.cpp): `Tactics::swap_selection_slots(clube, a, b, time)`, pela tela do
/// elenco (0x46835e). Vagas `a` e `b` (bytes com sinal, 0..19): troca os ids das duas vagas da escalação (+2 + 4 × vaga
/// da linha tática, 0x90ba90 com 4 bytes) e as cópias para desfazer do clube e do time somem; devolve 1. Sem linha
/// (0x8982b0): 0. Clube nulo ou vaga fora: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn swap_selection_slots(this: usize, club: usize, a: u32, b: u32, team: u32) -> u32 {
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if club == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_SWAP_SELECTION_SLOTS)(this, club, a, b, team); // message box: original only
    }
    let l = club_tactics_index(club, team);
    if l == -1 {
        return 0;
    }
    let swap: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_UTILS_SWAP_BYTES);
    let start = tactics_row_start(this, l);
    swap(start + tactics_row::SELECTION + 4 * va as usize, start + tactics_row::SELECTION + 4 * vb as usize, 4);
    drop_undo_copies(this, club, team);
    1
}

/// Returns 1 when the current human holds the selection undo copy of this club and team.
///
/// Ret 8: `Tactics::has_selection_undo(club, team)`, from the squad screen (0x458a58). With copies (+0x5f0) and the
/// human whose turn it is (`[0xb74c26 + 0x300 × (i16)[0xb77c7a]]`): his copy (index `[0xae2364]` − id − 1, from 0 to
/// 15) exists (+8) and is of this club (+0) and team (+4): 1; otherwise 0.
/// Original: `Tactics::has_selection_undo` (0x897250, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x897250 } (thiscall, ret 8, tactics.cpp): `Tactics::has_selection_undo(clube, time)`, pela tela do elenco
/// (0x458a58). Com cópias (+0x5f0) e o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]): a cópia dele (índice
/// [0xae2364] − id − 1, de 0 a 15) existe (+8) e é deste clube (+0) e time (+4): 1; senão 0.
pub(crate) unsafe extern "thiscall" fn has_selection_undo(this: usize, club: usize, team: u32) -> u32 {
    if read_u8(this + tactics_fields::UNDO_COUNT) == 0 {
        return 0;
    }
    let t = read_usize((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize);
    if t == 0 {
        return 0;
    }
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(t)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        return 0;
    }
    let e = this + tactics_fields::UNDO_SLOTS + k as usize * 0x5c;
    (read_usize(e + undo_slot::IN_USE) != 0 && read_usize(e) == club && read_usize(e + undo_slot::TEAM) as u32 == team) as u32
}

/// Club's row in the tactics table (FN_TACTICS_GET_CLUB_TACTICS_INDEX); −1 without one (duplicate of club_tactics_index).
///
/// [pt] A linha do clube na tabela tática (0x8982b0, o `this` não é usado; −1 sem linha).
pub(crate) unsafe fn club_tactics_row(club: usize, team: u32) -> i32 {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    row(club, team)
}

/// Address of dword cell c of tactics row k: [this + 0x601] + 0x91 × k + c.
///
/// [pt] A célula `c` (dword) da linha `k` da tabela tática: [this + 0x601] + 0x91 × k + `c`.
pub(crate) unsafe fn tactics_cell(this: usize, k: i32, c: usize) -> usize {
    (read_u32(this + tactics_fields::ROWS) as usize).wrapping_add((k as isize * 0x91) as usize).wrapping_add(c)
}

/// The club's selection slot in the tactics table becomes empty (−1) and the stored slots of the same club and team are
/// released.
///
/// Ret 0xc: `Tactics::clear_selection_slot(club, slot, team)`: slot (0..=19) of the club's line-up in the tactics table
/// (row from 0x8982b0, dword at +2 + 4 × slot) becomes −1 and, with stored spaces (+0x5f0), those of the same club and
/// team (16 spaces of 0x5c bytes starting at +0x30: club +0x30, team +0x34, used +0x38) are released (+0x38 = 0, +0x5f0
/// − 1). Returns 1; no row, 0. Null club or slot out of range: the original routine (box).
/// Original: `Tactics::clear_selection_slot` (0x896e90, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x896e90 } (thiscall, ret 0xc, tactics.cpp): `Tactics::clear_selection_slot(clube, vaga, time)`: a vaga (0..=19)
/// da escalação do clube na tabela tática (linha da 0x8982b0, dword em +2 + 4 × vaga) fica −1 e, com espaços guardados
/// (+0x5f0), os do mesmo clube e time (16 espaços de 0x5c bytes a partir de +0x30: clube +0x30, time +0x34, ocupado
/// +0x38) são soltos (+0x38 = 0, +0x5f0 − 1). Devolve 1; sem linha, 0. Clube nulo ou vaga fora: a original (caixa).
pub(crate) unsafe extern "thiscall" fn clear_selection_slot(this: usize, club: usize, slot: u32, team: u32) -> u32 {
    let v = slot as u8 as i8;
    if club == 0 || !(0..=0x13).contains(&v) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_CLEAR_SELECTION_SLOT)(this, club, slot, team);
    }
    let k = club_tactics_row(club, team);
    if k == -1 {
        return 0;
    }
    crate::comparison::branch("cpq_13 limpar vaga");
    core::ptr::write_unaligned(tactics_cell(this, k, 2 + 4 * v as usize) as *mut i32, -1); // a vaga
    if *((this + tactics_fields::UNDO_COUNT) as *const u8) != 0 {
        for e in 0..16 {
            let a = this + tactics_fields::UNDO_SLOTS + 0x5c * e;
            if read_u32(a + undo_slot::IN_USE) != 0 && read_u32(a) as usize == club && read_u32(a + undo_slot::TEAM) == team {
                core::ptr::write_unaligned((a + undo_slot::IN_USE) as *mut u32, 0);
                *((this + tactics_fields::UNDO_COUNT) as *mut u8) = (*((this + tactics_fields::UNDO_COUNT) as *const u8)).wrapping_sub(1);
            }
        }
    }
    1
}

/// Stores the taker of a set piece (first or second choice); without a person, clears it and the second choice moves up
/// to the first.
///
/// Ret 0x14: `Tactics::set_set_piece_taker(club, role, choice, person, team)`: taker `choice` (0..=1) of set piece
/// `role` (0..=4) in the club's tactic row (0x8982b0; dword at +0x5e + 4 × (choice + 2 × role)). With `person` (pointer
/// to the id), writes the id. Without it, the cell becomes −1 and, if it is the first choice, the next one with a taker
/// (> −1) moves up to it and becomes −1. Returns 1; no row, 0. Null club, role or choice out of range: the original
/// routine (box).
/// Original: `Tactics::set_set_piece_taker` (0x897bb0, tactics.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x897bb0 } (thiscall, ret 0x14, tactics.cpp): `Tactics::set_set_piece_taker(clube, papel, escolha, pessoa, time)`:
/// o batedor `escolha` (0..=1) da bola parada `papel` (0..=4) na linha tática do clube (0x8982b0; dword em +0x5e +
/// 4 × (escolha + 2 × papel)). Com `pessoa` (ponteiro para o id), grava o id. Sem ela, a célula fica −1 e, sendo a
/// primeira escolha, a seguinte com batedor (> −1) sobe para ela e fica −1. Devolve 1; sem linha, 0. Clube nulo,
/// papel ou escolha fora: a original (caixa).
pub(crate) unsafe extern "thiscall" fn set_set_piece_taker(this: usize, club: usize, role: u32, choice: u32, person: usize, team: u32) -> u32 {
    let (p, e) = (role as u8 as i8, choice as u8 as i8);
    if club == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_SET_SET_PIECE_TAKER)(this, club, role, choice, person, team);
    }
    let k = club_tactics_row(club, team);
    if k == -1 {
        return 0;
    }
    let cell = |j: i8| tactics_cell(this, k, 0x5e + 4 * (j as i32 + 2 * p as i32) as usize) as *mut i32;
    if person != 0 {
        core::ptr::write_unaligned(cell(e), read_u32(person) as i32);
        return 1;
    }
    crate::comparison::branch("cpq_13 batedor limpo");
    core::ptr::write_unaligned(cell(e), -1); // sem batedor
    if e < 1 {
        let mut j = e + 1;
        while j < 2 {
            let c = core::ptr::read_unaligned(cell(j));
            if c > -1 {
                crate::comparison::branch("cpq_13 batedor sobe");
                core::ptr::write_unaligned(cell(e), c);
                core::ptr::write_unaligned(cell(j), -1);
                return 1;
            }
            j += 1;
        }
    }
    1
}

/// the formation record (0x18e3 bytes) in the two tables
/// Original: `formation_record` (0x896570, tactics.cpp, thiscall).
///
/// [pt] { 0x896570 } (thiscall, ret 8, tactics.cpp): o registro (0x18e3 bytes) da formação `f`: as
/// [+0x5fd] primeiras em [+0x5f9], as seguintes (até [+0x5f5] mais, só sem `humano`) em [+0x5f1].
/// Formação negativa ou fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn formation_record(t: usize, f: i16, human: u32) -> usize {
    let primeiras = core::ptr::read_unaligned((t + tactics_fields::PRESET_TACTIC_COUNT) as *const i32);
    let limit = if human != 0 { primeiras } else { core::ptr::read_unaligned((t + tactics_fields::CUSTOM_TACTIC_COUNT) as *const i32).wrapping_add(primeiras) };
    let f = f as i32;
    if f < 0 || f >= limit {
        return original_fn::<unsafe extern "thiscall" fn(usize, i16, u32) -> usize>(IDX_FORMATION_RECORD)(t, f as i16, human);
    }
    if f < primeiras {
        core::ptr::read_unaligned((t + tactics_fields::PRESET_TACTICS) as *const usize).wrapping_add(f as usize * 0x18e3)
    } else {
        core::ptr::read_unaligned((t + tactics_fields::CUSTOM_TACTICS) as *const usize).wrapping_add((f - primeiras) as usize * 0x18e3)
    }
}

compare_value!(compare_formation_record, formation_record, IDX_FORMATION_RECORD, (f: i16, humano: u32) -> usize);

/// Sets the two extra slots (+0x56, +0x5a) of the club's row.
///
/// Ret 0x10: the two extra slots (+0x56 and +0x5a) of the row of `club` in the tactics table (0x8982b0 with `human`; no
/// row, 0): with `person`, `*person` goes into slot `which` 0/1; without it, the slot becomes −1 and, for slot 0, slot
/// 1 moves down to 0 and becomes −1. Returns 1. Null `club` or `which` out of range: the original routine (error box).
/// Original: `extra_tactic_slot` (0x897ee0, tactics.cpp, thiscall).
///
/// [pt] { 0x897ee0 } (thiscall, ret 0x10, tactics.cpp): as duas vagas extras (+0x56 e +0x5a) da linha do
/// `clube` na tabela tática (0x8982b0 com `humano`; sem linha, 0): com `pessoa`, `*pessoa` na vaga
/// `qual` 0/1; sem, a vaga vira −1 e, na 0, a 1 desce para a 0 e fica −1. Devolve 1. `clube` nulo ou
/// `qual` fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn extra_tactic_slot(t: usize, club: usize, which: i8, person: *const u32, human: u32) -> u32 {
    if club == 0 || !(0..=1).contains(&which) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_EXTRA_TACTIC_SLOT)(t, club, which, person, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    let row = table_row(t, l);
    let home = (row + tactics_row::PLAYMAKER + 4 * which as usize) as *mut u32;
    if !person.is_null() {
        crate::comparison::branch("vaga extra gravada");
        core::ptr::write_unaligned(home, *person);
        return 1;
    }
    crate::comparison::branch("vaga extra apagada");
    core::ptr::write_unaligned(home, u32::MAX);
    if which == 0 {
        let second = core::ptr::read_unaligned((row + tactics_row::SECOND_PLAYMAKER) as *const u32);
        core::ptr::write_unaligned((row + tactics_row::PLAYMAKER) as *mut u32, second);
        core::ptr::write_unaligned((row + tactics_row::SECOND_PLAYMAKER) as *mut u32, u32::MAX);
    }
    1
}

/// Returns the club's tactic: for a human club (without +0x9a01 or with holiday option 2), the slot for the team and
/// the manager's active block; otherwise, by the index in the club row, the preset block (below the preset count) or
/// the club's own.
///
/// Ret 8: `Tactics::get_club_tactic(club, team)`, from the formation of each match (0x6e0230), from the player's
/// weaknesses (0x5470d0), from the media and from the screens. A human club (0x600a50 with 1, 1) whose manager (+0xcf)
/// does not have +0x9a01 (0x5fd210) or has holiday option 2 (0x5fea50): the slot for the team (0x5fbe20) and his active
/// block (0x5fbd00). For the others, the index (signed word) in the club's row (id × 0x91 in `[this+0x601]`): below the
/// count of preset tactics (`[this+0x5fd]`) the preset block (`[this+0x5f9]`), otherwise its own (`[this+0x5f1]`, index
/// − count); blocks of 0x18e3 bytes. Null club: the original routine ("Error" box, 0).
/// Original: `Tactics::get_club_tactic` (0x896660, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x896660 } (thiscall, ret 8, tactics.cpp): `Tactics::get_club_tactic(clube, time)`, pela formação de cada jogo
/// (0x6e0230), pelas fraquezas do jogador (0x5470d0), pela mídia e pelas telas. Clube humano (0x600a50 com 1, 1) cujo
/// técnico (+0xcf) não tem +0x9a01 (0x5fd210) ou tem a opção 2 das férias (0x5fea50): a vaga pelo time (0x5fbe20) e o
/// bloco ativo dele (0x5fbd00). Nos outros, o índice (palavra com sinal) na linha do clube (id × 0x91 em [this+0x601]):
/// abaixo da conta das táticas prontas ([this+0x5fd]) o bloco pronto ([this+0x5f9]), senão o próprio ([this+0x5f1],
/// índice − conta); blocos de 0x18e3 bytes. Clube nulo: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn get_club_tactic(this: usize, club: usize, team: u32) -> usize {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CLUB_TACTIC)(this, club, team); // message box: original only
    }
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human(club, 1, 1, 0, 0) != 0 {
        let manager = read_usize(G_HUMAN_MANAGER);
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
        let option: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
        if t9a01(manager, read_usize(club + club::MANAGER)) == 0 || option(manager, 2, read_usize(club + club::MANAGER)) != 0 {
            let choose: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
            crate::comparison::branch("cpx_8 tatica do humano");
            choose(manager, team, club, read_usize(club + club::MANAGER)); // cpx8: the slot by team
            return exe::human_manager_get_active_tactic(manager, read_usize(club + club::MANAGER));
        }
    }
    let id = read_i32(club);
    let index = read_i16(read_usize(this + tactics_fields::ROWS).wrapping_add(id.wrapping_mul(0x91) as usize)) as i32;
    let prontas = read_i32(this + tactics_fields::PRESET_TACTIC_COUNT);
    if index < prontas { // cpx8: tática pronta ou própria
        crate::comparison::branch("cpx_8 tatica pronta");
        read_usize(this + tactics_fields::PRESET_TACTICS).wrapping_add(index.wrapping_mul(0x18e3) as usize) // cpx8: o bloco pronto
    } else {
        crate::comparison::branch("cpx_8 tatica propria");
        read_usize(this + tactics_fields::CUSTOM_TACTICS).wrapping_add(index.wrapping_sub(prontas).wrapping_mul(0x18e3) as usize) // cpx8: o bloco próprio
    }
}

/// Returns the club's selection slot (0..19) holding the player, or 0xff.
///
/// Ret 0xc: the slot (0..19) of the club's line-up where the person is: the club's tactic row (0x8982b0 with `team`; -1
/// returns 0) and the 20 slots (dwords at +0x601 + row × 0x91 + 2) compared with the id; not there: 0xff. Null club or
/// person: the original routine (error box).
/// Original: `Tactics::find_selection_slot` (0x8972c0, tactics.cpp, thiscall).
///
/// [pt] { 0x8972c0 } (thiscall, ret 0xc, tactics.cpp): a vaga (0..19) da escalação do clube em que está a
/// pessoa: linha tática do clube (0x8982b0 com `time`; -1 devolve 0) e as 20 vagas (dwords em +0x601 +
/// linha x 0x91 + 2) comparadas com o id; não está: 0xff. Clube ou pessoa nulos: a original (caixa
/// de erro).
pub(crate) unsafe extern "thiscall" fn find_selection_slot(tactic: usize, club: usize, person: *const i32, team: u32) -> u8 {
    if club == 0 || person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(IDX_FIND_SELECTION_SLOT)(tactic, club, person, team);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, team);
    if k == -1 {
        return 0;
    }
    let id = *person;
    let slots = core::ptr::read_unaligned((tactic + tactics_fields::ROWS) as *const usize).wrapping_add((k as usize).wrapping_mul(0x91) + 2);
    match (0..20).find(|&i| core::ptr::read_unaligned((slots + 4 * i) as *const i32) == id) {
        Some(i) => i as u8,
        None => 0xff,
    }
}

/// Stores the person in slot 0..19 of the club's row (0x8982b0) and clears the club's pending instructions.
///
/// Ret 0x10: writes `*person` into slot `slot` 0..19 of the row of `club` (0x8982b0 with `human`; no row, 0) and
/// deletes, among the 16 pending instructions (+0x30, 0x5c bytes: club, human, active), the active ones of that club
/// and `human`, decrementing counter +0x5f0. Returns 1. Null `club` or `person`, slot out of range: the original
/// routine (error box).
/// Original: `set_tactic_slot` (0x896a90, tactics.cpp, thiscall).
///
/// [pt] { 0x896a90 } (thiscall, ret 0x10, tactics.cpp): grava `*pessoa` na vaga `vaga` 0..19 da linha do
/// `clube` (0x8982b0 com `humano`; sem linha, 0) e apaga, entre as 16 instruções pendentes
/// (+0x30, 0x5c bytes: clube, humano, ativa), as ativas desse clube e `humano`, descontando o
/// contador +0x5f0. Devolve 1. `clube` ou `pessoa` nulos, vaga fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn set_tactic_slot(t: usize, club: usize, slot: i8, person: *const u32, human: u32) -> u32 {
    if club == 0 || !(0..=19).contains(&slot) || person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_SET_TACTIC_SLOT)(t, club, slot, person, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned((table_row(t, l) + 2 + 4 * slot as usize) as *mut u32, *person);
    let pendentes = (t + tactics_fields::UNDO_COUNT) as *mut u8;
    if *pendentes != 0 {
        for k in 0..16 {
            let e = t + tactics_fields::UNDO_SLOTS + 0x5c * k;
            let active = (e + undo_slot::IN_USE) as *mut u32;
            if *active != 0 && *(e as *const usize) == club && *((e + undo_slot::TEAM) as *const u32) == human {
                *active = 0;
                *pendentes = (*pendentes).wrapping_sub(1);
            }
        }
    }
    1
}

/// Returns the slot's person; without a record, or from another nation in a national side, the slot becomes −1.
///
/// Ret 0xc: the person (in `[0xae23bc]`, 0x6e bytes) of slot `slot` 0..19 of the row of `club`; 0 with no row or an
/// empty (negative) slot. A slot with a person without a player sheet (+0x61), or, in a national side (0x53b900), a
/// person without a nation (+0x24) or from another nation (+0x53): the slot becomes −1 and returns 0. Null `club` or
/// slot out of range: the original routine (error box).
/// Original: `tactic_slot_person` (0x8968f0, tactics.cpp, thiscall).
///
/// [pt] { 0x8968f0 } (thiscall, ret 0xc, tactics.cpp): a pessoa (em [0xae23bc], 0x6e bytes) da vaga `vaga`
/// 0..19 da linha do `clube`; 0 sem linha ou vaga vazia (negativa). Vaga com pessoa sem ficha
/// (+0x61), ou, numa seleção (0x53b900), pessoa sem nação (+0x24) ou de outra nação (+0x53): a vaga
/// vira −1 e devolve 0. `clube` nulo ou vaga fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn tactic_slot_person(t: usize, club: usize, slot: i8, human: u32) -> usize {
    if club == 0 || !(0..=19).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_TACTIC_SLOT_PERSON)(t, club, slot, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    let home = (table_row(t, l) + 2 + 4 * slot as usize) as *mut i32;
    let id = core::ptr::read_unaligned(home);
    if id <= -1 {
        return 0;
    }
    let persons = *(G_STAFF_TABLE as *const usize);
    let p = persons.wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if core::ptr::read_unaligned((p + staff::PLAYER) as *const usize) == 0 {
        core::ptr::write_unaligned(home, -1);
        return 0;
    }
    if national_team(club) != 0 {
        let nation = core::ptr::read_unaligned((p + staff::NATIONAL_JOB) as *const usize);
        if nation == 0 || core::ptr::read_unaligned((nation + nation::THREE_LETTER_NAME) as *const u32) != core::ptr::read_unaligned((club + club::NATION) as *const u32) {
            core::ptr::write_unaligned(home, -1);
            return 0;
        }
    }
    p
}

/// stores the formation in the club row
/// Original: `set_tactic_formation` (0x8967d0, tactics.cpp, thiscall).
///
/// [pt] { 0x8967d0 } (thiscall, ret 0xc, tactics.cpp): grava a formação `formacao` (palavra) na linha do
/// `clube`; 0 sem linha, senão 1. `clube` nulo ou formação fora de 0..[+0x5fd]+[+0x5f5]−1: a
/// original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn set_tactic_formation(t: usize, club: usize, formation: i16, human: u32) -> u32 {
    let limit = core::ptr::read_unaligned((t + tactics_fields::PRESET_TACTIC_COUNT) as *const i32)
        .wrapping_add(core::ptr::read_unaligned((t + tactics_fields::CUSTOM_TACTIC_COUNT) as *const i32))
        .wrapping_sub(1);
    if club == 0 || formation < 0 || formation as i32 > limit {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(IDX_SET_TACTIC_FORMATION)(t, club, formation, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned(table_row(t, l) as *mut i16, formation);
    1
}

