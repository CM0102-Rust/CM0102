// ---------- %%LEVA%%a leva: as faltas do dia: o dia das ausências, das faltas ao treino e dos atrasos, a falta ao treino marcada, o atraso registrado, marcar a falta, o fim da ausência e as marcas do contrato (awol.cpp, staff_contracts.cpp) ----------

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
unsafe fn maybe_drop_attribute(a: usize, zero: bool) {
    let v = cpw7_b(a) as i8;
    if v > 1 && (cpw7_sortear(v as i32) == 0) == zero {
        *(a as *mut i8) = v - 1;
    }
}

unsafe fn awol_llist_next(lista: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(lista)
}

/// O cursor da lista volta ao primeiro nó (+0xa = +2).
unsafe fn awol_llist_rewind(lista: usize) {
    cpw7_pd(lista + 0xa, cpw7_d(lista + 2) as u32);
}

/// A data `d` (dia com sinal em +0, ano sem sinal em +2) já passou: hoje ([0xae2c90]) é depois dela.
unsafe fn date_has_passed(d: usize) -> bool {
    let (dia, ano) = (cpw7_w(0xae2c90), cpw7_w(0xae2c92) as u16);
    let a = cpw7_w(d + 2) as u16;
    ano > a || (ano == a && dia > cpw7_w(d))
}

/// Uma semana depois da data `d` (0x54c770), já passou?
unsafe fn weeks_after_have_passed(d: usize, semanas: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let mut t = [0u8; 8];
    date_has_passed(f(d, &mut t, semanas))
}

/// O fim do caso de falta ao treino `r` (pessoa em +0): com o byte da pessoa na tabela de marcas ([this+0xc]) não zero,
/// troca o bit 2 dele (o `or 2` da original nunca dá zero) e tira o registro da lista das faltas ([this+4], 0x68d1f0).
unsafe fn close_awol_case(this: usize, r: usize) {
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m ^= 2;
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    crate::comparacao::ramo("cpw_7 falta tirada da lista");
    tirar(cpw7_d(this + 4), r); // cpw7: tira o caso da lista
}

