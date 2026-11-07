use super::*;

// ---------- 273a leva: as regras de transferência da Inglaterra, dos EUA e da Austrália: as janelas de transferência (australia_rules.cpp, england_rules.cpp, usa_rules.cpp) ----------


/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
pub(crate) unsafe fn cqk1_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
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
pub(crate) unsafe fn cqk1_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk1_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
pub(crate) unsafe fn cqk1_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
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

/// A janela da Inglaterra: de 2 de junho (andando para a segunda-feira [inferência]) a 26 de março (andando para a quinta-feira [inferência]).
pub(crate) const CQK1_INGLATERRA: [[u8; 6]; 2] = [
    [0x07, 0x00, 0x01, 0x02, 0x05, 0x01], // cqk1: a Inglaterra abre em junho
    [0x07, 0x00, 0x04, 0x1a, 0x02, 0x00],
];

/// the English rule (7): one transfer window, from 2 June (moved to the Monday [inference]) to 26 March (moved to the
/// Thursday [inference]); England's other rules (non-EU players, loans: 5 a season and 2 at once, same league) live in
/// the vtable 0x969f4c methods
/// Original: `ENGLAND_RULES::ENGLAND_RULES` (0x579b30, england_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x579b30 } (thiscall, ret 0x14, england_rules.cpp, `ENGLAND_RULES::ENGLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Inglaterra (regra 7): a base (0x8e84a0), a vtable 0x969f4c, 1 janela(s) de `CQK1_INGLATERRA`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_inglaterra(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_1 inglaterra");
    cqk1_construir(this, [a1, a2, a3, a4, a5], 0x969f4c, 1, &CQK1_INGLATERRA, 0x9c4498, 0x4f);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_inglaterra(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1676);
    let mut r = 0usize;
    cqk1_comparar("regras_inglaterra", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_inglaterra(s, a1, a2, a3, a4, a5);
    });
    r
}

/// A janela dos EUA: de 7 de janeiro (andando para a quarta-feira [inferência]) a 15 de agosto.
pub(crate) const CQK1_EUA: [[u8; 6]; 2] = [
    [0x19, 0x00, 0x03, 0x07, 0x00, 0x01],
    [0x19, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqk1: os EUA fecham em agosto
];

/// the USA rule (25): one window, from 7 January (moved to the Wednesday [inference]) to 15 August; +0x13 = 4 (default
/// 50) [inference: the foreign squad limit]
/// Original: `USA_RULES::USA_RULES` (0x90b940, usa_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x90b940 } (thiscall, ret 0x14, usa_rules.cpp, `USA_RULES::USA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência dos EUA (regra 25): a base (0x8e84a0), a vtable 0x970a38, 1 janela(s) de `CQK1_EUA` e o +0x13 = 4 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_eua(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_1 eua");
    cqk1_construir(this, [a1, a2, a3, a4, a5], 0x970a38, 1, &CQK1_EUA, 0xaa211c, 0x54);
    *((this + 0x13) as *mut u8) = 4;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_eua(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1677);
    let mut r = 0usize;
    cqk1_comparar("regras_eua", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_eua(s, a1, a2, a3, a4, a5);
    });
    r
}

/// A janela da Austrália: de 11 de junho (andando para o sábado [inferência]) a 17 de abril.
pub(crate) const CQK1_AUSTRALIA: [[u8; 6]; 2] = [
    [0x02, 0x00, 0x06, 0x0b, 0x05, 0x01], // cqk1: a Austrália abre em junho
    [0x02, 0x00, 0xff, 0x11, 0x03, 0x00],
];

/// the Australian rule (2): one window, from 11 June (moved to the Saturday [inference]) to 17 April; +0x13 = 6
/// [inference: the foreign squad limit]
/// Original: `AUSTRALIA_RULES::Australia_rules` (0x412c80, australia_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x412c80 } (thiscall, ret 0x14, australia_rules.cpp, `AUSTRALIA_RULES::Australia_rules`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Austrália (regra 2): a base (0x8e84a0), a vtable 0x967830, 1 janela(s) de `CQK1_AUSTRALIA` e o +0x13 = 6 [inferência: o limite de estrangeiros]. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_australia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_1 australia");
    cqk1_construir(this, [a1, a2, a3, a4, a5], 0x967830, 1, &CQK1_AUSTRALIA, 0x987cfc, 0x51);
    *((this + 0x13) as *mut u8) = 6;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_australia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1678);
    let mut r = 0usize;
    cqk1_comparar("regras_australia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_australia(s, a1, a2, a3, a4, a5);
    });
    r
}

