// ---------- octogesima quarta leva: competicao da pessoa, bytes da selecao, selecoes especiais, data com marca ----------

/// { 0x7751e0 } (ret 4, national_teams.cpp): a competicao (0xae23d0, 107 bytes) do indice +4 do
/// registro da pessoa em [this+4] (110 bytes); sem pessoa ou indice negativo, 0.
unsafe extern "thiscall" fn competicao_da_pessoa(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let reg = (*((this + 4) as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110));
    let i = *((reg + 4) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(0xae23d0 as *const usize)).wrapping_add((i as usize).wrapping_mul(107))
}

/// O byte `o` do bloco da classe (0x76d780: 1 com `flag`, 2 para a segunda selecao, senao 0) da
/// nacao do clube (+0x53) nos registros de [this+4]. A original chama 0x5409b0 duas vezes.
unsafe fn byte_da_selecao(this: usize, clube: usize, flag: u32, o: usize) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let classe: usize = if segunda(clube) == 0 {
        if flag == 0 { 0 } else { 1 }
    } else if flag == 0 {
        2
    } else {
        1
    };
    segunda(clube);
    let nacao = *(*((clube + 0x53) as *const usize) as *const i32);
    *((*((this + 4) as *const usize)).wrapping_add((nacao as usize).wrapping_mul(110)).wrapping_add(classe * 20 + o) as *const u8)
}

/// { 0x7754c0 } (ret 8): o byte +0x42 do bloco da selecao do clube.
unsafe extern "thiscall" fn byte_42_da_selecao(this: usize, clube: usize, flag: u32) -> u8 {
    byte_da_selecao(this, clube, flag, 0x42)
}

/// { 0x775520 } (ret 8): o byte +0x43 do bloco da selecao do clube.
unsafe extern "thiscall" fn byte_43_da_selecao(this: usize, clube: usize, flag: u32) -> u8 {
    byte_da_selecao(this, clube, flag, 0x43)
}

/// { 0x775790 } (stdcall): selecao (0x53b900) ou um dos clubes de 0x9d00a4 e 0x9d00a8?
unsafe extern "stdcall" fn selecao_ou_especial(clube: usize) -> u32 {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if e_selecao(clube) != 0 {
        return 1;
    }
    let id = *(clube as *const u32);
    (id == *(0x9d00a4 as *const u32) || id == *(0x9d00a8 as *const u32)) as u32
}

/// { 0x776380 } (stdcall): a pessoa nao gosta (0x547d30) da selecao principal (0x53b3d0) da nacao +0x1a?
unsafe extern "stdcall" fn nao_gosta_da_selecao(pessoa: usize) -> u32 {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let s = principal(*((pessoa + 0x1a) as *const usize));
    (nao_gosta(pessoa, s) != 0) as u32
}

/// { 0x777590 } (ret 4): copia a data (0x417c10) e o byte +8 de `origem`; devolve `this`.
unsafe extern "thiscall" fn copiar_data_e_marca(this: usize, origem: usize) -> usize {
    let mut tmp = [0u8; 8];
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    copiar(this, tmp.as_mut_ptr() as usize, origem);
    *((this + 8) as *mut u8) = *((origem + 8) as *const u8);
    this
}

// ---------- octogesima terceira leva: mais campos por pessoa (national_teams.cpp), global 0x9b5984 ----------

/// O bloco `k` da pessoa nos registros de [this+4]: id x 110 + k x 20.
unsafe fn bloco_da_pessoa(this: usize, pessoa: usize, k: i8) -> usize {
    (*((this + 4) as *const usize))
        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
}

/// { 0x76c0f0 } (ret 8): liga (1) o byte +0x44 do bloco `k` da pessoa.
unsafe extern "thiscall" fn ligar_44_da_pessoa(this: usize, pessoa: usize, k: i8) {
    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *mut u8) = 1;
}

/// { 0x76e010 } (ret 8): o byte +0x44 do bloco `k` da pessoa, com sinal.
unsafe extern "thiscall" fn byte_44_da_pessoa(this: usize, pessoa: usize, k: i8) -> i32 {
    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *const i8) as i32
}

/// { 0x76e040 } (ret 8): o endereco do campo +0x3a do bloco `k` da pessoa.
unsafe extern "thiscall" fn campo_3a_da_pessoa(this: usize, pessoa: usize, k: i8) -> usize {
    bloco_da_pessoa(this, pessoa, k) + 0x3a
}

/// { 0x76e070 } (stdcall): sem selecao (+0x24), 0xff; segunda selecao (0x5409b0), 2; senao o bit 3
/// do byte +0x5e. Devolve so `al`.
unsafe extern "stdcall" fn classe_pela_selecao(pessoa: usize) -> u8 {
    let selecao = *((pessoa + 0x24) as *const usize);
    if selecao == 0 {
        return 0xff;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(selecao) != 0 {
        return 2;
    }
    (*((pessoa + 0x5e) as *const u8) >> 3) & 1
}

/// { 0x535920 } (Database.cpp): a global 0x9b5984.
unsafe extern "C" fn global_9b5984() -> u32 {
    *(0x9b5984 as *const u32)
}

// ---------- 270a leva: as notícias das seleções aos clubes: o relatório do jogo, a convocação e os adversários (national_teams.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32)>(1623);
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
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32)>(1624)(this, selecao, segunda, jogo, tipo, sem_adversarios, adversarios, conta);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(1625);
    *core::ptr::addr_of_mut!(CQF11_PLANO) = Some(Vec::new());
    cqf11_adversarios_da_selecao(this, selecao, jogo);
    let plano = (*core::ptr::addr_of_mut!(CQF11_PLANO)).take().unwrap_or_default();
    let r = o(this, selecao, jogo);
    let agora = cqf11_itens(r);
    PLACAR.registrar(agora == plano, || format!("cqf11_adversarios_da_selecao(seleção {:#x}, jogo {:#x}) plano {:x?} original {:x?}", selecao, jogo, plano, agora));
    r
}

// ---------- a escolha do elenco da seleção (0x76c120), com a correção dos elencos de 23 e 25 ----------

unsafe fn cqc12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc12_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// Os códigos de posição das vagas da original: elenco de 26 (0x76c771..0x76c84c), de 24 (0x76c870..0x76c92c) e o de
/// qualquer outro tamanho (0x76c939..0x76c9de, 22 vagas), com os comuns gravados depois (0x76c860/0x76c865 e as vagas
/// 0, 1, 3, 5, 6 e 21). O código 0xc da vaga 12/11/10 vem do ebp (0xc desde 0x76c2e4/0x76c6a9).
const CQC12_26: [u16; 26] = [1, 1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240, 0x240];
const CQC12_24: [u16; 24] = [1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
const CQC12_22: [u16; 22] = [1, 1, 1, 0x202, 0x204, 0x204, 0x204, 0x204, 0x84, 0x804, 0xc, 0x210, 0x210, 0x220, 0x90, 0x810,
    0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
/// As vagas que a tabela de 24 tem a mais que a de 22, na ordem em que aparecem nela: 0x202 (líbero/centro, a vaga
/// 3), 0x88 e 0x808 (volantes pelos lados, 9 e 10) e 0x210 (o 3º meia central, 14).
const CQC12_EXTRAS: [u16; 3] = [0x202, 0x88, 0x808];

/// Os códigos de posição das `tam` vagas (o byte +0x42 do registro do tipo). Fiel à original em 26 e 24 e nas 22
/// primeiras vagas dos outros tamanhos (a tabela de 22). Correção do original aprovada (01/11/2026): a original só
/// preenche as vagas de 22, 24 ou 26 jogadores; nos elencos de 23 (as fases finais, 0x76d430: a Copa de 2002, em
/// 29/05/2002) e 25, as vagas a partir da 22ª ficam com a sobra da pilha (local_86c), que vai à escala (0x6e2000) e
/// ao filtro dos escalados (0x5b7d20), e a sobra muda a cada execução (metades de ponteiros da heap). Aqui as vagas
/// além das 22 da tabela da original recebem, em ordem, as que a tabela de 24 acrescenta à de 22 (CQC12_EXTRAS):
/// o elenco de 23 = o de 22 + 0x202 (a primeira vaga que o de 24 acrescenta); o de 25 = o de 22 + 0x202, 0x88 e
/// 0x808. Por que assim: as 22 primeiras vagas continuam as que a original grava nesses tamanhos (a comparação
/// confere todas), e cada vaga a mais vai na direção da tabela maior seguinte. O elenco de 25 parte da tabela de 22
/// e não da de 24 porque é a que a original usa para ele (só 24 e 26 têm tabela própria). Outros tamanhos: como a
/// original nas 22 primeiras; as demais com código 0 (ninguém passa no filtro). ponytail: o jogo só pede 23, 24 e 26.
unsafe fn cqc12_codigos(tam: u8) -> [u16; 50] {
    let mut c = [0u16; 50];
    match tam {
        0x1a => c[..26].copy_from_slice(&CQC12_26),
        0x18 => c[..24].copy_from_slice(&CQC12_24),
        _ => c[..22].copy_from_slice(&CQC12_22),
    }
    if tam == 23 || tam == 25 {
        crate::comparacao::ramo(if tam == 23 { "cqc_12 correção do elenco de 23" } else { "cqc_12 correção do elenco de 25" });
        for i in 22..tam as usize {
            c[i] = CQC12_EXTRAS[i - 22]; // cqc12: as vagas da correção
        }
    }
    c
}

/// O registro da nação nas seleções (+4 da tabela, 0x6e bytes pelo id da nação).
unsafe fn cqc12_registro(this: usize, nacao: usize) -> usize {
    cqc12_d(this + 4).wrapping_add(cqc12_d(nacao).wrapping_mul(0x6e))
}

/// As faixas da reputação aceita (+9 da ficha +0x61): (mínimo exclusive, máximo), −1 e 0x2711 fora da Austrália
/// ([0x9cf224]); para ela, pela competição da partida (amistoso 0x5c6710 de [0xae2c74], ou a da Oceania: 0x8ca/0x2134;
/// Copa das Confederações 0xabe/0x222e; eliminatórias da Oceania 0x8ca e 9000, 8000 ou 10001 pelo byte +0x42; Copa
/// do Mundo 0xabe/10000).
unsafe fn cqc12_limites(nacao: usize, p: usize) -> (i16, i16) {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    // 0x5c6710: push [p + 0x14]; mov ecx, [0xae2c74] -> thiscall ret 4
    if cqc12_d(nacao) != cqc12_d(0x9cf224) {
        return (-1, 0x2711);
    }
    if p == 0 || (cqc12_d(0xae2c74) != 0 && amistoso(cqc12_d(0xae2c74), cqc12_d(p + 0x14)) != 0) {
        return (0x8ca, 0x2134);
    }
    let k = cqc12_d(cqc12_d(p + 0x14));
    if k == cqc12_d(0x9cf8a0) {
        (0x8ca, 0x2134)
    } else if k == cqc12_d(0x9cf964) {
        (0xabe, 0x222e)
    } else if k == cqc12_d(0x9cf76c) {
        (0x8ca, match cqc12_b(p + 0x42) { 1 => 0x1f40, 2 => 0x2711, _ => 0x2328 })
    } else if k == cqc12_d(0x9cf79c) {
        (0xabe, 0x2710)
    } else {
        (-1, 0x2711)
    }
}

/// O time do tipo (0x53b3d0 a principal nos tipos 0 e 1, 0x53b4f0 o sub-21), a marca de elenco (+0x5e: 8 no tipo
/// 1, 4 nos outros) e o time da tática (1 no tipo 1).
unsafe fn cqc12_time(nacao: usize, t: i8) -> (usize, u8, u32) {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    // 0x53b3d0 e 0x53b4f0: push nacao; add esp, 4 -> cdecl
    match t {
        1 => (principal(nacao), 8, 1),
        0 => (principal(nacao), 4, 0),
        _ => (sub21(nacao), 4, 0),
    }
}

/// Os tipos de reserva percorridos: só o do tipo, fora o sub-21 (2) dos Jogos Olímpicos ([0x9cf7a0]), que olha as
/// reservas 0 e 1; o sub-21 fora deles olha a 1.
unsafe fn cqc12_faixa(t: i8, p: usize) -> (usize, usize) {
    if t != 2 {
        (0, 0)
    } else if p != 0 && cqc12_d(p) == cqc12_d(0x9cf7a0) {
        (0, 1)
    } else {
        (1, 1)
    }
}

/// As entradas das reservas do tipo `k`, grupo `g`: (vetor [this + 0xc + 4k] a partir do início, quantas). O início é
/// o dword +8 + 0x15k do registro mais os bytes +0xe + 0x15k + 4j dos grupos anteriores; a quantidade, o byte +0xd +
/// 0x15k + 4g (como a original lê: a soma e a quantidade saem de bytes vizinhos).
unsafe fn cqc12_grupo(this: usize, reg: usize, k: usize, g: usize) -> (usize, i32, i32) {
    let base = reg + 0xd + 0x15 * k;
    let mut ini = cqc12_d(base - 5) as i32;
    for j in 0..g {
        ini = ini.wrapping_add(cqc12_b(base + 1 + 4 * j) as i32);
    }
    let fim = ini.wrapping_add(cqc12_b(base + 4 * g) as i32);
    (cqc12_d(this + 0xc + 4 * k).wrapping_add((ini as usize).wrapping_mul(4)), ini, fim)
}

/// O jogador `j` das reservas entra na lista da escala? (0x76cb84..0x76cf6c)
#[allow(clippy::too_many_arguments)]
unsafe fn cqc12_candidato(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), j: usize) -> bool {
    let tipo_ok: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let bits: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x76db00usize);
    // 0x53b620: push byte +0x3d; add esp, 4 -> cdecl; 0x62c400: push j; mov ecx, [0xb63d28] -> thiscall ret 4 (al);
    // 0x7a7eb0: push j; mov ecx, [0xb7434c] -> stdcall ret 4 (al); 0x547d30: push clube; push j; add esp, 0xc (junto
    // com o da 0x53b3d0) -> cdecl; 0x4cf5a0: push j; mov ecx, [0xae1050] -> thiscall ret 4; 0x55d270: push 0; push 1;
    // push [p + 0x14]; push j; mov ecx, [0xae28f0] -> thiscall ret 0x10; 0x76db00: 6 pushes (nação, 0, j, 0, 0, 0);
    // mov ecx, this -> thiscall ret 0x18
    if j == 0 || tipo_ok(cqc12_b(j + 0x3d) as u32) == 0 || lesao(cqc12_d(0xb63d28), j) != 0 {
        return false;
    }
    if cqc12_d(0xb7434c) != 0 && bits(j) != 0 {
        return false;
    }
    if cqc12_d(j + 0x39) == 0 || nao_gosta(j, principal(cqc12_d(j + 0x1a))) != 0 {
        return false;
    }
    let tj = cqc12_d(j + 0x24);
    if tj != 0 && (cqc12_b(j + 0x28) != 0xb || cqc12_d(tj + 0x53) != nacao) {
        return false;
    }
    if p != 0 {
        let k = cqc12_d(p);
        let c = cqc12_d(j + 0x39);
        let outra_nacao = |c: usize| c != 0 && cqc12_d(c + 0x53) != 0 && cqc12_d(cqc12_d(c + 0x53) + 0x71) != cqc12_d(nacao + 0x71);
        let mut contrato_ok = true;
        if k == cqc12_d(0x9cf668) || k == cqc12_d(0x9cf794) || k == cqc12_d(0x9cf798) {
            if outra_nacao(c) {
                // amistoso, B e sub-21 internacionais: clube de outra nação só com reputação (+0x80) até 0.8 da do
                // adversário (o mandante +0x1c é o próprio time: o visitante +0x20); fora dos amistosos, não
                if k != cqc12_d(0x9cf668) {
                    return false;
                }
                let s = cqc12_d(p + 0x1c);
                let outro = if s == elenco { cqc12_d(p + 0x20) } else { s };
                if cqc12_w(c + 0x80) as f64 * 0.8 > cqc12_w(outro + 0x80) as f64 {
                    return false;
                }
            }
            // clube com jogo (a tabela +0x24 pelo id do clube) e contrato com +0x3d acima de −10: não
            if cqc12_b(cqc12_d(this + 0x24).wrapping_add(cqc12_d(c))) != 0 {
                let x = contrato(cqc12_d(0xae1050), j);
                if x != 0 && (cqc12_b(contrato(cqc12_d(0xae1050), j) + 0x3d) as i8) > -10 {
                    contrato_ok = false;
                }
            }
            if !contrato_ok {
                return false;
            }
        } else if outra_nacao(c) && cqc12_d(c + 0x57) != 0
            && (cqc12_w(cqc12_d(p + 0x14) + 0x69) as f64) < cqc12_w(cqc12_d(c + 0x57) + 0x69) as f64 * 0.7
        {
            // nas outras competições, clube de outra nação cuja liga (+0x57) tem +0x69 acima da da competição / 0.7
            return false;
        }
        if suspenso(cqc12_d(0xae28f0), j, cqc12_d(p + 0x14), 1, 0) != 0 {
            return false;
        }
        if t == 0 {
            let m = cqc12_uw(p + 0x4d) & 0x300;
            let tj = cqc12_d(j + 0x24);
            if m != 0x100 && tj == elenco && cqc12_b(j + 0x5e) & 8 != 0 {
                return false;
            }
            if m != 0x200 && tj != 0 && tj != elenco {
                return false;
            }
        }
    }
    if cqc12_d(j + 0x24) == principal(nacao) && cqc12_b(j + 0x5e) & 4 != 0 {
        if pode_mudar(this, nacao, 0, j, 0, 0, 0) == 0 {
            return false;
        }
        if !(p != 0 && cqc12_d(p) == cqc12_d(0x9cf7a0)) && t != 0 {
            return false;
        }
    }
    // com a marca, só depois da data do elenco do tipo (ano +2 sem sinal, dia +0 com sinal)
    let depois = |a: usize| {
        let (ya, da) = (cqc12_uw(data + 2), cqc12_w(data));
        let (yr, dr) = (cqc12_uw(a + 2), cqc12_w(a));
        ya > yr || (ya == yr && da > dr)
    };
    if cqc12_b(j + 0x5e) & 8 != 0 && !depois(reg + 0x4e) {
        return false;
    }
    let tj = cqc12_d(j + 0x24);
    if tj == sub21(nacao) && cqc12_b(j + 0x5e) & 4 != 0 && !depois(reg + 0x62) {
        return false;
    }
    if t == 2 && cqc12_b(j + 0x18) as i8 > 0x17 { // cqc12: a idade do sub-21
        return false;
    }
    let nat = cqc12_d(j + 0x1a);
    if nat != nacao {
        // de outra nação: sem time, sem a marca +0x22, sem o bit 2 do +0xc do registro de 79 bytes, com a segunda
        // nação (+0x1e) nesta; entra se esta tiver +0x8e maior, ou (clube da mesma confederação +0x71 ou sem clube)
        // com +0x85 de 5 para cima
        if tj != 0 || cqc12_b(j + 0x22) != 0 || cqc12_b(cqc12_d(0xae2d0c).wrapping_add(cqc12_d(j).wrapping_mul(0x4f)) + 0xc) & 2 != 0
            || cqc12_d(j + 0x1e) != nacao || nat == 0
        {
            return false;
        }
        let c = cqc12_d(j + 0x39);
        let n2 = if c != 0 { cqc12_d(c + 0x53) } else { 0 };
        if cqc12_w(nacao + 0x8e) <= cqc12_w(nat + 0x8e) {
            if n2 != 0 && cqc12_d(nacao + 0x71) != cqc12_d(n2 + 0x71) {
                return false;
            }
            if (cqc12_b(nacao + 0x85) as i8) < 5 {
                return false;
            }
        }
    }
    let v = cqc12_w(cqc12_d(j + 0x61) + 9);
    v > lim.0 && v <= lim.1
}

/// A lista da escala (0x76cae5..0x76cfe6): as reservas dos tipos da faixa, grupo a grupo; devolve a quantidade (byte,
/// como a original) e deixa em `cursor` o ponteiro da última entrada percorrida mais 4 (a original o lê depois).
#[allow(clippy::too_many_arguments)]
unsafe fn cqc12_candidatos(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), cand: &mut [usize; 200], cursor: &mut usize) -> u8 {
    let mut n = 0u8;
    let (lo, hi) = cqc12_faixa(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (v, ini, fim) = cqc12_grupo(this, reg, k, g);
            *cursor = v;
            for _ in ini..fim {
                let j = cqc12_d(*cursor);
                if cqc12_candidato(this, nacao, t, data, p, elenco, reg, lim, j) {
                    // ponytail: a original grava em local_32c[(char) n]; de 128 em diante ela escreveria antes do vetor
                    if (n as i8) >= 0 {
                        cand[n as usize] = j;
                    }
                    n = n.wrapping_add(1);
                }
                *cursor += 4;
            }
        }
    }
    n
}

