// ---------- 266a leva: o fim de temporada das finanças: o fim de temporada da nação, as obras, o centro de treino e o dividendo de cada clube e a notícia do dividendo (fin_prm.cpp) ----------

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
unsafe fn cqc1_cobrar(fin: usize, mes: usize, ano: usize, v: i32) {
    cqc1_somar(fin + mes, v);
    cqc1_somar(fin + ano, v);
    core::ptr::write_unaligned(fin as *mut i64, cqc1_saldo(fin).wrapping_sub(v as i64));
}

unsafe fn cqc1_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
unsafe fn cqc1_humano(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0)
}

/// O registro de finanças do clube (0x167 bytes) pelo parceiro: 0x540a50 (3 pushes; add esp, 0xc -> cdecl(clube, &a,
/// 1)); o parceiro quando ele existe e o clube é o B (a = 0). `a` é a local da original (a 0x540a50 a grava).
unsafe fn cqc1_financas_do_parceiro(this: usize, clube: usize, a: &mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, a, 1);
    let c = if p != 0 && *a == 0 { p } else { clube };
    cqc1_d(this).wrapping_add((cqc1_i(c) as u32).wrapping_mul(0x167) as usize)
}

/// A notícia das obras e do treino (0x59ffb0: ecx = tabela; 3 pushes (tipo, valor, clube); sem add esp -> thiscall,
/// ret 0xc).
unsafe fn cqc1_noticia(this: usize, clube: usize, valor: i32, tipo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(0x59ffb0usize);
    f(this, clube, valor, tipo);
}

/// A notícia da obra recusada (0x59fae0: ecx = tabela; 3 pushes; sem add esp -> thiscall, ret 0xc; sorteia 2 fora do
/// motivo 2).
unsafe fn cqc1_recusa(this: usize, clube: usize, assunto: i32, motivo: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32) = core::mem::transmute(0x59fae0usize);
    f(this, clube, assunto, motivo);
}

/// A linha do clube na tabela dos técnicos ([[0xb67a44]] + id × 0x49); o byte +2 tem os pedidos à diretoria (bit 6:
/// estádio; bit 7: centro de treino).
unsafe fn cqc1_linha_dos_pedidos(clube: usize) -> usize {
    cqc1_d(cqc1_d(0xb67a44)).wrapping_add((cqc1_i(clube) as u32).wrapping_mul(0x49) as usize)
}

/// O técnico do clube (+0xcf) é humano (entre as 16 últimas pessoas): a notícia vai para ele (0x7861e0).
unsafe fn cqc1_tecnico_humano(clube: usize) -> bool {
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
unsafe extern "thiscall" fn fim_de_temporada_da_nacao(this: usize, entrada: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if depois(0xae2c90, cqc1_d(0xdd6ec8) as u32, cqc1_d(0xdd6ecc) as u32) != 0 {
        return;
    }
    if entrada == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(1509)(this, entrada); // a caixa
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
                        crate::comparacao::ramo("cqc_1 nacao inativa abaixo de 6000");
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
unsafe extern "thiscall" fn comparar_fim_de_temporada_da_nacao(this: usize, entrada: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1509);
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
        PLACAR.registrar(zerado, || format!("fim_de_temporada_da_nacao(entrada {:#x}) clube {} sem zerar os totais do ano", entrada, k));
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
unsafe extern "thiscall" fn fim_de_temporada_do_clube(this: usize, clube: usize) {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(1510)(this, clube); // a caixa
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return;
    }
    let mut local = 0u32; // o [esp + 0x20] da original (a 0x540a50 do dono e a média de público)
    let mut a = 0u32;
    let fin = cqc1_financas_do_parceiro(this, clube, &mut a);
    if fin == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(1510)(this, clube); // a caixa
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
            let f2 = if dono != 0 { cqc1_financas_do_parceiro(this, dono, &mut local) } else { 0 };
            if dono == 0 || f2 == 0 {
                *((clube + 0x6d) as *mut u8) = 1;
                cqc1_cobrar(fin, 0x8c, 0x12c, 20_000_000);
                cqc1_noticia(this, clube, cqc1_i(cqc1_d(clube + 0x69)), 4);
            } else if cqc1_b(f2 + 0x165) == 1 || cqc1_b(f2 + 0x165) == 2 {
                *((clube + 0x6d) as *mut u8) = 1;
                *((dono + 0x6d) as *mut u8) = 0;
                cqc1_cobrar(fin, 0x8c, 0x12c, 20_000_000);
                cqc1_somar(f2 + 0xb4, 20_000_000);
                cqc1_somar(f2 + 0x154, 20_000_000);
                core::ptr::write_unaligned(f2 as *mut i64, cqc1_saldo(f2).wrapping_add(20_000_000));
                cqc1_noticia(this, clube, cqc1_i(cqc1_d(clube + 0x69)), 4);
                cqc1_noticia(this, dono, cqc1_i(cqc1_d(clube + 0x69)), 5);
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
        cqc1_centro_de_treino(this, clube, saldo);
        let pedidos = cqc1_linha_dos_pedidos(clube) + 2;
        if cqc1_b(pedidos) & 0x40 != 0 && cqc1_humano(clube) != 0 {
            aviso = true;
            *(pedidos as *mut u8) &= 0xbf;
        }
        let est = cqc1_d(clube + 0x69);
        if est != 0 {
            if saldo > 0x10b0760 && cqc1_w(clube + 0x80) > 0x1d4c && cqc1_i(est + 0x3c) > 0x61a8 && cqc1_b(est + 0x4d) == 0 {
                *((est + 0x4d) as *mut u8) = 1;
                cqc1_cobrar(fin, 0x8c, 0x12c, 1_500_000);
                cqc1_noticia(this, clube, cqc1_i(cqc1_d(clube + 0x69)), 6);
                break 'obras;
            }
            if saldo > 0x47868c0 && cqc1_w(clube + 0x80) > 0x2422 && cqc1_i(est + 0x3c) > 0xe09c && cqc1_b(clube + 0x7f) == 0x14 && cqc1_b(est + 0x4c) == 0 {
                *((est + 0x4c) as *mut u8) = 1;
                cqc1_cobrar(fin, 0x8c, 0x12c, 7_500_000);
                break 'obras;
            }
        }
        let pode = saldo > 0x501bd0 || (cqc1_humano(clube) != 0 && saldo.wrapping_add((cqc1_w(clube + 0x80) as i32).wrapping_mul(1000)) > 0x501bd0);
        if !pode {
            if aviso {
                cqc1_recusa(this, clube, 2, -1);
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
            || (cap as f64 * f64_em(0x96a720)) < (local as i16 as i32) as f64
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
            let t = ftol(saldo as f64 * f64_em(0x968918));
            let mut v = ((t / 75) as i16 as i32 / 500) as i16 as i32 * 500;
            if v >= livre {
                v = livre;
            }
            if (v as i16) < 0x1f4 {
                if cqc1_i(est + 0x3c) >= 0x7530 {
                    break 'obras;
                }
                cqc1_somar(est + 0x44, 0x5dc);
                cqc1_recusa(this, clube, 2, 2);
                break 'obras;
            }
            let v = v as i16 as i32;
            cqc1_somar(est + 0x3c, v);
            cqc1_somar(cqc1_d(clube + 0x69) + 0x40, v);
            crate::comparacao::ramo("cqc_1 lugares sentados");
            cqc1_cobrar(fin, 0x8c, 0x12c, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(75))); // cqc1: os lugares sentados
            cqc1_noticia(this, clube, v, 3);
            break 'obras;
        }
        let tipo1 = cqc1_b(clube + 0x64) == 1;
        let base = if saldo > 0x1312d00 {
            if tipo1 { 16000 } else { 21000 }
        } else if tipo1 {
            ftol(saldo as f64 * f64_em(0x967880)) / 125
        } else {
            ftol(saldo as f64 * f64_em(0x968918)) / 50
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
                cqc1_recusa(this, clube, 2, 2);
            }
            break 'obras;
        }
        let n = n as i16 as i32;
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, n.wrapping_add(cap));
        if tipo1 {
            cqc1_somar(cqc1_d(clube + 0x69) + 0x40, n);
            cqc1_cobrar(fin, 0x8c, 0x12c, (n / 1000 + 1).wrapping_mul(6000).wrapping_add(n).wrapping_mul(125));
            cqc1_noticia(this, clube, n, 1);
        } else {
            cqc1_cobrar(fin, 0x8c, 0x12c, n.wrapping_add((n / 1000 + 1).wrapping_mul(1000)).wrapping_mul(50));
            cqc1_noticia(this, clube, n, 2);
        }
    }
    let pedidos = cqc1_linha_dos_pedidos(clube) + 2;
    if cqc1_b(pedidos) & 0x80 != 0 {
        if saldo > (500 - cqc1_w(clube + 0x80) as i32).wrapping_mul(500) {
            let melhorar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59b630usize);
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
unsafe fn cqc1_centro_de_treino(this: usize, clube: usize, saldo: i32) {
    let tecnico = cqc1_d(clube + 0xcf);
    if tecnico == 0 || cqc1_w(clube + 0x80) == 0 || cqc1_b(clube + 0x7f) as i8 >= 0x14 || cqc1_d(tecnico + 0x69) == 0 {
        return;
    }
    if cqc1_humano(clube) != 0 {
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
    cqc1_cobrar(proprio(), 0x8c, 0x12c, custo);
    let cl = *nivel;
    let rep = cqc1_w(clube + 0x80);
    if cl < 0x14 && (rep > 0x203a || cqc1_saldo(proprio()) > 0xe4e1c0) {
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
unsafe fn cqc1_dividendo(this: usize, clube: usize, fin: usize) {
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
                crate::comparacao::ramo("cqc_1 dividendo do pequeno pelo presidente");
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
    crate::comparacao::ramo("cqc_1 dividendo");
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
unsafe extern "thiscall" fn comparar_fim_de_temporada_do_clube(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1510);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if clube == 0 || selecao(clube) != 0 || cqc1_tecnico_humano(clube) {
        return o(this, clube);
    }
    let mut a = 0u32;
    let fin = cqc1_financas_do_parceiro(this, clube, &mut a);
    let estadio = cqc1_d(clube + 0x69);
    if fin == 0 || (estadio != 0 && cqc1_b(clube + 0x6d) == 0 && cqc1_d(clube + 0x57) != 0 && cqc1_i(fin) > 0x2160ec0) {
        return o(this, clube); // ponytail: a compra do estádio pode mexer no dono; só a original
    }
    let proprio = cqc1_d(this).wrapping_add((cqc1_i(clube) as u32).wrapping_mul(0x167) as usize);
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

/// { 0x59fe10 } (thiscall, ret 8, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_dividend(clube, valor)`, pelo fim de temporada do clube (0x59cfa0). A notícia 0x177c (0x77bbe0:
/// ecx = local; 2 pushes -> thiscall, ret 8) com o id do clube, o valor e a marca +0x82 (0x785790: ecx = local; 2
/// pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8).
unsafe extern "thiscall" fn noticia_do_dividendo(_this: usize, clube: usize, valor: i32) {
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
unsafe extern "thiscall" fn comparar_noticia_do_dividendo(this: usize, clube: usize, valor: i32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(1511)(this, clube, valor)
}

// ---------- 266a leva: o teto salarial: o salário máximo que o clube oferece e as notícias das obras e das obras recusadas (fin_prm.cpp) ----------

unsafe fn cqc2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc2_q(a: usize) -> i64 {
    core::ptr::read_unaligned(a as *const i64)
}

/// `v` × a constante f64 em `k`, truncado (_ftol).
unsafe fn cqc2_vezes(v: i32, k: usize) -> i32 {
    ftol(v as f64 * f64_em(k))
}

/// O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, a, 1, 0, 0)).
unsafe fn cqc2_humano(clube: usize, a: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, a, 1, 0, 0)
}

/// A faixa de riqueza (0x5986b0: mov ecx, finanças -> thiscall).
unsafe fn cqc2_faixa(fin: usize) -> i8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    f(fin) as i8
}

/// A seleção (0x53b900: push; add esp, 4 -> cdecl).
unsafe fn cqc2_selecao(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(clube) != 0
}

/// O valor do contrato da pessoa (0x4d0c20: mov ecx, [0xae1050]; push pessoa -> thiscall, ret 4), com sinal.
unsafe fn cqc2_valor(pessoa: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4d0c20usize);
    f(cqc2_d(0xae1050), pessoa) as i32
}

/// O bônus de quem paga mais (o trecho 0x596c0d..0x597238): clube da Inglaterra, Itália ou Espanha, reputação acima
/// de 4750, faixa 0 a 2, pelo fator salarial `w` (a palavra, com sinal) e pela marca +0x82.
unsafe fn cqc2_bonus(clube: usize, faixa: i8, w: i32) -> i32 {
    let nac = cqc2_d(clube + 0x53);
    if nac == 0 {
        return 0;
    }
    let n = cqc2_i(nac);
    if n != cqc2_i(0x9cf2e4) && n != cqc2_i(0x9cf360) && n != cqc2_i(0x9cf488) {
        return 0;
    }
    if cqc2_w(clube + 0x80) <= 0x128e || !(faixa == 2 || faixa == 1 || faixa == 0) {
        return 0;
    }
    let w16 = w as i16;
    let x = w16 as i32;
    let (base, oito) = if cqc2_b(clube + 0x82) != 0 { (0x4e2, 0x2710) } else { (0x2ee, 0x1770) };
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
unsafe fn cqc2_teto_do_humano(pessoa: usize, np: usize, cargo: i8) -> i32 {
    let idade = cqc2_b(pessoa + 0x18) as i8;
    let marca = cqc2_b(pessoa + 0x22) != 0;
    let cx = cqc2_w(np + 0xb);
    let tecnico = cargo < 4 && cargo != 0 && cargo != 7;
    if cx < 0xea6 {
        let ax = cqc2_w(np + 5);
        if ax < 0x3c {
            if idade <= 0x17 { 0x1388 } else { 0x1388 + 0x9c4 }
        } else if ax < 0x64 {
            if (cargo >= 4 || cargo == 0) && idade < 0x18 { 0x2710 } else { 0x3a98 }
        } else {
            0x61a8
        }
    } else if cx < 0x1482 {
        if cqc2_w(np + 5) >= 0x64 { 0x7530 } else { 0x7530 - 5000 }
    } else if cx < 0x1c52 {
        if idade > 0x22 && cqc2_w(np + 5) < 0x78 {
            if tecnico { 0x927c } else { 0x7ef4 }
        } else {
            let cx2 = cqc2_w(np + 5);
            if cx2 < 0x8c && cqc2_w(np + 0xd) < 0xea6 {
                0x9c40 + if marca { 0x1388 } else { 0 }
            } else if tecnico {
                if marca { 0xe09c } else if cx2 >= 0x8c { 0x13880 } else { 0x13880 - 15000 }
            } else {
                0xafc8
            }
        }
    } else if cqc2_w(np + 0xd) < 0x1a5e {
        0x186a0 + if marca { 0x61a8 } else { 0 }
    } else {
        0x2ab98
    }
}

/// { 0x5968d0 } (thiscall, ret 8, fin_prm.cpp; os asserts citam finance.CPP; 0x14ac bytes com x87):
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
unsafe extern "thiscall" fn teto_salarial(fin: usize, pessoa: usize, cargo_arg: u32) -> i32 {
    let mut cargo = cargo_arg as u8;
    let clube = cqc2_d(0xae23b4).wrapping_add((cqc2_i(fin + 8) as u32).wrapping_mul(0x245) as usize);
    let verba = ftol(cqc2_q(fin) as f64 * f64_em(0x968de8));
    let mut liga: i32 = 100;
    let mut fator: f64 = 0.25; // [esp + 0x3c]: 0x3fd00000_00000000
    let mut contrato: usize = 0;
    if cqc2_b(clube + 0x64) == 3 {
        return 0;
    }
    let rep = cqc2_w(clube + 0x80);
    let mut di: i32 = rep as i32 / 50;
    let faixa: i8 = if cqc2_selecao(clube) { 0 } else { cqc2_faixa(fin) };
    let nac = cqc2_d(clube + 0x53);
    let e = |g: usize| cqc2_i(nac) == cqc2_i(g);
    let tabela: usize = if nac == 0 || e(0x9cf510) || e(0x9cf2dc) || e(0x9cf3e4) || e(0x9cf224) {
        0x9c8498
    } else if [0x9cf36c, 0x9cf4a8, 0x9cf2ac, 0x9cf430, 0x9cf408, 0x9cf4f8, 0x9cf434, 0x9cf2c0, 0x9cf2fc, 0x9cf45c, 0x9cf314].iter().any(|&g| e(g)) {
        0x9c8548
    } else if e(0x9cf300) || (cqc2_b(clube + 0x82) != 0 && faixa != 2 && faixa != 1) {
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
        let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        contrato = com_clube(cqc2_d(0xae1050), pessoa, clube);
        if contrato != 0 {
            cargo = cqc2_b(contrato + 0x4f) >> 4;
        }
    }
    let mut r: i32; // [esp + 0x18]: o teto que sai
    let b20: i32;
    if pessoa != 0 {
        r = 0;
        if cqc2_d(pessoa + 0x39) == clube {
            r = cqc2_valor(pessoa);
            let t = cqc2_i(fin + 0x14).wrapping_mul(2);
            if r > t {
                r = t;
            }
        }
        if cqc2_i(fin + 0x14) > r {
            r = cqc2_i(fin + 0x14);
        }
        if cqc2_d(pessoa + 0x69) != 0 && cqc2_d(pessoa + 0x61) == 0 {
            let cap = match (cqc2_b(pessoa + 0x3d) as i8 as i32).wrapping_sub(5) {
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
            let mut dl = cqc2_b(nac + 0x7e) as i8;
            if dl < 1 {
                dl = 1;
            } else if dl > 3 {
                dl = 3;
            }
            let bl = cqc2_b(nac + 0x85) as i8;
            let q = bl as i32 as f64 / (dl as i32 * 20) as f64;
            f = q;
            if dl == 1 {
                let m = if e(0x9cf2e4) || e(0x9cf360) || e(0x9cf488) || e(0x9cf36c) {
                    f64_em(0x967890)
                } else if e(0x9cf4f8) {
                    f64_em(0x969040)
                } else {
                    let (k, tres) = if e(0x9cf314) || e(0x9cf300) || e(0x9cf45c) || e(0x9cf338) {
                        (0x96a5d0, false)
                    } else if e(0x9cf260) || e(0x9cf218) || e(0x9cf4a8) || e(0x9cf244) || e(0x9cf434) {
                        (0x96a5d8, false)
                    } else if e(0x9cf2c0) || e(0x9cf408) || e(0x9cf2fc) {
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
            let lc = cqc2_d(clube + 0x57);
            if lc != 0 {
                let d2 = cqc2_d(clube + 0x5b);
                let c16 = cqc2_w(lc + 0x69);
                let cubo = |x: i32| x.wrapping_mul(x).wrapping_mul(x);
                let v = if d2 != 0 && cqc2_w(d2 + 0x69) > c16 {
                    let t1 = ftol(cubo(c16 as i32) as f64 * f * f64_em(0x96a5c8));
                    let t2 = ftol(cubo(cqc2_w(d2 + 0x69) as i32) as f64 * f * f64_em(0x968928));
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
        let menos = cqc2_i(fin + 0xc).wrapping_sub(t.wrapping_add(verba) / 52); // cqc2: a folha e a verba por semana
        let idx = ((d16 / 10) * 8) as isize as usize;
        let vv = cqc2_q(tabela.wrapping_add(idx)).wrapping_sub(menos as i64) as f64 * f;
        let piso = cqc2_q((if padrao { 0x9c83e8usize } else { 0x9c8338 }).wrapping_add(idx)) as f64 * f;
        r = if vv <= piso { ftol(piso) } else { ftol(vv) };
        if !cqc2_selecao(clube) {
            match cqc2_faixa(fin) {
                1 => r = cqc2_vezes(r, 0x9689b8),
                2 => r = cqc2_vezes(r, 0x9689c0),
                _ => {}
            }
            if nac != 0 {
                let ita_esp = e(0x9cf360) || e(0x9cf488);
                if ita_esp {
                    if rep > 0x1e46 {
                        r = cqc2_vezes(r, 0x9689b8);
                    }
                } else if e(0x9cf2e4) && rep > 0x1e46 {
                    r = cqc2_vezes(r, 0x968fa0);
                }
                if cqc2_b(clube + 0x82) == 0 && rep > 0x1e46 && ita_esp {
                    r = cqc2_vezes(r, 0x9689b8);
                }
            }
        }
        b20 = cqc2_bonus(clube, faixa, s);
    }
    if pessoa != 0 {
        let repf = rep as i32 as f64;
        let t = repf * repf * fator * f64_em(0x96a5c0) + cqc2_i(fin + 0x10) as f64;
        let v = cqc2_valor(pessoa);
        s = if (v as f64) < t {
            crate::comparacao::ramo("cqc_2 teto pela reputacao");
            ftol(repf * repf * fator * f64_em(0x96a5c0) + cqc2_i(fin + 0x10) as f64) // cqc2: a reputação ao quadrado
        } else {
            cqc2_valor(pessoa)
        };
        let np = cqc2_d(pessoa + 0x61);
        let c = cargo as i8;
        if np != 0 && cqc2_w(np + 0xb) > 0x1482 && (c == 3 || c == 2 || c == 1) && (cqc2_b(pessoa + 0x18) as i8) < 0x20 {
            let np2 = || cqc2_d(pessoa + 0x61);
            if c == 3 {
                let bf = b20 as f64;
                let lim = bf * f64_em(0x9689b0);
                if (r as f64) < lim {
                    r = if cqc2_humano(clube, 0) == 0 && np2() != 0 && cqc2_w(np2() + 0xb) > 0x1c52 { ftol(bf * f64_em(0x9689d8)) } else { ftol(lim) };
                }
            } else if c == 2 {
                let lim = b20 as f64 * f64_em(0x9689d8);
                if (r as f64) < lim {
                    r = if cqc2_humano(clube, 0) == 0 && np2() != 0 && cqc2_w(np2() + 0xb) > 0x1e46 { b20 } else { ftol(lim) };
                }
            } else if r < b20 {
                r = b20;
            }
        }
        if cqc2_humano(clube, 0) != 0 {
            let np = cqc2_d(pessoa + 0x61);
            if np != 0 {
                let teto = cqc2_teto_do_humano(pessoa, np, cargo as i8);
                if teto < r {
                    r = teto.wrapping_sub(ftol(r.wrapping_sub(teto) as f64 * f64_em(0x969518)));
                }
            }
        }
        let np = cqc2_d(pessoa + 0x61);
        if np != 0 {
            if nac != 0 && (e(0x9cf360) || e(0x9cf2e4) || e(0x9cf314) || e(0x9cf300) || e(0x9cf488)) {
                let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
                if basco(clube) != 0 {
                    s = cqc2_vezes(s, 0x9689b8);
                }
                let tm = cqc2_d(0xde1f54);
                if tm != 0 {
                    let rr = cqc2_w(clube + 0x80);
                    let div: i32 = if rr < 0x3e8 { 0x3e8 } else { rr as i32 };
                    let tab = cqc2_d(cqc2_d(tm + 0x856) + 0xdc);
                    let x = cqc2_w(tab.wrapping_add((cqc2_i(clube) as usize).wrapping_mul(9)) + 5) as i32;
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
                    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                    contrato != 0 && cqc2_d(pessoa + 0x39) == clube && dias(contrato + 0x2d, cqc2_d(0xae2c90) as u32, cqc2_d(0xae2c94) as u32) < 0xc8
                };
                let rr = cqc2_w(clube + 0x80);
                let k: Option<usize> = if rr > 0x186a {
                    let np = cqc2_d(pessoa + 0x61);
                    if perto() {
                        let np = cqc2_d(pessoa + 0x61);
                        if cqc2_w(np + 0xd) > 0x1482 {
                            Some(0x968978)
                        } else {
                            let ax = cqc2_w(np + 0xb);
                            let cl = cqc2_b(contrato + 0x4f) & 0xf0;
                            if ax > 0x1c52 {
                                Some(0x9689c0)
                            } else if (cl == 0x30 && ax > 0x1482) || cl == 0x10 || cl == 0x20 {
                                Some(0x9689b8)
                            } else {
                                None
                            }
                        }
                    } else if cqc2_w(np + 0xd) > 0x1482 {
                        Some(0x9689c0)
                    } else if cqc2_w(np + 0xb) > 0x1a5e {
                        Some(0x9689b8)
                    } else {
                        None
                    }
                } else if rr > 0x109a {
                    if perto() {
                        let cx = cqc2_w(cqc2_d(pessoa + 0x61) + 0xb);
                        let al = cqc2_b(contrato + 0x4f) & 0xf0;
                        if cx > 0x1a5e {
                            Some(0x9689b8)
                        } else if (al == 0x30 && cx > 0x1482) || al == 0x10 || al == 0x20 {
                            Some(0x968f88)
                        } else {
                            None
                        }
                    } else if cqc2_w(cqc2_d(pessoa + 0x61) + 0xb) > 0x1e46 {
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
            let pres = cqc2_d(clube + 0xbf);
            if pres != 0 && cqc2_d(pres + 0x69) != 0 && (cqc2_w(clube + 0x80) as i32) < cqc2_w(cqc2_d(pessoa + 0x61) + 0xb) as i32 + 0x4e2 {
                crate::comparacao::ramo("cqc_2 presidente");
                s = ftol(((cqc2_b(pres + 0x57) as i8 as i32) as f64 * f64_em(0x967898) + f64_em(0x9689d8)) * s as f64); // cqc2: o presidente
                // 0x547810 (push pres; push pessoa; add esp, 8 -> cdecl), 0x547790 (push clube; push pessoa; add esp, 8 -> cdecl)
                let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
                let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
                if gosta(pessoa, pres) != 0 || favorito(pessoa, clube) != 0 {
                    s = ftol(((cqc2_b(cqc2_d(clube + 0xbf) + 0x57) as i8 as i32) as f64 * f64_em(0x967898) + f64_em(0x967890)) * s as f64);
                }
                let pres = cqc2_d(clube + 0xbf);
                let pc = cqc2_d(pres + 0x69);
                let perfil = cqc2_b(pc + 0xf) as i8;
                if perfil < 6 {
                    let t = cqc2_d(clube + 0xcf);
                    let k = if t != 0 && (cqc2_b(t + 0x57) as i8) > 0xf && cqc2_d(t + 0x69) != 0 && cqc2_w(cqc2_d(t + 0x69) + 0xa) > 0x1c52 { 0x9689c0 } else { 0x9689b8 };
                    s = cqc2_vezes(s, k);
                } else if perfil > 0xf {
                    s = cqc2_vezes(s, 0x9689d0);
                }
                if cqc2_b(clube + 0x82) == 0 && (cqc2_b(pc + 0x20) as i8) > 0xf && (cqc2_b(pres + 0x57) as i8) > 0xf {
                    let rr = cqc2_w(clube + 0x80);
                    let k = if rr >= 0xabe && cqc2_w(cqc2_d(pessoa + 0x61) + 0xb) >= 0x1c52 {
                        if rr < 0x1676 { 0x9689c0 } else { 0x96a5a8 }
                    } else {
                        0x9689b8
                    };
                    s = cqc2_vezes(s, k);
                }
            }
            let c = cargo as i8;
            let np = cqc2_d(pessoa + 0x61);
            match c {
                1 => s = cqc2_vezes(s, 0x968928),
                2 => {
                    if cqc2_humano(clube, 1) == 0 && cqc2_w(np + 0xb) > 0x1a5e {
                        s = cqc2_vezes(s, 0x968928);
                    } else {
                        s = ftol(s as f64);
                        let ok = cqc2_humano(clube, 1) != 0 || {
                            let np = cqc2_d(pessoa + 0x61);
                            np != 0 && cqc2_w(np + 0xb) < 0x1e46
                        };
                        if ok && s > 0x14c08 {
                            s = ftol(s.wrapping_sub(0x14c08) as f64 * f64_em(0x968918) + f64_em(0x96a5a0));
                        }
                    }
                }
                3 => {
                    if cqc2_humano(clube, 1) == 0 && cqc2_w(np + 0xb) > 0x1a5e {
                        s = cqc2_vezes(s, 0x968928);
                    } else {
                        s = cqc2_vezes(s, 0x9689d0);
                        if s > 0xd6d8 {
                            s = 0xd6d8;
                        }
                    }
                }
                4 => {
                    let k = if cqc2_humano(clube, 1) == 0 && cqc2_w(np + 0xb) > 0x1a5e { 0x9689d0 } else { 0x969040 };
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
        s = ftol(x * x * fator * f64_em(0x96a5c0) + cqc2_i(fin + 0x10) as f64);
        if s < b20 {
            s = ftol(s as f64 * f64_em(0x969040) + b20 as f64 * f64_em(0x9689e0));
        }
        if !cqc2_selecao(clube) && nac != 0 {
            if e(0x9cf314) || e(0x9cf300) {
                match cqc2_faixa(fin) {
                    1 => s = cqc2_vezes(s, 0x9689b8),
                    2 => s = cqc2_vezes(s, 0x9689c0),
                    _ => {}
                }
            } else if e(0x9cf360) || e(0x9cf2e4) || e(0x9cf488) {
                match cqc2_faixa(fin) {
                    0 => {
                        if cqc2_w(clube + 0x80) > 0x109a {
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
    let tipo = cqc2_b(clube + 0x64);
    if tipo == 1 {
        if s < 0xfa {
            s = 0xfa;
        }
        if cqc2_w(clube + 0x80) < 0x8ca && s > 0x834 {
            s = ftol((s as f64 - f32_em(0x96a5b8) as f64) * f64_em(0x968918) + f64_em(0x96a5b0));
        }
    } else if tipo == 2 {
        s = cqc2_vezes(s, 0x969040);
        if cqc2_w(clube + 0x80) < 0xabe && s > 0x4b0 {
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
unsafe extern "thiscall" fn comparar_teto_salarial(fin: usize, pessoa: usize, cargo: u32) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(1512)(fin, pessoa, cargo);
    let n = teto_salarial(fin, pessoa, cargo);
    PLACAR.registrar(r == n, || format!("teto_salarial(fin {:x}, pessoa {:x}, cargo {}) original={} rust={}", fin, pessoa, cargo as u8 as i8, r, n));
    r
}

/// { 0x59ffb0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_ground_works(clube, valor, tipo)`, pelo fim de temporada do clube (0x59cfa0: compra do estádio,
/// aquecimento, expansão e lugares, centro de treino) e pelos eventos da semana (0x59ed70). A notícia 0x177d (0x77bbe0:
/// lea ecx, local; push 0; push 0x177d -> thiscall, ret 8) com o id do clube, o valor e o tipo (byte com sinal; 0x785790:
/// lea ecx, local; 2 pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; push clube;
/// push &local -> thiscall, ret 8).
unsafe extern "thiscall" fn noticia_das_obras(_this: usize, clube: usize, valor: u32, tipo: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x177d, 0);
    gravar(p, 0, cqc2_d(clube) as u32);
    gravar(p, 1, valor);
    gravar(p, 2, tipo as u8 as i8 as i32 as u32);
    entregar(cqc2_d(0xb7431c), p, clube);
}

/// O comparador da notícia das obras: só a original (ponytail: a notícia vai para a caixa do técnico humano e não pode
/// ser entregue duas vezes; com técnico da máquina não deixa nada).
unsafe extern "thiscall" fn comparar_noticia_das_obras(this: usize, clube: usize, valor: u32, tipo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1513)(this, clube, valor, tipo)
}

/// { 0x59fae0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_request_refused(clube, assunto, motivo)`, pelo fim de temporada do clube (0x59cfa0: o pedido de
/// estádio ou de centro de treino recusado). Sem clube, nada. A notícia 0x1788 (0x77bbe0) com o assunto (palavra com
/// sinal) no dado 1 e no dado 2 o motivo 2 ou o sorteio(2) (0x90cfc0: push 2; add esp, 4 -> cdecl; a palavra com
/// sinal), entregue ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn noticia_da_obra_recusada(_this: usize, clube: usize, assunto: u32, motivo: u32) {
    if clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1788, 0);
    gravar(p, 1, assunto as u16 as i16 as i32 as u32);
    let v = if motivo as u16 == 2 { 2 } else { sortear(2) };
    gravar(p, 2, v as u16 as i16 as i32 as u32);
    entregar(cqc2_d(0xb7431c), p, clube);
}

/// O comparador da notícia da obra recusada: só a original (ponytail: entrega ao técnico humano e sorteia; não pode
/// rodar duas vezes).
unsafe extern "thiscall" fn comparar_noticia_da_obra_recusada(this: usize, clube: usize, assunto: u32, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1514)(this, clube, assunto, motivo)
}

// ---------- 266a leva: os conflitos de calendário: os conflitos de um jogo nos dias em volta, os conflitos de um dia e a carga da conferência das pernas (fix_man.cpp) ----------

unsafe fn cqc3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O ensaio do comparador dos conflitos de um dia: a cópia não acrescenta (0x68d0d0 aloca o nó e a cópia do jogo) e
/// guarda os jogos que acrescentaria, na ordem.
static mut CQC3_ENSAIO: bool = false;
static mut CQC3_ACRESCIMOS: Vec<usize> = Vec::new();

/// Acrescenta a cópia dos 0x4f bytes do jogo no fim da lista (0x68d0d0: ecx = lista; push 0x4f; push jogo -> thiscall,
/// ret 8); no ensaio, só guarda o jogo.
unsafe fn cqc3_acrescentar(lista: usize, jogo: usize) {
    if CQC3_ENSAIO {
        CQC3_ACRESCIMOS.push(jogo);
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    f(lista, jogo, 0x4f);
}

/// Os dados dos nós da lista (cabeça em +2, o próximo em +6 do nó, o dado em +0), na ordem.
unsafe fn cqc3_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqc3_d(lista + 2);
    while no != 0 {
        v.push(cqc3_d(no));
        no = cqc3_d(no + 6);
    }
    v
}

/// O conteúdo (0x4f bytes por jogo) de uma sequência de jogos.
unsafe fn cqc3_conteudo(jogos: &[usize]) -> Vec<u8> {
    let mut v = Vec::new();
    for &j in jogos {
        v.extend_from_slice(core::slice::from_raw_parts(j as *const u8, 0x4f));
    }
    v
}

/// A data (dia do mês, mês 0..11) do ano dado (0x549f70: lea ecx, local; push −1; push ano; push mês; push dia ->
/// thiscall, ret 0x10): (dia do ano, ano).
unsafe fn cqc3_dma(dia: u32, mes: u32, ano: u32) -> (u16, u16) {
    let f: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, u32) -> *mut [u8; 8] = core::mem::transmute(0x549f70usize);
    let mut t = [0u8; 8];
    let r = f(&mut t, dia, mes, ano, 0xffff_ffff);
    (cqc3_w(r as usize), cqc3_w(r as usize + 2))
}

/// O jogo `x` está na liga de id [0x9cf590] por um dos clubes (+0x57 → +0).
unsafe fn cqc3_na_liga(clube: usize, id: i32) -> bool {
    let l = cqc3_d(clube + 0x57);
    l != 0 && cqc3_i(l) == id
}

/// A regra de conflito da competição (método +0x60 da competição [0xadadfc][id]: push b; push a; ecx = comp ->
/// thiscall): 0 sem a tabela ou sem a competição.
unsafe fn cqc3_regra(id_de: usize, a: usize, b: usize) -> u32 {
    let tab = cqc3_d(0xadadfc);
    if tab == 0 {
        return 0;
    }
    let c = cqc3_d(tab.wrapping_add((cqc3_i(id_de) as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(cqc3_d(cqc3_d(c) + 0x60));
    f(c, a, b)
}

/// O jogo `x` entra (0x16 se já foi jogado, +0x43 diferente de 0xff) e a gravidade sobe ao tipo dele (+0x3a).
unsafe fn cqc3_entra_pelo_tipo(grav: *mut i32, lista: usize, x: usize, conferir_jogado: bool) {
    if conferir_jogado && cqc3_b(x + 0x43) != 0xff {
        *grav = 0x16;
        return;
    }
    cqc3_acrescentar(lista, x);
    let t = cqc3_b(x + 0x3a) as i8 as i32;
    if t > *grav {
        *grav = t;
    }
}

/// O fim comum dos ramos do mesmo clube e do mesmo estádio (0x5adf91/0x5ae060): sem gravidade, 0x14 (`vinte`) ou 1.
unsafe fn cqc3_sem_gravidade(grav: *mut i32, vinte: bool) {
    if *grav == 0 {
        *grav = if vinte { 0x14 } else { 1 };
    }
}

/// { 0x5adba0 } (thiscall, ret 0x1c, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::collect_day_clashes(jogo,
/// gravidade, lista, nações, n, data, deslocamento)`, pelos conflitos de um jogo (0x5ad780) no dia `data` + deslocamento
/// (0x54c5b0: push n de 16 bits; push &local; ecx = data -> thiscall, ret 8). Copa (+0x3a = 1) só no próprio dia. No
/// próprio dia, as datas proibidas dão 19 e saem: 25 de dezembro (fora das competições da nação de [0x9cf36c]), o
/// domingo nas da nação de [0x9cf3e4] (0x54a3f0: ecx = data -> thiscall), entre 10 e 20 de abril nas de [0x9cf31c] (as
/// datas pela 0x549f70: 4 pushes -> thiscall); 17 de maio com clube da nação [0x9cf408] (também com deslocamento,
/// sem sair, se for o próprio dia). Depois, para cada nação de `nações` (fora as 0xff) e cada uma das 3 fatias, a lista
/// do dia (0x5abf10: ecx = gerenciador; 4 pushes -> thiscall, ret 0x10) com o cursor do começo: cada jogo dela (0x68d1b0)
/// que divide clube e fatia (+0x40/+0x41) entra pelo tipo; clube em outra fatia, entra com 0x14 ou 1 pelo +0x3f e pelo
/// tipo 1; o mesmo estádio (+0x18) fora da competição de [0x9cf76c], igual; as seleções da nação dos clubes (0x53b3d0:
/// push; add esp, 4 -> cdecl) e os clubes de reputação acima de 5000 pela regra da competição (+0x60); os jogos da
/// competição de [0x9cf690] com clube da liga de [0x9cf590] dão 19; jogo já jogado (+0x43) dá 0x16. Clube ausente: a
/// caixa e o fim do jogo (a original).
unsafe extern "thiscall" fn conflitos_do_dia(this: usize, jogo: usize, grav: *mut i32, lista: usize, nacoes: usize, n: u32, data: usize, desloc: u32) {
    // 0x53b900 (push; add esp, 4 -> cdecl), 0x53b3d0 (push; add esp, 4 -> cdecl)
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let semana: unsafe extern "thiscall" fn(*const [u8; 8]) -> i32 = core::mem::transmute(0x54a3f0usize);
    let lista_do_dia: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    // 0x76d940: ecx = [0xb74318]; push comp -> thiscall, ret 4 (a cópia Rust ignora o ecx)
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
    let e_sel = selecao(cqc3_d(jogo + 0x1c));
    let n1 = cqc3_d(cqc3_d(jogo + 0x1c) + 0x53);
    // ponytail: sem nação a original deixa a seleção sem valor; todo clube tem nação, então 0 não acontece no jogo
    let s1 = if n1 != 0 { principal(n1) } else { 0 };
    let n2 = cqc3_d(cqc3_d(jogo + 0x20) + 0x53);
    let s2 = if n2 != 0 { principal(n2) } else { 0 };
    let flag = desloc as u8 as i8;
    let mut dia = [0u8; 8];
    mais(data, &mut dia, flag as i16 as u16 as u32);
    let d = dia.as_ptr() as usize;
    let ano = || cqc3_d(d + 2) as u32; // o dword do ano como a original o empurra (a 0x549f70 e a 0x5abf10 leem a palavra)
    let mesma = |t: (u16, u16)| t.1 == cqc3_w(d + 2) && t.0 == cqc3_w(d);
    let dezenove = || {
        if *grav < 0x13 {
            *grav = 0x13;
        }
    };
    let tipo = cqc3_b(jogo + 0x3a);
    if tipo == 1 && flag != 0 {
        return;
    }
    if flag == 0 {
        let comp = cqc3_d(jogo + 0x14);
        let nc = if comp != 0 { cqc3_d(comp + 0x5d) } else { 0 };
        let e_da = |g: usize| nc != 0 && cqc3_i(nc) == cqc3_i(g);
        if mesma(cqc3_dma(0x19, 0xb, ano())) && !e_da(0x9cf36c) {
            crate::comparacao::ramo("cqc_3 natal");
            return dezenove();
        }
        if e_da(0x9cf3e4) && semana(&dia) as i16 == 6 {
            return dezenove();
        }
        if e_da(0x9cf31c) {
            let t = cqc3_dma(10, 3, ano());
            if t.1 < cqc3_w(d + 2) || (t.1 == cqc3_w(d + 2) && (t.0 as i16) < cqc3_w(d) as i16) {
                let t = cqc3_dma(0x14, 3, ano());
                if t.1 > cqc3_w(d + 2) || (t.1 == cqc3_w(d + 2) && (t.0 as i16) > cqc3_w(d) as i16) {
                    return dezenove();
                }
            }
        }
    }
    let nac = cqc3_d(0xae23a8).wrapping_add((cqc3_i(0x9cf408) as usize).wrapping_mul(0x122));
    if cqc3_d(cqc3_d(jogo + 0x1c) + 0x53) == nac || cqc3_d(cqc3_d(jogo + 0x20) + 0x53) == nac {
        if mesma(cqc3_dma(0x11, 4, ano())) && flag == 0 {
            return dezenove();
        }
    }
    let mut k: i8 = 0;
    while k < n as u8 as i8 {
        let p = nacoes.wrapping_add(k as isize as usize);
        if cqc3_b(p) != 0xff {
            let mut fatia: i8 = 0;
            while fatia <= 2 {
                let l = lista_do_dia(this, ano(), cqc3_d(d) as u32, fatia as u8 as u32, cqc3_b(p) as u32);
                if l != 0 {
                    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc3_d(l + 2));
                    let mut x = proximo(l);
                    while x != 0 {
                        let c1 = cqc3_d(x + 0x1c);
                        let h = cqc3_d(jogo + 0x1c);
                        let a = cqc3_d(jogo + 0x20);
                        if c1 == 0 || h == 0 || a == 0 {
                            // a caixa (linha 0xcc8) e o fim do jogo: a original
                            return orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(1515)(this, jogo, grav, lista, nacoes, n, data, desloc);
                        }
                        let c2 = cqc3_d(x + 0x20);
                        let (sx0, sx1, s0, s1b) = (cqc3_b(x + 0x40), cqc3_b(x + 0x41), cqc3_b(jogo + 0x40), cqc3_b(jogo + 0x41));
                        if (c1 == h && sx0 == s0) || (c1 == a && sx0 == s1b) || (c2 == h && sx1 == s0) || (c2 == a && sx1 == s1b) {
                            crate::comparacao::ramo("cqc_3 mesmo clube na mesma fatia");
                            cqc3_entra_pelo_tipo(grav, lista, x, true); // cqc3: o mesmo clube na mesma fatia
                        } else if (flag == 0 && c1 == h && sx0 != s0) || (c1 == a && sx0 != s1b) || (c2 == h && sx1 != s0) || (c2 == a && sx1 != s1b) {
                            if cqc3_b(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                cqc3_acrescentar(lista, x);
                                if cqc3_b(jogo + 0x3f) == cqc3_b(x + 0x3f) || cqc3_b(x + 0x3a) != 1 {
                                    if *grav == 0 {
                                        crate::comparacao::ramo("cqc_3 mesmo clube noutra fatia");
                                    }
                                    cqc3_sem_gravidade(grav, true); // cqc3: o mesmo clube noutra fatia
                                } else if cqc3_b(jogo + 0x3a) != 1 {
                                    cqc3_sem_gravidade(grav, false);
                                }
                            }
                        } else if flag == 0 && cqc3_d(x + 0x18) != 0 && cqc3_d(jogo + 0x18) == cqc3_d(x + 0x18) && cqc3_i(x) != cqc3_i(0x9cf76c) && cqc3_i(jogo) != cqc3_i(0x9cf76c) {
                            if cqc3_b(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                cqc3_acrescentar(lista, x);
                                let t = cqc3_b(x + 0x3a);
                                if cqc3_b(jogo + 0x3f) == cqc3_b(x + 0x3f) {
                                    if t != 1 {
                                        cqc3_sem_gravidade(grav, true);
                                    } else if *grav < 1 {
                                        *grav = 1;
                                    }
                                } else if t != 1 {
                                    cqc3_sem_gravidade(grav, true);
                                } else if cqc3_b(jogo + 0x3a) != 1 {
                                    cqc3_sem_gravidade(grav, false);
                                }
                            }
                        } else {
                            let abs_flag = (flag as i32).abs();
                            let selecoes = (e_sel == 0
                                && especial(cqc3_d(0xb74318), cqc3_d(x + 0x14)) == 0
                                && abs_flag < 2
                                && (cqc3_b(p) == 0 || cqc3_b(p) == 1)
                                && (cqc3_b(jogo + 0x3a) as i8) < 5
                                && c1 == s1)
                                || c1 == s2
                                || c2 == s1
                                || c2 == s2;
                            if selecoes {
                                if (cqc3_w(h + 0x80) as i16 > 0x1388 || cqc3_w(a + 0x80) as i16 > 0x1388) && cqc3_regra(jogo, jogo, x) != 0 {
                                    cqc3_entra_pelo_tipo(grav, lista, x, true);
                                }
                            } else if cqc3_i(x) == cqc3_i(0x9cf690) && (cqc3_na_liga(h, cqc3_i(0x9cf590)) || cqc3_na_liga(a, cqc3_i(0x9cf590))) {
                                *grav = 0x13;
                            } else if cqc3_i(jogo) == cqc3_i(0x9cf690) && (cqc3_na_liga(c1, cqc3_i(0x9cf590)) || cqc3_na_liga(c2, cqc3_i(0x9cf590))) {
                                cqc3_entra_pelo_tipo(grav, lista, x, false);
                            } else if e_sel != 0
                                && especial(cqc3_d(0xb74318), cqc3_d(jogo + 0x14)) == 0
                                && abs_flag < 2
                                && cqc3_b(p) != 0
                                && cqc3_b(p) != 1
                            {
                                let (nx1, nx2) = (cqc3_d(c1 + 0x53), cqc3_d(c2 + 0x53));
                                if (nx1 == n1 || nx2 == n1 || nx1 == n2 || nx2 == n2)
                                    && (cqc3_w(c1 + 0x80) as i16 > 0x1388 || cqc3_w(c2 + 0x80) as i16 > 0x1388)
                                    && (cqc3_b(x + 0x3a) as i8) < 5
                                    && cqc3_regra(x, x, jogo) != 0
                                {
                                    cqc3_entra_pelo_tipo(grav, lista, x, true);
                                }
                            }
                        }
                        x = proximo(l);
                    }
                }
                fatia += 1;
            }
        }
        k += 1;
    }
}

/// O comparador dos conflitos de um dia: a cópia em ensaio (guarda os jogos em vez de acrescentar, com uma gravidade
/// própria) e a original a partir do mesmo estado; confere a gravidade e os nós novos no fim da lista (os 0x4f bytes de
/// cada jogo, na ordem). As listas dos dias (0x5abf10) têm o cursor refeito nas duas.
unsafe extern "thiscall" fn comparar_conflitos_do_dia(this: usize, jogo: usize, grav: *mut i32, lista: usize, nacoes: usize, n: u32, data: usize, desloc: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(1515);
    if jogo == 0 || lista == 0 || grav.is_null() || cqc3_d(jogo + 0x1c) == 0 || cqc3_d(jogo + 0x20) == 0 {
        return o(this, jogo, grav, lista, nacoes, n, data, desloc);
    }
    let mut g = *grav;
    CQC3_ACRESCIMOS.clear();
    CQC3_ENSAIO = true;
    conflitos_do_dia(this, jogo, &mut g, lista, nacoes, n, data, desloc);
    CQC3_ENSAIO = false;
    let esperado = cqc3_conteudo(&CQC3_ACRESCIMOS);
    let ne = CQC3_ACRESCIMOS.len();
    let antes = cqc3_itens(lista);
    o(this, jogo, grav, lista, nacoes, n, data, desloc);
    let depois = cqc3_itens(lista);
    let igual = *grav == g && depois.len() == antes.len() + ne && depois[..antes.len()] == antes[..] && cqc3_conteudo(&depois[antes.len()..]) == esperado;
    PLACAR.registrar(igual, || format!("conflitos_do_dia(jogo {:x}, desloc {}) gravidade original={} rust={}, jogos novos original={} rust={}", jogo, desloc as u8 as i8, *grav, g, depois.len().saturating_sub(antes.len()), ne));
}

/// { 0x5ad780 } (thiscall, ret 0xc, fix_man.cpp; quadro SEH da lista nova): `FIXTURE_MANAGER::find_fixture_clashes(jogo,
/// gravidade, dois_dias)`, pelo calendário (0x5aa680, 0x5ab510, 0x5ae380, 0x5ae9d0). Sem jogo ou sem saída: a caixa (a
/// original). Zera a saída; as nações a olhar: 0 a 6, a nação-chave do jogo (0x48dd70: 2 pushes; add esp, 8 -> cdecl),
/// as das ligas dos dois clubes, a do estádio (0x48df40: push; add esp, 4 -> cdecl), as parceiras (0x5af5b0: push;
/// add esp, 4 -> cdecl) e, com seleção mandante (0x53b900), as das duas seleções (0x48dee0: push; add esp, 4 -> cdecl),
/// sem repetir. A chave 3 ou 5 fora da competição de [0x9cf7bc] desliga os dois dias antes. A lista nova (new 0xe:
/// 0x944cf1, push; add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e os conflitos
/// de cada dia (0x5adba0: ecx = this; 7 pushes -> thiscall, ret 0x1c) pela data do jogo (0x549ef0: push +0x28; push
/// +0x2a; ecx = local -> thiscall): abaixo do tipo 0x17, os dois dias antes (com a marca) e o anterior; o próprio dia e
/// o seguinte; de novo os dois antes com a marca. Sem conflito: a lista destruída e liberada (0x68d040: ecx = lista ->
/// thiscall; 0x944c94: push; add esp, 4 -> cdecl), 0x15 na saída se a seleção não deixa (0x776fc0: ecx = [0xb74318];
/// push jogo -> thiscall, ret 4) e 0. Senão a gravidade na saída e a lista.
unsafe extern "thiscall" fn conflitos_do_jogo(this: usize, jogo: usize, saida: *mut i32, dois_dias: u32) -> usize {
    if jogo == 0 || saida.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(1516)(this, jogo, saida, dois_dias); // a caixa
    }
    let chave_do_jogo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x48dd70usize);
    let do_estadio: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x48df40usize);
    let parceira: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x5af5b0usize);
    let da_selecao: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x48dee0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let nova_data: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let do_dia: unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32) = core::mem::transmute(0x5adba0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let pode: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x776fc0usize);
    let mut dois = dois_dias;
    *saida = 0;
    let mut a = [0xffu8; 16];
    for k in 0..7 {
        a[k] = k as u8;
    }
    let chave = chave_do_jogo(cqc3_d(jogo + 0x14), jogo) as i8;
    a[7] = chave as u8;
    let mut n: usize = 8;
    let casa = cqc3_d(jogo + 0x1c);
    if casa != 0 && cqc3_d(casa + 0x57) != 0 {
        a[8] = chave_do_jogo(cqc3_d(casa + 0x57), 0);
        n = 9;
    }
    let fora = cqc3_d(jogo + 0x20);
    if fora != 0 && cqc3_d(fora + 0x57) != 0 {
        a[n] = chave_do_jogo(cqc3_d(fora + 0x57), 0);
        n += 1;
    }
    if cqc3_d(jogo + 0x18) != 0 {
        a[n] = do_estadio(cqc3_d(jogo + 0x18));
        n += 1;
    }
    a[n] = parceira(cqc3_d(jogo + 0x1c));
    n += 1;
    a[n] = parceira(cqc3_d(jogo + 0x20));
    n += 1;
    if cqc3_i(jogo) != cqc3_i(0x9cf7bc) && (chave == 3 || chave == 5) {
        dois = 0;
    }
    if selecao(cqc3_d(jogo + 0x1c)) != 0 {
        a[n] = da_selecao(cqc3_d(jogo + 0x1c));
        n += 1;
        a[n] = da_selecao(cqc3_d(jogo + 0x20));
        n += 1;
    }
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if (a[i] as i8) > -1 && a[i] == a[j] {
                a[j] = 0xff;
            }
        }
    }
    let bloco = novo(0xe);
    let lista = if bloco != 0 { construir(bloco) } else { 0 };
    if lista == 0 {
        aviso_em(0x9cc208, 0xc1d); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        return 0;
    }
    let mut data = [0u8; 8];
    nova_data(&mut data, cqc3_w(jogo + 0x2a) as u32, cqc3_w(jogo + 0x28) as u32);
    let mut g: i32 = 0;
    let (pa, pd) = (a.as_ptr() as usize, data.as_ptr() as usize);
    if (cqc3_b(jogo + 0x3a) as i8) < 0x17 {
        if dois != 0 {
            do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, -2i32 as u32);
        }
        do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, -1i32 as u32);
    }
    do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, 0);
    crate::comparacao::ramo("cqc_3 dia seguinte");
    do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, 1); // cqc3: o dia seguinte
    if dois != 0 {
        do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, -2i32 as u32);
    }
    if g == 0 {
        destruir(lista);
        liberar(lista);
        if pode(cqc3_d(0xb74318), jogo) == 0 {
            *saida = 0x15;
        }
        return 0;
    }
    *saida = g;
    lista
}

/// O comparador dos conflitos de um jogo: a original e a cópia, cada uma com a sua lista nova, a partir do mesmo estado
/// (a original não muda o jogo: só cria a lista e mexe nos cursores das listas dos dias, que as duas refazem); confere a
/// devolução (lista ou 0), a saída e os jogos das duas listas (0x4f bytes cada, na ordem), e destrói e libera a lista
/// da cópia como o calendário faz (0x68d040, 0x944c94).
unsafe extern "thiscall" fn comparar_conflitos_do_jogo(this: usize, jogo: usize, saida: *mut i32, dois_dias: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(1516);
    if jogo == 0 || saida.is_null() {
        return o(this, jogo, saida, dois_dias);
    }
    let r = o(this, jogo, saida, dois_dias);
    let go = *saida;
    let mut gr: i32 = 0;
    let nr = conflitos_do_jogo(this, jogo, &mut gr, dois_dias);
    let (lo, lr) = (cqc3_conteudo(&cqc3_itens(r)), cqc3_conteudo(&cqc3_itens(nr)));
    let igual = (r == 0) == (nr == 0) && go == gr && lo == lr;
    PLACAR.registrar(igual, || format!("conflitos_do_jogo(jogo {:x}) original={:x} saida={} jogos={} rust={:x} saida={} jogos={}", jogo, r, go, lo.len() / 0x4f, nr, gr, lr.len() / 0x4f));
    if nr != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(nr);
        liberar(nr);
    }
    r
}

/// { 0x5af390 } (cdecl, fix_man.cpp): `fix_man_load_fixture_leg_checks()`, pela construção do calendário principal
/// (0x5a9cf0, na carga e na criação do jogo): esvazia a lista das conferências das pernas [0xae2a48] (0x68d080: ecx =
/// lista -> thiscall) e a refaz com os registros de 15 bytes do check_fixture_legs.tmp da pasta temporária (0x90bde0:
/// 2 pushes; add esp, 8 -> cdecl; fopen 0x945b00 "rb", fread 0x945bfe, fclose 0x945a09: cdecl), pela conta de 16 bits
/// do começo; registro curto: a caixa (linha 0x10b9) e entra o que ficou no registro (0x68d0d0).
unsafe extern "C" fn carregar_conferencia_das_pernas() {
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(*mut u8, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(0x68d0d0usize);
    esvaziar(0xae2a48);
    let f = abrir(caminho(0x987da0, 0x9cc258), 0x98f928); // "CM3_TEMP", "check_fixture_legs.tmp", "rb"
    if f == 0 {
        return;
    }
    // ponytail: com a leitura da conta falhando a original usa o lixo da pilha; aqui 0 (arquivo do próprio jogo)
    let mut conta = [0u8; 2];
    ler(conta.as_mut_ptr(), 2, 1, f);
    let conta = i16::from_le_bytes(conta);
    let mut reg = [0u8; 15];
    let mut k: i16 = 0;
    while k < conta {
        if ler(reg.as_mut_ptr(), 0xf, 1, f) != 1 {
            aviso_em(0x9cc208, 0x10b9);
        }
        acrescentar(0xae2a48, reg.as_ptr(), 0xf);
        k = k.wrapping_add(1);
    }
    fechar(f);
}

/// O comparador da carga das conferências: só a original (ponytail: lê o arquivo e aloca os nós da lista; não pode rodar
/// duas vezes).
unsafe extern "C" fn comparar_carregar_conferencia_das_pernas() {
    orig::<unsafe extern "C" fn()>(1517)()
}

// ---------- 266a leva: o calendário: a nova data de um jogo, a entrada do jogo no calendário e a troca do ano do calendário (fix_man.cpp) ----------

unsafe fn cqc4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc4_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

type Cqc4Data = [u8; 8];

/// `a` depois de `b` como a original compara as datas: o ano (+2) sem sinal, depois o dia (+0) com sinal.
unsafe fn cqc4_depois(a: usize, b: usize) -> bool {
    let (ya, yb) = (cqc4_w(a + 2), cqc4_w(b + 2));
    ya > yb || (ya == yb && (cqc4_w(a) as i16) > (cqc4_w(b) as i16))
}

/// As operações de data (thiscall; as de +n devolvem a data estática delas ou `ret`): 0x54a110 a data padrão,
/// 0x549ef0 a data (dia, ano) (push ano; push dia), 0x417c10 a atribuição (push fonte; push ret), 0x54c5b0 + dias,
/// 0x54c8e0 − dias, 0x54c770 + semanas (push n; push ret), 0x54a2d0 o dia da semana alvo (push dest; push alvo),
/// 0x54a3f0 o dia da semana (sem pushes).
unsafe fn cqc4_padrao(d: &mut Cqc4Data) {
    let f: unsafe extern "thiscall" fn(*mut Cqc4Data) -> usize = core::mem::transmute(0x54a110usize);
    f(d);
}
unsafe fn cqc4_nova(d: &mut Cqc4Data, dia: u16, ano: u16) -> usize {
    let f: unsafe extern "thiscall" fn(*mut Cqc4Data, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    f(d, dia as u32, ano as u32)
}
unsafe fn cqc4_atribuir(d: &mut Cqc4Data, fonte: usize) {
    let f: unsafe extern "thiscall" fn(*mut Cqc4Data, *mut Cqc4Data, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut ret = [0u8; 8];
    f(d, &mut ret, fonte);
}
unsafe fn cqc4_op(rotina: usize, d: usize, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut Cqc4Data, u32) -> usize = core::mem::transmute(rotina);
    let mut ret = [0u8; 8];
    // ponytail: a original passa um ret na pilha dela; as três escrevem nele ou na estática e devolvem o ponteiro,
    // que é lido logo depois (antes da próxima chamada), como na original
    let r = f(d, &mut ret, n);
    if r == ret.as_ptr() as usize {
        CQC4_RET = ret;
        return core::ptr::addr_of!(CQC4_RET) as usize;
    }
    r
}
static mut CQC4_RET: Cqc4Data = [0; 8];
unsafe fn cqc4_semana(d: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    f(d)
}
unsafe fn cqc4_alvo(d: usize, alvo: u32, dest: &mut Cqc4Data) {
    let f: unsafe extern "thiscall" fn(usize, u32, *mut Cqc4Data) -> u32 = core::mem::transmute(0x54a2d0usize);
    f(d, alvo, dest);
}

/// A faixa de horário (0x5af2f0: mov ecx, gerenciador; push dia; push comp -> thiscall, ret 8).
unsafe fn cqc4_faixa(this: usize, comp: usize, dia: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x5af2f0usize);
    f(this, comp, dia)
}

/// Os conflitos do jogo (0x5ad780: mov ecx, gerenciador; push marca; push &gravidade; push jogo -> thiscall, ret
/// 0xc): a lista (ou 0) e a gravidade.
unsafe fn cqc4_conflitos(this: usize, jogo: usize, dois: u32) -> (usize, i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(0x5ad780usize);
    let mut g: i32 = 0;
    let l = f(this, jogo, &mut g, dois);
    (l, g)
}

/// Destrói e libera a lista (0x68d040: mov ecx, lista -> thiscall; 0x944c94: push; add esp, 4 -> cdecl).
unsafe fn cqc4_liberar(l: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    destruir(l);
    liberar(l);
}

/// { 0x5ab510 } (thiscall, ret 4, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::find_new_fixture_date(jogo)`,
/// pela entrada no calendário (0x5aa680), pelos conflitos do mesmo dia (0x5ae380) e pela remarcação (0x5ae9d0). Sem
/// jogo: a caixa (a original). Pela casa das unidades da rodada (+0x30, 0x4b0850: push; add esp, 4 -> cdecl) 3 ou 4,
/// o registro de 15 bytes do jogo na lista de conferência das pernas [0xae2a48] (0x68d0d0). A janela pelo tipo (+0x3a):
/// 2, até a data de +0x24/+0x26 (sem ela, a caixa e 2 semanas), começando 4 semanas depois (se o fim passa de 20
/// semanas) ou 3 dias depois; 4 e 6, de 3 dias a 2 semanas; 5, de 0 a 1 semana; 7, 8 e 10, de 3 dias antes a 1 semana;
/// os outros, nenhuma. Em duas passadas (a segunda olhando os dois dias antes), pelos dias da semana na ordem 2, 1, 5,
/// 6, 0, 3, 4, de 4 em 4 dias da janela: o dia alvo (0x54a2d0) a partir de 2 dias depois de hoje e dentro da janela
/// vira a data (+0x28, +0x2a) e a faixa (+0x3f, 0x5af2f0); sem conflito (0x5ad780), com gravidade 1 ou abaixo do tipo,
/// fica. Sem achar: 2 dias depois da data do jogo (ou de hoje, se já passou).
unsafe extern "thiscall" fn nova_data_do_jogo(this: usize, jogo: usize) {
    if jogo == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(1518)(this, jogo); // a caixa
    }
    let (mut s, mut e, mut j, mut h, mut k) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    cqc4_padrao(&mut s);
    cqc4_padrao(&mut e);
    cqc4_padrao(&mut j);
    cqc4_padrao(&mut h);
    cqc4_padrao(&mut k);
    let dias: [i16; 9] = [2, 1, -1, 5, 6, -1, 0, 3, 4]; // cqc4: a ordem dos dias da semana
    let mut tmp = [0u8; 8];
    let r = cqc4_nova(&mut tmp, cqc4_w(jogo + 0x2a), cqc4_w(jogo + 0x28));
    cqc4_atribuir(&mut j, r);
    let unidades: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    let rodada = cqc4_w(jogo + 0x30) as i16 as i32;
    if unidades(rodada) == 3 || unidades(rodada) == 4 {
        let mut reg = [0u8; 15];
        reg[0..4].copy_from_slice(&core::slice::from_raw_parts(jogo as *const u8, 4));
        reg[4] = cqc4_b(jogo + 0x42);
        reg[5..7].copy_from_slice(&cqc4_w(jogo + 0x34).to_le_bytes());
        reg[7..11].copy_from_slice(&core::slice::from_raw_parts((jogo + 0xc) as *const u8, 4));
        reg[11..15].copy_from_slice(&core::slice::from_raw_parts((jogo + 0x10) as *const u8, 4));
        let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(0x68d0d0usize);
        acrescentar(0xae2a48, reg.as_ptr(), 0xf);
    }
    let pj = j.as_ptr() as usize;
    match (cqc4_b(jogo + 0x3a) as i8 as i32).wrapping_sub(2) {
        0 => {
            if cqc4_w(jogo + 0x24) != 0 {
                let mut t = [0u8; 8];
                let r = cqc4_nova(&mut t, cqc4_w(jogo + 0x26), cqc4_w(jogo + 0x24));
                cqc4_atribuir(&mut e, r);
            } else {
                aviso_em(0x9cc208, 0x4f9); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
                let r = cqc4_op(0x54c770, pj, 2);
                cqc4_atribuir(&mut e, r);
            }
            let fim = e;
            let r = cqc4_op(0x54c770, pj, 0x14);
            let r = if cqc4_depois(fim.as_ptr() as usize, r) { cqc4_op(0x54c770, pj, 4) } else { cqc4_op(0x54c5b0, pj, 3) };
            cqc4_atribuir(&mut s, r);
        }
        2 | 4 => {
            let r = cqc4_op(0x54c770, pj, 2);
            cqc4_atribuir(&mut e, r);
            let r = cqc4_op(0x54c5b0, pj, 3);
            cqc4_atribuir(&mut s, r);
        }
        3 => {
            let r = cqc4_op(0x54c770, pj, 1);
            cqc4_atribuir(&mut e, r);
            cqc4_atribuir(&mut s, pj);
        }
        5 | 6 | 8 => {
            let r = cqc4_op(0x54c770, pj, 1);
            cqc4_atribuir(&mut e, r);
            let r = cqc4_op(0x54c8e0, pj, 3);
            cqc4_atribuir(&mut s, r);
        }
        _ => {}
    }
    let (ps, pe, ph, pk) = (s.as_ptr() as usize, e.as_ptr() as usize, h.as_ptr() as usize, k.as_ptr() as usize);
    for passada in 0..2u8 {
        for i in 0..3usize {
            for jj in 0..3usize {
                let alvo = dias[i * 3 + jj];
                if alvo == -1 {
                    continue;
                }
                cqc4_atribuir(&mut k, ps);
                while cqc4_depois(pe, pk) {
                    cqc4_alvo(pk, alvo as u16 as u32, &mut h);
                    let t = cqc4_op(0x54c5b0, 0xae2c90, 2);
                    let t = [cqc4_d(t), cqc4_d(t + 4)];
                    let pt = t.as_ptr() as usize;
                    if !cqc4_depois(pt, ph) && !cqc4_depois(ph, pe) {
                        crate::comparacao::ramo("cqc_4 nova data");
                        cqc4_pw(jogo + 0x28, cqc4_w(ph + 2));
                        cqc4_pw(jogo + 0x2a, cqc4_w(ph));
                        *((jogo + 0x3f) as *mut u8) = cqc4_faixa(this, cqc4_d(jogo + 0x14), alvo as u16 as u32);
                        let (l, g) = cqc4_conflitos(this, jogo, (passada == 1) as u32);
                        if l == 0 {
                            if g == 0 {
                                return;
                            }
                        } else {
                            cqc4_liberar(l);
                        }
                        if g == 1 || (cqc4_b(jogo + 0x3a) as i8 as i32) > g {
                            return;
                        }
                    }
                    let r = cqc4_op(0x54c5b0, pk, 4); // cqc4: o passo da busca
                    cqc4_atribuir(&mut k, r);
                }
            }
        }
    }
    if cqc4_depois(0xae2c90, pj) {
        cqc4_atribuir(&mut j, 0xae2c90);
    }
    let r = cqc4_op(0x54c5b0, pj, 2);
    cqc4_atribuir(&mut j, r);
    cqc4_pw(jogo + 0x2a, cqc4_w(pj));
    cqc4_pw(jogo + 0x28, cqc4_w(pj + 2));
}

/// O comparador da nova data: lado a lado no jogo (0x4f bytes); os conflitos de dentro (0x5ad780) criam e destroem a
/// lista deles. Só a original quando o jogo entra na lista de conferência das pernas (aloca o nó).
unsafe extern "thiscall" fn comparar_nova_data_do_jogo(this: usize, jogo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1518);
    let unidades: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    // a caixa da janela sem data (+0x24, tipo 2) apareceria duas vezes: só a original
    if jogo == 0 || matches!(unidades(cqc4_w(jogo + 0x30) as i16 as i32), 3 | 4) || (cqc4_b(jogo + 0x3a) == 2 && cqc4_w(jogo + 0x24) == 0) {
        return o(this, jogo);
    }
    let reg = [(jogo, 0x4f)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo), || nova_data_do_jogo(this, jogo));
    PLACAR.registrar(igual, || format!("nova_data_do_jogo(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// { 0x5aa680 } (thiscall, ret 8, fix_man.cpp; quadro SEH das datas e do gerenciador novo): `FIXTURE_MANAGER::add_fixture(jogo,
/// zerar)`, pelas copas, ligas, amistosos e remarcações (20 chamadores). `zerar` zera a conta das remarcações em
/// cadeia (palavra [0xae2a40]). Acerta a fase (0x5af180: push; add esp, 4 -> cdecl), +0x2e = −1; jogo de data já
/// passada e não jogado: nova data (0x5ab510). Jogo da liga do mandante (+0x57 ou +0x60) sem rodada (+0x30, +0x32) nem
/// fase (+0x3b), uma vez (bit 0x800 de +0x4d): no sábado ou domingo, vai ao dia preferido do mandante (+0x72: 4, 5 ou
/// 6) se lá não houver conflito. Com menos de 10 remarcações em cadeia, os conflitos (0x5ad780): gravidade 0x14, o
/// conflito do mesmo dia (0x5ae380) e 1; copa ou tipo 9 recusado pela competição (+0x64): 0; gravidade de pelo menos o
/// tipo: 0 se a seleção do jogo tem convocação ([0xb74434], 0x7e6f20), senão nova data e entra de novo; senão entra
/// (0x5ab230) e os jogos do conflito saem: os amistosos (0x5c6710/0x5c6570 em [0xae2c74]), o tipo 9 (0x5abc30), os
/// outros remarcados (0x5abc30, 0x5ab510, 0x5aa680) com a notícia (0x5aac70 em [0xae2a58]). No ano do bloco (+0x1130):
/// a nação-chave (0x48dd70), a ordem (0x5ab3c0), os jogos de seleções (0x7e6720 em [0xb74434], 0x76bab0 em [0xb74318]),
/// os vizinhos (0x5ae9d0, chave 3 ou 5); se não entrou, na lista do dia, fatia e chave (0x68d0d0). Fora do ano, no
/// bloco do outro ano (+0x112c: 0x5aa2d0 para o ano, ou novo de 0x1184 bytes, 0x5a9cf0; sem memória, a caixa e 0).
unsafe extern "thiscall" fn jogo_no_calendario(this: usize, jogo: usize, zerar: u32) -> u32 {
    let fase: unsafe extern "C" fn(usize) = core::mem::transmute(0x5af180usize);
    let nova_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab510usize);
    let entrar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let contagem = 0xae2a40 as *mut i16;
    let mut entrou = false;
    if zerar != 0 {
        *contagem = 0;
    }
    fase(jogo);
    cqc4_pw(jogo + 0x2e, 0xffff);
    let hoje = [cqc4_d(0xae2c90), cqc4_d(0xae2c94)];
    let mut dj = [0u8; 8];
    let r = cqc4_nova(&mut dj, cqc4_w(jogo + 0x2a), cqc4_w(jogo + 0x28));
    if !cqc4_depois(r, hoje.as_ptr() as usize) && cqc4_b(jogo + 0x43) == 0xff {
        nova_data(this, jogo);
    }
    let marcas = cqc4_w(jogo + 0x4d);
    let casa = cqc4_d(jogo + 0x1c);
    let comp = cqc4_d(jogo + 0x14);
    if marcas & 0x800 == 0
        && (comp == cqc4_d(casa + 0x57) || comp == cqc4_d(casa + 0x60))
        && cqc4_w(jogo + 0x32) == 0
        && cqc4_w(jogo + 0x30) == 0
        && cqc4_b(jogo + 0x3b) == 0xff
    {
        cqc4_pw(jogo + 0x4d, marcas | 0x800);
        let preferido = cqc4_b(casa + 0x72);
        if preferido != 0xff && (preferido == 5 || preferido == 6 || preferido == 4) {
            let mut d = [0u8; 8];
            cqc4_nova(&mut d, cqc4_w(jogo + 0x2a), cqc4_w(jogo + 0x28));
            let pd = d.as_ptr() as usize;
            let dia = cqc4_semana(pd) as u8;
            if preferido != dia && (dia == 5 || dia == 6) {
                let mut t = [0u8; 8];
                cqc4_padrao(&mut t);
                crate::comparacao::ramo("cqc_4 dia preferido do mandante");
                cqc4_alvo(pd, preferido as i8 as i16 as u16 as u32, &mut t); // cqc4: o dia preferido do mandante
                let pt = t.as_ptr() as usize;
                cqc4_pw(jogo + 0x28, cqc4_w(pt + 2));
                cqc4_pw(jogo + 0x2a, cqc4_w(pt));
                *((jogo + 0x3f) as *mut u8) = cqc4_faixa(this, cqc4_d(jogo + 0x14), cqc4_semana(pt) as u32);
                let (l, g) = cqc4_conflitos(this, jogo, 0);
                if l != 0 || g != 0 {
                    cqc4_pw(jogo + 0x2a, cqc4_w(pd));
                    cqc4_pw(jogo + 0x28, cqc4_w(pd + 2));
                    *((jogo + 0x3f) as *mut u8) = cqc4_faixa(this, cqc4_d(jogo + 0x14), cqc4_semana(pd) as u32);
                    if l != 0 {
                        cqc4_liberar(l);
                    }
                }
            }
        }
    }
    if *contagem < 10 {
        let (l, g) = cqc4_conflitos(this, jogo, 0);
        if l != 0 || g != 0 {
            *contagem = (*contagem).wrapping_add(1);
            if g == 0x14 {
                crate::comparacao::ramo("cqc_4 mesmo dia");
                let mesmo_dia: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5ae380usize);
                mesmo_dia(this, jogo, l); // cqc4: o conflito do mesmo dia
                if l != 0 {
                    cqc4_liberar(l);
                }
                return 1;
            }
            let tipo = cqc4_b(jogo + 0x3a);
            if (tipo == 1 || tipo == 9) && cqc4_d(jogo + 0x14) != 0 && cqc4_d(0xadadfc) != 0 {
                let c = cqc4_d(cqc4_d(0xadadfc).wrapping_add((cqc4_d(cqc4_d(jogo + 0x14)) as usize).wrapping_mul(4)));
                if c != 0 {
                    let aceita: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cqc4_d(cqc4_d(c) + 0x64));
                    if aceita(c, jogo) == 0 {
                        if l != 0 {
                            cqc4_liberar(l);
                        }
                        return 0;
                    }
                }
            }
            // 0x7e6f20: mov ecx, [0xb74434]; push comp -> thiscall, ret 4
            let convocada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
            if g >= cqc4_b(jogo + 0x3a) as i8 as i32 {
                if l != 0 {
                    cqc4_liberar(l);
                }
                if convocada(cqc4_d(0xb74434), cqc4_d(jogo + 0x14)) != 0 {
                    return 0;
                }
                nova_data(this, jogo);
                return (entrar(this, jogo, 0) != 0) as u32;
            }
            let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab230usize);
            guardar(this, jogo);
            entrou = true;
            if l != 0 {
                let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
                let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
                let tirar_amistoso: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c6570usize);
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
                let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x5aac70usize);
                core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc4_d(l + 2));
                let mut x = proximo(l);
                while x != 0 {
                    if amistoso(cqc4_d(0xae2c74), cqc4_d(x + 0x14)) != 0 {
                        tirar_amistoso(cqc4_d(0xae2c74), x, 0);
                    } else if cqc4_b(x + 0x3a) == 9 {
                        tirar(this, x);
                    } else {
                        if convocada(cqc4_d(0xb74434), cqc4_d(jogo + 0x14)) != 0 {
                            return 0; // como na original, sem liberar a lista
                        }
                        let mut copia = [0u8; 0x4f];
                        core::ptr::copy_nonoverlapping(x as *const u8, copia.as_mut_ptr(), 0x4f);
                        let pc = copia.as_mut_ptr() as usize;
                        if tirar(this, x) != 0 {
                            nova_data(this, pc);
                            entrar(this, pc, 0);
                        }
                        noticia(cqc4_d(0xae2a58), pc, jogo, 0);
                    }
                    x = proximo(l);
                }
                cqc4_liberar(l);
            }
        }
    }
    let ano = cqc4_w(jogo + 0x28);
    let mut chave: i8 = 0;
    if ano == cqc4_w(this + 0x1130) {
        let chave_do_jogo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x48dd70usize);
        chave = chave_do_jogo(cqc4_d(jogo + 0x14), jogo) as i8;
        let ordem: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab3c0usize);
        ordem(this, jogo);
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(cqc4_d(jogo + 0x1c)) != 0 && selecao(cqc4_d(jogo + 0x20)) != 0 && cqc4_b(jogo + 0x43) == 0xff {
            let c = cqc4_d(jogo + 0x14);
            let id = cqc4_d(c) as i32;
            if id != cqc4_d(0x9cf794) as i32 && id != cqc4_d(0x9cf798) as i32 && cqc4_b(jogo + 0x4e) & 3 == 0 {
                let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
                let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
                if especial(cqc4_d(0xb74318), c) == 0 || amistoso(cqc4_d(0xae2c74), cqc4_d(jogo + 0x14)) != 0 {
                    let marcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e6720usize);
                    marcar(cqc4_d(0xb74434), jogo);
                }
            }
            let jogo_de_selecoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76bab0usize);
            jogo_de_selecoes(cqc4_d(0xb74318), jogo);
        }
        if chave == 3 || chave == 5 {
            let vizinhos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ae9d0usize);
            vizinhos(this, jogo);
        }
    }
    if entrou {
        return 1;
    }
    let ano = cqc4_w(jogo + 0x28);
    if ano == cqc4_w(this + 0x1130) {
        let dia = cqc4_w(jogo + 0x2a) as usize;
        let fatia = cqc4_b(jogo + 0x3f) as usize;
        let linha = cqc4_d(this + (dia * 3 + fatia) * 4);
        let lista = cqc4_d(linha.wrapping_add((chave as isize as usize).wrapping_mul(4)));
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
        return (acrescentar(lista, jogo, 0x4f) != 0) as u32;
    }
    let mut outro = cqc4_d(this + 0x112c);
    if outro != 0 {
        if cqc4_w(outro + 0x1130) != ano {
            let ano_do_bloco: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
            ano_do_bloco(outro, ano as u32);
        }
    } else {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let construir: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x5a9cf0usize);
        let b = novo(0x1184);
        outro = if b != 0 { construir(b, ano as u32, this, 0) } else { 0 };
        core::ptr::write_unaligned((this + 0x112c) as *mut usize, outro);
        if outro == 0 {
            aviso_em(0x9cc208, 0x2e6);
            return 0;
        }
    }
    entrar(cqc4_d(this + 0x112c), jogo, 0);
    1
}

/// O comparador da entrada no calendário: só a original (ponytail: grava nas listas do calendário, aloca nós e o
/// bloco do outro ano, remarca e entrega notícias; não pode rodar duas vezes). A cópia segue no modo Rust, com os
/// contadores.
unsafe extern "thiscall" fn comparar_jogo_no_calendario(this: usize, jogo: usize, zerar: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1519)(this, jogo, zerar)
}

/// { 0x5aa2d0 } (thiscall, ret 4, fix_man.cpp; quadro SEH dos blocos novos): `FIXTURE_MANAGER::set_year(ano)`, pela entrada
/// no calendário (0x5aa680, o bloco do outro ano), pela guarda e pela retirada de jogos e pelos amistosos. O mesmo ano
/// (palavra, +0x1130): 1. Grava os jogos do ano (0x5ac060: ecx = bloco; push 0 -> thiscall; falhando, a caixa e 0); se
/// o outro bloco (+0x112c) está no ano pedido, ele vai para o ano vizinho do deste (0x5aa2d0 nele); destrói e libera as
/// listas dos 366 dias × 3 fatias × nações-chave ([0xb64738], byte com sinal; 0x68d040, 0x944c94); o outro bloco no
/// ano pedido, se este é o principal (+0x1183), é destruído e liberado (0x5aa0d0: ecx = bloco -> thiscall; 0x944c94); o
/// ano novo, bissexto em +0x1132 (0x54cfe0: push ano; add esp, 4 -> cdecl), as listas novas (new 0xe: 0x944cf1, push;
/// add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e a carga dos jogos do ano
/// (0x5ac3b0: ecx = bloco -> thiscall). Devolve 1.
unsafe extern "thiscall" fn ano_do_calendario(this: usize, ano: u32) -> u32 {
    let a = ano as u16;
    if a == cqc4_w(this + 0x1130) {
        return 1;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5ac060usize);
    if gravar(this, 0) == 0 {
        aviso_em(0x9cc208, 0x126);
        return 0;
    }
    let outro = cqc4_d(this + 0x112c);
    if outro != 0 {
        let oa = cqc4_w(outro + 0x1130);
        if oa == a {
            let vizinho = if cqc4_w(this + 0x1130) as u32 == oa as u32 + 1 { oa.wrapping_sub(1) } else { oa.wrapping_add(1) };
            let mudar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
            mudar(outro, vizinho as u32);
        }
    }
    let n = || cqc4_b(0xb64738) as i8;
    for dia in 0..0x16eusize {
        if n() > 0 {
            let base = this + 8 + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                for p in [base - 8, base - 4, base] {
                    let slot = cqc4_d(p).wrapping_add((k as i32 as usize).wrapping_mul(4));
                    let l = cqc4_d(slot);
                    if l != 0 {
                        cqc4_liberar(l);
                        core::ptr::write_unaligned(cqc4_d(p).wrapping_add((k as i32 as usize).wrapping_mul(4)) as *mut usize, 0);
                    }
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let outro = cqc4_d(this + 0x112c);
    if outro != 0 && cqc4_w(outro + 0x1130) == a && cqc4_b(this + 0x1183) != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5aa0d0usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(outro);
        liberar(outro);
        core::ptr::write_unaligned((this + 0x112c) as *mut usize, 0);
    }
    cqc4_pw(this + 0x1130, a);
    let bissexto: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x54cfe0usize);
    *((this + 0x1132) as *mut u8) = (bissexto(ano) != 0) as u8;
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    crate::comparacao::ramo("cqc_4 ano novo do calendario");
    for dia in 0..0x16eusize {
        if n() > 0 {
            let e = this + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                let desloc = (k as i32 as usize).wrapping_mul(4);
                for j in 0..3usize {
                    let b = novo(0xe);
                    let l = if b != 0 { construir(b) } else { 0 };
                    core::ptr::write_unaligned(cqc4_d(e + j * 4).wrapping_add(desloc) as *mut usize, l);
                }
                if (0..3usize).any(|j| cqc4_d(cqc4_d(e + j * 4).wrapping_add(desloc)) == 0) {
                    aviso_em(0x9cc208, 0x170);
                    return 0;
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let carregar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5ac3b0usize);
    carregar(this);
    1
}

/// O comparador do ano do calendário: só a original (ponytail: grava e lê os arquivos dos jogos do ano e refaz todas
/// as listas; não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_ano_do_calendario(this: usize, ano: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1520)(this, ano)
}
// ---------- 266a leva: a copa: os jogos do confronto, a notícia do confronto pendente e os avisos das janelas de transferência (Cup.cpp, transfer_manager.cpp) ----------

unsafe fn cqc5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqc5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4).
unsafe fn cqc5_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc5_d(0xb7431c), clube) != 0
}

/// A data da partida (+0x2a dia, +0x28 ano, pelos dwords que a original empurra: 0x549ef0 push ano; push dia; lea ecx,
/// local -> thiscall, ret 8) que não estiver depois de hoje ([0xae2c90]: ano sem sinal, dia com sinal) passa a amanhã
/// (0x54c5b0: push 1; push saída; mov ecx, 0xae2c90 -> thiscall(data, saída, n), ret 8); e a partida vai ao calendário
/// (0x5aa680: push 1; push &partida; ecx = [0xae2a58] -> thiscall, ret 8).
unsafe fn cqc5_ao_calendario(p: usize) {
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let (mut t, mut saida) = ([0u8; 8], [0u8; 8]);
    let f = nova(t.as_mut_ptr() as usize, cqc5_d(p + 0x2a) as u32, cqc5_d(p + 0x28) as u32);
    let (hd, ha) = (cqc5_w(0xae2c90), cqc5_w(0xae2c92) as u16);
    let fa = cqc5_w(f + 2) as u16;
    let antes = ha < fa || (ha == fa && hd < cqc5_w(f));
    if !antes {
        let s = saida.as_mut_ptr() as usize;
        mais(0xae2c90, s, 1);
        cqc5_pw(p + 0x2a, cqc5_w(s) as u16);
        cqc5_pw(p + 0x28, cqc5_w(s + 2) as u16);
    }
    juntar(cqc5_d(0xae2a58), p, 1);
}

/// { 0x51cb90 } (thiscall, ret 0xc, Cup.cpp, quadro SEH sem destrutor no caminho normal: as datas e a notícia locais):
/// `Cup::create_tie_fixtures(rodada, confronto, adiar)`, da montagem da rodada (0x51c800) e da tela da competição.
/// Confronto −1: o primeiro da rodada com a marca +0x34 = 1 (adiar) ou 2 (criar); nenhum: 0. Adiar: marca 2. Criar:
/// marca 3; com time faltando (+0/+2 = −1), a notícia do confronto pendente (0x522950: push confronto; push rodada;
/// mov ecx, esi -> thiscall, ret 8). Senão a partida numa local de 0x4f bytes: a competição (+0x14) e o id (+0, −1
/// sem), −1 em +8, o ano da copa (+0x40) em +0x2c, +0x3a em +0x32, da rodada (0x68 bytes em +0xa3) o código +7 (+3
/// na de duas mãos, +0x21 = 2) em +0x30, +9 em +0x34, +0xb em +0x36, as regras +0xd em +0x38 (sem mão de volta e com
/// 0x800: 0x1000 se o nível (+4 do time) do mandante não é menor que o do visitante, senão 0x800), +0x17 em +0x3a, a
/// liga (+0x44) em +0x42, as marcas (0xff, 1, 0), o virtual +0x7c da copa (push &partida; mov ecx, esi), o bit 0x400
/// de +0x4d (sem mão de volta e +0x20 > 0), os clubes (+0x1c, +0x20) e ids (+0xc, +0x10; −1 sem clube), a data (dia
/// +3, ano +5 + o da copa), a fatia +6, a data da rodada seguinte (+0x6b/+0x6d; na última, 0), o estádio (0x85c260:
/// push +0x24 da copa; push &partida; push +0x13; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc) e o calendário
/// (cqc5_ao_calendario). Com técnico humano num dos dois e sem o bit 0x100 em +0xd, a notícia 0x14 aos dois. Na de duas
/// mãos, com a copa ainda em [0xadadfc]: a volta +0x22 dias depois (0x549ef0, 0x54c5b0, 0x417c10), o código +4, a
/// fatia pelo dia da semana (0x54a3f0: sábado ou domingo 1, senão 2), mando invertido, o estádio e o calendário.
/// Devolve 1.
unsafe extern "thiscall" fn copa_criar_jogos_do_confronto(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let id = cqc5_d(cqc5_d(this + 4));
    let ri = rodada as i16 as i32;
    let (mut k, mut adiar) = (confronto, adiar);
    if confronto as i16 == -1 {
        let n = cqc5_w(cpq11_rodada(this, ri) + 0x1a);
        let mut i: u32 = 0;
        loop {
            if (i as i16) >= n {
                return 0;
            }
            let m = cqc5_b(cpq11_confronto(this, ri, i as i16 as i32) + 0x34);
            if m == 1 {
                adiar = 1;
                break;
            }
            if m == 2 {
                adiar = 0;
                break;
            }
            i = i.wrapping_add(1);
        }
        k = i;
        if k as i16 == -1 {
            return 0;
        }
    }
    let t = cpq11_confronto(this, ri, k as i16 as i32);
    if adiar != 0 {
        *((t + 0x34) as *mut u8) = 2;
        return 1;
    }
    *((t + 0x34) as *mut u8) = 3;
    if cqc5_w(t) == -1 || cqc5_w(t + 2) == -1 {
        let pendente: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x522950usize);
        pendente(this, rodada, k);
        return 1;
    }
    let mut buf = [0u8; 0x50];
    let p = buf.as_mut_ptr() as usize;
    let comp = cqc5_d(this + 4);
    if comp != 0 {
        cqc5_pd(p + 0x14, comp);
        cqc5_pd(p, cqc5_d(comp));
    } else {
        cqc5_pd(p, usize::MAX);
        cqc5_pd(p + 0x14, 0);
    }
    cqc5_pw(p + 0x2c, cqc5_w(this + 0x40) as u16);
    cqc5_pw(p + 0x32, cqc5_w(this + 0x3a) as u16);
    let e = cpq11_rodada(this, ri);
    let mut codigo = cqc5_w(e + 7);
    if cqc5_b(e + 0x21) == 2 {
        codigo = codigo.wrapping_add(3);
    }
    cqc5_pw(p + 0x30, codigo as u16);
    cqc5_pw(p + 0x34, cqc5_w(e + 9) as u16);
    cqc5_pb(p + 0x3a, cqc5_b(e + 0x17));
    cqc5_pw(p + 0x36, cqc5_w(e + 0xb) as u16);
    // ponytail: a original faz and 0xf4ff na palavra +0x4d de um local sem valor (o +0x4d e os bits 2, 4..7 de
    // +0x4e ficam com a sobra de pilha) e não grava +4, +0x18, +0x2e, +0x2f, +0x3c antes do estádio: aqui, zero
    cqc5_pw(p + 0x38, cqc5_w(e + 0xd) as u16);
    for x in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a] {
        cqc5_pb(p + x, 0xff);
    }
    cqc5_pd(p + 8, usize::MAX);
    cqc5_pb(p + 0x42, cqc5_b(this + 0x44));
    cqc5_pb(p + 0x3d, 0);
    cqc5_pb(p + 0x3e, 0);
    cqc5_pb(p + 0x3b, 0xff);
    cqc5_pb(p + 0x40, 1);
    cqc5_pb(p + 0x41, 1);
    if (cqc5_b(e + 0x21) as i8) < 2 && cqc5_w(p + 0x38) as u16 & 0x800 != 0 {
        let nc = cqc5_b(cpq11_time(this, cqc5_w(t)) + 4) as i8;
        let nf = cqc5_b(cpq11_time(this, cqc5_w(t + 2)) + 4) as i8;
        cqc5_pw(p + 0x38, if nc >= nf { 0x1000 } else { 0x800 });
    }
    let virtual_7c: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqc5_d(cqc5_d(this) + 0x7c));
    virtual_7c(this, p);
    let e = cpq11_rodada(this, ri);
    if (cqc5_b(e + 0x21) as i8) < 2 && (cqc5_b(e + 0x20) as i8) > 0 {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) | 4);
    } else {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & !4);
    }
    let t = cpq11_confronto(this, ri, k as i16 as i32);
    let (lc, lf) = (cqc5_w(t), cqc5_w(t + 2)); // cqc5: os lados da ida
    let casa = cqc5_d(cpq11_time(this, lc));
    cqc5_pd(p + 0x1c, casa);
    cqc5_pd(p + 0xc, if casa != 0 { cqc5_d(casa) } else { usize::MAX });
    let fora = cqc5_d(cpq11_time(this, lf));
    cqc5_pd(p + 0x20, fora);
    cqc5_pd(p + 0x10, if fora != 0 { cqc5_d(fora) } else { usize::MAX });
    let ano_copa = cqc5_w(this + 0x40);
    cqc5_pw(p + 0x28, (cqc5_b(e + 5) as i8 as i16).wrapping_add(ano_copa) as u16);
    cqc5_pw(p + 0x2a, cqc5_w(e + 3) as u16); // cqc5: o dia da ida
    cqc5_pb(p + 0x3f, cqc5_b(e + 6));
    if ri < cqc5_w(this + 0x3c) as i32 - 1 {
        cqc5_pw(p + 0x26, cqc5_w(e + 0x6b) as u16);
        cqc5_pw(p + 0x24, (cqc5_b(e + 0x6d) as i8 as i16).wrapping_add(cqc5_w(this + 0x40)) as u16);
    } else {
        cqc5_pw(p + 0x26, 0);
        cqc5_pw(p + 0x24, 0);
    }
    let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
    estadio(cqc5_d(0xdd7ef4), cqc5_d(e + 0x13), p, cqc5_d(this + 0x24));
    crate::comparacao::ramo("cqc_5 jogo do confronto");
    cqc5_ao_calendario(p);
    let (casa, fora) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20));
    if (cqc5_humano(casa) || cqc5_humano(fora)) && cqc5_w(cpq11_rodada(this, ri) + 0xd) as u16 & 0x100 == 0 {
        let mut noticia = [0u32; 0x38];
        let n = noticia.as_mut_ptr() as usize;
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        construir(n, 0x14, 0);
        gravar(n, 0, cqc5_d(cqc5_d(this + 4)) as u32);
        gravar(n, 1, cqc5_w(this + 0x3a) as i32 as u32);
        gravar(n, 2, cqc5_w(cpq11_rodada(this, ri) + 7) as i32 as u32);
        gravar(n, 5, (cqc5_b(cpq11_rodada(this, ri) + 0x21) == 2) as u32);
        gravar(n, 6, cqc5_d(p + 4) as u32);
        gravar(n, 7, cqc5_w(p + 0x2a) as u16 as u32);
        gravar(n, 8, cqc5_w(p + 0x28) as u16 as u32);
        if cqc5_humano(casa) {
            gravar(n, 3, cqc5_d(p + 0xc) as u32);
            gravar(n, 4, cqc5_d(p + 0x10) as u32);
        } else {
            gravar(n, 3, cqc5_d(p + 0x10) as u32);
            gravar(n, 4, cqc5_d(p + 0xc) as u32);
        }
        entregar(cqc5_d(0xb7431c), n, casa);
        entregar(cqc5_d(0xb7431c), n, fora);
    }
    let e = cpq11_rodada(this, ri);
    if cqc5_d(cqc5_d(0xadadfc) + id * 4) == 0 || cqc5_b(e + 0x21) != 2 {
        return 1;
    }
    crate::comparacao::ramo("cqc_5 jogo de volta");
    if (cqc5_b(e + 0x20) as i8) < 1 {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & !4);
    } else {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) | 4);
    }
    cqc5_pw(p + 0x30, cqc5_w(e + 7).wrapping_add(4) as u16);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    let (mut a, mut s, mut c, mut x) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let pa = a.as_mut_ptr() as usize;
    // ponytail: a original empurra o dia e o ano em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as
    // palavras
    nova(pa, cqc5_w(e + 3) as u16 as u32, (cqc5_b(e + 5) as i8 as i16).wrapping_add(cqc5_w(this + 0x40)) as u16 as u32);
    let r = mais(pa, s.as_mut_ptr() as usize, cqc5_b(e + 0x22) as u32);
    copia(pa, c.as_mut_ptr() as usize, r);
    cqc5_pw(p + 0x28, cqc5_w(pa + 2) as u16);
    cqc5_pw(p + 0x2a, cqc5_w(pa) as u16);
    let dia = nova(x.as_mut_ptr() as usize, cqc5_d(pa) as u32, cqc5_d(pa + 2) as u32);
    let sd = semana(dia) as i16;
    cqc5_pb(p + 0x3f, if sd == 5 || sd == 6 { 1 } else { 2 });
    let (c1, c2, i1, i2) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20), cqc5_d(p + 0xc), cqc5_d(p + 0x10)); // cqc5: os lados da volta
    cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & 0xf4);
    cqc5_pd(p + 0x20, c1);
    cqc5_pd(p + 0xc, i2);
    cqc5_pd(p + 0x10, i1);
    cqc5_pd(p + 0x1c, c2);
    let e = cpq11_rodada(this, ri);
    estadio(cqc5_d(0xdd7ef4), cqc5_d(e + 0x13), p, cqc5_d(this + 0x24));
    cqc5_ao_calendario(p);
    1
}

/// (ano, dia) + n dias (o dia vai de 0 a 364, ou 365 no bissexto da 0x549ef0).
fn cqc5_somar((ano, dia): (i32, i32), n: i32) -> (i32, i32) {
    let (mut a, mut d) = (ano, dia + n);
    loop {
        let t = if crate::data::bissexto(a as u16) == 1 { 366 } else { 365 };
        if d < t {
            return (a, d);
        }
        d -= t;
        a += 1;
    }
}

/// Os jogos do clube (id) na competição de hoje até (ano, dia) (0x5acbe0: lista nula, tipo 0xfe e lado 0xff
/// quaisquer), para o comparador. A janela não passa da data dos jogos novos + 7 dias: a busca cria o bloco do ano que
/// faltar (0x5abf10), e o da partida a 0x5aa680 cria de qualquer jeito.
unsafe fn cqc5_jogos(clube: u32, comp: u32, (ano, dia): (i32, i32)) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x5acbe0usize);
    achar(cqc5_d(0xae2a58), cqc5_w(0xae2c90) as u16 as u32, cqc5_w(0xae2c92) as u16 as u32, dia as u32, ano as u32, clube, 0,
        comp, 0xfe, 0xff) & 0xffff
}

/// Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
unsafe extern "thiscall" fn comparar_copa_criar_jogos_do_confronto(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(1521);
    let ri = rodada as i16 as i32;
    let id = cqc5_d(cqc5_d(this + 4)) as u32;
    let (mut k, mut ad) = (confronto as i16 as i32, adiar);
    if k == -1 {
        let n = cqc5_w(cpq11_rodada(this, ri) + 0x1a) as i32;
        k = -1;
        for i in 0..n.max(0) {
            let m = cqc5_b(cpq11_confronto(this, ri, i) + 0x34);
            if m == 1 || m == 2 {
                k = i;
                ad = (m == 1) as u32;
                break;
            }
        }
        if k == -1 {
            let r = o(this, rodada, confronto, adiar);
            PLACAR.registrar(r == 0, || format!("copa_criar_jogos_do_confronto(rodada {}) sem confronto marcado: {}", ri, r));
            return r;
        }
    }
    let t = cpq11_confronto(this, ri, k);
    let completo = cqc5_w(t) != -1 && cqc5_w(t + 2) != -1;
    let e = cpq11_rodada(this, ri);
    let duas = cqc5_b(e + 0x21) == 2;
    let casa = if completo { cqc5_d(cpq11_time(this, cqc5_w(t))) } else { 0 };
    let ic = if casa != 0 { cqc5_d(casa) as u32 } else { u32::MAX };
    let conferir = ad == 0 && completo && ic != u32::MAX;
    // o fim da janela: a data da rodada (+ os dias até a volta) ou amanhã, o que vier depois, + 7 dias
    let ida = (cqc5_b(e + 5) as i8 as i32 + cqc5_w(this + 0x40) as i32, cqc5_w(e + 3) as i32);
    let ultimo = if duas { cqc5_somar(ida, cqc5_b(e + 0x22) as i32) } else { ida };
    let amanha = cqc5_somar((cqc5_w(0xae2c92) as u16 as i32, cqc5_w(0xae2c90) as i32), 1);
    let fim = cqc5_somar(ultimo.max(amanha), 7);
    let antes = if conferir { cqc5_jogos(ic, id, fim) } else { 0 };
    let r = o(this, rodada, confronto, adiar);
    let marca = cqc5_b(t + 0x34);
    let mut ok = r == 1 && marca == if ad != 0 { 2 } else { 3 };
    let mut esperado = 0;
    let mut depois = 0;
    if conferir && cqc5_d(cqc5_d(0xadadfc) + id as usize * 4) != 0 {
        esperado = 1 + duas as u32;
        depois = cqc5_jogos(ic, id, fim);
        ok = ok && depois == antes + esperado;
    }
    PLACAR.registrar(ok, || format!("copa_criar_jogos_do_confronto(rodada {}, confronto {}, adiar {}) devolveu {} marca {} jogos {} -> {} (esperado +{})",
        ri, k, ad, r, marca, antes, depois, esperado));
    r
}

/// { 0x522950 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: a notícia local):
/// `Cup::post_draw_news(rodada, confronto)`, só da 0x51cb90. Com os dois times definidos, ou rodada (0x68 bytes em
/// +0xa3) com menos de 2 confrontos (+0x1a): nada. Cada lado sem time (+0/+2 = −1) vem do confronto de origem (rodada
/// +0xe/+0x10, confronto +0xa/+0xc; sem origem ou com time faltando lá: nada): os dois clubes possíveis; com time, o
/// clube dele (+0xa7, 6 bytes). A notícia 0x26: 0 = o id da copa, 1 = +0x3a, 2 = o código da rodada (+7), 3/4 = o
/// mandante e −1 ou os dois possíveis, 5/6 = o visitante e −1 ou os dois possíveis; aos técnicos de cada clube dela
/// (0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; os possíveis só se não nulos).
unsafe extern "thiscall" fn copa_noticia_do_confronto_pendente(this: usize, rodada: u32, confronto: u32) {
    let ri = rodada as i16 as i32;
    let t = cpq11_confronto(this, ri, confronto as i16 as i32);
    let (mut a1, mut a2, mut b1, mut b2, mut casa) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let h = cqc5_w(t);
    if h != -1 && cqc5_w(t + 2) != -1 {
        return;
    }
    if cqc5_w(cpq11_rodada(this, ri) + 0x1a) < 2 {
        return;
    }
    if h == -1 {
        let r2 = cqc5_w(t + 0xe);
        if r2 == -1 {
            return;
        }
        let o = cpq11_confronto(this, r2 as i32, cqc5_w(t + 0xa) as i32);
        let (x, y) = (cqc5_w(o), cqc5_w(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        a1 = cqc5_d(cpq11_time(this, x));
        a2 = cqc5_d(cpq11_time(this, y));
    } else {
        casa = cqc5_d(cpq11_time(this, h));
    }
    let fora;
    let v = cqc5_w(t + 2);
    if v == -1 {
        let r2 = cqc5_w(t + 0x10);
        if r2 == -1 {
            return;
        }
        let o = cpq11_confronto(this, r2 as i32, cqc5_w(t + 0xc) as i32);
        let (x, y) = (cqc5_w(o), cqc5_w(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        b1 = cqc5_d(cpq11_time(this, x));
        b2 = cqc5_d(cpq11_time(this, y));
        fora = 0;
    } else {
        fora = cqc5_d(cpq11_time(this, v));
    }
    crate::comparacao::ramo("cqc_5 confronto pendente");
    let mut noticia = [0u32; 0x38];
    let n = noticia.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    construir(n, 0x26, 0);
    gravar(n, 0, cqc5_d(cqc5_d(this + 4)) as u32);
    gravar(n, 1, cqc5_w(this + 0x3a) as i32 as u32);
    gravar(n, 2, cqc5_w(cpq11_rodada(this, ri) + 7) as i32 as u32);
    if casa != 0 {
        gravar(n, 3, cqc5_d(casa) as u32);
        gravar(n, 4, u32::MAX);
    } else {
        gravar(n, 3, cqc5_d(a1) as u32);
        gravar(n, 4, cqc5_d(a2) as u32);
    }
    if fora != 0 {
        gravar(n, 5, cqc5_d(fora) as u32);
        gravar(n, 6, u32::MAX);
    } else {
        gravar(n, 5, cqc5_d(b1) as u32);
        gravar(n, 6, cqc5_d(b2) as u32);
    }
    let news = cqc5_d(0xb7431c);
    if casa != 0 {
        entregar(news, n, casa);
    } else {
        if a1 != 0 {
            entregar(news, n, a1);
        }
        if a2 != 0 {
            entregar(news, n, a2);
        }
    }
    if fora != 0 {
        entregar(news, n, fora);
    } else {
        if b1 != 0 {
            entregar(news, n, b1);
        }
        if b2 != 0 {
            entregar(news, n, b2);
        }
    }
}

/// Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
unsafe extern "thiscall" fn comparar_copa_noticia_do_confronto_pendente(this: usize, rodada: u32, confronto: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1522)(this, rodada, confronto) // ponytail: só a notícia
}

/// { 0x8c6bc0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::update_transfer_window_news(hoje)`, do dia das transferências. Os meses anterior, atual e
/// seguinte (0x54cfb0: mov ecx, hoje -> thiscall, ret 0; o anterior de janeiro é dezembro do ano −1, o seguinte de
/// dezembro é janeiro do ano +1), hoje + 1 semana (0x54c770: push 1; push saída; mov ecx, hoje -> thiscall, ret 8) e
/// ontem (0x54c8e0: idem). Para cada janela do vetor [this+0x8a4] (conta +0x8a8, relida): no primeiro dos três meses
/// igual ao dela (+4), a data dela naquele ano (0x40a7d0: push ano; push saída; mov ecx, janela -> thiscall, ret 8;
/// copiada pela 0x417c10). Hoje: o virtual +0x2c das regras da nação dela ([this+0x8ac + 4·(+0)]: push o dword do ano
/// da data; push o código 1, ou 3 se a janela fecha (+5), com os bytes altos do mesmo dword; push janela -> thiscall,
/// ret 0xc); ontem e janela que abre: o virtual com 0 e as ofertas em espera da nação das regras (+0xa; 0x8c4e10: push
/// nação; mov ecx, esi -> thiscall, ret 4); daqui a uma semana e janela que abre: o virtual com 2.
unsafe extern "thiscall" fn janelas_avisos_do_dia(this: usize, hoje: usize) {
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let data_da_janela: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x40a7d0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4e10usize);
    let mut anos = [0i16; 3];
    let mut d = [0u8; 8];
    let pd = d.as_mut_ptr() as usize;
    padrao(pd);
    let mut meses = [0u8; 3];
    meses[0] = mes(hoje).wrapping_sub(1);
    if (meses[0] as i8) < 0 {
        meses[0] = 0xb;
        anos[0] = -1;
    }
    meses[1] = mes(hoje);
    meses[2] = mes(hoje).wrapping_add(1);
    if (meses[2] as i8) > 0xb {
        meses[2] = 0;
        anos[2] = 1;
    }
    let (mut semana, mut ontem) = ([0u8; 8], [0u8; 8]);
    let (ps, po) = (semana.as_mut_ptr() as usize, ontem.as_mut_ptr() as usize);
    mais_semanas(hoje, ps, 1);
    menos(hoje, po, 1);
    let mut v = cqc5_d(this + 0x8a4);
    let mut i: i32 = 0;
    while i < cqc5_d(this + 0x8a8) as i32 {
        for j in 0..3 {
            let w = cqc5_d(v);
            if cqc5_b(w + 4) != meses[j] {
                continue;
            }
            let ano = (anos[j] as u16).wrapping_add(cqc5_w(hoje + 2) as u16);
            let (mut t, mut c) = ([0u8; 8], [0u8; 8]);
            let r = data_da_janela(w, t.as_mut_ptr() as usize, ano as u32);
            copia(pd, c.as_mut_ptr() as usize, r);
            let (hd, ha) = (cqc5_w(hoje), cqc5_w(hoje + 2));
            let (dd, da, dw) = (cqc5_w(pd), cqc5_w(pd + 2), cqc5_d(pd + 2) as u32);
            let regras = |w: usize| cqc5_d(this + 0x8ac + (cqc5_b(w) as i8 as isize * 4) as usize);
            let aviso = |w: usize, codigo: u32| {
                let rg = regras(w);
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(cqc5_d(cqc5_d(rg) + 0x2c));
                f(rg, w, codigo, dw);
            };
            if ha == da && hd == dd {
                let codigo = if cqc5_b(w + 5) != 0 { 3 } else { 1 };
                aviso(w, (dw & 0xffff_ff00) | codigo); // cqc5: o aviso do dia da janela
                break;
            }
            if cqc5_w(po + 2) == da && cqc5_w(po) == dd && cqc5_b(w + 5) == 0 {
                aviso(w, 0);
                let nacao = cqc5_d(regras(cqc5_d(v)) + 0xa);
                if nacao != 0 {
                    crate::comparacao::ramo("cqc_5 janela aberta");
                    espera(this, nacao); // cqc5: as ofertas em espera da janela que abriu
                }
                break;
            }
            if cqc5_w(ps + 2) == da && cqc5_w(ps) == dd && cqc5_b(w + 5) == 0 {
                aviso(w, 2);
                break;
            }
        }
        i += 1;
        v += 4;
    }
}

/// Comparador da 0x8c6bc0: só a original (ela só grava nas locais; os avisos das regras alocam notícias e as ofertas
/// em espera mudam de status e são reagendadas).
unsafe extern "thiscall" fn comparar_janelas_avisos_do_dia(this: usize, hoje: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1523)(this, hoje) // ponytail: nada próprio a conferir
}

// ---------- 266a leva: a copa e as transferências: os jogos da rodada levados às datas marcadas, o jogador que sai da troca e as permissões de trabalho da oferta (Cup.cpp, transfer_manager.cpp) ----------

unsafe fn cqc7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc7_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqc7_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c] -> thiscall(notícias, clube),
/// ret 4).
unsafe fn cqc7_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc7_d(0xb7431c), clube) != 0
}

/// A pessoa (0x6e bytes em [0xae23bc]) e o clube (0x245 bytes em [0xae23b4]) pelo id.
unsafe fn cqc7_pessoa(id: usize) -> usize {
    cqc7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqc7_clube(id: usize) -> usize {
    cqc7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), gravado em [0xae1080].
unsafe fn cqc7_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqc7_d(0xae1050), pessoa);
    cqc7_pd(0xae1080, c);
    c
}

/// Lista vazia (palavra +0 = 0) da oferta (+4): destruída (0x68d040), liberada (0x944c94) e +4 = 0, como a original
/// (que lê a palavra antes de testar o ponteiro).
unsafe fn cqc7_soltar_lista_vazia(oferta: usize) {
    let l = cqc7_d(oferta + 4);
    if cqc7_w(l) != 0 {
        return;
    }
    if l != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(l);
        liberar(l);
    }
    cqc7_pd(oferta + 4, 0);
}

/// A entrada da rodada (0x68 bytes em [copa + 0xa3], o índice como o dword que a original multiplica).
unsafe fn cqc7_rodada(copa: usize, rodada: u32) -> usize {
    cqc7_d(copa + 0xa3).wrapping_add((rodada as usize).wrapping_mul(0x68))
}

/// Os jogos da copa na data e fatia da rodada, numa lista (0x5abd90: push lista; push liga +0x44; push rodada; push
/// id; push fatia +6; push dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c).
unsafe fn cqc7_jogos_da_rodada(copa: usize, data: usize, rodada: u32, lista: usize) -> u32 {
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    // ponytail: a original empurra o dia, a fatia e o tipo em registradores com sobra nos bytes altos; a 0x5abd90 só
    // lê as palavras e os bytes
    jogos(cqc7_d(0xae2a58), cqc7_w(data + 2) as u16 as u32, cqc7_w(data) as u16 as u32, cqc7_b(cqc7_rodada(copa, rodada) + 6) as u32,
        cqc7_d(cqc7_d(copa + 4)) as u32, rodada, cqc7_b(copa + 0x44) as u32, lista) & 0xffff
}

/// O jogo `f` está na data marcada da rodada: dia (+0x2a, sem sinal) = +3 da rodada, ano (+0x28) = +5 + o da copa
/// (+0x40), fatia (+0x3f) = +6 e o tipo (+0x42) da liga.
unsafe fn cqc7_na_data_da_rodada(copa: usize, e: usize, f: usize) -> bool {
    cqc7_w(f + 0x2a) as u16 as i32 == cqc7_w(e + 3) as i32
        && cqc7_w(f + 0x28) as u16 as i32 == cqc7_b(e + 5) as i8 as i32 + cqc7_w(copa + 0x40) as i32
        && cqc7_b(f + 0x3f) as i32 == cqc7_b(e + 6) as i8 as i32
        && cqc7_b(f + 0x42) == cqc7_b(copa + 0x44)
}

/// { 0x51ebe0 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: as notícias, a data e a lista
/// locais): `Cup::move_round_fixtures_to_set_dates(data, rodada)`, da montagem da rodada (0x51c800, em Rust). A notícia
/// 0x14 local é montada logo (0x77bbe0). Rodada (0x68 bytes em +0xa3) com o bit 0x100 em +0xd e a primeira vaga
/// marcada (+0x29 ≠ 0xff): os jogos da copa na data e fatia (cqc7_jogos_da_rodada). Com jogos, a notícia leva o id da
/// copa, +0x3a, o código da rodada (+7) e a rodada de duas mãos (+0x21 = 2). Cada jogo do tipo da liga (+0x42) ocupa
/// uma vaga (7 bytes a partir de +0x23: +0 regras do estádio, +4 dia da semana, +5 fatia, +6 tipo; até 8, até a
/// primeira 0xff): na data da rodada, vai ao dia da semana da vaga (0x549ef0 num bloco de 8 bytes de 0x944cf1, 0x54a2d0,
/// liberado depois), com a notícia 3 aos clubes fora do tipo 3, sai do calendário (0x5abc30), ganha a data, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta (0x5aa680); sem a copa ainda em [0xadadfc], para; senão a vaga fica
/// 0xff. Com ou sem a mudança, a notícia 0x14 vai aos clubes se um é de técnico humano. Sem vaga (fim das vagas ou
/// vaga vazia): só a notícia, com os dados do jogo.
unsafe extern "thiscall" fn copa_levar_jogos_as_datas(this: usize, data: usize, rodada: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut noticia = [0u32; 0x38];
    let n1 = noticia.as_mut_ptr() as usize;
    construir(n1, 0x14, 0);
    let id = cqc7_d(cqc7_d(this + 4));
    let e = cqc7_rodada(this, rodada);
    if cqc7_w(e + 0xd) as u16 & 0x100 == 0 || cqc7_b(e + 0x29) == 0xff {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    if cqc7_jogos_da_rodada(this, data, rodada, l) != 0 {
        gravar(n1, 0, cqc7_d(cqc7_d(this + 4)) as u32);
        gravar(n1, 1, cqc7_w(this + 0x3a) as i32 as u32);
        gravar(n1, 2, cqc7_w(cqc7_rodada(this, rodada) + 7) as i32 as u32);
        gravar(n1, 5, (cqc7_b(cqc7_rodada(this, rodada) + 0x21) == 2) as u32);
        // a notícia 0x14 do jogo (dados 3, 4, 6, 7, 8) aos dois clubes, se um deles é de técnico humano
        let noticia_do_jogo = |c1: usize, c2: usize, i1: usize, i2: usize, est: usize, dia: u16, ano: u16| {
            if cqc7_humano(c1) || cqc7_humano(c2) {
                if cqc7_humano(c1) {
                    gravar(n1, 3, i1 as u32);
                    gravar(n1, 4, i2 as u32);
                } else {
                    gravar(n1, 3, i2 as u32);
                    gravar(n1, 4, i1 as u32);
                }
                gravar(n1, 6, est as u32);
                gravar(n1, 7, dia as u32);
                gravar(n1, 8, ano as u32);
                entregar(cqc7_d(0xb7431c), n1, c1);
                entregar(cqc7_d(0xb7431c), n1, c2);
            }
        };
        cqc7_pd(l + 0xa, cqc7_d(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if cqc7_b(this + 0x44) == cqc7_b(f + 0x42) {
                let e = cqc7_rodada(this, rodada);
                if j < 0x38 && cqc7_b(e + j + 0x29) != 0xff {
                    let mut buf = [0u8; 0x50];
                    let r = buf.as_mut_ptr() as usize;
                    core::ptr::copy_nonoverlapping(f as *const u8, r as *mut u8, 0x4f);
                    if cqc7_na_data_da_rodada(this, e, f) {
                        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
                        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
                        let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
                        let p = alocar(8);
                        let dp = if p != 0 { nova(p, cqc7_w(f + 0x2a) as u16 as u32, cqc7_w(f + 0x28) as u16 as u32) } else { 0 };
                        let mut t = [0u8; 8];
                        let pt = t.as_mut_ptr() as usize;
                        padrao(pt);
                        let e = cqc7_rodada(this, rodada);
                        ajustar(dp, cqc7_b(e + j + 0x27) as i8 as i16 as u16 as u32, pt); // cqc7: o dia da semana marcado
                        if cqc7_b(cqc7_rodada(this, rodada) + j + 0x29) != 3 {
                            let mut noticia2 = [0u32; 0x38];
                            let n2 = noticia2.as_mut_ptr() as usize;
                            construir(n2, 3, 0);
                            gravar(n2, 0, cqc7_d(r + 0xc) as u32);
                            gravar(n2, 1, cqc7_d(r + 0x10) as u32);
                            gravar(n2, 2, cqc7_d(r) as u32);
                            gravar(n2, 3, cqc7_w(r + 0x32) as i32 as u32);
                            gravar(n2, 4, cqc7_w(r + 0x30) as i32 as u32);
                            gravar(n2, 5, cqc7_w(r + 0x2a) as u16 as u32);
                            gravar(n2, 6, cqc7_w(r + 0x28) as u16 as u32);
                            gravar(n2, 7, cqc7_b(r + 0x3f) as u32);
                            gravar(n2, 8, cqc7_w(pt) as i32 as u32);
                            gravar(n2, 9, cqc7_w(pt + 2) as u16 as u32);
                            gravar(n2, 10, cqc7_b(cqc7_rodada(this, rodada) + j + 0x28) as i8 as i32 as u32);
                            gravar(n2, 11, cqc7_b(cqc7_rodada(this, rodada) + j + 0x29) as i8 as i32 as u32);
                            entregar(cqc7_d(0xb7431c), n2, cqc7_d(r + 0x1c));
                            entregar(cqc7_d(0xb7431c), n2, cqc7_d(r + 0x20));
                        }
                        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
                        tirar(cqc7_d(0xae2a58), r);
                        crate::comparacao::ramo("cqc_7 jogo levado a data");
                        cqc7_pw(r + 0x2a, cqc7_w(pt) as u16); // cqc7: o dia marcado
                        cqc7_pw(r + 0x28, cqc7_w(pt + 2) as u16);
                        let v = cqc7_rodada(this, rodada) + j;
                        cqc7_pb(r + 0x3f, cqc7_b(v + 0x28)); // cqc7: a fatia marcada
                        cqc7_pb(r + 0x3b, cqc7_b(v + 0x29));
                        let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
                        estadio(cqc7_d(0xdd7ef4), cqc7_d(cqc7_rodada(this, rodada) + j + 0x23), r, cqc7_d(this + 0x24));
                        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
                        juntar(cqc7_d(0xae2a58), r, 1);
                        if dp != 0 {
                            let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
                            liberar(dp);
                        }
                        if cqc7_d(cqc7_d(0xadadfc) + id * 4) == 0 {
                            destruir(l);
                            return;
                        }
                        cqc7_pb(cqc7_rodada(this, rodada) + j + 0x29, 0xff);
                    }
                    noticia_do_jogo(cqc7_d(r + 0x1c), cqc7_d(r + 0x20), cqc7_d(r + 0xc), cqc7_d(r + 0x10), cqc7_d(r + 4),
                        cqc7_w(r + 0x2a) as u16, cqc7_w(r + 0x28) as u16);
                    j += 7;
                } else {
                    noticia_do_jogo(cqc7_d(f + 0x1c), cqc7_d(f + 0x20), cqc7_d(f + 0xc), cqc7_d(f + 0x10), cqc7_d(f + 4),
                        cqc7_w(f + 0x2a) as u16, cqc7_w(f + 0x28) as u16);
                }
            }
            f = proximo(l);
        }
    }
    destruir(l);
}

/// Comparador da 0x51ebe0: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, e as notícias
/// vão aos técnicos). O plano, feito antes: os jogos da copa na data e fatia (0x5abd90 numa lista provisória), na
/// ordem da lista; cada um do tipo da liga ocupa a vaga seguinte (de 7 em 7 bytes, até 0x38 e até a primeira 0xff), e
/// a vaga do que está na data da rodada fica 0xff; as outras vagas não mudam. Sem a copa em [0xadadfc] depois: nada a
/// conferir.
unsafe extern "thiscall" fn comparar_copa_levar_jogos_as_datas(this: usize, data: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1524);
    let e = cqc7_rodada(this, rodada);
    if cqc7_w(e + 0xd) as u16 & 0x100 == 0 || cqc7_b(e + 0x29) == 0xff {
        return o(this, data, rodada);
    }
    let id = cqc7_d(cqc7_d(this + 4));
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let mut esperado = [0u8; 8];
    for k in 0..8 {
        esperado[k] = cqc7_b(e + k * 7 + 0x29);
    }
    if cqc7_jogos_da_rodada(this, data, rodada, l) != 0 {
        cqc7_pd(l + 0xa, cqc7_d(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if cqc7_b(this + 0x44) == cqc7_b(f + 0x42) && j < 0x38 && esperado[j / 7] != 0xff {
                if cqc7_na_data_da_rodada(this, e, f) {
                    esperado[j / 7] = 0xff;
                }
                j += 7;
            }
            f = proximo(l);
        }
    }
    destruir(l);
    o(this, data, rodada);
    if cqc7_d(cqc7_d(0xadadfc) + id * 4) == 0 {
        return; // ponytail: a copa saiu do jogo; nada a conferir
    }
    let e = cqc7_rodada(this, rodada);
    let depois: [u8; 8] = core::array::from_fn(|k| cqc7_b(e + k * 7 + 0x29));
    PLACAR.registrar(depois == esperado, || format!("copa_levar_jogos_as_datas(rodada {}) vagas {:02x?} (esperado {:02x?})",
        rodada, depois, esperado));
}

/// { 0x8c41c0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a lista local):
/// `TRANSFER_MANAGER::remove_exchange_player(pessoa, oferta, só_esta, motivo)`, da oferta de contrato ao jogador
/// (0x8c1e40), da confirmação pelo clube (0x8c3310), da soltura dos jogadores da oferta (0x8c6ae0) e da tela. Só esta:
/// sem a lista da troca (+4), nada; com a pessoa nela (+0 de cada item = id), sai (0x68d1f0), o registro de 8 bytes dela
/// ([this+0x86e] + id × 8, +2) perde uma oferta, a notícia (0x8de150 com o valor = +0x52 da pessoa + +0x1b da oferta e o
/// motivo) e, com o dono do contrato do alvo (0x4cf5a0, fora do tipo 5) do computador (0x600a50), a resposta dele
/// (0x8c8090 com &oferta e 1): oferta resolvida (status 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3) só solta a lista
/// vazia; senão o preço (+0x1b) vira o valor e a oferta segue (0x8c2f50). No fim, a lista vazia sai. Senão, com a pessoa
/// em alguma oferta (+2 do registro): as ofertas dela pelo clube (0x8c5e40 numa lista local), menos esta; em cada uma, sai
/// da lista da troca (a lista vazia sai) e, desde a primeira que a tinha, o preço vira +0x52 + o preço, a notícia (motivo
/// 0) e a oferta segue.
unsafe extern "thiscall" fn tirar_jogador_da_troca(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x8de150usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let menos_uma = |p: usize| {
        let a = cqc7_d(this + 0x86e) + cqc7_d(p) * 8 + 2;
        cqc7_pb(a, cqc7_b(a).wrapping_sub(1));
    };
    let mut oferta = oferta;
    if so_esta != 0 {
        let c = cqc7_contrato(cqc7_pessoa(cqc7_d(oferta + 0xc)));
        let dono = if c != 0 && cqc7_b(c + 0x35) & 0x3f != 5 { cqc7_clube(cqc7_d(c + 4)) } else { 0 };
        let troca = cqc7_d(oferta + 4);
        if troca == 0 {
            destruir(l);
            return;
        }
        cqc7_pd(troca + 0xa, cqc7_d(troca + 2));
        let mut it = proximo(cqc7_d(oferta + 4));
        while it != 0 && cqc7_d(pessoa) != cqc7_d(it) {
            it = proximo(cqc7_d(oferta + 4));
        }
        if it != 0 {
            tirar(cqc7_d(oferta + 4), it);
            menos_uma(pessoa);
            let v = cqc7_d(pessoa + 0x52).wrapping_add(cqc7_d(oferta + 0x1b));
            noticia(this, oferta, pessoa, v, motivo);
            let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if dono != 0 && clube_humano(dono, 0, 1, 0, 0) == 0 {
                let resposta: unsafe extern "thiscall" fn(usize, *mut usize, u32) = core::mem::transmute(0x8c8090usize);
                resposta(this, &mut oferta, 1);
                let s = cqc7_b(oferta + 0x2c);
                if matches!(s, 2 | 3 | 0x14 | 0x10 | 4) || (s == 0xf && cqc7_b(oferta + 0x2e) >= 3) {
                    cqc7_soltar_lista_vazia(oferta);
                    destruir(l);
                    return;
                }
            }
            crate::comparacao::ramo("cqc_7 sai da troca da oferta");
            cqc7_pd(oferta + 0x1b, v);
            seguir(this, oferta);
        }
        cqc7_soltar_lista_vazia(oferta);
    } else {
        if cqc7_b(cqc7_d(this + 0x86e) + cqc7_d(pessoa) * 8 + 2) != 0 {
            let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
            trocas(this, cqc7_pessoa(cqc7_d(oferta + 0xc)), l);
            cqc7_pd(l + 0xa, cqc7_d(l + 2));
            let mut achou = false;
            let mut it = proximo(l);
            while it != 0 {
                let o = cqc7_d(cqc7_d(this) + cqc7_d(it) * 4);
                if o != oferta {
                    if cqc7_d(o + 4) != 0 {
                        let t = cqc7_d(o + 4);
                        cqc7_pd(t + 0xa, cqc7_d(t + 2));
                        let mut x = proximo(cqc7_d(o + 4));
                        while x != 0 {
                            if cqc7_d(pessoa) == cqc7_d(x) {
                                achou = true;
                                tirar(cqc7_d(o + 4), x);
                                menos_uma(pessoa);
                                break;
                            }
                            x = proximo(cqc7_d(o + 4));
                        }
                        cqc7_soltar_lista_vazia(o);
                    }
                    if achou {
                        crate::comparacao::ramo("cqc_7 sai da troca das outras");
                        let v = cqc7_d(pessoa + 0x52).wrapping_add(cqc7_d(o + 0x1b));
                        noticia(this, o, pessoa, v, 0);
                        cqc7_pd(o + 0x1b, v);
                        seguir(this, o);
                    }
                }
                it = proximo(l);
            }
        }
    }
    destruir(l);
}

/// Comparador da 0x8c41c0: só a original (os nós da troca são liberados, a notícia vai aos técnicos e a oferta segue
/// pela 0x8c2f50, que pode fechar a transferência e tirar outros jogadores das trocas, ou ser respondida pelo vendedor
/// do computador, 0x8c8090, que pode mudar a oferta da vaga).
unsafe extern "thiscall" fn comparar_tirar_jogador_da_troca(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1525)(this, pessoa, oferta, so_esta, motivo) // ponytail: o efeito segue por outras rotinas
}

/// A permissão de trabalho da pessoa no clube: precisa (0x8c7560: push 0; push clube; push pessoa; mov ecx, this ->
/// thiscall, ret 0xc), a proposta de contrato dela ao clube (0x4cc9e0: push id do clube; push id da pessoa; mov ecx,
/// [0xae1050] -> thiscall, ret 8) e a concessão (0x8c7950: push 0; push 0; push proposta; push nação do clube +0x53; push
/// pessoa; mov ecx, this -> thiscall, ret 0x14, sem sorteio). Devolve (precisa, proposta, concedida).
unsafe fn cqc7_permissao(this: usize, pessoa: usize, clube: usize) -> (bool, usize, bool) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if precisa(this, pessoa, clube, 0) == 0 {
        return (false, 0, false);
    }
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let r = proposta(cqc7_d(0xae1050), cqc7_d(pessoa) as u32, cqc7_d(clube) as u32);
    if r == 0 {
        return (true, 0, false);
    }
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7950usize);
    (true, r, concedida(this, pessoa, cqc7_d(clube + 0x53), r, 0, 0) != 0)
}

/// { 0x8c7690 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::check_offer_work_permits(oferta, quieto)`,
/// do processamento da oferta (status 0xa, 0x8c6880) e da conclusão (0x8cbc40). O dono do contrato do alvo (0x4cf5a0,
/// [0xae1080]; qualquer tipo). Alvo (+0xc) com permissão a pedir no comprador (+8) e proposta, sem concessão
/// (cqc7_permissao): a oferta desfeita (0x8de790 com 0x13), status 0x10, as propostas fechadas (0x8c6db0), as marcas de
/// lance saem (0x84aeb0 no gerente de listas [this+0x856]: o comprador e o alvo, depois o dono do contrato do alvo e cada
/// jogador da troca), reagendada em 10 dias (0x8c6220) e devolve 0. Concedida (ou sem proposta) e sem `quieto`: a
/// notícia da concessão (0x8e3130). Depois, cada jogador da troca (+4) no dono do contrato do alvo: recusado, a notícia
/// (0x8e35a0 com o valor +0x52 + o preço), sai da troca (0x68d1f0), o registro de 8 bytes dele perde uma oferta e o
/// preço (+0x1b) vira o valor; concedido sem `quieto`, a notícia. A lista vazia sai; sem `quieto`, a oferta segue
/// (0x8c2f50). Devolve 1.
unsafe extern "thiscall" fn permissoes_da_oferta(this: usize, oferta: usize, quieto: u32) -> u32 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let concedida_noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8e3130usize);
    let alvo = cqc7_pessoa(cqc7_d(oferta + 0xc));
    let comprador = cqc7_clube(cqc7_d(oferta + 8));
    let c = cqc7_contrato(alvo);
    let dono = if c != 0 { cqc7_clube(cqc7_d(c + 4)) } else { 0 };
    let (precisa, proposta, concedida) = cqc7_permissao(this, alvo, comprador);
    if precisa {
        if proposta != 0 && !concedida {
            crate::comparacao::ramo("cqc_7 permissao recusada");
            let desfeita: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8de790usize);
            let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
            let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
            let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
            desfeita(this, oferta, comprador, 0x13);
            cqc7_pb(oferta + 0x2c, 0x10); // cqc7: a oferta cai pela permissão
            fechar(this, oferta);
            let alvo = cqc7_pessoa(cqc7_d(oferta + 0xc));
            let c = cqc7_contrato(alvo);
            let dono = if c != 0 { cqc7_clube(cqc7_d(c + 4)) } else { 0 };
            desmarcar(cqc7_d(this + 0x856), cqc7_clube(cqc7_d(oferta + 8)), alvo);
            if cqc7_d(oferta + 4) != 0 {
                let t = cqc7_d(oferta + 4);
                cqc7_pd(t + 0xa, cqc7_d(t + 2));
                let mut it = proximo(cqc7_d(oferta + 4));
                while it != 0 {
                    desmarcar(cqc7_d(this + 0x856), dono, cqc7_pessoa(cqc7_d(it)));
                    it = proximo(cqc7_d(oferta + 4));
                }
            }
            agendar(this, oferta, 0xa, 0);
            return 0;
        }
        if quieto == 0 {
            concedida_noticia(this, alvo, comprador, oferta);
        }
    }
    if cqc7_d(oferta + 4) != 0 {
        let t = cqc7_d(oferta + 4);
        cqc7_pd(t + 0xa, cqc7_d(t + 2));
        let mut it = proximo(cqc7_d(oferta + 4));
        while it != 0 {
            let x = cqc7_pessoa(cqc7_d(it));
            let (precisa, proposta, concedida) = cqc7_permissao(this, x, dono);
            if precisa {
                if proposta != 0 && !concedida {
                    crate::comparacao::ramo("cqc_7 troca recusada pela permissao");
                    let v = cqc7_d(x + 0x52).wrapping_add(cqc7_d(oferta + 0x1b));
                    let recusada: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8e35a0usize);
                    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                    recusada(this, oferta, x, v);
                    tirar(cqc7_d(oferta + 4), it);
                    let a = cqc7_d(this + 0x86e) + cqc7_d(x) * 8 + 2;
                    cqc7_pb(a, cqc7_b(a).wrapping_sub(1));
                    cqc7_pd(oferta + 0x1b, v);
                } else if quieto == 0 {
                    concedida_noticia(this, x, dono, oferta);
                }
            }
            it = proximo(cqc7_d(oferta + 4));
        }
        cqc7_soltar_lista_vazia(oferta);
    }
    if quieto == 0 {
        let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
        seguir(this, oferta);
    }
    1
}

/// Comparador da 0x8c7690: só a original (as notícias alocam, a oferta é reagendada e segue pela 0x8c2f50). O plano,
/// feito antes pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo recusado devolve 0 com a
/// oferta no status 0x10; senão 1.
unsafe extern "thiscall" fn comparar_permissoes_da_oferta(this: usize, oferta: usize, quieto: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1526);
    let (precisa, proposta, concedida) = cqc7_permissao(this, cqc7_pessoa(cqc7_d(oferta + 0xc)), cqc7_clube(cqc7_d(oferta + 8)));
    let recusada = precisa && proposta != 0 && !concedida;
    let r = o(this, oferta, quieto);
    let ok = if recusada { r == 0 && cqc7_b(oferta + 0x2c) == 0x10 } else { r == 1 };
    PLACAR.registrar(ok, || format!("permissoes_da_oferta(oferta {:x}) recusada prevista {} devolveu {}", oferta, recusada, r));
    r
}

// ---------- 266a leva: a diretoria depois da partida: a confiança pelo resultado, a notícia da reação e a avaliação da federação (manager_manager.cpp) ----------

unsafe fn cqc8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqc8_u(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc8_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// Uma constante de ponto flutuante do exe (lida no lugar, como a original faz).
unsafe fn cqc8_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// _ftol (0x945640): truncado para inteiro (a original usa só eax).
fn cqc8_ftol(x: f64) -> i32 {
    x as i64 as i32
}

unsafe fn cqc8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A linha da diretoria do clube no gerenciador dos técnicos: [this] + id × 0x49.
unsafe fn cqc8_linha(this: usize, clube: usize) -> usize {
    cqc8_d(this).wrapping_add((cqc8_i(clube) as usize).wrapping_mul(0x49))
}

/// 0x53b900 (cdecl, 1): o clube é seleção.
unsafe fn cqc8_selecao(c: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(c)
}

/// O peso do jogo (o double em [esp+0x2c]) e o fator (o double em [esp+0x34], 0,5 de início), pela competição da
/// partida: sem competição 0,01; fora da competição do clube (0x48f150) 0,2; amistoso (0x5c6710) 0,67 se o mandante
/// é seleção, senão 0,05; não jogada (+0x40) 0,1; na liga do mandante 1 com o fator 0,06 × (+0x69 da liga) + 0,27;
/// senão 1 − |500 × (+0x69) − reputação| / 15000, × 1,25 na copa (+0x3a = 8) e pela fase (+0x30: 0x78 × 1,25, 0x82
/// × 1,5, 0x8c × 0,75, 0x96 e 0xa0 × 2).
unsafe fn cqc8_peso(clube: usize, partida: usize, fator: &mut f64) -> f64 {
    let comp = cqc8_d(partida + 0x14);
    if comp == 0 {
        return cqc8_f(0x967898);
    }
    let na_comp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x48f150usize);
    let obj = cqc8_d(cqc8_d(0xadadfc).wrapping_add((cqc8_i(partida) as usize).wrapping_mul(4)));
    if na_comp(obj, clube) == 0 {
        return cqc8_f(0x968918);
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    if amistoso(cqc8_d(0xae2c74), comp) != 0 {
        return if cqc8_selecao(cqc8_d(partida + 0x1c)) != 0 { cqc8_f(0x968f80) } else { cqc8_f(0x967888) };
    }
    if cqc8_u(partida + 0x40) == 0 {
        return cqc8_f(0x967880);
    }
    if comp == cqc8_d(cqc8_d(partida + 0x1c) + 0x57) {
        *fator = cqc8_w(comp + 0x69) as i32 as f64 * cqc8_f(0x96d1e0) + cqc8_f(0x96d1d8);
        return cqc8_f(0x967890);
    }
    let x = (cqc8_w(comp + 0x69) as i32).wrapping_mul(500).wrapping_sub(cqc8_w(clube + 0x80) as i32);
    let mut p = cqc8_f(0x967890) - (x as f64).abs() * cqc8_f(0x968d78);
    if cqc8_u(partida + 0x3a) == 8 {
        p *= cqc8_f(0x968928);
    }
    match cqc8_w(partida + 0x30) as u16 {
        0x78 => p *= cqc8_f(0x968928),
        0x82 => p *= cqc8_f(0x968940),
        0x8c => p *= cqc8_f(0x969040),
        0x96 | 0xa0 => p = p + p,
        _ => {}
    }
    p
}

/// A nota de uma linha da escalação (0x49 bytes; `e` aponta o byte da nota em +0x1c0/+0x1c4): o byte que entra nas
/// médias (0 = fora). Com +0x4c do jogo, os 11 titulares valem 90; reservas (k ≥ 11) só com o byte de entrada
/// (e − 2 ≠ 0xff) e menos ele. `nota`, `troca` são os deslocamentos dos dois bytes tentados em ordem (o primeiro ≥ 0
/// vale; senão o +0x4d do jogo).
unsafe fn cqc8_nota(jogo: usize, e: usize, k: i32, nota: usize, troca: usize, entrada: usize) -> Option<i8> {
    if cqc8_u(jogo + 0x4c) != 0 && k < 11 {
        return Some(0x5a);
    }
    let escolher = || {
        let v = cqc8_b(e + nota);
        if v > -1 {
            return v;
        }
        let v = cqc8_b(e + troca);
        if v > -1 { v } else { cqc8_b(jogo + 0x4d) }
    };
    if k < 11 {
        return Some(escolher());
    }
    let dl = cqc8_b(e + entrada);
    if dl == -1 {
        return None;
    }
    Some(escolher().wrapping_sub(dl))
}

/// { 0x695240 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::update_board_confidence_after_match(
/// clube, partida, jogo, mandante)`, pela diretoria depois da partida (0x6951e0, um lado por vez). O ajuste começa
/// pela disputa (±20/30 contra o adversário ou pela seleção da partida +0x18), soma a diferença de reputação / 35 (sem
/// jogo) ou, com o jogo, a nota do time (média das notas por idade +0x1c4, as notas +0x127/+0x128, a diferença com o
/// adversário, a soma de +0x4d − nota; x87: (... + 0,1) × 67 − 470, × fator) mais a reputação média dos adversários
/// (pela nação, 0x53d980) contra a do clube / 40, até ±100. Pelo placar (+0x47/+0x48 ou +0x43/+0x44): vitória 0x800,
/// derrota 0x1000 (fora de amistoso, × 2 ou × 5 com goleada; abaixo de −50 com técnico da IA, o pedido à federação
/// 0x6a9e60 por sorteio), empate 0x400 (na copa, +15 e as fases 0x7b/0x7c/0x85/0x86/0x99/0x9a). O agregado (+0x4b/+0x4c,
/// +0x49/+0x4a, +0x43/+0x44) dá 0x100/0x200 e ±10/15/20; o clássico (0x40, mesma cidade ou rival a até 25 km) ou o
/// rival/confronto (0x80) multiplicam o peso (2,25 ou 1,5) e dão ±12/±5. Peso × 0,5 na seleção B, × 2 na principal,
/// × 2 ou 1,5 na copa (marca 0x400 da linha); o ajuste × peso, ÷ 3 ou ÷ 2 pelas marcas. As marcas 1/2/4/8/0x10/0x20
/// (pelo placar, pelo ajuste e pela confiança +6) vão ao texto (0x6a69c0) e à notícia da reação (0x6a5fa0, só técnico
/// humano). A história da mídia 0x3d (0x74b060, −250 em +0x16) por sorteio no clube dirigível; a 0x5c na liga com o
/// clube em perigo. A confiança: +6 += 0,7 ×, +0xa += 0,5 ×, +8 += 1,15 ×, +0xc += 0,2 × (metade com a marca
/// 0x7a7eb0 no negativo), entre 1 e 10000; pisos pela fase e pela força da competição; o protesto da torcida (0x6a5d10);
/// na liga ou no negativo, pensar em demitir (0x69aba0, 1 na derrota); senão sai da lista dos ultimatos (0x68d1f0).
unsafe extern "thiscall" fn confianca_depois_da_partida(this: usize, clube: usize, partida: usize, jogo: usize, mandante: u32) {
    let outro = if mandante != 0 { cqc8_d(partida + 0x20) } else { cqc8_d(partida + 0x1c) };
    let vis = mandante == 0;
    let mut fator: f64 = 0.5;
    let mut peso = cqc8_peso(clube, partida, &mut fator);
    // a disputa (pela seleção da partida +0x18)
    let ev = cqc8_d(partida + 0x18);
    let cidade = |p: usize| p != 0 && cqc8_d(p + 0x38) != 0;
    let mut aj: i32 = 0;
    if cqc8_selecao(clube) != 0 {
        if cidade(ev) && cqc8_d(cqc8_d(ev + 0x38) + 0x1f) == cqc8_d(clube + 0x53) {
            aj = -20;
        } else if cqc8_d(clube + 0x69) == ev {
            aj = -20;
        } else if cqc8_d(outro + 0x69) == ev {
            aj = 20;
        } else if cidade(ev) && cqc8_d(cqc8_d(ev + 0x38) + 0x1f) == cqc8_d(outro + 0x53) {
            aj = 20;
        }
    } else if cqc8_d(clube + 0x69) == ev {
        aj = -20;
    } else if cqc8_d(outro + 0x69) == ev {
        aj = if cqc8_d(outro + 0x53) != cqc8_d(clube + 0x53) { 30 } else { 20 };
    }
    let limitar = |q: i32| {
        let d = q as i16;
        if d < -100 { -100 } else if d > 100 { 100 } else { q }
    };
    let c: i32;
    if jogo == 0 {
        let x = (cqc8_w(clube + 0x80) as i32).wrapping_sub(cqc8_w(outro + 0x80) as i32);
        c = limitar(x / -35);
        aj = aj.wrapping_add(c);
    } else {
        let base = jogo + 0x625 * vis as usize;
        let (mut s1, mut s2, mut di) = (0i32, 0i32, 0u16);
        for k in 0..20i32 {
            let e = base + 0x1c4 + k as usize * 0x49;
            if cqc8_i(e - 0x34) == -1 {
                continue;
            }
            // a soma de +0x4d − nota, só quando a nota (e) vale
            let quatro_d = cqc8_w(jogo + 0x4d);
            let al = if cqc8_u(jogo + 0x4c) != 0 && k < 11 {
                0x5a
            } else if k < 11 {
                let v = cqc8_b(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v
                } else {
                    let v = cqc8_b(e - 4);
                    if v > -1 { v } else { cqc8_b(jogo + 0x4d) }
                }
            } else {
                let dl = cqc8_b(e - 2);
                if dl == -1 {
                    continue;
                }
                let v = cqc8_b(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v.wrapping_sub(dl)
                } else {
                    let v = cqc8_b(e - 4);
                    (if v > -1 { v } else { cqc8_b(jogo + 0x4d) }).wrapping_sub(dl)
                }
            };
            if al == 0 {
                continue;
            }
            s1 = s1.wrapping_add(al as i32);
            s2 = s2.wrapping_add((cqc8_u(e + 1) as i32).wrapping_mul(al as i32));
        }
        let media = if s1 != 0 { s2 as f64 / s1 as f64 } else { fator };
        let ob = jogo + 0x625 * (!vis) as usize;
        let b127 = cqc8_u(base + 0x127) as i32;
        let b128 = cqc8_u(base + 0x128) as i32;
        let o127 = cqc8_u(ob + 0x127) as i32;
        crate::comparacao::ramo("cqc_8 nota do jogo");
        let mut v = (b127 - 10) as f64 * cqc8_f(0x969020) + media;
        v += (b128 - 10) as f64 * cqc8_f(0x969020);
        v += (b127 - o127) as f64 * cqc8_f(0x968a98);
        v += (di as i16) as f64 * cqc8_f(0x96d1d0);
        v += cqc8_f(0x967880);
        v = v * cqc8_f(0x96d1c8) - cqc8_f(0x96d1c0); // cqc8: a nota do jogo (× 67 − 470)
        v *= fator;
        aj = aj.wrapping_add(cqc8_ftol(v));
        // a reputação média dos adversários escalados (pela nação do clube)
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        let (mut s1, mut s2) = (0i32, 0i32);
        for k in 0..20i32 {
            let e = ob + 0x1c0 + k as usize * 0x49;
            let id = cqc8_i(e - 0x30);
            if id == -1 {
                continue;
            }
            let p = cqc8_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            let nacao = cqc8_d(clube + 0x53);
            let ficha = cqc8_d(p + 0x61);
            let rep = if cqc8_d(outro + 0x53) == nacao {
                cqc8_w(ficha + 0xb)
            } else if cqc8_d(p + 0x1a) == nacao {
                cqc8_w(ficha + 9)
            } else if estrangeira(p, nacao, 1) == 0 {
                cqc8_w(cqc8_d(p + 0x61) + 9)
            } else {
                cqc8_w(cqc8_d(p + 0x61) + 0xd)
            };
            let al = match cqc8_nota(jogo, e, k, 4, 0, 2) {
                Some(a) => a,
                None => continue,
            };
            if al == 0 {
                continue;
            }
            s1 = s1.wrapping_add(al as i32);
            s2 = s2.wrapping_add((rep as i32).wrapping_mul(al as i32));
        }
        // ponytail: sem ninguém escalado a original soma o ponteiro da partida (o ecx que sobrou): reproduzido
        let m = if s1 != 0 { s2.wrapping_div(s1) } else { partida as i32 };
        let x = (cqc8_w(clube + 0x80) as i32).wrapping_sub((cqc8_w(outro + 0x80) as i32).wrapping_add(m) / 2);
        c = limitar(x / -40);
        aj = aj.wrapping_add(c);
    }
    // o placar: A e B (as notas de quem é o clube), C e D (o agregado), como a original guarda
    let p8 = |o: usize| cqc8_b(partida + o) as i32;
    let (a, b, cc, dd) = if cqc8_b(partida + 0x47) > -1 {
        if mandante != 0 { (p8(0x47), p8(0x48), p8(0x4b), p8(0x4c)) } else { (p8(0x48), p8(0x47), p8(0x4c), p8(0x4b)) }
    } else if mandante != 0 {
        (p8(0x43), p8(0x44), p8(0x4b), p8(0x4c))
    } else {
        (p8(0x44), p8(0x43), p8(0x4c), p8(0x4b))
    };
    let c16 = c as i16 as i32;
    let mut fl: u32;
    if a > b {
        fl = 0x800;
        crate::comparacao::ramo("cqc_8 vitoria");
        aj = aj.wrapping_add((c16 / 10 + 20).wrapping_mul(a - b)); // cqc8: a vitória
    } else if a < b {
        aj = aj.wrapping_add((20 - c16 / 10).wrapping_mul(a - b));
        fl = 0x1000;
        let comp = cqc8_d(partida + 0x14);
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        let mut fim = false;
        if comp != 0 && amistoso(cqc8_d(0xae2c74), comp) == 0 {
            let dif = b - a;
            if dif > 10 {
                aj = aj.wrapping_mul(5);
                if (aj as i16) < 0x4e2 {
                    aj = 0x4e2;
                    fim = true;
                }
            } else if dif > 5 {
                aj = aj.wrapping_add(aj);
            }
        }
        if !fim && (aj as i16) < -0x32 && clube != 0 {
            let tec = cqc8_d(clube + 0xcf);
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if tec != 0 && cqc8_d(tec + 0x69) != 0 && humano(clube, 0, 1, 0, 0) == 0 {
                let tec = cqc8_d(clube + 0xcf);
                let r = cqc8_sortear(6);
                if cqc8_b(tec + 0x5d) < (r as u8).wrapping_add(6) as i8
                    && cqc8_b(cqc8_d(0xae2d14).wrapping_add((cqc8_i(clube) as usize).wrapping_mul(6)) + 4) < 0
                    && cqc8_sortear(cqc8_b(cqc8_d(tec + 0x69) + 0x1d) as i32 + 1) == 0
                {
                    let pedido: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a9e60usize);
                    pedido(this, clube, 1);
                }
            }
        }
    } else {
        fl = 0x400;
        if cqc8_u(partida + 0x3a) == 8 {
            let f = cqc8_w(partida + 0x32) as i32;
            if (0x41f..=0x426).contains(&f) && vis {
                aj = aj.wrapping_add(0xf);
            }
            let s = (cqc8_w(partida + 0x30) as i32).wrapping_sub(0x7b);
            if (0..=0x1f).contains(&s) && matches!(s, 0 | 1 | 10 | 11 | 30 | 31) {
                aj = aj.wrapping_add(if vis { a.wrapping_mul(10) } else { b.wrapping_mul(-5) });
            }
        }
    }
    // o agregado
    let g4b = cqc8_b(partida + 0x4b);
    if g4b != -1 {
        let g4c = cqc8_b(partida + 0x4c);
        // (marca, ajuste) pelo lado
        let mais = |fl: &mut u32, aj: &mut i32, n: i32| {
            *fl |= 0x200;
            *aj = aj.wrapping_add(n);
        };
        let menos = |fl: &mut u32, aj: &mut i32, n: i32| {
            *fl |= 0x100;
            *aj = aj.wrapping_sub(n);
        };
        if g4b > g4c {
            if mandante != 0 { mais(&mut fl, &mut aj, 20) } else { menos(&mut fl, &mut aj, 20) }
        } else if g4b < g4c {
            if mandante == 0 { mais(&mut fl, &mut aj, 20) } else { menos(&mut fl, &mut aj, 20) }
        } else {
            let (g49, g4a) = (cqc8_b(partida + 0x49), cqc8_b(partida + 0x4a));
            if g49 > g4a {
                if mandante != 0 { mais(&mut fl, &mut aj, 10) } else { menos(&mut fl, &mut aj, 10) }
            } else if g49 < g4a {
                if mandante == 0 { mais(&mut fl, &mut aj, 10) } else { menos(&mut fl, &mut aj, 10) }
            } else if cqc8_u(partida + 0x38) & 4 == 0 {
                let x = g4b as i32 - cqc8_b(partida + 0x43) as i32;
                let fora = x > cqc8_b(partida + 0x44) as i32;
                if fora == (mandante != 0) { menos(&mut fl, &mut aj, 15) } else { mais(&mut fl, &mut aj, 15) }
            }
        }
    }
    // o clássico e o rival
    let rival = cqc8_d(clube + 0xb3) == outro || cqc8_d(clube + 0xb7) == outro || cqc8_d(clube + 0xbb) == outro;
    let f1 = cqc8_d(clube + 0x69);
    let mesma = f1 != 0 && cqc8_d(outro + 0x69) != 0 && cqc8_d(f1 + 0x38) == cqc8_d(cqc8_d(outro + 0x69) + 0x38);
    if mesma {
        fl |= if rival { 0x40 } else { 0x80 };
    } else if rival {
        fl |= 0x80;
        if f1 != 0 {
            let e1 = cqc8_d(f1 + 0x38);
            let f2 = cqc8_d(outro + 0x69);
            if e1 != 0 && f2 != 0 {
                let e2 = cqc8_d(f2 + 0x38);
                if e2 != 0 {
                    let x = cqc8_f(e1 + 0x2b) - cqc8_f(e2 + 0x2b);
                    let y = cqc8_f(e1 + 0x23) - cqc8_f(e2 + 0x23);
                    if (y * y + x * x).sqrt() * cqc8_f(0x96d1b8) <= cqc8_f(0x968890) {
                        fl |= 0x40;
                    }
                }
            }
        }
    } else {
        let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
        if confronto(clube, outro) != 0 {
            fl |= 0x80;
        }
    }
    if fl & 0x40 != 0 {
        peso *= cqc8_f(0x968ec8);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(0xc);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(0xc);
        }
    } else if fl & 0x80 != 0 {
        peso *= cqc8_f(0x968940);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(5);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(5);
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let mut mult = if segunda(clube) != 0 {
        peso * cqc8_f(0x968968)
    } else {
        let s = cqc8_selecao(clube);
        if s != 0 { peso + peso } else { peso }
    };
    if cqc8_u(partida + 0x3a) == 8 {
        if cqc8_u(cqc8_linha(this, clube) + 0xf) & 4 != 0 { // a palavra +0xe & 0x400
            mult = mult + mult;
            if (aj as i16) < 0 {
                aj = cqc8_ftol((aj as i16) as f64 * cqc8_f(0x968940));
            }
        } else {
            mult *= cqc8_f(0x968940);
        }
    }
    let mut dx = cqc8_ftol((aj as i16) as f64 * mult);
    if fl & 0x800 != 0 && (dx as i16) < 0 {
        dx = (dx as i16 as i32) / 3;
    }
    if fl & 0x1000 != 0 && (dx as i16) > 0 {
        dx = (dx as i16 as i32) / 3;
    }
    if fl & 0x200 != 0 && (dx as i16) < 0 {
        dx = (dx as i16 as i32) / 2;
    }
    if fl & 0x100 != 0 && (dx as i16) > 0 {
        dx = (dx as i16 as i32) / 2;
    }
    let aj = dx;
    let d16 = aj as i16;
    let conf6 = || cqc8_w(cqc8_linha(this, clube) + 6);
    if a + 5 < b {
        fl |= if clube == 0 || conf6() < 0x10b3 { 1 } else { 2 };
    } else if a - 5 > b {
        fl |= 8;
    } else if d16 < -0x78 && (clube == 0 || conf6() < 0x10b3) {
        fl |= 1;
    } else if d16 < -0x4b && (clube == 0 || conf6() < 0x1964) {
        fl |= 2;
    } else if d16 < -0x1e {
        fl |= 4;
    } else if d16 > 0x78 && clube != 0 && conf6() > 0x1964 {
        fl |= 8;
    } else if d16 > 0x4b {
        fl |= 0x20;
    } else if d16 > 0x1e {
        fl |= 0x10;
    }
    // o texto e a notícia da reação
    let marcas = cqc8_w(partida + 0x38) as u16;
    if marcas & 0x2810 == 0 {
        let texto: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(0x6a69c0usize);
        let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
        if texto(this, clube, outro, cqc8_d(partida + 0x14), fl as i16 as i32, 0xad9c64, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0) != 0
            && humano(cqc8_d(0xb7431c), clube) != 0
        {
            let c6 = conf6();
            if c6 > 0x1a5e {
                if fl & 1 != 0 {
                    fl ^= 5;
                } else if fl & 2 != 0 {
                    fl ^= 6;
                }
            } else if c6 > 0x1482 {
                if fl & 1 != 0 {
                    fl ^= 3;
                }
            } else if c6 < 0x1482 && fl & 8 != 0 {
                fl ^= 0x28;
            }
            let lim = ((vis as u16).wrapping_mul(0x4e2)).wrapping_add(cqc8_w(outro + 0x80) as u16).wrapping_add(0x1f4) as i16;
            if fl & 7 == 0 || cqc8_w(clube + 0x80) >= lim || cqc8_b(partida + 0x43) != cqc8_b(partida + 0x44) {
                let reacao: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32) = core::mem::transmute(0x6a5fa0usize);
                reacao(this, clube, outro, partida, fl, a, b, cc, dd);
            }
        }
    }
    // a história 0x3d da mídia
    let marcas = cqc8_w(partida + 0x38) as u16;
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let historia: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, i32) = core::mem::transmute(0x74b060usize);
    if fl & 0xa00 == 0 && marcas & 0x2810 == 0 && dirigivel(clube) != 0 {
        let s = aj as i16 as i32;
        let oito = || cqc8_w(cqc8_linha(this, clube) + 8) as i32;
        let r = cqc8_sortear(0x5a);
        let mut sai = false;
        if s < -0x5a - r {
            let r2 = cqc8_sortear(0x7d0);
            sai = oito().wrapping_add(s.wrapping_mul(10)) < 0xfa0 - r2;
        }
        if !sai {
            let r3 = cqc8_sortear(0x78);
            sai = s < -0x1e - r3 && oito().wrapping_add(s.wrapping_mul(10)) < 0x898;
        }
        if sai {
            historia(cqc8_d(0xb6b7c8), clube, 0x3d, 0, partida, 0, -1);
            let l = cqc8_linha(this, clube) + 0x16;
            cqc8_pw(l, cqc8_w(l).wrapping_add(-0xfa));
        }
    }
    // a história 0x5c: o clube em perigo na liga
    let id = cqc8_i(partida);
    if id > -1 && id == cqc8_i(clube + 0x1db) && id != cqc8_i(0x9cf6f8) && cqc8_u(partida + 0x3a) == 8
        && (cqc8_w(partida + 0x30) as i32 / 10) * 10 != 0x96
    {
        let co = cqc8_d(cqc8_d(0xadadfc).wrapping_add((id as usize).wrapping_mul(4)));
        if co != 0 && cqc8_d(co + 8) != 0 {
            let linha_da_liga: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
            let ln = linha_da_liga(cqc8_d(co + 8), clube);
            if fl & 0x100 == 0 {
                let n = cqc8_u(ln + 0x72);
                if n < 9 && n != 0 && cqc8_u(ln + 0x74) == 1 && (aj as i16 as i32) > (n as i32 * n as i32) / 2 + 0x32 {
                    historia(cqc8_d(0xb6b7c8), clube, 0x5c, 0, partida, 0, -1);
                }
            }
        }
    }
    let mut aj = aj;
    if (aj as i16) < 0 {
        let tec = cqc8_d(clube + 0xcf);
        let marca: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
        if tec != 0 && marca(cqc8_d(0xb7434c), tec) != 0 {
            aj = cqc8_ftol((aj as i16) as f64 * cqc8_f(0x968968));
        }
    }
    // a confiança
    let v = (aj as i16) as f64;
    crate::comparacao::ramo("cqc_8 confianca");
    for (o, k) in [(6usize, 0x968e40usize), (0xa, 0x968968), (8, 0x968980), (0xc, 0x968918)] {
        let l = cqc8_linha(this, clube) + o;
        let t = cqc8_ftol(v * cqc8_f(k)); // cqc8: a confiança pelo ajuste
        cqc8_pw(l, cqc8_w(l).wrapping_add(t as i16));
    }
    for o in [6usize, 0xa, 8, 0xc] {
        let l = cqc8_linha(this, clube) + o;
        let x = cqc8_w(l);
        if x > 0x2710 {
            cqc8_pw(l, 0x2710);
        } else if x < 1 {
            cqc8_pw(l, 1);
        }
    }
    let piso = |o: usize, v: i16| {
        let l = cqc8_linha(this, clube) + o;
        if cqc8_w(l) < v {
            cqc8_pw(l, v);
        }
    };
    let comp = cqc8_d(partida + 0x14);
    if (aj as i16) > 0 && cqc8_d(clube + 0xcf) != 0 {
        if comp != 0 {
            let n = cqc8_w(comp + 0x69);
            if n > 5 {
                let rep = cqc8_w(clube + 0x80);
                let fase = cqc8_w(partida + 0x30) as u16;
                if (n as i32) * 500 > rep as i32 {
                    if fase == 0x82 || fase == 0x78 {
                        piso(6, 0x1964);
                        piso(8, 0x1964);
                    } else if fase == 0x96 {
                        piso(6, 0x2134);
                        piso(8, 0x2134);
                    } else if comp != cqc8_d(clube + 0x57) && (rep < 0xea6 || cqc8_u(partida + 0x3a) == 8) {
                        piso(6, 0x10b3);
                        piso(8, 0x1964);
                    }
                } else if fase == 0x82 || fase == 0x96 {
                    piso(6, 0x1964);
                    piso(8, 0x1964);
                } else if comp != cqc8_d(clube + 0x57) && (fase == 0x78 || cqc8_u(partida + 0x3a) == 8) {
                    piso(6, 0x10b3);
                    piso(8, 0x10b3);
                }
            }
        }
    } else if (aj as i16) < 0 && comp != 0 && cqc8_w(partida + 0x30) == 0x96 && cqc8_u(partida + 0x3a) == 5 {
        let l = cqc8_linha(this, clube);
        if cqc8_u(l + 0xe) & 4 != 0 {
            if cqc8_w(l + 6) > 0x10b3 {
                cqc8_pw(l + 6, 0x10b3);
            } else if cqc8_w(l + 8) > 0xdac {
                cqc8_pw(l + 6, 0xdac);
            }
            let x = cqc8_w(l + 8);
            if x > 0x10b3 {
                cqc8_pw(l + 8, 0x10b3);
            } else if x > 0xdac {
                cqc8_pw(l + 8, 0xdac);
            }
        }
    }
    if cqc8_d(clube + 0xcf) != 0 && (aj as i16) < -0x4b && cqc8_w(cqc8_linha(this, clube) + 8) < 0xbb8 {
        let protesto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6a5d10usize);
        protesto(this, clube);
    }
    if cqc8_d(partida + 0x14) == cqc8_d(clube + 0x57) || (aj as i16) < 0 {
        if cqc8_w(partida + 0x30) != 0xa0 {
            let demitir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x69aba0usize);
            demitir(this, clube, (a < b) as u32);
        }
        return;
    }
    let lista = cqc8_d(this + 0x10);
    if cqc8_w(lista) == 0 {
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqc8_d(lista + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let mut i = 0i32;
    while i < cqc8_w(cqc8_d(this + 0x10)) as i32 {
        let e = proximo(cqc8_d(this + 0x10));
        if e != 0 && cqc8_i(clube) == cqc8_i(e) {
            tirar(cqc8_d(this + 0x10), e);
            return;
        }
        i += 1;
    }
}

// ponytail: chama o pedido à federação (0x6a9e60), a notícia da reação (0x6a5fa0), as histórias da mídia (0x74b060), o
// protesto, a demissão (0x69aba0) e tira item da lista dos ultimatos (0x68d1f0): alocam, liberam e sorteiam conforme o
// caminho; não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_confianca_depois_da_partida(this: usize, clube: usize, partida: usize, jogo: usize, mandante: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(1527)(this, clube, partida, jogo, mandante)
}

/// { 0x6a5fa0 } (thiscall, ret 0x20, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_board_match_reaction_news(clube, adversário, partida, marcas, a, b, c, d)`: com clube,
/// adversário, partida e competição (+0x14), a notícia 0x138c (0x77bbe0) com as marcas, os dois ids, os placares a e
/// b, o agregado c e d e +8 da partida; os até 3 jogadores que a diretoria cita (0x6aaee0, com a derrota a > b), um
/// sorteio(4), a derrota e as 5 marcas deles; o registro da partida pelo clube (0x7e4750) gravado na notícia
/// (0x7e4de0); entregue ao técnico do clube (0x7861e0).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn noticia_da_reacao_da_diretoria(this: usize, clube: usize, outro: usize, partida: usize, marcas: u32, a: i32, b: i32, c: i32, d: i32) {
    if clube == 0 || outro == 0 || partida == 0 || cqc8_d(partida + 0x14) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let citados: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, usize, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8 =
        core::mem::transmute(0x6aaee0usize);
    let registro: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
    let no_registro: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x7e4de0usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cqc_8 noticia da reacao");
    let mut noticia = [0u32; 0x3a]; // a notícia local (0xe0 bytes na original)
    let n = noticia.as_mut_ptr() as usize;
    let mut reg = [0u8; 0x20]; // o registro da partida
    let r = reg.as_mut_ptr() as usize;
    // a derrota e as 5 marcas dos citados (os bytes +0xe, +0x11, +0x13, +0x12, +0xf e +0x10 da original)
    let (mut m1, mut m2, mut m3, mut m4, mut m5) = (0u8, 0u8, 0u8, 0u8, 0u8);
    construir(n, 0x138c, 0);
    let mut k: u8 = 0;
    data_padrao(r);
    let mut ids = [-1i32; 3];
    for v in [marcas as i16 as i32 as u32, cqc8_d(clube) as u32, cqc8_d(outro) as u32, a as u32, b as u32, c as u32, d as u32, cqc8_d(partida + 8) as u32] {
        gravar(n, k as i8, v);
        k = k.wrapping_add(1);
    }
    let mut derrota = (a > b) as u8;
    citados(this, partida, clube, ids.as_mut_ptr(), cqc8_d(partida + 8), &mut derrota, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5);
    for v in ids {
        gravar(n, k as i8, v as u32);
        k = k.wrapping_add(1);
    }
    let s = cqc8_sortear(4);
    gravar(n, k as i8, s as u32);
    k = k.wrapping_add(1);
    for v in [derrota, m1, m2, m3, m4, m5] {
        gravar(n, k as i8, v as i8 as i32 as u32);
        k = k.wrapping_add(1);
    }
    registro(partida, clube, r);
    no_registro(r, n, &mut k);
    ao_tecnico(cqc8_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_noticia_da_reacao_da_diretoria(this: usize, clube: usize, outro: usize, partida: usize, marcas: u32, a: i32, b: i32, c: i32, d: i32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32)>(1528)(this, clube, outro, partida, marcas, a, b, c, d)
}

/// { 0x6a5a60 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_fa_performance_news(competição, seleção, nota)`: a partir da data do começo do jogo
/// ([0xdd6ec8]), a notícia 0x1781 (0x77bbe0) com o id da seleção, o id da competição (*competição) e a nota da
/// federação, pelo alcance (0x786090 com a seleção e 0, 0, 0, 0).
unsafe extern "thiscall" fn noticia_da_avaliacao_da_federacao(_this: usize, comp: usize, selecao: usize, nota: u32) {
    let dia = cqc8_w(0xdd6ec8);
    let ano = cqc8_w(0xdd6eca) as u16;
    let hoje = cqc8_w(0xae2c92) as u16;
    if ano > hoje || (ano == hoje && dia > cqc8_w(0xae2c90)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    crate::comparacao::ramo("cqc_8 avaliacao da federacao");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1781, 0);
    gravar(n, 0, cqc8_d(selecao) as u32);
    gravar(n, 1, cqc8_d(comp) as u32);
    gravar(n, 2, nota as i8 as i32 as u32);
    por_alcance(cqc8_d(0xb7431c), n, selecao, 0, 0, 0, 0);
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_avaliacao_da_federacao(this: usize, comp: usize, selecao: usize, nota: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1529)(this, comp, selecao, nota)
}

// ---------- 266a leva: as expectativas da diretoria para a temporada, a vaga de técnico e a notícia da oferta de emprego (manager_manager.cpp) ----------

unsafe fn cqc9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqc9_u(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc9_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cqc9_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqc9_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqc9_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// _ftol (0x945640): truncado (a original usa só eax ou ax).
fn cqc9_ftol(x: f64) -> i32 {
    x as i64 as i32
}

unsafe fn cqc9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A linha da diretoria do clube: [this] + id × 0x49.
unsafe fn cqc9_linha(this: usize, clube: usize) -> usize {
    cqc9_d(this).wrapping_add((cqc9_i(clube) as usize).wrapping_mul(0x49))
}

/// O objeto da competição da divisão do clube: [[0xadadfc] + [+0x57] × 4].
unsafe fn cqc9_comp(clube: usize) -> usize {
    cqc9_d(cqc9_d(0xadadfc).wrapping_add((cqc9_i(cqc9_d(clube + 0x57)) as usize).wrapping_mul(4)))
}

/// Método virtual sem argumentos da competição (+0x98 ou +0x9c da tabela), thiscall.
unsafe fn cqc9_virtual(obj: usize, desloc: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqc9_d(cqc9_d(obj) + desloc));
    f(obj)
}

unsafe fn cqc9_selecao(c: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(c)
}

/// { 0x6a3770 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::set_season_expectations(clube, notícia)`.
/// Zera as marcas (+0xe) e a meta (+0x2c), +0x30 = reputação² / 4, +0x38 ÷ 4, +0 = 0. Sem a marca 1 de +2, com técnico
/// e fora das seleções: pelos dias do técnico no cargo (0x54cdb0), acima de 3650 com confiança +6 < 3500 (sorteio(30)
/// acima do +0x1d do presidente, ou de 10) ou acima de 1825 com +6 < 3000 (sorteio(20)), o item {1, 6, id} na lista
/// [this+4] (0x68d0d0). Seleção: a B nada; a principal pega as expectativas da competição dela (0x7751e0 e o virtual
/// +0x70) e a notícia da federação (0x6a44a0). Clube: as vagas da UEFA da nação (0x905c60 em [0xde1f58]); sem técnico
/// nada; sem divisão com competição: 0x10. A meta = a posição de perigo da linha da liga (+0x72; sem ela, 8 com 0x40 e a
/// notícia, e sai), menos 1 se for a última; pela riqueza (0x5986b0: 2 → −3/−2, 1 → −2/−1, 0 com verba (0x598800) acima de
/// 250 × reputação → −1, −1 → +1 abaixo de 5, −2 → +1); pela divisão anterior (+0x5b: mais forte +4/+2, mais fraca −7/−3/−2)
/// ou pelos dias do técnico (abaixo de 365 +1; acima de 1095 −1 acima de 10); entre 1 e o número de clubes − 1; com a
/// marca 1, a média com +0x73 da linha entre +0x74 e +0x75. As marcas: 2 (vaga de campeão), 1 (vagas da UEFA, fora da
/// Austrália), 4 (+0xbe), 8 (+0xbf), 0x40 (metade de cima), 0x20 (fuga do rebaixamento, +0xc1) ou 0x10/0; sorteios do
/// presidente (0x80, 0x100, 0x200 pela meta) e a liga da copa ([0xae23d0], 0x400 ou a troca 0x80/0x200). A notícia
/// (0x6a4a20). Confianças abaixo de 6500 sobem a (5x + 27500 ou 25000) / 10; a riqueza em +1; a ficha do técnico (+0xa,
/// +0xc) sobe para a reputação do clube; o técnico novo (dos 16 últimos) ganha 1 em +0x59 até 20; zera +2, +3, +4, +5,
/// +0x12..+0x16, +0x28 e +0x40 & 7; as multas (0x5a5930 em [0xae2a3c]).
unsafe extern "thiscall" fn expectativas_da_temporada(this: usize, clube: usize, noticia: u32) {
    let ln = || cqc9_linha(this, clube);
    cqc9_pw(ln() + 0xe, 0);
    cqc9_pd(ln() + 0x2c, 0);
    let rep = cqc9_w(clube + 0x80) as i32;
    core::ptr::write_unaligned((ln() + 0x30) as *mut f64, rep.wrapping_mul(rep) as f64 * cqc9_f(0x9689e0));
    let l38 = ln() + 0x38;
    core::ptr::write_unaligned(l38 as *mut f64, cqc9_f(l38) * cqc9_f(0x9689e0));
    cqc9_pb(ln(), 0);
    let mut dias: i32 = 0;
    if cqc9_u(ln() + 2) & 1 == 0 && cqc9_d(clube + 0xcf) != 0 && cqc9_selecao(clube) == 0 {
        let tec = cqc9_d(clube + 0xcf);
        let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        dias = ate(0xae2c90, cqc9_d(tec + 0x3e) as u32, cqc9_d(tec + 0x42) as u32);
        let pres = || {
            let bf = cqc9_d(clube + 0xbf);
            if bf != 0 && cqc9_d(bf + 0x69) != 0 { Some(cqc9_b(cqc9_d(bf + 0x69) + 0x1d) as i32) } else { None }
        };
        let sair = if dias > 0xe42 && cqc9_w(ln() + 6) < 0xdac {
            match pres() {
                Some(_) => {
                    let r = cqc9_sortear(0x1e);
                    r > pres().unwrap_or(0)
                }
                None => cqc9_sortear(0x1e) > 10,
            }
        } else if dias > 0x721 && cqc9_w(ln() + 6) < 0xbb8 {
            match pres() {
                Some(_) => {
                    let r = cqc9_sortear(0x14);
                    r > pres().unwrap_or(0)
                }
                None => cqc9_sortear(0x14) > 10,
            }
        } else {
            false
        };
        if sair {
            crate::comparacao::ramo("cqc_9 saida pelo tempo");
            // o item de 6 bytes {1, 6, id do clube} copiado na lista [this+4]
            let mut item = [0u8; 8];
            item[0] = 1;
            item[1] = 6;
            item[2..6].copy_from_slice(&(cqc9_d(clube) as u32).to_le_bytes());
            let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x68d0d0usize);
            acrescentar(cqc9_d(this + 4), item.as_mut_ptr(), 6);
        }
    }
    if cqc9_selecao(clube) != 0 {
        let federacao: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x6a44a0usize);
        let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
        let mut feita = false;
        if segunda(clube) == 0 {
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7751e0usize);
            let ent = achar(cqc9_d(0xb74318), cqc9_d(clube + 0x53));
            let co = if ent != 0 { cqc9_d(cqc9_d(0xadadfc).wrapping_add((cqc9_i(ent) as usize).wrapping_mul(4))) } else { 0 };
            if co != 0 {
                let exp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cqc9_d(cqc9_d(co) + 0x70));
                let w = exp(co, clube) as u16;
                cqc9_pw(ln() + 0xe, w as i16);
                cqc9_pd(ln() + 0x2c, cqc9_d(ent) as u32);
                if noticia as u8 != 0 {
                    // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a44a0 lê a palavra
                    federacao(this, clube, cqc9_w(ln() + 0xe) as u16 as u32, cqc9_d(ln() + 0x2c) as u32);
                }
                feita = true;
            } else {
                cqc9_pw(ln() + 0xe, 0);
                cqc9_pd(ln() + 0x2c, 0xffff_ffff);
            }
        }
        if !feita && noticia as u8 != 0 {
            federacao(this, clube, 0, 0xffff_ffff);
        }
        cqc9_fim(this, clube);
        return;
    }
    // as vagas da UEFA da nação: campeão (o byte do 2º argumento) e as outras (o 3º)
    let mut vagas: u32 = clube as u32; // a original grava no lugar do argumento `clube` (só o byte é lido)
    let mut outras: u8 = 0;
    let uefa: unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) = core::mem::transmute(0x905c60usize);
    uefa(cqc9_d(0xde1f58), cqc9_d(clube + 0x53), &mut vagas, &mut outras);
    let vagas = vagas as u8 as i8 as i32;
    let outras = outras as i8 as i32;
    if cqc9_d(clube + 0xcf) == 0 {
        cqc9_fim(this, clube);
        return;
    }
    let div = cqc9_d(clube + 0x57);
    let nacao = cqc9_d(clube + 0x53);
    if div == 0 || nacao == 0 || cqc9_u(nacao + 0x11c) == 0 || cqc9_comp(clube) == 0 {
        cqc9_pw(ln() + 0xe, 0x10);
        cqc9_noticia(this, clube, noticia);
        cqc9_fim(this, clube);
        return;
    }
    let linha_da_liga: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let lg = linha_da_liga(cqc9_d(cqc9_comp(clube) + 8), clube);
    let mut copa: usize = 0;
    let id = cqc9_i(clube + 0x1db);
    if id > -1 && id != cqc9_i(0x9cf6f8) {
        let cid = cqc9_i(cqc9_d(0xae23d0).wrapping_add((id as usize).wrapping_mul(0x6b)));
        let co = cqc9_d(cqc9_d(0xadadfc).wrapping_add((cid as usize).wrapping_mul(4)));
        copa = linha_da_liga(cqc9_d(co + 8), clube);
    }
    if lg == 0 || cqc9_u(lg + 0x72) < 1 {
        cqc9_pd(ln() + 0x2c, 8);
        cqc9_pb(ln() + 0xe, cqc9_u(ln() + 0xe) | 0x40);
        let diretoria: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a4a20usize);
        // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a4a20 lê a palavra
        diretoria(this, clube, cqc9_w(ln() + 0xe) as u16 as u32);
        return;
    }
    let meta = || cqc9_i(ln() + 0x2c);
    let por = |v: i32| cqc9_pd(ln() + 0x2c, v as u32);
    por(cqc9_u(lg + 0x72) as i32); // cqc9: a meta pela posição de perigo
    let n = cqc9_w(cqc9_comp(clube) + 0x3e) as i32;
    if meta() == n {
        por(n - 1);
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let fin = financas(cqc9_d(0xae2a38), clube);
    crate::comparacao::ramo("cqc_9 meta");
    match faixa(fin) as i8 {
        2 => por(if meta() > 5 { meta() - 3 } else { meta() - 2 }),
        1 => por(if meta() > 5 { meta() - 2 } else { meta() - 1 }),
        0 => {
            let verba: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
            let v = verba(fin, 0, 0, 0, 1, 0);
            if v > (cqc9_w(clube + 0x80) as i32 * 250) as i64 && meta() > 5 {
                por(meta() - 1);
            }
        }
        -1 => {
            if meta() < 5 {
                por(meta() + 1);
            }
        }
        -2 => por(meta() + 1),
        _ => {}
    }
    let (d57, d5b) = (cqc9_d(clube + 0x57), cqc9_d(clube + 0x5b));
    if d57 != 0 && d5b != 0 && d57 != d5b {
        if cqc9_w(d57 + 0x69) > cqc9_w(d5b + 0x69) {
            if meta() > 10 {
                por(meta() + 1);
            }
            por(if meta() > 5 { meta() + 2 } else { meta() + 4 });
        } else {
            let x = meta();
            if x > 10 {
                por(x - 7);
            } else if x > 5 {
                por(x - 3);
            } else if x > 2 {
                por(x - 2);
            }
        }
    } else if dias < 0x16d {
        por(meta() + 1);
    } else if dias > 0x447 {
        if meta() > 10 {
            por(meta() - 1);
        }
    } else if dias > 0x721 {
        // (inalcançável: a original testa depois de dias ≤ 1095)
        if meta() > 10 {
            por(meta() - 2);
        }
        if meta() > 5 {
            por(meta() - 1);
        }
    }
    let x = meta();
    let x = if x < 1 {
        1
    } else {
        let lim = cqc9_w(cqc9_comp(clube) + 0x3e) as i32 - 1;
        if lim < x { lim } else { x }
    };
    por(x as i8 as i32);
    if cqc9_u(ln() + 2) & 1 != 0 && cqc9_u(lg + 0x74) >= 1 && cqc9_u(lg + 0x75) >= 1 {
        let v = (cqc9_u(lg + 0x73) as i32).wrapping_add(meta());
        por(cqc9_ftol(v as f64 * cqc9_f(0x968968)) as i8 as i32);
        let (lo, hi) = (cqc9_u(lg + 0x74) as i32, cqc9_u(lg + 0x75) as i32);
        let x = meta();
        let x = if lo > x { lo } else if hi < x { hi } else { x };
        por(x as i8 as i32);
    }
    // as marcas pela meta
    let co = cqc9_comp(clube);
    let ou = |m: u8| cqc9_pb(ln() + 0xe, cqc9_u(ln() + 0xe) | m);
    'marcas: {
        if cqc9_virtual(co, 0x98) == 0 && meta() <= vagas {
            ou(2);
            break 'marcas;
        }
        let na = cqc9_d(clube + 0x53);
        if na != 0 && cqc9_i(na) != cqc9_i(0x9cf224) && cqc9_virtual(cqc9_comp(clube), 0x98) == 0 && meta() <= vagas + outras {
            ou(1);
            break 'marcas;
        }
        let co = cqc9_comp(clube);
        if cqc9_virtual(co, 0x98) != 0 && meta() <= cqc9_b(cqc9_comp(clube) + 0xbe) as i32 {
            ou(4);
            break 'marcas;
        }
        let co = cqc9_comp(clube);
        let bf = cqc9_b(co + 0xbf);
        if bf != 0 && meta() <= bf as i32 {
            ou(8);
            break 'marcas;
        }
        if meta() <= cqc9_w(co + 0x3e) as i32 / 2 {
            cqc9_pw(ln() + 0xe, 0x40);
            break 'marcas;
        }
        if cqc9_virtual(co, 0x9c) != 0 {
            let co = cqc9_comp(clube);
            if meta() >= cqc9_w(co + 0x3e) as i32 - cqc9_b(co + 0xc1) as i32 {
                ou(0x20);
                break 'marcas;
            }
        }
        let r = cqc9_virtual(cqc9_comp(clube), 0x9c);
        cqc9_pw(ln() + 0xe, if r != 0 { 0x10 } else { 0 });
    }
    // os sorteios do presidente e a liga da copa
    let bf = cqc9_d(clube + 0xbf);
    if cqc9_u(ln() + 2) & 1 == 0 && cqc9_virtual(cqc9_comp(clube), 0x98) == 0 && bf != 0 && cqc9_d(bf + 0x69) != 0 {
        let p1d = || cqc9_b(cqc9_d(cqc9_d(clube + 0xbf) + 0x69) + 0x1d) as i32;
        crate::comparacao::ramo("cqc_9 sorteio do presidente");
        let r = cqc9_sortear(0x32); // cqc9: o sorteio do presidente
        if r > p1d() + meta() * 5 {
            ou(0x80);
        } else {
            let r = cqc9_sortear(0x4b);
            if r > p1d() + meta() * 5 {
                cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 1);
            } else {
                let r = cqc9_sortear(100);
                if r > p1d() + meta() * 5 {
                    cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 2);
                }
            }
        }
        if copa != 0 && cqc9_u(copa + 0x74) <= 1 {
            let t = (0x1d - p1d()) / 5;
            let p = cqc9_u(copa + 0x72) as i32;
            if p <= t {
                cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 4);
            } else if p <= t + 4 {
                cqc9_pw(ln() + 0xe, cqc9_w(ln() + 0xe) ^ 0x80);
                cqc9_pw(ln() + 0xe, cqc9_w(ln() + 0xe) ^ 0x200);
                cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 1);
            }
        }
    }
    cqc9_noticia(this, clube, noticia);
    cqc9_fim(this, clube);
}

/// A notícia das expectativas (0x6a4a20 com as marcas +0xe) quando pedida.
unsafe fn cqc9_noticia(this: usize, clube: usize, noticia: u32) {
    if noticia as u8 != 0 {
        let diretoria: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a4a20usize);
        // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a4a20 lê a palavra
        diretoria(this, clube, cqc9_w(cqc9_linha(this, clube) + 0xe) as u16 as u32);
    }
}

/// O fim comum da 0x6a3770 (a partir de 0x6a415f): as confianças, a riqueza, a ficha do técnico, as marcas zeradas e as
/// multas.
unsafe fn cqc9_fim(this: usize, clube: usize) {
    let ln = || cqc9_linha(this, clube);
    if cqc9_w(ln() + 6) < 0x1964 {
        crate::comparacao::ramo("cqc_9 confianca de volta");
        for (o, k) in [(6usize, 0x96d288usize), (8, 0x9688d0), (0xa, 0x9688d0), (0xc, 0x9688d0)] {
            let x = cqc9_w(ln() + o) as i32;
            let v = (x as f64 * cqc9_f(0x9688a0) + cqc9_f(k)) * cqc9_f(0x967880); // cqc9: a confiança de volta
            cqc9_pw(ln() + o, cqc9_ftol(v) as i16);
        }
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let r = if cqc9_selecao(clube) == 0 {
        let fin = financas(cqc9_d(0xae2a38), clube);
        if fin != 0 { faixa(fin) } else { 0 }
    } else {
        0
    };
    cqc9_pb(ln() + 1, r);
    let rep = || cqc9_w(clube + 0x80);
    let tec = cqc9_d(clube + 0xcf);
    if tec != 0 && cqc9_d(tec + 0x69) != 0 {
        let fx = cqc9_d(tec + 0x69);
        let a = cqc9_w(fx + 0xa);
        if a < rep() {
            let t = cqc9_ftol((rep() as i32 - a as i32) as f64 * cqc9_f(0x969518));
            cqc9_pw(fx + 0xa, a.wrapping_sub(t as i16));
            let fx = cqc9_d(cqc9_d(clube + 0xcf) + 0x69);
            if cqc9_w(fx + 0xa) > 0x2710 {
                cqc9_pw(fx + 0xa, 0x2710);
            }
            let fx = cqc9_d(cqc9_d(clube + 0xcf) + 0x69);
            let c = cqc9_w(fx + 0xc);
            let t = cqc9_ftol((rep() as i32 - c as i32) as f64 * cqc9_f(0x96d280));
            cqc9_pw(fx + 0xc, c.wrapping_sub(t as i16));
            let fx = cqc9_d(cqc9_d(clube + 0xcf) + 0x69);
            if cqc9_w(fx + 0xc) > 0x2710 {
                cqc9_pw(fx + 0xc, 0x2710);
            }
        }
    }
    let tec = cqc9_d(clube + 0xcf);
    if tec != 0 && cqc9_d(tec + 0x69) != 0 {
        let fx = cqc9_d(tec + 0x69);
        let a = cqc9_w(fx + 0xa);
        if rep() > a {
            let t = cqc9_ftol((rep() as i32 - a as i32) as f64 * cqc9_f(0x969518));
            cqc9_pw(fx + 0xa, a.wrapping_sub(t as i16));
        }
    }
    let tec = cqc9_d(clube + 0xcf);
    if tec != 0 && cqc9_i(tec) >= cqc9_i(0xae2364).wrapping_sub(0x10) && cqc9_b(tec + 0x59) < 0x14 {
        cqc9_pb(tec + 0x59, cqc9_u(tec + 0x59).wrapping_add(1));
    }
    cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & 0xfe);
    cqc9_pb(ln() + 4, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xe1);
    cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & 0xdf);
    cqc9_pd(ln() + 0x28, 0);
    cqc9_pb(ln() + 5, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xfe);
    cqc9_pb(ln() + 0x40, cqc9_u(ln() + 0x40) & 7);
    cqc9_pw(ln() + 0x12, 0);
    cqc9_pw(ln() + 0x14, 0);
    cqc9_pw(ln() + 0x16, 0);
    let multas = cqc9_d(0xae2a3c);
    if multas != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5a5930usize);
        f(multas, clube);
    }
}

// ponytail: acrescenta na lista das saídas (0x68d0d0, aloca), posta as notícias (0x6a44a0, 0x6a4a20) e sorteia: não roda
// duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_expectativas_da_temporada(this: usize, clube: usize, noticia: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1530)(this, clube, noticia)
}

/// A pessoa de índice `k` ([0xae23bc] + k × 0x6e).
unsafe fn cqc9_pessoa(k: i32) -> usize {
    cqc9_d(0xae23bc).wrapping_add((k as usize).wrapping_mul(0x6e))
}

/// O byte +0xb do estado da pessoa de índice `k` ([0xae2d0c] + k × 0x4f).
unsafe fn cqc9_estado(k: i32) -> u8 {
    cqc9_u(cqc9_d(0xae2d0c).wrapping_add((k as usize).wrapping_mul(0x4f)) + 0xb)
}

/// A nota do candidato (0x69d1b0: thiscall, 4: pessoa, clube, 0, 5; original).
unsafe fn cqc9_nota(this: usize, p: usize, clube: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x69d1b0usize);
    f(this, p, clube, 0, 5)
}

/// 0x69f810 (thiscall, 4: pessoa, clube e a nota mínima em double, em dois dwords).
unsafe fn cqc9_serve(this: usize, p: usize, clube: usize, minimo: f64) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(0x69f810usize);
    f(this, p, clube, minimo)
}

/// 0x4cf6d0 (thiscall em [0xae1050], 1): a pessoa tem ficha de contrato.
unsafe fn cqc9_com_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    f(cqc9_d(0xae1050), p)
}

/// 0x69fc60 (thiscall, 1): a pessoa já está numa lista de vagas.
unsafe fn cqc9_na_lista(this: usize, p: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x69fc60usize);
    f(this, p)
}

/// O item de 0x26 bytes da vaga (o que a original monta em [esp+0x1c]): +0 achou um livre, +1 0, +2 o id dele, +6 o
/// índice do escolhido, +0xa o id do técnico que saiu, +0xe o id do clube, +0x12 a nota anterior, +0x16 a data da
/// chegada e +0x1e a data do anúncio (8 bytes cada, 0x54a110).
struct Cqc9Vaga([u8; 0x28]);

impl Cqc9Vaga {
    fn b(&mut self, o: usize, v: u8) {
        self.0[o] = v;
    }
    fn d(&mut self, o: usize, v: i32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn rd(&self, o: usize) -> i32 {
        i32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
    }
    fn w(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn rw(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.0[o], self.0[o + 1]])
    }
}

/// { 0x69ba50 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::add_job_vacancy(clube, técnico que saiu)`. Clube sem divisão e reputação abaixo de 5000: nada;
/// seleção B com ranking (+0x88 da nação) até 1000 ou principal até 100: nada. A diretoria recomeça (as quatro
/// confianças 5000, +0xe = 0x10, as marcas zeradas). O escolhido: de todas as pessoas com ficha de técnico (+0x69; os 16
/// últimos, os humanos, só com +0x5f), fora o que saiu, sem estado 0x30, sem contrato (0x4cf6d0), fora das listas
/// (0x69fc60), da mesma nação ou com +0xa ≥ 3750, do mesmo continente (+0x75) ou com +0xc ≥ 4250 e o clube com
/// reputação ≥ 4750, que serve para o cargo (0x69f810 com 3,5), fora de jogadores com contrato em vigor nos cargos
/// 11/12/13/15, só técnico de seleção (+0x3d = 5) numa seleção: a maior nota (0x69d1b0). Sem ninguém: o auxiliar (+0xd3),
/// o primeiro de +0x19f ou, de trás para a frente a partir do 17º dos últimos, a primeira pessoa livre nos cargos 0, 5, 6,
/// 7, 8, 11, 12, 15 (sem dar a volta: a caixa "Can't find a manager" e nada). Seleção ou clube grande (reputação acima
/// de 4500 + sorteio(2000)): sorteia tantas pessoas quanto o número delas e fica com a melhor livre (com 150 dias no
/// cargo, 3,15). O humano rejeitado pelo clube (0x547e70). As datas: antes do começo do jogo, de 1 a 7 dias depois dele (o
/// anúncio) e a chegada de 1 a 7 dias depois; senão, o anúncio de 1 a 3 dias e a chegada de 4 a 8 dias (7 a 11 se quem
/// saiu é humano; 10 a 14 com a notícia da oferta 0x6a0540 ao clube humano do escolhido), às vezes mais 0 a 19; a
/// virada do ano. O item vai para a lista [this+8] (0x68d0d0, o cursor guardado); os humanos (0x5fd210, 0x5fea50) de
/// clube menor se candidatam (0x69ca00).
unsafe extern "thiscall" fn vaga_de_tecnico(this: usize, clube: usize, saiu: usize) {
    // ponytail: a nota anterior (+0x12) fica sem valor na original quando o escolhido é o auxiliar ou o de +0x19f (sobra de
    // pilha); aqui 0, um valor que o jogo também produz
    let mut v = Cqc9Vaga([0; 0x28]);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(v.0.as_mut_ptr().add(0x16));
    data_padrao(v.0.as_mut_ptr().add(0x1e));
    let mut melhor: i32 = -1_000_000;
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if cqc9_selecao(clube) == 0 && cqc9_d(clube + 0x57) == 0 && cqc9_w(clube + 0x80) < 0x1388 {
        return;
    }
    if cqc9_selecao(clube) != 0 {
        if segunda(clube) != 0 && cqc9_i(cqc9_d(clube + 0x53) + 0x88) <= 0x3e8 {
            return;
        }
        if segunda(clube) == 0 && cqc9_i(cqc9_d(clube + 0x53) + 0x88) <= 0x64 {
            return;
        }
    }
    v.d(0xe, cqc9_i(clube));
    v.d(6, -1);
    let ln = || cqc9_linha(this, clube);
    for o in [6usize, 8, 0xa, 0xc] {
        cqc9_pw(ln() + o, 0x1388);
    }
    cqc9_pb(ln(), 0);
    cqc9_pw(ln() + 0xe, 0x10);
    for o in [0x10usize, 0x12, 0x14, 0x16] {
        cqc9_pw(ln() + o, 0);
    }
    for m in [0xfeu8, 0xfd, 0xfb, 0xf7, 0xef] {
        cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & m);
    }
    cqc9_pb(ln() + 4, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xe1);
    for m in [0xdfu8, 0xbf, 0x7f] {
        cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & m);
    }
    cqc9_pd(ln() + 0x28, 0);
    cqc9_pb(ln() + 5, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xfe);
    cqc9_pb(ln() + 0x40, cqc9_u(ln() + 0x40) & 7);
    v.d(0xa, if saiu != 0 { cqc9_i(saiu) } else { -1 });
    // o melhor de todos
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let mut k = 0i32;
    while k < cqc9_i(0xae2364) {
        let p = cqc9_pessoa(k);
        let ultimos = cqc9_i(0xae2364) - 0x10;
        let ok = 'c: {
            if cqc9_d(p + 0x69) == 0 || (k >= ultimos && cqc9_u(p + 0x5f) == 0) || saiu == p {
                break 'c false;
            }
            if k >= ultimos && cqc9_selecao(clube) != 0 && cqc9_d(p + 0x1a) != cqc9_d(clube + 0x53)
                && cqc9_w(cqc9_d(p + 0x69) + 0xc) < 0xfa0
            {
                break 'c false;
            }
            if k >= cqc9_i(0xae2364) - 0x10 && dirigivel(clube) == 0 {
                break 'c false;
            }
            if cqc9_estado(k) & 0x30 != 0 || cqc9_com_contrato(p) != 0 || cqc9_na_lista(this, p) != 0 {
                break 'c false;
            }
            let (n1, n2) = (cqc9_d(clube + 0x53), cqc9_d(p + 0x1a));
            if n1 == 0 || n2 == 0 || (n1 != n2 && cqc9_w(cqc9_d(p + 0x69) + 0xa) < 0xea6) {
                break 'c false;
            }
            if cqc9_u(n1 + 0x75) != cqc9_u(n2 + 0x75)
                && (cqc9_w(cqc9_d(p + 0x69) + 0xc) < 0x109a || cqc9_w(clube + 0x80) < 0x128e)
            {
                break 'c false;
            }
            if cqc9_serve(this, p, clube, 3.5) == 0 {
                break 'c false;
            }
            if cqc9_d(p + 0x61) != 0 && cqc9_d(p + 0x39) != 0 && matches!(cqc9_u(p + 0x3d), 0xb | 0xc | 0xd | 0xf) {
                let c = contrato(cqc9_d(0xae1050), p);
                if c != 0 && (cqc9_u(c + 0x49) != 0 || cqc9_u(c + 0x20) == 0) {
                    break 'c false;
                }
            }
            if cqc9_selecao(clube) != 0 && cqc9_u(p + 0x3d) != 5 {
                break 'c false;
            }
            true
        };
        if ok {
            let s = cqc9_nota(this, p, clube);
            if s > melhor {
                crate::comparacao::ramo("cqc_9 candidato melhor");
                v.d(6, k); // cqc9: o candidato escolhido
                v.d(0x12, melhor);
                melhor = s;
            }
        }
        k += 1;
    }
    if v.rd(6) == -1 {
        let ajudante = cqc9_d(clube + 0xd3);
        let lista = cqc9_d(clube + 0x19f);
        if ajudante != 0 {
            v.d(6, cqc9_i(ajudante));
        } else if lista != 0 {
            v.d(6, cqc9_i(lista));
        } else {
            // de trás para a frente, a partir do 17º dos últimos (a original parte do índice do ponteiro de quem saiu)
            let mut primeira = true;
            let mut i = saiu as i32;
            let mut inicio = 0i32;
            loop {
                if primeira {
                    primeira = false;
                    i = cqc9_i(0xae2364) - 0x11;
                    inicio = i;
                } else {
                    i -= 1;
                    if i < 0 {
                        i = cqc9_i(0xae2364) - 0x10;
                    }
                    if i == inicio {
                        let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
                        fmt(0xde1f64, if segunda(clube) != 0 { 0xa04d8c } else { 0xa04d28 }, clube + 4);
                        return;
                    }
                }
                v.d(6, i);
                if cqc9_estado(i) & 0x30 != 0 || i >= cqc9_i(0xae2364) - 0x10 {
                    v.d(6, -1);
                    continue;
                }
                let p = cqc9_pessoa(i);
                if cqc9_d(p + 0x69) == 0 || !matches!(cqc9_u(p + 0x3d), 0 | 0xb | 0xc | 5 | 8 | 0xf | 7 | 6) {
                    v.d(6, -1);
                    continue;
                }
                if cqc9_com_contrato(p) != 0 || cqc9_na_lista(this, cqc9_pessoa(v.rd(6))) != 0 {
                    v.d(6, -1);
                    continue;
                }
                let q = cqc9_pessoa(v.rd(6));
                if cqc9_d(q + 0x24) != 0 && cqc9_u(q + 0x28) == 5 {
                    v.d(6, -1);
                    continue;
                }
                if cqc9_selecao(clube) == 0 {
                    let q = cqc9_pessoa(v.rd(6));
                    let c = cqc9_d(q + 0x39);
                    if c != 0 {
                        let na = cqc9_d(c + 0x53);
                        let d57 = cqc9_d(c + 0x57);
                        let forte = (na != 0 && cqc9_u(na + 0x11c) != 0 && d57 != 0
                            && cqc9_d(cqc9_d(0xadadfc).wrapping_add((cqc9_i(d57) as usize).wrapping_mul(4))) != 0)
                            || cqc9_w(c + 0x80) > 0x1964;
                        if forte && cqc9_u(q + 0x3d) == 5 {
                            v.d(6, -1);
                            continue;
                        }
                    }
                }
                v.d(0x12, 100);
                if v.rd(6) != -1 {
                    break;
                }
            }
        }
    }
    v.b(1, 0);
    v.b(0, 0);
    v.d(2, -1);
    let grande = cqc9_selecao(clube) != 0 || (cqc9_w(clube + 0x80) > 0x1194 && {
        let r = cqc9_sortear(0x7d0);
        cqc9_w(clube + 0x80) as i32 > r + 0x1194
    });
    if grande {
        let mut melhor: i32 = -1_000_000;
        let mut k = 0i32;
        while k < cqc9_i(0xae2364) {
            let r = cqc9_sortear(cqc9_i(0xae2364));
            let p = cqc9_pessoa(r);
            // ponytail: o estado lido é o do contador `k`, não o da pessoa sorteada, como na original
            if cqc9_d(p + 0x69) != 0 && saiu != p && cqc9_estado(k) & 0x30 == 0
                && cqc9_serve(this, p, clube, f64::from_bits(0x4009_3333_3333_3333)) != 0
                && (cqc9_i(p) < cqc9_i(0xae2364) - 0x10 || cqc9_u(p + 0x5f) != 0)
            {
                let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                if ate(0xae2c90, cqc9_d(p + 0x3e) as u32, cqc9_d(p + 0x42) as u32) >= 0x96 && cqc9_na_lista(this, p) == 0 {
                    let s = cqc9_nota(this, p, clube);
                    if s > melhor {
                        v.b(0, 1);
                        v.d(2, cqc9_i(p));
                        melhor = s;
                    }
                }
            }
            k += 1;
        }
    }
    let ultimos = cqc9_i(0xae2364) - 0x10;
    if cqc9_i(cqc9_pessoa(v.rd(6))) >= ultimos && saiu != 0 && cqc9_i(saiu) < ultimos {
        let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
        rejeitar(saiu, clube, 1);
    }
    // as datas: +0x16 a chegada (dia +0x16, ano +0x18), +0x1e o anúncio (dia +0x1e, ano +0x20)
    let (dia0, ano0) = (cqc9_w(0xdd6ec8), cqc9_w(0xdd6eca) as u16);
    let (dia, ano) = (cqc9_w(0xae2c90), cqc9_w(0xae2c92) as u16);
    let antes_do_comeco = ano0 > ano || (ano0 == ano && dia0 > dia);
    if antes_do_comeco && segunda(clube) == 0 {
        v.w(0x20, ano0);
        let r = cqc9_sortear(7);
        v.w(0x1e, (r as u16).wrapping_add(dia0 as u16).wrapping_add(1));
        if v.rw(0x1e) as i16 > 0x16c {
            let r = cqc9_sortear(5);
            v.w(0x20, v.rw(0x20).wrapping_add(1));
            v.w(0x1e, r as u16);
        }
        v.w(0x18, v.rw(0x20));
        let r = cqc9_sortear(7);
        v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(1));
        if cqc9_sortear(200) == 0 {
            let r = cqc9_sortear(0x14);
            v.w(0x16, v.rw(0x16).wrapping_add(r as u16));
        }
    } else {
        v.w(0x20, ano);
        let r = cqc9_sortear(3);
        v.w(0x1e, (r as u16).wrapping_add(dia as u16).wrapping_add(1));
        if v.rw(0x1e) as i16 > 0x16c {
            let r = cqc9_sortear(5);
            v.w(0x20, v.rw(0x20).wrapping_add(1));
            v.w(0x1e, r as u16);
        }
        v.w(0x18, v.rw(0x20));
        let q = cqc9_pessoa(v.rd(6));
        let c = cqc9_d(q + 0x39);
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if c != 0 && humano(c, 0, 1, 0, 0) != 0 && cqc9_u(cqc9_pessoa(v.rd(6)) + 0x3d) != 5 {
            let oferta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x6a0540usize);
            oferta(this, cqc9_pessoa(v.rd(6)), clube);
            let r = cqc9_sortear(5);
            v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(10));
        }
        let r = cqc9_sortear(5);
        let mais = if saiu != 0 && cqc9_i(saiu) >= cqc9_i(0xae2364) - 0x10 { 7 } else { 4 };
        v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(mais)); // cqc9: a chegada do técnico
        if cqc9_sortear(500) == 0 {
            let r = cqc9_sortear(0x14);
            v.w(0x16, v.rw(0x16).wrapping_add(r as u16));
        }
    }
    if v.rw(0x16) as i16 > 0x16c {
        let r = cqc9_sortear(5);
        v.w(0x18, v.rw(0x18).wrapping_add(1));
        v.w(0x16, (r as u16).wrapping_add(4));
    }
    let lista = cqc9_d(this + 8);
    let cursor = cqc9_d(lista + 0xa);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x68d0d0usize);
    acrescentar(lista, v.0.as_mut_ptr(), 0x26);
    cqc9_pd(cqc9_d(this + 8) + 0xa, cursor as u32);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    if tecnico(clube) == 0 && dirigivel(clube) != 0 && cqc9_selecao(clube) == 0 {
        let ativo: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
        let quer: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
        let candidatar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ca00usize);
        let mut i = cqc9_i(0xae2364) - 0x10;
        while i < cqc9_i(0xae2364) {
            let p = cqc9_pessoa(i);
            if cqc9_u(p + 0x5f) != 0 && ativo(cqc9_d(0xb63c98), p) != 0 && quer(cqc9_d(0xb63c98), 1, p) != 0 {
                let c = cqc9_d(p + 0x39);
                if c == 0 || cqc9_w(c + 0x80) < cqc9_w(clube + 0x80) {
                    candidatar(this, clube, p);
                }
            }
            i += 1;
        }
    }
}

// ponytail: acrescenta a vaga na lista [this+8] (0x68d0d0, aloca), sorteia, posta a notícia da oferta e chama as
// candidaturas (0x69ca00): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas
// mutações.
unsafe extern "thiscall" fn comparar_vaga_de_tecnico(this: usize, clube: usize, saiu: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1531)(this, clube, saiu)
}

/// { 0x6a0540 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_job_offer_news(pessoa, clube)`: com a pessoa empregada (+0x39) e o clube, a partir do começo
/// do jogo ([0xdd6ec8]), a notícia 0x1775 (0x77bbe0) com o id da pessoa, os três nomes (+0x33 de +4, +8, +0xc), o id do
/// clube onde ela está, o do clube que oferece e o cargo (+0x3d), ao técnico do clube dela (0x7861e0).
unsafe extern "thiscall" fn noticia_da_oferta_de_emprego(_this: usize, pessoa: usize, clube: usize) {
    if pessoa == 0 || cqc9_d(pessoa + 0x39) == 0 || clube == 0 {
        return;
    }
    let dia = cqc9_w(0xdd6ec8);
    let ano = cqc9_w(0xdd6eca) as u16;
    let hoje = cqc9_w(0xae2c92) as u16;
    if ano > hoje || (ano == hoje && dia > cqc9_w(0xae2c90)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cqc_9 noticia da oferta");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1775, 0);
    gravar(n, 0, cqc9_d(pessoa) as u32);
    gravar(n, 1, cqc9_d(cqc9_d(pessoa + 4) + 0x33) as u32);
    gravar(n, 2, cqc9_d(cqc9_d(pessoa + 8) + 0x33) as u32);
    gravar(n, 3, cqc9_d(cqc9_d(pessoa + 0xc) + 0x33) as u32);
    gravar(n, 4, cqc9_d(cqc9_d(pessoa + 0x39)) as u32);
    gravar(n, 5, cqc9_d(clube) as u32);
    gravar(n, 6, cqc9_b(pessoa + 0x3d) as i32 as u32);
    ao_tecnico(cqc9_d(0xb7431c), n, cqc9_d(pessoa + 0x39));
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_oferta_de_emprego(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1532)(this, pessoa, clube)
}

// ---------- 266a leva: os boatos de transferência, a notícia de que o clube não vende o jogador e a notícia do título (media.cpp) ----------

unsafe fn cqc10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc10_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqc10_u(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn cqc10_clube(id: i32) -> usize {
    cqc10_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245))
}

unsafe fn cqc10_humano(c: usize, a: u32, b: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(c, a, b, 0, 0)
}

/// Os clubes humanos que recebem notícias do jogador (0x8d1da0 em [0xde1f54]: 4 na pilha, ret 0x10; o ecx da original é
/// o gerenciador das transferências).
unsafe fn cqc10_destinos(jogador: usize, marcas: u32, saida: *mut usize, max: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    f(cqc10_d(0xde1f54), jogador, marcas, saida, max)
}

/// O id e os três nomes (+0x33 de +4, +8, +0xc) de uma pessoa, nos índices `k`..`k+3` da notícia.
unsafe fn cqc10_nomes(gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    gravar(n, k, cqc10_d(p) as u32);
    gravar(n, k + 1, cqc10_d(cqc10_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cqc10_d(cqc10_d(p + 8) + 0x33) as u32);
    gravar(n, k + 3, cqc10_d(cqc10_d(p + 0xc) + 0x33) as u32);
}

/// Acrescenta `c` aos destinos se ainda não estiver lá (até 30).
fn cqc10_acrescentar(dest: &mut [usize; 30], n: &mut u8, c: usize) {
    if !dest[..(*n as i8).max(0) as usize].contains(&c) {
        dest[*n as i8 as usize] = c;
        *n = n.wrapping_add(1);
    }
}

/// { 0x754a30 } (thiscall, ret 0, media.cpp, quadro SEH sem destrutor no caminho normal):
/// `Media::process_transfer_interest()`, pela rotina diária (0x5cdd8f). Para cada registro de interesse (vetor de 25
/// bytes em [this+0xc], conta [this+0x14]: +0 jogador, +4 clube, +8 data, +0x10 estado, +0x11, +0x15): vencido (mais de
/// 31 dias, 0x54c5b0), sem técnico no clube dono (o +0xcf do clube do contrato do jogador, guardado da volta anterior
/// quando ele não tem contrato) ou sem ficha: sai (o último toma o lugar). Senão, se o clube pode chegar ao jogador
/// (0x8ce990): estado 0 e sorteio(25) zero dão o boato: os até 5 clubes interessados nele (registros do mesmo jogador
/// ainda em 0, marcados 1; um já marcado cancela), a notícia 0xfbc (0x77bbe0) com o jogador, o dono, o pedido de
/// transferência (abaixo de 15 em +0x5b e +0x59, reputação acima do sorteio(12500) e o +0x15 acima de 12,5 × +0x59 + 500:
/// 0x24, senão sorteio(3)), +0x11, a resposta do técnico da IA (0x735870), as forças (0x546ab0), as posições (0x5b7d20),
/// o técnico do dono, a lista de transferências (0x544520) e os interessados por reputação; aos humanos (0x8d1da0, os
/// interessados e os humanos da mesma divisão ou da mesma nação e 3000 abaixo) (0x7861e0); com o pedido, a reação do
/// elenco (0x755340); com a resposta, o aviso ao rival (0x758010) e a resposta à história (0x741020). No estado 1, o
/// dono da IA, a data do registro depois de +0x3e do jogador, o vínculo sem +0x49 e fora da lista, felicidade acima de
/// sorteio(50) + 40 e sorteio(30) zero: a notícia de que não vende (0x587600) e os registros dele vão a 2. No fim, o
/// vetor é ordenado (0x945333 com 0x757ea0).
unsafe extern "thiscall" fn boatos_de_transferencia(this: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let resposta: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x735870usize);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    if cqc10_i(this + 0x14) == 0 {
        return;
    }
    // ponytail: o técnico do dono fica da volta anterior quando o jogador não tem contrato (a original não o regrava);
    // na primeira volta a original lê a pilha sem valor: aqui 0
    let mut tec_dono: usize = 0;
    let mut i = 0i32;
    while i < cqc10_i(this + 0x14) {
        let r = cqc10_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(25));
        let pessoa = cqc10_d(0xae23bc).wrapping_add((cqc10_i(r) as usize).wrapping_mul(0x6e));
        let clube = cqc10_clube(cqc10_i(r + 4));
        let c = contrato_de(cqc10_d(0xae1050), pessoa);
        core::ptr::write_unaligned(0xae1080 as *mut usize, c);
        let (mut dono, mut vinc) = (0usize, 0usize);
        if c != 0 {
            dono = cqc10_clube(cqc10_i(c + 4));
            if dono != 0 {
                vinc = vinculo(cqc10_d(0xae1050), pessoa, dono);
                tec_dono = cqc10_d(dono + 0xcf);
            }
        }
        let mut hoje = [0u8; 8];
        core::ptr::copy_nonoverlapping(0xae2c90 as *const u8, hoje.as_mut_ptr(), 8);
        let mut fim = [0u8; 8];
        let p = mais(r + 8, fim.as_mut_ptr(), 0x1f); // cqc10: o interesse vale 31 dias
        crate::comparacao::ramo("cqc_10 interesse");
        let (hd, ha) = (i16::from_le_bytes([hoje[0], hoje[1]]), u16::from_le_bytes([hoje[2], hoje[3]]));
        let vencido = ha > cqc10_w(p + 2) as u16 || (ha == cqc10_w(p + 2) as u16 && hd > cqc10_w(p));
        if vencido || tec_dono == 0 || cqc10_d(pessoa + 0x61) == 0 {
            let n = cqc10_i(this + 0x14);
            let ultimo = cqc10_d(this + 0xc).wrapping_add((n as usize).wrapping_mul(25)).wrapping_sub(25);
            core::ptr::copy(ultimo as *const u8, r as *mut u8, 25);
            core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1);
            continue; // o último veio para o lugar i: a original volta um e avança um
        }
        if chegar(cqc10_d(0xde1f54), pessoa, clube, 0, 0) == 0 {
            i += 1;
            continue;
        }
        if cqc10_u(r + 0x10) == 0 && {
            crate::comparacao::ramo("cqc_10 sorteio do boato");
            cqc10_sortear(0x19) == 0 // cqc10: o sorteio do boato
        } {
            // os clubes interessados no jogador
            let mut ids = [-1i32; 5];
            ids[0] = cqc10_i(r + 4);
            *((r + 0x10) as *mut u8) = 1;
            let mut cnt: i8 = 1;
            let mut cancelar = false;
            let mut j = 0i32;
            while j < cqc10_i(this + 0x14) && cnt < 5 {
                let rj = cqc10_d(this + 0xc).wrapping_add((j as usize).wrapping_mul(25));
                if j != i && cqc10_i(rj) == cqc10_i(r) {
                    if cqc10_u(rj + 0x10) != 0 {
                        cancelar = true;
                        break;
                    }
                    ids[cnt as usize] = cqc10_i(rj + 4);
                    cnt += 1;
                    *((rj + 0x10) as *mut u8) = 1;
                }
                j += 1;
            }
            if cancelar {
                i += 1;
                continue;
            }
            crate::comparacao::ramo("cqc_10 boato");
            let mut noticia = [0u32; 0x3a]; // a notícia local
            let n = noticia.as_mut_ptr() as usize;
            construir(n, 0xfbc, 0);
            cqc10_nomes(gravar, n, 0, pessoa);
            gravar(n, 4, if dono != 0 { cqc10_d(dono) as u32 } else { 0xffff_ffff });
            let mut pede = false;
            let mut sai = false;
            if cqc10_b(pessoa + 0x5b) < 0xf && cqc10_b(pessoa + 0x59) < 0xf {
                crate::comparacao::ramo("cqc_10 pedido de transferencia");
                let rr = cqc10_sortear(0x30d4); // cqc10: o sorteio da reputação para o pedido
                if (cqc10_w(cqc10_d(pessoa + 0x61) + 0xd) as i32) > rr {
                    let lim = cqc10_b(pessoa + 0x59) as i32 as f64 * 12.5 + 500.0;
                    if lim < cqc10_i(r + 0x15) as f64 {
                        gravar(n, 5, 0x24);
                        pede = true;
                        sai = true;
                    }
                }
            }
            if !sai {
                let s = cqc10_sortear(3);
                gravar(n, 5, s as u32);
            }
            gravar(n, 6, cqc10_d(r + 0x11) as u32);
            let resp = if dono != 0 && cqc10_humano(dono, 0, 1) == 0 {
                resposta(this, cqc10_d(dono + 0xcf), pessoa, 0x1f)
            } else {
                0
            };
            gravar(n, 7, resp);
            gravar(n, 8, forcas(pessoa, dono));
            gravar(n, 9, posicoes(cqc10_d(pessoa + 0x61)) as i16 as i32 as u32);
            if tec_dono != 0 {
                cqc10_nomes(gravar, n, 0xa, tec_dono);
            } else {
                for k in 0xa..=0xd {
                    gravar(n, k, 0xffff_ffff);
                }
            }
            gravar(n, 0xe, (listada(pessoa, 0) != 0) as u32);
            gravar(n, 0xf, cnt as i32 as u32);
            // os interessados por reputação (do maior para o menor)
            let rep = |id: i32| cqc10_w(cqc10_clube(id) + 0x80);
            let c = cnt as usize;
            for a in 0..c.saturating_sub(1) {
                for b in a + 1..c {
                    if rep(ids[b]) > rep(ids[a]) {
                        ids.swap(a, b);
                    }
                }
            }
            for (k, &id) in ids.iter().take(c).enumerate() {
                gravar(n, 0x10 + k as i8, id as u32);
            }
            // os destinos
            let mut dest = [0usize; 30];
            let mut nd: u8 = if cqc10_d(0xde1f54) != 0 { cqc10_destinos(pessoa, 0xda, dest.as_mut_ptr(), 0x1e) } else { 0 };
            for &id in ids.iter().take(c) {
                if (nd as i8) >= 0x1e {
                    break;
                }
                cqc10_acrescentar(&mut dest, &mut nd, cqc10_clube(id));
            }
            let total = cqc10_i(0xae2364);
            for q in 0..16i32 {
                let h = cqc10_d(0xae23bc).wrapping_add(((total - 1 - q) as usize).wrapping_mul(0x6e));
                let hc = cqc10_d(h + 0x39);
                if cqc10_u(h + 0x5f) == 0 || hc == 0 {
                    continue;
                }
                for &id in ids.iter().take(c) {
                    if (nd as i8) >= 0x1e {
                        break;
                    }
                    let ci = cqc10_clube(id);
                    let hc = cqc10_d(h + 0x39);
                    let junto = cqc10_d(ci + 0x57) == cqc10_d(hc + 0x57)
                        || (cqc10_d(ci + 0x53) == cqc10_d(hc + 0x53) && (cqc10_w(ci + 0x80) as i32 - 0xbb8) > cqc10_w(hc + 0x80) as i32);
                    if junto {
                        cqc10_acrescentar(&mut dest, &mut nd, hc);
                    }
                }
            }
            for &d in dest.iter().take((nd as i8).max(0) as usize) {
                ao_tecnico(cqc10_d(0xb7431c), n, d);
            }
            if pede {
                let reacao: unsafe extern "thiscall" fn(usize, u32, usize, usize, *mut i32, u32) = core::mem::transmute(0x755340usize);
                reacao(this, 0x1f, pessoa, dono, ids.as_mut_ptr(), cnt as u8 as u32);
            }
            if dono != 0 && cqc10_humano(dono, 0, 1) == 0 && resp != 0 {
                if clube != 0 {
                    let rival: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x758010usize);
                    rival(this, pessoa, clube, dono, resp);
                }
                let historia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x741020usize);
                historia(this, pessoa, dono, 0x1f, resp);
            }
            i += 1;
            continue;
        }
        // estado 1: o dono da IA pode avisar que não vende
        if cqc10_u(r + 0x10) == 1 && dono != 0 && cqc10_humano(dono, 1, 1) == 0 {
            let (rd, ra) = (cqc10_w(r + 8), cqc10_w(r + 0xa) as u16);
            let (pd, pa) = (cqc10_w(pessoa + 0x3e), cqc10_w(pessoa + 0x40) as u16);
            let depois = ra > pa || (ra == pa && rd > pd);
            if depois && vinc != 0 && cqc10_u(vinc + 0x49) == 0 && listada(pessoa, 0) == 0 {
                let feliz = cqc10_b(vinc + 0x3d) as i32;
                let s = cqc10_sortear(0x32);
                if feliz > s + 0x28 && cqc10_sortear(0x1e) == 0 {
                    crate::comparacao::ramo("cqc_10 nao vende");
                    let nao_vende: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(0x587600usize);
                    nao_vende(pessoa, dono, cqc10_d(dono + 0xbf), 0);
                    let mut j = 0i32;
                    while j < cqc10_i(this + 0x14) {
                        let rj = cqc10_d(this + 0xc).wrapping_add((j as usize).wrapping_mul(25));
                        if cqc10_i(rj) == cqc10_i(cqc10_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(25))) {
                            *((rj + 0x10) as *mut u8) = 2;
                        }
                        j += 1;
                    }
                }
            }
        }
        i += 1;
    }
    let n = cqc10_i(this + 0x14);
    if n != 0 {
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cqc10_d(this + 0xc), n, 0x19, 0x757ea0);
    }
}

// ponytail: tira registros (o vetor), sorteia, constrói e entrega notícias e chama as reações e respostas (0x755340,
// 0x758010, 0x741020, 0x587600): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.
unsafe extern "thiscall" fn comparar_boatos_de_transferencia(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1533)(this)
}

/// { 0x587600 } (cdecl, media.cpp no fluxo; european_cup.cpp no mapa, sem outro chamador; quadro SEH sem destrutor no
/// caminho normal): `news_create_not_for_sale_story(jogador, clube, presidente, outro)`: com ficha e clube, a notícia
/// 0xfbe (0x77bbe0) com o jogador, o outro clube (ou −1), o clube e um sorteio(3); o presidente, se gosta do jogador
/// (0x547810) ou com sorteio(+0x16 da ficha dele) acima de sorteio(20), senão −1 no 7; reputação (+0xd da ficha) acima de
/// 8000: a todos os técnicos (0x786780), senão aos humanos interessados (0x8d1da0 com 0xd8, até 30, 0x7861e0).
unsafe extern "C" fn noticia_nao_vende(jogador: usize, clube: usize, presidente: usize, outro: usize) {
    if jogador == 0 || cqc10_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let mut dest = [0usize; 30];
    let nd: u8 = if cqc10_d(0xde1f54) != 0 { cqc10_destinos(jogador, 0xd8, dest.as_mut_ptr(), 0x1e) } else { 0 };
    crate::comparacao::ramo("cqc_10 noticia nao vende");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0xfbe, 0);
    cqc10_nomes(gravar, n, 0, jogador);
    gravar(n, 4, if outro != 0 { cqc10_d(outro) as u32 } else { 0xffff_ffff });
    gravar(n, 5, cqc10_d(clube) as u32);
    let s = cqc10_sortear(3);
    gravar(n, 6, s as u32);
    let cita = gosta(jogador, presidente) != 0 || (presidente != 0 && cqc10_d(presidente + 0x69) != 0 && {
        let a = cqc10_sortear(cqc10_b(cqc10_d(presidente + 0x69) + 0x16) as i32);
        a > cqc10_sortear(0x14)
    });
    if cita {
        cqc10_nomes(gravar, n, 7, presidente);
    } else {
        gravar(n, 7, 0xffff_ffff);
    }
    let f = cqc10_d(jogador + 0x61);
    if f != 0 && cqc10_w(f + 0xd) > 0x1f40 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(cqc10_d(0xb7431c), n);
    } else {
        let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        for &d in dest.iter().take((nd as i8).max(0) as usize) {
            ao_tecnico(cqc10_d(0xb7431c), n, d);
        }
    }
}

// ponytail: a notícia vai aos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "C" fn comparar_noticia_nao_vende(jogador: usize, clube: usize, presidente: usize, outro: usize) {
    orig::<unsafe extern "C" fn(usize, usize, usize, usize)>(1534)(jogador, clube, presidente, outro)
}

/// { 0x75c390 } (thiscall, ret 0xc, media.cpp, quadro SEH sem destrutor no caminho normal):
/// `Media::queue_trophy_praise_story(clube, competição, marca)`, pelos registros do título (0x8693e2, 0x86b000). Clube
/// com técnico, competição e fora das seleções: a notícia 0x2e (0x77bbe0) com o técnico, o clube, a competição, um
/// sorteio(2), o veículo (0x757f30), o comentarista (0x7dab90 com a nação ou −1), a marca, a confiança da diretoria
/// (+6 da linha em [0xb67a44], ou 5000), a liga do clube e os dias do técnico (0x54cdb0); com 180 dias ou mais, a
/// reputação do clube e a entrega ao técnico (0x7861e0).
unsafe extern "thiscall" fn noticia_do_titulo(_this: usize, clube: usize, comp: usize, marca: u32) {
    if clube == 0 || cqc10_d(clube + 0xcf) == 0 || comp == 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x757f30usize);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(0x7dab90usize);
    let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    crate::comparacao::ramo("cqc_10 noticia do titulo");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x2e, 0);
    cqc10_nomes(gravar, n, 0, cqc10_d(clube + 0xcf));
    gravar(n, 4, cqc10_d(clube) as u32);
    gravar(n, 5, cqc10_d(comp) as u32);
    let s = cqc10_sortear(2);
    gravar(n, 6, s as u32);
    let v = veiculo(clube);
    gravar(n, 7, v);
    let na = cqc10_d(clube + 0x53);
    let c = comentarista(cqc10_d(0xb74430), clube, v, if na != 0 { cqc10_i(na) } else { -1 });
    gravar(n, 8, c as u32);
    gravar(n, 9, marca as i8 as i32 as u32);
    let mm = cqc10_d(0xb67a44);
    let conf = if mm != 0 {
        cqc10_w(cqc10_d(mm).wrapping_add((cqc10_i(clube) as usize).wrapping_mul(0x49)) + 6) as i32
    } else {
        0x1388
    };
    gravar(n, 0xa, conf as u32);
    gravar(n, 0xb, (cqc10_d(clube + 0x57) == comp) as u32);
    let tec = cqc10_d(clube + 0xcf);
    let dias = ate(0xae2c90, cqc10_d(tec + 0x3e) as u32, cqc10_d(tec + 0x42) as u32);
    gravar(n, 0xc, dias as u32);
    if dias < 0xb4 {
        return;
    }
    gravar(n, 0xd, cqc10_w(clube + 0x80) as i32 as u32);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    ao_tecnico(cqc10_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_titulo(this: usize, clube: usize, comp: usize, marca: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1535)(this, clube, comp, marca)
}

// ---------- 266a leva: os amistosos entre seleções nas datas internacionais e os convites de amistoso e de torneio (friendly.cpp) ----------

unsafe fn cqc11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc11_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A lista de nações `q` do Friendly (vetor de listas em +0x34f6; a primeira parte indexa pela id da nação, a segunda
/// pela posição na lista ordenada +0x34fa, como na original).
unsafe fn cqc11_lista(this: usize, q: usize) -> usize {
    cqc11_d(cqc11_d(this + 0x34f6).wrapping_add(q.wrapping_mul(4)))
}

/// Os itens (os 4 bytes de cada) da lista, percorrendo os nós sem mexer no cursor (+2 o primeiro; nó: +0 o item, +6 o
/// seguinte).
unsafe fn cqc11_itens(lista: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqc11_d(lista + 2);
    while no != 0 {
        v.push(cqc11_d(cqc11_d(no)) as u32);
        no = cqc11_d(no + 6);
    }
    v
}

/// O ensaio do comparador da 0x5cbdf0: as nações usadas (o vetor local da original), as listas antes e o que a cópia
/// acrescentaria a cada uma (índice da lista, item).
#[derive(Clone)]
struct Cqc11Ensaio {
    usadas: Vec<u8>,
    antes: std::rc::Rc<Vec<Vec<u32>>>,
    novos: Vec<Vec<u32>>,
}

/// A fita do ensaio: os sorteios que a original fez (n, resposta), o próximo e se um n não bateu. Também estão nela os
/// sorteios de dentro das rotinas que a original chama no amistoso (0x53b3d0, 0x5c6f00, 0x5c5f40), que o ensaio pula.
static mut CQC11_FITA: (Vec<(i32, i32)>, usize, bool) = (Vec::new(), 0, false);

/// O sorteio do ensaio: o próximo da fita, com o mesmo n.
unsafe extern "C" fn cqc11_sorteio_da_fita(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(CQC11_FITA);
    match f.0.get(f.1) {
        Some(&(m, r)) if m == n && !f.2 => {
            f.1 += 1;
            r
        }
        _ => {
            f.2 = true;
            0
        }
    }
}

unsafe fn cqc11_fita_fechou() -> bool {
    let f = &*core::ptr::addr_of!(CQC11_FITA);
    !f.2 && f.1 == f.0.len()
}

impl Cqc11Ensaio {
    fn usada(&self, id: usize) -> bool {
        self.usadas.get(id).is_some_and(|&b| b != 0)
    }
    fn usar(&mut self, id: usize) {
        if id >= self.usadas.len() {
            self.usadas.resize(id + 1, 0);
        }
        self.usadas[id] = 1;
    }
    fn por(&mut self, q: usize, item: u32) {
        if q >= self.novos.len() {
            self.novos.resize(q + 1, Vec::new());
        }
        self.novos[q].push(item);
    }
    fn tem(&self, q: usize, item: u32) -> bool {
        self.antes.get(q).is_some_and(|v| v.contains(&item)) || self.novos.get(q).is_some_and(|v| v.contains(&item))
    }
}

/// Um jogo da primeira parte: com nação nos dois clubes (+0x1c, +0x20; +0x53, a id em +0), as duas ficam usadas e cada
/// uma vai à lista (pela id) da outra: a de casa recebe a de fora, depois o contrário (0x68d0d0 com 4: thiscall na
/// lista). No ensaio, só o registro.
unsafe fn cqc11_jogo(this: usize, p: usize, usadas: usize, ens: Option<&mut Cqc11Ensaio>) {
    let na = cqc11_d(cqc11_d(p + 0x1c) + 0x53);
    if na == 0 {
        return;
    }
    let a = cqc11_d(na) as u32;
    let nb = cqc11_d(cqc11_d(p + 0x20) + 0x53);
    if nb == 0 {
        return;
    }
    let b = cqc11_d(nb) as u32;
    match ens {
        Some(e) => {
            e.usar(a as usize);
            e.usar(b as usize);
            e.por(a as usize, b);
            e.por(b as usize, a);
        }
        None => {
            let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
            *((usadas.wrapping_add(a as usize)) as *mut u8) = 1;
            crate::comparacao::ramo("cqc_11 nacoes com jogo");
            *((usadas.wrapping_add(b as usize)) as *mut u8) = 1; // cqc11: as nações com jogo perto da data
            let (va, vb) = (a, b);
            por(cqc11_lista(this, a as usize), &vb as *const u32 as usize, 4);
            por(cqc11_lista(this, b as usize), &va as *const u32 as usize, 4);
        }
    }
}

/// A primeira parte da 0x5cbdf0: de 4 dias antes a 4 dias depois da data (+3 do registro de datas; 0x54c8e0, 0x54c5b0
/// e 0x417c10), nas 3 fatias e nos 2 grupos de cada dia (0x5ae6a0), cada jogo com nação nos dois clubes (+0x1c, +0x20;
/// +0x53, a id em +0) marca as duas nações como usadas e cada uma vai à lista da outra (pela id: a de casa recebe a de
/// fora, depois o contrário; 0x68d0d0 com 4). No ensaio, nada se grava fora das datas estáticas (o cursor das listas
/// do calendário não se mexe).
unsafe fn cqc11_primeira_parte(this: usize, datas: usize, usadas: usize, mut ens: Option<&mut Cqc11Ensaio>) {
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let (mut x, mut y, mut t, mut u) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pt, pu) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    let data = datas + 3;
    copiar(px, pu, menos(data, pt, 4)); // a primeira data: 4 dias antes
    copiar(py, pu, mais(data, pt, 4)); // a última: 4 dias depois
    while cqc11_uw(py + 2) > cqc11_uw(px + 2) || (cqc11_uw(py + 2) == cqc11_uw(px + 2) && cqc11_w(py) >= cqc11_w(px)) {
        for fatia in 0u32..3 {
            for grupo in 0u32..2 {
                // ponytail: a original empurra os dois bytes com sobra de pilha em cima; a 0x5abf10 só lê o byte
                let l = lista_de(px, fatia, grupo);
                if let Some(e) = ens.as_deref_mut() {
                    let mut no = cqc11_d(l + 2);
                    while no != 0 {
                        cqc11_jogo(this, cqc11_d(no), usadas, Some(&mut *e));
                        no = cqc11_d(no + 6);
                    }
                } else {
                    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc11_d(l + 2));
                    let mut p = proximo(l);
                    while p != 0 {
                        cqc11_jogo(this, p, usadas, None);
                        p = proximo(l);
                    }
                }
            }
        }
        copiar(px, pu, mais(px, pt, 1)); // o dia seguinte
    }
}

/// A nação `n` (ponteiro) está numa das confederações da data: alguma das `quantas` entradas de 20 bytes a partir de
/// `datas` tem em +0xc −1 ou a id do continente da nação (+0x71, lido sem conferir o nulo, como na original). Sem
/// entradas: não.
unsafe fn cqc11_na_data(datas: usize, quantas: i32, n: usize) -> bool {
    let mut k = 0;
    while k < quantas {
        let c = cqc11_d(datas + 0xc + k as usize * 0x14) as i32;
        if c == -1 || cqc11_d(cqc11_d(n + 0x71)) as i32 == c {
            return true;
        }
        k += 1;
    }
    false
}

/// Na data só da África (+2 = 1), a nação africana (+0x71 = [0x9cfa08]) fica de fora.
unsafe fn cqc11_africana_fora(datas: usize, n: usize) -> bool {
    *((datas + 2) as *const u8) == 1 && cqc11_d(n + 0x71) != 0 && cqc11_d(cqc11_d(n + 0x71)) == cqc11_d(0x9cfa08)
}

/// A segunda parte da 0x5cbdf0: cada nação `a` da lista ordenada (+0x34fa, conta +0x34fe relida a cada volta) ainda
/// livre, numa das confederações da data e não africana na data da África procura, entre as seguintes `b` livres, a
/// primeira que serve: sorteio de 500 (0x90cfc0, antes dos filtros de `b`), `b` na data e não africana, uma das duas
/// com +0x88 ≥ 5, o par não proibido (0x5cc3e0), mais um sorteio de 1000 se o continente (+0x71) é o mesmo, a soma
/// acima de 400 e `b` ainda não na lista da posição de `a` (+0x34f6, pela posição; o cursor volta ao começo). Achada:
/// o mando sorteado (0x90cfc0 de 2), as seleções principais (0x53b3d0, `b` antes) e o amistoso (0x5c6f00 com a data
/// +3); criado, a resposta forçada (0x5c5f40 com 1), as duas marcadas e cada uma na lista da posição da outra (`b` na
/// de `a`, depois `a` na de `b`; os 4 bytes do registro, a id). Criado ou não, a próxima `a`. No ensaio, o amistoso
/// não é criado: vale o que a original fez (a id de `b` apareceu na lista da posição de `a`), os sorteios vêm da fita
/// e, depois do mando, o resto continua (desde `a` seguinte) no primeiro sorteio de 500 da fita (ou no fim) com que
/// fecha: os de dentro das rotinas do amistoso ficam para trás. Começa na nação `i0`.
unsafe fn cqc11_segunda_parte(this: usize, datas: usize, n: u32, usadas: usize, mut ens: Option<&mut Cqc11Ensaio>, i0: i32) {
    let sortear: unsafe extern "C" fn(i32) -> i32 =
        if ens.is_some() { cqc11_sorteio_da_fita } else { core::mem::transmute(0x90cfc0usize) };
    let proibido: unsafe extern "thiscall" fn(usize, i32, i32) -> u32 = core::mem::transmute(0x5cc3e0usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize = core::mem::transmute(0x5c6f00usize);
    let responder: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c5f40usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    // os sorteios: push n; add esp, 4 -> cdecl; 0x53b3d0: push; add esp, 4 -> cdecl; 0x5cc3e0, 0x5c6f00, 0x5c5f40 e
    // 0x68d0d0: mov ecx, esi (ou a lista) -> thiscall
    let quantas = n as i16 as i32;
    let total = |this: usize| cqc11_d(this + 0x34fe) as i32;
    let nacao = |this: usize, k: i32| cqc11_d(cqc11_d(this + 0x34fa).wrapping_add(k as usize * 4));
    let mut i: i32 = i0;
    while i < total(this) {
        let a = nacao(this, i);
        let ida = cqc11_d(a);
        let livre_a = match ens.as_deref() {
            Some(e) => !e.usada(ida),
            None => *((usadas.wrapping_add(ida)) as *const u8) == 0,
        };
        if livre_a && quantas > 0 && cqc11_na_data(datas, quantas, a) && !cqc11_africana_fora(datas, a) {
            let mut j = i + 1;
            while j < total(this) {
                if ens.is_some() && (*core::ptr::addr_of!(CQC11_FITA)).2 {
                    return; // a fita já não bate
                }
                let b = nacao(this, j);
                let idb = cqc11_d(b);
                let livre_b = match ens.as_deref() {
                    Some(e) => !e.usada(idb),
                    None => *((usadas.wrapping_add(idb)) as *const u8) == 0,
                };
                if livre_b {
                    let mut v = sortear(500);
                    if cqc11_na_data(datas, quantas, b) && !cqc11_africana_fora(datas, b)
                        && (cqc11_d(a + 0x88) as i32 >= 5 || cqc11_d(b + 0x88) as i32 >= 5)
                        && proibido(this, ida as i32, idb as i32) == 0
                    {
                        if cqc11_d(a + 0x71) == cqc11_d(b + 0x71) {
                            v = v.wrapping_add(sortear(1000)); // cqc11: o mesmo continente
                        }
                        if v > 400 { // cqc11: a soma acima de 400
                            let ja = match ens.as_deref() {
                                Some(e) => e.tem(i as usize, idb as u32),
                                None => {
                                    let l = cqc11_lista(this, i as usize);
                                    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc11_d(l + 2));
                                    let mut achou = false;
                                    let mut p = proximo(l);
                                    while p != 0 {
                                        if cqc11_d(p) == idb {
                                            achou = true;
                                            break;
                                        }
                                        p = proximo(cqc11_lista(this, i as usize));
                                    }
                                    achou
                                }
                            };
                            if !ja {
                                crate::comparacao::ramo("cqc_11 amistoso entre selecoes");
                                let casa = sortear(2) as u32; // cqc11: o mando sorteado
                                match ens.as_deref_mut() {
                                    Some(e) => {
                                        if cqc11_itens(cqc11_lista(this, i as usize)).contains(&(idb as u32)) {
                                            e.usar(idb);
                                            e.usar(ida);
                                            e.por(i as usize, idb as u32);
                                            e.por(j as usize, ida as u32);
                                        }
                                        let f = core::ptr::addr_of_mut!(CQC11_FITA);
                                        if (*f).2 {
                                            return;
                                        }
                                        let base = e.clone();
                                        let pontos: Vec<usize> = {
                                            let v = &(*f).0;
                                            ((*f).1..=v.len()).filter(|&p| p == v.len() || v[p].0 == 500).collect()
                                        };
                                        for p in pontos {
                                            (*f).1 = p;
                                            *e = base.clone();
                                            cqc11_segunda_parte(this, datas, n, 0, Some(&mut *e), i + 1);
                                            if cqc11_fita_fechou() {
                                                return;
                                            }
                                            (*f).2 = false;
                                        }
                                        (*f).2 = true; // nenhum ponto fecha
                                        return;
                                    }
                                    None => {
                                        let sb = selecao(b);
                                        let sa = selecao(a);
                                        let m = acrescentar(this, sa, datas + 3, sb, casa);
                                        if m != 0 {
                                            responder(this, m, 1);
                                            *((usadas.wrapping_add(idb)) as *mut u8) = 1;
                                            *((usadas.wrapping_add(ida)) as *mut u8) = 1; // cqc11: as duas usadas
                                            por(cqc11_lista(this, i as usize), b, 4);
                                            por(cqc11_lista(this, j as usize), a, 4);
                                        }
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
}

/// { 0x5cbdf0 } (thiscall, ret 8, friendly.cpp, quadro SEH só das datas locais, sem destrutor): `Friendly::
/// arrange_international_friendlies(datas, quantas)`, pelo dia dos amistosos internacionais (0x5cbb00, as entradas de
/// hoje + 12 semanas). O vetor das nações usadas ([0xae2350] bytes zerados, 0x945501; sem memória: a caixa, linha
/// 0x1738, e nada), a primeira parte (as nações com jogo de 4 dias antes a 4 depois) e a segunda (os pares), e o vetor
/// sai (0x9452ca).
unsafe extern "thiscall" fn amistosos_entre_selecoes(this: usize, datas: usize, n: u32) {
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free: push; add esp, 4 -> cdecl
    let usadas = zerado(cqc11_d(0xae2350) as u32, 1);
    if usadas == 0 {
        aviso_em(0x9cd0a0, 0x1738);
        return;
    }
    cqc11_primeira_parte(this, datas, usadas, None);
    cqc11_segunda_parte(this, datas, n, usadas, None, 0);
    liberar(usadas);
}

/// Comparador: a original cria amistosos (0x5c6f00, 0x5c5f40), acrescenta às listas (0x68d0d0) e sorteia: só ela roda,
/// com o plano em ensaio. A primeira parte do ensaio roda antes dela (depois, o calendário já teria os amistosos
/// novos); a segunda, depois dela, pela fita dos sorteios que ela fez (aleatorio::rastrear: n e resposta, os de dentro
/// das rotinas do amistoso também), decide cada amistoso pelo que a original fez. Confere a sequência dos sorteios (os
/// do ensaio são os da original, salvo os de dentro das rotinas do amistoso, e a fita acaba junto) e, lista a lista de
/// nações, que os itens novos da original são os que o ensaio acrescentaria. (Lado a lado pelo estado do sorteador não
/// servia: a 0x5c6f00 e a 0x5c5f40 sorteiam por dentro na original e o ensaio não as chama.)
unsafe extern "thiscall" fn comparar_amistosos_entre_selecoes(this: usize, datas: usize, n: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1536);
    let nacoes = cqc11_d(0xae2350) as i32;
    let k = nacoes.max(cqc11_d(this + 0x34fe) as i32).max(0) as usize;
    let antes: Vec<Vec<u32>> = (0..k).map(|q| cqc11_itens(cqc11_lista(this, q))).collect();
    let mut e = Cqc11Ensaio { usadas: vec![0u8; nacoes.max(0) as usize], antes: std::rc::Rc::new(antes), novos: vec![Vec::new(); k] };
    cqc11_primeira_parte(this, datas, 0, Some(&mut e));
    let ((), fita) = crate::aleatorio::rastrear(|| o(this, datas, n));
    let quantos = fita.len();
    *core::ptr::addr_of_mut!(CQC11_FITA) = (fita, 0, false);
    cqc11_segunda_parte(this, datas, n, 0, Some(&mut e), 0);
    let igual = cqc11_fita_fechou();
    let (pos, falhou) = { let f = &*core::ptr::addr_of!(CQC11_FITA); (f.1, f.2) };
    *core::ptr::addr_of_mut!(CQC11_FITA) = (Vec::new(), 0, false);
    let mut dif = Vec::new();
    for q in 0..k.max(e.novos.len()) {
        let mut depois = if q < k { cqc11_itens(cqc11_lista(this, q)) } else { Vec::new() };
        let mut falta = false;
        for x in e.antes.get(q).map(|v| v.as_slice()).unwrap_or(&[]) {
            match depois.iter().position(|y| y == x) {
                Some(p) => {
                    depois.remove(p);
                }
                None => falta = true,
            }
        }
        let mut esperado = e.novos.get(q).cloned().unwrap_or_default();
        depois.sort_unstable();
        esperado.sort_unstable();
        if falta || depois != esperado {
            dif.push((q, depois, esperado));
        }
    }
    PLACAR.registrar(igual && dif.is_empty(), || {
        format!("amistosos_entre_selecoes(datas {:x}, {}) fita de {} sorteios, o ensaio parou em {} (n diferente: {}); listas (indice, original, ensaio) {:x?}",
            datas, n as i16, quantos, pos, falhou, dif.iter().take(4).collect::<Vec<_>>())
    });
}

/// { 0x5ca090 } (thiscall, ret 0x20, friendly.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `friendly_check_match_invitation(a, b, clube, dia, ano, data, texto, tamanho)`, pela notícia do convite (0x77bc80,
/// 0x782b70). Com texto e tamanho (palavra) abaixo de 100: a caixa da linha 0x1527 e 1 (a original). Sem a data: com
/// texto, os jogos do clube `a` (+0 id) no dia (dia, ano; 0x5acbe0 numa lista local) dizem se o amistoso foi marcado
/// (o primeiro jogo é de `a` contra `b`) ou cancelado (0x66f4e0 no texto); 0. Com a data: antes de hoje + 2 dias
/// (0x54c5b0), cancelado (com texto) e 0; o clube livre na data (0x5c6b70 com tipo 0): 1; senão, com texto, "{} ... are
/// unavailable" (o byte +0x52 duas vezes e o nome +0x38) e 0.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn convite_de_amistoso_valido(this: usize, a: usize, b: usize, clube: usize, dia: u32, ano: u32,
    data: usize, texto: usize, tamanho: u32) -> u32 {
    if texto != 0 && (tamanho as i16) < 100 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(1537)(
            this, a, b, clube, dia, ano, data, texto, tamanho); // a caixa
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    // 0x66f4e0: add esp, 8 / 0x14 depois -> cdecl (variádica)
    if data == 0 {
        if texto != 0 {
            let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
                core::mem::transmute(0x5acbe0usize);
            // 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
            let mut l = [0u8; 0x10];
            let lista = l.as_mut_ptr() as usize;
            nova(lista);
            jogos(cqc11_d(0xae2a58), dia, ano, dia, ano, cqc11_d(a) as u32, lista, u32::MAX, 0xffff_fffe, u32::MAX);
            core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqc11_d(lista + 2));
            let p = proximo(lista);
            let msg = if p != 0 && cqc11_d(p + 0x1c) == a && cqc11_d(p + 0x20) == b {
                0x9cd168 // "This match has been arranged."
            } else {
                0x9cd148 // "This match has been cancelled."
            };
            formatar(texto, msg);
            soltar(lista);
        }
        return 0;
    }
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mut t = [0u8; 8];
    let r = mais(0xae2c90, t.as_mut_ptr() as usize, 2);
    let (tano, tdia) = (cqc11_uw(r + 2), cqc11_w(r));
    if tano > cqc11_uw(data + 2) || (tano == cqc11_uw(data + 2) && tdia > cqc11_w(data)) {
        if texto != 0 {
            formatar(texto, 0x9cd148usize);
        }
        return 0;
    }
    let livre: unsafe extern "stdcall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c6b70usize);
    // 0x5c6b70: 3 pushes sem add esp -> stdcall (o mov ecx da original sobra)
    if livre(data, clube, 0) != 0 {
        return 1;
    }
    if texto != 0 {
        let k = *((clube + 0x52) as *const i8) as i32;
        formatar(texto, 0x9cd104usize, k, k, clube + 0x38); // "{}<%s - Club Name...>{} are unavailable for this match."
    }
    0
}

/// O texto da notícia (até o tamanho dado, no máximo 0x200 bytes), quando há.
fn cqc11_regioes_do_texto(texto: usize, tamanho: u32) -> Vec<(usize, usize)> {
    if texto == 0 {
        Vec::new()
    } else {
        vec![(texto, ((tamanho as i16).max(0) as usize).min(0x200))]
    }
}

/// Comparador: lado a lado pelo texto e a devolução (as listas locais nascem e morrem nas duas); com a caixa, só a
/// original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_convite_de_amistoso_valido(this: usize, a: usize, b: usize, clube: usize, dia: u32,
    ano: u32, data: usize, texto: usize, tamanho: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(1537);
    if texto != 0 && (tamanho as i16) < 100 {
        return o(this, a, b, clube, dia, ano, data, texto, tamanho);
    }
    let reg = cqc11_regioes_do_texto(texto, tamanho);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, a, b, clube, dia, ano, data, texto, tamanho),
        || convite_de_amistoso_valido(this, a, b, clube, dia, ano, data, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("convite_de_amistoso_valido(a {:x}, b {:x}, clube {:x}, data {:x}) original={} rust={} {:x?} x {:x?}",
        a, b, clube, data, r, n, mo, mr));
    r
}

/// { 0x5ca290 } (thiscall, ret 0x10, friendly.cpp): `friendly_check_tournament_invitation(clube, torneio, texto,
/// tamanho)`, pela notícia do convite para torneio. Com texto e tamanho abaixo de 100: a caixa da linha 0x1575 e 1 (a
/// original). Sem torneio ou com a data dele (+4) antes de hoje + 2 dias: "This competition has been cancelled" (com
/// texto) e 0. O clube livre na data (0x5c6b70 com o tipo +0xc): 1; senão, com texto, "{} ... are unavailable for this
/// competition." e 0.
unsafe extern "thiscall" fn convite_de_torneio_valido(this: usize, clube: usize, torneio: usize, texto: usize, tamanho: u32) -> u32 {
    if texto != 0 && (tamanho as i16) < 100 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(1538)(this, clube, torneio, texto, tamanho);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut cancelado = torneio == 0;
    if !cancelado {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u8; 8];
        let r = mais(0xae2c90, t.as_mut_ptr() as usize, 2);
        let (tano, tdia) = (cqc11_uw(r + 2), cqc11_w(r));
        let d = torneio + 4;
        cancelado = tano > cqc11_uw(d + 2) || (tano == cqc11_uw(d + 2) && tdia > cqc11_w(d));
    }
    if cancelado {
        if texto != 0 {
            formatar(texto, 0x9cd1d4usize); // "This competition has been cancelled"
        }
        return 0;
    }
    let livre: unsafe extern "stdcall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c6b70usize);
    // ponytail: a original empurra o tipo (byte +0xc) com o resto do ecx em cima; a 0x5c6b70 só lê o byte
    if livre(torneio + 4, clube, *((torneio + 0xc) as *const u8) as u32) != 0 {
        return 1;
    }
    if texto != 0 {
        let k = *((clube + 0x52) as *const i8) as i32;
        formatar(texto, 0x9cd188usize, k, k, clube + 0x38); // "... are unavailable for this competition."
    }
    0
}

/// Comparador: lado a lado pelo texto e a devolução; com a caixa, só a original.
unsafe extern "thiscall" fn comparar_convite_de_torneio_valido(this: usize, clube: usize, torneio: usize, texto: usize, tamanho: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(1538);
    if texto != 0 && (tamanho as i16) < 100 {
        return o(this, clube, torneio, texto, tamanho);
    }
    let reg = cqc11_regioes_do_texto(texto, tamanho);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, torneio, texto, tamanho),
        || convite_de_torneio_valido(this, clube, torneio, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("convite_de_torneio_valido(clube {:x}, torneio {:x}) original={} rust={} {:x?} x {:x?}",
        clube, torneio, r, n, mo, mr));
    r
}

// ---------- 266a leva: o dia das seleções e as buscas da sessão do técnico humano (national_teams.cpp, scout_manager.cpp) ----------

unsafe fn cqc12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc12_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A data `r` é a de hoje (o ano e o dia)?
unsafe fn cqc12_e_hoje(r: usize, hoje: usize) -> bool {
    cqc12_uw(r + 2) == cqc12_uw(hoje + 2) && cqc12_w(r) == cqc12_w(hoje)
}

/// Marca os clubes com jogo na data `d` e nos dias vizinhos (0x7775c0 na data, na véspera e no dia seguinte; a data
/// volta a ser a mesma no fim: 0x54c8e0/0x54c5b0 e 0x417c10).
unsafe fn cqc12_marcar_tres_dias(this: usize, d: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7775c0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    // 0x7775c0: push &data; mov ecx, edi -> thiscall, ret 4
    let (mut t, mut u) = ([0u8; 8], [0u8; 8]);
    let (pt, pu) = (t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    crate::comparacao::ramo("cqc_12 marca os clubes");
    marcar(this, d);
    copiar(d, pu, menos(d, pt, 1));
    marcar(this, d);
    copiar(d, pu, mais(d, pt, 2));
    marcar(this, d);
    copiar(d, pu, menos(d, pt, 1));
}

/// Os jogos da lista `l` (do dia daqui a duas semanas): para cada um, as convocações (0x76bab0) e, fora das fases
/// finais (a competição +0x14 numa das chaves 0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0,
/// 0x9cf78c, 0x9cf964), com mandante que não é a seleção B (0x5409b0) e a marca +0x40: os convocados por clube
/// (0x7771c0) das duas seleções; com o bit 0x200 de +0x4d, também os das seleções B (0x53b4f0 da nação +0x53 de cada
/// uma) e, sem ele e com o 0x100, os das duas com `b` = 1.
unsafe fn cqc12_jogos(this: usize, l: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let convocar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76bab0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7771c0usize);
    // 0x76bab0: push p; mov ecx, edi -> thiscall ret 4; 0x5409b0 e 0x53b4f0: push; add esp, 4 -> cdecl;
    // 0x7771c0: push p; push b; push time; mov ecx, edi -> thiscall ret 0xc
    cqc12_pd(l + 0xa, cqc12_d(l + 2));
    let mut p = proximo(l);
    while p != 0 {
        crate::comparacao::ramo("cqc_12 convoca");
        convocar(this, p); // cqc12: as convocações para o jogo
        let comp = cqc12_d(p + 0x14);
        let final_ = comp != 0
            && [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964]
                .iter().any(|&k| cqc12_d(comp) == cqc12_d(k));
        if !final_ && segunda(cqc12_d(p + 0x1c)) == 0 && *((p + 0x40) as *const u8) != 0 {
            registrar(this, cqc12_d(p + 0x1c), 0, p);
            registrar(this, cqc12_d(p + 0x20), 0, p);
            let f = cqc12_uw(p + 0x4d);
            if f & 0x200 != 0 {
                let b1 = selecao_b(cqc12_d(cqc12_d(p + 0x1c) + 0x53));
                registrar(this, b1, 0, p);
                let b2 = selecao_b(cqc12_d(cqc12_d(p + 0x20) + 0x53));
                registrar(this, b2, 0, p);
            } else if f & 0x100 != 0 {
                registrar(this, cqc12_d(p + 0x1c), 1, p);
                registrar(this, cqc12_d(p + 0x20), 1, p);
            }
        }
        p = proximo(l);
    }
}

/// { 0x76b100 } (thiscall, ret 4, national_teams.cpp, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::daily_update(hoje)`, pelo dia do jogo ([0xb74318], 0x5ccb07). A data daqui a duas semanas
/// (0x54c770); a tabela dos clubes com jogo (+0x24) sai (0x9452ca) e zera. Em 1º de janeiro, 15 de abril e 1º de agosto
/// (0x549f70 no ano de hoje), as reservas das seleções se refazem (0x76abf0). Nas 3 fatias do dia daqui a duas
/// semanas, as listas dos grupos 0 e 1 (0x5abf10): com jogo em alguma e sem a tabela, os clubes com jogo em volta da
/// data são marcados (cqc12_marcar_tres_dias); os jogos das duas listas (cqc12_jogos). Com a tabela, os jogos dos clubes
/// com convocados se remarcam (0x7777e0). Da lista em +0 saem (0x68d1f0) os itens do dia de hoje (+0x10). Com o ranking
/// (+0x1c), a atualização dele (0x58d460); no dia múltiplo de 70, a poda dos registros de convocação (0x7774b0); por fim
/// as notícias dos classificados (0x7779e0).
unsafe extern "thiscall" fn dia_das_selecoes(this: usize, hoje: usize) {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let refazer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x76abf0usize);
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let remarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7777e0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let ranking: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x58d460usize);
    let podar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7774b0usize);
    let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7779e0usize);
    // 0x54c770: push 2; push saida; mov ecx, hoje -> thiscall ret 8; free: add esp, 4 -> cdecl; 0x549f70: 4 pushes;
    // lea ecx, saida -> thiscall ret 0x10; 0x76abf0, 0x7774b0, 0x7779e0: mov ecx, edi sem pushes -> thiscall;
    // 0x5abf10: 4 pushes; mov ecx, [0xae2a58] -> thiscall ret 0x10; 0x7777e0: push &data; mov ecx, edi -> thiscall
    // ret 4; 0x68d1f0: push item; mov ecx, [edi] -> thiscall ret 4; 0x58d460: push hoje; mov ecx, [edi + 0x1c] ->
    // thiscall ret 4
    let mut dd = [0u8; 8];
    let d = dd.as_mut_ptr() as usize;
    semanas(hoje, d, 2);
    if cqc12_d(this + 0x24) != 0 {
        liberar(cqc12_d(this + 0x24));
        cqc12_pd(this + 0x24, 0);
    }
    // ponytail: a original empurra o ano (palavra) e a semana com sobra de registrador em cima; a 0x549f70 lê as palavras
    let ano = cqc12_uw(hoje + 2) as u32;
    let mut t = [0u8; 8];
    let pt = t.as_mut_ptr() as usize;
    if cqc12_e_hoje(dma(pt, 1, 0, ano, u32::MAX), hoje) // cqc12: 1º de janeiro
        || cqc12_e_hoje(dma(pt, 0xf, 3, ano, u32::MAX), hoje)
        || cqc12_e_hoje(dma(pt, 1, 7, ano, u32::MAX), hoje)
    {
        crate::comparacao::ramo("cqc_12 refaz as reservas");
        refazer(this);
    }
    let fixman = cqc12_d(0xae2a58);
    for fatia in 0u32..3 {
        // os dwords da original: o ano (com os 2 bytes seguintes da data) e o dia (com o ano em cima); ponytail: a
        // fatia vai sem a sobra de pilha dos bytes de cima (a 0x5abf10 faz and 0xff)
        let l0 = lista(fixman, cqc12_d(d + 2) as u32, cqc12_d(d) as u32, fatia, 0);
        let l1 = lista(fixman, cqc12_d(d + 2) as u32, cqc12_d(d) as u32, fatia, 1);
        if ((l0 != 0 && cqc12_w(l0) != 0) || (l1 != 0 && cqc12_w(l1) != 0)) && cqc12_d(this + 0x24) == 0 {
            cqc12_marcar_tres_dias(this, d); // cqc12: os clubes com jogo em volta da data
        }
        if l0 != 0 {
            cqc12_jogos(this, l0);
        }
        if l1 != 0 {
            cqc12_jogos(this, l1);
        }
    }
    if cqc12_d(this + 0x24) != 0 {
        remarcar(this, d);
    }
    if cqc12_d(this) != 0 {
        let dia = cqc12_w(hoje);
        let l = cqc12_d(this);
        cqc12_pd(l + 0xa, cqc12_d(l + 2));
        let mut p = proximo(cqc12_d(this));
        while p != 0 {
            if cqc12_w(p + 0x10) == dia {
                tirar(cqc12_d(this), p);
            }
            p = proximo(cqc12_d(this));
        }
    }
    if cqc12_d(this + 0x1c) != 0 {
        ranking(cqc12_d(this + 0x1c), hoje);
    }
    if cqc12_w(hoje) as i32 % 0x46 == 0 {
        podar(this);
    }
    classificados(this);
}

/// Quantos itens da lista em +0 são do dia `dia` (+0x10), percorrendo os nós sem mexer no cursor.
unsafe fn cqc12_do_dia(this: usize, dia: i16) -> u32 {
    let l = cqc12_d(this);
    if l == 0 {
        return 0;
    }
    let mut n = 0;
    let mut no = cqc12_d(l + 2);
    while no != 0 {
        if cqc12_w(cqc12_d(no) + 0x10) == dia {
            n += 1;
        }
        no = cqc12_d(no + 6);
    }
    n
}

/// Comparador: a original convoca (0x76bab0: notícias, elencos), remarca jogos (0x7777e0), aloca a tabela (0x7775c0)
/// e libera itens (0x68d1f0): não pode rodar duas vezes. Só ela roda, conferindo o plano do que a própria rotina grava:
/// nenhum item do dia de hoje sobra na lista em +0.
unsafe extern "thiscall" fn comparar_dia_das_selecoes(this: usize, hoje: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1539);
    let dia = cqc12_w(hoje);
    let antes = cqc12_do_dia(this, dia);
    o(this, hoje);
    let sobra = cqc12_do_dia(this, dia);
    PLACAR.registrar(sobra == 0, || format!("dia_das_selecoes(dia {}) itens do dia antes {} depois {}", dia, antes, sobra));
}

/// O índice (0..15) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um byte da original
/// ((id − [0xae2364]) + 16 em 8 bits); Err(()) sem técnico, Ok(None) fora (a caixa).
unsafe fn cqc12_indice() -> Result<Option<usize>, ()> {
    let t = cqc12_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// O registro de busca `k` (0x2b0 bytes) da sessão `s`.
unsafe fn cqc12_busca(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// { 0x7f8420 } (thiscall, ret 0, scout_manager.cpp): `SCOUT_MANAGER::current_search_has_no_club()`, pelas telas das
/// buscas (0x482a9b). A sessão do técnico humano da vez ([this + 0xc + 4 × índice]): a busca atual (o byte com sinal
/// em +0 da sessão) sem clube (+0x191 = −2). Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x454; a
/// original) e 0.
unsafe extern "thiscall" fn busca_atual_sem_clube(this: usize) -> u32 {
    let i = match cqc12_indice() {
        Err(()) => return 0,
        Ok(None) => return orig::<unsafe extern "thiscall" fn(usize) -> u32>(1540)(this), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    (cqc12_d(cqc12_busca(s, *(s as *const i8) as i32) + 0x191) as i32 == -2) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_busca_atual_sem_clube(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1540);
    if matches!(cqc12_indice(), Ok(None)) {
        return o(this);
    }
    let r = o(this);
    let n = busca_atual_sem_clube(this);
    PLACAR.registrar(r == n, || format!("busca_atual_sem_clube() original={} rust={}", r, n));
    r
}

/// { 0x7f8500 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::is_club_in_session_searches(clube)`, pelas telas
/// (0x482aaf, 0x483040, 0x7b34e1). Alguma das 10 buscas da sessão do técnico humano da vez tem o clube (+0x191 = +0
/// do clube): 1. Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x480; a original) e 0.
unsafe extern "thiscall" fn clube_nas_buscas_da_sessao(this: usize, clube: usize) -> u32 {
    let i = match cqc12_indice() {
        Err(()) => return 0,
        Ok(None) => return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1541)(this, clube), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    let id = cqc12_d(clube);
    // ponytail: a original testa o contador contra 0xff (nunca) para usar a busca atual (+0); fica o contador
    (0..10).any(|k| cqc12_d(cqc12_busca(s, k) + 0x191) == id) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_clube_nas_buscas_da_sessao(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1541);
    if matches!(cqc12_indice(), Ok(None)) {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = clube_nas_buscas_da_sessao(this, clube);
    PLACAR.registrar(r == n, || format!("clube_nas_buscas_da_sessao(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- a escolha do elenco da seleção (0x76c120), com a correção dos elencos de 23 e 25 ----------

unsafe fn cqc12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc12_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// Os códigos de posição das vagas da original: elenco de 26 (0x76c771..0x76c84c), de 24 (0x76c870..0x76c92c) e o de
/// qualquer outro tamanho (0x76c939..0x76c9de, 22 vagas), com os comuns gravados depois (0x76c860/0x76c865 e as vagas
/// 0, 1, 3, 5, 6 e 21). O código 0xc da vaga 12/11/10 vem do ebp (0xc desde 0x76c2e4/0x76c6a9).
const CQC12_26: [u16; 26] = [1, 1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240, 0x240];
const CQC12_24: [u16; 24] = [1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
const CQC12_22: [u16; 22] = [1, 1, 1, 0x202, 0x204, 0x204, 0x204, 0x204, 0x84, 0x804, 0xc, 0x210, 0x210, 0x220, 0x90, 0x810,
    0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
/// As vagas que a tabela de 24 tem a mais que a de 22, na ordem em que aparecem nela: 0x202 (líbero/centro, a vaga
/// 3), 0x88 e 0x808 (volantes pelos lados, 9 e 10) e 0x210 (o 3º meia central, 14).
const CQC12_EXTRAS: [u16; 3] = [0x202, 0x88, 0x808];

/// Os códigos de posição das `tam` vagas (o byte +0x42 do registro do tipo). Fiel à original em 26 e 24 e nas 22
/// primeiras vagas dos outros tamanhos (a tabela de 22). Correção do original aprovada (01/11/2026): a original só
/// preenche as vagas de 22, 24 ou 26 jogadores; nos elencos de 23 (as fases finais, 0x76d430: a Copa de 2002, em
/// 29/05/2002) e 25, as vagas a partir da 22ª ficam com a sobra da pilha (local_86c), que vai à escala (0x6e2000) e
/// ao filtro dos escalados (0x5b7d20), e a sobra muda a cada execução (metades de ponteiros da heap). Aqui as vagas
/// além das 22 da tabela da original recebem, em ordem, as que a tabela de 24 acrescenta à de 22 (CQC12_EXTRAS):
/// o elenco de 23 = o de 22 + 0x202 (a primeira vaga que o de 24 acrescenta); o de 25 = o de 22 + 0x202, 0x88 e
/// 0x808. Por que assim: as 22 primeiras vagas continuam as que a original grava nesses tamanhos (a comparação
/// confere todas), e cada vaga a mais vai na direção da tabela maior seguinte. O elenco de 25 parte da tabela de 22
/// e não da de 24 porque é a que a original usa para ele (só 24 e 26 têm tabela própria). Outros tamanhos: como a
/// original nas 22 primeiras; as demais com código 0 (ninguém passa no filtro). ponytail: o jogo só pede 23, 24 e 26.
unsafe fn cqc12_codigos(tam: u8) -> [u16; 50] {
    let mut c = [0u16; 50];
    match tam {
        0x1a => c[..26].copy_from_slice(&CQC12_26),
        0x18 => c[..24].copy_from_slice(&CQC12_24),
        _ => c[..22].copy_from_slice(&CQC12_22),
    }
    if tam == 23 || tam == 25 {
        crate::comparacao::ramo(if tam == 23 { "cqc_12 correção do elenco de 23" } else { "cqc_12 correção do elenco de 25" });
        for i in 22..tam as usize {
            c[i] = CQC12_EXTRAS[i - 22]; // cqc12: as vagas da correção
        }
    }
    c
}

/// O registro da nação nas seleções (+4 da tabela, 0x6e bytes pelo id da nação).
unsafe fn cqc12_registro(this: usize, nacao: usize) -> usize {
    cqc12_d(this + 4).wrapping_add(cqc12_d(nacao).wrapping_mul(0x6e))
}

/// As faixas da reputação aceita (+9 da ficha +0x61): (mínimo exclusive, máximo), −1 e 0x2711 fora da Austrália
/// ([0x9cf224]); para ela, pela competição da partida (amistoso 0x5c6710 de [0xae2c74], ou a da Oceania: 0x8ca/0x2134;
/// Copa das Confederações 0xabe/0x222e; eliminatórias da Oceania 0x8ca e 9000, 8000 ou 10001 pelo byte +0x42; Copa
/// do Mundo 0xabe/10000).
unsafe fn cqc12_limites(nacao: usize, p: usize) -> (i16, i16) {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    // 0x5c6710: push [p + 0x14]; mov ecx, [0xae2c74] -> thiscall ret 4
    if cqc12_d(nacao) != cqc12_d(0x9cf224) {
        return (-1, 0x2711);
    }
    if p == 0 || (cqc12_d(0xae2c74) != 0 && amistoso(cqc12_d(0xae2c74), cqc12_d(p + 0x14)) != 0) {
        return (0x8ca, 0x2134);
    }
    let k = cqc12_d(cqc12_d(p + 0x14));
    if k == cqc12_d(0x9cf8a0) {
        (0x8ca, 0x2134)
    } else if k == cqc12_d(0x9cf964) {
        (0xabe, 0x222e)
    } else if k == cqc12_d(0x9cf76c) {
        (0x8ca, match cqc12_b(p + 0x42) { 1 => 0x1f40, 2 => 0x2711, _ => 0x2328 })
    } else if k == cqc12_d(0x9cf79c) {
        (0xabe, 0x2710)
    } else {
        (-1, 0x2711)
    }
}

/// O time do tipo (0x53b3d0 a principal nos tipos 0 e 1, 0x53b4f0 o sub-21), a marca de elenco (+0x5e: 8 no tipo
/// 1, 4 nos outros) e o time da tática (1 no tipo 1).
unsafe fn cqc12_time(nacao: usize, t: i8) -> (usize, u8, u32) {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    // 0x53b3d0 e 0x53b4f0: push nacao; add esp, 4 -> cdecl
    match t {
        1 => (principal(nacao), 8, 1),
        0 => (principal(nacao), 4, 0),
        _ => (sub21(nacao), 4, 0),
    }
}

/// Os tipos de reserva percorridos: só o do tipo, fora o sub-21 (2) dos Jogos Olímpicos ([0x9cf7a0]), que olha as
/// reservas 0 e 1; o sub-21 fora deles olha a 1.
unsafe fn cqc12_faixa(t: i8, p: usize) -> (usize, usize) {
    if t != 2 {
        (0, 0)
    } else if p != 0 && cqc12_d(p) == cqc12_d(0x9cf7a0) {
        (0, 1)
    } else {
        (1, 1)
    }
}

/// As entradas das reservas do tipo `k`, grupo `g`: (vetor [this + 0xc + 4k] a partir do início, quantas). O início é
/// o dword +8 + 0x15k do registro mais os bytes +0xe + 0x15k + 4j dos grupos anteriores; a quantidade, o byte +0xd +
/// 0x15k + 4g (como a original lê: a soma e a quantidade saem de bytes vizinhos).
unsafe fn cqc12_grupo(this: usize, reg: usize, k: usize, g: usize) -> (usize, i32, i32) {
    let base = reg + 0xd + 0x15 * k;
    let mut ini = cqc12_d(base - 5) as i32;
    for j in 0..g {
        ini = ini.wrapping_add(cqc12_b(base + 1 + 4 * j) as i32);
    }
    let fim = ini.wrapping_add(cqc12_b(base + 4 * g) as i32);
    (cqc12_d(this + 0xc + 4 * k).wrapping_add((ini as usize).wrapping_mul(4)), ini, fim)
}

/// O jogador `j` das reservas entra na lista da escala? (0x76cb84..0x76cf6c)
#[allow(clippy::too_many_arguments)]
unsafe fn cqc12_candidato(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), j: usize) -> bool {
    let tipo_ok: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let bits: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x76db00usize);
    // 0x53b620: push byte +0x3d; add esp, 4 -> cdecl; 0x62c400: push j; mov ecx, [0xb63d28] -> thiscall ret 4 (al);
    // 0x7a7eb0: push j; mov ecx, [0xb7434c] -> stdcall ret 4 (al); 0x547d30: push clube; push j; add esp, 0xc (junto
    // com o da 0x53b3d0) -> cdecl; 0x4cf5a0: push j; mov ecx, [0xae1050] -> thiscall ret 4; 0x55d270: push 0; push 1;
    // push [p + 0x14]; push j; mov ecx, [0xae28f0] -> thiscall ret 0x10; 0x76db00: 6 pushes (nação, 0, j, 0, 0, 0);
    // mov ecx, this -> thiscall ret 0x18
    if j == 0 || tipo_ok(cqc12_b(j + 0x3d) as u32) == 0 || lesao(cqc12_d(0xb63d28), j) != 0 {
        return false;
    }
    if cqc12_d(0xb7434c) != 0 && bits(j) != 0 {
        return false;
    }
    if cqc12_d(j + 0x39) == 0 || nao_gosta(j, principal(cqc12_d(j + 0x1a))) != 0 {
        return false;
    }
    let tj = cqc12_d(j + 0x24);
    if tj != 0 && (cqc12_b(j + 0x28) != 0xb || cqc12_d(tj + 0x53) != nacao) {
        return false;
    }
    if p != 0 {
        let k = cqc12_d(p);
        let c = cqc12_d(j + 0x39);
        let outra_nacao = |c: usize| c != 0 && cqc12_d(c + 0x53) != 0 && cqc12_d(cqc12_d(c + 0x53) + 0x71) != cqc12_d(nacao + 0x71);
        let mut contrato_ok = true;
        if k == cqc12_d(0x9cf668) || k == cqc12_d(0x9cf794) || k == cqc12_d(0x9cf798) {
            if outra_nacao(c) {
                // amistoso, B e sub-21 internacionais: clube de outra nação só com reputação (+0x80) até 0.8 da do
                // adversário (o mandante +0x1c é o próprio time: o visitante +0x20); fora dos amistosos, não
                if k != cqc12_d(0x9cf668) {
                    return false;
                }
                let s = cqc12_d(p + 0x1c);
                let outro = if s == elenco { cqc12_d(p + 0x20) } else { s };
                if cqc12_w(c + 0x80) as f64 * 0.8 > cqc12_w(outro + 0x80) as f64 {
                    return false;
                }
            }
            // clube com jogo (a tabela +0x24 pelo id do clube) e contrato com +0x3d acima de −10: não
            if cqc12_b(cqc12_d(this + 0x24).wrapping_add(cqc12_d(c))) != 0 {
                let x = contrato(cqc12_d(0xae1050), j);
                if x != 0 && (cqc12_b(contrato(cqc12_d(0xae1050), j) + 0x3d) as i8) > -10 {
                    contrato_ok = false;
                }
            }
            if !contrato_ok {
                return false;
            }
        } else if outra_nacao(c) && cqc12_d(c + 0x57) != 0
            && (cqc12_w(cqc12_d(p + 0x14) + 0x69) as f64) < cqc12_w(cqc12_d(c + 0x57) + 0x69) as f64 * 0.7
        {
            // nas outras competições, clube de outra nação cuja liga (+0x57) tem +0x69 acima da da competição / 0.7
            return false;
        }
        if suspenso(cqc12_d(0xae28f0), j, cqc12_d(p + 0x14), 1, 0) != 0 {
            return false;
        }
        if t == 0 {
            let m = cqc12_uw(p + 0x4d) & 0x300;
            let tj = cqc12_d(j + 0x24);
            if m != 0x100 && tj == elenco && cqc12_b(j + 0x5e) & 8 != 0 {
                return false;
            }
            if m != 0x200 && tj != 0 && tj != elenco {
                return false;
            }
        }
    }
    if cqc12_d(j + 0x24) == principal(nacao) && cqc12_b(j + 0x5e) & 4 != 0 {
        if pode_mudar(this, nacao, 0, j, 0, 0, 0) == 0 {
            return false;
        }
        if !(p != 0 && cqc12_d(p) == cqc12_d(0x9cf7a0)) && t != 0 {
            return false;
        }
    }
    // com a marca, só depois da data do elenco do tipo (ano +2 sem sinal, dia +0 com sinal)
    let depois = |a: usize| {
        let (ya, da) = (cqc12_uw(data + 2), cqc12_w(data));
        let (yr, dr) = (cqc12_uw(a + 2), cqc12_w(a));
        ya > yr || (ya == yr && da > dr)
    };
    if cqc12_b(j + 0x5e) & 8 != 0 && !depois(reg + 0x4e) {
        return false;
    }
    let tj = cqc12_d(j + 0x24);
    if tj == sub21(nacao) && cqc12_b(j + 0x5e) & 4 != 0 && !depois(reg + 0x62) {
        return false;
    }
    if t == 2 && cqc12_b(j + 0x18) as i8 > 0x17 { // cqc12: a idade do sub-21
        return false;
    }
    let nat = cqc12_d(j + 0x1a);
    if nat != nacao {
        // de outra nação: sem time, sem a marca +0x22, sem o bit 2 do +0xc do registro de 79 bytes, com a segunda
        // nação (+0x1e) nesta; entra se esta tiver +0x8e maior, ou (clube da mesma confederação +0x71 ou sem clube)
        // com +0x85 de 5 para cima
        if tj != 0 || cqc12_b(j + 0x22) != 0 || cqc12_b(cqc12_d(0xae2d0c).wrapping_add(cqc12_d(j).wrapping_mul(0x4f)) + 0xc) & 2 != 0
            || cqc12_d(j + 0x1e) != nacao || nat == 0
        {
            return false;
        }
        let c = cqc12_d(j + 0x39);
        let n2 = if c != 0 { cqc12_d(c + 0x53) } else { 0 };
        if cqc12_w(nacao + 0x8e) <= cqc12_w(nat + 0x8e) {
            if n2 != 0 && cqc12_d(nacao + 0x71) != cqc12_d(n2 + 0x71) {
                return false;
            }
            if (cqc12_b(nacao + 0x85) as i8) < 5 {
                return false;
            }
        }
    }
    let v = cqc12_w(cqc12_d(j + 0x61) + 9);
    v > lim.0 && v <= lim.1
}

/// A lista da escala (0x76cae5..0x76cfe6): as reservas dos tipos da faixa, grupo a grupo; devolve a quantidade (byte,
/// como a original) e deixa em `cursor` o ponteiro da última entrada percorrida mais 4 (a original o lê depois).
#[allow(clippy::too_many_arguments)]
unsafe fn cqc12_candidatos(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), cand: &mut [usize; 200], cursor: &mut usize) -> u8 {
    let mut n = 0u8;
    let (lo, hi) = cqc12_faixa(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (v, ini, fim) = cqc12_grupo(this, reg, k, g);
            *cursor = v;
            for _ in ini..fim {
                let j = cqc12_d(*cursor);
                if cqc12_candidato(this, nacao, t, data, p, elenco, reg, lim, j) {
                    // ponytail: a original grava em local_32c[(char) n]; de 128 em diante ela escreveria antes do vetor
                    if (n as i8) >= 0 {
                        cand[n as usize] = j;
                    }
                    n = n.wrapping_add(1);
                }
                *cursor += 4;
            }
        }
    }
    n
}

/// { 0x76c120 } (thiscall, ret 0x14, national_teams.cpp, ~4,9 KB, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::select_squad(nação, tipo, data, partida, forçado)`, pela convocação (0x76bab0, em Rust: nação +0x53
/// do lado, tipo, 0xae2c90, partida, 0), pela montagem das tabelas (0x76a840: partida 0) e pelas notícias (0x784100:
/// forçado 1). Tipo 0 principal, 1 B/olímpico (marca 8, time da tática 1), 2 sub-21 (0x53b4f0). Sem `forçado`: nas
/// fases finais (+0x14 da partida numa das 9 chaves) um item (time, competição, data do jogo − 10, 1000) entra na lista
/// em +0 (0x68d0d0, criada com 0x68d020 se faltar); a data do elenco do tipo (+0x3a do registro: hoje − 1 sem partida,
/// a do jogo nos tipos 0 e acima de 2, a do jogo − 1 nos tipos 1 e 2, que antes marcam as duas nações no vetor
/// [this + 0x36] nos Jogos Olímpicos ou dispensam o sub-21 marcado, 0x76d880) e a de hoje (+0x32); as idades e o
/// tamanho (0x76d430 em +0x42/+0x43). Com técnico humano que decide (0x600a50, 0x5fd6b0; ou pela seleção principal
/// de uma seleção B sem técnico): os machucados por mais de 10 dias (0x62c400 = 2, 0x62ed70 +0x2e) saem do elenco; se
/// ainda for dele, o pedido ao técnico (0x76f1a0) e fim. Senão: os códigos das vagas (cqc12_codigos, com a correção
/// de 23 e 25); saem do elenco os de outro time, os com a marca e, no tipo 0 com o bit 1 de +0x4e da partida, os sem
/// ela (0x896c60 e +0x5e/+0x24/+0x28); a lista das reservas (cqc12_candidatos) vai à escala (0x6e2000, modo 0x2100914
/// no tipo 1, 0x2000814 no 0 e 0x814 no 2); cada escalado que serve na vaga (0x5b7d20) entra no elenco: sai do time
/// anterior (vaga, 0x76db00, 0x896c60) ou das vagas da tática do próprio (0x8968f0/0x896e90, 0x76db00), ganha a marca
/// (0x540980), o time e +0x28 (0xc de 5 ou 0xc, senão 0xb) e o +9 do registro de 79 bytes vai a 0..100. Por fim o
/// elenco é ordenado (0x945333 com 0x778ef0), a notícia (0x76e0b0) e as vagas da tática com quem não é do time saem.
/// As caixas de erro (linhas 0x748 e 0x76b) como a original.
unsafe extern "thiscall" fn escolher_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let inserir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76d880usize);
    let idades: unsafe extern "stdcall" fn(usize, u32, usize, usize) = core::mem::transmute(0x76d430usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x5fd6b0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let dias_de_lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    let pedir: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x76f1a0usize);
    let escalar: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(0x6e2000usize);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x76db00usize);
    let da_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x8968f0usize);
    let limpar_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896e90usize);
    let marcar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(0x76e0b0usize);
    // 0x549ef0: push +0x28; push +0x2a; lea ecx, saida -> thiscall ret 8 (palavras); 0x54c8e0: push n; push saida; mov
    // ecx, data -> thiscall ret 8; 0x417c10: push origem; push saida; lea ecx, destino -> thiscall ret 8; 0x54a110: lea
    // ecx -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 -> cdecl; 0x68d020: mov ecx, bloco -> thiscall ret 0;
    // 0x68d0d0: push 0x12; push item; mov ecx, [this] -> thiscall ret 8; 0x76d880: push nação; mov ecx, this ->
    // thiscall ret 4; 0x76d430: 4 pushes (partida, tipo, +0x42, +0x43) -> stdcall ret 0x10; 0x600a50: 5 pushes (time,
    // 0, 1, 0, 0); add esp -> cdecl; 0x5fd6b0: push 0; push técnico; mov ecx, [0xb63c98] -> thiscall ret 8 (al);
    // 0x5409b0: push; add esp, 4 -> cdecl; 0x62ed70: push j; mov ecx, [0xb63d28] -> thiscall ret 4; 0x896c60: push 1;
    // push time; push j; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0x10; 0x76f1a0: 3 pushes (elenco, time,
    // partida); mov ecx, this -> ret 0xc; 0x6e2000: 15 pushes; add esp, 0x3c -> cdecl; 0x5b7d20: push +0x61; add esp,
    // 4 -> cdecl (ax); 0x8968f0 e 0x896e90: push time; push vaga; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0xc;
    // 0x540980: push marca; push j; add esp, 8 -> cdecl; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x76e0b0:
    // 7 pushes; mov ecx, this -> ret 0x1c
    let t = tipo as u8 as i8;
    let reg = cqc12_registro(this, nacao);
    let lim = cqc12_limites(nacao, p);
    let (elenco, marca, time) = cqc12_time(nacao, t);
    let chaves = [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964];
    let finais = p != 0 && cqc12_d(p + 0x14) != 0 && chaves.iter().any(|&k| cqc12_d(cqc12_d(p + 0x14)) == cqc12_d(k));
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tatica = || cqc12_d(0xde1d08);
    if forcado == 0 {
        let (mut d88c, mut d87c, mut d8a0) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p88c, p87c, p8a0) = (d88c.as_mut_ptr() as usize, d87c.as_mut_ptr() as usize, d8a0.as_mut_ptr() as usize);
        if finais {
            crate::comparacao::ramo("cqc_12 elenco das fases finais");
            let d = data_nova(p87c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            menos(d, p88c, 10);
            let comp = cqc12_d(p + 0x14);
            if cqc12_d(this) == 0 {
                let m = novo(0xe);
                cqc12_pd(this, if m != 0 { lista_nova(m) } else { 0 });
            }
            let mut item = [0u8; 0x12];
            let pi = item.as_mut_ptr() as usize;
            padrao(pi + 8);
            cqc12_pd(pi + 4, cqc12_d(comp));
            cqc12_pd(pi, cqc12_d(elenco));
            copia(pi + 8, p87c, p88c);
            core::ptr::write_unaligned((pi + 0x10) as *mut u16, 1000);
            inserir(cqc12_d(this), pi, 0x12);
        }
        if p == 0 {
            let x = menos(data, p88c, 1);
            copia(r + 0x3a, p87c, x);
        } else if t != 1 && t != 2 {
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            copia(r + 0x3a, p87c, x);
        } else {
            // a nação de cada lado (+0x53 do time) e a casa dela no vetor [this + 0x36]
            let nac = |o: usize| cqc12_d(cqc12_d(p + o) + 0x53);
            let casa = |o: usize| cqc12_d(nac(o)).wrapping_add(cqc12_d(this + 0x36));
            if cqc12_d(p) == cqc12_d(0x9cf7a0) {
                cqc12_pb(casa(0x1c), 1);
                cqc12_pb(casa(0x20), 1);
            } else {
                if cqc12_b(casa(0x1c)) != 0 {
                    dispensar(this, nac(0x1c));
                }
                if cqc12_b(casa(0x20)) != 0 {
                    dispensar(this, nac(0x20));
                }
                cqc12_pb(casa(0x1c), 0);
                cqc12_pb(casa(0x20), 0);
            }
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            let y = menos(x, p87c, 1);
            copia(r + 0x3a, p8a0, y);
        }
        copia(r + 0x32, p88c, data);
        idades(p, tipo, r + 0x42, r + 0x43);
        let tecnico = cqc12_d(principal(nacao) + 0xcf);
        let decide = || {
            (humano(elenco, 0, 1, 0, 0) != 0 && (t == 0 || t9a06(cqc12_d(0xb63c98), tecnico, 0) != 0))
                || (segunda(elenco) != 0 && cqc12_d(elenco + 0xcf) == 0
                    && humano(principal(cqc12_d(elenco + 0x53)), 0, 1, 0, 0) != 0
                    && t9a06(cqc12_d(0xb63c98), tecnico, 0) != 0)
        };
        if decide() {
            for k in 0..50 {
                let s = elenco + 0xd7 + 4 * k;
                let j = cqc12_d(s);
                if j == 0 || lesao(cqc12_d(0xb63d28), j) != 2 || cqc12_b(j + 0x5e) & marca == 0 {
                    continue;
                }
                let l = dias_de_lesao(cqc12_d(0xb63d28), j);
                if l != 0 && cqc12_w(l + 0x2e) > 10 {
                    tirar(tatica(), elenco, j, time, 1);
                    cqc12_pb(j + 0x5e, cqc12_b(j + 0x5e) & 3);
                    cqc12_pd(j + 0x24, 0);
                    cqc12_pb(j + 0x28, if cqc12_b(j + 0x28) == 0xc { 5 } else { 0 });
                    cqc12_pd(s, 0);
                }
            }
        }
        if decide() {
            crate::comparacao::ramo("cqc_12 elenco pelo técnico humano");
            cqc12_pb(r + 0x44, 0);
            pedir(this, elenco, time, p);
            return;
        }
    }
    let tam = r + 0x42;
    let codigos = cqc12_codigos(cqc12_b(tam));
    // saem do elenco: os de outro time; os com a marca; no tipo 0 com o bit 1 de +0x4e da partida, os sem ela
    for k in 0..50 {
        let s = elenco + 0xd7 + 4 * k;
        let j = cqc12_d(s);
        if j == 0 {
            continue;
        }
        if cqc12_d(j + 0x24) != elenco {
            tirar(tatica(), elenco, j, time, 1);
        } else {
            let antigo = if cqc12_b(j + 0x5e) & marca != 0 {
                5
            } else if t == 0 && cqc12_b(p + 0x4e) & 1 != 0 {
                0xb
            } else {
                continue;
            };
            tirar(tatica(), elenco, j, time, 1);
            cqc12_pb(j + 0x5e, cqc12_b(j + 0x5e) & 3);
            cqc12_pd(j + 0x24, 0);
            cqc12_pb(j + 0x28, if cqc12_b(j + 0x28) == 0xc { antigo } else { 0 });
        }
        cqc12_pd(s, 0); // cqc12: a vaga do elenco fica livre
    }
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let n = cqc12_candidatos(this, nacao, t, data, p, elenco, reg, lim, &mut cand, &mut cursor);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut saida = [0usize; 50];
    crate::comparacao::ramo("cqc_12 escala do elenco");
    escalar(cand.as_ptr() as usize, n as u32, codigos.as_ptr() as usize, (modo & !0xff) | cqc12_b(tam) as u32, 0,
        saida.as_mut_ptr() as usize, modo, elenco, 0, 0, 0, 0, 0, 0, 0);
    let mut i = 0usize;
    // ponytail: a original não limita a 50 (o jogo pede até 26)
    while (i as i32) < cqc12_b(tam) as i8 as i32 && i < 50 {
        let j = saida[i];
        if j != 0 && codigos[i] & mascara(cqc12_d(j + 0x61)) != 0 {
            if let Some(k) = (0..50).find(|&k| cqc12_d(elenco + 0xd7 + 4 * k) == 0) {
                cqc12_pd(elenco + 0xd7 + 4 * k, j);
            }
            let tj = cqc12_d(j + 0x24);
            if tj != elenco && tj != 0 {
                // de outro time: sai da vaga de lá (com a permissão, a marca de convocado do tipo dele zera) e da tática
                let v = ((cqc12_b(j + 0x5e) as i8 as i32 & 8) >> 3) as u32;
                let c3: u8 = if segunda(tj) == 0 { (v != 0) as u8 } else if v != 0 { 1 } else { 2 };
                let mut c = cqc12_d(j + 0x24) + 0xd7;
                for _ in 0..50 {
                    if cqc12_d(c) == j {
                        cqc12_pd(c, 0);
                        if pode_mudar(this, cqc12_d(cqc12_d(j + 0x24) + 0x53), c3 as u32, 0, 0, 0, 0) != 0 {
                            cqc12_pb(reg + 0x43 + c3 as usize * 0x14, 0);
                            break;
                        }
                        aviso_em(0xa5b2a4, 0x748);
                    }
                    c += 4;
                }
                cursor = c;
                tirar(tatica(), cqc12_d(j + 0x24), j, v, 1);
            } else if tj == elenco && cqc12_b(j + 0x5e) & marca == 0 {
                // já do time sem a marca: sai das vagas da tática do time (time da tática invertido)
                let bz = (time == 0) as u32;
                let c3: u8 = if segunda(elenco) == 0 { (bz != 0) as u8 } else if bz != 0 { 1 } else { 2 };
                for v in 0..20 {
                    if da_vaga(tatica(), cqc12_d(j + 0x24), v, bz) != j {
                        continue;
                    }
                    limpar_vaga(tatica(), cqc12_d(j + 0x24), v, bz);
                    // a original passa como jogador o dword do último ponteiro de percurso (cursor): reproduzido
                    if c3 != 0 && pode_mudar(this, cqc12_d(cqc12_d(j + 0x24) + 0x53), c3 as u32, cqc12_d(cursor), 0, 0, 0) != 0 {
                        cqc12_pb(reg + 0x43 + c3 as usize * 0x14, 0);
                        break;
                    }
                    aviso_em(0xa5b2a4, 0x76b);
                }
            }
            marcar(j, marca as u32);
            cqc12_pd(j + 0x24, elenco); // cqc12: o convocado ganha o time
            let x = cqc12_b(j + 0x28);
            cqc12_pb(j + 0x28, if x == 5 || x == 0xc { 0xc } else { 0xb });
            let e = cqc12_d(0xae2d0c).wrapping_add(cqc12_d(j).wrapping_mul(0x4f)) + 9;
            cqc12_pb(e, (cqc12_b(e) as i8).clamp(0, 100) as u8);
        }
        i += 1;
    }
    ordenar(elenco + 0xd7, 0x32, 4, 0x778ef0);
    let chave = if p == 0 { u32::MAX } else { cqc12_d(cqc12_d(p + 0x14)) as u32 };
    noticia(this, elenco, time, p, chave, finais as u32, 0, 0);
    for v in 0..20 {
        let x = da_vaga(tatica(), elenco, v, time);
        if x != 0 && cqc12_d(x + 0x24) != elenco {
            limpar_vaga(tatica(), elenco, v, time);
        }
    }
}

/// O que a escala da 0x76c120 lê e a original muda depois dela: o registro da nação (0x6e bytes) e, de cada jogador
/// das reservas percorridas, +0x24..+0x28 (time, estado) e +0x5e (marcas de elenco).
unsafe fn cqc12_regioes(this: usize, nacao: usize, t: i8, p: usize) -> Vec<(usize, usize)> {
    let reg = cqc12_registro(this, nacao);
    let mut v = vec![(reg, 0x6e)];
    let (lo, hi) = cqc12_faixa(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (a, ini, fim) = cqc12_grupo(this, reg, k, g);
            for i in 0..(fim - ini).max(0) as usize {
                let j = cqc12_d(a + 4 * i);
                if j != 0 {
                    v.push((j + 0x24, 5));
                    v.push((j + 0x5e, 1));
                }
            }
        }
    }
    v
}

/// Comparador: a original aloca (0x68d020/0x68d0d0), mexe nos elencos, nas vagas da tática e manda notícias (0x76e0b0,
/// 0x76f1a0): não pode rodar duas vezes. Só ela roda, com uma gravação da fita aberta nas regiões de cqc12_regioes:
/// a escala (0x6e2000) entra nela como unidade com os vetores de entrada (a lista e os códigos) e o estado das
/// regiões logo antes. Depois, com esse estado de volta, a cópia Rust monta a lista (cqc12_candidatos) e os códigos
/// (cqc12_codigos), que têm de bater com os da original, junto com a quantidade, o modo e o time; o estado de depois
/// da original volta no fim. Nos elencos de 23 e 25 a original passa a sobra da pilha nas vagas a partir da 22ª: elas
/// ficam fora (correção conhecida, o ramo); as 22 primeiras e o resto conferem. Sem escala (técnico humano), nada a
/// comparar. Dentro de uma gravação de fora, só a original.
unsafe extern "thiscall" fn comparar_escolher_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32)>(1542);
    if gravando() {
        return o(this, nacao, tipo, data, p, forcado);
    }
    let t = tipo as u8 as i8;
    let regioes = cqc12_regioes(this, nacao, t, p);
    let externa = fita().take();
    *fita() = Some((regioes.clone(), Vec::new()));
    let nivel = DENTRO.swap(0, Relaxed);
    o(this, nacao, tipo, data, p, forcado);
    let gravadas = fita().take().map_or(Vec::new(), |f| f.1);
    DENTRO.store(nivel, Relaxed);
    *fita() = externa;
    let Some((_, _, a, _, antes, _, (ent, _))) = gravadas.iter().find(|c| c.0 == 0x6e2000) else {
        return;
    };
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    antes.escrever();
    let reg = cqc12_registro(this, nacao);
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tam = cqc12_b(r + 0x42);
    let (elenco, _, _) = cqc12_time(nacao, t);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let (n, codigos) = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        let n = cqc12_candidatos(this, nacao, t, data, p, elenco, reg, cqc12_limites(nacao, p), &mut cand, &mut cursor);
        (n, cqc12_codigos(tam))
    }));
    sorteio.escrever();
    depois.escrever();
    let n1 = (a[1] as u8 as i8).max(0) as usize;
    let dw = |k: usize| u32::from_le_bytes([ent[k], ent[k + 1], ent[k + 2], ent[k + 3]]) as usize;
    let lista_o: Vec<usize> = (0..n1.min(ent.len() / 4)).map(|k| dw(4 * k)).collect();
    let cod_o: Vec<u16> = ent.get(4 * n1..).unwrap_or(&[]).chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let corrigido = tam == 23 || tam == 25;
    let lim = if corrigido { 22.min(cod_o.len()) } else { cod_o.len() };
    let igual = a[1] as u8 == n && a[2] == modo && a[3] as usize == elenco && cod_o.len() == tam as usize
        && lista_o[..] == cand[..n1.min(200)] && cod_o[..lim] == codigos[..lim];
    PLACAR.registrar(igual, || format!(
        "escolher_elenco_da_selecao(nação {:x}, tipo {}, tam {}) original: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}; rust: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}",
        nacao, t, tam, a[1] as u8, a[2], a[3], lista_o, cod_o, n, modo, elenco, &cand[..(n as i8).max(0) as usize], &codigos[..tam as usize]));
}

// ---------- 266a leva: os olheiros dos técnicos humanos na partida e a sessão de edição das buscas (scout_manager.cpp) ----------

unsafe fn cqc13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x7f8c00 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `SCOUT_MANAGER::scout_match(partida, resultado)`, pelo fechamento da partida do dia (0x6b5f00, Rust). Para cada um
/// dos 16 últimos registros de pessoa ([0xae23bc], 0x6e bytes, do último para trás) com +0x5f = 1 (o técnico humano):
/// num vetor zerado de 10 marcas (0x945501), os 10 observadores numa ordem sorteada (0x90cfc0 de 10 até achar um não
/// marcado); o registro de cada um (0x7f6cd0) com +0x19d, numa partida com as marcas +0x40 e +0x41 e do clube do
/// técnico (+0x39 = +0x1c ou +0x20 da partida), começa a observação (0x7ae9c0 com −1, −1, −1, 0, 1); e observa a
/// partida (0x7b5be0, os relatórios na lista local). O vetor sai (0x9452ca). Por fim os relatórios vão (0x7f9890) e a
/// lista é destruída. Devolve 1.
unsafe extern "thiscall" fn olheiros_na_partida(this: usize, p: usize, resultado: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let registro: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let comecar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x7ae9c0usize);
    let observar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x7b5be0usize);
    let enviar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f9890usize);
    // calloc: push 4; push 0xa; add esp, 8 -> cdecl (10, 4); sorteio: push 0xa; add esp, 4 -> cdecl; free: cdecl;
    // 0x7f6cd0: push pessoa; push qual; mov ecx, [this] -> thiscall ret 8; 0x7ae9c0: 5 pushes; mov ecx, registro ->
    // thiscall ret 0x14; 0x7b5be0: push &lista; push resultado; push partida; mov ecx, registro -> thiscall ret 0xc;
    // 0x7f9890: push &lista; mov ecx, this -> thiscall ret 4; 0x68d020/0x68d040: lea ecx, lista -> thiscall
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    for k in 0..16i32 {
        let idx = (cqc13_d(0xae2364) as i32).wrapping_sub(k).wrapping_sub(1);
        let pessoa = cqc13_d(0xae23bc).wrapping_add((idx as usize).wrapping_mul(0x6e));
        if *((pessoa + 0x5f) as *const u8) != 1 {
            continue;
        }
        crate::comparacao::ramo("cqc_13 olheiros do humano");
        let marcas = zerado(10, 4);
        for _ in 0..10 { // cqc13: os 10 observadores
            let q = loop {
                let q = sortear(10) as i8; // cqc13: o observador sorteado
                if cqc13_d(marcas.wrapping_add((q as isize * 4) as usize)) == 0 {
                    break q; // cqc13: um ainda não marcado
                }
            };
            core::ptr::write_unaligned(marcas.wrapping_add((q as isize * 4) as usize) as *mut u32, 1);
            // ponytail: a original empurra o dword com o byte do observador e sobra de pilha em cima; a 0x7f6cd0 lê o
            // byte
            let r = registro(this, q as u8 as u32, pessoa);
            if cqc13_d(r + 0x19d) != 0 && *((p + 0x40) as *const u8) != 0 && *((p + 0x41) as *const u8) != 0 {
                let clube = cqc13_d(pessoa + 0x39);
                if cqc13_d(p + 0x1c) == clube || cqc13_d(p + 0x20) == clube {
                    comecar(r, u32::MAX, u32::MAX, u32::MAX, 0, 1);
                }
            }
            observar(r, p, resultado, lista);
        }
        liberar(marcas);
    }
    enviar(this, lista);
    soltar(lista);
    1
}

/// Comparador: a observação (0x7b5be0, original) grava o que os olheiros sabem e os relatórios, a 0x7ae9c0 começa
/// buscas e a 0x7f9890 entrega as notícias: não pode rodar duas vezes. Só a original roda (ponytail: ela mesma só grava
/// o vetor e a lista locais, que saem); confere a devolução (1).
unsafe extern "thiscall" fn comparar_olheiros_na_partida(this: usize, p: usize, resultado: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1543)(this, p, resultado);
    PLACAR.registrar(r == 1, || format!("olheiros_na_partida(partida {:x}) original={}", p, r));
    r
}

/// O índice (0..15) do técnico `tecnico` (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um
/// byte da original ((id − [0xae2364]) + 16 em 8 bits); None nos casos da caixa (sem técnico ou fora).
unsafe fn cqc13_indice(tecnico: usize) -> Option<usize> {
    let t = if tecnico != 0 {
        tecnico
    } else {
        cqc13_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// { 0x7f6be0 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_edit_session(tecnico)`, pela montagem dos
/// conhecidos da busca (0x7afeb7). A sessão de edição do técnico humano (+0xc + 4 × índice). Sem técnico ou índice
/// fora: a caixa (linhas 0xde e 0xe8; a original) e 0.
unsafe extern "thiscall" fn sessao_de_edicao(this: usize, tecnico: usize) -> usize {
    let Some(i) = cqc13_indice(tecnico) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1544)(this, tecnico); // a caixa
    };
    cqc13_d(this + 0xc + i * 4)
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_sessao_de_edicao(this: usize, tecnico: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1544);
    if cqc13_indice(tecnico).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = sessao_de_edicao(this, tecnico);
    PLACAR.registrar(r == n, || format!("sessao_de_edicao(tecnico {:x}) original={:x} rust={:x}", tecnico, r, n));
    r
}

/// { 0x7f6af0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_edit_session(sessao, tecnico)`, pelas telas
/// das buscas (0x8085bc, 0x812d88...). Grava a sessão em +0xc + 4 × índice do técnico e devolve 1. Sem técnico ou
/// índice fora: a caixa (linhas 0xb8 e 0xc2; a original) e 0.
unsafe extern "thiscall" fn escolher_sessao_de_edicao(this: usize, sessao: usize, tecnico: usize) -> u32 {
    let Some(i) = cqc13_indice(tecnico) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1545)(this, sessao, tecnico); // a caixa
    };
    core::ptr::write_unaligned((this + 0xc + i * 4) as *mut usize, sessao);
    1
}

/// Comparador: lado a lado pelas 16 sessões e a devolução; com a caixa, só a original.
unsafe extern "thiscall" fn comparar_escolher_sessao_de_edicao(this: usize, sessao: usize, tecnico: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1545);
    if cqc13_indice(tecnico).is_none() {
        return o(this, sessao, tecnico);
    }
    let reg = [(this + 0xc, 0x40)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, sessao, tecnico),
        || escolher_sessao_de_edicao(this, sessao, tecnico));
    PLACAR.registrar(igual && r == n, || format!("escolher_sessao_de_edicao(sessao {:x}, tecnico {:x}) original={} rust={} {:x?} x {:x?}",
        sessao, tecnico, r, n, mo, mr));
    r
}

// ---------- 266a leva: o dia das transferências: o dia da gerência, a oferta que vence hoje e as restrições da oferta (transfer_manager.cpp) ----------

unsafe fn cqd1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// A pessoa do id (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cqd1_pessoa(id: usize) -> usize {
    cqd1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do id (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqd1_clube(id: usize) -> usize {
    cqd1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080] como
/// a original faz, e o clube dele (+4); sem contrato, ou com o tipo 5 em +0x35 quando `sem_5`: 0.
unsafe fn cqd1_clube_do_contrato(pessoa: usize, sem_5: bool) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqd1_d(0xae1050), pessoa);
    cqd1_pd(0xae1080, c);
    if c == 0 || (sem_5 && cqd1_b(c + 0x35) & 0x3f == 5) {
        0
    } else {
        cqd1_clube(cqd1_d(c + 4))
    }
}

/// O clube é humano (0x600a50: push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
unsafe fn cqd1_humano(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0) != 0
}

/// Reagenda a oferta (0x8c6220: push 0; push dias; push oferta; mov ecx, this -> thiscall, ret 0xc).
unsafe fn cqd1_agendar(this: usize, o: usize, dias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, o, dias, 0);
}

/// O próximo da lista (0x68d1b0: mov ecx, lista -> thiscall, ret 0).
unsafe fn cqd1_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// O bloco que a 0x8c63a0 repete: o clube do contrato do alvo (sem olhar o tipo 5), os lances do comprador (+8) pelo
/// alvo (+0xc) desmarcados na lista de observação (0x84aeb0: push jogador; push clube; mov ecx, [this+0x856] ->
/// thiscall, ret 8) e os do clube do contrato por cada jogador da troca (lista em +4, cursor no começo).
unsafe fn cqd1_desmarcar(this: usize, o: usize) {
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let atual = cqd1_clube_do_contrato(alvo, false);
    desmarcar(cqd1_d(this + 0x856), cqd1_clube(cqd1_d(o + 8)), alvo);
    let l = cqd1_d(o + 4);
    if l == 0 {
        return;
    }
    cqd1_pd(l + 0xa, cqd1_d(l + 2));
    loop {
        let p = cqd1_proximo(cqd1_d(o + 4));
        if p == 0 {
            break;
        }
        desmarcar(cqd1_d(this + 0x856), atual, cqd1_pessoa(cqd1_d(p)));
    }
}

/// { 0x8c5880 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::daily_update(hoje, fatia)`. Sem data base (ano +0x84f zerado), ela vira hoje (0x417c10: push
/// hoje; push saída; lea ecx, [this+0x84d] -> thiscall, ret 8) e a fatia +0x855 a de hoje. Só com hoje igual à base e a
/// fatia igual a +0x855: na fatia 0 os avisos das janelas (0x8c6bc0: push hoje; mov ecx, this -> thiscall, ret 4); no
/// dia múltiplo de 7, cada oferta vencida (data +0x12/+0x14 antes de hoje; status +0x2c fora de 0x13, 7, 8 e 0xa) é
/// reagendada para amanhã (0x8c6220 com 1 dia); a fatia +0x855 sobe 1; a fila da transferência livre (0x8c18c0: mov
/// ecx, this -> thiscall, ret 0); o balde da vez (listas de 0xe bytes em this+0x18 + 0xe·(3·[+0x84c] + fatia), cursor no
/// começo): cada índice válido do vetor [this] (conta +4) com a oferta de hoje e desta fatia (+0x1a) é processado
/// (0x8c63a0: push vaga; mov ecx, this -> thiscall, ret 4); o balde é esvaziado (0x68d080). Na fatia 2: a base passa a
/// amanhã (0x54c5b0 push 1; push saída; mov ecx, base -> thiscall, ret 8 e 0x417c10), +0x855 volta a 0, +0x84c sobe (de
/// 49 volta a 0); mudando o mês de hoje para amanhã (0x54cfb0: mov ecx, data -> thiscall, ret 0), as prestações
/// (0x8caee0); no dia 0, as transferências antigas aos arquivos (0x8c3eb0) e as permissões de trabalho (0x8c7330); o
/// virtual +0 de cada uma das 28 regras de nação ([this+0x8ac], push hoje; mov ecx, regras); no dia múltiplo de 7, as
/// permissões de novo. Na fatia 0 (fora a 2), as transferências futuras (0x8cbbe0). No dia 100 da fatia 2: as datas 100
/// de 5, 2 e 10 anos atrás (0x549ef0: push ano; push 0x64; lea ecx, local -> thiscall, ret 8) e, para cada pessoa
/// (registros de 0x6e bytes em [0xae23bc], conta [0xae2364], relida) com a data +0x3e/+0x40 antes da de 2 anos, clube
/// ([[+0x39]+0x53]) numa nação ([+0x71]) de id [0x9cfa10] e sem ser comunitária lá (0x53d740: push clube; push pessoa;
/// add esp, 8 -> cdecl): +0x1e = o clube, se ele é [0x9cf488] ou [0x9cf434]; se a data é antes da de 5 anos e o clube
/// não é [0x9cf360], [0x9cf2fc] ou [0x9cf448]; ou se a data é antes da de 10 anos.
unsafe extern "thiscall" fn dia_das_transferencias(this: usize, hoje: usize, fatia: u32) {
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let chamar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c18c0usize);
    let janelas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6bc0usize);
    let processar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c63a0usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let prestacoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8caee0usize);
    let antigas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c3eb0usize);
    let permissoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c7330usize);
    let futuras: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8cbbe0usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let f = fatia as u8;
    let mut t20 = [0u8; 8];
    if cqd1_w(this + 0x84f) == 0 {
        copia(this + 0x84d, t20.as_mut_ptr() as usize, hoje);
        cqd1_pb(this + 0x855, f);
    }
    if cqd1_w(hoje + 2) != cqd1_w(this + 0x84f) || cqd1_w(hoje) != cqd1_w(this + 0x84d) || cqd1_b(this + 0x855) != f {
        return;
    }
    if f == 0 {
        janelas(this, hoje);
    }
    if cqd1_w(hoje) as i32 % 7 == 0 {
        let mut i: i32 = 0;
        while i < cqd1_d(this + 4) as i32 {
            let o = cqd1_d(cqd1_d(this).wrapping_add((i as usize).wrapping_mul(4)));
            if o != 0 {
                let (ha, oa) = (cqd1_w(hoje + 2) as u16, cqd1_w(o + 0x14) as u16);
                let vencida = ha > oa || (ha == oa && cqd1_w(hoje) > cqd1_w(o + 0x12));
                let s = cqd1_b(o + 0x2c);
                if vencida && s != 0x13 && s != 7 && s != 8 && s != 0xa {
                    cqd1_agendar(this, o, 1);
                }
            }
            i += 1;
        }
    }
    cqd1_pb(this + 0x855, cqd1_b(this + 0x855).wrapping_add(1));
    chamar(this);
    let balde = |this: usize| {
        let k = (cqd1_b(this + 0x84c) as i8 as i32) * 3 + f as i8 as i32;
        this.wrapping_add((k as isize * 0xe) as usize).wrapping_add(0x18)
    };
    let l = balde(this);
    cqd1_pd(l + 0xa, cqd1_d(l + 2));
    loop {
        let p = cqd1_proximo(balde(this));
        if p == 0 {
            break;
        }
        let idx = cqd1_d(p) as i32;
        if idx < cqd1_d(this + 4) as i32 && idx >= 0 {
            let vaga = cqd1_d(this).wrapping_add(idx as usize * 4);
            let o = cqd1_d(vaga);
            if o != 0 && cqd1_w(hoje + 2) == cqd1_w(o + 0x14) && cqd1_w(hoje) == cqd1_w(o + 0x12)
                && cqd1_b(o + 0x1a) == f { // cqd1: a oferta desta fatia
                crate::comparacao::ramo("cqd_1 oferta do dia");
                processar(this, vaga);
            }
        }
    }
    esvaziar(balde(this));
    if f == 2 {
        let (mut t28, mut t30) = ([0u8; 8], [0u8; 8]);
        let r = mais(this + 0x84d, t28.as_mut_ptr() as usize, 1);
        copia(this + 0x84d, t30.as_mut_ptr() as usize, r);
        cqd1_pb(this + 0x855, 0);
        let j = cqd1_b(this + 0x84c).wrapping_add(1);
        cqd1_pb(this + 0x84c, j);
        if (j as i8) >= 0x32 {
            cqd1_pb(this + 0x84c, 0);
        }
        let amanha = mais(hoje, t30.as_mut_ptr() as usize, 1);
        let m = mes(hoje);
        if m != mes(amanha) {
            crate::comparacao::ramo("cqd_1 prestações do mês");
            prestacoes(this); // cqd1: as prestações na virada do mês
        }
        if cqd1_w(hoje) == 0 {
            antigas(this);
            permissoes(this);
        }
        for k in 0..0x1c {
            let r = cqd1_d(this + 0x8ac + k * 4);
            if r != 0 {
                let v: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqd1_d(cqd1_d(r)));
                v(r, hoje);
            }
        }
        if cqd1_w(hoje) as i32 % 7 == 0 {
            permissoes(this);
        }
    } else if f == 0 {
        futuras(this);
    }
    if cqd1_w(hoje) != 100 || f != 2 {
        return;
    }
    // ponytail: a original empurra o ano em eax com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
    let ano = cqd1_w(hoje + 2) as u16;
    let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
    let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
    let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
    let mut s = cqd1_d(0xae23bc);
    let mut i: i32 = 0;
    while i < cqd1_d(0xae2364) as i32 {
        if cqd1_naturaliza(s, p2, p5, p10, comunitaria) {
            cqd1_pd(s + 0x1e, cqd1_d(cqd1_d(s + 0x39) + 0x53));
        }
        i += 1;
        s += 0x6e;
    }
}

/// A condição do dia 100 da 0x8c5880 (também para o comparador): a data +0x3e (dia com sinal)/+0x40 (ano sem sinal) da
/// pessoa antes da de 2 anos, o clube na nação [0x9cfa10] sem ser comunitária, e o caso pelo id do clube.
unsafe fn cqd1_naturaliza(s: usize, p2: usize, p5: usize, p10: usize, comunitaria: unsafe extern "C" fn(usize, usize) -> u32) -> bool {
    let antes = |d: usize| {
        let (da, sa) = (cqd1_w(d + 2) as u16, cqd1_w(s + 0x40) as u16);
        da > sa || (da == sa && cqd1_w(d) > cqd1_w(s + 0x3e))
    };
    if !antes(p2) {
        return false;
    }
    let j = cqd1_d(s + 0x39);
    if j == 0 {
        return false;
    }
    let c = cqd1_d(j + 0x53);
    if c == 0 {
        return false;
    }
    let n = cqd1_d(c + 0x71);
    if n == 0 || cqd1_d(n) != cqd1_d(0x9cfa10) || comunitaria(s, c) != 0 {
        return false;
    }
    let id = cqd1_d(c);
    if id == cqd1_d(0x9cf488) || id == cqd1_d(0x9cf434) {
        return true;
    }
    if antes(p5) && id != cqd1_d(0x9cf360) && id != cqd1_d(0x9cf2fc) && id != cqd1_d(0x9cf448) {
        return true;
    }
    antes(p10)
}

/// Comparador da 0x8c5880: só a original (ela processa as ofertas, que mudam elencos, finanças e notícias). Confere o
/// que ela garante, contra o estado de antes: rodando o dia (hoje igual à base e a fatia igual a +0x855), na fatia 2 a
/// base vira amanhã, +0x855 volta a 0 e +0x84c sobe (de 49 a 0); nas outras +0x855 sobe 1; e no dia 100 da fatia 2
/// cada pessoa que cumpre a condição (o laço é o último passo, então o estado de depois é o dele) fica com +0x1e = o
/// clube.
unsafe extern "thiscall" fn comparar_dia_das_transferencias(this: usize, hoje: usize, fatia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1546);
    let f = fatia as u8;
    let sem_base = cqd1_w(this + 0x84f) == 0;
    let roda = if sem_base {
        true
    } else {
        cqd1_w(hoje + 2) == cqd1_w(this + 0x84f) && cqd1_w(hoje) == cqd1_w(this + 0x84d) && cqd1_b(this + 0x855) == f
    };
    let (f855, j84c) = (if sem_base { f } else { cqd1_b(this + 0x855) }, cqd1_b(this + 0x84c));
    let mut amanha = [0u8; 8];
    {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let r = mais(hoje, amanha.as_mut_ptr() as usize, 1);
        amanha = core::ptr::read_unaligned(r as *const [u8; 8]);
    }
    o(this, hoje, fatia);
    if !roda {
        return;
    }
    let mut ok;
    if f == 2 {
        let j = j84c.wrapping_add(1);
        let esperado = if (j as i8) >= 0x32 { 0 } else { j };
        let base = core::ptr::read_unaligned((this + 0x84d) as *const [u8; 4]);
        ok = cqd1_b(this + 0x855) == 0 && cqd1_b(this + 0x84c) == esperado && base[..] == amanha[..4];
    } else {
        ok = cqd1_b(this + 0x855) == f855.wrapping_add(1);
    }
    let mut erradas = 0u32;
    if f == 2 && cqd1_w(hoje) == 100 {
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
        let ano = cqd1_w(hoje + 2) as u16;
        let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
        let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
        let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
        let mut s = cqd1_d(0xae23bc);
        for _ in 0..(cqd1_d(0xae2364) as i32).max(0) {
            if cqd1_naturaliza(s, p2, p5, p10, comunitaria) && cqd1_d(s + 0x1e) != cqd1_d(cqd1_d(s + 0x39) + 0x53) {
                erradas += 1;
            }
            s += 0x6e;
        }
        ok = ok && erradas == 0;
    }
    PLACAR.registrar(ok, || format!("dia_das_transferencias(dia {}, fatia {}): +0x855 {} -> {}, +0x84c {} -> {}, pessoas sem o clube em +0x1e {}",
        cqd1_w(hoje), f, f855, cqd1_b(this + 0x855), j84c, cqd1_b(this + 0x84c), erradas));
}

/// { 0x8c63a0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH: as listas locais, construídas pela 0x68d020 e
/// destruídas pela 0x68d040 na mesma ordem): `TRANSFER_MANAGER::process_due_offer(vaga)`, a oferta que vence hoje, pelo
/// status +0x2c. O vendedor é o clube do contrato do alvo (+0xc; fora o tipo 5), o comprador o +8.
/// - 0: sem vendedor, a aceitação do lance (0x8c0080: push 0; push vaga -> thiscall, ret 8). Vendedor humano: status 4,
///   14 dias (0x8c6220) e os lances desmarcados (cqd1_desmarcar). Senão, sem a marca +0x1f = 1, a resposta da IA
///   (0x8c8090: push 0; push vaga, ret 8); com ela, a previsão do vendedor (0x84af00, stdcall(vendedor, alvo, 0 i64,
///   comprador, −1 i64, 1): o ecx = [this+0x856] não é lido): recusa, a do empréstimo (0x8bfa30: push 0; push 0; push
///   vaga -> thiscall, ret 0xc); aceita, a decisão (0x4cc930: push oferta; mov ecx, [0xae1050] -> thiscall, ret 4),
///   status 8 e a notícia da aceitação (0x8d76d0: push oferta, ret 4); com a decisão, a notícia da recusa em se mudar
///   (0x8d8310: push 0; push oferta, ret 8), status 0xf, +0x2e = 3, os lances desmarcados, 10 dias, e se a oferta e as
///   da lista em +0 estão fechadas (0x8d0d90, stdcall(oferta)), o bit 0 do registro de 8 bytes do alvo em [this+0x86e]
///   apagado.
/// - 2, 3, 4, 0xf, 0x10, 0x14: com 0xf e +0x2e < 3, +0x2e = 3 e 10 dias; senão a oferta sai (0x8d0f60: push oferta,
///   ret 4).
/// - 1: comprador humano: status 4, os lances desmarcados e 10 dias; senão 0x8c0f40 (push vaga, ret 4) com +0x1f = 1 ou
///   0x8c0810 (push 0; push vaga, ret 8).
/// - 5, 6: comprador humano: status 9 e 10 dias; senão o contrato ao alvo (0x8c1e40: push oferta; push alvo; push
///   comprador, ret 0xc).
/// - 7 com troca (+4): vendedor humano: status 8 e 10 dias; senão, numa cópia da lista (0x68d2f0: push lista; lea ecx,
///   local, ret 4), o contrato a cada jogador da troca até a oferta ficar ativa.
/// - 0xa: as permissões (0x8c7690: push 0; push oferta, ret 8). 0xe: a conclusão (0x8c34e0: push 0; push oferta, ret 8).
/// - 0x11, 0x12: status 0; com +0x10, a oferta passa aos clubes (0x8c2f50: push oferta, ret 4); senão, com técnico
///   humano no clube do contrato (0x7875e0, stdcall(clube): o ecx = [0xb7431c] não é lido), a notícia da oferta feita
///   (0x8d3090: push oferta, ret 4) e 28 dias; senão o valor da oferta (0x8e7b50: mov ecx, oferta -> thiscall(oferta, 0,
///   0, 0, 0, 1), ret 0x14) e a previsão do vendedor (vendedor, alvo, valor, comprador, −1, 0): 0 aceita (0x8c0080),
///   1 ou 2 recusa (0x8bf680: push 0; push 0; push vaga, ret 0xc).
unsafe extern "thiscall" fn processar_oferta_vencida(this: usize, vaga: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut l1 = [0u8; 0x10];
    lista_nova(l1.as_mut_ptr() as usize);
    cqd1_processar(this, vaga);
    destruir(l1.as_mut_ptr() as usize);
}

unsafe fn cqd1_processar(this: usize, vaga: usize) {
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0080usize);
    let responder: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c8090usize);
    let prever: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(0x84af00usize);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bfa30usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bf680usize);
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cc930usize);
    let aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d76d0usize);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d8310usize);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x8d0d90usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d0f60usize);
    let ia_1a: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c0f40usize);
    let ia_1b: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0810usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x8c1e40usize);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8c7690usize);
    let concluir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c34e0usize);
    let passar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let tecnico_humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_feita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d3090usize);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let o = cqd1_d(vaga);
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let vendedor = cqd1_clube_do_contrato(alvo, true);
    let comprador = cqd1_clube(cqd1_d(o + 8));
    let s = cqd1_b(o + 0x2c);
    if s == 0 {
        if vendedor == 0 {
            aceitar(this, vaga, 0);
            return;
        }
        if cqd1_humano(vendedor) {
            cqd1_pb(o + 0x2c, 4);
            cqd1_agendar(this, o, 0xe);
            cqd1_desmarcar(this, o);
            return;
        }
        if cqd1_b(cqd1_d(vaga) + 0x1f) != 1 {
            crate::comparacao::ramo("cqd_1 resposta da IA");
            responder(this, vaga, 0); // cqd1: a resposta da IA ao lance
            return;
        }
        if prever(vendedor, alvo, 0, comprador, -1, 1) != 0 {
            recusar_emprestimo(this, vaga, 0, 0);
            return;
        }
        let r = decidir(cqd1_d(0xae1050), cqd1_d(vaga));
        cqd1_pb(cqd1_d(vaga) + 0x2c, 8);
        aceita(this, cqd1_d(vaga));
        if r == 0 {
            return;
        }
        recusa_mudar(this, cqd1_d(vaga), 0);
        cqd1_pb(cqd1_d(vaga) + 0x2c, 0xf);
        cqd1_pb(cqd1_d(vaga) + 0x2e, 3);
        cqd1_desmarcar(this, cqd1_d(vaga));
        cqd1_agendar(this, cqd1_d(vaga), 0xa);
        let o3 = cqd1_d(vaga);
        if fechada(o3) == 0 {
            return;
        }
        let l = cqd1_d(o3);
        if l != 0 {
            cqd1_pd(l + 0xa, cqd1_d(l + 2));
            loop {
                let p = cqd1_proximo(cqd1_d(o3));
                if p == 0 {
                    break;
                }
                if fechada(cqd1_d(cqd1_d(this).wrapping_add(cqd1_d(p).wrapping_mul(4)))) == 0 {
                    return;
                }
            }
        }
        let m = cqd1_d(this + 0x86e).wrapping_add(cqd1_d(o3 + 0xc).wrapping_mul(8));
        if cqd1_b(m) & 1 != 0 {
            cqd1_pb(m, cqd1_b(m) & 0xfe);
        }
        return;
    }
    if matches!(s, 2 | 3 | 4 | 0xf | 0x10 | 0x14) {
        if s == 0xf && cqd1_b(o + 0x2e) < 3 {
            cqd1_pb(o + 0x2e, 3);
            cqd1_agendar(this, o, 0xa);
        } else {
            tirar(this, o);
        }
        return;
    }
    if s == 1 {
        if cqd1_humano(comprador) {
            cqd1_pb(o + 0x2c, 4);
            cqd1_desmarcar(this, o);
            cqd1_agendar(this, o, 0xa);
        } else if cqd1_b(cqd1_d(vaga) + 0x1f) == 1 {
            ia_1a(this, vaga);
        } else {
            ia_1b(this, vaga, 0);
        }
        return;
    }
    if s == 5 || s == 6 {
        if cqd1_humano(comprador) {
            cqd1_pb(o + 0x2c, 9);
            cqd1_agendar(this, o, 0xa);
        } else {
            crate::comparacao::ramo("cqd_1 contrato ao jogador");
            contrato(this, comprador, cqd1_pessoa(cqd1_d(o + 0xc)), o); // cqd1: o contrato ao alvo
        }
        return;
    }
    if cqd1_d(o + 4) != 0 && s == 7 {
        if cqd1_humano(vendedor) {
            cqd1_pb(o + 0x2c, 8);
            cqd1_agendar(this, o, 0xa);
            return;
        }
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d2f0usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let mut l2 = [0u8; 0x10];
        let l = l2.as_mut_ptr() as usize;
        lista_nova(l);
        copiar(l, cqd1_d(o + 4));
        cqd1_pd(l + 0xa, cqd1_d(l + 2));
        loop {
            let p = cqd1_proximo(l);
            if p == 0 {
                break;
            }
            contrato(this, vendedor, cqd1_pessoa(cqd1_d(p)), o);
            let s2 = cqd1_b(o + 0x2c);
            if matches!(s2, 2 | 3 | 4 | 0x10 | 0x14) || (s2 == 0xf && cqd1_b(o + 0x2e) >= 3) {
                break;
            }
        }
        destruir(l);
        return;
    }
    if s == 0xa {
        permissoes(this, o, 0);
        return;
    }
    if s == 0xe {
        concluir(this, o, 0);
        return;
    }
    if s != 0x11 && s != 0x12 {
        return;
    }
    cqd1_pb(o + 0x2c, 0);
    if cqd1_b(o + 0x10) != 0 {
        passar(this, o);
        return;
    }
    let clube = cqd1_clube_do_contrato(cqd1_pessoa(cqd1_d(o + 0xc)), false);
    if tecnico_humano(clube) != 0 {
        noticia_feita(this, o);
        cqd1_agendar(this, o, 0x1c);
        return;
    }
    let v = valor(o, 0, 0, 0, 0, 1);
    let r = prever(vendedor, cqd1_pessoa(cqd1_d(o + 0xc)), v as i64, comprador, -1, 0);
    if r == 0 {
        aceitar(this, vaga, 0);
    } else if (r as i8) > 0 && (r as i8) <= 2 {
        recusar(this, vaga, 0, 0);
    }
}

/// Comparador da 0x8c63a0: só a original (cada ramo muda o status, reagenda, aloca notícias, conclui ou tira a oferta).
unsafe extern "thiscall" fn comparar_processar_oferta_vencida(this: usize, vaga: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1547)(this, vaga) // ponytail: só a original
}

/// { 0x8cfc90 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH: a data e a lista locais, a lista construída pela
/// 0x68d020 e destruída pela 0x68d040): `TRANSFER_MANAGER::check_offer_restrictions(oferta, decide, motivo, sem_elenco)`;
/// 0 libera, 1 ou 2 barram. O comprador é o +8, o alvo o +0xc e o vendedor o clube do contrato dele (fora o tipo 5).
/// - Com +0x30 & 0xf e status fora de 0x13: a data de conclusão (0x8cee70: push &data; push oferta, ret 8) vai ao
///   motivo; motivo diferente de 0 dá 2; hoje antes da data dá 0.
/// - Sem `sem_elenco`, alvo com jogador (+0x61) e tipo 11..15 em +0x11 (0x53b620, cdecl do byte): decide o vendedor com
///   +0x2f = 0 e o vendedor não obrigado a aceitar (0x8cd240, stdcall(oferta, 0, 0)), ou decide o comprador: o veto
///   (0x8cdb80: push motivo; push decide; push alvo; push vendedor; push comprador; push oferta, ret 0x18) dá 2.
/// - O orçamento do comprador (0x5a0590: push comprador; mov ecx, [0xae2a38], ret 4; 0x598800: push 0 ×4; push alvo;
///   mov ecx, finanças, ret 0x14, devolve i64; negativo vira 0) ao degrau seguinte (0x43f4e0: push 4; lea ecx, &dinheiro,
///   ret 4) e o valor da oferta (0x8e7a70: mov ecx, oferta -> thiscall(oferta, 0, 1, 1, 0), ret 0x10). Decide o
///   comprador: valor > orçamento × 1,1: motivo 3; comprador humano dá 1; senão o orçamento mais o +0x1b das ofertas do
///   comprador (0x8cdd80: push 0; push comprador; push pessoa, ret 0xc) pelos jogadores da lista de observação dele
///   (0x8420e0: push &lista; push comprador; mov ecx, [this+0x856], ret 8) passando do valor dá 1, senão 2. Dentro do
///   orçamento, sem vaga no elenco (0x8cd450: push motivo; push sem_elenco; push oferta; push clube, ret 0x10) dá 1.
/// - Decide o vendedor: sem vaga no elenco dele dá 1. Motivo 0. Com troca (+4) ou tipo 11..15: a regra da nação do
///   decide ([this+0x8ac], a de +0xa igual a [decide+0x53]): o virtual +0xc (push motivo; push 0xae2c90; push decide;
///   push oferta; mov ecx, regra) diferente de 0 dá 0; motivo 9 com +0x30 & 0xf = 5 dá 0; senão 2. Sem regra: 0.
unsafe extern "thiscall" fn restricoes_da_oferta(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, usize) -> u16 = core::mem::transmute(0x8cee70usize);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x8cd240usize);
    let veto: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32 = core::mem::transmute(0x8cdb80usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let orcamento: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let degrau: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x43f4e0usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(0x8cd450usize);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    let comprador = cqd1_clube(cqd1_d(o + 8));
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let vendedor = cqd1_clube_do_contrato(alvo, true);
    if cqd1_b(o + 0x30) & 0xf != 0 && cqd1_b(o + 0x2c) != 0x13 {
        let r = conclusao(this, o, pd);
        if !motivo.is_null() {
            *motivo = r;
        }
        if r != 0 {
            return 2;
        }
        let (ha, da) = (cqd1_w(0xae2c92) as u16, cqd1_w(pd + 2) as u16);
        if ha < da || (ha == da && cqd1_w(0xae2c90) < cqd1_w(pd)) {
            return 0;
        }
    }
    if sem_elenco == 0 && cqd1_d(alvo + 0x61) != 0 && tipo(cqd1_b(o + 0x11) as i8) != 0 {
        let mut vetar = false;
        if decide == vendedor && cqd1_b(o + 0x2f) == 0 && deve_aceitar(o, 0, 0) == 0 {
            vetar = true;
        } else if decide == comprador {
            vetar = true;
        }
        if vetar && veto(this, o, comprador, vendedor, alvo, decide, motivo) != 0 {
            return 2;
        }
    }
    let mut b = orcamento(financas(cqd1_d(0xae2a38), comprador), alvo, 0, 0, 0, 0);
    if b < 0 {
        b = 0;
    }
    let mut dinheiro = b as u32;
    let v = degrau(&mut dinheiro as *mut u32 as usize, 4) as i64;
    let t = total(o, 0, 1, 1, 0);
    if decide == comprador {
        crate::comparacao::ramo("cqd_1 orçamento");
        if t as f64 > v as f64 * 1.1 { // cqd1: acima do orçamento
            if !motivo.is_null() {
                *motivo = 3;
            }
            if cqd1_humano(comprador) {
                return 1;
            }
            let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let entradas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8420e0usize);
            let oferta_do_clube: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8cdd80usize);
            let mut l = [0u8; 0x10];
            let pl = l.as_mut_ptr() as usize;
            lista_nova(pl);
            entradas(cqd1_d(this + 0x856), comprador, pl);
            cqd1_pd(pl + 0xa, cqd1_d(pl + 2));
            let mut soma = v;
            loop {
                let p = cqd1_proximo(pl);
                if p == 0 {
                    break;
                }
                let x = oferta_do_clube(this, cqd1_pessoa(cqd1_d(p)), comprador, 0);
                if x != 0 {
                    soma = soma.wrapping_add(cqd1_d(x + 0x1b) as i32 as i64);
                }
            }
            destruir(pl);
            return if soma > t as i64 { 1 } else { 2 };
        }
        if vaga(this, comprador, o, sem_elenco, motivo) == 0 {
            return 1;
        }
    }
    if vendedor == decide && vaga(this, vendedor, o, sem_elenco, motivo) == 0 {
        return 1;
    }
    if !motivo.is_null() {
        *motivo = 0;
    }
    if cqd1_d(o + 4) == 0 && tipo(cqd1_b(o + 0x11) as i8) == 0 {
        return 0;
    }
    for k in 0..0x1c {
        let r = cqd1_d(this + 0x8ac + k * 4);
        if cqd1_d(r + 0xa) == cqd1_d(decide + 0x53) {
            if r == 0 {
                return 0;
            }
            let regra: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32 =
                core::mem::transmute(cqd1_d(cqd1_d(r) + 0xc));
            if regra(r, o, decide, 0xae2c90, motivo) != 0 {
                return 0;
            }
            if *motivo == 9 && cqd1_b(o + 0x30) & 0xf == 5 {
                return 0;
            }
            return 2;
        }
    }
    0
}

/// Comparador da 0x8cfc90: só a original (a vaga no elenco, 0x8cd450, pode liberar o jogador mais barato, e a lista de
/// observação aloca nós).
unsafe extern "thiscall" fn comparar_restricoes_da_oferta(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u8>(1548)(this, o, decide, motivo, sem_elenco) // ponytail: só a original
}
