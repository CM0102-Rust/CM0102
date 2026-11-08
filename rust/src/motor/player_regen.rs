use super::*;

// ---------- 271a leva: o regen jovem: a pessoa aposentada refeita como jogador novo do banco de fichas (nação, nomes, idiomas, idade, habilidade, reputações, personalidade) e posta num clube; a saída da contest (player_regen.cpp, dispute.cpp) ----------








/// Calls a one-argument thiscall routine by address (the person clean-up routines).
///
/// [pt] Uma chamada thiscall de um argumento pelo endereço (as rotinas de limpeza da pessoa).
pub(crate) unsafe fn call_thiscall_1(rotina: usize, this: usize, a: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(rotina);
    f(this, a);
}

/// Pointer to byte k of the person's 79-byte state record ([0xae2d0c] + 0x4f x id).
///
/// [pt] O byte `k` do registro de 79 bytes da pessoa ([0xae2d0c] + 0x4f × id).
pub(crate) unsafe fn staff_state_byte(p: usize, k: usize) -> *mut u8 {
    (read_u32_as_usize(G_STAFF_STATE_TABLE) as isize + read_i32(p) as isize * 0x4f + k as isize) as *mut u8
}

/// Integer times an .rdata double constant, truncated (_ftol) to 16 bits.
///
/// [pt] O inteiro × a constante double do .rdata, truncado (fild; fmul qword; _ftol), em 16 bits.
pub(crate) unsafe fn mul_rdata_const(v: i16, k: usize) -> i16 {
    match_pl_ftol(v as f64 * read_f64_at(k)) as i16
}

/// Sets current ability (record +5) from PA (+7): trunc(random(trunc(PA x m1)) + PA x m2).
///
/// [pt] A habilidade (ficha +5) pelo sorteio: r = sorteio(trunc(PA × m1)); trunc(r + PA × m2) (fild r; fild PA; fmul m2;
/// faddp; _ftol), a PA na ficha +7.
pub(crate) unsafe fn roll_current_ability(f: usize, m1: usize, m2: usize) {
    let pa = read_i16(f + 7) as f64;
    let r = friendly_random(match_pl_ftol(pa * read_f64_at(m1)));
    crate::comparacao::ramo("cqg_7 habilidade");
    write_i16(f + 5, match_pl_ftol(r as f64 + pa * read_f64_at(m2)) as i16); // cqg7: a habilidade sorteada pela PA
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
pub(crate) unsafe fn cap_current_ability(f: usize, teto: i16, n: i32) {
    if read_i16(f + 5) > teto && friendly_random(n) != 0 {
        write_i16(f + 5, teto);
    }
}

/// rebuilds the retired person of the regen queue as a young player: the front record of the pool, the nation (or
/// another small one), the second nation, the nation names and languages, the age and ability (by the small nation
/// reputation or by position and by draw), the reputations and their caps, the birth, the personality attributes by age
/// and, by chance, the best club or a club of the nation
/// Original: `PLAYER_REGEN::regenerate_as_youth_player` (0x7abac0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7abac0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::regenerate_as_youth_player(pessoa)`, pela fila dos
/// regens (0x7a65d0) em todo regen jovem. Pessoa não aposentada (bits 0x30 do +0xb do registro de 79 bytes) ou com
/// clube: só limpa os bits. Senão: sai da seleção (0x775ba0), pega a ficha da frente do banco de fichas (this+0x3c,
/// entradas de 16 bytes: ficha, nação, segunda nação; 0x7a8470), vira jogador (+0x5f = 2, +0x61 a ficha) e o registro
/// de 79 bytes ganha a marca do regen (0x40) e perde as outras; as listas e históricos da pessoa são limpos (névoa,
/// estatísticas, histórico, suspensões, lesões, contest, disciplina, físico, contratos); a nação da ficha (ou, se ela
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
    let marca = staff_state_byte(p, 0xb);
    let b = *marca;
    if b & 0x30 == 0 || read_u32_as_usize(p + 0x39) != 0 {
        *marca = b & 0xcf;
        return;
    }
    call_thiscall_1(0x775ba0, read_u32_as_usize(G_NATIONAL_TEAMS), p);
    let banco = read_u32_as_usize(this + 0x3c);
    let (ficha_id, nacao_id, nacao2_id) = (read_i32(banco), read_i32(banco + 4), read_i32(banco + 8));
    let tirar_do_banco: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_PLAYER_REGEN_REMOVE_PLAYER_DATA_FROM_POOL);
    tirar_do_banco(this, ficha_id);
    *((p + 0x5f) as *mut u8) = 2;
    core::ptr::write_unaligned((p + 0x61) as *mut usize, read_u32_as_usize(G_PLAYER_DATA_TABLE).wrapping_add((ficha_id as isize * 0x46) as usize));
    *staff_state_byte(p, 0xb) |= 0x40;
    *staff_state_byte(p, 0xd) = 0;
    *staff_state_byte(p, 0xc) &= 0xfd;
    *staff_state_byte(p, 0xb) &= 0x7f;
    *staff_state_byte(p, 0xc) &= 0xfe;
    *staff_state_byte(p, 9) = 0x9c;
    if read_u32_as_usize(G_FOG_OF_WAR) != 0 && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0 {
        call_thiscall_1(0x5afd00, read_u32_as_usize(G_FOG_OF_WAR), p);
    }
    call_thiscall_1(0x7c6650, read_u32_as_usize(G_PLAYER_STATS), p);
    call_thiscall_1(0x86fb80, read_u32_as_usize(G_STAFF_RECORDS), p);
    call_thiscall_1(0x55e1d0, read_u32_as_usize(G_DISCIPLINE), p);
    call_thiscall_1(0x631f90, read_u32_as_usize(G_INJURY), p);
    call_thiscall_1(0x56a090, read_u32_as_usize(G_DISPUTE), p);
    call_thiscall_1(0x556830, read_u32_as_usize(G_DISCIPLINE), read_u32_as_usize(p));
    call_thiscall_1(0x62bfa0, read_u32_as_usize(G_INJURY), read_u32_as_usize(p));
    call_thiscall_1(0x4d0b50, read_u32_as_usize(G_CONTRACT_MANAGER), p);
    call_thiscall_1(0x4d0bd0, read_u32_as_usize(G_CONTRACT_MANAGER), p);
    *((p + 0x22) as *mut u8) = 0;
    *((p + 0x23) as *mut u8) = 0;
    if read_u32_as_usize(p + 0x69) != 0 {
        core::ptr::write_unaligned((p + 0x69) as *mut u32, 0);
    }
    *staff_state_byte(p, 0xb) &= 0xf0;
    *staff_state_byte(p, 8) = 0;
    *staff_state_byte(p, 9) = 0x9c;
    let nacoes = read_u32_as_usize(G_NATION_TABLE);
    let nac = nacoes.wrapping_add((nacao_id as isize * 0x122) as usize);
    core::ptr::write_unaligned((p + 0x1a) as *mut usize, nac);
    if read_i32(nac + 0x88) < 100 && friendly_random(4) == 0 {
        let k = friendly_random(read_i32(0xae2350));
        let outra = read_u32_as_usize(G_NATION_TABLE).wrapping_add((k as isize * 0x122) as usize);
        if read_u32_as_usize(outra + 0x71) != 0 && read_i32(outra + 0x88) < 100 {
            core::ptr::write_unaligned((p + 0x1a) as *mut usize, outra);
        }
    }
    if nacao2_id > 0 && nacao2_id < read_i32(0xae2350) {
        if !(nacao_id == read_i32(KEY_NATION_SPAIN) && friendly_random(100) == 0) {
            if nacao2_id != read_i32(KEY_NATION_PAYS_BASQUE) && friendly_random(3) != 0 {
                core::ptr::write_unaligned((p + 0x1e) as *mut u32, 0);
            } else {
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, read_u32_as_usize(G_NATION_TABLE).wrapping_add((nacao2_id as isize * 0x122) as usize));
            }
        }
    } else {
        core::ptr::write_unaligned((p + 0x1e) as *mut u32, 0);
    }
    let nome1: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    let nome2: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_FOR_NATION);
    let nome3: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_EMPTY_COMMON_NAME);
    let v = nome1(read_u32_as_usize(p + 0x1a));
    core::ptr::write_unaligned((p + 4) as *mut usize, v);
    let v = nome2(read_u32_as_usize(p + 0x1a));
    core::ptr::write_unaligned((p + 8) as *mut usize, v);
    let v = nome3(read_u32_as_usize(p + 0x1a));
    core::ptr::write_unaligned((p + 0xc) as *mut usize, v);
    call_thiscall_1(0x5fff30, read_u32_as_usize(G_HUMAN_MANAGER), p);
    write_i16(p + 0x40, read_i16(G_CURRENT_DATE_YEAR));
    write_i16(p + 0x3e, read_i16(G_CURRENT_DATE));
    core::ptr::write_unaligned((p + 0x39) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x24) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x4e) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x52) as *mut u32, 0);
    // os idiomas: a original empurra o dword do argumento com o byte de baixo trocado (as chamadas leem o byte)
    let tirar_idioma: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(FN_PLAYERSTATS_REMOVE_LANGUAGE);
    for k in 1..=0x40i8 {
        tirar_idioma(read_u32_as_usize(G_PLAYER_STATS), p, k);
    }
    let peso: unsafe extern "stdcall" fn(usize, i8) -> i16 = core::mem::transmute(FN_PLAYER_STATS_GET_NATIVE_LANGUAGE_POINTS);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_ADD_LANGUAGE);
    let base = (0x14i16.wrapping_sub(read_i8(p + 0x56) as i16)).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = peso(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            // ponytail: a fluência vai num dword da pilha com só o byte de baixo gravado (a 0x7c6a30 lê o byte)
            let nivel = if q > 0xa { 0xa } else { q as u8 };
            acrescentar(read_u32_as_usize(G_PLAYER_STATS), p, k as u8 as u32, nivel as u32, 1);
        }
    }
    *((p + 0x3d) as *mut u8) = 0xb;
    *((p + 0x28) as *mut u8) = 0xb;
    *staff_state_byte(p, 0xb) &= 0xcf;
    let f = read_u32_as_usize(p + 0x61);
    let nac = read_u32_as_usize(p + 0x1a);
    let idade: i8;
    if nac != 0 && *((nac + 0x11c) as *const u8) == 0 && read_i32(nac + 0x88) < 0x15e {
        // a nação pequena: a idade pela reputação dela e a habilidade pela PA
        let r1 = friendly_random(read_i32(nac + 0x88));
        let r2 = friendly_random(1000);
        idade = if r1 < r2 {
            (friendly_random(5) as i8).wrapping_add(0x17)
        } else {
            (friendly_random(7) as i8).wrapping_add(0x11)
        };
        crate::comparacao::ramo("cqg_7 nacao pequena");
        if idade > 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if idade > 0x1a {
            roll_current_ability(f, 0x968918, 0x968e48);
        } else if idade > 0x17 {
            roll_current_ability(f, 0x968fb0, 0x96a688);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        let r = friendly_random(0x2ee);
        write_i16(f + 9, (r as i16).wrapping_add(read_i16(f + 5).wrapping_mul(0x1e)).wrapping_add(0xfa));
        let mut l121 = idade <= 0x12;
        if !l121 && read_i16(f + 7) > 0x8c && read_i16(f + 5) > 0x64 && read_i16(f + 9) < 0x1964 && idade < 0x16 && friendly_random(4) != 0 {
            write_i16(f + 9, mul_rdata_const(read_i16(f + 9), 0x968978));
        }
        if idade <= 0x12 {
            l121 = true;
        }
        if l121 {
            let a = read_i16(f + 5);
            let r = friendly_random(0xc8) as i16;
            let v = if a < 0x8c {
                r.wrapping_add(read_i16(f + 5).wrapping_mul(5)).wrapping_add(1)
            } else {
                r.wrapping_add(read_i16(f + 5).wrapping_add(0x19).wrapping_mul(10))
            };
            write_i16(f + 0xd, v);
        } else {
            let a = read_i16(f + 5);
            if a < 0x78 {
                let r = friendly_random(0xc8) as i16;
                write_i16(f + 0xd, r.wrapping_add(read_i16(f + 5).wrapping_mul(0xf)).wrapping_add(1));
            } else if idade > 0x15 && a > 0x78 {
                let r = friendly_random(0xc8) as i16;
                write_i16(f + 0xd, r.wrapping_add(read_i16(f + 5).wrapping_add(0xa).wrapping_mul(0x19)));
            } else {
                let r = friendly_random(0xc8) as i16;
                let soma = if a < 0x8c { 1 } else { 0xfa };
                write_i16(f + 0xd, r.wrapping_add(read_i16(f + 5).wrapping_mul(0x14)).wrapping_add(soma));
            }
        }
        if read_i16(f + 9) < 0x8ca {
            let r = friendly_random(5);
            write_i16(f + 0xd, (r as i16).wrapping_add(1));
        }
        if read_i16(f + 9) < read_i16(f + 0xd) {
            write_i16(f + 9, read_i16(f + 0xd).wrapping_add(0xa));
            write_i16(f + 0xd, 1);
        }
        let r = friendly_random(0x32);
        write_i16(f + 0xb, (r as i16).wrapping_add(read_i16(f + 9)));
        if idade > 0x17 {
            if read_i16(f + 9) < 0x1770 {
                write_i16(f + 9, mul_rdata_const(read_i16(f + 9), 0x969588));
            }
            if read_i16(f + 0xb) < 0x1770 {
                write_i16(f + 0xb, mul_rdata_const(read_i16(f + 0xb), 0x969588));
            }
            let a = read_i16(f + 5);
            if a < mul_rdata_const(a, 0x96a5d0) && friendly_random(4) != 0 {
                write_i16(f + 5, mul_rdata_const(read_i16(f + 5), 0x96a5d0));
            }
        } else if idade < 0x14 {
            if read_i16(f + 5) < mul_rdata_const(read_i16(f + 7), 0x968968) && friendly_random(4) != 0 {
                write_i16(f + 5, mul_rdata_const(read_i16(f + 7), 0x968968));
            }
            if idade < 0x12 {
                cap_current_ability(f, 0x78, 0x96);
            } else {
                cap_current_ability(f, 0x78, 0x32);
            }
        } else {
            if read_i16(f + 9) < 0x1676 {
                write_i16(f + 9, mul_rdata_const(read_i16(f + 9), 0x968928));
            }
            if read_i16(f + 0xb) < 0x1676 {
                write_i16(f + 0xb, mul_rdata_const(read_i16(f + 0xb), 0x968928));
            }
            if read_i16(f + 5) < mul_rdata_const(read_i16(f + 7), 0x968e48) && friendly_random(4) != 0 {
                write_i16(f + 5, mul_rdata_const(read_i16(f + 7), 0x968e48));
            }
            cap_current_ability(f, 0x8c, 0x32);
        }
    } else {
        // a nação grande ou sem nação: a habilidade pela posição e a idade por sorteio(10000)
        if *((f + 0x15) as *const u8) == 0x14 {
            roll_current_ability(f, 0x968e48, 0x968e80);
        } else if *((f + 0x11) as *const u8) == 0x14 || *((f + 0x10) as *const u8) == 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if *((f + 0xf) as *const u8) == 0x14 {
            roll_current_ability(f, 0x968968, 0x9689e0);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        if friendly_random(0xfa) != 0 {
            let r = friendly_random(0x2ee) as i16;
            write_i16(f + 9, r.wrapping_add(read_i16(f + 5).wrapping_mul(0x14)).wrapping_add(0xfa)); // cqg7: a reputação +9 do regen
            crate::comparacao::ramo("cqg_7 injury_word_0c do regen");
            let r = friendly_random(0x14) as i16;
            write_i16(f + 0xd, r.wrapping_add(read_i16(f + 5).wrapping_mul(5)).wrapping_add(1));
        } else {
            let r = friendly_random(0x2ee) as i16;
            write_i16(f + 9, r.wrapping_add(read_i16(f + 5).wrapping_mul(0xf)).wrapping_add(0x7d));
            write_i16(f + 0xd, 1);
        }
        if read_i16(f + 9) < read_i16(f + 0xd) {
            write_i16(f + 9, read_i16(f + 0xd).wrapping_add(0xa));
            write_i16(f + 0xd, 1);
        }
        let r = friendly_random(0x32);
        write_i16(f + 0xb, (r as i16).wrapping_add(read_i16(f + 9)));
        let s = friendly_random(0x2710);
        crate::comparacao::ramo("cqg_7 nacao grande");
        if s == 0 {
            idade = (friendly_random(4) as i8).wrapping_add(0xf);
            if idade == 0xf {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s < 0x1388 {
            idade = (friendly_random(3) as i8).wrapping_add(0x10); // cqg7: a idade de 16 a 18
            crate::comparacao::ramo("cqg_7 idade 16 a 18");
            if idade < 0x12 {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s > 0x26f7 {
            idade = (friendly_random(6) as i8).wrapping_add(0x15);
        } else {
            idade = (friendly_random(4) as i8).wrapping_add(0x11);
            if idade == 0x11 {
                cap_current_ability(f, 0x5a, 0xfa);
            } else {
                cap_current_ability(f, 0x6e, 0x96);
            }
        }
    }
    let grande: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_NATION_IS_MAJOR_NATION);
    if grande(read_u32_as_usize(p + 0x1a)) == 0 {
        if idade < 0x19 && read_i16(f + 5) < 0x78 {
            write_i16(f + 0xd, 1);
        } else {
            write_i16(f + 0xd, mul_rdata_const(read_i16(f + 0xd), 0x96a5d0));
        }
    }
    write_i16(p + 0x12, read_i16(G_CURRENT_DATE_YEAR).wrapping_sub(idade as i16)); // cqg7: o ano do nascimento
    let r = friendly_random(0x16d);
    write_i16(p + 0x10, r as i16);
    let anos: unsafe extern "C" fn(usize, *const u16) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_AGE);
    let a = anos(p, G_CURRENT_DATE as *const u16);
    write_i16(p + 0x18, a as u8 as i8 as i16);
    // +0x1b
    if read_i16(f + 7) > 0x64 && (read_i8(f + 0x15) >= 0xf || read_i8(f + 0x14) >= 0xf) {
        if friendly_random(3) != 0 {
            let r = friendly_random(0xb);
            write_i8(f + 0x1b, (r as i8).wrapping_add(0xa));
        } else if friendly_random(3) == 0 {
            let r = friendly_random(7);
            write_i8(f + 0x1b, (r as i8).wrapping_add(0xe));
        } else {
            let r = friendly_random(0x10);
            write_i8(f + 0x1b, (r as i8).wrapping_add(5));
        }
    } else if friendly_random(4) != 0 {
        let r = friendly_random(0xa);
        write_i8(f + 0x1b, (r as i8).wrapping_add(8));
    } else {
        let r = friendly_random(0x10);
        write_i8(f + 0x1b, (r as i8).wrapping_add(5));
    }
    adjust_personality_attribute(f + 0x1b, 5, 5, 0xa, 0xa, 8);
    // +0x36 pelo +0x1b
    let a = read_i8(f + 0x1b);
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
    write_i8(f + 0x36, v);
    // +0x1d (o goleiro de 20 com 4/5 pela PA)
    if read_i8(f + 0xf) == 0x14 && friendly_random(5) != 0 {
        let r = friendly_random(0x14);
        let v = if read_i16(f + 7) as i32 > r + 0x64 {
            (friendly_random(0xb) as i8).wrapping_add(0xa)
        } else {
            (friendly_random(0x10) as i8).wrapping_add(5)
        };
        write_i8(f + 0x1d, v);
    } else {
        let r = friendly_random(0x14);
        write_i8(f + 0x1d, (r as i8).wrapping_add(1));
        adjust_personality_attribute(f + 0x1d, 6, 5, 6, 0, 0);
    }
    // +0x2e
    let mut k: i8 = -5;
    if read_i8(f + 0x11) >= 0xf {
        k = if read_i8(f + 0x19) >= 0xf { 5 } else { 0 };
    } else if ((read_i8(f + 0x13) >= 0xf || read_i8(f + 0x12) >= 0xf) && read_i8(f + 0x19) >= 0xf)
        || read_i8(f + 0xf) >= 0xf
        || read_i8(f + 0x15) >= 0xf
    {
        k = 0;
    }
    let r1 = friendly_random(k as i32 * 3);
    let r2 = friendly_random(0x14);
    write_i8(f + 0x2e, (r1 as i8).wrapping_add(r2 as i8));
    let a = read_i8(f + 0x2e);
    write_i8(f + 0x2e, if a < 1 { 1 } else if a > 0x14 { 0x14 } else { a });
    // +0x34
    let r = friendly_random(0x14);
    write_i8(f + 0x34, (r as i8).wrapping_add(1));
    adjust_personality_attribute(f + 0x34, 6, 8, 0xa, 0, 0);
    // +0x44
    let r = friendly_random(0x14);
    write_i8(f + 0x44, (r as i8).wrapping_add(1));
    if read_i8(f + 0x44) > 0xf && friendly_random(3) != 0 {
        subtract_random(f + 0x44, 5);
    } else {
        let a = read_i8(f + 0x44);
        if a < 5 {
            add_random_if_below(f + 0x44, 5, 0xa);
        } else if a < 0xa {
            add_random_if_below(f + 0x44, 0xa, 5);
        }
    }
    // +0x3c pela idade
    if idade < 0x12 {
        let r = friendly_random(0x14);
        write_i8(f + 0x3c, (r as i8).wrapping_add(1));
        adjust_personality_attribute(f + 0x3c, 8, 5, 8, 0xa, 6);
    } else if idade < 0x18 {
        let r = friendly_random(0x12);
        write_i8(f + 0x3c, (r as i8).wrapping_add(3));
        adjust_personality_attribute(f + 0x3c, 5, 5, 0xa, 0xa, 5);
    } else {
        let r = friendly_random(0xf);
        write_i8(f + 0x3c, (r as i8).wrapping_add(6));
        adjust_personality_attribute(f + 0x3c, 3, 0xa, 8, 0, 0);
    }
    if read_i8(f + 0x3c) < 0xa && read_i8(f + 0x44) > 0xf {
        let r = friendly_random(5);
        write_i8(f + 0x3c, read_i8(f + 0x3c).wrapping_add(r as i8));
    }
    // +0x3d pela idade
    let r = friendly_random(0x14);
    write_i8(f + 0x3d, (r as i8).wrapping_add(1));
    if idade < 0x12 {
        adjust_personality_attribute(f + 0x3d, 0xa, 5, 6, 0, 0);
    } else if idade < 0x18 {
        adjust_personality_attribute(f + 0x3d, 8, 5, 8, 0, 0);
    } else {
        adjust_personality_attribute(f + 0x3d, 5, 5, 0xa, 0, 0);
    }
    if (read_i8(f + 0x11) == 0x14 || read_i8(f + 0x12) == 0x14) && read_i8(f + 0x3d) < 0xa {
        let r = friendly_random(5);
        write_i8(f + 0x3d, read_i8(f + 0x3d).wrapping_add(r as i8));
    }
    if read_i8(f + 0x2e) > 0xf && read_i8(f + 0x3d) < 0xa {
        let r = friendly_random(5);
        write_i8(f + 0x3d, read_i8(f + 0x3d).wrapping_add(r as i8));
    }
    // +0x1e, +0x24, +0x2c, +0x2f (e +0x21) pela idade
    if idade < 0x18 {
        if idade < 0x12 {
            subtract_random(f + 0x1e, 0x28);
            subtract_random(f + 0x24, 0x28);
            let a = read_i8(f + 0x2c);
            if a > 0xf {
                subtract_random(f + 0x2c, 8);
            } else if a > 0xa {
                subtract_random(f + 0x2c, 5);
            }
            let a = read_i8(f + 0x2f);
            if a > 0xf {
                subtract_random(f + 0x2f, 0xa);
            } else if a > 0xa {
                subtract_random(f + 0x2f, 8);
            }
        } else {
            subtract_random(f + 0x1e, 0x14);
            subtract_random(f + 0x24, 0x14);
            if read_i8(f + 0x2c) > 0xf {
                subtract_random(f + 0x2c, 5);
            }
            let a = read_i8(f + 0x2f);
            if a > 0xf {
                subtract_random(f + 0x2f, 8);
            } else if a > 0xa {
                subtract_random(f + 0x2f, 5);
            }
        }
        if read_i8(f + 0x2f) > 0xf {
            subtract_random(f + 0x2f, 8);
        }
    } else if idade > 0x19 && idade < 0x1d {
        add_random_if_below(f + 0x3d, 5, 8);
        add_random_if_below(f + 0x21, 5, 8);
        add_random_if_below(f + 0x3c, 5, 8);
        subtract_random(f + 0x1e, 0xa);
        subtract_random(f + 0x24, 0xa);
        add_random_if_below(f + 0x2c, 5, 8);
        let a = read_i8(f + 0x2f);
        if a > 0xf {
            subtract_random(f + 0x2f, 5);
        } else if a > 0xa {
            subtract_random(f + 0x2f, 3);
        }
    }
    // o clube
    let nac = read_u32_as_usize(p + 0x1a);
    if idade < 0x13 && nac != 0 && *((nac + 0x11c) as *const u8) != 0 && friendly_random(4) != 0 {
        let melhor: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_PLAYER_REGEN_PICK_BEST_CLUB_FOR_PLAYER);
        let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_SIGN_FOR_CLUB);
        let c = melhor(this, p);
        if c != 0 && contratar(p, c) != 0 {
            crate::comparacao::ramo("cqg_7 melhor clube");
            let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
            if humano(c) != 0 {
                let noticia: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(FN_PLAYER_REGEN_POST_YOUTH_PROMOTION_NEWS);
                noticia(p, c);
            }
            if read_i16(read_u32_as_usize(p + 0x61) + 7) > 0x50 {
                let t = read_u32_as_usize(c + 0xbf);
                if t != 0 && read_u32_as_usize(t + 0x69) != 0 {
                    let r = friendly_random(100);
                    if read_i8(read_u32_as_usize(t + 0x69) + 0x16) as i32 > r {
                        let gostar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_STAFF);
                        gostar(t, p, 0);
                    }
                }
            }
        }
    } else if (nac != 0 && *((nac + 0x11c) as *const u8) == 0 && friendly_random(0xf) != 0) || friendly_random(0xa) == 0 {
        let da_nacao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_PLAYER_REGEN_PICK_CLUB_FROM_NATION_LIST);
        let c = da_nacao(this, p);
        if c != 0 {
            let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_SIGN_FOR_CLUB);
            contratar(p, c);
        }
    }
    if read_u32_as_usize(p + 0x39) == 0 && idade < 0x14 {
        let f = read_u32_as_usize(p + 0x61);
        if read_i16(f + 0xb) > 0x4e2 {
            write_i16(f + 0xb, 0x4e2);
        }
        if read_i16(f + 9) > 0x4e2 {
            write_i16(f + 9, 0x4e2);
        }
        write_i16(f + 0xd, 1);
    }
    let fisicos: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_PLAYER_PHYSICAL_ATTRIBUTES);
    let atributos: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_PLAYER_ATTRIBUTES);
    fisicos(p);
    atributos(p);
}

