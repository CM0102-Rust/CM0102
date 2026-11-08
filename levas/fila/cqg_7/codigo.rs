// ---------- %%LEVA%%a leva: o regen jovem: a pessoa aposentada refeita como jogador novo do banco de fichas (nação, nomes, idiomas, idade, habilidade, reputações, personalidade) e posta num clube; a saída da disputa (player_regen.cpp, dispute.cpp) ----------

unsafe fn cqg7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqg7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg7_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqg7_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqg7_pb(a: usize, v: i8) {
    *(a as *mut i8) = v;
}

unsafe fn cqg7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// Uma chamada thiscall de um argumento pelo endereço (as rotinas de limpeza da pessoa).
unsafe fn call_thiscall_1(rotina: usize, this: usize, a: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(rotina);
    f(this, a);
}

/// O byte `k` do registro de 79 bytes da pessoa ([0xae2d0c] + 0x4f × id).
unsafe fn staff_state_byte(p: usize, k: usize) -> *mut u8 {
    (cqg7_d(0xae2d0c) as isize + cqg7_i(p) as isize * 0x4f + k as isize) as *mut u8
}

/// O inteiro × a constante double do .rdata, truncado (fild; fmul qword; _ftol), em 16 bits.
unsafe fn mul_rdata_const(v: i16, k: usize) -> i16 {
    match_pl_ftol(v as f64 * read_f64_at(k)) as i16
}

/// A habilidade (ficha +5) pelo sorteio: r = sorteio(trunc(PA × m1)); trunc(r + PA × m2) (fild r; fild PA; fmul m2;
/// faddp; _ftol), a PA na ficha +7.
unsafe fn roll_current_ability(f: usize, m1: usize, m2: usize) {
    let pa = cqg7_w(f + 7) as f64;
    let r = cqg7_sortear(match_pl_ftol(pa * read_f64_at(m1)));
    crate::comparison::branch("cqg_7 habilidade");
    cqg7_pw(f + 5, match_pl_ftol(r as f64 + pa * read_f64_at(m2)) as i16); // cqg7: a habilidade sorteada pela PA
}

/// O acerto de um atributo de personalidade de 1..20: acima de 15 cai sorteio(`cai`); abaixo de `a1` sobe sorteio(`s1`);
/// abaixo de `a2` sobe sorteio(`s2`) (0 = nada).
unsafe fn adjust_personality_attribute(a: usize, cai: i32, a1: i8, s1: i32, a2: i8, s2: i32) {
    let v = cqg7_b(a);
    if v > 0xf {
        let r = cqg7_sortear(cai);
        cqg7_pb(a, cqg7_b(a).wrapping_sub(r as i8));
    } else if v < a1 {
        let r = cqg7_sortear(s1);
        cqg7_pb(a, cqg7_b(a).wrapping_add(r as i8));
    } else if s2 != 0 && v < a2 {
        let r = cqg7_sortear(s2);
        cqg7_pb(a, cqg7_b(a).wrapping_add(r as i8));
    }
}

/// Abaixo de `lim`: soma sorteio(n).
unsafe fn add_random_if_below(a: usize, lim: i8, n: i32) {
    if cqg7_b(a) < lim {
        let r = cqg7_sortear(n);
        cqg7_pb(a, cqg7_b(a).wrapping_add(r as i8));
    }
}

/// Tira sorteio(n).
unsafe fn subtract_random(a: usize, n: i32) {
    let r = cqg7_sortear(n);
    cqg7_pb(a, cqg7_b(a).wrapping_sub(r as i8));
}

/// O teto da habilidade: acima de `teto` e sorteio(n) diferente de 0, o teto.
unsafe fn cap_current_ability(f: usize, teto: i16, n: i32) {
    if cqg7_w(f + 5) > teto && cqg7_sortear(n) != 0 {
        cqg7_pw(f + 5, teto);
    }
}

