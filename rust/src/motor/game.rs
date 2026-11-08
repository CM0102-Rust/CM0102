use super::*;






/// The nation's coefficient history: five past years (+0xb0..+0xd0) and the current year (+0xd8).
///
/// [pt] A história do coeficiente da nação: +0xb0, +0xb8, +0xc0, +0xc8, +0xd0 (cinco anos) e +0xd8 (o ano corrente).
pub(crate) unsafe fn nation_coefficient_history(na: usize) -> [f64; 6] {
    let mut h = [0f64; 6];
    for (k, x) in h.iter_mut().enumerate() {
        *x = read_f64(na + 0xb0 + k * 8);
    }
    h
}

/// Month of the game date (Date get_month on 0xae2c90).
///
/// [pt] O mês da data do jogo (0x54cfb0 sobre 0xae2c90, o al com sinal).
pub(crate) unsafe fn current_month() -> i32 {
    let month: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    month(G_CURRENT_DATE) as u8 as i8 as i32
}

/// December step: adds the reputation bonus if the mark is below 12, shifts the coefficient history one year and zeroes
/// the current year and the mark.
///
/// [pt] Dezembro na história `h` da nação `na` (`marca` = +0x121): com marca < 12, o ano corrente ganha
/// reputação (+0x8e) × 0,05 × (12 − marca) × 3; a história anda um ano, o ano corrente e a marca zeram.
pub(crate) unsafe fn roll_coefficient_year(na: usize, h: &mut [f64; 6], mark: &mut u8) {
    crate::comparacao::ramo("cpt_6 dezembro");
    if (*mark as i8) < 0xc {
        crate::comparacao::ramo("cpt_6 bonus de dezembro");
        h[5] = read_i16(na + 0x8e) as i32 as f64 * read_f64_at(0x967888) * (0xc - *mark as i8 as i32) as f64 * read_f64_at(0x968930) + h[5]; // cpt6: o bônus de dezembro; 0x967888 = 0.05, 0x968930 = 3
    }
    h[0] = h[1];
    h[1] = h[2];
    h[2] = h[3];
    h[3] = h[4];
    h[4] = h[5];
    h[5] = 0.0; // cpt6: o ano corrente zerado
    *mark = 0;
}

/// the first time, each nation history gets the coefficient at +0xa8; on the table day of the month, the weighted
/// history sum in an array that is sorted and freed; on the December day, the current year gets reputation × 0.05 × (12
/// − mark) × 3 (mark < 12), the history moves one year and the current year and the mark are cleared
/// Original: `game_update_nation_coefficients` (0x5d5f20, game.cpp, cdecl).
///
/// [pt] { 0x5d5f20 } (cdecl, game.cpp): `game_update_nation_coefficients()`, todo dia pelo laço do dia do jogo (0x5cda10).
/// Na primeira vez ([0x9cd218]), a história de cada nação (+0xb0..+0xd0) recebe o coeficiente de +0xa8. No dia do
/// mês da tabela 0x9cd200 (palavra pelo mês 0x54cfb0): um vetor (malloc de 12 bytes por nação: a nação e a soma
/// 0,6·c0 + 0,4·b8 + 0,2·b0 + 0,8·c8 + d8 + d0), ordenado (qsort 0x945333 com 0x4afba0) e solto; no dia de dezembro
/// ([0x9cd216]) cada nação passa também pelo bônus e pelo deslocamento da história (`roll_coefficient_year`).
pub(crate) unsafe extern "C" fn game_update_nation_coefficients() {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let qsort: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(FN_CRT_QSORT);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT) != 0 {
        if (read_usize(G_NATION_COUNT) as i32) > 0 {
            let mut p = read_usize(G_NATION_TABLE) + 0xd0;
            let mut i = 0i16;
            loop {
                let v = read_f64(p - 0x28);
                for k in 0..5usize {
                    write_f64(p - k * 8, v);
                }
                i = i.wrapping_add(1);
                p += 0x122;
                if i as i32 >= read_usize(G_NATION_COUNT) as i32 {
                    break;
                }
            }
        }
        *(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT as *mut u8) = 0;
    }
    let today = read_usize(G_CURRENT_DATE) as i16;
    if read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_TABLE + (current_month() * 2) as usize) != today {
        return;
    }
    let array = malloc(read_usize(G_NATION_COUNT).wrapping_mul(0xc));
    if array == 0 {
        show_error_at(0x9cd2a4, 0x156d);
        return;
    }
    let dez = read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_DECEMBER) == read_i16(G_CURRENT_DATE); // cpt6: o dia de dezembro
    let mut n = read_usize(G_NATION_COUNT) as i32;
    if n > 0 {
        let mut na = read_usize(G_NATION_TABLE);
        let mut i = 0i16;
        loop {
            let e = array + i as i32 as usize * 0xc;
            *(e as *mut usize) = na;
            let s = read_f64(na + 0xc0) * read_f64_at(0x968e48) + read_f64(na + 0xb8) * read_f64_at(0x968e00) + read_f64(na + 0xb0) * read_f64_at(0x968918) // 0x968e48 = 0.6, 0x968e00 = 0.4, 0x968918 = 0.2
                + read_f64(na + 0xc8) * read_f64_at(0x9689b0) + read_f64(na + 0xd8) + read_f64(na + 0xd0); // 0x9689b0 = 0.8
            write_f64(e + 4, s);
            if dez {
                let mut h = nation_coefficient_history(na);
                let mut mark = read_u8(na + 0x121);
                roll_coefficient_year(na, &mut h, &mut mark);
                for (k, x) in h.iter().enumerate() {
                    write_f64(na + 0xb0 + k * 8, *x);
                }
                *((na + 0x121) as *mut u8) = mark;
            }
            i = i.wrapping_add(1);
            na += 0x122;
            n = read_usize(G_NATION_COUNT) as i32;
            if i as i32 >= n {
                break;
            }
        }
    }
    qsort(array, n as usize, 0xc, 0x4afba0);
    free(array);
}


