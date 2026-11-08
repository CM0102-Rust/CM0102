use super::*;

// ---------- 266a leva: o fim de temporada das finanças: o fim de temporada da nação, as obras, o centro de treino e o dividendo de cada clube e a notícia do dividendo (fin_prm.cpp) ----------







/// Soma `v` nos dois acumuladores do ano e do mês (`ano`, `mes`) e o tira do saldo (i64; o cdq/sub/sbb da original).
pub(crate) unsafe fn cqc1_cobrar(fin: usize, mes: usize, ano: usize, v: i32) {
    cpv5_somar(fin + mes, v);
    cpv5_somar(fin + ano, v);
    core::ptr::write_unaligned(fin as *mut i64, cpv6_saldo(fin).wrapping_sub(v as i64));
}


/// O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
pub(crate) unsafe fn cqc1_humano(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, 0, 1, 0, 0)
}

/// O registro de finanças do clube (0x167 bytes) pelo parceiro: 0x540a50 (3 pushes; add esp, 0xc -> cdecl(clube, &a,
/// 1)); o parceiro quando ele existe e o clube é o B (a = 0). `a` é a local da original (a 0x540a50 a grava).
pub(crate) unsafe fn cqc1_financas_do_parceiro(this: usize, clube: usize, a: &mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let p = parceiro(clube, a, 1);
    let c = if p != 0 && *a == 0 { p } else { clube };
    read_usize(this).wrapping_add((read_i32(c) as u32).wrapping_mul(0x167) as usize)
}

/// A notícia das obras e do treino (0x59ffb0: ecx = tabela; 3 pushes (tipo, valor, clube); sem add esp -> thiscall,
/// ret 0xc).
pub(crate) unsafe fn cqc1_noticia(this: usize, clube: usize, valor: i32, tipo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(FN_FINANCE_POST_STADIUM_WORKS_NEWS);
    f(this, clube, valor, tipo);
}

/// A notícia da obra recusada (0x59fae0: ecx = tabela; 3 pushes; sem add esp -> thiscall, ret 0xc; sorteia 2 fora do
/// motivo 2).
pub(crate) unsafe fn cqc1_recusa(this: usize, clube: usize, assunto: i32, motivo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(FN_FINANCE_POST_WORKS_CANCELLED_NEWS);
    f(this, clube, assunto, motivo);
}

/// A linha do clube na tabela dos técnicos ([[0xb67a44]] + id × 0x49); o byte +2 tem os pedidos à diretoria (bit 6:
/// estádio; bit 7: centro de treino).
pub(crate) unsafe fn cqc1_linha_dos_pedidos(clube: usize) -> usize {
    read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x49) as usize)
}

/// O técnico do clube (+0xcf) é humano (entre as 16 últimas pessoas): a notícia vai para ele (0x7861e0).
pub(crate) unsafe fn cqc1_tecnico_humano(clube: usize) -> bool {
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
pub(crate) unsafe extern "thiscall" fn fim_de_temporada_da_nacao(this: usize, entrada: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if depois(0xae2c90, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return;
    }
    if entrada == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIM_DE_TEMPORADA_DA_NACAO)(this, entrada); // a caixa
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

/// O comparador do fim de temporada da nação: só a original, porque ela chama o fim de temporada de cada clube
/// (0x59cfa0), que sorteia e entrega notícias ao técnico humano (a cópia segue no modo Rust, com o contador). Confere o
/// que a rotina garante e nenhuma chamada desfaz: o último clube da entrada (a ordem dos clubes) fica com os totais do
/// ano (+0x110, 0x50 bytes) zerados. Ponytail: os anteriores podem ganhar valores pela 0x59cfa0 de um clube seguinte
/// (o B que usa o registro do A, o estádio dividido); conferir todos pede o plano dessas exceções.
pub(crate) unsafe extern "thiscall" fn comparar_fim_de_temporada_da_nacao(this: usize, entrada: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIM_DE_TEMPORADA_DA_NACAO);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if entrada == 0 || depois(0xae2c90, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
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
        PLACAR.registrar(zerado, || format!("fim_de_temporada_da_nacao(entrada {:#x}) clube {} sem zerar os totais do ano", entrada, k));
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
pub(crate) unsafe extern "thiscall" fn fim_de_temporada_do_clube(this: usize, clube: usize) {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIM_DE_TEMPORADA_DO_CLUBE)(this, clube); // a caixa
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        return;
    }
    let mut local = 0u32; // o [esp + 0x20] da original (a 0x540a50 do dono e a média de público)
    let mut a = 0u32;
    let fin = cqc1_financas_do_parceiro(this, clube, &mut a);
    if fin == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIM_DE_TEMPORADA_DO_CLUBE)(this, clube); // a caixa
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
            let f2 = if dono != 0 { cqc1_financas_do_parceiro(this, dono, &mut local) } else { 0 };
            if dono == 0 || f2 == 0 {
                *((clube + club::OWN_STADIUM) as *mut u8) = 1;
                cqc1_cobrar(fin, 0x8c, 0x12c, 20_000_000);
                cqc1_noticia(this, clube, read_i32(read_usize(clube + club::STADIUM)), 4);
            } else if read_u8(f2 + 0x165) == 1 || read_u8(f2 + 0x165) == 2 {
                *((clube + club::OWN_STADIUM) as *mut u8) = 1;
                *((dono + 0x6d) as *mut u8) = 0;
                cqc1_cobrar(fin, 0x8c, 0x12c, 20_000_000);
                cpv5_somar(f2 + 0xb4, 20_000_000);
                cpv5_somar(f2 + 0x154, 20_000_000);
                core::ptr::write_unaligned(f2 as *mut i64, cpv6_saldo(f2).wrapping_add(20_000_000));
                cqc1_noticia(this, clube, read_i32(read_usize(clube + club::STADIUM)), 4);
                cqc1_noticia(this, dono, read_i32(read_usize(clube + club::STADIUM)), 5);
            }
            break 'obras;
        }
        if read_u8(clube + club::OWN_STADIUM) == 0 || read_usize(clube + club::DIVISION) == 0 {
            break 'obras;
        }
        let aperto = read_u8(fin + 0x166) as i8;
        if saldo > 0xb71b0 && aperto > 10 {
            let v = 10i32.wrapping_sub((aperto as i32).wrapping_mul(10000));
            cpv5_somar(fin + 0x12c, v);
            saldo = saldo.wrapping_sub(v);
            cpv5_somar(fin + 0x8c, v);
            core::ptr::write_unaligned(fin as *mut i64, cpv6_saldo(fin).wrapping_sub(v as i64));
        }
        cqc1_centro_de_treino(this, clube, saldo);
        let pedidos = cqc1_linha_dos_pedidos(clube) + 2;
        if read_u8(pedidos) & 0x40 != 0 && cqc1_humano(clube) != 0 {
            aviso = true;
            *(pedidos as *mut u8) &= 0xbf;
        }
        let est = read_usize(clube + club::STADIUM);
        if est != 0 {
            if saldo > 0x10b0760 && read_i16(clube + club::REPUTATION) > 0x1d4c && read_i32(est + 0x3c) > 0x61a8 && read_u8(est + 0x4d) == 0 {
                *((est + 0x4d) as *mut u8) = 1;
                cqc1_cobrar(fin, 0x8c, 0x12c, 1_500_000);
                cqc1_noticia(this, clube, read_i32(read_usize(clube + club::STADIUM)), 6);
                break 'obras;
            }
            if saldo > 0x47868c0 && read_i16(clube + club::REPUTATION) > 0x2422 && read_i32(est + 0x3c) > 0xe09c && read_u8(clube + club::TRAINING) == 0x14 && read_u8(est + 0x4c) == 0 {
                *((est + 0x4c) as *mut u8) = 1;
                cqc1_cobrar(fin, 0x8c, 0x12c, 7_500_000);
                break 'obras;
            }
        }
        let pode = saldo > 0x501bd0 || (cqc1_humano(clube) != 0 && saldo.wrapping_add((read_i16(clube + club::REPUTATION) as i32).wrapping_mul(1000)) > 0x501bd0);
        if !pode {
            if aviso {
                cqc1_recusa(this, clube, 2, -1);
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
            || (cap as f64 * f64_em(0x96a720)) < (local as i16 as i32) as f64
            || (rep.wrapping_mul(2) as i32 > cap && rep > 0x109a);
        if !expandir {
            if cpe1_sorteio(0x14).wrapping_add(5) >= read_i16(read_usize(clube + club::DIVISION) + 0x69) as i32 {
                break 'obras;
            }
            let est = read_usize(clube + club::STADIUM);
            let livre = read_i32(est + 0x3c).wrapping_sub(read_i32(est + 0x40));
            if livre <= 0 {
                break 'obras;
            }
            let t = ftol(saldo as f64 * f64_em(0x968918));
            let mut v = ((t / 75) as i16 as i32 / 500) as i16 as i32 * 500;
            if v >= livre {
                v = livre;
            }
            if (v as i16) < 0x1f4 {
                if read_i32(est + 0x3c) >= 0x7530 {
                    break 'obras;
                }
                cpv5_somar(est + 0x44, 0x5dc);
                cqc1_recusa(this, clube, 2, 2);
                break 'obras;
            }
            let v = v as i16 as i32;
            cpv5_somar(est + 0x3c, v);
            cpv5_somar(read_usize(clube + club::STADIUM) + 0x40, v);
            crate::comparacao::ramo("cqc_1 lugares sentados");
            cqc1_cobrar(fin, 0x8c, 0x12c, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(75))); // cqc1: os lugares sentados
            cqc1_noticia(this, clube, v, 3);
            break 'obras;
        }
        let tipo1 = read_u8(clube + club::PROFESSIONAL_STATUS) == 1;
        let base = if saldo > 0x1312d00 {
            if tipo1 { 16000 } else { 21000 }
        } else if tipo1 {
            ftol(saldo as f64 * f64_em(0x967880)) / 125
        } else {
            ftol(saldo as f64 * f64_em(0x968918)) / 50
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
                cpv5_somar(est + 0x44, 0x5dc);
            }
            if aviso {
                cqc1_recusa(this, clube, 2, 2);
            }
            break 'obras;
        }
        let n = n as i16 as i32;
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, n.wrapping_add(cap));
        if tipo1 {
            cpv5_somar(read_usize(clube + club::STADIUM) + 0x40, n);
            cqc1_cobrar(fin, 0x8c, 0x12c, (n / 1000 + 1).wrapping_mul(6000).wrapping_add(n).wrapping_mul(125));
            cqc1_noticia(this, clube, n, 1);
        } else {
            cqc1_cobrar(fin, 0x8c, 0x12c, n.wrapping_add((n / 1000 + 1).wrapping_mul(1000)).wrapping_mul(50));
            cqc1_noticia(this, clube, n, 2);
        }
    }
    let pedidos = cqc1_linha_dos_pedidos(clube) + 2;
    if read_u8(pedidos) & 0x80 != 0 {
        if saldo > (500 - read_i16(clube + club::REPUTATION) as i32).wrapping_mul(500) {
            let melhorar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINANCETABLE_UPGRADE_TRAINING_FACILITIES);
            melhorar(this, clube);
            cqc1_noticia(this, clube, -1, 8);
        } else {
            cqc1_recusa(this, clube, 8, -1);
        }
        *((cqc1_linha_dos_pedidos(clube) + 2) as *mut u8) &= 0x7f;
    }
    cqc1_dividendo(this, clube, fin);
}

/// O centro de treino do clube da máquina no fim da temporada (o trecho 0x59d32a..0x59d54d da 0x59cfa0), cobrado nas
/// finanças do próprio clube ([tabela] + id × 0x167, sem o parceiro).
pub(crate) unsafe fn cqc1_centro_de_treino(this: usize, clube: usize, saldo: i32) {
    let tecnico = read_usize(clube + club::MANAGER);
    if tecnico == 0 || read_i16(clube + club::REPUTATION) == 0 || read_u8(clube + club::TRAINING) as i8 >= 0x14 || read_usize(tecnico + 0x69) == 0 {
        return;
    }
    if cqc1_humano(clube) != 0 {
        return;
    }
    let q = ((read_i16(clube + club::REPUTATION) as i32) / 500) as i8 as i32 + 8;
    if (read_u8(clube + club::TRAINING) as i8 as i32) <= q || saldo <= 0x225510 {
        return;
    }
    if cpe1_sorteio(0x28) >= read_u8(read_usize(read_usize(clube + club::MANAGER) + 0x69) + 0x22) as i8 as i32 {
        return;
    }
    let nivel = (clube + club::TRAINING) as *mut i8;
    let proprio = || read_usize(this).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x167) as usize);
    let cl = *nivel;
    let custo = (cl as i32) * 50000 + 75000;
    if cl < 0x14 {
        *nivel = cl + 1;
    }
    cqc1_cobrar(proprio(), 0x8c, 0x12c, custo);
    let cl = *nivel;
    let rep = read_i16(clube + club::REPUTATION);
    if cl < 0x14 && (rep > 0x203a || cpv6_saldo(proprio()) > 0xe4e1c0) {
        let custo = (4_500_000 - 200_000 * cl as i32).max(1_000_000);
        *nivel = 0x14;
        cqc1_cobrar(proprio(), 0x8c, 0x12c, custo);
        return;
    }
    if (cl < 0x10 && rep > 0x1482) || (cl < 0x14 && rep > 0x1c52) {
        let custo = (cl as i32) * 50000 + 75000;
        *nivel = cl + 1;
        cqc1_cobrar(proprio(), 0x8c, 0x12c, custo);
    }
}

