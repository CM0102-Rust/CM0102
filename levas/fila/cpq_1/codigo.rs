// ---------- %%LEVA%%a leva: a diretoria e o técnico: pensar em demitir o técnico, dar o ultimato e tirar o técnico do cargo (manager_manager.cpp) ----------

unsafe fn cpq1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpq1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpq1_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A constante double do exe em `a`.
unsafe fn cpq1_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// A linha da diretoria do clube (73 bytes em [this], pelo id do clube).
unsafe fn club_board_row(this: usize, clube: usize) -> usize {
    cpq1_d(this).wrapping_add(cpq1_d(clube).wrapping_mul(0x49))
}

/// O registro de não jogador do presidente do clube (+0xbf, +0x69), ou 0.
unsafe fn chairman_record(clube: usize) -> usize {
    let p = cpq1_d(clube + 0xbf);
    if p == 0 {
        0
    } else {
        cpq1_d(p + 0x69)
    }
}

unsafe fn cpq1_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// Os dias da data `(d0, d1)` (por valor) até a data do jogo (0x54cdb0).
unsafe fn days_until_today(d0: usize, d1: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    f(0xae2c90, d0, d1)
}

unsafe fn list_add_tail(lista: usize, item: usize, n: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    f(lista, item, n);
}

/// A saída do técnico na lista das saídas [this+4] (6 bytes: `tipo`, o motivo 0x69b350 e o id do clube) e o bit 2 em
/// +2 da linha da diretoria.
unsafe fn record_departure(this: usize, clube: usize, tipo: u8) {
    let id = cpq1_d(clube) as u32;
    let motivo: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x69b350usize);
    let mut reg = [0u8; 8];
    reg[0] = tipo;
    reg[2..6].copy_from_slice(&id.to_le_bytes());
    reg[1] = motivo(this, id);
    list_add_tail(cpq1_d(this + 4), reg.as_mut_ptr() as usize, 6);
    *((club_board_row(this, clube) + 2) as *mut u8) |= 2;
}

/// A nota do técnico atual (0x69d1b0 com 0 e 5; 0 sem técnico ou sem registro de não jogador) contra a melhor nota de
/// candidato (0x69c540): passando de 450 a mais, a confiança (+6 da linha) cai a 90% (mínimo 1) e, com mais de 30 dias
/// desde +0x18 da linha, sai a notícia da pressão (0x6a0e70 com 1). Devolve se passou.
unsafe fn check_better_candidate(this: usize, clube: usize) -> bool {
    let nota_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x69d1b0usize);
    let melhor: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x69c540usize);
    let tec = cpq1_d(clube + 0xcf);
    let nota = if tec != 0 && cpq1_d(tec + 0x69) != 0 { nota_de(this, tec, clube, 0, 5) } else { 0 };
    let m = melhor(this, clube, cpq1_d(clube + 0xcf));
    if nota <= m.wrapping_add(0x1c2) {
        return false;
    }
    let c = club_board_row(this, clube) + 6;
    cpq1_pw(c, match_pl_ftol(cpq1_w(c) as i32 as f64 * cpq1_k(0x9689d8)) as i16); // 0,9
    if cpq1_w(c) < 1 {
        cpq1_pw(c, 1);
    }
    let l = club_board_row(this, clube);
    if days_until_today(cpq1_d(l + 0x18), cpq1_d(l + 0x1c)) > 0x1e {
        let pressure: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a0e70usize);
        pressure(this, clube, 1);
    }
    true
}

