// ---------- %%LEVA%%a leva: a Liga dos Campeões: o sorteio dos grupos da primeira fase, a ordem dos potes e as posições previstas (european_cup.cpp) ----------

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
    orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
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
    let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(%%I1%%)(a, b);
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
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
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
