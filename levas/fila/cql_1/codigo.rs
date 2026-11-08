// ---------- %%LEVA%%a leva: os números de camisa: o número de um jogador no clube, se o clube usa números e gravar o número (squad_manager.cpp) ----------

unsafe fn cql1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// O contrato `k` (não nulo) vale no clube `c`: o clube do contrato (+4, id) é o próprio `c` ([c] = id), ou é o clube
/// que a 0x540a50(c, 0, 1) devolve (cdecl de 3, conferida: push 1; push 0; push clube; add esp, 0xc) [inferência: o
/// clube principal de um time B]. O id do contrato é relido depois da chamada, como na original.
unsafe fn contract_valid_for_club(k: usize, c: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    if cql1_d(k + 4) == cql1_d(c) {
        return true;
    }
    let p = parceiro(c, 0, 1);
    let id = cql1_d(k + 4) as usize;
    (cql1_d(0xae23b4) as usize).wrapping_add(id.wrapping_mul(0x245)) == p
}

/// { 0x85a760 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::get_squad_number`; o this não é lido): o número
/// de camisa (char, negativo nos clubes sem técnico humano) da pessoa `p` no clube `c`. Sem um dos dois, 0. Pessoa fora
/// da tabela das pessoas com contrato (id ≥ [0xae2364]): o byte +4 da ficha de jogador (+0x61). Senão o contrato
/// (0x4cf5a0, thiscall em [0xae1050], push pessoa), se a pessoa é jogador (0x53b620 com o +0x3d, cdecl de 1) e o
/// contrato vale no clube: o +0x3a do contrato; senão o contrato de empréstimo (0x4cf6d0, thiscall em [0xae1050], push
/// pessoa), se vale no clube: o +0x3a dele; senão 0.
unsafe extern "thiscall" fn get_squad_number(_this: usize, p: usize, c: usize) -> i8 {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let jogador: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    if p == 0 || c == 0 {
        return 0;
    }
    if cql1_d(p) as i32 >= cql1_d(0xae2364) as i32 {
        return *((cql1_d(p + 0x61) as usize + 4) as *const i8);
    }
    let k = contrato(cql1_d(0xae1050) as usize, p);
    if k != 0 && jogador(*((p + 0x3d) as *const i8)) != 0 && contract_valid_for_club(k, c) {
        crate::comparison::branch("cql_1 numero do contrato");
        return *((k + 0x3a) as *const i8); // cql1: o número do contrato
    }
    let k = emprestimo(cql1_d(0xae1050) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        return *((k + 0x3a) as *const i8);
    }
    0
}

/// A devolução das duas (a rotina só lê; as chamadas de dentro são buscas).
unsafe extern "thiscall" fn comparar_get_squad_number(this: usize, p: usize, c: usize) -> i8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i8>(%%I0%%);
    let r = o(this, p, c);
    let n = get_squad_number(this, p, c);
    PLACAR.record(r == n, || format!("get_squad_number(pessoa {:x}, clube {:x}): original {} rust {}", p, c, r, n));
    r
}

/// { 0x85add0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::club_uses_squad_numbers`; o this não é lido):
/// 1 se o clube usa números de camisa: a competição da divisão do clube (o id em [[c+0x57]], o objeto na tabela
/// [0xadadfc]) existe e tem o bit 2 do +0x52 e, com `outra` (Comp*), a outra competição (a da tabela pelo id [outra])
/// também tem o bit. Sem divisão ou sem a competição, 0; sem clube, a caixa da linha 0x4b0 e 0.
unsafe extern "thiscall" fn club_uses_squad_numbers(_this: usize, c: usize, outra: usize) -> u32 {
    if c == 0 {
        show_error_at(0xa82b88, 0x4b0);
        return 0;
    }
    let div = cql1_d(c + 0x57) as usize;
    if div == 0 {
        return 0;
    }
    let tabela = cql1_d(0xadadfc) as usize;
    let comp = cql1_d(tabela + cql1_d(div) as usize * 4) as usize;
    if comp == 0 {
        return 0;
    }
    if *((comp + 0x52) as *const u8) & 2 == 0 {
        return 0;
    }
    if outra != 0 {
        let comp2 = cql1_d(tabela + cql1_d(outra) as usize * 4) as usize;
        if *((comp2 + 0x52) as *const u8) & 2 == 0 {
            return 0;
        }
    }
    crate::comparison::branch("cql_1 usa numeros");
    1 // cql1: usa números
}

/// A devolução das duas (a rotina só lê); sem clube, só a original (a caixa não roda duas vezes).
unsafe extern "thiscall" fn comparar_club_uses_squad_numbers(this: usize, c: usize, outra: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    if c == 0 {
        return o(this, c, outra);
    }
    let r = o(this, c, outra);
    let n = club_uses_squad_numbers(this, c, outra);
    PLACAR.record(r == n, || format!("club_uses_squad_numbers(clube {:x}, outra {:x}): original {} rust {}", c, outra, r, n));
    r
}

/// { 0x85a850 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::set_squad_number`; o this não é lido): grava o
/// número `n` (char de −50 a 50; fora disso a caixa da linha 0x332) no +0x3a do contrato da pessoa `p` (0x4cf5a0) se ele
/// vale no clube `c`, senão no do contrato de empréstimo (0x4cf6d0) se ele vale no clube. Sem pessoa ou sem clube, nada.
unsafe extern "thiscall" fn set_squad_number(_this: usize, p: usize, c: usize, n: u32) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if p == 0 || c == 0 {
        return;
    }
    let n = n as u8 as i8;
    if !(-50..=50).contains(&n) {
        show_error_at(0xa82b88, 0x332);
        return;
    }
    let k = contrato(cql1_d(0xae1050) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        crate::comparison::branch("cql_1 grava no contrato");
        *((k + 0x3a) as *mut i8) = n; // cql1: grava no contrato
        return;
    }
    let k = emprestimo(cql1_d(0xae1050) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        *((k + 0x3a) as *mut i8) = n;
    }
}

/// Lado a lado: o +0x3a dos dois contratos da pessoa (achados antes; as buscas só leem). Número fora da faixa (a caixa),
/// sem pessoa ou sem clube: só a original.
unsafe extern "thiscall" fn comparar_set_squad_number(this: usize, p: usize, c: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I2%%);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if p == 0 || c == 0 || !(-50..=50).contains(&(n as u8 as i8)) {
        return o(this, p, c, n);
    }
    let mut reg = Vec::new();
    for k in [contrato(cql1_d(0xae1050) as usize, p), emprestimo(cql1_d(0xae1050) as usize, p)] {
        if k != 0 {
            reg.push((k + 0x3a, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p, c, n), || set_squad_number(this, p, c, n));
    PLACAR.record(igual, || format!("set_squad_number(pessoa {:x}, clube {:x}, {}): original {:?} rust {:?}", p, c, n as u8 as i8, mo.0, mr.0));
}
