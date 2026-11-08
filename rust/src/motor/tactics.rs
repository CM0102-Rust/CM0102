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
pub(crate) unsafe extern "thiscall" fn stored_taker(this: usize, clube: usize, kind: u32, side: u32, human: u32) -> usize {
    let (t, l) = (kind as u8 as i8, side as u8 as i8);
    if clube == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(IDX_STORED_TAKER)(this, clube, kind, side, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, human);
    if k == -1 {
        crate::comparacao::ramo("batedor sem linha");
        return 0;
    }
    let cell = (*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 0x5e + (l as i32 + t as i32 * 2) as usize * 4) as *mut i32;
    let v = *cell;
    if v > (*(G_STAFF_COUNT as *const i32)).wrapping_add(0x52d) {
        crate::comparacao::ramo("batedor alem das pessoas");
        *cell = -1;
        return 0;
    }
    if v <= -1 {
        crate::comparacao::ramo("batedor vazio");
        return 0;
    }
    let person = (*(G_STAFF_TABLE as *const usize)).wrapping_add(v as usize * staff::SIZE);
    if *((person + staff::PLAYER) as *const u32) == 0 {
        crate::comparacao::ramo("batedor sem ficha");
        *cell = -1;
        return 0;
    }
    if *((person + staff::CLUB_JOB) as *const usize) == clube {
        crate::comparacao::ramo("batedor do clube");
        return person;
    }
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_IS_NATIONAL_OR_ALL_STAR_TEAM);
    if national_team(clube) != 0 || contract_bond(*(G_NATIONAL_TEAMS as *const usize), clube) != 0 {
        crate::comparacao::ramo("batedor de fora do clube");
        return person;
    }
    crate::comparacao::ramo("batedor saiu do clube");
    *cell = -1;
    0
}

