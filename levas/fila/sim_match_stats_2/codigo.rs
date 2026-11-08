// ---------- %%LEVA%%a leva: o registro da partida, a ordem dos eventos e o idioma que o jogador perde (match_stats.cpp, player_stats.cpp) ----------

/// { 0x729ff0 } (thiscall, ret 0xc, match_stats.cpp; `this` não é usado): preenche o registro de
/// estatística `reg` da partida `m` (jogo em [m+0x4792]): clubes, datas, público (+0x34) e renda
/// (+0x38, 0x59af40 só sem `sem_renda`), o placar por tempo, o estádio, o árbitro, o minuto / 11, o
/// clima (0x923010 em +0xd76), até 50 eventos de 4 bytes em +0x57 (ordenados por 0x72b160, fim 0xff)
/// e, por time (+0x11f + 0x625 × t), as estatísticas, a formação (0x5b2ae0), o capitão (0x897590 com
/// `sem_renda`) e os 20 jogadores de 0x49 bytes (número da camisa por 0x85a760, dando um com
/// 0x85a470 quando falta). Partida ou registro nulos: a original (caixa de erro). Devolve 1.
unsafe extern "thiscall" fn fill_match_record(estat: usize, m: usize, reg: usize, sem_renda: u32) -> u32 {
    if m == 0 || reg == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I0%%)(estat, m, reg, sem_renda);
    }
    let b = |p: usize| *(p as *const u8);
    let w = |p: usize| core::ptr::read_unaligned(p as *const u16);
    let d = |p: usize| core::ptr::read_unaligned(p as *const u32);
    let pb = |p: usize, v: u8| *(p as *mut u8) = v;
    let pw = |p: usize, v: u16| core::ptr::write_unaligned(p as *mut u16, v);
    let pd = |p: usize, v: u32| core::ptr::write_unaligned(p as *mut u32, v);
    let data: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let atexit: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x94627ausize);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x923010usize);
    let renda: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x59af40usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    let capitao: unsafe extern "thiscall" fn(usize, usize, i32, i32) -> i8 = core::mem::transmute(0x897590usize);
    let posicao: unsafe extern "thiscall" fn(usize, i32, i32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    // a data estática (dia 0x29 de 1972) da primeira chamada: a marca antes da construção, como na original
    if b(0xb6b7c4) & 1 == 0 {
        pb(0xb6b7c4, b(0xb6b7c4) | 1);
        data(0xb6b7b8, 0x29, 0x7b4);
        atexit(0x72aa70);
    }
    let jogo = || d(m + 0x4792) as usize;
    pd(reg, d(jogo() + 0xc));
    pd(reg + 4, d(jogo() + 0x10));
    pw(reg + 8, w(jogo() + 0x2a));
    pw(reg + 0xa, w(jogo() + 0x28));
    pd(reg + 0x34, d(m + 0xf634));
    for k in 0..5 {
        pw(reg + 0xc + 2 * k, w(m + 0x1f6 + 2 * k));
        pw(reg + 0x16 + 2 * k, w(m + 0x200 + 2 * k));
        pw(reg + 0x20 + 2 * k, w(m + 0x20a + 2 * k));
        pw(reg + 0x2a + 2 * k, w(m + 0x214 + 2 * k));
    }
    pd(reg + 0x38, 0);
    let estadio = d(m + 0x477a) as i32;
    let p = if estadio > -1 { d((d(0xae23bc) as i32 + estadio * 0x6e + 0x61) as usize) as usize } else { 0 };
    pd(reg + 0x3c, if p != 0 { d(p) } else { u32::MAX });
    let q = d(m + 0xf5b6) as usize;
    pd(reg + 0x40, if q != 0 && d(q) as i32 > -1 { d(q) } else { u32::MAX });
    pw(reg + 0x4d, (w(m + 0x8ed0) as i16 as i32 / 11) as u16);
    pw(reg + 0x4f, w(m + 0x8ed4));
    pw(reg + 0x51, w(m + 0x8ed2));
    pb(reg + 0x4c, b(m));
    pd(reg + 0x53, matches!(b(m + 0x8eb3), 1 | 4) as u32);
    clima(d(0xde2f74) as usize, reg + 0xd76, jogo());
    if sem_renda == 0 {
        pd(reg + 0x38, renda(d(0xae2a38) as usize, jogo(), d(reg + 0x34)));
    }
    // os eventos: código do lance → tipo (1..8); 7 e 8 marcam +0x53 e guardam a ordem no lugar do minuto
    let ev = d(m + 0x8f7a) as usize;
    let mut n: u32 = 0;
    if ev != 0 && (w(ev + 6) as i16) > 0 {
        let mut i: i32 = 0;
        while i < w(ev + 6) as i16 as i32 {
            let e = ev + 0x720 + (i as i16 as i32 * 14) as usize;
            let c = w(e) as i16 as i32;
            let tipo: u8 = match c {
                0x1f56 | 0x1f57 => 6,
                0x1fcc | 0x2013..=0x201b | 0x2114 => 1,
                0x1fcd => 2,
                0x1fce => 4,
                0x1fcf => 3,
                0x1ff5..=0x1ff8 => 5,
                0x2000 => 7,
                0x2008 => 8,
                _ => 0,
            };
            if tipo != 0 && n != 0x32 {
                let especial = tipo >= 7;
                if especial {
                    pd(reg + 0x53, 1);
                }
                let s = reg + 0x57 + 4 * n as usize;
                pb(s, (b(e + 6) != 0) as u8 | ((n as u8).wrapping_add(1) << 1));
                pb(s + 1, b(e + 5));
                pb(s + 2, tipo);
                pb(s + 3, if especial { n as u8 } else { b(e + 4) });
                n += 1;
            }
            i += 1;
        }
        if n > 0 {
            ordenar(reg + 0x57, n, 4, 0x72b160);
            for k in 0..n as usize {
                pb(reg + 0x57 + 4 * k, b(reg + 0x57 + 4 * k) & 1);
            }
        }
    }
    if n < 0x32 {
        pb(reg + 0x57 + 4 * n as usize, 0xff);
    }
    for t in 0..2usize {
        let tm = reg + 0x625 * t + 0x11f;
        let s = m + 0x36 * t + 0xf5bc;
        let id = d(reg + 4 * t) as i32;
        let clube = (d(0xae23b4) as i32 + id * 0x245) as usize;
        let usa = b(jogo() + 0x40 + t) != 0 && usa_numeros(d(0xdd7ef0) as usize, clube, 0) != 0;
        for (o, de) in [(0, 1), (1, 0), (3, 4), (4, 5), (2, 6), (5, 7), (6, 8), (7, 9), (8, 2), (9, 3), (0xa, 0xb),
                        (0xb, 0xa), (0xc, 0xc), (0xd, 0xd), (0xe, 0xe), (0xf, 0x10), (0x10, 0xf)] {
            pb(tm + o, b(s + de));
        }
        for (o, de) in [(0x11, 0x26), (0x13, 0x28), (0x15, 0x2a), (0x17, 0x2c), (0x19, 0x2e), (0x1b, 0x30), (0x1d, 0x34)] {
            pw(tm + o, w(s + de));
        }
        for k in 0..5 {
            pb(tm + 0x1f + k, b(s + 0x12 + k));
            pb(tm + 0x24 + k, b(s + 0x21 + k));
            pb(tm + 0x29 + k, b(s + 0x1c + k));
            pb(tm + 0x2e + k, b(s + 0x17 + k));
        }
        pb(tm + 0x33, b(m + 0x8d4f + t));
        if sem_renda != 0 {
            let fora = (b(jogo() + 0x40 + t) == 0) as i32;
            let c = capitao(d(0xde1d08) as usize, d(jogo() + 0x1c + 4 * t) as usize, fora, 0) as i32;
            pb(tm + 0x34, b((m as isize + (c + t as i32 * 20) as isize * 0x1be + 0x4796) as usize));
        } else {
            pb(tm + 0x34, b(s + 0x11));
        }
        let f = m + 0x18e3 * t;
        for k in 0..11 {
            pw(tm + 0x35 + 2 * k, posicao(f + 0xc3a8, k as i32, 0));
            pw(tm + 0x4b + 2 * k, posicao(f + 0xc3a8, k as i32, 1));
        }
        pd(tm + 0x61, d(f + 0xc92c));
        let g = m + 0xd0 * t;
        let tecnico = d(g + 0x904d) as usize;
        let ultimo = if tecnico != 0 {
            pd(tm + 0x65, d(tecnico));
            pw(tm + 0x69, w(g + 0x9032));
            pw(tm + 0x6b, w(g + 0x9034));
            w(g + 0x9036)
        } else {
            pd(tm + 0x65, u32::MAX);
            pw(tm + 0x69, 0xffff);
            pw(tm + 0x6b, 0xffff);
            0xffff
        };
        pw(tm + 0x6d, ultimo);
        pw(tm + 0x6f, w(m + 2 * t + 0x1d6));
        for k in 0..20usize {
            let x = tm + 0x8d + 0x49 * k;
            let y = m + 0x22d8 * t + 0x47ff + 0x1be * k;
            let gerente = || d(0xdd7ef0) as usize;
            if usa {
                if d(y + 4) == 0 {
                    pb(x, 0);
                } else {
                    let mut v = (numero(gerente(), d(y) as usize, clube) as i32).wrapping_abs() as u8;
                    pb(x, v);
                    if v == 0 {
                        dar_numero(gerente(), clube, d(y) as usize);
                        v = (numero(gerente(), d(y) as usize, clube) as i32).wrapping_abs() as u8;
                        pb(x, v);
                    }
                }
            } else {
                pb(x, if d(y) == 0 { 0 } else { (k as u8) + 1 });
            }
            pd(x - 0x1c, if d(y) != 0 { d(d(y) as usize) } else { u32::MAX });
            pb(x + 1, b(y - 0x50));
            pb(x + 2, b(y - 0x4f));
            if d(x - 0x1c) as i32 <= -1 {
                continue;
            }
            pd(x - 0x18, if d(y + 4) != 0 { d(d(y + 4) as usize) } else { u32::MAX });
            pd(x - 0x14, d(y - 0xc));
            pd(x - 0x10, d(y - 8));
            pd(x - 0xc, d(y - 4));
            let mut copia = [0u8; 8];
            copiar_data(x - 8, copia.as_mut_ptr() as usize, if d(y) != 0 { d(y) as usize + 0x10 } else { 0xb6b7b8 });
            pb(x + 3, b(y - 0x4d));
            // ponytail: fld f32 × fmul f64 (0,01) em 80 bits no x87; em f64 dá o mesmo truncamento (f32 × 0,01 nunca cai a 2^-53 de um inteiro)
            let nota = core::ptr::read_unaligned((y - 0x1c) as *const f32) as f64 * core::ptr::read_unaligned(0x967898 as *const f64);
            pb(x + 4, ftol(nota) as u8);
            for (o, de) in [(5, 0x5d), (6, 0x5a), (7, 0x5c), (8, 0x5b), (9, 0x5f), (0xa, 0x5e), (0xb, 0x59), (0xc, 0x58),
                            (0xd, 0x55), (0xe, 0x54), (0xf, 0x56), (0x10, 0x45), (0x11, 0x46), (0x12, 0x44), (0x13, 0x43),
                            (0x16, 0x4a), (0x17, 0x47), (0x14, 0x49)] {
                pb(x + o, b(y - de));
            }
            pw(x + 0x1a, ftol(core::ptr::read_unaligned((y - 0x18) as *const f32) as f64) as u16);
            pb(x + 0x15, b(y - 0x52));
            pb(x + 0x18, b(y - 0x48));
            pw(x + 0x1c, w(y - 0x22));
            pw(x + 0x1e, w(y - 0x20));
            pw(x + 0x20, w(y - 0x1e));
            pb(x + 0x19, b(y - 0x4e));
            for k2 in 0..9 {
                pb(x + 0x22 + k2, b(y - 0x68 + k2));
            }
            pb(x + 0x2b, b(y - 0x57));
        }
    }
    1
}

