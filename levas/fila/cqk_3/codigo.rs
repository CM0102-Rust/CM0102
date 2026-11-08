// ---------- %%LEVA%%a leva: as regras de transferência do Brasil, da Alemanha e da Escócia: as janelas de transferência (brazil_rules.cpp, germany_rules.cpp, scotland_rules.cpp) ----------

unsafe fn cqk3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
unsafe fn cqk3_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
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
unsafe fn brazil_rules_construct_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk3_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqk3_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqk3_d(this + 4) as usize, cqk3_d(r + 4) as usize);
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

/// A janela do Brasil: de 11 de dezembro (andando para a quarta-feira [inferência]) a 8 de novembro (andando para a sexta-feira [inferência]).
const CQK3_BRASIL: [[u8; 6]; 2] = [
    [0x04, 0x00, 0x03, 0x0b, 0x0b, 0x01], // cqk3: o Brasil abre em dezembro
    [0x04, 0x00, 0x05, 0x08, 0x0a, 0x00], // cqk3: o Brasil fecha em novembro
];

/// { 0x43f0e0 } (thiscall, ret 0x14, brazil_rules.cpp, `BRAZIL_RULES::BRAZIL_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência do Brasil (regra 4): a base (0x8e84a0), a vtable 0x968838, 1 janela(s) de `CQK3_BRASIL` e o +0x13 = 3 (o padrão da base é 50) [inferência: o limite de estrangeiros contratados]. Devolve o objeto.
unsafe extern "thiscall" fn construct_brazil_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_3 brasil");
    brazil_rules_construct_transfer_rules(this, [a1, a2, a3, a4, a5], 0x968838, 1, &CQK3_BRASIL, 0x98b104, 0x50);
    *((this + 0x13) as *mut u8) = 3; // cqk3: o limite do Brasil
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
unsafe extern "thiscall" fn comparar_construct_brazil_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    cqk3_comparar("construct_brazil_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_brazil_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Alemanha: de 1º de julho a 15 de agosto e de 1º de julho a 15 de janeiro.
const CQK3_ALEMANHA: [[u8; 6]; 4] = [
    [0x0a, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x00, 0xff, 0x0f, 0x07, 0x00],
    [0x0a, 0x01, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// { 0x5dffe0 } (thiscall, ret 0, germany_rules.cpp, `GERMANY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Alemanha (regra 10), depois da base e da vtable): as
/// 2 janelas de transferência de `CQK3_ALEMANHA`.
unsafe extern "thiscall" fn brazil_rules_init_transfer_windows(this: usize) {
    crate::comparacao::ramo("cqk_3 alemanha");
    cqk3_janelas(this, 2, &CQK3_ALEMANHA, 0x9cd538, 0x4d);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
unsafe extern "thiscall" fn comparar_brazil_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqk3_comparar("brazil_rules_init_transfer_windows", this, || o(this), |s| brazil_rules_init_transfer_windows(s));
}

/// A janela da Escócia: de 30 de maio (andando para o sábado [inferência]) a 30 de março (andando para a quinta-feira [inferência]).
const CQK3_ESCOCIA: [[u8; 6]; 2] = [
    [0x15, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x15, 0x00, 0x04, 0x1e, 0x02, 0x00],
];

/// { 0x7f6390 } (thiscall, ret 0x14, scotland_rules.cpp, `SCOTLAND_RULES::SCOTLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Escócia (regra 21): a base (0x8e84a0), a vtable 0x96f838, 1 janela(s) de `CQK3_ESCOCIA` e o dword +0x15 = 1. Devolve o objeto.
unsafe extern "thiscall" fn construct_scotland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_3 escocia");
    brazil_rules_construct_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96f838, 1, &CQK3_ESCOCIA, 0xa7b058, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
unsafe extern "thiscall" fn comparar_construct_scotland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    cqk3_comparar("construct_scotland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_scotland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