/// saves or restores the club's 20 instructions (dwords of the tactics row + 2) in one of 16 slots of 0x5c bytes
/// (+0x30; used count at +0x5f0), the current human's ([0xb74c26 + [0xb77c7a] × 0x300]; [0xae2364] − id − 1); when
/// saving, the instructions become −1. Compared side by side (the slots and the row)
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
pub(crate) unsafe extern "thiscall" fn store_instructions(this: usize, clube: usize, person: u32, usar: u32, volta: u32) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_STORE_INSTRUCTIONS)(this, clube, person, usar, volta);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, person);
    if k == -1 {
        return 0;
    }
    let instrucao = |i: usize| (*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 2 + 4 * i) as *mut u32;
    let ocupados = (this + 0x5f0) as *mut u8;
    // o espaco do humano da vez, se houver
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
                crate::comparacao::ramo("instructions guardadas");
                if *u(s, 0x38) == 0 {
                    *ocupados = (*ocupados).wrapping_add(1);
                }
                *u(s, 0x30) = clube as u32;
                *u(s, 0x34) = person;
                *u(s, 0x38) = 1;
                for i in 0..20 {
                    *u(s, 0x3c + 4 * i) = *instrucao(i);
                }
            }
        }
        crate::comparacao::ramo("instructions apagadas");
        for i in 0..20 {
            *instrucao(i) = u32::MAX;
        }
        return 1;
    }
    if usar != 0 {
        if let Some(s) = espaco() {
            if *u(s, 0x38) != 0 && *u(s, 0x30) == clube as u32 && *u(s, 0x34) == person {
                crate::comparacao::ramo("instructions devolvidas");
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
            if *u(s, 0x38) != 0 && *u(s, 0x30) == clube as u32 && *u(s, 0x34) == person {
                crate::comparacao::ramo("instructions liberadas");
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
pub(crate) unsafe extern "thiscall" fn read_tactics_row(this: usize, clube: usize, target: *mut u8, human: u32) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(IDX_READ_TACTICS_ROW)(this, clube, target, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, human);
    if k == -1 {
        return 0;
    }
    crate::comparacao::ramo("linha tatica lida");
    core::ptr::copy((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91) as *const u8, target, 0x91);
    1
}

/// the reverse, writes the 0x91 bytes into the row (row -1 or beyond the total +0x605: 0). Called by 0x6dbb10
/// (`store_tactic`). Compared side by side
/// Original: `write_tactics_row` (0x8984d0, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8984d0 } (ret 0xc, tactics.cpp): o inverso da 0x8983f0, grava `origem` (0x91 bytes) na linha
/// tatica do clube. Linha -1 ou alem do total (+0x605): 0.
pub(crate) unsafe extern "thiscall" fn write_tactics_row(this: usize, clube: usize, source: *const u8, human: u32) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(IDX_WRITE_TACTICS_ROW)(this, clube, source, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, human);
    if k == -1 || k >= *((this + 0x605) as *const i32) {
        return 0;
    }
    crate::comparacao::ramo("linha tatica gravada");
    core::ptr::copy(source, (*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91) as *mut u8, 0x91);
    1
}

/// Memory regions of the tactics copy routines: the source/destination (0x91 bytes) and the club's tactics row, if any,
/// plus the RNG global.
///
/// [pt] Regioes das duas: o destino/origem e a linha do clube (se houver).
pub(crate) unsafe fn tactics_row_regions(this: usize, clube: usize, p: usize, human: u32) -> Vec<(usize, usize)> {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, human);
    let mut reg = vec![(0xb67a34, 4), (p, 0x91)];
    if k != -1 && k < *((this + 0x605) as *const i32) {
        reg.push(((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91), 0x91));
    }
    reg
}



/// 0x6beeb0 with the out-of-engine calls injected (commentary, injuries, player exits): simulates a match without
/// highlights, period by period, including extra time, penalties and ratings.
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
        let q = (p + 0x35) as *mut i16;
        *q = (*q).wrapping_add(d);
    };
    let pt = || u(m, 0x4792);
    let pb = |o: usize| (u(m, 0x4792) + o) as *mut i8;
    let goals = |side: usize| *b(0xf5bd + side * 0x36) as i32;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, c: u8, side: usize| {
        call(FN_MATCH_EVENTS_ADD_EVENT, u(m, 0x8f7a), [code, m as u32, c as u32, side as u32, u32::MAX]);
    };
    // goleiros (tabela 1) ativos do lado sofrem o gol
    let goleiros = |side: usize| {
        for i in 0..20 {
            let q = jog(side, i);
            if byte(q, 0x19) >= 0 && player_table(q) == 1 {
                more((q + 0xf) as *mut u8);
            }
        }
    };
    let mut end = false;
    let mut decide = false;
    let mut local_28 = 0u8;
    let mut side = 0usize;
    *w(0x8ed0) = 0;
    *w(0x8ed4) = 0x1ef;
    let penalties = 'periodo: loop {
        team_strength(m, 0);
        team_strength(m, 1);
        for o in [0x1da, 0x1de, 0x1e2, 0x1e6] {
            let (a, x) = (*w(o) as f64, *w(o + 2) as f64);
            let middle = x * read_f64_at(0x968968); // 0x968968 = 0.5
            if !(a >= middle) {
                *w(o) = match_pl_ftol(middle) as i16;
            } else if a > x + x {
                *w(o) = match_pl_ftol(x + x) as i16;
            }
        }
        let c7 = sortear(2) as u8;
        let mut round = 0u8;
        'rodadas: while *b(0x1a8) == 0 {
            side = if c7 != 0 { round as usize } else { (round == 0) as usize };
            let other = (side == 0) as usize;
            let strong = || sortear(word(0x1de, side) + word(0x1da, side)) > sortear(word(0x1da, other) + word(0x1de, other));
            // os tres ataques (o terceiro pela forca)
            for k in 0..3 {
                if k >= 2 && !strong() {
                    continue;
                }
                mais16(w(0xf5e2 + side * 0x36));
                let mut t = 0u8;
                let i = loop {
                    let i = sortear(0x14) as i8;
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
                let f = (read_f32_at(p + 0xc5) as f64 + read_f32_at(0x96afb0) as f64) as f32; // 0x96afb0 = 20
                let r = sortear((word(0x1da, other) + 200) / 19);
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
                let f = (read_f32_at(p + 0xdd) as f64 + read_f32_at(0x96b020) as f64) as f32; // 0x96b020 = 15
                let r = sortear(word(0x1de, other) / 5);
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
                    let i = sortear(0x14) as i8;
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
                let f = (read_f32_at(p + 0xad) as f64 + read_f32_at(0x96b020) as f64) as f32; // 0x96b020 = 15
                let r = sortear((word(0x1f2, other) + 100) / 7);
                if !(r as f64 >= f as f64) {
                    mais16(w(0xf5ec + side * 0x36));
                    add(p, 0x14);
                } else {
                    add(p, -8);
                }
            }
            if sortear(10) == 0 {
                more(b(0xf5c8 + side * 0x36));
            }
            let p = jog(side, choose_by_role(m, side as u32) as i8);
            let v = match_pl_ftol(read_f32_at(p + 0xb9) as f64 + read_f32_at(0x96afb0) as f64); // 0x96afb0 = 20
            if sortear(v) < sortear(2) {
                add(p, -0xf);
                more(b(0xf5ca + side * 0x36));
            }
            // os cartoes do outro lado, ou o tipo do lance
            local_28 = 0;
            let mut c20 = false;
            if sortear(word(0x1ea, other)) > sortear(0x640) {
                let mo = m + other * 0x36;
                more((mo + 0xf5c9) as *mut u8);
                let expulsar = |c: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
                    let p = jog(other, c as i8);
                    more((p + 0x23) as *mut u8);
                    more((mo + 0xf5cb) as *mut u8);
                    player_left_pitch_with(m, p, 1, call);
                    team_strength(m, other as i8);
                    narrate(call, 0x1ff8, c, other);
                };
                if sortear(word(0x1ea, other)) > sortear(0x61a8) {
                    crate::comparacao::ramo("simulacao rapida expulsao");
                    let c = choose_by_condition(m, other as i8, 4);
                    add(jog(other, c as i8), -0xfa);
                    expulsar(c, call);
                } else if sortear(word(0x1ea, other)) > sortear(0x6a4) {
                    crate::comparacao::ramo("simulacao rapida give_yellow_card");
                    let c = choose_by_condition(m, other as i8, 2);
                    let p = jog(other, c as i8);
                    add(p, -0x32);
                    more((p + 0x24) as *mut u8);
                    more((mo + 0xf5cc) as *mut u8);
                    if byte(p, 0x24) == 2 {
                        crate::comparacao::ramo("simulacao rapida segundo give_yellow_card");
                        expulsar(c, call);
                    }
                    if *((mo + 0xf5cb) as *const u8) > 4 {
                        // abandono: 3 x 0 para o lado da vez
                        crate::comparacao::ramo("simulacao rapida abandono");
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
                            more(b(0xf5bd));
                        }
                        break 'rodadas;
                    }
                }
                if sortear(0x1e) != 0 {
                    c20 = true;
                    more((mo + 0xf5c6) as *mut u8);
                } else {
                    local_28 = 1;
                }
            } else if sortear(word(0x1de, side)) > sortear(3000) {
                c20 = true;
                more(b(0xf5c7 + side * 0x36));
            }
            // o fator do placar
            let d = goals(side) - goals(other);
            let goal_difference = if side == 0 { *pb(0x4b) as i32 - *pb(0x4c) as i32 } else { *pb(0x4c) as i32 - *pb(0x4b) as i32 };
            let x = if goal_difference + d > 3 {
                d as f64
            } else if goals(side) + goals(other) != 0 {
                read_f32_at(0x968e70) as f64 // 0x968e70 = 1
            } else if *w(0x8ed0) > 0x3c {
                read_f32_at(0x96d320) as f64 // 0x96d320 = 0.6499999762
            } else {
                read_f32_at(0x96d248) as f64 // 0x96d248 = 0.75
            };
            let (v, repartir_outro): (f64, unsafe extern "thiscall" fn(usize, i8, i16)) = if c20 {
                (word(0x1f2, other) as f64 * x * read_f64_at(0x96d310), split_c) // 0x96d310 = 6.1
            } else {
                (word(0x1da, other) as f64 * x * read_f64_at(0x96d318), split_b) // 0x96d318 = 4.8
            };
            let r1 = sortear(match_pl_ftol(v));
            let l24 = sortear(word(0x1de, side)).wrapping_sub(r1) as i16;
            if l24 > 0 {
                split_a(m, side as i8, 0x1c);
                repartir_outro(m, other as i8, -0x14);
            } else {
                split_a(m, side as i8, -6);
                repartir_outro(m, other as i8, 0x28);
            }
            // o chute
            if l24 > 0 && sortear(goals(1) + goals(0)) <= 3 {
                let ml = m + side * 0x36;
                if local_28 != 0 {
                    more((ml + 0xf5c0) as *mut u8);
                }
                more((ml + 0xf5be) as *mut u8);
                let c8 = choose_by_role_e(m, side as u32, c20 as u32) as i8;
                let a = if c20 { sortear(word(0x1ee, side)) } else { sortear(word(0x1e2, side) * (local_28 as i32 + 1)) };
                let us = a as i16 as i32;
                if us + 0xa5 > sortear(0x172) + (0x168 - l24 as i32) {
                    more((ml + 0xf5bf) as *mut u8);
                    if c20 {
                        split_d(m, side as i8, a as i16);
                    } else {
                        split_e(m, side as i8, a as i16);
                    }
                    if us + 0x2d > sortear((7 - local_28 as i32 * 3) * word(0x1e6, other)) {
                        crate::comparacao::ramo("simulacao rapida gol");
                        let c9 = choose_by_role_c(m, side as u32, local_28 as u32);
                        let p = jog(side, c9 as i8);
                        more((p + 0xc) as *mut u8);
                        add(p, 0x1db);
                        goleiros(other);
                        if local_28 != 0 {
                            crate::comparacao::ramo("simulacao rapida gol de penalty");
                            narrate(call, 0x1fcf, c9, side);
                            more((ml + 0xf5c1) as *mut u8);
                            more((p + 0xd) as *mut u8);
                            more((p + 0xe) as *mut u8);
                        } else {
                            narrate(call, 0x1fcc, c9, side);
                            if c20 {
                                let o = match sortear(4) {
                                    0 | 1 => 0xf5c3,
                                    2 => 0xf5c5,
                                    _ => 0xf5c4,
                                };
                                more((ml + o) as *mut u8);
                            }
                        }
                        if c8 > -1 {
                            let q = jog(side, c8);
                            more((q + 0x10) as *mut u8);
                            add(q, 0x15e);
                        }
                        more(b(0xf5c2 + other * 0x36));
                        more((ml + 0xf5bd) as *mut u8);
                        for (s, delta) in [(side, 0x4b), (other, -0x43)] {
                            for i in 0..20 {
                                let q = jog(s, i);
                                if byte(q, 0x19) >= 0 {
                                    add(q, delta);
                                }
                            }
                        }
                        if decide {
                            crate::comparacao::ramo("simulacao rapida gol de ouro");
                            *b(0x1a8) = 1;
                            break 'rodadas;
                        }
                    } else {
                        split_f(m, other as i8, 0xe1);
                        if local_28 != 0 {
                            crate::comparacao::ramo("simulacao rapida penalty perdido");
                            let c = choose_by_role_c(m, side as u32, local_28 as u32);
                            narrate(call, 0x1fce, c, side);
                            let p = jog(side, c as i8);
                            more((p + 0xd) as *mut u8);
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
        // o fim do periodo
        mais16(w(0x8ed0));
        let t = *w(0x8ed0);
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
        if *b(0x1a8) != 0 {
            *pb(0x47) = goals(0) as i8;
            *pb(0x48) = goals(1) as i8;
            break 'periodo false;
        }
        match t {
            0x2d => {
                *w(0x8ed4) = 0x3de;
                *pb(0x45) = goals(0) as i8;
                *pb(0x46) = goals(1) as i8;
            }
            0x5a => {
                *pb(0x43) = goals(0) as i8;
                *pb(0x44) = goals(1) as i8;
                *w(0x8ed4) = 0x483;
                let agregado = *pb(0x4b) != -1;
                if (agregado && !igual_no_agregado(0x43, 0x44)) || (!agregado && *pb(0x43) != *pb(0x44)) {
                    break 'periodo false;
                }
                if w38 & 0x80 != 0 {
                    crate::comparacao::ramo("simulacao rapida prorrogacao com gol de ouro");
                    decide = true;
                    continue 'periodo;
                }
                if w38 & 2 != 0 {
                    crate::comparacao::ramo("simulacao rapida prorrogacao");
                    continue 'periodo;
                }
                if agregado && w38 & 4 == 0 && *pb(0x44) != *pb(0x4b) && w38 & 0x4000 == 0 {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            0x69 => *w(0x8ed4) = 0x528,
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
        crate::comparacao::ramo("simulacao rapida penalties");
        sortear(5);
        sortear(5);
        *pb(0x49) = sortear(5) as i8;
        sortear(5);
        sortear(5);
        *pb(0x4a) = sortear(5) as i8;
        if *pb(0x49) == *pb(0x4a) {
            let o = if sortear(2) != 0 { 0x49 } else { 0x4a };
            *pb(o) = (*pb(o)).wrapping_add(1);
        }
    }
    // o agregado
    if *pb(0x4b) != -1 {
        let (a, x) = if *pb(0x47) != -1 { (0x47, 0x48) } else { (0x43, 0x44) };
        *pb(0x4b) = (*pb(0x4b)).wrapping_add(*pb(a));
        *pb(0x4c) = (*pb(0x4c)).wrapping_add(*pb(x));
    }
    // o gol do mando
    let c1 = u(pt(), 0x1c);
    if c1 != 0 && u(pt(), 0x14) == u(c1, 0x57) && (*pb(0x43) as i32 + *pb(0x44) as i32) < 5 && *pb(0x43) == *pb(0x44) && sortear(4) == 0 {
        let (r0, r1) = (*((c1 + 0x80) as *const i16) as i32, *((u(pt(), 0x20) + 0x80) as *const i16) as i32);
        let s = if r0 > r1 { Some(0) } else if r0 + 0xfa < r1 { Some(1) } else { None };
        if let Some(s) = s {
            crate::comparacao::ramo("simulacao rapida gol do mando");
            more(b(0xf5be + s * 0x36));
            let c = choose_by_role_c(m, s as u32, local_28 as u32);
            let p = jog(s, c as i8);
            more((p + 0xc) as *mut u8);
            add(p, 0x1db);
            goleiros(1 - s);
            narrate(call, 0x1fcc, c, s);
        }
    }
    // as lesoes (duas passadas pelo ultimo lado da vez)
    if *(pt() as *const i32) != -2 {
        for _ in 0..2 {
            for i in 0..20 {
                let p = jog(side, i);
                let person = u(p, 0x6d);
                if byte(p, 0x19) < 0 || person == 0 {
                    continue;
                }
                let f = read_f32_at(p + 0x4d);
                let mut cl = (byte(person, 0x2d) as u8).wrapping_mul(2).wrapping_add(5);
                if !(f >= read_f32_at(0x96d30c)) { // 0x96d30c = 6750
                    cl = cl.wrapping_add(0xf);
                } else if !(f >= read_f32_at(0x96d308)) { // 0x96d308 = 8000
                    cl = cl.wrapping_add(5);
                }
                if sortear(cl as i8 as i32) < sortear(400) {
                    continue;
                }
                let comp = u(pt(), 0x14);
                let league = u(G_FRIENDLY, 0);
                let ceiling = if comp == 0 || (league != 0 && is_friendly(league, comp) != 0) { 0x5b } else { 0x66 };
                let g = sortear(ceiling) as u8;
                let kind = match sortear(10) {
                    0 => 1,
                    1..=3 => 3,
                    4..=6 => 4,
                    7..=9 => 5,
                    _ => continue,
                };
                crate::comparacao::ramo("simulacao rapida lesao");
                call(FN_CM3_MATCH_ENGINE_INJURE_PLAYER, m, [kind, g as u32, p as u32, 0, 0]);
            }
        }
    }
    // as notas
    for s in 0..2 {
        for i in 0..20i8 {
            let p = jog(s, i);
            let q = (p + 0x35) as *mut i16;
            if *(pt() as *const i32) == -2 && i < 11 {
                crate::comparacao::ramo("simulacao rapida nota do amistoso");
                let clube = if s != 0 { u(pt(), 0x20) } else { u(pt(), 0x1c) };
                let a = (*((p + 0x3b) as *const i16) as f64 - read_f64_at(0x968e30)) * read_f64_at(0x968ff8); // 0x968e30 = 40, 0x968ff8 = 50
                let r = *((clube + club::REPUTATION) as *const i16) as f64 * read_f64_at(0x969018) - read_f64_at(0x96d300); // 0x969018 = 1.6, 0x96d300 = 6000
                let v = match_pl_ftol(((a - r) * read_f64_at(0x96d2f8) + read_f64_at(0x967890)) * *q as f64) as i16; // 0x96d2f8 = 2e-05, 0x967890 = 1
                *q = v.clamp(0xdac, 0x2ee0);
            }
            let n = (match_pl_ftol((*q as i32 + 0x1770) as f64 * read_f64_at(0x96a608) + read_f64_at(0x96d2f0)) as i8).clamp(1, 10); // 0x96a608 = 0.0004, 0x96d2f0 = 1.83
            *((p + 0x1b) as *mut i8) = n;
            *((p + 0x51) as *mut u32) = *((p + 0x4d) as *const u32);
        }
    }
    for (a, x) in [(0xf5e2, 0xf5e4), (0xf618, 0xf61a)] {
        if *w(a) < 10 {
            crate::comparacao::ramo("simulacao rapida contador reiniciado");
            *w(a) = 0x14;
            *w(x) = (sortear(6) + 12) as i16;
        } else if *w(a) == *w(x) {
            crate::comparacao::ramo("simulacao rapida contador igual");
            let r = sortear(3);
            *w(x) = (*w(x)).wrapping_add((-1i32).wrapping_sub(r) as i16);
        }
    }
}



/// Club's tactics row index (0x8982b0) and its start in the table at +0x601 (0x91 bytes per row), or None for row −1.
///
/// [pt] Linha tatica do clube (0x8982b0) e o inicio dela na tabela em +0x601 (0x91 bytes por linha), ou
/// `None` na linha -1.
pub(crate) unsafe fn tactics_row_of_club(this: usize, clube: usize, human: u32) -> Option<usize> {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(clube, human);
    (l != -1).then(|| (*((this + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)))
}

/// Returns the squad slot (0..19) of the club's tactics row whose person id matches the id stored at +0x52 of the row
/// (the designated player, captain per the function map [inference]); none: -1 with `menos_um`, else 0; no row: 0. A
/// null club calls the original (error box).
///
/// [pt] { 0x897590 } (tactics.cpp): a vaga (0..20) cuja pessoa (indice em +2 + 4 x vaga, no vetor de
/// 0xae23bc) tem o id guardado em +0x52 da linha; sem nenhuma: -1 com `menos_um`, senao 0. Linha
/// -1: 0. Clube nulo: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn marked_slot(this: usize, clube: usize, human: u32, minus_one: u32) -> i8 {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(IDX_MARKED_SLOT)(this, clube, human, minus_one);
    }
    let Some(linha) = tactics_row_of_club(this, clube, human) else { return 0 };
    let persons = *(G_STAFF_TABLE as *const usize);
    for vaga in 0..20usize {
        let i = *((linha + 2 + 4 * vaga) as *const i32);
        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == *((linha + 0x52) as *const i32) {
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
pub(crate) unsafe extern "thiscall" fn mark_slot(this: usize, clube: usize, slot: i8, human: u32) -> u32 {
    if clube == 0 || !(0..20).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(IDX_MARK_SLOT)(this, clube, slot, human);
    }
    let Some(linha) = tactics_row_of_club(this, clube, human) else { return 0 };
    let i = *((linha + 2 + 4 * slot as usize) as *const i32);
    if i == -1 {
        return 0;
    }
    *((linha + 0x52) as *mut i32) = i;
    1
}

/// Returns the person (staff table 0xae23bc, 110 bytes each) referenced by the club's tactics row at +0x56 + 4 x `qual`
/// (0 or 1); 0 when the index is negative or the club has no row. Null club or `qual` out of range go to the original
/// (error box).
///
/// [pt] { 0x897dc0 } (tactics.cpp): a pessoa (vetor de 0xae23bc, 110 bytes cada) do indice em +0x56 +
/// 4 x `qual` (0 ou 1) da linha; indice negativo ou linha -1: 0. Clube nulo ou `qual` fora: a
/// original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn row_person(this: usize, clube: usize, which: i8, human: u32) -> usize {
    if clube == 0 || !(0..2).contains(&which) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_ROW_PERSON)(this, clube, which, human);
    }
    let Some(linha) = tactics_row_of_club(this, clube, human) else { return 0 };
    let i = *((linha + 0x56 + 4 * which as usize) as *const i32);
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
pub(crate) unsafe extern "stdcall" fn tactics_row(clube: usize, human: u32) -> i32 {
    let and_human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if and_human(clube, 1, 1, 0, 0) != 0 && human != 0 {
        let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
        if second(clube) != 0 {
            return -1;
        }
        let manager = *((clube + club::MANAGER) as *const usize);
        if manager == 0 {
            return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(clube, human);
        }
        let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        let clubs = *(G_CLUB_COUNT as *const i32);
        let base = (*(G_STAFF_COUNT as *const i32)).wrapping_sub(*(manager as *const i32));
        let i = base.wrapping_add(if is_national_team(clube) != 0 { 0xf } else { -1 }).wrapping_add(clubs);
        if i < 0 || i >= clubs.wrapping_add(0x20) {
            return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(clube, human);
        }
        return i;
    }
    *(clube as *const i32)
}

/// Returns byte `pos` (0..10) of the club's tactics row (index from 0x8982b0, for the human manager when `humano`) in
/// the table at +0x601 (0x91 bytes per row, from +0x86); row -1: 0. Null club or position out of range: falls back to
/// the original (error box).
///
/// [pt] { 0x898040 } (tactics.cpp): byte `pos` (0..=10) da linha tatica do clube (0x8982b0) na tabela
/// em +0x601 (0x91 bytes por linha, a partir de +0x86); linha -1: 0. Clube nulo ou posicao fora:
/// a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn tactics_byte(this: usize, clube: usize, pos: i8, human: u32) -> u8 {
    if clube == 0 || !(0..=10).contains(&pos) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8>(IDX_TACTICS_BYTE)(this, clube, pos, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(clube, human);
    if l == -1 {
        return 0;
    }
    let table = *((this + 0x601) as *const usize);
    *((table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x86 + pos as usize) as *const u8)
}



/// Row of the tactic statistics table ([this], 0x28 bytes per tactic) used by the club: word +0 of the club's 0x91-byte
/// tactics row.
///
/// [pt] A linha da tabela de estatísticas das táticas ([this], 0x28 bytes por tática) usada pelo clube: a palavra +0 do
/// registro de 0x91 bytes do clube (id em +0) na tabela tática (+0x601).
pub(crate) unsafe fn tactic_stats_row(this: usize, clube: usize) -> usize {
    let id = read_usize(clube);
    let k = read_i16(read_usize(this + 0x601).wrapping_add(id.wrapping_mul(0x91))) as i32;
    read_usize(this).wrapping_add((k as isize * 0x28) as usize)
}

/// Returns x as f64, or 1.0 when x < 1.
///
/// [pt] x < 1 vale 1 (palavra com sinal), como as três comparações antes de cada `fild`.
pub(crate) fn at_least_one(x: i16) -> f64 {
    if x < 1 { 1.0 } else { x as f64 }
}

/// at the end of a match with no human, adds to each club's tactic row (0x28 bytes) the games, wins, draws, losses,
/// goals and points (3 per win, 1 per draw) and redoes the three double ratios (+0x10/+0x18/+0x20) that the AI
/// formation choice reads
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
pub(crate) unsafe extern "thiscall" fn record_tactic_result(this: usize, jogo: usize) -> u32 {
    if jogo == 0 {
        show_error_at(0xa8b908, 0x1f9);
        return 0;
    }
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    for k in 0..2 {
        let c = read_usize(jogo + 0x1c + 4 * k);
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
        let c = read_usize(jogo + 0x1c + 4 * k);
        if c == 0 {
            show_error_at(0xa8b908, 0x20a);
            continue;
        }
        let l = tactic_stats_row(this, c);
        record(jogo, c, pr);
        let m = |o: usize| (l + o) as *mut i16;
        crate::comparacao::ramo("cpz_5 linha da tatica");
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
        core::ptr::write_unaligned(f(0x18), r * g * q2 * q); // cpz5: a razão da postura normal
        core::ptr::write_unaligned(f(0x20), r * r * h);
    }
    1
}


/// the captain stored (+0x52) in the club and team row of the tactics table; no row, 0
/// Original: `Tactics::get_captain` (0x8976a0, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x8976a0 } (thiscall, ret 8, tactics.cpp): `Tactics::get_captain(clube, time)`, das telas de tática (0x8a7440). Clube
/// nulo: a caixa da linha 0x527 e 0. A linha do clube (0x8982b0, thiscall(this, clube, time), ret 8; −1: 0); devolve o
/// dword +0x52 do registro de 0x91 bytes da linha na tabela tática (+0x601).
pub(crate) unsafe extern "thiscall" fn get_captain(this: usize, clube: usize, team: u32) -> u32 {
    if clube == 0 {
        show_error_at(0xa8b908, 0x527);
        return 0;
    }
    let row: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(this, clube, team);
    if k == -1 {
        return 0;
    }
    read_usize(read_usize(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52) as u32
}


/// writes the captain (+0x52) in the club and team row of the tactics table and returns 1; no row, 0
/// Original: `Tactics::set_captain` (0x897880, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x897880 } (thiscall, ret 0xc, tactics.cpp): `Tactics::set_captain(clube, &pessoa, time)`, ao aplicar as mudanças da
/// tela de tática (0x8a80d0). Clube nulo: a caixa da linha 0x579; ponteiro nulo: a da 0x581; 0. A linha do clube
/// (0x8982b0; −1: 0); grava o dword apontado em +0x52 do registro da linha (+0x601) e devolve 1.
pub(crate) unsafe extern "thiscall" fn set_captain(this: usize, clube: usize, person: usize, team: u32) -> u32 {
    if clube == 0 {
        show_error_at(0xa8b908, 0x579);
        return 0;
    }
    if person == 0 {
        show_error_at(0xa8b908, 0x581);
        return 0;
    }
    let row: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(this, clube, team);
    if k == -1 {
        return 0;
    }
    let p = read_usize(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    core::ptr::write_unaligned(p as *mut u32, core::ptr::read_unaligned(person as *const u32));
    1
}







/// Club's row in the tactics table (FN_TACTICS_GET_CLUB_TACTICS_INDEX, stdcall(club, human)); −1 without one.
///
/// [pt] A linha do clube na tabela tática (0x8982b0, stdcall(clube, humano); −1 sem linha).
pub(crate) unsafe fn club_tactics_index(clube: usize, human: u32) -> i32 {
    let f: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    f(clube, human)
}

/// Start of tactics row l (0x91 bytes each, from [this+0x601]).
///
/// [pt] O começo da linha `l` (0x91 bytes cada, a partir de [this+0x601]).
pub(crate) unsafe fn tactics_row_start(this: usize, l: i32) -> usize {
    read_usize(this + 0x601).wrapping_add(l.wrapping_mul(0x91) as usize)
}

/// Clears the undo copies (16 × 0x5c bytes at this+0x30) of the club and team and decrements the count at +0x5f0.
///
/// [pt] As 16 cópias para desfazer (0x5c bytes a partir de this+0x30: +0 clube, +4 time, +8 a cópia; conta em +0x5f0): a do
/// clube e do time some (+8 = 0) e a conta desce.
pub(crate) unsafe fn drop_undo_copies(this: usize, clube: usize, team: u32) {
    if read_u8(this + 0x5f0) == 0 {
        return;
    }
    for k in 0..0x10usize {
        let e = this + 0x30 + k * 0x5c;
        if read_usize(e + 8) != 0 && read_usize(e) == clube && read_usize(e + 4) as u32 == team {
            write_u32(e + 8, 0);
            *((this + 0x5f0) as *mut u8) = read_u8(this + 0x5f0).wrapping_sub(1);
        }
    }
}

/// Memory regions of the club's lineup routines: its tactics row (if any) and the undo copies with their count.
///
/// [pt] As regiões da escalação do clube: a linha dele na tabela tática (se houver) e as cópias para desfazer com a conta.
pub(crate) unsafe fn lineup_regions(this: usize, clube: usize, team: u32) -> Vec<(usize, usize)> {
    let mut reg = vec![(this + 0x30, 0x5c1)];
    let l = club_tactics_index(clube, team);
    if l != -1 {
        reg.push((tactics_row_start(this, l), 0x91));
    }
    reg
}

/// empties (−1) the selection slots holding the person; with roles, also the two extra slots and the set piece takers;
/// the undo copies of the club and team are dropped; 1 when removed from a slot or set piece
/// Original: `Tactics::remove_player_from_selection` (0x896c60, tactics.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x896c60 } (thiscall, ret 0x10, tactics.cpp): `Tactics::remove_player_from_selection(clube, pessoa, time,
/// papeis)`, pelos contratos (troca de lugar no elenco, empréstimo e o fim dele), pelas saídas do elenco, pela escalação
/// automática (0x6dbc50), pelas seleções, pela aposentadoria e pelas transferências. Sem linha tática (0x8982b0): 0. As
/// 20 vagas da escalação (+2 da linha, ids) com a pessoa (id em +0) viram −1. Com `papeis`: as duas vagas extras (+0x56,
/// +0x5a: 0x897dc0 e 0x897ee0) e os batedores das cinco bolas paradas nos dois lados (0x897990 e 0x897bb0) que são ela
/// ficam vazios. As cópias para desfazer do clube e do time somem. Devolve 1 se a tirou de uma vaga ou de bola parada.
/// Clube ou pessoa nulos: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn remove_player_from_selection(this: usize, clube: usize, person: usize, team: u32, papeis: u32) -> u32 {
    if clube == 0 || person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_REMOVE_PLAYER_FROM_SELECTION)(this, clube, person, team, papeis); // caixa: só a original
    }
    let l = club_tactics_index(clube, team);
    if l == -1 {
        return 0;
    }
    let mut found = 0;
    for k in 0..20usize { // cpx9: as vagas da escalação
        let slot = tactics_row_start(this, l) + 2 + 4 * k;
        if read_i32(slot) == read_i32(person) {
            crate::comparacao::ramo(if k < 11 { "cpx_9 tirou titular" } else { "cpx_9 tirou reserva" });
            write_u32(slot, u32::MAX); // cpx9: a vaga fica vazia
            found = 1;
        }
    }
    if papeis != 0 {
        let of_row: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_PLAYMAKER);
        let extra: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_PLAYMAKER);
        // simplification: a original empurra o tipo e o lado com sobra nos bytes altos (o deslocamento da linha e a pilha);
        // as chamadas leem só o byte baixo, e a cópia passa os valores limpos
        let batedor: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_SET_PIECE_TAKER);
        let mark: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_SET_PIECE_TAKER);
        for qual in 0..2u32 {
            if of_row(this, clube, qual, team) == person {
                extra(this, clube, qual, 0, team);
            }
        }
        for tipo in 0..5u32 {
            for lado in 0..2u32 {
                if batedor(this, clube, tipo, lado, team) == person {
                    crate::comparacao::ramo("cpx_9 batedor tirado");
                    mark(this, clube, tipo, lado, 0, team);
                    found = 1;
                }
            }
        }
    }
    drop_undo_copies(this, clube, team);
    found
}


