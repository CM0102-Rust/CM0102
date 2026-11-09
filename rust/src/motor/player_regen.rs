use super::*;

/// Calls a one-argument thiscall routine by address (the person clean-up routines).
///
/// [pt] Uma chamada thiscall de um argumento pelo endereço (as rotinas de limpeza da pessoa).
pub(crate) unsafe fn call_thiscall_1(routine: usize, this: usize, a: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(routine);
    f(this, a);
}

/// Pointer to byte k of the person's 79-byte state record ([0xae2d0c] + 0x4f x id).
///
/// [pt] O byte `k` do registro de 79 bytes da pessoa ([0xae2d0c] + 0x4f × id).
pub(crate) unsafe fn staff_state_byte(p: usize, k: usize) -> *mut u8 {
    (read_usize(G_STAFF_STATE_TABLE) as isize + read_i32(p) as isize * 0x4f + k as isize) as *mut u8
}

/// Integer times an .rdata double constant, truncated (_ftol) to 16 bits.
///
/// [pt] O inteiro × a constante double do .rdata, truncado (fild; fmul qword; _ftol), em 16 bits.
pub(crate) unsafe fn mul_rdata_const(v: i16, k: usize) -> i16 {
    ftol(v as f64 * read_f64(k)) as i16
}

/// Sets current ability (record +5) from PA (+7): trunc(random(trunc(PA x m1)) + PA x m2).
///
/// [pt] A habilidade (ficha +5) pelo sorteio: r = sorteio(trunc(PA × m1)); trunc(r + PA × m2) (fild r; fild PA; fmul m2;
/// faddp; _ftol), a PA na ficha +7.
pub(crate) unsafe fn roll_current_ability(f: usize, m1: usize, m2: usize) {
    let pa = read_i16(f + player_sheet::POTENTIAL_ABILITY) as f64;
    let r = friendly_random(ftol(pa * read_f64(m1)));
    crate::comparison::branch("cqg_7 ability");
    write_i16(f + player_sheet::CURRENT_ABILITY, ftol(r as f64 + pa * read_f64(m2)) as i16); // cqg7: the ability drawn by the PA
}

/// Adjusts a 1..20 personality byte: above 15 drops random(cai); below a1 rises random(s1); below a2 rises random(s2).
///
/// [pt] O acerto de um atributo de personalidade de 1..20: acima de 15 cai sorteio(`cai`); abaixo de `a1` sobe sorteio(`s1`);
/// abaixo de `a2` sobe sorteio(`s2`) (0 = nada).
pub(crate) unsafe fn adjust_personality_attribute(a: usize, cai: i32, a1: i8, s1: i32, a2: i8, s2: i32) {
    let v = read_i8(a);
    if v > 0xf {
        let r = friendly_random(cai);
        write_i8(a, read_i8(a).wrapping_sub(r as i8));
    } else if v < a1 {
        let r = friendly_random(s1);
        write_i8(a, read_i8(a).wrapping_add(r as i8));
    } else if s2 != 0 && v < a2 {
        let r = friendly_random(s2);
        write_i8(a, read_i8(a).wrapping_add(r as i8));
    }
}

/// If the byte is below lim, adds random(n).
///
/// [pt] Abaixo de `lim`: soma sorteio(n).
pub(crate) unsafe fn add_random_if_below(a: usize, lim: i8, n: i32) {
    if read_i8(a) < lim {
        let r = friendly_random(n);
        write_i8(a, read_i8(a).wrapping_add(r as i8));
    }
}

/// Subtracts random(n) from the byte.
///
/// [pt] Tira sorteio(n).
pub(crate) unsafe fn subtract_random(a: usize, n: i32) {
    let r = friendly_random(n);
    write_i8(a, read_i8(a).wrapping_sub(r as i8));
}

/// If current ability (+5) is above teto and random(n) != 0, sets it to teto.
///
/// [pt] O teto da habilidade: acima de `teto` e sorteio(n) diferente de 0, o teto.
pub(crate) unsafe fn cap_current_ability(f: usize, ceiling: i16, n: i32) {
    if read_i16(f + player_sheet::CURRENT_ABILITY) > ceiling && friendly_random(n) != 0 {
        write_i16(f + player_sheet::CURRENT_ABILITY, ceiling);
    }
}

/// Rebuilds a retired person from the regen queue as a young player: player record, nation, names, languages, age and
/// ability, reputations, birth date, personality attributes and, by chance, a club.
///
/// `PLAYER_REGEN::regenerate_as_youth_player(person)`, from the regen queue (`process_regen_queue`) for every young regen. A person
/// not retired (bits 0x30 of +0xb of the 79-byte record) or with a club: it only clears the bits. Otherwise: leaves the
/// national team (`national_teams_remove_player`), takes the front record of the player record pool (this+0x3c, 16-byte entries: record,
/// nation, second nation; `remove_player_data_from_pool`), becomes a player (+0x5f = 2, +0x61 the record) and the 79-byte record gets the
/// regen mark (0x40) and loses the others; the person's lists and histories are cleared (fog, statistics, history,
/// suspensions, injuries, dispute, discipline, fitness, contracts); the record's nation (or, if its reputation is below
/// 100, with 1/4, any one with reputation below 100), the second nation (Spain with 1/100 keeps the previous one;
/// another one other than the Basque Country with 2/3 none), the nation's names, the languages (all removed; the
/// nation's by weight, level (weight − (20 − +0x56) × 10) / 75 up to 10), the joining date today, +0x3d/+0x28 = 11. The
/// age and ability: a small nation (reputation below 350, +0x11c zero) by the reputation (23..27 or 17..23 years) and
/// by the PA; otherwise by the record's position and the age by random(10000) (15..18 rare, 16..18, 17..20, 21..26).
/// The reputations (+9, +0xb, +0xd), the ability caps, the birth year and day, the age (`database_get_staff_age`), the personality
/// attributes (+0x1b, +0x36, +0x1d, +0x2e, +0x34, +0x44, +0x3c, +0x3d, +0x1e, +0x24, +0x2c, +0x2f, +0x21) by age; under
/// 19 and a nation with +0x11c, with 3/4, the best club (`pick_best_club_for_player`) and the contract (`player_regen_sign_for_club`, news `player_regen_post_youth_promotion_news` at a human
/// club and the assistant manager may like him); otherwise, by chance, a club of the nation (`pick_club_from_nation_list`). Without a club
/// and under 20, the reputations up to 1250 and +0xd = 1. At the end, the attributes normalised (`staff_normalise_player_physical_attributes`, `staff_normalise_player_attributes`).
/// Original: `PLAYER_REGEN::regenerate_as_youth_player` (0x7abac0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7abac0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::regenerate_as_youth_player(pessoa)`, pela fila dos
/// regens (0x7a65d0) em todo regen jovem. Pessoa não aposentada (bits 0x30 do +0xb do registro de 79 bytes) ou com
/// clube: só limpa os bits. Senão: sai da seleção (0x775ba0), pega a ficha da frente do banco de fichas (this+0x3c,
/// entradas de 16 bytes: ficha, nação, segunda nação; 0x7a8470), vira jogador (+0x5f = 2, +0x61 a ficha) e o registro
/// de 79 bytes ganha a marca do regen (0x40) e perde as outras; as listas e históricos da pessoa são limpos (névoa,
/// estatísticas, histórico, suspensões, lesões, disputa, disciplina, físico, contratos); a nação da ficha (ou, se ela
/// tem reputação abaixo de 100, com 1/4, uma qualquer de reputação abaixo de 100), a segunda nação (a Espanha com 1/100
/// mantém a anterior; outra que não o País Basco com 2/3 nenhuma), os nomes da nação, os idiomas (todos tirados; os da
/// nação pelo peso, nível (peso − (20 − +0x56) × 10) / 75 até 10), a data de entrada hoje, +0x3d/+0x28 = 11. A idade e
/// a habilidade: nação pequena (reputação abaixo de 350, +0x11c zero) pela reputação (23..27 ou 17..23 anos) e pela PA;
/// senão pela posição da ficha e a idade por sorteio(10000) (15..18 raros, 16..18, 17..20, 21..26). As reputações (+9,
/// +0xb, +0xd), os tetos da habilidade, o ano e o dia do nascimento, a idade (0x53ab90), os atributos de personalidade
/// (+0x1b, +0x36, +0x1d, +0x2e, +0x34, +0x44, +0x3c, +0x3d, +0x1e, +0x24, +0x2c, +0x2f, +0x21) pela idade; abaixo de 19
/// anos e nação de +0x11c, com 3/4, o melhor clube (0x7ad0e0) e o contrato (0x7a6f10, notícia 0x7a8ba0 no clube humano e
/// o auxiliar pode gostar dele); senão, com chance, um clube da nação (0x7ad590). Sem clube e abaixo de 20 anos, as
/// reputações até 1250 e +0xd = 1. No fim, os atributos normalizados (0x538ec0, 0x538bc0).
pub(crate) unsafe extern "thiscall" fn regenerate_as_youth_player(this: usize, p: usize) {
    let mark = staff_state_byte(p, staff_state::STATUS_FLAGS);
    let b = *mark;
    if b & 0x30 == 0 || read_usize(p + staff::CLUB_JOB) != 0 {
        *mark = b & 0xcf;
        return;
    }
    call_thiscall_1(0x775ba0, read_usize(G_NATIONAL_TEAMS), p);
    let bench = read_usize(this + player_regen_fields::PLAYER_DATA_POOL);
    let (ficha_id, nacao_id, nacao2_id) = (read_i32(bench), read_i32(bench + regen_pool_item::NATION_ID), read_i32(bench + regen_pool_item::SECOND_NATION_ID));
    exe::player_regen_remove_player_data_from_pool(this, ficha_id);
    *((p + staff::CLASSIFICATION) as *mut u8) = 2;
    core::ptr::write_unaligned((p + staff::PLAYER) as *mut usize, read_usize(G_PLAYER_DATA_TABLE).wrapping_add((ficha_id as isize * 0x46) as usize));
    *staff_state_byte(p, staff_state::STATUS_FLAGS) |= 0x40;
    *staff_state_byte(p, 0xd) = 0;
    *staff_state_byte(p, 0xc) &= 0xfd;
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0x7f;
    *staff_state_byte(p, 0xc) &= 0xfe;
    *staff_state_byte(p, 9) = 0x9c;
    if read_usize(G_FOG_OF_WAR) != 0 && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0 {
        call_thiscall_1(0x5afd00, read_usize(G_FOG_OF_WAR), p);
    }
    call_thiscall_1(0x7c6650, read_usize(G_PLAYER_STATS), p);
    call_thiscall_1(0x86fb80, read_usize(G_STAFF_RECORDS), p);
    call_thiscall_1(0x55e1d0, read_usize(G_DISCIPLINE), p);
    call_thiscall_1(0x631f90, read_usize(G_INJURY), p);
    call_thiscall_1(0x56a090, read_usize(G_DISPUTE), p);
    call_thiscall_1(0x556830, read_usize(G_DISCIPLINE), read_usize(p));
    call_thiscall_1(0x62bfa0, read_usize(G_INJURY), read_usize(p));
    call_thiscall_1(0x4d0b50, read_usize(G_CONTRACT_MANAGER), p);
    call_thiscall_1(0x4d0bd0, read_usize(G_CONTRACT_MANAGER), p);
    *((p + staff::INT_APPS) as *mut u8) = 0;
    *((p + staff::INT_GOALS) as *mut u8) = 0;
    if read_usize(p + staff::NON_PLAYER) != 0 {
        core::ptr::write_unaligned((p + staff::NON_PLAYER) as *mut u32, 0);
    }
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xf0;
    *staff_state_byte(p, 8) = 0;
    *staff_state_byte(p, 9) = 0x9c;
    let nations = read_usize(G_NATION_TABLE);
    let nation = nations.wrapping_add((nacao_id as isize * 0x122) as usize);
    core::ptr::write_unaligned((p + staff::NATION) as *mut usize, nation);
    if read_i32(nation + nation::NUMBER_STAFF) < 100 && friendly_random(4) == 0 {
        let k = friendly_random(read_i32(G_NATION_COUNT));
        let other = read_usize(G_NATION_TABLE).wrapping_add((k as isize * 0x122) as usize);
        if read_usize(other + nation::CONTINENT) != 0 && read_i32(other + nation::NUMBER_STAFF) < 100 {
            core::ptr::write_unaligned((p + staff::NATION) as *mut usize, other);
        }
    }
    if nacao2_id > 0 && nacao2_id < read_i32(G_NATION_COUNT) {
        if !(nacao_id == read_i32(KEY_NATION_SPAIN) && friendly_random(100) == 0) {
            if nacao2_id != read_i32(KEY_NATION_PAYS_BASQUE) && friendly_random(3) != 0 {
                core::ptr::write_unaligned((p + staff::SECOND_NATION) as *mut u32, 0);
            } else {
                core::ptr::write_unaligned((p + staff::SECOND_NATION) as *mut usize, read_usize(G_NATION_TABLE).wrapping_add((nacao2_id as isize * 0x122) as usize));
            }
        }
    } else {
        core::ptr::write_unaligned((p + staff::SECOND_NATION) as *mut u32, 0);
    }
    let v = exe::database_pick_first_name_for_nation(read_usize(p + staff::NATION));
    core::ptr::write_unaligned((p + staff::FIRST_NAME) as *mut usize, v);
    let v = exe::database_pick_second_name_for_nation(read_usize(p + staff::NATION));
    core::ptr::write_unaligned((p + staff::SECOND_NAME) as *mut usize, v);
    let v = exe::database_get_empty_common_name(read_usize(p + staff::NATION));
    core::ptr::write_unaligned((p + staff::COMMON_NAME) as *mut usize, v);
    call_thiscall_1(0x5fff30, read_usize(G_HUMAN_MANAGER), p);
    write_i16(p + 0x40, read_i16(G_CURRENT_DATE_YEAR));
    write_i16(p + staff::DATE_JOINED_CLUB, read_i16(G_CURRENT_DATE));
    core::ptr::write_unaligned((p + staff::CLUB_JOB) as *mut u32, 0);
    core::ptr::write_unaligned((p + staff::NATIONAL_JOB) as *mut u32, 0);
    core::ptr::write_unaligned((p + staff::WAGE) as *mut u32, 0);
    core::ptr::write_unaligned((p + staff::VALUE) as *mut u32, 0);
    // the languages: the original pushes the argument’s dword with the low byte replaced (the calls read the byte)
    for k in 1..=0x40i8 {
        exe::playerstats_remove_language(read_usize(G_PLAYER_STATS), p, k);
    }
    let base = (0x14i16.wrapping_sub(read_i8(p + staff::ADAPTABILITY) as i16)).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = exe::player_stats_get_native_language_points(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            // simplification: the fluency goes in a stack dword with only the low byte written (0x7c6a30 reads the byte)
            let level = if q > 0xa { 0xa } else { q as u8 };
            exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, k as u8 as u32, level as u32, 1);
        }
    }
    *((p + staff::JOB_FOR_CLUB) as *mut u8) = 0xb;
    *((p + staff::JOB_FOR_NATION) as *mut u8) = 0xb;
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xcf;
    let f = read_usize(p + staff::PLAYER);
    let nation = read_usize(p + staff::NATION);
    let age: i8;
    if nation != 0 && *((nation + nation::LEAGUE_SELECTED) as *const u8) == 0 && read_i32(nation + nation::NUMBER_STAFF) < 0x15e {
        // the small nation: age by its reputation and ability by PA
        let r1 = friendly_random(read_i32(nation + nation::NUMBER_STAFF));
        let r2 = friendly_random(1000);
        age = if r1 < r2 {
            (friendly_random(5) as i8).wrapping_add(0x17)
        } else {
            (friendly_random(7) as i8).wrapping_add(0x11)
        };
        crate::comparison::branch("cqg_7 small nation");
        if age > 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if age > 0x1a {
            roll_current_ability(f, 0x968918, 0x968e48);
        } else if age > 0x17 {
            roll_current_ability(f, 0x968fb0, 0x96a688);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        let r = friendly_random(0x2ee);
        write_i16(f + player_sheet::HOME_REPUTATION, (r as i16).wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(0x1e)).wrapping_add(0xfa));
        let mut l121 = age <= 0x12;
        if !l121 && read_i16(f + player_sheet::POTENTIAL_ABILITY) > 0x8c && read_i16(f + player_sheet::CURRENT_ABILITY) > 0x64 && read_i16(f + player_sheet::HOME_REPUTATION) < 0x1964 && age < 0x16 && friendly_random(4) != 0 {
            write_i16(f + player_sheet::HOME_REPUTATION, mul_rdata_const(read_i16(f + player_sheet::HOME_REPUTATION), 0x968978));
        }
        if age <= 0x12 {
            l121 = true;
        }
        if l121 {
            let a = read_i16(f + player_sheet::CURRENT_ABILITY);
            let r = friendly_random(0xc8) as i16;
            let v = if a < 0x8c {
                r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(5)).wrapping_add(1)
            } else {
                r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_add(0x19).wrapping_mul(10))
            };
            write_i16(f + player_sheet::WORLD_REPUTATION, v);
        } else {
            let a = read_i16(f + player_sheet::CURRENT_ABILITY);
            if a < 0x78 {
                let r = friendly_random(0xc8) as i16;
                write_i16(f + player_sheet::WORLD_REPUTATION, r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(0xf)).wrapping_add(1));
            } else if age > 0x15 && a > 0x78 {
                let r = friendly_random(0xc8) as i16;
                write_i16(f + player_sheet::WORLD_REPUTATION, r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_add(0xa).wrapping_mul(0x19)));
            } else {
                let r = friendly_random(0xc8) as i16;
                let sum = if a < 0x8c { 1 } else { 0xfa };
                write_i16(f + player_sheet::WORLD_REPUTATION, r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(0x14)).wrapping_add(sum));
            }
        }
        if read_i16(f + player_sheet::HOME_REPUTATION) < 0x8ca {
            let r = friendly_random(5);
            write_i16(f + player_sheet::WORLD_REPUTATION, (r as i16).wrapping_add(1));
        }
        if read_i16(f + player_sheet::HOME_REPUTATION) < read_i16(f + player_sheet::WORLD_REPUTATION) {
            write_i16(f + player_sheet::HOME_REPUTATION, read_i16(f + player_sheet::WORLD_REPUTATION).wrapping_add(0xa));
            write_i16(f + player_sheet::WORLD_REPUTATION, 1);
        }
        let r = friendly_random(0x32);
        write_i16(f + player_sheet::CURRENT_REPUTATION, (r as i16).wrapping_add(read_i16(f + player_sheet::HOME_REPUTATION)));
        if age > 0x17 {
            if read_i16(f + player_sheet::HOME_REPUTATION) < 0x1770 {
                write_i16(f + player_sheet::HOME_REPUTATION, mul_rdata_const(read_i16(f + player_sheet::HOME_REPUTATION), 0x969588));
            }
            if read_i16(f + player_sheet::CURRENT_REPUTATION) < 0x1770 {
                write_i16(f + player_sheet::CURRENT_REPUTATION, mul_rdata_const(read_i16(f + player_sheet::CURRENT_REPUTATION), 0x969588));
            }
            let a = read_i16(f + player_sheet::CURRENT_ABILITY);
            if a < mul_rdata_const(a, 0x96a5d0) && friendly_random(4) != 0 {
                write_i16(f + player_sheet::CURRENT_ABILITY, mul_rdata_const(read_i16(f + player_sheet::CURRENT_ABILITY), 0x96a5d0));
            }
        } else if age < 0x14 {
            if read_i16(f + player_sheet::CURRENT_ABILITY) < mul_rdata_const(read_i16(f + player_sheet::POTENTIAL_ABILITY), 0x968968) && friendly_random(4) != 0 {
                write_i16(f + player_sheet::CURRENT_ABILITY, mul_rdata_const(read_i16(f + player_sheet::POTENTIAL_ABILITY), 0x968968));
            }
            if age < 0x12 {
                cap_current_ability(f, 0x78, 0x96);
            } else {
                cap_current_ability(f, 0x78, 0x32);
            }
        } else {
            if read_i16(f + player_sheet::HOME_REPUTATION) < 0x1676 {
                write_i16(f + player_sheet::HOME_REPUTATION, mul_rdata_const(read_i16(f + player_sheet::HOME_REPUTATION), 0x968928));
            }
            if read_i16(f + player_sheet::CURRENT_REPUTATION) < 0x1676 {
                write_i16(f + player_sheet::CURRENT_REPUTATION, mul_rdata_const(read_i16(f + player_sheet::CURRENT_REPUTATION), 0x968928));
            }
            if read_i16(f + player_sheet::CURRENT_ABILITY) < mul_rdata_const(read_i16(f + player_sheet::POTENTIAL_ABILITY), 0x968e48) && friendly_random(4) != 0 {
                write_i16(f + player_sheet::CURRENT_ABILITY, mul_rdata_const(read_i16(f + player_sheet::POTENTIAL_ABILITY), 0x968e48));
            }
            cap_current_ability(f, 0x8c, 0x32);
        }
    } else {
        // the big nation or no nation: the ability by position and the age by random roll(10000)
        if *((f + player_sheet::ATTACKER) as *const u8) == 0x14 {
            roll_current_ability(f, 0x968e48, 0x968e80);
        } else if *((f + player_sheet::DEFENDER) as *const u8) == 0x14 || *((f + player_sheet::SWEEPER) as *const u8) == 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if *((f + player_sheet::GOALKEEPER) as *const u8) == 0x14 {
            roll_current_ability(f, 0x968968, 0x9689e0);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        if friendly_random(0xfa) != 0 {
            let r = friendly_random(0x2ee) as i16;
            write_i16(f + player_sheet::HOME_REPUTATION, r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(0x14)).wrapping_add(0xfa)); // cqg7: the +9 reputation of the regen
            crate::comparison::branch("cqg_7 regen injury_word_0c");
            let r = friendly_random(0x14) as i16;
            write_i16(f + player_sheet::WORLD_REPUTATION, r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(5)).wrapping_add(1));
        } else {
            let r = friendly_random(0x2ee) as i16;
            write_i16(f + player_sheet::HOME_REPUTATION, r.wrapping_add(read_i16(f + player_sheet::CURRENT_ABILITY).wrapping_mul(0xf)).wrapping_add(0x7d));
            write_i16(f + player_sheet::WORLD_REPUTATION, 1);
        }
        if read_i16(f + player_sheet::HOME_REPUTATION) < read_i16(f + player_sheet::WORLD_REPUTATION) {
            write_i16(f + player_sheet::HOME_REPUTATION, read_i16(f + player_sheet::WORLD_REPUTATION).wrapping_add(0xa));
            write_i16(f + player_sheet::WORLD_REPUTATION, 1);
        }
        let r = friendly_random(0x32);
        write_i16(f + player_sheet::CURRENT_REPUTATION, (r as i16).wrapping_add(read_i16(f + player_sheet::HOME_REPUTATION)));
        let s = friendly_random(0x2710);
        crate::comparison::branch("cqg_7 big nation");
        if s == 0 {
            age = (friendly_random(4) as i8).wrapping_add(0xf);
            if age == 0xf {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s < 0x1388 {
            age = (friendly_random(3) as i8).wrapping_add(0x10); // cqg7: the age from 16 to 18
            crate::comparison::branch("cqg_7 age 16 to 18");
            if age < 0x12 {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s > 0x26f7 {
            age = (friendly_random(6) as i8).wrapping_add(0x15);
        } else {
            age = (friendly_random(4) as i8).wrapping_add(0x11);
            if age == 0x11 {
                cap_current_ability(f, 0x5a, 0xfa);
            } else {
                cap_current_ability(f, 0x6e, 0x96);
            }
        }
    }
    if exe::nation_is_major_nation(read_usize(p + staff::NATION)) == 0 {
        if age < 0x19 && read_i16(f + player_sheet::CURRENT_ABILITY) < 0x78 {
            write_i16(f + player_sheet::WORLD_REPUTATION, 1);
        } else {
            write_i16(f + player_sheet::WORLD_REPUTATION, mul_rdata_const(read_i16(f + player_sheet::WORLD_REPUTATION), 0x96a5d0));
        }
    }
    write_i16(p + 0x12, read_i16(G_CURRENT_DATE_YEAR).wrapping_sub(age as i16)); // cqg7: the birth year
    let r = friendly_random(0x16d);
    write_i16(p + staff::DATE_OF_BIRTH, r as i16);
    let a = exe::database_get_staff_age(p, G_CURRENT_DATE as *const u16);
    write_i16(p + staff::AGE, a as u8 as i8 as i16);
    // +0x1b
    if read_i16(f + player_sheet::POTENTIAL_ABILITY) > 0x64 && (read_i8(f + player_sheet::ATTACKER) >= 0xf || read_i8(f + player_sheet::ATTACKING_MIDFIELDER) >= 0xf) {
        if friendly_random(3) != 0 {
            let r = friendly_random(0xb);
            write_i8(f + player_sheet::ACCELERATION, (r as i8).wrapping_add(0xa));
        } else if friendly_random(3) == 0 {
            let r = friendly_random(7);
            write_i8(f + player_sheet::ACCELERATION, (r as i8).wrapping_add(0xe));
        } else {
            let r = friendly_random(0x10);
            write_i8(f + player_sheet::ACCELERATION, (r as i8).wrapping_add(5));
        }
    } else if friendly_random(4) != 0 {
        let r = friendly_random(0xa);
        write_i8(f + player_sheet::ACCELERATION, (r as i8).wrapping_add(8));
    } else {
        let r = friendly_random(0x10);
        write_i8(f + player_sheet::ACCELERATION, (r as i8).wrapping_add(5));
    }
    adjust_personality_attribute(f + player_sheet::ACCELERATION, 5, 5, 0xa, 0xa, 8);
    // +0x36 by +0x1b
    let a = read_i8(f + player_sheet::ACCELERATION);
    let v = if a > 0xf {
        (friendly_random(0xb) as i8).wrapping_add(0xa)
    } else if a > 0xa {
        (friendly_random(0xa) as i8).wrapping_add(8)
    } else if a < 5 {
        (friendly_random(0xa) as i8).wrapping_add(1)
    } else if friendly_random(4) != 0 {
        (friendly_random(0x10) as i8).wrapping_add(5)
    } else {
        (friendly_random(0xa) as i8).wrapping_add(5)
    };
    write_i8(f + player_sheet::PLAYER_PACE, v);
    // +0x1d (the 20-rated goalkeeper with 4/5 by PA)
    if read_i8(f + player_sheet::GOALKEEPER) == 0x14 && friendly_random(5) != 0 {
        let r = friendly_random(0x14);
        let v = if read_i16(f + player_sheet::POTENTIAL_ABILITY) as i32 > r + 0x64 {
            (friendly_random(0xb) as i8).wrapping_add(0xa)
        } else {
            (friendly_random(0x10) as i8).wrapping_add(5)
        };
        write_i8(f + player_sheet::AGILITY, v);
    } else {
        let r = friendly_random(0x14);
        write_i8(f + player_sheet::AGILITY, (r as i8).wrapping_add(1));
        adjust_personality_attribute(f + player_sheet::AGILITY, 6, 5, 6, 0, 0);
    }
    // +0x2e
    let mut k: i8 = -5;
    if read_i8(f + player_sheet::DEFENDER) >= 0xf {
        k = if read_i8(f + player_sheet::CENTRAL) >= 0xf { 5 } else { 0 };
    } else if ((read_i8(f + player_sheet::MIDFIELDER) >= 0xf || read_i8(f + player_sheet::DEFENSIVE_MIDFIELDER) >= 0xf) && read_i8(f + player_sheet::CENTRAL) >= 0xf)
        || read_i8(f + player_sheet::GOALKEEPER) >= 0xf
        || read_i8(f + player_sheet::ATTACKER) >= 0xf
    {
        k = 0;
    }
    let r1 = friendly_random(k as i32 * 3);
    let r2 = friendly_random(0x14);
    write_i8(f + player_sheet::JUMPING, (r1 as i8).wrapping_add(r2 as i8));
    let a = read_i8(f + player_sheet::JUMPING);
    write_i8(f + player_sheet::JUMPING, if a < 1 { 1 } else if a > 0x14 { 0x14 } else { a });
    // +0x34
    let r = friendly_random(0x14);
    write_i8(f + player_sheet::NATURAL_FITNESS, (r as i8).wrapping_add(1));
    adjust_personality_attribute(f + player_sheet::NATURAL_FITNESS, 6, 8, 0xa, 0, 0);
    // +0x44
    let r = friendly_random(0x14);
    write_i8(f + player_sheet::WORK_RATE, (r as i8).wrapping_add(1));
    if read_i8(f + player_sheet::WORK_RATE) > 0xf && friendly_random(3) != 0 {
        subtract_random(f + player_sheet::WORK_RATE, 5);
    } else {
        let a = read_i8(f + player_sheet::WORK_RATE);
        if a < 5 {
            add_random_if_below(f + player_sheet::WORK_RATE, 5, 0xa);
        } else if a < 0xa {
            add_random_if_below(f + player_sheet::WORK_RATE, 0xa, 5);
        }
    }
    // +0x3c by age
    if age < 0x12 {
        let r = friendly_random(0x14);
        write_i8(f + player_sheet::STAMINA, (r as i8).wrapping_add(1));
        adjust_personality_attribute(f + player_sheet::STAMINA, 8, 5, 8, 0xa, 6);
    } else if age < 0x18 {
        let r = friendly_random(0x12);
        write_i8(f + player_sheet::STAMINA, (r as i8).wrapping_add(3));
        adjust_personality_attribute(f + player_sheet::STAMINA, 5, 5, 0xa, 0xa, 5);
    } else {
        let r = friendly_random(0xf);
        write_i8(f + player_sheet::STAMINA, (r as i8).wrapping_add(6));
        adjust_personality_attribute(f + player_sheet::STAMINA, 3, 0xa, 8, 0, 0);
    }
    if read_i8(f + player_sheet::STAMINA) < 0xa && read_i8(f + player_sheet::WORK_RATE) > 0xf {
        let r = friendly_random(5);
        write_i8(f + player_sheet::STAMINA, read_i8(f + player_sheet::STAMINA).wrapping_add(r as i8));
    }
    // +0x3d by age
    let r = friendly_random(0x14);
    write_i8(f + player_sheet::STRENGTH, (r as i8).wrapping_add(1));
    if age < 0x12 {
        adjust_personality_attribute(f + player_sheet::STRENGTH, 0xa, 5, 6, 0, 0);
    } else if age < 0x18 {
        adjust_personality_attribute(f + player_sheet::STRENGTH, 8, 5, 8, 0, 0);
    } else {
        adjust_personality_attribute(f + player_sheet::STRENGTH, 5, 5, 0xa, 0, 0);
    }
    if (read_i8(f + player_sheet::DEFENDER) == 0x14 || read_i8(f + player_sheet::DEFENSIVE_MIDFIELDER) == 0x14) && read_i8(f + player_sheet::STRENGTH) < 0xa {
        let r = friendly_random(5);
        write_i8(f + player_sheet::STRENGTH, read_i8(f + player_sheet::STRENGTH).wrapping_add(r as i8));
    }
    if read_i8(f + player_sheet::JUMPING) > 0xf && read_i8(f + player_sheet::STRENGTH) < 0xa {
        let r = friendly_random(5);
        write_i8(f + player_sheet::STRENGTH, read_i8(f + player_sheet::STRENGTH).wrapping_add(r as i8));
    }
    // +0x1e, +0x24, +0x2c, +0x2f (and +0x21) by age
    if age < 0x18 {
        if age < 0x12 {
            subtract_random(f + player_sheet::ANTICIPATION, 0x28);
            subtract_random(f + player_sheet::DECISIONS, 0x28);
            let a = read_i8(f + player_sheet::IMPORTANT_MATCHES);
            if a > 0xf {
                subtract_random(f + player_sheet::IMPORTANT_MATCHES, 8);
            } else if a > 0xa {
                subtract_random(f + player_sheet::IMPORTANT_MATCHES, 5);
            }
            let a = read_i8(f + player_sheet::LEADERSHIP);
            if a > 0xf {
                subtract_random(f + player_sheet::LEADERSHIP, 0xa);
            } else if a > 0xa {
                subtract_random(f + player_sheet::LEADERSHIP, 8);
            }
        } else {
            subtract_random(f + player_sheet::ANTICIPATION, 0x14);
            subtract_random(f + player_sheet::DECISIONS, 0x14);
            if read_i8(f + player_sheet::IMPORTANT_MATCHES) > 0xf {
                subtract_random(f + player_sheet::IMPORTANT_MATCHES, 5);
            }
            let a = read_i8(f + player_sheet::LEADERSHIP);
            if a > 0xf {
                subtract_random(f + player_sheet::LEADERSHIP, 8);
            } else if a > 0xa {
                subtract_random(f + player_sheet::LEADERSHIP, 5);
            }
        }
        if read_i8(f + player_sheet::LEADERSHIP) > 0xf {
            subtract_random(f + player_sheet::LEADERSHIP, 8);
        }
    } else if age > 0x19 && age < 0x1d {
        add_random_if_below(f + player_sheet::STRENGTH, 5, 8);
        add_random_if_below(f + player_sheet::CONSISTENCY, 5, 8);
        add_random_if_below(f + player_sheet::STAMINA, 5, 8);
        subtract_random(f + player_sheet::ANTICIPATION, 0xa);
        subtract_random(f + player_sheet::DECISIONS, 0xa);
        add_random_if_below(f + player_sheet::IMPORTANT_MATCHES, 5, 8);
        let a = read_i8(f + player_sheet::LEADERSHIP);
        if a > 0xf {
            subtract_random(f + player_sheet::LEADERSHIP, 5);
        } else if a > 0xa {
            subtract_random(f + player_sheet::LEADERSHIP, 3);
        }
    }
    // the club
    let nation = read_usize(p + staff::NATION);
    if age < 0x13 && nation != 0 && *((nation + nation::LEAGUE_SELECTED) as *const u8) != 0 && friendly_random(4) != 0 {
        let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_SIGN_FOR_CLUB);
        let c = exe::player_regen_pick_best_club_for_player(this, p);
        if c != 0 && contratar(p, c) != 0 {
            crate::comparison::branch("cqg_7 best club");
            let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
            if human(c) != 0 {
                exe::player_regen_post_youth_promotion_news(p, c);
            }
            if read_i16(read_usize(p + staff::PLAYER) + 7) > 0x50 {
                let t = read_usize(c + club::CHAIRMAN);
                if t != 0 && read_usize(t + staff::NON_PLAYER) != 0 {
                    let r = friendly_random(100);
                    if read_i8(read_usize(t + staff::NON_PLAYER) + 0x16) as i32 > r {
                        exe::staff_add_favoured_staff(t, p, 0);
                    }
                }
            }
        }
    } else if (nation != 0 && *((nation + nation::LEAGUE_SELECTED) as *const u8) == 0 && friendly_random(0xf) != 0) || friendly_random(0xa) == 0 {
        let c = exe::player_regen_pick_club_from_nation_list(this, p);
        if c != 0 {
            let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_SIGN_FOR_CLUB);
            contratar(p, c);
        }
    }
    if read_usize(p + staff::CLUB_JOB) == 0 && age < 0x14 {
        let f = read_usize(p + staff::PLAYER);
        if read_i16(f + player_sheet::CURRENT_REPUTATION) > 0x4e2 {
            write_i16(f + player_sheet::CURRENT_REPUTATION, 0x4e2);
        }
        if read_i16(f + player_sheet::HOME_REPUTATION) > 0x4e2 {
            write_i16(f + player_sheet::HOME_REPUTATION, 0x4e2);
        }
        write_i16(f + player_sheet::WORLD_REPUTATION, 1);
    }
    exe::staff_normalise_player_physical_attributes(p);
    exe::staff_normalise_player_attributes(p);
}

