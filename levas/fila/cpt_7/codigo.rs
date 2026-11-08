// ---------- %%LEVA%%a leva: o estádio neutro e o clima inicial: a nota do estádio para o jogo, o fator da distância no público e a leitura do weather.cfg (stadium.cpp, weather.cpp) ----------

unsafe fn cpt7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpt7_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpt7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A distância entre duas cidades (x em +0x23, y em +0x2b, doubles) × 44,44, truncada (_ftol).
unsafe fn cpt7_distancia(a: usize, b: usize) -> i32 {
    let dx = (cpt7_f(a + 0x23) - cpt7_f(b + 0x23)).abs();
    let dy = (cpt7_f(a + 0x2b) - cpt7_f(b + 0x2b)).abs();
    ftol((dy * dy + dx * dx).sqrt() * f64_em(0x96fe40))
}

/// { 0x85da00 } (thiscall, ret 0x14, stadium.cpp): `Stadium::score_venue_for_fixture(estadio, marcas, casa, fora,
/// jogo)`, pela busca do estádio neutro 0x85c800 (com as marcas 0x20 nos jogos da carreira). O estádio nacional da
/// Inglaterra (nação [0x9cf2e4] +0x80) antes de 2004: 0. A nota parte de 100000:
/// - marcas & 8, com as cidades dos dois clubes (+0x69 → +0x38) e a do estádio (+0x38) e mais de 10000 lugares (+0x3c):
///   pelas distâncias d1 (casa-fora), d2 (casa-estádio), d3 (fora-estádio): com d1 < 30, 200000/180000/140000 se d2 e
///   d3 < 15/30/50 e 50000 se uma passa de 100; senão 200000 se as duas < 0,588·d1 e 50000 se uma ≥ 0,8333·d1;
/// - marcas & 0x20: o público esperado (10000; seleção da casa 60000; senão a soma dos públicos +0x73 dos dois clubes,
///   no mínimo 5000 e, acima de 40000, a metade sem ficar abaixo de nenhum dos dois); lugares acima dele: × 1,2; acima
///   de 1,5× dele: / ((lugares − 1,5·esperado)·8,33e-5 + 1); abaixo de 0,3/0,5/0,7/0,9 dele: × 0,1/0,3/0,7/0,9;
/// - senão, com marcas & 4: a nota é sorteio(lugares).
/// No fim, nota × (sorteio(100) × 0,004 + 0,8), truncada.
unsafe extern "thiscall" fn score_venue_for_fixture(_this: usize, est: usize, marcas: u32, casa: usize, fora: usize, jogo: usize) -> i32 {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let inglaterra = cpt7_d(0xae23a8).wrapping_add(cpt7_d(0x9cf2e4).wrapping_mul(0x122));
    if est == cpt7_d(inglaterra + 0x80) && core::ptr::read_unaligned(0xae2c92 as *const u16) < 0x7d4 {
        return 0;
    }
    let m = marcas as u8;
    let mut nota: i32 = 100000;
    if m & 8 != 0 {
        let cc = cpt7_d(casa + 0x69);
        let cf = cpt7_d(fora + 0x69);
        let (cidc, cidf) = (if cc != 0 { cpt7_d(cc + 0x38) } else { 0 }, if cf != 0 { cpt7_d(cf + 0x38) } else { 0 });
        let cide = cpt7_d(est + 0x38);
        if cc != 0 && cidc != 0 && cf != 0 && cidf != 0 && cide != 0 && cpt7_i(est + 0x3c) > 0x2710 {
            let d1 = cpt7_distancia(cidc, cidf);
            let d2 = cpt7_distancia(cidc, cide);
            let d3 = cpt7_distancia(cidf, cide);
            if d1 < 0x1e {
                if d2 < 0xf && d3 < 0xf {
                    nota = 200000;
                } else if d2 < 0x1e && d3 < 0x1e {
                    nota = 180000;
                } else if d2 < 0x32 && d3 < 0x32 {
                    nota = 140000;
                } else if d2 >= 0x64 || d3 >= 0x64 {
                    nota = 50000;
                }
            } else {
                let (f1, f2, f3) = (d1 as f64, d2 as f64, d3 as f64);
                let perto = f1 * f64_em(0x96fe38);
                let longe = f1 * f64_em(0x969088);
                if f2 < perto && f3 < perto {
                    nota = 200000;
                } else if !(f2 < longe && f3 < longe) {
                    nota = 50000;
                }
            }
        }
    }
    let lugares = cpt7_i(est + 0x3c);
    if m & 0x20 != 0 {
        let mut esperado: i32 = 0x2710;
        if jogo != 0 {
            if is_national_team(cpt7_d(jogo + 0x1c)) != 0 {
                esperado = 0xea60;
            } else {
                let a = cpt7_i(cpt7_d(jogo + 0x20) + 0x73);
                let c = cpt7_i(cpt7_d(jogo + 0x1c) + 0x73);
                let s = c.wrapping_add(a);
                if s < 0x1388 {
                    esperado = 0x1388;
                } else if s > 0x9c40 {
                    let mut h = s / 2;
                    if h <= c {
                        h = c;
                    }
                    if h <= a {
                        h = a;
                    }
                    esperado = h;
                } else {
                    esperado = s;
                }
            }
        }
        if lugares > esperado {
            nota = ftol(nota as f64 * f64_em(0x968978));
        }
        let (l, e) = (lugares as f64, esperado as f64);
        let t = e * f64_em(0x968940);
        if l > t {
            nota = ftol(nota as f64 / ((l - t) * f64_em(0x96fe30) + f64_em(0x967890)));
        } else if l < e * f64_em(0x968e80) {
            nota = ftol(nota as f64 * f64_em(0x967880));
        } else if l < e * f64_em(0x968968) {
            nota = ftol(nota as f64 * f64_em(0x968e80));
        } else if l < e * f64_em(0x968e40) {
            nota = ftol(nota as f64 * f64_em(0x968e40));
        } else if l < e * f64_em(0x9689d8) {
            nota = ftol(nota as f64 * f64_em(0x9689d8));
        }
    } else if m & 4 != 0 {
        nota = cpt7_sortear(lugares);
    }
    crate::comparacao::ramo("cpt_7 nota do estadio");
    let r = cpt7_sortear(100); // cpt7: o sorteio da nota
    ftol((r as f64 * f64_em(0x968d60) + f64_em(0x9689b0)) * nota as f64)
}

