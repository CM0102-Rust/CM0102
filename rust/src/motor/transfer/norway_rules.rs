// ---------- 277a leva: as regras de transferência da Noruega, da Irlanda do Norte e do País de Gales: as janelas de transferência (northern_ireland_rules.cpp, norway_rules.cpp, wales_rules.cpp) ----------


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
    let (ja, jb) = (read_u32(this + 4) as usize, read_u32(r + 4) as usize);
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

