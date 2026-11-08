// ---------- %%LEVA%%a leva: o contrato da comissão: montar a proposta pelo contrato atual, zerar a moral do contrato novo e a faixa do vencimento (staff_contracts.cpp) ----------

unsafe fn cpl5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl5_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpl5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpl5_por_b(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// A parte comum da 0x866ee0 e da 0x85e590 (inline nesta): zera a moral (+0x44), o estado +0x4e e as queixas (+0x45),
/// limpa os vínculos de opinião da pessoa do contrato (0x549c90 com o registro [0xae23bc] + id × 0x6e), zera +0x40,
/// põe 4 em +0x4e, zera a felicidade (+0x3f) e +0x3e; com o clube e a nação dele (+0x53), o nível do idioma da nação
/// (+0x77) para a pessoa (0x7c6830 em [0xb74350], sem o contrato; no máximo 10) nos 4 bits baixos de +0x4f.
unsafe fn cpl5_zerar_moral(this: usize, pessoa: usize, clube: usize) {
    crate::comparacao::ramo("cpl_5 moral zerada");
    cpl5_por_b(this + 0x44, 0);
    cpl5_por_b(this + 0x4e, 0);
    cpl5_por(this + 0x45, 0);
    let limpar: unsafe extern "C" fn(usize) = core::mem::transmute(0x549c90usize);
    limpar(cpl5_d(0xae23bc).wrapping_add(cpl5_d(this).wrapping_mul(0x6e)));
    cpl5_por(this + 0x40, 0);
    cpl5_por_b(this + 0x4e, 4);
    cpl5_por_b(this + 0x3f, 0); // a felicidade
    cpl5_por_b(this + 0x3e, 0);
    if clube != 0 && cpl5_d(clube + 0x53) != 0 {
        let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x7c6830usize);
        let r = fluencia(cpl5_d(0xb74350), pessoa, clube, cpl5_b(cpl5_d(clube + 0x53) + 0x77) as u32, 0) as i8;
        let c = if r > 10 { 10 } else { r } as u8;
        let b = cpl5_b(this + 0x4f);
        cpl5_por_b(this + 0x4f, (b & 0xf0) | (c & 0xf));
    }
}

/// { 0x866ee0 } (thiscall, ret 8, staff_contracts.cpp): `STAFF_CONTRACT::reset_morale_state(pessoa, clube)`: o
/// `cpl5_zerar_moral`. Pela assinatura do contrato (CONTRACT_MANAGER::sign_contract 0x4cf720, nos termos que
/// depois são copiados para o contrato guardado).
unsafe extern "thiscall" fn reset_morale_state(this: usize, pessoa: usize, clube: usize) {
    cpl5_zerar_moral(this, pessoa, clube);
}

/// { 0x85e590 } (thiscall, ret 0x18, staff_contracts.cpp): `STAFF_CONTRACT::init_from_current_contract(pessoa, clube,
/// fim, percentual, sem_acesso, multa)`: a proposta de transferência ou empréstimo pelo contrato atual da pessoa
/// (0x4cf5a0 em [0xae1050]; sem contrato, caixa 0xf5 e nada): id da pessoa +0, clube +4, 0 em +8, salário +0xc =
/// salário atual × percentual (com sinal) / 100, a data de hoje (0xae2c90) em +0x25, o fim em +0x2d, os bits
/// +0x35 (& 0xc6 | 6), o salário também em +0x36, 0 em +0x3a e +0x49, −1 em +0x4a, o `cpl5_zerar_moral`, 0 em
/// +0x3b (16 bits), +0x1c..=+0x20 e +0x21, −1 em +0x10/+0x14/+0x18 e as cláusulas: +0x1c sem acesso, +0x1d multa
/// mínima. As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn init_from_current_contract(this: usize, pessoa: usize, clube: usize, fim: usize, percentual: u32, sem_acesso: u32, multa: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpl5_d(0xae1050), pessoa);
    if c == 0 {
        aviso_em(0xa8360c, 0xf5);
        return;
    }
    crate::comparacao::ramo("cpl_5 contrato pelo atual");
    cpl5_por(this, cpl5_d(pessoa));
    cpl5_por(this + 8, 0);
    cpl5_por(this + 4, cpl5_d(clube));
    let salario = (cpl5_d(c + 0xc) as i32).wrapping_mul(percentual as u8 as i8 as i32) / 100; // o percentual do salário
    cpl5_por(this + 0xc, salario as usize);
    core::ptr::write_unaligned((this + 0x25) as *mut [u8; 8], core::ptr::read_unaligned(0xae2c90 as *const [u8; 8])); // hoje
    core::ptr::write_unaligned((this + 0x2d) as *mut [u8; 8], core::ptr::read_unaligned(fim as *const [u8; 8])); // o fim do contrato
    let b = (cpl5_b(this + 0x35) & 0xc6) | 6;
    cpl5_por(this + 0x36, cpl5_d(this + 0xc));
    cpl5_por_b(this + 0x35, b);
    cpl5_por_b(this + 0x3a, 0);
    cpl5_por_b(this + 0x49, 0);
    cpl5_por(this + 0x4a, usize::MAX);
    cpl5_zerar_moral(this, pessoa, clube);
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    for k in 0x1c..=0x20 {
        cpl5_por_b(this + k, 0);
    }
    cpl5_por(this + 0x10, usize::MAX);
    cpl5_por(this + 0x14, usize::MAX);
    cpl5_por(this + 0x18, usize::MAX);
    cpl5_por(this + 0x21, 0);
    cpl5_por_b(this + 0x1c, sem_acesso as u8);
    cpl5_por_b(this + 0x1d, multa as u8);
}

