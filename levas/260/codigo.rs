// ---------- 260a leva: as faltas do dia: o dia das ausências, das faltas ao treino e dos atrasos, a falta ao treino marcada, o atraso registrado, marcar a falta, o fim da ausência e as marcas do contrato (awol.cpp, staff_contracts.cpp) ----------

unsafe fn cpw7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw7_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpw7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O byte com sinal `a` (atributo de 1 a 20) cai um com o sorteio dele: zero (`zero`) ou não zero, se for maior que 1.
unsafe fn cpw7_cair(a: usize, zero: bool) {
    let v = cpw7_b(a) as i8;
    if v > 1 && (cpw7_sortear(v as i32) == 0) == zero {
        *(a as *mut i8) = v - 1;
    }
}

unsafe fn cpw7_proximo(lista: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(lista)
}

/// O cursor da lista volta ao primeiro nó (+0xa = +2).
unsafe fn cpw7_inicio(lista: usize) {
    cpw7_pd(lista + 0xa, cpw7_d(lista + 2) as u32);
}

/// A data `d` (dia com sinal em +0, ano sem sinal em +2) já passou: hoje ([0xae2c90]) é depois dela.
unsafe fn cpw7_passou(d: usize) -> bool {
    let (dia, ano) = (cpw7_w(0xae2c90), cpw7_w(0xae2c92) as u16);
    let a = cpw7_w(d + 2) as u16;
    ano > a || (ano == a && dia > cpw7_w(d))
}

/// Uma semana depois da data `d` (0x54c770), já passou?
unsafe fn cpw7_semana_passou(d: usize, semanas: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let mut t = [0u8; 8];
    cpw7_passou(f(d, &mut t, semanas))
}

/// O fim do caso de falta ao treino `r` (pessoa em +0): com o byte da pessoa na tabela de marcas ([this+0xc]) não zero,
/// troca o bit 2 dele (o `or 2` da original nunca dá zero) e tira o registro da lista das faltas ([this+4], 0x68d1f0).
unsafe fn cpw7_tirar_falta(this: usize, r: usize) {
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m ^= 2;
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    crate::comparacao::ramo("cpw_7 falta tirada da lista");
    tirar(cpw7_d(this + 4), r); // cpw7: tira o caso da lista
}

unsafe fn cpw7_faltou(this: usize, pessoa: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x41a360usize);
    f(this, pessoa);
}

/// { 0x4190d0 } (thiscall, awol.cpp): `AWOL::daily_update()`, todo dia pelo laço do jogo (0x5cccd0). Três listas com o
/// cursor em +0xa: (1) ausências ([this]): a primeira cuja volta (data em +4) já passou acaba (0x419710); (2) faltas ao
/// treino ([this+4]): a primeira com uma semana passada da data (+4) é resolvida: caso repetido (byte +0xc) com dois
/// sorteios zero pelo profissionalismo (+0x5b) e pelo temperamento (+0x5d) da pessoa, os dois caem pelo sorteio e ela
/// falta de novo (0x41a360); caso novo: os colegas do clube (50 vagas em +0xd7) com o contrato do faltoso podem pedir a
/// saída (0x867560 com 2) ou perder profissionalismo, moral (+0x44 do jogador) e temperamento; e o faltoso, por
/// sorteio(profissionalismo) abaixo de sorteio(15) + 1, perde profissionalismo e moral, falta de novo e briga com o
/// clube (0x72c1f0 com 8); (3) atrasos ([this+8]): o primeiro com uma semana passada (+8) que vença os sorteios perde
/// profissionalismo e temperamento ou moral. Cada caso resolvido sai da lista (0x68d1f0) e a marca do bit 2 troca. Uma
/// lista que acaba antes da conta encerra o dia.
unsafe extern "thiscall" fn faltas_do_dia(this: usize) {
    // (1) as ausências
    let l = cpw7_d(this);
    if l != 0 {
        cpw7_inicio(l);
        if cpw7_w(l) > 0 {
            let mut i = 0;
            loop {
                let r = cpw7_proximo(cpw7_d(this));
                if r == 0 {
                    return;
                }
                if cpw7_passou(r + 4) {
                    let fim: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x419710usize);
                    fim(this, r);
                    break;
                }
                i += 1;
                if i >= cpw7_w(cpw7_d(this)) as i32 {
                    break;
                }
            }
        }
    }
    // (2) as faltas ao treino
    let l = cpw7_d(this + 4);
    if l != 0 {
        cpw7_inicio(l);
        if cpw7_w(l) > 0 {
            let mut i = 0;
            loop {
                let r = cpw7_proximo(cpw7_d(this + 4));
                if r == 0 {
                    return;
                }
                if cpw7_semana_passou(r + 4, 1) { // cpw7: uma semana depois
                    crate::comparacao::ramo("cpw_7 falta vencida");
                    cpw7_resolver_falta(this, r);
                    break;
                }
                i += 1;
                if i >= cpw7_w(cpw7_d(this + 4)) as i32 {
                    break;
                }
            }
        }
    }
    // (3) os atrasos
    let l = cpw7_d(this + 8);
    if l == 0 {
        return;
    }
    cpw7_inicio(l);
    if cpw7_w(l) <= 0 {
        return;
    }
    let mut i = 0;
    loop {
        let r = cpw7_proximo(cpw7_d(this + 8));
        if r == 0 {
            return;
        }
        if cpw7_semana_passou(r + 8, 1) {
            let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
            let resolveu = if cpw7_b(r + 0x10) != 0 {
                if cpw7_d(pessoa + 0x61) == 0 {
                    true
                } else if cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(cpw7_b(pessoa + 0x5d) as i8 as i32) == 0 {
                    cpw7_cair(pessoa + 0x5b, true);
                    cpw7_cair(pessoa + 0x5d, true);
                    true
                } else {
                    false
                }
            } else {
                let a = cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32);
                let b = cpw7_sortear(5);
                if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
                    cpw7_cair(pessoa + 0x5b, true);
                    let j = cpw7_d(pessoa + 0x61);
                    if j != 0 {
                        cpw7_cair(j + 0x44, true);
                    }
                    true
                } else {
                    false
                }
            };
            if resolveu {
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                tirar(cpw7_d(this + 8), r);
                return;
            }
        }
        i += 1;
        if i >= cpw7_w(cpw7_d(this + 8)) as i32 {
            return;
        }
    }
}

/// A falta ao treino `r` com uma semana passada (o meio da lista (2) de `faltas_do_dia`).
unsafe fn cpw7_resolver_falta(this: usize, r: usize) {
    let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
    if cpw7_b(r + 0xc) != 0 {
        // o caso repetido
        if cpw7_d(pessoa + 0x61) != 0 {
            crate::comparacao::ramo("cpw_7 caso repetido");
            if cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(cpw7_b(pessoa + 0x5d) as i8 as i32) == 0 { // cpw7: os sorteios do caso repetido
                cpw7_cair(pessoa + 0x5b, true);
                cpw7_cair(pessoa + 0x5d, true);
                cpw7_tirar_falta(this, r);
                cpw7_faltou(this, pessoa);
                return;
            }
        }
        return cpw7_tirar_falta(this, r);
    }
    // o caso novo: os colegas do clube
    crate::comparacao::ramo("cpw_7 caso novo");
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contrato_de(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k as u32);
    let clube = if k != 0 { cpw7_d(0xae23b4).wrapping_add(cpw7_d(k + 4).wrapping_mul(0x245)) } else { 0 };
    if clube != 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
        let pedir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867560usize);
        for v in 0..0x32usize {
            let s = cpw7_d(clube + 0xd7 + v * 4);
            if s == 0 || s == pessoa || cpw7_d(s + 0x61) == 0 {
                continue;
            }
            let ct = contrato(cpw7_d(0xae1050), pessoa, clube);
            if ct == 0 {
                continue;
            }
            if marcas(ct) & 2 == 0 && cpw7_b(s + 0x5b) as i8 > 0xf && cpw7_b(s + 0x57) as i8 > 0xa {
                let a = cpw7_sortear(cpw7_b(s + 0x5b) as i8 as i32);
                let b = cpw7_sortear(0x1e);
                let f = cpw7_b(ct + 0x4f) & 0xf0;
                if a > b && (f == 0x10 || f == 0x20) && cpw7_b(s + 0x18) as i8 > 0x17 && cpw7_w(cpw7_d(s + 0x61) + 0xb) > 0x1482 {
                    pedir(ct, 2);
                }
            }
            if cpw7_b(s + 0x18) as i8 >= 0x15 || cpw7_w(cpw7_d(s + 0x61) + 0xb) <= 0x1482 {
                continue;
            }
            cpw7_cair(s + 0x5b, true);
            let j = cpw7_d(s + 0x61);
            if cpw7_b(j + 0x44) as i8 > 1 && cpw7_sortear(cpw7_b(s + 0x5b) as i8 as i32) == 0 {
                *((j + 0x44) as *mut i8) -= 1;
            }
            cpw7_cair(s + 0x5d, true);
        }
    }
    // o faltoso
    let j = cpw7_d(pessoa + 0x61);
    if j != 0 {
        let a = cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32);
        let b = cpw7_sortear(0xf);
        if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
            cpw7_cair(pessoa + 0x5b, false);
            cpw7_cair(cpw7_d(pessoa + 0x61) + 0x44, false);
            cpw7_tirar_falta(this, r);
            cpw7_faltou(this, pessoa);
            if clube != 0 && cpw7_d(pessoa + 0x61) != 0 {
                let brigar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x72c1f0usize);
                brigar(cpw7_d(0xb6b7c8), pessoa, clube, 8, 0, 0);
            }
            return;
        }
    }
    cpw7_tirar_falta(this, r);
}

// ponytail: sorteia, tira e libera nós das listas (0x68d1f0), dá notícia de volta (0x419710) e de falta (0x41a360): só
// a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_faltas_do_dia(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1308)(this)
}

/// { 0x418ff0 } (thiscall, ret 4, awol.cpp): `AWOL::has_missed_training(pessoa)`, pelos jogadores do dia (0x5a69b0) e
/// pelas telas. Pessoa nula ou fora da conta [0xae2364]: falso. Com o bit 2 da marca dela ([this+0xc] + id) e sem
/// contrato (0x4cf5a0, guardado em [0xae1080]), tira a falta (0x4197c0) antes. Devolve o bit 2 da marca.
unsafe extern "thiscall" fn teve_falta_ao_treino(this: usize, pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let id = cpw7_i(pessoa);
    if id >= cpw7_i(0xae2364) {
        return id as u32 & 0xffffff00;
    }
    if cpw7_b(cpw7_d(this + 0xc).wrapping_add(id as usize)) & 2 != 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let k = contrato_de(cpw7_d(0xae1050), pessoa);
        cpw7_pd(0xae1080, k as u32);
        if k == 0 || cpw7_d(0xae23b4).wrapping_add(cpw7_d(k + 4).wrapping_mul(0x245)) == 0 {
            let limpar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4197c0usize);
            limpar(this, pessoa);
        }
    }
    (cpw7_b(cpw7_d(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize)) & 2) as u32
}

/// O comparador da falta ao treino: lado a lado pela devolução (byte baixo) e pelo [0xae1080]; sem contrato com o bit 2,
/// só a original (a 0x4197c0 tira e libera o nó da lista).
unsafe extern "thiscall" fn comparar_teve_falta_ao_treino(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1309);
    if pessoa != 0 && cpw7_i(pessoa) < cpw7_i(0xae2364) && cpw7_b(cpw7_d(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize)) & 2 != 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let k = contrato_de(cpw7_d(0xae1050), pessoa);
        if k == 0 || cpw7_d(0xae23b4).wrapping_add(cpw7_d(k + 4).wrapping_mul(0x245)) == 0 {
            // ponytail: tira a falta da lista (libera o nó): só a original
            return o(this, pessoa);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(this, pessoa), || teve_falta_ao_treino(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("teve_falta_ao_treino(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// { 0x419070 } (thiscall, ret 4, awol.cpp): `AWOL::has_fitness_incident(pessoa)`, pelos jogadores do dia (0x5a69b0).
/// Pessoa nula, fora da conta [0xae2364] ou sem a lista dos atrasos ([this+8]): falso; senão percorre a lista (cursor em
/// +0xa) atrás do registro da pessoa (id em +0). Verdadeiro no byte baixo; os altos como a original deixa no registrador.
unsafe extern "thiscall" fn teve_incidente_de_forma(this: usize, pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let id = cpw7_i(pessoa);
    if id >= cpw7_i(0xae2364) {
        return id as u32 & 0xffffff00;
    }
    let l = cpw7_d(this + 8);
    if l == 0 {
        return 0;
    }
    cpw7_inicio(l);
    if cpw7_w(l) <= 0 {
        return l as u32 & 0xffffff00;
    }
    let mut i = 0;
    loop {
        let r = cpw7_proximo(cpw7_d(this + 8));
        if r == 0 {
            return 0;
        }
        if cpw7_i(r) == cpw7_i(pessoa) {
            return (cpw7_i(pessoa) as u32 & 0xffffff00) | 1;
        }
        i += 1;
        let n = cpw7_w(cpw7_d(this + 8)) as i32;
        if i >= n {
            return n as u32 & 0xffffff00;
        }
    }
}

/// O comparador do atraso: lado a lado pela devolução (byte baixo) e pelo cursor da lista.
unsafe extern "thiscall" fn comparar_teve_incidente_de_forma(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1310);
    let l = cpw7_d(this + 8);
    let regioes: Vec<(usize, usize)> = if l != 0 { vec![(l + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa), || teve_incidente_de_forma(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("teve_incidente_de_forma(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

// ---- as originais que a faltas_do_dia chama e que liam resíduo da pilha particular (bancada da 259ª) ----

/// O ensaio dos comparadores de `marcar_falta_ao_treino`, `fim_da_ausencia` e `marcas_do_contrato`: a cópia para antes
/// das chamadas que alocam, dão notícia ou não podem rodar duas vezes e deixa em CPW7_DECISAO o que faria.
static mut CPW7_ENSAIO: bool = false;
static mut CPW7_DECISAO: u32 = 0;

/// O clube do contrato `k` (+4 id, [0xae23b4], 0x245 bytes), ou 0.
unsafe fn cpw7_clube_do_contrato(k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    cpw7_d(0xae23b4).wrapping_add(cpw7_d(k + 4).wrapping_mul(0x245))
}

/// O byte +0xc (caso repetido) do registro da falta. A 0x41a360 não o grava: o registro de 13 bytes é montado num local
/// e o +0xc fica com o que a chamada anterior deixou no dword E−0x10 da pilha (E: o esp na entrada). No caminho normal
/// da original, pela faltas_ao_treino (0x5d6aa0), a chamada anterior que alcança essa altura é a 0x54cdb0 (dias desde a
/// chegada), que guarda ali o seu `this`, a data do jogo 0xae2c90: o byte baixo é 0x90. A cópia usa esse valor (um caso
/// repetido); o comparador não o compara (o da original é resíduo de verdade).
const CPW7_BYTE_DO_CASO: u8 = 0x90;

/// { 0x41a360 } (thiscall, ret 4, awol.cpp, SEH só com a data local, sem destrutor): `AWOL::miss_training(pessoa)`,
/// pelas faltas ao treino do dia (0x5d6aa0) e pela faltas_do_dia. Jogador (+0x61) dentro da conta, com contrato
/// (0x4cf5a0, em [0xae1080]) num clube e sem a marca 2: contrato com moral (+0x44) até −100 e o bit 0x2000 (depois o
/// 0x800) em +0x45, com sorteio(profissionalismo + 10) e sorteio(+0x59 + 10) zero, some (0x419840 com 0, 0). Senão
/// marca o bit 2, notícia da falta (0x41a630, com o clube do contrato), o byte +0xa da linha de 79 bytes ([0xae2d0c])
/// soma sorteio(10), a condição (0x62c590) cai 25 × (21 − o +0x34 do jogador) + sorteio + 100 (0x62bd10), o registro
/// (pessoa, hoje, caso) entra na lista das faltas ([this+4], 0x68d0d0); clube da IA (0x600a50) com técnico (+0xcf, +0x69)
/// e sorteio(+0x14 dele) não zero: a multa (0x5a5980, prazo 1 se sorteio(nota do técnico 0x544410 com o +0x19) passar
/// de sorteio(5), senão 2; motivo 1).
unsafe extern "thiscall" fn marcar_falta_ao_treino(this: usize, pessoa: usize) {
    if pessoa == 0 || cpw7_d(pessoa + 0x61) == 0 || cpw7_i(pessoa) >= cpw7_i(0xae2364) {
        return;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contrato_de(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k as u32);
    let clube = cpw7_clube_do_contrato(k);
    if k == 0 || clube == 0 {
        return;
    }
    let id = cpw7_i(pessoa);
    let m = cpw7_d(this + 0xc).wrapping_add(id as usize) as *mut u8;
    if *m & 2 != 0 {
        return;
    }
    let k2 = contrato_de(cpw7_d(0xae1050), pessoa);
    if k2 != 0 && cpw7_b(k2 + 0x44) as i8 <= -100 {
        for bit in [0x2000u32, 0x800] {
            if cpw7_d(k2 + 0x45) as u32 & bit != 0
                && cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32 + 10) == 0
                && cpw7_sortear(cpw7_b(pessoa + 0x59) as i8 as i32 + 10) == 0
            {
                crate::comparacao::ramo("cpw_7 sumiu em vez de faltar");
                if CPW7_ENSAIO {
                    CPW7_DECISAO = 1;
                    return;
                }
                let sumir: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x419840usize);
                sumir(this, pessoa, 0, 0);
                return;
            }
        }
    }
    crate::comparacao::ramo("cpw_7 marcou a falta");
    if CPW7_ENSAIO {
        CPW7_DECISAO = 2;
        return;
    }
    *m |= 2;
    let k3 = contrato_de(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k3 as u32);
    let noticia: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(0x41a630usize);
    noticia(pessoa, cpw7_clube_do_contrato(k3));
    let linha = cpw7_d(0xae2d0c).wrapping_add((cpw7_i(pessoa) as usize).wrapping_mul(79)).wrapping_add(0xa) as *mut u8;
    let r = cpw7_sortear(10);
    *linha = (*linha).wrapping_add(r as u8);
    let condicao: unsafe extern "thiscall" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x62c590usize);
    let c = condicao(cpw7_d(0xb63d28), pessoa, 0);
    // as contas da original são de 32 bits com bytes altos de outros valores no registrador, mas só a palavra baixa
    // chega (a condição é short na 0x62bd10)
    let queda = 0x15i16.wrapping_sub(cpw7_b(cpw7_d(pessoa + 0x61) + 0x34) as i8 as i16).wrapping_mul(25);
    let r = cpw7_sortear(queda as i32);
    let nova = (c as u16).wrapping_sub((queda as u16).wrapping_add(r as u16).wrapping_add(0x64));
    let gravar_condicao: unsafe extern "thiscall" fn(usize, usize, i16, i16) = core::mem::transmute(0x62bd10usize);
    gravar_condicao(cpw7_d(0xb63d28), pessoa, nova as i16, 0);
    let mut reg = [0u8; 13];
    reg[..4].copy_from_slice(&cpw7_i(pessoa).to_le_bytes());
    reg[4..12].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    reg[12] = CPW7_BYTE_DO_CASO;
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    acrescentar(cpw7_d(this + 4), reg.as_ptr() as usize, 0xd);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    let tecnico = cpw7_d(clube + 0xcf);
    if tecnico == 0 {
        return;
    }
    let np = cpw7_d(tecnico + 0x69);
    if np == 0 || cpw7_sortear(cpw7_b(np + 0x14) as i8 as i32) == 0 {
        return;
    }
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    let a = cpw7_sortear(nota(np, cpw7_b(np + 0x19) as u32) as i32);
    let b = cpw7_sortear(5);
    crate::comparacao::ramo("cpw_7 multa pela falta");
    let multar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x5a5980usize);
    multar(cpw7_d(0xae2a3c), pessoa, if a > b { 1 } else { 2 }, 1);
}

/// O comparador da falta marcada, em ensaio: a cópia para na decisão (0 nada, 1 some, 2 marca a falta) e a original roda
/// a partir do mesmo estado (sorteio, marca da pessoa, [0xae1080]); a decisão da original sai da marca (bit 1 novo:
/// sumiu; bit 2 novo: faltou). Sem decisão, o estado inteiro tem de bater (os mesmos sorteios). O byte do caso (+0xc do
/// registro) não entra: na original é resíduo da pilha.
unsafe extern "thiscall" fn comparar_marcar_falta_ao_treino(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1311);
    if pessoa == 0 || cpw7_d(pessoa + 0x61) == 0 || cpw7_i(pessoa) >= cpw7_i(0xae2364) {
        return o(this, pessoa);
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize);
    let mut regioes = vec![(m, 1), (0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| marcar_falta_ao_treino(this, pessoa)));
    CPW7_ENSAIO = false;
    let n = CPW7_DECISAO;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let b0 = cpw7_b(m);
    o(this, pessoa);
    let novo = cpw7_b(m) & !b0;
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let r = if novo & 2 != 0 { 2 } else if novo & 1 != 0 { 1 } else { 0 };
    let igual = (r == n || (n == 1 && b0 & 1 != 0)) && (n != 0 || mo == mr);
    PLACAR.registrar(igual, || format!("marcar_falta_ao_treino(pessoa {:#x}) original={} {:x?} rust={} {:x?}", pessoa, r, mo, n, mr));
}

/// { 0x419710 } (thiscall, ret 4, awol.cpp): `AWOL::end_absence(registro)`, pela faltas_do_dia. Registro com a marca
/// da pessoa (+0 id; [this+0xc] + id) não zero: com contrato (0x4cf5a0, em [0xae1080]) num clube, a notícia da volta
/// (0x419ad0 com o clube +0x39 da pessoa, 0 e −1); troca o bit 1 da marca (o `or 1` da original nunca dá zero) e tira o
/// registro da lista das ausências ([this], 0x68d1f0). Não lê local sem valor.
unsafe extern "thiscall" fn fim_da_ausencia(this: usize, r: usize) {
    if r == 0 {
        return;
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_7 fim da ausencia");
    let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contrato_de(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k as u32);
    if k != 0 && cpw7_clube_do_contrato(k) != 0 {
        if CPW7_ENSAIO {
            CPW7_DECISAO = 1;
        } else {
            let p = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
            let noticia: unsafe extern "stdcall" fn(usize, usize, u32, u32) = core::mem::transmute(0x419ad0usize);
            noticia(p, cpw7_d(p + 0x39), 0, 0xffffffff);
        }
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    *m ^= 1;
    if CPW7_ENSAIO {
        return;
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    tirar(cpw7_d(this), r);
}

/// O comparador do fim da ausência, em ensaio: a cópia sem a notícia e sem tirar da lista; a marca da pessoa e, sem
/// notícia, o [0xae1080] têm de bater com os da original.
unsafe extern "thiscall" fn comparar_fim_da_ausencia(this: usize, r: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1312);
    if r == 0 {
        return o(this, r);
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize);
    let regioes = [(m, 1), (0xae1080, 4)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| fim_da_ausencia(this, r));
    CPW7_ENSAIO = false;
    let noticia = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, r);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = mo.0[0] == mr.0[0] && (noticia || mo.0[1] == mr.0[1]);
    PLACAR.registrar(igual, || format!("fim_da_ausencia(marca {:#x}) original={:x?} rust={:x?}", m, mo, mr));
}

/// { 0x867560 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_flags(marcas)`, chamada por muitas rotinas
/// (no motor.rs pelo endereço; antes só a porta do pós-jogo a desviava, no modo comparação). Marcas acima de 0x1000: a
/// caixa "Error" e segue (aqui: a original inteira). Liga as marcas em +0x40; pessoa (+0 id) que não é das funções
/// 0xc, 0xd e 0xf, contrato com +0x44 positivo, sem +0x49, que quer sair (0x866f60), sem transferência futura
/// (0x8cca10 com 0), sem os bits 1 e 8 de +0x4e, fora da fila da transferência livre (0x8c1870) e com mais de 20 anos
/// (+0x18) ou reputação atual (+0xb do jogador) acima de 4250: o pedido de transferência (0x4d8610 em [0xae1050]).
/// Não lê local sem valor.
unsafe extern "thiscall" fn marcas_do_contrato(c: usize, marcas: u32) {
    if marcas as i32 > 0x1000 {
        // ponytail: a caixa "Error" (antes de qualquer gravação): a original inteira
        return orig::<unsafe extern "thiscall" fn(usize, u32)>(1313)(c, marcas);
    }
    cpw7_pd(c + 0x40, cpw7_d(c + 0x40) as u32 | marcas);
    let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(c) as usize).wrapping_mul(0x6e));
    let funcao = cpw7_b(pessoa + 0x3d);
    if funcao == 0xc || funcao == 0xd || funcao == 0xf {
        return;
    }
    let moral = cpw7_b(c + 0x44) as i8;
    if moral == 0 || moral < 0 || cpw7_b(c + 0x49) != 0 {
        return;
    }
    let quer_sair: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    if quer_sair(c) == 0 {
        return;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if futura(cpw7_d(0xde1f54), pessoa, 0) != 0 || cpw7_b(c + 0x4e) & 9 != 0 {
        return;
    }
    let na_fila: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c1870usize);
    if na_fila(cpw7_d(0xde1f54), pessoa) != 0 {
        return;
    }
    if cpw7_b(pessoa + 0x18) as i8 <= 0x14 && cpw7_w(cpw7_d(pessoa + 0x61) + 0xb) <= 0x109a {
        return;
    }
    crate::comparacao::ramo("cpw_7 pedido de saida");
    if CPW7_ENSAIO {
        CPW7_DECISAO = 1;
        return;
    }
    let pedido: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d8610usize);
    pedido(cpw7_d(0xae1050), c);
}

/// O comparador das marcas do contrato, em ensaio: a cópia sem o pedido de transferência e a original a partir do mesmo
/// estado (marcas e sorteio); sem pedido, tudo tem de bater; com pedido (que sorteia e dá notícia), só as marcas.
unsafe extern "thiscall" fn comparar_marcas_do_contrato(c: usize, marcas: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1313);
    if marcas as i32 > 0x1000 {
        return o(c, marcas);
    }
    let mut regioes = vec![(c + 0x40, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| marcas_do_contrato(c, marcas)));
    CPW7_ENSAIO = false;
    let pediu = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(c, marcas);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if pediu { mo.0[0] == mr.0[0] } else { mo == mr };
    PLACAR.registrar(igual, || format!("marcas_do_contrato({:#x}, {:#x}) pediu={} original={:x?} rust={:x?}", c, marcas, pediu, mo, mr));
}

