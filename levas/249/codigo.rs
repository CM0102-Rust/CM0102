// ---------- 249a leva: a copa: quem passa no jogo e no confronto e a ordem dos cabeças de chave no sorteio (Cup.cpp) ----------

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
unsafe fn cpm1_jogo(this: usize, f: usize) -> Option<u32> {
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
        let penaltis = ((!(fl as u8)) & 1 | 4) as i8;
        if p1 > p2 {
            let c = cpm1_b(f + 0x3d);
            if c == 1 {
                cpm1_por(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                cpm1_por(f + 0x3d, c.wrapping_sub(3));
            } else {
                cpm1_por(f + 0x3d, penaltis);
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
                cpm1_por(f + 0x3e, penaltis);
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
unsafe extern "thiscall" fn decidir_jogo_da_copa(this: usize, jogo: usize) -> u32 {
    if jogo != 0 {
        if let Some(r) = cpm1_jogo(this, jogo) {
            return r;
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(925)(this, jogo)
}

/// O corpo da 0x51f2f0: `None` no empate nos pênaltis com a regra deles (a original mostra a caixa "Error" com a linha
/// 0x11d1 e devolve 4; nada foi gravado antes).
unsafe fn cpm1_confronto(conf: usize, j: usize) -> Option<u32> {
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
unsafe extern "thiscall" fn decidir_confronto_da_copa(this: usize, confronto: usize, jogo: usize) -> u32 {
    match cpm1_confronto(confronto, jogo) {
        Some(r) => r,
        None => orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(926)(this, confronto, jogo),
    }
}

/// { 0x51ebc0 } (cdecl, Cup.cpp): `cup_compare_draw_entries_by_seed(a, b)`: a comparação do qsort das entradas de 7
/// bytes do sorteio (0x51d2d0 e 0x51d9f0): o cabeça de chave (byte com sinal +6) de `b` menos o de `a` (maior primeiro).
unsafe extern "C" fn ordem_dos_cabecas_de_chave(a: usize, b: usize) -> i32 {
    crate::comparacao::ramo("cpm_1 ordem dos cabecas de chave");
    cpm1_b(b + 6) as i32 - cpm1_b(a + 6) as i32 // o maior primeiro
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; jogo nulo ou empate nos pênaltis (a
/// caixa): só a original.
unsafe extern "thiscall" fn comparar_decidir_jogo_da_copa(this: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(925);
    if jogo == 0 {
        return o(this, jogo);
    }
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, jogo)), || cpm1_jogo(this, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decidir_jogo_da_copa(regra {:x}) original={} rust={} {:x?} x {:x?}", cpm1_w(jogo + 0x38) as u16, r, n, mo, mr));
    }
    r
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; empate nos pênaltis (a caixa): só a
/// original.
unsafe extern "thiscall" fn comparar_decidir_confronto_da_copa(this: usize, confronto: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(926);
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, confronto, jogo)), || cpm1_confronto(confronto, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decidir_confronto_da_copa(regra {:x}, jogo {}) original={} rust={} {:x?} x {:x?}",
            cpm1_w(jogo + 0x38) as u16, cpm1_b(confronto + 0x33), r, n, mo, mr));
    }
    r
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_ordem_dos_cabecas_de_chave(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(927)(a, b);
    let n = ordem_dos_cabecas_de_chave(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_cabecas_de_chave({}, {}) original={} rust={}", cpm1_b(a + 6), cpm1_b(b + 6), r, n));
    r
}

// ---------- 249a leva: a tabela da liga: iniciar a linha do clube, o vetor dos clubes e as posições finais (league.cpp) ----------

unsafe fn cpm2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpm2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpm2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Linhas da tabela: 0x3b bytes (+0 clube, +4 posição, +5 confrontos diretos de 14 bytes, +9 histórico das posições,
/// +0x11/+0x1f/+0x2d os totais, em casa e fora, +0x37 situação, +0x38 marca, +0x39, +0x3a clubes).
const CPM2_LINHA: usize = 0x3b;

/// A regra 4 (confrontos diretos) entre as 4 regras de desempate (+0xc5, até a primeira 0).
unsafe fn cpm2_regra_dos_confrontos(this: usize) -> bool {
    for k in 0..4 {
        match cpm2_b(this + 0xc5 + k) {
            0 => return false,
            4 => return true,
            _ => {}
        }
    }
    false
}

/// { 0x687430 } (thiscall, ret 0x10, league.cpp): `League::init_table_row(i, clube, marca, linhas)`: põe o clube na
/// linha `i` (char) da tabela `linhas` (nula: a da liga, +0xb1): o clube com situação +0x64 = 3 passa a 2; o total de
/// rodadas +0xa7 ainda −1 vira (clubes +0x3e − 1) × turnos +0x3c (mais de 50 clubes: caixa e segue); a linha recebe o
/// clube, a posição `i`, os três blocos de totais zerados, a marca (+0x38), situação 0xff, 1 em +0x39 e os clubes
/// (+0x3a); com turnos, o histórico das posições (+9, +0xa7 bytes; sem memória, caixa e fim); com a regra 4, os
/// confrontos diretos (+5, 14 bytes por adversário, zerados). Na última linha com confrontos, cada linha recebe os
/// adversários na ordem da tabela.
unsafe extern "thiscall" fn iniciar_linha_da_tabela(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if clube != 0 && cpm2_b(clube + 0x64) == 3 {
        *((clube + 0x64) as *mut u8) = 2;
    }
    if cpm2_w(this + 0xa7) == -1 && cpm2_w(this + 0x3e) > 0 {
        if cpm2_w(this + 0x3e) > 0x32 {
            aviso_em(0x9f15f4, 0xfe9); // mais de 50 clubes: a caixa, e segue
        }
        let total = (cpm2_w(this + 0x3e) - 1).wrapping_mul(cpm2_w(this + 0x3c));
        core::ptr::write_unaligned((this + 0xa7) as *mut i16, total);
    }
    let tab = if linhas == 0 { cpm2_d(this + 0xb1) } else { linhas };
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * CPM2_LINHA as isize) as usize);
    cpm2_por(l, clube);
    *((l + 4) as *mut u8) = i as u8;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        cpm2_por(e - 4, 0);
        core::ptr::write_bytes(e as *mut u8, 0, 4);
        core::ptr::write_bytes((e + 4) as *mut u8, 0, 6);
    }
    *((l + 0x38) as *mut u8) = marca as u8;
    *((l + 0x37) as *mut u8) = 0xff;
    *((l + 0x39) as *mut u8) = 1;
    *((l + 0x3a) as *mut u8) = cpm2_b(this + 0x3e);
    cpm2_por(l + 5, 0);
    if cpm2_w(this + 0x3c) > 0 {
        let p = alocar(cpm2_w(this + 0xa7) as i32 as u32);
        cpm2_por(l + 9, p);
        if p == 0 {
            aviso_em(0x9f15f4, 0x100c);
            return;
        }
    } else {
        cpm2_por(l + 9, 0);
    }
    if cpm2_regra_dos_confrontos(this) {
        let n = cpm2_w(this + 0x3e) as i32;
        cpm2_por(l + 5, alocar((n * 14 - 14) as u32));
        // os confrontos zerados (+4..+0xd de cada um; a original grava o +4 duas vezes, sem conferir a memória)
        let mut k = 0i32;
        while k < cpm2_w(this + 0x3e) as i32 - 1 {
            let e = cpm2_d(l + 5) + 14 * k as usize;
            core::ptr::write_bytes((e + 4) as *mut u8, 0, 10);
            k += 1;
        }
    }
    let n = cpm2_w(this + 0x3e) as isize;
    if idx == n - 1 && cpm2_d(l + 5) != 0 {
        crate::comparacao::ramo("cpm_2 confrontos diretos");
        let mut a = 0isize;
        while a < cpm2_w(this + 0x3e) as isize {
            let mut k = 0usize;
            let mut j = 0isize;
            while j < cpm2_w(this + 0x3e) as isize {
                if a != j {
                    let p = cpm2_d(tab + a as usize * CPM2_LINHA + 5);
                    cpm2_por(p + 14 * k, cpm2_d(tab + j as usize * CPM2_LINHA)); // o clube adversário
                    k += 1;
                }
                j += 1;
            }
            a += 1;
        }
    }
}

/// { 0x687a20 } (thiscall, ret 8, league.cpp): `League::get_club_array(saida, n)`: um vetor novo (malloc 0x944e46) com
/// os clubes das linhas da tabela (+0xb1) em `*saida` e a conta (+0x3e) em `*n`; sem memória, caixa e `*n` = 0.
unsafe extern "thiscall" fn vetor_dos_clubes_da_liga(this: usize, saida: *mut usize, n: *mut i16) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let p = alocar((cpm2_w(this + 0x3e) as i32 * 4) as u32);
    *saida = p;
    if p == 0 {
        aviso_em(0x9f15f4, 0x10cc);
        *n = 0;
        return;
    }
    crate::comparacao::ramo("cpm_2 vetor dos clubes");
    let mut i = 0i16;
    while i < cpm2_w(this + 0x3e) {
        cpm2_por(p + 4 * i as usize, cpm2_d(cpm2_d(this + 0xb1) + i as usize * CPM2_LINHA));
        i += 1;
    }
    *n = cpm2_w(this + 0x3e); // a conta
}

/// { 0x68aa30 } (thiscall, league.cpp): `League::record_final_positions()`: cada clube da tabela (+0xb1, na ordem) que
/// ainda está na divisão da liga (+0x57 = +4 dela) guarda a divisão como a da última temporada (+0x5b) e a posição
/// (linha + 1) em +0x5f.
unsafe extern "thiscall" fn gravar_posicoes_finais_da_liga(this: usize) {
    let mut i = 0i16;
    while i < cpm2_w(this + 0x3e) {
        let divisao = cpm2_d(this + 4);
        let c = cpm2_d(cpm2_d(this + 0xb1) + i as usize * CPM2_LINHA);
        if cpm2_d(c + 0x57) == divisao {
            crate::comparacao::ramo("cpm_2 posicao final");
            cpm2_por(c + 0x5b, divisao);
            *((c + 0x5f) as *mut u8) = (i as u8).wrapping_add(1); // a posição final
        }
        i += 1;
    }
}

/// Comparador: a original aloca (histórico e confrontos): roda só ela e confere a linha com o plano feito antes (o
/// total de rodadas, a situação do clube, os campos, os blocos zerados, os vetores presentes e, na última linha, os
/// adversários de cada linha).
unsafe extern "thiscall" fn comparar_iniciar_linha_da_tabela(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, u32, usize)>(928);
    let a7 = cpm2_w(this + 0xa7);
    let n = cpm2_w(this + 0x3e);
    let a7_esperado = if a7 == -1 && n > 0 { (n - 1).wrapping_mul(cpm2_w(this + 0x3c)) } else { a7 };
    let situacao = if clube != 0 { cpm2_b(clube + 0x64) } else { 0 };
    let situacao_esperada = if situacao == 3 { 2 } else { situacao };
    let tab = if linhas == 0 { cpm2_d(this + 0xb1) } else { linhas };
    let confrontos = cpm2_regra_dos_confrontos(this);
    o(this, i, clube, marca, linhas);
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * CPM2_LINHA as isize) as usize);
    let mut ok = cpm2_d(l) == clube && cpm2_b(l + 4) == i as u8 && cpm2_b(l + 0x37) == 0xff && cpm2_b(l + 0x38) == marca as u8
        && cpm2_b(l + 0x39) == 1 && cpm2_b(l + 0x3a) == n as u8 && cpm2_w(this + 0xa7) == a7_esperado
        && (clube == 0 || cpm2_b(clube + 0x64) == situacao_esperada)
        && (cpm2_d(l + 9) != 0) == (cpm2_w(this + 0x3c) > 0) && (cpm2_d(l + 5) != 0) == confrontos;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        ok &= (e - 4..e + 10).all(|a| cpm2_b(a) == 0);
    }
    let ultima = idx == n as isize - 1 && cpm2_d(l + 5) != 0;
    if cpm2_d(l + 5) != 0 && !ultima {
        let p = cpm2_d(l + 5);
        ok &= (0..(n as usize).saturating_sub(1)).all(|k| (p + 14 * k + 4..p + 14 * k + 14).all(|a| cpm2_b(a) == 0));
    }
    if ultima {
        for a in 0..n as usize {
            let p = cpm2_d(tab + a * CPM2_LINHA + 5);
            let outros = (0..n as usize).filter(|&j| j != a);
            ok &= outros.enumerate().all(|(k, j)| cpm2_d(p + 14 * k) == cpm2_d(tab + j * CPM2_LINHA));
        }
    }
    PLACAR.registrar(ok, || format!("iniciar_linha_da_tabela(linha {}, clube {:x}, marca {}) clubes {}, rodadas {} (esperava {}), confrontos {} ({:x}), histórico {:x}",
        idx, clube, marca as u8, n, cpm2_w(this + 0xa7), a7_esperado, confrontos, cpm2_d(l + 5), cpm2_d(l + 9)));
}