/// { 0x76c120 } (thiscall, ret 0x14, national_teams.cpp, ~4,9 KB, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::select_squad(nação, tipo, data, partida, forçado)`, pela convocação (0x76bab0, em Rust: nação +0x53
/// do lado, tipo, 0xae2c90, partida, 0), pela montagem das tabelas (0x76a840: partida 0) e pelas notícias (0x784100:
/// forçado 1). Tipo 0 principal, 1 B/olímpico (marca 8, time da tática 1), 2 sub-21 (0x53b4f0). Sem `forçado`: nas
/// fases finais (+0x14 da partida numa das 9 chaves) um item (time, competição, data do jogo − 10, 1000) entra na lista
/// em +0 (0x68d0d0, criada com 0x68d020 se faltar); a data do elenco do tipo (+0x3a do registro: hoje − 1 sem partida,
/// a do jogo nos tipos 0 e acima de 2, a do jogo − 1 nos tipos 1 e 2, que antes marcam as duas nações no vetor
/// [this + 0x36] nos Jogos Olímpicos ou dispensam o sub-21 marcado, 0x76d880) e a de hoje (+0x32); as idades e o
/// tamanho (0x76d430 em +0x42/+0x43). Com técnico humano que decide (0x600a50, 0x5fd6b0; ou pela seleção principal
/// de uma seleção B sem técnico): os machucados por mais de 10 dias (0x62c400 = 2, 0x62ed70 +0x2e) saem do elenco; se
/// ainda for dele, o pedido ao técnico (0x76f1a0) e fim. Senão: os códigos das vagas (cqc12_codigos, com a correção
/// de 23 e 25); saem do elenco os de outro time, os com a marca e, no tipo 0 com o bit 1 de +0x4e da partida, os sem
/// ela (0x896c60 e +0x5e/+0x24/+0x28); a lista das reservas (cqc12_candidatos) vai à escala (0x6e2000, modo 0x2100914
/// no tipo 1, 0x2000814 no 0 e 0x814 no 2); cada escalado que serve na vaga (0x5b7d20) entra no elenco: sai do time
/// anterior (vaga, 0x76db00, 0x896c60) ou das vagas da tática do próprio (0x8968f0/0x896e90, 0x76db00), ganha a marca
/// (0x540980), o time e +0x28 (0xc de 5 ou 0xc, senão 0xb) e o +9 do registro de 79 bytes vai a 0..100. Por fim o
/// elenco é ordenado (0x945333 com 0x778ef0), a notícia (0x76e0b0) e as vagas da tática com quem não é do time saem.
/// As caixas de erro (linhas 0x748 e 0x76b) como a original.
unsafe extern "thiscall" fn escolher_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let inserir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76d880usize);
    let idades: unsafe extern "stdcall" fn(usize, u32, usize, usize) = core::mem::transmute(0x76d430usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x5fd6b0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let dias_de_lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    let pedir: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x76f1a0usize);
    let escalar: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(0x6e2000usize);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x76db00usize);
    let da_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x8968f0usize);
    let limpar_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896e90usize);
    let marcar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(0x76e0b0usize);
    // 0x549ef0: push +0x28; push +0x2a; lea ecx, saida -> thiscall ret 8 (palavras); 0x54c8e0: push n; push saida; mov
    // ecx, data -> thiscall ret 8; 0x417c10: push origem; push saida; lea ecx, destino -> thiscall ret 8; 0x54a110: lea
    // ecx -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 -> cdecl; 0x68d020: mov ecx, bloco -> thiscall ret 0;
    // 0x68d0d0: push 0x12; push item; mov ecx, [this] -> thiscall ret 8; 0x76d880: push nação; mov ecx, this ->
    // thiscall ret 4; 0x76d430: 4 pushes (partida, tipo, +0x42, +0x43) -> stdcall ret 0x10; 0x600a50: 5 pushes (time,
    // 0, 1, 0, 0); add esp -> cdecl; 0x5fd6b0: push 0; push técnico; mov ecx, [0xb63c98] -> thiscall ret 8 (al);
    // 0x5409b0: push; add esp, 4 -> cdecl; 0x62ed70: push j; mov ecx, [0xb63d28] -> thiscall ret 4; 0x896c60: push 1;
    // push time; push j; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0x10; 0x76f1a0: 3 pushes (elenco, time,
    // partida); mov ecx, this -> ret 0xc; 0x6e2000: 15 pushes; add esp, 0x3c -> cdecl; 0x5b7d20: push +0x61; add esp,
    // 4 -> cdecl (ax); 0x8968f0 e 0x896e90: push time; push vaga; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0xc;
    // 0x540980: push marca; push j; add esp, 8 -> cdecl; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x76e0b0:
    // 7 pushes; mov ecx, this -> ret 0x1c
    let t = tipo as u8 as i8;
    let reg = cqc12_registro(this, nacao);
    let lim = cqc12_limites(nacao, p);
    let (elenco, marca, time) = cqc12_time(nacao, t);
    let chaves = [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964];
    let finais = p != 0 && cqc12_d(p + 0x14) != 0 && chaves.iter().any(|&k| cqc12_d(cqc12_d(p + 0x14)) == cqc12_d(k));
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tatica = || cqc12_d(0xde1d08);
    if forcado == 0 {
        let (mut d88c, mut d87c, mut d8a0) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p88c, p87c, p8a0) = (d88c.as_mut_ptr() as usize, d87c.as_mut_ptr() as usize, d8a0.as_mut_ptr() as usize);
        if finais {
            crate::comparacao::ramo("cqc_12 elenco das fases finais");
            let d = data_nova(p87c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            menos(d, p88c, 10);
            let comp = cqc12_d(p + 0x14);
            if cqc12_d(this) == 0 {
                let m = novo(0xe);
                cqc12_pd(this, if m != 0 { lista_nova(m) } else { 0 });
            }
            let mut item = [0u8; 0x12];
            let pi = item.as_mut_ptr() as usize;
            padrao(pi + 8);
            cqc12_pd(pi + 4, cqc12_d(comp));
            cqc12_pd(pi, cqc12_d(elenco));
            copia(pi + 8, p87c, p88c);
            core::ptr::write_unaligned((pi + 0x10) as *mut u16, 1000);
            inserir(cqc12_d(this), pi, 0x12);
        }
        if p == 0 {
            let x = menos(data, p88c, 1);
            copia(r + 0x3a, p87c, x);
        } else if t != 1 && t != 2 {
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            copia(r + 0x3a, p87c, x);
        } else {
            // a nação de cada lado (+0x53 do time) e a casa dela no vetor [this + 0x36]
            let nac = |o: usize| cqc12_d(cqc12_d(p + o) + 0x53);
            let casa = |o: usize| cqc12_d(nac(o)).wrapping_add(cqc12_d(this + 0x36));
            if cqc12_d(p) == cqc12_d(0x9cf7a0) {
                cqc12_pb(casa(0x1c), 1);
                cqc12_pb(casa(0x20), 1);
            } else {
                if cqc12_b(casa(0x1c)) != 0 {
                    dispensar(this, nac(0x1c));
                }
                if cqc12_b(casa(0x20)) != 0 {
                    dispensar(this, nac(0x20));
                }
                cqc12_pb(casa(0x1c), 0);
                cqc12_pb(casa(0x20), 0);
            }
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            let y = menos(x, p87c, 1);
            copia(r + 0x3a, p8a0, y);
        }
        copia(r + 0x32, p88c, data);
        idades(p, tipo, r + 0x42, r + 0x43);
        let tecnico = cqc12_d(principal(nacao) + 0xcf);
        let decide = || {
            (humano(elenco, 0, 1, 0, 0) != 0 && (t == 0 || t9a06(cqc12_d(0xb63c98), tecnico, 0) != 0))
                || (segunda(elenco) != 0 && cqc12_d(elenco + 0xcf) == 0
                    && humano(principal(cqc12_d(elenco + 0x53)), 0, 1, 0, 0) != 0
                    && t9a06(cqc12_d(0xb63c98), tecnico, 0) != 0)
        };
        if decide() {
            for k in 0..50 {
                let s = elenco + 0xd7 + 4 * k;
                let j = cqc12_d(s);
                if j == 0 || lesao(cqc12_d(0xb63d28), j) != 2 || cqc12_b(j + 0x5e) & marca == 0 {
                    continue;
                }
                let l = dias_de_lesao(cqc12_d(0xb63d28), j);
                if l != 0 && cqc12_w(l + 0x2e) > 10 {
                    tirar(tatica(), elenco, j, time, 1);
                    cqc12_pb(j + 0x5e, cqc12_b(j + 0x5e) & 3);
                    cqc12_pd(j + 0x24, 0);
                    cqc12_pb(j + 0x28, if cqc12_b(j + 0x28) == 0xc { 5 } else { 0 });
                    cqc12_pd(s, 0);
                }
            }
        }
        if decide() {
            crate::comparacao::ramo("cqc_12 elenco pelo técnico humano");
            cqc12_pb(r + 0x44, 0);
            pedir(this, elenco, time, p);
            return;
        }
    }
    let tam = r + 0x42;
    let codigos = cqc12_codigos(cqc12_b(tam));
    // saem do elenco: os de outro time; os com a marca; no tipo 0 com o bit 1 de +0x4e da partida, os sem ela
    for k in 0..50 {
        let s = elenco + 0xd7 + 4 * k;
        let j = cqc12_d(s);
        if j == 0 {
            continue;
        }
        if cqc12_d(j + 0x24) != elenco {
            tirar(tatica(), elenco, j, time, 1);
        } else {
            let antigo = if cqc12_b(j + 0x5e) & marca != 0 {
                5
            } else if t == 0 && cqc12_b(p + 0x4e) & 1 != 0 {
                0xb
            } else {
                continue;
            };
            tirar(tatica(), elenco, j, time, 1);
            cqc12_pb(j + 0x5e, cqc12_b(j + 0x5e) & 3);
            cqc12_pd(j + 0x24, 0);
            cqc12_pb(j + 0x28, if cqc12_b(j + 0x28) == 0xc { antigo } else { 0 });
        }
        cqc12_pd(s, 0); // cqc12: a vaga do elenco fica livre
    }
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let n = cqc12_candidatos(this, nacao, t, data, p, elenco, reg, lim, &mut cand, &mut cursor);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut saida = [0usize; 50];
    crate::comparacao::ramo("cqc_12 escala do elenco");
    escalar(cand.as_ptr() as usize, n as u32, codigos.as_ptr() as usize, (modo & !0xff) | cqc12_b(tam) as u32, 0,
        saida.as_mut_ptr() as usize, modo, elenco, 0, 0, 0, 0, 0, 0, 0);
    let mut i = 0usize;
    // ponytail: a original não limita a 50 (o jogo pede até 26)
    while (i as i32) < cqc12_b(tam) as i8 as i32 && i < 50 {
        let j = saida[i];
        if j != 0 && codigos[i] & mascara(cqc12_d(j + 0x61)) != 0 {
            if let Some(k) = (0..50).find(|&k| cqc12_d(elenco + 0xd7 + 4 * k) == 0) {
                cqc12_pd(elenco + 0xd7 + 4 * k, j);
            }
            let tj = cqc12_d(j + 0x24);
            if tj != elenco && tj != 0 {
                // de outro time: sai da vaga de lá (com a permissão, a marca de convocado do tipo dele zera) e da tática
                let v = ((cqc12_b(j + 0x5e) as i8 as i32 & 8) >> 3) as u32;
                let c3: u8 = if segunda(tj) == 0 { (v != 0) as u8 } else if v != 0 { 1 } else { 2 };
                let mut c = cqc12_d(j + 0x24) + 0xd7;
                for _ in 0..50 {
                    if cqc12_d(c) == j {
                        cqc12_pd(c, 0);
                        if pode_mudar(this, cqc12_d(cqc12_d(j + 0x24) + 0x53), c3 as u32, 0, 0, 0, 0) != 0 {
                            cqc12_pb(reg + 0x43 + c3 as usize * 0x14, 0);
                            break;
                        }
                        aviso_em(0xa5b2a4, 0x748);
                    }
                    c += 4;
                }
                cursor = c;
                tirar(tatica(), cqc12_d(j + 0x24), j, v, 1);
            } else if tj == elenco && cqc12_b(j + 0x5e) & marca == 0 {
                // já do time sem a marca: sai das vagas da tática do time (time da tática invertido)
                let bz = (time == 0) as u32;
                let c3: u8 = if segunda(elenco) == 0 { (bz != 0) as u8 } else if bz != 0 { 1 } else { 2 };
                for v in 0..20 {
                    if da_vaga(tatica(), cqc12_d(j + 0x24), v, bz) != j {
                        continue;
                    }
                    limpar_vaga(tatica(), cqc12_d(j + 0x24), v, bz);
                    // a original passa como jogador o dword do último ponteiro de percurso (cursor): reproduzido
                    if c3 != 0 && pode_mudar(this, cqc12_d(cqc12_d(j + 0x24) + 0x53), c3 as u32, cqc12_d(cursor), 0, 0, 0) != 0 {
                        cqc12_pb(reg + 0x43 + c3 as usize * 0x14, 0);
                        break;
                    }
                    aviso_em(0xa5b2a4, 0x76b);
                }
            }
            marcar(j, marca as u32);
            cqc12_pd(j + 0x24, elenco); // cqc12: o convocado ganha o time
            let x = cqc12_b(j + 0x28);
            cqc12_pb(j + 0x28, if x == 5 || x == 0xc { 0xc } else { 0xb });
            let e = cqc12_d(0xae2d0c).wrapping_add(cqc12_d(j).wrapping_mul(0x4f)) + 9;
            cqc12_pb(e, (cqc12_b(e) as i8).clamp(0, 100) as u8);
        }
        i += 1;
    }
    ordenar(elenco + 0xd7, 0x32, 4, 0x778ef0);
    let chave = if p == 0 { u32::MAX } else { cqc12_d(cqc12_d(p + 0x14)) as u32 };
    noticia(this, elenco, time, p, chave, finais as u32, 0, 0);
    for v in 0..20 {
        let x = da_vaga(tatica(), elenco, v, time);
        if x != 0 && cqc12_d(x + 0x24) != elenco {
            limpar_vaga(tatica(), elenco, v, time);
        }
    }
}

/// O que a escala da 0x76c120 lê e a original muda depois dela: o registro da nação (0x6e bytes) e, de cada jogador
/// das reservas percorridas, +0x24..+0x28 (time, estado) e +0x5e (marcas de elenco).
unsafe fn cqc12_regioes(this: usize, nacao: usize, t: i8, p: usize) -> Vec<(usize, usize)> {
    let reg = cqc12_registro(this, nacao);
    let mut v = vec![(reg, 0x6e)];
    let (lo, hi) = cqc12_faixa(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (a, ini, fim) = cqc12_grupo(this, reg, k, g);
            for i in 0..(fim - ini).max(0) as usize {
                let j = cqc12_d(a + 4 * i);
                if j != 0 {
                    v.push((j + 0x24, 5));
                    v.push((j + 0x5e, 1));
                }
            }
        }
    }
    v
}

/// Comparador: a original aloca (0x68d020/0x68d0d0), mexe nos elencos, nas vagas da tática e manda notícias (0x76e0b0,
/// 0x76f1a0): não pode rodar duas vezes. Só ela roda, com uma gravação da fita aberta nas regiões de cqc12_regioes:
/// a escala (0x6e2000) entra nela como unidade com os vetores de entrada (a lista e os códigos) e o estado das
/// regiões logo antes. Depois, com esse estado de volta, a cópia Rust monta a lista (cqc12_candidatos) e os códigos
/// (cqc12_codigos), que têm de bater com os da original, junto com a quantidade, o modo e o time; o estado de depois
/// da original volta no fim. Nos elencos de 23 e 25 a original passa a sobra da pilha nas vagas a partir da 22ª: elas
/// ficam fora (correção conhecida, o ramo); as 22 primeiras e o resto conferem. Sem escala (técnico humano), nada a
/// comparar. Dentro de uma gravação de fora, só a original.
unsafe extern "thiscall" fn comparar_escolher_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32)>(1542);
    if gravando() {
        return o(this, nacao, tipo, data, p, forcado);
    }
    let t = tipo as u8 as i8;
    let regioes = cqc12_regioes(this, nacao, t, p);
    let externa = fita().take();
    *fita() = Some((regioes.clone(), Vec::new()));
    let nivel = DENTRO.swap(0, Relaxed);
    o(this, nacao, tipo, data, p, forcado);
    let gravadas = fita().take().map_or(Vec::new(), |f| f.1);
    DENTRO.store(nivel, Relaxed);
    *fita() = externa;
    let Some((_, _, a, _, antes, _, (ent, _))) = gravadas.iter().find(|c| c.0 == 0x6e2000) else {
        return;
    };
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    antes.escrever();
    let reg = cqc12_registro(this, nacao);
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tam = cqc12_b(r + 0x42);
    let (elenco, _, _) = cqc12_time(nacao, t);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let (n, codigos) = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        let n = cqc12_candidatos(this, nacao, t, data, p, elenco, reg, cqc12_limites(nacao, p), &mut cand, &mut cursor);
        (n, cqc12_codigos(tam))
    }));
    sorteio.escrever();
    depois.escrever();
    let n1 = (a[1] as u8 as i8).max(0) as usize;
    let dw = |k: usize| u32::from_le_bytes([ent[k], ent[k + 1], ent[k + 2], ent[k + 3]]) as usize;
    let lista_o: Vec<usize> = (0..n1.min(ent.len() / 4)).map(|k| dw(4 * k)).collect();
    let cod_o: Vec<u16> = ent.get(4 * n1..).unwrap_or(&[]).chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let corrigido = tam == 23 || tam == 25;
    let lim = if corrigido { 22.min(cod_o.len()) } else { cod_o.len() };
    let igual = a[1] as u8 == n && a[2] == modo && a[3] as usize == elenco && cod_o.len() == tam as usize
        && lista_o[..] == cand[..n1.min(200)] && cod_o[..lim] == codigos[..lim];
    PLACAR.registrar(igual, || format!(
        "escolher_elenco_da_selecao(nação {:x}, tipo {}, tam {}) original: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}; rust: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}",
        nacao, t, tam, a[1] as u8, a[2], a[3], lista_o, cod_o, n, modo, elenco, &cand[..(n as i8).max(0) as usize], &codigos[..tam as usize]));
}

// ---------- 262a leva: o jogador a serviço da seleção: a seleção ou o time das estrelas que o tem na data, o jogador que serve à seleção do técnico e as competições das convocações (national_teams.cpp) ----------

unsafe fn cpz6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz6_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do time das estrelas `g` ([0x9d00a4] ou [0x9d00a8]): o registro de 0x245 bytes em [0xae23b4].
unsafe fn cpz6_estrelas(g: usize) -> usize {
    cpz6_d(0xae23b4).wrapping_add(cpz6_d(g).wrapping_mul(0x245))
}

/// { 0x7757d0 } (thiscall, ret 0x14, national_teams.cpp): `NationalTeams::get_international_duty_club(jogador, data,
/// clube, comentario, adversario)`, da escalação (0x6dbb80, 0x6dbc50), dos olheiros, das transferências e das telas.
/// Com uma lista provisória (0x68d020, destruída no fim por 0x68d040). Clube seleção (0x53b900) ou um dos times das
/// estrelas (id em [0x9d00a4]/[0x9d00a8]): 0. A janela vai da véspera (0x54c8e0) ao dia seguinte (0x54c5b0). Jogador
/// convocado (+0x24, a seleção): os jogos da seleção na janela (0x5acbe0, lado pelo bit 4 de +0x5e: ligado 1, senão 0;
/// os dias e anos como os dwords da original); com jogo e sem adversário, ou com o adversário (+0x14 do jogo) entre
/// eles, ele está a serviço (o texto 0xa62ff4 em `comentario`, 0x66f4e0) e devolve a seleção. Com o bit 4 ligado, as
/// convocações da lista [this] (+0 a seleção, +4 o adversário, +8 a data) com a seleção dele (e o adversário, se dado)
/// e data antes da pedida: está no time (o texto 0xa62fc8) e devolve a seleção. Senão o time das estrelas (a
/// competição [[0xadadfc] + 4·[0x9cf690]]: a data do jogo pelo registro +0xa3, dia +3 e ano +5 + o ano +0x40, 0x549ef0)
/// para jogador de clube (+0x39) da liga [0x9cf590] (+0x57), com a data na janela: os 50 nomes (+0xd7) dos dois times
/// (primeiro [0x9d00a8]; o texto 0xa62fa4 em 0xde1f64 se há `comentario`). Senão 0.
unsafe extern "thiscall" fn selecao_que_tem_o_jogador(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                      adversario: usize) -> usize {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut lista = [0u8; 0x10];
    let pl = lista.as_mut_ptr() as usize;
    criar(pl);
    let sai = |r: usize| -> usize {
        destruir(pl);
        r
    };
    if clube != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(clube) != 0 {
            return sai(0);
        }
        let id = cpz6_d(clube);
        if id == cpz6_d(0x9d00a4) || id == cpz6_d(0x9d00a8) {
            return sai(0);
        }
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (mut lo, mut hi) = ([0u8; 8], [0u8; 8]);
    let (plo, phi) = (lo.as_mut_ptr() as usize, hi.as_mut_ptr() as usize);
    menos(data, plo, 1); // cpz6: a véspera
    mais(data, phi, 1); // cpz6: o dia seguinte
    let nacional = cpz6_d(jogador + 0x24);
    if nacional != 0 {
        let sem_bit = (cpz6_b(jogador + 0x5e) & 4 == 0) as u32;
        // ponytail: a original empurra o lado com os 24 bits altos de ecx sobrando; a 0x5acbe0 só lê o byte
        let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(0x5acbe0usize);
        let n = jogos(cpz6_d(0xae2a58), cpz6_d(plo) as u32, cpz6_d(plo + 2) as u32, cpz6_d(phi) as u32, cpz6_d(phi + 2) as u32,
            cpz6_d(nacional) as u32, pl, u32::MAX, 0xffff_fffe, (sem_bit == 0) as u32);
        let mut servico = false;
        if n as u16 != 0 {
            if adversario == 0 {
                servico = true;
            } else {
                core::ptr::write_unaligned((pl + 0xa) as *mut usize, cpz6_d(pl + 2));
                loop {
                    let j = proximo(pl);
                    if j == 0 {
                        break;
                    }
                    if cpz6_d(j + 0x14) == adversario {
                        servico = true;
                        break;
                    }
                }
            }
        }
        if servico {
            if comentario != 0 {
                texto(comentario, 0xa62ff4, 0xad9c64usize);
            }
            crate::comparacao::ramo("cpz_6 a servico da selecao");
            return sai(cpz6_d(jogador + 0x24)); // cpz6: a serviço da seleção
        }
        let conv = cpz6_d(this);
        if conv != 0 && sem_bit == 0 {
            core::ptr::write_unaligned((conv + 0xa) as *mut usize, cpz6_d(conv + 2));
            loop {
                let e = proximo(cpz6_d(this));
                if e == 0 {
                    break;
                }
                if cpz6_d(e) != cpz6_d(cpz6_d(jogador + 0x24)) {
                    continue;
                }
                if adversario != 0 && cpz6_d(e + 4) != cpz6_d(adversario) {
                    continue;
                }
                let (dia, ano) = (cpz6_w(e + 8) as i16, cpz6_w(e + 0xa));
                let hoje = cpz6_w(data + 2);
                if ano < hoje || (ano == hoje && dia < cpz6_w(data) as i16) {
                    if comentario != 0 {
                        texto(comentario, 0xa62fc8, 0xad9c64usize);
                    }
                    crate::comparacao::ramo("cpz_6 no time da selecao");
                    return sai(cpz6_d(jogador + 0x24)); // cpz6: já no time da seleção
                }
            }
        }
    }
    let comp = cpz6_d(cpz6_d(0xadadfc).wrapping_add(cpz6_d(0x9cf690).wrapping_mul(4)));
    if comp == 0 {
        return sai(0);
    }
    let reg = cpz6_d(comp + 0xa3);
    let ano = (cpz6_b(reg + 5) as i8 as i16).wrapping_add(cpz6_w(comp + 0x40) as i16) as u16;
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut jogo = [0u8; 8];
    let pj = jogo.as_mut_ptr() as usize;
    data_nova(pj, cpz6_w(reg + 3) as u32, ano as u32);
    let cj = cpz6_d(jogador + 0x39);
    if cj == 0 {
        return sai(0);
    }
    let liga = cpz6_d(cj + 0x57);
    if liga == 0 || cpz6_d(liga) != cpz6_d(0x9cf590) {
        return sai(0);
    }
    let (jd, ja) = (cpz6_w(pj) as i16, cpz6_w(pj + 2));
    let (ld, la) = (cpz6_w(plo) as i16, cpz6_w(plo + 2));
    if la > ja || (la == ja && ld > jd) {
        return sai(0);
    }
    let (hd, ha) = (cpz6_w(phi) as i16, cpz6_w(phi + 2));
    if ha < ja || (ha == ja && hd < jd) {
        return sai(0);
    }
    for i in 0..0x32usize {
        for g in [0x9d00a8usize, 0x9d00a4] {
            if cpz6_d(cpz6_estrelas(g) + 0xd7 + 4 * i) == jogador {
                if comentario != 0 {
                    texto(0xde1f64, 0xa62fa4, 0xad9c64usize);
                }
                crate::comparacao::ramo("cpz_6 time das estrelas");
                return sai(cpz6_estrelas(g));
            }
        }
    }
    sai(0)
}

