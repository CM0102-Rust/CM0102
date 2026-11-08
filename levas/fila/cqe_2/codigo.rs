// ---------- %%LEVA%%a leva: a opção de troca na resposta ao lance e o fim do empréstimo agendado (transfer_manager.cpp, contract_manager.cpp) ----------

unsafe fn cqe2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqe2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqe2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqe2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqe2_pessoa(id: usize) -> usize {
    cqe2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqe2_clube(id: usize) -> usize {
    cqe2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

unsafe fn cqe2_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080], e o
/// clube dele (sem contrato: 0; o tipo 5 conta).
unsafe fn contract_club_any_type(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqe2_d(0xae1050), pessoa);
    cqe2_pd(0xae1080, c);
    if c == 0 { 0 } else { cqe2_clube(cqe2_d(c + 4)) }
}

/// As marcas da nota pelo alcance da reputação (0x5408c0, cdecl(pessoa, técnico do vendedor +0xcf)): 0..4 dão 0x40,
/// 0x50, 0x10, 0x30, 0x20 sobre `m` (as duas tabelas de salto da original, 0x8c8fa4 e 0x8c8fb8); fora disso a original
/// deixa o ebp como estava (`antes`).
unsafe fn reputation_scope_flags(pessoa: usize, vendedor: usize, m: u32, antes: u32) -> u32 {
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x5408c0usize);
    match escopo(pessoa, cqe2_d(vendedor + 0xcf)) as i8 {
        0 => m | 0x40,
        1 => m | 0x50,
        2 => m | 0x10,
        3 => m | 0x30,
        4 => m | 0x20,
        _ => antes,
    }
}

/// A nota da pessoa para o vendedor (0x6e2f20, cdecl de 15 dwords: pessoa, +0x61, 0, a posição, as marcas, o vendedor,
/// 0, 0, −1 e seis zeros).
unsafe fn score_player_for_seller(pessoa: usize, posicao: u16, marcas: u32, vendedor: usize) -> i32 {
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32 =
        core::mem::transmute(0x6e2f20usize);
    nota(pessoa, cqe2_d(pessoa + 0x61), 0, posicao as u32, marcas, vendedor, 0, 0, -1, 0, 0, 0, 0, 0, 0)
}

