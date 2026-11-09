use super::*;

/// Adds v to the month and year expense accumulators of a finance record and subtracts it from the i64 balance.
///
/// [pt] Soma `v` nos dois acumuladores do ano e do mês (`ano`, `mes`) e o tira do saldo (i64; o cdq/sub/sbb da original).
pub(crate) unsafe fn charge_expense(fin: usize, month: usize, year: usize, v: i32) {
    add_to_i32(fin + month, v);
    add_to_i32(fin + year, v);
    core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_sub(v as i64));
}

/// Calls FN_HUMAN_MANAGER_IS_HUMAN_CLUB (`exe::human_manager_is_human_club`) with (club, 0, 1, 0, 0).
///
/// [pt] O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
pub(crate) unsafe fn fin_prm_is_human_club(club: Club) -> u32 {
    exe::human_manager_is_human_club(club, 0, 1, 0, 0)
}

/// Finance record (0x167 bytes) of the club, using the linked parent when the club is a B team (`exe::club_get_linked_b_team`); writes the
/// original's local flag.
///
/// [pt] O registro de finanças do clube (0x167 bytes) pelo parceiro: 0x540a50 (3 pushes; add esp, 0xc -> cdecl(clube, &a,
/// 1)); o parceiro quando ele existe e o clube é o B (a = 0). `a` é a local da original (a 0x540a50 a grava).
pub(crate) unsafe fn finance_of_parent_club(this: usize, club: Club, a: &mut u32) -> usize {
    let p = exe::club_get_linked_b_team(club, a, 1);
    let c = if p != 0 && *a == 0 { p } else { club.addr() };
    read_usize(this).wrapping_add((read_i32(c) as u32).wrapping_mul(0x167) as usize)
}

/// Posts the stadium works / training news (FN_FINANCE_POST_STADIUM_WORKS_NEWS) with club, value and type.
///
/// [pt] A notícia das obras e do treino (0x59ffb0: ecx = tabela; 3 pushes (tipo, valor, clube); sem add esp -> thiscall,
/// ret 0xc).
pub(crate) unsafe fn post_works_news(this: usize, club: Club, value: i32, kind: i32) {
    exe::finance_post_stadium_works_news(this, club, value, kind);
}

/// Posts the refused/cancelled works news (FN_FINANCE_POST_WORKS_CANCELLED_NEWS, may draw random numbers).
///
/// [pt] A notícia da obra recusada (0x59fae0: ecx = tabela; 3 pushes; sem add esp -> thiscall, ret 0xc; sorteia 2 fora do
/// motivo 2).
pub(crate) unsafe fn post_works_refused_news(this: usize, club: Club, subject: i32, reason: i32) {
    exe::finance_post_works_cancelled_news(this, club, subject, reason);
}

/// Club's row in the MANAGER_MANAGER board table ([[G_MANAGER_MANAGER]] + id × 0x49); byte +2 holds the board requests
/// (bit 6 stadium, bit 7 training ground).
///
/// [pt] A linha do clube na tabela dos técnicos ([[0xb67a44]] + id × 0x49); o byte +2 tem os pedidos à diretoria (bit 6:
/// estádio; bit 7: centro de treino).
pub(crate) unsafe fn board_requests_row(club: Club) -> usize {
    read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(club.addr()) as u32).wrapping_mul(0x49) as usize)
}

/// True if the club's manager (club::MANAGER) is one of the 16 human managers.
///
/// [pt] O técnico do clube (+0xcf) é humano (entre as 16 últimas pessoas): a notícia vai para ele (0x7861e0).
pub(crate) unsafe fn fin_prm_has_human_manager(club: Club) -> bool {
    let t = club.manager().addr();
    t != 0 && read_i32(t) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10)
}

/// Ends the financial season for the clubs of a key nation: after the game start date, for each club with a league and
/// an active nation or reputation 5000 whose row is the given key nation, the club's season end runs, and the season
/// totals (+0x110) move to last season (+0xc0) and are cleared; stops at the first national team club.
///
/// The asserts cite finance.CPP. `FinanceTable::end_season_for_nation(entry)`, called from the season loop (0x5d5ae0,
/// on the day before the season start of the key nation `entry` of table 0xb63d60, 72 bytes each). Nothing before the
/// game start date (`[0xdd6ec8]` after today, `exe::date_operator_3c`: ecx = 0xae2c90; 2 pushes; no add esp -> thiscall, ret 8).
/// Without an entry: the error box (the original). For each club of the table (0x245 bytes; it stops at the first one
/// that is a national team, `exe::database_is_national_team`: push; add esp, 4 -> cdecl) with a league (+0x57) and an active (+0x11c) nation
/// (+0x53) or a reputation (+0x80) of 5000, whose row (`key_nation_index_of_club`: push; add esp, 4 -> cdecl) is the entry: the club's
/// season end (`season_end_club_update`: ecx = table; push club -> thiscall, ret 4) and the year's totals (+0x110, 0x50 bytes) move
/// into the previous year's (+0xc0) and are cleared, in the record whose index equals the club's.
/// Original: `FinanceTable::end_season_for_nation` (0x59cdd0, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x59cdd0 } (thiscall, ret 4, fin_prm.cpp; os asserts citam finance.CPP): `FinanceTable::end_season_for_nation(entrada)`,
/// pelo laço das temporadas (0x5d5ae0, no dia anterior ao início da temporada da nação-chave `entrada` da tabela
/// 0xb63d60, 72 bytes cada). Nada antes da data de início do jogo ([0xdd6ec8] depois de hoje, 0x456770: ecx = 0xae2c90;
/// 2 pushes; sem add esp -> thiscall, ret 8). Sem entrada: a caixa (a original). Para cada clube da tabela (0x245
/// bytes; para no primeiro que for seleção, 0x53b900: push; add esp, 4 -> cdecl) com liga (+0x57) e nação (+0x53) ativa
/// (+0x11c) ou reputação (+0x80) de 5000, cuja linha (0x669360: push; add esp, 4 -> cdecl) é a entrada: o fim de
/// temporada do clube (0x59cfa0: ecx = tabela; push clube -> thiscall, ret 4) e os totais do ano (+0x110, 0x50 bytes)
/// passam para os do ano anterior (+0xc0) e zeram, no registro de índice igual ao do clube.
pub(crate) unsafe extern "thiscall" fn fin_prm_end_season_for_nation(this: usize, entry: usize) {
    if exe::date_operator_3c(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return;
    }
    if entry == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIN_PRM_END_SEASON_FOR_NATION)(this, entry); // a caixa
    }
    let row: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut k = 0i32;
    let mut desloc = 0usize;
    while k < read_i32(G_CLUB_COUNT) {
        if exe::database_is_national_team(club) != 0 {
            return;
        }
        let nation = read_usize(club + club::NATION);
        if read_usize(club + club::DIVISION) != 0 && nation != 0 {
            let active = read_u8(nation + nation::LEAGUE_SELECTED) != 0;
            if active || read_i16(club + club::REPUTATION) >= 0x1388 { // cqc1: the club of an inactive nation by reputation
                let l = row(club) as i8 as i32;
                if (0xb63d60 + l * 72) as usize == entry {
                    if !active && read_i16(club + club::REPUTATION) < 0x1770 {
                        crate::comparison::branch("cqc_1 inactive nation below 6000");
                    }
                    exe::financetable_season_end_club_update(this, club);
                    let f = read_usize(this).wrapping_add(desloc);
                    core::ptr::copy_nonoverlapping((f + 0x110) as *const u8, (f + 0xc0) as *mut u8, 0x50); // cqc1: the year totals move to the previous year
                    core::ptr::write_bytes((f + 0x110) as *mut u8, 0, 0x50);
                }
            }
        }
        k += 1;
        desloc += 0x167;
        club += 0x245;
    }
}

/// The club's financial season end: ground purchase (20 million, from an owner in crisis), the squeeze, the AI club
/// training facilities by random draw, undersoil heating (1.5 million) and roof (7.5 million), expansion or seating by
/// balance and average attendance, the human's board requests (stadium and training) and the dividend by reputation,
/// balance and chairman, with a random draw.
///
/// `FinanceTable::season_end_club_update(club)`, called from the nation's season end (`fin_prm_end_season_for_nation`). National team:
/// nothing; no club: the error box (the original). The finances through the partner (`exe::club_get_linked_b_team`) and the balance (the low
/// word, i32). With a stadium (+0x69):
///
/// - not the owner (+0x6d = 0), with a league (+0x57) and a balance above 35 million: buys the stadium for 20 million
/// (+0x8c, +0x12c, balance; +0x6d = 1; news 4), alone or from the owner (another club with the same stadium and +0x6d),
/// if its finances are in crisis (+0x165 = 1 or 2): the owner stops being one (+0x6d = 0) and receives the 20 million
/// (+0xb4, +0x154, balance; news 5);
///
/// - owner and with a league: the squeeze (+0x166 above 10, balance above 750 thousand) charges 10 − squeeze × 10000;
/// the training facilities (+0x7f below 20) of an AI club with a manager (+0xcf, +0x69 with +0x22) go up by random(40),
/// charged to the club's own finances (level × 50000 + 75000, then up to 20 or one more level by reputation and
/// balance); the human's stadium request (bit 6 of +2 of the manager row) becomes the notice; the undersoil heating
/// (stadium +0x4d, 1.5 million, news 6) and the roof (+0x4c, 7.5 million); the expansion (by balance × 0.2, or × 0.1
/// when +0x64 = 1, up to the ceiling +0x44 and a third of it; news 1 and 2) when the average attendance (`exe::comp_stats_find_club_entry` from
/// the league table: ecx = `[comp + 8]`; push club; no add esp -> thiscall) exceeds 0.92 of the capacity (+0x3c), by
/// reputation or by the notice; otherwise the seating (+0x40) by random(20) and the league size (+0x69), by balance ×
/// 0.2 / 75 (news 3); the ceiling goes up by 1500 below 30 thousand places; the refusal (`news_request_refused`) to the human who
/// asked.
///
/// Then, the human's training facilities request (bit 7): above (500 − reputation) × 500, the upgrade (`upgrade_training_facilities`: ecx =
/// table; push club -> thiscall, ret 4; news 8), otherwise the refusal; and the dividend (+0x94, +0x134, balance; news
/// `news_dividend`: ecx = table; 2 pushes -> thiscall, ret 8) by reputation, balance and chairman (+0xbf), with random(501) in
/// the two larger branches.
/// Original: `FinanceTable::season_end_club_update` (0x59cfa0, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x59cfa0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::season_end_club_update(clube)`, pelo fim de temporada da
/// nação (0x59cdd0). Seleção: nada; sem clube: a caixa (a original). As finanças pelo parceiro (0x540a50) e o saldo
/// (a palavra baixa, i32). Com estádio (+0x69):
/// - sem ser dono (+0x6d = 0), com liga (+0x57) e saldo acima de 35 milhões: compra o estádio por 20 milhões (+0x8c,
///   +0x12c, saldo; +0x6d = 1; notícia 4), sozinho ou do dono (outro clube com o mesmo estádio e +0x6d), se as finanças
///   dele estão em crise (+0x165 = 1 ou 2): o dono deixa de ser (+0x6d = 0) e recebe os 20 milhões (+0xb4, +0x154,
///   saldo; notícia 5);
/// - dono e com liga: o aperto (+0x166 acima de 10, saldo acima de 750 mil) cobra 10 − aperto × 10000; o centro de
///   treino (+0x7f abaixo de 20) do clube da máquina com técnico (+0xcf, +0x69 com o +0x22) sobe pelo sorteio(40),
///   cobrado nas finanças do próprio clube (nível × 50000 + 75000, depois até 20 ou mais um nível pela reputação e
///   pelo saldo); o pedido de estádio do humano (bit 6 do +2 da linha dos técnicos) vira o aviso; o aquecimento (+0x4d
///   do estádio, 1,5 milhão, notícia 6) e o coberto (+0x4c, 7,5 milhões); a expansão (por saldo × 0,2 ou × 0,1 no
///   +0x64 = 1, até o teto +0x44 e um terço dele; notícias 1 e 2) quando o público médio (0x4a4850 da tabela da liga:
///   ecx = [comp + 8]; push clube; sem add esp -> thiscall) passa de 0,92 da capacidade (+0x3c), pela reputação ou
///   pelo aviso; senão os lugares sentados (+0x40) pelo sorteio(20) e o tamanho da liga (+0x69), por saldo × 0,2 / 75
///   (notícia 3); o teto sobe 1500 abaixo de 30 mil lugares; a recusa (0x59fae0) ao humano que pediu.
/// Depois, o pedido de centro de treino do humano (bit 7): acima de (500 − reputação) × 500, a melhora (0x59b630: ecx =
/// tabela; push clube -> thiscall, ret 4; notícia 8), senão a recusa; e o dividendo (+0x94, +0x134, saldo; notícia
/// 0x59fe10: ecx = tabela; 2 pushes -> thiscall, ret 8) pela reputação, pelo saldo e pelo presidente (+0xbf), com
/// sorteio(501) nos dois ramos maiores.
pub(crate) unsafe extern "thiscall" fn season_end_club_update(this: usize, club_addr: usize) {
    if club_addr == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE)(this, club_addr); // a caixa
    }
    let club = Club(club_addr);
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let mut local = 0u32; // the original’s [esp + 0x20] (the owner’s 0x540a50 and the average attendance)
    let mut a = 0u32;
    let fin = Finance(finance_of_parent_club(this, club, &mut a));
    if fin.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE)(this, club.addr()); // a caixa
    }
    // the original reads only the low dword of the 64-bit balance
    let mut cash = fin.balance() as i32;
    let mut asked = false;
    let stadium = club.stadium();
    'works: {
        if stadium.is_null() {
            break 'works;
        }
        if club.own_stadium() == 0 && !club.division().is_null() {
            // a tenant club buys the ground from the club that owns it
            if cash <= 0x2160ec0 {
                break 'works;
            }
            let mut owner = Club(0);
            let mut other = Club(read_usize(G_CLUB_TABLE));
            let mut k = 0i32;
            while k < read_i32(G_CLUB_COUNT) {
                if other != club && other.stadium() == stadium && other.own_stadium() != 0 {
                    owner = other;
                    break;
                }
                k += 1;
                other = Club(other.addr() + club::SIZE);
            }
            let owner_fin = if !owner.is_null() { Finance(finance_of_parent_club(this, owner, &mut local)) } else { Finance(0) };
            if owner.is_null() || owner_fin.is_null() {
                club.set_own_stadium(1);
                charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, 20_000_000);
                post_works_news(this, club, read_i32(club.stadium().addr()), 4);
            } else if owner_fin.crisis_state() == 1 || owner_fin.crisis_state() == 2 {
                club.set_own_stadium(1);
                owner.set_own_stadium(0);
                charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, 20_000_000);
                owner_fin.set_month_other_income(owner_fin.month_other_income().wrapping_add(20_000_000));
                owner_fin.set_season_other_income(owner_fin.season_other_income().wrapping_add(20_000_000));
                owner_fin.set_balance(owner_fin.balance().wrapping_add(20_000_000));
                post_works_news(this, club, read_i32(club.stadium().addr()), 4);
                post_works_news(this, owner, read_i32(club.stadium().addr()), 5);
            }
            break 'works;
        }
        if club.own_stadium() == 0 || club.division().is_null() {
            break 'works;
        }
        let squeeze = fin.squeeze_counter() as i8;
        if cash > 0xb71b0 && squeeze > 10 {
            let v = 10i32.wrapping_sub((squeeze as i32).wrapping_mul(10000));
            fin.set_season_ground_costs(fin.season_ground_costs().wrapping_add(v));
            cash = cash.wrapping_sub(v);
            fin.set_month_ground_costs(fin.month_ground_costs().wrapping_add(v));
            fin.set_balance(fin.balance().wrapping_sub(v as i64));
        }
        upgrade_ai_training_ground(this, club, cash);
        let requests = board_requests_row(club) + 2;
        if read_u8(requests) & 0x40 != 0 && fin_prm_is_human_club(club) != 0 {
            asked = true;
            *(requests as *mut u8) &= 0xbf;
        }
        let ground = club.stadium();
        if !ground.is_null() {
            if cash > 0x10b0760 && club.reputation() > 0x1d4c && ground.capacity() > 0x61a8 && ground.under_soil_heating() == 0 {
                ground.set_under_soil_heating(1);
                charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, 1_500_000);
                post_works_news(this, club, read_i32(club.stadium().addr()), 6);
                break 'works;
            }
            if cash > 0x47868c0 && club.reputation() > 0x2422 && ground.capacity() > 0xe09c && club.training() as u8 == 0x14 && ground.covered() == 0 {
                ground.set_covered(1);
                charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, 7_500_000);
                break 'works;
            }
        }
        let can = cash > 0x501bd0 || (fin_prm_is_human_club(club) != 0 && cash.wrapping_add((club.reputation() as i32).wrapping_mul(1000)) > 0x501bd0);
        if !can {
            if asked {
                post_works_refused_news(this, club, 2, -1);
            }
            break 'works;
        }
        let league = club.division();
        if !league.is_null() {
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(league.addr()).wrapping_mul(4)));
            if comp != 0 && read_usize(comp + 8) != 0 {
                let l = exe::comp_stats_find_club_entry(read_usize(comp + 8), club);
                if l != 0 && read_u8(l + 5) != 0 {
                    local = read_i32(l + 6).wrapping_div(read_u8(l + 5) as i32) as u32;
                }
            }
        }
        let ground = club.stadium();
        let cap = ground.capacity();
        if cap > 0x1e848 {
            break 'works;
        }
        let rep = club.reputation();
        let expand = asked
            || (cap as f64 * read_f64(rdata::F64_0_92)) < (local as i16 as i32) as f64
            || (rep.wrapping_mul(2) as i32 > cap && rep > 0x109a);
        if !expand {
            if friendly_random(0x14).wrapping_add(5) >= club.division().reputation() as i32 {
                break 'works;
            }
            let ground = club.stadium();
            let free = ground.capacity().wrapping_sub(ground.seating_capacity());
            if free <= 0 {
                break 'works;
            }
            let t = ftol(cash as f64 * read_f64(rdata::F64_0_2));
            let mut v = ((t / 75) as i16 as i32 / 500) as i16 as i32 * 500;
            if v >= free {
                v = free;
            }
            if (v as i16) < 0x1f4 {
                if ground.capacity() >= 0x7530 {
                    break 'works;
                }
                ground.set_expansion_capacity(ground.expansion_capacity().wrapping_add(0x5dc));
                post_works_refused_news(this, club, 2, 2);
                break 'works;
            }
            let v = v as i16 as i32;
            ground.set_capacity(ground.capacity().wrapping_add(v));
            let s = club.stadium();
            s.set_seating_capacity(s.seating_capacity().wrapping_add(v));
            crate::comparison::branch("cqc_1 seats");
            charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(75))); // cqc1: os lugares sentados
            post_works_news(this, club, v, 3);
            break 'works;
        }
        let amateur = club.professional_status() == 1;
        let base = if cash > 0x1312d00 {
            if amateur { 16000 } else { 21000 }
        } else if amateur {
            ftol(cash as f64 * read_f64(rdata::F64_0_1)) / 125
        } else {
            ftol(cash as f64 * read_f64(rdata::F64_0_2)) / 50
        };
        let lim = (base as i16 as i32) / 500 * 500;
        let ceiling = ground.expansion_capacity();
        let mut v = lim;
        if (lim as i16 as i32).wrapping_add(cap) > ceiling {
            let dif = ceiling.wrapping_sub(cap);
            v = if dif < 0 { 0 } else { dif };
        }
        let mut third = ceiling / 3;
        if third < 0x1388 {
            third = 0x1388;
        }
        let mut n = v as i16 as i32;
        if n >= third {
            n = third;
        }
        if (n as i16) < 0x3e8 {
            if cap < 0x7530 {
                ground.set_expansion_capacity(ground.expansion_capacity().wrapping_add(0x5dc));
            }
            if asked {
                post_works_refused_news(this, club, 2, 2);
            }
            break 'works;
        }
        let n = n as i16 as i32;
        ground.set_capacity(n.wrapping_add(cap));
        if amateur {
            let s = club.stadium();
            s.set_seating_capacity(s.seating_capacity().wrapping_add(n));
            charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, (n / 1000 + 1).wrapping_mul(6000).wrapping_add(n).wrapping_mul(125));
            post_works_news(this, club, n, 1);
        } else {
            charge_expense(fin.addr(), finance::MONTH_GROUND_COSTS, finance::SEASON_GROUND_COSTS, n.wrapping_add((n / 1000 + 1).wrapping_mul(1000)).wrapping_mul(50));
            post_works_news(this, club, n, 2);
        }
    }
    let requests = board_requests_row(club) + 2;
    if read_u8(requests) & 0x80 != 0 {
        if cash > (500 - club.reputation() as i32).wrapping_mul(500) {
            exe::financetable_upgrade_training_facilities(this, club);
            post_works_news(this, club, -1, 8);
        } else {
            post_works_refused_news(this, club, 8, -1);
        }
        *((board_requests_row(club) + 2) as *mut u8) &= 0x7f;
    }
    pay_dividend(this, club, fin.addr());
}

