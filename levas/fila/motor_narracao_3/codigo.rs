// ---------- %%LEVA%%a leva: o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista ----------

/// Avança a posição de leitura da fila da narração (+0xc, anel de 12): marca a nova vaga (+0x3d =
/// −1, tira o bit 0x8000 do evento) e, chegando na de escrita (+8, e então +0x2d68 = −1) ou na
/// guardada (+0xa), desliga a fila (+5 = 0). Devolve se desligou.
unsafe fn avancar_leitura(t: usize) -> bool {
    let c = (t + 0xc) as *mut i16;
    *c = if c.read().wrapping_add(1) > 0xb { 0 } else { c.read().wrapping_add(1) };
    let p = t + 0x30 + *c as usize * 14;
    *((p + 0xd) as *mut u8) = 0xff;
    *(p as *mut u16) &= 0x7fff;
    let mut desligou = false;
    if *c == *((t + 8) as *const i16) {
        *((t + 0x2d68) as *mut u32) = u32::MAX;
        *((t + 5) as *mut u8) = 0;
        desligou = true;
    }
    if *c == *((t + 0xa) as *const i16) {
        *((t + 5) as *mut u8) = 0;
        desligou = true;
    }
    desligou
}

/// { 0x6d78b0 } (thiscall, ret 4, match_events.cpp): o próximo evento da narração a mostrar (0 com a
/// fila desligada, +5): pula as vagas vazias; no texto do evento (tabela [0xb68790 + 0x12 × (evento
/// − 8000)], registro +2 de 0x110 bytes, +0x108) anda até a próxima parte separada por `|` (a parte
/// atual em +0xd; o bit 0x8000 marca que há mais) e, acabado o texto, avança a leitura. Em `saida`,
/// o registro de 0x86 bytes da vaga (+0xd8).
unsafe extern "thiscall" fn proximo_evento(t: usize, saida: *mut usize) -> usize {
    if *((t + 5) as *const u8) == 0 {
        return 0;
    }
    if !saida.is_null() {
        *saida = (t as isize + 0xd8 + *((t + 0xc) as *const i16) as isize * 0x86) as usize;
    }
    let vaga = |t: usize| t + 0x30 + *((t + 0xc) as *const i16) as usize * 14;
    let mut p = vaga(t);
    while *(p as *const u16) == 0 {
        if avancar_leitura(t) {
            return 0;
        }
        p = vaga(t);
    }
    let evento = (*(p as *const u16) & 0x7fff).wrapping_add(0xe0c0) as i16 as isize;
    let registro = *(p as *const i16).add(1) as isize;
    let tabela = *((0xb68790 + evento * 0x12) as *const isize);
    let texto = *((tabela + registro * 0x110 + 0x108) as *const *const u8);
    let n = {
        let mut k = 0usize;
        while *texto.add(k) != 0 {
            k += 1;
        }
        k as i16
    };
    let parte = (p + 0xd) as *mut i8;
    let mut c = *parte;
    let mut s = 0i16;
    while s < n {
        if *texto.offset(s as isize) == b'|' {
            if c == -1 {
                *parte = 0;
                break;
            }
            c = c.wrapping_sub(1);
            if c < 0 {
                s += 1;
                *(p as *mut u16) |= 0x8000;
                *parte = (*parte).wrapping_add(1);
                break;
            }
        }
        s += 1;
    }
    if *parte != -1 {
        while s < n && *texto.offset(s as isize) != b'|' {
            s += 1;
        }
    }
    if n <= s {
        avancar_leitura(t);
    }
    p
}

/// { 0x6db2d0 } (cdecl, match_events.cpp): escreve em `buf` (`tam` bytes) o nome do jogador `jogador`
/// do lado `lado` da partida `pt` (registro de 0x49 bytes em +0x190: id e os três nomes +0x198..+0x1a0,
/// lados de 0x625 bytes) pela 0x53e140: com `curto` e sorteio(5) = 0 a forma 1; senão a forma 5, e se
/// algum dos 100 jogadores de um dos dois clubes (+0, +4; lista da 0x541f90) com outro id ou outro
/// nome tem a mesma forma 5, a forma `longa`.
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn nome_do_jogador_na_narracao(pt: usize, jogador: i8, lado: i8, buf: *mut u8, tam: u32, curto: u8, _x: u32, longa: u32) {
    let nome: unsafe extern "C" fn(u32, u32, u32, *mut u8, u32, u32, u32) = core::mem::transmute(0x53e140usize);
    let r = (pt as isize + jogador as isize * 0x49 + lado as isize * 0x625) as usize;
    let d = |o: usize| *((r + o) as *const u32);
    if curto != 0 && sortear(5) == 0 {
        nome(d(0x198), d(0x19c), d(0x1a0), buf, tam, 1, d(0x190));
        return;
    }
    nome(d(0x198), d(0x19c), d(0x1a0), buf, tam, 5, d(0x190));
    let lista: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let textos: unsafe extern "C" fn(*const u8, *const u8, i32, u32) -> i32 = core::mem::transmute(0x90c1d0usize);
    for k in 0..2 {
        let clube = *((pt + 4 * k) as *const u32) as usize;
        let mut l = [0usize; 100];
        lista(*(0xae23b4 as *const usize) + clube * 0x245, l.as_mut_ptr(), 1);
        for &q in l.iter() {
            if q == 0 {
                continue;
            }
            let n = |i: usize| *((*((q + 4 * i) as *const usize) + 0x33) as *const u32);
            if *(q as *const u32) == d(0x190) && n(1) == d(0x198) && n(2) == d(0x19c) && n(3) == d(0x1a0) {
                continue;
            }
            let mut outro = [0u8; 200];
            nome(n(1), n(2), n(3), outro.as_mut_ptr(), 200, 5, d(0x190));
            if textos(buf, outro.as_ptr(), 1, 0) == 0 {
                nome(d(0x198), d(0x19c), d(0x1a0), buf, tam, longa, d(0x190));
                return;
            }
        }
    }
}

