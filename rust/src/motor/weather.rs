use super::*;

/// Postponement outcome: random(n) != 0 -> 0 (no change), 0 -> 6 (abandoned).
///
/// [pt] O fim do adiamento: sorteio(n) diferente de zero não adia (0), zero abandona (6).
pub(crate) unsafe fn abandon_if_draw_zero(n: i32) -> u8 {
    if friendly_random(n) != 0 {
        0
    } else {
        6
    }
}

/// Postponement outcome: random(n) == 0 -> 0, otherwise 5 (postponed).
///
/// [pt] O fim do adiamento: sorteio(n) zero não adia (0), diferente de zero adia (5).
pub(crate) unsafe fn postpone_if_draw_nonzero(n: i32) -> u8 {
    if friendly_random(n) == 0 {
        0
    } else {
        5
    }
}

/// Decides whether the weather postpones a match: only the home club's league matches at an uncovered stadium, outside
/// the end of the key nation's season; by the evening slot and by the weather.
///
/// `Weather::check_postponement(match)`, from the match day (`set_up_the_day`) and from the match preparation (`prepare_match`).
/// Without a stadium (+0x18) with a city (+0x38) and places (+0x3c), without a competition (+0x14), with the home club
/// (+0x1c) outside a league (+0x57) or the match outside its league, or a covered stadium (+0x4c): 0. In the home
/// club's key nation (`key_nation_index_of_club`, table 0xb63d60 of 0x48 bytes), near the end of the year's season (+0x39/+0x3b): 0. In
/// the evening slot (+0x3f = 2), by the home club's finances (`get_club_finance`, band `get_wealth_band`): random(1000/2500/5000) zero
/// returns 0x12. Then the match weather (`get_match_weather`, written into the argument's slot as in the original): rain 5 at 1 to
/// 13 degrees, an uncovered stadium (+0x4d) of up to 20000 places: a random draw by places and home club (+0x64), zero
/// = 9 (postponement); rain 8 (up to 15000 places) and 9 (ice; by wind 4 and by 10000 places): a chain of random draws
/// that returns 0, 5 or 6.
/// Original: `Weather::check_postponement` (0x923630, weather.cpp, thiscall, ret 4).
///
/// [pt] { 0x923630 } (thiscall, ret 4, weather.cpp): `Weather::check_postponement(jogo)`, pelo dia dos jogos (0x6b3c10) e
/// pela preparação da partida (0x6b7f20). Sem estádio (+0x18) com cidade (+0x38) e lugares (+0x3c), sem competição
/// (+0x14), com o mandante (+0x1c) fora de liga (+0x57) ou o jogo fora da liga dele, ou estádio coberto (+0x4c): 0. Na
/// nação-chave do mandante (0x669360, tabela 0xb63d60 de 0x48 bytes), perto do fim da temporada do ano (+0x39/+0x3b): 0.
/// Na fatia da noite (+0x3f = 2), pelas finanças do mandante (0x5a0590, faixa 0x5986b0): sorteio(1000/2500/5000) zero
/// devolve 0x12. Depois o clima do jogo (0x923010, gravado na vaga do argumento como na original): chuva 5 com 1 a 13
/// graus, estádio descoberto (+0x4d) de até 20000 lugares: sorteio por lugares e mandante (+0x64), zero = 9; chuva 8
/// (até 15000 lugares) e 9 (pelo vento 4 e por 10000 lugares): uma cadeia de sorteios que devolve 0, 5 ou 6.
pub(crate) unsafe extern "thiscall" fn check_postponement(this: usize, game: usize) -> u8 {
    let game = Fixture(game);
    let est = game.stadium();
    if est == 0 || read_usize(est + stadium::CITY) == 0 || read_usize(est + stadium::CAPACITY) == 0 || game.competition().is_null() {
        return 0;
    }
    let home = game.home_club();
    if home.division().addr() == 0 {
        return 0;
    }
    let k: i8 = if !home.is_null() { (exe::key_nation_index_of_club(home) as i8) } else { -1 };
    if game.competition() != Competition(home.division().addr()) || read_u8(est + stadium::COVERED) != 0 {
        return 0;
    }
    if (0..0x23).contains(&k) {
        let e = k as usize * 0x48;
        let end = core::ptr::read_unaligned((G_LEAGUE_WORLD_CUP_END_DATE + e) as *const i16);
        let year = core::ptr::read_unaligned((0xb63d9b + e) as *const u16);
        let day = core::ptr::read_unaligned(G_CURRENT_DATE as *const i16);
        let year_current = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
        if end > 0x3c {
            if year == year_current && (end as i32 - 0x3c) < day as i32 {
                return 0;
            }
        } else if year == year_current && day > 0x154 {
            return 0;
        }
    }
    if game.time_slice() == 2 { // cpu5: the night slice
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), home);
        if fin != 0 {
            let f = exe::finance_get_wealth_band(fin) as i8;
            let n = if f == -2 {
                1000
            } else if f == -1 && (read_u8(fin + 0x166) as i8) > 0xf {
                0x9c4
            } else {
                5000
            };
            crate::comparison::branch("cpu_5 finances rollover");
            if friendly_random(n) == 0 { // cpu5: the finances roll
                return 0x12;
            }
        }
    }
    // simplification: the original writes the weather into the slot of its own argument; if the weather is not written (no regions), what remains are the
    // bytes of the game pointer, as here.
    let mut cl = game.addr() as u32;
    exe::weather_get_match_weather(this, &mut cl as *mut u32 as usize, game);
    let [vento, chuva, temp, _] = cl.to_le_bytes();
    let temp = temp as i8;
    let est = game.stadium();
    let home = game.home_club();
    match chuva {
        5 => {
            if !(1..0xe).contains(&temp) || read_u8(est + stadium::UNDER_SOIL_HEATING) != 0 {
                return 0;
            }
            let cap = read_i32(est + stadium::CAPACITY);
            if cap > 0x4e20 {
                return 0;
            }
            let mandante = home.professional_status() == 1;
            let n = if cap > 0x30d4 {
                0xfa
            } else if cap > 0x1388 {
                if mandante { 0x96 } else { 0x1e }
            } else if mandante {
                0x32
            } else {
                0xa
            };
            crate::comparison::branch("cpu_5 heavy rain");
            if friendly_random(n) != 0 { // cpu5: the heavy rain random draw
                0
            } else {
                9
            }
        }
        8 => {
            if read_u8(est + stadium::UNDER_SOIL_HEATING) != 0 {
                return 0;
            }
            let cap = read_i32(est + stadium::CAPACITY);
            if cap > 0x3a98 {
                return 0;
            }
            let mandante = home.professional_status() == 1;
            if cap > 0x1d4c && mandante {
                if friendly_random(4) != 0 {
                    return 5;
                }
                return abandon_if_draw_zero(4);
            }
            if cap <= 0x1d4c && !mandante {
                if friendly_random(0x19) != 0 {
                    return 5;
                }
                return if friendly_random(3) == 0 { 0 } else { 6 };
            }
            if friendly_random(0x1e) == 0 {
                return 6;
            }
            postpone_if_draw_nonzero(0xf)
        }
        9 => {
            let (d1, d2, d3) = if vento == 4 { (10, 0x14, 10) } else { (0x14, 10, 4) };
            if read_u8(est + stadium::UNDER_SOIL_HEATING) != 0 && friendly_random(d1) != 0 {
                return 0;
            }
            if read_i32(est + stadium::CAPACITY) > 0x2710 {
                if vento == 4 {
                    if friendly_random(d2) != 0 {
                        return 5;
                    }
                    return abandon_if_draw_zero(d3);
                }
                if friendly_random(d2) == 0 {
                    return abandon_if_draw_zero(d3);
                }
                return 5;
            }
            if friendly_random(0x32) == 0 {
                return 6;
            }
            postpone_if_draw_nonzero(if vento == 4 { 8 } else { 4 })
        }
        _ => 0,
    }
}

