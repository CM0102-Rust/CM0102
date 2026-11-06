// ---------- 277a leva: as regras de transferência da Noruega, da Irlanda do Norte e do País de Gales: as janelas de transferência (northern_ireland_rules.cpp, norway_rules.cpp, wales_rules.cpp) ----------

unsafe fn cqm2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0 domingo ... 6 sábado, 0xff nenhum) [inferência], +3 o dia, +4 o
/// mês (0 = janeiro), +5 1 no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória,
/// a caixa da linha `linha` do fonte `arquivo`).
unsafe fn cqm2_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqm2_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqm2_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqm2_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqm2_d(this + 4) as usize, cqm2_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Noruega: uma janela, de 1º de setembro a 31 de agosto.
const CQM2_NORUEGA: [[u8; 6]; 2] = [
    [0x11, 0x00, 0xff, 0x01, 0x08, 0x01],
    [0x11, 0x00, 0xff, 0x1f, 0x07, 0x00], // cqm2: a Noruega
];

/// { 0x7944f0 } (thiscall, ret 0x14, norway_rules.cpp, `NORWAY_RULES::NORWAY_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Noruega (regra 17): a base (0x8e84a0), a vtable 0x96dfe4, 1 janela(s) de `CQM2_NORUEGA`. Devolve o objeto.
unsafe extern "thiscall" fn regras_noruega(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_2 noruega");
    cqm2_construir(this, [a1, a2, a3, a4, a5], 0x96dfe4, 1, &CQM2_NORUEGA, 0xa69870, 0x51);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm2_comparar`).
unsafe extern "thiscall" fn comparar_regras_noruega(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1810);
    let mut r = 0usize;
    cqm2_comparar("regras_noruega", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_noruega(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Irlanda do Norte: uma janela, de 6 de maio (andando para o sábado [inferência]) a 20 de março.
const CQM2_IRLANDA_DO_NORTE: [[u8; 6]; 2] = [
    [0x10, 0x00, 0x06, 0x06, 0x04, 0x01], // cqm2: a Irlanda do Norte
    [0x10, 0x00, 0xff, 0x14, 0x02, 0x00],
];

/// { 0x7936d0 } (thiscall, ret 0x14, northern_ireland_rules.cpp, `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda do Norte (regra 16): a base (0x8e84a0), a vtable 0x96df9c, 1 janela(s) de `CQM2_IRLANDA_DO_NORTE`. Devolve o objeto.
unsafe extern "thiscall" fn regras_irlanda_do_norte(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_2 irlanda_do_norte");
    cqm2_construir(this, [a1, a2, a3, a4, a5], 0x96df9c, 1, &CQM2_IRLANDA_DO_NORTE, 0xa697f4, 0x4e);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm2_comparar`).
unsafe extern "thiscall" fn comparar_regras_irlanda_do_norte(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1811);
    let mut r = 0usize;
    cqm2_comparar("regras_irlanda_do_norte", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_irlanda_do_norte(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas do País de Gales: uma janela, de 7 de maio (andando para o domingo [inferência]) a 20 de março.
const CQM2_PAIS_DE_GALES: [[u8; 6]; 2] = [
    [0x1a, 0x00, 0x00, 0x07, 0x04, 0x01], // cqm2: a País de Gales
    [0x1a, 0x00, 0xff, 0x14, 0x02, 0x00],
];

/// { 0x90f080 } (thiscall, ret 0x14, wales_rules.cpp, `WALES_RULES::WALES_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência do País de Gales (regra 26): a base (0x8e84a0), a vtable 0x970a9c, 1 janela(s) de `CQM2_PAIS_DE_GALES`. Devolve o objeto.
unsafe extern "thiscall" fn regras_pais_de_gales(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_2 pais_de_gales");
    cqm2_construir(this, [a1, a2, a3, a4, a5], 0x970a9c, 1, &CQM2_PAIS_DE_GALES, 0xad4404, 0x4e);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm2_comparar`).
unsafe extern "thiscall" fn comparar_regras_pais_de_gales(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1812);
    let mut r = 0usize;
    cqm2_comparar("regras_pais_de_gales", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_pais_de_gales(s, a1, a2, a3, a4, a5);
    });
    r
}

// ---------- 277a leva: as regras de transferência da Turquia, da Polônia e da Finlândia: as janelas de transferência (finland_rules.cpp, poland_rules.cpp, turkey_rules.cpp) ----------

unsafe fn cqm3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0 domingo ... 6 sábado, 0xff nenhum) [inferência], +3 o dia, +4 o
/// mês (0 = janeiro), +5 1 no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória,
/// a caixa da linha `linha` do fonte `arquivo`).
unsafe fn cqm3_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqm3_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqm3_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqm3_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqm3_d(this + 4) as usize, cqm3_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Turquia: uma janela, de 1º de junho a 31 de janeiro.
const CQM3_TURQUIA: [[u8; 6]; 2] = [
    [0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia
    [0x18, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// { 0x9020a0 } (thiscall, ret 0x14, turkey_rules.cpp, `TURKEY_RULES::TURKEY_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Turquia (regra 24): a base (0x8e84a0), a vtable 0x97075c, 1 janela(s) de `CQM3_TURQUIA`; o +0x13 = 8 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 8 (o padrão é 30). Devolve o objeto.
unsafe extern "thiscall" fn regras_turquia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 turquia");
    cqm3_construir(this, [a1, a2, a3, a4, a5], 0x97075c, 1, &CQM3_TURQUIA, 0xaa19a8, 0x52);
    *((this + 0x13) as *mut u8) = 8;
    *((this + 0x14) as *mut u8) = 8;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm3_comparar`).
unsafe extern "thiscall" fn comparar_regras_turquia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1813);
    let mut r = 0usize;
    cqm3_comparar("regras_turquia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_turquia(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Polônia: uma janela, de 1º de julho a 20 de fevereiro.
const CQM3_POLONIA: [[u8; 6]; 2] = [
    [0x12, 0x00, 0xff, 0x01, 0x06, 0x01], // cqm3: a Polônia
    [0x12, 0x00, 0xff, 0x14, 0x01, 0x00],
];

/// { 0x7cd830 } (thiscall, ret 0x14, poland_rules.cpp, `POLAND_RULES::POLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Polônia (regra 18): a base (0x8e84a0), a vtable 0x96e61c, 1 janela(s) de `CQM3_POLONIA`; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30). Devolve o objeto.
unsafe extern "thiscall" fn regras_polonia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 polonia");
    cqm3_construir(this, [a1, a2, a3, a4, a5], 0x96e61c, 1, &CQM3_POLONIA, 0xa786b8, 0x51);
    *((this + 0x13) as *mut u8) = 5;
    *((this + 0x14) as *mut u8) = 5;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm3_comparar`).
unsafe extern "thiscall" fn comparar_regras_polonia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1814);
    let mut r = 0usize;
    cqm3_comparar("regras_polonia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_polonia(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Finlândia: uma janela, de 16 de novembro a 15 de agosto.
const CQM3_FINLANDIA: [[u8; 6]; 2] = [
    [0x08, 0x00, 0xff, 0x10, 0x0a, 0x01],
    [0x08, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqm3: a Finlândia
];

/// { 0x5a9950 } (thiscall, ret 0x14, finland_rules.cpp, `FINLAND_RULES::FINLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Finlândia (regra 8): a base (0x8e84a0), a vtable 0x96a75c, 1 janela(s) de `CQM3_FINLANDIA`; o dword +0x15 = 1. Devolve o objeto.
unsafe extern "thiscall" fn regras_finlandia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 finlandia");
    cqm3_construir(this, [a1, a2, a3, a4, a5], 0x96a75c, 1, &CQM3_FINLANDIA, 0x9cc134, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm3_comparar`).
unsafe extern "thiscall" fn comparar_regras_finlandia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1815);
    let mut r = 0usize;
    cqm3_comparar("regras_finlandia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_finlandia(s, a1, a2, a3, a4, a5);
    });
    r
}

// ---------- 277a leva: as regras de transferência da Argentina, de Portugal e da Coreia do Sul: as janelas de transferência (argentina_rules.cpp, korea_rules.cpp, portugal_rules.cpp) ----------

unsafe fn cqn1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0 domingo ... 6 sábado, 0xff nenhum) [inferência], +3 o dia, +4 o
/// mês (0 = janeiro), +5 1 no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória,
/// a caixa da linha `linha` do fonte `arquivo`).
unsafe fn cqn1_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqn1_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqn1_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqn1_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqn1_d(this + 4) as usize, cqn1_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Argentina: duas janelas, de 24 de julho (andando para o domingo [inferência]) a 6 de agosto (andando para a quarta-feira [inferência]); de 14 de dezembro (andando para o domingo [inferência]) a 4 de março (andando para a quarta-feira [inferência]).
const CQN1_ARGENTINA: [[u8; 6]; 4] = [
    [0x01, 0x00, 0x00, 0x18, 0x06, 0x01],
    [0x01, 0x00, 0x03, 0x06, 0x07, 0x00], // cqn1: a Argentina
    [0x01, 0x01, 0x00, 0x0e, 0x0b, 0x01],
    [0x01, 0x01, 0x03, 0x04, 0x02, 0x00],
];

/// { 0x40a620 } (thiscall, ret 0, argentina_rules.cpp, `ARGENTINA_RULES::init_transfer_windows`; chamada pelo construtor das regras da Argentina (regra 1), depois da base e da vtable): as
/// 2 janelas de transferência de `CQN1_ARGENTINA`.
unsafe extern "thiscall" fn janelas_argentina(this: usize) {
    crate::comparacao::ramo("cqn_1 argentina");
    cqn1_janelas(this, 2, &CQN1_ARGENTINA, 0x987960, 0x50);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn1_comparar`).
unsafe extern "thiscall" fn comparar_janelas_argentina(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1816);
    cqn1_comparar("janelas_argentina", this, || o(this), |s| janelas_argentina(s));
}

/// As janelas de Portugal: duas janelas, de 1º de junho a 1º de setembro; de 15 de dezembro a 15 de janeiro.
const CQN1_PORTUGAL: [[u8; 6]; 4] = [
    [0x13, 0x00, 0xff, 0x01, 0x05, 0x01],
    [0x13, 0x00, 0xff, 0x01, 0x08, 0x00], // cqn1: a Portugal
    [0x13, 0x01, 0xff, 0x0f, 0x0b, 0x01],
    [0x13, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// { 0x7d3df0 } (thiscall, ret 0, portugal_rules.cpp, `PORTUGAL_RULES::init_transfer_windows`; chamada pelo construtor das regras de Portugal (regra 19), depois da base e da vtable): as
/// 2 janelas de transferência de `CQN1_PORTUGAL`.
unsafe extern "thiscall" fn janelas_portugal(this: usize) {
    crate::comparacao::ramo("cqn_1 portugal");
    cqn1_janelas(this, 2, &CQN1_PORTUGAL, 0xa78844, 0x4f);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn1_comparar`).
unsafe extern "thiscall" fn comparar_janelas_portugal(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1817);
    cqn1_comparar("janelas_portugal", this, || o(this), |s| janelas_portugal(s));
}

/// As janelas da Coreia do Sul: uma janela, de 25 de novembro (andando para o domingo [inferência]) a 25 de julho.
const CQN1_COREIA: [[u8; 6]; 2] = [
    [0x1b, 0x00, 0x00, 0x19, 0x0a, 0x01],
    [0x1b, 0x00, 0xff, 0x19, 0x06, 0x00], // cqn1: a Coreia do Sul
];

/// { 0x66e630 } (thiscall, ret 0x14, korea_rules.cpp, `KOREA_RULES::KOREA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Coreia do Sul (regra 27): a base (0x8e84a0), a vtable 0x96cfec, 1 janela(s) de `CQN1_COREIA`; o +0x13 = 7 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
unsafe extern "thiscall" fn regras_coreia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqn_1 coreia");
    cqn1_construir(this, [a1, a2, a3, a4, a5], 0x96cfec, 1, &CQN1_COREIA, 0x9eb20c, 0x51);
    *((this + 0x13) as *mut u8) = 7;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn1_comparar`).
unsafe extern "thiscall" fn comparar_regras_coreia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1818);
    let mut r = 0usize;
    cqn1_comparar("regras_coreia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_coreia(s, a1, a2, a3, a4, a5);
    });
    r
}

// ---------- 277a leva: as regras de transferência da Grécia, da Rússia e da Bélgica: as janelas de transferência (belgium_rules.cpp, greece_rules.cpp, russia_rules.cpp) ----------

unsafe fn cqn2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0 domingo ... 6 sábado, 0xff nenhum) [inferência], +3 o dia, +4 o
/// mês (0 = janeiro), +5 1 no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória,
/// a caixa da linha `linha` do fonte `arquivo`).
unsafe fn cqn2_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqn2_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqn2_d(this + 4) as usize, cqn2_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Grécia: duas janelas, de 22 de maio (andando para o domingo [inferência]) a 29 de agosto (andando para o sábado [inferência]); de 1º de janeiro a 20 de janeiro.
const CQN2_GRECIA: [[u8; 6]; 4] = [
    [0x0b, 0x00, 0x00, 0x16, 0x04, 0x01],
    [0x0b, 0x00, 0x06, 0x1d, 0x07, 0x00], // cqn2: a Grécia
    [0x0b, 0x01, 0xff, 0x01, 0x00, 0x01],
    [0x0b, 0x01, 0xff, 0x14, 0x00, 0x00],
];

/// { 0x5ecf20 } (thiscall, ret 0, greece_rules.cpp, `GREECE_RULES::init_transfer_windows`; chamada pelo construtor das regras da Grécia (regra 11), depois da base e da vtable): as
/// 2 janelas de transferência de `CQN2_GRECIA`.
unsafe extern "thiscall" fn janelas_grecia(this: usize) {
    crate::comparacao::ramo("cqn_2 grecia");
    cqn2_janelas(this, 2, &CQN2_GRECIA, 0x9cdbc8, 0x51);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn2_comparar`).
unsafe extern "thiscall" fn comparar_janelas_grecia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1819);
    cqn2_comparar("janelas_grecia", this, || o(this), |s| janelas_grecia(s));
}

/// As janelas da Rússia: duas janelas, de 15 de dezembro a 24 de março (andando para a quinta-feira [inferência]); de 28 de junho (andando para a terça-feira [inferência]) a 4 de agosto.
const CQN2_RUSSIA: [[u8; 6]; 4] = [
    [0x14, 0x00, 0xff, 0x0f, 0x0b, 0x01],
    [0x14, 0x00, 0x04, 0x18, 0x02, 0x00],
    [0x14, 0x01, 0x02, 0x1c, 0x05, 0x01],
    [0x14, 0x01, 0xff, 0x04, 0x07, 0x00], // cqn2: a Rússia
];

/// { 0x7ec7e0 } (thiscall, ret 0, russia_rules.cpp, `RUSSIA_RULES::init_transfer_windows`; chamada pelo construtor das regras da Rússia (regra 20), depois da base e da vtable): as
/// 2 janelas de transferência de `CQN2_RUSSIA`.
unsafe extern "thiscall" fn janelas_russia(this: usize) {
    crate::comparacao::ramo("cqn_2 russia");
    cqn2_janelas(this, 2, &CQN2_RUSSIA, 0xa7ad38, 0x4d);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn2_comparar`).
unsafe extern "thiscall" fn comparar_janelas_russia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1820);
    cqn2_comparar("janelas_russia", this, || o(this), |s| janelas_russia(s));
}

/// As janelas da Bélgica: três janelas, de 1º de junho a 30 de junho; de 1º de junho a 31 de dezembro; de 1º de junho a 31 de março.
const CQN2_BELGICA: [[u8; 6]; 6] = [
    [0x03, 0x00, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x00, 0xff, 0x1e, 0x05, 0x00],
    [0x03, 0x01, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x01, 0xff, 0x1f, 0x0b, 0x00], // cqn2: a Bélgica
    [0x03, 0x02, 0xff, 0x01, 0x05, 0x01],
    [0x03, 0x02, 0xff, 0x1f, 0x02, 0x00],
];

/// { 0x424c00 } (thiscall, ret 0, belgium_rules.cpp, `BELGIUM_RULES::init_transfer_windows`; chamada pelo construtor das regras da Bélgica (regra 3), depois da base e da vtable): as
/// 3 janelas de transferência de `CQN2_BELGICA`.
unsafe extern "thiscall" fn janelas_belgica(this: usize) {
    crate::comparacao::ramo("cqn_2 belgica");
    cqn2_janelas(this, 3, &CQN2_BELGICA, 0x989e34, 0x53);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn2_comparar`).
unsafe extern "thiscall" fn comparar_janelas_belgica(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1821);
    cqn2_comparar("janelas_belgica", this, || o(this), |s| janelas_belgica(s));
}