/// Comparador: só a original (ponytail: a rotina tira a ficha do banco, limpa listas e históricos, aloca idiomas e o
/// contrato e manda notícia; não pode rodar duas vezes). A cópia roda no modo Rust (contadores "cqg_7 ...") e é provada
/// pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_regenerate_as_youth_player(this: usize, p: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REGENERATE_AS_YOUTH_PLAYER)(this, p)
}

/// clears the person dispute flag and removes her entry from the dispute list
/// Original: `DISPUTE::remove_dispute` (0x56a090, dispute.cpp, thiscall).
///
/// [pt] { 0x56a090 } (thiscall, ret 4, dispute.cpp): `DISPUTE::remove_dispute(pessoa)`: sem pessoa ou sem a tabela de marcas
/// (+4), ou com a marca da pessoa zerada: nada. Senão zera a marca e, com a lista (+0), volta o cursor ao começo (+0xa =
/// +2) e procura, nas `conta` (+0, palavra) primeiras, a entrada do id da pessoa (0x68d1b0): achando, tira-a (0x68d1f0).
pub(crate) unsafe extern "thiscall" fn remove_dispute(this: usize, p: usize) {
    if p == 0 || read_u32_as_usize(this + 4) == 0 {
        return;
    }
    let m = (read_u32_as_usize(this + 4) as isize + read_i32(p) as isize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m = 0;
    let lista = read_u32_as_usize(this);
    if lista == 0 {
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32_as_usize(lista + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut k = 0i32;
    while (k as i16 as i32) < read_i16(read_u32_as_usize(this)) as i32 {
        let no = proximo(read_u32_as_usize(this));
        if no == 0 {
            return;
        }
        if read_i32(no) == read_i32(p) {
            crate::comparacao::ramo("cqg_7 saiu da contest");
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
            tirar(read_u32_as_usize(this), no);
            return;
        }
        k += 1;
    }
}

/// Comparador: sem a marca (nada a fazer), lado a lado na marca e no cursor da lista. Com a marca, só a original com o
/// plano: a marca zerada e a conta da lista uma a menos quando a pessoa está nela (a procura feita antes, com o cursor
/// devolvido).
pub(crate) unsafe extern "thiscall" fn comparar_remove_dispute(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_DISPUTE);
    if p == 0 || read_u32_as_usize(this + 4) == 0 {
        return o(this, p);
    }
    let m = read_u32_as_usize(this + 4).wrapping_add(read_i32(p) as isize as usize);
    let lista = read_u32_as_usize(this);
    if *(m as *const u8) == 0 {
        let mut reg = vec![(m, 1)];
        if lista != 0 {
            reg.push((lista, 0xe));
        }
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p), || remove_dispute(this, p));
        PLACAR.registrar(igual, || format!("remove_dispute(pessoa {:x}) {:x?} x {:x?}", p, mo.0, mr.0));
        return;
    }
    let mut achou = 0i16;
    let conta_antes = if lista != 0 { read_i16(lista) } else { 0 };
    if lista != 0 {
        let cursor = read_u32_as_usize(lista + 0xa);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32_as_usize(lista + 2) as u32);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        for _ in 0..conta_antes.max(0) {
            let no = proximo(lista);
            if no == 0 {
                break;
            }
            if read_i32(no) == read_i32(p) {
                achou = 1;
                break;
            }
        }
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor as u32);
    }
    o(this, p);
    let conta_depois = if lista != 0 { read_i16(lista) } else { 0 };
    PLACAR.registrar(*(m as *const u8) == 0 && conta_depois == conta_antes - achou,
        || format!("remove_dispute(pessoa {:x}) conta {} -> {} (achou {})", p, conta_antes, conta_depois, achou));
}

// ---------- 271a leva: o player_setup.cfg: a pessoa pelo nome (no clube ou em todos), o texto entre aspas da linha e o tipo da linha pelo nome (player_regen.cpp) ----------




/// Does the person's name k (+4 first, +8 second, +0xc common) match text (up to 26 chars)? Empty text always matches.
///
/// [pt] O nome `k` (+4 primeiro, +8 segundo, +0xc o comum; o registro do nome começa pelo texto) da pessoa bate com `texto`
/// (até 26 caracteres)? `texto` vazio sempre bate.
pub(crate) unsafe fn name_matches(p: usize, k: usize, texto: usize) -> bool {
    *(texto as *const u8) == 0 || strncmp(read_u32_as_usize(p + k), texto, 0x1a) == 0
}

/// finds the person of a player_setup.cfg line by the three names: among all people or, with the club name, in its
/// squad (when asked) and its manager
/// Original: `player_setup_find_staff` (0x7a50f0, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a50f0 } (cdecl, player_regen.cpp): `player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco)`. Sem
/// `clube` (nulo ou vazio): a primeira das [0xae2364] pessoas ([0xae23bc], 0x6e bytes) cujos nomes não vazios batem
/// (cada nome conferido só se o texto e o nome da pessoa existem). Com `clube`: o clube com esse nome (0x545880, 51
/// caracteres); sem achar, 0. Com `no_elenco` 1: o primeiro das 50 vagas do elenco (+0xd7) com os nomes que batem;
/// senão (ou nenhum do elenco), o técnico do clube (+0xcf) se os nomes batem; senão 0.
pub(crate) unsafe extern "C" fn player_setup_find_staff(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    if clube == 0 || *(clube as *const u8) == 0 {
        let mut p = read_u32_as_usize(G_STAFF_TABLE);
        for _ in 0..read_i32(0xae2364).max(0) {
            let falha = (*(primeiro as *const u8) != 0 && read_u32_as_usize(p + 4) != 0 && strncmp(read_u32_as_usize(p + 4), primeiro, 0x1a) != 0)
                || (*(segundo as *const u8) != 0 && read_u32_as_usize(p + 8) != 0 && strncmp(read_u32_as_usize(p + 8), segundo, 0x1a) != 0) // cqh9: o segundo nome
                || (*(comum as *const u8) != 0 && read_u32_as_usize(p + 0xc) != 0 && strncmp(read_u32_as_usize(p + 0xc), comum, 0x1a) != 0);
            if !falha {
                crate::comparacao::ramo("cqh_9 pessoa pelo nome");
                return p;
            }
            p += 0x6e;
        }
        return 0;
    }
    let nome_do_clube: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CLUB_GET_ENGLISH_NAME);
    let mut achado = 0usize;
    for k in 0..read_i32(0xae235c).max(0) {
        let c = read_u32_as_usize(G_CLUB_TABLE).wrapping_add(k as usize * 0x245);
        if strncmp(nome_do_clube(c), clube, 0x33) == 0 {
            achado = c;
            break;
        }
    }
    if achado == 0 {
        return 0;
    }
    let c = achado;
    if no_elenco as u8 == 1 {
        for k in 0..0x32usize {
            let q = read_u32_as_usize(c + 0xd7 + 4 * k);
            if q != 0 && name_matches(q, 4, primeiro) && name_matches(q, 8, segundo) && name_matches(q, 0xc, comum) {
                crate::comparacao::ramo("cqh_9 jogador do clube pelo nome");
                return q; // cqh9: o do elenco pelo nome
            }
        }
    }
    let t = read_u32_as_usize(c + 0xcf);
    if t == 0 {
        return 0;
    }
    if name_matches(t, 4, primeiro) && name_matches(t, 8, segundo) && name_matches(t, 0xc, comum) {
        return t;
    }
    0
}

/// Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_player_setup_find_staff(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32) -> usize>(IDX_PLAYER_SETUP_FIND_STAFF)(primeiro, comum, segundo, clube, no_elenco);
    let n = player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco);
    PLACAR.registrar(r == n, || format!("player_setup_find_staff(clube {:x}, elenco {}) original={:x} rust={:x}", clube, no_elenco as u8, r, n));
    r
}

/// copies the next quoted text of the line (skipping spaces and line breaks before it) and returns where to continue
/// Original: `player_setup_read_quoted_token` (0x7a5090, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a5090 } (cdecl, player_regen.cpp): `player_setup_read_quoted_token(origem, destino)`: copia para `destino` o
/// texto entre as primeiras aspas de `origem`: espaços e '\r' no começo do texto e '\n' antes dele são pulados; termina
/// nas aspas de fecho ou no zero (o destino ganha o zero) e devolve o ponteiro dois depois do fim.
pub(crate) unsafe extern "C" fn player_setup_read_quoted_token(origem: usize, destino: usize) -> usize {
    let (mut i, mut n, mut aberto) = (0usize, 0usize, false);
    loop {
        let c = *((origem + i) as *const u8);
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
            crate::comparacao::ramo("cqh_9 texto entre aspas");
            *((destino + n) as *mut u8) = c; // cqh9: o caractere do texto
            n += 1;
        }
    }
    *((destino + n) as *mut u8) = 0;
    origem + i + 1
}

/// Comparador: lado a lado no destino (até 0x100 bytes) e na devolução.
pub(crate) unsafe extern "C" fn comparar_player_setup_read_quoted_token(origem: usize, destino: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_PLAYER_SETUP_READ_QUOTED_TOKEN);
    if origem == 0 || destino == 0 {
        return o(origem, destino);
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(destino, 0x100)], || o(origem, destino), || player_setup_read_quoted_token(origem, destino));
    PLACAR.registrar(igual && r == n, || format!("player_setup_read_quoted_token({:x}) original={:x} rust={:x}", origem, r, n));
    r
}

/// Os tipos de linha do player_setup.cfg (o texto no .rdata e o bit), na ordem da original (strncmp de 100); o último,
/// AWOL (0xa6b2f8), dá 0x40, e nenhum, 0.
pub(crate) const CQH9_TIPOS: [(usize, u32); 14] = [
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

/// the type bit of a player_setup.cfg line (injury, manager, retirement, transfer, loan, doping, misconduct, passport,
/// tactics, colour blindness, language, absence)
/// Original: `player_setup_type_from_name` (0x7a3140, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a3140 } (cdecl, player_regen.cpp): `player_setup_type_from_name(nome)`: o bit do tipo da linha (INJURY 1,
/// MANAGER_CONF 2, RETIREMENT 0x200, INT_RETIREMENT 4, FUTURE_TRANSFER 8, LOAN 0x10, NANDROLONE e DRUGS 0x100,
/// MISCONDUCT 0x1000, FALSE_PASSPORT 0x800, ARSE 0x20, PRESET_TACTICS 0x2000, COLOURBLIND 0x400, GERMAN 0x80, AWOL 0x40;
/// nenhum 0), pela aplicação das linhas (0x7a3a10) na criação do jogo.
pub(crate) unsafe extern "C" fn player_setup_type_from_name(nome: usize) -> u32 {
    for &(texto, bit) in CQH9_TIPOS.iter() {
        if strncmp(nome, texto, 100) == 0 {
            crate::comparacao::ramo("cqh_9 tipo da linha");
            return bit;
        }
    }
    if strncmp(nome, 0xa6b2f8, 100) == 0 { 0x40 } else { 0 }
}

/// Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_player_setup_type_from_name(nome: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_PLAYER_SETUP_TYPE_FROM_NAME)(nome);
    let n = player_setup_type_from_name(nome);
    PLACAR.registrar(r == n, || format!("player_setup_type_from_name({:x}) original={:x} rust={:x}", nome, r, n));
    r
}

// ---------- 267a leva: as aposentadorias do dia: as anunciadas e as que vencem, a notícia da aposentadoria e a do jovem promovido (player_regen.cpp) ----------






/// Pointer to the retirement flags byte (+0xb) of person id's 79-byte record in [0xae2d0c].
///
/// [pt] O byte das marcas da aposentadoria da pessoa `id` (registros de 79 bytes em [0xae2d0c], +0xb).
pub(crate) unsafe fn retirement_flags_byte(id: i32) -> *mut u8 {
    read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((id as isize * 0x4f + 0xb) as usize) as *mut u8
}

/// The person's contract (0x4cf5a0) without storing it in [0xae1080].
///
/// [pt] O contrato da pessoa (0x4cf5a0 no gerente [0xae1050]), sem guardar em [0xae1080] (a 0x7a59e0 não guarda).
pub(crate) unsafe fn contract_of_uncached(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    f(read_u32_as_usize(G_CONTRACT_MANAGER), p)
}

/// Calls PLAYER_REGEN::remove_retirement (0x7a8690): removes the pending retirement (later ones shift up).
///
/// [pt] PLAYER_REGEN::remove_retirement (0x7a8690): tira a aposentadoria pendente (as seguintes sobem um lugar).
pub(crate) unsafe fn player_regen_remove_retirement(this: usize, item: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_PLAYER_REGEN_REMOVE_RETIREMENT);
    f(this, item);
}

/// Calls a thiscall(manager [g], person) routine.
///
/// [pt] Uma rotina thiscall(gerente [g], pessoa).
pub(crate) unsafe fn call_on_global_manager(rotina: usize, g: usize, p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(rotina);
    f(read_u32_as_usize(g), p);
}

/// Coach attribute from retired player: min(max(P, X) x k, cap), truncated.
///
/// [pt] O atributo do técnico que vira o jogador aposentado: min(max(P, X) × k, teto), truncado (as contas x87 da original
/// em f64; P e X inteiros).
pub(crate) fn coach_attribute_capped(pv: f64, x: f64, k: f64, teto: f64) -> u16 {
    let m = if pv > x { pv } else { x };
    let v = m * k;
    match_pl_ftol(if v < teto { v } else { teto }) as u16
}

/// Sets the three coach attributes (+0xa, +8, +0xc of non-player record nj) from the player's caps squared vs his
/// record fields, scaled and capped.
///
/// [pt] Os três atributos do técnico (+0xa, +8, +0xc do registro de não jogador `nj`) pelo jogador `p` (ficha `f`): a
/// os jogos pela seleção (+0x22, byte) ao quadrado (pow 0x945640/0x945ff0) contra +0xb, +9 e +0xd da ficha (o último com o quadrado ×
/// 0,75), × `k` e até os tetos.
pub(crate) unsafe fn set_coach_attributes(p: usize, f: usize, nj: usize, k: f64, tetos: [f64; 3]) {
    let r = *((p + 0x22) as *const u8) as f64;
    let q = game_pow(r, 2.0);
    core::ptr::write_unaligned((nj + 0xa) as *mut u16, coach_attribute_capped(q, read_i16(f + 0xb) as f64, k, tetos[0]));
    let q = game_pow(r, 2.0);
    core::ptr::write_unaligned((nj + 8) as *mut u16, coach_attribute_capped(q, read_i16(f + 9) as f64, k, tetos[1]));
    let q = game_pow(r, 2.0) * read_f64_at(0x969040);
    core::ptr::write_unaligned((nj + 0xc) as *mut u16, coach_attribute_capped(q, read_i16(f + 0xd) as f64, k, tetos[2]));
}

/// Retired player without non-player record may become a coach by random rolls; takes a spare non-player record, fixes
/// keeper fields and sets coach attributes.
///
/// [pt] O jogador aposentado sem registro de não jogador (+0x69) pode virar técnico: sorteio(+0x5b) acima de sorteio(100);
/// sorteio(ftol(+0xb da ficha + reputação³)) acima de sorteio(50000), ou (+0x8e da nação +0x1a) × reputação acima de
/// sorteio(200000) + 200000; com mais de 2 fichas reservadas (this +0x20): a primeira (this +0x1c; registros de 0x44
/// bytes em [0xae23c0]) vira o +0x69 e sai da reserva (0x7a82b0); goleiro (+0xf da ficha 20): +0x10/+0x11 trocados
/// quando +0x10 é maior e, com sorteio(4) nulo, +0x11 = sorteio(50) + 50; a normalização (0x539490) e os atributos do
/// técnico (0x9/7500 com +0xb da ficha acima de 7500; senão, com sorteio(+0x57) ≠ 0, 0,75/6000/3750; senão
/// 0,5/4500/1).
pub(crate) unsafe fn maybe_become_coach(this: usize, p: usize) {
    let r = *((p + 0x22) as *const u8);
    let ra = friendly_random(read_i8(p + 0x5b) as i32);
    if ra <= friendly_random(100) {
        return;
    }
    let f = read_u32_as_usize(p + 0x61);
    let t = match_pl_ftol(read_i16(f + 0xb) as f64 + game_pow(r as f64, 3.0));
    let r1 = friendly_random(t);
    let passa = r1 > friendly_random(50000) || { // cqd10: o sorteio de virar técnico
        let r3 = friendly_random(0x30d40).wrapping_add(0x30d40);
        let v = (read_i16(read_u32_as_usize(p + 0x1a) + 0x8e) as i32).wrapping_mul(*((p + 0x22) as *const u8) as i32);
        v > r3
    };
    if !passa || read_i32(this + 0x20) <= 2 {
        return;
    }
    crate::comparacao::ramo("cqd_10 vira tecnico");
    let id = read_i32(read_u32_as_usize(this + 0x1c));
    core::ptr::write_unaligned((p + 0x69) as *mut u32, read_u32_as_usize(G_NON_PLAYER_TABLE).wrapping_add((id as isize * 0x44) as usize) as u32);
    let reserva: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_PLAYER_REGEN_REMOVE_SPARE_NON_PLAYER_RECORD);
    reserva(this, read_i32(read_u32_as_usize(this + 0x1c)));
    if read_i8(read_u32_as_usize(p + 0x61) + 0xf) == 0x14 {
        let nj = read_u32_as_usize(p + 0x69);
        let (a, b) = (read_i8(nj + 0x10), read_i8(nj + 0x11));
        if a > b {
            *((nj + 0x11) as *mut i8) = a;
            *((read_u32_as_usize(p + 0x69) + 0x10) as *mut i8) = b;
            if friendly_random(4) == 0 {
                let v = (friendly_random(0x32) as u8).wrapping_add(0x32);
                *((read_u32_as_usize(p + 0x69) + 0x11) as *mut u8) = v;
            }
        }
    }
    let normalizar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_NON_PLAYER_ATTRIBUTES);
    normalizar(p);
    let f = read_u32_as_usize(p + 0x61);
    let nj = read_u32_as_usize(p + 0x69);
    if read_i16(f + 0xb) > 0x1d4c {
        set_coach_attributes(p, f, nj, read_f64_at(0x9689d8), [read_f64_at(0x9690b8), read_f64_at(0x9690b8), read_f64_at(0x96e298)]);
    } else if friendly_random(read_i8(p + 0x57) as i32) != 0 {
        set_coach_attributes(p, f, nj, read_f64_at(0x969040), [read_f64_at(0x96d300), read_f64_at(0x96d300), read_f64_at(0x96e290)]);
    } else {
        set_coach_attributes(p, f, nj, read_f64_at(0x968968), [read_f64_at(0x96e288), read_f64_at(0x96e288), read_f64_at(0x967890)]);
    }
}

