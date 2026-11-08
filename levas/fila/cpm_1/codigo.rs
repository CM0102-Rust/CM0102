// ---------- %%LEVA%%a leva: a copa: quem passa no jogo e no confronto e a ordem dos cabeças de chave no sorteio (Cup.cpp) ----------

unsafe fn cpm1_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpm1_por(a: usize, v: i8) {
    *(a as *mut i8) = v;
}

unsafe fn cpm1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpm1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O corpo da 0x51c020: `Some(lado)`, ou `None` no empate também nos pênaltis com a regra deles (a original mostra a
/// caixa "Error" com a linha 0x97e e segue para W.O. e pontos): quem chama roda a original inteira, que refaz o único
/// que já foi gravado (+0x3d/+0x3e = 1 com o bit 0x10, idempotente).
unsafe fn fixture_winner_side(this: usize, f: usize) -> Option<u32> {
    let fl = cpm1_w(f + 0x38) as u16;
    if fl & 0x10 != 0 {
        cpm1_por(f + 0x3d, 1);
        cpm1_por(f + 0x3e, 1);
    }
    let (g1, g2) = (cpm1_b(f + 0x43), cpm1_b(f + 0x44));
    if g1 > g2 {
        if cpm1_b(f + 0x3d) == 1 {
            cpm1_por(f + 0x3d, 2);
        }
        if fl & 0x2000 != 0 {
            cpm1_por(f + 0x3d, cpm1_b(f + 0x3d).wrapping_sub(3));
        }
        return Some(1);
    }
    if g1 < g2 {
        if cpm1_b(f + 0x3e) == 1 {
            cpm1_por(f + 0x3e, 2);
        }
        if fl & 0x2000 != 0 {
            cpm1_por(f + 0x3e, cpm1_b(f + 0x3e).wrapping_sub(3));
        }
        return Some(2);
    }
    if fl & 2 != 0 {
        let (a1, a2) = (cpm1_b(f + 0x47), cpm1_b(f + 0x48));
        if a1 > a2 {
            crate::comparacao::ramo("cpm_1 jogo na prorrogacao");
            let c = cpm1_b(f + 0x3d);
            if c == 1 {
                cpm1_por(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3d, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3d, 3);
            }
            return Some(1); // o mandante na prorrogação
        }
        if a1 < a2 {
            let c = cpm1_b(f + 0x3e);
            if c == 1 {
                cpm1_por(f + 0x3e, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3e, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3e, 3);
            }
            return Some(2); // o visitante na prorrogação
        }
    }
    if fl & 0x21 != 0 {
        let (p1, p2) = (cpm1_b(f + 0x49), cpm1_b(f + 0x4a));
        // o código dos pênaltis: 4, ou 5 sem o bit 1 da regra (not al; and al, 1; or al, 4)
        let penalties = ((!(fl as u8)) & 1 | 4) as i8;
        if p1 > p2 {
            let c = cpm1_b(f + 0x3d);
            if c == 1 {
                cpm1_por(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3d, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3d, penalties);
            }
            return Some(1);
        }
        if p1 < p2 {
            let c = cpm1_b(f + 0x3e);
            if c == 1 {
                cpm1_por(f + 0x3e, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3e, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3e, penalties);
            }
            return Some(2);
        }
        return None; // empate nos pênaltis: a caixa da original
    }
    if fl & 0x800 != 0 {
        cpm1_por(f + 0x3d, 8); // W.O. a favor do mandante
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        cpm1_por(f + 0x3e, 8);
        return Some(2);
    }
    if fl & 0x2000 == 0 {
        return Some(0);
    }
    let a = cpm1_b(f + 0x3d).wrapping_sub(1);
    let c = cpm1_b(f + 0x3e).wrapping_sub(1);
    cpm1_por(f + 0x3d, a);
    cpm1_por(f + 0x3e, c);
    if a < c {
        return Some(1);
    }
    if a > c {
        return Some(2);
    }
    // empate nos pontos: o cabeça de chave menor (byte +4 da entrada de 6 bytes [this+0xa7], +0x3e entradas)
    let n = cpm1_w(this + 0x3e);
    if n <= 0 {
        return Some(2);
    }
    let (c1, c2) = (cpm1_d(f + 0x1c), cpm1_d(f + 0x20));
    let (mut s1, mut s2) = (0i8, 0i8);
    let t = cpm1_d(this + 0xa7);
    for k in 0..n as usize {
        let e = t + 6 * k;
        if cpm1_d(e) == c1 {
            s1 = cpm1_b(e + 4);
        }
        if cpm1_d(e) == c2 {
            s2 = cpm1_b(e + 4);
        }
    }
    Some(if s1 < s2 { 1 } else { 2 })
}

/// { 0x51c020 } (thiscall, ret 4, Cup.cpp): `Cup::decide_fixture_winner(jogo)`: quem passa num jogo de copa (1
/// mandante, 2 visitante, 0 ninguém) pela regra (+0x38): o placar (+0x43/+0x44), a prorrogação (bit 2, +0x47/+0x48), os
/// pênaltis (bits 0x21, +0x49/+0x4a), o W.O. (0x800/0x1000) e os pontos (0x2000: os códigos −1 e o cabeça de chave),
/// gravando o código de cada lado (+0x3d/+0x3e: 2 venceu, 3 na prorrogação, 4/5 nos pênaltis, 8 W.O.; −3 com 0x2000).
/// Jogo nulo ou empate nos pênaltis: a original (caixa "Error").
unsafe extern "thiscall" fn decide_fixture_winner(this: usize, jogo: usize) -> u32 {
    if jogo != 0 {
        if let Some(r) = fixture_winner_side(this, jogo) {
            return r;
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, jogo)
}

/// O corpo da 0x51f2f0: `None` no empate nos pênaltis com a regra deles (a original mostra a caixa "Error" com a linha
/// 0x11d1 e devolve 4; nada foi gravado antes).
unsafe fn tie_winner_side(conf: usize, j: usize) -> Option<u32> {
    let t = |k: usize| cpm1_b(conf + k);
    let fl = cpm1_w(j + 0x38) as u16;
    if t(0x33) as u8 != 1 {
        // primeiro jogo (ou jogo único) do confronto: 3
        if fl & 0x10 != 0 {
            let lado1 = if t(0x14) != t(0x15) {
                t(0x14) > t(0x15)
            } else if t(0x18) != t(0x19) {
                t(0x18) > t(0x19)
            } else {
                t(0x1a) > t(0x1b)
            };
            cpm1_por(j + if lado1 { 0x3d } else { 0x3e }, 1);
        } else if fl & 0x2000 != 0 {
            let pares = [(0x14, 0x15), (0x18, 0x19), (0x1a, 0x1b)];
            match pares.iter().find(|p| t(p.0) != t(p.1)) {
                Some(&(a, b)) => {
                    let lado = if t(a) > t(b) { 0x3d } else { 0x3e };
                    cpm1_por(j + lado, cpm1_b(j + lado).wrapping_sub(3));
                }
                None => {
                    cpm1_por(j + 0x3d, cpm1_b(j + 0x3d).wrapping_sub(1));
                    cpm1_por(j + 0x3e, cpm1_b(j + 0x3e).wrapping_sub(1));
                }
            }
        }
        return Some(3);
    }
    // o jogo de volta: o agregado (os gols +0x14 e +0x1f contra +0x1e e +0x15, em 16 bits)
    let tot1 = (t(0x14) as i16).wrapping_add(t(0x1f) as i16);
    let tot2 = (t(0x1e) as i16).wrapping_add(t(0x15) as i16);
    let volta2 = t(0x15) as i16;
    if fl & 0x10 != 0 {
        // pontos: quem venceu cada jogo (placar, prorrogação, pênaltis); um de cada: 4
        let lado_volta = if t(0x14) != t(0x15) {
            t(0x14) > t(0x15)
        } else if t(0x18) != t(0x19) {
            t(0x18) > t(0x19)
        } else {
            t(0x1a) > t(0x1b)
        };
        let lado_ida = if t(0x1e) != t(0x1f) {
            t(0x1e) > t(0x1f)
        } else if t(0x22) != t(0x23) {
            t(0x22) > t(0x23)
        } else {
            t(0x24) > t(0x25)
        };
        if lado_volta == lado_ida {
            cpm1_por(j + 0x3d, 1);
            cpm1_por(j + 0x3e, 1);
            return Some(4);
        }
        if lado_volta {
            cpm1_por(j + 0x3e, 2);
            return Some(2);
        }
        cpm1_por(j + 0x3d, 2);
        return Some(1);
    }
    if fl & 0x2000 != 0 {
        let pares = [(0x43, 0x44), (0x47, 0x48), (0x49, 0x4a)];
        match pares.iter().find(|p| cpm1_b(j + p.0) != cpm1_b(j + p.1)) {
            Some(&(a, b)) => {
                let lado = if cpm1_b(j + a) > cpm1_b(j + b) { 0x3d } else { 0x3e };
                cpm1_por(j + lado, cpm1_b(j + lado).wrapping_sub(3));
            }
            None => {
                cpm1_por(j + 0x3d, cpm1_b(j + 0x3d).wrapping_sub(1));
                cpm1_por(j + 0x3e, cpm1_b(j + 0x3e).wrapping_sub(1));
            }
        }
        if cpm1_b(j + 0x3d) < -5 {
            return Some(1);
        }
        return Some(if cpm1_b(j + 0x3e) >= -5 { 4 } else { 2 });
    }
    if tot1 > tot2 {
        return Some(2); // agregado
    }
    if tot1 < tot2 {
        return Some(1);
    }
    crate::comparacao::ramo("cpm_1 confronto empatado no agregado");
    if fl & 0x800 != 0 {
        cpm1_por(j + 0x3d, 8);
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        cpm1_por(j + 0x3e, 8);
        return Some(2);
    }
    if fl & 2 != 0 && t(0x22) as u8 != 0xff {
        if fl & 0x4000 != 0 && (t(0x22) as i32 + t(0x15) as i32) == (t(0x23) as i32 + t(0x14) as i32) {
            if t(0x23) > t(0x15) {
                cpm1_por(j + 0x3e, 6);
                return Some(2);
            }
            if t(0x23) < t(0x15) {
                cpm1_por(j + 0x3d, 6);
                return Some(1);
            }
        }
        let x = t(0x23) as i32 - t(0x1f) as i32;
        let y = t(0x22) as i32 - t(0x1e) as i32;
        if y > x {
            cpm1_por(j + 0x3d, 3);
            return Some(1);
        }
        if y < x {
            cpm1_por(j + 0x3e, 3);
            return Some(2);
        }
    }
    if fl & 4 == 0 && fl & 0x4000 == 0 {
        // os gols fora (o visitante da volta, +0x15, contra o da ida, +0x1f)
        let fora = t(0x1f) as i16;
        if volta2 < fora {
            cpm1_por(j + 0x3e, 6);
            return Some(2);
        }
        if volta2 > fora {
            cpm1_por(j + 0x3d, 6);
            return Some(1);
        }
    }
    if fl & 0x21 == 0 {
        return Some(4);
    }
    let (p1, p2) = (t(0x24), t(0x25));
    let codigo = if fl & 1 != 0 { 4 } else { 5 };
    if p2 < p1 {
        cpm1_por(j + 0x3d, codigo);
        return Some(1);
    }
    if p2 > p1 {
        cpm1_por(j + 0x3e, codigo);
        return Some(2);
    }
    None
}

/// { 0x51f2f0 } (thiscall, ret 8, Cup.cpp; `this` não é lido): `Cup::decide_tie_winner(confronto, jogo)`: no primeiro
/// jogo (+0x33 ≠ 1), só marca o vencedor (pontos 0x10: código 1; 0x2000: −3, ou −1 nos dois) e devolve 3. Na volta,
/// pelo agregado (+0x14 + +0x1f contra +0x1e + +0x15): 2 o primeiro lado à frente, 1 o segundo; empatado: W.O.
/// (0x800/0x1000, código 8), a prorrogação (bit 2 com +0x22 ≠ 0xff: com 0x4000 os gols fora dela; código 6, senão a
/// diferença da prorrogação, código 3), os gols fora (sem 4 nem 0x4000, código 6) e os pênaltis (0x21, +0x24/+0x25:
/// código 4 com o bit 1, senão 5); nada decidido: 4. Pontos (0x10) e regra 0x2000 têm os ramos próprios. Empate nos
/// pênaltis: a original (caixa "Error", 4).
unsafe extern "thiscall" fn decide_tie_winner(this: usize, confronto: usize, jogo: usize) -> u32 {
    match tie_winner_side(confronto, jogo) {
        Some(r) => r,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, confronto, jogo),
    }
}

/// { 0x51ebc0 } (cdecl, Cup.cpp): `cup_compare_draw_entries_by_seed(a, b)`: a comparação do qsort das entradas de 7
/// bytes do sorteio (0x51d2d0 e 0x51d9f0): o cabeça de chave (byte com sinal +6) de `b` menos o de `a` (maior primeiro).
unsafe extern "C" fn cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    crate::comparacao::ramo("cpm_1 ordem dos cabecas de chave");
    cpm1_b(b + 6) as i32 - cpm1_b(a + 6) as i32 // o maior primeiro
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; jogo nulo ou empate nos pênaltis (a
/// caixa): só a original.
unsafe extern "thiscall" fn comparar_decide_fixture_winner(this: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    if jogo == 0 {
        return o(this, jogo);
    }
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, jogo)), || fixture_winner_side(this, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_fixture_winner(regra {:x}) original={} rust={} {:x?} x {:x?}", cpm1_w(jogo + 0x38) as u16, r, n, mo, mr));
    }
    r
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; empate nos pênaltis (a caixa): só a
/// original.
unsafe extern "thiscall" fn comparar_decide_tie_winner(this: usize, confronto: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, confronto, jogo)), || tie_winner_side(confronto, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_tie_winner(regra {:x}, jogo {}) original={} rust={} {:x?} x {:x?}",
            cpm1_w(jogo + 0x38) as u16, cpm1_b(confronto + 0x33), r, n, mo, mr));
    }
    r
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = cup_compare_draw_entries_by_seed(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_draw_entries_by_seed({}, {}) original={} rust={}", cpm1_b(a + 6), cpm1_b(b + 6), r, n));
    r
}
