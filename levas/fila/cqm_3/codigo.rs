// ---------- %%LEVA%%a leva: as regras de transferência da Turquia, da Polônia e da Finlândia: as janelas de transferência (finland_rules.cpp, poland_rules.cpp, turkey_rules.cpp) ----------

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
        show_error_at(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn turkey_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
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
    PLACAR.record(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Turquia: uma janela, de 1º de junho a 31 de janeiro.
const TURKEY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia
    [0x18, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// { 0x9020a0 } (thiscall, ret 0x14, turkey_rules.cpp, `TURKEY_RULES::TURKEY_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Turquia (regra 24): a base (0x8e84a0), a vtable 0x97075c, 1 janela(s) de `TURKEY_TRANSFER_WINDOWS`; o +0x13 = 8 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 8 (o padrão é 30). Devolve o objeto.
unsafe extern "thiscall" fn construct_turkey_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_3 turquia");
    turkey_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x97075c, 1, &TURKEY_TRANSFER_WINDOWS, 0xaa19a8, 0x52);
    *((this + 0x13) as *mut u8) = 8;
    *((this + 0x14) as *mut u8) = 8;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm3_comparar`).
unsafe extern "thiscall" fn comparar_construct_turkey_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    cqm3_comparar("construct_turkey_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_turkey_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Polônia: uma janela, de 1º de julho a 20 de fevereiro.
const POLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x12, 0x00, 0xff, 0x01, 0x06, 0x01], // cqm3: a Polônia
    [0x12, 0x00, 0xff, 0x14, 0x01, 0x00],
];

/// { 0x7cd830 } (thiscall, ret 0x14, poland_rules.cpp, `POLAND_RULES::POLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Polônia (regra 18): a base (0x8e84a0), a vtable 0x96e61c, 1 janela(s) de `POLAND_TRANSFER_WINDOWS`; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30). Devolve o objeto.
unsafe extern "thiscall" fn construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_3 polonia");
    turkey_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96e61c, 1, &POLAND_TRANSFER_WINDOWS, 0xa786b8, 0x51);
    *((this + 0x13) as *mut u8) = 5;
    *((this + 0x14) as *mut u8) = 5;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm3_comparar`).
unsafe extern "thiscall" fn comparar_construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    cqm3_comparar("construct_poland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_poland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Finlândia: uma janela, de 16 de novembro a 15 de agosto.
const FINLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x08, 0x00, 0xff, 0x10, 0x0a, 0x01],
    [0x08, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqm3: a Finlândia
];

/// { 0x5a9950 } (thiscall, ret 0x14, finland_rules.cpp, `FINLAND_RULES::FINLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Finlândia (regra 8): a base (0x8e84a0), a vtable 0x96a75c, 1 janela(s) de `FINLAND_TRANSFER_WINDOWS`; o dword +0x15 = 1. Devolve o objeto.
unsafe extern "thiscall" fn construct_finland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_3 finlandia");
    turkey_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96a75c, 1, &FINLAND_TRANSFER_WINDOWS, 0x9cc134, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm3_comparar`).
unsafe extern "thiscall" fn comparar_construct_finland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    cqm3_comparar("construct_finland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_finland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