/// End-of-season training ground upgrade for AI clubs (part of `season_end_club_update`): raises club::TRAINING by chance and
/// reputation and charges the cost to the club's own finances.
///
/// [pt] O centro de treino do clube da máquina no fim da temporada (o trecho 0x59d32a..0x59d54d da 0x59cfa0), cobrado nas
/// finanças do próprio clube ([tabela] + id × 0x167, sem o parceiro).
pub(crate) unsafe fn upgrade_ai_training_ground(this: usize, club: Club, goal_difference: i32) {
    let manager = Staff(club.manager().addr());
    if manager.is_null() || club.reputation() == 0 || (club.training() as u8) as i8 >= 0x14 || manager.non_player() == 0 {
        return;
    }
    if fin_prm_is_human_club(club) != 0 {
        return;
    }
    let q = ((club.reputation() as i32) / 500) as i8 as i32 + 8;
    if ((club.training() as u8) as i8 as i32) <= q || goal_difference <= 0x225510 {
        return;
    }
    if friendly_random(0x28) >= read_u8(read_usize(club.manager().addr() + 0x69) + 0x22) as i8 as i32 {
        return;
    }
    let level = (club.addr() + club::TRAINING) as *mut i8;
    let own = || read_usize(this).wrapping_add((read_i32(club.addr()) as u32).wrapping_mul(0x167) as usize);
    let cl = *level;
    let cost = (cl as i32) * 50000 + 75000;
    if cl < 0x14 {
        *level = cl + 1;
    }
    charge_expense(own(), 0x8c, 0x12c, cost);
    let cl = *level;
    let rep = club.reputation();
    if cl < 0x14 && (rep > 0x203a || balance(own()) > 0xe4e1c0) {
        let cost = (4_500_000 - 200_000 * cl as i32).max(1_000_000);
        *level = 0x14;
        charge_expense(own(), 0x8c, 0x12c, cost);
        return;
    }
    if (cl < 0x10 && rep > 0x1482) || (cl < 0x14 && rep > 0x1c52) {
        let cost = (cl as i32) * 50000 + 75000;
        *level = cl + 1;
        charge_expense(own(), 0x8c, 0x12c, cost);
    }
}

/// End-of-season dividend (part of `season_end_club_update`) by the i64 balance, reputation, PLC flag and chairman, charged to the
/// finance record.
///
/// [pt] O dividendo do fim da temporada (o trecho 0x59db6f..0x59dd25 da 0x59cfa0), pelo saldo i64 das finanças.
pub(crate) unsafe fn pay_dividend(this: usize, club: Club, fin: usize) {
    let rep = club.reputation() as i32;
    let mut lim = rep.wrapping_mul(8000);
    if lim < 0x7a120 {
        lim = 0x7a120;
    }
    let s = balance(fin);
    let v;
    if s > lim as i64 && club.plc() != 0 {
        let r = friendly_random(0x1f5);
        if (rep as i16) < 0xfa0 {
            v = r.wrapping_add(0x5dc).wrapping_mul(rep).max(0xc350);
        } else {
            v = r.wrapping_add(0x7d0).wrapping_mul(rep);
        }
    } else {
        let mut s16 = rep as i16;
        let presence = Staff(club.chairman().addr());
        if !presence.is_null() && presence.non_player() != 0 {
            let e = (7i16.wrapping_sub((presence.loyalty() as u8) as i8 as i16)).wrapping_mul(3).wrapping_sub(read_u8(presence.non_player() + 0x20) as i8 as i16);
            s16 = (rep as i16).wrapping_add(e.wrapping_mul(5));
            if s16 < 0x1f4 {
                s16 = 0x1f4;
            }
        }
        let sv = s16 as i32;
        if s > sv.wrapping_mul(6000) as i64 {
            let r = friendly_random(0x1f5);
            if club.reputation() < 0xfa0 {
                crate::comparison::branch("cqc_1 small club dividend by chairman");
                v = r.wrapping_add(0x3e8).wrapping_mul(rep); // cqc1: the small club’s dividend by the chairman
            } else {
                v = r.wrapping_add(0x5dc).wrapping_mul(rep);
            }
        } else if s > sv.wrapping_mul(3000) as i64 {
            v = if club.reputation() < 0xfa0 { rep.wrapping_mul(500) } else { rep.wrapping_mul(750) };
        } else {
            return;
        }
    }
    crate::comparison::branch("cqc_1 dividend");
    add_to_i32(fin + finance::MONTH_DIVIDENDS, v);
    add_to_i32(fin + finance::SEASON_DIVIDENDS, v);
    core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_sub(v as i64)); // cqc1: the dividend leaves the balance
    exe::finance_post_dividend_news(this, club, v);
}

/// Sends news 0x177c with the club id, the amount and the +0x82 flag to the club's manager.
///
/// SEH frame only for the local news item, no destructor on the normal path. `FinanceTable::news_dividend(club,
/// amount)`, called from the club's season end (`season_end_club_update`). News 0x177c (`construct_news_item`: ecx = local; 2 pushes -> thiscall,
/// ret 8) with the club id, the amount and the +0x82 flag (`set_news_field`: ecx = local; 2 pushes -> thiscall, ret 8),
/// delivered to the club's manager (`send_to_club_manager`: ecx = `[0xb7431c]`; 2 pushes -> thiscall, ret 8).
/// Original: `FinanceTable::news_dividend` (0x59fe10, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59fe10 } (thiscall, ret 8, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_dividend(clube, valor)`, pelo fim de temporada do clube (0x59cfa0). A notícia 0x177c (0x77bbe0:
/// ecx = local; 2 pushes -> thiscall, ret 8) com o id do clube, o valor e a marca +0x82 (0x785790: ecx = local; 2
/// pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn news_dividend(_this: usize, club: usize, value: i32) {
    let club = Club(club);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x177c, 0);
    exe::newsitem_set_slot(p, 0, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(p, 1, value as u32);
    exe::newsitem_set_slot(p, 2, club.plc() as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Multiplies v by the exe double constant at k and truncates (_ftol).
///
/// [pt] `v` × a constante f64 em `k`, truncado (_ftol).
pub(crate) unsafe fn mul_exe_f64(v: i32, k: usize) -> i32 {
    ftol(v as f64 * read_f64(k))
}

/// Calls FN_HUMAN_MANAGER_IS_HUMAN_CLUB (`exe::human_manager_is_human_club`) with (club, a, 1, 0, 0) (passes argument a as the second parameter).
///
/// [pt] O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, a, 1, 0, 0)).
pub(crate) unsafe fn is_human_club_with(club: Club, a: u32) -> u32 {
    exe::human_manager_is_human_club(club, a, 1, 0, 0)
}

/// True if the club is a national team (`exe::database_is_national_team`).
///
/// [pt] A seleção (0x53b900: push; add esp, 4 -> cdecl).
pub(crate) unsafe fn fin_prm_is_national_team(club: usize) -> bool {
    exe::database_is_national_team(club) != 0
}

/// Person's contract wage via CONTRACT_MANAGER (`contract_value`, FN_CONTRACT_MANAGER_GET_WAGE), signed.
///
/// [pt] O valor do contrato da pessoa (0x4d0c20: mov ecx, [0xae1050]; push pessoa -> thiscall, ret 4), com sinal.
pub(crate) unsafe fn contract_wage(person: Staff) -> i32 {
    exe::contract_manager_get_wage(read_usize(G_CONTRACT_MANAGER), person) as i32
}

/// Wage bonus for big-spending clubs (England, Italy or Spain, reputation above 4750, wealth band 0..2) by wage factor
/// w and the PLC flag.
///
/// [pt] O bônus de quem paga mais (o trecho 0x596c0d..0x597238): clube da Inglaterra, Itália ou Espanha, reputação acima
/// de 4750, faixa 0 a 2, pelo fator salarial `w` (a palavra, com sinal) e pela marca +0x82.
pub(crate) unsafe fn big_spender_bonus(club: Club, band: i8, w: i32) -> i32 {
    let nation = club.nation().addr();
    if nation == 0 {
        return 0;
    }
    let n = read_i32(nation);
    if n != read_i32(KEY_NATION_ENGLAND) && n != read_i32(KEY_NATION_ITALY) && n != read_i32(KEY_NATION_SPAIN) {
        return 0;
    }
    if club.reputation() <= 0x128e || !(band == 2 || band == 1 || band == 0) {
        return 0;
    }
    let w16 = w as i16;
    let x = w16 as i32;
    let (base, oito) = if club.plc() != 0 { (0x4e2, 0x2710) } else { (0x2ee, 0x1770) };
    if w16 < 0x1676 || band == 0 {
        x.wrapping_sub(base).wrapping_mul(6)
    } else if band == 1 {
        x.wrapping_mul(8).wrapping_sub(oito)
    } else {
        x.wrapping_sub(base).wrapping_mul(10)
    }
}

/// Wage cap of a human manager from the non-player reputations (+0xb, +0xd), experience (+5), the person's age, flag
/// and job.
///
/// [pt] O teto do técnico humano pelas reputações (+0xb, +0xd) e pela experiência (+5) do não-jogador `np`, pela idade
/// (+0x18) e pela marca +0x22 da pessoa e pelo cargo (o trecho 0x597405..0x597566).
pub(crate) unsafe fn human_manager_wage_cap(person: Staff, np: usize, job: i8) -> i32 {
    let age = (person.age() as u8) as i8;
    let mark = person.int_apps() != 0;
    let cx = read_i16(np + player_sheet::CURRENT_REPUTATION);
    let manager = job < 4 && job != 0 && job != 7;
    if cx < 0xea6 {
        let ax = read_i16(np + player_sheet::CURRENT_ABILITY);
        if ax < 0x3c {
            if age <= 0x17 { 0x1388 } else { 0x1388 + 0x9c4 }
        } else if ax < 0x64 {
            if (job >= 4 || job == 0) && age < 0x18 { 0x2710 } else { 0x3a98 }
        } else {
            0x61a8
        }
    } else if cx < 0x1482 {
        if read_i16(np + player_sheet::CURRENT_ABILITY) >= 0x64 { 0x7530 } else { 0x7530 - 5000 }
    } else if cx < 0x1c52 {
        if age > 0x22 && read_i16(np + player_sheet::CURRENT_ABILITY) < 0x78 {
            if manager { 0x927c } else { 0x7ef4 }
        } else {
            let cx2 = read_i16(np + player_sheet::CURRENT_ABILITY);
            if cx2 < 0x8c && read_i16(np + player_sheet::WORLD_REPUTATION) < 0xea6 {
                0x9c40 + if mark { 0x1388 } else { 0 }
            } else if manager {
                if mark { 0xe09c } else if cx2 >= 0x8c { 0x13880 } else { 0x13880 - 15000 }
            } else {
                0xafc8
            }
        }
    } else if read_i16(np + player_sheet::WORLD_REPUTATION) < 0x1a5e {
        0x186a0 + if mark { 0x61a8 } else { 0 }
    } else {
        0x2ab98
    }
}

/// Computes the highest wage the club can offer: by reputation, wealth band and nation table; with a person, by the
/// current contract, reputation squared, the roles, the human ceiling, the market, the contract near its end and the
/// chairman; without a person, by the wage table less the payroll and budget per week, with the floor, the league and
/// the nation; adjustments for clubs with +0x64 1 and 2, the league minimum + 100 and the lower of both ceilings.
///
/// The asserts cite finance.CPP; 0x14ac bytes with x87. `Finance::get_max_wage(person, role)`, called from the contract
/// and transfer offers, the screens and the wage bill (`update_wage_bill`, without a person and with role −1). Club with +0x64 =
/// 3: 0. The wage factor by reputation (/ 50, plus the wealth band `get_wealth_band` and the nation's wage table: 0x9c8498 small
/// nations, 0x9c8548 medium, 0x9c85f8 France and clubs with +0x82, 0x9c86a8 the others) × 47.6 (ceiling 10000). With a
/// person: the value of the contract at the club (`contract_value`, up to 2 × the finances' ceiling +0x14, at least that; a
/// player limited by type +0x3d), the bonus for the one who pays more, reputation² × 0.25 × 0.0001 + the floor +0x10,
/// the rules for roles 1 to 3, the human's ceiling, the nation (Basque club `database_is_basque_club`, the market from the transfer
/// manager's table, the contract near its end `exe::date_days_since`), the chairman (+0xbf: the liking `exe::staff_is_favoured_by` and the favourite
/// `favourite_of`, the profile +0xf, the manager +0xcf) and roles 1 to 6. Without a person: the nation factor (+0x85 / 20 ×
/// the type +0x7e, for type 1 nations), the league (+0x57 and +0x5b, the size +0x69 cubed), the table by (factor / 10)
/// minus (+0xc − (factor³ × nation / divisor + balance × 1.3) / 52), with the floor 0x9c83e8 or 0x9c8338, the band and
/// the nation. Then the club with +0x64 1 and 2, the league minimum + 100 and the lower of the two.
/// Original: `Finance::get_max_wage` (0x5968d0, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x5968d0 } (thiscall, ret 8, fin_prm.cpp; os asserts citam finance.CPP; 0x14ac bytes com x87):
/// `Finance::get_max_wage(pessoa, cargo)`, pelas propostas de contrato e de transferência, pelas telas e pela folha
/// salarial (0x598370, sem pessoa e cargo −1). Clube de +0x64 = 3: 0. O fator salarial pela reputação (/ 50, mais pela
/// faixa de riqueza 0x5986b0 e pela tabela de salários da nação: 0x9c8498 nações pequenas, 0x9c8548 médias, 0x9c85f8
/// França e clubes de +0x82, 0x9c86a8 as outras) × 47,6 (teto 10000). Com pessoa: o valor do contrato no clube
/// (0x4d0c20, até 2 × o teto +0x14 das finanças, no mínimo ele; jogador limitado pelo tipo +0x3d), o bônus de quem
/// paga mais, a reputação² × 0,25 × 0,0001 + o piso +0x10, as regras dos cargos 1 a 3, o teto do humano, a nação
/// (clube basco 0x53d7f0, o mercado pela tabela do gerente de transferências, o contrato perto do fim 0x54cdb0), o
/// presidente (+0xbf: o gosto 0x547810 e o favorito 0x547790, o perfil +0xf, o técnico +0xcf) e os cargos 1 a 6. Sem
/// pessoa: o fator da nação (+0x85 / 20 × o tipo +0x7e, pelas nações do tipo 1), a liga (+0x57 e +0x5b, o tamanho
/// +0x69 ao cubo), a tabela por (fator / 10) menos (+0xc − (fator³ × nação / divisor + saldo × 1,3) / 52), com o piso
/// 0x9c83e8 ou 0x9c8338, a faixa e a nação. Depois o clube de +0x64 1 e 2, o mínimo da liga + 100 e o menor dos dois.
pub(crate) unsafe extern "thiscall" fn get_max_wage(fin: usize, person: usize, job_arg: u32) -> i32 {
    let person = Staff(person);
    let mut job = job_arg as u8;
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((read_i32(fin + finance::CLUB_ID) as u32).wrapping_mul(0x245) as usize));
    let budget = ftol(read_i64(fin) as f64 * read_f64(rdata::F64_1_3));
    let mut league: i32 = 100;
    let mut factor: f64 = 0.25; // [esp + 0x3c]: 0x3fd00000_00000000
    let mut contract: usize = 0;
    if club.professional_status() == 3 {
        return 0;
    }
    let rep = club.reputation();
    let mut di: i32 = rep as i32 / 50;
    let band: i8 = if fin_prm_is_national_team(club.addr()) { 0 } else { wealth_band(fin) };
    let nation = club.nation().addr();
    let e = |g: usize| read_i32(nation) == read_i32(g);
    let tabela: usize = if nation == 0 || e(KEY_NATION_WALES) || e(KEY_NATION_REPUBLIC_OF_IRELAND) || e(KEY_NATION_NORTHERN_IRELAND) || e(KEY_NATION_AUSTRALIA) {
        0x9c8498
    } else if [KEY_NATION_JAPAN, KEY_NATION_SWEDEN, KEY_NATION_CROATIA, KEY_NATION_POLAND, KEY_NATION_NORWAY, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, KEY_NATION_DENMARK, KEY_NATION_FINLAND, KEY_NATION_SCOTLAND, KEY_NATION_GERMANY].iter().any(|&g| e(g)) {
        0x9c8548
    } else if e(KEY_NATION_FRANCE) || (club.plc() != 0 && band != 2 && band != 1) {
        0x9c85f8
    } else {
        0x9c86a8
    };
    let default = tabela == 0x9c86a8;
    match band {
        0 => {
            if rep > 0x1676 {
                di = if default { (rep as i32 - 0x186a) / 25 + 0x73 } else { di + 5 };
            }
        }
        1 => {
            di = if rep <= 0x1482 { di + 5 } else if default { (rep as i32 - 0x1482) / 25 + 0x69 } else { di + 10 };
        }
        2 => {
            di = if rep <= 0x128e { di + 10 } else if default { (rep as i32 - 0x128e) / 25 + 0x5f } else { di + 0xf };
        }
        _ => {}
    }
    let mut s = ftol((di as i16) as f64 * read_f64(rdata::F64_47_6));
    if (s as i16) > 0x2710 {
        s = 0x2710;
    }
    if (di as i16) > 0xd2 {
        di = 0xd2;
    }
    let human = is_human_club_with(club, 0);
    if human != 0 && job == 0xff {
        contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club);
        if contract != 0 {
            job = read_u8(contract + contract::SQUAD_STATUS) >> 4;
        }
    }
    let mut r: i32; // [esp + 0x18]: the ceiling that comes out
    let b20: i32;
    if !person.is_null() {
        r = 0;
        if person.club_job() == club.addr() {
            r = contract_wage(person);
            let t = read_i32(fin + finance::WAGE_CEILING).wrapping_mul(2);
            if r > t {
                r = t;
            }
        }
        if read_i32(fin + finance::WAGE_CEILING) > r {
            r = read_i32(fin + finance::WAGE_CEILING);
        }
        if person.non_player() != 0 && person.player() == 0 {
            let cap = match (person.job_for_club() as i8 as i32).wrapping_sub(5) {
                0 | 1 | 2 => 0x88b8,
                3 => 0x4e20,
                4 => 0x5dc,
                5 => 0x3e8,
                _ => 0x2ee,
            };
            if r > cap {
                r = cap;
            }
        }
        b20 = big_spender_bonus(club, band, s);
    } else {
        let mut f = factor;
        if nation != 0 {
            let mut dl = read_u8(nation + nation::STATE_OF_DEVELOPMENT) as i8;
            if dl < 1 {
                dl = 1;
            } else if dl > 3 {
                dl = 3;
            }
            let bl = read_u8(nation + nation::LEAGUE_STANDARD) as i8;
            let q = bl as i32 as f64 / (dl as i32 * 20) as f64;
            f = q;
            if dl == 1 {
                let m = if e(KEY_NATION_ENGLAND) || e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN) || e(KEY_NATION_JAPAN) {
                    read_f64(rdata::F64_1)
                } else if e(KEY_NATION_UNITED_STATES) {
                    read_f64(rdata::F64_0_75)
                } else {
                    let (k, tres) = if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SCOTLAND) || e(KEY_NATION_HOLLAND) {
                        (0x96a5d0, false)
                    } else if e(KEY_NATION_BRAZIL) || e(KEY_NATION_ARGENTINA) || e(KEY_NATION_SWEDEN) || e(KEY_NATION_BELGIUM) || e(KEY_NATION_PORTUGAL) {
                        (0x96a5d8, false)
                    } else if e(KEY_NATION_DENMARK) || e(KEY_NATION_NORWAY) || e(KEY_NATION_FINLAND) {
                        (0x969510, true)
                    } else {
                        (0x968e80, false)
                    };
                    let d16 = di as i16 as i32;
                    if d16 <= (bl as i32).wrapping_mul(10) {
                        read_f64(k)
                    } else {
                        let div = if bl < 1 { 1 } else { bl as i32 };
                        let mut m = d16 as f64 / div as f64 * read_f64(rdata::F64_0_1) * read_f64(k);
                        if tres && !(m <= read_f64(rdata::F64_0_75)) {
                            m = read_f64(rdata::F64_0_75);
                        } else if !(m <= read_f64(rdata::F64_1)) {
                            m = read_f64(rdata::F64_1);
                        }
                        m
                    }
                };
                f = q * m;
            }
            factor = f;
            let lc = club.division().addr();
            if lc != 0 {
                let d2 = club.last_division().addr();
                let c16 = read_i16(lc + competition::REPUTATION);
                let cubo = |x: i32| x.wrapping_mul(x).wrapping_mul(x);
                let v = if d2 != 0 && read_i16(d2 + competition::REPUTATION) > c16 {
                    let t1 = ftol(cubo(c16 as i32) as f64 * f * read_f64(rdata::F64_NEG_1_25));
                    let t2 = ftol(cubo(read_i16(d2 + competition::REPUTATION) as i32) as f64 * f * read_f64(rdata::F64_1_25));
                    t2.wrapping_sub(t1)
                } else {
                    ftol(cubo(c16 as i32) as f64 * f * read_f64(rdata::F64_2_5))
                };
                league = if v < 0xc8 { 0xc8 } else { v };
            }
        }
        let d16 = di as i16 as i32;
        let mut q10 = (200 - d16) / 10;
        if q10 < 1 {
            q10 = 1;
        }
        let a = d16 as f64;
        let t = ftol(a * a * a * f / q10 as f64);
        crate::comparison::branch("cqc_2 ceiling by the table");
        let minus = read_i32(fin + finance::WAGE_BILL).wrapping_sub(t.wrapping_add(budget) / 52); // cqc2: the payroll and the budget per week
        let idx = ((d16 / 10) * 8) as isize as usize;
        let vv = read_i64(tabela.wrapping_add(idx)).wrapping_sub(minus as i64) as f64 * f;
        let floor = read_i64((if default { 0x9c83e8usize } else { 0x9c8338 }).wrapping_add(idx)) as f64 * f;
        r = if vv <= floor { ftol(floor) } else { ftol(vv) };
        if !fin_prm_is_national_team(club.addr()) {
            match wealth_band(fin) {
                1 => r = mul_exe_f64(r, 0x9689b8),
                2 => r = mul_exe_f64(r, 0x9689c0),
                _ => {}
            }
            if nation != 0 {
                let ita_esp = e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN);
                if ita_esp {
                    if rep > 0x1e46 {
                        r = mul_exe_f64(r, 0x9689b8);
                    }
                } else if e(KEY_NATION_ENGLAND) && rep > 0x1e46 {
                    r = mul_exe_f64(r, 0x968fa0);
                }
                if club.plc() == 0 && rep > 0x1e46 && ita_esp {
                    r = mul_exe_f64(r, 0x9689b8);
                }
            }
        }
        b20 = big_spender_bonus(club, band, s);
    }
    if !person.is_null() {
        let repf = rep as i32 as f64;
        let t = repf * repf * factor * read_f64(rdata::F64_0_0001) + read_i32(fin + finance::HIGHEST_WAGE) as f64;
        let v = contract_wage(person);
        s = if (v as f64) < t {
            crate::comparison::branch("cqc_2 ceiling by injury_word_0c");
            ftol(repf * repf * factor * read_f64(rdata::F64_0_0001) + read_i32(fin + finance::HIGHEST_WAGE) as f64) // cqc2: reputation squared;
        } else {
            contract_wage(person)
        };
        let np = person.player();
        let c = job as i8;
        if np != 0 && read_i16(np + player_sheet::CURRENT_REPUTATION) > 0x1482 && (c == 3 || c == 2 || c == 1) && ((person.age() as u8) as i8) < 0x20 {
            let np2 = || person.player();
            if c == 3 {
                let bf = b20 as f64;
                let lim = bf * read_f64(rdata::F64_0_8);
                if (r as f64) < lim {
                    r = if is_human_club_with(club, 0) == 0 && np2() != 0 && read_i16(np2() + 0xb) > 0x1c52 { ftol(bf * read_f64(rdata::F64_0_9)) } else { ftol(lim) };
                }
            } else if c == 2 {
                let lim = b20 as f64 * read_f64(rdata::F64_0_9);
                if (r as f64) < lim {
                    r = if is_human_club_with(club, 0) == 0 && np2() != 0 && read_i16(np2() + 0xb) > 0x1e46 { b20 } else { ftol(lim) };
                }
            } else if r < b20 {
                r = b20;
            }
        }
        if is_human_club_with(club, 0) != 0 {
            let np = person.player();
            if np != 0 {
                let ceiling = human_manager_wage_cap(person, np, job as i8);
                if ceiling < r {
                    r = ceiling.wrapping_sub(ftol(r.wrapping_sub(ceiling) as f64 * read_f64(rdata::F64_NEG_0_1)));
                }
            }
        }
        let np = person.player();
        if np != 0 {
            if nation != 0 && (e(KEY_NATION_ITALY) || e(KEY_NATION_ENGLAND) || e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SPAIN)) {
                if exe::database_is_basque_club(club) != 0 {
                    s = mul_exe_f64(s, 0x9689b8);
                }
                let tm = read_usize(G_TRANSFER_MANAGER);
                if tm != 0 {
                    let rr = club.reputation();
                    let div: i32 = if rr < 0x3e8 { 0x3e8 } else { rr as i32 };
                    let table = read_usize(read_usize(tm + 0x856) + 0xdc);
                    let x = read_i16(table.wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(9)) + 5) as i32;
                    let mut m = x as f64 / div as f64;
                    if !(read_f64(rdata::F64_1) < m || read_f64(rdata::F64_1) == m) {
                        m = read_f64(rdata::F64_1);
                    } else if read_f64(rdata::F64_1_2) < m {
                        m = read_f64(rdata::F64_1_2);
                    }
                    s = ftol(s as f64 * m);
                }
                // the contract near its end (0x54cdb0: lea ecx, contract + 0x2d; push [0xae2c94]; push [0xae2c90] ->
                // thiscall, ret 8), less than 200 days
                let near = || {
                    contract != 0 && person.club_job() == club.addr() && exe::date_days_since(contract + contract::CONTRACT_EXPIRES, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 0xc8
                };
                let rr = club.reputation();
                let k: Option<usize> = if rr > 0x186a {
                    let np = person.player();
                    if near() {
                        let np = person.player();
                        if read_i16(np + player_sheet::WORLD_REPUTATION) > 0x1482 {
                            Some(0x968978)
                        } else {
                            let ax = read_i16(np + player_sheet::CURRENT_REPUTATION);
                            let cl = read_u8(contract + contract::SQUAD_STATUS) & 0xf0;
                            if ax > 0x1c52 {
                                Some(0x9689c0)
                            } else if (cl == 0x30 && ax > 0x1482) || cl == 0x10 || cl == 0x20 {
                                Some(0x9689b8)
                            } else {
                                None
                            }
                        }
                    } else if read_i16(np + player_sheet::WORLD_REPUTATION) > 0x1482 {
                        Some(0x9689c0)
                    } else if read_i16(np + player_sheet::CURRENT_REPUTATION) > 0x1a5e {
                        Some(0x9689b8)
                    } else {
                        None
                    }
                } else if rr > 0x109a {
                    if near() {
                        let cx = read_i16(person.player() + 0xb);
                        let al = read_u8(contract + contract::SQUAD_STATUS) & 0xf0;
                        if cx > 0x1a5e {
                            Some(0x9689b8)
                        } else if (al == 0x30 && cx > 0x1482) || al == 0x10 || al == 0x20 {
                            Some(0x968f88)
                        } else {
                            None
                        }
                    } else if read_i16(person.player() + 0xb) > 0x1e46 {
                        Some(0x968f88)
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(k) = k {
                    s = mul_exe_f64(s, k);
                }
            }
            let presence = club.chairman().addr();
            if presence != 0 && read_usize(presence + staff::NON_PLAYER) != 0 && (club.reputation() as i32) < read_i16(person.player() + 0xb) as i32 + 0x4e2 {
                crate::comparison::branch("cqc_2 chairman");
                s = ftol(((read_u8(presence + staff::AMBITION) as i8 as i32) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_9)) * s as f64); // cqc2: o presidente;
                // 0x547810 (push pres; push person; add esp, 8 -> cdecl), 0x547790 (push club; push person; add esp, 8 -> cdecl)
                if exe::staff_is_favoured_by(person, presence) != 0 || exe::club_favours_staff(person, club) != 0 {
                    s = ftol(((read_u8(club.chairman().addr() + 0x57) as i8 as i32) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_1)) * s as f64);
                }
                let presence = club.chairman().addr();
                let pc = read_usize(presence + staff::NON_PLAYER);
                let perfil = read_u8(pc + non_player::BUSINESS) as i8;
                if perfil < 6 {
                    let t = club.manager().addr();
                    let k = if t != 0 && (read_u8(t + 0x57) as i8) > 0xf && read_usize(t + 0x69) != 0 && read_i16(read_usize(t + 0x69) + 0xa) > 0x1c52 { 0x9689c0 } else { 0x9689b8 };
                    s = mul_exe_f64(s, k);
                } else if perfil > 0xf {
                    s = mul_exe_f64(s, 0x9689d0);
                }
                if club.plc() == 0 && (read_u8(pc + non_player::RESOURCES) as i8) > 0xf && (read_u8(presence + staff::AMBITION) as i8) > 0xf {
                    let rr = club.reputation();
                    let k = if rr >= 0xabe && read_i16(person.player() + 0xb) >= 0x1c52 {
                        if rr < 0x1676 { 0x9689c0 } else { 0x96a5a8 }
                    } else {
                        0x9689b8
                    };
                    s = mul_exe_f64(s, k);
                }
            }
            let c = job as i8;
            let np = person.player();
            match c {
                1 => s = mul_exe_f64(s, 0x968928),
                2 => {
                    if is_human_club_with(club, 1) == 0 && read_i16(np + player_sheet::CURRENT_REPUTATION) > 0x1a5e {
                        s = mul_exe_f64(s, 0x968928);
                    } else {
                        s = ftol(s as f64);
                        let ok = is_human_club_with(club, 1) != 0 || {
                            let np = person.player();
                            np != 0 && read_i16(np + player_sheet::CURRENT_REPUTATION) < 0x1e46
                        };
                        if ok && s > 0x14c08 {
                            s = ftol(s.wrapping_sub(0x14c08) as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_85000));
                        }
                    }
                }
                3 => {
                    if is_human_club_with(club, 1) == 0 && read_i16(np + player_sheet::CURRENT_REPUTATION) > 0x1a5e {
                        s = mul_exe_f64(s, 0x968928);
                    } else {
                        s = mul_exe_f64(s, 0x9689d0);
                        if s > 0xd6d8 {
                            s = 0xd6d8;
                        }
                    }
                }
                4 => {
                    let k = if is_human_club_with(club, 1) == 0 && read_i16(np + player_sheet::CURRENT_REPUTATION) > 0x1a5e { 0x9689d0 } else { 0x969040 };
                    s = mul_exe_f64(s, k);
                    if s > 0x927c {
                        s = 0x927c;
                    }
                }
                5 => {
                    s = if s > 0xc80 { ftol(s.wrapping_sub(0xc80) as f64 * read_f64(rdata::F64_0_3) + read_f64(rdata::F64_1920)) } else { mul_exe_f64(s, 0x968e48) };
                    if s > 0x30d4 {
                        s = 0x30d4;
                    }
                }
                6 => {
                    s = if s > 0xc80 { ftol(s.wrapping_sub(0xc80) as f64 * read_f64(rdata::F64_0_15) + read_f64(rdata::F64_1600)) } else { mul_exe_f64(s, 0x968968) };
                    if s > 0x203a {
                        s = 0x203a;
                    }
                }
                _ => {}
            }
        }
    } else {
        let x = (s as i16) as i32 as f64;
        s = ftol(x * x * factor * read_f64(rdata::F64_0_0001) + read_i32(fin + finance::HIGHEST_WAGE) as f64);
        if s < b20 {
            s = ftol(s as f64 * read_f64(rdata::F64_0_75) + b20 as f64 * read_f64(rdata::F64_0_25));
        }
        if !fin_prm_is_national_team(club.addr()) && nation != 0 {
            if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) {
                match wealth_band(fin) {
                    1 => s = mul_exe_f64(s, 0x9689b8),
                    2 => s = mul_exe_f64(s, 0x9689c0),
                    _ => {}
                }
            } else if e(KEY_NATION_ITALY) || e(KEY_NATION_ENGLAND) || e(KEY_NATION_SPAIN) {
                match wealth_band(fin) {
                    0 => {
                        if club.reputation() > 0x109a {
                            s = mul_exe_f64(s, 0x968fa0);
                        }
                    }
                    1 => s = mul_exe_f64(s, 0x9689b8),
                    2 => s = mul_exe_f64(s, 0x9689c0),
                    _ => {}
                }
            }
        }
    }
    let kind = club.professional_status();
    if kind == 1 {
        if s < 0xfa {
            s = 0xfa;
        }
        if club.reputation() < 0x8ca && s > 0x834 {
            s = ftol((s as f64 - read_f32(rdata::F32_2100) as f64) * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_2100));
        }
    } else if kind == 2 {
        s = mul_exe_f64(s, 0x969040);
        if club.reputation() < 0xabe && s > 0x4b0 {
            s = ftol((s as f64 - read_f32(rdata::F32_1200) as f64) * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_1200));
        }
        if s < 0x96 {
            s = 0x96;
        }
    }
    if s < league.wrapping_add(100) {
        s = league.wrapping_add(100);
    }
    if league > r {
        return league;
    }
    if s >= r { r } else { s }
}

