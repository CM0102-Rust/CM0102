// ---------- %%LEVA%%a leva: a virada das copas: a Copa do Brasil, a Copa do Imperador e a Copa da Liga japonesa (a temporada nova: as tabelas liberadas, o ano, os inscritos, as rodadas e a árvore) (bra_cup.cpp, jap_emp_cup.cpp, jap_j_cup.cpp) ----------

unsafe fn cqr5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov edx, [esi]; mov ecx, esi; call [edx + o]).
unsafe fn call_cup_virtual(this: usize, o: usize) -> i32 {
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
unsafe fn start_new_cup_season(this: usize, inscritos: usize, arquivo: usize, linhas: [u32; 3], ano: i16, ramo: &'static str) -> u32 {
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
        show_error_at(arquivo, linhas[0]);
        return 0;
    }
    if call_cup_virtual(this, 0x8c) < 0 {
        show_error_at(arquivo, linhas[1]);
        return 0;
    }
    if call_cup_virtual(this, 0x94) < 0 {
        show_error_at(arquivo, linhas[2]);
        return 0;
    }
    call_cup_virtual(this, 0x5c) as u32
}

/// O comparador da virada: só a original (ponytail: libera as tabelas e refaz os inscritos, as rodadas e a árvore, que
/// alocam e sorteiam; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn compare_cup_new_season(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = cqr5_w(this + 0x40);
    let r = original();
    let depois = cqr5_w(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// { 0x427d10 } (thiscall, ret 0, bra_cup.cpp; a virtual +0x08 da Copa do Brasil): `BraCup::start_new_season()`, a
/// virada (`start_new_cup_season`) com os inscritos de BraCup::build_team_table (0x427c20) e as caixas do bra_cup.cpp nas linhas
/// 0x189, 0x190 e 0x197.
unsafe extern "thiscall" fn bra_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(this, 0x427c20, 0x98a3bc, [0x189, 0x190, 0x197], 1, "cqr_5 virada da copa do brasil") // cqr5: a Copa do Brasil
}

unsafe extern "thiscall" fn comparar_bra_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("bra_cup_start_new_season", this, || o(this))
}

/// { 0x662950 } (thiscall, ret 0, jap_emp_cup.cpp; a virtual +0x08 da Copa do Imperador): `JapEmpCup::start_new_season()`,
/// a virada (`start_new_cup_season`) com os inscritos de JapEmpCup::collect_entrants (0x6624f0) e as caixas do jap_emp_cup.cpp nas
/// linhas 0x217, 0x21e e 0x225.
unsafe extern "thiscall" fn bra_cup_start_new_season_662950(this: usize) -> u32 {
    start_new_cup_season(this, 0x6624f0, 0x9eab90, [0x217, 0x21e, 0x225], 1, "cqr_5 virada da copa do imperador") // cqr5: a Copa do Imperador
}

unsafe extern "thiscall" fn comparar_bra_cup_start_new_season_662950(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("bra_cup_start_new_season_662950", this, || o(this))
}

/// { 0x665bc0 } (thiscall, ret 0, jap_j_cup.cpp; a virtual +0x08 da Copa da Liga japonesa): `JapJCup::start_new_season()`,
/// a virada (`start_new_cup_season`) com os inscritos de 0x665a00 (thiscall, ret 0) e as caixas do jap_j_cup.cpp nas linhas 0x1a3,
/// 0x1aa e 0x1b1.
unsafe extern "thiscall" fn jap_j_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(this, 0x665a00, 0x9eaf14, [0x1a3, 0x1aa, 0x1b1], 1, "cqr_5 virada da copa da liga japonesa") // cqr5: a Copa da Liga japonesa
}

unsafe extern "thiscall" fn comparar_jap_j_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("jap_j_cup_start_new_season", this, || o(this))
}