// simplification: rolls: original only; the copy takes the branches.

/// Returns the finance record (0x167 bytes) by the club id; for a B club with an A partner, the A one (`exe::club_get_linked_b_team`); null
/// club, error box.
///
/// `FinanceTable::get_club_finance(club)` (finance.cpp): the finance record (0x167 bytes at [this]) of the club by id
/// (+0); a B club with an A partner (`exe::club_get_linked_b_team` with the returned byte 0): the A one's. Null club: the "Error" box of
/// finance.CPP (as in the original).
/// Original: `FinanceTable::get_club_finance` (0x5a0590, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5a0590 } (thiscall, ret 4, finance.cpp): `FinanceTable::get_club_finance(clube)`: o registro de finanças
/// (0x167 bytes em [this]) do clube pelo id (+0); clube B com parceiro A (0x540a50 com o byte devolvido 0): o do A.
/// Clube nulo: a caixa "Error" de finance.CPP (a original).
pub(crate) unsafe extern "thiscall" fn get_club_finance(this: usize, club: usize) -> usize {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_CLUB_FINANCE)(this, club);
    }
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a: u32 = 0;
    let mut c = club;
    let p = partner(club, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        c = p;
    }
    read_usize(this).wrapping_add((read_usize(c) as u32).wrapping_mul(0x167) as usize)
}

