// ---------- %%LEVA%%a leva: as transferências futuras: a do jogador, tirar a da oferta e acrescentar a da oferta acertada (transfer_manager.cpp) ----------

unsafe fn cpr3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpr3_pessoa(id: usize) -> usize {
    cpr3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O byte de marcas da pessoa de id `id` no registro de 8 bytes em [this+0x86e].
unsafe fn cpr3_marca(this: usize, id: usize) -> *mut u8 {
    cpr3_d(this + 0x86e).wrapping_add(id.wrapping_mul(8)) as *mut u8
}

/// Na lista de índices de ofertas `l` (o cursor volta ao começo), o índice da primeira oferta de estado 0x13
/// (transferência futura), ou None.
unsafe fn cpr3_futura_na_lista(this: usize, l: usize) -> Option<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    cpr3_por(l + 0xa, cpr3_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if cpr3_b(cpr3_d(cpr3_d(this).wrapping_add(cpr3_d(e).wrapping_mul(4))) + 0x2c) == 0x13 {
            return Some(cpr3_d(e));
        }
        e = proximo(l);
    }
    None
}

/// O corpo da 0x8cca10 sobre a lista local `l` (já construída).
unsafe fn cpr3_futura(this: usize, jogador: usize, saida: *mut usize, l: usize) -> u32 {
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    if *cpr3_marca(this, cpr3_d(jogador)) & 0x10 == 0 {
        return 0;
    }
    let mut k = None;
    if ofertas(this, jogador, l, 0) != 0 {
        k = cpr3_futura_na_lista(this, l).filter(|&i| i != usize::MAX);
    }
    if k.is_none() {
        limpar(l);
        if trocas(this, jogador, l) != 0 {
            k = cpr3_futura_na_lista(this, l).filter(|&i| i != usize::MAX);
        }
    }
    let Some(k) = k else {
        // ponytail: a marca é relida pela original depois das chamadas; o vetor [this+0x86e] não muda entre elas
        *cpr3_marca(this, cpr3_d(jogador)) &= 0xef; // sem transferência futura: a marca sai
        return 0;
    };
    crate::comparacao::ramo("cpr_3 futura achada");
    if !saida.is_null() {
        let n = cpr3_d(this + 0x928) as i32;
        for i in 0..n.max(0) as usize {
            let r = cpr3_d(this + 0x920).wrapping_add(i * 0xc);
            if cpr3_d(r + 8) == k {
                *saida = r;
                break;
            }
        }
    }
    1 // a transferência futura existe
}

/// { 0x8cca10 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::get_future_transfer(jogador, saida)`: sem a marca 0x10 ([this+0x86e] + id × 8), 0. Pelas ofertas
/// pelo jogador (0x8c5cf0) e, sem achar, pelas trocas que o oferecem (0x8c5e40, com a lista esvaziada por 0x68d080), a
/// primeira de estado 0x13 com índice ≠ −1; com `saida`, o registro de 12 bytes do vetor [this+0x920] (conta +0x928)
/// com a oferta em +8; 1. Nenhuma: tira a marca 0x10 e 0.
unsafe extern "thiscall" fn get_future_transfer(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = cpr3_futura(this, jogador, saida, l);
    soltar(l);
    r
}

/// { 0x8cbf70 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_future_transfer(oferta)`: tira a
/// marca 0x10 do jogador (+0xc) e dos jogadores da troca (lista +4, o cursor volta ao começo); no vetor [this+0x920]
/// (conta +0x928), o primeiro registro com a oferta (+0x20 da oferta, +8 do registro) sai: os seguintes descem um
/// (0x8cbf40) e a conta cai de 1.
unsafe extern "thiscall" fn remove_future_transfer(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cbf40usize);
    *cpr3_marca(this, cpr3_d(oferta + 0xc)) &= 0xef;
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(cpr3_d(oferta + 4));
        while e != 0 {
            *cpr3_marca(this, cpr3_d(e)) &= 0xef;
            e = proximo(cpr3_d(oferta + 4));
        }
    }
    let n = cpr3_d(this + 0x928) as i32;
    let alvo = cpr3_d(oferta + 0x20);
    for i in 0..n.max(0) {
        if cpr3_d(cpr3_d(this + 0x920).wrapping_add(i as usize * 0xc) + 8) == alvo {
            crate::comparacao::ramo("cpr_3 tirar futura");
            let mut j = i;
            while j < cpr3_d(this + 0x928) as i32 - 1 {
                let r = cpr3_d(this + 0x920).wrapping_add(j as usize * 0xc);
                copiar(r, r + 0xc);
                j += 1;
            }
            cpr3_por(this + 0x928, cpr3_d(this + 0x928).wrapping_sub(1));
            return;
        }
    }
}