/// 1 on days 0, 80, 173 and 250 of the year (the retirement scan days), otherwise 0
/// Original: `is_player_regen_day` (0x5d6250, game.cpp, cdecl).
///
/// [pt] { 0x5d6250 } (cdecl, game.cpp): `is_player_regen_day(dia)`: 1 nos dias 0, 80, 173 e 250 do ano (a busca das
/// aposentadorias), senão 0. Pelo laço do dia do jogo (0x5cda97, com a data inteira: o dia é a palavra baixa).
/// A original só devolve o al (o ah fica com o byte alto do dia).
pub(crate) unsafe extern "C" fn is_player_regen_day(day: u32) -> u32 {
    let d = day as u16;
    (d == 0 || d == 0x50 || d == 0xad || d == 0xfa) as u32
}


/// the sum of two signed bytes clamped between the given minimum and maximum
/// Original: `clamp_char_sum` (0x5d61d0, game.cpp, cdecl).
///
/// [pt] { 0x5d61d0 } (cdecl, game.cpp): `clamp_char_sum(a, b, min, max)`: a soma dos dois bytes com sinal (16 bits),
/// presa entre os outros dois. Devolve o ax da original: `min` ou `max` estendidos, ou o byte alto do `max`
/// estendido com a soma no al (os 26 chamadores, no diário dos jogadores, só gravam o al).
pub(crate) unsafe extern "C" fn clamp_char_sum(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let s = (a as u8 as i8 as i16).wrapping_add(b as u8 as i8 as i16);
    let lo = min as u8 as i8 as i16;
    let hi = max as u8 as i8 as i16;
    if lo > s {
        lo as u16 as u32
    } else if hi < s {
        hi as u16 as u32
    } else {
        ((hi as u16 & 0xff00) | (s as u16 & 0xff)) as u32
    }
}







/// get_contract for a person, stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
pub(crate) unsafe fn game_get_contract_and_store(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), p);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
    c
}

/// Club of a contract (by id at +4), or 0.
///
/// [pt] O clube do contrato (pelo id em +4), ou 0 sem contrato.
pub(crate) unsafe fn game_club_of_contract(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        club_record(read_usize(c + 4))
    }
}

/// CONTRACT_MANAGER get_contract_for_club (0x4cf5f0).
///
/// [pt] O vínculo da pessoa com o clube (0x4cf5f0).
pub(crate) unsafe fn game_contract_for_club(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    f(read_usize(G_CONTRACT_MANAGER), p, clube)
}

/// staff_is_disliked_by (0x547840): does `pessoa` dislike `alvo`.
///
/// [pt] `pessoa` não gosta de `alvo` (0x547840)?
pub(crate) unsafe fn is_disliked_by(target: usize, person: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    f(target, person)
}