/// { 0x72b160 } (cdecl, match_stats.cpp, comparador do qsort dos eventos de 4 bytes): mesmo grupo
/// (tipos 1..3 entre si, 7..8 entre si, ou o mesmo tipo +2) desempata pelo minuto (+3, sem sinal) e
/// depois pela ordem de chegada (+0 com sinal, metade); grupos diferentes: a diferença dos tipos.
unsafe extern "C" fn match_stats_compare_events(a: *const u8, b: *const u8) -> i32 {
    let (ta, tb) = (*a.add(2), *b.add(2));
    let mesmo = (matches!(ta, 1..=3) && matches!(tb, 1..=3)) || (matches!(ta, 7 | 8) && matches!(tb, 7 | 8));
    if !mesmo {
        let r = ta as i8 as i32 - tb as i8 as i32;
        if r != 0 {
            return r;
        }
    }
    let r = *a.add(3) as i32 - *b.add(3) as i32;
    if r != 0 {
        return r;
    }
    (*a as i8 as i32 >> 1) - (*b as i8 as i32 >> 1)
}

/// { 0x7c6bf0 } (thiscall, ret 8, player_stats.cpp): tira o idioma `idioma` do jogador `ficha` da
/// tabela de idiomas ([this+0x3c], [this+0x44] entradas de 6 bytes: id, idioma, fluência), achado por
/// busca binária (0x946464 com 0x7c6d40): as seguintes descem uma casa, a última vira id −1 e
/// idioma 0 e a contagem cai. Sem tabela ou sem a entrada: nada.
unsafe extern "thiscall" fn remove_language(estat: usize, ficha: usize, idioma: i8) {
    let tabela = *((estat + 0x3c) as *const usize);
    if tabela == 0 {
        return;
    }
    let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    // ponytail: o byte 5 da chave fica com lixo de pilha na original; o comparador 0x7c6d40 só lê id e idioma
    let mut chave = [0u8; 6];
    chave[..4].copy_from_slice(&(*(ficha as *const u32)).to_le_bytes());
    chave[4] = idioma as u8;
    let contagem = (estat + 0x44) as *mut i32;
    let achada = buscar(chave.as_ptr() as usize, tabela, *contagem as u32, 6, 0x7c6d40);
    if achada == 0 {
        return;
    }
    let mut i = (achada.wrapping_sub(tabela) as u32 / 6) as i32;
    while i < *contagem - 1 {
        let (para, de) = (tabela + 6 * i as usize, tabela + 6 * (i + 1) as usize);
        core::ptr::write_unaligned(para as *mut u32, core::ptr::read_unaligned(de as *const u32));
        core::ptr::write_unaligned((para + 4) as *mut u16, core::ptr::read_unaligned((de + 4) as *const u16));
        i += 1;
    }
    let ultima = (tabela as isize + *contagem as isize * 6) as usize;
    core::ptr::write_unaligned((ultima - 6) as *mut u32, u32::MAX);
    *((ultima - 2) as *mut u8) = 0;
    *contagem -= 1;
}

