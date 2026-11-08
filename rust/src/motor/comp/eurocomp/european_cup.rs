use super::*;

// ---------- 272a leva: a Liga dos Campeões: o sorteio dos grupos da primeira fase, a ordem dos potes e as posições previstas (european_cup.cpp) ----------







/// The club's nation (+0x53).
///
/// [pt] A nação do clube (+0x53).
pub(crate) unsafe fn club_nation(clube: usize) -> usize {
    read_u32_as_usize(clube + club::NATION)
}

/// the 32 clubs sorted by pot, the same-nation partners in opposite halves, the draw of the 8 groups pot by pot, the
/// swap of same-nation clubs in a group, the coefficient, the 1,800,000 prize and its news, the 8 stages and the third
/// qualifying round losers into the UEFA Cup
/// Original: `EuropeanCup::draw_phase1_groups` (0x585290, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x585290 } (thiscall, ret 0, european_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `EuropeanCup::draw_phase1_groups()`, da fase seguinte da Liga dos Campeões (0x5851d0, fase 0).
/// 1) A lista dos 32 (0x945501(0x20, 9) push 9; push 0x20; add esp, 8 -> cdecl; 9 bytes: +0 o clube, +4 o parceiro da
///    mesma nação, +8 a metade: 1 grupos A..D, 2 grupos E..H; sem memória, a caixa da linha 0x4cc): os 16 primeiros da
///    tabela [+0x14] (6 bytes) e os times da tabela [+0xa7] (6 bytes) com +5 = 1 (os vencedores da terceira
///    preliminar). Menos de 32: a caixa da linha 0x4ee, +0x30 = −1, a lista liberada e +0xb1 = 0.
/// 2) A lista ordenada pelo pote (qsort 0x945333 com 0x586f60: +0x1df do clube e a reputação) e cada clube sem parceiro
///    ganha o primeiro seguinte sem parceiro da mesma nação (os dois se apontam).
/// 3) Os grupos (0x945501(0x20, 4): 8 grupos × 4 potes; sem memória, a caixa da linha 0x515 e o jogo para): para cada
///    pote de 8 (k = 0..3) começam livres 4 vagas em cada metade, menos as já marcadas; 8 vezes um clube do pote
///    sorteado (0x90cfc0(8) + 8k, de novo enquanto a vaga já saiu); sem metade, sorteia uma (0x90cfc0(livres A +
///    livres B) < livres B: a segunda) e o parceiro, se é do mesmo pote, fica na outra metade (sem vaga lá, não marca);
///    entra no primeiro grupo da metade com o pote livre.
/// 4) Em cada metade e pote, um clube com outro da mesma nação no grupo troca de lugar com o do mesmo pote do primeiro
///    outro grupo da metade sem a nação dele.
/// 5) Cada grupo: os 4 clubes ganham 1 ponto do coeficiente da nação (0x9058b0 em [0xde1f58]: push 1; push nação ->
///    thiscall, ret 8), o prêmio de 1.800.000 (0x5a0590 em [0xae2a38] push clube -> thiscall, ret 4; 0x5999a0 push
///    valor; mov ecx, finanças -> thiscall, ret 4) e a notícia do prêmio (0x48e3e0: push −2; push 0; push 0x449; push
///    −1; push 0; push valor; push clube; mov ecx, this -> thiscall, ret 0x1c); a tabela das rodadas do método +0x3c
///    (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) e o
///    construtor League_Stage 0x68b100: this, 4, o grupo, 2, +4, a tabela, as rodadas (word), 3, 1, 8, &{4,1,2,0},
///    &{2,0,0,0}, o ano (word), g, o id (word), 0xf, 2, 0, 8, −1, 0, 2; a original empurra o ano com lixo nos bytes
///    altos do edx, que o construtor não lê: a cópia passa o ano limpo) em [+0xc][g] (sem fase, a caixa da linha 0x5c9
///    e volta, sem liberar a tabela nem os grupos); a tabela das rodadas liberada, +0x30 = g e a notícia do grupo
///    (0x684230).
/// 6) Os grupos liberados; os times de +0xa7 com +5 = 2 (os perdedores da terceira preliminar) vão para a Copa da UEFA
///    ([[0xadadfc] + 4 × [0x9cf6f0]]; sem ela, a caixa da linha 0x5d8): nas últimas 24 vagas da tabela +0xa7 dela (a
///    partir de +0x3e − 24), +4 e +5 zerados, na tabela +0x14 dela com a situação 5 (+0x36 sobe) e com a competição
///    europeia do clube (+0x1db) = [0x9cf6f0]; depois as posições previstas da UEFA (método +0x5c dela).
pub(crate) unsafe extern "thiscall" fn draw_phase1_groups(this: usize) {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let pontos: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_UEFASEEDING_ADD_NATION_POINTS);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let send_prize_money_news: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let noticia_do_grupo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let regras_a: [u8; 4] = [4, 1, 2, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let lista = zerada(0x20, 9);
    if lista == 0 {
        show_error_at(0x9c4900, 0x4cc);
        return;
    }
    let ent = |i: i32| lista.wrapping_add((i * 9) as usize);
    let mut n = 0i16;
    for k in 0..16usize {
        let e = ent(n as i32);
        write_u32_from_usize(e, read_u32_as_usize(read_u32_as_usize(this + 0x14) + k * 6));
        write_u32_from_usize(e + 4, 0);
        n += 1;
    }
    let mut k = 0i32;
    while k < read_i16(this + 0x3e) as i32 {
        let t = read_u32_as_usize(this + 0xa7) + k as usize * 6;
        if read_u8(t + 5) == 1 {
            let e = ent(n as i32);
            write_u32_from_usize(e, read_u32_as_usize(t));
            write_u32_from_usize(e + 4, 0);
            n += 1;
        }
        k += 1;
    }
    if n < 0x20 {
        show_error_at(0x9c4900, 0x4ee);
        write_u32_from_usize(this + 0x30, usize::MAX);
        soltar(lista);
        write_u8(this + 0xb1, 0);
        return;
    }
    ordenar(lista, 0x20, 9, 0x586f60);
    for j in 0..0x20i32 {
        let e = ent(j);
        if read_u32_as_usize(e + 4) != 0 || j + 1 >= 0x20 {
            continue;
        }
        for i in (j + 1)..0x20 {
            let o = ent(i);
            if read_u32_as_usize(o + 4) == 0 && club_nation(read_u32_as_usize(e)) == club_nation(read_u32_as_usize(o)) {
                crate::comparacao::ramo("cqg_4 parceiro da mesma nação");
                write_u32_from_usize(e + 4, read_u32_as_usize(o)); // cqg4: o parceiro da mesma nação
                write_u32_from_usize(o + 4, read_u32_as_usize(e));
                break;
            }
        }
    }
    let grupos = zerada(0x20, 4);
    if grupos == 0 {
        fatal_error_at(0x9c4900, 0x515);
    }
    let vaga = |g: i32, pote: i32| grupos + ((pote + g * 4) * 4) as usize;
    let mut pote = 0i32;
    let mut base = 0i32;
    while base < 0x20 {
        let (mut livres_b, mut livres_a) = (4i8, 4i8); // [esp+0x12] a metade 2, [esp+0x13] a metade 1
        for i in 0..8 {
            match read_u8(ent(base + i) + 8) {
                2 => livres_b -= 1,
                1 => livres_a -= 1,
                _ => {}
            }
        }
        for _ in 0..8 {
            let mut s = sorteio(8).wrapping_add(base); // cqg4: o clube sorteado do pote
            while read_u32_as_usize(ent(s)) == 0 {
                s = sorteio(8).wrapping_add(base);
            }
            crate::comparacao::ramo("cqg_4 clube sorteado");
            let e = ent(s);
            if read_u8(e + 8) == 0 {
                crate::comparacao::ramo("cqg_4 metade sorteada");
                let b = livres_b as i32;
                if sorteio(livres_a as i32 + b) < b { // cqg4: a metade sorteada
                    write_u8(e + 8, 2);
                    livres_b -= 1;
                } else {
                    write_u8(e + 8, 1);
                    livres_a -= 1;
                }
                let par = read_u32_as_usize(e + 4);
                if par != 0 {
                    if let Some(i) = (0..0x20).find(|&i| read_u32_as_usize(ent(i)) == par) {
                        let mesmo_pote = i / 8 == s / 8;
                        let marca = if mesmo_pote {
                            let m = read_u8(e + 8);
                            if (m == 2 && livres_a == 0) || (m == 1 && livres_b == 0) {
                                false
                            } else {
                                if m == 2 {
                                    livres_a -= 1;
                                } else {
                                    livres_b -= 1;
                                }
                                true
                            }
                        } else {
                            true
                        };
                        if marca {
                            write_u8(ent(i) + 8, if read_u8(e + 8) != 2 { 2 } else { 1 });
                        }
                    }
                }
            }
            let ini: i32 = if read_u8(e + 8) == 1 { 0 } else { 4 };
            for g in ini..ini + 4 {
                if read_u32_as_usize(vaga(g, pote)) == 0 {
                    write_u32_from_usize(vaga(g, pote), read_u32_as_usize(e));
                    write_u32_from_usize(e, 0);
                    break;
                }
            }
        }
        base += 8;
        pote += 1;
    }
    soltar(lista);
    for metade in 0..2i32 {
        let (ini, fim) = (metade * 4, metade * 4 + 4);
        for p in 0..4i32 {
            let mut g = ini;
            while g < fim {
                let nacao = club_nation(read_u32_as_usize(vaga(g, p)));
                let repetida = (0..4).any(|q| q != p && nacao == club_nation(read_u32_as_usize(vaga(g, q))));
                if repetida {
                    for g2 in ini..fim {
                        if g2 == g {
                            continue;
                        }
                        if (0..4).all(|q| club_nation(read_u32_as_usize(vaga(g, p))) != club_nation(read_u32_as_usize(vaga(g2, q)))) {
                            crate::comparacao::ramo("cqg_4 troca pela nação");
                            let x = read_u32_as_usize(vaga(g, p));
                            write_u32_from_usize(vaga(g, p), read_u32_as_usize(vaga(g2, p)));
                            write_u32_from_usize(vaga(g2, p), x);
                            break;
                        }
                    }
                }
                g += 1;
            }
        }
    }
    for g in 0..8i32 {
        let linha = vaga(g, 0);
        for q in 0..4usize {
            let c = read_u32_as_usize(linha + q * 4);
            pontos(read_u32_as_usize(G_UEFA_SEEDING), club_nation(c), 1);
            let valor = 1_800_000i32; // cqg4: o prêmio da fase de grupos
            crate::comparacao::ramo("cqg_4 prêmio da fase de grupos");
            premio(financas(read_u32_as_usize(G_FINANCE_TABLE), c), valor);
            send_prize_money_news(this, c, valor, 0, -1, 0x449, 0, -2);
        }
        let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
        let (mut rodadas, mut id) = (0i16, 0i16);
        let tabela = rodadas_da_copa(this, g as u32, &mut rodadas, &mut id, 0);
        let p = novo(0xee);
        let fase = if p != 0 {
            construir(
                p,
                this,
                4,
                linha,
                2,
                read_u32_as_usize(this + 4),
                tabela,
                rodadas as u16 as u32,
                3,
                1,
                8,
                regras_a.as_ptr(),
                regras_b.as_ptr(),
                read_i16(this + 0x40) as u16 as u32,
                g as u32,
                id as u16 as u32,
                0xf,
                2,
                0,
                8,
                -1,
                0,
                2,
            )
        } else {
            0
        };
        write_u32_from_usize(read_u32_as_usize(this + 0xc) + g as usize * 4, fase);
        if read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4) == 0 {
            show_error_at(0x9c4900, 0x5c9);
            return;
        }
        soltar(tabela);
        write_u32_from_usize(this + 0x30, g as usize);
        noticia_do_grupo(read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4));
    }
    soltar(grupos);
    let uefa = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) * 4);
    if uefa == 0 {
        show_error_at(0x9c4900, 0x5d8);
        return;
    }
    let mut destino = ((read_i16(uefa + 0x3e) as i32 - 0x18) * 6) as usize;
    let mut k = 0i32;
    while k < read_i16(this + 0x3e) as i32 {
        let t = read_u32_as_usize(this + 0xa7) + k as usize * 6;
        if read_u8(t + 5) == 2 {
            let e = read_u32_as_usize(uefa + 0xa7).wrapping_add(destino);
            destino = destino.wrapping_add(6);
            write_u32_from_usize(e, read_u32_as_usize(t));
            write_u8(e + 4, 0);
            write_u8(e + 5, 0);
            let n = read_i16(uefa + 0x36) as i32;
            write_u32_from_usize(read_u32_as_usize(uefa + 0x14) + (n * 6) as usize, read_u32_as_usize(e));
            let n = read_i16(uefa + 0x36) as i32;
            write_u8(read_u32_as_usize(uefa + 0x14) + (n * 6) as usize + 4, 5);
            write_u16(uefa + 0x36, (read_i16(uefa + 0x36) as u16).wrapping_add(1));
            write_u32_from_usize(read_u32_as_usize(e) + 0x1db, read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP));
        }
        k += 1;
    }
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(uefa) + 0x5c));
    previstas(uefa);
}

