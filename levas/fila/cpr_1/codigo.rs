// ---------- %%LEVA%%a leva: os limites da escalação: a tabela de restrições da partida, o limite do banco do dia e quem precisa de permissão de trabalho (transfer_manager.cpp) ----------

unsafe fn cpr1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Os 42 bytes da tabela de restrições com 0xff (o `rep stosd` de 10 duplas e o `stosw` da original).
unsafe fn clear_restriction_table(t: *mut u8) {
    core::ptr::write_bytes(t, 0xff, 42);
}

/// { 0x8cf6b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::fill_squad_restrictions(partida, t,
/// so_a_principal)`: enche os 42 bytes de `t` com 0xff. Com partida: o alcance das regras (0x8d2fa0); 0xff para aí. Fora
/// do tipo 4, a primeira das 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo
/// (+0x12) enche a tabela (virtual +0x1c com a partida e `t`). Com `so_a_principal`, fica só o primeiro limite que
/// existe: +5 ou +3 (gravado em +5), +2 ou +0 (gravado em +2), ou +0x16 com +0x17; o resto volta a 0xff.
unsafe extern "thiscall" fn fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    clear_restriction_table(t);
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
        crate::comparison::branch("cpr_1 limit de nao comunitarios");
        clear_restriction_table(t);
        *t.add(5) = v; // o limite de não comunitários
        return;
    }
    let v = if *t.add(2) != 0xff { *t.add(2) } else { *t };
    if v != 0xff {
        clear_restriction_table(t);
        *t.add(2) = v;
        return;
    }
    let (idade, n) = (*t.add(0x16), *t.add(0x17));
    clear_restriction_table(t);
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
unsafe extern "thiscall" fn transfer_manager_check_matchday_squad_limit(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, _nao_usado: u32, saida: usize) -> u32 {
    let limit: unsafe extern "thiscall" fn(usize, usize, *mut u8) = core::mem::transmute(0x8cf7e0usize);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8968f0usize);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut l = [0u8; 6];
    limit(this, partida, l.as_mut_ptr());
    let tipo = l[0];
    if tipo == 0 {
        return 0;
    }
    let humano = ((clube == cpr1_d(partida + 0x1c) && cpr1_b(partida + 0x40) == 0) || (clube == cpr1_d(partida + 0x20) && cpr1_b(partida + 0x41) == 0)) as u32;
    if tipo == 1 {
        crate::comparison::branch("cpr_1 conta nao comunitarios");
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
unsafe extern "thiscall" fn player_needs_work_permit(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    if clube == 0 || cpr1_d(clube + 0x53) == 0 {
        return 0;
    }
    if jogador == 0 {
        show_error_at(0xa8d514, 0x1c23);
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
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpr1_d(0xae1050), jogador);
    if c != 0 && cpr1_b(c + 0x35) & 0x3f == 2 {
        let id = cpr1_d(jogador) as i32;
        if id < cpr1_d(0xae2364) as i32 && cpr1_b(cpr1_d(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) & 0x20 != 0 {
            let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
            if bsearch(&id, cpr1_d(this + 0x93a), cpr1_d(this + 0x93e), 0xe, 0x8e55c0) != 0 {
                crate::comparison::branch("cpr_1 permissao achada");
                return 0; // tem a permissão
            }
        }
    }
    1
}

/// Comparador: lado a lado pelos 42 bytes da tabela (o alcance e a virtual das regras só escrevem nela).
unsafe extern "thiscall" fn comparar_fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(t as usize, 42)], || o(this, partida, t, so_a_principal), || fill_squad_restrictions(this, partida, t, so_a_principal));
    PLACAR.record(igual, || format!("fill_squad_restrictions({:x}, {}) {:x?} x {:x?}", partida, so_a_principal, mo, mr));
}

/// Comparador: lado a lado pela saída (6 bytes) e pelo aviso (0x100 bytes, com `msg`), e pela devolução. A 0x8968f0 pode
/// pôr −1 numa vaga sem pessoa válida: a original já deixou assim e a cópia acha a mesma vaga vazia.
unsafe extern "thiscall" fn comparar_transfer_manager_check_matchday_squad_limit(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, nao_usado: u32, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize) -> u32>(%%I1%%);
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
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side(&regioes[..n], || o(this, clube, partida, msg, jogadores, nao_usado, saida), || transfer_manager_check_matchday_squad_limit(this, clube, partida, msg, jogadores, nao_usado, saida));
    PLACAR.record(igual && r == c, || format!("transfer_manager_check_matchday_squad_limit(clube {:x}, partida {:x}) original={} rust={} {:x?} x {:x?}", clube, partida, r, c, mo, mr));
    r
}

/// Comparador: pelo valor (só lê). Jogador nulo com clube de nação: a caixa, só a original.
unsafe extern "thiscall" fn comparar_player_needs_work_permit(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I2%%);
    if jogador == 0 {
        return o(this, jogador, clube, ignorar_a_dada);
    }
    let r = o(this, jogador, clube, ignorar_a_dada);
    let n = player_needs_work_permit(this, jogador, clube, ignorar_a_dada);
    PLACAR.record(r == n, || format!("player_needs_work_permit(jogador {}, clube {:x}, {}) original={} rust={}", cpr1_d(jogador), clube, ignorar_a_dada, r, n));
    r
}