/// Does the person's name k (+4 first, +8 second, +0xc common) match text (up to 26 chars)? Empty text always matches.
///
/// [pt] O nome `k` (+4 primeiro, +8 segundo, +0xc o comum; o registro do nome começa pelo texto) da pessoa bate com `texto`
/// (até 26 caracteres)? `texto` vazio sempre bate.
pub(crate) unsafe fn name_matches(p: usize, k: usize, text: usize) -> bool {
    *(text as *const u8) == 0 || strncmp(read_usize(p + k), text, 0x1a) == 0
}

/// Finds the person of a player_setup.cfg line by the three names: among all persons or, with the club name, in its
/// squad (when asked) and its manager.
///
/// `player_setup_find_staff(first, common, second, club, in_squad)` (cdecl). Without `club` (null or empty): the first
/// of the [0xae2364] persons ([0xae23bc], 0x6e bytes) whose non-empty names match (each name checked only if both the
/// text and the person's name exist). With `club`: the club with that name (`club_get_english_name`, 51 characters); not found, 0.
/// With `in_squad` 1: the first of the 50 squad slots (+0xd7) whose names match; otherwise (or none in the squad), the
/// club's manager (+0xcf) if the names match; otherwise 0.
/// Original: `player_setup_find_staff` (0x7a50f0, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a50f0 } (cdecl, player_regen.cpp): `player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco)`. Sem
/// `clube` (nulo ou vazio): a primeira das [0xae2364] pessoas ([0xae23bc], 0x6e bytes) cujos nomes não vazios batem
/// (cada nome conferido só se o texto e o nome da pessoa existem). Com `clube`: o clube com esse nome (0x545880, 51
/// caracteres); sem achar, 0. Com `no_elenco` 1: o primeiro das 50 vagas do elenco (+0xd7) com os nomes que batem;
/// senão (ou nenhum do elenco), o técnico do clube (+0xcf) se os nomes batem; senão 0.
pub(crate) unsafe extern "C" fn player_setup_find_staff(first: usize, common: usize, second: usize, club: usize, in_squad: u32) -> usize {
    if club == 0 || *(club as *const u8) == 0 {
        let mut p = read_usize(G_STAFF_TABLE);
        for _ in 0..read_i32(G_STAFF_COUNT).max(0) {
            let falha = (*(first as *const u8) != 0 && read_usize(p + staff::FIRST_NAME) != 0 && strncmp(read_usize(p + staff::FIRST_NAME), first, 0x1a) != 0)
                || (*(second as *const u8) != 0 && read_usize(p + staff::SECOND_NAME) != 0 && strncmp(read_usize(p + staff::SECOND_NAME), second, 0x1a) != 0) // cqh9: the second name
                || (*(common as *const u8) != 0 && read_usize(p + staff::COMMON_NAME) != 0 && strncmp(read_usize(p + staff::COMMON_NAME), common, 0x1a) != 0);
            if !falha {
                crate::comparison::branch("cqh_9 person by name");
                return p;
            }
            p += 0x6e;
        }
        return 0;
    }
    let mut found = 0usize;
    for k in 0..read_i32(G_CLUB_COUNT).max(0) {
        let c = read_usize(G_CLUB_TABLE).wrapping_add(k as usize * club::SIZE);
        if strncmp(exe::club_get_english_name(c), club, 0x33) == 0 {
            found = c;
            break;
        }
    }
    if found == 0 {
        return 0;
    }
    let c = found;
    if in_squad as u8 == 1 {
        for k in 0..0x32usize {
            let q = read_usize(c + club::SQUAD + 4 * k);
            if q != 0 && name_matches(q, 4, first) && name_matches(q, 8, second) && name_matches(q, 0xc, common) {
                crate::comparison::branch("cqh_9 club player by name");
                return q; // cqh9: the squad one by name
            }
        }
    }
    let t = read_usize(c + club::MANAGER);
    if t == 0 {
        return 0;
    }
    if name_matches(t, 4, first) && name_matches(t, 8, second) && name_matches(t, 0xc, common) {
        return t;
    }
    0
}

/// Copies the next quoted text of the line (skipping spaces and line breaks before it) and returns where to continue.
///
/// `player_setup_read_quoted_token(source, dest)` (cdecl): copies into `dest` the text between the first quotes of
/// `source`: spaces and '\r' at the start of the text and '
///
/// ' before it are skipped; it ends at the closing quote or at the zero (the destination gets the zero) and returns the
/// pointer two past the end.
/// Original: `player_setup_read_quoted_token` (0x7a5090, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a5090 } (cdecl, player_regen.cpp): `player_setup_read_quoted_token(origem, destino)`: copia para `destino` o
/// texto entre as primeiras aspas de `origem`: espaços e '\r' no começo do texto e '\n' antes dele são pulados; termina
/// nas aspas de fecho ou no zero (o destino ganha o zero) e devolve o ponteiro dois depois do fim.
pub(crate) unsafe extern "C" fn player_setup_read_quoted_token(source: usize, target: usize) -> usize {
    let (mut i, mut n, mut aberto) = (0usize, 0usize, false);
    loop {
        let c = *((source + i) as *const u8);
        i += 1;
        if c == b'"' {
            if aberto {
                break;
            }
            aberto = true;
            continue;
        }
        if (c == b' ' || c == b'\r') && n == 0 {
            continue;
        }
        if c == b'\n' {
            if n == 0 {
                continue;
            }
        } else if c == 0 {
            break;
        }
        if aberto {
            crate::comparison::branch("cqh_9 quoted text");
            *((target + n) as *mut u8) = c; // cqh9: the text character
            n += 1;
        }
    }
    *((target + n) as *mut u8) = 0;
    source + i + 1
}

/// Os tipos de linha do player_setup.cfg (o texto no .rdata e o bit), na ordem da original (strncmp de 100); o último,
/// AWOL (0xa6b2f8), dá 0x40, e nenhum, 0.
pub(crate) const PLAYER_SETUP_LINE_TYPES: [(usize, u32); 14] = [
    (0xa6b3a0, 1),      // INJURY
    (0xa6b390, 2),      // MANAGER_CONF
    (0xa6b384, 0x200),  // RETIREMENT
    (0xa6b374, 4),      // INT_RETIREMENT
    (0xa6b364, 8),      // FUTURE_TRANSFER
    (0xa6b35c, 0x10),   // LOAN
    (0xa6b350, 0x100),  // NANDROLONE
    (0xa6b348, 0x100),  // DRUGS
    (0xa6b33c, 0x1000), // MISCONDUCT
    (0xa6b32c, 0x800),  // FALSE_PASSPORT
    (0xa6b324, 0x20),   // ARSE
    (0xa6b314, 0x2000), // PRESET_TACTICS
    (0xa6b308, 0x400),  // COLOURBLIND
    (0xa6b300, 0x80),   // GERMAN
];

/// Returns the type bit of a player_setup.cfg line.
///
/// `player_setup_type_from_name(name)` (cdecl): the bit of the line type (INJURY 1, MANAGER_CONF 2, RETIREMENT 0x200,
/// INT_RETIREMENT 4, FUTURE_TRANSFER 8, LOAN 0x10, NANDROLONE and DRUGS 0x100, MISCONDUCT 0x1000, FALSE_PASSPORT 0x800,
/// ARSE 0x20, PRESET_TACTICS 0x2000, COLOURBLIND 0x400, GERMAN 0x80, AWOL 0x40; none 0), used when applying the lines
/// (0x7a3a10) on game creation.
/// Original: `player_setup_type_from_name` (0x7a3140, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a3140 } (cdecl, player_regen.cpp): `player_setup_type_from_name(nome)`: o bit do tipo da linha (INJURY 1,
/// MANAGER_CONF 2, RETIREMENT 0x200, INT_RETIREMENT 4, FUTURE_TRANSFER 8, LOAN 0x10, NANDROLONE e DRUGS 0x100,
/// MISCONDUCT 0x1000, FALSE_PASSPORT 0x800, ARSE 0x20, PRESET_TACTICS 0x2000, COLOURBLIND 0x400, GERMAN 0x80, AWOL 0x40;
/// nenhum 0), pela aplicação das linhas (0x7a3a10) na criação do jogo.
pub(crate) unsafe extern "C" fn player_setup_type_from_name(name: usize) -> u32 {
    for &(texto, bit) in PLAYER_SETUP_LINE_TYPES.iter() {
        if strncmp(name, texto, 100) == 0 {
            crate::comparison::branch("cqh_9 line type");
            return bit;
        }
    }
    if strncmp(name, 0xa6b2f8, 100) == 0 { 0x40 } else { 0 }
}

/// Pointer to the retirement flags byte (+0xb) of person id's 79-byte record in [0xae2d0c].
///
/// [pt] O byte das marcas da aposentadoria da pessoa `id` (registros de 79 bytes em [0xae2d0c], +0xb).
pub(crate) unsafe fn retirement_flags_byte(id: i32) -> *mut u8 {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as isize * 0x4f + 0xb) as usize) as *mut u8
}

/// The person's contract (`exe::contract_manager_get_contract`) without storing it in [0xae1080].
///
/// [pt] O contrato da pessoa (0x4cf5a0 no gerente [0xae1050]), sem guardar em [0xae1080] (a 0x7a59e0 não guarda).
pub(crate) unsafe fn contract_of_uncached(p: usize) -> usize {
    exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p)
}

/// Calls PLAYER_REGEN::remove_retirement (`remove_retirement`): removes the pending retirement (later ones shift up).
///
/// [pt] PLAYER_REGEN::remove_retirement (0x7a8690): tira a aposentadoria pendente (as seguintes sobem um lugar).
pub(crate) unsafe fn player_regen_remove_retirement(this: usize, item: usize) {
    exe::player_regen_remove_retirement(this, item);
}

/// Calls a thiscall(manager [g], person) routine.
///
/// [pt] Uma rotina thiscall(gerente [g], pessoa).
pub(crate) unsafe fn call_on_global_manager(routine: usize, g: usize, p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(routine);
    f(read_usize(g), p);
}

/// Coach attribute from retired player: min(max(P, X) x k, cap), truncated.
///
/// [pt] O atributo do técnico que vira o jogador aposentado: min(max(P, X) × k, teto), truncado (as contas x87 da original
/// em f64; P e X inteiros).
pub(crate) fn coach_attribute_capped(pv: f64, x: f64, k: f64, ceiling: f64) -> u16 {
    let m = if pv > x { pv } else { x };
    let v = m * k;
    ftol(if v < ceiling { v } else { ceiling }) as u16
}

/// Sets the three coach attributes (+0xa, +8, +0xc of non-player record nj) from the player's caps squared vs his
/// record fields, scaled and capped.
///
/// [pt] Os três atributos do técnico (+0xa, +8, +0xc do registro de não jogador `nj`) pelo jogador `p` (ficha `f`): a
/// os jogos pela seleção (+0x22, byte) ao quadrado (pow 0x945640/0x945ff0) contra +0xb, +9 e +0xd da ficha (o último com o quadrado ×
/// 0,75), × `k` e até os tetos.
pub(crate) unsafe fn set_coach_attributes(p: usize, f: usize, nj: usize, k: f64, tetos: [f64; 3]) {
    let r = *((p + staff::INT_APPS) as *const u8) as f64;
    let q = game_pow(r, 2.0);
    core::ptr::write_unaligned((nj + non_player::CURRENT_REPUTATION) as *mut u16, coach_attribute_capped(q, read_i16(f + player_sheet::CURRENT_REPUTATION) as f64, k, tetos[0]));
    let q = game_pow(r, 2.0);
    core::ptr::write_unaligned((nj + non_player::HOME_REPUTATION) as *mut u16, coach_attribute_capped(q, read_i16(f + player_sheet::HOME_REPUTATION) as f64, k, tetos[1]));
    let q = game_pow(r, 2.0) * read_f64(rdata::F64_0_75);
    core::ptr::write_unaligned((nj + non_player::WORLD_REPUTATION) as *mut u16, coach_attribute_capped(q, read_i16(f + player_sheet::WORLD_REPUTATION) as f64, k, tetos[2]));
}

/// A retired player without a non-player record (+0x69) may become a coach by random rolls.
///
/// random(+0x5b) above random(100); random(ftol(player sheet's +0xb + reputation³)) above random(50000), or (the
/// nation's +0x8e, via +0x1a) × reputation above random(200000) + 200000; with more than 2 player sheets in reserve
/// (this +0x20): the first (this +0x1c; records of 0x44 bytes in [0xae23c0]) becomes the +0x69 and leaves the reserve
/// (`remove_spare_non_player_record`).
///
/// Goalkeeper (player sheet's +0xf 20): +0x10/+0x11 swapped when +0x10 is larger and, with random(4) zero, +0x11 =
/// random(50) + 50; the normalisation (`staff_normalise_non_player_attributes`) and the coach attributes (0x9/7500 with the player sheet's +0xb above
/// 7500; otherwise, with random(+0x57) ≠ 0, 0.75/6000/3750; otherwise 0.5/4500/1).
///
/// [pt] O jogador aposentado sem registro de não jogador (+0x69) pode virar técnico: sorteio(+0x5b) acima de sorteio(100);
/// sorteio(ftol(+0xb da ficha + reputação³)) acima de sorteio(50000), ou (+0x8e da nação +0x1a) × reputação acima de
/// sorteio(200000) + 200000; com mais de 2 fichas reservadas (this +0x20): a primeira (this +0x1c; registros de 0x44
/// bytes em [0xae23c0]) vira o +0x69 e sai da reserva (0x7a82b0); goleiro (+0xf da ficha 20): +0x10/+0x11 trocados
/// quando +0x10 é maior e, com sorteio(4) nulo, +0x11 = sorteio(50) + 50; a normalização (0x539490) e os atributos do
/// técnico (0x9/7500 com +0xb da ficha acima de 7500; senão, com sorteio(+0x57) ≠ 0, 0,75/6000/3750; senão
/// 0,5/4500/1).
pub(crate) unsafe fn maybe_become_coach(this: usize, p: usize) {
    let r = *((p + staff::INT_APPS) as *const u8);
    let ra = friendly_random(read_i8(p + staff::PROFESSIONALISM) as i32);
    if ra <= friendly_random(100) {
        return;
    }
    let f = read_usize(p + staff::PLAYER);
    let t = ftol(read_i16(f + player_sheet::CURRENT_REPUTATION) as f64 + game_pow(r as f64, 3.0));
    let r1 = friendly_random(t);
    let passes = r1 > friendly_random(50000) || { // cqd10: the random draw to become a manager
        let r3 = friendly_random(0x30d40).wrapping_add(0x30d40);
        let v = (read_i16(read_usize(p + staff::NATION) + 0x8e) as i32).wrapping_mul(*((p + staff::INT_APPS) as *const u8) as i32);
        v > r3
    };
    if !passes || read_i32(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) <= 2 {
        return;
    }
    crate::comparison::branch("cqd_10 becomes coach");
    let id = read_i32(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS));
    core::ptr::write_unaligned((p + staff::NON_PLAYER) as *mut u32, read_usize(G_NON_PLAYER_TABLE).wrapping_add((id as isize * 0x44) as usize) as u32);
    exe::player_regen_remove_spare_non_player_record(this, (read_i32(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS))) as i32 as u32);
    if read_i8(read_usize(p + staff::PLAYER) + 0xf) == 0x14 {
        let nj = read_usize(p + staff::NON_PLAYER);
        let (a, b) = (read_i8(nj + non_player::COACHING), read_i8(nj + non_player::COACHING_GKS));
        if a > b {
            *((nj + non_player::COACHING_GKS) as *mut i8) = a;
            *((read_usize(p + staff::NON_PLAYER) + 0x10) as *mut i8) = b;
            if friendly_random(4) == 0 {
                let v = (friendly_random(0x32) as u8).wrapping_add(0x32);
                *((read_usize(p + staff::NON_PLAYER) + 0x11) as *mut u8) = v;
            }
        }
    }
    exe::staff_normalise_non_player_attributes(p);
    let f = read_usize(p + staff::PLAYER);
    let nj = read_usize(p + staff::NON_PLAYER);
    if read_i16(f + player_sheet::CURRENT_REPUTATION) > 0x1d4c {
        set_coach_attributes(p, f, nj, read_f64(rdata::F64_0_9), [read_f64(rdata::F64_7500), read_f64(rdata::F64_7500), read_f64(rdata::F64_5750)]);
    } else if friendly_random(read_i8(p + staff::AMBITION) as i32) != 0 {
        set_coach_attributes(p, f, nj, read_f64(rdata::F64_0_75), [read_f64(rdata::F64_6000), read_f64(rdata::F64_6000), read_f64(rdata::F64_3750)]);
    } else {
        set_coach_attributes(p, f, nj, read_f64(rdata::F64_0_5), [read_f64(rdata::F64_4500), read_f64(rdata::F64_4500), read_f64(rdata::F64_1)]);
    }
}

