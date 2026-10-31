// ---------- 254a leva: os limites da escalação: a tabela de restrições da partida, o limite do banco do dia e quem precisa de permissão de trabalho (transfer_manager.cpp) ----------

unsafe fn cpr1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Os 42 bytes da tabela de restrições com 0xff (o `rep stosd` de 10 duplas e o `stosw` da original).
unsafe fn cpr1_limpar(t: *mut u8) {
    core::ptr::write_bytes(t, 0xff, 42);
}

/// { 0x8cf6b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::fill_squad_restrictions(partida, t,
/// so_a_principal)`: enche os 42 bytes de `t` com 0xff. Com partida: o alcance das regras (0x8d2fa0); 0xff para aí. Fora
/// do tipo 4, a primeira das 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo
/// (+0x12) enche a tabela (virtual +0x1c com a partida e `t`). Com `so_a_principal`, fica só o primeiro limite que
/// existe: +5 ou +3 (gravado em +5), +2 ou +0 (gravado em +2), ou +0x16 com +0x17; o resto volta a 0xff.
unsafe extern "thiscall" fn restricoes_da_partida(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    cpr1_limpar(t);
    if partida == 0 {
        return;
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(0x8d2fa0usize);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff {
        return;
    }
    if tipo != 4 {
        for k in 0..28 {
            let r = cpr1_d(this + 0x8ac + k * 4);
            let casa = if tipo == 2 {
                cpr1_d(r + 0xe) == continente && cpr1_b(r + 0x12) == 2
            } else {
                cpr1_d(r + 0xa) == nacao && cpr1_b(r + 0x12) == tipo
            };
            if casa {
                let encher: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(cpr1_d(cpr1_d(r) + 0x1c));
                encher(r, partida, t);
                break;
            }
        }
    }
    if so_a_principal == 0 {
        return;
    }
    let v = if *t.add(5) != 0xff { *t.add(5) } else { *t.add(3) };
    if v != 0xff {
        crate::comparacao::ramo("cpr_1 limite de nao comunitarios");
        cpr1_limpar(t);
        *t.add(5) = v; // o limite de não comunitários
        return;
    }
    let v = if *t.add(2) != 0xff { *t.add(2) } else { *t };
    if v != 0xff {
        cpr1_limpar(t);
        *t.add(2) = v;
        return;
    }
    let (idade, n) = (*t.add(0x16), *t.add(0x17));
    cpr1_limpar(t);
    if idade != 0xff {
        *t.add(0x16) = idade;
        *t.add(0x17) = n;
    }
}

/// { 0x8cf880 } (thiscall pelo `this` em ecx, que a original passa direto à 0x8cf7e0; ret 0x18, transfer_manager.cpp):
/// `transfer_manager_check_matchday_squad_limit(clube, partida, msg, jogadores, _, saida)`: o limite da partida
/// (0x8cf7e0 em 6 bytes locais: +0 tipo, +1 idade, +5 quantidade); sem tipo, 0. O lado humano (+0x1c/+0x40 ou
/// +0x20/+0x41) vai para a 0x8968f0. Pelas (+0x4d & 0xf) + 11 vagas, a pessoa da vaga (`jogadores[k]` ou a tática
/// [0xde1d08] 0x8968f0) conta se é estrangeira na nação do clube (0x53d980 com 1; tipo 1, também fora da UE, 0x53d740)
/// ou passa da idade (+0x18, tipo 3): +2 (titulares) ou +3 (as outras) e +4 (todas). A conta vai a `saida`; passando da
/// quantidade, 1 e, com `msg`, o aviso (0x66f4e0) pelo tipo, pela conta e pela quantidade.
unsafe extern "thiscall" fn limite_do_banco_do_dia(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, _nao_usado: u32, saida: usize) -> u32 {
    let limite: unsafe extern "thiscall" fn(usize, usize, *mut u8) = core::mem::transmute(0x8cf7e0usize);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8968f0usize);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut l = [0u8; 6];
    limite(this, partida, l.as_mut_ptr());
    let tipo = l[0];
    if tipo == 0 {
        return 0;
    }
    let humano = ((clube == cpr1_d(partida + 0x1c) && cpr1_b(partida + 0x40) == 0) || (clube == cpr1_d(partida + 0x20) && cpr1_b(partida + 0x41) == 0)) as u32;
    if tipo == 1 {
        crate::comparacao::ramo("cpr_1 conta nao comunitarios");
    }
    let mut k: i8 = 0;
    while k < ((cpr1_b(partida + 0x4d) & 0xf).wrapping_add(0xb)) as i8 {
        let p = if jogadores == 0 {
            // ponytail: a original empurra a dupla da partida com o byte baixo trocado pela vaga (a 0x8968f0 lê o byte)
            da_tatica(cpr1_d(0xde1d08), clube, (partida & !0xff) | k as u8 as usize, humano)
        } else {
            cpr1_d(jogadores + (k as isize * 4) as usize)
        };
        if p != 0 {
            let conta = match tipo {
                2 => estrangeira(p, cpr1_d(clube + 0x53), 1) != 0,
                1 => estrangeira(p, cpr1_d(clube + 0x53), 1) != 0 && comunitaria(p, cpr1_d(clube + 0x53)) == 0,
                3 => cpr1_b(p + 0x18) as i8 > l[1] as i8,
                _ => false,
            };
            if conta {
                if k < 11 {
                    l[2] = l[2].wrapping_add(1);
                } else {
                    l[3] = l[3].wrapping_add(1);
                }
                l[4] = l[4].wrapping_add(1);
            }
        }
        k += 1;
    }
    if saida != 0 {
        core::ptr::copy_nonoverlapping(l.as_ptr(), saida as *mut u8, 6);
    }
    let (c, n) = (l[4] as i8, l[5] as i8);
    if !(1..=3).contains(&tipo) || c <= n {
        return 0;
    }
    if msg == 0 {
        return 1;
    }
    if tipo == 3 {
        if c > 1 {
            texto(msg, 0xa8ee08, c as i32, l[1] as i8 as i32);
        } else {
            texto(msg, 0xa8edbc, l[1] as i8 as i32);
        }
        let mut fim = msg; // o fim do texto (o `repne scasb` da original)
        while cpr1_b(fim) != 0 {
            fim += 1;
        }
        if n > 1 {
            texto(fim, 0xa8ed8c, n as i32);
        } else if n == 1 {
            texto(fim, 0xa8ed70);
        } else {
            texto(fim, 0xa8ed48);
        }
        return 1;
    }
    // os textos do tipo 1 (não comunitários) e do tipo 2 (estrangeiros)
    let f: [usize; 5] = if tipo == 1 { [0xa8f208, 0xa8f19c, 0xa8f134, 0xa8f0cc, 0xa8f07c] } else { [0xa8effc, 0xa8ef90, 0xa8ef28, 0xa8eebc, 0xa8ee68] };
    if c > 1 {
        if n > 1 {
            texto(msg, f[0], c as i32, n as i32);
        } else if n == 1 {
            texto(msg, f[1], c as i32);
        } else {
            texto(msg, f[2], c as i32);
        }
    } else if n == 1 {
        texto(msg, f[3], c as i32);
    } else if n == 0 {
        texto(msg, f[4]);
    }
    1
}

/// { 0x8c7560 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::player_needs_work_permit(jogador, clube,
/// ignorar_a_dada)`: 0 sem clube ou sem a nação dele (+0x53); jogador nulo: a caixa "Error" 0x1c23 e 0. Nação sem a marca
/// 0xe (+0x76) ou jogador da UE (0x53d740 com nação 0): 0. Com `ignorar_a_dada`: 1. Senão 0 só se o contrato (0x4cf5a0)
/// é do tipo 2 (+0x35 & 0x3f), o id cabe em [0xae2364], tem a marca 0x20 ([this+0x86e] + id × 8) e a permissão está no
/// vetor [this+0x93a] (conta +0x93e; busca binária 0x946464 com 0x8e55c0); senão 1.
unsafe extern "thiscall" fn precisa_de_permissao_de_trabalho(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    if clube == 0 || cpr1_d(clube + 0x53) == 0 {
        return 0;
    }
    if jogador == 0 {
        aviso_em(0xa8d514, 0x1c23);
        return 0;
    }
    if cpr1_b(cpr1_d(clube + 0x53) + 0x76) != 0xe {
        return 0;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    if comunitaria(jogador, 0) != 0 {
        return 0;
    }
    if ignorar_a_dada != 0 {
        return 1;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpr1_d(0xae1050), jogador);
    if c != 0 && cpr1_b(c + 0x35) & 0x3f == 2 {
        let id = cpr1_d(jogador) as i32;
        if id < cpr1_d(0xae2364) as i32 && cpr1_b(cpr1_d(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) & 0x20 != 0 {
            let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
            if bsearch(&id, cpr1_d(this + 0x93a), cpr1_d(this + 0x93e), 0xe, 0x8e55c0) != 0 {
                crate::comparacao::ramo("cpr_1 permissao achada");
                return 0; // tem a permissão
            }
        }
    }
    1
}

/// Comparador: lado a lado pelos 42 bytes da tabela (o alcance e a virtual das regras só escrevem nela).
unsafe extern "thiscall" fn comparar_restricoes_da_partida(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32)>(1083);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(t as usize, 42)], || o(this, partida, t, so_a_principal), || restricoes_da_partida(this, partida, t, so_a_principal));
    PLACAR.registrar(igual, || format!("restricoes_da_partida({:x}, {}) {:x?} x {:x?}", partida, so_a_principal, mo, mr));
}

/// Comparador: lado a lado pela saída (6 bytes) e pelo aviso (0x100 bytes, com `msg`), e pela devolução. A 0x8968f0 pode
/// pôr −1 numa vaga sem pessoa válida: a original já deixou assim e a cópia acha a mesma vaga vazia.
unsafe extern "thiscall" fn comparar_limite_do_banco_do_dia(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, nao_usado: u32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize) -> u32>(1084);
    let mut regioes = [(0usize, 0usize); 2];
    let mut n = 0;
    if saida != 0 {
        regioes[n] = (saida, 6);
        n += 1;
    }
    if msg != 0 {
        regioes[n] = (msg, 0x100);
        n += 1;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes[..n], || o(this, clube, partida, msg, jogadores, nao_usado, saida), || limite_do_banco_do_dia(this, clube, partida, msg, jogadores, nao_usado, saida));
    PLACAR.registrar(igual && r == c, || format!("limite_do_banco_do_dia(clube {:x}, partida {:x}) original={} rust={} {:x?} x {:x?}", clube, partida, r, c, mo, mr));
    r
}

/// Comparador: pelo valor (só lê). Jogador nulo com clube de nação: a caixa, só a original.
unsafe extern "thiscall" fn comparar_precisa_de_permissao_de_trabalho(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1085);
    if jogador == 0 {
        return o(this, jogador, clube, ignorar_a_dada);
    }
    let r = o(this, jogador, clube, ignorar_a_dada);
    let n = precisa_de_permissao_de_trabalho(this, jogador, clube, ignorar_a_dada);
    PLACAR.registrar(r == n, || format!("precisa_de_permissao_de_trabalho(jogador {}, clube {:x}, {}) original={} rust={}", cpr1_d(jogador), clube, ignorar_a_dada, r, n));
    r
}

// ---------- 254a leva: quem pode negociar: se o clube pode chegar ao jogador, os termos mandados ao jogador da troca e a fila da transferência livre do clube (transfer_manager.cpp) ----------

unsafe fn cpr2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// Os itens de uma LLIST, do começo (o cursor +0xa volta a +2 e fica no fim, como depois de uma volta da original).
unsafe fn cpr2_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    cpr2_por(l + 0xa, cpr2_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O corpo da 0x8ce990 sobre a lista local `l` (já construída): devolve (motivo, pode).
unsafe fn cpr2_pode(this: usize, jogador: usize, clube: usize, ignorar_abertas: u32, l: usize) -> (u16, u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let acertada: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x4cca30usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x8c5fc0usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let nat = cpr2_d(jogador + 0x1a);
    if nat != 0 && cpr2_d(nat) == cpr2_d(0x9cf458) {
        let cc = cpr2_d(jogador + 0x39);
        if (cc == 0 || cpr2_d(cc + 0x53) == nat) && clube != 0 && cpr2_d(clube + 0x53) != nat {
            return (0x27, 0); // não pode sair da nação
        }
    }
    let marca = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8));
    if cpr2_b(marca) & 0x40 != 0 {
        if acertada(cpr2_d(0xae1050), cpr2_d(jogador) as i32) != 0 {
            crate::comparacao::ramo("cpr_2 contrato acertado");
            return (0x21, 0); // contrato acertado
        }
        let m = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8)) as *mut u8;
        *m &= 0xbf;
    }
    if ignorar_abertas == 0 && (trocas(this, jogador, 0) != 0 || negociacoes(this, jogador, 0) != 0) {
        return (7, 0);
    }
    // a fila da transferência livre (+0x92c)
    let fila = this + 0x92c;
    cpr2_por(fila + 0xa, cpr2_d(fila + 2));
    let mut e = proximo(fila);
    while e != 0 {
        if cpr2_d(jogador) == cpr2_d(e) {
            return (1, 0);
        }
        e = proximo(fila);
    }
    if sob_oferta(this, jogador, 0) != 0 {
        ofertas(this, jogador, l, 0);
        cpr2_por(l + 0xa, cpr2_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            let k = cpr2_d(e) as i32;
            let vaga = if k < cpr2_d(this + 4) as i32 { cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)) } else { cpr2_d(this) };
            let o = cpr2_d(vaga);
            let st = cpr2_b(o + 0x2c);
            if (0xa..=0xe).contains(&st) {
                crate::comparacao::ramo("cpr_2 oferta em negociacao");
                *((o + 0x2d) as *mut u8) = 1;
                return (0x12, 0); // já em negociação
            }
            if st == 0x13 {
                crate::comparacao::ramo("cpr_2 transferencia futura");
                return (0x20, 0); // transferência futura acertada
            }
            e = proximo(l);
        }
    }
    if clube == 0 || jogador == 0 || cpr2_d(jogador + 0x61) == 0 {
        return (0, 1);
    }
    if basco(clube) != 0 {
        let nacao = cpr2_d(0xae23a8).wrapping_add(cpr2_d(0x9cf424).wrapping_mul(0x122));
        if estrangeira(jogador, nacao, 1) != 0 {
            return (0x19, 0);
        }
        return (0, 1);
    }
    let n = cpr2_d(clube + 0x53);
    if n != 0 && cpr2_d(n) == cpr2_d(0x9cf4dc) {
        let c57 = cpr2_d(clube + 0x57);
        if c57 == 0 || cpr2_d(c57) != cpr2_d(0x9cf8d8) {
            if estrangeira(jogador, n, 1) != 0 {
                return (0x28, 0);
            }
            return (0, 1);
        }
    }
    let id = cpr2_d(clube);
    if id == cpr2_d(0x9d0128) || id == cpr2_d(0x9d01a0) {
        let n1 = cpr2_d(jogador + 0x1a);
        if n1 == 0 || cpr2_d(n1) == cpr2_d(0x9cf488) {
            let n2 = cpr2_d(jogador + 0x1e);
            if n2 == 0 || cpr2_d(n2) != cpr2_d(0x9cf424) {
                return (0x1a, 0);
            }
        }
    }
    (0, 1)
}

/// { 0x8ce990 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::can_club_bid_for_player(jogador, clube, motivo, ignorar_abertas)`: não pode (0 e o motivo) se o
/// jogador é da nação [0x9cf458] sem clube ou com clube dela e o clube é de fora (0x27); com a marca de contrato
/// acertado (0x40) e o acerto (0x4cca30) (0x21; sem acerto, a marca sai); sem `ignorar_abertas`, com trocas que o
/// oferecem (0x8c5e40) ou negociações de troca (0x8c5fc0) (7); na fila da transferência livre (+0x92c) (1); sob oferta
/// (0x8cde70), pelas ofertas por ele (0x8c5cf0): uma em negociação (estados 0xa a 0xe; marca +0x2d) (0x12) ou uma
/// transferência futura (0x13) (0x20). Pelo clube: basco (0x53d7f0) com jogador estrangeiro na nação [0x9cf424] (0x19);
/// da nação [0x9cf4dc] fora da [0x9cf8d8] (+0x57) com jogador estrangeiro (0x28); os clubes [0x9d0128]/[0x9d01a0] com
/// jogador sem a nação [0x9cf424] (0x1a). Senão pode (1, motivo 0).
unsafe extern "thiscall" fn clube_pode_chegar_ao_jogador(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let (m, r) = cpr2_pode(this, jogador, clube, ignorar_abertas, l);
    if !motivo.is_null() {
        *motivo = m;
    }
    soltar(l);
    r
}