// ---------- 277a leva: a inscrição das três copas europeias, a tabela inicial do ranking da FIFA e a lista dos estádios neutros (fifa_rankings.cpp, squad_manager.cpp, stadium.cpp) ----------

unsafe fn cqn3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As três copas europeias da inscrição, na ordem: a Liga dos Campeões, a Copa da UEFA e a Recopa [inferência] (as
/// chaves dos ids das competições).
const CQN3_COPAS: [usize; 3] = [0x9cf6e8, 0x9cf6f0, 0x9cf6f8];

/// Os clubes da competição `comp` pelo método +0x58 dela (thiscall(comp, &lista, etapa), devolve a conta em ax); a
/// lista alocada é solta pela 0x9452ca depois de `cada` passar por todos.
unsafe fn cqn3_clubes(comp: usize, etapa: u32, mut cada: impl FnMut(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let metodo: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(cqn3_d(cqn3_d(comp) as usize + 0x58) as usize);
    let mut lista = 0usize;
    let n = metodo(comp, &mut lista, etapa) as i16 as i32;
    for i in 0..n.max(0) as usize {
        cada(cqn3_d(lista + 4 * i) as usize);
    }
    if lista != 0 {
        soltar(lista);
    }
}

/// { 0x85a9b0 } (thiscall, ret 4, squad_manager.cpp, `SQUAD_MANAGER::process_euro_registration`): a etapa `etapa`
/// (byte) da inscrição nas copas europeias. Na etapa 1, o ano das inscrições (+0, word) fica o ano do jogo e a marca de
/// inscrição (+0x6d) de todas as pessoas com contrato (registros de 0x6e bytes em [0xae23bc]) é zerada. Depois, em cada
/// uma das três copas (`CQN3_COPAS`; a competição pela tabela [0xadadfc]), cada clube da lista (o método +0x58 com a
/// etapa) que está nela (+0x1db = o id): com técnico humano (0x7875e0, ecx = [0xb7431c]) recebe a notícia da inscrição
/// (0x85b020(this, clube, id, etapa), original); senão a inscrição é feita (0x85aae0(clube, etapa, 0), com ecx =
/// [0xdd7ef0]).
unsafe extern "thiscall" fn inscricao_europeia(this: usize, etapa: u32) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x85b020usize);
    let inscrever: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8 = core::mem::transmute(0x85aae0usize);
    if etapa as u8 == 1 {
        *(this as *mut u16) = *(0xae2c92 as *const u16);
        let mut i = 0i32;
        while i < cqn3_d(0xae2364) as i32 {
            let m = (cqn3_d(0xae23bc) as usize + 0x6e * i as usize + 0x6d) as *mut u8;
            if *m != 0 {
                *m = 0;
            }
            i += 1;
        }
    }
    for &chave in CQN3_COPAS.iter() {
        let id = cqn3_d(chave);
        let comp = cqn3_d(cqn3_d(0xadadfc) as usize + 4 * id as usize) as usize;
        if comp == 0 {
            continue;
        }
        cqn3_clubes(comp, etapa, |c| {
            if c == 0 || cqn3_d(c + 0x1db) != id {
                return;
            }
            if humano(cqn3_d(0xb7431c) as usize, c) != 0 {
                noticia(this, c, id, etapa);
            } else {
                crate::comparacao::ramo("cqn_3 inscreve o clube");
                inscrever(cqn3_d(0xdd7ef0) as usize, c, etapa, 0); // cqn3: inscreve o clube
            }
        });
    }
}