/// O dividendo do fim da temporada (o trecho 0x59db6f..0x59dd25 da 0x59cfa0), pelo saldo i64 das finanças.
pub(crate) unsafe fn cqc1_dividendo(this: usize, clube: usize, fin: usize) {
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let mut lim = rep.wrapping_mul(8000);
    if lim < 0x7a120 {
        lim = 0x7a120;
    }
    let s = cpv6_saldo(fin);
    let v;
    if s > lim as i64 && read_u8(clube + club::PLC) != 0 {
        let r = cpe1_sorteio(0x1f5);
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
            let r = cpe1_sorteio(0x1f5);
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
    cpv5_somar(fin + 0x94, v);
    cpv5_somar(fin + 0x134, v);
    core::ptr::write_unaligned(fin as *mut i64, cpv6_saldo(fin).wrapping_sub(v as i64)); // cqc1: o dividendo sai do saldo
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_FINANCE_POST_DIVIDEND_NEWS);
    noticia(this, clube, v);
}

/// O comparador do fim de temporada do clube: lado a lado (a cópia sem contar os sorteios) nas finanças do parceiro e do
/// próprio clube, no clube, no estádio (78 bytes), na linha dos pedidos (0x49 bytes) e no estado do sorteio. As notícias
/// (0x59ffb0, 0x59fae0, 0x59fe10) são montadas na pilha e só são entregues ao técnico humano (0x7861e0): com técnico da
/// máquina elas não deixam nada além do sorteio(2) da recusa, que as duas rodadas fazem. Só a original: o técnico
/// humano (a notícia entra na caixa dele) e o estádio dividido (mexe noutro clube e nas finanças dele).
pub(crate) unsafe extern "thiscall" fn comparar_fim_de_temporada_do_clube(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIM_DE_TEMPORADA_DO_CLUBE);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if clube == 0 || selecao(clube) != 0 || cqc1_tecnico_humano(clube) {
        return o(this, clube);
    }
    let mut a = 0u32;
    let fin = cqc1_financas_do_parceiro(this, clube, &mut a);
    let estadio = read_usize(clube + club::STADIUM);
    if fin == 0 || (estadio != 0 && read_u8(clube + club::OWN_STADIUM) == 0 && read_usize(clube + club::DIVISION) != 0 && read_i32(fin) > 0x2160ec0) {
        return o(this, clube); // ponytail: a compra do estádio pode mexer no dono; só a original
    }
    let proprio = read_usize(this).wrapping_add((read_i32(clube) as u32).wrapping_mul(0x167) as usize);
    let mut reg = vec![(fin, 0x167), (clube, 0x245), (cqc1_linha_dos_pedidos(clube), 0x49)];
    if proprio != fin {
        reg.push((proprio, 0x167));
    }
    if estadio != 0 {
        reg.push((estadio, 0x4e));
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || crate::aleatorio::sem_contar(|| fim_de_temporada_do_clube(this, clube)));
    PLACAR.registrar(igual, || format!("fim_de_temporada_do_clube(clube {:x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// news 0x177c with the club id, the amount and the +0x82 flag, sent to the club manager
/// Original: `FinanceTable::news_dividend` (0x59fe10, fin_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x59fe10 } (thiscall, ret 8, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_dividend(clube, valor)`, pelo fim de temporada do clube (0x59cfa0). A notícia 0x177c (0x77bbe0:
/// ecx = local; 2 pushes -> thiscall, ret 8) com o id do clube, o valor e a marca +0x82 (0x785790: ecx = local; 2
/// pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn noticia_do_dividendo(_this: usize, clube: usize, valor: i32) {
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

/// O comparador da notícia do dividendo: só a original (ponytail: a notícia vai para a caixa do técnico humano e não
/// pode ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_do_dividendo(this: usize, clube: usize, valor: i32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_NOTICIA_DO_DIVIDENDO)(this, clube, valor)
}

// ---------- 266a leva: o teto salarial: o salário máximo que o clube oferece e as notícias das obras e das obras recusadas (fin_prm.cpp) ----------





pub(crate) unsafe fn cqc2_q(a: usize) -> i64 {
    core::ptr::read_unaligned(a as *const i64)
}

/// `v` × a constante f64 em `k`, truncado (_ftol).
pub(crate) unsafe fn cqc2_vezes(v: i32, k: usize) -> i32 {
    ftol(v as f64 * f64_em(k))
}

/// O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, a, 1, 0, 0)).
pub(crate) unsafe fn cqc2_humano(clube: usize, a: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, a, 1, 0, 0)
}


/// A seleção (0x53b900: push; add esp, 4 -> cdecl).
pub(crate) unsafe fn cqc2_selecao(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    f(clube) != 0
}

/// O valor do contrato da pessoa (0x4d0c20: mov ecx, [0xae1050]; push pessoa -> thiscall, ret 4), com sinal.
pub(crate) unsafe fn cqc2_valor(pessoa: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_WAGE);
    f(read_usize(G_CONTRACT_MANAGER), pessoa) as i32
}

/// O bônus de quem paga mais (o trecho 0x596c0d..0x597238): clube da Inglaterra, Itália ou Espanha, reputação acima
/// de 4750, faixa 0 a 2, pelo fator salarial `w` (a palavra, com sinal) e pela marca +0x82.
pub(crate) unsafe fn cqc2_bonus(clube: usize, faixa: i8, w: i32) -> i32 {
    let nac = read_usize(clube + club::NATION);
    if nac == 0 {
        return 0;
    }
    let n = read_i32(nac);
    if n != read_i32(KEY_NATION_ENGLAND) && n != read_i32(KEY_NATION_ITALY) && n != read_i32(KEY_NATION_SPAIN) {
        return 0;
    }
    if read_i16(clube + club::REPUTATION) <= 0x128e || !(faixa == 2 || faixa == 1 || faixa == 0) {
        return 0;
    }
    let w16 = w as i16;
    let x = w16 as i32;
    let (base, oito) = if read_u8(clube + club::PLC) != 0 { (0x4e2, 0x2710) } else { (0x2ee, 0x1770) };
    if w16 < 0x1676 || faixa == 0 {
        x.wrapping_sub(base).wrapping_mul(6)
    } else if faixa == 1 {
        x.wrapping_mul(8).wrapping_sub(oito)
    } else {
        x.wrapping_sub(base).wrapping_mul(10)
    }
}

