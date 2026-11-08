// ---------- %%LEVA%%a leva: a federação e a diretoria: avaliar a seleção na competição, o clube em apuro financeiro e a faixa da confiança da diretoria (manager_manager.cpp) ----------

unsafe fn cpq2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq2_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A linha da diretoria (ou da federação) do clube: 73 bytes em [this], pelo id.
unsafe fn cpq2_linha(this: usize, clube: usize) -> usize {
    cpq2_d(this).wrapping_add(cpq2_d(clube).wrapping_mul(0x49))
}

unsafe fn cpq2_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A confiança `c` (+6 da linha) somada de `v` (16 bits), presa a `[piso, teto]` (primeiro o teto).
unsafe fn add_clamped_i16(c: usize, v: i16, piso: i16, teto: i16) {
    cpq2_pw(c, cpq2_w(c).wrapping_add(v));
    if cpq2_w(c) > teto {
        cpq2_pw(c, teto);
    } else if cpq2_w(c) < piso {
        cpq2_pw(c, piso);
    }
}

/// A saída do técnico da seleção na lista das saídas [this+4] (6 bytes: 1, `motivo`, o id da seleção).
unsafe fn record_national_departure(this: usize, selecao: usize, motivo: u8) {
    let mut reg = [0u8; 8];
    reg[0] = 1;
    reg[1] = motivo;
    reg[2..6].copy_from_slice(&(cpq2_d(selecao) as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    acrescentar(cpq2_d(this + 4), reg.as_mut_ptr() as usize, 6);
}

/// A federação pensa em demitir (0x69aba0 sem conferir o ultimato).
unsafe fn consider_sacking(this: usize, selecao: usize) {
    crate::comparison::branch("cpq_2 a federacao pensa");
    let pensar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x69aba0usize);
    pensar(this, selecao, 0);
}

/// { 0x6a5610 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::review_national_team_performance(comp,
/// selecao)`: seleção com técnico (+0xcf) e competição em [0xadadfc]: com o técnico há menos de 365 dias, a confiança
/// da federação (+6 da linha) sobe a 6500. A nota do técnico na competição (virtual +0x74 dela) muda a confiança: 3:
/// +1250 (teto 10000, piso 8500); 2: +750 (teto 10000, piso 6500), e a seleção da máquina com o técnico
/// há mais de 3650 dias sai se sorteio(15000) + 3750 < reputação (+0x80); 0: −250 e, só com +0xe da linha = −3, 3500 e a
/// saída (mais de 3650 dias) ou, com reputação acima de 6750 e mais de 1460 dias, sorteio(15000) contra a reputação
/// (abaixo: saída; senão a federação pensa, 0x69aba0); −1: −750 (1..3500), saída depois de 1460 dias, sorteio(20000)
/// depois de 730; −2: −1250 (1..3000), saída depois de 730 dias, senão sorteio(15000) + 1250; −3: −1750 (1..3000) e a
/// saída. A avaliação vai para a notícia (0x6a5a60) logo depois da mudança. As saídas vão com motivo 6 (2, 0 e −1 com
/// mais tempo) ou 0.
unsafe extern "thiscall" fn review_national_team_performance(this: usize, comp: usize, selecao: usize) {
    let tec = cpq2_d(selecao + 0xcf);
    let objeto = || cpq2_d(cpq2_d(0xadadfc).wrapping_add(cpq2_d(comp).wrapping_mul(4)));
    if tec == 0 || objeto() == 0 {
        return;
    }
    let dias_de: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let dias = dias_de(0xae2c90, cpq2_d(tec + 0x3e), cpq2_d(tec + 0x42));
    if dias < 0x16d && cpq2_w(cpq2_linha(this, selecao) + 6) < 0x1964 {
        cpq2_pw(cpq2_linha(this, selecao) + 6, 0x1964); // técnico novo
    }
    let o = objeto();
    let nota_de: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cpq2_d(cpq2_d(o) + 0x74));
    let nota = nota_de(o, selecao) as u8 as i8;
    crate::comparison::branch("cpq_2 selecao avaliada");
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x6a5a60usize);
    let c = cpq2_linha(this, selecao) + 6;
    let injury_word_0c = || cpq2_w(selecao + 0x80);
    match nota {
        3 => {
            add_clamped_i16(c, 0x4e2, 0x2134, 0x2710);
            avaliar(this, comp, selecao, 3);
        }
        2 => {
            add_clamped_i16(c, 0x2ee, 0x1964, 0x2710);
            avaliar(this, comp, selecao, 2);
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if humano(selecao, 0, 1, 0, 0) != 0 || dias <= 0xe42 {
                return;
            }
            if (cpq2_sortear(0x3a98) + 0xea6) as i16 >= injury_word_0c() {
                return;
            }
            record_national_departure(this, selecao, 6);
        }
        0 => {
            cpq2_pw(c, cpq2_w(c).wrapping_sub(250));
            if cpq2_w(cpq2_linha(this, selecao) + 0xe) != -3 {
                return;
            }
            cpq2_pw(c, 0xdac);
            avaliar(this, comp, selecao, 0);
            if dias > 0xe42 {
                return record_national_departure(this, selecao, 6);
            }
            if injury_word_0c() <= 0x1a5e || dias <= 0x5b4 {
                return;
            }
            if cpq2_sortear(0x3a98) >= injury_word_0c() as i32 {
                return consider_sacking(this, selecao);
            }
            record_national_departure(this, selecao, 6);
        }
        -1 => {
            add_clamped_i16(c, -750, 1, 0xdac);
            avaliar(this, comp, selecao, -1);
            if dias > 0x5b4 {
                return record_national_departure(this, selecao, 6);
            }
            if dias <= 0x2da {
                return;
            }
            if cpq2_sortear(0x4e20) >= injury_word_0c() as i32 {
                return consider_sacking(this, selecao);
            }
            record_national_departure(this, selecao, 0);
        }
        -2 => {
            add_clamped_i16(c, -1250, 1, 0xbb8);
            avaliar(this, comp, selecao, -2);
            if dias <= 0x2da && cpq2_sortear(0x3a98) + 0x4e2 >= injury_word_0c() as i32 {
                return consider_sacking(this, selecao);
            }
            record_national_departure(this, selecao, 0);
        }
        -3 => {
            add_clamped_i16(c, -1750, 1, 0xbb8);
            avaliar(this, comp, selecao, -3);
            record_national_departure(this, selecao, 0);
        }
        _ => {}
    }
}

