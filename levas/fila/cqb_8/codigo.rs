// ---------- %%LEVA%%a leva: a reação do elenco à história de um jogador, o aviso aos rivais de posição e o rótulo das respostas à mídia (media.cpp) ----------

unsafe fn cqb8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqb8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O vínculo da `pessoa` com o `clube` (0x4cf5f0: thiscall em [0xae1050], 2 na pilha; grava [0xae1080]).
unsafe fn media_contract_for_club(pessoa: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cqb8_d(0xae1050), pessoa, clube)
}

/// O contrato inteiro (+0 a +0x4f) da `pessoa` no `clube`, como região do comparador.
unsafe fn contract_region(pessoa: usize, clube: usize) -> Option<(usize, usize)> {
    let c = media_contract_for_club(pessoa, clube);
    (c != 0).then_some((c, 0x50))
}

/// Uma rotina sem porta que por dentro chama unidades da fita (0x544520 e 0x544600 chamam a 0x4cf5f0; a 0x62c400
/// alcança a 0x600a50 e portas): na comparação pela fita, a original grava essas chamadas internas no nível de cima
/// (com o comparador da rotina, a original e a cópia dele gravam cada uma a sua). A cópia roda a rotina numa fita de
/// rascunho e pede as mesmas chamadas por `chamar`, na ordem. Fora da comparação (sem GRAVAR_POS) só chama.
unsafe fn replay_inner_tape_calls<R>(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, f: impl FnOnce() -> R) -> R {
    if !GRAVAR_POS.load(std::sync::atomic::Ordering::Relaxed) {
        return f();
    }
    let externa = tape_lock().replace((Vec::new(), Vec::new()));
    let r = f();
    let gravadas = core::mem::replace(&mut *tape_lock(), externa).map_or(Vec::new(), |g| g.1);
    for c in gravadas {
        chamar(c.0, c.1, c.2);
    }
    r
}