/// Sends news 0x177d with the club id, the amount and the works type to the club's manager.
///
/// SEH frame only for the local news item, no destructor on the normal path. `FinanceTable::news_ground_works(club,
/// amount, kind)`, called from the club's season end (`season_end_club_update`: stadium purchase, heating, expansion and seating,
/// training facilities) and from the week's events (`process_club_finance_events`). News 0x177d (`construct_news_item`: lea ecx, local; push 0; push
/// 0x177d -> thiscall, ret 8) with the club id, the amount and the kind (signed byte; `set_news_field`: lea ecx, local; 2
/// pushes -> thiscall, ret 8), delivered to the club's manager (`send_to_club_manager`: ecx = `[0xb7431c]`; push club; push &local ->
/// thiscall, ret 8).
/// Original: `FinanceTable::news_ground_works` (0x59ffb0, fin_prm.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x59ffb0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_ground_works(clube, valor, tipo)`, pelo fim de temporada do clube (0x59cfa0: compra do estádio,
/// aquecimento, expansão e lugares, centro de treino) e pelos eventos da semana (0x59ed70). A notícia 0x177d (0x77bbe0:
/// lea ecx, local; push 0; push 0x177d -> thiscall, ret 8) com o id do clube, o valor e o tipo (byte com sinal; 0x785790:
/// lea ecx, local; 2 pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; push clube;
/// push &local -> thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn news_ground_works(_this: usize, club: usize, value: u32, kind: u32) {
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x177d, 0);
    exe::newsitem_set_slot(p, 0, read_usize(club) as u32);
    exe::newsitem_set_slot(p, 1, value);
    exe::newsitem_set_slot(p, 2, kind as u8 as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Sends news 0x1788 with the subject and the reason (2, or a random draw of 2) to the club's manager; no club,
/// nothing.
///
/// SEH frame only for the local news item, no destructor on the normal path. `FinanceTable::news_request_refused(club,
/// subject, reason)`, called from the club's season end (`season_end_club_update`: the stadium or training facilities request refused).
/// No club, nothing. News 0x1788 (`construct_news_item`) with the subject (signed word) in data slot 1 and, in data slot 2, reason 2
/// or random(2) (`exe::utils_random`: push 2; add esp, 4 -> cdecl; the signed word), delivered to the club's manager (`send_to_club_manager`).
/// Original: `FinanceTable::news_request_refused` (0x59fae0, fin_prm.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x59fae0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_request_refused(clube, assunto, motivo)`, pelo fim de temporada do clube (0x59cfa0: o pedido de
/// estádio ou de centro de treino recusado). Sem clube, nada. A notícia 0x1788 (0x77bbe0) com o assunto (palavra com
/// sinal) no dado 1 e no dado 2 o motivo 2 ou o sorteio(2) (0x90cfc0: push 2; add esp, 4 -> cdecl; a palavra com
/// sinal), entregue ao técnico do clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn news_request_refused(_this: usize, club: usize, subject: u32, reason: u32) {
    if club == 0 {
        return;
    }
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1788, 0);
    exe::newsitem_set_slot(p, 1, subject as u16 as i16 as i32 as u32);
    let v = if reason as u16 == 2 { 2 } else { exe::utils_random(2) };
    exe::newsitem_set_slot(p, 2, v as u16 as i16 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Returns the number of clubs (G_CLUB_COUNT).
pub(crate) unsafe fn club_count() -> u32 {
    *(G_CLUB_COUNT as *const u32)
}

/// writes the finances of every club into the finance.dat item (0x167 bytes × [0xae235c] from [this]); no container,
/// nothing; on error the "Error" box (finance.CPP 0x115e, 0x1166) without closing the item
/// Original: `FinanceTable::save_finance_dat` (0x59b3b0, fin_prm.cpp, thiscall).
///
/// [pt] { 0x59b3b0 } (thiscall, ret 4, fin_prm.cpp; o fonte se chama finance.CPP): `FinanceTable::save_finance_dat(save)`:
/// grava no item `finance.dat` (0x9326e0 gravando) as finanças de todos os clubes, 0x167 bytes × [0xae235c] a partir
/// de [this]. Sem contêiner, nada; erro ao abrir ou ao gravar: a caixa "Error" (finance.CPP 0x115e, 0x1166), sem
/// fechar o item (como a original).
pub(crate) unsafe extern "thiscall" fn save_finance_dat(this: usize, save: usize) {
    crate::comparison::branch("cpx_5 save finances");
    if save == 0 {
        return;
    }
    let mut z = exe::zopen(save, 0x9c87b0, 1, 0x16, 4);
    if z == 0 {
        show_error_at(0x9c8784, 0x115e);
        return;
    }
    if exe::zwrite(read_usize(this), 0x167, club_count(), z) != club_count() { // cpx5: the finance records
        show_error_at(0x9c8784, 0x1166);
        return;
    }
    exe::zclose(&mut z);
}

/// Reads the finances of every club from the finance.dat item into `[this]` and returns 1; no container, 0; on error
/// the "Error" box (0x1137, 0x113f) and 0 (the constructor then rebuilds the finances from scratch).
///
/// `FinanceTable::load_from_save(save)`: reads from the `finance.dat` item (`exe::zopen`, reading) the finances of all
/// clubs, 0x167 bytes × `[0xae235c]`, into `[this]`, and returns 1. No container: 0 with no box; error on open or short
/// read: the "Error" box (finance.CPP 0x1137, 0x113f), 0, and the item stays open (as the original). With 0, the
/// constructor (0x59a55c) rebuilds each club's finances from scratch (0x5961d0).
/// Original: `FinanceTable::load_from_save` (0x59b2a0, fin_prm.cpp, thiscall).
///
/// [pt] { 0x59b2a0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::load_from_save(save)`: lê do item `finance.dat`
/// (0x9326e0 lendo) as finanças de todos os clubes, 0x167 bytes × [0xae235c] em [this], e devolve 1. Sem contêiner,
/// 0 sem caixa; erro ao abrir ou leitura curta: a caixa "Error" (finance.CPP 0x1137, 0x113f), 0 e o item fica aberto
/// (como a original). Com 0, o construtor (0x59a55c) refaz as finanças de cada clube do zero (0x5961d0).
pub(crate) unsafe extern "thiscall" fn load_from_save(this: usize, save: usize) -> u32 {
    crate::comparison::branch("cpx_5 load finances");
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, 0x9c87b0, 0, 0x16, 4);
    if z == 0 {
        show_error_at(0x9c8784, 0x1137);
        return 0;
    }
    if exe::zread(read_usize(this), 0x167, club_count(), z) != club_count() {
        show_error_at(0x9c8784, 0x113f);
        return 0;
    }
    exe::zclose(&mut z);
    1 // cpx5: loaded (the constructor does not redo the finances)
}

/// The finance record (0x167 bytes) of the match club in `campo` (`+0x1c` home, `+0x20` away), found the way the
/// original routine finds it without `0x5a0590`.
///
/// Uses the partner club (`0x540a50`: 3 pushes; add esp, 0xc -> cdecl(club, &a, 1)) if it exists and the club is the B
/// team (a = 0), and [table] + id × 0x167. Some(0) without a club; None if it is a national team (`0x53b900`: push; add
/// esp, 4 -> cdecl): the original routine exits without doing anything.
///
/// [pt] O registro de finanças (0x167 bytes) do clube da partida em `campo` (+0x1c o mandante, +0x20 o visitante), como a
/// original o acha sem a 0x5a0590: o clube parceiro (0x540a50: 3 pushes; add esp, 0xc -> cdecl(clube, &a, 1)) se ele
/// existe e o clube é o B (a = 0), e [tabela] + id × 0x167. Some(0) sem clube; None se for seleção (0x53b900: push;
/// add esp, 4 -> cdecl): a original sai sem fazer nada.
pub(crate) unsafe fn match_club_finance(table: usize, partida: usize, field: usize) -> Option<usize> {
    let club = read_usize(partida + field);
    if club == 0 {
        return Some(0);
    }
    if exe::database_is_national_team(club) != 0 {
        return None;
    }
    let mut eh_a = 0u32;
    let p = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { club };
    Some(read_usize(table).wrapping_add((read_i32(c) as usize).wrapping_mul(0x167)))
}

/// Signed division truncated toward zero, as the original's magic-number divisions.
///
/// [pt] A divisão com o arredondamento para zero da original (os imul por constante mágica e o cdq/sar).
pub(crate) fn div_trunc(a: i32, b: i32) -> i32 {
    a.wrapping_div(b)
}