/// Lado a lado: o ano das inscrições (+0), a marca +0x6d de todas as pessoas com contrato (a tabela inteira) e o
/// [0xae1080] da 0x85aae0. Com algum clube das três copas com técnico humano (a notícia não roda duas vezes), só a
/// original.
unsafe extern "thiscall" fn comparar_inscricao_europeia(this: usize, etapa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1822);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let mut tem_humano = false;
    for &chave in CQN3_COPAS.iter() {
        let id = cqn3_d(chave);
        let comp = cqn3_d(cqn3_d(0xadadfc) as usize + 4 * id as usize) as usize;
        if comp != 0 {
            cqn3_clubes(comp, etapa, |c| {
                if c != 0 && cqn3_d(c + 0x1db) == id && humano(cqn3_d(0xb7431c) as usize, c) != 0 {
                    tem_humano = true;
                }
            });
        }
    }
    if tem_humano {
        return o(this, etapa);
    }
    let n = (cqn3_d(0xae2364) as i32).max(0) as usize;
    let reg = [(this, 2usize), (cqn3_d(0xae23bc) as usize, n * 0x6e), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, etapa), || inscricao_europeia(this, etapa));
    PLACAR.registrar(igual, || {
        let base = cqn3_d(0xae23bc) as usize;
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| {
            let a0 = a.0;
            a.1.iter().zip(b.1.iter()).enumerate().filter(|(_, (x, y))| x != y).map(move |(i, (x, y))| (a0 + i, *x, *y))
        }).take(12).map(|(e, x, y)| if e >= base && e < base + n * 0x6e { format!("pessoa {} +{:x}: {}/{}", (e - base) / 0x6e, (e - base) % 0x6e, x, y) } else { format!("{:x}: {}/{}", e, x, y) }).collect();
        format!("inscricao_europeia(etapa {}): {}", etapa as u8, d.join(", "))
    });
}