/// The 32 clubs the draw puts in groups: the first 16 entrants of [+0x14] plus table entries (+0xa7) with +5 = 1,
/// sorted.
///
/// [pt] Os 32 clubes que o sorteio põe nos grupos: os 16 primeiros de [+0x14] e os de +0xa7 com +5 = 1.
pub(crate) unsafe fn group_draw_clubs(this: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..16usize).map(|k| read_u32_as_usize(read_u32_as_usize(this + 0x14) + k * 6)).collect();
    for k in 0..read_i16(this + 0x3e).max(0) as usize {
        let t = read_u32_as_usize(this + 0xa7) + k * 6;
        if read_u8(t + 5) == 1 {
            v.push(read_u32_as_usize(t));
        }
    }
    v.sort_unstable();
    v
}

/// Só a original (sorteia, paga prêmios, aloca as fases e manda notícias), conferindo o plano feito antes: com 32
/// clubes, as 8 fases em [+0xc], +0x30 = 7 e os clubes das tabelas das fases (+0xb1, linhas de 0x3b bytes, 4 por
/// fase) são exatamente os 32 do plano.
pub(crate) unsafe extern "thiscall" fn comparar_draw_phase1_groups(this: usize) {
    let plano = group_draw_clubs(this);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DRAW_PHASE1_GROUPS)(this);
    if plano.len() != 32 || read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) * 4) == 0 {
        return;
    }
    let fases = read_u32_as_usize(this + 0xc);
    let mut vistos = Vec::with_capacity(32);
    let mut ok = read_u32_as_usize(this + 0x30) == 7;
    for g in 0..8usize {
        let f = read_u32_as_usize(fases + g * 4);
        if f == 0 || read_u32_as_usize(f + 0xb1) == 0 {
            ok = false;
            break;
        }
        for r in 0..4usize {
            vistos.push(read_u32_as_usize(read_u32_as_usize(f + 0xb1) + r * 0x3b));
        }
    }
    vistos.sort_unstable();
    ok = ok && vistos == plano;
    PLACAR.registrar(ok, || format!("draw_phase1_groups: as fases não têm os 32 clubes do plano (+0x30 = {})", read_u32_as_usize(this + 0x30) as i32));
}

/// the pot comparison: the club's European seed (+0x1df), and on a tie the higher reputation first
/// Original: `european_cup_compare_clubs_by_seed` (0x586f60, european_cup.cpp, cdecl).
///
/// [pt] { 0x586f60 } (cdecl, european_cup.cpp): `european_cup_compare_clubs_by_seed(a, b)`, a comparação da qsort dos potes
/// do sorteio (0x585290): o byte +0x1df (a semente europeia) de a menos o de b; empate: a reputação (+0x80, word) de b
/// menos a de a.
pub(crate) unsafe extern "C" fn european_cup_compare_clubs_by_seed(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    let r = read_u8(x + 0x1df) as i8 as i32 - read_u8(y + 0x1df) as i8 as i32;
    if r != 0 {
        return r;
    }
    read_i16(y + 0x80) as i32 - read_i16(x + 0x80) as i32
}

/// Pura: a devolução.
pub(crate) unsafe extern "C" fn comparar_european_cup_compare_clubs_by_seed(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_EUROPEAN_CUP_COMPARE_CLUBS_BY_SEED)(a, b);
    let n = european_cup_compare_clubs_by_seed(a, b);
    PLACAR.registrar(r == n, || format!("european_cup_compare_clubs_by_seed: original={} rust={}", r, n));
    r
}