/// { 0x8c8860 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da LLIST local): `TRANSFER_MANAGER::
/// make_exchange_option(comprador, alvo, oferta, saida, atracao, valor)`, a opção de troca da resposta da IA ao lance
/// (0x8c8090). O vendedor é o clube do contrato do alvo (0x4cf5a0, em [0xae1080]); a LLIST local (0x68d020/0x68d040). O
/// contrato do alvo com o vendedor (0x4cf5f0). Sem a lista de observação do vendedor (o byte +4 do registro de 9 bytes em
/// [[this+0x856]+0xdc]): 0. Senão o elenco do comprador (0x541f90 com 0: 50 vagas), a oferta nova (0x944cf1 de 0x41
/// bytes e 0x8e5680) em *saida, preparada (0x8e78f0: push valor; push comprador; push alvo; mov ecx, oferta), atração 50,
/// +0x1b = valor e o teto = valor × 1,1 (_ftol). Para o alvo (vaga −1, fora do contrato de estatuto 7) e as 25 vagas da
/// lista do vendedor (o +0xf, fora dos jogadores do próprio vendedor), enquanto a troca não tem 3: a posição (0x5b7d20),
/// as marcas (0x4800, ou 0x4900 fora do estatuto 2 e 3; fora do 5, | 0x80 e o alcance) e a nota base. Depois o próprio e
/// os 50 do elenco com a mesma posição: contrato com o clube atual do candidato (0x4cf5f0) de tipo diferente de 5, no
/// comprador, sem ficha (0x4cf6d0), sem os bits 0x30 de +0xb do registro de 79 bytes, piso (0x4d1600 com 0, 1, 1) até o
/// teto; sem oferta (0x8c5cf0), troca (0x8c5e40 na lista local) nem negociação de troca do alvo (0x8c5fc0), ou só a troca
/// desta oferta (+0x20); o vendedor chega a ele (0x8ce990 com 0, 1); interesse (0x846920: push 0; push estatuto; push 0;
/// push 1; push vendedor; push jogador; mov ecx, [this+0x856]) de pelo menos 3,15; a nota dele / a base acima de 0,9: a
/// atração sobe 20 (nota maior) ou 5, entre 0 e 100; fora da troca (+4, criada se faltar), ele entra (0x68d0d0 com 7
/// bytes: id, 0, estatuto), o valor dele (0x840d80 com −1, 1) sai do teto e de +0x1b (no mínimo 0), e a vaga acaba.
/// Devolve 1.
unsafe extern "thiscall" fn make_exchange_option(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x8e78f0usize);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let piso_de: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x8c5fc0usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(0x846920usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let valor_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(0x840d80usize);
    let vendedor = contract_club_any_type(alvo);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let pl = lista.as_mut_ptr() as usize;
    lista_nova(pl);
    let cv = contrato_com(cqe2_d(0xae1050), alvo, vendedor);
    let reg = cqe2_d(cqe2_d(this + 0x856) + 0xdc).wrapping_add(cqe2_d(vendedor).wrapping_mul(9));
    if cqe2_b(reg + 4) == 0 { // cqe2: o vendedor sem lista de observação ativa
        destruir_lista(pl);
        return 0;
    }
    let mut elenco = [0usize; 50];
    reunir(comprador, elenco.as_mut_ptr(), 0);
    let m = alocar(0x41);
    let nova = if m != 0 { nova_oferta(m) } else { 0 };
    *saida = nova;
    preparar(nova, alvo, comprador, valor);
    crate::comparison::branch("cqe_2 opção montada");
    *atracao = 0x32; // cqe2: a atração da opção
    cqe2_pd(nova + 0x1b, valor as u32 as usize); // cqe2: o preço da opção
    let mut teto = match_pl_ftol(valor as f64 * read_f64_at(0x9689c0));
    // ponytail: a posição, a nota base, as marcas e o estatuto ficam de uma vaga para a outra (a original os deixa na
    // pilha sem valor até a primeira vaga com ficha +0x61; candidato sem ficha não acontece: começam em zero). As marcas
    // fora de 0..4 do alcance (que a 0x5408c0 não devolve) seriam o ebp anterior, de início o ponteiro da oferta.
    let mut posicao: u16 = 0;
    let mut base: i32 = 0;
    let mut marcas: u32 = 0;
    let mut estatuto: u8 = 0;
    let mut ebp: u32 = m as u32;
    let estatuto_da_vaga = |i: i8| (cqe2_b(cqe2_d(reg).wrapping_add((i as i32 * 0x25) as usize) + 0x24) >> 1) & 0xf;
    let mut i: i8 = -1;
    loop {
        let t = cqe2_d(nova + 4);
        if t != 0 && cqe2_w(t) == 3 {
            break;
        }
        'vaga: {
            let cand = if i == -1 {
                if cqe2_b(cv + 0x4f) & 0xf0 == 0x70 {
                    break 'vaga;
                }
                alvo
            } else {
                let id = cqe2_d(cqe2_d(reg).wrapping_add((i as i32 * 0x25) as usize) + 0xf) as i32;
                if id <= -1 { // cqe2: a vaga sem alvo
                    break 'vaga;
                }
                let p = cqe2_pessoa(id as usize);
                if cqe2_d(p + 0x39) == vendedor {
                    break 'vaga;
                }
                crate::comparison::branch("cqe_2 vaga com alvo");
                p
            };
            let cc = contrato_com(cqe2_d(0xae1050), cand, contract_club_any_type(cand));
            let mut j: i8 = -1;
            loop {
                'jogador: {
                    let x = if j == -1 {
                        if cqe2_d(cand + 0x61) == 0 {
                            break 'jogador;
                        }
                        posicao = mascara(cqe2_d(cand + 0x61));
                        marcas = 0x4800;
                        estatuto = if i == -1 { cqe2_b(cc + 0x4f) >> 4 } else { estatuto_da_vaga(i) };
                        if estatuto != 2 && estatuto != 3 {
                            marcas = 0x4900;
                        }
                        if estatuto == 5 {
                            ebp = marcas;
                        } else {
                            marcas |= 0x80;
                            ebp = reputation_scope_flags(cand, vendedor, marcas, ebp);
                        }
                        base = score_player_for_seller(cand, posicao, ebp, vendedor);
                        cand
                    } else {
                        let x = elenco[j as usize];
                        if x == 0 || cqe2_d(x) as i32 >= cqe2_d(0xae2364) as i32 || cqe2_d(x + 0x61) == 0 {
                            break 'jogador;
                        }
                        if mascara(cqe2_d(x + 0x61)) != posicao {
                            break 'jogador;
                        }
                        x
                    };
                    let cx = contrato_com(cqe2_d(0xae1050), x, contract_club_any_type(cand));
                    if cx == 0 || cqe2_d(x + 0x39) != comprador || ficha(cqe2_d(0xae1050), x) != 0 {
                        break 'jogador;
                    }
                    if cqe2_b(cqe2_d(0xae2d0c).wrapping_add(cqe2_d(x).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 || cqe2_b(cx + 0x35) & 0x3f == 5 {
                        break 'jogador;
                    }
                    if piso_de(cqe2_d(0xae1050), comprador, 0, x, 0, 1, 1) > teto {
                        break 'jogador;
                    }
                    esvaziar(pl);
                    if ofertas(this, x, 0, 0) != 0 || trocas(this, x, pl) != 0 || negociacoes(this, alvo, 0) != 0 {
                        if cqe2_w(pl) != 1 {
                            break 'jogador;
                        }
                        cqe2_pd(pl + 0xa, cqe2_d(pl + 2));
                        let p = cqe2_proximo(pl);
                        if cqe2_d(oferta + 0x20) != cqe2_d(p) {
                            break 'jogador;
                        }
                    }
                    if chegar(this, x, vendedor, 0, 1) == 0 {
                        break 'jogador;
                    }
                    // ponytail: no alvo a original empurra o estatuto com os bytes altos do ecx que a 0x8ce990 deixou (lixo);
                    // nas vagas, com os de 0x25 × vaga; a 0x846920 recebe um char: só o byte conta
                    let st = if i == -1 {
                        (cqe2_b(cc + 0x4f) >> 4) as u32
                    } else {
                        ((i as i32 * 0x25) as u32 & 0xffff_ff00) | estatuto_da_vaga(i) as u32
                    };
                    let r = interesse(cqe2_d(this + 0x856), x, vendedor, 1, 0, st, 0);
                    if !(r >= read_f64_at(0x968e58)) { // cqe2: o interesse abaixo de 3,15
                        break 'jogador;
                    }
                    crate::comparison::branch("cqe_2 interesse");
                    if estatuto == 5 {
                        ebp = marcas;
                    } else {
                        marcas |= 0x80;
                        ebp = reputation_scope_flags(x, vendedor, marcas, ebp);
                    }
                    let nx = score_player_for_seller(x, posicao, ebp, vendedor);
                    if nx == 0 || base == 0 {
                        break 'jogador;
                    }
                    if !(nx as f64 / base as f64 > read_f64_at(0x9689d8)) { // cqe2: a nota até 0,9 da base
                        break 'jogador;
                    }
                    crate::comparison::branch("cqe_2 nota da troca");
                    let a = if nx > base { (*atracao).wrapping_add(20) } else { (*atracao).wrapping_add(5) };
                    *atracao = if a < 0 { 0 } else if a > 100 { 100 } else { a };
                    if cqe2_d(nova + 4) == 0 {
                        let m = alocar(0xe);
                        let l = if m != 0 { lista_nova(m) } else { 0 };
                        cqe2_pd(nova + 4, l);
                    }
                    let t = cqe2_d(nova + 4);
                    cqe2_pd(t + 0xa, cqe2_d(t + 2));
                    loop {
                        let p = cqe2_proximo(cqe2_d(nova + 4));
                        if p == 0 {
                            break;
                        }
                        if cqe2_d(p) == cqe2_d(x) {
                            break 'jogador;
                        }
                    }
                    // ponytail: o 7º byte do item a original copia da pilha sem valor; aqui zero
                    let mut item = [0u8; 8];
                    cqe2_pd(item.as_mut_ptr() as usize, cqe2_d(x));
                    item[5] = estatuto;
                    acrescentar(cqe2_d(nova + 4), item.as_mut_ptr() as usize, 7);
                    let entrada = cqe2_d(reg).wrapping_add((i as i32 * 0x25) as usize);
                    let mut fora: i32 = 0;
                    let v = valor_jogador(cqe2_d(cqe2_d(0xde1f54) + 0x856), vendedor, x, entrada, &mut fora, -1, 1);
                    crate::comparison::branch("cqe_2 jogador na troca");
                    teto = teto.wrapping_sub(v);
                    let r = (cqe2_d(nova + 0x1b) as i32).wrapping_sub(v); // cqe2: o valor dele sai do preço
                    cqe2_pd(nova + 0x1b, if r < 0 { 0 } else { r as u32 as usize });
                    break 'vaga;
                }
                j += 1;
                if j >= 0x32 {
                    break;
                }
            }
        }
        i += 1;
        if i >= 0x19 {
            break;
        }
    }
    destruir_lista(pl);
    1
}

