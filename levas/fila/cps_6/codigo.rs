// ---------- %%LEVA%%a leva: o dia do jogo: as faltas ao treino, as competições do dia e a confiança da diretoria no clube (game.cpp) ----------

unsafe fn cps6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps6_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cps6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cps6_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cps6_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
unsafe fn cps6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cps6_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O clube do contrato (registros de 0x245 bytes em [0xae23b4], pelo id em +4), ou 0 sem contrato.
unsafe fn contract_club_record(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cps6_d(0xae23b4).wrapping_add(cps6_d(c + 4).wrapping_mul(0x245))
    }
}

/// Dias desde a data de +0x3e da pessoa até hoje (0x54cdb0 sobre a data do jogo 0xae2c90, a outra por valor).
unsafe fn days_since_staff_date(p: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    f(0xae2c90, cps6_d(p + 0x3e) as u32, cps6_d(p + 0x42) as u32)
}

/// { 0x5d6aa0 } (cdecl, game.cpp): `game_check_players_missing_training()`, todo dia pelo laço do dia do jogo. A partir
/// de uma pessoa sorteada (sorteio(n − n/250 − 1)), n/250 pessoas seguidas: jogador (+0x61, função +0x3d = 11) com
/// contrato e clube, presente (0x418f60), com o registro de treino (0x8b39f0, +0x15) e sem lesão grave (0x62c400 ≠ 2).
/// (1) Com +0x5b < 8, jogador +0x2f < 8 e a soma (jogador +0x2f, +0x18, +0x5b, vínculo +0x44, −5 se +0x5b ou o moral
/// valem 1, −5 se não gosta do técnico, +2 se gosta) < 43: sorteio(+0x5b + 100, 50 ou 25 conforme os dias desde +0x3e:
/// < 30, < 90, < 180) zerado é falta. (2) Senão, com +0x5b < 10 e vínculo com o clube: as marcas do vínculo (+0x45:
/// 0x8000, 0x30000; com +0x5b < 5, 0x7800) dão sorteios(+0x5b) que, zerados, são falta. A falta: 0x41a360.
unsafe extern "C" fn game_check_players_missing_training() {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    let treino: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b39f0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let falta: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x41a360usize);
    let n = cps6_d(0xae2364) as i32;
    let q = n / 250;
    crate::comparison::branch("cps_6 absences of the day");
    let r = cps6_sortear(n - q - 1); // cps6: a primeira pessoa
    let mut atual = cps6_d(0xae23bc).wrapping_add((r as usize).wrapping_mul(0x6e));
    for _ in 0..q {
        let p = atual;
        atual = atual.wrapping_add(0x6e);
        if cps6_d(p + 0x61) == 0 || cps6_b(p + 0x3d) != 0xb {
            continue;
        }
        let c = cps6_contrato(p);
        if c == 0 {
            continue;
        }
        let clube = contract_club_record(c);
        if clube == 0 {
            continue;
        }
        if ausente(cps6_d(0xad9c6c), p, 0) != 0 {
            continue;
        }
        let t = treino(cps6_d(0xde1f44), p);
        if t == 0 || cps6_b(t + 0x15) == 0 {
            continue;
        }
        if lesao(cps6_d(0xb63d28), p) == 2 {
            continue;
        }
        let jog = cps6_d(p + 0x61);
        let cl = cps6_b(p + 0x5b);
        let dl = cps6_b(jog + 0x44);
        let mut bl = cps6_b(jog + 0x2f).wrapping_add(cps6_b(p + 0x18)).wrapping_add(cl).wrapping_add(dl);
        if cl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if dl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if nao_gosta(cps6_d(clube + 0xcf), p) != 0 {
            bl = bl.wrapping_add(0xfb);
        }
        if gosta(cps6_d(clube + 0xcf), p) != 0 {
            bl = bl.wrapping_add(2);
        }
        let mut e_falta = false;
        let a = cps6_b(p + 0x5b);
        if (a as i8) < 8 && (cps6_b(cps6_d(p + 0x61) + 0x2f) as i8) < 8 && (bl as i8) < 0x2b {
            let mut s = a;
            if days_since_staff_date(p) < 0x1e {
                s = s.wrapping_add(0x64);
            } else if days_since_staff_date(p) < 0x5a {
                s = s.wrapping_add(0x32);
            } else if days_since_staff_date(p) < 0xb4 {
                s = s.wrapping_add(0x19);
            }
            crate::comparison::branch("cps_6 absence by sum");
            if cps6_sortear(s as i8 as i32) == 0 { // cps6: o sorteio da soma
                e_falta = true;
            }
        }
        if !e_falta {
            if cps6_sb(p + 0x5b) >= 0xa {
                continue;
            }
            let clube = contract_club_record(cps6_contrato(p));
            let v = contract_bond(cps6_d(0xae1050), p, clube);
            if v == 0 {
                continue;
            }
            if cps6_d(v + 0x45) & 0x8000 != 0 && cps6_sortear(cps6_sb(p + 0x5b)) == 0 {
                e_falta = true;
            } else if cps6_d(v + 0x45) & 0x30000 != 0 && cps6_sortear(cps6_sb(p + 0x5b)) == 0 {
                e_falta = true;
            } else {
                let a = cps6_sb(p + 0x5b);
                if a >= 5 || cps6_d(v + 0x45) & 0x7800 == 0 || cps6_sortear(a) != 0 {
                    continue;
                }
                e_falta = true;
            }
        }
        if e_falta {
            crate::comparison::branch("cps_6 missed training");
            falta(cps6_d(0xad9c6c), p); // cps6: a falta
        }
    }
}

