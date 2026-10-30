// ---------- 253a leva: a diretoria e o técnico: pensar em demitir o técnico, dar o ultimato e tirar o técnico do cargo (manager_manager.cpp) ----------

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
unsafe fn cpq1_linha(this: usize, clube: usize) -> usize {
    cpq1_d(this).wrapping_add(cpq1_d(clube).wrapping_mul(0x49))
}

/// O registro de não jogador do presidente do clube (+0xbf, +0x69), ou 0.
unsafe fn cpq1_presidente(clube: usize) -> usize {
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
unsafe fn cpq1_dias(d0: usize, d1: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    f(0xae2c90, d0, d1)
}

unsafe fn cpq1_acrescentar(lista: usize, item: usize, n: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    f(lista, item, n);
}

/// A saída do técnico na lista das saídas [this+4] (6 bytes: `tipo`, o motivo 0x69b350 e o id do clube) e o bit 2 em
/// +2 da linha da diretoria.
unsafe fn cpq1_saida(this: usize, clube: usize, tipo: u8) {
    let id = cpq1_d(clube) as u32;
    let motivo: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x69b350usize);
    let mut reg = [0u8; 8];
    reg[0] = tipo;
    reg[2..6].copy_from_slice(&id.to_le_bytes());
    reg[1] = motivo(this, id);
    cpq1_acrescentar(cpq1_d(this + 4), reg.as_mut_ptr() as usize, 6);
    *((cpq1_linha(this, clube) + 2) as *mut u8) |= 2;
}

/// A nota do técnico atual (0x69d1b0 com 0 e 5; 0 sem técnico ou sem registro de não jogador) contra a melhor nota de
/// candidato (0x69c540): passando de 450 a mais, a confiança (+6 da linha) cai a 90% (mínimo 1) e, com mais de 30 dias
/// desde +0x18 da linha, sai a notícia da pressão (0x6a0e70 com 1). Devolve se passou.
unsafe fn cpq1_trocar_pelo_melhor(this: usize, clube: usize) -> bool {
    let nota_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x69d1b0usize);
    let melhor: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x69c540usize);
    let tec = cpq1_d(clube + 0xcf);
    let nota = if tec != 0 && cpq1_d(tec + 0x69) != 0 { nota_de(this, tec, clube, 0, 5) } else { 0 };
    let m = melhor(this, clube, cpq1_d(clube + 0xcf));
    if nota <= m.wrapping_add(0x1c2) {
        return false;
    }
    let c = cpq1_linha(this, clube) + 6;
    cpq1_pw(c, ftol(cpq1_w(c) as i32 as f64 * cpq1_k(0x9689d8)) as i16); // 0,9
    if cpq1_w(c) < 1 {
        cpq1_pw(c, 1);
    }
    let l = cpq1_linha(this, clube);
    if cpq1_dias(cpq1_d(l + 0x18), cpq1_d(l + 0x1c)) > 0x1e {
        let pressao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a0e70usize);
        pressao(this, clube, 1);
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
unsafe extern "thiscall" fn pensar_em_demitir_o_tecnico(this: usize, clube: usize, conferir: u32) {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if antes(0xae2c90, cpq1_d(0xdd6ec8) as u32, cpq1_d(0xdd6ecc) as u32) != 0 {
        return;
    }
    if cpq1_b(cpq1_linha(this, clube) + 2) & 2 != 0 {
        return;
    }
    if conferir as u8 != 0 {
        let resolver: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6ac470usize);
        if resolver(this, clube) != 0 {
            return;
        }
    }
    if cpq1_w(cpq1_linha(this, clube) + 6) > 0x10b3 {
        return;
    }
    let tec = cpq1_d(clube + 0xcf);
    if cpq1_dias(cpq1_d(tec + 0x3e), cpq1_d(tec + 0x42)) < 0x96 {
        return;
    }
    let tec = cpq1_d(clube + 0xcf);
    if tec != 0 && cpq1_d(tec + 0x69) != 0 {
        let conf = cpq1_w(cpq1_d(tec + 0x69) + 0xa);
        if conf > 0x1388 && cpq1_w(clube + 0x80).wrapping_sub(0x2ee) < conf
            && cpq1_dias(cpq1_d(tec + 0x3e), cpq1_d(tec + 0x42)) < 0x190 {
            let pres = cpq1_presidente(clube);
            let n = if pres != 0 { cpq1_b(pres + 0x1d) as i8 as i32 } else { 10 };
            if cpq1_sortear(n) != 0 {
                cpq1_pw(cpq1_linha(this, clube) + 6, 0x10b3);
                return;
            }
        }
    }
    let v = cpq1_w(cpq1_linha(this, clube) + 6);
    let mut limite: i32 = if v > 0xdac { 10 } else if v <= 0xbb8 { 500 } else { 200 };
    let div = cpq1_d(clube + 0x57);
    let comp = if div != 0 { cpq1_d(cpq1_d(0xadadfc).wrapping_add(cpq1_d(div).wrapping_mul(4))) } else { 0 };
    if comp != 0 && cpq1_d(comp + 8) != 0 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let ln = busca(cpq1_d(comp + 8), clube);
        if ln != 0 {
            let pres = cpq1_presidente(clube);
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
                if limite as i16 == 10 || s19 > 2 {
                    return;
                }
                if limite as i16 == 200 {
                    limite = 0x32;
                }
            } else {
                let s41 = cpq1_i(ln + 0x41);
                if s41 > 1 {
                    if limite as i16 == 10 || s41 > 3 {
                        return;
                    }
                    if limite as i16 == 200 {
                        limite = 100;
                    }
                } else if cpq1_i(ln + 0x55) > 1 {
                    limite = ftol(limite as i16 as i32 as f64 * cpq1_k(0x968940)); // 1,5
                } else if cpq1_i(ln + 0x2d) > 1 {
                    if limite as i16 == 10 {
                        limite = 200;
                    } else if limite as i16 == 200 {
                        limite = 300;
                    }
                }
            }
        }
    }
    let marca: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let teto = if marca(cpq1_d(0xb7434c), cpq1_d(clube + 0xcf)) != 0 {
        ftol(limite as i16 as i32 as f64 * cpq1_k(0x9689e0)) // 0,25
    } else {
        limite
    };
    let pres = cpq1_presidente(clube);
    let r = if pres != 0 {
        cpq1_sortear(((cpq1_b(pres + 0x1d) as i8 as i32) * 50) as i16 as i32)
    } else {
        cpq1_sortear(500)
    };
    if r > teto as i16 as i32 {
        return;
    }
    let pres = cpq1_presidente(clube);
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
            crate::comparacao::ramo("cpq_1 o tecnico sai");
            cpq1_saida(this, clube, 2);
            return;
        }
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let tem_presidente = cpq1_presidente(clube) != 0;
    if tem_presidente {
        if (selecao(clube) != 0 || cpq1_d(clube + 0x57) != 0)
            && cpq1_sortear(cpq1_b(cpq1_presidente(clube) + 0x1d) as i8 as i32) < 5 && cpq1_sortear(0x14) != 0 {
            if cpq1_trocar_pelo_melhor(this, clube) {
                return;
            }
        } else {
            let l = cpq1_linha(this, clube);
            if cpq1_dias(cpq1_d(l + 0x18), cpq1_d(l + 0x1c)) > 0xe {
                let pressao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a0e70usize);
                pressao(this, clube, 2);
            }
            return;
        }
    } else if (selecao(clube) != 0 || cpq1_d(clube + 0x57) != 0) && cpq1_sortear(3) == 0 && cpq1_trocar_pelo_melhor(this, clube) {
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
    crate::comparacao::ramo("cpq_1 demitido");
    cpq1_saida(this, clube, 1);
}

// ponytail: sorteia e, conforme o sorteio, aloca (a saída na lista, o ultimato, a notícia): o caminho só se sabe
// rodando. Só a original; a cópia leva os ramos e a carreira a prova (as demissões mudam tudo).
unsafe extern "thiscall" fn comparar_pensar_em_demitir_o_tecnico(this: usize, clube: usize, conferir: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1045)(this, clube, conferir)
}

/// O ultimato com a lista local (já construída): o primeiro jogo do clube (0x5acbe0 do gerenciador [0xae2a58]: de hoje
/// a 8 semanas, 0x54c770, qualquer competição, tipo −2, lado 1) que não é hoje, com +0x40 e +0x43 = 0xff, com
/// competição (+0x14): a notícia (0x6ac700 com o adversário, +0x1c ou +0x20) e o id do clube na lista dos ultimatos
/// [this+0x10]; devolve 1.
unsafe fn cpq1_ultimato(this: usize, clube: usize, l: usize) -> u8 {
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
    crate::comparacao::ramo("cpq_1 ultimato dado");
    let id = cpq1_d(clube) as u32;
    cpq1_acrescentar(cpq1_d(this + 0x10), &id as *const u32 as usize, 4);
    1
}

/// { 0x6ac560 } (thiscall, ret 4, manager_manager.cpp, SEH da lista local): `MANAGER_MANAGER::issue_board_ultimatum(clube)`:
/// clube com técnico (+0xcf) e divisão (+0x57): a lista local (LLIST 0x68d020, solta pela 0x68d040) recebe os jogos
/// das próximas 8 semanas; o primeiro que vale dá a notícia do ultimato e põe o clube na lista dos ultimatos
/// [this+0x10]: devolve 1. Senão 0.
unsafe extern "thiscall" fn dar_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    nova(l);
    let r = cpq1_ultimato(this, clube, l);
    soltar(l);
    r
}

// ponytail: aloca a lista dos jogos, a notícia e o item da lista dos ultimatos: só a original, conferida pelo plano:
// a lista dos ultimatos [this+0x10] cresce de 1 quando devolve 1 e fica igual com 0.
unsafe extern "thiscall" fn comparar_dar_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1046);
    if cpq1_d(this + 0x10) == 0 {
        return o(this, clube);
    }
    let antes = cpq1_w(cpq1_d(this + 0x10)) as i32;
    let r = o(this, clube);
    let depois = cpq1_w(cpq1_d(this + 0x10)) as i32;
    PLACAR.registrar(depois == antes + (r != 0) as i32, || {
        format!("dar_o_ultimato_da_diretoria(clube {:x}) devolveu {}, ultimatos {} -> {}", clube, r, antes, depois)
    });
    r
}

/// As marcas do contrato que a saída do técnico apaga: lidas pela 0x8676f0 (+0x40; a 0x10000000 é apagada pela
/// 0x8674e0, que mexe no +0x45) e lidas no +0x45 (apagadas pela 0x8674e0, que também atualiza a pessoa).
const CPQ1_PELA_LEITURA: [(u32, usize); 3] = [(0x1000_0000, 0x8674e0), (0x80, 0x8676d0), (0x100, 0x8676d0)];
const CPQ1_NO_45: [u32; 8] = [0x800_0000, 8, 0x200, 0x400, 0x2000, 0x1_0000, 0x8_0000, 0x80_0000];

/// O técnico do clube no cargo (pelo +0x39 do clube contratado, cargo +0x3d; ou pelo +0x24 da seleção, cargo +0x28):
/// (técnico, cargo).
unsafe fn cpq1_tecnico_no_cargo(clube: usize) -> Option<(usize, u8)> {
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
unsafe extern "thiscall" fn tirar_o_tecnico_do_cargo(this: usize, clube: usize, tipo: u32, motivo: u32, manter: u32) {
    let (tec, cargo) = match cpq1_tecnico_no_cargo(clube) {
        Some(t) => t,
        None => return,
    };
    crate::comparacao::ramo("cpq_1 tirar o tecnico");
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7874b0usize);
        responder(cpq1_d(0xb7431c), tec, clube);
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
        cpq1_pw(cpq1_linha(this, clube) + 6, 0xdac);
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
        for (m, f) in CPQ1_PELA_LEITURA {
            if marcas(ct) & m != 0 {
                let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                apagar(ct, m);
            }
        }
        for m in CPQ1_NO_45 {
            if cpq1_d(ct + 0x45) as u32 & m != 0 {
                queixa(ct, m);
            }
        }
    }
}

// ponytail: tira o contrato, solta a notícia e grava o histórico em arquivo (não roda duas vezes): só a original,
// conferida depois: sem `manter` a confiança em 3500 e, fora da seleção, nenhum contrato do elenco com as marcas.
unsafe extern "thiscall" fn comparar_tirar_o_tecnico_do_cargo(this: usize, clube: usize, tipo: u32, motivo: u32, manter: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1047);
    let tira = cpq1_tecnico_no_cargo(clube).is_some();
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
    let conf = cpq1_w(cpq1_linha(this, clube) + 6);
    PLACAR.registrar(sobra == 0 && (manter != 0 || conf == 0xdac), || {
        format!("tirar_o_tecnico_do_cargo(clube {:x}, tipo {}, manter {}) marcas que sobraram {:#x}, confiança {}", clube, tipo as u8, manter, sobra, conf)
    });
}

// ---------- 253a leva: a federação e a diretoria: avaliar a seleção na competição, o clube em apuro financeiro e a faixa da confiança da diretoria (manager_manager.cpp) ----------

unsafe fn cpq2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq2_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A linha da diretoria (ou da federação) do clube: 73 bytes em [this], pelo id.
unsafe fn cpq2_linha(this: usize, clube: usize) -> usize {
    cpq2_d(this).wrapping_add(cpq2_d(clube).wrapping_mul(0x49))
}

unsafe fn cpq2_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A confiança `c` (+6 da linha) somada de `v` (16 bits), presa a `[piso, teto]` (primeiro o teto).
unsafe fn cpq2_somar(c: usize, v: i16, piso: i16, teto: i16) {
    cpq2_pw(c, cpq2_w(c).wrapping_add(v));
    if cpq2_w(c) > teto {
        cpq2_pw(c, teto);
    } else if cpq2_w(c) < piso {
        cpq2_pw(c, piso);
    }
}

/// A saída do técnico da seleção na lista das saídas [this+4] (6 bytes: 1, `motivo`, o id da seleção).
unsafe fn cpq2_saida(this: usize, selecao: usize, motivo: u8) {
    let mut reg = [0u8; 8];
    reg[0] = 1;
    reg[1] = motivo;
    reg[2..6].copy_from_slice(&(cpq2_d(selecao) as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    acrescentar(cpq2_d(this + 4), reg.as_mut_ptr() as usize, 6);
}

/// A federação pensa em demitir (0x69aba0 sem conferir o ultimato).
unsafe fn cpq2_pensar(this: usize, selecao: usize) {
    crate::comparacao::ramo("cpq_2 a federacao pensa");
    let pensar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x69aba0usize);
    pensar(this, selecao, 0);
}

/// { 0x6a5610 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::review_national_team_performance(comp,
/// selecao)`: seleção com técnico (+0xcf) e competição em [0xadadfc]: com o técnico há menos de 365 dias, a confiança
/// da federação (+6 da linha) sobe a 6500. A nota do técnico na competição (virtual +0x74 dela) muda a confiança: 3:
/// +1250 (teto 10000, piso 8500); 2: +750 (teto 10000, piso 6500), e a seleção da máquina com o técnico
/// há mais de 3650 dias sai se sorteio(15000) + 3750 < reputação (+0x80); 0: −250 e, só com +0xe da linha = −3, 3500 e a
/// saída (mais de 3650 dias) ou, com reputação acima de 6750 e mais de 1460 dias, sorteio(15000) contra a reputação
/// (abaixo: saída; senão a federação pensa, 0x69aba0); −1: −750 (1..3500), saída depois de 1460 dias, sorteio(20000)
/// depois de 730; −2: −1250 (1..3000), saída depois de 730 dias, senão sorteio(15000) + 1250; −3: −1750 (1..3000) e a
/// saída. A avaliação vai para a notícia (0x6a5a60) logo depois da mudança. As saídas vão com motivo 6 (2, 0 e −1 com
/// mais tempo) ou 0.
unsafe extern "thiscall" fn avaliar_a_selecao_na_competicao(this: usize, comp: usize, selecao: usize) {
    let tec = cpq2_d(selecao + 0xcf);
    let objeto = || cpq2_d(cpq2_d(0xadadfc).wrapping_add(cpq2_d(comp).wrapping_mul(4)));
    if tec == 0 || objeto() == 0 {
        return;
    }
    let dias_de: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let dias = dias_de(0xae2c90, cpq2_d(tec + 0x3e), cpq2_d(tec + 0x42));
    if dias < 0x16d && cpq2_w(cpq2_linha(this, selecao) + 6) < 0x1964 {
        cpq2_pw(cpq2_linha(this, selecao) + 6, 0x1964); // técnico novo
    }
    let o = objeto();
    let nota_de: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cpq2_d(cpq2_d(o) + 0x74));
    let nota = nota_de(o, selecao) as u8 as i8;
    crate::comparacao::ramo("cpq_2 selecao avaliada");
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x6a5a60usize);
    let c = cpq2_linha(this, selecao) + 6;
    let reputacao = || cpq2_w(selecao + 0x80);
    match nota {
        3 => {
            cpq2_somar(c, 0x4e2, 0x2134, 0x2710);
            avaliar(this, comp, selecao, 3);
        }
        2 => {
            cpq2_somar(c, 0x2ee, 0x1964, 0x2710);
            avaliar(this, comp, selecao, 2);
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if humano(selecao, 0, 1, 0, 0) != 0 || dias <= 0xe42 {
                return;
            }
            if (cpq2_sortear(0x3a98) + 0xea6) as i16 >= reputacao() {
                return;
            }
            cpq2_saida(this, selecao, 6);
        }
        0 => {
            cpq2_pw(c, cpq2_w(c).wrapping_sub(250));
            if cpq2_w(cpq2_linha(this, selecao) + 0xe) != -3 {
                return;
            }
            cpq2_pw(c, 0xdac);
            avaliar(this, comp, selecao, 0);
            if dias > 0xe42 {
                return cpq2_saida(this, selecao, 6);
            }
            if reputacao() <= 0x1a5e || dias <= 0x5b4 {
                return;
            }
            if cpq2_sortear(0x3a98) >= reputacao() as i32 {
                return cpq2_pensar(this, selecao);
            }
            cpq2_saida(this, selecao, 6);
        }
        -1 => {
            cpq2_somar(c, -750, 1, 0xdac);
            avaliar(this, comp, selecao, -1);
            if dias > 0x5b4 {
                return cpq2_saida(this, selecao, 6);
            }
            if dias <= 0x2da {
                return;
            }
            if cpq2_sortear(0x4e20) >= reputacao() as i32 {
                return cpq2_pensar(this, selecao);
            }
            cpq2_saida(this, selecao, 0);
        }
        -2 => {
            cpq2_somar(c, -1250, 1, 0xbb8);
            avaliar(this, comp, selecao, -2);
            if dias <= 0x2da && cpq2_sortear(0x3a98) + 0x4e2 >= reputacao() as i32 {
                return cpq2_pensar(this, selecao);
            }
            cpq2_saida(this, selecao, 0);
        }
        -3 => {
            cpq2_somar(c, -1750, 1, 0xbb8);
            avaliar(this, comp, selecao, -3);
            cpq2_saida(this, selecao, 0);
        }
        _ => {}
    }
}

