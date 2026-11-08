use super::*;

// ---------- centesima quadragesima primeira leva: a lesao (0x6d0980, 0x62be00) e o destaque do jogador (0x717470) ----------

/// Unidades da fita da lesao: a 0x68d0d0 recebe o registro montado na pilha (confere o conteudo).
pub(crate) static GRAVAR_LESAO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(crate) fn gravando_lesao() -> bool {
    GRAVAR_LESAO.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// { 0x62be00 } (ret 0xc, injury.cpp) com a copia na lista injetada (0x68d0d0): registra a lesao da
/// ficha na lista de lesoes do objeto (+0x28; sem lista, a original, que a cria). Ficha ja na lista
/// (pelo id): mesmo tipo soma a gravidade (+5, no maximo 100); outro tipo fica o mais grave. Ficha nova:
/// o registro (id, tipo, gravidade) vai para a lista. Devolve em `tipo` e `chance` o que ficou.
pub(crate) unsafe fn registrar_lesao_com(this: usize, ficha: usize, tipo: *mut u8, chance: *mut u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let l = *((this + 0x28) as *const usize);
    if l == 0 {
        crate::comparacao::ramo("lesao lista nova");
        let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(314);
        return o(this, ficha, tipo, chance);
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let id = *(ficha as *const u32);
    let mut e = prox(l);
    while e != 0 && *(e as *const u32) != id {
        e = prox(l);
    }
    if e == 0 {
        crate::comparacao::ramo("lesao registrada");
        let mut r = [0u8; 6];
        r[..4].copy_from_slice(&id.to_le_bytes());
        r[4] = *tipo;
        r[5] = *chance;
        pedir_vetores(&[(r.as_ptr() as usize, 6)], &[]);
        chamar(0x68d0d0, l, [0, 6, 0, 0, 0]);
        return;
    }
    let (t, g) = ((e + 4) as *mut u8, (e + 5) as *mut u8);
    if *t == *tipo {
        crate::comparacao::ramo("lesao agravada");
        *g = if *chance as i8 as i32 + *g as i8 as i32 > 0x64 { 0x64 } else { (*chance).wrapping_add(*g) };
    } else if *chance as i8 > *g as i8 {
        crate::comparacao::ramo("lesao mais grave");
        *g = *chance;
        *t = *tipo;
    }
    *tipo = *t;
    *chance = *g;
}

pub(crate) unsafe fn despachar_lesao(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    // 0x68d0d0: o bloco pedido por ultimo (pedir_vetores) e o que vai para a lista
    let p = VETORES_PEDIDOS.lock().unwrap_or_else(|e| e.into_inner()).take().map_or(0, |v| v.0[0].0);
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(rotina);
    f(this, p, a[1]);
    0
}

/// the record's injury in the injury list (+0x28 of [0xb63d28]): same injury adds the severity (up to 100), another
/// keeps the worse; a new record goes to the list (0x68d0d0, the record built on the stack checked by the tape). No
/// list: the original (creates the list)
/// Original: `register_injury` (0x62be00, injury.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x62be00 }: ver `registrar_lesao_com`.
pub(crate) unsafe extern "thiscall" fn registrar_lesao(this: usize, ficha: usize, tipo: *mut u8, chance: *mut u8) {
    registrar_lesao_com(this, ficha, tipo, chance, &mut |r, t, a| despachar_lesao(r, t, a))
}

/// { 0x6d0980 } (ret 0x10, match_eng.cpp) com a copia na lista de lesoes injetada (0x68d0d0): a lesao
/// do jogador `p` na partida. Com mais de 3 lesoes no jogo (+0xf639), sorteio(lesoes) e sorteio(500)
/// nao nulos a evitam; mais de 6, nunca. Tipo valido: registra (0x62be00, que devolve tipo e
/// gravidade). +0x55 = `o` (o causador perde 10 em +0x31). Gravidade acima de 50: tira gravidade^2 do
/// folego (+0x4d) e marca +0x1c; senao tira sorteio(gravidade^2 / 2), grava sorteio(isso x 0,5) em
/// +0x59 e reduz +0xd5 a metade. Folego abaixo do limite: sai (+0x8eb5, +0x1d, +0x1c); senao para
/// (0x7160e0 com 5 ou 3) conforme a forca da pessoa (+0x2d x 25). Folego entre 1 e o teto; saindo,
/// mais uma lesao no jogo.
pub(crate) unsafe fn lesionar_com(m: usize, tipo: u32, chance: u32, p: usize, o: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let w = |x: usize| (m + x) as *mut i16;
    let b = |q: usize, x: usize| (q + x) as *mut u8;
    let n = *w(0xf639);
    if n > 3 && sortear(n as i32) != 0 && sortear(0x1f4) != 0 {
        return;
    }
    if *w(0xf639) > 6 {
        return;
    }
    let (mut t, mut g) = (tipo as u8, chance as u8);
    if t as i8 > -1 {
        registrar_lesao_com(*(G_INJURY as *const usize), *((p + 0x69) as *const usize), &mut t, &mut g, chamar);
    }
    *((p + 0x55) as *mut usize) = o;
    if o != 0 {
        let q = (o + 0x31) as *mut i16;
        *q = (*q).wrapping_sub(10);
    }
    let folego = (p + 0x4d) as *mut f32;
    let c = g as i8 as i32;
    let mut local = 0f32;
    if g as i8 > 0x32 {
        crate::comparacao::ramo("lesao grave");
        *b(p, 0x1c) = 1;
        *folego = (*folego as f64 - (c * c) as f64) as f32;
    } else {
        local = sortear(ftol((c * c) as f64 * f64_em(0x968968))) as f32;
        *folego = (*folego as f64 - local as f64) as f32;
        let r = sortear(ftol(local as f64 * f32_em(0x968f18) as f64));
        *((p + 0x59) as *mut f32) = r as f32;
        let d5 = (p + 0xd5) as *mut f32;
        *d5 = (*d5 as f64 * f32_em(0x968f18) as f64) as f32;
    }
    if !(*folego as f64 >= f64_em(0x9688e8)) {
        crate::comparacao::ramo("lesao sem folego");
        *b(m, 0x8eb5) = 1;
        *b(p, 0x1d) = 1;
        *b(p, 0x1c) = 1;
    } else {
        let pessoa = *((p + 0x6d) as *const usize);
        let v = (byte(pessoa, 0x2d) as i32 * 25) as f64 + f64_em(0x96d458);
        if v > *folego as f64 {
            crate::comparacao::ramo("lesao para 5");
            gravar_2b(p, 5);
            *b(m, 0x8eb5) = 1;
            *b(p, 0x1d) = 1;
        } else {
            gravar_2b(p, 3);
            if local >= f32_em(0x96d1a8) && *b(p, 0x1c) != 0 {
                crate::comparacao::ramo("lesao para 3 e sai");
                *b(m, 0x8eb5) = 1;
                *b(p, 0x1d) = 1;
            }
        }
    }
    let x = *folego as f64;
    if !(x >= f64_em(0x967890)) {
        *folego = f64_em(0x967890) as f32;
    } else if x > f64_em(0x968910) {
        *folego = f64_em(0x968910) as f32;
    }
    if *b(m, 0x8eb5) != 0 {
        *w(0xf639) = (*w(0xf639)).wrapping_add(1);
    }
}


/// As regioes da lesao: o ponteiro da lista de lesoes, o cabecalho, os nos (o ultimo ganha o
/// proximo quando a original acrescenta) e os registros de 6 bytes.
pub(crate) unsafe fn regioes_das_lesoes() -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    let dono = *(G_INJURY as *const usize);
    let lista = if dono != 0 { *((dono + 0x28) as *const usize) } else { 0 };
    if lista != 0 {
        reg.extend([(dono + 0x28, 4), (lista, 0xe)]);
        let mut no = core::ptr::read_unaligned((lista + 2) as *const usize);
        for _ in 0..(*(lista as *const i16)).max(0) {
            if no == 0 {
                break;
            }
            reg.push((no, 0xa));
            let e = *(no as *const usize);
            if e != 0 {
                reg.push((e, 6));
            }
            no = core::ptr::read_unaligned((no + 6) as *const usize);
        }
    }
    reg
}


pub(crate) unsafe fn fita_da_lesao(m: usize, tipo: u32, chance: u32, p: usize, o: usize) {
    use std::sync::atomic::Ordering::Relaxed;
    let f = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize)>(313);
    let dono = *(G_INJURY as *const usize);
    if dono == 0 || *((dono + 0x28) as *const usize) == 0 {
        // sem lista, a original a cria (aloca): fica sem comparacao
        return f(m, tipo, chance, p, o);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    if o != 0 {
        reg.push((o + 0x31, 2));
    }
    reg.extend(regioes_das_lesoes());
    let gravar = GRAVAR_LESAO.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || f(m, tipo, chance, p, o), |chamar| {
        lesionar_com(m, tipo, chance, p, o, &mut |rotina, this, args| chamar(rotina, this, args, [0, u32::MAX, 0, 0, 0]))
    });
    GRAVAR_LESAO.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("lesionar(tipo {}, chance {}) {} chamadas: {}", tipo as u8 as i8, chance as u8 as i8, k, detalhe_fita()));
}

/// A 0x62be00 so e chamada pela 0x6d0980: a comparacao e a dela (a lista entra nas regioes).
pub(crate) unsafe extern "thiscall" fn comparar_registrar_lesao(this: usize, ficha: usize, tipo: *mut u8, chance: *mut u8) {
    orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(314)(this, ficha, tipo, chance)
}

/// can the player earn highlight `codigo` (0x1f40..0x21e4)? Not if the side already had it (table at engine +0x21e) or
/// the record is from outside; 0x219e..0x21b8 with rules on performance, goals, cards, time on the pitch and the
/// history search (0x757da0, last 7 or 30 days). Compared side by side
/// Original: `player_highlight` (0x717470, thiscall, ret 4).
///
/// [pt] { 0x717470 } (ret 4): o jogador pode ganhar o destaque `codigo` (0x1f40..=0x21e4) nesta partida?
/// Nao se o lado ja o teve (tabela de dwords em +0x21e do motor, 4000 bytes por lado) ou se a ficha
/// (+0x69) e de fora da base ([0xae2364]). Os codigos 0x219e..=0x21b8 tem regras proprias (atuacao
/// +0x35 contra um sorteio ou contra o tempo em campo, gols +0xc, cartoes, e a busca no historico
/// 0x757da0 nos ultimos 7 ou 30 dias, datas da 0x54c8e0); os demais da faixa, sim.
pub(crate) unsafe extern "thiscall" fn destaque_do_jogador(p: usize, codigo: u32) -> u32 {
    let m = *((p + 0x19e) as *const usize);
    let hoje = 0xae2c90usize;
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(FN_DATE_SUB_DAYS);
    let (mut d7, mut d30) = ([0u8; 8], [0u8; 8]);
    datar(hoje, &mut d7, 7);
    datar(hoje, &mut d30, 0x1e);
    let c = codigo as u16 as i16 as i32;
    if !(0x1f40..=0x21e4).contains(&c) {
        return 0;
    }
    let lado = byte(p, 0x27) as i32;
    if *((m as isize + ((c + lado * 1000) * 4) as isize - 0x7ae2) as *const u32) != 0 {
        return 0;
    }
    let ficha = *((p + 0x69) as *const usize);
    if ficha != 0 && *(ficha as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let atuacao = *((p + 0x35) as *const i16) as i32;
    let b = |o: usize| byte(p, o);
    let minuto = *((m + 0x8ed0) as *const i16) as i32 / 11;
    let em_campo = minuto - b(0x1f) as i32;
    let limite = |t: i32| (t * 33 + 0xfa0).min(0x1964);
    // as condicoes comuns: sem gol, sem cartao, +0x17 ate 8, +0x03 ate 2 e a atuacao ate o limite
    let comum = || b(0xc) == 0 && b(0x10) == 0 && b(0x17) <= 8 && (b(3) as u8) <= 2 && atuacao <= limite(em_campo);
    let buscar = |cod: u32, data: &[u8; 8]| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, *const [u8; 8]) -> u32 = core::mem::transmute(FN_MEDIA_HAS_LOGGED_STORY);
        f(*(G_MEDIA as *const usize), ficha, *((p + 0xf9) as *const usize), -1, cod, data) != 0
    };
    let algum = |de: u32, ate: u32, data: &[u8; 8]| (de..=ate).any(|k| buscar(k as u16 as u32, data));
    let r = match c {
        0x219e => comum() && (algum(0, 0x1d, &d7) || algum(0x1e, 0x27, &d7)),
        0x219f => atuacao >= sortear(0x7d0) + 0x1b58 && algum(0, 0x1d, &d7),
        0x21a0 => {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            if dias(hoje, *((ficha + 0x3e) as *const u32), *((ficha + 0x42) as *const u32)) < 0x5a || atuacao <= sortear(0x7d0) + 0x1b58 {
                false
            } else {
                let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MEDIA_HAS_RESPONSE_TWO_FOR_PLAYER);
                f(*(G_MEDIA as *const usize), ficha) != 0
            }
        }
        0x21a1 | 0x21a2 => {
            let gols = b(0xa) as u8 as i32;
            if p != *((m + 0xf5aa) as *const usize) || *((m + 0xf5a6) as *const u32) == 0 {
                false
            } else if c == 0x21a1 {
                if gols <= 4 || *((p + 0x39) as *const i16) <= 0xfa {
                    false
                } else {
                    let r = sortear(6) + 12;
                    em_campo / gols < r && buscar(0x52, &d7)
                }
            } else if gols >= 3 || gols == 0 {
                false
            } else {
                let r = sortear(0x19) + 0x32;
                minuto > r && em_campo / gols > 0x1e && buscar(0x52, &d7)
            }
        }
        0x21b0 => atuacao > sortear(1000) + 0x1f40 && buscar(0x34, &d7),
        0x21b1 => comum() && buscar(0x34, &d7),
        0x21b2 => comum() && buscar(0x33, &d30),
        0x21b3 => atuacao > sortear(1000) + 0x1f40 && buscar(0x33, &d30),
        0x21b4 => atuacao >= sortear(0x7d0) + 0x1b58 && algum(0x28, 0x3b, &d7),
        0x21b5 => atuacao > sortear(1000) + 0x1f40 && b(0x30) != 0,
        0x21b6 => {
            let sim = atuacao > sortear(0x7d0) + 0x1c52 && b(0xc) == 1 && em_campo > 0x41 && b(0x30) != 0;
            if sim {
                crate::comparacao::ramo("destaque marca 21b5");
                *((m + 0xbf2 + lado as usize * 4000) as *mut u32) = 1;
            }
            sim
        }
        0x21b7 => b(0x30) != 0 && b(0xc) == 0 && b(0x10) == 0 && b(0x17) <= 8 && (b(3) as u8) <= 2 && em_campo >= 0x37 && atuacao <= limite(em_campo),
        0x21b8 => {
            atuacao < 0x1964
                && (b(0x14) as u8 as i32) > sortear(3)
                && tabela_do_jogador(p) & 4 != 0
                && byte(*((p + 0x6d) as *const usize), 0x11) as i32 > sortear(4) + 0xf
                && b(0xc) == 0
                && (b(0x17) as i32) < sortear(6)
                && em_campo > sortear(0x32) + 0x1e
        }
        _ => true,
    };
    crate::comparacao::ramo(if r { "destaque sim" } else { "destaque nao" });
    r as u32
}

pub(crate) unsafe extern "thiscall" fn comparar_destaque_do_jogador(p: usize, codigo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(315);
    if gravando_no_topo() {
        return o(p, codigo);
    }
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(p, codigo), || crate::aleatorio::sem_contar(|| destaque_do_jogador(p, codigo)));
    PLACAR.registrar(igual && r == n, || format!("destaque_do_jogador({:#x}) original={} rust={} {:x?} x {:x?}", codigo as u16, r, n, mo, mr));
    r
}

// ---------- centesima oitava leva: as folhas das lesoes (0x62ec00, 0x62ece0) ----------

/// { 0x62ec00 } (ret 4, injury.cpp): o byte de gravidade do tipo de lesao `tipo` (tabela de 13 bytes
/// por tipo em 0x9e7ad9).
pub(crate) unsafe extern "thiscall" fn gravidade_da_lesao(_lesoes: usize, tipo: i8) -> u8 {
    *((0x9e7ad9 + tipo as isize as usize * 13) as *const u8)
}

/// { 0x62ece0 } (ret 4, injury.cpp): o grau da lesao da pessoa (registro de 31 bytes por id na
/// tabela das lesoes, +0x13, pelo menos 1), ou 0 sem pessoa, sem ficha (+0x61), id fora da tabela
/// (0xae2364) ou sem lesao (+0x12 negativo).
pub(crate) unsafe extern "thiscall" fn grau_da_lesao(lesoes: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) || *((pessoa + 0x61) as *const usize) == 0 {
        return 0;
    }
    let r = *(lesoes as *const usize) + (id as usize).wrapping_mul(31);
    if byte(r, 0x12) <= -1 {
        return 0;
    }
    let g = byte(r, 0x13);
    if g > 1 { g as u8 } else { 1 }
}

// ---------- 270a leva: a lesão pelo nome do player_setup.cfg; a notícia da volta aos treinos e o que ela acerta (o tratamento, as férias e a moral do jogador, a relação com o técnico, a fuga) e o nome da categoria da lesão (player_regen.cpp, injury.cpp) ----------

/// Os nomes das lesões do player_setup.cfg (a comparação, os caracteres comparados, o texto no .rdata e o tipo), na
/// ordem da original; o último, ARTHRITIS (8 caracteres, 0xa6b3a8), dá 0x4c, e nenhum, 0x4b.
pub(crate) const CQF9_LESOES: [(u32, usize, u8); 75] = [(4, 0xa6b840, 0), (3, 0xa6b83c, 1), (5, 0xa6b834, 2), (10, 0xa6b824, 3), (10, 0xa6b80c, 4), (10, 0xa6b7fc, 5), (10, 0xa6b7ec, 6), (10, 0xa6b7dc, 7), (10, 0xa6b7d0, 8), (10, 0xa6b7c0, 9), (10, 0xa6b7b4, 10), (15, 0xa6b7a0, 11), (15, 0xa6b788, 12), (10, 0xa6b778, 80), (10, 0xa6b768, 13), (10, 0xa6b758, 14), (10, 0xa6b74c, 15), (10, 0xa6b73c, 16), (10, 0xa6b730, 17), (10, 0xa6b71c, 18), (10, 0xa6b70c, 19), (10, 0xa6b6fc, 20), (10, 0xa6b6ec, 21), (10, 0xa6b6dc, 22), (10, 0xa6b6c8, 23), (10, 0xa6b6b4, 24), (10, 0xa6b6a0, 25), (10, 0xa6b690, 26), (10, 0xa6b678, 27), (10, 0xa6b664, 28), (10, 0xa6b654, 29), (10, 0xa6b644, 30), (10, 0xa6b634, 31), (10, 0xa6b628, 32), (10, 0xa6b618, 33), (10, 0xa6b608, 34), (10, 0xa6b5f8, 35), (10, 0xa6b5e4, 36), (5, 0xa6b5d8, 37), (10, 0xa6b5c8, 38), (10, 0xa6b5bc, 39), (10, 0xa6b5ac, 40), (10, 0xa6b5a0, 41), (14, 0xa6b590, 42), (10, 0xa6b584, 43), (10, 0xa6b574, 44), (10, 0xa6b564, 45), (10, 0xa6b554, 46), (5, 0xa6b548, 47), (10, 0xa6b538, 48), (10, 0xa6b52c, 49), (10, 0xa6b51c, 50), (10, 0xa6b50c, 52), (15, 0xa6b4fc, 53), (5, 0xa6b4f0, 54), (13, 0xa6b4e0, 55), (10, 0xa6b4d0, 56), (10, 0xa6b4c4, 57), (10, 0xa6b4b0, 58), (10, 0xa6b49c, 59), (10, 0xa6b48c, 60), (10, 0xa6b47c, 61), (10, 0xa6b46c, 62), (10, 0xa6b458, 63), (10, 0xa6b448, 64), (10, 0xa6b438, 65), (5, 0xa6b42c, 66), (10, 0xa6b420, 67), (5, 0xa6b414, 68), (5, 0xa6b408, 69), (13, 0xa6b3f8, 70), (10, 0xa6b3e8, 71), (19, 0xa6b3d4, 72), (10, 0xa6b3c0, 73), (10, 0xa6b3b4, 74)];

/// strncmp (0x946fa0, cdecl).
pub(crate) unsafe fn cqf9_strncmp(a: usize, b: usize, n: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_CRT_STRNCMP);
    f(a, b, n)
}






/// O ensaio do comparador: a cópia roda sem a entrega da notícia (0x786090, que aloca) e para antes da fuga (0x419840,
/// que põe na lista e manda notícia), guardando a decisão.
pub(crate) static mut CQF9_ENSAIO: bool = false;
pub(crate) static mut CQF9_FUGIU: bool = false;

/// O registro de forma do jogador na tabela das lesões ([this] + 31 × id).
pub(crate) unsafe fn cqf9_forma(this: usize, p: usize) -> usize {
    read_u32_as_usize(this).wrapping_add((*(p as *const i32) as isize * 31) as usize)
}

/// Gosta/não gosta do técnico (0x547810/0x547840) e tirar dos favoritos/rejeitados (0x547c10/0x547c50), cdecl.
pub(crate) unsafe fn cqf9_pref(rotina: usize, p: usize, tec: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(rotina);
    f(p, tec)
}

/// Acrescentar aos favoritos (0x547a10) ou aos rejeitados (0x547af0), cdecl(pessoa, outra, 1).
pub(crate) unsafe fn cqf9_acrescentar(rotina: usize, p: usize, tec: usize) {
    let f: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(rotina);
    f(p, tec, 1);
}

