// ---------- %%LEVA%%a leva: as listas de posição da lista de observação: montar as listas, ordená-las a cada dia e os clubes interessados no jogador (shortlist_manager.cpp) ----------

unsafe fn cpo8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo8_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpo8_clube(id: usize) -> usize {
    cpo8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpo8_pessoa(id: usize) -> usize {
    cpo8_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0, guardado em [0xae1080]), o clube dele (id em +4) e o contrato com esse
/// clube (0x4cf5f0).
unsafe fn contract_with_own_club(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = contract_of(cpo8_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpo8_clube(cpo8_d(c + 4)) } else { 0 };
    com_clube(cpo8_d(0xae1050), p, clube)
}

/// { 0x839e60 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::build_position_lists(não usado)`: libera
/// os 32 vetores das listas de posição (+0x18: 16 pela pessoa, +0x58: 16 pela chave do contrato); o tamanho deles
/// (+0xd8) é o maior número de pessoas (todas, humanos inclusive) que servem para uma das 16 vagas (0x844300); aloca os
/// 32 com 12 bytes por entrada (sem memória: a caixa "Error" e o jogo para) e zera as contas (+0x98). Depois, cada
/// pessoa fora dos 16 humanos, em cada vaga que serve (conta no limite: a caixa e o jogo para): as duas entradas com o
/// id dela (+0), o id da nação (+4; sem nação em +0x1a, a 0x90cfc0 sorteia uma e a grava na pessoa), na primeira o valor
/// (+0x52) e na segunda o +0x3d do contrato com o clube do contrato (sem contrato: 100). Roda a cada 30 dias.
unsafe extern "thiscall" fn build_position_lists(this: usize, _nao_usado: u32) {
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    // ponytail: a original empurra a vaga num dword com 3 bytes de lixo da pilha acima; a 0x844300 só lê o byte
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x844300usize);
    crate::comparacao::ramo("cpo_8 montar listas");
    for k in 0..32usize {
        let v = this + 0x18 + k * 4;
        if cpo8_d(v) != 0 {
            livrar(cpo8_d(v));
            cpo8_w(v, 0);
        }
    }
    cpo8_w(this + 0xd8, 0);
    for vaga in 0..16u32 {
        let base = cpo8_d(0xae23bc);
        let mut n = 0i32;
        let mut i = 0usize;
        while (i as i32) < cpo8_i(0xae2364) {
            if serve(base.wrapping_add(i * 0x6e), vaga) != 0 {
                n += 1;
            }
            i += 1;
        }
        if n > cpo8_i(this + 0xd8) {
            cpo8_w(this + 0xd8, n as u32);
        }
    }
    for k in 0..32usize {
        let v = alocar(cpo8_d(this + 0xd8).wrapping_mul(12) as u32);
        cpo8_w(this + 0x18 + k * 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x10c);
        }
    }
    for k in 0..16usize {
        cpo8_w(this + 0x98 + k * 4, 0);
    }
    let base = cpo8_d(0xae23bc);
    let mut i = 0usize;
    while (i as i32) < cpo8_i(0xae2364).wrapping_sub(0x10) {
        let p = base.wrapping_add(i * 0x6e);
        for vaga in 0..16u32 {
            if serve(p, vaga) == 0 {
                continue;
            }
            let pv = vaga as usize;
            let conta = cpo8_i(this + 0x98 + pv * 4);
            if conta >= cpo8_i(this + 0xd8) {
                fatal_error_at(0xa825a4, 0x121);
            }
            let o = (conta as usize).wrapping_mul(12);
            let (l1, l2) = (cpo8_d(this + 0x18 + pv * 4).wrapping_add(o), cpo8_d(this + 0x58 + pv * 4).wrapping_add(o));
            cpo8_w(l1, i as u32);
            cpo8_w(l2, i as u32);
            let mut nacao = cpo8_d(p + 0x1a);
            if nacao == 0 {
                let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
                nacao = cpo8_d(0xae23a8).wrapping_add((sortear(cpo8_i(0xae2350)) as usize).wrapping_mul(0x122));
                cpo8_w(p + 0x1a, nacao as u32);
            }
            cpo8_w(l1 + 4, cpo8_d(nacao) as u32);
            cpo8_w(l2 + 4, cpo8_d(cpo8_d(p + 0x1a)) as u32);
            cpo8_w(l1 + 8, cpo8_d(p + 0x52) as u32);
            let k = contract_with_own_club(p);
            cpo8_w(l2 + 8, if k != 0 { cpo8_b(k + 0x3d) as i8 as i32 as u32 } else { 100 });
            cpo8_w(this + 0x98 + pv * 4, (conta + 1) as u32);
        }
        i += 1;
    }
}