// ponytail: solta a notícia da avaliação (0x6a5a60), sorteia e aloca a saída (ou chama a 0x69aba0): só a original;
// a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_avaliar_a_selecao_na_competicao(this: usize, comp: usize, selecao: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1048)(this, comp, selecao)
}

/// { 0x69eb50 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::is_club_in_financial_trouble(clube)`: clube
/// dirigível (0x544820), fora da seleção (0x53b900), com a confiança da diretoria em +8 da linha abaixo de 3000 e o saldo
/// (64 bits no registro de finanças, 0x5a0590 de [0xae2a38]) negativo: 1. Senão 0. Pelo motor (cantos da torcida).
unsafe extern "thiscall" fn clube_em_apuro_financeiro(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let dirigivel: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544820usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if dirigivel(clube) as u8 == 0 || selecao(clube) != 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let f = financas(cpq2_d(0xae2a38), clube);
    if cpq2_w(cpq2_linha(this, clube) + 8) >= 0xbb8 || f == 0 {
        return 0;
    }
    (core::ptr::read_unaligned(f as *const i64) < 0) as u8
}

unsafe extern "thiscall" fn comparar_clube_em_apuro_financeiro(this: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1049)(this, clube);
    let n = clube_em_apuro_financeiro(this, clube);
    PLACAR.registrar(r == n, || format!("clube_em_apuro_financeiro(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x690b70 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_board_confidence_band(clube)`: a faixa
/// (0..6) da confiança da diretoria (+6 da linha) pelos limites 3000, 3500, 4275, 6500, 8500 e 9500. Pelo texto do
/// relatório da diretoria (0x690d80).
unsafe extern "thiscall" fn faixa_da_confianca_da_diretoria(this: usize, clube: usize) -> u8 {
    let v = cpq2_w(cpq2_linha(this, clube) + 6);
    [0xbb8i16, 0xdac, 0x10b3, 0x1964, 0x2134, 0x251c].iter().take_while(|&&l| v >= l).count() as u8
}

unsafe extern "thiscall" fn comparar_faixa_da_confianca_da_diretoria(this: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1050)(this, clube);
    let n = faixa_da_confianca_da_diretoria(this, clube);
    PLACAR.registrar(r == n, || format!("faixa_da_confianca_da_diretoria(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 253a leva: os registros de carreira: o acesso, o rebaixamento e a classificação para a fase seguinte (staff_records.cpp) ----------

unsafe fn cpq3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpq3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpq3_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpq3_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpq3_somar(a: usize, v: i32) {
    cpq3_pi(a, cpq3_i(a).wrapping_add(v));
}

/// A constante double do exe em `a`.
unsafe fn cpq3_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Uma entrada de histórico na pilha: a data (0x54a110) com a data do jogo copiada (0x417c10).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
unsafe fn cpq3_entrada() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    nova(e.as_mut_ptr());
    copia(e.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    e
}

unsafe fn cpq3_por(e: &mut [u8; 0x40], o: usize, v: u32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// A entrada `e` no fim da lista do arquivo `arq` (FILE_LLIST, 0x58e340) da pessoa ou do clube `id` (o começo e o fim
/// em `base` + id × 8). Sem `gravar` (a cópia pelo comparador, depois da original), nada.
unsafe fn cpq3_historico(gravar: bool, arq: usize, base: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
    let p = base.wrapping_add(id.wrapping_mul(8));
    f(arq, p, p + 4, e.as_ptr());
}

/// O histórico da pessoa (o arquivo [this], as listas em [this+0xc]).
unsafe fn cpq3_da_pessoa(gravar: bool, this: usize, pessoa: usize, e: &[u8; 0x40]) {
    cpq3_historico(gravar, cpq3_d(this), cpq3_d(this + 0xc), cpq3_d(pessoa), e);
}

/// O histórico do clube (o arquivo [this+4], as listas em [this+0x10]) com o tipo e a competição.
unsafe fn cpq3_do_clube(gravar: bool, this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = cpq3_entrada();
    e[8] = tipo;
    cpq3_por(&mut e, 9, cpq3_d(comp) as u32);
    cpq3_historico(gravar, cpq3_d(this + 4), cpq3_d(this + 0x10), cpq3_d(clube), &e);
}

/// A entrada da pessoa: o clube (+8), o tipo (+0xc) e a competição (+0xd).
unsafe fn cpq3_entrada_do_clube(clube: usize, tipo: u8, comp: u32) -> [u8; 0x40] {
    let mut e = cpq3_entrada();
    cpq3_por(&mut e, 8, cpq3_d(clube) as u32);
    e[0xc] = tipo;
    cpq3_por(&mut e, 0xd, comp);
    e
}

/// Os jogadores do elenco (50 vagas em +0xd7) que contam nos registros: com registro de jogador (+0x61), id abaixo de
/// [0xae2364] e que não são o técnico.
unsafe fn cpq3_elenco(clube: usize) -> impl Iterator<Item = usize> {
    (0..50usize).filter_map(move |k| {
        let p = cpq3_d(clube + 0xd7 + 4 * k);
        if p == 0 || cpq3_d(p + 0x61) == 0 || cpq3_i(p) >= cpq3_i(0xae2364) || p == cpq3_d(clube + 0xcf) {
            None
        } else {
            Some(p)
        }
    })
}

unsafe fn cpq3_contrato(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cpq3_d(0xae1050), p, clube)
}

/// O contrato `c` do jogador `p` vai a renovação (a 0x4cda70 roda): com o gerente de transferências, o contrato com
/// mais de 300 dias (desde +0x25) e o jogador na lista do clube (0x849ce0, item +0xf = id dele). Só leitura.
unsafe fn cpq3_vai_renovar(clube: usize, p: usize, c: usize) -> bool {
    let gt = cpq3_d(0xde1f54);
    if gt == 0 {
        return false;
    }
    let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    if dias(0xae2c90, cpq3_d(c + 0x25), cpq3_d(c + 0x29)) <= 0x12c {
        return false;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let e = entrada(cpq3_d(gt + 0x856), clube, p, 0, 0);
    e != 0 && cpq3_d(e + 0xf) == cpq3_d(p)
}

/// A oferta de renovação do contrato (0x4cda70 do gerente de contratos: o jogador, sem clube nem termos, vaga −1,
/// fator 1,0, modo 1).
unsafe fn cpq3_renovar(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    f(cpq3_d(0xae1050), p, 0, 0, -1, 0, 0x3ff0_0000, 1);
}

unsafe fn cpq3_moral(p: usize, delta: i32) {
    let f: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(0x544710usize);
    f(p, delta, 1);
}

/// A nota do não jogador em `a` sobe ftol(x × peso × 10/61) e fica no máximo entre x × peso (16 bits) e ela mesma,
/// presa a 10000.
unsafe fn cpq3_nota_do_acesso(a: usize, x: i32, peso: i32) {
    cpq3_pw(a, cpq3_w(a).wrapping_add(ftol(x.wrapping_mul(peso) as f64 * cpq3_k(0x96ff88)) as i16));
    let minimo = (x as i16).wrapping_mul(peso as i16);
    let v = cpq3_w(a);
    cpq3_pw(a, if minimo > v { minimo } else if v > 0x2710 { 0x2710 } else { v });
}

/// O acesso; `gravar` falso só na cópia pelo comparador (sem os históricos em arquivo).
unsafe fn cpq3_acesso(this: usize, clube: usize, comp: usize, peso: u32, gravar: bool) {
    if clube == 0 || comp == 0 {
        aviso_em(0xa83b78, 0x301);
        return;
    }
    let e = cpq3_entrada_do_clube(clube, 8, cpq3_d(comp) as u32);
    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_add(500));
    if cpq3_w(clube + 0x80) > 0x2710 {
        cpq3_pw(clube + 0x80, 0x2710);
    }
    let v = cpq3_i(clube + 0x7b);
    if v < 0x249f0 && v < cpq3_i(clube + 0x73).wrapping_add(500) {
        cpq3_pi(clube + 0x7b, v.wrapping_add(cpq3_w(comp + 0x69) as i32 * 10));
    }
    if cpq3_d(clube + 0xcf) != 0 {
        let tec = cpq3_d(clube + 0xcf);
        if cpq3_d(tec + 0x69) != 0 {
            let objeto = cpq3_d(cpq3_d(0xadadfc).wrapping_add(cpq3_d(comp).wrapping_mul(4)));
            let mut pela = comp;
            if objeto != 0 && cpq3_i(objeto + 0x1c) > -1 {
                let cc = cpq3_d(0xae23d0).wrapping_add((cpq3_i(objeto + 0x1c) as usize).wrapping_mul(0x6b));
                pela = if cc != 0 { cc } else { comp };
            }
            let (p1, p2) = if cpq3_b(objeto + 0x42) as i8 != 1 {
                (400, 400)
            } else {
                let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
                (200, if estrangeira(tec, cpq3_d(clube + 0x53), 1) != 0 { 200 } else { 400 })
            };
            let x = cpq3_w(pela + 0x69) as i32;
            let nj = || cpq3_d(cpq3_d(clube + 0xcf) + 0x69);
            crate::comparacao::ramo("cpq_3 acesso do tecnico");
            cpq3_nota_do_acesso(nj() + 0xa, x, 400);
            cpq3_nota_do_acesso(nj() + 8, x, p2);
            cpq3_nota_do_acesso(nj() + 0xc, x, p1);
        }
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xcf), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, cpq3_d(clube + 0xcf), 1);
        if r != 0 {
            let pontos = (cpq3_w(comp + 0x69) as i32).wrapping_mul(peso as u8 as i8 as i32).wrapping_mul(2500) / 100;
            cpq3_somar(r + 0xb6, pontos);
            cpq3_somar(r + 0xba, pontos);
            cpq3_somar(r + 0xca, pontos);
        }
    }
    if cpq3_d(clube + 0xbf) != 0 {
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xbf), &e);
    }
    for p in cpq3_elenco(clube) {
        cpq3_moral(p, 0xf);
        let c = cpq3_contrato(p, clube);
        if c == 0 {
            continue;
        }
        let v = cpq3_b(c + 0x44) as i8;
        let s = v as i32 + 0x4b;
        *((c + 0x44) as *mut u8) = if s > 0x64 { 0x64 } else if s < -0x64 { 0x9c } else { v.wrapping_add(0x4b) as u8 };
        let tipo = cpq3_b(c + 0x4f) >> 4;
        if (tipo == 0 || (tipo > 3 && tipo != 5)) || !cpq3_vai_renovar(clube, p, c) {
            continue;
        }
        crate::comparacao::ramo("cpq_3 acesso renova");
        cpq3_renovar(p);
    }
    cpq3_do_clube(gravar, this, clube, 8, comp);
}

/// { 0x869480 } (thiscall, ret 0xc, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_promotion_record(clube,
/// comp, peso)`: a reputação do clube (+0x80) sobe 500 (até 10000) e +0x7b sobe 10 × a importância da competição
/// (+0x69) se está abaixo de 150000 e de +0x73 + 500. O técnico (+0xcf) com não jogador: as notas +0xa, +8 e +0xc sobem
/// pela importância da competição (a do registro de competição de clube +0x1c dela, se há) × 400, × 200/400 (estrangeiro
/// na liga com +0x42 = 1, 0x53d980) e × 200/400 (liga com +0x42 = 1) × 10/61, com o mínimo de importância × peso. O
/// histórico do técnico e do presidente (tipo 8), os pontos do técnico (0x86f5b0: +0xb6, +0xba, +0xca somam importância
/// × peso × 25), o moral +15 de cada jogador do elenco (0x544710), o vínculo do contrato (+0x44) +75 (−100..100) e a
/// renovação (0x4cda70) dos contratos dos tipos 1, 2, 3 e 5 que vencem; por fim o histórico do clube (tipo 8). Clube ou
/// competição nulos: a caixa "Error" (linha 0x301).
unsafe extern "thiscall" fn registrar_o_acesso(this: usize, clube: usize, comp: usize, peso: u32) {
    crate::comparacao::ramo("cpq_3 acesso");
    cpq3_acesso(this, clube, comp, peso, true)
}

/// As regiões que o acesso ou o rebaixamento mexem (sem os históricos): o estado do sorteador (o moral sorteia), o clube,
/// o não jogador do técnico, o registro de pontos dele (criado antes, como a original faria), o moral (+0x45) e as
/// reputações (+9..+0xe) de cada jogador do elenco e o vínculo (+0x44) do contrato dele. `None` se algum contrato vai a
/// renovação (a 0x4cda70 faz a oferta: não roda duas vezes; no acesso os tipos 1, 2, 3 e 5, no rebaixamento 1 e 2).
unsafe fn cpq3_regioes(this: usize, clube: usize, acesso: bool) -> Option<Vec<(usize, usize)>> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube, 0x245));
    let tec = cpq3_d(clube + 0xcf);
    if tec != 0 {
        if cpq3_d(tec + 0x69) != 0 {
            reg.push((cpq3_d(tec + 0x69), 0x10));
        }
        if acesso {
            let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
            let r = registro(this, tec, 1);
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    for p in cpq3_elenco(clube) {
        let j = cpq3_d(p + 0x61);
        reg.extend([(j + 9, 6), (j + 0x45, 1)]);
        let c = cpq3_contrato(p, clube);
        if c != 0 {
            let tipo = cpq3_b(c + 0x4f) >> 4;
            let renova = if acesso { tipo != 0 && (tipo <= 3 || tipo == 5) } else { tipo == 1 || tipo == 2 };
            if renova && cpq3_vai_renovar(clube, p, c) {
                return None;
            }
            reg.push((c + 0x44, 1));
        }
    }
    Some(reg)
}

/// Comparador: lado a lado pelas regiões (a cópia sem os históricos em arquivo, que a original já gravou, e sem contar
/// os sorteios); com renovação de contrato ou clube/competição nulos, só a original.
unsafe extern "thiscall" fn comparar_registrar_o_acesso(this: usize, clube: usize, comp: usize, peso: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1051);
    let reg = if clube != 0 && comp != 0 { cpq3_regioes(this, clube, true) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp, peso),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp, peso),
        || crate::aleatorio::sem_contar(|| cpq3_acesso(this, clube, comp, peso, false)));
    PLACAR.registrar(igual, || format!("registrar_o_acesso(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}

/// A nota do não jogador em `a` soma `v` (16 bits) e fica presa a 1..10000.
unsafe fn cpq3_nota_do_rebaixamento(a: usize, v: i16) {
    cpq3_pw(a, cpq3_w(a).wrapping_add(v));
    let n = cpq3_w(a);
    cpq3_pw(a, if n < 1 { 1 } else if n > 0x2710 { 0x2710 } else { n });
}

/// As reputações (+0xb atual, +9 na nação, +0xd mundial) do registro de jogador `j` × `k`.
unsafe fn cpq3_reputacoes(j: usize, k: f64) {
    for o in [0xb, 9, 0xd] {
        cpq3_pw(j + o, ftol(cpq3_w(j + o) as i32 as f64 * k) as i16);
    }
}

unsafe fn cpq3_rebaixamento(this: usize, clube: usize, comp: usize, gravar: bool) {
    if clube == 0 || comp == 0 {
        aviso_em(0xa83b78, 0x476);
        return;
    }
    let e = cpq3_entrada_do_clube(clube, 9, cpq3_d(comp) as u32);
    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_sub(500));
    if cpq3_w(clube + 0x80) < 0x2ee {
        cpq3_pw(clube + 0x80, 0x2ee);
    }
    if cpq3_d(clube + 0xcf) != 0 {
        let tec = cpq3_d(clube + 0xcf);
        if cpq3_d(tec + 0x69) != 0 {
            let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
            let fator: i16 = if estrangeira(tec, cpq3_d(clube + 0x53), 1) != 0 { -8 } else { -16 };
            let nj = || cpq3_d(cpq3_d(clube + 0xcf) + 0x69);
            let x = || cpq3_w(comp + 0x69);
            crate::comparacao::ramo("cpq_3 rebaixamento do tecnico");
            cpq3_nota_do_rebaixamento(nj() + 0xa, x().wrapping_mul(-16));
            cpq3_nota_do_rebaixamento(nj() + 8, x().wrapping_mul(fator));
            cpq3_nota_do_rebaixamento(nj() + 0xc, x().wrapping_mul(-8));
        }
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xcf), &e);
    }
    if cpq3_d(clube + 0xbf) != 0 {
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xbf), &e);
    }
    for p in cpq3_elenco(clube) {
        cpq3_moral(p, -0xf);
        let c = cpq3_contrato(p, clube);
        if c == 0 {
            continue;
        }
        let j = cpq3_d(p + 0x61);
        match cpq3_b(c + 0x4f) >> 4 {
            1 | 2 => {
                if j != 0 {
                    cpq3_reputacoes(j, cpq3_k(0x9689d8)); // 0,9
                }
                if cpq3_vai_renovar(clube, p, c) {
                    crate::comparacao::ramo("cpq_3 rebaixamento renova");
                    cpq3_renovar(p);
                }
            }
            3..=5 => {
                if j != 0 {
                    cpq3_reputacoes(j, cpq3_k(0x9689d0)); // 0,95
                }
            }
            _ => {}
        }
    }
    cpq3_do_clube(gravar, this, clube, 9, comp);
}