/// Positions 0x586900 records, in call order: knockout from the final back, second and first group stages, then
/// preliminaries; returns (club index, position) pairs and the knockout count.
///
/// [pt] As posições que a 0x586900 grava, na ordem das chamadas à 0x4a2540: (índice do clube no vetor ordenado, posição).
/// `mata` é a tabela das rodadas do mata-mata (0x68 bytes por rodada, +0x1a a conta de confrontos) com `nm` rodadas;
/// as preliminares vêm de [+0xa3] com +0x3c rodadas.
/// 1) O mata-mata, da final para trás: na final, os da primeira volta ganham a posição 1 (o contador sobe a cada um);
///    em toda rodada, cada um dos seguintes ganha o contador do começo da rodada (que sobe a cada um).
/// 2) A segunda fase: 2 × (4 clubes com o contador; o contador sobe 4). 3) A primeira fase: 2 × (8 clubes; sobe 8).
/// 4) As preliminares, da última para a primeira: cada clube da rodada com o contador do começo dela (sobe a cada um).
pub(crate) unsafe fn planned_final_positions(this: usize, mata: usize, nm: i16) -> (Vec<(i16, u8)>, usize) {
    let mut v = Vec::new();
    let mut i = 0i16;
    let mut cont = 1u8;
    let mut r = nm.wrapping_sub(1);
    while r >= 0 {
        let n = read_i16(mata + r as usize * 0x68 + 0x1a);
        if r == nm.wrapping_sub(1) {
            let mut k = 0i16;
            while k < n {
                crate::comparacao::ramo("cqg_4 campeão");
                v.push((i, 1u8)); // cqg4: o campeão
                i += 1;
                cont = cont.wrapping_add(1);
                k += 1;
            }
        }
        let p = cont;
        let mut k = 0i16;
        while k < read_i16(mata + r as usize * 0x68 + 0x1a) {
            v.push((i, p));
            i += 1;
            cont = cont.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    let n_mata = v.len();
    for &(vezes, quantos) in &[(2, 4u8), (2, 8u8)] {
        for _ in 0..vezes {
            let p = cont;
            cont = cont.wrapping_add(quantos);
            for _ in 0..quantos {
                v.push((i, p));
                i += 1;
            }
        }
    }
    let mut r = read_i16(this + 0x3c).wrapping_sub(1);
    while r >= 0 {
        let p = cont;
        let mut k = 0i16;
        while k < read_i16(read_u32_as_usize(this + 0xa3) + r as usize * 0x68 + 0x1a) {
            crate::comparacao::ramo("cqg_4 posição na preliminar");
            v.push((i, p)); // cqg4: a posição na preliminar
            i += 1;
            cont = cont.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    (v, n_mata)
}

/// the entrants sorted by strength and each one's predicted position: the knockout from the final back, the second and
/// first group phases and the qualifying rounds
/// Original: `EuropeanCup::rank_teams` (0x586900, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x586900 } (thiscall, ret 0, european_cup.cpp, método +0x5c da Liga dos Campeões): `EuropeanCup::rank_teams()`, as
/// posições previstas. Sem estatística (+8): a caixa da linha 0x81b. Os inscritos da tabela [+0x14] (6 bytes, +0x36
/// deles) num vetor (0x944e46; sem memória, a caixa da linha 0x823), ordenado pela força (0x4c08f0: push n; push vetor;
/// add esp, 8 -> cdecl); a tabela do mata-mata pelo método +0x3c (push 0; push &id; push &rodadas; push 0xc; mov ecx ->
/// thiscall, ret 0x10), liberada depois; a posição de cada um, na ordem do vetor, pela 0x4a2540 (push posição; push
/// clube; mov ecx, [+8] -> thiscall, ret 8; a original empurra o dword da pilha com a posição no byte de baixo e lixo
/// nos outros, e a 0x4a2540 só grava o byte: a cópia passa o byte limpo) no plano de `planned_final_positions`; o vetor liberado.
pub(crate) unsafe extern "thiscall" fn european_cup_rank_teams(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    if read_u32_as_usize(this + 8) == 0 {
        show_error_at(0x9c4900, 0x81b);
        return;
    }
    let v = alocar(((read_i16(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0x9c4900, 0x823);
        return;
    }
    let mut i = 0i16;
    while i < read_i16(this + 0x36) {
        core::ptr::write_unaligned((v + i as usize * 4) as *mut u32, read_u32_as_usize(read_u32_as_usize(this + 0x14) + i as usize * 6) as u32);
        i += 1;
    }
    ordenar(v, read_i16(this + 0x36) as i32);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let mata = rodadas_da_copa(this, 0xc, &mut nm, &mut id, 0);
    // as posições do mata-mata antes de liberar a tabela dele, as outras depois (como a original)
    let (plano, n_mata) = planned_final_positions(this, mata, nm);
    for (j, &(k, p)) in plano.iter().enumerate() {
        if j == n_mata {
            soltar(mata);
        }
        posicao(read_u32_as_usize(this + 8), read_u32_as_usize(v + k as usize * 4), p as u32);
    }
    if n_mata == plano.len() {
        soltar(mata);
    }
    soltar(v);
}

/// Só a original (acrescenta registros à estatística), conferindo o plano feito antes (o vetor ordenado numa cópia e a
/// tabela do mata-mata pedida ao método +0x3c e liberada): a posição (+0x72 do registro achado por 0x4a4850) de cada
/// clube do plano.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EUROPEAN_CUP_RANK_TEAMS);
    if read_u32_as_usize(this + 8) == 0 || read_u32_as_usize(this + 0x14) == 0 || read_i16(this + 0x36) <= 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut u32, i32) = core::mem::transmute(FN_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let n = read_i16(this + 0x36) as usize;
    let mut v: Vec<u32> = (0..n).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0x14) + i * 6) as u32).collect();
    ordenar(v.as_mut_ptr(), n as i32);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let mata = rodadas_da_copa(this, 0xc, &mut nm, &mut id, 0);
    let plano = if mata != 0 { planned_final_positions(this, mata, nm).0 } else { Vec::new() };
    soltar(mata);
    o(this);
    if plano.iter().any(|&(k, _)| k as usize >= n) {
        return; // a original lê além do vetor: sem conferência
    }
    let errado = plano.iter().find(|&&(k, p)| {
        let c = v[k as usize] as usize;
        c != 0 && {
            let e = achar(read_u32_as_usize(this + 8), c);
            e == 0 || *((e + 0x72) as *const u8) != p
        }
    });
    PLACAR.registrar(errado.is_none(), || format!("european_cup_rank_teams: a posição {:?} não bate", errado));
}

// ---------- 272a leva: a Liga dos Campeões: o sorteio dos grupos da segunda fase e a ordem dos classificados; o grupo de uma fase (european_cup.cpp, comp_util.cpp) ----------








/// Copies a 7-byte group entry (club, same-nation count, first-stage group, winner/runner-up) as the original does.
///
/// [pt] Copia a entrada de 7 bytes `de` (+0 o clube, +4 os da mesma nação, +5 o grupo da primeira fase, +6 0 vencedor / 1
/// segundo) para `para`, como a original: o dword, o word +4 e o byte +6.
pub(crate) unsafe fn copy_group_entry(para: usize, de: usize) {
    write_u32_from_usize(para, read_u32_as_usize(de));
    write_u16(para + 4, read_i16(de + 4) as u16);
    write_u8(para + 6, read_u8(de + 6));
}

/// the 16 qualifiers sorted, the same-group pairs, each one's group choice (shuffled order, fewest teams, no same group
/// nor nation), the redraw up to 20 times, the coefficient, the 1,200,000 prize and its news, stages 8 to 11 and the
/// third places into the UEFA Cup
/// Original: `EuropeanCup::draw_phase2_groups` (0x585ab0, european_cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x585ab0 } (thiscall, ret 4, european_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `EuropeanCup::draw_phase2_groups(tentativa)`, da fase seguinte da Liga dos Campeões (0x5851d0, fase 8,
/// tentativa 0).
/// 1) Os 16 classificados (0x945501(0x10, 7); sem memória, a caixa da linha 0x61c): de cada grupo b da primeira fase
///    ([+0xc][b]) o primeiro e o segundo da tabela (0x6826d0 push 0 / push 1; mov ecx, fase -> thiscall, ret 4), com o
///    grupo b e 0/1; +4 conta os da lista com a mesma nação (o próprio incluído); a lista ordenada (qsort 0x945333 com
///    0x5871e0) e os cartões de cada um zerados no órgão da competição (0x7e9180, como no sorteio da Copa do Mundo).
/// 2) Os pares do mesmo grupo da primeira fase (duas listas zeradas de 16, sem memória as caixas 0x649 e 0x651): cada
///    clube ainda na lista vai para a vaga k (vencedor) ou k + 1 (segundo) e o outro do grupo dele para a outra; k sobe 2.
/// 3) Cada um dos 16, na ordem dos pares: a ordem dos 4 grupos embaralhada (5 trocas de 0x90cfc0(4) e 0x90cfc0(4));
///    o grupo com menos times entre os que têm vaga na metade dele (vencedores nas vagas 0/1, segundos nas 2/3) e não
///    têm time do mesmo grupo da primeira fase nem da mesma nação recebe o clube.
/// 4) Sobrou clube sem grupo: com a tentativa < 20, as listas liberadas e o sorteio de novo com a tentativa + 1 (a
///    chamada fica dentro desta); na 20ª, cada sobra ocupa a primeira vaga livre (o clube continua na lista dos pares).
/// 5) Cada grupo h (0..3): os 4 clubes ganham 1 ponto do coeficiente, o prêmio de 1.200.000 e a notícia (0x48e3e0 com a
///    rodada 0x44a); a fase h + 8 (o método +0x3c, 0x944cf1(0xee) e o construtor 0x68b100 com os mesmos argumentos do
///    sorteio da primeira fase, a lista dos 4 clubes copiada num vetor de 0x40 bytes (0x944e46); sem fase, a caixa da
///    linha 0x715), +0x30 = h + 8 e a notícia do grupo. As quatro listas liberadas.
/// 6) Os terceiros da primeira fase (0x6826d0 push 2) vão para a Copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]; sem ela, a
///    caixa da linha 0x729): nas últimas 8 vagas da tabela +0xa7 dela, de trás para a frente, na tabela +0x14 com a
///    situação 5 e com a competição europeia do clube = a UEFA; os quartos (push 3) ficam sem competição europeia (−1);
///    depois as posições previstas da UEFA (método +0x5c).
pub(crate) unsafe extern "thiscall" fn draw_phase2_groups(this: usize, tentativa: u32) {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let cartoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_RULINGBODY_RESET_BOOKINGS_FOR_CLUB);
    let pontos: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_UEFASEEDING_ADD_NATION_POINTS);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let send_prize_money_news: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let noticia_do_grupo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let regras_a: [u8; 4] = [4, 1, 2, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let w = zerada(0x10, 7);
    if w == 0 {
        show_error_at(0x9c4900, 0x61c);
        return;
    }
    let e7 = |l: usize, i: i32| l.wrapping_add((i * 7) as usize);
    for b in 0..8i32 {
        let fase = || read_u32_as_usize(read_u32_as_usize(this + 0xc) + b as usize * 4);
        let e = e7(w, 2 * b);
        write_u32_from_usize(e, read_u32_as_usize(linha(fase(), 0)));
        write_u8(e + 6, 0);
        write_u8(e + 5, b as u8);
        write_u8(e + 4, 0);
        let e = e7(w, 2 * b + 1);
        write_u32_from_usize(e, read_u32_as_usize(linha(fase(), 1)));
        write_u8(e + 6, 1);
        write_u8(e + 5, b as u8);
        write_u8(e + 4, 0);
    }
    for i in 0..16 {
        for j in 0..16 {
            if club_nation(read_u32_as_usize(e7(w, i))) == club_nation(read_u32_as_usize(e7(w, j))) {
                write_u8(e7(w, i) + 4, read_u8(e7(w, i) + 4).wrapping_add(1));
            }
        }
    }
    ordenar(w, 0x10, 7, 0x5871e0);
    for i in 0..16 {
        let orgao = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(G_DISCIPLINE)).wrapping_add((read_u8(this + 0x50) as i8 as i32 * 4) as usize));
        cartoes(orgao, read_u32_as_usize(e7(w, i)));
    }
    let r = zerada(0x10, 7);
    if r == 0 {
        show_error_at(0x9c4900, 0x649);
        return;
    }
    let g = zerada(0x10, 7);
    if g == 0 {
        show_error_at(0x9c4900, 0x651);
        return;
    }
    let mut k = 0i8;
    for i in 0..16i32 {
        let e = e7(w, i);
        if read_u32_as_usize(e) == 0 {
            continue;
        }
        let c = k as i32;
        copy_group_entry(e7(r, if read_u8(e + 6) != 0 { c + 1 } else { c }), e);
        write_u32_from_usize(e, 0);
        for a in 0..16i32 {
            if i != a && read_u8(e + 5) == read_u8(e7(w, a) + 5) {
                let o = e7(w, a);
                let destino = if read_u32_as_usize(e7(r, c)) != 0 { e7(r, c + 1) } else { e7(r, c) };
                crate::comparacao::ramo("cqg_6 par do mesmo grupo");
                copy_group_entry(destino, o); // cqg6: o outro do grupo
                k = k.wrapping_add(2);
                write_u32_from_usize(o, 0);
                break;
            }
        }
    }
    let mut ordem = [0u8, 1, 2, 3];
    for i in 0..16i32 {
        let p = e7(r, i);
        let mut menos = 4i8;
        let mut melhor = 0xffu8;
        let s: i32 = if read_u8(p + 6) != 0 { 2 } else { 0 };
        for _ in 0..5 {
            let a = sorteio(4) as i8 as usize;
            let b = sorteio(4) as i8 as usize; // cqg6: a troca da ordem dos grupos
            crate::comparacao::ramo("cqg_6 troca da ordem");
            ordem.swap(a, b);
        }
        for q in 0..4usize {
            let gr = ordem[q] as i8 as i32;
            let mut n = 0i8;
            let mut conflito = false;
            for j in 0..4i32 {
                let v = e7(g, j + gr * 4);
                let clube = read_u32_as_usize(v);
                if clube != 0 {
                    n += 1;
                    if read_u8(v + 5) == read_u8(p + 5) || club_nation(clube) == club_nation(read_u32_as_usize(p)) { // cqg6: mesmo grupo ou nação
                        conflito = true;
                        break;
                    }
                }
            }
            let vaga = read_u32_as_usize(e7(g, s + gr * 4)) == 0 || read_u32_as_usize(e7(g, s + 1 + gr * 4)) == 0;
            if vaga && !conflito && n < menos {
                menos = n;
                melhor = ordem[q];
            }
        }
        if melhor != 0xff {
            crate::comparacao::ramo("cqg_6 grupo escolhido");
            let base = melhor as i8 as i32 * 4;
            let v = if read_u32_as_usize(e7(g, s + base)) == 0 { e7(g, s + base) } else { e7(g, s + 1 + base) };
            copy_group_entry(v, p);
            write_u32_from_usize(p, 0);
        }
    }
    for i in 0..16i32 {
        let p = e7(r, i);
        if read_u32_as_usize(p) == 0 {
            continue;
        }
        if (tentativa as u8 as i8) < 0x14 {
            crate::comparacao::ramo("cqg_6 sorteio de novo");
            soltar(g);
            soltar(r);
            soltar(w);
            let de_novo: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE2_GROUPS);
            de_novo(this, (tentativa as u8).wrapping_add(1) as u32);
            return;
        }
        if let Some(v) = (0..16).map(|j| e7(g, j)).find(|&v| read_u32_as_usize(v) == 0) {
            copy_group_entry(v, p);
        }
    }
    let t = alocar(0x40);
    for i in 0..16usize {
        write_u32_from_usize(t + i * 4, read_u32_as_usize(e7(g, i as i32)));
    }
    for h in 0..4i32 {
        for q in 0..4i32 {
            let c = read_u32_as_usize(e7(g, h * 4 + q));
            pontos(read_u32_as_usize(G_UEFA_SEEDING), club_nation(c), 1);
            let valor = 1_200_000i32; // cqg6: o prêmio da segunda fase
            crate::comparacao::ramo("cqg_6 prêmio da segunda fase");
            premio(financas(read_u32_as_usize(G_FINANCE_TABLE), c), valor);
            send_prize_money_news(this, c, valor, 0, -1, 0x44a, 0, -2);
        }
        let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
        let (mut rodadas, mut id) = (0i16, 0i16);
        let tabela = rodadas_da_copa(this, (h + 8) as u32, &mut rodadas, &mut id, 0);
        let p = novo(0xee);
        let fase = if p != 0 {
            construir(
                p,
                this,
                4,
                t + h as usize * 16,
                2,
                read_u32_as_usize(this + 4),
                tabela,
                rodadas as u16 as u32,
                3,
                1,
                8,
                regras_a.as_ptr(),
                regras_b.as_ptr(),
                read_i16(this + 0x40) as u16 as u32,
                (h + 8) as u32,
                id as u16 as u32,
                0xf,
                2,
                0,
                8,
                -1,
                0,
                2,
            )
        } else {
            0
        };
        let slot = read_u32_as_usize(this + 0xc) + (h + 8) as usize * 4;
        write_u32_from_usize(slot, fase);
        if read_u32_as_usize(read_u32_as_usize(this + 0xc) + (h + 8) as usize * 4) == 0 {
            show_error_at(0x9c4900, 0x715);
            return;
        }
        soltar(tabela);
        write_u32_from_usize(this + 0x30, (h + 8) as usize);
        noticia_do_grupo(read_u32_as_usize(read_u32_as_usize(this + 0xc) + (h + 8) as usize * 4));
    }
    soltar(w);
    soltar(g);
    soltar(r);
    soltar(t);
    let uefa = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) * 4);
    if uefa == 0 {
        show_error_at(0x9c4900, 0x729);
        return;
    }
    let mut destino = ((read_i16(uefa + 0x3e) as i32 - 1) * 6) as usize;
    for b in 0..8usize {
        let e = read_u32_as_usize(uefa + 0xa7).wrapping_add(destino);
        destino = destino.wrapping_sub(6);
        let terceiro = read_u32_as_usize(linha(read_u32_as_usize(read_u32_as_usize(this + 0xc) + b * 4), 2));
        write_u8(e + 4, 0);
        write_u32_from_usize(e, terceiro);
        write_u8(e + 5, 0);
        let n = read_i16(uefa + 0x36) as i32;
        write_u32_from_usize(read_u32_as_usize(uefa + 0x14) + (n * 6) as usize, read_u32_as_usize(e));
        let n = read_i16(uefa + 0x36) as i32;
        write_u8(read_u32_as_usize(uefa + 0x14) + (n * 6) as usize + 4, 5);
        write_u16(uefa + 0x36, (read_i16(uefa + 0x36) as u16).wrapping_add(1));
        write_u32_from_usize(read_u32_as_usize(e) + 0x1db, read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP));
        let quarto = read_u32_as_usize(linha(read_u32_as_usize(read_u32_as_usize(this + 0xc) + b * 4), 3));
        write_u32_from_usize(quarto + 0x1db, usize::MAX);
    }
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(uefa) + 0x5c));
    previstas(uefa);
}

