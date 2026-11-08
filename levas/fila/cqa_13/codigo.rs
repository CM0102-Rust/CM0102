// ---------- %%LEVA%%a leva: as reações dos elencos à transferência: os contratos dos dois elencos, a comemoração da contratação e o companheiro desassossegado pela venda (media.cpp) ----------

unsafe fn cqa13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa13_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal em `a`.
unsafe fn cqa13_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cqa13_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A pessoa do id (0x6e bytes em [0xae23bc]).
unsafe fn media_staff_by_id(id: i32) -> usize {
    cqa13_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize)
}

/// O contrato da pessoa com o clube (0x4cf5f0 em [0xae1050]), ou 0.
unsafe fn cqa13_contrato(pessoa: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cqa13_d(0xae1050), pessoa, clube)
}

/// A máscara da posição natural da pessoa (0x5481a0).
unsafe fn natural_position_mask(pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    f(pessoa)
}

/// As marcas de evento do contrato (+0x40, 0x8676f0).
unsafe fn media_contract_flags(c: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
    f(c)
}

/// Apaga marcas de evento do contrato (0x8676d0).
unsafe fn clear_contract_flags(c: usize, marcas: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8676d0usize);
    f(c, marcas);
}

/// A reputação atual do jogador (+0xb da ficha +0x61).
unsafe fn current_reputation(pessoa: usize) -> i16 {
    cqa13_w(cqa13_d(pessoa + 0x61) + 0xb)
}

/// O ensaio do comparador da 0x758200: a cópia roda sem nada que aloque, dê notícia ou peça transferência; quando chega
/// a um desses, `MEDIA_REACTION_STOPPED` e ela para (a comparação fica só com a original).
static mut MEDIA_REACTION_DRY_RUN: bool = false;
static mut MEDIA_REACTION_STOPPED: bool = false;

/// Liga marcas no contrato (STAFF_CONTRACT::set_flags 0x867560). No ensaio, pela cópia em ensaio dela (MISS_TRAINING_DRY_RUN): se
/// ela pediria transferência, para. Devolve se parou.
unsafe fn set_contract_flags_or_stop(c: usize, marcas: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN {
        MISS_TRAINING_DRY_RUN = true;
        MISS_TRAINING_DECISION = 0;
        set_flags(c, marcas);
        MISS_TRAINING_DRY_RUN = false;
        if MISS_TRAINING_DECISION != 0 {
            MEDIA_REACTION_STOPPED = true;
        }
        return MEDIA_REACTION_STOPPED;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867560usize);
    f(c, marcas);
    false
}

/// A decisão de pedir transferência da 0x867700 com as `marcas` (sem efeito: o +0x45 volta): funções 0xb, 0xf e 0xd,
/// fora o técnico e o auxiliar do clube do contrato, moral (+0x44) não negativa, sem +0x49, quer sair (0x866f60, com as
/// marcas já ligadas), sem transferência futura (0x8cca10), sem os bits 1 e 8 de +0x4e, fora da fila da transferência
/// livre (0x8c1870) e com mais de 20 anos ou reputação acima de 4250.
unsafe fn would_request_transfer(c: usize, marcas: u32) -> bool {
    let pessoa = media_staff_by_id(cqa13_i(c));
    let funcao = *((pessoa + 0x3d) as *const u8);
    if funcao != 0xb && funcao != 0xf && funcao != 0xd {
        return false;
    }
    let clube = cqa13_d(0xae23b4).wrapping_add((cqa13_i(c + 4) as u32).wrapping_mul(0x245) as usize);
    for o in [0xcf, 0xd3] {
        let t = cqa13_d(clube + o);
        if t != 0 && cqa13_d(c) == cqa13_d(t) {
            return false;
        }
    }
    if cqa13_c(c + 0x44) < 0 || *((c + 0x49) as *const u8) != 0 {
        return false;
    }
    let antes = cqa13_d(c + 0x45);
    core::ptr::write_unaligned((c + 0x45) as *mut usize, antes | marcas as usize);
    let quer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    let sair = quer(c) != 0;
    core::ptr::write_unaligned((c + 0x45) as *mut usize, antes);
    if !sair {
        return false;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if futura(cqa13_d(0xde1f54), pessoa, 0) != 0 || *((c + 0x4e) as *const u8) & 9 != 0 {
        return false;
    }
    let na_fila: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c1870usize);
    if na_fila(cqa13_d(0xde1f54), pessoa) != 0 {
        return false;
    }
    !(cqa13_c(pessoa + 0x18) <= 0x14 && current_reputation(pessoa) <= 0x109a)
}