/// { 0x8cb9a0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::add_future_transfer(oferta, data)`:
/// acrescenta ao vetor [this+0x920] (capacidade +0x924, conta +0x928; cheio, cresce de 10 em 10, sem memória: a caixa
/// 0x27c3 e nada) o registro de 12 bytes com a `data` (0x417c10) e a oferta (+0x20 em +8) e reordena (qsort 0x945333
/// com 0x778e90). A oferta fica com +0x2d = 1 e o estado 0x13. Suspende os clubes que miram o jogador (0x842b00 de
/// [this+0x856], com 0) e liga a marca 0x10 dele; com contrato (0x4cf5a0) marcado em +0x49, a oferta vai a +0x4a do
/// contrato. O mesmo (suspender e marcar) para cada jogador da troca (lista +4).
unsafe extern "thiscall" fn add_future_transfer(this: usize, oferta: usize, data: usize) {
    crate::comparacao::ramo("cpr_3 acrescentar futura");
    if !cpl3_crescer(this, 0x920, 4, 10, 0xc, 0x27c3, 0xa8ec74) {
        return;
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut [u16; 4], usize) -> usize = core::mem::transmute(0x417c10usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut temp = [0u16; 4];
    copia(cpr3_d(this + 0x920).wrapping_add(cpr3_d(this + 0x928).wrapping_mul(0xc)), &mut temp, data);
    cpr3_por(cpr3_d(this + 0x920).wrapping_add(cpr3_d(this + 0x928).wrapping_mul(0xc)) + 8, cpr3_d(oferta + 0x20));
    let n = cpr3_d(this + 0x928).wrapping_add(1);
    cpr3_por(this + 0x928, n);
    qsort(cpr3_d(this + 0x920), n as u32, 0xc, 0x778e90);
    *((oferta + 0x2d) as *mut u8) = 1;
    *((oferta + 0x2c) as *mut u8) = 0x13; // transferência futura
    suspender(cpr3_d(this + 0x856), cpr3_pessoa(cpr3_d(oferta + 0xc)), 0);
    *cpr3_marca(this, cpr3_d(oferta + 0xc)) |= 0x10; // a marca da transferência futura
    let c = contract_of(cpr3_d(0xae1050), cpr3_pessoa(cpr3_d(oferta + 0xc)));
    if c != 0 && cpr3_b(c + 0x49) != 0 {
        cpr3_por(c + 0x4a, cpr3_d(oferta + 0x20));
    }
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(cpr3_d(oferta + 4));
        while e != 0 {
            suspender(cpr3_d(this + 0x856), cpr3_pessoa(cpr3_d(e)), 0);
            *cpr3_marca(this, cpr3_d(e)) |= 0x10;
            e = proximo(cpr3_d(oferta + 4));
        }
    }
}

/// Comparador: lado a lado pela marca do jogador e por `saida`, e pela devolução (tirar a marca é o único outro efeito;
/// a cópia, depois da original, acha o mesmo).
unsafe extern "thiscall" fn comparar_get_future_transfer(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(%%I0%%);
    let regioes = [(cpr3_marca(this, cpr3_d(jogador)) as usize, 1), (saida as usize, 4)];
    let n = if saida.is_null() { 1 } else { 2 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, saida), || get_future_transfer(this, jogador, saida));
    PLACAR.registrar(igual && r == c, || format!("get_future_transfer(jogador {}) original={} rust={} {:x?} x {:x?}", cpr3_d(jogador), r, c, mo, mr));
    r
}

/// Comparador: lado a lado pela conta e pelo vetor das futuras, pelas marcas do jogador e dos da troca e pelo cursor
/// da lista da troca (não aloca nem libera).
unsafe extern "thiscall" fn comparar_remove_future_transfer(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let mut regioes = vec![(this + 0x928, 4), (cpr3_marca(this, cpr3_d(oferta + 0xc)) as usize, 1)];
    let n = cpr3_d(this + 0x928) as i32;
    if n > 0 {
        regioes.push((cpr3_d(this + 0x920), n as usize * 0xc));
    }
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            regioes.push((cpr3_marca(this, cpr3_d(e)) as usize, 1));
            e = proximo(l);
        }
        regioes.push((l + 0xa, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || remove_future_transfer(this, oferta));
    PLACAR.registrar(igual, || format!("remove_future_transfer(oferta {:x}) {:x?} x {:x?}", cpr3_d(oferta + 0x20), mo, mr));
}

/// Comparador: a original acrescenta (pode realocar), reordena e suspende as listas de alvos: só ela, conferida pelo
/// plano: a conta sobe de 1, o registro (data e oferta) está no vetor, o estado 0x13 com +0x2d = 1 e a marca 0x10 do
/// jogador.
unsafe extern "thiscall" fn comparar_add_future_transfer(this: usize, oferta: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    let n = cpr3_d(this + 0x928);
    let mut alvo = [0u8; 0xc];
    alvo[..8].copy_from_slice(core::slice::from_raw_parts(data as *const u8, 8));
    alvo[8..].copy_from_slice(&(cpr3_d(oferta + 0x20) as u32).to_le_bytes());
    o(this, oferta, data);
    if cpr3_d(this + 0x920) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpr3_d(this + 0x928);
    let marca = *cpr3_marca(this, cpr3_d(oferta + 0xc));
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpr3_d(this + 0x920), conta, 0xc, &alvo) && cpr3_b(oferta + 0x2c) == 0x13 && cpr3_b(oferta + 0x2d) == 1 && marca & 0x10 != 0;
    PLACAR.registrar(ok, || format!("add_future_transfer(oferta {:x}): conta {} -> {}, estado {:#x}, marca {:#x}", cpr3_d(oferta + 0x20), n, conta, cpr3_b(oferta + 0x2c), marca));
}
