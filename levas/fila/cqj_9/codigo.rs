// ---------- %%LEVA%%a leva: o destino de cada clube no fim das fases da Série A, da Série C e do Baiano (bra_nat_first.cpp, bra_nat_third.cpp, bra_reg_bahia.cpp) ----------

unsafe fn cqj9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqj9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, todos já em Rust):
/// 0x86bdd0 classificou-se (clube, comp, fase, rodada, peso; ret 0x14).
unsafe fn cqj9_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cqj9_d(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86a1c0 rebaixado (clube, liga; ret 8).
unsafe fn cqj9_rebaixado(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
    f(cqj9_d(0xdd7f00), clube, comp);
}

/// 0x868c50 campeão da liga (clube, liga; ret 8).
unsafe fn cqj9_campeao(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
    f(cqj9_d(0xdd7f00), clube, comp);
}

/// 0x869480 promovido (clube, liga, peso; ret 0xc).
unsafe fn cqj9_promovido(clube: usize, comp: usize, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
    f(cqj9_d(0xdd7f00), clube, comp, peso);
}

/// 0x68ab70 `League::find_club_row(clube, 0)` (thiscall, ret 8): a linha do clube na tabela da liga.
unsafe fn cqj9_linha(liga: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x68ab70usize);
    f(liga, clube, 0)
}

/// A Série A (0x42abc0). `fazer` = falso: o ensaio do comparador, sem os registros nem a gravação; devolve
/// (devolução, linha gravada ou 0, destino gravado).
unsafe fn cqj9_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = cqj9_d(this + 4);
    if fa == 0xff {
        // a fase de pontos corridos
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqj_9 serie a classificado");
                cqj9_classificou(clube, comp, 0x41c, 0, 0x28);
            }
        } else if st == 3 && fazer {
            cqj9_rebaixado(clube, comp);
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    // o mata-mata
    if st == 1 {
        let l = cqj9_linha(this, clube);
        if fazer {
            crate::comparacao::ramo("cqj_9 serie a campeao");
            cqj9_campeao(clube, comp); // cqj9: o registro do campeão da Série A
            *((l + 0x37) as *mut u8) = 5;
        }
        return (0, l, 5);
    }
    if st == 2 || st == 0xff {
        let l = cqj9_linha(this, clube);
        if fazer {
            *((l + 0x37) as *mut u8) = 0xfe; // eliminado
        }
        return (0, l, 0xfe);
    }
    (0, 0, 0)
}

/// A Série C (0x42eb00); como a da Série A.
unsafe fn cqj9_serie_c(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    let comp = cqj9_d(this + 4);
    if fa < 7 {
        // os grupos
        if st == 6 && fazer {
            cqj9_classificou(clube, comp, 0x14, 0, 0x28);
        }
        return (0, 0, 0);
    }
    if fa == 7 {
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_classificou(clube, comp, 0, 0x1e, 0x28);
        }
        return (0, 0, 0);
    }
    if fa < 10 {
        // o quadrangular final: os dois que sobem
        if st != 0 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_promovido(clube, comp, 0x32);
            cqj9_classificou(clube, comp, 0, 0x96, 0x28);
        }
        // a linha do clube na liga (k = -1) ou nos estágios 0..6, a primeira que tiver o clube, ganha o destino 0
        let mut k: i8 = -1;
        let mut achada = 0;
        while k < 7 {
            let liga = if k == -1 { this } else { cqj9_d(cqj9_d(this + 0xc) + 4 * k as i32 as usize) };
            let n = cqj9_w(liga + 0x3e);
            if n > 0 {
                let t = cqj9_d(liga + 0xb1);
                let mut i: i8 = 0;
                while (i as i16) < n {
                    let r = t.wrapping_add(0x3b * i as i32 as usize);
                    if cqj9_d(r) == clube {
                        achada = r;
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if achada != 0 {
                    break;
                }
            }
            k += 1;
        }
        if achada != 0 && fazer {
            crate::comparacao::ramo("cqj_9 serie c promovido");
            *((achada + 0x37) as *mut u8) = 0; // cqj9: o destino de quem sobe da Série C
        }
        return (0, achada, 0);
    }
    if fa == 10 {
        // a final
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_campeao(clube, comp);
        }
    }
    (0, 0, 0)
}