/// Processes a match's money: the TV money by attendance (1.75 × attendance), the gate receipts (+0x38 of the result),
/// the match costs and the home club's expenses (with random draws), the competition TV money by stage or type, divided
/// by the nation type; in the home club's league, TV 3/4 and 1/4, the net to the home club and the stadium upkeep;
/// outside it, half of each to both.
///
/// `FinanceTable::process_match_income(fixture, result)`, called from the closing of the day's match (`close_day_match`). No
/// result or no fixture: the error box (the original). A national team on either side: nothing. The amounts: with
/// attendance (+0x34 of the result), the TV by attendance (1 / (random(10) + 2) as an integer, which is 0, plus 1.75, ×
/// attendance) and the gate receipts (+0x38); the match costs (unless both clubs have +0x64 = 1: attendance / 4;
/// otherwise by bands of attendance or of the capacity +0x3c of the fixture's stadium +0x18, with a floor of 500 by
/// flag +0x40); the home club's expenses (with +0x64 = 1: random + attendance × 1.25 + reputation / 2 + 300, × 1.25
/// above 4750, or random(400) + attendance × 1.25 + 600 without reputation; otherwise two random draws + attendance);
/// the competition TV money by stage (+0x3b) or type (+0x3a: finals by the reputations, 6 and 4 by the competition's
/// +0x69); all divided by the nation type (+0x7e: 2 and 3) and once more in Wales and the Irelands. In the home club's
/// league (+0x57 = +0x14, not at a neutral ground +0x3c): TV 3/4 to the home club and 1/4 to the away club (+0x158,
/// +0xb8, balance), the net to the home club (+0x140, +0xa0, balance) and, with its own stadium (+0x6d, +0x69), the
/// upkeep ((random(15) + 1) × 0.01 + 0.85) × the stadium's +0x3c (+0x12c, +0x8c, subtracted from the balance). Outside
/// it: half of each to both.
/// Original: `FinanceTable::process_match_income` (0x59a5d0, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59a5d0 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::process_match_income(partida, resultado)`, pelo fechamento
/// da partida do dia (0x6b5f00). Sem resultado ou sem partida: a caixa (a original). Seleção de um dos lados: nada. Os
/// valores: com público (+0x34 do resultado), a TV pelo público (1 / (sorteio(10) + 2) inteiro, que é 0, mais 1,75, ×
/// público) e a renda do estádio (+0x38); os custos do jogo (sem os dois clubes de +0x64 = 1: público / 4; senão por
/// faixas do público ou da capacidade +0x3c do estádio +0x18 da partida, com piso de 500 pela marca +0x40); os gastos
/// do mandante (de +0x64 = 1: sorteio + público × 1,25 + reputação / 2 + 300, × 1,25 acima de 4750, ou sorteio(400) +
/// público × 1,25 + 600 sem reputação; senão dois sorteios + público); o dinheiro da TV da competição pela fase (+0x3b)
/// ou pelo tipo (+0x3a: as finais pelas reputações, 6 e 4 pelo +0x69 da competição); tudo dividido pelo tipo da nação
/// (+0x7e: 2 e 3) e outra vez no País de Gales e nas Irlandas. Na liga do mandante (+0x57 = +0x14, fora de campo
/// neutro +0x3c): a TV 3/4 ao mandante e 1/4 ao visitante (+0x158, +0xb8, saldo), o líquido ao mandante (+0x140,
/// +0xa0, saldo) e, com estádio próprio (+0x6d, +0x69), a manutenção ((sorteio(15) + 1) × 0,01 + 0,85) × o +0x3c do
/// estádio (+0x12c, +0x8c, menos no saldo). Fora dela: metade de cada um para os dois.
pub(crate) unsafe extern "thiscall" fn process_match_income(table: usize, partida: usize, result: usize) {
    if result == 0 || partida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PROCESS_MATCH_INCOME)(table, partida, result); // a caixa
    }
    let Some(mandante) = match_club_finance(table, partida, 0x1c) else { return };
    let Some(visitante) = match_club_finance(table, partida, 0x20) else { return };
    let (mut tv, mut publico, mut renda) = (0i32, 0i32, 0i32);
    if read_i32(result + 0x34) != 0 {
        publico = read_i32(result + 0x34);
        renda = read_i32(result + 0x38);
        let q = div_trunc(1, friendly_random(10).wrapping_add(2));
        crate::comparison::branch("cqb_1 paying attendance");
        tv = ftol((q as f64 + read_f64(rdata::F64_1_75)) * publico as f64); // cqb1: TV by attendance (1,75);
    }
    let home = read_usize(partida + fixture::HOME_CLUB);
    let away = Club(read_usize(partida + fixture::AWAY_CLUB));
    let mut cost; // the original’s ebp
    if read_u8(home + club::PROFESSIONAL_STATUS) != 1 && away.professional_status() != 1 {
        cost = ftol(publico as f64 * read_f64(rdata::F64_0_25));
    } else {
        let est = read_usize(partida + fixture::STADIUM);
        let x = if est == 0 {
            let k = if publico > 20000 {
                0x96a6a0
            } else if publico > 10000 {
                0x96a698
            } else if publico > 5000 {
                0x968e48
            } else {
                0x968968
            };
            publico as f64 * read_f64(k)
        } else {
            let cap = read_i32(est + stadium::CAPACITY);
            let k = if cap > 17500 {
                0x968fe0
            } else if cap > 12500 {
                0x968968
            } else if cap > 5000 {
                if publico < 5000 { 0x968e00 } else { 0x96a6a8 }
            } else {
                0x969510
            };
            cap as f64 * read_f64(k)
        };
        cost = ftol(x);
        if read_u8(partida + fixture::HOME_SIDE_MARK) != 0 && cost < 500 {
            cost = 500;
        }
    }
    let home = read_usize(partida + fixture::HOME_CLUB);
    let mut gastos; // the original’s [esp+0x10] (starts with the attendance)
    if read_u8(home + club::PROFESSIONAL_STATUS) == 1 {
        let rep = read_i16(home + club::REPUTATION);
        if rep != 0 {
            let r = friendly_random(0xc8);
            let x = (r as f64 + publico as f64 * read_f64(rdata::F64_1_25)) + rep as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_300);
            gastos = ftol(x);
            if rep > 0x128e {
                gastos = ftol(gastos as f64 * read_f64(rdata::F64_1_25));
            }
        } else {
            let r = friendly_random(0x190);
            gastos = ftol((r as f64 + publico as f64 * read_f64(rdata::F64_1_25)) + read_f64(rdata::F64_600));
        }
    } else {
        let rep = read_i16(home + club::REPUTATION);
        let r1 = if rep != 0 { friendly_random(ftol(rep as i32 as f64 * read_f64(rdata::F64_0_25))) } else { friendly_random(0xfa) };
        let r1f = r1 as f32; // fstp dword: the first roll stored as float
        let r2 = friendly_random(0xfa);
        gastos = ftol((r2 as f64 + r1f as f64) + publico as f64);
    }
    let mut dinheiro = 0i32; // the original’s [esp+0x1c]
    let kind_ok = match (read_u8(partida + fixture::STAGE) as i8 as i32).wrapping_add(1) as u32 {
        1 => {
            dinheiro = 25000;
            true
        }
        2 => {
            dinheiro = 75000;
            true
        }
        3 => {
            dinheiro = 125000;
            true
        }
        _ => false, // 0 (stage −1) and outside the table: by type
    };
    if !kind_ok {
        let t = read_u8(partida + fixture::CATEGORY);
        if t == 8 || t == 7 {
            let sum = read_i16(home + club::REPUTATION) as i32 + away.reputation() as i32;
            dinheiro = if sum > 10000 {
                125000
            } else if sum <= 7500 {
                25000
            } else {
                75000
            };
        } else if t == 6 || t == 4 {
            let w = read_i16(read_usize(partida + fixture::COMPETITION) + 0x69);
            let m = if w > 5 { w as i32 } else { 5 };
            dinheiro = div_trunc(m.wrapping_mul(if t == 6 { 125000 } else { 25000 }), 20);
        }
    }
    let nation = read_usize(home + club::NATION);
    let kind = if nation != 0 { read_u8(nation + nation::STATE_OF_DEVELOPMENT) as i8 } else { 3 };
    if kind == 2 {
        dinheiro = div_trunc(dinheiro, 5);
        tv = div_trunc(tv, 2);
        cost = div_trunc(cost, 4);
        gastos = div_trunc(gastos, 2);
    } else if kind != 1 {
        dinheiro = div_trunc(dinheiro, 10);
        tv = div_trunc(tv, 10);
        cost = div_trunc(cost, 15);
        gastos = div_trunc(gastos, 10);
    }
    if home != 0 && nation != 0 {
        let n = read_i32(nation);
        if n == read_i32(KEY_NATION_WALES) || n == read_i32(KEY_NATION_REPUBLIC_OF_IRELAND) || n == read_i32(KEY_NATION_NORTHERN_IRELAND) {
            dinheiro = div_trunc(dinheiro, 5);
            tv = div_trunc(tv, 2);
            cost = div_trunc(cost, 4);
            gastos = div_trunc(gastos, 2);
        }
    }
    let liquido = tv.wrapping_sub(gastos).wrapping_sub(cost).wrapping_add(renda);
    let add = |fin: usize, a: usize, b: usize, v: i32| {
        fin_prm_add_i32(fin + a, v);
        fin_prm_add_i32(fin + b, v);
        add_to_balance(fin, v as i64);
    };
    if read_u8(partida + fixture::NEUTRAL_VENUE) == 0 && read_usize(read_usize(partida + fixture::HOME_CLUB) + 0x57) == read_usize(partida + fixture::COMPETITION) {
        let x = dinheiro as f64;
        add(mandante, 0x158, 0xb8, ftol(x * read_f64(rdata::F64_0_75)));
        add(visitante, 0x158, 0xb8, ftol(x * read_f64(rdata::F64_0_25)));
        crate::comparison::branch("cqb_1 net to the home side");
        add(mandante, 0x140, 0xa0, liquido); // cqb1: o líquido ao mandante
        let home = read_usize(partida + fixture::HOME_CLUB);
        let est = read_usize(home + club::STADIUM);
        if read_u8(home + club::OWN_STADIUM) != 0 && est != 0 {
            let base = read_i32(est + stadium::CAPACITY);
            let r = friendly_random(0xf).wrapping_add(1);
            let c = ftol((r as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_85)) * base as f64);
            fin_prm_add_i32(mandante + 0x12c, c);
            fin_prm_add_i32(mandante + 0x8c, c);
            add_to_balance(mandante, -(c as i64));
        }
        return;
    }
    let half = ftol(dinheiro as f64 * read_f64(rdata::F64_0_5));
    add(mandante, 0x158, 0xb8, half);
    add(visitante, 0x158, 0xb8, half);
    let l2 = div_trunc(liquido, 2);
    add(mandante, 0x140, 0xa0, l2);
    add(visitante, 0x140, 0xa0, l2);
}

/// Computes the gate receipts: the price by the home club's reputation (/ 500; × 0.5 in friendlies, + 3 or + 4 in
/// competitions) × attendance, less the finance discount in the home club's league; at a neutral ground, by the
/// competition's +0x69; adjusted by nation type, league, key clubs and the nation at 0x9cf314.
///
/// `FinanceTable::calc_gate_receipts(fixture, attendance)`, called from the fixture record (`fill_match_record`, the result's
/// +0x38, which `process_match_income` adds to the home club's net). No fixture: the error box (the original); no attendance: 0;
/// national team at home: 0. At a neutral ground (+0x3c) of competition +0x14: (its +0x69 + 8) × attendance. Otherwise
/// the price by the home club's reputation (+0x80 / 500): without a competition or in a friendly (`exe::friendly_is_friendly_competition`: push comp;
/// ecx = `[0xae2c74]`; no add esp -> thiscall, ret 4), × 0.5 × attendance; otherwise + 4 (one of the clubs with +0x64 =
/// 1) or + 3, × attendance; in the home club's league (+0x57), minus the discount (the finances' float +0x160) × the
/// amount. Then by the nation type (+0x7e: 2 → /4; 3 → /10), × 1.25 in the league of 0x9cf570 or by the clubs of
/// 0x9d0250 (1.5), 0x9d0110 and 0x9d024c (1.3), 0x9d0248 (0.75) and 0x9d0244 (0.9), and × 1.2 in the nation of
/// 0x9cf314.
/// Original: `FinanceTable::calc_gate_receipts` (0x59af40, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59af40 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::calc_gate_receipts(partida, publico)`, pelo registro da
/// partida (0x729ff0, o +0x38 do resultado, que a 0x59a5d0 soma ao líquido do mandante). Sem partida: a caixa (a
/// original); sem público: 0; seleção mandante: 0. Em campo neutro (+0x3c) da competição +0x14: (+0x69 dela + 8) ×
/// público. Senão o preço pela reputação do mandante (+0x80 / 500): sem competição ou em amistoso (0x5c6710: push comp;
/// ecx = [0xae2c74]; sem add esp -> thiscall, ret 4), × 0,5 × público; senão + 4 (um dos clubes de +0x64 = 1) ou + 3, ×
/// público; na liga do mandante (+0x57), menos o desconto (float +0x160 das finanças) × o valor. Depois pelo tipo da
/// nação (+0x7e: 2 → /4; 3 → /10), × 1,25 na liga de 0x9cf570 ou pelos clubes de 0x9d0250 (1,5), 0x9d0110 e 0x9d024c
/// (1,3), 0x9d0248 (0,75) e 0x9d0244 (0,9), e × 1,2 na nação de 0x9cf314.
pub(crate) unsafe extern "thiscall" fn calc_gate_receipts(table: usize, partida: usize, attendance: i32) -> i32 {
    if partida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_CALC_GATE_RECEIPTS)(table, partida, attendance); // a caixa
    }
    if attendance == 0 {
        return 0;
    }
    let Some(fin) = match_club_finance(table, partida, 0x1c) else { return 0 };
    let comp = read_usize(partida + fixture::COMPETITION);
    let mut v: i32;
    if read_u8(partida + fixture::NEUTRAL_VENUE) != 0 && comp != 0 {
        v = (read_i16(comp + competition::REPUTATION) as i32).wrapping_add(8).wrapping_mul(attendance);
    } else {
        let home = read_usize(partida + fixture::HOME_CLUB);
        crate::comparison::branch("cqb_1 price by injury_word_0c");
        let price = div_trunc(read_i16(home + club::REPUTATION) as i32, 500); // cqb1: the price by reputation
        let friendly = comp != 0 && {
            exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0
        };
        if comp == 0 || friendly {
            v = ftol(price as f64 * read_f64(rdata::F64_0_5) * attendance as f64);
        } else if read_u8(home + club::PROFESSIONAL_STATUS) == 1 || read_u8(read_usize(partida + fixture::AWAY_CLUB) + 0x64) == 1 {
            v = price.wrapping_add(4).wrapping_mul(attendance);
        } else {
            v = price.wrapping_add(3).wrapping_mul(attendance);
        }
        if fin != 0 && read_usize(home + club::DIVISION) == comp {
            let desconto = *((fin + 0x160) as *const f32) as f64;
            v = ftol(v as f64 - desconto * v as f64);
        }
    }
    let home = read_usize(partida + fixture::HOME_CLUB);
    let nation = read_usize(home + club::NATION);
    let kind = if nation != 0 { read_u8(nation + nation::STATE_OF_DEVELOPMENT) as i8 } else { 3 };
    if kind == 2 {
        v = div_trunc(v, 4);
    } else if kind != 1 {
        v = div_trunc(v, 10);
    }
    let league = read_usize(home + club::DIVISION);
    let k = if league != 0 && read_i32(league) == read_i32(KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        Some(0x968928)
    } else {
        let id = read_i32(home);
        if id == read_i32(G_KEY_CLUB_CHELSEA) {
            Some(0x968940)
        } else if id == read_i32(G_KEY_CLUB_ARSENAL) {
            Some(0x968de8)
        } else if id == read_i32(G_KEY_CLUB_EVERTON) {
            Some(0x969040)
        } else if id == read_i32(G_KEY_CLUB_LIVERPOOL) {
            Some(0x9689d8)
        } else if id == read_i32(G_KEY_CLUB_TOTTENHAM_HOTSPUR) {
            Some(0x968de8)
        } else {
            None
        }
    };
    if let Some(k) = k {
        v = ftol(v as f64 * read_f64(k));
    }
    if nation != 0 && read_i32(nation) == read_i32(KEY_NATION_GERMANY) {
        v = ftol(v as f64 * read_f64(rdata::F64_1_2));
    }
    v
}

/// adds the fine to the season (+0x128) and month (+0x88) fines and takes it from the balance
/// Original: `Finance::pay_league_fine` (0x599850, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x599850 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_league_fine(valor)`, pelas multas da liga (0x49f570,
/// 0x6a9e60): soma o valor às multas do ano (+0x128) e do mês (+0x88) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_league_fine(this: usize, value: i32) {
    fin_prm_add_i32(this + finance::SEASON_FINES, value);
    fin_prm_add_i32(this + finance::MONTH_FINES, value);
    add_to_balance(this, -(value as i64));
}

/// Returns the finance i64 balance (+0) split as (low u32, high i32).
///
/// [pt] O saldo (i64 em +0) separado em (baixo, alto), como a original o lê em dois dwords.
pub(crate) unsafe fn balance_halves(fin: usize) -> (u32, i32) {
    (core::ptr::read_unaligned(fin as *const u32), core::ptr::read_unaligned((fin + 4) as *const i32))
}

/// Adds v to finance fields +0x154 and +0xb4 and to the i64 balance.
///
/// [pt] O valor entra em +0x154, +0xb4 e no saldo (estendido com sinal).
pub(crate) unsafe fn credit_income(fin: usize, v: i32) {
    add_to_i32(fin + finance::SEASON_OTHER_INCOME, v);
    add_to_i32(fin + finance::MONTH_OTHER_INCOME, v);
    credit_balance(fin, v);
}

/// True if the club's nation is Sweden, Norway or Denmark (event value drops to one third).
///
/// [pt] A nação do clube é uma das de 0x9cf4a8, 0x9cf408 ou 0x9cf2c0 (o valor do evento cai a um terço).
pub(crate) unsafe fn is_scandinavian_club(club: Club) -> bool {
    let nation = club.nation().addr();
    if nation == 0 {
        return false;
    }
    let n = read_i32(nation);
    n == read_i32(KEY_NATION_SWEDEN) || n == read_i32(KEY_NATION_NORWAY) || n == read_i32(KEY_NATION_DENMARK)
}