/// The 16 clubs that go to the second stage (top two of each first-stage group), sorted.
///
/// [pt] Os 16 clubes que vão para a segunda fase (o primeiro e o segundo de cada grupo da primeira).
pub(crate) unsafe fn second_stage_clubs(this: usize) -> Vec<usize> {
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let mut v = Vec::with_capacity(16);
    for b in 0..8usize {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + b * 4);
        v.push(read_u32_as_usize(linha(f, 0)));
        v.push(read_u32_as_usize(linha(f, 1)));
    }
    v.sort_unstable();
    v
}

/// Só a original (sorteia, paga prêmios, aloca as fases e manda notícias; e se chama de novo), conferindo o plano: na
/// chamada de fora (tentativa 0), as fases 8..11 existem, +0x30 = 11 e os clubes das tabelas delas (+0xb1, linhas de
/// 0x3b bytes) são os 16 classificados.
pub(crate) unsafe extern "thiscall" fn comparar_draw_phase2_groups(this: usize, tentativa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW_PHASE2_GROUPS);
    if tentativa as u8 != 0 || read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) * 4) == 0 {
        return o(this, tentativa);
    }
    let plano = second_stage_clubs(this);
    o(this, tentativa);
    let mut vistos = Vec::with_capacity(16);
    let mut ok = read_u32_as_usize(this + 0x30) == 11;
    for h in 8..12usize {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + h * 4);
        if f == 0 || read_u32_as_usize(f + 0xb1) == 0 {
            ok = false;
            break;
        }
        for r in 0..4usize {
            vistos.push(read_u32_as_usize(read_u32_as_usize(f + 0xb1) + r * 0x3b));
        }
    }
    vistos.sort_unstable();
    ok = ok && vistos == plano;
    PLACAR.registrar(ok, || format!("draw_phase2_groups: as fases não têm os 16 classificados (+0x30 = {})", read_u32_as_usize(this + 0x30) as i32));
}

/// the qualifiers comparison: more same-nation clubs first, then the higher nation word +0x8e, then the group winner
/// before the runner-up
/// Original: `european_cup_compare_phase2_draw_entries` (0x5871e0, european_cup.cpp, cdecl).
///
/// [pt] { 0x5871e0 } (cdecl, european_cup.cpp): `european_cup_compare_phase2_draw_entries(a, b)`, a comparação da qsort dos
/// classificados (0x585ab0): os da mesma nação (+4) de b menos os de a; empate: o word +0x8e da nação (+0x53 do clube) de
/// b menos o de a; empate: +6 (o segundo) de a menos o de b.
pub(crate) unsafe extern "C" fn european_cup_compare_phase2_draw_entries(a: usize, b: usize) -> i32 {
    let r = read_u8(b + 4) as i8 as i32 - read_u8(a + 4) as i8 as i32;
    if r != 0 {
        return r;
    }
    let r = read_i16(club_nation(read_u32_as_usize(b)) + 0x8e) as i32 - read_i16(club_nation(read_u32_as_usize(a)) + 0x8e) as i32;
    if r != 0 {
        return r;
    }
    read_u8(a + 6) as i8 as i32 - read_u8(b + 6) as i8 as i32
}

/// Pura: a devolução.
pub(crate) unsafe extern "C" fn comparar_european_cup_compare_phase2_draw_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_EUROPEAN_CUP_COMPARE_PHASE2_DRAW_ENTRIES)(a, b);
    let n = european_cup_compare_phase2_draw_entries(a, b);
    PLACAR.registrar(r == n, || format!("european_cup_compare_phase2_draw_entries: original={} rust={}", r, n));
    r
}



// ---------- 262a leva: a Liga dos Campeões: os registros pela fase alcançada, o sorteio das quartas e a lista dos clubes inscritos (european_cup.cpp) ----------





/// comp_util_scale_by_round (0x4bf850): a value between low and high by round, 0..100.
///
/// [pt] comp_util_scale_by_round (0x4bf850, cdecl(baixo, alto, rodada, rodadas)): a faixa pela rodada, de 0 a 100.
pub(crate) unsafe fn scale_by_round(baixo: u32, alto: u32, rodada: i16, rodadas: u32) -> u32 {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_SCALE_BY_ROUND);
    // ponytail: a original empurra a rodada num registrador com lixo nos 16 bits altos; a 0x4bf850 só lê a palavra.
    f(baixo, alto, rodada as u16 as u32, rodadas)
}

/// Staff_Records add_qualified_for_round_record for the club.
///
/// [pt] Staff_Records::add_qualified_for_round_record (0x86bdd0: thiscall(registros, clube, comp, fase, rodada, peso),
/// ret 0x14).
pub(crate) unsafe fn european_cup_add_qualified_record(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
    f(eng_second_staff_records(), clube, comp, fase, rodada, peso);
}

/// Staff_Records add_failed_to_qualify_record for the club.
///
/// [pt] Staff_Records::add_failed_to_qualify_record (0x86c1d0: thiscall(registros, clube, comp, fase, _), ret 0x10).
pub(crate) unsafe fn european_cup_add_failed_to_qualify_record(clube: usize, comp: usize, fase: u32, x: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FAILED_TO_QUALIFY_RECORD);
    f(eng_second_staff_records(), clube, comp, fase, x);
}

/// Staff_Records add_knocked_out_record for the club.
///
/// [pt] Staff_Records::add_knocked_out_record (0x86c000: thiscall(registros, clube, comp, fase, rodada, _), ret 0x14).
pub(crate) unsafe fn european_cup_add_knocked_out_record(clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
    f(eng_second_staff_records(), clube, comp, fase, rodada, x);
}

/// Removes the club from its European competition this season (EURO_FLAG +0x1db = -1).
///
/// [pt] O clube sai da competição europeia da temporada (+0x1db = −1).
pub(crate) unsafe fn leave_euro_competition(clube: usize) {
    core::ptr::write_unaligned((clube + club::EURO_FLAG) as *mut i32, -1);
}

