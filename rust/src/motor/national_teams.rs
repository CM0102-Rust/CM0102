use super::*;

// ---------- octogesima quarta leva: competicao da pessoa, bytes da selecao, selecoes especiais, data com marca ----------

/// { 0x7751e0 } (ret 4, national_teams.cpp): a competicao (0xae23d0, 107 bytes) do indice +4 do
/// registro da pessoa em [this+4] (110 bytes); sem pessoa ou indice negativo, 0.
pub(crate) unsafe extern "thiscall" fn competicao_da_pessoa(this: usize, pessoa: usize) -> usize {
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
pub(crate) unsafe fn byte_da_selecao(this: usize, clube: usize, flag: u32, o: usize) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
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
pub(crate) unsafe extern "thiscall" fn byte_42_da_selecao(this: usize, clube: usize, flag: u32) -> u8 {
    byte_da_selecao(this, clube, flag, 0x42)
}

/// { 0x775520 } (ret 8): o byte +0x43 do bloco da selecao do clube.
pub(crate) unsafe extern "thiscall" fn byte_43_da_selecao(this: usize, clube: usize, flag: u32) -> u8 {
    byte_da_selecao(this, clube, flag, 0x43)
}

/// { 0x775790 } (stdcall): selecao (0x53b900) ou um dos clubes de 0x9d00a4 e 0x9d00a8?
pub(crate) unsafe extern "stdcall" fn selecao_ou_especial(clube: usize) -> u32 {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if e_selecao(clube) != 0 {
        return 1;
    }
    let id = *(clube as *const u32);
    (id == *(0x9d00a4 as *const u32) || id == *(0x9d00a8 as *const u32)) as u32
}

/// { 0x776380 } (stdcall): a pessoa nao gosta (0x547d30) da selecao principal (0x53b3d0) da nacao +0x1a?
pub(crate) unsafe extern "stdcall" fn nao_gosta_da_selecao(pessoa: usize) -> u32 {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let s = principal(*((pessoa + 0x1a) as *const usize));
    (nao_gosta(pessoa, s) != 0) as u32
}

/// { 0x777590 } (ret 4): copia a data (0x417c10) e o byte +8 de `origem`; devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_data_e_marca(this: usize, origem: usize) -> usize {
    let mut tmp = [0u8; 8];
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    copiar(this, tmp.as_mut_ptr() as usize, origem);
    *((this + 8) as *mut u8) = *((origem + 8) as *const u8);
    this
}

// ---------- octogesima terceira leva: mais campos por pessoa (national_teams.cpp), global 0x9b5984 ----------

/// O bloco `k` da pessoa nos registros de [this+4]: id x 110 + k x 20.
pub(crate) unsafe fn bloco_da_pessoa(this: usize, pessoa: usize, k: i8) -> usize {
    (*((this + 4) as *const usize))
        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
}

/// { 0x76c0f0 } (ret 8): liga (1) o byte +0x44 do bloco `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn ligar_44_da_pessoa(this: usize, pessoa: usize, k: i8) {
    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *mut u8) = 1;
}

/// { 0x76e010 } (ret 8): o byte +0x44 do bloco `k` da pessoa, com sinal.
pub(crate) unsafe extern "thiscall" fn byte_44_da_pessoa(this: usize, pessoa: usize, k: i8) -> i32 {
    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *const i8) as i32
}

/// { 0x76e040 } (ret 8): o endereco do campo +0x3a do bloco `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn campo_3a_da_pessoa(this: usize, pessoa: usize, k: i8) -> usize {
    bloco_da_pessoa(this, pessoa, k) + 0x3a
}

/// { 0x76e070 } (stdcall): sem selecao (+0x24), 0xff; segunda selecao (0x5409b0), 2; senao o bit 3
/// do byte +0x5e. Devolve so `al`.
pub(crate) unsafe extern "stdcall" fn classe_pela_selecao(pessoa: usize) -> u8 {
    let selecao = *((pessoa + 0x24) as *const usize);
    if selecao == 0 {
        return 0xff;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if segunda(selecao) != 0 {
        return 2;
    }
    (*((pessoa + 0x5e) as *const u8) >> 3) & 1
}


// ---------- 270a leva: as notícias das seleções aos clubes: o relatório do jogo, a convocação e os adversários (national_teams.cpp) ----------



/// Os valores de 4 bytes dos itens de uma lista (nó: +0 o dado, +6 o próximo; cabeça em +2), sem mexer no cursor.
pub(crate) unsafe fn cqf11_itens(lista: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = read_usize(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(read_usize(read_usize(no)) as u32);
        no = read_usize(no + 6);
    }
    v
}

/// O ensaio do relatório (0x7706c0): os sorteios vêm da fita da original e a notícia não é entregue.
pub(crate) static mut CQF11_FITA: Option<(Vec<(i32, i32)>, usize, bool)> = None;

pub(crate) unsafe fn cqf11_sortear(n: i32) -> i32 {
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
            let s: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
            s(n)
        }
    }
}

/// O código de como o jogador foi no jogo (do registro dele no lado do clube, +0x190 + 0x49 × k): pelo tipo +0x21
/// (1 → 2, 2 → 8, 3 → 0x20, 4 a 8 → 0x40), +4 com nota (+0x34), +0x10 se é o melhor da partida (+0x3c = o id da ficha
/// +0x61), +1 se +0x23 − +0x24 é 1; 0 se ele não está no registro.
pub(crate) unsafe fn cqf11_codigo(resultado: usize, lado: usize, jogador: usize) -> u32 {
    let id = read_usize(jogador);
    for k in 0..0x14usize {
        let e = resultado + 0x190 + lado * 0x625 + k * 0x49;
        if read_usize(e) != id {
            continue;
        }
        let mut c: u32 = match read_u8(e + 0x21) {
            1 => 2,
            2 => 8,
            3 => 0x20,
            4..=8 => 0x40,
            _ => 0,
        };
        if read_u8(e + 0x34) != 0xff {
            c |= 4;
        }
        if read_usize(resultado + 0x3c) == read_usize(read_usize(jogador + 0x61)) {
            c |= 0x10;
        }
        if (read_u8(e + 0x23) as i32) - (read_u8(e + 0x24) as i32) == 1 {
            c |= 1;
        }
        return c;
    }
    0
}