/// O teto do técnico humano pelas reputações (+0xb, +0xd) e pela experiência (+5) do não-jogador `np`, pela idade
/// (+0x18) e pela marca +0x22 da pessoa e pelo cargo (o trecho 0x597405..0x597566).
pub(crate) unsafe fn cqc2_teto_do_humano(pessoa: usize, np: usize, cargo: i8) -> i32 {
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
pub(crate) unsafe extern "thiscall" fn teto_salarial(fin: usize, pessoa: usize, cargo_arg: u32) -> i32 {
    let mut cargo = cargo_arg as u8;
    let clube = read_usize(G_CLUB_TABLE).wrapping_add((read_i32(fin + 8) as u32).wrapping_mul(0x245) as usize);
    let verba = ftol(cqc2_q(fin) as f64 * f64_em(0x968de8));
    let mut liga: i32 = 100;
    let mut fator: f64 = 0.25; // [esp + 0x3c]: 0x3fd00000_00000000
    let mut contrato: usize = 0;
    if read_u8(clube + club::PROFESSIONAL_STATUS) == 3 {
        return 0;
    }
    let rep = read_i16(clube + club::REPUTATION);
    let mut di: i32 = rep as i32 / 50;
    let faixa: i8 = if cqc2_selecao(clube) { 0 } else { cpv6_faixa(fin) };
    let nac = read_usize(clube + club::NATION);
    let e = |g: usize| read_i32(nac) == read_i32(g);
    let tabela: usize = if nac == 0 || e(KEY_NATION_WALES) || e(KEY_NATION_REPUBLIC_OF_IRELAND) || e(KEY_NATION_NORTHERN_IRELAND) || e(KEY_NATION_AUSTRALIA) {
        0x9c8498
    } else if [KEY_NATION_JAPAN, KEY_NATION_SWEDEN, KEY_NATION_CROATIA, KEY_NATION_POLAND, KEY_NATION_NORWAY, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, KEY_NATION_DENMARK, KEY_NATION_FINLAND, KEY_NATION_SCOTLAND, KEY_NATION_GERMANY].iter().any(|&g| e(g)) {
        0x9c8548
    } else if e(KEY_NATION_FRANCE) || (read_u8(clube + club::PLC) != 0 && faixa != 2 && faixa != 1) {
        0x9c85f8
    } else {
        0x9c86a8
    };
    let padrao = tabela == 0x9c86a8;
    match faixa {
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
    let mut s = ftol((di as i16) as f64 * f64_em(0x96a5e0));
    if (s as i16) > 0x2710 {
        s = 0x2710;
    }
    if (di as i16) > 0xd2 {
        di = 0xd2;
    }
    let humano = cqc2_humano(clube, 0);
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
            r = cqc2_valor(pessoa);
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
        b20 = cqc2_bonus(clube, faixa, s);
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
                    f64_em(0x967890)
                } else if e(KEY_NATION_UNITED_STATES) {
                    f64_em(0x969040)
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
                        f64_em(k)
                    } else {
                        let div = if bl < 1 { 1 } else { bl as i32 };
                        let mut m = d16 as f64 / div as f64 * f64_em(0x967880) * f64_em(k);
                        if tres && !(m <= f64_em(0x969040)) {
                            m = f64_em(0x969040);
                        } else if !(m <= f64_em(0x967890)) {
                            m = f64_em(0x967890);
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
                    let t1 = ftol(cubo(c16 as i32) as f64 * f * f64_em(0x96a5c8));
                    let t2 = ftol(cubo(read_i16(d2 + 0x69) as i32) as f64 * f * f64_em(0x968928));
                    t2.wrapping_sub(t1)
                } else {
                    ftol(cubo(c16 as i32) as f64 * f * f64_em(0x968e18))
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
        let t = ftol(a * a * a * f / q10 as f64);
        crate::comparacao::ramo("cqc_2 teto pela tabela");
        let menos = read_i32(fin + 0xc).wrapping_sub(t.wrapping_add(verba) / 52); // cqc2: a folha e a verba por semana
        let idx = ((d16 / 10) * 8) as isize as usize;
        let vv = cqc2_q(tabela.wrapping_add(idx)).wrapping_sub(menos as i64) as f64 * f;
        let piso = cqc2_q((if padrao { 0x9c83e8usize } else { 0x9c8338 }).wrapping_add(idx)) as f64 * f;
        r = if vv <= piso { ftol(piso) } else { ftol(vv) };
        if !cqc2_selecao(clube) {
            match cpv6_faixa(fin) {
                1 => r = cqc2_vezes(r, 0x9689b8),
                2 => r = cqc2_vezes(r, 0x9689c0),
                _ => {}
            }
            if nac != 0 {
                let ita_esp = e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN);
                if ita_esp {
                    if rep > 0x1e46 {
                        r = cqc2_vezes(r, 0x9689b8);
                    }
                } else if e(KEY_NATION_ENGLAND) && rep > 0x1e46 {
                    r = cqc2_vezes(r, 0x968fa0);
                }
                if read_u8(clube + club::PLC) == 0 && rep > 0x1e46 && ita_esp {
                    r = cqc2_vezes(r, 0x9689b8);
                }
            }
        }
        b20 = cqc2_bonus(clube, faixa, s);
    }
    if pessoa != 0 {
        let repf = rep as i32 as f64;
        let t = repf * repf * fator * f64_em(0x96a5c0) + read_i32(fin + 0x10) as f64;
        let v = cqc2_valor(pessoa);
        s = if (v as f64) < t {
            crate::comparacao::ramo("cqc_2 teto pela reputacao");
            ftol(repf * repf * fator * f64_em(0x96a5c0) + read_i32(fin + 0x10) as f64) // cqc2: a reputação ao quadrado
        } else {
            cqc2_valor(pessoa)
        };
        let np = read_usize(pessoa + staff::PLAYER);
        let c = cargo as i8;
        if np != 0 && read_i16(np + 0xb) > 0x1482 && (c == 3 || c == 2 || c == 1) && (read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8) < 0x20 {
            let np2 = || read_usize(pessoa + staff::PLAYER);
            if c == 3 {
                let bf = b20 as f64;
                let lim = bf * f64_em(0x9689b0);
                if (r as f64) < lim {
                    r = if cqc2_humano(clube, 0) == 0 && np2() != 0 && read_i16(np2() + 0xb) > 0x1c52 { ftol(bf * f64_em(0x9689d8)) } else { ftol(lim) };
                }
            } else if c == 2 {
                let lim = b20 as f64 * f64_em(0x9689d8);
                if (r as f64) < lim {
                    r = if cqc2_humano(clube, 0) == 0 && np2() != 0 && read_i16(np2() + 0xb) > 0x1e46 { b20 } else { ftol(lim) };
                }
            } else if r < b20 {
                r = b20;
            }
        }
        if cqc2_humano(clube, 0) != 0 {
            let np = read_usize(pessoa + staff::PLAYER);
            if np != 0 {
                let teto = cqc2_teto_do_humano(pessoa, np, cargo as i8);
                if teto < r {
                    r = teto.wrapping_sub(ftol(r.wrapping_sub(teto) as f64 * f64_em(0x969518)));
                }
            }
        }
        let np = read_usize(pessoa + staff::PLAYER);
        if np != 0 {
            if nac != 0 && (e(KEY_NATION_ITALY) || e(KEY_NATION_ENGLAND) || e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SPAIN)) {
                let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_BASQUE_CLUB);
                if basco(clube) != 0 {
                    s = cqc2_vezes(s, 0x9689b8);
                }
                let tm = read_usize(G_TRANSFER_MANAGER);
                if tm != 0 {
                    let rr = read_i16(clube + club::REPUTATION);
                    let div: i32 = if rr < 0x3e8 { 0x3e8 } else { rr as i32 };
                    let tab = read_usize(read_usize(tm + 0x856) + 0xdc);
                    let x = read_i16(tab.wrapping_add((read_i32(clube) as usize).wrapping_mul(9)) + 5) as i32;
                    let mut m = x as f64 / div as f64;
                    if !(f64_em(0x967890) < m || f64_em(0x967890) == m) {
                        m = f64_em(0x967890);
                    } else if f64_em(0x968978) < m {
                        m = f64_em(0x968978);
                    }
                    s = ftol(s as f64 * m);
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
                    s = cqc2_vezes(s, k);
                }
            }
            let pres = read_usize(clube + club::CHAIRMAN);
            if pres != 0 && read_usize(pres + 0x69) != 0 && (read_i16(clube + club::REPUTATION) as i32) < read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) as i32 + 0x4e2 {
                crate::comparacao::ramo("cqc_2 presidente");
                s = ftol(((read_u8(pres + 0x57) as i8 as i32) as f64 * f64_em(0x967898) + f64_em(0x9689d8)) * s as f64); // cqc2: o presidente
                // 0x547810 (push pres; push pessoa; add esp, 8 -> cdecl), 0x547790 (push clube; push pessoa; add esp, 8 -> cdecl)
                let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
                let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
                if gosta(pessoa, pres) != 0 || favorito(pessoa, clube) != 0 {
                    s = ftol(((read_u8(read_usize(clube + club::CHAIRMAN) + 0x57) as i8 as i32) as f64 * f64_em(0x967898) + f64_em(0x967890)) * s as f64);
                }
                let pres = read_usize(clube + club::CHAIRMAN);
                let pc = read_usize(pres + 0x69);
                let perfil = read_u8(pc + 0xf) as i8;
                if perfil < 6 {
                    let t = read_usize(clube + club::MANAGER);
                    let k = if t != 0 && (read_u8(t + 0x57) as i8) > 0xf && read_usize(t + 0x69) != 0 && read_i16(read_usize(t + 0x69) + 0xa) > 0x1c52 { 0x9689c0 } else { 0x9689b8 };
                    s = cqc2_vezes(s, k);
                } else if perfil > 0xf {
                    s = cqc2_vezes(s, 0x9689d0);
                }
                if read_u8(clube + club::PLC) == 0 && (read_u8(pc + 0x20) as i8) > 0xf && (read_u8(pres + 0x57) as i8) > 0xf {
                    let rr = read_i16(clube + club::REPUTATION);
                    let k = if rr >= 0xabe && read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) >= 0x1c52 {
                        if rr < 0x1676 { 0x9689c0 } else { 0x96a5a8 }
                    } else {
                        0x9689b8
                    };
                    s = cqc2_vezes(s, k);
                }
            }
            let c = cargo as i8;
            let np = read_usize(pessoa + staff::PLAYER);
            match c {
                1 => s = cqc2_vezes(s, 0x968928),
                2 => {
                    if cqc2_humano(clube, 1) == 0 && read_i16(np + 0xb) > 0x1a5e {
                        s = cqc2_vezes(s, 0x968928);
                    } else {
                        s = ftol(s as f64);
                        let ok = cqc2_humano(clube, 1) != 0 || {
                            let np = read_usize(pessoa + staff::PLAYER);
                            np != 0 && read_i16(np + 0xb) < 0x1e46
                        };
                        if ok && s > 0x14c08 {
                            s = ftol(s.wrapping_sub(0x14c08) as f64 * f64_em(0x968918) + f64_em(0x96a5a0));
                        }
                    }
                }
                3 => {
                    if cqc2_humano(clube, 1) == 0 && read_i16(np + 0xb) > 0x1a5e {
                        s = cqc2_vezes(s, 0x968928);
                    } else {
                        s = cqc2_vezes(s, 0x9689d0);
                        if s > 0xd6d8 {
                            s = 0xd6d8;
                        }
                    }
                }
                4 => {
                    let k = if cqc2_humano(clube, 1) == 0 && read_i16(np + 0xb) > 0x1a5e { 0x9689d0 } else { 0x969040 };
                    s = cqc2_vezes(s, k);
                    if s > 0x927c {
                        s = 0x927c;
                    }
                }
                5 => {
                    s = if s > 0xc80 { ftol(s.wrapping_sub(0xc80) as f64 * f64_em(0x968e80) + f64_em(0x96a598)) } else { cqc2_vezes(s, 0x968e48) };
                    if s > 0x30d4 {
                        s = 0x30d4;
                    }
                }
                6 => {
                    s = if s > 0xc80 { ftol(s.wrapping_sub(0xc80) as f64 * f64_em(0x968f20) + f64_em(0x96a590)) } else { cqc2_vezes(s, 0x968968) };
                    if s > 0x203a {
                        s = 0x203a;
                    }
                }
                _ => {}
            }
        }
    } else {
        let x = (s as i16) as i32 as f64;
        s = ftol(x * x * fator * f64_em(0x96a5c0) + read_i32(fin + 0x10) as f64);
        if s < b20 {
            s = ftol(s as f64 * f64_em(0x969040) + b20 as f64 * f64_em(0x9689e0));
        }
        if !cqc2_selecao(clube) && nac != 0 {
            if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) {
                match cpv6_faixa(fin) {
                    1 => s = cqc2_vezes(s, 0x9689b8),
                    2 => s = cqc2_vezes(s, 0x9689c0),
                    _ => {}
                }
            } else if e(KEY_NATION_ITALY) || e(KEY_NATION_ENGLAND) || e(KEY_NATION_SPAIN) {
                match cpv6_faixa(fin) {
                    0 => {
                        if read_i16(clube + club::REPUTATION) > 0x109a {
                            s = cqc2_vezes(s, 0x968fa0);
                        }
                    }
                    1 => s = cqc2_vezes(s, 0x9689b8),
                    2 => s = cqc2_vezes(s, 0x9689c0),
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
            s = ftol((s as f64 - f32_em(0x96a5b8) as f64) * f64_em(0x968918) + f64_em(0x96a5b0));
        }
    } else if tipo == 2 {
        s = cqc2_vezes(s, 0x969040);
        if read_i16(clube + club::REPUTATION) < 0xabe && s > 0x4b0 {
            s = ftol((s as f64 - f32_em(0x96a588) as f64) * f64_em(0x968918) + f64_em(0x96a580));
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

/// O comparador do teto salarial: pela devolução (a rotina só lê; as chamadas de dentro também).
pub(crate) unsafe extern "thiscall" fn comparar_teto_salarial(fin: usize, pessoa: usize, cargo: u32) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_TETO_SALARIAL)(fin, pessoa, cargo);
    let n = teto_salarial(fin, pessoa, cargo);
    PLACAR.registrar(r == n, || format!("teto_salarial(fin {:x}, pessoa {:x}, cargo {}) original={} rust={}", fin, pessoa, cargo as u8 as i8, r, n));
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
pub(crate) unsafe extern "thiscall" fn noticia_das_obras(_this: usize, clube: usize, valor: u32, tipo: u32) {
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

/// O comparador da notícia das obras: só a original (ponytail: a notícia vai para a caixa do técnico humano e não pode
/// ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_das_obras(this: usize, clube: usize, valor: u32, tipo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NOTICIA_DAS_OBRAS)(this, clube, valor, tipo)
}

/// no club, nothing; news 0x1788 with the subject and the reason (2, or a draw of 2), sent to the club manager
/// Original: `FinanceTable::news_request_refused` (0x59fae0, fin_prm.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x59fae0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_request_refused(clube, assunto, motivo)`, pelo fim de temporada do clube (0x59cfa0: o pedido de
/// estádio ou de centro de treino recusado). Sem clube, nada. A notícia 0x1788 (0x77bbe0) com o assunto (palavra com
/// sinal) no dado 1 e no dado 2 o motivo 2 ou o sorteio(2) (0x90cfc0: push 2; add esp, 4 -> cdecl; a palavra com
/// sinal), entregue ao técnico do clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn noticia_da_obra_recusada(_this: usize, clube: usize, assunto: u32, motivo: u32) {
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

/// O comparador da notícia da obra recusada: só a original (ponytail: entrega ao técnico humano e sorteia; não pode
/// rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_obra_recusada(this: usize, clube: usize, assunto: u32, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NOTICIA_DA_OBRA_RECUSADA)(this, clube, assunto, motivo)
}

// ---------- 265a leva: o save das finanças e do gerente de elencos: gravar e carregar o finance.dat e gravar o squad_manager.dat (fin_prm.cpp, squad_manager.cpp) ----------


pub(crate) unsafe fn cpx5_clubes() -> u32 {
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
pub(crate) unsafe extern "thiscall" fn gravar_financas(this: usize, save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("cpx_5 gravar financas");
    if save == 0 {
        return;
    }
    let mut z = abrir_item(save, 0x9c87b0, 1, 0x16, 4);
    if z == 0 {
        aviso_em(0x9c8784, 0x115e);
        return;
    }
    if gravar_item(read_usize(this), 0x167, cpx5_clubes(), z) != cpx5_clubes() { // cpx5: os registros das finanças
        aviso_em(0x9c8784, 0x1166);
        return;
    }
    fechar_item(&mut z);
}

/// Comparador da 0x59b3b0: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_gravar_financas(this: usize, save: usize) {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .qst gravado no modo rust com e sem a leva, item a item)
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_GRAVAR_FINANCAS)(this, save)
}

/// reads the finances of every club from the finance.dat item into [this] and returns 1; no container 0; on error the
/// "Error" box (0x1137, 0x113f) and 0 (the constructor rebuilds the finances from scratch)
/// Original: `FinanceTable::load_from_save` (0x59b2a0, fin_prm.cpp, thiscall).
///
/// [pt] { 0x59b2a0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::load_from_save(save)`: lê do item `finance.dat`
/// (0x9326e0 lendo) as finanças de todos os clubes, 0x167 bytes × [0xae235c] em [this], e devolve 1. Sem contêiner,
/// 0 sem caixa; erro ao abrir ou leitura curta: a caixa "Error" (finance.CPP 0x1137, 0x113f), 0 e o item fica aberto
/// (como a original). Com 0, o construtor (0x59a55c) refaz as finanças de cada clube do zero (0x5961d0).
pub(crate) unsafe extern "thiscall" fn carregar_financas(this: usize, save: usize) -> u32 {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("cpx_5 carregar financas");
    if save == 0 {
        return 0;
    }
    let mut z = abrir_item(save, 0x9c87b0, 0, 0x16, 4);
    if z == 0 {
        aviso_em(0x9c8784, 0x1137);
        return 0;
    }
    if ler_item(read_usize(this), 0x167, cpx5_clubes(), z) != cpx5_clubes() {
        aviso_em(0x9c8784, 0x113f);
        return 0;
    }
    fechar_item(&mut z);
    1 // cpx5: carregou (o construtor não refaz as finanças)
}

