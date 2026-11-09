// ---------- %%LEVA%%a leva: o fim de temporada das finanças: o fim de temporada da nação, as obras, o centro de treino e o dividendo de cada clube e a notícia do dividendo (fin_prm.cpp) ----------

unsafe fn cqc1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc1_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cqc1_i(a).wrapping_add(v));
}

unsafe fn cqc1_saldo(fin: usize) -> i64 {
    core::ptr::read_unaligned(fin as *const i64)
}

/// Soma `v` nos dois acumuladores do ano e do mês (`ano`, `mes`) e o tira do saldo (i64; o cdq/sub/sbb da original).
unsafe fn charge_expense(fin: usize, mes: usize, ano: usize, v: i32) {
    cqc1_somar(fin + mes, v);
    cqc1_somar(fin + ano, v);
    core::ptr::write_unaligned(fin as *mut i64, cqc1_saldo(fin).wrapping_sub(v as i64));
}

unsafe fn cqc1_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
unsafe fn fin_prm_is_human_club(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0)
}

/// O registro de finanças do clube (0x167 bytes) pelo parceiro: 0x540a50 (3 pushes; add esp, 0xc -> cdecl(clube, &a,
/// 1)); o parceiro quando ele existe e o clube é o B (a = 0). `a` é a local da original (a 0x540a50 a grava).
unsafe fn finance_of_parent_club(this: usize, clube: usize, a: &mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, a, 1);
    let c = if p != 0 && *a == 0 { p } else { clube };
    cqc1_d(this).wrapping_add((cqc1_i(c) as u32).wrapping_mul(0x167) as usize)
}

/// A notícia das obras e do treino (0x59ffb0: ecx = tabela; 3 pushes (tipo, valor, clube); sem add esp -> thiscall,
/// ret 0xc).
unsafe fn post_works_news(this: usize, clube: usize, valor: i32, tipo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(0x59ffb0usize);
    f(this, clube, valor, tipo);
}

/// A notícia da obra recusada (0x59fae0: ecx = tabela; 3 pushes; sem add esp -> thiscall, ret 0xc; sorteia 2 fora do
/// motivo 2).
unsafe fn post_works_refused_news(this: usize, clube: usize, assunto: i32, motivo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(0x59fae0usize);
    f(this, clube, assunto, motivo);
}

/// A linha do clube na tabela dos técnicos ([[0xb67a44]] + id × 0x49); o byte +2 tem os pedidos à diretoria (bit 6:
/// estádio; bit 7: centro de treino).
unsafe fn board_requests_row(clube: usize) -> usize {
    cqc1_d(cqc1_d(0xb67a44)).wrapping_add((cqc1_i(clube) as u32).wrapping_mul(0x49) as usize)
}

/// O técnico do clube (+0xcf) é humano (entre as 16 últimas pessoas): a notícia vai para ele (0x7861e0).
unsafe fn fin_prm_has_human_manager(clube: usize) -> bool {
    let t = cqc1_d(clube + 0xcf);
    t != 0 && cqc1_i(t) >= cqc1_i(0xae2364).wrapping_sub(0x10)
}