/// City's weather region record ([this] + r x 0x6e) if 0 <= r < [this+8].
///
/// [pt] A região do clima de uma cidade (+0x34) se for válida (0 ≤ r < [this + 8]): [this] + r × 0x6e.
pub(crate) unsafe fn weather_region(this: usize, city: usize) -> Option<usize> {
    let r = read_i32(city + city::WEATHER);
    if r >= 0 && r < read_i32(this + 8) {
        Some(read_usize(this).wrapping_add(r as usize * 0x6e))
    } else {
        None
    }
}

/// Returns the match weather: wind (0..4), rain (0, 1, 2, 3, 5), temperature and its band, from the city, its region
/// and the period by the day; below zero, the rain becomes snow/ice (6..9).
///
/// `Weather::get_match_weather(out, match)`, in the match preparation (`prepare_match`, at +0x4756 of the engine: the wind
/// weighs on long shots), in the postponement (`check_postponement`) and in the screens. Without regions ([this], [this + 8]):
/// nothing. The city: the stadium's (+0x18 → +0x38), otherwise that of the home club's stadium (+0x1c → +0x69 → +0x38),
/// otherwise the capital of its nation (+0x53 → +0x7a). With a city: out[3] = the city's wind (table [this + 4], 4
/// bytes per city, byte 3) and its region (otherwise that of the nation's capital +0x1f, otherwise the first). The
/// period by the match day (+0x2a; without a match, the game day) against the region's limits +0x1e/+0x31/+0x44/+0x57
/// (3, 0, 1, 2, 3); in the period (19 bytes from +0x20): out[0] = wind by the city's reading 2 (+0x20..+0x24: 0..4),
/// out[1] = rain by reading 1 (+0x25..+0x29: 0, 1, 2, 3, 5), and the temperature band by reading 2 (+0x2a..+0x30: 1, 0,
/// 2..6, otherwise 3). Without a city: out[3], [0] and [1] = 0 and band 2. out[2] = the temperature (`get_temperature`); below
/// zero, the rain becomes snow/ice: 1 → 6, 2 and 3 → 6/7/8 and 5 → 8/7/9 by the city's wind 0/1/other.
/// Original: `Weather::get_match_weather` (0x923010, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x923010 } (thiscall, ret 8, weather.cpp): `Weather::get_match_weather(saida, jogo)`, na preparação da partida
/// (0x6b7f20, em +0x4756 do motor: o vento pesa nos chutes de longe), no adiamento (0x923630) e nas telas. Sem regiões
/// ([this], [this + 8]): nada. A cidade: a do estádio (+0x18 → +0x38), senão a do estádio do mandante (+0x1c → +0x69 →
/// +0x38), senão a capital da nação dele (+0x53 → +0x7a). Com cidade: saida[3] = o vento da cidade (tabela [this + 4], 4
/// bytes por cidade, byte 3) e a região dela (senão a da capital da nação +0x1f, senão a primeira). O período pelo dia do
/// jogo (+0x2a; sem jogo o dia do jogo) contra os limites +0x1e/+0x31/+0x44/+0x57 da região (3, 0, 1, 2, 3); no período
/// (19 bytes a partir de +0x20): saida[0] = vento pela leitura 2 da cidade (+0x20..+0x24: 0..4), saida[1] = chuva pela
/// leitura 1 (+0x25..+0x29: 0, 1, 2, 3, 5), e a faixa da temperatura pela leitura 2 (+0x2a..+0x30: 1, 0, 2..6, senão 3).
/// Sem cidade: saida[3], [0] e [1] = 0 e a faixa 2. saida[2] = a temperatura (0x9232c0); abaixo de zero, a chuva vira
/// neve/gelo: 1 → 6, 2 e 3 → 6/7/8 e 5 → 8/7/9 pelo vento da cidade 0/1/outro.
pub(crate) unsafe extern "thiscall" fn get_match_weather(this: usize, s: *mut u8, game: usize) {
    let game = Fixture(game);
    if read_usize(this) == 0 || read_usize(this + 8) == 0 {
        return;
    }
    let mut cid = 0usize;
    if !game.is_null() {
        let est = game.stadium();
        if est != 0 {
            cid = read_usize(est + stadium::CITY);
        }
    }
    if cid == 0 {
        let home = Club(game.home_club().addr());
        if !home.is_null() {
            let c = home.stadium().addr();
            if c != 0 && read_usize(c + stadium::CITY) != 0 {
                cid = read_usize(c + stadium::CITY);
            } else {
                let n = home.nation().addr();
                if n != 0 && read_usize(n + 0x7a) != 0 {
                    cid = read_usize(n + 0x7a);
                }
            }
        }
    }
    let cidades = read_usize(this + 4);
    let mut reg = read_usize(this);
    if cid != 0 {
        let leituras = cidades.wrapping_add(read_usize(cid).wrapping_mul(4));
        *s.add(3) = *((leituras + 3) as *const u8);
        if let Some(r) = weather_region(this, cid) {
            reg = r;
        } else {
            let n = read_usize(cid + city::NATION);
            if n != 0 {
                let capital = read_usize(n + 0x7a);
                if capital != 0 {
                    if let Some(r) = weather_region(this, capital) {
                        reg = r;
                    }
                }
            }
        }
    } else {
        *s.add(3) = 0;
    }
    let day = if !game.is_null() {
        (game.day() as i16)
    } else {
        core::ptr::read_unaligned(G_CURRENT_DATE as *const i16)
    };
    let limit = |o: usize| core::ptr::read_unaligned((reg + o) as *const i16);
    let p: usize = if day < limit(0x1e) {
        3
    } else if day < limit(0x31) {
        0
    } else if day < limit(0x44) {
        1
    } else if day < limit(0x57) {
        2
    } else {
        3
    };
    let band: u32 = if cid == 0 {
        *s = 0;
        *s.add(1) = 0;
        2
    } else {
        let per = reg + 19 * p;
        let lim = |o: usize| read_i8(per + o);
        let leitura = |k: usize| read_i8(cidades.wrapping_add(read_usize(cid).wrapping_mul(4)) + k);
        let a = leitura(2);
        *s = if a < lim(0x20) {
            0
        } else if a < lim(0x21) {
            crate::comparison::branch("cpu_6 light wind");
            1 // cpu6: o vento fraco
        } else if a < lim(0x22) {
            crate::comparison::branch("cpu_6 moderate wind");
            2 // cpu6: o vento moderado
        } else if a < lim(0x23) {
            crate::comparison::branch("cpu_6 strong wind");
            3 // cpu6: o vento forte
        } else if a >= lim(0x24) {
            0
        } else {
            4
        };
        let c = leitura(1);
        *s.add(1) = if c < lim(0x25) {
            0
        } else if c < lim(0x26) {
            1
        } else if c < lim(0x27) {
            2
        } else if c < lim(0x28) {
            3
        } else if c < lim(0x29) {
            5
        } else {
            0
        };
        let t = leitura(2);
        if t < lim(0x2a) {
            1
        } else if t < lim(0x2b) {
            0
        } else if t < lim(0x2c) {
            2
        } else if t < lim(0x2d) {
            3
        } else if t < lim(0x2e) {
            4
        } else if t < lim(0x2f) {
            5
        } else if t < lim(0x30) {
            6
        } else {
            3
        }
    };
    let t = exe::weather_get_temperature(this, band, game) as i8;
    *s.add(2) = t as u8;
    if t >= 0 {
        return;
    }
    let vento = *s.add(3) as i8;
    match *s.add(1) {
        1 => *s.add(1) = 6,
        2 | 3 => *s.add(1) = match vento {
            0 => 6,
            1 => 7,
            _ => 8,
        },
        5 => *s.add(1) = match vento {
            0 => 8,
            1 => 7,
            _ => 9,
        },
        _ => {}
    }
}