/// Handles a due retirement item for person p: undoes it for young/contracted/transfer cases, otherwise posts news,
/// leaves jobs and frees the slot.
///
/// [pt] A aposentadoria que venceu (o item `item` da pessoa `p`).
pub(crate) unsafe fn process_due_retirement(this: usize, item: usize, p: usize, slot: &mut [u32; 3]) {
    let id = read_i32(item);
    if read_i8(p + staff::AGE) < 0x1e && read_i8(item + retirement_entry::REASON) != 8 // cqd10: the young player does not retire
        && (read_usize(p + staff::CLUB_JOB) != 0 || *retirement_flags_byte(read_i32(p)) & 0xf != 0) {
        crate::comparison::branch("cqd_10 retirement undone");
        player_regen_remove_retirement(this, item);
        return;
    }
    if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 || exe::transfer_manager_is_player_under_offer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        *retirement_flags_byte(id) &= 0xcf;
        player_regen_remove_retirement(this, item);
        return;
    }
    let old = read_i32(item + retirement_entry::CLUB_ID);
    let club = read_usize(p + staff::CLUB_JOB);
    let changed = if old != -1 { club != 0 && read_i32(club) != old } else { club != 0 };
    let mut fix = true;
    if changed {
        if exe::date_days_since(G_CURRENT_DATE, read_usize(p + staff::DATE_JOINED_CLUB) as u32, read_usize(p + 0x42) as u32) < 0xd2 && read_i8(p + staff::AGE) < 0x26 {
            fix = false;
            let c = contract_of_uncached(p);
            if c != 0 && exe::staff_is_transfer_listed_or_requested(p, 0) == 0 && read_usize(c + 0x45) == 0 {
                crate::comparison::branch("cqd_10 changed club and stays");
                *retirement_flags_byte(id) &= 0xcf;
                player_regen_remove_retirement(this, item);
                return;
            }
        }
    }
    if fix && read_usize(p + staff::CLUB_JOB) == 0 && read_i32(item + retirement_entry::CLUB_ID) != -1 {
        core::ptr::write_unaligned((item + retirement_entry::CLUB_ID) as *mut i32, -1);
    }
    // simplification: the original pushes the reason and the announced mark in registers with leftover high bytes; the
    // 0x7a92e0 reads the bytes
    exe::player_regen_post_retirement_news(p, *((item + retirement_entry::REASON) as *const u8) as u32, *((item + retirement_entry::ANNOUNCED) as *const u8) as u32);
    exe::contract_manager_remove_pending_offers_for_staff(read_usize(G_CONTRACT_MANAGER), p);
    if read_usize(p + staff::PLAYER) != 0 && read_usize(p + staff::NON_PLAYER) != 0 {
        if contract_of_uncached(p) != 0 {
            let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_BONUSES_AND_CLAUSES);
            clear(contract_of_uncached(p));
        }
    } else {
        crate::comparison::branch("cqd_10 removes the contract");
        call_on_global_manager(0x4d0b50, G_CONTRACT_MANAGER, p); // cqd10: removes the contract
    }
    call_on_global_manager(0x775ba0, G_NATIONAL_TEAMS, p);
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    if sm != 0 {
        let c = read_usize(p + staff::CLUB_JOB);
        if c != 0 {
            (exe::shortlist_manager_update_staff_status(sm, p, c, 4, 2) as u8);
        }
        exe::shortlist_manager_suspend_clubs_shortlisting_player(sm, p, 0);
        let c = read_usize(p + staff::CLUB_JOB);
        if c != 0 {
            *(read_usize(sm + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_i32(c) as isize * 9 + 8) as usize) as *mut u8) = 100;
        }
    }
    let mut bench = false;
    if read_usize(p + staff::PLAYER) != 0 {
        crate::comparison::branch("cqd_10 leaves the squad");
        exe::player_regen_retire_from_playing_staff(p); // cqd10: sai do elenco
        if read_usize(p + staff::CLUB_JOB) == 0 {
            call_on_global_manager(0x4d0b50, G_CONTRACT_MANAGER, p);
        }
        call_on_global_manager(0x4196a0, G_AWOL, p);
        call_on_global_manager(0x4197c0, G_AWOL, p);
        call_on_global_manager(0x56a090, G_DISPUTE, p);
        if read_usize(p + staff::PLAYER) != 0 {
            if read_usize(p + staff::NON_PLAYER) != 0 {
                bench = true;
            } else {
                maybe_become_coach(this, p);
            }
        }
    }
    if !bench {
        bench = read_usize(p + staff::NON_PLAYER) != 0 && read_usize(p + staff::PLAYER) != 0;
    }
    if bench {
        // the player with a non-player record: the record goes to the regen database and he stays a non-player
        crate::comparison::branch("cqd_10 player to the bench");
        call_on_global_manager(0x7c6650, G_PLAYER_STATS, p);
        call_on_global_manager(0x55e1d0, G_DISCIPLINE, p);
        call_on_global_manager(0x631f90, G_INJURY, p);
        let pais = read_usize(p + staff::NATION);
        if pais == 0 {
            let n = friendly_random(read_i32(G_NATION_COUNT));
            exe::player_regen_add_player_data_to_pool(this, read_i32(p), n as u32, u32::MAX);
        } else if read_usize(p + staff::SECOND_NATION) == 0 {
            exe::player_regen_add_player_data_to_pool(this, read_i32(p), read_usize(pais) as u32, u32::MAX);
        } else {
            exe::player_regen_add_player_data_to_pool(this, read_i32(p), read_usize(pais) as u32, read_usize(read_usize(p + staff::SECOND_NATION)) as u32);
        }
        core::ptr::write_unaligned((p + staff::PLAYER) as *mut u32, 0);
        *retirement_flags_byte(read_i32(item)) &= 0xcf;
        *((p + staff::CLASSIFICATION) as *mut u8) = 1;
        if read_usize(p + staff::CLUB_JOB) != 0 {
            let c = contract_of_uncached(p);
            if c != 0 {
                exe::staff_contract_generate_initial_contract(c, p, 0);
            }
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    // the slot for the regen: today (up to age 29) or 1 to 65 days later (past day 365, random roll(35) of the year
    // seguinte)
    slot[0] = read_i32(item) as u32;
    let today = read_usize(G_CURRENT_DATE) as u32;
    let (dia, ano) = if read_i8(p + staff::AGE) < 0x1e {
        (today as u16, read_i16(G_CURRENT_DATE_YEAR) as u16)
    } else {
        let v = (friendly_random(0x41) as u32).wrapping_add(today).wrapping_add(1) as u16; // cqd10: the regen day
        crate::comparison::branch("cqd_10 regen day drawn");
        if v as i16 <= 0x16d {
            (v, read_i16(G_CURRENT_DATE_YEAR) as u16)
        } else {
            (friendly_random(0x23) as u16, (read_i16(G_CURRENT_DATE_YEAR) as u16).wrapping_add(1))
        }
    };
    slot[1] = (slot[1] & 0xffff_0000) | dia as u32;
    slot[1] = (slot[1] & 0xffff) | (ano as u32) << 16;
    let m = retirement_flags_byte(read_i32(item));
    *m = (*m & 0xef) | 0x20;
    let nj = read_usize(p + staff::NON_PLAYER);
    if nj != 0 {
        let clear: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_CLEAR_MANAGER_RECORD);
        clear(read_usize(G_STAFF_RECORDS), nj);
        let c = read_usize(p + staff::CLUB_JOB);
        let job = |p: usize| read_i8(p + staff::JOB_FOR_CLUB);
        let exit = if c == 0 || job(p) == 5 {
            true
        } else {
            if exe::human_manager_is_human_club(c, 1, 1, 0, 0) != 0 && !matches!(job(p), 1 | 3 | 2 | 4) {
                true
            } else if friendly_random(4) != 0 {
                false
            } else {
                !matches!(job(p), 1 | 3 | 2 | 4 | 0xa | 9)
            }
        };
        if exit {
            exe::player_regen_retire_from_non_playing_staff(p);
            exe::player_regen_queue_staff_for_regen(this, slot.as_ptr());
        } else {
            crate::comparison::branch("cqd_10 redone as non-player");
            exe::player_regen_regenerate_as_non_player(this, p);
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    exe::player_regen_queue_staff_for_regen(this, slot.as_ptr());
    player_regen_remove_retirement(this, item);
}

/// Processes the day's pending retirements: drops some, retires the others (news, contract, national team, squad),
/// possibly turns the player into a coach, stores the record in the regen pool or queues the regen slot, and announces
/// the future ones.
///
/// `PLAYER_REGEN::process_retirements()`, every day from the day loop; SEH frame with no destructor on the normal path.
/// The local slot for the regen (id and the default date `exe::date_date_54a110`, which leaves leap flag 1 in it); it sorts the
/// pending retirements ([this+4], [this+8] of 26 bytes: +0 the person, +4 the club or −1, +8/+0xa the announcement day
/// and year, +0x10/+0x12 those of the retirement, +0x18 the reason, +0x19 announced) with `player_regen_compare_retirements` and walks through
/// them (the pointer only advances on an announcement: `remove_retirement` pulls the following ones). Due (the date has passed): a
/// young player (up to 29) with a club or flag, except reason 8, gives up; with a contract record (`contract_record`) or under
/// offer (`is_player_under_offer`), gives up (flags & 0xcf); one who moved club less than 210 days ago, up to 37 years old, with a
/// contract, off the list and without +0x45, stays; otherwise the news (`player_regen_post_retirement_news`), the offers (`remove_pending_offers_for_staff`), the contract,
/// the national team (`national_teams_remove_player`), the watch lists, the squad (`retire_from_playing_staff`), the absences (`clear_awol`, `clear_missed_training`) and the
/// dispute (`remove_dispute`); the player may become a coach (attributes from reputation and record); with a player record and
/// a non-player record, the record goes to the pool (`add_player_data_to_pool`) and the new contract (0x85e960); otherwise the regen
/// slot (`queue_staff_for_regen`) and, with a non-player record, he leaves the jobs (`player_regen_retire_from_non_playing_staff`) or is rebuilt as a non-player
/// (`exe::player_regen_regenerate_as_non_player`). Not due: with the announcement date passed and not announced, the announcement news (`player_regen_post_retirement_announcement_news`, if flags
/// & 0x30 are not 0x10), the announced flag and flags 0x10; otherwise it stops. At the end it sorts the regen slots
/// ([this+0x10], [this+0x14] of 12 bytes) with `player_regen_compare_queue_entries`.
/// Original: `PLAYER_REGEN::process_retirements` (0x7a59e0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a59e0 } (thiscall, ret 0, player_regen.cpp; quadro SEH sem destrutor no caminho normal):
/// `PLAYER_REGEN::process_retirements()`, todo dia pelo laço do dia. A vaga local para o regen (id e a data padrão
/// 0x54a110, que deixa o bissexto 1 nela); ordena as aposentadorias pendentes ([this+4], [this+8] de 26 bytes: +0 a
/// pessoa, +4 o clube ou −1, +8/+0xa o dia e o ano do anúncio, +0x10/+0x12 os da aposentadoria, +0x18 o motivo, +0x19
/// anunciada) com 0x7ae2a0 e anda por elas (o ponteiro só avança no anúncio: a 0x7a8690 puxa as seguintes). Vencida
/// (a data passou): o jovem (até 29) com clube ou marca, fora do motivo 8, desiste; com ficha de contrato (0x4cf6d0)
/// ou sob oferta (0x8cde70), desiste (marcas & 0xcf); quem mudou de clube há menos de 210 dias, até 37 anos, com
/// contrato, fora da lista e sem +0x45, fica; senão a notícia (0x7a92e0), as ofertas (0x4ccbf0), o contrato, a seleção
/// (0x775ba0), as listas de observação, o elenco (0x7aace0), as faltas (0x4196a0, 0x4197c0) e a disputa (0x56a090); o
/// jogador pode virar técnico; com ficha e registro de não jogador, a ficha vai para o banco (0x7a8350) e o contrato
/// novo (0x85e960); senão a vaga para o regen (0x7a87d0) e, com registro de não jogador, sai dos cargos (0x7aaeb0) ou
/// é refeito como não jogador (0x7ab340). Não vencida: com a data do anúncio passada e sem anúncio, a notícia do
/// anúncio (0x7aa340, se as marcas & 0x30 não forem 0x10), a marca de anunciada e as marcas 0x10; senão para. No fim
/// ordena as vagas para o regen ([this+0x10], [this+0x14] de 12 bytes) com 0x7ae240.
pub(crate) unsafe extern "thiscall" fn process_retirements(this: usize) {
    let mut slot = [0u32; 3]; // the person and the date (day, year, leap)
    exe::date_date_54a110(slot.as_mut_ptr() as usize + 4);
    exe::crt_qsort(read_usize(this + player_regen_fields::RETIREMENTS), read_usize(this + player_regen_fields::RETIREMENT_COUNT) as u32, 0x1a, 0x7ae2a0);
    let n = read_i32(this + player_regen_fields::RETIREMENT_COUNT);
    let mut item = read_usize(this + player_regen_fields::RETIREMENTS);
    let mut i = 0i32;
    while i < n {
        let today = read_usize(G_CURRENT_DATE) as u32;
        let (dia, ano) = (today as u16 as i16, (today >> 16) as u16);
        let (vdia, vano) = (read_i16(item + retirement_entry::RETIRE_DATE), read_i16(item + retirement_entry::RETIRE_YEAR) as u16);
        if ano > vano || (ano == vano && dia > vdia) {
            let p = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(item) as isize * 0x6e) as usize);
            process_due_retirement(this, item, p, &mut slot);
        } else {
            if *((item + retirement_entry::ANNOUNCED) as *const u8) != 0 {
                break;
            }
            let (adia, aano) = (read_i16(item + retirement_entry::ANNOUNCE_DATE), read_i16(item + retirement_entry::ANNOUNCE_YEAR) as u16);
            if !(ano > aano || (ano == aano && dia > adia)) {
                break;
            }
            crate::comparison::branch("cqd_10 retirement announced");
            let id = read_i32(item);
            if *retirement_flags_byte(id) & 0x30 != 0x10 {
                // simplification: the original pushes the reason in edx with leftover upper bytes; 0x7aa340 reads the byte
                exe::player_regen_post_retirement_announcement_news(read_usize(G_STAFF_TABLE).wrapping_add((id as isize * 0x6e) as usize), *((item + retirement_entry::REASON) as *const u8) as u32);
            }
            let id = read_i32(item);
            *((item + retirement_entry::ANNOUNCED) as *mut u8) = 1;
            let m = retirement_flags_byte(id);
            *m = (*m & 0xdf) | 0x10;
            item += 0x1a;
        }
        i += 1;
    }
    exe::crt_qsort(read_usize(this + player_regen_fields::REGEN_QUEUE), read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT) as u32, 0xc, 0x7ae240);
}

// simplification: calls the news (they allocate and deliver), the regen as a non-player, the new contracts and the lists: the path
// is only known by running and cannot run twice. Original only; the copy carries the branches and the career proves it.

/// Posts news item 0xbba about the retirement to the managers (all, the country, the division, the club or the
/// interested clubs, by fame) and pays the club insurance when injury retires a valuable player.
///
/// `player_regen_post_retirement_news(person, reason, announced)` (stdcall, ret 0xc; `process_retirements` puts this in ecx, unused;
/// SEH frame with no destructor on the normal path). No person, a player with job 8 or 5, no club, or a club without a
/// human manager (`news_club_has_human_manager`): nothing. News 0xbba (local): 0..3 the person (id and the +0x33 of +4/+8/+0xc), 4 the
/// reason, 5 announced, 6 the club (−1 if none), 7 the nation (+0x1a) with the national team caps (+0x22) or −1, 8 the
/// job (0xf becomes 8, 0xd becomes 6), 9 the reputation, 0xa +0x28, 0xb player record and non-player record, 0xc the
/// age, 0xd the non-player record, 0xe the player record. The interested clubs (`exe::transfer_manager_collect_player_news_recipients` on [0xde1f54] with 0xa, up to
/// 30, only for a player). A major nation (+0x85 above 8) and a star (+0xd of the player record above 8500, or a
/// manager of job 5 with +0xc above 8750): to everyone (`send_to_all_managers`); a lesser star (+0xd above 7500, or job 5 with +0xa
/// above 7250): to the managers of the nation and of the club's nation (`send_to_nation_managers`); a manager of job 5 at a club with a
/// division: to those of the division (`send_to_division_managers`); a non-player: to the club (`send_to_club_manager`); a player: to the interested
/// clubs. Reason 8 (injury) with +0xb of the player record above 4250 and a contract club (`exe::contract_manager_get_contract`, cached in
/// [0xae1080]): the insurance (`exe::news_post_insurance_payout` with +0x52).
/// Original: `player_regen_post_retirement_news` (0x7a92e0, player_regen.cpp, stdcall).
///
/// [pt] { 0x7a92e0 } (stdcall, ret 0xc, player_regen.cpp; a 0x7a59e0 põe o this no ecx, sem uso; quadro SEH sem destrutor
/// no caminho normal): `player_regen_post_retirement_news(pessoa, motivo, anunciada)`. Sem pessoa, jogador com cargo 8
/// ou 5, sem clube ou clube sem técnico humano (0x7875e0): nada. A notícia 0xbba (local): 0..3 a pessoa (id e os +0x33
/// de +4/+8/+0xc), 4 o motivo, 5 anunciada, 6 o clube (−1 sem), 7 a nação (+0x1a) com os jogos pela seleção (+0x22) ou −1, 8 o
/// cargo (0xf vira 8, 0xd vira 6), 9 a reputação, 0xa +0x28, 0xb ficha e registro de não jogador, 0xc a idade, 0xd o
/// registro de não jogador, 0xe a ficha. Os interessados (0x8d1da0 em [0xde1f54] com 0xa, até 30, só para jogador).
/// Nação de peso (+0x85 acima de 8) e estrela (+0xd da ficha acima de 8500, ou técnico de cargo 5 com +0xc acima de
/// 8750): a todos (0x786780); estrela menor (+0xd acima de 7500, ou cargo 5 com +0xa acima de 7250): aos técnicos da
/// nação e da do clube (0x7863f0); técnico de cargo 5 de clube com divisão: aos da divisão (0x7862d0); não jogador: ao
/// clube (0x7861e0); jogador: aos interessados. Motivo 8 com +0xb da ficha acima de 4250 e clube de contrato
/// (0x4cf5a0, guardado em [0xae1080]): o seguro (0x58a980 com +0x52).
pub(crate) unsafe extern "stdcall" fn player_regen_post_retirement_news(p: usize, reason: u32, anunciada: u32) {
    if p == 0 {
        return;
    }
    if read_usize(p + staff::PLAYER) != 0 && matches!(read_i8(p + staff::JOB_FOR_CLUB), 8 | 5) {
        return;
    }
    let club = read_usize(p + staff::CLUB_JOB);
    if club == 0 {
        return;
    }
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if human(club) == 0 {
        return;
    }
    crate::comparison::branch("cqd_10 retirement news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    exe::new_newsitem(nw, 0xbba, 0);
    exe::newsitem_set_slot(nw, 0, read_usize(p) as u32);
    for k in 1..4usize {
        exe::newsitem_set_slot(nw, k as i8, read_usize(read_usize(p + staff::FIRST_NAME * k) + 0x33) as u32);
    }
    exe::newsitem_set_slot(nw, 4, reason as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(nw, 5, anunciada as u8 as i8 as i32 as u32);
    let c = read_usize(p + staff::CLUB_JOB);
    exe::newsitem_set_slot(nw, 6, if c != 0 { read_usize(c) as u32 } else { u32::MAX });
    exe::newsitem_set_slot(nw, 7, if *((p + staff::INT_APPS) as *const u8) != 0 { read_usize(read_usize(p + staff::NATION)) as u32 } else { u32::MAX });
    let job = match read_i8(p + staff::JOB_FOR_CLUB) {
        0xf => 8,
        0xd => 6,
        v => v,
    };
    exe::newsitem_set_slot(nw, 8, job as i32 as u32);
    exe::newsitem_set_slot(nw, 9, *((p + staff::INT_APPS) as *const u8) as u32);
    exe::newsitem_set_slot(nw, 0xa, read_i8(p + staff::JOB_FOR_NATION) as i32 as u32);
    exe::newsitem_set_slot(nw, 0xb, (read_usize(p + staff::PLAYER) != 0 && read_usize(p + staff::NON_PLAYER) != 0) as u32);
    exe::newsitem_set_slot(nw, 0xc, read_i8(p + staff::AGE) as i32 as u32);
    exe::newsitem_set_slot(nw, 0xd, (read_usize(p + staff::NON_PLAYER) != 0) as u32);
    exe::newsitem_set_slot(nw, 0xe, (read_usize(p + staff::PLAYER) != 0) as u32);
    let mut clubs = [0usize; 30];
    let tm = read_usize(G_TRANSFER_MANAGER);
    let n = if tm != 0 && read_usize(p + staff::PLAYER) != 0 {
        exe::transfer_manager_collect_player_news_recipients(tm, p, 0xa, clubs.as_mut_ptr(), 0x1e) as i8
    } else {
        0
    };
    let mn = read_usize(G_NEWS);
    let pais = read_usize(p + staff::NATION);
    let f = read_usize(p + staff::PLAYER);
    let nj = read_usize(p + staff::NON_PLAYER);
    let cargo5 = || read_i8(p + staff::JOB_FOR_CLUB) == 5;
    if pais != 0 && read_i8(pais + nation::LEAGUE_STANDARD) > 8
        && ((f != 0 && read_i16(f + player_sheet::WORLD_REPUTATION) > 0x2134) || (nj != 0 && read_i16(nj + non_player::WORLD_REPUTATION) > 0x222e && cargo5())) {
        exe::news_send_to_all_managers(mn, nw);
    } else if (f != 0 && read_i16(f + player_sheet::WORLD_REPUTATION) > 0x1d4c) || (nj != 0 && read_i16(nj + non_player::CURRENT_REPUTATION) > 0x1c52 && cargo5()) {
        exe::news_send_to_nation_managers(mn, nw, pais);
        let c = read_usize(p + staff::CLUB_JOB);
        if c != 0 && read_usize(c + 0x53) != read_usize(p + staff::NATION) {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), nw, read_usize(c + 0x53));
        }
    } else {
        let c = read_usize(p + staff::CLUB_JOB);
        if c != 0 && read_usize(c + 0x57) != 0 && cargo5() {
            exe::news_send_to_division_managers(mn, nw, read_usize(c + 0x57));
        } else if f == 0 {
            exe::news_send_to_club_manager(mn, nw, c);
        } else if n > 0 {
            for k in 0..n as usize {
                exe::news_send_to_club_manager(read_usize(G_NEWS), nw, clubs[k]);
            }
        }
    }
    if reason as u8 == 8 {
        let f = read_usize(p + staff::PLAYER);
        if f != 0 && read_i16(f + player_sheet::CURRENT_REPUTATION) > 0x109a {
            let c = contract_of_uncached(p);
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            if c != 0 {
                let club = read_usize(G_CLUB_TABLE).wrapping_add((read_i32(c + 4) as isize * 0x245) as usize);
                if club != 0 {
                    crate::comparison::branch("cqd_10 retirement insurance");
                    exe::news_post_insurance_payout(club, p, read_usize(p + staff::VALUE) as u32);
                }
            }
        }
    }
}

// simplification: allocates and delivers the news item (0x77bbe0, 0x786780...). Original only; the copy carries the branch.

