use super::*;

/// The nation's coefficient history: five past years (+0xb0..+0xd0) and the current year (+0xd8).
///
/// [pt] A história do coeficiente da nação: +0xb0, +0xb8, +0xc0, +0xc8, +0xd0 (cinco anos) e +0xd8 (o ano corrente).
pub(crate) unsafe fn nation_coefficient_history(na: Nation) -> [f64; 6] {
    let mut h = [0f64; 6];
    for (k, x) in h.iter_mut().enumerate() {
        *x = read_f64(na.addr() + nation::FIFACOEFFICIENT91 + k * 8);
    }
    h
}

/// Month of the game date (Date get_month on 0xae2c90).
///
/// [pt] O mês da data do jogo (0x54cfb0 sobre 0xae2c90, o al com sinal).
pub(crate) unsafe fn current_month() -> i32 {
    (exe::date_get_month(G_CURRENT_DATE) as i8) as i32
}

/// December step: adds the reputation bonus if the mark is below 12, shifts the coefficient history one year and zeroes
/// the current year and the mark.
///
/// [pt] Dezembro na história `h` da nação `na` (`marca` = +0x121): com marca < 12, o ano corrente ganha
/// reputação (+0x8e) × 0,05 × (12 − marca) × 3; a história anda um ano, o ano corrente e a marca zeram.
pub(crate) unsafe fn roll_coefficient_year(na: Nation, h: &mut [f64; 6], mark: &mut u8) {
    crate::comparison::branch("cpt_6 December");
    if (*mark as i8) < 0xc {
        crate::comparison::branch("cpt_6 December bonus");
        h[5] = na.reputation() as i32 as f64 * read_f64(rdata::F64_0_05) * (0xc - *mark as i8 as i32) as f64 * read_f64(rdata::F64_3) + h[5]; // cpt6: the December bonus;
    }
    h[0] = h[1];
    h[1] = h[2];
    h[2] = h[3];
    h[3] = h[4];
    h[4] = h[5];
    h[5] = 0.0; // cpt6: the current year zeroed
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
    if read_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT) != 0 {
        if (read_usize(G_NATION_COUNT) as i32) > 0 {
            let mut p = read_usize(G_NATION_TABLE) + nation::FIFACOEFFICIENT95;
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
        write_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT, 0);
    }
    let today = read_usize(G_CURRENT_DATE) as i16;
    if read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_TABLE + (current_month() * 2) as usize) != today {
        return;
    }
    let array = exe::crt_malloc((read_usize(G_NATION_COUNT).wrapping_mul(0xc)) as u32);
    if array == 0 {
        show_error_at(0x9cd2a4, 0x156d);
        return;
    }
    let dez = read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_DECEMBER) == read_i16(G_CURRENT_DATE); // cpt6: the December day
    let mut n = read_usize(G_NATION_COUNT) as i32;
    if n > 0 {
        let mut na = read_usize(G_NATION_TABLE);
        let mut i = 0i16;
        loop {
            let e = array + i as i32 as usize * 0xc;
            write_usize(e, na);
            let s = read_f64(na + nation::FIFACOEFFICIENT93) * read_f64(rdata::F64_0_6) + read_f64(na + nation::FIFACOEFFICIENT92) * read_f64(rdata::F64_0_4) + read_f64(na + nation::FIFACOEFFICIENT91) * read_f64(rdata::F64_0_2)
                + read_f64(na + nation::FIFACOEFFICIENT94) * read_f64(rdata::F64_0_8) + read_f64(na + nation::FIFACOEFFICIENT96) + read_f64(na + nation::FIFACOEFFICIENT95);
            write_f64(e + 4, s);
            if dez {
                let mut h = nation_coefficient_history(Nation(na));
                let mut mark = read_u8(na + nation::GAMES_PLAYED);
                roll_coefficient_year(Nation(na), &mut h, &mut mark);
                for (k, x) in h.iter().enumerate() {
                    write_f64(na + nation::FIFACOEFFICIENT91 + k * 8, *x);
                }
                write_u8(na + nation::GAMES_PLAYED, mark);
            }
            i = i.wrapping_add(1);
            na += 0x122;
            n = read_usize(G_NATION_COUNT) as i32;
            if i as i32 >= n {
                break;
            }
        }
    }
    exe::crt_qsort(array, (n as usize) as u32, (0xc) as u32, 0x4afba0);
    exe::crt_free(array);
}

