// ---------- %%LEVA%%a leva: a criação do jogo: inicializar os jogadores e as pessoas (Database.cpp) ----------

/// Uma célula (3 bytes: palavra com sinal + peso) dos vetores de atributos da 0x535a80.
fn cel_dbv1(v: &[u8; 189], k: isize) -> i16 {
    let i = (k * 3) as usize;
    i16::from_le_bytes([v[i], v[i + 1]])
}

fn por_dbv1(v: &mut [u8; 189], k: isize, x: i16) {
    let i = (k * 3) as usize;
    v[i..i + 2].copy_from_slice(&x.to_le_bytes());
}

fn peso_dbv1(v: &[u8; 189], k: isize) -> u8 {
    v[(k * 3) as usize + 2]
}

/// O peso que manda sortear de novo: 2 com a célula 0 (goleiro) a partir de 15, senão 1.
fn peso_alvo_dbv1(v: &[u8; 189]) -> u8 {
    if cel_dbv1(v, 0) >= 0xf { 2 } else { 1 }
}

/// Os 63 terminadores (palavra 0xff81, peso 0x81), como a 0x535a80 prepara os vetores.
unsafe fn limpar_vetor_dbv1(v: usize) {
    for k in 0..63 {
        core::ptr::write_unaligned((v + 3 * k) as *mut u16, 0xff81);
        *((v + 3 * k + 2) as *mut u8) = 0x81;
    }
}

/// A idade da 0x535a80: byte do ano da data − byte do ano de nascimento (+0x12), menos 1 se o dia do ano de
/// nascimento (+0x10) passa o da data.
unsafe fn idade_dbv1(p: usize, data: usize) -> i8 {
    let a = (*((data + 2) as *const u8)).wrapping_sub(*((p + 0x12) as *const u8));
    let d = core::ptr::read_unaligned((p + 0x10) as *const i16) as i32 - core::ptr::read_unaligned(data as *const i16) as i32;
    (if d > 0 { a.wrapping_sub(1) } else { a }) as i8
}

/// O ajuste do livre (sem clube) da 0x535a80: x − x × 0,25 (com +0x22) ou x − x × 0,6, truncado.
unsafe fn livre_dbv1(p: usize, x: i16) -> i16 {
    let x = x as f64;
    let k = if *((p + 0x22) as *const u8) != 0 { 0x9689e0 } else { 0x968e48 };
    ftol(x - x * f64_em(k)) as i16
}

fn faixa_dbv1(x: i16, a: i16, b: i16) -> bool {
    x >= a && x <= b
}

