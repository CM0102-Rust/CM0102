// ---------- %%LEVA%%a leva: as notícias das seleções aos clubes: o relatório do jogo, a convocação e os adversários (national_teams.cpp) ----------

unsafe fn cqf11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqf11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Os valores de 4 bytes dos itens de uma lista (nó: +0 o dado, +6 o próximo; cabeça em +2), sem mexer no cursor.
unsafe fn cqf11_itens(lista: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqf11_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqf11_d(cqf11_d(no)) as u32);
        no = cqf11_d(no + 6);
    }
    v
}

/// O ensaio do relatório (0x7706c0): os sorteios vêm da fita da original e a notícia não é entregue.
static mut CQF11_FITA: Option<(Vec<(i32, i32)>, usize, bool)> = None;

unsafe fn cqf11_sortear(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(CQF11_FITA);
    match f {
        Some((v, k, falhou)) => match v.get(*k) {
            Some(&(m, r)) if m == n && !*falhou => {
                *k += 1;
                r
            }
            _ => {
                *falhou = true;
                0
            }
        },
        None => {
            let s: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
            s(n)
        }
    }
}

/// O código de como o jogador foi no jogo (do registro dele no lado do clube, +0x190 + 0x49 × k): pelo tipo +0x21
/// (1 → 2, 2 → 8, 3 → 0x20, 4 a 8 → 0x40), +4 com nota (+0x34), +0x10 se é o melhor da partida (+0x3c = o id da ficha
/// +0x61), +1 se +0x23 − +0x24 é 1; 0 se ele não está no registro.
unsafe fn cqf11_codigo(resultado: usize, lado: usize, jogador: usize) -> u32 {
    let id = cqf11_d(jogador);
    for k in 0..0x14usize {
        let e = resultado + 0x190 + lado * 0x625 + k * 0x49;
        if cqf11_d(e) != id {
            continue;
        }
        let mut c: u32 = match cqf11_b(e + 0x21) {
            1 => 2,
            2 => 8,
            3 => 0x20,
            4..=8 => 0x40,
            _ => 0,
        };
        if cqf11_b(e + 0x34) != 0xff {
            c |= 4;
        }
        if cqf11_d(resultado + 0x3c) == cqf11_d(cqf11_d(jogador + 0x61)) {
            c |= 0x10;
        }
        if (cqf11_b(e + 0x23) as i32) - (cqf11_b(e + 0x24) as i32) == 1 {
            c |= 1;
        }
        return c;
    }
    0
}