/// O técnico gostar mais: tira dos rejeitados e, com sorteio(+0x59) acima de 5, põe nos favoritos.
pub(crate) unsafe fn cqf9_gostar_mais(p: usize, clube: usize) {
    if read_u32_as_usize(clube + 0xcf) != 0 && cqf9_pref(0x547840, p, read_u32_as_usize(clube + 0xcf)) != 0 {
        cqf9_pref(0x547c50, p, read_u32_as_usize(clube + 0xcf));
    }
    if read_u32_as_usize(clube + 0xcf) != 0 && cpe1_sorteio(read_i8(p + 0x59) as i32) > 5 {
        cqf9_acrescentar(0x547a10, p, read_u32_as_usize(clube + 0xcf));
    }
}

/// the news of the player return to light or full training; with a type 12 category: the outcome of the treatment, the
/// return and the holidays (the contract grievances, the happiness, the liking for the manager, the chance of going
/// absent and the personality attribute changes)
/// Original: `Injury::post_recovery_news` (0x630ee0, injury.cpp, thiscall).
///
/// [pt] { 0x630ee0 } (thiscall, ret 0x14, injury.cpp, quadro SEH da notícia local): `Injury::post_recovery_news(jogador,
/// estágio, tipo, forçar, dias)`, pelas voltas aos treinos (0x62e010) e pelo fim da lesão (0x62e700). Sem jogador:
/// nada; sem forçar, só com clube de técnico humano (0x7875e0). A notícia 2001 (0x77bbe0) com o estágio, o tipo, o id,
/// os três nomes, o nascimento, o ano e os dias (0x785790, dados 0..8). Tipo de categoria 12 (a tabela de 13 bytes em
/// 0x9e7ad9): sorteio(32000) e: tipo 'Q' (férias): com contrato (0x4cf5a0), ficha e clube, cada queixa do
/// contrato (+0x45: 0x20000, 0x20, 0x40) cai (0x8674e0) com sorteio(+0x5b) > sorteio(25); sem nenhuma cair, sorteio(+0x5b)
/// 0 leva à chance de fuga (com o gerente das faltas [0xad9c6c] e +0x59 abaixo de 12, sorteio(+0x59) < sorteio(5): a
/// fuga 0x419840 com 4 e a notícia não sai); senão a felicidade do contrato (+0x44) sobe 15 (−100..100) e, com marca
/// caída, o técnico gosta mais (resultado 2); depois sorteio(+0x5b + +0x59, − 5 com valor +0xb da ficha acima de 7250) <
/// sorteio(20) e o sorteio(4) acerta +0x57, +0x5d, +0x44 da ficha e +0x59. Estágio 0, tipo 'M' (tratamento): 0 dá
/// resultado 2 (forma +0x12 = 0xff, +0x13 = 0); abaixo de 5, resultado 1 (+0x13 = 0); senão resultado 0 e, com +0x5d
/// abaixo de 5, o técnico pode perder a simpatia (0x547810/0x547c10, sorteio(+0x5d) 0: rejeitado 0x547af0); +0x59 (ou
/// +0x57) acima de 10: o técnico gosta mais. Estágio 1, tipos 'O' e 'N': o resultado pelo sorteio, a forma e a marca
/// +0xc do registro de 79 bytes ([0xae2d0c]) cai. Dados 9 (resultado) e 10; a entrega pelo alcance `forcar` (0x786090:
/// o clube, a nação do clube ou da pessoa e o continente dela).
pub(crate) unsafe extern "thiscall" fn noticia_da_volta_aos_treinos(this: usize, p: usize, estagio: u32, tipo: u32, forcar: u32, dias: u32) {
    if p == 0 {
        return;
    }
    if forcar as u8 == 0 {
        let clube = read_u32_as_usize(p + 0x39);
        if clube == 0 {
            return;
        }
        let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
        if humano(clube) == 0 {
            return;
        }
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let dado: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x7d1, 0);
    let est = estagio as u8 as i8;
    let tp = tipo as u8 as i8;
    dado(n, 0, est as i32 as u32);
    dado(n, 1, tp as i32 as u32);
    dado(n, 2, read_u32_as_usize(p) as u32);
    dado(n, 3, read_u32_as_usize(read_u32_as_usize(p + 4) + 0x33) as u32);
    dado(n, 4, read_u32_as_usize(read_u32_as_usize(p + 8) + 0x33) as u32);
    dado(n, 5, read_u32_as_usize(read_u32_as_usize(p + 0xc) + 0x33) as u32);
    dado(n, 6, *((p + 0x10) as *const i16) as i32 as u32);
    dado(n, 7, *((p + 0x12) as *const u16) as u32);
    dado(n, 8, dias as u16 as i16 as i32 as u32);
    let clube = read_u32_as_usize(p + 0x39);
    let mut res: i8 = -1;
    let mut f1: i8 = 0;
    if *((0x9e7ad9 + tp as isize * 13) as *const u8) == 0xc {
        let s = cpe1_sorteio(32000) as i16;
        if tp == 0x51 {
            res = 0;
            let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
            let c = contrato_de(read_u32_as_usize(G_CONTRACT_MANAGER), p);
            if c == 0 || read_u32_as_usize(p + 0x61) == 0 || clube == 0 {
                f1 = 1;
            } else {
                let tirar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
                let mut caiu = false;
                for marca in [0x20000u32, 0x20] {
                    if read_u32_as_usize(c + 0x45) as u32 & marca != 0 {
                        let a = cpe1_sorteio(read_i8(p + 0x5b) as i32);
                        let b = cpe1_sorteio(0x19);
                        if a > b {
                            tirar(c, marca);
                            caiu = true;
                        }
                    }
                }
                let mut feliz = caiu;
                if read_u32_as_usize(c + 0x45) as u32 & 0x40 != 0 {
                    let a = cpe1_sorteio(read_i8(p + 0x5b) as i32);
                    let b = cpe1_sorteio(0x19);
                    if a > b {
                        tirar(c, 0x40);
                        caiu = true;
                        feliz = true;
                    }
                }
                if !feliz && cpe1_sorteio(read_i8(p + 0x5b) as i32) != 0 {
                    feliz = true;
                }
                let mut seguir = true;
                if feliz {
                    let v = read_i8(c + 0x44) as i32 + 0xf; // cqf9: a felicidade sobe 15
                    crate::comparacao::ramo("cqf_9 felicidade pelas ferias");
                    *((c + 0x44) as *mut i8) = if v > 100 { 100 } else if v < -100 { -100 } else { v as i8 };
                }
                if feliz && caiu {
                    res = 2;
                    cqf9_gostar_mais(p, clube);
                } else {
                    if read_u32_as_usize(G_AWOL) != 0 && read_i8(p + 0x59) < 0xc {
                        let a = cpe1_sorteio(read_i8(p + 0x59) as i32);
                        let b = cpe1_sorteio(5);
                        if a < b {
                            if CQF9_ENSAIO {
                                CQF9_FUGIU = true;
                                return;
                            }
                            let fugir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_AWOL_START_ABSENCE);
                            fugir(read_u32_as_usize(G_AWOL), p, clube, 4);
                            return;
                        }
                    }
                    res = 0;
                    seguir = true;
                }
                if seguir {
                    let mut x = (read_i8(p + 0x5b) as i16).wrapping_add(read_i8(p + 0x59) as i16);
                    f1 = 1;
                    if *((read_u32_as_usize(p + 0x61) + 0xb) as *const i16) > 0x1c52 {
                        x = x.wrapping_sub(5);
                    }
                    let a = cpe1_sorteio(x as i32);
                    let b = cpe1_sorteio(0x14);
                    if a < b {
                        let r = cpe1_sorteio(4);
                        let f = read_u32_as_usize(p + 0x61);
                        let subir_57 = |p: usize| {
                            if read_i8(p + 0x57) < 0x14 {
                                *((p + 0x57) as *mut i8) += 1;
                            }
                        };
                        let descer_5d_44 = |p: usize, f: usize| {
                            if read_i8(p + 0x5d) > 1 {
                                *((p + 0x5d) as *mut i8) -= 1;
                            }
                            if read_i8(f + 0x44) > 1 {
                                *((f + 0x44) as *mut i8) -= 1;
                            }
                        };
                        crate::comparacao::ramo("cqf_9 acerto pelas ferias");
                        if r == 0 {
                            subir_57(p); // cqf9: o sorteio 0 sobe +0x57
                            descer_5d_44(p, f);
                        } else if r == 1 {
                            subir_57(p);
                        } else {
                            descer_5d_44(p, f);
                        }
                        if r == 0 || r == 1 {
                            if read_i8(p + 0x59) > 1 {
                                *((p + 0x59) as *mut i8) -= 1;
                            }
                        }
                    }
                }
            }
        } else if est == 0 {
            if tp != 0x4d {
                res = -1;
            } else {
                let forma = cqf9_forma(this, p);
                let mut v: Option<i8> = None;
                crate::comparacao::ramo("cqf_9 tratamento");
                if s == 0 { // cqf9: o tratamento dá certo
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                    v = Some(read_i8(p + 0x59));
                } else if s < 5 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                    if read_i8(p + 0x59) > 0xf {
                        v = Some(read_i8(p + 0x57));
                    }
                } else {
                    res = 0;
                    if read_i8(p + 0x5d) < 5 {
                        f1 = -1;
                        if clube != 0 {
                            if read_u32_as_usize(clube + 0xcf) != 0 && cqf9_pref(0x547810, p, read_u32_as_usize(clube + 0xcf)) != 0 {
                                cqf9_pref(0x547c10, p, read_u32_as_usize(clube + 0xcf));
                            }
                            if read_u32_as_usize(clube + 0xcf) != 0 && cpe1_sorteio(read_i8(p + 0x5d) as i32) == 0 {
                                cqf9_acrescentar(0x547af0, p, read_u32_as_usize(clube + 0xcf));
                            }
                        }
                    }
                }
                if let Some(v) = v {
                    if v > 10 {
                        f1 = 1;
                        if clube != 0 {
                            cqf9_gostar_mais(p, clube);
                        }
                    }
                }
            }
        } else {
            let forma = cqf9_forma(this, p);
            crate::comparacao::ramo("cqf_9 volta da lesao");
            if tp == 0x4f {
                if s < 30000 { // cqf9: a volta 'O' completa
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                } else if cpe1_sorteio(5) != 0 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                } else {
                    res = 3;
                    let a = forma + 0x13;
                    let r = cpe1_sorteio(4);
                    *(a as *mut u8) = (*(a as *const u8)).wrapping_add((r as u8).wrapping_add(1));
                }
            } else if tp == 0x4e {
                if s < 0x733c { // cqf9: a volta 'N' completa
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                } else if s < 30000 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                } else {
                    res = 0;
                }
            }
            let e = read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((*(p as *const i32) as isize * 0x4f + 0xc) as usize);
            *(e as *mut u8) &= 0xfe; // cqf9: a marca +0xc cai
        }
    }
    dado(n, 9, res as i32 as u32);
    dado(n, 10, f1 as i32 as u32);
    if CQF9_ENSAIO {
        return;
    }
    let nacao = if clube != 0 { read_u32_as_usize(clube + 0x53) } else { read_u32_as_usize(p + 0x1a) };
    let continente = if nacao != 0 { read_u32_as_usize(nacao + 0x71) } else { 0 };
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    entregar(read_u32_as_usize(G_NEWS), n, clube, 0, nacao, continente, forcar);
}

/// Comparador em ensaio (modelo CPV6_ENSAIO): a cópia roda primeiro sem a entrega da notícia e parando antes da fuga,
/// depois a original a partir do mesmo estado; compara o registro do jogador (0x6e), a ficha (0x46), o contrato (0x4f),
/// o registro de forma (31), o byte +0xc do registro de 79 bytes, o registro do técnico do clube e o estado do sorteio.
/// Com a fuga na cópia (a original põe na lista, sorteia e manda notícia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_volta_aos_treinos(this: usize, p: usize, estagio: u32, tipo: u32, forcar: u32, dias: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1619);
    if p == 0 {
        return o(this, p, estagio, tipo, forcar, dias);
    }
    let mut reg: Vec<(usize, usize)> = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.push((p, 0x6e));
    if read_u32_as_usize(p + 0x61) != 0 {
        reg.push((read_u32_as_usize(p + 0x61), 0x46));
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let antes_1080 = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    let c = contrato_de(read_u32_as_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = antes_1080;
    if c != 0 {
        reg.push((c, 0x4f));
    }
    reg.push((0xae1080, 4));
    reg.push((cqf9_forma(this, p), 31));
    reg.push((read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((*(p as *const i32) as isize * 0x4f) as usize), 0x4f));
    let clube = read_u32_as_usize(p + 0x39);
    if clube != 0 && read_u32_as_usize(clube + 0xcf) != 0 {
        reg.push((read_u32_as_usize(clube + 0xcf), 0x6e));
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQF9_ENSAIO = true;
    CQF9_FUGIU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| noticia_da_volta_aos_treinos(this, p, estagio, tipo, forcar, dias)));
    CQF9_ENSAIO = false;
    let fugiu = CQF9_FUGIU;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, p, estagio, tipo, forcar, dias);
    if fugiu {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("noticia_da_volta_aos_treinos(pessoa {:x}, estagio {}, tipo {:x}) {:x?} x {:x?}", p, estagio as u8, tipo as u8, mo.0, mr.0));
}

/// the translated injury category name (internal, foot, shin, knee, thigh, groin, upper body, hand, arm, shoulder,
/// neck, head, rehabilitation); returns the name gender in language 3
/// Original: `injury_get_category_name` (0x630c50, injury.cpp, stdcall).
///
/// [pt] { 0x630c50 } (stdcall, ret 0xc, injury.cpp): `injury_get_category_name(categoria, texto, tamanho)`: o nome traduzido
/// (0x66f4e0) da categoria da lesão 0..12 (interna, pé, canela, joelho, coxa, virilha, tronco, mão, braço, ombro,
/// pescoço, cabeça, recuperação); fora disso a caixa (linha 0x10e7). Devolve 2 com o idioma 3 (0x66f4d0), senão 0 (o
/// gênero do nome). Tamanho abaixo de 20: a caixa (linha 0x10d0) e o al zerado.
pub(crate) unsafe extern "stdcall" fn nome_da_categoria_da_lesao(categoria: u32, texto: usize, tamanho: i32) -> u32 {
    if tamanho < 0x14 {
        aviso_em(0x9e7f28, 0x10d0);
        return 0;
    }
    const NOMES: [usize; 13] = [0x9e89ac, 0x9e89a0, 0x9e8994, 0x9e8988, 0x9e8978, 0x9e8968, 0x9e8954, 0x9e8948, 0x9e893c, 0x9e892c, 0x9e8920, 0x9e8914, 0x9e8904];
    let k = categoria as u8 as i8 as i32 as u32;
    if k <= 0xc {
        let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        traduzir(texto, NOMES[k as usize]);
    } else {
        aviso_em(0x9e7f28, 0x10e7);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    if idioma() == 3 { 2 } else { 0 }
}

/// Comparador: lado a lado no texto (o tamanho, até 0x100) e na devolução; o tamanho curto (a caixa): só a original.
pub(crate) unsafe extern "stdcall" fn comparar_nome_da_categoria_da_lesao(categoria: u32, texto: usize, tamanho: i32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(u32, usize, i32) -> u32>(1620);
    if tamanho < 0x14 || texto == 0 {
        return o(categoria, texto, tamanho);
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(texto, (tamanho as usize).min(0x100))], || o(categoria, texto, tamanho), || nome_da_categoria_da_lesao(categoria, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("nome_da_categoria_da_lesao({}) original={} rust={}", categoria as u8 as i8, r, n));
    r
}

// ---------- 264a leva: a lesão nova (o registro, a duração pelo fisioterapeuta, a recaída e a aposentadoria), a notícia da lesão e a ausência sem licença (injury.cpp, awol.cpp) ----------







/// O registro físico de 31 bytes da pessoa `id` em [this].
pub(crate) unsafe fn cqb5_registro(this: usize, id: usize) -> usize {
    read_usize(this).wrapping_add(id.wrapping_mul(0x1f))
}

/// A lesão `i` (0x30 bytes) do vetor [this+4].
pub(crate) unsafe fn cqb5_lesao(this: usize, i: usize) -> usize {
    read_usize(this + 4).wrapping_add(i.wrapping_mul(0x30))
}

/// O contrato da pessoa (0x4cf5a0: thiscall([0xae1050], pessoa), ret 4), guardado em [0xae1080].
pub(crate) unsafe fn cqb5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato).
pub(crate) unsafe fn cqb5_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(0x245)) }
}

/// A tabela dos tipos de lesão (13 bytes por tipo): +0 (0x9e7adb) dias de base (i16), +2 (0x9e7add) dias variáveis
/// (i16), +4 (0x9e7adf) percentual da volta ao treino (i8), +5 (0x9e7ae0) pode voltar (i32).
pub(crate) fn cqb5_tipo(tipo: i8) -> usize {
    (tipo as i32 * 13) as usize
}

/// Os dias da volta ao treino (+0x2a) pela duração `dias`: ftol(percentual × 0,01 × dias) (fild, fmul qword
/// [0x967898], fimul, _ftol 0x945640; x87 em dupla precisão), no máximo 60 dias antes do fim.
pub(crate) unsafe fn cqb5_volta_ao_treino(tipo: i8, dias: i16) -> i16 {
    let pct = read_i8_as_i32(0x9e7adf + cqb5_tipo(tipo));
    let x = ftol(pct as f64 * f64_em(0x967898) * dias as f64) as i16;
    if (x as i32) + 0x3c < dias as i32 { dias.wrapping_sub(0x3c) } else { x }
}

