use super::*;







/// Adds v to the month and year expense accumulators of a finance record and subtracts it from the i64 balance.
///
/// [pt] Soma `v` nos dois acumuladores do ano e do mês (`ano`, `mes`) e o tira do saldo (i64; o cdq/sub/sbb da original).
pub(crate) unsafe fn charge_expense(fin: usize, mes: usize, ano: usize, v: i32) {
    add_to_i32(fin + mes, v);
    add_to_i32(fin + ano, v);
    core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_sub(v as i64));
}


/// Calls FN_HUMAN_MANAGER_IS_HUMAN_CLUB (0x600a50) with (club, 0, 1, 0, 0).
///
/// [pt] O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
pub(crate) unsafe fn fin_prm_is_human_club(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, 0, 1, 0, 0)
}

/// Finance record (0x167 bytes) of the club, using the linked parent when the club is a B team (0x540a50); writes the
/// original's local flag.
///
/// [pt] O registro de finanças do clube (0x167 bytes) pelo parceiro: 0x540a50 (3 pushes; add esp, 0xc -> cdecl(clube, &a,
/// 1)); o parceiro quando ele existe e o clube é o B (a = 0). `a` é a local da original (a 0x540a50 a grava).
pub(crate) unsafe fn finance_of_parent_club(this: usize, clube: usize, a: &mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let p = parceiro(clube, a, 1);
    let c = if p != 0 && *a == 0 { p } else { clube };
    read_usize(this).wrapping_add((read_i32(c) as u32).wrapping_mul(0x167) as usize)
}

/// Posts the stadium works / training news (FN_FINANCE_POST_STADIUM_WORKS_NEWS) with club, value and type.
///
/// [pt] A notícia das obras e do treino (0x59ffb0: ecx = tabela; 3 pushes (tipo, valor, clube); sem add esp -> thiscall,
/// ret 0xc).
pub(crate) unsafe fn post_works_news(this: usize, clube: usize, valor: i32, tipo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(FN_FINANCE_POST_STADIUM_WORKS_NEWS);
    f(this, clube, valor, tipo);
}

/// Posts the refused/cancelled works news (FN_FINANCE_POST_WORKS_CANCELLED_NEWS, may draw random numbers).
///
/// [pt] A notícia da obra recusada (0x59fae0: ecx = tabela; 3 pushes; sem add esp -> thiscall, ret 0xc; sorteia 2 fora do
/// motivo 2).
pub(crate) unsafe fn post_works_refused_news(this: usize, clube: usize, assunto: i32, motivo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(FN_FINANCE_POST_WORKS_CANCELLED_NEWS);
    f(this, clube, assunto, motivo);
}

/// Club's row in the MANAGER_MANAGER board table ([[G_MANAGER_MANAGER]] + id × 0x49); byte +2 holds the board requests
/// (bit 6 stadium, bit 7 training ground).
///
/// [pt] A linha do clube na tabela dos técnicos ([[0xb67a44]] + id × 0x49); o byte +2 tem os pedidos à diretoria (bit 6:
/// estádio; bit 7: centro de treino).
pub(crate) unsafe fn board_requests_row(clube: usize) -> usize {
    read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x49) as usize)
}

/// True if the club's manager (club::MANAGER) is one of the 16 human managers.
///
/// [pt] O técnico do clube (+0xcf) é humano (entre as 16 últimas pessoas): a notícia vai para ele (0x7861e0).
pub(crate) unsafe fn fin_prm_has_human_manager(clube: usize) -> bool {
    let t = read_usize(clube + club::MANAGER);
    t != 0 && read_i32(t) >= read_i32(0xae2364).wrapping_sub(0x10)
}

/// after the game start date, for each club with a league and an active nation or reputation 5000 whose row is the
/// given key nation: the club season end, and the season totals (+0x110) move to last season (+0xc0) and are cleared;
/// stops at the first national team club
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
pub(crate) unsafe extern "thiscall" fn fin_prm_end_season_for_nation(this: usize, entrada: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if depois(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return;
    }
    if entrada == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIN_PRM_END_SEASON_FOR_NATION)(this, entrada); // a caixa
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let do_clube: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINANCETABLE_SEASON_END_CLUB_UPDATE);
    let mut clube = read_usize(G_CLUB_TABLE);
    let mut k = 0i32;
    let mut desloc = 0usize;
    while k < read_i32(0xae235c) {
        if selecao(clube) != 0 {
            return;
        }
        let nacao = read_usize(clube + club::NATION);
        if read_usize(clube + club::DIVISION) != 0 && nacao != 0 {
            let ativa = read_u8(nacao + nation::NATION_LEAGUE_SELECTED) != 0;
            if ativa || read_i16(clube + club::REPUTATION) >= 0x1388 { // cqc1: o clube de nação inativa pela reputação
                let l = linha(clube) as i8 as i32;
                if (0xb63d60 + l * 72) as usize == entrada {
                    if !ativa && read_i16(clube + club::REPUTATION) < 0x1770 {
                        crate::comparacao::ramo("cqc_1 nacao inativa abaixo de 6000");
                    }
                    do_clube(this, clube);
                    let f = read_usize(this).wrapping_add(desloc);
                    core::ptr::copy_nonoverlapping((f + 0x110) as *const u8, (f + 0xc0) as *mut u8, 0x50); // cqc1: os totais do ano passam para o ano anterior
                    core::ptr::write_bytes((f + 0x110) as *mut u8, 0, 0x50);
                }
            }
        }
        k += 1;
        desloc += 0x167;
        clube += 0x245;
    }
}

/// Comparer for `fin_prm_end_season_for_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do fim de temporada da nação: só a original, porque ela chama o fim de temporada de cada clube
/// (0x59cfa0), que sorteia e entrega notícias ao técnico humano (a cópia segue no modo Rust, com o contador). Confere o
/// que a rotina garante e nenhuma chamada desfaz: o último clube da entrada (a ordem dos clubes) fica com os totais do
/// ano (+0x110, 0x50 bytes) zerados. Ponytail: os anteriores podem ganhar valores pela 0x59cfa0 de um clube seguinte
/// (o B que usa o registro do A, o estádio dividido); conferir todos pede o plano dessas exceções.
pub(crate) unsafe extern "thiscall" fn comparar_fin_prm_end_season_for_nation(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIN_PRM_END_SEASON_FOR_NATION);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if entrada == 0 || depois(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return o(this, entrada);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let mut ultimo = None;
    let mut clube = read_usize(G_CLUB_TABLE);
    for k in 0..read_i32(0xae235c).max(0) as usize {
        if selecao(clube) != 0 {
            break;
        }
        let nacao = read_usize(clube + club::NATION);
        if read_usize(clube + club::DIVISION) != 0 && nacao != 0 && (read_u8(nacao + nation::NATION_LEAGUE_SELECTED) != 0 || read_i16(clube + club::REPUTATION) >= 0x1388) && (0xb63d60 + (linha(clube) as i8 as i32) * 72) as usize == entrada {
            ultimo = Some(k);
        }
        clube += 0x245;
    }
    o(this, entrada);
    if let Some(k) = ultimo {
        let f = read_usize(this) + k * 0x167 + 0x110;
        let zerado = core::slice::from_raw_parts(f as *const u8, 0x50).iter().all(|&x| x == 0);
        PLACAR.registrar(zerado, || format!("fin_prm_end_season_for_nation(entrada {:#x}) clube {} sem zerar os totais do ano", entrada, k));
    }
}

/// ground purchase (20 million, from an owner in crisis), the squeeze, the AI club training facilities by draw,
/// undersoil heating (1.5 million) and roof (7.5 million), expansion or seating by balance and average attendance, the
/// human's board requests (stadium and training) and the dividend by reputation, balance and chairman, with a draw
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
pub(crate) unsafe extern "thiscall" fn season_end_club_update(this: usize, clube: usize) {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE)(this, clube); // a caixa
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        return;
    }
    let mut local = 0u32; // o [esp + 0x20] da original (a 0x540a50 do dono e a média de público)
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, clube, &mut a);
    if fin == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE)(this, clube); // a caixa
    }
    let mut saldo = read_i32(fin);
    let mut aviso = false;
    let estadio = read_usize(clube + club::STADIUM);
    'obras: {
        if estadio == 0 {
            break 'obras;
        }
        if read_u8(clube + club::OWN_STADIUM) == 0 && read_usize(clube + club::DIVISION) != 0 {
            if saldo <= 0x2160ec0 {
                break 'obras;
            }
            let mut dono = 0usize;
            let mut outro = read_usize(G_CLUB_TABLE);
            let mut k = 0i32;
            while k < read_i32(0xae235c) {
                if outro != clube && read_usize(outro + 0x69) == estadio && read_u8(outro + 0x6d) != 0 {
                    dono = outro;
                    break;
                }
                k += 1;
                outro += 0x245;
            }
            let f2 = if dono != 0 { finance_of_parent_club(this, dono, &mut local) } else { 0 };
            if dono == 0 || f2 == 0 {
                *((clube + club::OWN_STADIUM) as *mut u8) = 1;
                charge_expense(fin, 0x8c, 0x12c, 20_000_000);
                post_works_news(this, clube, read_i32(read_usize(clube + club::STADIUM)), 4);
            } else if read_u8(f2 + 0x165) == 1 || read_u8(f2 + 0x165) == 2 {
                *((clube + club::OWN_STADIUM) as *mut u8) = 1;
                *((dono + 0x6d) as *mut u8) = 0;
                charge_expense(fin, 0x8c, 0x12c, 20_000_000);
                add_to_i32(f2 + 0xb4, 20_000_000);
                add_to_i32(f2 + 0x154, 20_000_000);
                core::ptr::write_unaligned(f2 as *mut i64, balance(f2).wrapping_add(20_000_000));
                post_works_news(this, clube, read_i32(read_usize(clube + club::STADIUM)), 4);
                post_works_news(this, dono, read_i32(read_usize(clube + club::STADIUM)), 5);
            }
            break 'obras;
        }
        if read_u8(clube + club::OWN_STADIUM) == 0 || read_usize(clube + club::DIVISION) == 0 {
            break 'obras;
        }
        let aperto = read_u8(fin + 0x166) as i8;
        if saldo > 0xb71b0 && aperto > 10 {
            let v = 10i32.wrapping_sub((aperto as i32).wrapping_mul(10000));
            add_to_i32(fin + 0x12c, v);
            saldo = saldo.wrapping_sub(v);
            add_to_i32(fin + 0x8c, v);
            core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_sub(v as i64));
        }
        upgrade_ai_training_ground(this, clube, saldo);
        let pedidos = board_requests_row(clube) + 2;
        if read_u8(pedidos) & 0x40 != 0 && fin_prm_is_human_club(clube) != 0 {
            aviso = true;
            *(pedidos as *mut u8) &= 0xbf;
        }
        let est = read_usize(clube + club::STADIUM);
        if est != 0 {
            if saldo > 0x10b0760 && read_i16(clube + club::REPUTATION) > 0x1d4c && read_i32(est + 0x3c) > 0x61a8 && read_u8(est + 0x4d) == 0 {
                *((est + 0x4d) as *mut u8) = 1;
                charge_expense(fin, 0x8c, 0x12c, 1_500_000);
                post_works_news(this, clube, read_i32(read_usize(clube + club::STADIUM)), 6);
                break 'obras;
            }
            if saldo > 0x47868c0 && read_i16(clube + club::REPUTATION) > 0x2422 && read_i32(est + 0x3c) > 0xe09c && read_u8(clube + club::TRAINING) == 0x14 && read_u8(est + 0x4c) == 0 {
                *((est + 0x4c) as *mut u8) = 1;
                charge_expense(fin, 0x8c, 0x12c, 7_500_000);
                break 'obras;
            }
        }
        let pode = saldo > 0x501bd0 || (fin_prm_is_human_club(clube) != 0 && saldo.wrapping_add((read_i16(clube + club::REPUTATION) as i32).wrapping_mul(1000)) > 0x501bd0);
        if !pode {
            if aviso {
                post_works_refused_news(this, clube, 2, -1);
            }
            break 'obras;
        }
        let liga = read_usize(clube + club::DIVISION);
        if liga != 0 {
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(liga).wrapping_mul(4)));
            if comp != 0 && read_usize(comp + 8) != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
                let l = busca(read_usize(comp + 8), clube);
                if l != 0 && read_u8(l + 5) != 0 {
                    local = read_i32(l + 6).wrapping_div(read_u8(l + 5) as i32) as u32;
                }
            }
        }
        let est = read_usize(clube + club::STADIUM);
        let cap = read_i32(est + 0x3c);
        if cap > 0x1e848 {
            break 'obras;
        }
        let rep = read_i16(clube + club::REPUTATION);
        let expandir = aviso
            || (cap as f64 * read_f64_at(0x96a720)) < (local as i16 as i32) as f64 // 0x96a720 = 0.92
            || (rep.wrapping_mul(2) as i32 > cap && rep > 0x109a);
        if !expandir {
            if friendly_random(0x14).wrapping_add(5) >= read_i16(read_usize(clube + club::DIVISION) + 0x69) as i32 {
                break 'obras;
            }
            let est = read_usize(clube + club::STADIUM);
            let livre = read_i32(est + 0x3c).wrapping_sub(read_i32(est + 0x40));
            if livre <= 0 {
                break 'obras;
            }
            let t = match_pl_ftol(saldo as f64 * read_f64_at(0x968918)); // 0x968918 = 0.2
            let mut v = ((t / 75) as i16 as i32 / 500) as i16 as i32 * 500;
            if v >= livre {
                v = livre;
            }
            if (v as i16) < 0x1f4 {
                if read_i32(est + 0x3c) >= 0x7530 {
                    break 'obras;
                }
                add_to_i32(est + 0x44, 0x5dc);
                post_works_refused_news(this, clube, 2, 2);
                break 'obras;
            }
            let v = v as i16 as i32;
            add_to_i32(est + 0x3c, v);
            add_to_i32(read_usize(clube + club::STADIUM) + 0x40, v);
            crate::comparacao::ramo("cqc_1 lugares sentados");
            charge_expense(fin, 0x8c, 0x12c, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(75))); // cqc1: os lugares sentados
            post_works_news(this, clube, v, 3);
            break 'obras;
        }
        let tipo1 = read_u8(clube + club::PROFESSIONAL_STATUS) == 1;
        let base = if saldo > 0x1312d00 {
            if tipo1 { 16000 } else { 21000 }
        } else if tipo1 {
            match_pl_ftol(saldo as f64 * read_f64_at(0x967880)) / 125 // 0x967880 = 0.1
        } else {
            match_pl_ftol(saldo as f64 * read_f64_at(0x968918)) / 50 // 0x968918 = 0.2
        };
        let lim = (base as i16 as i32) / 500 * 500;
        let teto = read_i32(est + 0x44);
        let mut v = lim;
        if (lim as i16 as i32).wrapping_add(cap) > teto {
            let dif = teto.wrapping_sub(cap);
            v = if dif < 0 { 0 } else { dif };
        }
        let mut terco = teto / 3;
        if terco < 0x1388 {
            terco = 0x1388;
        }
        let mut n = v as i16 as i32;
        if n >= terco {
            n = terco;
        }
        if (n as i16) < 0x3e8 {
            if cap < 0x7530 {
                add_to_i32(est + 0x44, 0x5dc);
            }
            if aviso {
                post_works_refused_news(this, clube, 2, 2);
            }
            break 'obras;
        }
        let n = n as i16 as i32;
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, n.wrapping_add(cap));
        if tipo1 {
            add_to_i32(read_usize(clube + club::STADIUM) + 0x40, n);
            charge_expense(fin, 0x8c, 0x12c, (n / 1000 + 1).wrapping_mul(6000).wrapping_add(n).wrapping_mul(125));
            post_works_news(this, clube, n, 1);
        } else {
            charge_expense(fin, 0x8c, 0x12c, n.wrapping_add((n / 1000 + 1).wrapping_mul(1000)).wrapping_mul(50));
            post_works_news(this, clube, n, 2);
        }
    }
    let pedidos = board_requests_row(clube) + 2;
    if read_u8(pedidos) & 0x80 != 0 {
        if saldo > (500 - read_i16(clube + club::REPUTATION) as i32).wrapping_mul(500) {
            let melhorar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINANCETABLE_UPGRADE_TRAINING_FACILITIES);
            melhorar(this, clube);
            post_works_news(this, clube, -1, 8);
        } else {
            post_works_refused_news(this, clube, 8, -1);
        }
        *((board_requests_row(clube) + 2) as *mut u8) &= 0x7f;
    }
    pay_dividend(this, clube, fin);
}

