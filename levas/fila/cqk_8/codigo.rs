// ---------- %%LEVA%%a leva: o progresso do clube nas estatísticas da competição no Mineiro (Paranaense, Carioca), na MLS e na Série C (bra_reg_minas_gerais.cpp, usa_mls.cpp, bra_nat_third.cpp) ----------

/// 0x4a4850 `Comp_Stats::find_club_entry(clube)` (thiscall, ret 4): a entrada do clube nas estatísticas da competição
/// ([this+8]); ordena a tabela pelo clube quando preciso (marca +0x18) e busca. Original (não portada).
unsafe fn bra_reg_minas_gerais_comp_stats_club_entry(this: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    f(core::ptr::read_unaligned((this + 8) as *const u32) as usize, clube)
}

unsafe fn write_progress_codes(e: usize, atual: u8, minima: u8, maxima: u8) {
    *((e + 0x73) as *mut u8) = atual;
    *((e + 0x74) as *mut u8) = minima;
    *((e + 0x75) as *mut u8) = maxima;
}

/// { 0x4350d0 } (thiscall, ret 0x14, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::set_club_progress(clube, fase, a,
/// b, c)` (slot 27 do Mineiro, do Paranaense e do Carioca): grava os três códigos de progresso do clube na entrada dele
/// (+0x73, +0x74, +0x75); na fase de pontos corridos (-1), o segundo abaixo de 3 vira 1 e o terceiro abaixo de 3 vira 2.
/// Devolve a entrada.
unsafe extern "thiscall" fn bra_reg_minas_gerais_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (a, mut b, c) = (a as u8, b as u8, c as u8);
    if fase as u8 == 0xff {
        if b < 3 {
            b = 1;
        }
        if c < 3 {
            crate::comparison::branch("cqk_8 mineiro third below 3");
            write_progress_codes(e, a, b, 2); // cqk8: o terceiro código do Mineiro
            return e;
        }
    }
    crate::comparison::branch("cqk_8 mineiro as it comes");
    write_progress_codes(e, a, b, c); // cqk8: os códigos do Mineiro como vêm
    e
}

/// Comparador: lado a lado pela entrada do clube (+0x73..+0x75), achada antes (a 0x4a4850 ordena na primeira vez e só
/// busca depois; a original e a cópia a chamam de novo); a devolução.
unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(%%I0%%)(this, clube, fase, a, b, c), || bra_reg_minas_gerais_set_club_progress(this, clube, fase, a, b, c));
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_minas_gerais_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// O código da MLS na temporada regular: 1..=3 viram `ate_3`, 4 vira 10, os outros ficam.
fn mls_progress_code(x: u8, ate_3: u8) -> u8 {
    match x {
        1..=3 => ate_3,
        4 => 10,
        _ => x,
    }
}

/// { 0x909dd0 } (thiscall, ret 0x14, usa_mls.cpp): `UsaMls::set_club_progress(clube, fase, a, b, c)`: nas fases 0 e 1
/// e na temporada (-1), a: 1 → 1, 2 → 3, 3 → 7, 4 → 10; b: 1..3 → 1, 4 → 10; c: 1..3 → 7, 4 → 10; na fase 2, cada
/// código vira 2 × código − 1; depois, os códigos como vêm. Devolve a entrada.
unsafe extern "thiscall" fn usa_mls_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 2 {
        let (b, c) = (mls_progress_code(b, 1), mls_progress_code(c, 7));
        let a = match a {
            1 => 1,
            2 => {
                crate::comparison::branch("cqk_8 mls second");
                3 // cqk8: o código 2 da MLS
            }
            3 => 7,
            4 => 10,
            _ => a,
        };
        write_progress_codes(e, a, b, c);
    } else if fa == 2 {
        write_progress_codes(e, a.wrapping_mul(2).wrapping_sub(1), b.wrapping_mul(2).wrapping_sub(1), c.wrapping_mul(2).wrapping_sub(1));
    } else {
        write_progress_codes(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
unsafe extern "thiscall" fn compare_usa_mls_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(%%I1%%)(this, clube, fase, a, b, c), || usa_mls_set_club_progress(this, clube, fase, a, b, c));
    SCOREBOARD.record(igual && r == n, || format!("usa_mls_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// O código da Série C nos grupos (fases até 6): 1 e 2 viram `um_dois`, 3 → 0x11, 4 → 0x19, 5 → 0x21, 6 → 0x29,
/// 7 → 0x31 (a posição no grupo levada para a escala de 49 lugares [inferência]); os outros ficam.
fn serie_c_progress_code(x: u8, um_dois: u8) -> u8 {
    match x {
        1 | 2 => um_dois,
        3 => 0x11,
        4 => 0x19,
        5 => 0x21,
        6 => 0x29,
        7 => 0x31,
        _ => x,
    }
}

/// { 0x42e100 } (thiscall, ret 0x14, bra_nat_third.cpp): `BraNatThird::set_club_progress(clube, fase, a, b, c)`: nos
/// grupos (fase < 7), a e b: 1, 2 → 1, 3 → 0x11 ... 7 → 0x31; c: 1, 2 → 9, depois igual; nas fases 7 em diante, os
/// códigos como vêm (o trecho das fases 1 a 3 que a original tem depois do teste `fase >= 7` nunca roda). Devolve a
/// entrada.
unsafe extern "thiscall" fn bra_nat_third_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 7 {
        let (a, c) = (serie_c_progress_code(a, 1), serie_c_progress_code(c, 9));
        let b2 = serie_c_progress_code(b, 1);
        if b == 3 {
            crate::comparison::branch("cqk_8 serie c second 3");
        }
        write_progress_codes(e, a, b2, c); // cqk8: os códigos dos grupos da Série C
    } else {
        write_progress_codes(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
unsafe extern "thiscall" fn compare_bra_nat_third_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(%%I2%%)(this, clube, fase, a, b, c), || bra_nat_third_set_club_progress(this, clube, fase, a, b, c));
    SCOREBOARD.record(igual && r == n, || format!("bra_nat_third_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}
