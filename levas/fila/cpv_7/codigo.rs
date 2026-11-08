// ---------- %%LEVA%%a leva: os salários do pessoal e os bônus: o teto salarial do pessoal, o pagamento dos bônus e o ajuste do teto salarial (finance.cpp) ----------

unsafe fn cpv7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpv7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x597db0 } (thiscall, ret 4, finance.cpp): `Finance::get_max_staff_wage(cargo)`, o maior salário semanal que o
/// clube oferece a um membro do pessoal (renovação 0x4cda70, interesse pelo cargo 0x844780, telas de contrato). Clube
/// pelo id (+8); tipo 3 (+0x64): 0. O padrão pelo cargo (5 a 7: 35000; 8: 20000; 9: 1500; 10: 1000; outros: 750);
/// seleção (0x53b900): o padrão. Sem nação (+0x53): padrão × 0,25. Com nação: o nível (+0x85) / (20 × tipo +0x7e
/// entre 1 e 3) × padrão; no tipo 1, Inglaterra, Itália e Espanha × 1,05 com reputação acima de 7250, Japão nada, EUA
/// acima de 1750 com 0,75 do excedente, e os demais grupos com o excedente acima de 2500/1250/750 pelo fator (0,75/0,75/
/// 0,3, ou reputação/50 / nível × 0,1 × ele, até 1); com liga (+0x57), × peso da liga (+0x69) × 0,2. Pela faixa das
/// finanças (0x5986b0): −2 × 0,75, −1 × 0,95, 1 × 1,5, 2 × 2. Limitado ao teto do clube (+0x14) e à faixa do cargo.
unsafe extern "thiscall" fn get_max_staff_wage(this: usize, cargo: i8) -> i32 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let id = cpv7_d(this + 8);
    let clube = cpv7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245));
    if cpv7_b(clube + 0x64) == 3 {
        return 0;
    }
    let r50 = cpv7_w(clube + 0x80) as i32 / 50;
    let idx = (cargo as i32).wrapping_sub(5);
    let padrao: i32 = match idx {
        0..=2 => 35000,
        3 => 20000,
        4 => 1500,
        5 => 1000,
        _ => 750,
    };
    if selecao(clube) != 0 {
        return padrao;
    }
    let nac = cpv7_d(clube + 0x53);
    let mut w;
    if nac == 0 {
        w = match_pl_ftol(padrao as f64 * read_f64_at(0x9689e0));
    } else {
        let tipo = (cpv7_b(nac + 0x7e) as i8).clamp(1, 3);
        let k = cpv7_b(nac + 0x85) as i8 as i32;
        w = match_pl_ftol(k as f64 / (tipo as i32 * 20) as f64 * padrao as f64);
        if tipo == 1 {
            let n = cpv7_i(nac);
            let e = |a: usize| n == cpv7_i(a);
            if e(0x9cf2e4) || e(0x9cf360) || e(0x9cf488) {
                if cpv7_w(clube + 0x80) > 0x1c52 {
                    crate::comparison::branch("cpv_7 clube grande");
                    w = match_pl_ftol(w as f64 * read_f64_at(0x9689b8)); // cpv7: o clube grande da Inglaterra, Itália e Espanha
                }
            } else if e(0x9cf36c) {
            } else if e(0x9cf4f8) {
                if w > 0x6d6 {
                    w = match_pl_ftol(w.wrapping_sub(0x6d6) as f64 * read_f64_at(0x969040) + read_f64_at(0x96a5f8));
                }
            } else {
                let (base, limiar, soma) = if e(0x9cf314) || e(0x9cf300) || e(0x9cf45c) || e(0x9cf338) {
                    (0x969040, 0x9c4, 0x9690b0)
                } else if e(0x9cf4a8) || e(0x9cf2c0) || e(0x9cf244) || e(0x9cf434) {
                    (0x969040, 0x4e2, 0x96a5e8)
                } else {
                    (0x968e80, 0x2ee, 0x96a5f0)
                };
                let mut f = read_f64_at(base);
                if r50 > k.wrapping_mul(10) {
                    let d = if k >= 1 { k } else { 1 };
                    f = r50 as f64 / d as f64 * read_f64_at(0x967880) * read_f64_at(base);
                }
                if w > limiar {
                    if f > read_f64_at(0x967890) {
                        f = read_f64_at(0x967890);
                    }
                    w = match_pl_ftol(w.wrapping_sub(limiar) as f64 * f + read_f64_at(soma));
                }
            }
        }
        let liga = cpv7_d(clube + 0x57);
        if liga != 0 {
            crate::comparison::branch("cpv_7 peso da liga");
            w = match_pl_ftol(cpv7_w(liga + 0x69) as f64 * w as f64 * read_f64_at(0x968918)); // cpv7: o peso da liga
        }
    }
    match band(this) as i8 {
        -2 => w = match_pl_ftol(w as f64 * read_f64_at(0x969040)),
        -1 => w = match_pl_ftol(w as f64 * read_f64_at(0x9689d0)),
        1 => w = match_pl_ftol(w as f64 * read_f64_at(0x968940)),
        2 => w = w.wrapping_add(w),
        _ => {}
    }
    let teto = cpv7_i(this + 0x14);
    if teto < w {
        w = teto;
    }
    let (min, max) = match idx {
        0 => (0x6d6, 0x88b8),
        1 | 2 => (0x1f4, 0x4e20),
        3 => (0xfa, 0x1d4c),
        4 => (0x7d, 0x7d0),
        5 => (0x7d, 0x5dc),
        _ => (0x7d, 0x3e8),
    };
    if w > max {
        max
    } else if w < min {
        min
    } else {
        w.min(0x3d090)
    }
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn compare_get_max_staff_wage(this: usize, cargo: i8) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(%%I0%%)(this, cargo);
    let n = get_max_staff_wage(this, cargo);
    SCOREBOARD.record(r == n, || format!("get_max_staff_wage(fin {:#x}, cargo {}) original={} rust={}", this, cargo, r, n));
    r
}

