// ---------- %%LEVA%%a leva: os idiomas do jogador (acrescentar e melhorar) e zerar as estatísticas do jogador regenerado (player_stats.cpp) ----------

unsafe fn cpo12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo12_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de idioma (6 bytes: +0 id da pessoa, +4 idioma, +5 fluência) da `pessoa` no `idioma`: bsearch
/// (0x946464, comparação 0x7c6d40) na tabela [this+0x3c], [this+0x44] registros. 0 sem achar.
unsafe fn find_language_record(this: usize, pessoa: usize, idioma: u8) -> usize {
    let bsearch: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut chave = [0u8; 8];
    core::ptr::write_unaligned(chave.as_mut_ptr() as *mut u32, cpo12_d(pessoa) as u32);
    chave[4] = idioma; // a comparação só lê o byte +4 da chave
    bsearch(chave.as_ptr(), cpo12_d(this + 0x3c), cpo12_d(this + 0x44) as u32, 6, 0x7c6d40)
}

/// { 0x7c6a30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::add_language(pessoa, idioma, fluencia,
/// ordenar)`: a tabela dos idiomas (6 bytes por registro em [this+0x3c], capacidade +0x40, conta +0x44) nasce com 100
/// (malloc 0x258) e cresce 100 (realloc) quando cheia; o registro novo no fim: o id da pessoa, o idioma e a fluência
/// (no máximo 10, com sinal); com `ordenar`, qsort pela comparação 0x7c6d40. Sem memória: caixa fatal.
unsafe extern "thiscall" fn add_language(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    if cpo12_d(this + 0x3c) == 0 {
        let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let p = malloc(0x258);
        cpo12_por(this + 0x3c, p as u32);
        if p == 0 {
            show_error_at(0xa78310, 0x11fd);
            fatal_exit();
        }
        cpo12_por(this + 0x40, 0x64);
    }
    let cap = cpo12_d(this + 0x40);
    if cap == cpo12_d(this + 0x44) {
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realloc(cpo12_d(this + 0x3c), (cap as u32).wrapping_mul(3).wrapping_add(0x12c).wrapping_mul(2));
        cpo12_por(this + 0x3c, p as u32);
        if p == 0 {
            show_error_at(0xa78310, 0x1208);
            fatal_exit();
        }
        cpo12_por(this + 0x40, (cpo12_d(this + 0x40) as u32).wrapping_add(0x64));
    }
    crate::comparison::branch("cpo_12 new language");
    let n = cpo12_d(this + 0x44);
    let r = cpo12_d(this + 0x3c).wrapping_add(n.wrapping_mul(6));
    cpo12_por(r, cpo12_d(pessoa) as u32);
    *((r + 4) as *mut u8) = idioma as u8;
    let f = fluencia as u8;
    *((r + 5) as *mut u8) = if (f as i8) > 10 { 10 } else { f };
    cpo12_por(this + 0x44, (n as u32).wrapping_add(1));
    if ordenar != 0 {
        crate::comparison::branch("cpo_12 languages sorted");
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cpo12_d(this + 0x3c), cpo12_d(this + 0x44) as u32, 6, 0x7c6d40);
    }
}

/// { 0x7c6ca0 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::improve_language(pessoa, idioma, aumento,
/// ordenar)`: sem tabela, ou sem o registro da pessoa no idioma (`find_language_record`), acrescenta-o com a fluência
/// `aumento` (0x7c6a30); com ele, soma o aumento à fluência (byte), no máximo 10 (com sinal).
unsafe extern "thiscall" fn improve_language(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x7c6a30usize);
    if cpo12_d(this + 0x3c) == 0 {
        acrescentar(this, pessoa, idioma, aumento, ordenar);
        return;
    }
    let r = find_language_record(this, pessoa, idioma as u8);
    if r == 0 {
        acrescentar(this, pessoa, idioma, aumento, ordenar);
        return;
    }
    crate::comparison::branch("cpo_12 language improved");
    let f = (*((r + 5) as *const u8)).wrapping_add(aumento as u8);
    *((r + 5) as *mut u8) = f;
    if (f as i8) > 10 {
        *((r + 5) as *mut u8) = 10;
    }
}