/// Comparador da 0x7757d0 (2,1 milhões de chamadas até 2002-06-01: o lado a lado leve, sem alocar até 256 bytes): cada
/// uma com a sua lista provisória (como a original); no texto de 0xde1f64 (0x80) e no do comentário (0x40) e na devolução.
unsafe extern "thiscall" fn comparar_selecao_que_tem_o_jogador(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                               adversario: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize>(1395);
    let regioes = [(0xde1f64usize, 0x80usize), (comentario, 0x40)];
    let reg = if comentario != 0 { &regioes[..] } else { &regioes[..1] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg,|| o(this, jogador, data, clube, comentario, adversario),
        || selecao_que_tem_o_jogador(this, jogador, data, clube, comentario, adversario));
    PLACAR.registrar(igual && r == n, || format!("selecao_que_tem_o_jogador(jogador {:x}, data {:x}, clube {:x}, adversario {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        jogador, data, clube, adversario, r, n, mo, mr));
    r
}

/// { 0x76d9b0 } (stdcall, ret 8, national_teams.cpp): `national_teams_player_is_eligible(jogador, tecnico)`, da busca de
/// jogadores (0x7afa70, 0x807450) e da tela de comparação. Sem ficha de jogador (+0x61), sem a seleção do técnico (+0x24)
/// ou ela não sendo seleção (0x53b900): 0. Já convocado para ela: 1. Seleção do segundo conjunto (0x5409b0, a sub-21)
/// e idade (+0x18) acima de 21: 0. Com +0x22 ou o bit 2 de +0xc do registro de 79 bytes da pessoa ([0xae2d0c]), a
/// nacionalidade (+0x1a) tem de ser a da seleção (+0x53); convocado para outra seleção de outra nação: 0. A nação da
/// seleção tem de ser a primeira ou a segunda (+0x1e) nacionalidade; sendo a nação [0x9cf434] e a segunda também: 0.
/// Senão 1.
unsafe extern "stdcall" fn jogador_serve_a_selecao(jogador: usize, tecnico: usize) -> u32 {
    let sel = cpz6_d(tecnico + 0x24);
    if cpz6_d(jogador + 0x61) == 0 || sel == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(sel) == 0 {
        return 0;
    }
    if cpz6_d(jogador + 0x24) == sel {
        return 1;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(sel) != 0 && cpz6_b(jogador + 0x18) as i8 > 0x15 {
        return 0;
    }
    let nacao = cpz6_d(sel + 0x53);
    let primeira = cpz6_d(jogador + 0x1a);
    if cpz6_b(jogador + 0x22) != 0 && primeira != nacao {
        return 0;
    }
    let p = cpz6_d(0xae2d0c).wrapping_add(cpz6_d(jogador).wrapping_mul(79));
    if cpz6_b(p + 0xc) & 2 != 0 && cpz6_d(jogador + 0x1a) != nacao {
        return 0;
    }
    let outra = cpz6_d(jogador + 0x24);
    if outra != 0 && cpz6_d(outra + 0x53) != nacao {
        return 0;
    }
    let primeira = cpz6_d(jogador + 0x1a);
    if primeira != nacao && cpz6_d(jogador + 0x1e) != nacao {
        return 0;
    }
    let x = cpz6_d(0x9cf434);
    if cpz6_d(nacao) == x {
        let s = cpz6_d(jogador + 0x1e);
        if s != 0 && cpz6_d(s) == x {
            return 0;
        }
    }
    1
}

/// Comparador da 0x76d9b0: pela devolução (só lê).
unsafe extern "stdcall" fn comparar_jogador_serve_a_selecao(jogador: usize, tecnico: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u32>(1396)(jogador, tecnico);
    let n = jogador_serve_a_selecao(jogador, tecnico);
    PLACAR.registrar(r == n, || format!("jogador_serve_a_selecao(jogador {:x}, tecnico {:x}) original={} rust={}", jogador, tecnico, r, n));
    r
}

/// { 0x775580 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::collect_duty_competitions(saida)`, dos olheiros
/// (0x7f7010). Sem a lista de convocações [this] ou sem `saida`: 0. Para cada convocação, a competição dela (+4) entra
/// em `saida` (0x68d0d0, 4 bytes) se ainda não estiver lá; devolve quantas entraram (byte).
unsafe extern "thiscall" fn juntar_competicoes_das_convocacoes(this: usize, saida: usize) -> u8 {
    let conv = cpz6_d(this);
    if conv == 0 || saida == 0 {
        return 0;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let mut n: u8 = 0;
    core::ptr::write_unaligned((conv + 0xa) as *mut usize, cpz6_d(conv + 2));
    let mut e = proximo(cpz6_d(this));
    while e != 0 {
        core::ptr::write_unaligned((saida + 0xa) as *mut usize, cpz6_d(saida + 2));
        let mut ja = false;
        loop {
            let x = proximo(saida);
            if x == 0 {
                break;
            }
            if cpz6_d(x) == cpz6_d(e + 4) {
                ja = true;
                break;
            }
        }
        if !ja {
            acrescentar(saida, e + 4, 4);
            n = n.wrapping_add(1);
        }
        e = proximo(cpz6_d(this));
    }
    n
}

/// Os valores dos nós de uma lista (cabeça em +2, nó: +0 o item, +6 o próximo), sem mexer no cursor (+0xa).
unsafe fn cpz6_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpz6_d(lista + 2);
    while no != 0 {
        v.push(cpz6_d(no));
        no = cpz6_d(no + 6);
    }
    v
}

// ponytail: a 0x775580 acrescenta nós (aloca): só a original roda. O plano, feito antes pelos nós (sem mexer nos
// cursores): as competições das convocações que ainda não estão em `saida`, na ordem, sem repetir; depois, a devolução
// é quantas e os últimos itens de `saida` (4 bytes cada) são elas.
unsafe extern "thiscall" fn comparar_juntar_competicoes_das_convocacoes(this: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1397);
    let conv = cpz6_d(this);
    if conv == 0 || saida == 0 {
        let r = o(this, saida);
        PLACAR.registrar(r == 0, || format!("juntar_competicoes_das_convocacoes(saida {:x}) sem lista original={}", saida, r));
        return r;
    }
    let mut tem: Vec<usize> = cpz6_itens(saida).iter().map(|&x| cpz6_d(x)).collect();
    let antes = tem.len();
    for e in cpz6_itens(conv) {
        let c = cpz6_d(e + 4);
        if !tem.contains(&c) {
            tem.push(c);
        }
    }
    let r = o(this, saida);
    let depois: Vec<usize> = cpz6_itens(saida).iter().map(|&x| cpz6_d(x)).collect();
    let igual = r == (tem.len() - antes) as u8 && depois == tem;
    PLACAR.registrar(igual, || format!("juntar_competicoes_das_convocacoes(saida {:x}) original={} plano={:x?} depois={:x?}", saida, r, tem, depois));
    r
}

// ---------- 261a leva: as convocações para o jogo de seleções e duas notícias: escolher os elencos dos dois lados, a notícia do convocado e a do desfecho na copa (national_teams.cpp, Cup.cpp) ----------

unsafe fn cpx13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O ensaio do comparador da 0x76bab0: a cópia roda sem escolher elenco (0x76c120), sem dispensar os acima da idade do
/// sub-21 (0x76d880: no ensaio, só o byte +0x6b que ela zera depois da data) e sem apagar repetidos (0x76c060).
static mut CPX13_ENSAIO: bool = false;
/// No ensaio: a cópia pediu um elenco (a comparação não vale).
static mut CPX13_PEDIU: bool = false;

/// O registro da nação do clube na tabela das seleções ([this+4], 0x6e bytes, pelo id da nação +0x53).
unsafe fn cpx13_registro(this: usize, clube: usize) -> usize {
    cpx13_d(this + 4).wrapping_add(cpx13_d(cpx13_d(clube + 0x53)).wrapping_mul(0x6e))
}

/// Um elenco do registro `reg` (tipo 0 principal, 1 B/olímpico, 2 sub-21: a marca de hoje em +0x32/+0x46/+0x5a, a
/// data do elenco em +0x3a/+0x4e/+0x62, a marca de convocado em +0x43/+0x57/+0x6b): com o elenco de até 14 dias (0x54cdb0
/// sobre hoje) e a marca (e `extra`), grava hoje (0x417c10) e leva a data do elenco até a data do jogo (+0x2a/+0x28)
/// se ela for anterior; senão pede o elenco (0x76c120 com a nação, o tipo, hoje, a partida e 0).
unsafe fn cpx13_elenco(this: usize, p: usize, clube: usize, reg: usize, tipo: u32, extra: impl FnOnce() -> bool) {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let base = reg + [0x32usize, 0x46, 0x5a][tipo as usize];
    let data = base + 8;
    let d = dias(0xae2c90, cpx13_d(data) as u32, cpx13_d(data + 4) as u32);
    if d <= 0xe && cpx13_b(base + 0x11) != 0 && extra() {
        let mut r = [0u8; 8];
        copia(base, r.as_mut_ptr(), 0xae2c90);
        let mut jogo = [0u8; 8];
        data_nova(jogo.as_mut_ptr(), cpx13_w(p + 0x2a) as u16 as u32, cpx13_w(p + 0x28) as u16 as u32);
        let (ra, rd) = (cpx13_w(data + 2) as u16, cpx13_w(data) as i16);
        let (ja, jd) = (u16::from_le_bytes([jogo[2], jogo[3]]), i16::from_le_bytes([jogo[0], jogo[1]]));
        if ra < ja || (ra == ja && rd < jd) {
            core::ptr::copy_nonoverlapping(data as *const u8, r.as_mut_ptr(), 8);
            copia(data, r.as_mut_ptr(), jogo.as_ptr() as usize);
        }
        return;
    }
    if CPX13_ENSAIO {
        CPX13_PEDIU = true;
        return;
    }
    let escolher: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32) = core::mem::transmute(0x76c120usize);
    crate::comparacao::ramo("cpx_13 elenco pedido");
    escolher(this, cpx13_d(clube + 0x53), tipo, 0xae2c90, p, 0); // cpx13: o elenco
}

/// Os elencos B/olímpico (+0x4e da partida, bit 0x100) ou sub-21 (bit 0x200, ou o clube é seleção B 0x5409b0) do lado
/// `clube`; o sub-21 só sem convocado: competição [0x9cf7a0] ou a nação fora do vetor [this+0x36].
unsafe fn cpx13_elencos_menores(this: usize, p: usize, clube: usize, reg: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let m = cpx13_w(p + 0x4d) as u16;
    if m & 0x100 != 0 {
        cpx13_elenco(this, p, clube, reg, 1, || true);
    } else if m & 0x200 != 0 || segunda(clube) != 0 {
        cpx13_elenco(this, p, clube, reg, 2, || {
            cpx13_d(p) == cpx13_d(0x9cf7a0) || cpx13_b(cpx13_d(cpx13_d(clube + 0x53)).wrapping_add(cpx13_d(this + 0x36))) == 0
        });
    }
}

/// O que a 0x76d880 grava no registro (fora das pessoas): depois da data do sub-21 (+0x62/+0x66, 0x456740 sobre
/// hoje), zera a marca +0x6b.
unsafe fn cpx13_dispensar(this: usize, clube: usize) {
    if !CPX13_ENSAIO {
        let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76d880usize);
        return dispensar(this, cpx13_d(clube + 0x53));
    }
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let reg = cpx13_registro(this, clube);
    if depois(0xae2c90, cpx13_d(reg + 0x62) as u32, cpx13_d(reg + 0x66) as u32) != 0 {
        *((reg + 0x6b) as *mut u8) = 0;
    }
}

/// { 0x76bab0 } (thiscall, ret 4, national_teams.cpp, quadro SEH sem destrutor): `NationalTeams::select_squads_for_fixture
/// (partida)`: sem a marca +0x40 com o bit 1 de +0x4e, ou mandante seleção B (0x5409b0) com o bit 2, nada. Só a
/// partir de duas semanas antes do jogo (0x54cab0 sobre a data +0x2a/+0x28). Mandante: fora de seleção B, o elenco
/// principal (cpx13_elenco tipo 0) e, sem o bit 2, a dispensa do sub-21 (0x76d880) das duas nações; os elencos
/// menores (cpx13_elencos_menores). Visitante: o mesmo sem a dispensa. Por fim apaga os repetidos (0x76c060) das duas.
unsafe extern "thiscall" fn convocar_selecoes_para_o_jogo(this: usize, p: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let semanas_antes: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x76c060usize);
    if cpx13_b(p + 0x40) == 0 && cpx13_b(p + 0x4e) & 1 != 0 {
        return;
    }
    if segunda(cpx13_d(p + 0x1c)) != 0 && cpx13_b(p + 0x4e) & 2 != 0 {
        return;
    }
    let mut jogo = [0u8; 8];
    data_nova(jogo.as_mut_ptr(), cpx13_w(p + 0x2a) as u16 as u32, cpx13_w(p + 0x28) as u16 as u32);
    let (hd, ha) = (cpx13_w(0xae2c90), cpx13_w(0xae2c92) as u16);
    let mut r = [0u8; 8];
    let q = semanas_antes(jogo.as_mut_ptr(), r.as_mut_ptr(), 2); // cpx13: a janela
    let (qa, qd) = (cpx13_w(q as usize + 2) as u16, cpx13_w(q as usize));
    if ha < qa || (ha == qa && hd < qd) {
        return;
    }
    let (casa, fora) = (cpx13_d(p + 0x1c), cpx13_d(p + 0x20));
    let reg = cpx13_registro(this, casa);
    if segunda(casa) == 0 {
        cpx13_elenco(this, p, casa, reg, 0, || true);
        if cpx13_b(p + 0x4e) & 2 == 0 {
            cpx13_dispensar(this, cpx13_d(p + 0x1c));
            cpx13_dispensar(this, cpx13_d(p + 0x20));
        }
    }
    cpx13_elencos_menores(this, p, casa, reg);
    let reg = cpx13_registro(this, fora);
    if segunda(fora) == 0 {
        cpx13_elenco(this, p, fora, reg, 0, || true);
    }
    cpx13_elencos_menores(this, p, fora, reg);
    if !CPX13_ENSAIO {
        repetidos(cpx13_d(cpx13_d(p + 0x1c) + 0x53));
        repetidos(cpx13_d(cpx13_d(p + 0x20) + 0x53));
    }
}

/// Comparador: a original roda antes; a cópia depois, em ensaio, a partir dos mesmos registros das duas nações (+0x32
/// a +0x6c: marcas de hoje, datas dos elencos e marcas de convocado). Se a cópia pediu algum elenco (0x76c120 aloca e
/// mexe nos elencos), a comparação não vale e fica a original; senão os registros têm de bater.
unsafe extern "thiscall" fn comparar_convocar_selecoes_para_o_jogo(this: usize, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1341);
    let regioes = [(cpx13_registro(this, cpx13_d(p + 0x1c)) + 0x32, 0x3a), (cpx13_registro(this, cpx13_d(p + 0x20)) + 0x32, 0x3a)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    o(this, p);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    CPX13_ENSAIO = true;
    CPX13_PEDIU = false;
    crate::aleatorio::pausar_rastro(|| convocar_selecoes_para_o_jogo(this, p));
    CPX13_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    if !CPX13_PEDIU {
        PLACAR.registrar(mo == mr, || format!("convocar_selecoes_para_o_jogo({:x}, {:x}) original={:x?} rust={:x?}", this, p, mo, mr));
    }
}

/// O clube do contrato da pessoa (0x4cf5a0 de [0xae1050], gravado em [0xae1080]; clube +4 em [0xae23b4], 0x245
/// bytes), ou 0.
unsafe fn cpx13_clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpx13_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    cpx13_d(0xae23b4).wrapping_add(cpx13_d(c + 4).wrapping_mul(0x245))
}

/// O ensaio do comparador da 0x76fd10: a cópia roda sem entregar a notícia.
static mut CPX13_ENSAIO_NOTICIA: bool = false;

