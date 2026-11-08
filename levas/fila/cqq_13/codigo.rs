// ---------- %%LEVA%%a leva: os registros dos clubes nas copas da Itália e no Troféu da Liga inglesa: a Copa da Itália, a Copa da Série C e o LDV Vans Trophy (ita_cup.cpp, ita_c_cup.cpp, eng_auto_cup.cpp) ----------

unsafe fn cqq13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqq13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão (0x86a800) ou o vice (0x86b0b0) no registro de staff ([0xdd7f00]; thiscall(registros, clube, jogo, a6),
/// ret 0xc).
unsafe fn cqq13_final(g: usize, clube: usize, jogo: usize, a6: u32) {
    let h: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(g);
    h(cpz4_registros(), clube, jogo, a6);
}

/// A palavra +0x6f (o nome) da rodada +0x34 do jogo na tabela de rodadas `rodadas` (0x68 bytes por rodada).
unsafe fn cqq13_nome_da_rodada(rodadas: usize, jogo: usize) -> u32 {
    let r = cqq13_w(jogo + 0x34) as isize;
    core::ptr::read_unaligned((rodadas.wrapping_add((r * 0x68) as usize) + 0x6f) as *const u16) as u32
}

/// O corpo das duas copas italianas (`update_staff_records_for_status(clube, desfecho, fase, _, jogo, a6)`, a virtual
/// +0x44; devolve 0). Fase (byte com sinal) abaixo de `limite`, os grupos: 6, a classificação (0x86bdd0: fase 0, rodada
/// `rodada_g`, peso `peso_g`); 0xfe, a não classificação (0x86c1d0: 0x41c, 0x28). Fase `limite`, o mata-mata, peso =
/// 0x4bf850(0x32, 0x64, rodada do jogo +0x34, 5): 0xff, a eliminação (0x86c000: +0x32, +0x30 do jogo, peso); 0, a
/// classificação com +0x32 e o nome da rodada do jogo na tabela de rodadas da fase do mata-mata ([[+0xc] + 4 × limite]
/// + 0xa3; os 16 bits altos que a original empurra são os do ponteiro da fase); 1, o campeão (0x86a800); 2, o vice
/// (0x86b0b0).
unsafe fn cqq13_italia(this: usize, clube: usize, desfecho: u32, fase: u32, jogo: usize, a6: u32, limit: i8, rodada_g: u32, peso_g: u32, nome: &'static str) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqq13_d(this + 4);
    if f < limit {
        if d == 6 {
            crate::comparison::branch(nome);
            european_cup_add_qualified_record(clube, comp, 0, rodada_g, peso_g);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x28); // cqq13: não passou dos grupos
        }
    } else if f == limit {
        match d {
            0xff => {
                let peso = scale_by_round(0x32, 0x64, cqq13_w(jogo + 0x34), 5);
                // ponytail: +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
                european_cup_add_knocked_out_record(clube, comp, cqq13_w(jogo + 0x32) as u16 as u32, cqq13_w(jogo + 0x30) as u16 as u32, peso);
            }
            0 => {
                let peso = scale_by_round(0x32, 0x64, cqq13_w(jogo + 0x34), 5);
                let mata = cqq13_d(cqq13_d(this + 0xc) + limit as usize * 4);
                let rodada = (mata as u32 & 0xffff_0000) | cqq13_nome_da_rodada(cqq13_d(mata + 0xa3), jogo);
                european_cup_add_qualified_record(clube, comp, cqq13_w(jogo + 0x32) as u16 as u32, rodada, peso);
            }
            1 => cqq13_final(0x86a800, clube, jogo, a6),
            2 => cqq13_final(0x86b0b0, clube, jogo, a6),
            _ => {}
        }
    }
    0
}