/// Comparador: a original aloca o vetor: roda só ela e confere a conta e os clubes, na ordem da tabela.
unsafe extern "thiscall" fn comparar_vetor_dos_clubes_da_liga(this: usize, saida: *mut usize, n: *mut i16) {
    orig::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(929)(this, saida, n);
    let p = *saida;
    if p == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpm2_w(this + 0x3e);
    let ok = *n == conta && (0..conta.max(0) as usize).all(|i| cpm2_d(p + 4 * i) == cpm2_d(cpm2_d(this + 0xb1) + i * CPM2_LINHA));
    PLACAR.registrar(ok, || format!("vetor_dos_clubes_da_liga conta {} (tabela {})", *n, conta));
}

/// Comparador: lado a lado pela divisão da última temporada e pela posição (+0x5b..+0x5f) de cada clube da tabela.
unsafe extern "thiscall" fn comparar_gravar_posicoes_finais_da_liga(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(930);
    let reg: Vec<(usize, usize)> = (0..cpm2_w(this + 0x3e).max(0) as usize)
        .map(|i| (cpm2_d(cpm2_d(this + 0xb1) + i * CPM2_LINHA) + 0x5b, 5))
        .collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || gravar_posicoes_finais_da_liga(this));
    PLACAR.registrar(igual, || format!("gravar_posicoes_finais_da_liga {:x?} x {:x?}", mo, mr));
}

// ---------- 249a leva: a mudança de divisão do clube e os embaralhamentos das listas de clubes (league.cpp, comp_util.cpp) ----------

