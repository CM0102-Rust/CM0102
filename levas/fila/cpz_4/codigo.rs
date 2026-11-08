// ---------- %%LEVA%%a leva: a Liga dos Campeões: os registros pela fase alcançada, o sorteio das quartas e a lista dos clubes inscritos (european_cup.cpp) ----------

unsafe fn cpz4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cpz4_registros() -> usize {
    cpz4_d(0xdd7f00)
}

/// comp_util_scale_by_round (0x4bf850, cdecl(baixo, alto, rodada, rodadas)): a faixa pela rodada, de 0 a 100.
unsafe fn cpz4_escala(baixo: u32, alto: u32, rodada: i16, rodadas: u32) -> u32 {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
    // ponytail: a original empurra a rodada num registrador com lixo nos 16 bits altos; a 0x4bf850 só lê a palavra.
    f(baixo, alto, rodada as u16 as u32, rodadas)
}

/// Staff_Records::add_qualified_for_round_record (0x86bdd0: thiscall(registros, clube, comp, fase, rodada, peso),
/// ret 0x14).
unsafe fn cpz4_classificado(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cpz4_registros(), clube, comp, fase, rodada, peso);
}

/// Staff_Records::add_failed_to_qualify_record (0x86c1d0: thiscall(registros, clube, comp, fase, _), ret 0x10).
unsafe fn cpz4_nao_classificado(clube: usize, comp: usize, fase: u32, x: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
    f(cpz4_registros(), clube, comp, fase, x);
}

/// Staff_Records::add_knocked_out_record (0x86c000: thiscall(registros, clube, comp, fase, rodada, _), ret 0x14).
unsafe fn cpz4_eliminado(clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
    f(cpz4_registros(), clube, comp, fase, rodada, x);
}

/// O clube sai da competição europeia da temporada (+0x1db = −1).
unsafe fn cpz4_sair(clube: usize) {
    core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1);
}

/// { 0x584ae0 } (thiscall, ret 0x18, european_cup.cpp): `EuropeanCup::update_staff_records_for_status(clube,
/// desfecho, fase, _, jogo, final)`, a virtual +0x44 da Liga dos Campeões. Devolve 0. Comp = +4.
/// - Na qualificação (fase 0xff): desfecho 0 ou 1, a classificação pela rodada do jogo (+0x34): na rodada 0, com a
///   fase +0x3a e o +0x6f da primeira rodada (+0xa3) e o peso 0x4bf850(1, 0x14, 0, 2); noutra, 0x41c, 0 e
///   0x4bf850(1, 0x14, rodada, 2). Desfecho 0xff ou 2: a eliminação (0x86c000) com +0x32 e +0x30 do jogo e o peso
///   0x4bf850(1, 0x14, rodada, 2), e o clube sai (+0x1db = −1).
/// - Na fase 1 (fase < 8): 6, a classificação (0x44a, −1, 0x32); 0xfe, a não classificação (0x449, 0x32) e sai.
/// - Na fase 2 (8 a 11): 6, a classificação (0, 0x78, 0x32); 0xfe, a não classificação (0x44a, 0x32) e sai.
/// - No mata-mata (12): 1, o título (Staff_Records::add_cup_winners_record 0x86a800, thiscall(registros, clube, jogo,
///   final), ret 0xc) e sai; 2, o vice (0x86b0b0, idem) e sai; 0, a classificação (0, o +0x6f da rodada do jogo na
///   tabela de rodadas da fase do mata-mata ([+0xc] + 0x30, +0xa3, 0x68 bytes), 0x4bf850(0x4b, 0x64, rodada, 2));
///   0xff, a eliminação (0, +0x30 do jogo, 0x4bf850(0x4b, 0x64, rodada, 2)) e sai.
unsafe extern "thiscall" fn european_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, _a: u32, jogo: usize, fim: u32) -> u32 {
    let d = desfecho as u8;
    let f = fase as u8;
    let comp = cpz4_d(this + 4);
    if f == 0xff {
        match d {
            0 | 1 => {
                let r = cpz4_w(jogo + 0x34);
                if r == 0 {
                    let peso = cpz4_escala(1, 0x14, 0, 2);
                    let rodada = cpz4_w(cpz4_d(this + 0xa3) + 0x6f) as u16 as u32;
                    // ponytail: a fase +0x3a vai num registrador com lixo nos 16 bits altos; a 0x86bdd0 só usa a palavra.
                    cpz4_classificado(clube, comp, cpz4_w(this + 0x3a) as u16 as u32, rodada, peso);
                } else {
                    let peso = cpz4_escala(1, 0x14, r, 2);
                    cpz4_classificado(clube, comp, 0x41c, 0, peso);
                }
            }
            0xff | 2 => {
                let peso = cpz4_escala(1, 0x14, cpz4_w(jogo + 0x34), 2);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras.
                cpz4_eliminado(clube, comp, cpz4_w(jogo + 0x32) as u16 as u32, cpz4_w(jogo + 0x30) as u16 as u32, peso);
                crate::comparacao::ramo("cpz_4 caiu na qualificacao");
                cpz4_sair(clube); // cpz4: quem cai na qualificação sai da competição europeia
            }
            _ => {}
        }
    } else if (f as i8) < 8 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0x44a, 0xffff_ffff, 0x32);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x449, 0x32);
            crate::comparacao::ramo("cpz_4 caiu na fase 1");
            cpz4_sair(clube); // cpz4: quem cai na fase 1 sai da competição europeia
        }
    } else if (f as i8) < 0xc {
        if d == 6 {
            crate::comparacao::ramo("cpz_4 passou da fase 2");
            cpz4_classificado(clube, comp, 0, 0x78, 0x32); // cpz4: pass_ball da fase 2 pesa 0x32
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x44a, 0x32);
            cpz4_sair(clube);
        }
    } else if f == 0xc {
        match d {
            1 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x86a800usize);
                titulo(cpz4_registros(), clube, jogo, fim);
                cpz4_sair(clube);
            }
            2 => {
                let vice: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x86b0b0usize);
                vice(cpz4_registros(), clube, jogo, fim);
                cpz4_sair(clube);
            }
            0 => {
                let peso = cpz4_escala(0x4b, 0x64, cpz4_w(jogo + 0x34), 2);
                let mata = cpz4_d(cpz4_d(this + 0xc) + 0x30);
                let rodadas = cpz4_d(mata + 0xa3);
                let r = cpz4_w(jogo + 0x34) as i32;
                // os bits de cima do registrador que a original empurra são os do ponteiro das fases (+0xc)
                let rodada = (cpz4_d(this + 0xc) as u32 & 0xffff_0000) | cpz4_w(rodadas.wrapping_add((r as usize).wrapping_mul(0x68)) + 0x6f) as u16 as u32;
                cpz4_classificado(clube, comp, 0, rodada, peso);
            }
            0xff => {
                let peso = cpz4_escala(0x4b, 0x64, cpz4_w(jogo + 0x34), 2);
                cpz4_eliminado(clube, comp, 0, cpz4_w(jogo + 0x30) as u16 as u32, peso);
                cpz4_sair(clube);
            }
            _ => {}
        }
    }
    0
}