/// { 0x58dd50 } (thiscall, ret 0, fifa_rankings.cpp, `FIFA_RANKINGS::create_default_table`): a tabela inicial do
/// ranking da FIFA (registros de 0x2c bytes, um por nação, calloc 0x945501 de [0xae2350] × 0x2c em +0; sem memória, a
/// caixa da linha 0x2da). Cada registro: +0 o índice da nação; a força da nação (+0xa8, double, nações de 0x122 bytes em
/// [0xae23a8]) × 1/35 dá os pontos (+0xc, float) e, × 1000 (truncado, _ftol), os 6 dwords +0x14..+0x28 [inferência: os
/// pontos de cada um dos últimos anos]. Depois a ordem pelos pontos (0x58d610(this, &conta), thiscall, a lista ordenada
/// das nações com continente) dá a posição: +0x12 = a posição de antes (+0x10) e +0x10 = 1, 2, 3...; a lista é solta.
unsafe extern "thiscall" fn ranking_fifa_inicial(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let ordem: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(0x58d610usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let t = calloc(cqn3_d(0xae2350), 0x2c);
    *(this as *mut usize) = t;
    if t == 0 {
        aviso_em(0x9c74f4, 0x2da);
        return;
    }
    let c1 = core::ptr::read_unaligned(0x96a348 as *const f64);
    let c2 = core::ptr::read_unaligned(0x968908 as *const f64);
    let mut k = 0i32;
    while k < cqn3_d(0xae2350) as i32 {
        let r = t + 0x2c * k as usize;
        *(r as *mut i32) = k;
        let a = core::ptr::read_unaligned((cqn3_d(0xae23a8) as usize + 0x122 * k as usize + 0xa8) as *const f64) * c1;
        let p = (a * c2) as i32;
        for j in 0..6 {
            *((r + 0x14 + 4 * j) as *mut i32) = p;
        }
        *((r + 0xc) as *mut f32) = a as f32;
        k += 1;
    }
    let mut n = 0i32;
    let l = ordem(this, &mut n);
    for i in 0..n.max(0) as usize {
        let r = cqn3_d(l + 4 * i) as usize;
        *((r + 0x12) as *mut u16) = *((r + 0x10) as *const u16);
        *((r + 0x10) as *mut u16) = (i + 1) as u16;
    }
    soltar(l);
}

/// Lado a lado à mão: a original no objeto de quem chama (ela aloca a tabela em +0), a cópia num objeto de rascunho
/// (aloca a dela); iguais as duas tabelas inteiras (a do rascunho é solta).
unsafe extern "thiscall" fn comparar_ranking_fifa_inicial(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1823);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    o(this);
    let mut s = [0usize; 1];
    ranking_fifa_inicial(s.as_mut_ptr() as usize);
    let (ta, tb) = (cqn3_d(this) as usize, s[0]);
    let n = (cqn3_d(0xae2350) as i32).max(0) as usize * 0x2c;
    let igual = if ta != 0 && tb != 0 {
        core::slice::from_raw_parts(ta as *const u8, n) == core::slice::from_raw_parts(tb as *const u8, n)
    } else {
        ta == tb
    };
    let dif = if igual || ta == 0 || tb == 0 {
        None
    } else {
        (0..n).find(|&i| *((ta + i) as *const u8) != *((tb + i) as *const u8))
    };
    if tb != 0 {
        soltar(tb);
    }
    PLACAR.registrar(igual, || format!("ranking_fifa_inicial: a primeira diferença na nação {:?}", dif.map(|i| (i / 0x2c, i % 0x2c))));
}

/// { 0x85dfc0 } (thiscall, ret 0, stadium.cpp, `Stadium::build_venue_list`): refaz a lista dos estádios para jogos
/// em campo neutro (+0xc a lista de registros de 16 bytes, +0x10 a conta, +0x14 a capacidade; a velha é solta). As
/// sedes das próximas edições de três competições (as chaves 0x9cf79c, 0x9cf7a4, 0x9cf7ac; 0x5fa7d0 no HostCountry
/// [0xb5e134], sem ele a caixa da linha 0x858) são achadas, mas não mudam nada (o laço delas termina igual nos dois
/// caminhos). Entram os estádios (registros de 0x4e bytes em [0xae23b0], conta [0xae2358]) com cidade (+0x38), nação da
/// cidade (+0x1f) e continente da nação (+0x71), fora o "City of Manchester Stadium" (ainda em obras em 2001) e, nas
/// nações com competições (+0x11c), os de menos de 5000 lugares (+0x3c). Cada registro: +0 o id da nação, +4 o id do
/// continente, +8 se é o estádio nacional (+0x80 da nação), +0xc o estádio. A lista cresce de 30 em 30 (0x944e46,
/// realloc 0x945667; sem memória, as caixas das linhas 0x89c e 0x8ac, e para). No fim é ordenada (qsort com a
/// 0x85e2f0) e +0x18 = 1.
unsafe extern "thiscall" fn lista_de_estadios_neutros(this: usize) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sedes: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u8 = core::mem::transmute(0x5fa7d0usize);
    let (lista, conta, cap) = ((this + 0xc) as *mut usize, (this + 0x10) as *mut i32, (this + 0x14) as *mut i32);
    if *lista != 0 {
        soltar(*lista);
        *lista = 0;
        *cap = 0;
        *conta = 0;
    }
    let hc = cqn3_d(0xb5e134) as usize;
    if hc == 0 {
        aviso_em(0xa835b0, 0x858);
    } else {
        // as sedes: só achadas (a original não as usa depois); as saídas se sobrepõem como na original
        let mut s = [0u32; 8];
        let mut b = sedes(hc, cqn3_d(0x9cf79c), s.as_mut_ptr(), s.as_mut_ptr().add(1)) as i8;
        b = b.wrapping_add(sedes(hc, cqn3_d(0x9cf7a4), s.as_mut_ptr().add(b as usize), s.as_mut_ptr().add(b as usize + 1)) as i8);
        sedes(hc, cqn3_d(0x9cf7ac), s.as_mut_ptr().add(b as usize), s.as_mut_ptr().add(b as usize + 1));
    }
    let mut k = 0i32;
    while k < cqn3_d(0xae2358) as i32 {
        let e = cqn3_d(0xae23b0) as usize + 0x4e * k as usize;
        k = (k as i16).wrapping_add(1) as i32;
        let cidade = cqn3_d(e + 0x38) as usize;
        if cidade == 0 {
            continue;
        }
        let nacao = cqn3_d(cidade + 0x1f) as usize;
        if nacao == 0 || cqn3_d(nacao + 0x71) == 0 {
            continue;
        }
        if core::ffi::CStr::from_ptr((e + 4) as *const core::ffi::c_char).to_bytes() == b"City of Manchester Stadium" {
            continue;
        }
        if *((nacao + 0x11c) as *const u8) != 0 && (cqn3_d(e + 0x3c) as i32) < 0x1388 { // cqn3: menos de 5000 lugares
            crate::comparacao::ramo("cqn_3 estadio pequeno");
            continue; // cqn3: o estádio pequeno fica de fora
        }
        if *lista == 0 {
            *lista = alocar(0x1e0);
            if *lista == 0 {
                aviso_em(0xa835b0, 0x89c);
                return;
            }
            *cap = 0x1e;
        } else if *conta == *cap {
            *lista = realocar(*lista, ((*cap + 0x1e) << 4) as u32);
            if *lista == 0 {
                aviso_em(0xa835b0, 0x8ac);
                return;
            }
            *cap += 0x1e;
        }
        let r = *lista + 16 * *conta as usize;
        *(r as *mut u32) = cqn3_d(nacao);
        *((r + 4) as *mut u32) = cqn3_d(cqn3_d(nacao + 0x71) as usize);
        *((r + 0xc) as *mut usize) = e;
        crate::comparacao::ramo("cqn_3 entra na lista");
        *((r + 8) as *mut u32) = (cqn3_d(nacao + 0x80) as usize == e) as u32; // cqn3: o estádio nacional
        *conta += 1;
    }
    ordenar(*lista, *conta as u32, 0x10, 0x85e2f0);
    *((this + 0x18) as *mut u8) = 1;
}

