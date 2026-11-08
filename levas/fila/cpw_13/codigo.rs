// ---------- %%LEVA%%a leva: os convocados por clube na data do jogo da seleção, a importância do jogo e a fase da competição (national_teams.cpp, comp.cpp) ----------

unsafe fn cpw13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O registro de convocações do clube `c` em [this + 0x20] (6 bytes: +0 vetor de entradas de 9 bytes, +4 capacidade,
/// +5 quantas), se o clube conta: id dentro dos [0xae235c] − 2 × [0xae2350] registros, com nação (+0x53) marcada
/// (+0x11c) e que não é a de [0x9cf260].
unsafe fn cpw13_registro(this: usize, c: usize) -> Option<usize> {
    let n = (cpw13_d(0xae235c) as i32).wrapping_sub((cpw13_d(0xae2350) as i32).wrapping_mul(2));
    let id = cpw13_d(c) as i32;
    if id >= n || id < 0 {
        return None;
    }
    let nacao = cpw13_d(c + 0x53);
    if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 || cpw13_d(nacao) == cpw13_d(0x9cf260) {
        return None;
    }
    Some(cpw13_d(this + 0x20).wrapping_add(6 * id as usize))
}

/// Soma `conta` convocados do clube `c` na data `d` (dia +0, ano +2): na entrada da mesma data, se houver; senão uma
/// entrada nova (com o vetor cheio, mais 3 vagas por 0x945667) com a data e a conta, e o vetor reordenado pela data
/// (qsort 0x945333 com 0x778e90). Sem memória: a caixa (linha 0x21c8) e false (a rotina para).
unsafe fn cpw13_somar(this: usize, c: usize, conta: u8, d: *mut u8) -> bool {
    let rec = match cpw13_registro(this, c) {
        Some(r) => r,
        None => return true,
    };
    let quantas = *((rec + 5) as *const i8);
    let (dia, ano) = (core::ptr::read_unaligned(d as *const u16), core::ptr::read_unaligned((d as usize + 2) as *const u16));
    let mut k: i8 = 0;
    while k < quantas {
        let e = cpw13_d(rec).wrapping_add(9 * k as i32 as usize);
        if core::ptr::read_unaligned((e + 2) as *const u16) == ano && core::ptr::read_unaligned(e as *const u16) == dia {
            crate::comparacao::ramo("cpw_13 soma na data");
            *((e + 8) as *mut u8) = (*((e + 8) as *const u8)).wrapping_add(conta); // cpw13: soma na data
            return true;
        }
        k += 1;
    }
    if quantas == *((rec + 4) as *const i8) {
        let cap = (*((rec + 4) as *const u8)).wrapping_add(3);
        *((rec + 4) as *mut u8) = cap;
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let v = realocar(cpw13_d(rec), (cap as i8 as i32 * 9) as u32);
        core::ptr::write_unaligned(rec as *mut usize, v);
        if v == 0 {
            aviso_em(0xa5b2a4, 0x21c8);
            return false;
        }
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> usize = core::mem::transmute(0x417c10usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let q = *((rec + 5) as *const i8) as i32;
    let e = cpw13_d(rec).wrapping_add(9 * q as usize);
    let mut t = [0u8; 8];
    copia(e, t.as_mut_ptr(), d);
    crate::comparacao::ramo("cpw_13 registro novo");
    *((e + 8) as *mut u8) = conta; // cpw13: a conta do registro novo
    let q = (*((rec + 5) as *const i8)).wrapping_add(1);
    *((rec + 5) as *mut i8) = q;
    qsort(cpw13_d(rec), q as i32 as u32, 9, 0x778e90);
    true
}

/// { 0x7771c0 } (thiscall, ret 0xc, national_teams.cpp; quadro SEH só do estado, as datas não têm destrutor):
/// `NationalTeams::record_club_call_ups(time, b, partida)`: ordena o elenco do time (+0xd7, 50 vagas) pelo clube (qsort
/// com 0x778ef0). Com a partida: a data dela (+0x2a, +0x28; um dia antes no segundo time 0x5409b0 ou com `b`); conta os
/// convocados seguidos de cada clube (+0x39), pulando as vagas vazias, as pessoas fora da faixa ([0xae2364]) e as de
/// outro elenco (+0x5e bit 3 sem `b`, bit 2 com), e ao mudar de clube soma a conta do anterior no registro dele
/// (cpw13_somar). O último clube da volta fica sem registro (como na original).
unsafe extern "thiscall" fn record_club_call_ups(this: usize, time: usize, b: u32, p: usize) {
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let mut d = [0u8; 8];
    padrao(d.as_mut_ptr());
    qsort(time + 0xd7, 0x32, 4, 0x778ef0);
    if p == 0 {
        return;
    }
    let nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let menos: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    // ponytail: a original empurra os curtos +0x2a e +0x28 com o alto do registro; a 0x549ef0 só lê os curtos
    let r = nova(t1.as_mut_ptr(), *((p + 0x2a) as *const u16) as u32, *((p + 0x28) as *const u16) as u32);
    copia(d.as_mut_ptr(), t2.as_mut_ptr(), r);
    if segunda(time) != 0 || b != 0 {
        let r = menos(d.as_mut_ptr(), t2.as_mut_ptr(), 1);
        copia(d.as_mut_ptr(), t1.as_mut_ptr(), r);
    }
    let mut anterior = 0usize;
    let mut conta: u8 = 0;
    for i in 0..0x32usize {
        let s = cpw13_d(time + 0xd7 + 4 * i);
        if s == 0 || cpw13_d(s) as i32 >= cpw13_d(0xae2364) as i32 {
            continue;
        }
        let marcas = *((s + 0x5e) as *const u8);
        if (b == 0 && marcas & 8 != 0) || (b != 0 && marcas & 4 != 0) {
            continue;
        }
        let c = cpw13_d(s + 0x39);
        if c == anterior && anterior != 0 {
            conta = conta.wrapping_add(1);
            continue;
        }
        if anterior != 0 && !cpw13_somar(this, anterior, conta, d.as_mut_ptr()) {
            return;
        }
        anterior = c;
        conta = 1;
    }
    if anterior != 0 && cpw13_registro(this, anterior).is_some() {
        crate::comparacao::ramo("cpw_13 último clube sem registro"); // cpw13: o último clube fica sem registro
    }
}

/// O registro de algum clube que a 0x7771c0 pode somar está com o vetor cheio (capacidade = quantas, inclusive sem
/// vetor)? Então ela pode realocar.
unsafe fn cpw13_pode_realocar(this: usize, time: usize) -> bool {
    (0..0x32usize).any(|i| {
        let s = cpw13_d(time + 0xd7 + 4 * i);
        if s == 0 || cpw13_d(s) as i32 >= cpw13_d(0xae2364) as i32 || cpw13_d(s + 0x39) == 0 {
            return false;
        }
        match cpw13_registro(this, cpw13_d(s + 0x39)) {
            Some(rec) => *((rec + 4) as *const i8) == *((rec + 5) as *const i8),
            None => false,
        }
    })
}

/// Comparador: lado a lado no elenco do time (50 vagas) e, de cada clube do elenco que conta, no registro (6 bytes) e
/// no vetor de entradas inteiro (capacidade × 9). Quando algum vetor pode crescer (0x945667 solta o velho; não roda
/// duas vezes): só a original, conferindo depois o elenco ordenado pelo clube (o mesmo qsort sobre uma cópia, antes).
unsafe extern "thiscall" fn comparar_record_club_call_ups(this: usize, time: usize, b: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    if p != 0 && cpw13_pode_realocar(this, time) {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        let mut esperado: Vec<u32> = (0..0x32usize).map(|i| cpw13_d(time + 0xd7 + 4 * i) as u32).collect();
        // o mesmo qsort da CRT (a ordem dos iguais é a dele), fora do rastro da original
        crate::aleatorio::pausar_rastro(|| qsort(esperado.as_mut_ptr() as usize, 0x32, 4, 0x778ef0));
        o(this, time, b, p);
        let obtido: Vec<u32> = (0..0x32usize).map(|i| cpw13_d(time + 0xd7 + 4 * i) as u32).collect();
        PLACAR.registrar(obtido == esperado, || format!("record_club_call_ups({:x}) elenco original={:x?} esperado={:x?}", time, obtido, esperado));
        return;
    }
    let mut reg = vec![(time + 0xd7, 0xc8)];
    let mut vistos = Vec::new();
    for i in 0..0x32usize {
        let s = cpw13_d(time + 0xd7 + 4 * i);
        if s == 0 || cpw13_d(s) as i32 >= cpw13_d(0xae2364) as i32 {
            continue;
        }
        let c = cpw13_d(s + 0x39);
        if c == 0 || vistos.contains(&c) {
            continue;
        }
        vistos.push(c);
        if let Some(rec) = cpw13_registro(this, c) {
            reg.push((rec, 6));
            let cap = *((rec + 4) as *const i8);
            if cpw13_d(rec) != 0 && cap > 0 {
                reg.push((cpw13_d(rec), 9 * cap as usize));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, time, b, p), || record_club_call_ups(this, time, b, p));
    PLACAR.registrar(igual, || format!("record_club_call_ups({:x}, b {}, partida {:x}) {:x?} x {:x?}", time, b, p, mo, mr));
}

/// { 0x48e190 } (thiscall, ret 0xc, comp.cpp): `Comp::get_fixture_match_importance(partida, a, b)`: a fase da partida
/// (+0x42; 0xff = a própria competição, senão [+0xc] + 4 × fase) responde pela virtual +0x54 com os mesmos argumentos.
unsafe extern "thiscall" fn get_fixture_match_importance(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let fase = *((p + 0x42) as *const u8);
    let c = if fase == 0xff { this } else { cpw13_d(cpw13_d(this + 0xc).wrapping_add(4 * (fase as i8 as i32) as usize)) };
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(cpw13_d(cpw13_d(c) + 0x54));
    f(c, p, a, b)
}

/// Comparador: pelo valor (a virtual da fase só lê; roda duas vezes).
unsafe extern "thiscall" fn comparar_get_fixture_match_importance(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I1%%)(this, p, a, b);
    let n = get_fixture_match_importance(this, p, a, b);
    PLACAR.registrar(r == n, || format!("get_fixture_match_importance({:x}, partida {:x}, {}, {}) original={} rust={}", this, p, a, b, r, n));
    r
}

/// { 0x48d200 } (thiscall, ret 4, comp.cpp): `Comp::get_stage(k)`: −1 a própria competição, 0..+0x30 a fase
/// [+0xc] + 4 × k; fora disso a caixa (linha 0x36e) e 0, pela original.
unsafe extern "thiscall" fn get_stage(this: usize, k: i32) -> usize {
    if k < -1 || k > cpw13_d(this + 0x30) as i32 {
        return orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I2%%)(this, k);
    }
    if k == -1 {
        return this;
    }
    cpw13_d(cpw13_d(this + 0xc).wrapping_add(4 * k as usize))
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
unsafe extern "thiscall" fn comparar_get_stage(this: usize, k: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I2%%);
    if k < -1 || k > cpw13_d(this + 0x30) as i32 {
        return o(this, k); // ponytail: a caixa, só a original
    }
    let r = o(this, k);
    let n = get_stage(this, k);
    PLACAR.registrar(r == n, || format!("get_stage({:x}, {}) original={:x} rust={:x}", this, k, r, n));
    r
}