/// { 0x599960 } (thiscall, ret 4, finance.cpp): `Finance::pay_bonuses(valor)`, pelo fim do jogo dos contratos
/// (CONTRACT_MANAGER::update_after_match 0x4ca6d0, três chamadas): soma o valor nas despesas de bônus (+0x138 e +0x98)
/// e o tira do saldo (i64 em +0, estendido com sinal).
unsafe extern "thiscall" fn pay_bonuses(this: usize, valor: i32) {
    for o in [0x138usize, 0x98] {
        core::ptr::write_unaligned((this + o) as *mut i32, cpv7_i(this + o).wrapping_add(valor));
    }
    crate::comparison::branch("cpv_7 bonus pagos");
    let s = core::ptr::read_unaligned(this as *const i64);
    core::ptr::write_unaligned(this as *mut i64, s.wrapping_sub(valor as i64)); // cpv7: o bônus sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn compare_pay_bonuses(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I1%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, valor), || pay_bonuses(this, valor));
    SCOREBOARD.record(igual, || format!("pay_bonuses(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// { 0x598340 } (thiscall, ret 4, finance.cpp): `Finance::adjust_max_wage(percentual)`, pela resposta da diretoria
/// (MANAGER_MANAGER::handle_board_request 0x6a91b0) e pela 0x6acee8: o teto salarial (+0x14) vira (percentual + 100) ×
/// 0,01 × teto, truncado.
unsafe extern "thiscall" fn adjust_max_wage(this: usize, write_percent_pair: i8) {
    let v = match_pl_ftol((write_percent_pair as i32 + 100) as f64 * read_f64_at(0x967898) * cpv7_i(this + 0x14) as f64);
    core::ptr::write_unaligned((this + 0x14) as *mut i32, v);
}

/// Comparador: lado a lado no teto salarial (+0x14).
unsafe extern "thiscall" fn compare_adjust_max_wage(this: usize, write_percent_pair: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(%%I2%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this + 0x14, 4)], || o(this, write_percent_pair), || adjust_max_wage(this, write_percent_pair));
    SCOREBOARD.record(igual, || format!("adjust_max_wage(fin {:#x}, {}) original={:x?} rust={:x?}", this, write_percent_pair, mo, mr));
}