/// As queixas do contrato (STAFF_CONTRACT::apply_event_flags 0x867700 com a pessoa). No ensaio, para antes se ela
/// pediria transferência. Devolve se parou.
unsafe fn apply_event_flags_or_stop(c: usize, marcas: u32, pessoa: usize) -> bool {
    if MEDIA_REACTION_DRY_RUN && would_request_transfer(c, marcas) {
        MEDIA_REACTION_STOPPED = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x867700usize);
    f(c, marcas, pessoa);
    false
}

/// Tira queixas do contrato (STAFF_CONTRACT::clear_grievances 0x8674e0). No ensaio, para antes se o pedido de
/// transferência seria retirado (a decisão `should_withdraw_transfer_request` com as queixas já tiradas). Devolve se parou.
unsafe fn clear_grievances_or_stop(c: usize, queixas: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN {
        let antes = cqa13_d(c + 0x45);
        core::ptr::write_unaligned((c + 0x45) as *mut usize, antes & !(queixas as usize));
        let retira = should_withdraw_transfer_request(c).is_some();
        core::ptr::write_unaligned((c + 0x45) as *mut usize, antes);
        if retira {
            MEDIA_REACTION_STOPPED = true;
            return true;
        }
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
    f(c, queixas);
    false
}

/// A comemoração da contratação (0x75b6e0); no ensaio, para.
unsafe fn signing_celebration_or_stop(this: usize, p: usize, clube: usize, tipo: u32, porta_voz: usize) -> bool {
    if MEDIA_REACTION_DRY_RUN {
        MEDIA_REACTION_STOPPED = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) = core::mem::transmute(0x75b6e0usize);
    f(this, p, clube, tipo, 1, porta_voz);
    false
}

/// O porta-voz do elenco: entre os 50 do clube com ficha, fora `p`, com reputação acima de 4250, o maior sorteio(reputação
/// + 20 × (+0x5b + +0x59) ± pela idade: abaixo de 19 −1500, de 23 −750, de 28 −250, de 34 +250), acima de 0.
unsafe fn squad_spokesman(clube: usize, p: usize) -> usize {
    let (mut melhor, mut quem) = (0i32, 0usize);
    for j in 0..50usize {
        let m = cqa13_d(clube + 0xd7 + 4 * j);
        if m == 0 || cqa13_d(m + 0x61) == 0 || m == p {
            continue;
        }
        let r = current_reputation(m);
        if r <= 0x109a {
            continue;
        }
        let mut x = (r as i32).wrapping_add((cqa13_c(m + 0x5b) + cqa13_c(m + 0x59)) * 20);
        let idade = cqa13_c(m + 0x18);
        x += if idade < 0x13 {
            -0x5dc
        } else if idade < 0x17 {
            -0x2ee
        } else if idade < 0x1c {
            -0xfa
        } else if idade < 0x22 {
            0xfa
        } else {
            0
        };
        let s = cqa13_sortear(x);
        if s > melhor {
            melhor = s;
            quem = m;
        }
    }
    quem
}

/// { 0x758200 } (thiscall, ret 0x10, media.cpp): `Media::process_transfer_squad_reactions(jogador, novo, velho, flag)`,
/// no fim de cada transferência (TRANSFER_MANAGER::move_player_to_club, 0x8ca879). Só para o jogador (ficha +0x61) de 19
/// anos ou mais com reputação de 4250. (A) A história da mídia na frente da fila (0xbd0, +0x11 = 0x25, novo clube de id
/// 0x25; nunca na prática): queixas no contrato da pessoa dela. (B) No elenco do novo clube (50 vagas +0xd7), quem está
/// lá há mais de +0x5a + 15 dias (0x54cdb0 de +0x3e) e tem contrato: a marca 0x800 com a mesma posição troca por queixa
/// (0x867700 0x80000000) e sorteios; a 0x400 sai com o elenco forte (0x53c590 até o limite pela reputação); com mais de 2
/// da posição, o jogador forte pode virar queixa ou a marca 0x200; a queixa 0x20000000 sai (0x8674e0) quando chega
/// alguém maior. Promessa de craque (+0xd do jogador acima de 6250): a torcida (+8 da linha do clube em [[0xb67a44]],
/// +500 até 10000) e a comemoração (0x75b6e0); se algo mudou, sorteio(3) e o porta-voz comemoram. (C) No elenco do velho
/// clube (com liga), com a força e a reputação do clube (ou a nação entre as sete de 0x9cf31c..): as queixas 0x40000000 /
/// 0x20000000 pela saída do companheiro, a moral −25, a marca 0x400, as 0x200/0x800 e a queixa 0x80000000 tiradas; se
/// algo mudou e o jogador era craque, o companheiro desassossegado (0x75cfc0).
unsafe extern "thiscall" fn process_transfer_squad_reactions(this: usize, p: usize, novo: usize, velho: usize, flag: u32) {
    if p == 0 || novo == 0 {
        return;
    }
    let ficha = cqa13_d(p + 0x61);
    if ficha == 0 || cqa13_c(p + 0x18) < 0x13 || cqa13_w(ficha + 0xb) < 0x109a {
        return;
    }
    let mut mudou = false;
    // (A) a história da mídia na frente da fila
    if cqa13_i(this + 8) > 0 {
        let h = cqa13_d(this);
        if cqa13_i(h + 0x13) == 0xbd0 && cqa13_w(h + 0x11) == 0x25 && cqa13_i(novo) == 0x25 {
            let tipo = *((h + 0x10) as *const u8);
            let id = cqa13_i(h + 8);
            if id != -1 {
                let q = media_staff_by_id(id);
                let rp = current_reputation(p);
                if natural_position_mask(p) == natural_position_mask(q) && (rp > 0x128e || rp as i32 - 0x5dc > current_reputation(q) as i32) {
                    let c = cqa13_contrato(q, novo);
                    if c != 0 {
                        crate::comparison::branch("cqa_13 media story");
                        mudou = true;
                        if tipo == 1 {
                            if apply_event_flags_or_stop(c, 0x8000000, 0) || apply_event_flags_or_stop(c, 0x80000000, p) {
                                return;
                            }
                        } else if tipo == 2 {
                            if media_contract_flags(c) & 0x800 != 0
                                && (set_contract_flags_or_stop(c, 0x200) || apply_event_flags_or_stop(c, 0x80000000, p) || set_contract_flags_or_stop(c, 0x80))
                            {
                                return;
                            }
                        } else if cqa13_sortear(cqa13_c(q + 0x5b)) < cqa13_sortear(10) {
                            if set_contract_flags_or_stop(c, 0x80) {
                                return;
                            }
                            if cqa13_sortear(cqa13_c(q + 0x5a)) == 0 && apply_event_flags_or_stop(c, 0x80000000, p) {
                                return;
                            }
                        }
                        if media_contract_flags(c) & 0x800 != 0 {
                            clear_contract_flags(c, 0x800);
                        }
                    }
                }
            }
        }
    }
    // (B) o elenco do novo clube
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let mesma_posicao: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x548150usize);
    let forca = contar(novo, 0, 0) as i8 as i32;
    let rn = cqa13_w(novo + 0x80);
    let limit = if rn < 0x9c4 { 0x14 } else if rn < 0x1482 { 0x19 } else if rn >= 0x1c52 { 0x20 } else { 0x1e }; // cqa13: o limite do elenco forte
    let mesma = mesma_posicao(novo, natural_position_mask(p)) as i8 as i32;
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let categoria: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    for j in 0..50usize {
        let m = cqa13_d(novo + 0xd7 + 4 * j);
        if m == 0 || cqa13_d(m + 0x61) == 0 || m == p {
            continue;
        }
        if dias(0xae2c90, cqa13_d(m + 0x3e) as u32, cqa13_d(m + 0x42) as u32) <= cqa13_c(m + 0x5a) + 0xf {
            continue;
        }
        let c = cqa13_contrato(m, novo);
        if c == 0 {
            continue;
        }
        if media_contract_flags(c) & 0x800 != 0 && natural_position_mask(p) == natural_position_mask(m) {
            clear_contract_flags(c, 0x800);
            if apply_event_flags_or_stop(c, 0x80000000, p) {
                return;
            }
            if cqa13_sortear(cqa13_c(m + 0x5a)) == 0 && set_contract_flags_or_stop(c, 0x200) {
                return;
            }
            if cqa13_sortear(cqa13_c(m + 0x5b)) == 0 && set_contract_flags_or_stop(c, 0x80) {
                return;
            }
        }
        if media_contract_flags(c) & 0x400 != 0 && forca >= limit {
            crate::comparison::branch("cqa_13 strong squad");
            clear_contract_flags(c, 0x400);
        }
        if mesma > 2 && media_contract_flags(c) & 0x400 == 0 && current_reputation(m) > 0x109a && natural_position_mask(p) == natural_position_mask(m) {
            let rp = current_reputation(p);
            if rp > 0x128e || rp as i32 - 0x5dc > cqa13_w(novo + 0x80) as i32 {
                let a = cqa13_c(m + 0x5a);
                if a < 0xf && cqa13_c(m + 0x5b) < 0xf && a < mesma + 4 && cqa13_c(m + 0x57) < mesma + 8 && cqa13_sortear(a) < cqa13_sortear(6) {
                    let parou = if categoria(m, c) == 1 || categoria(m, c) == 2 {
                        apply_event_flags_or_stop(c, 0x80000000, p)
                    } else {
                        set_contract_flags_or_stop(c, 0x200)
                    };
                    if parou {
                        return;
                    }
                }
            }
        }
        if cqa13_i(c + 0x45) & 0x20000000 != 0 {
            let rp = current_reputation(p);
            if rp > 0x1482 || rp as i32 - 0x4e2 > cqa13_w(novo + 0x80) as i32 {
                mudou = true;
                if forca > 0x28 || cqa13_sortear(cqa13_c(m + 0x5a)) != 0 {
                    crate::comparison::branch("cqa_13 complaint cleared");
                    if clear_grievances_or_stop(c, 0x20000000) { // cqa13: chega alguém maior
                        return;
                    }
                }
            }
        }
    }
    let ficha = cqa13_d(p + 0x61);
    if cqa13_w(ficha + 0xd) > 0x186a {
        let rp = cqa13_w(ficha + 0xb);
        if rp as i32 + 0x2ee > cqa13_w(novo + 0x80) as i32 || rp > 0x222e {
            let linha = cqa13_d(cqa13_d(0xb67a44)).wrapping_add((cqa13_i(novo) as u32).wrapping_mul(0x49) as usize);
            let x = cqa13_w(linha + 8).wrapping_add(0x1f4); // cqa13: a torcida comemora
            let x = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            crate::comparison::branch("cqa_13 goal_crowd_reaction celebrates");
            core::ptr::write_unaligned((linha + 8) as *mut i16, x);
            if signing_celebration_or_stop(this, p, novo, 1, 0) {
                return;
            }
        }
    }
    if mudou {
        let ficha = cqa13_d(p + 0x61);
        if cqa13_w(ficha + 0xd) > 0x186a
            && cqa13_w(ficha + 0xb) as i32 + 0x2ee > cqa13_w(novo + 0x80) as i32
            && cqa13_c(p + 0x5b) > 0xa
            && cqa13_c(p + 0x59) > 6
            && cqa13_sortear(3) != 0
        {
            let quem = squad_spokesman(novo, p);
            if quem != 0 && signing_celebration_or_stop(this, p, novo, 2, quem) {
                return;
            }
        }
    }
    // (C) o elenco do velho clube
    let mut mudou = false;
    if velho == 0 || cqa13_d(velho + 0x57) == 0 {
        return;
    }
    if cqa13_d(cqa13_d(0xadadfc).wrapping_add(4 * cqa13_d(cqa13_d(velho + 0x57)))) == 0 {
        return;
    }
    let forca = contar(velho, 0, 0) as i8 as i32;
    let nacao = cqa13_d(velho + 0x53);
    let sete = nacao != 0 && [0x9cf31c, 0x9cf4a8, 0x9cf510, 0x9cf2fc, 0x9cf2dc, 0x9cf3e4, 0x9cf224].iter().any(|&k| cqa13_d(nacao) == cqa13_d(k));
    let rv = cqa13_w(velho + 0x80);
    let (lim1, lim2) = if sete || rv < 0x1482 {
        (0x19, 0x2d)
    } else if rv < 0x1c52 {
        (0x1e, 0x2d)
    } else {
        (0x23, 0x2e)
    };
    let mesma = mesma_posicao(velho, natural_position_mask(p)) as i8 as i32;
    let candidato: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x548810usize);
    for j in 0..50usize {
        let m = cqa13_d(velho + 0xd7 + 4 * j);
        if m == 0 || cqa13_d(m + 0x61) == 0 || m == p {
            continue;
        }
        let c = cqa13_contrato(m, velho);
        if c == 0 {
            continue;
        }
        let mut q: i32 = if cqa13_i(c + 0x45) != 0 { 3 } else { 0 };
        if media_contract_flags(c) != 0 {
            q += 1;
        }
        if cqa13_c(c + 0x44) < -0x19 {
            q += 1;
        }
        let q = q as i8 as i32;
        let mut queixa = 0u32; // 0x40000000 ou 0x20000000 pela saída do companheiro
        let mut segundo = true;
        if flag as u8 != 0
            && media_contract_flags(c) & 0x200 == 0
            && media_contract_flags(c) & 0x800 == 0
            && cqa13_i(c + 0x45) as u32 & 0x80000000 == 0
            && (forca < 0x28 || mesma < 6)
            && (natural_position_mask(p) != natural_position_mask(m) || mesma < 2)
        {
            let rp = current_reputation(p);
            if rp > 0x1e46 || rp > cqa13_w(novo + 0x80).wrapping_sub(0x2ee) {
                let r = cqa13_sortear(6);
                if cqa13_c(m + 0x57) + q > r + 10 && cqa13_c(c + 0x3d) > 0x19 && current_reputation(m) > 0x109a
                    && cqa13_sortear(cqa13_c(m + 0x5b)) < q + 5 && cqa13_sortear(cqa13_c(m + 0x5a)) == 0
                {
                    segundo = false;
                    queixa = if cqa13_c(p + 0x18) < 0x18 && cqa13_w(cqa13_d(p + 0x61) + 0xd) < 0x128e { 0x40000000 } else { 0x20000000 };
                }
            }
        }
        if segundo
            && cqa13_c(p + 0x18) > 0x17
            && flag as u8 != 0
            && candidato(m, novo, p) != 0
            && natural_position_mask(p) != natural_position_mask(m)
            && cqa13_sortear(cqa13_c(m + 0x5b)) < q + 5
        {
            let rp = current_reputation(p);
            if rp > 0x1e46 || rp > cqa13_w(novo + 0x80).wrapping_sub(0x2ee) {
                queixa = 0x20000000;
            } else {
                crate::comparison::branch("cqa_13 morale drops");
                let x = cqa13_c(c + 0x44) - 0x19; // cqa13: a moral do companheiro cai
                *((c + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { x as i8 };
            }
        }
        if queixa != 0 {
            if apply_event_flags_or_stop(c, queixa, p) {
                return;
            }
            mudou = true;
        }
        if cqa13_c(m + 0x18) > 0x17 && media_contract_flags(c) & 0x400 == 0 && cqa13_c(m + 0x57) > 0xf {
            let r = (cqa13_sortear(10) as u8).wrapping_add(10) as i8;
            if *((m + 0x58) as *const i8) > r && cqa13_c(m + 0x5b) > 0xa && current_reputation(p) > 0x109a && current_reputation(m) > 0x109a && forca < lim1 && set_contract_flags_or_stop(c, 0x400) {
                return;
            }
        }
        let iguais = || natural_position_mask(p) == natural_position_mask(m);
        if (forca < lim2 || mesma < 2) && iguais() && media_contract_flags(c) & 0x200 != 0 {
            clear_contract_flags(c, 0x200);
        }
        let tirar_800 = if forca < lim2 {
            let r1 = cqa13_sortear(cqa13_c(p + 0x5a));
            (r1 > cqa13_sortear(10) && iguais()) || mesma < 2
        } else {
            mesma < 2
        };
        if tirar_800 && media_contract_flags(c) & 0x800 != 0 {
            clear_contract_flags(c, 0x800);
        }
        if ((forca < lim2 && iguais()) || mesma < 2) && cqa13_i(c + 0x45) as u32 & 0x80000000 != 0 && clear_grievances_or_stop(c, 0x80000000) {
            return;
        }
        if media_contract_flags(c) & 0x400 != 0 {
            if cqa13_i(c + 0x45) as u32 & 0x80000000 != 0 && clear_grievances_or_stop(c, 0x80000000) {
                return;
            }
            if media_contract_flags(c) & 0x800 != 0 {
                clear_contract_flags(c, 0x800);
            }
            if media_contract_flags(c) & 0x200 != 0 {
                clear_contract_flags(c, 0x200);
            }
        }
    }
    if !mudou || cqa13_c(p + 0x18) >= 0x23 {
        return;
    }
    let ficha = cqa13_d(p + 0x61);
    if cqa13_w(ficha + 5) > 0x64
        && cqa13_w(ficha + 0xd) > 0x186a
        && cqa13_w(ficha + 0xb) as i32 + 0x2ee > cqa13_w(velho + 0x80) as i32
        && cqa13_c(p + 0x5b) > 0xa
        && cqa13_c(p + 0x59) > 6
        && cqa13_sortear(3) != 0
    {
        let quem = squad_spokesman(velho, p);
        if quem != 0 {
            if MEDIA_REACTION_DRY_RUN {
                MEDIA_REACTION_STOPPED = true;
                return;
            }
            let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) = core::mem::transmute(0x75cfc0usize);
            f(this, velho, novo, p, quem);
        }
    }
}