// ponytail: os registros vão para o arquivo, mexem nas reputações e no moral (sorteiam) e o título e o vice gravam o
// elenco da final: não rodam duas vezes. Só a original roda; o plano confere o que a rotina mesma grava (+0x1db do
// clube: −1 nos casos em que ele sai, intacto nos outros).
unsafe extern "thiscall" fn comparar_european_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, a: u32, jogo: usize, fim: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%);
    let (d, f) = (desfecho as u8, fase as u8);
    // as faixas da original comparam o byte da fase com sinal (0x80 a 0xfe contam como fase 1)
    let sai = if f == 0xff {
        d == 0xff || d == 2
    } else if (f as i8) < 0xc {
        d == 0xfe
    } else if f == 0xc {
        d == 1 || d == 2 || d == 0xff
    } else {
        false
    };
    let antes = if clube != 0 { core::ptr::read_unaligned((clube + 0x1db) as *const i32) } else { 0 };
    let r = o(this, clube, desfecho, fase, a, jogo, fim);
    let depois = if clube != 0 { core::ptr::read_unaligned((clube + 0x1db) as *const i32) } else { 0 };
    let igual = r == 0 && (clube == 0 || depois == if sai { -1 } else { antes });
    PLACAR.registrar(igual, || format!("european_cup_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} +0x1db {} -> {} sai={}", this, clube, d, f, r, antes, depois, sai));
    r
}

/// Os quatro sorteados da fase 2 (os índices dos grupos, todos diferentes): a, b ≠ a, c ≠ a, b e d ≠ a, b, c, cada um
/// com sorteio(4) (0x90cfc0) até sair um que serve (a d sorteia pelo menos uma vez: começa igual à a).
unsafe fn cpz4_sorteio_dos_grupos() -> [i8; 4] {
    let a = sortear(4) as i8;
    let mut b = sortear(4) as i8;
    while b == a {
        b = sortear(4) as i8;
    }
    let mut c = sortear(4) as i8;
    while c == a || c == b {
        c = sortear(4) as i8;
    }
    let mut d = a;
    while d == a || d == b || d == c {
        d = sortear(4) as i8;
    }
    [a, b, c, d]
}