/// { 0x86a1c0 } (thiscall, ret 8, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_relegation_record(clube,
/// comp)`: a reputação do clube (+0x80) cai 500 (mínimo 750). O técnico com não jogador: as notas +0xa (−16), +8 (−8
/// estrangeiro, 0x53d980, ou −16) e +0xc (−8) × a importância da competição (+0x69), presas a 1..10000; o histórico do
/// técnico e do presidente (tipo 9). Cada jogador do elenco: moral −15 (0x544710) e, pelo tipo do contrato (+0x4f >> 4):
/// 1 e 2, reputações × 0,9 e a renovação (0x4cda70) se vence; 3 a 5, reputações × 0,95. Por fim o histórico do clube
/// (tipo 9). Clube ou competição nulos: a caixa "Error" (linha 0x476).
unsafe extern "thiscall" fn registrar_o_rebaixamento(this: usize, clube: usize, comp: usize) {
    crate::comparacao::ramo("cpq_3 rebaixamento");
    cpq3_rebaixamento(this, clube, comp, true)
}

/// Comparador: como o do acesso (sem o registro de pontos, que o rebaixamento não usa).
unsafe extern "thiscall" fn comparar_registrar_o_rebaixamento(this: usize, clube: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1052);
    let reg = if clube != 0 && comp != 0 { cpq3_regioes(this, clube, false) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp),
        || crate::aleatorio::sem_contar(|| cpq3_rebaixamento(this, clube, comp, false)));
    PLACAR.registrar(igual, || format!("registrar_o_rebaixamento(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

unsafe fn cpq3_classificacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32, gravar: bool) {
    if clube == 0 {
        aviso_em(0xa83b78, 0x703);
        return;
    }
    let mut e = cpq3_entrada_do_clube(clube, 3, if comp != 0 { cpq3_d(comp) as u32 } else { u32::MAX });
    cpq3_por(&mut e, 0x11, fase as u16 as i16 as i32 as u32);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0870usize);
    cpq3_por(&mut e, 0x15, base(rodada) as u16 as i16 as i32 as u32);
    if cpq3_d(clube + 0xcf) == 0 {
        return;
    }
    cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xcf), &e);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let r = registro(this, cpq3_d(clube + 0xcf), 1);
    if r == 0 {
        return;
    }
    // ponytail: a original faz pow(x, 2,0) (0x945ff0): x × x, exato para a importância (16 bits)
    let x = cpq3_w(comp + 0x69) as f64;
    let pontos = ftol(x * x * (peso as u8 as i8 as i32) as f64);
    cpq3_somar(r + 0xb6, pontos);
    cpq3_somar(r + 0xca, pontos);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        crate::comparacao::ramo("cpq_3 classificado selecao");
        cpq3_somar(r + 0xbe, pontos);
    } else {
        crate::comparacao::ramo("cpq_3 classificado clube");
        cpq3_somar(r + 0xba, pontos);
    }
}

/// { 0x86bdd0 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_qualified_for_round_record(
/// clube, comp, fase, rodada, peso)`: com técnico (+0xcf), o histórico dele (tipo 3: clube, competição ou −1, a fase com
/// sinal e a rodada base 0x4b0870) e os pontos (0x86f5b0): importância² × peso somados em +0xb6, +0xca e, na seleção
/// (0x53b900), +0xbe, ou no clube +0xba. Clube nulo: a caixa "Error" (linha 0x703).
unsafe extern "thiscall" fn registrar_a_classificacao_na_fase(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    cpq3_classificacao(this, clube, comp, fase, rodada, peso, true)
}

/// Comparador: lado a lado pelo registro de pontos do técnico (criado antes, como a original faria), a cópia sem o
/// histórico em arquivo; sem clube ou sem técnico, só a original.
unsafe extern "thiscall" fn comparar_registrar_a_classificacao_na_fase(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(1053);
    if clube == 0 || cpq3_d(clube + 0xcf) == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let r = registro(this, cpq3_d(clube + 0xcf), 1);
    if r == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, clube, comp, fase, rodada, peso),
        || cpq3_classificacao(this, clube, comp, fase, rodada, peso, false));
    PLACAR.registrar(igual, || format!("registrar_a_classificacao_na_fase(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}

// ---------- 253a leva: os registros de carreira: a transferência, a eliminação e a não classificação (staff_records.cpp) ----------

unsafe fn cpq4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpq4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpq4_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpq4_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// O inteiro de 64 bits em `a` somado de `v` (estendido com sinal: cdq, add, adc).
unsafe fn cpq4_somar64(a: usize, v: i32) {
    let x = core::ptr::read_unaligned(a as *const i64);
    core::ptr::write_unaligned(a as *mut i64, x.wrapping_add(v as i64));
}

unsafe fn cpq4_por(e: &mut [u8; 0x40], o: usize, v: i32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// A pessoa do id (0x6e bytes em [0xae23bc]).
unsafe fn cpq4_pessoa(id: usize) -> usize {
    cpq4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A data do jogo copiada (0x417c10) em `a` (a cópia também vai para uma data local).
unsafe fn cpq4_datar(a: *mut u8) {
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(a, t.as_mut_ptr(), 0xae2c90);
}

/// Uma entrada de histórico na pilha com a data construída (0x54a110).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
unsafe fn cpq4_entrada() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    nova(e.as_mut_ptr());
    e
}

/// A entrada `e` no fim da lista da pessoa `id` no arquivo do histórico das pessoas (FILE_LLIST [this], 0x58e340; o
/// começo e o fim em [this+0xc] + id × 8). Sem `gravar` (a cópia pelo comparador), nada.
unsafe fn cpq4_historico(gravar: bool, this: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
    let p = cpq4_d(this + 0xc).wrapping_add(id.wrapping_mul(8));
    f(cpq4_d(this), p, p + 4, e.as_ptr());
}

/// O id do clube contratante da pessoa (+0x39), ou −1.
unsafe fn cpq4_clube_de(pessoa: usize) -> i32 {
    let c = cpq4_d(pessoa + 0x39);
    if c == 0 {
        -1
    } else {
        cpq4_i(c)
    }
}

unsafe fn cpq4_id_ou_menos_1(p: usize) -> i32 {
    if p == 0 {
        -1
    } else {
        cpq4_i(p)
    }
}

/// O id da pessoa e os de +4, +8 e +0xc dela (+0x33 de cada) em `e` a partir de `o`.
unsafe fn cpq4_quem(e: &mut [u8; 0x40], o: usize, p: usize) {
    cpq4_por(e, o, cpq4_i(p));
    for k in 0..3usize {
        cpq4_por(e, o + 4 + 4 * k, cpq4_i(cpq4_d(p + 4 + 4 * k) + 0x33));
    }
}

/// Os itens da lista da troca (LLIST em +4 da transferência, relida a cada passo; o cursor reposto antes), na ordem.
unsafe fn cpq4_troca(t: usize, mut f: impl FnMut(usize)) {
    let l = cpq4_d(t + 4);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpq4_d(l + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(cpq4_d(t + 4));
    while e != 0 {
        f(e);
        e = proximo(cpq4_d(t + 4));
    }
}

/// O tipo do cargo é de jogador (0x53b620: 11, 12, 13 ou 15)?
unsafe fn cpq4_jogador(cargo: u8) -> bool {
    let f: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    f(cargo as u32) != 0 // ponytail: a original empurra o registro com o byte no al; a 0x53b620 lê só o byte
}

unsafe fn cpq4_registro(this: usize, tec: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    f(this, tec, 1)
}

/// Os lados da transferência: (pessoa, contrato da pessoa 0x4cf5a0, vendedor ou 0, comprador).
unsafe fn cpq4_lados(t: usize) -> (usize, usize, usize, usize) {
    let pessoa = cpq4_pessoa(cpq4_d(t + 0xc));
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpq4_d(0xae1050), pessoa);
    let vendedor = if c != 0 && cpq4_b(c + 0x35) & 0x3f != 5 {
        cpq4_d(0xae23b4).wrapping_add(cpq4_d(c + 4).wrapping_mul(0x245))
    } else {
        0
    };
    (pessoa, c, vendedor, cpq4_d(0xae23b4).wrapping_add(cpq4_d(t + 8).wrapping_mul(0x245)))
}

unsafe fn cpq4_transferencia(this: usize, t: usize, gravar: bool) {
    let (pessoa, c, vendedor, comprador) = cpq4_lados(t);
    *(0xae1080 as *mut usize) = c;
    let mut e = cpq4_entrada();
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let valor = total(t, 1, 0, 0, 1);
    let com_troca = || (cpq4_d(t + 4) != 0) as i32;
    let tipo = cpq4_b(t + 0x11) as i8 as i32;
    if comprador != 0 && cpq4_d(comprador + 0xcf) != 0 {
        let tec = cpq4_d(comprador + 0xcf);
        let r = if cpq4_jogador(cpq4_b(t + 0x11)) { cpq4_registro(this, tec) } else { 0 };
        if r != 0 {
            crate::comparacao::ramo("cpq_4 compra");
            if valor > cpq4_i(r + 0x18) {
                crate::comparacao::ramo("cpq_4 maior compra");
                for k in 0..4usize {
                    cpq4_pi(r + 4 * k, if k == 0 { cpq4_i(pessoa) } else { cpq4_i(cpq4_d(pessoa + 4 * k) + 0x33) });
                }
                cpq4_datar((r + 0x10) as *mut u8);
                cpq4_pi(r + 0x18, valor);
                *((r + 0x1c) as *mut u8) = com_troca() as u8;
                cpq4_pi(r + 0x1d, cpq4_id_ou_menos_1(vendedor));
                cpq4_pi(r + 0x21, cpq4_i(comprador));
            }
            cpq4_pw(r + 0xa0, cpq4_w(r + 0xa0).wrapping_add(1));
            cpq4_somar64(r + 0xa4, valor);
            if cpq4_d(t + 4) != 0 {
                cpq4_pw(r + 0xa2, cpq4_w(r + 0xa2).wrapping_add(cpq4_w(cpq4_d(t + 4))));
                cpq4_troca(t, |x| cpq4_somar64(r + 0xac, cpq4_i(cpq4_pessoa(cpq4_d(x)) + 0x52)));
            }
        }
        // o histórico do técnico comprador: a compra e cada jogador dado na troca
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x64;
        cpq4_por(&mut e, 8, cpq4_clube_de(tec));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_por(&mut e, 0x1d, valor);
        cpq4_por(&mut e, 0x21, com_troca());
        cpq4_por(&mut e, 0x25, cpq4_id_ou_menos_1(vendedor));
        cpq4_por(&mut e, 0x29, tipo);
        cpq4_historico(gravar, this, cpq4_d(tec), &e);
        cpq4_troca(t, |x| {
            e[0xc] = 0x65;
            cpq4_quem(&mut e, 0xd, cpq4_pessoa(cpq4_d(x)));
            cpq4_por(&mut e, 0x1d, 0);
            cpq4_por(&mut e, 0x21, 1);
            cpq4_por(&mut e, 0x25, cpq4_i(vendedor));
            cpq4_por(&mut e, 0x29, tipo);
            cpq4_historico(gravar, this, cpq4_d(tec), &e);
        });
    }
    if vendedor != 0 && cpq4_d(vendedor + 0xcf) != 0 {
        let tec = cpq4_d(vendedor + 0xcf);
        let r = if cpq4_jogador(cpq4_b(pessoa + 0x3d)) { cpq4_registro(this, tec) } else { 0 };
        if r != 0 {
            crate::comparacao::ramo("cpq_4 venda");
            if valor > cpq4_i(r + 0x3d) {
                for k in 0..4usize {
                    cpq4_pi(r + 0x25 + 4 * k, if k == 0 { cpq4_i(pessoa) } else { cpq4_i(cpq4_d(pessoa + 4 * k) + 0x33) });
                }
                cpq4_datar((r + 0x35) as *mut u8);
                cpq4_pi(r + 0x3d, valor);
                *((r + 0x41) as *mut u8) = com_troca() as u8;
                cpq4_pi(r + 0x42, cpq4_i(vendedor));
                cpq4_pi(r + 0x46, cpq4_id_ou_menos_1(comprador));
            }
            cpq4_pw(r + 0xa2, cpq4_w(r + 0xa2).wrapping_add(1));
            cpq4_somar64(r + 0xac, valor);
            if cpq4_d(t + 4) != 0 {
                cpq4_pw(r + 0xa0, cpq4_w(r + 0xa0).wrapping_add(cpq4_w(cpq4_d(t + 4))));
                cpq4_troca(t, |x| cpq4_somar64(r + 0xa4, cpq4_i(cpq4_pessoa(cpq4_d(x)) + 0x52)));
            }
        }
        // o histórico do técnico vendedor: a venda e cada jogador recebido na troca
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x65;
        cpq4_por(&mut e, 8, cpq4_clube_de(tec));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_por(&mut e, 0x1d, valor);
        cpq4_por(&mut e, 0x21, com_troca());
        cpq4_por(&mut e, 0x25, cpq4_id_ou_menos_1(comprador));
        cpq4_por(&mut e, 0x29, tipo);
        cpq4_historico(gravar, this, cpq4_d(tec), &e);
        cpq4_troca(t, |x| {
            e[0xc] = 0x64;
            cpq4_quem(&mut e, 0xd, cpq4_pessoa(cpq4_d(x)));
            cpq4_por(&mut e, 0x1d, 0);
            cpq4_por(&mut e, 0x21, 1);
            cpq4_por(&mut e, 0x25, cpq4_i(comprador));
            cpq4_por(&mut e, 0x29, 0xb);
            cpq4_historico(gravar, this, cpq4_d(tec), &e);
        });
    }
    // o histórico do jogador transferido e o de cada jogador da troca
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x66;
    cpq4_por(&mut e, 8, cpq4_clube_de(pessoa));
    cpq4_por(&mut e, 0xd, cpq4_id_ou_menos_1(comprador));
    cpq4_por(&mut e, 0x11, valor);
    cpq4_por(&mut e, 0x19, tipo);
    cpq4_por(&mut e, 0x15, com_troca());
    if pessoa != 0 {
        cpq4_historico(gravar, this, cpq4_d(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
    cpq4_troca(t, |x| {
        let q = cpq4_pessoa(cpq4_d(x));
        cpq4_por(&mut e, 8, cpq4_i(cpq4_d(q + 0x39)));
        cpq4_por(&mut e, 0x11, 0);
        cpq4_por(&mut e, 0xd, cpq4_i(vendedor));
        cpq4_por(&mut e, 0x15, 1);
        cpq4_por(&mut e, 0x19, cpq4_b(x + 6) as i8 as i32);
        if q != 0 {
            cpq4_historico(gravar, this, cpq4_d(q), &e);
        } else {
            aviso_em(0xa83b78, 0x1010);
        }
    });
}

/// { 0x86cd70 } (thiscall, ret 4, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_transfer_records(transf)`:
/// o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o vendedor (fora do estado 5); o valor total da oferta
/// (0x8e7a70 com 1, 0, 0, 1). O técnico do comprador, sendo de jogador o tipo da oferta (+0x11, 0x53b620): no registro
/// (0x86f5b0) a maior compra (+0..+0x24 se o valor passa +0x18), as compras (+0xa0), o gasto (+0xa4, 64 bits), os
/// jogadores dados (+0xa2) e o valor deles (+0x52 de cada, em +0xac). O técnico do vendedor, sendo de jogador o cargo
/// da pessoa (+0x3d): a maior venda (+0x25..+0x49 se passa +0x3d), as vendas (+0xa2), a receita (+0xac), os recebidos
/// (+0xa0) e o valor deles (+0xa4). Os históricos (tipos 0x64 compra, 0x65 venda, 0x66 transferência) do técnico do
/// comprador, do vendedor, da pessoa e dos jogadores da troca, no arquivo (0x58e340).
unsafe extern "thiscall" fn registrar_a_transferencia(this: usize, t: usize) {
    cpq4_transferencia(this, t, true)
}

/// Comparador: lado a lado pela global [0xae1080], o estado do sorteador (o valor da oferta pode pedir a avaliação), a
/// lista da troca (o cursor) e os registros dos dois técnicos (criados antes só quando a original os pede); a cópia
/// sem os históricos em arquivo, que a original já gravou, e sem contar os sorteios.
unsafe extern "thiscall" fn comparar_registrar_a_transferencia(this: usize, t: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1054);
    let (pessoa, _, vendedor, comprador) = cpq4_lados(t);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if cpq4_d(t + 4) != 0 {
        reg.push((cpq4_d(t + 4), 0xe));
    }
    for (clube, cargo) in [(comprador, cpq4_b(t + 0x11)), (vendedor, cpq4_b(pessoa + 0x3d))] {
        if clube != 0 && cpq4_d(clube + 0xcf) != 0 && cpq4_jogador(cargo) {
            let r = cpq4_registro(this, cpq4_d(clube + 0xcf));
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, t),
        || crate::aleatorio::sem_contar(|| cpq4_transferencia(this, t, false)));
    PLACAR.registrar(igual, || format!("registrar_a_transferencia(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// A entrada da pessoa: o clube (+8), o tipo (+0xc), a competição ou −1 (+0xd) e a fase com sinal (+0x11).
unsafe fn cpq4_entrada_da_fase(clube: usize, tipo: u8, comp: usize, fase: u32) -> [u8; 0x40] {
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    cpq4_por(&mut e, 8, cpq4_i(clube));
    e[0xc] = tipo;
    cpq4_por(&mut e, 0xd, cpq4_id_ou_menos_1(comp));
    cpq4_por(&mut e, 0x11, fase as u16 as i16 as i32);
    e
}

/// Na competição de seleção guardada (0x76d940 de [0xb74318]), as convocações da seleção para ela acabam (0x774f90).
unsafe fn cpq4_fim_das_convocacoes(clube: usize, comp: usize) {
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
    if especial(cpq4_d(0xb74318), comp) != 0 {
        crate::comparacao::ramo("cpq_4 fim das convocacoes");
        let encerrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x774f90usize);
        encerrar(cpq4_d(0xb74318), clube, comp);
    }
}

/// { 0x86c000 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_knocked_out_record(clube,
/// comp, fase, rodada, _)`: o histórico do técnico (+0xcf; tipo 4: clube, competição ou −1, a fase com sinal e a rodada
/// base 0x4b0870) e o fim das convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x73b).
unsafe extern "thiscall" fn registrar_a_eliminacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, _x: u32) {
    if clube == 0 {
        return aviso_em(0xa83b78, 0x73b);
    }
    crate::comparacao::ramo("cpq_4 eliminado");
    let mut e = cpq4_entrada_da_fase(clube, 4, comp, fase);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0870usize);
    cpq4_por(&mut e, 0x15, base(rodada) as u16 as i16 as i32);
    if cpq4_d(clube + 0xcf) != 0 {
        cpq4_historico(true, this, cpq4_d(cpq4_d(clube + 0xcf)), &e);
    }
    cpq4_fim_das_convocacoes(clube, comp);
}

// ponytail: o histórico vai para o arquivo e as convocações acabam (não rodam duas vezes) e a cópia sem eles não mexe
// em nada: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_registrar_a_eliminacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(1055)(this, clube, comp, fase, rodada, x)
}

/// { 0x86c1d0 } (thiscall, ret 0x10, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_failed_to_qualify_record(
/// clube, comp, fase, _)`: o histórico do técnico (tipo 7: clube, competição ou −1, a fase com sinal) e o fim das
/// convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x768).
unsafe extern "thiscall" fn registrar_a_nao_classificacao(this: usize, clube: usize, comp: usize, fase: u32, _x: u32) {
    if clube == 0 {
        return aviso_em(0xa83b78, 0x768);
    }
    crate::comparacao::ramo("cpq_4 nao classificado");
    let e = cpq4_entrada_da_fase(clube, 7, comp, fase);
    if cpq4_d(clube + 0xcf) != 0 {
        cpq4_historico(true, this, cpq4_d(cpq4_d(clube + 0xcf)), &e);
    }
    cpq4_fim_das_convocacoes(clube, comp);
}

// ponytail: como a eliminação: só a original.
unsafe extern "thiscall" fn comparar_registrar_a_nao_classificacao(this: usize, clube: usize, comp: usize, fase: u32, x: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1056)(this, clube, comp, fase, x)
}

// ---------- 253a leva: as datas da transferência: a data de conclusão da oferta, o primeiro dia que as regras permitem e a janela fixa de transferências (transfer_manager.cpp) ----------

unsafe fn cpq5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O dia de hoje: Data de 8 bytes em 0xae2c90 (+0 dia do ano i16, +2 ano u16, +4 bissexto).
const CPQ5_HOJE: usize = 0xae2c90;

/// As estáticas das operações de data (0x54c5b0, 0x54c8e0, 0x54c770, 0x54cdb0: valor e guarda), para os comparadores.
const CPQ5_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// `a` antes de `b` na ordem dos originais: o ano (+2, 16 bits sem sinal) e, no mesmo ano, o dia (+0, 16 bits com sinal).
unsafe fn cpq5_antes(a: *const [u32; 2], b: *const [u32; 2]) -> bool {
    let ler = |p: *const [u32; 2]| (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p as usize + 2) as *const u16));
    let ((da, aa), (db, ab)) = (ler(a), ler(b));
    aa < ab || (aa == ab && da < db)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq5_clube(id: usize) -> usize {
    cpq5_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq5_pessoa(id: usize) -> usize {
    cpq5_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do contrato da pessoa da oferta (+0xc): 0x4cf5a0 com o gerente de contratos [0xae1050], guardado em
/// [0xae1080]; contrato de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn cpq5_vendedor(oferta: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpq5_d(0xae1050), cpq5_pessoa(cpq5_d(oferta + 0xc)));
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpq5_b(c + 0x35) & 0x3f != 5 {
        cpq5_clube(cpq5_d(c + 4))
    } else {
        0
    }
}

/// O método +0xc das regras (`TRANSFER_RULES::can_make_offer(oferta, clube, data, motivo)`): 1 permite; 0 e o motivo.
unsafe fn cpq5_permite(regra: usize, oferta: usize, clube: usize, data: *const [u32; 2], motivo: *mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const [u32; 2], *mut u32) -> u32 = core::mem::transmute(cpq5_d(cpq5_d(regra) + 0xc));
    f(regra, oferta, clube, data, motivo)
}

/// Os motivos que não seguram a data (o dia vale assim mesmo): 0x14, 0x15, 0x1f e 0x11.
fn cpq5_motivo_que_libera(m: u32) -> bool {
    matches!(m as u16, 0x14 | 0x15 | 0x1f | 0x11)
}

/// { 0x8cee70 } (thiscall, ret 8, transfer_manager.cpp): `transfer_manager_get_offer_completion_date(oferta, saida)`:
/// a data em que a oferta pode se concretizar, pelo tipo de prazo (+0x30 & 0xf): 0 hoje; 5 o início da janela fixa
/// (0x8d0bb0); 1 e 4 o primeiro dia que as regras permitem (0x8cf0e0, que dá o motivo); 2 o fim da temporada da nação
/// do clube vendedor mais 7 dias (0x669bd0; fora da temporada, hoje; sem vendedor, a caixa "Error" 0x31da); 3 o dia
/// seguinte ao fim do contrato (+0x2d), nunca antes de hoje (sem contrato, hoje). Outros tipos não gravam. Devolve o
/// motivo (só o ax: 0 fora dos tipos 1 e 4). O vendedor (0x4cf5a0, guardado em [0xae1080]) é calculado sempre.
unsafe extern "thiscall" fn data_de_conclusao_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut t, mut t2) = ([0u32; 2], [0u32; 2]);
    let vendedor = cpq5_vendedor(oferta);
    match cpq5_b(oferta + 0x30) & 0xf {
        0 => {
            crate::comparacao::ramo("cpq_5 conclusao imediata");
            copia(saida, &mut t, hoje); // tipo 0: hoje
        }
        5 => {
            let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(0x8d0bb0usize);
            janela(saida, 0);
        }
        1 | 4 => {
            let primeiro: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(0x8cf0e0usize);
            return primeiro(this, oferta, saida);
        }
        2 => {
            if vendedor == 0 {
                aviso_em(0xa8d514, 0x31da);
            } else {
                let temporada: unsafe extern "C" fn(usize, usize, *mut [u32; 2]) -> u32 = core::mem::transmute(0x669bd0usize);
                if temporada(vendedor, 0, saida) != 0 {
                    let r = mais(saida, &mut t, 7);
                    copia(saida, &mut t2, r);
                } else {
                    copia(saida, &mut t2, hoje);
                }
            }
        }
        3 => {
            let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contrato_de(cpq5_d(0xae1050), cpq5_pessoa(cpq5_d(oferta + 0xc)));
            if c != 0 {
                let r = mais((c + 0x2d) as *const [u32; 2], &mut t2, 1);
                copia(saida, &mut t, r);
                if cpq5_antes(saida, hoje) {
                    copia(saida, &mut t2, hoje);
                }
            } else {
                copia(saida, &mut t2, hoje);
            }
        }
        _ => {}
    }
    0
}