unsafe fn cpm3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm3_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x6830b0 } (thiscall, ret 0xc, league.cpp; `this` não é lido): `League::move_club_to_division(clube, divisao,
/// vaga)`: a divisão atual (+0x57) vira a da última temporada (+0x5b) e a nova entra; as cláusulas de divisão dos
/// jogadores são limpas (0x4cd120 em [0xae1050]), com os números da camisa também se a divisão velha não os usava e a
/// nova usa (0x85add0 em [0xdd7ef0], antes e depois); com `vaga` e sem técnico (+0xcf), a vaga de técnico (0x69ba50 em
/// [0xb67a44]); numa divisão de liga (+0x43 = 2 na competição [0xadadfc][id]) com estádio mínimo (+0xe2 ≠ −1) e ficha
/// do estádio (+0x69), a obra do estádio (0x599e00 nas finanças 0x5a0590 de [0xae2a38], lugares +0xe4 e +0xe2); e,
/// com a competição carregada, a situação +0x64 = 3 passa a 2.
unsafe extern "thiscall" fn mudar_clube_de_divisao(_this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("cpm_3 mudar de divisao");
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85add0usize);
    let limpar_clausulas: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4cd120usize);
    let antes = usa_numeros(cpm3_d(0xdd7ef0), clube, 0);
    cpm3_por(clube + 0x5b, cpm3_d(clube + 0x57)); // a divisão da última temporada
    cpm3_por(clube + 0x57, divisao);
    let numeros = (antes == 0 && usa_numeros(cpm3_d(0xdd7ef0), clube, 0) != 0) as u32;
    limpar_clausulas(cpm3_d(0xae1050), clube, numeros);
    if vaga != 0 && cpm3_d(clube + 0xcf) == 0 {
        let abrir_vaga: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x69ba50usize);
        abrir_vaga(cpm3_d(0xb67a44), clube, 0);
    }
    if divisao == 0 {
        return;
    }
    let comps = cpm3_d(0xadadfc);
    let id = cpm3_d(divisao);
    if comps != 0 {
        let comp = cpm3_d(comps + 4 * id);
        if comp != 0 && cpm3_d(clube + 0x69) != 0 && *((comp + 0x43) as *const u8) == 2 {
            let lugares = core::ptr::read_unaligned((comp + 0xe2) as *const i16);
            if lugares != -1 {
                crate::comparacao::ramo("cpm_3 estadio da divisao");
                let capacidade = core::ptr::read_unaligned((comp + 0xe4) as *const i16);
                let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
                let obra: unsafe extern "thiscall" fn(usize, i32, i32, usize, u32) -> u32 = core::mem::transmute(0x599e00usize);
                let f = financas(cpm3_d(0xae2a38), clube);
                obra(f, capacidade as i32, lugares as i32, id, 0); // a obra do estádio
            }
        }
    }
    // a original relê [0xadadfc] depois da obra (sem a obra, o mesmo valor; nulo, ela lê do endereço 0 também)
    if cpm3_d(cpm3_d(0xadadfc) + 4 * id) != 0 && *((clube + 0x64) as *const u8) == 3 {
        *((clube + 0x64) as *mut u8) = 2;
    }
}

unsafe fn cpm3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// As trocas de `vezes` pares sorteados (0x90cfc0(n) duas vezes; a primeira dá o índice que recebe o item do segundo)
/// no vetor de ponteiros `itens`, como as duas originais (índices de 16 bits com sinal).
unsafe fn cpm3_trocar(itens: usize, n: i32) {
    let a = cpm3_sorteio(n) as i16 as isize;
    let b = cpm3_sorteio(n) as i16 as isize;
    let pa = (itens as isize + 4 * a) as usize;
    let pb = (itens as isize + 4 * b) as usize;
    let (va, vb) = (cpm3_d(pa), cpm3_d(pb));
    cpm3_por(pa, vb);
    cpm3_por(pb, va);
}

/// { 0x4afb40 } (cdecl, comp_util.cpp): `comp_util_shuffle_pointer_array(itens, n)`: embaralha o vetor de `n` (curto)
/// ponteiros com 3 × n trocas de pares sorteados (0x90cfc0(n) duas vezes cada).
unsafe extern "C" fn embaralhar_ponteiros(itens: usize, n: u32) {
    let n = n as u16 as i16 as i32;
    if 3 * n <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar ponteiros");
    let mut k = 0i16;
    loop {
        cpm3_trocar(itens, n);
        k = k.wrapping_add(1); // mais uma troca
        if k as i32 >= 3 * n {
            break;
        }
    }
}

/// { 0x4afbd0 } (cdecl, comp_util.cpp): `comp_util_sort_and_shuffle_top_clubs(clubes, n, escolhas)`: ordena os `n`
/// clubes pela reputação, maior primeiro e nulos no fim (qsort 0x945333 com 0x4b07d0), e embaralha os m = min(3 ×
/// escolhas, n) primeiros com m trocas de pares sorteados entre eles. `n` negativo ou menor que `escolhas`: caixa
/// "Error" (a original); `escolhas` < 1: nada.
unsafe extern "C" fn ordenar_e_embaralhar_os_melhores(clubes: usize, n: u32, escolhas: u32) {
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return orig::<unsafe extern "C" fn(usize, u32, u32)>(933)(clubes, n, escolhas); // a caixa
    }
    if e < 1 {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(clubes, nn as i32 as u32, 4, 0x4b07d0);
    let mut m = e.wrapping_mul(3);
    if m > nn {
        m = nn;
    }
    if m <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar os melhores");
    for _ in 0..m as i32 { // m trocas
        cpm3_trocar(clubes, m as i32);
    }
}

/// Comparador: a mudança chama as notícias, a vaga de técnico e a obra (não podem rodar duas vezes): só a original (a
/// prova é a carreira e o modo Rust).
unsafe extern "thiscall" fn comparar_mudar_clube_de_divisao(this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("mudar_clube_de_divisao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(931)(this, clube, divisao, vaga)
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pelo vetor, a cópia em sem_contar.
unsafe extern "C" fn comparar_embaralhar_ponteiros(itens: usize, n: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(932);
    let k = (n as u16 as i16).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if k > 0 {
        reg.push((itens, 4 * k));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(itens, n),
        || crate::aleatorio::sem_contar(|| embaralhar_ponteiros(itens, n)));
    PLACAR.registrar(igual, || format!("embaralhar_ponteiros({}) {:x?} x {:x?}", k, mo, mr));
}

/// Comparador: ordena (qsort, determinístico) e sorteia: lado a lado pelo estado do sorteio e pelos `n` clubes, a cópia
/// em sem_contar. Caixa (`n` negativo ou menor que `escolhas`): só a original.
unsafe extern "C" fn comparar_ordenar_e_embaralhar_os_melhores(clubes: usize, n: u32, escolhas: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32)>(933);
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return o(clubes, n, escolhas);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if nn > 0 {
        reg.push((clubes, 4 * nn as usize));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clubes, n, escolhas),
        || crate::aleatorio::sem_contar(|| ordenar_e_embaralhar_os_melhores(clubes, n, escolhas)));
    PLACAR.registrar(igual, || format!("ordenar_e_embaralhar_os_melhores({}, {}) {:x?} x {:x?}", nn, e, mo, mr));
}

// ---------- 249a leva: a disciplina: contar o jogo cumprido da suspensão, soltar a suspensão e apagar o histórico de suspensões da pessoa (discipline.cpp) ----------