/// { 0x63ff00 } (thiscall, ret 0x18, ita_cup.cpp): `ItaCup::update_staff_records_for_status`: grupos abaixo da fase 7
/// (classificação na rodada 0xa, peso 0x28), o mata-mata na fase 7.
unsafe extern "thiscall" fn cqq13_copa_da_italia_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqq13_italia(this, clube, desfecho, fase, jogo, a6, 7, 0xa, 0x28, "cqq_13 classificado nos grupos da copa da italia") // cqq13: a Copa da Itália
}

// ponytail: os registros (histórico, reputação, moral) não rodam duas vezes. Só a original; o plano: a devolução 0.
unsafe extern "thiscall" fn compare_cqq13_copa_da_italia_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, clube, desfecho, fase, a4, jogo, a6);
    SCOREBOARD.record(r == 0, || format!("cqq13_copa_da_italia_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// { 0x63e270 } (thiscall, ret 0x18, ita_c_cup.cpp): `ItaCCup::update_staff_records_for_status`: grupos abaixo da fase
/// 0xf (classificação na rodada 0x14, peso 0x28), o mata-mata na fase 0xf.
unsafe extern "thiscall" fn cqq13_copa_c_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqq13_italia(this, clube, desfecho, fase, jogo, a6, 0xf, 0x14, 0x28, "cqq_13 classificado nos grupos da copa da serie c")
}

// ponytail: os registros não rodam duas vezes. Só a original; o plano: a devolução 0.
unsafe extern "thiscall" fn compare_cqq13_copa_c_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, clube, desfecho, fase, a4, jogo, a6);
    SCOREBOARD.record(r == 0, || format!("cqq13_copa_c_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// { 0x56bee0 } (thiscall, ret 0x18, eng_auto_cup.cpp): `EngAutoCup::update_staff_records_for_status(clube, desfecho,
/// fase, _, jogo, a6)`; devolve 0. Nas chaves (fase ≤ 0), peso = 0x4bf850(1, 0x50, rodada do jogo, 5): −1 ou 2, a
/// eliminação (0x86c000: +0x32, +0x30 do jogo, peso); 0, a classificação com +0x32 e o nome da rodada do jogo na tabela
/// +0xa3 da própria copa; 1, a classificação para a final (fase 0, rodada 0x96, peso 0x5a). Na final (fase 1): 1 o
/// campeão (0x86a800), 2 o vice (0x86b0b0).
unsafe extern "thiscall" fn cqq13_trofeu_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqq13_d(this + 4);
    if f <= 0 {
        match d {
            0xff | 2 => {
                crate::comparison::branch("cqq_13 eliminado no trofeu");
                let peso = scale_by_round(1, 0x50, cqq13_w(jogo + 0x34), 5); // cqq13: o peso da eliminação no LDV
                european_cup_add_knocked_out_record(clube, comp, cqq13_w(jogo + 0x32) as u16 as u32, cqq13_w(jogo + 0x30) as u16 as u32, peso);
            }
            0 => {
                let peso = scale_by_round(1, 0x50, cqq13_w(jogo + 0x34), 5);
                // os 16 bits altos que a original empurra com a palavra do nome e com +0x32 vêm de contas pequenas
                // com a rodada (3r e 13r): zero
                european_cup_add_qualified_record(clube, comp, cqq13_w(jogo + 0x32) as u16 as u32, cqq13_nome_da_rodada(cqq13_d(this + 0xa3), jogo), peso);
            }
            1 => european_cup_add_qualified_record(clube, comp, 0, 0x96, 0x5a),
            _ => {}
        }
    } else if f == 1 {
        match d {
            1 => cqq13_final(0x86a800, clube, jogo, a6),
            2 => cqq13_final(0x86b0b0, clube, jogo, a6),
            _ => {}
        }
    }
    0
}

// ponytail: os registros não rodam duas vezes. Só a original; o plano: a devolução 0.
unsafe extern "thiscall" fn compare_cqq13_trofeu_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, clube, desfecho, fase, a4, jogo, a6);
    SCOREBOARD.record(r == 0, || format!("cqq13_trofeu_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}