/// swaps the ids of two selection slots (0..19) and drops the undo copies of the club and team
/// Original: `Tactics::swap_selection_slots` (0x8973e0, tactics.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8973e0 } (thiscall, ret 0x10, tactics.cpp): `Tactics::swap_selection_slots(clube, a, b, time)`, pela tela do
/// elenco (0x46835e). Vagas `a` e `b` (bytes com sinal, 0..19): troca os ids das duas vagas da escalação (+2 + 4 × vaga
/// da linha tática, 0x90ba90 com 4 bytes) e as cópias para desfazer do clube e do time somem; devolve 1. Sem linha
/// (0x8982b0): 0. Clube nulo ou vaga fora: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn swap_selection_slots(this: usize, clube: usize, a: u32, b: u32, team: u32) -> u32 {
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_SWAP_SELECTION_SLOTS)(this, clube, a, b, team); // caixa: só a original
    }
    let l = club_tactics_index(clube, team);
    if l == -1 {
        return 0;
    }
    let swap: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_UTILS_SWAP_BYTES);
    let start = tactics_row_start(this, l);
    swap(start + 2 + 4 * va as usize, start + 2 + 4 * vb as usize, 4);
    drop_undo_copies(this, clube, team);
    1
}


/// 1 when the current human holds the selection undo copy of this club and team
/// Original: `Tactics::has_selection_undo` (0x897250, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x897250 } (thiscall, ret 8, tactics.cpp): `Tactics::has_selection_undo(clube, time)`, pela tela do elenco
/// (0x458a58). Com cópias (+0x5f0) e o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]): a cópia dele (índice
/// [0xae2364] − id − 1, de 0 a 15) existe (+8) e é deste clube (+0) e time (+4): 1; senão 0.
pub(crate) unsafe extern "thiscall" fn has_selection_undo(this: usize, clube: usize, team: u32) -> u32 {
    if read_u8(this + 0x5f0) == 0 {
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
    let e = this + 0x30 + k as usize * 0x5c;
    (read_usize(e + 8) != 0 && read_usize(e) == clube && read_usize(e + 4) as u32 == team) as u32
}



/// Club's row in the tactics table (FN_TACTICS_GET_CLUB_TACTICS_INDEX); −1 without one (duplicate of club_tactics_index).
///
/// [pt] A linha do clube na tabela tática (0x8982b0, o `this` não é usado; −1 sem linha).
pub(crate) unsafe fn club_tactics_row(clube: usize, team: u32) -> i32 {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    row(clube, team)
}

/// Address of dword cell c of tactics row k: [this + 0x601] + 0x91 × k + c.
///
/// [pt] A célula `c` (dword) da linha `k` da tabela tática: [this + 0x601] + 0x91 × k + `c`.
pub(crate) unsafe fn tactics_cell(this: usize, k: i32, c: usize) -> usize {
    (read_u32(this + 0x601) as usize).wrapping_add((k as isize * 0x91) as usize).wrapping_add(c)
}

/// the club selection slot in the tactics table becomes empty (−1) and the stored slots of the same club and team are
/// released
/// Original: `Tactics::clear_selection_slot` (0x896e90, tactics.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x896e90 } (thiscall, ret 0xc, tactics.cpp): `Tactics::clear_selection_slot(clube, vaga, time)`: a vaga (0..=19)
/// da escalação do clube na tabela tática (linha da 0x8982b0, dword em +2 + 4 × vaga) fica −1 e, com espaços guardados
/// (+0x5f0), os do mesmo clube e time (16 espaços de 0x5c bytes a partir de +0x30: clube +0x30, time +0x34, ocupado
/// +0x38) são soltos (+0x38 = 0, +0x5f0 − 1). Devolve 1; sem linha, 0. Clube nulo ou vaga fora: a original (caixa).
pub(crate) unsafe extern "thiscall" fn clear_selection_slot(this: usize, clube: usize, slot: u32, team: u32) -> u32 {
    let v = slot as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_CLEAR_SELECTION_SLOT)(this, clube, slot, team);
    }
    let k = club_tactics_row(clube, team);
    if k == -1 {
        return 0;
    }
    crate::comparacao::ramo("cpq_13 limpar vaga");
    core::ptr::write_unaligned(tactics_cell(this, k, 2 + 4 * v as usize) as *mut i32, -1); // a vaga
    if *((this + 0x5f0) as *const u8) != 0 {
        for e in 0..16 {
            let a = this + 0x30 + 0x5c * e;
            if read_u32(a + 8) != 0 && read_u32(a) as usize == clube && read_u32(a + 4) == team {
                core::ptr::write_unaligned((a + 8) as *mut u32, 0);
                *((this + 0x5f0) as *mut u8) = (*((this + 0x5f0) as *const u8)).wrapping_sub(1);
            }
        }
    }
    1
}