/// Tries a cash boost for the club: outside seven nations and flags +0x64 2 and 3, the divisor by wealth band and
/// reputation; with the chairman record, three random draws (against +0xf, the debt and 750 × +0x20) give (3 × (+0x20)²
/// + 750) × reputation + 1 million plus the balance; without it, two draws give (reputation + 1000) × 1000; a third in
/// three nations; then the money, the news by type and the end of the crisis.
///
/// `FinanceTable::try_finance_boost_event(club, finances)`, called from the week's events pass (`process_club_finance_events`). Outside
/// Wales, the Irelands and the nations of 0x9cf408, 0x9cf2c0, 0x9cf4a8 and 0x9cf244, and without flag +0x64 at 2 or 3.
/// The divisor by the band (`get_wealth_band`): rich 0.001; 0 by reputation (up to 4250: 0.0125 with the chairman +0xbf of +0x57
/// and the record +0x69 of +0x20 above 15, otherwise 0.0075; up to 6250: 0.05 or 0.025 the same way; above: 0.025); −1:
/// 0.1 above 5250, 0.075 above 3250, 0.025; −2: 0.125 above 3750, 0.075. With the chairman's record: random(its +0xf)
/// above random(30 / divisor), random(+0x20 × 1 million) above the debt and reputation below 750 × +0x20: (3 × (+0x20)²
/// + 750) × reputation + 1 million, plus the low balance (less if the balance is negative), with the ceiling of
/// reputation² + 1.5 million below 2000. Without it: random(15000 / divisor) below the reputation and random(8) zero:
/// (reputation + 1000) × 1000. A third in the nations of 0x9cf4a8, 0x9cf408 and 0x9cf2c0. Into +0x154, +0xb4 and the
/// balance; the news (`exe::finance_post_finance_boost_news`) by type (by the budget 0x598800, the flag +0x82 and random draws of 4: 3, 5, 6, 7 with
/// +0x82 = 1, or 1); with the record, its +0x20 drops by 1 above 5; ends the crisis (+0x165 = 0); returns 1.
/// Original: `FinanceTable::try_finance_boost_event` (0x59e050, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59e050 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::try_finance_boost_event(clube, finanças)`, pelo passe dos
/// eventos da semana (0x59ed70). Fora do País de Gales, das Irlandas e das nações de 0x9cf408, 0x9cf2c0, 0x9cf4a8 e
/// 0x9cf244, e sem a marca +0x64 em 2 ou 3. O divisor pela faixa (0x5986b0): ricos 0,001; 0 pela reputação (até 4250:
/// 0,0125 com o presidente +0xbf de +0x57 e a ficha +0x69 de +0x20 acima de 15, senão 0,0075; até 6250: 0,05 ou 0,025
/// do mesmo jeito; acima: 0,025); −1: 0,1 acima de 5250, 0,075 acima de 3250, 0,025; −2: 0,125 acima de 3750, 0,075. Com
/// a ficha do presidente: sorteio(+0xf dela) acima de sorteio(30 / divisor), sorteio(+0x20 × 1 milhão) acima da dívida
/// e reputação abaixo de 750 × +0x20: (3 × (+0x20)² + 750) × reputação + 1 milhão, mais o saldo baixo (menos se o saldo
/// é negativo), com o teto de reputação² + 1,5 milhão abaixo de 2000. Sem ela: sorteio(15000 / divisor) abaixo da
/// reputação e sorteio(8) zero: (reputação + 1000) × 1000. Um terço nas nações de 0x9cf4a8, 0x9cf408 e 0x9cf2c0. Em
/// +0x154, +0xb4 e no saldo; a notícia (0x59f300) pelo tipo (pela verba 0x598800, a marca +0x82 e sorteios de 4: 3, 5,
/// 6, 7 com +0x82 = 1, ou 1); com a ficha, o +0x20 dela cai 1 acima de 5; encerra a crise (+0x165 = 0); devolve 1.
pub(crate) unsafe extern "thiscall" fn try_finance_boost_event(this: usize, club: usize, fin: usize) -> u32 {
    let club = Club(club);
    if !club.is_null() {
        let nation = club.nation().addr();
        if nation != 0 {
            let n = read_i32(nation);
            if [KEY_NATION_WALES, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_NORWAY, KEY_NATION_DENMARK, KEY_NATION_SWEDEN, KEY_NATION_BELGIUM].iter().any(|&g| n == read_i32(g)) {
                return 0;
            }
            let m = club.professional_status();
            if m == 2 || m == 3 {
                return 0;
            }
        }
    }
    let rep = club.reputation();
    let presence = Staff(club.chairman().addr());
    let strong = |a: usize, b: usize| a != 0 && b != 0; // (pointers checked before the bytes)
    let div = match wealth_band(fin) {
        -2 => read_f64(if rep > 0xea6 { 0x968fd0 } else { 0x96a730 }),
        -1 => read_f64(if rep > 0x1482 { 0x967880 } else if rep > 0xcb2 { 0x96a730 } else { 0x967878 }),
        0 => {
            if rep <= 0x109a {
                let sheet = if !presence.is_null() { presence.non_player() } else { 0 };
                if strong(presence.addr(), sheet) && presence.ambition() > 0xf && read_i8(sheet + player_sheet::BRAVERY) > 0xf {
                    read_f64(rdata::F64_0_0125)
                } else {
                    crate::comparison::branch("cqb_2 low band 0 divisor");
                    read_f64(rdata::F64_0_0075) // cqb2: the divisor of low-reputation band 0;
                }
            } else if rep < 0x186a {
                let sheet = if !presence.is_null() { presence.non_player() } else { 0 };
                if strong(presence.addr(), sheet) && read_i8(sheet + player_sheet::BRAVERY) > 0xf && presence.ambition() > 0xf {
                    read_f64(rdata::F64_0_05)
                } else {
                    read_f64(rdata::F64_0_025)
                }
            } else {
                read_f64(rdata::F64_0_025)
            }
        }
        _ => read_f64(rdata::F64_0_001),
    };
    let sheet = if !presence.is_null() { presence.non_player() } else { 0 };
    let value;
    if !presence.is_null() && sheet != 0 {
        crate::comparison::branch("cqb_2 with the record");
        let r1 = friendly_random(ftol(read_f64(rdata::F64_30) / div)); // cqb2: the random draw with the player sheet;
        let r2 = friendly_random(read_i8(sheet + player_sheet::GOALKEEPER) as i32);
        if r2 <= r1 {
            return 0;
        }
        let (lo, hi) = balance_halves(fin);
        let n = read_i8(read_usize(club.chairman().addr() + 0x69) + 0x20) as i32;
        let r3 = friendly_random(n.wrapping_mul(1_000_000)) as i64;
        let divida = ((hi as i64) << 32 | lo as i64).wrapping_neg();
        if r3 <= divida {
            return 0;
        }
        let rep = rep as i32;
        let n = read_i8(read_usize(club.chairman().addr() + 0x69) + 0x20) as i32;
        if rep >= n.wrapping_mul(750) {
            return 0;
        }
        // simplification: the original does pow(+0x20, 2.0) (0x945ff0); the square of a byte is exact in both
        let mut v = ftol(((n as f64 * n as f64) * read_f64(rdata::F64_3) + read_f64(rdata::F64_750)) * rep as f64 + read_f64(rdata::F64_1000000));
        v = if hi < 0 { v.wrapping_sub(lo as i32) } else { v.wrapping_add(lo as i32) };
        if club.reputation() < 0x7d0 {
            let ceiling = rep.wrapping_mul(rep).wrapping_add(0x16e360);
            if v > ceiling {
                v = ceiling;
            }
        }
        if is_scandinavian_club(club) {
            v /= 3;
        }
        value = v;
    } else {
        crate::comparison::branch("cqb_2 without the record");
        let r = friendly_random(ftol(read_f64(rdata::F64_15000) / div)); // cqb2: the random draw without the player sheet;
        if rep as i32 <= r {
            return 0;
        }
        if friendly_random(8) != 0 {
            return 0;
        }
        let mut v = (club.reputation() as i32).wrapping_add(1000).wrapping_mul(1000);
        if is_scandinavian_club(club) {
            v /= 3;
        }
        value = v;
    }
    credit_income(fin, value);
    let budget = transfer_budget(fin) as i32;
    let mark = club.plc();
    let with_sheet = !presence.is_null() && sheet != 0;
    let mut kind = 0u32;
    if budget <= 0 && mark == 0 {
        kind = 3;
    } else if mark != 0 && friendly_random(4) == 0 {
        kind = 5;
    }
    if kind == 0 {
        if friendly_random(4) == 0 {
            if with_sheet {
                let r2 = club.reputation() as i32;
                let t = ftol(r2.wrapping_mul(r2) as f64 * read_f64(rdata::F64_0_3));
                if value < t && value > 50000 {
                    kind = 6;
                }
            } else {
                kind = 6;
            }
        }
    }
    if kind == 0 {
        let nation = club.nation().addr();
        if club.plc() == 0 && club.reputation() > 0x128e && friendly_random(4) == 0
            && (!with_sheet || (nation != 0 && read_i32(nation) != read_i32(KEY_NATION_ITALY)))
        {
            post_finance_boost_news(this, club, value, budget as u32, 7);
            club.set_plc(1);
            kind = 7;
        } else {
            kind = 1;
        }
    }
    if kind != 7 {
        post_finance_boost_news(this, club, value, budget as u32, kind);
    }
    if with_sheet {
        let f = read_usize(club.chairman().addr() + 0x69);
        let x = read_i8(f + non_player::RESOURCES);
        if x > 5 {
            *((f + non_player::RESOURCES) as *mut i8) = x - 1;
        }
    }
    *((fin + finance::CRISIS_STATE) as *mut u8) = 0;
    1
}

/// The comparer of the rare finance events, as a dry run (model RARE_FINANCE_EVENT_DRY_RUN: the copy without the news
/// item or the budget, which allocate; without the chairman change in `try_board_takeover`).
///
/// The copy and the original start from the same state (the finance record, the random draw and the `extras`). With no
/// event: everything equal; with an event: the return value, the balance, +0xb4, +0x154, +0x165 and the `conferir`
/// regions (the news type and the +0x82 mark depend on the budget, and so do the random draws after it).
///
/// [pt] O comparador dos eventos raros, em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN: a cópia sem a notícia nem a verba, que alocam; sem a
/// troca do presidente na 0x59e940): a cópia e a original a partir do mesmo estado (o registro de finanças, o sorteio e
/// os `extras`). Sem evento: tudo igual; com evento: a devolução, o saldo, +0xb4, +0x154, +0x165 e os `conferir` (a
/// tipo da notícia e a marca +0x82 dependem da verba, e os sorteios depois dela também).
pub(crate) unsafe fn compare_rare_finance_event(name: &str, fin: usize, extras: &[(usize, usize)], check: &[usize], original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let mut regions = vec![(fin, 0x167)];
    regions.extend(crate::rng::STATE_REGIONS);
    regions.extend_from_slice(extras);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    RARE_FINANCE_EVENT_DRY_RUN = true;
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(copy));
    RARE_FINANCE_EVENT_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let same = if r == 0 && n == 0 {
        mo == mr
    } else {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let fields = r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165];
        fields && check.iter().all(|&k| mo.0[3 + k].1 == mr.0[3 + k].1)
    };
    SCOREBOARD.record(same, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", name, fin, r, mo, n, mr));
    r
}

/// Tries a board takeover of an indebted club: the divisor by band and the factor by reputation and league; random
/// draws by the chairman record and by 15000 / divisor; (reputation + 1000) × 1000 less the balance (or just what
/// clears the balance); with the record, the new chairman (drawn nation, names and record); the news (type 2) and the
/// end of the crisis.
///
/// `FinanceTable::try_board_takeover(club, finances)`, called from the week's events pass (`process_club_finance_events`, clubs in debt).
/// The divisor by the band (`get_wealth_band`: −2: 0.4 above reputation 4250, otherwise 0.25; −1: 0.2 or 0.05; others: 0.001)
/// and the factor (reputation × 0.002 / the +0x69 of league +0x57, at least 0.75; without a league 0.5). With the
/// chairman's (+0xbf) record (+0x69): random(its +0x16) zero; without it: random(20) zero. Then random(15000 / divisor)
/// below reputation × factor: (reputation + 1000) × 1000 minus the low balance; above 40 million, with the reputation
/// below random(2500) + 2000, only what clears the balance. Into +0xb4, +0x154 and the balance; with the record, the
/// new chairman: the nation (the club's if random(1000) ≠ 0, otherwise a drawn one), the names (`database_pick_first_name_for_nation`, `free_record`,
/// `database_pick_second_name_for_nation`) and the record (+0x1d, +0xf, +0x16, +0x20 = random(20) + 1; +0x20 drawn again if it came out below 5); the
/// news (`exe::finance_post_finance_boost_news`, type 2, with the budget 0x598800); ends the crisis (+0x165 = 0); returns 1.
/// Original: `FinanceTable::try_board_takeover` (0x59e940, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59e940 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::try_board_takeover(clube, finanças)`, pelo passe dos
/// eventos da semana (0x59ed70, clubes endividados). O divisor pela faixa (0x5986b0: −2: 0,4 acima de reputação 4250,
/// senão 0,25; −1: 0,2 ou 0,05; outras: 0,001) e o fator (reputação × 0,002 / o +0x69 da liga +0x57, no mínimo 0,75;
/// sem liga 0,5). Com a ficha (+0x69) do presidente (+0xbf): sorteio(+0x16 dela) zero; sem ela: sorteio(20) zero. Depois
/// sorteio(15000 / divisor) abaixo de reputação × fator: (reputação + 1000) × 1000 menos o saldo baixo; acima de 40
/// milhões, com a reputação abaixo de sorteio(2500) + 2000, só o que zera o saldo. Em +0xb4, +0x154 e no saldo; com a
/// ficha, o presidente novo: a nação (a do clube se sorteio(1000) ≠ 0, senão uma sorteada), os nomes (0x539c60,
/// 0x53a5b0, 0x53a1b0) e a ficha (+0x1d, +0xf, +0x16, +0x20 = sorteio(20) + 1; o +0x20 de novo se ficou abaixo de 5); a
/// notícia (0x59f300, tipo 2, com a verba 0x598800); encerra a crise (+0x165 = 0); devolve 1.
pub(crate) unsafe extern "thiscall" fn try_board_takeover(this: usize, club: usize, fin: usize) -> u32 {
    let club = Club(club);
    let rep = club.reputation();
    let div = match wealth_band(fin) {
        -2 => if rep > 0x109a { 0.4f64 } else { 0.25 },
        -1 => if rep > 0x109a { 0.2 } else { 0.05 },
        _ => 0.001,
    };
    let league = club.division().addr();
    let mut factor = if league != 0 {
        let k = rep as i32 as f64 * read_f64(rdata::F64_0_002) / read_i16(league + competition::REPUTATION) as i32 as f64;
        if !(k >= read_f64(rdata::F64_0_75)) { read_f64(rdata::F64_0_75) } else { k } // fcomp: smaller or unordered (NaN) becomes 0.75;
    } else {
        0.5
    };
    let presence = club.chairman().addr();
    let sheet = if presence != 0 { read_usize(presence + staff::NON_PLAYER) } else { 0 };
    let with_sheet = presence != 0 && sheet != 0;
    let first = if with_sheet { friendly_random(read_i8(sheet + player_sheet::WING_BACK) as i32) } else { friendly_random(0x14) };
    if first != 0 {
        return 0;
    }
    factor *= club.reputation() as i32 as f64;
    let r = friendly_random(ftol(read_f64(rdata::F64_15000) / div));
    if r as f64 >= factor {
        return 0;
    }
    crate::comparison::branch("cqb_2 chairman change");
    let (lo, _) = balance_halves(fin);
    let mut v = (club.reputation() as i32).wrapping_add(1000).wrapping_mul(1000).wrapping_sub(lo as i32);
    if v > 0x2625a00 {
        let s = friendly_random(0x9c4);
        if (club.reputation() as i32) < s.wrapping_add(0x7d0) {
            v = (balance_halves(fin).0 as i32).wrapping_neg();
        }
    }
    add_to_i32(fin + finance::MONTH_OTHER_INCOME, v);
    add_to_i32(fin + finance::SEASON_OTHER_INCOME, v);
    credit_balance(fin, v);
    let budget = transfer_budget(fin);
    if with_sheet && !RARE_FINANCE_EVENT_DRY_RUN {
        let presence = club.chairman().addr();
        let nation = club.nation().addr();
        let new = if nation != 0 && friendly_random(0x3e8) != 0 {
            nation
        } else {
            let k = friendly_random(read_i32(G_NATION_COUNT));
            read_usize(G_NATION_TABLE).wrapping_add((k as usize).wrapping_mul(nation::SIZE))
        };
        core::ptr::write_unaligned((presence + staff::NATION) as *mut usize, new);
        // 0x539c60, 0x53a5b0, 0x53a1b0: push nation; call (add esp, 0xc after the three) -> cdecl(nation)
        let x = (exe::database_pick_first_name_for_nation(read_usize(club.chairman().addr() + 0x1a)) as u32);
        core::ptr::write_unaligned((club.chairman().addr() + 4) as *mut u32, x);
        let x = (exe::database_get_empty_common_name(read_usize(club.chairman().addr() + 0x1a)) as u32);
        core::ptr::write_unaligned((club.chairman().addr() + 0xc) as *mut u32, x);
        let x = (exe::database_pick_second_name_for_nation(read_usize(club.chairman().addr() + 0x1a)) as u32);
        core::ptr::write_unaligned((club.chairman().addr() + 8) as *mut u32, x);
        if read_usize(club.chairman().addr() + 0x69) != 0 {
            for o in [0x1dusize, 0xf, 0x16, 0x20] {
                let x = (friendly_random(0x14) as u8).wrapping_add(1);
                *((read_usize(club.chairman().addr() + 0x69) + o) as *mut u8) = x;
            }
            if read_i8(read_usize(club.chairman().addr() + 0x69) + 0x20) < 5 {
                let x = (friendly_random(0x14) as u8).wrapping_add(1);
                *((read_usize(club.chairman().addr() + 0x69) + 0x20) as *mut u8) = x;
            }
        }
    }
    post_finance_boost_news(this, club, v, budget, 2);
    *((fin + finance::CRISIS_STATE) as *mut u8) = 0;
    1
}

/// adds the amount to the season (+0x120) and month (+0x80) other costs and takes it from the balance
/// Original: `Finance::pay_other_costs` (0x5999e0, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x5999e0 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_other_costs(valor)`, pela 0x7e5b10: soma o valor aos
/// outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_other_costs(this: usize, value: i32) {
    add_to_i32(this + finance::SEASON_OTHER_COSTS, value);
    add_to_i32(this + finance::MONTH_OTHER_COSTS, value);
    add_to_balance(this, -(value as i64));
}

/// Recomputes the wage bill: sums the squad wages (less the other club's share for loanees) and the staff wages into
/// the wage bill (+0xc) and the highest wage (+0x10); with fewer than 15 players, the missing places at a wage by
/// reputation; plus reputation² × a factor by club type and nation; the wage ceiling (+0x14) by `get_max_wage`.
///
/// `Finance::update_wage_bill()`, called from the daily finance pass (0x59b9c0, on Saturdays, clubs with a nation),
/// contract signing (`sign_contract`), the daily contracts pass (0x4c7ac0), loans (`sign_loan_contract`, `end_loan`) and the wage bills at
/// creation (`refresh_all_wage_bills`). Clears the highest wage (+0x10) and the bill (+0xc) and sums the wages (the contract's +0xc) of
/// the 50 squad slots (+0xd7; contract with the club `exe::contract_manager_get_contract_for_club`; wage −1: the default wage `exe::staff_contract_default_staff_contract` truncated; on loan
/// (`contract_record`) outside type 6: minus the other contract's wage) and of the staff (+0x19f × 5, +0x1b3 × 7, +0x1cf × 3,
/// contract `exe::contract_manager_get_contract`). Squad with fewer than 15 slots filled: the missing ones at a wage by reputation (+0x80: above
/// 4750, rep − 1950; above 2250, (rep − 2250) × 0.5 + 550; otherwise rep × 0.25; outside type 1 (+0x64), × 0.5; divided
/// by the nation type (+0x7e, between 1 and 3)). Plus reputation² × k (type 1: England and Italy 0.001 / 0.0008 /
/// 0.0006 above 5250 / 3250; others 0.0004; outside type 1: 0.0002 above 3250, otherwise 0.0001), truncated. The wage
/// ceiling (+0x14) is redone by Finance::get_max_wage (`get_max_wage`, 0, −1).
/// Original: `Finance::update_wage_bill` (0x598370, finance.cpp, thiscall).
///
/// [pt] { 0x598370 } (thiscall, finance.cpp): `Finance::update_wage_bill()`, pelo passe diário das finanças (0x59b9c0, no
/// sábado, clube com nação), pela assinatura de contrato (0x4cf720), pelo passe diário dos contratos (0x4c7ac0), pelo
/// empréstimo (0x4d01e0, 0x4d07e0) e pelas folhas da criação (0x59cf50). Zera o maior salário (+0x10) e a folha (+0xc)
/// e soma os salários (+0xc do contrato) das 50 vagas do elenco (+0xd7; contrato com o clube 0x4cf5f0; salário −1: o
/// salário padrão 0x864550 truncado; emprestado (0x4cf6d0) fora do tipo 6: menos o salário do outro contrato) e do
/// pessoal (+0x19f × 5, +0x1b3 × 7, +0x1cf × 3, contrato 0x4cf5a0). Elenco com menos de 15 vagas ocupadas: as que
/// faltam a um salário pela reputação (+0x80: acima de 4750, rep − 1950; acima de 2250, (rep − 2250) × 0,5 + 550; senão
/// rep × 0,25; fora do tipo 1 (+0x64), × 0,5; dividido pelo tipo da nação (+0x7e, entre 1 e 3)). Mais reputação² × k
/// (tipo 1: Inglaterra e Itália 0,001 / 0,0008 / 0,0006 acima de 5250 / 3250; outras 0,0004; fora do tipo 1: 0,0002
/// acima de 3250, senão 0,0001), truncado. O teto salarial (+0x14) refeito por Finance::get_max_wage (0x5968d0, 0, −1).
pub(crate) unsafe extern "thiscall" fn update_wage_bill(this: usize) {
    let ceiling: unsafe extern "thiscall" fn(usize, u32, i32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    write_i32(this + finance::HIGHEST_WAGE, 0);
    write_i32(this + league::STAGES, 0);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + finance::CLUB_ID).wrapping_mul(club::SIZE)));
    let mut ocupadas: i8 = 0;
    for k in 0..50usize {
        let p = read_usize(club.addr() + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        ocupadas = ocupadas.wrapping_add(1);
        let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
        if c == 0 {
            continue;
        }
        let mut w = read_i32(c + contract::WAGE);
        if w == -1 {
            w = ftol(exe::staff_contract_default_staff_contract(c, p, club, 0, (-1) as i32 as u32, 0xb, 0));
        }
        if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 6 {
            w = w.wrapping_sub(read_i32(exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) + 0xc)); // the share of the other contract
        }
        if w > read_i32(this + finance::HIGHEST_WAGE) {
            write_i32(this + finance::HIGHEST_WAGE, w);
        }
        write_i32(this + league::STAGES, read_i32(this + league::STAGES).wrapping_add(w));
    }
    // the staff: the sum goes to the payroll after the three lists (the same sum, in 32 bits)
    let mut staff = 0i32;
    for (base, n) in [(0x19fusize, 5usize), (0x1b3, 7), (0x1cf, 3)] {
        for k in 0..n {
            let p = read_usize(club.addr() + base + 4 * k);
            if p == 0 {
                continue;
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            if c == 0 {
                continue;
            }
            let w = read_i32(c + contract::WAGE);
            if w > read_i32(this + finance::HIGHEST_WAGE) {
                write_i32(this + finance::HIGHEST_WAGE, w);
            }
            staff = staff.wrapping_add(w);
        }
    }
    if staff != 0 {
        crate::comparison::branch("cpz_11 staff payroll");
        write_i32(this + league::STAGES, read_i32(this + league::STAGES).wrapping_add(staff)); // cpz11: the staff on the payroll
    }
    let tipo1 = club.professional_status() == 1;
    let rep = club.reputation() as i32;
    if ocupadas < 15 {
        let mut v = if rep > 0x128e {
            rep - 0x79e
        } else if rep > 0x8ca {
            ftol((rep - 0x8ca) as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_550))
        } else {
            ftol(rep as f64 * read_f64(rdata::F64_0_25))
        };
        if !tipo1 {
            v = ftol(v as f64 * read_f64(rdata::F64_0_5));
        }
        let nation = club.nation().addr();
        if nation != 0 {
            let t = (read_u8(nation + nation::STATE_OF_DEVELOPMENT) as i8).clamp(1, 3) as i32;
            v = v.wrapping_div(t);
        }
        let foul = 15 - ocupadas as i32;
        write_i32(this + league::STAGES, read_i32(this + league::STAGES).wrapping_add(foul.wrapping_mul(v)));
    }
    let k = if tipo1 {
        let nation = club.nation().addr();
        let grande = nation != 0 && (read_i32(nation) == read_i32(KEY_NATION_ENGLAND) || read_i32(nation) == read_i32(KEY_NATION_ITALY));
        if !grande {
            0x96a608
        } else if rep > 0x1482 {
            0x96a350
        } else if rep > 0xcb2 {
            0x96a618
        } else {
            0x96a610
        }
    } else if rep > 0xcb2 {
        0x96a600
    } else {
        0x96a5c0
    };
    crate::comparison::branch("cpz_11 payroll by injury_word_0c");
    let folha = ftol(rep.wrapping_mul(rep) as f64 * read_f64(k) + read_i32(this + league::STAGES) as f64); // cpz11: the reputation on the payroll
    write_i32(this + league::STAGES, folha);
    let t = ceiling(this, 0, -1);
    write_i32(this + finance::WAGE_CEILING, t);
}