// ---------- 260a leva: a oferta de transferência recusada pelo clube vendedor e a retirada pelo comprador, com a notícia da oferta recusada (transfer_manager.cpp) ----------

unsafe fn cpx1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx1_clube(id: usize) -> usize {
    cpx1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx1_pessoa(id: usize) -> usize {
    cpx1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como
/// a original faz depois de cada chamada.
unsafe fn cpx1_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpx1_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
unsafe fn cpx1_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx1_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O ensaio do comparador: a cópia roda sem as chamadas que mexem no resto do jogo (o desassossego com sorteio, a
/// notícia, o agendamento, as listas de observação e as ofertas de contrato), que não podem rodar duas vezes.
static mut CPX1_ENSAIO: bool = false;

/// A gerência da lista de observação (Shortlist_Manager em +0x856 da gerência de transferências).
unsafe fn cpx1_listas(this: usize) -> usize {
    cpx1_d(this + 0x856)
}

/// Shortlist_Manager::clear_bid_markers_for_player (0x84aeb0: thiscall(gerência, clube, pessoa), ret 8); no ensaio,
/// nada.
unsafe fn cpx1_desmarcar(this: usize, clube: usize, pessoa: usize) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    f(cpx1_listas(this), clube, pessoa);
}

/// Shortlist_Manager::update_staff_status (0x843e10: thiscall(gerência, pessoa, clube, evento, tipo), ret 0x10); no
/// ensaio, nada.
unsafe fn cpx1_situacao_na_lista(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    f(cpx1_listas(this), pessoa, clube, evento, tipo);
}

/// Uma chamada thiscall(gerência de transferências, oferta) com ret 4 (0x8c1290 desassossegar, 0x8cbf70 tirar a
/// transferência futura, 0x8c6db0 fechar as ofertas de contrato da oferta); no ensaio, nada.
unsafe fn cpx1_com_a_oferta(f: usize, this: usize, o: usize) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    f(this, o);
}

/// O fim comum da recusa e da retirada, a partir do teste da oferta encerrada: sem oferta rival (+0) em aberto, a marca
/// de oferta em andamento (bit 0 do registro de 8 bytes da pessoa em +0x86e) sai.
unsafe fn cpx1_encerrada(o: usize) -> bool {
    match cpx1_b(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpx1_b(o + 0x2e) >= 3,
        _ => false,
    }
}

unsafe fn cpx1_tirar_marca(this: usize, o: usize) {
    if !cpx1_encerrada(o) {
        return;
    }
    let rivais = cpx1_d(o);
    if rivais != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((rivais + 0xa) as *mut usize, cpx1_d(rivais + 2));
        let mut e = proximo(cpx1_d(o));
        while e != 0 {
            let r = cpx1_d(cpx1_d(this).wrapping_add(cpx1_d(e).wrapping_mul(4)));
            if !cpx1_encerrada(r) {
                return;
            }
            e = proximo(cpx1_d(o));
        }
    }
    let m = (cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8))) as *mut u8;
    if *m & 1 != 0 {
        *m &= 0xfe;
    }
}

/// O começo da lista de observação do comprador (0x849ce0) e as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a
/// 0x4ccb30 de cada jogador da troca, cuja marca +4 zera), com +0x10 zerado entre elas; e, com troca, vendedor e
/// vendedor do computador (0x600a50), a situação na lista do comprador de cada jogador da troca que estava nela
/// (0x842080, 0x843e10 evento 1).
unsafe fn cpx1_lista_do_comprador(this: usize, o: usize) {
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let e = entrada(cpx1_listas(this), cpx1_clube(cpx1_d(o + 8)), cpx1_pessoa(cpx1_d(o + 0xc)), (cpx1_b(o + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o registrador do ponteiro da entrada com o setne no byte baixo; a 0x843e10 só lê
        // o byte
        let tipo = (e as u32 & !0xff) | (cpx1_b(e) != 0xb) as u32;
        cpx1_situacao_na_lista(this, cpx1_pessoa(cpx1_d(o + 0xc)), cpx1_clube(cpx1_d(o + 8)), 1, tipo);
    }
}

unsafe fn cpx1_ofertas_de_contrato(this: usize, o: usize, vendedor: usize, comprador: usize) {
    if !CPX1_ENSAIO {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
        tirar(cpx1_d(0xae1050), cpx1_pessoa(cpx1_d(o + 0xc)), cpx1_clube(cpx1_d(o + 8)));
    }
    *((o + 0x10) as *mut u8) = 0;
    let troca = cpx1_d(o + 4);
    if troca != 0 {
        let tirar_da_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
        for e in cpx1_itens(troca) {
            if !CPX1_ENSAIO {
                tirar_da_vaga(cpx1_d(0xae1050), cpx1_pessoa(cpx1_d(e)), cpx1_d(o + 0x20));
            }
            *((e + 4) as *mut u8) = 0;
        }
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if cpx1_d(o + 4) != 0 && vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0 {
        let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
        for e in cpx1_itens(cpx1_d(o + 4)) {
            if na_lista(cpx1_listas(this), cpx1_pessoa(cpx1_d(e)), comprador) != 0 {
                cpx1_situacao_na_lista(this, cpx1_pessoa(cpx1_d(e)), comprador, 1, 0);
            }
        }
    }
}

/// Desmarca os lances do comprador pelo jogador (0x84aeb0) e os do clube do contrato (`vendedor`, ou 0) por cada
/// jogador da troca.
unsafe fn cpx1_desmarcar_lances(this: usize, o: usize, vendedor: usize) {
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    cpx1_desmarcar(this, cpx1_clube(cpx1_d(o + 8)), p); // cpx1: desmarcar os lances do comprador
    let troca = cpx1_d(o + 4);
    if troca != 0 {
        for e in cpx1_itens(troca) {
            cpx1_desmarcar(this, vendedor, cpx1_pessoa(cpx1_d(e)));
        }
    }
}

/// O clube do contrato da pessoa (ou 0), com o contrato guardado em [0xae1080].
unsafe fn cpx1_clube_do_contrato(pessoa: usize) -> usize {
    let c = cpx1_contrato(pessoa);
    if c == 0 {
        0
    } else {
        cpx1_clube(cpx1_d(c + 4))
    }
}

/// A notícia da oferta recusada (0x8d5170: thiscall(gerência, oferta, motivo), ret 8); no ensaio, nada.
unsafe fn cpx1_noticia(this: usize, o: usize, motivo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d5170usize);
    f(this, o, motivo);
}

/// TRANSFER_MANAGER::schedule_offer (0x8c6220: thiscall(gerência, oferta, dias, fatias), ret 0xc); no ensaio, nada.
unsafe fn cpx1_agendar(this: usize, o: usize, dias: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, o, dias, 0);
}

/// { 0x8bf680 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_transfer_offer(vaga, marcar,
/// motivo)`, pelo clube vendedor que recusa (0x8c36b0 e a avaliação dos lances). Com `marcar`, +0x2d da oferta = 1; o
/// jogador pode se desassossegar (0x8c1290); transferência futura (estado 0x13): sai da lista (0x8cbf70). Estado 2
/// (recusada); os lances do comprador pelo jogador e os do clube do contrato pelos da troca saem (0x84aeb0); as ofertas
/// de contrato ligadas fecham (0x8c6db0); a notícia (0x8d5170); a oferta volta em 25 dias (0x8c6220) para sair. Sem
/// rival em aberto, a marca de oferta da pessoa sai (+0x86e). A lista de observação do comprador (0x849ce0, 0x843e10),
/// as ofertas de contrato (0x4cca70, 0x4ccb30) e +0x10 = 0; com troca e vendedor do computador, os jogadores da troca
/// na lista do comprador (0x842080, 0x843e10).
unsafe extern "thiscall" fn recusar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    crate::comparacao::ramo("cpx_1 recusa desassossega");
    cpx1_com_a_oferta(0x8c1290, this, o); // cpx1: o desassossego da recusa
    let atual = *vaga;
    if cpx1_b(atual + 0x2c) == 0x13 {
        cpx1_com_a_oferta(0x8cbf70, this, atual);
    }
    *((o + 0x2c) as *mut u8) = 2;
    let vendedor2 = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    crate::comparacao::ramo("cpx_1 recusa desmarca");
    cpx1_desmarcar_lances(this, o, vendedor2);
    cpx1_com_a_oferta(0x8c6db0, this, o);
    cpx1_noticia(this, o, motivo);
    cpx1_agendar(this, o, 0x19);
    cpx1_tirar_marca(this, o);
    cpx1_lista_do_comprador(this, o);
    cpx1_ofertas_de_contrato(this, o, vendedor, comprador);
}

/// { 0x8bfcb0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::withdraw_transfer_offer(vaga, marcar,
/// motivo)`, pelo comprador que desiste (a revisão dos lances e as ofertas desfeitas). Com `marcar`, +0x2d = 1;
/// transferência futura: sai da lista (0x8cbf70); estado de 0xb para cima: as ofertas de contrato guardadas voltam
/// (0x4cf4a0). Estado 3 (retirada); as ofertas de contrato ligadas fecham (0x8c6db0); os lances saem (0x84aeb0); +0x10 =
/// 4; a oferta volta em 10 dias (0x8c6220). Sem rival em aberto, a marca de oferta da pessoa sai. Motivo 2: o jogador
/// pode se desassossegar (0x8c1290). A lista de observação do comprador, a notícia (0x8d5170), as ofertas de contrato e
/// +0x10 = 0; com troca e vendedor do computador, os jogadores da troca na lista do comprador.
unsafe extern "thiscall" fn retirar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    let atual = *vaga;
    if cpx1_b(atual + 0x2c) == 0x13 {
        cpx1_com_a_oferta(0x8cbf70, this, atual);
    }
    if cpx1_b(o + 0x2c) as i8 >= 0xb && !CPX1_ENSAIO {
        let reenviar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cf4a0usize);
        reenviar(cpx1_d(0xae1050), cpx1_d(o + 0xc));
    }
    *((o + 0x2c) as *mut u8) = 3;
    cpx1_com_a_oferta(0x8c6db0, this, o);
    let vendedor2 = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    cpx1_desmarcar_lances(this, o, vendedor2);
    *((o + 0x10) as *mut u8) = 4;
    cpx1_agendar(this, o, 0xa);
    cpx1_tirar_marca(this, o);
    if motivo as u16 == 2 { // cpx1: só o motivo 2 desassossega
        cpx1_com_a_oferta(0x8c1290, this, o);
    } else {
        crate::comparacao::ramo("cpx_1 retirada sem desassossego");
    }
    cpx1_lista_do_comprador(this, o);
    cpx1_noticia(cpx1_d(0xde1f54), o, motivo);
    cpx1_ofertas_de_contrato(this, o, vendedor, comprador);
}

/// As vagas de uma LLIST (ou nenhuma), sem mexer no cursor.
unsafe fn cpx1_nos(l: usize) -> Vec<usize> {
    if l == 0 {
        return Vec::new();
    }
    let cursor = cpx1_d(l + 0xa);
    let v = cpx1_itens(l);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    v
}