/// { 0x7706c0 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH da notícia local): `NationalTeams::
/// send_club_international_report_news(clube, jogo, resultado, destaque, elenco, conta)`, pelo resultado entre seleções
/// (0x76b600), de cada lado. O elenco (vetor de `conta` pessoas, o byte baixo) é ordenado pelo clube (0x945333 com
/// 0x778ef0); cada grupo de jogadores do mesmo clube vira uma notícia 0x24 ao técnico do clube (0x7861e0): o registro
/// da partida pelo clube (0x7e4750, uma vez só) e os dados dele (0x7e4de0), o destaque (id e chaves, se é do clube;
/// senão −1 nos quatro), quantos são, o clube, até 4 jogadores (o destaque primeiro, os outros pelo código de como
/// foram, do maior para o menor), 0x420e9, sorteio(2) e os códigos. Os jogadores do grupo saem do vetor (zerados).
unsafe extern "thiscall" fn cqf11_relatorio_da_selecao(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
    let _ = this;
    let q = conta as u8 as i8;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let registro_da_partida: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(0x7e4750usize);
    let dados_da_partida: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e4de0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    // 0x945333 (qsort): push 0x778ef0; push 4; push conta; push elenco; add esp, 0x10 -> cdecl; 0x77bbe0: push 0; push
    // 0x24; lea ecx, notícia -> thiscall; 0x785790: push valor; push índice; lea ecx, notícia -> thiscall, ret 8;
    // 0x7e4750: push &registro; push clube; push jogo; add esp, 0xc -> cdecl; 0x7e4de0: push &índice; push &notícia;
    // push &registro; add esp, 0xc -> cdecl; 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
    ordenar(elenco as usize, q as i32, 4, 0x778ef0);
    if q <= 0 {
        return;
    }
    let lado = (clube != cqf11_d(jogo + 0x1c)) as usize;
    let mut registro = [0u8; 0x20]; // a Data em +0 (0x54a110 na entrada), o resto a 0x7e4750 preenche
    let reg = registro.as_mut_ptr() as usize;
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(reg);
    let mut montado = false;
    let mut noticia = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = noticia.as_mut_ptr() as usize;
    let mut i: i8 = 1;
    loop {
        let slot = elenco.add(i as usize - 1);
        let primeiro = *slot;
        if primeiro != 0 && cqf11_d(primeiro + 0x39) != 0 {
            let mut codigos = [0u32; 4];
            let mut jogadores = [0usize; 4];
            codigos[0] = cqf11_codigo(resultado, lado, primeiro);
            jogadores[0] = primeiro;
            let mut quantos: i8 = 1;
            let mut j = i;
            while j < q {
                let pj = elenco.add(j as usize);
                let outro = *pj;
                if outro != 0 {
                    if cqf11_d(outro + 0x39) != cqf11_d(primeiro + 0x39) {
                        break;
                    }
                    if quantos < 4 {
                        let k = quantos as usize;
                        codigos[k] = cqf11_codigo(resultado, lado, outro);
                        jogadores[k] = outro;
                        if outro == destaque {
                            // o destaque vai para a frente (troca com o primeiro, código junto)
                            jogadores[k] = jogadores[0];
                            jogadores[0] = outro;
                            codigos.swap(0, k);
                        }
                    }
                    quantos += 1;
                    *pj = 0;
                }
                j += 1;
            }
            construir(nt, 0x24, 0);
            if !montado {
                montado = true;
                registro_da_partida(jogo, clube, reg);
            }
            let mut idx: u8 = 0;
            dados_da_partida(reg, nt, &mut idx as *mut u8);
            let mut pr = |v: u32| {
                gravar(nt, idx as u32, v);
                idx = idx.wrapping_add(1);
            };
            if cqf11_d(destaque + 0x39) == cqf11_d(primeiro + 0x39) {
                pr(cqf11_d(destaque) as u32);
                pr(cqf11_d(cqf11_d(destaque + 4) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(destaque + 8) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(destaque + 0xc) + 0x33) as u32);
            } else {
                for _ in 0..4 {
                    pr(u32::MAX);
                }
            }
            pr(quantos as i32 as u32);
            pr(cqf11_d(cqf11_d(primeiro + 0x39)) as u32);
            // os de depois do primeiro, pelo código, do maior para o menor (até a 3ª posição e 4 jogadores)
            let fim = quantos as i32 - 1;
            let mut a: i8 = 1;
            if fim > 1 {
                while a < 3 {
                    let mut b = a + 1;
                    while b < quantos && b < 4 {
                        if (codigos[a as usize] as i32) < (codigos[b as usize] as i32) { // cqf11: a ordem dos jogadores
                            codigos.swap(a as usize, b as usize);
                            jogadores.swap(a as usize, b as usize);
                        }
                        b += 1;
                    }
                    a += 1;
                    if a as i32 >= fim {
                        break;
                    }
                }
            }
            let n = quantos.min(4);
            for k in 0..n.max(0) as usize {
                let p = jogadores[k];
                pr(cqf11_d(p) as u32);
                pr(cqf11_d(cqf11_d(p + 4) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(p + 8) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(p + 0xc) + 0x33) as u32);
            }
            pr(0x420e9);
            crate::comparacao::ramo("cqf_11 noticia do clube");
            let k_sorteio = idx;
            idx = idx.wrapping_add(1);
            let r = cqf11_sortear(2); // cqf11: o sorteio da notícia
            gravar(nt, k_sorteio as u32, r as u32);
            for k in 0..n.max(0) as usize {
                gravar(nt, idx as u32, codigos[k]);
                idx = idx.wrapping_add(1);
            }
            if (*core::ptr::addr_of!(CQF11_FITA)).is_none() {
                entregar(cqf11_d(0xb7431c), nt, cqf11_d(primeiro + 0x39));
            }
        }
        let anterior = i;
        i = i.wrapping_add(1);
        if anterior >= q {
            break;
        }
    }
}