/// { 0x8cf0e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_allowed_transfer_date(oferta,
/// saida)`: grava hoje na saída e começa no maior entre hoje e, no tipo de prazo 4 (+0x30 & 0xf), o fim do contrato
/// (+0x2d). Tipo 5: o início da janela fixa (0x8d0bb0) e o método +0xc da regra da nação do comprador (+8) dá o motivo
/// (9 não conta). Pessoa sem ficha de jogador (+0x61): hoje. Senão anda dia a dia (até hoje + 365, sem chegar) pelas
/// regras da nação do comprador e, com o +4 da oferta e um vendedor (0x4cf5a0, guardado em [0xae1080]), da do vendedor:
/// o primeiro dia que as duas permitem (ou cujo motivo libera: 0x14, 0x15, 0x1f, 0x11) vai para a saída e devolve 0;
/// sem dia, devolve o primeiro motivo (9 não conta) e a saída fica hoje. Só o ax é devolvido.
unsafe extern "thiscall" fn primeiro_dia_permitido_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut dia, mut limite, mut inicio, mut t, mut t2) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    padrao(&mut dia);
    mais(hoje, &mut limite, 0x16d);
    padrao(&mut inicio);
    let mut motivo = 0u32;
    // ponytail: o motivo da regra é uma variável só, sem valor antes da primeira chamada (a original não a zera; a
    // regra o grava sempre que devolve 0)
    let mut m = 0u32;
    copia(saida, &mut t, hoje);
    let pessoa = cpq5_pessoa(cpq5_d(oferta + 0xc));
    if cpq5_b(oferta + 0x30) & 0xf == 4 {
        let c = contrato_de(cpq5_d(0xae1050), pessoa);
        if c != 0 {
            copia(&mut inicio, &mut t, (c + 0x2d) as *const [u32; 2]);
            t = *hoje;
            if cpq5_antes(&inicio, hoje) {
                copia(&mut inicio, &mut t, hoje);
            }
        } else {
            copia(&mut inicio, &mut t, hoje);
        }
    } else {
        copia(&mut inicio, &mut t, hoje);
    }
    let comprador = cpq5_clube(cpq5_d(oferta + 8));
    let vendedor = cpq5_vendedor(oferta);
    if cpq5_b(oferta + 0x30) & 0xf == 5 {
        let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(0x8d0bb0usize);
        janela(saida, 0);
        let regra = cpo10_regra(this, cpq5_d(comprador + 0x53));
        if regra != 0 && cpq5_permite(regra, oferta, comprador, saida, &mut m) == 0 && m as u16 != 9 {
            motivo = m;
        }
        return motivo as u16;
    }
    if cpq5_d(pessoa + 0x61) == 0 {
        copia(saida, &mut t, hoje);
        return 0;
    }
    let regra_c = cpo10_regra(this, cpq5_d(comprador + 0x53));
    let regra_v = if vendedor != 0 && cpq5_d(oferta + 4) != 0 { cpo10_regra(this, cpq5_d(vendedor + 0x53)) } else { 0 };
    crate::comparacao::ramo("cpq_5 dia permitido");
    copia(&mut dia, &mut t, &inicio); // o primeiro dia testado é o início
    while cpq5_antes(&dia, &limite) {
        let mut ve_vendedor = true;
        if regra_c != 0 && cpq5_permite(regra_c, oferta, comprador, &dia, &mut m) == 0 {
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            ve_vendedor = cpq5_motivo_que_libera(motivo);
        }
        if ve_vendedor {
            if regra_v == 0 || cpq5_permite(regra_v, oferta, vendedor, &dia, &mut m) != 0 {
                copia(saida, &mut t2, &dia);
                return 0;
            }
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            if cpq5_motivo_que_libera(motivo) {
                copia(saida, &mut t2, &dia);
                return 0;
            }
        }
        let r = mais(&dia, &mut t, 1);
        copia(&mut dia, &mut t2, r);
    }
    motivo as u16
}

