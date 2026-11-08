// ---------- %%LEVA%%a leva: as regras de transferência da Inglaterra, dos EUA e da Austrália: as janelas de transferência (australia_rules.cpp, england_rules.cpp, usa_rules.cpp) ----------

unsafe fn cqk1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
unsafe fn set_transfer_windows(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
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
unsafe fn england_rules_construct_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    set_transfer_windows(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn compare_transfer_rules_ctor(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqk1_d(this + 4) as usize, cqk1_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    SCOREBOARD.record(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// A janela da Inglaterra: de 2 de junho (andando para a segunda-feira [inferência]) a 26 de março (andando para a quinta-feira [inferência]).
const ENGLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x07, 0x00, 0x01, 0x02, 0x05, 0x01], // cqk1: a Inglaterra abre em junho
    [0x07, 0x00, 0x04, 0x1a, 0x02, 0x00],
];

/// { 0x579b30 } (thiscall, ret 0x14, england_rules.cpp, `ENGLAND_RULES::ENGLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Inglaterra (regra 7): a base (0x8e84a0), a vtable 0x969f4c, 1 janela(s) de `ENGLAND_TRANSFER_WINDOWS`. Devolve o objeto.
unsafe extern "thiscall" fn construct_england_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_1 inglaterra");
    england_rules_construct_transfer_rules(this, [a1, a2, a3, a4, a5], 0x969f4c, 1, &ENGLAND_TRANSFER_WINDOWS, 0x9c4498, 0x4f);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
unsafe extern "thiscall" fn comparar_construct_england_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_england_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_england_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// A janela dos EUA: de 7 de janeiro (andando para a quarta-feira [inferência]) a 15 de agosto.
const USA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x19, 0x00, 0x03, 0x07, 0x00, 0x01],
    [0x19, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqk1: os EUA fecham em agosto
];

/// { 0x90b940 } (thiscall, ret 0x14, usa_rules.cpp, `USA_RULES::USA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência dos EUA (regra 25): a base (0x8e84a0), a vtable 0x970a38, 1 janela(s) de `USA_TRANSFER_WINDOWS` e o +0x13 = 4 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
unsafe extern "thiscall" fn construct_usa_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_1 eua");
    england_rules_construct_transfer_rules(this, [a1, a2, a3, a4, a5], 0x970a38, 1, &USA_TRANSFER_WINDOWS, 0xaa211c, 0x54);
    *((this + 0x13) as *mut u8) = 4;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
unsafe extern "thiscall" fn comparar_construct_usa_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_usa_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_usa_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// A janela da Austrália: de 11 de junho (andando para o sábado [inferência]) a 17 de abril.
const AUSTRALIA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x02, 0x00, 0x06, 0x0b, 0x05, 0x01], // cqk1: a Austrália abre em junho
    [0x02, 0x00, 0xff, 0x11, 0x03, 0x00],
];

/// { 0x412c80 } (thiscall, ret 0x14, australia_rules.cpp, `AUSTRALIA_RULES::Australia_rules`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Austrália (regra 2): a base (0x8e84a0), a vtable 0x967830, 1 janela(s) de `AUSTRALIA_TRANSFER_WINDOWS` e o +0x13 = 6 [inferência: o limite de estrangeiros]. Devolve o objeto.
unsafe extern "thiscall" fn australia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_1 australia");
    england_rules_construct_transfer_rules(this, [a1, a2, a3, a4, a5], 0x967830, 1, &AUSTRALIA_TRANSFER_WINDOWS, 0x987cfc, 0x51);
    *((this + 0x13) as *mut u8) = 6;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
unsafe extern "thiscall" fn comparar_australia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("australia_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        australia_rules(s, a1, a2, a3, a4, a5);
    });
    r
}