/// Returns the match temperature: (day × band) modulo 6/7/8 plus the band base (−5 to 35; other band −100); in the
/// evening slot, − 2 below 10, otherwise × 0.8.
///
/// `Weather::get_temperature(band, match)`, from the match weather. With the day d (of the match +0x2a, unsigned;
/// without a match, the game day) and x = d × band: band 0: x % 6 + 1; 1: x % 6 − 5; 2: x % 7 + 7; 3: x % 8 + 14; 4: x
/// % 7 + 22; 5: x % 6 + 29; 6: x % 6 + 35; other: −100. A match in the evening slot (+0x3f = 2): below 10, − 2;
/// otherwise × 0.8 truncated. (It reads the match's +0x3f even without a match, as the original does.)
/// Original: `Weather::get_temperature` (0x9232c0, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x9232c0 } (thiscall, ret 8, weather.cpp): `Weather::get_temperature(faixa, jogo)`, pelo clima da partida. Com o
/// dia d (do jogo +0x2a, sem sinal; sem jogo o dia do jogo) e x = d × faixa: faixa 0: x % 6 + 1; 1: x % 6 − 5; 2:
/// x % 7 + 7; 3: x % 8 + 14; 4: x % 7 + 22; 5: x % 6 + 29; 6: x % 6 + 35; outra: −100. Jogo da fatia da noite (+0x3f = 2):
/// abaixo de 10, − 2; senão × 0,8 truncado. (Lê +0x3f do jogo mesmo sem jogo, como a original.)
pub(crate) unsafe extern "thiscall" fn get_temperature(_this: usize, band: u32, game: usize) -> u8 {
    let game = Fixture(game);
    let f = band as u8 as i8 as i32;
    let d: i32 = if !game.is_null() {
        game.day() as i32
    } else {
        core::ptr::read_unaligned(G_CURRENT_DATE as *const i16) as i32
    };
    let x = d.wrapping_mul(f);
    let t: i8 = match f {
        0 => (x % 6 + 1) as i8,
        1 => (x % 6 - 5) as i8,
        2 => (x % 7 + 7) as i8,
        3 => (x % 8 + 14) as i8,
        4 => (x % 7 + 0x16) as i8,
        5 => (x % 6 + 0x1d) as i8,
        6 => (x % 6 + 0x23) as i8,
        _ => -100,
    };
    if game.time_slice() == 2 {
        if t < 10 {
            return (t as u8).wrapping_sub(2);
        }
        return ftol(t as f64 * read_f64(rdata::F64_0_8)) as u8;
    }
    t as u8
}