/// Handles a due retirement item for person p: undoes it for young/contracted/transfer cases, otherwise posts news,
/// leaves jobs and frees the slot.
///
/// [pt] A aposentadoria que venceu (o item `item` da pessoa `p`).
pub(crate) unsafe fn process_due_retirement(this: usize, item: usize, p: usize, vaga: &mut [u32; 3]) {
    let id = read_i32(item);
    if read_i8(p + 0x18) < 0x1e && read_i8(item + 0x18) != 8 // cqd10: o jovem não se aposenta
        && (read_u32_as_usize(p + 0x39) != 0 || *retirement_flags_byte(read_i32(p)) & 0xf != 0) {
        crate::comparacao::ramo("cqd_10 aposentadoria desfeita");
        player_regen_remove_retirement(this, item);
        return;
    }
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    if ficha(read_u32_as_usize(G_CONTRACT_MANAGER), p) != 0 || oferta(read_u32_as_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        *retirement_flags_byte(id) &= 0xcf;
        player_regen_remove_retirement(this, item);
        return;
    }
    let antigo = read_i32(item + 4);
    let clube = read_u32_as_usize(p + 0x39);
    let mudou = if antigo != -1 { clube != 0 && read_i32(clube) != antigo } else { clube != 0 };
    let mut acertar = true;
    if mudou {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        if dias(0xae2c90, read_u32_as_usize(p + 0x3e) as u32, read_u32_as_usize(p + 0x42) as u32) < 0xd2 && read_i8(p + 0x18) < 0x26 {
            acertar = false;
            let c = contract_of_uncached(p);
            let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
            if c != 0 && listado(p, 0) == 0 && read_u32_as_usize(c + 0x45) == 0 {
                crate::comparacao::ramo("cqd_10 mudou de clube e fica");
                *retirement_flags_byte(id) &= 0xcf;
                player_regen_remove_retirement(this, item);
                return;
            }
        }
    }
    if acertar && read_u32_as_usize(p + 0x39) == 0 && read_i32(item + 4) != -1 {
        core::ptr::write_unaligned((item + 4) as *mut i32, -1);
    }
    // ponytail: a original empurra o motivo e a marca de anunciada em registradores com bytes de cima que sobram; a
    // 0x7a92e0 lê os bytes
    let noticia: unsafe extern "stdcall" fn(usize, u32, u32) = core::mem::transmute(FN_PLAYER_REGEN_POST_RETIREMENT_NEWS);
    noticia(p, *((item + 0x18) as *const u8) as u32, *((item + 0x19) as *const u8) as u32);
    let ofertas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_PENDING_OFFERS_FOR_STAFF);
    ofertas(read_u32_as_usize(G_CONTRACT_MANAGER), p);
    if read_u32_as_usize(p + 0x61) != 0 && read_u32_as_usize(p + 0x69) != 0 {
        if contract_of_uncached(p) != 0 {
            let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_BONUSES_AND_CLAUSES);
            limpar(contract_of_uncached(p));
        }
    } else {
        crate::comparacao::ramo("cqd_10 tira o contrato");
        call_on_global_manager(0x4d0b50, 0xae1050, p); // cqd10: tira o contrato
    }
    call_on_global_manager(0x775ba0, 0xb74318, p);
    let sm = read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856);
    if sm != 0 {
        let c = read_u32_as_usize(p + 0x39);
        if c != 0 {
            let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
            situacao(sm, p, c, 4, 2);
        }
        let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
        suspender(sm, p, 0);
        let c = read_u32_as_usize(p + 0x39);
        if c != 0 {
            *(read_u32_as_usize(sm + 0xdc).wrapping_add((read_i32(c) as isize * 9 + 8) as usize) as *mut u8) = 100;
        }
    }
    let mut banco = false;
    if read_u32_as_usize(p + 0x61) != 0 {
        let elenco: unsafe extern "stdcall" fn(usize) = core::mem::transmute(FN_PLAYER_REGEN_RETIRE_FROM_PLAYING_STAFF);
        crate::comparacao::ramo("cqd_10 sai do elenco");
        elenco(p); // cqd10: sai do elenco
        if read_u32_as_usize(p + 0x39) == 0 {
            call_on_global_manager(0x4d0b50, 0xae1050, p);
        }
        call_on_global_manager(0x4196a0, 0xad9c6c, p);
        call_on_global_manager(0x4197c0, 0xad9c6c, p);
        call_on_global_manager(0x56a090, 0xae2a30, p);
        if read_u32_as_usize(p + 0x61) != 0 {
            if read_u32_as_usize(p + 0x69) != 0 {
                banco = true;
            } else {
                maybe_become_coach(this, p);
            }
        }
    }
    if !banco {
        banco = read_u32_as_usize(p + 0x69) != 0 && read_u32_as_usize(p + 0x61) != 0;
    }
    if banco {
        // o jogador com registro de não jogador: a ficha vai para o banco dos regens e ele segue como não jogador
        crate::comparacao::ramo("cqd_10 jogador para o banco");
        call_on_global_manager(0x7c6650, 0xb74350, p);
        call_on_global_manager(0x55e1d0, 0xae28f0, p);
        call_on_global_manager(0x631f90, 0xb63d28, p);
        let guardar: unsafe extern "thiscall" fn(usize, i32, u32, u32) = core::mem::transmute(FN_PLAYER_REGEN_ADD_PLAYER_DATA_TO_POOL);
        let pais = read_u32_as_usize(p + 0x1a);
        if pais == 0 {
            let n = friendly_random(read_i32(0xae2350));
            guardar(this, read_i32(p), n as u32, u32::MAX);
        } else if read_u32_as_usize(p + 0x1e) == 0 {
            guardar(this, read_i32(p), read_u32_as_usize(pais) as u32, u32::MAX);
        } else {
            guardar(this, read_i32(p), read_u32_as_usize(pais) as u32, read_u32_as_usize(read_u32_as_usize(p + 0x1e)) as u32);
        }
        core::ptr::write_unaligned((p + 0x61) as *mut u32, 0);
        *retirement_flags_byte(read_i32(item)) &= 0xcf;
        *((p + 0x5f) as *mut u8) = 1;
        if read_u32_as_usize(p + 0x39) != 0 {
            let c = contract_of_uncached(p);
            if c != 0 {
                let inicial: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_GENERATE_INITIAL_CONTRACT);
                inicial(c, p, 0);
            }
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    // a vaga para o regen: hoje (até 29 anos) ou de 1 a 65 dias depois (se pass_ball do dia 365, sorteio(35) do ano
    // seguinte)
    vaga[0] = read_i32(item) as u32;
    let hoje = read_u32_as_usize(G_CURRENT_DATE) as u32;
    let (dia, ano) = if read_i8(p + 0x18) < 0x1e {
        (hoje as u16, read_i16(G_CURRENT_DATE_YEAR) as u16)
    } else {
        let v = (friendly_random(0x41) as u32).wrapping_add(hoje).wrapping_add(1) as u16; // cqd10: o dia do regen
        crate::comparacao::ramo("cqd_10 dia do regen sorteado");
        if v as i16 <= 0x16d {
            (v, read_i16(G_CURRENT_DATE_YEAR) as u16)
        } else {
            (friendly_random(0x23) as u16, (read_i16(G_CURRENT_DATE_YEAR) as u16).wrapping_add(1))
        }
    };
    vaga[1] = (vaga[1] & 0xffff_0000) | dia as u32;
    vaga[1] = (vaga[1] & 0xffff) | (ano as u32) << 16;
    let m = retirement_flags_byte(read_i32(item));
    *m = (*m & 0xef) | 0x20;
    let guardar_vaga: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(FN_PLAYER_REGEN_QUEUE_STAFF_FOR_REGEN);
    let nj = read_u32_as_usize(p + 0x69);
    if nj != 0 {
        let limpar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_CLEAR_MANAGER_RECORD);
        limpar(read_u32_as_usize(G_STAFF_RECORDS), nj);
        let c = read_u32_as_usize(p + 0x39);
        let cargo = |p: usize| read_i8(p + 0x3d);
        let sair = if c == 0 || cargo(p) == 5 {
            true
        } else {
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if humano(c, 1, 1, 0, 0) != 0 && !matches!(cargo(p), 1 | 3 | 2 | 4) {
                true
            } else if friendly_random(4) != 0 {
                false
            } else {
                !matches!(cargo(p), 1 | 3 | 2 | 4 | 0xa | 9)
            }
        };
        if sair {
            let cargos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(FN_PLAYER_REGEN_RETIRE_FROM_NON_PLAYING_STAFF);
            cargos(p);
            guardar_vaga(this, vaga.as_ptr());
        } else {
            crate::comparacao::ramo("cqd_10 refeito como nao jogador");
            let refazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_PLAYER_REGEN_REGENERATE_AS_NON_PLAYER);
            refazer(this, p);
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    guardar_vaga(this, vaga.as_ptr());
    player_regen_remove_retirement(this, item);
}

/// the pending retirements of the day: drops those of young players with a club, of players with a pending contract,
/// under offer or recently moved; retires (news, contract, national team, squad), may turn the player into a coach
/// (attributes from reputation and record), stores the record in the regen bank or queues the regen slot, and announces
/// the future ones
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
/// (0x775ba0), as listas de observação, o elenco (0x7aace0), as faltas (0x4196a0, 0x4197c0) e a contest (0x56a090); o
/// jogador pode virar técnico; com ficha e registro de não jogador, a ficha vai para o banco (0x7a8350) e o contrato
/// novo (0x85e960); senão a vaga para o regen (0x7a87d0) e, com registro de não jogador, sai dos cargos (0x7aaeb0) ou
/// é refeito como não jogador (0x7ab340). Não vencida: com a data do anúncio passada e sem anúncio, a notícia do
/// anúncio (0x7aa340, se as marcas & 0x30 não forem 0x10), a marca de anunciada e as marcas 0x10; senão para. No fim
/// ordena as vagas para o regen ([this+0x10], [this+0x14] de 12 bytes) com 0x7ae240.
pub(crate) unsafe extern "thiscall" fn process_retirements(this: usize) {
    let mut vaga = [0u32; 3]; // a pessoa e a data (dia, ano, bissexto)
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    padrao(vaga.as_mut_ptr() as usize + 4);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32_as_usize(this + 4), read_u32_as_usize(this + 8) as u32, 0x1a, 0x7ae2a0);
    let n = read_i32(this + 8);
    let mut item = read_u32_as_usize(this + 4);
    let mut i = 0i32;
    while i < n {
        let hoje = read_u32_as_usize(G_CURRENT_DATE) as u32;
        let (dia, ano) = (hoje as u16 as i16, (hoje >> 16) as u16);
        let (vdia, vano) = (read_i16(item + 0x10), read_i16(item + 0x12) as u16);
        if ano > vano || (ano == vano && dia > vdia) {
            let p = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((read_i32(item) as isize * 0x6e) as usize);
            process_due_retirement(this, item, p, &mut vaga);
        } else {
            if *((item + 0x19) as *const u8) != 0 {
                break;
            }
            let (adia, aano) = (read_i16(item + 8), read_i16(item + 0xa) as u16);
            if !(ano > aano || (ano == aano && dia > adia)) {
                break;
            }
            crate::comparacao::ramo("cqd_10 aposentadoria anunciada");
            let id = read_i32(item);
            if *retirement_flags_byte(id) & 0x30 != 0x10 {
                let anunciar: unsafe extern "stdcall" fn(usize, u32) = core::mem::transmute(FN_PLAYER_REGEN_POST_RETIREMENT_ANNOUNCEMENT_NEWS);
                // ponytail: a original empurra o motivo no edx com bytes de cima que sobram; a 0x7aa340 lê o byte
                anunciar(read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as isize * 0x6e) as usize), *((item + 0x18) as *const u8) as u32);
            }
            let id = read_i32(item);
            *((item + 0x19) as *mut u8) = 1;
            let m = retirement_flags_byte(id);
            *m = (*m & 0xdf) | 0x10;
            item += 0x1a;
        }
        i += 1;
    }
    qsort(read_u32_as_usize(this + 0x10), read_u32_as_usize(this + 0x14) as u32, 0xc, 0x7ae240);
}

// ponytail: chama as notícias (alocam e entregam), o regen como não jogador, os contratos novos e as listas: o caminho
// só se sabe rodando e não pode rodar duas vezes. Só a original; a cópia leva os ramos e a carreira a prova.
pub(crate) unsafe extern "thiscall" fn comparar_process_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_RETIREMENTS)(this)
}

/// news item 0xbba about the retirement for the managers (all, the country, the division, the club or the interested
/// clubs, by fame) and the club insurance payout when injury retires a valuable player
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
pub(crate) unsafe extern "stdcall" fn player_regen_post_retirement_news(p: usize, motivo: u32, anunciada: u32) {
    if p == 0 {
        return;
    }
    if read_u32_as_usize(p + 0x61) != 0 && matches!(read_i8(p + 0x3d), 8 | 5) {
        return;
    }
    let clube = read_u32_as_usize(p + 0x39);
    if clube == 0 {
        return;
    }
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if humano(clube) == 0 {
        return;
    }
    crate::comparacao::ramo("cqd_10 noticia da aposentadoria");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    construir(nw, 0xbba, 0);
    gravar(nw, 0, read_u32_as_usize(p) as u32);
    for k in 1..4usize {
        gravar(nw, k as i8, read_u32_as_usize(read_u32_as_usize(p + 4 * k) + 0x33) as u32);
    }
    gravar(nw, 4, motivo as u8 as i8 as i32 as u32);
    gravar(nw, 5, anunciada as u8 as i8 as i32 as u32);
    let c = read_u32_as_usize(p + 0x39);
    gravar(nw, 6, if c != 0 { read_u32_as_usize(c) as u32 } else { u32::MAX });
    gravar(nw, 7, if *((p + 0x22) as *const u8) != 0 { read_u32_as_usize(read_u32_as_usize(p + 0x1a)) as u32 } else { u32::MAX });
    let cargo = match read_i8(p + 0x3d) {
        0xf => 8,
        0xd => 6,
        v => v,
    };
    gravar(nw, 8, cargo as i32 as u32);
    gravar(nw, 9, *((p + 0x22) as *const u8) as u32);
    gravar(nw, 0xa, read_i8(p + 0x28) as i32 as u32);
    gravar(nw, 0xb, (read_u32_as_usize(p + 0x61) != 0 && read_u32_as_usize(p + 0x69) != 0) as u32);
    gravar(nw, 0xc, read_i8(p + 0x18) as i32 as u32);
    gravar(nw, 0xd, (read_u32_as_usize(p + 0x69) != 0) as u32);
    gravar(nw, 0xe, (read_u32_as_usize(p + 0x61) != 0) as u32);
    let mut clubes = [0usize; 30];
    let tm = read_u32_as_usize(G_TRANSFER_MANAGER);
    let n = if tm != 0 && read_u32_as_usize(p + 0x61) != 0 {
        let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
        interessados(tm, p, 0xa, clubes.as_mut_ptr(), 0x1e) as i8
    } else {
        0
    };
    let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    let da_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mn = read_u32_as_usize(G_NEWS);
    let pais = read_u32_as_usize(p + 0x1a);
    let f = read_u32_as_usize(p + 0x61);
    let nj = read_u32_as_usize(p + 0x69);
    let cargo5 = || read_i8(p + 0x3d) == 5;
    if pais != 0 && read_i8(pais + 0x85) > 8
        && ((f != 0 && read_i16(f + 0xd) > 0x2134) || (nj != 0 && read_i16(nj + 0xc) > 0x222e && cargo5())) {
        todos(mn, nw);
    } else if (f != 0 && read_i16(f + 0xd) > 0x1d4c) || (nj != 0 && read_i16(nj + 0xa) > 0x1c52 && cargo5()) {
        do_pais(mn, nw, pais);
        let c = read_u32_as_usize(p + 0x39);
        if c != 0 && read_u32_as_usize(c + 0x53) != read_u32_as_usize(p + 0x1a) {
            do_pais(read_u32_as_usize(G_NEWS), nw, read_u32_as_usize(c + 0x53));
        }
    } else {
        let c = read_u32_as_usize(p + 0x39);
        if c != 0 && read_u32_as_usize(c + 0x57) != 0 && cargo5() {
            da_divisao(mn, nw, read_u32_as_usize(c + 0x57));
        } else if f == 0 {
            ao_clube(mn, nw, c);
        } else if n > 0 {
            for k in 0..n as usize {
                ao_clube(read_u32_as_usize(G_NEWS), nw, clubes[k]);
            }
        }
    }
    if motivo as u8 == 8 {
        let f = read_u32_as_usize(p + 0x61);
        if f != 0 && read_i16(f + 0xb) > 0x109a {
            let c = contract_of_uncached(p);
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            if c != 0 {
                let clube = read_u32_as_usize(G_CLUB_TABLE).wrapping_add((read_i32(c + 4) as isize * 0x245) as usize);
                if clube != 0 {
                    crate::comparacao::ramo("cqd_10 seguro da aposentadoria");
                    let seguro: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_INSURANCE_PAYOUT);
                    seguro(clube, p, read_u32_as_usize(p + 0x52) as u32);
                }
            }
        }
    }
}

// ponytail: aloca e entrega a notícia (0x77bbe0, 0x786780...). Só a original; a cópia leva o ramo.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_post_retirement_news(p: usize, motivo: u32, anunciada: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, u32, u32)>(IDX_PLAYER_REGEN_POST_RETIREMENT_NEWS)(p, motivo, anunciada)
}

/// news item 0xbce about the youth player joining the club, with the assistant manager or a coach verdict on his
/// ability
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
    crate::comparacao::ramo("cqd_10 noticia do jovem promovido");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let vista: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    let (mut v1, mut v2) = (0i8, 0i8);
    let mut quem = 0usize;
    construir(nw, 0xbce, 0);
    gravar(nw, 0, read_u32_as_usize(j) as u32);
    for k in 1..4usize {
        gravar(nw, k as i8, read_u32_as_usize(read_u32_as_usize(j + 4 * k) + 0x33) as u32);
    }
    gravar(nw, 4, read_u32_as_usize(c) as u32);
    gravar(nw, 5, read_i8(j + 0x18) as i32 as u32);
    gravar(nw, 6, (read_i8(c + 0x7f) > 0xf) as u32);
    let mut atual = 0usize;
    let mut k = -1i8;
    loop {
        if atual != 0 {
            break;
        }
        let t = if k == -1 { read_u32_as_usize(c + 0xd3) } else { read_u32_as_usize(c + 0x19f + 4 * k as usize) };
        if t != 0 {
            let r = friendly_random(0x9c4);
            let v = r.wrapping_add(read_i16(c + 0x80) as i32 / 2);
            let x = match_pl_ftol(v as f64 + read_i8(c + 0x7f) as f64 * read_f64_at(0x96afc8));
            // ponytail: a original empurra o eax inteiro do _ftol e o da primeira vista; a 0x6e86d0 lê a palavra
            let a = vista(t, j, c, 2, x as i16);
            let b = vista(t, j, c, 1, a);
            let rep = read_i16(c + 0x80) as i32;
            let alto = rep / 50 + 0x4b;
            if a as i32 > alto {
                v1 = 2;
                quem = t;
            } else if a as i32 > rep / 100 + 0x32 {
                v1 = 1;
                quem = t;
            }
            if b as i32 > alto {
                v2 = 2;
                quem = t;
            } else if b as i32 > rep / 100 + 0x32 {
                v2 = 1;
                quem = t;
            }
            atual = quem;
        }
        k += 1;
        if k >= 5 {
            break;
        }
    }
    if atual != 0 {
        gravar(nw, 7, (atual == read_u32_as_usize(c + 0xd3)) as u32);
        gravar(nw, 8, read_u32_as_usize(atual) as u32);
        for k in 1..4usize {
            gravar(nw, 8 + k as i8, read_u32_as_usize(read_u32_as_usize(atual + 4 * k) + 0x33) as u32);
        }
        gravar(nw, 0xc, v1 as i32 as u32);
        gravar(nw, 0xd, v2 as i32 as u32);
    } else {
        for k in 7..14i8 {
            gravar(nw, k, u32::MAX);
        }
    }
    let alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    alcance(read_u32_as_usize(G_NEWS), nw, c, 0, 0, 0, 0);
}

// ponytail: aloca e entrega a notícia e sorteia a opinião: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_post_youth_promotion_news(j: usize, c: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize)>(IDX_PLAYER_REGEN_POST_YOUTH_PROMOTION_NEWS)(j, c)
}

// ---------- 258a leva: o regen no clube: a vaga no elenco, o valor e o contrato do jovem; a aposentadoria pela lesão e a notícia da aposentadoria anunciada (player_regen.cpp) ----------