// ponytail: sorteia (e, com a marca 4, sorteia os lugares): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_score_venue_for_fixture(this: usize, est: usize, marcas: u32, casa: usize, fora: usize, jogo: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32>(%%I0%%)(this, est, marcas, casa, fora, jogo)
}

/// { 0x85d8a0 } (thiscall, ret 0x14, stadium.cpp): `Stadium::get_travel_distance_factor(jogo, cidade_do_estadio,
/// cidade_do_clube, base)`, no público do jogo (0x85cba0, pelo motor 0x6b7f20). D = distância entre as cidades
/// (pela metade com o jogo +0x30 = 0xa0 e +0x3a = 5); F = 1 − D·[0x96afe0], ao quadrado sem +0x3c; com D e a
/// reputação R (byte +0x7e da nação +0x53 do clube de fora +0x20) acima de [0x9689a0]: F / (R + 1)². Com base acima de
/// [0x969558], o mínimo 1 − ([0x968ea8] − base)·[0x968f20]; com +0x3a = 10, o mínimo 0,2; com os dois clubes da mesma
/// nação, 0,05; e sempre [0x96d2c8]. Devolve o double no st0.
unsafe extern "thiscall" fn get_travel_distance_factor(_this: usize, jogo: usize, cid_est: usize, cid_clube: usize, base: f64) -> f64 {
    let nacao = cpt7_d(cpt7_d(jogo + 0x20) + 0x53);
    let r = *((nacao + 0x7e) as *const i8) as i32 as f64;
    let dy = cpt7_f(cid_est + 0x2b) - cpt7_f(cid_clube + 0x2b);
    let dx = cpt7_f(cid_est + 0x23) - cpt7_f(cid_clube + 0x23);
    let mut d = (dx * dx + dy * dy).sqrt();
    let tipo = *((jogo + 0x3a) as *const u8);
    if core::ptr::read_unaligned((jogo + 0x30) as *const u16) == 0xa0 && tipo == 5 {
        d *= f64_em(0x968968);
    }
    let mut f = f64_em(0x967890) - d * f64_em(0x96afe0);
    if *((jogo + 0x3c) as *const u8) == 0 {
        f *= f;
    }
    if d > f64_em(0x9689a0) && r > f64_em(0x9689a0) {
        let q = r + f64_em(0x967890);
        f /= q * q;
    }
    if base > f64_em(0x969558) {
        let minimo = f64_em(0x967890) - (f64_em(0x968ea8) - base) * f64_em(0x968f20);
        if f < minimo {
            return minimo;
        }
    }
    if tipo == 0xa && f < f64_em(0x968918) {
        return f64_em(0x968918);
    }
    if f < f64_em(0x967888) && cpt7_d(cpt7_d(jogo + 0x1c) + 0x53) == nacao {
        return f64_em(0x967888);
    }
    if f < f64_em(0x96d2c8) {
        return f64_em(0x96d2c8);
    }
    f
}