/// { 0x8d0bb0 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_get_next_window_dates(inicio, fim)`: as
/// janelas fixas do ano de hoje são 1º a 31 de julho e 15 de dezembro a 15 de janeiro (0x549f70; comparadas só pelo
/// dia do ano, com sinal). Dentro de uma janela, o início é hoje e o fim o da janela (15 de janeiro do ano seguinte, se
/// for a de dezembro); antes de julho, 1º a 31 de julho; entre agosto e 14 de dezembro, 15 de dezembro a 15 de janeiro
/// do ano seguinte. O fim só com `fim` não nulo. Não usa o ecx (os chamadores passam o gerente).
unsafe extern "stdcall" fn janela_fixa_de_transferencias(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let dma: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> *mut [u32; 2] = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    // ponytail: a original empurra o ano num registrador com lixo acima (e o ano + 1 sobre a palavra do argumento); a
    // 0x549f70 só usa os 16 bits de baixo
    let ano = || *((CPQ5_HOJE + 2) as *const u16) as u32;
    let (mut a, mut b, mut c, mut d, mut e, mut t) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    dma(&mut a, 1, 6, ano(), 0xffff_ffff); // 1º de julho
    dma(&mut b, 0x1f, 6, ano(), 0xffff_ffff); // 31 de julho
    dma(&mut c, 0xf, 0xb, ano(), 0xffff_ffff); // 15 de dezembro
    dma(&mut d, 0xf, 0, ano(), 0xffff_ffff); // 15 de janeiro
    let dia = |x: &[u32; 2]| x[0] as u16 as i16;
    let h = *(CPQ5_HOJE as *const i16);
    if h >= dia(&a) && h <= dia(&b) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            copia(fim, &mut t, &b);
        }
    } else if h >= dia(&c) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            dma(&mut e, 0xf, 0, (ano() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copia(fim, &mut t, &e);
        }
    } else if h <= dia(&d) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            copia(fim, &mut t, &d);
        }
    } else if h < dia(&a) {
        crate::comparacao::ramo("cpq_5 janela de julho");
        copia(inicio, &mut t, &a);
        if !fim.is_null() {
            copia(fim, &mut t, &b);
        }
    } else {
        copia(inicio, &mut t, &c);
        if !fim.is_null() {
            dma(&mut e, 0xf, 0, (ano() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copia(fim, &mut t, &e);
        }
    }
}

/// As regiões dos comparadores de datas: a saída, o contrato guardado [0xae1080] e as estáticas das datas.
fn cpq5_regioes(saidas: &[usize]) -> Vec<(usize, usize)> {
    let mut r: Vec<(usize, usize)> = saidas.iter().filter(|&&s| s != 0).map(|&s| (s, 8)).collect();
    r.push((0xae1080, 4));
    r.extend_from_slice(&CPQ5_ESTATICAS);
    r
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo (as regras, a temporada e a
/// janela só calculam).
unsafe extern "thiscall" fn comparar_data_de_conclusao_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(1057);
    let reg = cpq5_regioes(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || data_de_conclusao_da_oferta(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("data_de_conclusao_da_oferta(tipo {}) motivo {} x {}, memória igual {}", cpq5_b(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo.
unsafe extern "thiscall" fn comparar_primeiro_dia_permitido_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(1058);
    let reg = cpq5_regioes(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || primeiro_dia_permitido_da_oferta(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("primeiro_dia_permitido_da_oferta(tipo {}) motivo {} x {}, memória igual {}", cpq5_b(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pelo início e o fim (só datas, sem estáticas).
unsafe extern "stdcall" fn comparar_janela_fixa_de_transferencias(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let o = orig::<unsafe extern "stdcall" fn(*mut [u32; 2], *mut [u32; 2])>(1059);
    let reg = cpq5_regioes(&[inicio as usize, fim as usize]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(inicio, fim), || janela_fixa_de_transferencias(inicio, fim));
    PLACAR.registrar(igual, || format!("janela_fixa_de_transferencias(hoje {:x}, fim {}) diferente", *(CPQ5_HOJE as *const u32), !fim.is_null()));
}

// ---------- 253a leva: o dinheiro da transferência: a compensação pela pessoa, pagar a transferência e guardar as cláusulas, e se o clube vendedor tem de aceitar (transfer_manager.cpp) ----------

unsafe fn cpq6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq6_clube(id: usize) -> usize {
    cpq6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq6_pessoa(id: usize) -> usize {
    cpq6_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do contrato da `pessoa` (0x4cf5a0 com o gerente de contratos [0xae1050], guardado em [0xae1080]); contrato
/// de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn cpq6_vendedor(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpq6_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpq6_b(c + 0x35) & 0x3f != 5 {
        cpq6_clube(cpq6_d(c + 4))
    } else {
        0
    }
}

unsafe fn cpq6_por_u32(r: &mut [u8; 13], k: usize, v: usize) {
    r[k..k + 4].copy_from_slice(&(v as u32).to_le_bytes());
}

/// As estáticas das operações de data (valor e guarda), para os comparadores.
const CPQ6_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// { 0x8d0780 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só da data local):
/// `transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas)`: sem pessoa ou comprador, a caixa
/// "Error" 0x372e e 0. Comissão técnica (sem ficha de jogador +0x61, com +0x69) de contrato tipo 2: o que falta do
/// contrato (dias até o fim +0x2d × o salário +0xc / 7) entre 1000 e 1000000. Regras novas ([0xae233c] = 1, sem o
/// argumento) com contrato tipo 2: a média entre o que falta e o valor do jogador no vendedor (0x4d0c60). Senão (contrato
/// que não é de tipo 5, as duas nações): entre clubes da nação [0x9cf4a8], metade do valor; fora disso, só abaixo de 24
/// anos na data (o fim do contrato, no tipo 2; senão hoje) e entre nações britânicas ou a Irlanda (0x5446d0), metade
/// do valor. Os outros casos, 0.
unsafe extern "stdcall" fn compensacao_pela_pessoa(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], usize) -> usize = core::mem::transmute(0x417c10usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let idade: unsafe extern "C" fn(usize, *const [u32; 2]) -> u16 = core::mem::transmute(0x53ab90usize);
    let britanica: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5446d0usize);
    let gerente = || cpq6_d(0xae1050);
    // o que falta do contrato: os dias até o fim × o salário semanal, por 7 (divisão truncada, como a original)
    let resto = |c: usize| dias_ate(c + 0x2d, cpq6_d(0xae2c90) as u32, cpq6_d(0xae2c94) as u32).wrapping_mul(cpq6_d(c + 0xc) as i32) / 7;
    let mut data = [0u32; 2];
    let mut t = [0u32; 2];
    padrao(&mut data);
    if pessoa == 0 || comprador == 0 {
        aviso_em(0xa8d514, 0x372e);
        return 0;
    }
    if cpq6_d(pessoa + 0x61) == 0 && cpq6_d(pessoa + 0x69) != 0 {
        let c = contrato_de(gerente(), pessoa);
        if c == 0 || cpq6_b(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        return resto(c).clamp(1000, 1_000_000);
    }
    if cpq6_b(0xae233c) == 1 && sem_regras_novas == 0 {
        let c = contrato_de(gerente(), pessoa);
        if c == 0 || cpq6_b(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        let r = resto(c);
        let v = valor(gerente(), pessoa, cpq6_clube(cpq6_d(c + 4)), 1, 0xffff_ffff, 0);
        return v.wrapping_add(r) / 2;
    }
    let c = contrato_de(gerente(), pessoa);
    if c == 0 {
        return 0;
    }
    let tipo = cpq6_b(c + 0x35) & 0x3f;
    if tipo == 5 {
        return 0;
    }
    let vendedor = cpq6_clube(cpq6_d(c + 4));
    if cpq6_d(comprador + 0x53) == 0 || cpq6_d(vendedor + 0x53) == 0 {
        return 0;
    }
    if tipo == 2 {
        copia(&mut data, &mut t, c + 0x2d);
    } else {
        copia(&mut data, &mut t, 0xae2c90);
    }
    let n = cpq6_d(0x9cf4a8);
    if cpq6_d(cpq6_d(vendedor + 0x53)) == n && cpq6_d(cpq6_d(comprador + 0x53)) == n {
        return valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2;
    }
    if idade(pessoa, &data) as u8 as i8 >= 0x18 {
        return 0;
    }
    if britanica(cpq6_d(comprador + 0x53)) == 0 || britanica(cpq6_d(vendedor + 0x53)) == 0 {
        return 0;
    }
    crate::comparacao::ramo("cpq_6 compensacao britanica");
    valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2 // menor de 24 anos entre clubes britânicos
}

/// { 0x8ca8a0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::pay_fee_and_record_clauses(oferta)`: soma o
/// valor (+0x1b) aos totais das transferências (0x448ad0); o comprador (+8) paga e o vendedor (clube do contrato da
/// pessoa +0xc, 0x4cf5a0) recebe a primeira parcela (o valor / parcelas +0x40, ou tudo), em 64 bits (0x599890,
/// 0x599800). Com parcelas, guarda as que faltam (13 bytes: a parcela, os ids do comprador e do vendedor, parcelas − 1)
/// na lista +0x872 (0x68d0d0) e soma ao +0x1c das finanças a parcela × −5/21 (comprador) e × 5/21 (vendedor). Guarda
/// as cláusulas: a porcentagem da próxima venda (+0x3f, 0x8caaf0, marca 8), o prêmio por jogo no time (+0x35 com o byte
/// +0x39, 0x8cac40, marca 2) e o prêmio por jogo pela seleção (+0x3a com +0x3e, 0x8cad90, marca 4); as marcas no
/// registro de 8 bytes da pessoa em [this+0x86e].
unsafe extern "thiscall" fn pagar_transferencia_e_clausulas(this: usize, oferta: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599890usize);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599800usize);
    let por_na_lista: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let proxima_venda: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8caaf0usize);
    let premio_time: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8cac40usize);
    let premio_selecao: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8cad90usize);
    let tabela = || cpq6_d(0xae2a38);
    let marcar = |bit: u8| {
        let m = (cpq6_d(this + 0x86e) + cpq6_d(oferta + 0xc) * 8) as *mut u8;
        *m |= bit;
    };
    let comprador = cpq6_clube(cpq6_d(oferta + 8));
    let vendedor = cpq6_vendedor(cpq6_pessoa(cpq6_d(oferta + 0xc)));
    let fin_c = financas(tabela(), comprador);
    // ponytail: sem vendedor, a original leria as finanças de um lugar sem valor (e cairia adiante, em [vendedor]); a
    // cópia não paga ninguém
    let fin_v = if vendedor != 0 { financas(tabela(), vendedor) } else { 0 };
    let valor = cpq6_d(oferta + 0x1b) as i32;
    let n = cpq6_b(oferta + 0x40) as i8;
    let primeira = if n > 0 { (valor / n as i32) as i64 } else { valor as i64 };
    totais(cpq6_d(0xad9dac), valor, comprador, vendedor);
    crate::comparacao::ramo("cpq_6 pagar transferencia");
    pagar(fin_c, primeira);
    if fin_v != 0 {
        receber(fin_v, primeira); // o vendedor recebe
    }
    let mut r = [0u8; 13];
    let n = cpq6_b(oferta + 0x40) as i8;
    if n > 0 {
        let parcela = cpq6_d(oferta + 0x1b) as i32 / n as i32;
        cpq6_por_u32(&mut r, 0, parcela as usize);
        cpq6_por_u32(&mut r, 4, cpq6_d(comprador));
        cpq6_por_u32(&mut r, 8, cpq6_d(vendedor));
        r[12] = (n as u8).wrapping_sub(1);
        let fc = financas(tabela(), comprador);
        let fv = financas(tabela(), vendedor);
        if fc != 0 {
            let s = (fc + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(ftol(parcela as f64 * f64_em(0x970420)));
        }
        if fv != 0 {
            let s = (fv + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(ftol(parcela as f64 * f64_em(0x970418)));
        }
        por_na_lista(this + 0x872, r.as_ptr(), 0xd);
    }
    let p = cpq6_b(oferta + 0x3f);
    if (p as i8) > 0 {
        r[0] = p;
        cpq6_por_u32(&mut r, 1, cpq6_d(comprador));
        cpq6_por_u32(&mut r, 5, cpq6_d(vendedor));
        cpq6_por_u32(&mut r, 9, cpq6_d(oferta + 0xc));
        proxima_venda(this, r.as_ptr());
        marcar(8);
    }
    let v = cpq6_d(oferta + 0x35) as i32;
    if v > 0 {
        cpq6_por_u32(&mut r, 0, cpq6_d(oferta + 0xc));
        cpq6_por_u32(&mut r, 4, v as usize);
        cpq6_por_u32(&mut r, 9, cpq6_d(vendedor));
        r[8] = cpq6_b(oferta + 0x39);
        premio_time(this, r.as_ptr());
        marcar(2);
    }
    let v = cpq6_d(oferta + 0x3a) as i32;
    if v > 0 {
        cpq6_por_u32(&mut r, 0, cpq6_d(oferta + 0xc));
        cpq6_por_u32(&mut r, 4, v as usize);
        cpq6_por_u32(&mut r, 9, cpq6_d(vendedor));
        r[8] = cpq6_b(oferta + 0x3e);
        premio_selecao(this, r.as_ptr());
        marcar(4);
    }
}

/// { 0x8cd240 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só dos valores locais):
/// `transfer_manager_selling_club_must_accept(oferta, estrito, motivo)`: sem vendedor (clube do contrato da pessoa
/// +0xc que não é de tipo 5, 0x4cf5a0), 1. Com cláusula de rescisão ativa (0x867b50), sem troca (+4) nem parcelas
/// (+0x40) e o valor (+0x1b) não abaixo da multa (+0x21 do contrato; os dois por 0x43f460 com 4): motivo 0x1b e 1. Com
/// o vendedor em crise (+0x165 das finanças = 1), o preço pedido (0x4d1600) não abaixo de 20 × a reputação (+0x80;
/// pelo menos 20000), o valor ponderado da oferta (0x8e7b50) acima de 50000 e o dobro dele acima do preço: motivo 0x1c
/// e 1 (com `estrito`, só se o preço ficar abaixo de 1,3 × o valor ponderado). Senão 0.
unsafe extern "stdcall" fn clube_vendedor_deve_aceitar(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let ativa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
    let exibir: unsafe extern "thiscall" fn(*const u32, u32) -> i64 = core::mem::transmute(0x43f460usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let comprador = cpq6_clube(cpq6_d(oferta + 8));
    let pessoa = cpq6_pessoa(cpq6_d(oferta + 0xc));
    let vendedor = cpq6_vendedor(pessoa);
    if vendedor == 0 {
        return 1;
    }
    let c = contrato_de(cpq6_d(0xae1050), pessoa);
    crate::comparacao::ramo("cpq_6 clube deve aceitar");
    if ativa(c) != 0 && cpq6_d(oferta + 4) == 0 && cpq6_b(oferta + 0x40) == 0 {
        let (multa, valor) = (cpq6_d(c + 0x21) as u32, cpq6_d(oferta + 0x1b) as u32);
        let m = exibir(&multa, 4);
        let v = exibir(&valor, 4);
        if v >= m {
            if !motivo.is_null() {
                *motivo = 0x1b;
            }
            return 1;
        }
    }
    if cpq6_b(financas(cpq6_d(0xae2a38), vendedor) + 0x165) != 1 {
        return 0;
    }
    let pedido = preco(cpq6_d(0xae1050), vendedor, comprador, pessoa, 0, 1, 1);
    let w = ponderado(oferta, 0, 0, 1, 0, 1);
    let piso = (*((vendedor + 0x80) as *const i16) as i32 * 20).max(20000);
    if pedido < piso || w.wrapping_mul(2) <= pedido || w <= 50000 {
        return 0;
    }
    if estrito != 0 {
        let w2 = ponderado(oferta, 0, 0, 1, 0, 1);
        if !((pedido as f64) < w2 as f64 * f64_em(0x968de8)) {
            return 0;
        }
    }
    if !motivo.is_null() {
        *motivo = 0x1c;
    }
    1
}

/// Comparador: pelo valor (lado a lado pelas estáticas das datas; o valor do jogador 0x4d0c60 e a idade só calculam).
unsafe extern "stdcall" fn comparar_compensacao_pela_pessoa(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> i32>(1060);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&CPQ6_ESTATICAS, || o(pessoa, comprador, sem_regras_novas), || compensacao_pela_pessoa(pessoa, comprador, sem_regras_novas));
    PLACAR.registrar(igual && ro == rr, || format!("compensacao_pela_pessoa(pessoa {:x}) {} x {}, estáticas iguais {}", if pessoa != 0 { cpq6_d(pessoa) } else { 0 }, ro, rr, igual));
    ro
}

/// Comparador: paga, acrescenta nas listas (aloca) e marca: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_pagar_transferencia_e_clausulas(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1061)(this, oferta)
}

/// Comparador: lado a lado pelo motivo e [0xae1080], e pelo valor devolvido (o preço e o valor ponderado só calculam).
unsafe extern "stdcall" fn comparar_clube_vendedor_deve_aceitar(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32, *mut u16) -> u32>(1062);
    let mut reg: Vec<(usize, usize)> = vec![(0xae1080, 4)];
    if !motivo.is_null() {
        reg.push((motivo as usize, 2));
    }
    reg.extend_from_slice(&CPQ6_ESTATICAS);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(oferta, estrito, motivo), || clube_vendedor_deve_aceitar(oferta, estrito, motivo));
    PLACAR.registrar(igual && ro == rr, || format!("clube_vendedor_deve_aceitar(estrito {}) {} x {}, memória igual {}", estrito, ro, rr, igual));
    ro
}

// ---------- 253a leva: os contratos da transferência: a permissão de trabalho na mudança de clube, concluir a oferta de contrato aceita e oferecer contrato ao jogador da oferta (transfer_manager.cpp) ----------

unsafe fn cpq7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq7_clube(id: usize) -> usize {
    cpq7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq7_pessoa(id: usize) -> usize {
    cpq7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A permissão de trabalho da `pessoa` (14 bytes, ordenadas pelo id: [this+0x93a], conta +0x93e), como a original
/// procura: pessoa com id fora de [0xae2364] ou sem a marca (bit 0x20 do registro de 8 bytes em [this+0x86e]), 0; senão
/// a busca binária (0x946464 com 0x8e55c0).
unsafe fn cpq7_permissao(this: usize, pessoa: usize) -> usize {
    if pessoa != 0 && cpq7_d(pessoa) as i32 >= cpq7_d(0xae2364) as i32 {
        return 0;
    }
    let id = cpq7_d(pessoa) as u32;
    if cpq7_b(cpq7_d(this + 0x86e) + id as usize * 8) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(&id, cpq7_d(this + 0x93a), cpq7_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// { 0x8c7e20 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só da data local):
/// `TRANSFER_MANAGER::update_work_permit_on_move(pessoa, clube)`: se o jogador precisa de permissão no clube novo
/// (0x8c7560 com 1) e já tem uma: troca a nação da permissão (+0xa) pela do clube (+0x53), zera +4 e +6 e, se mudou de
/// nação e as duas não são da marca 0xe (+0x76), grava o ano de hoje (+8); clube sem nação: a caixa "Error" 0x1dc2. Sem
/// permissão: a nova (0x8c7070 com o ano de hoje, ordenando), depois de hoje + 52 semanas (0x54c770, sem uso). Se não
/// precisa (a 0x8c7560 é chamada de novo) ou sem clube: tira a permissão que tiver (a marca 0x20, a última no lugar
/// dela, conta − 1 e reordena com 0x945333).
unsafe extern "thiscall" fn permissao_de_trabalho_na_mudanca(this: usize, pessoa: usize, clube: usize) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if clube != 0 {
        if precisa(this, pessoa, clube, 1) != 0 {
            let r = cpq7_permissao(this, pessoa);
            if r != 0 {
                let nacao = cpq7_d(0xae23a8).wrapping_add(cpq7_d(r + 0xa).wrapping_mul(0x122));
                let nc = cpq7_d(clube + 0x53);
                if nc == 0 {
                    aviso_em(0xa8d514, 0x1dc2);
                    return;
                }
                if nacao != nc && !(cpq7_b(nacao + 0x76) == 0xe && cpq7_b(nc + 0x76) == 0xe) {
                    core::ptr::write_unaligned((r + 8) as *mut u16, *((0xae2c92) as *const u16));
                }
                core::ptr::write_unaligned((r + 4) as *mut u16, 0);
                core::ptr::write_unaligned((r + 6) as *mut u16, 0);
                core::ptr::write_unaligned((r + 0xa) as *mut u32, cpq7_d(nc) as u32);
                return;
            }
            let semanas: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c770usize);
            let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
            let mut t = [0u32; 2];
            semanas(0xae2c90, &mut t, 0x34);
            crate::comparacao::ramo("cpq_7 permissao nova");
            // ponytail: a original empurra o ano com lixo nos 16 bits altos; a 0x8c7070 só grava a palavra
            let ano = *((0xae2c92) as *const u16) as u32;
            dar(this, pessoa, cpq7_d(clube + 0x53), ano, 0, 1); // a permissão nova
            return;
        }
        if precisa(this, pessoa, clube, 1) != 0 {
            return;
        }
    }
    let r = cpq7_permissao(this, pessoa);
    if r == 0 {
        return;
    }
    let marca = (cpq7_d(this + 0x86e) + cpq7_d(pessoa) * 8) as *mut u8;
    *marca &= 0xdf;
    let ultima = cpq7_d(this + 0x93a) + cpq7_d(this + 0x93e) * 0xe - 0xe;
    core::ptr::copy(ultima as *const u8, r as *mut u8, 0xe);
    let n = cpq7_d(this + 0x93e).wrapping_sub(1);
    core::ptr::write_unaligned((this + 0x93e) as *mut usize, n);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpq7_d(this + 0x93a), n as u32, 0xe, 0x8e55c0);
}

/// { 0x8c38b0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::complete_accepted_contract_offer(oferta)`: oferta de contrato (+0 a pessoa, +4 o clube, +8 as
/// luvas, +0x61 o cargo). Se o jogador precisa de permissão no clube (0x8c7560 com 1) e ela não sai (0x8c7950): com
/// contrato de tipo 5 (0x4cf5a0, guardado em [0xae1080]), a notícia da oferta que caiu (0x8de790 com uma
/// TRANSFER_OFFER local: 0x8e5680, ids em +8 e +0xc, +0x1f = 0, motivo 0x13; 0x8e5710); senão a notícia da permissão
/// recusada (0x8e4290); devolve 0. Senão assina (0x4cf720) e, assinado, o clube paga as luvas (0x5a0590, 0x5998e0) e
/// devolve 1.
unsafe extern "thiscall" fn concluir_oferta_de_contrato_aceita(this: usize, oferta: usize) -> u32 {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c7950usize);
    let pessoa = cpq7_pessoa(cpq7_d(oferta));
    let clube = cpq7_clube(cpq7_d(oferta + 4));
    if precisa(this, pessoa, clube, 1) != 0 && concedida(this, pessoa, cpq7_d(clube + 0x53), oferta, 0, 0) == 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato_de(cpq7_d(0xae1050), pessoa);
        *(0xae1080 as *mut usize) = c;
        if c != 0 && cpq7_b(c + 0x35) & 0x3f == 5 {
            let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x8e5680usize);
            let caiu: unsafe extern "thiscall" fn(usize, *mut u8, usize, u32) = core::mem::transmute(0x8de790usize);
            let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(0x8e5710usize);
            let mut o = [0u8; 0x44];
            construir(o.as_mut_ptr());
            o[8..12].copy_from_slice(&(cpq7_d(clube) as u32).to_le_bytes());
            o[0xc..0x10].copy_from_slice(&(cpq7_d(pessoa) as u32).to_le_bytes());
            o[0x1f] = 0;
            caiu(this, o.as_mut_ptr(), clube, 0x13);
            destruir(o.as_mut_ptr());
        } else {
            let recusada: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e4290usize);
            recusada(this, pessoa, oferta);
        }
        return 0;
    }
    let assinar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4cf720usize);
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x4cf720 e a 0x53bd10 só usam o byte
    if assinar(cpq7_d(0xae1050), oferta, cpq7_b(oferta + 0x61) as u32) == 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let luvas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5998e0usize);
    let valor = cpq7_d(oferta + 8) as u32;
    let fin = financas(cpq7_d(0xae2a38), clube);
    crate::comparacao::ramo("cpq_7 luvas");
    luvas(fin, valor); // as luvas
    1
}

/// { 0x8c1e40 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH do contrato local):
/// `TRANSFER_MANAGER::offer_contract_to_offer_player(clube, pessoa, oferta)`: monta o contrato (STAFF_CONTRACT local,
/// 0x85e410) que o clube ofereceria (0x4cda70 com a vaga +0x20 da oferta, fator de salário 1 e modo 2). Montado:
/// acrescenta a oferta de contrato (0x4cc490 com a vaga +0x20, o valor +0x24, 0xb, 0, 1, 0). Sem contrato: sendo o
/// jogador principal (+0xc), atualiza a lista do clube (0x849ce0 e 0x843e10 com 1) e retira a oferta (0x8bfcb0, motivo
/// 0x16); sendo da troca, soma o valor dele (+0x52) ao da oferta (+0x1b) e o tira da troca (0x8c41c0 com 1 e 0x16).
/// Devolve o que a última chamada devolveu.
unsafe extern "thiscall" fn oferecer_contrato_ao_jogador_da_oferta(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x85e410usize);
    let oferecer: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, f64, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    let mut c = [0u8; 0x50];
    construir(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_7 contrato da oferta");
    if oferecer(cpq7_d(0xae1050), pessoa, clube, c.as_mut_ptr(), cpq7_d(oferta + 0x20), 1.0f64, 2) != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cc490usize);
        return acrescentar(cpq7_d(0xae1050), c.as_mut_ptr(), cpq7_d(oferta + 0x20), cpq7_d(oferta + 0x24), 0xb, 0, 1, 0);
    }
    if cpq7_d(pessoa) == cpq7_d(oferta + 0xc) {
        let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
        let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
        let e = entrada(cpq7_d(this + 0x856), clube, pessoa, (cpq7_b(oferta + 0x11) != 0xb) as u32, 0);
        if e != 0 {
            // ponytail: a original empurra o tipo (setne) com o resto do ponteiro da entrada acima; a 0x843e10 só lê o byte
            atualizar(cpq7_d(this + 0x856), pessoa, clube, 1, (cpq7_b(e) != 0xb) as u32);
        }
        let mut o = oferta;
        return retirar(this, &mut o, 0, 0x16);
    }
    let v = (oferta + 0x1b) as *mut i32;
    core::ptr::write_unaligned(v, core::ptr::read_unaligned(v).wrapping_add(cpq7_d(pessoa + 0x52) as i32));
    let tirar_da_troca: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c41c0usize);
    tirar_da_troca(cpq7_d(0xde1f54), pessoa, oferta, 1, 0x16)
}