/// Comparador da 0x7706c0: a original entrega as notícias (aloca nas caixas): só ela roda, com os sorteios rastreados e o
/// elenco guardado antes. Depois o elenco volta ao de antes, a cópia refaz em ensaio (sorteios da fita, sem entregar) e o
/// elenco tem de sair igual ao da original, com os sorteios do ensaio iguais aos da fita (mesmos n, todos usados).
unsafe extern "thiscall" fn comparar_cqf11_relatorio_da_selecao(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32)>(%%I0%%);
    let q = (conta as u8 as i8).max(0) as usize;
    let antes: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    let ((), fita) = crate::aleatorio::rastrear(|| o(this, clube, jogo, resultado, destaque, elenco, conta));
    let depois_o: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    let n = fita.len();
    for k in 0..q {
        *elenco.add(k) = antes[k];
    }
    *core::ptr::addr_of_mut!(CQF11_FITA) = Some((fita, 0, false));
    crate::aleatorio::sem_contar(|| cqf11_relatorio_da_selecao(this, clube, jogo, resultado, destaque, elenco, conta));
    let (k, falhou) = match &*core::ptr::addr_of!(CQF11_FITA) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(CQF11_FITA) = None;
    let depois_r: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    for k in 0..q {
        *elenco.add(k) = depois_o[k];
    }
    PLACAR.registrar(!falhou && k == n && depois_o == depois_r, || format!("cqf11_relatorio_da_selecao(clube {:#x}, jogo {:#x}, conta {}) fita de {} sorteios, ensaio usou {} (n diferente: {}); elenco original {:x?} copia {:x?}",
        clube, jogo, q, n, k, falhou, depois_o, depois_r));
}