/// { 0x8c2060 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::note_terms_offered_to_exchange_player(k,
/// pessoa)`: `k` negativo ou fora da conta (+4): a caixa "Error" (0xdc3 ou 0xdc8). Na oferta `k` (vazia: nada), pela
/// lista dos jogadores da troca (+4), marca (+4 do item) o da `pessoa`; nenhum item marcado: o estado (+0x2c) vira 8 (se
/// era 7) ou 6.
unsafe extern "thiscall" fn termos_ao_jogador_da_troca(this: usize, k: i32, pessoa: u32) {
    if k < 0 {
        aviso_em(0xa8d514, 0xdc3);
        return;
    }
    if k >= cpr2_d(this + 4) as i32 {
        aviso_em(0xa8d514, 0xdc8);
        return;
    }
    let o = cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)));
    if o == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cpr2_d(o + 4);
    cpr2_por(l + 0xa, cpr2_d(l + 2));
    let mut nenhum = true;
    let mut e = proximo(cpr2_d(o + 4));
    while e != 0 {
        if cpr2_d(e) as u32 == pessoa {
            *((e + 4) as *mut u8) = 1; // termos mandados
        }
        if cpr2_b(e + 4) != 0 {
            nenhum = false;
        }
        e = proximo(cpr2_d(o + 4));
    }
    if nenhum {
        *((o + 0x2c) as *mut u8) = if cpr2_b(o + 0x2c) == 7 { 8 } else { 6 };
    }
}

/// { 0x8c1780 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::count_club_free_transfer_queue(clube)`:
/// clube nulo: a caixa "Error" 0xc04 e 0. Senão conta as pessoas da fila da transferência livre (+0x92c; o cursor volta
/// ao começo) com o clube do contrato (+0x39) igual ao clube ou ao parceiro dele (0x540a50 com 0 e 1), não nulo.
unsafe extern "thiscall" fn fila_livre_do_clube(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        aviso_em(0xa8d514, 0xc04);
        return 0;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let p = parceiro(clube, 0, 1);
    cpr2_por(this + 0x936, cpr2_d(this + 0x92e));
    let mut n = 0u8;
    let mut e = proximo(this + 0x92c);
    while e != 0 {
        let c = cpr2_d(cpr2_d(0xae23bc).wrapping_add(cpr2_d(e).wrapping_mul(0x6e)) + 0x39);
        if c == clube || (c == p && p != 0) {
            n = n.wrapping_add(1);
        }
        e = proximo(this + 0x92c);
    }
    n
}

/// Comparador: lado a lado pelo motivo, pela marca da pessoa (+0x86e) e pelo cursor da fila livre, e pela devolução.
/// As outras gravações (tirar a marca 0x40, marcar +0x2d da oferta em negociação) a cópia repete sobre o que a original
/// deixou e chega ao mesmo.
unsafe extern "thiscall" fn comparar_clube_pode_chegar_ao_jogador(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32>(1086);
    let marca = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8));
    let regioes = [(marca, 1), (this + 0x936, 4), (motivo as usize, 2)];
    let n = if motivo.is_null() { 2 } else { 3 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, clube, motivo, ignorar_abertas), || clube_pode_chegar_ao_jogador(this, jogador, clube, motivo, ignorar_abertas));
    PLACAR.registrar(igual && r == c, || format!("clube_pode_chegar_ao_jogador(jogador {}, clube {:x}, {}) original={} rust={} {:x?} x {:x?}", cpr2_d(jogador), clube, ignorar_abertas, r, c, mo, mr));
    r
}

/// Comparador: lado a lado pelo estado da oferta, pelas marcas (+4) dos itens da troca e pelo cursor da lista. Fora da
/// conta: a caixa, só a original.
unsafe extern "thiscall" fn comparar_termos_ao_jogador_da_troca(this: usize, k: i32, pessoa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, u32)>(1087);
    if k < 0 || k >= cpr2_d(this + 4) as i32 || cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4))) == 0 {
        return o(this, k, pessoa);
    }
    let of = cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)));
    let l = cpr2_d(of + 4);
    let mut regioes = vec![(of + 0x2c, 1), (l + 0xa, 4)];
    for e in cpr2_itens(l) {
        regioes.push((e + 4, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, k, pessoa), || termos_ao_jogador_da_troca(this, k, pessoa));
    PLACAR.registrar(igual, || format!("termos_ao_jogador_da_troca({}, pessoa {}) {:x?} x {:x?}", k, pessoa, mo, mr));
}

/// Comparador: lado a lado pelo cursor da fila e pela devolução (só lê). Clube nulo: a caixa, só a original.
unsafe extern "thiscall" fn comparar_fila_livre_do_clube(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1088);
    if clube == 0 {
        return o(this, clube);
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x936, 4)], || o(this, clube), || fila_livre_do_clube(this, clube));
    PLACAR.registrar(igual && r == c, || format!("fila_livre_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

// ---------- 254a leva: as transferências futuras: a do jogador, tirar a da oferta e acrescentar a da oferta acertada (transfer_manager.cpp) ----------

unsafe fn cpr3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpr3_pessoa(id: usize) -> usize {
    cpr3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O byte de marcas da pessoa de id `id` no registro de 8 bytes em [this+0x86e].
unsafe fn cpr3_marca(this: usize, id: usize) -> *mut u8 {
    cpr3_d(this + 0x86e).wrapping_add(id.wrapping_mul(8)) as *mut u8
}

/// Na lista de índices de ofertas `l` (o cursor volta ao começo), o índice da primeira oferta de estado 0x13
/// (transferência futura), ou None.
unsafe fn cpr3_futura_na_lista(this: usize, l: usize) -> Option<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    cpr3_por(l + 0xa, cpr3_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if cpr3_b(cpr3_d(cpr3_d(this).wrapping_add(cpr3_d(e).wrapping_mul(4))) + 0x2c) == 0x13 {
            return Some(cpr3_d(e));
        }
        e = proximo(l);
    }
    None
}

/// O corpo da 0x8cca10 sobre a lista local `l` (já construída).
unsafe fn cpr3_futura(this: usize, jogador: usize, saida: *mut usize, l: usize) -> u32 {
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    if *cpr3_marca(this, cpr3_d(jogador)) & 0x10 == 0 {
        return 0;
    }
    let mut k = None;
    if ofertas(this, jogador, l, 0) != 0 {
        k = cpr3_futura_na_lista(this, l).filter(|&i| i != usize::MAX);
    }
    if k.is_none() {
        limpar(l);
        if trocas(this, jogador, l) != 0 {
            k = cpr3_futura_na_lista(this, l).filter(|&i| i != usize::MAX);
        }
    }
    let Some(k) = k else {
        // ponytail: a marca é relida pela original depois das chamadas; o vetor [this+0x86e] não muda entre elas
        *cpr3_marca(this, cpr3_d(jogador)) &= 0xef; // sem transferência futura: a marca sai
        return 0;
    };
    crate::comparacao::ramo("cpr_3 futura achada");
    if !saida.is_null() {
        let n = cpr3_d(this + 0x928) as i32;
        for i in 0..n.max(0) as usize {
            let r = cpr3_d(this + 0x920).wrapping_add(i * 0xc);
            if cpr3_d(r + 8) == k {
                *saida = r;
                break;
            }
        }
    }
    1 // a transferência futura existe
}

/// { 0x8cca10 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::get_future_transfer(jogador, saida)`: sem a marca 0x10 ([this+0x86e] + id × 8), 0. Pelas ofertas
/// pelo jogador (0x8c5cf0) e, sem achar, pelas trocas que o oferecem (0x8c5e40, com a lista esvaziada por 0x68d080), a
/// primeira de estado 0x13 com índice ≠ −1; com `saida`, o registro de 12 bytes do vetor [this+0x920] (conta +0x928)
/// com a oferta em +8; 1. Nenhuma: tira a marca 0x10 e 0.
unsafe extern "thiscall" fn transferencia_futura_do_jogador(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = cpr3_futura(this, jogador, saida, l);
    soltar(l);
    r
}

/// { 0x8cbf70 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_future_transfer(oferta)`: tira a
/// marca 0x10 do jogador (+0xc) e dos jogadores da troca (lista +4, o cursor volta ao começo); no vetor [this+0x920]
/// (conta +0x928), o primeiro registro com a oferta (+0x20 da oferta, +8 do registro) sai: os seguintes descem um
/// (0x8cbf40) e a conta cai de 1.
unsafe extern "thiscall" fn tirar_transferencia_futura(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cbf40usize);
    *cpr3_marca(this, cpr3_d(oferta + 0xc)) &= 0xef;
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(cpr3_d(oferta + 4));
        while e != 0 {
            *cpr3_marca(this, cpr3_d(e)) &= 0xef;
            e = proximo(cpr3_d(oferta + 4));
        }
    }
    let n = cpr3_d(this + 0x928) as i32;
    let alvo = cpr3_d(oferta + 0x20);
    for i in 0..n.max(0) {
        if cpr3_d(cpr3_d(this + 0x920).wrapping_add(i as usize * 0xc) + 8) == alvo {
            crate::comparacao::ramo("cpr_3 tirar futura");
            let mut j = i;
            while j < cpr3_d(this + 0x928) as i32 - 1 {
                let r = cpr3_d(this + 0x920).wrapping_add(j as usize * 0xc);
                copiar(r, r + 0xc);
                j += 1;
            }
            cpr3_por(this + 0x928, cpr3_d(this + 0x928).wrapping_sub(1));
            return;
        }
    }
}

/// { 0x8cb9a0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::add_future_transfer(oferta, data)`:
/// acrescenta ao vetor [this+0x920] (capacidade +0x924, conta +0x928; cheio, cresce de 10 em 10, sem memória: a caixa
/// 0x27c3 e nada) o registro de 12 bytes com a `data` (0x417c10) e a oferta (+0x20 em +8) e reordena (qsort 0x945333
/// com 0x778e90). A oferta fica com +0x2d = 1 e o estado 0x13. Suspende os clubes que miram o jogador (0x842b00 de
/// [this+0x856], com 0) e liga a marca 0x10 dele; com contrato (0x4cf5a0) marcado em +0x49, a oferta vai a +0x4a do
/// contrato. O mesmo (suspender e marcar) para cada jogador da troca (lista +4).
unsafe extern "thiscall" fn acrescentar_transferencia_futura(this: usize, oferta: usize, data: usize) {
    crate::comparacao::ramo("cpr_3 acrescentar futura");
    if !cpl3_crescer(this, 0x920, 4, 10, 0xc, 0x27c3, 0xa8ec74) {
        return;
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut [u16; 4], usize) -> usize = core::mem::transmute(0x417c10usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut temp = [0u16; 4];
    copia(cpr3_d(this + 0x920).wrapping_add(cpr3_d(this + 0x928).wrapping_mul(0xc)), &mut temp, data);
    cpr3_por(cpr3_d(this + 0x920).wrapping_add(cpr3_d(this + 0x928).wrapping_mul(0xc)) + 8, cpr3_d(oferta + 0x20));
    let n = cpr3_d(this + 0x928).wrapping_add(1);
    cpr3_por(this + 0x928, n);
    qsort(cpr3_d(this + 0x920), n as u32, 0xc, 0x778e90);
    *((oferta + 0x2d) as *mut u8) = 1;
    *((oferta + 0x2c) as *mut u8) = 0x13; // transferência futura
    suspender(cpr3_d(this + 0x856), cpr3_pessoa(cpr3_d(oferta + 0xc)), 0);
    *cpr3_marca(this, cpr3_d(oferta + 0xc)) |= 0x10; // a marca da transferência futura
    let c = contrato_de(cpr3_d(0xae1050), cpr3_pessoa(cpr3_d(oferta + 0xc)));
    if c != 0 && cpr3_b(c + 0x49) != 0 {
        cpr3_por(c + 0x4a, cpr3_d(oferta + 0x20));
    }
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(cpr3_d(oferta + 4));
        while e != 0 {
            suspender(cpr3_d(this + 0x856), cpr3_pessoa(cpr3_d(e)), 0);
            *cpr3_marca(this, cpr3_d(e)) |= 0x10;
            e = proximo(cpr3_d(oferta + 4));
        }
    }
}

/// Comparador: lado a lado pela marca do jogador e por `saida`, e pela devolução (tirar a marca é o único outro efeito;
/// a cópia, depois da original, acha o mesmo).
unsafe extern "thiscall" fn comparar_transferencia_futura_do_jogador(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(1089);
    let regioes = [(cpr3_marca(this, cpr3_d(jogador)) as usize, 1), (saida as usize, 4)];
    let n = if saida.is_null() { 1 } else { 2 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, saida), || transferencia_futura_do_jogador(this, jogador, saida));
    PLACAR.registrar(igual && r == c, || format!("transferencia_futura_do_jogador(jogador {}) original={} rust={} {:x?} x {:x?}", cpr3_d(jogador), r, c, mo, mr));
    r
}

/// Comparador: lado a lado pela conta e pelo vetor das futuras, pelas marcas do jogador e dos da troca e pelo cursor
/// da lista da troca (não aloca nem libera).
unsafe extern "thiscall" fn comparar_tirar_transferencia_futura(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1090);
    let mut regioes = vec![(this + 0x928, 4), (cpr3_marca(this, cpr3_d(oferta + 0xc)) as usize, 1)];
    let n = cpr3_d(this + 0x928) as i32;
    if n > 0 {
        regioes.push((cpr3_d(this + 0x920), n as usize * 0xc));
    }
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            regioes.push((cpr3_marca(this, cpr3_d(e)) as usize, 1));
            e = proximo(l);
        }
        regioes.push((l + 0xa, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || tirar_transferencia_futura(this, oferta));
    PLACAR.registrar(igual, || format!("tirar_transferencia_futura(oferta {:x}) {:x?} x {:x?}", cpr3_d(oferta + 0x20), mo, mr));
}

/// Comparador: a original acrescenta (pode realocar), reordena e suspende as listas de alvos: só ela, conferida pelo
/// plano: a conta sobe de 1, o registro (data e oferta) está no vetor, o estado 0x13 com +0x2d = 1 e a marca 0x10 do
/// jogador.
unsafe extern "thiscall" fn comparar_acrescentar_transferencia_futura(this: usize, oferta: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1091);
    let n = cpr3_d(this + 0x928);
    let mut alvo = [0u8; 0xc];
    alvo[..8].copy_from_slice(core::slice::from_raw_parts(data as *const u8, 8));
    alvo[8..].copy_from_slice(&(cpr3_d(oferta + 0x20) as u32).to_le_bytes());
    o(this, oferta, data);
    if cpr3_d(this + 0x920) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpr3_d(this + 0x928);
    let marca = *cpr3_marca(this, cpr3_d(oferta + 0xc));
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpr3_d(this + 0x920), conta, 0xc, &alvo) && cpr3_b(oferta + 0x2c) == 0x13 && cpr3_b(oferta + 0x2d) == 1 && marca & 0x10 != 0;
    PLACAR.registrar(ok, || format!("acrescentar_transferencia_futura(oferta {:x}): conta {} -> {}, estado {:#x}, marca {:#x}", cpr3_d(oferta + 0x20), n, conta, cpr3_b(oferta + 0x2c), marca));
}

// ---------- 254a leva: as ofertas abertas e as desfeitas: as ofertas pelos jogadores do clube, desfazer as ofertas pelo jogador e guardar o percentual da próxima venda (transfer_manager.cpp) ----------