/// Lado a lado à mão: a original no objeto de quem chama, a cópia num rascunho com os mesmos 0x20 bytes de antes e a
/// lista velha zerada (a original já a soltou); iguais os campos (fora o ponteiro +0xc) e as listas (a do rascunho é
/// solta). Sem o HostCountry (a caixa), só a original.
unsafe extern "thiscall" fn comparar_lista_de_estadios_neutros(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1824);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqn3_d(0xb5e134) == 0 {
        return o(this);
    }
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x20);
    s[0xc..0x18].fill(0);
    o(this);
    let r = s.as_mut_ptr() as usize;
    lista_de_estadios_neutros(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x20);
    let campos = a[..0xc] == s[..0xc] && a[0x10..0x20] == s[0x10..0x20];
    let (la, lb) = (cqn3_d(this + 0xc) as usize, cqn3_d(r + 0xc) as usize);
    let n = (cqn3_d(this + 0x10) as i32).max(0) as usize * 16;
    let listas = if la != 0 && lb != 0 {
        core::slice::from_raw_parts(la as *const u8, n) == core::slice::from_raw_parts(lb as *const u8, n)
    } else {
        la == lb
    };
    if lb != 0 {
        soltar(lb);
    }
    PLACAR.registrar(campos && listas, || format!("lista_de_estadios_neutros: campos {} listas {} (contas {} e {})", campos, listas, cqn3_d(this + 0x10) as i32, cqn3_d(r + 0x10) as i32));
}

// ---------- 277a leva: a tabela das três divisões nacionais do Brasil: os 28 clubes da Série A (e o dinheiro da TV), os 28 da Série B e o sorteio dos 56 da Série C (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------

unsafe fn cql7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cql7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube está na competição do global `g`: o ponteiro do clube em `campo` (+0x57 a divisão nacional) não é nulo e o
/// id do registro (+0) é o do global.
unsafe fn cql7_da_divisao(clube: usize, campo: usize, g: usize) -> bool {
    let comp = cql7_d(clube + campo);
    comp != 0 && cql7_d(comp) == cql7_d(g)
}