/// O comparador da recusa e da retirada: a cópia em ensaio (sem as chamadas que mexem no resto do jogo) e a original a
/// partir do mesmo estado (a oferta, as duas listas dela, as vagas da troca, o registro da pessoa em +0x86e e
/// [0xae1080]); iguais o estado (+0x2c), +0x10, a marca +4 de cada jogador da troca e a marca de oferta da pessoa, que
/// a original grava sem depender das chamadas.
unsafe fn cpx1_comparar(nome: &str, this: usize, vaga: *mut usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let o = *vaga;
    let marca = cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8));
    let nos = cpx1_nos(cpx1_d(o + 4));
    let mut regioes = vec![(o, 0x30usize), (marca, 8), (0xae1080, 4)];
    for l in [cpx1_d(o), cpx1_d(o + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    for &e in &nos {
        regioes.push((e, 5));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX1_ENSAIO = true;
    crate::aleatorio::pausar_rastro(copia);
    CPX1_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let (a, b) = (&mo.0, &mr.0);
    let mut igual = a[0].1[0x2c] == b[0].1[0x2c] && a[0].1[0x10] == b[0].1[0x10] && a[1].1[0] == b[1].1[0];
    for k in regioes.len() - nos.len()..regioes.len() {
        igual &= a[k].1[4] == b[k].1[4];
    }
    PLACAR.registrar(igual, || format!("{}(oferta {:#x}) original={:x?} rust={:x?}", nome, o, mo, mr));
}

unsafe extern "thiscall" fn comparar_recusar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(1314);
    cpx1_comparar("recusar_oferta_de_transferencia", this, vaga, || o(this, vaga, marcar, motivo), || recusar_oferta_de_transferencia(this, vaga, marcar, motivo))
}

unsafe extern "thiscall" fn comparar_retirar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(1315);
    cpx1_comparar("retirar_oferta_de_transferencia", this, vaga, || o(this, vaga, marcar, motivo), || retirar_oferta_de_transferencia(this, vaga, marcar, motivo))
}