/// Shared end of 0x5d6d10/0x5d7190: finds a squad mate the person dislikes and, on the rare date condition, posts
/// dressing-room unrest and sets the contract event flag.
///
/// [pt] O fim comum do atrito (0x5d6d10 e 0x5d7190): o primeiro colega do elenco (+0xd7, 50 vagas) de quem a pessoa não
/// gosta; achado, e só quando dia % 14 + id da pessoa dá zero (na prática a pessoa de id 0 a cada 14 dias), a queixa
/// (0x587c60) contra o técnico (+0xcf, tipo 0), o auxiliar (+0xd3, 1) ou outro (2) e a marca no vínculo `k` (0x867700
/// com 0x2000, 0x1000 ou 0x800).
pub(crate) unsafe fn dressing_room_friction(p: usize, clube: usize, k: usize) {
    let mut j = 0usize;
    loop {
        if j >= 0x32 {
            return;
        }
        let q = read_usize(clube + club::SQUAD + j * 4);
        if q != p && q != 0 && is_disliked_by(q, p) != 0 {
            break;
        }
        j += 1;
    }
    let q = read_usize(clube + club::SQUAD + j * 4);
    if q == 0 {
        return;
    }
    if ((read_i16(G_CURRENT_DATE) as i32) % 0xe).wrapping_add(read_usize(p) as i32) != 0 {
        return;
    }
    crate::comparacao::ramo("cps_5 atrito com o tecnico");
    let queixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_DRESSING_ROOM_UNREST);
    let mark: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    let (tipo, marca) = if q == read_usize(clube + club::MANAGER) {
        (0, 0x2000)
    } else if q == read_usize(clube + club::ASSISTANT_MANAGER) {
        (1, 0x1000)
    } else {
        (2, 0x800)
    };
    queixa(p, q, tipo);
    mark(k, marca, 0);
}

/// ten random pairs of team-mates: a disliked one stops being disliked and may fight (0x5888a0); otherwise becomes
/// disliked, liked or stops being liked; every 50 days, random clubs go through the dressing room friction; on the
/// other days each player, by attribute draws, may reach the friction with the manager
/// Original: `game_update_squad_relationships` (0x5d6d10, game.cpp, cdecl).
///
/// [pt] { 0x5d6d10 } (cdecl, game.cpp): `game_update_squad_relationships()`, todo dia pelo laço do dia do jogo. (1) Dez
/// vezes: um clube sorteado e duas vagas do elenco sorteadas (50); pessoas diferentes e sorteio(+0x5b da primeira) = 0:
/// se a primeira não gosta da segunda, deixa de não gostar (0x547c50) e, no mesmo clube de contrato e com o vínculo
/// marcado em +0x45 & 0x800, a briga (0x5888a0); senão, 1/3 passa a não gostar (0x547af0), 1/3 do resto passa a gostar
/// (0x547a10) e o resto deixa de gostar (0x547c10). (2) Dia múltiplo de 50: [0xae235c]/100 clubes sorteados passam
/// pelo atrito no vestiário (0x5d7190). Nos outros dias, cada pessoa (menos as 17 últimas) com ficha de jogador e
/// preferências, contrato e o vínculo do clube com a pessoa de +0xff: três sorteios (+0x5b, +0x5d, +0x5a, cada um + 5)
/// filtram; a soma de cinco atributos menos 3, 5, 15, 15 por sorteios zerados e 5 se a reputação do jogador (+0xb)
/// passa de 4500 e da do clube (+0x80) dá o sorteio que, zerado, leva ao atrito com o técnico.
pub(crate) unsafe extern "C" fn game_update_squad_relationships() {
    let tirar_rejeitada: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_STAFF_REMOVE_DISLIKED_STAFF);
    let briga: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_NEWS_POST_FEUD_SETTLED);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let favorecer: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_STAFF);
    let tirar_favorita: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_STAFF_REMOVE_FAVOURED_STAFF);
    for _ in 0..10 {
        let clube = club_record(friendly_random(read_usize(G_CLUB_COUNT) as i32) as usize);
        let a = read_usize(clube.wrapping_add(friendly_random(0x32) as usize * 4) + 0xd7);
        let b = read_usize(clube.wrapping_add(friendly_random(0x32) as usize * 4) + 0xd7);
        if a == 0 || b == 0 || a == b {
            continue;
        }
        if friendly_random(read_i8_as_i32(a + 0x5b)) != 0 {
            continue;
        }
        if is_disliked_by(b, a) != 0 {
            tirar_rejeitada(a, b);
            let c = game_get_contract_and_store(a);
            if c == 0 || game_club_of_contract(c) == 0 {
                continue;
            }
            let ca = game_club_of_contract(game_get_contract_and_store(a));
            let cb = game_club_of_contract(game_get_contract_and_store(b));
            if ca != cb {
                continue;
            }
            let k = game_contract_for_club(a, game_club_of_contract(game_get_contract_and_store(a)));
            if k == 0 || read_usize(k + 0x45) & 0x800 == 0 {
                continue;
            }
            briga(a, b);
        } else if friendly_random(3) == 0 {
            rejeitar(a, b, 1);
        } else if friendly_random(3) == 0 {
            favorecer(a, b, 1);
        } else {
            tirar_favorita(a, b);
        }
    }
    if (read_i16(G_CURRENT_DATE) as i32) % 0x32 == 0 {
        let atrito: unsafe extern "C" fn(usize) = core::mem::transmute(FN_GAME_CHECK_CLUB_DRESSING_ROOM_UNREST);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 / 100 {
            atrito(club_record(friendly_random(read_usize(G_CLUB_COUNT) as i32) as usize));
            i += 1;
        }
        return;
    }
    let base = read_usize(G_STAFF_TABLE);
    let end = base.wrapping_add((read_usize(G_STAFF_COUNT).wrapping_sub(17)).wrapping_mul(staff::SIZE));
    let mut p = base;
    while p < end {
        player_vs_manager_check(p);
        p += 0x6e;
    }
}