/// stores the taker of a set piece (first or second choice); without a person, clears it and the second choice moves up
/// to the first
/// Original: `Tactics::set_set_piece_taker` (0x897bb0, tactics.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x897bb0 } (thiscall, ret 0x14, tactics.cpp): `Tactics::set_set_piece_taker(clube, papel, escolha, pessoa, time)`:
/// o batedor `escolha` (0..=1) da bola parada `papel` (0..=4) na linha tática do clube (0x8982b0; dword em +0x5e +
/// 4 × (escolha + 2 × papel)). Com `pessoa` (ponteiro para o id), grava o id. Sem ela, a célula fica −1 e, sendo a
/// primeira escolha, a seguinte com batedor (> −1) sobe para ela e fica −1. Devolve 1; sem linha, 0. Clube nulo,
/// papel ou escolha fora: a original (caixa).
pub(crate) unsafe extern "thiscall" fn set_set_piece_taker(this: usize, clube: usize, role: u32, choice: u32, person: usize, team: u32) -> u32 {
    let (p, e) = (role as u8 as i8, choice as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_SET_SET_PIECE_TAKER)(this, clube, role, choice, person, team);
    }
    let k = club_tactics_row(clube, team);
    if k == -1 {
        return 0;
    }
    let cell = |j: i8| tactics_cell(this, k, 0x5e + 4 * (j as i32 + 2 * p as i32) as usize) as *mut i32;
    if person != 0 {
        core::ptr::write_unaligned(cell(e), read_u32(person) as i32);
        return 1;
    }
    crate::comparacao::ramo("cpq_13 batedor limpo");
    core::ptr::write_unaligned(cell(e), -1); // sem batedor
    if e < 1 {
        let mut j = e + 1;
        while j < 2 {
            let c = core::ptr::read_unaligned(cell(j));
            if c > -1 {
                crate::comparacao::ramo("cpq_13 batedor sobe");
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
    let primeiras = core::ptr::read_unaligned((t + 0x5fd) as *const i32);
    let limit = if human != 0 { primeiras } else { core::ptr::read_unaligned((t + 0x5f5) as *const i32).wrapping_add(primeiras) };
    let f = f as i32;
    if f < 0 || f >= limit {
        return original_fn::<unsafe extern "thiscall" fn(usize, i16, u32) -> usize>(IDX_FORMATION_RECORD)(t, f as i16, human);
    }
    if f < primeiras {
        core::ptr::read_unaligned((t + 0x5f9) as *const usize).wrapping_add(f as usize * 0x18e3)
    } else {
        core::ptr::read_unaligned((t + 0x5f1) as *const usize).wrapping_add((f - primeiras) as usize * 0x18e3)
    }
}



comparar_valor!(comparar_formation_record, formation_record, IDX_FORMATION_RECORD, (f: i16, humano: u32) -> usize);



/// the two extra slots (+0x56, +0x5a) of the club row
/// Original: `extra_tactic_slot` (0x897ee0, tactics.cpp, thiscall).
///
/// [pt] { 0x897ee0 } (thiscall, ret 0x10, tactics.cpp): as duas vagas extras (+0x56 e +0x5a) da linha do
/// `clube` na tabela tática (0x8982b0 com `humano`; sem linha, 0): com `pessoa`, `*pessoa` na vaga
/// `qual` 0/1; sem, a vaga vira −1 e, na 0, a 1 desce para a 0 e fica −1. Devolve 1. `clube` nulo ou
/// `qual` fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn extra_tactic_slot(t: usize, clube: usize, which: i8, person: *const u32, human: u32) -> u32 {
    if clube == 0 || !(0..=1).contains(&which) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_EXTRA_TACTIC_SLOT)(t, clube, which, person, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(clube, human);
    if l == -1 {
        return 0;
    }
    let row = linha_da_tabela(t, l);
    let home = (row + 0x56 + 4 * which as usize) as *mut u32;
    if !person.is_null() {
        crate::comparacao::ramo("vaga extra gravada");
        core::ptr::write_unaligned(home, *person);
        return 1;
    }
    crate::comparacao::ramo("vaga extra apagada");
    core::ptr::write_unaligned(home, u32::MAX);
    if which == 0 {
        let second = core::ptr::read_unaligned((row + 0x5a) as *const u32);
        core::ptr::write_unaligned((row + 0x56) as *mut u32, second);
        core::ptr::write_unaligned((row + 0x5a) as *mut u32, u32::MAX);
    }
    1
}




/// for a human club (without +0x9a01 or with holiday option 2), the slot for the team and the manager active block;
/// otherwise, by the index in the club row, the preset block (below the preset count) or the own one
/// Original: `Tactics::get_club_tactic` (0x896660, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x896660 } (thiscall, ret 8, tactics.cpp): `Tactics::get_club_tactic(clube, time)`, pela formação de cada jogo
/// (0x6e0230), pelas fraquezas do jogador (0x5470d0), pela mídia e pelas telas. Clube humano (0x600a50 com 1, 1) cujo
/// técnico (+0xcf) não tem +0x9a01 (0x5fd210) ou tem a opção 2 das férias (0x5fea50): a vaga pelo time (0x5fbe20) e o
/// bloco ativo dele (0x5fbd00). Nos outros, o índice (palavra com sinal) na linha do clube (id × 0x91 em [this+0x601]):
/// abaixo da conta das táticas prontas ([this+0x5fd]) o bloco pronto ([this+0x5f9]), senão o próprio ([this+0x5f1],
/// índice − conta); blocos de 0x18e3 bytes. Clube nulo: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn get_club_tactic(this: usize, clube: usize, team: u32) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CLUB_TACTIC)(this, clube, team); // caixa: só a original
    }
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human(clube, 1, 1, 0, 0) != 0 {
        let manager = read_usize(G_HUMAN_MANAGER);
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
        let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
        if t9a01(manager, read_usize(clube + club::MANAGER)) == 0 || opcao(manager, 2, read_usize(clube + club::MANAGER)) != 0 {
            let escolher: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
            let active: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_ACTIVE_TACTIC);
            crate::comparacao::ramo("cpx_8 tatica do humano");
            escolher(manager, team, clube, read_usize(clube + club::MANAGER)); // cpx8: a vaga pelo time
            return active(manager, read_usize(clube + club::MANAGER));
        }
    }
    let id = read_i32(clube);
    let indice = read_i16(read_usize(this + 0x601).wrapping_add(id.wrapping_mul(0x91) as usize)) as i32;
    let prontas = read_i32(this + 0x5fd);
    if indice < prontas { // cpx8: tática pronta ou própria
        crate::comparacao::ramo("cpx_8 tatica pronta");
        read_usize(this + 0x5f9).wrapping_add(indice.wrapping_mul(0x18e3) as usize) // cpx8: o bloco pronto
    } else {
        crate::comparacao::ramo("cpx_8 tatica propria");
        read_usize(this + 0x5f1).wrapping_add(indice.wrapping_sub(prontas).wrapping_mul(0x18e3) as usize) // cpx8: o bloco próprio
    }
}


