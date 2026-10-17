// ---------- centesima quadragesima oitava leva: o motor nasce, prepara a partida e morre (0x6b7e50, 0x6b7f20, 0x6b97a0) ----------

/// { 0x6b7e50 } (match_eng.cpp): o construtor do motor. Os vetores de objetos pelo construtor de
/// vetores do runtime (0x9463af): os 40 jogadores (0x1be bytes, 0x6eada0), as duas taticas (0xd0,
/// 0x5961c0) e as duas copias das linhas taticas (0x18e3, 0x5b27c0); o rigor do arbitro (0x6e9460) e
/// a lista de eventos vazia (+0x478e).
unsafe extern "thiscall" fn construir_motor(m: usize) -> usize {
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let rigor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6e9460usize);
    vetor(m + 0x4796, 0x1be, 0x28, 0x6eada0, 0x6ebd60);
    vetor(m + 0x9032, 0xd0, 2, 0x5961c0, 0x6dbb10);
    vetor(m + 0x91e2, 0x18e3, 2, 0x5b27c0, 0x5a53c0);
    vetor(m + 0xc3a8, 0x18e3, 2, 0x5b27c0, 0x5a53c0);
    rigor(m + 0xf5b2);
    *((m + 0x478e) as *mut usize) = 0;
    m
}

/// { 0x6b97a0 } (match_eng.cpp): o destrutor do motor. O narrador proprio (+0x8e9d) e a lista de
/// eventos (+0x478e) sao destruidos e liberados; depois o rigor e os vetores, na ordem inversa.
unsafe extern "thiscall" fn destruir_motor(m: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5a8f60usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let narr = *((m + 0x8f7a) as *const usize);
    if *((m + 0x8e9d) as *const u8) != 0 && narr != 0 {
        destruir(narr);
        liberar(narr);
        *((m + 0x8f7a) as *mut usize) = 0;
        *((m + 0x8e9d) as *mut u8) = 0;
    }
    let lista = *((m + 0x478e) as *const usize);
    if lista != 0 {
        destruir_lista(lista);
        liberar(lista);
        *((m + 0x478e) as *mut usize) = 0;
    }
    destruir(m + 0xf5b2);
    vetor(m + 0xc3a8, 0x18e3, 2, 0x5a53c0);
    vetor(m + 0x91e2, 0x18e3, 2, 0x5a53c0);
    vetor(m + 0x9032, 0xd0, 2, 0x6dbb10);
    vetor(m + 0x4796, 0x1be, 0x28, 0x6ebd60);
}

/// O angulo (graus, 0..=359) da casa (i, j) vista do centro, para a tabela global 0xb67c30 que a
/// bola desviada usa (+0x8ed8); a original calcula com fpatan, o Rust com atan.
unsafe fn angulo_da_casa(i: i32, j: i32) -> u16 {
    let graus = f64_em(0x96d2b8);
    let mut a: i32 = if i == 0 {
        if j > 0 {
            0
        } else if j < 0 {
            0xb4
        } else {
            return 0;
        }
    } else if i > 0 && j == 0 {
        0x5a
    } else if i < 0 && j == 0 {
        0x10e
    } else if i > 0 {
        if j > 0 {
            ftol((i as f64 / j as f64).abs().atan() * graus)
        } else {
            ftol((j as f64 / i as f64).abs().atan() * graus + f64_em(0x96d2b0))
        }
    } else if j < 0 {
        ftol((i as f64 / j as f64).abs().atan() * graus + f64_em(0x96d2a8))
    } else {
        ftol((j as f64 / i as f64).abs().atan() * graus + f64_em(0x96d2a0))
    };
    if a as i16 > 0x167 {
        a = 0;
    } else if (a as i16) < 0 {
        a += 0x168;
    }
    a as u16
}

/// Monta e poe na lista de eventos (+0x478e, 0x68d0d0) o evento de pre-jogo de 0x19 bytes: a ficha,
/// o clube, as bandeiras, o titular, o lado (ou a vaga), o codigo e -1 nos campos livres.
unsafe fn evento_de_prejogo(m: usize, ficha: usize, clube: usize, bandeiras: u32, titular: u8, lado: u8, codigo: u16) {
    let mut ev = [0u8; 0x19];
    ev[0..4].copy_from_slice(&*(ficha as *const [u8; 4]));
    ev[4..8].copy_from_slice(&*(clube as *const [u8; 4]));
    ev[8..0xc].copy_from_slice(&bandeiras.to_le_bytes());
    ev[0xc] = titular;
    ev[0xd] = lado;
    ev[0xe] = 0;
    ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
    ev[0x13..0x15].copy_from_slice(&codigo.to_le_bytes());
    ev[0x15..0x19].copy_from_slice(&u32::MAX.to_le_bytes());
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    copiar(*((m + 0x478e) as *const usize), ev.as_ptr() as usize, 0x19);
}