/// Returns 1 on days 0, 80, 173 and 250 of the year (the retirement scan days), otherwise 0.
///
/// `is_player_regen_day(day)`: 1 on days 0, 80, 173 and 250 of the year (the retirement search), otherwise 0. Called
/// from the game's day loop (0x5cda97, with the whole date: the day is the low word). The original returns only `al`
/// (`ah` keeps the day's high byte).
/// Original: `is_player_regen_day` (0x5d6250, game.cpp, cdecl).
///
/// [pt] { 0x5d6250 } (cdecl, game.cpp): `is_player_regen_day(dia)`: 1 nos dias 0, 80, 173 e 250 do ano (a busca das
/// aposentadorias), senão 0. Pelo laço do dia do jogo (0x5cda97, com a data inteira: o dia é a palavra baixa).
/// A original só devolve o al (o ah fica com o byte alto do dia).
pub(crate) unsafe extern "C" fn is_player_regen_day(day: u32) -> u32 {
    let d = day as u16;
    (d == 0 || d == 0x50 || d == 0xad || d == 0xfa) as u32
}

/// Returns the sum of two signed bytes clamped between the given minimum and maximum.
///
/// `clamp_char_sum(a, b, min, max)`: the sum of the two signed bytes (16 bits), clamped between the other two. Returns
/// the original's `ax`: `min` or `max` extended, or the high byte of `max` extended with the sum in `al` (the 26
/// callers, in the players' daily pass, write only `al`).
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
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// Club of a contract (by id at +4), or 0.
///
/// [pt] O clube do contrato (pelo id em +4), ou 0 sem contrato.
pub(crate) unsafe fn game_club_of_contract(c: usize) -> Club {
    if c == 0 {
        Club(0)
    } else {
        Club(club_record(read_usize(c + 4)))
    }
}

/// CONTRACT_MANAGER get_contract_for_club (`exe::contract_manager_get_contract_for_club`).
///
/// [pt] O vínculo da pessoa com o clube (0x4cf5f0).
pub(crate) unsafe fn game_contract_for_club(p: usize, club: Club) -> Contract {
    Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club))
}

/// staff_is_disliked_by (`exe::staff_is_disliked_by`): does `pessoa` dislike `alvo`.
///
/// [pt] `pessoa` não gosta de `alvo` (0x547840)?
pub(crate) unsafe fn is_disliked_by(target: usize, person: Staff) -> u32 {
    exe::staff_is_disliked_by(target, person)
}

/// The shared end of the friction routines (`game_update_squad_relationships` and `game_check_club_dressing_room_unrest`): finds a squad mate the person dislikes and, on the
/// rare date condition, posts the complaint and sets the link mark.
///
/// The first squad mate (+0xd7, 50 slots) the person does not like; if found, and only when day % 14 + the person's id
/// gives zero (in practice the person with id 0, every 14 days), the complaint (`exe::news_post_dressing_room_unrest`) against the manager (+0xcf,
/// type 0), the assistant (+0xd3, 1) or someone else (2) and the mark on link `k` (`exe::staff_contract_apply_event_flags` with 0x2000, 0x1000 or
/// 0x800).
///
/// [pt] O fim comum do atrito (0x5d6d10 e 0x5d7190): o primeiro colega do elenco (+0xd7, 50 vagas) de quem a pessoa não
/// gosta; achado, e só quando dia % 14 + id da pessoa dá zero (na prática a pessoa de id 0 a cada 14 dias), a queixa
/// (0x587c60) contra o técnico (+0xcf, tipo 0), o auxiliar (+0xd3, 1) ou outro (2) e a marca no vínculo `k` (0x867700
/// com 0x2000, 0x1000 ou 0x800).
pub(crate) unsafe fn dressing_room_friction(p: Staff, club: Club, k: usize) {
    let mut j = 0usize;
    loop {
        if j >= 0x32 {
            return;
        }
        let q = club.squad(j);
        if q != p.addr() && q != 0 && is_disliked_by(q, p) != 0 {
            break;
        }
        j += 1;
    }
    let q = club.squad(j);
    if q == 0 {
        return;
    }
    if ((read_i16(G_CURRENT_DATE) as i32) % 0xe).wrapping_add(read_usize(p.addr()) as i32) != 0 {
        return;
    }
    crate::comparison::branch("cps_5 friction with the manager");
    let (tipo, marca) = if q == club.manager().addr() {
        (0, 0x2000)
    } else if q == club.assistant_manager() {
        (1, 0x1000)
    } else {
        (2, 0x800)
    };
    exe::news_post_dressing_room_unrest(p, q, tipo);
    exe::staff_contract_apply_event_flags(k, marca, 0);
}

