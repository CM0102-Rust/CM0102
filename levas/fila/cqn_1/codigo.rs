// ---------- %%LEVA%%a leva: as regras de transferência da Argentina, de Portugal e da Coreia do Sul: as janelas de transferência (argentina_rules.cpp, korea_rules.cpp, portugal_rules.cpp) ----------

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
unsafe extern "thiscall" fn argentina_rules_init_transfer_windows(this: usize) {
    crate::comparacao::ramo("cqn_1 argentina");
    cqn1_janelas(this, 2, &CQN1_ARGENTINA, 0x987960, 0x50);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn1_comparar`).
unsafe extern "thiscall" fn comparar_argentina_rules_init_transfer_windows(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqn1_comparar("argentina_rules_init_transfer_windows", this, || o(this), |s| argentina_rules_init_transfer_windows(s));
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
unsafe extern "thiscall" fn argentina_rules_init_transfer_windows_7d3df0(this: usize) {
    crate::comparacao::ramo("cqn_1 portugal");
    cqn1_janelas(this, 2, &CQN1_PORTUGAL, 0xa78844, 0x4f);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn1_comparar`).
unsafe extern "thiscall" fn comparar_argentina_rules_init_transfer_windows_7d3df0(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqn1_comparar("argentina_rules_init_transfer_windows_7d3df0", this, || o(this), |s| argentina_rules_init_transfer_windows_7d3df0(s));
}

/// As janelas da Coreia do Sul: uma janela, de 25 de novembro (andando para o domingo [inferência]) a 25 de julho.
const CQN1_COREIA: [[u8; 6]; 2] = [
    [0x1b, 0x00, 0x00, 0x19, 0x0a, 0x01],
    [0x1b, 0x00, 0xff, 0x19, 0x06, 0x00], // cqn1: a Coreia do Sul
];

/// { 0x66e630 } (thiscall, ret 0x14, korea_rules.cpp, `KOREA_RULES::KOREA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Coreia do Sul (regra 27): a base (0x8e84a0), a vtable 0x96cfec, 1 janela(s) de `CQN1_COREIA`; o +0x13 = 7 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
unsafe extern "thiscall" fn construct_korea_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqn_1 coreia");
    cqn1_construir(this, [a1, a2, a3, a4, a5], 0x96cfec, 1, &CQN1_COREIA, 0x9eb20c, 0x51);
    *((this + 0x13) as *mut u8) = 7;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn1_comparar`).
unsafe extern "thiscall" fn comparar_construct_korea_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    cqn1_comparar("construct_korea_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_korea_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