/// O Baiano (0x430210); como a da Série A. O clube vem da tabela (+0xb1, +0x3e linhas), não da 0x68ab70.
unsafe fn cqj9_baiano(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = cqj9_d(this + 4);
    if fa == 0xff {
        if st == 6 {
            if fazer {
                cqj9_classificou(clube, comp, 0, 0x82, 0x32);
            }
        } else if st == 3 && fazer {
            crate::comparacao::ramo("cqj_9 baiano rebaixado");
            cqj9_rebaixado(clube, comp); // cqj9: o rebaixado do Baiano
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let n = cqj9_w(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if cqj9_d(cqj9_d(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    cqj9_campeao(clube, comp);
                }
                let l = cqj9_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (0, l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // ponytail: a original empurra os registradores inteiros (eax com o lixo do chamador acima da
                    // palavra, ecx com a metade de cima do ponteiro da tabela); a 0x4bf850 e a 0x86c000 só leem as
                    // palavras (0x4b0870 / `as u16`): vão as palavras.
                    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
                    let peso = escala(0x1e, 0x32, cqj9_w(jogo + 0x34) as u16 as u32, 2);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                    elim(cqj9_d(0xdd7f00), clube, comp, cqj9_w(jogo + 0x32) as u16 as u32, cqj9_w(jogo + 0x30) as u16 as u32, peso);
                }
                let l = cqj9_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (0, l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0, 0)
}

/// { 0x42abc0 } (thiscall, ret 0x18, bra_nat_first.cpp): `BraNatFirst::update_staff_records_for_status` (slot 18):
/// na fase de pontos corridos, os 8 classificados (situação 6) ganham o registro de classificação e os rebaixados (3) o
/// de rebaixamento; no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5, os eliminados (2 ou
/// -1) o destino -2. Devolve 0.
unsafe extern "thiscall" fn bra_nat_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqj9_serie_a(this, clube, situacao, fase, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação): a devolução e o destino gravado na linha.
unsafe extern "thiscall" fn comparar_bra_nat_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_serie_a(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%)(this, clube, situacao, fase, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_nat_first_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x42eb00 } (thiscall, ret 0x18, bra_nat_third.cpp): `BraNatThird::update_staff_records_for_status`: nos grupos
/// (fase < 7), os classificados (6) ganham o registro de classificação; na fase 7, quem passa (1) também (os outros: -1);
/// no quadrangular (fases 8 e 9), os que sobem (0) ganham os registros de acesso e de classificação e o destino 0 na
/// primeira tabela que os tiver (a liga ou os estágios 0..6); na final (10), o campeão (1) ganha o registro de campeão.
/// Devolve 0, ou -1 nas situações que a fase não espera.
unsafe extern "thiscall" fn bra_nat_third_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqj9_serie_c(this, clube, situacao, fase, true).0
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_bra_nat_third_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_serie_c(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I1%%)(this, clube, situacao, fase, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_nat_third_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x430210 } (thiscall, ret 0x18, bra_reg_bahia.cpp): `BraRegBahia::update_staff_records_for_status` (o slot 18 de
/// sete estaduais: Baiano, Gaúcho, Norte, Nordeste, Pernambucano, Catarinense e Paulista): na fase de pontos corridos, os
/// classificados (6) ganham o registro de classificação (rodada 0x82, peso 0x32) e os rebaixados (3) o de rebaixamento;
/// no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5; os eliminados (2 ou -1), o registro
/// de eliminação com o peso da rodada (0x4bf850: de 30 a 50 pela rodada do jogo) e o destino -2. Devolve 0.
unsafe extern "thiscall" fn bra_reg_bahia_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqj9_baiano(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_bra_reg_bahia_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_baiano(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_reg_bahia_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}