/// Comparador: grava as permissões (pode crescer o vetor e reordena): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_permissao_de_trabalho_na_mudanca(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1063)(this, pessoa, clube)
}

/// Comparador: assina o contrato (aloca), paga e posta notícias: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_concluir_oferta_de_contrato_aceita(this: usize, oferta: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1064)(this, oferta)
}

/// Comparador: acrescenta ofertas de contrato (aloca), mexe nas listas e retira ofertas: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_oferecer_contrato_ao_jogador_da_oferta(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1065)(this, clube, pessoa, oferta)
}

// ---------- 253a leva: as ofertas de transferência: desfazer a oferta, fazer a oferta de empréstimo e concluir as transferências futuras do dia (transfer_manager.cpp) ----------

unsafe fn cpq8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq8_clube(id: usize) -> usize {
    cpq8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq8_pessoa(id: usize) -> usize {
    cpq8_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato da pessoa da oferta (+0xc; 0x4cf5a0 com o gerente [0xae1050]), guardado em [0xae1080].
unsafe fn cpq8_contrato(oferta: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)));
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato da pessoa da oferta; contrato de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn cpq8_vendedor(oferta: usize) -> usize {
    let c = cpq8_contrato(oferta);
    if c != 0 && cpq8_b(c + 0x35) & 0x3f != 5 {
        cpq8_clube(cpq8_d(c + 4))
    } else {
        0
    }
}

/// Recomeça o cursor da LLIST (+0xa = +2, o primeiro nó).
unsafe fn cpq8_recomecar(lista: usize) {
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cpq8_d(lista + 2));
}

/// O estado da oferta (+0x2c) que deixa a marca 1 da pessoa sair: 2, 3, 0x14, 0x10, 4 e 0xf com +0x2e ≥ 3.
unsafe fn cpq8_estado_encerrado(oferta: usize) -> bool {
    match cpq8_b(oferta + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpq8_b(oferta + 0x2e) >= 3,
        _ => false,
    }
}

/// { 0x8c44e0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::collapse_offer(oferta, clube, motivo,
/// tirar_futura)`: na oferta de estado 0x13 tira a marca 0x10 da pessoa (+0xc) e dos jogadores da troca (lista +4) e,
/// com `tirar_futura`, tira a transferência futura (0x8cbf70). Põe o estado 0x10, fecha as propostas (0x8c6db0), solta
/// os lances do comprador (+8) pela pessoa e do vendedor (clube do contrato, guardado em [0xae1080]) pelos da troca
/// (0x84aeb0), posta a notícia (0x8de790 com o clube e o motivo) e agenda a oferta (0x8c6220 com 0x14). Encerrada a
/// oferta e todas as da lista +0 (estados de `cpq8_estado_encerrado`), tira a marca 1 da pessoa. Atualiza a lista do
/// comprador (0x849ce0 e 0x843e10 com 1), tira a oferta de contrato (0x4cca70), zera +0x10 e solta os contratos da troca
/// (0x4ccb30 com a vaga +0x20; +4 do item = 0).
unsafe extern "thiscall" fn desfazer_oferta(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let marca = |id: usize| (cpq8_d(this + 0x86e) + id * 8) as *mut u8;
    crate::comparacao::ramo("cpq_8 desfazer oferta");
    if cpq8_b(oferta + 0x2c) == 0x13 {
        *marca(cpq8_d(oferta + 0xc)) &= 0xef;
        if cpq8_d(oferta + 4) != 0 {
            cpq8_recomecar(cpq8_d(oferta + 4));
            loop {
                let it = proximo(cpq8_d(oferta + 4));
                if it == 0 {
                    break;
                }
                *marca(cpq8_d(it)) &= 0xef;
            }
        }
        if tirar_futura != 0 {
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cbf70usize);
            tirar(this, oferta);
        }
    }
    *((oferta + 0x2c) as *mut u8) = 0x10;
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
    fechar(this, oferta);
    let pessoa = cpq8_pessoa(cpq8_d(oferta + 0xc));
    let c = cpq8_contrato(oferta);
    let vendedor = if c != 0 { cpq8_clube(cpq8_d(c + 4)) } else { 0 };
    desmarcar(cpq8_d(this + 0x856), cpq8_clube(cpq8_d(oferta + 8)), pessoa);
    if cpq8_d(oferta + 4) != 0 {
        cpq8_recomecar(cpq8_d(oferta + 4));
        loop {
            let it = proximo(cpq8_d(oferta + 4));
            if it == 0 {
                break;
            }
            desmarcar(cpq8_d(this + 0x856), vendedor, cpq8_pessoa(cpq8_d(it)));
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8de790usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    noticia(this, oferta, clube, motivo);
    agendar(this, oferta, 0x14, 0);
    if cpq8_estado_encerrado(oferta) {
        let mut todas = true;
        if cpq8_d(oferta) != 0 {
            cpq8_recomecar(cpq8_d(oferta));
            loop {
                let it = proximo(cpq8_d(oferta));
                if it == 0 {
                    break;
                }
                if !cpq8_estado_encerrado(cpq8_d(cpq8_d(this) + cpq8_d(it) * 4)) {
                    todas = false;
                    break;
                }
            }
        }
        if todas {
            let m = marca(cpq8_d(oferta + 0xc));
            if *m & 1 != 0 {
                *m &= 0xfe;
            }
        }
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
    let e = entrada(cpq8_d(this + 0x856), cpq8_clube(cpq8_d(oferta + 8)), cpq8_pessoa(cpq8_d(oferta + 0xc)), (cpq8_b(oferta + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o tipo (setne) com lixo nos bytes altos; a 0x843e10 só lê o byte
        atualizar(cpq8_d(this + 0x856), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8)), 1, (cpq8_b(e) != 0xb) as u32);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    tirar_contrato(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8))); // a oferta de contrato sai
    *((oferta + 0x10) as *mut u8) = 0;
    if cpq8_d(oferta + 4) != 0 {
        let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
        cpq8_recomecar(cpq8_d(oferta + 4));
        loop {
            let it = proximo(cpq8_d(oferta + 4));
            if it == 0 {
                break;
            }
            soltar(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(it)), cpq8_d(oferta + 0x20));
            *((it + 4) as *mut u8) = 0;
        }
    }
}

/// { 0x8bed60 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::make_loan_offer(jogador, clube, fim, percentual, a5, a6)`: numa TRANSFER_OFFER local (0x8e5680)
/// monta a oferta de empréstimo (0x8e7810) e, num STAFF_CONTRACT local (0x85e410), o contrato pelo atual (0x85e590 com o
/// fim, a porcentagem do salário e as duas cláusulas); acrescenta a oferta (0x8be300 com o contrato) e a oferta de
/// contrato (0x4cc490 com a vaga +0x20 e o valor +0x24 da oferta, 0xb, 0, 1, 0). Destrói a oferta local (0x8e5710) e
/// devolve a vaga (+0x20).
unsafe extern "thiscall" fn fazer_oferta_de_emprestimo(this: usize, jogador: usize, clube: usize, fim: usize, percentual: u32, a5: u32, a6: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x8e5680usize);
    let emprestimo: unsafe extern "thiscall" fn(*mut u8, usize, usize) = core::mem::transmute(0x8e7810usize);
    let construir_contrato: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x85e410usize);
    let pelo_atual: unsafe extern "thiscall" fn(*mut u8, usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(0x8be300usize);
    let acrescentar_contrato: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cc490usize);
    let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(0x8e5710usize);
    let campo = |o: &[u8; 0x44], k: usize| u32::from_le_bytes([o[k], o[k + 1], o[k + 2], o[k + 3]]);
    let mut o = [0u8; 0x44];
    construir(o.as_mut_ptr());
    emprestimo(o.as_mut_ptr(), jogador, clube);
    let mut c = [0u8; 0x50];
    construir_contrato(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_8 oferta de emprestimo");
    pelo_atual(c.as_mut_ptr(), jogador, clube, fim, percentual, a5, a6); // o salário pela porcentagem
    acrescentar(this, o.as_mut_ptr(), c.as_mut_ptr());
    acrescentar_contrato(cpq8_d(0xae1050), c.as_mut_ptr(), campo(&o, 0x20), campo(&o, 0x24), 0xb, 0, 1, 0);
    let vaga = campo(&o, 0x20);
    destruir(o.as_mut_ptr());
    vaga
}

/// { 0x8cbbe0 } (thiscall, sem argumentos, transfer_manager.cpp): `TRANSFER_MANAGER::process_due_future_transfers()`:
/// percorre as transferências futuras (12 bytes: +0 a data, +8 o índice da oferta; [this+0x920], conta +0x928) até a
/// primeira depois de hoje (0x456740). Oferta nula: a caixa "Error" 0x2846. Com as permissões (0x8c7690 com 1): se as
/// restrições (0x8cfc90) seguram pelo comprador ou, havendo vendedor, pelo vendedor (motivo 9 não conta), desfaz a
/// oferta (0x8c44e0 com o clube e o motivo); senão a notícia (0x8dbef0), conclui (0x8c9390), estado 0x14 e agenda
/// (0x8c6220 com 0xa). Depois tira as processadas do começo do vetor (0x8cbf40 uma a uma) e diminui a conta.
unsafe extern "thiscall" fn concluir_transferencias_futuras(this: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8c7690usize);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(0x8cfc90usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c44e0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8dbef0usize);
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c9390usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    // o motivo das restrições: só a palavra baixa é gravada; os 16 bits altos ficam 0, como na original
    let mut motivo = 0u32;
    let mut feitas = 0i32;
    while feitas < cpq8_d(this + 0x928) as i32 {
        let e = cpq8_d(this + 0x920) + feitas as usize * 0xc;
        if depois(e, cpq8_d(0xae2c90) as u32, cpq8_d(0xae2c94) as u32) != 0 {
            break;
        }
        let oferta = cpq8_d(cpq8_d(this) + cpq8_d(e + 8) * 4);
        if oferta == 0 {
            aviso_em(0xa8d514, 0x2846);
        } else if permissoes(this, oferta, 1) != 0 {
            let comprador = cpq8_clube(cpq8_d(oferta + 8));
            let mut segura = false;
            if restricoes(this, oferta, comprador, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                desfazer(this, oferta, comprador, motivo, 0);
                segura = true;
            } else if cpq8_vendedor(oferta) != 0 {
                let v = cpq8_vendedor(oferta);
                if restricoes(this, oferta, v, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                    let v = cpq8_vendedor(oferta);
                    desfazer(this, oferta, v, motivo, 0);
                    segura = true;
                }
            }
            if !segura {
                noticia(this, oferta);
                crate::comparacao::ramo("cpq_8 transferencia futura");
                concluir(this, oferta); // a transferência acontece
                *((oferta + 0x2c) as *mut u8) = 0x14;
                agendar(this, oferta, 0xa, 0);
            }
        }
        feitas += 1;
    }
    if feitas < 0 {
        aviso_em(0xa8d514, 0x288a);
    }
    if feitas == 0 {
        return;
    }
    let mut k = feitas;
    if k < cpq8_d(this + 0x928) as i32 {
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cbf40usize);
        while k < cpq8_d(this + 0x928) as i32 {
            let v = cpq8_d(this + 0x920);
            copiar(v + (k - feitas) as usize * 0xc, v + k as usize * 0xc);
            k += 1;
        }
    }
    let n = (this + 0x928) as *mut i32;
    core::ptr::write_unaligned(n, core::ptr::read_unaligned(n).wrapping_sub(feitas));
}

/// Comparador: muda o estado, as listas e as marcas, posta notícia e agenda: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_desfazer_oferta(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1066)(this, oferta, clube, motivo, tirar_futura)
}

/// Comparador: acrescenta a oferta e a oferta de contrato (alocam): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_fazer_oferta_de_emprestimo(this: usize, jogador: usize, clube: usize, fim: usize, percentual: u32, a5: u32, a6: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32>(1067)(this, jogador, clube, fim, percentual, a5, a6)
}

/// Comparador: conclui e desfaz transferências (elencos, dinheiro, notícias): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_concluir_transferencias_futuras(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1068)(this)
}

// ---------- 253a leva: os regens: a nota do doador de dados de não jogador, a ordem das aposentadorias e a ordem da fila de regens (player_regen.cpp) ----------

