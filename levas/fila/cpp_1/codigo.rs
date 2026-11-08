// ---------- %%LEVA%%a leva: as datas de decisão e as ofertas do clube: a ordem das datas, decidir as datas vencidas e as ofertas de contrato do clube (contract_manager.cpp) ----------

unsafe fn cpp1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// As comparações de data do jogo (thiscall na data `a`, a outra por valor): 0x456740 `>`, 0x456770 `<`, 0x4a7f20 `==`.
unsafe fn cpp1_data(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, cpp1_d(b), cpp1_d(b + 4))
}

/// { 0x4dc910 } (cdecl, contract_manager.cpp): `compare_decision_dates(a, b)`: a ordem do qsort das datas de decisão
/// (entradas de 14 bytes: +0 pessoa, +4 data, +0xc fatia do dia, +0xd tipo): a data de `a` antes (0x456770): −1;
/// depois (0x456740): 1; no mesmo dia, a fatia de `a` menos a de `b` (com sinal). Quente (o qsort): sem contador.
unsafe extern "C" fn compare_decision_dates(a: usize, b: usize) -> i32 {
    if cpp1_data(0x456770, a + 4, b + 4) != 0 {
        return -1; // a antes de b
    }
    if cpp1_data(0x456740, a + 4, b + 4) != 0 {
        return 1;
    }
    cpp1_b(a + 0xc) as i8 as i32 - cpp1_b(b + 0xc) as i8 as i32
}

unsafe extern "C" fn comparar_compare_decision_dates(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%);
    let r = o(a, b);
    let n = compare_decision_dates(a, b);
    PLACAR.registrar(r == n, || format!("compare_decision_dates original={} rust={}", r, n));
    r
}

/// { 0x4cd9c0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::process_due_decision_dates(data, fatia)`:
/// do índice +0x18 até a conta +0x14 do vetor das datas de decisão (+0xc, entradas de 14 bytes, ordenadas pela
/// 0x4dc910): para na primeira data depois de `data` (0x456740) ou, no mesmo dia (0x4a7f20), com a fatia da entrada
/// maior que `fatia`. Cada entrada vencida com pessoa (+0 ≠ −1) fica com −1 e, sendo do tipo 3 (+0xd), a pessoa decide
/// as ofertas de contrato (0x4ce4e0). O índice +0x18 sobe a cada entrada (relido depois da decisão).
unsafe extern "thiscall" fn process_due_decision_dates(this: usize, data: usize, fatia: u32) {
    let n = |c: usize| cpp1_d(c) as i32;
    if n(this + 0x18) >= n(this + 0x14) {
        return;
    }
    let resolver: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ce4e0usize);
    loop {
        let e = (cpp1_d(this + 0xc) as usize).wrapping_add((n(this + 0x18) as usize).wrapping_mul(14));
        if cpp1_data(0x456740, e + 4, data) != 0 {
            return; // ainda não venceu
        }
        if cpp1_data(0x4a7f20, e + 4, data) != 0 && cpp1_b(e + 0xc) as i8 > fatia as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let pessoa = (cpp1_d(0xae23bc) as usize).wrapping_add((id as usize).wrapping_mul(0x6e));
            core::ptr::write_unaligned(e as *mut i32, -1);
            if cpp1_b(e + 0xd) == 3 {
                crate::comparacao::ramo("cpp_1 decidir ofertas");
                resolver(this, pessoa); // a pessoa decide as ofertas
            }
        }
        let i = n(this + 0x18) + 1;
        core::ptr::write_unaligned((this + 0x18) as *mut i32, i);
        if i >= n(this + 0x14) {
            return;
        }
    }
}

