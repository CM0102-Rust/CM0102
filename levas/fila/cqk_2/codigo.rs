// ---------- %%LEVA%%a leva: as regras de transferência da Espanha, da Itália e da França: as janelas de transferência (france_rules.cpp, italy_rules.cpp, spain_rules.cpp) ----------

unsafe fn cqk2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
unsafe fn cqk2_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
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
unsafe fn spain_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk2_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqk2_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqk2_d(this + 4) as usize, cqk2_d(r + 4) as usize);
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

/// As janelas da Espanha: de 17 de junho a 20 de agosto e de 15 de dezembro a 31 de janeiro.
const SPAIN_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x16, 0x00, 0x03, 0x11, 0x05, 0x01],
    [0x16, 0x00, 0x04, 0x14, 0x07, 0x00], // cqk2: a Espanha fecha em agosto
    [0x16, 0x01, 0xff, 0x0f, 0x0b, 0x01],
    [0x16, 0x01, 0xff, 0x1f, 0x00, 0x00],
];

/// { 0x859470 } (thiscall, ret 0, spain_rules.cpp, `SPAIN_RULES::init_transfer_windows`; chamada pelo construtor das regras da Espanha (regra 22), depois da base e da vtable): as
/// 2 janelas de transferência de `SPAIN_TRANSFER_WINDOWS`.
unsafe extern "thiscall" fn spain_rules_init_transfer_windows(this: usize) {
    crate::comparacao::ramo("cqk_2 espanha");
    cqk2_janelas(this, 2, &SPAIN_TRANSFER_WINDOWS, 0xa82ac8, 0x54);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk2_comparar`).
unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqk2_comparar("spain_rules_init_transfer_windows", this, || o(this), |s| spain_rules_init_transfer_windows(s));
}

/// As janelas da Itália: de 1º de julho a 27 de setembro (andando para a quinta-feira [inferência]), de 2 a 31 de janeiro, e a terceira, de 1º de julho a 30 de abril [inferência: a dos jogadores sem contrato].
const ITALY_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x0e, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x00, 0x04, 0x1b, 0x09, 0x00], // cqk2: a Itália fecha em setembro
    [0x0e, 0x01, 0xff, 0x02, 0x00, 0x01],
    [0x0e, 0x01, 0xff, 0x1f, 0x00, 0x00],
    [0x0e, 0x02, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x02, 0xff, 0x1e, 0x03, 0x00],
];

/// { 0x6613f0 } (thiscall, ret 0, italy_rules.cpp, `ITALY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Itália (regra 14), depois da base e da vtable): as
/// 3 janelas de transferência de `ITALY_TRANSFER_WINDOWS`.
unsafe extern "thiscall" fn spain_rules_init_transfer_windows_6613f0(this: usize) {
    crate::comparacao::ramo("cqk_2 italia");
    cqk2_janelas(this, 3, &ITALY_TRANSFER_WINDOWS, 0x9ea9a4, 0x52);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk2_comparar`).
unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows_6613f0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqk2_comparar("spain_rules_init_transfer_windows_6613f0", this, || o(this), |s| spain_rules_init_transfer_windows_6613f0(s));
}

/// As janelas da França: de 30 de maio a 31 de agosto, de 1º de setembro a 20 de dezembro e de 21 de dezembro a 10 de janeiro (com os dias da semana para os quais as datas andam [inferência]).
const FRANCE_TRANSFER_WINDOWS: [[u8; 6]; 6] = [
    [0x09, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x09, 0x00, 0x03, 0x1f, 0x07, 0x00], // cqk2: a França fecha em agosto
    [0x09, 0x01, 0x04, 0x01, 0x08, 0x01],
    [0x09, 0x01, 0x04, 0x14, 0x0b, 0x00],
    [0x09, 0x02, 0x05, 0x15, 0x0b, 0x01],
    [0x09, 0x02, 0x04, 0x0a, 0x00, 0x00],
];

/// { 0x5c15e0 } (thiscall, ret 0, france_rules.cpp, `FRANCE_RULES::init_transfer_windows`; chamada pelo construtor das regras da França (regra 9), depois da base e da vtable): as
/// 3 janelas de transferência de `FRANCE_TRANSFER_WINDOWS`.
unsafe extern "thiscall" fn spain_rules_init_transfer_windows_5c15e0(this: usize) {
    crate::comparacao::ramo("cqk_2 franca");
    cqk2_janelas(this, 3, &FRANCE_TRANSFER_WINDOWS, 0x9cccf4, 0x4f);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk2_comparar`).
unsafe extern "thiscall" fn comparar_spain_rules_init_transfer_windows_5c15e0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqk2_comparar("spain_rules_init_transfer_windows_5c15e0", this, || o(this), |s| spain_rules_init_transfer_windows_5c15e0(s));
}
