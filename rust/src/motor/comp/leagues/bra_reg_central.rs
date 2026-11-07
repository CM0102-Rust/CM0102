// ---------- 274a leva: as regras dos estaduais do Centro-Oeste, Gaúcho (Nordeste, Paulista), Baiano e Catarinense (bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_santa.cpp) ----------




/// As regras comuns dos estaduais (as três rotinas gravam o mesmo conjunto; mudam os turnos +0x3c, as vagas de cima
/// +0xbe e os rebaixados +0xc1): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates confronto
/// direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb (copiadas em cada partida: +0x38 e +0x36) em 0.
unsafe fn cqi8_regras(o: usize, turnos: u16, cima: u8, rebaixados: u8) {
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0xc1, rebaixados);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xbe, cima);
    write_u16_at(o, 0xd9, 8);
    write_u16_at(o, 0x3c, turnos);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
}

/// O Centro-Oeste (0x430ff0): turno único, 8 vagas de cima (o mata-mata de 8), 3 rebaixados.
unsafe fn cqi8_regras_centro(o: usize) {
    cqi8_regras(o, 1, 8, 3); // cqi8: o Centro-Oeste
}

/// O Gaúcho (0x4322d0, também o Nordeste e o Paulista): turno único, 4 vagas de cima, 2 rebaixados.
unsafe fn cqi8_regras_gaucho(o: usize) {
    cqi8_regras(o, 1, 4, 2); // cqi8: o Gaúcho
}

/// O Catarinense (0x43bc70, também o Baiano): dois turnos, 4 vagas de cima, 1 rebaixado.
unsafe fn cqi8_regras_catarinense(o: usize) {
    cqi8_regras(o, 2, 4, 1); // cqi8: o Catarinense
}