/// Comparador: a decisão das ofertas (0x4ce4e0) mexe nos contratos, nas ofertas e no próprio vetor: o plano (as datas
/// lidas antes, sem rodar nada) diz se alguma entrada do tipo 3 com pessoa vence; com ela, só a original. Sem ela (o
/// caso de todo dia), lado a lado pelo índice +0x18 e pelas entradas que vencem (ficam com −1).
unsafe extern "thiscall" fn comparar_process_due_decision_dates(this: usize, data: usize, fatia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%);
    let (i0, n) = (cpp1_d(this + 0x18) as i32, cpp1_d(this + 0x14) as i32);
    let mut i = i0;
    let mut decide = false;
    while i < n {
        let e = (cpp1_d(this + 0xc) as usize).wrapping_add((i as usize).wrapping_mul(14));
        if cpp1_data(0x456740, e + 4, data) != 0
            || (cpp1_data(0x4a7f20, e + 4, data) != 0 && cpp1_b(e + 0xc) as i8 > fatia as u8 as i8) {
            break;
        }
        decide |= cpp1_d(e) as i32 != -1 && cpp1_b(e + 0xd) == 3;
        i += 1;
    }
    if decide {
        return o(this, data, fatia);
    }
    let mut reg = vec![(this + 0x18, 4)];
    if i > i0 {
        reg.push(((cpp1_d(this + 0xc) as usize).wrapping_add((i0 as usize).wrapping_mul(14)), ((i - i0) as usize) * 14));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data, fatia), || process_due_decision_dates(this, data, fatia));
    PLACAR.registrar(igual, || format!("process_due_decision_dates(fatia {}) {:x?} x {:x?}", fatia as u8 as i8, mo, mr));
}

/// { 0x4ca300 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_current_contract_offers(clube, saida)`:
/// percorre a lista das ofertas (+0x34; cursor reposto, 0x68d1b0) e copia cada oferta do clube (+4 = id) fora dos
/// estados 5 e 6 (+0x35 & 0x3f) para um vetor novo de 111 bytes por oferta, que cresce de 20 em 20 (realloc 0x945667
/// com a capacidade em byte com sinal). Grava o vetor em `saida` e devolve quantas (byte). A original confere a lista
/// (+0x34), não o vetor, depois do realloc: só com a lista nula vai à caixa "cannot allocate memory" (linha 0x64c) e
/// devolve 0 com a saída nula. Quem a chama (0x598a06, o orçamento de transferências) libera o vetor.
unsafe extern "thiscall" fn get_current_contract_offers(this: usize, clube: usize, saida: usize) -> u8 {
    let mut n: i8 = 0;
    let mut cap: i8 = 0;
    let mut vetor: usize = 0;
    let lista = cpp1_d(this + 0x34) as usize;
    if lista != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpp1_d(lista + 2));
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let mut item = proximo(lista);
        while item != 0 {
            let estado = cpp1_b(item + 0x35) & 0x3f;
            if cpp1_d(item + 4) == cpp1_d(clube) && estado != 5 && estado != 6 {
                if cap == n {
                    cap = cap.wrapping_add(20);
                    vetor = realocar(vetor, (cap as i32 * 111) as u32);
                    if cpp1_d(this + 0x34) == 0 {
                        cpl4_sem_espaco(0x64c, 0x9a8220); // "cannot allocate memory"
                        core::ptr::write_unaligned(saida as *mut u32, 0);
                        return 0;
                    }
                }
                crate::comparacao::ramo("cpp_1 oferta do clube");
                core::ptr::copy(item as *const u8, vetor.wrapping_add((n as i32 * 111) as usize) as *mut u8, 111);
                n = n.wrapping_add(1);
            }
            item = proximo(cpp1_d(this + 0x34) as usize);
        }
    }
    core::ptr::write_unaligned(saida as *mut u32, vetor as u32);
    n as u8
}

/// Comparador: a original aloca o vetor que quem chama libera: roda só ela e confere o plano feito antes (as ofertas
/// do clube lidas da lista, sem alocar): a conta e os 111 bytes de cada uma, na ordem.
unsafe extern "thiscall" fn comparar_get_current_contract_offers(this: usize, clube: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I2%%);
    let lista = cpp1_d(this + 0x34) as usize;
    if lista == 0 || clube == 0 || saida == 0 {
        return o(this, clube, saida);
    }
    let mut plano: Vec<u8> = Vec::new();
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpp1_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut item = proximo(lista);
    while item != 0 {
        let estado = cpp1_b(item + 0x35) & 0x3f;
        if cpp1_d(item + 4) == cpp1_d(clube) && estado != 5 && estado != 6 {
            plano.extend_from_slice(core::slice::from_raw_parts(item as *const u8, 111));
        }
        item = proximo(lista);
    }
    let r = o(this, clube, saida);
    let vetor = cpp1_d(saida) as usize;
    let k = plano.len() / 111;
    let ok = r as usize == k && (k == 0 || core::slice::from_raw_parts(vetor as *const u8, k * 111) == &plano[..]);
    PLACAR.registrar(ok, || format!("get_current_contract_offers(clube {:x}) original {} ofertas, plano {}", clube, r, k));
    r
}
