// ---------- %%LEVA%%a leva: o registro de histórias da mídia: registrar uma história, expirar as antigas e se a história já foi contada ----------

unsafe fn cpl6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl6_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpl6_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A data `a` é anterior à `b`? Como a original: a palavra +2 sem sinal e, igual, a palavra +0 com sinal.
unsafe fn cpl6_antes(a: usize, b: usize) -> bool {
    let (x, y) = (cpl6_w(a + 2), cpl6_w(b + 2));
    if x != y {
        return x < y;
    }
    (cpl6_w(a) as i16) < (cpl6_w(b) as i16)
}

/// O id (+0) do sujeito, ou −1 sem ele.
unsafe fn cpl6_id(p: usize) -> usize {
    if p != 0 { cpl6_d(p) } else { usize::MAX }
}

/// Os 23 bytes que a 0x757a50 grava no registro (data de hoje [0xae2c90], ids, resposta, subtipo, tipo).
unsafe fn cpl6_registro(a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) -> [u8; 0x17] {
    let mut r = [0u8; 0x17];
    r[..8].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    r[8..0xc].copy_from_slice(&(cpl6_id(a) as u32).to_le_bytes());
    r[0xc..0x10].copy_from_slice(&(cpl6_id(b) as u32).to_le_bytes());
    r[0x10] = resposta as u8;
    r[0x11..0x13].copy_from_slice(&(subtipo as u8 as i8 as i16).to_le_bytes());
    r[0x13..].copy_from_slice(&tipo.to_le_bytes());
    r
}

unsafe fn cpl6_ordenar(base: usize, n: usize) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(base, n as u32, 0x17, 0x757ee0);
}

/// { 0x757a50 } (thiscall, ret 0x14, media.cpp): `Media::log_story(a, b, tipo, subtipo, resposta)`: acrescenta ao
/// registro de histórias ([this], capacidade +4, conta +8; registros de 23 bytes) a data de hoje (+0, 0x417c10 de
/// [0xae2c90]), o id de `a` (+8) e o de `b` (+0xc) (−1 sem eles), o `tipo` (+0x13), o `subtipo` (byte com sinal, em
/// palavra +0x11) e a `resposta` (+0x10); a conta sobe e o registro é reordenado (qsort 0x945333 com 0x757ee0). Sem
/// registro, aloca 20 (0x944e46; sem memória, caixa 0x6e11 e fim); cheio, cresce 20 (realloc 0x945667; caixa 0x6e1c).
unsafe extern "thiscall" fn log_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) {
    crate::comparacao::ramo("cpl_6 registrar historia");
    if cpl6_d(this) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let p = alocar(0x1cc);
        cpl6_por(this, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6e11);
            erro_fatal();
        }
        cpl6_por(this + 4, 0x14);
    }
    let cap = cpl6_d(this + 4);
    if cap == cpl6_d(this + 8) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpl6_d(this), cap.wrapping_add(0x14).wrapping_mul(0x17) as u32);
        cpl6_por(this, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6e1c);
            erro_fatal();
        }
        cpl6_por(this + 4, cpl6_d(this + 4).wrapping_add(0x14));
    }
    let item = || cpl6_d(this).wrapping_add(cpl6_d(this + 8).wrapping_mul(0x17));
    cpl6_por(item() + 8, cpl6_id(a));
    cpl6_por(item() + 0xc, cpl6_id(b));
    cph3_data(item(), 0xae2c90);
    cpl6_por(item() + 0x13, tipo as usize);
    core::ptr::write_unaligned((item() + 0x11) as *mut i16, subtipo as u8 as i8 as i16);
    *((item() + 0x10) as *mut u8) = resposta as u8;
    let n = cpl6_d(this + 8).wrapping_add(1);
    cpl6_por(this + 8, n); // mais uma história
    cpl6_ordenar(cpl6_d(this), n);
}

/// { 0x757c70 } (thiscall, ret 0, media.cpp): `Media::expire_story_log()`: cada história do registro anterior ao
/// limite (hoje − 100 dias; a do time, tipo 0xbdf, hoje − (40 + sorteio(40)), um sorteio 0x90cfc0 por história dessas
/// a cada passada; datas da 0x54c8e0 sobre [0xae2c90]) recebe por cima a última (23 bytes), a conta desce e o mesmo
/// lugar é visto de novo. Sobrando alguma, reordena (qsort com 0x757ee0). Sem registro, nada.
unsafe extern "thiscall" fn expire_story_log(this: usize) {
    if cpl6_d(this) == 0 {
        return;
    }
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut i = 0i32;
    while i < cpl6_d(this + 8) as i32 {
        let r = cpl6_d(this).wrapping_add((i as usize).wrapping_mul(0x17));
        let dias = if cpl6_d(r + 0x13) == 0xbdf {
            crate::comparacao::ramo("cpl_6 expirar historia do time");
            let s = sorteio(0x28); // o sorteio da história do time
            (s + 0x28) as u32
        } else {
            100 // os dias das outras histórias
        };
        let mut local = [0u8; 8];
        let limite = datar(0xae2c90, &mut local, dias);
        if cpl6_antes(r, limite) {
            let n = cpl6_d(this + 8);
            let ultimo = cpl6_d(this).wrapping_add(n.wrapping_mul(0x17)).wrapping_sub(0x17);
            core::ptr::copy(ultimo as *const u8, r as *mut u8, 0x17);
            cpl6_por(this + 8, n.wrapping_sub(1));
            i -= 1; // o mesmo lugar de novo
        }
        i += 1;
    }
    if cpl6_d(this + 8) != 0 {
        cpl6_ordenar(cpl6_d(this), cpl6_d(this + 8));
    }
}