/// Posts news item 0xbce about a youth player joining the club, with the assistant manager's or a coach's verdict on
/// his ability.
///
/// `player_regen_post_youth_promotion_news(player, club)` (stdcall, ret 8; SEH frame with no destructor on the normal
/// path), from the young regen (`regenerate_as_youth_player`). No player or club: nothing. News 0xbce (local): 0..3 the player (id and the
/// +0x33 of +4/+8/+0xc), 4 the club id, 5 the age, 6 the club's strength (+0x7f) above 15. The assistant manager
/// (+0xd3) and the five coaches (+0x19f) give their opinion until the first one who gives a verdict: the perceived
/// reputation = random(2500) + club reputation (+0x80) / 2 + strength × 12.5 (truncated); the two perceived abilities
/// (`perceived_ability` with 2 and with 1) against reputation / 50 + 75 (verdict 2) and reputation / 100 + 50 (1). With a verdict:
/// 7 assistant?, 8..0xb who (id and names), 0xc and 0xd the verdicts; otherwise −1. Delivered at the club's scope
/// (`send_by_scope`).
/// Original: `player_regen_post_youth_promotion_news` (0x7a8ba0, player_regen.cpp, stdcall).
///
/// [pt] { 0x7a8ba0 } (stdcall, ret 8, player_regen.cpp; quadro SEH sem destrutor no caminho normal):
/// `player_regen_post_youth_promotion_news(jogador, clube)`, pelo regen jovem (0x7abac0). Sem jogador ou clube: nada.
/// A notícia 0xbce (local): 0..3 o jogador (id e os +0x33 de +4/+8/+0xc), 4 o id do clube, 5 a idade, 6 a força do
/// clube (+0x7f) acima de 15. O auxiliar (+0xd3) e os cinco treinadores (+0x19f) opinam até o primeiro que der
/// veredito: a reputação vista = sorteio(2500) + reputação do clube (+0x80) / 2 + força × 12,5 (truncada); as duas
/// habilidades vistas (0x6e86d0 com 2 e com 1) contra reputação / 50 + 75 (veredito 2) e reputação / 100 + 50 (1).
/// Com veredito: 7 auxiliar?, 8..0xb quem (id e nomes), 0xc e 0xd os vereditos; senão −1. Ao alcance do clube
/// (0x786090).
pub(crate) unsafe extern "stdcall" fn player_regen_post_youth_promotion_news(j: usize, c: usize) {
    if j == 0 || c == 0 {
        return;
    }
    crate::comparison::branch("cqd_10 promoted youngster news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    let (mut v1, mut v2) = (0i8, 0i8);
    let mut who = 0usize;
    exe::new_newsitem(nw, 0xbce, 0);
    exe::newsitem_set_slot(nw, 0, read_usize(j) as u32);
    for k in 1..4usize {
        exe::newsitem_set_slot(nw, k as i8, read_usize(read_usize(j + staff::FIRST_NAME * k) + 0x33) as u32);
    }
    exe::newsitem_set_slot(nw, 4, read_usize(c) as u32);
    exe::newsitem_set_slot(nw, 5, read_i8(j + staff::AGE) as i32 as u32);
    exe::newsitem_set_slot(nw, 6, (read_i8(c + club::TRAINING) > 0xf) as u32);
    let mut current = 0usize;
    let mut k = -1i8;
    loop {
        if current != 0 {
            break;
        }
        let t = if k == -1 { read_usize(c + club::ASSISTANT_MANAGER) } else { read_usize(c + club::COACHES + 4 * k as usize) };
        if t != 0 {
            let r = friendly_random(0x9c4);
            let v = r.wrapping_add(read_i16(c + club::REPUTATION) as i32 / 2);
            let x = ftol(v as f64 + read_i8(c + club::TRAINING) as f64 * read_f64(rdata::F64_12_5));
            // simplification: the original pushes the whole eax of _ftol and that of the first view; 0x6e86d0 reads the word
            let a = exe::match_man_judge_player_ability(t, j, c, 2, x as i16);
            let b = exe::match_man_judge_player_ability(t, j, c, 1, a);
            let rep = read_i16(c + club::REPUTATION) as i32;
            let high = rep / 50 + 0x4b;
            if a as i32 > high {
                v1 = 2;
                who = t;
            } else if a as i32 > rep / 100 + 0x32 {
                v1 = 1;
                who = t;
            }
            if b as i32 > high {
                v2 = 2;
                who = t;
            } else if b as i32 > rep / 100 + 0x32 {
                v2 = 1;
                who = t;
            }
            current = who;
        }
        k += 1;
        if k >= 5 {
            break;
        }
    }
    if current != 0 {
        exe::newsitem_set_slot(nw, 7, (current == read_usize(c + club::ASSISTANT_MANAGER)) as u32);
        exe::newsitem_set_slot(nw, 8, read_usize(current) as u32);
        for k in 1..4usize {
            exe::newsitem_set_slot(nw, 8 + k as i8, read_usize(read_usize(current + 4 * k) + 0x33) as u32);
        }
        exe::newsitem_set_slot(nw, 0xc, v1 as i32 as u32);
        exe::newsitem_set_slot(nw, 0xd, v2 as i32 as u32);
    } else {
        for k in 7..14i8 {
            exe::newsitem_set_slot(nw, k, u32::MAX);
        }
    }
    exe::news_send_by_scope(read_usize(G_NEWS), nw, c, 0, 0, 0, 0);
}

// simplification: allocates and delivers the news and rolls the opinion: original only; the copy takes the branch.

/// First free (zero) slot of the club's 50 squad slots (+0xd7); 0x32 if none.
///
/// [pt] A primeira vaga livre (dword 0) das 50 do elenco do clube (+0xd7); 0x32 sem vaga.
pub(crate) unsafe fn first_free_squad_slot(club: usize) -> usize {
    (0..0x32usize).find(|&k| read_i32(club + club::SQUAD + 4 * k) == 0).unwrap_or(0x32)
}

/// Signs the regen for a club: the first free squad slot, the value, the contract end, the contract and the default
/// wage; returns 1 (0 without a slot).
///
/// `player_regen_sign_for_club(person, club)` (stdcall, ret 8; SEH frame only for the local dates), from
/// PLAYER_REGEN::regenerate_as_youth_player (`regenerate_as_youth_player`) for every young regen: in the first free slot of the 50 in the
/// squad (+0xd7; no slot, 0) the person, her club (+0x39) and +0x4e = 0; the value (+0x52) through `calc_player_value` (club, 1,
/// −1, 0), the squad (`staff_set_squad_flags` with 1), one time in four (random(4) = 0) `exe::staff_add_favoured_club` (person, club, 0); a club with +0x64
/// = 3: value at most 5000. The contract end (+0x46): under 19 (+0x18), today's year + 2; otherwise + random(6) + 1; in
/// the month of the club's season end (`contract_manager_season_end_month`) and on day random(28) + 1 (`exe::date_date_549f70`). The contract (`add_staff_contract` with the
/// value); a human club (`exe::human_manager_is_human_club` with 1, 1): for the reserves (`move_to_reserves`) and the contract bits +0x4f (0x60 high) and
/// +0x35 (1 low). With a contract: the default wage (`set_default_wage`), +0x3a = 0 and, outside the human club, the end (+0x2f)
/// at most today's year + 5 (with day +0x2d = 0). At the end the squad numbers ([0xdd7ef0], `club_uses_squad_numbers` and `allocate_squad_number`).
/// Returns 1.
/// Original: `player_regen_sign_for_club` (0x7a6f10, player_regen.cpp, stdcall, ret 8).
///
/// [pt] { 0x7a6f10 } (stdcall, ret 8, player_regen.cpp, quadro SEH só das datas locais): `player_regen_sign_for_club(pessoa,
/// clube)`, pela PLAYER_REGEN::regenerate_as_youth_player (0x7abac0) em todo regen jovem: na primeira vaga livre das 50
/// do elenco (+0xd7; sem vaga, 0) a pessoa, o clube dela (+0x39) e +0x4e = 0; o valor (+0x52) pela 0x4d0c60 (clube, 1,
/// −1, 0), o elenco (0x540980 com 1), uma vez em quatro (exe::utils_random(4) = 0) a 0x547d90 (pessoa, clube, 0); clube com +0x64
/// = 3: valor até 5000. O fim do contrato (+0x46): abaixo de 19 anos (+0x18), o ano de hoje + 2; senão + exe::utils_random(6) +
/// 1; no mês do fim da temporada do clube (0x4d45f0) e no dia exe::utils_random(28) + 1 (0x549f70). O contrato (0x4c7950 com o
/// valor); clube humano (0x600a50 com 1, 1): para os reservas (0x4cc0f0) e os bits +0x4f (0x60 em cima) e +0x35 (1 em
/// baixo) do contrato. Com contrato: o salário padrão (0x8626e0), +0x3a = 0 e, fora do humano, o fim (+0x2f) até o ano de
/// hoje + 5 (com o dia +0x2d = 0). No fim os números do elenco ([0xdd7ef0], 0x85add0 e 0x85a470). Devolve 1.
pub(crate) unsafe extern "stdcall" fn player_regen_sign_for_club(person: usize, club: usize) -> u8 {
    let k = first_free_squad_slot(club);
    if k == 0x32 {
        return 0;
    }
    let manager = read_i32(G_CONTRACT_MANAGER) as usize;
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let squad: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let month_of_end: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SEASON_END_MONTH);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    crate::comparison::branch("cpv_9 regen signed");
    write_i32(club + club::SQUAD + 4 * k, person as i32);
    write_i32(person + staff::CLUB_JOB, club as i32);
    write_i32(person + staff::WAGE, 0);
    let v = value(manager, person, club, 1, -1, 0);
    write_i32(person + staff::VALUE, v); // cpv9: the regen value
    squad(person, 1);
    if exe::utils_random(4) == 0 { // cpv9: once in four
        crate::comparison::branch("cpv_9 one in four");
        exe::staff_add_favoured_club(person, club, 0);
    }
    if *((club + club::PROFESSIONAL_STATUS) as *const u8) == 3 && read_i32(person + staff::VALUE) > 0x1388 {
        write_i32(person + staff::VALUE, 0x1388);
    }
    // simplification: the original writes the year in the low word of the `clube` argument and pushes the dword; 0x549f70 reads 16 bits
    let year = *(G_CURRENT_DATE_YEAR as *const u16) as u32;
    let (mut d, mut ret) = ([0u8; 8], [0u8; 8]);
    let day_and_year = if *((person + staff::AGE) as *const i8) < 0x13 {
        crate::comparison::branch("cpv_9 youngster contract");
        let a = year.wrapping_add(2); // cpv9: o jovem por dois anos
        let month = month_of_end(club);
        (exe::utils_random(0x1c), month, a)
    } else {
        let a = (exe::utils_random(6) as u32).wrapping_add(year).wrapping_add(1);
        let month = month_of_end(club);
        (exe::utils_random(0x1c), month, a)
    };
    let day = (day_and_year.0 as u8).wrapping_add(1) as i8 as i16 as u16 as u32;
    let p = (exe::date_date_549f70((d.as_mut_ptr()) as *mut [u16; 4], day, day_and_year.1, day_and_year.2, 0xffff_ffff) as *mut u8);
    (exe::date_operator_3d(person + staff::DATE_EXPIRES_CLUB, (ret.as_mut_ptr()) as usize, (p) as usize) as *mut u8);
    exe::contract_manager_add_staff_contract(manager, person, read_i32(person + staff::VALUE) as u32);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        exe::contract_manager_move_to_reserves(read_i32(G_CONTRACT_MANAGER) as usize, person, club);
        let c = contract(read_i32(G_CONTRACT_MANAGER) as usize, person);
        *((c + contract::SQUAD_STATUS) as *mut u8) = *((c + contract::SQUAD_STATUS) as *const u8) & 0xf | 0x60;
        let c = contract(read_i32(G_CONTRACT_MANAGER) as usize, person);
        *((c + contract::CONTRACT_TYPE) as *mut u8) = *((c + contract::CONTRACT_TYPE) as *const u8) & 0xc1 | 1;
    }
    let c = contract(read_i32(G_CONTRACT_MANAGER) as usize, person);
    if c != 0 {
        exe::staff_contract_set_default_wage(c, person);
        *((c + 0x3a) as *mut u8) = 0;
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
            let a = (*(G_CURRENT_DATE_YEAR as *const u16)).wrapping_add(5);
            if core::ptr::read_unaligned((c + 0x2f) as *const u16) > a {
                core::ptr::write_unaligned((c + 0x2f) as *mut u16, a);
                core::ptr::write_unaligned((c + contract::CONTRACT_EXPIRES) as *mut u16, 0);
            }
        }
    }
    if exe::squad_manager_club_uses_squad_numbers(read_i32(G_SQUAD_MANAGER) as usize, club, (0) as u32) != 0 {
        exe::squad_manager_allocate_squad_number(read_i32(G_SQUAD_MANAGER) as usize, club, 0);
    }
    1
}

/// Replaces the person's pending retirement with a new one through injury (reason 8), random(days / 2) + 1 days ahead.
///
/// `PLAYER_REGEN::schedule_injury_retirement(person, days)`, from the new injury (`add_injury`) that ends the career; SEH
/// frame only for the local dates. It removes the person's pending retirement (the first with her id in the 26-byte
/// table [this+4], count +8; `remove_retirement`) and adds (`add_retirement`) the new one: id, club id (+0x39, or −1), today's date +
/// random(days / 2) + 1 (`exe::date_add_days`) at +8 and again at +0x10, reason 8 (+0x18) and 0 (+0x19).
/// Original: `PLAYER_REGEN::schedule_injury_retirement` (0x7a7f20, player_regen.cpp, thiscall, ret 8).
///
/// [pt] { 0x7a7f20 } (thiscall, ret 8, player_regen.cpp, quadro SEH só das datas locais):
/// `PLAYER_REGEN::schedule_injury_retirement(pessoa, dias)`, pela lesão nova (0x62d250) que encerra a carreira: tira a
/// aposentadoria pendente da pessoa (a primeira com o id dela na tabela de 26 bytes [this+4], conta +8; 0x7a8690) e
/// acrescenta (0x7a8510) a nova: id, id do clube (+0x39, ou −1), a data de hoje + exe::utils_random(dias / 2) + 1 (0x54c5b0) em
/// +8 e de novo em +0x10, o motivo 8 (+0x18) e 0 (+0x19).
pub(crate) unsafe extern "thiscall" fn schedule_injury_retirement(this: usize, person: usize, days: i16) {
    let n = read_i32(this + player_regen_fields::RETIREMENT_COUNT);
    if person != 0 && n > 0 {
        let base = read_i32(this + player_regen_fields::RETIREMENTS) as usize;
        let id = read_i32(person);
        if let Some(i) = (0..n as usize).find(|&i| read_i32(base + 0x1a * i) == id) {
            crate::comparison::branch("cpv_9 removes the pending retirement");
            exe::player_regen_remove_retirement(this, base + 0x1a * i);
        }
    }
    let mut r = [0u8; 0x1a];
    let club = read_i32(person + staff::CLUB_JOB) as usize;
    let rp = r.as_mut_ptr();
    core::ptr::write_unaligned(rp as *mut i32, read_i32(person));
    core::ptr::write_unaligned(rp.add(4) as *mut i32, if club == 0 { -1 } else { read_i32(club) });
    let q = exe::utils_random(days as i32 / 2);
    let (mut t, mut t2) = ([0u8; 8], [0u8; 8]);
    let d = (exe::date_add_days(G_CURRENT_DATE, (t.as_mut_ptr()) as usize, q.wrapping_add(1) as u32) as *mut u8);
    (exe::date_operator_3d((rp.add(8)) as usize, (t2.as_mut_ptr()) as usize, (d) as usize) as *mut u8);
    (exe::date_operator_3d((rp.add(0x10)) as usize, (t2.as_mut_ptr()) as usize, (rp.add(8)) as usize) as *mut u8);
    r[0x18] = 8;
    r[0x19] = 0;
    exe::player_regen_add_retirement(this, r.as_ptr());
}

/// Builds news 0xbbb (id, names, reason, contract club, age) and sends it to the managers of the interested clubs.
///
/// `player_regen_post_retirement_announcement_news(person, reason)` (stdcall, ret 8; SEH frame without destructor),
/// from PLAYER_REGEN::process_retirements (`process_retirements`): except for a player (+0x61) with job +0x3d 8 or 5, news 0xbbb in
/// a local (`construct_news_item` with 0) with data 0 = id, 1 to 3 = the names (+0x33 of +4, +8 and +0xc), 4 = reason, 5 = the id of
/// the contract club (`exe::contract_manager_get_contract`, cached in [0xae1080]; club of the 0x245-byte record in [0xae23b4] by +4; a contract of
/// type +0x35 & 0x3f = 5 or no club: −1) and 6 = the age (+0x18); it goes to the managers of up to 30 interested clubs
/// ([0xde1f54], `exe::transfer_manager_collect_player_news_recipients` with 10).
/// Original: `player_regen_post_retirement_announcement_news` (0x7aa340, player_regen.cpp, stdcall, ret 8).
///
/// [pt] { 0x7aa340 } (stdcall, ret 8, player_regen.cpp, quadro SEH sem destrutor): `player_regen_post_retirement_announcement_news(
/// pessoa, motivo)`, pela PLAYER_REGEN::process_retirements (0x7a59e0): fora do jogador (+0x61) com motivo +0x3d 8 ou
/// 5, a notícia 0xbbb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os nomes (+0x33 de +4, +8 e +0xc), 4 =
/// motivo, 5 = o id do clube do contrato (0x4cf5a0, guardado em [0xae1080]; clube do registro de 0x245 bytes em
/// [0xae23b4] pelo +4; contrato de tipo +0x35 & 0x3f = 5 ou sem clube: −1) e 6 = a idade (+0x18); vai para os técnicos
/// dos até 30 clubes interessados ([0xde1f54], 0x8d1da0 com 10).
pub(crate) unsafe extern "stdcall" fn player_regen_post_retirement_announcement_news(person: usize, reason: i8) {
    if person == 0 {
        return;
    }
    if read_i32(person + staff::PLAYER) != 0 && matches!(*((person + staff::JOB_FOR_CLUB) as *const u8), 8 | 5) {
        return;
    }
    crate::comparison::branch("player_regen_post_retirement_announcement_news");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbbb, 0);
    exe::newsitem_set_slot(n, 0, read_i32(person) as u32);
    for (k, o) in [(1i8, 4usize), (2, 8), (3, 0xc)] {
        exe::newsitem_set_slot(n, k, read_i32(read_i32(person + o) as usize + 0x33) as u32);
    }
    exe::newsitem_set_slot(n, 4, reason as i32 as u32);
    let c = exe::contract_manager_get_contract(read_i32(G_CONTRACT_MANAGER) as usize, person);
    write_i32(G_CONTRACT_LOOKUP_RESULT, c as i32);
    let mut club = -1i32;
    if c != 0 && *((c + contract::CONTRACT_TYPE) as *const u8) & 0x3f != 5 {
        let idx = read_i32(c + contract::CLUB) as u32;
        let record = (read_i32(G_CLUB_TABLE) as u32).wrapping_add(idx.wrapping_mul(0x245)) as usize;
        if record != 0 {
            club = read_i32(record);
        }
    }
    exe::newsitem_set_slot(n, 5, club as u32);
    exe::newsitem_set_slot(n, 6, *((person + staff::AGE) as *const i8) as i32 as u32);
    let mut list = [0usize; 0x1e];
    let q = if read_i32(G_TRANSFER_MANAGER) != 0 { exe::transfer_manager_collect_player_news_recipients(read_i32(G_TRANSFER_MANAGER) as usize, person, 0xa, list.as_mut_ptr(), 0x1e) } else { 0 };
    for i in 0..(q as i8).max(0) as usize {
        exe::news_send_to_club_manager(read_i32(G_NEWS) as usize, n, list[i]);
    }
}

/// Appends to the pool of free player records the player record id of a person who stops playing, with the nation and
/// second nation, growing the table by 250.
///
/// `PLAYER_REGEN::add_player_data_to_pool(person, nation, second)`: appends to the free player pool (16 bytes at
/// [this+0x3c], count +0x40, capacity +0x44) the player record id (+0x61) of the person with index `person` (0x6e-byte
/// record in [0xae23bc]) at +0, the nation at +4 and the second nation at +8 (+0xc is left as it is). Count = capacity
/// − 1: grows by 250 (realloc `exe::crt_realloc`; out of memory, box 0xa0a and end). From process_retirements (`process_retirements`), when a
/// player who also has a non-player record stops playing.
/// Original: `PLAYER_REGEN::add_player_data_to_pool` (0x7a8350, player_regen.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7a8350 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_player_data_to_pool(pessoa, nacao,
/// segunda)`: acrescenta ao banco de jogadores livres (16 bytes em [this+0x3c], conta +0x40, capacidade +0x44) o id da
/// ficha de jogador (+0x61) da pessoa de índice `pessoa` (registro de 0x6e bytes em [0xae23bc]) em +0, a nação em +4 e
/// a segunda nação em +8 (+0xc fica como está). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória,
/// caixa 0xa0a e fim). Pela process_retirements (0x7a59e0), quando um jogador que também tem ficha de não jogador
/// deixa de jogar.
pub(crate) unsafe extern "thiscall" fn add_player_data_to_pool(this: usize, person: i32, nation: u32, second: u32) {
    if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::PLAYER_DATA_POOL), read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_add(250) << 4);
        write_usize(this + player_regen_fields::PLAYER_DATA_POOL, p);
        write_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa0a);
        }
    }
    crate::comparison::branch("cpu_9 player bench");
    let record = read_usize(G_STAFF_TABLE).wrapping_add((person as u32).wrapping_mul(0x6e) as usize);
    let item = read_usize(this + player_regen_fields::PLAYER_DATA_POOL).wrapping_add(read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) << 4);
    write_usize(item, read_usize(read_usize(record + staff::PLAYER)));
    write_usize(item + regen_pool_item::NATION_ID, nation as usize);
    write_usize(item + regen_pool_item::SECOND_NATION_ID, second as usize);
    write_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT).wrapping_add(1));
}

/// whether the spare non-player record list holds any entry
/// Original: `PLAYER_REGEN::has_spare_non_player_record` (0x7a6e20, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a6e20 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::has_spare_non_player_record()`: se a reserva de
/// fichas de não jogador (conta em +0x20) tem alguma. O eax leva os 24 bits altos da conta (setne no al), como na
/// original.
pub(crate) unsafe extern "thiscall" fn has_spare_non_player_record(this: usize) -> u32 {
    let c = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) as u32;
    (c & 0xffff_ff00) | (c != 0) as u32
}

/// Looks up the person's contract (cached in [0xae1080]) and returns his contract with that club (`exe::contract_manager_get_contract_for_club`).
///
/// [pt] O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) e, pelo clube dele (registro de 0x245 bytes pelo +4; sem
/// contrato, 0), o contrato da pessoa com esse clube (0x4cf5f0), que a rotina devolve.
pub(crate) unsafe fn contract_with_current_club(person: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let club = if c != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE)) } else { 0 };
    exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club)
}

/// Status byte of the club contract (+0x3d); -100 without contract.
///
/// [pt] O estatuto do contrato com o clube (+0x3d); sem ele, −100 (0x9c).
pub(crate) unsafe fn contract_status(c2: usize) -> i8 {
    if c2 == 0 {
        0x9cu8 as i8
    } else {
        read_i8(c2 + 0x3d)
    }
}

/// The retirement age limit for non-players, by job type; reproduces the original's stray stack byte when there is no
/// job.
///
/// Job type +0x3d; without it +0x28; without both, as in the original routine, the low byte of the output pointer,
/// which it reads from the wrong stack slot.
///
/// 1, 2 and 4: 70 + random(10); 8 and 9: 60 + random(5); the others: 61 + random(5) with a club (+0x39) or national
/// team (+0x24), otherwise 55 + random(10). Byte.
///
/// [pt] O limite de idade de quem não é jogador, pelo tipo do cargo (+0x3d; sem ele +0x28; sem os dois, como a original,
/// o byte baixo do ponteiro da saída, que ela lê da vaga errada da pilha): 1, 2 e 4: 70 + sorteio(10); 8 e 9:
/// 60 + sorteio(5); os outros: 61 + sorteio(5) com clube (+0x39) ou seleção (+0x24), senão 55 + sorteio(10). Byte.
pub(crate) unsafe fn non_player_age_limit(person: usize, out: usize) -> i8 {
    let mut t = read_i8(person + staff::JOB_FOR_CLUB);
    if t == 0 {
        t = read_i8(person + staff::JOB_FOR_NATION);
        if t == 0 {
            t = out as u8 as i8; // simplification: the original reads the low byte of the `saida` argument here
        }
    }
    let (n, base) = match t {
        1 | 2 | 4 => (10, 0x46u8),
        8 | 9 => (5, 0x3c),
        _ => {
            if read_usize(person + staff::CLUB_JOB) != 0 || read_usize(person + staff::NATIONAL_JOB) != 0 {
                (5, 0x3d)
            } else {
                (10, 0x37)
            }
        }
    };
    (random(n) as u8).wrapping_add(base) as i8
}

/// Player retirement age limit from his record: 34, 30 or 31 by attribute bytes +0xf/+0x15/+0x14.
///
/// [pt] O limite de idade do jogador pela ficha: 34 com +0xf ≥ 15; senão 30 com +0x15 ≥ 15; senão 31 com +0x14 < 15;
/// senão 30 (bytes com sinal).
pub(crate) unsafe fn player_age_limit(j: usize) -> i8 {
    if read_i8(j + player_sheet::GOALKEEPER) >= 15 {
        0x22
    } else if read_i8(j + player_sheet::ATTACKER) >= 15 {
        0x1e
    } else if read_i8(j + player_sheet::ATTACKING_MIDFIELDER) < 15 {
        0x1f
    } else {
        0x1e
    }
}

/// Fills the retirement output entry: reason (+0x18), +0x19 = 0, person id and club id (-1 without club).
///
/// [pt] A saída de quem se aposenta: o motivo (+0x18), +0x19 = 0, o id (+0) e o id do clube (+4; −1 sem clube).
pub(crate) unsafe fn fill_retirement_entry(person: usize, out: usize, reason: u8) {
    *((out + retirement_entry::REASON) as *mut u8) = reason;
    *((out + retirement_entry::ANNOUNCED) as *mut u8) = 0;
    core::ptr::write_unaligned(out as *mut u32, read_usize(person) as u32);
    let c = read_usize(person + staff::CLUB_JOB);
    core::ptr::write_unaligned((out + retirement_entry::CLUB_ID) as *mut u32, if c != 0 { read_usize(c) as u32 } else { u32::MAX });
}

/// Sets both output dates (+8 and +0x10) to tomorrow (day + 1, current year).
///
/// [pt] Amanhã (dia + 1, só a palavra) nas duas datas da saída (+8 e +0x10; ano em +0xa e +0x12).
pub(crate) unsafe fn set_dates_tomorrow(out: usize) {
    let (dia, ano) = (read_u16(G_CURRENT_DATE).wrapping_add(1), read_u16(G_CURRENT_DATE_YEAR));
    write_u16(out + retirement_entry::ANNOUNCE_DATE, dia);
    write_u16(out + retirement_entry::ANNOUNCE_YEAR, ano);
    write_u16(out + retirement_entry::RETIRE_DATE, dia);
    write_u16(out + retirement_entry::RETIRE_YEAR, ano);
}