/// O vetor que a 0x755340 monta na pilha (58 dwords, ela percorre 57): os 50 do elenco (0x541f90 com 0, cdecl), o
/// +0xbf do clube, o técnico (+0xcf) só sem humano no clube (0x600a50 com 0, 1, 0, 0, cdecl, por `chamar`: unidade da
/// fita) e fora dos tipos 11 a 15 (0x53b620, cdecl, o byte +0x3d), o +0xd3 e os cinco dwords de +0x19f.
unsafe fn club_staff_vector(clube: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> [usize; 58] {
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let tipo: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let mut v = [0usize; 58];
    reunir(clube, v.as_mut_ptr(), 0);
    v[50] = cqb8_d(clube + 0xbf);
    let tecnico = cqb8_d(clube + 0xcf);
    v[51] = if chamar(0x600a50, 0, [clube as u32, 0, 1, 0, 0]) != 0 || (tecnico != 0 && tipo(*((tecnico + 0x3d) as *const u8) as u32) != 0) {
        0
    } else {
        tecnico
    };
    v[52] = cqb8_d(clube + 0xd3);
    for k in 0..5 {
        v[53 + k] = cqb8_d(clube + 0x19f + 4 * k);
    }
    v
}

/// { 0x755340 } (thiscall, ret 0x14, media.cpp): `Media::apply_squad_reaction_to_player_story(tópico, jogador, clube,
/// ids, n)`, depois de uma história sobre o jogador (o boato de transferência 0x754a30 com o tópico 0x1f e os clubes
/// interessados; 0x72c1f0 e 0x733a60 sem ids). A chance começa em 10 (5 se o jogador é favorito do clube, 0x547790; 15
/// se rejeitado, 0x5477d0) e cresce por clube interessado: 100 no clássico de Glasgow, 10 se a liga tem o confronto
/// (0x4b02e0), 15 se o clube é rival dele (+0xb3/+0xb7/+0xbb), 5 se é da mesma divisão (fora da divisão inferior).
/// Sorteio(chance) acima de sorteio(10): o clube rejeita o jogador (0x547940 com 1). Listado (0x544520): −25; sem
/// vínculo: −15; senão −10 no fim de contrato (0x867d30) e mais a felicidade (+0x3d) / 10; no mínimo 5. Para cada uma
/// das 57 pessoas do vetor (club_staff_vector) com preferências (+0x65): a nota (10, o +0x1d da ficha de técnico + 10, ou pelo
/// +0x5d, idade, +0x5b e lista a pedido do jogador do elenco), ajustada pelo tópico 0x20/0x1e, pelo +0x59/+0x5a/+0x5b,
/// por gostar ou não do jogador (0x547810/0x547840) e pelo jogador ser favorito ou rejeitado do clube, no mínimo 5;
/// sorteio(chance) acima de sorteio(nota): a pessoa passa a não gostar do jogador (0x547af0 com 1) e, com vínculo e
/// sorteio(nota) abaixo de sorteio(10), a marca 1 do contrato (0x867560) ou, nos tópicos 0x1e/0x20 ou com sorteio(nota)
/// zero, as queixas 0x800 (0x867700). As três com efeito vão por `chamar` (as portas do pós-jogo), e também as unidades
/// da fita que a original chama: o vínculo 0x4cf5f0 (0x75551e, 0x7557dc), o humano 0x600a50 (0x755598) e as 0x4cf5f0
/// de dentro da 0x544520 (0x7554e8) e da 0x544600 (0x75567f, 0x7556ad), estas por `replay_inner_tape_calls`.
unsafe fn squad_reaction_to_story_with(topico: u32, jogador: usize, clube: usize, ids: usize, n: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547940usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    let a_pedido: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544600usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    if clube == 0 || jogador == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_8 reacao do elenco");
    let mut chance: i32 = 0xa; // cqb8: a chance inicial
    if favorito(jogador, clube) != 0 {
        chance = 5;
    } else if rejeitado(jogador, clube) != 0 {
        chance = 0xf;
    }
    if ids != 0 {
        let n = n as i8 as i32;
        let mut k = 0i32;
        while k < n {
            let id = cqb8_d(ids + 4 * k as usize);
            let outro = cqb8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245));
            k += 1;
            let (celtic, rangers) = (cqb8_d(0x9d0118), cqb8_d(0x9d011c));
            let meu = cqb8_d(clube);
            if (meu == celtic || meu == rangers) && (cqb8_d(outro) == celtic || cqb8_d(outro) == rangers) {
                chance += 100;
                continue;
            }
            if confronto(clube, outro) != 0 {
                chance += 10;
                continue;
            }
            if clube == cqb8_d(outro + 0xb3) || clube == cqb8_d(outro + 0xb7) || clube == cqb8_d(outro + 0xbb) {
                chance += 0xf;
                continue;
            }
            let div = cqb8_d(clube + 0x57);
            if div != 0 && cqb8_d(div) != cqb8_d(0x9cf764) && div == cqb8_d(outro + 0x57) {
                chance += 5;
            }
        }
    }
    let r1 = cqb8_sortear(chance);
    if r1 > cqb8_sortear(10) {
        rejeitar(clube, jogador, 1);
    }
    // cqb8: a 0x4cf5f0 (de dentro da 0x544520 e direta) é unidade da fita: vai por `chamar`
    if replay_inner_tape_calls(chamar, || listada(jogador, 0)) != 0 {
        chance -= 0x19;
    } else {
        let c = chamar(0x4cf5f0, cqb8_d(0xae1050), [jogador as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 {
            chance -= 0xf;
        } else {
            if band(c) != 0 {
                chance -= 10;
            }
            chance += (cqb8_b(c + 0x3d) as i32 / 10) as i8 as i32;
        }
    }
    if chance < 5 {
        chance = 5;
    }
    let v = club_staff_vector(clube, chamar);
    let topico = topico as u16;
    for &p in v.iter().take(57) {
        if p == 0 || p == jogador || cqb8_d(p + 0x65) == 0 {
            continue;
        }
        let mut nota: i8 = 10;
        let tecnico = cqb8_d(p + 0x69);
        if tecnico != 0 {
            nota = cqb8_b(tecnico + 0x1d).wrapping_add(10);
        } else if cqb8_d(p + 0x61) != 0 {
            if cqb8_b(jogador + 0x18) > 0x14 && cqb8_b(p + 0x18) < 0x17 && cqb8_w(cqb8_d(p + 0x61) + 0xb) < 0x109a {
                continue;
            }
            crate::comparacao::ramo("cqb_8 nota do jogador");
            nota = cqb8_b(p + 0x5d).wrapping_add(10); // cqb8: a nota do jogador do elenco
            if cqb8_b(p + 0x18) < 0x14 {
                nota = nota.wrapping_sub(5);
            }
            let m = cqb8_b(p + 0x5b);
            if m == 0x14 {
                nota = nota.wrapping_add(1);
            } else if m == 1 {
                nota = nota.wrapping_sub(if replay_inner_tape_calls(chamar, || a_pedido(p, 0)) != 0 { 0xf } else { 10 });
                if cqb8_w(cqb8_d(p + 0x61) + 0xb) > 0x1676 {
                    nota = nota.wrapping_sub(10);
                }
            } else if m < 6 {
                nota = nota.wrapping_sub(if replay_inner_tape_calls(chamar, || a_pedido(p, 0)) != 0 { 10 } else { 5 });
                if cqb8_w(cqb8_d(p + 0x61) + 0xb) > 0x1676 {
                    nota = nota.wrapping_sub(5);
                }
            }
        }
        if topico == 0x20 && cqb8_b(p + 0x59) > 0xf && cqb8_b(p + 0x5d) < 6 {
            nota = nota.wrapping_sub(5);
        }
        let a59 = cqb8_b(p + 0x59);
        if a59 > 0xf {
            if cqb8_b(p + 0x5b) != 0x14 {
                nota = nota.wrapping_sub(2);
            }
        } else if a59 < 5 {
            nota = nota.wrapping_add(2);
        }
        if topico == 0x1e {
            let (a5b, a5a) = (cqb8_b(p + 0x5b), cqb8_b(p + 0x5a));
            if a5b > 0xf {
                if a5a < 0xf {
                    nota = nota.wrapping_sub(1);
                }
            } else if a5b >= 6 {
                nota = nota.wrapping_sub(if a5a >= 0xf { 1 } else { 3 });
            } else {
                nota = nota.wrapping_sub(if a5a < 6 { 10 } else { 5 });
            }
        }
        if gosta(jogador, p) != 0 {
            nota = nota.wrapping_add(0x19);
        } else if nao_gosta(jogador, p) != 0 {
            nota = nota.wrapping_add(cqb8_b(jogador + 0x5b).wrapping_sub(0x19));
            if nota > 0xf {
                nota = 0xf;
            }
        }
        if favorito(jogador, clube) != 0 {
            nota = nota.wrapping_add(5);
        } else if rejeitado(jogador, clube) != 0 {
            nota = nota.wrapping_sub(5);
        }
        if nota < 5 {
            nota = 5;
        }
        let r1 = cqb8_sortear(chance);
        if r1 > cqb8_sortear(nota as i32) { // cqb8: o desafeto sorteado
            crate::comparacao::ramo("cqb_8 desafeto sorteado");
            chamar(0x547af0, 0, [p as u32, jogador as u32, 1, 0, 0]);
            let c = chamar(0x4cf5f0, cqb8_d(0xae1050), [p as u32, clube as u32, 0, 0, 0]) as usize;
            if c != 0 {
                let r = cqb8_sortear(nota as i32);
                if r < cqb8_sortear(10) {
                    if topico != 0x1e && topico != 0x20 && cqb8_sortear(nota as i32) != 0 {
                        chamar(0x867560, c, [1, 0, 0, 0, 0]);
                    } else {
                        chamar(0x867700, c, [0x800, 0, 0, 0, 0]);
                    }
                }
            }
        }
    }
}

unsafe extern "thiscall" fn apply_squad_reaction_to_player_story(_this: usize, topico: u32, jogador: usize, clube: usize, ids: usize, n: u32) {
    squad_reaction_to_story_with(topico, jogador, clube, ids, n, &mut |r, t, a| dispatch_post_match_call(r, t, a))
}

/// Comparador: pela fita das portas do pós-jogo (comparar_pos): as desafeições (0x547af0), a marca (0x867560) e as
/// queixas (0x867700) rodam uma vez, na original, e a cópia confere a chamada e recebe o estado de depois. Regiões: o
/// sorteador, [0xae1080], as seis pessoas do clube (+0x9b, a 0x547940), o contrato do jogador e, de cada pessoa do
/// vetor, as preferências (+0x1c..+0x33), o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
unsafe extern "thiscall" fn comparar_apply_squad_reaction_to_player_story(this: usize, topico: u32, jogador: usize, clube: usize, ids: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, u32)>(%%I0%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if clube != 0 && jogador != 0 {
        reg.push((clube + 0x9b, 24));
        reg.extend(contract_region(jogador, clube));
        for &p in club_staff_vector(clube, &mut |r, t, a| dispatch_post_match_call(r, t, a)).iter().take(57) {
            if p == 0 {
                continue;
            }
            let pref = cqb8_d(p + 0x65);
            if pref != 0 {
                reg.push((pref + 0x1c, 24));
            }
            reg.push((p + 0x58, 6));
            reg.extend(contract_region(p, clube));
        }
    }
    comparar_pos("apply_squad_reaction_to_player_story", reg, || o(this, topico, jogador, clube, ids, n),
        |chamar| squad_reaction_to_story_with(topico, jogador, clube, ids, n, chamar));
}

/// { 0x758010 } (thiscall, ret 0x10, media.cpp): `Media::warn_squad_of_position_rival(jogador, clube, _, _)`, pelo
/// boato de transferência (0x754a30) e pela resposta à história do jogador (0x741020). Com ficha, clube com mais de 2
/// do elenco na máscara da posição dele (0x548150 com 0x5481a0), divisão com competição ([0xadadfc]) e reputação
/// (+0xb da ficha) acima de 4750: cada um dos 50 do elenco (+0xd7) com ficha, vínculo (0x4cf5f0) de felicidade (+0x3d)
/// positiva, o jogador sem lesão de estado 2 (0x62c400 em [0xb63d28]), sem as marcas 0x101 (+0x45), fora da lista
/// (0x544520), acima de 20 anos, do tipo 0x10/0x20/0x30 (+0x4f & 0xf0), com +0x5a abaixo de n + 5 e +0x5b abaixo de
/// n + 10, sorteio(+0x5a) abaixo de sorteio(n), a mesma máscara e reputação acima de 3750: a primeira vez, a história
/// 0x25 dele (0x72c1f0 em [0xb6b7c8]); sem a marca 0x800 (0x8676f0), a marca 0x800 (0x867560). As duas com efeito vão
/// por `chamar`, e também o vínculo 0x4cf5f0 (0x7580c2, unidade da fita) e, por `replay_inner_tape_calls`, o que a 0x62c400
/// (0x7580ee) e a 0x544520 (0x75810a) chamam de unidades por dentro.
unsafe fn warn_position_rivals_with(jogador: usize, clube: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    let contar: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x548150usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let bandeiras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
    if jogador == 0 || cqb8_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let n = contar(clube, mascara(jogador)) as i8;
    if n <= 2 {
        return;
    }
    let div = cqb8_d(clube + 0x57);
    if div == 0 || cqb8_d(cqb8_d(0xadadfc) + 4 * cqb8_d(div)) == 0 {
        return;
    }
    if cqb8_w(cqb8_d(jogador + 0x61) + 0xb) <= 0x128e {
        return;
    }
    crate::comparacao::ramo("cqb_8 rival de posicao");
    let n = n as i32;
    let mut postou = false;
    for i in 0..50 {
        let p = cqb8_d(clube + 0xd7 + 4 * i);
        if p == 0 || cqb8_d(p + 0x61) == 0 || p == jogador {
            continue;
        }
        // cqb8: a 0x4cf5f0 é unidade da fita (direta e de dentro da 0x544520; a 0x62c400 alcança a 0x600a50)
        let c = chamar(0x4cf5f0, cqb8_d(0xae1050), [p as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 || cqb8_d(p + 0x61) == 0 || cqb8_b(c + 0x3d) <= 0 {
            continue;
        }
        if replay_inner_tape_calls(chamar, || lesao(cqb8_d(0xb63d28), jogador)) == 2 || cqb8_d(c + 0x45) & 0x101 != 0
            || replay_inner_tape_calls(chamar, || listada(p, clube)) != 0
        {
            continue;
        }
        if cqb8_b(p + 0x18) <= 0x14 {
            continue;
        }
        let t = *((c + 0x4f) as *const u8) & 0xf0;
        if t != 0x10 && t != 0x20 && t != 0x30 {
            continue;
        }
        let a5a = cqb8_b(p + 0x5a) as i32;
        if a5a >= n + 5 || cqb8_b(p + 0x5b) as i32 >= n + 10 {
            continue;
        }
        let r1 = cqb8_sortear(n);
        if cqb8_sortear(a5a) >= r1 {
            continue;
        }
        let m = mascara(jogador);
        if m != mascara(p) || cqb8_w(cqb8_d(p + 0x61) + 0xb) <= 0xea6 {
            continue;
        }
        if !postou {
            chamar(0x72c1f0, cqb8_d(0xb6b7c8), [p as u32, clube as u32, 0x25, 0, jogador as u32]);
            postou = true;
        }
        if bandeiras(c) & 0x800 != 0 {
            continue;
        }
        chamar(0x867560, c, [0x800, 0, 0, 0, 0]);
    }
}

unsafe extern "thiscall" fn warn_squad_of_position_rival(_this: usize, jogador: usize, clube: usize, _a: u32, _b: u32) {
    warn_position_rivals_with(jogador, clube, &mut |r, t, a| dispatch_post_match_call(r, t, a))
}

/// Comparador: pela fita das portas do pós-jogo (a história 0x72c1f0 e a marca 0x867560 rodam só na original). Regiões:
/// o sorteador, [0xae1080] e, de cada um dos 50 do elenco, o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
unsafe extern "thiscall" fn comparar_warn_squad_of_position_rival(this: usize, jogador: usize, clube: usize, a: u32, b: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I1%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if clube != 0 {
        for i in 0..50 {
            let p = cqb8_d(clube + 0xd7 + 4 * i);
            if p != 0 {
                reg.push((p + 0x58, 6));
                reg.extend(contract_region(p, clube));
            }
        }
    }
    comparar_pos("warn_squad_of_position_rival", reg, || o(this, jogador, clube, a, b), |chamar| warn_position_rivals_with(jogador, clube, chamar));
}

/// As respostas 1 e 2 de cada caso da tabela de saltos da 0x735160 (a 3ª, de todos, é "No Comment", 0xa27808); o caso
/// vem da tabela de bytes do exe em 0x7356f4 (tipo − 3, até 0x7f; acima, o 21).
const CQB8_RESPOSTAS: [(usize, usize); 22] = [
    (0xa27700, 0xa2771c), (0xa27670, 0xa2767c), (0xa27688, 0xa276a4), (0xa276e4, 0xa276f0), (0xa2773c, 0xa27770),
    (0xa2764c, 0xa2765c), (0xa27430, 0xa2743c), (0xa2762c, 0xa2763c), (0xa2744c, 0xa27460), (0xa275d0, 0xa27534),
    (0xa27554, 0xa27574), (0xa27510, 0xa27534), (0xa275b4, 0xa275c0), (0xa274e0, 0xa274f8), (0xa27588, 0xa275a0),
    (0xa274b4, 0xa274c4), (0xa2744c, 0xa27494), (0xa2744c, 0xa27610), (0xa275e8, 0xa275f8), (0xa2744c, 0xa2747c),
    (0xa27798, 0xa277c8), (0xa2740c, 0xa2741c),
];

/// { 0x735160 } (stdcall, ret 0x10, media.cpp): `media_format_response_option_label(tipo, opção, saída, tamanho)`: o
/// rótulo da opção de resposta a uma história ("Deny rumour", "Defend Player", "No Comment"...) na saída (0x66f4e0),
/// pela tela das histórias (0x742b3b, com 0xde1f64 e 0x7d0). Saída nula ou tamanho abaixo de 15: nada.
unsafe extern "stdcall" fn media_format_response_option_label(tipo: u32, opcao: u32, saida: usize, tamanho: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    if saida == 0 || (tamanho as i16) < 0xf {
        return;
    }
    let k = (tipo as i16 as i32).wrapping_sub(3) as u32;
    let caso = if k > 0x7f { 21 } else { *((0x7356f4 + k as usize) as *const u8) as usize };
    let (um, dois) = CQB8_RESPOSTAS[caso];
    let texto = match opcao as i8 {
        1 => um,
        2 => dois,
        _ => 0xa27808,
    };
    formatar(saida, texto);
}

/// Comparador: lado a lado nos primeiros bytes da saída (até 0x50, o maior rótulo tem 61).
unsafe extern "stdcall" fn comparar_media_format_response_option_label(tipo: u32, opcao: u32, saida: usize, tamanho: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, u32, usize, u32)>(%%I2%%);
    let reg = if saida != 0 && (tamanho as i16) >= 0xf { vec![(saida, (tamanho as u16 as usize).min(0x50))] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tipo, opcao, saida, tamanho), || media_format_response_option_label(tipo, opcao, saida, tamanho));
    PLACAR.registrar(igual, || format!("media_format_response_option_label({}, {}) {:x?} x {:x?}", tipo as i16, opcao as i8, mo, mr));
}
