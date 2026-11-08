// ---------- %%LEVA%%a leva: o prêmio do ano no histórico (o técnico conta, o jogador premiado agradece ou renova) e os recordes de vitória e de derrota (staff_records.cpp, record_utils.cpp) ----------

unsafe fn cpy10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O corpo da 0x86c390; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
unsafe fn add_year_award(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32, gravar: bool) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x34;
    put_i32(&mut e, 8, if clube != 0 { cpy10_d(clube) as i32 } else { -1 });
    put_i32(&mut e, 0xd, cpy10_d(premio) as i32);
    put_i32(&mut e, 0x11, (a4 & 0xffff) as i32);
    put_i32(&mut e, 0x15, a5 as u8 as i8 as i32);
    put_i32(&mut e, 0x19, credito as u8 as i8 as i32);
    append_person_history_by_id(gravar, this, cpy10_d(p), &e);
    let (t3d, t28) = (cpy10_b(p + 0x3d), cpy10_b(p + 0x28));
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = staff_records_manager_record(this, p);
        if r != 0 {
            crate::comparison::branch("cpy_10 prêmio do técnico");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
        }
        return;
    }
    if cpy10_d(p + 0x61) == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpy10_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 || cpy10_b(c + 0x35) & 0x3f == 5 {
        return;
    }
    if cpy10_d(0xae23b4).wrapping_add(cpy10_d(c + 4).wrapping_mul(0x245)) == 0 {
        return;
    }
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = do_clube(cpy10_d(0xae1050), p, contract_club_except_type5(p));
    if k == 0 {
        return;
    }
    if cpy10_d(0xde1f54) != 0 {
        let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, cpy10_d(k + 0x25), cpy10_d(k + 0x29)) > 0x12c { // cpy10: mais de 300 dias
            let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
            let dono = contract_club_except_type5(p);
            let x = entrada(cpy10_d(cpy10_d(0xde1f54) + 0x856), dono, p, 0, 0);
            if x != 0 && cpy10_d(x + 0xf) == cpy10_d(p) {
                crate::comparison::branch("cpy_10 renovação");
                offer_renewal(p);
            }
        }
    }
    if credito as u8 != 1 {
        crate::comparison::branch("cpy_10 sem crédito");
        return; // cpy10: só o creditado
    }
    if cpy10_d(k + 0x45) != 0 {
        return;
    }
    let v59 = cpy10_b(p + 0x59) as i8;
    if v59 <= 0xf { // cpy10: o +0x59
        return;
    }
    if cpy10_b(p + 0x5b) as i8 <= 0xa {
        return;
    }
    crate::comparison::branch("cpy_10 sorteio do agradecimento");
    if sortear(v59 as i32) <= 5 {
        return;
    }
    let agradece: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x58a070usize);
    agradece(p, clube, premio);
}

/// { 0x86c390 } (thiscall, ret 0x18, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_year_award_record(
/// p, clube, premio, a4, a5, credito)`, pelo registro dos vencedores do prêmio do ano (YEAR_AWARD::record_winners
/// 0x9302f0: o primeiro, o segundo e o terceiro). A entrada 0x34 do histórico da pessoa (data do jogo, clube ou −1, o id
/// do prêmio, a4 na palavra, a5 e o crédito em bytes com sinal; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na
/// palavra +0x9e do registro dele (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o
/// clube (0x4cf5f0): com o gerente de transferências, contrato de mais de 300 dias e o jogador na lista do clube
/// (0x849ce0), a renovação (0x4cda70); depois, só com o crédito 1, sem a marca +0x45 do contrato, com +0x59 acima de 15
/// e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do agradecimento ao técnico (0x58a070). Sem pessoa ou sem
/// prêmio: a original (caixa "Error").
unsafe extern "thiscall" fn add_year_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32) {
    if p == 0 || premio == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(%%I0%%)(this, p, clube, premio, a4, a5, credito);
    }
    add_year_award(this, p, clube, premio, a4, a5, credito, true)
}

/// Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
unsafe extern "thiscall" fn comparar_add_year_award_record(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(%%I0%%);
    if p == 0 || premio == 0 {
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let (t3d, t28) = (cpy10_b(p + 0x3d), cpy10_b(p + 0x28));
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparison::branch("add_year_award_record jogador");
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let r = staff_records_manager_record(this, p);
    if r == 0 {
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 0xd2)], || o(this, p, clube, premio, a4, a5, credito),
        || add_year_award(this, p, clube, premio, a4, a5, credito, false));
    PLACAR.record(igual, || format!("add_year_award_record({:x}, prêmio {:x}) {:x?} x {:x?}", p, cpy10_d(premio), mo, mr));
}

/// { 0x7e4a30 } (cdecl, record_utils.cpp): `match_record_is_bigger_win(a, b)`, pelos recordes do clube depois de cada
/// jogo (0x444930) e pelos da competição (0x49fb93): 1 se `a` é vitória (gols +0x19 > +0x1a, bytes com sinal) por
/// diferença maior que a de `b`, ou igual com mais gols feitos. Registro nulo: a original (caixa "Error", 0).
unsafe extern "C" fn match_record_is_bigger_win(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%)(a, b);
    }
    let (fa, sa, fb, signed_byte_at) = (cpy10_b(a + 0x19) as i8, cpy10_b(a + 0x1a) as i8, cpy10_b(b + 0x19) as i8, cpy10_b(b + 0x1a) as i8);
    if fa <= sa {
        return 0;
    }
    let (da, db) = (fa as i32 - sa as i32, fb as i32 - signed_byte_at as i32);
    (da > db || (da == db && fa > fb)) as u32
}

/// Comparador: pura; registro nulo (a caixa), só a original.
unsafe extern "C" fn comparar_match_record_is_bigger_win(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = match_record_is_bigger_win(a, b);
    PLACAR.record(r == n, || format!("match_record_is_bigger_win({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x7e4b00 } (cdecl, record_utils.cpp): `match_record_is_bigger_defeat(a, b)`, pelos recordes do clube depois de cada
/// jogo (0x444930): 1 se `a` é derrota (+0x19 < +0x1a) por diferença maior que a de `b`, ou igual com mais gols
/// sofridos. Registro nulo: a original (caixa "Error", 0).
unsafe extern "C" fn match_record_is_bigger_defeat(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I2%%)(a, b);
    }
    let (fa, sa, fb, signed_byte_at) = (cpy10_b(a + 0x19) as i8, cpy10_b(a + 0x1a) as i8, cpy10_b(b + 0x19) as i8, cpy10_b(b + 0x1a) as i8);
    if fa >= sa {
        return 0;
    }
    let (da, db) = (sa as i32 - fa as i32, signed_byte_at as i32 - fb as i32);
    (da > db || (da == db && sa > signed_byte_at)) as u32
}

/// Comparador: pura; registro nulo (a caixa), só a original.
unsafe extern "C" fn comparar_match_record_is_bigger_defeat(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I2%%)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = match_record_is_bigger_defeat(a, b);
    PLACAR.record(r == n, || format!("match_record_is_bigger_defeat({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