/// { 0x6b7f20 } (ret 0x10, match_eng.cpp, 6 KB): prepara a partida `jogo` no motor. Zera o estado,
/// o clima (+0x4756: 0x923010, 0x923970, 0x923630 na original), os uniformes (0x6d47e0), o arbitro
/// (0x6e9470) e as condicoes do gramado (0x85cba0, +0x91d2), os dois elencos (0x6db500, com a caixa de
/// erro abaixo de 11), o narrador (dado ou novo, 0x6d5bf0), a forca dos lados e a marcacao; a lista
/// de eventos de pre-jogo (0x68d020) recebe os jogadores insatisfeitos com o contrato (0x13), o
/// interesse de outro clube (0x1e), a forma (5, 4) e o fim de contrato (6). Sem `rapida`, sorteia
/// quem comeca (+0x8eb6/+0x8eae) e a bola parada 8. Devolve 0 se a partida acabou antes (+0x8eba).
#[allow(clippy::cognitive_complexity)]
unsafe fn preparar_partida_com(m: usize, jogo: usize, rapida: u8, narrador: usize, ao_vivo: u8) -> u8 {
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut u16;
    let dw = |o: usize| (m + o) as *mut u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let zerar = |de: usize, ate: usize| core::ptr::write_bytes((m + de) as *mut u8, 0, ate - de);
    *((m + 0x4792) as *mut usize) = jogo;
    *b(0x8eb3) = 0;
    *b(0x8eb4) = 0;
    zerar(0x21e, 0x21e + 8000);
    *b(0x8ea5) = 0;
    *b(0x1a3) = 0;
    *b(0x1cc) = 0;
    *b(0x1cb) = 0;
    *w(0x1d6) = *((u(jogo, 0x1c) + 0x80) as *const u16);
    *w(0x1d8) = *((u(jogo, 0x20) + 0x80) as *const u16);
    *b(0xaf) = 0;
    *b(0xb0) = 0;
    *dw(0xf596) = 0;
    *dw(0xf59a) = 0;
    *w(0x8d4c) = 0xffff;
    *b(0x8d4e) = 0;
    *w(0x8d4a) = 0xffff;
    let tempo = (jogo + 0x4d) as *mut u16;
    if *((jogo + 0x4d) as *const u8) & 0xf > 9 {
        *tempo = (*tempo & 0xfff9) | 9;
    }
    if (*((jogo + 0x4d) as *const u8) & 0xf0) as u32 > 0x90 {
        *tempo = (*tempo & 0xff9f) | 0x90;
    }
    let clima = *(0xde2f74 as *const usize);
    let preparar_clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x923010usize);
    let ceu: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x923970usize);
    let vento: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x923630usize);
    preparar_clima(clima, m + 0x4756, jogo);
    *b(1) = ceu(clima, m + 0x4756, jogo);
    let v = vento(clima, jogo);
    *tempo = ((((v & 3) as i8 as i16) << 12) as u16) | (*tempo & 0xcfff);
    if *tempo & 0x2000 == 0 {
        *tempo &= 0xcfff;
        *b(0xf638) = 0xff;
    } else {
        *b(0xf638) = if sortear(2) != 0 { (sortear(0x19) as u8).wrapping_add(0xf) } else { (sortear(0x1e) as u8).wrapping_add(0x32) };
        if matches!(*b(1), 0 | 2 | 4) {
            *tempo &= 0xcfff;
            *b(0xf638) = 0xff;
        }
    }
    *dw(0xf56e) = (m + 0x97cd) as u32;
    *dw(0xf572) = (m + 0xb9f8) as u32;
    *dw(0x8ebc) = (m + 0x930f) as u32;
    *dw(0x8ec0) = (m + 0xabf2) as u32;
    *dw(0x8ec4) = (m + 0x9330) as u32;
    *dw(0x8ec8) = (m + 0xac13) as u32;
    if *(0xa15854 as *const u8) != 0 {
        for i in -8..9i32 {
            for j in -11..12i32 {
                *((0xb67c30 + ((i * 24 + j) * 2) as isize as usize) as *mut u16) = angulo_da_casa(i, j);
            }
        }
        *(0xa15854 as *mut u8) = 0;
    }
    *w(0xf639) = 0;
    *b(0x8eba) = 0;
    *w(0x8ed0) = 0;
    *b(0x8eb7) = 0;
    *b(0x8eb5) = 0;
    *w(0x8ed4) = 0x1ef;
    *w(0x8ed2) = 0;
    *b(0x1a8) = 0;
    *b(0x1a9) = 0;
    *b(0x1aa) = 0;
    *w(0x8ecc) = 0;
    *w(0x8ece) = 0;
    *b(4) = 0xb;
    *b(5) = 0xb;
    *w(0x1d2) = 0x2710;
    *w(0x1d4) = 0x2710;
    *dw(0x477a) = u32::MAX;
    for o in [0x1a4, 0x1cd, 8, 9, 6, 7, 0x1a5, 0x1a6] {
        *b(o) = 0;
    }
    *dw(0x4786) = 0;
    *dw(0x4782) = 0;
    *b(0x1ab) = 0xff;
    *b(0x1ac) = 0xff;
    *dw(0x476a) = 0xc47a0000;
    *dw(0x476e) = 0xc47a0000;
    *b(0x1c5) = 0;
    *b(0x1c6) = 0;
    *b(0x1c7) = 0;
    *dw(0x477e) = 0;
    *b(0x8eb1) = 0;
    *b(0x1a7) = 0;
    *b(0x8d48) = 0;
    *b(0x8d49) = 0;
    *w(0x475f) = 0xffff;
    *dw(0x4761) = 0;
    *dw(0x4765) = 0;
    *b(0x4769) = 0;
    *b(0x475a) = 0;
    for o in 0x475b..=0x475e {
        *b(o) = 0xff;
    }
    *dw(0x478a) = 0;
    *b(2) = 0xff;
    *b(3) = 0xff;
    zerar(0x8f02, 0x8f7a);
    zerar(0x1f6, 0x21e);
    zerar(0xa, 0xaf);
    zerar(0x8d51, 0x8e9b);
    *b(0x8e9b) = 0;
    *b(0x8eab) = 0xff;
    *w(0x8ed8) = 0xb4;
    *b(0x8eaa) = 0;
    *dw(0xf582) = 0;
    *b(0x8ea9) = 0;
    for o in [0xf576, 0xf57a, 0xf57e, 0xf5ae, 0x91de, 0xf586, 0xf58a, 0xf5a6, 0xf5aa, 0xf58e, 0xf592] {
        *dw(o) = 0;
    }
    zerar(0x215e, 0x215e + 0x97e * 4);
    *dw(0xf59e) = 0;
    *dw(0xf5a2) = 0;
    for l in 0..2usize {
        let e = 0xf5bd + 0x36 * l;
        zerar(e - 1, e + 0x10);
        zerar(e + 0x11, e + 0x35);
        *b(0x8d46 + l) = 0;
    }
    *dw(0xf628) = 0x40c00000;
    *dw(0xf62c) = 0x40c00000;
    *dw(0xf630) = 0x40a00000;
    *b(0) = rapida;
    if *(0xa5a390 as *const u8) == 0 {
        *b(0) = 1;
        *b(0x8e9d) = 0;
    }
    *b(0x8ebb) = ao_vivo;
    let (casa, fora) = (u(jogo, 0x1c), u(jogo, 0x20));
    *b(0x8d4f) = uniforme(casa, fora, 1);
    *b(0x8d50) = uniforme(fora, casa, 0);
    let x2e = *((jogo + 0x2e) as *const i16);
    let arbitro = if *(0xb74340 as *const u32) != 0 && *(jogo as *const i32) != -2 && x2e > -1 {
        (*(0xae23e8 as *const u32)).wrapping_add((x2e as i32 * 0x2b) as u32) as usize
    } else {
        0
    };
    iniciar_rigor(m + 0xf5b2, m as u32, arbitro);
    let comp = u(jogo, 0x14);
    if *(jogo as *const i32) != -2 && *(comp as *const u32) != *(0x9cf684 as *const u32) {
        let gramado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85cba0usize);
        *dw(0xf634) = gramado(*(0xdd7ef4 as *const usize), jogo, m + 0x91d2);
    } else {
        *dw(0xf634) = 0;
        *dw(0x91d2) = 0;
        *dw(0x91d6) = 0;
        *dw(0x91da) = 0;
    }
    if *dw(0x91d2) == 0 {
        *dw(0x91d2) = 100;
        *dw(0x91d6) = 0x5a;
        *dw(0x91da) = 0xa;
    }
    if *(jogo as *const i32) == -2 {
        *w(0x1ad) = 0;
    } else {
        // o desgaste do gramado (+0x1ad) e o quanto ele pesa (+0x1ae)
        let (g2, g6, ga) = (*dw(0x91d2) as i32 as f64, *dw(0x91d6) as i32 as f64, *dw(0x91da) as i32 as f64);
        let x = g6 * f64_em(0x9688a8) / g2;
        let t = (f64_em(0x9688a8) - x) as f32;
        let a = (ga * f64_em(0x96a5c0) + f64_em(0x9688a0)) - x * f64_em(0x968a80);
        let a = if !(a > f64_em(0x9688b0)) && a < f64_em(0x9689a0) { f64_em(0x9689a0) } else if a > f64_em(0x9688b0) { f64_em(0x9688b0) } else { a };
        *b(0x1ad) = ftol(a) as u8;
        let estadio = u(casa, 0x69);
        let base = if estadio == u(jogo, 0x18) || estadio == u(fora, 0x69) { f32_em(0x968e70) } else { f32_em(0x968970) } as f64;
        let c = (g6 * f64_em(0x96a5c0) + f64_em(0x9688a0)) - t as f64 * f64_em(0x968a80);
        let cf = c as f32 as f64;
        let menor = if c > f64_em(0x9688b0) { f64_em(0x9688b0) } else { cf };
        let r = if base > menor { base } else if cf > f64_em(0x9688b0) { f64_em(0x9688b0) } else { cf };
        *b(0x1ae) = ftol(r) as u8;
    }
    // os elencos
    let escalar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize, usize, usize, *mut i8) -> i8 = core::mem::transmute(0x6db500usize);
    let (mut n_casa, mut n_fora) = (0i8, 0i8);
    let pouco = |n: i8, linha: u32, texto: usize| {
        let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(0x549ef0usize);
        let data_texto: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32, u32) = core::mem::transmute(0x54ae10usize);
        let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
        let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
        let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
        let mut data = [0u8; 8];
        let mut quando = [0u8; 200];
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        let j = *((m + 0x4792) as *const usize);
        data_nova(data.as_mut_ptr(), *((j + 0x2a) as *const u16) as u32, *((j + 0x28) as *const u16) as u32);
        data_texto(data.as_mut_ptr(), quando.as_mut_ptr(), 200, 0xff);
        let id = *(j as *const i32);
        let comp = if id == -2 { 0xa158dc } else if id == -1 { 0xa158ec } else { u(j, 0x14) + 0x38 };
        formatar(0xde1f64, texto, n as i32, u(j, 0x1c) + 0x38, u(j, 0x20) + 0x38, comp, quando.as_ptr());
        separar(0xa15884, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
        formatar(0xae24d0, 0x9870e8, s);
        caixa(0x9870e0, 0xae24d0, 0);
        *(0xb67a34 as *mut u32) = 0;
    };
    let nc = escalar(m + 0x9032, m, u(casa, 0xcf), casa + 0xd7, m + 0x4796, 0, casa, fora, jogo, &mut n_casa);
    if nc < 0xb {
        pouco(nc, 0x264, 0xa158b0);
    }
    let nf = escalar(m + 0x9102, m, u(fora, 0xcf), fora + 0xd7, m + 0x6a6e, 1, fora, casa, jogo, &mut n_fora);
    if nf < 0xb {
        pouco(nf, 0x28c, 0xa15858);
    }
    if (n_fora as i32 + n_casa as i32) < 5 && ao_vivo == 0 {
        *b(0) = 1;
    }
    core::ptr::copy_nonoverlapping((m + 0x91e2) as *const u8, (m + 0xc3a8) as *mut u8, 0xc71 * 4 + 2);
    *w(0x1ce) = 0x6e;
    *w(0x1d0) = 0x6e;
    *dw(0x8f7a) = 0;
    if *b(0x8eba) != 0 {
        *b(0) = 1;
        *b(0x8e9d) = 0;
        return 0;
    }
    // o narrador
    if narrador != 0 {
        *dw(0x8f7a) = narrador as u32;
        *b(0x8e9d) = 0;
    } else {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let narrador_novo: unsafe extern "thiscall" fn(usize, u8) -> usize = core::mem::transmute(0x6d5bf0usize);
        let p = novo(0x2db1);
        *dw(0x8f7a) = if p != 0 { narrador_novo(p, *b(0x8ebb)) as u32 } else { 0 };
        *b(0x8e9d) = 1;
        *(*dw(0x8f7a) as usize as *mut u8) = 1;
    }
    forca_do_lado(m, 0);
    forca_do_lado(m, 1);
    marcacao_individual(m);
    *b(0x8ea5) = 1;
    let vagas = *(0xde1d08 as *const usize);
    let titular = |l: usize, k: i8| m + (if l == 0 { 0x4796 } else { 0x6a6e }) + (k as isize * 0x1be) as usize;
    for (l, o_clube, o_marca, o_destino) in [(0usize, 0x1c, 0x40, 0xf5cd), (1, 0x20, 0x41, 0xf603)] {
        let k = vaga_marcada(vagas, u(jogo, o_clube), (*((jogo + o_marca) as *const u8) == 0) as u32, 0);
        *b(o_destino) = if titular(l, k) == 0 {
            0xff
        } else {
            let k = vaga_marcada(vagas, u(jogo, o_clube), (*((jogo + o_marca) as *const u8) == 0) as u32, 0);
            *(titular(l, k) as *const u8)
        };
    }
    // a lista de eventos de pre-jogo
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let p = novo(0xe);
    let lista = if p != 0 { lista_nova(p) } else { 0 };
    *dw(0x478e) = lista as u32;
    'eventos: {
        if lista == 0 || e_selecao(casa) != 0 {
            break 'eventos;
        }
        if *b(0) != 0 {
            return 1;
        }
        let comp = u(jogo, 0x14);
        if *((jogo + 0x40) as *const u8) == 0 || comp == 0 {
            break 'eventos;
        }
        let amistosos = *(0xae2c74 as *const usize);
        if amistosos != 0 && e_amistoso(amistosos, comp) != 0 {
            break 'eventos;
        }
        eventos_de_prejogo(m, jogo);
    }
    if *b(0) == 0 {
        if sortear(2) != 0 {
            *b(0x8eb6) = 1;
            *b(0x8eae) = 0;
        } else {
            *b(0x8eb6) = 0;
            *b(0x8eae) = 1;
        }
        *b(0x8eb2) = 8;
        *b(0x1ca) = 0;
    }
    1
}