unsafe fn cpm4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de 29 bytes da pessoa `id` em [this+0x14] (+0x14/+0x18: o começo e o último deslocamento do histórico).
unsafe fn cpm4_registro(this: usize, id: usize) -> usize {
    cpm4_d(this + 0x14).wrapping_add(id.wrapping_mul(29))
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn cpm4_orgao(this: usize, regra: u8) -> usize {
    cpm4_d(cpm4_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada (14 bytes; +6 o índice da suspensão, −1 sem) da pessoa no órgão: 0x7e82e0 sem criar (só lê).
unsafe fn cpm4_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// O item (id, nação) da pessoa no índice das nações (+0xc, 8 bytes por item, conta +0x12 com sinal): bsearch
/// (0x946464 com 0x55e5f0) pela nação (+0x1a) e, sem achar, pela segunda nação (+0x1e, se houver). 0 sem achar.
unsafe fn cpm4_no_indice(this: usize, id: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let pessoa = cpm4_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    let mut chave = [id, cpm4_d(cpm4_d(pessoa + 0x1a)) as u32];
    let e = bsearch(chave.as_ptr(), cpm4_d(this + 0xc), cpm4_w(this + 0x12) as i32, 8, 0x55e5f0);
    if e != 0 {
        return e;
    }
    let segunda = cpm4_d(cpm4_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)) + 0x1e);
    if segunda == 0 {
        return 0;
    }
    chave[1] = cpm4_d(segunda) as u32;
    bsearch(chave.as_ptr(), cpm4_d(this + 0xc), cpm4_w(this + 0x12) as i32, 8, 0x55e5f0)
}

/// O que a 0x556e90 faz com o jogo: a entrada no órgão, a suspensão (20 bytes em [this+4]), o contador novo de jogos
/// cumpridos (+1) e se a suspensão termina.
struct Cpm4Jogo {
    entrada: usize,
    suspensao: usize,
    contador: Option<u8>,
    cumprida: bool,
}

/// O plano do jogo (só lê; a 0x7e82e0 sem criar e a 0x54c5b0 só gravam a data de rascunho dela): a suspensão da
/// pessoa no órgão da `regra`; por jogos (+0 ≠ 0xff): a partir do começo (dia +4, ano +6; hoje [0xae2c90]: dia +0,
/// ano +2), com `conta`, +1 no contador (+1), e termina ao chegar em +0; por dias (+0 = 0xff): termina quando hoje
/// chega ao começo + dias (+2) (0x54c5b0).
unsafe fn cpm4_jogo(this: usize, id: u32, regra: u32, conta: u32) -> Cpm4Jogo {
    let mut j = Cpm4Jogo { entrada: 0, suspensao: 0, contador: None, cumprida: false };
    let e = cpm4_entrada(cpm4_orgao(this, regra as u8), id);
    if e == 0 || cpm4_w(e + 6) == -1 {
        return j;
    }
    j.entrada = e;
    let cinco = (cpm4_w(e + 6) as i32 as u32).wrapping_mul(5); // o lea da original (o alto vai no edx abaixo)
    let s = cpm4_d(this + 4).wrapping_add((cinco as usize).wrapping_mul(4));
    j.suspensao = s;
    let hoje = cpm4_d(0xae2c90) as u32;
    let (dia, ano) = (hoje as u16 as i16, (hoje >> 16) as u16);
    let jogos = *(s as *const u8);
    if jogos != 0xff {
        let (dia0, ano0) = (cpm4_w(s + 4), cpm4_w(s + 6) as u16);
        if ano < ano0 || (ano == ano0 && dia < dia0) { // ainda não começou
            crate::comparacao::ramo("cpm_4 antes do inicio");
            return j;
        }
        if conta == 0 {
            return j;
        }
        crate::comparacao::ramo("cpm_4 jogo cumprido");
        let n = (*((s + 1) as *const u8)).wrapping_add(1); // mais um jogo cumprido
        j.contador = Some(n);
        j.cumprida = n == jogos;
    } else {
        let mut fim = [0u8; 8];
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        // a original empurra o edx com os dias no dx e, no alto, o do índice × 5 (zero abaixo de 13108 suspensões)
        let f = mais_dias(s + 4, fim.as_mut_ptr(), (cinco & 0xffff_0000) | cpm4_w(s + 2) as u16 as u32);
        let (dia1, ano1) = (cpm4_w(f), cpm4_w(f + 2) as u16);
        if ano < ano1 || (ano == ano1 && dia < dia1) {
            return j;
        }
        j.cumprida = true;
    }
    j
}

/// { 0x556e90 } (thiscall, ret 0xc, discipline.cpp): `Discipline::update_ban_progress(id, regra, conta)`: a suspensão
/// da pessoa no órgão da `regra` (plano em `cpm4_jogo`): por jogos, grava o contador novo (+1); cumprida, vai para o
/// histórico (0x58e340 sobre [this+0x18] com o começo e o último deslocamento do registro de 29 bytes) e é solta
/// (0x556d00). Sem SEH, sem sorteio, sem caixa.
unsafe extern "thiscall" fn contar_jogo_da_suspensao(this: usize, id: u32, regra: u32, conta: u32) {
    let j = cpm4_jogo(this, id, regra, conta);
    if let Some(n) = j.contador {
        *((j.suspensao + 1) as *mut u8) = n;
    }
    if !j.cumprida {
        return;
    }
    let r = cpm4_registro(this, id as usize);
    let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(0x58e340usize);
    let g = historico(cpm4_d(this + 0x18), r + 0x14, r + 0x18, j.suspensao);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
    // o índice vai no ax; o alto do eax é o da devolução da 0x58e340 (a 0x556d00 só lê a palavra)
    soltar(this, id, (g as u32 & 0xffff_0000) | cpm4_w(j.entrada + 6) as u16 as u32);
}

/// { 0x556d00 } (thiscall, ret 8, discipline.cpp): `Discipline::release_ban(id, indice)`: a suspensão `indice` (20
/// bytes em [this+4]; a palavra com sinal): para cada uma das 4 regras (+0xc..+0xf, 0xff = sem), o órgão a esquece
/// (0x7e8590); na quarta, o item da pessoa sai do índice das nações (o último vem para o lugar, conta +0x12 − 1, qsort
/// 0x945333 com 0x55e5f0; sem achar, caixa "Error" e segue). Por fim a suspensão fica livre (+0 = 0xff, +2 = −1).
/// Devolve a suspensão.
unsafe extern "thiscall" fn soltar_suspensao(this: usize, id: u32, indice: u32) -> usize {
    let s = cpm4_d(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    let esquecer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7e8590usize);
    for k in 0..4usize {
        let regra = *((s + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        crate::comparacao::ramo("cpm_4 soltar do orgao");
        esquecer(cpm4_orgao(this, regra), id); // o órgão esquece a suspensão
        if k != 3 {
            continue;
        }
        let e = cpm4_no_indice(this, id);
        if e == 0 {
            aviso_em(0x9b8f18, 0x47d);
            continue;
        }
        let n = cpm4_w(this + 0x12).wrapping_sub(1);
        core::ptr::write_unaligned((this + 0x12) as *mut i16, n);
        let ultimo = cpm4_d(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
        core::ptr::write_unaligned(e as *mut u32, cpm4_d(ultimo) as u32);
        core::ptr::write_unaligned((e + 4) as *mut u32, cpm4_d(ultimo + 4) as u32);
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cpm4_d(this + 0xc), cpm4_w(this + 0x12) as i32, 8, 0x55e5f0);
    }
    *(s as *mut u8) = 0xff;
    core::ptr::write_unaligned((s + 2) as *mut u16, 0xffff);
    s
}

/// { 0x55e1d0 } (thiscall, ret 4, discipline.cpp): `Discipline::clear_ban_history(pessoa)`: o histórico de suspensões
/// da pessoa (registro de 29 bytes do id em [this+0x14]) fica sem o último (0x58f110 sobre [this+0x18]: +0x18 = −1).
/// Devolve o que a 0x58f110 devolve. Pessoa nula ou não jogadora (+0x61): a original (caixa fatal).
unsafe extern "thiscall" fn apagar_historico_de_suspensoes(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 || cpm4_d(pessoa + 0x61) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(936)(this, pessoa); // caixa fatal: só a original
    }
    let r = cpm4_registro(this, cpm4_d(pessoa));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x58f110usize);
    sem_ultimo(cpm4_d(this + 0x18), r + 0x14, r + 0x18)
}

/// Comparador: a 0x58e340 grava o histórico no arquivo (não pode rodar duas vezes): o plano antes, a original roda;
/// confere o contador, ou a suspensão solta (livre e a entrada do órgão sem índice), ou nada mudado.
unsafe extern "thiscall" fn comparar_contar_jogo_da_suspensao(this: usize, id: u32, regra: u32, conta: u32) {
    let j = cpm4_jogo(this, id, regra, conta);
    let antes = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
    orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(934)(this, id, regra, conta);
    if j.suspensao == 0 {
        return;
    }
    let s = j.suspensao;
    let (jogos, contador, dias) = (*(s as *const u8), *((s + 1) as *const u8), cpm4_w(s + 2));
    let ok = if j.cumprida {
        jogos == 0xff && dias == -1 && cpm4_w(j.entrada + 6) == -1
    } else {
        contador == j.contador.unwrap_or(antes)
    };
    PLACAR.registrar(ok, || {
        format!("contar_jogo_da_suspensao({}, {}, {}) contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{} entrada {}",
            id, regra as u8, conta, contador, j.contador, antes, j.cumprida, jogos, dias, cpm4_w(j.entrada + 6))
    });
}

/// Comparador: lado a lado pela suspensão, o índice das nações (conta e itens) e as entradas da pessoa nos órgãos das
/// regras (achadas antes, sem criar). Quarta regra sem o item no índice: só a original (caixa).
unsafe extern "thiscall" fn comparar_soltar_suspensao(this: usize, id: u32, indice: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(935);
    let s = cpm4_d(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    if *((s + 0xf) as *const u8) != 0xff && cpm4_no_indice(this, id) == 0 {
        return o(this, id, indice);
    }
    let mut reg = vec![(s, 20), (this + 0x10, 4)];
    let n = cpm4_w(this + 0x12);
    if n > 0 {
        reg.push((cpm4_d(this + 0xc), n as usize * 8));
    }
    for k in 0..4usize {
        let regra = *((s + 0xc + k) as *const u8);
        if regra != 0xff {
            let e = cpm4_entrada(cpm4_orgao(this, regra), id);
            if e != 0 {
                reg.push((e, 14));
            }
        }
    }
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id, indice), || soltar_suspensao(this, id, indice));
    PLACAR.registrar(igual && r == m, || format!("soltar_suspensao({}, {}) {:x?} x {:x?}", id, indice as u16 as i16, mo, mr));
    r
}

/// Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn comparar_apagar_historico_de_suspensoes(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(936);
    if pessoa == 0 || cpm4_d(pessoa + 0x61) == 0 {
        return o(this, pessoa);
    }
    let r = cpm4_registro(this, cpm4_d(pessoa));
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 0x14, 8)], || o(this, pessoa), || apagar_historico_de_suspensoes(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("apagar_historico_de_suspensoes({}) {:x?} x {:x?}", cpm4_d(pessoa), mo, mr));
    a
}

// ---------- 249a leva: as lesões: sortear a lesão recorrente, recomeçar o histórico de lesões e reiniciar o físico do jogador (injury.cpp) ----------

unsafe fn cpm5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 31 bytes do físico da pessoa `id` em [this] (+0x12 o tipo da lesão recorrente, 0xff sem; +0x13 o
/// nível; +0x14/+0x18 o começo e o último deslocamento do histórico de lesões).
unsafe fn cpm5_registro(this: usize, id: usize) -> usize {
    cpm5_d(this).wrapping_add(id.wrapping_mul(31))
}

unsafe fn cpm5_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O que a 0x62e500 decide depois de sortear.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Cpm5Recorrente {
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
unsafe fn cpm5_plano(this: usize, registro: usize, lesao: *const u8) -> Cpm5Recorrente {
    if cpm5_b(registro + 0x12) != 0xff || cpm5_d((0x9e7ae0isize + *lesao as i8 as isize * 13) as usize) == 0 {
        return Cpm5Recorrente::Nada;
    }
    let s = cpm5_sorteio(0x14);
    if s != 0 { // 1 em 20
        return Cpm5Recorrente::Nada;
    }
    crate::comparacao::ramo("cpm_5 lesao recorrente");
    *((registro + 0x13) as *mut u8) = s as u8;
    *((registro + 0x12) as *mut u8) = *lesao;
    if *(0xae2364 as *const i32) <= 0 {
        return Cpm5Recorrente::Marcada;
    }
    let (mut melhor, mut escolhido, mut pessoa) = (0i8, 0usize, 0usize);
    let mut r = cpm5_d(this);
    let mut i = 0i32;
    while i < *(0xae2364 as *const i32) {
        if cpm5_b(r + 0x12) != 0xff {
            let p = cpm5_d(0xae23bc).wrapping_add(i as usize * 0x6e);
            let v = cpm5_sorteio(0x65) as u8;
            let jogador = cpm5_d(p + 0x61);
            let nota = if jogador == 0 { 0x64 } else { v.wrapping_add(cpm5_b(r + 0x13).wrapping_mul(0xfb)).wrapping_sub(cpm5_b(jogador + 0x2d)) };
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
    let clube = cpm5_d(pessoa + 0x39);
    if cpm5_d(pessoa + 0x61) != 0 && clube != 0 && core::ptr::read_unaligned((clube + 0x80) as *const i16) > 0x109a {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) == 0 && cpm5_sorteio(3) == 0 && pessoa != 0 {
            return Cpm5Recorrente::Tratar(pessoa);
        }
    }
    Cpm5Recorrente::Limpar(escolhido)
}

/// { 0x62e500 } (thiscall, ret 8, injury.cpp): `Injury::roll_recurring_injury(registro, lesao)`: o plano
/// (`cpm5_plano`); tratar: a lesão nova da pessoa (0x62d250 com 0x4e, 0x4f ou 0x4d pelo tipo recorrente do registro
/// dela, 0xff fora das pessoas ou sem jogador; o tipo vai no byte baixo do argumento `registro`, como a original);
/// limpar: o registro escolhido sem recorrente (+0x12 = 0xff, +0x13 = 0).
unsafe extern "thiscall" fn sortear_lesao_recorrente(this: usize, registro: usize, lesao: *const u8) {
    match cpm5_plano(this, registro, lesao) {
        Cpm5Recorrente::Tratar(p) => {
            let id = cpm5_d(p) as i32;
            let tipo = if id < *(0xae2364 as *const i32) && cpm5_d(p + 0x61) != 0 {
                cpm5_b(cpm5_registro(this, id as usize) + 0x12)
            } else {
                0xff
            };
            let nova: u8 = match tipo {
                0x0b | 0x11 | 0x17 | 0x1e | 0x23 | 0x29 => 0x4e,
                0x0c | 0x0d | 0x12 | 0x14 | 0x19..=0x1c | 0x1f | 0x24 | 0x25 | 0x2c | 0x2d | 0x3b => 0x4f,
                _ => 0x4d,
            };
            let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x62d250usize);
            lesionar(this, (registro as u32 & 0xffff_ff00) | nova as u32, 0, p, 0, 1, -1, 0, 0);
        }
        Cpm5Recorrente::Limpar(r) => {
            *((r + 0x12) as *mut u8) = 0xff;
            *((r + 0x13) as *mut u8) = 0;
        }
        _ => {}
    }
}

/// { 0x631f90 } (thiscall, ret 4, injury.cpp): `Injury::reset_player_history(pessoa)`: o histórico de lesões da pessoa
/// (registro de 31 bytes do id em [this]) fica sem o último (0x58f110 sobre [this+0xc]: +0x18 = −1). Devolve o que a
/// 0x58f110 devolve. Pessoa nula: a original (caixa fatal).
unsafe extern "thiscall" fn recomecar_historico_de_lesoes(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(938)(this, pessoa); // caixa fatal: só a original
    }
    let r = cpm5_registro(this, cpm5_d(pessoa));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x58f110usize);
    sem_ultimo(cpm5_d(this + 0xc), r + 0x14, r + 0x18)
}

/// { 0x62bfa0 } (thiscall, ret 4, injury.cpp): `Injury::reset_player(id)`: tira da lista [this+0x28] (do começo: o
/// cursor +0xa volta a +2) cada item do id (0x68d1f0); com lesão (+0xe ≠ −1), termina (0x62e700, sem notícia);
/// recomeça o histórico (0x631f90); o registro de 31 bytes recomeça: data de hoje (0x417c10 de [0xae2c90]), condição
/// e forma 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1 (+0x14,
/// +0x18), +0x1d 0xff. O bit 0 do +0xc do registro de 79 bytes ([0xae2d0c]) cai. Devolve o endereço desse byte.
unsafe extern "thiscall" fn reiniciar_fisico_do_jogador(this: usize, id: u32) -> usize {
    crate::comparacao::ramo("cpm_5 reiniciar jogador");
    if cpm5_d(this + 0x28) != 0 {
        let l = cpm5_d(this + 0x28);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpm5_d(l + 2));
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
        let mut e = proximo(cpm5_d(this + 0x28));
        while e != 0 {
            if cpm5_d(e) as u32 == id {
                tirar(cpm5_d(this + 0x28), e);
            }
            e = proximo(cpm5_d(this + 0x28));
        }
    }
    let r = cpm5_registro(this, id as usize);
    let pessoa = cpm5_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if cpm5_d(r + 0xe) as i32 != -1 {
        let terminar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62e700usize);
        terminar(this, pessoa, 0);
    }
    let recomecar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x631f90usize);
    recomecar(this, cpm5_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
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
    let g = cpm5_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    *(g as *mut u8) &= 0xfe;
    g
}