/// at game creation, the wage bill of every club up to the first national team
/// Original: `FinanceTable::refresh_all_wage_bills` (0x59cf50, finance.cpp, thiscall).
///
/// [pt] { 0x59cf50 } (thiscall, finance.cpp): `FinanceTable::refresh_all_wage_bills()`, só pela criação do jogo (0x828110):
/// para cada clube de [0xae23b4] até a primeira seleção (0x53b900), a folha (0x598370) do registro de finanças dele
/// ([this] + i × 0x167).
pub(crate) unsafe extern "thiscall" fn refresh_all_wage_bills(this: usize) {
    let mut fin = read_usize(this);
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let club = read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE));
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        exe::finance_update_wage_bill(fin);
        fin = fin.wrapping_add(0x167);
        i += 1;
    }
}

/// Returns the days until the season start of the club's nation, inside the season ticket window (up to two months
/// before); flags whether the season year has come.
///
/// `finance_get_days_to_season_start(club, &reached)`, called from the daily finance pass (0x59b9c0; ecx holds the
/// table, which it does not read): days until the start of the season of the club's nation (72-byte row at 0xb63d60 via
/// `key_nation_index_of_club`; without a row, the "Error" box at line 0x1185 and 0). `reached` = 1 if today's year reaches the season
/// year (2002 with +0x17, otherwise the year of `[0xdd6ec8]` + 1). Start month (+0x34) below 2: in the start year
/// (+0x2c), only after month 12 − start month; in another year, only before the start month; otherwise, only 0..2
/// months before the start. Inside the window, the days until the start date (+0x2a, `exe::date_days_since`) if positive; outside, 0.
/// Original: `finance_get_days_to_season_start` (0x59b4c0, finance.cpp, thiscall, ret 8 (o ecx não é lido)).
///
/// [pt] { 0x59b4c0 } (ret 8, finance.cpp): `finance_get_days_to_season_start(clube, &passou)`, pelo passe diário das
/// finanças (0x59b9c0; ecx com a tabela, que ela não lê): dias até o começo da temporada da nação do clube (linha de
/// 72 bytes em 0xb63d60 pela 0x669360; sem linha, a caixa "Error" da linha 0x1185 e 0). `passou` = 1 se o ano de hoje
/// chega ao ano da temporada (2002 com +0x17, senão o ano de [0xdd6ec8] + 1). Mês do começo (+0x34) abaixo de 2: no ano
/// do começo (+0x2c), só depois do mês 12 − mês do começo; noutro ano, só antes do mês do começo; senão, só a 0..2
/// meses do começo. Na janela, os dias até a data do começo (+0x2a, 0x54cdb0) se positivos; fora, 0.
pub(crate) unsafe extern "thiscall" fn finance_get_days_to_season_start(_this: usize, club: usize, passed: *mut u32) -> i32 {
    *passed = 0;
    let k = exe::key_nation_index_of_club(club) as i8;
    if k < 0 {
        show_error_at(0x9c8784, 0x1185);
        return 0;
    }
    let e = 0xb63d60usize.wrapping_add((k as isize * 72) as usize);
    let year: u16 = if read_usize(e + 0x17) != 0 { 0x7d2 } else { (read_i16(G_GAME_START_DATE_YEAR) as u16).wrapping_add(1) };
    if read_i16(G_CURRENT_DATE_YEAR) as u16 >= year {
        *passed = 1;
    }
    let start = read_u8(e + 0x34) as i8;
    let year_of_start = read_i16(e + 0x2c) as u16;
    let month = exe::date_get_month(G_CURRENT_DATE) as i8;
    let window = if start < 2 {
        if year_of_start == read_i16(G_CURRENT_DATE_YEAR) as u16 {
            month as i32 > 12 - start as i32
        } else {
            month < start
        }
    } else {
        let d = start.wrapping_sub(month);
        (0..=2).contains(&d)
    };
    if !window {
        return 0;
    }
    let r = exe::date_days_since(e + 0x2a, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
    if r > 0 {
        crate::comparison::branch("cpz_11 days until the season");
        return r; // cpz11: the days until the season
    }
    0
}

/// Adds v to the i64 at address a.
///
/// [pt] Soma `v` (64 bits) ao i64 em `a` (o add/adc da original).
pub(crate) unsafe fn fin_prm_add_i64(a: usize, v: i64) {
    core::ptr::write_unaligned(a as *mut i64, core::ptr::read_unaligned(a as *const i64).wrapping_add(v));
}

/// Adds v to the i32 at address a.
///
/// [pt] Soma `v` ao i32 em `a`.
pub(crate) unsafe fn fin_prm_add_i32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, core::ptr::read_unaligned(a as *const i32).wrapping_add(v));
}

/// Builds an i64 from low and high dwords.
///
/// [pt] O valor de 64 bits que a original recebe em dois dwords (baixo primeiro).
pub(crate) fn i64_from_halves(low: u32, high: u32) -> i64 {
    ((high as u64) << 32 | low as u64) as i64
}

/// Adds the amount (64 bits) to the sales of the month (+0x78) and of the year (+0x118) and to the balance.
///
/// `Finance::add_players_sold(amount)` (64-bit amount), called from the transfer payment
/// (TRANSFER_MANAGER::pay_fee_and_record_clauses `pay_fee_and_record_clauses` and the instalments 0x8caf.., 0x8cb3..) and from the previous
/// clubs' share (`pay_former_club_fee_cuts`): adds the amount to the sales of the month (+0x78) and of the year (+0x118), in 64 bits, and
/// to the balance (i64 at +0).
/// Original: `Finance::add_players_sold` (0x599800, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x599800 } (thiscall, ret 8, finance.cpp): `Finance::add_players_sold(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0 e as parcelas 0x8caf.., 0x8cb3..) e pela
/// parte dos clubes anteriores (0x8e4f20): soma o valor às vendas do mês (+0x78) e do ano (+0x118), em 64 bits, e ao
/// saldo (i64 em +0).
pub(crate) unsafe extern "thiscall" fn add_players_sold(this: usize, low: u32, high: u32) {
    let v = i64_from_halves(low, high);
    fin_prm_add_i64(this + finance::MONTH_PLAYERS_SOLD, v);
    fin_prm_add_i64(this + finance::SEASON_PLAYERS_SOLD, v);
    if v != 0 {
        crate::comparison::branch("cpz_12 sale received"); // only a nonzero value changes the balance
    }
    fin_prm_add_i64(this, v); // cpz12: the sale enters the balance
}

/// Adds the amount (64 bits) to the purchases of the month (+0x70) and of the year (+0x110) and takes it from the
/// balance.
///
/// `Finance::pay_players_bought(amount)` (64-bit amount), called from the transfer payment (`pay_fee_and_record_clauses`) and the
/// instalments (0x8caf.., 0x8cb3..): adds the amount to the purchases of the month (+0x70) and of the year (+0x110), in
/// 64 bits, and subtracts it from the balance.
/// Original: `Finance::pay_players_bought` (0x599890, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x599890 } (thiscall, ret 8, finance.cpp): `Finance::pay_players_bought(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (0x8ca8a0) e pelas parcelas (0x8caf.., 0x8cb3..): soma o valor às compras do mês (+0x70) e do ano
/// (+0x110), em 64 bits, e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_players_bought(this: usize, low: u32, high: u32) {
    let v = i64_from_halves(low, high);
    fin_prm_add_i64(this + finance::MONTH_PLAYERS_BOUGHT, v);
    fin_prm_add_i64(this + finance::SEASON_PLAYERS_BOUGHT, v);
    if v != 0 {
        crate::comparison::branch("cpz_12 purchase paid"); // only a nonzero value changes the balance
    }
    fin_prm_add_i64(this, v.wrapping_neg()); // cpz12: the purchase leaves the balance
}

/// Adds the amount to the signing-on fees of the month (+0x9c) and of the year (+0x13c) and takes it from the balance.
///
/// `Finance::pay_sign_on_fee(amount)`, called from an accepted contract offer (`complete_accepted_contract_offer`) and from the player's change
/// of club (`exe::transfer_manager_move_player_to_club`): adds the amount to the signing-on fees of the year (+0x13c) and of the month (+0x9c) and
/// subtracts it from the balance (sign-extended).
/// Original: `Finance::pay_sign_on_fee` (0x5998e0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5998e0 } (thiscall, ret 4, finance.cpp): `Finance::pay_sign_on_fee(valor)`, pela oferta de contrato aceita
/// (0x8c38b0) e pela mudança do jogador de clube (0x8c9720): soma o valor às luvas do ano (+0x13c) e do mês (+0x9c) e o
/// tira do saldo (estendido com sinal).
pub(crate) unsafe extern "thiscall" fn pay_sign_on_fee(this: usize, value: i32) {
    fin_prm_add_i32(this + finance::SEASON_SIGN_ON_FEES, value);
    fin_prm_add_i32(this + finance::MONTH_SIGN_ON_FEES, value);
    if value != 0 {
        crate::comparison::branch("cpz_12 signing-on fees paid"); // only a nonzero value changes the balance
    }
    fin_prm_add_i64(this, (value as i64).wrapping_neg()); // cpz12: the signing-on fees leave the balance
}

/// Adds v to the i64 balance at +0 of a finance record.
///
/// [pt] Soma `v` (estendido com sinal: o cdq/adc da original) ao saldo, i64 em +0.
pub(crate) unsafe fn add_to_balance(fin: usize, v: i64) {
    core::ptr::write_unaligned(fin as *mut i64, core::ptr::read_unaligned(fin as *const i64).wrapping_add(v));
}

/// Adds the amount to the prize money of the month (+0xa8) and of the year (+0x148) and to the balance.
///
/// `Finance::add_prize_money(amount)`, called from the cups (Cup::process_fixture `exe::cup_process_fixture`,
/// EuropeanCup::process_fixture `process_fixture`, the stage draws), the end of the leagues (League::pay_prize_money `pay_prize_money`)
/// and the season honours: adds the amount to the prize money of the year (+0x148) and of the month (+0xa8) and to the
/// balance.
/// Original: `Finance::add_prize_money` (0x5999a0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5999a0 } (thiscall, ret 4, finance.cpp): `Finance::add_prize_money(valor)`, pelas copas (Cup::process_fixture
/// 0x51a150, EuropeanCup::process_fixture 0x583b10, os sorteios das fases), pelo fim das ligas (League::pay_prize_money
/// 0x682e10) e pelas honras da temporada: soma o valor aos prêmios do ano (+0x148) e do mês (+0xa8) e ao saldo.
pub(crate) unsafe extern "thiscall" fn add_prize_money(this: usize, value: i32) {
    fin_prm_add_i32(this + finance::SEASON_PRIZE_MONEY, value);
    fin_prm_add_i32(this + finance::MONTH_PRIZE_MONEY, value);
    if value != 0 {
        crate::comparison::branch("cpz_13 prize received"); // only a nonzero value changes the balance
    }
    add_to_balance(this, value as i64); // cpz13: the prize enters the balance
}

/// Adds the amount to the TV income of the month (+0xb8) and of the year (+0x158) and to the balance.
///
/// `Finance::add_tv_revenue(amount)`, called only from the league TV rights (League::pay_tv_rights `pay_tv_rights`, when each
/// league's table is built for the new season): adds the amount to the TV income of the year (+0x158) and of the month
/// (+0xb8) and to the balance.
/// Original: `Finance::add_tv_revenue` (0x5987c0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5987c0 } (thiscall, ret 4, finance.cpp): `Finance::add_tv_revenue(valor)`, só pelos direitos de TV da liga
/// (League::pay_tv_rights 0x683010, pela montagem da tabela de cada liga na temporada nova): soma o valor à TV do ano
/// (+0x158) e do mês (+0xb8) e ao saldo.
pub(crate) unsafe extern "thiscall" fn add_tv_revenue(this: usize, value: i32) {
    fin_prm_add_i32(this + finance::SEASON_TV_REVENUE, value);
    fin_prm_add_i32(this + finance::MONTH_TV_REVENUE, value);
    if value != 0 {
        crate::comparison::branch("cpz_13 TV money received"); // only a nonzero value changes the balance
    }
    add_to_balance(this, value as i64); // cpz13: the TV money enters the balance
}

/// adds the amount to the wages of the month (+0x90) and of the year (+0x130) and takes it from the balance (the free
/// release compensation)
/// Original: `Finance::pay_wages` (0x599920, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599920 } (thiscall, ret 4, finance.cpp): `Finance::pay_wages(valor)`, só pela liberação sem custo
/// (TRANSFER_MANAGER::free_transfer_staff 0x8c1af0: a compensação do contrato ao clube): soma o valor aos salários do ano
/// (+0x130) e do mês (+0x90) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_wages(this: usize, value: i32) {
    fin_prm_add_i32(this + finance::SEASON_WAGES, value);
    fin_prm_add_i32(this + finance::MONTH_WAGES, value);
    if value != 0 {
        crate::comparison::branch("cpz_13 wages paid"); // only a nonzero value changes the balance
    }
    add_to_balance(this, (value as i64).wrapping_neg()); // cpz13: the salary leaves the balance
}

/// Adds v to the i32 at address a (duplicate of fin_prm_add_i32).
pub(crate) unsafe fn add_to_i32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, read_i32(a).wrapping_add(v));
}

/// Adds sign-extended v to the i64 balance at +0 of a finance record (i32 amount).
///
/// [pt] O saldo (i64 em +0 do registro de finanças) mais `v` (estendido com sinal, como o cdq/adc da original).
pub(crate) unsafe fn add_i32_to_balance(fin: usize, v: i32) {
    let s = core::ptr::read_unaligned(fin as *const i64);
    core::ptr::write_unaligned(fin as *mut i64, s.wrapping_add(v as i64));
}

/// Returns the club's finance record (0x167 bytes) in FinanceTable this: [this] + club id × 0x167.
///
/// [pt] O registro de finanças (0x167 bytes) do clube na tabela `this` (FinanceTable), pelo id (+0) do clube.
pub(crate) unsafe fn fin_prm_club_finance(this: usize, club: Club) -> usize {
    read_usize(this).wrapping_add((read_usize(club.addr()) as u32).wrapping_mul(0x167) as usize)
}

/// Adds the weekly merchandising income: attendance × reputation × factor (by the stadium of the +0xbf club or by
/// reputation; Japan one step up), × (random(11) + 95)%, added to +0xbc, +0x15c and the balance.
///
/// `FinanceTable::add_weekly_merchandising_income(club)`, called from the daily finance pass (0x59b9c0, on day 6 of the
/// week, league clubs). The base: with the +0xbf club and its stadium (+0x69), the level (+0xf) × 25 + the reputation
/// (+0x80), × 0.65 truncated if byte +0x6d is zero, picks the factor by 5500 and 4000; without them, the reputation
/// picks it by 5250 and 3750 (one step higher in the nation of Japan 0x9cf36c). Attendance (+0x73) × reputation ×
/// factor, truncated; × (random(11) + 95) × 0.01, truncated, added to +0xbc, +0x15c and the balance.
/// Original: `FinanceTable::add_weekly_merchandising_income` (0x59b7e0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x59b7e0 } (thiscall, ret 4, finance.cpp): `FinanceTable::add_weekly_merchandising_income(clube)`, pelo passe
/// diário das finanças (0x59b9c0, no dia 6 da semana, clubes de liga). A base: com o clube de +0xbf e o estádio dele
/// (+0x69), o nível (+0xf) × 25 + a reputação (+0x80), × 0,65 truncado se o byte +0x6d for zero, escolhe o fator por
/// 5500 e 4000; sem eles, a reputação escolhe por 5250 e 3750 (um degrau acima na nação do Japão 0x9cf36c). Público
/// (+0x73) × reputação × fator, truncado; × (sorteio(11) + 95) × 0,01, truncado, soma em +0xbc, +0x15c e no saldo.
pub(crate) unsafe extern "thiscall" fn add_weekly_merchandising_income(this: usize, club: usize) {
    let club = Club(club);
    let other = Staff(club.chairman().addr());
    let est = if !other.is_null() { other.non_player() } else { 0 };
    let rep = club.reputation() as i32;
    let factor = if est != 0 {
        let s = (read_u8(est + non_player::BUSINESS) as i8 as i32).wrapping_mul(25);
        let mut v = rep.wrapping_add(s);
        if club.own_stadium() == 0 {
            v = ftol(v as f64 * read_f64(rdata::F64_0_65));
        }
        if v > 0x157c {
            0x96a6d0
        } else if v > 0xfa0 {
            0x96a6c8
        } else {
            0x96a6b8
        }
    } else {
        let nation = club.nation().addr();
        let japao = nation != 0 && read_i32(nation) == read_i32(KEY_NATION_JAPAN);
        if rep > 0x1482 {
            if japao { 0x96a6c0 } else { 0x96a6d0 }
        } else if rep > 0xea6 {
            if japao { 0x96a6d0 } else { 0x96a6c8 }
        } else if japao {
            0x96a6c8
        } else {
            0x96a6b8
        }
    };
    let base = ftol(club.attendance() as f64 * rep as f64 * read_f64(factor));
    crate::comparison::branch("cpv_5 merchandising");
    let r = friendly_random(0xb); // cpv5: the merchandising random draw
    let v = ftol((r + 0x5f).wrapping_mul(base) as f64 * read_f64(rdata::F64_0_01));
    let fin = fin_prm_club_finance(this, club);
    add_to_i32(fin + 0x15c, v);
    add_to_i32(fin + 0xbc, v);
    add_i32_to_balance(fin, v);
}