/// First free (zero) slot of the club's 50 squad slots (+0xd7); 0x32 if none.
///
/// [pt] A primeira vaga livre (dword 0) das 50 do elenco do clube (+0xd7); 0x32 sem vaga.
pub(crate) unsafe fn first_free_squad_slot(clube: usize) -> usize {
    (0..0x32usize).find(|&k| read_i32(clube + club::SQUAD + 4 * k) == 0).unwrap_or(0x32)
}

/// puts the regen in the first free slot of the 50 in the squad, with the value (0x4d0c60; at most 5000 at a club with
/// +0x64 = 3), the contract end (below 19 years, two years from now; otherwise random(6) + 1) in the club's season end
/// month on a random day, the contract, the default wage and, outside the human club, an end at most five years ahead;
/// returns 1 (0 without a slot)
/// Original: `player_regen_sign_for_club` (0x7a6f10, player_regen.cpp, stdcall, ret 8).
///
/// [pt] { 0x7a6f10 } (stdcall, ret 8, player_regen.cpp, quadro SEH só das datas locais): `player_regen_sign_for_club(pessoa,
/// clube)`, pela PLAYER_REGEN::regenerate_as_youth_player (0x7abac0) em todo regen jovem: na primeira vaga livre das 50
/// do elenco (+0xd7; sem vaga, 0) a pessoa, o clube dela (+0x39) e +0x4e = 0; o valor (+0x52) pela 0x4d0c60 (clube, 1,
/// −1, 0), o elenco (0x540980 com 1), uma vez em quatro (sorteio(4) = 0) a 0x547d90 (pessoa, clube, 0); clube com +0x64
/// = 3: valor até 5000. O fim do contrato (+0x46): abaixo de 19 anos (+0x18), o ano de hoje + 2; senão + sorteio(6) +
/// 1; no mês do fim da temporada do clube (0x4d45f0) e no dia sorteio(28) + 1 (0x549f70). O contrato (0x4c7950 com o
/// valor); clube humano (0x600a50 com 1, 1): para os reservas (0x4cc0f0) e os bits +0x4f (0x60 em cima) e +0x35 (1 em
/// baixo) do contrato. Com contrato: o salário padrão (0x8626e0), +0x3a = 0 e, fora do humano, o fim (+0x2f) até o ano de
/// hoje + 5 (com o dia +0x2d = 0). No fim os números do elenco ([0xdd7ef0], 0x85add0 e 0x85a470). Devolve 1.
pub(crate) unsafe extern "stdcall" fn player_regen_sign_for_club(pessoa: usize, clube: usize) -> u8 {
    let k = first_free_squad_slot(clube);
    if k == 0x32 {
        return 0;
    }
    let gerente = read_i32(G_CONTRACT_MANAGER) as usize;
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let f547d90: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_CLUB);
    let mes_do_fim: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SEASON_END_MONTH);
    let data_dma: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> *mut u8 = core::mem::transmute(FN_DATE_DATE_549F70);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, *const u8) -> *mut u8 = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let contrato_novo: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_STAFF_CONTRACT);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_MOVE_TO_RESERVES);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let salario: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_DEFAULT_WAGE);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
    crate::comparacao::ramo("cpv_9 regen contratado");
    write_i32(clube + club::SQUAD + 4 * k, pessoa as i32);
    write_i32(pessoa + staff::CLUB_JOB, clube as i32);
    write_i32(pessoa + staff::WAGE, 0);
    let v = valor(gerente, pessoa, clube, 1, -1, 0);
    write_i32(pessoa + staff::VALUE, v); // cpv9: o valor do regen
    elenco(pessoa, 1);
    if sorteio(4) == 0 { // cpv9: uma vez em quatro
        crate::comparacao::ramo("cpv_9 uma em quatro");
        f547d90(pessoa, clube, 0);
    }
    if *((clube + club::PROFESSIONAL_STATUS) as *const u8) == 3 && read_i32(pessoa + staff::VALUE) > 0x1388 {
        write_i32(pessoa + staff::VALUE, 0x1388);
    }
    // ponytail: a original grava o ano na palavra de baixo do argumento `clube` e empurra o dword; a 0x549f70 lê 16 bits
    let ano = *(G_CURRENT_DATE_YEAR as *const u16) as u32;
    let (mut d, mut ret) = ([0u8; 8], [0u8; 8]);
    let dia_e_ano = if *((pessoa + staff::YEAR_OF_BIRTH) as *const i8) < 0x13 {
        crate::comparacao::ramo("cpv_9 contrato do jovem");
        let a = ano.wrapping_add(2); // cpv9: o jovem por dois anos
        let mes = mes_do_fim(clube);
        (sorteio(0x1c), mes, a)
    } else {
        let a = (sorteio(6) as u32).wrapping_add(ano).wrapping_add(1);
        let mes = mes_do_fim(clube);
        (sorteio(0x1c), mes, a)
    };
    let dia = (dia_e_ano.0 as u8).wrapping_add(1) as i8 as i16 as u16 as u32;
    let p = data_dma(d.as_mut_ptr(), dia, dia_e_ano.1, dia_e_ano.2, 0xffff_ffff);
    copia(pessoa + staff::DATE_EXPIRES_CLUB, ret.as_mut_ptr(), p);
    contrato_novo(gerente, pessoa, read_i32(pessoa + staff::VALUE) as u32);
    if humano(clube, 1, 1, 0, 0) != 0 {
        reservas(read_i32(G_CONTRACT_MANAGER) as usize, pessoa, clube);
        let c = contrato(read_i32(G_CONTRACT_MANAGER) as usize, pessoa);
        *((c + 0x4f) as *mut u8) = *((c + 0x4f) as *const u8) & 0xf | 0x60;
        let c = contrato(read_i32(G_CONTRACT_MANAGER) as usize, pessoa);
        *((c + 0x35) as *mut u8) = *((c + 0x35) as *const u8) & 0xc1 | 1;
    }
    let c = contrato(read_i32(G_CONTRACT_MANAGER) as usize, pessoa);
    if c != 0 {
        salario(c, pessoa);
        *((c + 0x3a) as *mut u8) = 0;
        if humano(clube, 1, 1, 0, 0) == 0 {
            let a = (*(G_CURRENT_DATE_YEAR as *const u16)).wrapping_add(5);
            if core::ptr::read_unaligned((c + 0x2f) as *const u16) > a {
                core::ptr::write_unaligned((c + 0x2f) as *mut u16, a);
                core::ptr::write_unaligned((c + 0x2d) as *mut u16, 0);
            }
        }
    }
    if usa_numeros(read_i32(G_SQUAD_MANAGER) as usize, clube, 0) != 0 {
        dar_numero(read_i32(G_SQUAD_MANAGER) as usize, clube, 0);
    }
    1
}

/// Comparador: o contrato novo aloca e o valor sorteia: só a original, conferindo o plano (a vaga, o clube da pessoa, a
/// devolução e o +0x3a do contrato).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_sign_for_club(pessoa: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_PLAYER_REGEN_SIGN_FOR_CLUB);
    let k = first_free_squad_slot(clube);
    let r = o(pessoa, clube);
    if k == 0x32 {
        PLACAR.registrar(r == 0, || format!("player_regen_sign_for_club({:x}, {:x}) sem vaga original={}", pessoa, clube, r));
        return r;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_i32(G_CONTRACT_MANAGER) as usize, pessoa);
    let ok = r == 1 && read_i32(clube + club::SQUAD + 4 * k) == pessoa as i32 && read_i32(pessoa + staff::CLUB_JOB) == clube as i32 && (c == 0 || *((c + 0x3a) as *const u8) == 0);
    PLACAR.registrar(ok, || format!("player_regen_sign_for_club({:x}, {:x}) vaga {} original={}", pessoa, clube, k, r));
    r
}

/// removes the person's pending retirement and adds the new one, through injury (reason 8), random(days / 2) + 1 days
/// ahead
/// Original: `PLAYER_REGEN::schedule_injury_retirement` (0x7a7f20, player_regen.cpp, thiscall, ret 8).
///
/// [pt] { 0x7a7f20 } (thiscall, ret 8, player_regen.cpp, quadro SEH só das datas locais):
/// `PLAYER_REGEN::schedule_injury_retirement(pessoa, dias)`, pela lesão nova (0x62d250) que encerra a carreira: tira a
/// aposentadoria pendente da pessoa (a primeira com o id dela na tabela de 26 bytes [this+4], conta +8; 0x7a8690) e
/// acrescenta (0x7a8510) a nova: id, id do clube (+0x39, ou −1), a data de hoje + sorteio(dias / 2) + 1 (0x54c5b0) em
/// +8 e de novo em +0x10, o motivo 8 (+0x18) e 0 (+0x19).
pub(crate) unsafe extern "thiscall" fn schedule_injury_retirement(this: usize, pessoa: usize, dias: i16) {
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_PLAYER_REGEN_REMOVE_RETIREMENT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *mut u8 = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, *const u8) -> *mut u8 = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_PLAYER_REGEN_ADD_RETIREMENT);
    let n = read_i32(this + 8);
    if pessoa != 0 && n > 0 {
        let base = read_i32(this + 4) as usize;
        let id = read_i32(pessoa);
        if let Some(i) = (0..n as usize).find(|&i| read_i32(base + 0x1a * i) == id) {
            crate::comparacao::ramo("cpv_9 tira a aposentadoria pendente");
            tirar(this, base + 0x1a * i);
        }
    }
    let mut r = [0u8; 0x1a];
    let clube = read_i32(pessoa + staff::CLUB_JOB) as usize;
    let rp = r.as_mut_ptr();
    core::ptr::write_unaligned(rp as *mut i32, read_i32(pessoa));
    core::ptr::write_unaligned(rp.add(4) as *mut i32, if clube == 0 { -1 } else { read_i32(clube) });
    let q = sorteio(dias as i32 / 2);
    let (mut t, mut t2) = ([0u8; 8], [0u8; 8]);
    let d = mais(0xae2c90, t.as_mut_ptr(), q.wrapping_add(1) as u32);
    copia(rp.add(8), t2.as_mut_ptr(), d);
    copia(rp.add(0x10), t2.as_mut_ptr(), rp.add(8));
    r[0x18] = 8;
    r[0x19] = 0;
    acrescentar(this, r.as_ptr());
}

/// Comparador: lado a lado no estado do sorteio, na tabela (ponteiro, conta e capacidade, e as entradas até a
/// capacidade). Com a tabela cheia (conta = capacidade − 1, a 0x7a8510 realoca): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_injury_retirement(this: usize, pessoa: usize, dias: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i16)>(IDX_SCHEDULE_INJURY_RETIREMENT);
    let (n, cap) = (read_i32(this + 8), read_i32(this + 0xc));
    if pessoa == 0 || n >= cap.wrapping_sub(1) || cap <= 0 {
        return o(this, pessoa, dias);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 4, 12));
    reg.push((read_i32(this + 4) as usize, cap as usize * 0x1a));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, dias), || crate::aleatorio::sem_contar(|| schedule_injury_retirement(this, pessoa, dias)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("schedule_injury_retirement({:x}, {}) diferencas em {:x?}", pessoa, dias, dif)
    });
}

/// builds news 0xbbb (id, names, reason, contract club, age) and sends it to the managers of the interested clubs
/// Original: `player_regen_post_retirement_announcement_news` (0x7aa340, player_regen.cpp, stdcall, ret 8).
///
/// [pt] { 0x7aa340 } (stdcall, ret 8, player_regen.cpp, quadro SEH sem destrutor): `player_regen_post_retirement_announcement_news(
/// pessoa, motivo)`, pela PLAYER_REGEN::process_retirements (0x7a59e0): fora do jogador (+0x61) com motivo +0x3d 8 ou
/// 5, a notícia 0xbbb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os nomes (+0x33 de +4, +8 e +0xc), 4 =
/// motivo, 5 = o id do clube do contrato (0x4cf5a0, guardado em [0xae1080]; clube do registro de 0x245 bytes em
/// [0xae23b4] pelo +4; contrato de tipo +0x35 & 0x3f = 5 ou sem clube: −1) e 6 = a idade (+0x18); vai para os técnicos
/// dos até 30 clubes interessados ([0xde1f54], 0x8d1da0 com 10).
pub(crate) unsafe extern "stdcall" fn player_regen_post_retirement_announcement_news(pessoa: usize, motivo: i8) {
    if pessoa == 0 {
        return;
    }
    if read_i32(pessoa + staff::PLAYER) != 0 && matches!(*((pessoa + staff::JOB_FOR_CLUB) as *const u8), 8 | 5) {
        return;
    }
    crate::comparacao::ramo("player_regen_post_retirement_announcement_news");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbb, 0);
    gravar(n, 0, read_i32(pessoa) as u32);
    for (k, o) in [(1i8, 4usize), (2, 8), (3, 0xc)] {
        gravar(n, k, read_i32(read_i32(pessoa + o) as usize + 0x33) as u32);
    }
    gravar(n, 4, motivo as i32 as u32);
    let c = contrato(read_i32(G_CONTRACT_MANAGER) as usize, pessoa);
    write_i32(0xae1080, c as i32);
    let mut clube = -1i32;
    if c != 0 && *((c + 0x35) as *const u8) & 0x3f != 5 {
        let idx = read_i32(c + 4) as u32;
        let registro = (read_i32(G_CLUB_TABLE) as u32).wrapping_add(idx.wrapping_mul(0x245)) as usize;
        if registro != 0 {
            clube = read_i32(registro);
        }
    }
    gravar(n, 5, clube as u32);
    gravar(n, 6, *((pessoa + staff::YEAR_OF_BIRTH) as *const i8) as i32 as u32);
    let mut lista = [0usize; 0x1e];
    let q = if read_i32(G_TRANSFER_MANAGER) != 0 { interessados(read_i32(G_TRANSFER_MANAGER) as usize, pessoa, 0xa, lista.as_mut_ptr(), 0x1e) } else { 0 };
    for i in 0..(q as i8).max(0) as usize {
        ao_tecnico(read_i32(G_NEWS) as usize, n, lista[i]);
    }
}

/// Comparador: a notícia vai para as caixas dos técnicos: só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_post_retirement_announcement_news(pessoa: usize, motivo: i8) {
    original_fn::<unsafe extern "stdcall" fn(usize, i8)>(IDX_PLAYER_REGEN_POST_RETIREMENT_ANNOUNCEMENT_NEWS)(pessoa, motivo)
}

// ---------- 257a leva: as fichas de treino iniciais dos jogadores, o banco de jogadores livres do regen e se há reserva de não jogador (training_manager.cpp, player_regen.cpp) ----------





/// appends to the pool of free player records the player data id of a person who stops playing, with the nation and
/// second nation, growing the table by 250
/// Original: `PLAYER_REGEN::add_player_data_to_pool` (0x7a8350, player_regen.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7a8350 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_player_data_to_pool(pessoa, nacao,
/// segunda)`: acrescenta ao banco de jogadores livres (16 bytes em [this+0x3c], conta +0x40, capacidade +0x44) o id da
/// ficha de jogador (+0x61) da pessoa de índice `pessoa` (registro de 0x6e bytes em [0xae23bc]) em +0, a nação em +4 e
/// a segunda nação em +8 (+0xc fica como está). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória,
/// caixa 0xa0a e fim). Pela process_retirements (0x7a59e0), quando um jogador que também tem ficha de não jogador
/// deixa de jogar.
pub(crate) unsafe extern "thiscall" fn add_player_data_to_pool(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    if read_usize(this + 0x40) == read_usize(this + 0x44).wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + 0x3c), read_usize(this + 0x44).wrapping_add(250) << 4);
        write_usize(this + 0x3c, p);
        write_usize(this + 0x44, read_usize(this + 0x44).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa0a);
        }
    }
    crate::comparacao::ramo("cpu_9 banco de jogadores");
    let registro = read_usize(G_STAFF_TABLE).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize);
    let item = read_usize(this + 0x3c).wrapping_add(read_usize(this + 0x40) << 4);
    write_usize(item, read_usize(read_usize(registro + 0x61)));
    write_usize(item + 4, nacao as usize);
    write_usize(item + 8, segunda as usize);
    write_usize(this + 0x40, read_usize(this + 0x40).wrapping_add(1));
}

/// Comparador: o id da ficha antes; a original acrescenta (pode realocar); confere a conta e o item novo.
pub(crate) unsafe extern "thiscall" fn comparar_add_player_data_to_pool(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    let n = read_usize(this + 0x40);
    let id = read_usize(read_usize(read_usize(G_STAFF_TABLE).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize) + 0x61)) as u32;
    original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32)>(IDX_ADD_PLAYER_DATA_TO_POOL)(this, pessoa, nacao, segunda);
    let item = read_usize(this + 0x3c).wrapping_add(n << 4);
    let g = [read_usize(item) as u32, read_usize(item + 4) as u32, read_usize(item + 8) as u32];
    PLACAR.registrar(read_usize(this + 0x40) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("add_player_data_to_pool({}, {}, {}) conta {} -> {}, item {:?} (id {})", pessoa, nacao, segunda, n, read_usize(this + 0x40), g, id)
    });
}

/// whether the spare non-player record list holds any entry
/// Original: `PLAYER_REGEN::has_spare_non_player_record` (0x7a6e20, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a6e20 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::has_spare_non_player_record()`: se a reserva de
/// fichas de não jogador (conta em +0x20) tem alguma. O eax leva os 24 bits altos da conta (setne no al), como na
/// original.
pub(crate) unsafe extern "thiscall" fn has_spare_non_player_record(this: usize) -> u32 {
    let c = read_u32_as_usize(this + 0x20) as u32;
    (c & 0xffff_ff00) | (c != 0) as u32
}

/// Comparador: a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_has_spare_non_player_record(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_SPARE_NON_PLAYER_RECORD)(this);
    let n = has_spare_non_player_record(this);
    PLACAR.registrar(r == n, || format!("has_spare_non_player_record({:x}) original={:x} rust={:x}", this, r, n));
    r
}

// ---------- 254a leva: as aposentadorias: quem se aposenta no começo do jogo, quem se aposenta pela idade e a lista inicial (player_regen.cpp) ----------






