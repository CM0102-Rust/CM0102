// ---------- %%LEVA%%a leva: a MLS: a fase seguinte, o play-off dos oito e a final da MLS Cup (usa_mls.cpp) ----------

unsafe fn cqr4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqr4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`: o clube (+0) e os pontos (+0x19, word).
unsafe fn cqr4_linha(t: usize, i: usize) -> (usize, i16) {
    let l = cqr4_d(t + 0xb1) + i * 0x3b;
    (cqr4_d(l), cqr4_w(l + 0x19))
}

/// Grava o registro de 6 bytes `k` (clube, pontos) do vetor `r`.
fn cqr4_por(r: &mut [u8], k: usize, (c, p): (usize, i16)) {
    r[6 * k..6 * k + 4].copy_from_slice(&(c as u32).to_le_bytes());
    r[6 * k + 4..6 * k + 6].copy_from_slice(&p.to_le_bytes());
}

/// O clube do registro de 6 bytes `k` do vetor `r`.
fn cqr4_clube(r: &[u8], k: usize) -> usize {
    u32::from_le_bytes([r[6 * k], r[6 * k + 1], r[6 * k + 2], r[6 * k + 3]]) as usize
}

/// { 0x909400 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::advance_stage()`, a fase seguinte da MLS (as fases 0 e 1 são
/// as duas outras divisões, que correm junto com a liga; a 2, o play-off; a 3, a final). Na última fase (+0x30 ≥
/// +0x2c − 1, com sinal): a caixa de erro da linha 0x55b e nada mais. Senão: da fase 1 vai à 2 e da 2 à 3 (de outra
/// fase, +0x30 fica); então, na 2, monta o play-off (0x9094c0) e, na 3, a final (0x9097d0), em call direto com o ecx da
/// liga.
unsafe extern "thiscall" fn usa_mls_advance_stage(this: usize) {
    let fase = cqr4_d(this + 0x30) as i32;
    if fase >= (cqr4_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0xaa1fb4, 0x55b);
    }
    if fase == 1 {
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
    } else if fase == 2 {
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 3);
    }
    match cqr4_d(this + 0x30) {
        2 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x9094c0usize);
            f(this);
        }
        3 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x9097d0usize);
            f(this);
        }
        _ => {}
    }
}

/// { 0x9094c0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_playoff_stage()`, o play-off
/// da MLS, da fase seguinte.
/// 1) Uma lista de 8 (malloc 0x944e46 de 0x20; sem memória: a caixa da linha 0x58d e volta).
/// 2) Os três campeões de divisão (o primeiro da tabela da liga, da fase 0 e da fase 1), em registros de 6 bytes (o
///    clube e os pontos, +0x19 da linha), ordenados pelos pontos (qsort 0x945333 com 0x90a130: mais pontos primeiro),
///    vão às vagas 1, 3 e 5 da lista.
/// 3) Os outros: o segundo, o terceiro e o quarto de cada tabela (na ordem: liga, fase 0, fase 1, linha a linha), nove
///    registros ordenados pelos pontos do mesmo jeito; os cinco melhores vão às vagas 7, 6, 4, 2 e 0 (o melhor na 7).
///    Os pares do play-off: o melhor campeão contra o quinto melhor dos outros, o segundo campeão contra o quarto, o
///    terceiro campeão contra o terceiro, e o melhor dos outros contra o segundo [inferência: os pares da lista].
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 2, &rodadas, &extra, 0), ret 0x10); a fase (operator
///    new 0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 8, a lista, as rodadas, +4, a
///    tabela, o ano +0x40, 2, 1, o extra, 0x12, 1, 0, 0 e sementes nulas; a cópia passa as words limpas) em [+0xc][2]
///    e a tabela liberada (free 0x9452ca). Sem fase: a caixa da linha 0x5c4. Com ela: Cup::set_up_round (0x51c800,
///    thiscall(fase, 0), ret 4). A lista não é liberada (a fase fica com ela; a original a deixa assim).
unsafe extern "thiscall" fn usa_mls_create_playoff_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let (a, b) = (cqr4_d(cqr4_d(this + 0xc)), cqr4_d(cqr4_d(this + 0xc) + 4));
    let lista = alocar(0x20);
    if lista == 0 {
        return aviso_em(0xaa1fb4, 0x58d);
    }
    let poe = |k: usize, c: usize| core::ptr::write_unaligned((lista + 4 * k) as *mut usize, c);
    let mut r = [0u8; 54];
    cqr4_por(&mut r, 0, cqr4_linha(this, 0));
    cqr4_por(&mut r, 1, cqr4_linha(a, 0));
    cqr4_por(&mut r, 2, cqr4_linha(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    poe(1, cqr4_clube(&r, 0)); // cqr4: o melhor campeão de divisão
    poe(3, cqr4_clube(&r, 1));
    poe(5, cqr4_clube(&r, 2));
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            cqr4_por(&mut r, k, cqr4_linha(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    poe(7, cqr4_clube(&r, 0));
    poe(6, cqr4_clube(&r, 1));
    poe(4, cqr4_clube(&r, 2));
    poe(2, cqr4_clube(&r, 3));
    poe(0, cqr4_clube(&r, 4)); // cqr4: o quinto melhor dos outros
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqr4_d(cqr4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 8, lista, rodadas as u16 as u32, cqr4_d(this + 4), t, cqr4_w(this + 0x40) as u16 as u32, 2, 1,
                  extra as u16 as u32, 0x12, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqr4_d(this + 0xc) + 8) as *mut usize, fase);
    soltar(t);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0xaa1fb4, 0x5c4);
    }
    crate::comparacao::ramo("cqr_4 play-off");
    montar(fase, 0);
}

/// { 0x9097d0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_final_stage()`, a final da MLS
/// Cup, da fase seguinte: os dois primeiros times do play-off ([+0xc][2], os registros de 6 bytes de +0xa7, na ordem)
/// com a marca de vencedor (+5 = 1) numa lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x5e5 e
/// volta); a tabela das rodadas pela virtual +0x3c (thiscall(this, 3, &rodadas, &extra, 0), ret 0x10); a fase
/// (operator new 0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00: a liga, 2, a lista, as rodadas, +4, a tabela, o ano,
/// 3, 1, o extra, 0x14, 0, 0, 0 e sementes nulas) em [+0xc][3] e a tabela liberada. Sem fase: a caixa da linha 0x5fe e
/// volta (sem liberar a lista). Com ela: Cup::set_up_round (thiscall(fase, 0)) e a lista liberada (free 0x9452ca).
unsafe extern "thiscall" fn usa_mls_create_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let po = cqr4_d(cqr4_d(this + 0xc) + 8);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0xaa1fb4, 0x5e5);
    }
    let mut n = 0i8;
    let mut i = 0i32;
    while i < cqr4_w(po + 0x3e) as i32 && n < 2 {
        let e = cqr4_d(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            core::ptr::write_unaligned((lista + 4 * n as usize) as *mut usize, cqr4_d(e)); // cqr4: os finalistas
            n += 1;
        }
        i += 1;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqr4_d(cqr4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 3, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista, rodadas as u16 as u32, cqr4_d(this + 4), t, cqr4_w(this + 0x40) as u16 as u32, 3, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqr4_d(this + 0xc) + 0xc) as *mut usize, fase);
    soltar(t);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 0xc);
    if fase == 0 {
        return aviso_em(0xaa1fb4, 0x5fe);
    }
    crate::comparacao::ramo("cqr_4 final");
    montar(fase, 0);
    soltar(lista);
}