/// records the injury: replaces the previous one only for types 0x4d/0x4e/0x4f/0x51, finds or grows the slot, records
/// the match, the length (base + variable × (35 − best physio) / 20 with draws and the age and proneness extras), the
/// return to training (type percentage, at most 60 days before the end), the date, the news, the recurrence, the
/// national team, the relapse, the retirement and the outbreak
/// Original: `Injury::add_injury` (0x62d250, injury.cpp, thiscall, ret 0x20).
///
/// [pt] { 0x62d250 } (thiscall, ret 0x20, injury.cpp, quadro de 0x220 bytes sem SEH): `Injury::add_injury(tipo, retroativa,
/// pessoa, partida, surto, arg da notícia, recorrência, só_recorrência)`, pelo dia das lesões (0x62b7b0), pelos efeitos
/// físicos da partida (0x62c080), pelos registros iniciais (0x62cf70), pela recaída (0x62e500), pelos surtos de doença
/// (0x62e9e0) e por duas originais. Pessoa nula, sem ficha (+0x61) ou tipo fora de 0..0x51: a original (caixa e −1); id
/// além de [0xae2364]: −1. O físico do dia (0x62c680) e o treino ([0xde1f44], 0x8b1750 com 0). Só recorrência: com
/// recorrência e o tipo que pode voltar, a marca de recaída do registro (+0x12 tipo, +0x13: 1 se sorteio(rec / 10) e
/// sorteio(rec) derem zero com rec negativo; rec se positivo); −1. Senão, com lesão (+0xe ≠ −1): só os tipos 0x4d, 0x4e,
/// 0x4f e 0x51 a trocam (os dias restantes +0x2c ficam, a velha vai ao histórico 0x58e340 e a vaga fica livre). +0x1d
/// = tipo; a primeira vaga livre (byte 0xff) do vetor [this+4] (conta [this+8]) ou mais 100 vagas (realloc; sem
/// memória, a caixa da linha 0x909 e −1). O tipo na vaga; a partida (pelo clube da casa +0x1c ou de fora +0x20 que tem
/// a pessoa nos 50 do elenco +0xd7) grava o registro da partida em +1 (0x7e4750), senão +0x1a = 0xff. O melhor
/// fisioterapeuta do clube (0x544410 dos três +0x1cf com +0x69). Dias = base + variáveis × (35 − melhor) / 20 (menos
/// sorteio deles se sorteio(melhor + 5) > sorteio(20); 0 no tipo 0x51) + os restantes da velha (+ sorteios pela idade
/// e pela propensão +0x2d da ficha, com base acima de 10). O +0xa do registro de 79 bytes sobe 1 a cada sorteio(5 ×
/// natural + 12) zero, por dia, até 100. Lesão de agora (retroativa 0): de 100 dias, fora do clube humano e fora da
/// lista, a marca 100 do clube na lista de observação; dias ≥ sorteio(50) + 30, os clubes que miram o jogador o
/// suspendem (0x842b00). +0x28 = +0x2c = dias, +0x2a = +0x2e = a volta ao treino; +0x20 = a data (hoje, ou hoje −
/// sorteio(dias) − 1 na retroativa). O físico de novo, a notícia (0x62dca0), +0xe = a vaga, a recorrência, a seleção
/// (0x775cd0), +0x1c = 0, a recaída (0x62e500), a contagem das recaídas (de duas: aposentadoria 0x7a7f20 por sorteio),
/// a aposentadoria pela idade e pela volta ao treino longa e o surto (0x62e820) nos tipos 0, 1 e 2. Devolve a vaga.
pub(crate) unsafe extern "thiscall" fn lesao_nova(this: usize, tipo_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let tipo = tipo_arg as u8 as i8;
    let original = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(1470);
    if p == 0 {
        return original(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia); // a caixa: só a original
    }
    if read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return -1;
    }
    if read_usize(p + 0x61) == 0 || !(0..0x52).contains(&tipo) {
        return original(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia); // as caixas: só a original
    }
    let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
    fisico(this, p);
    if read_usize(G_TRAINING_MANAGER) != 0 {
        let treino: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_PLAYER_TRAINING);
        treino(read_usize(G_TRAINING_MANAGER), p, 0);
    }
    let t = cqb5_tipo(tipo);
    let rec = rec_arg as u8 as i8;
    if so_recorrencia != 0 {
        if rec == 0 || read_usize(G_INJURY_TYPE_CAN_RECUR + t) == 0 {
            return -1;
        }
        if rec < 0 {
            if cpe1_sorteio(rec as i32 / 10) == 0 {
                let r = cqb5_registro(this, read_usize(p));
                *((r + 0x12) as *mut u8) = tipo as u8;
                *((r + 0x13) as *mut u8) = 1;
            }
            return -1;
        }
        let r = cqb5_registro(this, read_usize(p));
        *((r + 0x12) as *mut u8) = tipo as u8;
        *((r + 0x13) as *mut u8) = rec as u8;
        return -1;
    }
    let antiga = read_usize(cqb5_registro(this, read_usize(p)) + 0xe) as i32;
    let mut restantes: u16 = 0; // a palavra baixa do local (a alta é 0)
    if antiga != -1 {
        if !(tipo == 0x4d || tipo == 0x4e || tipo == 0x4f || tipo == 0x51) {
            return -1;
        }
        crate::comparacao::ramo("cqb_5 troca a lesao");
        let r = cqb5_registro(this, read_usize(p));
        let l = cqb5_lesao(this, antiga as usize);
        restantes = read_i16(l + 0x2c) as u16;
        let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
        historico(read_usize(this + 0xc), r + 0x14, r + 0x18, l);
        *(l as *mut u8) = 0xff;
        core::ptr::write_unaligned((cqb5_registro(this, read_usize(p)) + 0xe) as *mut i32, -1);
    }
    *((cqb5_registro(this, read_usize(p)) + 0x1d) as *mut u8) = tipo as u8;
    // a vaga
    let mut vaga: i32 = -1;
    let vetor = read_usize(this + 4);
    if vetor != 0 {
        let n = read_usize(this + 8) as i32;
        let mut i = 0;
        while i < n {
            if read_u8(vetor + i as usize * 0x30) == 0xff {
                vaga = i;
                break;
            }
            i += 1;
        }
    }
    if vaga == -1 {
        let novo = (read_usize(this + 8) as i32).wrapping_add(100);
        let tam = (novo as usize).wrapping_mul(0x30);
        let v = if vetor != 0 {
            let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(vetor, tam)
        } else {
            let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(tam)
        };
        *((this + 4) as *mut usize) = v;
        if v == 0 {
            aviso_em(0x9e7f28, 0x909); // injury.cpp
            return -1;
        }
        let velho = read_usize(this + 8) as i32;
        let mut i = velho;
        while i < novo {
            *((read_usize(this + 4) + i as usize * 0x30) as *mut u8) = 0xff;
            i += 1;
        }
        *((this + 8) as *mut i32) = novo;
        vaga = velho;
    }
    let off = vaga as usize * 0x30;
    *((read_usize(this + 4) + off) as *mut u8) = tipo as u8;
    let mut da_partida = false;
    if jogo != 0 {
        for k in 0..50usize {
            let mut clube = read_usize(jogo + 0x1c);
            let achou = if read_usize(clube + 0xd7 + 4 * k) == p {
                true
            } else if read_usize(read_usize(jogo + 0x20) + 0xd7 + 4 * k) == p {
                clube = read_usize(jogo + 0x20);
                true
            } else {
                false
            };
            if achou {
                if clube != 0 {
                    let registrar: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_FILL_MATCH_RECORD_FROM_FIXTURE);
                    registrar(jogo, clube, read_usize(this + 4) + off + 1);
                    da_partida = true;
                }
                break;
            }
        }
    }
    if !da_partida {
        *((read_usize(this + 4) + off + 0x1a) as *mut u8) = 0xff;
    }
    // o melhor fisioterapeuta
    let mut melhor: i8 = 0;
    if read_usize(p + 0x39) != 0 {
        let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
        for k in 0..3usize {
            let f = read_usize(read_usize(p + 0x39) + 0x1cf + 4 * k);
            if f == 0 {
                continue;
            }
            let np = read_usize(f + 0x69);
            if np == 0 {
                continue;
            }
            if conhecer(np, read_u8(np + 0x1e) as i8) as i8 > melhor {
                let np = read_usize(read_usize(read_usize(p + 0x39) + 0x1cf + 4 * k) + 0x69);
                melhor = conhecer(np, read_u8(np + 0x1e) as i8) as i8;
            }
        }
    }
    // os dias
    crate::comparacao::ramo("cqb_5 duracao");
    let mut extra = (read_i16(G_INJURY_TYPE_VARIABLE_DAYS + t) as i32).wrapping_mul(0x23 - melhor as i32) / 20; // cqb5: o fisioterapeuta encurta
    let s1 = cpe1_sorteio(melhor as i32 + 5);
    let s2 = cpe1_sorteio(0x14);
    if s1 > s2 {
        extra = extra.wrapping_sub(cpe1_sorteio(extra as i16 as i32));
    }
    if tipo == 0x51 {
        extra = 0;
    }
    extra = extra.wrapping_add(restantes as i32);
    let idade = read_u8(p + 0x18) as i8;
    if tipo != 0x51 && read_i16(G_INJURY_TYPE_BASE_DAYS + t) > 10 {
        if idade > 0x1e {
            let ax = (read_i8_as_i32(read_usize(p + 0x61) + 0x2d) as i16).wrapping_add(idade as i16).wrapping_sub(0x23);
            if ax > 0 {
                extra = extra.wrapping_add(cpe1_sorteio(ax as i32));
            }
        }
        let al = read_u8(read_usize(p + 0x61) + 0x2d) as i8;
        if al > 10 {
            extra = extra.wrapping_add(cpe1_sorteio((al as i16 - 10) as i32));
        }
    }
    let dias = read_i16(G_INJURY_TYPE_BASE_DAYS + t).wrapping_add(extra as i16);
    let n = dias as i32;
    let mut i = 0;
    while i < n {
        let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 0xa;
        if read_u8(g) >= 100 {
            break;
        }
        if cpe1_sorteio(read_i8_as_i32(read_usize(p + 0x61) + 0x34) * 5 + 0xc) == 0 {
            let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 0xa;
            *(g as *mut u8) = read_u8(g).wrapping_add(1);
        }
        i += 1;
    }
    if retroativa == 0 {
        let clube = read_usize(p + 0x39);
        if clube != 0 {
            let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if humano(clube, 0, 1, 0, 0) == 0 && dias >= 100 {
                let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
                if listada(p, 0) == 0 && read_usize(G_TRANSFER_MANAGER) != 0 {
                    let tm = read_usize(G_TRANSFER_MANAGER);
                    let m = read_usize(read_usize(tm + 0x856) + 0xdc).wrapping_add(read_usize(read_usize(p + 0x39)).wrapping_mul(9)) + 8;
                    *(m as *mut u8) = 100;
                }
            }
        }
        let s = cpe1_sorteio(0x32);
        if n >= s + 0x1e && read_usize(G_TRANSFER_MANAGER) != 0 && read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856) != 0 {
            crate::comparacao::ramo("cqb_5 suspende as miras");
            let suspender: unsafe extern "thiscall" fn(usize, usize, u8) = core::mem::transmute(FN_SHORTLIST_MANAGER_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
            suspender(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), p, 0); // cqb5: os clubes que o miram suspendem
        }
    }
    write_i16(read_usize(this + 4) + off + 0x28, dias);
    write_i16(read_usize(this + 4) + off + 0x2c, dias);
    crate::comparacao::ramo("cqb_5 volta ao treino");
    let volta = cqb5_volta_ao_treino(tipo, dias); // cqb5: a volta ao treino pelo percentual
    write_i16(read_usize(this + 4) + off + 0x2a, volta);
    write_i16(read_usize(this + 4) + off + 0x2e, read_i16(read_usize(this + 4) + off + 0x2a));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut saida = [0u8; 8];
    if retroativa != 0 {
        let s = cpe1_sorteio(n);
        let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
        let mut d = [0u8; 8];
        let r = menos(0xae2c90, d.as_mut_ptr(), (s + 1) as u32);
        copiar(read_usize(this + 4) + off + 0x20, saida.as_mut_ptr(), r);
    } else {
        copiar(read_usize(this + 4) + off + 0x20, saida.as_mut_ptr(), 0xae2c90);
    }
    fisico(this, p);
    let noticia_fn: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_INJURY_POST_INJURY_NEWS);
    noticia_fn(this, p, read_usize(this + 4) + off, noticia);
    core::ptr::write_unaligned((cqb5_registro(this, read_usize(p)) + 0xe) as *mut i32, vaga);
    if rec != 0 && read_usize(G_INJURY_TYPE_CAN_RECUR + t) != 0 {
        if rec < 0 {
            if cpe1_sorteio(rec as i32 / 10) == 0 {
                *((cqb5_registro(this, read_usize(p)) + 0x12) as *mut u8) = tipo as u8;
                if cpe1_sorteio(rec as i32) == 0 {
                    *((cqb5_registro(this, read_usize(p)) + 0x13) as *mut u8) = 1;
                }
            }
        } else {
            *((cqb5_registro(this, read_usize(p)) + 0x12) as *mut u8) = tipo as u8;
            *((cqb5_registro(this, read_usize(p)) + 0x13) as *mut u8) = rec as u8;
        }
    }
    if read_usize(p + 0x24) != 0 && read_usize(G_NATIONAL_TEAMS) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_NATIONALTEAMS_WITHDRAW_INJURED_PLAYER);
        tirar(read_usize(G_NATIONAL_TEAMS), p, read_i16(read_usize(this + 4) + off + 0x28) as u16 as u32);
    }
    *((cqb5_registro(this, read_usize(p)) + 0x1c) as *mut u8) = 0;
    let recaida: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_INJURY_ROLL_RECURRING_INJURY);
    recaida(this, cqb5_registro(this, read_usize(p)), read_usize(this + 4) + off);
    let aposentar: unsafe extern "thiscall" fn(usize, usize, i16) = core::mem::transmute(FN_PLAYER_REGEN_SCHEDULE_INJURY_RETIREMENT);
    let r = cqb5_registro(this, read_usize(p));
    if read_u8(r + 0x12) as i8 == tipo && rec >= 0 {
        *((r + 0x13) as *mut u8) = read_u8(r + 0x13).wrapping_add(1);
        let c = read_u8(cqb5_registro(this, read_usize(p)) + 0x13) as i8;
        if c > 1 {
            let mut e = 200 - read_i16(G_INJURY_TYPE_BASE_DAYS + t) as i32;
            if e <= 5 {
                e = 5;
            }
            if rec > 0 {
                e /= rec as i32;
            }
            let s1 = cpe1_sorteio(c as i32);
            let s2 = cpe1_sorteio(e);
            if s1 > s2 && read_usize(G_PLAYER_REGEN) != 0 {
                crate::comparacao::ramo("cqb_5 aposenta pelas recaidas");
                aposentar(read_usize(G_PLAYER_REGEN), p, read_i16(read_usize(this + 4) + off + 0x2a));
            }
        }
    }
    // a aposentadoria pela idade e pela volta ao treino longa
    let volta = || read_i16(read_usize(this + 4) + off + 0x2a) as i32;
    let propensao = || read_i8_as_i32(read_usize(p + 0x61) + 0x2d);
    let anos = || read_i8_as_i32(p + 0x18);
    let encerra = (idade > 0x24 && volta() > 0x78 - propensao() - anos() && cpe1_sorteio(volta()) > 0x5a - anos())
        || (idade > 0x21 && volta() > 0x96 - propensao() - anos() && cpe1_sorteio(volta()) > 0x5a - anos())
        || (idade > 0x1f && volta() > 0x12c - anos() && cpe1_sorteio(volta()) > 0xc8 - anos())
        || (volta() > 0x1f4 && cpe1_sorteio(read_i8_as_i32(read_usize(p + 0x61) + 0x34) + 5) == 0);
    if encerra && read_usize(G_PLAYER_REGEN) != 0 {
        crate::comparacao::ramo("cqb_5 aposenta pela lesao");
        aposentar(read_usize(G_PLAYER_REGEN), p, read_i16(read_usize(this + 4) + off + 0x2a));
    }
    if surto != 0 && (tipo == 1 || tipo == 0 || tipo == 2) {
        let clube = read_usize(p + 0x39);
        if clube != 0 {
            let surto_fn: unsafe extern "thiscall" fn(usize, usize, u32, i32, usize) = core::mem::transmute(FN_INJURY_ADD_ILLNESS_OUTBREAK);
            surto_fn(this, 0xae2c90, tipo_arg, read_usize(clube) as i32, 0);
        }
    }
    vaga
}

/// Comparador da 0x62d250: só a original (aloca, entrega a notícia, agenda aposentadoria e chama originais que sorteiam
/// no meio, como o treino 0x8b1750 e a recaída 0x62e500, que pode lesionar de novo: não pode rodar duas vezes). Confere
/// o que a lesão garante depois: argumentos inválidos ou só recorrência dão −1; −1 de resto só com a pessoa ainda
/// lesionada (+0xe ≠ −1) ou sem vetor; vaga válida com a pessoa ainda nela (+0xe = vaga, sem recaída que a tenha
/// trocado): o tipo na vaga e em +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e = +0x2a e +0x2a = a volta ao treino pela
/// duração +0x28 (o x87 da cópia).
pub(crate) unsafe extern "thiscall" fn comparar_lesao_nova(this: usize, tipo_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(1470);
    let r = o(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia);
    let tipo = tipo_arg as u8 as i8;
    let valida = p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && read_usize(p + 0x61) != 0 && (0..0x52).contains(&tipo);
    let ok = if !valida || so_recorrencia != 0 {
        r == -1
    } else {
        let reg = cqb5_registro(this, read_usize(p));
        let atual = read_usize(reg + 0xe) as i32;
        if r == -1 {
            atual != -1 || read_usize(this + 4) == 0
        } else if r < 0 || atual != r {
            r >= 0 // trocada depois (a recaída lesionou de novo): nada a conferir
        } else {
            let l = cqb5_lesao(this, r as usize);
            // a recaída (0x62e500) pode lesionar de novo a própria pessoa com 0x4d/0x4e/0x4f: a troca libera esta vaga e
            // a nova lesão a reocupa (a primeira livre), com o tipo dela; confere a nova
            let t = read_u8(l) as i8;
            let tipo = if t != tipo && (0x4d..=0x4f).contains(&t) { t } else { tipo };
            read_u8(l) as i8 == tipo
                && read_u8(reg + 0x1d) as i8 == tipo
                && read_u8(reg + 0x1c) == 0
                && read_i16(l + 0x2c) == read_i16(l + 0x28)
                && read_i16(l + 0x2e) == read_i16(l + 0x2a)
                && read_i16(l + 0x2a) == cqb5_volta_ao_treino(tipo, read_i16(l + 0x28))
        }
    };
    PLACAR.registrar(ok, || format!("lesao_nova(tipo {:#x}, retroativa {}, pessoa {:#x}, recorrencia {}, so {}) = {}", tipo_arg, retroativa, p, rec_arg as u8 as i8, so_recorrencia, r));
    r
}

/// injury news 2000: the person, the type, the date, the days, the birth date and the match; over 100 days, to everyone
/// with potential above 8000 or to the interested clubs; up to 100, to the human club, the contract club and the
/// national team
/// Original: `injury_post_injury_news` (0x62dca0, injury.cpp, thiscall (this sem uso), ret 0xc).
///
/// [pt] { 0x62dca0 } (o mapa a dá como stdcall: a original troca o ecx por [0xde1f54] logo na entrada; o único ponto de
/// chamada, na 0x62d250, põe a Injury em ecx: thiscall com o this sem uso; ret 0xc; quadro SEH sem destrutor):
/// `injury_post_injury_news(pessoa, lesão, arg)`. Os clubes interessados (0x8d1da0 em [0xde1f54] com 0x5a, até 30) e a
/// seleção (+0x24) se couber; a notícia 0x7d0 (local): 0..3 a pessoa (id e os +0x33 de +4/+8/+0xc, 0 se faltam), 4 =
/// o tipo, 5/6 = a data da lesão (+0x20 com sinal, +0x22 sem), 7 = os dias (+0x28), 8/9 = o nascimento (+0x10, +0x12),
/// 0xa..0xc = o registro da partida (+0x12, +0x16, +9) ou −1 sem partida (+0x1a = 0xff), 0xd = arg, 0xe = id do clube
/// ou −1, 0xf = 0x420e9; a partida na notícia (0x7e4de0 com o byte 0x10). Mais de 100 dias: com potencial (+0xd da
/// ficha) acima de 8000, a todos (0x786780); senão aos interessados (0x7861e0). Até 100: clube humano (0x7875e0), ao
/// clube; o clube do contrato (0x4cf5a0) se for outro; a seleção.
pub(crate) unsafe extern "thiscall" fn noticia_da_lesao(_this: usize, p: usize, l: usize, arg: u32) {
    crate::comparacao::ramo("cqb_5 noticia da lesao");
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut clubes = [0usize; 30];
    let mut n: i8 = 0;
    if tm != 0 {
        let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
        n = interessados(tm, p, 0x5a, clubes.as_mut_ptr(), 0x1e) as i8;
    }
    let sel = read_usize(p + 0x24);
    if sel != 0 && (0..0x1e).contains(&n) {
        clubes[n as usize] = sel;
        n += 1;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    construir(nw, 0x7d0, 0);
    gravar(nw, 0, read_usize(p) as u32);
    for k in 1..4usize {
        let q = read_usize(p + 4 * k);
        gravar(nw, k as i8, if q != 0 { read_usize(q + 0x33) as u32 } else { 0 });
    }
    gravar(nw, 4, read_i8_as_i32(l) as u32);
    gravar(nw, 5, read_i16(l + 0x20) as i32 as u32);
    gravar(nw, 6, read_i16(l + 0x22) as u16 as u32);
    gravar(nw, 7, read_i16(l + 0x28) as i32 as u32);
    gravar(nw, 8, read_i16(p + 0x10) as i32 as u32);
    gravar(nw, 9, read_i16(p + 0x12) as u16 as u32);
    if read_u8(l + 0x1a) != 0xff {
        gravar(nw, 0xa, read_usize(l + 0x12) as u32);
        gravar(nw, 0xb, read_usize(l + 0x16) as u32);
        gravar(nw, 0xc, read_usize(l + 9) as u32);
    } else {
        gravar(nw, 0xa, u32::MAX);
        gravar(nw, 0xb, u32::MAX);
        gravar(nw, 0xc, u32::MAX);
    }
    gravar(nw, 0xd, arg);
    let clube = read_usize(p + 0x39);
    gravar(nw, 0xe, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX });
    gravar(nw, 0xf, 0x420e9);
    let mut marca: u8 = 0x10;
    let partida_na_noticia: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(FN_WRITE_MATCH_RECORD_TO_NEWS);
    partida_na_noticia(l + 1, nw, &mut marca);
    let news = read_usize(G_NEWS);
    if read_i16(l + 0x28) > 100 {
        let f = read_usize(p + 0x61);
        if f != 0 && read_i16(f + 0xd) > 0x1f40 {
            let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
            todos(read_usize(G_NEWS), nw);
        } else {
            let mut i: i8 = 0;
            while i < n {
                entregar(read_usize(G_NEWS), nw, clubes[i as usize]);
                i += 1;
            }
        }
        return;
    }
    if read_usize(p + 0x39) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if humano(news, read_usize(p + 0x39)) == 0 {
        return;
    }
    entregar(read_usize(G_NEWS), nw, read_usize(p + 0x39));
    let c = cqb5_contrato(p);
    if c != 0 && cqb5_clube_do_contrato(c) != 0 {
        let x = cqb5_clube_do_contrato(cqb5_contrato(p));
        if x != read_usize(p + 0x39) {
            let y = cqb5_clube_do_contrato(cqb5_contrato(p));
            entregar(read_usize(G_NEWS), nw, y);
        }
    }
    let sel = read_usize(p + 0x24);
    if sel != 0 {
        entregar(read_usize(G_NEWS), nw, sel);
    }
}

/// Comparador da 0x62dca0: só a original (entrega a notícia, que não pode sair duas vezes; a cópia leva o ramo no modo
/// Rust). ponytail: sem plano a conferir além do que a original entrega.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_lesao(this: usize, p: usize, l: usize, arg: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1471)(this, p, l, arg)
}



// ---------- 259a leva: o dia das lesões: as lesões que envelhecem e as lesões no treino, as notícias da volta aos treinos e o jogador a um cartão da suspensão (injury.cpp, discipline.cpp) ----------







/// Soma `delta` ao byte `a` (o `add`/`dec` de 8 bits da original).
pub(crate) unsafe fn cpw9_somar_byte(a: usize, delta: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(delta);
}

/// O registro de 31 bytes da pessoa de índice `idx` em [this].
pub(crate) unsafe fn cpw9_registro(this: usize, idx: i32) -> usize {
    read_usize(this).wrapping_add((idx as u32).wrapping_mul(31) as usize)
}

/// A categoria da recorrência (+0x12 do registro, com sinal) é a da tabela de tipos (0x9e7ad9, 13 bytes por tipo)?
pub(crate) unsafe fn cpw9_recorrente(this: usize, idx: i32, categoria: u8) -> bool {
    let r = read_i8(cpw9_registro(this, idx) + 0x12);
    r > -1 && *((0x9e7ad9 + (r as isize * 13) as usize) as *const u8) == categoria
}

/// O ensaio do comparador: a cópia roda só a fase do treino (sem soltar a lista das batidas, sem o físico do dia 180,
/// sem a lesão nova nem as rotinas do fim); `CPW9_PAROU` marca que ela chegou a uma lesão nova (0x62d250, que aloca e
/// manda notícia) ou ao físico de todos.
pub(crate) static mut CPW9_ENSAIO: bool = false;
pub(crate) static mut CPW9_PAROU: bool = false;
/// O que a cópia deixou nas regiões da fase do treino, à espera de a original chegar às notícias da volta (0x62e010).
pub(crate) static mut CPW9_ESPERA: Option<(Vec<(usize, usize)>, crate::comparacao::Memoria)> = None;

/// O switch dos tipos de lesão (tipo − 0xc, byte da tabela 0x62bcd8): 0 os do +0x1b/+0x36, 1 nenhuma lesão (0x14), 2 o
/// 0x1c, 3 o 0x2d, 4 os 0x3b e 0x3c, 5 só a lesão.
pub(crate) const CPW9_CASOS: [u8; 0x31] = [
    0, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 0, 5, 5, 0, 0, 2, 5, 5, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 3, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 4, 4,
];