/// { 0x535a80 } (cdecl, Database.cpp): `database_player_initialisation(data)`, uma vez na criação (create_game
/// 0x828110), com a barra "Player Initialisation" (0x66f4e0, 0x802f80 a cada 1% das pessoas). Para cada pessoa: nome
/// comum (+0xc = −2: a linha guardada em [0x9b59b4], a primeira vazia ou uma sorteada), primeiro e segundo nome (−2:
/// 0x539c60, 0x53a1b0), as datas (+0x3e entrada no clube, +0x46, +0x31), os 8 atributos de pessoa (+0x56..+0x5d, fora
/// de 1..20: sorteados), o ajuste pela região da nação (+0x76), a nação que falta (seleção, clube ou sorteada). Do
/// jogador (+0x61): as habilidades (+0xb, +9, +0xd, nesta ordem, cada uma da que já existe), o potencial (+7: −2/−1/0
/// raros pelos sorteios 2000/750/100) e a atual (+5), a idade (nascimento sorteado fora de 14..50), as posições e os
/// lados no vetor (0x53a610), e os atributos fora de 1..20 por um molde: o jogador mais parecido entre 250 sorteados
/// (vetores globais 0xae2044/0xae2114, bandeira 0x9b59b8) ou, sem molde, pela habilidade. Do não jogador (+0x69): as
/// reputações (×50, +0xc/+0xa/+8), +4/+6, +0x13, o molde entre 250 (0xae18bc/0xae2238, 0x9b59b9), o cargo (+0x3d 8/9/10),
/// os nomes famosos (0x84b8b0: +0x43) e o cargo que falta. No fim de cada pessoa a idade (+0x18). Depois, a segunda
/// volta: os vetores relativos à habilidade (×10 − habilidade, ±200, /2) e as normalizações (0x538ec0, 0x538bc0,
/// 0x539490). Os vetores da pilha (0x54, 0x290) só são limpos na entrada: o que uma pessoa deixa passa à seguinte, como
/// na original; o melhor escore ([esp+0x40], −31073 na entrada) também. Sem pessoas: a caixa e o fim, pela original.
/// A local de [esp+0x114] (1, zerada com clube de reputação 0) nunca é lida.
unsafe extern "C" fn database_player_initialisation(data: usize) {
    if *(0xae2364 as *const i32) == 0 || *(0xae23bc as *const usize) == 0 {
        return orig::<unsafe extern "C" fn(usize)>(%%I0%%)(data);
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mostrar: unsafe extern "thiscall" fn(usize, usize, i32, u32, i32) = core::mem::transmute(0x802f80usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a1b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
    let semanas_antes: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let vet_j: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize = core::mem::transmute(0x53a610usize);
    let vet_nj: unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32 = core::mem::transmute(0x53a8b0usize);
    let de_vet_j: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53aa10usize);
    let de_vet_nj: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53a980usize);
    let nome_e: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x84b8b0usize);
    let norm_fisicos: unsafe extern "C" fn(usize) = core::mem::transmute(0x538ec0usize);
    let norm_jogador: unsafe extern "C" fn(usize) = core::mem::transmute(0x538bc0usize);
    let norm_nao_jogador: unsafe extern "C" fn(usize) = core::mem::transmute(0x539490usize);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let sb = |a: usize| *(a as *const i8);
    let b = |a: usize| *(a as *const u8);
    let wd = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let ww = |a: usize, v: i16| core::ptr::write_unaligned(a as *mut i16, v);
    let wb = |a: usize, v: u8| *(a as *mut u8) = v;
    // o sorteio em byte (al) e em palavra (ax)
    let s8 = |n: i32| sorteio(n) as i8;
    let s16 = |n: i32| sorteio(n) as i16;
    let mut lixo = [[0u32; 2]; 12]; // as datas de passagem da pilha (ninguém as lê depois)
    let l = |k: usize, t: &mut [[u32; 2]; 12]| t[k].as_mut_ptr() as usize;

    let cent = gi(0xae2364) / 100;
    texto(0xde1f64, 0x9b6414);
    mostrar(0xb74450, 0xde1f64, 0, 100, -1);
    let mut v1 = [0u8; 189];
    let mut v2 = [0u8; 189];
    limpar_vetor_dbv1(v1.as_mut_ptr() as usize);
    limpar_vetor_dbv1(v2.as_mut_ptr() as usize);
    let (mut barra, mut e0c, mut e10) = (0i8, 0i32, -0x7961i32);
    let (mut e19, mut e05, mut e11, mut e1a) = (0u8, 0u8, 0u8, 0u8);

    let mut i = 0i32;
    let mut p = rd(0xae23bc);
    while i < gi(0xae2364) {
        // o nome comum
        if gi(p + 0xc) == -2 {
            let mut k = gi(0x9b59b4);
            if k == -1 {
                let n = gi(0xae23a0);
                let base = rd(0xae23f8);
                let mut j = 0i32;
                let mut achou = false;
                while j < n {
                    if b(base.wrapping_add((j as usize).wrapping_mul(0x3c))) == 0 {
                        *(0x9b59b4 as *mut i32) = j;
                        achou = true;
                        break;
                    }
                    j += 1;
                }
                k = if achou { j } else { sorteio(n) };
            }
            wd(p + 0xc, rd(0xae23f8).wrapping_add((k as usize).wrapping_mul(0x3c)));
        }
        if gi(p + 4) == -2 {
            wd(p + 4, primeiro(rd(p + 0x1a)));
        }
        if gi(p + 8) == -2 {
            wd(p + 8, segundo(rd(p + 0x1a)));
        }
        // a entrada no clube (+0x3e)
        if rd(p + 0x39) == 0 {
            copiar(p + 0x3e, l(0, &mut lixo), data);
        }
        let ano = w(p + 0x40) as u16;
        if ano < 0x7b2 {
            if rd(p + 0x39) != 0 {
                acertar(p + 0x3e, 1, 0, 0x7cc, -1);
            } else {
                ww(p + 0x3e, w(data));
                ww(p + 0x40, w(data + 2));
            }
        } else if ano == 0x7cc && w(p + 0x3e) == 1 && rd(p + 0x39) != 0 {
            let nac = rd(rd(p + 0x39) + 0x53);
            if nac != 0 && gi(rd(nac + 0x71)) == gi(0x9cfa10) && comunitaria(p, nac) == 0 {
                let a = w(data + 2);
                let r = sorteio(2);
                let d = data_nova(l(1, &mut lixo), 1, (a as i32 - r - 1) as u32);
                copiar(p + 0x3e, l(2, &mut lixo), d);
            }
        }
        if rd(p + 0x39) != 0 && rd(rd(p + 0x39) + 0x53) == 0 {
            wd(p + 0x39, 0);
        }
        let d = semanas_antes(0xdd6ec8, l(3, &mut lixo), 5);
        let (dd, da, pa) = (w(d), w(d + 2) as u16, w(p + 0x40) as u16);
        if da < pa || (da == pa && dd < w(p + 0x3e)) {
            let d = semanas_antes(0xdd6ec8, l(4, &mut lixo), 5);
            copiar(p + 0x3e, l(5, &mut lixo), d);
        }
        for &(o, k) in &[(0x46usize, 6usize), (0x31, 8)] {
            let a = w(p + o + 2) as u16;
            if a < 0x7cf || a > 0x7da {
                let d = data_nova(l(k, &mut lixo), 0x1f, 0x76c);
                if w(d + 2) != w(p + o + 2) || w(d) != w(p + o) {
                    let d = data_nova(l(k + 1, &mut lixo), 0x1f, 0x76c);
                    copiar(p + o, l(10, &mut lixo), d);
                }
            }
        }
        // a barra, a cada 1%
        if e0c < cent {
            e0c += 1;
        } else {
            e0c = 0;
            barra = barra.wrapping_add(1);
            texto(0xde1f64, 0x9b6414);
            mostrar(0xb74450, 0xde1f64, barra as i32, 100, -1);
        }
        // os atributos de pessoa: −1 sorteia 5..20; fora de 1..20, o menor (min) ou a média de dois sorteios
        for &(o, min) in &[(0x56usize, true), (0x57, false), (0x58, false), (0x59, true), (0x5a, false), (0x5b, false), (0x5c, true)] {
            let v = sb(p + o);
            if v < 1 || v > 20 {
                let n = if v == -1 {
                    s8(0x10).wrapping_add(5)
                } else {
                    let a = s8(0x14).wrapping_add(1);
                    let c = s8(0x14).wrapping_add(1);
                    if min {
                        if a <= c { a } else { c }
                    } else {
                        ((c as i32 + a as i32) / 2) as i8
                    }
                };
                wb(p + o, n as u8);
            }
        }
        let v = sb(p + 0x5d);
        if v < 1 || v > 20 {
            let n = if v == -1 {
                s8(0xb).wrapping_add(10)
            } else {
                let a = s8(0xb).wrapping_add(5);
                let c = s8(0xb).wrapping_add(5);
                ((c as i32 + a as i32) / 2) as i8
            };
            wb(p + 0x5d, n as u8);
        }
        // o ajuste pela região da nação (+0x76)
        let nac = rd(p + 0x1a);
        if nac != 0 {
            let menos = |o: usize, n: i32| wb(p + o, b(p + o).wrapping_sub(sorteio(n) as u8));
            let mais = |o: usize, n: i32| wb(p + o, b(p + o).wrapping_add(sorteio(n) as u8));
            let t = sb(nac + 0x76) as i32;
            if (t - 4) as u32 <= 9 {
                match t {
                    6 | 8 => menos(0x5c, 10),
                    4 | 0xb | 0xc | 0xd => {
                        if t != 0xb && t != 0xd {
                            menos(0x5c, 0xf);
                            menos(0x5d, 0xf);
                        }
                        if t != 0xd {
                            mais(0x5c, 8);
                            mais(0x5d, 8);
                        }
                        menos(0x5c, 5);
                        menos(0x5d, 5);
                    }
                    _ => {}
                }
            }
            for o in [0x5cusize, 0x5d] {
                let v = sb(p + o);
                wb(p + o, if v < 1 { 1 } else if v > 20 { 20 } else { v as u8 });
            }
        }
        let t79 = rd(0xae2d0c).wrapping_add((i as usize).wrapping_mul(0x4f)) + 0xb;
        wb(t79, b(t79) & 0xf0);
        if rd(p + 0x1a) == 0 {
            let s = rd(p + 0x24);
            let c = rd(p + 0x39);
            if s != 0 && rd(s + 0x53) != 0 {
                wd(p + 0x1a, rd(s + 0x53));
            } else if c != 0 && rd(c + 0x53) != 0 {
                wd(p + 0x1a, rd(c + 0x53));
            } else {
                let r = sorteio(gi(0xae2350));
                wd(p + 0x1a, rd(0xae23a8).wrapping_add((r as usize).wrapping_mul(0x122)));
            }
        }

        let j = rd(p + 0x61);
        if j != 0 {
            // ---- o jogador: habilidades (+0xb, +9, +0xd), potencial (+7) e atual (+5) ----
            let ca0 = w(j + 5);
            let mut e24 = w(j + 7);
            if sorteio(2000) == 0 {
                e24 = -2;
            } else if sorteio(0x2ee) == 0 {
                e24 = -1;
            } else if sorteio(100) == 0 {
                e24 = 0;
            }
            let mut e34 = w(j + 0xd);
            let mut e2c = w(j + 9);
            let mut e20 = w(j + 0xb);
            wb(j + 0x45, 10);
            let nac = rd(p + 0x1a);
            let mut rn = sb(nac + 0x8e);
            if rn == 0 {
                formatar(0xde1f64, 0x9b63f0, w(nac + 0x8e) as i32);
                rn = s8(10).wrapping_add(1);
            }
            let rc: i8 = if rd(p + 0x39) == 0 {
                s8(2).wrapping_add(1)
            } else {
                let v = sb(rd(p + 0x39) + 0x80);
                if v != 0 { v } else { s8(4).wrapping_add(1) }
            };
            let e30 = rc as i32;
            let di = ((((e30 * 3 + rn as i32) >> 2) * 10) as i16).min(0x50);
            if !faixa_dbv1(e20, 1, 200) {
                let c = rd(p + 0x39);
                let mut x: i16 = if faixa_dbv1(e2c, 1, 200) && c != 0 && rd(p + 0x1a) == rd(c + 0x53) {
                    let r = s16(10);
                    let y = e2c.wrapping_sub(r).wrapping_add(s16(10));
                    if y < 1 { 1 } else if y > 200 { 200 } else { y }
                } else if faixa_dbv1(e34, 1, 200) {
                    if e34 < 0xb4 { e34.wrapping_sub(s16(10)).wrapping_add(0x14) } else { s16(10).wrapping_add(0xaa) }
                } else if e24 == -1 && ca0 > 100 {
                    s16(0x28).wrapping_add(0x3c)
                } else if faixa_dbv1(ca0, 1, 200) {
                    if ca0 > 0x4b { ca0.wrapping_sub(s16(0x32)) } else { s16(0x19).wrapping_add(1) }
                } else {
                    s16(di as i32).wrapping_add(1).min(0x19)
                };
                if rd(p + 0x39) == 0 {
                    x = livre_dbv1(p, x);
                }
                e20 = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            if !faixa_dbv1(e2c, 1, 200) {
                let mut x: i16 = if faixa_dbv1(e20, 1, 200) {
                    if e20 < 10 {
                        s16(5).wrapping_add(1)
                    } else if e20 > 0xbe {
                        s16(10).wrapping_add(0xb9)
                    } else {
                        let y = e20.wrapping_sub(s16(10));
                        y.wrapping_add(s16(10))
                    }
                } else if faixa_dbv1(e34, 1, 200) {
                    if e34 > 0xb4 { s16(10).wrapping_add(0xb4) } else { e34.wrapping_sub(s16(10)).wrapping_add(0x14) }
                } else if e24 == -1 {
                    s16(0x14).wrapping_add(0x50)
                } else if faixa_dbv1(ca0, 1, 200) {
                    if ca0 > 0x32 { ca0.wrapping_sub(s16(0x32)) } else { s16(0x19).wrapping_add(1) }
                } else {
                    s16(di as i32).wrapping_add(1).min(0x19)
                };
                if rd(p + 0x39) == 0 {
                    x = livre_dbv1(p, x);
                }
                e2c = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            if !faixa_dbv1(e34, 1, 200) {
                let mut x: i16 = if faixa_dbv1(e2c, 1, 200) {
                    if e2c > 100 { ftol(e2c as f64 * f64_em(0x9689e0)) as i16 } else { s16(3).wrapping_add(1) }
                } else if faixa_dbv1(e20, 1, 200) {
                    ftol(e20 as f64 * f64_em(if e20 < 0xaf { 0x968f20 } else { 0x9689e0 })) as i16
                } else if faixa_dbv1(ca0, 1, 200) {
                    let t = ftol(ca0 as f64 * f64_em(0x9689e0));
                    let r = sorteio(t);
                    s16(r).wrapping_add(1)
                } else {
                    ((di as i32 / 3) as i16).min(5)
                };
                if rd(p + 0x39) == 0 {
                    x = livre_dbv1(p, x);
                }
                e34 = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            let c = rd(p + 0x39);
            if c == 0 || rd(c + 0x53) == rd(p + 0x1a) {
                e2c = e20;
            }
            // a idade (nascimento sorteado fora de 14..50)
            let mut idade = idade_dbv1(p, data);
            if idade < 0xe || idade > 0x32 {
                let a18 = w(p + 0x18);
                let ano: u32 = if a18 > 0x76c { a18 as u16 as u32 } else if rd(p + 0x69) != 0 { 0x7af } else { 0x7bc };
                let mes = sorteio(0xb);
                let dia = sorteio(0x1b) + 1;
                acertar(p + 0x10, dia as u32, mes as u32, ano, -1);
                idade = idade_dbv1(p, data);
            }
            ww(p + 0x18, idade as i16);
            // o potencial (+7)
            let teto = |mut s: i16, ca0: i16| {
                if ca0 > s {
                    s = ca0;
                }
                if s < 10 { 10 } else if s > 200 { 200 } else { s }
            };
            if e24 == -2 {
                let mut s = if sorteio(10) != 0 { s16(0x51).wrapping_add(0x78) } else { s16(0x29).wrapping_add(0xa0) };
                if sorteio(0x14) == 0 {
                    s = s.wrapping_sub(s16(0x32));
                } else if s > 0xa5 {
                    s = s.wrapping_sub(s16(0x1e));
                }
                e24 = teto(s, ca0);
            } else if e24 == -1
                || (e24 > 0x8c && (idade as i32) < sorteio(0x1e) && ca0 < 0xaa && sorteio(0x271a - e30) == 0)
            {
                let mut s = if sorteio(10) != 0 { s16(0x51).wrapping_add(0x50) } else { s16(0x51).wrapping_add(0x78) };
                if sorteio(0x14) == 0 {
                    s = s.wrapping_sub(s16(0x32));
                } else if s > 0x8c {
                    s = s.wrapping_sub(s16(0x1e));
                }
                e24 = teto(s, ca0);
            } else if !faixa_dbv1(e24, 1, 200) || (idade < 0x19 && sorteio(0x19) == 0) {
                let mut k: i8 = if sorteio(100) == 0 {
                    s8(0x15)
                } else if sorteio(2) == 0 {
                    s8(6).wrapping_add(6)
                } else if sorteio(2000) == 0 {
                    s8(0x14).wrapping_add(1)
                } else {
                    s8(7).wrapping_add(4)
                };
                if k < 1 || k > 20 {
                    k = s8(10).wrapping_add(3);
                }
                let r = s16(0x3c);
                let s = r.wrapping_add(w((0x9b5988isize + k as isize * 2) as usize).wrapping_sub(0x1e));
                e24 = teto(s, ca0);
            }
            // a atual (+5)
            let mut ca = ca0;
            if !faixa_dbv1(ca0, 1, 200) {
                let k = s8(10).wrapping_add(0x14);
                let pa = e24 as i32;
                let a = s16(pa);
                let a = if a < 5 { 5 } else { a as i32 };
                let r = sorteio(((pa >> 2) * a) / 200);
                let y = (10000 - e30) / 200;
                let c16 = (r + y * 2 + 3) as i16;
                let f = ((k as i32) - (idade as i32)).abs().max(3);
                let v = pa - (((c16 as i32 * f) / 5) as i16 as i32);
                ca = if v > 200 { 200 } else if (v as i16) >= 1 { v as i16 } else { 1 };
                if ca == 1 && sorteio(10) != 0 {
                    ca = s16(pa / 5).wrapping_add(1);
                }
            }
            if rd(p + 0x39) == 0 && (b(p + 0x22) == 0 || idade_dbv1(p, data) > 0x20) {
                let r = sorteio(6); // a original guarda ca em [0x20] só para o fild
                let f = ftol(ca as f64 * f64_em(0x969518));
                ca = ca.wrapping_add(r.wrapping_mul(f) as i16);
                ca = if ca < 1 { 1 } else if ca > 200 { 200 } else { ca };
            }
            if ca > e24 {
                ca = e24.wrapping_sub(s16(0x28));
                if ca < 1 {
                    ca = s16(5).wrapping_add(1);
                }
            }
            ww(j + 5, ca);
            ww(j + 7, e24);
            ww(j + 0xb, e20.wrapping_mul(50));
            ww(j + 9, e2c.wrapping_mul(50));
            ww(j + 0xd, e34.wrapping_mul(50));
            wb(rd(0xae2d0c).wrapping_add((gi(p) as usize).wrapping_mul(0x4f)) + 9, 0x9c);
            // ---- as posições (células 0..8) e os lados (9..11) ----
            vet_j(j, p, v1.as_mut_ptr() as usize, &mut e19, &mut e05, &mut e11);
            if cel_dbv1(&v1, 0) > 0xf && cel_dbv1(&v1, 0) < 0x14 {
                por_dbv1(&mut v1, 0, 0x14);
                for k in [1isize, 2, 3, 4, 5, 6, 10, 11, 9] {
                    por_dbv1(&mut v1, k, 1);
                }
            }
            let rf = || (s8(6).wrapping_add(0xf)) as i16;
            // (a, c) = os valores de partida das células 5 e 4 no fim das posições
            let mut ac: Option<(i16, i16)> = None;
            let mut melhor: i8 = -1;
            let mut tem20 = false;
            for k in 0..8isize {
                let a = cel_dbv1(&v1, k);
                if a == 0x14 {
                    tem20 = true;
                    break;
                }
                if a > 5 && a < 0x14 && (melhor == -1 || a > cel_dbv1(&v1, melhor as isize)) {
                    melhor = k as i8;
                }
            }
            if !tem20 {
                if melhor > -1 {
                    por_dbv1(&mut v1, melhor as isize, 0x14);
                } else if sorteio(0x14) == 0 {
                    por_dbv1(&mut v1, 0, 0x14);
                    for k in [1isize, 2, 3, 5, 6, 10, 11, 9] {
                        por_dbv1(&mut v1, k, 1);
                    }
                    ac = Some((1, 1));
                } else {
                    let r = s16(0xb6);
                    // cada faixa do sorteio: as células sorteadas (15..20, nesta ordem) e as que viram 20
                    let (sorteadas, vinte): (&[isize], &[isize]) = if r < 1 {
                        (&[], &[1])
                    } else if r < 7 {
                        (&[1], &[2])
                    } else if r < 0xb {
                        (&[1, 3], &[2])
                    } else if r < 0xf {
                        (&[1, 7], &[2])
                    } else if r < 0x10 {
                        (&[1, 3, 4], &[2])
                    } else if r < 0x11 {
                        (&[1, 7, 4], &[2])
                    } else if r < 0x25 {
                        (&[], &[2])
                    } else if r < 0x31 {
                        (&[3], &[2])
                    } else if r < 0x3d {
                        (&[7], &[2])
                    } else if r < 0x49 {
                        (&[7, 4], &[2])
                    } else if r < 0x55 {
                        (&[3, 4], &[2])
                    } else if r < 0x56 {
                        (&[6], &[2])
                    } else if r < 0x65 {
                        (&[3], &[4])
                    } else if r < 0x67 {
                        (&[3, 5], &[4])
                    } else if r < 0x6a {
                        (&[7, 5], &[4])
                    } else if r < 0x7e {
                        (&[], &[4])
                    } else if r < 0x8c {
                        (&[5], &[4])
                    } else if r < 0x8e {
                        (&[6, 5], &[4])
                    } else if r < 0x8f {
                        (&[6], &[4])
                    } else if r < 0x95 {
                        (&[], &[4, 5])
                    } else if r < 0x9b {
                        (&[6], &[5])
                    } else if r < 0xa1 {
                        (&[5], &[6])
                    } else {
                        (&[], &[6])
                    };
                    for &k in vinte {
                        por_dbv1(&mut v1, k, 0x14);
                    }
                    for &k in sorteadas {
                        let x = rf();
                        por_dbv1(&mut v1, k, x);
                    }
                }
            }
            if ac.is_none() {
                // os lados: direito (9), esquerdo (10), centro (11)
                if cel_dbv1(&v1, 0) != 0x14 {
                    let mut lados = true;
                    if cel_dbv1(&v1, 1) >= 0xf {
                        por_dbv1(&mut v1, 11, 0x14);
                    } else if cel_dbv1(&v1, 7) >= 0xf && cel_dbv1(&v1, 9) < 0xf && cel_dbv1(&v1, 10) < 0xf {
                        if sorteio(10) >= 3 {
                            por_dbv1(&mut v1, 9, 0x14);
                            lados = false;
                        } else {
                            por_dbv1(&mut v1, 10, 0x14);
                        }
                    }
                    while lados {
                        if cel_dbv1(&v1, 9) >= 0x14 || cel_dbv1(&v1, 10) >= 0x14 || cel_dbv1(&v1, 11) >= 0x14 {
                            break;
                        }
                        if sorteio(0xb) < 4 || sorteio(cel_dbv1(&v1, 11) as i32) > 5 {
                            por_dbv1(&mut v1, 11, 0x14);
                        }
                        if sorteio(0xb) > 8 || sorteio(cel_dbv1(&v1, 10) as i32) > 5 {
                            por_dbv1(&mut v1, 10, 0x14);
                        }
                        if sorteio(0xb) < 4 || sorteio(cel_dbv1(&v1, 9) as i32) > 5 {
                            por_dbv1(&mut v1, 9, 0x14);
                            break;
                        }
                    }
                }
                let a = cel_dbv1(&v1, 5);
                let mut c = cel_dbv1(&v1, 4);
                if a >= 0xf && c < 0xf && cel_dbv1(&v1, 11) < 0xf {
                    c = a;
                }
                ac = Some((a, c));
            }
            let (a, c) = ac.unwrap();
            let m = (a as i32 / 2).max(c as i32);
            por_dbv1(&mut v1, 4, m as i8 as i16);
            let m = (cel_dbv1(&v1, 1) as i32 / 2).max(cel_dbv1(&v1, 2) as i32);
            por_dbv1(&mut v1, 2, m as i8 as i16);
            if cel_dbv1(&v1, 6) >= 0xf && cel_dbv1(&v1, 11) < 0xf {
                let x = s8(0xb).wrapping_add(10);
                let y = s8(7).wrapping_add(0xe) as i16;
                let v5 = cel_dbv1(&v1, 5);
                por_dbv1(&mut v1, 5, (if y > v5 { y } else { v5 }) as i8 as i16);
                let v4 = cel_dbv1(&v1, 4);
                por_dbv1(&mut v1, 4, (if x as i16 > v4 { x as i16 } else { v4 }) as i8 as i16);
            }
            // as células 12 e 13
            let um_a_vinte = || (s8(sorteio(0x14)).wrapping_add(1)) as i16;
            let mut a = cel_dbv1(&v1, 12);
            let mut c = cel_dbv1(&v1, 13);
            let mut conferir_c = true;
            if a == 0x14 || c == 0x14 {
            } else if faixa_dbv1(a, 1, 20) {
                if faixa_dbv1(c, 1, 20) && c <= a {
                    a = 0x14;
                    por_dbv1(&mut v1, 12, a);
                } else {
                    c = 0x14;
                    por_dbv1(&mut v1, 13, c);
                    conferir_c = false;
                }
            } else if faixa_dbv1(c, 1, 20) {
                a = 0x14;
                por_dbv1(&mut v1, 12, a);
            } else if cel_dbv1(&v1, 10) > cel_dbv1(&v1, 9) {
                por_dbv1(&mut v1, 12, 0x14);
                c = um_a_vinte();
                a = cel_dbv1(&v1, 12);
                por_dbv1(&mut v1, 13, c);
            } else {
                por_dbv1(&mut v1, 13, 0x14);
                let n = um_a_vinte();
                c = cel_dbv1(&v1, 13);
                por_dbv1(&mut v1, 12, n);
                a = n;
            }
            if conferir_c && !faixa_dbv1(c, 1, 20) {
                c = um_a_vinte();
                a = cel_dbv1(&v1, 12);
                por_dbv1(&mut v1, 13, c);
            }
            if !faixa_dbv1(a, 1, 20) {
                let n = um_a_vinte();
                c = cel_dbv1(&v1, 13);
                por_dbv1(&mut v1, 12, n);
                a = n;
            }
            if a != 0x14 && c != 0x14 {
                if sorteio(4) != 0 {
                    a = cel_dbv1(&v1, 12);
                    por_dbv1(&mut v1, 13, 0x14);
                } else {
                    a = 0x14;
                    por_dbv1(&mut v1, 12, a);
                }
            }
            // ---- os atributos fora de 1..20 (da célula 12 em diante): pelo molde ----
            let mut fora = false;
            if a != -0x7f {
                let mut k = 12isize;
                loop {
                    let x = cel_dbv1(&v1, k);
                    if x < 1 || x > 20 {
                        fora = true;
                        break;
                    }
                    k += 1;
                    if cel_dbv1(&v1, k) == -0x7f {
                        break;
                    }
                }
            }
            if fora {
                let mut molde = 0usize;
                let mut cont = 0xfai32;
                loop {
                    let r = sorteio(gi(0xae2364));
                    let q = rd(0xae23bc).wrapping_add((r as usize).wrapping_mul(110));
                    let cj = rd(q + 0x61);
                    if cj != 0 && faixa_dbv1(w(cj + 5), 1, 200) {
                        let mut e35 = 0i8;
                        if b(0x9b59b8) != 0 {
                            limpar_vetor_dbv1(0xae2044);
                            limpar_vetor_dbv1(0xae2114);
                            wb(0x9b59b8, 0);
                        }
                        vet_j(j, p, 0xae2044, &mut e1a, core::ptr::null_mut(), core::ptr::null_mut());
                        vet_j(cj, q, 0xae2114, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
                        let dif = (w(j + 5) as i32 - w(cj + 5) as i32).abs();
                        let mut esc = ftol(dif as f64 * f64_em(0x968e18));
                        let g0 = w(0xae2044);
                        let comparar = if g0 == 0x14 {
                            if w(0xae2114) == g0 {
                                esc = 0;
                                false
                            } else {
                                true
                            }
                        } else {
                            g0 != -0x7f
                        };
                        if comparar {
                            let mut o = 0isize;
                            loop {
                                let dv = w((0xae2114 + o) as usize);
                                if dv < 1 || dv > 20 {
                                    esc += 100;
                                } else {
                                    let sv = w((0xae2044 + o) as usize);
                                    if faixa_dbv1(sv, 1, 20) {
                                        if e35 == 0 {
                                            esc += (g0 as i32 - w(0xae2114) as i32).abs() * 1000;
                                        } else if e35 < e1a as i8 {
                                            esc += (sv as i32 - dv as i32).abs() * 25;
                                        } else {
                                            esc += (sv as i32 - dv as i32).abs() * 16;
                                        }
                                    }
                                }
                                e35 = e35.wrapping_add(1);
                                o = e35 as isize * 3;
                                if w((0xae2044 + o) as usize) == -0x7f {
                                    break;
                                }
                            }
                        }
                        if esc < e10 || molde == 0 {
                            molde = cj;
                            e10 = esc;
                        }
                    }
                    cont -= 1; // os 250 sorteados
                    if cont == 0 {
                        break;
                    }
                }
                let ini = e19 as i8 as isize;
                if molde == 0 {
                    let mut k = ini;
                    while cel_dbv1(&v1, k) != -0x7f {
                        let x = cel_dbv1(&v1, k);
                        if x < 1 || x > 20 {
                            let r = s8(7);
                            let c = r.wrapping_add(((w(j + 5) as i32 / 10) as i8).wrapping_sub(3));
                            let n = if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 };
                            por_dbv1(&mut v1, k, n);
                            if peso_dbv1(&v1, k) == peso_alvo_dbv1(&v1) {
                                por_dbv1(&mut v1, k, s16(5).wrapping_add(1));
                            }
                        }
                        k += 1;
                    }
                } else {
                    vet_j(molde, p, v2.as_mut_ptr() as usize, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
                    let mut hj = w(j + 5);
                    if !faixa_dbv1(hj, 1, 200) {
                        hj = s16(100).wrapping_add(1);
                    }
                    let mut hm = w(molde + 5);
                    if !faixa_dbv1(hm, 1, 200) {
                        hm = hj;
                    }
                    let mut k = ini;
                    while cel_dbv1(&v1, k) != -0x7f {
                        let x = cel_dbv1(&v1, k);
                        if faixa_dbv1(x, 1, 20) {
                            if peso_dbv1(&v1, k) == peso_alvo_dbv1(&v1) {
                                por_dbv1(&mut v1, k, (x as i32 - 10).max(1) as i8 as i16);
                            }
                        } else {
                            let t = cel_dbv1(&v2, k);
                            let n = if t > 0 && t < 0x15 {
                                let mut c: i8 = if k < e11 as i8 as isize {
                                    (((hj as i32 - hm as i32) / 10) as i8).wrapping_add(t as u8 as i8)
                                } else {
                                    let c = t as u8 as i8;
                                    if c <= 5 {
                                        c.wrapping_add(s8(5))
                                    } else if c >= 0xf {
                                        c.wrapping_sub(s8(4))
                                    } else {
                                        c
                                    }
                                };
                                c = c.wrapping_add(s8(6).wrapping_sub(3));
                                if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 }
                            } else {
                                let r = s8(0xb);
                                let c = r.wrapping_add(((w(j + 5) as i32 / 10) as i8).wrapping_sub(5));
                                if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 }
                            };
                            por_dbv1(&mut v1, k, n);
                            let o = k * 3;
                            if o == 0x9c || o == 0x9f {
                                let x = cel_dbv1(&v1, k);
                                if x < 0xc {
                                    por_dbv1(&mut v1, k, x.wrapping_add(s8(0xd - x as i32) as i16));
                                }
                            } else if (o == 0x6c || o == 0x6f || o == 0x78) && idade < 0x16 && cel_dbv1(&v1, k) > 0xf {
                                let x = cel_dbv1(&v1, k).wrapping_sub(s8(5) as i16);
                                por_dbv1(&mut v1, k, x);
                            }
                            if peso_dbv1(&v1, k) == peso_alvo_dbv1(&v1) {
                                por_dbv1(&mut v1, k, s16(10).wrapping_add(1));
                            }
                        }
                        k += 1;
                    }
                }
            }
            de_vet_j(j, p, v1.as_mut_ptr() as usize);
            if b(j + 0xf) == 0x14 && sb(j + 0x2e) < 8 {
                wb(j + 0x2e, (sorteio(6) as u8).wrapping_add(5));
            }
        } else {
            // ---- só não jogador: a idade fora de 29..75 sorteia o nascimento ----
            let idade = idade_dbv1(p, data);
            if idade < 0x1d || idade > 0x4b {
                let a18 = w(p + 0x18);
                let ano: u32 = if a18 > 0x76c && a18 < 0x7af { a18 as u16 as u32 } else { (sorteio(0x1a) + 0x794) as u32 };
                let mes = sorteio(0xb);
                let dia = sorteio(0x1b) + 1;
                acertar(p + 0x10, dia as u32, mes as u32, ano, -1);
            }
            ww(p + 0x18, idade_dbv1(p, data) as i16);
        }

        let nj = rd(p + 0x69);
        if nj != 0 {
            // ---- o não jogador: as reputações (+0xc, +0xa, +8) ----
            let a = w(nj + 0xc).wrapping_mul(50);
            let di = w(nj + 0xa).wrapping_mul(50);
            let bp = w(nj + 8).wrapping_mul(50);
            ww(nj + 0xa, di);
            ww(nj + 0xc, a);
            ww(nj + 8, bp);
            let rep = |x: usize| w(x + 0x80);
            let vezes = |x: usize, k: usize| ftol(rep(x) as f64 * f64_em(k)) as i16;
            if !faixa_dbv1(a, 1, 10000) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 0xc, if c != 0 && rep(c) > 0x1676 {
                    vezes(c, 0x969510)
                } else if s != 0 {
                    vezes(s, 0x968968)
                } else {
                    1
                });
            }
            let x = w(nj + 0xc);
            let bx = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            ww(nj + 0xc, bx);
            if !faixa_dbv1(di, 1, 10000) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 0xa, if faixa_dbv1(bp, 1, 10000) {
                    bp
                } else if faixa_dbv1(bx, 1, 10000) {
                    bx
                } else if c != 0 {
                    rep(c)
                } else if s != 0 {
                    vezes(s, 0x968978)
                } else {
                    1
                });
            }
            let x = w(nj + 0xa);
            let di2 = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            ww(nj + 0xa, di2);
            if !faixa_dbv1(bp, 1, 10000) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 8, if faixa_dbv1(di2, 1, 10000) {
                    di2
                } else if faixa_dbv1(bx, 1, 10000) {
                    bx
                } else if c != 0 {
                    vezes(c, 0x9689d8)
                } else if s != 0 {
                    vezes(s, 0x968978)
                } else {
                    1
                });
            }
            let x = w(nj + 8);
            ww(nj + 8, if x < 1 { 1 } else if x > 10000 { 10000 } else { x });
            let c = rd(p + 0x39);
            if c == 0 || rd(c + 0x53) == rd(p + 0x1a) {
                ww(nj + 8, di2);
            }
            if !faixa_dbv1(w(nj + 4), 1, 200) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 4, if c != 0 {
                    (rep(c) as i32 / 50) as i16
                } else if s != 0 {
                    (rep(s) as i32 / 50) as i16
                } else {
                    s16(100).wrapping_add(1)
                });
            }
            if !faixa_dbv1(w(nj + 6), 1, 200) {
                let r = s16(200).wrapping_add(1);
                let x = w(nj + 4);
                ww(nj + 6, if x > r { x } else { r });
            }
            let nac = rd(p + 0x1a);
            if nac != 0 && b(nj + 0x13) == 0 {
                let id = gi(nac);
                let casa = [0x9cf2dcusize, 0x9cf2e4, 0x9cf45c, 0x9cf3e4, 0x9cf510, 0x9cf408].iter().any(|&k| id == gi(k));
                let v = if casa {
                    let x = 0xdu8.wrapping_sub(sorteio(0xe) as u8);
                    x.wrapping_add(sorteio(8) as u8)
                } else {
                    let x = 5u8.wrapping_sub(sorteio(5) as u8);
                    x.wrapping_add(sorteio(7) as u8)
                };
                wb(nj + 0x13, v);
            }
            // ---- os atributos fora de 1..20: pelo molde ----
            vet_nj(nj, p, v1.as_mut_ptr() as usize, core::ptr::null_mut());
            let mut fora = false;
            if cel_dbv1(&v1, 0) != -0x7f {
                let mut k = 0isize;
                loop {
                    let x = cel_dbv1(&v1, k);
                    if x < 1 || x > 20 {
                        fora = true;
                        break;
                    }
                    k += 1;
                    if cel_dbv1(&v1, k) == -0x7f {
                        break;
                    }
                }
            }
            if fora {
                let c = rd(p + 0x39);
                if b(p + 0x3d) == 5 && c != 0 && rep(c) > 0x186a {
                    // o treinador de clube grande
                    let e_360 = || rd(p + 0x1a) != 0 && gi(rd(p + 0x1a)) == gi(0x9cf360);
                    let fora20 = |o: usize| sb(nj + o) < 1 || sb(nj + o) > 20;
                    if fora20(0x13) {
                        wb(nj + 0x13, (if e_360() { s8(0x10).wrapping_add(1) } else { s8(0xe).wrapping_add(7) }) as u8);
                    }
                    if fora20(0xe) {
                        let c = rd(p + 0x39);
                        wb(nj + 0xe, (if rep(c) > 0x1e46 {
                            if e_360() { s8(4).wrapping_add(0xd) } else { s8(8).wrapping_add(0xd) }
                        } else {
                            s8(9).wrapping_add(0xc)
                        }) as u8);
                    }
                    if fora20(0x1b) {
                        wb(nj + 0x1b, s8(0xd).wrapping_add(8) as u8);
                    }
                    if fora20(0x1f) {
                        wb(nj + 0x1f, s8(0xd).wrapping_add(8) as u8);
                    }
                    if fora20(0x21) {
                        let c = rd(p + 0x39);
                        wb(nj + 0x21, (if rep(c) > 0x1e46 {
                            if e_360() { s8(7).wrapping_add(0xe) } else { s8(9).wrapping_add(0xc) }
                        } else {
                            s8(0xb).wrapping_add(0xa)
                        }) as u8);
                    }
                }
                let mut molde = 0usize;
                let mut melhor = e10;
                let mut cont = 0xfai32;
                loop {
                    let r = sorteio(gi(0xae2364));
                    let q = rd(0xae23bc).wrapping_add((r as usize).wrapping_mul(110));
                    let cn = rd(q + 0x69);
                    if cn != 0 && faixa_dbv1(w(cn + 4), 1, 200) {
                        let mut e35 = 0i8;
                        if b(0x9b59b9) != 0 {
                            limpar_vetor_dbv1(0xae18bc);
                            limpar_vetor_dbv1(0xae2238);
                            wb(0x9b59b9, 0);
                        }
                        vet_nj(nj, p, 0xae18bc, core::ptr::null_mut());
                        vet_nj(cn, q, 0xae2238, core::ptr::null_mut());
                        let dif = (w(nj + 4) as i32 - w(cn + 4) as i32).abs();
                        let mut esc = ftol(dif as f64 * f64_em(0x968e18));
                        if w(0xae18bc) != -0x7f {
                            let mut o = 0isize;
                            loop {
                                let dv = w((0xae2238 + o) as usize);
                                if dv < 1 || dv > 20 {
                                    esc += 100;
                                } else {
                                    let sv = w((0xae18bc + o) as usize);
                                    if faixa_dbv1(sv, 1, 20) {
                                        esc += (sv as i32 - dv as i32).abs() * 16;
                                    }
                                }
                                e35 = e35.wrapping_add(1);
                                o = e35 as isize * 3;
                                if w((0xae18bc + o) as usize) == -0x7f {
                                    break;
                                }
                            }
                        }
                        if esc < melhor || molde == 0 {
                            molde = cn;
                            melhor = esc;
                        }
                    }
                    cont -= 1;
                    if cont == 0 {
                        break;
                    }
                }
                e10 = melhor;
                if molde != 0 {
                    vet_nj(molde, p, v2.as_mut_ptr() as usize, core::ptr::null_mut());
                    let mut hn = w(nj + 4);
                    if !faixa_dbv1(hn, 1, 200) {
                        hn = s16(100).wrapping_add(1);
                    }
                    let mut hm = w(molde + 4);
                    if !faixa_dbv1(hm, 1, 200) {
                        hm = hn;
                    }
                    if cel_dbv1(&v1, 0) != -0x7f {
                        let mut k = 0isize;
                        loop {
                            let x = cel_dbv1(&v1, k);
                            if x < 1 || x > 20 {
                                let t = cel_dbv1(&v2, k);
                                let c = if faixa_dbv1(t, 1, 20) {
                                    let d = (((hn as i32 - hm as i32) / 10) as i8).wrapping_add(t as u8 as i8);
                                    d.wrapping_add(s8(6).wrapping_sub(3))
                                } else {
                                    let r = s8(6);
                                    r.wrapping_add(((w(nj + 4) as i32 / 10) as i8).wrapping_sub(3))
                                };
                                por_dbv1(&mut v1, k, if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 });
                            }
                            k += 1;
                            if cel_dbv1(&v1, k) == -0x7f {
                                break;
                            }
                        }
                    }
                } else if cel_dbv1(&v1, 0) != -0x7f {
                    let mut k = 0isize;
                    loop {
                        let x = cel_dbv1(&v1, k);
                        if x < 1 || x > 20 {
                            let r = s8(7);
                            let c = r.wrapping_add(((w(nj + 4) as i32 / 10) as i8).wrapping_sub(3));
                            por_dbv1(&mut v1, k, if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 });
                        }
                        k += 1;
                        if cel_dbv1(&v1, k) == -0x7f {
                            break;
                        }
                    }
                }
            }
            de_vet_nj(nj, p, v1.as_mut_ptr() as usize);
            // ---- o cargo: olheiro (9), fisioterapeuta (8), treinador (10) ----
            let ate = |o: usize, max: i8, n: i32, base: u8| {
                if sb(nj + o) > max {
                    wb(nj + o, (sorteio(n) as u8).wrapping_add(base));
                }
            };
            let cargo = b(p + 0x3d);
            if cargo == 9 && sorteio(100) != 0 {
                let a17 = sb(nj + 0x17);
                wb(nj + 0xf, 1);
                wb(nj + 0x1e, 1);
                if a17 < 5 {
                    wb(nj + 0x17, (sorteio(5) as u8).wrapping_add(4));
                }
                if sb(nj + 0x18) < 4 {
                    wb(nj + 0x18, (sorteio(5) as u8).wrapping_add(3));
                }
                ate(0x10, 8, 8, 1);
                ate(0x11, 8, 8, 1);
                ate(0x12, 5, 5, 1);
                ate(0x13, 5, 5, 1);
                ate(0x1a, 5, 5, 1);
            } else if cargo == 8 && sorteio(100) != 0 {
                wb(nj + 0xf, 1);
                wb(nj + 0x1e, 1);
            } else if cargo == 0xa && sorteio(100) != 0 {
                let a1e = sb(nj + 0x1e);
                wb(nj + 0xf, 1);
                if a1e < 5 {
                    wb(nj + 0x1e, (sorteio(10) as u8).wrapping_add(4));
                }
                for o in [0x10usize, 0x11, 0x12, 0x13, 0x17, 0x18, 0x1a] {
                    ate(o, 5, 5, 1);
                }
            }
            // ---- os nomes famosos (+0x43) ----
            let nome = |s: usize| nome_e(p, s) != 0;
            if nome(0x9b63e0) {
                wb(nj + 0x43, 3);
                wb(nj + 0x13, 7);
                wb(nj + 0xe, 0x11);
            } else if nome(0x9b63d0) || nome(0x9b63c4) || nome(0x9b63b4) || nome(0x9b63a8) {
                wb(nj + 0x43, 3);
            } else if nome(0x9b6398) || nome(0x9b6388) || nome(0x9b6378) || nome(0x9b6368) || nome(0x9b635c) {
                wb(nj + 0x43, 2);
            } else if nome(0x9b634c) {
                wb(nj + 0x43, 6);
            } else {
                let x = sb(nj + 0x43);
                if x == 0 || x >= 10 {
                    let r = sorteio(0x1a);
                    if (r as u32) <= 0x19 {
                        const TABELA: [u8; 26] = [0, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8];
                        wb(nj + 0x43, TABELA[r as usize]);
                    }
                }
            }
            for k in 0..8 {
                wd(nj + 0x23 + 4 * k, 0);
            }
            // ---- o cargo que falta ----
            if b(p + 0x3d) == 0 && b(p + 0x28) == 0 {
                let (c21, c1b, a10, a11) = (sb(nj + 0x21), sb(nj + 0x1b), sb(nj + 0x10), sb(nj + 0x11));
                wb(p + 0x3d, if rd(p + 0x61) != 0 {
                    0xb
                } else if c21 > 10 && c1b > 10 {
                    5
                } else if (a10 > 6 || a11 > 6) && c1b > 10 && c21 > 10 {
                    6
                } else if a10 <= 6 && a11 <= 6 {
                    9 + (sb(nj + 0x1e) > 10) as u8
                } else {
                    8
                });
            }
        }
        ww(p + 0x18, idade_dbv1(p, data) as i16);
        i += 1;
        p += 110;
    }

    // ---- a segunda volta: os vetores relativos à habilidade e as normalizações ----
    let mut v3 = [0u8; 189];
    let mut q = rd(0xae23bc);
    let mut k = 0i32;
    while k < gi(0xae2364) {
        let jj = rd(q + 0x61);
        let nn = rd(q + 0x69);
        if jj != 0 {
            limpar_vetor_dbv1(v3.as_mut_ptr() as usize);
            let (mut fim_l, mut fim_p, mut fim_q) = (0u8, 0u8, 0u8);
            vet_j(jj, q, v3.as_mut_ptr() as usize, &mut fim_l, &mut fim_p, &mut fim_q);
            let a = fim_l.wrapping_add(2) as i8;
            let f = fim_q as i8;
            if a < f {
                for c in a as isize..f as isize {
                    let x = cel_dbv1(&v3, c).wrapping_mul(10).wrapping_sub(w(jj + 5));
                    let x = if x > 200 { 200 } else if x < -200 { -200 } else { x };
                    por_dbv1(&mut v3, c, (x as i32 / 2) as i16);
                }
            }
            de_vet_j(jj, q, v3.as_mut_ptr() as usize);
            norm_fisicos(q);
            norm_jogador(q);
        }
        if nn != 0 {
            limpar_vetor_dbv1(v3.as_mut_ptr() as usize);
            let mut fim = 0u8;
            vet_nj(nn, q, v3.as_mut_ptr() as usize, &mut fim);
            let mut c = fim as i8;
            while cel_dbv1(&v3, c as isize) != -0x7f {
                let x = cel_dbv1(&v3, c as isize).wrapping_mul(10).wrapping_sub(w(nn + 4));
                let x = if x > 200 { 200 } else if x < -200 { -200 } else { x };
                por_dbv1(&mut v3, c as isize, (x as i32 / 2) as i16);
                c = c.wrapping_add(1);
            }
            de_vet_nj(nn, q, v3.as_mut_ptr() as usize);
            norm_nao_jogador(q);
        }
        k += 1;
        q += 110;
    }
}