/// Comparador em ensaio: a cópia sem notícia nem pedido de transferência (para quando chega a um) e a original a partir
/// do mesmo estado: o sorteio, [0xae1080] (a ficha do último contrato), a data estática da 0x54cdb0, a linha da torcida
/// do novo clube em [[0xb67a44]] e, de cada um dos elencos dos dois clubes (e da pessoa da história da mídia), o contrato
/// com o clube (0x50 bytes) e a linha de opiniões em [0xae2d0c] (0x4f bytes). Quando a cópia para: só a original.
unsafe extern "thiscall" fn compare_process_transfer_squad_reactions(this: usize, p: usize, novo: usize, velho: usize, flag: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(%%I0%%);
    if p == 0 || novo == 0 || cqa13_d(p + 0x61) == 0 || cqa13_c(p + 0x18) < 0x13 || current_reputation(p) < 0x109a {
        return o(this, p, novo, velho, flag);
    }
    let ficha = cqa13_d(0xae1080);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((0xae1080, 4));
    reg.push((0xae2470, 8));
    if cqa13_d(0xb67a44) != 0 {
        reg.push((cqa13_d(cqa13_d(0xb67a44)).wrapping_add((cqa13_i(novo) as u32).wrapping_mul(0x49) as usize), 0x49));
    }
    let mut juntar = |m: usize, clube: usize| {
        if m == 0 {
            return;
        }
        let c = cqa13_contrato(m, clube);
        if c != 0 {
            reg.push((c, 0x50));
        }
        reg.push((cqa13_d(0xae2d0c).wrapping_add((cqa13_i(m) as u32).wrapping_mul(0x4f) as usize), 0x4f));
    };
    for clube in [novo, velho] {
        if clube != 0 {
            for j in 0..50usize {
                juntar(cqa13_d(clube + 0xd7 + 4 * j), clube);
            }
        }
    }
    if cqa13_i(this + 8) > 0 && cqa13_i(cqa13_d(this) + 8) != -1 {
        juntar(media_staff_by_id(cqa13_i(cqa13_d(this) + 8)), novo);
    }
    core::ptr::write_unaligned(0xae1080 as *mut usize, ficha); // os contratos lidos acima não contam
    let antes = crate::comparison::MemorySnapshot::read(&reg);
    MEDIA_REACTION_DRY_RUN = true;
    MEDIA_REACTION_STOPPED = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| process_transfer_squad_reactions(this, p, novo, velho, flag)));
    MEDIA_REACTION_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    antes.write_back();
    if MEDIA_REACTION_STOPPED {
        crate::comparison::branch("cqa_13 reactions only in the original");
        return o(this, p, novo, velho, flag);
    }
    o(this, p, novo, velho, flag);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("process_transfer_squad_reactions(jogador {:#x}, novo {:#x}, velho {:#x}) diferencas em {:x?}", p, novo, velho, dif)
    });
}