/// { 0x76fd10 } (stdcall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor):
/// `national_teams_post_player_called_up_news(jogador, nação, vaga)`: só com técnico humano (0x7875e0) no clube +0x39
/// do jogador ou no clube do contrato (que precisa existir). A notícia 0x21 (local): 0 o id, 1..3 os +0x33 de +4, +8,
/// +0xc; vaga 0: 4 a seleção principal (0x53b3d0), 5 = 0; vaga 1: a principal e 1; outra: a B (0x53b4f0) e 0.
/// Entregue ao técnico (0x7861e0) do +0x39, se houver, e ao do clube do contrato quando é outro.
unsafe extern "stdcall" fn noticia_do_convocado(jogador: usize, nacao: usize, vaga: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let c39 = cpx13_d(jogador + 0x39);
    if c39 == 0 || humano(c39) == 0 {
        if cpx13_clube_do_contrato(jogador) == 0 {
            return;
        }
        if humano(cpx13_clube_do_contrato(jogador)) == 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpx_13 noticia do convocado");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x21, 0);
    gravar(n, 0, cpx13_d(jogador) as u32);
    gravar(n, 1, cpx13_d(cpx13_d(jogador + 4) + 0x33) as u32);
    gravar(n, 2, cpx13_d(cpx13_d(jogador + 8) + 0x33) as u32);
    gravar(n, 3, cpx13_d(cpx13_d(jogador + 0xc) + 0x33) as u32);
    match vaga as u8 {
        0 => {
            gravar(n, 4, cpx13_d(principal(nacao)) as u32);
            gravar(n, 5, 0);
        }
        1 => {
            gravar(n, 4, cpx13_d(principal(nacao)) as u32);
            gravar(n, 5, 1);
        }
        _ => {
            gravar(n, 4, cpx13_d(b(nacao)) as u32);
            gravar(n, 5, 0);
        }
    }
    let noticias = cpx13_d(0xb7431c);
    let c39 = cpx13_d(jogador + 0x39);
    if c39 != 0 && !CPX13_ENSAIO_NOTICIA {
        ao_clube(noticias, n, c39);
    }
    if cpx13_clube_do_contrato(jogador) == cpx13_d(jogador + 0x39) {
        return;
    }
    let c = cpx13_clube_do_contrato(jogador);
    if !CPX13_ENSAIO_NOTICIA {
        ao_clube(noticias, n, c);
    }
}

/// Comparador: a cópia em ensaio (sem entregar) e a original a partir do mesmo [0xae1080] (o último contrato lido, que
/// as duas gravam).
unsafe extern "stdcall" fn comparar_noticia_do_convocado(jogador: usize, nacao: usize, vaga: u32) {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, u32)>(1342);
    let regioes = [(0xae1080usize, 4usize)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX13_ENSAIO_NOTICIA = true;
    crate::aleatorio::pausar_rastro(|| noticia_do_convocado(jogador, nacao, vaga));
    CPX13_ENSAIO_NOTICIA = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(jogador, nacao, vaga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("noticia_do_convocado({:x}, {:x}, {}) original={:x?} rust={:x?}", jogador, nacao, vaga, mo, mr));
}

/// { 0x519690 } (thiscall, ret 0x14, Cup.cpp, quadro SEH sem destrutor): `Cup::add_team_result_news(clube, desfecho,
/// a3, partida, a5)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o clube, o
/// desfecho, a3, a saída (0xff), a partida e a5; −1 dela ou sem partida: 0. A notícia 0x10 (local): 0 o clube, 1 a
/// competição, 2 o desfecho, 3 a categoria do resultado (0x4b0890), 4 o que a virtual devolveu, o jogo a partir do
/// dado 5 (0x4afeb0, que avança o cursor) e no cursor a rodada seguinte (+7 da rodada +0x34 + 1, até +0x3c − 1; 0x8c
/// ou fora: −1). Saída ≠ 0xff: a entrega pelo alcance (0x786090). Desfecho 1 na rodada 0x96 ou 3 na 0x8c (+0x30): à
/// região da competição (0x786860); senão ao técnico do clube (0x7861e0). Devolve 1.
unsafe extern "thiscall" fn noticia_do_desfecho_na_copa(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let categoria: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b0890usize);
    let gravar_jogo: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x4afeb0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_regiao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x786860usize);
    let pelo_alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    // ponytail: a original grava só o byte da saída num dword da pilha e empurra o dword; a 0x786090 lê o byte
    let mut saida: u32 = 0xff;
    if amistoso(cpx13_d(0xae2c74), cpx13_d(copa + 4)) != 0 {
        return 0;
    }
    let comp = cpx13_d(cpx13_d(0xadadfc).wrapping_add(cpx13_d(cpx13_d(copa + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, usize, u32) -> i32 = core::mem::transmute(cpx13_d(cpx13_d(comp) + 0x44));
    let r = virtual44(comp, clube, desfecho, a3, &mut saida, p, a5);
    if r == -1 || p == 0 {
        return 0;
    }
    crate::comparacao::ramo("cpx_13 noticia do desfecho");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x10, 0);
    gravar(n, 0, cpx13_d(clube) as u32);
    gravar(n, 1, cpx13_d(cpx13_d(copa + 4)) as u32);
    gravar(n, 2, desfecho as u8 as i8 as i32 as u32);
    gravar(n, 3, categoria(clube, p) as i8 as i32 as u32);
    gravar(n, 4, r as u32);
    // ponytail: o cursor é um dword da pilha com 5 no byte baixo (o resto é sobra); só o byte baixo conta
    let mut cursor = [5u8, 0, 0, 0];
    gravar_jogo(p, n, cursor.as_mut_ptr());
    let k = cpx13_w(p + 0x34) as i32;
    let proxima = if k < cpx13_w(copa + 0x3c) as i32 - 1 {
        let v = cpx13_w(cpx13_d(copa + 0xa3).wrapping_add((k as usize).wrapping_mul(0x68)) + 0x6f);
        if v != 0x8c { v as i32 } else { -1 }
    } else {
        -1
    };
    gravar(n, cursor[0] as i8, proxima as u32);
    let noticias = cpx13_d(0xb7431c);
    if saida as u8 != 0xff {
        let nacao = cpx13_d(clube + 0x53);
        let continente = if nacao != 0 { cpx13_d(nacao + 0x71) } else { 0 };
        pelo_alcance(noticias, n, clube, cpx13_d(copa + 4), nacao, continente, saida);
        return 1;
    }
    let rodada = cpx13_w(p + 0x30);
    if (desfecho as u8 == 1 && rodada == 0x96) || (desfecho as u8 == 3 && rodada == 0x8c) {
        a_regiao(noticias, n, cpx13_d(copa + 4));
    } else {
        ao_clube(noticias, n, clube);
    }
    1
}

// ponytail: a virtual +0x44 da copa acrescenta registros às pessoas e a notícia vai às caixas dos técnicos (alocam): só
// a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_desfecho_na_copa(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(1343)(copa, clube, desfecho, a3, p, a5)
}

// ---------- 259a leva: o jogo entre seleções: as convocações, os jogos e gols pela seleção, o nome do elenco e as rodadas conhecidas (national_teams.cpp, comp.cpp) ----------

unsafe fn cpw12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O ensaio do comparador da 0x76b600: a cópia roda sem o ranking da FIFA (0x58cf70, acrescenta pontos ao histórico),
/// sem a primeira convocação (0x86cc50, vai para o arquivo), sem o jogo pela seleção no registro de transferências
/// (0x8ce920) e sem a notícia aos clubes (0x7706c0, aloca).
static mut CPW12_ENSAIO: bool = false;

/// A vaga `k` (0..20) da escalação do lado `lado` no resumo do jogo `m`: m + 0x625 × lado + 0x190 + 0x49 × k.
fn cpw12_vaga(m: usize, lado: usize, k: usize) -> usize {
    m.wrapping_add(0x625 * lado + 0x190 + 0x49 * k)
}

/// A pessoa da vaga, se a vaga conta (id ≠ −1 e titular, ou reserva que entrou: +0x32 ≠ 0xff) e a pessoa está na
/// faixa (id < [0xae2364]; o ponteiro nulo passa sem o teste, como na original).
unsafe fn cpw12_pessoa(vaga: usize, k: usize) -> Option<usize> {
    let id = cpw12_d(vaga) as i32;
    if id == -1 || (k >= 11 && *((vaga + 0x32) as *const u8) == 0xff) {
        return None;
    }
    let p = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if p != 0 && cpw12_d(p) as i32 >= cpw12_d(0xae2364) as i32 {
        return None;
    }
    Some(p)
}

/// O registro geral da pessoa (79 bytes em [0xae2d0c]).
unsafe fn cpw12_geral(p: usize) -> usize {
    cpw12_d(0xae2d0c).wrapping_add(cpw12_d(p).wrapping_mul(0x4f))
}

/// O registro da nação do clube nas seleções: [this + 4] + 0x6e × id.
unsafe fn cpw12_nacao(this: usize, clube: usize) -> usize {
    cpw12_d(this + 4).wrapping_add(cpw12_d(cpw12_d(clube + 0x53)).wrapping_mul(0x6e))
}

/// Jogou pela nação do clube: se a segunda nação (+0x1e) é a dele, ela vira a primeira (+0x1a) e a antiga a segunda.
unsafe fn cpw12_trocar_nacao(p: usize, clube: usize) {
    if cpw12_d(clube + 0x53) == cpw12_d(p + 0x1e) {
        core::ptr::write_unaligned((p + 0x1e) as *mut usize, cpw12_d(p + 0x1a));
        core::ptr::write_unaligned((p + 0x1a) as *mut usize, cpw12_d(clube + 0x53));
    }
}

/// O jogo de nação por lado (o seu código de competição em [p + 0x14]): conta o jogo da nação (+2 do registro dela,
/// curto) e cada pessoa que conta na escalação.
unsafe fn cpw12_lados(this: usize, p: usize, m: usize, mut f: impl FnMut(usize, usize, usize, usize)) {
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
        core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            if let Some(pessoa) = cpw12_pessoa(vaga, k) {
                f(clube, vaga, k, pessoa);
            }
        }
    }
}

/// { 0x76b600 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::record_international_result(partida, resumo)`:
/// só entre duas seleções (0x53b900). Pela competição da partida ([+0x14]): a de [0x9cf794] marca +0xd = 1 no registro
/// geral de quem jogou; a de [0x9cf798] liga o bit 2 do +0xc e troca a nação (+0x1a/+0x1e); a de [0x9cf7a0] nada; as
/// outras: o ranking da FIFA (0x58cf70 em [this+0x1c]) e, de quem jogou, a primeira convocação (0x86cc50 com o
/// adversário, se +0x22 = 0, depois da troca da nação), um jogo (+0x22) e os gols (+0x23 += +0x21 da vaga) pela
/// seleção e o jogo no registro (0x8ce920 com o número de jogos da nação). Em todas, o jogo da nação (+2) sobe. Depois,
/// por lado: o tipo (1 se +0x40 = 0; senão 2 na segunda seleção 0x5409b0, senão 0); na data do tipo (+0x3a + 20 ×
/// tipo, 0x4a7f20 com a de hoje) zera +0x44 + 20 × tipo; a notícia aos clubes (0x7706c0) com o destaque (vaga +0x153) e
/// a lista de quem jogou. Sem SEH, sem caixa, sem sorteio.
unsafe extern "thiscall" fn resultado_entre_selecoes(this: usize, p: usize, m: usize) {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if e_selecao(cpw12_d(p + 0x1c)) == 0 || e_selecao(cpw12_d(p + 0x20)) == 0 {
        return;
    }
    let comp = cpw12_d(cpw12_d(p + 0x14));
    if comp == cpw12_d(0x9cf794) {
        cpw12_lados(this, p, m, |_, _, _, pessoa| *((cpw12_geral(pessoa) + 0xd) as *mut u8) = 1);
    } else if comp == cpw12_d(0x9cf798) {
        cpw12_lados(this, p, m, |clube, _, _, pessoa| {
            *((cpw12_geral(pessoa) + 0xc) as *mut u8) |= 2;
            cpw12_trocar_nacao(pessoa, clube);
        });
    } else if comp != cpw12_d(0x9cf7a0) {
        let fifa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x58cf70usize);
        let primeira: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86cc50usize);
        let jogo_pela_selecao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8ce920usize);
        if !CPW12_ENSAIO {
            fifa(cpw12_d(this + 0x1c), p);
        }
        for lado in 0..2usize {
            let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
            let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
            core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
            for k in 0..20usize {
                let vaga = cpw12_vaga(m, lado, k);
                let id = cpw12_d(vaga) as i32;
                if id == -1 {
                    continue;
                }
                if k >= 11 && *((vaga + 0x32) as *const u8) == 0xff { // cpw12: a reserva que não entrou
                    crate::comparacao::ramo("cpw_12 reserva sem entrar");
                    continue;
                }
                let pessoa = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
                if pessoa != 0 && cpw12_d(pessoa) as i32 >= cpw12_d(0xae2364) as i32 {
                    continue;
                }
                if *((pessoa + 0x22) as *const u8) == 0 {
                    cpw12_trocar_nacao(pessoa, clube);
                    let adversario = if clube == cpw12_d(p + 0x1c) { cpw12_d(p + 0x20) } else { cpw12_d(p + 0x1c) };
                    if !CPW12_ENSAIO {
                        primeira(cpw12_d(0xdd7f00), pessoa, adversario);
                    }
                }
                crate::comparacao::ramo("cpw_12 jogo pela seleção");
                *((pessoa + 0x22) as *mut u8) = (*((pessoa + 0x22) as *const u8)).wrapping_add(1); // cpw12: um jogo
                *((pessoa + 0x23) as *mut u8) = (*((pessoa + 0x23) as *const u8)).wrapping_add(*((vaga + 0x21) as *const u8)); // cpw12: os gols
                if !CPW12_ENSAIO {
                    // a original empurra o edx com o curto dos jogos embaixo e o alto de 55 × id da nação
                    let id_nacao = cpw12_d(cpw12_d(clube + 0x53)) as u32;
                    let n = core::ptr::read_unaligned(jogos) as u16 as u32;
                    jogo_pela_selecao(cpw12_d(0xde1f54), pessoa, (id_nacao.wrapping_mul(0x37) & 0xffff_0000) | n);
                }
            }
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32) = core::mem::transmute(0x7706c0usize);
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let b = *((p + 0x40) as *const u8) == 0;
        let tipo: usize = if segunda(clube) == 0 { if b { 1 } else { 0 } } else if b { 1 } else { 2 };
        let reg = cpw12_nacao(this, clube) + 20 * tipo;
        if mesma_data(reg + 0x3a, cpw12_d(0xae2c90) as u32, cpw12_d(0xae2c94) as u32) != 0 {
            *((reg + 0x44) as *mut u8) = 0;
        }
        let mut lista = [0usize; 20];
        let mut q = 0usize;
        for k in 0..20usize {
            if let Some(pessoa) = cpw12_pessoa(cpw12_vaga(m, lado, k), k) {
                lista[q] = pessoa;
                q += 1;
            }
        }
        let destaque_k = *((m + 0x625 * lado + 0x153) as *const i8) as isize;
        let destaque_id = cpw12_d((m + 0x625 * lado + 0x190) .wrapping_add((0x49 * destaque_k) as usize));
        let destaque = cpw12_d(0xae23bc).wrapping_add(destaque_id.wrapping_mul(0x6e));
        if !CPW12_ENSAIO {
            // ponytail: a original empurra a conta num dword da local com só o byte de baixo gravado; a 0x7706c0 lê o byte
            noticia(this, clube, p, m, destaque, lista.as_mut_ptr(), q as u32);
        }
    }
}

/// As regiões que a 0x76b600 grava por conta própria: de cada lado, o registro da nação (o +2 e os +0x44 dos tipos) e,
/// de cada vaga com pessoa na faixa, os campos +0x1a..+0x24 dela e o +0xc/+0xd do registro geral.
unsafe fn cpw12_regioes(this: usize, p: usize, m: usize) -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let r = cpw12_nacao(this, clube);
        reg.extend([(r + 2, 2), (r + 0x44, 1), (r + 0x58, 1), (r + 0x6c, 1)]);
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            let id = cpw12_d(vaga) as i32;
            if id == -1 {
                continue;
            }
            let pessoa = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            if pessoa != 0 && (cpw12_d(pessoa) as i32) < cpw12_d(0xae2364) as i32 {
                reg.push((pessoa + 0x1a, 10));
                reg.push((cpw12_geral(pessoa) + 0xc, 2));
            }
        }
    }
    reg
}

/// Comparador: o ensaio (a cópia sem o ranking, a primeira convocação, o registro de transferências e a notícia, que
/// alocam, vão para o arquivo ou gravam fora) e a original a partir do mesmo estado, conferidos nas regiões que a
/// rotina grava por conta própria (cpw12_regioes). Fora de jogo entre seleções: só a original.
unsafe extern "thiscall" fn comparar_resultado_entre_selecoes(this: usize, p: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1302);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if e_selecao(cpw12_d(p + 0x1c)) == 0 || e_selecao(cpw12_d(p + 0x20)) == 0 {
        return o(this, p, m); // ponytail: não grava nada, só a original
    }
    let regioes = cpw12_regioes(this, p, m);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW12_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| resultado_entre_selecoes(this, p, m)));
    CPW12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, p, m);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("resultado_entre_selecoes(partida {:x}, resumo {:x}) original={:x?} rust={:x?}", p, m, mo, mr));
}

/// { 0x775610 } (thiscall, ret 0x14, national_teams.cpp; `this` não é lido): `NationalTeams::format_squad_label(tipo,
/// sub23, saida, tamanho, minusculo)`: tamanho abaixo de 20: a caixa (linha 0x1c98). Escreve (0x66f4e0) o nome do
/// elenco: tipo 0 "Main", 1 "'B'", outros "Under 23" (com sub23) ou "Under 21"; com minúsculo = 1, em minúsculas.
unsafe extern "thiscall" fn nome_do_elenco_da_selecao(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    if tamanho < 0x14 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(1303)(this, tipo, sub23, saida, tamanho, minusculo);
    }
    let texto: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(0x66f4e0usize);
    let s = match (minusculo as u8 == 1, tipo as u8, sub23 != 0) {
        (true, 0, _) => 0x9cd9b4,
        (true, 1, _) => 0xa62fa0,
        (true, _, true) => 0xa62f94,
        (true, _, false) => 0xa62f88,
        (false, 0, _) => 0xa62f80,
        (false, 1, _) => 0xa62fa0,
        (false, _, true) => 0xa62f74,
        (false, _, false) => 0xa62f68,
    };
    texto(saida, s)
}

/// Comparador: lado a lado no começo da saída (até 64 bytes) e na devolução. A caixa: só a original.
unsafe extern "thiscall" fn comparar_nome_do_elenco_da_selecao(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(1303);
    if tamanho < 0x14 || saida == 0 {
        return o(this, tipo, sub23, saida, tamanho, minusculo); // ponytail: a caixa, só a original
    }
    let reg = [(saida, (tamanho as usize).min(0x40))];
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tipo, sub23, saida, tamanho, minusculo),
        || nome_do_elenco_da_selecao(this, tipo, sub23, saida, tamanho, minusculo));
    PLACAR.registrar(igual && ro == rr, || format!("nome_do_elenco_da_selecao({}, {}, {}) original={:x} {:x?} rust={:x} {:x?}", tipo as u8, sub23, minusculo as u8, ro, mo, rr, mr));
    ro
}

/// { 0x48d410 } (thiscall, ret 8, comp.cpp): `Comp::are_round_fixtures_known(fase, rodada)`: a fase (−1 = a própria
/// competição; acima de +0x30: 0; abaixo de −1: a caixa da linha 0x36e, pela original) é liga (+0x43 = 2): 1. Senão 1
/// se a rodada é antes da atual (+0xab, byte) ou é a atual, a última (+0x3c − 1) e com o código 0x8c (+7 do registro
/// de 0x68 bytes dela em +0xa3); senão 0.
unsafe extern "thiscall" fn rodada_com_jogos_conhecidos(this: usize, fase: i32, rodada: i32) -> u32 {
    if fase > cpw12_d(this + 0x30) as i32 {
        return 0;
    }
    if fase < -1 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1304)(this, fase, rodada);
    }
    let c = if fase == -1 { this } else { cpw12_d(cpw12_d(this + 0xc).wrapping_add(4 * fase as usize)) };
    if *((c + 0x43) as *const u8) == 2 {
        return 1;
    }
    let atual = *((c + 0xab) as *const i8) as i32;
    if rodada < atual {
        return 1;
    }
    if rodada != atual {
        return 0;
    }
    let n = core::ptr::read_unaligned((c + 0x3c) as *const i16) as i32;
    if atual != n - 1 {
        return 0;
    }
    let cod = core::ptr::read_unaligned((cpw12_d(c + 0xa3).wrapping_add((0x68 * (n - 1)) as usize) + 7) as *const i16);
    (cod == 0x8c) as u32
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
unsafe extern "thiscall" fn comparar_rodada_com_jogos_conhecidos(this: usize, fase: i32, rodada: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1304);
    if fase < -1 && fase <= cpw12_d(this + 0x30) as i32 {
        return o(this, fase, rodada); // ponytail: a caixa, só a original
    }
    let r = o(this, fase, rodada);
    let n = rodada_com_jogos_conhecidos(this, fase, rodada);
    PLACAR.registrar(r == n, || format!("rodada_com_jogos_conhecidos({:x}, {}, {}) original={} rust={}", this, fase, rodada, r, n));
    r
}

// ---------- 258a leva: o convocado machucado: tirar da seleção, a notícia ao clube dele e o pedido de substituto ao técnico da seleção (national_teams.cpp) ----------

unsafe fn cpv11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id.
unsafe fn cpv11_registro(this: usize, nacao: usize) -> usize {
    cpv11_d(this + 4).wrapping_add((cpv11_d(nacao) as i32).wrapping_mul(0x6e) as usize)
}

/// O tipo de convocação do jogador no clube da seleção `clube`: 2 no segundo conjunto de seleções (0x5409b0), senão o
/// bit 3 das marcas de elenco (+0x5e).
unsafe fn cpv11_tipo(clube: usize, p: usize) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(clube) != 0 {
        2
    } else {
        (cpv11_b(p + 0x5e) >> 3) & 1 // cpv11: o tipo pelas marcas
    }
}

/// Os dados 0..3 da notícia: o id do jogador e os de +4, +8 e +0xc dele (+0x33 de cada).
unsafe fn cpv11_quem(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, 0, cpv11_d(p) as u32);
    for k in 0..3usize {
        gravar(n, 1 + k as i8, cpv11_d(cpv11_d(p + 4 + 4 * k) + 0x33) as u32);
    }
}

/// O dado 6 da notícia: a gravidade da lesão do jogador (0x62ec00 sobre o tipo do registro 0x62ed70), ou 0.
unsafe fn cpv11_gravidade(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let gravidade: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x62ec00usize);
    let v = if lesao(cpv11_d(0xb63d28), p) != 0 {
        let t = cpv11_b(lesao(cpv11_d(0xb63d28), p)) as i8;
        gravidade(cpv11_d(0xb63d28), t) as i8 as i32
    } else {
        0
    };
    gravar(n, 6, v as u32);
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0.
unsafe fn cpv11_clube_do_contrato(p: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv11_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        0
    } else {
        cpv11_d(0xae23b4).wrapping_add(cpv11_d(c + 4).wrapping_mul(0x245))
    }
}