/// Updates the relationships in the squads: ten random pairs of team-mates: a disliked one stops being disliked and may
/// fight (`exe::news_post_feud_settled`); otherwise becomes disliked, liked or stops being liked; every 50 days, random clubs go through the
/// dressing room friction; on the other days each player, by attribute draws, may reach friction with the manager.
///
/// `game_update_squad_relationships()`, every day from the game's day loop. (1) Ten times: a random club and two random
/// squad slots (50); different people and random(the first one's +0x5b) = 0: if the first dislikes the second, he stops
/// disliking him (`staff_remove_disliked_staff`) and, at the same contract club and with the link flagged in +0x45 & 0x800, the fight
/// (`exe::news_post_feud_settled`); otherwise, 1/3 start disliking (`exe::staff_add_disliked_staff`), 1/3 of the rest start liking (`staff_add_favoured_staff`) and the rest stop
/// liking (`staff_remove_favoured_staff`). (2) Day a multiple of 50: `[0xae235c]`/100 random clubs go through the dressing room friction
/// (`game_check_club_dressing_room_unrest`). On the other days, each person (except the last 17) with a player record and preferences, a contract and
/// the club's link with the +0xff person: three random draws (+0x5b, +0x5d, +0x5a, each + 5) filter; the sum of five
/// attributes minus 3, 5, 15, 15 by draws that came out zero and 5 if the player's reputation (+0xb) exceeds 4500 and
/// the club's (+0x80) gives the draw that, when zero, leads to friction with the manager.
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
    for _ in 0..10 {
        let club = club_record(friendly_random(read_usize(G_CLUB_COUNT) as i32) as usize);
        let a = Staff(read_usize(club.wrapping_add(friendly_random(0x32) as usize * 4) + club::SQUAD));
        let b = read_usize(club.wrapping_add(friendly_random(0x32) as usize * 4) + club::SQUAD);
        if a.is_null() || b == 0 || a.addr() == b {
            continue;
        }
        if friendly_random(a.professionalism() as i32) != 0 {
            continue;
        }
        if is_disliked_by(b, a) != 0 {
            exe::staff_remove_disliked_staff(a, b);
            let c = game_get_contract_and_store(a.addr());
            if c == 0 || game_club_of_contract(c) == Club(0) {
                continue;
            }
            let ca = game_club_of_contract(game_get_contract_and_store(a.addr()));
            let cb = game_club_of_contract(game_get_contract_and_store(b));
            if ca != cb {
                continue;
            }
            let k = game_contract_for_club(a.addr(), Club((game_club_of_contract(game_get_contract_and_store(a.addr()))).addr()));
            if k.is_null() || (k.grievances() as usize) & 0x800 == 0 {
                continue;
            }
            exe::news_post_feud_settled(a, b);
        } else if friendly_random(3) == 0 {
            exe::staff_add_disliked_staff(a, b, 1);
        } else if friendly_random(3) == 0 {
            exe::staff_add_favoured_staff(a, b, 1);
        } else {
            exe::staff_remove_favoured_staff(a, b);
        }
    }
    if (read_i16(G_CURRENT_DATE) as i32) % 0x32 == 0 {
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 / 100 {
            exe::game_check_club_dressing_room_unrest(club_record(friendly_random(read_usize(G_CLUB_COUNT) as i32) as usize));
            i += 1;
        }
        return;
    }
    let base = read_usize(G_STAFF_TABLE);
    let end = base.wrapping_add((read_usize(G_STAFF_COUNT).wrapping_sub(17)).wrapping_mul(staff::SIZE));
    let mut p = base;
    while p < end {
        player_vs_manager_check(Staff(p));
        p += 0x6e;
    }
}

