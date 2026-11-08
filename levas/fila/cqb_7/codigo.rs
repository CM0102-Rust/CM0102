// ---------- %%LEVA%%a leva: a permissão de trabalho concedida, o estatuto pelo motivo da recusa e a vaga da negociação (transfer_manager.cpp) ----------

unsafe fn cqb7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb7_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato), sem olhar o tipo.
unsafe fn cqb7_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { cqb7_d(0xae23b4).wrapping_add(cqb7_d(c + 4).wrapping_mul(0x245)) }
}

/// A data `a` vem depois da data em `d` (0x456740: thiscall(a, d.dword0, d.dword1), ret 8; a data vai por valor).
unsafe fn cqb7_depois(a: usize, d: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    f(a, cqb7_d(d) as u32, cqb7_d(d + 4) as u32) != 0
}

/// O nome (texto C em [pessoa+4]) contém `agulha` (0x945a60, strstr: cdecl(texto, agulha)) e o clube da oferta (+4 =
/// id; nome em +4 do clube) é o de 0xa8eb48 ("Falkirk"; a comparação de bytes que a original faz em linha).
unsafe fn cqb7_excecao(p: usize, agulha: usize, oferta: usize) -> bool {
    let nome = cqb7_d(p + 4);
    if nome == 0 {
        return false;
    }
    let strstr: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945a60usize);
    if strstr(nome, agulha) == 0 || oferta == 0 {
        return false;
    }
    let mut a = cqb7_d(0xae23b4).wrapping_add(cqb7_d(oferta + 4).wrapping_mul(0x245)) + 4;
    let mut b = 0xa8eb48usize;
    loop {
        let (x, y) = (cqb7_b(a), cqb7_b(b));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        a += 1;
        b += 1;
    }
}

/// As marcas da pessoa na gerência (8 bytes em [this+0x86e] + id × 8): bit 0x20 em +0 (tem registro de permissão),
/// +3 o histórico de 16 jogos da seleção (u16, bit 0 o mais novo) e +5 a conta dos jogos da seleção já vistos (u16).
unsafe fn cqb7_marcas(this: usize, p: usize) -> usize {
    cqb7_d(this + 0x86e).wrapping_add(cqb7_d(p).wrapping_mul(8))
}