/// { 0x75b6e0 } (thiscall, ret 0x14, media.cpp, quadro SEH da notícia local): `Media::queue_signing_celebration_story(
/// jogador, clube, tipo, a, porta_voz)`, pelas reações à transferência (0x758200). Jogador com +0x1a, clube atual
/// (+0x39) com técnico (+0xcf) e +0x69, ficha (+0x61); tipo 2 sem porta-voz: nada. A notícia 0xbe5: o jogador (id e as
/// três chaves), o id do clube dele, sorteio(2), o veículo (0x757fa0), o comentarista (0x7dab90 com o clube, o veículo e
/// a nação +0x53 ou −1), as posições (0x5b7d20), o tipo, a resposta do técnico da máquina (0x735870 com 0x82; 0 com
/// técnico humano), no tipo 2 o porta-voz (id e chaves), a reputação do clube acima de 7250, o técnico (id e chaves) e
/// `a`; vai ao técnico do clube do jogador (0x7861e0).
unsafe extern "thiscall" fn queue_signing_celebration_story(this: usize, p: usize, clube: usize, tipo: u32, a: u32, porta_voz: usize) {
    if p == 0 || cqa13_d(p + 0x1a) == 0 {
        return;
    }
    let atual = cqa13_d(p + 0x39);
    if atual == 0 || cqa13_d(atual + 0xcf) == 0 || cqa13_d(atual + 0x69) == 0 || cqa13_d(p + 0x61) == 0 {
        return;
    }
    if porta_voz == 0 && tipo as u8 == 2 {
        return;
    }
    crate::comparison::branch("cqa_13 celebration news");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x757fa0usize);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(0x7dab90usize);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let resposta: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x735870usize);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbe5, 0);
    gravar(nt, 0, cqa13_d(p) as u32);
    for k in 1..4usize {
        gravar(nt, k as i8, cqa13_d(cqa13_d(p + 4 * k) + 0x33) as u32);
    }
    gravar(nt, 4, cqa13_d(cqa13_d(p + 0x39)) as u32);
    gravar(nt, 5, cqa13_sortear(2) as u32);
    let v = veiculo(p);
    gravar(nt, 6, v);
    let nacao = cqa13_d(clube + 0x53);
    gravar(nt, 7, comentarista(cqa13_d(0xb74430), clube, v, if nacao != 0 { cqa13_i(nacao) } else { -1 }) as u32);
    gravar(nt, 8, posicoes(cqa13_d(p + 0x61)) as i16 as i32 as u32);
    gravar(nt, 9, tipo as u8 as i8 as i32 as u32);
    let mut r = 0u32;
    if humano(cqa13_d(p + 0x39), 0, 1, 0, 0) == 0 {
        r = resposta(this, cqa13_d(cqa13_d(p + 0x39) + 0xcf), p, 0x82);
    }
    gravar(nt, 10, r);
    let mut k: i8 = 0xb;
    if tipo as u8 == 2 {
        gravar(nt, 0xb, cqa13_d(porta_voz) as u32);
        for j in 1..4usize {
            gravar(nt, 0xb + j as i8, cqa13_d(cqa13_d(porta_voz + 4 * j) + 0x33) as u32);
        }
        k = 0xf;
    }
    gravar(nt, k, (cqa13_w(clube + 0x80) > 0x1c52) as u32);
    let t = cqa13_d(cqa13_d(p + 0x39) + 0xcf);
    gravar(nt, k + 1, cqa13_d(t) as u32);
    for j in 1..4usize {
        gravar(nt, k + 1 + j as i8, cqa13_d(cqa13_d(t + 4 * j) + 0x33) as u32);
    }
    // ponytail: a original empurra o índice num dword de bytes de cima sem valor; a 0x785790 lê o char
    gravar(nt, k + 5, a as u8 as i8 as i32 as u32);
    entregar(cqa13_d(0xb7431c), nt, cqa13_d(p + 0x39));
}