/// End-of-season training ground upgrade for AI clubs (part of 0x59cfa0): raises club::TRAINING by chance and
/// reputation and charges the cost to the club's own finances.
///
/// [pt] O centro de treino do clube da máquina no fim da temporada (o trecho 0x59d32a..0x59d54d da 0x59cfa0), cobrado nas
/// finanças do próprio clube ([tabela] + id × 0x167, sem o parceiro).
pub(crate) unsafe fn upgrade_ai_training_ground(this: usize, clube: usize, saldo: i32) {
    let tecnico = read_usize(clube + club::MANAGER);
    if tecnico == 0 || read_i16(clube + club::REPUTATION) == 0 || read_u8(clube + club::TRAINING) as i8 >= 0x14 || read_usize(tecnico + 0x69) == 0 {
        return;
    }
    if fin_prm_is_human_club(clube) != 0 {
        return;
    }
    let q = ((read_i16(clube + club::REPUTATION) as i32) / 500) as i8 as i32 + 8;
    if (read_u8(clube + club::TRAINING) as i8 as i32) <= q || saldo <= 0x225510 {
        return;
    }
    if friendly_random(0x28) >= read_u8(read_usize(read_usize(clube + club::MANAGER) + 0x69) + 0x22) as i8 as i32 {
        return;
    }
    let nivel = (clube + club::TRAINING) as *mut i8;
    let proprio = || read_usize(this).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x167) as usize);
    let cl = *nivel;
    let custo = (cl as i32) * 50000 + 75000;
    if cl < 0x14 {
        *nivel = cl + 1;
    }
    charge_expense(proprio(), 0x8c, 0x12c, custo);
    let cl = *nivel;
    let rep = read_i16(clube + club::REPUTATION);
    if cl < 0x14 && (rep > 0x203a || balance(proprio()) > 0xe4e1c0) {
        let custo = (4_500_000 - 200_000 * cl as i32).max(1_000_000);
        *nivel = 0x14;
        charge_expense(proprio(), 0x8c, 0x12c, custo);
        return;
    }
    if (cl < 0x10 && rep > 0x1482) || (cl < 0x14 && rep > 0x1c52) {
        let custo = (cl as i32) * 50000 + 75000;
        *nivel = cl + 1;
        charge_expense(proprio(), 0x8c, 0x12c, custo);
    }
}

/// End-of-season dividend (part of 0x59cfa0) by the i64 balance, reputation, PLC flag and chairman, charged to the
/// finance record.
///
/// [pt] O dividendo do fim da temporada (o trecho 0x59db6f..0x59dd25 da 0x59cfa0), pelo saldo i64 das finanças.
pub(crate) unsafe fn pay_dividend(this: usize, clube: usize, fin: usize) {
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let mut lim = rep.wrapping_mul(8000);
    if lim < 0x7a120 {
        lim = 0x7a120;
    }
    let s = balance(fin);
    let v;
    if s > lim as i64 && read_u8(clube + club::PLC) != 0 {
        let r = friendly_random(0x1f5);
        if (rep as i16) < 0xfa0 {
            v = r.wrapping_add(0x5dc).wrapping_mul(rep).max(0xc350);
        } else {
            v = r.wrapping_add(0x7d0).wrapping_mul(rep);
        }
    } else {
        let mut s16 = rep as i16;
        let pres = read_usize(clube + club::CHAIRMAN);
        if pres != 0 && read_usize(pres + 0x69) != 0 {
            let e = (7i16.wrapping_sub(read_u8(pres + 0x59) as i8 as i16)).wrapping_mul(3).wrapping_sub(read_u8(read_usize(pres + 0x69) + 0x20) as i8 as i16);
            s16 = (rep as i16).wrapping_add(e.wrapping_mul(5));
            if s16 < 0x1f4 {
                s16 = 0x1f4;
            }
        }
        let sv = s16 as i32;
        if s > sv.wrapping_mul(6000) as i64 {
            let r = friendly_random(0x1f5);
            if read_i16(clube + club::REPUTATION) < 0xfa0 {
                crate::comparacao::ramo("cqc_1 dividendo do pequeno pelo presidente");
                v = r.wrapping_add(0x3e8).wrapping_mul(rep); // cqc1: o dividendo do pequeno pelo presidente
            } else {
                v = r.wrapping_add(0x5dc).wrapping_mul(rep);
            }
        } else if s > sv.wrapping_mul(3000) as i64 {
            v = if read_i16(clube + club::REPUTATION) < 0xfa0 { rep.wrapping_mul(500) } else { rep.wrapping_mul(750) };
        } else {
            return;
        }
    }
    crate::comparacao::ramo("cqc_1 dividendo");
    add_to_i32(fin + 0x94, v);
    add_to_i32(fin + 0x134, v);
    core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_sub(v as i64)); // cqc1: o dividendo sai do saldo
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_FINANCE_POST_DIVIDEND_NEWS);
    noticia(this, clube, v);
}

/// Comparer for `season_end_club_update` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do fim de temporada do clube: lado a lado (a cópia sem contar os sorteios) nas finanças do parceiro e do
/// próprio clube, no clube, no estádio (78 bytes), na linha dos pedidos (0x49 bytes) e no estado do sorteio. As notícias
/// (0x59ffb0, 0x59fae0, 0x59fe10) são montadas na pilha e só são entregues ao técnico humano (0x7861e0): com técnico da
/// máquina elas não deixam nada além do sorteio(2) da recusa, que as duas rodadas fazem. Só a original: o técnico
/// humano (a notícia entra na caixa dele) e o estádio dividido (mexe noutro clube e nas finanças dele).
pub(crate) unsafe extern "thiscall" fn comparar_season_end_club_update(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if clube == 0 || selecao(clube) != 0 || fin_prm_has_human_manager(clube) {
        return o(this, clube);
    }
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, clube, &mut a);
    let estadio = read_usize(clube + club::STADIUM);
    if fin == 0 || (estadio != 0 && read_u8(clube + club::OWN_STADIUM) == 0 && read_usize(clube + club::DIVISION) != 0 && read_i32(fin) > 0x2160ec0) {
        return o(this, clube); // ponytail: a compra do estádio pode mexer no dono; só a original
    }
    let proprio = read_usize(this).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x167) as usize);
    let mut reg = vec![(fin, 0x167), (clube, 0x245), (board_requests_row(clube), 0x49)];
    if proprio != fin {
        reg.push((proprio, 0x167));
    }
    if estadio != 0 {
        reg.push((estadio, 0x4e));
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || crate::aleatorio::sem_contar(|| season_end_club_update(this, clube)));
    PLACAR.registrar(igual, || format!("season_end_club_update(clube {:x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// news 0x177c with the club id, the amount and the +0x82 flag, sent to the club manager
/// Original: `FinanceTable::news_dividend` (0x59fe10, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59fe10 } (thiscall, ret 8, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_dividend(clube, valor)`, pelo fim de temporada do clube (0x59cfa0). A notícia 0x177c (0x77bbe0:
/// ecx = local; 2 pushes -> thiscall, ret 8) com o id do clube, o valor e a marca +0x82 (0x785790: ecx = local; 2
/// pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn news_dividend(_this: usize, clube: usize, valor: i32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x177c, 0);
    gravar(p, 0, read_usize(clube) as u32);
    gravar(p, 1, valor as u32);
    gravar(p, 2, read_u8(clube + club::PLC) as i8 as i32 as u32);
    entregar(read_usize(G_NEWS), p, clube);
}

/// Comparer for `news_dividend` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia do dividendo: só a original (ponytail: a notícia vai para a caixa do técnico humano e não
/// pode ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn comparar_news_dividend(this: usize, clube: usize, valor: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_NEWS_DIVIDEND)(this, clube, valor)
}





/// Reads the i64 at address a.
pub(crate) unsafe fn read_i64(a: usize) -> i64 {
    core::ptr::read_unaligned(a as *const i64)
}

/// Multiplies v by the exe double constant at k and truncates (_ftol).
///
/// [pt] `v` × a constante f64 em `k`, truncado (_ftol).
pub(crate) unsafe fn mul_exe_f64(v: i32, k: usize) -> i32 {
    match_pl_ftol(v as f64 * read_f64_at(k))
}

/// Calls FN_HUMAN_MANAGER_IS_HUMAN_CLUB (0x600a50) with (club, a, 1, 0, 0) (passes argument a as the second parameter).
///
/// [pt] O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, a, 1, 0, 0)).
pub(crate) unsafe fn is_human_club_with(clube: usize, a: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, a, 1, 0, 0)
}


/// True if the club is a national team (0x53b900).
///
/// [pt] A seleção (0x53b900: push; add esp, 4 -> cdecl).
pub(crate) unsafe fn fin_prm_is_national_team(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    f(clube) != 0
}

/// Person's contract wage via CONTRACT_MANAGER (0x4d0c20, FN_CONTRACT_MANAGER_GET_WAGE), signed.
///
/// [pt] O valor do contrato da pessoa (0x4d0c20: mov ecx, [0xae1050]; push pessoa -> thiscall, ret 4), com sinal.
pub(crate) unsafe fn contract_wage(pessoa: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_WAGE);
    f(read_usize(G_CONTRACT_MANAGER), pessoa) as i32
}