unsafe fn awol_miss_training(this: usize, pessoa: usize) {
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
unsafe extern "thiscall" fn awol_daily_update(this: usize) {
    // (1) as ausências
    let l = cpw7_d(this);
    if l != 0 {
        awol_llist_rewind(l);
        if cpw7_w(l) > 0 {
            let mut i = 0;
            loop {
                let r = awol_llist_next(cpw7_d(this));
                if r == 0 {
                    return;
                }
                if date_has_passed(r + 4) {
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
        awol_llist_rewind(l);
        if cpw7_w(l) > 0 {
            let mut i = 0;
            loop {
                let r = awol_llist_next(cpw7_d(this + 4));
                if r == 0 {
                    return;
                }
                if weeks_after_have_passed(r + 4, 1) { // cpw7: uma semana depois
                    crate::comparacao::ramo("cpw_7 falta vencida");
                    resolve_awol_case(this, r);
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
    awol_llist_rewind(l);
    if cpw7_w(l) <= 0 {
        return;
    }
    let mut i = 0;
    loop {
        let r = awol_llist_next(cpw7_d(this + 8));
        if r == 0 {
            return;
        }
        if weeks_after_have_passed(r + 8, 1) {
            let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
            let resolveu = if cpw7_b(r + 0x10) != 0 {
                if cpw7_d(pessoa + 0x61) == 0 {
                    true
                } else if cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(cpw7_b(pessoa + 0x5d) as i8 as i32) == 0 {
                    maybe_drop_attribute(pessoa + 0x5b, true);
                    maybe_drop_attribute(pessoa + 0x5d, true);
                    true
                } else {
                    false
                }
            } else {
                let a = cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32);
                let b = cpw7_sortear(5);
                if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
                    maybe_drop_attribute(pessoa + 0x5b, true);
                    let j = cpw7_d(pessoa + 0x61);
                    if j != 0 {
                        maybe_drop_attribute(j + 0x44, true);
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

/// A falta ao treino `r` com uma semana passada (o meio da lista (2) de `awol_daily_update`).
unsafe fn resolve_awol_case(this: usize, r: usize) {
    let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
    if cpw7_b(r + 0xc) != 0 {
        // o caso repetido
        if cpw7_d(pessoa + 0x61) != 0 {
            crate::comparacao::ramo("cpw_7 caso repetido");
            if cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(cpw7_b(pessoa + 0x5d) as i8 as i32) == 0 { // cpw7: os sorteios do caso repetido
                maybe_drop_attribute(pessoa + 0x5b, true);
                maybe_drop_attribute(pessoa + 0x5d, true);
                close_awol_case(this, r);
                awol_miss_training(this, pessoa);
                return;
            }
        }
        return close_awol_case(this, r);
    }
    // o caso novo: os colegas do clube
    crate::comparacao::ramo("cpw_7 caso novo");
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contract_of(cpw7_d(0xae1050), pessoa);
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
            maybe_drop_attribute(s + 0x5b, true);
            let j = cpw7_d(s + 0x61);
            if cpw7_b(j + 0x44) as i8 > 1 && cpw7_sortear(cpw7_b(s + 0x5b) as i8 as i32) == 0 {
                *((j + 0x44) as *mut i8) -= 1;
            }
            maybe_drop_attribute(s + 0x5d, true);
        }
    }
    // o faltoso
    let j = cpw7_d(pessoa + 0x61);
    if j != 0 {
        let a = cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32);
        let b = cpw7_sortear(0xf);
        if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
            maybe_drop_attribute(pessoa + 0x5b, false);
            maybe_drop_attribute(cpw7_d(pessoa + 0x61) + 0x44, false);
            close_awol_case(this, r);
            awol_miss_training(this, pessoa);
            if clube != 0 && cpw7_d(pessoa + 0x61) != 0 {
                let brigar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x72c1f0usize);
                brigar(cpw7_d(0xb6b7c8), pessoa, clube, 8, 0, 0);
            }
            return;
        }
    }
    close_awol_case(this, r);
}

// ponytail: sorteia, tira e libera nós das listas (0x68d1f0), dá notícia de volta (0x419710) e de falta (0x41a360): só
// a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_awol_daily_update(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x418ff0 } (thiscall, ret 4, awol.cpp): `AWOL::has_missed_training(pessoa)`, pelos jogadores do dia (0x5a69b0) e
/// pelas telas. Pessoa nula ou fora da conta [0xae2364]: falso. Com o bit 2 da marca dela ([this+0xc] + id) e sem
/// contrato (0x4cf5a0, guardado em [0xae1080]), tira a falta (0x4197c0) antes. Devolve o bit 2 da marca.
unsafe extern "thiscall" fn has_missed_training(this: usize, pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let id = cpw7_i(pessoa);
    if id >= cpw7_i(0xae2364) {
        return id as u32 & 0xffffff00;
    }
    if cpw7_b(cpw7_d(this + 0xc).wrapping_add(id as usize)) & 2 != 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let k = contract_of(cpw7_d(0xae1050), pessoa);
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
unsafe extern "thiscall" fn comparar_has_missed_training(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    if pessoa != 0 && cpw7_i(pessoa) < cpw7_i(0xae2364) && cpw7_b(cpw7_d(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize)) & 2 != 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let k = contract_of(cpw7_d(0xae1050), pessoa);
        if k == 0 || cpw7_d(0xae23b4).wrapping_add(cpw7_d(k + 4).wrapping_mul(0x245)) == 0 {
            // ponytail: tira a falta da lista (libera o nó): só a original
            return o(this, pessoa);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(this, pessoa), || has_missed_training(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("has_missed_training(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// { 0x419070 } (thiscall, ret 4, awol.cpp): `AWOL::has_fitness_incident(pessoa)`, pelos jogadores do dia (0x5a69b0).
/// Pessoa nula, fora da conta [0xae2364] ou sem a lista dos atrasos ([this+8]): falso; senão percorre a lista (cursor em
/// +0xa) atrás do registro da pessoa (id em +0). Verdadeiro no byte baixo; os altos como a original deixa no registrador.
unsafe extern "thiscall" fn has_fitness_incident(this: usize, pessoa: usize) -> u32 {
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
    awol_llist_rewind(l);
    if cpw7_w(l) <= 0 {
        return l as u32 & 0xffffff00;
    }
    let mut i = 0;
    loop {
        let r = awol_llist_next(cpw7_d(this + 8));
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
unsafe extern "thiscall" fn comparar_has_fitness_incident(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let l = cpw7_d(this + 8);
    let regioes: Vec<(usize, usize)> = if l != 0 { vec![(l + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa), || has_fitness_incident(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("has_fitness_incident(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

// ---- as originais que a awol_daily_update chama e que liam resíduo da pilha particular (bancada da 259ª) ----

/// O ensaio dos comparadores de `miss_training`, `end_absence` e `set_flags`: a cópia para antes
/// das chamadas que alocam, dão notícia ou não podem rodar duas vezes e deixa em CPW7_DECISAO o que faria.
static mut CPW7_ENSAIO: bool = false;
static mut CPW7_DECISAO: u32 = 0;

/// O clube do contrato `k` (+4 id, [0xae23b4], 0x245 bytes), ou 0.
unsafe fn awol_club_of_contract(k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    cpw7_d(0xae23b4).wrapping_add(cpw7_d(k + 4).wrapping_mul(0x245))
}

/// O byte +0xc (caso repetido) do registro da falta. A 0x41a360 não o grava: o registro de 13 bytes é montado num local
/// e o +0xc fica com o que a chamada anterior deixou no dword E−0x10 da pilha (E: o esp na entrada). No caminho normal
/// da original, pela game_check_players_missing_training (0x5d6aa0), a chamada anterior que alcança essa altura é a 0x54cdb0 (dias desde a
/// chegada), que guarda ali o seu `this`, a data do jogo 0xae2c90: o byte baixo é 0x90. A cópia usa esse valor (um caso
/// repetido); o comparador não o compara (o da original é resíduo de verdade).
const AWOL_RESIDUE_BYTE: u8 = 0x90;

/// { 0x41a360 } (thiscall, ret 4, awol.cpp, SEH só com a data local, sem destrutor): `AWOL::miss_training(pessoa)`,
/// pelas faltas ao treino do dia (0x5d6aa0) e pela awol_daily_update. Jogador (+0x61) dentro da conta, com contrato
/// (0x4cf5a0, em [0xae1080]) num clube e sem a marca 2: contrato com moral (+0x44) até −100 e o bit 0x2000 (depois o
/// 0x800) em +0x45, com sorteio(profissionalismo + 10) e sorteio(+0x59 + 10) zero, some (0x419840 com 0, 0). Senão
/// marca o bit 2, notícia da falta (0x41a630, com o clube do contrato), o byte +0xa da linha de 79 bytes ([0xae2d0c])
/// soma sorteio(10), a condição (0x62c590) cai 25 × (21 − o +0x34 do jogador) + sorteio + 100 (0x62bd10), o registro
/// (pessoa, hoje, caso) entra na lista das faltas ([this+4], 0x68d0d0); clube da IA (0x600a50) com técnico (+0xcf, +0x69)
/// e sorteio(+0x14 dele) não zero: a multa (0x5a5980, prazo 1 se sorteio(nota do técnico 0x544410 com o +0x19) passar
/// de sorteio(5), senão 2; motivo 1).
unsafe extern "thiscall" fn miss_training(this: usize, pessoa: usize) {
    if pessoa == 0 || cpw7_d(pessoa + 0x61) == 0 || cpw7_i(pessoa) >= cpw7_i(0xae2364) {
        return;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contract_of(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k as u32);
    let clube = awol_club_of_contract(k);
    if k == 0 || clube == 0 {
        return;
    }
    let id = cpw7_i(pessoa);
    let m = cpw7_d(this + 0xc).wrapping_add(id as usize) as *mut u8;
    if *m & 2 != 0 {
        return;
    }
    let k2 = contract_of(cpw7_d(0xae1050), pessoa);
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
    let k3 = contract_of(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k3 as u32);
    let noticia: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(0x41a630usize);
    noticia(pessoa, awol_club_of_contract(k3));
    let linha = cpw7_d(0xae2d0c).wrapping_add((cpw7_i(pessoa) as usize).wrapping_mul(79)).wrapping_add(0xa) as *mut u8;
    let r = cpw7_sortear(10);
    *linha = (*linha).wrapping_add(r as u8);
    let match_condition: unsafe extern "thiscall" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x62c590usize);
    let c = match_condition(cpw7_d(0xb63d28), pessoa, 0);
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
    reg[12] = AWOL_RESIDUE_BYTE;
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
unsafe extern "thiscall" fn comparar_miss_training(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I3%%);
    if pessoa == 0 || cpw7_d(pessoa + 0x61) == 0 || cpw7_i(pessoa) >= cpw7_i(0xae2364) {
        return o(this, pessoa);
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize);
    let mut regioes = vec![(m, 1), (0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| miss_training(this, pessoa)));
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
    PLACAR.registrar(igual, || format!("miss_training(pessoa {:#x}) original={} {:x?} rust={} {:x?}", pessoa, r, mo, n, mr));
}

/// { 0x419710 } (thiscall, ret 4, awol.cpp): `AWOL::end_absence(registro)`, pela awol_daily_update. Registro com a marca
/// da pessoa (+0 id; [this+0xc] + id) não zero: com contrato (0x4cf5a0, em [0xae1080]) num clube, a notícia da volta
/// (0x419ad0 com o clube +0x39 da pessoa, 0 e −1); troca o bit 1 da marca (o `or 1` da original nunca dá zero) e tira o
/// registro da lista das ausências ([this], 0x68d1f0). Não lê local sem valor.
unsafe extern "thiscall" fn end_absence(this: usize, r: usize) {
    if r == 0 {
        return;
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_7 fim da ausencia");
    let pessoa = cpw7_d(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contract_of(cpw7_d(0xae1050), pessoa);
    cpw7_pd(0xae1080, k as u32);
    if k != 0 && awol_club_of_contract(k) != 0 {
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
unsafe extern "thiscall" fn comparar_end_absence(this: usize, r: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I4%%);
    if r == 0 {
        return o(this, r);
    }
    let m = cpw7_d(this + 0xc).wrapping_add(cpw7_i(r) as usize);
    let regioes = [(m, 1), (0xae1080, 4)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| end_absence(this, r));
    CPW7_ENSAIO = false;
    let noticia = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, r);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = mo.0[0] == mr.0[0] && (noticia || mo.0[1] == mr.0[1]);
    PLACAR.registrar(igual, || format!("end_absence(marca {:#x}) original={:x?} rust={:x?}", m, mo, mr));
}

/// { 0x867560 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_flags(marcas)`, chamada por muitas rotinas
/// (no motor.rs pelo endereço; antes só a porta do pós-jogo a desviava, no modo comparação). Marcas acima de 0x1000: a
/// caixa "Error" e segue (aqui: a original inteira). Liga as marcas em +0x40; pessoa (+0 id) que não é das funções
/// 0xc, 0xd e 0xf, contrato com +0x44 positivo, sem +0x49, que quer sair (0x866f60), sem transferência futura
/// (0x8cca10 com 0), sem os bits 1 e 8 de +0x4e, fora da fila da transferência livre (0x8c1870) e com mais de 20 anos
/// (+0x18) ou reputação atual (+0xb do jogador) acima de 4250: o pedido de transferência (0x4d8610 em [0xae1050]).
/// Não lê local sem valor.
unsafe extern "thiscall" fn set_flags(c: usize, marcas: u32) {
    if marcas as i32 > 0x1000 {
        // ponytail: a caixa "Error" (antes de qualquer gravação): a original inteira
        return original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I5%%)(c, marcas);
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
unsafe extern "thiscall" fn comparar_set_flags(c: usize, marcas: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I5%%);
    if marcas as i32 > 0x1000 {
        return o(c, marcas);
    }
    let mut regioes = vec![(c + 0x40, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| set_flags(c, marcas)));
    CPW7_ENSAIO = false;
    let pediu = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(c, marcas);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if pediu { mo.0[0] == mr.0[0] } else { mo == mr };
    PLACAR.registrar(igual, || format!("set_flags({:#x}, {:#x}) pediu={} original={:x?} rust={:x?}", c, marcas, pediu, mo, mr));
}
