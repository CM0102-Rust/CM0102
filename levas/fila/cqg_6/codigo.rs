// ---------- %%LEVA%%a leva: a Liga dos Campeões: o sorteio dos grupos da segunda fase e a ordem dos classificados (european_cup.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
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
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = campeoes_ordem_da_segunda_fase(a, b);
    PLACAR.registrar(r == n, || format!("campeoes_ordem_da_segunda_fase: original={} rust={}", r, n));
    r
}