/// the match squad is sorted by club; each group of the same club becomes news 0x24 to its manager: the match data, the
/// star player, how many, up to 4 players (the star first, the others by how they did), draw(2) and the codes; the
/// group players leave the array
/// Original: `NationalTeams::send_club_international_report_news` (0x7706c0, national_teams.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x7706c0 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH da notícia local): `NationalTeams::
/// send_club_international_report_news(clube, jogo, resultado, destaque, elenco, conta)`, pelo resultado entre seleções
/// (0x76b600), de cada lado. O elenco (vetor de `conta` pessoas, o byte baixo) é ordenado pelo clube (0x945333 com
/// 0x778ef0); cada grupo de jogadores do mesmo clube vira uma notícia 0x24 ao técnico do clube (0x7861e0): o registro
/// da partida pelo clube (0x7e4750, uma vez só) e os dados dele (0x7e4de0), o destaque (id e chaves, se é do clube;
/// senão −1 nos quatro), quantos são, o clube, até 4 jogadores (o destaque primeiro, os outros pelo código de como
/// foram, do maior para o menor), 0x420e9, sorteio(2) e os códigos. Os jogadores do grupo saem do vetor (zerados).
pub(crate) unsafe extern "thiscall" fn cqf11_relatorio_da_selecao(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
    let _ = this;
    let q = conta as u8 as i8;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let registro_da_partida: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(FN_FILL_MATCH_RECORD_FROM_FIXTURE);
    let dados_da_partida: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(FN_WRITE_MATCH_RECORD_TO_NEWS);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    // 0x945333 (qsort): push 0x778ef0; push 4; push conta; push elenco; add esp, 0x10 -> cdecl; 0x77bbe0: push 0; push
    // 0x24; lea ecx, notícia -> thiscall; 0x785790: push valor; push índice; lea ecx, notícia -> thiscall, ret 8;
    // 0x7e4750: push &registro; push clube; push jogo; add esp, 0xc -> cdecl; 0x7e4de0: push &índice; push &notícia;
    // push &registro; add esp, 0xc -> cdecl; 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
    ordenar(elenco as usize, q as i32, 4, 0x778ef0);
    if q <= 0 {
        return;
    }
    let lado = (clube != read_usize(jogo + 0x1c)) as usize;
    let mut registro = [0u8; 0x20]; // a Data em +0 (0x54a110 na entrada), o resto a 0x7e4750 preenche
    let reg = registro.as_mut_ptr() as usize;
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    data_padrao(reg);
    let mut montado = false;
    let mut noticia = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = noticia.as_mut_ptr() as usize;
    let mut i: i8 = 1;
    loop {
        let slot = elenco.add(i as usize - 1);
        let primeiro = *slot;
        if primeiro != 0 && read_usize(primeiro + 0x39) != 0 {
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
                    if read_usize(outro + 0x39) != read_usize(primeiro + 0x39) {
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
            if read_usize(destaque + 0x39) == read_usize(primeiro + 0x39) {
                pr(read_usize(destaque) as u32);
                pr(read_usize(read_usize(destaque + 4) + 0x33) as u32);
                pr(read_usize(read_usize(destaque + 8) + 0x33) as u32);
                pr(read_usize(read_usize(destaque + 0xc) + 0x33) as u32);
            } else {
                for _ in 0..4 {
                    pr(u32::MAX);
                }
            }
            pr(quantos as i32 as u32);
            pr(read_usize(read_usize(primeiro + 0x39)) as u32);
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
                pr(read_usize(p) as u32);
                pr(read_usize(read_usize(p + 4) + 0x33) as u32);
                pr(read_usize(read_usize(p + 8) + 0x33) as u32);
                pr(read_usize(read_usize(p + 0xc) + 0x33) as u32);
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
                entregar(read_usize(0xb7431c), nt, read_usize(primeiro + 0x39));
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
pub(crate) unsafe extern "thiscall" fn comparar_cqf11_relatorio_da_selecao(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
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

/// the called-up players (flag 4, or 8 for the second team) of human-managed clubs, grouped by club, become news 0x1d
/// to the manager: national team, club, type, how many, the opponents (given list or built by 0x76d560) and up to 3
/// players
/// Original: `national_teams_post_call_up_news` (0x76e0b0, national_teams.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x76e0b0 } (thiscall, ret 0x1c, national_teams.cpp, quadro SEH da notícia local): `national_teams_post_call_up_news(
/// seleção, segunda, jogo, tipo, sem_adversarios, adversarios, conta)`, pela convocação (0x76c120) e pela tela. Das 50
/// vagas do elenco da seleção (+0xd7, ponteiros), cada jogador com a marca (+0x5e: 8 na segunda seleção, senão 4) e
/// clube de técnico humano (0x7875e0) entra no grupo do clube dele; quando o próximo da vaga não é do mesmo clube (ou
/// não tem a marca, ou acabou), sai a notícia 0x1d ao técnico do clube (0x7861e0): a seleção, o clube, `sem_adversarios`,
/// o tipo, quantos, `segunda`, quantos adversários e, com menos de 4, os ids deles (da lista dada ou montada uma vez pela
/// 0x76d560 com o jogo, sem lista e sem `sem_adversarios`), e até 3 jogadores (id e chaves). A lista montada aqui é
/// destruída e liberada no fim (0x68d040, 0x944c94).
pub(crate) unsafe extern "thiscall" fn cqf11_noticia_da_convocacao(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let montar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_NATIONAL_TEAMS_COLLECT_FIXTURE_OPPONENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
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
        let p = read_usize(selecao + 0xd7 + (idx - 1) * 4);
        if p == 0 || read_u8(p + 0x5e) & marca == 0 || read_usize(p + 0x39) == 0 || humano(read_usize(0xb7431c), read_usize(p + 0x39)) == 0 {
            continue;
        }
        if lista == 0 && sem_adversarios == 0 && jogo != 0 {
            lista = montar(this, selecao, jogo);
            quantos_adv = *(lista as *const i16);
        }
        if n < 3 {
            tres[n as usize] = p;
        }
        let clube = read_usize(p + 0x39);
        n = n.wrapping_add(1);
        if idx < 0x32 {
            let prox = read_usize(selecao + 0xd7 + idx * 4);
            if prox != 0 && read_usize(prox + 0x39) == clube && read_u8(prox + 0x5e) & marca != 0 {
                continue;
            }
        }
        crate::comparacao::ramo("cqf_11 noticia da convocacao");
        construir(nt, 0x1d, 0);
        gravar(nt, 0, read_usize(selecao) as u32);
        gravar(nt, 1, read_usize(clube) as u32);
        gravar(nt, 2, sem_adversarios);
        gravar(nt, 3, tipo);
        gravar(nt, 4, n as i32 as u32);
        gravar(nt, 5, segunda);
        gravar(nt, 6, quantos_adv as i32 as u32);
        if quantos_adv < 4 && lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
            let mut k: u8 = 7;
            let mut item = proximo(lista);
            while item != 0 {
                gravar(nt, k as u32, read_usize(read_usize(item)) as u32);
                k = k.wrapping_add(1);
                item = proximo(lista);
            }
        }
        if n <= 3 && n > 0 {
            let mut k: u8 = 0xa;
            for t in 0..n as usize {
                let j = tres[t];
                gravar(nt, k as u32, read_usize(j) as u32);
                gravar(nt, k.wrapping_add(1) as u32, read_usize(read_usize(j + 4) + 0x33) as u32);
                gravar(nt, k.wrapping_add(2) as u32, read_usize(read_usize(j + 8) + 0x33) as u32);
                gravar(nt, k.wrapping_add(3) as u32, read_usize(read_usize(j + 0xc) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
        }
        entregar(read_usize(0xb7431c), nt, clube);
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
pub(crate) unsafe extern "thiscall" fn comparar_cqf11_noticia_da_convocacao(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32)>(1624)(this, selecao, segunda, jogo, tipo, sem_adversarios, adversarios, conta);
}

/// O ensaio da 0x76d560: os valores que iriam para a lista nova, sem alocar.
pub(crate) static mut CQF11_PLANO: Option<Vec<u32>> = None;

/// a new list with the match opponent (its nation's first team, for the second team) and the opponents of the fixtures
/// from tomorrow to 13 days later that are not finals
/// Original: `national_teams_collect_fixture_opponents` (0x76d560, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x76d560 } (thiscall, ret 8, national_teams.cpp, quadro SEH da lista local): `national_teams_collect_fixture_
/// opponents(seleção, jogo)`, pela notícia da convocação (0x76e0b0) e pelo pedido ao técnico (0x76f1a0). Uma lista
/// nova (0x944cf1 com 0xe, 0x68d020) com o adversário do jogo (o visitante se o mandante é da nação da seleção, senão o
/// mandante; na segunda seleção, a seleção principal da nação dele, 0x53b4f0) e os adversários dos outros jogos da
/// seleção de amanhã até 13 dias depois (0x5acbe0, com o byte +0x40 do jogo) que não são de fase final (as nove
/// competições de seleções). Devolve a lista.
pub(crate) unsafe extern "thiscall" fn cqf11_adversarios_da_selecao(this: usize, selecao: usize, jogo: usize) -> usize {
    let _ = this;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
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
                p.push(read_usize(v) as u32);
            }
        } else {
            acrescentar(nl, v, 4);
        }
    };
    // o 9º argumento da 0x5acbe0: o dword do argumento `jogo` na pilha (ou do 0x53b4f0 gravado em cima) com o byte baixo
    // trocado pelo +0x40 do jogo
    let mut celula: usize = jogo;
    let mandante = jogo + 0x1c;
    if read_usize(read_usize(mandante) + 0x53) == read_usize(selecao + 0x53) {
        if segunda(selecao) != 0 {
            celula = principal(read_usize(read_usize(jogo + 0x20) + 0x53));
            por(&celula as *const usize as usize);
        } else {
            por(jogo + 0x20);
        }
    } else if segunda(selecao) != 0 {
        celula = principal(read_usize(read_usize(mandante) + 0x53));
        por(&celula as *const usize as usize);
    } else {
        por(mandante);
    }
    celula = (celula & !0xff) | read_u8(jogo + 0x40) as usize;
    let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    let dia_ano = |p: usize| (read_usize(p) as u32, core::ptr::read_unaligned((p + 2) as *const u32));
    data_nova(p0, *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    mais(p0, p1, 1);
    mais(p1, p2, 0xd);
    let (a1, a2) = dia_ano(p1);
    let (a3, a4) = dia_ano(p2);
    achar(read_usize(0xae2a58), a1, a2, a3, a4, read_usize(selecao) as u32, l, -1, -2, celula as u32);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let finais = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
    let mut item = proximo(l);
    while item != 0 {
        let c = read_usize(item + 0x14);
        if c == 0 || !finais.iter().any(|&k| read_usize(c) == read_usize(k)) {
            if read_usize(read_usize(item + 0x1c) + 0x53) == read_usize(selecao + 0x53) {
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
pub(crate) unsafe extern "thiscall" fn comparar_cqf11_adversarios_da_selecao(this: usize, selecao: usize, jogo: usize) -> usize {
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



/// Os códigos de posição das vagas da original: elenco de 26 (0x76c771..0x76c84c), de 24 (0x76c870..0x76c92c) e o de
/// qualquer outro tamanho (0x76c939..0x76c9de, 22 vagas), com os comuns gravados depois (0x76c860/0x76c865 e as vagas
/// 0, 1, 3, 5, 6 e 21). O código 0xc da vaga 12/11/10 vem do ebp (0xc desde 0x76c2e4/0x76c6a9).
pub(crate) const CQC12_26: [u16; 26] = [1, 1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240, 0x240];
pub(crate) const CQC12_24: [u16; 24] = [1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
pub(crate) const CQC12_22: [u16; 22] = [1, 1, 1, 0x202, 0x204, 0x204, 0x204, 0x204, 0x84, 0x804, 0xc, 0x210, 0x210, 0x220, 0x90, 0x810,
    0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
/// As vagas que a tabela de 24 tem a mais que a de 22, na ordem em que aparecem nela: 0x202 (líbero/centro, a vaga
/// 3), 0x88 e 0x808 (volantes pelos lados, 9 e 10) e 0x210 (o 3º meia central, 14).
pub(crate) const CQC12_EXTRAS: [u16; 3] = [0x202, 0x88, 0x808];

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
pub(crate) unsafe fn cqc12_codigos(tam: u8) -> [u16; 50] {
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
pub(crate) unsafe fn cqc12_registro(this: usize, nacao: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(nacao).wrapping_mul(0x6e))
}

/// As faixas da reputação aceita (+9 da ficha +0x61): (mínimo exclusive, máximo), −1 e 0x2711 fora da Austrália
/// ([0x9cf224]); para ela, pela competição da partida (amistoso 0x5c6710 de [0xae2c74], ou a da Oceania: 0x8ca/0x2134;
/// Copa das Confederações 0xabe/0x222e; eliminatórias da Oceania 0x8ca e 9000, 8000 ou 10001 pelo byte +0x42; Copa
/// do Mundo 0xabe/10000).
pub(crate) unsafe fn cqc12_limites(nacao: usize, p: usize) -> (i16, i16) {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    // 0x5c6710: push [p + 0x14]; mov ecx, [0xae2c74] -> thiscall ret 4
    if read_usize(nacao) != read_usize(KEY_NATION_AUSTRALIA) {
        return (-1, 0x2711);
    }
    if p == 0 || (read_usize(0xae2c74) != 0 && amistoso(read_usize(0xae2c74), read_usize(p + 0x14)) != 0) {
        return (0x8ca, 0x2134);
    }
    let k = read_usize(read_usize(p + 0x14));
    if k == read_usize(KEY_CLUB_COMP_OCEANIA_NATIONS_CUP) {
        (0x8ca, 0x2134)
    } else if k == read_usize(KEY_CLUB_COMP_CONFEDERATIONS_CUP) {
        (0xabe, 0x222e)
    } else if k == read_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
        (0x8ca, match read_u8(p + 0x42) { 1 => 0x1f40, 2 => 0x2711, _ => 0x2328 })
    } else if k == read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) {
        (0xabe, 0x2710)
    } else {
        (-1, 0x2711)
    }
}

/// O time do tipo (0x53b3d0 a principal nos tipos 0 e 1, 0x53b4f0 o sub-21), a marca de elenco (+0x5e: 8 no tipo
/// 1, 4 nos outros) e o time da tática (1 no tipo 1).
pub(crate) unsafe fn cqc12_time(nacao: usize, t: i8) -> (usize, u8, u32) {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    // 0x53b3d0 e 0x53b4f0: push nacao; add esp, 4 -> cdecl
    match t {
        1 => (principal(nacao), 8, 1),
        0 => (principal(nacao), 4, 0),
        _ => (sub21(nacao), 4, 0),
    }
}

/// Os tipos de reserva percorridos: só o do tipo, fora o sub-21 (2) dos Jogos Olímpicos ([0x9cf7a0]), que olha as
/// reservas 0 e 1; o sub-21 fora deles olha a 1.
pub(crate) unsafe fn cqc12_faixa(t: i8, p: usize) -> (usize, usize) {
    if t != 2 {
        (0, 0)
    } else if p != 0 && read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
        (0, 1)
    } else {
        (1, 1)
    }
}

/// As entradas das reservas do tipo `k`, grupo `g`: (vetor [this + 0xc + 4k] a partir do início, quantas). O início é
/// o dword +8 + 0x15k do registro mais os bytes +0xe + 0x15k + 4j dos grupos anteriores; a quantidade, o byte +0xd +
/// 0x15k + 4g (como a original lê: a soma e a quantidade saem de bytes vizinhos).
pub(crate) unsafe fn cqc12_grupo(this: usize, reg: usize, k: usize, g: usize) -> (usize, i32, i32) {
    let base = reg + 0xd + 0x15 * k;
    let mut ini = read_usize(base - 5) as i32;
    for j in 0..g {
        ini = ini.wrapping_add(read_u8(base + 1 + 4 * j) as i32);
    }
    let fim = ini.wrapping_add(read_u8(base + 4 * g) as i32);
    (read_usize(this + 0xc + 4 * k).wrapping_add((ini as usize).wrapping_mul(4)), ini, fim)
}

/// O jogador `j` das reservas entra na lista da escala? (0x76cb84..0x76cf6c)
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn cqc12_candidato(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), j: usize) -> bool {
    let tipo_ok: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let bits: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_CAN_CHANGE_SQUAD);
    // 0x53b620: push byte +0x3d; add esp, 4 -> cdecl; 0x62c400: push j; mov ecx, [0xb63d28] -> thiscall ret 4 (al);
    // 0x7a7eb0: push j; mov ecx, [0xb7434c] -> stdcall ret 4 (al); 0x547d30: push clube; push j; add esp, 0xc (junto
    // com o da 0x53b3d0) -> cdecl; 0x4cf5a0: push j; mov ecx, [0xae1050] -> thiscall ret 4; 0x55d270: push 0; push 1;
    // push [p + 0x14]; push j; mov ecx, [0xae28f0] -> thiscall ret 0x10; 0x76db00: 6 pushes (nação, 0, j, 0, 0, 0);
    // mov ecx, this -> thiscall ret 0x18
    if j == 0 || tipo_ok(read_u8(j + 0x3d) as u32) == 0 || lesao(read_usize(0xb63d28), j) != 0 {
        return false;
    }
    if read_usize(0xb7434c) != 0 && bits(j) != 0 {
        return false;
    }
    if read_usize(j + 0x39) == 0 || nao_gosta(j, principal(read_usize(j + 0x1a))) != 0 {
        return false;
    }
    let tj = read_usize(j + 0x24);
    if tj != 0 && (read_u8(j + 0x28) != 0xb || read_usize(tj + 0x53) != nacao) {
        return false;
    }
    if p != 0 {
        let k = read_usize(p);
        let c = read_usize(j + 0x39);
        let outra_nacao = |c: usize| c != 0 && read_usize(c + 0x53) != 0 && read_usize(read_usize(c + 0x53) + 0x71) != read_usize(nacao + 0x71);
        let mut contrato_ok = true;
        if k == read_usize(KEY_CLUB_COMP_FRIENDLY) || k == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) || k == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
            if outra_nacao(c) {
                // amistoso, B e sub-21 internacionais: clube de outra nação só com reputação (+0x80) até 0.8 da do
                // adversário (o mandante +0x1c é o próprio time: o visitante +0x20); fora dos amistosos, não
                if k != read_usize(KEY_CLUB_COMP_FRIENDLY) {
                    return false;
                }
                let s = read_usize(p + 0x1c);
                let outro = if s == elenco { read_usize(p + 0x20) } else { s };
                if read_i16(c + 0x80) as f64 * 0.8 > read_i16(outro + 0x80) as f64 {
                    return false;
                }
            }
            // clube com jogo (a tabela +0x24 pelo id do clube) e contrato com +0x3d acima de −10: não
            if read_u8(read_usize(this + 0x24).wrapping_add(read_usize(c))) != 0 {
                let x = contrato(read_usize(0xae1050), j);
                if x != 0 && (read_u8(contrato(read_usize(0xae1050), j) + 0x3d) as i8) > -10 {
                    contrato_ok = false;
                }
            }
            if !contrato_ok {
                return false;
            }
        } else if outra_nacao(c) && read_usize(c + 0x57) != 0
            && (read_i16(read_usize(p + 0x14) + 0x69) as f64) < read_i16(read_usize(c + 0x57) + 0x69) as f64 * 0.7
        {
            // nas outras competições, clube de outra nação cuja liga (+0x57) tem +0x69 acima da da competição / 0.7
            return false;
        }
        if suspenso(read_usize(0xae28f0), j, read_usize(p + 0x14), 1, 0) != 0 {
            return false;
        }
        if t == 0 {
            let m = cqc12_uw(p + 0x4d) & 0x300;
            let tj = read_usize(j + 0x24);
            if m != 0x100 && tj == elenco && read_u8(j + 0x5e) & 8 != 0 {
                return false;
            }
            if m != 0x200 && tj != 0 && tj != elenco {
                return false;
            }
        }
    }
    if read_usize(j + 0x24) == principal(nacao) && read_u8(j + 0x5e) & 4 != 0 {
        if pode_mudar(this, nacao, 0, j, 0, 0, 0) == 0 {
            return false;
        }
        if !(p != 0 && read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES)) && t != 0 {
            return false;
        }
    }
    // com a marca, só depois da data do elenco do tipo (ano +2 sem sinal, dia +0 com sinal)
    let depois = |a: usize| {
        let (ya, da) = (cqc12_uw(data + 2), read_i16(data));
        let (yr, dr) = (cqc12_uw(a + 2), read_i16(a));
        ya > yr || (ya == yr && da > dr)
    };
    if read_u8(j + 0x5e) & 8 != 0 && !depois(reg + 0x4e) {
        return false;
    }
    let tj = read_usize(j + 0x24);
    if tj == sub21(nacao) && read_u8(j + 0x5e) & 4 != 0 && !depois(reg + 0x62) {
        return false;
    }
    if t == 2 && read_u8(j + 0x18) as i8 > 0x17 { // cqc12: a idade do sub-21
        return false;
    }
    let nat = read_usize(j + 0x1a);
    if nat != nacao {
        // de outra nação: sem time, sem a marca +0x22, sem o bit 2 do +0xc do registro de 79 bytes, com a segunda
        // nação (+0x1e) nesta; entra se esta tiver +0x8e maior, ou (clube da mesma confederação +0x71 ou sem clube)
        // com +0x85 de 5 para cima
        if tj != 0 || read_u8(j + 0x22) != 0 || read_u8(read_usize(0xae2d0c).wrapping_add(read_usize(j).wrapping_mul(0x4f)) + 0xc) & 2 != 0
            || read_usize(j + 0x1e) != nacao || nat == 0
        {
            return false;
        }
        let c = read_usize(j + 0x39);
        let n2 = if c != 0 { read_usize(c + 0x53) } else { 0 };
        if read_i16(nacao + 0x8e) <= read_i16(nat + 0x8e) {
            if n2 != 0 && read_usize(nacao + 0x71) != read_usize(n2 + 0x71) {
                return false;
            }
            if (read_u8(nacao + 0x85) as i8) < 5 {
                return false;
            }
        }
    }
    let v = read_i16(read_usize(j + 0x61) + 9);
    v > lim.0 && v <= lim.1
}

/// A lista da escala (0x76cae5..0x76cfe6): as reservas dos tipos da faixa, grupo a grupo; devolve a quantidade (byte,
/// como a original) e deixa em `cursor` o ponteiro da última entrada percorrida mais 4 (a original o lê depois).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn cqc12_candidatos(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), cand: &mut [usize; 200], cursor: &mut usize) -> u8 {
    let mut n = 0u8;
    let (lo, hi) = cqc12_faixa(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (v, ini, fim) = cqc12_grupo(this, reg, k, g);
            *cursor = v;
            for _ in ini..fim {
                let j = read_usize(*cursor);
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

/// the national squad selection (senior, B/Olympic or under-21): in the finals the list item; the squad dates, the ages
/// and the size; with a human manager, the request to him; otherwise those from another side and the flagged ones
/// leave, the pool list goes to the slot assignment and each picked player who fits the slot joins the squad (leaving
/// the previous side and the tactic slots); the sorting, the news and the tactic slots. Approved fix of the original
/// (01/11/2026): position codes also for 23- and 25-man squads (the original left stack leftovers in the slots from the
/// 22nd on): the original 22 plus, in order, 0x202, 0x88 and 0x808, the slots the 24 table has beyond the 22 table
/// Original: `NationalTeams::select_squad` (0x76c120, national_teams.cpp, thiscall).
///
/// [pt] { 0x76c120 } (thiscall, ret 0x14, national_teams.cpp, ~4,9 KB, quadro SEH só das datas locais, sem destrutor):
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
pub(crate) unsafe extern "thiscall" fn escolher_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let inserir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_RELEASE_OVERAGE_U21_PLAYERS);
    let idades: unsafe extern "stdcall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_NATIONAL_TEAMS_GET_SQUAD_SIZE_LIMITS);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let dias_de_lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    let pedir: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_SQUAD_REQUEST_NEWS);
    let escalar: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_CAN_CHANGE_SQUAD);
    let da_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let limpar_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_CLEAR_SELECTION_SLOT);
    let marcar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_CALL_UP_NEWS);
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
    let chaves = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
    let finais = p != 0 && read_usize(p + 0x14) != 0 && chaves.iter().any(|&k| read_usize(read_usize(p + 0x14)) == read_usize(k));
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tatica = || read_usize(0xde1d08);
    if forcado == 0 {
        let (mut d88c, mut d87c, mut d8a0) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p88c, p87c, p8a0) = (d88c.as_mut_ptr() as usize, d87c.as_mut_ptr() as usize, d8a0.as_mut_ptr() as usize);
        if finais {
            crate::comparacao::ramo("cqc_12 elenco das fases finais");
            let d = data_nova(p87c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            menos(d, p88c, 10);
            let comp = read_usize(p + 0x14);
            if read_usize(this) == 0 {
                let m = novo(0xe);
                write_usize(this, if m != 0 { lista_nova(m) } else { 0 });
            }
            let mut item = [0u8; 0x12];
            let pi = item.as_mut_ptr() as usize;
            padrao(pi + 8);
            write_usize(pi + 4, read_usize(comp));
            write_usize(pi, read_usize(elenco));
            copia(pi + 8, p87c, p88c);
            core::ptr::write_unaligned((pi + 0x10) as *mut u16, 1000);
            inserir(read_usize(this), pi, 0x12);
        }
        if p == 0 {
            let x = menos(data, p88c, 1);
            copia(r + 0x3a, p87c, x);
        } else if t != 1 && t != 2 {
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            copia(r + 0x3a, p87c, x);
        } else {
            // a nação de cada lado (+0x53 do time) e a casa dela no vetor [this + 0x36]
            let nac = |o: usize| read_usize(read_usize(p + o) + 0x53);
            let casa = |o: usize| read_usize(nac(o)).wrapping_add(read_usize(this + 0x36));
            if read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
                write_u8(casa(0x1c), 1);
                write_u8(casa(0x20), 1);
            } else {
                if read_u8(casa(0x1c)) != 0 {
                    dispensar(this, nac(0x1c));
                }
                if read_u8(casa(0x20)) != 0 {
                    dispensar(this, nac(0x20));
                }
                write_u8(casa(0x1c), 0);
                write_u8(casa(0x20), 0);
            }
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            let y = menos(x, p87c, 1);
            copia(r + 0x3a, p8a0, y);
        }
        copia(r + 0x32, p88c, data);
        idades(p, tipo, r + 0x42, r + 0x43);
        let tecnico = read_usize(principal(nacao) + 0xcf);
        let decide = || {
            (humano(elenco, 0, 1, 0, 0) != 0 && (t == 0 || t9a06(read_usize(0xb63c98), tecnico, 0) != 0))
                || (segunda(elenco) != 0 && read_usize(elenco + 0xcf) == 0
                    && humano(principal(read_usize(elenco + 0x53)), 0, 1, 0, 0) != 0
                    && t9a06(read_usize(0xb63c98), tecnico, 0) != 0)
        };
        if decide() {
            for k in 0..50 {
                let s = elenco + 0xd7 + 4 * k;
                let j = read_usize(s);
                if j == 0 || lesao(read_usize(0xb63d28), j) != 2 || read_u8(j + 0x5e) & marca == 0 {
                    continue;
                }
                let l = dias_de_lesao(read_usize(0xb63d28), j);
                if l != 0 && read_i16(l + 0x2e) > 10 {
                    tirar(tatica(), elenco, j, time, 1);
                    write_u8(j + 0x5e, read_u8(j + 0x5e) & 3);
                    write_usize(j + 0x24, 0);
                    write_u8(j + 0x28, if read_u8(j + 0x28) == 0xc { 5 } else { 0 });
                    write_usize(s, 0);
                }
            }
        }
        if decide() {
            crate::comparacao::ramo("cqc_12 elenco pelo técnico humano");
            write_u8(r + 0x44, 0);
            pedir(this, elenco, time, p);
            return;
        }
    }
    let tam = r + 0x42;
    let codigos = cqc12_codigos(read_u8(tam));
    // saem do elenco: os de outro time; os com a marca; no tipo 0 com o bit 1 de +0x4e da partida, os sem ela
    for k in 0..50 {
        let s = elenco + 0xd7 + 4 * k;
        let j = read_usize(s);
        if j == 0 {
            continue;
        }
        if read_usize(j + 0x24) != elenco {
            tirar(tatica(), elenco, j, time, 1);
        } else {
            let antigo = if read_u8(j + 0x5e) & marca != 0 {
                5
            } else if t == 0 && read_u8(p + 0x4e) & 1 != 0 {
                0xb
            } else {
                continue;
            };
            tirar(tatica(), elenco, j, time, 1);
            write_u8(j + 0x5e, read_u8(j + 0x5e) & 3);
            write_usize(j + 0x24, 0);
            write_u8(j + 0x28, if read_u8(j + 0x28) == 0xc { antigo } else { 0 });
        }
        write_usize(s, 0); // cqc12: a vaga do elenco fica livre
    }
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let n = cqc12_candidatos(this, nacao, t, data, p, elenco, reg, lim, &mut cand, &mut cursor);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut saida = [0usize; 50];
    crate::comparacao::ramo("cqc_12 escala do elenco");
    escalar(cand.as_ptr() as usize, n as u32, codigos.as_ptr() as usize, (modo & !0xff) | read_u8(tam) as u32, 0,
        saida.as_mut_ptr() as usize, modo, elenco, 0, 0, 0, 0, 0, 0, 0);
    let mut i = 0usize;
    // ponytail: a original não limita a 50 (o jogo pede até 26)
    while (i as i32) < read_u8(tam) as i8 as i32 && i < 50 {
        let j = saida[i];
        if j != 0 && codigos[i] & mascara(read_usize(j + 0x61)) != 0 {
            if let Some(k) = (0..50).find(|&k| read_usize(elenco + 0xd7 + 4 * k) == 0) {
                write_usize(elenco + 0xd7 + 4 * k, j);
            }
            let tj = read_usize(j + 0x24);
            if tj != elenco && tj != 0 {
                // de outro time: sai da vaga de lá (com a permissão, a marca de convocado do tipo dele zera) e da tática
                let v = ((read_u8(j + 0x5e) as i8 as i32 & 8) >> 3) as u32;
                let c3: u8 = if segunda(tj) == 0 { (v != 0) as u8 } else if v != 0 { 1 } else { 2 };
                let mut c = read_usize(j + 0x24) + 0xd7;
                for _ in 0..50 {
                    if read_usize(c) == j {
                        write_usize(c, 0);
                        if pode_mudar(this, read_usize(read_usize(j + 0x24) + 0x53), c3 as u32, 0, 0, 0, 0) != 0 {
                            write_u8(reg + 0x43 + c3 as usize * 0x14, 0);
                            break;
                        }
                        aviso_em(0xa5b2a4, 0x748);
                    }
                    c += 4;
                }
                cursor = c;
                tirar(tatica(), read_usize(j + 0x24), j, v, 1);
            } else if tj == elenco && read_u8(j + 0x5e) & marca == 0 {
                // já do time sem a marca: sai das vagas da tática do time (time da tática invertido)
                let bz = (time == 0) as u32;
                let c3: u8 = if segunda(elenco) == 0 { (bz != 0) as u8 } else if bz != 0 { 1 } else { 2 };
                for v in 0..20 {
                    if da_vaga(tatica(), read_usize(j + 0x24), v, bz) != j {
                        continue;
                    }
                    limpar_vaga(tatica(), read_usize(j + 0x24), v, bz);
                    // a original passa como jogador o dword do último ponteiro de percurso (cursor): reproduzido
                    if c3 != 0 && pode_mudar(this, read_usize(read_usize(j + 0x24) + 0x53), c3 as u32, read_usize(cursor), 0, 0, 0) != 0 {
                        write_u8(reg + 0x43 + c3 as usize * 0x14, 0);
                        break;
                    }
                    aviso_em(0xa5b2a4, 0x76b);
                }
            }
            marcar(j, marca as u32);
            write_usize(j + 0x24, elenco); // cqc12: o convocado ganha o time
            let x = read_u8(j + 0x28);
            write_u8(j + 0x28, if x == 5 || x == 0xc { 0xc } else { 0xb });
            let e = read_usize(0xae2d0c).wrapping_add(read_usize(j).wrapping_mul(0x4f)) + 9;
            write_u8(e, (read_u8(e) as i8).clamp(0, 100) as u8);
        }
        i += 1;
    }
    ordenar(elenco + 0xd7, 0x32, 4, 0x778ef0);
    let chave = if p == 0 { u32::MAX } else { read_usize(read_usize(p + 0x14)) as u32 };
    noticia(this, elenco, time, p, chave, finais as u32, 0, 0);
    for v in 0..20 {
        let x = da_vaga(tatica(), elenco, v, time);
        if x != 0 && read_usize(x + 0x24) != elenco {
            limpar_vaga(tatica(), elenco, v, time);
        }
    }
}