/// Comparador: a 0x62d250 (tratar) não pode rodar duas vezes: o plano antes, com o estado do sorteio guardado (sem
/// contar nem rastrear) e as marcas do registro devolvidas; a original roda. Confere as marcas do registro e do
/// escolhido e, fora do tratar, o estado do sorteio igual ao do plano (os mesmos sorteios).
unsafe extern "thiscall" fn comparar_sortear_lesao_recorrente(this: usize, registro: usize, lesao: *const u8) {
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let marcas = (cpm5_b(registro + 0x12), cpm5_b(registro + 0x13));
    let plano = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cpm5_plano(this, registro, lesao)));
    let depois_do_plano = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    *((registro + 0x12) as *mut u8) = marcas.0;
    *((registro + 0x13) as *mut u8) = marcas.1;
    let tipo = *lesao;
    orig::<unsafe extern "thiscall" fn(usize, usize, *const u8)>(937)(this, registro, lesao);
    let agora = (cpm5_b(registro + 0x12), cpm5_b(registro + 0x13));
    let mesmo_sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois_do_plano;
    let ok = match plano {
        Cpm5Recorrente::Nada => agora == marcas && mesmo_sorteio,
        Cpm5Recorrente::Marcada => agora == (tipo, 0) && mesmo_sorteio,
        Cpm5Recorrente::Limpar(r) => {
            (cpm5_b(r + 0x12), cpm5_b(r + 0x13)) == (0xff, 0) && (r == registro || agora == (tipo, 0)) && mesmo_sorteio
        }
        Cpm5Recorrente::Tratar(_) => true, // a 0x62d250 sorteia e grava o resto
    };
    PLACAR.registrar(ok, || {
        format!("sortear_lesao_recorrente(tipo {}) plano {:x?}, marcas {:?} -> {:?}, mesmo sorteio={}", tipo, plano, marcas, agora, mesmo_sorteio)
    });
}

/// Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn comparar_recomecar_historico_de_lesoes(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(938);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = cpm5_registro(this, cpm5_d(pessoa));
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 0x14, 8)], || o(this, pessoa), || recomecar_historico_de_lesoes(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("recomecar_historico_de_lesoes({}) {:x?} x {:x?}", cpm5_d(pessoa), mo, mr));
    a
}

/// Comparador: a 0x68d1f0 libera o item tirado e a 0x62e700 grava o histórico no arquivo: o plano antes (os itens
/// que ficam, o byte do registro de 79 bytes, os bytes do registro que não mudam); a original roda; confere.
unsafe extern "thiscall" fn comparar_reiniciar_fisico_do_jogador(this: usize, id: u32) -> usize {
    let l = cpm5_d(this + 0x28);
    let esperado: Vec<usize> = if l != 0 { cpd1_itens(l).into_iter().filter(|&e| cpm5_d(e) as u32 != id).collect() } else { Vec::new() };
    let r = cpm5_registro(this, id as usize);
    let g = cpm5_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    let (byte, nivel, b1c) = (cpm5_b(g) & 0xfe, cpm5_b(r + 0x13), cpm5_b(r + 0x1c));
    let ret = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(939)(this, id);
    let itens = if l != 0 { cpd1_itens(l) } else { Vec::new() };
    let w = |o: usize| core::ptr::read_unaligned((r + o) as *const i16);
    let dd = |o: usize| cpm5_d(r + o) as i32;
    let ok = itens == esperado && ret == g && cpm5_b(g) == byte
        && cpm5_d(r) == cpm5_d(0xae2c90) && cpm5_d(r + 4) == cpm5_d(0xae2c94)
        && w(8) == 10000 && w(0xa) == 10000 && w(0xc) == 0 && dd(0xe) == -1 && cpm5_b(r + 0x12) == 0xff
        && cpm5_b(r + 0x13) == nivel && dd(0x14) == -1 && dd(0x18) == -1 && cpm5_b(r + 0x1c) == b1c && cpm5_b(r + 0x1d) == 0xff;
    PLACAR.registrar(ok, || {
        let v: Vec<u8> = (0..31).map(|o| cpm5_b(r + o)).collect();
        format!("reiniciar_fisico_do_jogador({}) itens {} (esperava {}), byte {:#x} (esperava {:#x}), registro {:02x?}", id, itens.len(), esperado.len(), cpm5_b(g), byte, v)
    });
    ret
}

// ---------- 249a leva: os recordes do clube: somar o valor da transferência aos totais de compra e venda, os recordes de jogos e gols na carreira pelo clube e o recorde de nota da temporada (club_records.cpp) ----------

unsafe fn cpm6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm6_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpm6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpm6_por_w(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A notícia de recorde (id do clube, tipo) já está no vetor de 6 bytes [this+0x24] (conta +0x2c)?
unsafe fn cpm6_noticia_na_fila(this: usize, id: usize, tipo: u8) -> bool {
    let n = cpm6_d(this + 0x2c) as i32;
    let v = cpm6_d(this + 0x24);
    (0..n.max(0) as usize).any(|k| cpm6_d(v + 6 * k) == id && *((v + 6 * k + 4) as *const u8) == tipo)
}

/// { 0x448ad0 } (thiscall, ret 0xc, club_records.cpp): `Club_Records::add_transfer_totals(valor, comprador,
/// vendedor)`: o vendedor soma o `valor` ao recebido na temporada (+0x36b do registro da temporada 0x4528d0; −1 =
/// ainda nada: fica o valor) e zera o ano (+0x36f); acima do recorde de sempre (0x452710, mesmo campo), o recorde passa
/// a ser o da temporada com o ano da temporada do clube (0x669950), e com o total no mínimo (+0x80 do clube + 500) ×
/// 1000 e o ano diferente do recorde, a notícia de recorde 0x1f (0x452cb0) se ainda não está na fila. O comprador, o
/// mesmo pelo gasto (+0x365, ano +0x369, notícia 0x1e), mas o mínimo é conferido com o recorde ANTIGO e sem o ano. Os
/// registros são criados se faltarem (o segundo argumento 1). Comprador ou vendedor nulo: nada, 1. Devolve 1.
unsafe extern "thiscall" fn somar_totais_de_transferencia(this: usize, valor: i32, comprador: usize, vendedor: usize) -> u32 {
    if comprador == 0 || vendedor == 0 {
        return 1;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let ano_do_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669950usize);
    let noticia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x452cb0usize);
    let somar = |campo: usize| {
        let v = cpm6_d(campo) as i32;
        cpm6_por(campo, if v == -1 { valor } else { v.wrapping_add(valor) } as usize);
    };
    // o vendedor: o recebido
    let t = temporada(this, vendedor, 1);
    let s = sempre(this, vendedor, 1);
    somar(t + 0x36b);
    cpm6_por_w(t + 0x36f, 0);
    let ano = ano_do_clube(vendedor) as i16;
    if (cpm6_d(s + 0x36b) as i32) < cpm6_d(t + 0x36b) as i32 {
        let minimo = (cpm6_w(vendedor + 0x80) as i32 + 500).wrapping_mul(1000);
        if cpm6_d(t + 0x36b) as i32 >= minimo && ano != cpm6_w(s + 0x36f) && !cpm6_noticia_na_fila(this, cpm6_d(vendedor), 0x1f) {
            crate::comparacao::ramo("cpm_6 noticia de recorde recebido");
            noticia(this, cpm6_d(vendedor) as u32, 0x1f); // recorde de valor recebido
        }
        cpm6_por(s + 0x36b, cpm6_d(t + 0x36b));
        cpm6_por_w(s + 0x36f, ano);
    }
    // o comprador: o gasto
    let t = temporada(this, comprador, 1);
    let s = sempre(this, comprador, 1);
    somar(t + 0x365);
    cpm6_por_w(t + 0x369, 0);
    let ano = ano_do_clube(comprador) as i16;
    if (cpm6_d(t + 0x365) as i32) > cpm6_d(s + 0x365) as i32 {
        let minimo = (cpm6_w(comprador + 0x80) as i32 + 500).wrapping_mul(1000);
        // a original confere o mínimo com o recorde antigo (o ecx que ainda tem o de sempre)
        if cpm6_d(s + 0x365) as i32 >= minimo && !cpm6_noticia_na_fila(this, cpm6_d(comprador), 0x1e) {
            crate::comparacao::ramo("cpm_6 noticia de recorde pago");
            noticia(this, cpm6_d(comprador) as u32, 0x1e); // recorde de valor pago
        }
        cpm6_por(s + 0x365, cpm6_d(t + 0x365));
        cpm6_por_w(s + 0x369, ano);
    }
    1
}