/// Comparador da 0x59b2a0: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_carregar_financas(this: usize, save: usize) -> u32 {
    // ponytail: lê do contêiner (a posição do item anda; não pode rodar duas vezes); a cópia é provada pela carga
    // do save de férias na prova do save (levas/N/CARGA)
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CARREGAR_FINANCAS)(this, save)
}



// ---------- 264a leva: a renda das partidas: a renda da partida nas finanças dos dois clubes, a renda do estádio e a multa da liga (fin_prm.cpp) ----------






/// O registro de finanças (0x167 bytes) do clube da partida em `campo` (+0x1c o mandante, +0x20 o visitante), como a
/// original o acha sem a 0x5a0590: o clube parceiro (0x540a50: 3 pushes; add esp, 0xc -> cdecl(clube, &a, 1)) se ele
/// existe e o clube é o B (a = 0), e [tabela] + id × 0x167. Some(0) sem clube; None se for seleção (0x53b900: push;
/// add esp, 4 -> cdecl): a original sai sem fazer nada.
pub(crate) unsafe fn cqb1_financas(tabela: usize, partida: usize, campo: usize) -> Option<usize> {
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

/// A divisão com o arredondamento para zero da original (os imul por constante mágica e o cdq/sar).
pub(crate) fn cqb1_div(a: i32, b: i32) -> i32 {
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
pub(crate) unsafe extern "thiscall" fn renda_da_partida(tabela: usize, partida: usize, resultado: usize) {
    if resultado == 0 || partida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RENDA_DA_PARTIDA)(tabela, partida, resultado); // a caixa
    }
    let Some(mandante) = cqb1_financas(tabela, partida, 0x1c) else { return };
    let Some(visitante) = cqb1_financas(tabela, partida, 0x20) else { return };
    let (mut tv, mut publico, mut renda) = (0i32, 0i32, 0i32);
    if read_i32(resultado + 0x34) != 0 {
        publico = read_i32(resultado + 0x34);
        renda = read_i32(resultado + 0x38);
        let q = cqb1_div(1, cpe1_sorteio(10).wrapping_add(2));
        crate::comparacao::ramo("cqb_1 publico pagante");
        tv = ftol((q as f64 + f64_em(0x968f98)) * publico as f64); // cqb1: a TV pelo público (1,75)
    }
    let casa = read_usize(partida + 0x1c);
    let fora = read_usize(partida + 0x20);
    let mut custo; // o ebp da original
    if read_u8(casa + 0x64) != 1 && read_u8(fora + 0x64) != 1 {
        custo = ftol(publico as f64 * f64_em(0x9689e0));
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
            publico as f64 * f64_em(k)
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
            cap as f64 * f64_em(k)
        };
        custo = ftol(x);
        if read_u8(partida + 0x40) != 0 && custo < 500 {
            custo = 500;
        }
    }
    let casa = read_usize(partida + 0x1c);
    let mut gastos; // o [esp+0x10] da original (começa com o público)
    if read_u8(casa + 0x64) == 1 {
        let rep = read_i16(casa + 0x80);
        if rep != 0 {
            let r = cpe1_sorteio(0xc8);
            let x = (r as f64 + publico as f64 * f64_em(0x968928)) + rep as f64 * f64_em(0x968968) + f64_em(0x96a6b0);
            gastos = ftol(x);
            if rep > 0x128e {
                gastos = ftol(gastos as f64 * f64_em(0x968928));
            }
        } else {
            let r = cpe1_sorteio(0x190);
            gastos = ftol((r as f64 + publico as f64 * f64_em(0x968928)) + f64_em(0x96a690));
        }
    } else {
        let rep = read_i16(casa + 0x80);
        let r1 = if rep != 0 { cpe1_sorteio(ftol(rep as i32 as f64 * f64_em(0x9689e0))) } else { cpe1_sorteio(0xfa) };
        let r1f = r1 as f32; // fstp dword: o primeiro sorteio guardado em float
        let r2 = cpe1_sorteio(0xfa);
        gastos = ftol((r2 as f64 + r1f as f64) + publico as f64);
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
            dinheiro = cqb1_div(m.wrapping_mul(if t == 6 { 125000 } else { 25000 }), 20);
        }
    }
    let nac = read_usize(casa + 0x53);
    let tipo = if nac != 0 { read_u8(nac + 0x7e) as i8 } else { 3 };
    if tipo == 2 {
        dinheiro = cqb1_div(dinheiro, 5);
        tv = cqb1_div(tv, 2);
        custo = cqb1_div(custo, 4);
        gastos = cqb1_div(gastos, 2);
    } else if tipo != 1 {
        dinheiro = cqb1_div(dinheiro, 10);
        tv = cqb1_div(tv, 10);
        custo = cqb1_div(custo, 15);
        gastos = cqb1_div(gastos, 10);
    }
    if casa != 0 && nac != 0 {
        let n = read_i32(nac);
        if n == read_i32(KEY_NATION_WALES) || n == read_i32(KEY_NATION_REPUBLIC_OF_IRELAND) || n == read_i32(KEY_NATION_NORTHERN_IRELAND) {
            dinheiro = cqb1_div(dinheiro, 5);
            tv = cqb1_div(tv, 2);
            custo = cqb1_div(custo, 4);
            gastos = cqb1_div(gastos, 2);
        }
    }
    let liquido = tv.wrapping_sub(gastos).wrapping_sub(custo).wrapping_add(renda);
    let somar = |fin: usize, a: usize, b: usize, v: i32| {
        cpz12_somar32(fin + a, v);
        cpz12_somar32(fin + b, v);
        cpz13_somar_saldo(fin, v as i64);
    };
    if read_u8(partida + 0x3c) == 0 && read_usize(read_usize(partida + 0x1c) + 0x57) == read_usize(partida + 0x14) {
        let x = dinheiro as f64;
        somar(mandante, 0x158, 0xb8, ftol(x * f64_em(0x969040)));
        somar(visitante, 0x158, 0xb8, ftol(x * f64_em(0x9689e0)));
        crate::comparacao::ramo("cqb_1 liquido ao mandante");
        somar(mandante, 0x140, 0xa0, liquido); // cqb1: o líquido ao mandante
        let casa = read_usize(partida + 0x1c);
        let est = read_usize(casa + 0x69);
        if read_u8(casa + 0x6d) != 0 && est != 0 {
            let base = read_i32(est + 0x3c);
            let r = cpe1_sorteio(0xf).wrapping_add(1);
            let c = ftol((r as f64 * f64_em(0x967898) + f64_em(0x969520)) * base as f64);
            cpz12_somar32(mandante + 0x12c, c);
            cpz12_somar32(mandante + 0x8c, c);
            cpz13_somar_saldo(mandante, -(c as i64));
        }
        return;
    }
    let metade = ftol(dinheiro as f64 * f64_em(0x968968));
    somar(mandante, 0x158, 0xb8, metade);
    somar(visitante, 0x158, 0xb8, metade);
    let l2 = cqb1_div(liquido, 2);
    somar(mandante, 0x140, 0xa0, l2);
    somar(visitante, 0x140, 0xa0, l2);
}

/// Comparador da 0x59a5d0: lado a lado nos dois registros de finanças e no estado do sorteio (os sorteios de 10, dos
/// gastos e da manutenção). Sem resultado, sem partida, sem um dos clubes ou com seleção: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_renda_da_partida(tabela: usize, partida: usize, resultado: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RENDA_DA_PARTIDA);
    if resultado == 0 || partida == 0 || read_usize(partida + 0x1c) == 0 || read_usize(partida + 0x20) == 0 {
        return o(tabela, partida, resultado);
    }
    let (Some(fm), Some(fv)) = (cqb1_financas(tabela, partida, 0x1c), cqb1_financas(tabela, partida, 0x20)) else {
        return o(tabela, partida, resultado); // seleção: a original não faz nada
    };
    if fm == fv {
        return o(tabela, partida, resultado); // ponytail: o mesmo registro nos dois lados, só a original
    }
    let mut reg = vec![(fm, 0x167), (fv, 0x167)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tabela, partida, resultado), || crate::aleatorio::sem_contar(|| renda_da_partida(tabela, partida, resultado)));
    PLACAR.registrar(igual, || format!("renda_da_partida(partida {:x}) original={:x?} rust={:x?}", partida, mo, mr));
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
pub(crate) unsafe extern "thiscall" fn renda_do_estadio(tabela: usize, partida: usize, publico: i32) -> i32 {
    if partida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_RENDA_DO_ESTADIO)(tabela, partida, publico); // a caixa
    }
    if publico == 0 {
        return 0;
    }
    let Some(fin) = cqb1_financas(tabela, partida, 0x1c) else { return 0 };
    let comp = read_usize(partida + 0x14);
    let mut v: i32;
    if read_u8(partida + 0x3c) != 0 && comp != 0 {
        v = (read_i16(comp + competition::CLUB_COMP_REPUTATION) as i32).wrapping_add(8).wrapping_mul(publico);
    } else {
        let casa = read_usize(partida + 0x1c);
        crate::comparacao::ramo("cqb_1 preco pela reputacao");
        let preco = cqb1_div(read_i16(casa + 0x80) as i32, 500); // cqb1: o preço pela reputação
        let amistoso = comp != 0 && {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
            f(read_usize(G_FRIENDLY), comp) != 0
        };
        if comp == 0 || amistoso {
            v = ftol(preco as f64 * f64_em(0x968968) * publico as f64);
        } else if read_u8(casa + 0x64) == 1 || read_u8(read_usize(partida + 0x20) + 0x64) == 1 {
            v = preco.wrapping_add(4).wrapping_mul(publico);
        } else {
            v = preco.wrapping_add(3).wrapping_mul(publico);
        }
        if fin != 0 && read_usize(casa + 0x57) == comp {
            let desconto = *((fin + 0x160) as *const f32) as f64;
            v = ftol(v as f64 - desconto * v as f64);
        }
    }
    let casa = read_usize(partida + 0x1c);
    let nac = read_usize(casa + 0x53);
    let tipo = if nac != 0 { read_u8(nac + 0x7e) as i8 } else { 3 };
    if tipo == 2 {
        v = cqb1_div(v, 4);
    } else if tipo != 1 {
        v = cqb1_div(v, 10);
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
        v = ftol(v as f64 * f64_em(k));
    }
    if nac != 0 && read_i32(nac) == read_i32(KEY_NATION_GERMANY) {
        v = ftol(v as f64 * f64_em(0x968978));
    }
    v
}

/// Comparador da 0x59af40: a devolução (só lê; a 0x540a50 pode zerar a marca +0x244 do clube, igual nas duas). Sem
/// partida: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_renda_do_estadio(tabela: usize, partida: usize, publico: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_RENDA_DO_ESTADIO);
    if partida == 0 {
        return o(tabela, partida, publico);
    }
    let r = o(tabela, partida, publico);
    let n = renda_do_estadio(tabela, partida, publico);
    PLACAR.registrar(r == n, || format!("renda_do_estadio(partida {:x}, publico {}) original={} rust={}", partida, publico, r, n));
    r
}

/// adds the fine to the season (+0x128) and month (+0x88) fines and takes it from the balance
/// Original: `Finance::pay_league_fine` (0x599850, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x599850 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_league_fine(valor)`, pelas multas da liga (0x49f570,
/// 0x6a9e60): soma o valor às multas do ano (+0x128) e do mês (+0x88) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pagar_multa_da_liga(this: usize, valor: i32) {
    cpz12_somar32(this + 0x128, valor);
    cpz12_somar32(this + 0x88, valor);
    cpz13_somar_saldo(this, -(valor as i64));
}