/// O que a escala da 0x76c120 lê e a original muda depois dela: o registro da nação (0x6e bytes) e, de cada jogador
/// das reservas percorridas, +0x24..+0x28 (time, estado) e +0x5e (marcas de elenco).
pub(crate) unsafe fn cqc12_regioes(this: usize, nacao: usize, t: i8, p: usize) -> Vec<(usize, usize)> {
    let reg = cqc12_registro(this, nacao);
    let mut v = vec![(reg, 0x6e)];
    let (lo, hi) = cqc12_faixa(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (a, ini, fim) = cqc12_grupo(this, reg, k, g);
            for i in 0..(fim - ini).max(0) as usize {
                let j = read_usize(a + 4 * i);
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
pub(crate) unsafe extern "thiscall" fn comparar_escolher_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
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
    let tam = read_u8(r + 0x42);
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




/// O clube do time das estrelas `g` ([0x9d00a4] ou [0x9d00a8]): o registro de 0x245 bytes em [0xae23b4].
pub(crate) unsafe fn cpz6_estrelas(g: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(read_usize(g).wrapping_mul(0x245))
}

/// the national side holding the player from the day before to the day after (a national side fixture in the window,
/// against the given opponent if any, or a call-up already started) or the MLS all-star squad on the game date; writes
/// the comment; otherwise 0 (and 0 for a national side or all-star club)
/// Original: `NationalTeams::get_international_duty_club` (0x7757d0, national_teams.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7757d0 } (thiscall, ret 0x14, national_teams.cpp): `NationalTeams::get_international_duty_club(jogador, data,
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
pub(crate) unsafe extern "thiscall" fn selecao_que_tem_o_jogador(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                      adversario: usize) -> usize {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut lista = [0u8; 0x10];
    let pl = lista.as_mut_ptr() as usize;
    criar(pl);
    let sai = |r: usize| -> usize {
        destruir(pl);
        r
    };
    if clube != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        if selecao(clube) != 0 {
            return sai(0);
        }
        let id = read_usize(clube);
        if id == read_usize(0x9d00a4) || id == read_usize(0x9d00a8) {
            return sai(0);
        }
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let (mut lo, mut hi) = ([0u8; 8], [0u8; 8]);
    let (plo, phi) = (lo.as_mut_ptr() as usize, hi.as_mut_ptr() as usize);
    menos(data, plo, 1); // cpz6: a véspera
    mais(data, phi, 1); // cpz6: o dia seguinte
    let nacional = read_usize(jogador + 0x24);
    if nacional != 0 {
        let sem_bit = (read_u8(jogador + 0x5e) & 4 == 0) as u32;
        // ponytail: a original empurra o lado com os 24 bits altos de ecx sobrando; a 0x5acbe0 só lê o byte
        let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        let n = jogos(read_usize(0xae2a58), read_usize(plo) as u32, read_usize(plo + 2) as u32, read_usize(phi) as u32, read_usize(phi + 2) as u32,
            read_usize(nacional) as u32, pl, u32::MAX, 0xffff_fffe, (sem_bit == 0) as u32);
        let mut servico = false;
        if n as u16 != 0 {
            if adversario == 0 {
                servico = true;
            } else {
                core::ptr::write_unaligned((pl + 0xa) as *mut usize, read_usize(pl + 2));
                loop {
                    let j = proximo(pl);
                    if j == 0 {
                        break;
                    }
                    if read_usize(j + 0x14) == adversario {
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
            return sai(read_usize(jogador + 0x24)); // cpz6: a serviço da seleção
        }
        let conv = read_usize(this);
        if conv != 0 && sem_bit == 0 {
            core::ptr::write_unaligned((conv + 0xa) as *mut usize, read_usize(conv + 2));
            loop {
                let e = proximo(read_usize(this));
                if e == 0 {
                    break;
                }
                if read_usize(e) != read_usize(read_usize(jogador + 0x24)) {
                    continue;
                }
                if adversario != 0 && read_usize(e + 4) != read_usize(adversario) {
                    continue;
                }
                let (dia, ano) = (read_u16(e + 8) as i16, read_u16(e + 0xa));
                let hoje = read_u16(data + 2);
                if ano < hoje || (ano == hoje && dia < read_u16(data) as i16) {
                    if comentario != 0 {
                        texto(comentario, 0xa62fc8, 0xad9c64usize);
                    }
                    crate::comparacao::ramo("cpz_6 no time da selecao");
                    return sai(read_usize(jogador + 0x24)); // cpz6: já no time da seleção
                }
            }
        }
    }
    let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS).wrapping_mul(4)));
    if comp == 0 {
        return sai(0);
    }
    let reg = read_usize(comp + 0xa3);
    let ano = (read_u8(reg + 5) as i8 as i16).wrapping_add(read_u16(comp + 0x40) as i16) as u16;
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mut jogo = [0u8; 8];
    let pj = jogo.as_mut_ptr() as usize;
    data_nova(pj, read_u16(reg + 3) as u32, ano as u32);
    let cj = read_usize(jogador + 0x39);
    if cj == 0 {
        return sai(0);
    }
    let liga = read_usize(cj + 0x57);
    if liga == 0 || read_usize(liga) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return sai(0);
    }
    let (jd, ja) = (read_u16(pj) as i16, read_u16(pj + 2));
    let (ld, la) = (read_u16(plo) as i16, read_u16(plo + 2));
    if la > ja || (la == ja && ld > jd) {
        return sai(0);
    }
    let (hd, ha) = (read_u16(phi) as i16, read_u16(phi + 2));
    if ha < ja || (ha == ja && hd < jd) {
        return sai(0);
    }
    for i in 0..0x32usize {
        for g in [0x9d00a8usize, 0x9d00a4] {
            if read_usize(cpz6_estrelas(g) + 0xd7 + 4 * i) == jogador {
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
pub(crate) unsafe extern "thiscall" fn comparar_selecao_que_tem_o_jogador(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
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

/// the player may play for the manager's national side: nationality, under-21 age, current call-up and the
/// second-nationality rule
/// Original: `national_teams_player_is_eligible` (0x76d9b0, national_teams.cpp, stdcall, ret 8).
///
/// [pt] { 0x76d9b0 } (stdcall, ret 8, national_teams.cpp): `national_teams_player_is_eligible(jogador, tecnico)`, da busca de
/// jogadores (0x7afa70, 0x807450) e da tela de comparação. Sem ficha de jogador (+0x61), sem a seleção do técnico (+0x24)
/// ou ela não sendo seleção (0x53b900): 0. Já convocado para ela: 1. Seleção do segundo conjunto (0x5409b0, a sub-21)
/// e idade (+0x18) acima de 21: 0. Com +0x22 ou o bit 2 de +0xc do registro de 79 bytes da pessoa ([0xae2d0c]), a
/// nacionalidade (+0x1a) tem de ser a da seleção (+0x53); convocado para outra seleção de outra nação: 0. A nação da
/// seleção tem de ser a primeira ou a segunda (+0x1e) nacionalidade; sendo a nação [0x9cf434] e a segunda também: 0.
/// Senão 1.
pub(crate) unsafe extern "stdcall" fn jogador_serve_a_selecao(jogador: usize, tecnico: usize) -> u32 {
    let sel = read_usize(tecnico + 0x24);
    if read_usize(jogador + 0x61) == 0 || sel == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(sel) == 0 {
        return 0;
    }
    if read_usize(jogador + 0x24) == sel {
        return 1;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if segunda(sel) != 0 && read_u8(jogador + 0x18) as i8 > 0x15 {
        return 0;
    }
    let nacao = read_usize(sel + 0x53);
    let primeira = read_usize(jogador + 0x1a);
    if read_u8(jogador + 0x22) != 0 && primeira != nacao {
        return 0;
    }
    let p = read_usize(0xae2d0c).wrapping_add(read_usize(jogador).wrapping_mul(79));
    if read_u8(p + 0xc) & 2 != 0 && read_usize(jogador + 0x1a) != nacao {
        return 0;
    }
    let outra = read_usize(jogador + 0x24);
    if outra != 0 && read_usize(outra + 0x53) != nacao {
        return 0;
    }
    let primeira = read_usize(jogador + 0x1a);
    if primeira != nacao && read_usize(jogador + 0x1e) != nacao {
        return 0;
    }
    let x = read_usize(KEY_NATION_PORTUGAL);
    if read_usize(nacao) == x {
        let s = read_usize(jogador + 0x1e);
        if s != 0 && read_usize(s) == x {
            return 0;
        }
    }
    1
}

/// Comparador da 0x76d9b0: pela devolução (só lê).
pub(crate) unsafe extern "stdcall" fn comparar_jogador_serve_a_selecao(jogador: usize, tecnico: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u32>(1396)(jogador, tecnico);
    let n = jogador_serve_a_selecao(jogador, tecnico);
    PLACAR.registrar(r == n, || format!("jogador_serve_a_selecao(jogador {:x}, tecnico {:x}) original={} rust={}", jogador, tecnico, r, n));
    r
}

/// adds to a list each duty competition it does not yet hold and returns how many
/// Original: `NationalTeams::collect_duty_competitions` (0x775580, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x775580 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::collect_duty_competitions(saida)`, dos olheiros
/// (0x7f7010). Sem a lista de convocações [this] ou sem `saida`: 0. Para cada convocação, a competição dela (+4) entra
/// em `saida` (0x68d0d0, 4 bytes) se ainda não estiver lá; devolve quantas entraram (byte).
pub(crate) unsafe extern "thiscall" fn juntar_competicoes_das_convocacoes(this: usize, saida: usize) -> u8 {
    let conv = read_usize(this);
    if conv == 0 || saida == 0 {
        return 0;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let mut n: u8 = 0;
    core::ptr::write_unaligned((conv + 0xa) as *mut usize, read_usize(conv + 2));
    let mut e = proximo(read_usize(this));
    while e != 0 {
        core::ptr::write_unaligned((saida + 0xa) as *mut usize, read_usize(saida + 2));
        let mut ja = false;
        loop {
            let x = proximo(saida);
            if x == 0 {
                break;
            }
            if read_usize(x) == read_usize(e + 4) {
                ja = true;
                break;
            }
        }
        if !ja {
            acrescentar(saida, e + 4, 4);
            n = n.wrapping_add(1);
        }
        e = proximo(read_usize(this));
    }
    n
}

/// Os valores dos nós de uma lista (cabeça em +2, nó: +0 o item, +6 o próximo), sem mexer no cursor (+0xa).
pub(crate) unsafe fn cpz6_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = read_usize(lista + 2);
    while no != 0 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

// ponytail: a 0x775580 acrescenta nós (aloca): só a original roda. O plano, feito antes pelos nós (sem mexer nos
// cursores): as competições das convocações que ainda não estão em `saida`, na ordem, sem repetir; depois, a devolução
// é quantas e os últimos itens de `saida` (4 bytes cada) são elas.
pub(crate) unsafe extern "thiscall" fn comparar_juntar_competicoes_das_convocacoes(this: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1397);
    let conv = read_usize(this);
    if conv == 0 || saida == 0 {
        let r = o(this, saida);
        PLACAR.registrar(r == 0, || format!("juntar_competicoes_das_convocacoes(saida {:x}) sem lista original={}", saida, r));
        return r;
    }
    let mut tem: Vec<usize> = cpz6_itens(saida).iter().map(|&x| read_usize(x)).collect();
    let antes = tem.len();
    for e in cpz6_itens(conv) {
        let c = read_usize(e + 4);
        if !tem.contains(&c) {
            tem.push(c);
        }
    }
    let r = o(this, saida);
    let depois: Vec<usize> = cpz6_itens(saida).iter().map(|&x| read_usize(x)).collect();
    let igual = r == (tem.len() - antes) as u8 && depois == tem;
    PLACAR.registrar(igual, || format!("juntar_competicoes_das_convocacoes(saida {:x}) original={} plano={:x?} depois={:x?}", saida, r, tem, depois));
    r
}

// ---------- 261a leva: as convocações para o jogo de seleções e duas notícias: escolher os elencos dos dois lados, a notícia do convocado e a do desfecho na copa (national_teams.cpp, Cup.cpp) ----------




/// O ensaio do comparador da 0x76bab0: a cópia roda sem escolher elenco (0x76c120), sem dispensar os acima da idade do
/// sub-21 (0x76d880: no ensaio, só o byte +0x6b que ela zera depois da data) e sem apagar repetidos (0x76c060).
pub(crate) static mut CPX13_ENSAIO: bool = false;
/// No ensaio: a cópia pediu um elenco (a comparação não vale).
pub(crate) static mut CPX13_PEDIU: bool = false;

/// O registro da nação do clube na tabela das seleções ([this+4], 0x6e bytes, pelo id da nação +0x53).
pub(crate) unsafe fn cpx13_registro(this: usize, clube: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(read_usize(clube + 0x53)).wrapping_mul(0x6e))
}

/// Um elenco do registro `reg` (tipo 0 principal, 1 B/olímpico, 2 sub-21: a marca de hoje em +0x32/+0x46/+0x5a, a
/// data do elenco em +0x3a/+0x4e/+0x62, a marca de convocado em +0x43/+0x57/+0x6b): com o elenco de até 14 dias (0x54cdb0
/// sobre hoje) e a marca (e `extra`), grava hoje (0x417c10) e leva a data do elenco até a data do jogo (+0x2a/+0x28)
/// se ela for anterior; senão pede o elenco (0x76c120 com a nação, o tipo, hoje, a partida e 0).
pub(crate) unsafe fn cpx13_elenco(this: usize, p: usize, clube: usize, reg: usize, tipo: u32, extra: impl FnOnce() -> bool) {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let base = reg + [0x32usize, 0x46, 0x5a][tipo as usize];
    let data = base + 8;
    let d = dias(0xae2c90, read_usize(data) as u32, read_usize(data + 4) as u32);
    if d <= 0xe && read_u8(base + 0x11) != 0 && extra() {
        let mut r = [0u8; 8];
        copia(base, r.as_mut_ptr(), 0xae2c90);
        let mut jogo = [0u8; 8];
        data_nova(jogo.as_mut_ptr(), read_i16(p + 0x2a) as u16 as u32, read_i16(p + 0x28) as u16 as u32);
        let (ra, rd) = (read_i16(data + 2) as u16, read_i16(data) as i16);
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
    let escolher: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32) = core::mem::transmute(FN_NATIONALTEAMS_SELECT_SQUAD);
    crate::comparacao::ramo("cpx_13 elenco pedido");
    escolher(this, read_usize(clube + 0x53), tipo, 0xae2c90, p, 0); // cpx13: o elenco
}

/// Os elencos B/olímpico (+0x4e da partida, bit 0x100) ou sub-21 (bit 0x200, ou o clube é seleção B 0x5409b0) do lado
/// `clube`; o sub-21 só sem convocado: competição [0x9cf7a0] ou a nação fora do vetor [this+0x36].
pub(crate) unsafe fn cpx13_elencos_menores(this: usize, p: usize, clube: usize, reg: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let m = read_i16(p + 0x4d) as u16;
    if m & 0x100 != 0 {
        cpx13_elenco(this, p, clube, reg, 1, || true);
    } else if m & 0x200 != 0 || segunda(clube) != 0 {
        cpx13_elenco(this, p, clube, reg, 2, || {
            read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) || read_u8(read_usize(read_usize(clube + 0x53)).wrapping_add(read_usize(this + 0x36))) == 0
        });
    }
}

/// O que a 0x76d880 grava no registro (fora das pessoas): depois da data do sub-21 (+0x62/+0x66, 0x456740 sobre
/// hoje), zera a marca +0x6b.
pub(crate) unsafe fn cpx13_dispensar(this: usize, clube: usize) {
    if !CPX13_ENSAIO {
        let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_RELEASE_OVERAGE_U21_PLAYERS);
        return dispensar(this, read_usize(clube + 0x53));
    }
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let reg = cpx13_registro(this, clube);
    if depois(0xae2c90, read_usize(reg + 0x62) as u32, read_usize(reg + 0x66) as u32) != 0 {
        *((reg + 0x6b) as *mut u8) = 0;
    }
}

/// from two weeks before the match, for each side: a senior, B/Olympic or under-21 squad at most 14 days old and
/// flagged only moves its date up to the match; otherwise asks for a new one (0x76c120); releases the over-age under-21
/// players and clears duplicates
/// Original: `NationalTeams::select_squads_for_fixture` (0x76bab0, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x76bab0 } (thiscall, ret 4, national_teams.cpp, quadro SEH sem destrutor): `NationalTeams::select_squads_for_fixture
/// (partida)`: sem a marca +0x40 com o bit 1 de +0x4e, ou mandante seleção B (0x5409b0) com o bit 2, nada. Só a
/// partir de duas semanas antes do jogo (0x54cab0 sobre a data +0x2a/+0x28). Mandante: fora de seleção B, o elenco
/// principal (cpx13_elenco tipo 0) e, sem o bit 2, a dispensa do sub-21 (0x76d880) das duas nações; os elencos
/// menores (cpx13_elencos_menores). Visitante: o mesmo sem a dispensa. Por fim apaga os repetidos (0x76c060) das duas.
pub(crate) unsafe extern "thiscall" fn convocar_selecoes_para_o_jogo(this: usize, p: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let semanas_antes: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(FN_NATIONAL_TEAMS_REMOVE_DUPLICATE_SQUAD_ENTRIES);
    if read_u8(p + 0x40) == 0 && read_u8(p + 0x4e) & 1 != 0 {
        return;
    }
    if segunda(read_usize(p + 0x1c)) != 0 && read_u8(p + 0x4e) & 2 != 0 {
        return;
    }
    let mut jogo = [0u8; 8];
    data_nova(jogo.as_mut_ptr(), read_i16(p + 0x2a) as u16 as u32, read_i16(p + 0x28) as u16 as u32);
    let (hd, ha) = (read_i16(0xae2c90), read_i16(0xae2c92) as u16);
    let mut r = [0u8; 8];
    let q = semanas_antes(jogo.as_mut_ptr(), r.as_mut_ptr(), 2); // cpx13: a janela
    let (qa, qd) = (read_i16(q as usize + 2) as u16, read_i16(q as usize));
    if ha < qa || (ha == qa && hd < qd) {
        return;
    }
    let (casa, fora) = (read_usize(p + 0x1c), read_usize(p + 0x20));
    let reg = cpx13_registro(this, casa);
    if segunda(casa) == 0 {
        cpx13_elenco(this, p, casa, reg, 0, || true);
        if read_u8(p + 0x4e) & 2 == 0 {
            cpx13_dispensar(this, read_usize(p + 0x1c));
            cpx13_dispensar(this, read_usize(p + 0x20));
        }
    }
    cpx13_elencos_menores(this, p, casa, reg);
    let reg = cpx13_registro(this, fora);
    if segunda(fora) == 0 {
        cpx13_elenco(this, p, fora, reg, 0, || true);
    }
    cpx13_elencos_menores(this, p, fora, reg);
    if !CPX13_ENSAIO {
        repetidos(read_usize(read_usize(p + 0x1c) + 0x53));
        repetidos(read_usize(read_usize(p + 0x20) + 0x53));
    }
}

/// Comparador: a original roda antes; a cópia depois, em ensaio, a partir dos mesmos registros das duas nações (+0x32
/// a +0x6c: marcas de hoje, datas dos elencos e marcas de convocado). Se a cópia pediu algum elenco (0x76c120 aloca e
/// mexe nos elencos), a comparação não vale e fica a original; senão os registros têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_convocar_selecoes_para_o_jogo(this: usize, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1341);
    let regioes = [(cpx13_registro(this, read_usize(p + 0x1c)) + 0x32, 0x3a), (cpx13_registro(this, read_usize(p + 0x20)) + 0x32, 0x3a)];
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
pub(crate) unsafe fn cpx13_clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    read_usize(0xae23b4).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
}

/// O ensaio do comparador da 0x76fd10: a cópia roda sem entregar a notícia.
pub(crate) static mut CPX13_ENSAIO_NOTICIA: bool = false;

/// with a human manager at the player's club, news item 0x21 (the player, the national side and the slot) to the
/// manager of club +0x39 and of the contract club
/// Original: `national_teams_post_player_called_up_news` (0x76fd10, national_teams.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x76fd10 } (stdcall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor):
/// `national_teams_post_player_called_up_news(jogador, nação, vaga)`: só com técnico humano (0x7875e0) no clube +0x39
/// do jogador ou no clube do contrato (que precisa existir). A notícia 0x21 (local): 0 o id, 1..3 os +0x33 de +4, +8,
/// +0xc; vaga 0: 4 a seleção principal (0x53b3d0), 5 = 0; vaga 1: a principal e 1; outra: a B (0x53b4f0) e 0.
/// Entregue ao técnico (0x7861e0) do +0x39, se houver, e ao do clube do contrato quando é outro.
pub(crate) unsafe extern "stdcall" fn noticia_do_convocado(jogador: usize, nacao: usize, vaga: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let c39 = read_usize(jogador + 0x39);
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
    gravar(n, 0, read_usize(jogador) as u32);
    gravar(n, 1, read_usize(read_usize(jogador + 4) + 0x33) as u32);
    gravar(n, 2, read_usize(read_usize(jogador + 8) + 0x33) as u32);
    gravar(n, 3, read_usize(read_usize(jogador + 0xc) + 0x33) as u32);
    match vaga as u8 {
        0 => {
            gravar(n, 4, read_usize(principal(nacao)) as u32);
            gravar(n, 5, 0);
        }
        1 => {
            gravar(n, 4, read_usize(principal(nacao)) as u32);
            gravar(n, 5, 1);
        }
        _ => {
            gravar(n, 4, read_usize(b(nacao)) as u32);
            gravar(n, 5, 0);
        }
    }
    let noticias = read_usize(0xb7431c);
    let c39 = read_usize(jogador + 0x39);
    if c39 != 0 && !CPX13_ENSAIO_NOTICIA {
        ao_clube(noticias, n, c39);
    }
    if cpx13_clube_do_contrato(jogador) == read_usize(jogador + 0x39) {
        return;
    }
    let c = cpx13_clube_do_contrato(jogador);
    if !CPX13_ENSAIO_NOTICIA {
        ao_clube(noticias, n, c);
    }
}

/// Comparador: a cópia em ensaio (sem entregar) e a original a partir do mesmo [0xae1080] (o último contrato lido, que
/// as duas gravam).
pub(crate) unsafe extern "stdcall" fn comparar_noticia_do_convocado(jogador: usize, nacao: usize, vaga: u32) {
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


// ponytail: a virtual +0x44 da copa acrescenta registros às pessoas e a notícia vai às caixas dos técnicos (alocam): só
// a original; a cópia leva o ramo.

// ---------- 259a leva: o jogo entre seleções: as convocações, os jogos e gols pela seleção, o nome do elenco e as rodadas conhecidas (national_teams.cpp, comp.cpp) ----------


/// O ensaio do comparador da 0x76b600: a cópia roda sem o ranking da FIFA (0x58cf70, acrescenta pontos ao histórico),
/// sem a primeira convocação (0x86cc50, vai para o arquivo), sem o jogo pela seleção no registro de transferências
/// (0x8ce920) e sem a notícia aos clubes (0x7706c0, aloca).
pub(crate) static mut CPW12_ENSAIO: bool = false;

/// A vaga `k` (0..20) da escalação do lado `lado` no resumo do jogo `m`: m + 0x625 × lado + 0x190 + 0x49 × k.
pub(crate) fn cpw12_vaga(m: usize, lado: usize, k: usize) -> usize {
    m.wrapping_add(0x625 * lado + 0x190 + 0x49 * k)
}

/// A pessoa da vaga, se a vaga conta (id ≠ −1 e titular, ou reserva que entrou: +0x32 ≠ 0xff) e a pessoa está na
/// faixa (id < [0xae2364]; o ponteiro nulo passa sem o teste, como na original).
pub(crate) unsafe fn cpw12_pessoa(vaga: usize, k: usize) -> Option<usize> {
    let id = read_usize(vaga) as i32;
    if id == -1 || (k >= 11 && *((vaga + 0x32) as *const u8) == 0xff) {
        return None;
    }
    let p = read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if p != 0 && read_usize(p) as i32 >= read_usize(0xae2364) as i32 {
        return None;
    }
    Some(p)
}

/// O registro geral da pessoa (79 bytes em [0xae2d0c]).
pub(crate) unsafe fn cpw12_geral(p: usize) -> usize {
    read_usize(0xae2d0c).wrapping_add(read_usize(p).wrapping_mul(0x4f))
}

/// O registro da nação do clube nas seleções: [this + 4] + 0x6e × id.
pub(crate) unsafe fn cpw12_nacao(this: usize, clube: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(read_usize(clube + 0x53)).wrapping_mul(0x6e))
}

/// Jogou pela nação do clube: se a segunda nação (+0x1e) é a dele, ela vira a primeira (+0x1a) e a antiga a segunda.
pub(crate) unsafe fn cpw12_trocar_nacao(p: usize, clube: usize) {
    if read_usize(clube + 0x53) == read_usize(p + 0x1e) {
        core::ptr::write_unaligned((p + 0x1e) as *mut usize, read_usize(p + 0x1a));
        core::ptr::write_unaligned((p + 0x1a) as *mut usize, read_usize(clube + 0x53));
    }
}

/// O jogo de nação por lado (o seu código de competição em [p + 0x14]): conta o jogo da nação (+2 do registro dela,
/// curto) e cada pessoa que conta na escalação.
pub(crate) unsafe fn cpw12_lados(this: usize, p: usize, m: usize, mut f: impl FnMut(usize, usize, usize, usize)) {
    for lado in 0..2usize {
        let clube = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
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

/// between two national sides: counts each nation's match; by competition, marks the general record, swaps the
/// nationality or, in real internationals, scores the FIFA ranking and adds caps and goals to everyone who played;
/// clears the type's +0x44 on its date and sends the news to the clubs
/// Original: `NationalTeams::record_international_result` (0x76b600, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x76b600 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::record_international_result(partida, resumo)`:
/// só entre duas seleções (0x53b900). Pela competição da partida ([+0x14]): a de [0x9cf794] marca +0xd = 1 no registro
/// geral de quem jogou; a de [0x9cf798] liga o bit 2 do +0xc e troca a nação (+0x1a/+0x1e); a de [0x9cf7a0] nada; as
/// outras: o ranking da FIFA (0x58cf70 em [this+0x1c]) e, de quem jogou, a primeira convocação (0x86cc50 com o
/// adversário, se +0x22 = 0, depois da troca da nação), um jogo (+0x22) e os gols (+0x23 += +0x21 da vaga) pela
/// seleção e o jogo no registro (0x8ce920 com o número de jogos da nação). Em todas, o jogo da nação (+2) sobe. Depois,
/// por lado: o tipo (1 se +0x40 = 0; senão 2 na segunda seleção 0x5409b0, senão 0); na data do tipo (+0x3a + 20 ×
/// tipo, 0x4a7f20 com a de hoje) zera +0x44 + 20 × tipo; a notícia aos clubes (0x7706c0) com o destaque (vaga +0x153) e
/// a lista de quem jogou. Sem SEH, sem caixa, sem sorteio.
pub(crate) unsafe extern "thiscall" fn resultado_entre_selecoes(this: usize, p: usize, m: usize) {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if e_selecao(read_usize(p + 0x1c)) == 0 || e_selecao(read_usize(p + 0x20)) == 0 {
        return;
    }
    let comp = read_usize(read_usize(p + 0x14));
    if comp == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) {
        cpw12_lados(this, p, m, |_, _, _, pessoa| *((cpw12_geral(pessoa) + 0xd) as *mut u8) = 1);
    } else if comp == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
        cpw12_lados(this, p, m, |clube, _, _, pessoa| {
            *((cpw12_geral(pessoa) + 0xc) as *mut u8) |= 2;
            cpw12_trocar_nacao(pessoa, clube);
        });
    } else if comp != read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
        let fifa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIFA_RANKINGS_RECORD_MATCH_RESULT);
        let primeira: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FIRST_CAP_RECORD);
        let jogo_pela_selecao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_RECORD_INTERNATIONAL_APPEARANCE);
        if !CPW12_ENSAIO {
            fifa(read_usize(this + 0x1c), p);
        }
        for lado in 0..2usize {
            let clube = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
            let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
            core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
            for k in 0..20usize {
                let vaga = cpw12_vaga(m, lado, k);
                let id = read_usize(vaga) as i32;
                if id == -1 {
                    continue;
                }
                if k >= 11 && *((vaga + 0x32) as *const u8) == 0xff { // cpw12: a reserva que não entrou
                    crate::comparacao::ramo("cpw_12 reserva sem entrar");
                    continue;
                }
                let pessoa = read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
                if pessoa != 0 && read_usize(pessoa) as i32 >= read_usize(0xae2364) as i32 {
                    continue;
                }
                if *((pessoa + 0x22) as *const u8) == 0 {
                    cpw12_trocar_nacao(pessoa, clube);
                    let adversario = if clube == read_usize(p + 0x1c) { read_usize(p + 0x20) } else { read_usize(p + 0x1c) };
                    if !CPW12_ENSAIO {
                        primeira(read_usize(0xdd7f00), pessoa, adversario);
                    }
                }
                crate::comparacao::ramo("cpw_12 jogo pela seleção");
                *((pessoa + 0x22) as *mut u8) = (*((pessoa + 0x22) as *const u8)).wrapping_add(1); // cpw12: um jogo
                *((pessoa + 0x23) as *mut u8) = (*((pessoa + 0x23) as *const u8)).wrapping_add(*((vaga + 0x21) as *const u8)); // cpw12: os gols
                if !CPW12_ENSAIO {
                    // a original empurra o edx com o curto dos jogos embaixo e o alto de 55 × id da nação
                    let id_nacao = read_usize(read_usize(clube + 0x53)) as u32;
                    let n = core::ptr::read_unaligned(jogos) as u16 as u32;
                    jogo_pela_selecao(read_usize(0xde1f54), pessoa, (id_nacao.wrapping_mul(0x37) & 0xffff_0000) | n);
                }
            }
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3D_3D);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32) = core::mem::transmute(FN_NATIONALTEAMS_SEND_CLUB_INTERNATIONAL_REPORT_NEWS);
    for lado in 0..2usize {
        let clube = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
        let b = *((p + 0x40) as *const u8) == 0;
        let tipo: usize = if segunda(clube) == 0 { if b { 1 } else { 0 } } else if b { 1 } else { 2 };
        let reg = cpw12_nacao(this, clube) + 20 * tipo;
        if mesma_data(reg + 0x3a, read_usize(0xae2c90) as u32, read_usize(0xae2c94) as u32) != 0 {
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
        let destaque_id = read_usize((m + 0x625 * lado + 0x190) .wrapping_add((0x49 * destaque_k) as usize));
        let destaque = read_usize(0xae23bc).wrapping_add(destaque_id.wrapping_mul(0x6e));
        if !CPW12_ENSAIO {
            // ponytail: a original empurra a conta num dword da local com só o byte de baixo gravado; a 0x7706c0 lê o byte
            noticia(this, clube, p, m, destaque, lista.as_mut_ptr(), q as u32);
        }
    }
}

/// As regiões que a 0x76b600 grava por conta própria: de cada lado, o registro da nação (o +2 e os +0x44 dos tipos) e,
/// de cada vaga com pessoa na faixa, os campos +0x1a..+0x24 dela e o +0xc/+0xd do registro geral.
pub(crate) unsafe fn cpw12_regioes(this: usize, p: usize, m: usize) -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    for lado in 0..2usize {
        let clube = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
        let r = cpw12_nacao(this, clube);
        reg.extend([(r + 2, 2), (r + 0x44, 1), (r + 0x58, 1), (r + 0x6c, 1)]);
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            let id = read_usize(vaga) as i32;
            if id == -1 {
                continue;
            }
            let pessoa = read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            if pessoa != 0 && (read_usize(pessoa) as i32) < read_usize(0xae2364) as i32 {
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
pub(crate) unsafe extern "thiscall" fn comparar_resultado_entre_selecoes(this: usize, p: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1302);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if e_selecao(read_usize(p + 0x1c)) == 0 || e_selecao(read_usize(p + 0x20)) == 0 {
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

/// writes the squad name (Main, B, Under 21 or Under 23), in lower case when asked
/// Original: `NationalTeams::format_squad_label` (0x775610, national_teams.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x775610 } (thiscall, ret 0x14, national_teams.cpp; `this` não é lido): `NationalTeams::format_squad_label(tipo,
/// sub23, saida, tamanho, minusculo)`: tamanho abaixo de 20: a caixa (linha 0x1c98). Escreve (0x66f4e0) o nome do
/// elenco: tipo 0 "Main", 1 "'B'", outros "Under 23" (com sub23) ou "Under 21"; com minúsculo = 1, em minúsculas.
pub(crate) unsafe extern "thiscall" fn nome_do_elenco_da_selecao(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    if tamanho < 0x14 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(1303)(this, tipo, sub23, saida, tamanho, minusculo);
    }
    let texto: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
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
pub(crate) unsafe extern "thiscall" fn comparar_nome_do_elenco_da_selecao(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
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



// ---------- 258a leva: o convocado machucado: tirar da seleção, a notícia ao clube dele e o pedido de substituto ao técnico da seleção (national_teams.cpp) ----------



/// O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id.
pub(crate) unsafe fn cpv11_registro(this: usize, nacao: usize) -> usize {
    read_usize(this + 4).wrapping_add((read_usize(nacao) as i32).wrapping_mul(0x6e) as usize)
}

/// O tipo de convocação do jogador no clube da seleção `clube`: 2 no segundo conjunto de seleções (0x5409b0), senão o
/// bit 3 das marcas de elenco (+0x5e).
pub(crate) unsafe fn cpv11_tipo(clube: usize, p: usize) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if segunda(clube) != 0 {
        2
    } else {
        (read_u8(p + 0x5e) >> 3) & 1 // cpv11: o tipo pelas marcas
    }
}

/// Os dados 0..3 da notícia: o id do jogador e os de +4, +8 e +0xc dele (+0x33 de cada).
pub(crate) unsafe fn cpv11_quem(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    gravar(n, 0, read_usize(p) as u32);
    for k in 0..3usize {
        gravar(n, 1 + k as i8, read_usize(read_usize(p + 4 + 4 * k) + 0x33) as u32);
    }
}

/// O dado 6 da notícia: a gravidade da lesão do jogador (0x62ec00 sobre o tipo do registro 0x62ed70), ou 0.
pub(crate) unsafe fn cpv11_gravidade(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let gravidade: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_INJURY_GET_TYPE_CATEGORY);
    let v = if lesao(read_usize(0xb63d28), p) != 0 {
        let t = read_u8(lesao(read_usize(0xb63d28), p)) as i8;
        gravidade(read_usize(0xb63d28), t) as i8 as i32
    } else {
        0
    };
    gravar(n, 6, v as u32);
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0.
pub(crate) unsafe fn cpv11_clube_do_contrato(p: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        0
    } else {
        read_usize(0xae23b4).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
    }
}

/// with 14 or more injury days, removes the call-up from the national tactics and squad; before the squad date, tells
/// his club, opens the slot and calls up a replacement or asks the human manager
/// Original: `NationalTeams::withdraw_injured_player` (0x775cd0, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x775cd0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::withdraw_injured_player(p, dias)`: com 14 dias
/// de lesão ou mais, o convocado (+0x28 = 0xb) do clube da seleção (+0x24) sai: com a lesão do tipo 0x51 só sai da
/// tática (0x896c60, duas vezes) e do elenco (0x7762f0). Senão, só antes da data do tipo (+0x3a do registro do tipo:
/// [this+4] + 0x6e × id da nação + 20 × tipo; 0x456740 sobre [0xae2c90]) e se a lista [this] não tem o clube com uma
/// data já passada: a notícia ao clube dele (0x76f7d0), a saída, a vaga do tipo (+0x45) sobe 1, o nível (0x76d810) e,
/// se o técnico da seleção é humano e decide (0x600a50, 0x5fd6b0), o pedido de substituto (0x7701f0); senão o
/// substituto (0x775f90) e as convocações repetidas apagadas (0x76c060). Sem clube: a original (caixa e 0).
pub(crate) unsafe extern "thiscall" fn tirar_convocado_machucado(this: usize, p: usize, dias_arg: u32) {
    let dias = dias_arg as u16 as i16;
    if dias < 0xe { // cpv11: o limite de dias
        return;
    }
    let clube = read_usize(p + 0x24);
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1263)(this, p, dias_arg);
    }
    if read_u8(p + 0x28) != 0xb {
        return;
    }
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_REMOVE_PLAYER_FROM_SQUAD);
    let maior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_PLAYER_WITHDRAWN_NEWS);
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_REPLACEMENT_NEEDED_NEWS);
    let substituto: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_CALL_UP_REPLACEMENT);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(FN_NATIONAL_TEAMS_REMOVE_DUPLICATE_SQUAD_ENTRIES);
    let nacao = read_usize(clube + 0x53);
    let rec = cpv11_registro(this, nacao);
    // ponytail: a original empurra o dword da local com o tipo só no byte de baixo (os outros são sobra de pilha); as
    // chamadas leem só o byte
    let tipo = cpv11_tipo(clube, p);
    let inj = lesao(read_usize(0xb63d28), p);
    if inj != 0 && read_u8(inj) == 0x51 {
        crate::comparacao::ramo("cpv_11 doença");
        marcar(read_usize(0xde1d08), clube, p, 1, 1);
        marcar(read_usize(0xde1d08), clube, p, 0, 1);
        tirar(this, nacao, tipo as u32, p);
        return;
    }
    let vaga = rec.wrapping_add((tipo as i8 as i32 * 20) as usize);
    if maior(vaga + 0x3a, read_usize(0xae2c90) as u32, read_usize(0xae2c94) as u32) == 0 {
        return;
    }
    let lista = read_usize(this);
    if lista != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_usize(lista + 2) as u32);
        let mut e = proximo(read_usize(this));
        while e != 0 {
            if read_usize(e) == read_usize(clube) && maior(0xae2c90, read_usize(e + 8) as u32, read_usize(e + 0xc) as u32) != 0 {
                return;
            }
            e = proximo(read_usize(this));
        }
    }
    if dias < 0x1c {
        crate::comparacao::ramo("cpv_11 lesão de 14 a 27 dias");
    }
    noticia(this, p, nacao, tipo as u32);
    marcar(read_usize(0xde1d08), clube, p, 1, 1);
    marcar(read_usize(0xde1d08), clube, p, 0, 1);
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
            if read_usize(clube + 0xcf) != 0 || humano(principal(nacao), 0, 1, 0, 0) == 0 {
                break 'decide false;
            }
        }
        let t = read_usize(principal(nacao) + 0xcf);
        tipo == 0 || t9a06(read_usize(0xb63c98), t, core::ptr::null_mut()) != 0
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
pub(crate) unsafe fn cpv11_plano(this: usize, p: usize, dias: u32) -> u8 {
    if (dias as u16 as i16) < 0xe || read_u8(p + 0x28) != 0xb {
        return 0;
    }
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let maior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let clube = read_usize(p + 0x24);
    let tipo = cpv11_tipo(clube, p);
    let inj = lesao(read_usize(0xb63d28), p);
    if inj != 0 && read_u8(inj) == 0x51 {
        return 1;
    }
    let vaga = cpv11_registro(this, read_usize(clube + 0x53)).wrapping_add((tipo as i8 as i32 * 20) as usize);
    if maior(vaga + 0x3a, read_usize(0xae2c90) as u32, read_usize(0xae2c94) as u32) == 0 {
        return 0;
    }
    let lista = read_usize(this);
    if lista != 0 {
        let mut no = read_usize(lista + 2);
        while no != 0 {
            let e = read_usize(no);
            if e == 0 {
                break;
            }
            if read_usize(e) == read_usize(clube) && maior(0xae2c90, read_usize(e + 8) as u32, read_usize(e + 0xc) as u32) != 0 {
                return 0;
            }
            no = read_usize(no + 6);
        }
    }
    2
}

