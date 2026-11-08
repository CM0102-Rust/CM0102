// ---------- %%LEVA%%a leva: o destino de cada clube no fim das fases do Centro-Oeste e do Goiano, do Mineiro, Paranaense e Carioca, e da NSL (bra_reg_central.cpp, bra_reg_minas_gerais.cpp, aus_nsl.cpp) ----------

unsafe fn cqk7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, já em Rust): 0x86bdd0 classificou-se (clube, comp,
/// fase, rodada, peso; ret 0x14).
unsafe fn cqk7_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cqk7_d(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86a1c0 rebaixado (clube, liga; ret 8).
unsafe fn cqk7_rebaixado(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
    f(cqk7_d(0xdd7f00), clube, comp);
}

/// 0x868c50 campeão da liga (clube, liga; ret 8).
unsafe fn cqk7_campeao(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
    f(cqk7_d(0xdd7f00), clube, comp);
}

/// A fase de pontos corridos (fase -1) dos estaduais: os classificados (6) ganham o registro de classificação (rodada
/// 0x82, peso 0x32), os rebaixados (3) o de rebaixamento. Igual nas duas rotinas dos estaduais.
unsafe fn record_state_league_stage(this: usize, clube: usize, st: u8, fazer: bool) {
    let comp = cqk7_d(this + 4);
    if st == 6 {
        if fazer {
            crate::comparison::branch("cqk_7 classificado do estadual");
            cqk7_classificou(clube, comp, 0, 0x82, 0x32); // cqk7: o classificado do estadual
        }
    } else if st == 3 && fazer {
        crate::comparison::branch("cqk_7 rebaixado do estadual");
        cqk7_rebaixado(clube, comp); // cqk7: o rebaixado do estadual
    }
}

/// O mata-mata dos estaduais: a linha do clube na tabela (+0xb1, +0x3e linhas); o campeão (1) ganha o registro de
/// campeão e o destino 5; os eliminados (2 ou -1), o registro de eliminação (0x86c000) com o peso da escala 0x4bf850
/// (30 a 50 pela rodada do jogo em `rodadas`) e o destino -2. Devolve (linha gravada ou 0, destino).
unsafe fn record_state_knockout(this: usize, clube: usize, st: u8, jogo: usize, rodadas: u32, fazer: bool) -> (usize, u8) {
    let comp = cqk7_d(this + 4);
    let n = cqk7_w(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if cqk7_d(cqk7_d(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    crate::comparison::branch("cqk_7 campeao do estadual");
                    cqk7_campeao(clube, comp); // cqk7: o registro do campeão do estadual
                }
                let l = cqk7_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // ponytail: a original empurra eax (lixo do chamador acima da palavra) e ecx/edx inteiros; a
                    // 0x4bf850 e a 0x86c000 só leem as palavras: vão as palavras.
                    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
                    crate::comparison::branch("cqk_7 eliminado do estadual");
                    let peso = escala(0x1e, 0x32, cqk7_w(jogo + 0x34) as u16 as u32, rodadas);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                    elim(cqk7_d(0xdd7f00), clube, comp, cqk7_w(jogo + 0x32) as u16 as u32, cqk7_w(jogo + 0x30) as u16 as u32, peso); // cqk7: o registro do eliminado
                }
                let l = cqk7_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0)
}

/// O Centro-Oeste e o Goiano (0x4314e0): a fase -1 de pontos corridos e o mata-mata na fase 0 (escala de 3 rodadas).
/// `fazer` = falso: o ensaio do comparador. Devolve (devolução, linha gravada ou 0, destino).
unsafe fn centro_oeste_end_of_stage(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    if fa == 0xff {
        record_state_league_stage(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let (l, d) = record_state_knockout(this, clube, st, jogo, 3, fazer);
    (0, l, d)
}

/// O Mineiro, o Paranaense e o Carioca (0x434d90): a fase -1 de pontos corridos; as fases 0 e 1 (os turnos) devolvem -1;
/// o mata-mata na fase 2 (escala de 1 rodada: a média, 40); as outras, 0.
unsafe fn mineiro_end_of_stage(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    if fa == -1 {
        record_state_league_stage(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa < 2 {
        return (u32::MAX, 0, 0);
    }
    if fa != 2 {
        return (0, 0, 0);
    }
    let (l, d) = record_state_knockout(this, clube, st, jogo, 1, fazer);
    (0, l, d)
}

/// A NSL (0x411980). Devolve (devolução, 0, 0): não grava na tabela.
unsafe fn nsl_end_of_stage(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = cqk7_d(this + 4);
    if fa == 0xff {
        // a temporada regular
        if st == 6 {
            if fazer {
                crate::comparison::branch("cqk_7 nsl classificado");
                cqk7_classificou(clube, comp, 0x435, 0, 0x28); // cqk7: o classificado da NSL
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if fa == 0 {
        // a fase final
        if st == 6 {
            if fazer {
                cqk7_classificou(clube, comp, 0, 0x96, 0x28);
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            if fazer {
                let nao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
                nao(cqk7_d(0xdd7f00), clube, comp, 0x435, 0x28);
            }
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if st == 1 && fazer {
        cqk7_campeao(clube, comp);
    }
    (0, 0, 0)
}

/// { 0x4314e0 } (thiscall, ret 0x18, bra_reg_central.cpp): `BraRegCentral::update_staff_records_for_status` (slot 18
/// do Centro-Oeste e do Goiano): os classificados e rebaixados dos pontos corridos; no mata-mata (fase 0), o campeão
/// (destino 5) e os eliminados (destino -2, registro de eliminação com peso de 30 a 50 em 3 rodadas). Devolve 0.
unsafe extern "thiscall" fn bra_reg_central_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    centro_oeste_end_of_stage(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e o destino na linha.
unsafe extern "thiscall" fn compare_bra_reg_central_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = centro_oeste_end_of_stage(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    SCOREBOARD.record(r == n && gravado == dest, || format!("bra_reg_central_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x434d90 } (thiscall, ret 0x18, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::update_staff_records_for_status`
/// (slot 18 do Mineiro, do Paranaense e do Carioca): os pontos corridos como no Centro-Oeste; os turnos (fases 0 e 1)
/// devolvem -1; o mata-mata na fase 2 (peso pela escala de uma rodada só: 40). Devolve 0 ou -1.
unsafe extern "thiscall" fn bra_reg_minas_gerais_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mineiro_end_of_stage(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = mineiro_end_of_stage(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    SCOREBOARD.record(r == n && gravado == dest, || format!("bra_reg_minas_gerais_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x411980 } (thiscall, ret 0x18, aus_nsl.cpp): `AusNsl::update_staff_records_for_status`: na temporada regular, os
/// seis classificados (6) ganham o registro de classificação para a fase final (fase 0x435); os de fora (-2) devolvem
/// -1; na fase final (fase 0), quem avança (6) ganha o registro (rodada 0x96) e quem cai (-2) o de não classificado
/// (fase 0x435) e -1; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
unsafe extern "thiscall" fn aus_nsl_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    nsl_end_of_stage(this, clube, situacao, fase, true).0
}

/// Comparador: só a original (os registros alocam) com a devolução prevista pelo ensaio.
unsafe extern "thiscall" fn compare_aus_nsl_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, _, _) = nsl_end_of_stage(this, clube, situacao, fase, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I2%%)(this, clube, situacao, fase, a4, a5, a6);
    SCOREBOARD.record(r == n, || format!("aus_nsl_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}
