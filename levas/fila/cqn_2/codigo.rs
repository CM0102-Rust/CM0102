// ---------- %%LEVA%%a leva: as regras de transferência da Grécia, da Rússia e da Bélgica: as janelas de transferência (belgium_rules.cpp, greece_rules.cpp, russia_rules.cpp) ----------

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
        show_error_at(arquivo, linha);
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
unsafe extern "thiscall" fn greece_rules_init_transfer_windows(this: usize) {
    crate::comparacao::ramo("cqn_2 grecia");
    cqn2_janelas(this, 2, &CQN2_GRECIA, 0x9cdbc8, 0x51);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn2_comparar`).
unsafe extern "thiscall" fn comparar_greece_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqn2_comparar("greece_rules_init_transfer_windows", this, || o(this), |s| greece_rules_init_transfer_windows(s));
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
unsafe extern "thiscall" fn greece_rules_init_transfer_windows_7ec7e0(this: usize) {
    crate::comparacao::ramo("cqn_2 russia");
    cqn2_janelas(this, 2, &CQN2_RUSSIA, 0xa7ad38, 0x4d);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn2_comparar`).
unsafe extern "thiscall" fn comparar_greece_rules_init_transfer_windows_7ec7e0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqn2_comparar("greece_rules_init_transfer_windows_7ec7e0", this, || o(this), |s| greece_rules_init_transfer_windows_7ec7e0(s));
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
unsafe extern "thiscall" fn greece_rules_init_transfer_windows_424c00(this: usize) {
    crate::comparacao::ramo("cqn_2 belgica");
    cqn2_janelas(this, 3, &CQN2_BELGICA, 0x989e34, 0x53);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqn2_comparar`).
unsafe extern "thiscall" fn comparar_greece_rules_init_transfer_windows_424c00(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqn2_comparar("greece_rules_init_transfer_windows_424c00", this, || o(this), |s| greece_rules_init_transfer_windows_424c00(s));
}