// ponytail: sorteia e a falta (0x41a360) multa, sorteia e mexe na forma: só a original; a cópia leva os ramos.
unsafe extern "C" fn compare_game_check_players_missing_training() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// { 0x5ce060 } (cdecl, game.cpp): `game_update_comps_for_date(marca)`: cada competição da lista ([0xadadfc], conta
/// [0xadadf8] relida a cada volta) recebe o dia (virtual +0x14 com a data do jogo 0xae2c90 e a marca); com a marca, o
/// passo seguinte do [0xae2a58] (0x5aaf00, que a original chama em salto de cauda). Pelo laço do dia do jogo (com 1).
unsafe extern "C" fn game_update_comps_for_date(marca: u32) {
    let m = marca as u8 as i8;
    let mut i = 0i32;
    while i < cps6_d(0xadadf8) as i32 {
        let c = cps6_d(cps6_d(0xadadfc) + i as usize * 4);
        if c != 0 {
            let dia: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cps6_d(cps6_d(c) + 0x14));
            dia(c, 0xae2c90, m as i32);
        }
        i += 1;
    }
    if m != 0 {
        crate::comparison::branch("cps_6 competitions of the day");
        let seguinte: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5aaf00usize);
        seguinte(cps6_d(0xae2a58));
    }
}

// ponytail: passa o dia de todas as competições (virtuais) e do [0xae2a58]: só a original; a cópia leva o ramo.
unsafe extern "C" fn compare_game_update_comps_for_date(marca: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I1%%)(marca)
}

/// A confiança da diretoria no clube: a palavra +6 da linha do clube (73 bytes por id em [this]).
unsafe fn board_confidence_ptr(this: usize, clube: usize) -> usize {
    cps6_d(this).wrapping_add(cps6_d(clube).wrapping_mul(0x49)) + 6
}

/// { 0x5cdde0 } (thiscall, ret 8, game.cpp): `manager_manager_adjust_board_confidence(clube, delta)`: soma `delta`
/// (palavra) à confiança da diretoria no clube (+6 da linha do MANAGER_MANAGER [0xb67a44]), presa em 1..10000. Clube
/// nulo: nada. Pelo laço do dia do jogo (com −5).
unsafe extern "thiscall" fn manager_manager_adjust_board_confidence(this: usize, clube: usize, delta: u32) {
    if clube == 0 {
        return;
    }
    let c = board_confidence_ptr(this, clube);
    cps6_pw(c, cps6_w(c).wrapping_add(delta as u16 as i16));
    let v = cps6_w(c);
    if v < 1 {
        cps6_pw(c, 1);
    } else if v > 0x2710 {
        cps6_pw(c, 0x2710);
    }
}

/// Comparador: lado a lado pela confiança (2 bytes).
unsafe extern "thiscall" fn compare_manager_manager_adjust_board_confidence(this: usize, clube: usize, delta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%);
    if clube == 0 {
        return o(this, clube, delta);
    }
    let c = board_confidence_ptr(this, clube);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(c, 2)], || o(this, clube, delta), || manager_manager_adjust_board_confidence(this, clube, delta));
    SCOREBOARD.record(igual, || format!("manager_manager_adjust_board_confidence(clube {:x}, {}) {:x?} x {:x?}", clube, delta as i32, mo, mr));
}