/// { 0x8d5170 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_rejected_news(oferta, motivo)`, pela recusa, pela retirada e pela recusa do empréstimo. Os clubes humanos
/// que recebem (0x8d1130, com 0x2d se a oferta estiver recusada, senão 0x2e; até 30); sem nenhum, nada. A notícia 0xfa1:
/// a pessoa (o id e as três chaves), o clube do contrato (sem contrato de tipo 5; nenhum no motivo 6 com +0x2f), o
/// comprador, a lista da troca (+4), o estado, o motivo, +0x1f, o valor total (0x8e7a70 com 0, 0, 0, 1) e a divisão do
/// clube do contrato (+0x57); a cada destinatário, na vaga 12, 1 se for o comprador ou o clube do contrato.
unsafe extern "thiscall" fn noticia_oferta_recusada(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    let tipo5 = |c: usize| cpx1_b(c + 0x35) & 0x3f == 5;
    let c = cpx1_contrato(p);
    let do_contrato = if c == 0 || tipo5(c) { 0 } else { cpx1_clube(cpx1_d(c + 4)) };
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (cpx1_b(o + 0x2c) != 2) as u32 + 0x2d, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    let c = cpx1_contrato(p);
    let mut clube = if c == 0 || tipo5(c) { 0 } else { cpx1_clube(cpx1_d(c + 4)) };
    let c = cpx1_contrato(p);
    if c != 0 && tipo5(c) {
        clube = 0;
    }
    if motivo as i16 == 6 && cpx1_b(o + 0x2f) != 0 {
        clube = 0;
    }
    crate::comparacao::ramo("cpx_1 noticia da recusa");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa1, 0);
    gravar(nt, 0, cpx1_d(p) as u32);
    gravar(nt, 1, cpx1_d(cpx1_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpx1_d(cpx1_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpx1_d(cpx1_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, if clube != 0 { cpx1_d(clube) as u32 } else { u32::MAX });
    gravar(nt, 5, cpx1_d(o + 8) as u32);
    gravar(nt, 6, cpx1_d(o + 4) as u32);
    gravar(nt, 7, cpx1_b(o + 0x2c) as i8 as u32);
    gravar(nt, 8, motivo as i16 as u32);
    gravar(nt, 9, cpx1_b(o + 0x1f) as i8 as u32);
    gravar(nt, 10, total(o, 0, 0, 0, 1));
    let divisao = if clube != 0 { cpx1_d(clube + 0x57) } else { 0 };
    gravar(nt, 11, if divisao != 0 { cpx1_d(divisao) as u32 } else { u32::MAX });
    for &d in lista.iter().take(n.max(0) as usize) {
        gravar(nt, 12, (d == comprador || d == do_contrato) as u32);
        entregar(cpx1_d(0xb7431c), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_oferta_recusada(this: usize, o: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1316)(this, o, motivo)
}

// ---------- 260a leva: o jogador desassossegado pelo lance de um clube maior, a notícia da oferta desfeita e a do prêmio por jogos (transfer_manager.cpp) ----------

unsafe fn cpx2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx2_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpx2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx2_clube(id: usize) -> usize {
    cpx2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx2_pessoa(id: usize) -> usize {
    cpx2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpx2_contrato_de(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx2_d(0xae1050), pessoa)
}

/// O mesmo, guardado em [0xae1080] como a original faz.
unsafe fn cpx2_contrato(pessoa: usize) -> usize {
    let c = cpx2_contrato_de(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O ensaio do comparador: a cópia para antes da queixa (0x867700), da fuga (0x419840) e das antipatias (0x547e70,
/// 0x547af0), que mexem no resto do jogo, e marca que chegou lá.
static mut CPX2_ENSAIO: bool = false;
static mut CPX2_CHEGOU: bool = false;

/// { 0x8c1290 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::unsettle_player_after_bid(oferta)`, pela
/// recusa (0x8bf680), pela retirada no motivo 2 (0x8bfcb0) e pelo clube que recusa (0x8c36b0). Sorteio(5750) + 4250
/// abaixo da reputação do comprador (+0x80): +0x2d da oferta = 1. Com +0x2d = 1, clube do contrato, comprador e ficha: o
/// interesse x (0x846920 pelo estatuto 0x849f00) e a nota n = (8 × ambição² − 1600) − reputação / 40 − min(x⁴, 300 ×
/// x) − 2 × lealdade²... (os atributos +0x5d, +0x57, +0x5b, +0x59 pela pow do jogo); listado (+0x4e & 8): − 2000 −
/// sorteio(6000). Com x de 5,5, n < −sorteio(700), (n < −4000 ou +0x5b + +0x59 < 15), reputação acima de 3500 e o
/// valor ponderado da oferta (0x8e7b50) acima do menor entre 1,5 × valor (0x4d0c60) e 0,8 × preço (0x4d1600): a queixa
/// 0x80 (0x867700); jogador de 5250, idade 19 a 31, n < −8000, com o critério (0x866f60), sem a marca +1 do registro
/// +0x86e e presente (0x418f60): some (0x419840) e, com +0x44 − sorteio(200) + sorteio(400) < −150, rejeita o clube
/// (0x547e70). x acima de 10, n < −5000 ou o menor abaixo de 0,75 × o ponderado: antipatia ao técnico (0x547af0).
unsafe extern "thiscall" fn desassossegar_pelo_lance(this: usize, o: usize) {
    let p = cpx2_pessoa(cpx2_d(o + 0xc));
    let c = cpx2_contrato(p);
    let do_contrato = if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    let comprador = cpx2_clube(cpx2_d(o + 8));
    let r = sortear(0x1676);
    crate::comparacao::ramo("cpx_2 sorteio do desassossego");
    if (cpx2_w(comprador + 0x80) as i32) > r.wrapping_add(0x109a) { // cpx2: o sorteio do desassossego
        *((o + 0x2d) as *mut u8) = 1;
    }
    if cpx2_b(o + 0x2d) != 1 || do_contrato == 0 || comprador == 0 || cpx2_d(p + 0x61) == 0 {
        return;
    }
    let c2 = cpx2_contrato_de(p);
    let listas = cpx2_d(this + 0x856);
    let estatuto: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(0x849f00usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
    let e = estatuto(listas, p, comprador, 0);
    // ponytail: a original empurra o dword da vaga onde guardou o byte (o resto é sobra de pilha); a 0x846920 só lê o
    // byte
    let x = interesse(listas, p, comprador, 1, 0, e as u32, 0);
    let pw = pow_do_jogo(x, f64_em(0x968938));
    let a = x * f64_em(0x96a6b0);
    let v = if !(pw >= a) { pw } else { a };
    let mut n = ftol(pow_do_jogo(cpx2_sb(p + 0x5d) as f64, f64_em(0x968950)) * f64_em(0x968f10) - f64_em(0x96a590)) as i16;
    let ficha = cpx2_d(p + 0x61);
    n = n.wrapping_sub((cpx2_w(ficha + 0xb) as i32 / 40) as i16);
    n = n.wrapping_sub(ftol(v) as i16);
    n = n.wrapping_sub(ftol(pow_do_jogo(cpx2_sb(p + 0x57) as f64, f64_em(0x968950))) as i16);
    n = n.wrapping_add(ftol(pow_do_jogo(cpx2_sb(p + 0x5b) as f64, f64_em(0x968930))) as i16);
    n = n.wrapping_add(ftol(pow_do_jogo(cpx2_sb(p + 0x59) as f64, f64_em(0x968930))) as i16);
    if c2 == 0 {
        return;
    }
    if cpx2_b(c2 + 0x4e) & 8 != 0 {
        crate::comparacao::ramo("cpx_2 listado");
        n = n.wrapping_add((-2000i32).wrapping_sub(sortear(0x1770)) as i16); // cpx2: o desconto do listado
    }
    crate::comparacao::ramo("cpx_2 interesse avaliado");
    if !(x >= f64_em(0x96d298)) { // cpx2: o interesse mínimo
        return;
    }
    if n as i32 >= -sortear(0x2bc) {
        return;
    }
    if n >= -4000 && cpx2_sb(p + 0x5b) + cpx2_sb(p + 0x59) >= 0xf {
        return;
    }
    if cpx2_w(cpx2_d(p + 0x61) + 0xb) <= 0xdac {
        return;
    }
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let a1 = ftol(valor(cpx2_d(0xae1050), p, do_contrato, 0, -1, 0) as f64 * f64_em(0x968940));
    let b1 = ftol(preco(cpx2_d(0xae1050), do_contrato, comprador, p, 0, 1, 1) as f64 * f64_em(0x9689b0));
    let pond = ponderado(o, 0, 0, 0, 1, 1);
    let menor = if a1 < b1 { a1 } else { b1 };
    if pond < menor {
        return;
    }
    if CPX2_ENSAIO {
        CPX2_CHEGOU = true;
        return;
    }
    crate::comparacao::ramo("cpx_2 desassossegado");
    let queixa: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
    queixa(c2, 0x80, 0);
    let idade = cpx2_sb(p + 0x18);
    if cpx2_w(cpx2_d(p + 0x61) + 0xb) > 0x1482 && idade > 0x12 && idade < 0x20 && n < -8000 {
        let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
        let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
        if criterio(c2) != 0
            && cpx2_b(cpx2_d(this + 0x86e).wrapping_add(cpx2_d(p).wrapping_mul(8)) + 1) == 0
            && ausente(cpx2_d(0xad9c6c), p, 0) == 0
        {
            let sumir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x419840usize);
            sumir(cpx2_d(0xad9c6c), p, comprador, 1);
            let forca = cpx2_sb(c2 + 0x44);
            let r1 = sortear(0xc8);
            let r2 = sortear(0x190);
            if forca.wrapping_sub(r1).wrapping_add(r2) < -150 {
                let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
                rejeitar(p, do_contrato, 1);
            }
        }
    }
    if x > f64_em(0x9688b0) || n < -5000 || (menor as f64) < pond as f64 * f64_em(0x969040) {
        let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
        desafeto(p, cpx2_d(do_contrato + 0xcf), 1);
    }
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado (a oferta, [0xae1080] e o sorteio). Sem chegar
/// à queixa (quase sempre), tudo igual; chegando, só a original (a queixa, a fuga e as antipatias não podem rodar duas
/// vezes e a original sorteia mais).
unsafe extern "thiscall" fn comparar_desassossegar_pelo_lance(this: usize, o: usize) {
    let orig_f = orig::<unsafe extern "thiscall" fn(usize, usize)>(1317);
    let mut regioes = vec![(o, 0x30usize), (0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX2_ENSAIO = true;
    CPX2_CHEGOU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| desassossegar_pelo_lance(this, o)));
    CPX2_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    orig_f(this, o);
    if CPX2_CHEGOU {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("desassossegar_pelo_lance(oferta {:#x}) original={:x?} rust={:x?}", o, mo, mr));
}

/// { 0x8de790 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_collapsed_news(oferta, clube, motivo)`, pela oferta de contrato aceita (0x8c38b0), pela oferta desfeita
/// (0x8c44e0), pelas permissões de trabalho (0x8c7690) e por 0x8c4950. Os clubes humanos que recebem (0x8d1130 com 0x2f,
/// até 30); sem nenhum, nada. A notícia 0xfb0: o clube do contrato (sem contrato de tipo 5), o comprador, a pessoa (o id
/// e as três chaves), o clube, o motivo e +0x1f; a cada destinatário, menos o próprio clube no motivo 0xb.
unsafe extern "thiscall" fn noticia_oferta_desfeita(this: usize, o: usize, clube: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpx2_pessoa(cpx2_d(o + 0xc));
    let c = cpx2_contrato(p);
    let mut do_contrato = if c == 0 || cpx2_b(c + 0x35) & 0x3f == 5 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    let comprador = cpx2_clube(cpx2_d(o + 8));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x2f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    if do_contrato != 0 && cpx2_b(cpx2_contrato_de(p) + 0x35) & 0x3f == 5 {
        do_contrato = 0;
    }
    crate::comparacao::ramo("cpx_2 noticia da oferta desfeita");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb0, 0);
    gravar(nt, 0, if do_contrato != 0 { cpx2_d(do_contrato) as u32 } else { u32::MAX });
    gravar(nt, 1, cpx2_d(comprador) as u32);
    gravar(nt, 2, cpx2_d(p) as u32);
    gravar(nt, 3, cpx2_d(cpx2_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx2_d(cpx2_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx2_d(cpx2_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 6, if clube != 0 { cpx2_d(clube) as u32 } else { u32::MAX });
    gravar(nt, 7, motivo as i16 as u32);
    gravar(nt, 8, cpx2_b(o + 0x1f) as i8 as u32);
    for &d in lista.iter().take(n.max(0) as usize) {
        if d != clube || motivo as u16 != 0xb {
            entregar(cpx2_d(0xb7431c), nt, d);
        }
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_oferta_desfeita(this: usize, o: usize, clube: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1318)(this, o, clube, motivo)
}

/// { 0x8db8a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_appearance_money_news(cláusula, valor, tipo, calado)`, pelo prêmio por jogos (0x8cb340). Calado, sem contrato ou
/// sem o clube dele: nada. Senão a notícia 0xfa6: a pessoa da cláusula (+0: o id e as três chaves), +4, o valor, o
/// clube que paga (+9), o tipo e o clube do contrato; ao clube que paga e ao clube do contrato.
unsafe extern "thiscall" fn noticia_premio_por_jogos(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    let _ = this;
    if calado != 0 {
        return;
    }
    let p = cpx2_pessoa(cpx2_d(clausula));
    let c = cpx2_contrato(p);
    if c == 0 || cpx2_clube(cpx2_d(c + 4)) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpx_2 noticia do premio por jogos");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa6, 0);
    gravar(nt, 0, cpx2_d(p) as u32);
    gravar(nt, 1, cpx2_d(cpx2_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpx2_d(cpx2_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpx2_d(cpx2_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpx2_d(clausula + 4) as u32);
    gravar(nt, 5, valor);
    gravar(nt, 6, cpx2_d(clausula + 9) as u32);
    gravar(nt, 7, tipo as i8 as u32);
    let c = cpx2_contrato(p);
    // a original lê o id do clube do contrato mesmo sem contrato (o registro 0): nunca acontece aqui
    let clube = if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    gravar(nt, 8, cpx2_d(clube) as u32);
    entregar(cpx2_d(0xb7431c), nt, cpx2_clube(cpx2_d(clausula + 9)));
    let c = cpx2_contrato(p);
    entregar(cpx2_d(0xb7431c), nt, if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) });
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_premio_por_jogos(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1319)(this, clausula, valor, tipo, calado)
}

// ---------- 260a leva: a transferência efetivada, com a notícia da transferência concluída e a do acordo entre os clubes (transfer_manager.cpp) ----------

unsafe fn cpx3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx3_clube(id: usize) -> usize {
    cpx3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx3_pessoa(id: usize) -> usize {
    cpx3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpx3_contrato_de(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx3_d(0xae1050), pessoa)
}

/// O mesmo, guardado em [0xae1080] como a original faz.
unsafe fn cpx3_contrato(pessoa: usize) -> usize {
    let c = cpx3_contrato_de(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato principal que não é de tipo 5 (ou 0), com o contrato guardado em [0xae1080].
unsafe fn cpx3_clube_nao_5(pessoa: usize) -> usize {
    let c = cpx3_contrato(pessoa);
    if c == 0 || cpx3_b(c + 0x35) & 0x3f == 5 {
        0
    } else {
        cpx3_clube(cpx3_d(c + 4))
    }
}

/// Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
unsafe fn cpx3_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx3_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O teste da oferta encerrada (estados 2, 3, 0x14, 0x10, 4; 0xf com +0x2e de 3).
unsafe fn cpx3_encerrada(o: usize) -> bool {
    match cpx3_b(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpx3_b(o + 0x2e) >= 3,
        _ => false,
    }
}

/// O clube que recebe: sem técnico humano (0x600a50) e com números de camisa (SQUAD_MANAGER 0x85add0 em [0xdd7ef0]),
/// os números de quem chegou (0x85a470).
unsafe fn cpx3_numeros(clube: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    if usa(cpx3_d(0xdd7ef0), clube, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85a470usize);
        numerar(cpx3_d(0xdd7ef0), clube, 0); // cpx3: os números de camisa de quem chegou
    }
}

/// { 0x8c9390 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::finalise_transfer(oferta)`, pelo
/// fechamento da oferta (0x8c34e0) e pela transferência futura. A confiança das diretorias (0x6978f0); as regras da
/// nação do comprador (as 28 de +0x8ac, pela nação +0xa): método 1 (sem +0x1f) ou 2 com a oferta; as cláusulas
/// anteriores (0x8c91e0); com vendedor (contrato sem tipo 5), o pagamento (0x8ca8a0); sem +0x1f = 1, os registros do
/// técnico (0x86cd70); os recordes do clube (0x448760). Com troca, o primeiro da troca sai do elenco do clube dele
/// (+0xd7, 50 vagas). Com valor (+0x1b) e sem +0x69 na pessoa, a parte dos clubes anteriores (0x8e4f20). O jogador vai
/// ao comprador (0x8c9720) e os da troca ao vendedor, com os números de camisa; estado 0x14, a oferta volta em 10 dias
/// (0x8c6220); sem rival em aberto, a marca de oferta (+0x86e) sai; os lances (0x84aeb0) e as ofertas de contrato
/// ligadas (0x8c6db0).
unsafe extern "thiscall" fn efetivar_transferencia(this: usize, o: usize) {
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let comprador = cpx3_clube(cpx3_d(o + 8));
    let vendedor = cpx3_clube_nao_5(p);
    let confianca: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6978f0usize);
    crate::comparacao::ramo("cpx_3 confianca das diretorias");
    confianca(cpx3_d(0xb67a44), o); // cpx3: a confiança das diretorias
    let nacao = cpx3_d(cpx3_clube(cpx3_d(o + 8)) + 0x53);
    for k in 0..0x1c {
        let regras = cpx3_d(this + 0x8ac + k * 4);
        if cpx3_d(regras + 0xa) == nacao {
            if regras != 0 {
                let metodo = cpx3_d(cpx3_d(regras) + if cpx3_b(o + 0x1f) == 0 { 4 } else { 8 });
                let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(metodo);
                f(regras, o);
            }
            break;
        }
    }
    let com_a_oferta = |f: usize, t: usize| {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
        f(t, o);
    };
    com_a_oferta(0x8c91e0, this);
    if vendedor != 0 {
        crate::comparacao::ramo("cpx_3 paga a transferencia");
        com_a_oferta(0x8ca8a0, this); // cpx3: o pagamento da transferência
    }
    if cpx3_b(o + 0x1f) != 1 {
        com_a_oferta(0x86cd70, cpx3_d(0xdd7f00));
    }
    com_a_oferta(0x448760, cpx3_d(0xad9dac));
    let troca = cpx3_d(o + 4);
    if troca != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((troca + 0xa) as *mut usize, cpx3_d(troca + 2));
        let q = cpx3_pessoa(cpx3_d(proximo(cpx3_d(o + 4))));
        let clube = cpx3_d(q + 0x39);
        for j in 0..0x32usize {
            let vaga = clube + 0xd7 + j * 4;
            if cpx3_d(vaga) == q {
                *(vaga as *mut usize) = 0;
                break;
            }
        }
    }
    if cpx3_d(o + 0x1b) != 0 && cpx3_d(p + 0x69) == 0 {
        crate::comparacao::ramo("cpx_3 parte dos clubes anteriores");
        com_a_oferta(0x8e4f20, this); // cpx3: a parte dos clubes anteriores
    }
    let mudar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8c9720usize);
    mudar(this, p, comprador, o);
    cpx3_numeros(comprador);
    if cpx3_d(o + 4) != 0 {
        for e in cpx3_itens(cpx3_d(o + 4)) {
            mudar(this, cpx3_pessoa(cpx3_d(e)), vendedor, o);
        }
        cpx3_numeros(vendedor);
    }
    *((o + 0x2c) as *mut u8) = 0x14;
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    agendar(this, o, 0xa, 0);
    'marca: {
        if !cpx3_encerrada(o) {
            break 'marca;
        }
        let rivais = cpx3_d(o);
        if rivais != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            core::ptr::write_unaligned((rivais + 0xa) as *mut usize, cpx3_d(rivais + 2));
            let mut e = proximo(cpx3_d(o));
            while e != 0 {
                if !cpx3_encerrada(cpx3_d(cpx3_d(this).wrapping_add(cpx3_d(e).wrapping_mul(4)))) {
                    break 'marca;
                }
                e = proximo(cpx3_d(o));
            }
        }
        let m = (cpx3_d(this + 0x86e).wrapping_add(cpx3_d(o + 0xc).wrapping_mul(8))) as *mut u8;
        if *m & 1 != 0 {
            *m &= 0xfe;
        }
    }
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let c = cpx3_contrato(p);
    let novo = if c == 0 { 0 } else { cpx3_clube(cpx3_d(c + 4)) };
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    desmarcar(cpx3_d(this + 0x856), cpx3_clube(cpx3_d(o + 8)), p);
    if cpx3_d(o + 4) != 0 {
        for e in cpx3_itens(cpx3_d(o + 4)) {
            desmarcar(cpx3_d(this + 0x856), novo, cpx3_pessoa(cpx3_d(e)));
        }
    }
    com_a_oferta(0x8c6db0, this);
}

// ponytail: muda jogadores de clube, paga e agenda (não pode rodar duas vezes): só a original, com o plano do estado
// final (0x14, que nenhuma chamada depois dele muda); a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_efetivar_transferencia(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1320)(this, o);
    let e = cpx3_b(o + 0x2c);
    PLACAR.registrar(e == 0x14, || format!("efetivar_transferencia(oferta {:#x}) estado={:#x} (plano 0x14)", o, e));
}

/// A notícia local com a pessoa (o id e as três chaves) nas vagas `k` a `k` + 3.
unsafe fn cpx3_pessoa_na_noticia(nt: usize, k: u8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(nt, k as i8, cpx3_d(p) as u32);
    gravar(nt, k.wrapping_add(1) as i8, cpx3_d(cpx3_d(p + 4) + 0x33) as u32);
    gravar(nt, k.wrapping_add(2) as i8, cpx3_d(cpx3_d(p + 8) + 0x33) as u32);
    gravar(nt, k.wrapping_add(3) as i8, cpx3_d(cpx3_d(p + 0xc) + 0x33) as u32);
}

/// A vaga 5 das duas notícias: o id do clube do contrato (sem tipo 5) ou −1 (a original relê o contrato e, sem ele, lê
/// o id no endereço 0, o que não acontece porque acabou de achá-lo).
unsafe fn cpx3_vaga_5(p: usize) -> u32 {
    if cpx3_clube_nao_5(p) == 0 {
        return u32::MAX;
    }
    cpx3_d(cpx3_clube_nao_5(p)) as u32
}

/// Os da troca (+4) na notícia a partir da vaga `k` (os três primeiros, quatro vagas cada); devolve (quantos, a vaga
/// seguinte).
unsafe fn cpx3_troca_na_noticia(nt: usize, o: usize, mut k: u8) -> (i8, u8) {
    let mut n = 0i8;
    let troca = cpx3_d(o + 4);
    if troca != 0 {
        for e in cpx3_itens(troca) {
            if n < 3 {
                cpx3_pessoa_na_noticia(nt, k, cpx3_pessoa(cpx3_d(e)));
                k = k.wrapping_add(4);
            }
            n = n.wrapping_add(1);
        }
    }
    (n, k)
}

/// A entrega: com técnico humano no comprador (0x7875e0), a todos os técnicos (0x786780); senão a cada destinatário.
unsafe fn cpx3_entregar(nt: usize, o: usize, lista: &[usize], n: i8) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(cpx3_d(0xb7431c), cpx3_clube(cpx3_d(o + 8))) != 0 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(cpx3_d(0xb7431c), nt);
        return;
    }
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(cpx3_d(0xb7431c), nt, d);
    }
}

/// { 0x8dbef0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_completed_news(oferta)`, pelo fechamento da oferta (0x8c34e0). Os clubes humanos que recebem (0x8d1130
/// com 0x5f, até 30); sem nenhum, nada. A notícia 0xfa8: a pessoa, o comprador, o clube do contrato, o valor (+0x1b), 0,
/// o valor total (0x8e7a70 com 1, 1, 0, 1), +0x2f, +0x11, as cláusulas (0x8e7ef0); com +0x2f, contrato de clube e +0x30
/// & 0xf = 3, o fim do contrato se ainda vier (vagas 10 e 11). Os da troca (até 3, da vaga 15) e quantos (14); um
/// sorteio(3) na vaga seguinte; com ficha, 1, os pontos fortes (0x546ab0, sorteia) e as posições (0x5b7d20), senão 0.
unsafe extern "thiscall" fn noticia_transferencia_concluida(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let clausulas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8e7ef0usize);
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x5f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia concluida");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa8, 0);
    cpx3_pessoa_na_noticia(nt, 0, p);
    gravar(nt, 4, cpx3_d(o + 8) as u32);
    gravar(nt, 5, cpx3_vaga_5(p));
    gravar(nt, 6, cpx3_d(o + 0x1b) as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, total(o, 1, 1, 0, 1));
    gravar(nt, 9, cpx3_b(o + 0x2f) as i8 as u32);
    gravar(nt, 0xc, cpx3_b(o + 0x11) as i8 as u32);
    gravar(nt, 0xd, clausulas(o));
    if cpx3_b(o + 0x2f) != 0 && cpx3_clube_nao_5(p) != 0 && cpx3_b(o + 0x30) & 0xf == 3 {
        let c = cpx3_contrato_de(p);
        let ano = cpx3_w(0xae2c92) as u16;
        let vem = c != 0
            && (ano < cpx3_w(c + 0x2f) as u16 || (ano == cpx3_w(c + 0x2f) as u16 && cpx3_w(0xae2c90) < cpx3_w(c + 0x2d)));
        if vem {
            gravar(nt, 10, cpx3_w(c + 0x2d) as i32 as u32);
            gravar(nt, 11, cpx3_w(c + 0x2f) as u16 as u32);
        } else {
            gravar(nt, 10, 0);
            gravar(nt, 11, 0);
        }
    }
    let (m, k) = cpx3_troca_na_noticia(nt, o, 0xf);
    gravar(nt, 0xe, m as i32 as u32);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    gravar(nt, k as i8, sorteio(3) as u32);
    let ficha = cpx3_d(p + 0x61);
    if ficha != 0 {
        gravar(nt, k.wrapping_add(1) as i8, 1);
        let fortes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
        let posicoes: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x5b7d20usize);
        let clube = cpx3_clube_nao_5(p);
        gravar(nt, k.wrapping_add(2) as i8, fortes(p, clube));
        gravar(nt, k.wrapping_add(3) as i8, posicoes(cpx3_d(p + 0x61)) as i32 as u32);
    } else {
        gravar(nt, k.wrapping_add(1) as i8, 0);
    }
    cpx3_entregar(nt, o, &lista, n);
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_transferencia_concluida(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1321)(this, o)
}

/// { 0x8e05d0 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_agreed_news(oferta, data)`, pela aprovação dos clubes (0x8c3310). Os clubes humanos que recebem
/// (0x8d1130 com 0xf, até 30); sem nenhum, nada. A notícia 0xfb3: a pessoa, o comprador, o clube do contrato, o valor
/// (+0x1b), +0x2f, o valor total (0x8e7a70 com 0, 1, 0, 1), o nibble baixo de +0x30 com sinal, a data da conclusão (dia
/// com sinal, ano), os da troca (até 3, da vaga 13) e quantos (12).
unsafe extern "thiscall" fn noticia_transferencia_acertada(this: usize, o: usize, data: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0xf, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia acertada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb3, 0);
    cpx3_pessoa_na_noticia(nt, 0, p);
    gravar(nt, 4, cpx3_d(o + 8) as u32);
    gravar(nt, 5, cpx3_vaga_5(p));
    gravar(nt, 6, cpx3_d(o + 0x1b) as u32);
    gravar(nt, 7, cpx3_b(o + 0x2f) as i8 as u32);
    gravar(nt, 8, total(o, 0, 1, 0, 1));
    gravar(nt, 9, (((cpx3_b(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    gravar(nt, 0xa, cpx3_w(data) as i32 as u32);
    gravar(nt, 0xb, cpx3_w(data + 2) as u16 as u32);
    let (m, _) = cpx3_troca_na_noticia(nt, o, 0xd);
    gravar(nt, 0xc, m as i32 as u32);
    cpx3_entregar(nt, o, &lista, n);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_transferencia_acertada(this: usize, o: usize, data: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1322)(this, o, data)
}

// ---------- 260a leva: o empréstimo do arquivo de configuração, a notícia do fim do contrato e a do pedido de adiamento da transferência (transfer_manager.cpp, contract_manager.cpp) ----------

unsafe fn cpx4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx4_clube(id: usize) -> usize {
    cpx4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx4_pessoa(id: usize) -> usize {
    cpx4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): copia a data nos dois.
unsafe fn cpx4_atribuir(destino: usize, origem: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), origem);
}

/// A data `a` (dia com sinal em +0, ano em +2) vem antes da `b`.
unsafe fn cpx4_antes(a: usize, b: usize) -> bool {
    let (ya, yb) = (cpx4_w(a + 2) as u16, cpx4_w(b + 2) as u16);
    ya < yb || (ya == yb && cpx4_w(a) < cpx4_w(b))
}

/// As condições do empréstimo da configuração, na ordem da original: o time B ligado ao clube (0x540a50; vale só se não
/// for o principal), o elenco do clube abaixo de 50 (0x53c590), a pessoa sem empréstimo (0x4cf6d0), o começo até hoje,
/// o fim de hoje em diante e um contrato (0x4cf5a0). Devolve o time B (ou 0) quando passam.
unsafe fn cpx4_pode(p: usize, clube: usize, inicio: usize, fim: usize) -> Option<usize> {
    let ligado: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mut principal = 0u32;
    let mut b = ligado(clube, &mut principal, 1);
    if b != 0 && principal != 0 {
        b = 0;
    }
    if contar(clube, 0, 0) as i8 >= 0x32 || emprestado(cpx4_d(0xae1050), p) != 0 {
        return None;
    }
    if cpx4_antes(0xae2c90, inicio) || cpx4_antes(fim, 0xae2c90) {
        return None;
    }
    if contrato(cpx4_d(0xae1050), p) == 0 {
        return None;
    }
    Some(b)
}

/// { 0x8bf400 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH do contrato local): `TRANSFER_MANAGER::
/// add_config_loan(pessoa, clube, começo, fim)`, pelos empréstimos do arquivo de configuração no começo do jogo. Pelas
/// condições (cpx4_pode), o empréstimo vai ao time B ligado ou ao clube: com o fim depois da data +0x46 da pessoa, a
/// data e o fim do contrato (+0x2d) passam ao fim; o contrato local (0x85e410, 0x85e590 com o fim, 0, 1, 1; começo em
/// +0x25) é assinado (0x4d01e0); as suspensões ficam guardadas (0x5563f0); o elenco 1 (0x540980); +0x6d 2 vira 0x10; os
/// números de camisa (0x85add0, 0x85a470); no time B, a pessoa vai às reservas (0x4cc0f0). Precisando de permissão de
/// trabalho no clube (0x8c7560), o fim recebido anda 7 dias e a permissão é dada (0x8c7070 com a nação do clube e o ano
/// do começo).
unsafe extern "thiscall" fn emprestimo_da_configuracao(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let mut local = [0u8; 0x50]; // o STAFF_CONTRACT local (0x50 bytes)
    let l = local.as_mut_ptr() as usize;
    construir(l);
    let b = match cpx4_pode(p, clube, inicio, fim) {
        Some(b) => b,
        None => return,
    };
    if cpx4_antes(p + 0x46, fim) {
        crate::comparacao::ramo("cpx_4 contrato estendido");
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        cpx4_atribuir(p + 0x46, fim); // cpx4: a data da pessoa até o fim
        cpx4_atribuir(contrato(cpx4_d(0xae1050), p) + 0x2d, fim); // cpx4: o fim do contrato até o fim do empréstimo
    }
    let destino = if b != 0 { b } else { clube };
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    preencher(l, p, destino, fim, 0, 1, 1);
    cpx4_atribuir(l + 0x25, inicio);
    crate::comparacao::ramo("cpx_4 emprestimo da configuracao");
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d01e0usize);
    assinar(cpx4_d(0xae1050), l); // cpx4: o contrato de empréstimo
    let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5563f0usize);
    guardar(cpx4_d(0xae28f0), p);
    let elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x540980usize);
    elenco(p, 1);
    if cpx4_b(p + 0x6d) == 2 {
        *((p + 0x6d) as *mut u8) = 0x10;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    if usa(cpx4_d(0xdd7ef0), destino, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85a470usize);
        numerar(cpx4_d(0xdd7ef0), destino, 0);
    }
    if b != 0 && b != clube {
        let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cc0f0usize);
        reservas(cpx4_d(0xae1050), p, b);
    }
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if precisa(this, p, clube, 0) != 0 {
        crate::comparacao::ramo("cpx_4 permissao de trabalho");
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u8; 8];
        let r = mais_dias(fim, t.as_mut_ptr(), 7);
        cpx4_atribuir(fim, r);
        // a original empurra cx sobre o ecx que a 0x417c10 zera: o ano limpo
        let permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
        permissao(this, p, cpx4_d(clube + 0x53), cpx4_w(inicio + 2) as u16 as u32, 1, 1); // cpx4: a permissão de trabalho
    }
}

// ponytail: assina o empréstimo, mexe no elenco e nas permissões (não pode rodar duas vezes): só a original, com o
// plano pelas mesmas condições (passando, a pessoa sai emprestada; senão o empréstimo dela fica como estava).
unsafe extern "thiscall" fn comparar_emprestimo_da_configuracao(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let tinha = emprestado(cpx4_d(0xae1050), p);
    let passa = cpx4_pode(p, clube, inicio, fim).is_some();
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(1323)(this, p, clube, inicio, fim);
    let tem = emprestado(cpx4_d(0xae1050), p);
    let igual = if passa { tem != 0 } else { tem == tinha };
    PLACAR.registrar(igual, || format!("emprestimo_da_configuracao(pessoa {:#x}) passa={} antes={:#x} depois={:#x}", cpx4_d(p), passa, tinha, tem));
}

/// O contrato principal da pessoa como a 0x4da590 o acha (o mesmo da 0x4cf5a0, em linha): sem pessoa, id fora da tabela
/// ou índice negativo no registro de 79 bytes, nenhum; guardado em [0xae1080].
unsafe fn cpx4_contrato(p: usize) -> usize {
    let c = if p == 0 || cpx4_d(p) as i32 >= cpx4_d(0xae2364) as i32 {
        0
    } else {
        let k = cpx4_d(cpx4_d(0xae2d0c).wrapping_add(cpx4_d(p).wrapping_mul(0x4f))) as i32;
        if k < 0 {
            0
        } else {
            cpx4_d(cpx4_d(0xae1050)).wrapping_add((k as usize).wrapping_mul(0x50))
        }
    };
    *(0xae1080 as *mut usize) = c;
    c
}

/// Os clubes interessados na pessoa (Shortlist_Manager 0x842180 com 1, 1, 0) numa LLIST local já construída; devolve os
/// ids, do começo.
unsafe fn cpx4_interessados(p: usize, l: usize) -> Vec<usize> {
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    interessados(cpx4_d(cpx4_d(0xde1f54) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx4_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(cpx4_d(e + 4));
        e = proximo(l);
    }
    v
}

/// { 0x4da590 } (thiscall, ret 0xc, contract_manager.cpp, quadro SEH da LLIST e da notícia locais): `CONTRACT_MANAGER::
/// post_contract_end_news(pessoa, tipo, amplo)`, pelo fim de contrato, de teste ou de empréstimo (0x4c7ac0). Sem
/// contrato ou sem o clube dele: nada. A notícia 0xbb9: o clube do contrato, a pessoa (o id e as três chaves), o tipo, o
/// clube do contrato no tipo 6 (senão −1) e `amplo`. Ampla, com ficha: reputação mundial (+0xd) acima de 7000, a todos
/// os técnicos (0x786780); nacional (+0xb) acima de 7000, aos técnicos da nação da pessoa, da do clube e da do clube
/// atual (+0x39) (0x7863f0) e aos clubes interessados (0x842180) de outras nações. Senão ao clube, ao clube atual no
/// tipo 6 e aos interessados (0x7861e0).
unsafe extern "thiscall" fn noticia_fim_do_contrato(this: usize, p: usize, tipo: u32, amplo: u32) {
    let c = cpx4_contrato(p);
    if c == 0 || cpx4_clube(cpx4_d(c + 4)) == 0 {
        return;
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    crate::comparacao::ramo("cpx_4 noticia do fim do contrato");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbb9, 0);
    let c = cpx4_contrato(p);
    // a original lê o id no endereço 0 sem contrato: não acontece, acabou de achá-lo
    let clube = if c == 0 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    gravar(nt, 0, cpx4_d(clube) as u32);
    gravar(nt, 2, cpx4_d(p) as u32);
    gravar(nt, 3, cpx4_d(cpx4_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx4_d(cpx4_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx4_d(cpx4_d(p + 0xc) + 0x33) as u32);
    let tipo = tipo as u8;
    gravar(nt, 6, tipo as i8 as u32);
    let c = cpx4_contrato(p);
    let clube = if c == 0 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    if tipo == 6 {
        // o clube do contrato pela tabela do próprio gerente (+0), sem guardar em [0xae1080]; sem contrato, a original
        // lê o endereço 4 (não acontece: acabou de achá-lo)
        let k = if p == 0 || cpx4_d(p) as i32 >= cpx4_d(0xae2364) as i32 {
            -1
        } else {
            cpx4_d(cpx4_d(0xae2d0c).wrapping_add(cpx4_d(p).wrapping_mul(0x4f))) as i32
        };
        let v = if k < 0 { cpx4_d(4) } else { cpx4_d(cpx4_d(this).wrapping_add((k as usize).wrapping_mul(0x50)) + 4) };
        gravar(nt, 7, v as u32);
    } else {
        gravar(nt, 7, u32::MAX);
    }
    gravar(nt, 8, amplo);
    let ficha = cpx4_d(p + 0x61);
    let atual = cpx4_d(p + 0x39);
    let nacao = cpx4_d(p + 0x1a);
    let jornal = cpx4_d(0xb7431c);
    let mut padrao = true;
    if amplo != 0 && ficha != 0 {
        if cpx4_w(ficha + 0xd) > 0x1b58 {
            todos(jornal, nt);
            padrao = false;
        } else if cpx4_w(ficha + 0xb) > 0x1b58 {
            padrao = false;
            do_pais(jornal, nt, nacao);
            if nacao != cpx4_d(clube + 0x53) {
                do_pais(jornal, nt, cpx4_d(clube + 0x53));
            }
            if atual != 0 && atual != clube {
                let n = cpx4_d(atual + 0x53);
                if n != cpx4_d(clube + 0x53) && n != nacao {
                    do_pais(jornal, nt, n);
                }
            }
            for id in cpx4_interessados(p, l) {
                if id == cpx4_d(clube) || (tipo == 6 && id == cpx4_d(cpx4_d(p + 0x39))) {
                    continue;
                }
                let outro = cpx4_clube(id);
                let n = cpx4_d(outro + 0x53);
                if n != cpx4_d(p + 0x1a) && n != cpx4_d(cpx4_d(p + 0x39) + 0x53) && n != cpx4_d(clube + 0x53) {
                    entregar(cpx4_d(0xb7431c), nt, outro);
                }
            }
        }
    }
    if padrao {
        entregar(jornal, nt, clube);
        if tipo == 6 {
            entregar(cpx4_d(0xb7431c), nt, cpx4_d(p + 0x39));
        }
        for id in cpx4_interessados(p, l) {
            if id == cpx4_d(clube) || (tipo == 6 && id == cpx4_d(cpx4_d(p + 0x39))) {
                continue;
            }
            entregar(cpx4_d(0xb7431c), nt, cpx4_clube(id));
        }
    }
    soltar(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_fim_do_contrato(this: usize, p: usize, tipo: u32, amplo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1324)(this, p, tipo, amplo)
}

/// { 0x8dda90 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_delay_request_news(oferta, motivo)`, pelo clube que pede para adiar a transferência uma semana. Os
/// clubes humanos que recebem (0x8d1130 com 1, ou 2 fora do estado 0x11; até 30); sem nenhum, nada. A notícia 0xfab: o
/// clube do contrato (sem tipo 5) ou −1, o comprador, a pessoa (o id e as três chaves), o estado, 0, 0, a vaga +0x20 e o
/// motivo; a cada destinatário.
unsafe extern "thiscall" fn noticia_pedido_de_adiamento(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let p = cpx4_pessoa(cpx4_d(o + 0xc));
    let c = contrato(cpx4_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let do_contrato = if c == 0 || cpx4_b(c + 0x35) & 0x3f == 5 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    let comprador = cpx4_clube(cpx4_d(o + 8));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (cpx4_b(o + 0x2c) != 0x11) as u32 + 1, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_4 noticia do adiamento");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfab, 0);
    gravar(nt, 0, if do_contrato != 0 { cpx4_d(do_contrato) as u32 } else { u32::MAX });
    gravar(nt, 1, cpx4_d(comprador) as u32);
    gravar(nt, 2, cpx4_d(p) as u32);
    gravar(nt, 3, cpx4_d(cpx4_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx4_d(cpx4_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx4_d(cpx4_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 6, cpx4_b(o + 0x2c) as i8 as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, 0);
    gravar(nt, 9, cpx4_d(o + 0x20) as u32);
    gravar(nt, 10, motivo as i16 as u32);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(cpx4_d(0xb7431c), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_pedido_de_adiamento(this: usize, o: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1325)(this, o, motivo)
}

// ---------- 260a leva: a tática ativa do clube: a tática do time do clube, a vaga de tática do técnico humano e a tática ativa dele (tactics.cpp, human_manager.cpp) ----------

unsafe fn cpx8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpx8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O técnico pedido ou, nulo, o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
unsafe fn cpx8_tecnico(tecnico: usize) -> usize {
    if tecnico != 0 {
        return tecnico;
    }
    cpx8_d((cpx8_w(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize)
}

/// O índice do registro do técnico humano no gerenciador `this` (id − [0xae2364] + 16), se estiver entre 0 e a conta
/// ([this+4]) − 1; fora disso a original mostra a caixa "Error".
unsafe fn cpx8_indice(this: usize, tecnico: usize) -> Option<usize> {
    let k = cpx8_i(tecnico).wrapping_sub(cpx8_i(0xae2364)).wrapping_add(0x10);
    if k < 0 || k > cpx8_i(this + 4).wrapping_sub(1) {
        return None;
    }
    Some(k as usize)
}

/// O registro do técnico humano `k` (0xa6ef bytes cada, a partir de [this]).
unsafe fn cpx8_registro(this: usize, k: usize) -> usize {
    cpx8_d(this).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// { 0x5fbd00 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::get_active_tactic(tecnico)`, pela escalação
/// (0x6db500), pela formação (0x6e0230), pelo dia de jogo (0x6b4360), pela tática do clube (0x896660) e pelas telas. O
/// bloco de tática (0x1ecd bytes) da vaga ativa (byte com sinal em +0x9a69 do registro) do técnico (nulo: o humano da
/// vez). Sem técnico ou fora da conta: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tatica_ativa_do_humano(this: usize, tecnico: usize) -> usize {
    let t = cpx8_tecnico(tecnico);
    let Some(k) = (if t != 0 { cpx8_indice(this, t) } else { None }) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1328)(this, tecnico); // caixa: só a original
    };
    let vaga = cpx8_b(cpx8_registro(this, k) + 0x9a69) as i8 as i32;
    cpx8_d(this).wrapping_add(vaga.wrapping_mul(0x1ecd) as usize).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// Comparador: a devolução (rotina pura); sem técnico ou fora da conta, só a original (caixa).
unsafe extern "thiscall" fn comparar_tatica_ativa_do_humano(this: usize, tecnico: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1328);
    let t = cpx8_tecnico(tecnico);
    if t == 0 || cpx8_indice(this, t).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = tatica_ativa_do_humano(this, tecnico);
    PLACAR.registrar(r == n, || format!("tatica_ativa_do_humano(tecnico {:#x}) original={:#x} rust={:#x}", tecnico, r, n));
    r
}

/// { 0x5fbe20 } (thiscall, ret 0xc, human_manager.cpp): `HUMAN_MANAGER::select_tactic_slot(reserva, clube, tecnico)`,
/// pelos mesmos chamadores da 0x5fbd00. Técnico nulo: o humano da vez (sem nenhum, nada); clube nulo: o do técnico
/// (+0x39). A vaga ativa (+0x9a69 do registro): 4 na segunda seleção (0x5409b0), 3 com `reserva` numa seleção
/// (0x53b900), 2 numa seleção, 1 com `reserva`, 0 no time principal. Fora da conta: a original (caixa "Error").
unsafe extern "thiscall" fn escolher_vaga_de_tatica(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let t = cpx8_tecnico(tecnico);
    if t == 0 {
        return;
    }
    let Some(k) = cpx8_indice(this, t) else {
        return orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(1327)(this, reserva, clube, tecnico); // caixa: só a original
    };
    let clube = if clube != 0 { clube } else { cpx8_d(t + 0x39) };
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let vaga: u8 = if clube != 0 && segunda(clube) != 0 {
        4
    } else if reserva != 0 && clube != 0 && selecao(clube) != 0 {
        3
    } else if clube != 0 && selecao(clube) != 0 {
        2 // com `reserva` e fora da seleção a original pergunta de novo (0x53b900 duas vezes)
    } else if reserva != 0 {
        1
    } else {
        0
    };
    *((cpx8_registro(this, k) + 0x9a69) as *mut u8) = vaga;
}

/// Comparador: lado a lado pelo byte da vaga ativa do registro; sem técnico (nada) ou fora da conta (caixa), só a
/// original.
unsafe extern "thiscall" fn comparar_escolher_vaga_de_tatica(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(1327);
    let t = cpx8_tecnico(tecnico);
    let Some(k) = (if t != 0 { cpx8_indice(this, t) } else { None }) else {
        return o(this, reserva, clube, tecnico);
    };
    let reg = [(cpx8_registro(this, k) + 0x9a69, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, reserva, clube, tecnico), || escolher_vaga_de_tatica(this, reserva, clube, tecnico));
    PLACAR.registrar(igual, || format!("escolher_vaga_de_tatica(reserva {}, clube {:#x}, tecnico {:#x}) {:x?} x {:x?}", reserva, clube, tecnico, mo, mr));
}

/// { 0x896660 } (thiscall, ret 8, tactics.cpp): `Tactics::get_club_tactic(clube, time)`, pela formação de cada jogo
/// (0x6e0230), pelas fraquezas do jogador (0x5470d0), pela mídia e pelas telas. Clube humano (0x600a50 com 1, 1) cujo
/// técnico (+0xcf) não tem +0x9a01 (0x5fd210) ou tem a opção 2 das férias (0x5fea50): a vaga pelo time (0x5fbe20) e o
/// bloco ativo dele (0x5fbd00). Nos outros, o índice (palavra com sinal) na linha do clube (id × 0x91 em [this+0x601]):
/// abaixo da conta das táticas prontas ([this+0x5fd]) o bloco pronto ([this+0x5f9]), senão o próprio ([this+0x5f1],
/// índice − conta); blocos de 0x18e3 bytes. Clube nulo: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tatica_do_clube(this: usize, clube: usize, time: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1326)(this, clube, time); // caixa: só a original
    }
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        let gerente = cpx8_d(0xb63c98);
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
        let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
        if t9a01(gerente, cpx8_d(clube + 0xcf)) == 0 || opcao(gerente, 2, cpx8_d(clube + 0xcf)) != 0 {
            let escolher: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(0x5fbe20usize);
            let ativa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fbd00usize);
            crate::comparacao::ramo("cpx_8 tatica do humano");
            escolher(gerente, time, clube, cpx8_d(clube + 0xcf)); // cpx8: a vaga pelo time
            return ativa(gerente, cpx8_d(clube + 0xcf));
        }
    }
    let id = cpx8_i(clube);
    let indice = cpx8_w(cpx8_d(this + 0x601).wrapping_add(id.wrapping_mul(0x91) as usize)) as i32;
    let prontas = cpx8_i(this + 0x5fd);
    if indice < prontas { // cpx8: tática pronta ou própria
        crate::comparacao::ramo("cpx_8 tatica pronta");
        cpx8_d(this + 0x5f9).wrapping_add(indice.wrapping_mul(0x18e3) as usize) // cpx8: o bloco pronto
    } else {
        crate::comparacao::ramo("cpx_8 tatica propria");
        cpx8_d(this + 0x5f1).wrapping_add(indice.wrapping_sub(prontas).wrapping_mul(0x18e3) as usize) // cpx8: o bloco próprio
    }
}

/// Comparador: lado a lado pela devolução e, havendo técnico humano no clube, pelo byte da vaga ativa dele (a 0x5fbe20
/// grava). Clube nulo (caixa): só a original.
unsafe extern "thiscall" fn comparar_tatica_do_clube(this: usize, clube: usize, time: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1326);
    if clube == 0 {
        return o(this, clube, time);
    }
    let gerente = cpx8_d(0xb63c98);
    let t = cpx8_d(clube + 0xcf);
    let mut reg = Vec::new();
    if gerente != 0 && t != 0 {
        if let Some(k) = cpx8_indice(gerente, t) {
            reg.push((cpx8_registro(gerente, k) + 0x9a69, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, time), || tatica_do_clube(this, clube, time));
    PLACAR.registrar(igual && r == n, || format!("tatica_do_clube(clube {:#x}, time {}) original={:#x} {:x?} rust={:#x} {:x?}", clube, time, r, mo, n, mr));
    r
}

// ---------- 260a leva: a escalação guardada do clube: tirar o jogador da escalação, trocar duas vagas e a cópia para desfazer (tactics.cpp) ----------

unsafe fn cpx9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpx9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx9_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A linha do clube na tabela tática (0x8982b0, stdcall(clube, humano); −1 sem linha).
unsafe fn cpx9_linha(clube: usize, humano: u32) -> i32 {
    let f: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    f(clube, humano)
}

/// O começo da linha `l` (0x91 bytes cada, a partir de [this+0x601]).
unsafe fn cpx9_inicio(this: usize, l: i32) -> usize {
    cpx9_d(this + 0x601).wrapping_add(l.wrapping_mul(0x91) as usize)
}

/// As 16 cópias para desfazer (0x5c bytes a partir de this+0x30: +0 clube, +4 time, +8 a cópia; conta em +0x5f0): a do
/// clube e do time some (+8 = 0) e a conta desce.
unsafe fn cpx9_soltar_copias(this: usize, clube: usize, time: u32) {
    if cpx9_b(this + 0x5f0) == 0 {
        return;
    }
    for k in 0..0x10usize {
        let e = this + 0x30 + k * 0x5c;
        if cpx9_d(e + 8) != 0 && cpx9_d(e) == clube && cpx9_d(e + 4) as u32 == time {
            cpx9_pd(e + 8, 0);
            *((this + 0x5f0) as *mut u8) = cpx9_b(this + 0x5f0).wrapping_sub(1);
        }
    }
}

/// As regiões da escalação do clube: a linha dele na tabela tática (se houver) e as cópias para desfazer com a conta.
unsafe fn cpx9_regioes(this: usize, clube: usize, time: u32) -> Vec<(usize, usize)> {
    let mut reg = vec![(this + 0x30, 0x5c1)];
    let l = cpx9_linha(clube, time);
    if l != -1 {
        reg.push((cpx9_inicio(this, l), 0x91));
    }
    reg
}

/// { 0x896c60 } (thiscall, ret 0x10, tactics.cpp): `Tactics::remove_player_from_selection(clube, pessoa, time,
/// papeis)`, pelos contratos (troca de lugar no elenco, empréstimo e o fim dele), pelas saídas do elenco, pela escalação
/// automática (0x6dbc50), pelas seleções, pela aposentadoria e pelas transferências. Sem linha tática (0x8982b0): 0. As
/// 20 vagas da escalação (+2 da linha, ids) com a pessoa (id em +0) viram −1. Com `papeis`: as duas vagas extras (+0x56,
/// +0x5a: 0x897dc0 e 0x897ee0) e os batedores das cinco bolas paradas nos dois lados (0x897990 e 0x897bb0) que são ela
/// ficam vazios. As cópias para desfazer do clube e do time somem. Devolve 1 se a tirou de uma vaga ou de bola parada.
/// Clube ou pessoa nulos: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tirar_da_escalacao(this: usize, clube: usize, pessoa: usize, time: u32, papeis: u32) -> u32 {
    if clube == 0 || pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(1329)(this, clube, pessoa, time, papeis); // caixa: só a original
    }
    let l = cpx9_linha(clube, time);
    if l == -1 {
        return 0;
    }
    let mut achou = 0;
    for k in 0..20usize { // cpx9: as vagas da escalação
        let vaga = cpx9_inicio(this, l) + 2 + 4 * k;
        if cpx9_i(vaga) == cpx9_i(pessoa) {
            crate::comparacao::ramo(if k < 11 { "cpx_9 tirou titular" } else { "cpx_9 tirou reserva" });
            cpx9_pd(vaga, u32::MAX); // cpx9: a vaga fica vazia
            achou = 1;
        }
    }
    if papeis != 0 {
        let da_linha: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x897dc0usize);
        let extra: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x897ee0usize);
        // ponytail: a original empurra o tipo e o lado com sobra nos bytes altos (o deslocamento da linha e a pilha);
        // as chamadas leem só o byte baixo, e a cópia passa os valores limpos
        let batedor: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x897990usize);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x897bb0usize);
        for qual in 0..2u32 {
            if da_linha(this, clube, qual, time) == pessoa {
                extra(this, clube, qual, 0, time);
            }
        }
        for tipo in 0..5u32 {
            for lado in 0..2u32 {
                if batedor(this, clube, tipo, lado, time) == pessoa {
                    crate::comparacao::ramo("cpx_9 batedor tirado");
                    marcar(this, clube, tipo, lado, 0, time);
                    achou = 1;
                }
            }
        }
    }
    cpx9_soltar_copias(this, clube, time);
    achou
}

/// Comparador: lado a lado pela devolução, pela linha do clube na tabela tática (as vagas, as extras e os batedores) e
/// pelas cópias para desfazer com a conta. Clube ou pessoa nulos (caixa): só a original.
unsafe extern "thiscall" fn comparar_tirar_da_escalacao(this: usize, clube: usize, pessoa: usize, time: u32, papeis: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(1329);
    if clube == 0 || pessoa == 0 {
        return o(this, clube, pessoa, time, papeis);
    }
    let reg = cpx9_regioes(this, clube, time);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, pessoa, time, papeis), || tirar_da_escalacao(this, clube, pessoa, time, papeis));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_escalacao(clube {:#x}, pessoa {:#x}, time {}, papeis {}) original={} {:x?} rust={} {:x?}", clube, pessoa, time, papeis, r, mo, n, mr));
    r
}

/// { 0x8973e0 } (thiscall, ret 0x10, tactics.cpp): `Tactics::swap_selection_slots(clube, a, b, time)`, pela tela do
/// elenco (0x46835e). Vagas `a` e `b` (bytes com sinal, 0..19): troca os ids das duas vagas da escalação (+2 + 4 × vaga
/// da linha tática, 0x90ba90 com 4 bytes) e as cópias para desfazer do clube e do time somem; devolve 1. Sem linha
/// (0x8982b0): 0. Clube nulo ou vaga fora: a original (caixa "Error", 0).
unsafe extern "thiscall" fn trocar_vagas_da_escalacao(this: usize, clube: usize, a: u32, b: u32, time: u32) -> u32 {
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(1330)(this, clube, a, b, time); // caixa: só a original
    }
    let l = cpx9_linha(clube, time);
    if l == -1 {
        return 0;
    }
    let trocar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x90ba90usize);
    let inicio = cpx9_inicio(this, l);
    trocar(inicio + 2 + 4 * va as usize, inicio + 2 + 4 * vb as usize, 4);
    cpx9_soltar_copias(this, clube, time);
    1
}