/// { 0x69aba0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::consider_sacking_manager(clube, conferir)`:
/// antes da data [0xdd6ec8] (0x456770), com o bit 2 em +2 da linha da diretoria, com o ultimato resolvido (0x6ac470,
/// só com `conferir`), com a confiança (+6) acima de 4275 ou com o técnico no cargo há menos de 150 dias: nada. Técnico
/// bem visto (+0xa do não jogador acima de 5000 e acima da reputação do clube +0x80 − 750) há menos de 400 dias: com
/// sorteio(paciência do presidente +0x1d, ou 10) ≠ 0 a confiança vai a 4275. O limite (10 acima de 3500, 500 até 3000,
/// 200 no meio) muda pela linha do clube na tabela da divisão (0x4a4850): posição (+4) acima de 30 − paciência/2 nas
/// vagas de cima (+0xbf, ou +0xbe pela virtual +0x98 da competição) segura; 5 últimos resultados (+0x14..+0x18) com
/// média acima de 2 seguram; as séries +0x19, +0x41, +0x55 e +0x2d mexem no limite (×1,5 em +0x55). Com o técnico
/// marcado (0x7a7eb0), o limite ×0,25. Sorteio(paciência × 50, ou 500) acima do limite segura; sorteio(paciência + 1,
/// ou 11) = 0 dá o ultimato (0x6ac560; dado, nada mais). Técnico da máquina (0x600a50), sem a marca, com sorteio(+0x5a)
/// e sorteio(4) nulos sai (saída 2). Com presidente: no clube de liga ou seleção, sorteio(paciência) < 5 e sorteio(20)
/// ≠ 0 tenta trocar pelo melhor candidato; senão a notícia da pressão (0x6a0e70 com 2) depois de 14 dias, e só. Sem
/// presidente: sorteio(3) = 0 tenta a troca. Sem troca, a confiança do não jogador cai 250 (mínimo 1500 com presidente,
/// 1 sem) e o técnico é demitido (saída 1).
unsafe extern "thiscall" fn consider_sacking_manager(this: usize, clube: usize, conferir: u32) {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if antes(0xae2c90, cpq1_d(0xdd6ec8) as u32, cpq1_d(0xdd6ecc) as u32) != 0 {
        return;
    }
    if cpq1_b(club_board_row(this, clube) + 2) & 2 != 0 {
        return;
    }
    if conferir as u8 != 0 {
        let resolver: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6ac470usize);
        if resolver(this, clube) != 0 {
            return;
        }
    }
    if cpq1_w(club_board_row(this, clube) + 6) > 0x10b3 {
        return;
    }
    let tec = cpq1_d(clube + 0xcf);
    if days_until_today(cpq1_d(tec + 0x3e), cpq1_d(tec + 0x42)) < 0x96 {
        return;
    }
    let tec = cpq1_d(clube + 0xcf);
    if tec != 0 && cpq1_d(tec + 0x69) != 0 {
        let conf = cpq1_w(cpq1_d(tec + 0x69) + 0xa);
        if conf > 0x1388 && cpq1_w(clube + 0x80).wrapping_sub(0x2ee) < conf
            && days_until_today(cpq1_d(tec + 0x3e), cpq1_d(tec + 0x42)) < 0x190 {
            let pres = chairman_record(clube);
            let n = if pres != 0 { cpq1_b(pres + 0x1d) as i8 as i32 } else { 10 };
            if cpq1_sortear(n) != 0 {
                cpq1_pw(club_board_row(this, clube) + 6, 0x10b3);
                return;
            }
        }
    }
    let v = cpq1_w(club_board_row(this, clube) + 6);
    let mut limit: i32 = if v > 0xdac { 10 } else if v <= 0xbb8 { 500 } else { 200 };
    let div = cpq1_d(clube + 0x57);
    let comp = if div != 0 { cpq1_d(cpq1_d(0xadadfc).wrapping_add(cpq1_d(div).wrapping_mul(4))) } else { 0 };
    if comp != 0 && cpq1_d(comp + 8) != 0 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let ln = busca(cpq1_d(comp + 8), clube);
        if ln != 0 {
            let pres = chairman_record(clube);
            let paciencia: i8 = if pres != 0 { (cpq1_b(pres + 0x1d) as i8) >> 1 } else { 5 };
            let teto = 0x1e - paciencia as i32;
            let comp = cpq1_d(cpq1_d(0xadadfc).wrapping_add(cpq1_d(cpq1_d(clube + 0x57)).wrapping_mul(4)));
            let vagas = cpq1_b(comp + 0xbf) as i8;
            if vagas != 0 && cpq1_b(ln + 0x74) as i32 <= vagas as i32 && cpq1_b(ln + 4) as i32 > teto {
                return;
            }
            let virtual_98: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpq1_d(cpq1_d(comp) + 0x98));
            if virtual_98(comp) != 0 {
                let comp = cpq1_d(cpq1_d(0xadadfc).wrapping_add(cpq1_d(cpq1_d(clube + 0x57)).wrapping_mul(4)));
                if cpq1_b(ln + 0x74) as i32 <= cpq1_b(comp + 0xbe) as i8 as i32 && cpq1_b(ln + 4) as i32 > teto {
                    return;
                }
            }
            let (mut pontos, mut jogos) = (0i32, 0i32);
            for k in (0x14..=0x18usize).rev() {
                match cpq1_b(ln + k) {
                    3 => {
                        pontos += 3;
                        jogos += 1;
                    }
                    1 | 2 => {
                        pontos += 1;
                        jogos += 1;
                    }
                    0xfd => jogos += 1,
                    _ => {}
                }
            }
            if jogos as i16 > 0 && ((pontos as i16 as i32) / (jogos as i16 as i32)) as f64 > cpq1_k(0x968950) {
                return; // média acima de 2
            }
            let s19 = cpq1_i(ln + 0x19);
            if s19 > 1 {
                if limit as i16 == 10 || s19 > 2 {
                    return;
                }
                if limit as i16 == 200 {
                    limit = 0x32;
                }
            } else {
                let s41 = cpq1_i(ln + 0x41);
                if s41 > 1 {
                    if limit as i16 == 10 || s41 > 3 {
                        return;
                    }
                    if limit as i16 == 200 {
                        limit = 100;
                    }
                } else if cpq1_i(ln + 0x55) > 1 {
                    limit = match_pl_ftol(limit as i16 as i32 as f64 * cpq1_k(0x968940)); // 1,5
                } else if cpq1_i(ln + 0x2d) > 1 {
                    if limit as i16 == 10 {
                        limit = 200;
                    } else if limit as i16 == 200 {
                        limit = 300;
                    }
                }
            }
        }
    }
    let marca: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let teto = if marca(cpq1_d(0xb7434c), cpq1_d(clube + 0xcf)) != 0 {
        match_pl_ftol(limit as i16 as i32 as f64 * cpq1_k(0x9689e0)) // 0,25
    } else {
        limit
    };
    let pres = chairman_record(clube);
    let r = if pres != 0 {
        cpq1_sortear(((cpq1_b(pres + 0x1d) as i8 as i32) * 50) as i16 as i32)
    } else {
        cpq1_sortear(500)
    };
    if r > teto as i16 as i32 {
        return;
    }
    let pres = chairman_record(clube);
    let ultimato = if pres != 0 { cpq1_sortear(cpq1_b(pres + 0x1d) as i8 as i32 + 1) == 0 } else { cpq1_sortear(0xb) == 0 };
    if ultimato {
        let dar: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6ac560usize);
        if dar(this, clube) != 0 {
            return;
        }
    }
    if cpq1_d(clube + 0xcf) != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) == 0 && marca(cpq1_d(0xb7434c), cpq1_d(clube + 0xcf)) == 0
            && cpq1_sortear(cpq1_b(cpq1_d(clube + 0xcf) + 0x5a) as i8 as i32) == 0 && cpq1_sortear(4) == 0 {
            crate::comparison::branch("cpq_1 the manager leaves");
            record_departure(this, clube, 2);
            return;
        }
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let tem_presidente = chairman_record(clube) != 0;
    if tem_presidente {
        if (selecao(clube) != 0 || cpq1_d(clube + 0x57) != 0)
            && cpq1_sortear(cpq1_b(chairman_record(clube) + 0x1d) as i8 as i32) < 5 && cpq1_sortear(0x14) != 0 {
            if check_better_candidate(this, clube) {
                return;
            }
        } else {
            let l = club_board_row(this, clube);
            if days_until_today(cpq1_d(l + 0x18), cpq1_d(l + 0x1c)) > 0xe {
                let pressure: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a0e70usize);
                pressure(this, clube, 2);
            }
            return;
        }
    } else if (selecao(clube) != 0 || cpq1_d(clube + 0x57) != 0) && cpq1_sortear(3) == 0 && check_better_candidate(this, clube) {
        return;
    }
    let tec = cpq1_d(clube + 0xcf);
    if tec != 0 && cpq1_d(tec + 0x69) != 0 {
        let nj = cpq1_d(tec + 0x69);
        cpq1_pw(nj + 0xa, cpq1_w(nj + 0xa).wrapping_sub(250));
        let piso: i16 = if tem_presidente { 0x5dc } else { 1 };
        if cpq1_w(nj + 0xa) < piso {
            cpq1_pw(nj + 0xa, piso);
        }
    }
    crate::comparison::branch("cpq_1 sacked");
    record_departure(this, clube, 1);
}