/// Wage bonus for big-spending clubs (England, Italy or Spain, reputation above 4750, wealth band 0..2) by wage factor
/// w and the PLC flag.
///
/// [pt] O bônus de quem paga mais (o trecho 0x596c0d..0x597238): clube da Inglaterra, Itália ou Espanha, reputação acima
/// de 4750, faixa 0 a 2, pelo fator salarial `w` (a palavra, com sinal) e pela marca +0x82.
pub(crate) unsafe fn big_spender_bonus(clube: usize, band: i8, w: i32) -> i32 {
    let nac = read_usize(clube + club::NATION);
    if nac == 0 {
        return 0;
    }
    let n = read_i32(nac);
    if n != read_i32(KEY_NATION_ENGLAND) && n != read_i32(KEY_NATION_ITALY) && n != read_i32(KEY_NATION_SPAIN) {
        return 0;
    }
    if read_i16(clube + club::REPUTATION) <= 0x128e || !(band == 2 || band == 1 || band == 0) {
        return 0;
    }
    let w16 = w as i16;
    let x = w16 as i32;
    let (base, oito) = if read_u8(clube + club::PLC) != 0 { (0x4e2, 0x2710) } else { (0x2ee, 0x1770) };
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
pub(crate) unsafe fn human_manager_wage_cap(pessoa: usize, np: usize, cargo: i8) -> i32 {
    let idade = read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8;
    let marca = read_u8(pessoa + staff::INT_APPS) != 0;
    let cx = read_i16(np + 0xb);
    let tecnico = cargo < 4 && cargo != 0 && cargo != 7;
    if cx < 0xea6 {
        let ax = read_i16(np + 5);
        if ax < 0x3c {
            if idade <= 0x17 { 0x1388 } else { 0x1388 + 0x9c4 }
        } else if ax < 0x64 {
            if (cargo >= 4 || cargo == 0) && idade < 0x18 { 0x2710 } else { 0x3a98 }
        } else {
            0x61a8
        }
    } else if cx < 0x1482 {
        if read_i16(np + 5) >= 0x64 { 0x7530 } else { 0x7530 - 5000 }
    } else if cx < 0x1c52 {
        if idade > 0x22 && read_i16(np + 5) < 0x78 {
            if tecnico { 0x927c } else { 0x7ef4 }
        } else {
            let cx2 = read_i16(np + 5);
            if cx2 < 0x8c && read_i16(np + 0xd) < 0xea6 {
                0x9c40 + if marca { 0x1388 } else { 0 }
            } else if tecnico {
                if marca { 0xe09c } else if cx2 >= 0x8c { 0x13880 } else { 0x13880 - 15000 }
            } else {
                0xafc8
            }
        }
    } else if read_i16(np + 0xd) < 0x1a5e {
        0x186a0 + if marca { 0x61a8 } else { 0 }
    } else {
        0x2ab98
    }
}

/// the highest wage by reputation, wealth band and nation table; with a person, by the current contract, reputation
/// squared, the roles, the human ceiling, the market, the contract near its end and the chairman; without a person, by
/// the wage table less the payroll and budget per week, with the floor, the league and the nation; adjustments for
/// clubs of +0x64 1 and 2, the league minimum + 100 and the lower of both ceilings
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
pub(crate) unsafe extern "thiscall" fn get_max_wage(fin: usize, pessoa: usize, cargo_arg: u32) -> i32 {
    let mut cargo = cargo_arg as u8;
    let clube = read_usize(G_CLUB_TABLE).wrapping_add((read_i32(fin + 8) as u32).wrapping_mul(0x245) as usize);
    let verba = match_pl_ftol(read_i64(fin) as f64 * read_f64_at(0x968de8)); // 0x968de8 = 1.3
    let mut liga: i32 = 100;
    let mut fator: f64 = 0.25; // [esp + 0x3c]: 0x3fd00000_00000000
    let mut contrato: usize = 0;
    if read_u8(clube + club::PROFESSIONAL_STATUS) == 3 {
        return 0;
    }
    let rep = read_i16(clube + club::REPUTATION);
    let mut di: i32 = rep as i32 / 50;
    let band: i8 = if fin_prm_is_national_team(clube) { 0 } else { wealth_band(fin) };
    let nac = read_usize(clube + club::NATION);
    let e = |g: usize| read_i32(nac) == read_i32(g);
    let tabela: usize = if nac == 0 || e(KEY_NATION_WALES) || e(KEY_NATION_REPUBLIC_OF_IRELAND) || e(KEY_NATION_NORTHERN_IRELAND) || e(KEY_NATION_AUSTRALIA) {
        0x9c8498
    } else if [KEY_NATION_JAPAN, KEY_NATION_SWEDEN, KEY_NATION_CROATIA, KEY_NATION_POLAND, KEY_NATION_NORWAY, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, KEY_NATION_DENMARK, KEY_NATION_FINLAND, KEY_NATION_SCOTLAND, KEY_NATION_GERMANY].iter().any(|&g| e(g)) {
        0x9c8548
    } else if e(KEY_NATION_FRANCE) || (read_u8(clube + club::PLC) != 0 && band != 2 && band != 1) {
        0x9c85f8
    } else {
        0x9c86a8
    };
    let padrao = tabela == 0x9c86a8;
    match band {
        0 => {
            if rep > 0x1676 {
                di = if padrao { (rep as i32 - 0x186a) / 25 + 0x73 } else { di + 5 };
            }
        }
        1 => {
            di = if rep <= 0x1482 { di + 5 } else if padrao { (rep as i32 - 0x1482) / 25 + 0x69 } else { di + 10 };
        }
        2 => {
            di = if rep <= 0x128e { di + 10 } else if padrao { (rep as i32 - 0x128e) / 25 + 0x5f } else { di + 0xf };
        }
        _ => {}
    }
    let mut s = match_pl_ftol((di as i16) as f64 * read_f64_at(0x96a5e0)); // 0x96a5e0 = 47.6
    if (s as i16) > 0x2710 {
        s = 0x2710;
    }
    if (di as i16) > 0xd2 {
        di = 0xd2;
    }
    let humano = is_human_club_with(clube, 0);
    if humano != 0 && cargo == 0xff {
        let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contrato = com_clube(read_usize(G_CONTRACT_MANAGER), pessoa, clube);
        if contrato != 0 {
            cargo = read_u8(contrato + contract::SQUAD_STATUS) >> 4;
        }
    }
    let mut r: i32; // [esp + 0x18]: o teto que sai
    let b20: i32;
    if pessoa != 0 {
        r = 0;
        if read_usize(pessoa + staff::CLUB_JOB) == clube {
            r = contract_wage(pessoa);
            let t = read_i32(fin + 0x14).wrapping_mul(2);
            if r > t {
                r = t;
            }
        }
        if read_i32(fin + 0x14) > r {
            r = read_i32(fin + 0x14);
        }
        if read_usize(pessoa + staff::NON_PLAYER) != 0 && read_usize(pessoa + staff::PLAYER) == 0 {
            let cap = match (read_u8(pessoa + staff::JOB_FOR_CLUB) as i8 as i32).wrapping_sub(5) {
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
        b20 = big_spender_bonus(clube, band, s);
    } else {
        let mut f = fator;
        if nac != 0 {
            let mut dl = read_u8(nac + 0x7e) as i8;
            if dl < 1 {
                dl = 1;
            } else if dl > 3 {
                dl = 3;
            }
            let bl = read_u8(nac + 0x85) as i8;
            let q = bl as i32 as f64 / (dl as i32 * 20) as f64;
            f = q;
            if dl == 1 {
                let m = if e(KEY_NATION_ENGLAND) || e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN) || e(KEY_NATION_JAPAN) {
                    read_f64_at(0x967890) // 0x967890 = 1
                } else if e(KEY_NATION_UNITED_STATES) {
                    read_f64_at(0x969040) // 0x969040 = 0.75
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
                        read_f64_at(k)
                    } else {
                        let div = if bl < 1 { 1 } else { bl as i32 };
                        let mut m = d16 as f64 / div as f64 * read_f64_at(0x967880) * read_f64_at(k); // 0x967880 = 0.1
                        if tres && !(m <= read_f64_at(0x969040)) { // 0x969040 = 0.75
                            m = read_f64_at(0x969040); // 0x969040 = 0.75
                        } else if !(m <= read_f64_at(0x967890)) { // 0x967890 = 1
                            m = read_f64_at(0x967890); // 0x967890 = 1
                        }
                        m
                    }
                };
                f = q * m;
            }
            fator = f;
            let lc = read_usize(clube + club::DIVISION);
            if lc != 0 {
                let d2 = read_usize(clube + club::LAST_DIVISION);
                let c16 = read_i16(lc + 0x69);
                let cubo = |x: i32| x.wrapping_mul(x).wrapping_mul(x);
                let v = if d2 != 0 && read_i16(d2 + 0x69) > c16 {
                    let t1 = match_pl_ftol(cubo(c16 as i32) as f64 * f * read_f64_at(0x96a5c8)); // 0x96a5c8 = -1.25
                    let t2 = match_pl_ftol(cubo(read_i16(d2 + 0x69) as i32) as f64 * f * read_f64_at(0x968928)); // 0x968928 = 1.25
                    t2.wrapping_sub(t1)
                } else {
                    match_pl_ftol(cubo(c16 as i32) as f64 * f * read_f64_at(0x968e18)) // 0x968e18 = 2.5
                };
                liga = if v < 0xc8 { 0xc8 } else { v };
            }
        }
        let d16 = di as i16 as i32;
        let mut q10 = (200 - d16) / 10;
        if q10 < 1 {
            q10 = 1;
        }
        let a = d16 as f64;
        let t = match_pl_ftol(a * a * a * f / q10 as f64);
        crate::comparacao::ramo("cqc_2 teto pela tabela");
        let menos = read_i32(fin + 0xc).wrapping_sub(t.wrapping_add(verba) / 52); // cqc2: a folha e a verba por semana
        let idx = ((d16 / 10) * 8) as isize as usize;
        let vv = read_i64(tabela.wrapping_add(idx)).wrapping_sub(menos as i64) as f64 * f;
        let piso = read_i64((if padrao { 0x9c83e8usize } else { 0x9c8338 }).wrapping_add(idx)) as f64 * f;
        r = if vv <= piso { match_pl_ftol(piso) } else { match_pl_ftol(vv) };
        if !fin_prm_is_national_team(clube) {
            match wealth_band(fin) {
                1 => r = mul_exe_f64(r, 0x9689b8),
                2 => r = mul_exe_f64(r, 0x9689c0),
                _ => {}
            }
            if nac != 0 {
                let ita_esp = e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN);
                if ita_esp {
                    if rep > 0x1e46 {
                        r = mul_exe_f64(r, 0x9689b8);
                    }
                } else if e(KEY_NATION_ENGLAND) && rep > 0x1e46 {
                    r = mul_exe_f64(r, 0x968fa0);
                }
                if read_u8(clube + club::PLC) == 0 && rep > 0x1e46 && ita_esp {
                    r = mul_exe_f64(r, 0x9689b8);
                }
            }
        }
        b20 = big_spender_bonus(clube, band, s);
    }
    if pessoa != 0 {
        let repf = rep as i32 as f64;
        let t = repf * repf * fator * read_f64_at(0x96a5c0) + read_i32(fin + 0x10) as f64; // 0x96a5c0 = 0.0001
        let v = contract_wage(pessoa);
        s = if (v as f64) < t {
            crate::comparacao::ramo("cqc_2 teto pela injury_word_0c");
            match_pl_ftol(repf * repf * fator * read_f64_at(0x96a5c0) + read_i32(fin + 0x10) as f64) // cqc2: a reputação ao quadrado; 0x96a5c0 = 0.0001
        } else {
            contract_wage(pessoa)
        };
        let np = read_usize(pessoa + staff::PLAYER);
        let c = cargo as i8;
        if np != 0 && read_i16(np + 0xb) > 0x1482 && (c == 3 || c == 2 || c == 1) && (read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8) < 0x20 {
            let np2 = || read_usize(pessoa + staff::PLAYER);
            if c == 3 {
                let bf = b20 as f64;
                let lim = bf * read_f64_at(0x9689b0); // 0x9689b0 = 0.8
                if (r as f64) < lim {
                    r = if is_human_club_with(clube, 0) == 0 && np2() != 0 && read_i16(np2() + 0xb) > 0x1c52 { match_pl_ftol(bf * read_f64_at(0x9689d8)) } else { match_pl_ftol(lim) }; // 0x9689d8 = 0.9
                }
            } else if c == 2 {
                let lim = b20 as f64 * read_f64_at(0x9689d8); // 0x9689d8 = 0.9
                if (r as f64) < lim {
                    r = if is_human_club_with(clube, 0) == 0 && np2() != 0 && read_i16(np2() + 0xb) > 0x1e46 { b20 } else { match_pl_ftol(lim) };
                }
            } else if r < b20 {
                r = b20;
            }
        }
        if is_human_club_with(clube, 0) != 0 {
            let np = read_usize(pessoa + staff::PLAYER);
            if np != 0 {
                let teto = human_manager_wage_cap(pessoa, np, cargo as i8);
                if teto < r {
                    r = teto.wrapping_sub(match_pl_ftol(r.wrapping_sub(teto) as f64 * read_f64_at(0x969518))); // 0x969518 = -0.1
                }
            }
        }
        let np = read_usize(pessoa + staff::PLAYER);
        if np != 0 {
            if nac != 0 && (e(KEY_NATION_ITALY) || e(KEY_NATION_ENGLAND) || e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SPAIN)) {
                let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_BASQUE_CLUB);
                if basco(clube) != 0 {
                    s = mul_exe_f64(s, 0x9689b8);
                }
                let tm = read_usize(G_TRANSFER_MANAGER);
                if tm != 0 {
                    let rr = read_i16(clube + club::REPUTATION);
                    let div: i32 = if rr < 0x3e8 { 0x3e8 } else { rr as i32 };
                    let tab = read_usize(read_usize(tm + 0x856) + 0xdc);
                    let x = read_i16(tab.wrapping_add((read_i32(clube) as usize).wrapping_mul(9)) + 5) as i32;
                    let mut m = x as f64 / div as f64;
                    if !(read_f64_at(0x967890) < m || read_f64_at(0x967890) == m) { // 0x967890 = 1
                        m = read_f64_at(0x967890); // 0x967890 = 1
                    } else if read_f64_at(0x968978) < m { // 0x968978 = 1.2
                        m = read_f64_at(0x968978); // 0x968978 = 1.2
                    }
                    s = match_pl_ftol(s as f64 * m);
                }
                // o contrato perto do fim (0x54cdb0: lea ecx, contrato + 0x2d; push [0xae2c94]; push [0xae2c90] ->
                // thiscall, ret 8), menos de 200 dias
                let perto = || {
                    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
                    contrato != 0 && read_usize(pessoa + staff::CLUB_JOB) == clube && dias(contrato + contract::CONTRACT_EXPIRES, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 0xc8
                };
                let rr = read_i16(clube + club::REPUTATION);
                let k: Option<usize> = if rr > 0x186a {
                    let np = read_usize(pessoa + staff::PLAYER);
                    if perto() {
                        let np = read_usize(pessoa + staff::PLAYER);
                        if read_i16(np + 0xd) > 0x1482 {
                            Some(0x968978)
                        } else {
                            let ax = read_i16(np + 0xb);
                            let cl = read_u8(contrato + contract::SQUAD_STATUS) & 0xf0;
                            if ax > 0x1c52 {
                                Some(0x9689c0)
                            } else if (cl == 0x30 && ax > 0x1482) || cl == 0x10 || cl == 0x20 {
                                Some(0x9689b8)
                            } else {
                                None
                            }
                        }
                    } else if read_i16(np + 0xd) > 0x1482 {
                        Some(0x9689c0)
                    } else if read_i16(np + 0xb) > 0x1a5e {
                        Some(0x9689b8)
                    } else {
                        None
                    }
                } else if rr > 0x109a {
                    if perto() {
                        let cx = read_i16(read_usize(pessoa + staff::PLAYER) + 0xb);
                        let al = read_u8(contrato + contract::SQUAD_STATUS) & 0xf0;
                        if cx > 0x1a5e {
                            Some(0x9689b8)
                        } else if (al == 0x30 && cx > 0x1482) || al == 0x10 || al == 0x20 {
                            Some(0x968f88)
                        } else {
                            None
                        }
                    } else if read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) > 0x1e46 {
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
            let pres = read_usize(clube + club::CHAIRMAN);
            if pres != 0 && read_usize(pres + 0x69) != 0 && (read_i16(clube + club::REPUTATION) as i32) < read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) as i32 + 0x4e2 {
                crate::comparacao::ramo("cqc_2 presidente");
                s = match_pl_ftol(((read_u8(pres + 0x57) as i8 as i32) as f64 * read_f64_at(0x967898) + read_f64_at(0x9689d8)) * s as f64); // cqc2: o presidente; 0x967898 = 0.01, 0x9689d8 = 0.9
                // 0x547810 (push pres; push pessoa; add esp, 8 -> cdecl), 0x547790 (push clube; push pessoa; add esp, 8 -> cdecl)
                let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
                let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
                if gosta(pessoa, pres) != 0 || favorito(pessoa, clube) != 0 {
                    s = match_pl_ftol(((read_u8(read_usize(clube + club::CHAIRMAN) + 0x57) as i8 as i32) as f64 * read_f64_at(0x967898) + read_f64_at(0x967890)) * s as f64); // 0x967898 = 0.01, 0x967890 = 1
                }
                let pres = read_usize(clube + club::CHAIRMAN);
                let pc = read_usize(pres + 0x69);
                let perfil = read_u8(pc + 0xf) as i8;
                if perfil < 6 {
                    let t = read_usize(clube + club::MANAGER);
                    let k = if t != 0 && (read_u8(t + 0x57) as i8) > 0xf && read_usize(t + 0x69) != 0 && read_i16(read_usize(t + 0x69) + 0xa) > 0x1c52 { 0x9689c0 } else { 0x9689b8 };
                    s = mul_exe_f64(s, k);
                } else if perfil > 0xf {
                    s = mul_exe_f64(s, 0x9689d0);
                }
                if read_u8(clube + club::PLC) == 0 && (read_u8(pc + 0x20) as i8) > 0xf && (read_u8(pres + 0x57) as i8) > 0xf {
                    let rr = read_i16(clube + club::REPUTATION);
                    let k = if rr >= 0xabe && read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) >= 0x1c52 {
                        if rr < 0x1676 { 0x9689c0 } else { 0x96a5a8 }
                    } else {
                        0x9689b8
                    };
                    s = mul_exe_f64(s, k);
                }
            }
            let c = cargo as i8;
            let np = read_usize(pessoa + staff::PLAYER);
            match c {
                1 => s = mul_exe_f64(s, 0x968928),
                2 => {
                    if is_human_club_with(clube, 1) == 0 && read_i16(np + 0xb) > 0x1a5e {
                        s = mul_exe_f64(s, 0x968928);
                    } else {
                        s = match_pl_ftol(s as f64);
                        let ok = is_human_club_with(clube, 1) != 0 || {
                            let np = read_usize(pessoa + staff::PLAYER);
                            np != 0 && read_i16(np + 0xb) < 0x1e46
                        };
                        if ok && s > 0x14c08 {
                            s = match_pl_ftol(s.wrapping_sub(0x14c08) as f64 * read_f64_at(0x968918) + read_f64_at(0x96a5a0)); // 0x968918 = 0.2, 0x96a5a0 = 85000
                        }
                    }
                }
                3 => {
                    if is_human_club_with(clube, 1) == 0 && read_i16(np + 0xb) > 0x1a5e {
                        s = mul_exe_f64(s, 0x968928);
                    } else {
                        s = mul_exe_f64(s, 0x9689d0);
                        if s > 0xd6d8 {
                            s = 0xd6d8;
                        }
                    }
                }
                4 => {
                    let k = if is_human_club_with(clube, 1) == 0 && read_i16(np + 0xb) > 0x1a5e { 0x9689d0 } else { 0x969040 };
                    s = mul_exe_f64(s, k);
                    if s > 0x927c {
                        s = 0x927c;
                    }
                }
                5 => {
                    s = if s > 0xc80 { match_pl_ftol(s.wrapping_sub(0xc80) as f64 * read_f64_at(0x968e80) + read_f64_at(0x96a598)) } else { mul_exe_f64(s, 0x968e48) }; // 0x968e80 = 0.3, 0x96a598 = 1920
                    if s > 0x30d4 {
                        s = 0x30d4;
                    }
                }
                6 => {
                    s = if s > 0xc80 { match_pl_ftol(s.wrapping_sub(0xc80) as f64 * read_f64_at(0x968f20) + read_f64_at(0x96a590)) } else { mul_exe_f64(s, 0x968968) }; // 0x968f20 = 0.15, 0x96a590 = 1600
                    if s > 0x203a {
                        s = 0x203a;
                    }
                }
                _ => {}
            }
        }
    } else {
        let x = (s as i16) as i32 as f64;
        s = match_pl_ftol(x * x * fator * read_f64_at(0x96a5c0) + read_i32(fin + 0x10) as f64); // 0x96a5c0 = 0.0001
        if s < b20 {
            s = match_pl_ftol(s as f64 * read_f64_at(0x969040) + b20 as f64 * read_f64_at(0x9689e0)); // 0x969040 = 0.75, 0x9689e0 = 0.25
        }
        if !fin_prm_is_national_team(clube) && nac != 0 {
            if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) {
                match wealth_band(fin) {
                    1 => s = mul_exe_f64(s, 0x9689b8),
                    2 => s = mul_exe_f64(s, 0x9689c0),
                    _ => {}
                }
            } else if e(KEY_NATION_ITALY) || e(KEY_NATION_ENGLAND) || e(KEY_NATION_SPAIN) {
                match wealth_band(fin) {
                    0 => {
                        if read_i16(clube + club::REPUTATION) > 0x109a {
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
    let tipo = read_u8(clube + club::PROFESSIONAL_STATUS);
    if tipo == 1 {
        if s < 0xfa {
            s = 0xfa;
        }
        if read_i16(clube + club::REPUTATION) < 0x8ca && s > 0x834 {
            s = match_pl_ftol((s as f64 - read_f32_at(0x96a5b8) as f64) * read_f64_at(0x968918) + read_f64_at(0x96a5b0)); // 0x96a5b8 = 2100, 0x968918 = 0.2, 0x96a5b0 = 2100
        }
    } else if tipo == 2 {
        s = mul_exe_f64(s, 0x969040);
        if read_i16(clube + club::REPUTATION) < 0xabe && s > 0x4b0 {
            s = match_pl_ftol((s as f64 - read_f32_at(0x96a588) as f64) * read_f64_at(0x968918) + read_f64_at(0x96a580)); // 0x96a588 = 1200, 0x968918 = 0.2, 0x96a580 = 1200
        }
        if s < 0x96 {
            s = 0x96;
        }
    }
    if s < liga.wrapping_add(100) {
        s = liga.wrapping_add(100);
    }
    if liga > r {
        return liga;
    }
    if s >= r { r } else { s }
}

/// Comparer for `get_max_wage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do teto salarial: pela devolução (a rotina só lê; as chamadas de dentro também).
pub(crate) unsafe extern "thiscall" fn comparar_get_max_wage(fin: usize, pessoa: usize, cargo: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_GET_MAX_WAGE)(fin, pessoa, cargo);
    let n = get_max_wage(fin, pessoa, cargo);
    PLACAR.registrar(r == n, || format!("get_max_wage(fin {:x}, pessoa {:x}, cargo {}) original={} rust={}", fin, pessoa, cargo as u8 as i8, r, n));
    r
}

/// news 0x177d with the club id, the amount and the works type, sent to the club manager
/// Original: `FinanceTable::news_ground_works` (0x59ffb0, fin_prm.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x59ffb0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_ground_works(clube, valor, tipo)`, pelo fim de temporada do clube (0x59cfa0: compra do estádio,
/// aquecimento, expansão e lugares, centro de treino) e pelos eventos da semana (0x59ed70). A notícia 0x177d (0x77bbe0:
/// lea ecx, local; push 0; push 0x177d -> thiscall, ret 8) com o id do clube, o valor e o tipo (byte com sinal; 0x785790:
/// lea ecx, local; 2 pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; push clube;
/// push &local -> thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn news_ground_works(_this: usize, clube: usize, valor: u32, tipo: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x177d, 0);
    gravar(p, 0, read_usize(clube) as u32);
    gravar(p, 1, valor);
    gravar(p, 2, tipo as u8 as i8 as i32 as u32);
    entregar(read_usize(G_NEWS), p, clube);
}

/// Comparer for `news_ground_works` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia das obras: só a original (ponytail: a notícia vai para a caixa do técnico humano e não pode
/// ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn comparar_news_ground_works(this: usize, clube: usize, valor: u32, tipo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_GROUND_WORKS)(this, clube, valor, tipo)
}

/// no club, nothing; news 0x1788 with the subject and the reason (2, or a draw of 2), sent to the club manager
/// Original: `FinanceTable::news_request_refused` (0x59fae0, fin_prm.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x59fae0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_request_refused(clube, assunto, motivo)`, pelo fim de temporada do clube (0x59cfa0: o pedido de
/// estádio ou de centro de treino recusado). Sem clube, nada. A notícia 0x1788 (0x77bbe0) com o assunto (palavra com
/// sinal) no dado 1 e no dado 2 o motivo 2 ou o sorteio(2) (0x90cfc0: push 2; add esp, 4 -> cdecl; a palavra com
/// sinal), entregue ao técnico do clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn news_request_refused(_this: usize, clube: usize, assunto: u32, motivo: u32) {
    if clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1788, 0);
    gravar(p, 1, assunto as u16 as i16 as i32 as u32);
    let v = if motivo as u16 == 2 { 2 } else { sortear(2) };
    gravar(p, 2, v as u16 as i16 as i32 as u32);
    entregar(read_usize(G_NEWS), p, clube);
}

/// Comparer for `news_request_refused` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da notícia da obra recusada: só a original (ponytail: entrega ao técnico humano e sorteia; não pode
/// rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_news_request_refused(this: usize, clube: usize, assunto: u32, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_REQUEST_REFUSED)(this, clube, assunto, motivo)
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
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("cpx_5 gravar financas");
    if save == 0 {
        return;
    }
    let mut z = abrir_item(save, 0x9c87b0, 1, 0x16, 4);
    if z == 0 {
        show_error_at(0x9c8784, 0x115e);
        return;
    }
    if gravar_item(read_usize(this), 0x167, club_count(), z) != club_count() { // cpx5: os registros das finanças
        show_error_at(0x9c8784, 0x1166);
        return;
    }
    fechar_item(&mut z);
}

/// Comparer for `save_finance_dat` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59b3b0: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_save_finance_dat(this: usize, save: usize) {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .qst gravado no modo rust com e sem a leva, item a item)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_FINANCE_DAT)(this, save)
}