/// { 0x7abac0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::regenerate_as_youth_player(pessoa)`, pela fila dos
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
unsafe extern "thiscall" fn regenerate_as_youth_player(this: usize, p: usize) {
    let marca = staff_state_byte(p, 0xb);
    let b = *marca;
    if b & 0x30 == 0 || cqg7_d(p + 0x39) != 0 {
        *marca = b & 0xcf;
        return;
    }
    call_thiscall_1(0x775ba0, cqg7_d(0xb74318), p);
    let banco = cqg7_d(this + 0x3c);
    let (ficha_id, nacao_id, nacao2_id) = (cqg7_i(banco), cqg7_i(banco + 4), cqg7_i(banco + 8));
    let tirar_do_banco: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x7a8470usize);
    tirar_do_banco(this, ficha_id);
    *((p + 0x5f) as *mut u8) = 2;
    core::ptr::write_unaligned((p + 0x61) as *mut usize, cqg7_d(0xae23c4).wrapping_add((ficha_id as isize * 0x46) as usize));
    *staff_state_byte(p, 0xb) |= 0x40;
    *staff_state_byte(p, 0xd) = 0;
    *staff_state_byte(p, 0xc) &= 0xfd;
    *staff_state_byte(p, 0xb) &= 0x7f;
    *staff_state_byte(p, 0xc) &= 0xfe;
    *staff_state_byte(p, 9) = 0x9c;
    if cqg7_d(0xae2a60) != 0 && *(0x9cc270 as *const u8) != 0 {
        call_thiscall_1(0x5afd00, cqg7_d(0xae2a60), p);
    }
    call_thiscall_1(0x7c6650, cqg7_d(0xb74350), p);
    call_thiscall_1(0x86fb80, cqg7_d(0xdd7f00), p);
    call_thiscall_1(0x55e1d0, cqg7_d(0xae28f0), p);
    call_thiscall_1(0x631f90, cqg7_d(0xb63d28), p);
    call_thiscall_1(0x56a090, cqg7_d(0xae2a30), p);
    call_thiscall_1(0x556830, cqg7_d(0xae28f0), cqg7_d(p));
    call_thiscall_1(0x62bfa0, cqg7_d(0xb63d28), cqg7_d(p));
    call_thiscall_1(0x4d0b50, cqg7_d(0xae1050), p);
    call_thiscall_1(0x4d0bd0, cqg7_d(0xae1050), p);
    *((p + 0x22) as *mut u8) = 0;
    *((p + 0x23) as *mut u8) = 0;
    if cqg7_d(p + 0x69) != 0 {
        core::ptr::write_unaligned((p + 0x69) as *mut u32, 0);
    }
    *staff_state_byte(p, 0xb) &= 0xf0;
    *staff_state_byte(p, 8) = 0;
    *staff_state_byte(p, 9) = 0x9c;
    let nacoes = cqg7_d(0xae23a8);
    let nac = nacoes.wrapping_add((nacao_id as isize * 0x122) as usize);
    core::ptr::write_unaligned((p + 0x1a) as *mut usize, nac);
    if cqg7_i(nac + 0x88) < 100 && cqg7_sortear(4) == 0 {
        let k = cqg7_sortear(cqg7_i(0xae2350));
        let outra = cqg7_d(0xae23a8).wrapping_add((k as isize * 0x122) as usize);
        if cqg7_d(outra + 0x71) != 0 && cqg7_i(outra + 0x88) < 100 {
            core::ptr::write_unaligned((p + 0x1a) as *mut usize, outra);
        }
    }
    if nacao2_id > 0 && nacao2_id < cqg7_i(0xae2350) {
        if !(nacao_id == cqg7_i(0x9cf488) && cqg7_sortear(100) == 0) {
            if nacao2_id != cqg7_i(0x9cf424) && cqg7_sortear(3) != 0 {
                core::ptr::write_unaligned((p + 0x1e) as *mut u32, 0);
            } else {
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, cqg7_d(0xae23a8).wrapping_add((nacao2_id as isize * 0x122) as usize));
            }
        }
    } else {
        core::ptr::write_unaligned((p + 0x1e) as *mut u32, 0);
    }
    let nome1: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    let nome2: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a1b0usize);
    let nome3: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a5b0usize);
    let v = nome1(cqg7_d(p + 0x1a));
    core::ptr::write_unaligned((p + 4) as *mut usize, v);
    let v = nome2(cqg7_d(p + 0x1a));
    core::ptr::write_unaligned((p + 8) as *mut usize, v);
    let v = nome3(cqg7_d(p + 0x1a));
    core::ptr::write_unaligned((p + 0xc) as *mut usize, v);
    call_thiscall_1(0x5fff30, cqg7_d(0xb63c98), p);
    cqg7_pw(p + 0x40, cqg7_w(0xae2c92));
    cqg7_pw(p + 0x3e, cqg7_w(0xae2c90));
    core::ptr::write_unaligned((p + 0x39) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x24) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x4e) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x52) as *mut u32, 0);
    // os idiomas: a original empurra o dword do argumento com o byte de baixo trocado (as chamadas leem o byte)
    let tirar_idioma: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(0x7c6bf0usize);
    for k in 1..=0x40i8 {
        tirar_idioma(cqg7_d(0xb74350), p, k);
    }
    let peso: unsafe extern "stdcall" fn(usize, i8) -> i16 = core::mem::transmute(0x7c6930usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x7c6a30usize);
    let base = (0x14i16.wrapping_sub(cqg7_b(p + 0x56) as i16)).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = peso(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            // ponytail: a fluência vai num dword da pilha com só o byte de baixo gravado (a 0x7c6a30 lê o byte)
            let nivel = if q > 0xa { 0xa } else { q as u8 };
            acrescentar(cqg7_d(0xb74350), p, k as u8 as u32, nivel as u32, 1);
        }
    }
    *((p + 0x3d) as *mut u8) = 0xb;
    *((p + 0x28) as *mut u8) = 0xb;
    *staff_state_byte(p, 0xb) &= 0xcf;
    let f = cqg7_d(p + 0x61);
    let nac = cqg7_d(p + 0x1a);
    let idade: i8;
    if nac != 0 && *((nac + 0x11c) as *const u8) == 0 && cqg7_i(nac + 0x88) < 0x15e {
        // a nação pequena: a idade pela reputação dela e a habilidade pela PA
        let r1 = cqg7_sortear(cqg7_i(nac + 0x88));
        let r2 = cqg7_sortear(1000);
        idade = if r1 < r2 {
            (cqg7_sortear(5) as i8).wrapping_add(0x17)
        } else {
            (cqg7_sortear(7) as i8).wrapping_add(0x11)
        };
        crate::comparison::branch("cqg_7 nacao pequena");
        if idade > 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if idade > 0x1a {
            roll_current_ability(f, 0x968918, 0x968e48);
        } else if idade > 0x17 {
            roll_current_ability(f, 0x968fb0, 0x96a688);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        let r = cqg7_sortear(0x2ee);
        cqg7_pw(f + 9, (r as i16).wrapping_add(cqg7_w(f + 5).wrapping_mul(0x1e)).wrapping_add(0xfa));
        let mut l121 = idade <= 0x12;
        if !l121 && cqg7_w(f + 7) > 0x8c && cqg7_w(f + 5) > 0x64 && cqg7_w(f + 9) < 0x1964 && idade < 0x16 && cqg7_sortear(4) != 0 {
            cqg7_pw(f + 9, mul_rdata_const(cqg7_w(f + 9), 0x968978));
        }
        if idade <= 0x12 {
            l121 = true;
        }
        if l121 {
            let a = cqg7_w(f + 5);
            let r = cqg7_sortear(0xc8) as i16;
            let v = if a < 0x8c {
                r.wrapping_add(cqg7_w(f + 5).wrapping_mul(5)).wrapping_add(1)
            } else {
                r.wrapping_add(cqg7_w(f + 5).wrapping_add(0x19).wrapping_mul(10))
            };
            cqg7_pw(f + 0xd, v);
        } else {
            let a = cqg7_w(f + 5);
            if a < 0x78 {
                let r = cqg7_sortear(0xc8) as i16;
                cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0xf)).wrapping_add(1));
            } else if idade > 0x15 && a > 0x78 {
                let r = cqg7_sortear(0xc8) as i16;
                cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_add(0xa).wrapping_mul(0x19)));
            } else {
                let r = cqg7_sortear(0xc8) as i16;
                let soma = if a < 0x8c { 1 } else { 0xfa };
                cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0x14)).wrapping_add(soma));
            }
        }
        if cqg7_w(f + 9) < 0x8ca {
            let r = cqg7_sortear(5);
            cqg7_pw(f + 0xd, (r as i16).wrapping_add(1));
        }
        if cqg7_w(f + 9) < cqg7_w(f + 0xd) {
            cqg7_pw(f + 9, cqg7_w(f + 0xd).wrapping_add(0xa));
            cqg7_pw(f + 0xd, 1);
        }
        let r = cqg7_sortear(0x32);
        cqg7_pw(f + 0xb, (r as i16).wrapping_add(cqg7_w(f + 9)));
        if idade > 0x17 {
            if cqg7_w(f + 9) < 0x1770 {
                cqg7_pw(f + 9, mul_rdata_const(cqg7_w(f + 9), 0x969588));
            }
            if cqg7_w(f + 0xb) < 0x1770 {
                cqg7_pw(f + 0xb, mul_rdata_const(cqg7_w(f + 0xb), 0x969588));
            }
            let a = cqg7_w(f + 5);
            if a < mul_rdata_const(a, 0x96a5d0) && cqg7_sortear(4) != 0 {
                cqg7_pw(f + 5, mul_rdata_const(cqg7_w(f + 5), 0x96a5d0));
            }
        } else if idade < 0x14 {
            if cqg7_w(f + 5) < mul_rdata_const(cqg7_w(f + 7), 0x968968) && cqg7_sortear(4) != 0 {
                cqg7_pw(f + 5, mul_rdata_const(cqg7_w(f + 7), 0x968968));
            }
            if idade < 0x12 {
                cap_current_ability(f, 0x78, 0x96);
            } else {
                cap_current_ability(f, 0x78, 0x32);
            }
        } else {
            if cqg7_w(f + 9) < 0x1676 {
                cqg7_pw(f + 9, mul_rdata_const(cqg7_w(f + 9), 0x968928));
            }
            if cqg7_w(f + 0xb) < 0x1676 {
                cqg7_pw(f + 0xb, mul_rdata_const(cqg7_w(f + 0xb), 0x968928));
            }
            if cqg7_w(f + 5) < mul_rdata_const(cqg7_w(f + 7), 0x968e48) && cqg7_sortear(4) != 0 {
                cqg7_pw(f + 5, mul_rdata_const(cqg7_w(f + 7), 0x968e48));
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
        if cqg7_sortear(0xfa) != 0 {
            let r = cqg7_sortear(0x2ee) as i16;
            cqg7_pw(f + 9, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0x14)).wrapping_add(0xfa)); // cqg7: a reputação +9 do regen
            crate::comparison::branch("cqg_7 injury_word_0c do regen");
            let r = cqg7_sortear(0x14) as i16;
            cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(5)).wrapping_add(1));
        } else {
            let r = cqg7_sortear(0x2ee) as i16;
            cqg7_pw(f + 9, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0xf)).wrapping_add(0x7d));
            cqg7_pw(f + 0xd, 1);
        }
        if cqg7_w(f + 9) < cqg7_w(f + 0xd) {
            cqg7_pw(f + 9, cqg7_w(f + 0xd).wrapping_add(0xa));
            cqg7_pw(f + 0xd, 1);
        }
        let r = cqg7_sortear(0x32);
        cqg7_pw(f + 0xb, (r as i16).wrapping_add(cqg7_w(f + 9)));
        let s = cqg7_sortear(0x2710);
        crate::comparison::branch("cqg_7 nacao grande");
        if s == 0 {
            idade = (cqg7_sortear(4) as i8).wrapping_add(0xf);
            if idade == 0xf {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s < 0x1388 {
            idade = (cqg7_sortear(3) as i8).wrapping_add(0x10); // cqg7: a idade de 16 a 18
            crate::comparison::branch("cqg_7 idade 16 a 18");
            if idade < 0x12 {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s > 0x26f7 {
            idade = (cqg7_sortear(6) as i8).wrapping_add(0x15);
        } else {
            idade = (cqg7_sortear(4) as i8).wrapping_add(0x11);
            if idade == 0x11 {
                cap_current_ability(f, 0x5a, 0xfa);
            } else {
                cap_current_ability(f, 0x6e, 0x96);
            }
        }
    }
    let grande: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x549ea0usize);
    if grande(cqg7_d(p + 0x1a)) == 0 {
        if idade < 0x19 && cqg7_w(f + 5) < 0x78 {
            cqg7_pw(f + 0xd, 1);
        } else {
            cqg7_pw(f + 0xd, mul_rdata_const(cqg7_w(f + 0xd), 0x96a5d0));
        }
    }
    cqg7_pw(p + 0x12, cqg7_w(0xae2c92).wrapping_sub(idade as i16)); // cqg7: o ano do nascimento
    let r = cqg7_sortear(0x16d);
    cqg7_pw(p + 0x10, r as i16);
    let anos: unsafe extern "C" fn(usize, *const u16) -> u16 = core::mem::transmute(0x53ab90usize);
    let a = anos(p, 0xae2c90 as *const u16);
    cqg7_pw(p + 0x18, a as u8 as i8 as i16);
    // +0x1b
    if cqg7_w(f + 7) > 0x64 && (cqg7_b(f + 0x15) >= 0xf || cqg7_b(f + 0x14) >= 0xf) {
        if cqg7_sortear(3) != 0 {
            let r = cqg7_sortear(0xb);
            cqg7_pb(f + 0x1b, (r as i8).wrapping_add(0xa));
        } else if cqg7_sortear(3) == 0 {
            let r = cqg7_sortear(7);
            cqg7_pb(f + 0x1b, (r as i8).wrapping_add(0xe));
        } else {
            let r = cqg7_sortear(0x10);
            cqg7_pb(f + 0x1b, (r as i8).wrapping_add(5));
        }
    } else if cqg7_sortear(4) != 0 {
        let r = cqg7_sortear(0xa);
        cqg7_pb(f + 0x1b, (r as i8).wrapping_add(8));
    } else {
        let r = cqg7_sortear(0x10);
        cqg7_pb(f + 0x1b, (r as i8).wrapping_add(5));
    }
    adjust_personality_attribute(f + 0x1b, 5, 5, 0xa, 0xa, 8);
    // +0x36 pelo +0x1b
    let a = cqg7_b(f + 0x1b);
    let v = if a > 0xf {
        (cqg7_sortear(0xb) as i8).wrapping_add(0xa)
    } else if a > 0xa {
        (cqg7_sortear(0xa) as i8).wrapping_add(8)
    } else if a < 5 {
        (cqg7_sortear(0xa) as i8).wrapping_add(1)
    } else if cqg7_sortear(4) != 0 {
        (cqg7_sortear(0x10) as i8).wrapping_add(5)
    } else {
        (cqg7_sortear(0xa) as i8).wrapping_add(5)
    };
    cqg7_pb(f + 0x36, v);
    // +0x1d (o goleiro de 20 com 4/5 pela PA)
    if cqg7_b(f + 0xf) == 0x14 && cqg7_sortear(5) != 0 {
        let r = cqg7_sortear(0x14);
        let v = if cqg7_w(f + 7) as i32 > r + 0x64 {
            (cqg7_sortear(0xb) as i8).wrapping_add(0xa)
        } else {
            (cqg7_sortear(0x10) as i8).wrapping_add(5)
        };
        cqg7_pb(f + 0x1d, v);
    } else {
        let r = cqg7_sortear(0x14);
        cqg7_pb(f + 0x1d, (r as i8).wrapping_add(1));
        adjust_personality_attribute(f + 0x1d, 6, 5, 6, 0, 0);
    }
    // +0x2e
    let mut k: i8 = -5;
    if cqg7_b(f + 0x11) >= 0xf {
        k = if cqg7_b(f + 0x19) >= 0xf { 5 } else { 0 };
    } else if ((cqg7_b(f + 0x13) >= 0xf || cqg7_b(f + 0x12) >= 0xf) && cqg7_b(f + 0x19) >= 0xf)
        || cqg7_b(f + 0xf) >= 0xf
        || cqg7_b(f + 0x15) >= 0xf
    {
        k = 0;
    }
    let r1 = cqg7_sortear(k as i32 * 3);
    let r2 = cqg7_sortear(0x14);
    cqg7_pb(f + 0x2e, (r1 as i8).wrapping_add(r2 as i8));
    let a = cqg7_b(f + 0x2e);
    cqg7_pb(f + 0x2e, if a < 1 { 1 } else if a > 0x14 { 0x14 } else { a });
    // +0x34
    let r = cqg7_sortear(0x14);
    cqg7_pb(f + 0x34, (r as i8).wrapping_add(1));
    adjust_personality_attribute(f + 0x34, 6, 8, 0xa, 0, 0);
    // +0x44
    let r = cqg7_sortear(0x14);
    cqg7_pb(f + 0x44, (r as i8).wrapping_add(1));
    if cqg7_b(f + 0x44) > 0xf && cqg7_sortear(3) != 0 {
        subtract_random(f + 0x44, 5);
    } else {
        let a = cqg7_b(f + 0x44);
        if a < 5 {
            add_random_if_below(f + 0x44, 5, 0xa);
        } else if a < 0xa {
            add_random_if_below(f + 0x44, 0xa, 5);
        }
    }
    // +0x3c pela idade
    if idade < 0x12 {
        let r = cqg7_sortear(0x14);
        cqg7_pb(f + 0x3c, (r as i8).wrapping_add(1));
        adjust_personality_attribute(f + 0x3c, 8, 5, 8, 0xa, 6);
    } else if idade < 0x18 {
        let r = cqg7_sortear(0x12);
        cqg7_pb(f + 0x3c, (r as i8).wrapping_add(3));
        adjust_personality_attribute(f + 0x3c, 5, 5, 0xa, 0xa, 5);
    } else {
        let r = cqg7_sortear(0xf);
        cqg7_pb(f + 0x3c, (r as i8).wrapping_add(6));
        adjust_personality_attribute(f + 0x3c, 3, 0xa, 8, 0, 0);
    }
    if cqg7_b(f + 0x3c) < 0xa && cqg7_b(f + 0x44) > 0xf {
        let r = cqg7_sortear(5);
        cqg7_pb(f + 0x3c, cqg7_b(f + 0x3c).wrapping_add(r as i8));
    }
    // +0x3d pela idade
    let r = cqg7_sortear(0x14);
    cqg7_pb(f + 0x3d, (r as i8).wrapping_add(1));
    if idade < 0x12 {
        adjust_personality_attribute(f + 0x3d, 0xa, 5, 6, 0, 0);
    } else if idade < 0x18 {
        adjust_personality_attribute(f + 0x3d, 8, 5, 8, 0, 0);
    } else {
        adjust_personality_attribute(f + 0x3d, 5, 5, 0xa, 0, 0);
    }
    if (cqg7_b(f + 0x11) == 0x14 || cqg7_b(f + 0x12) == 0x14) && cqg7_b(f + 0x3d) < 0xa {
        let r = cqg7_sortear(5);
        cqg7_pb(f + 0x3d, cqg7_b(f + 0x3d).wrapping_add(r as i8));
    }
    if cqg7_b(f + 0x2e) > 0xf && cqg7_b(f + 0x3d) < 0xa {
        let r = cqg7_sortear(5);
        cqg7_pb(f + 0x3d, cqg7_b(f + 0x3d).wrapping_add(r as i8));
    }
    // +0x1e, +0x24, +0x2c, +0x2f (e +0x21) pela idade
    if idade < 0x18 {
        if idade < 0x12 {
            subtract_random(f + 0x1e, 0x28);
            subtract_random(f + 0x24, 0x28);
            let a = cqg7_b(f + 0x2c);
            if a > 0xf {
                subtract_random(f + 0x2c, 8);
            } else if a > 0xa {
                subtract_random(f + 0x2c, 5);
            }
            let a = cqg7_b(f + 0x2f);
            if a > 0xf {
                subtract_random(f + 0x2f, 0xa);
            } else if a > 0xa {
                subtract_random(f + 0x2f, 8);
            }
        } else {
            subtract_random(f + 0x1e, 0x14);
            subtract_random(f + 0x24, 0x14);
            if cqg7_b(f + 0x2c) > 0xf {
                subtract_random(f + 0x2c, 5);
            }
            let a = cqg7_b(f + 0x2f);
            if a > 0xf {
                subtract_random(f + 0x2f, 8);
            } else if a > 0xa {
                subtract_random(f + 0x2f, 5);
            }
        }
        if cqg7_b(f + 0x2f) > 0xf {
            subtract_random(f + 0x2f, 8);
        }
    } else if idade > 0x19 && idade < 0x1d {
        add_random_if_below(f + 0x3d, 5, 8);
        add_random_if_below(f + 0x21, 5, 8);
        add_random_if_below(f + 0x3c, 5, 8);
        subtract_random(f + 0x1e, 0xa);
        subtract_random(f + 0x24, 0xa);
        add_random_if_below(f + 0x2c, 5, 8);
        let a = cqg7_b(f + 0x2f);
        if a > 0xf {
            subtract_random(f + 0x2f, 5);
        } else if a > 0xa {
            subtract_random(f + 0x2f, 3);
        }
    }
    // o clube
    let nac = cqg7_d(p + 0x1a);
    if idade < 0x13 && nac != 0 && *((nac + 0x11c) as *const u8) != 0 && cqg7_sortear(4) != 0 {
        let melhor: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7ad0e0usize);
        let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a6f10usize);
        let c = melhor(this, p);
        if c != 0 && contratar(p, c) != 0 {
            crate::comparison::branch("cqg_7 melhor clube");
            let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
            if humano(c) != 0 {
                let noticia: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(0x7a8ba0usize);
                noticia(p, c);
            }
            if cqg7_w(cqg7_d(p + 0x61) + 7) > 0x50 {
                let t = cqg7_d(c + 0xbf);
                if t != 0 && cqg7_d(t + 0x69) != 0 {
                    let r = cqg7_sortear(100);
                    if cqg7_b(cqg7_d(t + 0x69) + 0x16) as i32 > r {
                        let gostar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547a10usize);
                        gostar(t, p, 0);
                    }
                }
            }
        }
    } else if (nac != 0 && *((nac + 0x11c) as *const u8) == 0 && cqg7_sortear(0xf) != 0) || cqg7_sortear(0xa) == 0 {
        let da_nacao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7ad590usize);
        let c = da_nacao(this, p);
        if c != 0 {
            let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a6f10usize);
            contratar(p, c);
        }
    }
    if cqg7_d(p + 0x39) == 0 && idade < 0x14 {
        let f = cqg7_d(p + 0x61);
        if cqg7_w(f + 0xb) > 0x4e2 {
            cqg7_pw(f + 0xb, 0x4e2);
        }
        if cqg7_w(f + 9) > 0x4e2 {
            cqg7_pw(f + 9, 0x4e2);
        }
        cqg7_pw(f + 0xd, 1);
    }
    let fisicos: unsafe extern "C" fn(usize) = core::mem::transmute(0x538ec0usize);
    let atributos: unsafe extern "C" fn(usize) = core::mem::transmute(0x538bc0usize);
    fisicos(p);
    atributos(p);
}