/// O clube da linha `k` do grupo `g` da fase 2 (a fase [+0xc] + 0x20 + 4 × g; League::get_table_row 0x6826d0,
/// thiscall(grupo, k), ret 4).
unsafe fn cpz4_do_grupo(this: usize, g: i8, k: u32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x6826d0usize);
    cpz4_d(linha(cpz4_d(cpz4_d(this + 0xc) + 0x20 + 4 * (g as i32 as usize)), k))
}

/// { 0x586330 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::draw_quarter_finals()`, da fase seguinte 0x5851d0
/// na fase 12. A lista das 8 (calloc 0x945501 de 8 × 4; sem memória: a caixa da linha 0x762) com os grupos sorteados
/// a, b, c e d: o 1º de a × o 2º de b, o 2º de a × o 1º de b, o 1º de c × o 2º de d e o 2º de c × o 1º de d. Cada um
/// passa pela 0x7e9180 (thiscall(o objeto [[0xae28f0]] + 4 × +0x50, clube), ret 4). A virtual +0x3c (create_round_table:
/// fase 12) e a fase do mata-mata (operator new 0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00 com a copa, 8, a lista,
/// as rodadas, +4, a tabela, +0x40, 0xc, 2, o extra, 0x14, 1, 0, 0, 0) na vaga [+0xc] + 0x30; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa da linha 0x78a. Senão o 3º e o 4º de cada grupo da fase 2 saem (+0x1db =
/// −1), a rodada montada (Cup::set_up_round 0x51c800, thiscall(fase, 0)) e +0x30 = 12.
unsafe extern "thiscall" fn draw_quarter_finals(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let lista = calloc(8, 4);
    if lista == 0 {
        return aviso_em(0x9c4900, 0x762);
    }
    crate::comparacao::ramo("cpz_4 sorteio das quartas");
    let [a, b, c, d] = cpz4_sorteio_dos_grupos();
    let ordem: [(i8, u32); 8] = [(a, 0), (b, 1), (a, 1), (b, 0), (c, 0), (d, 1), (c, 1), (d, 0)];
    for (k, &(g, l)) in ordem.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cpz4_do_grupo(this, g, l));
    }
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e9180usize);
    for k in 0..8 {
        let obj = cpz4_d(cpz4_d(cpz4_d(0xae28f0)) + 4 * (cpz4_b(this + 0x50) as i8 as i32 as usize));
        tv(obj, cpz4_d(lista + 4 * k));
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cpz4_d(cpz4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0xc, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: as rodadas, o extra e a palavra +0x40 vão em registradores com lixo nos 16 bits altos; o construtor
    // só lê as palavras.
    let fase = if p != 0 {
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 8, lista, rodadas as u16 as u32, cpz4_d(this + 4), t, cpz4_w(this + 0x40) as u16 as u32, 0xc, 2, extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cpz4_d(this + 0xc) + 0x30) as *mut usize, fase);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(lista);
    liberar(t);
    let fase = cpz4_d(cpz4_d(this + 0xc) + 0x30);
    if fase == 0 {
        return aviso_em(0x9c4900, 0x78a);
    }
    for g in 0..4i8 {
        cpz4_sair(cpz4_do_grupo(this, g, 2));
        cpz4_sair(cpz4_do_grupo(this, g, 3));
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, 0xc);
}

// ponytail: o sorteio sorteia, a 0x7e9180 e o construtor alocam e a rodada cria os jogos: só a original. O plano,
// feito antes: os 8 primeiros e segundos dos grupos e o 3º e 4º de cada um; depois, +0x30 = 12, a fase do mata-mata
// com 8 clubes que são os primeiros e segundos (como conjunto) e o 3º e o 4º de cada grupo com +0x1db = −1.
unsafe extern "thiscall" fn comparar_draw_quarter_finals(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let mut vivos: Vec<usize> = Vec::new();
    let mut fora: Vec<usize> = Vec::new();
    for g in 0..4i8 {
        vivos.push(cpz4_do_grupo(this, g, 0));
        vivos.push(cpz4_do_grupo(this, g, 1));
        fora.push(cpz4_do_grupo(this, g, 2));
        fora.push(cpz4_do_grupo(this, g, 3));
    }
    o(this);
    let fase = cpz4_d(cpz4_d(this + 0xc) + 0x30);
    if fase == 0 {
        PLACAR.registrar(false, || format!("draw_quarter_finals({:#x}) sem fase", this));
        return;
    }
    let lista = cpz4_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| cpz4_d(lista + 6 * k)).collect();
    let mut esperado = vivos.clone();
    achou.sort_unstable();
    esperado.sort_unstable();
    let saiu = fora.iter().all(|&c| core::ptr::read_unaligned((c + 0x1db) as *const i32) == -1);
    let igual = achou == esperado && saiu && cpz4_w(fase + 0x3e) == 8 && core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xc;
    PLACAR.registrar(igual, || format!("draw_quarter_finals({:#x}) esperado={:x?} lista={:x?} saiu={} +0x30={}", this, esperado, achou, saiu, core::ptr::read_unaligned((this + 0x30) as *const i32)));
}