/// O corpo comum da Série A e da Série B (o mesmo molde dos estaduais): +0x3e = `n` clubes; a tabela (malloc 0x944e46
/// de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245
/// bytes, [0xae235c] deles) da competição do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada divisão.
unsafe fn cql7_tabela(this: usize, n: i16, arquivo: usize, linha: u32, campo: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cql7_d(0xae23b4);
    while (i as i32) < cql7_d(0xae235c) as i32 {
        if cql7_da_divisao(clube, campo, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cql7_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cql7_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x42a480 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_table()`: a Série A tem 28 clubes (os de
/// divisão nacional +0x57 = [0x9cf9ec]); cheia, a TV paga 150.000 a cada clube (League::pay_tv_rights 0x683010:
/// push 0; push 0x249f0; mov ecx, edi -> thiscall(liga, valor, filtro), ret 8; manda a notícia da TV) e devolve 1.
/// Fonte 0x98aa30, linha 0x1a1.
unsafe extern "thiscall" fn cql7_tabela_da_serie_a(this: usize) -> u32 {
    if cql7_tabela(this, 0x1c, 0x98aa30, 0x1a1, 0x57, 0x9cf9ec, 0, "cql_7 serie a cheia") == 0 {
        return 0;
    }
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, 0x249f0, 0);
    1
}

/// { 0x42ca70 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::create_table()`: a Série B tem 28 clubes (+0x57 =
/// [0x9cf9e0]), sem dinheiro da TV. Fonte 0x98ab64, linha 0x1aa.
unsafe extern "thiscall" fn cql7_tabela_da_serie_b(this: usize) -> u32 {
    cql7_tabela(this, 0x1c, 0x98ab64, 0x1aa, 0x57, 0x9cf9e0, 0, "cql_7 serie b cheia")
}

/// { 0x42dc90 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::create_table()`: +0x3e = 7; a lista dos
/// participantes (+0xee) refeita (a velha liberada, 0x9452ca push; add esp -> cdecl; a nova com 0xe0 bytes = 56
/// ponteiros; sem memória: a caixa da linha 0x18f e devolve 1, como a original); a tabela (+0xb1, 7 × 0x3b; sem
/// memória: a caixa da linha 0x196 e 0); a lista recebe, na ordem do banco, até 56 clubes cuja divisão (+0x57) é a
/// própria Série C (o registro em [this+4]); a lista é embaralhada (comp_util_shuffle_pointer_array 0x4afb40: push
/// conta (word); push lista; add esp, 8 -> cdecl; 3 × conta trocas sorteadas) e os 7 primeiros do sorteio entram nas
/// linhas 0..6 (0x687430, a linha num dword limpo). Fonte 0x98aba4. Devolve 1.
unsafe extern "thiscall" fn cql7_tabela_da_serie_c(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let embaralhar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4afb40usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 7);
    let velha = cql7_d(this + 0xee);
    if velha != 0 {
        soltar(velha);
    }
    let lista = alocar(0xe0);
    core::ptr::write_unaligned((this + 0xee) as *mut usize, lista);
    if lista == 0 {
        aviso_em(0x98aba4, 0x18f);
        return 1;
    }
    let t = alocar((cql7_w(this + 0x3e) as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(0x98aba4, 0x196);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut clube = cql7_d(0xae23b4);
    while i < cql7_d(0xae235c) as i32 {
        if (k as i8) >= 0x38 {
            break;
        }
        if cql7_d(clube + 0x57) == cql7_d(this + 4) {
            core::ptr::write_unaligned((cql7_d(this + 0xee) + (k as i8 as isize as usize) * 4) as *mut usize, clube);
            k = k.wrapping_add(1);
        }
        i += 1;
        clube = clube.wrapping_add(0x245);
    }
    crate::comparacao::ramo("cql_7 serie c sorteio");
    embaralhar(cql7_d(this + 0xee), k as i8 as i32); // cql7: o sorteio dos clubes da Série C
    for j in 0..7usize {
        iniciar(this, j as u32, cql7_d(cql7_d(this + 0xee) + j * 4), 0, 0);
    }
    1
}

/// Os clubes da competição do global `g` (ponteiro em `campo`), na ordem do banco, até `n`.
unsafe fn cql7_plano(n: usize, campo: usize, g: usize) -> Vec<usize> {
    let mut plano = Vec::new();
    let mut i: i32 = 0;
    while i < cql7_d(0xae235c) as i32 && plano.len() < n {
        let clube = cql7_d(0xae23b4) + (i as usize) * 0x245;
        if cql7_da_divisao(clube, campo, g) {
            plano.push(clube);
        }
        i += 1;
    }
    plano
}

/// O comparador da Série A e da Série B: a original aloca a tabela (e na A paga a TV com notícia): só ela. O plano,
/// feito antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano
/// encheu (senão 0) e as linhas da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cql7_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let plano = cql7_plano(n as usize, 0x57, g);
    let r = original();
    let t = cql7_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cql7_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cql7_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cql7_tabela_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1825);
    cql7_comparar("cql7_tabela_da_serie_a", this, 0x1c, 0x9cf9ec, || o(this))
}

unsafe extern "thiscall" fn comparar_cql7_tabela_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1826);
    cql7_comparar("cql7_tabela_da_serie_b", this, 0x1c, 0x9cf9e0, || o(this))
}

/// O comparador da Série C: a original libera e aloca e sorteia: só ela. O plano, feito antes: os clubes cuja divisão é
/// [this+4], na ordem do banco, até 56. Depois: a devolução 1, +0x3e = 7, a lista (+0xee) com os clubes do plano em
/// alguma ordem (o sorteio) e as linhas 0..6 da tabela com os 7 primeiros da lista.
unsafe extern "thiscall" fn comparar_cql7_tabela_da_serie_c(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1827);
    let mut plano = Vec::new();
    let mut i: i32 = 0;
    while i < cql7_d(0xae235c) as i32 && plano.len() < 0x38 {
        let clube = cql7_d(0xae23b4) + (i as usize) * 0x245;
        if cql7_d(clube + 0x57) == cql7_d(this + 4) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = o(this);
    let (lista, t) = (cql7_d(this + 0xee), cql7_d(this + 0xb1));
    let mut igual = r == 1 && cql7_w(this + 0x3e) == 7 && lista != 0 && t != 0 && plano.len() >= 7;
    let mut sorteados: Vec<usize> = Vec::new();
    if igual {
        sorteados = (0..plano.len()).map(|k| cql7_d(lista + k * 4)).collect();
        let linhas_ok = (0..7usize).all(|j| cql7_d(t + j * 0x3b) == sorteados[j]);
        let (mut a, mut b) = (sorteados.clone(), plano.clone());
        a.sort_unstable();
        b.sort_unstable();
        igual = linhas_ok && a == b;
    }
    PLACAR.registrar(igual, || format!("cql7_tabela_da_serie_c({:#x}) original={} plano={:x?} lista={:x?}", this, r, plano, sorteados));
    r
}

// ---------- 277a leva: a tabela dos estaduais: Baiano, Centro-Oeste e Gaúcho (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cql8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cql8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cql8_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cql8_d(clube + 0x60);
    comp != 0 && cql8_d(comp) == cql8_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cql8_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cql8_d(0xae23b4);
    while (i as i32) < cql8_d(0xae235c) as i32 {
        if cql8_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cql8_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cql8_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x42fd10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_table()`: o Baiano tem 10 clubes (divisão reserva +0x60 = [0x9cf9c0]).
/// Fonte 0x98acac, linha 0x167.
unsafe extern "thiscall" fn cql8_tabela_do_baiano(this: usize) -> u32 {
    cql8_tabela(this, 0xa, 0x98acac, 0x167, 0x9cf9c0, 0, "cql_8 baiano cheio")
}

/// { 0x430ef0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::load_region_clubs()`: o Centro-Oeste tem 15 clubes (divisão reserva +0x60 = [0x9cf998]).
/// Fonte 0x98ae10, linha 0x168.
unsafe extern "thiscall" fn cql8_tabela_do_centro_oeste(this: usize) -> u32 {
    cql8_tabela(this, 0xf, 0x98ae10, 0x168, 0x9cf998, 0, "cql_8 centro_oeste cheio")
}

/// { 0x4321d0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::load_region_clubs()`: o Gaúcho tem 17 clubes (divisão reserva +0x60 = [0x9cf9c8]).
/// Fonte 0x98ae50, linha 0x163.
unsafe extern "thiscall" fn cql8_tabela_do_gaucho(this: usize) -> u32 {
    cql8_tabela(this, 0x11, 0x98ae50, 0x163, 0x9cf9c8, 0, "cql_8 gaucho cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cql8_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cql8_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cql8_d(0xae23b4) + (i as usize) * 0x245;
        if cql8_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cql8_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cql8_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cql8_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cql8_tabela_do_baiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1828);
    cql8_comparar("cql8_tabela_do_baiano", this, 0xa, 0x9cf9c0, || o(this))
}

unsafe extern "thiscall" fn comparar_cql8_tabela_do_centro_oeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1829);
    cql8_comparar("cql8_tabela_do_centro_oeste", this, 0xf, 0x9cf998, || o(this))
}

unsafe extern "thiscall" fn comparar_cql8_tabela_do_gaucho(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1830);
    cql8_comparar("cql8_tabela_do_gaucho", this, 0x11, 0x9cf9c8, || o(this))
}

// ---------- 277a leva: a tabela dos estaduais: Goiano, Mineiro e Norte (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------

unsafe fn cql9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cql9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cql9_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cql9_d(clube + 0x60);
    comp != 0 && cql9_d(comp) == cql9_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cql9_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cql9_d(0xae23b4);
    while (i as i32) < cql9_d(0xae235c) as i32 {
        if cql9_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cql9_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cql9_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x433540 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_table()`: o Goiano tem 14 clubes (divisão reserva +0x60 = [0x9cf9b4]).
/// Fonte 0x98ae90, linha 0x160.
unsafe extern "thiscall" fn cql9_tabela_do_goiano(this: usize) -> u32 {
    cql9_tabela(this, 0xe, 0x98ae90, 0x160, 0x9cf9b4, 0, "cql_9 goiano cheio")
}

/// { 0x434810 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_table()`: o Mineiro tem 12 clubes (divisão reserva +0x60 = [0x9cf9d4]).
/// Fonte 0x98aed0, linha 0x169.
unsafe extern "thiscall" fn cql9_tabela_do_mineiro(this: usize) -> u32 {
    cql9_tabela(this, 0xc, 0x98aed0, 0x169, 0x9cf9d4, 0, "cql_9 mineiro cheio")
}

/// { 0x435d80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_table()`: o Norte tem 10 clubes (divisão reserva +0x60 = [0x9cf994]).
/// Fonte 0x98af14, linha 0x167.
unsafe extern "thiscall" fn cql9_tabela_do_norte(this: usize) -> u32 {
    cql9_tabela(this, 0xa, 0x98af14, 0x167, 0x9cf994, 0, "cql_9 norte cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cql9_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cql9_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cql9_d(0xae23b4) + (i as usize) * 0x245;
        if cql9_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cql9_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cql9_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cql9_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cql9_tabela_do_goiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1831);
    cql9_comparar("cql9_tabela_do_goiano", this, 0xe, 0x9cf9b4, || o(this))
}

unsafe extern "thiscall" fn comparar_cql9_tabela_do_mineiro(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1832);
    cql9_comparar("cql9_tabela_do_mineiro", this, 0xc, 0x9cf9d4, || o(this))
}

unsafe extern "thiscall" fn comparar_cql9_tabela_do_norte(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1833);
    cql9_comparar("cql9_tabela_do_norte", this, 0xa, 0x9cf994, || o(this))
}

// ---------- 277a leva: a tabela dos estaduais: Nordeste, Paranaense e Pernambucano (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------