/// One person of the non-50th-day loop: rolls the player's attitude toward the club's manager and ends with
/// dressing_room_friction [inference].
///
/// [pt] Uma pessoa do laço de fora dos dias múltiplos de 50 (0x5d6f15..0x5d7115).
pub(crate) unsafe fn player_vs_manager_check(p: Staff) {
    if p.player().is_null() || p.staff_preferences() == 0 {
        return;
    }
    let c = game_get_contract_and_store(p.addr());
    if c == 0 {
        return;
    }
    let club = Club((game_club_of_contract(c)).addr());
    if club.is_null() {
        return;
    }
    // original defect, reproduced: the contract of squad slot 10, not the player's own (docs/DEFEITOS_DO_ORIGINAL.md)
    let k = game_contract_for_club(club.squad(10), club);
    if k.is_null() {
        return;
    }
    if friendly_random((p.professionalism() as i32) + 5) != 0 && friendly_random((p.temperament() as i32) + 5) != 0 && friendly_random((p.pressure() as i32) + 5) != 0 {
        return;
    }
    let mut e = (p.professionalism() as i32) + ((p.pressure() as i32) + (p.loyalty() as i32)) + ((p.sportsmanship() as i32) + ((p.age() as i8) as i32));
    crate::comparison::branch("cps_5 drawn manager");
    if friendly_random(p.loyalty() as i32) == 0 { // cps5: the first discount
        e -= 3;
    }
    if friendly_random(p.pressure() as i32) == 0 {
        e -= 5;
    }
    if (k.grievances() as usize) != 0 || friendly_random(0x64) == 0 {
        if friendly_random(p.professionalism() as i32) == 0 {
            e -= 0xf;
        }
        if friendly_random(p.temperament() as i32) == 0 {
            e -= 0xf;
        }
    }
    let rep = p.player().current_reputation();
    if rep > 0x1194 && rep > club.reputation() {
        e -= 5;
    }
    if friendly_random(e) >= 1 {
        return;
    }
    if p.staff_preferences() == 0 {
        return;
    }
    dressing_room_friction(p, club, k.addr());
}

// simplification: rolls and touches the preferences (0x547a10, 0x547af0, 0x547c10, 0x547c50), quarrel and complaint: original only;
// the copy takes the branches (runs every day).

/// each squad player with a contract and preferences: the first team-mate he dislikes; with day % 14 + id = 0, the
/// complaint against the manager, the assistant or another (`exe::news_post_dressing_room_unrest`) and the flag on the contract (`exe::staff_contract_apply_event_flags`)
/// Original: `game_check_club_dressing_room_unrest` (0x5d7190, game.cpp, cdecl).
///
/// [pt] { 0x5d7190 } (cdecl, game.cpp): `game_check_club_dressing_room_unrest(clube)`: cada pessoa do elenco (+0xd7, 50
/// vagas) com vínculo com o clube (0x4cf5f0) e preferências (+0x65) passa pelo atrito com o técnico (o primeiro colega
/// de quem não gosta; a queixa só com dia % 14 + id = 0). Pelas relações do elenco (0x5d6d10) nos dias múltiplos de 50.
pub(crate) unsafe extern "C" fn game_check_club_dressing_room_unrest(club: usize) {
    let club = Club(club);
    for i in 0..0x32usize {
        let p = Staff(club.squad(i));
        if p.is_null() {
            continue;
        }
        let k = game_contract_for_club(p.addr(), club);
        if k.is_null() {
            continue;
        }
        if p.staff_preferences() == 0 {
            continue;
        }
        dressing_room_friction(p, club, k.addr());
    }
}

// simplification: the complaint (0x587c60) and the mark on the link (0x867700) do not run twice: original only; the copy carries the
// friction branch.

/// True if the person has a valid id, belongs to the club (+0x39) and has a player record.
///
/// [pt] A pessoa conta para a liderança do clube: id válido (< [0xae2364]), contrato com o clube (+0x39) e ficha de jogador.
pub(crate) unsafe fn is_club_squad_player(p: Staff, club: usize) -> bool {
    !p.is_null() && (read_usize(p.addr()) as i32) < read_usize(G_STAFF_COUNT) as i32 && p.club_job() == Club(club) && !p.player().is_null()
}