/// { 0x8c7950 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::is_work_permit_granted(pessoa, nação,
/// oferta, registro, sorteio)`, pela permissão na lista de observação (0x84b180, sem oferta, registro nem sorteio),
/// pela oferta de contrato aceita (0x8c38b0, com a oferta) e pelas permissões da oferta (0x8c7690). Contrato da pessoa
/// (0x4cf5a0, sem gravar [0xae1080]): de tipo 2 é o "empréstimo", outro é o "contrato". Com o registro de permissão
/// (o dado, ou achado pela busca binária 0x946464 em [this+0x93a], conta [this+0x93e], 14 bytes, quando a marca 0x20
/// existe): sem oferta, sai se jogos (+4) > 3 × recusas (+6) ou recusas < 4; com oferta, a mesma regra se o empréstimo
/// acaba depois da oferta (+0x2d) ou o clube do contrato não é o da oferta (+4), e sai se o empréstimo não acaba depois.
/// Não jogador (+0x69 sem ficha +0x61): reputação (+0xc) acima de 160. Em 2001, os dois casos do Collin e do Devon no
/// Falkirk. Potencial (+0xd da ficha) acima de 8500 (com sorteio: 8000 + sorteio(1000)); salário da oferta (+0xc) acima
/// de 34999; menos de 24 anos com habilidade (+7) acima de 190: sai. Menos de 5 jogos pela seleção (+0x22): não sai;
/// mais de 20 com reputação da nação (+0x8e) acima de 3000 e menos de 32 anos: sai. O histórico anda pelos jogos novos
/// da seleção ([[0xb74318]+4] + id da nação × 0x6e + 2; cada um empurra um bit zero) e a conta fica. A janela: 16
/// jogos, ou os do período de dois anos (0x58d710, até 16) a partir de 2003; com mais de 11 jogos marcados (sorteio:
/// 11 − sorteio(3) + sorteio(3)) ou mais de 70% (70 − sorteio(10) + sorteio(10)): sai.
unsafe extern "thiscall" fn is_work_permit_granted(this: usize, p: usize, _nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cqb7_d(0xae1050), p);
    let (emprestimo, contrato) = if c != 0 && cqb7_b(c + 0x35) & 0x3f != 2 { (0, c) } else { (c, 0) };
    let mut reg = registro;
    let tem_registro = if reg != 0 {
        true
    } else if (p == 0 || (cqb7_d(p) as i32) < cqb7_d(0xae2364) as i32) && cqb7_b(cqb7_marcas(this, p)) & 0x20 != 0 {
        let chave = cqb7_d(p) as u32;
        let busca: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        reg = busca(&chave, cqb7_d(this + 0x93a), cqb7_d(this + 0x93e), 0xe, 0x8e55c0);
        reg != 0
    } else {
        false
    };
    if tem_registro {
        let jogos = core::ptr::read_unaligned((reg + 4) as *const u16) as u32;
        let recusas = core::ptr::read_unaligned((reg + 6) as *const u16) as u32;
        let pela_conta = jogos > recusas * 3 || recusas < 4;
        if oferta == 0 {
            if pela_conta {
                return 1;
            }
        } else {
            let fora = (emprestimo != 0 && cqb7_depois(oferta + 0x2d, emprestimo + 0x2d))
                || (contrato != 0 && cqb7_d(oferta + 4) != cqb7_d(contrato + 4));
            if fora && pela_conta {
                return 1;
            }
            if emprestimo != 0 && !cqb7_depois(oferta + 0x2d, emprestimo + 0x2d) {
                return 1;
            }
        }
    }
    let np = cqb7_d(p + 0x69);
    if np != 0 && cqb7_d(p + 0x61) == 0 {
        return (cqb7_w(np + 0xc) > 0xa0) as u32;
    }
    if cqb7_w(0xae2c92) == 0x7d1 && (cqb7_excecao(p, 0x9b5ac0, oferta) || cqb7_w(0xae2c92) == 0x7d1 && cqb7_excecao(p, 0x9b5ab0, oferta)) {
        crate::comparacao::ramo("cqb_7 excecao de 2001");
        return 1;
    }
    let ficha = cqb7_d(p + 0x61);
    if sorteio != 0 {
        if ficha != 0 && cqb7_w(ficha + 0xd) as i32 > cqb7_sorteio(1000) + 8000 {
            return 1;
        }
    } else if ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o potencial");
        if cqb7_w(ficha + 0xd) > 0x2134 { // cqb7: o potencial acima de 8500
            return 1;
        }
    }
    if oferta != 0 && cqb7_d(oferta + 0xc) as i32 > 0x88b7 {
        return 1;
    }
    let idade = cqb7_b(p + 0x18) as i8;
    if idade < 0x18 && ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o jovem");
        if cqb7_w(ficha + 7) > 0xbe { // cqb7: o jovem acima de 190
            return 1;
        }
    }
    let jogos = cqb7_b(p + 0x22);
    if jogos < 5 {
        return 0;
    }
    if jogos > 0x14 {
        let nacao = cqb7_d(p + 0x1a);
        if nacao != 0 && cqb7_w(nacao + 0x8e) > 0xbb8 && idade < 0x20 {
            return 1;
        }
    }
    // o histórico pelos jogos novos da seleção
    let selecao = cqb7_w(cqb7_d(cqb7_d(0xb74318) + 4).wrapping_add(cqb7_d(cqb7_d(p + 0x1a)).wrapping_mul(0x6e)) + 2);
    let novos = selecao.wrapping_sub(cqb7_w(cqb7_marcas(this, p) + 5));
    if novos > 0 {
        for _ in 0..novos {
            let h = cqb7_marcas(this, p) + 3;
            core::ptr::write_unaligned(h as *mut u16, core::ptr::read_unaligned(h as *const u16) << 1);
            *(h as *mut u8) &= 0xfe;
        }
    }
    core::ptr::write_unaligned((cqb7_marcas(this, p) + 5) as *mut i16, selecao);
    let ano = cqb7_w(0xae2c92) as u16;
    let janela: i16 = if (ano as i32) - 2 < 0x7d1 {
        0x10
    } else {
        let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
        let ate = mes(0xae2c90);
        let de = mes(0xae2c90);
        let periodo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16 = core::mem::transmute(0x58d710usize);
        let n = periodo(cqb7_d(cqb7_d(0xb74318) + 0x1c), cqb7_d(p + 0x1a), de, (ano as u32).wrapping_sub(2), ate, ano as u32) as i16;
        if n > 0x10 { 0x10 } else { n }
    };
    let mut marcados: i16 = 0;
    let mut k: i16 = 0;
    while k < janela && k < 0x10 {
        let mascara = 1u16 << k; // ftol(pow(2.0, k)) (0x945ff0 e 0x945640): exato de 0 a 15
        if core::ptr::read_unaligned((cqb7_marcas(this, p) + 3) as *const u16) & mascara != 0 {
            marcados += 1;
        }
        k += 1;
    }
    if sorteio != 0 {
        let a = cqb7_sorteio(3);
        let limite = 0xb - a + cqb7_sorteio(3);
        if marcados as i32 > limite {
            return 1;
        }
        if janela == 0 {
            return 0;
        }
        let b = cqb7_sorteio(10);
        let limite = 0x46 - b + cqb7_sorteio(10);
        return ((marcados as i32 * 100) / janela as i32 > limite) as u32;
    }
    crate::comparacao::ramo("cqb_7 jogos pela selecao");
    if marcados > 0xb { // cqb7: mais de 11 jogos marcados
        return 1;
    }
    if janela == 0 {
        return 0;
    }
    ((marcados as i32 * 100) / janela as i32 > 0x46) as u32
}