unsafe fn cpr4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpr4_pessoa(id: usize) -> usize {
    cpr4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpr4_clube(id: usize) -> usize {
    cpr4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// { 0x8c6100 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_open_offers_for_club_players(clube,
/// lista)`: pelas ofertas (vetor [this], conta +4), fora das vazias e das de estado 2, 3, 4, 0x10 e 0x14 e das de estado
/// 0xf com +0x2e ≥ 3: o contrato do jogador (+0xc; 0x4cf5a0, guardado em [0xae1080]) que não é de empréstimo (+0x35 &
/// 0x3f ≠ 5) é do `clube`? Conta e, com `lista`, acrescenta o índice (4 bytes, 0x68d0d0). Devolve a conta.
unsafe extern "thiscall" fn ofertas_abertas_pelos_jogadores_do_clube(this: usize, clube: usize, lista: usize) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let mut n = 0u32;
    let mut i: i32 = 0;
    let (mut desfeita, mut emprestado) = (false, false); // os contadores, uma vez por chamada
    while i < cpr4_d(this + 4) as i32 {
        let o = cpr4_d(cpr4_d(this).wrapping_add((i as usize).wrapping_mul(4)));
        if o != 0 {
            let conta = match cpr4_b(o + 0x2c) {
                2 | 3 | 4 | 0x10 | 0x14 => false,
                0xf => {
                    desfeita |= cpr4_b(o + 0x2e) >= 3;
                    cpr4_b(o + 0x2e) < 3 // desfeita (+0x2e = 3) não conta
                }
                _ => true,
            };
            if conta {
                let c = contrato_de(cpr4_d(0xae1050), cpr4_pessoa(cpr4_d(o + 0xc)));
                cpr4_por(0xae1080, c);
                emprestado |= c != 0 && cpr4_b(c + 0x35) & 0x3f == 5 && cpr4_clube(cpr4_d(c + 4)) == clube;
                let dono = if c != 0 && cpr4_b(c + 0x35) & 0x3f != 5 { cpr4_clube(cpr4_d(c + 4)) } else { 0 }; // o empréstimo não conta
                if dono == clube {
                    if lista != 0 {
                        acrescentar(lista, &i, 4);
                    }
                    n += 1;
                }
            }
        }
        i += 1;
    }
    if desfeita {
        crate::comparacao::ramo("cpr_4 oferta desfeita");
    }
    if emprestado {
        crate::comparacao::ramo("cpr_4 jogador emprestado");
    }
    n
}

/// { 0x8c47d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::collapse_offers_for_player(pessoa)`: com ofertas pela pessoa (byte +1 do registro de 8 bytes em
/// [this+0x86e]), pelas ofertas por ela (0x8c5cf0 na lista local): a de transferência futura (0x13) perde a futura
/// (0x8cbf70); todas ficam no estado 0xf com +0x2e = 3; o vendedor é o clube do contrato do jogador da oferta (0x4cf5a0,
/// guardado em [0xae1080]); os lances do comprador (+8) pelo jogador e os do vendedor pelos da troca (lista +4) saem
/// (0x84aeb0 de [this+0x856]) e a oferta é agendada (0x8c6220 com 10 dias).
unsafe extern "thiscall" fn desfazer_ofertas_pelo_jogador(this: usize, pessoa: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar_futura: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cbf70usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    if cpr4_b(cpr4_d(this + 0x86e).wrapping_add(cpr4_d(pessoa).wrapping_mul(8)) + 1) != 0 {
        ofertas(this, pessoa, l, 0);
        cpr4_por(l + 0xa, cpr4_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            crate::comparacao::ramo("cpr_4 desfazer oferta");
            let o = cpr4_d(cpr4_d(this).wrapping_add(cpr4_d(e).wrapping_mul(4)));
            if cpr4_b(o + 0x2c) == 0x13 {
                tirar_futura(this, o);
            }
            *((o + 0x2c) as *mut u8) = 0xf; // desfeita
            *((o + 0x2e) as *mut u8) = 3;
            let jogador = cpr4_pessoa(cpr4_d(o + 0xc));
            let c = contrato_de(cpr4_d(0xae1050), jogador);
            cpr4_por(0xae1080, c);
            let vendedor = if c != 0 { cpr4_clube(cpr4_d(c + 4)) } else { 0 };
            desmarcar(cpr4_d(this + 0x856), cpr4_clube(cpr4_d(o + 8)), jogador);
            let t = cpr4_d(o + 4);
            if t != 0 {
                cpr4_por(t + 0xa, cpr4_d(t + 2));
                let mut x = proximo(cpr4_d(o + 4));
                while x != 0 {
                    desmarcar(cpr4_d(this + 0x856), vendedor, cpr4_pessoa(cpr4_d(x)));
                    x = proximo(cpr4_d(o + 4));
                }
            }
            agendar(this, o, 0xa, 0); // a oferta volta daqui a 10 dias
            e = proximo(l);
        }
    }
    soltar(l);
}

/// { 0x8caaf0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_next_sale_percentage(registro)`:
/// acrescenta o percentual da próxima venda (13 bytes copiados de `registro`) ao vetor [this+0x880] (capacidade +0x884,
/// conta +0x888); cheio, cresce de 50 em 50 (sem memória: caixa 0x2553 e nada). Reordena (qsort 0x945333 com 0x8e54d0,
/// a ordem da busca 0x8cb0f0).
unsafe extern "thiscall" fn guardar_percentual_da_proxima_venda(this: usize, registro: usize) {
    crate::comparacao::ramo("cpr_4 percentual da proxima venda");
    if !cpl3_crescer(this, 0x880, 4, 50, 0xd, 0x2553, 0xa8eb50) {
        return;
    }
    let n = cpr4_d(this + 0x888);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpr4_d(this + 0x880).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    cpr4_por(this + 0x888, n.wrapping_add(1)); // mais um percentual
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpr4_d(this + 0x880), cpr4_d(this + 0x888) as u32, 0xd, 0x8e54d0);
}