/// Comparador: só a original (ponytail: a rotina tira a ficha do banco, limpa listas e históricos, aloca idiomas e o
/// contrato e manda notícia; não pode rodar duas vezes). A cópia roda no modo Rust (contadores "cqg_7 ...") e é provada
/// pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_regenerate_as_youth_player(this: usize, p: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, p)
}

/// { 0x56a090 } (thiscall, ret 4, dispute.cpp): `DISPUTE::remove_dispute(pessoa)`: sem pessoa ou sem a tabela de marcas
/// (+4), ou com a marca da pessoa zerada: nada. Senão zera a marca e, com a lista (+0), volta o cursor ao começo (+0xa =
/// +2) e procura, nas `conta` (+0, palavra) primeiras, a entrada do id da pessoa (0x68d1b0): achando, tira-a (0x68d1f0).
unsafe extern "thiscall" fn remove_dispute(this: usize, p: usize) {
    if p == 0 || cqg7_d(this + 4) == 0 {
        return;
    }
    let m = (cqg7_d(this + 4) as isize + cqg7_i(p) as isize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m = 0;
    let lista = cqg7_d(this);
    if lista == 0 {
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqg7_d(lista + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut k = 0i32;
    while (k as i16 as i32) < cqg7_w(cqg7_d(this)) as i32 {
        let no = proximo(cqg7_d(this));
        if no == 0 {
            return;
        }
        if cqg7_i(no) == cqg7_i(p) {
            crate::comparison::branch("cqg_7 saiu da contest");
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
            tirar(cqg7_d(this), no);
            return;
        }
        k += 1;
    }
}

/// Comparador: sem a marca (nada a fazer), lado a lado na marca e no cursor da lista. Com a marca, só a original com o
/// plano: a marca zerada e a conta da lista uma a menos quando a pessoa está nela (a procura feita antes, com o cursor
/// devolvido).
unsafe extern "thiscall" fn comparar_remove_dispute(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    if p == 0 || cqg7_d(this + 4) == 0 {
        return o(this, p);
    }
    let m = cqg7_d(this + 4).wrapping_add(cqg7_i(p) as isize as usize);
    let lista = cqg7_d(this);
    if *(m as *const u8) == 0 {
        let mut reg = vec![(m, 1)];
        if lista != 0 {
            reg.push((lista, 0xe));
        }
        let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p), || remove_dispute(this, p));
        PLACAR.record(igual, || format!("remove_dispute(pessoa {:x}) {:x?} x {:x?}", p, mo.0, mr.0));
        return;
    }
    let mut achou = 0i16;
    let conta_antes = if lista != 0 { cqg7_w(lista) } else { 0 };
    if lista != 0 {
        let cursor = cqg7_d(lista + 0xa);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqg7_d(lista + 2) as u32);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        for _ in 0..conta_antes.max(0) {
            let no = proximo(lista);
            if no == 0 {
                break;
            }
            if cqg7_i(no) == cqg7_i(p) {
                achou = 1;
                break;
            }
        }
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor as u32);
    }
    o(this, p);
    let conta_depois = if lista != 0 { cqg7_w(lista) } else { 0 };
    PLACAR.record(*(m as *const u8) == 0 && conta_depois == conta_antes - achou,
        || format!("remove_dispute(pessoa {:x}) conta {} -> {} (achou {})", p, conta_antes, conta_depois, achou));
}