// ponytail: sorteia e, conforme o sorteio, aloca (a saída na lista, o ultimato, a notícia): o caminho só se sabe
// rodando. Só a original; a cópia leva os ramos e a carreira a prova (as demissões mudam tudo).
unsafe extern "thiscall" fn compare_consider_sacking_manager(this: usize, clube: usize, conferir: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, clube, conferir)
}

/// O ultimato com a lista local (já construída): o primeiro jogo do clube (0x5acbe0 do gerenciador [0xae2a58]: de hoje
/// a 8 semanas, 0x54c770, qualquer competição, tipo −2, lado 1) que não é hoje, com +0x40 e +0x43 = 0xff, com
/// competição (+0x14): a notícia (0x6ac700 com o adversário, +0x1c ou +0x20) e o id do clube na lista dos ultimatos
/// [this+0x10]; devolve 1.
unsafe fn issue_ultimatum(this: usize, clube: usize, l: usize) -> u8 {
    if clube == 0 || cpq1_d(clube + 0xcf) == 0 || cpq1_d(clube + 0x57) == 0 {
        return 0;
    }
    let nacao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
    nacao(clube); // a original descarta o valor
    let semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c770usize);
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let p1 = semanas(0xae2c90, d1.as_mut_ptr(), 8);
    let ano_fim = cpq1_w(p1 + 2) as u16 as u32;
    let p2 = semanas(0xae2c90, d2.as_mut_ptr(), 8);
    let dia_fim = cpq1_w(p2) as u16 as u32;
    // ponytail: a original empurra as palavras com a metade alta do registro: no dia de hoje e no dia do fim a do
    // ponteiro da segunda data, no ano de hoje a do ponteiro da lista (reproduzidas pelos daqui); no ano do fim a do
    // edi de quem chamou, que a 0x5acbe0 descarta (lê só as palavras do fim).
    let alto = |p: usize| (p as u32) & 0xffff_0000;
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, i32) -> u32 =
        core::mem::transmute(0x5acbe0usize);
    achar(cpq1_d(0xae2a58), alto(p2) | cpq1_w(0xae2c90) as u16 as u32, alto(l) | cpq1_w(0xae2c92) as u16 as u32,
          alto(p2) | dia_fim, ano_fim, cpq1_d(clube) as u32, l, -1, -2, 1);
    if cpq1_w(l) == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpq1_d(l + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(l);
    while e != 0 {
        let hoje = cpq1_w(e + 0x2a) as u16 as i32 == cpq1_w(0xae2c90) as i32 && cpq1_w(e + 0x28) == cpq1_w(0xae2c92);
        if !hoje && cpq1_b(e + 0x40) != 0 && cpq1_b(e + 0x43) == 0xff {
            break;
        }
        e = proximo(l);
    }
    if e == 0 || cpq1_d(e + 0x14) == 0 {
        return 0;
    }
    let adversario = if cpq1_d(e + 0x1c) == clube { cpq1_d(e + 0x20) } else { cpq1_d(e + 0x1c) };
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x6ac700usize);
    noticia(this, clube, adversario, cpq1_d(e + 0x14));
    crate::comparison::branch("cpq_1 ultimatum given");
    let id = cpq1_d(clube) as u32;
    list_add_tail(cpq1_d(this + 0x10), &id as *const u32 as usize, 4);
    1
}