/// One person of the non-50th-day loop: rolls the player's attitude toward the club's manager and ends with
/// dressing_room_friction [inference].
///
/// [pt] Uma pessoa do laço de fora dos dias múltiplos de 50 (0x5d6f15..0x5d7115).
pub(crate) unsafe fn player_vs_manager_check(p: usize) {
    if read_usize(p + 0x61) == 0 || read_usize(p + 0x65) == 0 {
        return;
    }
    let c = game_get_contract_and_store(p);
    if c == 0 {
        return;
    }
    let clube = game_club_of_contract(c);
    if clube == 0 {
        return;
    }
    let k = game_contract_for_club(read_usize(clube + 0xff), clube);
    if k == 0 {
        return;
    }
    if friendly_random(read_i8_as_i32(p + 0x5b) + 5) != 0 && friendly_random(read_i8_as_i32(p + 0x5d) + 5) != 0 && friendly_random(read_i8_as_i32(p + 0x5a) + 5) != 0 {
        return;
    }
    let mut e = read_i8_as_i32(p + 0x5b) + (read_i8_as_i32(p + 0x5a) + read_i8_as_i32(p + 0x59)) + (read_i8_as_i32(p + 0x5c) + read_i8_as_i32(p + 0x18));
    crate::comparacao::ramo("cps_5 tecnico sorteado");
    if friendly_random(read_i8_as_i32(p + 0x59)) == 0 { // cps5: o primeiro desconto
        e -= 3;
    }
    if friendly_random(read_i8_as_i32(p + 0x5a)) == 0 {
        e -= 5;
    }
    if read_usize(k + 0x45) != 0 || friendly_random(0x64) == 0 {
        if friendly_random(read_i8_as_i32(p + 0x5b)) == 0 {
            e -= 0xf;
        }
        if friendly_random(read_i8_as_i32(p + 0x5d)) == 0 {
            e -= 0xf;
        }
    }
    let rep = read_i16(read_usize(p + 0x61) + 0xb);
    if rep > 0x1194 && rep > read_i16(clube + club::REPUTATION) {
        e -= 5;
    }
    if friendly_random(e) >= 1 {
        return;
    }
    if read_usize(p + 0x65) == 0 {
        return;
    }
    dressing_room_friction(p, clube, k);
}

// simplification: rolls and touches the preferences (0x547a10, 0x547af0, 0x547c10, 0x547c50), quarrel and complaint: original only;
// the copy takes the branches (runs every day).