// ---------- 268a leva: as regras de empréstimo da Inglaterra: pode emprestar, a conta da temporada e o dia (england_rules.cpp) ----------



pub(crate) unsafe fn cqe5_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de um contrato: +4 é o índice em [0xae23b4] (registros de 0x245 bytes), como as regras calculam.
pub(crate) unsafe fn cqe5_clube_do_contrato(c: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
}

/// O contrato principal da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), que a original
/// guarda em [0xae1080] a cada consulta.
pub(crate) unsafe fn cqe5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// A entrada de 16 bytes do clube na tabela dos empréstimos da temporada ([[0xde1f54] + 0x91c] + id do clube que pega emprestado × 16, o byte +0
/// é a conta).
pub(crate) unsafe fn cqe5_conta(clube: usize) -> usize {
    read_usize(read_usize(0xde1f54) + 0x91c).wrapping_add(read_usize(clube).wrapping_mul(16))
}

/// refuses (0, with the message and the reason) between clubs of the same English division (0x1d), with five loans this
/// season (0x1e), with more than one player on loan from another club in the squad (0x26) and outside the window (9);
/// otherwise 1
/// Original: `ENGLAND_RULES::can_loan_player` (0x579d60, england_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x579d60 } (thiscall, ret 0x10, england_rules.cpp): `ENGLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`, pelas regras da nação em `TRANSFER_MANAGER::can_loan_player` (0x8d0040). Sem contrato (ou sem clube do
/// contrato): pode. O clube que pede e o dono na mesma divisão inglesa ([0x9cf5bc]): motivo 0x1d. Cinco empréstimos na
/// temporada (a tabela [[0xde1f54] + 0x91c]): 0x1e. Mais de um jogador de outro clube no elenco (as 50 vagas em +0xd7,
/// contados só com o id do jogador abaixo de [0xae2364]): 0x26. Fora da janela (0x8e8e50): 9, com o adjetivo da nação.
pub(crate) unsafe extern "thiscall" fn cqe5_pode_emprestar(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x66f4e0: o texto da mensagem (push ...; push formato; push destino; add esp, 0x14) -> cdecl variádica
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = cqe5_contrato(jogador);
    if c == 0 || cqe5_clube_do_contrato(c) == 0 {
        return 1;
    }
    let c2 = cqe5_contrato(jogador);
    let dono = if c2 != 0 { cqe5_clube_do_contrato(c2) } else { 0 };
    let comp = read_usize(clube + 0x57);
    if comp == read_usize(dono + 0x57) && comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) {
        crate::comparacao::ramo("cqe_5 mesma divisao");
        if msg != 0 {
            let id = read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION);
            let r = read_usize(0xae23d0).wrapping_add(id.wrapping_mul(0x6b));
            let g = read_u8(r + 0x37) as i8 as i32;
            texto(msg, 0x9c4590, g, g, r + 4);
        }
        if !motivo.is_null() {
            *motivo = 0x1d;
        }
        return 0;
    }
    if read_u8(cqe5_conta(clube)) as i8 >= 5 {
        // cqe5: cinco na temporada
        crate::comparacao::ramo("cqe_5 cinco na temporada");
        if msg != 0 {
            let g = read_u8(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c4534, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x1e;
        }
        return 0;
    }
    // 0x4cf6d0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4 (o contrato de empréstimo)
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = read_usize(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        if jogador != 0 && read_usize(jogador) as i32 >= read_usize(0xae2364) as i32 {
            continue;
        }
        if emprestimo(read_usize(0xae1050), p) == 0 {
            continue;
        }
        let cp = cqe5_contrato(p);
        let de = if cp != 0 { cqe5_clube_do_contrato(cp) } else { 0 };
        if de != clube {
            n = n.wrapping_add(1);
        }
    }
    if n > 1 { // cqe5: dois ao mesmo tempo
        crate::comparacao::ramo("cqe_5 dois ao mesmo tempo");
        if msg != 0 {
            let g = read_u8(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c44d4, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x26;
        }
        return 0;
    }
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8 (a janela aberta hoje)
    let janela: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_FIND_OPEN_WINDOW);
    if janela(this, 0xae2c90, 0) != 0 {
        return 1;
    }
    crate::comparacao::ramo("cqe_5 fora da janela");
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nação; add esp, 0x10 -> cdecl (o adjetivo da nação)
        let adjetivo: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_NATION_GET_NATIONALITY_ADJECTIVE);
        let mut buf = [0u8; 100];
        adjetivo(read_usize(this + 0xa), buf.as_mut_ptr(), 100, 2);
        texto(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !motivo.is_null() {
        *motivo = 9;
    }
    0
}