/// Comparador: sem `lista`, lado a lado por [0xae1080] e pela devolução; com `lista` (cresce a lista: não pode rodar
/// duas vezes), a cópia conta antes sem lista e a original roda: confere a conta.
unsafe extern "thiscall" fn comparar_ofertas_abertas_pelos_jogadores_do_clube(this: usize, clube: usize, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1092);
    if lista != 0 {
        let n = ofertas_abertas_pelos_jogadores_do_clube(this, clube, 0);
        let r = o(this, clube, lista);
        PLACAR.registrar(r == n, || format!("ofertas_abertas_pelos_jogadores_do_clube({:x}, lista) original={} rust={}", clube, r, n));
        return r;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, clube, 0), || ofertas_abertas_pelos_jogadores_do_clube(this, clube, 0));
    PLACAR.registrar(igual && r == c, || format!("ofertas_abertas_pelos_jogadores_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparador: agenda (aloca) e mexe nas listas de alvos: só a original. O plano antes: as ofertas pela pessoa (0x8c5cf0
/// numa lista própria, destruída depois); depois confere que todas ficaram no estado 0xf com +0x2e = 3.
unsafe extern "thiscall" fn comparar_desfazer_ofertas_pelo_jogador(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1093);
    if cpr4_b(cpr4_d(this + 0x86e).wrapping_add(cpr4_d(pessoa).wrapping_mul(8)) + 1) == 0 {
        return o(this, pessoa);
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    ofertas(this, pessoa, l, 0);
    let mut alvo = Vec::new();
    cpr4_por(l + 0xa, cpr4_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        alvo.push(cpr4_d(cpr4_d(this).wrapping_add(cpr4_d(e).wrapping_mul(4))));
        e = proximo(l);
    }
    soltar(l);
    o(this, pessoa);
    let ok = alvo.iter().all(|&of| cpr4_b(of + 0x2c) == 0xf && cpr4_b(of + 0x2e) == 3);
    PLACAR.registrar(ok, || format!("desfazer_ofertas_pelo_jogador(pessoa {}): {} ofertas, estados {:x?}", cpr4_d(pessoa), alvo.len(), alvo.iter().map(|&of| (cpr4_b(of + 0x2c), cpr4_b(of + 0x2e))).collect::<Vec<_>>()));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
unsafe extern "thiscall" fn comparar_guardar_percentual_da_proxima_venda(this: usize, registro: usize) {
    let n = cpr4_d(this + 0x888);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1094)(this, registro);
    if cpr4_d(this + 0x880) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpr4_d(this + 0x888);
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpr4_d(this + 0x880), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("guardar_percentual_da_proxima_venda: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

// ---------- 254a leva: as diretorias no início da carreira, as apelações de suspensão e a partida que o humano pode ver (manager_manager.cpp, human_manager.cpp) ----------

unsafe fn cpr5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpr5_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpr5_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpr5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cpr5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O técnico (ou o selecionador) do clube (0x53ca90).
unsafe fn cpr5_tecnico(clube: usize) -> usize {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    f(clube)
}

unsafe fn cpr5_humano(clube: usize, qualquer: u8, pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, qualquer, pessoa, 0)
}

/// Uma das confianças da linha (+6..+0xc): sorteio(`n1`) + `h` guardado em float, menos sorteio(500), truncado.
unsafe fn cpr5_confianca(c: usize, n1: i32, h: f32) {
    let r1 = cpr5_sortear(n1);
    let s = (r1 as f64 + h as f64) as f32;
    let r2 = cpr5_sortear(0x1f4);
    cpr5_pw(c, ftol(s as f64 - r2 as f64) as i16);
}

/// Abaixo de 5500: 5500 + sorteio(250); acima de 10000: 10000.
unsafe fn cpr5_prender_5500(c: usize) {
    if cpr5_w(c) < 0x157c {
        cpr5_pw(c, (cpr5_sortear(0xfa) + 0x157c) as i16);
    } else if cpr5_w(c) > 0x2710 {
        cpr5_pw(c, 0x2710);
    }
}

/// Os campos da linha que as duas saídas zeram (+0x10..+0x16, os bits de +2, +3, +4, +5, +0x28 e +0x40).
unsafe fn cpr5_zerar_marcas(l: usize) {
    for k in [0x10usize, 0x12, 0x14, 0x16] {
        cpr5_pw(l + k, 0);
    }
    cpr5_pb(l + 2, cpr5_b(l + 2) & 0xe0);
    cpr5_pb(l + 4, 0);
    cpr5_pb(l + 3, cpr5_b(l + 3) & 0xe1);
    cpr5_pb(l + 2, cpr5_b(l + 2) & 0x1f);
    cpr5_pd(l + 0x28, 0);
    cpr5_pb(l + 5, 0);
    cpr5_pb(l + 3, cpr5_b(l + 3) & 0xfe);
    cpr5_pb(l + 0x40, cpr5_b(l + 0x40) & 7);
    cpr5_pb(l + 0x40, cpr5_b(l + 0x40) & 0xf8);
}

/// { 0x68f130 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::init_club_board_records()`: no começo da
/// carreira (pelo construtor 0x68dbf0). (1) Cada pessoa com clube (+0x24): função (+0x28) 5 ou 12 vira o técnico do
/// clube (+0xcf); 11 fica; 13 e 15 viram 11 (com ficha de jogador +0x61) ou 8; as outras perdem o clube; sem contrato
/// (+0x39) e sem função de origem (+0x3d), a função vai para +0x3d; fora de 11 a função zera. (2) Cada clube (as
/// seleções só com nação e +0x71): a segunda seleção tem a reputação (+0x80) presa a 200 × a da nação (+0x8e) e as
/// vagas zeradas; a linha dele (73 bytes em [this]) ganha as datas (23/07/1971 em +0x41, hoje em +0x18), a reputação² ×
/// 0,25 (double em +0x30) e zeros. Com técnico: a data 23/08/1971 em +0x20 (pela linha do id), a expectativa `f` (a
/// reputação, a nota da diretoria +0xa ou 4500, que cai a 0,75 × reputação), `h` = f / (reputação × 0,5 + 2500) ×
/// 7500 e as quatro confianças (+6, +8, +0xa, +0xc) por sorteios, presas embaixo (5750 ou 5500 com sorteio) e em
/// cima (10000; +6 acima de 9500 vira 10000 − sorteio(500)); zera as marcas e põe as expectativas da temporada
/// (0x6a3770 com 1). Sem técnico (fora do clube parceiro): confianças 5000, +0xe = 16, marcas zeradas; clube
/// dirigível (ou com competição e nação com +0x11c & 3) recebe as expectativas e a vaga (0x69ba50).
unsafe extern "thiscall" fn iniciar_diretorias_dos_clubes(this: usize) {
    // (1) as pessoas
    let mut k = 0i32;
    while k < cpr5_d(0xae2364) as i32 {
        let p = cpr5_d(0xae23bc).wrapping_add(k as usize * 0x6e);
        k += 1;
        let clube = cpr5_d(p + 0x24);
        if clube == 0 {
            continue;
        }
        let f = cpr5_b(p + 0x28);
        match f {
            5 | 12 => {
                cpr5_pd(clube + 0xcf, p as u32);
                continue;
            }
            11 => continue,
            _ => {}
        }
        let mut tira = true;
        if f == 0xf || f == 0xd {
            if cpr5_d(p + 0x61) != 0 {
                cpr5_pb(p + 0x28, 0xb);
                tira = false;
            } else {
                cpr5_pb(p + 0x28, 8);
            }
        }
        if tira {
            cpr5_pd(p + 0x24, 0);
        }
        if cpr5_d(p + 0x39) == 0 && cpr5_b(p + 0x3d) == 0 {
            cpr5_pb(p + 0x3d, cpr5_b(p + 0x28));
        }
        if cpr5_b(p + 0x28) != 0xb {
            cpr5_pb(p + 0x28, 0);
        }
    }
    // (2) os clubes
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let data_dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    let mut parceiro = 0u32; // a original guarda numa variável só, de clube em clube
    let mut i = 0i32;
    let mut ofs = 0usize;
    while i < cpr5_d(0xae235c) as i32 {
        let clube = cpr5_d(0xae23b4).wrapping_add(i as usize * 0x245);
        let l = cpr5_d(this).wrapping_add(ofs);
        i += 1;
        ofs += 0x49;
        if e_selecao(clube) != 0 {
            let n = cpr5_d(clube + 0x53);
            if n == 0 || cpr5_d(n + 0x71) == 0 {
                continue;
            }
        }
        if segunda(clube) != 0 {
            let n = cpr5_d(clube + 0x53);
            if n != 0 && cpr5_w(clube + 0x80) as i32 > cpr5_w(n + 0x8e) as i32 * 200 {
                cpr5_pw(clube + 0x80, cpr5_w(n + 0x8e).wrapping_mul(200));
            }
            cpr5_pd(clube + 0xcf, 0);
            cpr5_pd(clube + 0xbf, 0);
            cpr5_pd(clube + 0xd3, 0);
            core::ptr::write_bytes((clube + 0xc3) as *mut u8, 0, 12);
            core::ptr::write_bytes((clube + 0x19f) as *mut u8, 0, 0x14 + 0x1c + 12); // +0x19f..+0x1db
        }
        let (mut d, mut t) = ([0u8; 8], [0u8; 8]);
        let r = data_dma(d.as_mut_ptr() as usize, 0x17, 6, 0x7b3, 0xffff_ffff);
        copia(l + 0x41, t.as_mut_ptr() as usize, r);
        cpr5_pw(l + 0xe, 0);
        cpr5_pd(l + 0x2c, 0);
        copia(l + 0x18, t.as_mut_ptr() as usize, 0xae2c90);
        cpr5_pb(l, 0);
        let rep = cpr5_w(clube + 0x80) as i32;
        core::ptr::write_unaligned((l + 0x30) as *mut f64, rep.wrapping_mul(rep) as f64 * 0.25);
        cpr5_pd(l + 0x38, 0);
        cpr5_pd(l + 0x3c, 0);
        if cpr5_tecnico(clube) == 0 {
            // sem técnico
            let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
            if parceiro_de(clube, &mut parceiro, 1) != 0 && parceiro != 0 {
                continue;
            }
            crate::comparacao::ramo("cpr_5 diretoria sem tecnico");
            for c in [6usize, 8, 0xa, 0xc] {
                cpr5_pw(l + c, 0x1388);
            }
            cpr5_pw(l + 0xe, 0x10);
            cpr5_zerar_marcas(l);
            let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
            if dirigivel(clube) == 0 {
                if e_selecao(clube) != 0 {
                    continue;
                }
                let objs = cpr5_d(0xadadfc);
                let comp = cpr5_d(clube + 0x57);
                let n = cpr5_d(clube + 0x53);
                if objs == 0 || comp == 0 || n == 0 || cpr5_d(objs + cpr5_d(comp) * 4) == 0 || cpr5_b(n + 0x11c) & 3 == 0 {
                    continue;
                }
            }
            expectativas(this, clube, 1);
            let vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ba50usize);
            vaga(this, clube, 0);
            continue;
        }
        // com técnico
        let r = data_dma(d.as_mut_ptr() as usize, 0x17, 7, 0x7b3, 0xffff_ffff);
        copia(cpr5_d(this).wrapping_add(cpr5_d(clube).wrapping_mul(0x49)) + 0x20, t.as_mut_ptr() as usize, r);
        let mut f: f32;
        let mut direto = false; // a 4500 salta o teste do zero e do 500
        if cpr5_d(cpr5_tecnico(clube) + 0x69) == 0 && cpr5_humano(clube, 1, 0) == 0 {
            f = rep as f32;
        } else if cpr5_humano(clube, 1, 0) != 0 {
            if cpr5_d(clube + 0xcf) != 0 && cpr5_d(cpr5_tecnico(clube) + 0x69) != 0 {
                f = cpr5_w(cpr5_d(cpr5_tecnico(clube) + 0x69) + 0xa) as f32;
            } else {
                f = 4500.0;
                direto = true;
            }
        } else {
            f = cpr5_w(cpr5_d(cpr5_tecnico(clube) + 0x69) + 0xa) as f32;
            let teto = rep as f64 * 0.75;
            if (cpr5_w(cpr5_d(cpr5_tecnico(clube) + 0x69) + 0xa) as f64) < teto {
                let v = ftol(teto) as i16;
                cpr5_pw(cpr5_d(cpr5_tecnico(clube) + 0x69) + 0xa, v); // a nota da diretoria cai a 0,75 × reputação
            }
        }
        if !direto {
            if f == 0.0 {
                let r80 = cpr5_w(clube + 0x80);
                f = if r80 != 0 { (r80 as f64 * 0.75) as f32 } else { 5000.0 };
            } else if f < 500.0 {
                f = (cpr5_sortear(0x3e8) + 0x1f4) as f32;
            }
        }
        let mut g = rep as f64 * 0.5 + 2500.0;
        if g == 0.0 {
            g = f as f64 * 1.5;
        } else if g < 500.0 {
            g = (cpr5_sortear(0x2ee) + 0xfa) as f64;
        }
        crate::comparacao::ramo("cpr_5 diretoria com tecnico");
        let h = ((f as f64 / g) * 7500.0) as f32;
        cpr5_confianca(l + 6, 0x1f4, h);
        let c = cpr5_w(l + 6);
        if c < 0x1676 {
            crate::comparacao::ramo("cpr_5 confianca baixa");
            cpr5_pw(l + 6, (cpr5_sortear(0x2ee) + 0x1676) as i16);
        } else if c > 0x251c {
            cpr5_pw(l + 6, (0x2710 - cpr5_sortear(0x1f4)) as i16);
        }
        cpr5_confianca(l + 8, 0x1f4, h);
        if cpr5_w(l + 8) < 0x1676 {
            cpr5_pw(l + 8, (cpr5_sortear(0x1f4) + 0x1676) as i16);
        } else if cpr5_w(l + 8) > 0x2710 {
            cpr5_pw(l + 8, 0x2710);
        }
        cpr5_confianca(l + 0xa, 0x6d6, h);
        cpr5_prender_5500(l + 0xa);
        cpr5_confianca(l + 0xc, 0x9c4, h);
        cpr5_prender_5500(l + 0xc);
        cpr5_zerar_marcas(l);
        expectativas(this, clube, 1);
    }
}

// ponytail: sorteia, põe as expectativas (0x6a3770) e abre vagas (0x69ba50, aloca): só a original; a cópia leva os
// ramos (roda uma vez, no começo da carreira).
unsafe extern "thiscall" fn comparar_iniciar_diretorias_dos_clubes(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1095)(this)
}

/// { 0x6abef0 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::process_ban_appeals()`: percorre a lista das
/// apelações (+0x14, a conta em palavra com sinal relida a cada volta) e, para cada uma cuja data (+5) já não é depois
/// de hoje (0x456740), com resultado (+4): as até 4 regras (+0xd..+0x10, ≥ 0) recebem o resultado na disciplina
/// (0x557000 com os campos +0x11..+0x18); depois a notícia do resultado (0x6ac060) e a apelação sai (0x68d1f0). Item
/// nulo: a caixa "Error" (0x57f5) e para. Pelo dia a dia da diretoria (0x68f9c0); só as apelações do humano entram na
/// lista.
unsafe extern "thiscall" fn apelacoes_de_suspensao(this: usize) {
    let lista = || cpr5_d(this + 0x14);
    cpr5_pd(lista() + 0xa, cpr5_d(lista() + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let aplicar: unsafe extern "thiscall" fn(usize, u32, i8, u8, u8, i16, u8, i16, u8) = core::mem::transmute(0x557000usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u8, u8, i16, u8) = core::mem::transmute(0x6ac060usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut k = 0i32;
    while k < cpr5_w(lista()) as i32 {
        let item = proximo(lista());
        if item == 0 {
            return aviso_em(0x9f18a4, 0x57f5);
        }
        if depois(item + 5, cpr5_d(0xae2c90) as u32, cpr5_d(0xae2c94) as u32) == 0 {
            crate::comparacao::ramo("cpr_5 apelacao decidida");
            let p = cpr5_d(0xae23bc).wrapping_add(cpr5_d(item).wrapping_mul(0x6e));
            if cpr5_b(item + 4) != 0 {
                for j in 0..4usize {
                    let regra = cpr5_b(item + 0xd + j) as i8;
                    if regra > -1 {
                        // ponytail: a original empurra os campos em registradores com lixo em cima; a 0x557000 lê só
                        // o byte ou a palavra de cada um.
                        aplicar(cpr5_d(0xae28f0), cpr5_d(p) as u32, regra, cpr5_b(item + 0x11), cpr5_b(item + 0x12),
                                cpr5_w(item + 0x13), cpr5_b(item + 0x15), cpr5_w(item + 0x16), cpr5_b(item + 0x18));
                    }
                }
            }
            noticia(this, p, cpr5_b(item + 4), cpr5_b(item + 0x15), cpr5_w(item + 0x16), cpr5_b(item + 0x18));
            remover(lista(), item);
        }
        k += 1;
    }
}

// ponytail: aplica a disciplina, solta a notícia e libera a apelação: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_apelacoes_de_suspensao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1096)(this)
}

/// { 0x5fe700 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::can_watch_fixture(partida, humano)`: o humano
/// (ou, nulo, o da tela atual [0xb74c26 + 0x300 × [0xb77c7a]]) pode ver a partida se ela tem +0x43 negativo, nenhum dos
/// dois clubes (+0x1c, +0x20, com as marcas +0x40, +0x41) é dele (0x600a50) e, em cada grupo das nações-chave
/// ([0xb64738]), nenhuma partida da lista do dia dela (data +0x28/+0x2a, fatia +0x3f; 0x5ae6a0) tem um clube dele: 1.
/// Senão 0. Partida nula: a caixa "Error" (0x707) e 0. Pela limpeza das partidas assistidas (0x5fe8d0) e pela escolha
/// da partida (0x5fe500).
unsafe extern "thiscall" fn pode_ver_a_partida(this: usize, partida: usize, humano: usize) -> u32 {
    if partida == 0 {
        aviso_em(0x9cf11c, 0x707);
        return 0;
    }
    let mut humano = humano;
    if humano == 0 {
        humano = cpr5_d(0xb74c26usize.wrapping_add((cpr5_w(0xb77c7a) as isize as usize).wrapping_mul(0x300)));
        if humano == 0 {
            return 0;
        }
    }
    if cpr5_b(partida + 0x43) as i8 > -1 {
        return 0;
    }
    let dele = |p: usize| cpr5_humano(cpr5_d(p + 0x1c), cpr5_b(p + 0x40), humano) != 0
        || cpr5_humano(cpr5_d(p + 0x20), cpr5_b(p + 0x41), humano) != 0;
    if dele(partida) {
        return 0;
    }
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut grupo = 0u8;
    while (grupo as i8) < cpr5_b(0xb64738) as i8 {
        let mut d = [0u8; 8];
        let data = data_nova(d.as_mut_ptr() as usize, cpr5_w(partida + 0x2a) as u16 as u32, cpr5_w(partida + 0x28) as u16 as u32);
        // ponytail: a original empurra o grupo como palavra inteira (os bytes de cima são sobra de pilha); a 0x5ae6a0
        // repassa o tipo (char) e a 0x5abf10 lê só o byte.
        let lista = lista_de(data, cpr5_b(partida + 0x3f) as u32, grupo as u32);
        if lista != 0 {
            cpr5_pd(lista + 0xa, cpr5_d(lista + 2) as u32);
            let mut p = proximo(lista);
            while p != 0 {
                if dele(p) {
                    return 0;
                }
                p = proximo(lista);
            }
        }
        grupo = grupo.wrapping_add(1);
    }
    1
}

/// Comparador: a original e a cópia (as duas só leem; a volta da lista recomeça o cursor), pelo valor.
unsafe extern "thiscall" fn comparar_pode_ver_a_partida(this: usize, partida: usize, humano: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1097)(this, partida, humano);
    if partida == 0 {
        return r;
    }
    let n = pode_ver_a_partida(this, partida, humano);
    PLACAR.registrar(r == n, || format!("pode_ver_a_partida(partida {:x}, humano {:x}) original={} rust={}", partida, humano, r, n));
    r
}

// ---------- 254a leva: as aposentadorias: quem se aposenta no começo do jogo, quem se aposenta pela idade e a lista inicial (player_regen.cpp) ----------

unsafe fn cpr8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpr8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpr8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpr8_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpr8_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) e, pelo clube dele (registro de 0x245 bytes pelo +4; sem
/// contrato, 0), o contrato da pessoa com esse clube (0x4cf5f0), que a rotina devolve.
unsafe fn cpr8_contratos(pessoa: usize) -> usize {
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = de(cpr8_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpr8_d(0xae23b4).wrapping_add(cpr8_d(c + 4).wrapping_mul(0x245)) } else { 0 };
    let com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    com(cpr8_d(0xae1050), pessoa, clube)
}

/// O estatuto do contrato com o clube (+0x3d); sem ele, −100 (0x9c).
unsafe fn cpr8_estatuto(c2: usize) -> i8 {
    if c2 == 0 {
        0x9cu8 as i8
    } else {
        cpr8_b(c2 + 0x3d)
    }
}

/// O limite de idade de quem não é jogador, pelo tipo do cargo (+0x3d; sem ele +0x28; sem os dois, como a original,
/// o byte baixo do ponteiro da saída, que ela lê da vaga errada da pilha): 1, 2 e 4: 70 + sorteio(10); 8 e 9:
/// 60 + sorteio(5); os outros: 61 + sorteio(5) com clube (+0x39) ou seleção (+0x24), senão 55 + sorteio(10). Byte.
unsafe fn cpr8_limite_sem_jogador(pessoa: usize, saida: usize) -> i8 {
    let mut t = cpr8_b(pessoa + 0x3d);
    if t == 0 {
        t = cpr8_b(pessoa + 0x28);
        if t == 0 {
            t = saida as u8 as i8; // ponytail: a original lê aqui o byte baixo do argumento `saida`
        }
    }
    let (n, base) = match t {
        1 | 2 | 4 => (10, 0x46u8),
        8 | 9 => (5, 0x3c),
        _ => {
            if cpr8_d(pessoa + 0x39) != 0 || cpr8_d(pessoa + 0x24) != 0 {
                (5, 0x3d)
            } else {
                (10, 0x37)
            }
        }
    };
    (sortear(n) as u8).wrapping_add(base) as i8
}

/// O limite de idade do jogador pela ficha: 34 com +0xf ≥ 15; senão 30 com +0x15 ≥ 15; senão 31 com +0x14 < 15;
/// senão 30 (bytes com sinal).
unsafe fn cpr8_limite_do_jogador(j: usize) -> i8 {
    if cpr8_b(j + 0xf) >= 15 {
        0x22
    } else if cpr8_b(j + 0x15) >= 15 {
        0x1e
    } else if cpr8_b(j + 0x14) < 15 {
        0x1f
    } else {
        0x1e
    }
}

/// A saída de quem se aposenta: o motivo (+0x18), +0x19 = 0, o id (+0) e o id do clube (+4; −1 sem clube).
unsafe fn cpr8_preencher(pessoa: usize, saida: usize, motivo: u8) {
    *((saida + 0x18) as *mut u8) = motivo;
    *((saida + 0x19) as *mut u8) = 0;
    core::ptr::write_unaligned(saida as *mut u32, cpr8_d(pessoa) as u32);
    let c = cpr8_d(pessoa + 0x39);
    core::ptr::write_unaligned((saida + 4) as *mut u32, if c != 0 { cpr8_d(c) as u32 } else { u32::MAX });
}

/// Amanhã (dia + 1, só a palavra) nas duas datas da saída (+8 e +0x10; ano em +0xa e +0x12).
unsafe fn cpr8_amanha(saida: usize) {
    let (dia, ano) = (cpr8_u(0xae2c90).wrapping_add(1), cpr8_u(0xae2c92));
    cpr8_pw(saida + 8, dia);
    cpr8_pw(saida + 0xa, ano);
    cpr8_pw(saida + 0x10, dia);
    cpr8_pw(saida + 0x12, ano);
}

/// Sem nação-chave válida na aposentadoria pela idade: hoje + sorteio(20) nas duas datas; passando de 365, o dia
/// sorteio(10) do ano seguinte.
unsafe fn cpr8_fora_por_idade(saida: usize) {
    let a = (sortear(20) as u32).wrapping_add(cpr8_d(0xae2c90) as u32) as u16;
    let ano = cpr8_u(0xae2c92);
    if a as i16 > 0x16d {
        let d = sortear(10) as u16;
        cpr8_pw(saida + 0x10, d);
        cpr8_pw(saida + 0x12, ano.wrapping_add(1));
        cpr8_pw(saida + 8, d);
        cpr8_pw(saida + 0xa, ano.wrapping_add(1));
    } else {
        cpr8_pw(saida + 8, a);
        cpr8_pw(saida + 0xa, ano);
        cpr8_pw(saida + 0x10, a);
        cpr8_pw(saida + 0x12, ano);
    }
}

/// As datas da aposentadoria na saída (+8 o anúncio, +0x10 a aposentadoria; só o dia e o ano, o +4 de cada data fica
/// como está), pela nação-chave do clube (0x669360) ou, sem clube, da nação (+0x1a, 0x669340); sem os dois, amanhã.
/// Índice 0..0x22: o anúncio em hoje + 1 + sorteio(95) (passando de 365, o dia sorteio(60) do ano seguinte); a
/// aposentadoria no fim de temporada da nação (+0x15 do registro de 72 bytes em 0xb63d60) + sorteio(30) (passando de
/// 365, sorteio(30) do ano seguinte; com +0x1d, um ano a mais); a aposentadoria antes do anúncio fica no anúncio, e a
/// menos de 45 dias dele (0x54cdb0) o anúncio vai para ela. Índice fora: `fora` (cada rotina tem a sua).
unsafe fn cpr8_datas(pessoa: usize, saida: usize, fora: unsafe fn(usize)) {
    let clube = cpr8_d(pessoa + 0x39);
    let k = if clube != 0 {
        let pelo_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
        pelo_clube(clube) as u8 as i8
    } else {
        let nacao = cpr8_d(pessoa + 0x1a);
        if nacao == 0 {
            return cpr8_amanha(saida);
        }
        let pela_nacao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669340usize);
        pela_nacao(nacao) as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return fora(saida);
    }
    crate::comparacao::ramo("cpr_8 datas pela nacao");
    let ano = cpr8_u(0xae2c92);
    let a = (sortear(95) as u32).wrapping_add(cpr8_d(0xae2c90) as u32).wrapping_add(1) as u16;
    let (dia1, ano1) = if a as i16 > 0x16d { (sortear(60) as u16, ano.wrapping_add(1)) } else { (a, ano) };
    cpr8_pw(saida + 0xa, ano1);
    cpr8_pw(saida + 8, dia1);
    let t = 0xb63d60usize.wrapping_add((k as usize) * 0x48);
    let d2 = (sortear(30) as u16).wrapping_add(cpr8_u(t + 0x15)); // o fim de temporada da nação
    cpr8_pw(saida + 0x10, d2);
    let mais_um = *((t + 0x1d) as *const u8) != 0;
    if d2 as i16 > 0x16d {
        cpr8_pw(saida + 0x10, sortear(30) as u16);
        cpr8_pw(saida + 0x12, ano.wrapping_add(if mais_um { 2 } else { 1 }));
    } else {
        cpr8_pw(saida + 0x12, if mais_um { ano.wrapping_add(1) } else { ano });
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t8 = [0u8; 8];
    let (da, aa) = (cpr8_u(saida + 0x12), cpr8_u(saida + 0xa));
    if da < aa || (da == aa && cpr8_w(saida + 0x10) < cpr8_w(saida + 8)) {
        copia(saida + 0x10, t8.as_mut_ptr(), saida + 8);
        return;
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    if dias(saida + 0x10, cpr8_d(saida + 8) as u32, cpr8_d(saida + 0xc) as u32) >= 0x2d {
        return;
    }
    copia(saida + 8, t8.as_mut_ptr(), saida + 0x10);
}

/// { 0x7a79c0 } (thiscall sem ler o this, ret 8, player_regen.cpp): `player_regen_check_initial_retirement(pessoa,
/// saida)`: no jogo novo (pela 0x7a58f0), a pessoa com clube (+0x39) se aposenta nesta temporada? O contrato
/// (cpr8_contratos) e o estatuto (cpr8_estatuto). Sem ficha de jogador (+0x61): sem ficha de técnico (+0x69) sempre;
/// com ela, passando do limite (cpr8_limite_sem_jogador); motivo 0x20. Jogador: passando do limite da ficha
/// (cpr8_limite_do_jogador), v = ⌊⌊1 − (idade − limite) × 0,1⌋ × habilidade (+0xb)⌋ (16 bits); acima de 45 anos,
/// motivo 1. Clube sem competição (+0x57; sem clube, nação sem +0x11c): sorteio(3) ≠ 0 aposenta, senão só estatuto
/// negativo com clube. Senão: v ≥ 5000 ou +5 > 80, não; +5 < 10 ou v < 1500, sim; com clube, só estatuto negativo;
/// sem clube, motivo 2 com sorteio(25 × +0x3c) = 0. Quem se aposenta: a saída (cpr8_preencher) e as datas
/// (cpr8_datas; sem nação-chave válida, amanhã); devolve 1.
unsafe extern "thiscall" fn aposentadoria_no_inicio(_this: usize, pessoa: usize, saida: usize) -> u8 {
    if cpr8_d(pessoa + 0x39) == 0 {
        return 0;
    }
    let c2 = cpr8_contratos(pessoa);
    let estatuto = cpr8_estatuto(c2);
    let idade = cpr8_b(pessoa + 0x18);
    let j = cpr8_d(pessoa + 0x61);
    let motivo: u8 = 'm: {
        if j == 0 {
            if cpr8_d(pessoa + 0x69) != 0 && idade <= cpr8_limite_sem_jogador(pessoa, saida) {
                return 0;
            }
            break 'm 0x20;
        }
        let lim = cpr8_limite_do_jogador(j);
        if idade <= lim {
            return 0;
        }
        let f = ftol(f64_em(0x967890) - (idade as i32 - lim as i32) as f64 * f64_em(0x967880)) as i16;
        let v = ftol(f as f64 * cpr8_w(j + 0xb) as f64) as i16;
        if idade > 0x2d {
            break 'm 1;
        }
        let clube = cpr8_d(pessoa + 0x39);
        let sorteia = if clube != 0 {
            cpr8_d(clube + 0x57) == 0
        } else {
            let n = cpr8_d(pessoa + 0x1a);
            n != 0 && *((n + 0x11c) as *const u8) == 0
        };
        if sorteia {
            if sortear(3) != 0 {
                break 'm 1;
            }
            if clube == 0 || estatuto >= 0 {
                return 0;
            }
            break 'm 1;
        }
        if v >= 0x1388 {
            return 0;
        }
        let hab = cpr8_w(j + 5);
        if hab > 0x50 {
            return 0; // cpr8: a habilidade alta segura o jogador
        }
        if hab < 0xa || v < 0x5dc {
            crate::comparacao::ramo("cpr_8 inicio habilidade");
            break 'm 1;
        }
        if clube != 0 {
            if estatuto < 0 {
                break 'm 1;
            }
            return 0;
        }
        if sortear(25 * cpr8_b(j + 0x3c) as i32) != 0 {
            return 0;
        }
        2
    };
    cpr8_preencher(pessoa, saida, motivo);
    cpr8_datas(pessoa, saida, cpr8_amanha);
    1
}

/// { 0x7a7310 } (thiscall sem ler o this, ret 8, player_regen.cpp): `player_regen_check_retirement(pessoa, saida)`:
/// na busca de novas aposentadorias (0x7a5790), a pessoa se aposenta? Sem ficha de jogador: como na 0x7a79c0 (motivo
/// 0x20). Jogador acima do limite da ficha: acima de 33 anos, o limite cai 1 sem contrato com o clube, ou com +0x45 no
/// contrato e +0x15/+0x14 da ficha = 20 ou o nibble alto de +0x4f = 7; com clube e contrato de salário (+0xc) acima de
/// 750 × (+0x59 + 10) até depois deste ano (+0x2f), cai 1 (e mais 1 com +0x59 < 6). v = ⌊(1 − (idade − limite) ×
/// 0,075) × habilidade (+0xb)⌋ (16 bits). Motivo 1: acima de 45; acima de 34 sem clube e desempregado há mais de 65
/// dias (desde +0x3e); 3 anos acima do limite, mais de 31 e +0x3e do contrato > 5; 4 acima: sem clube; ou contrato a
/// menos de 300 dias do fim (0x85fa00), estatuto abaixo de 50 e técnico (+0x69) ou +0x57 > sorteio(20) com o
/// potencial × 0,85 acima da habilidade atual (+7 e +5). Depois v > 5500, não. Clube em competição fora do jogo
/// ([[0xadadfc] + id × 4] = 0, ou sem +0x57; sem clube, nação sem +0x11c): sorteio(3) = 0 aposenta; com clube e
/// estatuto negativo também; senão só com v < 3500. Senão como na 0x7a79c0 (+5 > 100 não; +5 < 10 ou v < 1500 sim;
/// com clube só estatuto negativo; sem clube motivo 2 com sorteio(25 × +0x3c) = 0). Quem se aposenta: a saída e as
/// datas (sem nação-chave válida, cpr8_fora_por_idade); devolve 1.
unsafe extern "thiscall" fn aposentadoria_por_idade(_this: usize, pessoa: usize, saida: usize) -> u8 {
    let c2 = cpr8_contratos(pessoa);
    let estatuto = cpr8_estatuto(c2);
    let idade = cpr8_b(pessoa + 0x18);
    let j = cpr8_d(pessoa + 0x61);
    let motivo: u8 = 'm: {
        if j == 0 {
            if cpr8_d(pessoa + 0x69) != 0 && idade <= cpr8_limite_sem_jogador(pessoa, saida) {
                return 0;
            }
            break 'm 0x20;
        }
        let mut lim = cpr8_limite_do_jogador(j);
        if idade <= lim {
            return 0;
        }
        if idade > 0x21 {
            if c2 == 0 {
                lim = lim.wrapping_sub(1);
            } else if cpr8_d(c2 + 0x45) != 0
                && (cpr8_b(j + 0x15) == 0x14 || cpr8_b(j + 0x14) == 0x14 || *((c2 + 0x4f) as *const u8) & 0xf0 == 0x70)
            {
                lim = lim.wrapping_sub(1);
            }
        }
        if cpr8_d(pessoa + 0x39) != 0 && c2 != 0 {
            let x = cpr8_b(pessoa + 0x59) as i32;
            if (cpr8_d(c2 + 0xc) as i32) > (x + 10).wrapping_mul(750) && cpr8_u(c2 + 0x2f) > cpr8_u(0xae2c92) {
                lim = lim.wrapping_sub(1);
                if x < 6 {
                    lim = lim.wrapping_sub(1);
                }
            }
        }
        let v = ftol((f64_em(0x967890) - (idade as i32 - lim as i32) as f64 * f64_em(0x96a730)) * cpr8_w(j + 0xb) as f64) as i16;
        if idade > 0x2d {
            break 'm 1;
        }
        if idade > 0x22 && cpr8_d(pessoa + 0x39) == 0 {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            if dias(0xae2c90, cpr8_d(pessoa + 0x3e) as u32, cpr8_d(pessoa + 0x42) as u32) > 0x41 {
                break 'm 1;
            }
        }
        let (i, l) = (idade as i32, lim as i32);
        if i > l + 2 && idade > 0x1f && c2 != 0 && cpr8_b(c2 + 0x3e) > 5 {
            break 'm 1;
        }
        if i > l + 3 {
            if cpr8_d(pessoa + 0x39) == 0 {
                break 'm 1;
            }
            let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x85fa00usize);
            if c2 != 0 && (fim(c2) as i16) < 300 && estatuto < 0x32 {
                if cpr8_d(pessoa + 0x69) != 0 {
                    break 'm 1;
                }
                if cpr8_b(pessoa + 0x57) as i32 > sortear(20) {
                    let (cur, pot) = (cpr8_w(j + 5) as f64, cpr8_w(j + 7) as f64);
                    if !(pot * f64_em(0x969520) <= cur) {
                        break 'm 1;
                    }
                }
            }
        }
        if v > 0x157c {
            return 0;
        }
        let clube = cpr8_d(pessoa + 0x39);
        let sorteia = if clube != 0 {
            let lg = cpr8_d(clube + 0x57);
            lg == 0 || cpr8_d(cpr8_d(0xadadfc).wrapping_add(cpr8_d(lg).wrapping_mul(4))) == 0
        } else {
            let n = cpr8_d(pessoa + 0x1a);
            n != 0 && *((n + 0x11c) as *const u8) == 0
        };
        if sorteia {
            crate::comparacao::ramo("cpr_8 fora do jogo");
            if sortear(3) == 0 {
                break 'm 1; // cpr8: um em três se aposenta
            }
            if cpr8_d(pessoa + 0x39) != 0 && estatuto < 0 {
                break 'm 1;
            }
            if v >= 0xdac {
                return 0;
            }
            break 'm 1;
        }
        let cur = cpr8_w(j + 5);
        if cur > 0x64 {
            return 0;
        }
        if cur < 0xa || v < 0x5dc {
            break 'm 1;
        }
        if clube != 0 {
            if estatuto < 0 {
                break 'm 1;
            }
            return 0;
        }
        if sortear(25 * cpr8_b(j + 0x3c) as i32) != 0 {
            return 0;
        }
        2
    };
    cpr8_preencher(pessoa, saida, motivo);
    cpr8_datas(pessoa, saida, cpr8_fora_por_idade);
    1
}