/// { 0x59cdd0 } (thiscall, ret 4, fin_prm.cpp; os asserts citam finance.CPP): `FinanceTable::end_season_for_nation(entrada)`,
/// pelo laço das temporadas (0x5d5ae0, no dia anterior ao início da temporada da nação-chave `entrada` da tabela
/// 0xb63d60, 72 bytes cada). Nada antes da data de início do jogo ([0xdd6ec8] depois de hoje, 0x456770: ecx = 0xae2c90;
/// 2 pushes; sem add esp -> thiscall, ret 8). Sem entrada: a caixa (a original). Para cada clube da tabela (0x245
/// bytes; para no primeiro que for seleção, 0x53b900: push; add esp, 4 -> cdecl) com liga (+0x57) e nação (+0x53) ativa
/// (+0x11c) ou reputação (+0x80) de 5000, cuja linha (0x669360: push; add esp, 4 -> cdecl) é a entrada: o fim de
/// temporada do clube (0x59cfa0: ecx = tabela; push clube -> thiscall, ret 4) e os totais do ano (+0x110, 0x50 bytes)
/// passam para os do ano anterior (+0xc0) e zeram, no registro de índice igual ao do clube.
unsafe extern "thiscall" fn fin_prm_end_season_for_nation(this: usize, entrada: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if depois(0xae2c90, cqc1_d(0xdd6ec8) as u32, cqc1_d(0xdd6ecc) as u32) != 0 {
        return;
    }
    if entrada == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, entrada); // a caixa
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    let do_clube: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59cfa0usize);
    let mut clube = cqc1_d(0xae23b4);
    let mut k = 0i32;
    let mut desloc = 0usize;
    while k < cqc1_i(0xae235c) {
        if selecao(clube) != 0 {
            return;
        }
        let nacao = cqc1_d(clube + 0x53);
        if cqc1_d(clube + 0x57) != 0 && nacao != 0 {
            let ativa = cqc1_b(nacao + 0x11c) != 0;
            if ativa || cqc1_w(clube + 0x80) >= 0x1388 { // cqc1: o clube de nação inativa pela reputação
                let l = linha(clube) as i8 as i32;
                if (0xb63d60 + l * 72) as usize == entrada {
                    if !ativa && cqc1_w(clube + 0x80) < 0x1770 {
                        crate::comparison::branch("cqc_1 inactive nation below 6000");
                    }
                    do_clube(this, clube);
                    let f = cqc1_d(this).wrapping_add(desloc);
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
unsafe extern "thiscall" fn compare_fin_prm_end_season_for_nation(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if entrada == 0 || depois(0xae2c90, cqc1_d(0xdd6ec8) as u32, cqc1_d(0xdd6ecc) as u32) != 0 {
        return o(this, entrada);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    let mut ultimo = None;
    let mut clube = cqc1_d(0xae23b4);
    for k in 0..cqc1_i(0xae235c).max(0) as usize {
        if selecao(clube) != 0 {
            break;
        }
        let nacao = cqc1_d(clube + 0x53);
        if cqc1_d(clube + 0x57) != 0 && nacao != 0 && (cqc1_b(nacao + 0x11c) != 0 || cqc1_w(clube + 0x80) >= 0x1388) && (0xb63d60 + (linha(clube) as i8 as i32) * 72) as usize == entrada {
            ultimo = Some(k);
        }
        clube += 0x245;
    }
    o(this, entrada);
    if let Some(k) = ultimo {
        let f = cqc1_d(this) + k * 0x167 + 0x110;
        let zerado = core::slice::from_raw_parts(f as *const u8, 0x50).iter().all(|&x| x == 0);
        SCOREBOARD.record(zerado, || format!("fin_prm_end_season_for_nation(entrada {:#x}) clube {} sem zerar os totais do ano", entrada, k));
    }
}

/// { 0x59cfa0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::season_end_club_update(clube)`, pelo fim de temporada da
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
unsafe extern "thiscall" fn season_end_club_update(this: usize, clube: usize) {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, clube); // a caixa
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return;
    }
    let mut local = 0u32; // o [esp + 0x20] da original (a 0x540a50 do dono e a média de público)
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, clube, &mut a);
    if fin == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, clube); // a caixa
    }
    let mut saldo = cqc1_i(fin);
    let mut aviso = false;
    let estadio = cqc1_d(clube + 0x69);
    'obras: {
        if estadio == 0 {
            break 'obras;
        }
        if cqc1_b(clube + 0x6d) == 0 && cqc1_d(clube + 0x57) != 0 {
            if saldo <= 0x2160ec0 {
                break 'obras;
            }
            let mut dono = 0usize;
            let mut outro = cqc1_d(0xae23b4);
            let mut k = 0i32;
            while k < cqc1_i(0xae235c) {
                if outro != clube && cqc1_d(outro + 0x69) == estadio && cqc1_b(outro + 0x6d) != 0 {
                    dono = outro;
                    break;
                }
                k += 1;
                outro += 0x245;
            }
            let f2 = if dono != 0 { finance_of_parent_club(this, dono, &mut local) } else { 0 };
            if dono == 0 || f2 == 0 {
                *((clube + 0x6d) as *mut u8) = 1;
                charge_expense(fin, 0x8c, 0x12c, 20_000_000);
                post_works_news(this, clube, cqc1_i(cqc1_d(clube + 0x69)), 4);
            } else if cqc1_b(f2 + 0x165) == 1 || cqc1_b(f2 + 0x165) == 2 {
                *((clube + 0x6d) as *mut u8) = 1;
                *((dono + 0x6d) as *mut u8) = 0;
                charge_expense(fin, 0x8c, 0x12c, 20_000_000);
                cqc1_somar(f2 + 0xb4, 20_000_000);
                cqc1_somar(f2 + 0x154, 20_000_000);
                core::ptr::write_unaligned(f2 as *mut i64, cqc1_saldo(f2).wrapping_add(20_000_000));
                post_works_news(this, clube, cqc1_i(cqc1_d(clube + 0x69)), 4);
                post_works_news(this, dono, cqc1_i(cqc1_d(clube + 0x69)), 5);
            }
            break 'obras;
        }
        if cqc1_b(clube + 0x6d) == 0 || cqc1_d(clube + 0x57) == 0 {
            break 'obras;
        }
        let aperto = cqc1_b(fin + 0x166) as i8;
        if saldo > 0xb71b0 && aperto > 10 {
            let v = 10i32.wrapping_sub((aperto as i32).wrapping_mul(10000));
            cqc1_somar(fin + 0x12c, v);
            saldo = saldo.wrapping_sub(v);
            cqc1_somar(fin + 0x8c, v);
            core::ptr::write_unaligned(fin as *mut i64, cqc1_saldo(fin).wrapping_sub(v as i64));
        }
        upgrade_ai_training_ground(this, clube, saldo);
        let pedidos = board_requests_row(clube) + 2;
        if cqc1_b(pedidos) & 0x40 != 0 && fin_prm_is_human_club(clube) != 0 {
            aviso = true;
            *(pedidos as *mut u8) &= 0xbf;
        }
        let est = cqc1_d(clube + 0x69);
        if est != 0 {
            if saldo > 0x10b0760 && cqc1_w(clube + 0x80) > 0x1d4c && cqc1_i(est + 0x3c) > 0x61a8 && cqc1_b(est + 0x4d) == 0 {
                *((est + 0x4d) as *mut u8) = 1;
                charge_expense(fin, 0x8c, 0x12c, 1_500_000);
                post_works_news(this, clube, cqc1_i(cqc1_d(clube + 0x69)), 6);
                break 'obras;
            }
            if saldo > 0x47868c0 && cqc1_w(clube + 0x80) > 0x2422 && cqc1_i(est + 0x3c) > 0xe09c && cqc1_b(clube + 0x7f) == 0x14 && cqc1_b(est + 0x4c) == 0 {
                *((est + 0x4c) as *mut u8) = 1;
                charge_expense(fin, 0x8c, 0x12c, 7_500_000);
                break 'obras;
            }
        }
        let pode = saldo > 0x501bd0 || (fin_prm_is_human_club(clube) != 0 && saldo.wrapping_add((cqc1_w(clube + 0x80) as i32).wrapping_mul(1000)) > 0x501bd0);
        if !pode {
            if aviso {
                post_works_refused_news(this, clube, 2, -1);
            }
            break 'obras;
        }
        let liga = cqc1_d(clube + 0x57);
        if liga != 0 {
            let comp = cqc1_d(cqc1_d(0xadadfc).wrapping_add(cqc1_d(liga).wrapping_mul(4)));
            if comp != 0 && cqc1_d(comp + 8) != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                let l = busca(cqc1_d(comp + 8), clube);
                if l != 0 && cqc1_b(l + 5) != 0 {
                    local = cqc1_i(l + 6).wrapping_div(cqc1_b(l + 5) as i32) as u32;
                }
            }
        }
        let est = cqc1_d(clube + 0x69);
        let cap = cqc1_i(est + 0x3c);
        if cap > 0x1e848 {
            break 'obras;
        }
        let rep = cqc1_w(clube + 0x80);
        let expandir = aviso
            || (cap as f64 * read_f64(0x96a720)) < (local as i16 as i32) as f64
            || (rep.wrapping_mul(2) as i32 > cap && rep > 0x109a);
        if !expandir {
            if cqc1_sortear(0x14).wrapping_add(5) >= cqc1_w(cqc1_d(clube + 0x57) + 0x69) as i32 {
                break 'obras;
            }
            let est = cqc1_d(clube + 0x69);
            let livre = cqc1_i(est + 0x3c).wrapping_sub(cqc1_i(est + 0x40));
            if livre <= 0 {
                break 'obras;
            }
            let t = match_pl_ftol(saldo as f64 * read_f64(0x968918));
            let mut v = ((t / 75) as i16 as i32 / 500) as i16 as i32 * 500;
            if v >= livre {
                v = livre;
            }
            if (v as i16) < 0x1f4 {
                if cqc1_i(est + 0x3c) >= 0x7530 {
                    break 'obras;
                }
                cqc1_somar(est + 0x44, 0x5dc);
                post_works_refused_news(this, clube, 2, 2);
                break 'obras;
            }
            let v = v as i16 as i32;
            cqc1_somar(est + 0x3c, v);
            cqc1_somar(cqc1_d(clube + 0x69) + 0x40, v);
            crate::comparison::branch("cqc_1 seats");
            charge_expense(fin, 0x8c, 0x12c, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(75))); // cqc1: os lugares sentados
            post_works_news(this, clube, v, 3);
            break 'obras;
        }
        let tipo1 = cqc1_b(clube + 0x64) == 1;
        let base = if saldo > 0x1312d00 {
            if tipo1 { 16000 } else { 21000 }
        } else if tipo1 {
            match_pl_ftol(saldo as f64 * read_f64(0x967880)) / 125
        } else {
            match_pl_ftol(saldo as f64 * read_f64(0x968918)) / 50
        };
        let lim = (base as i16 as i32) / 500 * 500;
        let teto = cqc1_i(est + 0x44);
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
                cqc1_somar(est + 0x44, 0x5dc);
            }
            if aviso {
                post_works_refused_news(this, clube, 2, 2);
            }
            break 'obras;
        }
        let n = n as i16 as i32;
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, n.wrapping_add(cap));
        if tipo1 {
            cqc1_somar(cqc1_d(clube + 0x69) + 0x40, n);
            charge_expense(fin, 0x8c, 0x12c, (n / 1000 + 1).wrapping_mul(6000).wrapping_add(n).wrapping_mul(125));
            post_works_news(this, clube, n, 1);
        } else {
            charge_expense(fin, 0x8c, 0x12c, n.wrapping_add((n / 1000 + 1).wrapping_mul(1000)).wrapping_mul(50));
            post_works_news(this, clube, n, 2);
        }
    }
    let pedidos = board_requests_row(clube) + 2;
    if cqc1_b(pedidos) & 0x80 != 0 {
        if saldo > (500 - cqc1_w(clube + 0x80) as i32).wrapping_mul(500) {
            let melhorar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59b630usize);
            melhorar(this, clube);
            post_works_news(this, clube, -1, 8);
        } else {
            post_works_refused_news(this, clube, 8, -1);
        }
        *((board_requests_row(clube) + 2) as *mut u8) &= 0x7f;
    }
    pay_dividend(this, clube, fin);
}