/// each squad player with a contract and preferences: the first team-mate he dislikes; with day % 14 + id = 0, the
/// complaint against the manager, the assistant or another (0x587c60) and the flag on the contract (0x867700)
/// Original: `game_check_club_dressing_room_unrest` (0x5d7190, game.cpp, cdecl).
///
/// [pt] { 0x5d7190 } (cdecl, game.cpp): `game_check_club_dressing_room_unrest(clube)`: cada pessoa do elenco (+0xd7, 50
/// vagas) com vínculo com o clube (0x4cf5f0) e preferências (+0x65) passa pelo atrito com o técnico (o primeiro colega
/// de quem não gosta; a queixa só com dia % 14 + id = 0). Pelas relações do elenco (0x5d6d10) nos dias múltiplos de 50.
pub(crate) unsafe extern "C" fn game_check_club_dressing_room_unrest(clube: usize) {
    for i in 0..0x32usize {
        let p = read_usize(clube + club::SQUAD + i * 4);
        if p == 0 {
            continue;
        }
        let k = game_contract_for_club(p, clube);
        if k == 0 {
            continue;
        }
        if read_usize(p + 0x65) == 0 {
            continue;
        }
        dressing_room_friction(p, clube, k);
    }
}

// simplification: the complaint (0x587c60) and the mark on the link (0x867700) do not run twice: original only; the copy carries the
// friction branch.

/// True if the person has a valid id, belongs to the club (+0x39) and has a player record.
///
/// [pt] A pessoa conta para a liderança do clube: id válido (< [0xae2364]), contrato com o clube (+0x39) e ficha de jogador.
pub(crate) unsafe fn is_club_squad_player(p: usize, clube: usize) -> bool {
    p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && read_usize(p + 0x39) == clube && read_usize(p + 0x61) != 0
}

/// one club in seven each day (not national teams): squad leadership and leadership × morale summed; each player draws
/// from them and his own morale and the morale goes up or down by one (0x544710)
/// Original: `game_spread_squad_morale_by_leadership` (0x5ce0b0, game.cpp, cdecl).
///
/// [pt] { 0x5ce0b0 } (cdecl, game.cpp): `game_spread_squad_morale_by_leadership()`, todo dia pelo laço do dia do jogo. Cada
/// clube que não é seleção (0x53b900), com sorteio(7) = 0: soma a liderança (jogador +0x2f) e liderança × moral do
/// vínculo (+0x44) do elenco (+0xd7, 50 vagas); menos de 16 jogadores contam 10 cada um que falta. Com soma não nula,
/// cada jogador sorteia (2 × Σ liderança × moral) / (liderança dele + soma) mais sorteio(moral / 12), em byte com sinal:
/// negativo, sorteio(s) < sorteio(−5) baixa o moral (0x544710 com −1); positivo, sorteio(s) > sorteio(5) sobe (+1).
pub(crate) unsafe extern "C" fn game_spread_squad_morale_by_leadership() {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let morale: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let n = read_usize(G_CLUB_COUNT);
    let mut clube = read_usize(G_CLUB_TABLE);
    let end = clube.wrapping_add(n.wrapping_mul(club::SIZE));
    while clube < end {
        let slots = clube + club::SQUAD;
        if is_national_team(clube) == 0 && friendly_random(7) == 0 {
            let (mut soma, mut lideranca, mut q) = (0i32, 0i32, 0i32);
            for i in 0..0x32usize {
                let p = read_usize(slots + i * 4);
                if !is_club_squad_player(p, clube) {
                    continue;
                }
                let c = game_contract_for_club(p, clube);
                if c == 0 {
                    continue;
                }
                let l = read_i8_as_i32(read_usize(p + 0x61) + 0x2f);
                soma = soma.wrapping_add(read_i8_as_i32(c + 0x44) * l);
                lideranca += l;
                q += 1;
            }
            if q < 0x10 {
                lideranca += (0x10 - q) * 10;
            }
            for i in 0..0x32usize {
                if lideranca == 0 {
                    break;
                }
                let p = read_usize(slots + i * 4);
                if !is_club_squad_player(p, clube) {
                    continue;
                }
                let c = game_contract_for_club(p, clube);
                let m = if c == 0 { 0 } else { read_i8_as_i32(c + 0x44) };
                let l = read_i8_as_i32(read_usize(p + 0x61) + 0x2f);
                crate::comparacao::ramo("cps_5 moral sorteado");
                let r1 = friendly_random(soma.wrapping_mul(2) / (l + lideranca));
                let r2 = friendly_random(m / 12); // cps5: a parte do moral
                let s = (r1 as u8).wrapping_add(r2 as u8) as i8;
                if s < 0 {
                    let e = friendly_random(s as i32);
                    if e < friendly_random(-5) {
                        morale(p, -1, 1);
                    }
                } else if s > 0 {
                    let e = friendly_random(s as i32);
                    if e > friendly_random(5) {
                        crate::comparacao::ramo("cps_5 moral sobe");
                        morale(p, 1, 1); // cps5: sobe o moral
                    }
                }
            }
        }
        clube = clube.wrapping_add(0x245);
    }
}

