// ---------- 272a leva: a Liga dos Campeões: o sorteio dos grupos da primeira fase, a ordem dos potes e as posições previstas (european_cup.cpp) ----------

unsafe fn cqg4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqg4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqg4_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqg4_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// A nação do clube (+0x53).
unsafe fn cqg4_nacao(clube: usize) -> usize {
    cqg4_d(clube + 0x53)
}

/// { 0x585290 } (thiscall, ret 0, european_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
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
unsafe extern "thiscall" fn campeoes_sortear_grupos(this: usize) {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let pontos: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x9058b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia_do_premio: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let noticia_do_grupo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let regras_a: [u8; 4] = [4, 1, 2, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let lista = zerada(0x20, 9);
    if lista == 0 {
        aviso_em(0x9c4900, 0x4cc);
        return;
    }
    let ent = |i: i32| lista.wrapping_add((i * 9) as usize);
    let mut n = 0i16;
    for k in 0..16usize {
        let e = ent(n as i32);
        cqg4_pd(e, cqg4_d(cqg4_d(this + 0x14) + k * 6));
        cqg4_pd(e + 4, 0);
        n += 1;
    }
    let mut k = 0i32;
    while k < cqg4_w(this + 0x3e) as i32 {
        let t = cqg4_d(this + 0xa7) + k as usize * 6;
        if cqg4_b(t + 5) == 1 {
            let e = ent(n as i32);
            cqg4_pd(e, cqg4_d(t));
            cqg4_pd(e + 4, 0);
            n += 1;
        }
        k += 1;
    }
    if n < 0x20 {
        aviso_em(0x9c4900, 0x4ee);
        cqg4_pd(this + 0x30, usize::MAX);
        soltar(lista);
        cqg4_pb(this + 0xb1, 0);
        return;
    }
    ordenar(lista, 0x20, 9, 0x586f60);
    for j in 0..0x20i32 {
        let e = ent(j);
        if cqg4_d(e + 4) != 0 || j + 1 >= 0x20 {
            continue;
        }
        for i in (j + 1)..0x20 {
            let o = ent(i);
            if cqg4_d(o + 4) == 0 && cqg4_nacao(cqg4_d(e)) == cqg4_nacao(cqg4_d(o)) {
                crate::comparacao::ramo("cqg_4 parceiro da mesma nação");
                cqg4_pd(e + 4, cqg4_d(o)); // cqg4: o parceiro da mesma nação
                cqg4_pd(o + 4, cqg4_d(e));
                break;
            }
        }
    }
    let grupos = zerada(0x20, 4);
    if grupos == 0 {
        erro_em(0x9c4900, 0x515);
    }
    let vaga = |g: i32, pote: i32| grupos + ((pote + g * 4) * 4) as usize;
    let mut pote = 0i32;
    let mut base = 0i32;
    while base < 0x20 {
        let (mut livres_b, mut livres_a) = (4i8, 4i8); // [esp+0x12] a metade 2, [esp+0x13] a metade 1
        for i in 0..8 {
            match cqg4_b(ent(base + i) + 8) {
                2 => livres_b -= 1,
                1 => livres_a -= 1,
                _ => {}
            }
        }
        for _ in 0..8 {
            let mut s = sorteio(8).wrapping_add(base); // cqg4: o clube sorteado do pote
            while cqg4_d(ent(s)) == 0 {
                s = sorteio(8).wrapping_add(base);
            }
            crate::comparacao::ramo("cqg_4 clube sorteado");
            let e = ent(s);
            if cqg4_b(e + 8) == 0 {
                crate::comparacao::ramo("cqg_4 metade sorteada");
                let b = livres_b as i32;
                if sorteio(livres_a as i32 + b) < b { // cqg4: a metade sorteada
                    cqg4_pb(e + 8, 2);
                    livres_b -= 1;
                } else {
                    cqg4_pb(e + 8, 1);
                    livres_a -= 1;
                }
                let par = cqg4_d(e + 4);
                if par != 0 {
                    if let Some(i) = (0..0x20).find(|&i| cqg4_d(ent(i)) == par) {
                        let mesmo_pote = i / 8 == s / 8;
                        let marca = if mesmo_pote {
                            let m = cqg4_b(e + 8);
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
                            cqg4_pb(ent(i) + 8, if cqg4_b(e + 8) != 2 { 2 } else { 1 });
                        }
                    }
                }
            }
            let ini: i32 = if cqg4_b(e + 8) == 1 { 0 } else { 4 };
            for g in ini..ini + 4 {
                if cqg4_d(vaga(g, pote)) == 0 {
                    cqg4_pd(vaga(g, pote), cqg4_d(e));
                    cqg4_pd(e, 0);
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
                let nacao = cqg4_nacao(cqg4_d(vaga(g, p)));
                let repetida = (0..4).any(|q| q != p && nacao == cqg4_nacao(cqg4_d(vaga(g, q))));
                if repetida {
                    for g2 in ini..fim {
                        if g2 == g {
                            continue;
                        }
                        if (0..4).all(|q| cqg4_nacao(cqg4_d(vaga(g, p))) != cqg4_nacao(cqg4_d(vaga(g2, q)))) {
                            crate::comparacao::ramo("cqg_4 troca pela nação");
                            let x = cqg4_d(vaga(g, p));
                            cqg4_pd(vaga(g, p), cqg4_d(vaga(g2, p)));
                            cqg4_pd(vaga(g2, p), x);
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
            let c = cqg4_d(linha + q * 4);
            pontos(cqg4_d(0xde1f58), cqg4_nacao(c), 1);
            let valor = 1_800_000i32; // cqg4: o prêmio da fase de grupos
            crate::comparacao::ramo("cqg_4 prêmio da fase de grupos");
            premio(financas(cqg4_d(0xae2a38), c), valor);
            noticia_do_premio(this, c, valor, 0, -1, 0x449, 0, -2);
        }
        let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqg4_d(cqg4_d(this) + 0x3c));
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
                cqg4_d(this + 4),
                tabela,
                rodadas as u16 as u32,
                3,
                1,
                8,
                regras_a.as_ptr(),
                regras_b.as_ptr(),
                cqg4_w(this + 0x40) as u16 as u32,
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
        cqg4_pd(cqg4_d(this + 0xc) + g as usize * 4, fase);
        if cqg4_d(cqg4_d(this + 0xc) + g as usize * 4) == 0 {
            aviso_em(0x9c4900, 0x5c9);
            return;
        }
        soltar(tabela);
        cqg4_pd(this + 0x30, g as usize);
        noticia_do_grupo(cqg4_d(cqg4_d(this + 0xc) + g as usize * 4));
    }
    soltar(grupos);
    let uefa = cqg4_d(cqg4_d(0xadadfc) + cqg4_d(0x9cf6f0) * 4);
    if uefa == 0 {
        aviso_em(0x9c4900, 0x5d8);
        return;
    }
    let mut destino = ((cqg4_w(uefa + 0x3e) as i32 - 0x18) * 6) as usize;
    let mut k = 0i32;
    while k < cqg4_w(this + 0x3e) as i32 {
        let t = cqg4_d(this + 0xa7) + k as usize * 6;
        if cqg4_b(t + 5) == 2 {
            let e = cqg4_d(uefa + 0xa7).wrapping_add(destino);
            destino = destino.wrapping_add(6);
            cqg4_pd(e, cqg4_d(t));
            cqg4_pb(e + 4, 0);
            cqg4_pb(e + 5, 0);
            let n = cqg4_w(uefa + 0x36) as i32;
            cqg4_pd(cqg4_d(uefa + 0x14) + (n * 6) as usize, cqg4_d(e));
            let n = cqg4_w(uefa + 0x36) as i32;
            cqg4_pb(cqg4_d(uefa + 0x14) + (n * 6) as usize + 4, 5);
            cqg4_pw(uefa + 0x36, (cqg4_w(uefa + 0x36) as u16).wrapping_add(1));
            cqg4_pd(cqg4_d(e) + 0x1db, cqg4_d(0x9cf6f0));
        }
        k += 1;
    }
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqg4_d(cqg4_d(uefa) + 0x5c));
    previstas(uefa);
}

/// Os 32 clubes que o sorteio põe nos grupos: os 16 primeiros de [+0x14] e os de +0xa7 com +5 = 1.
unsafe fn cqg4_plano(this: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..16usize).map(|k| cqg4_d(cqg4_d(this + 0x14) + k * 6)).collect();
    for k in 0..cqg4_w(this + 0x3e).max(0) as usize {
        let t = cqg4_d(this + 0xa7) + k * 6;
        if cqg4_b(t + 5) == 1 {
            v.push(cqg4_d(t));
        }
    }
    v.sort_unstable();
    v
}

/// Só a original (sorteia, paga prêmios, aloca as fases e manda notícias), conferindo o plano feito antes: com 32
/// clubes, as 8 fases em [+0xc], +0x30 = 7 e os clubes das tabelas das fases (+0xb1, linhas de 0x3b bytes, 4 por
/// fase) são exatamente os 32 do plano.
unsafe extern "thiscall" fn comparar_campeoes_sortear_grupos(this: usize) {
    let plano = cqg4_plano(this);
    orig::<unsafe extern "thiscall" fn(usize)>(1646)(this);
    if plano.len() != 32 || cqg4_d(cqg4_d(0xadadfc) + cqg4_d(0x9cf6f0) * 4) == 0 {
        return;
    }
    let fases = cqg4_d(this + 0xc);
    let mut vistos = Vec::with_capacity(32);
    let mut ok = cqg4_d(this + 0x30) == 7;
    for g in 0..8usize {
        let f = cqg4_d(fases + g * 4);
        if f == 0 || cqg4_d(f + 0xb1) == 0 {
            ok = false;
            break;
        }
        for r in 0..4usize {
            vistos.push(cqg4_d(cqg4_d(f + 0xb1) + r * 0x3b));
        }
    }
    vistos.sort_unstable();
    ok = ok && vistos == plano;
    PLACAR.registrar(ok, || format!("campeoes_sortear_grupos: as fases não têm os 32 clubes do plano (+0x30 = {})", cqg4_d(this + 0x30) as i32));
}

/// { 0x586f60 } (cdecl, european_cup.cpp): `european_cup_compare_clubs_by_seed(a, b)`, a comparação da qsort dos potes
/// do sorteio (0x585290): o byte +0x1df (a semente europeia) de a menos o de b; empate: a reputação (+0x80, word) de b
/// menos a de a.
unsafe extern "C" fn campeoes_ordem_dos_potes(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    let r = cqg4_b(x + 0x1df) as i8 as i32 - cqg4_b(y + 0x1df) as i8 as i32;
    if r != 0 {
        return r;
    }
    cqg4_w(y + 0x80) as i32 - cqg4_w(x + 0x80) as i32
}

/// Pura: a devolução.
unsafe extern "C" fn comparar_campeoes_ordem_dos_potes(a: *const usize, b: *const usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(1647)(a, b);
    let n = campeoes_ordem_dos_potes(a, b);
    PLACAR.registrar(r == n, || format!("campeoes_ordem_dos_potes: original={} rust={}", r, n));
    r
}

unsafe fn cqg4p_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg4p_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// As posições que a 0x586900 grava, na ordem das chamadas à 0x4a2540: (índice do clube no vetor ordenado, posição).
/// `mata` é a tabela das rodadas do mata-mata (0x68 bytes por rodada, +0x1a a conta de confrontos) com `nm` rodadas;
/// as preliminares vêm de [+0xa3] com +0x3c rodadas.
/// 1) O mata-mata, da final para trás: na final, os da primeira volta ganham a posição 1 (o contador sobe a cada um);
///    em toda rodada, cada um dos seguintes ganha o contador do começo da rodada (que sobe a cada um).
/// 2) A segunda fase: 2 × (4 clubes com o contador; o contador sobe 4). 3) A primeira fase: 2 × (8 clubes; sobe 8).
/// 4) As preliminares, da última para a primeira: cada clube da rodada com o contador do começo dela (sobe a cada um).
unsafe fn cqg4p_plano(this: usize, mata: usize, nm: i16) -> (Vec<(i16, u8)>, usize) {
    let mut v = Vec::new();
    let mut i = 0i16;
    let mut cont = 1u8;
    let mut r = nm.wrapping_sub(1);
    while r >= 0 {
        let n = cqg4p_w(mata + r as usize * 0x68 + 0x1a);
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
        while k < cqg4p_w(mata + r as usize * 0x68 + 0x1a) {
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
    let mut r = cqg4p_w(this + 0x3c).wrapping_sub(1);
    while r >= 0 {
        let p = cont;
        let mut k = 0i16;
        while k < cqg4p_w(cqg4p_d(this + 0xa3) + r as usize * 0x68 + 0x1a) {
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

/// { 0x586900 } (thiscall, ret 0, european_cup.cpp, método +0x5c da Liga dos Campeões): `EuropeanCup::rank_teams()`, as
/// posições previstas. Sem estatística (+8): a caixa da linha 0x81b. Os inscritos da tabela [+0x14] (6 bytes, +0x36
/// deles) num vetor (0x944e46; sem memória, a caixa da linha 0x823), ordenado pela força (0x4c08f0: push n; push vetor;
/// add esp, 8 -> cdecl); a tabela do mata-mata pelo método +0x3c (push 0; push &id; push &rodadas; push 0xc; mov ecx ->
/// thiscall, ret 0x10), liberada depois; a posição de cada um, na ordem do vetor, pela 0x4a2540 (push posição; push
/// clube; mov ecx, [+8] -> thiscall, ret 8; a original empurra o dword da pilha com a posição no byte de baixo e lixo
/// nos outros, e a 0x4a2540 só grava o byte: a cópia passa o byte limpo) no plano de `cqg4p_plano`; o vetor liberado.
unsafe extern "thiscall" fn campeoes_posicoes_previstas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqg4p_d(this + 8) == 0 {
        aviso_em(0x9c4900, 0x81b);
        return;
    }
    let v = alocar(((cqg4p_w(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        aviso_em(0x9c4900, 0x823);
        return;
    }
    let mut i = 0i16;
    while i < cqg4p_w(this + 0x36) {
        core::ptr::write_unaligned((v + i as usize * 4) as *mut u32, cqg4p_d(cqg4p_d(this + 0x14) + i as usize * 6) as u32);
        i += 1;
    }
    ordenar(v, cqg4p_w(this + 0x36) as i32);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqg4p_d(cqg4p_d(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let mata = rodadas_da_copa(this, 0xc, &mut nm, &mut id, 0);
    // as posições do mata-mata antes de liberar a tabela dele, as outras depois (como a original)
    let (plano, n_mata) = cqg4p_plano(this, mata, nm);
    for (j, &(k, p)) in plano.iter().enumerate() {
        if j == n_mata {
            soltar(mata);
        }
        posicao(cqg4p_d(this + 8), cqg4p_d(v + k as usize * 4), p as u32);
    }
    if n_mata == plano.len() {
        soltar(mata);
    }
    soltar(v);
}

/// Só a original (acrescenta registros à estatística), conferindo o plano feito antes (o vetor ordenado numa cópia e a
/// tabela do mata-mata pedida ao método +0x3c e liberada): a posição (+0x72 do registro achado por 0x4a4850) de cada
/// clube do plano.
unsafe extern "thiscall" fn comparar_campeoes_posicoes_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1648);
    if cqg4p_d(this + 8) == 0 || cqg4p_d(this + 0x14) == 0 || cqg4p_w(this + 0x36) <= 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut u32, i32) = core::mem::transmute(0x4c08f0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let n = cqg4p_w(this + 0x36) as usize;
    let mut v: Vec<u32> = (0..n).map(|i| cqg4p_d(cqg4p_d(this + 0x14) + i * 6) as u32).collect();
    ordenar(v.as_mut_ptr(), n as i32);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqg4p_d(cqg4p_d(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let mata = rodadas_da_copa(this, 0xc, &mut nm, &mut id, 0);
    let plano = if mata != 0 { cqg4p_plano(this, mata, nm).0 } else { Vec::new() };
    soltar(mata);
    o(this);
    if plano.iter().any(|&(k, _)| k as usize >= n) {
        return; // a original lê além do vetor: sem conferência
    }
    let errado = plano.iter().find(|&&(k, p)| {
        let c = v[k as usize] as usize;
        c != 0 && {
            let e = achar(cqg4p_d(this + 8), c);
            e == 0 || *((e + 0x72) as *const u8) != p
        }
    });
    PLACAR.registrar(errado.is_none(), || format!("campeoes_posicoes_previstas: a posição {:?} não bate", errado));
}

// ---------- 272a leva: a Liga dos Campeões: o sorteio dos grupos da segunda fase e a ordem dos classificados; o grupo de uma fase (european_cup.cpp, comp_util.cpp) ----------

unsafe fn cqg6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqg6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqg6_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqg6_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// A nação do clube (+0x53).
unsafe fn cqg6_nacao(clube: usize) -> usize {
    cqg6_d(clube + 0x53)
}

/// Copia a entrada de 7 bytes `de` (+0 o clube, +4 os da mesma nação, +5 o grupo da primeira fase, +6 0 vencedor / 1
/// segundo) para `para`, como a original: o dword, o word +4 e o byte +6.
unsafe fn cqg6_copiar(para: usize, de: usize) {
    cqg6_pd(para, cqg6_d(de));
    cqg6_pw(para + 4, cqg6_w(de + 4) as u16);
    cqg6_pb(para + 6, cqg6_b(de + 6));
}

/// { 0x585ab0 } (thiscall, ret 4, european_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
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
unsafe extern "thiscall" fn campeoes_sortear_segunda_fase(this: usize, tentativa: u32) {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x6826d0usize);
    let cartoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e9180usize);
    let pontos: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x9058b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia_do_premio: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let noticia_do_grupo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let regras_a: [u8; 4] = [4, 1, 2, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let w = zerada(0x10, 7);
    if w == 0 {
        aviso_em(0x9c4900, 0x61c);
        return;
    }
    let e7 = |l: usize, i: i32| l.wrapping_add((i * 7) as usize);
    for b in 0..8i32 {
        let fase = || cqg6_d(cqg6_d(this + 0xc) + b as usize * 4);
        let e = e7(w, 2 * b);
        cqg6_pd(e, cqg6_d(linha(fase(), 0)));
        cqg6_pb(e + 6, 0);
        cqg6_pb(e + 5, b as u8);
        cqg6_pb(e + 4, 0);
        let e = e7(w, 2 * b + 1);
        cqg6_pd(e, cqg6_d(linha(fase(), 1)));
        cqg6_pb(e + 6, 1);
        cqg6_pb(e + 5, b as u8);
        cqg6_pb(e + 4, 0);
    }
    for i in 0..16 {
        for j in 0..16 {
            if cqg6_nacao(cqg6_d(e7(w, i))) == cqg6_nacao(cqg6_d(e7(w, j))) {
                cqg6_pb(e7(w, i) + 4, cqg6_b(e7(w, i) + 4).wrapping_add(1));
            }
        }
    }
    ordenar(w, 0x10, 7, 0x5871e0);
    for i in 0..16 {
        let orgao = cqg6_d(cqg6_d(cqg6_d(0xae28f0)).wrapping_add((cqg6_b(this + 0x50) as i8 as i32 * 4) as usize));
        cartoes(orgao, cqg6_d(e7(w, i)));
    }
    let r = zerada(0x10, 7);
    if r == 0 {
        aviso_em(0x9c4900, 0x649);
        return;
    }
    let g = zerada(0x10, 7);
    if g == 0 {
        aviso_em(0x9c4900, 0x651);
        return;
    }
    let mut k = 0i8;
    for i in 0..16i32 {
        let e = e7(w, i);
        if cqg6_d(e) == 0 {
            continue;
        }
        let c = k as i32;
        cqg6_copiar(e7(r, if cqg6_b(e + 6) != 0 { c + 1 } else { c }), e);
        cqg6_pd(e, 0);
        for a in 0..16i32 {
            if i != a && cqg6_b(e + 5) == cqg6_b(e7(w, a) + 5) {
                let o = e7(w, a);
                let destino = if cqg6_d(e7(r, c)) != 0 { e7(r, c + 1) } else { e7(r, c) };
                crate::comparacao::ramo("cqg_6 par do mesmo grupo");
                cqg6_copiar(destino, o); // cqg6: o outro do grupo
                k = k.wrapping_add(2);
                cqg6_pd(o, 0);
                break;
            }
        }
    }
    let mut ordem = [0u8, 1, 2, 3];
    for i in 0..16i32 {
        let p = e7(r, i);
        let mut menos = 4i8;
        let mut melhor = 0xffu8;
        let s: i32 = if cqg6_b(p + 6) != 0 { 2 } else { 0 };
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
                let clube = cqg6_d(v);
                if clube != 0 {
                    n += 1;
                    if cqg6_b(v + 5) == cqg6_b(p + 5) || cqg6_nacao(clube) == cqg6_nacao(cqg6_d(p)) { // cqg6: mesmo grupo ou nação
                        conflito = true;
                        break;
                    }
                }
            }
            let vaga = cqg6_d(e7(g, s + gr * 4)) == 0 || cqg6_d(e7(g, s + 1 + gr * 4)) == 0;
            if vaga && !conflito && n < menos {
                menos = n;
                melhor = ordem[q];
            }
        }
        if melhor != 0xff {
            crate::comparacao::ramo("cqg_6 grupo escolhido");
            let base = melhor as i8 as i32 * 4;
            let v = if cqg6_d(e7(g, s + base)) == 0 { e7(g, s + base) } else { e7(g, s + 1 + base) };
            cqg6_copiar(v, p);
            cqg6_pd(p, 0);
        }
    }
    for i in 0..16i32 {
        let p = e7(r, i);
        if cqg6_d(p) == 0 {
            continue;
        }
        if (tentativa as u8 as i8) < 0x14 {
            crate::comparacao::ramo("cqg_6 sorteio de novo");
            soltar(g);
            soltar(r);
            soltar(w);
            let de_novo: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x585ab0usize);
            de_novo(this, (tentativa as u8).wrapping_add(1) as u32);
            return;
        }
        if let Some(v) = (0..16).map(|j| e7(g, j)).find(|&v| cqg6_d(v) == 0) {
            cqg6_copiar(v, p);
        }
    }
    let t = alocar(0x40);
    for i in 0..16usize {
        cqg6_pd(t + i * 4, cqg6_d(e7(g, i as i32)));
    }
    for h in 0..4i32 {
        for q in 0..4i32 {
            let c = cqg6_d(e7(g, h * 4 + q));
            pontos(cqg6_d(0xde1f58), cqg6_nacao(c), 1);
            let valor = 1_200_000i32; // cqg6: o prêmio da segunda fase
            crate::comparacao::ramo("cqg_6 prêmio da segunda fase");
            premio(financas(cqg6_d(0xae2a38), c), valor);
            noticia_do_premio(this, c, valor, 0, -1, 0x44a, 0, -2);
        }
        let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqg6_d(cqg6_d(this) + 0x3c));
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
                cqg6_d(this + 4),
                tabela,
                rodadas as u16 as u32,
                3,
                1,
                8,
                regras_a.as_ptr(),
                regras_b.as_ptr(),
                cqg6_w(this + 0x40) as u16 as u32,
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
        let slot = cqg6_d(this + 0xc) + (h + 8) as usize * 4;
        cqg6_pd(slot, fase);
        if cqg6_d(cqg6_d(this + 0xc) + (h + 8) as usize * 4) == 0 {
            aviso_em(0x9c4900, 0x715);
            return;
        }
        soltar(tabela);
        cqg6_pd(this + 0x30, (h + 8) as usize);
        noticia_do_grupo(cqg6_d(cqg6_d(this + 0xc) + (h + 8) as usize * 4));
    }
    soltar(w);
    soltar(g);
    soltar(r);
    soltar(t);
    let uefa = cqg6_d(cqg6_d(0xadadfc) + cqg6_d(0x9cf6f0) * 4);
    if uefa == 0 {
        aviso_em(0x9c4900, 0x729);
        return;
    }
    let mut destino = ((cqg6_w(uefa + 0x3e) as i32 - 1) * 6) as usize;
    for b in 0..8usize {
        let e = cqg6_d(uefa + 0xa7).wrapping_add(destino);
        destino = destino.wrapping_sub(6);
        let terceiro = cqg6_d(linha(cqg6_d(cqg6_d(this + 0xc) + b * 4), 2));
        cqg6_pb(e + 4, 0);
        cqg6_pd(e, terceiro);
        cqg6_pb(e + 5, 0);
        let n = cqg6_w(uefa + 0x36) as i32;
        cqg6_pd(cqg6_d(uefa + 0x14) + (n * 6) as usize, cqg6_d(e));
        let n = cqg6_w(uefa + 0x36) as i32;
        cqg6_pb(cqg6_d(uefa + 0x14) + (n * 6) as usize + 4, 5);
        cqg6_pw(uefa + 0x36, (cqg6_w(uefa + 0x36) as u16).wrapping_add(1));
        cqg6_pd(cqg6_d(e) + 0x1db, cqg6_d(0x9cf6f0));
        let quarto = cqg6_d(linha(cqg6_d(cqg6_d(this + 0xc) + b * 4), 3));
        cqg6_pd(quarto + 0x1db, usize::MAX);
    }
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqg6_d(cqg6_d(uefa) + 0x5c));
    previstas(uefa);
}

/// Os 16 clubes que vão para a segunda fase (o primeiro e o segundo de cada grupo da primeira).
unsafe fn cqg6_plano(this: usize) -> Vec<usize> {
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x6826d0usize);
    let mut v = Vec::with_capacity(16);
    for b in 0..8usize {
        let f = cqg6_d(cqg6_d(this + 0xc) + b * 4);
        v.push(cqg6_d(linha(f, 0)));
        v.push(cqg6_d(linha(f, 1)));
    }
    v.sort_unstable();
    v
}

/// Só a original (sorteia, paga prêmios, aloca as fases e manda notícias; e se chama de novo), conferindo o plano: na
/// chamada de fora (tentativa 0), as fases 8..11 existem, +0x30 = 11 e os clubes das tabelas delas (+0xb1, linhas de
/// 0x3b bytes) são os 16 classificados.
unsafe extern "thiscall" fn comparar_campeoes_sortear_segunda_fase(this: usize, tentativa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1649);
    if tentativa as u8 != 0 || cqg6_d(cqg6_d(0xadadfc) + cqg6_d(0x9cf6f0) * 4) == 0 {
        return o(this, tentativa);
    }
    let plano = cqg6_plano(this);
    o(this, tentativa);
    let mut vistos = Vec::with_capacity(16);
    let mut ok = cqg6_d(this + 0x30) == 11;
    for h in 8..12usize {
        let f = cqg6_d(cqg6_d(this + 0xc) + h * 4);
        if f == 0 || cqg6_d(f + 0xb1) == 0 {
            ok = false;
            break;
        }
        for r in 0..4usize {
            vistos.push(cqg6_d(cqg6_d(f + 0xb1) + r * 0x3b));
        }
    }
    vistos.sort_unstable();
    ok = ok && vistos == plano;
    PLACAR.registrar(ok, || format!("campeoes_sortear_segunda_fase: as fases não têm os 16 classificados (+0x30 = {})", cqg6_d(this + 0x30) as i32));
}

/// { 0x5871e0 } (cdecl, european_cup.cpp): `european_cup_compare_phase2_draw_entries(a, b)`, a comparação da qsort dos
/// classificados (0x585ab0): os da mesma nação (+4) de b menos os de a; empate: o word +0x8e da nação (+0x53 do clube) de
/// b menos o de a; empate: +6 (o segundo) de a menos o de b.
unsafe extern "C" fn campeoes_ordem_da_segunda_fase(a: usize, b: usize) -> i32 {
    let r = cqg6_b(b + 4) as i8 as i32 - cqg6_b(a + 4) as i8 as i32;
    if r != 0 {
        return r;
    }
    let r = cqg6_w(cqg6_nacao(cqg6_d(b)) + 0x8e) as i32 - cqg6_w(cqg6_nacao(cqg6_d(a)) + 0x8e) as i32;
    if r != 0 {
        return r;
    }
    cqg6_b(a + 6) as i8 as i32 - cqg6_b(b + 6) as i8 as i32
}

/// Pura: a devolução.
unsafe extern "C" fn comparar_campeoes_ordem_da_segunda_fase(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1650)(a, b);
    let n = campeoes_ordem_da_segunda_fase(a, b);
    PLACAR.registrar(r == n, || format!("campeoes_ordem_da_segunda_fase: original={} rust={}", r, n));
    r
}

/// { 0x4b01d0 } (cdecl, comp_util.cpp): `comp_util_get_parent_stage_id(id)`, o grupo de uma fase pelo id: 0x3e8..0x3fb,
/// 0x41f..0x42e, 0x413, 0x414, 0x47b e 0x3fc -> 0x41c; 0x473..0x474 -> 0x410; 0x475..0x479 -> 0x47a; 0x3fd..0x406 ->
/// 0xa; 0x407..0x40b e 0x472 -> 0x14; 0x47e..0x47f -> 0x1e; 0x40c..0x40e -> 0x82; 0x43d..0x444 -> 0x449; 0x445..0x448
/// -> 0x44a; 0x44d..0x450 -> 0x451; o resto −1 (na ordem dos testes da original).
unsafe extern "C" fn grupo_da_fase(id: i32) -> i32 {
    let em = |a: i32, b: i32| id >= a && id <= b;
    if em(0x3e8, 0x3fb) {
        return 0x41c;
    }
    if em(0x473, 0x474) {
        return 0x410;
    }
    if em(0x475, 0x479) {
        return 0x47a;
    }
    if em(0x3fd, 0x406) {
        crate::comparacao::ramo("cqg_6 grupo 0xa");
        return 0xa; // cqg6: o grupo das fases 0x3fd..0x406
    }
    if em(0x407, 0x40b) || id == 0x472 {
        return 0x14;
    }
    if em(0x47e, 0x47f) {
        return 0x1e;
    }
    if em(0x41f, 0x42e) {
        return 0x41c;
    }
    if em(0x40c, 0x40e) {
        return 0x82;
    }
    if id == 0x413 || id == 0x414 || id == 0x47b || id == 0x3fc {
        return 0x41c;
    }
    if em(0x43d, 0x444) {
        return 0x449;
    }
    if em(0x445, 0x448) {
        return 0x44a;
    }
    if em(0x44d, 0x450) {
        return 0x451;
    }
    -1
}

/// Pura: a devolução.
unsafe extern "C" fn comparar_grupo_da_fase(id: i32) -> i32 {
    let r = orig::<unsafe extern "C" fn(i32) -> i32>(1651)(id);
    let n = grupo_da_fase(id);
    PLACAR.registrar(r == n, || format!("grupo_da_fase({:#x}): original={:#x} rust={:#x}", id, r, n));
    r
}

// ---------- 272a leva: a Liga dos Campeões: a tabela das rodadas das fases de grupos; a liga: as expectativas da temporada e o rebaixamento de um clube (european_cup.cpp, league.cpp) ----------

/// As datas das 6 rodadas das fases de grupos (0x68a160: rodada, dia, mês 0..11, anos a somar, dia da semana; código 2,
/// valor 0), pela metade da fase: [0] primeira fase, grupos 0..3; [1] primeira fase, 4..7; [2] segunda fase, 8..9; [3]
/// segunda fase, 10..11.
const CQH4_DATAS: [[(i32, i32, i32, i32, i32); 6]; 4] = [
    [(0, 0xc, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // cqh4: a primeira fase, grupos A a D
    [(0, 0xd, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // cqh4: a primeira fase, grupos E a H
    [(0, 0x15, 0xa, 0, 1), (1, 6, 0xb, 0, 2), (2, 0xe, 1, 1, 2), (3, 0x14, 1, 1, 1), (4, 7, 2, 1, 2), (5, 0xd, 2, 1, 1)],
    [(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0xe, 2, 1, 2)], // cqh4: a segunda fase, grupos C e D
];

/// { 0x583c50 } (thiscall, ret 0x10, european_cup.cpp, método +0x3c da Liga dos Campeões, quadro SEH pelas datas locais
/// sem destrutor no caminho normal): `EuropeanCup::create_round_table(fase, &rodadas, &id, &é_liga)`, dos sorteios dos
/// grupos (0x585290, 0x585ab0) e das posições previstas (0x586900). Nas fases de grupos (char 0..11, e as negativas
/// abaixo de −1, que caem no mesmo ramo): é_liga (se pedido) = 1, rodadas = 6, id = 0x43d + fase; a tabela de 6 rodadas
/// de 0x41 bytes (0x944e46(0x186); sem memória, a caixa da linha 0x17a ou 0x1aa e devolve 0); na segunda fase, antes, a
/// data de 1º de março do ano seguinte levada à terça (0x549f70), um dia antes (0x54c8e0, que grava a data estática dela)
/// e a cópia (0x417c10), sem uso depois (a cópia as chama do mesmo jeito); as datas de `CQH4_DATAS` (0x68a160 com o ano
/// +0x40 da copa) e o jogo 0 de cada rodada vazio (0x68a1c0 com −1). As preliminares (−1) e o mata-mata (12 em diante):
/// a original pelo trampolim. Devolve a tabela.
unsafe extern "thiscall" fn campeoes_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1652)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 6;
    *id = (f as i16).wrapping_add(0x43d);
    let t = alocar(0x186);
    if t == 0 {
        aviso_em(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
        return 0;
    }
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as u16 as u32;
    let metade = if f < 8 {
        if f < 4 { 0 } else { 1 }
    } else {
        let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
        let menos: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let copia: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], usize) -> usize = core::mem::transmute(0x417c10usize);
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

/// Os bytes que a tabela tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4, +0x3d..+0x40
/// e o jogo 0 (+5..+0xb).
unsafe fn cqh4_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..6usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). As
/// preliminares e o mata-mata: só a original.
unsafe extern "thiscall" fn comparar_campeoes_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1652);
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || campeoes_rodadas_da_fase(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqh4_gravados(a) == cqh4_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("campeoes_rodadas_da_fase({}): saídas {} tabela {}", f, saidas, tabela));
    a
}

unsafe fn cqh4l_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh4l_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x68aa80 } (thiscall, ret 0, league.cpp, método da liga): `League::set_club_season_expectations()`: com o
/// gerenciador dos técnicos ([0xb67a44]), as expectativas da temporada de cada clube da tabela (+0xb1, linhas de 0x3b
/// bytes, +0x3e clubes) pela 0x6a3770 (push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8: com a notícia).
unsafe extern "thiscall" fn liga_expectativas_da_temporada(this: usize) {
    if cqh4l_d(0xb67a44) == 0 {
        return;
    }
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    let mut i = 0i16;
    while i < cqh4l_w(this + 0x3e) {
        crate::comparacao::ramo("cqh_4 expectativa do clube");
        let clube = cqh4l_d(cqh4l_d(this + 0xb1) + i as usize * 0x3b);
        expectativas(cqh4l_d(0xb67a44), clube, 1); // cqh4: com a notícia
        i += 1; // cqh4: o próximo clube
    }
}

/// Só a original (as expectativas mudam a diretoria e mandam notícias; não rodam duas vezes): a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_liga_expectativas_da_temporada(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1653)(this)
}

/// { 0x6831a0 } (thiscall, ret 0xc, league.cpp; this não é lido): `League::relegate_club_to_division(clube, divisão,
/// _)`: a competição do clube (+0x57) vira a divisão e a anterior vai para +0x5b; as cláusulas de rebaixamento (0x4cd030
/// push antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8); se a divisão não é jogada (sem a tabela das
/// competições [0xadadfc], sem divisão ou sem a competição do id dela) e o clube tem técnico humano que decide
/// (0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl), o técnico sai (0x69b790 push 0; push
/// 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10).
unsafe extern "thiscall" fn liga_rebaixar_clube(_this: usize, clube: usize, divisao: usize, _x: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let antiga = cqh4l_d(clube + 0x57);
    core::ptr::write_unaligned((clube + 0x57) as *mut u32, divisao as u32);
    core::ptr::write_unaligned((clube + 0x5b) as *mut u32, antiga as u32);
    clausulas(cqh4l_d(0xae1050), clube, antiga);
    let comps = cqh4l_d(0xadadfc);
    if comps != 0 && divisao != 0 && cqh4l_d(comps + cqh4l_d(divisao) * 4) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 {
        crate::comparacao::ramo("cqh_4 técnico sai");
        tirar(cqh4l_d(0xb67a44), clube, 1, 7, 0);
    }
}

/// Só a original (cláusulas de contrato e o técnico que sai não rodam duas vezes), conferindo o que ela grava: +0x57 =
/// a divisão e +0x5b = a competição de antes.
unsafe extern "thiscall" fn comparar_liga_rebaixar_clube(this: usize, clube: usize, divisao: usize, x: u32) {
    let antiga = cqh4l_d(clube + 0x57);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1654)(this, clube, divisao, x);
    PLACAR.registrar(cqh4l_d(clube + 0x57) == divisao && cqh4l_d(clube + 0x5b) == antiga, || format!("liga_rebaixar_clube: +0x57 {:#x} +0x5b {:#x}", cqh4l_d(clube + 0x57), cqh4l_d(clube + 0x5b)));
}

