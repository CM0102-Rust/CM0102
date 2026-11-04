// ---------- %%LEVA%%a leva: o destino das seleções nos torneios: a Copa Africana de Nações, a Copa América e a eliminatória da Oceania (african_nations.cpp, goldcup.cpp, wc_oceania_league.cpp) ----------

unsafe fn cqj10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqj10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push a6; push jogo; push clube; mov ecx,
/// [0xdd7f00] -> thiscall, ret 0xc).
unsafe fn cqj10_final(f: usize, clube: usize, jogo: usize, a6: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(cpz4_registros(), clube, jogo, a6);
}

/// A virtual +0x44 de um torneio de seleções com grupos e mata-mata (a Copa Africana e a Copa América): `fase` (byte com
/// sinal) abaixo de `grupos` é a fase de grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x78, peso 0x19); 0xfe,
/// a não classificação (0x86c1d0: fase 0x41c, peso 0x19), que encerra as convocações. Na fase `grupos` (o mata-mata,
/// a fase [+0xc][grupos]): 0, a classificação com a fase +0x32 do jogo, o +0x6f da rodada do jogo (+0x34) na tabela de
/// rodadas da fase (+0xa3, 0x68 bytes) e o peso 0x4bf850(0x28, 0x64, rodada, 3); 0xff, a eliminação (0x86c000) com
/// +0x32, +0x30 e o mesmo peso; 1, 2 e 3, os registros de campeão, vice e terceiro. Devolve 0.
unsafe fn cqj10_selecao(nome: u8, this: usize, clube: usize, desfecho: u32, fase: u32, jogo: usize, a6: u32, grupos: i8) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqj10_d(this + 4);
    if f < grupos {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            if nome == 0 {
                crate::comparacao::ramo("cqj_10 africana fora nos grupos");
                cpz4_nao_classificado(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa Africana
            } else {
                crate::comparacao::ramo("cqj_10 copa america fora nos grupos");
                cpz4_nao_classificado(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa América
            }
        }
    } else if f == grupos {
        match d {
            0 => {
                let r = cqj10_w(jogo + 0x34);
                let peso = cpz4_escala(0x28, 0x64, r, 3);
                let mata = cqj10_d(cqj10_d(this + 0xc) + grupos as usize * 4);
                let rodada = cqj10_w(cqj10_d(mata + 0xa3).wrapping_add((r as isize * 0x68) as usize) + 0x6f) as u16 as u32;
                cpz4_classificado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, rodada, peso);
            }
            0xff => {
                let peso = cpz4_escala(0x28, 0x64, cqj10_w(jogo + 0x34), 3);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
                if nome == 0 {
                    crate::comparacao::ramo("cqj_10 africana fora no mata-mata");
                    cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso); // cqj10: quem cai no mata-mata da Copa Africana
                } else {
                    cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso);
                }
            }
            1 => cqj10_final(0x86a800, clube, jogo, a6),
            2 => cqj10_final(0x86b0b0, clube, jogo, a6),
            3 => cqj10_final(0x86b710, clube, jogo, a6),
            _ => {}
        }
    }
    0
}

/// { 0x401ba0 } (thiscall, ret 0x18, african_nations.cpp; a virtual +0x44): `AfricanNations::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 3 fases de grupos (o
/// mata-mata na fase 3); o desfecho 4 não faz nada.
unsafe extern "thiscall" fn cqj10_copa_africana_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(0, this, clube, desfecho, fase, jogo, a6, 3)
}

// ponytail: os registros vão para o histórico dos técnicos e encerram as convocações: não rodam duas vezes, e a rotina
// não grava nada além deles. Só a original; o plano confere a devolução 0.
unsafe extern "thiscall" fn comparar_cqj10_copa_africana_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("cqj10_copa_africana_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// { 0x5e13b0 } (thiscall, ret 0x18, goldcup.cpp; a virtual +0x44 da Copa América): `GoldCup::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 2 fases de grupos (o
/// mata-mata na fase 2).
unsafe extern "thiscall" fn cqj10_copa_america_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(1, this, clube, desfecho, fase, jogo, a6, 2)
}