/// Comparador: lado a lado pela devolução, pela linha tática do clube e pelas cópias para desfazer. Clube nulo ou vaga
/// fora (caixa): só a original.
unsafe extern "thiscall" fn comparar_trocar_vagas_da_escalacao(this: usize, clube: usize, a: u32, b: u32, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(1330);
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return o(this, clube, a, b, time);
    }
    let reg = cpx9_regioes(this, clube, time);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, a, b, time), || trocar_vagas_da_escalacao(this, clube, a, b, time));
    PLACAR.registrar(igual && r == n, || format!("trocar_vagas_da_escalacao(clube {:#x}, {}, {}, time {}) original={} {:x?} rust={} {:x?}", clube, va, vb, time, r, mo, n, mr));
    r
}

/// { 0x897250 } (thiscall, ret 8, tactics.cpp): `Tactics::has_selection_undo(clube, time)`, pela tela do elenco
/// (0x458a58). Com cópias (+0x5f0) e o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]): a cópia dele (índice
/// [0xae2364] − id − 1, de 0 a 15) existe (+8) e é deste clube (+0) e time (+4): 1; senão 0.
unsafe extern "thiscall" fn tem_copia_da_escalacao(this: usize, clube: usize, time: u32) -> u32 {
    if cpx9_b(this + 0x5f0) == 0 {
        return 0;
    }
    let t = cpx9_d((cpx9_w(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize);
    if t == 0 {
        return 0;
    }
    let k = cpx9_i(0xae2364).wrapping_sub(cpx9_i(t)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        return 0;
    }
    let e = this + 0x30 + k as usize * 0x5c;
    (cpx9_d(e + 8) != 0 && cpx9_d(e) == clube && cpx9_d(e + 4) as u32 == time) as u32
}

/// Comparador: a devolução (rotina pura).
unsafe extern "thiscall" fn comparar_tem_copia_da_escalacao(this: usize, clube: usize, time: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1331)(this, clube, time);
    let n = tem_copia_da_escalacao(this, clube, time);
    PLACAR.registrar(r == n, || format!("tem_copia_da_escalacao(clube {:#x}, time {}) original={} rust={}", clube, time, r, n));
    r
}

// ---------- 260a leva: as notícias do técnico: a diretoria pensa no cargo do técnico, a saída do técnico e o técnico novo (manager_manager.cpp) ----------

unsafe fn cpx10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpx10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A data das notícias de técnico ([0xdd6ec8]: dia com sinal em +0, ano sem sinal em +2) ainda não chegou (é depois de
/// hoje, [0xae2c90])? Então nenhuma notícia sai.
unsafe fn cpx10_antes_da_data() -> bool {
    let (dia, ano) = (cpx10_w(0xdd6ec8), cpx10_w(0xdd6eca) as u16);
    let hoje = cpx10_w(0xae2c92) as u16;
    ano > hoje || (ano == hoje && dia > cpx10_w(0xae2c90))
}

/// A linha do clube na tabela da diretoria (0x49 bytes por id, a partir de [this]).
unsafe fn cpx10_linha(this: usize, clube: usize) -> usize {
    cpx10_d(this).wrapping_add(cpx10_i(clube).wrapping_mul(0x49) as usize)
}

unsafe fn cpx10_gravar(n: usize, k: u32, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

/// O id e as três chaves (+0x33 de +4, +8 e +0xc) da pessoa `p` nos dados `k`..`k`+3 da notícia.
unsafe fn cpx10_gravar_pessoa(n: usize, k: u32, p: usize) {
    cpx10_gravar(n, k, cpx10_d(p) as u32);
    cpx10_gravar(n, k + 1, cpx10_d(cpx10_d(p + 4) + 0x33) as u32);
    cpx10_gravar(n, k + 2, cpx10_d(cpx10_d(p + 8) + 0x33) as u32);
    cpx10_gravar(n, k + 3, cpx10_d(cpx10_d(p + 0xc) + 0x33) as u32);
}

/// O técnico humano (uma das 16 últimas pessoas: id ≥ [0xae2364] − 16)?
unsafe fn cpx10_humano(p: usize) -> bool {
    cpx10_i(p) >= cpx10_i(0xae2364).wrapping_sub(0x10)
}

/// Os dois nomes do técnico humano (os textos de +4 e +8, 0x7858c0) a partir do dado `primeiro`; o próximo dado livre
/// depois do primeiro nome vai no dado `k_proximo`.
unsafe fn cpx10_nomes(n: usize, p: usize, primeiro: u8, k_proximo: u32) {
    let texto: unsafe extern "thiscall" fn(usize, u32, usize, *mut u8) -> u32 = core::mem::transmute(0x7858c0usize);
    // ponytail: a original guarda o próximo dado no byte baixo de uma local que tinha a data [0xdd6ec8] e empurra o
    // dword inteiro como campo; a 0x7858c0 lê só o byte baixo, e a cópia passa o valor limpo
    let mut proximo = primeiro;
    texto(n, primeiro as u32, cpx10_d(p + 4), &mut proximo);
    cpx10_gravar(n, k_proximo, proximo as i8 as i32 as u32);
    texto(n, proximo as u32, cpx10_d(p + 8), &mut proximo);
}

/// O ensaio do comparador: a cópia roda sem entregar a notícia (as entregas copiam a notícia nas caixas dos técnicos
/// humanos, alocam e podem dar a resposta automática) e guarda o alcance que daria (+0xde: 4 todos, 2 os do país, 1 os
/// da divisão; 0 nenhuma) e a notícia montada.
static mut CPX10_ENSAIO: bool = false;
static mut CPX10_ALCANCE: u8 = 0;
static mut CPX10_NOTICIA: [u8; 0xe0] = [0; 0xe0];

/// A entrega da notícia `n` pelo `alcance` (4: 0x786780; 2: 0x7863f0 com a nação; 1: 0x7862d0 com a divisão) pelas
/// notícias [0xb7431c]; no ensaio, só o plano.
unsafe fn cpx10_entregar(n: usize, alcance: u8, alvo: usize) {
    if CPX10_ENSAIO {
        CPX10_ALCANCE = alcance;
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(CPX10_NOTICIA) as *mut u8, 0xe0);
        return;
    }
    let noticias = cpx10_d(0xb7431c);
    match alcance {
        4 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
            f(noticias, n);
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
            f(noticias, n, alvo);
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
            f(noticias, n, alvo);
        }
    }
}

/// A entrega das notícias de saída e de técnico novo de um técnico da máquina: clube que pode ser dirigido (0x544820);
/// seleção (0x53b900) ou reputação (+0x80) acima de 7250: a todos; senão aos do país (+0x53), se houver.
unsafe fn cpx10_entregar_pelo_clube(n: usize, clube: usize) {
    let pode: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if pode(clube) == 0 {
        return;
    }
    if selecao(clube) != 0 || cpx10_w(clube + 0x80) > 0x1c52 {
        cpx10_entregar(n, 4, 0);
    } else {
        let nacao = cpx10_d(clube + 0x53);
        if nacao != 0 {
            cpx10_entregar(n, 2, nacao);
        }
    }
}

/// O comparador das três: a cópia em ensaio (sem entregar) e a original a partir do mesmo estado (as `regioes` e o
/// sorteio). Iguais: as `regioes`; os sorteios da cópia são o começo dos da original (a resposta automática da entrega
/// pode sortear depois); e cada técnico humano cuja caixa (News +8: 12 bytes por humano, início do anel em +0, próxima
/// vaga em +4) andou recebeu a notícia da cópia: tipo (+0), dados e data (+5..+0xd6) e o alcance (+0xde); sem entrega
/// planejada, nenhuma caixa anda.
unsafe fn cpx10_comparar(nome: &str, regioes: Vec<(usize, usize)>, original: impl FnOnce(), copia: impl FnOnce()) {
    let noticias = cpx10_d(0xb7431c);
    let caixas = if noticias != 0 { cpx10_d(noticias + 8) } else { 0 };
    let antes_caixas: Vec<(i32, i32)> = if caixas != 0 { (0..0x10).map(|k| (cpx10_i(caixas + 12 * k), cpx10_i(caixas + 12 * k + 4))).collect() } else { Vec::new() };
    let mut todas = regioes.clone();
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    CPX10_ENSAIO = true;
    CPX10_ALCANCE = 0;
    let (_, rr) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(copia));
    CPX10_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    let (alcance, noticia) = (CPX10_ALCANCE, *core::ptr::addr_of!(CPX10_NOTICIA));
    antes.escrever();
    let (_, ro) = crate::aleatorio::rastrear(original);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let mut entregues = 0;
    let mut noticia_igual = true;
    let caixas = if noticias != 0 { cpx10_d(noticias + 8) } else { 0 };
    if caixas != 0 && antes_caixas.len() == 0x10 {
        for (k, &(inicio, vaga)) in antes_caixas.iter().enumerate() {
            let e = caixas + 12 * k;
            if cpx10_i(e + 4) == vaga && cpx10_i(e) == inicio {
                continue;
            }
            entregues += 1;
            let v = if inicio == -1 { cpx10_i(e) } else { vaga }; // a primeira notícia abre o anel no início
            let s = cpx10_d(noticias).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != noticia[0..4] || g[5..0xd6] != noticia[5..0xd6] || g[0xde] != alcance {
                noticia_igual = false;
            }
        }
    }
    let igual = mo == mr && ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && noticia_igual && (alcance != 0 || entregues == 0);
    PLACAR.registrar(igual, || format!("{} original {:x?} sorteios {:?} rust {:x?} sorteios {:?}; alcance {} entregues {} noticia igual {}", nome, mo, ro, mr, rr, alcance, entregues, noticia_igual));
}