unsafe fn cpq9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq9_b(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpq9_w(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

unsafe fn cpq9_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// { 0x7a7150 } (stdcall, ret 4, player_regen.cpp): `player_regen_score_non_player_candidate(pessoa)`: a nota da pessoa
/// como doadora dos dados de não jogador (pela 0x7a6e60, a cada pessoa não jogadora). −10000 com +0x69, com os bits
/// 0x30 do +0xb do registro de 79 bytes ([0xae2d0c]) ou abaixo de 31 anos (+0x18). Senão sorteia 150 × (idade − 31);
/// com clube (+0x39) de reputação (+0x80) e jogador (+0x61), + 250 × (habilidade atual +0xb ÷ reputação); com
/// jogador, + sorteio(⌊(+0x2f ÷ 2)³⌋) (pow 0x945ff0 e _ftol), +100 com +0xd > 7500 e +50 com +0xb > 7500; os
/// atributos + 5 × (3 × (+0x5d + +0x5a) + 2 × (+0x59 + +0x58) + +0x56) + ⌊+0x5b²⌋ + ⌊+0x57²⌋. Com clube: sorteio da
/// nota (+75 se o clube não tem +0xcf); sem clube: + (+0x22)², e a nota dobra se o +0xd do jogador passa de um
/// sorteio(10000) (lê o jogador sem conferir, como a original). Quente (todas as pessoas a cada chamada): sem contador.
unsafe extern "stdcall" fn nota_do_doador_de_dados(pessoa: usize) -> i32 {
    if cpq9_d(pessoa + 0x69) != 0 {
        return -10000;
    }
    let id = cpq9_d(pessoa);
    if *((cpq9_d(0xae2d0c).wrapping_add(id.wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
        return -10000;
    }
    let idade = cpq9_b(pessoa + 0x18);
    if idade < 31 {
        return -10000;
    }
    let mut nota = sortear((idade - 31) * 150); // a idade
    let clube = cpq9_d(pessoa + 0x39);
    if clube != 0 {
        let reputacao = cpq9_w(clube + 0x80);
        let jogador = cpq9_d(pessoa + 0x61);
        if reputacao != 0 && jogador != 0 {
            nota = nota.wrapping_add((cpq9_w(jogador + 0xb) / reputacao).wrapping_mul(250));
        }
    }
    let jogador = cpq9_d(pessoa + 0x61);
    if jogador != 0 {
        let v = cpq9_b(jogador + 0x2f) / 2;
        nota = nota.wrapping_add(sortear(ftol(pow_do_jogo(v as f64, 3.0))));
        if cpq9_w(jogador + 0xd) > 7500 {
            nota = nota.wrapping_add(100);
        }
        if cpq9_w(jogador + 0xb) > 7500 {
            nota = nota.wrapping_add(50);
        }
    }
    let quadrados = ftol(pow_do_jogo(cpq9_b(pessoa + 0x5b) as f64, 2.0)).wrapping_add(ftol(pow_do_jogo(cpq9_b(pessoa + 0x57) as f64, 2.0)));
    let soma = 3 * (cpq9_b(pessoa + 0x5d) + cpq9_b(pessoa + 0x5a)) + 2 * (cpq9_b(pessoa + 0x59) + cpq9_b(pessoa + 0x58)) + cpq9_b(pessoa + 0x56);
    nota = nota.wrapping_add(soma * 5).wrapping_add(quadrados);
    let clube = cpq9_d(pessoa + 0x39);
    if clube != 0 {
        if cpq9_d(clube + 0xcf) == 0 {
            nota = nota.wrapping_add(75);
        }
        return sortear(nota);
    }
    let b = *((pessoa + 0x22) as *const u8) as i32;
    nota = nota.wrapping_add(b * b);
    let r = sortear(10000); // sem clube
    if cpq9_w(cpq9_d(pessoa + 0x61) + 0xd) > r {
        nota = nota.wrapping_add(nota);
    }
    sortear(nota)
}

/// A data que vale numa entrada da lista de aposentadorias: a do anúncio (+8: dia +0, ano +2) se ainda não passou da
/// data do jogo ([0xae2c90] dia, [0xae2c92] ano: o ano sem sinal, o dia com sinal), senão a da aposentadoria (+0x10).
unsafe fn cpq9_data_da_entrada(e: usize) -> usize {
    let (dia, ano) = (cpq9_w(0xae2c90), cpq9_u(0xae2c92));
    let a = cpq9_u(e + 0xa);
    if a < ano || (a == ano && cpq9_w(e + 8) < dia) {
        e + 0x10
    } else {
        e + 8
    }
}

/// { 0x7ae2a0 } (cdecl, player_regen.cpp): `player_regen_compare_retirements(a, b)`: o qsort da lista de aposentadorias
/// (registros de 0x1a bytes; pela 0x7a59e0 todo dia, que para na primeira entrada futura, e pelas listas iniciais
/// 0x7a5790/0x7a58f0 e pela 0x7a8690): pela data que vale (cpq9_data_da_entrada), o ano sem sinal e o dia com sinal;
/// no empate pelo id (+0). Sem `a`, −1; sem `b`, 1. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_das_aposentadorias(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (da, db) = (cpq9_data_da_entrada(a), cpq9_data_da_entrada(b));
    let (aa, ab) = (cpq9_u(da + 2), cpq9_u(db + 2));
    if aa != ab {
        return aa as i32 - ab as i32; // o ano mais cedo primeiro
    }
    let (ja, jb) = (cpq9_w(da), cpq9_w(db));
    if ja == jb {
        return (cpq9_d(a) as i32).wrapping_sub(cpq9_d(b) as i32);
    }
    ja - jb
}

/// { 0x7ae240 } (cdecl, player_regen.cpp): `player_regen_compare_queue_entries(a, b)`: o qsort da fila de regens
/// (registros de 0xc bytes, no fim da 0x7a59e0 todo dia; a 0x7a65d0 cria os regens das entradas vencidas): pelo ano
/// (+6, sem sinal), depois pelo dia (+4, com sinal), depois pelo id (+0). Sem `a`, −1; sem `b`, 1. Quente (qsort):
/// sem contador.
unsafe extern "C" fn ordem_da_fila_de_regens(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (aa, ab) = (cpq9_u(a + 6), cpq9_u(b + 6));
    if aa > ab {
        return 1; // o ano mais tarde depois
    }
    if aa < ab {
        return -1;
    }
    let (ja, jb) = (cpq9_w(a + 4), cpq9_w(b + 4));
    if ja == jb {
        return (cpq9_d(a) as i32).wrapping_sub(cpq9_d(b) as i32);
    }
    ja - jb
}

/// Comparador: lado a lado pelo estado do sorteio (a cópia sem contar), e a nota.
unsafe extern "stdcall" fn comparar_nota_do_doador_de_dados(pessoa: usize) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> i32>(1069);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa),
        || crate::aleatorio::sem_contar(|| nota_do_doador_de_dados(pessoa)));
    PLACAR.registrar(igual && r == n, || format!("nota_do_doador_de_dados({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_das_aposentadorias(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1070)(a, b);
    let n = ordem_das_aposentadorias(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_aposentadorias({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_da_fila_de_regens(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1071)(a, b);
    let n = ordem_da_fila_de_regens(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_fila_de_regens({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 253a leva: as estatísticas dos jogadores e a aposentadoria marcada: o registro da categoria, criar o registro e se a pessoa está na lista de aposentadorias (player_stats.cpp, player_regen.cpp) ----------

unsafe fn cpq10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A vaga da categoria `categoria` (1..6, byte com sinal) no índice do jogador: o registro de 39 bytes do id dele
/// (+0) em [this+0x30], dword 4 × (categoria − 1). None sem jogador ou fora de 1..6.
unsafe fn cpq10_vaga(this: usize, jogador: usize, categoria: u32) -> Option<usize> {
    let k = (categoria as u8 as i8 as i32).wrapping_sub(1);
    if jogador == 0 || k as u32 > 5 {
        return None;
    }
    Some(cpq10_d(this + 0x30).wrapping_add(cpq10_d(jogador).wrapping_mul(39)) + 4 * k as usize)
}

/// A caixa "Memory Allocation Error" de player_stats.cpp (0xa78310): "v%s %s.%s %d (%s)" (versão 0x90d130, nome e
/// extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
unsafe fn cpq10_sem_memoria(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa78310, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// { 0x7c3920 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::get_category_record(jogador, categoria)`: o
/// registro de 0x20 bytes ([this+4] + 32 × índice) da categoria do jogador (cpq10_vaga), ou 0 quando o índice é −1
/// ou menor. Sem jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10e6 e
/// pela PlayerStats::get_stat 0x7c1e30 (as médias do motor de partida). Quente: sem contador.
unsafe extern "thiscall" fn registro_da_categoria(this: usize, jogador: usize, categoria: u32) -> usize {
    let vaga = match cpq10_vaga(this, jogador, categoria) {
        Some(v) => v,
        _ => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1072)(this, jogador, categoria),
    };
    let i = cpq10_d(vaga) as i32; // o índice da categoria
    if i <= -1 {
        return 0;
    }
    cpq10_d(this + 4).wrapping_add((i as usize) << 5)
}

/// { 0x7c5d80 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::create_stats(jogador, categoria)`: o registro da
/// categoria do jogador (cpq10_vaga); sem índice (−1), com registros soltos ([this+0x38]), o primeiro registro de
/// [this+4] (conta +0xc) com +0xe = 0xffff vira o dele (zerado; os soltos descem); senão um no fim (zerado; a conta
/// sobe): cheio (capacidade +8), cresce de 100 em 100 (realloc 0x945667; sem memória, a caixa 0xef4 e 0). Sem
/// jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10f9.
unsafe extern "thiscall" fn criar_estatisticas(this: usize, jogador: usize, categoria: u32) -> usize {
    let vaga = match cpq10_vaga(this, jogador, categoria) {
        Some(v) => v,
        _ => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1073)(this, jogador, categoria),
    };
    if cpq10_d(vaga) as i32 != -1 {
        return cpq10_d(this + 4).wrapping_add(cpq10_d(vaga) << 5);
    }
    if cpq10_d(this + 0x38) != 0 {
        let n = cpq10_d(this + 0xc) as i32;
        let mut i = 0i32;
        while i < n {
            let r = cpq10_d(this + 4).wrapping_add((i as usize) << 5);
            if core::ptr::read_unaligned((r + 0xe) as *const u16) == 0xffff {
                crate::comparacao::ramo("cpq_10 registro solto");
                core::ptr::write_bytes(r as *mut u8, 0, 0x20);
                cpq10_por(vaga, i as usize);
                cpq10_por(this + 0x38, cpq10_d(this + 0x38).wrapping_sub(1));
                return r;
            }
            i += 1;
        }
    }
    let n = cpq10_d(this + 0xc) as i32;
    if n >= cpq10_d(this + 8) as i32 {
        let cap = cpq10_d(this + 8).wrapping_add(100);
        cpq10_por(this + 8, cap);
        let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpq10_d(this + 4), cap << 5);
        cpq10_por(this + 4, p);
        if p == 0 {
            cpq10_sem_memoria(0xef4, 0xa784fc); // "PLAYER_STATS::create_stats() : Unable to allocate enough memory"
            return 0;
        }
    }
    crate::comparacao::ramo("cpq_10 registro novo");
    cpq10_por(vaga, cpq10_d(this + 0xc)); // o índice novo
    cpq10_por(this + 0xc, cpq10_d(this + 0xc).wrapping_add(1));
    let r = cpq10_d(this + 4).wrapping_add(cpq10_d(vaga) << 5);
    core::ptr::write_bytes(r as *mut u8, 0, 0x20);
    r
}

/// { 0x7ae350 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::is_retirement_pending(pessoa)`: 1 quando o id da
/// pessoa (+0) está na lista de aposentadorias ([this+4], registros de 0x1a bytes, conta +8 com sinal), senão 0 (e 0
/// sem pessoa). Pela MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: quem não é jogador e vai se aposentar não
/// serve para técnico.
unsafe extern "thiscall" fn aposentadoria_marcada(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let n = cpq10_d(this + 8) as i32;
    let id = cpq10_d(pessoa);
    let mut i = 0i32;
    while i < n {
        if cpq10_d(cpq10_d(this + 4) + 0x1a * i as usize) == id {
            crate::comparacao::ramo("cpq_10 aposentadoria marcada");
            return 1; // está na lista
        }
        i += 1;
    }
    0
}

/// Comparador: pelo valor (só lê); os casos de erro só na original.
unsafe extern "thiscall" fn comparar_registro_da_categoria(this: usize, jogador: usize, categoria: u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1072)(this, jogador, categoria);
    if cpq10_vaga(this, jogador, categoria).is_none() {
        return r;
    }
    let n = registro_da_categoria(this, jogador, categoria);
    PLACAR.registrar(r == n, || format!("registro_da_categoria({:x}, {:x}, {}) original={:x} rust={:x}", this, jogador, categoria as u8 as i8, r, n));
    r
}

/// Comparador: a original realoca o vetor, então só ela roda; antes, o plano (o índice que o jogador vai ter, a
/// conta e os soltos depois); depois, o registro devolvido, o índice na vaga, a conta, os soltos e o registro
/// zerado quando é novo. ponytail: os casos de erro e a falta de memória só na original, sem conferir.
unsafe extern "thiscall" fn comparar_criar_estatisticas(this: usize, jogador: usize, categoria: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1073);
    let vaga = match cpq10_vaga(this, jogador, categoria) {
        Some(v) => v,
        _ => return o(this, jogador, categoria),
    };
    let atual = cpq10_d(vaga) as i32;
    let (n, soltos) = (cpq10_d(this + 0xc), cpq10_d(this + 0x38));
    let solto = if atual == -1 && soltos != 0 {
        (0..n as i32).find(|&i| core::ptr::read_unaligned((cpq10_d(this + 4) + ((i as usize) << 5) + 0xe) as *const u16) == 0xffff)
    } else {
        None
    };
    let (indice, n2, soltos2) = match (atual, solto) {
        (-1, Some(i)) => (i, n, soltos.wrapping_sub(1)),
        (-1, None) => (n as i32, n.wrapping_add(1), soltos),
        (i, _) => (i, n, soltos),
    };
    let r = o(this, jogador, categoria);
    if r == 0 {
        return r;
    }
    let esperado = cpq10_d(this + 4).wrapping_add((indice as usize) << 5);
    let zerado = atual != -1 || (0..0x20).all(|k| *((r + k) as *const u8) == 0);
    let igual = r == esperado && cpq10_d(vaga) as i32 == indice && cpq10_d(this + 0xc) == n2 && cpq10_d(this + 0x38) == soltos2 && zerado;
    PLACAR.registrar(igual, || format!("criar_estatisticas({:x}, {:x}, {}) original={:x} plano={:x} índice {} x {} conta {} x {} soltos {} x {} zerado={}",
        this, jogador, categoria as u8 as i8, r, esperado, cpq10_d(vaga) as i32, indice, cpq10_d(this + 0xc), n2, cpq10_d(this + 0x38), soltos2, zerado));
    r
}

comparar_valor!(comparar_aposentadoria_marcada, aposentadoria_marcada, 1074, (pessoa: usize) -> u8);

// ---------- 253a leva: a copa: o sorteio dos times da rodada, os confrontos no estádio compartilhado e a escala pela rodada (Cup.cpp, comp_util.cpp) ----------

unsafe fn cpq11_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpq11_us(a: usize) -> usize {
    cpq11_d(a) as usize
}

unsafe fn cpq11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq11_por_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// `base + i × tam` (o índice com sinal, como o movsx do original).
fn cpq11_em(base: usize, i: i32, tam: usize) -> usize {
    base.wrapping_add((i as isize).wrapping_mul(tam as isize) as usize)
}

/// O confronto `k` (0x35 bytes) da rodada `r` da copa: [[copa + 0xad] + 4r] + 0x35k.
unsafe fn cpq11_confronto(copa: usize, r: i32, k: i32) -> usize {
    cpq11_em(cpq11_us(cpq11_em(cpq11_us(copa + 0xad), r, 4)), k, 0x35)
}

/// A entrada da rodada `r` (0x68 bytes) em [copa + 0xa3]: +0x18 vagas de times, +0x1a confrontos.
unsafe fn cpq11_rodada(copa: usize, r: i32) -> usize {
    cpq11_em(cpq11_us(copa + 0xa3), r, 0x68)
}

/// O clube do time `t` da copa (6 bytes em [copa + 0xa7]: +0 clube, +4 nível).
unsafe fn cpq11_time(copa: usize, t: i16) -> usize {
    cpq11_em(cpq11_us(copa + 0xa7), t as i32, 6)
}

/// Os confrontos de todas as rodadas da copa (rodadas em +0x3c), para os comparadores.
unsafe fn cpq11_confrontos(copa: usize) -> Vec<(usize, usize)> {
    let mut v = vec![(0xb67a34, 4)];
    for r in 0..(cpq11_w(copa + 0x3c) as i32).max(0) {
        let f = cpq11_us(cpq11_em(cpq11_us(copa + 0xad), r, 4));
        let n = cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32;
        if f != 0 && n > 0 {
            v.push((f, n as usize * 0x35));
        }
    }
    v
}

/// A caixa "Memory Allocation Error" ("v%s %s.%s %d (%s)" de Cup.cpp na linha `linha`) e o jogo para (como a original).
unsafe fn cpq11_sem_memoria(linha: u32, texto: usize) -> ! {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b55ac, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
    *(0xb67a34 as *mut u32) = 0;
    fechar1();
    fechar2();
    sair(-1)
}

/// { 0x51e220 } (thiscall, ret 4, Cup.cpp): `Cup::randomize_round_teams(rodada)`: o sorteio dos times da rodada.
/// 1) Tira os times dos confrontos (+0x1a da rodada) para um vetor de 7 bytes por vaga (malloc de 7 × vagas +0x18;
///    sem memória, caixa da linha 0xe7f e o jogo para): o mandante (+0, com o vínculo +0xe/+0xa) e o visitante (+2,
///    com +0x10/+0xc), cada um com o nível do time (+4 do time); a vaga do confronto vira −2 (livre); sem time (−1),
///    a entrada fica −1 (com 0xff no nível).
/// 2) Embaralha: para cada posição, troca com a entrada sorteada entre todas (0x90cfc0(n)).
/// 3) Cada entrada com time vai a um confronto sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira
///    volta só confrontos com as duas vagas vazias (< 0); depois qualquer um com vaga livre (−2); dando a volta
///    inteira na segunda, caixa da linha 0xf4c e sai (a marca da segunda volta não volta a zero). Com as duas vagas
///    livres, sorteia o lado (0x90cfc0(2): ≠ 0 mandante). O time leva o vínculo (+0xe/+0xa ou +0x10/+0xc) e, com
///    vínculo, o confronto da rodada seguinte ganha a rodada (+0x12), o confronto sorteado (+6) e o lado (+8: 1
///    mandante, 0 visitante).
/// Libera o vetor (0x9452ca).
unsafe extern "thiscall" fn copa_sortear_times_da_rodada(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let vagas = cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32;
    let vetor = alocar(vagas.wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xe7f, 0x9b574c); // "CUP::randomize_round_teams() : Cannot allocate memory"
    }
    let entrada = |i: i16| cpq11_em(vetor, i as i32, 7);
    // 1) os times saem dos confrontos
    let mut n: i16 = 0;
    let mut k: i32 = 0;
    while (k as i16) < cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = cpq11_confronto(copa, r, k as i16 as i32);
            let e = entrada(n);
            n = n.wrapping_add(1);
            let time = cpq11_w(t + lado);
            if time != -1 {
                cpq11_por_w(e, time as u16);
                cpq11_por_w(e + 2, cpq11_w(t + ligado) as u16);
                cpq11_por_w(e + 4, cpq11_w(t + jogo) as u16);
                *((e + 6) as *mut u8) = *((cpq11_time(copa, time) + 4) as *const u8); // o nível do time
                cpq11_por_w(t + lado, 0xfffe); // a vaga fica livre
            } else {
                cpq11_por_w(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    // 2) o embaralhamento
    if n > 0 {
        let mut p = vetor;
        for _ in 0..n as i32 {
            let q = entrada(sorteio(n as i32) as i16);
            let (a, b, c) = (cpq11_d(q), cpq11_w(q + 4), *((q + 6) as *const u8));
            core::ptr::write_unaligned(q as *mut u32, cpq11_d(p));
            cpq11_por_w(q + 4, cpq11_w(p + 4) as u16);
            *((q + 6) as *mut u8) = *((p + 6) as *const u8);
            core::ptr::write_unaligned(p as *mut u32, a);
            cpq11_por_w(p + 4, b as u16);
            *((p + 6) as *mut u8) = c;
            p += 7;
        }
    }
    // 3) cada time vai a um confronto sorteado
    let mut segunda_volta = false;
    let mut i: i16 = 0;
    while i < n {
        let e = entrada(i);
        i = i.wrapping_add(1);
        if cpq11_w(e) == -1 {
            continue;
        }
        let mut s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32) as i16;
        let inicio = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, s as i32);
            let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
            if segunda_volta || (casa < 0 && fora < 0) {
                let casa_livre = casa == -2;
                if fora == -2 {
                    if casa_livre && sorteio(2) != 0 {
                        crate::comparacao::ramo("cpq_11 sorteio mandante");
                        break true;
                    }
                    break false;
                }
                if casa_livre {
                    break true;
                }
            }
            s = s.wrapping_add(1);
            if s >= cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
                s = 0;
            }
            if s == inicio {
                if segunda_volta {
                    aviso_em(0x9b55ac, 0xf4c);
                    liberar(vetor);
                    return;
                }
                segunda_volta = true;
            }
        };
        let t = cpq11_confronto(copa, r, s as i32);
        let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
        cpq11_por_w(t + lado, cpq11_w(e) as u16);
        cpq11_por_w(t + ligado, cpq11_w(e + 2) as u16);
        cpq11_por_w(t + jogo, cpq11_w(e + 4) as u16);
        let seguinte = cpq11_w(e + 2);
        if seguinte != -1 {
            let v = cpq11_confronto(copa, seguinte as i32, cpq11_w(e + 4) as i32);
            cpq11_por_w(v + 0x12, rodada as u16);
            cpq11_por_w(v + 6, s as u16);
            cpq11_por_w(v + 8, mandante as u16);
        }
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
unsafe extern "thiscall" fn comparar_copa_sortear_times_da_rodada(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1075);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| copa_sortear_times_da_rodada(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_sortear_times_da_rodada(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Os pares de palavras do confronto `t` que a 0x522610 troca: mandante/visitante (+0/+2), +0xa/+0xc e +0xe/+0x10.
unsafe fn cpq11_inverter(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xe, 0x10), (0xa, 0xc)] {
        let (x, y) = (cpq11_w(t + a), cpq11_w(t + b));
        cpq11_por_w(t + a, y as u16);
        cpq11_por_w(t + b, x as u16);
    }
}

/// { 0x522610 } (thiscall, ret 4, Cup.cpp): `Cup::separate_shared_stadium_ties(rodada)`: para cada confronto `i` da
/// rodada, procura o primeiro `j > i` cujo mandante (ou, senão, o visitante) divide o estádio (+0x69 do clube, não
/// nulo) com o mandante (ou o visitante) de `i`; achando, inverte o confronto `j` (mandante e visitante, +0xa/+0xc e
/// +0xe/+0x10) e acerta o lado (+8) dos confrontos ligados na rodada seguinte: o de +0xe/+0xa fica 1 (mandante), o de
/// +0x10/+0xc fica 0. Os dois jogos no mesmo estádio deixam de cair no mesmo lado.
unsafe extern "thiscall" fn copa_separar_estadio_compartilhado(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let estadio = |t: i16| cpq11_d(cpq11_us(cpq11_time(copa, t)) as usize + 0x69);
    let mut n = cpq11_w(cpq11_rodada(copa, r) + 0x1a);
    let mut i: i32 = 0;
    while (i as i16) < n {
        let fi = cpq11_confronto(copa, r, i as i16 as i32);
        let casa_i = cpq11_w(fi);
        let mut j = i + 1;
        while (j as i16) < n {
            let fj = cpq11_confronto(copa, r, j as i16 as i32);
            let mut dividem = false;
            for lado in [0usize, 2] {
                let a = if lado == 0 { casa_i } else { cpq11_w(fi + 2) };
                let b = cpq11_w(fj + lado);
                if a != -1 && b != -1 {
                    crate::comparacao::ramo("cpq_11 estadio comparado");
                    let (sa, sb) = (estadio(a), estadio(b));
                    if sa == sb && sa != 0 {
                        dividem = true;
                        break;
                    }
                }
            }
            if dividem {
                cpq11_inverter(fj);
                let seguinte = cpq11_w(fj + 0xe);
                if seguinte != -1 {
                    cpq11_por_w(cpq11_confronto(copa, seguinte as i32, cpq11_w(fj + 0xa) as i32) + 8, 1);
                }
                let seguinte = cpq11_w(fj + 0x10);
                if seguinte != -1 {
                    cpq11_por_w(cpq11_confronto(copa, seguinte as i32, cpq11_w(fj + 0xc) as i32) + 8, 0);
                }
                break;
            }
            j += 1;
        }
        n = cpq11_w(cpq11_rodada(copa, r) + 0x1a);
        i += 1;
    }
}

/// Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
unsafe extern "thiscall" fn comparar_copa_separar_estadio_compartilhado(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1076);
    let reg = cpq11_confrontos(copa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada), || copa_separar_estadio_compartilhado(copa, rodada));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_separar_estadio_compartilhado(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// A conta da 0x4bf850 sem efeitos: o eax da original e o byte devolvido. Com uma rodada só, (baixo + alto) / 2
/// (truncada para o zero) sobre a soma; senão o quociente (alto − baixo) × rodada / (rodadas − 1), sem somar o baixo.
fn cpq11_escala(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> (u32, i8) {
    let (b, a) = (baixo as u8 as i8 as i32, alto as u8 as i8 as i32);
    let n = rodadas as u16 as i16 as i32;
    if n == 1 {
        let s = b.wrapping_add(a);
        let t = s.wrapping_sub(s >> 31); // cdq; sub eax, edx
        let v = (t >> 1) as i8;
        ((t as u32 & 0xffff_ff00) | v as u8 as u32, v)
    } else {
        let q = a.wrapping_sub(b).wrapping_mul(rodada as u16 as i16 as i32).wrapping_div(n - 1); // a média por rodada
        (q as u32, q as i8)
    }
}

/// { 0x4bf850 } (cdecl, comp_util.cpp): `comp_util_scale_by_round(baixo, alto, rodada, rodadas)` (bytes e palavras com
/// sinal; as palavras chegam em registradores com lixo acima, só a palavra é lida): com uma rodada só, a média
/// (baixo + alto) / 2; senão (alto − baixo) × rodada / (rodadas − 1). Fora de 0..=100: a original (caixa da linha
/// 0x256e, devolve 0 ou 100). Devolve o eax inteiro da original (quem chama empurra o registrador todo para
/// 0x86bdd0/0x86c000): o byte em al sobre a soma (rodada única) ou o quociente.
unsafe extern "C" fn escala_pela_rodada(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let (eax, v) = cpq11_escala(baixo, alto, rodada, rodadas);
    if !(0..=100).contains(&v) {
        return orig::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(1077)(baixo, alto, rodada, rodadas);
    }
    if rodadas as u16 != 1 {
        crate::comparacao::ramo("cpq_11 escala pela rodada");
    }
    eax
}

/// Comparador: pura (pelo eax inteiro). Fora da faixa (a caixa de erro) só a original.
unsafe extern "C" fn comparar_escala_pela_rodada(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(1077);
    if !(0..=100).contains(&cpq11_escala(baixo, alto, rodada, rodadas).1) {
        return o(baixo, alto, rodada, rodadas);
    }
    let r = o(baixo, alto, rodada, rodadas);
    let n = escala_pela_rodada(baixo, alto, rodada, rodadas);
    PLACAR.registrar(r == n, || format!("escala_pela_rodada({}, {}, {}, {}) original={:x} rust={:x}",
        baixo as u8 as i8, alto as u8 as i8, rodada as u16 as i16, rodadas as u16 as i16, r, n));
    r
}
// ---------- 253a leva: o lado do jogo, a ordem dos coeficientes das nações e o pedido de observação (comp_util.cpp, scout_manager.cpp) ----------

unsafe fn cpq12_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpq12_us(a: usize) -> usize {
    cpq12_d(a) as usize
}

/// { 0x4afa80 } (cdecl, comp_util.cpp): `comp_util_get_fixture_venue_side(clube, jogo)`: o lado do clube no jogo:
/// 1 em casa, 2 fora, 3 neutro. Sem campo no jogo (+0x18): pelo mandante (+0x1c), 3 se ele não tem estádio (+0x69),
/// senão 1 para o mandante e 2 para o outro. Seleção (0x53b900): 1 se a nação do campo (+0x38, +0x1f) é a do clube
/// (+0x53), 2 se é a do adversário, senão 3. Clube: o campo é o estádio do clube e não o do adversário, 1; dos dois,
/// 1 para o mandante e 2 para o outro; só do adversário, 2; de nenhum, 3. Quem chama lê só o al.
unsafe extern "C" fn lado_do_jogo_para_o_clube(clube: usize, jogo: usize) -> u8 {
    let campo = cpq12_d(jogo + 0x18);
    let casa = cpq12_us(jogo + 0x1c);
    if campo == 0 {
        if cpq12_d(casa + 0x69) == 0 {
            return 3;
        }
        return if clube != casa { 2 } else { 1 };
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        let nacao = cpq12_us(campo as usize + 0x38);
        if nacao != 0 && cpq12_d(nacao + 0x1f) == cpq12_d(clube + 0x53) {
            return 1;
        }
        let casa = cpq12_us(jogo + 0x1c);
        let outro = if casa == clube { cpq12_us(jogo + 0x20) } else { casa };
        if nacao != 0 && cpq12_d(nacao + 0x1f) == cpq12_d(outro + 0x53) {
            return 2;
        }
        return 3;
    }
    let casa = cpq12_us(jogo + 0x1c);
    let outro = if casa == clube { cpq12_us(jogo + 0x20) } else { casa };
    let campo = cpq12_d(jogo + 0x18);
    if campo != cpq12_d(clube + 0x69) {
        return if campo != cpq12_d(outro + 0x69) { 3 } else { 2 };
    }
    if campo == cpq12_d(outro + 0x69) {
        return if clube != casa { 2 } else { 1 }; // o mesmo estádio dos dois
    }
    crate::comparacao::ramo("cpq_12 em casa");
    1 // o estádio do clube
}

/// Comparador: pura (consulta 0x53b900, sem efeitos): pelo lado.
unsafe extern "C" fn comparar_lado_do_jogo_para_o_clube(clube: usize, jogo: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u8>(1078);
    let r = o(clube, jogo);
    let n = lado_do_jogo_para_o_clube(clube, jogo);
    PLACAR.registrar(r == n, || format!("lado_do_jogo_para_o_clube(clube {:x}, jogo {:x}) original={} rust={}", clube, jogo, r, n));
    r
}

/// { 0x4afba0 } (cdecl, comp_util.cpp): `compare_nation_coefficients_desc(a, b)`: a ordem do qsort (12 bytes por
/// entrada, o coeficiente em double em +4) da game_update_nation_coefficients 0x5d6187 (no dia do mês da tabela
/// 0x9cd200): maior antes (−1); menor ou sem ordem (NaN, a flag C0 do fcomp), 1; igual, 0. Quente (o qsort): sem
/// contador. A original ordena e libera o vetor logo depois: a ordem não chega ao jogo (só o comparando acusa).
unsafe extern "C" fn ordem_dos_coeficientes_das_nacoes(a: usize, b: usize) -> i32 {
    let x = core::ptr::read_unaligned((a + 4) as *const f64);
    let y = core::ptr::read_unaligned((b + 4) as *const f64);
    if x > y {
        return -1; // o maior antes
    }
    if !(x >= y) {
        return 1;
    }
    0
}

unsafe extern "C" fn comparar_ordem_dos_coeficientes_das_nacoes(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1079);
    let r = o(a, b);
    let n = ordem_dos_coeficientes_das_nacoes(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_coeficientes_das_nacoes original={} rust={}", r, n));
    r
}

/// { 0x7f8d70 } (thiscall, ret 0xc, scout_manager.cpp): `SCOUT_MANAGER::has_scout_request(jogador, clube, so_vencidos)`:
/// se o clube (nulo: o do humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]] +0x39) pediu a observação do jogador: busca
/// binária (0x946464 com 0x7fc270) pelo par (id do clube, id do jogador) nos pedidos (+0x5c, conta +0x64, 17 bytes)
/// com menos de 3 relatórios (+0x10, com sinal); com `so_vencidos`, só se a data do pedido (+8: dia, +0xa: ano) é
/// anterior a hoje ([0xae2c90], ano sem sinal, dia com sinal). Devolve 1 ou 0. Jogador nulo: a original (caixa da
/// linha 0x5da). A data local da original (0x54a110) só ocupa a chave depois do par.
unsafe extern "thiscall" fn pedido_de_observacao(this: usize, jogador: usize, clube: usize, so_vencidos: u32) -> u32 {
    if jogador == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1080)(this, jogador, clube, so_vencidos);
    }
    let mut clube = clube;
    if clube == 0 {
        let humano = cpq12_us(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize));
        if humano == 0 {
            return 0;
        }
        clube = cpq12_us(humano + 0x39);
        if clube == 0 {
            return 0;
        }
    }
    let lista = cpq12_us(this + 0x5c);
    let achado = if lista == 0 {
        0
    } else {
        let data: unsafe extern "thiscall" fn(*mut u8) -> *mut u8 = core::mem::transmute(0x54a110usize);
        let buscar: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        let mut chave = [0u8; 16];
        data(chave.as_mut_ptr().add(8));
        chave[4..8].copy_from_slice(&cpq12_d(jogador).to_le_bytes());
        chave[0..4].copy_from_slice(&cpq12_d(clube).to_le_bytes());
        buscar(chave.as_ptr(), lista, cpq12_d(this + 0x64), 0x11, 0x7fc270)
    };
    if achado == 0 {
        crate::comparacao::ramo("cpq_12 sem pedido");
        return 0; // sem pedido
    }
    if *((achado + 0x10) as *const i8) >= 3 {
        return 0;
    }
    if so_vencidos != 0 {
        let ano = core::ptr::read_unaligned((achado + 0xa) as *const u16);
        let hoje = *(0xae2c92 as *const u16);
        if ano > hoje {
            return 0;
        }
        if ano == hoje && core::ptr::read_unaligned((achado + 8) as *const i16) >= *(0xae2c90 as *const i16) {
            return 0;
        }
    }
    1
}

/// Comparador: consulta pura (a busca binária e a data local): pelo valor; jogador nulo só a original (a caixa).
unsafe extern "thiscall" fn comparar_pedido_de_observacao(this: usize, jogador: usize, clube: usize, so_vencidos: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1080);
    if jogador == 0 {
        return o(this, jogador, clube, so_vencidos);
    }
    let r = o(this, jogador, clube, so_vencidos);
    let n = pedido_de_observacao(this, jogador, clube, so_vencidos);
    PLACAR.registrar(r == n, || format!("pedido_de_observacao(jogador {:x}, clube {:x}, {}) original={} rust={}", jogador, clube, so_vencidos, r, n));
    r
}

// ---------- 253a leva: a escalação guardada: limpar uma vaga e o batedor de bola parada (tactics.cpp) ----------

unsafe fn cpq13_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// A linha do clube na tabela tática (0x8982b0, o `this` não é usado; −1 sem linha).
unsafe fn cpq13_linha(clube: usize, time: u32) -> i32 {
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    linha(clube, time)
}

/// A célula `c` (dword) da linha `k` da tabela tática: [this + 0x601] + 0x91 × k + `c`.
unsafe fn cpq13_celula(this: usize, k: i32, c: usize) -> usize {
    (cpq13_d(this + 0x601) as usize).wrapping_add((k as isize * 0x91) as usize).wrapping_add(c)
}

/// { 0x896e90 } (thiscall, ret 0xc, tactics.cpp): `Tactics::clear_selection_slot(clube, vaga, time)`: a vaga (0..=19)
/// da escalação do clube na tabela tática (linha da 0x8982b0, dword em +2 + 4 × vaga) fica −1 e, com espaços guardados
/// (+0x5f0), os do mesmo clube e time (16 espaços de 0x5c bytes a partir de +0x30: clube +0x30, time +0x34, ocupado
/// +0x38) são soltos (+0x38 = 0, +0x5f0 − 1). Devolve 1; sem linha, 0. Clube nulo ou vaga fora: a original (caixa).
unsafe extern "thiscall" fn limpar_vaga_da_escalacao(this: usize, clube: usize, vaga: u32, time: u32) -> u32 {
    let v = vaga as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1081)(this, clube, vaga, time);
    }
    let k = cpq13_linha(clube, time);
    if k == -1 {
        return 0;
    }
    crate::comparacao::ramo("cpq_13 limpar vaga");
    core::ptr::write_unaligned(cpq13_celula(this, k, 2 + 4 * v as usize) as *mut i32, -1); // a vaga
    if *((this + 0x5f0) as *const u8) != 0 {
        for e in 0..16 {
            let a = this + 0x30 + 0x5c * e;
            if cpq13_d(a + 8) != 0 && cpq13_d(a) as usize == clube && cpq13_d(a + 4) == time {
                core::ptr::write_unaligned((a + 8) as *mut u32, 0);
                *((this + 0x5f0) as *mut u8) = (*((this + 0x5f0) as *const u8)).wrapping_sub(1);
            }
        }
    }
    1
}