// ---------- 272a leva: a competição: todas as fases completas, a lista dos clubes e o progresso do clube (comp.cpp) ----------

unsafe fn cqh6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqh6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x48ce10 } (thiscall, ret 0, comp.cpp, método +0xc das competições): `Comp::all_stages_complete()`: só na
/// competição do topo (+0x1c ≤ −1), completa (método +0x10 com 1, 1: push 1; push 1; mov ecx -> thiscall, ret 8) ela e
/// cada uma da cadeia de baixo (+0x20, o índice em [0xadadfc]); devolve 1 ou 0 (no al).
unsafe extern "thiscall" fn competicao_todas_completas(this: usize) -> u8 {
    if (cqh6_d(this + 0x1c) as i32) > -1 {
        return 0;
    }
    let completa = |c: usize| -> u8 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cqh6_d(cqh6_d(c) + 0x10));
        f(c, 1, 1)
    };
    if completa(this) == 0 {
        return 0;
    }
    let mut i = cqh6_d(this + 0x20) as i32;
    while i >= 0 {
        let c = cqh6_d(cqh6_d(0xadadfc) + i as usize * 4);
        if completa(c) == 0 {
            return 0;
        }
        crate::comparacao::ramo("cqh_6 liga de baixo completa");
        i = cqh6_d(c + 0x20) as i32; // cqh6: a liga de baixo
    }
    1
}