/// Looks up the person's contract (cached in [0xae1080]) and returns his contract with that club (0x4cf5f0).
///
/// [pt] O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) e, pelo clube dele (registro de 0x245 bytes pelo +4; sem
/// contrato, 0), o contrato da pessoa com esse clube (0x4cf5f0), que a rotina devolve.
pub(crate) unsafe fn contract_with_current_club(pessoa: usize) -> usize {
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = de(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let clube = if c != 0 { read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(c + 4).wrapping_mul(0x245)) } else { 0 };
    let com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    com(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa, clube)
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

/// Retirement age limit for non-players by job type (70+r10, 60+r5, 61+r5 or 55+r10), reproducing the original's stray
/// stack byte when no job.
///
/// [pt] O limite de idade de quem não é jogador, pelo tipo do cargo (+0x3d; sem ele +0x28; sem os dois, como a original,
/// o byte baixo do ponteiro da saída, que ela lê da vaga errada da pilha): 1, 2 e 4: 70 + sorteio(10); 8 e 9:
/// 60 + sorteio(5); os outros: 61 + sorteio(5) com clube (+0x39) ou seleção (+0x24), senão 55 + sorteio(10). Byte.
pub(crate) unsafe fn non_player_age_limit(pessoa: usize, saida: usize) -> i8 {
    let mut t = read_i8(pessoa + staff::JOB_FOR_CLUB);
    if t == 0 {
        t = read_i8(pessoa + staff::JOB_FOR_NATION);
        if t == 0 {
            t = saida as u8 as i8; // ponytail: a original lê aqui o byte baixo do argumento `saida`
        }
    }
    let (n, base) = match t {
        1 | 2 | 4 => (10, 0x46u8),
        8 | 9 => (5, 0x3c),
        _ => {
            if read_u32_as_usize(pessoa + staff::CLUB_JOB) != 0 || read_u32_as_usize(pessoa + staff::NATIONAL_JOB) != 0 {
                (5, 0x3d)
            } else {
                (10, 0x37)
            }
        }
    };
    (sortear(n) as u8).wrapping_add(base) as i8
}

/// Player retirement age limit from his record: 34, 30 or 31 by attribute bytes +0xf/+0x15/+0x14.
///
/// [pt] O limite de idade do jogador pela ficha: 34 com +0xf ≥ 15; senão 30 com +0x15 ≥ 15; senão 31 com +0x14 < 15;
/// senão 30 (bytes com sinal).
pub(crate) unsafe fn player_age_limit(j: usize) -> i8 {
    if read_i8(j + 0xf) >= 15 {
        0x22
    } else if read_i8(j + 0x15) >= 15 {
        0x1e
    } else if read_i8(j + 0x14) < 15 {
        0x1f
    } else {
        0x1e
    }
}

/// Fills the retirement output entry: reason (+0x18), +0x19 = 0, person id and club id (-1 without club).
///
/// [pt] A saída de quem se aposenta: o motivo (+0x18), +0x19 = 0, o id (+0) e o id do clube (+4; −1 sem clube).
pub(crate) unsafe fn fill_retirement_entry(pessoa: usize, saida: usize, motivo: u8) {
    *((saida + 0x18) as *mut u8) = motivo;
    *((saida + 0x19) as *mut u8) = 0;
    core::ptr::write_unaligned(saida as *mut u32, read_u32_as_usize(pessoa) as u32);
    let c = read_u32_as_usize(pessoa + staff::CLUB_JOB);
    core::ptr::write_unaligned((saida + 4) as *mut u32, if c != 0 { read_u32_as_usize(c) as u32 } else { u32::MAX });
}

/// Sets both output dates (+8 and +0x10) to tomorrow (day + 1, current year).
///
/// [pt] Amanhã (dia + 1, só a palavra) nas duas datas da saída (+8 e +0x10; ano em +0xa e +0x12).
pub(crate) unsafe fn set_dates_tomorrow(saida: usize) {
    let (dia, ano) = (read_u16(0xae2c90).wrapping_add(1), read_u16(0xae2c92));
    write_u16(saida + 8, dia);
    write_u16(saida + 0xa, ano);
    write_u16(saida + 0x10, dia);
    write_u16(saida + 0x12, ano);
}

/// For age retirement without a valid key nation: both dates today + random(20), wrapping to random(10) of next year.
///
/// [pt] Sem nação-chave válida na aposentadoria pela idade: hoje + sorteio(20) nas duas datas; passando de 365, o dia
/// sorteio(10) do ano seguinte.
pub(crate) unsafe fn set_dates_no_key_nation(saida: usize) {
    let a = (sortear(20) as u32).wrapping_add(read_u32_as_usize(G_CURRENT_DATE) as u32) as u16;
    let ano = read_u16(0xae2c92);
    if a as i16 > 0x16d {
        let d = sortear(10) as u16;
        write_u16(saida + 0x10, d);
        write_u16(saida + 0x12, ano.wrapping_add(1));
        write_u16(saida + 8, d);
        write_u16(saida + 0xa, ano.wrapping_add(1));
    } else {
        write_u16(saida + 8, a);
        write_u16(saida + 0xa, ano);
        write_u16(saida + 0x10, a);
        write_u16(saida + 0x12, ano);
    }
}

/// Sets the announcement and retirement dates by the key nation of the club or nation (season end + random), with
/// fallbacks.
///
/// [pt] As datas da aposentadoria na saída (+8 o anúncio, +0x10 a aposentadoria; só o dia e o ano, o +4 de cada data fica
/// como está), pela nação-chave do clube (0x669360) ou, sem clube, da nação (+0x1a, 0x669340); sem os dois, amanhã.
/// Índice 0..0x22: o anúncio em hoje + 1 + sorteio(95) (passando de 365, o dia sorteio(60) do ano seguinte); a
/// aposentadoria no fim de temporada da nação (+0x15 do registro de 72 bytes em 0xb63d60) + sorteio(30) (passando de
/// 365, sorteio(30) do ano seguinte; com +0x1d, um ano a mais); a aposentadoria antes do anúncio fica no anúncio, e a
/// menos de 45 dias dele (0x54cdb0) o anúncio vai para ela. Índice fora: `fora` (cada rotina tem a sua).
pub(crate) unsafe fn set_retirement_dates(pessoa: usize, saida: usize, fora: unsafe fn(usize)) {
    let clube = read_u32_as_usize(pessoa + staff::CLUB_JOB);
    let k = if clube != 0 {
        let pelo_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
        pelo_clube(clube) as u8 as i8
    } else {
        let nacao = read_u32_as_usize(pessoa + staff::NATION);
        if nacao == 0 {
            return set_dates_tomorrow(saida);
        }
        let pela_nacao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
        pela_nacao(nacao) as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return fora(saida);
    }
    crate::comparacao::ramo("cpr_8 datas pela nacao");
    let ano = read_u16(0xae2c92);
    let a = (sortear(95) as u32).wrapping_add(read_u32_as_usize(G_CURRENT_DATE) as u32).wrapping_add(1) as u16;
    let (dia1, ano1) = if a as i16 > 0x16d { (sortear(60) as u16, ano.wrapping_add(1)) } else { (a, ano) };
    write_u16(saida + 0xa, ano1);
    write_u16(saida + 8, dia1);
    let t = 0xb63d60usize.wrapping_add((k as usize) * 0x48);
    let d2 = (sortear(30) as u16).wrapping_add(read_u16(t + 0x15)); // o fim de temporada da nação
    write_u16(saida + 0x10, d2);
    let mais_um = *((t + 0x1d) as *const u8) != 0;
    if d2 as i16 > 0x16d {
        write_u16(saida + 0x10, sortear(30) as u16);
        write_u16(saida + 0x12, ano.wrapping_add(if mais_um { 2 } else { 1 }));
    } else {
        write_u16(saida + 0x12, if mais_um { ano.wrapping_add(1) } else { ano });
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t8 = [0u8; 8];
    let (da, aa) = (read_u16(saida + 0x12), read_u16(saida + 0xa));
    if da < aa || (da == aa && read_i16(saida + 0x10) < read_i16(saida + 8)) {
        copia(saida + 0x10, t8.as_mut_ptr(), saida + 8);
        return;
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if dias(saida + 0x10, read_u32_as_usize(saida + 8) as u32, read_u32_as_usize(saida + 0xc) as u32) >= 0x2d {
        return;
    }
    copia(saida + 8, t8.as_mut_ptr(), saida + 0x10);
}

/// at a new game, does the person with a club retire? Non-players by the job age limit; players by the profile limit,
/// the ability, the club competition and the contract status; the output (id, club, reason) and the announcement and
/// retirement dates by the key nation
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
pub(crate) unsafe extern "thiscall" fn player_regen_check_initial_retirement(_this: usize, pessoa: usize, saida: usize) -> u8 {
    if read_u32_as_usize(pessoa + staff::CLUB_JOB) == 0 {
        return 0;
    }
    let c2 = contract_with_current_club(pessoa);
    let estatuto = contract_status(c2);
    let idade = read_i8(pessoa + staff::YEAR_OF_BIRTH);
    let j = read_u32_as_usize(pessoa + staff::PLAYER);
    let motivo: u8 = 'm: {
        if j == 0 {
            if read_u32_as_usize(pessoa + staff::NON_PLAYER) != 0 && idade <= non_player_age_limit(pessoa, saida) {
                return 0;
            }
            break 'm 0x20;
        }
        let lim = player_age_limit(j);
        if idade <= lim {
            return 0;
        }
        let f = match_pl_ftol(read_f64_at(0x967890) - (idade as i32 - lim as i32) as f64 * read_f64_at(0x967880)) as i16;
        let v = match_pl_ftol(f as f64 * read_i16(j + 0xb) as f64) as i16;
        if idade > 0x2d {
            break 'm 1;
        }
        let clube = read_u32_as_usize(pessoa + staff::CLUB_JOB);
        let sorteia = if clube != 0 {
            read_u32_as_usize(clube + club::DIVISION) == 0
        } else {
            let n = read_u32_as_usize(pessoa + staff::NATION);
            n != 0 && *((n + 0x11c) as *const u8) == 0
        };
        if sorteia {
            if sortear(3) != 0 {
                break 'm 1;
            }
            if clube == 0 || estatuto >= 0 {
                return 0;
            }
            break 'm 1;
        }
        if v >= 0x1388 {
            return 0;
        }
        let hab = read_i16(j + 5);
        if hab > 0x50 {
            return 0; // cpr8: a habilidade alta segura o jogador
        }
        if hab < 0xa || v < 0x5dc {
            crate::comparacao::ramo("cpr_8 inicio habilidade");
            break 'm 1;
        }
        if clube != 0 {
            if estatuto < 0 {
                break 'm 1;
            }
            return 0;
        }
        if sortear(25 * read_i8(j + 0x3c) as i32) != 0 {
            return 0;
        }
        2
    };
    fill_retirement_entry(pessoa, saida, motivo);
    set_retirement_dates(pessoa, saida, set_dates_tomorrow);
    1
}

/// in the search for new retirements, does the person retire? Like the start one, with the limit adjusted by contract
/// and wage, unemployment, contract end and potential; without a key nation, dates drawn near today
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
pub(crate) unsafe extern "thiscall" fn player_regen_check_retirement(_this: usize, pessoa: usize, saida: usize) -> u8 {
    let c2 = contract_with_current_club(pessoa);
    let estatuto = contract_status(c2);
    let idade = read_i8(pessoa + staff::YEAR_OF_BIRTH);
    let j = read_u32_as_usize(pessoa + staff::PLAYER);
    let motivo: u8 = 'm: {
        if j == 0 {
            if read_u32_as_usize(pessoa + staff::NON_PLAYER) != 0 && idade <= non_player_age_limit(pessoa, saida) {
                return 0;
            }
            break 'm 0x20;
        }
        let mut lim = player_age_limit(j);
        if idade <= lim {
            return 0;
        }
        if idade > 0x21 {
            if c2 == 0 {
                lim = lim.wrapping_sub(1);
            } else if read_u32_as_usize(c2 + 0x45) != 0
                && (read_i8(j + 0x15) == 0x14 || read_i8(j + 0x14) == 0x14 || *((c2 + 0x4f) as *const u8) & 0xf0 == 0x70)
            {
                lim = lim.wrapping_sub(1);
            }
        }
        if read_u32_as_usize(pessoa + staff::CLUB_JOB) != 0 && c2 != 0 {
            let x = read_i8(pessoa + staff::LOYALITY) as i32;
            if (read_u32_as_usize(c2 + 0xc) as i32) > (x + 10).wrapping_mul(750) && read_u16(c2 + 0x2f) > read_u16(0xae2c92) {
                lim = lim.wrapping_sub(1);
                if x < 6 {
                    lim = lim.wrapping_sub(1);
                }
            }
        }
        let v = match_pl_ftol((read_f64_at(0x967890) - (idade as i32 - lim as i32) as f64 * read_f64_at(0x96a730)) * read_i16(j + 0xb) as f64) as i16;
        if idade > 0x2d {
            break 'm 1;
        }
        if idade > 0x22 && read_u32_as_usize(pessoa + staff::CLUB_JOB) == 0 {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            if dias(0xae2c90, read_u32_as_usize(pessoa + staff::DATE_JOINED_CLUB) as u32, read_u32_as_usize(pessoa + 0x42) as u32) > 0x41 {
                break 'm 1;
            }
        }
        let (i, l) = (idade as i32, lim as i32);
        if i > l + 2 && idade > 0x1f && c2 != 0 && read_i8(c2 + 0x3e) > 5 {
            break 'm 1;
        }
        if i > l + 3 {
            if read_u32_as_usize(pessoa + staff::CLUB_JOB) == 0 {
                break 'm 1;
            }
            let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_DAYS_TO_END_DATE);
            if c2 != 0 && (fim(c2) as i16) < 300 && estatuto < 0x32 {
                if read_u32_as_usize(pessoa + staff::NON_PLAYER) != 0 {
                    break 'm 1;
                }
                if read_i8(pessoa + staff::AMBITION) as i32 > sortear(20) {
                    let (cur, pot) = (read_i16(j + 5) as f64, read_i16(j + 7) as f64);
                    if !(pot * read_f64_at(0x969520) <= cur) {
                        break 'm 1;
                    }
                }
            }
        }
        if v > 0x157c {
            return 0;
        }
        let clube = read_u32_as_usize(pessoa + staff::CLUB_JOB);
        let sorteia = if clube != 0 {
            let lg = read_u32_as_usize(clube + club::DIVISION);
            lg == 0 || read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(lg).wrapping_mul(4))) == 0
        } else {
            let n = read_u32_as_usize(pessoa + staff::NATION);
            n != 0 && *((n + 0x11c) as *const u8) == 0
        };
        if sorteia {
            crate::comparacao::ramo("cpr_8 fora do jogo");
            if sortear(3) == 0 {
                break 'm 1; // cpr8: um em três se aposenta
            }
            if read_u32_as_usize(pessoa + staff::CLUB_JOB) != 0 && estatuto < 0 {
                break 'm 1;
            }
            if v >= 0xdac {
                return 0;
            }
            break 'm 1;
        }
        let cur = read_i16(j + 5);
        if cur > 0x64 {
            return 0;
        }
        if cur < 0xa || v < 0x5dc {
            break 'm 1;
        }
        if clube != 0 {
            if estatuto < 0 {
                break 'm 1;
            }
            return 0;
        }
        if sortear(25 * read_i8(j + 0x3c) as i32) != 0 {
            return 0;
        }
        2
    };
    fill_retirement_entry(pessoa, saida, motivo);
    set_retirement_dates(pessoa, saida, set_dates_no_key_nation);
    1
}

/// at a new game, tests every eligible person (0x7a79c0), adds those who retire and sorts the list
/// Original: `PLAYER_REGEN::build_initial_retirement_list` (0x7a58f0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a58f0 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::build_initial_retirement_list`: no jogo novo, cada pessoa (0x6e bytes em [0xae23bc], até [0xae2364],
/// relida a cada passo) de id abaixo de [0xae2364] − 16, sem os bits 0x30 do +0xb do registro de 79 bytes
/// ([0xae2d0c]) e sem contrato (0x4cf5a0) com +0x49 é testada (0x7a79c0); quem se aposenta entra na lista
/// (0x7a8510). No fim o qsort da lista ([this+4], conta +8, 26 bytes, 0x7ae2a0).
pub(crate) unsafe extern "thiscall" fn build_initial_retirement_list(this: usize) {
    crate::comparacao::ramo("cpr_8 lista inicial");
    // ponytail: a original constrói as duas datas da entrada (0x54a110) e deixa o resto como estava na pilha; a
    // 0x7a79c0 grava tudo o que a 0x7a8510 copia antes de devolver 1 (a terceira data local nunca é lida: fica de fora)
    let mut e = [0u8; 0x1a];
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    padrao(e.as_mut_ptr().add(8));
    padrao(e.as_mut_ptr().add(0x10));
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let testar: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_CHECK_INITIAL_RETIREMENT);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_PLAYER_REGEN_ADD_RETIREMENT);
    let mut p = read_u32_as_usize(G_STAFF_TABLE);
    let mut i = 0i32;
    while i < read_u32_as_usize(G_STAFF_COUNT) as i32 {
        let id = read_u32_as_usize(p) as i32;
        if id < (read_u32_as_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10)
            && *((read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 == 0
        {
            let c = de(read_u32_as_usize(G_CONTRACT_MANAGER), p);
            if (c == 0 || *((c + 0x49) as *const u8) == 0) && testar(this, p, e.as_mut_ptr()) != 0 {
                acrescentar(this, e.as_ptr());
            }
        }
        i += 1;
        p = p.wrapping_add(0x6e);
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32_as_usize(this + 4), read_u32_as_usize(this + 8) as u32, 0x1a, 0x7ae2a0);
}

/// Regions the two retirement decisions touch: random state, [0xae1080], the output entry and 0x54cdb0's static date.
///
/// [pt] As regiões que as duas decisões mudam: o sorteio, [0xae1080], a saída e a data estática da 0x54cdb0.
pub(crate) unsafe fn retirement_decision_regions(saida: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&[(0xae1080, 4), (saida, 0x1a), (0xae2470, 8), (0xae2418, 1)]);
    reg
}

/// Comparador: lado a lado pelo sorteio, [0xae1080], a saída e a data estática (a cópia sem contar), e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_regen_check_initial_retirement(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_INITIAL_RETIREMENT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&retirement_decision_regions(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_initial_retirement(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_initial_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: como o da 0x7a79c0.
pub(crate) unsafe extern "thiscall" fn comparar_player_regen_check_retirement(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_RETIREMENT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&retirement_decision_regions(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_retirement(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ponytail: a lista inicial acrescenta (realloc) e sorteia em cada teste: não roda duas vezes. Só a original; o modo
// Rust e a carreira provam a cópia.
pub(crate) unsafe extern "thiscall" fn comparar_build_initial_retirement_list(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INITIAL_RETIREMENT_LIST)(this)
}

// ---------- 254a leva: a busca de novas aposentadorias, a saída dos cargos de quem se aposenta e o registro da liberação (player_regen.cpp, staff_records.cpp) ----------



/// Writes a zero dword at the address.
pub(crate) unsafe fn zero_dword(a: usize) {
    core::ptr::write_unaligned(a as *mut u32, 0);
}

/// tests every eligible person (except the young ones in game competitions, the flagged, those already listed and those
/// with a flagged contract) by age (0x7a7310) or unemployment (0x7a8050), adds those who retire and sorts the list
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
    // ponytail: a original constrói as duas datas da entrada (0x54a110) e deixa o resto como estava na pilha; as duas
    // decisões gravam tudo o que a 0x7a8510 copia antes de devolver 1 (a terceira data local nunca é lida: fica de fora)
    let mut e = [0u8; 0x1a];
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    padrao(e.as_mut_ptr().add(8));
    padrao(e.as_mut_ptr().add(0x10));
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let pela_idade: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_CHECK_RETIREMENT);
    let desempregado: unsafe extern "stdcall" fn(usize, *mut u8) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_CHECK_UNEMPLOYED_RETIREMENT);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_PLAYER_REGEN_ADD_RETIREMENT);
    let mut p = read_u32_as_usize(G_STAFF_TABLE);
    let fim = p.wrapping_add(read_u32_as_usize(G_STAFF_COUNT).wrapping_mul(0x6e));
    while p < fim {
        'x: {
            if read_i8(p + 0x18) < 0x1e {
                let c = read_u32_as_usize(p + 0x39);
                if c != 0 {
                    let lg = read_u32_as_usize(c + 0x57);
                    if lg != 0 && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(lg).wrapping_mul(4))) != 0 {
                        break 'x;
                    }
                }
            }
            let id = read_u32_as_usize(p) as i32;
            if id >= (read_u32_as_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
                break 'x;
            }
            if *((read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
                break 'x;
            }
            let k = read_u32_as_usize(this + 8) as i32;
            if k != 0 {
                let l = read_u32_as_usize(this + 4);
                for i in 0..k.max(0) as usize {
                    if read_u32_as_usize(l + i * 0x1a) as i32 == id {
                        break 'x;
                    }
                }
            }
            let c = de(read_u32_as_usize(G_CONTRACT_MANAGER), p);
            if c != 0 && *((c + 0x49) as *const u8) != 0 {
                break 'x;
            }
            if pela_idade(this, p, e.as_mut_ptr()) != 0 {
                acrescentar(this, e.as_ptr());
            } else if desempregado(p, e.as_mut_ptr()) != 0 {
                crate::comparacao::ramo("cpr_9 desempregado");
                acrescentar(this, e.as_ptr());
            }
        }
        p = p.wrapping_add(0x6e);
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32_as_usize(this + 4), read_u32_as_usize(this + 8) as u32, 0x1a, 0x7ae2a0);
}

/// Removes the person from the job slot of type cargo in club c (chairman, directors, manager with vacancy, assistant,
/// coaches, scouts, physios).
///
/// [pt] Tira a pessoa da vaga do `cargo` (tipo com sinal) no clube `c` (o da pessoa em +`campo`: 0x39 o clube, 0x24 a
/// seleção): 1 presidente (+0xbf); 2 as três de +0xc3; 5 e 12 técnico (+0xcf, só se é ele): no 5, sendo humano (id a
/// partir de [0xae2364] − 16), as ofertas do clube caem (0x8c4950); sai do cargo (0x69b790 com 3, 0, 0) e, se o clube
/// pode ser dirigido (0x544820), a vaga abre (0x69ba50); 6 auxiliar (+0xd3); 8 e 15 as cinco de +0x19f, depois da
/// distribuição dos treinadores do clube humano (0x600a50 com 1, 1; 0x8b2880 com 0); 9 as sete de +0x1b3 e, no
/// clube humano, +`campo` = 0 e as listas dos olheiros do técnico (0x7f7270); 10 as três de +0x1cf.
pub(crate) unsafe fn leave_job_slot(pessoa: usize, c: usize, cargo: i8, campo: usize) {
    let limpar = |base: usize, n: usize| {
        for k in 0..n {
            let a = c + base + 4 * k;
            if read_u32_as_usize(a) == pessoa {
                zero_dword(a);
            }
        }
    };
    let humano = || {
        let f: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        f(c, 1, 1, 0, 0) != 0
    };
    match cargo {
        1 => zero_dword(c + 0xbf),
        2 => limpar(0xc3, 3),
        5 | 12 => {
            if read_u32_as_usize(c + 0xcf) != pessoa {
                return;
            }
            crate::comparacao::ramo("cpr_9 tecnico");
            if cargo == 5 && (read_u32_as_usize(pessoa) as i32) >= (read_u32_as_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
                let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CANCEL_CLUB_OFFERS);
                cancelar(read_u32_as_usize(G_TRANSFER_MANAGER), c);
            }
            let sair: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_CLUB_MANAGER);
            sair(read_u32_as_usize(G_MANAGER_MANAGER), c, 3, 0, 0);
            let pode: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
            if pode(c) as u8 != 0 {
                let abrir: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_ADD_JOB_VACANCY);
                abrir(read_u32_as_usize(G_MANAGER_MANAGER), c, pessoa); // cpr9: a vaga do técnico abre
            }
        }
        6 => zero_dword(c + 0xd3),
        8 | 15 => {
            if humano() {
                let treinadores: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS);
                treinadores(read_u32_as_usize(G_TRAINING_MANAGER), c, 0);
            }
            crate::comparacao::ramo("cpr_9 treinador");
            limpar(0x19f, 5); // cpr9: as vagas de treinador
        }
        9 => {
            limpar(0x1b3, 7);
            if humano() {
                zero_dword(pessoa + campo);
                let listas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
                listas(read_u32_as_usize(G_SCOUT_MANAGER), read_u32_as_usize(c + 0xcf));
            }
        }
        10 => limpar(0x1cf, 3),
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
pub(crate) unsafe extern "stdcall" fn player_regen_retire_from_non_playing_staff(pessoa: usize) {
    let c = read_u32_as_usize(pessoa + staff::CLUB_JOB);
    if c != 0 {
        leave_job_slot(pessoa, c, read_i8(pessoa + staff::JOB_FOR_CLUB), 0x39);
        zero_dword(pessoa + staff::CLUB_JOB);
    }
    let s = read_u32_as_usize(pessoa + staff::NATIONAL_JOB);
    if s != 0 {
        leave_job_slot(pessoa, s, read_i8(pessoa + staff::JOB_FOR_NATION), 0x24);
        zero_dword(pessoa + staff::NATIONAL_JOB);
    }
    *((pessoa + staff::JOB_FOR_CLUB) as *mut u8) = 0;
    *((pessoa + staff::JOB_FOR_NATION) as *mut u8) = 0;
}


// ponytail: a busca acrescenta (realloc) e sorteia em cada teste: não roda duas vezes. Só a original; o modo Rust e a
// carreira provam a cópia.
pub(crate) unsafe extern "thiscall" fn comparar_find_new_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIND_NEW_RETIREMENTS)(this)
}