/// Returns the pitch condition from the rain, the temperature, the stadium flag and places and a fixture remainder
/// modulo 20: 0, 1, 2, 4, 8, 0x10, 0x20 or 0x40; covered stadium, 0.
///
/// `Weather::get_pitch_condition(weather, match)`, in the match preparation (`prepare_match`, at +1 of the engine: pitch 2 and
/// 0x20 weigh on shots). Without a match, without a stadium (+0x18) or a covered stadium (+0x4c): 0. h = (day +0x2a +
/// +0x10 + +0xc + +0) % 20. By the rain weather[1] and the temperature weather[2] (1 to 28 degrees), with the stadium
/// flag +0x4d and the places +0x3c: rain 5: 2/1 or 8/1; rain 1: 2 or 0 by h < 6/4; rain 2 and 3: by h < 4/1/3; rain 4:
/// 2 or 1/2; below 1 degree, rain 1 and 8: 2, 0x10 or 1; rain 9: 2, 1 or 0x20/1; no rain at 29 degrees or more: 0x40 or
/// 4; otherwise 0.
/// Original: `Weather::get_pitch_condition` (0x923970, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x923970 } (thiscall, ret 8, weather.cpp): `Weather::get_pitch_condition(clima, jogo)`, na preparação da partida
/// (0x6b7f20, em +1 do motor: o gramado 2 e 0x20 pesam nos chutes). Sem jogo, sem estádio (+0x18) ou estádio coberto
/// (+0x4c): 0. h = (dia +0x2a + +0x10 + +0xc + +0) % 20. Pela chuva clima[1] e a temperatura clima[2] (1 a 28 graus),
/// com o estádio de marca +0x4d e os lugares +0x3c: chuva 5: 2/1 ou 8/1; chuva 1: 2 ou 0 por h < 6/4; chuva 2 e 3: por
/// h < 4/1/3; chuva 4: 2 ou 1/2; abaixo de 1 grau, chuva 1 e 8: 2, 0x10 ou 1; chuva 9: 2, 1 ou 0x20/1; sem chuva a 29
/// graus ou mais: 0x40 ou 4; senão 0.
pub(crate) unsafe extern "thiscall" fn get_pitch_condition(_this: usize, clima: *const u8, game: usize) -> u8 {
    if game == 0 {
        return 0;
    }
    let est = read_usize(game + 0x18);
    if est == 0 || *((est + stadium::COVERED) as *const u8) != 0 {
        return 0;
    }
    let sum = (core::ptr::read_unaligned((game + 0x2a) as *const u16) as i32)
        .wrapping_add(read_i32(game + 0x10))
        .wrapping_add(read_i32(game + 0xc))
        .wrapping_add(read_i32(game));
    let h = sum % 20;
    let chuva = *clima.add(1);
    let t = *clima.add(2) as i8;
    let mark = *((est + stadium::UNDER_SOIL_HEATING) as *const u8) != 0;
    let places = read_i32(est + stadium::CAPACITY);
    let morno = (1..0x1d).contains(&t);
    if chuva == 5 && morno {
        if mark {
            return if places > 0x6b6c { 2 } else { 1 };
        }
        return if places <= 0x445c { 8 } else { 1 };
    }
    if chuva == 1 && morno {
        if !mark {
            return 2;
        }
        let cut = if places > 0x6b6c { 4 } else { 6 };
        return if h >= cut { 0 } else { 2 };
    }
    if (chuva == 2 || chuva == 3) && morno {
        if mark {
            if places <= 0x6b6c {
                return if h >= 4 { 0 } else { 2 };
            }
            return if h != 0 { 0 } else { 2 };
        }
        return if h >= 3 { 0 } else { 2 };
    }
    if chuva == 4 && morno {
        if mark {
            if places <= 0x6b6c {
                return 2;
            }
            return if h >= 3 { 0 } else { 2 };
        }
        return if places > 0x445c { 2 } else { 1 };
    }
    if t < 1 && (chuva == 1 || chuva == 8) {
        if mark || places > 0xafc8 {
            return 2;
        }
        return if places <= 0x30d4 { 0x10 } else { 1 };
    }
    if chuva == 9 {
        if mark {
            return if places > 0x7530 { 2 } else { 1 };
        }
        return if places <= 0x4e20 { 0x20 } else { 1 };
    }
    if chuva == 0 && t >= 0x1d {
        return if places <= 0x88b8 { 0x40 } else { 4 };
    }
    0
}