/// by outcome and stage: in qualifying, the qualified (0x86bdd0) or knocked-out (0x86c000) record weighted by round
/// (0x4bf850); in the group stages, qualified (6) or failed to qualify (0xfe, 0x86c1d0); in the knockout, winners
/// (0x86a800), runners-up (0x86b0b0), qualified or knocked out; whoever leaves gets +0x1db = -1; returns 0
/// Original: `EuropeanCup::update_staff_records_for_status` (0x584ae0, european_cup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x584ae0 } (thiscall, ret 0x18, european_cup.cpp): `EuropeanCup::update_staff_records_for_status(clube,
/// desfecho, fase, _, jogo, final)`, a virtual +0x44 da Liga dos Campeões. Devolve 0. Comp = +4.
/// - Na qualificação (fase 0xff): desfecho 0 ou 1, a classificação pela rodada do jogo (+0x34): na rodada 0, com a
///   fase +0x3a e o +0x6f da primeira rodada (+0xa3) e o peso 0x4bf850(1, 0x14, 0, 2); noutra, 0x41c, 0 e
///   0x4bf850(1, 0x14, rodada, 2). Desfecho 0xff ou 2: a eliminação (0x86c000) com +0x32 e +0x30 do jogo e o peso
///   0x4bf850(1, 0x14, rodada, 2), e o clube sai (+0x1db = −1).
/// - Na fase 1 (fase < 8): 6, a classificação (0x44a, −1, 0x32); 0xfe, a não classificação (0x449, 0x32) e sai.
/// - Na fase 2 (8 a 11): 6, a classificação (0, 0x78, 0x32); 0xfe, a não classificação (0x44a, 0x32) e sai.
/// - No mata-mata (12): 1, o título (Staff_Records::add_cup_winners_record 0x86a800, thiscall(registros, clube, jogo,
///   final), ret 0xc) e sai; 2, o vice (0x86b0b0, idem) e sai; 0, a classificação (0, o +0x6f da rodada do jogo na
///   tabela de rodadas da fase do mata-mata ([+0xc] + 0x30, +0xa3, 0x68 bytes), 0x4bf850(0x4b, 0x64, rodada, 2));
///   0xff, a eliminação (0, +0x30 do jogo, 0x4bf850(0x4b, 0x64, rodada, 2)) e sai.
pub(crate) unsafe extern "thiscall" fn european_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, _a: u32, jogo: usize, fim: u32) -> u32 {
    let d = desfecho as u8;
    let f = fase as u8;
    let comp = read_usize(this + 4);
    if f == 0xff {
        match d {
            0 | 1 => {
                let r = read_i16(jogo + 0x34);
                if r == 0 {
                    let peso = scale_by_round(1, 0x14, 0, 2);
                    let rodada = read_i16(read_usize(this + 0xa3) + 0x6f) as u16 as u32;
                    // ponytail: a fase +0x3a vai num registrador com lixo nos 16 bits altos; a 0x86bdd0 só usa a palavra.
                    european_cup_add_qualified_record(clube, comp, read_i16(this + 0x3a) as u16 as u32, rodada, peso);
                } else {
                    let peso = scale_by_round(1, 0x14, r, 2);
                    european_cup_add_qualified_record(clube, comp, 0x41c, 0, peso);
                }
            }
            0xff | 2 => {
                let peso = scale_by_round(1, 0x14, read_i16(jogo + 0x34), 2);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras.
                european_cup_add_knocked_out_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
                crate::comparacao::ramo("cpz_4 caiu na qualificacao");
                leave_euro_competition(clube); // cpz4: quem cai na qualificação sai da competição europeia
            }
            _ => {}
        }
    } else if (f as i8) < 8 {
        if d == 6 {
            european_cup_add_qualified_record(clube, comp, 0x44a, 0xffff_ffff, 0x32);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(clube, comp, 0x449, 0x32);
            crate::comparacao::ramo("cpz_4 caiu na fase 1");
            leave_euro_competition(clube); // cpz4: quem cai na fase 1 sai da competição europeia
        }
    } else if (f as i8) < 0xc {
        if d == 6 {
            crate::comparacao::ramo("cpz_4 passou da fase 2");
            european_cup_add_qualified_record(clube, comp, 0, 0x78, 0x32); // cpz4: pass_ball da fase 2 pesa 0x32
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(clube, comp, 0x44a, 0x32);
            leave_euro_competition(clube);
        }
    } else if f == 0xc {
        match d {
            1 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_CUP_WINNERS_RECORD);
                titulo(eng_second_staff_records(), clube, jogo, fim);
                leave_euro_competition(clube);
            }
            2 => {
                let vice: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_CUP_RUNNERS_UP_RECORD);
                vice(eng_second_staff_records(), clube, jogo, fim);
                leave_euro_competition(clube);
            }
            0 => {
                let peso = scale_by_round(0x4b, 0x64, read_i16(jogo + 0x34), 2);
                let mata = read_usize(read_usize(this + 0xc) + 0x30);
                let rodadas = read_usize(mata + 0xa3);
                let r = read_i16(jogo + 0x34) as i32;
                // os bits de cima do registrador que a original empurra são os do ponteiro das fases (+0xc)
                let rodada = (read_usize(this + 0xc) as u32 & 0xffff_0000) | read_i16(rodadas.wrapping_add((r as usize).wrapping_mul(0x68)) + 0x6f) as u16 as u32;
                european_cup_add_qualified_record(clube, comp, 0, rodada, peso);
            }
            0xff => {
                let peso = scale_by_round(0x4b, 0x64, read_i16(jogo + 0x34), 2);
                european_cup_add_knocked_out_record(clube, comp, 0, read_i16(jogo + 0x30) as u16 as u32, peso);
                leave_euro_competition(clube);
            }
            _ => {}
        }
    }
    0
}

// ponytail: os registros vão para o arquivo, mexem nas reputações e no moral (sorteiam) e o título e o vice gravam o
// elenco da final: não rodam duas vezes. Só a original roda; o plano confere o que a rotina mesma grava (+0x1db do
// clube: −1 nos casos em que ele sai, intacto nos outros).
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, a: u32, jogo: usize, fim: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_EUROPEAN_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let (d, f) = (desfecho as u8, fase as u8);
    // as faixas da original comparam o byte da fase com sinal (0x80 a 0xfe contam como fase 1)
    let sai = if f == 0xff {
        d == 0xff || d == 2
    } else if (f as i8) < 0xc {
        d == 0xfe
    } else if f == 0xc {
        d == 1 || d == 2 || d == 0xff
    } else {
        false
    };
    let antes = if clube != 0 { core::ptr::read_unaligned((clube + club::EURO_FLAG) as *const i32) } else { 0 };
    let r = o(this, clube, desfecho, fase, a, jogo, fim);
    let depois = if clube != 0 { core::ptr::read_unaligned((clube + club::EURO_FLAG) as *const i32) } else { 0 };
    let igual = r == 0 && (clube == 0 || depois == if sai { -1 } else { antes });
    PLACAR.registrar(igual, || format!("european_cup_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} +0x1db {} -> {} sai={}", this, clube, d, f, r, antes, depois, sai));
    r
}

/// Draws four distinct group indices (rand 4 each until unused).
///
/// [pt] Os quatro sorteados da fase 2 (os índices dos grupos, todos diferentes): a, b ≠ a, c ≠ a, b e d ≠ a, b, c, cada um
/// com sorteio(4) (0x90cfc0) até sair um que serve (a d sorteia pelo menos uma vez: começa igual à a).
pub(crate) unsafe fn draw_four_groups() -> [i8; 4] {
    let a = sortear(4) as i8;
    let mut b = sortear(4) as i8;
    while b == a {
        b = sortear(4) as i8;
    }
    let mut c = sortear(4) as i8;
    while c == a || c == b {
        c = sortear(4) as i8;
    }
    let mut d = a;
    while d == a || d == b || d == c {
        d = sortear(4) as i8;
    }
    [a, b, c, d]
}

/// Club in row k of second-stage group g (League get_table_row).
///
/// [pt] O clube da linha `k` do grupo `g` da fase 2 (a fase [+0xc] + 0x20 + 4 × g; League::get_table_row 0x6826d0,
/// thiscall(grupo, k), ret 4).
pub(crate) unsafe fn second_stage_group_club(this: usize, g: i8, k: u32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    read_usize(linha(read_usize(read_usize(this + 0xc) + 0x20 + 4 * (g as i32 as usize)), k))
}

/// draws the order of the 4 phase 2 groups (0x90cfc0) and pairs the quarter-finals (1st of a v 2nd of b, 2nd of a v 1st
/// of b, same for c and d), resets the 8 clubs bookings (0x7e9180), creates the knockout stage (0x522e00) in slot
/// +0x30, drops the 3rd and 4th of each group (+0x1db = -1), sets up the round (0x51c800) and +0x30 = 12
/// Original: `EuropeanCup::draw_quarter_finals` (0x586330, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x586330 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::draw_quarter_finals()`, da fase seguinte 0x5851d0
/// na fase 12. A lista das 8 (calloc 0x945501 de 8 × 4; sem memória: a caixa da linha 0x762) com os grupos sorteados
/// a, b, c e d: o 1º de a × o 2º de b, o 2º de a × o 1º de b, o 1º de c × o 2º de d e o 2º de c × o 1º de d. Cada um
/// passa pela 0x7e9180 (thiscall(o objeto [[0xae28f0]] + 4 × +0x50, clube), ret 4). A virtual +0x3c (create_round_table:
/// fase 12) e a fase do mata-mata (operator new 0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00 com a copa, 8, a lista,
/// as rodadas, +4, a tabela, +0x40, 0xc, 2, o extra, 0x14, 1, 0, 0, 0) na vaga [+0xc] + 0x30; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa da linha 0x78a. Senão o 3º e o 4º de cada grupo da fase 2 saem (+0x1db =
/// −1), a rodada montada (Cup::set_up_round 0x51c800, thiscall(fase, 0)) e +0x30 = 12.
pub(crate) unsafe extern "thiscall" fn draw_quarter_finals(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let lista = calloc(8, 4);
    if lista == 0 {
        return show_error_at(0x9c4900, 0x762);
    }
    crate::comparacao::ramo("cpz_4 sorteio das quartas");
    let [a, b, c, d] = draw_four_groups();
    let ordem: [(i8, u32); 8] = [(a, 0), (b, 1), (a, 1), (b, 0), (c, 0), (d, 1), (c, 1), (d, 0)];
    for (k, &(g, l)) in ordem.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, second_stage_group_club(this, g, l));
    }
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_RULINGBODY_RESET_BOOKINGS_FOR_CLUB);
    for k in 0..8 {
        let obj = read_usize(read_usize(read_usize(G_DISCIPLINE)) + 4 * (read_u8(this + 0x50) as i8 as i32 as usize));
        tv(obj, read_usize(lista + 4 * k));
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0xc, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: as rodadas, o extra e a palavra +0x40 vão em registradores com lixo nos 16 bits altos; o construtor
    // só lê as palavras.
    let fase = if p != 0 {
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 8, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 0xc, 2, extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 0x30) as *mut usize, fase);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(lista);
    liberar(t);
    let fase = read_usize(read_usize(this + 0xc) + 0x30);
    if fase == 0 {
        return show_error_at(0x9c4900, 0x78a);
    }
    for g in 0..4i8 {
        leave_euro_competition(second_stage_group_club(this, g, 2));
        leave_euro_competition(second_stage_group_club(this, g, 3));
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, 0xc);
}