/// For age retirement without a valid key nation: both dates today + random(20), wrapping to random(10) of next year.
///
/// [pt] Sem nação-chave válida na aposentadoria pela idade: hoje + sorteio(20) nas duas datas; passando de 365, o dia
/// sorteio(10) do ano seguinte.
pub(crate) unsafe fn set_dates_no_key_nation(out: usize) {
    let a = (random(20) as u32).wrapping_add(read_usize(G_CURRENT_DATE) as u32) as u16;
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if a as i16 > 0x16d {
        let d = random(10) as u16;
        write_u16(out + retirement_entry::RETIRE_DATE, d);
        write_u16(out + retirement_entry::RETIRE_YEAR, year.wrapping_add(1));
        write_u16(out + retirement_entry::ANNOUNCE_DATE, d);
        write_u16(out + retirement_entry::ANNOUNCE_YEAR, year.wrapping_add(1));
    } else {
        write_u16(out + retirement_entry::ANNOUNCE_DATE, a);
        write_u16(out + retirement_entry::ANNOUNCE_YEAR, year);
        write_u16(out + retirement_entry::RETIRE_DATE, a);
        write_u16(out + retirement_entry::RETIRE_YEAR, year);
    }
}

/// Sets the retirement dates on leaving (+8 the announcement, +0x10 the retirement; only day and year, the +4 of each
/// date stays as it is), by the key nation of the club (`key_nation_index_of_club`) or, with no club, of the nation (+0x1a, `key_nation_index_of_nation`);
/// with neither, tomorrow.
///
/// Index 0..0x22: the announcement on today + 1 + random draw(95) (past 365, day random draw(60) of the following
/// year); the retirement at the nation's season end (+0x15 of the 72-byte record at 0xb63d60) + random draw(30) (past
/// 365, random draw(30) of the following year; with +0x1d, one more year); a retirement before the announcement is
/// moved to the announcement, and if it is less than 45 days after it (`exe::date_days_since`) the announcement moves to it.
///
/// Index out of range: `fora` (each routine has its own).
///
/// [pt] As datas da aposentadoria na saída (+8 o anúncio, +0x10 a aposentadoria; só o dia e o ano, o +4 de cada data fica
/// como está), pela nação-chave do clube (0x669360) ou, sem clube, da nação (+0x1a, 0x669340); sem os dois, amanhã.
/// Índice 0..0x22: o anúncio em hoje + 1 + sorteio(95) (passando de 365, o dia sorteio(60) do ano seguinte); a
/// aposentadoria no fim de temporada da nação (+0x15 do registro de 72 bytes em 0xb63d60) + sorteio(30) (passando de
/// 365, sorteio(30) do ano seguinte; com +0x1d, um ano a mais); a aposentadoria antes do anúncio fica no anúncio, e a
/// menos de 45 dias dele (0x54cdb0) o anúncio vai para ela. Índice fora: `fora` (cada rotina tem a sua).
pub(crate) unsafe fn set_retirement_dates(person: usize, out: usize, away: unsafe fn(usize)) {
    let club = read_usize(person + staff::CLUB_JOB);
    let k = if club != 0 {
        let by_club: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
        by_club(club) as u8 as i8
    } else {
        let nation = read_usize(person + staff::NATION);
        if nation == 0 {
            return set_dates_tomorrow(out);
        }
        exe::key_nation_index_of_nation(nation) as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return away(out);
    }
    crate::comparison::branch("cpr_8 dates by nation");
    let year = read_u16(G_CURRENT_DATE_YEAR);
    let a = (random(95) as u32).wrapping_add(read_usize(G_CURRENT_DATE) as u32).wrapping_add(1) as u16;
    let (dia1, ano1) = if a as i16 > 0x16d { (random(60) as u16, year.wrapping_add(1)) } else { (a, year) };
    write_u16(out + retirement_entry::ANNOUNCE_YEAR, ano1);
    write_u16(out + retirement_entry::ANNOUNCE_DATE, dia1);
    let t = 0xb63d60usize.wrapping_add((k as usize) * 0x48);
    let d2 = (random(30) as u16).wrapping_add(read_u16(t + key_nation_record::SEASON_END_DAY)); // the nation’s end of season
    write_u16(out + retirement_entry::RETIRE_DATE, d2);
    let more_one = *((t + key_nation_record::SEASON_SPANS_YEAR) as *const u8) != 0;
    if d2 as i16 > 0x16d {
        write_u16(out + retirement_entry::RETIRE_DATE, random(30) as u16);
        write_u16(out + retirement_entry::RETIRE_YEAR, year.wrapping_add(if more_one { 2 } else { 1 }));
    } else {
        write_u16(out + retirement_entry::RETIRE_YEAR, if more_one { year.wrapping_add(1) } else { year });
    }
    let mut t8 = [0u8; 8];
    let (da, aa) = (read_u16(out + retirement_entry::RETIRE_YEAR), read_u16(out + retirement_entry::ANNOUNCE_YEAR));
    if da < aa || (da == aa && read_i16(out + retirement_entry::RETIRE_DATE) < read_i16(out + retirement_entry::ANNOUNCE_DATE)) {
        exe::date_operator_3d(out + retirement_entry::RETIRE_DATE, (t8.as_mut_ptr()) as usize, out + retirement_entry::ANNOUNCE_DATE);
        return;
    }
    if exe::date_days_since(out + retirement_entry::RETIRE_DATE, read_usize(out + retirement_entry::ANNOUNCE_DATE) as u32, read_usize(out + 0xc) as u32) >= 0x2d {
        return;
    }
    exe::date_operator_3d(out + retirement_entry::ANNOUNCE_DATE, (t8.as_mut_ptr()) as usize, out + retirement_entry::RETIRE_DATE);
}

/// At a new game, decides whether a person with a club retires this season: non-players by the job age limit, players
/// by the profile limit, the ability, the club's competition and the contract status; fills the output and the dates.
///
/// `player_regen_check_initial_retirement(person, out)` (thiscall without reading this, ret 8): on a new game (from
/// `build_initial_retirement_list`), does the person with a club (+0x39) retire this season? The contract (contract_with_current_club) and the
/// status (contract_status). Without a player record (+0x61): without a manager record (+0x69) always; with it, when
/// past the limit (non_player_age_limit); reason 0x20. A player: past the record's limit (player_age_limit), v = ⌊⌊1 −
/// (age − limit) × 0.1⌋ × ability (+0xb)⌋ (16 bits); over 45, reason 1. A club without a competition (+0x57; without a
/// club, a nation without +0x11c): random(3) ≠ 0 retires, otherwise only a negative status with a club. Otherwise: v ≥
/// 5000 or +5 > 80, no; +5 < 10 or v < 1500, yes; with a club, only a negative status; without a club, reason 2 with
/// random(25 × +0x3c) = 0. Whoever retires: the output (fill_retirement_entry: id, club, reason) and the announcement
/// and retirement dates (set_retirement_dates, by the key nation; without a valid key nation, tomorrow); returns 1.
/// Original: `player_regen_check_initial_retirement` (0x7a79c0, player_regen.cpp, thiscall, ret 8).
///
/// [pt] { 0x7a79c0 } (thiscall sem ler o this, ret 8, player_regen.cpp): `player_regen_check_initial_retirement(pessoa,
/// saida)`: no jogo novo (pela 0x7a58f0), a pessoa com clube (+0x39) se aposenta nesta temporada? O contrato
/// (contract_with_current_club) e o estatuto (contract_status). Sem ficha de jogador (+0x61): sem ficha de técnico (+0x69) sempre;
/// com ela, passando do limite (non_player_age_limit); motivo 0x20. Jogador: passando do limite da ficha
/// (player_age_limit), v = ⌊⌊1 − (idade − limite) × 0,1⌋ × habilidade (+0xb)⌋ (16 bits); acima de 45 anos,
/// motivo 1. Clube sem competição (+0x57; sem clube, nação sem +0x11c): sorteio(3) ≠ 0 aposenta, senão só estatuto
/// negativo com clube. Senão: v ≥ 5000 ou +5 > 80, não; +5 < 10 ou v < 1500, sim; com clube, só estatuto negativo;
/// sem clube, motivo 2 com sorteio(25 × +0x3c) = 0. Quem se aposenta: a saída (fill_retirement_entry) e as datas
/// (set_retirement_dates; sem nação-chave válida, amanhã); devolve 1.
pub(crate) unsafe extern "thiscall" fn player_regen_check_initial_retirement(_this: usize, person: usize, out: usize) -> u8 {
    if read_usize(person + staff::CLUB_JOB) == 0 {
        return 0;
    }
    let c2 = contract_with_current_club(person);
    let status = contract_status(c2);
    let age = read_i8(person + staff::AGE);
    let j = read_usize(person + staff::PLAYER);
    let reason: u8 = 'm: {
        if j == 0 {
            if read_usize(person + staff::NON_PLAYER) != 0 && age <= non_player_age_limit(person, out) {
                return 0;
            }
            break 'm 0x20;
        }
        let lim = player_age_limit(j);
        if age <= lim {
            return 0;
        }
        let f = ftol(read_f64(rdata::F64_1) - (age as i32 - lim as i32) as f64 * read_f64(rdata::F64_0_1)) as i16;
        let v = ftol(f as f64 * read_i16(j + player_sheet::CURRENT_REPUTATION) as f64) as i16;
        if age > 0x2d {
            break 'm 1;
        }
        let club = read_usize(person + staff::CLUB_JOB);
        let draws = if club != 0 {
            read_usize(club + club::DIVISION) == 0
        } else {
            let n = read_usize(person + staff::NATION);
            n != 0 && *((n + nation::LEAGUE_SELECTED) as *const u8) == 0
        };
        if draws {
            if random(3) != 0 {
                break 'm 1;
            }
            if club == 0 || status >= 0 {
                return 0;
            }
            break 'm 1;
        }
        if v >= 0x1388 {
            return 0;
        }
        let hab = read_i16(j + player_sheet::CURRENT_ABILITY);
        if hab > 0x50 {
            return 0; // cpr8: high ability keeps the player
        }
        if hab < 0xa || v < 0x5dc {
            crate::comparison::branch("cpr_8 initial ability");
            break 'm 1;
        }
        if club != 0 {
            if status < 0 {
                break 'm 1;
            }
            return 0;
        }
        if random(25 * read_i8(j + player_sheet::STAMINA) as i32) != 0 {
            return 0;
        }
        2
    };
    fill_retirement_entry(person, out, reason);
    set_retirement_dates(person, out, set_dates_tomorrow);
    1
}

/// In the search for new retirements, decides whether the person retires: like the initial check, with the limit
/// adjusted by contract and wage, unemployment, contract end and potential; without a key nation, dates drawn near
/// today.
///
/// `player_regen_check_retirement(person, out)` (thiscall without reading this, ret 8), from the search for new
/// retirements (`find_new_retirements`). Without a player record: as in `player_regen_check_initial_retirement` (reason 0x20). A player above the record's limit:
/// over 33, the limit drops 1 without a contract with the club, or with +0x45 in the contract and +0x15/+0x14 of the
/// record = 20, or the high nibble of +0x4f = 7; with a club and a contract with wage (+0xc) above 750 × (+0x59 + 10)
/// until after this year (+0x2f), it drops 1 (and 1 more with +0x59 < 6). v = ⌊(1 − (age − limit) × 0.075) × ability
/// (+0xb)⌋ (16 bits). Reason 1: over 45; over 34 without a club and unemployed for more than 65 days (since +0x3e); 3
/// years above the limit, over 31 and the contract's +0x3e > 5; 4 above: without a club; or a contract less than 300
/// days from its exe::staff_contract_days_to_end_date(`days_to_end_date`), status below 50 and manager (+0x69) or +0x57 > random(20) with potential × 0.85 above
/// the current ability (+7 and +5). Then v > 5500, no. A club in a competition not running in the game ([[0xadadfc] +
/// id × 4] = 0, or without +0x57; without a club, a nation without +0x11c): random(3) = 0 retires; with a club and a
/// negative status too; otherwise only with v < 3500. Otherwise as in `player_regen_check_initial_retirement` (+5 > 100 no; +5 < 10 or v < 1500 yes;
/// with a club only a negative status; without a club reason 2 with random(25 × +0x3c) = 0). Whoever retires: the
/// output and the dates (without a valid key nation, set_dates_no_key_nation, near today); returns 1.
/// Original: `player_regen_check_retirement` (0x7a7310, player_regen.cpp, thiscall, ret 8).
///
/// [pt] { 0x7a7310 } (thiscall sem ler o this, ret 8, player_regen.cpp): `player_regen_check_retirement(pessoa, saida)`:
/// na busca de novas aposentadorias (0x7a5790), a pessoa se aposenta? Sem ficha de jogador: como na 0x7a79c0 (motivo
/// 0x20). Jogador acima do limite da ficha: acima de 33 anos, o limite cai 1 sem contrato com o clube, ou com +0x45 no
/// contrato e +0x15/+0x14 da ficha = 20 ou o nibble alto de +0x4f = 7; com clube e contrato de salário (+0xc) acima de
/// 750 × (+0x59 + 10) até depois deste ano (+0x2f), cai 1 (e mais 1 com +0x59 < 6). v = ⌊(1 − (idade − limite) ×
/// 0,075) × habilidade (+0xb)⌋ (16 bits). Motivo 1: acima de 45; acima de 34 sem clube e desempregado há mais de 65
/// dias (desde +0x3e); 3 anos acima do limite, mais de 31 e +0x3e do contrato > 5; 4 acima: sem clube; ou contrato a
/// menos de 300 dias do fim (0x85fa00), estatuto abaixo de 50 e técnico (+0x69) ou +0x57 > sorteio(20) com o
/// potencial × 0,85 acima da habilidade atual (+7 e +5). Depois v > 5500, não. Clube em competição fora do jogo
/// ([[0xadadfc] + id × 4] = 0, ou sem +0x57; sem clube, nação sem +0x11c): sorteio(3) = 0 aposenta; com clube e
/// estatuto negativo também; senão só com v < 3500. Senão como na 0x7a79c0 (+5 > 100 não; +5 < 10 ou v < 1500 sim;
/// com clube só estatuto negativo; sem clube motivo 2 com sorteio(25 × +0x3c) = 0). Quem se aposenta: a saída e as
/// datas (sem nação-chave válida, set_dates_no_key_nation); devolve 1.
pub(crate) unsafe extern "thiscall" fn player_regen_check_retirement(_this: usize, person: usize, out: usize) -> u8 {
    let c2 = contract_with_current_club(person);
    let status = contract_status(c2);
    let age = read_i8(person + staff::AGE);
    let j = read_usize(person + staff::PLAYER);
    let reason: u8 = 'm: {
        if j == 0 {
            if read_usize(person + staff::NON_PLAYER) != 0 && age <= non_player_age_limit(person, out) {
                return 0;
            }
            break 'm 0x20;
        }
        let mut lim = player_age_limit(j);
        if age <= lim {
            return 0;
        }
        if age > 0x21 {
            if c2 == 0 {
                lim = lim.wrapping_sub(1);
            } else if read_usize(c2 + 0x45) != 0
                && (read_i8(j + player_sheet::ATTACKER) == 0x14 || read_i8(j + player_sheet::ATTACKING_MIDFIELDER) == 0x14 || *((c2 + contract::SQUAD_STATUS) as *const u8) & 0xf0 == 0x70)
            {
                lim = lim.wrapping_sub(1);
            }
        }
        if read_usize(person + staff::CLUB_JOB) != 0 && c2 != 0 {
            let x = read_i8(person + staff::LOYALTY) as i32;
            if (read_usize(c2 + contract::WAGE) as i32) > (x + 10).wrapping_mul(750) && read_u16(c2 + 0x2f) > read_u16(G_CURRENT_DATE_YEAR) {
                lim = lim.wrapping_sub(1);
                if x < 6 {
                    lim = lim.wrapping_sub(1);
                }
            }
        }
        let v = ftol((read_f64(rdata::F64_1) - (age as i32 - lim as i32) as f64 * read_f64(rdata::F64_0_075)) * read_i16(j + player_sheet::CURRENT_REPUTATION) as f64) as i16;
        if age > 0x2d {
            break 'm 1;
        }
        if age > 0x22 && read_usize(person + staff::CLUB_JOB) == 0 {
            let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            if days(G_CURRENT_DATE, read_usize(person + staff::DATE_JOINED_CLUB) as u32, read_usize(person + 0x42) as u32) > 0x41 {
                break 'm 1;
            }
        }
        let (i, l) = (age as i32, lim as i32);
        if i > l + 2 && age > 0x1f && c2 != 0 && read_i8(c2 + contract::UNHAPPY_FLAG) > 5 {
            break 'm 1;
        }
        if i > l + 3 {
            if read_usize(person + staff::CLUB_JOB) == 0 {
                break 'm 1;
            }
            if c2 != 0 && (exe::staff_contract_days_to_end_date(c2) as i16) < 300 && status < 0x32 {
                if read_usize(person + staff::NON_PLAYER) != 0 {
                    break 'm 1;
                }
                if read_i8(person + staff::AMBITION) as i32 > random(20) {
                    let (cur, pot) = (read_i16(j + player_sheet::CURRENT_ABILITY) as f64, read_i16(j + player_sheet::POTENTIAL_ABILITY) as f64);
                    if !(pot * read_f64(rdata::F64_0_85) <= cur) {
                        break 'm 1;
                    }
                }
            }
        }
        if v > 0x157c {
            return 0;
        }
        let club = read_usize(person + staff::CLUB_JOB);
        let draws = if club != 0 {
            let lg = read_usize(club + club::DIVISION);
            lg == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(lg).wrapping_mul(4))) == 0
        } else {
            let n = read_usize(person + staff::NATION);
            n != 0 && *((n + nation::LEAGUE_SELECTED) as *const u8) == 0
        };
        if draws {
            crate::comparison::branch("cpr_8 out of the game");
            if random(3) == 0 {
                break 'm 1; // cpr8: one in three retires
            }
            if read_usize(person + staff::CLUB_JOB) != 0 && status < 0 {
                break 'm 1;
            }
            if v >= 0xdac {
                return 0;
            }
            break 'm 1;
        }
        let cur = read_i16(j + player_sheet::CURRENT_ABILITY);
        if cur > 0x64 {
            return 0;
        }
        if cur < 0xa || v < 0x5dc {
            break 'm 1;
        }
        if club != 0 {
            if status < 0 {
                break 'm 1;
            }
            return 0;
        }
        if random(25 * read_i8(j + player_sheet::STAMINA) as i32) != 0 {
            return 0;
        }
        2
    };
    fill_retirement_entry(person, out, reason);
    set_retirement_dates(person, out, set_dates_no_key_nation);
    1
}

/// At a new game, tests every eligible person (`player_regen_check_initial_retirement`), adds those who retire to the list and sorts it.
///
/// `PLAYER_REGEN::build_initial_retirement_list`; SEH only for the local dates, with an empty destructor. On a new
/// game, each person (0x6e bytes in [0xae23bc], up to [0xae2364], re-read at each step) with id below [0xae2364] − 16,
/// without bits 0x30 of +0xb of the 79-byte record ([0xae2d0c]) and without a contract (`exe::contract_manager_get_contract`) with +0x49 is tested
/// (`player_regen_check_initial_retirement`); whoever retires goes into the list (`add_retirement`). At the end the qsort of the list ([this+4], count +8, 26
/// bytes, `player_regen_compare_retirements`).
/// Original: `PLAYER_REGEN::build_initial_retirement_list` (0x7a58f0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a58f0 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::build_initial_retirement_list`: no jogo novo, cada pessoa (0x6e bytes em [0xae23bc], até [0xae2364],
/// relida a cada passo) de id abaixo de [0xae2364] − 16, sem os bits 0x30 do +0xb do registro de 79 bytes
/// ([0xae2d0c]) e sem contrato (0x4cf5a0) com +0x49 é testada (0x7a79c0); quem se aposenta entra na lista
/// (0x7a8510). No fim o qsort da lista ([this+4], conta +8, 26 bytes, 0x7ae2a0).
pub(crate) unsafe extern "thiscall" fn build_initial_retirement_list(this: usize) {
    crate::comparison::branch("cpr_8 initial list");
    // simplification: the original builds the entry’s two dates (0x54a110) and leaves the rest as it was on the stack; the
    // 0x7a79c0 writes everything 0x7a8510 copies before returning 1 (the third local date is never read: left out)
    let mut e = [0u8; 0x1a];
    exe::date_date_54a110((e.as_mut_ptr().add(8)) as usize);
    exe::date_date_54a110((e.as_mut_ptr().add(0x10)) as usize);
    let mut p = read_usize(G_STAFF_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_STAFF_COUNT) as i32 {
        let id = read_usize(p) as i32;
        if id < (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10)
            && *((read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 == 0
        {
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            if (c == 0 || *((c + contract::LEAVING_ON_BOSMAN) as *const u8) == 0) && exe::player_regen_check_initial_retirement(this, p, e.as_mut_ptr()) != 0 {
                exe::player_regen_add_retirement(this, e.as_ptr());
            }
        }
        i += 1;
        p = p.wrapping_add(0x6e);
    }
    exe::crt_qsort(read_usize(this + player_regen_fields::RETIREMENTS), read_usize(this + player_regen_fields::RETIREMENT_COUNT) as u32, 0x1a, 0x7ae2a0);
}

/// Regions the two retirement decisions touch: random state, [0xae1080], the output entry and `exe::date_days_since`'s static date.
///
/// [pt] As regiões que as duas decisões mudam: o sorteio, [0xae1080], a saída e a data estática da 0x54cdb0.
pub(crate) unsafe fn retirement_decision_regions(out: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&[(G_CONTRACT_LOOKUP_RESULT, 4), (out, 0x1a), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)]);
    reg
}

// simplification: the initial list appends (realloc) and rolls in each test: it does not run twice. Original only; the mode
// Rust and the career prove the copy.

/// Writes a zero dword at the address.
pub(crate) unsafe fn zero_dword(a: usize) {
    core::ptr::write_unaligned(a as *mut u32, 0);
}

/// Tests every eligible person for retirement by age (`player_regen_check_retirement`) or unemployment (`player_regen_check_unemployed_retirement`), adds those who retire and
/// sorts the list.
///
/// `PLAYER_REGEN::find_new_retirements` (on the days of `is_player_regen_day`); SEH only for the local dates, with an empty
/// destructor. Each person (0x6e bytes in [0xae23bc], up to the end computed with the count [0xae2364] at the start).
/// Excluded: under 30 years old (+0x18) at a club of a competition that runs in the game ([[0xadadfc] + id × 4] of the
/// competition +0x57); id from [0xae2364] − 16 on (re-read at each step); bits 0x30 of +0xb of the 79-byte record
/// ([0xae2d0c]); those already in the list ([this+4], 26 bytes, count +8); a contract (`exe::contract_manager_get_contract`) with +0x49. The
/// others: retirement by age (`player_regen_check_retirement`) or, failing that, the unemployed one (`player_regen_check_unemployed_retirement`); whoever retires goes into the
/// list (`add_retirement`). At the end the qsort of the list (`player_regen_compare_retirements`).
/// Original: `PLAYER_REGEN::find_new_retirements` (0x7a5790, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a5790 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::find_new_retirements` (nos dias da 0x5d6250): cada pessoa (0x6e bytes em [0xae23bc], até o fim
/// calculado com a conta [0xae2364] do começo). Ficam de fora: menos de 30 anos (+0x18) em clube de competição que roda
/// no jogo ([[0xadadfc] + id × 4] da competição +0x57); id a partir de [0xae2364] − 16 (relida a cada passo); os bits
/// 0x30 do +0xb do registro de 79 bytes ([0xae2d0c]); quem já está na lista ([this+4], 26 bytes, conta +8); contrato
/// (0x4cf5a0) com +0x49. Os outros: aposentadoria pela idade (0x7a7310) ou, sem ela, do desempregado (0x7a8050); quem
/// se aposenta entra na lista (0x7a8510). No fim o qsort da lista (0x7ae2a0).
pub(crate) unsafe extern "thiscall" fn find_new_retirements(this: usize) {
    // simplification: the original builds the two entry dates (0x54a110) and leaves the rest as it was on the stack; the two
    // decisions write everything 0x7a8510 copies before returning 1 (the third local date is never read: left out)
    let mut e = [0u8; 0x1a];
    exe::date_date_54a110((e.as_mut_ptr().add(8)) as usize);
    exe::date_date_54a110((e.as_mut_ptr().add(0x10)) as usize);
    let mut p = read_usize(G_STAFF_TABLE);
    let end = p.wrapping_add(read_usize(G_STAFF_COUNT).wrapping_mul(staff::SIZE));
    while p < end {
        'x: {
            if read_i8(p + staff::AGE) < 0x1e {
                let c = read_usize(p + staff::CLUB_JOB);
                if c != 0 {
                    let lg = read_usize(c + 0x57);
                    if lg != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(lg).wrapping_mul(4))) != 0 {
                        break 'x;
                    }
                }
            }
            let id = read_usize(p) as i32;
            if id >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
                break 'x;
            }
            if *((read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
                break 'x;
            }
            let k = read_usize(this + player_regen_fields::RETIREMENT_COUNT) as i32;
            if k != 0 {
                let l = read_usize(this + player_regen_fields::RETIREMENTS);
                for i in 0..k.max(0) as usize {
                    if read_usize(l + i * 0x1a) as i32 == id {
                        break 'x;
                    }
                }
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            if c != 0 && *((c + 0x49) as *const u8) != 0 {
                break 'x;
            }
            if exe::player_regen_check_retirement(this, p, e.as_mut_ptr()) != 0 {
                exe::player_regen_add_retirement(this, e.as_ptr());
            } else if exe::player_regen_check_unemployed_retirement(p, e.as_mut_ptr()) != 0 {
                crate::comparison::branch("cpr_9 unemployed");
                exe::player_regen_add_retirement(this, e.as_ptr());
            }
        }
        p = p.wrapping_add(0x6e);
    }
    exe::crt_qsort(read_usize(this + player_regen_fields::RETIREMENTS), read_usize(this + player_regen_fields::RETIREMENT_COUNT) as u32, 0x1a, 0x7ae2a0);
}