// simplification: rolls random numbers and changes morale (0x544710, which also rolls): original only; the copy takes the branches.








/// Club record of a contract (G_CLUB_TABLE by id at +4), or 0.
///
/// [pt] O clube do contrato (registros de 0x245 bytes em [0xae23b4], pelo id em +4), ou 0 sem contrato.
pub(crate) unsafe fn contract_club_record(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
    }
}

/// Days from the person's date at +0x3e to today.
///
/// [pt] Dias desde a data de +0x3e da pessoa até hoje (0x54cdb0 sobre a data do jogo 0xae2c90, a outra por valor).
pub(crate) unsafe fn days_since_staff_date(p: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    f(G_CURRENT_DATE, read_usize(p + 0x3e) as u32, read_usize(p + 0x42) as u32)
}

/// n/250 consecutive people from a random one: a present player, with training and without a serious injury misses
/// training (0x41a360) by a draw on the sum of attributes, morale and likes of the manager (more likely when newly
/// arrived) or by the contract flags
/// Original: `game_check_players_missing_training` (0x5d6aa0, game.cpp, cdecl).
///
/// [pt] { 0x5d6aa0 } (cdecl, game.cpp): `game_check_players_missing_training()`, todo dia pelo laço do dia do jogo. A partir
/// de uma pessoa sorteada (sorteio(n − n/250 − 1)), n/250 pessoas seguidas: jogador (+0x61, função +0x3d = 11) com
/// contrato e clube, presente (0x418f60), com o registro de treino (0x8b39f0, +0x15) e sem lesão grave (0x62c400 ≠ 2).
/// (1) Com +0x5b < 8, jogador +0x2f < 8 e a soma (jogador +0x2f, +0x18, +0x5b, vínculo +0x44, −5 se +0x5b ou o moral
/// valem 1, −5 se não gosta do técnico, +2 se gosta) < 43: sorteio(+0x5b + 100, 50 ou 25 conforme os dias desde +0x3e:
/// < 30, < 90, < 180) zerado é falta. (2) Senão, com +0x5b < 10 e vínculo com o clube: as marcas do vínculo (+0x45:
/// 0x8000, 0x30000; com +0x5b < 5, 0x7800) dão sorteios(+0x5b) que, zerados, são falta. A falta: 0x41a360.
pub(crate) unsafe extern "C" fn game_check_players_missing_training() {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
    let treino: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_PLAYER_SCHEDULE);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let foul: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_MISS_TRAINING);
    let n = read_usize(G_STAFF_COUNT) as i32;
    let q = n / 250;
    crate::comparacao::ramo("cps_6 faltas do dia");
    let r = friendly_random(n - q - 1); // cps6: a primeira pessoa
    let mut current = read_usize(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(staff::SIZE));
    for _ in 0..q {
        let p = current;
        current = current.wrapping_add(0x6e);
        if read_usize(p + 0x61) == 0 || read_u8(p + 0x3d) != 0xb {
            continue;
        }
        let c = game_get_contract_and_store(p);
        if c == 0 {
            continue;
        }
        let clube = contract_club_record(c);
        if clube == 0 {
            continue;
        }
        if ausente(read_usize(G_AWOL), p, 0) != 0 {
            continue;
        }
        let t = treino(read_usize(G_TRAINING_MANAGER), p);
        if t == 0 || read_u8(t + 0x15) == 0 {
            continue;
        }
        if lesao(read_usize(G_INJURY), p) == 2 {
            continue;
        }
        let jog = read_usize(p + 0x61);
        let cl = read_u8(p + 0x5b);
        let dl = read_u8(jog + 0x44);
        let mut bl = read_u8(jog + 0x2f).wrapping_add(read_u8(p + 0x18)).wrapping_add(cl).wrapping_add(dl);
        if cl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if dl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if not_likes(read_usize(clube + club::MANAGER), p) != 0 {
            bl = bl.wrapping_add(0xfb);
        }
        if gosta(read_usize(clube + club::MANAGER), p) != 0 {
            bl = bl.wrapping_add(2);
        }
        let mut and_foul = false;
        let a = read_u8(p + 0x5b);
        if (a as i8) < 8 && (read_u8(read_usize(p + 0x61) + 0x2f) as i8) < 8 && (bl as i8) < 0x2b {
            let mut s = a;
            if days_since_staff_date(p) < 0x1e {
                s = s.wrapping_add(0x64);
            } else if days_since_staff_date(p) < 0x5a {
                s = s.wrapping_add(0x32);
            } else if days_since_staff_date(p) < 0xb4 {
                s = s.wrapping_add(0x19);
            }
            crate::comparacao::ramo("cps_6 falta pela soma");
            if friendly_random(s as i8 as i32) == 0 { // cps6: o sorteio da soma
                and_foul = true;
            }
        }
        if !and_foul {
            if read_i8_as_i32(p + 0x5b) >= 0xa {
                continue;
            }
            let clube = contract_club_record(game_get_contract_and_store(p));
            let v = contract_bond(read_usize(G_CONTRACT_MANAGER), p, clube);
            if v == 0 {
                continue;
            }
            if read_usize(v + 0x45) & 0x8000 != 0 && friendly_random(read_i8_as_i32(p + 0x5b)) == 0 {
                and_foul = true;
            } else if read_usize(v + 0x45) & 0x30000 != 0 && friendly_random(read_i8_as_i32(p + 0x5b)) == 0 {
                and_foul = true;
            } else {
                let a = read_i8_as_i32(p + 0x5b);
                if a >= 5 || read_usize(v + 0x45) & 0x7800 == 0 || friendly_random(a) != 0 {
                    continue;
                }
                and_foul = true;
            }
        }
        if and_foul {
            crate::comparacao::ramo("cps_6 falta ao treino");
            foul(read_usize(G_AWOL), p); // cps6: a falta
        }
    }
}