/// A lista dos inscritos (+0x14, 6 bytes: +0 o clube, +4 a rodada de entrada; +0x38 deles) que seguem na competição
/// (+0x1db = o id da copa, [+4]); com `sem_7`, fora os de rodada de entrada 7. Grava em `v` (até 71) e devolve a conta.
unsafe fn cpz4_inscritos(this: usize, sem_7: bool, v: *mut usize) -> i16 {
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while i < cpz4_w(this + 0x38) {
        let e = cpz4_d(this + 0x14).wrapping_add((i as i32 as usize).wrapping_mul(6));
        let c = cpz4_d(e);
        if core::ptr::read_unaligned((c + 0x1db) as *const i32) == core::ptr::read_unaligned(cpz4_d(this + 4) as *const i32) && !(sem_7 && cpz4_b(e + 4) == 7) {
            core::ptr::write_unaligned(v.add(n as i32 as usize), c);
            n = n.wrapping_add(1);
        }
        i += 1;
    }
    n
}

/// { 0x586660 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::collect_club_list(&lista, tipo)`, a virtual +0x58
/// da Liga dos Campeões. Tipo 1: os inscritos que seguem, fora os da rodada 7; tipo 2: todos os que seguem; tipo 3 na
/// fase 11: os que seguem; noutro caso a da competição (Comp::collect_club_list 0x48e1c0, thiscall(this, &lista, 0),
/// ret 8). A lista (malloc 0x944e46 de 0x11c bytes; sem memória: a caixa das linhas 0x7b5, 0x7cf ou 0x7e9, lista
/// nula e 0) vai em `lista`; devolve a conta na palavra de baixo (a de cima é a do ponteiro da lista).
unsafe extern "thiscall" fn european_cup_collect_club_list(this: usize, saida: usize, tipo: u32) -> u32 {
    let t = tipo as u8;
    let (linha, sem_7) = match t {
        1 => (0x7b5, true),
        2 => (0x7cf, false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => (0x7e9, false),
        _ => {
            let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x48e1c0usize);
            return base(this, saida, 0);
        }
    };
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0x11c);
    if v == 0 {
        aviso_em(0x9c4900, linha);
        core::ptr::write_unaligned(saida as *mut usize, 0);
        return 0;
    }
    crate::comparacao::ramo("cpz_4 lista dos inscritos");
    let n = cpz4_inscritos(this, sem_7, v as *mut usize);
    core::ptr::write_unaligned(saida as *mut usize, v);
    (v as u32 & 0xffff_0000) | n as u16 as u32
}

/// Comparador da 0x586660: a original aloca a lista (só ela roda; quem chama libera). O plano, feito antes numa lista
/// local pelos mesmos inscritos: a conta (palavra de baixo da devolução) e os clubes da lista dela iguais. Nos tipos
/// que vão para a 0x48e1c0 e sem lista: a devolução e a saída como a original deixou.
unsafe extern "thiscall" fn comparar_european_cup_collect_club_list(this: usize, saida: usize, tipo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    let t = tipo as u8;
    let sem_7 = match t {
        1 => Some(true),
        2 => Some(false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => Some(false),
        _ => None,
    };
    let mut plano = [0usize; 71];
    let n = match sem_7 {
        Some(s) if cpz4_w(this + 0x38) <= 71 => cpz4_inscritos(this, s, plano.as_mut_ptr()),
        _ => return o(this, saida, tipo),
    };
    let r = o(this, saida, tipo);
    let v = cpz4_d(saida);
    if v == 0 {
        PLACAR.registrar(r as u16 == 0, || format!("european_cup_collect_club_list({:#x}, {}) sem lista original={:#x}", this, t, r));
        return r;
    }
    let lista = core::slice::from_raw_parts(v as *const usize, (n.max(0) as usize).min(71));
    let igual = r as u16 as i16 == n && lista == &plano[..lista.len()];
    PLACAR.registrar(igual, || format!("european_cup_collect_club_list({:#x}, {}) original={:#x} rust={} lista={:x?} plano={:x?}", this, t, r, n, lista, &plano[..lista.len()]));
    r
}