// ponytail: o sorteio sorteia, a 0x7e9180 e o construtor alocam e a rodada cria os jogos: só a original. O plano,
// feito antes: os 8 primeiros e segundos dos grupos e o 3º e 4º de cada um; depois, +0x30 = 12, a fase do mata-mata
// com 8 clubes que são os primeiros e segundos (como conjunto) e o 3º e o 4º de cada grupo com +0x1db = −1.
pub(crate) unsafe extern "thiscall" fn comparar_draw_quarter_finals(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DRAW_QUARTER_FINALS);
    let mut vivos: Vec<usize> = Vec::new();
    let mut fora: Vec<usize> = Vec::new();
    for g in 0..4i8 {
        vivos.push(second_stage_group_club(this, g, 0));
        vivos.push(second_stage_group_club(this, g, 1));
        fora.push(second_stage_group_club(this, g, 2));
        fora.push(second_stage_group_club(this, g, 3));
    }
    o(this);
    let fase = read_usize(read_usize(this + 0xc) + 0x30);
    if fase == 0 {
        PLACAR.registrar(false, || format!("draw_quarter_finals({:#x}) sem fase", this));
        return;
    }
    let lista = read_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| read_usize(lista + 6 * k)).collect();
    let mut esperado = vivos.clone();
    achou.sort_unstable();
    esperado.sort_unstable();
    let saiu = fora.iter().all(|&c| core::ptr::read_unaligned((c + 0x1db) as *const i32) == -1);
    let igual = achou == esperado && saiu && read_i16(fase + 0x3e) == 8 && core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xc;
    PLACAR.registrar(igual, || format!("draw_quarter_finals({:#x}) esperado={:x?} lista={:x?} saiu={} +0x30={}", this, esperado, achou, saiu, core::ptr::read_unaligned((this + 0x30) as *const i32)));
}

/// Entrants (+0x14, 6 bytes each) still in the competition (+0x1db = the cup id), optionally excluding entry round 7;
/// writes them to v and returns the count.
///
/// [pt] A lista dos inscritos (+0x14, 6 bytes: +0 o clube, +4 a rodada de entrada; +0x38 deles) que seguem na competição
/// (+0x1db = o id da copa, [+4]); com `sem_7`, fora os de rodada de entrada 7. Grava em `v` (até 71) e devolve a conta.
pub(crate) unsafe fn active_entrants(this: usize, sem_7: bool, v: *mut usize) -> i16 {
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while i < read_i16(this + 0x38) {
        let e = read_usize(this + 0x14).wrapping_add((i as i32 as usize).wrapping_mul(6));
        let c = read_usize(e);
        if core::ptr::read_unaligned((c + 0x1db) as *const i32) == core::ptr::read_unaligned(read_usize(this + 4) as *const i32) && !(sem_7 && read_u8(e + 4) == 7) {
            core::ptr::write_unaligned(v.add(n as i32 as usize), c);
            n = n.wrapping_add(1);
        }
        i += 1;
    }
    n
}

/// allocates the list (0x11c bytes) of the entrants (+0x14) still in the cup (+0x1db = its id): kind 1 without the
/// round 7 entries, kind 2 all, kind 3 at stage 11; otherwise the competition one (0x48e1c0); returns the count
/// Original: `EuropeanCup::collect_club_list` (0x586660, european_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x586660 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::collect_club_list(&lista, tipo)`, a virtual +0x58
/// da Liga dos Campeões. Tipo 1: os inscritos que seguem, fora os da rodada 7; tipo 2: todos os que seguem; tipo 3 na
/// fase 11: os que seguem; noutro caso a da competição (Comp::collect_club_list 0x48e1c0, thiscall(this, &lista, 0),
/// ret 8). A lista (malloc 0x944e46 de 0x11c bytes; sem memória: a caixa das linhas 0x7b5, 0x7cf ou 0x7e9, lista
/// nula e 0) vai em `lista`; devolve a conta na palavra de baixo (a de cima é a do ponteiro da lista).
pub(crate) unsafe extern "thiscall" fn european_cup_collect_club_list(this: usize, saida: usize, tipo: u32) -> u32 {
    let t = tipo as u8;
    let (linha, sem_7) = match t {
        1 => (0x7b5, true),
        2 => (0x7cf, false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => (0x7e9, false),
        _ => {
            let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_COMP_COLLECT_CLUB_LIST);
            return base(this, saida, 0);
        }
    };
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alocar(0x11c);
    if v == 0 {
        show_error_at(0x9c4900, linha);
        core::ptr::write_unaligned(saida as *mut usize, 0);
        return 0;
    }
    crate::comparacao::ramo("cpz_4 lista dos inscritos");
    let n = active_entrants(this, sem_7, v as *mut usize);
    core::ptr::write_unaligned(saida as *mut usize, v);
    (v as u32 & 0xffff_0000) | n as u16 as u32
}

/// Comparador da 0x586660: a original aloca a lista (só ela roda; quem chama libera). O plano, feito antes numa lista
/// local pelos mesmos inscritos: a conta (palavra de baixo da devolução) e os clubes da lista dela iguais. Nos tipos
/// que vão para a 0x48e1c0 e sem lista: a devolução e a saída como a original deixou.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_collect_club_list(this: usize, saida: usize, tipo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_EUROPEAN_CUP_COLLECT_CLUB_LIST);
    let t = tipo as u8;
    let sem_7 = match t {
        1 => Some(true),
        2 => Some(false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => Some(false),
        _ => None,
    };
    let mut plano = [0usize; 71];
    let n = match sem_7 {
        Some(s) if read_i16(this + 0x38) <= 71 => active_entrants(this, s, plano.as_mut_ptr()),
        _ => return o(this, saida, tipo),
    };
    let r = o(this, saida, tipo);
    let v = read_usize(saida);
    if v == 0 {
        PLACAR.registrar(r as u16 == 0, || format!("european_cup_collect_club_list({:#x}, {}) sem lista original={:#x}", this, t, r));
        return r;
    }
    let lista = core::slice::from_raw_parts(v as *const usize, (n.max(0) as usize).min(71));
    let igual = r as u16 as i16 == n && lista == &plano[..lista.len()];
    PLACAR.registrar(igual, || format!("european_cup_collect_club_list({:#x}, {}) original={:#x} rust={} lista={:x?} plano={:x?}", this, t, r, n, lista, &plano[..lista.len()]));
    r
}

// ---------- 261a leva: os inscritos da Liga dos Campeões: a lista das quatro bandas, a tabela da qualificação e a fase seguinte (european_cup.cpp) ----------





/// CRT malloc(n).
pub(crate) unsafe fn european_cup_malloc(n: u32) -> usize {
    let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    f(n)
}

/// Writes a 6-byte entrant record: club (+0), +5 = 0, entry round (+4), in the original's order.
///
/// [pt] Uma entrada de 6 bytes da lista dos inscritos: o clube (+0), a rodada em que entra (+4) e +5 = 0 (nesta ordem,
/// como a original).
pub(crate) unsafe fn european_cup_write_entrant(p: usize, clube: usize, rodada: u8) {
    core::ptr::write_unaligned(p as *mut usize, clube);
    *((p + 5) as *mut u8) = 0;
    *((p + 4) as *mut u8) = rodada;
}

/// Loop of 0x584fa0: clubs entered in the Champions League (+0x1db) go to entrant list t by band (+0x1df) with their
/// entry round; returns the total count.
///
/// [pt] O laço da 0x584fa0 sobre os clubes ([0xae23b4], 0x245 bytes, [0xae235c] deles): quem tem a competição europeia
/// (+0x1db) da Liga dos Campeões ([0x9cf6e8]) entra pela banda (+0x1df) na lista `t`: 1 em +0 (até 16, rodada 7),
/// 2 em +0x60 (até 18, rodada 8), 3 em +0xcc (até 19, rodada 0xd), 4 em +0x13e (até 18, rodada 0). Devolve a soma
/// das quatro contas.
pub(crate) unsafe fn fill_champions_league_entrants(t: usize) -> i32 {
    let clubes = read_i32(0xae235c);
    let (mut n1, mut n2, mut n3, mut n4) = (0i32, 0i32, 0i32, 0i32);
    let (mut o1, mut o2, mut o3, mut o4) = (0usize, 0usize, 0usize, 0usize);
    if clubes <= 0 {
        return 0;
    }
    let cl = read_i32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    let mut c = read_usize(G_CLUB_TABLE);
    let mut i: i32 = 0;
    loop {
        if read_i32(c + 0x1db) == cl {
            match read_u8(c + 0x1df) {
                1 => {
                    if o1 < 0x60 {
                        european_cup_write_entrant(t + o1, c, 7);
                        n1 += 1;
                        o1 += 6;
                    }
                }
                2 => {
                    if o2 < 0x6c {
                        crate::comparacao::ramo("cpy_5 entrada da banda 2");
                        european_cup_write_entrant(t + 0x60 + o2, c, 8); // cpy5: a banda 2 entra na rodada 8
                        n2 += 1;
                        o2 += 6;
                    }
                }
                3 => {
                    if o3 < 0x72 {
                        crate::comparacao::ramo("cpy_5 entrada da banda 3");
                        european_cup_write_entrant(t + 0xcc + o3, c, 0xd); // cpy5: a banda 3 entra na rodada 0xd
                        n3 += 1;
                        o3 += 6;
                    }
                }
                4 => {
                    if o4 < 0x6c {
                        crate::comparacao::ramo("cpy_5 entrada da banda 4");
                        european_cup_write_entrant(t + 0x13e + o4, c, 0); // cpy5: a banda 4 entra na rodada 0
                        n4 += 1;
                        o4 += 6;
                    }
                }
                _ => {}
            }
        }
        c = c.wrapping_add(0x245);
        i += 1;
        if i as i16 as i32 >= clubes {
            break;
        }
    }
    n1 + n2 + n3 + n4
}

/// allocates the 71-entry list of 6-byte entries (+0x14) and fills it with the Champions League clubs (+0x1db =
/// [0x9cf6e8]) by band (+0x1df): 1 up to 16 (round 7), 2 up to 18 (8), 3 up to 19 (0xd), 4 up to 18 (0); short of 71,
/// +0x36 = 0 and returns 0
/// Original: `EuropeanCup::collect_entrants` (0x584fa0, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x584fa0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::collect_entrants()`, do construtor (0x583888, na
/// carreira nova) e do começo da temporada (0x584e5d). +0x36 e +0x38 = 71; libera a lista antiga (+0x14, free
/// 0x9452ca) e aloca 6 × 71 bytes (0x944e46; sem memória: a caixa da linha 0x44e e 0, com +0x14 nulo); preenche as
/// quatro bandas (fill_champions_league_entrants). Sem as 71 entradas: +0x36 = 0 e 0; senão 1.
pub(crate) unsafe extern "thiscall" fn european_cup_collect_entrants(this: usize) -> u32 {
    *((this + 0x36) as *mut i16) = 0x47;
    *((this + 0x38) as *mut i16) = 0x47;
    let velha = read_usize(this + 0x14);
    if velha != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        liberar(velha);
    }
    let t = european_cup_malloc((read_i16(this + 0x36) as i32).wrapping_mul(6) as u32);
    core::ptr::write_unaligned((this + 0x14) as *mut usize, t);
    if t == 0 {
        show_error_at(0x9c4900, 0x44e);
        return 0;
    }
    if fill_champions_league_entrants(t) != read_i16(this + 0x36) as i32 {
        *((this + 0x36) as *mut i16) = 0;
        return 0;
    }
    1
}