unsafe extern "thiscall" fn comparar_usa_mls_advance_stage(this: usize) {
    let antes = cqr4_d(this + 0x30) as i32;
    let ultima = antes >= (cqr4_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let depois = cqr4_d(this + 0x30) as i32;
    let esperado = if ultima { antes } else if antes == 1 || antes == 2 { antes + 1 } else { antes };
    PLACAR.registrar(depois == esperado, || format!("usa_mls_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela cópia da escolha (os mesmos registros e o mesmo qsort): a fase da terceira vaga existe, com +0x3e = 8, +4
/// = o +4 da liga e os oito na lista dela (+0xa7, 6 bytes; como conjunto).
unsafe extern "thiscall" fn comparar_usa_mls_create_playoff_stage(this: usize) {
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let (a, b) = (cqr4_d(cqr4_d(this + 0xc)), cqr4_d(cqr4_d(this + 0xc) + 4));
    let mut plano = [0usize; 8];
    let mut r = [0u8; 54];
    cqr4_por(&mut r, 0, cqr4_linha(this, 0));
    cqr4_por(&mut r, 1, cqr4_linha(a, 0));
    cqr4_por(&mut r, 2, cqr4_linha(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    for k in 0..3 {
        plano[k] = cqr4_clube(&r, k);
    }
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            cqr4_por(&mut r, k, cqr4_linha(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    for k in 0..5 {
        plano[3 + k] = cqr4_clube(&r, k);
    }
    orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 8);
    let igual = fase != 0 && cqr4_w(fase + 0x3e) == 8 && cqr4_d(fase + 4) == cqr4_d(this + 4) && {
        let mut nela = [0usize; 8];
        for (i, c) in nela.iter_mut().enumerate() {
            *c = cqr4_d(cqr4_d(fase + 0xa7) + 6 * i);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    PLACAR.registrar(igual, || format!("usa_mls_create_playoff_stage({:#x}) fase={:#x} plano={:x?}", this, fase, plano));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os dois primeiros vencedores do play-off. Depois, a fase da quarta vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (como conjunto).
unsafe extern "thiscall" fn comparar_usa_mls_create_final_stage(this: usize) {
    let po = cqr4_d(cqr4_d(this + 0xc) + 8);
    let mut v = [0usize; 2];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqr4_w(po + 0x3e) as i32 && n < 2 {
        let e = cqr4_d(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            v[n] = cqr4_d(e);
            n += 1;
        }
        i += 1;
    }
    orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 0xc);
    // ponytail: com menos de dois vencedores a original lê vagas da lista sem valor; aí só a fase é conferida.
    let igual = fase != 0 && n < 2 || fase != 0 && {
        let l = cqr4_d(fase + 0xa7);
        let (x, y) = (cqr4_d(l), cqr4_d(l + 6));
        cqr4_w(fase + 0x3e) == 2 && cqr4_d(fase + 4) == cqr4_d(this + 4) && ((x, y) == (v[0], v[1]) || (x, y) == (v[1], v[0]))
    };
    PLACAR.registrar(igual, || format!("usa_mls_create_final_stage({:#x}) finalistas={:x?} n={} fase={:#x}", this, v, n, fase));
}