/// { 0x83a780 } (thiscall, ret 0, shortlist_manager.cpp): `Shortlist_Manager::sort_position_lists()`: a cada dia,
/// refaz a chave (+8) das entradas das 16 vagas e ordena os dois vetores de cada uma (0x945333 com 0x83b670: pela chave
/// e pelo +4). Na primeira lista: o valor da pessoa (+0x52) se o contrato com o clube existe e a faixa do vencimento
/// (0x867d30) é 0, senão 0; o jogador de clube humano (0x600a50) na lista de transferências (0x5444a0) perde 1.500.000,
/// no mínimo um terço do valor. Na vaga 15 só a primeira, com o valor. Na segunda: −1000 com o bit 2 do +0x4e do
/// contrato, 2000 com os bits 0 ou 3 ou sem jogador (+0x61), sem clube, sem divisão (+0x57) ou divisão sem marca em
/// [0xadadfc] com reputação abaixo de 6000, ou sem contrato; senão o +0x3d do contrato.
unsafe extern "thiscall" fn sort_position_lists(this: usize) {
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    for vaga in 0..16usize {
        let mut j = 0i32;
        let mut o = 0usize;
        while j < cpo8_i(this + 0x98 + vaga * 4) {
            let p = cpo8_pessoa(cpo8_d(cpo8_d(this + 0x18 + vaga * 4) + o));
            let k = contract_with_own_club(p);
            if vaga == 15 {
                cpo8_w(cpo8_d(this + 0x54) + o + 8, cpo8_d(p + 0x52) as u32);
                j += 1;
                o += 12;
                continue;
            }
            if k != 0 && band(k) == 0 {
                cpo8_w(cpo8_d(this + 0x18 + vaga * 4) + o + 8, cpo8_d(p + 0x52) as u32);
                let clube = cpo8_d(p + 0x39);
                if clube != 0 && humano(clube, 0, 1, 0, 0) != 0 && listado(p, 0) != 0 {
                    let e = cpo8_d(this + 0x18 + vaga * 4) + o + 8;
                    cpo8_w(e, cpo8_i(e).wrapping_add(-0x16e360) as u32);
                    let terco = cpo8_i(p + 0x52) / 3;
                    if cpo8_i(e) < terco {
                        cpo8_w(e, terco as u32);
                    }
                }
            } else {
                if k != 0 {
                    crate::comparacao::ramo("cpo_8 band do contrato");
                }
                cpo8_w(cpo8_d(this + 0x18 + vaga * 4) + o + 8, 0);
            }
            // a segunda lista
            let p2 = cpo8_pessoa(cpo8_d(cpo8_d(this + 0x58 + vaga * 4) + o));
            let k2 = contract_with_own_club(p2);
            let clube = cpo8_d(p2 + 0x39);
            let chave: i32 = if cpo8_d(p2 + 0x61) == 0 || clube == 0 || cpo8_d(clube + 0x57) == 0 {
                0x7d0
            } else if cpo8_d(cpo8_d(0xadadfc) + cpo8_d(cpo8_d(clube + 0x57)).wrapping_mul(4)) == 0
                && core::ptr::read_unaligned((clube + 0x80) as *const i16) < 0x1770
            {
                0x7d0
            } else if k2 == 0 {
                0x7d0
            } else if cpo8_b(k2 + 0x4e) & 2 != 0 {
                -1000
            } else if cpo8_b(k2 + 0x4e) & 9 != 0 {
                0x7d0
            } else {
                cpo8_b(k2 + 0x3d) as i8 as i32
            };
            cpo8_w(cpo8_d(this + 0x58 + vaga * 4) + o + 8, chave as u32);
            j += 1;
            o += 12;
        }
        let n = cpo8_d(this + 0x98 + vaga * 4) as u32;
        ordenar(cpo8_d(this + 0x18 + vaga * 4), n, 12, 0x83b670);
        let n = cpo8_d(this + 0x98 + vaga * 4) as u32;
        ordenar(cpo8_d(this + 0x58 + vaga * 4), n, 12, 0x83b670);
    }
}