/// Spreads morale through the squads by leadership: one club in seven each day (not national teams): the squad's
/// leadership and leadership × morale are summed; each player draws from them and his own morale, and his morale goes
/// up or down by one (`staff_adjust_morale`).
///
/// `game_spread_squad_morale_by_leadership()`, every day from the game's day loop. Each club that is not a national
/// team (`exe::database_is_national_team`), with random(7) = 0: sums the leadership (player +0x2f) and leadership × morale of the link (+0x44)
/// over the squad (+0xd7, 50 slots); fewer than 16 players count 10 for each one missing. With a non-zero sum, each
/// player draws (2 × Σ leadership × morale) / (his leadership + sum) plus random(morale / 12), as a signed byte:
/// negative, random(s) < random(−5) lowers the morale (`staff_adjust_morale` with −1); positive, random(s) > random(5) raises it
/// (+1).
/// Original: `game_spread_squad_morale_by_leadership` (0x5ce0b0, game.cpp, cdecl).
///
/// [pt] { 0x5ce0b0 } (cdecl, game.cpp): `game_spread_squad_morale_by_leadership()`, todo dia pelo laço do dia do jogo. Cada
/// clube que não é seleção (0x53b900), com sorteio(7) = 0: soma a liderança (jogador +0x2f) e liderança × moral do
/// vínculo (+0x44) do elenco (+0xd7, 50 vagas); menos de 16 jogadores contam 10 cada um que falta. Com soma não nula,
/// cada jogador sorteia (2 × Σ liderança × moral) / (liderança dele + soma) mais sorteio(moral / 12), em byte com sinal:
/// negativo, sorteio(s) < sorteio(−5) baixa o moral (0x544710 com −1); positivo, sorteio(s) > sorteio(5) sobe (+1).
pub(crate) unsafe extern "C" fn game_spread_squad_morale_by_leadership() {
    let morale: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let n = read_usize(G_CLUB_COUNT);
    let mut club = read_usize(G_CLUB_TABLE);
    let end = club.wrapping_add(n.wrapping_mul(club::SIZE));
    while club < end {
        let slots = club + club::SQUAD;
        if exe::database_is_national_team(club) == 0 && friendly_random(7) == 0 {
            let (mut soma, mut lideranca, mut q) = (0i32, 0i32, 0i32);
            for i in 0..0x32usize {
                let p = read_usize(slots + i * 4);
                if !is_club_squad_player(Staff(p), club) {
                    continue;
                }
                let c = game_contract_for_club(p, Club(club));
                if c.is_null() {
                    continue;
                }
                let l = read_i8_as_i32(read_usize(p + staff::PLAYER) + player_sheet::LEADERSHIP);
                soma = soma.wrapping_add((c.morale() as i32) * l);
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
                if !is_club_squad_player(Staff(p), club) {
                    continue;
                }
                let c = game_contract_for_club(p, Club(club));
                let m = if c.is_null() { 0 } else { c.morale() as i32  };
                let l = read_i8_as_i32(read_usize(p + staff::PLAYER) + player_sheet::LEADERSHIP);
                crate::comparison::branch("cps_5 drawn morale");
                let r1 = friendly_random(soma.wrapping_mul(2) / (l + lideranca));
                let r2 = friendly_random(m / 12); // cps5: the morale part
                let s = (r1 as u8).wrapping_add(r2 as u8) as i8;
                if s < 0 {
                    let e = friendly_random(s as i32);
                    if e < friendly_random(-5) {
                        morale(p, -1, 1);
                    }
                } else if s > 0 {
                    let e = friendly_random(s as i32);
                    if e > friendly_random(5) {
                        crate::comparison::branch("cps_5 morale rises");
                        morale(p, 1, 1); // cps5: sobe o moral
                    }
                }
            }
        }
        club = club.wrapping_add(club::SIZE);
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
pub(crate) unsafe fn days_since_staff_date(p: Staff) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32)
}