/// builds news item 0x20 (the player left the national squad injured) and delivers it to his club and his contract club
/// when human-managed
/// Original: `national_teams_post_player_withdrawn_news` (0x76f7d0, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x76f7d0 } (thiscall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_player_withdrawn_news(p, nacao, tipo)`: se o clube do jogador (+0x39) ou o do contrato
/// (0x4cf5a0) é de técnico humano (0x7875e0), monta a notícia 0x20 numa local (0x77bbe0 com 0): dados 0..3 o jogador
/// e os de +4/+8/+0xc, 4 o id da seleção (principal 0x53b3d0 nos tipos 0 e 1, senão 0x53b4f0), 5 = (tipo == 1), 6 a
/// gravidade da lesão; entrega ao clube +0x39 e, se for outro, ao do contrato (0x7861e0).
pub(crate) unsafe extern "thiscall" fn noticia_da_saida_do_convocado(_this: usize, p: usize, nacao: usize, tipo: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    if read_usize(p + 0x39) == 0 || humano(read_usize(p + 0x39)) == 0 {
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
    gravar(n, 4, read_usize(sel) as u32);
    gravar(n, 5, um);
    cpv11_gravidade(n, p);
    let noticias = read_usize(0xb7431c);
    if read_usize(p + 0x39) != 0 {
        entregar(noticias, n, read_usize(p + 0x39));
    }
    if cpv11_clube_do_contrato(p) != read_usize(p + 0x39) {
        entregar(noticias, n, cpv11_clube_do_contrato(p));
    }
}

/// builds news item 0x22 (a replacement is needed) with the team, squad count, level and type, and delivers it to the
/// national side's human manager
/// Original: `national_teams_post_replacement_needed_news` (0x7701f0, national_teams.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7701f0 } (thiscall, ret 0x10, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_replacement_needed_news(p, nacao, tipo, nivel)`: a notícia 0x22 ao técnico da seleção: dados
/// 0..3 o jogador, 4 o id do time (tipo 1: principal com marca 8 e 5 = 1; tipo 0: principal, senão o sub-21 0x53b4f0,
/// marca 4 e 5 = 0), 6 a gravidade da lesão, 7 = 0, 8 os do elenco com a marca (0x53c590), 9 o nível, 0xa o tipo e 0xb
/// o id da nação. Entrega ao time se o técnico dele é humano (0x600a50), senão ao principal se o dele for.
pub(crate) unsafe extern "thiscall" fn noticia_do_pedido_de_substituto(_this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
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
    gravar(n, 4, read_usize(time) as u32);
    gravar(n, 5, um);
    cpv11_gravidade(n, p);
    gravar(n, 7, 0);
    // ponytail: a marca vai num dword da local com só o byte de baixo gravado; a 0x53c590 lê só o byte
    gravar(n, 8, contar(time, marca, 0) as i8 as i32 as u32);
    gravar(n, 9, nivel as u8 as i8 as i32 as u32);
    gravar(n, 0xa, tipo as u8 as i8 as i32 as u32);
    gravar(n, 0xb, read_usize(nacao) as u32);
    let noticias = read_usize(0xb7431c);
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
pub(crate) unsafe extern "thiscall" fn comparar_tirar_convocado_machucado(this: usize, p: usize, dias: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1263);
    let clube = read_usize(p + 0x24);
    if (dias as u16 as i16) < 0xe || clube == 0 {
        return o(this, p, dias);
    }
    let plano = cpv11_plano(this, p, dias);
    let time = cpd2_time(read_usize(clube + 0x53), cpv11_tipo(clube, p));
    let estava = time != 0 && (0..50usize).any(|c| read_usize(time + 0xd7 + 4 * c) == p);
    let antes = (clube, read_u8(p + 0x28));
    o(this, p, dias);
    let depois = (read_usize(p + 0x24), read_u8(p + 0x28));
    let ok = match plano {
        0 => depois == antes,
        _ => !estava || depois.0 == 0,
    };
    PLACAR.registrar(ok, || format!("tirar_convocado_machucado({:x}, {}) plano {} estava {} antes {:x?} depois {:x?}", p, dias as u16 as i16, plano, estava, antes, depois));
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_saida_do_convocado(this: usize, p: usize, nacao: usize, tipo: u32) {
    crate::comparacao::ramo("noticia_da_saida_do_convocado");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1264)(this, p, nacao, tipo)
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_do_pedido_de_substituto(this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    crate::comparacao::ramo("noticia_do_pedido_de_substituto");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1265)(this, p, nacao, tipo, nivel)
}

// ---------- 258a leva: a convocação pode mudar agora?, o jogador está na convocação e o adversário marcado no amistoso (national_teams.cpp, friendly.cpp) ----------



/// O nome do time (0x542100: time, marca de segundo time, destino de 0x47 bytes, 2, 1) e o byte de gramática (+0x52).
pub(crate) unsafe fn cpv12_nome_do_time(time: usize, segundo: u32, buf: &mut [u8; 0x48]) -> i32 {
    let nome: unsafe extern "C" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_GET_NATIONAL_TEAM_NAME);
    nome(time, segundo, buf.as_mut_ptr(), 0x47, 2, 1);
    read_u8(time + 0x52) as i8 as i32
}