/// { 0x842180 } (thiscall, ret 0x14, shortlist_manager.cpp): `Shortlist_Manager::collect_interested_clubs(jogador,
/// saída, ignorar, modo, só_humanos)`: esvazia a lista `saída` (0x68d080) e acrescenta (0x68d0d0, 8 bytes: id do
/// jogador, id do clube) os clubes interessados: com modo ≠ 3, os 16 humanos (do último para o primeiro) com +0x5f e
/// clube que têm o jogador na lista do olheiro (0x7f7490; com modo 2, fora os clubes que o humano da vez controla,
/// 0x600c90); sem `só_humanos`, os pares (jogador, clube) do vetor pela pessoa (+0, conta +0x10; 0x946464 até o primeiro
/// do jogador) cujo clube não está suspenso (+8 da entrada de 9 bytes do clube em +0xdc abaixo de 100, salvo com
/// `ignorar`), não é o do jogador e põe nele (+7 da vaga de 0x25 bytes com o jogador em +0xf; sem vaga, a primeira)
/// mais que o valor (+0x52).
unsafe extern "thiscall" fn collect_interested_clubs(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    if saida != 0 {
        let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
        limpar(saida);
    }
    if modo as u8 != 3 {
        let controla: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
        for k in 0..16i32 {
            let h = cpo8_pessoa(cpo8_i(0xae2364).wrapping_sub(k).wrapping_sub(1) as usize);
            let clube = cpo8_d(h + 0x39);
            if cpo8_b(h + 0x5f) == 0 || clube == 0 {
                continue;
            }
            if controla(clube, 0, 1) != 0 && modo as u8 == 2 {
                continue;
            }
            if in_list(cpo8_d(0xb74448), jogador, h) != 0 {
                let par = [cpo8_d(jogador) as u32, cpo8_d(clube) as u32];
                acrescentar(saida, par.as_ptr() as usize, 8);
            }
        }
    }
    if so_humanos != 0 {
        return;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpo8_d(jogador) as u32, 0xffff_ffff];
    let mut e = bsearch(chave.as_ptr(), cpo8_d(this), cpo8_i(this + 0x10), 8, 0x757ea0);
    if e == 0 {
        return;
    }
    let base = cpo8_d(this);
    let ultimo = base.wrapping_add((cpo8_i(this + 0x10) as usize).wrapping_mul(8)).wrapping_sub(8);
    let id = cpo8_d(jogador);
    if e > base {
        loop {
            let x = cpo8_d(e - 8);
            e -= 8;
            if x != id {
                e += 8;
                break;
            }
            if e <= base {
                break;
            }
        }
    }
    if cpo8_d(e) != id {
        return;
    }
    loop {
        let clube_id = cpo8_d(e + 4);
        let entrada = cpo8_d(this + 0xdc).wrapping_add(clube_id.wrapping_mul(9));
        let vagas = cpo8_d(entrada);
        let mut k = 0usize;
        while k < 0x28 && cpo8_d(vagas + 0xf + k * 0x25) != cpo8_d(jogador) {
            k += 1;
        }
        let valor = if k < 0x28 { cpo8_i(vagas + k * 0x25 + 7) } else { cpo8_i(vagas + 7) };
        if !(ignorar == 0 && cpo8_b(entrada + 8) >= 100) && valor > cpo8_i(jogador + 0x52) {
            let c = cpo8_d(jogador + 0x39);
            if !(c != 0 && cpo8_d(c) == clube_id) {
                crate::comparacao::ramo("cpo_8 clube interessado");
                acrescentar(saida, e, 8);
            }
        }
        if e == ultimo {
            return;
        }
        let prox = cpo8_d(e + 8);
        e += 8;
        if prox != cpo8_d(jogador) {
            return;
        }
    }
}