/// Comparador: só grava a vaga e os espaços guardados: lado a lado pela linha da escalação (20 dwords), pelos 16
/// espaços e pela conta (+0x5f0). Clube nulo ou vaga fora: só a original (a caixa).
unsafe extern "thiscall" fn comparar_limpar_vaga_da_escalacao(this: usize, clube: usize, vaga: u32, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1081);
    let v = vaga as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return o(this, clube, vaga, time);
    }
    let mut reg = vec![(this + 0x30, 16 * 0x5c), (this + 0x5f0, 1)];
    let k = cpq13_linha(clube, time);
    if k != -1 {
        reg.push((cpq13_celula(this, k, 2), 20 * 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, vaga, time), || limpar_vaga_da_escalacao(this, clube, vaga, time));
    PLACAR.registrar(igual && r == n, || format!("limpar_vaga_da_escalacao(vaga {}, time {}) original={} rust={} {:x?} x {:x?}", v, time, r, n, mo, mr));
    r
}

/// { 0x897bb0 } (thiscall, ret 0x14, tactics.cpp): `Tactics::set_set_piece_taker(clube, papel, escolha, pessoa, time)`:
/// o batedor `escolha` (0..=1) da bola parada `papel` (0..=4) na linha tática do clube (0x8982b0; dword em +0x5e +
/// 4 × (escolha + 2 × papel)). Com `pessoa` (ponteiro para o id), grava o id. Sem ela, a célula fica −1 e, sendo a
/// primeira escolha, a seguinte com batedor (> −1) sobe para ela e fica −1. Devolve 1; sem linha, 0. Clube nulo,
/// papel ou escolha fora: a original (caixa).
unsafe extern "thiscall" fn marcar_batedor_de_bola_parada(this: usize, clube: usize, papel: u32, escolha: u32, pessoa: usize, time: u32) -> u32 {
    let (p, e) = (papel as u8 as i8, escolha as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(1082)(this, clube, papel, escolha, pessoa, time);
    }
    let k = cpq13_linha(clube, time);
    if k == -1 {
        return 0;
    }
    let celula = |j: i8| cpq13_celula(this, k, 0x5e + 4 * (j as i32 + 2 * p as i32) as usize) as *mut i32;
    if pessoa != 0 {
        core::ptr::write_unaligned(celula(e), cpq13_d(pessoa) as i32);
        return 1;
    }
    crate::comparacao::ramo("cpq_13 batedor limpo");
    core::ptr::write_unaligned(celula(e), -1); // sem batedor
    if e < 1 {
        let mut j = e + 1;
        while j < 2 {
            let c = core::ptr::read_unaligned(celula(j));
            if c > -1 {
                crate::comparacao::ramo("cpq_13 batedor sobe");
                core::ptr::write_unaligned(celula(e), c);
                core::ptr::write_unaligned(celula(j), -1);
                return 1;
            }
            j += 1;
        }
    }
    1
}

/// Comparador: só grava a linha dos batedores: lado a lado pelos 10 dwords (+0x5e) da linha do clube. Clube nulo,
/// papel ou escolha fora: só a original (a caixa).
unsafe extern "thiscall" fn comparar_marcar_batedor_de_bola_parada(this: usize, clube: usize, papel: u32, escolha: u32, pessoa: usize, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(1082);
    let (p, e) = (papel as u8 as i8, escolha as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return o(this, clube, papel, escolha, pessoa, time);
    }
    let k = cpq13_linha(clube, time);
    let reg = if k != -1 { vec![(cpq13_celula(this, k, 0x5e), 10 * 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, papel, escolha, pessoa, time),
        || marcar_batedor_de_bola_parada(this, clube, papel, escolha, pessoa, time));
    PLACAR.registrar(igual && r == n, || format!("marcar_batedor_de_bola_parada(papel {}, escolha {}, pessoa {:x}) original={} rust={} {:x?} x {:x?}", p, e, pessoa, r, n, mo, mr));
    r
}