/// { 0x867d30 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::get_expiry_band()`: a faixa 0..=14 do fim do contrato
/// (+0x2d) contra as datas de hoje (0xae2c90) e as 16 datas do CONTRACT_MANAGER (0xae1010..0xae1090, montadas na
/// criação): tipo (+0x35 & 0x3f) 1 ou fim já passado, 14; tipo diferente de 2, 0; depois de [0xae1090], 0; os
/// intervalos entre 0xae1040/0xae1060/0xae1010/0xae1058/0xae1030 dão os pares 1-2, 3-4, 5-6, 7-8, 9-10 (o ímpar no dia
/// exato do limite de cima: 0x4a7f20) e, antes de 0xae1030, 11 no dia, 13 no dia de 0xae1048, senão 12. "Depois"
/// pela 0x456740. Chamada a cada contrato no passe diário (0x4c7ac0): sem contador.
unsafe extern "thiscall" fn get_expiry_band(this: usize) -> u8 {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let mesmo_dia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let fim = this + 0x2d;
    let m = |g: usize| depois(fim, cpl5_d(g) as u32, cpl5_d(g + 4) as u32) != 0;
    let q = |g: usize| mesmo_dia(fim, cpl5_d(g) as u32, cpl5_d(g + 4) as u32) != 0;
    if cpl5_b(this + 0x35) & 0x3f == 1 {
        return 14;
    }
    if !m(0xae2c90) {
        return 14;
    }
    if cpl5_b(this + 0x35) & 0x3f != 2 {
        return 0;
    }
    if m(0xae1090) {
        return 0;
    }
    if m(0xae1010) { // depois do limite do meio
        if m(0xae1060) {
            if m(0xae1040) {
                return if q(0xae1090) { 1 } else { 2 };
            }
            return if q(0xae1040) { 3 } else { 4 };
        }
        return if q(0xae1060) { 5 } else { 6 };
    }
    if m(0xae1030) {
        if m(0xae1058) {
            return if q(0xae1010) { 7 } else { 8 };
        }
        return if q(0xae1058) { 9 } else { 10 };
    }
    if q(0xae1030) {
        return 11;
    }
    if q(0xae1048) { 13 } else { 12 }
}

/// As regiões que as duas primeiras gravam: o contrato (0x50 bytes), o registro de 0x4f bytes da pessoa de id `id`
/// em [0xae2d0c] (os vínculos da 0x549c90, se o id está na tabela) e [0xb67a34] (a caixa).
unsafe fn cpl5_regioes(this: usize, id: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(this, 0x50), (0xb67a34, 4)];
    let id = cpl5_d(cpl5_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))) as i32;
    if id >= 0 && id < cpl5_d(0xae2364) as i32 {
        r.push((cpl5_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)), 0x4f));
    }
    r
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa (o idioma 0x7c6830 só lê).
unsafe extern "thiscall" fn comparar_reset_morale_state(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let regioes = cpl5_regioes(this, cpl5_d(this));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa, clube), || reset_morale_state(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("reset_morale_state(pessoa {}) {:x?} x {:x?}", cpl5_d(pessoa), mo, mr));
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa; sem contrato atual (a caixa), só a original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_init_from_current_contract(this: usize, pessoa: usize, clube: usize, fim: usize, percentual: u32, sem_acesso: u32, multa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(%%I1%%);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    if contract_of(cpl5_d(0xae1050), pessoa) == 0 {
        return o(this, pessoa, clube, fim, percentual, sem_acesso, multa);
    }
    let regioes = cpl5_regioes(this, cpl5_d(pessoa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, fim, percentual, sem_acesso, multa),
        || init_from_current_contract(this, pessoa, clube, fim, percentual, sem_acesso, multa));
    PLACAR.registrar(igual, || {
        format!("init_from_current_contract(pessoa {}, percentual {}, {}, {}) {:x?} x {:x?}", cpl5_d(pessoa), percentual as u8 as i8, sem_acesso as u8, multa as u8, mo, mr)
    });
}

/// Comparador: só lê; compara a faixa.
unsafe extern "thiscall" fn comparar_get_expiry_band(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(%%I2%%)(this);
    let n = get_expiry_band(this);
    PLACAR.registrar(r == n, || format!("get_expiry_band(pessoa {}) original={} rust={}", cpl5_d(this), r, n));
    r
}