/// Os itens (8 bytes) de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
unsafe fn llist_pairs(lista: usize) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let mut no = cpo8_d(lista + 2);
    while no != 0 {
        let x = cpo8_d(no);
        v.push((cpo8_d(x) as u32, cpo8_d(x + 4) as u32));
        no = cpo8_d(no + 6);
    }
    v
}

/// Comparador: só a original.
// ponytail: a original solta e aloca os 32 vetores (e pode sortear a nação de quem não tem); não roda duas vezes. Confere
// o resultado: as duas listas de cada vaga com os mesmos ids, cada entrada servindo para a vaga, a conta no limite.
unsafe extern "thiscall" fn comparar_build_position_lists(this: usize, nao_usado: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, nao_usado);
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x844300usize);
    let mut ruim = None;
    'f: for vaga in 0..16usize {
        let n = cpo8_i(this + 0x98 + vaga * 4);
        if n > cpo8_i(this + 0xd8) {
            ruim = Some((vaga, -1));
            break;
        }
        for j in 0..n.max(0) as usize {
            let (a, b) = (cpo8_d(this + 0x18 + vaga * 4) + j * 12, cpo8_d(this + 0x58 + vaga * 4) + j * 12);
            if cpo8_d(a) != cpo8_d(b) || serve(cpo8_pessoa(cpo8_d(a)), vaga as u32) == 0 {
                ruim = Some((vaga, j as i32));
                break 'f;
            }
        }
    }
    PLACAR.registrar(ruim.is_none(), || format!("build_position_lists: vaga e entrada {:?}", ruim));
}

/// Comparador: lado a lado pelos 32 vetores (12 bytes por entrada) e por [0xae1080].
unsafe extern "thiscall" fn comparar_sort_position_lists(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let mut reg = vec![(0xae1080usize, 4usize)];
    for vaga in 0..16usize {
        let n = cpo8_i(this + 0x98 + vaga * 4);
        if n > 0 {
            reg.push((cpo8_d(this + 0x18 + vaga * 4), n as usize * 12));
            reg.push((cpo8_d(this + 0x58 + vaga * 4), n as usize * 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || sort_position_lists(this));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0).take(8).collect();
        format!("sort_position_lists: vetores diferentes {:x?}", dif)
    });
}

/// Comparador: a original na lista de saída e a cópia numa lista nova (0x68d020, solta com 0x68d080); compara os pares.
unsafe extern "thiscall" fn comparar_collect_interested_clubs(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I2%%);
    o(this, jogador, saida, ignorar, modo, so_humanos);
    if saida == 0 {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let mut nova = [0u8; 0x10];
    let l = nova.as_mut_ptr() as usize;
    criar(l);
    collect_interested_clubs(this, jogador, l, ignorar, modo, so_humanos);
    let (a, b) = (llist_pairs(saida), llist_pairs(l));
    limpar(l);
    PLACAR.registrar(a == b, || format!("collect_interested_clubs({:#x}) original={:x?} rust={:x?}", jogador, a, b));
}