/// { 0x448cb0 } (thiscall, ret 0x10, club_records.cpp): `club_records_update_career_records(jogador, clube, jogos,
/// gols)`: nos recordes de sempre do clube (0x452710, criados se faltarem), se o tipo 0x17 vale para o clube (0x449590)
/// e os `jogos` passam o recorde (+0x247; −1 = sem recorde), o recorde = `jogos` e a referência do jogador em +0x249
/// (0x7e4920); o mesmo com o tipo 0x18 e os `gols` (+0x261, referência +0x263). Devolve 1. Jogador ou clube nulo, ou
/// sem registro: a caixa "Error" (linhas 0x6fa e 0x702) e 0.
unsafe extern "thiscall" fn recordes_de_carreira_no_clube(this: usize, jogador: usize, clube: usize, jogos: u32, gols: u32) -> u32 {
    if jogador == 0 || clube == 0 {
        aviso_em(0x98b6a0, 0x6fa);
        return 0;
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let r = sempre(this, clube, 1);
    if r == 0 {
        aviso_em(0x98b6a0, 0x702);
        return 0;
    }
    let vale: unsafe extern "stdcall" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x449590usize);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    for (tipo, campo, n) in [(0x17u32, 0x247usize, jogos as i16), (0x18, 0x261, gols as i16)] {
        if vale(tipo, clube, 1) != 0 {
            let v = cpm6_w(r + campo);
            if v == -1 || n > v {
                cpm6_por_w(r + campo, n);
                referencia(jogador, r + campo + 2);
            }
        }
    }
    1
}