/// the club selection slot (0..19) holding the player, or 0xff
/// Original: `Tactics::find_selection_slot` (0x8972c0, tactics.cpp, thiscall).
///
/// [pt] { 0x8972c0 } (thiscall, ret 0xc, tactics.cpp): a vaga (0..19) da escalação do clube em que está a
/// pessoa: linha tática do clube (0x8982b0 com `time`; -1 devolve 0) e as 20 vagas (dwords em +0x601 +
/// linha x 0x91 + 2) comparadas com o id; não está: 0xff. Clube ou pessoa nulos: a original (caixa
/// de erro).
pub(crate) unsafe extern "thiscall" fn find_selection_slot(tactic: usize, clube: usize, person: *const i32, team: u32) -> u8 {
    if clube == 0 || person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(IDX_FIND_SELECTION_SLOT)(tactic, clube, person, team);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, team);
    if k == -1 {
        return 0;
    }
    let id = *person;
    let slots = core::ptr::read_unaligned((tactic + 0x601) as *const usize).wrapping_add((k as usize).wrapping_mul(0x91) + 2);
    match (0..20).find(|&i| core::ptr::read_unaligned((slots + 4 * i) as *const i32) == id) {
        Some(i) => i as u8,
        None => 0xff,
    }
}


/// stores the person in slot 0..19 of the club row (0x8982b0) and clears the club's pending instructions
/// Original: `set_tactic_slot` (0x896a90, tactics.cpp, thiscall).
///
/// [pt] { 0x896a90 } (thiscall, ret 0x10, tactics.cpp): grava `*pessoa` na vaga `vaga` 0..19 da linha do
/// `clube` (0x8982b0 com `humano`; sem linha, 0) e apaga, entre as 16 instruções pendentes
/// (+0x30, 0x5c bytes: clube, humano, ativa), as ativas desse clube e `humano`, descontando o
/// contador +0x5f0. Devolve 1. `clube` ou `pessoa` nulos, vaga fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn set_tactic_slot(t: usize, clube: usize, slot: i8, person: *const u32, human: u32) -> u32 {
    if clube == 0 || !(0..=19).contains(&slot) || person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_SET_TACTIC_SLOT)(t, clube, slot, person, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(clube, human);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned((linha_da_tabela(t, l) + 2 + 4 * slot as usize) as *mut u32, *person);
    let pendentes = (t + 0x5f0) as *mut u8;
    if *pendentes != 0 {
        for k in 0..16 {
            let e = t + 0x30 + 0x5c * k;
            let active = (e + 8) as *mut u32;
            if *active != 0 && *(e as *const usize) == clube && *((e + 4) as *const u32) == human {
                *active = 0;
                *pendentes = (*pendentes).wrapping_sub(1);
            }
        }
    }
    1
}