/// Comparador: pelo valor (os bits do double).
unsafe extern "thiscall" fn comparar_get_travel_distance_factor(this: usize, jogo: usize, cid_est: usize, cid_clube: usize, base: f64) -> f64 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64>(%%I1%%)(this, jogo, cid_est, cid_clube, base);
    let n = get_travel_distance_factor(this, jogo, cid_est, cid_clube, base);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("get_travel_distance_factor(jogo {:x}, base {}) {} x {}", jogo, base, r, n));
    r
}

/// { 0x922810 } (thiscall, weather.cpp): `Weather::load_config()`, na criação do jogo (0x9226c0, Setup.cpp). Abre o
/// CM3_DATA\weather.cfg (0x90bde0, fopen 0x945b00 "r"); a primeira linha (0x90cba0, buffer de 1000) dá quantas regiões
/// ("%ld" em +8); as regiões (malloc de 0x6e bytes cada em +0) e a tabela das cidades (malloc de 4 bytes por cidade
/// [0xae2354] em +4); cada região pela 0x922c40 (a primeira que falha para o jogo; as outras só encerram a leitura);
/// cada cidade sorteia as três medidas ((sorteio(100) + sorteio(100)) × 0,5) e o vento (sorteio(3)). Fecha o arquivo
/// (0x945a09). Os erros (arquivo, linha, contagem, memória) são a caixa "Error" de weather.cpp e o jogo para.
unsafe extern "thiscall" fn load_config(this: usize) {
    let pasta: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler_linha: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(0x90cba0usize);
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(0x946fd8usize);
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
    let regiao: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x922c40usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let f = abrir(pasta(0x9b5348, 0xad4e18), 0x9899fc);
    if f == 0 {
        erro_em(0xad4dec, 0x68);
    }
    let mut linha = [0u8; 1000];
    if ler_linha(f, linha.as_mut_ptr()) == 0 {
        erro_em(0xad4dec, 0x6f);
    }
    if ler(linha.as_ptr(), 0x98c11c, this + 8) != 1 {
        erro_em(0xad4dec, 0x75);
    }
    let r = malloc(cpt7_d(this + 8).wrapping_mul(0x6e));
    *(this as *mut usize) = r;
    if r == 0 {
        erro_em(0xad4dec, 0x7e);
    }
    let c = malloc(cpt7_d(0xae2354).wrapping_mul(4));
    *((this + 4) as *mut usize) = c;
    if c == 0 {
        erro_em(0xad4dec, 0x86);
    }
    let mut i = 0i32;
    while i < cpt7_i(this + 8) {
        if regiao(this, f, cpt7_d(this) + i as usize * 0x6e) == 0 {
            if i == 0 {
                erro_em(0xad4dec, 0x92);
            }
            break;
        }
        i += 1;
    }
    let mut k = 0usize;
    while (k as i32) < cpt7_i(0xae2354) {
        crate::comparacao::ramo("cpt_7 clima inicial");
        for j in 0..3usize {
            let a = cpt7_sortear(100);
            let b = cpt7_sortear(100); // cpt7: o segundo sorteio da medida
            *((cpt7_d(this + 4) + k * 4 + j) as *mut u8) = ftol(a.wrapping_add(b) as f64 * f64_em(0x968968)) as u8;
        }
        let v = cpt7_sortear(3); // cpt7: o vento inicial
        *((cpt7_d(this + 4) + k * 4 + 3) as *mut u8) = v as u8;
        k += 1;
    }
    fechar(f);
}

// ponytail: abre o arquivo, aloca e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_load_config(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