/// A foto da oferta da opção para o comparador: os 0x41 bytes sem o ponteiro +4, sem +0x1a, +0x20..+0x23 e +0x31..+0x34
/// e só o nibble baixo de +0x30 (o que a 0x8e5680 e a 0x8e78f0 não preenchem fica com o que havia na memória nova), e os
/// itens da troca (id, 0, estatuto: o 7º byte vem da pilha da original).
unsafe fn option_offer_snapshot(o: usize) -> Vec<u8> {
    let mut v: Vec<u8> = (0..0x41).filter(|&k| !(4..8).contains(&k) && k != 0x1a && !(0x20..0x24).contains(&k) && !(0x31..0x35).contains(&k))
        .map(|k| if k == 0x30 { cqe2_b(o + k) & 0xf } else { cqe2_b(o + k) }).collect();
    let l = cqe2_d(o + 4);
    if l != 0 {
        v.extend_from_slice(&cqe2_w(l).to_le_bytes());
        let mut no = cqe2_d(l + 2);
        while no != 0 {
            let item = cqe2_d(no);
            v.extend((0..6).map(|k| cqe2_b(item + k)));
            no = cqe2_d(no + 6);
        }
    }
    v
}

/// Comparador da 0x8c8860: lado a lado a partir do mesmo estado de [0xae1080], *saida, *atracao e do sorteador (a
/// 0x8e78f0 sorteia); a cópia roda sem contar sorteios. Confere a devolução, a atração, [0xae1080] e a foto da oferta
/// nova de cada lado; a da cópia é destruída (0x8e5710) e liberada (0x944c94) e fica tudo como a original deixou.
unsafe extern "thiscall" fn comparar_make_exchange_option(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32>(%%I0%%);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let est = crate::rng::STATE_REGIONS;
    let regioes = [(0xae1080usize, 4usize), (saida as usize, 4), (atracao as usize, 1), est[0], est[1]];
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    let r = o(this, comprador, alvo, oferta, saida, atracao, valor);
    let depois = crate::comparison::MemorySnapshot::read(&regioes);
    let (ao, fo) = (*atracao, if r != 0 && *saida != 0 { option_offer_snapshot(*saida) } else { Vec::new() });
    let e_o = cqe2_d(0xae1080);
    antes.write_back();
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(|| make_exchange_option(this, comprador, alvo, oferta, saida, atracao, valor)));
    let (an, fn_) = (*atracao, if n != 0 && *saida != 0 { option_offer_snapshot(*saida) } else { Vec::new() });
    let e_n = cqe2_d(0xae1080);
    if n != 0 && *saida != 0 {
        let s = *saida;
        destruir(s);
        liberar(s);
    }
    depois.write_back();
    SCOREBOARD.record(r == n && ao == an && e_o == e_n && fo == fn_, || format!(
        "make_exchange_option(comprador {:#x}, alvo {:#x}, valor {}): original {} rust {}, atração {} x {}, [0xae1080] {:#x} x {:#x}, oferta {:x?} x {:x?}",
        comprador, alvo, valor, r, n, ao, an, e_o, e_n, fo, fn_));
    r
}