/// { 0x6ac560 } (thiscall, ret 4, manager_manager.cpp, SEH da lista local): `MANAGER_MANAGER::issue_board_ultimatum(clube)`:
/// clube com técnico (+0xcf) e divisão (+0x57): a lista local (LLIST 0x68d020, solta pela 0x68d040) recebe os jogos
/// das próximas 8 semanas; o primeiro que vale dá a notícia do ultimato e põe o clube na lista dos ultimatos
/// [this+0x10]: devolve 1. Senão 0.
unsafe extern "thiscall" fn issue_board_ultimatum(this: usize, clube: usize) -> u8 {
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    nova(l);
    let r = issue_ultimatum(this, clube, l);
    soltar(l);
    r
}

// ponytail: aloca a lista dos jogos, a notícia e o item da lista dos ultimatos: só a original, conferida pelo plano:
// a lista dos ultimatos [this+0x10] cresce de 1 quando devolve 1 e fica igual com 0.
unsafe extern "thiscall" fn compare_issue_board_ultimatum(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%);
    if cpq1_d(this + 0x10) == 0 {
        return o(this, clube);
    }
    let antes = cpq1_w(cpq1_d(this + 0x10)) as i32;
    let r = o(this, clube);
    let depois = cpq1_w(cpq1_d(this + 0x10)) as i32;
    SCOREBOARD.record(depois == antes + (r != 0) as i32, || {
        format!("issue_board_ultimatum(clube {:x}) devolveu {}, ultimatos {} -> {}", clube, r, antes, depois)
    });
    r
}