/// reads the finances of every club from the finance.dat item into [this] and returns 1; no container 0; on error the
/// "Error" box (0x1137, 0x113f) and 0 (the constructor rebuilds the finances from scratch)
/// Original: `FinanceTable::load_from_save` (0x59b2a0, fin_prm.cpp, thiscall).
///
/// [pt] { 0x59b2a0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::load_from_save(save)`: lê do item `finance.dat`
/// (0x9326e0 lendo) as finanças de todos os clubes, 0x167 bytes × [0xae235c] em [this], e devolve 1. Sem contêiner,
/// 0 sem caixa; erro ao abrir ou leitura curta: a caixa "Error" (finance.CPP 0x1137, 0x113f), 0 e o item fica aberto
/// (como a original). Com 0, o construtor (0x59a55c) refaz as finanças de cada clube do zero (0x5961d0).
pub(crate) unsafe extern "thiscall" fn load_from_save(this: usize, save: usize) -> u32 {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("cpx_5 carregar financas");
    if save == 0 {
        return 0;
    }
    let mut z = abrir_item(save, 0x9c87b0, 0, 0x16, 4);
    if z == 0 {
        show_error_at(0x9c8784, 0x1137);
        return 0;
    }
    if ler_item(read_usize(this), 0x167, club_count(), z) != club_count() {
        show_error_at(0x9c8784, 0x113f);
        return 0;
    }
    fechar_item(&mut z);
    1 // cpx5: carregou (o construtor não refaz as finanças)
}

/// Comparer for `load_from_save` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59b2a0: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_load_from_save(this: usize, save: usize) -> u32 {
    // ponytail: lê do contêiner (a posição do item anda; não pode rodar duas vezes); a cópia é provada pela carga
    // do save de férias na prova do save (levas/N/CARGA)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_FROM_SAVE)(this, save)
}








/// Finance record of the match club at field (+0x1c home, +0x20 away), via the linked parent for B teams; Some(0)
/// without club, None for a national team.
///
/// [pt] O registro de finanças (0x167 bytes) do clube da partida em `campo` (+0x1c o mandante, +0x20 o visitante), como a
/// original o acha sem a 0x5a0590: o clube parceiro (0x540a50: 3 pushes; add esp, 0xc -> cdecl(clube, &a, 1)) se ele
/// existe e o clube é o B (a = 0), e [tabela] + id × 0x167. Some(0) sem clube; None se for seleção (0x53b900: push;
/// add esp, 4 -> cdecl): a original sai sem fazer nada.
pub(crate) unsafe fn match_club_finance(tabela: usize, partida: usize, campo: usize) -> Option<usize> {
    let clube = read_usize(partida + campo);
    if clube == 0 {
        return Some(0);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        return None;
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a = 0u32;
    let p = parceiro(clube, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { clube };
    Some(read_usize(tabela).wrapping_add((read_i32(c) as usize).wrapping_mul(0x167)))
}

/// Signed division truncated toward zero, as the original's magic-number divisions.
///
/// [pt] A divisão com o arredondamento para zero da original (os imul por constante mágica e o cdq/sar).
pub(crate) fn div_trunc(a: i32, b: i32) -> i32 {
    a.wrapping_div(b)
}

/// the TV money by attendance (1.75 × attendance), the gate receipts (+0x38 of the result), the match costs and the
/// home club expenses (with draws), the competition TV money by stage or type, divided by the nation type; in the home
/// club league, TV 3/4 and 1/4, the net to the home club and the stadium upkeep; outside it, half of each to both
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
pub(crate) unsafe extern "thiscall" fn process_match_income(tabela: usize, partida: usize, resultado: usize) {
    if resultado == 0 || partida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PROCESS_MATCH_INCOME)(tabela, partida, resultado); // a caixa
    }
    let Some(mandante) = match_club_finance(tabela, partida, 0x1c) else { return };
    let Some(visitante) = match_club_finance(tabela, partida, 0x20) else { return };
    let (mut tv, mut publico, mut renda) = (0i32, 0i32, 0i32);
    if read_i32(resultado + 0x34) != 0 {
        publico = read_i32(resultado + 0x34);
        renda = read_i32(resultado + 0x38);
        let q = div_trunc(1, friendly_random(10).wrapping_add(2));
        crate::comparacao::ramo("cqb_1 publico pagante");
        tv = match_pl_ftol((q as f64 + read_f64_at(0x968f98)) * publico as f64); // cqb1: a TV pelo público (1,75); 0x968f98 = 1.75
    }
    let casa = read_usize(partida + 0x1c);
    let fora = read_usize(partida + 0x20);
    let mut custo; // o ebp da original
    if read_u8(casa + 0x64) != 1 && read_u8(fora + 0x64) != 1 {
        custo = match_pl_ftol(publico as f64 * read_f64_at(0x9689e0)); // 0x9689e0 = 0.25
    } else {
        let est = read_usize(partida + 0x18);
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
            publico as f64 * read_f64_at(k)
        } else {
            let cap = read_i32(est + 0x3c);
            let k = if cap > 17500 {
                0x968fe0
            } else if cap > 12500 {
                0x968968
            } else if cap > 5000 {
                if publico < 5000 { 0x968e00 } else { 0x96a6a8 }
            } else {
                0x969510
            };
            cap as f64 * read_f64_at(k)
        };
        custo = match_pl_ftol(x);
        if read_u8(partida + 0x40) != 0 && custo < 500 {
            custo = 500;
        }
    }
    let casa = read_usize(partida + 0x1c);
    let mut gastos; // o [esp+0x10] da original (começa com o público)
    if read_u8(casa + 0x64) == 1 {
        let rep = read_i16(casa + 0x80);
        if rep != 0 {
            let r = friendly_random(0xc8);
            let x = (r as f64 + publico as f64 * read_f64_at(0x968928)) + rep as f64 * read_f64_at(0x968968) + read_f64_at(0x96a6b0); // 0x968928 = 1.25, 0x968968 = 0.5, 0x96a6b0 = 300
            gastos = match_pl_ftol(x);
            if rep > 0x128e {
                gastos = match_pl_ftol(gastos as f64 * read_f64_at(0x968928)); // 0x968928 = 1.25
            }
        } else {
            let r = friendly_random(0x190);
            gastos = match_pl_ftol((r as f64 + publico as f64 * read_f64_at(0x968928)) + read_f64_at(0x96a690)); // 0x968928 = 1.25, 0x96a690 = 600
        }
    } else {
        let rep = read_i16(casa + 0x80);
        let r1 = if rep != 0 { friendly_random(match_pl_ftol(rep as i32 as f64 * read_f64_at(0x9689e0))) } else { friendly_random(0xfa) }; // 0x9689e0 = 0.25
        let r1f = r1 as f32; // fstp dword: o primeiro sorteio guardado em float
        let r2 = friendly_random(0xfa);
        gastos = match_pl_ftol((r2 as f64 + r1f as f64) + publico as f64);
    }
    let mut dinheiro = 0i32; // o [esp+0x1c] da original
    let tipo_ok = match (read_u8(partida + 0x3b) as i8 as i32).wrapping_add(1) as u32 {
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
        _ => false, // 0 (fase −1) e fora da tabela: pelo tipo
    };
    if !tipo_ok {
        let t = read_u8(partida + 0x3a);
        if t == 8 || t == 7 {
            let soma = read_i16(casa + 0x80) as i32 + read_i16(fora + 0x80) as i32;
            dinheiro = if soma > 10000 {
                125000
            } else if soma <= 7500 {
                25000
            } else {
                75000
            };
        } else if t == 6 || t == 4 {
            let w = read_i16(read_usize(partida + 0x14) + 0x69);
            let m = if w > 5 { w as i32 } else { 5 };
            dinheiro = div_trunc(m.wrapping_mul(if t == 6 { 125000 } else { 25000 }), 20);
        }
    }
    let nac = read_usize(casa + 0x53);
    let tipo = if nac != 0 { read_u8(nac + 0x7e) as i8 } else { 3 };
    if tipo == 2 {
        dinheiro = div_trunc(dinheiro, 5);
        tv = div_trunc(tv, 2);
        custo = div_trunc(custo, 4);
        gastos = div_trunc(gastos, 2);
    } else if tipo != 1 {
        dinheiro = div_trunc(dinheiro, 10);
        tv = div_trunc(tv, 10);
        custo = div_trunc(custo, 15);
        gastos = div_trunc(gastos, 10);
    }
    if casa != 0 && nac != 0 {
        let n = read_i32(nac);
        if n == read_i32(KEY_NATION_WALES) || n == read_i32(KEY_NATION_REPUBLIC_OF_IRELAND) || n == read_i32(KEY_NATION_NORTHERN_IRELAND) {
            dinheiro = div_trunc(dinheiro, 5);
            tv = div_trunc(tv, 2);
            custo = div_trunc(custo, 4);
            gastos = div_trunc(gastos, 2);
        }
    }
    let liquido = tv.wrapping_sub(gastos).wrapping_sub(custo).wrapping_add(renda);
    let somar = |fin: usize, a: usize, b: usize, v: i32| {
        fin_prm_add_i32(fin + a, v);
        fin_prm_add_i32(fin + b, v);
        add_to_balance(fin, v as i64);
    };
    if read_u8(partida + 0x3c) == 0 && read_usize(read_usize(partida + 0x1c) + 0x57) == read_usize(partida + 0x14) {
        let x = dinheiro as f64;
        somar(mandante, 0x158, 0xb8, match_pl_ftol(x * read_f64_at(0x969040))); // 0x969040 = 0.75
        somar(visitante, 0x158, 0xb8, match_pl_ftol(x * read_f64_at(0x9689e0))); // 0x9689e0 = 0.25
        crate::comparacao::ramo("cqb_1 liquido ao mandante");
        somar(mandante, 0x140, 0xa0, liquido); // cqb1: o líquido ao mandante
        let casa = read_usize(partida + 0x1c);
        let est = read_usize(casa + 0x69);
        if read_u8(casa + 0x6d) != 0 && est != 0 {
            let base = read_i32(est + 0x3c);
            let r = friendly_random(0xf).wrapping_add(1);
            let c = match_pl_ftol((r as f64 * read_f64_at(0x967898) + read_f64_at(0x969520)) * base as f64); // 0x967898 = 0.01, 0x969520 = 0.85
            fin_prm_add_i32(mandante + 0x12c, c);
            fin_prm_add_i32(mandante + 0x8c, c);
            add_to_balance(mandante, -(c as i64));
        }
        return;
    }
    let metade = match_pl_ftol(dinheiro as f64 * read_f64_at(0x968968)); // 0x968968 = 0.5
    somar(mandante, 0x158, 0xb8, metade);
    somar(visitante, 0x158, 0xb8, metade);
    let l2 = div_trunc(liquido, 2);
    somar(mandante, 0x140, 0xa0, l2);
    somar(visitante, 0x140, 0xa0, l2);
}

/// Comparer for `process_match_income` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59a5d0: lado a lado nos dois registros de finanças e no estado do sorteio (os sorteios de 10, dos
/// gastos e da manutenção). Sem resultado, sem partida, sem um dos clubes ou com seleção: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_match_income(tabela: usize, partida: usize, resultado: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PROCESS_MATCH_INCOME);
    if resultado == 0 || partida == 0 || read_usize(partida + 0x1c) == 0 || read_usize(partida + 0x20) == 0 {
        return o(tabela, partida, resultado);
    }
    let (Some(fm), Some(fv)) = (match_club_finance(tabela, partida, 0x1c), match_club_finance(tabela, partida, 0x20)) else {
        return o(tabela, partida, resultado); // seleção: a original não faz nada
    };
    if fm == fv {
        return o(tabela, partida, resultado); // ponytail: o mesmo registro nos dois lados, só a original
    }
    let mut reg = vec![(fm, 0x167), (fv, 0x167)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tabela, partida, resultado), || crate::aleatorio::sem_contar(|| process_match_income(tabela, partida, resultado)));
    PLACAR.registrar(igual, || format!("process_match_income(partida {:x}) original={:x?} rust={:x?}", partida, mo, mr));
}

/// the gate receipts: the price by the home club reputation (/ 500; × 0.5 in friendlies, + 3 or + 4 in competitions) ×
/// attendance, less the finance discount in the home club league; at a neutral ground by the competition +0x69;
/// adjusted by nation type, league, key clubs and the nation at 0x9cf314
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
pub(crate) unsafe extern "thiscall" fn calc_gate_receipts(tabela: usize, partida: usize, publico: i32) -> i32 {
    if partida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_CALC_GATE_RECEIPTS)(tabela, partida, publico); // a caixa
    }
    if publico == 0 {
        return 0;
    }
    let Some(fin) = match_club_finance(tabela, partida, 0x1c) else { return 0 };
    let comp = read_usize(partida + 0x14);
    let mut v: i32;
    if read_u8(partida + 0x3c) != 0 && comp != 0 {
        v = (read_i16(comp + competition::CLUB_COMP_REPUTATION) as i32).wrapping_add(8).wrapping_mul(publico);
    } else {
        let casa = read_usize(partida + 0x1c);
        crate::comparacao::ramo("cqb_1 preco pela injury_word_0c");
        let preco = div_trunc(read_i16(casa + 0x80) as i32, 500); // cqb1: o preço pela reputação
        let amistoso = comp != 0 && {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
            f(read_usize(G_FRIENDLY), comp) != 0
        };
        if comp == 0 || amistoso {
            v = match_pl_ftol(preco as f64 * read_f64_at(0x968968) * publico as f64); // 0x968968 = 0.5
        } else if read_u8(casa + 0x64) == 1 || read_u8(read_usize(partida + 0x20) + 0x64) == 1 {
            v = preco.wrapping_add(4).wrapping_mul(publico);
        } else {
            v = preco.wrapping_add(3).wrapping_mul(publico);
        }
        if fin != 0 && read_usize(casa + 0x57) == comp {
            let desconto = *((fin + 0x160) as *const f32) as f64;
            v = match_pl_ftol(v as f64 - desconto * v as f64);
        }
    }
    let casa = read_usize(partida + 0x1c);
    let nac = read_usize(casa + 0x53);
    let tipo = if nac != 0 { read_u8(nac + 0x7e) as i8 } else { 3 };
    if tipo == 2 {
        v = div_trunc(v, 4);
    } else if tipo != 1 {
        v = div_trunc(v, 10);
    }
    let liga = read_usize(casa + 0x57);
    let k = if liga != 0 && read_i32(liga) == read_i32(KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        Some(0x968928)
    } else {
        let id = read_i32(casa);
        if id == read_i32(0x9d0250) {
            Some(0x968940)
        } else if id == read_i32(0x9d0110) {
            Some(0x968de8)
        } else if id == read_i32(0x9d0248) {
            Some(0x969040)
        } else if id == read_i32(0x9d0244) {
            Some(0x9689d8)
        } else if id == read_i32(0x9d024c) {
            Some(0x968de8)
        } else {
            None
        }
    };
    if let Some(k) = k {
        v = match_pl_ftol(v as f64 * read_f64_at(k));
    }
    if nac != 0 && read_i32(nac) == read_i32(KEY_NATION_GERMANY) {
        v = match_pl_ftol(v as f64 * read_f64_at(0x968978)); // 0x968978 = 1.2
    }
    v
}

/// Comparer for `calc_gate_receipts` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x59af40: a devolução (só lê; a 0x540a50 pode zerar a marca +0x244 do clube, igual nas duas). Sem
/// partida: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_calc_gate_receipts(tabela: usize, partida: usize, publico: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_CALC_GATE_RECEIPTS);
    if partida == 0 {
        return o(tabela, partida, publico);
    }
    let r = o(tabela, partida, publico);
    let n = calc_gate_receipts(tabela, partida, publico);
    PLACAR.registrar(r == n, || format!("calc_gate_receipts(partida {:x}, publico {}) original={} rust={}", partida, publico, r, n));
    r
}

/// adds the fine to the season (+0x128) and month (+0x88) fines and takes it from the balance
/// Original: `Finance::pay_league_fine` (0x599850, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x599850 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_league_fine(valor)`, pelas multas da liga (0x49f570,
/// 0x6a9e60): soma o valor às multas do ano (+0x128) e do mês (+0x88) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_league_fine(this: usize, valor: i32) {
    fin_prm_add_i32(this + 0x128, valor);
    fin_prm_add_i32(this + 0x88, valor);
    add_to_balance(this, -(valor as i64));
}