/// Comparador de 0x535a80: original e cópia lado a lado sobre o banco que ela escreve (pessoas, jogadores e não
/// jogadores inteiros, a tabela de 79 bytes por pessoa em [0xae2d0c]), [0x9b59b4], as bandeiras 0x9b59b8/9 e os quatro
/// vetores globais dos moldes, o texto de 0xde1f64, a data guardada da 0x54cab0 e o estado do sorteador (a cópia sem
/// contar sorteios). Sem pessoas (caixa e fim): só a original.
unsafe extern "C" fn comparar_database_player_initialisation(data: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(%%I0%%);
    let gi = |a: usize| (*(a as *const i32)).max(0) as usize;
    let rd = |a: usize| *(a as *const usize);
    if gi(0xae2364) == 0 || rd(0xae23bc) == 0 {
        return o(data);
    }
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regioes.extend_from_slice(&[(0x9b59b4, 4), (0x9b59b8, 2), (0xae2044, 0xbd), (0xae2114, 0xbd), (0xae18bc, 0xbd),
        (0xae2238, 0xbd), (0xde1f64, 0x100), (0xae24aa, 1), (0xae24c8, 8)]);
    regioes.push((rd(0xae23bc), gi(0xae2364) * 110));
    regioes.push((rd(0xae2d0c), gi(0xae2364) * 0x4f));
    if rd(0xae23c4) != 0 {
        regioes.push((rd(0xae23c4), gi(0xae236c) * 70));
    }
    if rd(0xae23c0) != 0 {
        regioes.push((rd(0xae23c0), gi(0xae2368) * 68));
    }
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regioes, || o(data),
        || crate::aleatorio::sem_contar(|| database_player_initialisation(data)));
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])));
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("database_player_initialisation: diverge em {:x} original={:02x} rust={:02x}", a, x, y),
        None => "database_player_initialisation: estado do sorteador diferente".to_string(),
    });
    // ponytail: dezenas de MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
}