// ponytail: solta a notícia da avaliação (0x6a5a60), sorteia e aloca a saída (ou chama a 0x69aba0): só a original;
// a cópia leva os ramos.
unsafe extern "thiscall" fn compare_review_national_team_performance(this: usize, comp: usize, selecao: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, comp, selecao)
}

/// { 0x69eb50 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::is_club_in_financial_trouble(clube)`: clube
/// dirigível (0x544820), fora da seleção (0x53b900), com a confiança da diretoria em +8 da linha abaixo de 3000 e o saldo
/// (64 bits no registro de finanças, 0x5a0590 de [0xae2a38]) negativo: 1. Senão 0. Pelo motor (cantos da torcida).
unsafe extern "thiscall" fn is_club_in_financial_trouble(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let dirigivel: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544820usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if dirigivel(clube) as u8 == 0 || selecao(clube) != 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let f = financas(cpq2_d(0xae2a38), clube);
    if cpq2_w(cpq2_linha(this, clube) + 8) >= 0xbb8 || f == 0 {
        return 0;
    }
    (core::ptr::read_unaligned(f as *const i64) < 0) as u8
}

unsafe extern "thiscall" fn compare_is_club_in_financial_trouble(this: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, clube);
    let n = is_club_in_financial_trouble(this, clube);
    SCOREBOARD.record(r == n, || format!("is_club_in_financial_trouble(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x690b70 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_board_confidence_band(clube)`: a faixa
/// (0..6) da confiança da diretoria (+6 da linha) pelos limites 3000, 3500, 4275, 6500, 8500 e 9500. Pelo texto do
/// relatório da diretoria (0x690d80).
unsafe extern "thiscall" fn get_board_confidence_band(this: usize, clube: usize) -> u8 {
    let v = cpq2_w(cpq2_linha(this, clube) + 6);
    [0xbb8i16, 0xdac, 0x10b3, 0x1964, 0x2134, 0x251c].iter().take_while(|&&l| v >= l).count() as u8
}

unsafe extern "thiscall" fn compare_get_board_confidence_band(this: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, clube);
    let n = get_board_confidence_band(this, clube);
    SCOREBOARD.record(r == n, || format!("get_board_confidence_band(clube {:x}) original={} rust={}", clube, r, n));
    r
}