/// { 0x7a58f0 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::build_initial_retirement_list`: no jogo novo, cada pessoa (0x6e bytes em [0xae23bc], até [0xae2364],
/// relida a cada passo) de id abaixo de [0xae2364] − 16, sem os bits 0x30 do +0xb do registro de 79 bytes
/// ([0xae2d0c]) e sem contrato (0x4cf5a0) com +0x49 é testada (0x7a79c0); quem se aposenta entra na lista
/// (0x7a8510). No fim o qsort da lista ([this+4], conta +8, 26 bytes, 0x7ae2a0).
unsafe extern "thiscall" fn lista_inicial_de_aposentadorias(this: usize) {
    crate::comparacao::ramo("cpr_8 lista inicial");
    // ponytail: a original constrói as duas datas da entrada (0x54a110) e deixa o resto como estava na pilha; a
    // 0x7a79c0 grava tudo o que a 0x7a8510 copia antes de devolver 1 (a terceira data local nunca é lida: fica de fora)
    let mut e = [0u8; 0x1a];
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    padrao(e.as_mut_ptr().add(8));
    padrao(e.as_mut_ptr().add(0x10));
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let testar: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x7a79c0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x7a8510usize);
    let mut p = cpr8_d(0xae23bc);
    let mut i = 0i32;
    while i < cpr8_d(0xae2364) as i32 {
        let id = cpr8_d(p) as i32;
        if id < (cpr8_d(0xae2364) as i32).wrapping_sub(0x10)
            && *((cpr8_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 == 0
        {
            let c = de(cpr8_d(0xae1050), p);
            if (c == 0 || *((c + 0x49) as *const u8) == 0) && testar(this, p, e.as_mut_ptr()) != 0 {
                acrescentar(this, e.as_ptr());
            }
        }
        i += 1;
        p = p.wrapping_add(0x6e);
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpr8_d(this + 4), cpr8_d(this + 8) as u32, 0x1a, 0x7ae2a0);
}

/// As regiões que as duas decisões mudam: o sorteio, [0xae1080], a saída e a data estática da 0x54cdb0.
unsafe fn cpr8_regioes(saida: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&[(0xae1080, 4), (saida, 0x1a), (0xae2470, 8), (0xae2418, 1)]);
    reg
}

/// Comparador: lado a lado pelo sorteio, [0xae1080], a saída e a data estática (a cópia sem contar), e a devolução.
unsafe extern "thiscall" fn comparar_aposentadoria_no_inicio(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1098);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&cpr8_regioes(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| aposentadoria_no_inicio(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("aposentadoria_no_inicio(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: como o da 0x7a79c0.
unsafe extern "thiscall" fn comparar_aposentadoria_por_idade(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1099);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&cpr8_regioes(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| aposentadoria_por_idade(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("aposentadoria_por_idade(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ponytail: a lista inicial acrescenta (realloc) e sorteia em cada teste: não roda duas vezes. Só a original; o modo
// Rust e a carreira provam a cópia.
unsafe extern "thiscall" fn comparar_lista_inicial_de_aposentadorias(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1100)(this)
}

// ---------- 254a leva: a busca de novas aposentadorias, a saída dos cargos de quem se aposenta e o registro da liberação (player_regen.cpp, staff_records.cpp) ----------

unsafe fn cpr9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpr9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpr9_zerar(a: usize) {
    core::ptr::write_unaligned(a as *mut u32, 0);
}

/// { 0x7a5790 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::find_new_retirements` (nos dias da 0x5d6250): cada pessoa (0x6e bytes em [0xae23bc], até o fim
/// calculado com a conta [0xae2364] do começo). Ficam de fora: menos de 30 anos (+0x18) em clube de competição que roda
/// no jogo ([[0xadadfc] + id × 4] da competição +0x57); id a partir de [0xae2364] − 16 (relida a cada passo); os bits
/// 0x30 do +0xb do registro de 79 bytes ([0xae2d0c]); quem já está na lista ([this+4], 26 bytes, conta +8); contrato
/// (0x4cf5a0) com +0x49. Os outros: aposentadoria pela idade (0x7a7310) ou, sem ela, do desempregado (0x7a8050); quem
/// se aposenta entra na lista (0x7a8510). No fim o qsort da lista (0x7ae2a0).
unsafe extern "thiscall" fn novas_aposentadorias(this: usize) {
    // ponytail: a original constrói as duas datas da entrada (0x54a110) e deixa o resto como estava na pilha; as duas
    // decisões gravam tudo o que a 0x7a8510 copia antes de devolver 1 (a terceira data local nunca é lida: fica de fora)
    let mut e = [0u8; 0x1a];
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    padrao(e.as_mut_ptr().add(8));
    padrao(e.as_mut_ptr().add(0x10));
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let pela_idade: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x7a7310usize);
    let desempregado: unsafe extern "stdcall" fn(usize, *mut u8) -> u8 = core::mem::transmute(0x7a8050usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x7a8510usize);
    let mut p = cpr9_d(0xae23bc);
    let fim = p.wrapping_add(cpr9_d(0xae2364).wrapping_mul(0x6e));
    while p < fim {
        'x: {
            if cpr9_b(p + 0x18) < 0x1e {
                let c = cpr9_d(p + 0x39);
                if c != 0 {
                    let lg = cpr9_d(c + 0x57);
                    if lg != 0 && cpr9_d(cpr9_d(0xadadfc).wrapping_add(cpr9_d(lg).wrapping_mul(4))) != 0 {
                        break 'x;
                    }
                }
            }
            let id = cpr9_d(p) as i32;
            if id >= (cpr9_d(0xae2364) as i32).wrapping_sub(0x10) {
                break 'x;
            }
            if *((cpr9_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
                break 'x;
            }
            let k = cpr9_d(this + 8) as i32;
            if k != 0 {
                let l = cpr9_d(this + 4);
                for i in 0..k.max(0) as usize {
                    if cpr9_d(l + i * 0x1a) as i32 == id {
                        break 'x;
                    }
                }
            }
            let c = de(cpr9_d(0xae1050), p);
            if c != 0 && *((c + 0x49) as *const u8) != 0 {
                break 'x;
            }
            if pela_idade(this, p, e.as_mut_ptr()) != 0 {
                acrescentar(this, e.as_ptr());
            } else if desempregado(p, e.as_mut_ptr()) != 0 {
                crate::comparacao::ramo("cpr_9 desempregado");
                acrescentar(this, e.as_ptr());
            }
        }
        p = p.wrapping_add(0x6e);
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpr9_d(this + 4), cpr9_d(this + 8) as u32, 0x1a, 0x7ae2a0);
}

/// Tira a pessoa da vaga do `cargo` (tipo com sinal) no clube `c` (o da pessoa em +`campo`: 0x39 o clube, 0x24 a
/// seleção): 1 presidente (+0xbf); 2 as três de +0xc3; 5 e 12 técnico (+0xcf, só se é ele): no 5, sendo humano (id a
/// partir de [0xae2364] − 16), as ofertas do clube caem (0x8c4950); sai do cargo (0x69b790 com 3, 0, 0) e, se o clube
/// pode ser dirigido (0x544820), a vaga abre (0x69ba50); 6 auxiliar (+0xd3); 8 e 15 as cinco de +0x19f, depois da
/// distribuição dos treinadores do clube humano (0x600a50 com 1, 1; 0x8b2880 com 0); 9 as sete de +0x1b3 e, no
/// clube humano, +`campo` = 0 e as listas dos olheiros do técnico (0x7f7270); 10 as três de +0x1cf.
unsafe fn cpr9_sair_da_vaga(pessoa: usize, c: usize, cargo: i8, campo: usize) {
    let limpar = |base: usize, n: usize| {
        for k in 0..n {
            let a = c + base + 4 * k;
            if cpr9_d(a) == pessoa {
                cpr9_zerar(a);
            }
        }
    };
    let humano = || {
        let f: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        f(c, 1, 1, 0, 0) != 0
    };
    match cargo {
        1 => cpr9_zerar(c + 0xbf),
        2 => limpar(0xc3, 3),
        5 | 12 => {
            if cpr9_d(c + 0xcf) != pessoa {
                return;
            }
            crate::comparacao::ramo("cpr_9 tecnico");
            if cargo == 5 && (cpr9_d(pessoa) as i32) >= (cpr9_d(0xae2364) as i32).wrapping_sub(0x10) {
                let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4950usize);
                cancelar(cpr9_d(0xde1f54), c);
            }
            let sair: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
            sair(cpr9_d(0xb67a44), c, 3, 0, 0);
            let pode: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544820usize);
            if pode(c) as u8 != 0 {
                let abrir: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ba50usize);
                abrir(cpr9_d(0xb67a44), c, pessoa); // cpr9: a vaga do técnico abre
            }
        }
        6 => cpr9_zerar(c + 0xd3),
        8 | 15 => {
            if humano() {
                let treinadores: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b2880usize);
                treinadores(cpr9_d(0xde1f44), c, 0);
            }
            crate::comparacao::ramo("cpr_9 treinador");
            limpar(0x19f, 5); // cpr9: as vagas de treinador
        }
        9 => {
            limpar(0x1b3, 7);
            if humano() {
                cpr9_zerar(pessoa + campo);
                let listas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f7270usize);
                listas(cpr9_d(0xb74448), cpr9_d(c + 0xcf));
            }
        }
        10 => limpar(0x1cf, 3),
        _ => {}
    }
}

