// ---------- %%LEVA%%a leva: os interesses de transferência da mídia: acrescentar um interesse e as comparações das chaves de ids e das histórias ----------

unsafe fn cpl7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl7_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A chave da 0x754740: {id do jogador, id do clube ou −1} (os 8 bytes que a 0x757ea0 lê).
unsafe fn cpl7_chave(jogador: usize, clube: usize) -> [u32; 2] {
    [cpl7_d(jogador) as u32, if clube != 0 { cpl7_d(clube) as u32 } else { u32::MAX }]
}

/// O interesse já está no vetor? (bsearch 0x946464 com 0x757ea0; sem vetor, não).
unsafe fn cpl7_ja_esta(this: usize, chave: &[u32; 2]) -> bool {
    let base = cpl7_d(this + 0xc);
    if base == 0 {
        return false;
    }
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(chave.as_ptr() as usize, base, cpl7_d(this + 0x14) as u32, 0x19, 0x757ea0) != 0
}

/// O valor que a 0x754740 grava: o `valor` limitado a 50.000.000.
fn cpl7_valor(valor: i32) -> i32 {
    if valor < 0x2faf080 { valor } else { 0x2faf080 }
}

/// { 0x754740 } (thiscall, ret 0x14, media.cpp): `Media::add_transfer_interest(jogador, clube, valor, avaliacao)`: com
/// o vetor de interesses ([this+0xc], capacidade +0x10, conta +0x14; registros de 25 bytes), procura (bsearch 0x946464
/// com 0x757ea0) a chave {id do jogador, id do clube ou −1}: já está, nada. Senão acrescenta: id do jogador (+0), id do
/// clube (+4, lido sem conferir), a data de hoje (+8, 0x417c10 de [0xae2c90]), 0 (+0x10, o boato ainda não saiu), o
/// `valor` até 50.000.000 (+0x11) e a `avaliacao` × 100 truncada (+0x15, _ftol 0x945640); a conta sobe e reordena (qsort
/// com 0x757ea0). Sem vetor, aloca 20 (0x944e46; sem memória, caixa 0x665d e fim); cheio, cresce 20 (realloc 0x945667;
/// caixa 0x6668). A data padrão (0x54a110) da chave na pilha, como a original.
unsafe extern "thiscall" fn add_transfer_interest(this: usize, jogador: usize, clube: usize, valor: i32, avaliacao: f64) {
    let mut data = [0u8; 8];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(data.as_mut_ptr() as usize);
    let chave = cpl7_chave(jogador, clube);
    if cpl7_ja_esta(this, &chave) {
        crate::comparacao::ramo("cpl_7 interesse repetido");
        return; // o interesse já está
    }
    crate::comparacao::ramo("cpl_7 interesse novo");
    if cpl7_d(this + 0xc) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let p = alocar(0x1f4);
        cpl7_por(this + 0xc, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x665d);
            erro_fatal();
        }
        cpl7_por(this + 0x10, 0x14);
    }
    let cap = cpl7_d(this + 0x10);
    if cap == cpl7_d(this + 0x14) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpl7_d(this + 0xc), cap.wrapping_add(0x14).wrapping_mul(0x19) as u32);
        cpl7_por(this + 0xc, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6668);
            erro_fatal();
        }
        cpl7_por(this + 0x10, cpl7_d(this + 0x10).wrapping_add(0x14));
    }
    let item = || cpl7_d(this + 0xc).wrapping_add(cpl7_d(this + 0x14).wrapping_mul(0x19));
    cpl7_por(item(), cpl7_d(jogador));
    cpl7_por(item() + 4, cpl7_d(clube));
    cph3_data(item() + 8, 0xae2c90);
    *((item() + 0x10) as *mut u8) = 0; // o boato ainda não saiu
    cpl7_por(item() + 0x11, cpl7_valor(valor) as usize);
    cpl7_por(item() + 0x15, ftol(avaliacao * 100.0) as usize); // [0x9688a8] = 100.0
    let n = cpl7_d(this + 0x14).wrapping_add(1);
    cpl7_por(this + 0x14, n); // mais um interesse
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpl7_d(this + 0xc), n as u32, 0x19, 0x757ea0);
}

