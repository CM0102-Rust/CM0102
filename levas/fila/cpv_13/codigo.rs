// ---------- %%LEVA%%a leva: o prêmio do mês no histórico (o técnico conta, o jogador agradece ou renova), a faixa de reputação e a folga do amistoso (staff_records.cpp, friendly.cpp) ----------

unsafe fn cpv13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0 sem contrato ou
/// com o contrato do tipo 5 (+0x35 & 0x3f).
unsafe fn contract_club_except_type5(p: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv13_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 || cpv13_b(c + 0x35) & 0x3f == 5 {
        0
    } else {
        cpv13_d(0xae23b4).wrapping_add(cpv13_d(c + 4).wrapping_mul(0x245))
    }
}

/// O corpo da 0x86c8c0; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
unsafe fn add_month_award_record_body(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, gravar: bool) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x33;
    put_i32(&mut e, 8, if clube != 0 { cpv13_d(clube) as i32 } else { -1 });
    put_i32(&mut e, 0xd, cpv13_d(premio) as i32);
    put_i32(&mut e, 0x11, (a4 & 0xff) as i32);
    put_i32(&mut e, 0x15, (a5 & 0xffff) as i32);
    append_person_history_by_id(gravar, this, cpv13_d(p), &e);
    let (t3d, t28) = (cpv13_b(p + 0x3d), cpv13_b(p + 0x28));
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = staff_records_manager_record(this, p);
        if r != 0 {
            crate::comparison::branch("cpv_13 prêmio do técnico");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1)); // cpv13: conta o prêmio
        }
        return;
    }
    if cpv13_d(p + 0x61) == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv13_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 || cpv13_b(c + 0x35) & 0x3f == 5 {
        return;
    }
    if cpv13_d(0xae23b4).wrapping_add(cpv13_d(c + 4).wrapping_mul(0x245)) == 0 {
        return;
    }
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = do_clube(cpv13_d(0xae1050), p, contract_club_except_type5(p));
    if k == 0 {
        return;
    }
    let gt = cpv13_d(0xde1f54);
    if gt != 0 {
        let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, cpv13_d(k + 0x25), cpv13_d(k + 0x29)) > 0x12c {
            let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
            let x = entrada(cpv13_d(cpv13_d(0xde1f54) + 0x856), contract_club_except_type5(p), p, 0, 0);
            if x != 0 && cpv13_d(x + 0xf) == cpv13_d(p) {
                crate::comparison::branch("cpv_13 renovação");
                offer_renewal(p);
            }
        }
    }
    if cpv13_d(k + 0x45) != 0 {
        return;
    }
    let v59 = cpv13_b(p + 0x59) as i8;
    if v59 <= 0xf { // cpv13: o +0x59
        return;
    }
    if cpv13_b(p + 0x5b) as i8 <= 0xa { // cpv13: o +0x5b
        return;
    }
    crate::comparison::branch("cpv_13 sorteio do agradecimento");
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    if sorteio(v59 as i32) <= 5 {
        return;
    }
    let agradece: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x58a070usize);
    agradece(p, clube, premio);
}

/// { 0x86c8c0 } (thiscall, ret 0x14, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_month_award_record(
/// p, clube, premio, a4, a5)`: a entrada 0x33 do histórico da pessoa (data do jogo, clube ou −1, o id do prêmio, a4 no
/// byte, a5 na palavra; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na palavra +0x9e do registro dele
/// (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o clube (0x4cf5f0): com o gerente de
/// transferências, contrato de mais de 300 dias e o jogador na lista do clube (0x849ce0), a renovação (0x4cda70); depois,
/// sem a marca +0x45 do contrato, com +0x59 acima de 15 e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do
/// agradecimento ao técnico (0x58a070). Sem pessoa ou sem prêmio: a original (caixa "Error").
unsafe extern "thiscall" fn add_month_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32) {
    if p == 0 || premio == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(%%I0%%)(this, p, clube, premio, a4, a5);
    }
    add_month_award_record_body(this, p, clube, premio, a4, a5, true)
}

/// { 0x5c52d0 } (stdcall, ret 4, friendly.cpp): `friendly_club_reputation_tier(clube)`: a faixa da reputação (+0x80):
/// 5 a partir de 8000, 4 de 6000, 3 de 4000, 2 de 2000, senão 1 (só o al vale).
unsafe extern "stdcall" fn friendly_club_reputation_tier(clube: usize) -> u8 {
    let r = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    if r >= 0x1f40 {
        5
    } else if r >= 0x1770 {
        4
    } else if r >= 0xfa0 {
        3
    } else {
        1 + (r >= 0x7d0) as u8
    }
}

/// { 0x5c74d0 } (stdcall, ret 4, friendly.cpp): `friendly_get_rest_days_for_kind(tipo)`: a folga em volta do amistoso:
/// 2 nos tipos 1 e 10, 3 no 2, 1 nos 0, 3, 4 e 11. Os tipos 5..9 e fora de 0..11: a original (caixa "Error" e 1).
unsafe extern "stdcall" fn friendly_get_rest_days_for_kind(tipo: u32) -> u8 {
    match tipo as u8 as i8 {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => original_fn::<unsafe extern "stdcall" fn(u32) -> u8>(%%I2%%)(tipo),
    }
}

/// Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
unsafe extern "thiscall" fn comparar_add_month_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(%%I0%%);
    if p == 0 || premio == 0 {
        return o(this, p, clube, premio, a4, a5);
    }
    let (t3d, t28) = (cpv13_b(p + 0x3d), cpv13_b(p + 0x28));
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparison::branch("add_month_award_record jogador");
        return o(this, p, clube, premio, a4, a5);
    }
    let r = staff_records_manager_record(this, p);
    if r == 0 {
        return o(this, p, clube, premio, a4, a5);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 0xd2)], || o(this, p, clube, premio, a4, a5),
        || add_month_award_record_body(this, p, clube, premio, a4, a5, false));
    PLACAR.record(igual, || format!("add_month_award_record({:x}, prêmio {:x}) {:x?} x {:x?}", p, cpv13_d(premio), mo, mr));
}

/// Comparador: pura.
unsafe extern "stdcall" fn comparar_friendly_club_reputation_tier(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u8>(%%I1%%)(clube);
    let n = friendly_club_reputation_tier(clube);
    PLACAR.record(r == n, || format!("friendly_club_reputation_tier({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparador: pura; os tipos da caixa, só a original.
unsafe extern "stdcall" fn comparar_friendly_get_rest_days_for_kind(tipo: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> u8>(%%I2%%)(tipo);
    if !matches!(tipo as u8 as i8, 0..=4 | 10 | 11) {
        return r;
    }
    let n = friendly_get_rest_days_for_kind(tipo);
    PLACAR.record(r == n, || format!("friendly_get_rest_days_for_kind({}) original={} rust={}", tipo as u8 as i8, r, n));
    r
}