/// As marcas do contrato que a saída do técnico apaga: lidas pela 0x8676f0 (+0x40; a 0x10000000 é apagada pela
/// 0x8674e0, que mexe no +0x45) e lidas no +0x45 (apagadas pela 0x8674e0, que também atualiza a pessoa).
const CONTRACT_FLAGS_BY_READER: [(u32, usize); 3] = [(0x1000_0000, 0x8674e0), (0x80, 0x8676d0), (0x100, 0x8676d0)];
const CONTRACT_FLAGS_AT_45: [u32; 8] = [0x800_0000, 8, 0x200, 0x400, 0x2000, 0x1_0000, 0x8_0000, 0x80_0000];

/// O técnico do clube no cargo (pelo +0x39 do clube contratado, cargo +0x3d; ou pelo +0x24 da seleção, cargo +0x28):
/// (técnico, cargo).
unsafe fn manager_and_role(clube: usize) -> Option<(usize, u8)> {
    let tec = cpq1_d(clube + 0xcf);
    if tec == 0 {
        return None;
    }
    if cpq1_d(tec + 0x39) == clube {
        Some((tec, cpq1_b(tec + 0x3d)))
    } else if cpq1_d(tec + 0x24) == clube {
        Some((tec, cpq1_b(tec + 0x28)))
    } else {
        None
    }
}