/// frees the match knock list, lowers both counters of every injury, on day 180 of the first season the fitness of
/// everyone; on weekdays [0xae2364] / 150 draws (at least 10) of a person: a present, uninjured player who passes the
/// risk draws (player data +0x2d, proneness, recurrence) gets the category, the type (0x62d140), the attribute wear and
/// the new injury (0x62d250); at the end the return news (0x62e010) and the outbreaks (0x62e9e0)
/// Original: `Injury::daily_update` (0x62b7b0, injury.cpp, thiscall, ret 0).
///
/// [pt] { 0x62b7b0 } (thiscall, ret 0, injury.cpp): `Injury::daily_update()`, pelo laço do dia (0x5cccd0) e pela ida até a
/// data do começo (0x5cc850), todo dia: solta a lista das batidas da partida (+0x28, 0x68d040 e free); os dois contadores
/// (+0x2c, +0x2e) de cada lesão da tabela (0x30 bytes em [+4], conta +8) descem um; no dia 180 da primeira temporada
/// (0x456770 com a data do começo), o físico de todos (0x62c680). Fora do sábado e do domingo (0x54a3f0: 5 e 6), as lesões
/// no treino: [0xae2364] / 150 sorteios (pelo menos 10) de uma pessoa: jogador (+0x61) — senão o sorteio não conta —
/// nos cargos 11, 13 e 15, presente (0x418f60), sem lesão (+0xe = −1), passando pelos sorteios de risco (o +0x2d da ficha,
/// a propensão +0x1e e a recorrência +0x12/+0x13 do registro de 31 bytes; nota 20: sorteio(5) contra sorteio(2)), um em
/// três e propensão de 20: a categoria (a da recorrência ou sorteada pelas faixas 0x9e7f05/0x9e7f06), o tipo (0x62d140,
/// grave nos primeiros 1/14 dos sorteios), os atributos que a lesão gasta (+0x1b, +0x36, +0x1d da ficha) e a lesão nova
/// (0x62d250); fora da temporada (0x62ce30) nada; categoria 7 com nota abaixo de 15 não conta. No fim as notícias da volta
/// (0x62e010) e os surtos de doença (0x62e9e0).
pub(crate) unsafe extern "thiscall" fn dia_das_lesoes(this: usize) {
    let lista = read_usize(this + 0x28);
    if lista != 0 {
        if !CPW9_ENSAIO {
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
            destruir(lista);
            soltar(lista);
        }
        core::ptr::write_unaligned((this + 0x28) as *mut u32, 0);
    }
    let base = read_usize(this + 4);
    let mut k = 0i32;
    while k < read_i32(this + 8) {
        let e = base.wrapping_add((k as u32).wrapping_mul(0x30) as usize) + 0x2e;
        core::ptr::write_unaligned((e - 2) as *mut u16, read_u16(e - 2).wrapping_sub(1));
        core::ptr::write_unaligned(e as *mut u16, read_u16(e).wrapping_sub(1));
        k += 1;
    }
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if anterior(0xae2c90, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 && read_i16(G_CURRENT_DATE) == 0xb4 {
        if CPW9_ENSAIO {
            CPW9_PAROU = true;
            return;
        }
        let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
        let mut p = read_usize(G_STAFF_TABLE);
        let mut i = 0i32;
        while i < read_i32(0xae2364) {
            fisico(this, p);
            i += 1;
            p = p.wrapping_add(0x6e);
        }
    }
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    if semana(0xae2c90) as i16 != 6 && semana(0xae2c90) as i16 != 5 { // cpw9: sábado e domingo sem treino
        crate::comparacao::ramo("cpw_9 dia de treino");
        cpw9_treino(this);
    }
    if CPW9_ENSAIO {
        return;
    }
    let noticias: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_INJURY_POST_DUE_RECOVERY_NEWS);
    let surtos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_INJURY_SPREAD_ILLNESS_OUTBREAKS);
    noticias(this);
    surtos(this);
}

/// As lesões no treino do dia (o meio da 0x62b7b0).
pub(crate) unsafe fn cpw9_treino(this: usize) {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
    let fora_da_temporada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_INJURY_PLAYER_IN_CLOSE_SEASON);
    let escolher: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8 = core::mem::transmute(FN_INJURY_CHOOSE_INJURY_TYPE);
    let mut quantos = (read_i32(0xae2364) / 150) as i16; // cpw9: os sorteios do treino
    if quantos < 10 {
        quantos = 10;
    }
    let total = quantos as i32;
    let graves = (total / 14) as i16 as i32;
    let mut i = 0i32;
    while i < total {
        let idx = cpe1_sorteio(read_i32(0xae2364));
        let p = read_usize(G_STAFF_TABLE).wrapping_add((idx as u32).wrapping_mul(0x6e) as usize);
        'proximo: {
            if read_usize(p + 0x61) == 0 {
                i -= 1; // não jogador: o sorteio não conta
                break 'proximo;
            }
            let cargo = *((p + 0x3d) as *const u8);
            if cargo != 0xb && cargo != 0xf && cargo != 0xd {
                break 'proximo;
            }
            if ausente(read_usize(G_AWOL), p, 0) != 0 {
                break 'proximo;
            }
            if read_i8(read_usize(p + 0x61) + 0xf) == 0x14 {
                let r5 = cpe1_sorteio(5);
                let r2 = cpe1_sorteio(2);
                if r5 > r2 {
                    break 'proximo;
                }
            }
            let rec = cpw9_registro(this, idx);
            if read_i32(rec + 0xe) != -1 {
                break 'proximo;
            }
            if read_i8(rec + 0x12) > -1 {
                let rec = cpw9_registro(this, idx);
                let r = cpe1_sorteio(100);
                let lim = ((read_i8(read_usize(p + 0x61) + 0x2d) as i32 - 3) * 5 + read_i8(rec + 0x1e) as i32 + read_i8(rec + 0x13) as i32) * 2;
                if r > lim {
                    break 'proximo;
                }
            } else {
                let r = cpe1_sorteio(0xa0);
                let lim = ((read_i8(read_usize(p + 0x61) + 0x2d) as i32 - 3) * 5 + read_i8(cpw9_registro(this, idx) + 0x1e) as i32) * 2; // cpw9: o risco sem recorrência
                if r > lim {
                    break 'proximo;
                }
                crate::comparacao::ramo("cpw_9 risco sem recorrencia");
            }
            if cpe1_sorteio(3) != 0 {
                break 'proximo;
            }
            let rec = cpw9_registro(this, idx);
            if read_i8(rec + 0x1e) < 0x14 {
                break 'proximo;
            }
            let mut categoria: u8 = 0;
            let mut sortear_categoria = true;
            if read_i8(rec + 0x12) > -1 {
                let r1 = cpe1_sorteio(read_i8(rec + 0x13) as i32 + 2);
                let r2 = cpe1_sorteio(3);
                if r1 > r2 {
                    let r = read_i8(cpw9_registro(this, idx) + 0x12) as isize;
                    categoria = *((0x9e7ad9 + (r * 13) as usize) as *const u8);
                    sortear_categoria = false;
                }
            }
            if sortear_categoria {
                loop {
                    categoria = cpe1_sorteio(0xc) as u8;
                    if categoria == 0 && cpe1_sorteio(100) != 0 {
                        continue;
                    }
                    let c = categoria as i8 as isize * 3;
                    let n = *((0x9e7f06 + c as usize) as *const i8) as i32 - *((0x9e7f05 + c as usize) as *const i8) as i32 + 2;
                    if cpe1_sorteio(n) != 0 {
                        break;
                    }
                }
            }
            if read_i8(read_usize(p + 0x61) + 0xf) < 0xf && categoria == 7 {
                i -= 1;
                break 'proximo;
            }
            if fora_da_temporada(p) != 0 {
                break 'proximo;
            }
            let grave: i8 = if i < graves { -1 } else { 0 };
            let tipo = escolher(this, categoria as i8, grave, p);
            let k = (tipo as i8 as i32).wrapping_sub(0xc);
            let caso = if (k as u32) > 0x30 { 5 } else { CPW9_CASOS[k as usize] };
            if caso == 1 {
                break 'proximo; // o tipo 0x14: nenhuma lesão
            }
            if CPW9_ENSAIO {
                CPW9_PAROU = true;
                return;
            }
            cpw9_gasto_da_lesao(this, idx, p, categoria, caso);
            let nova: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_INJURY_ADD_INJURY);
            // ponytail: a original empurra o tipo num dword cujos bytes de cima não têm valor; a 0x62d250 lê um char
            nova(this, tipo as u32, 0, p, 0, 1, 0xffff_ffff, 0, 0);
        }
        i += 1;
    }
}

/// Os atributos que a lesão gasta (os casos 0, 2, 3 e 4 do switch da 0x62b7b0): a ficha (+0x61) é relida a cada vez.
pub(crate) unsafe fn cpw9_gasto_da_lesao(this: usize, idx: i32, p: usize, categoria: u8, caso: u8) {
    let ficha = || read_usize(p + 0x61);
    match caso {
        0 => {
            let a = read_i8(ficha() + 0x1b);
            if a == 0x14 {
                if cpw9_recorrente(this, idx, categoria) {
                    let f = ficha();
                    let r = cpe1_sorteio(3) as u8;
                    cpw9_somar_byte(f + 0x1b, 0xffu8.wrapping_sub(r));
                } else if cpe1_sorteio(3) != 0 {
                    let f = ficha();
                    let r = cpe1_sorteio(2) as u8;
                    cpw9_somar_byte(f + 0x1b, 0xffu8.wrapping_sub(r));
                }
            } else if a > 0xf && cpe1_sorteio(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
            if read_i8(ficha() + 0x36) == 0x14 {
                if cpw9_recorrente(this, idx, categoria) {
                    let f = ficha();
                    let r = cpe1_sorteio(3) as u8;
                    cpw9_somar_byte(f + 0x36, 0xffu8.wrapping_sub(r));
                } else if cpe1_sorteio(3) != 0 {
                    cpw9_somar_byte(ficha() + 0x36, 0xff);
                }
            } else if read_i8(ficha() + 0x1b) > 0x11 && cpe1_sorteio(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
        }
        2 => {
            if cpw9_recorrente(this, idx, categoria) && read_i8(ficha() + 0x1b) == 0x14 && cpe1_sorteio(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
        }
        3 => {
            let c = read_i8(ficha() + 0x1d);
            if c == 0x14 {
                *((ficha() + 0x1d) as *mut u8) = 0x13;
            } else if c > 0xf && cpe1_sorteio(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1d, 0xff);
            }
        }
        4 => {
            let c = read_i8(ficha() + 0x1d);
            if c == 0x14 {
                if cpe1_sorteio(3) != 0 {
                    cpw9_somar_byte(ficha() + 0x1d, 0xff);
                }
            } else if c > 0xf && cpe1_sorteio(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1d, 0xff);
            }
        }
        _ => {}
    }
}

/// Comparador lado a lado da fase do treino: a cópia em ensaio (sem soltar a lista, sem a lesão nova e sem as rotinas
/// do fim) e a original a partir do mesmo estado (o cabeçalho do Injury, a tabela de lesões e o sorteio); a original é
/// conferida quando chega às notícias da volta (0x62e010, desviada no mesmo item), antes das rotinas do fim. Com lesão
/// nova (aloca e manda notícia) ou com o físico de todos (dia 180 da primeira temporada): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_dia_das_lesoes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1293);
    let mut reg = vec![(this, 0x2c)];
    let n = read_i32(this + 8);
    if n > 0 && read_usize(this + 4) != 0 {
        reg.push((read_usize(this + 4), n as usize * 0x30));
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&reg);
    CPW9_ENSAIO = true;
    CPW9_PAROU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| dia_das_lesoes(this)));
    CPW9_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    if CPW9_PAROU {
        crate::comparacao::ramo("cpw_9 lesao nova so na original");
        return o(this);
    }
    *core::ptr::addr_of_mut!(CPW9_ESPERA) = Some((reg, mr));
    o(this);
    // sem a 0x62e010 desviada (desligada), a original não passou pelo ponto de conferência: fica sem comparação
    *core::ptr::addr_of_mut!(CPW9_ESPERA) = None;
}

/// at the human managers' clubs, the light training return news (once, +0x1c) and the daily fitness of every injured
/// player; in the return queue, the ones with date A today and date B today get the news (0x630ee0) and the ones with
/// past date B leave the front
/// Original: `Injury::post_due_recovery_news` (0x62e010, injury.cpp, thiscall, ret 0).
///
/// [pt] { 0x62e010 } (thiscall, ret 0, injury.cpp, quadro SEH só das datas locais): `Injury::post_due_recovery_news()`, pelo
/// dia das lesões (0x62b7b0), todo dia. Nos clubes dos 16 técnicos humanos (as últimas 16 pessoas de [0xae23bc]), cada
/// jogador do elenco (50 vagas +0xd7) lesionado (+0xe): no dia em que começo (+0x20) + dias parados (+0x2a, mais de 5)
/// chega, a notícia da volta ao treino leve (0x630ee0, uma vez: +0x1c do registro de 31 bytes) e o físico do dia
/// (0x62c680). Depois a fila das voltas ([+0x10], 22 bytes: +0 data A, +8 data B, +0x10 id, +0x14 e +0x15 o que a
/// notícia leva, conta +0x18): as de data A hoje (bsearch 0x946464 com 0x778e90, voltando enquanto não for antes de hoje,
/// 0x456740) dão a notícia com os dias entre B e A (0x54cc30); as de data B hoje (bsearch com 0x632040) a notícia da volta
/// completa; as de data B já passada saem do começo da fila.
pub(crate) unsafe extern "thiscall" fn noticias_de_volta_aos_treinos(this: usize) {
    crate::comparacao::ramo("cpw_9 noticias de volta");
    let volta: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_INJURY_POST_RECOVERY_NEWS);
    let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    for k in 0..0x10i32 {
        let n = read_i32(0xae2364);
        let s = read_usize(G_STAFF_TABLE).wrapping_add(((n - k - 1) as u32).wrapping_mul(0x6e) as usize);
        let clube = read_usize(s + 0x39);
        if clube == 0 {
            continue;
        }
        for j in 0..50usize {
            let p = read_usize(clube + 0xd7 + 4 * j);
            if p == 0 || read_i32(p) >= read_i32(0xae2364) {
                continue;
            }
            let i = read_i32(cpw9_registro(this, read_i32(p)) + 0xe);
            if i == -1 {
                continue;
            }
            let inj = read_usize(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
            let (dia, ano) = (read_i16(G_CURRENT_DATE), read_u16(0xae2c92));
            let mut t = [0u8; 8];
            let r = mais(inj + 0x20, t.as_mut_ptr(), read_u16(inj + 0x2a) as u32);
            if ano == read_u16(r + 2) && dia == read_i16(r) && read_i16(inj + 0x2a) > 5 {
                let marca = cpw9_registro(this, read_i32(p)) + 0x1c;
                if *(marca as *const u8) == 0 {
                    *(marca as *mut u8) = 1;
                    let dias = read_u16(inj + 0x28).wrapping_sub(read_u16(inj + 0x2a));
                    // ponytail: os bytes de cima dos argumentos de 8 e 16 bits não têm valor na original; a 0x630ee0 lê char/short
                    volta(this, p, 0, *(inj as *const u8) as u32, 0, dias as u32);
                }
            }
            fisico(this, p);
        }
    }
    if read_usize(this + 0x10) == 0 {
        return;
    }
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copiar: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let bsearch: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let depois: unsafe extern "thiscall" fn(*const u8, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_GET_DIFFERENCE);
    let (mut d1, mut d2, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    padrao(d1.as_mut_ptr());
    padrao(d2.as_mut_ptr());
    copiar(d1.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    copiar(d2.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    let pd1 = d1.as_ptr() as usize;
    let (d1dia, d1ano) = (read_i16(pd1), read_u16(pd1 + 2));
    let pd2 = d2.as_ptr() as usize;
    let (d2dia, d2ano) = (read_i16(pd2), read_u16(pd2 + 2));
    let ultimo = || read_usize(this + 0x10).wrapping_add((read_i32(this + 0x18) as u32).wrapping_mul(0x16) as usize).wrapping_sub(0x16);
    let pessoa = |e: usize| read_usize(G_STAFF_TABLE).wrapping_add((read_usize(e) as u32).wrapping_mul(0x6e) as usize);
    // as de data A hoje
    let mut e = bsearch(d1.as_ptr(), read_usize(this + 0x10), read_i32(this + 0x18) as u32, 0x16, 0x778e90);
    if e != 0 {
        if e > read_usize(this + 0x10) {
            loop {
                e -= 0x16;
                let (dia, ano) = (read_i16(e), read_u16(e + 2));
                if ano > d1ano || (ano == d1ano && dia > d1dia) || depois(d1.as_ptr(), read_usize(e) as u32, read_usize(e + 4) as u32) != 0 {
                    e += 0x16;
                    break;
                }
                if e <= read_usize(this + 0x10) {
                    break;
                }
            }
        }
        while e <= ultimo() {
            let (dia, ano) = (read_i16(e), read_u16(e + 2));
            if ano > d1ano || (ano == d1ano && dia > d1dia) {
                break;
            }
            if depois(d1.as_ptr(), read_usize(e) as u32, read_usize(e + 4) as u32) != 0 {
                break;
            }
            let p = pessoa(e + 0x10);
            let mut dif = [0u8; 8];
            diferenca(e + 8, dif.as_mut_ptr(), read_usize(e) as u32, read_usize(e + 4) as u32);
            let pd = dif.as_ptr() as usize;
            let dias = (read_u16(pd + 2) as u32).wrapping_mul(365).wrapping_add(read_u16(pd) as u32) as u16;
            volta(this, p, 0, *((e + 0x15) as *const u8) as u32, *((e + 0x14) as *const u8) as u32, dias as u32);
            e += 0x16;
        }
    }
    // as de data B hoje: a volta completa
    let mut e = bsearch(d1.as_ptr(), read_usize(this + 0x10), read_i32(this + 0x18) as u32, 0x16, 0x632040);
    if e == 0 {
        return;
    }
    let diferente = |b: usize| {
        let (dia, ano) = (read_i16(b), read_u16(b + 2));
        ano != d2ano || dia != d2dia
    };
    if e > read_usize(this + 0x10) {
        loop {
            e -= 0x16;
            if diferente(e + 8) {
                e += 0x16;
                break;
            }
            if e <= read_usize(this + 0x10) {
                break;
            }
        }
    }
    while e <= ultimo() {
        if diferente(e + 8) {
            break;
        }
        let p = pessoa(e + 0x10);
        volta(this, p, 1, *((e + 0x15) as *const u8) as u32, *((e + 0x14) as *const u8) as u32, 0);
        e += 0x16;
    }
    // as de data B já passada saem do começo da fila
    let n = read_i32(this + 0x18);
    let mut tirar: i16 = 0;
    if n > 0 {
        let (hd, ha) = (read_i16(G_CURRENT_DATE), read_u16(0xae2c92));
        let mut q = read_usize(this + 0x10) + 0xa;
        let mut i = 0i32;
        while i < n {
            let ano = read_u16(q);
            if ha < ano || (ha == ano && !(hd > read_i16(q - 2))) {
                break;
            }
            tirar = tirar.wrapping_add(1);
            i += 1;
            q += 0x16;
        }
        if tirar != 0 && tirar as i32 != n {
            let base = read_usize(this + 0x10);
            let t = tirar as i32 as usize;
            core::ptr::copy((base + t * 0x16) as *const u8, base as *mut u8, (n as usize - t) * 0x16);
        }
    }
    core::ptr::write_unaligned((this + 0x18) as *mut i32, read_i32(this + 0x18).wrapping_sub(tirar as i32));
}

// ponytail: as notícias da volta (0x630ee0) alocam e o físico do dia mexe no sorteio: só a original; a cópia leva o
// ramo. É também o ponto de conferência da fase do treino do dia das lesões (comparar_dia_das_lesoes).
pub(crate) unsafe extern "thiscall" fn comparar_noticias_de_volta_aos_treinos(this: usize) {
    if let Some((reg, mr)) = (*core::ptr::addr_of_mut!(CPW9_ESPERA)).take() {
        let mo = crate::comparacao::Memoria::ler(&reg);
        PLACAR.registrar(mo == mr, || format!("dia_das_lesoes({:x}) {:x?} x {:x?}", this, mo, mr));
    }
    orig::<unsafe extern "thiscall" fn(usize)>(1294)(this)
}



// ---------- 258a leva: o físico depois da partida: o ritmo de recuperação de cada jogador e as batidas que viram lesão, a batida do jogador na partida e o jogador de férias (injury.cpp) ----------



/// A pessoa da vaga de 0x49 bytes da partida (id em −0x30): sem id (−1) ou com id fora de [0xae2364], nenhuma. A
/// original confere o ponteiro nulo antes do id (ponteiro nulo passa).
pub(crate) unsafe fn cpv8_pessoa_da_vaga(vaga: usize) -> Option<(i32, usize)> {
    let id = read_i32(vaga - 0x30);
    if id == -1 {
        return None;
    }
    let pessoa = (read_i32(G_STAFF_TABLE) as u32).wrapping_add((id as u32).wrapping_mul(0x6e)) as usize;
    if pessoa != 0 && read_i32(pessoa) >= read_i32(G_STAFF_COUNT) {
        return None;
    }
    Some((id, pessoa))
}

/// O nível da vaga para o ritmo: +4 se não for negativo, senão +0, senão o padrão da partida (+0x4d).
pub(crate) unsafe fn cpv8_nivel(partida: usize, vaga: usize) -> i8 {
    let b = read_i8(vaga + 4);
    if b >= 0 {
        return b;
    }
    let b = read_i8(vaga);
    if b >= 0 {
        b
    } else {
        read_i8(partida + 0x4d)
    }
}

/// for the 20 slots of each side of the match: the rate (slot level plus (10000 − condition) / 100; half in a match
/// without the +0x40 flag; × 1.25 above 33 years, double below 23, with a random draw) goes to the recovery (0x62bd10);
/// the person's knocks in the match list become injuries (0x62d140, 0x62e700, 0x62d250)
/// Original: `Injury::apply_match_effects` (0x62c080, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62c080 } (thiscall, ret 8, injury.cpp): `Injury::apply_match_effects(jogo, partida)`, pela fechar_partida_do_dia
/// (0x6b5f00) e pelas estatísticas simuladas (0x84be20), em toda partida: para os dois lados da partida (0x625 bytes a
/// partir de +0x1c0), as 20 vagas de 0x49 bytes com pessoa (id em −0x30). O ritmo: o nível (+0x5a nas onze primeiras
/// com a marca +0x4c da partida; senão o da vaga, menos o +2 nas reservas, que com 0xff dá 0) mais (10000 − condição
/// +6) / 100, em 32 bits (o nível entra só nos 16 de baixo). Jogo sem a marca +0x40: a metade (16 bits). Com ela: acima
/// de 33 + sorteio(3) anos (+0x18), × 1,25 (0x968928, _ftol); senão abaixo de 23 − sorteio(5), o dobro. Antes do começo
/// do jogo ([0xdd6ec8] depois de hoje, 0x456740): 0. A recuperação (0x62bd10) com a condição e o ritmo. Com a batida
/// (−0x11) e a lista das batidas da partida (+0x28): cada batida da pessoa (o tipo 7 só com a ficha +0xf de 15 ou
/// mais) tem a gravidade por +5 (acima de 95: 2; de 66: 1; de 51: 0; senão a da batida anterior) e o tipo da lesão
/// (0x62d140); tipo válido: fim da lesão atual (0x62e700) e a lesão nova (0x62d250 com o jogo, ou 0 no jogo −2).
pub(crate) unsafe extern "thiscall" fn efeitos_fisicos_da_partida(this: usize, jogo: usize, partida: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let recuperar: unsafe extern "thiscall" fn(usize, usize, i16, i16) = core::mem::transmute(FN_INJURY_SET_MATCH_CONDITION);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tipo_de_lesao: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8 = core::mem::transmute(FN_INJURY_CHOOSE_INJURY_TYPE);
    let fim_da_lesao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_INJURY_END_PLAYER_INJURY);
    let nova_lesao: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, i32, u32, u32) = core::mem::transmute(FN_INJURY_ADD_INJURY);
    // ponytail: a gravidade é uma local da original que passa de uma batida para a outra; sem batida acima de 51 antes,
    // a original lê sobra de pilha e a cópia usa 0 (o comparador deixa toda partida com batida para a original).
    let mut gravidade: i8 = 0;
    for lado in 0..2usize {
        let base = partida + 0x1c0 + 0x625 * lado;
        for k in 0..0x14i8 {
            let vaga = base + 0x49 * k as usize;
            let Some((id, pessoa)) = cpv8_pessoa_da_vaga(vaga) else { continue };
            let marca = read_i8(partida + 0x4c) != 0;
            let nivel: i8 = if k < 0xb {
                if marca { 0x5a } else { cpv8_nivel(partida, vaga) }
            } else if *((vaga + 2) as *const u8) == 0xff {
                0
            } else {
                cpv8_nivel(partida, vaga).wrapping_sub(read_i8(vaga + 2))
            };
            let c = 0x2710i32.wrapping_sub(core::ptr::read_unaligned((vaga + 6) as *const i16) as i32);
            // ponytail: a original soma o ecx inteiro: o nível nos 16 de baixo, os 16 de cima de 10000 − condição
            let mut v = (c / 100).wrapping_add(((c as u32 & 0xffff_0000) | nivel as i16 as u16 as u32) as i32); // cpv8: o ritmo
            crate::comparacao::ramo("cpv_8 ritmo");
            if read_i8(jogo + 0x40) == 0 {
                crate::comparacao::ramo("cpv_8 metade");
                v = (v as i16 as i32) / 2; // cpv8: sem a marca do jogo, a metade
            } else if read_i8(pessoa + 0x18) as i32 > sorteio(3) + 0x21 {
                v = ((v as i16 as i32) as f64 * f64_em(0x968928)) as i32;
            } else if (read_i8(pessoa + 0x18) as i32) < 0x17 - sorteio(5) {
                crate::comparacao::ramo("cpv_8 jovem");
                v = v.wrapping_add(v); // cpv8: o jovem em dobro
            }
            if depois(0xdd6ec8, read_i32(G_CURRENT_DATE) as u32, read_i32(G_CURRENT_DATE_LEAP) as u32) != 0 {
                v = 0;
            }
            recuperar(this, pessoa, core::ptr::read_unaligned((vaga + 6) as *const i16), v as i16);
            if *((vaga - 0x11) as *const u8) == 0 || read_i32(this + 0x28) == 0 {
                continue;
            }
            let lista = read_i32(this + 0x28) as usize;
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
            let mut e = proximo(read_i32(this + 0x28) as usize);
            while e != 0 {
                let tipo = read_i8(e + 4);
                if read_i32(e) == id && (tipo != 7 || read_i8(read_i32(pessoa + 0x61) as usize + 0xf) >= 0xf) {
                    let g = read_i8(e + 5);
                    if g > 0x5f {
                        gravidade = 2;
                    } else if g > 0x41 {
                        gravidade = 1;
                    } else if g > 0x32 {
                        gravidade = 0;
                    }
                    let t = tipo_de_lesao(this, tipo, gravidade, pessoa);
                    if (t as i8) >= 0 {
                        crate::comparacao::ramo("cpv_8 batida vira lesao");
                        fim_da_lesao(this, pessoa, 0);
                        let j = if read_i32(jogo) == -2 { 0 } else { jogo };
                        nova_lesao(this, t as u32, 0, pessoa, j, 1, -1, 0, 0);
                    }
                }
                e = proximo(read_i32(this + 0x28) as usize);
            }
        }
    }
}

/// A pessoa tem batida na lista da partida que chega à lesão (a mesma conferência da 0x62c080)? O cursor (+0xa) volta
/// ao que era.
pub(crate) unsafe fn cpv8_tem_batida(this: usize, id: i32, pessoa: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let lista = read_i32(this + 0x28) as usize;
    let cursor = core::ptr::read_unaligned((lista + 0xa) as *const u32);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut achou = false;
    let mut e = proximo(lista);
    while e != 0 && !achou {
        achou = read_i32(e) == id && (read_i8(e + 4) != 7 || read_i8(read_i32(pessoa + 0x61) as usize + 0xf) >= 0xf);
        e = proximo(lista);
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor);
    achou
}

/// Comparador: lado a lado no estado do sorteio, no cursor da lista das batidas e na recuperação (+0xa, 4 bytes, do
/// registro de 31 bytes em [this]) de cada pessoa das vagas. Partida com batida que chega ao tipo da lesão: só a original (a lesão nova aloca e manda a
/// notícia; a gravidade da primeira batida pode ser sobra de pilha).
pub(crate) unsafe extern "thiscall" fn comparar_efeitos_fisicos_da_partida(this: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1254);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let mut batida = false;
    for lado in 0..2usize {
        for k in 0..0x14usize {
            let vaga = partida + 0x1c0 + 0x625 * lado + 0x49 * k;
            let Some((id, pessoa)) = cpv8_pessoa_da_vaga(vaga) else { continue };
            if pessoa == 0 {
                return o(this, jogo, partida);
            }
            reg.push(((read_i32(this) as u32).wrapping_add((id as u32).wrapping_mul(31)).wrapping_add(0xa) as usize, 4));
            if *((vaga - 0x11) as *const u8) != 0 && read_i32(this + 0x28) != 0 && cpv8_tem_batida(this, id, pessoa) {
                batida = true;
            }
        }
    }
    if batida {
        crate::comparacao::ramo("efeitos_fisicos_da_partida com batida");
        return o(this, jogo, partida);
    }
    if read_i32(this + 0x28) != 0 {
        reg.push((read_i32(this + 0x28) as usize + 0xa, 4)); // o cursor da lista das batidas
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, partida), || crate::aleatorio::sem_contar(|| efeitos_fisicos_da_partida(this, jogo, partida)));
    PLACAR.registrar(igual, || format!("efeitos_fisicos_da_partida({:x}, {:x}) {:x?} x {:x?}", jogo, partida, mo, mr));
}