/// Comparador da 0x8c7950: lado a lado nas marcas da pessoa (8 bytes), no sorteio e na devolução (a cópia em
/// sem_contar).
unsafe extern "thiscall" fn comparar_is_work_permit_granted(this: usize, p: usize, nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(%%I0%%);
    let mut regioes = vec![crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1]];
    if p != 0 && (cqb7_d(p) as i32) < cqb7_d(0xae2364) as i32 {
        regioes.push((cqb7_marcas(this, p), 8));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, p, nacao, oferta, registro, sorteio),
        || crate::aleatorio::sem_contar(|| is_work_permit_granted(this, p, nacao, oferta, registro, sorteio)));
    PLACAR.registrar(igual && r == n, || format!("is_work_permit_granted(pessoa {:#x}, oferta {:#x}, registro {:#x}, sorteio {}) original={} {:x?} rust={} {:x?}", p, oferta, registro, sorteio, r, mo, n, mr));
    r
}

/// { 0x8c5800 } (thiscall, ret 8, transfer_manager.cpp; o this não é usado): `transfer_manager_get_squad_status_for_reason
/// (pessoa, motivo)`, pela tela da oferta. O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) dá o clube; o
/// contrato com ele (0x4cf5f0); sem ele, 0. O motivo (16 bits com sinal) 0x200 dá 1, 0x400 dá 2, 0x2000 dá 5; os
/// outros, o estatuto do contrato (+0x4f >> 4). Devolve no al.
unsafe extern "thiscall" fn transfer_manager_get_squad_status_for_reason(_this: usize, p: usize, motivo: u32) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cqb7_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let cc = com_clube(cqb7_d(0xae1050), p, cqb7_clube_do_contrato(c));
    if cc == 0 {
        return 0;
    }
    match motivo as i16 as i32 {
        0x200 => 1,
        0x400 => 2,
        0x2000 => 5,
        _ => (cqb7_b(cc + 0x4f) >> 4) as u32,
    }
}

/// Comparador da 0x8c5800: lado a lado em [0xae1080] e no al devolvido.
unsafe extern "thiscall" fn comparar_transfer_manager_get_squad_status_for_reason(this: usize, p: usize, motivo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, p, motivo), || transfer_manager_get_squad_status_for_reason(this, p, motivo));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("transfer_manager_get_squad_status_for_reason(pessoa {:#x}, motivo {:#x}) original={} {:x?} rust={} {:x?}", p, motivo, r as u8, mo, n as u8, mr));
    r
}

/// { 0x8c6200 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::get_negotiation_slot(i)`, pela resposta
/// automática às notícias (0x782f70) e pelas telas: o endereço da vaga `i` do vetor de negociações ([this+0xc], conta
/// [this+0x10], com sinal); fora da conta, o começo do vetor.
unsafe extern "thiscall" fn get_negotiation_slot(this: usize, i: i32) -> usize {
    if i < cqb7_d(this + 0x10) as i32 {
        cqb7_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(4))
    } else {
        cqb7_d(this + 0xc)
    }
}

/// Comparador da 0x8c6200: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_get_negotiation_slot(this: usize, i: i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I2%%)(this, i);
    let n = get_negotiation_slot(this, i);
    PLACAR.registrar(r == n, || format!("get_negotiation_slot({}) original={:#x} rust={:#x}", i, r, n));
    r
}