/// A caixa "Error" com "v%s %s.%s %d" (como em show_error_box).
unsafe fn cqe2_caixa(arquivo: usize, linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(arquivo, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    cqe2_pd(0xb67a34, 0);
}

/// O contrato do registro de 79 bytes da pessoa em [0xae2d0c] pelo índice em `campo` (+0 o contrato, +4 o do
/// empréstimo) no vetor de 0x50 bytes em [`vetor`]: id fora do total ([0xae2364]) ou índice negativo, 0.
unsafe fn staff_contract_by_field(pessoa: usize, campo: usize, vetor: usize) -> usize {
    let id = cqe2_d(pessoa) as i32;
    if id >= cqe2_d(0xae2364) as i32 {
        return 0;
    }
    let k = cqe2_d(cqe2_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)) + campo) as i32;
    if k < 0 { 0 } else { (k as usize).wrapping_mul(0x50).wrapping_add(cqe2_d(vetor)) }
}

/// { 0x4ccca0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::schedule_loan_end(pessoa, clube)`, só pela
/// tela de transferências (0x8fbcc0). Sem pessoa, id fora do total ou sem o contrato de empréstimo (+4 do registro de 79
/// bytes, no vetor de this): a caixa (linha 0xe4a). Senão o fim (+0x2d) vira amanhã (0x54c5b0 com 1; 0x417c10) e o tipo
/// (+0x35) vira 7 (os bits 3..5 zerados). O contrato atual (+0 do registro, no vetor de [0xae1050]) em [0xae1080]: do
/// `clube`, o clube da pessoa (+0x39) com técnico humano (0x7875e0, stdcall: o ecx = [0xb7431c] não é lido); de outro,
/// o clube do contrato atual (lido de novo duas vezes) com técnico humano: a volta (0x4db0b0: push clube; push pessoa;
/// mov ecx, this).
unsafe extern "thiscall" fn schedule_loan_end(this: usize, pessoa: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let voltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4db0b0usize);
    let c = if pessoa == 0 { 0 } else { staff_contract_by_field(pessoa, 4, this) };
    if c == 0 {
        cqe2_caixa(0x9a8190, 0xe4a);
        return;
    }
    let mut amanha = [0u32; 2];
    let p = mais(0xae2c90, &mut amanha, 1);
    let mut tmp = [0u8; 8];
    copia(c + 0x2d, tmp.as_mut_ptr() as usize, p);
    cqe2_pb(c + 0x35, (cqe2_b(c + 0x35) & 0xc7) | 7);
    let a = staff_contract_by_field(pessoa, 0, cqe2_d(0xae1050));
    cqe2_pd(0xae1080, a);
    let atual = if a == 0 { 0 } else { cqe2_clube(cqe2_d(a + 4)) };
    if clube == atual {
        let x = cqe2_d(pessoa + 0x39);
        if x == 0 || human_manager_record_checked(x) == 0 {
            return;
        }
    } else {
        let a = staff_contract_by_field(pessoa, 0, cqe2_d(0xae1050));
        cqe2_pd(0xae1080, a);
        if a == 0 || cqe2_clube(cqe2_d(a + 4)) == 0 {
            return;
        }
        let a = staff_contract_by_field(pessoa, 0, cqe2_d(0xae1050));
        cqe2_pd(0xae1080, a);
        let x = if a == 0 { 0 } else { cqe2_clube(cqe2_d(a + 4)) };
        if human_manager_record_checked(x) == 0 {
            return;
        }
    }
    voltar(this, pessoa, clube);
}

/// Comparador da 0x4ccca0: só a original (a volta do empréstimo entrega notícias e mexe no elenco).
unsafe extern "thiscall" fn comparar_schedule_loan_end(this: usize, pessoa: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, pessoa, clube) // ponytail: só a original
}