/// { 0x76e0b0 } (thiscall, ret 0x1c, national_teams.cpp, quadro SEH da notícia local): `national_teams_post_call_up_news(
/// seleção, segunda, jogo, tipo, sem_adversarios, adversarios, conta)`, pela convocação (0x76c120) e pela tela. Das 50
/// vagas do elenco da seleção (+0xd7, ponteiros), cada jogador com a marca (+0x5e: 8 na segunda seleção, senão 4) e
/// clube de técnico humano (0x7875e0) entra no grupo do clube dele; quando o próximo da vaga não é do mesmo clube (ou
/// não tem a marca, ou acabou), sai a notícia 0x1d ao técnico do clube (0x7861e0): a seleção, o clube, `sem_adversarios`,
/// o tipo, quantos, `segunda`, quantos adversários e, com menos de 4, os ids deles (da lista dada ou montada uma vez pela
/// 0x76d560 com o jogo, sem lista e sem `sem_adversarios`), e até 3 jogadores (id e chaves). A lista montada aqui é
/// destruída e liberada no fim (0x68d040, 0x944c94).
unsafe extern "thiscall" fn cqf11_noticia_da_convocacao(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let montar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x76d560usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    // 0x7875e0: push clube; mov ecx, [0xb7431c] -> thiscall, ret 4; 0x76d560: push jogo; push seleção; mov ecx, this ->
    // thiscall, ret 8; 0x77bbe0 / 0x785790 / 0x68d1b0 / 0x68d040: lea ecx (ou mov ecx, lista) -> thiscall; 0x7861e0:
    // push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x944c94 (delete): push; add esp, 4 -> cdecl
    let mut lista = 0usize;
    let mut quantos_adv: i16 = 0;
    if adversarios != 0 {
        lista = adversarios;
        quantos_adv = conta as u8 as i8 as i16;
    }
    let marca: u8 = if segunda != 0 { 8 } else { 4 };
    let mut n: i8 = 0;
    let mut tres = [0usize; 3];
    let mut noticia = [0u32; 0x38];
    let nt = noticia.as_mut_ptr() as usize;
    for idx in 1..=0x32usize {
        let p = cqf11_d(selecao + 0xd7 + (idx - 1) * 4);
        if p == 0 || cqf11_b(p + 0x5e) & marca == 0 || cqf11_d(p + 0x39) == 0 || humano(cqf11_d(0xb7431c), cqf11_d(p + 0x39)) == 0 {
            continue;
        }
        if lista == 0 && sem_adversarios == 0 && jogo != 0 {
            lista = montar(this, selecao, jogo);
            quantos_adv = *(lista as *const i16);
        }
        if n < 3 {
            tres[n as usize] = p;
        }
        let clube = cqf11_d(p + 0x39);
        n = n.wrapping_add(1);
        if idx < 0x32 {
            let prox = cqf11_d(selecao + 0xd7 + idx * 4);
            if prox != 0 && cqf11_d(prox + 0x39) == clube && cqf11_b(prox + 0x5e) & marca != 0 {
                continue;
            }
        }
        crate::comparacao::ramo("cqf_11 noticia da convocacao");
        construir(nt, 0x1d, 0);
        gravar(nt, 0, cqf11_d(selecao) as u32);
        gravar(nt, 1, cqf11_d(clube) as u32);
        gravar(nt, 2, sem_adversarios);
        gravar(nt, 3, tipo);
        gravar(nt, 4, n as i32 as u32);
        gravar(nt, 5, segunda);
        gravar(nt, 6, quantos_adv as i32 as u32);
        if quantos_adv < 4 && lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqf11_d(lista + 2));
            let mut k: u8 = 7;
            let mut item = proximo(lista);
            while item != 0 {
                gravar(nt, k as u32, cqf11_d(cqf11_d(item)) as u32);
                k = k.wrapping_add(1);
                item = proximo(lista);
            }
        }
        if n <= 3 && n > 0 {
            let mut k: u8 = 0xa;
            for t in 0..n as usize {
                let j = tres[t];
                gravar(nt, k as u32, cqf11_d(j) as u32);
                gravar(nt, k.wrapping_add(1) as u32, cqf11_d(cqf11_d(j + 4) + 0x33) as u32);
                gravar(nt, k.wrapping_add(2) as u32, cqf11_d(cqf11_d(j + 8) + 0x33) as u32);
                gravar(nt, k.wrapping_add(3) as u32, cqf11_d(cqf11_d(j + 0xc) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
        }
        entregar(cqf11_d(0xb7431c), nt, clube);
        n = 0;
    }
    if lista != 0 && adversarios == 0 {
        soltar(lista);
        apagar(lista);
    }
}

/// Comparador da 0x76e0b0: ela só monta e entrega notícias (aloca nas caixas dos humanos) e monta e libera a lista dos
/// adversários: não pode rodar duas vezes e não grava nada do jogo além das caixas. Só a original roda (ponytail: o
/// efeito é a caixa de entrada do humano; a lista dos adversários é conferida pela 0x76d560).
unsafe extern "thiscall" fn comparar_cqf11_noticia_da_convocacao(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32)>(%%I1%%)(this, selecao, segunda, jogo, tipo, sem_adversarios, adversarios, conta);
}

/// O ensaio da 0x76d560: os valores que iriam para a lista nova, sem alocar.
static mut CQF11_PLANO: Option<Vec<u32>> = None;