/// Removes the person from the slot of job `cargo` (signed type) in club `c` (the person's club at +`campo`: 0x39 the
/// club, 0x24 the national team).
///
/// 1 chairman (`+0xbf`); 2 the three at `+0xc3`; 5 and 12 manager (`+0xcf`, only if it is him): for 5, if human (id
/// from [`0xae2364`] − 16 on), the club's offers fall (`0x8c4950`); he leaves the job (`0x69b790` with 3, 0, 0) and, if
/// the club can be managed (`0x544820`), the vacancy opens (`0x69ba50`); 6 assistant (`+0xd3`); 8 and 15 the five at
/// `+0x19f`, after distributing the coaches of the human club (`0x600a50` with 1, 1; `0x8b2880` with 0); 9 the seven at
/// `+0x1b3` and, at the human club, +`campo` = 0 and the manager's scout lists (`0x7f7270`); 10 the three at `+0x1cf`.
///
/// [pt] Tira a pessoa da vaga do `cargo` (tipo com sinal) no clube `c` (o da pessoa em +`campo`: 0x39 o clube, 0x24 a
/// seleção): 1 presidente (+0xbf); 2 as três de +0xc3; 5 e 12 técnico (+0xcf, só se é ele): no 5, sendo humano (id a
/// partir de [0xae2364] − 16), as ofertas do clube caem (0x8c4950); sai do cargo (0x69b790 com 3, 0, 0) e, se o clube
/// pode ser dirigido (0x544820), a vaga abre (0x69ba50); 6 auxiliar (+0xd3); 8 e 15 as cinco de +0x19f, depois da
/// distribuição dos treinadores do clube humano (0x600a50 com 1, 1; 0x8b2880 com 0); 9 as sete de +0x1b3 e, no
/// clube humano, +`campo` = 0 e as listas dos olheiros do técnico (0x7f7270); 10 as três de +0x1cf.
pub(crate) unsafe fn leave_job_slot(person: usize, c: usize, job: i8, field: usize) {
    let clear = |base: usize, n: usize| {
        for k in 0..n {
            let a = c + base + 4 * k;
            if read_usize(a) == person {
                zero_dword(a);
            }
        }
    };
    let human = || {
        exe::human_manager_is_human_club(c, 1, (1) as u32, (0) as u32, 0) != 0
    };
    match job {
        1 => zero_dword(c + club::CHAIRMAN),
        2 => clear(0xc3, 3),
        5 | 12 => {
            if read_usize(c + club::MANAGER) != person {
                return;
            }
            crate::comparison::branch("cpr_9 manager");
            if job == 5 && (read_usize(person) as i32) >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
                exe::transfer_manager_cancel_club_offers(read_usize(G_TRANSFER_MANAGER), c);
            }
            exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), c, 3, 0, 0);
            let can: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
            if can(c) as u8 != 0 {
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, person); // cpr9: the manager slot opens
            }
        }
        6 => zero_dword(c + club::ASSISTANT_MANAGER),
        8 | 15 => {
            if human() {
                exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), c, 0);
            }
            crate::comparison::branch("cpr_9 coach");
            clear(0x19f, 5); // cpr9: the coach slots
        }
        9 => {
            clear(0x1b3, 7);
            if human() {
                zero_dword(person + field);
                let lists: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
                lists(read_usize(G_SCOUT_MANAGER), read_usize(c + club::MANAGER));
            }
        }
        10 => clear(0x1cf, 3),
        _ => {}
    }
}

/// removes the person from the job slots at the club and national team (chairman, directors, manager with the vacancy
/// opened, assistant, coaches, scouts, physios) and clears the club, the national team and the job types
/// Original: `player_regen_retire_from_non_playing_staff` (0x7aaeb0, player_regen.cpp, stdcall, ret 4).
///
/// [pt] { 0x7aaeb0 } (stdcall, ret 4, player_regen.cpp): `player_regen_retire_from_non_playing_staff(pessoa)` (pela
/// 0x7a59e0, na aposentadoria): sai da vaga do cargo (+0x3d) no clube (+0x39) e da do cargo na seleção (+0x28) na
/// seleção (+0x24) (leave_job_slot); clube e seleção zerados, e os dois tipos de cargo.
pub(crate) unsafe extern "stdcall" fn player_regen_retire_from_non_playing_staff(person: usize) {
    let c = read_usize(person + staff::CLUB_JOB);
    if c != 0 {
        leave_job_slot(person, c, read_i8(person + staff::JOB_FOR_CLUB), 0x39);
        zero_dword(person + staff::CLUB_JOB);
    }
    let s = read_usize(person + staff::NATIONAL_JOB);
    if s != 0 {
        leave_job_slot(person, s, read_i8(person + staff::JOB_FOR_NATION), 0x24);
        zero_dword(person + staff::NATIONAL_JOB);
    }
    *((person + staff::JOB_FOR_CLUB) as *mut u8) = 0;
    *((person + staff::JOB_FOR_NATION) as *mut u8) = 0;
}

// simplification: the search appends (realloc) and rolls on each test: it does not run twice. Original only; Rust mode and the
// career runs prove the copy.

/// Does leaving jobs only touch the person and the two clubs (not a club manager nor coach/scout jobs of a human club)?
///
/// [pt] A saída dos cargos só mexe na pessoa e nos dois clubes? Não quando é técnico do clube (5, 12) nem nos cargos que
/// chamam rotinas do clube humano (8, 15, 9).
pub(crate) unsafe fn job_exit_is_memory_only(person: usize) -> bool {
    for (campo, tipo) in [(0x39usize, 0x3dusize), (0x24, 0x28)] {
        let c = read_usize(person + campo);
        if c == 0 {
            continue;
        }
        match read_i8(person + tipo) {
            5 | 12 if read_usize(c + club::MANAGER) == person => return false,
            8 | 15 | 9 if exe::human_manager_is_human_club(c, 1, (1) as u32, (0) as u32, 0) != 0 => return false,
            _ => {}
        }
    }
    true
}

/// Regions of leaving jobs: the person (0x6e) and his club and national team records (0x245).
pub(crate) unsafe fn job_exit_regions(person: usize) -> Vec<(usize, usize)> {
    vec![(person, 0x6e), (read_usize(person + staff::CLUB_JOB), 0x245), (read_usize(person + staff::NATIONAL_JOB), 0x245)]
}

// simplification: the history goes to the file (does not run twice) and the copy without it only writes [0xae1080]: only the
// original; the copy takes the branch.

/// Scores a person as the donor of non-player data: −10000 when under 31 or flagged; otherwise random draws and
/// attribute terms.
///
/// `player_regen_score_non_player_candidate(person)` (stdcall, ret 4): the person's score as donor of the non-player
/// data (from `player_regen_pick_staff_for_non_player_data`, for each non-player person). −10000 with +0x69, with bits 0x30 of +0xb of the 79-byte record
/// ([0xae2d0c]) or under 31 (+0x18). Otherwise random 150 × (age − 31); with a club (+0x39) with reputation (+0x80) and
/// a player record (+0x61), + 250 × (current ability +0xb ÷ reputation); with a player record, + random(⌊(+0x2f ÷ 2)³⌋)
/// (pow 0x945ff0 and _ftol), +100 with +0xd > 7500 and +50 with +0xb > 7500; the attributes + 5 × (3 × (+0x5d + +0x5a)
/// + 2 × (+0x59 + +0x58) + +0x56) + ⌊+0x5b²⌋ + ⌊+0x57²⌋. With a club: a random draw of the score (+75 if the club has
/// no +0xcf); without a club: + (+0x22)², and the score doubles if the player's +0xd exceeds a random(10000) (it reads
/// the player record without checking, as the original does). Hot (every person on each call): no counter.
/// Original: `player_regen_score_non_player_candidate` (0x7a7150, player_regen.cpp, stdcall, ret 4).
///
/// [pt] { 0x7a7150 } (stdcall, ret 4, player_regen.cpp): `player_regen_score_non_player_candidate(pessoa)`: a nota da pessoa
/// como doadora dos dados de não jogador (pela 0x7a6e60, a cada pessoa não jogadora). −10000 com +0x69, com os bits
/// 0x30 do +0xb do registro de 79 bytes ([0xae2d0c]) ou abaixo de 31 anos (+0x18). Senão sorteia 150 × (idade − 31);
/// com clube (+0x39) de reputação (+0x80) e jogador (+0x61), + 250 × (habilidade atual +0xb ÷ reputação); com
/// jogador, + sorteio(⌊(+0x2f ÷ 2)³⌋) (pow 0x945ff0 e _ftol), +100 com +0xd > 7500 e +50 com +0xb > 7500; os
/// atributos + 5 × (3 × (+0x5d + +0x5a) + 2 × (+0x59 + +0x58) + +0x56) + ⌊+0x5b²⌋ + ⌊+0x57²⌋. Com clube: sorteio da
/// nota (+75 se o clube não tem +0xcf); sem clube: + (+0x22)², e a nota dobra se o +0xd do jogador passa de um
/// sorteio(10000) (lê o jogador sem conferir, como a original). Quente (todas as pessoas a cada chamada): sem contador.
pub(crate) unsafe extern "stdcall" fn player_regen_score_non_player_candidate(person: usize) -> i32 {
    if read_usize(person + staff::NON_PLAYER) != 0 {
        return -10000;
    }
    let id = read_usize(person);
    if *((read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
        return -10000;
    }
    let age = read_i8_as_i32(person + staff::AGE);
    if age < 31 {
        return -10000;
    }
    let mut rating = random((age - 31) * 150); // a idade
    let club = read_usize(person + staff::CLUB_JOB);
    if club != 0 {
        let injury_word_0c = read_i16_as_i32(club + club::REPUTATION);
        let player = read_usize(person + staff::PLAYER);
        if injury_word_0c != 0 && player != 0 {
            rating = rating.wrapping_add((read_i16_as_i32(player + player_sheet::CURRENT_REPUTATION) / injury_word_0c).wrapping_mul(250));
        }
    }
    let player = read_usize(person + staff::PLAYER);
    if player != 0 {
        let v = read_i8_as_i32(player + player_sheet::LEADERSHIP) / 2;
        rating = rating.wrapping_add(random(ftol(game_pow(v as f64, 3.0))));
        if read_i16_as_i32(player + player_sheet::WORLD_REPUTATION) > 7500 {
            rating = rating.wrapping_add(100);
        }
        if read_i16_as_i32(player + player_sheet::CURRENT_REPUTATION) > 7500 {
            rating = rating.wrapping_add(50);
        }
    }
    let quadrados = ftol(game_pow(read_i8_as_i32(person + staff::PROFESSIONALISM) as f64, 2.0)).wrapping_add(ftol(game_pow(read_i8_as_i32(person + staff::AMBITION) as f64, 2.0)));
    let sum = 3 * (read_i8_as_i32(person + staff::TEMPERAMENT) + read_i8_as_i32(person + staff::PRESSURE)) + 2 * (read_i8_as_i32(person + staff::LOYALTY) + read_i8_as_i32(person + staff::DETERMINATION)) + read_i8_as_i32(person + staff::ADAPTABILITY);
    rating = rating.wrapping_add(sum * 5).wrapping_add(quadrados);
    let club = read_usize(person + staff::CLUB_JOB);
    if club != 0 {
        if read_usize(club + club::MANAGER) == 0 {
            rating = rating.wrapping_add(75);
        }
        return random(rating);
    }
    let b = *((person + staff::INT_APPS) as *const u8) as i32;
    rating = rating.wrapping_add(b * b);
    let r = random(10000); // no club
    if read_i16_as_i32(read_usize(person + staff::PLAYER) + 0xd) > r {
        rating = rating.wrapping_add(rating);
    }
    random(rating)
}

/// Effective date of a retirement list entry: announcement (+8) if not yet past the game date, else retirement (+0x10).
///
/// [pt] A data que vale numa entrada da lista de aposentadorias: a do anúncio (+8: dia +0, ano +2) se ainda não passou da
/// data do jogo ([0xae2c90] dia, [0xae2c92] ano: o ano sem sinal, o dia com sinal), senão a da aposentadoria (+0x10).
pub(crate) unsafe fn entry_effective_date(e: usize) -> usize {
    let (dia, ano) = (read_i16_as_i32(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let a = read_u16(e + retirement_entry::ANNOUNCE_YEAR);
    if a < ano || (a == ano && read_i16_as_i32(e + retirement_entry::ANNOUNCE_DATE) < dia) {
        e + retirement_entry::RETIRE_DATE
    } else {
        e + retirement_entry::ANNOUNCE_DATE
    }
}

/// qsort comparator of the retirement list: by the effective date, year and day; on a tie by id.
///
/// `player_regen_compare_retirements(a, b)` (cdecl): the qsort of the retirement list (0x1a-byte records; from `process_retirements`
/// every day, which stops at the first future entry, from the initial lists `find_new_retirements`/`build_initial_retirement_list` and from `remove_retirement`): by
/// the effective date (entry_effective_date: the announcement +8 if not yet past, otherwise the retirement +0x10), the
/// year unsigned and the day signed; on a tie by the id (+0). Without `a`, −1; without `b`, 1. Hot (qsort): no counter.
/// Original: `player_regen_compare_retirements` (0x7ae2a0, player_regen.cpp, cdecl).
///
/// [pt] { 0x7ae2a0 } (cdecl, player_regen.cpp): `player_regen_compare_retirements(a, b)`: o qsort da lista de aposentadorias
/// (registros de 0x1a bytes; pela 0x7a59e0 todo dia, que para na primeira entrada futura, e pelas listas iniciais
/// 0x7a5790/0x7a58f0 e pela 0x7a8690): pela data que vale (entry_effective_date), o ano sem sinal e o dia com sinal;
/// no empate pelo id (+0). Sem `a`, −1; sem `b`, 1. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (da, db) = (entry_effective_date(a), entry_effective_date(b));
    let (aa, ab) = (read_u16(da + 2), read_u16(db + 2));
    if aa != ab {
        return aa as i32 - ab as i32; // the earliest year first
    }
    let (ja, jb) = (read_i16_as_i32(da), read_i16_as_i32(db));
    if ja == jb {
        return (read_usize(a) as i32).wrapping_sub(read_usize(b) as i32);
    }
    ja - jb
}

/// qsort comparator of the regen queue: by year (+6), day (+4) and id (+0).
///
/// `player_regen_compare_queue_entries(a, b)` (cdecl): the qsort of the regen queue (0xc-byte records, at the end of
/// `process_retirements` every day; `process_regen_queue` creates the regens of the due entries): by year (+6, unsigned), then by day (+4,
/// signed), then by id (+0). Without `a`, −1; without `b`, 1. Hot (qsort): no counter.
/// Original: `player_regen_compare_queue_entries` (0x7ae240, player_regen.cpp, cdecl).
///
/// [pt] { 0x7ae240 } (cdecl, player_regen.cpp): `player_regen_compare_queue_entries(a, b)`: o qsort da fila de regens
/// (registros de 0xc bytes, no fim da 0x7a59e0 todo dia; a 0x7a65d0 cria os regens das entradas vencidas): pelo ano
/// (+6, sem sinal), depois pelo dia (+4, com sinal), depois pelo id (+0). Sem `a`, −1; sem `b`, 1. Quente (qsort):
/// sem contador.
pub(crate) unsafe extern "C" fn player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (aa, ab) = (read_u16(a + regen_queue_entry::REGEN_YEAR), read_u16(b + regen_queue_entry::REGEN_YEAR));
    if aa > ab {
        return 1; // o ano mais tarde depois
    }
    if aa < ab {
        return -1;
    }
    let (ja, jb) = (read_i16_as_i32(a + regen_queue_entry::REGEN_DATE), read_i16_as_i32(b + regen_queue_entry::REGEN_DATE));
    if ja == jb {
        return (read_usize(a) as i32).wrapping_sub(read_usize(b) as i32);
    }
    ja - jb
}

/// Removes person `p` from the team at `p + campo` (+0x39 the club, +0x24 the national team).
///
/// The tactic (`remove_player_from_selection` on [0xde1d08], with 0 and then with 1 as the 3rd argument) and the first of the 50 slots
/// (+0xd7) holding him, which is emptied. The team is re-read after each call, as in the original routine.
///
/// [pt] Tira a pessoa `p` do time em `p + campo` (+0x39 o clube, +0x24 a seleção): a tática (0x896c60 de [0xde1d08], com 0
/// e depois com 1 no 3º argumento) e a primeira das 50 vagas (+0xd7) com ela, que esvazia. O time é relido depois de
/// cada chamada, como na original.
pub(crate) unsafe fn remove_from_team(p: usize, field: usize) {
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), read_usize(p + field), p, 0, 1);
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), read_usize(p + field), p, 1, 1);
    let team = read_usize(p + field);
    if let Some(k) = (0..0x32usize).find(|&k| read_usize(team + club::SQUAD + 4 * k) == p) {
        core::ptr::write_unaligned((team + club::SQUAD + 4 * k) as *mut usize, 0);
    }
}

/// Formats the "already retired" message into buffer 0xde1f64 (first name, surname, id).
///
/// [pt] O aviso "already retired" no buffer [0xde1f64] (nome +4, sobrenome +8, id +0).
pub(crate) unsafe fn format_already_retired(p: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(0xde1f64, 0xa70150, read_usize(p + staff::FIRST_NAME), read_usize(p + staff::SECOND_NAME), read_usize(p));
}

/// Removes the player from his club and national team and changes the player job to the staff job; without a kept job,
/// clears the club and the national team.
///
/// `PLAYER_REGEN::retire_from_playing_staff(p)` (stdcall, ret 4): removes `p` from the club (+0x39) and the national
/// team (+0x24) (`remove_from_team`: the tactics and the 50 slots) and changes the player job to the staff one (tables
/// 0x7aae60 and 0x7aae90): by the club job (+0x3d, signed byte; outside 0..16 the default applies) 12 → 5, 13 → 6, 15 →
/// 8 (stays at the club), 0 and 16 (warning "already retired") nothing, 1 to 10 the routine ends there, the others 0x10
/// (retired). Then by the national team job (+0x28) in the same way, but 12/13/15 write into +0x3d (the original writes
/// there, not into +0x28) and the default writes 0x10 into +0x28. With no job kept, the club (+0x39) and the national
/// team (+0x24) are zeroed. No SEH, no random draw.
/// Original: `PLAYER_REGEN::retire_from_playing_staff` (0x7aace0, player_regen.cpp, stdcall, ret 4).
///
/// [pt] { 0x7aace0 } (stdcall, ret 4, player_regen.cpp): `PLAYER_REGEN::retire_from_playing_staff(p)`: tira `p` do clube
/// (+0x39) e da seleção (+0x24) (`remove_from_team`) e troca o cargo de jogador pelo da comissão (tabelas 0x7aae60 e
/// 0x7aae90): pelo cargo no clube (+0x3d, byte com sinal; fora de 0..16 vale o padrão) 12 → 5, 13 → 6, 15 → 8 (fica no
/// clube), 0 e 16 (aviso "already retired") nada, 1 a 10 a rotina acaba ali, os outros 0x10 (aposentado). Depois pelo
/// cargo na seleção (+0x28) do mesmo jeito, mas 12/13/15 gravam no +0x3d (a original grava ali, não no +0x28) e o
/// padrão grava 0x10 no +0x28. Sem cargo que fica, o clube (+0x39) e a seleção (+0x24) zeram. Sem SEH, sem sorteio.
pub(crate) unsafe extern "stdcall" fn retire_from_playing_staff(p: usize) {
    if read_usize(p + staff::CLUB_JOB) != 0 {
        remove_from_team(p, 0x39);
    }
    if read_usize(p + staff::NATIONAL_JOB) != 0 {
        remove_from_team(p, 0x24);
    }
    let stays_in_club = match read_i8(p + staff::JOB_FOR_CLUB) as i32 {
        0 => false,
        1..=10 => return, // staff: the original exits through the end of the routine
        12 => {
            write_i8(p + staff::JOB_FOR_CLUB, 5);
            true
        }
        13 => {
            write_i8(p + staff::JOB_FOR_CLUB, 6);
            true
        }
        15 => {
            write_i8(p + staff::JOB_FOR_CLUB, 8);
            true
        }
        16 => {
            format_already_retired(p);
            false
        }
        _ => {
            crate::comparison::branch("cpn_4 retired player");
            write_i8(p + staff::JOB_FOR_CLUB, 0x10); // aposentado
            false
        }
    };
    let stays_in_national_team = match read_i8(p + staff::JOB_FOR_NATION) as i32 {
        0 => false,
        1..=10 => return,
        12 => {
            write_i8(p + staff::JOB_FOR_CLUB, 5); // in +0x3d, like the original
            true
        }
        13 => {
            write_i8(p + staff::JOB_FOR_CLUB, 6);
            true
        }
        15 => {
            write_i8(p + staff::JOB_FOR_CLUB, 8);
            true
        }
        16 => {
            format_already_retired(p);
            false
        }
        _ => {
            write_i8(p + staff::JOB_FOR_NATION, 0x10);
            false
        }
    };
    if !stays_in_club {
        core::ptr::write_unaligned((p + staff::CLUB_JOB) as *mut usize, 0);
    }
    if !stays_in_national_team {
        core::ptr::write_unaligned((p + staff::NATIONAL_JOB) as *mut usize, 0);
    }
}

/// Youth with second nationality Basque Country may only join five specific clubs: true for other clubs.
///
/// [pt] O jovem com a segunda nacionalidade (+0x1e) igual a [0x9cf424] só vai para os clubes de id [0x9d0120], [0x9d0134],
/// [0x9d0128], [0x9d01a0] e [0x9d0124]: `true` para os outros clubes.
pub(crate) unsafe fn is_basque_restricted(player: usize, club: usize) -> bool {
    let e = read_usize(player + staff::SECOND_NATION);
    if e == 0 || read_usize(e) != read_usize(KEY_NATION_PAYS_BASQUE) {
        return false;
    }
    let id = read_usize(club);
    ![0x9d0120usize, 0x9d0134, G_KEY_CLUB_REAL_SOCIEDAD_C_F, G_KEY_CLUB_REAL_SOCIEDAD_C_F_B, 0x9d0124].iter().any(|&g| read_usize(g) == id)
}

/// Can the club bid for the player (`can_club_bid_for_player` of the transfer manager, not ignoring open offers)?
///
/// [pt] O clube pode chegar ao jogador (0x8ce990 de [0xde1f54], sem ignorar as ofertas abertas)? O motivo fica numa local
/// (a original o grava na vaga do argumento, que ninguém lê depois).
pub(crate) unsafe fn can_club_bid(player: usize, club: usize) -> bool {
    let mut reason = 0i16;
    exe::transfer_manager_can_club_bid_for_player(read_usize(G_TRANSFER_MANAGER), player, club, (&mut reason) as *mut i16 as *mut u16, 0) != 0
}

/// Picks the best-scored club (`player_regen_score_club_for_player`) among those where the youngster is not a foreigner, that are not a B team,
/// that may approach him and with a squad under 50.
///
/// `PLAYER_REGEN::pick_best_club_for_player(player)`: among the clubs ([0xae23b4], 0x245 bytes; the first [0xae235c] −
/// 2 × [0xae2350], re-read on every loop) where the player is not a foreigner (`database_is_staff_foreign` with the nation +0x53 and 1),
/// that are not the B team of a pair (`exe::club_get_linked_b_team` with 1, finding the partner with 0), not vetoed
/// (`is_basque_restricted`), that may approach him (`can_club_bid_for_player`) and with a squad (`database_count_club_squad`) under 50, the one with the
/// highest score (`player_regen_score_club_for_player`, random; compared in 16 bits, starting from −25000). None: 0. No SEH; the partner local is
/// only read when `exe::club_get_linked_b_team` finds it (and writes it).
/// Original: `PLAYER_REGEN::pick_best_club_for_player` (0x7ad0e0, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ad0e0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_best_club_for_player(jogador)`: entre os
/// clubes ([0xae23b4], 0x245 bytes; os [0xae235c] − 2 × [0xae2350] primeiros, relidos a cada volta) em que o jogador
/// não é estrangeiro (0x53d980 com a nação +0x53 e 1), que não são o time B de um par (0x540a50 com 1 achando o
/// parceiro com 0), não vetados (`is_basque_restricted`), que podem chegar a ele (0x8ce990) e com elenco (0x53c590) abaixo de 50,
/// o de maior nota (0x7ad220, sorteia; comparada em 16 bits, a partir de −25000). Nenhum: 0. Sem SEH; a local do
/// parceiro só é lida quando a 0x540a50 o acha (e a grava).
pub(crate) unsafe extern "thiscall" fn pick_best_club_for_player(_this: usize, player: usize) -> usize {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let total = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    let mut eh_a = 0u32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < total() {
        let candidate = estrangeiro(player, read_usize(club + club::NATION), 1) == 0
            && (exe::club_get_linked_b_team(club, &mut eh_a, 1) == 0 || eh_a != 0)
            && !is_basque_restricted(player, club);
        if candidate {
            let squad = exe::database_count_club_squad(club, 0, 0);
            if can_club_bid(player, club) && (squad as i8) < 0x32 {
                // simplification: the original pushes the squad byte with garbage in the high bits; 0x7ad220 reads only the byte
                let s = exe::player_regen_score_club_for_player(player, club, squad as u32) as i16;
                if s > melhor_nota {
                    crate::comparison::branch("cpn_4 better club for the youngster");
                    melhor = club;
                    melhor_nota = s;
                }
            }
        }
        i += 1;
        club += 0x245;
    }
    melhor
}