/// { 0x7aaeb0 } (stdcall, ret 4, player_regen.cpp): `player_regen_retire_from_non_playing_staff(pessoa)` (pela
/// 0x7a59e0, na aposentadoria): sai da vaga do cargo (+0x3d) no clube (+0x39) e da do cargo na seleção (+0x28) na
/// seleção (+0x24) (cpr9_sair_da_vaga); clube e seleção zerados, e os dois tipos de cargo.
unsafe extern "stdcall" fn sair_dos_cargos(pessoa: usize) {
    let c = cpr9_d(pessoa + 0x39);
    if c != 0 {
        cpr9_sair_da_vaga(pessoa, c, cpr9_b(pessoa + 0x3d), 0x39);
        cpr9_zerar(pessoa + 0x39);
    }
    let s = cpr9_d(pessoa + 0x24);
    if s != 0 {
        cpr9_sair_da_vaga(pessoa, s, cpr9_b(pessoa + 0x28), 0x24);
        cpr9_zerar(pessoa + 0x24);
    }
    *((pessoa + 0x3d) as *mut u8) = 0;
    *((pessoa + 0x28) as *mut u8) = 0;
}

/// { 0x86d800 } (thiscall, ret 4, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_free_release_records(pessoa)`: o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o clube
/// (registro de 0x245 bytes pelo +4). Com técnico no clube (+0xcf), o histórico dele (tipo 0x68: o clube, a pessoa e os
/// três ids +0x33 de +4, +8 e +0xc dela); depois o da pessoa (tipo 0x67: o clube ou −1), no arquivo (0x58e340).
/// Pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn registrar_a_liberacao(this: usize, pessoa: usize) {
    crate::comparacao::ramo("cpr_9 liberado");
    let mut e = cpq4_entrada();
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = de(cpr9_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpr9_d(0xae23b4).wrapping_add(cpr9_d(c + 4).wrapping_mul(0x245)) } else { 0 };
    if clube != 0 && cpr9_d(clube + 0xcf) != 0 {
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x68;
        cpq4_por(&mut e, 8, cpq4_i(clube));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_historico(true, this, cpr9_d(cpr9_d(clube + 0xcf)), &e);
    }
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x67;
    cpq4_por(&mut e, 8, cpq4_id_ou_menos_1(clube));
    if pessoa != 0 {
        cpq4_historico(true, this, cpr9_d(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
}

// ponytail: a busca acrescenta (realloc) e sorteia em cada teste: não roda duas vezes. Só a original; o modo Rust e a
// carreira provam a cópia.
unsafe extern "thiscall" fn comparar_novas_aposentadorias(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1101)(this)
}

/// A saída dos cargos só mexe na pessoa e nos dois clubes? Não quando é técnico do clube (5, 12) nem nos cargos que
/// chamam rotinas do clube humano (8, 15, 9).
unsafe fn cpr9_so_memoria(pessoa: usize) -> bool {
    for (campo, tipo) in [(0x39usize, 0x3dusize), (0x24, 0x28)] {
        let c = cpr9_d(pessoa + campo);
        if c == 0 {
            continue;
        }
        let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        match cpr9_b(pessoa + tipo) {
            5 | 12 if cpr9_d(c + 0xcf) == pessoa => return false,
            8 | 15 | 9 if humano(c, 1, 1, 0, 0) != 0 => return false,
            _ => {}
        }
    }
    true
}

/// Comparador: sem rotinas com efeito (cpr9_so_memoria), lado a lado pela pessoa (0x6e bytes) e pelos dois clubes
/// (0x245 bytes cada); senão (vaga do técnico, cancelamento de ofertas, listas do humano) só a original.
unsafe extern "stdcall" fn comparar_sair_dos_cargos(pessoa: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(1102);
    if !cpr9_so_memoria(pessoa) {
        return o(pessoa);
    }
    let mut reg = cpr9_regioes(pessoa);
    reg.retain(|r| r.0 != 0);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || sair_dos_cargos(pessoa));
    PLACAR.registrar(igual, || format!("sair_dos_cargos(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

unsafe fn cpr9_regioes(pessoa: usize) -> Vec<(usize, usize)> {
    vec![(pessoa, 0x6e), (cpr9_d(pessoa + 0x39), 0x245), (cpr9_d(pessoa + 0x24), 0x245)]
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia sem ele só grava [0xae1080]: só a
// original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_a_liberacao(this: usize, pessoa: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1103)(this, pessoa)
}

// ---------- 254a leva: o estado da lesão do jogador, a data da decisão sobre as ofertas e os candidatos a técnico do ano (injury.cpp, contract_manager.cpp, staff_records.cpp) ----------

unsafe fn cpr10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpr10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpr10_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O estado pela lesão atual da pessoa (o índice em [this] + id × 31 + 0xe; −1 sem lesão): a diferença da data do jogo
/// para a da lesão (+0x20 do registro de 48 bytes em [this+4], 0x54cc30) em anos × 365 + dias; abaixo de +0x2a, 2
/// (lesionado); abaixo de +0x28, 1 (recuperando a forma); senão 0.
unsafe fn cpr10_estado(this: usize, pessoa: usize) -> u8 {
    let id = cpr10_d(pessoa);
    let k = cpr10_d(cpr10_d(this).wrapping_add(id.wrapping_mul(31)) + 0xe) as i32;
    if k == -1 {
        return 0;
    }
    let l = cpr10_d(this + 4).wrapping_add((k as usize).wrapping_mul(48));
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), cpr10_d(l + 0x20) as u32, cpr10_d(l + 0x24) as u32);
    let dias = (cpr10_u(t.as_ptr() as usize + 2) as i32).wrapping_mul(365).wrapping_add(cpr10_w(t.as_ptr() as usize) as i32);
    if dias < cpr10_w(l + 0x2a) as i32 {
        return 2; // cpr10: lesionado
    }
    (dias < cpr10_w(l + 0x28) as i32) as u8 // cpr10: recuperando a forma
}

/// { 0x62c400 } (thiscall, ret 4, injury.cpp): `Injury::get_player_injury_status(pessoa)`: pessoa nula, a caixa
/// "Error" (linha 0x556) e 0; id a partir de [0xae2364], 0. Senão a forma do dia (0x62c680) e o estado
/// (cpr10_estado). Quente (escalação automática): sem contador.
unsafe extern "thiscall" fn estado_da_lesao(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        aviso_em(0x9e7f28, 0x556);
        return 0;
    }
    if cpr10_d(pessoa) as i32 >= cpr10_d(0xae2364) as i32 {
        return 0;
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    atualizar(this, pessoa);
    cpr10_estado(this, pessoa)
}

unsafe fn cpr10_mais(d: usize, n: u32, t: &mut [u8; 8]) -> usize {
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    mais(d, t.as_mut_ptr(), n)
}

/// A data `d` = a data em `p` (0x417c10).
unsafe fn cpr10_copiar(d: usize, p: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(d, t.as_mut_ptr(), p);
}

/// A data `d` = a data do jogo + `n` dias (0x54c5b0, 0x417c10).
unsafe fn cpr10_hoje_mais(d: usize, n: u32) {
    let mut t = [0u8; 8];
    cpr10_copiar(d, cpr10_mais(0xae2c90, n, &mut t));
}

/// A data `d` passa para o dia seguinte quando a fatia (+0xc da entrada, com sinal) passa de 2 (menos 3).
unsafe fn cpr10_acertar_fatia(e: usize, f: i8) {
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
unsafe fn cpr10_procurar(this: usize, id: i32, limite: &[u8; 8], alterar: bool) -> usize {
    let conta = cpr10_d(this + 0x14) as i32;
    let base = cpr10_d(this + 0xc);
    let (la, ld) = (u16::from_le_bytes([limite[2], limite[3]]), i16::from_le_bytes([limite[0], limite[1]]));
    let mut i = cpr10_d(this + 0x18) as i32;
    let mut anulada = -1i32;
    while i < conta {
        let r = base.wrapping_add((i as usize).wrapping_mul(14));
        let ra = cpr10_u(r + 6);
        if la < ra || (la == ra && ld < cpr10_w(r + 4)) {
            break;
        }
        if *((r + 0xd) as *const u8) == 3 && cpr10_d(r) as i32 == id {
            if alterar {
                core::ptr::write_unaligned(r as *mut u32, u32::MAX);
            }
            anulada = i;
            break;
        }
        i += 1;
    }
    let conta = cpr10_d(this + 0x14) as i32;
    i = i.max(conta.wrapping_sub(cpr10_d(this + 0x1c) as i32));
    while i < conta {
        let r = cpr10_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(14));
        if *((r + 0xd) as *const u8) == 3 && cpr10_d(r) as i32 == id && i != anulada {
            return r;
        }
        i += 1;
    }
    0
}

/// { 0x4cd230 } (thiscall, ret 8, contract_manager.cpp, SEH só da data local, de destrutor vazio):
/// `CONTRACT_MANAGER::schedule_offer_decision(id, vaga)`: a entrada de decisão de tipo 3 da pessoa (cpr10_procurar, com
/// o limite de hoje + 14; sem ela, uma nova em hoje + 1 + sorteio(5), fatia 2, pela 0x4cd6d0). Com a vaga da oferta
/// (0x8c61e0): o prazo da resposta (0x8cc5b0: dias × 3 + fatias) contra o que falta (dias até a data × 3 + fatias,
/// 0x54cdb0), o menor, no mínimo 1, vira a data (hoje + ÷3) e a fatia (+ resto, passando de 2 vai ao dia seguinte);
/// oferta com +0x2f e menos de 4 dias: hoje + 4 + sorteio(2). Sem vaga (−1): chegada a data, ela vira hoje e a fatia
/// sobe (até a do dia [0xae2c88]); se o contrato da pessoa ([this] + 80 × índice do registro de 79 bytes) acaba antes
/// de hoje + 180, hoje + 4 + sorteio(4). No fim, a data não passa de hoje + 14, e +0x1c sobe.
unsafe extern "thiscall" fn marcar_decisao_da_oferta(this: usize, id: i32, vaga: i32) {
    let mut limite = [0u8; 8];
    cpr10_mais(0xae2c90, 0xe, &mut limite);
    let mut e = cpr10_procurar(this, id, &limite, true);
    let pessoa = cpr10_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if e == 0 {
        crate::comparacao::ramo("cpr_10 nova decisao");
        let mut d = [0u8; 8];
        cpr10_mais(0xae2c90, (sortear(5) + 1) as u32, &mut d); // cpr10: a decisão em 1 a 5 dias
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, *const u8, u32, u32) = core::mem::transmute(0x4cd6d0usize);
        acrescentar(this, pessoa, d.as_ptr(), 2, 3);
        let n = cpr10_d(this + 0x14);
        e = cpr10_d(this + 0xc).wrapping_add(n.wrapping_sub(1).wrapping_mul(14));
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let data = e + 4;
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
        let of = vaga_da(cpr10_d(0xde1f54), vaga);
        // ponytail: a original passa as vagas dos dois argumentos como saída; começam com os bytes baixos deles
        let (mut a, mut b) = (id as u8, vaga as u8);
        let prazo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x8cc5b0usize);
        prazo(cpr10_d(0xde1f54), cpr10_d(of), &mut a, &mut b);
        let alvo = a.wrapping_mul(3).wrapping_add(b) as i8;
        let falta = dias(data, cpr10_d(0xae2c90) as u32, cpr10_d(0xae2c94) as u32);
        let mut f = (*((e + 0xc) as *const u8)).wrapping_sub(*(0xae2c88 as *const u8)) as i8;
        let mut dl = falta as u8;
        if f < 0 {
            f = f.wrapping_add(3);
            dl = dl.wrapping_sub(1);
        }
        let atual = dl.wrapping_mul(3).wrapping_add(f as u8) as i8;
        let x = if atual < alvo { atual.max(1) } else { alvo.max(1) } as i32;
        cpr10_hoje_mais(data, (x / 3) as u32);
        cpr10_acertar_fatia(e, ((x % 3) as u8).wrapping_add(*(0xae2c88 as *const u8)) as i8);
        if *((cpr10_d(of) + 0x2f) as *const u8) != 0 && dias(data, cpr10_d(0xae2c90) as u32, cpr10_d(0xae2c94) as u32) < 4 {
            cpr10_hoje_mais(data, (sortear(2) + 4) as u32);
        }
    } else {
        let (td, ta) = (cpr10_w(0xae2c90), cpr10_u(0xae2c92));
        let (rd, ra) = (cpr10_w(data), cpr10_u(data + 2));
        if ta > ra || (ta == ra && td >= rd) {
            cpr10_copiar(data, 0xae2c90);
            let f = *((e + 0xc) as *const i8);
            if f as i32 <= *(0xae2c88 as *const u8) as i32 {
                cpr10_acertar_fatia(e, f.wrapping_add(1));
            }
        }
        let p = cpr10_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        if p != 0 && (cpr10_d(p) as i32) < cpr10_d(0xae2364) as i32 {
            let x = cpr10_d(cpr10_d(0xae2d0c).wrapping_add(cpr10_d(p).wrapping_mul(79))) as i32;
            let c = if x >= 0 { cpr10_d(this).wrapping_add((x as usize).wrapping_mul(0x50)) } else { 0 };
            if c != 0 {
                let mut t = [0u8; 8];
                let m = cpr10_mais(0xae2c90, 0xb4, &mut t);
                let (ma, ca) = (cpr10_u(m + 2), cpr10_u(c + 0x2f));
                if ma > ca || (ma == ca && cpr10_w(m) > cpr10_w(c + 0x2d)) {
                    cpr10_hoje_mais(data, (sortear(4) + 4) as u32);
                }
            }
        }
    }
    let (td, ta) = (cpr10_w(0xae2c90), cpr10_u(0xae2c92));
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut t = [0u8; 8];
    let m = menos(data, t.as_mut_ptr(), 0xe);
    let ma = cpr10_u(m + 2);
    if ta < ma || (ta == ma && td < cpr10_w(m)) {
        cpr10_hoje_mais(data, 0xe);
    }
    core::ptr::write_unaligned((this + 0x1c) as *mut u32, cpr10_d(this + 0x1c).wrapping_add(1) as u32);
}

/// { 0x86fab0 } (thiscall, ret 8, staff_records.cpp, SEH sem objeto com destrutor):
/// `Staff_Records::get_manager_of_year_candidates(clubes, n)` (pela 0x768940): o qsort dos `n` clubes (0x870220) e, na
/// ordem, até três cujo técnico (+0xcf) chegou (+0x3e) antes de hoje − 28 semanas (0x54cab0; ano sem sinal, dia com
/// sinal) e tem registro (0x86f5b0 com 1) com jogos (+0x8e), copiados para o começo do próprio vetor. Devolve quantos.
unsafe extern "thiscall" fn candidatos_a_tecnico_do_ano(this: usize, clubes: usize, n: u32) -> u8 {
    let semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let mut lim = [0u8; 8];
    semanas(0xae2c90, lim.as_mut_ptr(), 0x1c);
    let n = n as u16 as i16 as i32;
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(clubes, n as u32, 4, 0x870220);
    let (ld, la) = (i16::from_le_bytes([lim[0], lim[1]]), u16::from_le_bytes([lim[2], lim[3]]));
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let mut k = 0u8;
    for i in 0..n.max(0) as usize {
        if k as i8 >= 3 {
            break;
        }
        let tec = cpr10_d(cpr10_d(clubes + 4 * i) + 0xcf);
        if tec == 0 {
            continue;
        }
        let ta = cpr10_u(tec + 0x40);
        if !(la > ta || (la == ta && ld > cpr10_w(tec + 0x3e))) {
            continue;
        }
        let r = registro(this, tec, 1);
        if r != 0 && cpr10_w(r + 0x8e) != 0 {
            core::ptr::write_unaligned((clubes + 4 * k as usize) as *mut u32, cpr10_d(clubes + 4 * i) as u32);
            k += 1;
        }
    }
    k
}