/// the match knock list (+0x28) item with the person's id, or 0
/// Original: `Injury::find_match_injury` (0x62bf60, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62bf60 } (thiscall, ret 4, injury.cpp): `Injury::find_match_injury(pessoa)`: o item da lista das batidas da
/// partida (+0x28; cursor +0xa recomeça de +2, 0x68d1b0) com o id da pessoa, ou 0 (sem lista também). Pelas telas do
/// pessoal (0x870650, 0x885820).
pub(crate) unsafe extern "thiscall" fn batida_do_jogador(this: usize, pessoa: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let lista = read_i32(this + 0x28) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut e = proximo(read_i32(this + 0x28) as usize);
    while e != 0 {
        if read_i32(e) == read_i32(pessoa) {
            return e;
        }
        e = proximo(read_i32(this + 0x28) as usize);
    }
    0
}

/// Comparador: lado a lado no cursor da lista e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_batida_do_jogador(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1255);
    let lista = read_i32(this + 0x28) as usize;
    let reg: &[(usize, usize)] = if lista == 0 { &[] } else { &[(lista + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, pessoa), || batida_do_jogador(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("batida_do_jogador({:x}) original={:x} {:x?} rust={:x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// 1 if the person's current injury (0x62ed70 on [0xb63d28]) is a holiday (type 0x51)
/// Original: `Injury::is_player_on_holiday` (0x631550, injury.cpp, thiscall, ret 4 (this não é lido)).
///
/// [pt] { 0x631550 } (ret 4, injury.cpp; `this` não é lido): `Injury::is_player_on_holiday(pessoa)`: 1 se a lesão atual
/// da pessoa (0x62ed70 no [0xb63d28]) é de férias (tipo 0x51). Pela tela do clube (0x4849f0).
pub(crate) unsafe extern "thiscall" fn jogador_de_ferias(_this: usize, pessoa: usize) -> u8 {
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let e = atual(read_i32(G_INJURY) as usize, pessoa);
    (e != 0 && *(e as *const u8) == 0x51) as u8
}

/// Comparador: a devolução (só o al).
pub(crate) unsafe extern "thiscall" fn comparar_jogador_de_ferias(this: usize, pessoa: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1256);
    let r = o(this, pessoa);
    let n = jogador_de_ferias(this, pessoa);
    PLACAR.registrar(r == n, || format!("jogador_de_ferias({:x}) original={} rust={}", pessoa, r, n));
    r
}

// ---------- 258a leva: o fim da lesão do jogador, a notícia do pedido de desculpas e a caixa de entrada esvaziada (injury.cpp, media.cpp, news.cpp) ----------


/// O registro de 31 bytes da pessoa `id` em [lesoes] (o físico e a lesão atual em +0xe).
pub(crate) unsafe fn cpv10_registro(lesoes: usize, id: i32) -> usize {
    (read_i32(lesoes) as u32).wrapping_add((id as u32).wrapping_mul(31)) as usize
}

/// ends the person's current injury: the history (except holidays), the return news at the human club (when asked),
/// entry type 0xff and the record's +0xe −1
/// Original: `Injury::end_player_injury` (0x62e700, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62e700 } (thiscall, ret 8, injury.cpp): `Injury::end_player_injury(pessoa, noticia)`, pelo físico do dia
/// (0x62c680, com notícia), pelas batidas da partida (0x62c080, sem) e pelo recomeço do físico (0x62bfa0, sem): com a
/// lesão atual (+0xe do registro, índice na tabela de 0x30 bytes em [this+4]; −1 nenhuma): fora das férias (tipo
/// 0x51), o histórico ([this+0xc], 0x58e340 com +0x14 e +0x18 do registro); com `noticia` e o clube da pessoa (+0x39)
/// humano (0x600a50), a notícia da volta (0x630ee0 com 1 e o tipo). O tipo vira 0xff e o +0xe −1. Sem pessoa: a caixa
/// de erro (linha 0xc87).
pub(crate) unsafe extern "thiscall" fn fim_da_lesao(this: usize, pessoa: usize, noticia: u32) {
    if pessoa == 0 {
        aviso_em(0x9e7f28, 0xc87);
        return;
    }
    let id = read_i32(pessoa);
    if id >= read_i32(G_STAFF_COUNT) { // cpv10: o id da pessoa
        return;
    }
    if id >= read_i32(G_STAFF_COUNT) / 2 {
        crate::comparacao::ramo("cpv_10 metade de cima");
    }
    let r = cpv10_registro(this, id);
    let i = read_i32(r + 0xe);
    if i == -1 { // cpv10: a lesão atual
        return;
    }
    if i == 0 {
        crate::comparacao::ramo("cpv_10 entrada zero");
    }
    let e = (read_i32(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    if *(e as *const u8) != 0x51 {
        let historico: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
        historico(read_i32(this + 0xc) as usize, r + 0x14, r + 0x18, e as *const u8);
    }
    let clube = read_i32(pessoa + 0x39) as usize;
    if noticia != 0 && clube != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if humano(clube, 0, 1, 0, 0) != 0 {
            let volta: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_INJURY_POST_RECOVERY_NEWS);
            // ponytail: a original empurra o ecx inteiro com o tipo no byte de baixo; a 0x630ee0 lê o byte
            volta(this, pessoa, 1, *(e as *const u8) as u32, 0, 0);
        }
    }
    crate::comparacao::ramo("cpv_10 fim da lesao");
    *(e as *mut u8) = 0xff; // cpv10: a entrada fica livre
    core::ptr::write_unaligned((cpv10_registro(this, read_i32(pessoa)) + 0xe) as *mut i32, -1); // cpv10: sem lesão atual
}

/// Comparador: lado a lado na entrada (tipo) e no +0xe do registro quando nada sai da rotina (férias e sem notícia);
/// com o histórico (arquivo) ou a notícia: só a original, conferindo o plano (tipo 0xff e +0xe −1). Sem pessoa: a
/// original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_fim_da_lesao(this: usize, pessoa: usize, noticia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1260);
    if pessoa == 0 || read_i32(pessoa) >= read_i32(G_STAFF_COUNT) {
        return o(this, pessoa, noticia);
    }
    let r = cpv10_registro(this, read_i32(pessoa));
    let i = read_i32(r + 0xe);
    if i == -1 {
        return o(this, pessoa, noticia);
    }
    let e = (read_i32(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let clube = read_i32(pessoa + 0x39) as usize;
    let fora = *(e as *const u8) != 0x51 || (noticia != 0 && clube != 0 && humano(clube, 0, 1, 0, 0) != 0);
    if fora {
        o(this, pessoa, noticia);
        let ok = *(e as *const u8) == 0xff && read_i32(cpv10_registro(this, read_i32(pessoa)) + 0xe) == -1;
        PLACAR.registrar(ok, || format!("fim_da_lesao({:x}, {}) plano: tipo {:x} e +0xe {}", pessoa, noticia, *(e as *const u8), read_i32(cpv10_registro(this, read_i32(pessoa)) + 0xe)));
        return;
    }
    let reg = [(e, 1), (r + 0xe, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, noticia), || fim_da_lesao(this, pessoa, noticia));
    PLACAR.registrar(igual, || format!("fim_da_lesao({:x}, {}) {:x?} x {:x?}", pessoa, noticia, mo, mr));
}




/// As regiões que a 0x787630 grava (a caixa e, com a primeira notícia, +0xd6..+0xde das 100 notícias).
pub(crate) unsafe fn cpv10_regioes_da_caixa(this: usize, caixa: usize) -> Vec<(usize, usize)> {
    let mut reg = vec![(caixa, 12)];
    let primeira = read_i32(caixa);
    if primeira != -1 {
        for k in 0..0x64u32 {
            reg.push(((read_i32(this) as u32).wrapping_add(k.wrapping_add(primeira as u32).wrapping_mul(0xdf)) as usize + 0xd6, 8));
        }
    }
    reg
}


// ---------- 254a leva: o estado da lesão do jogador, a data da decisão sobre as ofertas e os candidatos a técnico do ano (injury.cpp, contract_manager.cpp, staff_records.cpp) ----------




/// O estado pela lesão atual da pessoa (o índice em [this] + id × 31 + 0xe; −1 sem lesão): a diferença da data do jogo
/// para a da lesão (+0x20 do registro de 48 bytes em [this+4], 0x54cc30) em anos × 365 + dias; abaixo de +0x2a, 2
/// (lesionado); abaixo de +0x28, 1 (recuperando a forma); senão 0.
pub(crate) unsafe fn cpr10_estado(this: usize, pessoa: usize) -> u8 {
    let id = read_u32_as_usize(pessoa);
    let k = read_u32_as_usize(read_u32_as_usize(this).wrapping_add(id.wrapping_mul(31)) + 0xe) as i32;
    if k == -1 {
        return 0;
    }
    let l = read_u32_as_usize(this + 4).wrapping_add((k as usize).wrapping_mul(48));
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_GET_DIFFERENCE);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), read_u32_as_usize(l + 0x20) as u32, read_u32_as_usize(l + 0x24) as u32);
    let dias = (read_u16(t.as_ptr() as usize + 2) as i32).wrapping_mul(365).wrapping_add(read_i16(t.as_ptr() as usize) as i32);
    if dias < read_i16(l + 0x2a) as i32 {
        return 2; // cpr10: lesionado
    }
    (dias < read_i16(l + 0x28) as i32) as u8 // cpr10: recuperando a forma
}

/// updates the daily fitness (0x62c680) and returns 2 while the injury lasts, 1 while recovering match fitness and 0
/// when fit, by the gap from the injury date to the game date
/// Original: `Injury::get_player_injury_status` (0x62c400, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62c400 } (thiscall, ret 4, injury.cpp): `Injury::get_player_injury_status(pessoa)`: pessoa nula, a caixa
/// "Error" (linha 0x556) e 0; id a partir de [0xae2364], 0. Senão a forma do dia (0x62c680) e o estado
/// (cpr10_estado). Quente (escalação automática): sem contador.
pub(crate) unsafe extern "thiscall" fn estado_da_lesao(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        aviso_em(0x9e7f28, 0x556);
        return 0;
    }
    if read_u32_as_usize(pessoa) as i32 >= read_u32_as_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
    atualizar(this, pessoa);
    cpr10_estado(this, pessoa)
}

pub(crate) unsafe fn cpr10_mais(d: usize, n: u32, t: &mut [u8; 8]) -> usize {
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    mais(d, t.as_mut_ptr(), n)
}

/// A data `d` = a data em `p` (0x417c10).
pub(crate) unsafe fn cpr10_copiar(d: usize, p: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(d, t.as_mut_ptr(), p);
}

/// A data `d` = a data do jogo + `n` dias (0x54c5b0, 0x417c10).
pub(crate) unsafe fn cpr10_hoje_mais(d: usize, n: u32) {
    let mut t = [0u8; 8];
    cpr10_copiar(d, cpr10_mais(0xae2c90, n, &mut t));
}

/// A data `d` passa para o dia seguinte quando a fatia (+0xc da entrada, com sinal) passa de 2 (menos 3).
pub(crate) unsafe fn cpr10_acertar_fatia(e: usize, f: i8) {
    *((e + 0xc) as *mut i8) = f;
    if f > 2 {
        *((e + 0xc) as *mut i8) = f.wrapping_sub(3);
        let mut t = [0u8; 8];
        cpr10_copiar(e + 4, cpr10_mais(e + 4, 1, &mut t));
    }
}

/// A entrada de decisão de tipo 3 da pessoa no vetor de 14 bytes ([this+0xc], conta +0x14): da posição +0x18, nas
/// entradas até `limite` (ano sem sinal, dia com sinal), a primeira de tipo 3 da pessoa é anulada (id −1; sem
/// `alterar` só fica de fora) e a busca para; depois, a partir dessa posição (ou da conta − [this+0x1c], se maior),
/// a de tipo 3 da pessoa. Devolve o endereço dela ou 0.
pub(crate) unsafe fn cpr10_procurar(this: usize, id: i32, limite: &[u8; 8], alterar: bool) -> usize {
    let conta = read_u32_as_usize(this + 0x14) as i32;
    let base = read_u32_as_usize(this + 0xc);
    let (la, ld) = (u16::from_le_bytes([limite[2], limite[3]]), i16::from_le_bytes([limite[0], limite[1]]));
    let mut i = read_u32_as_usize(this + 0x18) as i32;
    let mut anulada = -1i32;
    while i < conta {
        let r = base.wrapping_add((i as usize).wrapping_mul(14));
        let ra = read_u16(r + 6);
        if la < ra || (la == ra && ld < read_i16(r + 4)) {
            break;
        }
        if *((r + 0xd) as *const u8) == 3 && read_u32_as_usize(r) as i32 == id {
            if alterar {
                core::ptr::write_unaligned(r as *mut u32, u32::MAX);
            }
            anulada = i;
            break;
        }
        i += 1;
    }
    let conta = read_u32_as_usize(this + 0x14) as i32;
    i = i.max(conta.wrapping_sub(read_u32_as_usize(this + 0x1c) as i32));
    while i < conta {
        let r = read_u32_as_usize(this + 0xc).wrapping_add((i as usize).wrapping_mul(14));
        if *((r + 0xd) as *const u8) == 3 && read_u32_as_usize(r) as i32 == id && i != anulada {
            return r;
        }
        i += 1;
    }
    0
}



/// Comparador: a original (com a forma do dia) e o estado relido pela regra (cpr10_estado, sem rodar a 0x62c680 de
/// novo: a original já a rodou).
pub(crate) unsafe extern "thiscall" fn comparar_estado_da_lesao(this: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1104)(this, pessoa);
    if pessoa != 0 && (read_u32_as_usize(pessoa) as i32) < read_u32_as_usize(G_STAFF_COUNT) as i32 {
        let n = cpr10_estado(this, pessoa);
        PLACAR.registrar(r == n, || format!("estado_da_lesao(pessoa {:x}) original={} rust={}", pessoa, r, n));
    }
    r
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
pub(crate) const CPR10_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];



// ---------- 252a leva: as doenças e a pré-temporada: acrescentar o surto de doença no clube, espalhar os surtos pelos elencos e o jogador fora da temporada (injury.cpp) ----------






/// O clube pelo índice (registro de 0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn cpp8_clube(i: i32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(0x245))
}