/// Os eventos de pre-jogo de cada jogador em campo (0x6b8d9b a 0x6b9745): o contrato (0x4cf5f0) e as
/// marcas da posicao (0x5481a0 contra 0x717400/0x717450) dao a insatisfacao (0x13, com pontos pela
/// ficha); a reputacao dele contra a do clube, o interesse (0x1e); a forma pela media (0x7c1e30) e
/// pelo indice de [0xb74350], 5 ou 4; o fim de contrato perto (0x54cdb0), 6.
#[allow(clippy::cognitive_complexity)]
unsafe fn eventos_de_prejogo(m: usize, jogo: usize) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let wq = |q: usize, o: usize| *((q + o) as *const i16);
    let contratos = *(0xae1050 as *const usize);
    let marcas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let vagas = *(0xde1d08 as *const usize);
    let indice = || {
        let t = *(0xb74350 as *const usize);
        let d = *((t + 0x18) as *const f64);
        if d == f64_em(0x9689a0) { f64_em(0x968998) } else { *((t + 0x10) as *const f64) / d }
    };
    for lado in 0..2i8 {
        let titular_da_vaga = |j: usize| {
            let (c, marca) = if lado == 0 { (u(jogo, 0x1c), *((jogo + 0x40) as *const u8)) } else { (u(jogo, 0x20), *((jogo + 0x41) as *const u8)) };
            let k = vaga_marcada(vagas, c, (marca == 0) as u32, 0);
            (j == m + 0x4796 + ((k as isize + lado as isize * 20) * 0x1be) as usize) as u8
        };
        for k in 0..20usize {
            let j = jogador_do_bloco(m, lado, k);
            if byte(j, 0x19) < 0 {
                continue;
            }
            let f = u(j, 0x69);
            let clube = if lado != 0 { u(jogo, 0x20) } else { u(jogo, 0x1c) };
            if e_selecao(clube) != 0 || u(clube, 0x53) == 0 {
                break;
            }
            if u(f, 0x61) == 0 {
                continue;
            }
            let c = contrato_com_clube(contratos, f, clube);
            if c == 0 {
                continue;
            }
            let a = marcas(f) as u16;
            let bb = ler_1bc(j);
            let d = tabela_b_do_jogador(j);
            let flags = bandeiras_do_contrato(c);
            let marcado = flags & 0x40 != 0 || *((c + 0x45) as *const u32) & 0x2000000 != 0;
            if marcado && a != 0 && ((d & a) == d || (bb & a) != bb) {
                crate::comparacao::ramo("prejogo insatisfeito");
                evento_de_prejogo(m, f, clube, 0, 0, k as u8, 0x13);
            } else if byte(f, 0x3d) != 0xc && a != 0 && (d & a) != d && (bb & a) != bb {
                let s: i16 = if a == 1 {
                    6
                } else if (d | 4) != 0 && byte(u(f, 0x61), 0x11) < 10 && byte(u(f, 0x61), 0x12) < 10 {
                    2
                } else if (d | 4) != 0 && (a | 4) != 0 {
                    -1
                } else if (d | 0x10) != 0 && (a | 0x10) != 0 {
                    0
                } else if (d | 0x40) != 0 && (a | 0x40) != 0 {
                    0
                } else {
                    1
                };
                let w = wq(u(f, 0x61), 0xb);
                let f5b = byte(f, 0x5b);
                let f57 = byte(f, 0x57);
                if w > 0xcb2
                    && (w > 0x1d4c || (w.wrapping_add((f57 as i16).wrapping_add(s.wrapping_mul(20)).wrapping_mul(25))) > wq(clube, 0x80))
                    && f5b < 0xf
                    && f57 > 5
                    && (byte(f, 0x5d) as i32 + f5b as i32) < 0x19
                {
                    let mut p: i16 = match s {
                        6 => 0x19,
                        2 => 5,
                        0 => -3,
                        _ => 0,
                    };
                    p += if f5b == 1 { 0x14 } else if f5b < 6 { 0xa } else { 5 };
                    if (a | 4) != 0 {
                        p -= 5;
                    }
                    if f57 == 1 {
                        p -= 3;
                    } else if f57 < 6 {
                        p -= 1;
                    }
                    if w > 0x186a {
                        p += if f57 > 0xf && f5b < 6 { 5 } else { 3 };
                    }
                    let f5a = byte(f, 0x5a);
                    if f5a > 0xa && byte(f, 0x18) < 0x18 && f57 > 0xf {
                        p += 5;
                    }
                    if f5a == 1 {
                        p -= 0xa;
                    } else if f5a < 6 {
                        p -= 5;
                    }
                    if *((c + 0x45) as *const u32) != 0 {
                        p += 5;
                    } else if bandeiras_do_contrato(c) != 0 {
                        p += 1;
                    } else {
                        p -= 3;
                    }
                    if bandeiras_do_contrato(c) & 0x40 != 0 {
                        p += 5;
                    }
                    if gosta_do_clube(f, clube) != 0 {
                        p -= 3;
                    }
                    if u(clube, 0xcf) != 0 && gosta_de(f, u(clube, 0xcf)) != 0 {
                        p -= 5;
                    }
                    if p > 5 && (p as i32) > sortear(0x1e) {
                        crate::comparacao::ramo("prejogo insatisfeito pela posicao");
                        evento_de_prejogo(m, f, clube, 0x40, 0, k as u8, 0x13);
                    }
                }
            }
            // o interesse de outro clube (0x6b917a)
            let pe = u(j, 0x6d);
            let r = sortear(7);
            if (byte(pe, 0x2f) as i32) > r + 6 && wq(pe, 0xb) > 0xea6 && byte(f, 0x5b) < 7 && (byte(f, 0x5b) as i32) < sortear(0xf) {
                let x = sortear(wq(u(j, 0x6d), 0xb) as i32);
                if sortear(wq(clube, 0x80) as i32) < x && *((c + 0x45) as *const u32) & 0x2f7a0b != 0 {
                    crate::comparacao::ramo("prejogo interesse");
                    evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 0x1e);
                }
            }
            // a forma
            let ficha_61 = u(f, 0x61);
            let mut forma = byte(f, 0x18);
            if ficha_61 != 0 {
                let x0f = byte(ficha_61, 0xf);
                if x0f == 0x14 {
                    forma = forma.wrapping_sub(2);
                } else if byte(ficha_61, 0x11) == 0x14 {
                    forma = forma.wrapping_sub(1);
                } else if byte(ficha_61, 0x15) == 0x14 || byte(ficha_61, 0x14) == 0x14 {
                    forma = forma.wrapping_add(1);
                }
                let x3c = byte(ficha_61, 0x3c);
                if x0f != 0x14 && x3c > 0xf {
                    forma = forma.wrapping_sub(2);
                } else if x3c > 0xa {
                    forma = forma.wrapping_sub(1);
                } else if x0f != 0x14 && x3c < 6 {
                    forma = forma.wrapping_add(1);
                }
            }
            let ficha_media = *(0xb74350 as *const usize);
            if forma < 0x17 {
                let m1 = media(ficha_media, ficha_61, 6, 1, 6.5) as f32;
                let m2 = media(ficha_media, u(f, 0x61), 6, 0x11, 6.5) as f32;
                let mut al = byte(f, 0x5b).wrapping_add(byte(f, 0x5a)).wrapping_add(byte(f, 0x18)).wrapping_sub(0x1e);
                if al < 4 {
                    al = 4;
                } else if forma < 0x12 && al > 0x19 {
                    al = 0x19;
                    let x3c = byte(u(f, 0x61), 0x3c);
                    if x3c < 0xa {
                        al = 0x19 - x3c;
                    }
                }
                let f61 = u(f, 0x61);
                if f61 != 0
                    && byte(f, 0x22) == 0
                    && wq(f61, 0xb) < 0x1b58
                    && wq(clube, 0x80) > 0xcb2
                    && (al as f64) < m1 as f64
                    && *((jogo + 0x40) as *const u8) != 0
                    && sortear(al as i32) == 0
                    && (m2 as f64) < indice()
                {
                    crate::comparacao::ramo("prejogo forma em baixa");
                    evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 5);
                }
            } else if forma > 0x20 {
                let m1 = media(ficha_media, ficha_61, 6, 1, 6.5) as f32;
                let m2 = media(ficha_media, u(f, 0x61), 6, 0x11, 6.5);
                if m1 as f64 > f32_em(0x9689a8) as f64 {
                    let mut z = m1 as f64 * f64_em(0x967880);
                    if !(z < f64_em(0x967890)) {
                        z = f64_em(0x967890);
                    }
                    if m2 < indice() - f64_em(0x967890) + z {
                        crate::comparacao::ramo("prejogo forma em alta");
                        evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 4);
                    }
                }
            }
            // o fim do contrato
            if sortear(byte(f, 0x5d) as i32) == 0 && byte(u(f, 0x61), 0x25) > 0xf {
                let d = dias(0xae2c90, *((f + 0x3e) as *const u32), *((f + 0x42) as *const u32));
                let r = sortear(200) + 0x32;
                if d > r && forma > 0x17 {
                    let g = byte(c, 0x3f);
                    if (g as i32) > sortear(4) + 2 {
                        crate::comparacao::ramo("prejogo fim de contrato");
                        evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 6);
                    }
                }
            }
        }
    }
}

/// { 0x6b7f20 } (ret 0x10, match_eng.cpp): ver `preparar_partida_com`.
unsafe extern "thiscall" fn preparar_partida(m: usize, jogo: usize, rapida: u32, narrador: usize, ao_vivo: u32) -> u8 {
    preparar_partida_com(m, jogo, rapida as u8, narrador, ao_vivo as u8)
}

// ponytail: o nascimento, a preparacao e o fim do motor nao tem comparacao propria (alocam o
// narrador e a lista, que a fita teria de reproduzir); validados pela carreira (Rust x original),
// como o laco do motor. As rotinas que eles chamam seguem comparadas.
unsafe extern "thiscall" fn comparar_construir_motor(m: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(325)(m)
}
unsafe extern "thiscall" fn comparar_preparar_partida(m: usize, jogo: usize, rapida: u32, narrador: usize, ao_vivo: u32) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8>(326)(m, jogo, rapida, narrador, ao_vivo)
}
unsafe extern "thiscall" fn comparar_destruir_motor(m: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(327)(m)
}
