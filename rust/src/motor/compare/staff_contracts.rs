use super::*;

// The comparers of the routines in ../staff_contracts.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../staff_contracts.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `has_active_release_clause` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_has_active_release_clause(c: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_ACTIVE_RELEASE_CLAUSE)(c);
    let n = has_active_release_clause(c);
    PLACAR.registrar(r == n, || format!("has_active_release_clause({:x}) original={} rust={}", c, r, n));
    r
}

/// Comparer for `check_fifa_protected_period` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_check_fifa_protected_period(c: usize, x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    // as saídas como estavam antes: a cópia parte delas (a original só grava `anos` no período)
    let (mut nl, mut na) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) -> u32>(IDX_CHECK_FIFA_PROTECTED_PERIOD)(c, x, no_limite, anos);
    let (ol, oa) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let n = check_fifa_protected_period(
        c,
        x,
        if no_limite.is_null() { core::ptr::null_mut() } else { &mut nl },
        if anos.is_null() { core::ptr::null_mut() } else { &mut na },
    );
    let ok = r == n && (no_limite.is_null() || ol == nl) && (anos.is_null() || oa == na);
    PLACAR.registrar(ok, || format!("check_fifa_protected_period({:x}) original={} {} {} rust={} {} {}", c, r, ol, oa, n, nl, na));
    r
}

/// Comparer for `staff_contract_is_bonus_type_allowed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_staff_contract_is_bonus_type_allowed(pessoa: usize, tipo: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_STAFF_CONTRACT_IS_BONUS_TYPE_ALLOWED)(pessoa, tipo);
    let n = staff_contract_is_bonus_type_allowed(pessoa, tipo);
    PLACAR.registrar(r == n, || format!("staff_contract_is_bonus_type_allowed({:x}, {}) original={} rust={}", pessoa, tipo as u8, r, n));
    r
}

/// Comparer for `reset_morale_state` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no contrato e nos vínculos da pessoa (o idioma 0x7c6830 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_reset_morale_state(this: usize, pessoa: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RESET_MORALE_STATE);
    let regioes = morale_reset_regions(this, read_usize(this));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa, clube), || reset_morale_state(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("reset_morale_state(pessoa {}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
}

/// Comparer for `init_from_current_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no contrato e nos vínculos da pessoa; sem contrato atual (a caixa), só a original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_init_from_current_contract(this: usize, pessoa: usize, clube: usize, fim: usize, write_percent_pair: u32, sem_acesso: u32, multa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_INIT_FROM_CURRENT_CONTRACT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    if contract_of(read_usize(G_CONTRACT_MANAGER), pessoa) == 0 {
        return o(this, pessoa, clube, fim, write_percent_pair, sem_acesso, multa);
    }
    let regioes = morale_reset_regions(this, read_usize(pessoa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, fim, write_percent_pair, sem_acesso, multa),
        || init_from_current_contract(this, pessoa, clube, fim, write_percent_pair, sem_acesso, multa));
    PLACAR.registrar(igual, || {
        format!("init_from_current_contract(pessoa {}, write_percent_pair {}, {}, {}) {:x?} x {:x?}", read_usize(pessoa), write_percent_pair as u8 as i8, sem_acesso as u8, multa as u8, mo, mr)
    });
}

/// Comparer for `get_expiry_band` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só lê; compara a faixa.
pub(crate) unsafe extern "thiscall" fn comparar_get_expiry_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_EXPIRY_BAND)(this);
    let n = get_expiry_band(this);
    PLACAR.registrar(r == n, || format!("get_expiry_band(pessoa {}) original={} rust={}", read_usize(this), r, n));
    r
}