/// { 0x7c6650 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::reset_player(pessoa)`: no registro de 39 bytes da
/// pessoa ([this+0x30] + id × 39), cada uma das 6 categorias (+0..+0x14) com registro (≠ −1) o libera (+0xe = −1 no
/// registro de 32 bytes de [this+4]; +1 nos livres [this+0x38]) e fica −1; zera +0x20, +0x22 (palavras) e +0x24..+0x26
/// (as notas recentes) e esvazia o histórico (0x58f110 sobre [this] com +0x18/+0x1c). Pessoa nula ou não jogadora
/// (+0x61): a original (caixa fatal). Devolve o que a 0x58f110 devolve. Do regen (0x7a59e0).
unsafe extern "thiscall" fn player_stats_reset_player(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 || cpo12_d(pessoa + 0x61) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, pessoa); // caixa fatal: só a original
    }
    let r = cpo12_d(this + 0x30).wrapping_add(cpo12_d(pessoa).wrapping_mul(39));
    for k in 0..6usize {
        let i = cpo12_d(r + 4 * k);
        if i as u32 == 0xffff_ffff {
            continue;
        }
        core::ptr::write_unaligned((cpo12_d(this + 4).wrapping_add(i << 5) + 0xe) as *mut u16, 0xffff);
        cpo12_por(this + 0x38, (cpo12_d(this + 0x38) as u32).wrapping_add(1));
        cpo12_por(r + 4 * k, 0xffff_ffff);
    }
    core::ptr::write_unaligned((r + 0x20) as *mut u16, 0);
    core::ptr::write_unaligned((r + 0x22) as *mut u16, 0);
    *((r + 0x26) as *mut u8) = 0;
    *((r + 0x25) as *mut u8) = 0;
    *((r + 0x24) as *mut u8) = 0;
    let esvaziar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x58f110usize);
    esvaziar(cpo12_d(this), r + 0x18, r + 0x1c)
}

/// Comparador: a 0x7c6a30 aloca e realoca a tabela (não pode rodar duas vezes): o plano antes (a conta e a
/// capacidade depois, a fluência), a original roda; confere a conta, a capacidade e o registro novo (no fim sem ordenar;
/// com `ordenar`, em algum lugar da tabela).
unsafe extern "thiscall" fn compare_add_language(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    let (n, cap) = if cpo12_d(this + 0x3c) == 0 { (cpo12_d(this + 0x44), 100) } else { (cpo12_d(this + 0x44), cpo12_d(this + 0x40)) };
    let cap = if cap == n { cap.wrapping_add(100) } else { cap };
    let f = fluencia as u8;
    let novo = (cpo12_d(pessoa) as u32, idioma as u8, if (f as i8) > 10 { 10 } else { f });
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I0%%)(this, pessoa, idioma, fluencia, ordenar);
    let base = cpo12_d(this + 0x3c);
    let e = |i: usize| {
        let r = base.wrapping_add(i * 6);
        (cpo12_d(r) as u32, *((r + 4) as *const u8), *((r + 5) as *const u8))
    };
    let conta = cpo12_d(this + 0x44);
    let mut ok = conta == n.wrapping_add(1) && cpo12_d(this + 0x40) == cap;
    if ok {
        ok = if ordenar == 0 { e(n) == novo } else { (0..conta).any(|i| e(i) == novo) };
    }
    SCOREBOARD.record(ok, || format!("add_language({:?}, ordenar {}) conta {} (esperava {}) capacidade {} (esperava {})",
        novo, ordenar, conta, n.wrapping_add(1), cpo12_d(this + 0x40), cap));
}

/// Comparador: sem tabela ou sem o registro, só a original (o acréscimo 0x7c6a30 tem o comparador dele); com o
/// registro, lado a lado nos 6 bytes dele.
unsafe extern "thiscall" fn compare_improve_language(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I1%%);
    let r = if cpo12_d(this + 0x3c) == 0 { 0 } else { find_language_record(this, pessoa, idioma as u8) };
    if r == 0 {
        return o(this, pessoa, idioma, aumento, ordenar);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 6)],
        || o(this, pessoa, idioma, aumento, ordenar), || improve_language(this, pessoa, idioma, aumento, ordenar));
    SCOREBOARD.record(igual, || format!("improve_language({}, idioma {}, +{}) {:x?} x {:x?}", cpo12_d(pessoa), idioma as u8, aumento as u8, mo, mr));
}

/// Comparador: lado a lado no registro de 39 bytes, na conta dos livres (+0x38) e na marca +0xe dos registros de
/// categoria liberados (a 0x58f110 só grava +0x1c, dentro do registro). Pessoa nula ou não jogadora: só a original.
unsafe extern "thiscall" fn compare_player_stats_reset_player(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    if pessoa == 0 || cpo12_d(pessoa + 0x61) == 0 {
        return o(this, pessoa);
    }
    let r = cpo12_d(this + 0x30).wrapping_add(cpo12_d(pessoa).wrapping_mul(39));
    let mut reg = vec![(r, 39), (this + 0x38, 4)];
    for k in 0..6usize {
        let i = cpo12_d(r + 4 * k);
        if i as u32 != 0xffff_ffff {
            reg.push((cpo12_d(this + 4).wrapping_add(i << 5) + 0xe, 2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, pessoa), || player_stats_reset_player(this, pessoa));
    SCOREBOARD.record(igual && a == b, || format!("player_stats_reset_player({}) {:x?} x {:x?}", cpo12_d(pessoa), mo, mr));
    a
}