/// { 0x757da0 } (thiscall, ret 0x14, media.cpp): `Media::has_logged_story(a, b, tipo, subtipo, desde)`: a história do
/// time (tipo 0xbdf): procura em ordem a de tipo 0xbdf com o id de `b` (+0xc) e o `subtipo` (palavra +0x11), sem olhar
/// `a` nem a data: 1 se achou. As outras: busca binária (bsearch 0x946464 com 0x757ee0) pela chave {id de `a` ou −1,
/// id de `b`, tipo, subtipo} (a data padrão 0x54a110 na chave, que a comparação não lê); achou e sem `desde`, 1; com
/// `desde`, 1 se a história não é anterior a ela. Senão 0. `b` não pode ser nulo (a original lê sem conferir).
unsafe extern "thiscall" fn has_logged_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, desde: usize) -> u32 {
    let mut chave = [0u8; 0x17];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(chave.as_mut_ptr() as usize);
    let s = subtipo as u16;
    if tipo == 0xbdf {
        crate::comparacao::ramo("cpl_6 historia do time ja contada");
        let n = cpl6_d(this + 8) as i32;
        for k in 0..n.max(0) as usize {
            let r = cpl6_d(this).wrapping_add(k * 0x17);
            if cpl6_d(r + 0x13) == 0xbdf && cpl6_d(r + 0xc) == cpl6_d(b) && cpl6_w(r + 0x11) == s {
                return 1; // a história do time já contada
            }
        }
        return 0;
    }
    chave[8..0xc].copy_from_slice(&(cpl6_id(a) as u32).to_le_bytes());
    chave[0x13..].copy_from_slice(&tipo.to_le_bytes());
    chave[0xc..0x10].copy_from_slice(&(cpl6_d(b) as u32).to_le_bytes());
    chave[0x11..0x13].copy_from_slice(&s.to_le_bytes());
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = bsearch(chave.as_ptr() as usize, cpl6_d(this), cpl6_d(this + 8) as u32, 0x17, 0x757ee0);
    if e == 0 {
        return 0;
    }
    if desde == 0 {
        return 1;
    }
    if cpl6_antes(e, desde) { 0 } else { 1 }
}

/// Comparador: a original pode alocar e realocar (não roda duas vezes): o registro esperado antes; confere a conta e
/// que o registro esperado está na lista (a ordem é a do qsort).
unsafe extern "thiscall" fn comparar_log_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) {
    let n = cpl6_d(this + 8);
    let esperado = cpl6_registro(a, b, tipo, subtipo, resposta);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I0%%)(this, a, b, tipo, subtipo, resposta);
    let m = cpl6_d(this + 8);
    let achou = (0..m).any(|k| {
        core::slice::from_raw_parts(cpl6_d(this).wrapping_add(k * 0x17) as *const u8, 0x17) == &esperado[..]
    });
    PLACAR.registrar(m == n.wrapping_add(1) && achou, || {
        format!("log_story(tipo {:#x}, subtipo {}) conta {} -> {}, esperado {:x?} achou {}", tipo, subtipo as u8 as i8, n, m, esperado, achou)
    });
}

/// Comparador: lado a lado pelo registro (ponteiro, capacidade e conta; as histórias) e o estado do sorteio (a cópia
/// sem contar os sorteios).
unsafe extern "thiscall" fn comparar_expire_story_log(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    if cpl6_d(this) == 0 {
        return o(this);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0xc));
    reg.push((cpl6_d(this), (cpl6_d(this + 8) as i32).max(0) as usize * 0x17));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| expire_story_log(this)));
    // mo.0[2] e mr.0[2]: ponteiro, capacidade e conta depois de cada lado
    PLACAR.registrar(igual, || format!("expire_story_log {:x?} x {:x?}", mo.0[2].1, mr.0[2].1));
}

/// Comparador: pelo valor (a busca e a data padrão da chave só leem).
unsafe extern "thiscall" fn comparar_has_logged_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, desde: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> u32>(%%I2%%)(this, a, b, tipo, subtipo, desde);
    let n = has_logged_story(this, a, b, tipo, subtipo, desde);
    PLACAR.registrar(r == n, || format!("has_logged_story(tipo {:#x}, subtipo {}, desde {}) original={} rust={}", tipo, subtipo as u16 as i16, desde != 0, r, n));
    r
}