/// Checks players missing training: n/250 consecutive people from a random one; a present player with training and
/// without a serious injury misses training (`miss_training`) by a random draw on the sum of attributes, morale and liking of
/// the manager (more likely when newly arrived) or by the contract flags.
///
/// `game_check_players_missing_training()`, every day from the game's day loop. Starting from a random person (random(n
/// − n/250 − 1)), n/250 consecutive people: a player (+0x61, function +0x3d = 11) with a contract and a club, present
/// (`is_player_awol`), with the training record (`get_player_schedule`, +0x15) and without a serious injury (`get_player_injury_status` ≠ 2). (1) With +0x5b <
/// 8, player +0x2f < 8 and the sum (player +0x2f, +0x18, +0x5b, link +0x44, −5 if +0x5b or the morale are 1, −5 if he
/// dislikes the manager, +2 if he likes him) < 43: random(+0x5b + 100, 50 or 25 depending on the days since +0x3e: <
/// 30, < 90, < 180) coming out zero is a miss. (2) Otherwise, with +0x5b < 10 and a link with the club: the link flags
/// (+0x45: 0x8000, 0x30000; with +0x5b < 5, 0x7800) give random(+0x5b) draws that, when zero, are a miss. The miss:
/// `miss_training`.
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
    let n = read_usize(G_STAFF_COUNT) as i32;
    let q = n / 250;
    crate::comparison::branch("cps_6 absences of the day");
    let r = friendly_random(n - q - 1); // cps6: the first person
    let mut current = read_usize(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(staff::SIZE));
    for _ in 0..q {
        let p = Staff(current);
        current = current.wrapping_add(0x6e);
        if p.player().is_null() || p.job_for_club() != job_kind::PLAYER {
            continue;
        }
        let c = game_get_contract_and_store(p.addr());
        if c == 0 {
            continue;
        }
        let club = contract_club_record(c);
        if club == 0 {
            continue;
        }
        if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
            continue;
        }
        let t = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
        if t == 0 || read_u8(t + training_schedule::TYPE) == 0 {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            continue;
        }
        let jog = p.player();
        let cl = p.professionalism() as u8;
        let dl = jog.work_rate() as u8;
        let mut bl = (jog.leadership() as u8).wrapping_add(p.age() as u8).wrapping_add(cl).wrapping_add(dl);
        if cl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if dl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if exe::staff_is_disliked_by(read_usize(club + club::MANAGER), p) != 0 {
            bl = bl.wrapping_add(0xfb);
        }
        if exe::staff_is_favoured_by(read_usize(club + club::MANAGER), p) != 0 {
            bl = bl.wrapping_add(2);
        }
        let mut and_foul = false;
        let a = p.professionalism() as u8;
        if (a as i8) < 8 && ((p.player().leadership() as u8) as i8) < 8 && (bl as i8) < 0x2b {
            let mut s = a;
            if days_since_staff_date(p) < 0x1e {
                s = s.wrapping_add(0x64);
            } else if days_since_staff_date(p) < 0x5a {
                s = s.wrapping_add(0x32);
            } else if days_since_staff_date(p) < 0xb4 {
                s = s.wrapping_add(0x19);
            }
            crate::comparison::branch("cps_6 absence by sum");
            if friendly_random(s as i8 as i32) == 0 { // cps6: the random draw of the sum
                and_foul = true;
            }
        }
        if !and_foul {
            if (p.professionalism() as i32) >= 0xa {
                continue;
            }
            let club = contract_club_record(game_get_contract_and_store(p.addr()));
            let v = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            if v == 0 {
                continue;
            }
            if read_usize(v + contract::GRIEVANCES) & (grievances::UNHAPPY_WITH_TRAINING as usize) != 0 && friendly_random(p.professionalism() as i32) == 0 {
                and_foul = true;
            } else if read_usize(v + contract::GRIEVANCES) & 0x30000 != 0 && friendly_random(p.professionalism() as i32) == 0 {
                and_foul = true;
            } else {
                let a = p.professionalism() as i32;
                if a >= 5 || read_usize(v + contract::GRIEVANCES) & 0x7800 == 0 || friendly_random(a) != 0 {
                    continue;
                }
                and_foul = true;
            }
        }
        if and_foul {
            crate::comparison::branch("cps_6 missed training");
            exe::awol_miss_training(read_usize(G_AWOL), p); // cps6: a falta
        }
    }
}

// simplification: draws and the foul (0x41a360) fines, draws and changes the form: original only; the copy carries the branches.

/// Gives the day to each competition in the list (virtual +0x14 with the game date and the flag); with the flag, the
/// next step of `[0xae2a58]` (`fix_man_daily_update`).
///
/// `game_update_comps_for_date(flag)`: each competition in the list (`[0xadadfc]`, count `[0xadadf8]` re-read on each
/// iteration) receives the day (virtual +0x14 with the game date 0xae2c90 and the flag); with the flag, the next step
/// of `[0xae2a58]` (`fix_man_daily_update`, which the original calls as a tail jump). Called from the game's day loop (with 1).
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
            let day: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(read_usize(read_usize(c) + comp_vtable::UPDATE_FOR_DATE));
            day(c, G_CURRENT_DATE, m as i32);
        }
        i += 1;
    }
    if m != 0 {
        crate::comparison::branch("cps_6 competitions of the day");
        exe::fixture_manager_daily_update(read_usize(G_FIXTURE_MANAGER));
    }
}