/// { 0x6e86d0 } (cdecl, match_man.cpp): a habilidade do jogador `p` (2 = atual, senão potencial) como
/// o observador `o` (0 = o técnico do clube `c`, +0xcf) a vê: a reputação `rep` (negativa: pela
/// 0x5407e0) / 50 mistura com a habilidade verdadeira (ficha +0x61: +5 atual, +7 potencial) e um
/// ruído pseudoaleatório (id do jogador + [0xdd7120] + id do observador) que encolhe com o
/// conhecimento do observador (0x544410 da ficha +0x69, níveis +0x17/+0x18, até 20). Com a ficha
/// do observador, conhecimento alto e a semente favorável, a habilidade atual exata.
unsafe extern "C" fn habilidade_vista(o: usize, p: usize, c: usize, tipo: i8, rep: i16) -> i16 {
    let mut semente = (*(p as *const i32)).wrapping_add(*(0xdd7120 as *const i32));
    let mut rep = if rep > -1 {
        rep / 50
    } else {
        let relacao: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
        relacao(p, if o == 0 { *((c + 0xcf) as *const usize) } else { o }, 0) / 50
    };
    let ficha = *((p + 0x61) as *const usize);
    let idade = if tipo == 2 {
        0
    } else {
        (*((p + 0x18) as *const u8)).wrapping_sub(if *((ficha + 0xf) as *const u8) == 0x14 { 0x1a } else { 0x17 }) as i8
    };
    let mut conhece = 0usize;
    if o != 0 {
        conhece = *((o + 0x69) as *const usize);
        semente = semente.wrapping_add(*(o as *const i32));
        if conhece != 0 && tipo == 2 && rep > 0x32 {
            let k = chance_de_conhecer(conhece, *((conhece + 0x17) as *const i8)) as i8 as i32;
            if rep as i32 + (k - 10) * 10 > semente % 200 {
                return core::ptr::read_unaligned((ficha + 5) as *const i16);
            }
        }
    }
    let mut certeza: i8 = 1;
    if conhece != 0 {
        let f = ((semente % 101) as f64 * f64_em(0x967898)) as f32 as f64;
        let x = if semente % 10 == 0 {
            f * f32_em(0x96afb0) as f64
        } else {
            let nivel = *((conhece + if tipo == 2 { 0x17 } else { 0x18 }) as *const i8);
            (chance_de_conhecer(conhece, nivel) as i8 as f64) * f
        };
        certeza = (ftol(x + f64_em(0x968968)) as i8).min(0x14);
    }
    let c32 = certeza as i32;
    let ruido = (semente % ((0x15 - c32) * 2) + c32 - 0x15) as i16 as i32;
    let verdade = if tipo == 2 {
        core::ptr::read_unaligned((ficha + 5) as *const i16)
    } else {
        if idade > 0 {
            rep = rep.wrapping_sub((idade as i16).wrapping_mul(idade as i16));
        }
        core::ptr::read_unaligned((ficha + 7) as *const i16)
    } as i32;
    let peso = (c32 / 2 + 10) as i8 as i32;
    (((verdade + ruido) * peso + (20 - peso) * rep as i32) / 20) as i16
}

unsafe extern "C" fn comparar_habilidade_vista(o: usize, p: usize, c: usize, tipo: i8, rep: i16) -> i16 {
    let r = orig::<unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16>(%%I2%%)(o, p, c, tipo, rep);
    let n = habilidade_vista(o, p, c, tipo, rep);
    PLACAR.registrar(r == n, || format!("habilidade_vista({:#x}, {:#x}, {}, {}) original={} rust={}", o, p, tipo, rep, r, n));
    r
}

unsafe extern "thiscall" fn comparar_proximo_evento(t: usize, saida: *mut usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize) -> usize>(%%I0%%);
    let mut reg = vec![(t, 0x2db4)];
    if !saida.is_null() {
        reg.push((saida as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, saida), || proximo_evento(t, saida));
    PLACAR.registrar(igual && r == n, || format!("proximo_evento({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", t, r, n, mo, mr));
    r
}

// ponytail: escreve em `buf` e chama a 0x53e140 e a 0x541f90 (efeitos fora); comparação pelo texto
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_nome_do_jogador_na_narracao(pt: usize, jogador: i8, lado: i8, buf: *mut u8, tam: u32, curto: u8, x: u32, longa: u32) {
    let o = orig::<unsafe extern "C" fn(usize, i8, i8, *mut u8, u32, u8, u32, u32)>(%%I1%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((buf as usize, tam as usize));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pt, jogador, lado, buf, tam, curto, x, longa), || crate::aleatorio::sem_contar(|| nome_do_jogador_na_narracao(pt, jogador, lado, buf, tam, curto, x, longa)));
    PLACAR.registrar(igual, || format!("nome_do_jogador_na_narracao({}, {}) {:x?} x {:x?}", jogador, lado, mo, mr));
}