/// Comparer for `pay_league_fine` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x599850: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_league_fine(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_LEAGUE_FINE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_league_fine(this, valor));
    PLACAR.registrar(igual, || format!("pay_league_fine(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
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
    add_to_i32(fin + 0x154, v);
    add_to_i32(fin + 0xb4, v);
    credit_balance(fin, v);
}

/// True if the club's nation is Sweden, Norway or Denmark (event value drops to one third).
///
/// [pt] A nação do clube é uma das de 0x9cf4a8, 0x9cf408 ou 0x9cf2c0 (o valor do evento cai a um terço).
pub(crate) unsafe fn is_scandinavian_club(clube: usize) -> bool {
    let nac = read_usize(clube + club::NATION);
    if nac == 0 {
        return false;
    }
    let n = read_i32(nac);
    n == read_i32(KEY_NATION_SWEDEN) || n == read_i32(KEY_NATION_NORWAY) || n == read_i32(KEY_NATION_DENMARK)
}

/// outside seven nations and flags +0x64 2 and 3, the divisor by wealth band and reputation; with the chairman record,
/// three draws (against +0xf, the debt and 750 × +0x20) give (3 × (+0x20)² + 750) × reputation + 1 million plus the
/// balance; without it, two draws give (reputation + 1000) × 1000; a third in three nations; the money, the news by
/// type and the end of the crisis
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
pub(crate) unsafe extern "thiscall" fn try_finance_boost_event(this: usize, clube: usize, fin: usize) -> u32 {
    if clube != 0 {
        let nac = read_usize(clube + club::NATION);
        if nac != 0 {
            let n = read_i32(nac);
            if [KEY_NATION_WALES, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_NORWAY, KEY_NATION_DENMARK, KEY_NATION_SWEDEN, KEY_NATION_BELGIUM].iter().any(|&g| n == read_i32(g)) {
                return 0;
            }
            let m = read_u8(clube + club::PROFESSIONAL_STATUS);
            if m == 2 || m == 3 {
                return 0;
            }
        }
    }
    let rep = read_i16(clube + club::REPUTATION);
    let pres = read_usize(clube + club::CHAIRMAN);
    let forte = |a: usize, b: usize| a != 0 && b != 0; // (ponteiros conferidos antes dos bytes)
    let div = match wealth_band(fin) {
        -2 => read_f64_at(if rep > 0xea6 { 0x968fd0 } else { 0x96a730 }),
        -1 => read_f64_at(if rep > 0x1482 { 0x967880 } else if rep > 0xcb2 { 0x96a730 } else { 0x967878 }),
        0 => {
            if rep <= 0x109a {
                let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && read_i8(pres + 0x57) > 0xf && read_i8(ficha + 0x20) > 0xf {
                    read_f64_at(0x96a740) // 0x96a740 = 0.0125
                } else {
                    crate::comparacao::ramo("cqb_2 divisor da band 0 baixa");
                    read_f64_at(0x96a738) // cqb2: o divisor da faixa 0 de reputação baixa; 0x96a738 = 0.0075
                }
            } else if rep < 0x186a {
                let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && read_i8(ficha + 0x20) > 0xf && read_i8(pres + 0x57) > 0xf {
                    read_f64_at(0x967888) // 0x967888 = 0.05
                } else {
                    read_f64_at(0x967878) // 0x967878 = 0.025
                }
            } else {
                read_f64_at(0x967878) // 0x967878 = 0.025
            }
        }
        _ => read_f64_at(0x96a350), // 0x96a350 = 0.001
    };
    let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
    let valor;
    if pres != 0 && ficha != 0 {
        crate::comparacao::ramo("cqb_2 com a ficha");
        let r1 = friendly_random(match_pl_ftol(read_f64_at(0x968dd8) / div)); // cqb2: o sorteio com a ficha; 0x968dd8 = 30
        let r2 = friendly_random(read_i8(ficha + 0xf) as i32);
        if r2 <= r1 {
            return 0;
        }
        let (lo, hi) = balance_halves(fin);
        let n = read_i8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as i32;
        let r3 = friendly_random(n.wrapping_mul(1_000_000)) as i64;
        let divida = ((hi as i64) << 32 | lo as i64).wrapping_neg();
        if r3 <= divida {
            return 0;
        }
        let rep = rep as i32;
        let n = read_i8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as i32;
        if rep >= n.wrapping_mul(750) {
            return 0;
        }
        // ponytail: a original faz pow(+0x20, 2,0) (0x945ff0); o quadrado de um byte é exato nos dois
        let mut v = match_pl_ftol(((n as f64 * n as f64) * read_f64_at(0x968930) + read_f64_at(0x96a5f0)) * rep as f64 + read_f64_at(0x9688d8)); // 0x968930 = 3, 0x96a5f0 = 750, 0x9688d8 = 1000000
        v = if hi < 0 { v.wrapping_sub(lo as i32) } else { v.wrapping_add(lo as i32) };
        if read_i16(clube + club::REPUTATION) < 0x7d0 {
            let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
            if v > teto {
                v = teto;
            }
        }
        if is_scandinavian_club(clube) {
            v /= 3;
        }
        valor = v;
    } else {
        crate::comparacao::ramo("cqb_2 sem a ficha");
        let r = friendly_random(match_pl_ftol(read_f64_at(0x96a728) / div)); // cqb2: o sorteio sem a ficha; 0x96a728 = 15000
        if rep as i32 <= r {
            return 0;
        }
        if friendly_random(8) != 0 {
            return 0;
        }
        let mut v = (read_i16(clube + club::REPUTATION) as i32).wrapping_add(1000).wrapping_mul(1000);
        if is_scandinavian_club(clube) {
            v /= 3;
        }
        valor = v;
    }
    credit_income(fin, valor);
    let verba = transfer_budget(fin) as i32;
    let marca = read_u8(clube + club::PLC);
    let com_ficha = pres != 0 && ficha != 0;
    let mut tipo = 0u32;
    if verba <= 0 && marca == 0 {
        tipo = 3;
    } else if marca != 0 && friendly_random(4) == 0 {
        tipo = 5;
    }
    if tipo == 0 {
        if friendly_random(4) == 0 {
            if com_ficha {
                let r2 = read_i16(clube + club::REPUTATION) as i32;
                let t = match_pl_ftol(r2.wrapping_mul(r2) as f64 * read_f64_at(0x968e80)); // 0x968e80 = 0.3
                if valor < t && valor > 50000 {
                    tipo = 6;
                }
            } else {
                tipo = 6;
            }
        }
    }
    if tipo == 0 {
        let nac = read_usize(clube + club::NATION);
        if read_u8(clube + club::PLC) == 0 && read_i16(clube + club::REPUTATION) > 0x128e && friendly_random(4) == 0
            && (!com_ficha || (nac != 0 && read_i32(nac) != read_i32(KEY_NATION_ITALY)))
        {
            post_finance_boost_news(this, clube, valor, verba as u32, 7);
            *((clube + club::PLC) as *mut u8) = 1;
            tipo = 7;
        } else {
            tipo = 1;
        }
    }
    if tipo != 7 {
        post_finance_boost_news(this, clube, valor, verba as u32, tipo);
    }
    if com_ficha {
        let f = read_usize(read_usize(clube + club::CHAIRMAN) + 0x69);
        let x = read_i8(f + 0x20);
        if x > 5 {
            *((f + 0x20) as *mut i8) = x - 1;
        }
    }
    *((fin + 0x165) as *mut u8) = 0;
    1
}

/// Rehearsal comparer for rare finance events: copy without news/budget vs original from the same state, comparing
/// return, balance, +0xb4, +0x154, +0x165 and extra regions.
///
/// [pt] O comparador dos eventos raros, em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN: a cópia sem a notícia nem a verba, que alocam; sem a
/// troca do presidente na 0x59e940): a cópia e a original a partir do mesmo estado (o registro de finanças, o sorteio e
/// os `extras`). Sem evento: tudo igual; com evento: a devolução, o saldo, +0xb4, +0x154, +0x165 e os `conferir` (a
/// tipo da notícia e a marca +0x82 dependem da verba, e os sorteios depois dela também).
pub(crate) unsafe fn compare_rare_finance_event(nome: &str, fin: usize, extras: &[(usize, usize)], conferir: &[usize], original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend_from_slice(extras);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    RARE_FINANCE_EVENT_DRY_RUN = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(copia));
    RARE_FINANCE_EVENT_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if r == 0 && n == 0 {
        mo == mr
    } else {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let campos = r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165];
        campos && conferir.iter().all(|&k| mo.0[3 + k].1 == mr.0[3 + k].1)
    };
    PLACAR.registrar(igual, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", nome, fin, r, mo, n, mr));
    r
}

/// Comparer for `try_finance_boost_event` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_finance_boost_event(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_FINANCE_BOOST_EVENT);
    let pres = if clube != 0 { read_usize(clube + club::CHAIRMAN) } else { 0 };
    let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // ponytail: sem clube ou sem finanças, só a original
    }
    let mut extras = vec![(clube + club::PLC, 1)];
    let mut conferir = Vec::new();
    if ficha != 0 {
        extras.push((ficha + 0x20, 1)); // o +0x20 da ficha cai em todo evento com ela
        conferir.push(1);
    }
    compare_rare_finance_event("try_finance_boost_event", fin, &extras, &conferir, || o(this, clube, fin), || try_finance_boost_event(this, clube, fin))
}

/// the divisor by band and the factor by reputation and league; draws by the chairman record and by 15000 / divisor;
/// (reputation + 1000) × 1000 less the balance (or just what clears the balance); with the record, the new chairman
/// (drawn nation, names and record); the news (type 2) and the end of the crisis
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
pub(crate) unsafe extern "thiscall" fn try_board_takeover(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = read_i16(clube + club::REPUTATION);
    let div = match wealth_band(fin) {
        -2 => if rep > 0x109a { 0.4f64 } else { 0.25 },
        -1 => if rep > 0x109a { 0.2 } else { 0.05 },
        _ => 0.001,
    };
    let liga = read_usize(clube + club::DIVISION);
    let mut fator = if liga != 0 {
        let k = rep as i32 as f64 * read_f64_at(0x969048) / read_i16(liga + 0x69) as i32 as f64; // 0x969048 = 0.002
        if !(k >= read_f64_at(0x969040)) { read_f64_at(0x969040) } else { k } // fcomp: menor ou sem ordem (NaN) vira 0,75; 0x969040 = 0.75
    } else {
        0.5
    };
    let pres = read_usize(clube + club::CHAIRMAN);
    let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
    let com_ficha = pres != 0 && ficha != 0;
    let primeiro = if com_ficha { friendly_random(read_i8(ficha + 0x16) as i32) } else { friendly_random(0x14) };
    if primeiro != 0 {
        return 0;
    }
    fator *= read_i16(clube + club::REPUTATION) as i32 as f64;
    let r = friendly_random(match_pl_ftol(read_f64_at(0x96a728) / div)); // 0x96a728 = 15000
    if r as f64 >= fator {
        return 0;
    }
    crate::comparacao::ramo("cqb_2 troca do presidente");
    let (lo, _) = balance_halves(fin);
    let mut v = (read_i16(clube + club::REPUTATION) as i32).wrapping_add(1000).wrapping_mul(1000).wrapping_sub(lo as i32);
    if v > 0x2625a00 {
        let s = friendly_random(0x9c4);
        if (read_i16(clube + club::REPUTATION) as i32) < s.wrapping_add(0x7d0) {
            v = (balance_halves(fin).0 as i32).wrapping_neg();
        }
    }
    add_to_i32(fin + 0xb4, v);
    add_to_i32(fin + 0x154, v);
    credit_balance(fin, v);
    let verba = transfer_budget(fin);
    if com_ficha && !RARE_FINANCE_EVENT_DRY_RUN {
        let pres = read_usize(clube + club::CHAIRMAN);
        let nac = read_usize(clube + club::NATION);
        let nova = if nac != 0 && friendly_random(0x3e8) != 0 {
            nac
        } else {
            let k = friendly_random(read_i32(0xae2350));
            read_usize(G_NATION_TABLE).wrapping_add((k as usize).wrapping_mul(nation::SIZE))
        };
        core::ptr::write_unaligned((pres + 0x1a) as *mut usize, nova);
        // 0x539c60, 0x53a5b0, 0x53a1b0: push nação; call (add esp, 0xc depois das três) -> cdecl(nação)
        let nome1: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
        let nome3: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_EMPTY_COMMON_NAME);
        let nome2: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_FOR_NATION);
        let x = nome1(read_usize(read_usize(clube + club::CHAIRMAN) + 0x1a));
        core::ptr::write_unaligned((read_usize(clube + club::CHAIRMAN) + 4) as *mut u32, x);
        let x = nome3(read_usize(read_usize(clube + club::CHAIRMAN) + 0x1a));
        core::ptr::write_unaligned((read_usize(clube + club::CHAIRMAN) + 0xc) as *mut u32, x);
        let x = nome2(read_usize(read_usize(clube + club::CHAIRMAN) + 0x1a));
        core::ptr::write_unaligned((read_usize(clube + club::CHAIRMAN) + 8) as *mut u32, x);
        if read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) != 0 {
            for o in [0x1dusize, 0xf, 0x16, 0x20] {
                let x = (friendly_random(0x14) as u8).wrapping_add(1);
                *((read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + o) as *mut u8) = x;
            }
            if read_i8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) < 5 {
                let x = (friendly_random(0x14) as u8).wrapping_add(1);
                *((read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as *mut u8) = x;
            }
        }
    }
    post_finance_boost_news(this, clube, v, verba, 2);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

/// Comparer for `try_board_takeover` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_board_takeover(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_TAKEOVER);
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // ponytail: sem clube ou sem finanças, só a original
    }
    compare_rare_finance_event("try_board_takeover", fin, &[], &[], || o(this, clube, fin), || try_board_takeover(this, clube, fin))
}

/// adds the amount to the season (+0x120) and month (+0x80) other costs and takes it from the balance
/// Original: `Finance::pay_other_costs` (0x5999e0, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x5999e0 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_other_costs(valor)`, pela 0x7e5b10: soma o valor aos
/// outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_other_costs(this: usize, valor: i32) {
    add_to_i32(this + 0x120, valor);
    add_to_i32(this + 0x80, valor);
    add_to_balance(this, -(valor as i64));
}

/// Comparer for `pay_other_costs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5999e0: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_other_costs(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_OTHER_COSTS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_other_costs(this, valor));
    PLACAR.registrar(igual, || format!("pay_other_costs(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}






/// sums the squad wages (less the other club's share for loanees) and the staff wages into the wage bill (+0xc) and the
/// highest wage (+0x10); with fewer than 15 players, the missing places at a wage by reputation; plus reputation² × a
/// factor by club type and nation; the wage ceiling (+0x14) by 0x5968d0
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
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32, u32) -> f64 = core::mem::transmute(FN_STAFF_CONTRACT_DEFAULT_STAFF_CONTRACT);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let teto: unsafe extern "thiscall" fn(usize, u32, i32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    write_i32(this + 0x10, 0);
    write_i32(this + 0xc, 0);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(club::SIZE));
    let mut ocupadas: i8 = 0;
    for k in 0..50usize {
        let p = read_usize(clube + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        ocupadas = ocupadas.wrapping_add(1);
        let c = com_clube(read_usize(G_CONTRACT_MANAGER), p, clube);
        if c == 0 {
            continue;
        }
        let mut w = read_i32(c + 0xc);
        if w == -1 {
            w = match_pl_ftol(padrao(c, p, clube, 0, -1, 0xb, 0));
        }
        if ficha(read_usize(G_CONTRACT_MANAGER), p) != 0 && read_u8(c + 0x35) & 0x3f != 6 {
            w = w.wrapping_sub(read_i32(ficha(read_usize(G_CONTRACT_MANAGER), p) + 0xc)); // a parte do outro contrato
        }
        if w > read_i32(this + 0x10) {
            write_i32(this + 0x10, w);
        }
        write_i32(this + 0xc, read_i32(this + 0xc).wrapping_add(w));
    }
    // o pessoal: a soma vai à folha depois das três listas (a mesma soma, em 32 bits)
    let mut comissao = 0i32;
    for (base, n) in [(0x19fusize, 5usize), (0x1b3, 7), (0x1cf, 3)] {
        for k in 0..n {
            let p = read_usize(clube + base + 4 * k);
            if p == 0 {
                continue;
            }
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
            if c == 0 {
                continue;
            }
            let w = read_i32(c + 0xc);
            if w > read_i32(this + 0x10) {
                write_i32(this + 0x10, w);
            }
            comissao = comissao.wrapping_add(w);
        }
    }
    if comissao != 0 {
        crate::comparacao::ramo("cpz_11 folha do pessoal");
        write_i32(this + 0xc, read_i32(this + 0xc).wrapping_add(comissao)); // cpz11: o pessoal na folha
    }
    let tipo1 = read_u8(clube + club::PROFESSIONAL_STATUS) == 1;
    let rep = read_i16(clube + club::REPUTATION) as i32;
    if ocupadas < 15 {
        let mut v = if rep > 0x128e {
            rep - 0x79e
        } else if rep > 0x8ca {
            match_pl_ftol((rep - 0x8ca) as f64 * read_f64_at(0x968968) + read_f64_at(0x96a620)) // 0x968968 = 0.5, 0x96a620 = 550
        } else {
            match_pl_ftol(rep as f64 * read_f64_at(0x9689e0)) // 0x9689e0 = 0.25
        };
        if !tipo1 {
            v = match_pl_ftol(v as f64 * read_f64_at(0x968968)); // 0x968968 = 0.5
        }
        let nacao = read_usize(clube + club::NATION);
        if nacao != 0 {
            let t = (read_u8(nacao + nation::STATE_OF_DEVELOPMENT) as i8).clamp(1, 3) as i32;
            v = v.wrapping_div(t);
        }
        let falta = 15 - ocupadas as i32;
        write_i32(this + 0xc, read_i32(this + 0xc).wrapping_add(falta.wrapping_mul(v)));
    }
    let k = if tipo1 {
        let nacao = read_usize(clube + club::NATION);
        let grande = nacao != 0 && (read_i32(nacao) == read_i32(KEY_NATION_ENGLAND) || read_i32(nacao) == read_i32(KEY_NATION_ITALY));
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
    crate::comparacao::ramo("cpz_11 folha pela injury_word_0c");
    let folha = match_pl_ftol(rep.wrapping_mul(rep) as f64 * read_f64_at(k) + read_i32(this + 0xc) as f64); // cpz11: a reputação na folha
    write_i32(this + 0xc, folha);
    let t = teto(this, 0, -1);
    write_i32(this + 0x14, t);
}

/// Comparer for `update_wage_bill` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na folha (+0xc), no maior salário (+0x10), no teto (+0x14) e no último contrato consultado
/// ([0xae1080], que o salário padrão 0x864550 grava); as outras chamadas (contratos, teto 0x5968d0) só leem.
pub(crate) unsafe extern "thiscall" fn comparar_update_wage_bill(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WAGE_BILL);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 12), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this), || update_wage_bill(this));
    PLACAR.registrar(igual, || format!("update_wage_bill(fin {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// at game creation, the wage bill of every club up to the first national team
/// Original: `FinanceTable::refresh_all_wage_bills` (0x59cf50, finance.cpp, thiscall).
///
/// [pt] { 0x59cf50 } (thiscall, finance.cpp): `FinanceTable::refresh_all_wage_bills()`, só pela criação do jogo (0x828110):
/// para cada clube de [0xae23b4] até a primeira seleção (0x53b900), a folha (0x598370) do registro de finanças dele
/// ([this] + i × 0x167).
pub(crate) unsafe extern "thiscall" fn refresh_all_wage_bills(this: usize) {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    let mut fin = read_usize(this);
    let mut i = 0i32;
    while i < read_i32(0xae235c) {
        let clube = read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE));
        if selecao(clube) != 0 {
            break;
        }
        folha(fin);
        fin = fin.wrapping_add(0x167);
        i += 1;
    }
}