/// Nation of the youth's list: his nation, with Wales and Ireland mapped to England and Northern Ireland too with
/// random(5) < 2.
///
/// [pt] A nação da lista do jovem: a nação (+0x1a) dele; [0x9cf510] e [0x9cf2dc] vão para [0x9cf2e4], e [0x9cf3e4] também
/// com exe::utils_random(5) < 2.
pub(crate) unsafe fn player_regen_youth_list_nation(player: usize) -> usize {
    let n = read_usize(read_usize(player + staff::NATION));
    if n == read_usize(KEY_NATION_WALES) || n == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) || (n == read_usize(KEY_NATION_NORTHERN_IRELAND) && exe::utils_random(5) < 2) {
        return read_usize(KEY_NATION_ENGLAND);
    }
    n
}

/// Club list of nation n: [[this] + 4n].
///
/// [pt] A lista de clubes da nação `n`: [[this] + 4n].
pub(crate) unsafe fn nation_club_list(this: usize, n: usize) -> usize {
    read_usize(read_usize(this).wrapping_add(n.wrapping_mul(4)))
}

/// Picks, from the club list of the youngster's nation, the club with the highest random draw of the score by
/// reputation against ability, among those that may approach him and with a squad under 50.
///
/// `PLAYER_REGEN::pick_club_from_nation_list(player)`: walks the club list of the youngster's nation
/// (`player_regen_youth_list_nation`, some nations mapped to [0x9cf2e4]; word +0 is the count, cursor +0xa restarts
/// from +2, the iterator `next_list_item` returns the id) and, among the clubs not vetoed (`is_basque_restricted`), that may
/// approach him (`can_club_bid_for_player`) and with a squad (`database_count_club_squad`) under 50, keeps the one with the highest random draw of the
/// score (5000 + reputation +0x80 minus its distances to the ability of the player record +0x61: +0xb, +7 × 50 and +5 ×
/// 50, in 16 bits), compared in 16 bits starting from −25000. No list or an empty list: 0. A list that ends before the
/// count: the "Error" box (line 0x19e9) and 0. No SEH.
/// Original: `PLAYER_REGEN::pick_club_from_nation_list` (0x7ad590, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ad590 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_club_from_nation_list(jogador)`: percorre a
/// lista de clubes da nação do jovem (`player_regen_youth_list_nation`; a palavra +0 é a conta, o cursor +0xa recomeça de +2, o
/// iterador 0x68d1b0 devolve o id) e, entre os clubes não vetados (`is_basque_restricted`), que podem chegar a ele (0x8ce990) e
/// com elenco (0x53c590) abaixo de 50, fica com o de maior sorteio da nota (5000 + reputação +0x80 menos as distâncias
/// dela à habilidade da ficha +0x61: +0xb, +7 × 50 e +5 × 50, em 16 bits), comparado em 16 bits a partir de −25000.
/// Sem lista ou lista vazia: 0. A lista que acaba antes da conta: a caixa "Error" (linha 0x19e9) e 0. Sem SEH.
pub(crate) unsafe extern "thiscall" fn pick_club_from_nation_list(this: usize, player: usize) -> usize {
    let n = player_regen_youth_list_nation(player);
    let l = nation_club_list(this, n);
    if l == 0 || read_i16(l) == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    if read_i16(nation_club_list(this, n)) <= 0 {
        return 0;
    }
    let mut i = 0i32;
    loop {
        let item = exe::llist_get_next(nation_club_list(this, n));
        if item == 0 {
            show_error_at(0xa6bf9c, 0x19e9);
            return 0;
        }
        let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(item).wrapping_mul(club::SIZE));
        if !is_basque_restricted(player, club) && can_club_bid(player, club) && (exe::database_count_club_squad(club, 0, 0) as i8) < 0x32 {
            let sheet = read_usize(player + staff::PLAYER);
            let rep = read_i16(club + club::REPUTATION) as i32;
            let c = rep
                .wrapping_sub((read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((read_i16(sheet + player_sheet::POTENTIAL_ABILITY) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((read_i16(sheet + player_sheet::CURRENT_ABILITY) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_add(0x1388);
            crate::comparison::branch("cpn_4 draw of the nation club");
            let rating = exe::utils_random(c as i16 as i32) as i16;
            if rating > melhor_nota {
                melhor = club;
                melhor_nota = rating;
            }
        }
        i += 1;
        if i as i16 >= read_i16(nation_club_list(this, n)) {
            break;
        }
    }
    melhor
}

/// Calls the game's CRT realloc.
pub(crate) unsafe fn crt_realloc(p: usize, size: usize) -> usize {
    exe::crt_realloc(p, size as u32)
}

/// Out-of-memory fatal error for the regen lists (player_regen.cpp at the given line): message box and exit like the
/// original.
///
/// [pt] A falta de memória das listas do regen (player_regen.cpp, linha `linha`): caixa e fim, como a original.
pub(crate) unsafe fn regen_out_of_memory(row: u32) -> ! {
    show_error_at(0xa6bf9c, row);
    fatal_exit()
}

/// Appends (id, nation, second nation) to the 16-byte spare non-player record list (+0x1c); grows by 250 when the
/// player pool (+0x40/+0x44) is full, as in the original.
///
/// `PLAYER_REGEN::add_spare_non_player_record(id, nation, second)`: appends (id +0, nation +4, second nation +8) to the
/// 16-byte list of spare non-player records ([this+0x1c], count +0x20, capacity +0x24; taken out by `remove_spare_non_player_record`). The
/// original decides to grow by the free player pool (count +0x40 = capacity +0x44 − 1), not by this list: kept
/// faithfully. Grows by 250 (realloc `exe::crt_realloc`; out of memory, box 0x9c4 and end).
/// Original: `PLAYER_REGEN::add_spare_non_player_record` (0x7a81a0, player_regen.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7a81a0 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_spare_non_player_record(id, nação,
/// segunda)`: acrescenta (id +0, nação +4, segunda nação +8) à lista de 16 bytes das reservas de não jogador
/// ([this+0x1c], conta +0x20, capacidade +0x24; tirada pela 0x7a82b0). O original decide crescer pelo banco de
/// jogadores livres (conta +0x40 = capacidade +0x44 − 1), não por esta lista: fiel. Cresce 250 (realloc 0x945667;
/// sem memória, caixa 0x9c4 e fim).
pub(crate) unsafe extern "thiscall" fn add_spare_non_player_record(this: usize, id: u32, nation: u32, second: u32) {
    crate::comparison::branch("cpk_2 manager reservation");
    if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS), read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250) << 4);
        write_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS, p);
        write_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0x9c4);
        }
    }
    let n = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT);
    let item = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS).wrapping_add(n << 4);
    write_usize(item, id as usize);
    write_usize(item + regen_pool_item::NATION_ID, nation as usize);
    write_usize(item + regen_pool_item::SECOND_NATION_ID, second as usize);
    write_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT, n.wrapping_add(1)); // one more reserve
}

/// Appends the record fields to the 26-byte pending retirement table (+4), growing by 250.
///
/// `PLAYER_REGEN::add_retirement(record)`: appends to the 26-byte table of pending retirements ([this+4], count +8,
/// capacity +0xc) the record's fields +0, +4 (dwords), +0x18 (byte), +8, +0xa, +0x10, +0x12 (words) and +0x19 (byte),
/// in that order (+0xc..+0x10 and +0x14..+0x18 are left as they are). Count = capacity − 1: grows by 250 (realloc
/// `exe::crt_realloc`; out of memory, box 0xa53 and end).
/// Original: `PLAYER_REGEN::add_retirement` (0x7a8510, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a8510 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::add_retirement(registro)`: acrescenta à tabela de
/// 26 bytes das aposentadorias pendentes ([this+4], conta +8, capacidade +0xc) os campos +0, +4 (dwords), +0x18
/// (byte), +8, +0xa, +0x10, +0x12 (palavras) e +0x19 (byte) do registro, nessa ordem (+0xc..+0x10 e +0x14..+0x18
/// ficam como estão). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória, caixa 0xa53 e fim).
pub(crate) unsafe extern "thiscall" fn add_retirement(this: usize, r: usize) {
    crate::comparison::branch("cpk_2 retirement");
    let cap = read_usize(this + player_regen_fields::RETIREMENT_CAPACITY);
    if read_usize(this + player_regen_fields::RETIREMENT_COUNT) == cap.wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::RETIREMENTS), cap.wrapping_add(250).wrapping_mul(26));
        write_usize(this + player_regen_fields::RETIREMENTS, p);
        write_usize(this + player_regen_fields::RETIREMENT_CAPACITY, read_usize(this + player_regen_fields::RETIREMENT_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa53);
        }
    }
    let item = || read_usize(this + player_regen_fields::RETIREMENTS).wrapping_add(read_usize(this + player_regen_fields::RETIREMENT_COUNT).wrapping_mul(26));
    write_usize(item(), read_usize(r));
    write_usize(item() + 4, read_usize(r + retirement_entry::CLUB_ID));
    *((item() + 0x18) as *mut u8) = *((r + retirement_entry::REASON) as *const u8);
    for o in [8usize, 0xa, 0x10, 0x12] {
        write_u16(item() + o, core::ptr::read_unaligned((r + o) as *const u16));
    }
    *((item() + 0x19) as *mut u8) = *((r + retirement_entry::ANNOUNCED) as *const u8);
    write_usize(this + player_regen_fields::RETIREMENT_COUNT, read_usize(this + player_regen_fields::RETIREMENT_COUNT).wrapping_add(1)); // one more retirement
}

/// Puts the person into the regen queue (+0x10) with the dates; the non-player data goes into the spare list (+0x1c,
/// random nation if none) and the player data into the free pool (+0x3c).
///
/// `PLAYER_REGEN::queue_staff_for_regen(entry)`: the person with id +0 of the entry (0x6e-byte record in [0xae23bc])
/// enters the regen queue (12 bytes at [this+0x10], count +0x14, capacity +0x18: id +0, words +4 and +6 of the entry).
/// With non-player data (+0x69): in the 16-byte list [this+0x1c] (count +0x20, capacity +0x24) their id +0, the nation
/// (+0x1a; without a nation, a random draw `exe::utils_random` among the [0xae2350]) +4 and the person's id +0xc. With player data
/// (+0x61): in the free player pool [this+0x3c] (count +0x40, capacity +0x44) their id +0, the nation's +4 (without
/// checking for null, as in the original) and the person's +0xc. Each list grows by 250 when count = capacity − 1
/// (realloc `exe::crt_realloc`; boxes 0xaac, 0xac5, 0xae9 and end; in the third the original checks the count, not the pointer:
/// kept faithfully). Id outside 0..=[0xae2364]: as in the original (box 0xa9d).
/// Original: `PLAYER_REGEN::queue_staff_for_regen` (0x7a87d0, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a87d0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::queue_staff_for_regen(entrada)`: a pessoa de id
/// +0 da entrada (registro de 0x6e bytes em [0xae23bc]) entra na fila do regen (12 bytes em [this+0x10], conta
/// +0x14, capacidade +0x18: id +0, palavras +4 e +6 da entrada). Com dados de não jogador (+0x69): na lista de 16
/// bytes [this+0x1c] (conta +0x20, capacidade +0x24) o id deles +0, a nação (+0x1a; sem nação, um sorteio
/// 0x90cfc0 entre as [0xae2350]) +4 e o id da pessoa +0xc. Com dados de jogador (+0x61): no banco de jogadores
/// livres [this+0x3c] (conta +0x40, capacidade +0x44) o id deles +0, o da nação +4 (sem conferir nula, como a
/// original) e o da pessoa +0xc. Cada lista cresce 250 quando conta = capacidade − 1 (realloc 0x945667; caixas 0xaac,
/// 0xac5, 0xae9 e fim; na terceira o original confere a conta, não o ponteiro: fiel). Id fora de 0..=[0xae2364]: a
/// original (caixa 0xa9d).
pub(crate) unsafe extern "thiscall" fn queue_staff_for_regen(this: usize, entry: usize) {
    let id = read_usize(entry) as i32;
    if id < 0 || id > *(G_STAFF_COUNT as *const i32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_QUEUE_STAFF_FOR_REGEN)(this, entry);
    }
    crate::comparison::branch("cpk_2 slot for the regen");
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    let cap = read_usize(this + player_regen_fields::REGEN_QUEUE_CAPACITY);
    if read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT) == cap.wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::REGEN_QUEUE), cap.wrapping_add(250).wrapping_mul(12));
        write_usize(this + player_regen_fields::REGEN_QUEUE, p);
        write_usize(this + player_regen_fields::REGEN_QUEUE_CAPACITY, read_usize(this + player_regen_fields::REGEN_QUEUE_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xaac);
        }
    }
    let queue = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add(read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT).wrapping_mul(12));
    write_usize(queue, id as usize);
    write_u16(queue + regen_queue_entry::REGEN_DATE, core::ptr::read_unaligned((entry + 4) as *const u16));
    write_u16(queue + regen_queue_entry::REGEN_YEAR, core::ptr::read_unaligned((entry + 6) as *const u16));
    write_usize(this + player_regen_fields::REGEN_QUEUE_COUNT, read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT).wrapping_add(1)); // one more in the regen queue
    if read_usize(person + staff::NON_PLAYER) != 0 {
        if read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) == read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_sub(1) {
            let p = crt_realloc(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS), read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250) << 4);
            write_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS, p);
            write_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250));
            if p == 0 {
                regen_out_of_memory(0xac5);
            }
        }
        let item = || read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS).wrapping_add(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) << 4);
        write_usize(item(), read_usize(read_usize(person + staff::NON_PLAYER)));
        let nation = read_usize(person + staff::NATION);
        if nation != 0 {
            write_usize(item() + 4, read_usize(nation));
        } else {
            let s = exe::utils_random(*(G_NATION_COUNT as *const i32));
            write_usize(item() + 4, s as usize);
        }
        write_usize(item() + 0xc, read_usize(person));
        write_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT).wrapping_add(1));
    }
    if read_usize(person + staff::PLAYER) != 0 {
        let cap = read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY);
        if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == cap.wrapping_sub(1) {
            let p = crt_realloc(read_usize(this + player_regen_fields::PLAYER_DATA_POOL), cap.wrapping_add(250) << 4);
            write_usize(this + player_regen_fields::PLAYER_DATA_POOL, p);
            write_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_add(250));
            if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == 0 {
                regen_out_of_memory(0xae9); // the original checks the count (+0x40), not the pointer
            }
        }
        let item = || read_usize(this + player_regen_fields::PLAYER_DATA_POOL).wrapping_add(read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) << 4);
        write_usize(item(), read_usize(read_usize(person + staff::PLAYER)));
        write_usize(item() + 4, read_usize(read_usize(person + staff::NATION)));
        write_usize(item() + 0xc, read_usize(person));
        write_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT).wrapping_add(1));
    }
}

/// The 26 bytes `add_retirement` copies from a retirement record (others zeroed).
///
/// [pt] Os 26 bytes que a 0x7a8510 copia do registro (os outros, zerados).
pub(crate) unsafe fn retirement_entry_fields(r: usize) -> [u8; 26] {
    let mut c = [0u8; 26];
    for (o, n) in [(0usize, 4usize), (4, 4), (8, 2), (0xa, 2), (0x10, 2), (0x12, 2), (0x18, 1), (0x19, 1)] {
        c[o..o + n].copy_from_slice(core::slice::from_raw_parts((r + o) as *const u8, n));
    }
    c
}

/// Removes a pending retirement: finds it (26 bytes), pulls the next ones up (`retire_record_assign`), blanks the last one (person
/// −1) and lowers the count.
///
/// `PLAYER_REGEN::remove_retirement(key)`: searches (bsearch `exe::crt_bsearch` with `player_regen_compare_retirements`) for the pending retirement in the
/// vector ([this+4], [this+8] records of 26 bytes); found: pulls the following ones up one place (`retire_record_assign`, the record
/// copy), the last one gets person −1 and +0x18, +8, +0xa, +0x10, +0x12 zeroed, and the count goes down. Without
/// re-sorting.
/// Original: `PLAYER_REGEN::remove_retirement` (0x7a8690, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a8690 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::remove_retirement(chave)`: busca (bsearch 0x946464
/// com 0x7ae2a0) a aposentadoria pendente no vetor ([this+4], [this+8] registros de 26 bytes); achou: puxa as
/// seguintes um lugar (0x7a8780, a cópia do registro), a última fica com pessoa −1 e +0x18, +8, +0xa, +0x10, +0x12
/// zerados, e a contagem desce. Sem reordenar.
pub(crate) unsafe extern "thiscall" fn remove_retirement(this: usize, key: usize) {
    let array = read_u32(this + player_regen_fields::RETIREMENTS) as usize;
    let s = exe::crt_bsearch(key, array, read_u32(this + player_regen_fields::RETIREMENT_COUNT), 0x1a, 0x7ae2a0);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(array) as u32 as u64 * 0x4ec4ec4f) >> 35) as i32; // o lugar (/ 26)
    while k < read_u32(this + player_regen_fields::RETIREMENT_COUNT) as i32 - 1 {
        let r = (read_u32(this + player_regen_fields::RETIREMENTS) as usize).wrapping_add(k as usize * 26);
        exe::retire_record_assign(r, r + retirement_entry::SIZE);
        k += 1;
    }
    let end = (read_u32(this + player_regen_fields::RETIREMENTS) as usize).wrapping_add((read_u32(this + player_regen_fields::RETIREMENT_COUNT) as usize).wrapping_mul(26)).wrapping_sub(26);
    core::ptr::write_unaligned(end as *mut u32, u32::MAX); // the last one without a person
    *((end + retirement_entry::REASON) as *mut u8) = 0;
    for o in [8usize, 0xa, 0x10, 0x12] {
        core::ptr::write_unaligned((end + o) as *mut u16, 0);
    }
    core::ptr::write_unaligned((this + player_regen_fields::RETIREMENT_COUNT) as *mut u32, read_u32(this + player_regen_fields::RETIREMENT_COUNT).wrapping_sub(1));
}

/// Is the person on the shortlist (`is_on_shortlist` on the shortlist manager [[0xde1f54]+0x856], no club)?
///
/// [pt] Está na lista de observados (0x842080 no gerente da lista [[0xde1f54]+0x856], sem clube)?
pub(crate) unsafe fn player_regen_is_on_shortlist(person: usize) -> bool {
    exe::shortlist_manager_is_on_shortlist(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize, person, 0) != 0
}

/// Decides whether an unemployed person retires: a weak manager without a club, a player unemployed for over a year, or
/// anyone with no record; fills the entry (reason 0x10, the game date) and returns 1.
///
/// `player_regen_check_unemployed_retirement(person, out)` (stdcall, ret 8): whoever is on the shortlist does not
/// retire. With a manager record (+0x69): without a club (+0x39) and without +0x24, with +0xa of the record below 50
/// and random(5) = 0. Without it, with a player record (+0x61): off the list (checked again), without a club and
/// without +0x24, unemployed for more than 365 days (365 × (year − year of +0x40) − day of +0x3e + day) and random(the
/// player's rating +0xb) < 750. With neither record: always. Retires: the output (26 bytes) receives the person, reason
/// 0x10, +0x19 = 0, the club (−1 if none) and the game date at +8 and +0x10; returns 1. Otherwise 0.
/// Original: `player_regen_check_unemployed_retirement` (0x7a8050, player_regen.cpp, stdcall, ret 8).
///
/// [pt] { 0x7a8050 } (stdcall, ret 8, player_regen.cpp): `player_regen_check_unemployed_retirement(pessoa, saida)`: quem
/// está na lista de observados não se aposenta. Com registro de técnico (+0x69): sem clube (+0x39) e sem +0x24, com
/// +0xa do registro abaixo de 50 e sorteio(5) = 0. Sem ele, com registro de jogador (+0x61): fora da lista (conferida
/// de novo), sem clube e sem +0x24, desempregado há mais de 365 dias (365 × (ano − ano de +0x40) − dia de +0x3e + dia)
/// e sorteio(nota +0xb do jogador) < 750. Sem nenhum dos dois registros: sempre. Aposenta: a saída (26 bytes) recebe a
/// pessoa, o motivo 0x10, +0x19 = 0, o clube (−1 sem) e a data do jogo em +8 e +0x10; devolve 1. Senão 0.
pub(crate) unsafe extern "stdcall" fn player_regen_check_unemployed_retirement(person: usize, out: usize) -> u8 {
    let mut without_record = false;
    if player_regen_is_on_shortlist(person) {
        return 0;
    }
    let manager = read_u32(person + staff::NON_PLAYER) as usize;
    if manager == 0 && read_u32(person + staff::PLAYER) == 0 {
        without_record = true;
    }
    let aposenta = if manager != 0 {
        if read_u32(person + staff::CLUB_JOB) != 0 || read_u32(person + staff::NATIONAL_JOB) != 0 {
            return 0;
        }
        *((manager + non_player::CURRENT_REPUTATION) as *const i16) < 50 && friendly_random(5) == 0 // the weak manager
    } else if read_u32(person + staff::PLAYER) == 0 {
        false
    } else {
        if player_regen_is_on_shortlist(person) {
            return 0;
        }
        if read_u32(person + staff::CLUB_JOB) != 0 || read_u32(person + staff::NATIONAL_JOB) != 0 {
            false
        } else {
            let years = (*(G_CURRENT_DATE_YEAR as *const u16) as i32).wrapping_sub(*((person + 0x40) as *const u16) as i32);
            let days = years.wrapping_mul(365).wrapping_sub(*((person + staff::DATE_JOINED_CLUB) as *const i16) as i32).wrapping_add(*(G_CURRENT_DATE as *const i16) as i32);
            if days > 0x16d {
                let rating = *((read_u32(person + staff::PLAYER) as usize + 0xb) as *const i16) as i32;
                friendly_random(rating) < 0x2ee // the roll by the rating
            } else {
                false
            }
        }
    };
    if !aposenta && !without_record {
        return 0;
    }
    core::ptr::write_unaligned(out as *mut u32, read_u32(person));
    *((out + retirement_entry::REASON) as *mut u8) = 0x10; // o motivo: desempregado
    *((out + retirement_entry::ANNOUNCED) as *mut u8) = 0;
    let club = read_u32(person + staff::CLUB_JOB) as usize;
    core::ptr::write_unaligned((out + retirement_entry::CLUB_ID) as *mut u32, if club != 0 { read_u32(club) } else { u32::MAX });
    for o in [8usize, 0x10] {
        core::ptr::write_unaligned((out + o) as *mut u16, *(G_CURRENT_DATE as *const u16));
        core::ptr::write_unaligned((out + o + 2) as *mut u16, *(G_CURRENT_DATE_YEAR as *const u16));
    }
    1
}

/// copies the regen queue entry (0xc bytes): the person (+0) and the regen date (+4)
/// Original: `regen_queue_entry_assign` (0x7a8b70, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a8b70 } (thiscall, ret 4, player_regen.cpp): `regen_queue_entry_assign`: copia a entrada da fila de regens
/// (0xc bytes) de `origem`: a pessoa (+0) e a data do regen em +4 (0x417c10, retorno numa local). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn regen_queue_entry_assign(this: usize, source: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(source as *const u32)); // the regen’s person
    let mut local = [0u8; 8];
    exe::date_operator_3d(this + regen_queue_entry::REGEN_DATE, (&mut local) as *mut [u8; 8] as usize, source + regen_queue_entry::REGEN_DATE);
    this
}

/// Picks the non-player with the highest positive score (`player_regen_score_non_player_candidate`) of the nation; with none, of any nation; otherwise
/// 0.
///
/// `player_regen_pick_staff_for_non_player_data(nation)`: among the non-player persons ([0xae23bc], 0x6e bytes,
/// [0xae2364] − 16 of them, count re-read on every loop), the one with the highest positive score from `player_regen_score_non_player_candidate`
/// (random; the incoming ecx goes along, as in the original) of nation `nation` ([+0x1a]+0 = nation; initial score
/// −1000); with none, the one with the highest positive score of any nation (initial −15000); otherwise 0.
/// Original: `player_regen_pick_staff_for_non_player_data` (0x7a6e60, player_regen.cpp, stdcall, ret 4).
///
/// [pt] { 0x7a6e60 } (thiscall, ret 4, player_regen.cpp): `player_regen_pick_staff_for_non_player_data(nacao)`: entre as
/// pessoas não jogadoras ([0xae23bc], 0x6e bytes, [0xae2364] − 16 delas, contagem relida a cada volta), a de maior
/// nota positiva da 0x7a7150 (sorteia; o ecx de entrada vai junto, como na original) da nação `nacao` ([+0x1a]+0 =
/// nacao; nota inicial −1000); sem nenhuma, a de maior nota positiva de qualquer nação (inicial −15000); senão 0.
pub(crate) unsafe extern "thiscall" fn player_regen_pick_staff_for_non_player_data(this: usize, nation: usize) -> usize {
    let how_many = || (*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10);
    let base = read_usize(G_STAFF_TABLE);
    let (mut melhor, mut escolhida) = (-1000i32, 0usize);
    let mut i = 0;
    while i < how_many() {
        let p = base + 0x6e * i as usize;
        let n = read_usize(p + staff::NATION);
        if n != 0 && read_usize(n) == nation {
            let s = exe::player_regen_score_non_player_candidate(this, p);
            if s > melhor && s > 0 { // the nation’s best
                melhor = s;
                escolhida = p;
            }
        }
        i += 1;
    }
    if escolhida != 0 {
        return escolhida;
    }
    crate::comparison::branch("donor from any nation");
    melhor = -15000;
    let base = read_usize(G_STAFF_TABLE); // reread, as in the original
    i = 0;
    while i < how_many() {
        let p = base + 0x6e * i as usize;
        let s = exe::player_regen_score_non_player_candidate(this, p);
        if s > melhor && s > 0 {
            melhor = s;
            escolhida = p;
        }
        i += 1;
    }
    escolhida
}

compare_value!(compare_is_on_shortlist, is_on_shortlist, IDX_IS_ON_SHORTLIST, (jogador: usize, clube: usize) -> u32);