// simplification: draws and the foul (0x41a360) fines, draws and changes the form: original only; the copy carries the branches.

/// each competition in the list gets the day (virtual +0x14 with the game date and the flag); with the flag, the next
/// step of [0xae2a58] (0x5aaf00)
/// Original: `game_update_comps_for_date` (0x5ce060, game.cpp, cdecl).
///
/// [pt] { 0x5ce060 } (cdecl, game.cpp): `game_update_comps_for_date(marca)`: cada competição da lista ([0xadadfc], conta
/// [0xadadf8] relida a cada volta) recebe o dia (virtual +0x14 com a data do jogo 0xae2c90 e a marca); com a marca, o
/// passo seguinte do [0xae2a58] (0x5aaf00, que a original chama em salto de cauda). Pelo laço do dia do jogo (com 1).
pub(crate) unsafe extern "C" fn game_update_comps_for_date(mark: u32) {
    let m = mark as u8 as i8;
    let mut i = 0i32;
    while i < read_usize(G_COMP_OBJECT_COUNT) as i32 {
        let c = read_usize(read_usize(G_COMP_OBJECT_TABLE) + i as usize * 4);
        if c != 0 {
            let day: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(read_usize(read_usize(c) + 0x14));
            day(c, G_CURRENT_DATE, m as i32);
        }
        i += 1;
    }
    if m != 0 {
        crate::comparacao::ramo("cps_6 competicoes do dia");
        let next: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIXTURE_MANAGER_DAILY_UPDATE);
        next(read_usize(G_FIXTURE_MANAGER));
    }
}

// simplification: advances the day of all competitions (virtual) and of [0xae2a58]: original only; the copy takes the branch.

/// Address of the club's board confidence word (+6 of its 73-byte row in [this]).
///
/// [pt] A confiança da diretoria no clube: a palavra +6 da linha do clube (73 bytes por id em [this]).
pub(crate) unsafe fn board_confidence_ptr(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49)) + 6
}

/// adds the delta to the board confidence in the club (+6 of the 73-byte club row), held to 1..10000
/// Original: `manager_manager_adjust_board_confidence` (0x5cdde0, game.cpp, thiscall, ret 8).
///
/// [pt] { 0x5cdde0 } (thiscall, ret 8, game.cpp): `manager_manager_adjust_board_confidence(clube, delta)`: soma `delta`
/// (palavra) à confiança da diretoria no clube (+6 da linha do MANAGER_MANAGER [0xb67a44]), presa em 1..10000. Clube
/// nulo: nada. Pelo laço do dia do jogo (com −5).
pub(crate) unsafe extern "thiscall" fn manager_manager_adjust_board_confidence(this: usize, clube: usize, delta: u32) {
    if clube == 0 {
        return;
    }
    let c = board_confidence_ptr(this, clube);
    write_i16(c, read_i16(c).wrapping_add(delta as u16 as i16));
    let v = read_i16(c);
    if v < 1 {
        write_i16(c, 1);
    } else if v > 0x2710 {
        write_i16(c, 0x2710);
    }
}