/// the slot's person; without a record or from another nation in a national team, the slot becomes −1
/// Original: `tactic_slot_person` (0x8968f0, tactics.cpp, thiscall).
///
/// [pt] { 0x8968f0 } (thiscall, ret 0xc, tactics.cpp): a pessoa (em [0xae23bc], 0x6e bytes) da vaga `vaga`
/// 0..19 da linha do `clube`; 0 sem linha ou vaga vazia (negativa). Vaga com pessoa sem ficha
/// (+0x61), ou, numa seleção (0x53b900), pessoa sem nação (+0x24) ou de outra nação (+0x53): a vaga
/// vira −1 e devolve 0. `clube` nulo ou vaga fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn tactic_slot_person(t: usize, clube: usize, slot: i8, human: u32) -> usize {
    if clube == 0 || !(0..=19).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_TACTIC_SLOT_PERSON)(t, clube, slot, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let l = tactics_row(clube, human);
    if l == -1 {
        return 0;
    }
    let home = (linha_da_tabela(t, l) + 2 + 4 * slot as usize) as *mut i32;
    let id = core::ptr::read_unaligned(home);
    if id <= -1 {
        return 0;
    }
    let persons = *(G_STAFF_TABLE as *const usize);
    let p = persons.wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if core::ptr::read_unaligned((p + 0x61) as *const usize) == 0 {
        core::ptr::write_unaligned(home, -1);
        return 0;
    }
    if national_team(clube) != 0 {
        let nacao = core::ptr::read_unaligned((p + 0x24) as *const usize);
        if nacao == 0 || core::ptr::read_unaligned((nacao + nation::THREE_LETTER_NAME) as *const u32) != core::ptr::read_unaligned((clube + club::NATION) as *const u32) {
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
pub(crate) unsafe extern "thiscall" fn set_tactic_formation(t: usize, clube: usize, formacao: i16, human: u32) -> u32 {
    let limit = core::ptr::read_unaligned((t + 0x5fd) as *const i32)
        .wrapping_add(core::ptr::read_unaligned((t + 0x5f5) as *const i32))
        .wrapping_sub(1);
    if clube == 0 || formacao < 0 || formacao as i32 > limit {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(IDX_SET_TACTIC_FORMATION)(t, clube, formacao, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(clube, human);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned(linha_da_tabela(t, l) as *mut i16, formacao);
    1
}