/// { 0x6a0e70 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::post_manager_under_pressure_news(clube, tipo)`,
/// por pensar em demitir o técnico (0x69aba0: tipo 1 na troca pelo melhor candidato, 2 depois de 14 dias). Clube sem
/// técnico (+0xcf) ou antes da data [0xdd6ec8]: nada. A notícia 0x1777: o técnico (0..3), o clube (4), o presidente
/// (+0xbf: 5..7 e o id em 10; sem ele, −1). O motivo (8) pela linha da diretoria (+6 a confiança): avaliação +8, +0xa ou
/// +0xc abaixo da confiança − 500 com sorteio(3) zero dá 2, 1 ou 3 (+1 com sorteio(5) não zero); senão 0. O tipo (9) e
/// o boato (11): sorteio(10), ou 1 se o técnico conhece bem o clube (0x544410 do não jogador +0x69 com +0x1b acima de
/// 10). Entrega: técnico humano, a todos; clube de reputação acima de 5000, aos do país; senão aos da divisão (+0x57)
/// que não seja a [0x9cf764]. Por fim a data de hoje em +0x18 da linha (0x417c10).
unsafe extern "thiscall" fn noticia_da_pressao(this: usize, clube: usize, tipo: u32) {
    if clube == 0 {
        return;
    }
    let tec = cpx10_d(clube + 0xcf);
    if tec == 0 || cpx10_antes_da_data() {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1777, 0);
    cpx10_gravar_pessoa(n, 0, tec);
    cpx10_gravar(n, 4, cpx10_d(clube) as u32);
    let pres = cpx10_d(clube + 0xbf);
    if pres != 0 {
        cpx10_gravar(n, 5, cpx10_d(cpx10_d(pres + 4) + 0x33) as u32);
        cpx10_gravar(n, 6, cpx10_d(cpx10_d(pres + 8) + 0x33) as u32);
        cpx10_gravar(n, 7, cpx10_d(cpx10_d(pres + 0xc) + 0x33) as u32);
        cpx10_gravar(n, 0xa, cpx10_d(pres) as u32);
    } else {
        for k in [5, 6, 7, 0xa] {
            cpx10_gravar(n, k, u32::MAX);
        }
    }
    let l = cpx10_linha(this, clube);
    let motivo: i8 = if (cpx10_w(l + 8) as i32) < cpx10_w(l + 6) as i32 - 500 /* cpx10: o limite pela avaliação +8 */ && {
        crate::comparacao::ramo("cpx_10 motivo pela avaliacao");
        cpx10_sortear(3) == 0
    } {
        2
    } else if (cpx10_w(l + 0xa) as i32) < cpx10_w(l + 6) as i32 - 500 && cpx10_sortear(3) == 0 {
        1
    } else if (cpx10_w(l + 0xc) as i32) < cpx10_w(l + 6) as i32 - 500 && cpx10_sortear(3) == 0 {
        3 + (cpx10_sortear(5) != 0) as i8
    } else {
        0
    };
    cpx10_gravar(n, 8, motivo as i32 as u32);
    cpx10_gravar(n, 9, tipo as u8 as i8 as i32 as u32);
    crate::comparacao::ramo("cpx_10 sorteio do boato");
    let mut boato = cpx10_sortear(10); // cpx10: o sorteio do boato
    let nj = cpx10_d(tec + 0x69);
    if nj != 0 {
        let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
        if conhecer(nj, cpx10_b(nj + 0x1b) as i8) as u8 as i8 > 10 {
            boato = 1;
        }
    }
    cpx10_gravar(n, 0xb, boato as u32);
    if cpx10_humano(tec) {
        cpx10_entregar(n, 4, 0);
    } else if cpx10_w(clube + 0x80) > 0x1388 {
        cpx10_entregar(n, 2, cpx10_d(clube + 0x53));
    } else {
        let div = cpx10_d(clube + 0x57);
        if div != 0 && cpx10_i(div) != cpx10_i(0x9cf764) {
            cpx10_entregar(n, 1, div);
        }
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut data = [0u32; 2];
    crate::comparacao::ramo("cpx_10 data da pressao");
    copiar(cpx10_linha(this, clube) + 0x18, data.as_mut_ptr() as usize, 0xae2c90); // cpx10: a data da notícia na linha
}

unsafe extern "thiscall" fn comparar_noticia_da_pressao(this: usize, clube: usize, tipo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1332);
    let regioes = if clube != 0 { vec![(cpx10_linha(this, clube), 0x49)] } else { Vec::new() };
    cpx10_comparar("noticia_da_pressao", regioes, || o(this, clube, tipo), || noticia_da_pressao(this, clube, tipo));
}

/// { 0x6a1b00 } (thiscall, ret 0x14, manager_manager.cpp): `MANAGER_MANAGER::post_manager_departure_news(tecnico, tipo,
/// motivo, clube, cargo)`, por tirar o técnico do cargo (0x69b790). Técnico ou clube nulos, antes da data [0xdd6ec8] ou
/// tipo 5: nada. A notícia 6000: o técnico (0..3), o tipo (4), o clube (5), o cargo (7), as chaves do presidente (+0xbf:
/// 8..10; sem ele, −1) e o motivo (11). Técnico humano: os nomes dele a partir do dado 13 (o próximo livre no 12) e a
/// todos; senão pelo clube (0x544820; seleção ou reputação acima de 7250 a todos, senão aos do país).
unsafe extern "thiscall" fn noticia_da_saida_do_tecnico(this: usize, tec: usize, tipo: u32, motivo: u32, clube: usize, cargo: u32) {
    let _ = this;
    if tec == 0 || clube == 0 || cpx10_antes_da_data() {
        return;
    }
    let t = tipo as u8 as i8;
    if t == 5 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1770, 0);
    cpx10_gravar_pessoa(n, 0, tec);
    cpx10_gravar(n, 4, t as i32 as u32);
    cpx10_gravar(n, 5, cpx10_d(clube) as u32);
    cpx10_gravar(n, 7, cargo as u8 as i8 as i32 as u32);
    let pres = cpx10_d(clube + 0xbf);
    for (k, d) in [(8, 4), (9, 8), (0xa, 0xc)] {
        cpx10_gravar(n, k, if pres != 0 { cpx10_d(cpx10_d(pres + d) + 0x33) as u32 } else { u32::MAX });
    }
    cpx10_gravar(n, 0xb, motivo as u8 as i8 as i32 as u32);
    crate::comparacao::ramo("cpx_10 noticia da saida");
    if cpx10_humano(tec) {
        cpx10_nomes(n, tec, 0xd, 0xc);
        cpx10_entregar(n, 4, 0);
    } else {
        cpx10_entregar_pelo_clube(n, clube);
    }
}

unsafe extern "thiscall" fn comparar_noticia_da_saida_do_tecnico(this: usize, tec: usize, tipo: u32, motivo: u32, clube: usize, cargo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(1333);
    cpx10_comparar("noticia_da_saida_do_tecnico", Vec::new(), || o(this, tec, tipo, motivo, clube, cargo), || noticia_da_saida_do_tecnico(this, tec, tipo, motivo, clube, cargo));
}

/// { 0x6a2e00 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::post_manager_appointment_news(tecnico,
/// anterior, clube, cargo)`, pelo dia do mercado de técnicos (0x68f9c0) e pela nomeação (0x69ebc0). Técnico ou clube
/// nulos, antes da data [0xdd6ec8] ou cargo 5: nada. Só clube de nação ativa (+0x11c da nação) com a divisão no vetor
/// [0xadadfc], ou seleção (0x53b900) de nação ativa. A notícia 0x1787: o técnico (0..3), o clube anterior (4; −1), o
/// clube (5), o cargo (6) e a função anterior (7: +0x28 numa seleção, +0x3d num clube; sem anterior, −1). Técnico
/// humano: os nomes a partir do dado 8 (o próximo livre no 7) e a todos; senão pelo clube, como a da saída.
unsafe extern "thiscall" fn noticia_do_tecnico_novo(this: usize, tec: usize, anterior: usize, clube: usize, cargo: u32) {
    let _ = this;
    if tec == 0 || clube == 0 || cpx10_antes_da_data() {
        return;
    }
    let c = cargo as u8 as i8;
    if c == 5 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nacao = cpx10_d(clube + 0x53);
    let ativa = nacao != 0 && cpx10_b(nacao + 0x11c) != 0 && {
        let div = cpx10_d(clube + 0x57);
        div != 0 && cpx10_d(cpx10_d(0xadadfc).wrapping_add(cpx10_i(div).wrapping_mul(4) as usize)) != 0
    };
    // a original lê +0x11c da nação da seleção sem conferir a nação (toda seleção tem uma)
    if !ativa && !(selecao(clube) != 0 && cpx10_b(cpx10_d(clube + 0x53) + 0x11c) != 0) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1787, 0);
    cpx10_gravar_pessoa(n, 0, tec);
    cpx10_gravar(n, 4, if anterior != 0 { cpx10_d(anterior) as u32 } else { u32::MAX });
    cpx10_gravar(n, 5, cpx10_d(clube) as u32);
    cpx10_gravar(n, 6, c as i32 as u32);
    let funcao = if anterior == 0 {
        u32::MAX
    } else if selecao(anterior) != 0 {
        cpx10_b(tec + 0x28) as i8 as i32 as u32
    } else {
        cpx10_b(tec + 0x3d) as i8 as i32 as u32
    };
    cpx10_gravar(n, 7, funcao);
    crate::comparacao::ramo("cpx_10 noticia do tecnico novo");
    if cpx10_humano(tec) {
        cpx10_nomes(n, tec, 8, 7);
        cpx10_entregar(n, 4, 0);
    } else {
        cpx10_entregar_pelo_clube(n, clube);
    }
}