/// tells whether the player may join or leave the squad now (retired, other nation, full squad, injured, squad window,
/// date and match day, human manager) and writes the reason
/// Original: `national_teams_can_change_squad` (0x76db00, national_teams.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x76db00 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH com a lista local da busca de jogos):
/// `national_teams_can_change_squad(nacao, tipo, p, entrando, msg, tam)`: o jogador (se houver) aposentado da seleção
/// (0x547d30) ou de clube de outra nação não muda (0). O time do tipo (0 e 1 a principal, o 1 com a marca 8 e o segundo
/// time; senão o sub-21 0x53b4f0, marca 4). Entrando: elenco cheio (0x53c590 contra +0x42 do registro do tipo) ou
/// jogador machucado (0x62c400 = 2) não muda. Pode mudar (1): entrando com vagas abertas (+0x45), o tipo sem janela
/// (+0x44 = 0), a data do tipo (+0x3a) já passada ou, no próprio dia, um jogo do time hoje (0x5acee0 numa lista local);
/// nos tipos 1 e 2 com técnico humano que decide (0x600a50), também quando a lista [this] não tem o time com data
/// passada e ele não deixa (0x5fd6b0). Os motivos vão para `msg` (0x66f4e0). Sem nação, ou `msg` com menos de 0x32
/// bytes: a original (caixa e 1).
pub(crate) unsafe extern "thiscall" fn pode_mudar_a_convocacao(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(1266)(this, nacao, tipo, p, entrando, msg, tam);
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let aposentado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let nome_do_jogador: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    if p != 0 {
        if aposentado(p, principal(read_usize(p + 0x1a))) != 0 { // cpv12: aposentado
            crate::comparacao::ramo("cpv_12 aposentado");
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                texto(msg, 0xa5b4c8, nome.as_ptr());
            }
            return 0;
        }
        let c = read_usize(p + 0x24);
        if c != 0 && read_usize(c + 0x53) != nacao {
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                let outra = read_usize(read_usize(p + 0x24) + 0x53);
                let g = read_u8(outra + 0x52) as i8 as i32;
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
    let mut tecnico = read_usize(principal(nacao) + 0xcf);
    if humano(principal(nacao), 0, 1, 0, 0) == 0 {
        tecnico = 0;
    }
    let rec = read_usize(this + 4).wrapping_add((read_usize(nacao) as i32).wrapping_mul(0x6e) as usize);
    let e = rec.wrapping_add((s as i8 as i32 * 20) as usize);
    if entrando != 0 {
        let teto = read_u8(e + 0x42) as i8;
        if (contar(time, marca, 0) as i8) >= teto {
            if msg != 0 {
                let mut buf = [0u8; 0x48];
                let g = cpv12_nome_do_time(time, segundo, &mut buf);
                texto(msg, 0xa5b3e0, g, g, teto as i32, buf.as_ptr());
            }
            return 0;
        }
        if estado(read_usize(0xb63d28), p) == 2 {
            if msg != 0 {
                texto(msg, 0xa5b3bc);
            }
            return 0;
        }
    }
    if read_u8(e + 0x45) != 0 && entrando != 0 {
        return 1;
    }
    if read_u8(e + 0x44) == 0 {
        crate::comparacao::ramo("cpv_12 sem janela");
        return 1; // cpv12: sem janela
    }
    let hoje = read_usize(0xae2c90) as u32;
    let (ano, dia) = ((hoje >> 16) as u16, hoje as u16 as i16);
    if s != 0 && tecnico != 0 {
        let mut passou = false;
        let lista = read_usize(this);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_usize(lista + 2) as u32);
            let mut it = proximo(read_usize(this));
            while it != 0 {
                if read_usize(it) == read_usize(time) {
                    let d = read_usize(it + 8) as u32;
                    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
                    if a < ano || (a == ano && dd < dia) {
                        passou = true;
                        break;
                    }
                }
                it = proximo(read_usize(this));
            }
        }
        if !passou && t9a06(read_usize(0xb63c98), tecnico, core::ptr::null_mut()) == 0 {
            return 1;
        }
    }
    let d = read_usize(e + 0x3a) as u32;
    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
    if a < ano || (a == ano && dd < dia) {
        crate::comparacao::ramo("cpv_12 data passada");
        return 1; // cpv12: data passada
    }
    if a == ano && dd as u16 == dia as u16 {
        crate::comparacao::ramo("cpv_12 jogo hoje");
        let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let buscar: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, u32, u32, u32, u32, usize, u32, i32, u32, u32) -> u16 =
            core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        // as vagas da data que a busca lê (a palavra do ano, o dword do dia, o byte da fatia), como a original monta
        let mut v_ano: u32 = ano as u32;
        let mut v_dia: u32 = hoje;
        let mut fatia: u8 = 0;
        let mut l = [0u32; 4];
        let lp = l.as_mut_ptr() as usize;
        nova(lp);
        let achou = buscar(read_usize(0xae2a58), &mut v_ano, &mut v_dia, &mut fatia, 1, -1, -1, -1, read_usize(time) as u32, -1, 0, 0, 0, 1, lp, 1, -2,
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

/// is the player in the squad of the type (senior, second or under-21) with the type flag?
/// Original: `national_teams_player_in_squad` (0x76da80, national_teams.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x76da80 } (stdcall, ret 0xc, national_teams.cpp): `national_teams_player_in_squad(p, nacao, tipo)`: 1 se o
/// clube da seleção do jogador (+0x24) é o time do tipo (0 e 1 a principal 0x53b3d0, 2 o sub-21 0x53b4f0) e ele tem a
/// marca do tipo (+0x5e: 4 nos tipos 0 e 2, 8 no 1). Sem clube ou outro tipo: 0.
pub(crate) unsafe extern "stdcall" fn esta_na_convocacao(p: usize, nacao: usize, tipo: u32) -> u32 {
    if read_usize(p + 0x24) == 0 {
        return 0;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let (time, marca) = match tipo as u8 {
        0 => (principal(nacao), 4u8),
        1 => (principal(nacao), 8),
        2 => (selecao_b(nacao), 4),
        _ => return 0,
    };
    (read_usize(p + 0x24) == time && read_u8(p + 0x5e) & marca != 0) as u32
}


/// Comparador: lado a lado pela devolução e pelo motivo (a original no destino, a cópia num destino próprio do mesmo
/// tamanho). As duas andam o cursor da lista [this] do mesmo jeito; a busca de jogos do dia monta e libera uma lista
/// local (sem estado que fique). Sem nação ou destino curto (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_pode_mudar_a_convocacao(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
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
pub(crate) unsafe extern "stdcall" fn comparar_esta_na_convocacao(p: usize, nacao: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(1267)(p, nacao, tipo);
    let n = esta_na_convocacao(p, nacao, tipo);
    PLACAR.registrar(r == n, || format!("esta_na_convocacao({:x}, {:x}, {}) original={} rust={}", p, nacao, tipo as u8, r, n));
    r
}


// ---------- 252a leva: as seleções e o calendário dos clubes: se o jogo do clube pode ser marcado com os convocados, a ordem das convocações pela data e a nação classificada para a fase final (national_teams.cpp) ----------


pub(crate) unsafe fn cpp12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Dia (palavra com sinal) e ano (palavra sem sinal) de uma data de 8 bytes.
pub(crate) unsafe fn cpp12_data(p: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p + 2) as *const u16))
}