/// Comparador: a original (com a forma do dia) e o estado relido pela regra (cpr10_estado, sem rodar a 0x62c680 de
/// novo: a original já a rodou).
unsafe extern "thiscall" fn comparar_estado_da_lesao(this: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1104)(this, pessoa);
    if pessoa != 0 && (cpr10_d(pessoa) as i32) < cpr10_d(0xae2364) as i32 {
        let n = cpr10_estado(this, pessoa);
        PLACAR.registrar(r == n, || format!("estado_da_lesao(pessoa {:x}) original={} rust={}", pessoa, r, n));
    }
    r
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const CPR10_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// Comparador: sem a entrada (a 0x4cd6d0 acrescenta e pode realocar) só a original; com ela, lado a lado pelo sorteio,
/// as datas estáticas, o vetor das decisões e +0x1c (a cópia sem contar).
unsafe extern "thiscall" fn comparar_marcar_decisao_da_oferta(this: usize, id: i32, vaga: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32)>(1105);
    let mut limite = [0u8; 8];
    cpr10_mais(0xae2c90, 0xe, &mut limite);
    if cpr10_procurar(this, id, &limite, false) == 0 {
        return o(this, id, vaga);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&CPR10_ESTATICAS);
    reg.push((this + 0x1c, 4));
    reg.push((cpr10_d(this + 0xc), cpr10_d(this + 0x14).wrapping_mul(14)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id, vaga),
        || crate::aleatorio::sem_contar(|| marcar_decisao_da_oferta(this, id, vaga)));
    PLACAR.registrar(igual, || format!("marcar_decisao_da_oferta(id {}, vaga {}) {:x?} x {:x?}", id, vaga, mo, mr));
}

/// Comparador: lado a lado pelo vetor dos clubes e pela data estática da 0x54cab0, e a devolução (os registros dos
/// técnicos que a 0x86f5b0 cria na original já existem para a cópia).
unsafe extern "thiscall" fn comparar_candidatos_a_tecnico_do_ano(this: usize, clubes: usize, n: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(1106);
    let k = (n as u16 as i16).max(0) as usize;
    let reg = [(clubes, 4 * k), (0xae24c8, 8), (0xae24aa, 1)];
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, n), || candidatos_a_tecnico_do_ano(this, clubes, n));
    PLACAR.registrar(igual && r == c, || format!("candidatos_a_tecnico_do_ano({}) original={} rust={} {:x?} x {:x?}", k, r, c, mo, mr));
    r
}

// ---------- 254a leva: a projeção da liga para a diretoria: as posições projetadas do clube e as duas ordens da projeção (league.cpp) ----------

unsafe fn cpr11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A entrada da projeção (0xe bytes, no bloco da 0x944cf1): +0 o clube, +4 os pontos por jogo (float), +8 os pontos,
/// +0xa os pontos projetados, +0xc os jogos.
unsafe fn cpr11_entrada(buf: usize, i: i32) -> usize {
    buf.wrapping_add((i as isize * 0xe) as usize)
}

/// { 0x68afb0 } (cdecl, league.cpp): `compare_projection_by_reputation_desc(a, b)`: o qsort das entradas da projeção
/// pela reputação do clube (+0 → +0x80, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_da_projecao_pela_reputacao(a: usize, b: usize) -> i32 {
    let ra = cpr11_w(cpr11_d(a) + 0x80) as i32;
    let rb = cpr11_w(cpr11_d(b) + 0x80) as i32;
    rb - ra // o clube de maior reputação primeiro
}

/// { 0x68afd0 } (cdecl, league.cpp): `compare_projection_by_points_desc(a, b)`: o qsort das entradas da projeção pelos
/// pontos projetados (+0xa, palavra com sinal), o maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_da_projecao_pelos_pontos(a: usize, b: usize) -> i32 {
    cpr11_w(b + 0xa) as i32 - cpr11_w(a + 0xa) as i32 // os mais pontos primeiro
}

/// A posição (1..) do clube na projeção já ordenada (a primeira entrada dele; `None` sem ela).
unsafe fn cpr11_posicao(buf: usize, n: i16, clube: usize) -> Option<i32> {
    (0..n as i32).find(|&i| cpr11_d(cpr11_entrada(buf, i)) == clube)
}

/// O clube com outra forma: os pontos por jogo (+4) vezes `fator` (o float guardado; o x87 segue com o de 53 bits) e
/// os pontos projetados de novo com ele.
unsafe fn cpr11_outra_forma(this: usize, e: usize, fator: f64) -> i32 {
    let r = *((e + 4) as *const f32) as f64 * fator;
    *((e + 4) as *mut f32) = r as f32; // fst: guarda o float e segue com o de 53 bits
    let resta = cpr11_w(this + 0xa7) as i32 - cpr11_w(e + 0xc) as i32;
    *((e + 0xa) as *mut i16) = (ftol(resta as f64 * r) as i16).wrapping_add(cpr11_w(e + 8));
    resta
}

/// { 0x68ac40 } (thiscall, ret 4, league.cpp; virtual +0x84 das ligas, chamada só por Comp_Stats 0x4ae300, que a
/// diretoria usa na confiança semanal 0x697c40 e no motivo da saída do técnico 0x69b350):
/// `League::update_projected_positions(clube)`: com o clube na tabela (+0xb1, 0x3b bytes, conta +0x3e: +0 o clube,
/// +0x11 os jogos, +0x19 os pontos), monta num bloco (0x944cf1) uma entrada de 0xe bytes por linha com os pontos por
/// jogo; no primeiro terço da temporada (jogos do clube × 3 < +0xa7), troca-os por uma escala pela reputação
/// (0,83 × +0xc2 do primeiro ao 0,23 × +0xc2 abaixo do último). Projeta os pontos no fim da temporada, ordena e acha a
/// posição do clube na forma normal, melhor (× 1,5) e pior (× 0,444); libera o bloco e entrega as três à virtual
/// +0x68 (`set_club_progress(clube, competição +0x44, normal, melhor, pior)`, que grava os bytes +0x73..+0x75 do
/// registro do clube no Comp_Stats). Devolve 1; fora da tabela, 0.
unsafe extern "thiscall" fn projetar_posicoes_da_liga(this: usize, clube: usize) -> u32 {
    let n = cpr11_w(this + 0x3e);
    let tabela = cpr11_d(this + 0xb1);
    let linha = match (0..n as i32).map(|i| tabela.wrapping_add((i as isize * 0x3b) as usize)).find(|&l| cpr11_d(l) == clube) {
        Some(l) => l,
        None => return 0,
    };
    if linha == 0 || n == 0 {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let buf = novo((n as i32 * 0xe) as u32);
    let mut i: i16 = 0;
    while i < cpr11_w(this + 0x3e) {
        let l = cpr11_d(this + 0xb1).wrapping_add((i as isize * 0x3b) as usize);
        let e = cpr11_entrada(buf, i as i32);
        core::ptr::write_unaligned(e as *mut usize, cpr11_d(l));
        *((e + 8) as *mut i16) = cpr11_w(l + 0x19);
        let jogos = *((l + 0x11) as *const i8);
        *((e + 0xc) as *mut i16) = jogos as i16;
        *((e + 4) as *mut f32) = if jogos > 0 { (cpr11_w(l + 0x19) as f64 / jogos as f64) as f32 } else { 0.0 };
        i += 1;
    }
    // os três locais da original (dwords): o byte baixo de cada um vira a posição; os bytes altos são o que sobrou
    // neles (o float A e a conta das linhas na escala pela reputação, o que resta de jogos do clube no terceiro)
    let (mut normal, mut melhor, mut pior) = (0u32, 0u32, 0u32);
    if (*((linha + 0x11) as *const i8) as i32) * 3 < cpr11_w(this + 0xa7) as i32 {
        crate::comparacao::ramo("cpr_11 projecao pela reputacao");
        let n = cpr11_w(this + 0x3e);
        qsort(buf, n as i32 as u32, 0xe, 0x68afb0);
        let c = *((this + 0xc2) as *const i8) as f64;
        let a = (c * f64_em(0x96d0e8)) as f32; // fstp no local: o primeiro arredonda para float
        normal = a.to_bits();
        let b = c * f64_em(0x96d0e0);
        if n > 0 {
            let d = a as f64 - b;
            let passo = d / n as f64;
            for k in 0..n as i32 {
                *((cpr11_entrada(buf, k) + 4) as *mut f32) = (a as f64 - passo * k as f64) as f32; // do maior ao menor
            }
            melhor = n as i32 as u32;
        }
    }
    let mut i: i16 = 0;
    while i < cpr11_w(this + 0x3e) {
        let e = cpr11_entrada(buf, i as i32);
        let resta = cpr11_w(this + 0xa7) as i32 - cpr11_w(e + 0xc) as i32;
        pior = resta as u32;
        *((e + 0xa) as *mut i16) = (ftol(resta as f64 * *((e + 4) as *const f32) as f64) as i16).wrapping_add(cpr11_w(e + 8));
        i += 1;
    }
    qsort(buf, cpr11_w(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = cpr11_posicao(buf, cpr11_w(this + 0x3e), clube) {
        normal = (normal & !0xff) | (k as u8).wrapping_add(1) as u32; // a posição na forma normal
        pior = cpr11_outra_forma(this, cpr11_entrada(buf, k), f64_em(0x968940)) as u32;
    }
    qsort(buf, cpr11_w(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = cpr11_posicao(buf, cpr11_w(this + 0x3e), clube) {
        melhor = (melhor & !0xff) | (k as u8).wrapping_add(1) as u32;
        pior = cpr11_outra_forma(this, cpr11_entrada(buf, k), f64_em(0x96d0d8)) as u32;
    }
    qsort(buf, cpr11_w(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    for k in 0..cpr11_w(this + 0x3e) as i32 {
        if cpr11_d(cpr11_entrada(buf, k)) == clube {
            pior = (pior & !0xff) | (k as u8).wrapping_add(1) as u32; // a última (o clube está uma vez só)
        }
    }
    liberar(buf);
    crate::comparacao::ramo("cpr_11 projecao");
    // ponytail: os bytes altos que a original deixa nos três locais fora da escala pela reputação são sobra de pilha
    // (aqui, zero); as 69 versões da virtual +0x68 só leem o byte baixo de cada argumento (ou repassam os dwords a
    // outra que lê)
    let progresso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) =
        core::mem::transmute(cpr11_d(cpr11_d(this) + 0x68));
    let competicao = (melhor & !0xff) | *((this + 0x44) as *const u8) as u32; // dl sobre o edx do segundo local
    progresso(this, clube, competicao, normal, melhor, pior);
    1
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_da_projecao_pela_reputacao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1108)(a, b);
    let n = ordem_da_projecao_pela_reputacao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_projecao_pela_reputacao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_da_projecao_pelos_pontos(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1109)(a, b);
    let n = ordem_da_projecao_pelos_pontos(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_projecao_pelos_pontos({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: o bloco da projeção é alocado e liberado dentro (não fica nada dele); o efeito é a virtual +0x68, que
/// grava os bytes +0x73..+0x75 do registro do clube no Comp_Stats da liga (+8, achado pela 0x4a4850, que dentro da
/// 0x4ae300 devolve o registro provisório dela): lado a lado nesses 3 bytes e na devolução. As duas versões que repassam
/// a outra competição (0x48e380, 0x7ef100) gravam fora da região: aí só a devolução pesa.
unsafe extern "thiscall" fn comparar_projetar_posicoes_da_liga(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1107);
    let stats = cpr11_d(this + 8);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let reg = if stats != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(this, clube); // ponytail: sem registro para olhar, só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(this, clube),
        || projetar_posicoes_da_liga(this, clube));
    PLACAR.registrar(igual && r == n, || format!("projetar_posicoes_da_liga(liga {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}",
        this, clube, r, n, mo, mr));
    r
}

// ---------- 254a leva: o calendário dos jogos: achar os jogos de um clube ou competição entre duas datas, os jogos da competição num dia e se a competição tem jogo na data (fix_man.cpp) ----------

unsafe fn cpr12_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// A lista dos jogos de um ano, dia, fatia e nação-chave no calendário `fm` (0x5abf10; cria o bloco do ano se a
/// corrente não o tem). A original compara o ano como palavra e mascara o dia, a fatia e a nação.
unsafe fn cpr12_lista(fm: usize, ano: u32, dia: u32, fatia: u32, nacao: u32) -> usize {
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    lista(fm, ano, dia, fatia, nacao)
}

/// O próximo item da lista (0x68d1b0: o cursor +0xa anda).
unsafe fn cpr12_proximo(l: usize) -> usize {
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    prox(l)
}

/// Põe o cursor da lista no começo (+0xa = +2) e devolve o primeiro item.
unsafe fn cpr12_primeiro(l: usize) -> usize {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpr12_d(l + 2));
    cpr12_proximo(l)
}

/// Acrescenta à lista `l` uma cópia dos 0x4f bytes do jogo `f` (0x68d0d0); 0 se falhou.
unsafe fn cpr12_copiar(l: usize, f: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    copiar(l, f, 0x4f)
}

/// A nação-chave ativa da competição `comp` (0x48dd70 sobre o registro [0xae23d0] + 0x6b × comp, sem jogo).
unsafe fn cpr12_nacao_da_competicao(comp: u32) -> u8 {
    let nacao: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x48dd70usize);
    nacao((cpr12_d(0xae23d0).wrapping_add(comp.wrapping_mul(0x6b))) as usize, 0) as u8
}

/// O jogo `f` passa nos filtros da 0x5acbe0: tipo (+0x42; 0xfe qualquer), clube em casa (+0xc) ou fora (+0x10; −1
/// qualquer) com o lado dele (+0x40 em casa, +0x41 fora; 0xff qualquer) e a competição (+0; −1 qualquer).
unsafe fn cpr12_passa(f: usize, clube: u32, comp: u32, tipo: u8, lado: u8) -> bool {
    if tipo != 0xfe && tipo != *((f + 0x42) as *const u8) {
        return false;
    }
    if clube != u32::MAX {
        let casa = cpr12_d(f + 0xc);
        if clube != casa && clube != cpr12_d(f + 0x10) {
            return false;
        }
        if lado != 0xff {
            let o_dele = if clube == casa { *((f + 0x40) as *const u8) } else { *((f + 0x41) as *const u8) };
            if o_dele != lado {
                return false;
            }
        }
    }
    comp == u32::MAX || comp == cpr12_d(f)
}

/// { 0x5acbe0 } (thiscall, ret 0x24, fix_man.cpp): `FIXTURE_MANAGER::find_fixtures(dia0, ano0, dia1, ano1, clube, lista,
/// comp, tipo, lado)`: conta (e, com `lista`, copia para ela) os jogos do calendário entre (dia0, ano0) e (dia1, ano1)
/// inclusive, dia a dia (366 por ano), nas três fatias de cada dia e nas listas das nações-chave: do clube (índice em
/// [0xae23b4], 0x245 bytes; seleção 0x53b900: as nações 0 e 1; senão 2..7, a da 0x48dee0 e a parceira 0x5af5b0, sem
/// repetir) ou só a da competição (0x48dd70). Filtros: tipo (0xfe qualquer), lado do clube (0xff qualquer) e
/// competição (−1 qualquer). Devolve a conta (palavra); a cópia que falha mostra a caixa (linha 0x93b) e para ali.
/// Quente (288 mil chamadas até 2002-06-01): sem contador.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn achar_jogos_do_calendario(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, clube: u32,
                                                      lista: usize, comp: u32, tipo: u32, lado: u32) -> u32 {
    let (tipo, lado) = (tipo as u8, lado as u8);
    let mut nacoes = [0xffu8; 8];
    let mut n = 0usize;
    if clube != u32::MAX {
        let reg = cpr12_d(0xae23b4).wrapping_add(clube.wrapping_mul(0x245)) as usize;
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(reg) != 0 {
            nacoes[0] = 0;
            nacoes[1] = 1;
            n = 2;
        } else {
            let do_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x48dee0usize);
            let parceira: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5af5b0usize);
            nacoes[..6].copy_from_slice(&[2, 3, 4, 5, 6, 7]);
            nacoes[6] = do_clube(reg) as u8;
            nacoes[7] = parceira(reg) as u8;
            n = 8;
            for i in 0..7 {
                if (i + 1..8).any(|j| nacoes[j] == nacoes[i]) {
                    nacoes[i] = 0xff; // repetida adiante
                }
            }
        }
    } else if (comp as i32) > -1 {
        nacoes[0] = cpr12_nacao_da_competicao(comp);
        n = 1;
    }
    let (mut dia, mut ano, mut total) = (dia0, ano0, 0u32);
    loop {
        if dia as u16 >= 0x16e {
            ano = ano.wrapping_add(1); // o ano seguinte, do dia 0
            dia = 0;
            continue;
        }
        if ano as u16 > ano1 as u16 || (ano as u16 == ano1 as u16 && dia as u16 > dia1 as u16) {
            return total & 0xffff; // passou do último dia
        }
        for fatia in 0..3u32 {
            for &nacao in &nacoes[..n] {
                if nacao == 0xff {
                    continue;
                }
                let l = cpr12_lista(this, ano, dia, fatia, nacao as u32);
                if l == 0 {
                    continue;
                }
                let mut f = cpr12_primeiro(l);
                while f != 0 {
                    if cpr12_passa(f, clube, comp, tipo, lado) {
                        if lista != 0 && cpr12_copiar(lista, f) == 0 {
                            aviso_em(0x9cc208, 0x93b);
                            return total & 0xffff;
                        }
                        total = total.wrapping_add(1);
                    }
                    f = cpr12_proximo(l);
                }
            }
        }
        dia = dia.wrapping_add(1);
    }
}

/// { 0x5abd90 } (thiscall, ret 0x1c, fix_man.cpp): `FIXTURE_MANAGER::collect_comp_fixtures_on_day(ano, dia, fatia,
/// comp, rodada, tipo, lista)` (do adiamento pela TV das ligas e da mudança das rodadas das copas para as datas fixas):
/// copia para `lista` os jogos do dia e fatia, na lista da nação-chave da competição (0x48dd70; −1: a nação 0), da
/// competição (+0; −1 qualquer), da rodada (+0x34, palavra; −1 qualquer) e do tipo (+0x42; 0xfe qualquer). Devolve
/// quantos (palavra); a cópia que falha mostra a caixa (linha 0x634) e para ali.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn jogos_da_competicao_no_dia(this: usize, ano: u32, dia: u32, fatia: u32, comp: u32, rodada: u32,
                                                       tipo: u32, lista: usize) -> u32 {
    let tipo = tipo as u8;
    // ponytail: a original empurra o local da nação com bytes altos de sobra de pilha; a 0x5abf10 só lê o byte
    let nacao = if comp != u32::MAX { cpr12_nacao_da_competicao(comp) } else { 0 };
    let mut total = 0u32;
    let l = cpr12_lista(this, ano, dia, fatia, nacao as u32);
    if l == 0 {
        return 0;
    }
    let mut f = cpr12_primeiro(l);
    while f != 0 {
        let mut ok = true;
        if comp != u32::MAX && comp != cpr12_d(f) {
            ok = false;
        }
        if rodada != u32::MAX && rodada as i32 != *((f + 0x34) as *const i16) as i32 {
            ok = false; // outra rodada
        }
        if (tipo == 0xfe || *((f + 0x42) as *const u8) == tipo) && ok {
            if cpr12_copiar(lista, f) == 0 {
                aviso_em(0x9cc208, 0x634);
                return total & 0xffff;
            }
            crate::comparacao::ramo("cpr_12 jogo da rodada");
            total = total.wrapping_add(1);
        }
        f = cpr12_proximo(l);
    }
    total & 0xffff
}

/// { 0x5ae6d0 } (stdcall, ret 8, fix_man.cpp; o `ecx` = [0xae2a58] que o chamador põe não é usado):
/// `fix_man_comp_has_fixture_on_date(data, comp)` (das eliminatórias da Eurocopa 0x57fd60): 1 se a competição tem
/// jogo (+0 = comp) na data em alguma das três fatias do calendário [0xae2a58], na lista da nação-chave dela
/// (0x48dd70). Competição −1 ou abaixo: 0; data nula: a caixa (linha 0xe89) e 0.
unsafe extern "stdcall" fn competicao_tem_jogo_na_data(data: usize, comp: u32) -> u8 {
    if (comp as i32) <= -1 {
        return 0;
    }
    let nacao = cpr12_nacao_da_competicao(comp);
    if data == 0 {
        aviso_em(0x9cc208, 0xe89);
        return 0;
    }
    let fm = cpr12_d(0xae2a58) as usize;
    // ponytail: a original empurra o ano e o dia (palavras) com bytes altos de sobra nos registradores; a 0x5abf10
    // compara o ano como palavra e mascara o dia (o dword do ano só segue se o bloco do ano faltar, e os dois anos da
    // carreira estão na corrente desde a criação do jogo)
    let (ano, dia) = (*((data + 2) as *const u16) as u32, *(data as *const u16) as u32);
    for fatia in 0..3u32 {
        let l = cpr12_lista(fm, ano, dia, fatia, nacao as u32);
        if l == 0 {
            continue;
        }
        let mut f = cpr12_primeiro(l);
        while f != 0 {
            if cpr12_d(f) == comp {
                return 1;
            }
            f = cpr12_proximo(l);
        }
    }
    0
}

/// Os itens que a original acrescentou à `lista` (depois da cauda de antes) são os da lista provisória `nova`, na
/// mesma ordem, 0x4f bytes cada?
unsafe fn cpr12_mesmos_itens(lista: usize, conta_antes: u16, cauda_antes: u32, nova: usize) -> bool {
    let acrescentados = (*(lista as *const u16)).wrapping_sub(conta_antes);
    if acrescentados != *(nova as *const u16) {
        return false;
    }
    let mut a = if conta_antes == 0 { cpr12_d(lista + 2) } else { cpr12_d(cauda_antes as usize + 6) } as usize;
    let mut b = cpr12_d(nova + 2) as usize;
    for _ in 0..acrescentados {
        if a == 0 || b == 0 {
            return false;
        }
        let (da, db) = (cpr12_d(a) as usize, cpr12_d(b) as usize);
        if core::slice::from_raw_parts(da as *const u8, 0x4f) != core::slice::from_raw_parts(db as *const u8, 0x4f) {
            return false;
        }
        a = cpr12_d(a + 6) as usize;
        b = cpr12_d(b + 6) as usize;
    }
    true
}

/// Roda a cópia com uma lista provisória (0x68d020; destruída pela 0x68d040) e confere os itens contra os que a
/// original acrescentou à lista real.
unsafe fn cpr12_com_lista_nova(lista: usize, conta_antes: u16, cauda_antes: u32, copia: impl FnOnce(usize) -> u32) -> (u32, bool) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut nova = [0u8; 0x10];
    let p = nova.as_mut_ptr() as usize;
    criar(p);
    let n = copia(p);
    let iguais = cpr12_mesmos_itens(lista, conta_antes, cauda_antes, p);
    destruir(p);
    (n, iguais)
}