/// Next line of the file into buf (utils read_config_line `exe::utils_read_config_line`); 0 at end.
///
/// [pt] A linha seguinte do arquivo (0x90cba0: FILE*, buffer), 0 no fim.
pub(crate) unsafe fn read_config_line(f: usize, buf: *mut u8) -> i32 {
    exe::utils_read_config_line(f, buf)
}

/// Updates each city's weather daily: the three readings become (reading + random(100)) × 0.5, clamped to 0..100; one
/// time in ten, a new wind = random(3).
///
/// `Weather::update_city_weather()`, every day from the game's day loop (0x5cd9aa, [0xde2f74]). Without the city table
/// (+4, 4 bytes per city): nothing. Each city ([0xae2354], re-read on every loop): the three readings (+0, +1, +2)
/// become (reading + random(100)) × 0.5 truncated, clamped to 0..100; random(10) zero gives the new wind (+3) =
/// random(3).
/// Original: `Weather::update_city_weather` (0x922ee0, weather.cpp, thiscall).
///
/// [pt] { 0x922ee0 } (thiscall, weather.cpp): `Weather::update_city_weather()`, todo dia pelo laço do dia do jogo (0x5cd9aa,
/// [0xde2f74]). Sem a tabela das cidades (+4, 4 bytes por cidade): nada. Cada cidade ([0xae2354], relida a cada volta):
/// as três medidas (+0, +1, +2) viram (medida + sorteio(100)) × 0,5 truncado, presas em 0..100; sorteio(10) zerado dá
/// o vento novo (+3) = sorteio(3).
pub(crate) unsafe extern "thiscall" fn update_city_weather(this: usize) {
    if read_usize(this + 4) == 0 || read_usize(G_CITY_COUNT) as i32 <= 0 {
        return;
    }
    crate::comparison::branch("cpt_5 weather of the day");
    let mut i = 0usize;
    loop {
        for k in 0..3usize { // cpt5: as três medidas
            let p = read_usize(this + 4) + i * 4 + k;
            let r = friendly_random(100);
            *(p as *mut u8) = ftol((r as i16 as i32 + *(p as *const i8) as i32) as f64 * read_f64(rdata::F64_0_5)) as u8;
            let p = read_usize(this + 4) + i * 4 + k;
            let v = *(p as *const i8);
            if v < 0 {
                *(p as *mut u8) = 0;
            } else if v > 0x64 {
                *(p as *mut u8) = 0x64;
            }
        }
        if friendly_random(10) == 0 { // cpt5: one in ten changes the wind
            crate::comparison::branch("cpt_5 new wind");
            let r = friendly_random(3); // cpt5: o vento novo
            *((read_usize(this + 4) + i * 4 + 3) as *mut u8) = r as u8;
        }
        i += 1;
        if i as i32 >= read_usize(G_CITY_COUNT) as i32 {
            break;
        }
    }
}

