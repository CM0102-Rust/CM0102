// ---------- 282a leva: a virada das copas: a Copa do Brasil, a Copa do Imperador e a Copa da Liga japonesa (a temporada nova: as tabelas liberadas, o ano, os inscritos, as rodadas e a árvore) (bra_cup.cpp, jap_emp_cup.cpp, jap_j_cup.cpp) ----------

unsafe fn cqr5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov edx, [esi]; mov ecx, esi; call [edx + o]).
unsafe fn cqr5_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqr5_d(cqr5_d(this) + o));
    f(this)
}

/// A virada da temporada de uma copa de mata-mata (`start_new_season`, slot 3, +0x08; o mesmo molde nas copas, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c, create_round_table pela fase
/// −1) e a árvore dos confrontos (virtual +0x94); por fim as posições previstas (virtual +0x5c), cujo eax ela devolve.
unsafe fn cqr5_virada(this: usize, inscritos: usize, arquivo: usize, linhas: [u32; 3], ano: i16, ramo: &'static str) -> u32 {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    for o in [0xa7usize, 0xa3] {
        let l = cqr5_d(this + o);
        if l != 0 {
            soltar(l);
            core::ptr::write_unaligned((this + o) as *mut u32, 0);
        }
    }
    if cqr5_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqr5_w(this + 0x3c) as i32 {
            let p = cqr5_d(cqr5_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
                core::ptr::write_unaligned((cqr5_d(this + 0xad) + i as usize * 4) as *mut u32, 0);
            }
            i += 1;
        }
        soltar(cqr5_d(this + 0xad));
        core::ptr::write_unaligned((this + 0xad) as *mut u32, 0);
    }
    let e = cqr5_d(this + 8);
    if e != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(e, 1);
    }
    let a = cqr5_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    *((this + 0xab) as *mut u8) = 0;
    *((this + 0xb1) as *mut u8) = 0;
    let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(inscritos);
    // ponytail: nas caixas a original sai com o eax da caixa (0x5e8290); quem chama a virada não lê a devolução
    if inscritos(this) < 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    if cqr5_virtual(this, 0x8c) < 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    if cqr5_virtual(this, 0x94) < 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    cqr5_virtual(this, 0x5c) as u32
}

/// O comparador da virada: só a original (ponytail: libera as tabelas e refaz os inscritos, as rodadas e a árvore, que
/// alocam e sorteiam; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqr5_comparar(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = cqr5_w(this + 0x40);
    let r = original();
    let depois = cqr5_w(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// { 0x427d10 } (thiscall, ret 0, bra_cup.cpp; a virtual +0x08 da Copa do Brasil): `BraCup::start_new_season()`, a
/// virada (`cqr5_virada`) com os inscritos de BraCup::build_team_table (0x427c20) e as caixas do bra_cup.cpp nas linhas
/// 0x189, 0x190 e 0x197.
unsafe extern "thiscall" fn cqr5_virada_copa_do_brasil(this: usize) -> u32 {
    cqr5_virada(this, 0x427c20, 0x98a3bc, [0x189, 0x190, 0x197], 1, "cqr_5 virada da copa do brasil") // cqr5: a Copa do Brasil
}

unsafe extern "thiscall" fn comparar_cqr5_virada_copa_do_brasil(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1999);
    cqr5_comparar("cqr5_virada_copa_do_brasil", this, || o(this))
}

/// { 0x662950 } (thiscall, ret 0, jap_emp_cup.cpp; a virtual +0x08 da Copa do Imperador): `JapEmpCup::start_new_season()`,
/// a virada (`cqr5_virada`) com os inscritos de JapEmpCup::collect_entrants (0x6624f0) e as caixas do jap_emp_cup.cpp nas
/// linhas 0x217, 0x21e e 0x225.
unsafe extern "thiscall" fn cqr5_virada_copa_do_imperador(this: usize) -> u32 {
    cqr5_virada(this, 0x6624f0, 0x9eab90, [0x217, 0x21e, 0x225], 1, "cqr_5 virada da copa do imperador") // cqr5: a Copa do Imperador
}

unsafe extern "thiscall" fn comparar_cqr5_virada_copa_do_imperador(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2000);
    cqr5_comparar("cqr5_virada_copa_do_imperador", this, || o(this))
}

/// { 0x665bc0 } (thiscall, ret 0, jap_j_cup.cpp; a virtual +0x08 da Copa da Liga japonesa): `JapJCup::start_new_season()`,
/// a virada (`cqr5_virada`) com os inscritos de 0x665a00 (thiscall, ret 0) e as caixas do jap_j_cup.cpp nas linhas 0x1a3,
/// 0x1aa e 0x1b1.
unsafe extern "thiscall" fn cqr5_virada_copa_da_liga_japonesa(this: usize) -> u32 {
    cqr5_virada(this, 0x665a00, 0x9eaf14, [0x1a3, 0x1aa, 0x1b1], 1, "cqr_5 virada da copa da liga japonesa") // cqr5: a Copa da Liga japonesa
}

unsafe extern "thiscall" fn comparar_cqr5_virada_copa_da_liga_japonesa(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2001);
    cqr5_comparar("cqr5_virada_copa_da_liga_japonesa", this, || o(this))
}

// ---------- 279a leva: a Copa do Brasil: as rodadas, a tabela dos times e as regras dos jogos (bra_cup.cpp) ----------