// ponytail: como a da Copa Africana: só a original, a devolução 0.
unsafe extern "thiscall" fn comparar_cqj10_copa_america_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("cqj10_copa_america_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// A competição atual da seleção acaba (0x775000 em [0xb74318]: push nação; mov ecx -> thiscall, ret 4).
unsafe fn cqj10_fim_da_competicao(clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x775000usize);
    f(cqj10_d(0xb74318), cqj10_d(clube + 0x53));
}

/// { 0x91e840 } (thiscall, ret 0x18, wc_oceania_league.cpp; a virtual +0x44): `WcOceaniaLeague::
/// update_staff_records_for_status(seleção, desfecho, fase, saida, jogo, _)`; devolve 0. Fase (byte com sinal) 0 ou
/// menos, os grupos: 6, a classificação (fase 0, rodada 0x82, peso 0x14); 0xfe, a não classificação (0x41c, 0x14) e a
/// competição atual da seleção acaba. Fase 1 (a final da Oceania): 1, a classificação (0xa0, 0, 0x1e); 2 ou 0xff, a
/// eliminação (+0x32 e +0x30 do jogo, peso 0x1e) e a competição acaba. Fase 2 (a repescagem): 1, a classificação para
/// a Copa do Mundo (a competição [0xae23d0] + 0x6b × [0x9cf79c], 0, 0, 0x64), a vaga nela (0x92e740 no objeto
/// [[0xadadfc] + 4 × [0x9cf79c]]: push seleção -> thiscall, ret 4) e *saida = 4; 2, a eliminação (peso 0x64) e a
/// competição acaba.
unsafe extern "thiscall" fn cqj10_eliminatoria_oceania_registros(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, _a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqj10_d(this + 4);
    let eliminar = |peso: u32| {
        // ponytail: +0x30 e +0x32 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
        cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso);
        cqj10_fim_da_competicao(clube);
    };
    if f < 1 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x41c, 0x14);
            cqj10_fim_da_competicao(clube);
        }
    } else if f == 1 {
        if d == 1 {
            cpz4_classificado(clube, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            crate::comparacao::ramo("cqj_10 oceania na copa do mundo");
            let id = cqj10_d(0x9cf79c);
            cpz4_classificado(clube, cqj10_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            let vaga: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x92e740usize);
            vaga(cqj10_d(cqj10_d(0xadadfc).wrapping_add(id.wrapping_mul(4))), clube);
            *(saida as *mut u8) = 4;
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

// ponytail: os registros, o fim da competição da seleção e a vaga na Copa do Mundo não rodam duas vezes. Só a original;
// o plano: a devolução 0 e, na repescagem ganha, *saida = 4 e a lista da Copa do Mundo (+0x36) um a mais (se cabia).
unsafe extern "thiscall" fn comparar_cqj10_eliminatoria_oceania_registros(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I2%%);
    let vaga = fase as u8 == 2 && desfecho as u8 == 1 && saida != 0;
    let mundial = cqj10_d(cqj10_d(0xadadfc).wrapping_add(cqj10_d(0x9cf79c).wrapping_mul(4)));
    if vaga && mundial == 0 {
        return o(this, clube, desfecho, fase, saida, jogo, a6);
    }
    let n = if vaga { cqj10_w(mundial + 0x36) } else { 0 };
    let cabe = vaga && n < cqj10_w(mundial + 0x38);
    let r = o(this, clube, desfecho, fase, saida, jogo, a6);
    let igual = r == 0 && (!vaga || (*(saida as *const u8) == 4 && cqj10_w(mundial + 0x36) == if cabe { n + 1 } else { n }));
    PLACAR.registrar(igual, || format!("cqj10_eliminatoria_oceania_registros({:#x}, {}, {}) devolveu {} vaga={} lista {}", clube, desfecho as u8, fase as u8, r, vaga, n));
    r
}
