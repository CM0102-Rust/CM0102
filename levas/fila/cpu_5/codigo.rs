// ---------- %%LEVA%%a leva: o adiamento pelo clima: a decisão de adiar ou abandonar o jogo, as finanças do clube e a ordem das reservas do estádio (weather.cpp, finance.cpp, stadium.cpp) ----------

unsafe fn cpu5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpu5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O fim do adiamento: sorteio(n) diferente de zero não adia (0), zero abandona (6).
unsafe fn abandon_if_draw_zero(n: i32) -> u8 {
    if cpu5_sortear(n) != 0 {
        0
    } else {
        6
    }
}

/// O fim do adiamento: sorteio(n) zero não adia (0), diferente de zero adia (5).
unsafe fn postpone_if_draw_nonzero(n: i32) -> u8 {
    if cpu5_sortear(n) == 0 {
        0
    } else {
        5
    }
}

/// { 0x923630 } (thiscall, ret 4, weather.cpp): `Weather::check_postponement(jogo)`, pelo dia dos jogos (0x6b3c10) e
/// pela preparação da partida (0x6b7f20). Sem estádio (+0x18) com cidade (+0x38) e lugares (+0x3c), sem competição
/// (+0x14), com o mandante (+0x1c) fora de liga (+0x57) ou o jogo fora da liga dele, ou estádio coberto (+0x4c): 0. Na
/// nação-chave do mandante (0x669360, tabela 0xb63d60 de 0x48 bytes), perto do fim da temporada do ano (+0x39/+0x3b): 0.
/// Na fatia da noite (+0x3f = 2), pelas finanças do mandante (0x5a0590, faixa 0x5986b0): sorteio(1000/2500/5000) zero
/// devolve 0x12. Depois o clima do jogo (0x923010, gravado na vaga do argumento como na original): chuva 5 com 1 a 13
/// graus, estádio descoberto (+0x4d) de até 20000 lugares: sorteio por lugares e mandante (+0x64), zero = 9; chuva 8
/// (até 15000 lugares) e 9 (pelo vento 4 e por 10000 lugares): uma cadeia de sorteios que devolve 0, 5 ou 6.
unsafe extern "thiscall" fn check_postponement(this: usize, jogo: usize) -> u8 {
    let chave: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(0x669360usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x923010usize);
    let est = cpu5_d(jogo + 0x18);
    if est == 0 || cpu5_d(est + 0x38) == 0 || cpu5_d(est + 0x3c) == 0 || cpu5_d(jogo + 0x14) == 0 {
        return 0;
    }
    let casa = cpu5_d(jogo + 0x1c);
    if cpu5_d(casa + 0x57) == 0 {
        return 0;
    }
    let k: i8 = if casa != 0 { chave(casa) } else { -1 };
    if cpu5_d(jogo + 0x14) != cpu5_d(casa + 0x57) || cpu5_b(est + 0x4c) != 0 {
        return 0;
    }
    if (0..0x23).contains(&k) {
        let e = k as usize * 0x48;
        let fim = core::ptr::read_unaligned((0xb63d99 + e) as *const i16);
        let ano = core::ptr::read_unaligned((0xb63d9b + e) as *const u16);
        let dia = core::ptr::read_unaligned(0xae2c90 as *const i16);
        let ano_atual = core::ptr::read_unaligned(0xae2c92 as *const u16);
        if fim > 0x3c {
            if ano == ano_atual && (fim as i32 - 0x3c) < dia as i32 {
                return 0;
            }
        } else if ano == ano_atual && dia > 0x154 {
            return 0;
        }
    }
    if cpu5_b(jogo + 0x3f) == 2 { // cpu5: a fatia da noite
        let fin = financas(cpu5_d(0xae2a38), casa);
        if fin != 0 {
            let f = band(fin) as i8;
            let n = if f == -2 {
                1000
            } else if f == -1 && (cpu5_b(fin + 0x166) as i8) > 0xf {
                0x9c4
            } else {
                5000
            };
            crate::comparacao::ramo("cpu_5 rolagem das financas");
            if cpu5_sortear(n) == 0 { // cpu5: a rolagem das finanças
                return 0x12;
            }
        }
    }
    // ponytail: a original grava o clima na vaga do próprio argumento; se o clima não grava (sem regiões), ficam os
    // bytes do ponteiro do jogo, como aqui.
    let mut cl = jogo as u32;
    clima(this, &mut cl as *mut u32 as usize, jogo);
    let [vento, chuva, temp, _] = cl.to_le_bytes();
    let temp = temp as i8;
    let est = cpu5_d(jogo + 0x18);
    let casa = cpu5_d(jogo + 0x1c);
    match chuva {
        5 => {
            if !(1..0xe).contains(&temp) || cpu5_b(est + 0x4d) != 0 {
                return 0;
            }
            let cap = cpu5_i(est + 0x3c);
            if cap > 0x4e20 {
                return 0;
            }
            let mandante = cpu5_b(casa + 0x64) == 1;
            let n = if cap > 0x30d4 {
                0xfa
            } else if cap > 0x1388 {
                if mandante { 0x96 } else { 0x1e }
            } else if mandante {
                0x32
            } else {
                0xa
            };
            crate::comparacao::ramo("cpu_5 chuva forte");
            if cpu5_sortear(n) != 0 { // cpu5: o sorteio da chuva forte
                0
            } else {
                9
            }
        }
        8 => {
            if cpu5_b(est + 0x4d) != 0 {
                return 0;
            }
            let cap = cpu5_i(est + 0x3c);
            if cap > 0x3a98 {
                return 0;
            }
            let mandante = cpu5_b(casa + 0x64) == 1;
            if cap > 0x1d4c && mandante {
                if cpu5_sortear(4) != 0 {
                    return 5;
                }
                return abandon_if_draw_zero(4);
            }
            if cap <= 0x1d4c && !mandante {
                if cpu5_sortear(0x19) != 0 {
                    return 5;
                }
                return if cpu5_sortear(3) == 0 { 0 } else { 6 };
            }
            if cpu5_sortear(0x1e) == 0 {
                return 6;
            }
            postpone_if_draw_nonzero(0xf)
        }
        9 => {
            let (d1, d2, d3) = if vento == 4 { (10, 0x14, 10) } else { (0x14, 10, 4) };
            if cpu5_b(est + 0x4d) != 0 && cpu5_sortear(d1) != 0 {
                return 0;
            }
            if cpu5_i(est + 0x3c) > 0x2710 {
                if vento == 4 {
                    if cpu5_sortear(d2) != 0 {
                        return 5;
                    }
                    return abandon_if_draw_zero(d3);
                }
                if cpu5_sortear(d2) == 0 {
                    return abandon_if_draw_zero(d3);
                }
                return 5;
            }
            if cpu5_sortear(0x32) == 0 {
                return 6;
            }
            postpone_if_draw_nonzero(if vento == 4 { 8 } else { 4 })
        }
        _ => 0,
    }
}