/// O calendário da fase principal pelo slot 16 (`create_schedule`, thiscall, ret 0x10, conferido nos seis alvos das
/// vtables destas rotinas): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqi8_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    write_u32_at(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os calendários não gravam nada
/// no objeto além delas (nenhum `mov [edi+..]` nos seis).
fn cqi8_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// { 0x430ff0 } (thiscall, bra_reg_central.cpp): `BraRegCentral::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    cqi8_regras_centro(this);
    crate::comparacao::ramo("cqi_8 regras do centro-oeste");
    cqi8_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1715)(this), || {
        cqi8_regras_centro(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_centro_oeste({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x4322d0 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::set_up_rules`, nas vtables do Gaúcho (0x96811c), do
/// Nordeste (0x9683ec) e do Paulista (0x968770); devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_gaucho(this: usize) -> u32 {
    cqi8_regras_gaucho(this);
    crate::comparacao::ramo("cqi_8 regras do gaucho");
    cqi8_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_gaucho(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1716)(this), || {
        cqi8_regras_gaucho(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_gaucho({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x43bc70 } (thiscall, bra_reg_santa.cpp): `BraRegSanta::set_up_rules`, nas vtables do Baiano (0x967fb4) e do
/// Catarinense (0x9686bc); devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_catarinense(this: usize) -> u32 {
    cqi8_regras_catarinense(this);
    crate::comparacao::ramo("cqi_8 regras do catarinense");
    cqi8_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_catarinense(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1717)(this), || {
        cqi8_regras_catarinense(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_catarinense({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- 274a leva: o destino de cada clube no fim das fases do Centro-Oeste e do Goiano, do Mineiro, Paranaense e Carioca, e da NSL (bra_reg_central.cpp, bra_reg_minas_gerais.cpp, aus_nsl.cpp) ----------



/// Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, já em Rust): 0x86bdd0 classificou-se (clube, comp,
/// fase, rodada, peso; ret 0x14).
unsafe fn cqk7_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(read_u32_as_usize(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86a1c0 rebaixado (clube, liga; ret 8).
unsafe fn cqk7_rebaixado(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
    f(read_u32_as_usize(0xdd7f00), clube, comp);
}

/// 0x868c50 campeão da liga (clube, liga; ret 8).
unsafe fn cqk7_campeao(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
    f(read_u32_as_usize(0xdd7f00), clube, comp);
}

/// A fase de pontos corridos (fase -1) dos estaduais: os classificados (6) ganham o registro de classificação (rodada
/// 0x82, peso 0x32), os rebaixados (3) o de rebaixamento. Igual nas duas rotinas dos estaduais.
unsafe fn cqk7_pontos_corridos(this: usize, clube: usize, st: u8, fazer: bool) {
    let comp = read_u32_as_usize(this + 4);
    if st == 6 {
        if fazer {
            crate::comparacao::ramo("cqk_7 classificado do estadual");
            cqk7_classificou(clube, comp, 0, 0x82, 0x32); // cqk7: o classificado do estadual
        }
    } else if st == 3 && fazer {
        crate::comparacao::ramo("cqk_7 rebaixado do estadual");
        cqk7_rebaixado(clube, comp); // cqk7: o rebaixado do estadual
    }
}

/// O mata-mata dos estaduais: a linha do clube na tabela (+0xb1, +0x3e linhas); o campeão (1) ganha o registro de
/// campeão e o destino 5; os eliminados (2 ou -1), o registro de eliminação (0x86c000) com o peso da escala 0x4bf850
/// (30 a 50 pela rodada do jogo em `rodadas`) e o destino -2. Devolve (linha gravada ou 0, destino).
unsafe fn cqk7_mata_mata(this: usize, clube: usize, st: u8, jogo: usize, rodadas: u32, fazer: bool) -> (usize, u8) {
    let comp = read_u32_as_usize(this + 4);
    let n = read_i16(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if read_u32_as_usize(read_u32_as_usize(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    crate::comparacao::ramo("cqk_7 campeao do estadual");
                    cqk7_campeao(clube, comp); // cqk7: o registro do campeão do estadual
                }
                let l = read_u32_as_usize(this + 0xb1) + 0x3b * i as usize;
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
                    crate::comparacao::ramo("cqk_7 eliminado do estadual");
                    let peso = escala(0x1e, 0x32, read_i16(jogo + 0x34) as u16 as u32, rodadas);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                    elim(read_u32_as_usize(0xdd7f00), clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso); // cqk7: o registro do eliminado
                }
                let l = read_u32_as_usize(this + 0xb1) + 0x3b * i as usize;
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
unsafe fn cqk7_centro(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    if fa == 0xff {
        cqk7_pontos_corridos(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let (l, d) = cqk7_mata_mata(this, clube, st, jogo, 3, fazer);
    (0, l, d)
}

/// O Mineiro, o Paranaense e o Carioca (0x434d90): a fase -1 de pontos corridos; as fases 0 e 1 (os turnos) devolvem -1;
/// o mata-mata na fase 2 (escala de 1 rodada: a média, 40); as outras, 0.
unsafe fn cqk7_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    if fa == -1 {
        cqk7_pontos_corridos(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa < 2 {
        return (u32::MAX, 0, 0);
    }
    if fa != 2 {
        return (0, 0, 0);
    }
    let (l, d) = cqk7_mata_mata(this, clube, st, jogo, 1, fazer);
    (0, l, d)
}

/// A NSL (0x411980). Devolve (devolução, 0, 0): não grava na tabela.
unsafe fn cqk7_nsl(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = read_u32_as_usize(this + 4);
    if fa == 0xff {
        // a temporada regular
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqk_7 nsl classificado");
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
                nao(read_u32_as_usize(0xdd7f00), clube, comp, 0x435, 0x28);
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
unsafe extern "thiscall" fn cqk7_destino_no_centro_oeste(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqk7_centro(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e o destino na linha.
unsafe extern "thiscall" fn comparar_cqk7_destino_no_centro_oeste(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqk7_centro(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1731)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqk7_destino_no_centro_oeste({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x434d90 } (thiscall, ret 0x18, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::update_staff_records_for_status`
/// (slot 18 do Mineiro, do Paranaense e do Carioca): os pontos corridos como no Centro-Oeste; os turnos (fases 0 e 1)
/// devolvem -1; o mata-mata na fase 2 (peso pela escala de uma rodada só: 40). Devolve 0 ou -1.
unsafe extern "thiscall" fn cqk7_destino_no_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqk7_mineiro(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqk7_destino_no_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqk7_mineiro(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1732)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqk7_destino_no_mineiro({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x411980 } (thiscall, ret 0x18, aus_nsl.cpp): `AusNsl::update_staff_records_for_status`: na temporada regular, os
/// seis classificados (6) ganham o registro de classificação para a fase final (fase 0x435); os de fora (-2) devolvem
/// -1; na fase final (fase 0), quem avança (6) ganha o registro (rodada 0x96) e quem cai (-2) o de não classificado
/// (fase 0x435) e -1; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
unsafe extern "thiscall" fn cqk7_destino_na_nsl(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqk7_nsl(this, clube, situacao, fase, true).0
}

/// Comparador: só a original (os registros alocam) com a devolução prevista pelo ensaio.
unsafe extern "thiscall" fn comparar_cqk7_destino_na_nsl(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, _, _) = cqk7_nsl(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(1733)(this, clube, situacao, fase, a4, a5, a6);
    PLACAR.registrar(r == n, || format!("cqk7_destino_na_nsl({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}