/// { 0x76d560 } (thiscall, ret 8, national_teams.cpp, quadro SEH da lista local): `national_teams_collect_fixture_
/// opponents(seleção, jogo)`, pela notícia da convocação (0x76e0b0) e pelo pedido ao técnico (0x76f1a0). Uma lista
/// nova (0x944cf1 com 0xe, 0x68d020) com o adversário do jogo (o visitante se o mandante é da nação da seleção, senão o
/// mandante; na segunda seleção, a seleção principal da nação dele, 0x53b4f0) e os adversários dos outros jogos da
/// seleção de amanhã até 13 dias depois (0x5acbe0, com o byte +0x40 do jogo) que não são de fase final (as nove
/// competições de seleções). Devolve a lista.
unsafe extern "thiscall" fn cqf11_adversarios_da_selecao(this: usize, selecao: usize, jogo: usize) -> usize {
    let _ = this;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    // 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx (ou mov ecx, bloco) -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 ->
    // cdecl; 0x68d0d0: push 4; push &valor; mov ecx, lista -> thiscall, ret 8; 0x5409b0 / 0x53b4f0: push; add esp, 4 ->
    // cdecl; 0x549ef0: push +0x28; push +0x2a; lea ecx -> thiscall, ret 8; 0x54c5b0: push n; push &saída; mov ecx, data
    // -> thiscall, ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
    let ensaio = (*core::ptr::addr_of!(CQF11_PLANO)).is_some();
    let mut local = [0u8; 16];
    let l = local.as_mut_ptr() as usize;
    nova(l);
    let mut nl = 0usize;
    if !ensaio {
        let m = novo(0xe);
        if m != 0 {
            nl = nova(m);
        }
    }
    let mut por = |v: usize| {
        if ensaio {
            if let Some(p) = &mut *core::ptr::addr_of_mut!(CQF11_PLANO) {
                p.push(cqf11_d(v) as u32);
            }
        } else {
            acrescentar(nl, v, 4);
        }
    };
    // o 9º argumento da 0x5acbe0: o dword do argumento `jogo` na pilha (ou do 0x53b4f0 gravado em cima) com o byte baixo
    // trocado pelo +0x40 do jogo
    let mut celula: usize = jogo;
    let mandante = jogo + 0x1c;
    if cqf11_d(cqf11_d(mandante) + 0x53) == cqf11_d(selecao + 0x53) {
        if segunda(selecao) != 0 {
            celula = principal(cqf11_d(cqf11_d(jogo + 0x20) + 0x53));
            por(&celula as *const usize as usize);
        } else {
            por(jogo + 0x20);
        }
    } else if segunda(selecao) != 0 {
        celula = principal(cqf11_d(cqf11_d(mandante) + 0x53));
        por(&celula as *const usize as usize);
    } else {
        por(mandante);
    }
    celula = (celula & !0xff) | cqf11_b(jogo + 0x40) as usize;
    let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    let dia_ano = |p: usize| (cqf11_d(p) as u32, core::ptr::read_unaligned((p + 2) as *const u32));
    data_nova(p0, *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    mais(p0, p1, 1);
    mais(p1, p2, 0xd);
    let (a1, a2) = dia_ano(p1);
    let (a3, a4) = dia_ano(p2);
    achar(cqf11_d(0xae2a58), a1, a2, a3, a4, cqf11_d(selecao) as u32, l, -1, -2, celula as u32);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqf11_d(l + 2));
    let finais = [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964];
    let mut item = proximo(l);
    while item != 0 {
        let c = cqf11_d(item + 0x14);
        if c == 0 || !finais.iter().any(|&k| cqf11_d(c) == cqf11_d(k)) {
            if cqf11_d(cqf11_d(item + 0x1c) + 0x53) == cqf11_d(selecao + 0x53) {
                por(item + 0x20);
            } else {
                por(item + 0x1c);
            }
        }
        item = proximo(l);
    }
    soltar(l);
    nl
}

/// Comparador da 0x76d560: a original aloca a lista nova (não pode rodar duas vezes): só ela roda; antes, a cópia em
/// ensaio diz os valores que vão na lista (sem alocar), e a lista da original tem de ter exatamente esses, na ordem.
unsafe extern "thiscall" fn comparar_cqf11_adversarios_da_selecao(this: usize, selecao: usize, jogo: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I2%%);
    *core::ptr::addr_of_mut!(CQF11_PLANO) = Some(Vec::new());
    cqf11_adversarios_da_selecao(this, selecao, jogo);
    let plano = (*core::ptr::addr_of_mut!(CQF11_PLANO)).take().unwrap_or_default();
    let r = o(this, selecao, jogo);
    let agora = cqf11_itens(r);
    PLACAR.registrar(agora == plano, || format!("cqf11_adversarios_da_selecao(seleção {:#x}, jogo {:#x}) plano {:x?} original {:x?}", selecao, jogo, plano, agora));
    r
}