/// Comparador da 0x584fa0: a original aloca (só ela roda); o plano confere depois tudo o que ela gravou: a lista nova
/// copiada e repreenchida pela cópia (fill_champions_league_entrants só lê os clubes, que a original não muda) tem de sair igual,
/// +0x36, +0x38 e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EUROPEAN_CUP_COLLECT_ENTRANTS)(this);
    let t = read_usize(this + 0x14);
    if t == 0 {
        PLACAR.registrar(r == 0, || format!("european_cup_collect_entrants({:x}) sem lista original={}", this, r));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x47 * 6).to_vec();
    let total = fill_champions_league_entrants(v.as_mut_ptr() as usize);
    let n = if total == 0x47 { 1 } else { 0 };
    let igual = r == n
        && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x47 * 6)
        && read_i16(this + 0x36) == if n == 1 { 0x47 } else { 0 }
        && read_i16(this + 0x38) == 0x47;
    PLACAR.registrar(igual, || format!("european_cup_collect_entrants({:x}) original={} +0x36={} rust={} total={}", this, r, read_i16(this + 0x36), n, total));
    r
}

/// Loop of 0x584990 over the 55 table rows: each row gets an entrant from the list (reverse order) and a code from the
/// entry round (8 -> 2, 0xd -> 1, else 0).
///
/// [pt] O laço da 0x584990 sobre as 55 linhas (+0x3e) da tabela `t`: a linha i leva o clube da entrada (+0x36) − i − 1 da
/// lista (+0x14; lista vazia: 0) e a rodada pela entrada (+0x36) − i, byte +4 da anterior (8 → 2, 0xd → 1, outra →
/// 0); +5 = 0. Com +0x36 = 0 a original lê antes da lista (índice negativo), como aqui.
pub(crate) unsafe fn fill_qualifying_table(this: usize, t: usize) {
    let mut i: i32 = 0;
    let mut k = 0usize;
    while i < read_i16(this + 0x3e) as i32 {
        let n = read_i16(this + 0x36) as i32;
        let lista = read_usize(this + 0x14);
        let clube = if n != 0 { read_usize(lista.wrapping_add((n - i - 1).wrapping_mul(6) as usize)) } else { 0 };
        core::ptr::write_unaligned((t + k) as *mut usize, clube);
        let lista = read_usize(this + 0x14);
        let codigo = read_u8(lista.wrapping_add(((n - i).wrapping_mul(6) - 2) as usize));
        *((t + k + 4) as *mut u8) = match codigo {
            8 => 2,
            0xd => 1,
            _ => 0,
        };
        *((t + k + 5) as *mut u8) = 0;
        i += 1;
        k += 6;
    }
}

/// allocates the 55 qualifying rows (+0xa7, +0x3e = 55): row i takes the club of entry 70 - i and its entry round (8 ->
/// 2, 0xd -> 1, other -> 0); out of memory, -1
/// Original: `EuropeanCup::build_qualifying_entry_table` (0x584990, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x584990 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::build_qualifying_entry_table()`, logo depois da
/// 0x584fa0 (0x58388f, 0x584e9a). +0x3e = 55; aloca 6 × 55 bytes em +0xa7 (sem memória: a caixa da linha 0x2f6 e −1)
/// e monta as linhas da qualificação de trás para a frente (fill_qualifying_table); devolve 1.
pub(crate) unsafe extern "thiscall" fn build_qualifying_entry_table(this: usize) -> i32 {
    *((this + 0x3e) as *mut i16) = 0x37;
    let t = european_cup_malloc(0x14a);
    core::ptr::write_unaligned((this + 0xa7) as *mut usize, t);
    if t == 0 {
        show_error_at(0x9c4900, 0x2f6);
        return -1;
    }
    fill_qualifying_table(this, t);
    1
}

/// Comparador da 0x584990: a original aloca (só ela roda); depois a tabela dela, copiada e remontada pela cópia,
/// tem de sair igual, com +0x3e = 55 e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_build_qualifying_entry_table(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BUILD_QUALIFYING_ENTRY_TABLE)(this);
    let t = read_usize(this + 0xa7);
    if t == 0 || read_i16(this + 0x3e) != 0x37 {
        PLACAR.registrar(r == -1 && t == 0, || format!("build_qualifying_entry_table({:x}) original={} tabela={:x} +0x3e={}", this, r, t, read_i16(this + 0x3e)));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x14a).to_vec();
    fill_qualifying_table(this, v.as_mut_ptr() as usize);
    let igual = r == 1 && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x14a);
    PLACAR.registrar(igual, || format!("build_qualifying_entry_table({:x}) original={} tabela={:x?}", this, r, &v[..]));
    r
}

/// moves to the next stage (+0x30) and draws the phase 1 groups (stage 0), the phase 2 groups (8) or the quarter-finals
/// (12); at the last stage, the error box
/// Original: `EuropeanCup::advance_stage` (0x5851d0, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5851d0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::advance_stage()` (a virtual +0x28, do passe da
/// copa 0x51f904). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x495. Senão +0x30 + 1 e, na fase
/// 0, o sorteio dos grupos da fase 1 (0x585290, thiscall(this)); na 8, os da fase 2 (0x585ab0, thiscall(this, 0),
/// ret 4); na 12, as quartas (0x586330, thiscall(this)). Quem chama não lê a devolução.
pub(crate) unsafe extern "thiscall" fn european_cup_advance_stage(this: usize) {
    let f = read_i32(this + 0x30);
    if f >= read_i32(this + 0x2c).wrapping_sub(1) {
        return show_error_at(0x9c4900, 0x495);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    crate::comparacao::ramo("cpy_5 fase seguinte");
    match f {
        0 => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE1_GROUPS);
            sortear(this);
        }
        8 => {
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE2_GROUPS);
            sortear(this, 0);
        }
        0xc => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROPEANCUP_DRAW_QUARTER_FINALS);
            sortear(this);
        }
        _ => {}
    }
}

// ponytail: os sorteios sorteiam, alocam as fases e mandam notícias: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_advance_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EUROPEAN_CUP_ADVANCE_STAGE)(this)
}

// ---------- 261a leva: o jogo da Liga dos Campeões: o prêmio e o coeficiente do jogo, o progresso do clube e o conflito de datas da competição (european_cup.cpp, comp.cpp) ----------





/// Club's finance record (FINANCETABLE get_club_finance on G_FINANCE_TABLE).
///
/// [pt] O registro de finanças do clube (0x5a0590: thiscall(tabela [0xae2a38], clube), ret 4).
pub(crate) unsafe fn european_cup_club_finance(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    f(read_usize(G_FINANCE_TABLE), clube)
}

/// Adds prize money to the finance record (balance, +0xa8 and +0x148).
///
/// [pt] Soma no saldo, em +0xa8 e em +0x148 (0x5999a0: thiscall(finanças, valor), ret 4).
pub(crate) unsafe fn european_cup_add_prize_money(fin: usize, valor: i32) {
    let f: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    f(fin, valor)
}

/// Adds coefficient points for the nation (UEFASEEDING add_nation_points).
///
/// [pt] Os pontos do coeficiente da UEFA para a nação (0x9058b0: thiscall(tabela [0xde1f58], nação, pontos), ret 8; soma
/// o byte dos pontos na palavra +0xf + 18 × índice (+0x1df da seleção 0x53b3d0 da nação)).
pub(crate) unsafe fn add_uefa_coefficient(nacao: usize, pontos: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_UEFASEEDING_ADD_NATION_POINTS);
    f(read_usize(G_UEFA_SEEDING), nacao, pontos);
}

/// O ensaio do comparador da 0x583b10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca) e sem a base da copa
/// (0x51a150, que processa o jogo inteiro e não pode rodar duas vezes).
pub(crate) static mut CPY6_ENSAIO: bool = false;

/// Sends the prize money news (Comp 0x48e3e0); nothing in dry run.
///
/// [pt] A notícia do prêmio (Comp 0x48e3e0: thiscall(this, clube, valor, 0, −1, 0, jogo, −2), ret 0x1c); no ensaio, nada.
pub(crate) unsafe fn european_cup_send_prize_money_news(this: usize, clube: usize, valor: u32, jogo: usize) {
    if CPY6_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    f(this, clube, valor, 0, -1, 0, jogo, -2);
}