/// { 0x775cd0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::withdraw_injured_player(p, dias)`: com 14 dias
/// de lesão ou mais, o convocado (+0x28 = 0xb) do clube da seleção (+0x24) sai: com a lesão do tipo 0x51 só sai da
/// tática (0x896c60, duas vezes) e do elenco (0x7762f0). Senão, só antes da data do tipo (+0x3a do registro do tipo:
/// [this+4] + 0x6e × id da nação + 20 × tipo; 0x456740 sobre [0xae2c90]) e se a lista [this] não tem o clube com uma
/// data já passada: a notícia ao clube dele (0x76f7d0), a saída, a vaga do tipo (+0x45) sobe 1, o nível (0x76d810) e,
/// se o técnico da seleção é humano e decide (0x600a50, 0x5fd6b0), o pedido de substituto (0x7701f0); senão o
/// substituto (0x775f90) e as convocações repetidas apagadas (0x76c060). Sem clube: a original (caixa e 0).
unsafe extern "thiscall" fn tirar_convocado_machucado(this: usize, p: usize, dias_arg: u32) {
    let dias = dias_arg as u16 as i16;
    if dias < 0xe { // cpv11: o limite de dias
        return;
    }
    let clube = cpv11_d(p + 0x24);
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1263)(this, p, dias_arg);
    }
    if cpv11_b(p + 0x28) != 0xb {
        return;
    }
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7762f0usize);
    let maior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x76f7d0usize);
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x76d810usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(0x5fd6b0usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x7701f0usize);
    let substituto: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) = core::mem::transmute(0x775f90usize);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x76c060usize);
    let nacao = cpv11_d(clube + 0x53);
    let rec = cpv11_registro(this, nacao);
    // ponytail: a original empurra o dword da local com o tipo só no byte de baixo (os outros são sobra de pilha); as
    // chamadas leem só o byte
    let tipo = cpv11_tipo(clube, p);
    let inj = lesao(cpv11_d(0xb63d28), p);
    if inj != 0 && cpv11_b(inj) == 0x51 {
        crate::comparacao::ramo("cpv_11 doença");
        marcar(cpv11_d(0xde1d08), clube, p, 1, 1);
        marcar(cpv11_d(0xde1d08), clube, p, 0, 1);
        tirar(this, nacao, tipo as u32, p);
        return;
    }
    let vaga = rec.wrapping_add((tipo as i8 as i32 * 20) as usize);
    if maior(vaga + 0x3a, cpv11_d(0xae2c90) as u32, cpv11_d(0xae2c94) as u32) == 0 {
        return;
    }
    let lista = cpv11_d(this);
    if lista != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpv11_d(lista + 2) as u32);
        let mut e = proximo(cpv11_d(this));
        while e != 0 {
            if cpv11_d(e) == cpv11_d(clube) && maior(0xae2c90, cpv11_d(e + 8) as u32, cpv11_d(e + 0xc) as u32) != 0 {
                return;
            }
            e = proximo(cpv11_d(this));
        }
    }
    if dias < 0x1c {
        crate::comparacao::ramo("cpv_11 lesão de 14 a 27 dias");
    }
    noticia(this, p, nacao, tipo as u32);
    marcar(cpv11_d(0xde1d08), clube, p, 1, 1);
    marcar(cpv11_d(0xde1d08), clube, p, 0, 1);
    if tipo != 2 {
        crate::comparacao::ramo("cpv_11 tipo pelas marcas");
    }
    tirar(this, nacao, tipo as u32, p);
    let a = (vaga + 0x45) as *mut u8;
    *a = (*a).wrapping_add(1);
    // a base vai no eax da original: o endereço da vaga com o byte de baixo trocado por (tipo == 2)
    let nivel = nivel_de(p, rec, (vaga as u32 & !0xff) | (tipo == 2) as u32);
    // a local do nível é a do registro com o byte de baixo trocado (só o byte é lido)
    let nivel = (rec as u32 & !0xff) | nivel as u32;
    let pede = 'decide: {
        if humano(clube, 0, 1, 0, 0) == 0 {
            if cpv11_d(clube + 0xcf) != 0 || humano(principal(nacao), 0, 1, 0, 0) == 0 {
                break 'decide false;
            }
        }
        let t = cpv11_d(principal(nacao) + 0xcf);
        tipo == 0 || t9a06(cpv11_d(0xb63c98), t, core::ptr::null_mut()) != 0
    };
    if pede {
        crate::comparacao::ramo("cpv_11 pedido de substituto");
        pedido(this, p, nacao, tipo as u32, nivel);
        return;
    }
    crate::comparacao::ramo("cpv_11 substituto");
    substituto(this, nacao, tipo as u32, nivel, p); // cpv11: o substituto
    repetidos(nacao);
}

/// O caminho que a 0x775cd0 vai tomar, só lendo (a lista [this] percorrida pelos nós, sem mexer no cursor): 0 nada, 1
/// doença (só sai), 2 sai com notícia. O clube (+0x24) já conferido.
unsafe fn cpv11_plano(this: usize, p: usize, dias: u32) -> u8 {
    if (dias as u16 as i16) < 0xe || cpv11_b(p + 0x28) != 0xb {
        return 0;
    }
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let maior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let clube = cpv11_d(p + 0x24);
    let tipo = cpv11_tipo(clube, p);
    let inj = lesao(cpv11_d(0xb63d28), p);
    if inj != 0 && cpv11_b(inj) == 0x51 {
        return 1;
    }
    let vaga = cpv11_registro(this, cpv11_d(clube + 0x53)).wrapping_add((tipo as i8 as i32 * 20) as usize);
    if maior(vaga + 0x3a, cpv11_d(0xae2c90) as u32, cpv11_d(0xae2c94) as u32) == 0 {
        return 0;
    }
    let lista = cpv11_d(this);
    if lista != 0 {
        let mut no = cpv11_d(lista + 2);
        while no != 0 {
            let e = cpv11_d(no);
            if e == 0 {
                break;
            }
            if cpv11_d(e) == cpv11_d(clube) && maior(0xae2c90, cpv11_d(e + 8) as u32, cpv11_d(e + 0xc) as u32) != 0 {
                return 0;
            }
            no = cpv11_d(no + 6);
        }
    }
    2
}

/// { 0x76f7d0 } (thiscall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_player_withdrawn_news(p, nacao, tipo)`: se o clube do jogador (+0x39) ou o do contrato
/// (0x4cf5a0) é de técnico humano (0x7875e0), monta a notícia 0x20 numa local (0x77bbe0 com 0): dados 0..3 o jogador
/// e os de +4/+8/+0xc, 4 o id da seleção (principal 0x53b3d0 nos tipos 0 e 1, senão 0x53b4f0), 5 = (tipo == 1), 6 a
/// gravidade da lesão; entrega ao clube +0x39 e, se for outro, ao do contrato (0x7861e0).
unsafe extern "thiscall" fn noticia_da_saida_do_convocado(_this: usize, p: usize, nacao: usize, tipo: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    if cpv11_d(p + 0x39) == 0 || humano(cpv11_d(p + 0x39)) == 0 {
        if cpv11_clube_do_contrato(p) == 0 {
            return;
        }
        if humano(cpv11_clube_do_contrato(p)) == 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpv_11 notícia da saída");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x20, 0);
    cpv11_quem(n, p);
    let (sel, um) = match tipo as u8 {
        0 => (principal(nacao), 0),
        1 => (principal(nacao), 1),
        _ => (selecao_b(nacao), 0),
    };
    gravar(n, 4, cpv11_d(sel) as u32);
    gravar(n, 5, um);
    cpv11_gravidade(n, p);
    let noticias = cpv11_d(0xb7431c);
    if cpv11_d(p + 0x39) != 0 {
        entregar(noticias, n, cpv11_d(p + 0x39));
    }
    if cpv11_clube_do_contrato(p) != cpv11_d(p + 0x39) {
        entregar(noticias, n, cpv11_clube_do_contrato(p));
    }
}

/// { 0x7701f0 } (thiscall, ret 0x10, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_replacement_needed_news(p, nacao, tipo, nivel)`: a notícia 0x22 ao técnico da seleção: dados
/// 0..3 o jogador, 4 o id do time (tipo 1: principal com marca 8 e 5 = 1; tipo 0: principal, senão o sub-21 0x53b4f0,
/// marca 4 e 5 = 0), 6 a gravidade da lesão, 7 = 0, 8 os do elenco com a marca (0x53c590), 9 o nível, 0xa o tipo e 0xb
/// o id da nação. Entrega ao time se o técnico dele é humano (0x600a50), senão ao principal se o dele for.
unsafe extern "thiscall" fn noticia_do_pedido_de_substituto(_this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    crate::comparacao::ramo("cpv_11 notícia do pedido");
    let (time, um, marca) = match tipo as u8 {
        0 => (principal(nacao), 0u32, 4u32),
        1 => (principal(nacao), 1, 8),
        _ => (selecao_b(nacao), 0, 4),
    };
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x22, 0);
    cpv11_quem(n, p);
    gravar(n, 4, cpv11_d(time) as u32);
    gravar(n, 5, um);
    cpv11_gravidade(n, p);
    gravar(n, 7, 0);
    // ponytail: a marca vai num dword da local com só o byte de baixo gravado; a 0x53c590 lê só o byte
    gravar(n, 8, contar(time, marca, 0) as i8 as i32 as u32);
    gravar(n, 9, nivel as u8 as i8 as i32 as u32);
    gravar(n, 0xa, tipo as u8 as i8 as i32 as u32);
    gravar(n, 0xb, cpv11_d(nacao) as u32);
    let noticias = cpv11_d(0xb7431c);
    if humano(time, 0, 1, 0, 0) != 0 {
        entregar(noticias, n, time);
    } else if humano(principal(nacao), 0, 1, 0, 0) != 0 {
        entregar(noticias, n, principal(nacao));
    }
}

/// Comparador: a retirada chama a notícia (aloca), a saída da tática e o substituto (originais com efeitos): só a
/// original roda. O caminho é decidido antes, só lendo (cpv11_plano), e conferido depois: sem retirada, o clube da
/// seleção (+0x24) e a marca de convocado (+0x28) ficam; com retirada, o jogador que estava numa das 50 vagas do time
/// sai delas (+0x24 zerado pela 0x7762f0). Sem clube (a caixa): só a original.
unsafe extern "thiscall" fn comparar_tirar_convocado_machucado(this: usize, p: usize, dias: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1263);
    let clube = cpv11_d(p + 0x24);
    if (dias as u16 as i16) < 0xe || clube == 0 {
        return o(this, p, dias);
    }
    let plano = cpv11_plano(this, p, dias);
    let time = cpd2_time(cpv11_d(clube + 0x53), cpv11_tipo(clube, p));
    let estava = time != 0 && (0..50usize).any(|c| cpv11_d(time + 0xd7 + 4 * c) == p);
    let antes = (clube, cpv11_b(p + 0x28));
    o(this, p, dias);
    let depois = (cpv11_d(p + 0x24), cpv11_b(p + 0x28));
    let ok = match plano {
        0 => depois == antes,
        _ => !estava || depois.0 == 0,
    };
    PLACAR.registrar(ok, || format!("tirar_convocado_machucado({:x}, {}) plano {} estava {} antes {:x?} depois {:x?}", p, dias as u16 as i16, plano, estava, antes, depois));
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
unsafe extern "thiscall" fn comparar_noticia_da_saida_do_convocado(this: usize, p: usize, nacao: usize, tipo: u32) {
    crate::comparacao::ramo("noticia_da_saida_do_convocado");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1264)(this, p, nacao, tipo)
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
unsafe extern "thiscall" fn comparar_noticia_do_pedido_de_substituto(this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    crate::comparacao::ramo("noticia_do_pedido_de_substituto");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1265)(this, p, nacao, tipo, nivel)
}

// ---------- 258a leva: a convocação pode mudar agora?, o jogador está na convocação e o adversário marcado no amistoso (national_teams.cpp, friendly.cpp) ----------

unsafe fn cpv12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv12_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O nome do time (0x542100: time, marca de segundo time, destino de 0x47 bytes, 2, 1) e o byte de gramática (+0x52).
unsafe fn cpv12_nome_do_time(time: usize, segundo: u32, buf: &mut [u8; 0x48]) -> i32 {
    let nome: unsafe extern "C" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(0x542100usize);
    nome(time, segundo, buf.as_mut_ptr(), 0x47, 2, 1);
    cpv12_b(time + 0x52) as i8 as i32
}

/// { 0x76db00 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH com a lista local da busca de jogos):
/// `national_teams_can_change_squad(nacao, tipo, p, entrando, msg, tam)`: o jogador (se houver) aposentado da seleção
/// (0x547d30) ou de clube de outra nação não muda (0). O time do tipo (0 e 1 a principal, o 1 com a marca 8 e o segundo
/// time; senão o sub-21 0x53b4f0, marca 4). Entrando: elenco cheio (0x53c590 contra +0x42 do registro do tipo) ou
/// jogador machucado (0x62c400 = 2) não muda. Pode mudar (1): entrando com vagas abertas (+0x45), o tipo sem janela
/// (+0x44 = 0), a data do tipo (+0x3a) já passada ou, no próprio dia, um jogo do time hoje (0x5acee0 numa lista local);
/// nos tipos 1 e 2 com técnico humano que decide (0x600a50), também quando a lista [this] não tem o time com data
/// passada e ele não deixa (0x5fd6b0). Os motivos vão para `msg` (0x66f4e0). Sem nação, ou `msg` com menos de 0x32
/// bytes: a original (caixa e 1).
unsafe extern "thiscall" fn pode_mudar_a_convocacao(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(1266)(this, nacao, tipo, p, entrando, msg, tam);
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let aposentado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(0x5fd6b0usize);
    if p != 0 {
        if aposentado(p, principal(cpv12_d(p + 0x1a))) != 0 { // cpv12: aposentado
            crate::comparacao::ramo("cpv_12 aposentado");
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                texto(msg, 0xa5b4c8, nome.as_ptr());
            }
            return 0;
        }
        let c = cpv12_d(p + 0x24);
        if c != 0 && cpv12_d(c + 0x53) != nacao {
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                let outra = cpv12_d(cpv12_d(p + 0x24) + 0x53);
                let g = cpv12_b(outra + 0x52) as i8 as i32;
                texto(msg, 0xa5b460, g, g, nome.as_ptr(), outra + 0x38);
            }
            return 0;
        }
    }
    let s = tipo as u8;
    // a marca vai na local [esp+0x1c] só no byte de baixo (o resto é sobra de pilha); a 0x53c590 lê só o byte
    let (time, segundo, marca) = match s {
        0 => (principal(nacao), 0u32, 4u32),
        1 => (principal(nacao), 1, 8),
        _ => (selecao_b(nacao), 0, 4),
    };
    let mut tecnico = cpv12_d(principal(nacao) + 0xcf);
    if humano(principal(nacao), 0, 1, 0, 0) == 0 {
        tecnico = 0;
    }
    let rec = cpv12_d(this + 4).wrapping_add((cpv12_d(nacao) as i32).wrapping_mul(0x6e) as usize);
    let e = rec.wrapping_add((s as i8 as i32 * 20) as usize);
    if entrando != 0 {
        let teto = cpv12_b(e + 0x42) as i8;
        if (contar(time, marca, 0) as i8) >= teto {
            if msg != 0 {
                let mut buf = [0u8; 0x48];
                let g = cpv12_nome_do_time(time, segundo, &mut buf);
                texto(msg, 0xa5b3e0, g, g, teto as i32, buf.as_ptr());
            }
            return 0;
        }
        if estado(cpv12_d(0xb63d28), p) == 2 {
            if msg != 0 {
                texto(msg, 0xa5b3bc);
            }
            return 0;
        }
    }
    if cpv12_b(e + 0x45) != 0 && entrando != 0 {
        return 1;
    }
    if cpv12_b(e + 0x44) == 0 {
        crate::comparacao::ramo("cpv_12 sem janela");
        return 1; // cpv12: sem janela
    }
    let hoje = cpv12_d(0xae2c90) as u32;
    let (ano, dia) = ((hoje >> 16) as u16, hoje as u16 as i16);
    if s != 0 && tecnico != 0 {
        let mut passou = false;
        let lista = cpv12_d(this);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpv12_d(lista + 2) as u32);
            let mut it = proximo(cpv12_d(this));
            while it != 0 {
                if cpv12_d(it) == cpv12_d(time) {
                    let d = cpv12_d(it + 8) as u32;
                    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
                    if a < ano || (a == ano && dd < dia) {
                        passou = true;
                        break;
                    }
                }
                it = proximo(cpv12_d(this));
            }
        }
        if !passou && t9a06(cpv12_d(0xb63c98), tecnico, core::ptr::null_mut()) == 0 {
            return 1;
        }
    }
    let d = cpv12_d(e + 0x3a) as u32;
    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
    if a < ano || (a == ano && dd < dia) {
        crate::comparacao::ramo("cpv_12 data passada");
        return 1; // cpv12: data passada
    }
    if a == ano && dd as u16 == dia as u16 {
        crate::comparacao::ramo("cpv_12 jogo hoje");
        let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let buscar: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, u32, u32, u32, u32, usize, u32, i32, u32, u32) -> u16 =
            core::mem::transmute(0x5acee0usize);
        // as vagas da data que a busca lê (a palavra do ano, o dword do dia, o byte da fatia), como a original monta
        let mut v_ano: u32 = ano as u32;
        let mut v_dia: u32 = hoje;
        let mut fatia: u8 = 0;
        let mut l = [0u32; 4];
        let lp = l.as_mut_ptr() as usize;
        nova(lp);
        let achou = buscar(cpv12_d(0xae2a58), &mut v_ano, &mut v_dia, &mut fatia, 1, -1, -1, -1, cpv12_d(time) as u32, -1, 0, 0, 0, 1, lp, 1, -2,
            (segundo == 0) as u32, 0);
        destruir(lp);
        if achou != 0 {
            return 1;
        }
    }
    if msg != 0 {
        let mut buf = [0u8; 0x48];
        let g = cpv12_nome_do_time(time, segundo, &mut buf);
        texto(msg, if entrando != 0 { 0xa5b34c } else { 0xa5b2d8 }, g, g, buf.as_ptr());
    }
    0
}

/// { 0x76da80 } (stdcall, ret 0xc, national_teams.cpp): `national_teams_player_in_squad(p, nacao, tipo)`: 1 se o
/// clube da seleção do jogador (+0x24) é o time do tipo (0 e 1 a principal 0x53b3d0, 2 o sub-21 0x53b4f0) e ele tem a
/// marca do tipo (+0x5e: 4 nos tipos 0 e 2, 8 no 1). Sem clube ou outro tipo: 0.
unsafe extern "stdcall" fn esta_na_convocacao(p: usize, nacao: usize, tipo: u32) -> u32 {
    if cpv12_d(p + 0x24) == 0 {
        return 0;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let (time, marca) = match tipo as u8 {
        0 => (principal(nacao), 4u8),
        1 => (principal(nacao), 8),
        2 => (selecao_b(nacao), 4),
        _ => return 0,
    };
    (cpv12_d(p + 0x24) == time && cpv12_b(p + 0x5e) & marca != 0) as u32
}

/// { 0x5c57d0 } (thiscall, ret 8, friendly.cpp; `this` não é lido): `Friendly::schedule_assign_opponent(agenda,
/// adversario)`: na primeira das `[agenda+8]` (byte com sinal) vagas de 25 bytes (a partir de +0x19) sem adversário
/// (dword +0), grava o adversário e marca o pedido (+0x21 da vaga anterior, ou seja agenda + 25·i + 0x21).
unsafe extern "thiscall" fn marcar_adversario_do_amistoso(_this: usize, agenda: usize, adversario: usize) {
    let n = cpv12_b(agenda + 8) as i8;
    let mut i: i8 = 0;
    while i < n {
        let vaga = agenda + 25 * (i as usize + 1);
        if cpv12_d(vaga) == 0 {
            core::ptr::write_unaligned(vaga as *mut usize, adversario);
            *((agenda + 25 * i as usize + 0x21) as *mut u8) = 1;
            return;
        }
        i += 1;
    }
}

/// Comparador: lado a lado pela devolução e pelo motivo (a original no destino, a cópia num destino próprio do mesmo
/// tamanho). As duas andam o cursor da lista [this] do mesmo jeito; a busca de jogos do dia monta e libera uma lista
/// local (sem estado que fique). Sem nação ou destino curto (a caixa): só a original.
unsafe extern "thiscall" fn comparar_pode_mudar_a_convocacao(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(1266);
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return o(this, nacao, tipo, p, entrando, msg, tam);
    }
    let r = o(this, nacao, tipo, p, entrando, msg, tam);
    let mut meu: Vec<u8> = if msg != 0 { vec![0u8; tam as usize] } else { Vec::new() };
    let n = pode_mudar_a_convocacao(this, nacao, tipo, p, entrando, if msg != 0 { meu.as_mut_ptr() as usize } else { 0 }, tam);
    let texto = |a: *const u8, m: usize| -> Vec<u8> { (0..m).map(|i| *a.add(i)).take_while(|&c| c != 0).collect() };
    let (to, tr) = if msg != 0 { (texto(msg as *const u8, tam as usize), texto(meu.as_ptr(), tam as usize)) } else { (Vec::new(), Vec::new()) };
    PLACAR.registrar(r == n && to == tr, || format!("pode_mudar_a_convocacao({:x}, {}, {:x}, {}) {}/{} {:?} x {:?}", nacao, tipo as u8, p, entrando, r, n,
        String::from_utf8_lossy(&to), String::from_utf8_lossy(&tr)));
    r
}

/// Comparador: pura.
unsafe extern "stdcall" fn comparar_esta_na_convocacao(p: usize, nacao: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(1267)(p, nacao, tipo);
    let n = esta_na_convocacao(p, nacao, tipo);
    PLACAR.registrar(r == n, || format!("esta_na_convocacao({:x}, {:x}, {}) original={} rust={}", p, nacao, tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelas vagas da agenda (até a última possível, 25 · (n + 1) + 4 bytes).
unsafe extern "thiscall" fn comparar_marcar_adversario_do_amistoso(this: usize, agenda: usize, adversario: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1268);
    let n = cpv12_b(agenda + 8) as i8;
    if n <= 0 {
        return o(this, agenda, adversario);
    }
    let reg = [(agenda, 25 * (n as usize + 1) + 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, agenda, adversario), || marcar_adversario_do_amistoso(this, agenda, adversario));
    PLACAR.registrar(igual, || format!("marcar_adversario_do_amistoso({:x}, {:x}) {:x?} x {:x?}", agenda, adversario, mo, mr));
}

// ---------- 252a leva: as seleções e o calendário dos clubes: se o jogo do clube pode ser marcado com os convocados, a ordem das convocações pela data e a nação classificada para a fase final (national_teams.cpp) ----------

unsafe fn cpp12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Dia (palavra com sinal) e ano (palavra sem sinal) de uma data de 8 bytes.
unsafe fn cpp12_data(p: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p + 2) as *const u16))
}

