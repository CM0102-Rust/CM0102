// ---------- %%LEVA%%a leva: o pedido de transferência do jogador, os cargos com contrato e quem não é da diretoria (contract_manager.cpp) ----------

unsafe fn cps1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x4d8610 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// handle_transfer_request(contrato)`: contrato nulo: a caixa "Error" 0x2999. Pessoa sem ficha de jogador (+0x61): nada.
/// Clube humano (0x7875e0): a notícia do pedido (0xbbf), montada na pilha. No clube do computador, fora do status de
/// elenco 1 (+0x4f & 0xf0 = 0x10): se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), o clube o põe na lista
/// (+0x4e: tira o bit 2 e liga o 8; moral +12 em 0x544710; as listas dos clubes 0x842c00; 0x4d71b0). Senão um sorteio
/// pela força do pedido (+0x44) e pelo ânimo (+0x59) contra outro de 50 dá a queixa 0x10000 (0x867700); a moral cai 12
/// e a força do pedido sobe ânimo × 5 + 56, presa em ±100.
unsafe extern "thiscall" fn pedido_de_transferencia(this: usize, contrato: usize) {
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2999);
    }
    let clube = cps1_d(0xae23b4).wrapping_add(cps1_d(contrato + 4).wrapping_mul(0x245));
    let pessoa = cps1_d(0xae23bc).wrapping_add(cps1_d(contrato).wrapping_mul(0x6e));
    if cps1_d(pessoa + 0x61) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(cps1_d(0xb7431c), clube) != 0 {
        // ponytail: o clube humano recebe a notícia montada na pilha (0x77bbe0 e a entrega estão desligadas): esse
        // caso fica com a original (que refaz a pergunta 0x7875e0, sem efeito).
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, contrato);
    }
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    if cps1_b(contrato + 0x4f) & 0xf0 != 0x10 {
        let preco: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
        let a = preco(this, clube, 0, pessoa, 0, 1, 1);
        let v = valor(this, pessoa, clube, 0, -1, 0);
        // o 1,5 é o double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: o valor passa do preço
            crate::comparacao::ramo("cps_1 listado pelo clube");
            let m = (contrato + 0x4e) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            moral(pessoa, 0xc, 0);
            let listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
            listas(cps1_d(cps1_d(0xde1f54) + 0x856), pessoa, 1);
            let listar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d71b0usize);
            listar(this, pessoa, clube);
            return;
        }
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let r = sorteio(cps1_b(contrato + 0x44) as i8 as i32 + (cps1_b(pessoa + 0x59) as i8 as i32 + 5) * 20);
    if r < sorteio(0x32) { // cps1: o sorteio da queixa
        crate::comparacao::ramo("cps_1 queixa do pedido");
        let queixa: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x867700usize);
        queixa(contrato, 0x10000, 0);
    }
    crate::comparacao::ramo("cps_1 pedido no clube do computador");
    moral(pessoa, 0xffff_fff4, 0); // cps1: a moral cai 12
    // relidos depois das chamadas, como na original (o `imul` de 8 bits fica com o byte baixo)
    let al = cps1_b(pessoa + 0x59).wrapping_mul(5).wrapping_add(0x38);
    let cl = cps1_b(contrato + 0x44);
    let s = cl as i8 as i32 + al as i8 as i32;
    let m = (contrato + 0x44) as *mut u8;
    *m = if s > 100 {
        100
    } else if s < -100 {
        0x9c
    } else {
        cl.wrapping_add(al)
    };
}

/// { 0x4cbb20 } (stdcall, ret 4, contract_manager.cpp): `is_contract_staff_job(cargo)`: 1 para os cargos (byte) 0, 6 a
/// 0xd e 0xf, os que têm contrato pelo gerente de contratos; senão 0.
unsafe extern "stdcall" fn cargo_com_contrato(cargo: u32) -> u32 {
    matches!(cargo as u8, 0 | 6..=0xd | 0xf) as u32
}

/// { 0x4cb960 } (stdcall, ret 4, contract_manager.cpp): `staff_is_not_board_member(pessoa)`: 0 se o cargo (+0x3d) é de
/// diretoria (1 a 4) num clube (+0x39); senão 1.
unsafe extern "stdcall" fn fora_da_diretoria(pessoa: usize) -> u32 {
    let cargo = cps1_b(pessoa + 0x3d);
    if (1..=4).contains(&cargo) && cps1_d(pessoa + 0x39) != 0 {
        return 0;
    }
    1
}

// ponytail: sorteia (0x90cfc0), põe na lista (0x842c00, 0x4d71b0), muda moral e queixas (que podem chamar o pedido de
// novo): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_pedido_de_transferencia(this: usize, contrato: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, contrato)
}

/// Comparador: pelo valor.
unsafe extern "stdcall" fn comparar_cargo_com_contrato(cargo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(u32) -> u32>(%%I1%%)(cargo);
    let n = cargo_com_contrato(cargo);
    PLACAR.registrar(r == n, || format!("cargo_com_contrato({}) original={} rust={}", cargo as u8, r, n));
    r
}

/// Comparador: pelo valor.
unsafe extern "stdcall" fn comparar_fora_da_diretoria(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(%%I2%%)(pessoa);
    let n = fora_da_diretoria(pessoa);
    PLACAR.registrar(r == n, || format!("fora_da_diretoria(pessoa {}, cargo {}) original={} rust={}", cps1_d(pessoa), cps1_b(pessoa + 0x3d), r, n));
    r
}