/// by the score (+0x47/+0x48, or +0x43/+0x44), 2 UEFA coefficient points to the winner's nation or 1 each on a draw
/// (0x9058b0); in the group stage (+0x42 from 0 to 11), 200 thousand to the winner or 100 thousand each, with the prize
/// news item (0x48e3e0); then the cup base (0x51a150)
/// Original: `EuropeanCup::process_fixture` (0x583b10, european_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x583b10 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::process_fixture(jogo, arg)` (a virtual +4 da Liga dos
/// Campeões, a cada jogo dela). O placar: +0x47/+0x48 (com +0x47 = −1, +0x43/+0x44), com sinal. O vencedor ganha 2
/// pontos no coeficiente da sua nação (+0x53; 0x9058b0), o empate 1 para cada. Na fase de grupos (+0x42 de 0 a 11):
/// 200 mil ao vencedor ou 100 mil a cada um no empate (0x5a0590 + 0x5999a0), com a notícia do prêmio (0x48e3e0;
/// no empate, as duas somas antes das duas notícias). Depois a base da copa (0x51a150 com o jogo e `arg`).
pub(crate) unsafe extern "thiscall" fn process_fixture(this: usize, jogo: usize, arg: u32) {
    let (ga, gb) = if read_u8(jogo + 0x47) != 0xff {
        (read_u8(jogo + 0x47) as i8, read_u8(jogo + 0x48) as i8)
    } else {
        (read_u8(jogo + 0x43) as i8, read_u8(jogo + 0x44) as i8)
    };
    let (a, b) = (read_usize(jogo + 0x1c), read_usize(jogo + 0x20));
    let (vencedor, perdedor) = if ga > gb {
        add_uefa_coefficient(read_usize(a + 0x53), 2);
        (read_usize(jogo + 0x1c), read_usize(jogo + 0x20))
    } else if ga < gb {
        add_uefa_coefficient(read_usize(b + 0x53), 2);
        (read_usize(jogo + 0x20), read_usize(jogo + 0x1c))
    } else {
        add_uefa_coefficient(read_usize(a + 0x53), 1);
        add_uefa_coefficient(read_usize(read_usize(jogo + 0x20) + 0x53), 1);
        (0, 0)
    };
    let _ = perdedor;
    let fase = read_u8(jogo + 0x42) as i8;
    if fase > -1 && fase < 0xc {
        if vencedor != 0 {
            crate::comparacao::ramo("cpy_6 premio da vitoria");
            european_cup_add_prize_money(european_cup_club_finance(vencedor), 0x30d40); // cpy6: o prêmio da vitória
            european_cup_send_prize_money_news(this, vencedor, 0x30d40, jogo);
        } else {
            crate::comparacao::ramo("cpy_6 premio do empate");
            european_cup_add_prize_money(european_cup_club_finance(read_usize(jogo + 0x1c)), 0x186a0); // cpy6: o prêmio do empate do mandante
            european_cup_add_prize_money(european_cup_club_finance(read_usize(jogo + 0x20)), 0x186a0);
            european_cup_send_prize_money_news(this, read_usize(jogo + 0x1c), 0x186a0, jogo);
            european_cup_send_prize_money_news(this, read_usize(jogo + 0x20), 0x186a0, jogo);
        }
    }
    if CPY6_ENSAIO {
        return;
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_CUP_PROCESS_FIXTURE);
    base(this, jogo, arg);
}

/// Address of the club nation's UEFA coefficient word ([G_UEFA_SEEDING] + 18 x national-team +0x1df + 0xf); 0 if
/// missing.
///
/// [pt] A palavra do coeficiente da nação do clube: [0xde1f58] + 18 × (+0x1df da seleção 0x53b3d0 da nação +0x53) + 0xf,
/// como a 0x9058b0 calcula; 0 sem tabela, sem nação ou sem seleção.
pub(crate) unsafe fn nation_coefficient_word(clube: usize) -> usize {
    let tabela = read_usize(G_UEFA_SEEDING);
    let nacao = read_usize(clube + club::NATION);
    if tabela == 0 || nacao == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let s = selecao(nacao);
    if s == 0 {
        return 0;
    }
    tabela.wrapping_add((read_u8(s + 0x1df) as i8 as isize * 18) as usize + 0xf)
}

/// Comparador da 0x583b10, em ensaio: a cópia sem a notícia e sem a base, e a original inteira, a partir do mesmo
/// estado (as palavras do coeficiente das duas nações e os registros de finanças dos dois clubes, restaurados depois
/// do ensaio). Confere as palavras do coeficiente, que a base não toca, salvo pelo sorteio das fases (0x585290,
/// 0x585ab0: o coeficiente dos classificados), que só roda quando a fase (+0x30) anda: aí, sem comparação. As
/// finanças não entram (a base paga o prêmio da rodada da copa nos mesmos campos, 0x51a2fe); o prêmio segue o ramo do
/// placar que os pontos do coeficiente já conferem. Sem clube, nação, seleção ou finanças: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_fixture(this: usize, jogo: usize, arg: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_FIXTURE);
    let (a, b) = if jogo != 0 { (read_usize(jogo + 0x1c), read_usize(jogo + 0x20)) } else { (0, 0) };
    if a == 0 || b == 0 {
        return o(this, jogo, arg); // ponytail: sem clube, só a original
    }
    let (ca, cb) = (nation_coefficient_word(a), nation_coefficient_word(b));
    let (fa, fb) = (european_cup_club_finance(a), european_cup_club_finance(b));
    if ca == 0 || cb == 0 || fa == 0 || fb == 0 {
        return o(this, jogo, arg); // ponytail: sem nação, seleção ou finanças, só a original (cai do mesmo jeito)
    }
    let conferir = [(ca, 2), (cb, 2)];
    let antes = crate::comparacao::Memoria::ler(&[(ca, 2), (cb, 2), (fa, 0x167), (fb, 0x167)]);
    let fase = read_i32(this + 0x30);
    CPY6_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| process_fixture(this, jogo, arg)));
    CPY6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&conferir);
    antes.escrever();
    o(this, jogo, arg);
    if read_i32(this + 0x30) != fase {
        return; // ponytail: a fase andou na base e o sorteio somou coeficiente: sem comparação
    }
    let mo = crate::comparacao::Memoria::ler(&conferir);
    PLACAR.registrar(mo == mr, || format!("process_fixture(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// converts the three progress codes by stage (qualifying, phase 1, phase 2) and writes them to +0x73..+0x75 of the
/// club's competition stats entry
/// Original: `EuropeanCup::set_club_progress` (0x586c00, european_cup.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x586c00 } (thiscall, ret 0x14, european_cup.cpp): `EuropeanCup::set_club_progress(clube, fase, p1, p2, p3)` (a
/// virtual +0x68, da base da copa 0x521b30): converte os três códigos de progresso pela fase e grava no registro do
/// clube na estatística da competição (0x4a4850 de +8, sem conferir o nulo, como a original) +0x73 ← p1, +0x74 ← p2,
/// +0x75 ← p3. Fase −1 (a qualificação): 1 → 0x19 (p2: 1), 17 → 0x21, 33 → 0x31, 47 → 0x3f; fase abaixo de 8 (com
/// sinal): 1 e 2 → 1 (p3: 0xd), 3 → 0x11, 4 → 0x19; abaixo de 12: 1 e 2 → 1 (p3: 7), 3 → 9, 4 → 0xd; os outros
/// códigos e as fases de 12 em diante passam como estão.
pub(crate) unsafe extern "thiscall" fn european_cup_set_club_progress(this: usize, clube: usize, fase: u32, p1: u32, p2: u32, p3: u32) {
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = buscar(read_usize(this + 8), clube);
    let (fase, p1, p2, p3) = (fase as u8 as i8, p1 as u8, p2 as u8, p3 as u8);
    let (c73, c74, c75) = if fase == -1 {
        let m = |p: u8, um: u8| match p {
            1 => um,
            17 => 0x21,
            33 => 0x31,
            47 => 0x3f,
            _ => p,
        };
        (m(p1, 0x19), m(p2, 1), m(p3, 0x19))
    } else if fase < 8 {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 0x11,
            4 => 0x19,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 0xd))
    } else if fase < 0xc {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 9,
            4 => 0xd,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 7))
    } else {
        (p1, p2, p3)
    };
    *((e + 0x74) as *mut u8) = c74;
    *((e + 0x73) as *mut u8) = c73;
    *((e + 0x75) as *mut u8) = c75;
}

/// Comparador da 0x586c00: lado a lado nos três bytes do registro do clube (+0x73..+0x75); sem registro (a original
/// cai), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_set_club_progress(this: usize, clube: usize, fase: u32, p1: u32, p2: u32, p3: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_EUROPEAN_CUP_SET_CLUB_PROGRESS);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = buscar(read_usize(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, p1, p2, p3); // ponytail: sem registro, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &[(e + 0x73, 3)],
        || o(this, clube, fase, p1, p2, p3),
        || european_cup_set_club_progress(this, clube, fase, p1, p2, p3),
    );
    PLACAR.registrar(igual, || format!("european_cup_set_club_progress(fase {} {} {} {}) original={:x?} rust={:x?}", fase as u8 as i8, p1 as u8, p2 as u8, p3 as u8, mo, mr));
}

/// in the group stages, the 6-round schedule (phase 1 dates in September and October and phase 2 from November to
/// March, by half) with fixture 0 of each round empty; the qualifying rounds and the knockout stay with the original
/// Original: `EuropeanCup::create_round_table` (0x583c50, european_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x583c50 } (thiscall, ret 0x10, european_cup.cpp, método +0x3c da Liga dos Campeões, quadro SEH pelas datas locais
/// sem destrutor no caminho normal): `EuropeanCup::create_round_table(fase, &rodadas, &id, &é_liga)`, dos sorteios dos
/// grupos (0x585290, 0x585ab0) e das posições previstas (0x586900). Nas fases de grupos (char 0..11, e as negativas
/// abaixo de −1, que caem no mesmo ramo): é_liga (se pedido) = 1, rodadas = 6, id = 0x43d + fase; a tabela de 6 rodadas
/// de 0x41 bytes (0x944e46(0x186); sem memória, a caixa da linha 0x17a ou 0x1aa e devolve 0); na segunda fase, antes, a
/// data de 1º de março do ano seguinte levada à terça (0x549f70), um dia antes (0x54c8e0, que grava a data estática dela)
/// e a cópia (0x417c10), sem uso depois (a cópia as chama do mesmo jeito); as datas de `CQH4_DATAS` (0x68a160 com o ano
/// +0x40 da copa) e o jogo 0 de cada rodada vazio (0x68a1c0 com −1). As preliminares (−1) e o mata-mata (12 em diante):
/// a original pelo trampolim. Devolve a tabela.
pub(crate) unsafe extern "thiscall" fn european_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUROPEAN_CUP_CREATE_ROUND_TABLE)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 6;
    *id = (f as i16).wrapping_add(0x43d);
    let t = alocar(0x186);
    if t == 0 {
        show_error_at(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
        return 0;
    }
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as u16 as u32;
    let metade = if f < 8 {
        if f < 4 { 0 } else { 1 }
    } else {
        let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
        let menos: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
        let copia: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let (mut d, mut t1, mut t2) = ([0u16; 4], [0u16; 4], [0u16; 4]);
        // ponytail: o ano + 1 vai no ax com o resto do eax nos bytes altos; a 0x549f70 lê o ano como word
        construir(&mut d, 1, 2, (ano as u16).wrapping_add(1) as u32, 2);
        let r = menos(&mut d, &mut t1, 1);
        copia(&mut d, &mut t2, r);
        if f < 10 { 2 } else { 3 }
    };
    crate::comparacao::ramo("cqh_4 tabela das rodadas");
    for &(i, dia, mes, anos, semana) in CQH4_DATAS[metade].iter() {
        data(t, i, dia, mes, anos, semana, 2, ano, 0);
    }
    for r in 0..6 {
        jogo(t, r, 0, -1, -1, -1, 0);
    }
    t
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). As
/// preliminares e o mata-mata: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUROPEAN_CUP_CREATE_ROUND_TABLE);
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || european_cup_create_round_table(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { league_written_table_bytes(a) == league_written_table_bytes(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("european_cup_create_round_table({}): saídas {} tabela {}", f, saidas, tabela));
    a
}