/// A Copa do Brasil: sete rodadas em ida e volta de fevereiro a julho do ano da copa (o jogo de volta 14 dias depois do
/// de ida até as oitavas, 21 nas quartas e 7 na semifinal; 4 dias nas três últimas, ao meio da semana [inferência]).
const CQO12_COPA_DO_BRASIL: [Cql10R; 7] = [
    cql10r((0x12, 0, 0, 6), (0x17, 1, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x36, 0x1b, 0x36, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo12: a 1ª rodada da Copa do Brasil
    cql10r((9, 2, 0, 3), (0x16, 2, 0, 2, 2, 0), 0x14, (0, 0, 1), 4, (0x22, 0x11, 7, 0x36), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((6, 3, 0, 3), (0x1a, 3, 0, 2, 2, 0), 0x1e, (0, 0, 1), 6, (0x16, 0xb, 5, 0x3d), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
    cql10r((0x12, 4, 0, 3), (7, 5, 0, 2, 2, 0), 0x28, (0, 0, 1), 6, (0x10, 8, 5, 0x42), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xf, 5, 0, 3), (0x11, 5, 0, 5, 1, 0), 0x78, (0, 0, 1), 6, (8, 4, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x16, 5, 0, 3), (0x18, 5, 0, 5, 1, 0), 0x82, (0, 0, 1), 6, (4, 2, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 5, 0, 3), (1, 6, 0, 5, 1, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 4), (0, 0xf4240, 0xc3500)),
];

/// { 0x427700 } (thiscall, ret 0x10, bra_cup.cpp; a virtual +0x3c da Copa do Brasil): `BraCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa4 sem memória.
unsafe extern "thiscall" fn cqo12_copa_do_brasil_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQO12_COPA_DO_BRASIL, 0x98a3bc, 0xa4, "cqo_12 tabela da copa do brasil")
}

unsafe extern "thiscall" fn comparar_cqo12_copa_do_brasil_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1915);
    cql10_comparar("cqo12_copa_do_brasil_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqo12_copa_do_brasil_tabela(this, fase, r, e, s))
}

unsafe fn cqo12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x427c20 } (thiscall, ret 0, bra_cup.cpp): `BraCup::build_team_table()`: +0x3e = 71 e a tabela dos times (0x944e46
/// de 0x1aa em +0xa7; sem memória, a caixa da linha 0x143 e −1); cada vaga i (6 bytes) recebe o clube da inscrição i da
/// lista +0x14 (6 bytes), a situação +5 = 0xff e a classe +4 = 0. Devolve 1.
unsafe fn cqo12_times(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x47);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x1aa);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x98a3bc, 0x143);
        return -1;
    }
    crate::comparacao::ramo("cqo_12 times da copa do brasil");
    let mut i = 0i16;
    while i < cqo12_w(this + 0x3e) {
        let o = i as usize * 6;
        let fonte = o; // cqo12: a inscrição de cada vaga
        let c = cqo12_d(cqo12_d(this + 0x14) + fonte);
        let e = cqo12_d(this + 0xa7) + o;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 5) as *mut u8) = 0xff;
        *((e + 4) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

unsafe extern "thiscall" fn cqo12_times_da_copa(this: usize) -> i32 {
    cqo12_times(this)
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 71 vagas (com a devolução 1) e liberada.
unsafe extern "thiscall" fn comparar_cqo12_times_da_copa(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1916);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqo12_d(this + 0xa7));
        r
    }, || {
        let r = cqo12_times(this);
        lr.set(cqo12_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { cqo12_w(this + 0x3e).max(0) as usize * 6 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqo12_times_da_copa({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x427ec0 } (thiscall, ret 0, bra_cup.cpp; uma virtual da copa): `BraCup::set_up_rounds()`: 7 reservas (+0x49), 3
/// substituições (+0x4a), o fundo de prêmios de 500 mil (+0x28) e a tabela das rodadas (a virtual +0x3c com a fase −1,
/// as rodadas em +0x3c e o extra em +0x3a, sem saída) em +0xa3. Devolve 1.
unsafe fn cqo12_regras(this: usize) -> i32 {
    crate::comparacao::ramo("cqo_12 regras da copa do brasil");
    *((this + 0x49) as *mut u8) = 7;
    *((this + 0x4a) as *mut u8) = 3; // cqo12: as substituições da Copa do Brasil
    core::ptr::write_unaligned((this + 0x28) as *mut u32, 0x7a120);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(cqo12_d(cqo12_d(this) + 0x3c));
    let t = tabela(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xa3) as *mut u32, t as u32);
    1
}

unsafe extern "thiscall" fn cqo12_regras_da_copa(this: usize) -> i32 {
    cqo12_regras(this)
}

/// Lado a lado: +0x28, +0x3a..+0x3d, +0x49/+0x4a e o [0xb67a34] a partir do mesmo estado; as tabelas das rodadas das
/// duas (+0xa3) comparadas nos bytes gravados (cql10_gravados) e a da cópia liberada.
unsafe extern "thiscall" fn comparar_cqo12_regras_da_copa(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1917);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let antes = cqo12_d(this + 0xa3);
    let reg = [(this + 0x28, 4usize), (this + 0x3a, 4), (this + 0x49, 2), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        to.set(cqo12_d(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, antes as u32);
        r
    }, || {
        let r = cqo12_regras(this);
        tr.set(cqo12_d(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = cqo12_w(this + 0x3c);
    let tabela = if to != 0 && tr != 0 { cql10_gravados(to, n) == cql10_gravados(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(tr);
    }
    PLACAR.registrar(igual && a == b && tabela, || format!("cqo12_regras_da_copa({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}

