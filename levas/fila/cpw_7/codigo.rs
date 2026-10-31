// ---------- %%LEVA%%a leva: as faltas do dia: o dia das ausências, das faltas ao treino e dos atrasos, a falta ao treino marcada e o atraso registrado (awol.cpp) ----------

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
    orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let l = cpw7_d(this + 8);
    let regioes: Vec<(usize, usize)> = if l != 0 { vec![(l + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa), || teve_incidente_de_forma(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("teve_incidente_de_forma(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}