/// the shortlist manager kept in the transfer manager
/// Original: `transfer_manager_get_shortlist_manager` (0x5d5ab0, game.cpp, thiscall).
///
/// [pt] { 0x5d5ab0 } (thiscall, game.cpp): o gerente da lista de observados guardado em +0x856 do gerente
/// de transferências.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_shortlist_manager(manager: usize) -> usize {
    core::ptr::read_unaligned((manager + 0x856) as *const usize)
}

/// the player's 6-byte entry in the training table
/// Original: `training_manager_get_player_entry` (0x5d5ac0, game.cpp, thiscall).
///
/// [pt] { 0x5d5ac0 } (thiscall, ret 4, game.cpp): a entrada de 6 bytes do jogador na tabela do treino
/// ([treino+0x651d]), pelo id dos dados do jogador ([[jogador+0x61]]).
pub(crate) unsafe extern "thiscall" fn training_manager_get_player_entry(treino: usize, player: usize) -> usize {
    let table = core::ptr::read_unaligned((treino + 0x651d) as *const usize);
    let id = *(core::ptr::read_unaligned((player + 0x61) as *const usize) as *const i32);
    (table as isize + id as isize * 6) as usize
}

/// Pointer to the training load byte (+0x1e) of a player's 31-byte condition record.
///
/// [pt] O byte da carga de treino do jogador `*id` no registro de condição (31 bytes por jogador, +0x1e).
pub(crate) unsafe fn training_load_byte(injuries: usize, id: *const i32) -> *mut u8 {
    (*(injuries as *const usize) as isize + *id as isize * 31 + 0x1e) as *mut u8
}





/// the f64 average +0x20/+0x28 or +0x10/+0x18 (zero divisor: 6.8)
/// Original: `game_average` (0x5d5a60, game.cpp, thiscall).
///
/// [pt] { 0x5d5a60 } (thiscall, ret 4, game.cpp): a média +0x20/+0x28 (com `segunda`) ou +0x10/+0x18 (f64,
/// como a original deixa no x87); com divisor zero (ou não comparável), 6.8.
pub(crate) unsafe extern "thiscall" fn game_average(t: usize, second: u8) -> f64 {
    let d = |o: usize| core::ptr::read_unaligned((t + o) as *const f64);
    let (n, q) = if second != 0 { (d(0x20), d(0x28)) } else { (d(0x10), d(0x18)) };
    if q == read_f64_at(0x9689a0) || q.is_nan() { // 0x9689a0 = 0
        return read_f64_at(0x968998); // 0x968998 = 6.8
    }
    n / q
}




comparar_valor!(comparar_contract_value, contract_value, IDX_CONTRACT_VALUE, (p: *const i32) -> u32);

/// the game CRT pow (0x945ff0) of two doubles, result in st0
/// Original: `pow_double` (0x5d5a20, game.cpp, cdecl).
///
/// [pt] { 0x5d5a20 } (cdecl, game.cpp): `pow_double(base, expoente)`: o pow da CRT do jogo (0x945ff0), resultado em st0.
pub(crate) unsafe extern "C" fn pow_double(base: f64, expoente: f64) -> f64 {
    game_pow(base, expoente)
}


/// player data qsort: current ability (+5), then potential (+7), ascending
/// Original: `compare_players_by_ability` (0x5d6220, game.cpp, cdecl).
///
/// [pt] { 0x5d6220 } (cdecl, game.cpp): `compare_players_by_ability(a, b)`, o qsort dos dados de jogador em
/// game_daily_staff_update (0x5d588f): pela habilidade atual (+5), depois pela potencial (+7), crescente.
pub(crate) unsafe extern "C" fn compare_players_by_ability(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    let h = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16) as i32;
    if h(pa, 5) == h(pb, 5) {
        return h(pa, 7) - h(pb, 7);
    }
    h(pa, 5) - h(pb, 5)
}