// simplification: rolls 3 or 4 times per city: original only; the copy takes the branches.

/// Reads one weather.cfg line with the region id and name ("%ld %s") and its four periods (`read_region_period`) at +0x1e, +0x31,
/// +0x44 and +0x57.
///
/// `Weather::read_region(f, region)`: one line of weather.cfg (`exe::utils_read_config_line`, buffer of 2000) with "%ld %s" (0xad4e24) into
/// the id (+0) and the name (+4) of the region (0x6e bytes); then the four periods (`read_region_period`) at +0x1e, +0x31, +0x44
/// and +0x57. Returns 1 if it read everything (only al, as in the original).
/// Original: `Weather::read_region` (0x922c40, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x922c40 } (thiscall, ret 8, weather.cpp): `Weather::read_region(f, regiao)`: uma linha do weather.cfg (0x90cba0,
/// buffer de 2000) com "%ld %s" (0xad4e24) no id (+0) e no nome (+4) da região (0x6e bytes); depois os quatro
/// períodos (0x922d20) em +0x1e, +0x31, +0x44 e +0x57. Devolve 1 se leu tudo (só o al, como a original).
pub(crate) unsafe extern "thiscall" fn read_region(this: usize, f: usize, region: usize) -> u8 {
    let read: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    crate::comparison::branch("cpt_5 weather region");
    let mut row = [0u8; 2000];
    if read_config_line(f, row.as_mut_ptr()) == 0 {
        return 0;
    }
    if read(row.as_ptr(), 0xad4e24, region, region + 4) != 2 {
        return 0;
    }
    for k in [0x1e, 0x31, 0x44, 0x57] {
        if exe::weather_read_region_period(this, f, region + k) == 0 {
            return 0;
        }
    }
    1
}

// simplification: reads from the file (cannot read twice): original only; the copy carries the branch.

/// Reads one line with the name and 18 integers (the day, five W, five P, seven H): the period gets the day (word) and
/// the 17 bytes; a null file gives the "Error" box.
///
/// `Weather::read_region_period(f, period)`: a null file gives the "Error" box (weather.cpp, 0xea) and 0. One line
/// (`exe::utils_read_config_line`) with the format at 0xad4e2c (the name and 18 integers: the day, five W, five P and seven H); with all 19
/// read, the period (0x13 bytes) receives the day (word at +0) and the next 17 bytes (+2..+0x12). Returns 1 if read
/// (only al).
/// Original: `Weather::read_region_period` (0x922d20, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x922d20 } (thiscall, ret 8, weather.cpp): `Weather::read_region_period(f, periodo)`: arquivo nulo é a caixa
/// "Error" (weather.cpp, 0xea) e 0. Uma linha (0x90cba0) com o formato de 0xad4e2c (o nome e 18 inteiros: o dia,
/// cinco W, cinco P e sete H); com os 19 lidos, o período (0x13 bytes) recebe o dia (palavra em +0) e os 17 bytes
/// seguintes (+2..+0x12). Devolve 1 se leu (só o al).
pub(crate) unsafe extern "thiscall" fn read_region_period(_this: usize, f: usize, period: usize) -> u8 {
    let read: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    if f == 0 {
        show_error_at(0xad4dec, 0xea);
        return 0;
    }
    crate::comparison::branch("cpt_5 weather period");
    let mut row = [0u8; 2000];
    if read_config_line(f, row.as_mut_ptr()) == 0 {
        return 0;
    }
    // simplification: the name is 200 bytes in the original; here 2000 so the %s does not overflow (with the names in the file, the same).
    let mut name = [0u8; 2000];
    let mut v = [0i32; 18];
    let p = v.as_mut_ptr();
    let n = read(row.as_ptr(), 0xad4e2c, name.as_mut_ptr(), p, p.add(1), p.add(2), p.add(3), p.add(4), p.add(5),
                p.add(6), p.add(7), p.add(8), p.add(9), p.add(10), p.add(11), p.add(12), p.add(13), p.add(14),
                p.add(15), p.add(16), p.add(17));
    if n != 0x13 {
        return 0;
    }
    core::ptr::write_unaligned(period as *mut u16, v[0] as u16);
    for k in 1..18usize {
        *((period + 1 + k) as *mut u8) = v[k] as u8;
    }
    1
}