/// { 0x69b790 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::remove_club_manager(clube, tipo, motivo,
/// manter)`: com o técnico no cargo: no clube humano (0x600a50) as respostas automáticas (0x7874b0); a data da saída
/// (+0x29 na seleção, +0x3e no clube) vira a de hoje; com cargo de técnico (12, 13 ou 15) sai do elenco (0x690730);
/// sai da comissão (0x690890); fora da seleção perde o contrato (0x4d0b50); fora do tipo 3 a notícia da saída (0x6a1b00
/// com 2 para os cargos de técnico, 3 para os outros); sem `manter` a confiança (+6 da linha) vai a 3500. O registro
/// da troca (0x869aa0, com 0 no tipo 2 e 2 nos outros), as multas do clube (0x5a5960) e, fora da seleção, as queixas e
/// marcas de cada jogador do elenco com contrato (0x4cf5a0) saem.
unsafe extern "thiscall" fn remove_club_manager(this: usize, clube: usize, tipo: u32, motivo: u32, manter: u32) {
    let (tec, cargo) = match manager_and_role(clube) {
        Some(t) => t,
        None => return,
    };
    crate::comparison::branch("cpq_1 remove the manager");
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        let respond: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7874b0usize);
        respond(cpq1_d(0xb7431c), tec, clube);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(if selecao(clube) != 0 { tec + 0x29 } else { tec + 0x3e }, t.as_mut_ptr(), 0xae2c90);
    let c = cargo as i8;
    let de_tecnico = c >= 0xc && (c <= 0xd || c == 0xf);
    let sair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x690890usize);
    if de_tecnico {
        let do_elenco: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x690730usize);
        do_elenco(this, tec, clube);
    }
    sair(this, tec, clube);
    if selecao(clube) == 0 {
        let sem_contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4d0b50usize);
        sem_contrato(cpq1_d(0xae1050), tec);
    }
    if tipo as u8 != 3 {
        let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) = core::mem::transmute(0x6a1b00usize);
        noticia(this, tec, tipo, motivo, clube, if de_tecnico { 2 } else { 3 });
    }
    if manter == 0 {
        cpq1_pw(club_board_row(this, clube) + 6, 0xdac);
    }
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869aa0usize);
    registrar(cpq1_d(0xdd7f00), tec, clube, if tipo as u8 == 2 { 0 } else { 2 });
    if cpq1_d(0xae2a3c) != 0 {
        let multas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5a5960usize);
        multas(cpq1_d(0xae2a3c), clube);
    }
    if selecao(clube) != 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
    let queixa: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
    for k in 0..50usize {
        let p = cpq1_d(clube + 0xd7 + 4 * k);
        if p == 0 || cpq1_d(p + 0x61) == 0 {
            continue;
        }
        let ct = contrato(cpq1_d(0xae1050), p);
        if ct == 0 {
            continue;
        }
        for (m, f) in CONTRACT_FLAGS_BY_READER {
            if marcas(ct) & m != 0 {
                let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                apagar(ct, m);
            }
        }
        for m in CONTRACT_FLAGS_AT_45 {
            if cpq1_d(ct + 0x45) as u32 & m != 0 {
                queixa(ct, m);
            }
        }
    }
}

// ponytail: tira o contrato, solta a notícia e grava o histórico em arquivo (não roda duas vezes): só a original,
// conferida depois: sem `manter` a confiança em 3500 e, fora da seleção, nenhum contrato do elenco com as marcas.
unsafe extern "thiscall" fn compare_remove_club_manager(this: usize, clube: usize, tipo: u32, motivo: u32, manter: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I2%%);
    let tira = manager_and_role(clube).is_some();
    o(this, clube, tipo, motivo, manter);
    if !tira {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
    let mut sobra = 0u32;
    if selecao(clube) == 0 {
        for k in 0..50usize {
            let p = cpq1_d(clube + 0xd7 + 4 * k);
            if p == 0 || cpq1_d(p + 0x61) == 0 {
                continue;
            }
            let ct = contrato(cpq1_d(0xae1050), p);
            if ct != 0 {
                // a 0x10000000 é lida em +0x40 (0x8676f0) e apagada em +0x45 (0x8674e0): não dá para conferir
                sobra |= (marcas(ct) & 0x180) | (cpq1_d(ct + 0x45) as u32 & 0x889_2608);
            }
        }
    }
    let conf = cpq1_w(club_board_row(this, clube) + 6);
    SCOREBOARD.record(sobra == 0 && (manter != 0 || conf == 0xdac), || {
        format!("remove_club_manager(clube {:x}, tipo {}, manter {}) marcas que sobraram {:#x}, confiança {}", clube, tipo as u8, manter, sobra, conf)
    });
}