/// Comparador: sem lista, pela devolução (a original e a cópia só andam os cursores das listas do calendário). Com
/// lista, a original enche a lista real e a cópia uma provisória: a conta (palavra) e os itens acrescentados.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_achar_jogos_do_calendario(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, clube: u32,
                                                               lista: usize, comp: u32, tipo: u32, lado: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32>(1110);
    if lista == 0 {
        let r = o(this, dia0, ano0, dia1, ano1, clube, 0, comp, tipo, lado);
        let n = achar_jogos_do_calendario(this, dia0, ano0, dia1, ano1, clube, 0, comp, tipo, lado);
        PLACAR.registrar(r as u16 == n as u16, || format!("achar_jogos_do_calendario({}/{} a {}/{}, clube {}, comp {}, tipo {:x}, lado {:x}) original={} rust={}",
            dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, clube as i32, comp as i32, tipo as u8, lado as u8, r as u16, n as u16));
        return r;
    }
    let (conta, cauda) = (*(lista as *const u16), cpr12_d(lista + 6));
    let r = o(this, dia0, ano0, dia1, ano1, clube, lista, comp, tipo, lado);
    let (n, iguais) = cpr12_com_lista_nova(lista, conta, cauda,
        |p| achar_jogos_do_calendario(this, dia0, ano0, dia1, ano1, clube, p, comp, tipo, lado));
    PLACAR.registrar(iguais && r as u16 == n as u16, || format!("achar_jogos_do_calendario({}/{} a {}/{}, clube {}, comp {}, tipo {:x}, lado {:x}, com lista) original={} rust={} itens iguais {}",
        dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, clube as i32, comp as i32, tipo as u8, lado as u8, r as u16, n as u16, iguais));
    r
}

/// Comparador: a original enche a lista real e a cópia uma provisória: a conta (palavra) e os itens acrescentados.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_jogos_da_competicao_no_dia(this: usize, ano: u32, dia: u32, fatia: u32, comp: u32, rodada: u32,
                                                                tipo: u32, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32>(1111);
    let (conta, cauda) = (*(lista as *const u16), cpr12_d(lista + 6));
    let r = o(this, ano, dia, fatia, comp, rodada, tipo, lista);
    let (n, iguais) = cpr12_com_lista_nova(lista, conta, cauda,
        |p| jogos_da_competicao_no_dia(this, ano, dia, fatia, comp, rodada, tipo, p));
    PLACAR.registrar(iguais && r as u16 == n as u16, || format!("jogos_da_competicao_no_dia({}/{} fatia {}, comp {}, rodada {}, tipo {:x}) original={} rust={} itens iguais {}",
        dia as u16, ano as u16, fatia as u8, comp as i32, rodada as i32, tipo as u8, r as u16, n as u16, iguais));
    r
}

/// Comparador: pela devolução (só anda os cursores das listas do calendário).
unsafe extern "stdcall" fn comparar_competicao_tem_jogo_na_data(data: usize, comp: u32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32) -> u8>(1112)(data, comp);
    let n = competicao_tem_jogo_na_data(data, comp);
    PLACAR.registrar(r == n, || format!("competicao_tem_jogo_na_data(data {:x}, comp {}) original={} rust={}", data, comp as i32, r, n));
    r
}

// ---------- 254a leva: o clube livre para o amistoso, o clube na copa e a nação parceira do calendário (friendly.cpp, Cup.cpp, fix_man.cpp) ----------

unsafe fn cpr13_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// { 0x5c6b70 } (stdcall, ret 0xc, friendly.cpp): `friendly_is_club_free_on_date(data, clube, tipo)` (da marcação dos
/// amistosos: excursões, torneios, respostas aos convites e a situação das datas do clube): a janela vai da véspera
/// (0x54c8e0) até `folga` dias depois (0x54c5b0; folga 2 nos tipos 1 e 10, 3 no tipo 2, 1 nos tipos 0, 3, 4 e 11;
/// outro tipo: a caixa, linha 0xf18, e folga 1). Com tipo ≠ 0, clube que não é seleção e nação-chave ativa, a janela
/// que cai na temporada da nação (início em 0xb63d8a, fim em 0xb63d99, registro de 72 bytes da 0x669340) não serve.
/// Senão, livre se a 0x5acbe0 não acha jogo do clube (+0 id) na janela (com uma lista provisória, destruída no fim).
/// Clube ou data nulos: 0.
unsafe extern "stdcall" fn clube_livre_na_data(data: usize, clube: usize, tipo: u32) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let criar_lista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let (mut a, mut b, mut t0, mut t1) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let mut lista = [0u8; 0x10];
    let (pa, pb, p0, p1, pl) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, t0.as_mut_ptr() as usize,
        t1.as_mut_ptr() as usize, lista.as_mut_ptr() as usize);
    data_padrao(pa);
    data_padrao(pb);
    criar_lista(pl);
    let sai = |r: u32| -> u32 {
        destruir_lista(pl);
        r
    };
    if clube == 0 || data == 0 {
        return sai(0);
    }
    let k = tipo as u8 as i8;
    let folga: u8 = match k {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => {
            aviso_em(0x9cd0a0, 0xf18);
            1
        }
    };
    crate::comparacao::ramo("cpr_13 clube livre");
    let r = menos_dias(data, p0, 1); // a véspera
    copiar(pa, p1, r);
    // ponytail: a original empurra a folga com os 16 bits altos do ponteiro que a 0x417c10 devolveu; a 0x54c5b0 só lê
    // a palavra
    let r = mais_dias(data, p1, folga as u32);
    copiar(pb, p0, r);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nacao = cpr13_d(clube + 0x53) as usize;
    if k != 0 && selecao(clube) == 0 && nacao != 0 && *((nacao + 0x11c) as *const u8) != 0 {
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669340usize);
        let reg = (indice(nacao) as u8 as i8 as isize * 72) as usize;
        let inicio = 0xb63d8ausize.wrapping_add(reg); // a temporada da nação-chave
        let (iano, idia) = (w(inicio, 2), w(inicio, 0) as i16);
        if iano < w(pb, 2) || (iano == w(pb, 2) && idia < w(pb, 0) as i16) {
            let fim = 0xb63d99usize.wrapping_add(reg);
            let (fano, fdia) = (w(fim, 2), w(fim, 0) as i16);
            if fano > w(pa, 2) || (fano == w(pa, 2) && fdia > w(pa, 0) as i16) {
                return sai(0); // a janela cai na temporada
            }
        }
    }
    // os dias e anos vão como os dwords da original (a palavra e os 2 bytes seguintes da data)
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x5acbe0usize);
    let n = jogos(cpr13_d(0xae2a58) as usize, cpr13_d(pa), cpr13_d(pa + 2), cpr13_d(pb), cpr13_d(pb + 2), cpr13_d(clube),
        pl, u32::MAX, 0xffff_fffe, u32::MAX);
    if n as u16 != 0 {
        crate::comparacao::ramo("cpr_13 ocupado");
        return sai(0); // tem jogo na janela
    }
    sai(1)
}

/// { 0x522910 } (thiscall, ret 4, Cup.cpp; virtual +0x80 das copas, chamada pela 0x4ae300 da projeção da diretoria):
/// `Cup::has_club(clube)`: 1 se o clube está na tabela de times da copa (+0xa7, 6 bytes por time, +0 o clube; conta
/// +0x3e); senão 0.
unsafe extern "thiscall" fn copa_tem_o_clube(this: usize, clube: u32) -> u32 {
    let n = *((this + 0x3e) as *const i16);
    let times = cpr13_d(this + 0xa7) as usize;
    for i in 0..n.max(0) as usize {
        if cpr13_d(times + i * 6) == clube {
            crate::comparacao::ramo("cpr_13 copa tem o clube");
            return 1; // está na copa
        }
    }
    0
}

/// { 0x5af5b0 } (cdecl, fix_man.cpp; das buscas de jogos 0x5acbe0, 0x5acee0 e 0x5ad780): `fix_man_get_partner_key_nation_index(clube)`:
/// para clube da Inglaterra ([0x9cf2e4]) ou do País de Gales ([0x9cf510]) (nação +0x53, id +0), o índice da nação-chave
/// ativa da outra (os 35 registros de 72 bytes em 0xb63d60: +0 a nação, ativa em +0x11c, +0x14 o índice); senão 0xff.
unsafe extern "C" fn nacao_parceira_do_clube(clube: usize) -> u8 {
    let nacao = cpr13_d(clube + 0x53) as usize;
    if nacao == 0 {
        return 0xff;
    }
    let (id, inglaterra, gales) = (cpr13_d(nacao), cpr13_d(0x9cf2e4), cpr13_d(0x9cf510));
    let outra = if id == inglaterra {
        gales
    } else if id == gales {
        inglaterra
    } else {
        return 0xff;
    };
    let alvo = cpr13_d(0xae23a8).wrapping_add(outra.wrapping_mul(0x122)) as usize;
    for i in 0..0x23usize {
        let p = cpr13_d(0xb63d60 + i * 72) as usize;
        if p != 0 && *((p + 0x11c) as *const u8) != 0 && p == alvo {
            return *((0xb63d74 + i * 72) as *const u8);
        }
    }
    0xff
}

/// Comparador: a cópia roda com a lista provisória dela, como a original: pela devolução.
unsafe extern "stdcall" fn comparar_clube_livre_na_data(data: usize, clube: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(1113)(data, clube, tipo);
    let n = clube_livre_na_data(data, clube, tipo);
    PLACAR.registrar(r == n, || format!("clube_livre_na_data(data {:x}, clube {:x}, tipo {}) original={} rust={}", data, clube, tipo as u8 as i8, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_copa_tem_o_clube(this: usize, clube: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1114)(this, clube);
    let n = copa_tem_o_clube(this, clube);
    PLACAR.registrar(r == n, || format!("copa_tem_o_clube(copa {:x}, clube {:x}) original={} rust={}", this, clube, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_nacao_parceira_do_clube(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1115)(clube);
    let n = nacao_parceira_do_clube(clube);
    PLACAR.registrar(r == n, || format!("nacao_parceira_do_clube({:x}) original={:x} rust={:x}", clube, r, n));
    r
}