/// Os convocados do clube (id < [0xae235c] − 2 × [0xae2350]: clube comum, não seleção) entre as datas `a` e `b`: a
/// lista do clube em [this + 0x20] + 6 × id (+0 os registros de 9 bytes em ordem de data, +5 a conta em byte com sinal);
/// pula os antes de `a`, para no primeiro depois de `b` e soma os bytes +8 (em byte, com a volta). Mais de 2: demais.
pub(crate) unsafe fn cpp12_muitos_convocados(this: usize, clube: usize, a: usize, b: usize) -> bool {
    if clube == 0 {
        return false;
    }
    let limite = cpp12_i(0xae235c).wrapping_sub(cpp12_i(0xae2350).wrapping_mul(2));
    let id = cpp12_i(clube);
    if id >= limite {
        return false;
    }
    let rec = read_usize(this + 0x20).wrapping_add((id as usize).wrapping_mul(6));
    let n = *((rec + 5) as *const i8);
    let lista = read_usize(rec);
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

/// friendly fixture: 1; otherwise 0 when the home or away club (a plain club) has more than 2 players on call-up from a
/// day before to a day after the fixture date, else 1; in DESLIGADAS since 2026-10-30: the detour alone (without
/// running the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of
/// batch 252): the detour changes the stack residue a later original reads; the copy stays in the code and does not run
/// Original: `NationalTeams::can_schedule_club_fixture` (0x776fc0, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x776fc0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::can_schedule_club_fixture(jogo)`: com a data
/// do jogo (0x549ef0 com +0x2a e +0x28), a janela de um dia antes (0x54c8e0) e um depois (0x54c5b0); jogo da competição
/// amistosa (0x5c6710 em [0xae2c74] com +0x14): 1. Senão 0 quando o mandante (+0x1c) ou o visitante (+0x20) tem mais
/// de 2 convocados na janela; senão 1. O quadro SEH só guarda o estado (as datas não têm destrutor).
pub(crate) unsafe extern "thiscall" fn pode_marcar_jogo_do_clube(this: usize, jogo: usize) -> u32 {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(FN_DATE_DATE);
    let menos_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let (mut d, mut a, mut b) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // ponytail: a original empurra as palavras +0x2a e +0x28 com o alto do registro (lixo); a 0x549ef0 só lê as
    // palavras baixas: a cópia passa as palavras limpas.
    data_nova(d.as_mut_ptr(), *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    menos_dias(d.as_mut_ptr(), a.as_mut_ptr(), 1);
    mais_dias(d.as_mut_ptr(), b.as_mut_ptr(), 1);
    if amistosa(read_usize(0xae2c74), read_usize(jogo + 0x14)) != 0 {
        return 1;
    }
    let (pa, pb) = (a.as_ptr() as usize, b.as_ptr() as usize);
    if cpp12_muitos_convocados(this, read_usize(jogo + 0x1c), pa, pb) {
        return 0;
    }
    if cpp12_muitos_convocados(this, read_usize(jogo + 0x20), pa, pb) {
        crate::comparacao::ramo("cpp_12 visitante convocado");
        return 0; // o visitante com convocados demais
    }
    1
}

/// qsort comparator of records by the date at +0 (0x456770/0x456740), earliest first
/// Original: `compare_call_up_records_by_date` (0x778e90, national_teams.cpp, cdecl).
///
/// [pt] { 0x778e90 } (cdecl, national_teams.cpp): `compare_call_up_records_by_date(a, b)`: o qsort dos registros com a data
/// em +0 (as convocações de 9 bytes, as lesões, as transferências): −1 se a de `a` é antes (0x456770), 1 se é depois
/// (0x456740), senão 0. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn ordem_das_convocacoes_pela_data(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let (d0, d1) = (read_usize(b) as u32, read_usize(b + 4) as u32);
    let r = if antes(a, d0, d1) != 0 { -1 } else { (depois(a, d0, d1) != 0) as i32 };
    r // a convocação mais cedo primeiro
}

/// with nation and finals both non-null, appends the pair to the new qualifiers list (+0x28)
/// Original: `NationalTeams::add_qualified_nation` (0x7779b0, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x7779b0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::add_qualified_nation(nacao, fase_final)`: com
/// as duas não nulas, acrescenta o registro de 8 bytes (nação, fase final) à lista das novas classificadas em
/// this + 0x28 (LLIST, 0x68d0d0).
pub(crate) unsafe extern "thiscall" fn acrescentar_nacao_classificada(this: usize, nacao: usize, fase_final: usize) {
    if nacao == 0 || fase_final == 0 {
        return;
    }
    let mut reg = [0u8; 8];
    reg[..4].copy_from_slice(&(nacao as u32).to_le_bytes());
    reg[4..].copy_from_slice(&(fase_final as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    acrescentar(this + 0x28, reg.as_mut_ptr() as usize, 8);
}

/// Comparador: pelo valor (só lê; as datas são locais).
pub(crate) unsafe extern "thiscall" fn comparar_pode_marcar_jogo_do_clube(this: usize, jogo: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1039)(this, jogo);
    let n = pode_marcar_jogo_do_clube(this, jogo);
    PLACAR.registrar(r == n, || format!("pode_marcar_jogo_do_clube({:x}) original={} rust={}", jogo, r, n));
    r
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_ordem_das_convocacoes_pela_data(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1040)(a, b);
    let n = ordem_das_convocacoes_pela_data(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_convocacoes_pela_data({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: a original aloca o nó (não roda duas vezes): só ela, e confere o plano: a conta da lista (+0x28, palavra)
/// sobe 1 com as duas não nulas e fica igual senão.
pub(crate) unsafe extern "thiscall" fn comparar_acrescentar_nacao_classificada(this: usize, nacao: usize, fase_final: usize) {
    let antes = *((this + 0x28) as *const i16) as i32;
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1041)(this, nacao, fase_final);
    let depois = *((this + 0x28) as *const i16) as i32;
    let esperado = antes + (nacao != 0 && fase_final != 0) as i32;
    PLACAR.registrar(depois == esperado, || format!("acrescentar_nacao_classificada({:x}, {:x}) conta {} -> {}", nacao, fase_final, antes, depois));
}

// ---------- 251a leva: as seleções: a competição atual da nação e os clubes com jogo na data; a disciplina: tirar o jogador regenerado (national_teams.cpp, discipline.cpp) ----------


pub(crate) unsafe fn cpo13_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A competição (id) é uma das fases finais grandes ([0x9cf79c], [0x9cf7a0], [0x9cf7a4], [0x9cf7a8], [0x9cf7ac],
/// [0x9cf790], [0x9cf8a0], [0x9cf78c], [0x9cf964])?
pub(crate) unsafe fn cpo13_final(id: u32) -> bool {
    [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
        .iter()
        .any(|&g| read_u32_as_usize(g) as u32 == id)
}

/// O que a 0x775220 grava: a competição atual da nação ([this+4] + id × 110 + 4), a da fila ([this+8] + id × 4) e se
/// avisa o gerenciador dos técnicos.
pub(crate) struct Cpo13Atual {
    pub(crate) vaga: usize,
    pub(crate) nova: Option<u32>,
    pub(crate) fila: usize,
    pub(crate) na_fila: Option<u32>,
    pub(crate) avisa: bool,
}

/// O plano da 0x775220 (só lê): com competição atual (≠ −1) que é fase final grande, a nova vai para a fila e a atual
/// fica (sem aviso); senão, se a nova é fase final grande e há atual, a atual vai para a fila; a nova vira a atual e o
/// gerenciador dos técnicos ([0xb67a44], se existe) é avisado.
pub(crate) unsafe fn cpo13_plano_atual(this: usize, nacao: usize, comp: usize) -> Cpo13Atual {
    let id = read_u32_as_usize(nacao);
    let vaga = read_u32_as_usize(this + 4).wrapping_add(id.wrapping_mul(110)) + 4;
    let fila = read_u32_as_usize(this + 8).wrapping_add(id.wrapping_mul(4));
    let atual = read_u32_as_usize(vaga) as u32;
    let nova = read_u32_as_usize(comp) as u32;
    let mut p = Cpo13Atual { vaga, nova: Some(nova), fila, na_fila: None, avisa: read_u32_as_usize(0xb67a44) != 0 };
    if atual != 0xffff_ffff {
        let r = read_u32_as_usize(0xae23d0).wrapping_add((atual as usize).wrapping_mul(107)); // registros de 107 bytes
        if r != 0 && cpo13_final(read_u32_as_usize(r) as u32) {
            return Cpo13Atual { vaga, nova: None, fila, na_fila: Some(nova), avisa: false };
        }
        if cpo13_final(nova) {
            p.na_fila = Some(atual);
        }
    }
    p
}

/// makes the competition the nation's current one; with a major finals under way, the new one goes to the queue; when
/// the new one is a major finals, the current one goes to the queue; notifies the manager handler (0x6a3770)
/// Original: `NationalTeams::set_current_competition` (0x775220, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x775220 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::set_current_competition(nacao, comp)`: a
/// competição atual da nação vira `comp` (plano em `cpo13_plano_atual`: a fase final grande em curso fica na frente e a
/// outra vai para a fila) e o gerenciador dos técnicos é avisado (0x6a3770 com o clube da seleção principal, 0x53b3d0,
/// e 1). Nação ou competição nula: a caixa "Error" (e volta).
pub(crate) unsafe extern "thiscall" fn definir_competicao_da_selecao(this: usize, nacao: usize, comp: usize) {
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
    let g = read_u32_as_usize(0xb67a44);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
        avisar(g, principal(nacao), 1);
    }
}

/// Comparador: a 0x6a3770 (o aviso) não pode rodar duas vezes: o plano antes, a original roda; confere a competição
/// atual e a da fila. Nação ou competição nula: só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_definir_competicao_da_selecao(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1003);
    if nacao == 0 || comp == 0 {
        return o(this, nacao, comp);
    }
    let p = cpo13_plano_atual(this, nacao, comp);
    let (atual, fila) = (read_u32_as_usize(p.vaga) as u32, read_u32_as_usize(p.fila) as u32);
    o(this, nacao, comp);
    let (atual2, fila2) = (read_u32_as_usize(p.vaga) as u32, read_u32_as_usize(p.fila) as u32);
    let ok = atual2 == p.nova.unwrap_or(atual) && fila2 == p.na_fila.unwrap_or(fila);
    PLACAR.registrar(ok, || format!("definir_competicao_da_selecao(nação {}, comp {}) atual {:#x} -> {:#x} (esperava {:x?}), fila {:#x} -> {:#x} (esperava {:x?})",
        read_u32_as_usize(nacao), read_u32_as_usize(comp), atual, atual2, p.nova, fila, fila2, p.na_fila));
}

/// Marca o clube `c` (se existe e o id < n) com o código na tabela [this+0x24].
pub(crate) unsafe fn cpo13_marcar(this: usize, c: usize, n: i32, codigo: u8) {
    if c != 0 && (read_u32_as_usize(c) as i32) < n {
        *(read_u32_as_usize(this + 0x24).wrapping_add(read_u32_as_usize(c)) as *mut u8) = codigo;
    }
}

/// O clube tem nação marcada (+0x53, +0x11c ≠ 0)?
pub(crate) unsafe fn cpo13_nacao_marcada(c: usize) -> bool {
    let n = read_u32_as_usize(c + 0x53);
    n != 0 && *((n + 0x11c) as *const u8) != 0
}

/// in the one-byte-per-club table, marks the clubs of the non-friendly fixtures on the date: 1 the club's league game,
/// 2 another, 3 a club of a flagged nation (entry with no nation); in DESLIGADAS since 2026-10-29: with the detour the
/// career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news
/// constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the
/// copy) is not isolated yet; the copy stays in the code and does not run
/// Original: `NationalTeams::mark_clubs_with_fixtures` (0x7775c0, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x7775c0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::mark_clubs_with_fixtures(data)`: a tabela de um
/// byte por clube ([this+0x24], n = clubes [0xae235c] − 2 × nações [0xae2350]; calloc na primeira, sem memória a
/// caixa "Error") recebe, para cada jogo não amistoso (0x5c6710) da data nas 3 fatias (0x5ae6a0) de cada uma das 0x21
/// entradas de 0x48 bytes de 0xb63df0 (a nação +0 ativa em +0x11c, ou sem nação; o grupo +0x14): sem nação, 3 para os
/// clubes (+0x1c, +0x20) de nação ativa; com nação, 1 quando a competição do jogo (+0x14) é a da liga do mandante (+0x57)
/// ou a +0x60 do visitante, senão 2, para os dois. Lida pela remarcação dos jogos dos clubes com convocados (0x7777e0).
pub(crate) unsafe extern "thiscall" fn marcar_clubes_com_jogo(this: usize, data: usize) {
    let n = (read_u32_as_usize(0xae235c) as i32).wrapping_sub((read_u32_as_usize(0xae2350) as i32).wrapping_mul(2));
    if read_u32_as_usize(this + 0x24) == 0 {
        let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
        let p = calloc(n, 1);
        cpo13_por(this + 0x24, p as u32);
        if p == 0 {
            aviso_em(0xa5b2a4, 0x2229);
            return;
        }
    }
    let lista_da_data: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    for i in 0..0x21usize {
        let e = 0xb63df0 + i * 0x48;
        let nacao = read_u32_as_usize(e);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        for fatia in 0..3u32 {
            // a original empurra a fatia num dword de pilha com o alto de lixo e o grupo no al: a 0x5abf10 só lê os bytes
            let lista = lista_da_data(data, fatia, *((e + 0x14) as *const u8) as u32);
            if lista == 0 {
                continue;
            }
            cpo13_por(lista + 0xa, read_u32_as_usize(lista + 2) as u32);
            let mut jogo = proximo(lista);
            while jogo != 0 {
                if amistoso(read_u32_as_usize(0xae2c74), read_u32_as_usize(jogo + 0x14)) == 0 {
                    let (casa, fora) = (read_u32_as_usize(jogo + 0x1c), read_u32_as_usize(jogo + 0x20));
                    if read_u32_as_usize(e) == 0 {
                        if casa != 0 && cpo13_nacao_marcada(casa) {
                            cpo13_marcar(this, casa, n, 3);
                        }
                        if fora != 0 && cpo13_nacao_marcada(fora) {
                            cpo13_marcar(this, fora, n, 3);
                        }
                    } else {
                        let comp = read_u32_as_usize(jogo + 0x14);
                        // o visitante +0x60 é lido sem conferir o ponteiro (como a original)
                        let codigo = if casa != 0 && (read_u32_as_usize(casa + 0x57) == comp || read_u32_as_usize(fora + 0x60) == comp) { 1 } else { 2 };
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
pub(crate) unsafe extern "thiscall" fn comparar_marcar_clubes_com_jogo(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1004);
    let n = (read_u32_as_usize(0xae235c) as i32).wrapping_sub((read_u32_as_usize(0xae2350) as i32).wrapping_mul(2));
    if read_u32_as_usize(this + 0x24) == 0 || n <= 0 {
        return o(this, data);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(read_u32_as_usize(this + 0x24), n as usize)], || o(this, data), || marcar_clubes_com_jogo(this, data));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<(usize, u8, u8)> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).map(|i| (i, a[i], b[i])).collect();
        format!("marcar_clubes_com_jogo({:x}) clube, original, rust: {:?}", core::ptr::read_unaligned(data as *const u32), dif)
    });
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
pub(crate) unsafe fn cpo13_orgao(this: usize, regra: u8) -> usize {
    read_u32_as_usize(read_u32_as_usize(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada da pessoa no órgão pela vaga dela (0x7e82e0 sem criar: só lê).
pub(crate) unsafe fn cpo13_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_RULINGBODY_FIND_OR_ADD_DISCIPLINE_ENTRY);
    achar(orgao, id, 0)
}



// ---------- 242a leva: as seleções: o fim das convocações de uma competição e a poda dos registros de convocação (national_teams.cpp) ----------


/// Os itens da lista `l` (0x68d0d0: começo em +2, nó = item +0 e seguinte +6), sem mexer no cursor (+0xa).
pub(crate) unsafe fn cpd1_itens(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = read_usize(l + 2);
    while no != 0 && v.len() < 100_000 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

/// O fim das convocações da lista [this] (as duas rotinas da 0x774f30/0x774f90): do começo (cursor +0xa = +2),
/// cada registro que `passa` recebe em +0x10 a palavra +0 (o dia) de amanhã (0x54c5b0 com 1 sobre [0xae2c90]).
pub(crate) unsafe fn cpd1_encerrar(this: usize, passa: impl Fn(usize) -> bool) {
    let l = read_usize(this);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *const u16 = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mut r = proximo(read_usize(this));
    while r != 0 {
        if passa(r) {
            let mut data = [0u8; 8];
            let dia = *mais_dias(0xae2c90, data.as_mut_ptr(), 1);
            core::ptr::write_unaligned((r + 0x10) as *mut u16, dia); // o dia de amanhã
        }
        r = proximo(read_usize(this));
    }
}

/// in the call-up record list, those of the competition (+4) end tomorrow (+0x10, the day from 0x54c5b0)
/// Original: `end_duty_for_competition` (0x774f30, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x774f30 } (thiscall, ret 4, national_teams.cpp): `end_duty_for_competition(comp)`: na lista de registros
/// de convocação [this], os da competição (+4 igual a *comp) terminam amanhã (+0x10). Sem SEH, sem caixa, sem
/// sorteio; a local é a data que a 0x54c5b0 grava.
pub(crate) unsafe extern "thiscall" fn encerrar_convocacoes_da_competicao(this: usize, comp: usize) {
    cpd1_encerrar(this, |r| read_usize(r + 4) == read_usize(comp) /* a competição */)
}

/// like 0x774f30, only the records of the competition (+4) and the nation (+0)
/// Original: `end_duty_for_nation_in_competition` (0x774f90, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x774f90 } (thiscall, ret 8, national_teams.cpp): `end_duty_for_nation_in_competition(nacao, comp)`: como a
/// 0x774f30, só os registros da competição (+4) e da nação (+0 igual a *nacao).
pub(crate) unsafe extern "thiscall" fn encerrar_convocacoes_da_nacao(this: usize, nacao: usize, comp: usize) {
    cpd1_encerrar(this, |r| read_usize(r + 4) == read_usize(comp) && read_usize(r) == read_usize(nacao) /* a nação */)
}

/// As regiões do fim das convocações: o cursor da lista e o +0x10 de cada registro.
pub(crate) unsafe fn cpd1_regioes_encerrar(this: usize) -> Vec<(usize, usize)> {
    let l = read_usize(this);
    if l == 0 {
        return Vec::new();
    }
    let mut reg = vec![(l + 0xa, 4)];
    reg.extend(cpd1_itens(l).into_iter().filter(|&r| r != 0).map(|r| (r + 0x10, 2)));
    reg
}

pub(crate) unsafe extern "thiscall" fn comparar_encerrar_convocacoes_da_competicao(this: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(788);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, comp), || encerrar_convocacoes_da_competicao(this, comp));
    PLACAR.registrar(igual, || format!("encerrar_convocacoes_da_competicao({:x}, {:x}) {:x?} x {:x?}", this, comp, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_encerrar_convocacoes_da_nacao(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(789);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao, comp), || encerrar_convocacoes_da_nacao(this, nacao, comp));
    PLACAR.registrar(igual, || format!("encerrar_convocacoes_da_nacao({:x}, {:x}, {:x}) {:x?} x {:x?}", this, nacao, comp, mo, mr));
}

/// O número de registros de convocação por clube da 0x7774b0: [0xae235c] − 2 × [0xae2350].
pub(crate) unsafe fn cpd1_quantos() -> i32 {
    (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2))
}

/// in each club record (+0x20, 6 bytes), drops the expired entries (0x456770) at the start, moves the rest down
/// (0x777590) and frees the emptied buffer
/// Original: `prune_club_call_up_records` (0x7774b0, national_teams.cpp, thiscall).
///
/// [pt] { 0x7774b0 } (thiscall, national_teams.cpp): `prune_club_call_up_records()`: cada registro de clube (+0x20,
/// 6 bytes: +0 vetor de entradas de 9 bytes, +4 marca, +5 quantas, byte com sinal) conta as entradas já vencidas
/// (0x456770: a data delas antes de [0xae2c90]; a rotina supõe que ficam no começo), desce as outras por cima
/// (0x777590) e diminui a contagem; zerada com vetor, solta o vetor (0x9452ca) e zera +4 e +0. Sem SEH, sem caixa,
/// sem sorteio; a local (o vencido) é zerada antes de ler.
pub(crate) unsafe extern "thiscall" fn podar_registros_de_convocacao(this: usize) {
    let n = cpd1_quantos();
    let mut rec = read_usize(this + 0x20);
    if n <= 0 {
        return;
    }
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CALL_UP_RECORD_ASSIGN);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let conta = |rec: usize| *((rec + 5) as *const i8);
    for _ in 0..n {
        let mut vencidas: i8 = 0;
        if conta(rec) > 0 {
            let mut k: i8 = 0;
            loop {
                let e = read_usize(rec).wrapping_add(9usize.wrapping_mul(k as i32 as usize));
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
                let v = read_usize(rec);
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
        if resto == 0 && read_usize(rec) != 0 {
            crate::comparacao::ramo("convocações soltar o vetor");
            free(read_usize(rec));
            *((rec + 4) as *mut u8) = 0;
            core::ptr::write_unaligned(rec as *mut usize, 0);
        }
        rec += 6;
    }
}

/// O registro `rec` da 0x7774b0 terminaria sem entradas e com vetor (a original solta o vetor)? Escrito à parte da
/// cópia, para o comparador.
pub(crate) unsafe fn cpd1_soltaria(rec: usize) -> bool {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let n = *((rec + 5) as *const i8);
    let v = read_usize(rec);
    let vencidas = (0..n.max(0) as usize).filter(|&k| antes(v + 9 * k, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0).count();
    v != 0 && n.wrapping_sub(vencidas as i8) == 0
}

/// Comparador: os registros (6 bytes cada) e os vetores de entradas. Algum vetor a soltar (0x9452ca; soltar duas
/// vezes): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_podar_registros_de_convocacao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(790);
    let n = cpd1_quantos().max(0) as usize;
    let recs = read_usize(this + 0x20);
    if n == 0 || recs == 0 || (0..n).any(|i| cpd1_soltaria(recs + 6 * i)) {
        return o(this);
    }
    let mut reg = vec![(recs, 6 * n)];
    for i in 0..n {
        let (v, k) = (read_usize(recs + 6 * i), *((recs + 6 * i + 5) as *const i8));
        if v != 0 && k > 0 {
            reg.push((v, 9 * k as usize));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || podar_registros_de_convocacao(this));
    PLACAR.registrar(igual, || format!("podar_registros_de_convocacao({:x}) {:x?} x {:x?}", this, mo, mr));
}

// ---------- 242a leva: as seleções: pôr e tirar do elenco e dispensar os acima da idade do sub-21 (national_teams.cpp) ----------


/// O time da nação para o tipo de convocação: 0 e 1 a seleção principal (0x53b3d0), os outros o sub-21 (0x53b4f0).
pub(crate) unsafe fn cpd2_time(nacao: usize, tipo: u8) -> usize {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    if tipo == 0 || tipo == 1 { principal(nacao) } else { sub21(nacao) }
}

/// Tira a pessoa `p` da tática do time (0x896c60 de [0xde1d08], com 1 e com 0 no 3º argumento), como as originais
/// fazem antes de esvaziar a vaga.
pub(crate) unsafe fn cpd2_tirar_da_tatica(time: usize, p: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    marcar(*(0xde1d08 as *const usize), time, p, 1, 1);
    marcar(*(0xde1d08 as *const usize), time, p, 0, 1);
}

/// A pessoa sai da seleção: sem time (+0x24), sem a marca +0x28 e só com os bits de clube (& 3) em +0x5e.
pub(crate) unsafe fn cpd2_limpar_pessoa(p: usize) {
    core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    *((p + 0x28) as *mut u8) = 0; // a marca de convocado
    *((p + 0x5e) as *mut u8) &= 3;
}

/// the first of the 50 free squad slots of the team (+0xd7; type 0/1 the senior team, else the under-21) takes the
/// person, who gets the team (+0x24), today's date (+0x29), mark 0xb (+0x28) and squad bit 4 or 8 (0x540980)
/// Original: `add_player_to_squad` (0x776250, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x776250 } (thiscall, ret 0xc, national_teams.cpp): `add_player_to_squad(nacao, tipo, p)`: a primeira vaga
/// livre das 50 do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe `p`; sem vaga, nada. A pessoa ganha o
/// time (+0x24), a data de hoje em +0x29 (0x417c10 de [0xae2c90]), a marca 0xb em +0x28 e o elenco (0x540980) 4
/// (tipo 0 e 2) ou 8. Sem SEH, sem caixa, sem sorteio; a local é a cópia da data que a 0x417c10 devolve.
pub(crate) unsafe extern "thiscall" fn por_no_elenco_da_selecao(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let tipo = tipo as u8;
    let time = cpd2_time(nacao, tipo);
    let elenco: u32 = if tipo == 0 || tipo == 2 { 4 } else { 8 };
    let vaga = match (0..50usize).find(|&c| read_usize(time + 0xd7 + 4 * c) == 0) {
        Some(c) => time + 0xd7 + 4 * c,
        None => return,
    };
    core::ptr::write_unaligned(vaga as *mut usize, p); // a vaga
    core::ptr::write_unaligned((p + 0x24) as *mut usize, time);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut data = [0u8; 8];
    copia(p + 0x29, data.as_mut_ptr(), 0xae2c90);
    *((p + 0x28) as *mut u8) = 0xb;
    let marcar_elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    marcar_elenco(p, elenco);
}

/// As regiões de pôr e tirar do elenco: as 50 vagas do time e a pessoa (+0x24 a +0x5e).
pub(crate) unsafe fn cpd2_regioes(time: usize, p: usize) -> [(usize, usize); 2] {
    [(time + 0xd7, 200), (p + 0x24, 0x3b)]
}

pub(crate) unsafe extern "thiscall" fn comparar_por_no_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(791);
    let time = cpd2_time(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = cpd2_regioes(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || por_no_elenco_da_selecao(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("por_no_elenco_da_selecao({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// removes the person from the team tactics (0x896c60) and, if found in a slot, empties it and clears +0x24, +0x28 and
/// the national bits of +0x5e
/// Original: `remove_player_from_squad` (0x7762f0, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7762f0 } (thiscall, ret 0xc, national_teams.cpp): `remove_player_from_squad(nacao, tipo, p)`: tira `p` da
/// tática do time (0x896c60, duas vezes) e, se ele está numa das 50 vagas (+0xd7), esvazia a vaga e limpa a
/// pessoa (+0x24, +0x28, +0x5e & 3). Sem SEH, sem sorteio, sem locais; caixa só com time ou pessoa nulos (0x896c60).
pub(crate) unsafe extern "thiscall" fn tirar_do_elenco_da_selecao(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let time = cpd2_time(nacao, tipo as u8);
    cpd2_tirar_da_tatica(time, p);
    if let Some(c) = (0..50usize).find(|&c| read_usize(time + 0xd7 + 4 * c) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * c) as *mut usize, 0);
        cpd2_limpar_pessoa(p);
    }
}

/// Comparador: as vagas e a pessoa. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha e não
/// muda nada, então a tática fica fora das regiões. Time ou pessoa nulos (a caixa da 0x896c60): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_tirar_do_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, p: usize) {
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
pub(crate) unsafe fn cpd2_registro(this: usize, nacao: usize) -> usize {
    read_usize(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize)
}

/// once the nation record date (+0x62) has passed, removes from the under-21 squad everyone older than 21 (+0x18) and
/// clears record +0x6b
/// Original: `release_overage_u21_players` (0x76d880, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x76d880 } (thiscall, ret 4, national_teams.cpp): `release_overage_u21_players(nacao)`: depois da data do
/// registro da nação (+0x62/+0x66; 0x456740 sobre [0xae2c90]), cada uma das 50 vagas do sub-21 (0x53b4f0) com
/// pessoa de idade (+0x18, byte com sinal) acima de 21 sai da tática (0x896c60, duas vezes), é limpa (+0x24, +0x28,
/// +0x5e & 3) e a vaga esvazia; por fim zera o byte +0x6b do registro. Sem SEH, sem sorteio; a local é o `this`.
pub(crate) unsafe extern "thiscall" fn dispensar_acima_da_idade_do_sub21(this: usize, nacao: usize) {
    let rec = cpd2_registro(this, nacao);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    if depois(0xae2c90, read_usize(rec + 0x62) as u32, read_usize(rec + 0x66) as u32) == 0 {
        return;
    }
    crate::comparacao::ramo("sub-21 dispensar acima da idade");
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let time = sub21(nacao);
    for c in 0..50usize {
        let vaga = time + 0xd7 + 4 * c;
        let p = read_usize(vaga);
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
pub(crate) unsafe extern "thiscall" fn comparar_dispensar_acima_da_idade_do_sub21(this: usize, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(793);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let time = sub21(nacao);
    if time == 0 {
        return o(this, nacao);
    }
    let mut reg = vec![(cpd2_registro(this, nacao) + 0x6b, 1), (time + 0xd7, 200)];
    reg.extend((0..50usize).map(|c| read_usize(time + 0xd7 + 4 * c)).filter(|&p| p != 0).map(|p| (p + 0x24, 0x3b)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao), || dispensar_acima_da_idade_do_sub21(this, nacao));
    PLACAR.registrar(igual, || format!("dispensar_acima_da_idade_do_sub21({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

// ---------- 242a leva: as seleções: a competição atual de cada nação e a convocação do substituto (national_teams.cpp) ----------



/// Avisa o gerenciador dos técnicos ([0xb67a44], se existe) da seleção principal da nação (0x6a3770 com 1).
pub(crate) unsafe fn cpd3_avisar_tecnicos(nacao: usize) {
    let g = *(0xb67a44 as *const usize);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
        avisar(g, principal(nacao), 1);
    }
}

/// the nation current competition (record +4) becomes −1 and, if there is a next one ([this+8][id]), that one takes its
/// place, notifying the managers (0x6a3770)
/// Original: `clear_current_competition` (0x775000, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x775000 } (thiscall, ret 4, national_teams.cpp): `clear_current_competition(nacao)`: a competição atual do
/// registro da nação ([this+4] + 0x6e × id, +4) vira −1; havendo a seguinte ([this+8][id] ≠ −1), ela passa a ser a
/// atual, a seguinte vira −1 e o gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
pub(crate) unsafe extern "thiscall" fn limpar_competicao_atual(this: usize, nacao: usize) {
    let id = *(nacao as *const i32);
    let atual = read_usize(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4;
    core::ptr::write_unaligned(atual as *mut i32, -1);
    let seguinte = read_usize(this + 8).wrapping_add((id as usize).wrapping_mul(4));
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
pub(crate) unsafe extern "thiscall" fn comparar_limpar_competicao_atual(this: usize, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(794);
    let id = *(nacao as *const i32);
    let seguinte = read_usize(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    if *(seguinte as *const i32) != -1 {
        return o(this, nacao);
    }
    let reg = [(read_usize(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4, 4), (seguinte, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao), || limpar_competicao_atual(this, nacao));
    PLACAR.registrar(igual, || format!("limpar_competicao_atual({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

/// A competição `v` (−1: nenhuma) acabou para a 0x775070? Sem `fim_de_fase`, quando +0x59 da competição
/// ([0xae23d0] + 107 × v) é nulo; com ele, quando é a fase [0xae23a4] + 0xc6 × [0x9cfa10].
pub(crate) unsafe fn cpd3_acabou(v: i32, fim_de_fase: bool) -> bool {
    let c = read_usize((*(0xae23d0 as *const usize)).wrapping_add(v.wrapping_mul(107) as usize) + 0x59);
    if fim_de_fase {
        c == (*(0xae23a4 as *const usize)).wrapping_add((*(KEY_CONTINENT_EUROPE as *const i32)).wrapping_mul(0xc6) as usize)
    } else {
        c == 0
    }
}

/// in the nations of the continent (+0x71), the current and next competitions that ended (competition +0x59 null, or
/// stage [0x9cfa10]) become −1; empty current and a next one: the next moves up, with the notice (0x6a3770)
/// Original: `release_competitions_of_continent` (0x775070, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x775070 } (thiscall, ret 8, national_teams.cpp): `release_competitions_of_continent(continente, fim_de_fase)`:
/// a cada nação ([0xae23a8], 0x122 bytes) do continente (+0x71), a competição atual (registro +4) e a seguinte
/// ([this+8][i]) que acabaram (cpd3_acabou) viram −1; se a atual ficou −1 e há seguinte, a seguinte passa a atual e o
/// gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
pub(crate) unsafe extern "thiscall" fn liberar_competicoes_do_continente(this: usize, continente: usize, fim_de_fase: u32) {
    let fim = fim_de_fase as u8 != 0;
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        let nacao = (*(0xae23a8 as *const usize)).wrapping_add(0x122 * i as usize);
        if read_usize(nacao + 0x71) == continente {
            let atual = read_usize(this + 4) + 0x6e * i as usize + 4;
            let v = *(atual as *const i32);
            if v != -1 && cpd3_acabou(v, fim) {
                *(atual as *mut i32) = -1; // a atual acabou
            }
            let seguinte = read_usize(this + 8) + 4 * i as usize;
            let v = *(seguinte as *const i32);
            if v != -1 && cpd3_acabou(v, fim) {
                *(seguinte as *mut i32) = -1;
            }
            let v = *((read_usize(this + 8) + 4 * i as usize) as *const i32);
            if v != -1 && *((read_usize(this + 4) + 0x6e * i as usize + 4) as *const i32) == -1 {
                crate::comparacao::ramo("seleção continente a seguinte vira a atual");
                *((read_usize(this + 4) + 0x6e * i as usize + 4) as *mut i32) = v;
                *((read_usize(this + 8) + 4 * i as usize) as *mut i32) = -1;
                cpd3_avisar_tecnicos(nacao); // o aviso do continente
            }
        }
        i += 1;
    }
}

/// Comparador: o aviso 0x6a3770 sorteia e mexe nos técnicos (não pode rodar duas vezes): só a original; a prova é a
/// carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (4 chamadas na carreira); comparar pedia separar a decisão do aviso.
pub(crate) unsafe extern "thiscall" fn comparar_liberar_competicoes_do_continente(this: usize, continente: usize, fim_de_fase: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(795)(this, continente, fim_de_fase)
}

/// in the pool of the level, the first eligible player of the nation who plays a position of the one leaving (0x5b7d20)
/// is called up (0x776250, leaving another squad first through 0x7762f0) with the news (0x76fd10); nobody: clears the
/// type slot count (+0x45)
/// Original: `call_up_replacement` (0x775f90, national_teams.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x775f90 } (thiscall, ret 0x10, national_teams.cpp): `call_up_replacement(nacao, tipo, nivel, saindo)`: no
/// trecho do nível (0x76d810) na reserva do tipo ([this+0xc], sub-21 [this+0x10]; começo +8/+0x1d do registro mais
/// as vagas dos níveis anteriores, quantos +0xd/+0x22), o primeiro que não é `saindo`, tem ficha (+0x61), gosta do
/// clube (0x547d30), está bem (0x62c400), não é regen marcado (0x7a7eb0) fora da principal, é da nação (time +0x24,
/// nacionalidades +0x1a/+0x1e, +0x22, bit 2 do registro de 79 bytes), não está suspenso (0x55d270) e joga numa das
/// posições de `saindo` (0x5b7d20). Sem time vai direto; convocado (+0x28 = 0xb) para outro tipo (2 com 0x5409b0,
/// senão o bit 3 de +0x5e) só depois da data do tipo (+0x3a; 0x456770) e, sendo o 0, se o clube deixa (0x600a50):
/// sai de lá (0x7762f0). Entra (0x776250), a vaga do tipo (+0x45) desce 1 e sai a notícia (0x76fd10). Ninguém: zera
/// +0x45. Sem SEH, sem sorteio; a local do tipo do outro elenco é gravada antes de ler (só o byte vale).
pub(crate) unsafe extern "thiscall" fn convocar_substituto(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let regen: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_REMOVE_PLAYER_FROM_SQUAD);
    let por: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_PLAYER_TO_SQUAD);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_PLAYER_CALLED_UP_NEWS);
    let mascara = posicoes(read_usize(saindo + 0x61)) as u8;
    let t = tipo as u8 as i8;
    let nivel = nivel as u8 as i8;
    let rec = read_usize(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize);
    let sub = (t == 2) as usize;
    let o = 0x15 * sub;
    let mut inicio = read_usize(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        inicio = inicio.wrapping_add(read_u8(rec + o + 0xe + 4 * j) as i32);
    }
    let fim = inicio.wrapping_add(read_u8(rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize) + 0xd) as i32);
    let reserva = read_usize(this + 4 * sub + 0xc);
    let vaga = rec.wrapping_add((20 * t as i32) as usize) + 0x45;
    let mut k = inicio;
    while k < fim {
        let q = read_usize(reserva.wrapping_add((4 * k) as usize));
        k += 1;
        if q == saindo || read_usize(q + 0x61) == 0 || nao_gosta(q, principal(read_usize(q + 0x1a))) != 0 {
            continue;
        }
        if estado(*(0xb63d28 as *const usize), q) != 0 {
            continue;
        }
        if *(0xb7434c as *const usize) != 0 && regen(q) != 0 && t != 0 {
            continue;
        }
        let time = read_usize(q + 0x24);
        if time != 0 && read_usize(time + 0x53) != nacao {
            continue;
        }
        if read_usize(q + 0x1a) != nacao {
            let registro = (*(0xae2d0c as *const usize)).wrapping_add((*(q as *const i32)).wrapping_mul(0x4f) as usize);
            if time != 0 || read_u8(q + 0x22) != 0 || read_u8(registro + 0xc) & 2 != 0 || read_usize(q + 0x1e) != nacao {
                continue;
            }
        }
        let comp = (*(0xae23d0 as *const usize)).wrapping_add((*(KEY_CLUB_COMP_FIFA_WORLD_CUP as *const i32)).wrapping_mul(107) as usize);
        if suspenso(*(0xae28f0 as *const usize), q, comp, 1, 0) != 0 {
            continue;
        }
        if posicoes(read_usize(q + 0x61)) as u8 & mascara & 0x7f == 0 { // as posições de quem sai
            continue;
        }
        let time = read_usize(q + 0x24);
        if time != 0 {
            if read_u8(q + 0x28) != 0xb {
                continue;
            }
            let outro: i8 = if segunda(time) != 0 { 2 } else { ((read_u8(q + 0x5e) >> 3) & 1) as i8 };
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
        *(vaga as *mut u8) = read_u8(vaga).wrapping_sub(1); // uma vaga a menos
        noticia(this, q, nacao, tipo);
        return;
    }
    *(vaga as *mut u8) = 0;
}

/// Comparador: a entrada no elenco sai com a notícia (0x76fd10) e a saída do outro mexe na tática: só a original
/// (a 0x776250 e a 0x7762f0 que ela chama passam pelos comparadores delas).
// ponytail: só a original; a prova é a carreira (140 chamadas) e o ramo da cópia no modo Rust.
pub(crate) unsafe extern "thiscall" fn comparar_convocar_substituto(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(796)(this, nacao, tipo, nivel, saindo)
}

// ---------- 242a leva: as seleções: as reservas de convocáveis de cada nação e as duas ordens delas (national_teams.cpp) ----------




/// A nota da pessoa da vaga `q` (ponteiro para a pessoa): o curto +9 da ficha (+0x61).
pub(crate) unsafe fn cpd4_nota(q: usize) -> i16 {
    read_i16(read_usize(read_usize(q) + 0x61) + 9)
}

/// O começo do trecho do nível `nivel` na reserva (`o` 0: a principal, 0x15: o sub-21): o +8 do registro mais as
/// vagas (+0xe) dos níveis anteriores.
pub(crate) unsafe fn cpd4_inicio(rec: usize, o: usize, nivel: i8) -> i32 {
    let mut v = read_usize(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        v = v.wrapping_add(read_u8(rec + o + 0xe + 4 * j) as i32);
    }
    v
}

/// Põe a pessoa `p` na reserva do registro `rec` (`sub` 0: a principal em [this+0xc]; 1: o sub-21 em [this+0x10],
/// campos 0x15 adiante), no trecho do nível dela (0x76d810). Com vaga (+0xd < +0xe), no fim do trecho, e a menor nota
/// (+0xf) desce se for o caso; cheio, só com nota acima da menor: entra na posição pela nota (decrescente, depois das
/// iguais), empurrando o último para fora, e a menor nota vira a do último. Trecho cheio: ordena (qsort 0x945333 com
/// 0x778ed0).
pub(crate) unsafe fn cpd4_reservar(this: usize, p: usize, rec: usize, sub: u32) {
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let o = 0x15 * sub as usize;
    let nivel = nivel_de(p, rec, sub) as i8;
    let c = rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize);
    let reserva = || read_usize(this + 0xc + 4 * sub as usize);
    let vaga = |k: i32| reserva().wrapping_add((4 * k) as usize);
    let nota = read_i16(read_usize(p + 0x61) + 9);
    if read_u8(c + 0xd) < read_u8(c + 0xe) {
        let ini = cpd4_inicio(rec, o, nivel);
        core::ptr::write_unaligned(vaga(ini.wrapping_add(read_u8(c + 0xd) as i32)) as *mut usize, p);
        *((c + 0xd) as *mut u8) = read_u8(c + 0xd).wrapping_add(1);
        if nota < read_i16(c + 0xf) {
            core::ptr::write_unaligned((c + 0xf) as *mut i16, nota); // a menor nota
        }
    } else if nota > read_i16(c + 0xf) {
        crate::comparacao::ramo("reservas cheias entra pela nota");
        let ini = cpd4_inicio(rec, o, nivel);
        let ultimo = (read_u8(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(2);
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
            core::ptr::write_unaligned(vaga(j + 1) as *mut usize, read_usize(vaga(j)));
            j -= 1;
        }
        core::ptr::write_unaligned(vaga(pos) as *mut usize, p);
        let ult = (read_u8(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(1);
        core::ptr::write_unaligned((c + 0xf) as *mut i16, cpd4_nota(vaga(ult)));
    }
    let vagas = read_u8(c + 0xe);
    if read_u8(c + 0xd) == vagas {
        qsort(vaga(cpd4_inicio(rec, o, nivel)), vagas as u32, 4, 0x778ed0);
    }
}

/// clears the 4 levels of each nation record and puts every eligible person in the nation pool (and the under-21 one,
/// up to 21 years and under 5 caps), also by the second nationality; full section: enters by rating, and sorts (qsort
/// with 0x778ed0)
/// Original: `build_selection_pools` (0x76abf0, national_teams.cpp, thiscall).
///
/// [pt] { 0x76abf0 } (thiscall, national_teams.cpp): `build_selection_pools()`: zera as contagens (+0xd, +0x22) e põe
/// 10000 nas menores notas (+0xf, +0x24) dos 4 níveis de cada registro de nação ([this+4], 0x6e bytes); depois, cada
/// pessoa ([0xae23bc], 0x6e bytes) com nação (+0x1a), ficha (+0x61), mais de 16 anos (+0x18), sem seleção ou já
/// convocada (+0x24, +0x28 = 0xb), na posição de jogador (0x53b620 de +0x3d) e com clube (+0x39) entra na reserva
/// da nação (cpd4_reservar) e, até 21 anos com menos de 5 jogos (+0x22), na do sub-21; e de novo pela segunda
/// nacionalidade (+0x1e: sem jogos pela primeira, outra e não a [0x9cf434]). Sem SEH, sem caixa, sem sorteio; as
/// locais são gravadas antes de ler (da nota guardada em 32 bits só vale o curto).
pub(crate) unsafe extern "thiscall" fn montar_reservas_das_selecoes(this: usize) {
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        for k in 0..4usize {
            let r = read_usize(this + 4) + 0x6e * i as usize + 4 * k;
            *((r + 0xd) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0xf) as *mut i16, 10000);
            *((r + 0x22) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0x24) as *mut i16, 10000);
        }
        i += 1;
    }
    let jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let mut p = *(0xae23bc as *const usize);
    let mut n = 0i32;
    while n < *(0xae2364 as *const i32) {
        let pode = read_usize(p + 0x1a) != 0
            && read_usize(p + 0x61) != 0
            && *((p + 0x18) as *const i8) > 0x10 // mais de 16 anos
            && (read_usize(p + 0x24) == 0 || read_u8(p + 0x28) == 0xb)
            && jogador(read_u8(p + 0x3d) as u32) != 0
            && read_usize(p + 0x39) != 0;
        if pode {
            for k in 0..2 {
                let id = if k == 0 {
                    *(read_usize(p + 0x1a) as *const i32)
                } else {
                    let n2 = read_usize(p + 0x1e);
                    if n2 == 0 || read_u8(p + 0x22) != 0 || n2 == read_usize(p + 0x1a) || *(n2 as *const i32) == *(KEY_NATION_PORTUGAL as *const i32) {
                        continue;
                    }
                    crate::comparacao::ramo("reservas segunda nacionalidade");
                    *(n2 as *const i32)
                };
                let rec = read_usize(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize);
                cpd4_reservar(this, p, rec, 0);
                if *((p + 0x18) as *const i8) > 0x15 || read_u8(p + 0x22) >= 5 { // o sub-21
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
pub(crate) unsafe extern "thiscall" fn comparar_montar_reservas_das_selecoes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(797);
    let n = (*(0xae2350 as *const i32)).max(0) as usize;
    let recs = read_usize(this + 4);
    let mut reg = vec![(recs, 0x6e * n)];
    for i in 0..n {
        let rec = recs + 0x6e * i;
        for sub in 0..2usize {
            let o = 0x15 * sub;
            let total: usize = (0..4).map(|k| read_u8(rec + o + 0xe + 4 * k) as usize).sum();
            if total > 0 {
                reg.push((read_usize(this + 0xc + 4 * sub).wrapping_add(4 * read_usize(rec + o + 8)), 4 * total));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || montar_reservas_das_selecoes(this));
    PLACAR.registrar(igual, || format!("montar_reservas_das_selecoes({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// the pool qsort order: rating (+9 of the player data) descending
/// Original: `compare_by_rating` (0x778ed0, national_teams.cpp, cdecl).
///
/// [pt] { 0x778ed0 } (cdecl, national_teams.cpp): a ordem das reservas para o qsort da 0x76abf0: pela nota (+9 da ficha
/// +0x61) decrescente (nota de b menos a de a).
pub(crate) unsafe extern "C" fn ordem_das_reservas(a: usize, b: usize) -> i32 {
    (cpd4_nota(b) as i32) - cpd4_nota(a) as i32 // decrescente
}

pub(crate) unsafe extern "C" fn comparar_ordem_das_reservas(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(798)(a, b);
    let n = ordem_das_reservas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_reservas({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// the order by club (+0x39): no club last, clubs by id, within a club squad bit 4 before 8 (+0x5e)
/// Original: `compare_by_club` (0x778ef0, national_teams.cpp, cdecl).
///
/// [pt] { 0x778ef0 } (cdecl, national_teams.cpp): a ordem das pessoas pelo clube (+0x39) para os qsort das seleções
/// (0x76c120, 0x7706c0, 0x7771c0): sem clube depois; clubes diferentes pelo id; no mesmo, o de elenco 4 (+0x5e)
/// antes do de elenco 8.
pub(crate) unsafe extern "C" fn ordem_pelo_clube(a: usize, b: usize) -> i32 {
    let (x, y) = (read_usize(a), read_usize(b));
    let cx = if x != 0 { read_usize(x + 0x39) } else { 0 };
    let cy = if y != 0 { read_usize(y + 0x39) } else { 0 };
    if cx == 0 {
        return (cy != 0) as i32;
    }
    if cy == 0 {
        return -1;
    }
    if cx != cy {
        return (*(cx as *const i32)).wrapping_sub(*(cy as *const i32)); // o id do clube
    }
    let (mx, my) = (read_u8(x + 0x5e), read_u8(y + 0x5e));
    if mx & 4 != 0 && my & 8 != 0 {
        return -1;
    }
    if mx & 8 != 0 && my & 4 != 0 {
        return 1;
    }
    0
}

pub(crate) unsafe extern "C" fn comparar_ordem_pelo_clube(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(799)(a, b);
    let n = ordem_pelo_clube(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pelo_clube({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 211a leva: o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados (national_teams.cpp, fix_man.cpp, shortlist_manager.cpp) ----------

/// takes the player out of his level pool in the nation and out of the squad, clearing his national team fields
/// Original: `NationalTeams::remove_player` (0x775ba0, national_teams.cpp, thiscall).
///
/// [pt] { 0x775ba0 } (thiscall, ret 4, national_teams.cpp): tira o `jogador` da seleção. Com nação (+0x1a)
/// e ficha (+0x61): no registro de 110 bytes da nação em [this+4], o nível (0x76d810 com o registro e
/// 0); se a nota do nível (palavra +0xf + nível x 4) for menor que a da ficha (+9), procura o jogador
/// na faixa do nível da lista [this+0xc] (início = dword +8 + bytes +0xe dos níveis anteriores,
/// tamanho no byte +0xd + nível x 4): achou, o último da faixa vai para a vaga dele e o tamanho
/// diminui. Depois, convocado (+0x24): a vaga dele entre as 50 (+0xd7) zera, +0x24 zera e o byte +0x5e
/// fica só com os bits 0..1.
pub(crate) unsafe extern "thiscall" fn tirar_jogador_da_selecao(this: usize, jogador: usize) {
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return;
    }
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
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



pub(crate) unsafe extern "thiscall" fn comparar_tirar_jogador_da_selecao(this: usize, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(517);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return o(this, jogador);
    }
    // as regiões: o jogador, o registro da nação, a faixa do nível na lista e a seleção (o nível,
    // 0x76d810, só lê)
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
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



// ---------- 210a leva: as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional (national_teams.cpp, tactics.cpp, media.cpp) ----------

/// As 50 vagas de convocação (dwords desalinhados em +0xd7) das duas seleções da nação: a principal
/// (0x53b3d0) e a B (0x53b4f0), nessa ordem.
pub(crate) unsafe fn convocacoes_das_selecoes(nacao: usize) -> [usize; 2] {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    [principal(nacao).wrapping_add(0xd7), b(nacao).wrapping_add(0xd7)]
}

/// clears repeated squad entries from the senior and B national sides of a nation
/// Original: `national_teams_remove_duplicate_squad_entries` (0x76c060, national_teams.cpp, stdcall).
///
/// [pt] { 0x76c060 } (stdcall, ret 4, national_teams.cpp): apaga as convocações repetidas das duas
/// seleções da nação: em cada uma, todo id não nulo zera as vagas seguintes com o mesmo id.
pub(crate) unsafe extern "stdcall" fn apagar_repetidos_das_convocacoes(nacao: usize) {
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



pub(crate) unsafe extern "stdcall" fn comparar_apagar_repetidos_das_convocacoes(nacao: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(508);
    if nacao == 0 {
        return o(nacao);
    }
    let [a, b] = convocacoes_das_selecoes(nacao);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 200), (b, 200)], || o(nacao), || apagar_repetidos_das_convocacoes(nacao));
    PLACAR.registrar(igual, || format!("apagar_repetidos_das_convocacoes {:x?} x {:x?}", mo, mr));
}

/// the player's level 1..3 for the national team by reputation and position ratings
/// Original: `national_team_level` (0x76d810, national_teams.cpp, stdcall).
///
/// [pt] { 0x76d810 } (stdcall, ret 0xc, national_teams.cpp): o nível 1..3 do jogador `j` para a seleção
/// (0 com reputação `*rep` abaixo de 100, ou 50 com `base`, ou goleiro de nota 15): 1 para líbero ou
/// zagueiro, 2 para volante, meia ou meia-atacante (nota 15 em +0x10..+0x14 da ficha), senão 3.
/// Devolve só `al`.
pub(crate) unsafe extern "stdcall" fn nivel_para_a_selecao(j: usize, rep: *const i16, base: u8) -> u8 {
    if *rep < if base != 0 { 0x32 } else { 0x64 } {
        return 0;
    }
    let ficha = core::ptr::read_unaligned((j + 0x61) as *const usize);
    let n = |o: usize| *((ficha + o) as *const i8) >= 0xf;
    if n(0xf) {
        0
    } else if n(0x10) || n(0x11) {
        1
    } else if n(0x12) || n(0x13) || n(0x14) {
        2
    } else {
        3
    }
}

pub(crate) unsafe extern "stdcall" fn comparar_nivel_para_a_selecao(j: usize, rep: *const i16, base: u8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8>(408)(j, rep, base);
    let n = nivel_para_a_selecao(j, rep, base);
    PLACAR.registrar(r == n, || format!("nivel_para_a_selecao({:#x}, {}, {}) original={} rust={}", j, *rep, base, r, n));
    r
}

/// the national teams day: the table of clubs with a match is freed; on 1 January, 15 April and 1 August the pools are
/// rebuilt; for the matches two weeks ahead, the clubs playing around the date are marked, the national teams call up
/// and the call-ups per club are recorded (outside the finals); with the table, the matches of clubs with called-up
/// players are rescheduled; the day's items leave, the FIFA ranking moves, every 70 days the records are pruned and the
/// qualified nations news goes out
/// Original: `NationalTeams::daily_update` (0x76b100, national_teams.cpp, thiscall).
///
/// [pt] { 0x76b100 } (thiscall, ret 4, national_teams.cpp, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::daily_update(hoje)`, pelo dia do jogo ([0xb74318], 0x5ccb07). A data daqui a duas semanas
/// (0x54c770); a tabela dos clubes com jogo (+0x24) sai (0x9452ca) e zera. Em 1º de janeiro, 15 de abril e 1º de agosto
/// (0x549f70 no ano de hoje), as reservas das seleções se refazem (0x76abf0). Nas 3 fatias do dia daqui a duas
/// semanas, as listas dos grupos 0 e 1 (0x5abf10): com jogo em alguma e sem a tabela, os clubes com jogo em volta da
/// data são marcados (cqc12_marcar_tres_dias); os jogos das duas listas (cqc12_jogos). Com a tabela, os jogos dos clubes
/// com convocados se remarcam (0x7777e0). Da lista em +0 saem (0x68d1f0) os itens do dia de hoje (+0x10). Com o ranking
/// (+0x1c), a atualização dele (0x58d460); no dia múltiplo de 70, a poda dos registros de convocação (0x7774b0); por fim
/// as notícias dos classificados (0x7779e0).
pub(crate) unsafe extern "thiscall" fn dia_das_selecoes(this: usize, hoje: usize) {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let refazer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_BUILD_SELECTION_POOLS);
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_GET_FIXTURE_LIST);
    let remarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_RESCHEDULE_CLASHING_CLUB_FIXTURES);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let ranking: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIFA_RANKINGS_UPDATE_RANKINGS);
    let podar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_PRUNE_CLUB_CALL_UP_RECORDS);
    let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_POST_QUALIFIED_NATIONS_NEWS);
    // 0x54c770: push 2; push saida; mov ecx, hoje -> thiscall ret 8; free: add esp, 4 -> cdecl; 0x549f70: 4 pushes;
    // lea ecx, saida -> thiscall ret 0x10; 0x76abf0, 0x7774b0, 0x7779e0: mov ecx, edi sem pushes -> thiscall;
    // 0x5abf10: 4 pushes; mov ecx, [0xae2a58] -> thiscall ret 0x10; 0x7777e0: push &data; mov ecx, edi -> thiscall
    // ret 4; 0x68d1f0: push item; mov ecx, [edi] -> thiscall ret 4; 0x58d460: push hoje; mov ecx, [edi + 0x1c] ->
    // thiscall ret 4
    let mut dd = [0u8; 8];
    let d = dd.as_mut_ptr() as usize;
    semanas(hoje, d, 2);
    if read_usize(this + 0x24) != 0 {
        liberar(read_usize(this + 0x24));
        write_usize(this + 0x24, 0);
    }
    // ponytail: a original empurra o ano (palavra) e a semana com sobra de registrador em cima; a 0x549f70 lê as palavras
    let ano = cqc12_uw(hoje + 2) as u32;
    let mut t = [0u8; 8];
    let pt = t.as_mut_ptr() as usize;
    if cqc12_e_hoje(dma(pt, 1, 0, ano, u32::MAX), hoje) // cqc12: 1º de janeiro
        || cqc12_e_hoje(dma(pt, 0xf, 3, ano, u32::MAX), hoje)
        || cqc12_e_hoje(dma(pt, 1, 7, ano, u32::MAX), hoje)
    {
        crate::comparacao::ramo("cqc_12 refaz as reservas");
        refazer(this);
    }
    let fixman = read_usize(0xae2a58);
    for fatia in 0u32..3 {
        // os dwords da original: o ano (com os 2 bytes seguintes da data) e o dia (com o ano em cima); ponytail: a
        // fatia vai sem a sobra de pilha dos bytes de cima (a 0x5abf10 faz and 0xff)
        let l0 = lista(fixman, read_usize(d + 2) as u32, read_usize(d) as u32, fatia, 0);
        let l1 = lista(fixman, read_usize(d + 2) as u32, read_usize(d) as u32, fatia, 1);
        if ((l0 != 0 && read_i16(l0) != 0) || (l1 != 0 && read_i16(l1) != 0)) && read_usize(this + 0x24) == 0 {
            cqc12_marcar_tres_dias(this, d); // cqc12: os clubes com jogo em volta da data
        }
        if l0 != 0 {
            cqc12_jogos(this, l0);
        }
        if l1 != 0 {
            cqc12_jogos(this, l1);
        }
    }
    if read_usize(this + 0x24) != 0 {
        remarcar(this, d);
    }
    if read_usize(this) != 0 {
        let dia = read_i16(hoje);
        let l = read_usize(this);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut p = proximo(read_usize(this));
        while p != 0 {
            if read_i16(p + 0x10) == dia {
                tirar(read_usize(this), p);
            }
            p = proximo(read_usize(this));
        }
    }
    if read_usize(this + 0x1c) != 0 {
        ranking(read_usize(this + 0x1c), hoje);
    }
    if read_i16(hoje) as i32 % 0x46 == 0 {
        podar(this);
    }
    classificados(this);
}

/// Comparador: a original convoca (0x76bab0: notícias, elencos), remarca jogos (0x7777e0), aloca a tabela (0x7775c0)
/// e libera itens (0x68d1f0): não pode rodar duas vezes. Só ela roda, conferindo o plano do que a própria rotina grava:
/// nenhum item do dia de hoje sobra na lista em +0.
pub(crate) unsafe extern "thiscall" fn comparar_dia_das_selecoes(this: usize, hoje: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1539);
    let dia = read_i16(hoje);
    let antes = cqc12_do_dia(this, dia);
    o(this, hoje);
    let sobra = cqc12_do_dia(this, dia);
    PLACAR.registrar(sobra == 0, || format!("dia_das_selecoes(dia {}) itens do dia antes {} depois {}", dia, antes, sobra));
}

/// the age limits (23/23 in youth competitions, 24/22, 26/22)
/// Original: `national_team_ages` (0x76d430, national_teams.cpp, stdcall).
///
/// [pt] { 0x76d430 } (stdcall, ret 0x10, national_teams.cpp): as idades limite (`*minima`, `*maxima`) de
/// uma seleção na competição `comp`: 23/23 nas competições olímpicas e de base ([comp+0x14] com id
/// numa das 9 constantes 0x9cf78c…0x9cf964); 24/22 com `tipo` 1 ou acima de 2; senão 26/22.
/// Ponteiros nulos: a original (caixa de erro).
pub(crate) unsafe extern "stdcall" fn idades_da_selecao(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    if minima.is_null() || maxima.is_null() {
        return orig::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(368)(comp, tipo, minima, maxima);
    }
    if comp != 0 {
        let c = *((comp + 0x14) as *const usize);
        if c != 0 {
            let id = *(c as *const i32);
            if [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
                .iter()
                .any(|&a| *(a as *const i32) == id)
            {
                *minima = 0x17;
                *maxima = 0x17;
                return;
            }
        }
    }
    if tipo != 0 && tipo != 2 {
        *minima = 0x18;
        *maxima = 0x16;
    } else {
        *minima = 0x1a;
        *maxima = 0x16;
    }
}

pub(crate) unsafe extern "stdcall" fn comparar_idades_da_selecao(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    let o = orig::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(368);
    let reg = if minima.is_null() || maxima.is_null() { vec![] } else { vec![(minima as usize, 1), (maxima as usize, 1)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(comp, tipo, minima, maxima), || idades_da_selecao(comp, tipo, minima, maxima));
    PLACAR.registrar(igual, || format!("idades_da_selecao({:#x}, {}) {:x?} x {:x?}", comp, tipo, mo, mr));
}

/// sorts the national squad by club and adds, on the match date, how many players each club gives up (the record that
/// stops club fixtures near that date)
/// Original: `NationalTeams::record_club_call_ups` (0x7771c0, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7771c0 } (thiscall, ret 0xc, national_teams.cpp; quadro SEH só do estado, as datas não têm destrutor):
/// `NationalTeams::record_club_call_ups(time, b, partida)`: ordena o elenco do time (+0xd7, 50 vagas) pelo clube (qsort
/// com 0x778ef0). Com a partida: a data dela (+0x2a, +0x28; um dia antes no segundo time 0x5409b0 ou com `b`); conta os
/// convocados seguidos de cada clube (+0x39), pulando as vagas vazias, as pessoas fora da faixa ([0xae2364]) e as de
/// outro elenco (+0x5e bit 3 sem `b`, bit 2 com), e ao mudar de clube soma a conta do anterior no registro dele
/// (cpw13_somar). O último clube da volta fica sem registro (como na original).
pub(crate) unsafe extern "thiscall" fn registrar_convocados_por_clube(this: usize, time: usize, b: u32, p: usize) {
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let mut d = [0u8; 8];
    padrao(d.as_mut_ptr());
    qsort(time + 0xd7, 0x32, 4, 0x778ef0);
    if p == 0 {
        return;
    }
    let nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let menos: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    // ponytail: a original empurra os curtos +0x2a e +0x28 com o alto do registro; a 0x549ef0 só lê os curtos
    let r = nova(t1.as_mut_ptr(), *((p + 0x2a) as *const u16) as u32, *((p + 0x28) as *const u16) as u32);
    copia(d.as_mut_ptr(), t2.as_mut_ptr(), r);
    if segunda(time) != 0 || b != 0 {
        let r = menos(d.as_mut_ptr(), t2.as_mut_ptr(), 1);
        copia(d.as_mut_ptr(), t1.as_mut_ptr(), r);
    }
    let mut anterior = 0usize;
    let mut conta: u8 = 0;
    for i in 0..0x32usize {
        let s = read_usize(time + 0xd7 + 4 * i);
        if s == 0 || read_usize(s) as i32 >= read_usize(0xae2364) as i32 {
            continue;
        }
        let marcas = *((s + 0x5e) as *const u8);
        if (b == 0 && marcas & 8 != 0) || (b != 0 && marcas & 4 != 0) {
            continue;
        }
        let c = read_usize(s + 0x39);
        if c == anterior && anterior != 0 {
            conta = conta.wrapping_add(1);
            continue;
        }
        if anterior != 0 && !cpw13_somar(this, anterior, conta, d.as_mut_ptr()) {
            return;
        }
        anterior = c;
        conta = 1;
    }
    if anterior != 0 && cpw13_registro(this, anterior).is_some() {
        crate::comparacao::ramo("cpw_13 último clube sem registro"); // cpw13: o último clube fica sem registro
    }
}

/// Comparador: lado a lado no elenco do time (50 vagas) e, de cada clube do elenco que conta, no registro (6 bytes) e
/// no vetor de entradas inteiro (capacidade × 9). Quando algum vetor pode crescer (0x945667 solta o velho; não roda
/// duas vezes): só a original, conferindo depois o elenco ordenado pelo clube (o mesmo qsort sobre uma cópia, antes).
pub(crate) unsafe extern "thiscall" fn comparar_registrar_convocados_por_clube(this: usize, time: usize, b: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(1305);
    if p != 0 && cpw13_pode_realocar(this, time) {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        let mut esperado: Vec<u32> = (0..0x32usize).map(|i| read_usize(time + 0xd7 + 4 * i) as u32).collect();
        // o mesmo qsort da CRT (a ordem dos iguais é a dele), fora do rastro da original
        crate::aleatorio::pausar_rastro(|| qsort(esperado.as_mut_ptr() as usize, 0x32, 4, 0x778ef0));
        o(this, time, b, p);
        let obtido: Vec<u32> = (0..0x32usize).map(|i| read_usize(time + 0xd7 + 4 * i) as u32).collect();
        PLACAR.registrar(obtido == esperado, || format!("registrar_convocados_por_clube({:x}) elenco original={:x?} esperado={:x?}", time, obtido, esperado));
        return;
    }
    let mut reg = vec![(time + 0xd7, 0xc8)];
    let mut vistos = Vec::new();
    for i in 0..0x32usize {
        let s = read_usize(time + 0xd7 + 4 * i);
        if s == 0 || read_usize(s) as i32 >= read_usize(0xae2364) as i32 {
            continue;
        }
        let c = read_usize(s + 0x39);
        if c == 0 || vistos.contains(&c) {
            continue;
        }
        vistos.push(c);
        if let Some(rec) = cpw13_registro(this, c) {
            reg.push((rec, 6));
            let cap = *((rec + 4) as *const i8);
            if read_usize(rec) != 0 && cap > 0 {
                reg.push((read_usize(rec), 9 * cap as usize));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, time, b, p), || registrar_convocados_por_clube(this, time, b, p));
    PLACAR.registrar(igual, || format!("registrar_convocados_por_clube({:x}, b {}, partida {:x}) {:x?} x {:x?}", time, b, p, mo, mr));
}