/// Comparador da 0x599850: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pagar_multa_da_liga(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAGAR_MULTA_DA_LIGA);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_multa_da_liga(this, valor));
    PLACAR.registrar(igual, || format!("pagar_multa_da_liga(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}

// ---------- 264a leva: os eventos raros das finanças: o impulso das finanças, a troca do presidente e os outros custos (fin_prm.cpp) ----------



/// O saldo (i64 em +0) separado em (baixo, alto), como a original o lê em dois dwords.
pub(crate) unsafe fn cqb2_saldo(fin: usize) -> (u32, i32) {
    (core::ptr::read_unaligned(fin as *const u32), core::ptr::read_unaligned((fin + 4) as *const i32))
}

/// O valor entra em +0x154, +0xb4 e no saldo (estendido com sinal).
pub(crate) unsafe fn cqb2_creditar(fin: usize, v: i32) {
    cpv5_somar(fin + 0x154, v);
    cpv5_somar(fin + 0xb4, v);
    cpv6_somar_saldo(fin, v);
}

/// A nação do clube é uma das de 0x9cf4a8, 0x9cf408 ou 0x9cf2c0 (o valor do evento cai a um terço).
pub(crate) unsafe fn cqb2_nacao_de_um_terco(clube: usize) -> bool {
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
pub(crate) unsafe extern "thiscall" fn impulso_das_financas(this: usize, clube: usize, fin: usize) -> u32 {
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
    let div = match cpv6_faixa(fin) {
        -2 => f64_em(if rep > 0xea6 { 0x968fd0 } else { 0x96a730 }),
        -1 => f64_em(if rep > 0x1482 { 0x967880 } else if rep > 0xcb2 { 0x96a730 } else { 0x967878 }),
        0 => {
            if rep <= 0x109a {
                let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && read_i8(pres + 0x57) > 0xf && read_i8(ficha + 0x20) > 0xf {
                    f64_em(0x96a740)
                } else {
                    crate::comparacao::ramo("cqb_2 divisor da faixa 0 baixa");
                    f64_em(0x96a738) // cqb2: o divisor da faixa 0 de reputação baixa
                }
            } else if rep < 0x186a {
                let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && read_i8(ficha + 0x20) > 0xf && read_i8(pres + 0x57) > 0xf {
                    f64_em(0x967888)
                } else {
                    f64_em(0x967878)
                }
            } else {
                f64_em(0x967878)
            }
        }
        _ => f64_em(0x96a350),
    };
    let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
    let valor;
    if pres != 0 && ficha != 0 {
        crate::comparacao::ramo("cqb_2 com a ficha");
        let r1 = cpe1_sorteio(ftol(f64_em(0x968dd8) / div)); // cqb2: o sorteio com a ficha
        let r2 = cpe1_sorteio(read_i8(ficha + 0xf) as i32);
        if r2 <= r1 {
            return 0;
        }
        let (lo, hi) = cqb2_saldo(fin);
        let n = read_i8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as i32;
        let r3 = cpe1_sorteio(n.wrapping_mul(1_000_000)) as i64;
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
        let mut v = ftol(((n as f64 * n as f64) * f64_em(0x968930) + f64_em(0x96a5f0)) * rep as f64 + f64_em(0x9688d8));
        v = if hi < 0 { v.wrapping_sub(lo as i32) } else { v.wrapping_add(lo as i32) };
        if read_i16(clube + club::REPUTATION) < 0x7d0 {
            let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
            if v > teto {
                v = teto;
            }
        }
        if cqb2_nacao_de_um_terco(clube) {
            v /= 3;
        }
        valor = v;
    } else {
        crate::comparacao::ramo("cqb_2 sem a ficha");
        let r = cpe1_sorteio(ftol(f64_em(0x96a728) / div)); // cqb2: o sorteio sem a ficha
        if rep as i32 <= r {
            return 0;
        }
        if cpe1_sorteio(8) != 0 {
            return 0;
        }
        let mut v = (read_i16(clube + club::REPUTATION) as i32).wrapping_add(1000).wrapping_mul(1000);
        if cqb2_nacao_de_um_terco(clube) {
            v /= 3;
        }
        valor = v;
    }
    cqb2_creditar(fin, valor);
    let verba = cpv6_verba(fin) as i32;
    let marca = read_u8(clube + club::PLC);
    let com_ficha = pres != 0 && ficha != 0;
    let mut tipo = 0u32;
    if verba <= 0 && marca == 0 {
        tipo = 3;
    } else if marca != 0 && cpe1_sorteio(4) == 0 {
        tipo = 5;
    }
    if tipo == 0 {
        if cpe1_sorteio(4) == 0 {
            if com_ficha {
                let r2 = read_i16(clube + club::REPUTATION) as i32;
                let t = ftol(r2.wrapping_mul(r2) as f64 * f64_em(0x968e80));
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
        if read_u8(clube + club::PLC) == 0 && read_i16(clube + club::REPUTATION) > 0x128e && cpe1_sorteio(4) == 0
            && (!com_ficha || (nac != 0 && read_i32(nac) != read_i32(KEY_NATION_ITALY)))
        {
            cpv6_noticia(this, clube, valor, verba as u32, 7);
            *((clube + club::PLC) as *mut u8) = 1;
            tipo = 7;
        } else {
            tipo = 1;
        }
    }
    if tipo != 7 {
        cpv6_noticia(this, clube, valor, verba as u32, tipo);
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

/// O comparador dos eventos raros, em ensaio (modelo CPV6_ENSAIO: a cópia sem a notícia nem a verba, que alocam; sem a
/// troca do presidente na 0x59e940): a cópia e a original a partir do mesmo estado (o registro de finanças, o sorteio e
/// os `extras`). Sem evento: tudo igual; com evento: a devolução, o saldo, +0xb4, +0x154, +0x165 e os `conferir` (a
/// tipo da notícia e a marca +0x82 dependem da verba, e os sorteios depois dela também).
pub(crate) unsafe fn cqb2_comparar(nome: &str, fin: usize, extras: &[(usize, usize)], conferir: &[usize], original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend_from_slice(extras);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPV6_ENSAIO = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(copia));
    CPV6_ENSAIO = false;
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

pub(crate) unsafe extern "thiscall" fn comparar_impulso_das_financas(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IMPULSO_DAS_FINANCAS);
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
    cqb2_comparar("impulso_das_financas", fin, &extras, &conferir, || o(this, clube, fin), || impulso_das_financas(this, clube, fin))
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
pub(crate) unsafe extern "thiscall" fn troca_do_presidente(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = read_i16(clube + club::REPUTATION);
    let div = match cpv6_faixa(fin) {
        -2 => if rep > 0x109a { 0.4f64 } else { 0.25 },
        -1 => if rep > 0x109a { 0.2 } else { 0.05 },
        _ => 0.001,
    };
    let liga = read_usize(clube + club::DIVISION);
    let mut fator = if liga != 0 {
        let k = rep as i32 as f64 * f64_em(0x969048) / read_i16(liga + 0x69) as i32 as f64;
        if !(k >= f64_em(0x969040)) { f64_em(0x969040) } else { k } // fcomp: menor ou sem ordem (NaN) vira 0,75
    } else {
        0.5
    };
    let pres = read_usize(clube + club::CHAIRMAN);
    let ficha = if pres != 0 { read_usize(pres + 0x69) } else { 0 };
    let com_ficha = pres != 0 && ficha != 0;
    let primeiro = if com_ficha { cpe1_sorteio(read_i8(ficha + 0x16) as i32) } else { cpe1_sorteio(0x14) };
    if primeiro != 0 {
        return 0;
    }
    fator *= read_i16(clube + club::REPUTATION) as i32 as f64;
    let r = cpe1_sorteio(ftol(f64_em(0x96a728) / div));
    if r as f64 >= fator {
        return 0;
    }
    crate::comparacao::ramo("cqb_2 troca do presidente");
    let (lo, _) = cqb2_saldo(fin);
    let mut v = (read_i16(clube + club::REPUTATION) as i32).wrapping_add(1000).wrapping_mul(1000).wrapping_sub(lo as i32);
    if v > 0x2625a00 {
        let s = cpe1_sorteio(0x9c4);
        if (read_i16(clube + club::REPUTATION) as i32) < s.wrapping_add(0x7d0) {
            v = (cqb2_saldo(fin).0 as i32).wrapping_neg();
        }
    }
    cpv5_somar(fin + 0xb4, v);
    cpv5_somar(fin + 0x154, v);
    cpv6_somar_saldo(fin, v);
    let verba = cpv6_verba(fin);
    if com_ficha && !CPV6_ENSAIO {
        let pres = read_usize(clube + club::CHAIRMAN);
        let nac = read_usize(clube + club::NATION);
        let nova = if nac != 0 && cpe1_sorteio(0x3e8) != 0 {
            nac
        } else {
            let k = cpe1_sorteio(read_i32(0xae2350));
            read_usize(G_NATION_TABLE).wrapping_add((k as usize).wrapping_mul(0x122))
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
                let x = (cpe1_sorteio(0x14) as u8).wrapping_add(1);
                *((read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + o) as *mut u8) = x;
            }
            if read_i8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) < 5 {
                let x = (cpe1_sorteio(0x14) as u8).wrapping_add(1);
                *((read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as *mut u8) = x;
            }
        }
    }
    cpv6_noticia(this, clube, v, verba, 2);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_troca_do_presidente(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TROCA_DO_PRESIDENTE);
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // ponytail: sem clube ou sem finanças, só a original
    }
    cqb2_comparar("troca_do_presidente", fin, &[], &[], || o(this, clube, fin), || troca_do_presidente(this, clube, fin))
}

/// adds the amount to the season (+0x120) and month (+0x80) other costs and takes it from the balance
/// Original: `Finance::pay_other_costs` (0x5999e0, fin_prm.cpp, thiscall, ret 4).
///
/// [pt] { 0x5999e0 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_other_costs(valor)`, pela 0x7e5b10: soma o valor aos
/// outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pagar_outros_custos(this: usize, valor: i32) {
    cpv5_somar(this + 0x120, valor);
    cpv5_somar(this + 0x80, valor);
    cpz13_somar_saldo(this, -(valor as i64));
}

/// Comparador da 0x5999e0: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pagar_outros_custos(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAGAR_OUTROS_CUSTOS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_outros_custos(this, valor));
    PLACAR.registrar(igual, || format!("pagar_outros_custos(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}

// ---------- 262a leva: a folha salarial e a janela dos ingressos: a folha do clube, as folhas refeitas na criação e os dias até a temporada (finance.cpp) ----------






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
pub(crate) unsafe extern "thiscall" fn folha_salarial_do_clube(this: usize) {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32, u32) -> f64 = core::mem::transmute(FN_STAFF_CONTRACT_DEFAULT_STAFF_CONTRACT);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let teto: unsafe extern "thiscall" fn(usize, u32, i32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    write_i32(this + 0x10, 0);
    write_i32(this + 0xc, 0);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(0x245));
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
            w = ftol(padrao(c, p, clube, 0, -1, 0xb, 0));
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
            let c = contrato_de(read_usize(G_CONTRACT_MANAGER), p);
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
            ftol((rep - 0x8ca) as f64 * f64_em(0x968968) + f64_em(0x96a620))
        } else {
            ftol(rep as f64 * f64_em(0x9689e0))
        };
        if !tipo1 {
            v = ftol(v as f64 * f64_em(0x968968));
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
    crate::comparacao::ramo("cpz_11 folha pela reputacao");
    let folha = ftol(rep.wrapping_mul(rep) as f64 * f64_em(k) + read_i32(this + 0xc) as f64); // cpz11: a reputação na folha
    write_i32(this + 0xc, folha);
    let t = teto(this, 0, -1);
    write_i32(this + 0x14, t);
}

/// Comparador: lado a lado na folha (+0xc), no maior salário (+0x10), no teto (+0x14) e no último contrato consultado
/// ([0xae1080], que o salário padrão 0x864550 grava); as outras chamadas (contratos, teto 0x5968d0) só leem.
pub(crate) unsafe extern "thiscall" fn comparar_folha_salarial_do_clube(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_FOLHA_SALARIAL_DO_CLUBE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 12), (0xae1080, 4)], || o(this), || folha_salarial_do_clube(this));
    PLACAR.registrar(igual, || format!("folha_salarial_do_clube(fin {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// at game creation, the wage bill of every club up to the first national team
/// Original: `FinanceTable::refresh_all_wage_bills` (0x59cf50, finance.cpp, thiscall).
///
/// [pt] { 0x59cf50 } (thiscall, finance.cpp): `FinanceTable::refresh_all_wage_bills()`, só pela criação do jogo (0x828110):
/// para cada clube de [0xae23b4] até a primeira seleção (0x53b900), a folha (0x598370) do registro de finanças dele
/// ([this] + i × 0x167).
pub(crate) unsafe extern "thiscall" fn refazer_todas_as_folhas(this: usize) {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    let mut fin = read_usize(this);
    let mut i = 0i32;
    while i < read_i32(0xae235c) {
        let clube = read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(0x245));
        if selecao(clube) != 0 {
            break;
        }
        folha(fin);
        fin = fin.wrapping_add(0x167);
        i += 1;
    }
}

/// Comparador: lado a lado na tabela de finanças inteira ([this], [0xae235c] × 0x167 bytes) e em [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_refazer_todas_as_folhas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_REFAZER_TODAS_AS_FOLHAS);
    let n = (read_i32(0xae235c).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(read_usize(this), n), (0xae1080, 4)], || o(this), || refazer_todas_as_folhas(this));
    PLACAR.registrar(igual, || format!("refazer_todas_as_folhas({:#x}) original={:x?} rust={:x?}", this, mo, mr));
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
pub(crate) unsafe extern "thiscall" fn dias_ate_a_temporada(_this: usize, clube: usize, passou: *mut u32) -> i32 {
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    *passou = 0;
    let k = linha(clube) as i8;
    if k < 0 {
        aviso_em(0x9c8784, 0x1185);
        return 0;
    }
    let e = 0xb63d60usize.wrapping_add((k as isize * 72) as usize);
    let ano: u16 = if read_usize(e + 0x17) != 0 { 0x7d2 } else { (read_i16(G_GAME_START_DATE_YEAR) as u16).wrapping_add(1) };
    if read_i16(G_CURRENT_DATE_YEAR) as u16 >= ano {
        *passou = 1;
    }
    let inicio = read_u8(e + 0x34) as i8;
    let ano_do_inicio = read_i16(e + 0x2c) as u16;
    let mes = mes_de(0xae2c90) as i8;
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

/// Comparador: sem linha da nação, só a original (a caixa); senão lado a lado na saída `passou` e na data estática da
/// 0x54cdb0 (0xae2470), com a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_dias_ate_a_temporada(this: usize, clube: usize, passou: *mut u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(IDX_DIAS_ATE_A_TEMPORADA);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    if (linha(clube) as i8) < 0 {
        return o(this, clube, passou);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(passou as usize, 4), (0xae2470, 8)], || o(this, clube, passou), || dias_ate_a_temporada(this, clube, passou));
    PLACAR.registrar(igual && r == n, || format!("dias_ate_a_temporada(clube {:#x}) original={} {:x?} rust={} {:x?}", clube, r, mo, n, mr));
    r
}

// ---------- 262a leva: o dinheiro das transferências: a venda de jogador, a compra de jogador e as luvas (finance.cpp) ----------

/// Soma `v` (64 bits) ao i64 em `a` (o add/adc da original).
pub(crate) unsafe fn cpz12_somar64(a: usize, v: i64) {
    core::ptr::write_unaligned(a as *mut i64, core::ptr::read_unaligned(a as *const i64).wrapping_add(v));
}

/// Soma `v` ao i32 em `a`.
pub(crate) unsafe fn cpz12_somar32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, core::ptr::read_unaligned(a as *const i32).wrapping_add(v));
}