// ponytail: sorteia: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_check_postponement(this: usize, jogo: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%)(this, jogo)
}

/// { 0x5a0590 } (thiscall, ret 4, finance.cpp): `FinanceTable::get_club_finance(clube)`: o registro de finanças
/// (0x167 bytes em [this]) do clube pelo id (+0); clube B com parceiro A (0x540a50 com o byte devolvido 0): o do A.
/// Clube nulo: a caixa "Error" de finance.CPP (a original).
unsafe extern "thiscall" fn get_club_finance(this: usize, clube: usize) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let mut c = clube;
    let p = parceiro(clube, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        c = p;
    }
    cpu5_d(this).wrapping_add((cpu5_d(c) as u32).wrapping_mul(0x167) as usize)
}

/// Comparador: pelo valor (clube nulo: só a original, com a caixa de erro).
unsafe extern "thiscall" fn comparar_get_club_finance(this: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if clube == 0 {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = get_club_finance(this, clube);
    PLACAR.registrar(r == n, || format!("get_club_finance(clube {:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

/// { 0x85e380 } (cdecl, stadium.cpp): `stadium_compare_bookings(a, b)`, a ordem das reservas do estádio (qsort em
/// 0x85ba1b, 0x85c4ba, 0x85ddf3, 0x85df26): pela data (+4, Date::operator< 0x456770 e > 0x456740), depois pelo id do
/// estádio (+0).
unsafe extern "C" fn stadium_compare_bookings(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpu5_d(b + 4) as u32, cpu5_d(b + 8) as u32);
    if antes(a + 4, d0, d1) != 0 {
        return -1;
    }
    if depois(a + 4, d0, d1) != 0 {
        return 1;
    }
    cpu5_i(a).wrapping_sub(cpu5_i(b))
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_stadium_compare_bookings(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = stadium_compare_bookings(a, b);
    PLACAR.registrar(r == n, || format!("stadium_compare_bookings({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