/// { 0x757ea0 } (cdecl, media.cpp): `compare_id_pair_keys(a, b)` do qsort e do bsearch: nulo antes (`a` nulo −1, `b`
/// nulo 1); ids +0 diferentes, a diferença; iguais, 0 se um dos +4 é −1 (o coringa), senão a diferença dos +4.
unsafe extern "C" fn compare_id_pair_keys(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (x, y) = (cpl7_d(a) as i32, cpl7_d(b) as i32);
    if x != y {
        return x.wrapping_sub(y);
    }
    let (x, y) = (cpl7_d(a + 4) as i32, cpl7_d(b + 4) as i32);
    if x == -1 || y == -1 {
        return 0;
    }
    x.wrapping_sub(y)
}

/// { 0x757ee0 } (cdecl, media.cpp): `compare_story_log_records(a, b)` do qsort e do bsearch do registro de histórias:
/// nulo antes (`a` nulo −1, `b` nulo 1); pelo sujeito a (+8), o sujeito b (+0xc) e o tipo (+0x13), a diferença do
/// primeiro diferente; iguais, a diferença dos subtipos (palavras +0x11 com sinal).
unsafe extern "C" fn compare_story_log_records(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    for o in [8usize, 0xc, 0x13] {
        let (x, y) = (cpl7_d(a + o) as i32, cpl7_d(b + o) as i32);
        if x != y {
            return x.wrapping_sub(y);
        }
    }
    let s = |p: usize| core::ptr::read_unaligned((p + 0x11) as *const i16) as i32;
    s(a) - s(b)
}

/// Os 25 bytes que a 0x754740 acrescenta.
unsafe fn cpl7_interesse(jogador: usize, clube: usize, valor: i32, avaliacao: f64) -> [u8; 0x19] {
    let mut r = [0u8; 0x19];
    r[..4].copy_from_slice(&(cpl7_d(jogador) as u32).to_le_bytes());
    r[4..8].copy_from_slice(&(cpl7_d(clube) as u32).to_le_bytes());
    r[8..0x10].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    r[0x11..0x15].copy_from_slice(&cpl7_valor(valor).to_le_bytes());
    r[0x15..].copy_from_slice(&ftol(avaliacao * 100.0).to_le_bytes());
    r
}

/// Comparador: a original pode alocar e realocar (não roda duas vezes): o plano antes (já está? o registro novo);
/// confere a conta e que o registro novo está no vetor (a ordem é a do qsort).
unsafe extern "thiscall" fn comparar_add_transfer_interest(this: usize, jogador: usize, clube: usize, valor: i32, avaliacao: f64) {
    let n = cpl7_d(this + 0x14);
    let ja_esta = cpl7_ja_esta(this, &cpl7_chave(jogador, clube));
    let esperado = if ja_esta { None } else { Some(cpl7_interesse(jogador, clube, valor, avaliacao)) };
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, i32, f64)>(%%I0%%)(this, jogador, clube, valor, avaliacao);
    let m = cpl7_d(this + 0x14);
    let ok = match &esperado {
        None => m == n,
        Some(e) => m == n.wrapping_add(1) && (0..m).any(|k| {
            core::slice::from_raw_parts(cpl7_d(this + 0xc).wrapping_add(k * 0x19) as *const u8, 0x19) == &e[..]
        }),
    };
    PLACAR.registrar(ok, || format!("add_transfer_interest(valor {}, avaliacao {}) conta {} -> {}, esperado {:x?}", valor, avaliacao, n, m, esperado));
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_compare_id_pair_keys(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_id_pair_keys(a, b);
    PLACAR.registrar(r == n, || format!("compare_id_pair_keys({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_compare_story_log_records(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_story_log_records(a, b);
    PLACAR.registrar(r == n, || format!("compare_story_log_records({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