/// O valor de 64 bits que a original recebe em dois dwords (baixo primeiro).
pub(crate) fn cpz12_valor(baixo: u32, alto: u32) -> i64 {
    ((alto as u64) << 32 | baixo as u64) as i64
}

/// adds the amount (64 bits) to the sales of the month (+0x78) and of the year (+0x118) and to the balance
/// Original: `Finance::add_players_sold` (0x599800, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x599800 } (thiscall, ret 8, finance.cpp): `Finance::add_players_sold(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0 e as parcelas 0x8caf.., 0x8cb3..) e pela
/// parte dos clubes anteriores (0x8e4f20): soma o valor às vendas do mês (+0x78) e do ano (+0x118), em 64 bits, e ao
/// saldo (i64 em +0).
pub(crate) unsafe extern "thiscall" fn receber_venda_de_jogador(this: usize, baixo: u32, alto: u32) {
    let v = cpz12_valor(baixo, alto);
    cpz12_somar64(this + 0x78, v);
    cpz12_somar64(this + 0x118, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 venda recebida"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, v); // cpz12: a venda entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_receber_venda_de_jogador(this: usize, baixo: u32, alto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_RECEBER_VENDA_DE_JOGADOR);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || receber_venda_de_jogador(this, baixo, alto));
    PLACAR.registrar(igual, || format!("receber_venda_de_jogador(fin {:#x}, {}) original={:x?} rust={:x?}", this, cpz12_valor(baixo, alto), mo, mr));
}

/// adds the amount (64 bits) to the purchases of the month (+0x70) and of the year (+0x110) and takes it from the
/// balance
/// Original: `Finance::pay_players_bought` (0x599890, finance.cpp, thiscall, ret 8).
///
/// [pt] { 0x599890 } (thiscall, ret 8, finance.cpp): `Finance::pay_players_bought(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (0x8ca8a0) e pelas parcelas (0x8caf.., 0x8cb3..): soma o valor às compras do mês (+0x70) e do ano
/// (+0x110), em 64 bits, e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pagar_compra_de_jogador(this: usize, baixo: u32, alto: u32) {
    let v = cpz12_valor(baixo, alto);
    cpz12_somar64(this + 0x70, v);
    cpz12_somar64(this + 0x110, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 compra paga"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, v.wrapping_neg()); // cpz12: a compra sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pagar_compra_de_jogador(this: usize, baixo: u32, alto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_PAGAR_COMPRA_DE_JOGADOR);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || pagar_compra_de_jogador(this, baixo, alto));
    PLACAR.registrar(igual, || format!("pagar_compra_de_jogador(fin {:#x}, {}) original={:x?} rust={:x?}", this, cpz12_valor(baixo, alto), mo, mr));
}

/// adds the amount to the signing-on fees of the month (+0x9c) and of the year (+0x13c) and takes it from the balance
/// Original: `Finance::pay_sign_on_fee` (0x5998e0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5998e0 } (thiscall, ret 4, finance.cpp): `Finance::pay_sign_on_fee(valor)`, pela oferta de contrato aceita
/// (0x8c38b0) e pela mudança do jogador de clube (0x8c9720): soma o valor às luvas do ano (+0x13c) e do mês (+0x9c) e o
/// tira do saldo (estendido com sinal).
pub(crate) unsafe extern "thiscall" fn pagar_luvas(this: usize, valor: i32) {
    cpz12_somar32(this + 0x13c, valor);
    cpz12_somar32(this + 0x9c, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_12 luvas pagas"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, (valor as i64).wrapping_neg()); // cpz12: as luvas saem do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pagar_luvas(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAGAR_LUVAS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_luvas(this, valor));
    PLACAR.registrar(igual, || format!("pagar_luvas(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

// ---------- 262a leva: as receitas e a compensação: o prêmio em dinheiro, os direitos de TV e o salário pago (finance.cpp) ----------


/// Soma `v` (estendido com sinal: o cdq/adc da original) ao saldo, i64 em +0.
pub(crate) unsafe fn cpz13_somar_saldo(fin: usize, v: i64) {
    core::ptr::write_unaligned(fin as *mut i64, core::ptr::read_unaligned(fin as *const i64).wrapping_add(v));
}

/// adds the amount to the prize money of the month (+0xa8) and of the year (+0x148) and to the balance
/// Original: `Finance::add_prize_money` (0x5999a0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5999a0 } (thiscall, ret 4, finance.cpp): `Finance::add_prize_money(valor)`, pelas copas (Cup::process_fixture
/// 0x51a150, EuropeanCup::process_fixture 0x583b10, os sorteios das fases), pelo fim das ligas (League::pay_prize_money
/// 0x682e10) e pelas honras da temporada: soma o valor aos prêmios do ano (+0x148) e do mês (+0xa8) e ao saldo.
pub(crate) unsafe extern "thiscall" fn receber_premio_em_dinheiro(this: usize, valor: i32) {
    cpz12_somar32(this + 0x148, valor);
    cpz12_somar32(this + 0xa8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 premio recebido"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, valor as i64); // cpz13: o prêmio entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_receber_premio_em_dinheiro(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_RECEBER_PREMIO_EM_DINHEIRO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || receber_premio_em_dinheiro(this, valor));
    PLACAR.registrar(igual, || format!("receber_premio_em_dinheiro(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// adds the amount to the TV income of the month (+0xb8) and of the year (+0x158) and to the balance
/// Original: `Finance::add_tv_revenue` (0x5987c0, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5987c0 } (thiscall, ret 4, finance.cpp): `Finance::add_tv_revenue(valor)`, só pelos direitos de TV da liga
/// (League::pay_tv_rights 0x683010, pela montagem da tabela de cada liga na temporada nova): soma o valor à TV do ano
/// (+0x158) e do mês (+0xb8) e ao saldo.
pub(crate) unsafe extern "thiscall" fn receber_direitos_de_tv(this: usize, valor: i32) {
    cpz12_somar32(this + 0x158, valor);
    cpz12_somar32(this + 0xb8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 tv recebida"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, valor as i64); // cpz13: a TV entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_receber_direitos_de_tv(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_RECEBER_DIREITOS_DE_TV);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || receber_direitos_de_tv(this, valor));
    PLACAR.registrar(igual, || format!("receber_direitos_de_tv(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// adds the amount to the wages of the month (+0x90) and of the year (+0x130) and takes it from the balance (the free
/// release compensation)
/// Original: `Finance::pay_wages` (0x599920, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599920 } (thiscall, ret 4, finance.cpp): `Finance::pay_wages(valor)`, só pela liberação sem custo
/// (TRANSFER_MANAGER::free_transfer_staff 0x8c1af0: a compensação do contrato ao clube): soma o valor aos salários do ano
/// (+0x130) e do mês (+0x90) e o tira do saldo.
pub(crate) unsafe extern "thiscall" fn pagar_salarios(this: usize, valor: i32) {
    cpz12_somar32(this + 0x130, valor);
    cpz12_somar32(this + 0x90, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 salario pago"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, (valor as i64).wrapping_neg()); // cpz13: o salário sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pagar_salarios(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAGAR_SALARIOS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_salarios(this, valor));
    PLACAR.registrar(igual, || format!("pagar_salarios(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

// ---------- 258a leva: as receitas da semana e o centro de treino: o merchandising, os ingressos da temporada e a obra do centro de treino (finance.cpp) ----------





pub(crate) unsafe fn cpv5_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, read_i32(a).wrapping_add(v));
}

/// O saldo (i64 em +0 do registro de finanças) mais `v` (estendido com sinal, como o cdq/adc da original).
pub(crate) unsafe fn cpv5_somar_saldo(fin: usize, v: i32) {
    let s = core::ptr::read_unaligned(fin as *const i64);
    core::ptr::write_unaligned(fin as *mut i64, s.wrapping_add(v as i64));
}


/// O registro de finanças (0x167 bytes) do clube na tabela `this` (FinanceTable), pelo id (+0) do clube.
pub(crate) unsafe fn cpv5_financas(this: usize, clube: usize) -> usize {
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
pub(crate) unsafe extern "thiscall" fn receita_do_merchandising(this: usize, clube: usize) {
    let outro = read_usize(clube + club::CHAIRMAN);
    let est = if outro != 0 { read_usize(outro + 0x69) } else { 0 };
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let fator = if est != 0 {
        let s = (read_u8(est + 0xf) as i8 as i32).wrapping_mul(25);
        let mut v = rep.wrapping_add(s);
        if read_u8(clube + club::OWN_STADIUM) == 0 {
            v = ftol(v as f64 * f64_em(0x96a5d0));
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
    let base = ftol(read_i32(clube + club::ATTENDANCE) as f64 * rep as f64 * f64_em(fator));
    crate::comparacao::ramo("cpv_5 merchandising");
    let r = cpe1_sorteio(0xb); // cpv5: o sorteio do merchandising
    let v = ftol((r + 0x5f).wrapping_mul(base) as f64 * f64_em(0x967898));
    let fin = cpv5_financas(this, clube);
    cpv5_somar(fin + 0x15c, v);
    cpv5_somar(fin + 0xbc, v);
    cpv5_somar_saldo(fin, v);
}

/// Comparador: lado a lado no registro de finanças do clube e no estado do sorteio.
pub(crate) unsafe extern "thiscall" fn comparar_receita_do_merchandising(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RECEITA_DO_MERCHANDISING);
    let mut regioes = vec![(cpv5_financas(this, clube), 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || crate::aleatorio::sem_contar(|| receita_do_merchandising(this, clube)));
    PLACAR.registrar(igual, || format!("receita_do_merchandising(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
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
pub(crate) unsafe extern "thiscall" fn ingressos_da_temporada(this: usize, aplicar: i32) {
    let id = read_usize(this + 8);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(0x245));
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_INGRESSOS_DA_TEMPORADA)(this, aplicar);
    }
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let est = read_usize(clube + club::STADIUM);
    if est != 0 && read_i32(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, ftol(rep as f64 * f64_em(0x968e18)));
        let est = read_usize(clube + club::STADIUM);
        if read_i32(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    let gerente = read_usize(read_usize(G_MANAGER_MANAGER));
    let valor = read_i16(gerente.wrapping_add((read_usize(clube) as u32).wrapping_mul(73) as usize) + 8);
    let mut f = valor as f64 * f64_em(0x96a5c0);
    if !(f > f64_em(0x968f20)) {
        f = f64_em(0x968f20);
    }
    let t = ftol(read_i32(clube + club::ATTENDANCE) as f64 * f);
    let mut g = rep as f64 * f64_em(0x96a680);
    if !(g > f64_em(0x968968)) {
        g = f64_em(0x968968);
    }
    let u = ftol(t as f64 * g);
    let est = read_usize(clube + club::STADIUM);
    let teto = |e: usize| if e != 0 { read_i32(e + 0x3c) as f64 * f64_em(0x96a678) } else { f64_em(0x96a5e8) };
    let n = ftol(if (u as f64) < teto(est) { u as f64 } else { teto(est) });
    if n == 0 {
        return;
    }
    let ja = read_i32(this + 0x18);
    if ja >= n {
        return;
    }
    let mut passo = (read_u8(this + 0x164) as i8).wrapping_add(1);
    if passo < 0 || passo > 8 {
        passo = 8;
    }
    *((this + 0x164) as *mut i8) = passo;
    let b = ftol(n.wrapping_sub(ja) as f64 * f32_em(0x9c8758 + passo as usize * 4) as f64);
    crate::comparacao::ramo("cpv_5 ingressos vendidos");
    let r = cpe1_sorteio(0xb);
    let novos = (r + 0x5f).wrapping_mul(b) / 100;
    let total = ja.wrapping_add(novos);
    core::ptr::write_unaligned((this + 0x18) as *mut i32, total); // cpv5: os ingressos vendidos
    let preco = ftol((rep as f64 * f64_em(0x969048) + f64_em(0x968938)) * f64_em(0x968ea8));
    let mut receita = preco.wrapping_mul(novos);
    let est = read_usize(clube + club::STADIUM);
    let lugares = if est != 0 { read_i32(est + 0x3c) } else { 0 };
    let den = if lugares != 0 { lugares as f64 * f64_em(0x96a678) } else { f64_em(0x96a5e8) };
    let razao = total as f64 / den;
    core::ptr::write_unaligned((this + 0x160) as *mut f32, razao as f32);
    if razao > f64_em(0x96a678) {
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
        receita = ftol(receita as f64 * f64_em(fator));
    }
    if aplicar != 0 { // cpv5: a receita dos ingressos
        cpv5_somar(this + 0xac, receita);
        cpv5_somar(this + 0x14c, receita);
        cpv5_somar_saldo(this, receita);
    }
    let r1 = cpe1_sorteio(rep);
    let r2 = cpe1_sorteio(rep); // cpv5: o segundo sorteio da loja
    let k = r1.wrapping_add(r2).wrapping_add(1);
    let v = ftol((k as f64 * f64_em(0x96a350) + f64_em(0x967890)) * novos as f64);
    cpv5_somar(this + 0x15c, v);
    cpv5_somar(this + 0xbc, v);
    cpv5_somar_saldo(this, v);
}

/// Comparador: lado a lado no registro de finanças (this), nos lugares do estádio (+0x3c) e no estado do sorteio
/// (clube nulo: só a original).
pub(crate) unsafe extern "thiscall" fn comparar_ingressos_da_temporada(this: usize, aplicar: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_INGRESSOS_DA_TEMPORADA);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(0x245));
    if clube == 0 {
        return o(this, aplicar);
    }
    let mut regioes = vec![(this, 0x167)];
    let est = read_usize(clube + club::STADIUM);
    if est != 0 {
        regioes.push((est + 0x3c, 4));
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, aplicar), || crate::aleatorio::sem_contar(|| ingressos_da_temporada(this, aplicar)));
    PLACAR.registrar(igual, || format!("ingressos_da_temporada(fin {:#x}, {}) original={:x?} rust={:x?}", this, aplicar, mo, mr));
}

/// O custo da obra (+0x8c e +0x12c, menos no saldo) no registro de finanças do clube.
pub(crate) unsafe fn cpv5_cobrar_obra(this: usize, clube: usize, custo: i32) {
    let fin = cpv5_financas(this, clube);
    cpv5_somar(fin + 0x12c, custo);
    cpv5_somar(fin + 0x8c, custo);
    cpv5_somar_saldo(fin, custo.wrapping_neg());
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
pub(crate) unsafe extern "thiscall" fn melhorar_o_centro_de_treino(this: usize, clube: usize) {
    let nivel = (clube + club::TRAINING) as *mut i8;
    let custo = (*nivel as i32).wrapping_mul(50000).wrapping_add(75000);
    *nivel = if *nivel < 0x13 { (*nivel).wrapping_add(2) } else { (*nivel).wrapping_add(1) };
    cpv5_cobrar_obra(this, clube, custo);
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
        cpv5_cobrar_obra(this, clube, c);
    }
    if *nivel > 0x14 {
        *nivel = 0x14;
    }
}

/// Comparador: lado a lado no nível do centro de treino (+0x7f) e no registro de finanças do clube.
pub(crate) unsafe extern "thiscall" fn comparar_melhorar_o_centro_de_treino(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MELHORAR_O_CENTRO_DE_TREINO);
    let regioes = [(clube + club::TRAINING, 1), (cpv5_financas(this, clube), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || melhorar_o_centro_de_treino(this, clube));
    PLACAR.registrar(igual, || format!("melhorar_o_centro_de_treino(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

// ---------- 258a leva: os eventos financeiros da semana: o passe dos eventos do clube, a diretoria que paga a dívida e a injeção de dinheiro (finance.cpp) ----------






pub(crate) unsafe fn cpv6_saldo(fin: usize) -> i64 {
    core::ptr::read_unaligned(fin as *const i64)
}

/// O saldo (i64 em +0) mais `v` estendido com sinal (o cdq/adc da original).
pub(crate) unsafe fn cpv6_somar_saldo(fin: usize, v: i32) {
    core::ptr::write_unaligned(fin as *mut i64, cpv6_saldo(fin).wrapping_add(v as i64));
}


pub(crate) unsafe fn cpv6_faixa(fin: usize) -> i8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    f(fin) as i8
}

/// Um evento da semana (0x59e050, 0x59dd40, 0x59e5a0, 0x59e940): thiscall(tabela, clube, finanças), 1 se aconteceu.
pub(crate) unsafe fn cpv6_evento(rotina: usize, this: usize, clube: usize, fin: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(rotina);
    f(this, clube, fin)
}

/// O ensaio do comparador: a cópia roda sem a notícia (0x59f300) e sem a verba (0x598800), que alocam e não podem
/// rodar duas vezes.
pub(crate) static mut CPV6_ENSAIO: bool = false;

/// A notícia do evento (0x59f300: thiscall(tabela, clube, valor, verba, tipo), ret 0x10); no ensaio, nada.
pub(crate) unsafe fn cpv6_noticia(this: usize, clube: usize, valor: i32, verba: u32, tipo: u32) {
    if CPV6_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(FN_FINANCE_POST_FINANCE_BOOST_NEWS);
    f(this, clube, valor, verba, tipo);
}

/// A verba de transferências (Finance::get_transfer_budget 0x598800 com 0, 0, 0, 1, 0) que vai na notícia; no
/// ensaio, 0 (a notícia não sai).
pub(crate) unsafe fn cpv6_verba(fin: usize) -> u32 {
    if CPV6_ENSAIO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    f(fin, 0, 0, 0, 1, 0)
}

/// O fim da semana do clube no estado de crise (0x5a02f0: thiscall(tabela, id)).
pub(crate) unsafe fn cpv6_fim_da_crise(this: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FINANCE_POST_RECEIVERS_APPEASED_NEWS);
    f(this, read_usize(clube) as u32);
}

/// O estado (+0x165) volta a 0; se era 1 (crise), a notícia do fim da crise (0x5a02f0) antes.
pub(crate) unsafe fn cpv6_sair_da_crise(this: usize, clube: usize, fin: usize) {
    let s = read_u8(fin + 0x165);
    if s == 1 {
        cpv6_fim_da_crise(this, clube);
    }
    if s == 1 || s == 2 {
        *((fin + 0x165) as *mut u8) = 0;
    }
}

/// O centro de treino no máximo (20) cai um nível com sorteio(10) zero, se a reputação for menor que 7250.
pub(crate) unsafe fn cpv6_desgaste_do_centro(clube: usize) {
    let nivel = (clube + club::TRAINING) as *mut i8;
    if *nivel == 0x14 && read_i16(clube + club::REPUTATION) < 0x1c52 && cpe1_sorteio(10) == 0 {
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
pub(crate) unsafe extern "thiscall" fn eventos_financeiros_da_semana(this: usize, clube: usize, semana: u32) {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_EVENTOS_FINANCEIROS_DA_SEMANA)(this, clube, semana);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a: u32 = 0;
    let p = parceiro(clube, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { clube };
    let fin = read_usize(this).wrapping_add((read_usize(c) as u32).wrapping_mul(0x167) as usize);
    let aperto = (fin + 0x166) as *mut i8;
    match cpv6_faixa(fin) {
        -1 => {
            let s = read_u8(fin + 0x165);
            if s != 2 {
                if s == 1 {
                    cpv6_fim_da_crise(this, clube);
                }
                *((fin + 0x165) as *mut u8) = 2;
            }
            if semana == 0 {
                return;
            }
            if read_u8(clube + club::OWN_STADIUM) != 0 {
                *aperto = (*aperto).wrapping_add(1);
            }
            cpv6_desgaste_do_centro(clube);
            if cpv6_evento(0x59dd40, this, clube, fin) == 0 && cpv6_evento(0x59e5a0, this, clube, fin) == 0 {
                cpv6_evento(0x59e940, this, clube, fin);
            }
        }
        -2 => {
            if read_u8(fin + 0x165) != 1 {
                let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINANCE_POST_FINANCIAL_CRISIS_NEWS);
                let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
                let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
                let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
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
                    let k = contrato_de(g, j);
                    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = k;
                    let dono = if k != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(0x245)) } else { 0 };
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
            if cpv6_evento(0x59e940, this, clube, fin) == 0 && cpv6_evento(0x59e5a0, this, clube, fin) == 0 {
                cpv6_evento(0x59e050, this, clube, fin);
            }
            cpv6_desgaste_do_centro(clube);
            if read_u8(clube + club::OWN_STADIUM) != 0 {
                *aperto = (*aperto).wrapping_add(3);
            }
        }
        1 => {
            cpv6_sair_da_crise(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 0 {
                *aperto -= 1;
            }
            if cpv6_evento(0x59e050, this, clube, fin) == 0 && cpv6_evento(0x59dd40, this, clube, fin) == 0 {
                cpv6_evento(0x59e5a0, this, clube, fin);
            }
        }
        2 => {
            cpv6_sair_da_crise(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 5 {
                *aperto -= 1;
            }
        }
        _ => {
            cpv6_sair_da_crise(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 10 {
                *aperto -= 1;
            }
            crate::comparacao::ramo("cpv_6 impulso na faixa 0");
            let ok = cpv6_evento(0x59e050, this, clube, fin) != 0; // cpv6: o impulso na faixa 0
            if !ok && cpv6_evento(0x59dd40, this, clube, fin) == 0 {
                cpv6_evento(0x59e5a0, this, clube, fin);
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
            let r = cpe1_sorteio(4) as u8;
            *nivel = (*nivel).wrapping_add(0xffu8.wrapping_sub(r) as i8);
            return;
        }
        if n <= 1 || cpe1_sorteio(n as i32) == 0 {
            return;
        }
        *nivel = (*nivel).wrapping_sub(1);
        return;
    }
    let lugares = read_i32(est + 0x3c);
    if lugares <= 0xabe || cpe1_sorteio(read_i16(clube + club::REPUTATION) as i32) >= lugares {
        return;
    }
    let t = ftol(lugares as f64 * f64_em(0x967880)) as i16 as i32;
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
    cpv5_somar(est + 0x3c, fechar.wrapping_neg());
    let est = read_usize(clube + club::STADIUM);
    cpv5_somar(est + 0x44, fechar);
    *aperto = 0x14;
    let n = *nivel;
    if n > 0xf {
        let r = cpe1_sorteio(5) as u8;
        *nivel = (*nivel).wrapping_add(0xffu8.wrapping_sub(r) as i8);
        return;
    }
    if n <= 7 && (n <= 1 || cpe1_sorteio(n as i32) == 0) {
        return;
    }
    *nivel = (*nivel).wrapping_sub(1);
}

// ponytail: chama originais que sorteiam, alocam notícias e mexem nos contratos: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_eventos_financeiros_da_semana(this: usize, clube: usize, semana: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_EVENTOS_FINANCEIROS_DA_SEMANA)(this, clube, semana)
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
pub(crate) unsafe extern "thiscall" fn diretoria_paga_a_divida(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let div = match cpv6_faixa(fin) {
        -2 => 2.0f64,
        -1 => {
            if rep >= 0xdac {
                0.4
            } else {
                2.0
            }
        }
        0 => {
            if rep > 0xdac || cpv6_saldo(fin) >= rep.wrapping_mul(25) as i64 {
                return 0;
            }
            0.4
        }
        _ => return 0,
    };
    crate::comparacao::ramo("cpv_6 sorteio da divida");
    let r = cpe1_sorteio(7000); // cpv6: o sorteio da dívida
    if rep >= r.wrapping_add(0xdac) {
        return 0;
    }
    let n = ftol(f64_em(0x969000) / div);
    let outro = read_usize(clube + club::CHAIRMAN);
    if outro != 0 && read_usize(outro + 0x69) != 0 {
        if cpe1_sorteio(n) != 0 {
            return 0;
        }
        let est = read_usize(read_usize(clube + club::CHAIRMAN) + 0x69);
        if cpe1_sorteio(read_u8(est + 0x20) as i8 as i32) == 0 {
            return 0;
        }
    } else {
        if cpe1_sorteio(n) != 0 {
            return 0;
        }
        if cpe1_sorteio(10) == 0 {
            return 0;
        }
    }
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let saldo = cpv6_saldo(fin);
    let valor = if rep < 0x4e2 && saldo < rep.wrapping_mul(-450) as i64 {
        rep.wrapping_mul(450)
    } else if saldo >= rep.wrapping_mul(-300) as i64 {
        rep.wrapping_mul(150)
    } else {
        rep.wrapping_mul(300)
    };
    cpv5_somar(fin + 0x154, valor);
    cpv5_somar(fin + 0xb4, valor);
    cpv6_somar_saldo(fin, valor);
    cpv6_noticia(this, clube, valor, 0, 4);
    1
}

/// O comparador dos eventos: a cópia em ensaio (sem notícia nem verba) e a original a partir do mesmo estado (registro de
/// finanças e sorteio). Sem evento: tudo igual, inclusive o sorteio; com evento: a devolução e os campos que o evento
/// grava (saldo, +0xb4, +0x154, +0x165), porque a notícia e a verba da original podem sortear e gravar.
pub(crate) unsafe fn cpv6_comparar(nome: &str, fin: usize, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPV6_ENSAIO = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(copia));
    CPV6_ENSAIO = false;
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

pub(crate) unsafe extern "thiscall" fn comparar_diretoria_paga_a_divida(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DIRETORIA_PAGA_A_DIVIDA);
    cpv6_comparar("diretoria_paga_a_divida", fin, || o(this, clube, fin), || diretoria_paga_a_divida(this, clube, fin))
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
pub(crate) unsafe extern "thiscall" fn injecao_de_dinheiro(this: usize, clube: usize, fin: usize) -> u32 {
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
    let div = match cpv6_faixa(fin) {
        -2 => f64_em(0x9689e0),
        -1 => f64_em(0x967880),
        _ => f64_em(0x96a350),
    };
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let outro = read_usize(clube + club::CHAIRMAN);
    let mut valor;
    if outro != 0 && read_usize(outro + 0x69) != 0 && read_u8(clube + club::PLC) == 0 {
        let r = cpe1_sorteio(ftol(f64_em(0x968dd8) / div));
        let est = read_usize(read_usize(clube + club::CHAIRMAN) + 0x69);
        if (read_u8(est + 0xf) as i8 as i32) <= r {
            return 0;
        }
        let e = read_u8(read_usize(read_usize(clube + club::CHAIRMAN) + 0x69) + 0x20) as i8 as i32;
        if e <= cpe1_sorteio(5).wrapping_add(10) {
            return 0;
        }
        valor = e.wrapping_mul(20).wrapping_add(0x96).wrapping_mul(rep).wrapping_add(0xb71b0);
    } else {
        if cpe1_sorteio(ftol(f64_em(0x968938) / div)) != 0 {
            return 0;
        }
        if read_u8(clube + club::PLC) != 0 {
            return 0;
        }
        valor = rep.wrapping_mul(5).wrapping_add(0x3a98).wrapping_mul(50);
    }
    let saldo = cpv6_saldo(fin);
    if (valor.wrapping_mul(10) as i64) < saldo.wrapping_neg() {
        valor = ftol(-(saldo as f64 * f64_em(0x967880)));
    }
    if read_i16(clube + club::REPUTATION) < 0x7d0 {
        let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
        if valor > teto {
            valor = teto;
        }
    } else {
        let maior = (valor as f64 > rep.wrapping_mul(rep) as f64 * f64_em(0x9689e0)) as i32;
        if valor > 0x3d0900 + maior {
            valor = 0x3d0900 + maior;
        }
    }
    cpv5_somar(fin + 0x154, valor);
    cpv5_somar(fin + 0xb4, valor);
    cpv6_somar_saldo(fin, valor);
    let verba = cpv6_verba(fin);
    cpv6_noticia(this, clube, valor, verba, 3);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_injecao_de_dinheiro(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_INJECAO_DE_DINHEIRO);
    cpv6_comparar("injecao_de_dinheiro", fin, || o(this, clube, fin), || injecao_de_dinheiro(this, clube, fin))
}

// ---------- 258a leva: os salários do pessoal e os bônus: o teto salarial do pessoal, o pagamento dos bônus e o ajuste do teto salarial (finance.cpp) ----------





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
pub(crate) unsafe extern "thiscall" fn teto_salarial_do_pessoal(this: usize, cargo: i8) -> i32 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let id = read_usize(this + 8);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(0x245));
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
        w = ftol(padrao as f64 * f64_em(0x9689e0));
    } else {
        let tipo = (read_u8(nac + 0x7e) as i8).clamp(1, 3);
        let k = read_u8(nac + 0x85) as i8 as i32;
        w = ftol(k as f64 / (tipo as i32 * 20) as f64 * padrao as f64);
        if tipo == 1 {
            let n = read_i32(nac);
            let e = |a: usize| n == read_i32(a);
            if e(KEY_NATION_ENGLAND) || e(KEY_NATION_ITALY) || e(KEY_NATION_SPAIN) {
                if read_i16(clube + club::REPUTATION) > 0x1c52 {
                    crate::comparacao::ramo("cpv_7 clube grande");
                    w = ftol(w as f64 * f64_em(0x9689b8)); // cpv7: o clube grande da Inglaterra, Itália e Espanha
                }
            } else if e(KEY_NATION_JAPAN) {
            } else if e(KEY_NATION_UNITED_STATES) {
                if w > 0x6d6 {
                    w = ftol(w.wrapping_sub(0x6d6) as f64 * f64_em(0x969040) + f64_em(0x96a5f8));
                }
            } else {
                let (base, limiar, soma) = if e(KEY_NATION_GERMANY) || e(KEY_NATION_FRANCE) || e(KEY_NATION_SCOTLAND) || e(KEY_NATION_HOLLAND) {
                    (0x969040, 0x9c4, 0x9690b0)
                } else if e(KEY_NATION_SWEDEN) || e(KEY_NATION_DENMARK) || e(KEY_NATION_BELGIUM) || e(KEY_NATION_PORTUGAL) {
                    (0x969040, 0x4e2, 0x96a5e8)
                } else {
                    (0x968e80, 0x2ee, 0x96a5f0)
                };
                let mut f = f64_em(base);
                if r50 > k.wrapping_mul(10) {
                    let d = if k >= 1 { k } else { 1 };
                    f = r50 as f64 / d as f64 * f64_em(0x967880) * f64_em(base);
                }
                if w > limiar {
                    if f > f64_em(0x967890) {
                        f = f64_em(0x967890);
                    }
                    w = ftol(w.wrapping_sub(limiar) as f64 * f + f64_em(soma));
                }
            }
        }
        let liga = read_usize(clube + club::DIVISION);
        if liga != 0 {
            crate::comparacao::ramo("cpv_7 peso da liga");
            w = ftol(read_i16(liga + 0x69) as f64 * w as f64 * f64_em(0x968918)); // cpv7: o peso da liga
        }
    }
    match faixa(this) as i8 {
        -2 => w = ftol(w as f64 * f64_em(0x969040)),
        -1 => w = ftol(w as f64 * f64_em(0x9689d0)),
        1 => w = ftol(w as f64 * f64_em(0x968940)),
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

/// Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_teto_salarial_do_pessoal(this: usize, cargo: i8) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_TETO_SALARIAL_DO_PESSOAL)(this, cargo);
    let n = teto_salarial_do_pessoal(this, cargo);
    PLACAR.registrar(r == n, || format!("teto_salarial_do_pessoal(fin {:#x}, cargo {}) original={} rust={}", this, cargo, r, n));
    r
}

/// adds the amount to the bonus expenses (+0x138 and +0x98) and takes it from the balance
/// Original: `Finance::pay_bonuses` (0x599960, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x599960 } (thiscall, ret 4, finance.cpp): `Finance::pay_bonuses(valor)`, pelo fim do jogo dos contratos
/// (CONTRACT_MANAGER::update_after_match 0x4ca6d0, três chamadas): soma o valor nas despesas de bônus (+0x138 e +0x98)
/// e o tira do saldo (i64 em +0, estendido com sinal).
pub(crate) unsafe extern "thiscall" fn pagar_os_bonus(this: usize, valor: i32) {
    for o in [0x138usize, 0x98] {
        core::ptr::write_unaligned((this + o) as *mut i32, read_i32(this + o).wrapping_add(valor));
    }
    crate::comparacao::ramo("cpv_7 bonus pagos");
    let s = core::ptr::read_unaligned(this as *const i64);
    core::ptr::write_unaligned(this as *mut i64, s.wrapping_sub(valor as i64)); // cpv7: o bônus sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_pagar_os_bonus(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAGAR_OS_BONUS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_os_bonus(this, valor));
    PLACAR.registrar(igual, || format!("pagar_os_bonus(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// wage ceiling (+0x14) × (percent + 100)%, truncated
/// Original: `Finance::adjust_max_wage` (0x598340, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x598340 } (thiscall, ret 4, finance.cpp): `Finance::adjust_max_wage(percentual)`, pela resposta da diretoria
/// (MANAGER_MANAGER::handle_board_request 0x6a91b0) e pela 0x6acee8: o teto salarial (+0x14) vira (percentual + 100) ×
/// 0,01 × teto, truncado.
pub(crate) unsafe extern "thiscall" fn ajustar_o_teto_salarial(this: usize, percentual: i8) {
    let v = ftol((percentual as i32 + 100) as f64 * f64_em(0x967898) * read_i32(this + 0x14) as f64);
    core::ptr::write_unaligned((this + 0x14) as *mut i32, v);
}

/// Comparador: lado a lado no teto salarial (+0x14).
pub(crate) unsafe extern "thiscall" fn comparar_ajustar_o_teto_salarial(this: usize, percentual: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8)>(IDX_AJUSTAR_O_TETO_SALARIAL);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x14, 4)], || o(this, percentual), || ajustar_o_teto_salarial(this, percentual));
    PLACAR.registrar(igual, || format!("ajustar_o_teto_salarial(fin {:#x}, {}) original={:x?} rust={:x?}", this, percentual, mo, mr));
}

// ---------- 257a leva: a riqueza do clube e duas ordens: a faixa das finanças, a ordem dos jogadores pela habilidade e a dos estádios pela capacidade (finance.cpp, game.cpp, stadium.cpp) ----------

/// national team, 0; by the 64-bit balance against the club reputation R: above 2500·R + 5,000,000, 2; above 1000·R +
/// 3,500,000, 1; below −2,500,000 − 1500·R, −2; below −250·R (R < 5000) or (3750 − R)·1000, −1; otherwise 0
/// Original: `Finance::get_wealth_band` (0x5986b0, finance.cpp, thiscall).
///
/// [pt] { 0x5986b0 } (thiscall, finance.cpp): `Finance::get_wealth_band()`, a faixa do saldo (64 bits em +0/+4) do clube (+8,
/// vetor 0xae23b4) pela reputação R (+0x80), usada pelas transferências, pela diretoria, pelos técnicos, pelas finanças e
/// pelo adiamento. Seleção (0x53b900): 0. Acima de 2500·R + 5.000.000: 2; acima de 1000·R + 3.500.000: 1; abaixo de
/// −2.500.000 − 1500·R: −2; abaixo de −250·R (R < 5000) ou de (3750 − R)·1000 (R ≥ 5000): −1; senão 0.
pub(crate) unsafe extern "thiscall" fn faixa_de_riqueza(this: usize) -> u8 {
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

/// Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_faixa_de_riqueza(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_FAIXA_DE_RIQUEZA)(this);
    let n = faixa_de_riqueza(this);
    PLACAR.registrar(r == n, || format!("faixa_de_riqueza(financas {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}