/// Sells season tickets: stadium of at least 1500 places; demand by the manager-table value, attendance and reputation,
/// up to 87.5% of the places; new holders by the step fraction (+0x164) and a random draw; ratio at +0x160; income
/// (with apply) by price, nation type and key clubs; the shop with two random draws by reputation.
///
/// `Finance::sell_season_tickets(apply)`, called from the daily finance pass (0x59b9c0, on day 6 of the week, when
/// finance_get_days_to_season_start `finance_get_days_to_season_start` says the season is still to start). The club by id (+8, 0x245-byte
/// record); a stadium (+0x69) with fewer than 1500 places (+0x3c) uses reputation × 2.5 (minimum 1500). The demand: the
/// club's value in the managers' manager (`[[0xb67a44]]` + 73·id + 8) × 0.0001 (minimum 0.15) × attendance (+0x73), ×
/// reputation × 0.00025 (minimum 0.5), capped at 87.5% of the places (1250 without a stadium). Above those already sold
/// (+0x18): the step (+0x164, up to 8) picks the fraction (0x9c8758) of the difference, × (random(11) + 95) / 100 = the
/// new ones; the sold/places ratio goes to +0x160 (f32, up to 0.875). The income (price by reputation × new ones; /4 or
/// /10 by the nation type +0x7e; × 1.25 in Serie A, Chelsea 1.5, Arsenal and Tottenham 1.3, Everton 0.75, Liverpool
/// 0.9, Germany 1.2) is applied only with `apply`; then (random(rep) + random(rep) + 1) × 0.001 + 1, × new ones, into
/// +0x15c.
/// Original: `Finance::sell_season_tickets` (0x599a20, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599a20 } (thiscall, ret 4, finance.cpp): `Finance::sell_season_tickets(aplicar)`, pelo passe diário das finanças
/// (0x59b9c0, no dia 6 da semana, quando finance_get_days_to_season_start 0x59b4c0 diz que a temporada ainda vai
/// começar). O clube pelo id (+8, registro de 0x245 bytes); estádio (+0x69) com menos de 1500 lugares (+0x3c) passa a
/// reputação × 2,5 (mínimo 1500). A procura: o valor do clube no gerente de técnicos ([[0xb67a44]] + 73·id + 8) × 0,0001
/// (mínimo 0,15) × público (+0x73), × reputação × 0,00025 (mínimo 0,5), limitada a 87,5% dos lugares (1250 sem estádio).
/// Acima dos já vendidos (+0x18): o passo (+0x164, até 8) escolhe a fração (0x9c8758) da diferença, × (sorteio(11) +
/// 95) / 100 = os novos; a razão vendidos/lugares vai a +0x160 (f32, até 0,875). A receita (preço pela reputação × novos;
/// /4 ou /10 pelo tipo da nação +0x7e; × 1,25 na série A, Chelsea 1,5, Arsenal e Tottenham 1,3, Everton 0,75, Liverpool
/// 0,9, Alemanha 1,2) entra só com `aplicar`; depois (sorteio(rep) + sorteio(rep) + 1) × 0,001 + 1, × novos, em +0x15c.
pub(crate) unsafe extern "thiscall" fn sell_season_tickets(this: usize, apply: i32) {
    let id = read_usize(this + finance::CLUB_ID);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE)));
    if club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_SELL_SEASON_TICKETS)(this, apply);
    }
    let rep = club.reputation() as i32;
    let est = club.stadium().addr();
    if est != 0 && read_i32(est + stadium::CAPACITY) < 0x5dc {
        core::ptr::write_unaligned((est + stadium::CAPACITY) as *mut i32, ftol(rep as f64 * read_f64(rdata::F64_2_5)));
        let est = club.stadium().addr();
        if read_i32(est + stadium::CAPACITY) < 0x5dc {
            core::ptr::write_unaligned((est + stadium::CAPACITY) as *mut i32, 0x5dc);
        }
    }
    let manager = read_usize(read_usize(G_MANAGER_MANAGER));
    let value = read_i16(manager.wrapping_add((read_usize(club.addr()) as u32).wrapping_mul(73) as usize) + 8);
    let mut f = value as f64 * read_f64(rdata::F64_0_0001);
    if !(f > read_f64(rdata::F64_0_15)) {
        f = read_f64(rdata::F64_0_15);
    }
    let t = ftol(club.attendance() as f64 * f);
    let mut g = rep as f64 * read_f64(rdata::F64_0_00025);
    if !(g > read_f64(rdata::F64_0_5)) {
        g = read_f64(rdata::F64_0_5);
    }
    let u = ftol(t as f64 * g);
    let est = club.stadium().addr();
    let ceiling = |e: usize| if e != 0 { read_i32(e + 0x3c) as f64 * read_f64(rdata::F64_0_875) } else { read_f64(rdata::F64_1250) };
    let n = ftol(if (u as f64) < ceiling(est) { u as f64 } else { ceiling(est) });
    if n == 0 {
        return;
    }
    let ja = read_i32(this + finance::SEASON_TICKETS_SOLD);
    if ja >= n {
        return;
    }
    let mut step_towards = (read_u8(this + finance::SEASON_TICKET_STEP) as i8).wrapping_add(1);
    if step_towards < 0 || step_towards > 8 {
        step_towards = 8;
    }
    *((this + finance::SEASON_TICKET_STEP) as *mut i8) = step_towards;
    let b = ftol(n.wrapping_sub(ja) as f64 * read_f32(0x9c8758 + step_towards as usize * 4) as f64);
    crate::comparison::branch("cpv_5 tickets sold");
    let r = friendly_random(0xb);
    let new = (r + 0x5f).wrapping_mul(b) / 100;
    let total = ja.wrapping_add(new);
    core::ptr::write_unaligned((this + finance::SEASON_TICKETS_SOLD) as *mut i32, total); // cpv5: os ingressos vendidos
    let price = ftol((rep as f64 * read_f64(rdata::F64_0_002) + read_f64(rdata::F64_4)) * read_f64(rdata::F64_20));
    let mut income = price.wrapping_mul(new);
    let est = club.stadium().addr();
    let places = if est != 0 { read_i32(est + stadium::CAPACITY) } else { 0 };
    let den = if places != 0 { places as f64 * read_f64(rdata::F64_0_875) } else { read_f64(rdata::F64_1250) };
    let ratio = total as f64 / den;
    core::ptr::write_unaligned((this + finance::SEASON_TICKET_RATIO) as *mut f32, ratio as f32);
    if ratio > read_f64(rdata::F64_0_875) {
        core::ptr::write_unaligned((this + finance::SEASON_TICKET_RATIO) as *mut u32, 0x3f600000);
    }
    let nation = club.nation().addr();
    let kind = if nation != 0 { read_u8(nation + nation::STATE_OF_DEVELOPMENT) as i8 } else { 3 };
    match kind {
        1 => {}
        2 => income /= 4,
        _ => income /= 10,
    }
    let league = club.division().addr();
    let cid = read_i32(club.addr());
    let factor = if league != 0 && read_i32(league) == read_i32(KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        0x968928
    } else if cid == read_i32(G_KEY_CLUB_CHELSEA) {
        0x968940
    } else if cid == read_i32(G_KEY_CLUB_ARSENAL) {
        0x968de8
    } else if cid == read_i32(G_KEY_CLUB_EVERTON) {
        0x969040
    } else if cid == read_i32(G_KEY_CLUB_LIVERPOOL) {
        0x9689d8
    } else if cid == read_i32(G_KEY_CLUB_TOTTENHAM_HOTSPUR) {
        0x968de8
    } else if nation != 0 && read_i32(nation) == read_i32(KEY_NATION_GERMANY) {
        0x968978
    } else {
        0
    };
    if factor != 0 {
        income = ftol(income as f64 * read_f64(factor));
    }
    if apply != 0 { // cpv5: the ticket revenue
        add_to_i32(this + finance::MONTH_SEASON_TICKETS, income);
        add_to_i32(this + finance::SEASON_SEASON_TICKETS, income);
        add_i32_to_balance(this, income);
    }
    let r1 = friendly_random(rep);
    let r2 = friendly_random(rep); // cpv5: the second shop random draw
    let k = r1.wrapping_add(r2).wrapping_add(1);
    let v = ftol((k as f64 * read_f64(rdata::F64_0_001) + read_f64(rdata::F64_1)) * new as f64);
    add_to_i32(this + finance::SEASON_MERCHANDISING, v);
    add_to_i32(this + finance::MONTH_MERCHANDISING, v);
    add_i32_to_balance(this, v);
}

/// Charges a works cost to the club's finance record (+0x8c, +0x12c and minus the balance).
///
/// [pt] O custo da obra (+0x8c e +0x12c, menos no saldo) no registro de finanças do clube.
pub(crate) unsafe fn charge_works_cost(this: usize, club: Club, cost: i32) {
    let fin = fin_prm_club_finance(this, club);
    add_to_i32(fin + 0x12c, cost);
    add_to_i32(fin + 0x8c, cost);
    add_i32_to_balance(fin, cost.wrapping_neg());
}

/// Raises the training facilities level (+0x7f) by reputation (up to 20); each step charges the works to +0x8c, +0x12c
/// and the balance.
///
/// `FinanceTable::upgrade_training_facilities(club)`, called from the club's season end (`season_end_club_update`, with the training
/// request approved by the board). The level (+0x7f) goes up by 2 (1 from 19) and charges level × 50000 + 75000. Then,
/// below 20: with reputation (+0x80) above 8250, it goes to 20 and charges (22 − level) × 250000 (minimum 2 million);
/// below 14 with reputation above 5250, it goes up by 4 (minimum 16) and charges 3.5 million − level × 150000 (minimum
/// 1 million); below 16 with reputation above 4250, or below 20 with reputation above 7250: above 7250 it goes to 20,
/// otherwise up by 2 (minimum 12), and charges the new level × 50000 + 75000. Ceiling 20.
/// Original: `FinanceTable::upgrade_training_facilities` (0x59b630, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x59b630 } (thiscall, ret 4, finance.cpp): `FinanceTable::upgrade_training_facilities(clube)`, pelo fim da
/// temporada do clube (0x59cfa0, com o pedido de treino aprovado pela diretoria). O nível (+0x7f) sobe 2 (1 a partir de
/// 19) e cobra nível × 50000 + 75000. Depois, abaixo de 20: com reputação (+0x80) acima de 8250, vai a 20 e cobra
/// (22 − nível) × 250000 (mínimo 2 milhões); abaixo de 14 com reputação acima de 5250, sobe 4 (mínimo 16) e cobra 3,5
/// milhões − nível × 150000 (mínimo 1 milhão); abaixo de 16 com reputação acima de 4250, ou abaixo de 20 com reputação
/// acima de 7250: acima de 7250 vai a 20, senão sobe 2 (mínimo 12), e cobra o novo nível × 50000 + 75000. Teto 20.
pub(crate) unsafe extern "thiscall" fn upgrade_training_facilities(this: usize, club: usize) {
    let club = Club(club);
    let level = (club.addr() + club::TRAINING) as *mut i8;
    let cost = (*level as i32).wrapping_mul(50000).wrapping_add(75000);
    *level = if *level < 0x13 { (*level).wrapping_add(2) } else { (*level).wrapping_add(1) };
    charge_works_cost(this, club, cost);
    let n = *level;
    let rep = club.reputation();
    let second = if n < 0x14 && rep > 0x203a {
        *level = 0x14;
        Some((0x16 - n as i32).wrapping_mul(250000).max(2000000))
    } else if n < 0xe && rep > 0x1482 {
        let c = 3500000i32.wrapping_sub((n as i32).wrapping_mul(150000)).max(1000000);
        *level = n.wrapping_add(4);
        if *level < 0x10 {
            *level = 0x10;
        }
        Some(c)
    } else if (n < 0x10 && rep > 0x109a) || (n < 0x14 && rep > 0x1c52) {
        if rep > 0x1c52 {
            *level = 0x14;
        } else {
            *level = n.wrapping_add(2);
            if *level < 0xc {
                *level = 0xc;
            }
        }
        Some((*level as i32).wrapping_mul(50000).wrapping_add(75000))
    } else {
        None
    };
    if let Some(c) = second {
        charge_works_cost(this, club, c);
    }
    if *level > 0x14 {
        *level = 0x14;
    }
}

/// Returns the i64 balance at +0 of a finance record.
pub(crate) unsafe fn balance(fin: usize) -> i64 {
    core::ptr::read_unaligned(fin as *const i64)
}

/// Adds sign-extended v to the i64 balance at +0 of a finance record (duplicate of add_i32_to_balance).
///
/// [pt] O saldo (i64 em +0) mais `v` estendido com sinal (o cdq/adc da original).
pub(crate) unsafe fn credit_balance(fin: usize, v: i32) {
    core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_add(v as i64));
}

/// Returns the finance record's wealth band (FN_FINANCE_GET_WEALTH_BAND), signed.
pub(crate) unsafe fn wealth_band(fin: usize) -> i8 {
    exe::finance_get_wealth_band(fin) as i8
}

/// Calls a weekly finance event routine (thiscall(table, club, finance)); returns 1 if it happened.
///
/// [pt] Um evento da semana (0x59e050, 0x59dd40, 0x59e5a0, 0x59e940): thiscall(tabela, clube, finanças), 1 se aconteceu.
pub(crate) unsafe fn run_weekly_event(routine: usize, this: usize, club: Club, fin: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(routine);
    f(this, club.addr(), fin)
}

/// The comparer's dry run: the copy runs without the news item (`exe::finance_post_finance_boost_news`) and without the funding (0x598800), which
/// allocate and cannot run twice.
///
/// [pt] O ensaio do comparador: a cópia roda sem a notícia (0x59f300) e sem a verba (0x598800), que alocam e não podem
/// rodar duas vezes.
pub(crate) static mut RARE_FINANCE_EVENT_DRY_RUN: bool = false;

/// Posts the finance event news (FN_FINANCE_POST_FINANCE_BOOST_NEWS); nothing in rehearsal.
///
/// [pt] A notícia do evento (0x59f300: thiscall(tabela, clube, valor, verba, tipo), ret 0x10); no ensaio, nada.
pub(crate) unsafe fn post_finance_boost_news(this: usize, club: Club, value: i32, budget: u32, kind: u32) {
    if RARE_FINANCE_EVENT_DRY_RUN {
        return;
    }
    exe::finance_post_finance_boost_news(this, club, value, budget, kind);
}

/// Transfer budget for the news (Finance::get_transfer_budget 0x598800 with 0, 0, 0, 1, 0); 0 in rehearsal.
///
/// [pt] A verba de transferências (Finance::get_transfer_budget 0x598800 com 0, 0, 0, 1, 0) que vai na notícia; no
/// ensaio, 0 (a notícia não sai).
pub(crate) unsafe fn transfer_budget(fin: usize) -> u32 {
    if RARE_FINANCE_EVENT_DRY_RUN {
        return 0;
    }
    (exe::finance_get_transfer_budget(fin, 0, 0, 0, 1, 0) as u32)
}

/// Posts the end-of-crisis news (`exe::finance_post_receivers_appeased_news`, receivers appeased) for the club id.
///
/// [pt] O fim da semana do clube no estado de crise (0x5a02f0: thiscall(tabela, id)).
pub(crate) unsafe fn post_receivers_appeased(this: usize, club: Club) {
    exe::finance_post_receivers_appeased_news(this, read_usize(club.addr()) as u32);
}

/// Resets the finance state (+0x165) to 0 from 1 or 2; from 1 (crisis) posts the end-of-crisis news first.
///
/// [pt] O estado (+0x165) volta a 0; se era 1 (crise), a notícia do fim da crise (0x5a02f0) antes.
pub(crate) unsafe fn leave_crisis(this: usize, club: Club, fin: usize) {
    let s = read_u8(fin + finance::CRISIS_STATE);
    if s == 1 {
        post_receivers_appeased(this, club);
    }
    if s == 1 || s == 2 {
        *((fin + finance::CRISIS_STATE) as *mut u8) = 0;
    }
}

/// A maxed training ground (club::TRAINING = 20) drops one level on random(10) = 0 if reputation is below 7250.
///
/// [pt] O centro de treino no máximo (20) cai um nível com sorteio(10) zero, se a reputação for menor que 7250.
pub(crate) unsafe fn decay_training_ground(club: Club) {
    let level = (club.addr() + club::TRAINING) as *mut i8;
    if *level == 0x14 && club.reputation() < 0x1c52 && friendly_random(10) == 0 {
        *level = (*level).wrapping_sub(1);
    }
}

/// Processes the club's finance events by the finance band: ends or opens the crisis (+0x165), moves the squeeze
/// counter (+0x166), tries the boost, the debt, the injection and the board takeover, wears the training ground; with
/// squeeze above 50, removes the heating or closes 10% of the places.
///
/// `FinanceTable::process_club_finance_events(club, week)`, called from the daily finance pass (0x59b9c0: at the turn
/// of the week with week = 1 for every league club with reputation 5000 or an active nation; on other days only for the
/// club in crisis, with week = random zero). The club's finances (from partner A, `exe::club_get_linked_b_team`) and the band (`get_wealth_band`): 2
/// and 1 (rich) and 0 end the crisis (+0x165) and lower the squeeze counter (+0x166) to 5/0/10; 1 and 0 try the boost
/// (`try_finance_boost_event`), the debt (`try_board_debt_payment`) and the injection (`try_cash_injection`); −1 sets state 2, raises the squeeze by byte +0x6d,
/// wears the training facilities and tries the debt, the injection and the board takeover (`try_board_takeover`); −2 enters the
/// crisis (news `exe::finance_post_financial_crisis_news`, clears flag +0x82, and at the human club `exe::human_manager_is_human_club` lowers the value of the squad contracts
/// through `calc_player_value`, setting bit 4 of +0x4e) or, already in crisis, tries the takeover, the injection and the boost,
/// wears the facilities and raises the squeeze by 3. Then, with an active league (`[0xadadfc]`) and squeeze above 50:
/// removes the heating (+0x4d, `news_ground_works`) of a club with reputation below 4250, or closes 10% of the places (+0x3c to
/// +0x44, `exe::finance_post_stand_closed_news`, squeeze 20); the training facilities drop by random draw.
/// Original: `FinanceTable::process_club_finance_events` (0x59ed70, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x59ed70 } (thiscall, ret 8, finance.cpp): `FinanceTable::process_club_finance_events(clube, semana)`, pelo passe
/// diário das finanças (0x59b9c0: na virada da semana com semana = 1 para todo clube de liga com reputação de 5000 ou
/// nação ativa; nos outros dias só para o clube em crise, com semana = sorteio zero). As finanças do clube (do parceiro
/// A, 0x540a50) e a faixa (0x5986b0): 2 e 1 (ricos) e 0 encerram a crise (+0x165) e baixam o contador de aperto (+0x166)
/// até 5/0/10; 1 e 0 tentam o impulso (0x59e050), a dívida (0x59dd40) e a injeção (0x59e5a0); −1 marca o estado 2, sobe
/// o aperto pelo byte +0x6d, desgasta o centro de treino e tenta a dívida, a injeção e a troca da diretoria (0x59e940);
/// −2 entra em crise (notícia 0x59fc60, tira a marca +0x82, e no clube humano 0x600a50 baixa o valor dos contratos do
/// elenco pela 0x4d0c60, marcando o bit 4 de +0x4e) ou, já em crise, tenta a troca, a injeção e o impulso, desgasta o
/// centro e sobe o aperto 3. Depois, liga ativa ([0xadadfc]) e aperto acima de 50: tira o aquecimento (+0x4d, 0x59ffb0)
/// do clube de reputação abaixo de 4250, ou fecha 10% dos lugares (+0x3c para +0x44, 0x59f900, aperto 20); o centro de
/// treino cai pelo sorteio.
pub(crate) unsafe extern "thiscall" fn process_club_finance_events(this: usize, club: usize, week: u32) {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_CLUB_FINANCE_EVENTS)(this, club.addr(), week);
    }
    let mut eh_a: u32 = 0;
    let p = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { club.addr() };
    let fin = read_usize(this).wrapping_add((read_usize(c) as u32).wrapping_mul(0x167) as usize);
    let aperto = (fin + 0x166) as *mut i8;
    match wealth_band(fin) {
        -1 => {
            let s = read_u8(fin + 0x165);
            if s != 2 {
                if s == 1 {
                    post_receivers_appeased(this, club);
                }
                *((fin + 0x165) as *mut u8) = 2;
            }
            if week == 0 {
                return;
            }
            if club.own_stadium() != 0 {
                *aperto = (*aperto).wrapping_add(1);
            }
            decay_training_ground(club);
            if run_weekly_event(0x59dd40, this, club, fin) == 0 && run_weekly_event(0x59e5a0, this, club, fin) == 0 {
                run_weekly_event(0x59e940, this, club, fin);
            }
        }
        -2 => {
            if read_u8(fin + 0x165) != 1 {
                exe::finance_post_financial_crisis_news(this, club);
                if club.plc() != 0 {
                    club.set_plc(0);
                }
                *((fin + 0x165) as *mut u8) = 1;
                if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) == 0 {
                    return;
                }
                let mut squad = [0usize; 64];
                exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
                for &j in squad.iter().take(50) {
                    if j == 0 || read_usize(j + staff::PLAYER) == 0 {
                        continue;
                    }
                    let g = read_usize(G_CONTRACT_MANAGER);
                    let k = exe::contract_manager_get_contract(g, j);
                    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = k;
                    let owner = if k != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(club::SIZE)) } else { 0 };
                    let ct = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), j, owner);
                    if owner == club.addr() && ct != 0 {
                        let v = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), j, owner, 0, -1, 1);
                        if v < read_i32(j + staff::VALUE) {
                            *((ct + contract::TRANSFER_STATUS) as *mut u8) |= 4;
                            core::ptr::write_unaligned((j + staff::VALUE) as *mut i32, v);
                        }
                    }
                }
                return;
            }
            if week == 0 {
                return;
            }
            if run_weekly_event(0x59e940, this, club, fin) == 0 && run_weekly_event(0x59e5a0, this, club, fin) == 0 {
                run_weekly_event(0x59e050, this, club, fin);
            }
            decay_training_ground(club);
            if club.own_stadium() != 0 {
                *aperto = (*aperto).wrapping_add(3);
            }
        }
        1 => {
            leave_crisis(this, club, fin);
            if week == 0 {
                return;
            }
            if *aperto > 0 {
                *aperto -= 1;
            }
            if run_weekly_event(0x59e050, this, club, fin) == 0 && run_weekly_event(0x59dd40, this, club, fin) == 0 {
                run_weekly_event(0x59e5a0, this, club, fin);
            }
        }
        2 => {
            leave_crisis(this, club, fin);
            if week == 0 {
                return;
            }
            if *aperto > 5 {
                *aperto -= 1;
            }
        }
        _ => {
            leave_crisis(this, club, fin);
            if week == 0 {
                return;
            }
            if *aperto > 10 {
                *aperto -= 1;
            }
            crate::comparison::branch("cpv_6 boost in band 0");
            let ok = run_weekly_event(0x59e050, this, club, fin) != 0; // cpv6: the boost in band 0
            if !ok && run_weekly_event(0x59dd40, this, club, fin) == 0 {
                run_weekly_event(0x59e5a0, this, club, fin);
            }
        }
    }
    // a crise prolongada
    let league = club.division().addr();
    if league == 0 || read_i32(read_usize(G_COMP_OBJECT_TABLE) + read_usize(league).wrapping_mul(4)) == 0 || *aperto <= 0x32 {
        return;
    }
    let est = club.stadium().addr();
    if est == 0 {
        return;
    }
    let level = (club.addr() + club::TRAINING) as *mut i8;
    if read_u8(est + stadium::UNDER_SOIL_HEATING) != 0 && club.reputation() < 0x109a {
        let aquecimento: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_FINANCE_POST_STADIUM_WORKS_NEWS);
        aquecimento(this, club.addr(), 0, 7);
        *((club.stadium().addr() + 0x4d) as *mut u8) = 0;
        let n = *level;
        if n == 0x14 {
            let r = friendly_random(4) as u8;
            *level = (*level).wrapping_add(0xffu8.wrapping_sub(r) as i8);
            return;
        }
        if n <= 1 || friendly_random(n as i32) == 0 {
            return;
        }
        *level = (*level).wrapping_sub(1);
        return;
    }
    let places = read_i32(est + stadium::CAPACITY);
    if places <= 0xabe || friendly_random(club.reputation() as i32) >= places {
        return;
    }
    let t = ftol(places as f64 * read_f64(rdata::F64_0_1)) as i16 as i32;
    let dez = (t / 10).wrapping_mul(10);
    let close = dez as i16 as i32;
    let rest = read_i32(est + stadium::SEATING_CAPACITY);
    let tirou = if rest > close {
        core::ptr::write_unaligned((est + stadium::SEATING_CAPACITY) as *mut i32, rest - close);
        1
    } else {
        0
    };
    exe::finance_post_stand_closed_news(this, club, dez, tirou);
    let est = club.stadium().addr();
    add_to_i32(est + stadium::CAPACITY, close.wrapping_neg());
    let est = club.stadium().addr();
    add_to_i32(est + stadium::EXPANSION_CAPACITY, close);
    *aperto = 0x14;
    let n = *level;
    if n > 0xf {
        let r = friendly_random(5) as u8;
        *level = (*level).wrapping_add(0xffu8.wrapping_sub(r) as i8);
        return;
    }
    if n <= 7 && (n <= 1 || friendly_random(n as i32) == 0) {
        return;
    }
    *level = (*level).wrapping_sub(1);
}

