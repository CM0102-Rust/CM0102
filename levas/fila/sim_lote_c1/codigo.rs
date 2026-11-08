// ---------- %%LEVA%%a leva: se a liga tem rebaixamento, a data da rodada do modelo da liga e a importância do jogo da liga (league.cpp) ----------

/// { 0x6847b0 } (thiscall, league.cpp): 1 se a liga tem vagas de rebaixamento (byte +0xc1 não nulo).
unsafe extern "thiscall" fn has_relegation(liga: usize) -> u32 {
    (*((liga + 0xc1) as *const u8) != 0) as u32 // tem rebaixamento
}

/// { 0x68a160 } (cdecl, league.cpp): a data (Date::Date, 0x549f70, num Date local) de `dia`/`mes`/
/// (`anos` + `base`)/`semana` no registro `i` (palavra com sinal, 0x41 bytes) de `rodadas`: dia do ano
/// na palavra +0, ano − `base` na palavra +2, `codigo` no byte +4 e `valor` em +0x3d. Devolve o registro.
unsafe extern "C" fn league_round_set_date(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u16; 4];
    construir(&mut d, dia, mes, anos.wrapping_add(base), semana);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    core::ptr::write_unaligned(r as *mut u16, d[0]);
    core::ptr::write_unaligned((r + 2) as *mut u16, d[1].wrapping_sub(base as u16)); // ano relativo
    *((r + 4) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((r + 0x3d) as *mut u32, valor);
    r
}

/// { 0x68a3e0 } (thiscall, ret 0xc, league.cpp): a importância do jogo da `partida` para o clube do
/// `lado` (0 = casa, +0x1c; senão +0x20): (byte +0x51 da liga − 10) / 4 + 10. Grava 0xff em `saida`;
/// com o clube na tabela (+0xb1, linhas de 59 bytes, +0x3e clubes) e mais de 10 rodadas (+0xa9): +1
/// com +0x34 da partida abaixo de 3; senão, linha sem marca (+0x37 = 0xff), +1 e a margem do último
/// dia (0x68a4d0, que pode trocar a importância) em `saida`. Só o al é devolvido (quem chama usa al).
unsafe extern "thiscall" fn get_match_importance(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    let margem: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x68a4d0usize);
    *saida = 0xff;
    let clube = *((partida + if lado as u8 == 0 { 0x1c } else { 0x20 }) as *const usize);
    let mut imp = ((*((liga + 0x51) as *const i8) as i32 - 10) / 4) as u8;
    imp = imp.wrapping_add(10); // importância base
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let linha = |i: i16| (*((liga + 0xb1) as *const usize)).wrapping_add((i as isize * 59) as usize);
    // ponytail: a original testa i == −1 depois da busca; i começa em 0 e para antes de `clubes`, nunca é −1
    let mut i = 0i16;
    loop {
        if i >= clubes {
            return imp;
        }
        if *(linha(i) as *const usize) == clube {
            break;
        }
        i = i.wrapping_add(1);
    }
    if core::ptr::read_unaligned((liga + 0xa9) as *const i16) <= 10 {
        return imp;
    }
    if core::ptr::read_unaligned((partida + 0x34) as *const i16) < 3 {
        return imp.wrapping_add(1);
    }
    let r = linha(i);
    if *((r + 0x37) as *const u8) != 0xff {
        return imp;
    }
    let mut local = imp.wrapping_add(1);
    *saida = margem(liga, r, &mut local);
    local
}

unsafe extern "thiscall" fn compare_has_relegation(liga: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(liga);
    let n = has_relegation(liga);
    SCOREBOARD.record(r == n, || format!("has_relegation original={} rust={}", r, n));
    r
}

unsafe extern "C" fn compare_league_round_set_date(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 0x41)], || o(rodadas, i, dia, mes, anos, semana, codigo, base, valor),
        || league_round_set_date(rodadas, i, dia, mes, anos, semana, codigo, base, valor));
    SCOREBOARD.record(igual && a == b, || format!("league_round_set_date({}, {}/{} +{}) {:x?} x {:x?}", i as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "thiscall" fn compare_get_match_importance(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    // a 0x68a4d0 só lê a liga e grava na local e na `saida`: pode rodar nos dois lados
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u8>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(saida as usize, 1)], || o(liga, partida, lado, saida),
        || get_match_importance(liga, partida, lado, saida));
    SCOREBOARD.record(igual && r == n, || format!("get_match_importance original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}