/// adds 1 to the season loan count of the borrowing club when it is English
/// Original: `ENGLAND_RULES::record_loan` (0x579fd0, england_rules.cpp, thiscall, ret 4).
///
/// [pt] { 0x579fd0 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::record_loan(oferta)`, quando um empréstimo se
/// conclui (0x8c9390): o clube da oferta (+8, índice em [0xae23b4]) da nação inglesa ([0x9cf2e4]) soma 1 na conta da
/// temporada.
pub(crate) unsafe extern "thiscall" fn cqe5_registrar_emprestimo(_this: usize, oferta: usize) {
    let clube = read_usize(0xae23b4).wrapping_add(read_usize(oferta + 8).wrapping_mul(0x245));
    let nacao = read_usize(clube + 0x53);
    if nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_ENGLAND) {
        crate::comparacao::ramo("cqe_5 emprestimo contado");
        let p = cqe5_conta(clube);
        *(p as *mut u8) = read_u8(p).wrapping_add(1); // cqe5: a conta da temporada
    }
}

/// on 20 June, resets the season loan count of every club of this rule set nation
/// Original: `ENGLAND_RULES::daily_update` (0x57a020, england_rules.cpp, thiscall, ret 4).
///
/// [pt] { 0x57a020 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::daily_update(data)`, pelo dia das regras: em
/// 20 de junho (só o dia do ano, 0x549f70 com 1998), a conta da temporada de cada clube ([0xae235c] em [0xae23b4]) da
/// nação destas regras (+0xa) volta a zero.
pub(crate) unsafe extern "thiscall" fn cqe5_dia_das_regras(this: usize, data: usize) {
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if cqe5_h(data) != cqe5_h(r) {
        return;
    }
    let mut i: i32 = 0;
    while i < read_usize(0xae235c) as i32 {
        let clube = read_usize(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245));
        if read_usize(clube + 0x53) == read_usize(this + 0xa) {
            *(cqe5_conta(clube) as *mut u8) = 0;
        }
        i += 1;
    }
}

/// O texto da mensagem (até o zero, no máximo 0x200 bytes).
pub(crate) unsafe fn cqe5_texto(msg: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if msg == 0 {
        return v;
    }
    for k in 0..0x200usize {
        let b = read_u8(msg + k);
        v.push(b);
        if b == 0 {
            break;
        }
    }
    v
}

/// Comparador comum das regras de empréstimo (pode emprestar): lado a lado na devolução, em [0xae1080] e no motivo; a
/// mensagem, que só é escrita, é conferida pelo texto (antes da cópia o primeiro byte vai a zero, e no fim volta o
/// texto da original).
pub(crate) unsafe fn cqe5_comparar_pode(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, nome: &str, this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || {
        let x = o(this, clube, jogador, msg, motivo);
        to = cqe5_texto(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, clube, jogador, msg, motivo);
        tr = cqe5_texto(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    PLACAR.registrar(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", nome, this, clube, jogador, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}

pub(crate) unsafe extern "thiscall" fn comparar_cqe5_pode_emprestar(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe5_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1586), cqe5_pode_emprestar, "cqe5_pode_emprestar", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado no byte da conta do clube da oferta.
pub(crate) unsafe extern "thiscall" fn comparar_cqe5_registrar_emprestimo(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1587);
    let clube = read_usize(0xae23b4).wrapping_add(read_usize(oferta + 8).wrapping_mul(0x245));
    let regioes = [(cqe5_conta(clube), 1usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || cqe5_registrar_emprestimo(this, oferta));
    PLACAR.registrar(igual, || format!("cqe5_registrar_emprestimo(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

/// Comparador: lado a lado na tabela das contas inteira ([0xae235c] clubes × 16 bytes) só no dia em que ela muda (20
/// de junho); nos outros dias as duas rodam sem gravar nada e só a original fica valendo.
pub(crate) unsafe extern "thiscall" fn comparar_cqe5_dia_das_regras(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1588);
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if cqe5_h(data) != cqe5_h(r) {
        o(this, data);
        return;
    }
    let n = (read_usize(0xae235c) as i32).max(0) as usize;
    let regioes = [(read_usize(read_usize(0xde1f54) + 0x91c), n * 16)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, data), || cqe5_dia_das_regras(this, data));
    PLACAR.registrar(igual, || format!("cqe5_dia_das_regras(regras {:#x}) tabela diverge", this));
}