// simplification: calls originals that roll, allocate news items and touch contracts: original only; the copy takes the branch.

/// Tries a board payment of a poor club's debt (band −2/−1, or 0 with a low balance): after three random draws, the
/// board pays 450, 300 or 150 × reputation and sends the news.
///
/// `FinanceTable::try_board_debt_payment(club, finances)`, called from the events pass (`process_club_finance_events`). Band (`get_wealth_band`) −2,
/// −1 (divisor 2, or 0.4 with reputation of 3500), or 0 with reputation up to 3500 and a balance below 25 × reputation
/// (0.4); the others: 0. random(7000) + 3500 above the reputation; random(6 / divisor) zero; random(level +0x20 of the
/// stadium of the +0xbf club, or 10) not zero. The board pays 450 × reputation (below 1250 with a balance below −450 ×
/// reputation), 300 × (balance below −300 × reputation) or 150 ×: into +0x154, +0xb4 and the balance; the news
/// (`exe::finance_post_finance_boost_news`, type 4); returns 1.
/// Original: `FinanceTable::try_board_debt_payment` (0x59dd40, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x59dd40 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_board_debt_payment(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Faixa (0x5986b0) −2, −1 (divisor 2, ou 0,4 com reputação de 3500), ou 0 com reputação até 3500 e
/// saldo abaixo de 25 × reputação (0,4); as outras: 0. Sorteio(7000) + 3500 acima da reputação; sorteio(6 / divisor)
/// zero; sorteio(nível +0x20 do estádio do clube de +0xbf, ou 10) não zero. A diretoria paga 450 × reputação (abaixo de
/// 1250 com saldo abaixo de −450 × reputação), 300 × (saldo abaixo de −300 × reputação) ou 150 ×: em +0x154, +0xb4 e no
/// saldo; a notícia (0x59f300, tipo 4); devolve 1.
pub(crate) unsafe extern "thiscall" fn try_board_debt_payment(this: usize, club: usize, fin: usize) -> u32 {
    let club = Club(club);
    let rep = club.reputation() as i32;
    let div = match wealth_band(fin) {
        -2 => 2.0f64,
        -1 => {
            if rep >= 0xdac {
                0.4
            } else {
                2.0
            }
        }
        0 => {
            if rep > 0xdac || balance(fin) >= rep.wrapping_mul(25) as i64 {
                return 0;
            }
            0.4
        }
        _ => return 0,
    };
    crate::comparison::branch("cpv_6 debt draw");
    let r = friendly_random(7000); // cpv6: the debt random draw
    if rep >= r.wrapping_add(0xdac) {
        return 0;
    }
    let n = ftol(read_f64(rdata::F64_6) / div);
    let other = Staff(club.chairman().addr());
    if !other.is_null() && other.non_player() != 0 {
        if friendly_random(n) != 0 {
            return 0;
        }
        let est = read_usize(club.chairman().addr() + 0x69);
        if friendly_random(read_u8(est + non_player::RESOURCES) as i8 as i32) == 0 {
            return 0;
        }
    } else {
        if friendly_random(n) != 0 {
            return 0;
        }
        if friendly_random(10) == 0 {
            return 0;
        }
    }
    let rep = club.reputation() as i32;
    let goal_difference = balance(fin);
    let value = if rep < 0x4e2 && goal_difference < rep.wrapping_mul(-450) as i64 {
        rep.wrapping_mul(450)
    } else if goal_difference >= rep.wrapping_mul(-300) as i64 {
        rep.wrapping_mul(150)
    } else {
        rep.wrapping_mul(300)
    };
    add_to_i32(fin + finance::SEASON_OTHER_INCOME, value);
    add_to_i32(fin + finance::MONTH_OTHER_INCOME, value);
    credit_balance(fin, value);
    post_finance_boost_news(this, club, value, 0, 4);
    1
}

/// Comparer of the weekly finance events: the copy in rehearsal mode (no news item and no budget) and the original
/// routine from the same state (finance record and random draws).
///
/// With no event: everything equal, including the random draws. With an event: the return value and the fields the
/// event writes (balance, +0xb4, +0x154, +0x165), because the original's news item and budget may draw and write.
///
/// [pt] O comparador dos eventos: a cópia em ensaio (sem notícia nem verba) e a original a partir do mesmo estado (registro de
/// finanças e sorteio). Sem evento: tudo igual, inclusive o sorteio; com evento: a devolução e os campos que o evento
/// grava (saldo, +0xb4, +0x154, +0x165), porque a notícia e a verba da original podem sortear e gravar.
pub(crate) unsafe fn compare_finance_event(name: &str, fin: usize, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let mut regions = vec![(fin, 0x167)];
    regions.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    RARE_FINANCE_EVENT_DRY_RUN = true;
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(copy));
    RARE_FINANCE_EVENT_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let same = if r == 0 && n == 0 {
        mo == mr
    } else {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165]
    };
    SCOREBOARD.record(same, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", name, fin, r, mo, n, mr));
    r
}

/// Tries a cash injection for a club of reputation 4750 outside Wales and the Irelands: random draws by the band and
/// the stadium of the +0xbf club; the amount by reputation, up to 10% of the debt and 4 million; news and end of the
/// crisis.
///
/// `FinanceTable::try_cash_injection(club, finances)`, called from the events pass (`process_club_finance_events`). Reputation (+0x80) of
/// 4750, without flag +0x82, outside Wales and the Irelands. Divisor by the band (−2: 0.25; −1: 0.1; others: 0.001).
/// With the stadium of the +0xbf club: random(30 / divisor) below the level (+0xf) and level +0x20 above random(5) +
/// 10: (20 × level + 150) × reputation + 750000; without it: random(4 / divisor) zero: (5 × reputation + 15000) × 50.
/// Debt above 10 × the amount: 10% of the debt. Ceiling: reputation² + 1.5 million below 2000; otherwise 4 million (+1
/// if the amount exceeds reputation² / 4). Into +0x154, +0xb4 and the balance; the news (`exe::finance_post_finance_boost_news`, type 3, with the
/// budget 0x598800); ends the crisis (+0x165 = 0); returns 1.
/// Original: `FinanceTable::try_cash_injection` (0x59e5a0, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x59e5a0 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_cash_injection(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Reputação (+0x80) de 4750, sem a marca +0x82, fora do País de Gales e das Irlandas. Divisor pela
/// faixa (−2: 0,25; −1: 0,1; outras: 0,001). Com o estádio do clube de +0xbf: sorteio(30 / divisor) abaixo do nível
/// (+0xf) e o nível +0x20 acima de sorteio(5) + 10: (20 × nível + 150) × reputação + 750000; sem ele: sorteio(4 /
/// divisor) zero: (5 × reputação + 15000) × 50. Dívida acima de 10 × o valor: 10% da dívida. Teto: reputação² + 1,5
/// milhão abaixo de 2000; senão 4 milhões (+1 se o valor passar de reputação² / 4). Em +0x154, +0xb4 e no saldo; a
/// notícia (0x59f300, tipo 3, com a verba 0x598800); encerra a crise (+0x165 = 0); devolve 1.
pub(crate) unsafe extern "thiscall" fn try_cash_injection(this: usize, club: usize, fin: usize) -> u32 {
    let club = Club(club);
    if club.reputation() < 0x128e { // cpv6: the injection filter
        return 0;
    }
    if club.plc() != 0 {
        return 0;
    }
    if !club.is_null() {
        let nation = club.nation().addr();
        if nation != 0 {
            let n = read_i32(nation);
            if n == read_i32(KEY_NATION_WALES) || n == read_i32(KEY_NATION_REPUBLIC_OF_IRELAND) || n == read_i32(KEY_NATION_NORTHERN_IRELAND) {
                return 0;
            }
        }
    }
    crate::comparison::branch("cpv_6 injection evaluated");
    let div = match wealth_band(fin) {
        -2 => read_f64(rdata::F64_0_25),
        -1 => read_f64(rdata::F64_0_1),
        _ => read_f64(rdata::F64_0_001),
    };
    let rep = club.reputation() as i32;
    let other = Staff(club.chairman().addr());
    let mut value;
    if !other.is_null() && other.non_player() != 0 && club.plc() == 0 {
        let r = friendly_random(ftol(read_f64(rdata::F64_30) / div));
        let est = read_usize(club.chairman().addr() + 0x69);
        if (read_u8(est + non_player::BUSINESS) as i8 as i32) <= r {
            return 0;
        }
        let e = read_u8(read_usize(club.chairman().addr() + 0x69) + 0x20) as i8 as i32;
        if e <= friendly_random(5).wrapping_add(10) {
            return 0;
        }
        value = e.wrapping_mul(20).wrapping_add(0x96).wrapping_mul(rep).wrapping_add(0xb71b0);
    } else {
        if friendly_random(ftol(read_f64(rdata::F64_4) / div)) != 0 {
            return 0;
        }
        if club.plc() != 0 {
            return 0;
        }
        value = rep.wrapping_mul(5).wrapping_add(0x3a98).wrapping_mul(50);
    }
    let goal_difference = balance(fin);
    if (value.wrapping_mul(10) as i64) < goal_difference.wrapping_neg() {
        value = ftol(-(goal_difference as f64 * read_f64(rdata::F64_0_1)));
    }
    if club.reputation() < 0x7d0 {
        let ceiling = rep.wrapping_mul(rep).wrapping_add(0x16e360);
        if value > ceiling {
            value = ceiling;
        }
    } else {
        let greater = (value as f64 > rep.wrapping_mul(rep) as f64 * read_f64(rdata::F64_0_25)) as i32;
        if value > 0x3d0900 + greater {
            value = 0x3d0900 + greater;
        }
    }
    add_to_i32(fin + finance::SEASON_OTHER_INCOME, value);
    add_to_i32(fin + finance::MONTH_OTHER_INCOME, value);
    credit_balance(fin, value);
    let budget = transfer_budget(fin);
    post_finance_boost_news(this, club, value, budget, 3);
    *((fin + finance::CRISIS_STATE) as *mut u8) = 0;
    1
}

/// Returns the highest weekly wage the club offers a staff member for a job: default by job, nation level (+0x85) /
/// type, key nation adjustments, league weight × 0.2, finance band, club ceiling (+0x14) and job range.
///
/// `Finance::get_max_staff_wage(job)`, used by renewal (`contract_manager_offer_contract`), interest in the job (`exe::staff_interest_in_job_at_club`) and the contract
/// screens. Club by id (+8); type 3 (+0x64): 0. The default by job (5 to 7: 35000; 8: 20000; 9: 1500; 10: 1000; others:
/// 750); national team (`exe::database_is_national_team`): the default. Without a nation (+0x53): default × 0.25. With a nation: the level
/// (+0x85) / (20 × type +0x7e between 1 and 3) × default; in type 1, England, Italy and Spain × 1.05 with reputation
/// above 7250, Japan nothing, USA above 1750 with 0.75 of the excess, and the other groups with the excess above
/// 2500/1250/750 by the factor (0.75/0.75/0.3, or reputation/50 / level × 0.1 × it, up to 1); with a league (+0x57), ×
/// the league weight (+0x69) × 0.2. By the finance band (`get_wealth_band`): −2 × 0.75, −1 × 0.95, 1 × 1.5, 2 × 2. Limited to
/// the club's ceiling (+0x14) and to the job's range.
/// Original: `Finance::get_max_staff_wage` (0x597db0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x597db0 } (thiscall, ret 4, finance.cpp): `Finance::get_max_staff_wage(cargo)`, o maior salário semanal que o
/// clube oferece a um membro do pessoal (renovação 0x4cda70, interesse pelo cargo 0x844780, telas de contrato). Clube
/// pelo id (+8); tipo 3 (+0x64): 0. O padrão pelo cargo (5 a 7: 35000; 8: 20000; 9: 1500; 10: 1000; outros: 750);
/// seleção (0x53b900): o padrão. Sem nação (+0x53): padrão × 0,25. Com nação: o nível (+0x85) / (20 × tipo +0x7e
/// entre 1 e 3) × padrão; no tipo 1, Inglaterra, Itália e Espanha × 1,05 com reputação acima de 7250, Japão nada, EUA
/// acima de 1750 com 0,75 do excedente, e os demais grupos com o excedente acima de 2500/1250/750 pelo fator (0,75/0,75/
/// 0,3, ou reputação/50 / nível × 0,1 × ele, até 1); com liga (+0x57), × peso da liga (+0x69) × 0,2. Pela faixa das
/// finanças (0x5986b0): −2 × 0,75, −1 × 0,95, 1 × 1,5, 2 × 2. Limitado ao teto do clube (+0x14) e à faixa do cargo.
pub(crate) unsafe extern "thiscall" fn get_max_staff_wage(this: usize, job: i8) -> i32 {
    let id = read_usize(this + finance::CLUB_ID);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE)));
    if club.professional_status() == 3 {
        return 0;
    }
    let r50 = club.reputation() as i32 / 50;
    let idx = (job as i32).wrapping_sub(5);
    let default: i32 = match idx {
        0..=2 => 35000,
        3 => 20000,
        4 => 1500,
        5 => 1000,
        _ => 750,
    };
    if exe::database_is_national_team(club) != 0 {
        return default;
    }
    let nation = club.nation().addr();
    let mut w;
    if nation == 0 {
        w = ftol(default as f64 * read_f64(rdata::F64_0_25));
    } else {
        let kind = (read_u8(nation + nation::STATE_OF_DEVELOPMENT) as i8).clamp(1, 3);
        let k = read_u8(nation + nation::LEAGUE_STANDARD) as i8 as i32;
        w = ftol(k as f64 / (kind as i32 * 20) as f64 * default as f64);
        if kind == 1 {
            let n = read_i32(nation);
            let e = |a: usize| n == read_i32(a);
            if e(KEY_NATION_ENGLAND) || e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN) {
                if club.reputation() > 0x1c52 {
                    crate::comparison::branch("cpv_7 big club");
                    w = ftol(w as f64 * read_f64(rdata::F64_1_05)); // cpv7: the big club of England, Italy and Spain;
                }
            } else if e(KEY_NATION_JAPAN) {
            } else if e(KEY_NATION_UNITED_STATES) {
                if w > 0x6d6 {
                    w = ftol(w.wrapping_sub(0x6d6) as f64 * read_f64(rdata::F64_0_75) + read_f64(rdata::F64_1750));
                }
            } else {
                let (base, limiar, soma) = if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SCOTLAND) || e(KEY_NATION_HOLLAND) {
                    (0x969040, 0x9c4, 0x9690b0)
                } else if e(KEY_NATION_SWEDEN) || e(KEY_NATION_DENMARK) || e(KEY_NATION_BELGIUM) || e(KEY_NATION_PORTUGAL) {
                    (0x969040, 0x4e2, 0x96a5e8)
                } else {
                    (0x968e80, 0x2ee, 0x96a5f0)
                };
                let mut f = read_f64(base);
                if r50 > k.wrapping_mul(10) {
                    let d = if k >= 1 { k } else { 1 };
                    f = r50 as f64 / d as f64 * read_f64(rdata::F64_0_1) * read_f64(base);
                }
                if w > limiar {
                    if f > read_f64(rdata::F64_1) {
                        f = read_f64(rdata::F64_1);
                    }
                    w = ftol(w.wrapping_sub(limiar) as f64 * f + read_f64(soma));
                }
            }
        }
        let league = club.division().addr();
        if league != 0 {
            crate::comparison::branch("cpv_7 league weight");
            w = ftol(read_i16(league + competition::REPUTATION) as f64 * w as f64 * read_f64(rdata::F64_0_2)); // cpv7: the league weight;
        }
    }
    match exe::finance_get_wealth_band(this) as i8 {
        -2 => w = ftol(w as f64 * read_f64(rdata::F64_0_75)),
        -1 => w = ftol(w as f64 * read_f64(rdata::F64_0_95)),
        1 => w = ftol(w as f64 * read_f64(rdata::F64_1_5)),
        2 => w = w.wrapping_add(w),
        _ => {}
    }
    let ceiling = read_i32(this + finance::WAGE_CEILING);
    if ceiling < w {
        w = ceiling;
    }
    let (min, max) = match idx {
        0 => (0x6d6, 0x88b8),
        1 | 2 => (0x1f4, 0x4e20),
        3 => (0xfa, 0x1d4c),
        4 => (0x7d, 0x7d0),
        5 => (0x7d, 0x5dc),
        _ => (0x7d, 0x3e8),
    };
    if w > max {
        max
    } else if w < min {
        min
    } else {
        w.min(0x3d090)
    }
}

/// Adds the amount to the bonus expenses (+0x138 and +0x98) and takes it from the balance.
///
/// `Finance::pay_bonuses(amount)`, called from the contracts' end of match (CONTRACT_MANAGER::update_after_match
/// `update_after_match`, three calls): adds the amount to the bonus expenses (+0x138 and +0x98) and subtracts it from the balance
/// (i64 at +0, sign-extended).
/// Original: `Finance::pay_bonuses` (0x599960, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599960 } (thiscall, ret 4, finance.cpp): `Finance::pay_bonuses(valor)`, pelo fim do jogo dos contratos
/// (CONTRACT_MANAGER::update_after_match 0x4ca6d0, três chamadas): soma o valor nas despesas de bônus (+0x138 e +0x98)
/// e o tira do saldo (i64 em +0, estendido com sinal).
pub(crate) unsafe extern "thiscall" fn pay_bonuses(this: usize, value: i32) {
    for o in [0x138usize, 0x98] {
        core::ptr::write_unaligned((this + o) as *mut i32, read_i32(this + o).wrapping_add(value));
    }
    crate::comparison::branch("cpv_7 bonuses paid");
    let s = core::ptr::read_unaligned(this as *const i64);
    core::ptr::write_unaligned(this as *mut i64, s.wrapping_sub(value as i64)); // cpv7: the bonus leaves the balance
}

/// wage ceiling (+0x14) × (percent + 100)%, truncated
/// Original: `Finance::adjust_max_wage` (0x598340, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x598340 } (thiscall, ret 4, finance.cpp): `Finance::adjust_max_wage(percentual)`, pela resposta da diretoria
/// (MANAGER_MANAGER::handle_board_request 0x6a91b0) e pela 0x6acee8: o teto salarial (+0x14) vira (percentual + 100) ×
/// 0,01 × teto, truncado.
pub(crate) unsafe extern "thiscall" fn adjust_max_wage(this: usize, write_percent_pair: i8) {
    let v = ftol((write_percent_pair as i32 + 100) as f64 * read_f64(rdata::F64_0_01) * read_i32(this + finance::WAGE_CEILING) as f64);
    core::ptr::write_unaligned((this + finance::WAGE_CEILING) as *mut i32, v);
}

/// Returns the club's wealth band: national team, 0; by the 64-bit balance against the club reputation R: above 2500·R
/// + 5,000,000, 2; above 1000·R + 3,500,000, 1; below −2,500,000 − 1500·R, −2; below −250·R (R < 5000) or (3750 −
/// R)·1000, −1; otherwise 0.
///
/// `Finance::get_wealth_band()`, the band of the club's (+8, array 0xae23b4) balance (64 bits at +0/+4) by the
/// reputation R (+0x80), used by transfers, the board, the managers, the finances and the postponement. National team
/// (`exe::database_is_national_team`): 0. Above 2500·R + 5,000,000: 2; above 1000·R + 3,500,000: 1; below −2,500,000 − 1500·R: −2; below −250·R
/// (R < 5000) or (3750 − R)·1000 (R ≥ 5000): −1; otherwise 0.
/// Original: `Finance::get_wealth_band` (0x5986b0, finance.cpp, thiscall).
///
/// [pt] { 0x5986b0 } (thiscall, finance.cpp): `Finance::get_wealth_band()`, a faixa do saldo (64 bits em +0/+4) do clube (+8,
/// vetor 0xae23b4) pela reputação R (+0x80), usada pelas transferências, pela diretoria, pelos técnicos, pelas finanças e
/// pelo adiamento. Seleção (0x53b900): 0. Acima de 2500·R + 5.000.000: 2; acima de 1000·R + 3.500.000: 1; abaixo de
/// −2.500.000 − 1500·R: −2; abaixo de −250·R (R < 5000) ou de (3750 − R)·1000 (R ≥ 5000): −1; senão 0.
pub(crate) unsafe extern "thiscall" fn get_wealth_band(this: usize) -> u8 {
    let id = core::ptr::read_unaligned((this + finance::CLUB_ID) as *const u32);
    let club = Club(core::ptr::read_unaligned(G_CLUB_TABLE as *const usize).wrapping_add(id.wrapping_mul(0x245) as usize));
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    let rep = core::ptr::read_unaligned((club.addr() + club::REPUTATION) as *const i16);
    let r = rep as i32;
    let goal_difference = core::ptr::read_unaligned(this as *const i64);
    if goal_difference > r.wrapping_mul(2500).wrapping_add(5_000_000) as i64 {
        crate::comparison::branch("cpu_7 rich");
        return 2; // cpu7: a faixa rica
    }
    if goal_difference > r.wrapping_mul(1000).wrapping_add(3_500_000) as i64 {
        crate::comparison::branch("cpu_7 comfortable");
        return 1; // cpu7: a faixa confortável
    }
    if goal_difference < (-2_500_000i32).wrapping_sub(r.wrapping_mul(1500)) as i64 {
        return 0xfe;
    }
    let devendo = if rep < 0x1388 { r.wrapping_mul(-250) } else { (0xea6 - r).wrapping_mul(1000) };
    if goal_difference < devendo as i64 {
        crate::comparison::branch("cpu_7 in debt");
        return 0xff; // cpu7: a faixa devendo
    }
    0
}