// simplification: advances the day of all competitions (virtual) and of [0xae2a58]: original only; the copy takes the branch.

/// Address of the club's board confidence word (+6 of its 73-byte row in [this]).
///
/// [pt] A confiança da diretoria no clube: a palavra +6 da linha do clube (73 bytes por id em [this]).
pub(crate) unsafe fn board_confidence_ptr(this: usize, club: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(club).wrapping_mul(0x49)) + 6
}

/// Adds the delta to the board's confidence in the club (+6 of the 73-byte club row), held to 1..10000.
///
/// `manager_manager_adjust_board_confidence(club, delta)`: adds `delta` (word) to the board's confidence in the club
/// (+6 of the MANAGER_MANAGER row `[0xb67a44]`), clamped to 1..10000. Null club: nothing. Called from the game's day
/// loop (with −5).
/// Original: `manager_manager_adjust_board_confidence` (0x5cdde0, game.cpp, thiscall, ret 8).
///
/// [pt] { 0x5cdde0 } (thiscall, ret 8, game.cpp): `manager_manager_adjust_board_confidence(clube, delta)`: soma `delta`
/// (palavra) à confiança da diretoria no clube (+6 da linha do MANAGER_MANAGER [0xb67a44]), presa em 1..10000. Clube
/// nulo: nada. Pelo laço do dia do jogo (com −5).
pub(crate) unsafe extern "thiscall" fn manager_manager_adjust_board_confidence(this: usize, club: usize, delta: u32) {
    if club == 0 {
        return;
    }
    let c = board_confidence_ptr(this, club);
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
    let manager = TransferManagerFields(manager);
    manager.shortlist_manager()
}

/// the player's 6-byte entry in the training table
/// Original: `training_manager_get_player_entry` (0x5d5ac0, game.cpp, thiscall).
///
/// [pt] { 0x5d5ac0 } (thiscall, ret 4, game.cpp): a entrada de 6 bytes do jogador na tabela do treino
/// ([treino+0x651d]), pelo id dos dados do jogador ([[jogador+0x61]]).
pub(crate) unsafe extern "thiscall" fn training_manager_get_player_entry(training: usize, player: usize) -> usize {
    let player = Staff(player);
    let table = read_usize(training + training_manager_fields::PLAYER_ENTRIES);
    let id = read_i32(player.player().addr());
    (table as isize + id as isize * 6) as usize
}

/// Pointer to the training load byte (+0x1e) of a player's 31-byte condition record.
///
/// [pt] O byte da carga de treino do jogador `*id` no registro de condição (31 bytes por jogador, +0x1e).
pub(crate) unsafe fn training_load_byte(injuries: usize, id: *const i32) -> *mut u8 {
    (read_usize(injuries) as isize + *id as isize * 31 + (physical_record::PRONENESS as isize)) as *mut u8
}

/// the f64 average +0x20/+0x28 or +0x10/+0x18 (zero divisor: 6.8)
/// Original: `game_average` (0x5d5a60, game.cpp, thiscall).
///
/// [pt] { 0x5d5a60 } (thiscall, ret 4, game.cpp): a média +0x20/+0x28 (com `segunda`) ou +0x10/+0x18 (f64,
/// como a original deixa no x87); com divisor zero (ou não comparável), 6.8.
pub(crate) unsafe extern "thiscall" fn game_average(t: usize, second: u8) -> f64 {
    let (n, q) = if second != 0 { (read_f64(t + player_stats_fields::FLAGGED_RATING_SUM), read_f64(t + player_stats_fields::FLAGGED_RATED_COUNT)) } else { (read_f64(t + player_stats_fields::RATING_SUM), read_f64(t + player_stats_fields::RATED_COUNT)) };
    if q == read_f64(rdata::F64_0) || q.is_nan() {
        return read_f64(rdata::F64_6_8);
    }
    n / q
}

compare_value!(compare_contract_value, contract_value, IDX_CONTRACT_VALUE, (p: *const i32) -> u32);

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
    if (read_i16(pa + 5) as i32) == (read_i16(pb + 5) as i32) {
        return (read_i16(pa + 7) as i32) - (read_i16(pb + 7) as i32);
    }
    (read_i16(pa + 5) as i32) - (read_i16(pb + 5) as i32)
}

