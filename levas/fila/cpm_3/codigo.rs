// ---------- %%LEVA%%a leva: a mudança de divisão do clube e os embaralhamentos das listas de clubes (league.cpp, comp_util.cpp) ----------

unsafe fn cpm3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm3_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x6830b0 } (thiscall, ret 0xc, league.cpp; `this` não é lido): `League::move_club_to_division(clube, divisao,
/// vaga)`: a divisão atual (+0x57) vira a da última temporada (+0x5b) e a nova entra; as cláusulas de divisão dos
/// jogadores são limpas (0x4cd120 em [0xae1050]), com os números da camisa também se a divisão velha não os usava e a
/// nova usa (0x85add0 em [0xdd7ef0], antes e depois); com `vaga` e sem técnico (+0xcf), a vaga de técnico (0x69ba50 em
/// [0xb67a44]); numa divisão de liga (+0x43 = 2 na competição [0xadadfc][id]) com estádio mínimo (+0xe2 ≠ −1) e ficha
/// do estádio (+0x69), a obra do estádio (0x599e00 nas finanças 0x5a0590 de [0xae2a38], lugares +0xe4 e +0xe2); e,
/// com a competição carregada, a situação +0x64 = 3 passa a 2.
unsafe extern "thiscall" fn move_club_to_division(_this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("cpm_3 mudar de divisao");
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85add0usize);
    let limpar_clausulas: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4cd120usize);
    let antes = usa_numeros(cpm3_d(0xdd7ef0), clube, 0);
    cpm3_por(clube + 0x5b, cpm3_d(clube + 0x57)); // a divisão da última temporada
    cpm3_por(clube + 0x57, divisao);
    let numeros = (antes == 0 && usa_numeros(cpm3_d(0xdd7ef0), clube, 0) != 0) as u32;
    limpar_clausulas(cpm3_d(0xae1050), clube, numeros);
    if vaga != 0 && cpm3_d(clube + 0xcf) == 0 {
        let abrir_vaga: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x69ba50usize);
        abrir_vaga(cpm3_d(0xb67a44), clube, 0);
    }
    if divisao == 0 {
        return;
    }
    let comps = cpm3_d(0xadadfc);
    let id = cpm3_d(divisao);
    if comps != 0 {
        let comp = cpm3_d(comps + 4 * id);
        if comp != 0 && cpm3_d(clube + 0x69) != 0 && *((comp + 0x43) as *const u8) == 2 {
            let lugares = core::ptr::read_unaligned((comp + 0xe2) as *const i16);
            if lugares != -1 {
                crate::comparacao::ramo("cpm_3 estadio da divisao");
                let capacidade = core::ptr::read_unaligned((comp + 0xe4) as *const i16);
                let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
                let obra: unsafe extern "thiscall" fn(usize, i32, i32, usize, u32) -> u32 = core::mem::transmute(0x599e00usize);
                let f = financas(cpm3_d(0xae2a38), clube);
                obra(f, capacidade as i32, lugares as i32, id, 0); // a obra do estádio
            }
        }
    }
    // a original relê [0xadadfc] depois da obra (sem a obra, o mesmo valor; nulo, ela lê do endereço 0 também)
    if cpm3_d(cpm3_d(0xadadfc) + 4 * id) != 0 && *((clube + 0x64) as *const u8) == 3 {
        *((clube + 0x64) as *mut u8) = 2;
    }
}

unsafe fn cpm3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// As trocas de `vezes` pares sorteados (0x90cfc0(n) duas vezes; a primeira dá o índice que recebe o item do segundo)
/// no vetor de ponteiros `itens`, como as duas originais (índices de 16 bits com sinal).
unsafe fn swap_random_pair(itens: usize, n: i32) {
    let a = cpm3_sorteio(n) as i16 as isize;
    let b = cpm3_sorteio(n) as i16 as isize;
    let pa = (itens as isize + 4 * a) as usize;
    let pb = (itens as isize + 4 * b) as usize;
    let (va, vb) = (cpm3_d(pa), cpm3_d(pb));
    cpm3_por(pa, vb);
    cpm3_por(pb, va);
}

/// { 0x4afb40 } (cdecl, comp_util.cpp): `comp_util_shuffle_pointer_array(itens, n)`: embaralha o vetor de `n` (curto)
/// ponteiros com 3 × n trocas de pares sorteados (0x90cfc0(n) duas vezes cada).
unsafe extern "C" fn comp_util_shuffle_pointer_array(itens: usize, n: u32) {
    let n = n as u16 as i16 as i32;
    if 3 * n <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar ponteiros");
    let mut k = 0i16;
    loop {
        swap_random_pair(itens, n);
        k = k.wrapping_add(1); // mais uma troca
        if k as i32 >= 3 * n {
            break;
        }
    }
}

/// { 0x4afbd0 } (cdecl, comp_util.cpp): `comp_util_sort_and_shuffle_top_clubs(clubes, n, escolhas)`: ordena os `n`
/// clubes pela reputação, maior primeiro e nulos no fim (qsort 0x945333 com 0x4b07d0), e embaralha os m = min(3 ×
/// escolhas, n) primeiros com m trocas de pares sorteados entre eles. `n` negativo ou menor que `escolhas`: caixa
/// "Error" (a original); `escolhas` < 1: nada.
unsafe extern "C" fn comp_util_sort_and_shuffle_top_clubs(clubes: usize, n: u32, escolhas: u32) {
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I2%%)(clubes, n, escolhas); // a caixa
    }
    if e < 1 {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(clubes, nn as i32 as u32, 4, 0x4b07d0);
    let mut m = e.wrapping_mul(3);
    if m > nn {
        m = nn;
    }
    if m <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar os melhores");
    for _ in 0..m as i32 { // m trocas
        swap_random_pair(clubes, m as i32);
    }
}

/// Comparador: a mudança chama as notícias, a vaga de técnico e a obra (não podem rodar duas vezes): só a original (a
/// prova é a carreira e o modo Rust).
unsafe extern "thiscall" fn comparar_move_club_to_division(this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("move_club_to_division");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%)(this, clube, divisao, vaga)
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pelo vetor, a cópia em sem_contar.
unsafe extern "C" fn comparar_comp_util_shuffle_pointer_array(itens: usize, n: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(%%I1%%);
    let k = (n as u16 as i16).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if k > 0 {
        reg.push((itens, 4 * k));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(itens, n),
        || crate::aleatorio::sem_contar(|| comp_util_shuffle_pointer_array(itens, n)));
    PLACAR.registrar(igual, || format!("comp_util_shuffle_pointer_array({}) {:x?} x {:x?}", k, mo, mr));
}

/// Comparador: ordena (qsort, determinístico) e sorteia: lado a lado pelo estado do sorteio e pelos `n` clubes, a cópia
/// em sem_contar. Caixa (`n` negativo ou menor que `escolhas`): só a original.
unsafe extern "C" fn comparar_comp_util_sort_and_shuffle_top_clubs(clubes: usize, n: u32, escolhas: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I2%%);
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return o(clubes, n, escolhas);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if nn > 0 {
        reg.push((clubes, 4 * nn as usize));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clubes, n, escolhas),
        || crate::aleatorio::sem_contar(|| comp_util_sort_and_shuffle_top_clubs(clubes, n, escolhas)));
    PLACAR.registrar(igual, || format!("comp_util_sort_and_shuffle_top_clubs({}, {}) {:x?} x {:x?}", nn, e, mo, mr));
}