unsafe fn cqm7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqm7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cqm7_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cqm7_d(clube + 0x60);
    comp != 0 && cqm7_d(comp) == cqm7_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cqm7_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm7_d(0xae23b4);
    while (i as i32) < cqm7_d(0xae235c) as i32 {
        if cqm7_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqm7_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqm7_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x436d90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_table()`: o Nordeste tem 16 clubes (divisão reserva +0x60 = [0x9cf9a4]).
/// Fonte 0x98af54, linha 0x159.
unsafe extern "thiscall" fn cqm7_tabela_do_nordeste(this: usize) -> u32 {
    cqm7_tabela(this, 0x10, 0x98af54, 0x159, 0x9cf9a4, 0, "cqm_7 nordeste cheio")
}

/// { 0x437ec0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::create_table()`: o Paranaense tem 12 clubes (divisão reserva +0x60 = [0x9cf9d0]).
/// Fonte 0x98af98, linha 0x169.
unsafe extern "thiscall" fn cqm7_tabela_do_paranaense(this: usize) -> u32 {
    cqm7_tabela(this, 0xc, 0x98af98, 0x169, 0x9cf9d0, 0, "cqm_7 paranaense cheio")
}

/// { 0x439330 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_table()`: o Pernambucano tem 10 clubes (divisão reserva +0x60 = [0x9cf9bc]).
/// Fonte 0x98afd8, linha 0x167.
unsafe extern "thiscall" fn cqm7_tabela_do_pernambucano(this: usize) -> u32 {
    cqm7_tabela(this, 0xa, 0x98afd8, 0x167, 0x9cf9bc, 0, "cqm_7 pernambucano cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cqm7_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqm7_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqm7_d(0xae23b4) + (i as usize) * 0x245;
        if cqm7_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqm7_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cqm7_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqm7_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cqm7_tabela_do_nordeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1834);
    cqm7_comparar("cqm7_tabela_do_nordeste", this, 0x10, 0x9cf9a4, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm7_tabela_do_paranaense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1835);
    cqm7_comparar("cqm7_tabela_do_paranaense", this, 0xc, 0x9cf9d0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm7_tabela_do_pernambucano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1836);
    cqm7_comparar("cqm7_tabela_do_pernambucano", this, 0xa, 0x9cf9bc, || o(this))
}

// ---------- 277a leva: a tabela dos estaduais: Carioca, Catarinense e Paulista (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqm8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqm8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cqm8_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cqm8_d(clube + 0x60);
    comp != 0 && cqm8_d(comp) == cqm8_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cqm8_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm8_d(0xae23b4);
    while (i as i32) < cqm8_d(0xae235c) as i32 {
        if cqm8_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqm8_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqm8_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x43a500 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_table()`: o Carioca tem 12 clubes (divisão reserva +0x60 = [0x9cf9dc]).
/// Fonte 0x98b014, linha 0x169.
unsafe extern "thiscall" fn cqm8_tabela_do_carioca(this: usize) -> u32 {
    cqm8_tabela(this, 0xc, 0x98b014, 0x169, 0x9cf9dc, 0, "cqm_8 carioca cheio")
}

/// { 0x43bb70 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_table()`: o Catarinense tem 10 clubes (divisão reserva +0x60 = [0x9cf9b0]).
/// Fonte 0x98b050, linha 0x167.
unsafe extern "thiscall" fn cqm8_tabela_do_catarinense(this: usize) -> u32 {
    cqm8_tabela(this, 0xa, 0x98b050, 0x167, 0x9cf9b0, 0, "cqm_8 catarinense cheio")
}

/// { 0x43cc40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_table()`: o Paulista tem 16 clubes (divisão reserva +0x60 = [0x9cf9e4]).
/// Fonte 0x98b090, linha 0x159.
unsafe extern "thiscall" fn cqm8_tabela_do_paulista(this: usize) -> u32 {
    cqm8_tabela(this, 0x10, 0x98b090, 0x159, 0x9cf9e4, 0, "cqm_8 paulista cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cqm8_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqm8_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqm8_d(0xae23b4) + (i as usize) * 0x245;
        if cqm8_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqm8_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cqm8_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqm8_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cqm8_tabela_do_carioca(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1837);
    cqm8_comparar("cqm8_tabela_do_carioca", this, 0xc, 0x9cf9dc, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm8_tabela_do_catarinense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1838);
    cqm8_comparar("cqm8_tabela_do_catarinense", this, 0xa, 0x9cf9b0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm8_tabela_do_paulista(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1839);
    cqm8_comparar("cqm8_tabela_do_paulista", this, 0x10, 0x9cf9e4, || o(this))
}

// ---------- 277a leva: a tabela das ligas do Japão e da Austrália: os 16 clubes da J1 (o fundo de prêmios e a TV), os 12 da J2 (os que sobram saem da divisão) e os 13 da NSL (jap_j1.cpp, jap_j2.cpp, aus_nsl.cpp) ----------

unsafe fn cqm9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqm9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é da divisão do global `g`: a divisão do clube (+0x57) não é nula e o id do registro (+0) é o do global.
unsafe fn cqm9_da_divisao(clube: usize, g: usize) -> bool {
    let comp = cqm9_d(clube + 0x57);
    comp != 0 && cqm9_d(comp) == cqm9_d(g)
}

/// +0x3e = `n` e a tabela (malloc 0x944e46 de n × 0x3b, em +0xb1; sem memória: a caixa do fonte `arquivo` na linha
/// `linha`). Devolve a tabela (0 sem memória).
unsafe fn cqm9_alocar(this: usize, n: i16, arquivo: usize, linha: u32) -> usize {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
    }
    t
}

/// League::init_table_row (0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10). `girar` é 0 (o original: a linha
/// k); existe só para as mutações de cada liga. ponytail: a original empurra a linha no eax com bits de cima de sobra;
/// a 0x687430 só lê o byte.
unsafe fn cqm9_linha(this: usize, k: u8, n: i16, girar: u8, clube: usize) {
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    iniciar(this, (k as u32 + girar as u32) % n as u32, clube, 0, 0);
}

/// O laço do Japão: percorre todos os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles); cada clube da divisão do
/// global `g`, na ordem do banco, entra na linha seguinte enquanto houver linha; os que sobram perdem a divisão
/// (+0x57 = 0). Devolve 1 se a tabela encheu.
unsafe fn cqm9_japao(this: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm9_d(0xae23b4);
    while (i as i32) < cqm9_d(0xae235c) as i32 {
        if cqm9_da_divisao(clube, g) {
            if (k as i8 as i16) < cqm9_w(this + 0x3e) {
                cqm9_linha(this, k, cqm9_w(this + 0x3e), girar, clube);
                k = k.wrapping_add(1);
            } else {
                core::ptr::write_unaligned((clube + 0x57) as *mut usize, 0);
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqm9_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x6634e0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_table()`: a J1 tem 16 clubes (+0x57 = [0x9cf65c]; os que
/// sobram perdem a divisão); cheia, o fundo de prêmios com a cota 100.000 (League::calc_prize_fund 0x682f70: push
/// 0x186a0; mov ecx, edi -> thiscall, ret 4) em +0x28, +0xe1 = 1, a TV 3.000.000 a cada clube (League::pay_tv_rights
/// 0x683010: push 0; push 0x2dc6c0; mov ecx, edi -> thiscall, ret 8; manda a notícia da TV) e 1. Fonte 0x9eabc8, linha
/// 0x148.
unsafe extern "thiscall" fn cqm9_tabela_da_j1(this: usize) -> u32 {
    if cqm9_alocar(this, 0x10, 0x9eabc8, 0x148) == 0 {
        return 0;
    }
    if cqm9_japao(this, 0x9cf65c, 0, "cqm_9 j1 cheia") == 0 {
        return 0;
    }
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    let f = premio(this, 0x186a0);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar(this, 0x2dc6c0, 0);
    1
}

/// { 0x665030 } (thiscall, ret 0, jap_j2.cpp): `JapJ2::create_table()`: a J2 tem 12 clubes (+0x57 = [0x9cf768]; os que
/// sobram perdem a divisão), sem fundo de prêmios nem TV. Fonte 0x9eaedc, linha 0x13a. Devolve 1 se encheu.
unsafe extern "thiscall" fn cqm9_tabela_da_j2(this: usize) -> u32 {
    if cqm9_alocar(this, 0xc, 0x9eaedc, 0x13a) == 0 {
        return 0;
    }
    cqm9_japao(this, 0x9cf768, 0, "cqm_9 j2 cheia")
}

/// { 0x411b70 } (thiscall, ret 0, aus_nsl.cpp): `AusNsl::create_table()`: a NSL tem 13 clubes (+0x57 = [0x9cf89c]), na
/// ordem do banco, até encher (o contador num int, comparado no word); sem encher: a caixa da linha 0x252 e 0; cheia: 1,
/// sem fundo de prêmios nem TV. Fonte 0x987c20 (a caixa de memória na linha 0x241).
unsafe extern "thiscall" fn cqm9_tabela_da_nsl(this: usize) -> u32 {
    if cqm9_alocar(this, 0xd, 0x987c20, 0x241) == 0 {
        return 0;
    }
    let mut k: u32 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm9_d(0xae23b4);
    while (i as i32) < cqm9_d(0xae235c) as i32 {
        if (k as i16) >= cqm9_w(this + 0x3e) {
            break;
        }
        if cqm9_da_divisao(clube, 0x9cf89c) {
            cqm9_linha(this, k as u8, cqm9_w(this + 0x3e), 0, clube); // cqm9: a linha da NSL
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i16 != cqm9_w(this + 0x3e) {
        aviso_em(0x987c20, 0x252);
        return 0;
    }
    crate::comparacao::ramo("cqm_9 nsl cheia");
    1
}

/// O comparador dos três: a original aloca a tabela (e a J1 paga a TV com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (todos). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0), as
/// linhas da tabela (+0 o clube, 0x3b bytes) com os primeiros `n` do plano, na ordem; no Japão (`limpar`), os que
/// sobram do plano com a divisão zerada; na J1 cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqm9_comparar(nome: &str, this: usize, n: i16, g: usize, limpar: bool, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqm9_d(0xae235c) as i32 {
        let clube = cqm9_d(0xae23b4) + (i as usize) * 0x245;
        if cqm9_da_divisao(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqm9_d(this + 0xb1);
    let cabem = plano.len().min(n as usize);
    let esperado = (plano.len() >= n as usize) as u32;
    let mut igual = r == esperado && cqm9_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..cabem).map(|k| cqm9_d(t + k * 0x3b)).collect();
        igual = achou[..] == plano[..cabem];
        if limpar {
            igual = igual && plano[cabem..].iter().all(|&c| cqm9_d(c + 0x57) == 0);
        }
        if r == 1 && cota != 0 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cqm9_tabela_da_j1(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1840);
    cqm9_comparar("cqm9_tabela_da_j1", this, 0x10, 0x9cf65c, true, 0x186a0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm9_tabela_da_j2(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1841);
    cqm9_comparar("cqm9_tabela_da_j2", this, 0xc, 0x9cf768, true, 0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm9_tabela_da_nsl(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1842);
    cqm9_comparar("cqm9_tabela_da_nsl", this, 0xd, 0x9cf89c, false, 0, || o(this))
}

// ---------- 277a leva: as semifinais dos estaduais e regionais: Baiano, Norte e Nordeste (bra_reg_bahia.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp) ----------

unsafe fn cqn7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqn7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqn7_clube(liga: usize, l: usize) -> usize {
    cqn7_d(cqn7_d(liga + 0xb1) + l * 0x3b)
}

/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqn7_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn7_clube(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqn7_d(cqn7_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, cqn7_d(this + 4), t, cqn7_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(cqn7_d(this + 0xc) as *mut usize, fase);
    let fase = cqn7_d(cqn7_d(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// { 0x430050 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98acac, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn7_mata_mata_do_baiano(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // cqn7: 1º × 4º, 2º × 3º Baiano
}

/// { 0x4360c0 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af14, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn7_mata_mata_do_norte(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af14, 0x21b, 0x22f, "cqn_7 norte"); // cqn7: 1º × 4º, 2º × 3º Norte
}

/// { 0x4370d0 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af54, linhas 0x20d (sem lista) e 0x221 (sem fase).
unsafe extern "thiscall" fn cqn7_mata_mata_do_nordeste(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
unsafe fn cqn7_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn7_clube(this, l)).collect();
    original();
    let fase = cqn7_d(cqn7_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqn7_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| cqn7_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqn7_w(fase + 0x3e) as usize == linhas.len() && cqn7_d(fase + 4) == cqn7_d(this + 4) && cqn7_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqn7_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_baiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1843);
    cqn7_comparar("cqn7_mata_mata_do_baiano", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_norte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1844);
    cqn7_comparar("cqn7_mata_mata_do_norte", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_nordeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1845);
    cqn7_comparar("cqn7_mata_mata_do_nordeste", this, &[0, 3, 1, 2], || o(this))
}

// ---------- 277a leva: as semifinais dos estaduais e regionais: Pernambucano, Catarinense e Paulista (bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqn8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqn8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqn8_clube(liga: usize, l: usize) -> usize {
    cqn8_d(cqn8_d(liga + 0xb1) + l * 0x3b)
}

/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqn8_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn8_clube(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqn8_d(cqn8_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, cqn8_d(this + 4), t, cqn8_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(cqn8_d(this + 0xc) as *mut usize, fase);
    let fase = cqn8_d(cqn8_d(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// { 0x439710 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98afd8, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn8_mata_mata_do_pernambucano(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // cqn8: 1º × 4º, 2º × 3º Pernambucano
}

/// { 0x43bf50 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b050, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn8_mata_mata_do_catarinense(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // cqn8: 1º × 4º, 2º × 3º Catarinense
}

/// { 0x43cf80 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b090, linhas 0x20d (sem lista) e 0x221 (sem fase).
unsafe extern "thiscall" fn cqn8_mata_mata_do_paulista(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // cqn8: 1º × 4º, 2º × 3º Paulista
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
unsafe fn cqn8_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn8_clube(this, l)).collect();
    original();
    let fase = cqn8_d(cqn8_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqn8_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| cqn8_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqn8_w(fase + 0x3e) as usize == linhas.len() && cqn8_d(fase + 4) == cqn8_d(this + 4) && cqn8_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqn8_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_cqn8_mata_mata_do_pernambucano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1846);
    cqn8_comparar("cqn8_mata_mata_do_pernambucano", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn8_mata_mata_do_catarinense(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1847);
    cqn8_comparar("cqn8_mata_mata_do_catarinense", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn8_mata_mata_do_paulista(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1848);
    cqn8_comparar("cqn8_mata_mata_do_paulista", this, &[0, 3, 1, 2], || o(this))
}