unsafe extern "thiscall" fn comparar_noticia_do_tecnico_novo(this: usize, tec: usize, anterior: usize, clube: usize, cargo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(1334);
    cpx10_comparar("noticia_do_tecnico_novo", Vec::new(), || o(this, tec, anterior, clube, cargo), || noticia_do_tecnico_novo(this, tec, anterior, clube, cargo));
}

// ---------- 260a leva: o fim da temporada da liga: os prêmios em dinheiro, os registros pela situação do clube e a notícia da situação (league.cpp) ----------

unsafe fn cpx11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de finanças do clube (FinanceTable::get_club_finance 0x5a0590 de [0xae2a38]).
unsafe fn cpx11_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cpx11_d(0xae2a38), clube)
}

/// O ensaio do comparador da 0x682e10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca e não pode rodar duas
/// vezes).
static mut CPX11_ENSAIO: bool = false;

/// O divisor dos prêmios: 1 + 2 + … + n (16 bits, como o ax da original).
unsafe fn cpx11_soma_das_posicoes(n: i16) -> i32 {
    let (mut s, mut k) = (0i32, 0i32);
    if n > 0 {
        loop {
            s = s.wrapping_add(k).wrapping_add(1);
            k += 1;
            if k as i16 >= n {
                break;
            }
        }
    }
    s as i16 as i32
}

/// { 0x682e10 } (thiscall, ret 0, league.cpp): `League::pay_prize_money()`: com +0xe1 = 2, nada. Com +0xe1 = 3, a
/// competição ([0xadadfc] pelo id de +4) paga pela virtual +0x6c (sem a tabela das competições: a caixa da linha
/// 0x1df). Senão o fundo (+0x28) dividido por 1 + 2 + … + n (n = +0x3e linhas da tabela) é a parte; o clube da linha i
/// (+0xb1, 0x3b cada) recebe (n − i) partes no registro de finanças (0x5a0590; 0x5999a0 soma no saldo, em +0xa8 e
/// +0x148) e, acima de 1000, a notícia do prêmio (0x48e3e0 com a posição i + 1).
unsafe extern "thiscall" fn pagar_premios_da_liga(liga: usize) {
    let e1 = cpx11_b(liga + 0xe1);
    if e1 == 2 {
        return;
    }
    if e1 == 3 {
        let t = cpx11_d(0xadadfc);
        if t == 0 {
            return aviso_em(0x9f15f4, 0x1df);
        }
        let comp = cpx11_d(t.wrapping_add(cpx11_d(cpx11_d(liga + 4)).wrapping_mul(4)));
        let pagar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cpx11_d(cpx11_d(comp) + 0x6c));
        return pagar(comp, liga);
    }
    let s = cpx11_soma_das_posicoes(cpx11_w(liga + 0x3e));
    let fundo = cpx11_d(liga + 0x28) as i32;
    if s == 0 || (fundo == i32::MIN && s == -1) {
        // ponytail: a original divide por zero (ou estoura) e cai; só a original, para cair do mesmo jeito
        return orig::<unsafe extern "thiscall" fn(usize)>(1335)(liga);
    }
    let parte = fundo / s;
    if cpx11_w(liga + 0x3e) <= 0 {
        return;
    }
    let somar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, i32) = core::mem::transmute(0x48e3e0usize);
    let mut i: i16 = 0;
    loop {
        let valor = (cpx11_w(liga + 0x3e) as i32).wrapping_sub(i as i32).wrapping_mul(parte);
        let clube = cpx11_d(cpx11_d(liga + 0xb1).wrapping_add(0x3b * i as usize));
        let fin = cpx11_financas(clube);
        crate::comparacao::ramo("cpx_11 premio pago");
        somar(fin, valor); // cpx11: o prêmio
        if valor > 1000 && !CPX11_ENSAIO {
            // ponytail: a original empurra a posição no dl e o resto do edx (29 × i, sobra da conta da linha); a
            // 0x48e3e0 lê só o byte
            let pos = ((29 * i as i32 as u32) & !0xff) | (i as u8).wrapping_add(1) as u32;
            noticia(liga, clube, valor as u32, (valor >> 31) as u32, pos, 0, 0, -2);
        }
        i = i.wrapping_add(1);
        if i >= cpx11_w(liga + 0x3e) {
            break;
        }
    }
}

/// Comparador: com +0xe1 = 3 (a virtual da competição paga por conta própria) ou o divisor zero: só a original. Nos
/// outros casos, o ensaio: a cópia sem a notícia do prêmio (0x48e3e0 aloca) e a original a partir do mesmo estado, nos
/// registros de finanças (0x167 bytes) de todos os clubes da tabela.
unsafe extern "thiscall" fn comparar_pagar_premios_da_liga(liga: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1335);
    let n = cpx11_w(liga + 0x3e);
    let e1 = cpx11_b(liga + 0xe1);
    if e1 == 3 || cpx11_soma_das_posicoes(n) == 0 {
        return o(liga); // ponytail: a virtual ou a divisão por zero, só a original
    }
    let mut regioes: Vec<(usize, usize)> = Vec::new();
    if e1 != 2 {
        for i in 0..n.max(0) as usize {
            let fin = cpx11_financas(cpx11_d(cpx11_d(liga + 0xb1) + 0x3b * i));
            if fin != 0 && !regioes.iter().any(|r| r.0 == fin) {
                regioes.push((fin, 0x167));
            }
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pagar_premios_da_liga(liga)));
    CPX11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(liga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pagar_premios_da_liga({:x}) +0xe1 {} n {} original={:x?} rust={:x?}", liga, e1, n, mo, mr));
}

/// { 0x686940 } (thiscall, ret 0x18, league.cpp): `League::update_staff_records_for_status(clube, situação, fase,
/// saída, _, _)` (a virtual +0x44 da liga): situação 5, o título (Staff_Records 0x868c50 de [0xdd7f00] com o clube e
/// a competição +4); 0, o acesso (0x869480 com peso 100); 3, o rebaixamento (0x86a1c0). Devolve 0 e não grava a saída.
unsafe extern "thiscall" fn registros_pela_situacao_na_liga(liga: usize, clube: usize, situacao: u32, _fase: u32, _saida: usize, _a: u32, _b: u32) -> u32 {
    let registros = cpx11_d(0xdd7f00);
    let comp = cpx11_d(liga + 4);
    match situacao as u8 {
        5 => {
            crate::comparacao::ramo("cpx_11 registro do titulo");
            let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
            titulo(registros, clube, comp);
        }
        0 => {
            crate::comparacao::ramo("cpx_11 registro do acesso");
            let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
            acesso(registros, clube, comp, 0x64);
        }
        3 => {
            crate::comparacao::ramo("cpx_11 registro do rebaixamento");
            let rebaixamento: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
            rebaixamento(registros, clube, comp);
        }
        _ => {}
    }
    0
}

// ponytail: os três registros acrescentam ao histórico das pessoas e mexem na reputação (alocam): só a original; a
// cópia leva os ramos.
unsafe extern "thiscall" fn comparar_registros_pela_situacao_na_liga(liga: usize, clube: usize, situacao: u32, fase: u32, saida: usize, a: u32, b: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(1336)(liga, clube, situacao, fase, saida, a, b)
}

/// { 0x686670 } (thiscall, ret 0x14, league.cpp, quadro SEH sem destrutor): `League::announce_team_status(clube,
/// situação, fase, _, _)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o
/// clube, a situação, a fase e a saída (0xff); −1 dela: 0. A rodada (+0x3a; com fases, +0x44 = 0xff, e fase ≠ 0xff:
/// a da fase). A notícia 0xc (local): 0 o clube, 1 a competição, 2 a fase, 3 a situação, 4 a rodada, 5 o que a
/// virtual devolveu. Saída ≠ 0xff: a entrega pelo alcance (0x786090 com a nação do clube e o continente). Senão pela
/// situação: 5, à nação da competição (0x7863f0) sem o +0x1c, à divisão (0x7862d0) com ele; 0 e 3, à divisão e à
/// divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, se houver; 1 e 2, à divisão; 6, 7 e 0xfe, ao
/// técnico do clube (0x7861e0). Devolve 1.
unsafe extern "thiscall" fn anunciar_situacao_na_liga(liga: usize, clube: usize, situacao: u32, fase: u32, _a: u32, _b: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let pelo_alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    // ponytail: a original grava só o byte da saída num dword da pilha e empurra o dword inteiro para a 0x786090, que
    // lê o byte; a cópia passa o alto zerado
    let mut saida: u32 = 0xff;
    let mut rodada = cpx11_w(liga + 0x3a);
    if amistoso(cpx11_d(0xae2c74), cpx11_d(liga + 4)) != 0 {
        return 0;
    }
    let comp = cpx11_d(cpx11_d(0xadadfc).wrapping_add(cpx11_d(cpx11_d(liga + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, u32, u32) -> i32 = core::mem::transmute(cpx11_d(cpx11_d(comp) + 0x44));
    let r = virtual44(comp, clube, situacao, fase, &mut saida, 0, 0);
    if r == -1 {
        return 0;
    }
    if cpx11_b(liga + 0x44) == 0xff && fase as u8 != 0xff {
        let f = cpx11_d(cpx11_d(liga + 0xc).wrapping_add((fase as u8 as i8 as isize as usize).wrapping_mul(4)));
        rodada = cpx11_w(f + 0x3a);
    }
    crate::comparacao::ramo("cpx_11 noticia da situacao");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xc, 0);
    gravar(n, 0, cpx11_d(clube) as u32);
    gravar(n, 1, cpx11_d(cpx11_d(liga + 4)) as u32);
    gravar(n, 2, fase as u8 as i8 as i32 as u32);
    gravar(n, 3, situacao as u8 as i8 as i32 as u32);
    gravar(n, 4, rodada as i32 as u32);
    gravar(n, 5, r as u32);
    let noticias = cpx11_d(0xb7431c);
    if saida as u8 != 0xff {
        let nacao = cpx11_d(clube + 0x53);
        let continente = if nacao != 0 { cpx11_d(nacao + 0x71) } else { 0 };
        pelo_alcance(noticias, n, clube, cpx11_d(liga + 4), nacao, continente, saida);
        return 1;
    }
    // a divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, registros de 0x6b em [0xae23d0]
    let outra = |desl: usize| {
        let c = cpx11_d(cpx11_d(0xadadfc).wrapping_add(cpx11_d(cpx11_d(liga + 4)).wrapping_mul(4)));
        let e = cpx11_d(c + desl) as i32;
        if e != -1 {
            a_divisao(noticias, n, cpx11_d(0xae23d0).wrapping_add((e as usize).wrapping_mul(0x6b)));
        }
    };
    match situacao as u8 {
        5 => {
            if cpx11_d(liga + 0x1c) as i32 == -1 {
                a_nacao(noticias, n, cpx11_d(cpx11_d(liga + 4) + 0x5d));
            } else {
                a_divisao(noticias, n, cpx11_d(liga + 4));
            }
        }
        0 => {
            a_divisao(noticias, n, cpx11_d(liga + 4));
            outra(0x1c);
        }
        7 | 6 | 0xfe => {
            ao_clube(noticias, n, clube);
        }
        1 | 2 => {
            a_divisao(noticias, n, cpx11_d(liga + 4));
        }
        3 => {
            a_divisao(noticias, n, cpx11_d(liga + 4));
            outra(0x20);
        }
        _ => {}
    }
    1
}

// ponytail: a virtual +0x44 acrescenta registros e a notícia vai às caixas dos técnicos (alocam): só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_anunciar_situacao_na_liga(liga: usize, clube: usize, situacao: u32, fase: u32, a: u32, b: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(1337)(liga, clube, situacao, fase, a, b)
}

// ---------- 260a leva: as estatísticas do jogo e os recordes do clube: o jogo nas estatísticas dos jogadores, os recordes de posição na liga e as notícias dos recordes (player_stats.cpp, club_records.cpp) ----------

unsafe fn cpx12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx12_mais_b(a: usize, v: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(v);
}

unsafe fn cpx12_mais_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, core::ptr::read_unaligned(a as *const u16).wrapping_add(v));
}

unsafe fn cpx12_mais_f(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v + core::ptr::read_unaligned(a as *const f64));
}

unsafe fn cpx12_registro(this: usize, jogador: usize, cat: u8) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    f(this, jogador, cat as u32)
}

unsafe fn cpx12_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx12_d(0xae1050), pessoa)
}

/// A categoria das estatísticas do lado `lado` da partida (o bl da 0x7c0d70, partida com competição ≠ −2): amistoso
/// (0x5c6710 de [0xae2c74]), 5 se o mandante é seleção (0x53b900), senão 1; reservas, sub-21 e B
/// ([0x9cf684], [0x9cf798], [0x9cf794]) ou o clube do lado sem a marca +0x40 (mandante) / +0x41 (visitante): 1. Pelo
/// tipo (+0x42) da competição ([0xadadfc] pelo id +0): 4 → 5, 2 → 4, 1 → 2 se ela conta (0x4b0700), senão 3. Sem a
/// competição ou outro tipo: a caixa (linhas 0x103a, 0x105d; só com `avisar`), `erro` e 2.
unsafe fn cpx12_categoria(p: usize, lado: i32, avisar: bool, erro: &mut bool) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let clube = cpx12_d(p + if lado == 0 { 0x1c } else { 0x20 });
    if amistoso(cpx12_d(0xae2c74), cpx12_d(p + 0x14)) != 0 {
        return if selecao(cpx12_d(p + 0x1c)) != 0 { 5 } else { 1 };
    }
    let id = cpx12_d(cpx12_d(p + 0x14));
    if id == cpx12_d(0x9cf684) || id == cpx12_d(0x9cf798) || id == cpx12_d(0x9cf794) {
        return 1;
    }
    if clube == cpx12_d(p + 0x1c) && cpx12_b(p + 0x40) == 0 {
        return 1;
    }
    if clube == cpx12_d(p + 0x20) && cpx12_b(p + 0x41) == 0 {
        return 1;
    }
    let comp = cpx12_d(cpx12_d(0xadadfc).wrapping_add(cpx12_d(p).wrapping_mul(4)));
    if comp == 0 {
        *erro = true;
        if avisar {
            aviso_em(0xa78310, 0x103a);
        }
        return 2;
    }
    match cpx12_b(comp + 0x42) {
        4 => 5,
        2 => 4,
        1 => {
            if avisar {
                crate::comparacao::ramo("cpx_12 categoria da liga");
            }
            if conta(cpx12_d(p) as i32) != 0 { 2 } else { 3 } // cpx12: a categoria da liga
        }
        _ => {
            *erro = true;
            if avisar {
                aviso_em(0xa78310, 0x105d);
            }
            2
        }
    }
}

/// A linha `k` (0..20, 0x49 bytes; 11.. reservas) do lado `lado` do jogo conta: reserva que entrou (+0x32 ≠ 0xff),
/// pessoa (+0) e jogador (+4) ≠ −1 e a pessoa na tabela ([0xae23bc], 0x6e; id abaixo de [0xae2364]). Devolve
/// (linha, pessoa, dados do jogador em [0xae23c4], 0x46).
unsafe fn cpx12_linha(jogo: usize, lado: usize, k: usize) -> Option<(usize, usize, usize)> {
    let e = jogo + 0x625 * lado + 0x190 + 0x49 * k;
    if k > 10 && cpx12_b(e + 0x32) == 0xff {
        return None;
    }
    let (sid, pid) = (cpx12_d(e) as i32, cpx12_d(e + 4) as i32);
    if sid == -1 || pid == -1 {
        return None;
    }
    let pessoa = cpx12_d(0xae23bc).wrapping_add((sid as usize).wrapping_mul(0x6e));
    if pessoa != 0 && cpx12_d(pessoa) as i32 >= cpx12_d(0xae2364) as i32 {
        return None;
    }
    Some((e, pessoa, cpx12_d(0xae23c4).wrapping_add((pid as usize).wrapping_mul(0x46))))
}

/// As categorias de cada passada: a do jogo e a geral (6), que fica de fora no amistoso de clube (1) e no de seleção
/// (5).
fn cpx12_categorias(cat: u8) -> [Option<u8>; 2] {
    [Some(cat), if cat == 5 || cat == 1 { None } else { Some(6) }] // cpx12: a categoria geral
}