/// Os convocados do clube (id < [0xae235c] − 2 × [0xae2350]: clube comum, não seleção) entre as datas `a` e `b`: a
/// lista do clube em [this + 0x20] + 6 × id (+0 os registros de 9 bytes em ordem de data, +5 a conta em byte com sinal);
/// pula os antes de `a`, para no primeiro depois de `b` e soma os bytes +8 (em byte, com a volta). Mais de 2: demais.
unsafe fn cpp12_muitos_convocados(this: usize, clube: usize, a: usize, b: usize) -> bool {
    if clube == 0 {
        return false;
    }
    let limite = cpp12_i(0xae235c).wrapping_sub(cpp12_i(0xae2350).wrapping_mul(2));
    let id = cpp12_i(clube);
    if id >= limite {
        return false;
    }
    let rec = cpp12_d(this + 0x20).wrapping_add((id as usize).wrapping_mul(6));
    let n = *((rec + 5) as *const i8);
    let lista = cpp12_d(rec);
    let (a_dia, a_ano) = cpp12_data(a);
    let (b_dia, b_ano) = cpp12_data(b);
    let mut soma: i8 = 0;
    let mut k: i8 = 0;
    while k < n {
        let e = lista.wrapping_add(k as isize as usize * 9);
        let (dia, ano) = cpp12_data(e);
        k = k.wrapping_add(1);
        if a_ano > ano || (a_ano == ano && a_dia > dia) {
            continue; // antes da janela
        }
        if b_ano < ano || (b_ano == ano && b_dia < dia) {
            break; // depois da janela: a lista está em ordem
        }
        crate::comparacao::ramo("cpp_12 convocado na janela");
        soma = soma.wrapping_add(*((e + 8) as *const i8));
    }
    soma > 2 // mais de dois convocados
}

/// { 0x776fc0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::can_schedule_club_fixture(jogo)`: com a data
/// do jogo (0x549ef0 com +0x2a e +0x28), a janela de um dia antes (0x54c8e0) e um depois (0x54c5b0); jogo da competição
/// amistosa (0x5c6710 em [0xae2c74] com +0x14): 1. Senão 0 quando o mandante (+0x1c) ou o visitante (+0x20) tem mais
/// de 2 convocados na janela; senão 1. O quadro SEH só guarda o estado (as datas não têm destrutor).
unsafe extern "thiscall" fn pode_marcar_jogo_do_clube(this: usize, jogo: usize) -> u32 {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(0x549ef0usize);
    let menos_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let (mut d, mut a, mut b) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // ponytail: a original empurra as palavras +0x2a e +0x28 com o alto do registro (lixo); a 0x549ef0 só lê as
    // palavras baixas: a cópia passa as palavras limpas.
    data_nova(d.as_mut_ptr(), *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    menos_dias(d.as_mut_ptr(), a.as_mut_ptr(), 1);
    mais_dias(d.as_mut_ptr(), b.as_mut_ptr(), 1);
    if amistosa(cpp12_d(0xae2c74), cpp12_d(jogo + 0x14)) != 0 {
        return 1;
    }
    let (pa, pb) = (a.as_ptr() as usize, b.as_ptr() as usize);
    if cpp12_muitos_convocados(this, cpp12_d(jogo + 0x1c), pa, pb) {
        return 0;
    }
    if cpp12_muitos_convocados(this, cpp12_d(jogo + 0x20), pa, pb) {
        crate::comparacao::ramo("cpp_12 visitante convocado");
        return 0; // o visitante com convocados demais
    }
    1
}

/// { 0x778e90 } (cdecl, national_teams.cpp): `compare_call_up_records_by_date(a, b)`: o qsort dos registros com a data
/// em +0 (as convocações de 9 bytes, as lesões, as transferências): −1 se a de `a` é antes (0x456770), 1 se é depois
/// (0x456740), senão 0. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_das_convocacoes_pela_data(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpp12_d(b) as u32, cpp12_d(b + 4) as u32);
    let r = if antes(a, d0, d1) != 0 { -1 } else { (depois(a, d0, d1) != 0) as i32 };
    r // a convocação mais cedo primeiro
}

/// { 0x7779b0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::add_qualified_nation(nacao, fase_final)`: com
/// as duas não nulas, acrescenta o registro de 8 bytes (nação, fase final) à lista das novas classificadas em
/// this + 0x28 (LLIST, 0x68d0d0).
unsafe extern "thiscall" fn acrescentar_nacao_classificada(this: usize, nacao: usize, fase_final: usize) {
    if nacao == 0 || fase_final == 0 {
        return;
    }
    let mut reg = [0u8; 8];
    reg[..4].copy_from_slice(&(nacao as u32).to_le_bytes());
    reg[4..].copy_from_slice(&(fase_final as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    acrescentar(this + 0x28, reg.as_mut_ptr() as usize, 8);
}

/// Comparador: pelo valor (só lê; as datas são locais).
unsafe extern "thiscall" fn comparar_pode_marcar_jogo_do_clube(this: usize, jogo: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1039)(this, jogo);
    let n = pode_marcar_jogo_do_clube(this, jogo);
    PLACAR.registrar(r == n, || format!("pode_marcar_jogo_do_clube({:x}) original={} rust={}", jogo, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_das_convocacoes_pela_data(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1040)(a, b);
    let n = ordem_das_convocacoes_pela_data(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_convocacoes_pela_data({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: a original aloca o nó (não roda duas vezes): só ela, e confere o plano: a conta da lista (+0x28, palavra)
/// sobe 1 com as duas não nulas e fica igual senão.
unsafe extern "thiscall" fn comparar_acrescentar_nacao_classificada(this: usize, nacao: usize, fase_final: usize) {
    let antes = *((this + 0x28) as *const i16) as i32;
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1041)(this, nacao, fase_final);
    let depois = *((this + 0x28) as *const i16) as i32;
    let esperado = antes + (nacao != 0 && fase_final != 0) as i32;
    PLACAR.registrar(depois == esperado, || format!("acrescentar_nacao_classificada({:x}, {:x}) conta {} -> {}", nacao, fase_final, antes, depois));
}

// ---------- 251a leva: as seleções: a competição atual da nação e os clubes com jogo na data; a disciplina: tirar o jogador regenerado (national_teams.cpp, discipline.cpp) ----------

unsafe fn cpo13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo13_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A competição (id) é uma das fases finais grandes ([0x9cf79c], [0x9cf7a0], [0x9cf7a4], [0x9cf7a8], [0x9cf7ac],
/// [0x9cf790], [0x9cf8a0], [0x9cf78c], [0x9cf964])?
unsafe fn cpo13_final(id: u32) -> bool {
    [0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964usize]
        .iter()
        .any(|&g| cpo13_d(g) as u32 == id)
}

/// O que a 0x775220 grava: a competição atual da nação ([this+4] + id × 110 + 4), a da fila ([this+8] + id × 4) e se
/// avisa o gerenciador dos técnicos.
struct Cpo13Atual {
    vaga: usize,
    nova: Option<u32>,
    fila: usize,
    na_fila: Option<u32>,
    avisa: bool,
}

/// O plano da 0x775220 (só lê): com competição atual (≠ −1) que é fase final grande, a nova vai para a fila e a atual
/// fica (sem aviso); senão, se a nova é fase final grande e há atual, a atual vai para a fila; a nova vira a atual e o
/// gerenciador dos técnicos ([0xb67a44], se existe) é avisado.
unsafe fn cpo13_plano_atual(this: usize, nacao: usize, comp: usize) -> Cpo13Atual {
    let id = cpo13_d(nacao);
    let vaga = cpo13_d(this + 4).wrapping_add(id.wrapping_mul(110)) + 4;
    let fila = cpo13_d(this + 8).wrapping_add(id.wrapping_mul(4));
    let atual = cpo13_d(vaga) as u32;
    let nova = cpo13_d(comp) as u32;
    let mut p = Cpo13Atual { vaga, nova: Some(nova), fila, na_fila: None, avisa: cpo13_d(0xb67a44) != 0 };
    if atual != 0xffff_ffff {
        let r = cpo13_d(0xae23d0).wrapping_add((atual as usize).wrapping_mul(107)); // registros de 107 bytes
        if r != 0 && cpo13_final(cpo13_d(r) as u32) {
            return Cpo13Atual { vaga, nova: None, fila, na_fila: Some(nova), avisa: false };
        }
        if cpo13_final(nova) {
            p.na_fila = Some(atual);
        }
    }
    p
}

/// { 0x775220 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::set_current_competition(nacao, comp)`: a
/// competição atual da nação vira `comp` (plano em `cpo13_plano_atual`: a fase final grande em curso fica na frente e a
/// outra vai para a fila) e o gerenciador dos técnicos é avisado (0x6a3770 com o clube da seleção principal, 0x53b3d0,
/// e 1). Nação ou competição nula: a caixa "Error" (e volta).
unsafe extern "thiscall" fn definir_competicao_da_selecao(this: usize, nacao: usize, comp: usize) {
    if nacao == 0 || comp == 0 {
        aviso_em(0xa5b2a4, 0x1bb2);
        return;
    }
    let p = cpo13_plano_atual(this, nacao, comp);
    if let Some(f) = p.na_fila {
        cpo13_por(p.fila, f);
    }
    let Some(nova) = p.nova else {
        crate::comparacao::ramo("cpo_13 fase final fica");
        return;
    };
    crate::comparacao::ramo("cpo_13 competicao atual");
    cpo13_por(p.vaga, nova);
    let g = cpo13_d(0xb67a44);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
        avisar(g, principal(nacao), 1);
    }
}

/// Comparador: a 0x6a3770 (o aviso) não pode rodar duas vezes: o plano antes, a original roda; confere a competição
/// atual e a da fila. Nação ou competição nula: só a original (caixa).
unsafe extern "thiscall" fn comparar_definir_competicao_da_selecao(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1003);
    if nacao == 0 || comp == 0 {
        return o(this, nacao, comp);
    }
    let p = cpo13_plano_atual(this, nacao, comp);
    let (atual, fila) = (cpo13_d(p.vaga) as u32, cpo13_d(p.fila) as u32);
    o(this, nacao, comp);
    let (atual2, fila2) = (cpo13_d(p.vaga) as u32, cpo13_d(p.fila) as u32);
    let ok = atual2 == p.nova.unwrap_or(atual) && fila2 == p.na_fila.unwrap_or(fila);
    PLACAR.registrar(ok, || format!("definir_competicao_da_selecao(nação {}, comp {}) atual {:#x} -> {:#x} (esperava {:x?}), fila {:#x} -> {:#x} (esperava {:x?})",
        cpo13_d(nacao), cpo13_d(comp), atual, atual2, p.nova, fila, fila2, p.na_fila));
}

/// Marca o clube `c` (se existe e o id < n) com o código na tabela [this+0x24].
unsafe fn cpo13_marcar(this: usize, c: usize, n: i32, codigo: u8) {
    if c != 0 && (cpo13_d(c) as i32) < n {
        *(cpo13_d(this + 0x24).wrapping_add(cpo13_d(c)) as *mut u8) = codigo;
    }
}

/// O clube tem nação marcada (+0x53, +0x11c ≠ 0)?
unsafe fn cpo13_nacao_marcada(c: usize) -> bool {
    let n = cpo13_d(c + 0x53);
    n != 0 && *((n + 0x11c) as *const u8) != 0
}

/// { 0x7775c0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::mark_clubs_with_fixtures(data)`: a tabela de um
/// byte por clube ([this+0x24], n = clubes [0xae235c] − 2 × nações [0xae2350]; calloc na primeira, sem memória a
/// caixa "Error") recebe, para cada jogo não amistoso (0x5c6710) da data nas 3 fatias (0x5ae6a0) de cada uma das 0x21
/// entradas de 0x48 bytes de 0xb63df0 (a nação +0 ativa em +0x11c, ou sem nação; o grupo +0x14): sem nação, 3 para os
/// clubes (+0x1c, +0x20) de nação ativa; com nação, 1 quando a competição do jogo (+0x14) é a da liga do mandante (+0x57)
/// ou a +0x60 do visitante, senão 2, para os dois. Lida pela remarcação dos jogos dos clubes com convocados (0x7777e0).
unsafe extern "thiscall" fn marcar_clubes_com_jogo(this: usize, data: usize) {
    let n = (cpo13_d(0xae235c) as i32).wrapping_sub((cpo13_d(0xae2350) as i32).wrapping_mul(2));
    if cpo13_d(this + 0x24) == 0 {
        let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(0x945501usize);
        let p = calloc(n, 1);
        cpo13_por(this + 0x24, p as u32);
        if p == 0 {
            aviso_em(0xa5b2a4, 0x2229);
            return;
        }
    }
    let lista_da_data: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    for i in 0..0x21usize {
        let e = 0xb63df0 + i * 0x48;
        let nacao = cpo13_d(e);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        for fatia in 0..3u32 {
            // a original empurra a fatia num dword de pilha com o alto de lixo e o grupo no al: a 0x5abf10 só lê os bytes
            let lista = lista_da_data(data, fatia, *((e + 0x14) as *const u8) as u32);
            if lista == 0 {
                continue;
            }
            cpo13_por(lista + 0xa, cpo13_d(lista + 2) as u32);
            let mut jogo = proximo(lista);
            while jogo != 0 {
                if amistoso(cpo13_d(0xae2c74), cpo13_d(jogo + 0x14)) == 0 {
                    let (casa, fora) = (cpo13_d(jogo + 0x1c), cpo13_d(jogo + 0x20));
                    if cpo13_d(e) == 0 {
                        if casa != 0 && cpo13_nacao_marcada(casa) {
                            cpo13_marcar(this, casa, n, 3);
                        }
                        if fora != 0 && cpo13_nacao_marcada(fora) {
                            cpo13_marcar(this, fora, n, 3);
                        }
                    } else {
                        let comp = cpo13_d(jogo + 0x14);
                        // o visitante +0x60 é lido sem conferir o ponteiro (como a original)
                        let codigo = if casa != 0 && (cpo13_d(casa + 0x57) == comp || cpo13_d(fora + 0x60) == comp) { 1 } else { 2 };
                        crate::comparacao::ramo("cpo_13 clubes com jogo");
                        cpo13_marcar(this, casa, n, codigo); // o mandante
                        cpo13_marcar(this, fora, n, codigo); // o visitante
                    }
                }
                jogo = proximo(lista);
            }
        }
    }
}

/// Comparador: na primeira (sem tabela: calloc) só a original; depois, lado a lado na tabela (n bytes). A lista do
/// dia e o amistoso só leem (o cursor da lista volta ao começo a cada uso).
unsafe extern "thiscall" fn comparar_marcar_clubes_com_jogo(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1004);
    let n = (cpo13_d(0xae235c) as i32).wrapping_sub((cpo13_d(0xae2350) as i32).wrapping_mul(2));
    if cpo13_d(this + 0x24) == 0 || n <= 0 {
        return o(this, data);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(cpo13_d(this + 0x24), n as usize)], || o(this, data), || marcar_clubes_com_jogo(this, data));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<(usize, u8, u8)> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).map(|i| (i, a[i], b[i])).collect();
        format!("marcar_clubes_com_jogo({:x}) clube, original, rust: {:?}", core::ptr::read_unaligned(data as *const u32), dif)
    });
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn cpo13_orgao(this: usize, regra: u8) -> usize {
    cpo13_d(cpo13_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada da pessoa no órgão pela vaga dela (0x7e82e0 sem criar: só lê).
unsafe fn cpo13_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// { 0x556830 } (thiscall, ret 4, discipline.cpp): `Discipline::remove_player(id)`: do regen (0x7abac0): apaga o
/// histórico de suspensões (0x55e1d0), libera as 4 vagas do registro de 29 bytes (com entrada no órgão: a suspensão
/// solta, 0x556d00, a vaga livre e a entrada +0 = −1), tira da lista [this+0x1c] as suspensões guardadas dele
/// (0x68d1f0, enquanto a conta +0x1c > 0) e o registro fica 0xff com a conta 0.
unsafe extern "thiscall" fn tirar_jogador_da_disciplina(this: usize, id: u32) {
    let r = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let apagar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x55e1d0usize);
    apagar(this, cpo13_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)));
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo13_entrada(cpo13_orgao(this, regra), id);
        if e == 0 {
            continue;
        }
        crate::comparacao::ramo("cpo_13 regen solta a vaga");
        let i = core::ptr::read_unaligned((e + 6) as *const i16);
        if i > -1 {
            // o índice no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra)
            let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
            soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
        }
        let v = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29) + k * 5);
        *(v as *mut u8) = 0xff;
        cpo13_por(v + 1, 0xffff_ffff);
        cpo13_por(e, 0xffff_ffff);
    }
    let lista = this + 0x1c;
    cpo13_por(lista + 0xa, cpo13_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if cpo13_d(item) as u32 == id {
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    core::ptr::write_bytes(r as *mut u8, 0xff, 29);
    *((r + 0x1c) as *mut u8) = 0;
}

/// Comparador: a 0x556830 solta suspensões e tira (e libera) itens da lista: o plano antes (as entradas das vagas), a
/// original roda; confere o registro (0xff, conta 0) e as entradas livres (+0 = −1).
unsafe extern "thiscall" fn comparar_tirar_jogador_da_disciplina(this: usize, id: u32) {
    let r = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let mut entradas = Vec::new();
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra != 0xff {
            let e = cpo13_entrada(cpo13_orgao(this, regra), id);
            if e != 0 {
                entradas.push(e);
            }
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, u32)>(1005)(this, id);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let ok = depois[..0x1c].iter().all(|&b| b == 0xff) && depois[0x1c] == 0
        && entradas.iter().all(|&e| cpo13_d(e) as u32 == 0xffff_ffff);
    PLACAR.registrar(ok, || format!("tirar_jogador_da_disciplina({}) registro {:02x?} entradas {:x?}", id, depois, entradas));
}

// ---------- 242a leva: as seleções: o fim das convocações de uma competição e a poda dos registros de convocação (national_teams.cpp) ----------

unsafe fn cpd1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Os itens da lista `l` (0x68d0d0: começo em +2, nó = item +0 e seguinte +6), sem mexer no cursor (+0xa).
unsafe fn cpd1_itens(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpd1_d(l + 2);
    while no != 0 && v.len() < 100_000 {
        v.push(cpd1_d(no));
        no = cpd1_d(no + 6);
    }
    v
}

/// O fim das convocações da lista [this] (as duas rotinas da 0x774f30/0x774f90): do começo (cursor +0xa = +2),
/// cada registro que `passa` recebe em +0x10 a palavra +0 (o dia) de amanhã (0x54c5b0 com 1 sobre [0xae2c90]).
unsafe fn cpd1_encerrar(this: usize, passa: impl Fn(usize) -> bool) {
    let l = cpd1_d(this);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpd1_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *const u16 = core::mem::transmute(0x54c5b0usize);
    let mut r = proximo(cpd1_d(this));
    while r != 0 {
        if passa(r) {
            let mut data = [0u8; 8];
            let dia = *mais_dias(0xae2c90, data.as_mut_ptr(), 1);
            core::ptr::write_unaligned((r + 0x10) as *mut u16, dia); // o dia de amanhã
        }
        r = proximo(cpd1_d(this));
    }
}

/// { 0x774f30 } (thiscall, ret 4, national_teams.cpp): `end_duty_for_competition(comp)`: na lista de registros
/// de convocação [this], os da competição (+4 igual a *comp) terminam amanhã (+0x10). Sem SEH, sem caixa, sem
/// sorteio; a local é a data que a 0x54c5b0 grava.
unsafe extern "thiscall" fn encerrar_convocacoes_da_competicao(this: usize, comp: usize) {
    cpd1_encerrar(this, |r| cpd1_d(r + 4) == cpd1_d(comp) /* a competição */)
}

/// { 0x774f90 } (thiscall, ret 8, national_teams.cpp): `end_duty_for_nation_in_competition(nacao, comp)`: como a
/// 0x774f30, só os registros da competição (+4) e da nação (+0 igual a *nacao).
unsafe extern "thiscall" fn encerrar_convocacoes_da_nacao(this: usize, nacao: usize, comp: usize) {
    cpd1_encerrar(this, |r| cpd1_d(r + 4) == cpd1_d(comp) && cpd1_d(r) == cpd1_d(nacao) /* a nação */)
}

/// As regiões do fim das convocações: o cursor da lista e o +0x10 de cada registro.
unsafe fn cpd1_regioes_encerrar(this: usize) -> Vec<(usize, usize)> {
    let l = cpd1_d(this);
    if l == 0 {
        return Vec::new();
    }
    let mut reg = vec![(l + 0xa, 4)];
    reg.extend(cpd1_itens(l).into_iter().filter(|&r| r != 0).map(|r| (r + 0x10, 2)));
    reg
}

unsafe extern "thiscall" fn comparar_encerrar_convocacoes_da_competicao(this: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(788);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, comp), || encerrar_convocacoes_da_competicao(this, comp));
    PLACAR.registrar(igual, || format!("encerrar_convocacoes_da_competicao({:x}, {:x}) {:x?} x {:x?}", this, comp, mo, mr));
}

