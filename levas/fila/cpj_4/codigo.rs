// ---------- %%LEVA%%a leva: a oferta que bate o recorde de valor pago, a condição do jogador e o limite de escalação da partida ----------

unsafe fn cpj4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x4486c0 } (thiscall, ret 8, club_records.cpp): `Club_Records::offer_breaks_fee_record(oferta, exigir)`: o valor
/// da oferta (+0x1b) bate o recorde de valor pago do clube comprador (+8; os recordes de sempre pela 0x452710, criando
/// se preciso)? Tem de ser ao menos 50 × reputação (+0x80) + 500.000; com `exigir`, também ao menos reputação² × a
/// constante 0x967880 (_ftol); e acima de zero e do recorde (+0x313). Oferta nula: 0.
unsafe extern "thiscall" fn offer_breaks_fee_record(this: usize, oferta: usize, exigir: u32) -> u8 {
    if oferta == 0 {
        return 0;
    }
    let clube = cpj4_d(0xae23b4).wrapping_add(cpj4_d(oferta + 8).wrapping_mul(0x245));
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let rec = recordes(this, clube, 1);
    if rec == 0 || clube == 0 {
        return 0;
    }
    let rep = *((clube + 0x80) as *const i16) as i32;
    let valor = cpj4_d(oferta + 0x1b) as i32;
    if valor < rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {
        return 0; // abaixo do mínimo pela reputação
    }
    if exigir != 0 && valor < match_pl_ftol(rep as f64 * rep as f64 * read_f64(0x967880)) {
        return 0;
    }
    (valor != 0 && valor > cpj4_d(rec + 0x313) as i32) as u8
}

/// { 0x62c590 } (thiscall, ret 8, injury.cpp): `Injury::get_condition(jogador, em_porcento)`: a condição do jogador:
/// forma (+0xa) × condição de jogo (+8) / 10.000 do registro de 31 bytes [this] do id, no mínimo 100 e no máximo 9000
/// enquanto ele se recupera (0x62c400 = 1); em porcentagem (× 0x96bac8 + 0x968968, _ftol), no máximo 100. Pessoa fora
/// dos jogadores ([0xae2364]): 100 ou 10.000. Jogador nulo: 0.
unsafe extern "thiscall" fn get_condition(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    if jogador == 0 {
        return 0;
    }
    let id = cpj4_d(jogador) as i32;
    if id >= cpj4_d(0xae2364) as i32 {
        return if em_porcento != 0 { 100 } else { 10_000 };
    }
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let recuperando = estado(this, jogador) == 1;
    let r = cpj4_d(this).wrapping_add((id as usize).wrapping_mul(31));
    let v = *((r + 0xa) as *const i16) as i32 * *((r + 8) as *const i16) as i32 / 10_000;
    let mut c = if recuperando && v as i16 >= 9000 {
        9000 // o teto de quem se recupera
    } else if v as i16 > 100 {
        v
    } else {
        100
    };
    if em_porcento != 0 {
        c = match_pl_ftol(c as i16 as f64 * read_f32(0x96bac8) as f64 + read_f64(0x968968));
        if c as i16 > 100 {
            c = 100;
        }
    }
    c as u16
}

/// { 0x8cf7e0 } (thiscall, ret 8, transfer_manager.cpp): `get_selection_limit(partida, saida)`: zera os 6 bytes da
/// saída, pede a tabela de 42 bytes das restrições da partida (0x8cf6b0 com 1; o `this` passa direto) e fica com a
/// primeira que existe (≠ 0xff): +5 ou +3 (tipo 1), +2 ou +0 (tipo 2), +0x16 com a quantidade em +0x17 (tipo 3, a
/// idade em +1). O tipo vai em +0 e a quantidade em +5.
unsafe extern "thiscall" fn get_selection_limit(this: usize, partida: usize, saida: *mut u8) {
    core::ptr::write_unaligned(saida as *mut u32, 0);
    core::ptr::write_unaligned(saida.add(4) as *mut u16, 0);
    let mut t = [0u8; 0x2c];
    let restricoes: unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32 = core::mem::transmute(0x8cf6b0usize);
    restricoes(this, partida, t.as_mut_ptr(), 1);
    let (tipo, n) = if t[5] != 0xff {
        (1, t[5])
    } else if t[3] != 0xff {
        (1, t[3])
    } else if t[2] != 0xff {
        (2, t[2]) // o 3º limite
    } else if t[0] != 0xff {
        (2, t[0])
    } else if t[0x16] != 0xff {
        *saida.add(1) = t[0x16];
        (3, t[0x17])
    } else {
        return;
    };
    *saida = tipo;
    *saida.add(5) = n;
}

/// Comparador: a 0x452710 cria os recordes do clube na primeira vez (a original primeiro; a cópia acha os mesmos):
/// pelo valor.
unsafe extern "thiscall" fn compare_offer_breaks_fee_record(this: usize, oferta: usize, exigir: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I0%%)(this, oferta, exigir);
    let n = offer_breaks_fee_record(this, oferta, exigir);
    SCOREBOARD.record(r == n, || format!("offer_breaks_fee_record({:x}, {}) original={} rust={}", oferta, exigir, r, n));
    r
}

/// Comparador: pelo valor (a 0x62c400 só lê).
unsafe extern "thiscall" fn compare_get_condition(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(%%I1%%)(this, jogador, em_porcento);
    let n = get_condition(this, jogador, em_porcento);
    SCOREBOARD.record(r == n, || format!("get_condition({:x}, {}) original={} rust={}", jogador, em_porcento, r, n));
    r
}

/// Comparador: lado a lado pelos 6 bytes da saída (a 0x8cf6b0 só escreve na tabela local).
unsafe extern "thiscall" fn compare_get_selection_limit(this: usize, partida: usize, saida: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8)>(%%I2%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(saida as usize, 6)], || o(this, partida, saida), || get_selection_limit(this, partida, saida));
    SCOREBOARD.record(igual, || format!("get_selection_limit({:x}) {:x?} x {:x?}", partida, mo, mr));
}
