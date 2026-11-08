// ---------- %%LEVA%%a leva: a MLS: o destino de cada clube no fim das fases, o resultado nas tabelas das duas conferências e a conferência do Jogo das Estrelas (usa_mls.cpp) ----------

unsafe fn cqk9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// 0x86bdd0 Staff_Records::add_qualified_for_round_record (thiscall em [0xdd7f00], ret 0x14; já em Rust).
unsafe fn cqk9_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cqk9_d(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86c1d0 Staff_Records::add_failed_to_qualify_record (thiscall em [0xdd7f00], ret 0x10; já em Rust).
unsafe fn usa_mls_add_failed_to_qualify_record(clube: usize, comp: usize, fase: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
    f(cqk9_d(0xdd7f00), clube, comp, fase, peso);
}

/// 0x4bf850 comp_util_scale_by_round (cdecl): o peso de 50 a 70 pela rodada do jogo, em 2 rodadas.
unsafe fn weight_by_round(jogo: usize) -> u32 {
    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
    escala(0x32, 0x46, cqk9_w(jogo + 0x34) as u16 as u32, 2)
}

/// A MLS no fim das fases (0x908200). `fazer` = falso: o ensaio do comparador (só a devolução).
unsafe fn mls_stage_outcome(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> u32 {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    let comp = cqk9_d(this + 4);
    if fa < 2 {
        // a temporada (-1) e as duas primeiras rodadas dos play-offs
        if st == 6 {
            if fazer {
                crate::comparison::branch("cqk_9 mls classificado");
                cqk9_classificou(clube, comp, 0xa0, 0x78, 0x28); // cqk9: o classificado da MLS
            }
            return 0;
        }
        if st != 0xfe {
            return 0;
        }
        if fazer {
            let f = if fa == -1 { 0x413 } else if fa != 0 { 0x414 } else { 0x47b };
            usa_mls_add_failed_to_qualify_record(clube, comp, f, 0x28);
        }
        return u32::MAX;
    }
    if fa == 2 {
        // a final
        if st == 1 {
            if fazer {
                cqk9_classificou(clube, comp, 0x417, 0x96, 0x50);
            }
        } else if st == 0 {
            if fazer {
                // ponytail: a original empurra eax/ecx/edx inteiros (lixo acima das palavras: o do chamador, a metade
                // de cima do this); a 0x4bf850 e a 0x86bdd0 só leem as palavras: vão as palavras.
                let peso = weight_by_round(jogo);
                cqk9_classificou(clube, comp, cqk9_w(jogo + 0x32) as u16 as u32, cqk9_w(jogo + 0x30) as u16 as u32, peso);
            }
        } else if st == 0xff || st == 2 {
            if fazer {
                let peso = weight_by_round(jogo);
                let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                elim(cqk9_d(0xdd7f00), clube, comp, cqk9_w(jogo + 0x32) as u16 as u32, cqk9_w(jogo + 0x30) as u16 as u32, peso);
            }
        }
        return 0;
    }
    if st == 1 && fazer {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
        f(cqk9_d(0xdd7f00), clube, comp);
    }
    0
}

/// { 0x908200 } (thiscall, ret 0x18, usa_mls.cpp): `UsaMls::update_staff_records_for_status` (slot 18): na temporada e
/// nas fases 0 e 1, quem avança (6) ganha o registro de classificação (fase 0xa0, rodada 0x78) e quem fica de fora (-2) o
/// de não classificado (fase 0x413 na temporada, 0x47b na fase 0, 0x414 na 1) e -1; na fase 2, o vencedor (1) ganha
/// o registro da final (0x417, 0x96, peso 0x50), quem passa (0) o da rodada do jogo e os eliminados (-1, 2) o de
/// eliminação, com peso de 50 a 70 pela rodada; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
unsafe extern "thiscall" fn usa_mls_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mls_stage_outcome(this, clube, situacao, fase, jogo, true)
}

/// Comparador: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com a
/// devolução prevista pelo ensaio.
unsafe extern "thiscall" fn comparar_usa_mls_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let n = mls_stage_outcome(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, clube, situacao, fase, a4, jogo, a6);
    SCOREBOARD.record(r == n, || format!("usa_mls_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}

/// { 0x9099d0 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::add_result(jogo, estatisticas)` (slot 2): o jogo vai para a
/// tabela geral (`League::add_result` 0x685d30) com a marca +0x42 do jogo em -1 quando abaixo de 2; no estágio 1 (a
/// temporada regular), vai também para as tabelas das duas conferências (os estágios 0 e 1, pelo slot 2 deles, com a
/// marca 0 e 1 e sem o bloco de estatísticas +8, guardado e devolvido). Devolve 1.
unsafe extern "thiscall" fn usa_mls_add_result(this: usize, jogo: usize, estat: u32) -> u32 {
    if (*((jogo + 0x42) as *const i8)) < 2 {
        *((jogo + 0x42) as *mut u8) = 0xff;
    }
    let geral: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x685d30usize);
    geral(this, jogo, estat);
    if cqk9_d(this + 0x30) != 1 {
        return 1;
    }
    let guardado = cqk9_d(this + 8);
    *((this + 8) as *mut u32) = 0;
    *((jogo + 0x42) as *mut u8) = 0;
    let estagio = |k: usize| -> (usize, unsafe extern "thiscall" fn(usize, usize, u32) -> u32) {
        let e = cqk9_d(cqk9_d(this + 0xc) + 4 * k);
        (e, core::mem::transmute(cqk9_d(cqk9_d(e) + 4)))
    };
    let (e0, f0) = estagio(0);
    f0(e0, jogo, estat);
    *((jogo + 0x42) as *mut u8) = 1;
    let (e1, f1) = estagio(1);
    crate::comparison::branch("cqk_9 mls segunda conferencia");
    f1(e1, jogo, estat); // cqk9: o jogo na tabela da segunda conferência
    *((this + 8) as *mut u32) = guardado as u32;
    1
}

/// Comparador: só a original (ponytail: o resultado entra nas tabelas; não roda duas vezes), com o plano: a devolução
/// 1, a marca +0x42 do jogo (1 no estágio 1; senão -1 se estava abaixo de 2) e o bloco +8 devolvido.
unsafe extern "thiscall" fn comparar_usa_mls_add_result(this: usize, jogo: usize, estat: u32) -> u32 {
    let marca = *((jogo + 0x42) as *const i8);
    let bloco = cqk9_d(this + 8);
    let esperada = if cqk9_d(this + 0x30) == 1 { 1 } else if marca < 2 { -1 } else { marca };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, jogo, estat);
    let (m, b) = (*((jogo + 0x42) as *const i8), cqk9_d(this + 8));
    SCOREBOARD.record(r == 1 && m == esperada && b == bloco, || format!("usa_mls_add_result({:x}, {:x}) devolveu {} marca {} x {} bloco {:x} x {:x}", this, jogo, r, m, esperada, b, bloco));
    r
}

/// { 0x90a000 } (cdecl, usa_mls.cpp): `usa_mls_get_club_all_star_conference(clube)`: a conferência do clube no Jogo das
/// Estrelas: 0x413 (o Leste) para New England, MetroStars, Tampa Bay, D.C. United, Columbus e Miami (as chaves 0x9d0390,
/// 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380), 0x414 (o Oeste) para Colorado, Dallas, Kansas City, Los Angeles,
/// San Jose e Chicago (0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c); 0 para clube cuja liga (+0x57) não
/// é a MLS (chave 0x9cf590); clube da MLS fora dos doze: a caixa "Error" (a original).
unsafe extern "C" fn usa_mls_get_club_all_star_conference(clube: usize) -> u16 {
    let nacao = cqk9_d(clube + 0x57);
    if nacao == 0 || cqk9_d(nacao) != cqk9_d(0x9cf590) {
        return 0;
    }
    let id = cqk9_d(clube);
    const LESTE: [usize; 6] = [0x9d0390, 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380];
    const OESTE: [usize; 6] = [0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c];
    if LESTE.iter().any(|&k| cqk9_d(k) == id) {
        crate::comparison::branch("cqk_9 conferencia 0x413");
        return 0x413; // cqk9: a primeira conferência
    }
    if OESTE.iter().any(|&k| cqk9_d(k) == id) {
        return 0x414;
    }
    original_fn::<unsafe extern "C" fn(usize) -> u16>(%%I2%%)(clube)
}

/// Comparador: a devolução (só lê; o caso de erro fica com a original sozinha).
unsafe extern "C" fn comparar_usa_mls_get_club_all_star_conference(clube: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(%%I2%%)(clube);
    let nacao = cqk9_d(clube + 0x57);
    if nacao != 0 && cqk9_d(nacao) == cqk9_d(0x9cf590) && r == 0 {
        return r; // o caso de erro (a caixa já saiu na original)
    }
    let n = usa_mls_get_club_all_star_conference(clube);
    SCOREBOARD.record(r == n, || format!("usa_mls_get_club_all_star_conference({:x}) original={:x} rust={:x}", clube, r, n));
    r
}