/// O centro de treino do clube da máquina no fim da temporada (o trecho 0x59d32a..0x59d54d da 0x59cfa0), cobrado nas
/// finanças do próprio clube ([tabela] + id × 0x167, sem o parceiro).
unsafe fn upgrade_ai_training_ground(this: usize, clube: usize, saldo: i32) {
    let tecnico = cqc1_d(clube + 0xcf);
    if tecnico == 0 || cqc1_w(clube + 0x80) == 0 || cqc1_b(clube + 0x7f) as i8 >= 0x14 || cqc1_d(tecnico + 0x69) == 0 {
        return;
    }
    if fin_prm_is_human_club(clube) != 0 {
        return;
    }
    let q = ((cqc1_w(clube + 0x80) as i32) / 500) as i8 as i32 + 8;
    if (cqc1_b(clube + 0x7f) as i8 as i32) <= q || saldo <= 0x225510 {
        return;
    }
    if cqc1_sortear(0x28) >= cqc1_b(cqc1_d(cqc1_d(clube + 0xcf) + 0x69) + 0x22) as i8 as i32 {
        return;
    }
    let nivel = (clube + 0x7f) as *mut i8;
    let proprio = || cqc1_d(this).wrapping_add((cqc1_i(clube) as u32).wrapping_mul(0x167) as usize);
    let cl = *nivel;
    let custo = (cl as i32) * 50000 + 75000;
    if cl < 0x14 {
        *nivel = cl + 1;
    }
    charge_expense(proprio(), 0x8c, 0x12c, custo);
    let cl = *nivel;
    let rep = cqc1_w(clube + 0x80);
    if cl < 0x14 && (rep > 0x203a || cqc1_saldo(proprio()) > 0xe4e1c0) {
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

/// O dividendo do fim da temporada (o trecho 0x59db6f..0x59dd25 da 0x59cfa0), pelo saldo i64 das finanças.
unsafe fn pay_dividend(this: usize, clube: usize, fin: usize) {
    let rep = cqc1_w(clube + 0x80) as i32;
    let mut lim = rep.wrapping_mul(8000);
    if lim < 0x7a120 {
        lim = 0x7a120;
    }
    let s = cqc1_saldo(fin);
    let v;
    if s > lim as i64 && cqc1_b(clube + 0x82) != 0 {
        let r = cqc1_sortear(0x1f5);
        if (rep as i16) < 0xfa0 {
            v = r.wrapping_add(0x5dc).wrapping_mul(rep).max(0xc350);
        } else {
            v = r.wrapping_add(0x7d0).wrapping_mul(rep);
        }
    } else {
        let mut s16 = rep as i16;
        let pres = cqc1_d(clube + 0xbf);
        if pres != 0 && cqc1_d(pres + 0x69) != 0 {
            let e = (7i16.wrapping_sub(cqc1_b(pres + 0x59) as i8 as i16)).wrapping_mul(3).wrapping_sub(cqc1_b(cqc1_d(pres + 0x69) + 0x20) as i8 as i16);
            s16 = (rep as i16).wrapping_add(e.wrapping_mul(5));
            if s16 < 0x1f4 {
                s16 = 0x1f4;
            }
        }
        let sv = s16 as i32;
        if s > sv.wrapping_mul(6000) as i64 {
            let r = cqc1_sortear(0x1f5);
            if cqc1_w(clube + 0x80) < 0xfa0 {
                crate::comparison::branch("cqc_1 small club dividend by chairman");
                v = r.wrapping_add(0x3e8).wrapping_mul(rep); // cqc1: o dividendo do pequeno pelo presidente
            } else {
                v = r.wrapping_add(0x5dc).wrapping_mul(rep);
            }
        } else if s > sv.wrapping_mul(3000) as i64 {
            v = if cqc1_w(clube + 0x80) < 0xfa0 { rep.wrapping_mul(500) } else { rep.wrapping_mul(750) };
        } else {
            return;
        }
    }
    crate::comparison::branch("cqc_1 dividend");
    cqc1_somar(fin + 0x94, v);
    cqc1_somar(fin + 0x134, v);
    core::ptr::write_unaligned(fin as *mut i64, cqc1_saldo(fin).wrapping_sub(v as i64)); // cqc1: o dividendo sai do saldo
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x59fe10usize);
    noticia(this, clube, v);
}

/// O comparador do fim de temporada do clube: lado a lado (a cópia sem contar os sorteios) nas finanças do parceiro e do
/// próprio clube, no clube, no estádio (78 bytes), na linha dos pedidos (0x49 bytes) e no estado do sorteio. As notícias
/// (0x59ffb0, 0x59fae0, 0x59fe10) são montadas na pilha e só são entregues ao técnico humano (0x7861e0): com técnico da
/// máquina elas não deixam nada além do sorteio(2) da recusa, que as duas rodadas fazem. Só a original: o técnico
/// humano (a notícia entra na caixa dele) e o estádio dividido (mexe noutro clube e nas finanças dele).
unsafe extern "thiscall" fn compare_season_end_club_update(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if clube == 0 || selecao(clube) != 0 || fin_prm_has_human_manager(clube) {
        return o(this, clube);
    }
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, clube, &mut a);
    let estadio = cqc1_d(clube + 0x69);
    if fin == 0 || (estadio != 0 && cqc1_b(clube + 0x6d) == 0 && cqc1_d(clube + 0x57) != 0 && cqc1_i(fin) > 0x2160ec0) {
        return o(this, clube); // ponytail: a compra do estádio pode mexer no dono; só a original
    }
    let proprio = cqc1_d(this).wrapping_add((cqc1_i(clube) as u32).wrapping_mul(0x167) as usize);
    let mut reg = vec![(fin, 0x167), (clube, 0x245), (board_requests_row(clube), 0x49)];
    if proprio != fin {
        reg.push((proprio, 0x167));
    }
    if estadio != 0 {
        reg.push((estadio, 0x4e));
    }
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clube), || crate::rng::without_counting(|| season_end_club_update(this, clube)));
    SCOREBOARD.record(igual, || format!("season_end_club_update(clube {:x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// { 0x59fe10 } (thiscall, ret 8, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_dividend(clube, valor)`, pelo fim de temporada do clube (0x59cfa0). A notícia 0x177c (0x77bbe0:
/// ecx = local; 2 pushes -> thiscall, ret 8) com o id do clube, o valor e a marca +0x82 (0x785790: ecx = local; 2
/// pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8).
unsafe extern "thiscall" fn news_dividend(_this: usize, clube: usize, valor: i32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x177c, 0);
    gravar(p, 0, cqc1_d(clube) as u32);
    gravar(p, 1, valor as u32);
    gravar(p, 2, cqc1_b(clube + 0x82) as i8 as i32 as u32);
    entregar(cqc1_d(0xb7431c), p, clube);
}

/// O comparador da notícia do dividendo: só a original (ponytail: a notícia vai para a caixa do técnico humano e não
/// pode ser entregue duas vezes; com técnico da máquina não deixa nada).
unsafe extern "thiscall" fn compare_news_dividend(this: usize, clube: usize, valor: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(%%I2%%)(this, clube, valor)
}