// simplification: reads from the file: original only; the copy takes the branch.

/// Reads CM3_DATA\weather.cfg: the region count, the regions (`read_region`) and the city table, with the three readings and
/// the wind drawn at random; a file or memory error stops the game.
///
/// `Weather::load_config()`, on game creation (0x9226c0, Setup.cpp). Opens CM3_DATA\weather.cfg (`exe::utils_build_file_path`, fopen
/// `exe::crt_fopen` "r"); the first line (`exe::utils_read_config_line`, buffer of 1000) gives how many regions ("%ld" into +8); the regions (malloc
/// of 0x6e bytes each at +0) and the city table (malloc of 4 bytes per city [0xae2354] at +4); each region through
/// `read_region` (the first that fails stops the game; the others only end the reading); each city draws the three readings
/// ((random(100) + random(100)) × 0.5) and the wind (random(3)). Closes the file (`exe::crt_fclose`). The errors (file, line,
/// count, memory) give the "Error" box of weather.cpp and the game stops.
/// Original: `Weather::load_config` (0x922810, weather.cpp, thiscall).
///
/// [pt] { 0x922810 } (thiscall, weather.cpp): `Weather::load_config()`, na criação do jogo (0x9226c0, Setup.cpp). Abre o
/// CM3_DATA\weather.cfg (0x90bde0, fopen 0x945b00 "r"); a primeira linha (0x90cba0, buffer de 1000) dá quantas regiões
/// ("%ld" em +8); as regiões (malloc de 0x6e bytes cada em +0) e a tabela das cidades (malloc de 4 bytes por cidade
/// [0xae2354] em +4); cada região pela 0x922c40 (a primeira que falha para o jogo; as outras só encerram a leitura);
/// cada cidade sorteia as três medidas ((sorteio(100) + sorteio(100)) × 0,5) e o vento (sorteio(3)). Fecha o arquivo
/// (0x945a09). Os erros (arquivo, linha, contagem, memória) são a caixa "Error" de weather.cpp e o jogo para.
pub(crate) unsafe extern "thiscall" fn load_config(this: usize) {
    let read: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let f = exe::crt_fopen(exe::utils_build_file_path(0x9b5348, 0xad4e18), 0x9899fc);
    if f == 0 {
        fatal_error_at(0xad4dec, 0x68);
    }
    let mut row = [0u8; 1000];
    if exe::utils_read_config_line(f, row.as_mut_ptr()) == 0 {
        fatal_error_at(0xad4dec, 0x6f);
    }
    if read(row.as_ptr(), 0x98c11c, this + 8) != 1 {
        fatal_error_at(0xad4dec, 0x75);
    }
    let r = exe::crt_malloc((read_usize(this + 8).wrapping_mul(0x6e)) as u32);
    *(this as *mut usize) = r;
    if r == 0 {
        fatal_error_at(0xad4dec, 0x7e);
    }
    let c = exe::crt_malloc((read_usize(G_CITY_COUNT).wrapping_mul(4)) as u32);
    *((this + 4) as *mut usize) = c;
    if c == 0 {
        fatal_error_at(0xad4dec, 0x86);
    }
    let mut i = 0i32;
    while i < read_i32(this + 8) {
        if exe::weather_read_region(this, f, read_usize(this) + i as usize * 0x6e) == 0 {
            if i == 0 {
                fatal_error_at(0xad4dec, 0x92);
            }
            break;
        }
        i += 1;
    }
    let mut k = 0usize;
    while (k as i32) < read_i32(G_CITY_COUNT) {
        crate::comparison::branch("cpt_7 initial weather");
        for j in 0..3usize {
            let a = friendly_random(100);
            let b = friendly_random(100); // cpt7: the second random draw of the measure
            *((read_usize(this + 4) + k * 4 + j) as *mut u8) = ftol(a.wrapping_add(b) as f64 * read_f64(rdata::F64_0_5)) as u8;
        }
        let v = friendly_random(3); // cpt7: o vento inicial
        *((read_usize(this + 4) + k * 4 + 3) as *mut u8) = v as u8;
        k += 1;
    }
    exe::crt_fclose(f);
}

