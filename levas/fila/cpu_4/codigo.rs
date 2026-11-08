// ---------- %%LEVA%%a leva: as cláusulas do contrato: a cláusula de rescisão ativa, o período protegido da FIFA e o tipo de bônus permitido (staff_contracts.cpp) ----------

unsafe fn cpu4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x867b50 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::has_active_release_clause()`: 1 com a cláusula +0x1c
/// = 2, a +0x1d = 1 ou a +0x1f = 2; senão 0. Pelo dia dos contratos 0x4c7ac0, pelo valor do jogador 0x4d0c60, pelo preço
/// pedido 0x4d2070, pela resposta prevista do vendedor 0x84af00, pelo vendedor que tem de aceitar 0x8cd240 e pelo lance
/// 0x8e5d60. TapeCall a cada contrato: sem contador.
unsafe extern "thiscall" fn has_active_release_clause(c: usize) -> u32 {
    if cpu4_b(c + 0x1c) == 2 || cpu4_b(c + 0x1d) == 1 || cpu4_b(c + 0x1f) == 2 {
        return 1; // cpu4: com a clausula ativa
    }
    0
}

unsafe extern "thiscall" fn comparar_has_active_release_clause(c: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(c);
    let n = has_active_release_clause(c);
    SCOREBOARD.record(r == n, || format!("has_active_release_clause({:x}) original={} rust={}", c, r, n));
    r
}

/// { 0x868210 } (thiscall, ret 0xc, staff_contracts.cpp, quadro SEH só da data local):
/// `STAFF_CONTRACT::check_fifa_protected_period(_, &no_limite, &anos)`: zera `no_limite` (se não nulo). Sem as regras
/// novas da FIFA ([0xae233c] ≠ 1), 1. Contrato (data +0x25) anterior à data das regras ([0xae2330]: ano sem sinal, dia
/// com sinal), 1. Senão o período protegido: 3 anos com a idade da pessoa ([0xae23bc] + 110 × [+0]) na data do contrato
/// (0x53ab90) abaixo de 28, senão 2, em `anos` (se não nulo); a duração de hoje até a data (0x54cc30, duas vezes, como a
/// original) com menos anos que o período, 1; com os anos exatos e 0 dias, `no_limite` = 1; 0. Pelo dia dos contratos
/// 0x4c7ac0, pelo lance 0x8e5d60, pela troca com compensação 0x84b6f0 e por telas. Sem contador.
unsafe extern "thiscall" fn check_fifa_protected_period(c: usize, _x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    if !no_limite.is_null() {
        *no_limite = 0;
    }
    if cpu4_b(0xae233c) != 1 {
        return 1; // cpu4: sem as regras novas
    }
    let (rdia, rano) = (*(0xae2330 as *const i16), *(0xae2332 as *const u16));
    let d = c + 0x25;
    let (cdia, cano) = (core::ptr::read_unaligned(d as *const i16), core::ptr::read_unaligned((d + 2) as *const u16));
    if rano > cano || (rano == cano && rdia > cdia) {
        return 1;
    }
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let idade: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53ab90usize);
    let (d0, d1) = (cpu4_d(d) as u32, cpu4_d(d + 4) as u32);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    let pessoa = cpu4_d(0xae23bc).wrapping_add(cpu4_d(c).wrapping_mul(110));
    let limit: u8 = if (idade(pessoa, d) as u8 as i8) < 0x1c { 3 } else { 2 }; // cpu4: o periodo pela idade
    if !anos.is_null() {
        *anos = limit;
    }
    let mut l = [0u8; 8];
    let p = diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    core::ptr::copy_nonoverlapping(p as *const u8, l.as_mut_ptr(), 8);
    let a = u16::from_le_bytes([l[2], l[3]]) as i32;
    if a < limit as i32 {
        return 1; // cpu4: dentro do periodo
    }
    if !no_limite.is_null() && a == limit as i32 && u16::from_le_bytes([l[0], l[1]]) == 0 {
        *no_limite = 1;
    }
    0
}

unsafe extern "thiscall" fn comparar_check_fifa_protected_period(c: usize, x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    // as saídas como estavam antes: a cópia parte delas (a original só grava `anos` no período)
    let (mut nl, mut na) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) -> u32>(%%I1%%)(c, x, no_limite, anos);
    let (ol, oa) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let n = check_fifa_protected_period(
        c,
        x,
        if no_limite.is_null() { core::ptr::null_mut() } else { &mut nl },
        if anos.is_null() { core::ptr::null_mut() } else { &mut na },
    );
    let ok = r == n && (no_limite.is_null() || ol == nl) && (anos.is_null() || oa == na);
    SCOREBOARD.record(ok, || format!("check_fifa_protected_period({:x}) original={} {} {} rust={} {} {}", c, r, ol, oa, n, nl, na));
    r
}

/// { 0x867aa0 } (cdecl, staff_contracts.cpp): `staff_contract_is_bonus_type_allowed(pessoa, tipo)`: sem a ficha de
/// jogador (+0x61), 0; goleiro (+0xf da ficha = 0x14), 1 só para o tipo 2; senão 1 para os tipos que não são 2. Pela
/// tela do contrato.
unsafe extern "C" fn staff_contract_is_bonus_type_allowed(pessoa: usize, tipo: u32) -> u32 {
    let f = cpu4_d(pessoa + 0x61);
    if f == 0 {
        return 0;
    }
    if cpu4_b(f + 0xf) == 0x14 {
        return (tipo as u8 == 2) as u32;
    }
    (tipo as u8 != 2) as u32
}

unsafe extern "C" fn comparar_staff_contract_is_bonus_type_allowed(pessoa: usize, tipo: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(%%I2%%)(pessoa, tipo);
    let n = staff_contract_is_bonus_type_allowed(pessoa, tipo);
    SCOREBOARD.record(r == n, || format!("staff_contract_is_bonus_type_allowed({:x}, {}) original={} rust={}", pessoa, tipo as u8, r, n));
    r
}