/// Does leaving jobs only touch the person and the two clubs (not a club manager nor coach/scout jobs of a human club)?
///
/// [pt] A saída dos cargos só mexe na pessoa e nos dois clubes? Não quando é técnico do clube (5, 12) nem nos cargos que
/// chamam rotinas do clube humano (8, 15, 9).
pub(crate) unsafe fn job_exit_is_memory_only(pessoa: usize) -> bool {
    for (campo, tipo) in [(0x39usize, 0x3dusize), (0x24, 0x28)] {
        let c = read_u32_as_usize(pessoa + campo);
        if c == 0 {
            continue;
        }
        let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        match read_i8(pessoa + tipo) {
            5 | 12 if read_u32_as_usize(c + 0xcf) == pessoa => return false,
            8 | 15 | 9 if humano(c, 1, 1, 0, 0) != 0 => return false,
            _ => {}
        }
    }
    true
}

/// Comparador: sem rotinas com efeito (job_exit_is_memory_only), lado a lado pela pessoa (0x6e bytes) e pelos dois clubes
/// (0x245 bytes cada); senão (vaga do técnico, cancelamento de ofertas, listas do humano) só a original.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_retire_from_non_playing_staff(pessoa: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_PLAYER_REGEN_RETIRE_FROM_NON_PLAYING_STAFF);
    if !job_exit_is_memory_only(pessoa) {
        return o(pessoa);
    }
    let mut reg = job_exit_regions(pessoa);
    reg.retain(|r| r.0 != 0);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || player_regen_retire_from_non_playing_staff(pessoa));
    PLACAR.registrar(igual, || format!("player_regen_retire_from_non_playing_staff(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// Regions of leaving jobs: the person (0x6e) and his club and national team records (0x245).
pub(crate) unsafe fn job_exit_regions(pessoa: usize) -> Vec<(usize, usize)> {
    vec![(pessoa, 0x6e), (read_u32_as_usize(pessoa + staff::CLUB_JOB), 0x245), (read_u32_as_usize(pessoa + staff::NATIONAL_JOB), 0x245)]
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia sem ele só grava [0xae1080]: só a
// original; a cópia leva o ramo.

// ---------- 253a leva: os regens: a nota do doador de dados de não jogador, a ordem das aposentadorias e a ordem da fila de regens (player_regen.cpp) ----------





/// −10000 when under 31 or flagged; else a draw by age, ability over the club reputation, a draw by the cube of +0x2f,
/// the attributes and the squares of +0x5b and +0x57, and the final draw (+75 without club +0xcf; without a club, +
/// (+0x22)² and doubled against a 10000 draw)
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
pub(crate) unsafe extern "stdcall" fn player_regen_score_non_player_candidate(pessoa: usize) -> i32 {
    if read_usize(pessoa + staff::NON_PLAYER) != 0 {
        return -10000;
    }
    let id = read_usize(pessoa);
    if *((read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
        return -10000;
    }
    let idade = read_i8_as_i32(pessoa + staff::YEAR_OF_BIRTH);
    if idade < 31 {
        return -10000;
    }
    let mut nota = sortear((idade - 31) * 150); // a idade
    let clube = read_usize(pessoa + staff::CLUB_JOB);
    if clube != 0 {
        let injury_word_0c = read_i16_as_i32(clube + club::REPUTATION);
        let jogador = read_usize(pessoa + staff::PLAYER);
        if injury_word_0c != 0 && jogador != 0 {
            nota = nota.wrapping_add((read_i16_as_i32(jogador + 0xb) / injury_word_0c).wrapping_mul(250));
        }
    }
    let jogador = read_usize(pessoa + staff::PLAYER);
    if jogador != 0 {
        let v = read_i8_as_i32(jogador + 0x2f) / 2;
        nota = nota.wrapping_add(sortear(match_pl_ftol(game_pow(v as f64, 3.0))));
        if read_i16_as_i32(jogador + 0xd) > 7500 {
            nota = nota.wrapping_add(100);
        }
        if read_i16_as_i32(jogador + 0xb) > 7500 {
            nota = nota.wrapping_add(50);
        }
    }
    let quadrados = match_pl_ftol(game_pow(read_i8_as_i32(pessoa + staff::PROFESSIONALISM) as f64, 2.0)).wrapping_add(match_pl_ftol(game_pow(read_i8_as_i32(pessoa + staff::AMBITION) as f64, 2.0)));
    let soma = 3 * (read_i8_as_i32(pessoa + staff::TEMPERAMENT) + read_i8_as_i32(pessoa + staff::PRESSURE)) + 2 * (read_i8_as_i32(pessoa + staff::LOYALITY) + read_i8_as_i32(pessoa + staff::DETERMINATION)) + read_i8_as_i32(pessoa + staff::ADAPTABILITY);
    nota = nota.wrapping_add(soma * 5).wrapping_add(quadrados);
    let clube = read_usize(pessoa + staff::CLUB_JOB);
    if clube != 0 {
        if read_usize(clube + club::MANAGER) == 0 {
            nota = nota.wrapping_add(75);
        }
        return sortear(nota);
    }
    let b = *((pessoa + staff::INT_APPS) as *const u8) as i32;
    nota = nota.wrapping_add(b * b);
    let r = sortear(10000); // sem clube
    if read_i16_as_i32(read_usize(pessoa + staff::PLAYER) + 0xd) > r {
        nota = nota.wrapping_add(nota);
    }
    sortear(nota)
}

/// Effective date of a retirement list entry: announcement (+8) if not yet past the game date, else retirement (+0x10).
///
/// [pt] A data que vale numa entrada da lista de aposentadorias: a do anúncio (+8: dia +0, ano +2) se ainda não passou da
/// data do jogo ([0xae2c90] dia, [0xae2c92] ano: o ano sem sinal, o dia com sinal), senão a da aposentadoria (+0x10).
pub(crate) unsafe fn entry_effective_date(e: usize) -> usize {
    let (dia, ano) = (read_i16_as_i32(G_CURRENT_DATE), read_u16(0xae2c92));
    let a = read_u16(e + 0xa);
    if a < ano || (a == ano && read_i16_as_i32(e + 8) < dia) {
        e + 0x10
    } else {
        e + 8
    }
}

/// qsort comparator of the retirement list: by the effective date (the announcement +8 if not past yet, else the
/// retirement +0x10), year and day; on a tie by id
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
        return aa as i32 - ab as i32; // o ano mais cedo primeiro
    }
    let (ja, jb) = (read_i16_as_i32(da), read_i16_as_i32(db));
    if ja == jb {
        return (read_usize(a) as i32).wrapping_sub(read_usize(b) as i32);
    }
    ja - jb
}

/// qsort comparator of the regen queue: by year (+6), day (+4) and id (+0)
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
    let (aa, ab) = (read_u16(a + 6), read_u16(b + 6));
    if aa > ab {
        return 1; // o ano mais tarde depois
    }
    if aa < ab {
        return -1;
    }
    let (ja, jb) = (read_i16_as_i32(a + 4), read_i16_as_i32(b + 4));
    if ja == jb {
        return (read_usize(a) as i32).wrapping_sub(read_usize(b) as i32);
    }
    ja - jb
}

/// Comparador: lado a lado pelo estado do sorteio (a cópia sem contar), e a nota.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_score_non_player_candidate(pessoa: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> i32>(IDX_PLAYER_REGEN_SCORE_NON_PLAYER_CANDIDATE);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa),
        || crate::aleatorio::sem_contar(|| player_regen_score_non_player_candidate(pessoa)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_score_non_player_candidate({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_REGEN_COMPARE_RETIREMENTS)(a, b);
    let n = player_regen_compare_retirements(a, b);
    PLACAR.registrar(r == n, || format!("player_regen_compare_retirements({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_REGEN_COMPARE_QUEUE_ENTRIES)(a, b);
    let n = player_regen_compare_queue_entries(a, b);
    PLACAR.registrar(r == n, || format!("player_regen_compare_queue_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 250a leva: o jogador que se aposenta sai dos elencos e a escolha do clube do jovem: o de maior nota entre todos e o sorteado da lista da nação (player_regen.cpp) ----------





/// Removes p from the team at p + campo: tactics selection twice and the first of the 50 squad slots holding him.
///
/// [pt] Tira a pessoa `p` do time em `p + campo` (+0x39 o clube, +0x24 a seleção): a tática (0x896c60 de [0xde1d08], com 0
/// e depois com 1 no 3º argumento) e a primeira das 50 vagas (+0xd7) com ela, que esvazia. O time é relido depois de
/// cada chamada, como na original.
pub(crate) unsafe fn remove_from_team(p: usize, campo: usize) {
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    tirar(read_usize(G_TACTICS), read_usize(p + campo), p, 0, 1);
    tirar(read_usize(G_TACTICS), read_usize(p + campo), p, 1, 1);
    let time = read_usize(p + campo);
    if let Some(k) = (0..0x32usize).find(|&k| read_usize(time + 0xd7 + 4 * k) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * k) as *mut usize, 0);
    }
}

/// Formats the "already retired" message into buffer 0xde1f64 (first name, surname, id).
///
/// [pt] O aviso "already retired" no buffer [0xde1f64] (nome +4, sobrenome +8, id +0).
pub(crate) unsafe fn format_already_retired(p: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    formatar(0xde1f64, 0xa70150, read_usize(p + 4), read_usize(p + 8), read_usize(p));
}

/// removes the player from the tactics and the 50 slots of his club (+0x39) and national team (+0x24) and changes the
/// player job to the staff job (12 → 5, 13 → 6, 15 → 8, the others 0x10 retired); without a kept job, clears the club
/// and the national team
/// Original: `PLAYER_REGEN::retire_from_playing_staff` (0x7aace0, player_regen.cpp, stdcall, ret 4).
///
/// [pt] { 0x7aace0 } (stdcall, ret 4, player_regen.cpp): `PLAYER_REGEN::retire_from_playing_staff(p)`: tira `p` do clube
/// (+0x39) e da seleção (+0x24) (`remove_from_team`) e troca o cargo de jogador pelo da comissão (tabelas 0x7aae60 e
/// 0x7aae90): pelo cargo no clube (+0x3d, byte com sinal; fora de 0..16 vale o padrão) 12 → 5, 13 → 6, 15 → 8 (fica no
/// clube), 0 e 16 (aviso "already retired") nada, 1 a 10 a rotina acaba ali, os outros 0x10 (aposentado). Depois pelo
/// cargo na seleção (+0x28) do mesmo jeito, mas 12/13/15 gravam no +0x3d (a original grava ali, não no +0x28) e o
/// padrão grava 0x10 no +0x28. Sem cargo que fica, o clube (+0x39) e a seleção (+0x24) zeram. Sem SEH, sem sorteio.
pub(crate) unsafe extern "stdcall" fn retire_from_playing_staff(p: usize) {
    if read_usize(p + 0x39) != 0 {
        remove_from_team(p, 0x39);
    }
    if read_usize(p + 0x24) != 0 {
        remove_from_team(p, 0x24);
    }
    let fica_no_clube = match read_i8(p + 0x3d) as i32 {
        0 => false,
        1..=10 => return, // comissão: a original sai pelo fim da rotina
        12 => {
            write_i8(p + 0x3d, 5);
            true
        }
        13 => {
            write_i8(p + 0x3d, 6);
            true
        }
        15 => {
            write_i8(p + 0x3d, 8);
            true
        }
        16 => {
            format_already_retired(p);
            false
        }
        _ => {
            crate::comparacao::ramo("cpn_4 jogador aposentado");
            write_i8(p + 0x3d, 0x10); // aposentado
            false
        }
    };
    let fica_na_selecao = match read_i8(p + 0x28) as i32 {
        0 => false,
        1..=10 => return,
        12 => {
            write_i8(p + 0x3d, 5); // no +0x3d, como a original
            true
        }
        13 => {
            write_i8(p + 0x3d, 6);
            true
        }
        15 => {
            write_i8(p + 0x3d, 8);
            true
        }
        16 => {
            format_already_retired(p);
            false
        }
        _ => {
            write_i8(p + 0x28, 0x10);
            false
        }
    };
    if !fica_no_clube {
        core::ptr::write_unaligned((p + 0x39) as *mut usize, 0);
    }
    if !fica_na_selecao {
        core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    }
}

/// Youth with second nationality Basque Country may only join five specific clubs: true for other clubs.
///
/// [pt] O jovem com a segunda nacionalidade (+0x1e) igual a [0x9cf424] só vai para os clubes de id [0x9d0120], [0x9d0134],
/// [0x9d0128], [0x9d01a0] e [0x9d0124]: `true` para os outros clubes.
pub(crate) unsafe fn is_basque_restricted(jogador: usize, clube: usize) -> bool {
    let e = read_usize(jogador + 0x1e);
    if e == 0 || read_usize(e) != read_usize(KEY_NATION_PAYS_BASQUE) {
        return false;
    }
    let id = read_usize(clube);
    ![0x9d0120usize, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124].iter().any(|&g| read_usize(g) == id)
}

/// Can the club bid for the player (0x8ce990 of the transfer manager, not ignoring open offers)?
///
/// [pt] O clube pode chegar ao jogador (0x8ce990 de [0xde1f54], sem ignorar as ofertas abertas)? O motivo fica numa local
/// (a original o grava na vaga do argumento, que ninguém lê depois).
pub(crate) unsafe fn can_club_bid(jogador: usize, clube: usize) -> bool {
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let mut motivo = 0i16;
    pode(read_usize(G_TRANSFER_MANAGER), jogador, clube, &mut motivo, 0) != 0
}

/// among the clubs where the youngster is not a foreigner, that are not a B team, that may approach him and with a
/// squad under 50, the best scored one (0x7ad220)
/// Original: `PLAYER_REGEN::pick_best_club_for_player` (0x7ad0e0, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ad0e0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_best_club_for_player(jogador)`: entre os
/// clubes ([0xae23b4], 0x245 bytes; os [0xae235c] − 2 × [0xae2350] primeiros, relidos a cada volta) em que o jogador
/// não é estrangeiro (0x53d980 com a nação +0x53 e 1), que não são o time B de um par (0x540a50 com 1 achando o
/// parceiro com 0), não vetados (`is_basque_restricted`), que podem chegar a ele (0x8ce990) e com elenco (0x53c590) abaixo de 50,
/// o de maior nota (0x7ad220, sorteia; comparada em 16 bits, a partir de −25000). Nenhum: 0. Sem SEH; a local do
/// parceiro só é lida quando a 0x540a50 o acha (e a grava).
pub(crate) unsafe extern "thiscall" fn pick_best_club_for_player(_this: usize, jogador: usize) -> usize {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let nota_de: unsafe extern "stdcall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_PLAYER_REGEN_SCORE_CLUB_FOR_PLAYER);
    let total = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    let mut eh_a = 0u32;
    let mut clube = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < total() {
        let candidato = estrangeiro(jogador, read_usize(clube + club::NATION), 1) == 0
            && (parceiro(clube, &mut eh_a, 1) == 0 || eh_a != 0)
            && !is_basque_restricted(jogador, clube);
        if candidato {
            let elenco = elenco_de(clube, 0, 0);
            if can_club_bid(jogador, clube) && (elenco as i8) < 0x32 {
                // ponytail: a original empurra o byte do elenco com lixo nos bits altos; a 0x7ad220 só lê o byte
                let s = nota_de(jogador, clube, elenco as u32) as i16;
                if s > melhor_nota {
                    crate::comparacao::ramo("cpn_4 clube melhor para o jovem");
                    melhor = clube;
                    melhor_nota = s;
                }
            }
        }
        i += 1;
        clube += 0x245;
    }
    melhor
}

/// Nation of the youth's list: his nation, with Wales and Ireland mapped to England and Northern Ireland too with
/// random(5) < 2.
///
/// [pt] A nação da lista do jovem: a nação (+0x1a) dele; [0x9cf510] e [0x9cf2dc] vão para [0x9cf2e4], e [0x9cf3e4] também
/// com sorteio(5) < 2.
pub(crate) unsafe fn player_regen_youth_list_nation(jogador: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let n = read_usize(read_usize(jogador + 0x1a));
    if n == read_usize(KEY_NATION_WALES) || n == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) || (n == read_usize(KEY_NATION_NORTHERN_IRELAND) && sorteio(5) < 2) {
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

/// in the club list of the youngster's nation (some nations mapped to [0x9cf2e4]), the one with the highest draw of the
/// score by reputation against ability, among those that may approach him and with a squad under 50
/// Original: `PLAYER_REGEN::pick_club_from_nation_list` (0x7ad590, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ad590 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_club_from_nation_list(jogador)`: percorre a
/// lista de clubes da nação do jovem (`player_regen_youth_list_nation`; a palavra +0 é a conta, o cursor +0xa recomeça de +2, o
/// iterador 0x68d1b0 devolve o id) e, entre os clubes não vetados (`is_basque_restricted`), que podem chegar a ele (0x8ce990) e
/// com elenco (0x53c590) abaixo de 50, fica com o de maior sorteio da nota (5000 + reputação +0x80 menos as distâncias
/// dela à habilidade da ficha +0x61: +0xb, +7 × 50 e +5 × 50, em 16 bits), comparado em 16 bits a partir de −25000.
/// Sem lista ou lista vazia: 0. A lista que acaba antes da conta: a caixa "Error" (linha 0x19e9) e 0. Sem SEH.
pub(crate) unsafe extern "thiscall" fn pick_club_from_nation_list(this: usize, jogador: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let n = player_regen_youth_list_nation(jogador);
    let l = nation_club_list(this, n);
    if l == 0 || read_i16(l) == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    if read_i16(nation_club_list(this, n)) <= 0 {
        return 0;
    }
    let mut i = 0i32;
    loop {
        let item = proximo(nation_club_list(this, n));
        if item == 0 {
            show_error_at(0xa6bf9c, 0x19e9);
            return 0;
        }
        let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(item).wrapping_mul(0x245));
        if !is_basque_restricted(jogador, clube) && can_club_bid(jogador, clube) && (elenco_de(clube, 0, 0) as i8) < 0x32 {
            let ficha = read_usize(jogador + 0x61);
            let rep = read_i16(clube + club::REPUTATION) as i32;
            let c = rep
                .wrapping_sub((read_i16(ficha + 0xb) as i32).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((read_i16(ficha + 7) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((read_i16(ficha + 5) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_add(0x1388);
            crate::comparacao::ramo("cpn_4 sorteio do clube da nacao");
            let nota = sorteio(c as i16 as i32) as i16;
            if nota > melhor_nota {
                melhor = clube;
                melhor_nota = nota;
            }
        }
        i += 1;
        if i as i16 >= read_i16(nation_club_list(this, n)) {
            break;
        }
    }
    melhor
}

/// Comparador: lado a lado pela pessoa (+0x24..+0x3d), pelas 50 vagas do clube e da seleção e pelo aviso em
/// [0xde1f64]. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha, então a tática fica fora.
pub(crate) unsafe extern "stdcall" fn comparar_retire_from_playing_staff(p: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_RETIRE_FROM_PLAYING_STAFF);
    let mut reg = vec![(p + 0x24, 0x1a), (0xde1f64, 0x80)];
    for campo in [0x39usize, 0x24] {
        let time = read_usize(p + campo);
        if time != 0 {
            reg.push((time + 0xd7, 200));
        }
    }
    let (antes_clube, antes_cargo) = (read_usize(p + 0x39), read_i8(p + 0x3d));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(p), || retire_from_playing_staff(p));
    PLACAR.registrar(igual, || format!("retire_from_playing_staff(id {}, clube {:x}, cargo {}) {:x?} x {:x?}", read_usize(p), antes_clube, antes_cargo, mo, mr));
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x7ad220 sorteia), pelo clube devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_pick_best_club_for_player(this: usize, jogador: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_BEST_CLUB_FOR_PLAYER);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| pick_best_club_for_player(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("pick_best_club_for_player(id {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(jogador), r, n, mo, mr));
    r
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio, pelo clube devolvido e pelo cursor (+0xa) das listas
/// que podem ser percorridas (a da nação do jovem e a de [0x9cf2e4]).
pub(crate) unsafe extern "thiscall" fn comparar_pick_club_from_nation_list(this: usize, jogador: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_CLUB_FROM_NATION_LIST);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for n in [read_usize(read_usize(jogador + 0x1a)), read_usize(KEY_NATION_ENGLAND)] {
        let l = nation_club_list(this, n);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| pick_club_from_nation_list(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("pick_club_from_nation_list(id {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(jogador), r, n, mo, mr));
    r
}

// ---------- 247a leva: as listas do regen: a reserva de técnico, a aposentadoria pendente e a vaga para o regen ----------




/// Calls the game's CRT realloc.
pub(crate) unsafe fn crt_realloc(p: usize, tam: usize) -> usize {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    f(p, tam as u32)
}

/// Out-of-memory fatal error for the regen lists (player_regen.cpp at the given line): message box and exit like the
/// original.
///
/// [pt] A falta de memória das listas do regen (player_regen.cpp, linha `linha`): caixa e fim, como a original.
pub(crate) unsafe fn regen_out_of_memory(linha: u32) -> ! {
    show_error_at(0xa6bf9c, linha);
    fatal_exit()
}

/// appends (id, nation, second nation) to the 16-byte spare non-player list (+0x1c); grows by 250 when the player pool
/// (+0x40/+0x44) is full, as the original
/// Original: `PLAYER_REGEN::add_spare_non_player_record` (0x7a81a0, player_regen.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7a81a0 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_spare_non_player_record(id, nação,
/// segunda)`: acrescenta (id +0, nação +4, segunda nação +8) à lista de 16 bytes das reservas de não jogador
/// ([this+0x1c], conta +0x20, capacidade +0x24; tirada pela 0x7a82b0). O original decide crescer pelo banco de
/// jogadores livres (conta +0x40 = capacidade +0x44 − 1), não por esta lista: fiel. Cresce 250 (realloc 0x945667;
/// sem memória, caixa 0x9c4 e fim).
pub(crate) unsafe extern "thiscall" fn add_spare_non_player_record(this: usize, id: u32, nacao: u32, segunda: u32) {
    crate::comparacao::ramo("cpk_2 reserva de tecnico");
    if read_usize(this + 0x40) == read_usize(this + 0x44).wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + 0x1c), read_usize(this + 0x24).wrapping_add(250) << 4);
        write_usize(this + 0x1c, p);
        write_usize(this + 0x24, read_usize(this + 0x24).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0x9c4);
        }
    }
    let n = read_usize(this + 0x20);
    let item = read_usize(this + 0x1c).wrapping_add(n << 4);
    write_usize(item, id as usize);
    write_usize(item + 4, nacao as usize);
    write_usize(item + 8, segunda as usize);
    write_usize(this + 0x20, n.wrapping_add(1)); // mais uma reserva
}

/// appends the record fields to the 26-byte pending retirement table (+4), growing by 250
/// Original: `PLAYER_REGEN::add_retirement` (0x7a8510, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a8510 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::add_retirement(registro)`: acrescenta à tabela de
/// 26 bytes das aposentadorias pendentes ([this+4], conta +8, capacidade +0xc) os campos +0, +4 (dwords), +0x18
/// (byte), +8, +0xa, +0x10, +0x12 (palavras) e +0x19 (byte) do registro, nessa ordem (+0xc..+0x10 e +0x14..+0x18
/// ficam como estão). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória, caixa 0xa53 e fim).
pub(crate) unsafe extern "thiscall" fn add_retirement(this: usize, r: usize) {
    crate::comparacao::ramo("cpk_2 aposentadoria");
    let cap = read_usize(this + 0xc);
    if read_usize(this + 8) == cap.wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + 4), cap.wrapping_add(250).wrapping_mul(26));
        write_usize(this + 4, p);
        write_usize(this + 0xc, read_usize(this + 0xc).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa53);
        }
    }
    let item = || read_usize(this + 4).wrapping_add(read_usize(this + 8).wrapping_mul(26));
    write_usize(item(), read_usize(r));
    write_usize(item() + 4, read_usize(r + 4));
    *((item() + 0x18) as *mut u8) = *((r + 0x18) as *const u8);
    for o in [8usize, 0xa, 0x10, 0x12] {
        write_u16(item() + o, core::ptr::read_unaligned((r + o) as *const u16));
    }
    *((item() + 0x19) as *mut u8) = *((r + 0x19) as *const u8);
    write_usize(this + 8, read_usize(this + 8).wrapping_add(1)); // mais uma aposentadoria
}

/// the person enters the regen queue (+0x10) with the dates; the non-player data in the spare list (+0x1c, random
/// nation if none) and the player data in the free pool (+0x3c)
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
pub(crate) unsafe extern "thiscall" fn queue_staff_for_regen(this: usize, entrada: usize) {
    let id = read_usize(entrada) as i32;
    if id < 0 || id > *(G_STAFF_COUNT as *const i32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_QUEUE_STAFF_FOR_REGEN)(this, entrada);
    }
    crate::comparacao::ramo("cpk_2 vaga para o regen");
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e));
    let cap = read_usize(this + 0x18);
    if read_usize(this + 0x14) == cap.wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + 0x10), cap.wrapping_add(250).wrapping_mul(12));
        write_usize(this + 0x10, p);
        write_usize(this + 0x18, read_usize(this + 0x18).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xaac);
        }
    }
    let fila = read_usize(this + 0x10).wrapping_add(read_usize(this + 0x14).wrapping_mul(12));
    write_usize(fila, id as usize);
    write_u16(fila + 4, core::ptr::read_unaligned((entrada + 4) as *const u16));
    write_u16(fila + 6, core::ptr::read_unaligned((entrada + 6) as *const u16));
    write_usize(this + 0x14, read_usize(this + 0x14).wrapping_add(1)); // mais uma na fila do regen
    if read_usize(pessoa + staff::NON_PLAYER) != 0 {
        if read_usize(this + 0x20) == read_usize(this + 0x24).wrapping_sub(1) {
            let p = crt_realloc(read_usize(this + 0x1c), read_usize(this + 0x24).wrapping_add(250) << 4);
            write_usize(this + 0x1c, p);
            write_usize(this + 0x24, read_usize(this + 0x24).wrapping_add(250));
            if p == 0 {
                regen_out_of_memory(0xac5);
            }
        }
        let item = || read_usize(this + 0x1c).wrapping_add(read_usize(this + 0x20) << 4);
        write_usize(item(), read_usize(read_usize(pessoa + staff::NON_PLAYER)));
        let nacao = read_usize(pessoa + staff::NATION);
        if nacao != 0 {
            write_usize(item() + 4, read_usize(nacao));
        } else {
            let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
            let s = sorteio(*(G_NATION_COUNT as *const i32));
            write_usize(item() + 4, s as usize);
        }
        write_usize(item() + 0xc, read_usize(pessoa));
        write_usize(this + 0x20, read_usize(this + 0x20).wrapping_add(1));
    }
    if read_usize(pessoa + staff::PLAYER) != 0 {
        let cap = read_usize(this + 0x44);
        if read_usize(this + 0x40) == cap.wrapping_sub(1) {
            let p = crt_realloc(read_usize(this + 0x3c), cap.wrapping_add(250) << 4);
            write_usize(this + 0x3c, p);
            write_usize(this + 0x44, read_usize(this + 0x44).wrapping_add(250));
            if read_usize(this + 0x40) == 0 {
                regen_out_of_memory(0xae9); // a original confere a conta (+0x40), não o ponteiro
            }
        }
        let item = || read_usize(this + 0x3c).wrapping_add(read_usize(this + 0x40) << 4);
        write_usize(item(), read_usize(read_usize(pessoa + staff::PLAYER)));
        write_usize(item() + 4, read_usize(read_usize(pessoa + staff::NATION)));
        write_usize(item() + 0xc, read_usize(pessoa));
        write_usize(this + 0x40, read_usize(this + 0x40).wrapping_add(1));
    }
}