/// { 0x7c0d70 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::record_match(partida, jogo)`: partida ou jogo nulos,
/// as caixas e 0. Competição −2 (jogo simulado): a categoria é sorteio(9) ≠ 0 ? 2 : 3 para os dois lados; senão a de
/// cada lado (cpx12_categoria). Em cada linha que conta (cpx12_linha): a nota (+0x35) e um jogo nas somas em double
/// do this (+0x20/+0x28 com a marca +0x4c do jogo, senão +0x10/+0x18); nos registros de 0x20 bytes das categorias
/// (0x7c3920, ou criado pela 0x7c5d80): jogos, entradas, gols/assistências e o resto da linha (+0x21..+0x2d), gols a
/// favor (+0x11f do lado) e contra (+0x744 − lado), vitória e derrota, melhor em campo (+0x3c), as palavras
/// (+0x35, +0x3e, +0x3f, +0x42, +0x28, +0x25, +0x26) e os minutos (+0x34, +0x30 ou a duração +0x4d do jogo; o
/// reserva desconta a entrada +0x32). Na categoria 2 ou com a competição contando (0x4b0700): +1 jogo e os gols no
/// registro do jogador ([this+0x30], 0x27 bytes) e, sem ficha (0x4cf6d0), +1 no +0x3b do contrato (0x4cf5a0). Por
/// fim a forma (+0x24..+0x26) anda meio byte e recebe a nota. Devolve 1.
unsafe extern "thiscall" fn registrar_partida_nas_estatisticas(this: usize, p: usize, jogo: usize) -> u32 {
    if p == 0 || jogo == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1338)(this, p, jogo); // as caixas
    }
    let criar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c5d80usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let simulado = cpx12_d(p) as i32 == -2;
    let mut cat: u8 = 0;
    if simulado {
        cat = if sorteio(9) != 0 { 2 } else { 3 };
    }
    let mut contou = false;
    for lado in 0..2usize {
        if !simulado {
            let mut erro = false;
            cat = cpx12_categoria(p, lado as i32, true, &mut erro);
        }
        let gf = jogo + 0x625 * lado + 0x11f;
        let gc = jogo + 0x744 - 0x625 * lado;
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = cpx12_linha(jogo, lado, k) else { continue };
            let nota = cpx12_b(e + 0x35);
            if cpx12_b(jogo + 0x4c) != 0 {
                cpx12_mais_f(this + 0x20, nota as i32 as f64);
                cpx12_mais_f(this + 0x28, 1.0);
            } else {
                cpx12_mais_f(this + 0x10, nota as i32 as f64);
                cpx12_mais_f(this + 0x18, 1.0);
            }
            if !contou {
                contou = true;
                crate::comparacao::ramo("cpx_12 jogo registrado");
            }
            for c in cpx12_categorias(cat).into_iter().flatten() {
                let mut r = cpx12_registro(this, dados, c);
                if r == 0 {
                    r = criar(this, dados, c as u32);
                }
                cpx12_mais_b(r, 1); // cpx12: o jogo
                if k > 10 {
                    cpx12_mais_b(r + 1, 1);
                }
                cpx12_mais_b(r + 2, cpx12_b(e + 0x21));
                cpx12_mais_b(r + 3, cpx12_b(e + 0x22));
                cpx12_mais_b(r + 4, cpx12_b(e + 0x23));
                cpx12_mais_b(r + 5, cpx12_b(e + 0x24));
                cpx12_mais_b(r + 6, cpx12_b(e + 0x27));
                let (f, a) = (cpx12_b(gf), cpx12_b(gc));
                cpx12_mais_b(r + 7, f);
                cpx12_mais_b(r + 8, a);
                if f > a {
                    cpx12_mais_b(r + 9, 1);
                }
                if f < a {
                    cpx12_mais_b(r + 0xa, 1);
                }
                cpx12_mais_b(r + 0xb, cpx12_b(e + 0x2c));
                cpx12_mais_b(r + 0xc, cpx12_b(e + 0x2d));
                if cpx12_d(e + 4) == cpx12_d(jogo + 0x3c) {
                    cpx12_mais_b(r + 0xd, 1);
                }
                cpx12_mais_w(r + 0xe, nota as u16); // cpx12: a nota
                cpx12_mais_w(r + 0x10, cpx12_b(e + 0x3e) as u16);
                cpx12_mais_w(r + 0x12, cpx12_b(e + 0x3f) as u16);
                cpx12_mais_w(r + 0x14, cpx12_b(e + 0x42) as u16);
                cpx12_mais_w(r + 0x16, cpx12_b(e + 0x28) as u16);
                cpx12_mais_w(r + 0x18, cpx12_b(e + 0x25) as u16);
                cpx12_mais_w(r + 0x1a, cpx12_b(e + 0x26) as u16);
                let saiu = |e: usize| {
                    let (s34, s30) = (cpx12_b(e + 0x34) as i8, cpx12_b(e + 0x30) as i8);
                    if s34 > -1 {
                        s34 as i32
                    } else if s30 > -1 {
                        s30 as i32
                    } else {
                        cpx12_w(jogo + 0x4d) as i32
                    }
                };
                let min = r + 0x1c;
                if k < 11 {
                    core::ptr::write_unaligned(min as *mut i32, (cpx12_d(min) as i32).wrapping_add(saiu(e)));
                } else if cpx12_b(e + 0x32) != 0xff {
                    let v = saiu(e).wrapping_sub(cpx12_b(e + 0x32) as i8 as i32);
                    core::ptr::write_unaligned(min as *mut i32, (cpx12_d(min) as i32).wrapping_add(v));
                }
            }
            let reg = cpx12_d(this + 0x30).wrapping_add((cpx12_d(e + 4)).wrapping_mul(0x27));
            if cat == 2 || (cpx12_d(p) as i32 > -1 && conta(cpx12_d(p) as i32) != 0) {
                cpx12_mais_w(reg + 0x20, 1);
                cpx12_mais_w(reg + 0x22, cpx12_b(e + 0x21) as u16);
                if ficha(cpx12_d(0xae1050), pessoa) == 0 && cpx12_contrato(pessoa) != 0 {
                    cpx12_mais_w(cpx12_contrato(pessoa) + 0x3b, 1);
                }
            }
            let (f24, f25, f26) = (cpx12_b(reg + 0x24), cpx12_b(reg + 0x25), cpx12_b(reg + 0x26));
            *((reg + 0x24) as *mut u8) = (f25 << 4) | (f24 >> 4);
            *((reg + 0x25) as *mut u8) = (f26 << 4) | (f25 >> 4);
            *((reg + 0x26) as *mut u8) = ((nota ^ f26) & 0xf) ^ f26;
        }
    }
    1
}

/// Comparador lado a lado: as somas em double (+0x10..+0x30), os registros das categorias de cada linha que conta, o
/// trecho +0x20..+0x27 do registro do jogador e o +0x3b do contrato; no jogo simulado (competição −2), também o estado
/// do sorteio (e as duas categorias possíveis), com o sorteio da cópia fora da contagem. Partida ou jogo nulos, a caixa
/// de uma competição sem tipo conhecido ou registro de categoria a criar (0x7c5d80 cresce o vetor): só a original.
unsafe extern "thiscall" fn comparar_registrar_partida_nas_estatisticas(this: usize, p: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1338);
    if p == 0 || jogo == 0 {
        return o(this, p, jogo); // ponytail: as caixas, só a original
    }
    let simulado = cpx12_d(p) as i32 == -2;
    let mut regioes = vec![(this + 0x10, 0x20)];
    for lado in 0..2usize {
        let cats: Vec<u8> = if simulado {
            vec![2, 3, 6]
        } else {
            let mut erro = false;
            let c = cpx12_categoria(p, lado as i32, false, &mut erro);
            if erro {
                return o(this, p, jogo); // ponytail: a caixa, só a original
            }
            cpx12_categorias(c).into_iter().flatten().collect()
        };
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = cpx12_linha(jogo, lado, k) else { continue };
            for &c in &cats {
                let r = cpx12_registro(this, dados, c);
                if r == 0 {
                    return o(this, p, jogo); // ponytail: o registro seria criado, só a original
                }
                regioes.push((r, 0x20));
            }
            regioes.push((cpx12_d(this + 0x30).wrapping_add(cpx12_d(e + 4).wrapping_mul(0x27)) + 0x20, 7));
            let ct = cpx12_contrato(pessoa);
            if ct != 0 {
                regioes.push((ct + 0x3b, 2));
            }
        }
    }
    if simulado {
        regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, p, jogo), || {
        crate::aleatorio::sem_contar(|| registrar_partida_nas_estatisticas(this, p, jogo))
    });
    PLACAR.registrar(igual && ro == rr, || format!("registrar_partida_nas_estatisticas({:x}, {:x}, {:x}) original={} {:x?} rust={} {:x?}", this, p, jogo, ro, mo, rr, mr));
    ro
}

/// O ensaio do comparador da 0x447430: a cópia roda sem ordenar as tabelas (a original, que roda antes, já ordenou).
static mut CPX12_ENSAIO: bool = false;

unsafe fn cpx12_temporada(this: usize, clube: usize, criar: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    f(this, clube, criar)
}

unsafe fn cpx12_de_sempre(this: usize, clube: usize, criar: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    f(this, clube, criar)
}

/// A reputação (+0x69, palavra com sinal) da competição `id` (registros de 0x6b em [0xae23d0]).
unsafe fn cpx12_reputacao(id: i32) -> i16 {
    cpx12_w(cpx12_d(0xae23d0).wrapping_add((id as usize).wrapping_mul(0x6b)) + 0x69)
}

/// O recorde de sempre mudou: clube humano (0x600a50) ou seleção (0x53b900) com 5 temporadas ou mais (+0x68 do
/// resumo) e o recorde (id, tipo) ainda fora da fila (+0x24, 6 bytes, conta +0x2c): entra na fila (0x452cb0).
unsafe fn cpx12_fila_do_recorde(this: usize, clube: usize, resumo: usize, tipo: u8) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let guardar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x452cb0usize);
    if humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 {
        return;
    }
    if cpx12_w(resumo + 0x68) < 5 {
        return;
    }
    let n = cpx12_d(this + 0x2c) as i32;
    let mut e = cpx12_d(this + 0x24);
    for _ in 0..n.max(0) {
        if cpx12_d(e) == cpx12_d(clube) && cpx12_b(e + 4) == tipo {
            return;
        }
        e += 6;
    }
    guardar(this, cpx12_d(clube), tipo as u32);
}

/// As ligas da cadeia da 0x447430: a liga e, enquanto houver, a fase seguinte (0x48d200 da primeira, índice +0x44 + 1
/// até +0x30) do tipo 2 (+0x43) com as mesmas +0x3e linhas.
unsafe fn cpx12_cadeia(liga0: usize) -> Vec<usize> {
    let fase: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x48d200usize);
    let mut v = vec![liga0];
    let mut liga = liga0;
    loop {
        let k = cpx12_b(liga + 0x44) as i8 as i32;
        if k >= cpx12_d(liga0 + 0x30) as i32 {
            return v;
        }
        let f = fase(liga0, k + 1);
        if cpx12_b(f + 0x43) != 2 || cpx12_w(f + 0x3e) != cpx12_w(liga + 0x3e) {
            return v;
        }
        v.push(f);
        liga = f;
    }
}

/// { 0x447430 } (thiscall, ret 8, club_records.cpp): `Club_Records::update_league_position_records(liga, sempre)`:
/// liga nula ou com +0x44 ≠ 0xff, as caixas e 0. Em cada liga da cadeia (cpx12_cadeia), ordena a tabela (0x6827d0) e,
/// em cada linha com jogos (+0x11) e clube, pelos registros da temporada (0x4528d0) e de sempre (0x452710, criando):
/// melhor posição (tipo 1, 0x449590) e pior (tipo 2): sem registro (−1), competição (o id da primeira liga) de
/// reputação maior (melhor) ou menor (pior), ou a mesma com posição melhor ou pior: grava a posição (+0/+7), a
/// competição (+1/+8) e o ano (+5/+0xc; 0 na temporada, 0x669950 em sempre). Em sempre (com `sempre`; id acima de
/// [0xae2378] também é "sem"), a fila da notícia (cpx12_fila_do_recorde). Devolve 1.
unsafe extern "thiscall" fn recordes_de_posicao_na_liga(this: usize, liga0: usize, sempre: u32) -> u32 {
    if liga0 == 0 || cpx12_b(liga0 + 0x44) != 0xff {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1339)(this, liga0, sempre); // as caixas
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let aplica: unsafe extern "stdcall" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x449590usize);
    let ano_da_temporada: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x669950usize);
    let comp = cpx12_d(cpx12_d(liga0 + 4)) as i32;
    for liga in cpx12_cadeia(liga0) {
        if !CPX12_ENSAIO {
            ordenar(liga, 0);
        }
        let mut i = 0i32;
        while i < cpx12_w(liga + 0x3e) as i32 {
            let linha = cpx12_d(liga + 0xb1).wrapping_add(0x3b * i as usize);
            let clube = cpx12_d(linha);
            i += 1;
            if cpx12_b(linha + 0x11) == 0 || clube == 0 {
                continue;
            }
            let t = cpx12_temporada(this, clube, 1);
            let s = cpx12_de_sempre(this, clube, 1);
            let resumo = cpx12_d(this + 0x1c).wrapping_add(cpx12_d(clube).wrapping_mul(0x6a));
            let ano = ano_da_temporada(clube) as u16;
            let pos = cpx12_b(linha + 4);
            let fora = |v: i32| v < 0 || v > cpx12_d(0xae2378) as i32;
            if aplica(1, clube, 1) != 0 {
                let v = cpx12_d(t + 1) as i32;
                if v < 0 || cpx12_reputacao(comp) > cpx12_reputacao(v) || (comp == v && pos < cpx12_b(t)) {
                    *(t as *mut u8) = pos;
                    core::ptr::write_unaligned((t + 1) as *mut i32, comp);
                    core::ptr::write_unaligned((t + 5) as *mut u16, 0);
                }
                let v = cpx12_d(s + 1) as i32;
                if sempre != 0 && (fora(v) || cpx12_reputacao(comp) > cpx12_reputacao(v) || (comp == v && pos < cpx12_b(s))) {
                    crate::comparacao::ramo("cpx_12 melhor posicao de sempre");
                    *(s as *mut u8) = pos;
                    core::ptr::write_unaligned((s + 1) as *mut i32, comp);
                    core::ptr::write_unaligned((s + 5) as *mut u16, ano);
                    cpx12_fila_do_recorde(this, clube, resumo, 1);
                }
            }
            if aplica(2, clube, 1) != 0 {
                let v = cpx12_d(t + 8) as i32;
                if v < 0 || cpx12_reputacao(comp) < cpx12_reputacao(v) || (comp == v && pos > cpx12_b(t + 7)) {
                    *((t + 7) as *mut u8) = pos;
                    core::ptr::write_unaligned((t + 8) as *mut i32, comp);
                    core::ptr::write_unaligned((t + 0xc) as *mut u16, 0);
                }
                let v = cpx12_d(s + 8) as i32;
                if sempre != 0 && (fora(v) || cpx12_reputacao(comp) < cpx12_reputacao(v) || (comp == v && pos > cpx12_b(s + 7))) {
                    *((s + 7) as *mut u8) = pos;
                    core::ptr::write_unaligned((s + 8) as *mut i32, comp);
                    core::ptr::write_unaligned((s + 0xc) as *mut u16, ano);
                    cpx12_fila_do_recorde(this, clube, resumo, 2);
                }
            }
        }
    }
    1
}

/// Comparador: a original roda antes (ordena as tabelas, que a 0x6827d0 confere por conta própria) e a cópia depois,
/// em ensaio (sem ordenar), a partir dos mesmos recordes: os 14 primeiros bytes dos registros da temporada e de sempre
/// de cada clube da cadeia, o cabeçalho da fila (+0x24..+0x30) e a fila; e a devolução. Liga nula ou com +0x44 ≠ 0xff,
/// registro a criar (0x4528d0/0x452710 alocam) ou fila que pode crescer: só a original.
unsafe extern "thiscall" fn comparar_recordes_de_posicao_na_liga(this: usize, liga0: usize, sempre: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1339);
    if liga0 == 0 || cpx12_b(liga0 + 0x44) != 0xff {
        return o(this, liga0, sempre); // ponytail: as caixas, só a original
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let mut regioes = vec![(this + 0x24, 0xc)];
    let mut podem_entrar = 0i32;
    for liga in cpx12_cadeia(liga0) {
        for i in 0..cpx12_w(liga + 0x3e).max(0) as usize {
            let linha = cpx12_d(liga + 0xb1) + 0x3b * i;
            let clube = cpx12_d(linha);
            if cpx12_b(linha + 0x11) == 0 || clube == 0 {
                continue;
            }
            let (t, s) = (cpx12_temporada(this, clube, 0), cpx12_de_sempre(this, clube, 0));
            if t == 0 || s == 0 {
                return o(this, liga0, sempre); // ponytail: o registro seria criado, só a original
            }
            regioes.push((t, 0xe));
            regioes.push((s, 0xe));
            if humano(clube, 0, 1, 0, 0) != 0 || selecao(clube) != 0 {
                podem_entrar += 2;
            }
        }
    }
    let fila = cpx12_d(this + 0x24);
    if (cpx12_d(this + 0x2c) as i32).wrapping_add(podem_entrar) > cpx12_d(this + 0x28) as i32 || (fila == 0 && podem_entrar > 0) {
        return o(this, liga0, sempre); // ponytail: a fila pode crescer (realoca), só a original
    }
    if fila != 0 {
        regioes.push((fila, 6 * cpx12_d(this + 0x28)));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let ro = o(this, liga0, sempre);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    CPX12_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(|| recordes_de_posicao_na_liga(this, liga0, sempre));
    CPX12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    PLACAR.registrar(ro == rr && mo == mr, || format!("recordes_de_posicao_na_liga({:x}, {:x}, {}) original={} {:x?} rust={} {:x?}", this, liga0, sempre, ro, mo, rr, mr));
    ro
}

/// O ensaio do comparador da 0x448e30: a cópia roda sem preencher nem entregar as notícias e sem soltar a fila.
static mut CPX12_ENSAIO_FILA: bool = false;

/// { 0x448e30 } (thiscall, ret 0, club_records.cpp, quadro SEH sem destrutor): `Club_Records::send_record_news()`: sem
/// fila (+0x24), 1. Cada recorde da fila (6 bytes: id do clube, tipo, período em +5) de clube humano (0x7875e0) ou
/// seleção (0x53b900): a notícia 0x3e8 (local) preenchida pela 0x453120 com o registro da temporada (período 2,
/// 0x4528d0) ou de sempre (0x452710), sem criar; entregue à nação (0x7863f0) da seleção ou ao técnico do clube
/// (0x7861e0). Fila com capacidade (+0x28) de 100 ou mais: solta (0x9452ca) e zera +0x24/+0x28. A conta (+0x2c) volta
/// a 0. Devolve 1.
unsafe extern "thiscall" fn noticias_de_recordes(this: usize) -> u32 {
    if cpx12_d(this + 0x24) == 0 {
        return 1;
    }
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let preencher: unsafe extern "C" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x453120usize);
    let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut i = 0i32;
    while i < cpx12_d(this + 0x2c) as i32 {
        let e = cpx12_d(this + 0x24) + 6 * i as usize;
        i += 1;
        let id = cpx12_d(e);
        let tipo = cpx12_b(e + 4);
        let clube = cpx12_d(0xae23b4).wrapping_add(id).wrapping_add(id.wrapping_mul(0x244));
        if humano(clube) == 0 && selecao(clube) == 0 {
            continue;
        }
        crate::comparacao::ramo("cpx_12 noticia de recorde");
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construir(n, 0x3e8, 0);
        let reg = if cpx12_b(e + 5) == 2 { cpx12_temporada(this, clube, 0) } else { cpx12_de_sempre(this, clube, 0) };
        if CPX12_ENSAIO_FILA {
            continue;
        }
        // ponytail: a original empurra o dword da local com o tipo no byte baixo (o resto é sobra); a 0x453120 lê o byte
        preencher(id, reg, tipo as u32, n);
        if selecao(clube) != 0 {
            a_nacao(cpx12_d(0xb7431c), n, cpx12_d(clube + 0x53));
        } else {
            ao_clube(cpx12_d(0xb7431c), n, clube);
        }
    }
    if cpx12_d(this + 0x28) as i32 >= 100 {
        if !CPX12_ENSAIO_FILA {
            liberar(cpx12_d(this + 0x24));
        }
        core::ptr::write_unaligned((this + 0x24) as *mut usize, 0);
        core::ptr::write_unaligned((this + 0x28) as *mut usize, 0);
    }
    core::ptr::write_unaligned((this + 0x2c) as *mut usize, 0);
    1
}

/// Comparador: a cópia em ensaio (sem preencher nem entregar, sem soltar) roda antes, enquanto a fila está viva, e a
/// original depois a partir do mesmo cabeçalho (+0x24..+0x30): o cabeçalho e a devolução.
unsafe extern "thiscall" fn comparar_noticias_de_recordes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1340);
    let regioes = [(this + 0x24, 0xc)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX12_ENSAIO_FILA = true;
    let rr = crate::aleatorio::pausar_rastro(|| noticias_de_recordes(this));
    CPX12_ENSAIO_FILA = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let ro = o(this);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(ro == rr && mo == mr, || format!("noticias_de_recordes({:x}) original={} {:x?} rust={} {:x?}", this, ro, mo, rr, mr));
    ro
}
