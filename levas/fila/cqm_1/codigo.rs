// ---------- %%LEVA%%a leva: as regras de transferência da Holanda, da Croácia e da Irlanda: as janelas de transferência (croatia_rules.cpp, holland_rules.cpp, ireland_rules.cpp) ----------

unsafe fn cqm1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0 domingo ... 6 sábado, 0xff nenhum) [inferência], +3 o dia, +4 o
/// mês (0 = janeiro), +5 1 no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória,
/// a caixa da linha `linha` do fonte `arquivo`).
unsafe fn cqm1_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
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
unsafe fn holland_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqm1_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqm1_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqm1_d(this + 4) as usize, cqm1_d(r + 4) as usize);
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

/// As janelas da Holanda: uma janela, de 8 de junho (andando para o sábado [inferência]) a 4 de abril (andando para a sexta-feira [inferência]).
const CQM1_HOLANDA: [[u8; 6]; 2] = [
    [0x0c, 0x00, 0x06, 0x08, 0x05, 0x01], // cqm1: a Holanda
    [0x0c, 0x00, 0x05, 0x04, 0x03, 0x00],
];

/// { 0x5f9690 } (thiscall, ret 0x14, holland_rules.cpp, `HOLLAND_RULES::HOLLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Holanda (regra 12): a base (0x8e84a0), a vtable 0x96ba88, 1 janela(s) de `CQM1_HOLANDA`; o +0x13 = 50 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 50 (o padrão é 30). Devolve o objeto.
unsafe extern "thiscall" fn construct_holland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 holanda");
    holland_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96ba88, 1, &CQM1_HOLANDA, 0x9cf04c, 0x52);
    *((this + 0x13) as *mut u8) = 50;
    *((this + 0x14) as *mut u8) = 50;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm1_comparar`).
unsafe extern "thiscall" fn comparar_construct_holland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    cqm1_comparar("construct_holland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_holland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Croácia: uma janela, de 1º de junho a 30 de março.
const CQM1_CROACIA: [[u8; 6]; 2] = [
    [0x05, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm1: a Croácia
    [0x05, 0x00, 0xff, 0x1e, 0x02, 0x00],
];

/// { 0x518500 } (thiscall, ret 0x14, croatia_rules.cpp, `CROATIA_RULES::CROATIA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Croácia (regra 5): a base (0x8e84a0), a vtable 0x969394, 1 janela(s) de `CQM1_CROACIA`. Devolve o objeto.
unsafe extern "thiscall" fn construct_croatia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 croacia");
    holland_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x969394, 1, &CQM1_CROACIA, 0x9b5570, 0x4e);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm1_comparar`).
unsafe extern "thiscall" fn comparar_construct_croatia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    cqm1_comparar("construct_croatia_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_croatia_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Irlanda: uma janela, de 10 de maio a 31 de janeiro.
const CQM1_IRLANDA: [[u8; 6]; 2] = [
    [0x0d, 0x00, 0xff, 0x0a, 0x04, 0x01], // cqm1: a Irlanda
    [0x0d, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// { 0x63c210 } (thiscall, ret 0x14, ireland_rules.cpp, `IRELAND_RULES::IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda (regra 13): a base (0x8e84a0), a vtable 0x96c190, 1 janela(s) de `CQM1_IRLANDA`. Devolve o objeto.
unsafe extern "thiscall" fn construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 irlanda");
    holland_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96c190, 1, &CQM1_IRLANDA, 0x9ea12c, 0x4e);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm1_comparar`).
unsafe extern "thiscall" fn comparar_construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    cqm1_comparar("construct_ireland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_ireland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