/// Comparador: a original acrescenta (pode realocar); confere a conta e o item novo.
pub(crate) unsafe extern "thiscall" fn comparar_add_spare_non_player_record(this: usize, id: u32, nacao: u32, segunda: u32) {
    let n = read_usize(this + 0x20);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_ADD_SPARE_NON_PLAYER_RECORD)(this, id, nacao, segunda);
    let item = read_usize(this + 0x1c).wrapping_add(n << 4);
    let g = [read_usize(item) as u32, read_usize(item + 4) as u32, read_usize(item + 8) as u32];
    PLACAR.registrar(read_usize(this + 0x20) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("add_spare_non_player_record({}, {}, {}) conta {} -> {}, item {:?}", id, nacao, segunda, n, read_usize(this + 0x20), g)
    });
}

/// The 26 bytes 0x7a8510 copies from a retirement record (others zeroed).
///
/// [pt] Os 26 bytes que a 0x7a8510 copia do registro (os outros, zerados).
pub(crate) unsafe fn retirement_entry_fields(r: usize) -> [u8; 26] {
    let mut c = [0u8; 26];
    for (o, n) in [(0usize, 4usize), (4, 4), (8, 2), (0xa, 2), (0x10, 2), (0x12, 2), (0x18, 1), (0x19, 1)] {
        c[o..o + n].copy_from_slice(core::slice::from_raw_parts((r + o) as *const u8, n));
    }
    c
}

/// Comparador: o registro copiado antes; a original acrescenta (pode realocar); confere a conta e os campos.
pub(crate) unsafe extern "thiscall" fn comparar_add_retirement(this: usize, r: usize) {
    let n = read_usize(this + 8);
    let esperado = retirement_entry_fields(r);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_RETIREMENT)(this, r);
    let item = read_usize(this + 4).wrapping_add(n.wrapping_mul(26));
    let g = retirement_entry_fields(item);
    PLACAR.registrar(read_usize(this + 8) == n.wrapping_add(1) && g == esperado, || {
        format!("add_retirement conta {} -> {}, item {:x?} (esperava {:x?})", n, read_usize(this + 8), g, esperado)
    });
}

/// Comparador: a original sorteia e realoca (não pode rodar duas vezes): o plano antes (o que entra em cada lista);
/// confere as contas e os itens (a nação sorteada só pela faixa).
pub(crate) unsafe extern "thiscall" fn comparar_queue_staff_for_regen(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_QUEUE_STAFF_FOR_REGEN);
    let id = read_usize(entrada) as i32;
    if id < 0 || id > *(G_STAFF_COUNT as *const i32) {
        return o(this, entrada);
    }
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e));
    let (n1, n2, n3) = (read_usize(this + 0x14), read_usize(this + 0x20), read_usize(this + 0x40));
    let datas = read_usize(entrada + 4) as u32;
    let np = read_usize(pessoa + staff::NON_PLAYER);
    let jp = read_usize(pessoa + staff::PLAYER);
    let nac = read_usize(pessoa + staff::NATION);
    let reserva = (np != 0).then(|| (read_usize(np) as u32, (nac != 0).then(|| read_usize(nac) as u32), read_usize(pessoa) as u32));
    let banco = (jp != 0).then(|| (read_usize(jp) as u32, read_usize(nac) as u32, read_usize(pessoa) as u32));
    o(this, entrada);
    let fila = read_usize(this + 0x10).wrapping_add(n1.wrapping_mul(12));
    let mut ok = read_usize(this + 0x14) == n1 + 1 && read_usize(fila) as i32 == id && read_usize(fila + 4) as u32 == datas;
    ok &= read_usize(this + 0x20) == n2 + reserva.is_some() as usize;
    if let Some((rid, rnac, rp)) = reserva {
        let i = read_usize(this + 0x1c).wrapping_add(n2 << 4);
        let gn = read_usize(i + 4) as u32;
        ok &= read_usize(i) as u32 == rid && read_usize(i + 0xc) as u32 == rp
            && rnac.map_or(gn < (*(G_NATION_COUNT as *const i32)).max(1) as u32, |v| gn == v);
    }
    ok &= read_usize(this + 0x40) == n3 + banco.is_some() as usize;
    if let Some((bid, bnac, bp)) = banco {
        let i = read_usize(this + 0x3c).wrapping_add(n3 << 4);
        ok &= read_usize(i) as u32 == bid && read_usize(i + 4) as u32 == bnac && read_usize(i + 0xc) as u32 == bp;
    }
    PLACAR.registrar(ok, || {
        format!("queue_staff_for_regen({}) contas {} {} {} -> {} {} {}, reserva {:?}, banco {:?}", id, n1, n2, n3,
            read_usize(this + 0x14), read_usize(this + 0x20), read_usize(this + 0x40), reserva, banco)
    });
}

// ---------- 244a leva: as aposentadorias (player_regen.cpp): tirar a pendente e a decisão do desempregado ----------


/// finds the pending one (26 bytes), pulls the next ones up (0x7a8780), blanks the last one (person −1) and lowers the
/// count
/// Original: `PLAYER_REGEN::remove_retirement` (0x7a8690, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a8690 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::remove_retirement(chave)`: busca (bsearch 0x946464
/// com 0x7ae2a0) a aposentadoria pendente no vetor ([this+4], [this+8] registros de 26 bytes); achou: puxa as
/// seguintes um lugar (0x7a8780, a cópia do registro), a última fica com pessoa −1 e +0x18, +8, +0xa, +0x10, +0x12
/// zerados, e a contagem desce. Sem reordenar.
pub(crate) unsafe extern "thiscall" fn remove_retirement(this: usize, chave: usize) {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let vetor = read_u32(this + 4) as usize;
    let s = bsearch(chave, vetor, read_u32(this + 8), 0x1a, 0x7ae2a0);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(vetor) as u32 as u64 * 0x4ec4ec4f) >> 35) as i32; // o lugar (/ 26)
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_RETIRE_RECORD_ASSIGN);
    while k < read_u32(this + 8) as i32 - 1 {
        let r = (read_u32(this + 4) as usize).wrapping_add(k as usize * 26);
        copiar(r, r + 0x1a);
        k += 1;
    }
    let fim = (read_u32(this + 4) as usize).wrapping_add((read_u32(this + 8) as usize).wrapping_mul(26)).wrapping_sub(26);
    core::ptr::write_unaligned(fim as *mut u32, u32::MAX); // a última sem pessoa
    *((fim + 0x18) as *mut u8) = 0;
    for o in [8usize, 0xa, 0x10, 0x12] {
        core::ptr::write_unaligned((fim + o) as *mut u16, 0);
    }
    core::ptr::write_unaligned((this + 8) as *mut u32, read_u32(this + 8).wrapping_sub(1));
}

/// Is the person on the shortlist (0x842080 on the shortlist manager [[0xde1f54]+0x856], no club)?
///
/// [pt] Está na lista de observados (0x842080 no gerente da lista [[0xde1f54]+0x856], sem clube)?
pub(crate) unsafe fn player_regen_is_on_shortlist(pessoa: usize) -> bool {
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_IS_ON_SHORTLIST);
    observado(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize, pessoa, 0) != 0
}


/// outside the shortlist: the weak manager without a club (draw of 5), the player unemployed for over a year (draw by
/// rating < 750) or whoever has no record retires: fills the entry (reason 0x10, the game date) and returns 1
/// Original: `player_regen_check_unemployed_retirement` (0x7a8050, player_regen.cpp, stdcall, ret 8).
///
/// [pt] { 0x7a8050 } (stdcall, ret 8, player_regen.cpp): `player_regen_check_unemployed_retirement(pessoa, saida)`: quem
/// está na lista de observados não se aposenta. Com registro de técnico (+0x69): sem clube (+0x39) e sem +0x24, com
/// +0xa do registro abaixo de 50 e sorteio(5) = 0. Sem ele, com registro de jogador (+0x61): fora da lista (conferida
/// de novo), sem clube e sem +0x24, desempregado há mais de 365 dias (365 × (ano − ano de +0x40) − dia de +0x3e + dia)
/// e sorteio(nota +0xb do jogador) < 750. Sem nenhum dos dois registros: sempre. Aposenta: a saída (26 bytes) recebe a
/// pessoa, o motivo 0x10, +0x19 = 0, o clube (−1 sem) e a data do jogo em +8 e +0x10; devolve 1. Senão 0.
pub(crate) unsafe extern "stdcall" fn player_regen_check_unemployed_retirement(pessoa: usize, saida: usize) -> u8 {
    let mut sem_registro = false;
    if player_regen_is_on_shortlist(pessoa) {
        return 0;
    }
    let tecnico = read_u32(pessoa + staff::NON_PLAYER) as usize;
    if tecnico == 0 && read_u32(pessoa + staff::PLAYER) == 0 {
        sem_registro = true;
    }
    let aposenta = if tecnico != 0 {
        if read_u32(pessoa + staff::CLUB_JOB) != 0 || read_u32(pessoa + staff::NATIONAL_JOB) != 0 {
            return 0;
        }
        *((tecnico + 0xa) as *const i16) < 50 && friendly_random(5) == 0 // o técnico fraco
    } else if read_u32(pessoa + staff::PLAYER) == 0 {
        false
    } else {
        if player_regen_is_on_shortlist(pessoa) {
            return 0;
        }
        if read_u32(pessoa + staff::CLUB_JOB) != 0 || read_u32(pessoa + staff::NATIONAL_JOB) != 0 {
            false
        } else {
            let anos = (*(G_CURRENT_DATE_YEAR as *const u16) as i32).wrapping_sub(*((pessoa + 0x40) as *const u16) as i32);
            let dias = anos.wrapping_mul(365).wrapping_sub(*((pessoa + staff::DATE_JOINED_CLUB) as *const i16) as i32).wrapping_add(*(G_CURRENT_DATE as *const i16) as i32);
            if dias > 0x16d {
                let nota = *((read_u32(pessoa + staff::PLAYER) as usize + 0xb) as *const i16) as i32;
                friendly_random(nota) < 0x2ee // o sorteio pela nota
            } else {
                false
            }
        }
    };
    if !aposenta && !sem_registro {
        return 0;
    }
    core::ptr::write_unaligned(saida as *mut u32, read_u32(pessoa));
    *((saida + 0x18) as *mut u8) = 0x10; // o motivo: desempregado
    *((saida + 0x19) as *mut u8) = 0;
    let clube = read_u32(pessoa + staff::CLUB_JOB) as usize;
    core::ptr::write_unaligned((saida + 4) as *mut u32, if clube != 0 { read_u32(clube) } else { u32::MAX });
    for o in [8usize, 0x10] {
        core::ptr::write_unaligned((saida + o) as *mut u16, *(G_CURRENT_DATE as *const u16));
        core::ptr::write_unaligned((saida + o + 2) as *mut u16, *(G_CURRENT_DATE_YEAR as *const u16));
    }
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_remove_retirement(this: usize, chave: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_RETIREMENT);
    let n = (*((this + 8) as *const i32)).max(0) as usize;
    let reg = [(this + 4, 8), (read_u32(this + 4) as usize, n * 26)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, chave), || remove_retirement(this, chave));
    PLACAR.registrar(igual, || format!("remove_retirement {:x?} x {:x?}", mo, mr));
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pela saída (26 bytes), a cópia em sem_contar.
pub(crate) unsafe extern "stdcall" fn comparar_player_regen_check_unemployed_retirement(pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_UNEMPLOYED_RETIREMENT);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 0x1a));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_unemployed_retirement(pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_unemployed_retirement({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ---------- 243a leva: a cópia da entrada da fila de regens e a escolha do doador de dados (player_regen.cpp) e a busca na lista de observados (shortlist_manager.cpp) ----------


/// copies the regen queue entry (0xc bytes): the person (+0) and the regen date (+4)
/// Original: `regen_queue_entry_assign` (0x7a8b70, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7a8b70 } (thiscall, ret 4, player_regen.cpp): `regen_queue_entry_assign`: copia a entrada da fila de regens
/// (0xc bytes) de `origem`: a pessoa (+0) e a data do regen em +4 (0x417c10, retorno numa local). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn regen_queue_entry_assign(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32)); // a pessoa do regen
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copiar(this + 4, &mut local, origem + 4);
    this
}


/// the non-player with the highest positive score (0x7a7150) of the nation; with none, of any nation; else 0
/// Original: `player_regen_pick_staff_for_non_player_data` (0x7a6e60, player_regen.cpp, stdcall, ret 4).
///
/// [pt] { 0x7a6e60 } (thiscall, ret 4, player_regen.cpp): `player_regen_pick_staff_for_non_player_data(nacao)`: entre as
/// pessoas não jogadoras ([0xae23bc], 0x6e bytes, [0xae2364] − 16 delas, contagem relida a cada volta), a de maior
/// nota positiva da 0x7a7150 (sorteia; o ecx de entrada vai junto, como na original) da nação `nacao` ([+0x1a]+0 =
/// nacao; nota inicial −1000); sem nenhuma, a de maior nota positiva de qualquer nação (inicial −15000); senão 0.
pub(crate) unsafe extern "thiscall" fn player_regen_pick_staff_for_non_player_data(this: usize, nacao: usize) -> usize {
    let nota: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_PLAYER_REGEN_SCORE_NON_PLAYER_CANDIDATE);
    let quantas = || (*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10);
    let base = read_usize(G_STAFF_TABLE);
    let (mut melhor, mut escolhida) = (-1000i32, 0usize);
    let mut i = 0;
    while i < quantas() {
        let p = base + 0x6e * i as usize;
        let n = read_usize(p + 0x1a);
        if n != 0 && read_usize(n) == nacao {
            let s = nota(this, p);
            if s > melhor && s > 0 { // a melhor da nação
                melhor = s;
                escolhida = p;
            }
        }
        i += 1;
    }
    if escolhida != 0 {
        return escolhida;
    }
    crate::comparacao::ramo("doador de qualquer nação");
    melhor = -15000;
    let base = read_usize(G_STAFF_TABLE); // relida, como na original
    i = 0;
    while i < quantas() {
        let p = base + 0x6e * i as usize;
        let s = nota(this, p);
        if s > melhor && s > 0 {
            melhor = s;
            escolhida = p;
        }
        i += 1;
    }
    escolhida
}

pub(crate) unsafe extern "thiscall" fn comparar_regen_queue_entry_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_REGEN_QUEUE_ENTRY_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || regen_queue_entry_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("regen_queue_entry_assign {:x?} x {:x?}", mo, mr));
    r
}

comparar_valor!(comparar_is_on_shortlist, is_on_shortlist, IDX_IS_ON_SHORTLIST, (jogador: usize, clube: usize) -> u32);