/// Comparer for `clear_all_grievances` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem `conferir`, lado a lado pelas queixas e pelo registro de opiniões da pessoa (79 bytes, o que a
/// 0x549c90 escreve); com `conferir`, a retirada (0x4d9460) mexe na transferência: só a original (a 0x867240 passa
/// pelo comparador dela).
pub(crate) unsafe extern "thiscall" fn comparar_clear_all_grievances(this: usize, conferir: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CLEAR_ALL_GRIEVANCES);
    if conferir != 0 {
        return o(this, conferir);
    }
    let mut reg = vec![(this + 0x45, 4)];
    if let Some(c) = database_staff_state_record(staff_of_contract(this)) {
        reg.push((c, 79));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, conferir), || clear_all_grievances(this, conferir));
    PLACAR.registrar(igual, || format!("clear_all_grievances({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparer for `check_transfer_request_withdrawal` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: quando a decisão da cópia é não retirar, lado a lado pela ficha [0xae1080] e pelo contrato (0x50
/// bytes); quando retira, só a original (a 0x4d9460 mexe na transferência e a 0x867240 se chama de novo).
pub(crate) unsafe extern "thiscall" fn comparar_check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
    let ficha = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let retira = should_withdraw_transfer_request(this).is_some();
    *(G_CONTRACT_LOOKUP_RESULT as *mut u32) = ficha;
    if retira {
        return o(this, ctx);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4), (this, 0x50)], || o(this, ctx), || check_transfer_request_withdrawal(this, ctx));
    PLACAR.registrar(igual, || format!("check_transfer_request_withdrawal({:x}, {}) {:x?} x {:x?}", this, ctx, mo, mr));
}

/// Comparer for `set_flags` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador das marcas do contrato, em ensaio: a cópia sem o pedido de transferência e a original a partir do mesmo
/// estado (marcas e sorteio); sem pedido, tudo tem de bater; com pedido (que sorteia e dá notícia), só as marcas.
pub(crate) unsafe extern "thiscall" fn comparar_set_flags(c: usize, marcas: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SET_FLAGS);
    if marcas as i32 > 0x1000 {
        return o(c, marcas);
    }
    let mut regioes = vec![(c + 0x40, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    MISS_TRAINING_DRY_RUN = true;
    MISS_TRAINING_DECISION = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| set_flags(c, marcas)));
    MISS_TRAINING_DRY_RUN = false;
    let pediu = MISS_TRAINING_DECISION != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(c, marcas);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if pediu { mo.0[0] == mr.0[0] } else { mo == mr };
    PLACAR.registrar(igual, || format!("set_flags({:#x}, {:#x}) pediu={} original={:x?} rust={:x?}", c, marcas, pediu, mo, mr));
}

/// Comparer for `get_contract_satisfaction` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia a partir do mesmo estado (o sorteio, o último contrato pedido [0xae1080] que a
/// 0x85fc50 grava e o contrato); a devolução e a memória.
pub(crate) unsafe extern "thiscall" fn comparar_get_contract_satisfaction(this: usize, pessoa: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_CONTRACT_SATISFACTION);
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), (this, 0x50)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, pessoa),
        || crate::aleatorio::sem_contar(|| get_contract_satisfaction(this, pessoa)),
    );
    PLACAR.registrar(igual && r == n, || {
        format!("get_contract_satisfaction(pessoa {:#x}) original={} {:x?} rust={} {:x?}", if pessoa != 0 { read_usize(pessoa) } else { 0 }, r, mo, n, mr)
    });
    r
}

/// Comparer for `set_default_wage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo salário (+0xc). A 0x862720 grava a mesma ficha em [0xae1080] nas duas vezes.
pub(crate) unsafe extern "thiscall" fn comparar_set_default_wage(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_DEFAULT_WAGE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 4)], || o(this, pessoa), || set_default_wage(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("set_default_wage({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `construct_staff_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 0x4f bytes do contrato e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_construct_staff_contract(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_STAFF_CONTRACT);
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x4f)], || o(this), || construct_staff_contract(this));
    PLACAR.registrar(igual && ro == rr, || format!("construct_staff_contract({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}