/// Comparer for `refresh_all_wage_bills` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na tabela de finanças inteira ([this], [0xae235c] × 0x167 bytes) e em [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_refresh_all_wage_bills(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REFRESH_ALL_WAGE_BILLS);
    let n = (read_i32(0xae235c).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(read_usize(this), n), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this), || refresh_all_wage_bills(this));
    PLACAR.registrar(igual, || format!("refresh_all_wage_bills({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// the days until the season start of the club's nation, inside the season ticket window (up to two months before);
/// flags whether the season year has come
/// Original: `finance_get_days_to_season_start` (0x59b4c0, finance.cpp, thiscall, ret 8 (o ecx não é lido)).
///
/// [pt] { 0x59b4c0 } (ret 8, finance.cpp): `finance_get_days_to_season_start(clube, &passou)`, pelo passe diário das
/// finanças (0x59b9c0; ecx com a tabela, que ela não lê): dias até o começo da temporada da nação do clube (linha de
/// 72 bytes em 0xb63d60 pela 0x669360; sem linha, a caixa "Error" da linha 0x1185 e 0). `passou` = 1 se o ano de hoje
/// chega ao ano da temporada (2002 com +0x17, senão o ano de [0xdd6ec8] + 1). Mês do começo (+0x34) abaixo de 2: no ano
/// do começo (+0x2c), só depois do mês 12 − mês do começo; noutro ano, só antes do mês do começo; senão, só a 0..2
/// meses do começo. Na janela, os dias até a data do começo (+0x2a, 0x54cdb0) se positivos; fora, 0.
pub(crate) unsafe extern "thiscall" fn finance_get_days_to_season_start(_this: usize, clube: usize, passou: *mut u32) -> i32 {
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    *passou = 0;
    let k = linha(clube) as i8;
    if k < 0 {
        show_error_at(0x9c8784, 0x1185);
        return 0;
    }
    let e = 0xb63d60usize.wrapping_add((k as isize * 72) as usize);
    let ano: u16 = if read_usize(e + 0x17) != 0 { 0x7d2 } else { (read_i16(G_GAME_START_DATE_YEAR) as u16).wrapping_add(1) };
    if read_i16(G_CURRENT_DATE_YEAR) as u16 >= ano {
        *passou = 1;
    }
    let inicio = read_u8(e + 0x34) as i8;
    let ano_do_inicio = read_i16(e + 0x2c) as u16;
    let mes = mes_de(G_CURRENT_DATE) as i8;
    let janela = if inicio < 2 {
        if ano_do_inicio == read_i16(G_CURRENT_DATE_YEAR) as u16 {
            mes as i32 > 12 - inicio as i32
        } else {
            mes < inicio
        }
    } else {
        let d = inicio.wrapping_sub(mes);
        (0..=2).contains(&d)
    };
    if !janela {
        return 0;
    }
    let r = dias_ate(e + 0x2a, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
    if r > 0 {
        crate::comparacao::ramo("cpz_11 dias ate a temporada");
        return r; // cpz11: os dias até a temporada
    }
    0
}

/// Comparer for `finance_get_days_to_season_start` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem linha da nação, só a original (a caixa); senão lado a lado na saída `passou` e na data estática da
/// 0x54cdb0 (0xae2470), com a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_finance_get_days_to_season_start(this: usize, clube: usize, passou: *mut u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(IDX_FINANCE_GET_DAYS_TO_SEASON_START);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    if (linha(clube) as i8) < 0 {
        return o(this, clube, passou);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(passou as usize, 4), (G_DAYS_SINCE_SCRATCH_DATE, 8)], || o(this, clube, passou), || finance_get_days_to_season_start(this, clube, passou));
    PLACAR.registrar(igual && r == n, || format!("finance_get_days_to_season_start(clube {:#x}) original={} {:x?} rust={} {:x?}", clube, r, mo, n, mr));
    r
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
pub(crate) fn i64_from_halves(baixo: u32, alto: u32) -> i64 {
    ((alto as u64) << 32 | baixo as u64) as i64
}

/// adds the amount (64 bits) to the sales of the month (+0x78) and of the year (+0x118) and to the balance
/// Original: `Finance::add_players_sold` (0x599800, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x599800 } (thiscall, ret 8, finance.cpp): `Finance::add_players_sold(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0 e as parcelas 0x8caf.., 0x8cb3..) e pela
/// parte dos clubes anteriores (0x8e4f20): soma o valor às vendas do mês (+0x78) e do ano (+0x118), em 64 bits, e ao
/// saldo (i64 em +0).
pub(crate) unsafe extern "thiscall" fn add_players_sold(this: usize, baixo: u32, alto: u32) {
    let v = i64_from_halves(baixo, alto);
    fin_prm_add_i64(this + 0x78, v);
    fin_prm_add_i64(this + 0x118, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 venda recebida"); // só valor diferente de zero muda o saldo
    }
    fin_prm_add_i64(this, v); // cpz12: a venda entra no saldo
}

/// Comparer for `add_players_sold` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_add_players_sold(this: usize, baixo: u32, alto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_ADD_PLAYERS_SOLD);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || add_players_sold(this, baixo, alto));
    PLACAR.registrar(igual, || format!("add_players_sold(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(baixo, alto), mo, mr));
}

/// adds the amount (64 bits) to the purchases of the month (+0x70) and of the year (+0x110) and takes it from the
/// balance
/// Original: `Finance::pay_players_bought` (0x599890, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x599890 } (thiscall, ret 8, finance.cpp): `Finance::pay_players_bought(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (0x8ca8a0) e pelas parcelas (0x8caf.., 0x8cb3..): soma o valor às compras do mês (+0x70) e do ano
/// (+0x110), em 64 bits, e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_players_bought(this: usize, baixo: u32, alto: u32) {
    let v = i64_from_halves(baixo, alto);
    fin_prm_add_i64(this + 0x70, v);
    fin_prm_add_i64(this + 0x110, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 compra paga"); // só valor diferente de zero muda o saldo
    }
    fin_prm_add_i64(this, v.wrapping_neg()); // cpz12: a compra sai do saldo
}

/// Comparer for `pay_players_bought` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_players_bought(this: usize, baixo: u32, alto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_PAY_PLAYERS_BOUGHT);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || pay_players_bought(this, baixo, alto));
    PLACAR.registrar(igual, || format!("pay_players_bought(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(baixo, alto), mo, mr));
}

/// adds the amount to the signing-on fees of the month (+0x9c) and of the year (+0x13c) and takes it from the balance
/// Original: `Finance::pay_sign_on_fee` (0x5998e0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5998e0 } (thiscall, ret 4, finance.cpp): `Finance::pay_sign_on_fee(valor)`, pela oferta de contrato aceita
/// (0x8c38b0) e pela mudança do jogador de clube (0x8c9720): soma o valor às luvas do ano (+0x13c) e do mês (+0x9c) e o
/// tira do saldo (estendido com sinal).
pub(crate) unsafe extern "thiscall" fn pay_sign_on_fee(this: usize, valor: i32) {
    fin_prm_add_i32(this + 0x13c, valor);
    fin_prm_add_i32(this + 0x9c, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_12 luvas pagas"); // só valor diferente de zero muda o saldo
    }
    fin_prm_add_i64(this, (valor as i64).wrapping_neg()); // cpz12: as luvas saem do saldo
}

/// Comparer for `pay_sign_on_fee` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_sign_on_fee(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_SIGN_ON_FEE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_sign_on_fee(this, valor));
    PLACAR.registrar(igual, || format!("pay_sign_on_fee(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}


/// Adds v to the i64 balance at +0 of a finance record.
///
/// [pt] Soma `v` (estendido com sinal: o cdq/adc da original) ao saldo, i64 em +0.
pub(crate) unsafe fn add_to_balance(fin: usize, v: i64) {
    core::ptr::write_unaligned(fin as *mut i64, core::ptr::read_unaligned(fin as *const i64).wrapping_add(v));
}

/// adds the amount to the prize money of the month (+0xa8) and of the year (+0x148) and to the balance
/// Original: `Finance::add_prize_money` (0x5999a0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5999a0 } (thiscall, ret 4, finance.cpp): `Finance::add_prize_money(valor)`, pelas copas (Cup::process_fixture
/// 0x51a150, EuropeanCup::process_fixture 0x583b10, os sorteios das fases), pelo fim das ligas (League::pay_prize_money
/// 0x682e10) e pelas honras da temporada: soma o valor aos prêmios do ano (+0x148) e do mês (+0xa8) e ao saldo.
pub(crate) unsafe extern "thiscall" fn add_prize_money(this: usize, valor: i32) {
    fin_prm_add_i32(this + 0x148, valor);
    fin_prm_add_i32(this + 0xa8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 premio recebido"); // só valor diferente de zero muda o saldo
    }
    add_to_balance(this, valor as i64); // cpz13: o prêmio entra no saldo
}

/// Comparer for `add_prize_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_add_prize_money(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_PRIZE_MONEY);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || add_prize_money(this, valor));
    PLACAR.registrar(igual, || format!("add_prize_money(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// adds the amount to the TV income of the month (+0xb8) and of the year (+0x158) and to the balance
/// Original: `Finance::add_tv_revenue` (0x5987c0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5987c0 } (thiscall, ret 4, finance.cpp): `Finance::add_tv_revenue(valor)`, só pelos direitos de TV da liga
/// (League::pay_tv_rights 0x683010, pela montagem da tabela de cada liga na temporada nova): soma o valor à TV do ano
/// (+0x158) e do mês (+0xb8) e ao saldo.
pub(crate) unsafe extern "thiscall" fn add_tv_revenue(this: usize, valor: i32) {
    fin_prm_add_i32(this + 0x158, valor);
    fin_prm_add_i32(this + 0xb8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 tv recebida"); // só valor diferente de zero muda o saldo
    }
    add_to_balance(this, valor as i64); // cpz13: a TV entra no saldo
}

/// Comparer for `add_tv_revenue` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_add_tv_revenue(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_TV_REVENUE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || add_tv_revenue(this, valor));
    PLACAR.registrar(igual, || format!("add_tv_revenue(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// adds the amount to the wages of the month (+0x90) and of the year (+0x130) and takes it from the balance (the free
/// release compensation)
/// Original: `Finance::pay_wages` (0x599920, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599920 } (thiscall, ret 4, finance.cpp): `Finance::pay_wages(valor)`, só pela liberação sem custo
/// (TRANSFER_MANAGER::free_transfer_staff 0x8c1af0: a compensação do contrato ao clube): soma o valor aos salários do ano
/// (+0x130) e do mês (+0x90) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pay_wages(this: usize, valor: i32) {
    fin_prm_add_i32(this + 0x130, valor);
    fin_prm_add_i32(this + 0x90, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 salario pago"); // só valor diferente de zero muda o saldo
    }
    add_to_balance(this, (valor as i64).wrapping_neg()); // cpz13: o salário sai do saldo
}

/// Comparer for `pay_wages` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_wages(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_WAGES);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_wages(this, valor));
    PLACAR.registrar(igual, || format!("pay_wages(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
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
pub(crate) unsafe fn fin_prm_club_finance(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add((read_usize(clube) as u32).wrapping_mul(0x167) as usize)
}

/// attendance × reputation × factor (by the stadium of the +0xbf club or by reputation; Japan one step up), ×
/// (random(11) + 95)%, added to +0xbc, +0x15c and the balance
/// Original: `FinanceTable::add_weekly_merchandising_income` (0x59b7e0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x59b7e0 } (thiscall, ret 4, finance.cpp): `FinanceTable::add_weekly_merchandising_income(clube)`, pelo passe
/// diário das finanças (0x59b9c0, no dia 6 da semana, clubes de liga). A base: com o clube de +0xbf e o estádio dele
/// (+0x69), o nível (+0xf) × 25 + a reputação (+0x80), × 0,65 truncado se o byte +0x6d for zero, escolhe o fator por
/// 5500 e 4000; sem eles, a reputação escolhe por 5250 e 3750 (um degrau acima na nação do Japão 0x9cf36c). Público
/// (+0x73) × reputação × fator, truncado; × (sorteio(11) + 95) × 0,01, truncado, soma em +0xbc, +0x15c e no saldo.
pub(crate) unsafe extern "thiscall" fn add_weekly_merchandising_income(this: usize, clube: usize) {
    let outro = read_usize(clube + club::CHAIRMAN);
    let est = if outro != 0 { read_usize(outro + 0x69) } else { 0 };
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let fator = if est != 0 {
        let s = (read_u8(est + 0xf) as i8 as i32).wrapping_mul(25);
        let mut v = rep.wrapping_add(s);
        if read_u8(clube + club::OWN_STADIUM) == 0 {
            v = match_pl_ftol(v as f64 * read_f64_at(0x96a5d0)); // 0x96a5d0 = 0.65
        }
        if v > 0x157c {
            0x96a6d0
        } else if v > 0xfa0 {
            0x96a6c8
        } else {
            0x96a6b8
        }
    } else {
        let nac = read_usize(clube + club::NATION);
        let japao = nac != 0 && read_i32(nac) == read_i32(KEY_NATION_JAPAN);
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
    let base = match_pl_ftol(read_i32(clube + club::ATTENDANCE) as f64 * rep as f64 * read_f64_at(fator));
    crate::comparacao::ramo("cpv_5 merchandising");
    let r = friendly_random(0xb); // cpv5: o sorteio do merchandising
    let v = match_pl_ftol((r + 0x5f).wrapping_mul(base) as f64 * read_f64_at(0x967898)); // 0x967898 = 0.01
    let fin = fin_prm_club_finance(this, clube);
    add_to_i32(fin + 0x15c, v);
    add_to_i32(fin + 0xbc, v);
    add_i32_to_balance(fin, v);
}

/// Comparer for `add_weekly_merchandising_income` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças do clube e no estado do sorteio.
pub(crate) unsafe extern "thiscall" fn comparar_add_weekly_merchandising_income(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_WEEKLY_MERCHANDISING_INCOME);
    let mut regioes = vec![(fin_prm_club_finance(this, clube), 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || crate::aleatorio::sem_contar(|| add_weekly_merchandising_income(this, clube)));
    PLACAR.registrar(igual, || format!("add_weekly_merchandising_income(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// stadium of at least 1500 places; demand by the manager-table value, attendance and reputation, up to 87.5% of the
/// places; new holders by the step fraction (+0x164) and a random draw; ratio at +0x160; income (with apply) by price,
/// nation type and key clubs; the shop with two random draws by reputation
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
pub(crate) unsafe extern "thiscall" fn sell_season_tickets(this: usize, aplicar: i32) {
    let id = read_usize(this + 8);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE));
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_SELL_SEASON_TICKETS)(this, aplicar);
    }
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let est = read_usize(clube + club::STADIUM);
    if est != 0 && read_i32(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, match_pl_ftol(rep as f64 * read_f64_at(0x968e18))); // 0x968e18 = 2.5
        let est = read_usize(clube + club::STADIUM);
        if read_i32(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    let gerente = read_usize(read_usize(G_MANAGER_MANAGER));
    let valor = read_i16(gerente.wrapping_add((read_usize(clube) as u32).wrapping_mul(73) as usize) + 8);
    let mut f = valor as f64 * read_f64_at(0x96a5c0); // 0x96a5c0 = 0.0001
    if !(f > read_f64_at(0x968f20)) { // 0x968f20 = 0.15
        f = read_f64_at(0x968f20); // 0x968f20 = 0.15
    }
    let t = match_pl_ftol(read_i32(clube + club::ATTENDANCE) as f64 * f);
    let mut g = rep as f64 * read_f64_at(0x96a680); // 0x96a680 = 0.00025
    if !(g > read_f64_at(0x968968)) { // 0x968968 = 0.5
        g = read_f64_at(0x968968); // 0x968968 = 0.5
    }
    let u = match_pl_ftol(t as f64 * g);
    let est = read_usize(clube + club::STADIUM);
    let teto = |e: usize| if e != 0 { read_i32(e + 0x3c) as f64 * read_f64_at(0x96a678) } else { read_f64_at(0x96a5e8) }; // 0x96a678 = 0.875, 0x96a5e8 = 1250
    let n = match_pl_ftol(if (u as f64) < teto(est) { u as f64 } else { teto(est) });
    if n == 0 {
        return;
    }
    let ja = read_i32(this + 0x18);
    if ja >= n {
        return;
    }
    let mut step_towards = (read_u8(this + 0x164) as i8).wrapping_add(1);
    if step_towards < 0 || step_towards > 8 {
        step_towards = 8;
    }
    *((this + 0x164) as *mut i8) = step_towards;
    let b = match_pl_ftol(n.wrapping_sub(ja) as f64 * read_f32_at(0x9c8758 + step_towards as usize * 4) as f64);
    crate::comparacao::ramo("cpv_5 ingressos vendidos");
    let r = friendly_random(0xb);
    let novos = (r + 0x5f).wrapping_mul(b) / 100;
    let total = ja.wrapping_add(novos);
    core::ptr::write_unaligned((this + 0x18) as *mut i32, total); // cpv5: os ingressos vendidos
    let preco = match_pl_ftol((rep as f64 * read_f64_at(0x969048) + read_f64_at(0x968938)) * read_f64_at(0x968ea8)); // 0x969048 = 0.002, 0x968938 = 4, 0x968ea8 = 20
    let mut receita = preco.wrapping_mul(novos);
    let est = read_usize(clube + club::STADIUM);
    let lugares = if est != 0 { read_i32(est + 0x3c) } else { 0 };
    let den = if lugares != 0 { lugares as f64 * read_f64_at(0x96a678) } else { read_f64_at(0x96a5e8) }; // 0x96a678 = 0.875, 0x96a5e8 = 1250
    let razao = total as f64 / den;
    core::ptr::write_unaligned((this + 0x160) as *mut f32, razao as f32);
    if razao > read_f64_at(0x96a678) { // 0x96a678 = 0.875
        core::ptr::write_unaligned((this + 0x160) as *mut u32, 0x3f600000);
    }
    let nac = read_usize(clube + club::NATION);
    let tipo = if nac != 0 { read_u8(nac + 0x7e) as i8 } else { 3 };
    match tipo {
        1 => {}
        2 => receita /= 4,
        _ => receita /= 10,
    }
    let liga = read_usize(clube + club::DIVISION);
    let cid = read_i32(clube);
    let fator = if liga != 0 && read_i32(liga) == read_i32(KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        0x968928
    } else if cid == read_i32(0x9d0250) {
        0x968940
    } else if cid == read_i32(0x9d0110) {
        0x968de8
    } else if cid == read_i32(0x9d0248) {
        0x969040
    } else if cid == read_i32(0x9d0244) {
        0x9689d8
    } else if cid == read_i32(0x9d024c) {
        0x968de8
    } else if nac != 0 && read_i32(nac) == read_i32(KEY_NATION_GERMANY) {
        0x968978
    } else {
        0
    };
    if fator != 0 {
        receita = match_pl_ftol(receita as f64 * read_f64_at(fator));
    }
    if aplicar != 0 { // cpv5: a receita dos ingressos
        add_to_i32(this + 0xac, receita);
        add_to_i32(this + 0x14c, receita);
        add_i32_to_balance(this, receita);
    }
    let r1 = friendly_random(rep);
    let r2 = friendly_random(rep); // cpv5: o segundo sorteio da loja
    let k = r1.wrapping_add(r2).wrapping_add(1);
    let v = match_pl_ftol((k as f64 * read_f64_at(0x96a350) + read_f64_at(0x967890)) * novos as f64); // 0x96a350 = 0.001, 0x967890 = 1
    add_to_i32(this + 0x15c, v);
    add_to_i32(this + 0xbc, v);
    add_i32_to_balance(this, v);
}

/// Comparer for `sell_season_tickets` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (this), nos lugares do estádio (+0x3c) e no estado do sorteio
/// (clube nulo: só a original).
pub(crate) unsafe extern "thiscall" fn comparar_sell_season_tickets(this: usize, aplicar: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_SELL_SEASON_TICKETS);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(club::SIZE));
    if clube == 0 {
        return o(this, aplicar);
    }
    let mut regioes = vec![(this, 0x167)];
    let est = read_usize(clube + club::STADIUM);
    if est != 0 {
        regioes.push((est + 0x3c, 4));
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, aplicar), || crate::aleatorio::sem_contar(|| sell_season_tickets(this, aplicar)));
    PLACAR.registrar(igual, || format!("sell_season_tickets(fin {:#x}, {}) original={:x?} rust={:x?}", this, aplicar, mo, mr));
}

/// Charges a works cost to the club's finance record (+0x8c, +0x12c and minus the balance).
///
/// [pt] O custo da obra (+0x8c e +0x12c, menos no saldo) no registro de finanças do clube.
pub(crate) unsafe fn charge_works_cost(this: usize, clube: usize, custo: i32) {
    let fin = fin_prm_club_finance(this, clube);
    add_to_i32(fin + 0x12c, custo);
    add_to_i32(fin + 0x8c, custo);
    add_i32_to_balance(fin, custo.wrapping_neg());
}

/// the training facilities level (+0x7f) rises by reputation (up to 20) and each step charges the works to +0x8c,
/// +0x12c and the balance
/// Original: `FinanceTable::upgrade_training_facilities` (0x59b630, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x59b630 } (thiscall, ret 4, finance.cpp): `FinanceTable::upgrade_training_facilities(clube)`, pelo fim da
/// temporada do clube (0x59cfa0, com o pedido de treino aprovado pela diretoria). O nível (+0x7f) sobe 2 (1 a partir de
/// 19) e cobra nível × 50000 + 75000. Depois, abaixo de 20: com reputação (+0x80) acima de 8250, vai a 20 e cobra
/// (22 − nível) × 250000 (mínimo 2 milhões); abaixo de 14 com reputação acima de 5250, sobe 4 (mínimo 16) e cobra 3,5
/// milhões − nível × 150000 (mínimo 1 milhão); abaixo de 16 com reputação acima de 4250, ou abaixo de 20 com reputação
/// acima de 7250: acima de 7250 vai a 20, senão sobe 2 (mínimo 12), e cobra o novo nível × 50000 + 75000. Teto 20.
pub(crate) unsafe extern "thiscall" fn upgrade_training_facilities(this: usize, clube: usize) {
    let nivel = (clube + club::TRAINING) as *mut i8;
    let custo = (*nivel as i32).wrapping_mul(50000).wrapping_add(75000);
    *nivel = if *nivel < 0x13 { (*nivel).wrapping_add(2) } else { (*nivel).wrapping_add(1) };
    charge_works_cost(this, clube, custo);
    let n = *nivel;
    let rep = read_i16(clube + club::REPUTATION);
    let segundo = if n < 0x14 && rep > 0x203a {
        *nivel = 0x14;
        Some((0x16 - n as i32).wrapping_mul(250000).max(2000000))
    } else if n < 0xe && rep > 0x1482 {
        let c = 3500000i32.wrapping_sub((n as i32).wrapping_mul(150000)).max(1000000);
        *nivel = n.wrapping_add(4);
        if *nivel < 0x10 {
            *nivel = 0x10;
        }
        Some(c)
    } else if (n < 0x10 && rep > 0x109a) || (n < 0x14 && rep > 0x1c52) {
        if rep > 0x1c52 {
            *nivel = 0x14;
        } else {
            *nivel = n.wrapping_add(2);
            if *nivel < 0xc {
                *nivel = 0xc;
            }
        }
        Some((*nivel as i32).wrapping_mul(50000).wrapping_add(75000))
    } else {
        None
    };
    if let Some(c) = segundo {
        charge_works_cost(this, clube, c);
    }
    if *nivel > 0x14 {
        *nivel = 0x14;
    }
}

/// Comparer for `upgrade_training_facilities` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no nível do centro de treino (+0x7f) e no registro de finanças do clube.
pub(crate) unsafe extern "thiscall" fn comparar_upgrade_training_facilities(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPGRADE_TRAINING_FACILITIES);
    let regioes = [(clube + club::TRAINING, 1), (fin_prm_club_finance(this, clube), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || upgrade_training_facilities(this, clube));
    PLACAR.registrar(igual, || format!("upgrade_training_facilities(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
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
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    f(fin) as i8
}

/// Calls a weekly finance event routine (thiscall(table, club, finance)); returns 1 if it happened.
///
/// [pt] Um evento da semana (0x59e050, 0x59dd40, 0x59e5a0, 0x59e940): thiscall(tabela, clube, finanças), 1 se aconteceu.
pub(crate) unsafe fn run_weekly_event(rotina: usize, this: usize, clube: usize, fin: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(rotina);
    f(this, clube, fin)
}

/// O ensaio do comparador: a cópia roda sem a notícia (0x59f300) e sem a verba (0x598800), que alocam e não podem
/// rodar duas vezes.
pub(crate) static mut RARE_FINANCE_EVENT_DRY_RUN: bool = false;

/// Posts the finance event news (FN_FINANCE_POST_FINANCE_BOOST_NEWS); nothing in rehearsal.
///
/// [pt] A notícia do evento (0x59f300: thiscall(tabela, clube, valor, verba, tipo), ret 0x10); no ensaio, nada.
pub(crate) unsafe fn post_finance_boost_news(this: usize, clube: usize, valor: i32, verba: u32, tipo: u32) {
    if RARE_FINANCE_EVENT_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(FN_FINANCE_POST_FINANCE_BOOST_NEWS);
    f(this, clube, valor, verba, tipo);
}

/// Transfer budget for the news (Finance::get_transfer_budget 0x598800 with 0, 0, 0, 1, 0); 0 in rehearsal.
///
/// [pt] A verba de transferências (Finance::get_transfer_budget 0x598800 com 0, 0, 0, 1, 0) que vai na notícia; no
/// ensaio, 0 (a notícia não sai).
pub(crate) unsafe fn transfer_budget(fin: usize) -> u32 {
    if RARE_FINANCE_EVENT_DRY_RUN {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    f(fin, 0, 0, 0, 1, 0)
}

/// Posts the end-of-crisis news (0x5a02f0, receivers appeased) for the club id.
///
/// [pt] O fim da semana do clube no estado de crise (0x5a02f0: thiscall(tabela, id)).
pub(crate) unsafe fn post_receivers_appeased(this: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FINANCE_POST_RECEIVERS_APPEASED_NEWS);
    f(this, read_usize(clube) as u32);
}

/// Resets the finance state (+0x165) to 0 from 1 or 2; from 1 (crisis) posts the end-of-crisis news first.
///
/// [pt] O estado (+0x165) volta a 0; se era 1 (crise), a notícia do fim da crise (0x5a02f0) antes.
pub(crate) unsafe fn leave_crisis(this: usize, clube: usize, fin: usize) {
    let s = read_u8(fin + 0x165);
    if s == 1 {
        post_receivers_appeased(this, clube);
    }
    if s == 1 || s == 2 {
        *((fin + 0x165) as *mut u8) = 0;
    }
}

/// A maxed training ground (club::TRAINING = 20) drops one level on random(10) = 0 if reputation is below 7250.
///
/// [pt] O centro de treino no máximo (20) cai um nível com sorteio(10) zero, se a reputação for menor que 7250.
pub(crate) unsafe fn decay_training_ground(clube: usize) {
    let nivel = (clube + club::TRAINING) as *mut i8;
    if *nivel == 0x14 && read_i16(clube + club::REPUTATION) < 0x1c52 && friendly_random(10) == 0 {
        *nivel = (*nivel).wrapping_sub(1);
    }
}

/// by the finance band: ends or opens the crisis (+0x165), moves the squeeze counter (+0x166), tries the boost, the
/// debt, the injection and the board takeover, wears the training ground; with squeeze above 50, removes the heating or
/// closes 10% of the places
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
pub(crate) unsafe extern "thiscall" fn process_club_finance_events(this: usize, clube: usize, semana: u32) {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_CLUB_FINANCE_EVENTS)(this, clube, semana);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a: u32 = 0;
    let p = parceiro(clube, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { clube };
    let fin = read_usize(this).wrapping_add((read_usize(c) as u32).wrapping_mul(0x167) as usize);
    let aperto = (fin + 0x166) as *mut i8;
    match wealth_band(fin) {
        -1 => {
            let s = read_u8(fin + 0x165);
            if s != 2 {
                if s == 1 {
                    post_receivers_appeased(this, clube);
                }
                *((fin + 0x165) as *mut u8) = 2;
            }
            if semana == 0 {
                return;
            }
            if read_u8(clube + club::OWN_STADIUM) != 0 {
                *aperto = (*aperto).wrapping_add(1);
            }
            decay_training_ground(clube);
            if run_weekly_event(0x59dd40, this, clube, fin) == 0 && run_weekly_event(0x59e5a0, this, clube, fin) == 0 {
                run_weekly_event(0x59e940, this, clube, fin);
            }
        }
        -2 => {
            if read_u8(fin + 0x165) != 1 {
                let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINANCE_POST_FINANCIAL_CRISIS_NEWS);
                let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
                let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
                let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
                let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
                let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
                noticia(this, clube);
                if read_u8(clube + club::PLC) != 0 {
                    *((clube + club::PLC) as *mut u8) = 0;
                }
                *((fin + 0x165) as *mut u8) = 1;
                if humano(clube, 1, 1, 0, 0) == 0 {
                    return;
                }
                let mut elenco = [0usize; 64];
                reunir(clube, elenco.as_mut_ptr(), 0);
                for &j in elenco.iter().take(50) {
                    if j == 0 || read_usize(j + 0x61) == 0 {
                        continue;
                    }
                    let g = read_usize(G_CONTRACT_MANAGER);
                    let k = contract_of(g, j);
                    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = k;
                    let dono = if k != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(club::SIZE)) } else { 0 };
                    let ct = contrato(read_usize(G_CONTRACT_MANAGER), j, dono);
                    if dono == clube && ct != 0 {
                        let v = valor(read_usize(G_CONTRACT_MANAGER), j, dono, 0, -1, 1);
                        if v < read_i32(j + 0x52) {
                            *((ct + 0x4e) as *mut u8) |= 4;
                            core::ptr::write_unaligned((j + 0x52) as *mut i32, v);
                        }
                    }
                }
                return;
            }
            if semana == 0 {
                return;
            }
            if run_weekly_event(0x59e940, this, clube, fin) == 0 && run_weekly_event(0x59e5a0, this, clube, fin) == 0 {
                run_weekly_event(0x59e050, this, clube, fin);
            }
            decay_training_ground(clube);
            if read_u8(clube + club::OWN_STADIUM) != 0 {
                *aperto = (*aperto).wrapping_add(3);
            }
        }
        1 => {
            leave_crisis(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 0 {
                *aperto -= 1;
            }
            if run_weekly_event(0x59e050, this, clube, fin) == 0 && run_weekly_event(0x59dd40, this, clube, fin) == 0 {
                run_weekly_event(0x59e5a0, this, clube, fin);
            }
        }
        2 => {
            leave_crisis(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 5 {
                *aperto -= 1;
            }
        }
        _ => {
            leave_crisis(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 10 {
                *aperto -= 1;
            }
            crate::comparacao::ramo("cpv_6 impulso na band 0");
            let ok = run_weekly_event(0x59e050, this, clube, fin) != 0; // cpv6: o impulso na faixa 0
            if !ok && run_weekly_event(0x59dd40, this, clube, fin) == 0 {
                run_weekly_event(0x59e5a0, this, clube, fin);
            }
        }
    }
    // a crise prolongada
    let liga = read_usize(clube + club::DIVISION);
    if liga == 0 || read_i32(read_usize(G_COMP_OBJECT_TABLE) + read_usize(liga).wrapping_mul(4)) == 0 || *aperto <= 0x32 {
        return;
    }
    let est = read_usize(clube + club::STADIUM);
    if est == 0 {
        return;
    }
    let nivel = (clube + club::TRAINING) as *mut i8;
    if read_u8(est + 0x4d) != 0 && read_i16(clube + club::REPUTATION) < 0x109a {
        let aquecimento: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_FINANCE_POST_STADIUM_WORKS_NEWS);
        aquecimento(this, clube, 0, 7);
        *((read_usize(clube + club::STADIUM) + 0x4d) as *mut u8) = 0;
        let n = *nivel;
        if n == 0x14 {
            let r = friendly_random(4) as u8;
            *nivel = (*nivel).wrapping_add(0xffu8.wrapping_sub(r) as i8);
            return;
        }
        if n <= 1 || friendly_random(n as i32) == 0 {
            return;
        }
        *nivel = (*nivel).wrapping_sub(1);
        return;
    }
    let lugares = read_i32(est + 0x3c);
    if lugares <= 0xabe || friendly_random(read_i16(clube + club::REPUTATION) as i32) >= lugares {
        return;
    }
    let t = match_pl_ftol(lugares as f64 * read_f64_at(0x967880)) as i16 as i32; // 0x967880 = 0.1
    let dez = (t / 10).wrapping_mul(10);
    let fechar = dez as i16 as i32;
    let resto = read_i32(est + 0x40);
    let tirou = if resto > fechar {
        core::ptr::write_unaligned((est + 0x40) as *mut i32, resto - fechar);
        1
    } else {
        0
    };
    let fechados: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(FN_FINANCE_POST_STAND_CLOSED_NEWS);
    fechados(this, clube, dez, tirou);
    let est = read_usize(clube + club::STADIUM);
    add_to_i32(est + 0x3c, fechar.wrapping_neg());
    let est = read_usize(clube + club::STADIUM);
    add_to_i32(est + 0x44, fechar);
    *aperto = 0x14;
    let n = *nivel;
    if n > 0xf {
        let r = friendly_random(5) as u8;
        *nivel = (*nivel).wrapping_add(0xffu8.wrapping_sub(r) as i8);
        return;
    }
    if n <= 7 && (n <= 1 || friendly_random(n as i32) == 0) {
        return;
    }
    *nivel = (*nivel).wrapping_sub(1);
}

// ponytail: chama originais que sorteiam, alocam notícias e mexem nos contratos: só a original; a cópia leva o ramo.
/// Comparer for `process_club_finance_events` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_club_finance_events(this: usize, clube: usize, semana: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_CLUB_FINANCE_EVENTS)(this, clube, semana)
}

/// poor club (band −2/−1, or 0 with a low balance): three random draws; the board pays 450, 300 or 150 × reputation and
/// sends the news
/// Original: `FinanceTable::try_board_debt_payment` (0x59dd40, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x59dd40 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_board_debt_payment(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Faixa (0x5986b0) −2, −1 (divisor 2, ou 0,4 com reputação de 3500), ou 0 com reputação até 3500 e
/// saldo abaixo de 25 × reputação (0,4); as outras: 0. Sorteio(7000) + 3500 acima da reputação; sorteio(6 / divisor)
/// zero; sorteio(nível +0x20 do estádio do clube de +0xbf, ou 10) não zero. A diretoria paga 450 × reputação (abaixo de
/// 1250 com saldo abaixo de −450 × reputação), 300 × (saldo abaixo de −300 × reputação) ou 150 ×: em +0x154, +0xb4 e no
/// saldo; a notícia (0x59f300, tipo 4); devolve 1.
pub(crate) unsafe extern "thiscall" fn try_board_debt_payment(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = read_i16(clube + club::REPUTATION) as i32;
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
    crate::comparacao::ramo("cpv_6 sorteio da divida");
    let r = friendly_random(7000); // cpv6: o sorteio da dívida
    if rep >= r.wrapping_add(0xdac) {
        return 0;
    }
    let n = match_pl_ftol(read_f64_at(0x969000) / div); // 0x969000 = 6
    let outro = read_usize(clube + club::CHAIRMAN);
    if outro != 0 && read_usize(outro + 0x69) != 0 {
        if friendly_random(n) != 0 {
            return 0;
        }
        let est = read_usize(read_usize(clube + club::CHAIRMAN) + 0x69);
        if friendly_random(read_u8(est + 0x20) as i8 as i32) == 0 {
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
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let saldo = balance(fin);
    let valor = if rep < 0x4e2 && saldo < rep.wrapping_mul(-450) as i64 {
        rep.wrapping_mul(450)
    } else if saldo >= rep.wrapping_mul(-300) as i64 {
        rep.wrapping_mul(150)
    } else {
        rep.wrapping_mul(300)
    };
    add_to_i32(fin + 0x154, valor);
    add_to_i32(fin + 0xb4, valor);
    credit_balance(fin, valor);
    post_finance_boost_news(this, clube, valor, 0, 4);
    1
}

/// Rehearsal comparer for weekly finance events: copy without news/budget vs original from the same state, comparing
/// return and balance, +0xb4, +0x154, +0x165.
///
/// [pt] O comparador dos eventos: a cópia em ensaio (sem notícia nem verba) e a original a partir do mesmo estado (registro de
/// finanças e sorteio). Sem evento: tudo igual, inclusive o sorteio; com evento: a devolução e os campos que o evento
/// grava (saldo, +0xb4, +0x154, +0x165), porque a notícia e a verba da original podem sortear e gravar.
pub(crate) unsafe fn compare_finance_event(nome: &str, fin: usize, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    RARE_FINANCE_EVENT_DRY_RUN = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(copia));
    RARE_FINANCE_EVENT_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if r == 0 && n == 0 {
        mo == mr
    } else {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165]
    };
    PLACAR.registrar(igual, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", nome, fin, r, mo, n, mr));
    r
}

/// Comparer for `try_board_debt_payment` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_board_debt_payment(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_DEBT_PAYMENT);
    compare_finance_event("try_board_debt_payment", fin, || o(this, clube, fin), || try_board_debt_payment(this, clube, fin))
}

/// club of reputation 4750 outside Wales and the Irelands: draws by the band and the stadium of the +0xbf club; the
/// amount by reputation, up to 10% of the debt and 4 million; news and end of crisis
/// Original: `FinanceTable::try_cash_injection` (0x59e5a0, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x59e5a0 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_cash_injection(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Reputação (+0x80) de 4750, sem a marca +0x82, fora do País de Gales e das Irlandas. Divisor pela
/// faixa (−2: 0,25; −1: 0,1; outras: 0,001). Com o estádio do clube de +0xbf: sorteio(30 / divisor) abaixo do nível
/// (+0xf) e o nível +0x20 acima de sorteio(5) + 10: (20 × nível + 150) × reputação + 750000; sem ele: sorteio(4 /
/// divisor) zero: (5 × reputação + 15000) × 50. Dívida acima de 10 × o valor: 10% da dívida. Teto: reputação² + 1,5
/// milhão abaixo de 2000; senão 4 milhões (+1 se o valor passar de reputação² / 4). Em +0x154, +0xb4 e no saldo; a
/// notícia (0x59f300, tipo 3, com a verba 0x598800); encerra a crise (+0x165 = 0); devolve 1.
pub(crate) unsafe extern "thiscall" fn try_cash_injection(this: usize, clube: usize, fin: usize) -> u32 {
    if read_i16(clube + club::REPUTATION) < 0x128e { // cpv6: o filtro da injeção
        return 0;
    }
    if read_u8(clube + club::PLC) != 0 {
        return 0;
    }
    if clube != 0 {
        let nac = read_usize(clube + club::NATION);
        if nac != 0 {
            let n = read_i32(nac);
            if n == read_i32(KEY_NATION_WALES) || n == read_i32(KEY_NATION_REPUBLIC_OF_IRELAND) || n == read_i32(KEY_NATION_NORTHERN_IRELAND) {
                return 0;
            }
        }
    }
    crate::comparacao::ramo("cpv_6 injecao avaliada");
    let div = match wealth_band(fin) {
        -2 => read_f64_at(0x9689e0), // 0x9689e0 = 0.25
        -1 => read_f64_at(0x967880), // 0x967880 = 0.1
        _ => read_f64_at(0x96a350), // 0x96a350 = 0.001
    };
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let outro = read_usize(clube + club::CHAIRMAN);
    let mut valor;
    if outro != 0 && read_usize(outro + 0x69) != 0 && read_u8(clube + club::PLC) == 0 {
        let r = friendly_random(match_pl_ftol(read_f64_at(0x968dd8) / div)); // 0x968dd8 = 30
        let est = read_usize(read_usize(clube + club::CHAIRMAN) + 0x69);
        if (read_u8(est + 0xf) as i8 as i32) <= r {
            return 0;
        }
        let e = read_u8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as i8 as i32;
        if e <= friendly_random(5).wrapping_add(10) {
            return 0;
        }
        valor = e.wrapping_mul(20).wrapping_add(0x96).wrapping_mul(rep).wrapping_add(0xb71b0);
    } else {
        if friendly_random(match_pl_ftol(read_f64_at(0x968938) / div)) != 0 { // 0x968938 = 4
            return 0;
        }
        if read_u8(clube + club::PLC) != 0 {
            return 0;
        }
        valor = rep.wrapping_mul(5).wrapping_add(0x3a98).wrapping_mul(50);
    }
    let saldo = balance(fin);
    if (valor.wrapping_mul(10) as i64) < saldo.wrapping_neg() {
        valor = match_pl_ftol(-(saldo as f64 * read_f64_at(0x967880))); // 0x967880 = 0.1
    }
    if read_i16(clube + club::REPUTATION) < 0x7d0 {
        let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
        if valor > teto {
            valor = teto;
        }
    } else {
        let maior = (valor as f64 > rep.wrapping_mul(rep) as f64 * read_f64_at(0x9689e0)) as i32; // 0x9689e0 = 0.25
        if valor > 0x3d0900 + maior {
            valor = 0x3d0900 + maior;
        }
    }
    add_to_i32(fin + 0x154, valor);
    add_to_i32(fin + 0xb4, valor);
    credit_balance(fin, valor);
    let verba = transfer_budget(fin);
    post_finance_boost_news(this, clube, valor, verba, 3);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

/// Comparer for `try_cash_injection` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_cash_injection(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_CASH_INJECTION);
    compare_finance_event("try_cash_injection", fin, || o(this, clube, fin), || try_cash_injection(this, clube, fin))
}





/// the highest wage offered to staff by job: default by job, nation level (+0x85) / type, key nation adjustments,
/// league weight × 0.2, finance band, club ceiling (+0x14) and job range
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
pub(crate) unsafe extern "thiscall" fn get_max_staff_wage(this: usize, cargo: i8) -> i32 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let id = read_usize(this + 8);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE));
    if read_u8(clube + club::PROFESSIONAL_STATUS) == 3 {
        return 0;
    }
    let r50 = read_i16(clube + club::REPUTATION) as i32 / 50;
    let idx = (cargo as i32).wrapping_sub(5);
    let padrao: i32 = match idx {
        0..=2 => 35000,
        3 => 20000,
        4 => 1500,
        5 => 1000,
        _ => 750,
    };
    if selecao(clube) != 0 {
        return padrao;
    }
    let nac = read_usize(clube + club::NATION);
    let mut w;
    if nac == 0 {
        w = match_pl_ftol(padrao as f64 * read_f64_at(0x9689e0)); // 0x9689e0 = 0.25
    } else {
        let tipo = (read_u8(nac + 0x7e) as i8).clamp(1, 3);
        let k = read_u8(nac + 0x85) as i8 as i32;
        w = match_pl_ftol(k as f64 / (tipo as i32 * 20) as f64 * padrao as f64);
        if tipo == 1 {
            let n = read_i32(nac);
            let e = |a: usize| n == read_i32(a);
            if e(KEY_NATION_ENGLAND) || e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN) {
                if read_i16(clube + club::REPUTATION) > 0x1c52 {
                    crate::comparacao::ramo("cpv_7 clube grande");
                    w = match_pl_ftol(w as f64 * read_f64_at(0x9689b8)); // cpv7: o clube grande da Inglaterra, Itália e Espanha; 0x9689b8 = 1.05
                }
            } else if e(KEY_NATION_JAPAN) {
            } else if e(KEY_NATION_UNITED_STATES) {
                if w > 0x6d6 {
                    w = match_pl_ftol(w.wrapping_sub(0x6d6) as f64 * read_f64_at(0x969040) + read_f64_at(0x96a5f8)); // 0x969040 = 0.75, 0x96a5f8 = 1750
                }
            } else {
                let (base, limiar, soma) = if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SCOTLAND) || e(KEY_NATION_HOLLAND) {
                    (0x969040, 0x9c4, 0x9690b0)
                } else if e(KEY_NATION_SWEDEN) || e(KEY_NATION_DENMARK) || e(KEY_NATION_BELGIUM) || e(KEY_NATION_PORTUGAL) {
                    (0x969040, 0x4e2, 0x96a5e8)
                } else {
                    (0x968e80, 0x2ee, 0x96a5f0)
                };
                let mut f = read_f64_at(base);
                if r50 > k.wrapping_mul(10) {
                    let d = if k >= 1 { k } else { 1 };
                    f = r50 as f64 / d as f64 * read_f64_at(0x967880) * read_f64_at(base); // 0x967880 = 0.1
                }
                if w > limiar {
                    if f > read_f64_at(0x967890) { // 0x967890 = 1
                        f = read_f64_at(0x967890); // 0x967890 = 1
                    }
                    w = match_pl_ftol(w.wrapping_sub(limiar) as f64 * f + read_f64_at(soma));
                }
            }
        }
        let liga = read_usize(clube + club::DIVISION);
        if liga != 0 {
            crate::comparacao::ramo("cpv_7 peso da liga");
            w = match_pl_ftol(read_i16(liga + 0x69) as f64 * w as f64 * read_f64_at(0x968918)); // cpv7: o peso da liga; 0x968918 = 0.2
        }
    }
    match band(this) as i8 {
        -2 => w = match_pl_ftol(w as f64 * read_f64_at(0x969040)), // 0x969040 = 0.75
        -1 => w = match_pl_ftol(w as f64 * read_f64_at(0x9689d0)), // 0x9689d0 = 0.95
        1 => w = match_pl_ftol(w as f64 * read_f64_at(0x968940)), // 0x968940 = 1.5
        2 => w = w.wrapping_add(w),
        _ => {}
    }
    let teto = read_i32(this + 0x14);
    if teto < w {
        w = teto;
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

/// Comparer for `get_max_staff_wage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_max_staff_wage(this: usize, cargo: i8) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_GET_MAX_STAFF_WAGE)(this, cargo);
    let n = get_max_staff_wage(this, cargo);
    PLACAR.registrar(r == n, || format!("get_max_staff_wage(fin {:#x}, cargo {}) original={} rust={}", this, cargo, r, n));
    r
}

/// adds the amount to the bonus expenses (+0x138 and +0x98) and takes it from the balance
/// Original: `Finance::pay_bonuses` (0x599960, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599960 } (thiscall, ret 4, finance.cpp): `Finance::pay_bonuses(valor)`, pelo fim do jogo dos contratos
/// (CONTRACT_MANAGER::update_after_match 0x4ca6d0, três chamadas): soma o valor nas despesas de bônus (+0x138 e +0x98)
/// e o tira do saldo (i64 em +0, estendido com sinal).
pub(crate) unsafe extern "thiscall" fn pay_bonuses(this: usize, valor: i32) {
    for o in [0x138usize, 0x98] {
        core::ptr::write_unaligned((this + o) as *mut i32, read_i32(this + o).wrapping_add(valor));
    }
    crate::comparacao::ramo("cpv_7 bonus pagos");
    let s = core::ptr::read_unaligned(this as *const i64);
    core::ptr::write_unaligned(this as *mut i64, s.wrapping_sub(valor as i64)); // cpv7: o bônus sai do saldo
}

/// Comparer for `pay_bonuses` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pay_bonuses(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_BONUSES);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pay_bonuses(this, valor));
    PLACAR.registrar(igual, || format!("pay_bonuses(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// wage ceiling (+0x14) × (percent + 100)%, truncated
/// Original: `Finance::adjust_max_wage` (0x598340, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x598340 } (thiscall, ret 4, finance.cpp): `Finance::adjust_max_wage(percentual)`, pela resposta da diretoria
/// (MANAGER_MANAGER::handle_board_request 0x6a91b0) e pela 0x6acee8: o teto salarial (+0x14) vira (percentual + 100) ×
/// 0,01 × teto, truncado.
pub(crate) unsafe extern "thiscall" fn adjust_max_wage(this: usize, write_percent_pair: i8) {
    let v = match_pl_ftol((write_percent_pair as i32 + 100) as f64 * read_f64_at(0x967898) * read_i32(this + 0x14) as f64); // 0x967898 = 0.01
    core::ptr::write_unaligned((this + 0x14) as *mut i32, v);
}

/// Comparador: lado a lado no teto salarial (+0x14).
pub(crate) unsafe extern "thiscall" fn comparar_adjust_max_wage(this: usize, write_percent_pair: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_ADJUST_MAX_WAGE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x14, 4)], || o(this, write_percent_pair), || adjust_max_wage(this, write_percent_pair));
    PLACAR.registrar(igual, || format!("adjust_max_wage(fin {:#x}, {}) original={:x?} rust={:x?}", this, write_percent_pair, mo, mr));
}

/// national team, 0; by the 64-bit balance against the club reputation R: above 2500·R + 5,000,000, 2; above 1000·R +
/// 3,500,000, 1; below −2,500,000 − 1500·R, −2; below −250·R (R < 5000) or (3750 − R)·1000, −1; otherwise 0
/// Original: `Finance::get_wealth_band` (0x5986b0, finance.cpp, thiscall).
///
/// [pt] { 0x5986b0 } (thiscall, finance.cpp): `Finance::get_wealth_band()`, a faixa do saldo (64 bits em +0/+4) do clube (+8,
/// vetor 0xae23b4) pela reputação R (+0x80), usada pelas transferências, pela diretoria, pelos técnicos, pelas finanças e
/// pelo adiamento. Seleção (0x53b900): 0. Acima de 2500·R + 5.000.000: 2; acima de 1000·R + 3.500.000: 1; abaixo de
/// −2.500.000 − 1500·R: −2; abaixo de −250·R (R < 5000) ou de (3750 − R)·1000 (R ≥ 5000): −1; senão 0.
pub(crate) unsafe extern "thiscall" fn get_wealth_band(this: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let id = core::ptr::read_unaligned((this + 8) as *const u32);
    let clube = core::ptr::read_unaligned(G_CLUB_TABLE as *const usize).wrapping_add(id.wrapping_mul(0x245) as usize);
    if selecao(clube) != 0 {
        return 0;
    }
    let rep = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
    let r = rep as i32;
    let saldo = core::ptr::read_unaligned(this as *const i64);
    if saldo > r.wrapping_mul(2500).wrapping_add(5_000_000) as i64 {
        crate::comparacao::ramo("cpu_7 rica");
        return 2; // cpu7: a faixa rica
    }
    if saldo > r.wrapping_mul(1000).wrapping_add(3_500_000) as i64 {
        crate::comparacao::ramo("cpu_7 confortavel");
        return 1; // cpu7: a faixa confortável
    }
    if saldo < (-2_500_000i32).wrapping_sub(r.wrapping_mul(1500)) as i64 {
        return 0xfe;
    }
    let devendo = if rep < 0x1388 { r.wrapping_mul(-250) } else { (0xea6 - r).wrapping_mul(1000) };
    if saldo < devendo as i64 {
        crate::comparacao::ramo("cpu_7 devendo");
        return 0xff; // cpu7: a faixa devendo
    }
    0
}

/// Comparer for `get_wealth_band` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_wealth_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_WEALTH_BAND)(this);
    let n = get_wealth_band(this);
    PLACAR.registrar(r == n, || format!("get_wealth_band(financas {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}