/// Comparador: a 0x7a7150 sorteia (0x90cfc0) a cada pessoa; lado a lado pelo estado do sorteio, a cópia sem contar.
pub(crate) unsafe extern "thiscall" fn comparar_player_regen_pick_staff_for_non_player_data(this: usize, nacao: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_REGEN_PICK_STAFF_FOR_NON_PLAYER_DATA);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao),
        || crate::aleatorio::sem_contar(|| player_regen_pick_staff_for_non_player_data(this, nacao)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_pick_staff_for_non_player_data({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

// ---------- 194a leva: as listas de reserva do gerador de jogadores e o prêmio da liga (player_regen.cpp, league.cpp) ----------

/// Removes the 16-byte record with key id from a regen list ([regen+lista], count [regen+quantos]), shifts the rest,
/// marks the last with -1 and decrements the count.
///
/// [pt] O comum das duas: tira da lista de registros de 16 bytes ([regen+`lista`], quantos em
/// [regen+`quantos`]) o de chave `id` (+0), puxa os seguintes uma vaga para trás, marca o último com −1
/// em +0, +4 e +8 e diminui a conta.
pub(crate) unsafe fn remove_from_list16(regen: usize, id: i32, lista: usize, quantos: usize) {
    let n = || *((regen + quantos) as *const i32);
    let base = || *((regen + lista) as *const usize);
    let Some(i) = (0..n().max(0) as usize).find(|&i| *((base() + i * 16) as *const i32) == id) else {
        return;
    };
    let mut j = i;
    while (j as i32) < n() - 1 {
        core::ptr::copy((base() + (j + 1) * 16) as *const u8, (base() + j * 16) as *mut u8, 16);
        j += 1;
    }
    let ultimo = (base() as isize + (n() as isize - 1) * 16) as usize;
    for o in [0, 4, 8] {
        *((ultimo + o) as *mut i32) = -1;
    }
    *((regen + quantos) as *mut i32) -= 1;
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
pub(crate) unsafe fn list16_regions(regen: usize, lista: usize, quantos: usize) -> Vec<(usize, usize)> {
    let n = (*((regen + quantos) as *const i32)).max(0) as usize;
    let mut r = vec![(regen + quantos, 4)];
    if n > 0 {
        r.push((*((regen + lista) as *const usize), n * 16));
    }
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_remove_spare_non_player_record(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_REMOVE_SPARE_NON_PLAYER_RECORD);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&list16_regions(regen, 0x1c, 0x20), || o(regen, id), || remove_spare_non_player_record(regen, id));
    PLACAR.registrar(igual, || format!("remove_spare_non_player_record({}) {:x?} x {:x?}", id, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_remove_player_data_from_pool(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_REMOVE_PLAYER_DATA_FROM_POOL);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&list16_regions(regen, 0x3c, 0x40), || o(regen, id), || remove_player_data_from_pool(regen, id));
    PLACAR.registrar(igual, || format!("remove_player_data_from_pool({}) {:x?} x {:x?}", id, mo, mr));
}


// ---------- 192a leva: a entrada da aposentadoria, as competições do amistoso e a vaga da notícia (player_regen.cpp, friendly.cpp, news.cpp) ----------

/// the person's entry in the retirement list (by injury or already announced)
/// Original: `PLAYER_REGEN::find_retirement_entry` (0x7a7ed0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a7ed0 } (thiscall, ret 8, player_regen.cpp): a entrada da pessoa `*id` na lista das
/// aposentadorias ([+4], 0x1a bytes cada, [+8] quantas): com `por_lesao`, só se o motivo (+0x18) for 8;
/// senão só se já anunciada (+0x19). Fora isso, 0.
pub(crate) unsafe extern "thiscall" fn find_retirement_entry(regen: usize, id: *const i32, por_lesao: u32) -> usize {
    let lista = *((regen + 4) as *const usize);
    let n = *((regen + 8) as *const i32);
    for i in 0..n.max(0) as usize {
        let e = lista + i * 0x1a;
        if core::ptr::read_unaligned(e as *const i32) == *id {
            let ok = if por_lesao != 0 { *((e + 0x18) as *const u8) == 8 } else { *((e + 0x19) as *const u8) != 0 };
            return if ok { e } else { 0 };
        }
    }
    0
}



pub(crate) unsafe extern "thiscall" fn comparar_find_retirement_entry(regen: usize, id: *const i32, por_lesao: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, u32) -> usize>(IDX_FIND_RETIREMENT_ENTRY)(regen, id, por_lesao);
    let n = find_retirement_entry(regen, id, por_lesao);
    PLACAR.registrar(r == n, || format!("find_retirement_entry({}, {}) original={:#x} rust={:#x}", *id, por_lesao, r, n));
    r
}

/// the random score of how well the club suits the youth player: reputation against ability, youth facilities, human
/// control and nation
/// Original: `player_regen_score_club_for_player` (0x7ad220, player_regen.cpp, stdcall).
///
/// [pt] { 0x7ad220 } (stdcall, ret 0xc, player_regen.cpp): a nota do `clube` para o jovem `jogador`
/// (sorteio(nota)): 5000 + reputação (+0x80) menos as distâncias dela à habilidade (ficha +0x61: +0xb,
/// e +7, +5 × 50); com +7 acima de sorteio(20) + 100, o bônus de juniores (+0x7f, 20 = garantido,
/// senão sorteio(+0x7f) > sorteio(20)); clube de competição ativa (0xadadfc) com +7 > 50: o estado do
/// clube humano (0xae2d14) e a reputação contra o elenco (`elenco`); o treinador de juniores (+0xcf
/// → +0x69) do clube do computador (0x600a50) ou o bônus do humano; a mesma nação (+0x1a/+0x53) e os
/// clubes galeses (Swansea, Cardiff, Wrexham) e a Escócia. Só os 16 bits baixos da nota contam.
pub(crate) unsafe extern "stdcall" fn player_regen_score_club_for_player(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha = u(jogador, 0x61);
    let rep = w(clube, 0x80) as i32;
    let mut s = rep
        .wrapping_sub((w(ficha, 0xb) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 7).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 5).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_add(0x1388);
    let juniores = *((clube + club::TRAINING) as *const i8);
    if w(ficha, 7) as i32 > sorteio(0x14) + 0x64 {
        if juniores == 0x14 {
            s = s.wrapping_add(rep / 2 + 0x1b58);
        } else if sorteio(juniores as i32) > sorteio(0x14) {
            s = s.wrapping_add(rep / 2 + (juniores as i32 + 0x64) * 50);
        }
    }
    let comp = u(clube, 0x57);
    if w(ficha, 7) > 0x32 && comp != 0 && *((*(G_COMP_OBJECT_TABLE as *const usize) + 4 * *(comp as *const u32) as usize) as *const usize) != 0 {
        let estados = *(G_CLUB_STATE_TABLE as *const usize);
        if humano(clube, 1, 1, 0, 0) != 0 && estados != 0 && w(ficha, 7) > 0x78 {
            let e = *((estados + 6 * *(clube as *const u32) as usize + 4) as *const i8);
            if e < 0 {
                s = s.wrapping_add(0x1f4);
            } else if e == 0 {
                s = s.wrapping_add(0xfa);
            }
        }
        let r = w(clube, 0x80);
        s = s.wrapping_add(if r > 0x1676 {
            if elenco < 0x28 { 0x3e8 } else if elenco < 0x2d { 0x2ee } else { 0xfa }
        } else if r > 0x109a {
            if elenco < 0x23 { 0x2ee } else if elenco < 0x28 { 0x1f4 } else { 0xfa }
        } else {
            0xfa
        });
    }
    let tecnico = u(clube, 0xcf);
    let x = if tecnico != 0 { u(tecnico, 0x69) } else { 0 };
    if x != 0 && humano(clube, 1, 1, 0, 0) == 0 {
        s = s.wrapping_add((*((x + 0x22) as *const i8) as i32 - 10) * 30);
        if w(ficha, 7) as i32 > sorteio(0x4b) + 0x64 {
            let v = chance(x, *((x + 0x18) as *const i8)) as i8;
            s = s.wrapping_add((v as i32 - 10) * 60);
        }
        let v = chance(x, *((x + 0x18) as *const i8)) as i8;
        let t = v as i32 + *((x + 0x22) as *const i8) as i32 + juniores as i32;
        if t > 0x28 && w(clube, 0x80) > 0x109a && w(ficha, 7) > 0x50 {
            if (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            } else {
                s = s.wrapping_add(0xfa);
            }
        }
    } else if humano(clube, 1, 1, 0, 0) != 0 {
        s = s.wrapping_add(0xc8);
        if w(ficha, 7) > 0x78 {
            if juniores == 0x14 && (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            }
            s = s.wrapping_add(juniores as i32 * 25);
        }
    }
    let nacao = u(jogador, 0x1a);
    let nacao_do_clube = u(clube, 0x53);
    if nacao == nacao_do_clube {
        s = s.wrapping_add(0x3e8); // mesma nação
    }
    let g = |a: usize| *(a as *const u32);
    let (gales, inglaterra, escocia) = (g(KEY_NATION_WALES), g(KEY_NATION_ENGLAND), g(KEY_NATION_SCOTLAND));
    let id = g(clube);
    if id == g(0x9d0430) || id == g(0x9d0434) || (id == g(0x9d043c) && nacao != 0) {
        let n = g(nacao);
        if n == gales {
            s = s.wrapping_add(0x3e8);
        } else if n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        }
    }
    if g(nacao_do_clube) == escocia {
        let n = g(nacao);
        if n == gales || n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        } else if n == escocia {
            s = s.wrapping_add(0x3e8);
        }
    }
    sorteio(s as i16 as i32)
}

pub(crate) unsafe extern "stdcall" fn comparar_player_regen_score_club_for_player(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i8) -> i32>(IDX_PLAYER_REGEN_SCORE_CLUB_FOR_PLAYER);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(jogador, clube, elenco), || crate::aleatorio::sem_contar(|| player_regen_score_club_for_player(jogador, clube, elenco)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_score_club_for_player(elenco {}) original={} rust={} sorteio igual={}", elenco, r, n, igual));
    r
}

/// copies the pending retirement record: the person, the club, the two dates, the reason and the announced flag
/// Original: `retire_record_assign` (0x7a8780, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a8780 } (thiscall, ret 4, player_regen.cpp): copia a aposentadoria pendente (0x1a bytes) de
/// `origem`: o id da pessoa (+0) e do clube (+4), as datas em +8 e +0x10 (0x417c10, com o retorno numa
/// local), o motivo (+0x18) e a marca de anunciada (+0x19). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn retire_record_assign(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    *(this as *mut u32) = *(origem as *const u32);
    *((this + 4) as *mut u32) = *((origem + 4) as *const u32);
    copiar(this + 8, &mut local, origem + 8);
    copiar(this + 0x10, &mut local, origem + 0x10);
    *((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8); // motivo
    *((this + 0x19) as *mut u8) = *((origem + 0x19) as *const u8);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_retire_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RETIRE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || retire_record_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("retire_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// the injury type of a player_setup.cfg INJURY line by name (76 names, from cold to arthritis); none: 0x4b
/// Original: `player_setup_injury_from_name` (0x7a32e0, player_regen.cpp, cdecl).
///
/// [pt] { 0x7a32e0 } (cdecl, player_regen.cpp): `player_setup_injury_from_name(nome)`: o tipo da lesão (0..0x4c) pelo nome
/// da linha de lesão do player_setup.cfg (strncmp com os 76 nomes, cada um pelo seu tamanho, na ordem; o primeiro que
/// bate); nenhum: 0x4b. Pela aplicação das linhas (0x7a3a10) na criação do jogo. A devolução vale pelo al (a original
/// deixa o resto do eax do strncmp).
pub(crate) unsafe extern "C" fn player_setup_injury_from_name(nome: usize) -> u32 {
    for &(n, texto, tipo) in CQF9_LESOES.iter() {
        if strncmp(nome, texto, n) == 0 {
            crate::comparacao::ramo("cqf_9 lesao pelo nome");
            return tipo as u32; // cqf9: o tipo da lesão pelo nome
        }
    }
    if strncmp(nome, 0xa6b3a8, 8) == 0 { 0x4c } else { 0x4b }
}

/// Comparador: a devolução (o al; só lê).
pub(crate) unsafe extern "C" fn comparar_player_setup_injury_from_name(nome: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_PLAYER_SETUP_INJURY_FROM_NAME)(nome);
    let n = player_setup_injury_from_name(nome);
    PLACAR.registrar(r as u8 == n as u8, || format!("player_setup_injury_from_name({:x}) original={:x} rust={:x}", nome, r as u8, n as u8));
    r
}

/// one list per nation; each club with a division joins its nation's (a nation with +0x11c: only with +0x80 up to 2),
/// except national teams and the clubs at 0x9d00a4/0x9d00a8
/// Original: `PLAYER_REGEN::build_nation_club_lists` (0x7acfc0, player_regen.cpp, thiscall).
///
/// [pt] { 0x7acfc0 } (thiscall, ret 0, player_regen.cpp, quadro SEH do `new`): `PLAYER_REGEN::build_nation_club_lists()`:
/// [this] = 14 bytes por nação de [0xae2350] (0x944e46, sem teste), com uma lista nova (0x944cf1 de 0xe e 0x68d020;
/// sem memória, 0) por nação; cada clube de [0xae235c] entra (0x68d0d0, os 4 bytes do id) na lista da nação dele
/// (`injury_youth_list_nation`).
pub(crate) unsafe extern "thiscall" fn build_nation_club_lists(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let tabela = alocar((read_i32(0xae2350) as u32).wrapping_mul(14));
    core::ptr::write_unaligned(this as *mut usize, tabela);
    let mut i = 0i32;
    while i < read_i32(0xae2350) {
        let obj = novo(0xe);
        let l = if obj != 0 { lista_nova(obj) } else { 0 };
        core::ptr::write_unaligned(read_usize(this).wrapping_add(i as usize * 4) as *mut usize, l);
        i += 1;
    }
    let base = read_usize(G_CLUB_TABLE);
    let mut j = 0i32;
    while j < read_i32(0xae235c) {
        let clube = base.wrapping_add((j as usize).wrapping_mul(0x245));
        let nacao = injury_youth_list_nation(clube);
        if nacao != 0 {
            crate::comparacao::ramo("cpp_9 clube na lista");
            let l = read_usize(read_usize(this).wrapping_add((read_i32(nacao) as usize).wrapping_mul(4)));
            acrescentar(l, clube, 4);
        }
        j += 1;
    }
}

/// Comparador: a original aloca as listas: o plano antes (os ids dos clubes de cada nação, na ordem); a original roda;
/// confere os itens de cada lista.
pub(crate) unsafe extern "thiscall" fn comparar_build_nation_club_lists(this: usize) {
    let n = read_i32(0xae2350).max(0) as usize;
    let mut plano: Vec<Vec<usize>> = vec![Vec::new(); n];
    let base = read_usize(G_CLUB_TABLE);
    for j in 0..read_i32(0xae235c).max(0) as usize {
        let clube = base + j * 0x245;
        let nacao = injury_youth_list_nation(clube);
        if nacao != 0 && (read_i32(nacao) as usize) < n {
            plano[read_i32(nacao) as usize].push(read_usize(clube));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_CLUB_LISTS)(this);
    let errada = (0..n).find(|&k| {
        let l = read_usize(read_usize(this) + 4 * k);
        let ids: Vec<usize> = if l != 0 { list_items_capped(l).into_iter().map(|e| read_usize(e)).collect() } else { Vec::new() };
        l != 0 && ids != plano[k]
    });
    PLACAR.registrar(errada.is_none(), || format!("build_nation_club_lists: a lista da nação {:?} difere do plano", errada));
}

/// from the front of the youth regen queue (12 bytes: id and date), while the date is before today, the youth regen
/// (0x7abac0) and the entry leaves; with more than 2 reserve coaches, rebuilds count − 2 of them: the person lending
/// the data (0x7a6e60) gets the non-player record, the job at the AI club and the ratings from reputation (pow and
/// bands) or drawn; the reserve leaves (0x7a82b0)
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
    let regen: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_PLAYER_REGEN_REGENERATE_AS_YOUTH_PLAYER);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_REGEN_QUEUE_ENTRY_ASSIGN);
    let c0 = read_i32(this + 0x14);
    let primeira = read_usize(this + 0x10);
    let mut k = 0i32;
    while k < c0 {
        if !is_regen_entry_due(primeira) || read_i32(this + 0x40) == 0 {
            break;
        }
        let id = read_i32(primeira);
        if id < 0 || id > read_i32(0xae2364) {
            let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            formatar(0xde1f64, 0xa6bfcc, id);
            show_error_at(0xa6bf9c, 0x36e);
            return;
        }
        crate::comparacao::ramo("cpw_10 regen da fila");
        regen(this, read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize));
        let mut j = 0i32;
        while j < read_i32(this + 0x14) - 1 {
            let e = read_usize(this + 0x10).wrapping_add((j as u32).wrapping_mul(0xc) as usize);
            copiar(e, e + 0xc);
            j += 1;
        }
        let n = read_i32(this + 0x14);
        let ultima = read_usize(this + 0x10).wrapping_add((n as u32).wrapping_mul(0xc) as usize).wrapping_sub(0xc);
        core::ptr::write_unaligned(ultima as *mut i32, -1);
        write_u16(ultima + 4, 0);
        write_u16(ultima + 6, 0);
        core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1); // cpw10: a entrada sai
        k += 1;
    }
    let total = read_i32(this + 0x20);
    if total <= 2 {
        return;
    }
    let doador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_PLAYER_REGEN_PICK_STAFF_FOR_NON_PLAYER_DATA);
    let normalizar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_NON_PLAYER_ATTRIBUTES);
    let tirar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_PLAYER_REGEN_REMOVE_SPARE_NON_PLAYER_RECORD);
    for _ in 0..total - 2 {
        let s = doador(this, read_usize(read_usize(this + 0x1c) + 4));
        if s == 0 {
            return;
        }
        crate::comparacao::ramo("cpw_10 tecnico de reserva");
        let nj = read_usize(G_NON_PLAYER_TABLE).wrapping_add((read_usize(read_usize(this + 0x1c)) as u32).wrapping_mul(0x44) as usize);
        core::ptr::write_unaligned((s + 0x69) as *mut usize, nj);
        let ficha = read_usize(s + 0x61);
        if ficha == 0 {
            let a = friendly_random(0x5dc).wrapping_add(1) as u16;
            write_u16(read_usize(s + 0x69) + 8, a);
            let b = friendly_random(0x5dc).wrapping_add(1) as u16;
            write_u16(read_usize(s + 0x69) + 0xa, b);
            let c = friendly_random(0x1f4).wrapping_add(1) as u16;
            write_u16(read_usize(s + 0x69) + 0xc, c);
        } else {
            if *((ficha + 0xf) as *const i8) == 0x14 {
                let (cl, dl) = (*((nj + 0x10) as *const i8), *((nj + 0x11) as *const i8));
                if cl > dl {
                    *((nj + 0x11) as *mut i8) = cl;
                    *((read_usize(s + 0x69) + 0x10) as *mut i8) = dl;
                    if friendly_random(4) == 0 {
                        let r = friendly_random(0x32) as u8;
                        *((read_usize(s + 0x69) + 0x11) as *mut u8) = r.wrapping_add(0x32);
                    }
                }
            }
            normalizar(s);
            place_coach_at_ai_club(s);
            let rep = read_i16(read_usize(s + 0x61) + 0xb);
            let (k, teto) = if rep > 0x1d4c {
                (0x9689d8, 0x9690b8)
            } else if friendly_random(*((s + 0x57) as *const i8) as i32) != 0 {
                (0x969040, 0x96d300)
            } else {
                (0x968968, 0x96e288)
            };
            let a = rebuilt_coach_rating(s, rep, None, k, teto);
            write_u16(read_usize(s + 0x69) + 0xa, a);
            let b = rebuilt_coach_rating(s, read_i16(read_usize(s + 0x61) + 9), None, k, teto);
            write_u16(read_usize(s + 0x69) + 8, b);
            let c = rebuilt_coach_rating(s, read_i16(read_usize(s + 0x61) + 0xd), Some(0x969040), k, teto);
            write_u16(read_usize(s + 0x69) + 0xc, c);
        }
        tirar(this, read_i32(read_usize(this + 0x1c)));
    }
}

/// O comparador: a 0x7abac0 (o regen jovem) aloca o contrato e manda notícia e as reservas de técnico refazem pessoas
/// achadas pelo caminho: só a original, conferindo o plano da fila. Sem entrada vencida e sem reservas a refazer, os dois
/// lado a lado no cabeçalho (nada muda). Com vencidas (o regen pode pôr entradas novas na fila), reservas (mais de 2)
/// ou id inválido: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_regen_queue(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_REGEN_QUEUE);
    if read_i32(this + 0x20) > 2 {
        crate::comparacao::ramo("cpw_10 reservas so na original");
        return o(this);
    }
    let c0 = read_i32(this + 0x14);
    let base = read_usize(this + 0x10);
    let mut k = 0i32;
    if read_i32(this + 0x40) != 0 {
        while k < c0 {
            let e = base + k as usize * 0xc;
            if !is_regen_entry_due(e) {
                break;
            }
            let id = read_i32(e);
            if id < 0 || id > read_i32(0xae2364) {
                return o(this); // ponytail: a caixa de erro só na original
            }
            k += 1;
        }
    }
    if k == 0 {
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x44)], || o(this), || process_regen_queue(this));
        PLACAR.registrar(igual, || format!("process_regen_queue({:x}) sem vencidas {:x?} x {:x?}", this, mo, mr));
        return;
    }
    // o regen (0x7abac0) pode pôr entradas novas na fila (na carga do save, 4 vencidas de 10 e a conta caiu só 1)
    crate::comparacao::ramo("cpw_10 vencidas so na original");
    o(this);
}

/// 1 when the person's id is on the retirement list, else 0
/// Original: `PLAYER_REGEN::is_retirement_pending` (0x7ae350, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ae350 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::is_retirement_pending(pessoa)`: 1 quando o id da
/// pessoa (+0) está na lista de aposentadorias ([this+4], registros de 0x1a bytes, conta +8 com sinal), senão 0 (e 0
/// sem pessoa). Pela MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: quem não é jogador e vai se aposentar não
/// serve para técnico.
pub(crate) unsafe extern "thiscall" fn is_retirement_pending(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let n = read_usize(this + 8) as i32;
    let id = read_usize(pessoa);
    let mut i = 0i32;
    while i < n {
        if read_usize(read_usize(this + 4) + 0x1a * i as usize) == id {
            crate::comparacao::ramo("cpq_10 aposentadoria marcada");
            return 1; // está na lista
        }
        i += 1;
    }
    0
}

/// copies the first spare non-player record entry (id, nation, second nation) to the outputs and removes it from the
/// list
/// Original: `PLAYER_REGEN::take_spare_non_player_record` (0x7a6e30, player_regen.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7a6e30 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::take_spare_non_player_record(&id, &nacao,
/// &segunda)`, pela contract_manager (0x4ce4e0, duas chamadas): copia o primeiro item da reserva de fichas de não
/// jogador (this +0x1c: id, nação e segunda nação) para as três saídas e tira-o da reserva
/// (PLAYER_REGEN::remove_spare_non_player_record 0x7a82b0 com o id lido da saída); devolve o eax dela.
pub(crate) unsafe extern "thiscall" fn take_spare_non_player_record(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    crate::comparacao::ramo("cpt_10 ficha reservada");
    *id = read_u32_as_usize(read_u32_as_usize(this + 0x1c)) as u32;
    *nacao = read_u32_as_usize(read_u32_as_usize(this + 0x1c) + 4) as u32;
    *segunda = read_u32_as_usize(read_u32_as_usize(this + 0x1c) + 8) as u32;
    let tirar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_PLAYER_REGEN_REMOVE_SPARE_NON_PLAYER_RECORD);
    tirar(this, *id)
}

pub(crate) unsafe extern "thiscall" fn comparar_take_spare_non_player_record(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32) -> u32>(IDX_TAKE_SPARE_NON_PLAYER_RECORD)(this, id, nacao, segunda)
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

pub(crate) unsafe extern "stdcall" fn comparar_record_bits(p: *const i32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(*const i32) -> u8>(IDX_RECORD_BITS)(p);
    let n = record_bits(p);
    PLACAR.registrar(r == n, || format!("record_bits({:p}) original={} rust={}", p, r, n));
    r
}
