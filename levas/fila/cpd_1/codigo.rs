// ---------- %%LEVA%%a leva: as seleções: o fim das convocações de uma competição e a poda dos registros de convocação (national_teams.cpp) ----------

unsafe fn cpd1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Os itens da lista `l` (0x68d0d0: começo em +2, nó = item +0 e seguinte +6), sem mexer no cursor (+0xa).
unsafe fn cpd1_itens(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpd1_d(l + 2);
    while no != 0 && v.len() < 100_000 {
        v.push(cpd1_d(no));
        no = cpd1_d(no + 6);
    }
    v
}

/// O fim das convocações da lista [this] (as duas rotinas da 0x774f30/0x774f90): do começo (cursor +0xa = +2),
/// cada registro que `passa` recebe em +0x10 a palavra +0 (o dia) de amanhã (0x54c5b0 com 1 sobre [0xae2c90]).
unsafe fn cpd1_encerrar(this: usize, passa: impl Fn(usize) -> bool) {
    let l = cpd1_d(this);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpd1_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *const u16 = core::mem::transmute(0x54c5b0usize);
    let mut r = proximo(cpd1_d(this));
    while r != 0 {
        if passa(r) {
            let mut data = [0u8; 8];
            let dia = *mais_dias(0xae2c90, data.as_mut_ptr(), 1);
            core::ptr::write_unaligned((r + 0x10) as *mut u16, dia); // o dia de amanhã
        }
        r = proximo(cpd1_d(this));
    }
}

/// { 0x774f30 } (thiscall, ret 4, national_teams.cpp): `end_duty_for_competition(comp)`: na lista de registros
/// de convocação [this], os da competição (+4 igual a *comp) terminam amanhã (+0x10). Sem SEH, sem caixa, sem
/// sorteio; a local é a data que a 0x54c5b0 grava.
unsafe extern "thiscall" fn end_duty_for_competition(this: usize, comp: usize) {
    cpd1_encerrar(this, |r| cpd1_d(r + 4) == cpd1_d(comp) /* a competição */)
}

/// { 0x774f90 } (thiscall, ret 8, national_teams.cpp): `end_duty_for_nation_in_competition(nacao, comp)`: como a
/// 0x774f30, só os registros da competição (+4) e da nação (+0 igual a *nacao).
unsafe extern "thiscall" fn end_duty_for_nation_in_competition(this: usize, nacao: usize, comp: usize) {
    cpd1_encerrar(this, |r| cpd1_d(r + 4) == cpd1_d(comp) && cpd1_d(r) == cpd1_d(nacao) /* a nação */)
}

/// As regiões do fim das convocações: o cursor da lista e o +0x10 de cada registro.
unsafe fn cpd1_regioes_encerrar(this: usize) -> Vec<(usize, usize)> {
    let l = cpd1_d(this);
    if l == 0 {
        return Vec::new();
    }
    let mut reg = vec![(l + 0xa, 4)];
    reg.extend(cpd1_itens(l).into_iter().filter(|&r| r != 0).map(|r| (r + 0x10, 2)));
    reg
}

unsafe extern "thiscall" fn comparar_end_duty_for_competition(this: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, comp), || end_duty_for_competition(this, comp));
    PLACAR.registrar(igual, || format!("end_duty_for_competition({:x}, {:x}) {:x?} x {:x?}", this, comp, mo, mr));
}

unsafe extern "thiscall" fn comparar_end_duty_for_nation_in_competition(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao, comp), || end_duty_for_nation_in_competition(this, nacao, comp));
    PLACAR.registrar(igual, || format!("end_duty_for_nation_in_competition({:x}, {:x}, {:x}) {:x?} x {:x?}", this, nacao, comp, mo, mr));
}

/// O número de registros de convocação por clube da 0x7774b0: [0xae235c] − 2 × [0xae2350].
unsafe fn cpd1_quantos() -> i32 {
    (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2))
}

/// { 0x7774b0 } (thiscall, national_teams.cpp): `prune_club_call_up_records()`: cada registro de clube (+0x20,
/// 6 bytes: +0 vetor de entradas de 9 bytes, +4 marca, +5 quantas, byte com sinal) conta as entradas já vencidas
/// (0x456770: a data delas antes de [0xae2c90]; a rotina supõe que ficam no começo), desce as outras por cima
/// (0x777590) e diminui a contagem; zerada com vetor, solta o vetor (0x9452ca) e zera +4 e +0. Sem SEH, sem caixa,
/// sem sorteio; a local (o vencido) é zerada antes de ler.
unsafe extern "thiscall" fn prune_club_call_up_records(this: usize) {
    let n = cpd1_quantos();
    let mut rec = cpd1_d(this + 0x20);
    if n <= 0 {
        return;
    }
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x777590usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let conta = |rec: usize| *((rec + 5) as *const i8);
    for _ in 0..n {
        let mut vencidas: i8 = 0;
        if conta(rec) > 0 {
            let mut k: i8 = 0;
            loop {
                let e = cpd1_d(rec).wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                if antes(e, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0 {
                    vencidas = vencidas.wrapping_add(1); // uma vencida
                }
                k = k.wrapping_add(1);
                if k >= conta(rec) {
                    break;
                }
            }
        }
        if vencidas < conta(rec) {
            let mut k = vencidas;
            loop {
                crate::comparacao::ramo("convocações descer as que ficam");
                let v = cpd1_d(rec);
                let de = v.wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                let para = v.wrapping_add(9usize.wrapping_mul((k as i32 - vencidas as i32) as usize));
                copiar(para, de);
                k = k.wrapping_add(1);
                if k >= conta(rec) {
                    break;
                }
            }
        }
        let resto = conta(rec).wrapping_sub(vencidas);
        *((rec + 5) as *mut i8) = resto;
        if resto == 0 && cpd1_d(rec) != 0 {
            crate::comparacao::ramo("convocações soltar o vetor");
            free(cpd1_d(rec));
            *((rec + 4) as *mut u8) = 0;
            core::ptr::write_unaligned(rec as *mut usize, 0);
        }
        rec += 6;
    }
}

/// O registro `rec` da 0x7774b0 terminaria sem entradas e com vetor (a original solta o vetor)? Escrito à parte da
/// cópia, para o comparador.
unsafe fn cpd1_soltaria(rec: usize) -> bool {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let n = *((rec + 5) as *const i8);
    let v = cpd1_d(rec);
    let vencidas = (0..n.max(0) as usize).filter(|&k| antes(v + 9 * k, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0).count();
    v != 0 && n.wrapping_sub(vencidas as i8) == 0
}

/// Comparador: os registros (6 bytes cada) e os vetores de entradas. Algum vetor a soltar (0x9452ca; soltar duas
/// vezes): só a original.
unsafe extern "thiscall" fn comparar_prune_club_call_up_records(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let n = cpd1_quantos().max(0) as usize;
    let recs = cpd1_d(this + 0x20);
    if n == 0 || recs == 0 || (0..n).any(|i| cpd1_soltaria(recs + 6 * i)) {
        return o(this);
    }
    let mut reg = vec![(recs, 6 * n)];
    for i in 0..n {
        let (v, k) = (cpd1_d(recs + 6 * i), *((recs + 6 * i + 5) as *const i8));
        if v != 0 && k > 0 {
            reg.push((v, 9 * k as usize));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || prune_club_call_up_records(this));
    PLACAR.registrar(igual, || format!("prune_club_call_up_records({:x}) {:x?} x {:x?}", this, mo, mr));
}