pub(crate) unsafe fn cpp8_selecao(clube: usize) -> bool {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    e_selecao(clube) != 0
}

/// Um surto (17 bytes, vetor [this+0x1c], capacidade +0x20, conta +0x24): +0 a data (8 bytes), +8 o tipo da doença,
/// +9 o índice do clube, +0xd o id da partida (−1 sem).
pub(crate) const CPP8_SURTO: usize = 0x11;

/// Se o surto entra (os sorteios da 0x62e820, na ordem): tipo 4 nunca; tipo 3 só com 1 em 15 (sorteio de 15 = 0); os
/// outros com 3 em 4 (sorteio de 4 ≠ 0); e nunca numa seleção (0x53b900).
pub(crate) unsafe fn cpp8_surto_entra(tipo: u8, clube_i: i32) -> bool {
    if tipo == 4 {
        return false;
    }
    if tipo == 3 {
        if cpe1_sorteio(0xf) != 0 {
            return false;
        }
    } else if cpe1_sorteio(4) == 0 { // 1 em 4 fica de fora
        return false;
    }
    !cpp8_selecao(cpp8_clube(clube_i))
}

/// type 4 never; type 3 with 1 in 15, the others with 3 in 4; outside national teams, stores the outbreak (date, type,
/// club, fixture) in the array (grown by 30) and re-sorts it by date
/// Original: `Injury::add_illness_outbreak` (0x62e820, injury.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x62e820 } (thiscall, ret 0x10, injury.cpp): `Injury::add_illness_outbreak(data, tipo, clube, partida)`: o tipo
/// é o byte baixo; se o surto entra (`cpp8_surto_entra`), cresce o vetor de 30 em 30 quando cheio (0x945667, ou
/// 0x944e46 sem vetor; sem memória, a caixa "Error" da linha 0xd01 e nada), grava o surto no fim (data pela 0x417c10)
/// e reordena pela data (qsort 0x945333 com 0x778e90).
pub(crate) unsafe extern "thiscall" fn acrescentar_surto_de_doenca(this: usize, data: usize, tipo: u32, clube_i: i32, partida: usize) {
    let tipo = tipo as u8;
    if !cpp8_surto_entra(tipo, clube_i) {
        return;
    }
    crate::comparacao::ramo("cpp_8 surto novo");
    let cap = read_i32(this + 0x20);
    if read_i32(this + 0x24) >= cap {
        let cap = cap.wrapping_add(30);
        core::ptr::write_unaligned((this + 0x20) as *mut i32, cap);
        let tamanho = (cap as u32).wrapping_mul(CPP8_SURTO as u32);
        let v = read_usize(this + 0x1c);
        let novo = if v != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(v, tamanho)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(tamanho)
        };
        core::ptr::write_unaligned((this + 0x1c) as *mut usize, novo);
        if novo == 0 {
            aviso_em(0x9e7f28, 0xd01);
            return;
        }
    }
    let e = read_usize(this + 0x1c).wrapping_add((read_i32(this + 0x24) as usize).wrapping_mul(CPP8_SURTO));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut rascunho = [0u8; 8];
    copiar(e, rascunho.as_mut_ptr(), data);
    *((e + 8) as *mut u8) = tipo;
    core::ptr::write_unaligned((e + 9) as *mut i32, clube_i);
    let id = if partida != 0 { read_i32(partida) } else { -1 };
    core::ptr::write_unaligned((e + 0xd) as *mut i32, id);
    let n = read_i32(this + 0x24).wrapping_add(1);
    core::ptr::write_unaligned((this + 0x24) as *mut i32, n);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    ordenar(read_usize(this + 0x1c), n as u32, CPP8_SURTO as u32, 0x778e90);
}

/// A data de cinco dias atrás (0x54c8e0 sobre a data do jogo).
pub(crate) unsafe fn cpp8_cinco_dias_atras() -> [u8; 8] {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut d = [0u8; 8];
    datar(0xae2c90, &mut d, 5); // cinco dias atrás
    d
}

/// A data `limite` (dia com sinal em +0, ano sem sinal em +2) é depois da do surto `e`?
pub(crate) unsafe fn cpp8_depois(limite: &[u8; 8], e: usize) -> bool {
    let dia = i16::from_le_bytes([limite[0], limite[1]]);
    let ano = u16::from_le_bytes([limite[2], limite[3]]);
    let ano_e = read_i16(e + 2) as u16;
    ano > ano_e || (ano == ano_e && dia > read_i16(e))
}

/// up to the first outbreak after today: one older than five days is removed; the others make random squad players of
/// the club ill (type 3 only on the day, 1 to 4; the others 0 to 2; up to 20 tries) through the new injury 0x62d250
/// Original: `Injury::spread_illness_outbreaks` (0x62e9e0, injury.cpp, thiscall).
///
/// [pt] { 0x62e9e0 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::spread_illness_outbreaks()`: percorre
/// os surtos (ordenados pela data) até o primeiro depois de hoje. O de mais de cinco dias (0x54c8e0) conta para sair;
/// os outros espalham: tipo 3 só no próprio dia, com 1 a 4 doentes (sorteio de 4, +1); os outros 0 a 2 (sorteio de 3).
/// Fora das seleções (0x53b900), até 20 tentativas: uma das 50 vagas do elenco (sorteio de 50, +0xd7), jogador (id
/// abaixo de [0xae2364]) sem essa doença (+0x1d do registro de 31 bytes em [this]) fica doente (0x62d250: o tipo no
/// byte baixo da dword do deslocamento do surto, como a original; a partida do surto). No fim, os que saem vão
/// embora do começo do vetor (os outros descem) e a conta cai.
pub(crate) unsafe extern "thiscall" fn espalhar_surtos_de_doenca(this: usize) {
    let cinco = cpp8_cinco_dias_atras();
    let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(FN_INJURY_ADD_INJURY);
    let mut removidos: i32 = 0;
    let mut k: i32 = 0;
    let mut off: usize = 0;
    while k < read_i32(this + 0x24) {
        let e = read_usize(this + 0x1c).wrapping_add(off);
        let hoje4 = read_usize(G_CURRENT_DATE) as u32;
        let (ano_e, ano_h) = (read_i16(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < read_i16(e)) {
            break; // o primeiro surto depois de hoje
        }
        if cpp8_depois(&cinco, e) {
            crate::comparacao::ramo("cpp_8 surto antigo");
            removidos += 1;
        } else {
            let tipo = read_u8(e + 8);
            let quantos = if tipo == 3 {
                if read_usize(G_CURRENT_DATE) as u32 == read_usize(e) as u32 { Some((cpe1_sorteio(4) as u8).wrapping_add(1)) } else { None }
            } else {
                Some(cpe1_sorteio(3) as u8) // 0 a 2 doentes
            };
            if let Some(mut quantos) = quantos {
                crate::comparacao::ramo("cpp_8 espalhar");
                let clube = cpp8_clube(read_i32(read_usize(this + 0x1c).wrapping_add(off) + 9));
                if !cpp8_selecao(clube) {
                    let mut t = 0;
                    while t < 0x14 && (quantos as i8) > 0 {
                        let vaga = cpe1_sorteio(0x32) as i8 as isize;
                        let p = read_usize((clube + 0xd7).wrapping_add((vaga * 4) as usize));
                        if p != 0 && read_i32(p) < read_i32(0xae2364) {
                            let e = read_usize(this + 0x1c).wrapping_add(off);
                            let doenca = read_u8(e + 8);
                            let registro = read_usize(this).wrapping_add((read_i32(p) as usize).wrapping_mul(31));
                            if read_u8(registro + 0x1d) != doenca {
                                lesionar(this, (off as u32 & 0xffff_ff00) | doenca as u32, 0, p, 0, 0, read_i32(e + 0xd), 0, 0);
                                quantos = quantos.wrapping_sub(1);
                            }
                        }
                        t += 1;
                    }
                }
            }
        }
        k += 1;
        off += CPP8_SURTO;
    }
    let n = read_i32(this + 0x24);
    if removidos != 0 && removidos != n {
        let base = read_usize(this + 0x1c);
        core::ptr::copy((base + removidos as usize * CPP8_SURTO) as *const u8, base as *mut u8, (n - removidos) as usize * CPP8_SURTO);
    }
    core::ptr::write_unaligned((this + 0x24) as *mut i32, n - removidos);
}

/// 1 without a person; 1 when the club (without the +0x1db mark) is out of its nation's season with less than 40 days
/// to go and there is no future national call-up; otherwise 0
/// Original: `injury_player_in_close_season` (0x62ce30, injury.cpp, stdcall, ret 4).
///
/// [pt] { 0x62ce30 } (stdcall, ret 4, injury.cpp, quadro SEH sem destrutor): `injury_player_in_close_season(pessoa)`: sem
/// pessoa, 1. Com seleção (+0x24), a data dela (0x76e040 sobre [0xb74318], a nação +0x53 da seleção, a classe da
/// 0x76d780 pelo bit 3 do +0x5e) depois de hoje conta como convocado. Sem clube (+0x39), convocado ou clube com +0x1db
/// ≠ −1: 0. Senão, fora da temporada da nação do clube (0x669bd0 com as duas datas) e faltando menos de 40 dias
/// (0x54cdb0 de hoje até a segunda data, 16 bits sem sinal): 1; senão 0.
pub(crate) unsafe extern "stdcall" fn jogador_fora_da_temporada(pessoa: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut inicio = [0u32; 2];
    let mut fim = [0u32; 2];
    construir(inicio.as_mut_ptr());
    construir(fim.as_mut_ptr());
    if pessoa == 0 {
        return 1;
    }
    let mut convocado = false;
    let selecao = read_usize(pessoa + 0x24);
    if selecao != 0 {
        let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_SQUAD_SLOT);
        let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_SQUAD_SLOT_RECORD);
        let k = classe(selecao, (read_u8(pessoa + 0x5e) & 8) as u32);
        // ponytail: a original empurra a dword do argumento com só o byte baixo trocado pelo al; a 0x76e040 lê só o byte
        let q = campo(read_usize(G_NATIONAL_TEAMS), read_usize(selecao + 0x53), k as u32);
        let lo = read_usize(q) as u32;
        let (ano, ano_h) = ((lo >> 16) as u16, read_i16(G_CURRENT_DATE_YEAR) as u16);
        convocado = ano > ano_h || (ano == ano_h && (lo as u16 as i16) > read_i16(G_CURRENT_DATE));
    }
    let clube = read_usize(pessoa + 0x39);
    if clube == 0 || convocado || read_i32(clube + 0x1db) != -1 {
        return 0;
    }
    let temporada: unsafe extern "C" fn(usize, *mut u32, *mut u32) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
    if temporada(clube, inicio.as_mut_ptr(), fim.as_mut_ptr()) != 0 {
        return 0;
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if (dias(0xae2c90, fim[0], fim[1]) as u16) < 0x28 {
        return 1;
    }
    0
}

/// Comparador: a original sorteia, aloca e reordena: o plano antes (os sorteios e a seleção, com o estado do sorteio
/// guardado e devolvido); a original roda. Confere o mesmo estado do sorteio e, se o surto entrou, a conta +1 e o surto
/// (data, tipo, clube, partida) no vetor; senão a conta igual.
pub(crate) unsafe extern "thiscall" fn comparar_acrescentar_surto_de_doenca(this: usize, data: usize, tipo: u32, clube_i: i32, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, usize)>(1027);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let entra = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cpp8_surto_entra(tipo as u8, clube_i)));
    let depois_do_plano = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    let n0 = read_i32(this + 0x24);
    let mut esperado = [0u8; CPP8_SURTO];
    esperado[..8].copy_from_slice(&*(data as *const [u8; 8]));
    esperado[8] = tipo as u8;
    esperado[9..13].copy_from_slice(&clube_i.to_le_bytes());
    let id = if partida != 0 { read_i32(partida) } else { -1 };
    esperado[13..17].copy_from_slice(&id.to_le_bytes());
    o(this, data, tipo, clube_i, partida);
    let n = read_i32(this + 0x24);
    let mesmo_sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois_do_plano;
    let v = read_usize(this + 0x1c);
    let ok = if entra && v != 0 {
        n == n0 + 1 && (0..n.max(0) as usize).any(|k| *((v + k * CPP8_SURTO) as *const [u8; CPP8_SURTO]) == esperado)
    } else {
        n == n0
    };
    PLACAR.registrar(ok && mesmo_sorteio, || {
        format!("acrescentar_surto_de_doenca(tipo {}, clube {}) plano entra={}, conta {} -> {}, mesmo sorteio={}", tipo as u8, clube_i, entra, n0, n, mesmo_sorteio)
    });
}

/// Comparador: a original sorteia e adoece (0x62d250, que pode acrescentar surtos): roda só ela e confere o plano
/// feito antes: quantos surtos tinham mais de cinco dias (os que saem) e, depois, nenhum surto com mais de cinco dias e
/// a conta pelo menos a de antes menos os que saem.
pub(crate) unsafe extern "thiscall" fn comparar_espalhar_surtos_de_doenca(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1028);
    let cinco = cpp8_cinco_dias_atras();
    let n0 = read_i32(this + 0x24);
    let hoje4 = read_usize(G_CURRENT_DATE) as u32;
    let mut saem = 0;
    for k in 0..n0.max(0) as usize {
        let e = read_usize(this + 0x1c) + k * CPP8_SURTO;
        let (ano_e, ano_h) = (read_i16(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < read_i16(e)) {
            break;
        }
        if cpp8_depois(&cinco, e) {
            saem += 1;
        }
    }
    o(this);
    let n = read_i32(this + 0x24);
    let velho = (0..n.max(0) as usize).find(|&k| cpp8_depois(&cinco, read_usize(this + 0x1c) + k * CPP8_SURTO));
    PLACAR.registrar(velho.is_none() && n >= n0 - saem, || {
        format!("espalhar_surtos_de_doenca: conta {} -> {} (saem {}), surto velho {:?}", n0, n, saem, velho)
    });
}

/// Comparador: pelo valor (datas, a seleção e a temporada só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_jogador_fora_da_temporada(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1029)(pessoa);
    let n = jogador_fora_da_temporada(pessoa);
    PLACAR.registrar(r == n, || format!("jogador_fora_da_temporada({:x}) original={} rust={}", pessoa, r, n));
    r
}

// ---------- 252a leva: a criação do jogo: a forma inicial dos elencos, os registros de lesão com as lesões iniciais e as listas de clubes por nação dos jovens (injury.cpp, player_regen.cpp) ----------







/// Os clubes sem as seleções do fim do vetor: [0xae235c] − 2 × [0xae2350].
pub(crate) unsafe fn cpp9_clubes_sem_selecoes() -> i32 {
    read_i32(0xae235c).wrapping_sub(read_i32(0xae2350).wrapping_mul(2))
}

/// Os dias de pré-temporada que contam na forma inicial: dentro da temporada da nação do clube (0x669bd0), 0; fora,
/// os dias do começo dela até hoje (0x54cdb0 sobre a primeira data): acima de 80, 40; de 41 a 80, 80 − dias (no byte,
/// com sinal); senão os dias.
pub(crate) unsafe fn cpp9_dias_de_preparo(clube: usize, inicio: &mut [u32; 2], fim: &mut [u32; 2]) -> i32 {
    let temporada: unsafe extern "C" fn(usize, *mut u32, *mut u32) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
    if temporada(clube, inicio.as_mut_ptr(), fim.as_mut_ptr()) != 0 {
        return 0;
    }
    let dias: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let d = dias(inicio.as_mut_ptr(), read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
    if d > 0x50 {
        0x28
    } else if d > 0x28 {
        (0x50u8.wrapping_sub(d as u8)) as i8 as i32
    } else {
        d
    }
}

/// for each club, the pre-season days (0 in season; up to 40); each squad player's fitness = 10000 − 75 × (days −
/// natural fitness/2 − +0x5b/4, between 1 and 40); non-players leave the slot (and the club, job 0xb)
/// Original: `Injury::init_squad_fitness` (0x62a920, injury.cpp, thiscall).
///
/// [pt] { 0x62a920 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::init_squad_fitness()`: para cada
/// clube (sem as seleções), os dias de preparo (`cpp9_dias_de_preparo`); cada uma das 50 vagas (+0xd7) com pessoa de
/// id abaixo de [0xae2364]: jogador (+0x61): v = dias − (+0x34 do jogador)/2 − (+0x5b da pessoa)/4 (com sinal,
/// truncado), entre 1 e 40, e a forma (+8 do registro de 31 bytes em [this]) = 10000 − 75 × v; quem não é jogador sai
/// da vaga (0), e se o clube dele (+0x39) é este: cargo (+0x3d) 0xb fica sem clube e 8, 0xf vira 8, 0xd vira 6.
pub(crate) unsafe extern "thiscall" fn forma_inicial_dos_elencos(this: usize) {
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut inicio = [0u32; 2];
    let mut fim = [0u32; 2];
    construir(inicio.as_mut_ptr());
    construir(fim.as_mut_ptr());
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < cpp9_clubes_sem_selecoes() {
        let clube = base.wrapping_add((i as usize).wrapping_mul(0x245));
        let dias = cpp9_dias_de_preparo(clube, &mut inicio, &mut fim);
        for s in 0..50usize {
            let vaga = clube + 0xd7 + 4 * s;
            let p = read_usize(vaga);
            if p == 0 || read_i32(p) >= read_i32(0xae2364) {
                continue;
            }
            let jogador = read_usize(p + 0x61);
            if jogador != 0 {
                crate::comparacao::ramo("cpp_9 forma inicial");
                let v = dias - (read_u8(jogador + 0x34) as i8 as i32) / 2 - (read_u8(p + 0x5b) as i8 as i32) / 4;
                let v = v.clamp(1, 0x28);
                let r = read_usize(this).wrapping_add((read_i32(p) as usize).wrapping_mul(31));
                core::ptr::write_unaligned((r + 8) as *mut u16, (10000 - v * 75) as u16); // a forma inicial
            } else {
                if read_usize(p + 0x39) == clube {
                    match read_u8(p + 0x3d) {
                        0xb => {
                            core::ptr::write_unaligned((p + 0x39) as *mut usize, 0);
                            *((p + 0x3d) as *mut u8) = 8;
                        }
                        0xf => *((p + 0x3d) as *mut u8) = 8,
                        0xd => *((p + 0x3d) as *mut u8) = 6,
                        _ => {}
                    }
                }
                core::ptr::write_unaligned(vaga as *mut usize, 0);
            }
        }
        i += 1;
    }
}

/// allocates every person's 31-byte record fully fit; players: 1 in 10 of a recurring injury of the drawn type and 1 in
/// 100 of starting injured (0x62d140, 0x62d250); the totals in 0xde1f64
/// Original: `Injury::init_records` (0x62cf70, injury.cpp, thiscall).
///
/// [pt] { 0x62cf70 } (thiscall, ret 0, injury.cpp): `Injury::init_records()`: [this] = 31 bytes por pessoa de [0xae2364]
/// (0x944e46; sem memória, a caixa "Error" da linha 0x7d0 e nada). Cada registro: a data de hoje (0x417c10), forma e
/// condição 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1
/// (+0x14, +0x18), +0x1d 0xff, +0x1e 50. Jogador (+0x61): o tipo sorteado de 82 com peso (+8 da tabela de 13 bytes em
/// 0x9e7ad8) e 1 em 10 fica recorrente; 1 em 100 começa lesionado: categoria sorteada de 12, tipo pela 0x62d140
/// (gravidade −1) e a lesão nova 0x62d250. No fim os dois totais em 0xde1f64 ("%d players with ...").
pub(crate) unsafe extern "thiscall" fn registros_iniciais_de_lesao(this: usize) {
    let n = read_i32(0xae2364);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alocar((n as u32).wrapping_mul(31));
    core::ptr::write_unaligned(this as *mut usize, v);
    if v == 0 {
        aviso_em(0x9e7f28, 0x7d0);
        return;
    }
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let escolher: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_INJURY_CHOOSE_INJURY_TYPE);
    let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(FN_INJURY_ADD_INJURY);
    let (mut recorrentes, mut iniciais) = (0i32, 0i32);
    let mut r = v;
    let mut pessoa = read_usize(G_STAFF_TABLE);
    let mut i = 0i32;
    while i < read_i32(0xae2364) {
        let mut rascunho = [0u8; 8];
        copiar(r, rascunho.as_mut_ptr(), 0xae2c90);
        core::ptr::write_unaligned((r + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((r + 8) as *mut u16, 10000);
        core::ptr::write_unaligned((r + 0xa) as *mut u16, 10000);
        *((r + 0x12) as *mut u8) = 0xff;
        core::ptr::write_unaligned((r + 0xe) as *mut i32, -1);
        core::ptr::write_unaligned((r + 0x14) as *mut i32, -1);
        core::ptr::write_unaligned((r + 0x18) as *mut i32, -1);
        *((r + 0x1d) as *mut u8) = 0xff;
        *((r + 0x1e) as *mut u8) = 0x32;
        if read_usize(pessoa + 0x61) != 0 {
            let t = cpe1_sorteio(0x52) as u8;
            if read_usize((0x9e7ae0usize).wrapping_add((t as i8 as isize * 13) as usize)) != 0 && cpe1_sorteio(10) == 0 {
                recorrentes += 1;
                *((r + 0x12) as *mut u8) = t;
            }
            if cpe1_sorteio(100) == 0 { // 1 em 100 começa lesionado
                crate::comparacao::ramo("cpp_9 lesao inicial");
                // ponytail: a original empurra dwords da pilha com só o byte baixo gravado (o resto é lixo); a 0x62d140
                // e a 0x62d250 leem só o byte: aqui vão limpos
                let categoria = cpe1_sorteio(0xc) as u8;
                let tipo = escolher(this, categoria as u32, 0xffff_ffff, pessoa) as u8;
                lesionar(this, tipo as u32, 1, pessoa, 0, 1, -1, 0, 0);
                iniciais += 1;
            }
        }
        i += 1;
        pessoa = pessoa.wrapping_add(0x6e);
        r = r.wrapping_add(31);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    formatar(0xde1f64, 0x9e7f98, recorrentes);
    formatar(0xde1f64, 0x9e7f74, iniciais);
}

/// A nação em cuja lista dos jovens o clube entra (0x7acfc0): nenhuma para seleção (0x53b900), sem nação (+0x53),
/// nação sem +0x71 ou os dois clubes de 0x9d00a4/0x9d00a8. Nação com +0x11c: só com +0x80 até 2 e divisão (+0x57);
/// sem: com divisão.
pub(crate) unsafe fn cpp9_nacao_da_lista(clube: usize) -> usize {
    if cpp8_selecao(clube) {
        return 0;
    }
    let nacao = read_usize(clube + 0x53);
    if nacao == 0 || read_usize(nacao + 0x71) == 0 {
        return 0;
    }
    if read_u8(nacao + 0x11c) != 0 {
        crate::comparacao::ramo("cpp_9 nacao chave");
        if !(read_i16(clube + 0x80) <= 2 && read_usize(clube + 0x57) != 0) { // nação com +0x11c
            return 0;
        }
    } else if read_usize(clube + 0x57) == 0 {
        return 0;
    }
    let id = read_usize(clube);
    if id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) || id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) {
        return 0;
    }
    nacao
}


/// Comparador: lado a lado pelos registros de lesão (31 bytes × [0xae2364]), pelos clubes (as vagas) e pelo clube e
/// cargo (+0x39..+0x3d) de quem não é jogador nas vagas.
pub(crate) unsafe extern "thiscall" fn comparar_forma_inicial_dos_elencos(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1030);
    let base = read_usize(G_CLUB_TABLE);
    let mut regioes = vec![(read_usize(this), (read_i32(0xae2364).max(0) as usize) * 31), (base, (read_i32(0xae235c).max(0) as usize) * 0x245)];
    for i in 0..cpp9_clubes_sem_selecoes().max(0) as usize {
        for s in 0..50usize {
            let p = read_usize(base + i * 0x245 + 0xd7 + 4 * s);
            if p != 0 && read_i32(p) < read_i32(0xae2364) && read_usize(p + 0x61) == 0 {
                regioes.push((p + 0x39, 5));
            }
        }
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this), || forma_inicial_dos_elencos(this));
    PLACAR.registrar(igual, || format!("forma_inicial_dos_elencos: memória difere ({} regiões)", regioes.len()));
}

