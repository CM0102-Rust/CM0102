// ---------- centesima sexagesima quarta leva: o calendario das ligas (league.cpp, fix_man.cpp) ----------

/// { 0x68a030 } (cdecl, league.cpp): a próxima posição `pos` no rodízio do calendário de `n` (palavra)
/// clubes. Na liga especial (`liga` = [0x9cf6e8]): 2 vira 4, 5 vira 3, o resto avança 1. Com menos
/// de 2 turnos ou de 8 rodadas, ou a marca 0x40, avança 1; com 0x80, avança 1 trocando n/2 por
/// n−1 e n por n/2. Senão o estado `*passo` (0..3) alterna saltos: 1 e 2 voltam 1 (abaixo de 0,
/// n−1) e vão a 2 e 3; 3 salta 3 e volta a 0; 0 salta 3 (indo a 1) se `pos` < n−5 e sorteio(4) ≥ 2,
/// senão avança 1; os saltos dão a volta em n.
unsafe extern "C" fn girar_rodada(pos: i32, passo: *mut i8, liga: i32, rodadas: i16, turnos: i16, n: i16, marcas: u8) -> i32 {
    if liga == *(0x9cf6e8 as *const i32) {
        return match pos {
            2 => 4,
            5 => 3,
            _ => pos + 1,
        };
    }
    if turnos < 2 || rodadas < 8 || marcas & 0x40 != 0 {
        return pos + 1;
    }
    let n = n as i32;
    if marcas & 0x80 != 0 {
        let p = pos + 1;
        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p };
    }
    let volta = |p: i32| if p < n { p } else { p - n };
    match *passo {
        1 | 2 => {
            *passo += 1;
            if pos - 1 >= 0 { pos - 1 } else { n - 1 }
        }
        3 => {
            *passo = 0;
            volta(pos + 3)
        }
        _ => {
            let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
            if pos < n - 5 && sorteio(4) >= 2 {
                *passo = 1;
                volta(pos + 3)
            } else {
                volta(pos + 1)
            }
        }
    }
}

/// { 0x68a1c0 } (cdecl, league.cpp): grava o jogo `j` 0..7 da rodada `rodada` (0x41 bytes cada,
/// 7 por jogo a partir de `base`): +9 `a` (−1..6), +0xa `b` (−1..2), +0xb `c` (−1..4) e +5 o dword
/// `valor`. Fora das faixas: a original (caixas de erro, e grava assim mesmo).
unsafe extern "C" fn registrar_jogo_da_rodada(base: usize, rodada: i16, j: i16, a: i16, b: i8, c: i8, valor: u32) {
    if !(0..=7).contains(&j) || !(-1..=6).contains(&a) || !(-1..=2).contains(&b) || !(-1..=4).contains(&c) {
        return orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(365)(base, rodada, j, a, b, c, valor);
    }
    let e = base.wrapping_add((rodada as isize * 0x41 + j as isize * 7) as usize);
    *((e + 9) as *mut i8) = a as i8;
    *((e + 0xa) as *mut i8) = b;
    *((e + 0xb) as *mut i8) = c;
    core::ptr::write_unaligned((e + 5) as *mut u32, valor);
}

/// { 0x5af180 } (cdecl, fix_man.cpp): marca o ano (+0x28, palavra) do jogo `jogo` nas entradas dos
/// dois clubes (+0xc e +0x10) da tabela [0xae2a5c] (palavras: ano × 2 + bit 0). Entrada vazia: o
/// bit 0 liga, ou a entrada zera em 2001, e o ano entra; senão em 2001 o bit 0 apaga e o ano sobe se
/// for maior. Tabela nula: a original (caixa de erro).
unsafe extern "C" fn marcar_ano_no_calendario(jogo: usize) {
    let tabela = *(0xae2a5c as *const usize);
    if tabela == 0 {
        return orig::<unsafe extern "C" fn(usize)>(366)(jogo);
    }
    let ano = core::ptr::read_unaligned((jogo + 0x28) as *const u16);
    for o in [0xc, 0x10] {
        let e = tabela.wrapping_add((*((jogo + o) as *const i32) as usize).wrapping_mul(2)) as *mut u16;
        let v = core::ptr::read_unaligned(e);
        if v & 0xfffe == 0 {
            let w = if ano == 0x7d1 { 0 } else { v | 1 };
            core::ptr::write_unaligned(e, w & 1 | ano << 1);
        } else {
            if v & 1 != 0 && ano == 0x7d1 {
                core::ptr::write_unaligned(e, v & 0xfffe);
            }
            let v = core::ptr::read_unaligned(e);
            if ano > v >> 1 {
                core::ptr::write_unaligned(e, ano.wrapping_mul(2) | v & 1);
            }
        }
    }
}

unsafe extern "C" fn comparar_girar_rodada(pos: i32, passo: *mut i8, liga: i32, rodadas: i16, turnos: i16, n: i16, marcas: u8) -> i32 {
    let o = orig::<unsafe extern "C" fn(i32, *mut i8, i32, i16, i16, i16, u8) -> i32>(364);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((passo as usize, 1));
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pos, passo, liga, rodadas, turnos, n, marcas), || crate::aleatorio::sem_contar(|| girar_rodada(pos, passo, liga, rodadas, turnos, n, marcas)));
    PLACAR.registrar(igual && r == v, || format!("girar_rodada({}, {}, {}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", pos, liga, rodadas, turnos, n, marcas, r, mo, v, mr));
    r
}

unsafe extern "C" fn comparar_registrar_jogo_da_rodada(base: usize, rodada: i16, j: i16, a: i16, b: i8, c: i8, valor: u32) {
    let o = orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(365);
    let e = base.wrapping_add((rodada as isize * 0x41 + j as isize * 7) as usize);
    let reg = [(e + 5, 7)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(base, rodada, j, a, b, c, valor), || registrar_jogo_da_rodada(base, rodada, j, a, b, c, valor));
    PLACAR.registrar(igual, || format!("registrar_jogo_da_rodada({:#x}, {}, {}, {}, {}, {}, {:#x}) {:x?} x {:x?}", base, rodada, j, a, b, c, valor, mo, mr));
}

unsafe extern "C" fn comparar_marcar_ano_no_calendario(jogo: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(366);
    let tabela = *(0xae2a5c as *const usize);
    let reg: Vec<(usize, usize)> = if tabela == 0 {
        vec![]
    } else {
        [0xc, 0x10].iter().map(|&o| (tabela.wrapping_add((*((jogo + o) as *const i32) as usize).wrapping_mul(2)), 2)).collect()
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jogo), || marcar_ano_no_calendario(jogo));
    PLACAR.registrar(igual, || format!("marcar_ano_no_calendario({:#x}) {:x?} x {:x?}", jogo, mo, mr));
}
