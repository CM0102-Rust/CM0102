// ---------- %%LEVA%%a leva: as permissões de trabalho (transfer_manager.cpp): a oferta que precisa de permissão e a atualização anual ----------

unsafe fn cph6_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph6_pessoa(i: u32) -> usize {
    (cph6_d(0xae23bc) as usize).wrapping_add((i as usize).wrapping_mul(110))
}

unsafe fn cph6_clube(i: u32) -> usize {
    let i = i as usize;
    (cph6_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
}

/// A pessoa precisa de permissão para jogar no clube (0x8c7560, original, com 0 = sem ignorar as dadas)?
unsafe fn cph6_precisa(this: usize, pessoa: usize, clube: usize) -> bool {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    precisa(this, pessoa, clube, 0) != 0
}

/// { 0x8c7470 } (thiscall, ret 4, transfer_manager.cpp): `transfer_manager_offer_needs_work_permit(oferta)`: 1 se o
/// jogador da oferta (+0xc) precisa de permissão no clube da oferta (+8), ou se algum dos jogadores de troca (a lista
/// em +4, do começo: o cursor +0xa volta ao +2 e anda pela 0x68d1b0) precisa no clube do contrato dele (0x4cf5a0 em
/// [0xae1050], gravado em [0xae1080]; sem contrato, clube nulo); senão 0.
unsafe extern "thiscall" fn transfer_manager_offer_needs_work_permit(this: usize, oferta: usize) -> u32 {
    if cph6_precisa(this, cph6_pessoa(cph6_d(oferta + 0xc)), cph6_clube(cph6_d(oferta + 8))) {
        return 1; // o jogador da oferta
    }
    let lista = cph6_d(oferta + 4) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cph6_d(lista + 2)); // o cursor no começo
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mut item = proximo(lista);
    while item != 0 {
        let c = contract_of(cph6_d(0xae1050) as usize, cph6_pessoa(cph6_d(item)));
        *(0xae1080 as *mut usize) = c;
        let clube = if c != 0 { cph6_clube(cph6_d(c + 4)) } else { 0 };
        if cph6_precisa(this, cph6_pessoa(cph6_d(item)), clube) {
            return 1;
        }
        item = proximo(lista);
    }
    0
}

/// { 0x8c7330 } (thiscall, transfer_manager.cpp): `TRANSFER_MANAGER::update_work_permits`: no vetor de permissões
/// ([this+0x93a], [this+0x93e] registros de 14 bytes: +0 pessoa, +8 ano, +0xa nação), a marca da pessoa (byte em
/// [this+0x86e] + 8 × id) sem o bit 0x20 é zerada inteira; com mais de 4 anos (ano do jogo − +8) a pessoa ganha a
/// segunda nacionalidade (+0x1e = nação +0xa, 0x122 bytes em [0xae23a8]), perde o bit 0x20 e a permissão sai (pessoa
/// −1); quem não é jogador (+0x61) também perde. Saindo alguma: reordena (qsort 0x945333 com 0x8e55c0) e desconta as
/// do fim com pessoa −1.
unsafe extern "thiscall" fn update_work_permits(this: usize) {
    let n = (this + 0x93e) as *mut i32;
    let mut saiu = false;
    let mut i = 0i32;
    let mut off = 0usize;
    while i < *n {
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(off);
        let pessoa = cph6_pessoa(cph6_d(r));
        let marca = |p: usize| ((cph6_d(this + 0x86e) as usize).wrapping_add((cph6_d(p) as usize).wrapping_mul(8))) as *mut u8;
        if *marca(pessoa) & 0x20 == 0 {
            *marca(pessoa) = 0; // a marca sem permissão
        }
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(off);
        let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(*((r + 8) as *const u16) as i32);
        if anos > 4 {
            let nacao = cph6_d(r + 0xa) as usize;
            core::ptr::write_unaligned((pessoa + 0x1e) as *mut usize, (cph6_d(0xae23a8) as usize).wrapping_add(nacao.wrapping_mul(0x122)));
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((cph6_d(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true;
        } else if cph6_d(pessoa + 0x61) == 0 {
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((cph6_d(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true; // quem não é jogador
        }
        i += 1;
        off = off.wrapping_add(0xe);
    }
    if !saiu {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cph6_d(this + 0x93a) as usize, *n, 0xe, 0x8e55c0);
    let mut k = *n - 1;
    while k >= 0 {
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(k as usize * 0xe);
        if cph6_d(r) != u32::MAX {
            break;
        }
        *n -= 1;
        k -= 1;
    }
}

/// Comparador: lado a lado pela ficha [0xae1080] e pelo cursor (+0xa) da lista de troca.
unsafe extern "thiscall" fn comparar_transfer_manager_offer_needs_work_permit(this: usize, oferta: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let lista = cph6_d(oferta + 4) as usize;
    let mut reg = vec![(0xae1080, 4)];
    if lista != 0 {
        reg.push((lista + 0xa, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, oferta), || transfer_manager_offer_needs_work_permit(this, oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_offer_needs_work_permit({:x}) original={} rust={} {:x?} x {:x?}", oferta, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da pessoa e pela segunda
/// nacionalidade (+0x1e). Permissões com pessoa negativa não entram nas regiões (o original leria fora do vetor).
unsafe extern "thiscall" fn comparar_update_work_permits(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let n = (*((this + 0x93e) as *const i32)).max(0) as usize;
    let vetor = cph6_d(this + 0x93a) as usize;
    let mut reg = vec![(this + 0x93a, 8), (vetor, n * 0xe)];
    for k in 0..n {
        let id = cph6_d(vetor + k * 0xe);
        if (id as i32) < 0 {
            continue;
        }
        let pessoa = cph6_pessoa(id);
        reg.push(((cph6_d(this + 0x86e) as usize).wrapping_add((cph6_d(pessoa) as usize).wrapping_mul(8)), 1));
        reg.push((pessoa + 0x1e, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || update_work_permits(this));
    PLACAR.registrar(igual, || format!("update_work_permits {:x?} x {:x?}", mo, mr));
}
