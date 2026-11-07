// ---------- 284a leva: o dia da Copa do Rei (a copa refeita uma vez no ano dela) e o histórico da temporada de quatro copas (o da última fase): a Liga dos Campeões, a Copa da Grécia, a Copa da Liga irlandesa e a Copa da Série C italiana (spa_cup.cpp, european_cup.cpp, gre_cup.cpp, ire_lge_cup.cpp, ita_c_cup.cpp) ----------



/// A virtual `o` de `this` (thiscall, sem argumentos).
unsafe fn cqt5_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + o));
    f(this)
}

/// { 0x84d740 } (thiscall, ret 8, spa_cup.cpp; a virtual +0x14 da Copa do Rei, todo dia): `SpaCup::update_for_date(hoje,
/// fase)`. Uma vez (+0x45 = 0), quando o ano da classificação da UEFA ([0xde1f58] +0x439, palavra sem sinal) é o da copa
/// (+0x40, com sinal) e a fase não é 0: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos de cada
/// rodada (+0xad, +0x3c rodadas) liberados (0x9452ca; os ponteiros ficam), +0x45 = 1, os inscritos (SpaCup::
/// collect_entrants 0x84d910, thiscall), as rodadas (virtual +0x8c) e a árvore (virtual +0x94) — uma devolução negativa
/// dá a caixa (linhas 0x161, 0x168, 0x16f) e 0 — e as posições previstas (virtual +0x5c). Sempre, no fim, a atualização da
/// copa (0x51f890: push fase; push hoje; mov ecx -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn cqt5_copa_do_rei_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if read_u32_as_usize(this + 0x45) == 0 && read_i16(read_u32_as_usize(0xde1f58) + 0x439) as u16 as i32 == read_i16(this + 0x40) as i32 && fase != 0 { // cqt5: o ano da Copa do Rei
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        for o in [0xa7usize, 0xa3] {
            if read_u32_as_usize(this + o) != 0 {
                soltar(read_u32_as_usize(this + o));
            }
        }
        if read_u32_as_usize(this + 0xad) != 0 {
            let mut i = 0i32;
            while i < read_i16(this + 0x3c) as i32 {
                let p = read_u32_as_usize(read_u32_as_usize(this + 0xad) + i as usize * 4);
                if p != 0 {
                    soltar(p);
                }
                i += 1;
            }
            soltar(read_u32_as_usize(this + 0xad));
        }
        crate::comparacao::ramo("cqt_5 copa do rei refeita");
        core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqt5: a Copa do Rei refeita uma vez
        let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_SPACUP_COLLECT_ENTRANTS);
        // ponytail: nas caixas a original zera só o al (o resto do eax vem da caixa); quem chama lê o byte
        if inscritos(this) < 0 {
            aviso_em(0xa82810, 0x161);
            return 0;
        }
        if cqt5_virtual(this, 0x8c) < 0 {
            aviso_em(0xa82810, 0x168);
            return 0;
        }
        if cqt5_virtual(this, 0x94) < 0 {
            aviso_em(0xa82810, 0x16f);
            return 0;
        }
        cqt5_virtual(this, 0x5c);
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this, hoje, fase)
}

// ponytail: a virada libera e refaz as tabelas e a atualização da copa joga e sorteia: não rodam duas vezes. Só a
// original; o plano: +0x45 vai a 1 quando a condição vale, senão fica.
unsafe extern "thiscall" fn comparar_cqt5_copa_do_rei_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2087);
    let antes = read_u32_as_usize(this + 0x45);
    let vira = antes == 0 && read_i16(read_u32_as_usize(0xde1f58) + 0x439) as u16 as i32 == read_i16(this + 0x40) as i32 && fase != 0;
    let r = o(this, hoje, fase);
    let depois = read_u32_as_usize(this + 0x45);
    PLACAR.registrar(depois == if vira { 1 } else { antes }, || format!("cqt5_copa_do_rei_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// O histórico da temporada de uma copa (`record_season_honours`, 11 bytes: mov eax, [ecx+0xc]; mov ecx, [eax+o]; mov
/// edx, [ecx]; jmp [edx+0x30]): a virtual +0x30 da fase [+0xc][o/4] (a última fase da copa: a final), com o eax dela.
unsafe fn cqt5_historico(this: usize, o: usize, ramo: &'static str) -> i32 {
    crate::comparacao::ramo(ramo);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + o);
    cqt5_virtual(fase, 0x30)
}

/// { 0x584ad0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::record_season_honours()`: o histórico da fase 12
/// (+0x30: a final da Liga dos Campeões).
unsafe extern "thiscall" fn cqt5_historico_liga_dos_campeoes(this: usize) -> i32 {
    cqt5_historico(this, 0x30, "cqt_5 historico da liga dos campeoes")
}

/// { 0x5e93e0 } (thiscall, ret 0, gre_cup.cpp): `GreCup::record_season_honours()`: o histórico da fase 10 (+0x28).
unsafe extern "thiscall" fn cqt5_historico_copa_da_grecia(this: usize) -> i32 {
    cqt5_historico(this, 0x28, "cqt_5 historico da copa da grecia")
}

/// { 0x637750 } (thiscall, ret 0, ire_lge_cup.cpp): `IreLgeCup::record_season_honours()`: o histórico da fase 7 (+0x1c).
unsafe extern "thiscall" fn cqt5_historico_copa_da_liga_irlandesa(this: usize) -> i32 {
    cqt5_historico(this, 0x1c, "cqt_5 historico da copa da liga irlandesa")
}

/// { 0x63d770 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::record_season_honours()`: o histórico da fase 15 (+0x3c: o
/// mata-mata da Copa da Série C).
unsafe extern "thiscall" fn cqt5_historico_copa_da_serie_c(this: usize) -> i32 {
    cqt5_historico(this, 0x3c, "cqt_5 historico da copa da serie c")
}

// ponytail: o histórico grava nas estatísticas das competições (acha ou acrescenta entradas: aloca); só a original
unsafe extern "thiscall" fn comparar_cqt5_historico_liga_dos_campeoes(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2088)(this)
}

unsafe extern "thiscall" fn comparar_cqt5_historico_copa_da_grecia(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2089)(this)
}

unsafe extern "thiscall" fn comparar_cqt5_historico_copa_da_liga_irlandesa(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2090)(this)
}

unsafe extern "thiscall" fn comparar_cqt5_historico_copa_da_serie_c(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2091)(this)
}