unsafe extern "thiscall" fn comparar_fill_match_record(estat: usize, m: usize, reg: usize, sem_renda: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I0%%);
    // ponytail: a primeira chamada (data estática e atexit) e os erros só pela original
    if m == 0 || reg == 0 || *(0xb6b7c4 as *const u8) & 1 == 0 {
        return o(estat, m, reg, sem_renda);
    }
    // o número da camisa dado pela original (0x85a470) fica fora das regiões: a cópia já o acha dado
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg, 0xd7a)], || o(estat, m, reg, sem_renda),
        || fill_match_record(estat, m, reg, sem_renda));
    PLACAR.registrar(igual && r == n, || {
        let difs: Vec<usize> = mo.0[0].1.iter().zip(mr.0[0].1.iter()).enumerate().filter(|(_, (a, b))| a != b).map(|(i, _)| i).take(16).collect();
        format!("fill_match_record(sem_renda={}) original={} rust={} bytes diferentes em {:x?}", sem_renda, r, n, difs)
    });
    r
}

unsafe extern "C" fn comparar_match_stats_compare_events(a: *const u8, b: *const u8) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const u8, *const u8) -> i32>(%%I1%%)(a, b);
    let n = match_stats_compare_events(a, b);
    PLACAR.registrar(r == n, || format!("match_stats_compare_events({:x?}, {:x?}) original={} rust={}",
        core::slice::from_raw_parts(a, 4), core::slice::from_raw_parts(b, 4), r, n));
    r
}

unsafe extern "thiscall" fn comparar_remove_language(estat: usize, ficha: usize, idioma: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8)>(%%I2%%);
    let tabela = *((estat + 0x3c) as *const usize);
    let contagem = *((estat + 0x44) as *const i32);
    if tabela == 0 || contagem <= 0 {
        return o(estat, ficha, idioma);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(estat + 0x44, 4), (tabela, contagem as usize * 6)],
        || o(estat, ficha, idioma), || remove_language(estat, ficha, idioma));
    PLACAR.registrar(igual, || format!("remove_language({}) contagem {:x?} x {:x?}", idioma, mo.0[0].1, mr.0[0].1));
}