/// Comparador: a original aloca, sorteia e lesiona (0x62d250): roda só ela e confere o que a 0x62d250 não muda: o
/// vetor alocado e o +0x1e = 50 de todos os registros.
pub(crate) unsafe extern "thiscall" fn comparar_registros_iniciais_de_lesao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1031)(this);
    let v = read_usize(this);
    if v == 0 {
        return;
    }
    let errado = (0..read_i32(0xae2364).max(0) as usize).find(|&k| read_u8(v + 31 * k + 0x1e) != 0x32);
    PLACAR.registrar(errado.is_none(), || format!("registros_iniciais_de_lesao: registro {:?} sem o +0x1e = 50", errado));
}


// ---------- 249a leva: as lesões: sortear a lesão recorrente, recomeçar o histórico de lesões e reiniciar o físico do jogador (injury.cpp) ----------



/// O registro de 31 bytes do físico da pessoa `id` em [this] (+0x12 o tipo da lesão recorrente, 0xff sem; +0x13 o
/// nível; +0x14/+0x18 o começo e o último deslocamento do histórico de lesões).
pub(crate) unsafe fn cpm5_registro(this: usize, id: usize) -> usize {
    read_usize(this).wrapping_add(id.wrapping_mul(31))
}


/// O que a 0x62e500 decide depois de sortear.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Cpm5Recorrente {
    Nada,
    Marcada,
    Limpar(usize),
    Tratar(usize),
}

/// O plano da 0x62e500 (os sorteios e as marcas do registro; o resto fica para quem chama): 1 em 20 (sorteio 0x14) e
/// a lesão `tipo` podendo voltar ([0x9e7ae0 + 13 × tipo]), o registro sem recorrente fica com ela (+0x12 = tipo,
/// +0x13 = 0). Dos registros com recorrente (+0x12 ≠ 0xff), o de maior nota (sorteio 0x65 − 5 × nível − pessoa +0x2d
/// do jogador; 100 sem jogador, bytes com sinal), parando no primeiro com 95 ou mais. Achado: jogador de clube (+0x39)
/// de reputação (+0x80) acima de 4250, clube sem humano (0x600a50) e 1 em 3 (sorteio 3): tratar; senão limpar a marca
/// dele.
pub(crate) unsafe fn cpm5_plano(this: usize, registro: usize, lesao: *const u8) -> Cpm5Recorrente {
    if read_u8(registro + 0x12) != 0xff || read_usize((0x9e7ae0isize + *lesao as i8 as isize * 13) as usize) == 0 {
        return Cpm5Recorrente::Nada;
    }
    let s = cpe1_sorteio(0x14);
    if s != 0 { // 1 em 20
        return Cpm5Recorrente::Nada;
    }
    crate::comparacao::ramo("cpm_5 lesao recorrente");
    *((registro + 0x13) as *mut u8) = s as u8;
    *((registro + 0x12) as *mut u8) = *lesao;
    if *(G_STAFF_COUNT as *const i32) <= 0 {
        return Cpm5Recorrente::Marcada;
    }
    let (mut melhor, mut escolhido, mut pessoa) = (0i8, 0usize, 0usize);
    let mut r = read_usize(this);
    let mut i = 0i32;
    while i < *(G_STAFF_COUNT as *const i32) {
        if read_u8(r + 0x12) != 0xff {
            let p = read_usize(G_STAFF_TABLE).wrapping_add(i as usize * 0x6e);
            let v = cpe1_sorteio(0x65) as u8;
            let jogador = read_usize(p + 0x61);
            let nota = if jogador == 0 { 0x64 } else { v.wrapping_add(read_u8(r + 0x13).wrapping_mul(0xfb)).wrapping_sub(read_u8(jogador + 0x2d)) };
            if nota as i8 > melhor {
                (melhor, escolhido, pessoa) = (nota as i8, r, p);
            }
            if melhor >= 0x5f { // forte o bastante: para
                crate::comparacao::ramo("cpm_5 recorrente forte");
                break;
            }
        }
        i += 1;
        r += 0x1f;
    }
    if escolhido == 0 {
        return Cpm5Recorrente::Marcada;
    }
    let clube = read_usize(pessoa + 0x39);
    if read_usize(pessoa + 0x61) != 0 && clube != 0 && core::ptr::read_unaligned((clube + 0x80) as *const i16) > 0x109a {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if humano(clube, 0, 1, 0, 0) == 0 && cpe1_sorteio(3) == 0 && pessoa != 0 {
            return Cpm5Recorrente::Tratar(pessoa);
        }
    }
    Cpm5Recorrente::Limpar(escolhido)
}

/// with 1 in 20, a recurring-capable injury becomes recurring in the record (+0x12 type, +0x13 = 0); among the
/// recurring records, the best score (roll 0x65 − 5 × level − proneness, stops at 95): a player at a big club
/// (reputation > 4250) without a human and 1 in 3 gets a new injury 0x4d/0x4e/0x4f (0x62d250); otherwise his mark is
/// cleared; in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later
/// original reads: the career left be54a138); the copy stays in the code and does not run
/// Original: `Injury::roll_recurring_injury` (0x62e500, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62e500 } (thiscall, ret 8, injury.cpp): `Injury::roll_recurring_injury(registro, lesao)`: o plano
/// (`cpm5_plano`); tratar: a lesão nova da pessoa (0x62d250 com 0x4e, 0x4f ou 0x4d pelo tipo recorrente do registro
/// dela, 0xff fora das pessoas ou sem jogador; o tipo vai no byte baixo do argumento `registro`, como a original);
/// limpar: o registro escolhido sem recorrente (+0x12 = 0xff, +0x13 = 0).
pub(crate) unsafe extern "thiscall" fn sortear_lesao_recorrente(this: usize, registro: usize, lesao: *const u8) {
    match cpm5_plano(this, registro, lesao) {
        Cpm5Recorrente::Tratar(p) => {
            let id = read_usize(p) as i32;
            let tipo = if id < *(G_STAFF_COUNT as *const i32) && read_usize(p + 0x61) != 0 {
                read_u8(cpm5_registro(this, id as usize) + 0x12)
            } else {
                0xff
            };
            let nova: u8 = match tipo {
                0x0b | 0x11 | 0x17 | 0x1e | 0x23 | 0x29 => 0x4e,
                0x0c | 0x0d | 0x12 | 0x14 | 0x19..=0x1c | 0x1f | 0x24 | 0x25 | 0x2c | 0x2d | 0x3b => 0x4f,
                _ => 0x4d,
            };
            let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(FN_INJURY_ADD_INJURY);
            lesionar(this, (registro as u32 & 0xffff_ff00) | nova as u32, 0, p, 0, 1, -1, 0, 0);
        }
        Cpm5Recorrente::Limpar(r) => {
            *((r + 0x12) as *mut u8) = 0xff;
            *((r + 0x13) as *mut u8) = 0;
        }
        _ => {}
    }
}

/// the last offset of the person's injury history (+0x18 of the 31-byte record) = −1 (0x58f110)
/// Original: `Injury::reset_player_history` (0x631f90, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x631f90 } (thiscall, ret 4, injury.cpp): `Injury::reset_player_history(pessoa)`: o histórico de lesões da pessoa
/// (registro de 31 bytes do id em [this]) fica sem o último (0x58f110 sobre [this+0xc]: +0x18 = −1). Devolve o que a
/// 0x58f110 devolve. Pessoa nula: a original (caixa fatal).
pub(crate) unsafe extern "thiscall" fn recomecar_historico_de_lesoes(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(938)(this, pessoa); // caixa fatal: só a original
    }
    let r = cpm5_registro(this, read_usize(pessoa));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    sem_ultimo(read_usize(this + 0xc), r + 0x14, r + 0x18)
}

/// removes the id items from list +0x28, ends the injury (0x62e700), rewinds the history (0x631f90) and the 31-byte
/// record (today, condition and form 10000, wear 0, no injury nor recurrence); bit 0 of +0xc of the 79-byte record is
/// cleared
/// Original: `Injury::reset_player` (0x62bfa0, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62bfa0 } (thiscall, ret 4, injury.cpp): `Injury::reset_player(id)`: tira da lista [this+0x28] (do começo: o
/// cursor +0xa volta a +2) cada item do id (0x68d1f0); com lesão (+0xe ≠ −1), termina (0x62e700, sem notícia);
/// recomeça o histórico (0x631f90); o registro de 31 bytes recomeça: data de hoje (0x417c10 de [0xae2c90]), condição
/// e forma 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1 (+0x14,
/// +0x18), +0x1d 0xff. O bit 0 do +0xc do registro de 79 bytes ([0xae2d0c]) cai. Devolve o endereço desse byte.
pub(crate) unsafe extern "thiscall" fn reiniciar_fisico_do_jogador(this: usize, id: u32) -> usize {
    crate::comparacao::ramo("cpm_5 reiniciar jogador");
    if read_usize(this + 0x28) != 0 {
        let l = read_usize(this + 0x28);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
        let mut e = proximo(read_usize(this + 0x28));
        while e != 0 {
            if read_usize(e) as u32 == id {
                tirar(read_usize(this + 0x28), e);
            }
            e = proximo(read_usize(this + 0x28));
        }
    }
    let r = cpm5_registro(this, id as usize);
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e));
    if read_usize(r + 0xe) as i32 != -1 {
        let terminar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_INJURY_END_PLAYER_INJURY);
        terminar(this, pessoa, 0);
    }
    let recomecar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_RESET_PLAYER_HISTORY);
    recomecar(this, read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e)));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut rascunho = [0u8; 8];
    copiar(r, rascunho.as_mut_ptr(), 0xae2c90); // a data de hoje
    core::ptr::write_unaligned((r + 8) as *mut u16, 10000);
    core::ptr::write_unaligned((r + 0xa) as *mut u16, 10000);
    core::ptr::write_unaligned((r + 0xc) as *mut i16, 0); // sem desgaste
    core::ptr::write_unaligned((r + 0xe) as *mut i32, -1);
    *((r + 0x12) as *mut u8) = 0xff;
    core::ptr::write_unaligned((r + 0x14) as *mut i32, -1);
    core::ptr::write_unaligned((r + 0x18) as *mut i32, -1);
    *((r + 0x1d) as *mut u8) = 0xff;
    let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    *(g as *mut u8) &= 0xfe;
    g
}

/// Comparador: a 0x62d250 (tratar) não pode rodar duas vezes: o plano antes, com o estado do sorteio guardado (sem
/// contar nem rastrear) e as marcas do registro devolvidas; a original roda. Confere as marcas do registro e do
/// escolhido e, fora do tratar, o estado do sorteio igual ao do plano (os mesmos sorteios).
pub(crate) unsafe extern "thiscall" fn comparar_sortear_lesao_recorrente(this: usize, registro: usize, lesao: *const u8) {
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let marcas = (read_u8(registro + 0x12), read_u8(registro + 0x13));
    let plano = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cpm5_plano(this, registro, lesao)));
    let depois_do_plano = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    *((registro + 0x12) as *mut u8) = marcas.0;
    *((registro + 0x13) as *mut u8) = marcas.1;
    let tipo = *lesao;
    orig::<unsafe extern "thiscall" fn(usize, usize, *const u8)>(937)(this, registro, lesao);
    let agora = (read_u8(registro + 0x12), read_u8(registro + 0x13));
    let mesmo_sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois_do_plano;
    let ok = match plano {
        Cpm5Recorrente::Nada => agora == marcas && mesmo_sorteio,
        Cpm5Recorrente::Marcada => agora == (tipo, 0) && mesmo_sorteio,
        Cpm5Recorrente::Limpar(r) => {
            (read_u8(r + 0x12), read_u8(r + 0x13)) == (0xff, 0) && (r == registro || agora == (tipo, 0)) && mesmo_sorteio
        }
        Cpm5Recorrente::Tratar(_) => true, // a 0x62d250 sorteia e grava o resto
    };
    PLACAR.registrar(ok, || {
        format!("sortear_lesao_recorrente(tipo {}) plano {:x?}, marcas {:?} -> {:?}, mesmo sorteio={}", tipo, plano, marcas, agora, mesmo_sorteio)
    });
}

/// Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
pub(crate) unsafe extern "thiscall" fn comparar_recomecar_historico_de_lesoes(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(938);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = cpm5_registro(this, read_usize(pessoa));
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 0x14, 8)], || o(this, pessoa), || recomecar_historico_de_lesoes(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("recomecar_historico_de_lesoes({}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
    a
}

/// Comparador: a 0x68d1f0 libera o item tirado e a 0x62e700 grava o histórico no arquivo: o plano antes (os itens
/// que ficam, o byte do registro de 79 bytes, os bytes do registro que não mudam); a original roda; confere.
pub(crate) unsafe extern "thiscall" fn comparar_reiniciar_fisico_do_jogador(this: usize, id: u32) -> usize {
    let l = read_usize(this + 0x28);
    let esperado: Vec<usize> = if l != 0 { cpd1_itens(l).into_iter().filter(|&e| read_usize(e) as u32 != id).collect() } else { Vec::new() };
    let r = cpm5_registro(this, id as usize);
    let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    let (byte, nivel, b1c) = (read_u8(g) & 0xfe, read_u8(r + 0x13), read_u8(r + 0x1c));
    let ret = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(939)(this, id);
    let itens = if l != 0 { cpd1_itens(l) } else { Vec::new() };
    let w = |o: usize| core::ptr::read_unaligned((r + o) as *const i16);
    let dd = |o: usize| read_usize(r + o) as i32;
    let ok = itens == esperado && ret == g && read_u8(g) == byte
        && read_usize(r) == read_usize(G_CURRENT_DATE) && read_usize(r + 4) == read_usize(G_CURRENT_DATE_LEAP)
        && w(8) == 10000 && w(0xa) == 10000 && w(0xc) == 0 && dd(0xe) == -1 && read_u8(r + 0x12) == 0xff
        && read_u8(r + 0x13) == nivel && dd(0x14) == -1 && dd(0x18) == -1 && read_u8(r + 0x1c) == b1c && read_u8(r + 0x1d) == 0xff;
    PLACAR.registrar(ok, || {
        let v: Vec<u8> = (0..31).map(|o| read_u8(r + o)).collect();
        format!("reiniciar_fisico_do_jogador({}) itens {} (esperava {}), byte {:#x} (esperava {:#x}), registro {:02x?}", id, itens.len(), esperado.len(), read_u8(g), byte, v)
    });
    ret
}

// ---------- 176a leva: a recuperacao da lesao, a pessoa na lista de transferencias e a punicao da disciplina ----------

/// the recovery day (below −1000 in x87 with −0.0035 and 2.5; otherwise the pace or a draw)
/// Original: `recover_injury` (0x62bd10, injury.cpp, thiscall).
///
/// [pt] { 0x62bd10 } (thiscall, ret 0xc, injury.cpp): o dia de recuperação da pessoa `p` no registro de
/// 31 bytes (a partir de [this]): +0xa recebe `marca`; a palavra +0xc, abaixo de −1000, sobe
/// trunc((v × −0.0035 − 2.5) × `ritmo`); senão, abaixo de 1200 − (ficha +0x44 + ficha +0x3c +
/// pessoa +0x5b, 16 bits), sobe `ritmo`; senão, abaixo de 1250, sobe sorteio(`ritmo`) + 1.
/// Pessoa nula ou id fora de [0xae2364]: nada.
pub(crate) unsafe extern "thiscall" fn recuperar_lesao(t: usize, p: *const i32, marca: i16, ritmo: i16) {
    if p.is_null() || *p >= *(G_STAFF_COUNT as *const i32) {
        return;
    }
    let r = (*(t as *const usize)).wrapping_add((*p as usize).wrapping_mul(31));
    core::ptr::write_unaligned((r + 0xa) as *mut i16, marca);
    let e = (r + 0xc) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        let x = (v as f64 * f64_em(0x96bac0) - f64_em(0x968e18)) * ritmo as f64;
        core::ptr::write_unaligned(e, (x as i32).wrapping_add(v as i32) as i16);
        return;
    }
    let ficha = core::ptr::read_unaligned((p as usize + 0x61) as *const usize);
    let b = |a: usize| *(a as *const i8) as i16;
    let soma = b(ficha + 0x44).wrapping_add(b(ficha + 0x3c)).wrapping_add(b(p as usize + 0x5b));
    if (v as i32) < 0x4b0 - soma as i32 {
        core::ptr::write_unaligned(e, v.wrapping_add(ritmo));
    } else if v < 0x4e2 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        let s = sorteio(ritmo as i32).wrapping_add(1) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_add(s));
    }
}



pub(crate) unsafe extern "thiscall" fn comparar_recuperar_lesao(t: usize, p: *const i32, marca: i16, ritmo: i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, i16, i16)>(400);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !p.is_null() && *p < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((*p as usize).wrapping_mul(31)).wrapping_add(0xa), 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, p, marca, ritmo), || crate::aleatorio::sem_contar(|| recuperar_lesao(t, p, marca, ritmo)));
    PLACAR.registrar(igual, || format!("recuperar_lesao({:#x}, {:p}, {}, {}) {:x?} x {:x?}", t, p, marca, ritmo, mo, mr));
}


comparar_valor!(comparar_punicao_da_tabela, punicao_da_tabela, 402, (linha: i32, coluna: i8, tipo: i8) -> i32);

/// fitness × condition / 10,000 (minimum 100, maximum 9000 while recovering), or as a percentage (maximum 100)
/// Original: `Injury::get_condition` (0x62c590, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62c590 } (thiscall, ret 8, injury.cpp): `Injury::get_condition(jogador, em_porcento)`: a condição do jogador:
/// forma (+0xa) × condição de jogo (+8) / 10.000 do registro de 31 bytes [this] do id, no mínimo 100 e no máximo 9000
/// enquanto ele se recupera (0x62c400 = 1); em porcentagem (× 0x96bac8 + 0x968968, _ftol), no máximo 100. Pessoa fora
/// dos jogadores ([0xae2364]): 100 ou 10.000. Jogador nulo: 0.
pub(crate) unsafe extern "thiscall" fn condicao_do_jogador(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    if jogador == 0 {
        return 0;
    }
    let id = read_usize(jogador) as i32;
    if id >= read_usize(G_STAFF_COUNT) as i32 {
        return if em_porcento != 0 { 100 } else { 10_000 };
    }
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let recuperando = estado(this, jogador) == 1;
    let r = read_usize(this).wrapping_add((id as usize).wrapping_mul(31));
    let v = *((r + 0xa) as *const i16) as i32 * *((r + 8) as *const i16) as i32 / 10_000;
    let mut c = if recuperando && v as i16 >= 9000 {
        9000 // o teto de quem se recupera
    } else if v as i16 > 100 {
        v
    } else {
        100
    };
    if em_porcento != 0 {
        c = ftol(c as i16 as f64 * f32_em(0x96bac8) as f64 + f64_em(0x968968));
        if c as i16 > 100 {
            c = 100;
        }
    }
    c as u16
}

/// Comparador: pelo valor (a 0x62c400 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_condicao_do_jogador(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(874)(this, jogador, em_porcento);
    let n = condicao_do_jogador(this, jogador, em_porcento);
    PLACAR.registrar(r == n, || format!("condicao_do_jogador({:x}, {}) original={} rust={}", jogador, em_porcento, r, n));
    r
}