/// Removes the 16-byte record with key id from a regen list ([regen+lista], count [regen+quantos]), shifts the rest,
/// marks the last with -1 and decrements the count.
///
/// [pt] O comum das duas: tira da lista de registros de 16 bytes ([regen+`lista`], quantos em
/// [regen+`quantos`]) o de chave `id` (+0), puxa os seguintes uma vaga para trás, marca o último com −1
/// em +0, +4 e +8 e diminui a conta.
pub(crate) unsafe fn remove_from_list16(regen: usize, id: i32, list: usize, how_many: usize) {
    let n = || *((regen + how_many) as *const i32);
    let base = || *((regen + list) as *const usize);
    let Some(i) = (0..n().max(0) as usize).find(|&i| *((base() + i * 16) as *const i32) == id) else {
        return;
    };
    let mut j = i;
    while (j as i32) < n() - 1 {
        core::ptr::copy((base() + (j + 1) * 16) as *const u8, (base() + j * 16) as *mut u8, 16);
        j += 1;
    }
    let last = (base() as isize + (n() as isize - 1) * 16) as usize;
    for o in [0, 4, 8] {
        *((last + o) as *mut i32) = -1;
    }
    *((regen + how_many) as *mut i32) -= 1;
}

/// removes a non-player record from the spare list by id
/// Original: `PLAYER_REGEN::remove_spare_non_player_record` (0x7a82b0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a82b0 } (thiscall, ret 4, player_regen.cpp): tira o registro de reserva de não jogador (técnicos
/// e afins) de id `id` (lista +0x1c, quantos +0x20).
pub(crate) unsafe extern "thiscall" fn remove_spare_non_player_record(regen: usize, id: i32) {
    remove_from_list16(regen, id, 0x1c, 0x20);
}

/// removes the player data record from the free player pool
/// Original: `PLAYER_REGEN::remove_player_data_from_pool` (0x7a8470, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a8470 } (thiscall, ret 4, player_regen.cpp): tira do banco de jogadores livres o registro dos
/// dados de jogador `id` (lista +0x3c, quantos +0x40).
pub(crate) unsafe extern "thiscall" fn remove_player_data_from_pool(regen: usize, id: i32) {
    remove_from_list16(regen, id, 0x3c, 0x40);
}

/// Regions of a 16-byte regen list: the whole list and its counter.
///
/// [pt] As regiões das duas listas: a lista inteira e o contador.
pub(crate) unsafe fn list16_regions(regen: usize, list: usize, how_many: usize) -> Vec<(usize, usize)> {
    let n = (*((regen + how_many) as *const i32)).max(0) as usize;
    let mut r = vec![(regen + how_many, 4)];
    if n > 0 {
        r.push((*((regen + list) as *const usize), n * 16));
    }
    r
}

/// the person's entry in the retirement list (by injury or already announced)
/// Original: `PLAYER_REGEN::find_retirement_entry` (0x7a7ed0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a7ed0 } (thiscall, ret 8, player_regen.cpp): a entrada da pessoa `*id` na lista das
/// aposentadorias ([+4], 0x1a bytes cada, [+8] quantas): com `por_lesao`, só se o motivo (+0x18) for 8;
/// senão só se já anunciada (+0x19). Fora isso, 0.
pub(crate) unsafe extern "thiscall" fn find_retirement_entry(regen: usize, id: *const i32, by_injury: u32) -> usize {
    let list = *((regen + player_regen_fields::RETIREMENTS) as *const usize);
    let n = *((regen + player_regen_fields::RETIREMENT_COUNT) as *const i32);
    for i in 0..n.max(0) as usize {
        let e = list + i * 0x1a;
        if core::ptr::read_unaligned(e as *const i32) == *id {
            let ok = if by_injury != 0 { *((e + retirement_entry::REASON) as *const u8) == 8 } else { *((e + retirement_entry::ANNOUNCED) as *const u8) != 0 };
            return if ok { e } else { 0 };
        }
    }
    0
}

/// Returns the random score of how well the club suits the youth player: reputation against ability, youth facilities,
/// human control and nation.
///
/// The score of `club` for the young `player` (random(score)): 5000 + reputation (+0x80) minus its distances to the
/// ability (player record +0x61: +0xb, and +7, +5 × 50); with +7 above random(20) + 100, the youth bonus (+0x7f, 20 =
/// guaranteed, otherwise random(+0x7f) > random(20)); a club of an active competition (0xadadfc) with +7 > 50: the
/// human club state (0xae2d14) and the reputation against the squad (`squad`); the youth coach (+0xcf → +0x69) of a
/// computer club (`exe::human_manager_is_human_club`) or the human's bonus; the same nation (+0x1a/+0x53) and the Welsh clubs (Swansea, Cardiff,
/// Wrexham) and Scotland. Only the low 16 bits of the score count.
/// Original: `player_regen_score_club_for_player` (0x7ad220, player_regen.cpp, stdcall).
///
/// [pt] { 0x7ad220 } (stdcall, ret 0xc, player_regen.cpp): a nota do `clube` para o jovem `jogador`
/// (exe::utils_random(nota)): 5000 + reputação (+0x80) menos as distâncias dela à habilidade (ficha +0x61: +0xb,
/// e +7, +5 × 50); com +7 acima de exe::utils_random(20) + 100, o bônus de juniores (+0x7f, 20 = garantido,
/// senão exe::utils_random(+0x7f) > exe::utils_random(20)); clube de competição ativa (0xadadfc) com +7 > 50: o estado do
/// clube humano (0xae2d14) e a reputação contra o elenco (`elenco`); o treinador de juniores (+0xcf
/// → +0x69) do clube do computador (0x600a50) ou o bônus do humano; a mesma nação (+0x1a/+0x53) e os
/// clubes galeses (Swansea, Cardiff, Wrexham) e a Escócia. Só os 16 bits baixos da nota contam.
pub(crate) unsafe extern "stdcall" fn player_regen_score_club_for_player(player: usize, club: usize, squad: i8) -> i32 {
    let sheet = read_usize(player + staff::PLAYER);
    let rep = read_i16(club + club::REPUTATION) as i32;
    let mut s = rep
        .wrapping_sub((read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32 - rep).wrapping_abs())
        .wrapping_sub((read_i16(sheet + player_sheet::POTENTIAL_ABILITY).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_sub((read_i16(sheet + player_sheet::CURRENT_ABILITY).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_add(0x1388);
    let juniores = *((club + club::TRAINING) as *const i8);
    if read_i16(sheet + player_sheet::POTENTIAL_ABILITY) as i32 > exe::utils_random(0x14) + 0x64 {
        if juniores == 0x14 {
            s = s.wrapping_add(rep / 2 + 0x1b58);
        } else if exe::utils_random(juniores as i32) > exe::utils_random(0x14) {
            s = s.wrapping_add(rep / 2 + (juniores as i32 + 0x64) * 50);
        }
    }
    let comp = read_usize(club + club::DIVISION);
    if read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0x32 && comp != 0 && *((*(G_COMP_OBJECT_TABLE as *const usize) + 4 * *(comp as *const u32) as usize) as *const usize) != 0 {
        let estados = *(G_CLUB_STATE_TABLE as *const usize);
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && estados != 0 && read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0x78 {
            let e = *((estados + 6 * *(club as *const u32) as usize + 4) as *const i8);
            if e < 0 {
                s = s.wrapping_add(0x1f4);
            } else if e == 0 {
                s = s.wrapping_add(0xfa);
            }
        }
        let r = read_i16(club + club::REPUTATION);
        s = s.wrapping_add(if r > 0x1676 {
            if squad < 0x28 { 0x3e8 } else if squad < 0x2d { 0x2ee } else { 0xfa }
        } else if r > 0x109a {
            if squad < 0x23 { 0x2ee } else if squad < 0x28 { 0x1f4 } else { 0xfa }
        } else {
            0xfa
        });
    }
    let manager = read_usize(club + club::MANAGER);
    let x = if manager != 0 { read_usize(manager + staff::NON_PLAYER) } else { 0 };
    if x != 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
        s = s.wrapping_add((*((x + non_player::YOUNGSTERS) as *const i8) as i32 - 10) * 30);
        if read_i16(sheet + player_sheet::POTENTIAL_ABILITY) as i32 > exe::utils_random(0x4b) + 0x64 {
            let v = exe::non_player_attribute_rating(x, *((x + non_player::JUDGING_POTENTIAL) as *const i8)) as i8;
            s = s.wrapping_add((v as i32 - 10) * 60);
        }
        let v = exe::non_player_attribute_rating(x, *((x + non_player::JUDGING_POTENTIAL) as *const i8)) as i8;
        let t = v as i32 + *((x + non_player::YOUNGSTERS) as *const i8) as i32 + juniores as i32;
        if t > 0x28 && read_i16(club + club::REPUTATION) > 0x109a && read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0x50 {
            if (s as i16) < read_i16(club + club::REPUTATION) {
                s = read_i16(club + club::REPUTATION) as i32;
            } else {
                s = s.wrapping_add(0xfa);
            }
        }
    } else if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        s = s.wrapping_add(0xc8);
        if read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0x78 {
            if juniores == 0x14 && (s as i16) < read_i16(club + club::REPUTATION) {
                s = read_i16(club + club::REPUTATION) as i32;
            }
            s = s.wrapping_add(juniores as i32 * 25);
        }
    }
    let nation = read_usize(player + staff::NATION);
    let nation_of_club = read_usize(club + club::NATION);
    if nation == nation_of_club {
        s = s.wrapping_add(0x3e8); // same nation
    }
    let g = |a: usize| *(a as *const u32);
    let (gales, inglaterra, escocia) = (g(KEY_NATION_WALES), g(KEY_NATION_ENGLAND), g(KEY_NATION_SCOTLAND));
    let id = g(club);
    if id == g(0x9d0430) || id == g(0x9d0434) || (id == g(0x9d043c) && nation != 0) {
        let n = g(nation);
        if n == gales {
            s = s.wrapping_add(0x3e8);
        } else if n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        }
    }
    if g(nation_of_club) == escocia {
        let n = g(nation);
        if n == gales || n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        } else if n == escocia {
            s = s.wrapping_add(0x3e8);
        }
    }
    exe::utils_random(s as i16 as i32)
}

/// Copies a pending retirement record: the person, the club, the two dates, the reason and the announced flag.
///
/// Copies the pending retirement (0x1a bytes) from `source`: the person's id (+0) and the club's (+4), the dates at +8
/// and +0x10 (`exe::date_operator_3d`, with the return value in a local), the reason (+0x18) and the announced flag (+0x19). Returns
/// `this`.
/// Original: `retire_record_assign` (0x7a8780, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a8780 } (thiscall, ret 4, player_regen.cpp): copia a aposentadoria pendente (0x1a bytes) de
/// `origem`: o id da pessoa (+0) e do clube (+4), as datas em +8 e +0x10 (0x417c10, com o retorno numa
/// local), o motivo (+0x18) e a marca de anunciada (+0x19). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn retire_record_assign(this: usize, source: usize) -> usize {
    let mut local = [0u8; 8];
    *(this as *mut u32) = *(source as *const u32);
    *((this + retirement_entry::CLUB_ID) as *mut u32) = *((source + retirement_entry::CLUB_ID) as *const u32);
    exe::date_operator_3d(this + retirement_entry::ANNOUNCE_DATE, (&mut local) as *mut [u8; 8] as usize, source + retirement_entry::ANNOUNCE_DATE);
    exe::date_operator_3d(this + retirement_entry::RETIRE_DATE, (&mut local) as *mut [u8; 8] as usize, source + retirement_entry::RETIRE_DATE);
    *((this + retirement_entry::REASON) as *mut u8) = *((source + retirement_entry::REASON) as *const u8); // motivo
    *((this + retirement_entry::ANNOUNCED) as *mut u8) = *((source + retirement_entry::ANNOUNCED) as *const u8);
    this
}

/// Returns the injury type of a player_setup.cfg INJURY line by name (76 names, from cold to arthritis); none: 0x4b.
///
/// `player_setup_injury_from_name(name)` (cdecl): the injury type (0..0x4c) by the name of the player_setup.cfg injury
/// line (strncmp with the 76 names, each by its own length, in order; the first that matches); none: 0x4b. Used when
/// applying the lines (0x7a3a10) on game creation. The return value counts through al (the original leaves the rest of
/// eax from strncmp).
/// Original: `player_setup_injury_from_name` (0x7a32e0, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a32e0 } (cdecl, player_regen.cpp): `player_setup_injury_from_name(nome)`: o tipo da lesão (0..0x4c) pelo nome
/// da linha de lesão do player_setup.cfg (strncmp com os 76 nomes, cada um pelo seu tamanho, na ordem; o primeiro que
/// bate); nenhum: 0x4b. Pela aplicação das linhas (0x7a3a10) na criação do jogo. A devolução vale pelo al (a original
/// deixa o resto do eax do strncmp).
pub(crate) unsafe extern "C" fn player_setup_injury_from_name(name: usize) -> u32 {
    for &(n, texto, tipo) in INJURY_NAMES.iter() {
        if strncmp(name, texto, n) == 0 {
            crate::comparison::branch("cqf_9 injury by name");
            return tipo as u32; // cqf9: the injury type by name
        }
    }
    if strncmp(name, 0xa6b3a8, 8) == 0 { 0x4c } else { 0x4b }
}

/// Builds one club list per nation; each club with a division joins its nation's list, except national teams and the
/// clubs at 0x9d00a4/0x9d00a8.
///
/// `PLAYER_REGEN::build_nation_club_lists()`; SEH frame for the `new`. [this] = 14 bytes per nation of [0xae2350]
/// (`exe::crt_malloc`, no check), with a new list (`exe::crt_operator_new` of 0xe and `exe::new_llist`; out of memory, 0) per nation; each club of
/// [0xae235c] goes (0x68d0d0, the 4 bytes of the id) into its nation's list (`injury_youth_list_nation`: clubs with a
/// division, a nation with +0x11c only with +0x80 up to 2, except national teams and the clubs at 0x9d00a4/0x9d00a8).
/// Original: `PLAYER_REGEN::build_nation_club_lists` (0x7acfc0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7acfc0 } (thiscall, ret 0, player_regen.cpp, quadro SEH do `new`): `PLAYER_REGEN::build_nation_club_lists()`:
/// [this] = 14 bytes por nação de [0xae2350] (0x944e46, sem teste), com uma lista nova (0x944cf1 de 0xe e 0x68d020;
/// sem memória, 0) por nação; cada clube de [0xae235c] entra (0x68d0d0, os 4 bytes do id) na lista da nação dele
/// (`injury_youth_list_nation`).
pub(crate) unsafe extern "thiscall" fn build_nation_club_lists(this: usize) {
    let table = exe::crt_malloc((read_i32(G_NATION_COUNT) as u32).wrapping_mul(14));
    core::ptr::write_unaligned(this as *mut usize, table);
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        let obj = exe::crt_operator_new(0xe);
        let l = if obj != 0 { exe::new_llist(obj) } else { 0 };
        core::ptr::write_unaligned(read_usize(this).wrapping_add(i as usize * 4) as *mut usize, l);
        i += 1;
    }
    let base = read_usize(G_CLUB_TABLE);
    let mut j = 0i32;
    while j < read_i32(G_CLUB_COUNT) {
        let club = base.wrapping_add((j as usize).wrapping_mul(club::SIZE));
        let nation = injury_youth_list_nation(club);
        if nation != 0 {
            crate::comparison::branch("cpp_9 club in the list");
            let l = read_usize(read_usize(this).wrapping_add((read_i32(nation) as usize).wrapping_mul(4)));
            exe::llist_add_tail(l, club, 4);
        }
        j += 1;
    }
}

/// Processes the regen queue every day: due youth regens are created, and with more than 2 spare coach records, count −
/// 2 of them are given to persons who lend the data.
///
/// `PLAYER_REGEN::process_regen_queue()`, from the day loop (0x5cccd0), every day. The youth regen queue ([+0x10], 12
/// bytes: +0 id, +4 date; count +0x14): from the front, while the first one's date is before today (`exe::date_operator_3c`) and +0x40
/// is not zero, the youth regen (`regenerate_as_youth_player`) of the person (id outside 0..[0xae2364]: box "Invalid staff_id for regen"
/// and end), the following ones move up one place (`regen_queue_entry_assign`) and the last gets id −1. Then, with more than 2 spare
/// coach records ([+0x1c], count +0x20), for count − 2 of them: the person lending the data (`player_regen_pick_staff_for_non_player_data` by the first
/// one's nation +4) gets the spare's non-player record (+0x69, [0xae23c0] + 0x44 × id); a former player: the swap of
/// +0x10/+0x11 of the rating 20, the normalisation (`staff_normalise_non_player_attributes`), the job at the computer's club and the three ratings
/// (+0xa, +8, +0xc) by the reputation, +9 and +0xd of the player record (band above 7500, or random(+0x57) and the
/// other two; pow and bands); a non-player: random(1500) and random(500). The spare leaves (`remove_spare_non_player_record`).
/// Original: `PLAYER_REGEN::process_regen_queue` (0x7a65d0, player_regen.cpp, thiscall, ret 0).
///
/// [pt] { 0x7a65d0 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::process_regen_queue()`, pelo laço do dia (0x5cccd0),
/// todo dia. A fila dos regens jovens ([+0x10], 12 bytes: +0 id, +4 data; conta +0x14): da frente, enquanto a data da
/// primeira for antes de hoje (0x456770) e +0x40 não for zero, o regen jovem (0x7abac0) da pessoa (id fora de 0..[0xae2364]:
/// caixa "Invalid staff_id for regen" e fim), as seguintes sobem uma casa (0x7a8b70) e a última fica com id −1. Depois, com
/// mais de 2 reservas de técnico ([+0x1c], conta +0x20), para conta − 2 delas: a pessoa que empresta os dados (0x7a6e60
/// pela nação +4 da primeira) ganha o registro de não jogador da reserva (+0x69, [0xae23c0] + 0x44 × id); ex-jogador: a
/// troca de +0x10/+0x11 da nota 20, a normalização (0x539490), o cargo no clube da máquina e as três notas (+0xa, +8, +0xc)
/// pela reputação, +9 e +0xd da ficha (faixa acima de 7500, ou sorteio(+0x57) e as outras duas); não jogador: sorteio(1500)
/// e sorteio(500). A reserva sai (0x7a82b0).
pub(crate) unsafe extern "thiscall" fn process_regen_queue(this: usize) {
    let c0 = read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT);
    let first = read_usize(this + player_regen_fields::REGEN_QUEUE);
    let mut k = 0i32;
    while k < c0 {
        if !is_regen_entry_due(first) || read_i32(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == 0 {
            break;
        }
        let id = read_i32(first);
        if id < 0 || id > read_i32(G_STAFF_COUNT) {
            let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            format(0xde1f64, 0xa6bfcc, id);
            show_error_at(0xa6bf9c, 0x36e);
            return;
        }
        crate::comparison::branch("cpw_10 queued regen");
        exe::player_regen_regenerate_as_youth_player(this, read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize));
        let mut j = 0i32;
        while j < read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT) - 1 {
            let e = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add((j as u32).wrapping_mul(0xc) as usize);
            exe::regen_queue_entry_assign(e, e + 0xc);
            j += 1;
        }
        let n = read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT);
        let last = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add((n as u32).wrapping_mul(0xc) as usize).wrapping_sub(0xc);
        core::ptr::write_unaligned(last as *mut i32, -1);
        write_u16(last + regen_queue_entry::REGEN_DATE, 0);
        write_u16(last + regen_queue_entry::REGEN_YEAR, 0);
        core::ptr::write_unaligned((this + player_regen_fields::REGEN_QUEUE_COUNT) as *mut i32, n - 1); // cpw10: a entrada sai
        k += 1;
    }
    let total = read_i32(this + player_regen_fields::SPARE_NON_PLAYER_COUNT);
    if total <= 2 {
        return;
    }
    for _ in 0..total - 2 {
        let s = exe::player_regen_pick_staff_for_non_player_data(this, read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS) + 4));
        if s == 0 {
            return;
        }
        crate::comparison::branch("cpw_10 reserve manager");
        let nj = read_usize(G_NON_PLAYER_TABLE).wrapping_add((read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS)) as u32).wrapping_mul(0x44) as usize);
        core::ptr::write_unaligned((s + staff::NON_PLAYER) as *mut usize, nj);
        let sheet = read_usize(s + staff::PLAYER);
        if sheet == 0 {
            let a = friendly_random(0x5dc).wrapping_add(1) as u16;
            write_u16(read_usize(s + staff::NON_PLAYER) + 8, a);
            let b = friendly_random(0x5dc).wrapping_add(1) as u16;
            write_u16(read_usize(s + staff::NON_PLAYER) + 0xa, b);
            let c = friendly_random(0x1f4).wrapping_add(1) as u16;
            write_u16(read_usize(s + staff::NON_PLAYER) + 0xc, c);
        } else {
            if *((sheet + player_sheet::GOALKEEPER) as *const i8) == 0x14 {
                let (cl, dl) = (*((nj + non_player::COACHING) as *const i8), *((nj + non_player::COACHING_GKS) as *const i8));
                if cl > dl {
                    *((nj + non_player::COACHING_GKS) as *mut i8) = cl;
                    *((read_usize(s + staff::NON_PLAYER) + 0x10) as *mut i8) = dl;
                    if friendly_random(4) == 0 {
                        let r = friendly_random(0x32) as u8;
                        *((read_usize(s + staff::NON_PLAYER) + 0x11) as *mut u8) = r.wrapping_add(0x32);
                    }
                }
            }
            exe::staff_normalise_non_player_attributes(s);
            place_coach_at_ai_club(s);
            let rep = read_i16(read_usize(s + staff::PLAYER) + 0xb);
            let (k, teto) = if rep > 0x1d4c {
                (0x9689d8, 0x9690b8)
            } else if friendly_random(*((s + staff::AMBITION) as *const i8) as i32) != 0 {
                (0x969040, 0x96d300)
            } else {
                (0x968968, 0x96e288)
            };
            let a = rebuilt_coach_rating(s, rep, None, k, teto);
            write_u16(read_usize(s + staff::NON_PLAYER) + 0xa, a);
            let b = rebuilt_coach_rating(s, read_i16(read_usize(s + staff::PLAYER) + 9), None, k, teto);
            write_u16(read_usize(s + staff::NON_PLAYER) + 8, b);
            let c = rebuilt_coach_rating(s, read_i16(read_usize(s + staff::PLAYER) + 0xd), Some(0x969040), k, teto);
            write_u16(read_usize(s + staff::NON_PLAYER) + 0xc, c);
        }
        exe::player_regen_remove_spare_non_player_record(this, (read_i32(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS))) as i32 as u32);
    }
}

/// Returns 1 when the person's id is on the retirement list, otherwise 0.
///
/// `PLAYER_REGEN::is_retirement_pending(person)`: 1 when the person's id (+0) is on the retirement list ([this+4],
/// 0x1a-byte records, count +8 signed), otherwise 0 (and 0 without a person). Used by
/// MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: a non-player who is about to retire is not suitable as a
/// manager.
/// Original: `PLAYER_REGEN::is_retirement_pending` (0x7ae350, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ae350 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::is_retirement_pending(pessoa)`: 1 quando o id da
/// pessoa (+0) está na lista de aposentadorias ([this+4], registros de 0x1a bytes, conta +8 com sinal), senão 0 (e 0
/// sem pessoa). Pela MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: quem não é jogador e vai se aposentar não
/// serve para técnico.
pub(crate) unsafe extern "thiscall" fn is_retirement_pending(this: usize, person: usize) -> u8 {
    if person == 0 {
        return 0;
    }
    let n = read_usize(this + player_regen_fields::RETIREMENT_COUNT) as i32;
    let id = read_usize(person);
    let mut i = 0i32;
    while i < n {
        if read_usize(read_usize(this + player_regen_fields::RETIREMENTS) + 0x1a * i as usize) == id {
            crate::comparison::branch("cpq_10 retirement scheduled");
            return 1; // is in the list
        }
        i += 1;
    }
    0
}

/// Copies the first spare non-player record entry (id, nation, second nation) to the outputs and removes it from the
/// list.
///
/// `PLAYER_REGEN::take_spare_non_player_record(&id, &nation, &second)`, from contract_manager (`exe::contract_manager_resolve_contract_offers`, two calls):
/// copies the first item of the spare non-player record pool (this +0x1c: id, nation and second nation) into the three
/// outputs and removes it from the pool (PLAYER_REGEN::remove_spare_non_player_record `remove_spare_non_player_record` with the id read from
/// the output); returns its eax.
/// Original: `PLAYER_REGEN::take_spare_non_player_record` (0x7a6e30, player_regen.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7a6e30 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::take_spare_non_player_record(&id, &nacao,
/// &segunda)`, pela contract_manager (0x4ce4e0, duas chamadas): copia o primeiro item da reserva de fichas de não
/// jogador (this +0x1c: id, nação e segunda nação) para as três saídas e tira-o da reserva
/// (PLAYER_REGEN::remove_spare_non_player_record 0x7a82b0 com o id lido da saída); devolve o eax dela.
pub(crate) unsafe extern "thiscall" fn take_spare_non_player_record(this: usize, id: *mut u32, nation: *mut u32, second: *mut u32) -> u32 {
    crate::comparison::branch("cpt_10 reserved record");
    *id = read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS)) as u32;
    *nation = read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS) + 4) as u32;
    *second = read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS) + 8) as u32;
    exe::player_regen_remove_spare_non_player_record(this, *id)
}

/// bits 4..5 of byte +0xb of the person's 79-byte record
/// Original: `record_bits` (0x7a7eb0, player_regen.cpp, stdcall).
///
/// [pt] { 0x7a7eb0 } (stdcall, ret 4, player_regen.cpp): os bits 4..5 do byte +0xb do registro de 79 bytes
/// da pessoa `*p` em [0xae2d0c]. Devolve só `al`.
pub(crate) unsafe extern "stdcall" fn record_bits(p: *const i32) -> u8 {
    let r = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((*p as usize).wrapping_mul(79));
    (*((r + 0xb) as *const u8) >> 4) & 3
}