/// { 0x452a90 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_season_rating_record(clube)`: com jogos
/// na temporada (o byte +0x10 da entrada do clube, [this+0x1c] + id × 0x6a) e o registro da temporada (0x4528d0, sem
/// criar), a nota do recorde (+0x1f1) volta a −1.0 e, pelas 50 vagas do elenco (+0xd7), o jogador (não técnico humano:
/// id < [0xae2364]) com estatísticas (+0x61) no clube (+0x39 = clube ou o ligado a ele, 0x540a50) e registro de
/// estatísticas da temporada (0x7c3920, tipo 6) que jogou ao menos metade dos jogos (estatística 1, truncada, 0x7c2150 com
/// 6.5) e tem a maior média (estatística 0x11) vira o recorde: a nota (+0x1f1, float), os jogos (+0x20d), a referência
/// (+0x1f5, 0x7e4920) e +0x20e = 0. Devolve 1 se alguém virou recorde. Sem jogos ou sem registro: 0. Clube nulo: a
/// original (caixa 0x16f2, 0).
unsafe extern "thiscall" fn recorde_de_nota_da_temporada(this: usize, clube: usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(942)(this, clube); // caixa: só a original
    }
    let entrada = cpm6_d(this + 0x1c).wrapping_add(cpm6_d(clube).wrapping_mul(0x6a));
    if *((entrada + 0x10) as *const u8) == 0 {
        return 0;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let r = temporada(this, clube, 0);
    if r == 0 {
        return 0;
    }
    let ligado: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    let estatistica: unsafe extern "stdcall" fn(usize, u32, f32) -> f64 = core::mem::transmute(0x7c2150usize);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    let nota = (r + 0x1f1) as *mut f32;
    cpm6_por(r + 0x1f1, 0xbf800000); // −1.0f
    let mut achou = 0;
    for k in 0..50usize {
        let p = cpm6_d(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let est = cpm6_d(p + 0x61);
        if est == 0 || cpm6_d(p) as i32 >= cpm6_d(0xae2364) as i32 {
            continue;
        }
        if cpm6_d(p + 0x39) != clube && cpm6_d(p + 0x39) != ligado(clube, 0, 1) {
            continue;
        }
        let reg = registro(cpm6_d(0xb74350), est, 6);
        if reg == 0 {
            continue;
        }
        let jogou = ftol(estatistica(reg, 1, 6.5)) as u8;
        if (jogou as i32) < *((entrada + 0x10) as *const i8) as i32 / 2 { // menos da metade dos jogos
            continue;
        }
        let media = estatistica(reg, 0x11, 6.5);
        // fcomp com o −1.0 (double) e fcom com o float guardado: passa só com a média maior
        if *nota as f64 == f64_em(0x968948) || media > *nota as f64 {
            crate::comparacao::ramo("cpm_6 recorde de nota da temporada");
            *nota = media as f32;
            *((r + 0x20d) as *mut u8) = jogou;
            referencia(p, r + 0x1f5);
            cpm6_por_w(r + 0x20e, 0);
            achou = 1;
        }
    }
    achou
}

/// Comparador: os registros podem ser criados (realloc) e a notícia realoca: os quatro registros são pedidos antes, na
/// ordem da original (vendedor: temporada, sempre; comprador: temporada, sempre; a segunda vez só acha), e com a fila
/// de notícias sem folga para duas, só a original. Lado a lado pelos quatro registros (+0x365, 12 bytes), a conta das
/// notícias e as duas vagas seguintes.
unsafe extern "thiscall" fn comparar_somar_totais_de_transferencia(this: usize, valor: i32, comprador: usize, vendedor: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32>(940);
    if comprador == 0 || vendedor == 0 || (cpm6_d(this + 0x28).wrapping_sub(cpm6_d(this + 0x2c)) as i32) < 2 {
        return o(this, valor, comprador, vendedor); // ponytail: com realloc da fila, só a original
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let regs = [temporada(this, vendedor, 1), sempre(this, vendedor, 1), temporada(this, comprador, 1), sempre(this, comprador, 1)];
    let fila = cpm6_d(this + 0x24).wrapping_add(cpm6_d(this + 0x2c).wrapping_mul(6));
    let reg = [(regs[0] + 0x365, 12), (regs[1] + 0x365, 12), (regs[2] + 0x365, 12), (regs[3] + 0x365, 12), (this + 0x2c, 4), (fila, 12)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, valor, comprador, vendedor), || somar_totais_de_transferencia(this, valor, comprador, vendedor));
    PLACAR.registrar(igual && r == n, || format!("somar_totais_de_transferencia({}, {}, {}) {:x?} x {:x?}", valor, cpm6_d(comprador), cpm6_d(vendedor), mo, mr));
    r
}

/// Comparador: o registro de sempre é pedido antes (pode ser criado; a segunda vez só acha); lado a lado pelos dois
/// recordes e as referências (+0x247 a +0x27b). Nulos ou sem registro: só a original (a caixa).
unsafe extern "thiscall" fn comparar_recordes_de_carreira_no_clube(this: usize, jogador: usize, clube: usize, jogos: u32, gols: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(941);
    if jogador == 0 || clube == 0 {
        return o(this, jogador, clube, jogos, gols);
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let reg = sempre(this, clube, 1);
    if reg == 0 {
        return o(this, jogador, clube, jogos, gols);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(reg + 0x247, 0x34)], || o(this, jogador, clube, jogos, gols), || recordes_de_carreira_no_clube(this, jogador, clube, jogos, gols));
    PLACAR.registrar(igual && r == n, || format!("recordes_de_carreira_no_clube({}, {}, {}, {}) {:x?} x {:x?}", cpm6_d(jogador), cpm6_d(clube), jogos as i16, gols as i16, mo, mr));
    r
}

/// Comparador: lado a lado pelo recorde de nota (+0x1f1 a +0x210 do registro da temporada, pedido antes sem criar).
unsafe extern "thiscall" fn comparar_recorde_de_nota_da_temporada(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(942);
    if clube == 0 {
        return o(this, clube);
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let t = temporada(this, clube, 0);
    let reg: &[(usize, usize)] = if t == 0 { &[] } else { &[(t + 0x1f1, 0x1f)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube), || recorde_de_nota_da_temporada(this, clube));
    PLACAR.registrar(igual && r == n, || format!("recorde_de_nota_da_temporada({}) original={} rust={} {:x?} x {:x?}", cpm6_d(clube), r, n, mo, mr));
    r
}

// ---------- 249a leva: os amistosos: se a competição vale na data, se o clube já tem amistoso perto da data e acrescentar o amistoso planejado (friendly.cpp) ----------

unsafe fn cpm7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm7_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// (dia, ano) de uma data (+0 dia, palavra com sinal; +2 ano, palavra sem sinal).
unsafe fn cpm7_data(a: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(a as *const i16), core::ptr::read_unaligned((a + 2) as *const u16))
}

/// { 0x5c29b0 } (thiscall, ret 8, friendly.cpp): `Friendly::is_comp_current_on_date(comp, data)`: competição que não
/// é a do amistoso ([this+4]) e de id abaixo de [0xae2378] − 0x7f: 1. Senão, a vaga do torneio (this + 0x69 × (byte do
/// id − byte +0xaf), +0xb1 ocupada, +0xb5 a data): 1 se a `data` está depois da data da vaga − 7 dias (0x54c8e0) e
/// antes dela + 7 (0x54c5b0), pelo ano (sem sinal) e depois o dia (com sinal); vaga vazia: 0. O quadro SEH só guarda
/// as datas locais (os destrutores são vazios).
unsafe extern "thiscall" fn competicao_na_data_do_amistoso(this: usize, comp: usize, data: usize) -> u32 {
    if comp != cpm7_d(this + 4) && (cpm7_d(comp) as i32) < (cpm7_d(0xae2378) as i32).wrapping_sub(0x7f) {
        return 1;
    }
    let k = (*(comp as *const u8)).wrapping_sub(*((this + 0xaf) as *const u8)) as i8 as isize;
    let e = this.wrapping_add((k * 0x69) as usize);
    if cpm7_d(e + 0xb1) == 0 {
        return 0;
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (dia, ano) = cpm7_data(data);
    let mut a = [0u8; 8];
    let (adia, aano) = cpm7_data(menos(e + 0xb5, &mut a, 7));
    if ano < aano || (ano == aano && dia <= adia) {
        return 0;
    }
    let (dia, ano) = cpm7_data(data);
    let mut b = [0u8; 8];
    let (bdia, bano) = cpm7_data(mais(e + 0xb5, &mut b, 7));
    (ano < bano || (ano == bano && dia < bdia)) as u32
}

/// { 0x5c45a0 } (thiscall sem usar o this, ret 8, friendly.cpp): `Friendly::candidate_has_nearby_date(clube, data)`:
/// 1 se a lista de amistosos planejados do candidato (+0x12; itens com dia +0 e ano +2) tem a `data`, o dia antes
/// (0x54c8e0) ou o dia depois (0x54c5b0). Percorre do começo (o cursor +0xa = a cabeça +2; 0x68d1b0). Sem lista: 0. As
/// datas locais vêm antes (como a original); o quadro SEH é delas (destrutores vazios).
unsafe extern "thiscall" fn amistoso_perto_da_data(_this: usize, clube: usize, data: usize) -> u32 {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (mut antes, mut depois) = ([0u8; 8], [0u8; 8]);
    menos(data, &mut antes, 1);
    mais(data, &mut depois, 1);
    let l = cpm7_d(clube + 0x12);
    if l == 0 {
        return 0;
    }
    cpm7_por(l + 0xa, cpm7_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let antes = cpm7_data(antes.as_ptr() as usize);
    let depois = cpm7_data(depois.as_ptr() as usize);
    let mut e = proximo(l);
    while e != 0 {
        let item = cpm7_data(e);
        if item == cpm7_data(data) {
            return 1;
        }
        if item == antes {
            crate::comparacao::ramo("cpm_7 amistoso no dia antes");
            return 1;
        }
        if item == depois {
            crate::comparacao::ramo("cpm_7 amistoso no dia depois");
            return 1;
        }
        e = proximo(l);
    }
    0
}

/// { 0x5c4b60 } (thiscall, ret 0x10, friendly.cpp): `Friendly::add_planned_friendly(anfitriao, mandante, visitante,
/// data)`: acrescenta ao vetor de 0x14 bytes [this+0x34d8] (conta +0x34e0, capacidade +0x34e4; cheio: cresce de 100
/// em 100, realloc 0x945667; sem memória, a caixa "Error" 0x797 e volta) o amistoso: a data em +0 (0x417c10, a cópia
/// devolvida fica na pilha) e os ids (o dword apontado por cada argumento) em +8, +0xc e +0x10; a conta sobe.
unsafe extern "thiscall" fn acrescentar_amistoso_planejado(this: usize, anfitriao: usize, mandante: usize, visitante: usize, data: usize) {
    if cpm7_d(this + 0x34e0) == cpm7_d(this + 0x34e4) {
        crate::comparacao::ramo("cpm_7 amistosos crescem");
        let cap = cpm7_d(this + 0x34e4).wrapping_add(100);
        cpm7_por(this + 0x34e4, cap);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpm7_d(this + 0x34d8), cap.wrapping_mul(0x14) as u32);
        cpm7_por(this + 0x34d8, p);
        if p == 0 {
            aviso_em(0x9cd0a0, 0x797);
            return;
        }
    }
    crate::comparacao::ramo("cpm_7 amistoso planejado");
    let n = cpm7_d(this + 0x34e0);
    let item = cpm7_d(this + 0x34d8).wrapping_add(n.wrapping_mul(0x14));
    cpm7_por(item + 8, cpm7_d(anfitriao));
    cpm7_por(item + 0xc, cpm7_d(mandante)); // o mandante
    cpm7_por(item + 0x10, cpm7_d(visitante)); // o visitante
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut copia = [0u8; 8];
    copiar(item, &mut copia, data);
    cpm7_por(this + 0x34e0, n.wrapping_add(1)); // mais um amistoso
}

/// Comparador: pelo valor (as datas e a vaga só são lidas). Chamada pelas copas e ligas a cada data: sem contador.
unsafe extern "thiscall" fn comparar_competicao_na_data_do_amistoso(this: usize, comp: usize, data: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(943)(this, comp, data);
    let n = competicao_na_data_do_amistoso(this, comp, data);
    PLACAR.registrar(r == n, || format!("competicao_na_data_do_amistoso({}, {:?}) original={} rust={}", cpm7_d(comp), cpm7_data(data), r, n));
    r
}

/// Comparador: lado a lado pelo cursor da lista (+0xa) e o valor.
unsafe extern "thiscall" fn comparar_amistoso_perto_da_data(this: usize, clube: usize, data: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(944);
    let l = cpm7_d(clube + 0x12);
    let reg: &[(usize, usize)] = if l == 0 { &[] } else { &[(l + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube, data), || amistoso_perto_da_data(this, clube, data));
    PLACAR.registrar(igual && r == n, || format!("amistoso_perto_da_data({:?}) original={} rust={} {:x?} x {:x?}", cpm7_data(data), r, n, mo, mr));
    r
}

/// Comparador: a original acrescenta (pode realocar, não roda duas vezes): o plano antes (a conta, os ids e a data),
/// a original roda, confere o item novo e a conta.
unsafe extern "thiscall" fn comparar_acrescentar_amistoso_planejado(this: usize, anfitriao: usize, mandante: usize, visitante: usize, data: usize) {
    let n = cpm7_d(this + 0x34e0);
    let esperado = [cpm7_d(data), cpm7_d(data + 4), 0, cpm7_d(anfitriao), cpm7_d(mandante), cpm7_d(visitante)];
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(945)(this, anfitriao, mandante, visitante, data);
    let v = cpm7_d(this + 0x34d8);
    if v == 0 {
        return; // sem memória: a caixa já saiu
    }
    let item = v.wrapping_add(n.wrapping_mul(0x14));
    let feito = [cpm7_d(item), cpm7_d(item + 4), 0, cpm7_d(item + 8), cpm7_d(item + 0xc), cpm7_d(item + 0x10)];
    PLACAR.registrar(feito == esperado && cpm7_d(this + 0x34e0) == n.wrapping_add(1), || {
        format!("acrescentar_amistoso_planejado conta {} -> {}, item {:x?} (esperava {:x?})", n, cpm7_d(this + 0x34e0), feito, esperado)
    });
}