/// updates the day fitness (0x62c680) and returns the match condition (out of 10000, or as a percentage)
/// Original: `Injury::get_match_condition` (0x62c510, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62c510 } (thiscall, ret 8, injury.cpp): `Injury::get_match_condition(jogador, em_porcento)`: sem jogador, 0; id
/// fora das pessoas ([0xae2364]): 10000 (100 em porcento); senão atualiza o físico do dia (0x62c680, uma vez por dia)
/// e devolve a condição (registro de 31 bytes em [this], +0xa), em porcento × 0,01f truncado. Só o ax é o retorno.
pub(crate) unsafe extern "thiscall" fn condicao_de_jogo_do_jogador(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    if jogador == 0 {
        return 0;
    }
    let id = *(jogador as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return if em_porcento != 0 { 100 } else { 10000 };
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
    atualizar(this, jogador);
    let c = core::ptr::read_unaligned((*(this as *const usize)).wrapping_add((*(jogador as *const i32) as usize).wrapping_mul(31)).wrapping_add(0xa) as *const i16);
    if em_porcento != 0 {
        return ftol(c as f64 * *(0x96bac8 as *const f32) as f64) as u16; // em porcento
    }
    c as u16
}

/// stores the player's training load in the fitness record
/// Original: `Injury::set_training_load` (0x62c660, injury.cpp, thiscall).
///
/// [pt] { 0x62c660 } (thiscall, ret 8, injury.cpp): grava a carga de treino `valor` do jogador `*id`.
pub(crate) unsafe extern "thiscall" fn gravar_carga_de_treino(lesoes: usize, id: *const i32, valor: u8) {
    *byte_da_carga(lesoes, id) = valor;
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_carga_de_treino(lesoes: usize, id: *const i32, valor: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, u8)>(438);
    let a = byte_da_carga(lesoes, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(lesoes, id, valor), || gravar_carga_de_treino(lesoes, id, valor));
    PLACAR.registrar(igual, || format!("gravar_carga_de_treino({}, {}) {:x?} x {:x?}", *id, valor, mo, mr));
}

/// once a day: the days since the last update become free days, recovery days, stopped days (injury, ending it through
/// 0x62e700) or holiday days (club out of season, 0x669bd0); fitness (+8) recovers 1000 − 12 × (+0x1e − natural) per
/// free day, loses 66 per stopped day and 55 per holiday day, capped by natural fitness and with shrunk bands above
/// 7000; match condition (+0xa) rises with free and recovery days; stores today's date
/// Original: `Injury::update_player_fitness` (0x62c680, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62c680 } (thiscall, ret 4, injury.cpp, quadro SEH só das estáticas locais): `Injury::update_player_fitness(
/// pessoa)`: uma vez por dia (a data +0 do registro de 31 bytes da pessoa em [this] já é hoje: nada), pela condição de
/// jogo (0x62c510), pelo estado da lesão (0x62c400), pelo dia das lesões (0x62b7b0) e pelas notícias da volta (0x62e010):
/// toda pessoa que joga, todo dia em que alguém pergunta. T = dias desde a última atualização (0x54cc30). Lesionada
/// (+0xe na tabela de 0x30 bytes em [this+4]): pelos dias desde o começo (+0x20) contra +0x2a e +0x28, os dias parados
/// (perda), em recuperação ou livres, com o fim da lesão (0x62e700) quando passa de +0x28. Sem lesão: dias livres, salvo
/// na seleção com data futura (0x76d780/0x76e040) ou no clube fora da temporada (0x669bd0, guardado em 0xb63d2c/0xb63d30
/// com as datas 0xb63d18/0xb63d20): férias (o byte +0xa do registro de 79 bytes sobe com sorteio(+0x5b / 3 + 2) zero, até
/// 50) e a volta da pré-temporada (15 dias, ou o meio de um sorteio(2) com forma × condição abaixo de 80%). A forma (+8):
/// recuperação de 1000 − 12 × (+0x1e − natural (+0x34 da ficha), 0..100) por dia livre, perda de 66 por dia parado, teto
/// pela forma natural, trechos acima de 7000/9000/9850 encolhidos (0x96af30, 0x96bad0, 0x968968, 0x968918, 0x967880) e −55
/// por dia de férias; a condição de jogo (+0xa) sobe 500 × dias livres e 250 × em recuperação pelo fator do cansaço
/// (+0xc).
/// Grava as duas e a data de hoje.
pub(crate) unsafe extern "thiscall" fn fisico_do_dia(this: usize, pessoa: usize) {
    cpw8_estatica(1, 0xb63d08, 0x62ce20);
    cpw8_estatica(2, 0xb63d18, 0x62ce10);
    cpw8_estatica(4, 0xb63d20, 0x62ce00);
    if pessoa == 0 {
        return;
    }
    let id = read_i32(pessoa);
    if id >= read_i32(0xae2364) {
        return;
    }
    let ficha = read_usize(pessoa + 0x61);
    if ficha == 0 {
        return;
    }
    let rec = read_usize(this).wrapping_add((id as u32).wrapping_mul(31) as usize);
    if read_u16(rec + 2) == read_u16(0xae2c92) && read_i16(rec) == read_i16(G_CURRENT_DATE) {
        return; // já atualizado hoje
    }
    let t = cpw8_dias_desde(rec);
    let atual = read_i32(rec + 0xe);
    if atual > read_i32(this + 8) || atual < -1 {
        core::ptr::write_unaligned((rec + 0xe) as *mut i32, -1);
    }
    // os dias livres (l38), em recuperação (l3c), parados (l34) e de férias (l30)
    let (mut l3c, mut l38, mut l34, mut l30) = (0i32, 0i32, 0i32, 0i32);
    let i = read_i32(rec + 0xe);
    if i != -1 {
        let inj = read_usize(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
        let e = cpw8_dias_desde(inj + 0x20);
        let s = read_i16(inj + 0x2a) as i32;
        if e < s {
            l34 = t;
        } else {
            let f = read_i16(inj + 0x28) as i32;
            if e < f {
                if t < s.wrapping_add(e) {
                    l3c = e - s;
                    l34 = t - (e - s);
                } else {
                    l3c = t;
                }
            } else if t > e - s {
                l38 = t - f;
                l3c = (e - s) - l38;
                l34 = f - l3c;
                cpw8_terminar_lesao(this, pessoa);
            } else {
                l38 = t;
                cpw8_terminar_lesao(this, pessoa);
            }
        }
    } else {
        l38 = t;
        let selecao = read_usize(pessoa + 0x24);
        let mut ocupado = false;
        if selecao != 0 {
            let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_SQUAD_SLOT);
            let k = classe(selecao, (*((pessoa + 0x5e) as *const u8) & 8) as u32);
            let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_SQUAD_SLOT_RECORD);
            // ponytail: a original empurra o argumento com o al por cima (o resto é o ponteiro da pessoa); só o byte conta
            let d = campo(read_usize(G_NATIONAL_TEAMS), read_usize(selecao + 0x53), (pessoa as u32 & 0xffff_ff00) | k as u32);
            let (ano, hoje) = (read_u16(d + 2), read_u16(0xae2c92));
            ocupado = if ano > hoje {
                true
            } else if ano < hoje {
                false
            } else {
                read_i16(d) > read_i16(G_CURRENT_DATE)
            };
        }
        let clube = read_usize(pessoa + 0x39);
        if clube != 0 && !ocupado {
            if clube != read_usize(G_FITNESS_UPDATE_CACHED_CLUB) || read_u16(0xb63d0a) != read_u16(0xae2c92) || read_i16(G_FITNESS_UPDATE_CACHE_DATE) != read_i16(G_CURRENT_DATE) {
                let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
                let r = na_temporada(clube, 0xb63d18, 0xb63d20);
                *(G_FITNESS_UPDATE_CACHED_SEASON_FLAG as *mut u32) = (r == 0) as u32;
                *(G_FITNESS_UPDATE_CACHED_CLUB as *mut usize) = read_usize(pessoa + 0x39);
                cpw8_copiar_data(0xb63d08, 0xae2c90);
            }
            if read_i32(0xb63d30) != 0 {
                l38 = 0;
                let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
                let u6 = dias_ate(0xae2c90, read_usize(G_FITNESS_UPDATE_SEASON_END_DATE) as u32, read_usize(G_FITNESS_UPDATE_SEASON_END_DATE_LEAP) as u32) as u16;
                let u7 = dias_ate(0xb63d18, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) as u16;
                if u6 < u7 {
                    let lim = if (u6 as i32) < t { u6 as i32 } else { t };
                    l30 = lim;
                    l3c = t - lim;
                    for _ in 0..lim {
                        let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(pessoa) as u32).wrapping_mul(0x4f) as usize) + 0xa;
                        if *(g as *const u8) >= 0x32 {
                            break;
                        }
                        if cpe1_sorteio(read_i8(pessoa + 0x5b) as i32 / 3 + 2) == 0 {
                            *(g as *mut u8) = (*(g as *const u8)).wrapping_add(1);
                        }
                    }
                } else {
                    let mut x = u6 as i32 - (u7 as i32 + u6 as i32) / 2;
                    if x < 0 {
                        x = 0;
                    }
                    if t < x {
                        x = t;
                    }
                    l3c = x;
                    l34 = t - x;
                }
                if u7 < 0xf {
                    l38 = 0xf - u7 as i32;
                    l3c -= l38;
                    if l3c < 0 {
                        l3c = 0;
                    }
                } else {
                    let (f, mc) = (read_i16(rec + 8) as i32, read_i16(rec + 0xa) as i32);
                    if mc.wrapping_mul(f) / 10000 < 8000 { // cpw8: a forma abaixo de 80% na pré-temporada
                        crate::comparacao::ramo("cpw_8 sorteio da forma");
                        let r = cpe1_sorteio(2);
                        l38 = (r + l3c) / 2;
                        l3c -= l38;
                    }
                }
            }
        }
    }
    // a forma
    let f = read_i16(rec + 8) as i32;
    let natural = read_i8(ficha + 0x34);
    let mut v = read_i8(rec + 0x1e).wrapping_sub(natural);
    if v < 0 {
        v = 0;
    } else if v > 100 {
        v = 100;
    }
    let s = (1000 - 12 * v as i32) as i16 as i32; // cpw8: a recuperação por dia livre
    let metade = s / 2;
    let n = natural as i32;
    let piso = ((n * 25 / 20 + 15) * 100) as i16 as i32;
    if l34 != 0 {
        crate::comparacao::ramo("cpw_8 perda");
    }
    let mut x = f.wrapping_sub(66i32.wrapping_mul(l34)); // cpw8: a perda por dia parado
    if x < piso {
        x = 4000;
    }
    let m = n * 16 / 20 + 0x54;
    let mut teto = m.wrapping_mul(100);
    if (teto as i16 as i32) < x {
        teto = x;
    }
    x = x.wrapping_add((metade as i16 as i32).wrapping_mul(l3c));
    let teto16 = teto as i16 as i32;
    if x > teto16 {
        x = teto16;
    }
    if l38 != 0 {
        crate::comparacao::ramo("cpw_8 recuperacao");
    }
    x = x.wrapping_add(s.wrapping_mul(l38));
    if f > 0x267a {
        x = cpw8_trecho(x, f, 0x967880);
    } else if f > 0x2328 {
        x = cpw8_trecho(x, f, 0x968918);
        if x > 0x267a {
            x = cpw8_trecho(x, 0x267a, 0x968968);
        }
    } else if x > 0x1b58 {
        x = cpw8_trecho(x, 0x1b58, 0x96af30);
        if x > 0x2328 {
            x = cpw8_trecho(x, 0x2328, 0x96bad0);
        }
        if x > 0x267a {
            x = cpw8_trecho(x, 0x267a, 0x968968);
        }
    }
    if x > 10000 {
        x = 10000;
    }
    let mut forma = x.wrapping_sub(55i32.wrapping_mul(l30));
    if forma < piso {
        forma = 5000;
    }
    let mut e2 = m;
    if (m as i16 as i32) < forma {
        e2 = forma;
    }
    let e16 = e2 as i16 as i32;
    if forma > e16 {
        forma = e16;
    }
    // a condição de jogo, pelo fator do cansaço (+0xc)
    let cansaco = read_i16(rec + 0xc);
    let mc = read_i16(rec + 0xa) as i32;
    let teto_k = f64_em(0x9689d0);
    let k = if cansaco > 1000 {
        let kv = f64_em(0x968dc0) - cansaco as f64 * f64_em(0x96a608);
        if !(kv > teto_k) {
            kv
        } else if l3c > 0 {
            teto_k
        } else {
            kv
        }
    } else if l3c > 0 {
        teto_k
    } else {
        f64_em(0x967890)
    };
    let r1 = ftol(l38.wrapping_mul(500) as f64 * k);
    let r2 = ftol(l3c.wrapping_mul(250) as f64 * k);
    let mut cond = r2.wrapping_add(mc).wrapping_add(r1);
    if cond > 10000 {
        cond = 10000;
    }
    core::ptr::write_unaligned((rec + 8) as *mut i16, forma as i16);
    core::ptr::write_unaligned((rec + 0xa) as *mut i16, cond as i16);
    cpw8_copiar_data(rec, 0xae2c90);
}

/// Comparador lado a lado: o registro de 31 bytes da pessoa, o byte +0xa do registro de 79 bytes, as estáticas e o
/// guardado do clube (0xb63d08..0xb63d34) e o sorteio. A cópia roda em ensaio, sem o fim da lesão (0x62e700): quando
/// ela chega lá, fica a original sozinha (já rodou primeiro). Na primeira vez (estáticas por criar, com atexit) e sem
/// registro (pessoa nula, id fora da tabela, sem ficha): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_fisico_do_dia(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1290);
    if *(G_FITNESS_UPDATE_STATIC_INIT_FLAGS as *const u8) & 7 != 7 || pessoa == 0 || read_i32(pessoa) < 0 || read_i32(pessoa) >= read_i32(0xae2364) || read_usize(pessoa + 0x61) == 0 {
        return o(this, pessoa); // ponytail: as estáticas e o atexit da primeira vez não podem rodar duas vezes
    }
    let id = read_i32(pessoa) as usize;
    let rec = read_usize(this).wrapping_add(id.wrapping_mul(31));
    let geral = read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f));
    let e = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(rec, 31), (geral + 0xa, 1), (0xb63d08, 0x2c), e[0], e[1]];
    CPW8_TERMINOU = false;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || {
        CPW8_ENSAIO = true;
        crate::aleatorio::sem_contar(|| fisico_do_dia(this, pessoa));
        CPW8_ENSAIO = false;
    });
    if CPW8_TERMINOU {
        crate::comparacao::ramo("cpw_8 fim da lesao so na original");
        return;
    }
    PLACAR.registrar(igual, || format!("fisico_do_dia(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// the day's wear on word +0xc (age, draws of 25 and 13, goalkeeper cap)
/// Original: `daily_wear` (0x62c2f0, injury.cpp, thiscall).
///
/// [pt] { 0x62c2f0 } (thiscall, ret 4, injury.cpp): o desgaste do dia na palavra +0xc do registro de 31
/// bytes da pessoa `id` (a partir de [this]): abaixo de −1000 desce 1; abaixo de 19 anos (+0x18 da
/// pessoa em [0xae23bc]) e negativo, zera; senão, de −500 para cima e com idade ≥ sorteio(25), desce
/// 13, e nos outros casos desce sorteio(13); goleiro (posição +0xf da ficha = 20) fica até 1050.
/// Id fora de [0xae2364]: nada.
pub(crate) unsafe extern "thiscall" fn desgaste_diario(t: usize, id: i32) {
    if id >= *(G_STAFF_COUNT as *const i32) {
        return;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let e = (*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        core::ptr::write_unaligned(e, v.wrapping_sub(1));
    }
    let p = (*(G_STAFF_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(0x6e));
    let idade = *((p + 0x18) as *const i8);
    if idade < 0x13 && core::ptr::read_unaligned(e) < 0 {
        core::ptr::write_unaligned(e, 0);
    } else if core::ptr::read_unaligned(e) >= -500 && idade as i32 >= sorteio(0x19) {
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xd));
    } else {
        let r = sorteio(0xd) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(r));
    }
    let ficha = core::ptr::read_unaligned((p + 0x61) as *const usize);
    if ficha != 0 && *((ficha + 0xf) as *const u8) == 0x14 && core::ptr::read_unaligned(e) >= 0x41a {
        core::ptr::write_unaligned(e, 0x41a);
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_desgaste_diario(t: usize, id: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(396);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if id < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc), 2));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, id), || crate::aleatorio::sem_contar(|| desgaste_diario(t, id)));
    PLACAR.registrar(igual, || format!("desgaste_diario({:#x}, {}) {:x?} x {:x?}", t, id, mo, mr));
}

/// draws the injury type for the body part and severity, keeping the current injury or preferring the recurring one
/// Original: `Injury::choose_injury_type` (0x62d140, injury.cpp, thiscall).
///
/// [pt] sorteia o tipo de lesão da parte do corpo e da gravidade, mantendo a lesão atual ou preferindo a recorrente
pub(crate) unsafe extern "thiscall" fn escolher_tipo_de_lesao(lesoes: usize, categoria: i8, gravidade: i8, jogador: usize) -> u8 {
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mut melhor: i8 = 0x64;
    let mut escolhido: u8 = 0xff;
    let mut recorrente: u8 = 0xff;
    let a = atual(lesoes, jogador);
    if a != 0 {
        let t = *(a as *const u8);
        if !matches!(t, 0 | 1 | 2 | 3 | 4 | 0x4c | 0x4d | 0x4e | 0x4f | 0x50 | 0x51) {
            return t;
        }
    }
    let tipo = |k: i8, o: usize| *((0x9e7ad8 + o) as *const u8).offset(k as isize * 13);
    if jogador != 0 {
        let id = *(jogador as *const u32);
        let r = *((*(lesoes as *const u32)).wrapping_add(id.wrapping_mul(31)).wrapping_add(0x12) as usize as *const u8);
        if r != 0xff && tipo(r as i8, 1) as i8 == categoria {
            recorrente = r;
        }
    }
    let c = categoria as isize * 3;
    let mut bl = *((0x9e7f05 + c) as *const i8);
    let ultimo = (0x9e7f06 + c) as *const i8;
    if bl > *ultimo {
        return escolhido;
    }
    loop {
        let g = tipo(bl, 0xc) as i8;
        let serve = (gravidade == -1 || g == gravidade || g == -1) && gravidade != 3;
        if serve {
            let r = if bl as u8 == recorrente {
                (sorteio(3) as u8).wrapping_sub(1) as i8
            } else {
                sorteio(tipo(bl, 2) as i8 as i32) as i8
            };
            if r < melhor {
                melhor = r;
                escolhido = bl as u8;
            }
        }
        bl = bl.wrapping_add(1);
        if bl > *ultimo {
            return escolhido;
        }
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_escolher_tipo_de_lesao(lesoes: usize, categoria: i8, gravidade: i8, jogador: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8>(506);
    // ponytail: jogador nulo abre a caixa de erro do 0x62ed70; não roda duas vezes
    if jogador == 0 {
        return o(lesoes, categoria, gravidade, jogador);
    }
    // ponytail: a 0x62ed70 (primeiro efeito da 0x62d140, antes de qualquer sorteio dela) passa pela
    // 0x62c680, que sorteia só na primeira consulta do dia e guarda em cache; chamá-la aqui antes enche o
    // cache na mesma ordem da original, e os dois lados do lado_a_lado acham o cache cheio
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    atual(lesoes, jogador);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(lesoes, categoria, gravidade, jogador), || crate::aleatorio::sem_contar(|| escolher_tipo_de_lesao(lesoes, categoria, gravidade, jogador)));
    PLACAR.registrar(igual && r == n, || format!("escolher_tipo_de_lesao({}, {}) original={} rust={} sorteio igual={}", categoria, gravidade, r, n, igual));
    r
}

/// compares the key date with the one of a 0x16-byte return to training queue entry, for bsearch
/// Original: `injury_compare_entry_dates` (0x632040, injury.cpp, cdecl).
///
/// [pt] { 0x632040 } (cdecl, injury.cpp): `injury_compare_entry_dates(chave, item)`, a ordem do bsearch (0x946464) da fila
/// das voltas aos treinos (entradas de 0x16 bytes) em Injury::post_due_recovery_news (0x62e010, todo dia): a data em
/// +8 da chave antes da do item (0x456770), −1; depois (0x456740), 1; igual, 0.
pub(crate) unsafe extern "C" fn ordem_da_volta_aos_treinos(chave: usize, item: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    if antes(chave + 8, read_u32_as_usize(item + 8) as u32, read_u32_as_usize(item + 0xc) as u32) != 0 {
        return -1;
    }
    if depois(chave + 8, read_u32_as_usize(item + 8) as u32, read_u32_as_usize(item + 0xc) as u32) != 0 {
        return 1;
    }
    crate::comparacao::ramo("cps_9 data igual");
    0 // cps9: a volta de hoje achada
}

pub(crate) unsafe extern "C" fn comparar_ordem_da_volta_aos_treinos(chave: usize, item: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1142)(chave, item);
    let n = ordem_da_volta_aos_treinos(chave, item);
    PLACAR.registrar(r == n, || format!("ordem_da_volta_aos_treinos(chave {:x}, item {:x}) original={} rust={}", chave, item, r, n));
    r
}

/// the player's recurring injury type, or 0xff when he has none or is not a player
/// Original: `Injury::get_recurring_injury_type` (0x62ed30, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62ed30 } (thiscall, ret 4, injury.cpp): `Injury::get_recurring_injury_type(pessoa)`, pelas notícias (0x77bc80),
/// pela avaliação do fisioterapeuta (0x7a2650) e pelas telas: o tipo da lesão que volta (+0x12 da ficha de forma de 31
/// bytes da pessoa, em [this]); pessoa nula, id a partir de [0xae2364] ou sem a ficha de jogador (+0x61): 0xff. Os
/// chamadores leem só o al.
pub(crate) unsafe extern "thiscall" fn lesao_que_volta(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 || read_u32_as_usize(pessoa) as i32 >= read_u32_as_usize(G_STAFF_COUNT) as i32 || read_u32_as_usize(pessoa + 0x61) == 0 {
        return 0xff;
    }
    *((read_u32_as_usize(this) + 31 * read_u32_as_usize(pessoa)) as *const u8).add(0x12)
}

/// Comparador: o al devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_lesao_que_volta(this: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1184)(this, pessoa);
    let n = lesao_que_volta(this, pessoa);
    PLACAR.registrar(r == n, || format!("lesao_que_volta(pessoa {:x}) original={:x} rust={:x}", pessoa, r, n));
    r
}