/// Só a original (o método +0x10 marca a liga completa, grava recordes e paga prêmios: não roda duas vezes): a cópia
/// leva os ramos.
unsafe extern "thiscall" fn comparar_competicao_todas_completas(this: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize) -> u8>(1655)(this)
}

/// { 0x48e1c0 } (thiscall, ret 8, comp.cpp, método +0x58 das competições): `Comp::collect_club_list(&lista, _)`: um
/// vetor de 200 ponteiros (0x944e46(0x320); sem memória, a caixa da linha 0x681, a lista nula e 0) com os clubes
/// distintos de todas as fases (−1 a própria até +0x30; as outras em [+0xc]): numa liga (+0x43 = 2) os da tabela +0xb1
/// (linhas de 0x3b bytes), noutra os de +0xa7 (6 bytes), sem os nulos e os repetidos, até 200; grava o vetor em
/// `lista` e devolve a conta (word, no ax).
unsafe extern "thiscall" fn competicao_lista_de_clubes(this: usize, lista: *mut usize, _x: u32) -> u16 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0x320);
    if v == 0 {
        aviso_em(0x99b80c, 0x681);
        *lista = 0;
        return 0;
    }
    let mut n = 0i16;
    let mut s = -1i8;
    while s as i32 <= cqh6_d(this + 0x30) as i32 {
        let f = if s == -1 { this } else { cqh6_d(cqh6_d(this + 0xc) + (s as i32 * 4) as usize) };
        let liga = cqh6_b(f + 0x43) == 2;
        let mut i = 0i16;
        while i < cqh6_w(f + 0x3e) {
            let clube = if liga { cqh6_d(cqh6_d(f + 0xb1) + i as usize * 0x3b) } else { cqh6_d(cqh6_d(f + 0xa7) + i as usize * 6) };
            if clube != 0 && !(0..n).any(|k| cqh6_d(v + k as usize * 4) == clube) && n < 0xc8 { // cqh6: até 200 clubes
                crate::comparacao::ramo("cqh_6 clube na lista");
                core::ptr::write_unaligned((v + n as usize * 4) as *mut u32, clube as u32); // cqh6: o clube na lista
                n += 1;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    *lista = v;
    n as u16
}

/// Lado a lado: a original com a saída de quem chama, a cópia com saída própria, a partir do mesmo [0xb67a34]; a conta
/// e os clubes das duas listas iguais (a da cópia é liberada).
unsafe extern "thiscall" fn comparar_competicao_lista_de_clubes(this: usize, lista: *mut usize, x: u32) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(1656);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut l2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, lista, x), || competicao_lista_de_clubes(this, &mut l2, x));
    let mesmas = a == b && (*lista == 0) == (l2 == 0) && (*lista == 0 || core::slice::from_raw_parts(*lista as *const u32, a as usize) == core::slice::from_raw_parts(l2 as *const u32, b as usize));
    if l2 != 0 {
        soltar(l2);
    }
    PLACAR.registrar(igual && mesmas, || format!("competicao_lista_de_clubes: conta original={} rust={}", a, b));
    a
}
/// { 0x48cc70 } (thiscall, ret 0x10, comp.cpp): `Comp::find_club_progress(clube, &fase, &rodada, &situação)`: das
/// fases da última (+0x30) para a primeira (−1 a própria; as outras em [+0xc]), pulando as de id 0x3fc: numa liga
/// (+0x43 = 2), a linha da tabela do clube (0x6826d0 push i; mov ecx, fase -> thiscall, ret 4): fase = o id (+0x3a) e
/// situação = +0x37 da linha, e volta; numa copa, o último time da tabela +0xa7 (6 bytes) que é o clube dá fase = o id
/// e situação = +5, e a rodada é a da primeira rodada (de +0xab − 1 para trás, +0xa3 com 0x68 bytes, os confrontos de
/// +0xad com 0x35 bytes) com um confronto em que o time joga (+0 ou +2): rodada = +7 da rodada, e volta.
unsafe extern "thiscall" fn competicao_progresso_do_clube(this: usize, clube: usize, fase_id: *mut i16, rodada: *mut i16, situacao: *mut u8) {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let mut f = cqh6_d(this + 0x30) as i32;
    while f >= -1 {
        let s = if f > -1 { cqh6_d(cqh6_d(this + 0xc) + f as usize * 4) } else { this };
        if cqh6_w(s + 0x3a) != 0x3fc {
            if cqh6_b(s + 0x43) == 2 {
                let mut i = 0i8;
                while (i as i16) < cqh6_w(s + 0x3e) {
                    if cqh6_d(linha(s, i as i32)) == clube {
                        *fase_id = cqh6_w(s + 0x3a);
                        *situacao = cqh6_b(linha(s, i as i32) + 0x37);
                        return;
                    }
                    i = i.wrapping_add(1);
                }
            } else {
                let mut time: i16 = -1;
                let mut k = 0i16;
                while k < cqh6_w(s + 0x3e) {
                    let t = cqh6_d(s + 0xa7) + k as usize * 6;
                    if cqh6_d(t) == clube {
                        *fase_id = cqh6_w(s + 0x3a);
                        crate::comparacao::ramo("cqh_6 progresso na copa");
                        *situacao = cqh6_b(t + 5); // cqh6: a situação do time
                        time = k;
                    }
                    k += 1;
                }
                if time != -1 {
                    let mut r = (cqh6_b(s + 0xab) as i8 as i16).wrapping_sub(1);
                    while r >= 0 {
                        let n = cqh6_w(cqh6_d(s + 0xa3) + r as usize * 0x68 + 0x1a);
                        let confrontos = cqh6_d(cqh6_d(s + 0xad) + r as usize * 4);
                        let mut j = 0i16;
                        while j < n {
                            let c = confrontos.wrapping_add((j as i32 * 0x35) as usize);
                            if cqh6_w(c) == time || cqh6_w(c + 2) == time {
                                *rodada = cqh6_w(cqh6_d(s + 0xa3) + r as usize * 0x68 + 7);
                                return;
                            }
                            j += 1;
                        }
                        r -= 1;
                    }
                }
            }
        }
        f -= 1;
    }
}

/// Lado a lado nas três saídas (a original e a cópia com saídas próprias, começando dos valores de quem chama).
unsafe extern "thiscall" fn comparar_competicao_progresso_do_clube(this: usize, clube: usize, fase_id: *mut i16, rodada: *mut i16, situacao: *mut u8) {
    let (mut f2, mut r2, mut s2) = (*fase_id, *rodada, *situacao);
    orig::<unsafe extern "thiscall" fn(usize, usize, *mut i16, *mut i16, *mut u8)>(1657)(this, clube, fase_id, rodada, situacao);
    competicao_progresso_do_clube(this, clube, &mut f2, &mut r2, &mut s2);
    let ok = f2 == *fase_id && r2 == *rodada && s2 == *situacao;
    PLACAR.registrar(ok, || format!("competicao_progresso_do_clube: original ({}, {}, {}) rust ({}, {}, {})", *fase_id, *rodada, *situacao, f2, r2, s2));
}