unsafe extern "thiscall" fn comparar_encerrar_convocacoes_da_nacao(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(789);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao, comp), || encerrar_convocacoes_da_nacao(this, nacao, comp));
    PLACAR.registrar(igual, || format!("encerrar_convocacoes_da_nacao({:x}, {:x}, {:x}) {:x?} x {:x?}", this, nacao, comp, mo, mr));
}

/// O número de registros de convocação por clube da 0x7774b0: [0xae235c] − 2 × [0xae2350].
unsafe fn cpd1_quantos() -> i32 {
    (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2))
}

/// { 0x7774b0 } (thiscall, national_teams.cpp): `prune_club_call_up_records()`: cada registro de clube (+0x20,
/// 6 bytes: +0 vetor de entradas de 9 bytes, +4 marca, +5 quantas, byte com sinal) conta as entradas já vencidas
/// (0x456770: a data delas antes de [0xae2c90]; a rotina supõe que ficam no começo), desce as outras por cima
/// (0x777590) e diminui a contagem; zerada com vetor, solta o vetor (0x9452ca) e zera +4 e +0. Sem SEH, sem caixa,
/// sem sorteio; a local (o vencido) é zerada antes de ler.
unsafe extern "thiscall" fn podar_registros_de_convocacao(this: usize) {
    let n = cpd1_quantos();
    let mut rec = cpd1_d(this + 0x20);
    if n <= 0 {
        return;
    }
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x777590usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let conta = |rec: usize| *((rec + 5) as *const i8);
    for _ in 0..n {
        let mut vencidas: i8 = 0;
        if conta(rec) > 0 {
            let mut k: i8 = 0;
            loop {
                let e = cpd1_d(rec).wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                if antes(e, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0 {
                    vencidas = vencidas.wrapping_add(1); // uma vencida
                }
                k = k.wrapping_add(1);
                if k >= conta(rec) {
                    break;
                }
            }
        }
        if vencidas < conta(rec) {
            let mut k = vencidas;
            loop {
                crate::comparacao::ramo("convocações descer as que ficam");
                let v = cpd1_d(rec);
                let de = v.wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                let para = v.wrapping_add(9usize.wrapping_mul((k as i32 - vencidas as i32) as usize));
                copiar(para, de);
                k = k.wrapping_add(1);
                if k >= conta(rec) {
                    break;
                }
            }
        }
        let resto = conta(rec).wrapping_sub(vencidas);
        *((rec + 5) as *mut i8) = resto;
        if resto == 0 && cpd1_d(rec) != 0 {
            crate::comparacao::ramo("convocações soltar o vetor");
            free(cpd1_d(rec));
            *((rec + 4) as *mut u8) = 0;
            core::ptr::write_unaligned(rec as *mut usize, 0);
        }
        rec += 6;
    }
}

/// O registro `rec` da 0x7774b0 terminaria sem entradas e com vetor (a original solta o vetor)? Escrito à parte da
/// cópia, para o comparador.
unsafe fn cpd1_soltaria(rec: usize) -> bool {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let n = *((rec + 5) as *const i8);
    let v = cpd1_d(rec);
    let vencidas = (0..n.max(0) as usize).filter(|&k| antes(v + 9 * k, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0).count();
    v != 0 && n.wrapping_sub(vencidas as i8) == 0
}

/// Comparador: os registros (6 bytes cada) e os vetores de entradas. Algum vetor a soltar (0x9452ca; soltar duas
/// vezes): só a original.
unsafe extern "thiscall" fn comparar_podar_registros_de_convocacao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(790);
    let n = cpd1_quantos().max(0) as usize;
    let recs = cpd1_d(this + 0x20);
    if n == 0 || recs == 0 || (0..n).any(|i| cpd1_soltaria(recs + 6 * i)) {
        return o(this);
    }
    let mut reg = vec![(recs, 6 * n)];
    for i in 0..n {
        let (v, k) = (cpd1_d(recs + 6 * i), *((recs + 6 * i + 5) as *const i8));
        if v != 0 && k > 0 {
            reg.push((v, 9 * k as usize));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || podar_registros_de_convocacao(this));
    PLACAR.registrar(igual, || format!("podar_registros_de_convocacao({:x}) {:x?} x {:x?}", this, mo, mr));
}

// ---------- 242a leva: as seleções: pôr e tirar do elenco e dispensar os acima da idade do sub-21 (national_teams.cpp) ----------

unsafe fn cpd2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O time da nação para o tipo de convocação: 0 e 1 a seleção principal (0x53b3d0), os outros o sub-21 (0x53b4f0).
unsafe fn cpd2_time(nacao: usize, tipo: u8) -> usize {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    if tipo == 0 || tipo == 1 { principal(nacao) } else { sub21(nacao) }
}

/// Tira a pessoa `p` da tática do time (0x896c60 de [0xde1d08], com 1 e com 0 no 3º argumento), como as originais
/// fazem antes de esvaziar a vaga.
unsafe fn cpd2_tirar_da_tatica(time: usize, p: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    marcar(*(0xde1d08 as *const usize), time, p, 1, 1);
    marcar(*(0xde1d08 as *const usize), time, p, 0, 1);
}

/// A pessoa sai da seleção: sem time (+0x24), sem a marca +0x28 e só com os bits de clube (& 3) em +0x5e.
unsafe fn cpd2_limpar_pessoa(p: usize) {
    core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    *((p + 0x28) as *mut u8) = 0; // a marca de convocado
    *((p + 0x5e) as *mut u8) &= 3;
}

/// { 0x776250 } (thiscall, ret 0xc, national_teams.cpp): `add_player_to_squad(nacao, tipo, p)`: a primeira vaga
/// livre das 50 do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe `p`; sem vaga, nada. A pessoa ganha o
/// time (+0x24), a data de hoje em +0x29 (0x417c10 de [0xae2c90]), a marca 0xb em +0x28 e o elenco (0x540980) 4
/// (tipo 0 e 2) ou 8. Sem SEH, sem caixa, sem sorteio; a local é a cópia da data que a 0x417c10 devolve.
unsafe extern "thiscall" fn por_no_elenco_da_selecao(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let tipo = tipo as u8;
    let time = cpd2_time(nacao, tipo);
    let elenco: u32 = if tipo == 0 || tipo == 2 { 4 } else { 8 };
    let vaga = match (0..50usize).find(|&c| cpd2_d(time + 0xd7 + 4 * c) == 0) {
        Some(c) => time + 0xd7 + 4 * c,
        None => return,
    };
    core::ptr::write_unaligned(vaga as *mut usize, p); // a vaga
    core::ptr::write_unaligned((p + 0x24) as *mut usize, time);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut data = [0u8; 8];
    copia(p + 0x29, data.as_mut_ptr(), 0xae2c90);
    *((p + 0x28) as *mut u8) = 0xb;
    let marcar_elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    marcar_elenco(p, elenco);
}

/// As regiões de pôr e tirar do elenco: as 50 vagas do time e a pessoa (+0x24 a +0x5e).
unsafe fn cpd2_regioes(time: usize, p: usize) -> [(usize, usize); 2] {
    [(time + 0xd7, 200), (p + 0x24, 0x3b)]
}

unsafe extern "thiscall" fn comparar_por_no_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(791);
    let time = cpd2_time(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = cpd2_regioes(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || por_no_elenco_da_selecao(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("por_no_elenco_da_selecao({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// { 0x7762f0 } (thiscall, ret 0xc, national_teams.cpp): `remove_player_from_squad(nacao, tipo, p)`: tira `p` da
/// tática do time (0x896c60, duas vezes) e, se ele está numa das 50 vagas (+0xd7), esvazia a vaga e limpa a
/// pessoa (+0x24, +0x28, +0x5e & 3). Sem SEH, sem sorteio, sem locais; caixa só com time ou pessoa nulos (0x896c60).
unsafe extern "thiscall" fn tirar_do_elenco_da_selecao(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let time = cpd2_time(nacao, tipo as u8);
    cpd2_tirar_da_tatica(time, p);
    if let Some(c) = (0..50usize).find(|&c| cpd2_d(time + 0xd7 + 4 * c) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * c) as *mut usize, 0);
        cpd2_limpar_pessoa(p);
    }
}

/// Comparador: as vagas e a pessoa. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha e não
/// muda nada, então a tática fica fora das regiões. Time ou pessoa nulos (a caixa da 0x896c60): só a original.
unsafe extern "thiscall" fn comparar_tirar_do_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(792);
    let time = cpd2_time(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = cpd2_regioes(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || tirar_do_elenco_da_selecao(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("tirar_do_elenco_da_selecao({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id (+0 da nação).
unsafe fn cpd2_registro(this: usize, nacao: usize) -> usize {
    cpd2_d(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize)
}

/// { 0x76d880 } (thiscall, ret 4, national_teams.cpp): `release_overage_u21_players(nacao)`: depois da data do
/// registro da nação (+0x62/+0x66; 0x456740 sobre [0xae2c90]), cada uma das 50 vagas do sub-21 (0x53b4f0) com
/// pessoa de idade (+0x18, byte com sinal) acima de 21 sai da tática (0x896c60, duas vezes), é limpa (+0x24, +0x28,
/// +0x5e & 3) e a vaga esvazia; por fim zera o byte +0x6b do registro. Sem SEH, sem sorteio; a local é o `this`.
unsafe extern "thiscall" fn dispensar_acima_da_idade_do_sub21(this: usize, nacao: usize) {
    let rec = cpd2_registro(this, nacao);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    if depois(0xae2c90, cpd2_d(rec + 0x62) as u32, cpd2_d(rec + 0x66) as u32) == 0 {
        return;
    }
    crate::comparacao::ramo("sub-21 dispensar acima da idade");
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let time = sub21(nacao);
    for c in 0..50usize {
        let vaga = time + 0xd7 + 4 * c;
        let p = cpd2_d(vaga);
        if p != 0 && *((p + 0x18) as *const i8) > 0x15 { // acima de 21 anos
            cpd2_tirar_da_tatica(time, p);
            cpd2_limpar_pessoa(p);
            core::ptr::write_unaligned(vaga as *mut usize, 0);
        }
    }
    *((cpd2_registro(this, nacao) + 0x6b) as *mut u8) = 0;
}

/// Comparador: o byte +0x6b do registro, as vagas do sub-21 e as pessoas delas (+0x24 a +0x5e). A 0x896c60 da
/// cópia não acha mais as pessoas que a original já tirou (fica fora das regiões, como na 0x7762f0). Sub-21 nulo:
/// só a original.
unsafe extern "thiscall" fn comparar_dispensar_acima_da_idade_do_sub21(this: usize, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(793);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let time = sub21(nacao);
    if time == 0 {
        return o(this, nacao);
    }
    let mut reg = vec![(cpd2_registro(this, nacao) + 0x6b, 1), (time + 0xd7, 200)];
    reg.extend((0..50usize).map(|c| cpd2_d(time + 0xd7 + 4 * c)).filter(|&p| p != 0).map(|p| (p + 0x24, 0x3b)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao), || dispensar_acima_da_idade_do_sub21(this, nacao));
    PLACAR.registrar(igual, || format!("dispensar_acima_da_idade_do_sub21({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

// ---------- 242a leva: as seleções: a competição atual de cada nação e a convocação do substituto (national_teams.cpp) ----------

unsafe fn cpd3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpd3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Avisa o gerenciador dos técnicos ([0xb67a44], se existe) da seleção principal da nação (0x6a3770 com 1).
unsafe fn cpd3_avisar_tecnicos(nacao: usize) {
    let g = *(0xb67a44 as *const usize);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
        avisar(g, principal(nacao), 1);
    }
}

/// { 0x775000 } (thiscall, ret 4, national_teams.cpp): `clear_current_competition(nacao)`: a competição atual do
/// registro da nação ([this+4] + 0x6e × id, +4) vira −1; havendo a seguinte ([this+8][id] ≠ −1), ela passa a ser a
/// atual, a seguinte vira −1 e o gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
unsafe extern "thiscall" fn limpar_competicao_atual(this: usize, nacao: usize) {
    let id = *(nacao as *const i32);
    let atual = cpd3_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4;
    core::ptr::write_unaligned(atual as *mut i32, -1);
    let seguinte = cpd3_d(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    let v = *(seguinte as *const i32);
    if v != -1 {
        crate::comparacao::ramo("seleção a competição seguinte");
        core::ptr::write_unaligned(atual as *mut i32, v); // a seguinte vira a atual
        *(seguinte as *mut i32) = -1;
        cpd3_avisar_tecnicos(nacao); // o aviso da competição seguinte
    }
}

/// Comparador: a competição atual e a seguinte; com a seguinte (o aviso 0x6a3770 sorteia e mexe nos técnicos), só a
/// original.
unsafe extern "thiscall" fn comparar_limpar_competicao_atual(this: usize, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(794);
    let id = *(nacao as *const i32);
    let seguinte = cpd3_d(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    if *(seguinte as *const i32) != -1 {
        return o(this, nacao);
    }
    let reg = [(cpd3_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4, 4), (seguinte, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao), || limpar_competicao_atual(this, nacao));
    PLACAR.registrar(igual, || format!("limpar_competicao_atual({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

/// A competição `v` (−1: nenhuma) acabou para a 0x775070? Sem `fim_de_fase`, quando +0x59 da competição
/// ([0xae23d0] + 107 × v) é nulo; com ele, quando é a fase [0xae23a4] + 0xc6 × [0x9cfa10].
unsafe fn cpd3_acabou(v: i32, fim_de_fase: bool) -> bool {
    let c = cpd3_d((*(0xae23d0 as *const usize)).wrapping_add(v.wrapping_mul(107) as usize) + 0x59);
    if fim_de_fase {
        c == (*(0xae23a4 as *const usize)).wrapping_add((*(0x9cfa10 as *const i32)).wrapping_mul(0xc6) as usize)
    } else {
        c == 0
    }
}

/// { 0x775070 } (thiscall, ret 8, national_teams.cpp): `release_competitions_of_continent(continente, fim_de_fase)`:
/// a cada nação ([0xae23a8], 0x122 bytes) do continente (+0x71), a competição atual (registro +4) e a seguinte
/// ([this+8][i]) que acabaram (cpd3_acabou) viram −1; se a atual ficou −1 e há seguinte, a seguinte passa a atual e o
/// gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
unsafe extern "thiscall" fn liberar_competicoes_do_continente(this: usize, continente: usize, fim_de_fase: u32) {
    let fim = fim_de_fase as u8 != 0;
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        let nacao = (*(0xae23a8 as *const usize)).wrapping_add(0x122 * i as usize);
        if cpd3_d(nacao + 0x71) == continente {
            let atual = cpd3_d(this + 4) + 0x6e * i as usize + 4;
            let v = *(atual as *const i32);
            if v != -1 && cpd3_acabou(v, fim) {
                *(atual as *mut i32) = -1; // a atual acabou
            }
            let seguinte = cpd3_d(this + 8) + 4 * i as usize;
            let v = *(seguinte as *const i32);
            if v != -1 && cpd3_acabou(v, fim) {
                *(seguinte as *mut i32) = -1;
            }
            let v = *((cpd3_d(this + 8) + 4 * i as usize) as *const i32);
            if v != -1 && *((cpd3_d(this + 4) + 0x6e * i as usize + 4) as *const i32) == -1 {
                crate::comparacao::ramo("seleção continente a seguinte vira a atual");
                *((cpd3_d(this + 4) + 0x6e * i as usize + 4) as *mut i32) = v;
                *((cpd3_d(this + 8) + 4 * i as usize) as *mut i32) = -1;
                cpd3_avisar_tecnicos(nacao); // o aviso do continente
            }
        }
        i += 1;
    }
}

/// Comparador: o aviso 0x6a3770 sorteia e mexe nos técnicos (não pode rodar duas vezes): só a original; a prova é a
/// carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (4 chamadas na carreira); comparar pedia separar a decisão do aviso.
unsafe extern "thiscall" fn comparar_liberar_competicoes_do_continente(this: usize, continente: usize, fim_de_fase: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(795)(this, continente, fim_de_fase)
}

/// { 0x775f90 } (thiscall, ret 0x10, national_teams.cpp): `call_up_replacement(nacao, tipo, nivel, saindo)`: no
/// trecho do nível (0x76d810) na reserva do tipo ([this+0xc], sub-21 [this+0x10]; começo +8/+0x1d do registro mais
/// as vagas dos níveis anteriores, quantos +0xd/+0x22), o primeiro que não é `saindo`, tem ficha (+0x61), gosta do
/// clube (0x547d30), está bem (0x62c400), não é regen marcado (0x7a7eb0) fora da principal, é da nação (time +0x24,
/// nacionalidades +0x1a/+0x1e, +0x22, bit 2 do registro de 79 bytes), não está suspenso (0x55d270) e joga numa das
/// posições de `saindo` (0x5b7d20). Sem time vai direto; convocado (+0x28 = 0xb) para outro tipo (2 com 0x5409b0,
/// senão o bit 3 de +0x5e) só depois da data do tipo (+0x3a; 0x456770) e, sendo o 0, se o clube deixa (0x600a50):
/// sai de lá (0x7762f0). Entra (0x776250), a vaga do tipo (+0x45) desce 1 e sai a notícia (0x76fd10). Ninguém: zera
/// +0x45. Sem SEH, sem sorteio; a local do tipo do outro elenco é gravada antes de ler (só o byte vale).
unsafe extern "thiscall" fn convocar_substituto(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let regen: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7762f0usize);
    let por: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x776250usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x76fd10usize);
    let mascara = posicoes(cpd3_d(saindo + 0x61)) as u8;
    let t = tipo as u8 as i8;
    let nivel = nivel as u8 as i8;
    let rec = cpd3_d(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize);
    let sub = (t == 2) as usize;
    let o = 0x15 * sub;
    let mut inicio = cpd3_d(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        inicio = inicio.wrapping_add(cpd3_b(rec + o + 0xe + 4 * j) as i32);
    }
    let fim = inicio.wrapping_add(cpd3_b(rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize) + 0xd) as i32);
    let reserva = cpd3_d(this + 4 * sub + 0xc);
    let vaga = rec.wrapping_add((20 * t as i32) as usize) + 0x45;
    let mut k = inicio;
    while k < fim {
        let q = cpd3_d(reserva.wrapping_add((4 * k) as usize));
        k += 1;
        if q == saindo || cpd3_d(q + 0x61) == 0 || nao_gosta(q, principal(cpd3_d(q + 0x1a))) != 0 {
            continue;
        }
        if estado(*(0xb63d28 as *const usize), q) != 0 {
            continue;
        }
        if *(0xb7434c as *const usize) != 0 && regen(q) != 0 && t != 0 {
            continue;
        }
        let time = cpd3_d(q + 0x24);
        if time != 0 && cpd3_d(time + 0x53) != nacao {
            continue;
        }
        if cpd3_d(q + 0x1a) != nacao {
            let registro = (*(0xae2d0c as *const usize)).wrapping_add((*(q as *const i32)).wrapping_mul(0x4f) as usize);
            if time != 0 || cpd3_b(q + 0x22) != 0 || cpd3_b(registro + 0xc) & 2 != 0 || cpd3_d(q + 0x1e) != nacao {
                continue;
            }
        }
        let comp = (*(0xae23d0 as *const usize)).wrapping_add((*(0x9cf79c as *const i32)).wrapping_mul(107) as usize);
        if suspenso(*(0xae28f0 as *const usize), q, comp, 1, 0) != 0 {
            continue;
        }
        if posicoes(cpd3_d(q + 0x61)) as u8 & mascara & 0x7f == 0 { // as posições de quem sai
            continue;
        }
        let time = cpd3_d(q + 0x24);
        if time != 0 {
            if cpd3_b(q + 0x28) != 0xb {
                continue;
            }
            let outro: i8 = if segunda(time) != 0 { 2 } else { ((cpd3_b(q + 0x5e) >> 3) & 1) as i8 };
            if outro == t {
                continue;
            }
            let data = rec.wrapping_add((20 * outro as i32) as usize) + 0x3a;
            if antes(data, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) == 0 {
                continue;
            }
            if outro == 0 && humano(time, 0, 1, 0, 0) == 0 {
                continue;
            }
            crate::comparacao::ramo("substituto sai do outro elenco");
            tirar(this, nacao, outro as u8 as u32, q);
        }
        crate::comparacao::ramo("substituto convocado");
        por(this, nacao, tipo, q);
        *(vaga as *mut u8) = cpd3_b(vaga).wrapping_sub(1); // uma vaga a menos
        noticia(this, q, nacao, tipo);
        return;
    }
    *(vaga as *mut u8) = 0;
}

/// Comparador: a entrada no elenco sai com a notícia (0x76fd10) e a saída do outro mexe na tática: só a original
/// (a 0x776250 e a 0x7762f0 que ela chama passam pelos comparadores delas).
// ponytail: só a original; a prova é a carreira (140 chamadas) e o ramo da cópia no modo Rust.
unsafe extern "thiscall" fn comparar_convocar_substituto(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(796)(this, nacao, tipo, nivel, saindo)
}

// ---------- 242a leva: as seleções: as reservas de convocáveis de cada nação e as duas ordens delas (national_teams.cpp) ----------

unsafe fn cpd4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpd4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpd4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A nota da pessoa da vaga `q` (ponteiro para a pessoa): o curto +9 da ficha (+0x61).
unsafe fn cpd4_nota(q: usize) -> i16 {
    cpd4_w(cpd4_d(cpd4_d(q) + 0x61) + 9)
}

/// O começo do trecho do nível `nivel` na reserva (`o` 0: a principal, 0x15: o sub-21): o +8 do registro mais as
/// vagas (+0xe) dos níveis anteriores.
unsafe fn cpd4_inicio(rec: usize, o: usize, nivel: i8) -> i32 {
    let mut v = cpd4_d(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        v = v.wrapping_add(cpd4_b(rec + o + 0xe + 4 * j) as i32);
    }
    v
}

/// Põe a pessoa `p` na reserva do registro `rec` (`sub` 0: a principal em [this+0xc]; 1: o sub-21 em [this+0x10],
/// campos 0x15 adiante), no trecho do nível dela (0x76d810). Com vaga (+0xd < +0xe), no fim do trecho, e a menor nota
/// (+0xf) desce se for o caso; cheio, só com nota acima da menor: entra na posição pela nota (decrescente, depois das
/// iguais), empurrando o último para fora, e a menor nota vira a do último. Trecho cheio: ordena (qsort 0x945333 com
/// 0x778ed0).
unsafe fn cpd4_reservar(this: usize, p: usize, rec: usize, sub: u32) {
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x76d810usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let o = 0x15 * sub as usize;
    let nivel = nivel_de(p, rec, sub) as i8;
    let c = rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize);
    let reserva = || cpd4_d(this + 0xc + 4 * sub as usize);
    let vaga = |k: i32| reserva().wrapping_add((4 * k) as usize);
    let nota = cpd4_w(cpd4_d(p + 0x61) + 9);
    if cpd4_b(c + 0xd) < cpd4_b(c + 0xe) {
        let ini = cpd4_inicio(rec, o, nivel);
        core::ptr::write_unaligned(vaga(ini.wrapping_add(cpd4_b(c + 0xd) as i32)) as *mut usize, p);
        *((c + 0xd) as *mut u8) = cpd4_b(c + 0xd).wrapping_add(1);
        if nota < cpd4_w(c + 0xf) {
            core::ptr::write_unaligned((c + 0xf) as *mut i16, nota); // a menor nota
        }
    } else if nota > cpd4_w(c + 0xf) {
        crate::comparacao::ramo("reservas cheias entra pela nota");
        let ini = cpd4_inicio(rec, o, nivel);
        let ultimo = (cpd4_b(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(2);
        let mut pos = ini;
        let mut j = ultimo;
        while j >= ini {
            if nota < cpd4_nota(vaga(j)) {
                pos = j + 1; // depois das de nota maior
                break;
            }
            j -= 1;
        }
        let mut j = ultimo;
        while j >= pos {
            core::ptr::write_unaligned(vaga(j + 1) as *mut usize, cpd4_d(vaga(j)));
            j -= 1;
        }
        core::ptr::write_unaligned(vaga(pos) as *mut usize, p);
        let ult = (cpd4_b(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(1);
        core::ptr::write_unaligned((c + 0xf) as *mut i16, cpd4_nota(vaga(ult)));
    }
    let vagas = cpd4_b(c + 0xe);
    if cpd4_b(c + 0xd) == vagas {
        qsort(vaga(cpd4_inicio(rec, o, nivel)), vagas as u32, 4, 0x778ed0);
    }
}

/// { 0x76abf0 } (thiscall, national_teams.cpp): `build_selection_pools()`: zera as contagens (+0xd, +0x22) e põe
/// 10000 nas menores notas (+0xf, +0x24) dos 4 níveis de cada registro de nação ([this+4], 0x6e bytes); depois, cada
/// pessoa ([0xae23bc], 0x6e bytes) com nação (+0x1a), ficha (+0x61), mais de 16 anos (+0x18), sem seleção ou já
/// convocada (+0x24, +0x28 = 0xb), na posição de jogador (0x53b620 de +0x3d) e com clube (+0x39) entra na reserva
/// da nação (cpd4_reservar) e, até 21 anos com menos de 5 jogos (+0x22), na do sub-21; e de novo pela segunda
/// nacionalidade (+0x1e: sem jogos pela primeira, outra e não a [0x9cf434]). Sem SEH, sem caixa, sem sorteio; as
/// locais são gravadas antes de ler (da nota guardada em 32 bits só vale o curto).
unsafe extern "thiscall" fn montar_reservas_das_selecoes(this: usize) {
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        for k in 0..4usize {
            let r = cpd4_d(this + 4) + 0x6e * i as usize + 4 * k;
            *((r + 0xd) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0xf) as *mut i16, 10000);
            *((r + 0x22) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0x24) as *mut i16, 10000);
        }
        i += 1;
    }
    let jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let mut p = *(0xae23bc as *const usize);
    let mut n = 0i32;
    while n < *(0xae2364 as *const i32) {
        let pode = cpd4_d(p + 0x1a) != 0
            && cpd4_d(p + 0x61) != 0
            && *((p + 0x18) as *const i8) > 0x10 // mais de 16 anos
            && (cpd4_d(p + 0x24) == 0 || cpd4_b(p + 0x28) == 0xb)
            && jogador(cpd4_b(p + 0x3d) as u32) != 0
            && cpd4_d(p + 0x39) != 0;
        if pode {
            for k in 0..2 {
                let id = if k == 0 {
                    *(cpd4_d(p + 0x1a) as *const i32)
                } else {
                    let n2 = cpd4_d(p + 0x1e);
                    if n2 == 0 || cpd4_b(p + 0x22) != 0 || n2 == cpd4_d(p + 0x1a) || *(n2 as *const i32) == *(0x9cf434 as *const i32) {
                        continue;
                    }
                    crate::comparacao::ramo("reservas segunda nacionalidade");
                    *(n2 as *const i32)
                };
                let rec = cpd4_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize);
                cpd4_reservar(this, p, rec, 0);
                if *((p + 0x18) as *const i8) > 0x15 || cpd4_b(p + 0x22) >= 5 { // o sub-21
                    continue;
                }
                cpd4_reservar(this, p, rec, 1);
            }
        }
        n += 1;
        p += 0x6e;
    }
}

/// Comparador: os registros das nações e, em cada reserva (principal e sub-21), o trecho dos 4 níveis (a soma das
/// vagas +0xe a partir do começo +8).
unsafe extern "thiscall" fn comparar_montar_reservas_das_selecoes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(797);
    let n = (*(0xae2350 as *const i32)).max(0) as usize;
    let recs = cpd4_d(this + 4);
    let mut reg = vec![(recs, 0x6e * n)];
    for i in 0..n {
        let rec = recs + 0x6e * i;
        for sub in 0..2usize {
            let o = 0x15 * sub;
            let total: usize = (0..4).map(|k| cpd4_b(rec + o + 0xe + 4 * k) as usize).sum();
            if total > 0 {
                reg.push((cpd4_d(this + 0xc + 4 * sub).wrapping_add(4 * cpd4_d(rec + o + 8)), 4 * total));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || montar_reservas_das_selecoes(this));
    PLACAR.registrar(igual, || format!("montar_reservas_das_selecoes({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// { 0x778ed0 } (cdecl, national_teams.cpp): a ordem das reservas para o qsort da 0x76abf0: pela nota (+9 da ficha
/// +0x61) decrescente (nota de b menos a de a).
unsafe extern "C" fn ordem_das_reservas(a: usize, b: usize) -> i32 {
    (cpd4_nota(b) as i32) - cpd4_nota(a) as i32 // decrescente
}

unsafe extern "C" fn comparar_ordem_das_reservas(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(798)(a, b);
    let n = ordem_das_reservas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_reservas({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x778ef0 } (cdecl, national_teams.cpp): a ordem das pessoas pelo clube (+0x39) para os qsort das seleções
/// (0x76c120, 0x7706c0, 0x7771c0): sem clube depois; clubes diferentes pelo id; no mesmo, o de elenco 4 (+0x5e)
/// antes do de elenco 8.
unsafe extern "C" fn ordem_pelo_clube(a: usize, b: usize) -> i32 {
    let (x, y) = (cpd4_d(a), cpd4_d(b));
    let cx = if x != 0 { cpd4_d(x + 0x39) } else { 0 };
    let cy = if y != 0 { cpd4_d(y + 0x39) } else { 0 };
    if cx == 0 {
        return (cy != 0) as i32;
    }
    if cy == 0 {
        return -1;
    }
    if cx != cy {
        return (*(cx as *const i32)).wrapping_sub(*(cy as *const i32)); // o id do clube
    }
    let (mx, my) = (cpd4_b(x + 0x5e), cpd4_b(y + 0x5e));
    if mx & 4 != 0 && my & 8 != 0 {
        return -1;
    }
    if mx & 8 != 0 && my & 4 != 0 {
        return 1;
    }
    0
}

unsafe extern "C" fn comparar_ordem_pelo_clube(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(799)(a, b);
    let n = ordem_pelo_clube(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pelo_clube({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 211a leva: o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados (national_teams.cpp, fix_man.cpp, shortlist_manager.cpp) ----------

/// { 0x775ba0 } (thiscall, ret 4, national_teams.cpp): tira o `jogador` da seleção. Com nação (+0x1a)
/// e ficha (+0x61): no registro de 110 bytes da nação em [this+4], o nível (0x76d810 com o registro e
/// 0); se a nota do nível (palavra +0xf + nível x 4) for menor que a da ficha (+9), procura o jogador
/// na faixa do nível da lista [this+0xc] (início = dword +8 + bytes +0xe dos níveis anteriores,
/// tamanho no byte +0xd + nível x 4): achou, o último da faixa vai para a vaga dele e o tamanho
/// diminui. Depois, convocado (+0x24): a vaga dele entre as 50 (+0xd7) zera, +0x24 zera e o byte +0x5e
/// fica só com os bits 0..1.
unsafe extern "thiscall" fn tirar_jogador_da_selecao(this: usize, jogador: usize) {
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return;
    }
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(0x76d810usize);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let campo = |o: isize| (reg as isize).wrapping_add(n * 4 + o) as usize;
    if core::ptr::read_unaligned(campo(0xf) as *const i16) < core::ptr::read_unaligned((u(jogador + 0x61) + 9) as *const i16) {
        let mut inicio = u(reg + 8) as i32;
        for k in 0..n.max(0) as usize {
            inicio = inicio.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32);
        }
        let tamanho = campo(0xd) as *mut u8;
        let fim = inicio.wrapping_add(*tamanho as i32);
        let lista = |i: i32| (u(this + 0xc) as isize).wrapping_add(i as isize * 4) as *mut usize;
        let mut i = inicio;
        while i < fim {
            if core::ptr::read_unaligned(lista(i)) == jogador {
                *tamanho = (*tamanho).wrapping_sub(1);
                core::ptr::write_unaligned(lista(i), core::ptr::read_unaligned(lista(fim - 1)));
                break;
            }
            i += 1;
        }
    }
    let selecao = u(jogador + 0x24);
    if selecao == 0 {
        return;
    }
    if let Some(k) = (0..50).find(|&k| u(selecao + 0xd7 + 4 * k) == jogador) {
        core::ptr::write_unaligned((selecao + 0xd7 + 4 * k) as *mut usize, 0);
    }
    core::ptr::write_unaligned((jogador + 0x24) as *mut usize, 0);
    *((jogador + 0x5e) as *mut u8) &= 3;
}

/// { 0x5af2f0 } (thiscall, ret 8, fix_man.cpp; `this` não é lido): a faixa de horário do jogo da
/// competição `comp` no dia `dia` (palavra): Holanda, Irlanda, Bélgica e França (id da nação +0x5d
/// em [0x9cf338], [0x9cf2dc], [0x9cf244], [0x9cf300]) só têm 1 no dia 6; as outras, 1 nos dias 5 e 6;
/// senão 2. Devolve só `al`.
unsafe extern "thiscall" fn faixa_do_horario(_this: usize, comp: usize, dia: u32) -> u8 {
    let dia = dia as i16;
    if comp != 0 {
        let nacao = core::ptr::read_unaligned((comp + 0x5d) as *const usize);
        if nacao != 0 {
            let id = *(nacao as *const u32);
            if [0x9cf338usize, 0x9cf2dc, 0x9cf244, 0x9cf300].iter().any(|&g| id == *(g as *const u32)) {
                return if dia != 6 { 2 } else { 1 };
            }
        }
    }
    if dia == 6 || dia == 5 { 1 } else { 2 }
}

/// { 0x84b230 } (stdcall, ret 0xc, shortlist_manager.cpp; `posicoes` não é lido): o teto de preço
/// para a vaga de tipo `tipo` na lista de observados, pela reputação do clube (palavra +0x80): tipo 4
/// = x550 em Itália, Inglaterra e Espanha ([0x9cf360], [0x9cf2e4], [0x9cf488], nação +0x53), x475 em
/// Alemanha e França ([0x9cf314], [0x9cf300]), x330 nas outras; tipo 5 = x550; tipo 6 = x110; os
/// outros, -1.
unsafe extern "stdcall" fn teto_da_observacao(clube: usize, _posicoes: u32, tipo: u32) -> i32 {
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    match tipo as i8 {
        4 => {
            let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
            let eh = |gs: &[usize]| nacao != 0 && gs.iter().any(|&g| *(nacao as *const u32) == *(g as *const u32));
            if eh(&[0x9cf360, 0x9cf2e4, 0x9cf488]) {
                rep.wrapping_mul(550)
            } else if eh(&[0x9cf314, 0x9cf300]) {
                rep.wrapping_mul(475)
            } else {
                rep.wrapping_mul(330)
            }
        }
        5 => rep.wrapping_mul(550),
        6 => rep.wrapping_mul(110),
        _ => -1,
    }
}

unsafe extern "thiscall" fn comparar_tirar_jogador_da_selecao(this: usize, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(517);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return o(this, jogador);
    }
    // as regiões: o jogador, o registro da nação, a faixa do nível na lista e a seleção (o nível,
    // 0x76d810, só lê)
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(0x76d810usize);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let mut regioes = [(jogador, 110), (reg, 110), (jogador, 0), (jogador, 0)];
    if (0..=3).contains(&n) {
        let inicio = (0..n as usize).fold(u(reg + 8) as i32, |s, k| s.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32));
        let tamanho = *((reg + 0xd + 4 * n as usize) as *const u8) as usize;
        if inicio >= 0 && tamanho > 0 {
            regioes[2] = (u(this + 0xc) + 4 * inicio as usize, 4 * tamanho);
        }
    }
    if u(jogador + 0x24) != 0 {
        regioes[3] = (u(jogador + 0x24) + 0xd7, 200);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogador), || tirar_jogador_da_selecao(this, jogador));
    PLACAR.registrar(igual, || format!("tirar_jogador_da_selecao(nivel {}) {:x?} x {:x?}", n, mo, mr));
}

unsafe extern "thiscall" fn comparar_faixa_do_horario(this: usize, comp: usize, dia: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(518)(this, comp, dia);
    let n = faixa_do_horario(this, comp, dia);
    PLACAR.registrar(r == n, || format!("faixa_do_horario(dia {}) original={} rust={}", dia as i16, r, n));
    r
}

unsafe extern "stdcall" fn comparar_teto_da_observacao(clube: usize, posicoes: u32, tipo: u32) -> i32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32, u32) -> i32>(519)(clube, posicoes, tipo);
    let n = teto_da_observacao(clube, posicoes, tipo);
    PLACAR.registrar(r == n, || format!("teto_da_observacao(tipo {}) original={} rust={}", tipo as i8, r, n));
    r
}

// ---------- 210a leva: as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional (national_teams.cpp, tactics.cpp, media.cpp) ----------

/// As 50 vagas de convocação (dwords desalinhados em +0xd7) das duas seleções da nação: a principal
/// (0x53b3d0) e a B (0x53b4f0), nessa ordem.
unsafe fn convocacoes_das_selecoes(nacao: usize) -> [usize; 2] {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    [principal(nacao).wrapping_add(0xd7), b(nacao).wrapping_add(0xd7)]
}

/// { 0x76c060 } (stdcall, ret 4, national_teams.cpp): apaga as convocações repetidas das duas
/// seleções da nação: em cada uma, todo id não nulo zera as vagas seguintes com o mesmo id.
unsafe extern "stdcall" fn apagar_repetidos_das_convocacoes(nacao: usize) {
    for vagas in convocacoes_das_selecoes(nacao) {
        let vaga = |i: usize| (vagas + 4 * i) as *mut u32;
        for i in 0..50 {
            let v = core::ptr::read_unaligned(vaga(i));
            if v == 0 {
                continue;
            }
            for j in i + 1..50 {
                if core::ptr::read_unaligned(vaga(j)) == v {
                    core::ptr::write_unaligned(vaga(j), 0);
                }
            }
        }
    }
}

/// { 0x8972c0 } (thiscall, ret 0xc, tactics.cpp): a vaga (0..19) da escalação do clube em que está a
/// pessoa: linha tática do clube (0x8982b0 com `time`; -1 devolve 0) e as 20 vagas (dwords em +0x601 +
/// linha x 0x91 + 2) comparadas com o id; não está: 0xff. Clube ou pessoa nulos: a original (caixa
/// de erro).
unsafe extern "thiscall" fn vaga_do_escalado(tatica: usize, clube: usize, pessoa: *const i32, time: u32) -> u8 {
    if clube == 0 || pessoa.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(509)(tatica, clube, pessoa, time);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, time);
    if k == -1 {
        return 0;
    }
    let id = *pessoa;
    let vagas = core::ptr::read_unaligned((tatica + 0x601) as *const usize).wrapping_add((k as usize).wrapping_mul(0x91) + 2);
    match (0..20).find(|&i| core::ptr::read_unaligned((vagas + 4 * i) as *const i32) == id) {
        Some(i) => i as u8,
        None => 0xff,
    }
}

/// { 0x72b810 } (thiscall, ret 8, media.cpp; `this` não é lido): sorteia o comentarista
/// ex-internacional para o `clube` (ou para falar de `assunto`). 150 rodadas; em cada uma sorteia
/// pessoas (0x90cfc0 com [0xae2364], 110 bytes em [0xae23bc]) até achar uma com idade (+0x18) ≥ 37,
/// jogos pela seleção (+0x22) ≥ 50, ficha (+0x69), sem clube (+0x39) se tiver contrato (+0x61) e que
/// não seja o técnico do clube (+0xcf); a 100ª tentativa encerra a rodada. O peso é um sorteio de
/// reputação da nação (+0x1a, palavra +0x8e) × jogos, dobrado se for da nação do assunto (sem
/// assunto: tem de ser da nação do clube, +0x53). De outra nação: a reputação tem de vencer 6500 +
/// sorteio de 100, a ficha +0xc ≥ 6000 e o assunto não pode desgostar dele (0x547840). Fica o de
/// maior peso (o primeiro aceito sempre entra). Sem clube e sem assunto: 0.
unsafe extern "thiscall" fn escolher_comentarista(_this: usize, clube: usize, assunto: usize) -> usize {
    if assunto == 0 && clube == 0 {
        return 0;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let reputacao = |nacao: usize| core::ptr::read_unaligned((nacao + 0x8e) as *const i16) as i32;
    let (mut melhor, mut maior) = (0usize, 0i32);
    for _ in 0..150 {
        let mut tentativas = 0;
        let p = loop {
            let r = sorteio(*(0xae2364 as *const i32));
            tentativas += 1;
            if tentativas >= 100 {
                break 0;
            }
            let p = (*(0xae23bc as *const usize)).wrapping_add((r as usize).wrapping_mul(110));
            if *((p + 0x18) as *const i8) < 0x25 || *((p + 0x22) as *const u8) < 0x32 || u(p + 0x69) == 0 {
                continue;
            }
            if u(p + 0x61) != 0 && u(p + 0x39) != 0 {
                continue;
            }
            if u(clube + 0xcf) == p {
                continue;
            }
            break p;
        };
        if p == 0 {
            continue;
        }
        let nacao = u(p + 0x1a);
        let mut peso = sorteio(reputacao(nacao).wrapping_mul(*((p + 0x22) as *const u8) as i32));
        let mesma_nacao = if assunto == 0 {
            if nacao != u(clube + 0x53) {
                continue;
            }
            true
        } else {
            nacao == u(assunto + 0x1a)
        };
        if mesma_nacao {
            peso = peso.wrapping_add(peso);
        } else {
            let corte = sorteio(100).wrapping_add(0x1964);
            if reputacao(nacao) < corte
                || core::ptr::read_unaligned((u(p + 0x69) + 0xc) as *const i16) < 0x1770
                || nao_gosta(assunto, p) != 0
            {
                continue;
            }
        }
        if melhor == 0 || peso > maior {
            maior = peso;
            melhor = p;
        }
    }
    melhor
}

unsafe extern "stdcall" fn comparar_apagar_repetidos_das_convocacoes(nacao: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(508);
    if nacao == 0 {
        return o(nacao);
    }
    let [a, b] = convocacoes_das_selecoes(nacao);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 200), (b, 200)], || o(nacao), || apagar_repetidos_das_convocacoes(nacao));
    PLACAR.registrar(igual, || format!("apagar_repetidos_das_convocacoes {:x?} x {:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_vaga_do_escalado(tatica: usize, clube: usize, pessoa: *const i32, time: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(509)(tatica, clube, pessoa, time);
    if clube == 0 || pessoa.is_null() {
        return r;
    }
    let n = vaga_do_escalado(tatica, clube, pessoa, time);
    PLACAR.registrar(r == n, || format!("vaga_do_escalado(time {}) original={} rust={}", time, r, n));
    r
}

unsafe extern "thiscall" fn comparar_escolher_comentarista(this: usize, clube: usize, assunto: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(510);
    if assunto == 0 && clube == 0 {
        return o(this, clube, assunto);
    }
    // o comentarista sai só do sorteio
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube, assunto),
        || crate::aleatorio::sem_contar(|| escolher_comentarista(this, clube, assunto)));
    PLACAR.registrar(igual && r == n, || format!("escolher_comentarista {:#x} x {:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