// ponytail: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_queue_signing_celebration_story(this: usize, p: usize, clube: usize, tipo: u32, a: u32, porta_voz: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize)>(%%I1%%)(this, p, clube, tipo, a, porta_voz)
}

/// { 0x75cfc0 } (thiscall, ret 0x10, media.cpp, quadro SEH da notícia local): `Media::queue_sale_unrest_story(velho,
/// novo, jogador, companheiro)`, pelas reações à transferência (0x758200). Velho clube com técnico (+0xcf), fora da
/// seleção (0x53b900), companheiro e jogador com ficha (+0x61). A notícia 0xbe4: o técnico, o companheiro e o jogador
/// (id e as três chaves de cada), os ids dos dois clubes, sorteio(2) (+2 se +0x5b do companheiro não passar de
/// sorteio(5) + 10), o veículo (0x757f30), o comentarista (0x7dab90 com a nação +0x53 ou −1) e 2; vai ao técnico do velho
/// clube (0x7861e0).
unsafe extern "thiscall" fn queue_sale_unrest_story(_this: usize, velho: usize, novo: usize, p: usize, comp: usize) {
    if velho == 0 || cqa13_d(velho + 0xcf) == 0 || is_national_team(velho) != 0 {
        return;
    }
    if comp == 0 || cqa13_d(comp + 0x61) == 0 || p == 0 || cqa13_d(p + 0x61) == 0 {
        return;
    }
    crate::comparison::branch("cqa_13 unsettling news");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x757f30usize);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(0x7dab90usize);
    let mut local = [0u32; 0x3a];
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbe4, 0);
    let mut k: i8 = 0;
    for quem in [cqa13_d(velho + 0xcf), comp, p] {
        gravar(nt, k, cqa13_d(quem) as u32);
        for j in 1..4usize {
            gravar(nt, k + j as i8, cqa13_d(cqa13_d(quem + 4 * j) + 0x33) as u32);
        }
        k += 4;
    }
    gravar(nt, 0xc, cqa13_d(velho) as u32);
    gravar(nt, 0xd, cqa13_d(novo) as u32);
    let r = cqa13_sortear(5);
    let s = if cqa13_c(comp + 0x5b) > r + 10 { cqa13_sortear(2) } else { cqa13_sortear(2) + 2 };
    gravar(nt, 0xe, s as u32);
    let v = veiculo(velho);
    gravar(nt, 0xf, v);
    let nacao = cqa13_d(velho + 0x53);
    gravar(nt, 0x10, comentarista(cqa13_d(0xb74430), velho, v, if nacao != 0 { cqa13_i(nacao) } else { -1 }) as u32);
    gravar(nt, 0x11, 2);
    entregar(cqa13_d(0xb7431c), nt, velho);
}

// ponytail: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_queue_sale_unrest_story(this: usize, velho: usize, novo: usize, p: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(%%I2%%)(this, velho, novo, p, comp)
}
