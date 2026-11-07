// ---------- 267a leva: o fim da falta ao treino e da ausência sem licença e a notícia da falta ao treino (awol.cpp) ----------



/// Os itens da lista (o ponteiro em +0 de cada nó, da cabeça +2 seguindo o +6), sem mexer no cursor.
unsafe fn cqd11_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = read_usize(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

/// O corpo comum de { 0x4196a0 } e { 0x4197c0 }: com a marca `bit` da pessoa ([this+0xc] + id), a marca troca (o
/// `or bit` da original nunca dá zero) e, com a lista ([this + `lista`]), o cursor volta à cabeça e, entre os primeiros
/// `conta` (+0, palavra com sinal) itens, o primeiro com o id da pessoa em +0 sai (0x68d1f0, que libera o nó). Uma lista
/// que acaba antes da conta encerra.
unsafe fn cqd11_tirar(this: usize, pessoa: usize, bit: u8, lista: usize, falta: bool) {
    if pessoa == 0 {
        return;
    }
    let marcas = read_usize(this + 0xc);
    if marcas == 0 {
        return;
    }
    let p = marcas.wrapping_add(read_usize(pessoa));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return;
    }
    if falta {
        crate::comparacao::ramo("cqd_11 falta tirada");
    }
    *(p as *mut u8) = m ^ bit;
    let l = read_usize(this + lista);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    // 0x68d1b0: mov ecx, [edi + lista] -> thiscall; 0x68d1f0: push item; mov ecx, [edi + lista]; ret 4 -> thiscall
    let mut i: i32 = 0;
    if read_i16(l) as i32 <= 0 {
        return;
    }
    loop {
        let item = proximo(read_usize(this + lista));
        if item == 0 {
            return;
        }
        if read_usize(item) == read_usize(pessoa) {
            if falta {
                crate::comparacao::ramo("cqd_11 registro da falta tirado");
            }
            tirar(read_usize(this + lista), item);
            return;
        }
        i += 1;
        if i >= read_i16(read_usize(this + lista)) as i32 {
            return;
        }
    }
}

/// { 0x4196a0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_awol(pessoa)`, pelo fim da ausência sem contrato (0x418f60),
/// pelas aposentadorias (0x7a59e0) e pelas transferências (0x8c1af0, 0x8c2180, 0x8c9720). Pessoa e tabela de marcas
/// ([this+0xc]) não nulas e o bit 1 da marca: troca o bit e tira o registro da pessoa da lista das ausências ([this]).
unsafe extern "thiscall" fn limpar_ausencia(this: usize, pessoa: usize) {
    cqd11_tirar(this, pessoa, 1, 0, false);
}

/// { 0x4197c0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_missed_training(pessoa)`, pela falta ao treino de quem ficou
/// sem contrato (0x418ff0, que os jogadores do dia consultam) e pelas aposentadorias (0x7a59e0). Pessoa e tabela de
/// marcas não nulas e o bit 2 da marca: troca o bit e tira o registro da pessoa da lista das faltas ([this+4]).
unsafe extern "thiscall" fn limpar_falta_ao_treino(this: usize, pessoa: usize) {
    cqd11_tirar(this, pessoa, 2, 4, true);
}

/// O plano das duas: a marca depois e os itens que ficam na lista (o primeiro com o id da pessoa entre os `conta`
/// primeiros sai). Sem nada a fazer: o estado de antes.
unsafe fn cqd11_plano(this: usize, pessoa: usize, bit: u8, lista: usize) -> (Option<(usize, u8)>, usize, Vec<usize>) {
    let l = read_usize(this + lista);
    let itens = cqd11_itens(l);
    if pessoa == 0 || read_usize(this + 0xc) == 0 {
        return (None, l, itens);
    }
    let p = read_usize(this + 0xc).wrapping_add(read_usize(pessoa));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return (Some((p, m)), l, itens);
    }
    if l == 0 {
        return (Some((p, m ^ bit)), l, itens);
    }
    let conta = (read_i16(l) as i32).max(0) as usize;
    let mut fica = itens.clone();
    if let Some(k) = itens.iter().take(conta).position(|&x| read_usize(x) == read_usize(pessoa)) {
        fica.remove(k);
    }
    (Some((p, m ^ bit)), l, fica)
}

/// Comparador das duas: a original tira e libera o nó da lista (não pode rodar duas vezes): só ela roda, com o plano
/// montado antes (a marca e os itens que ficam, sem ler o item tirado depois).
unsafe fn cqd11_comparar(this: usize, pessoa: usize, bit: u8, lista: usize, o: unsafe extern "thiscall" fn(usize, usize), nome: &str) {
    let (marca, l, fica) = cqd11_plano(this, pessoa, bit, lista);
    o(this, pessoa);
    let depois = cqd11_itens(l);
    let marca_ok = marca.map_or(true, |(p, m)| *(p as *const u8) == m);
    PLACAR.registrar(marca_ok && depois == fica, || format!("{}(pessoa {:#x}) marca plano {:x?} agora {:x?}; lista plano {:x?} agora {:x?}",
        nome, pessoa, marca, marca.map(|(p, _)| *(p as *const u8)), fica, depois));
}

unsafe extern "thiscall" fn comparar_limpar_ausencia(this: usize, pessoa: usize) {
    cqd11_comparar(this, pessoa, 1, 0, orig::<unsafe extern "thiscall" fn(usize, usize)>(1570), "limpar_ausencia");
}

unsafe extern "thiscall" fn comparar_limpar_falta_ao_treino(this: usize, pessoa: usize) {
    cqd11_comparar(this, pessoa, 2, 4, orig::<unsafe extern "thiscall" fn(usize, usize)>(1571), "limpar_falta_ao_treino");
}

/// O ensaio da notícia: os sorteios vêm da fita da original (n e resposta) e, em vez de entregar, a cópia anota o
/// destino (1 o clube do jogador, 2 a nação do clube, 0 nenhum). A fita fica com o próximo e se algum n não bateu.
static mut CQD11_FITA: Option<(Vec<(i32, i32)>, usize, bool)> = None;
static mut CQD11_DESTINO: (u32, usize) = (0, 0);

unsafe fn cqd11_sortear(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(CQD11_FITA);
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

/// { 0x41a630 } (thiscall, ret 8, awol.cpp, SEH da notícia local, sem destrutor): `awol_post_missed_training_news(jogador,
/// clube)`, pela falta ao treino (0x41a360). Jogador com ficha (+0x61) e clube com técnico (+0xcf): a notícia 0xbc9
/// (0x77bbe0) com o jogador (id e as chaves +0x33 de +4, +8, +0xc), o id do clube, sorteio(3), o clube de humano que
/// decide (0x600a50 com 0, 1) e quem relata: o técnico, se o clube tem técnico e não é de humano (0x600a50 com 1, 1;
/// id e chaves); senão, com sorteio(2), um dos 5 treinadores (+0x19f, sorteio(5) até achar ou mil voltas), sem
/// sorteio o auxiliar (+0xd3); sem ninguém (ou id −1), −1 nos quatro. Clube de humano (0x7875e0): ao clube do jogador
/// (+0x39, 0x7861e0); senão, com nação (+0x53) e reputação do jogador (+0x61, +0xb) acima de 7250, às caixas da nação
/// (0x7863f0).
unsafe extern "thiscall" fn noticia_da_falta_ao_treino(this: usize, jogador: usize, clube: usize) {
    let _ = this;
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || clube == 0 || read_usize(clube + 0xcf) == 0 { // cqd11: sem jogador ou técnico
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    // 0x77bbe0: push 0; push 0xbc9; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall; 0x600a50: 5
    // pushes; add esp, 0x14 -> cdecl (clube, 0, 1, 0, 0) e (clube, 1, 1, 0, 0)
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbc9, 0);
    gravar(nt, 0, read_usize(jogador) as u32);
    gravar(nt, 1, read_usize(read_usize(jogador + 4) + 0x33) as u32);
    gravar(nt, 2, read_usize(read_usize(jogador + 8) + 0x33) as u32);
    gravar(nt, 3, read_usize(read_usize(jogador + 0xc) + 0x33) as u32);
    gravar(nt, 4, read_usize(clube) as u32);
    crate::comparacao::ramo("cqd_11 noticia da falta");
    let r = cqd11_sortear(3); // cqd11: o sorteio da notícia
    gravar(nt, 5, r as u32);
    gravar(nt, 6, humano(clube, 0, 1, 0, 0));
    let tecnico = read_usize(clube + 0xcf);
    if tecnico != 0 && humano(clube, 1, 1, 0, 0) == 0 { // cqd11: o técnico da IA relata
        crate::comparacao::ramo("cqd_11 relator tecnico");
        let t = read_usize(clube + 0xcf);
        gravar(nt, 7, read_usize(t) as u32);
        gravar(nt, 8, read_usize(read_usize(t + 4) + 0x33) as u32);
        gravar(nt, 9, read_usize(read_usize(t + 8) + 0x33) as u32);
        gravar(nt, 0xa, read_usize(read_usize(t + 0xc) + 0x33) as u32);
    } else {
        let quem = if cqd11_sortear(2) != 0 {
            let mut k;
            let mut voltas = 0;
            loop {
                k = cqd11_sortear(5);
                voltas += 1;
                if voltas >= 1000 || read_usize(clube.wrapping_add(k as usize * 4) + 0x19f) != 0 {
                    break;
                }
            }
            read_usize(clube.wrapping_add(k as usize * 4) + 0x19f)
        } else {
            read_usize(clube + 0xd3)
        };
        let id = if quem != 0 { read_usize(quem) as i32 } else { -1 };
        if id == -1 {
            for k in 7..=0xa {
                gravar(nt, k, u32::MAX);
            }
        } else {
            let p = read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            gravar(nt, 7, id as u32);
            gravar(nt, 8, read_usize(read_usize(p + 4) + 0x33) as u32);
            gravar(nt, 9, read_usize(read_usize(p + 8) + 0x33) as u32);
            gravar(nt, 0xa, read_usize(read_usize(p + 0xc) + 0x33) as u32);
        }
    }
    let noticias = read_usize(0xb7431c);
    let de_humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    // 0x7875e0: mov ecx, [0xb7431c]; push clube (ret 4); 0x7861e0: push clube +0x39; push notícia; mov ecx, [0xb7431c];
    // 0x7863f0: push nação; push notícia; mov ecx, [0xb7431c] -> thiscall nas três
    let ensaio = (*core::ptr::addr_of!(CQD11_FITA)).is_some();
    if de_humano(noticias, clube) != 0 {
        if ensaio {
            CQD11_DESTINO = (1, read_usize(jogador + 0x39));
        } else {
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
            entregar(noticias, nt, read_usize(jogador + 0x39));
        }
        return;
    }
    let nacao = read_usize(clube + 0x53);
    if nacao == 0 || read_i16(read_usize(jogador + 0x61) + 0xb) <= 0x1c52 {
        return;
    }
    if ensaio {
        CQD11_DESTINO = (2, nacao);
    } else {
        let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
        a_nacao(noticias, nt, nacao);
    }
}

/// Comparador: a original monta a notícia e a entrega (aloca na caixa): só ela roda, com os sorteios rastreados; depois
/// a cópia em ensaio refaz o caminho com os sorteios da fita, sem entregar. Confere que os sorteios do ensaio são o
/// começo da fita da original (o resto é da entrega) com os mesmos n, e anota o destino (a original não o expõe).
unsafe extern "thiscall" fn comparar_noticia_da_falta_ao_treino(this: usize, jogador: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1572);
    let ((), fita) = crate::aleatorio::rastrear(|| o(this, jogador, clube));
    let n = fita.len();
    *core::ptr::addr_of_mut!(CQD11_FITA) = Some((fita, 0, false));
    CQD11_DESTINO = (0, 0);
    crate::aleatorio::sem_contar(|| noticia_da_falta_ao_treino(this, jogador, clube));
    let (k, falhou) = match &*core::ptr::addr_of!(CQD11_FITA) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(CQD11_FITA) = None;
    let destino = CQD11_DESTINO;
    PLACAR.registrar(!falhou && k <= n, || format!("noticia_da_falta_ao_treino(jogador {:#x}, clube {:#x}) fita de {} sorteios, o ensaio usou {} (n diferente: {}), destino {:x?}",
        jogador, clube, n, k, falhou, destino));
}

// ---------- 260a leva: as faltas do dia: o dia das ausências, das faltas ao treino e dos atrasos, a falta ao treino marcada, o atraso registrado, marcar a falta, o fim da ausência e as marcas do contrato (awol.cpp, staff_contracts.cpp) ----------


unsafe fn cpw7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}




unsafe fn cpw7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// O byte com sinal `a` (atributo de 1 a 20) cai um com o sorteio dele: zero (`zero`) ou não zero, se for maior que 1.
unsafe fn cpw7_cair(a: usize, zero: bool) {
    let v = read_u8(a) as i8;
    if v > 1 && (cpw7_sortear(v as i32) == 0) == zero {
        *(a as *mut i8) = v - 1;
    }
}

unsafe fn cpw7_proximo(lista: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    f(lista)
}

/// O cursor da lista volta ao primeiro nó (+0xa = +2).
unsafe fn cpw7_inicio(lista: usize) {
    write_u32(lista + 0xa, read_usize(lista + 2) as u32);
}

/// A data `d` (dia com sinal em +0, ano sem sinal em +2) já passou: hoje ([0xae2c90]) é depois dela.
unsafe fn cpw7_passou(d: usize) -> bool {
    let (dia, ano) = (read_i16(0xae2c90), read_i16(0xae2c92) as u16);
    let a = read_i16(d + 2) as u16;
    ano > a || (ano == a && dia > read_i16(d))
}

/// Uma semana depois da data `d` (0x54c770), já passou?
unsafe fn cpw7_semana_passou(d: usize, semanas: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let mut t = [0u8; 8];
    cpw7_passou(f(d, &mut t, semanas))
}

/// O fim do caso de falta ao treino `r` (pessoa em +0): com o byte da pessoa na tabela de marcas ([this+0xc]) não zero,
/// troca o bit 2 dele (o `or 2` da original nunca dá zero) e tira o registro da lista das faltas ([this+4], 0x68d1f0).
unsafe fn cpw7_tirar_falta(this: usize, r: usize) {
    let m = read_usize(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m ^= 2;
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    crate::comparacao::ramo("cpw_7 falta tirada da lista");
    tirar(read_usize(this + 4), r); // cpw7: tira o caso da lista
}

unsafe fn cpw7_faltou(this: usize, pessoa: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_MISS_TRAINING);
    f(this, pessoa);
}

/// { 0x4190d0 } (thiscall, awol.cpp): `AWOL::daily_update()`, todo dia pelo laço do jogo (0x5cccd0). Três listas com o
/// cursor em +0xa: (1) ausências ([this]): a primeira cuja volta (data em +4) já passou acaba (0x419710); (2) faltas ao
/// treino ([this+4]): a primeira com uma semana passada da data (+4) é resolvida: caso repetido (byte +0xc) com dois
/// sorteios zero pelo profissionalismo (+0x5b) e pelo temperamento (+0x5d) da pessoa, os dois caem pelo sorteio e ela
/// falta de novo (0x41a360); caso novo: os colegas do clube (50 vagas em +0xd7) com o contrato do faltoso podem pedir a
/// saída (0x867560 com 2) ou perder profissionalismo, moral (+0x44 do jogador) e temperamento; e o faltoso, por
/// sorteio(profissionalismo) abaixo de sorteio(15) + 1, perde profissionalismo e moral, falta de novo e briga com o
/// clube (0x72c1f0 com 8); (3) atrasos ([this+8]): o primeiro com uma semana passada (+8) que vença os sorteios perde
/// profissionalismo e temperamento ou moral. Cada caso resolvido sai da lista (0x68d1f0) e a marca do bit 2 troca. Uma
/// lista que acaba antes da conta encerra o dia.
unsafe extern "thiscall" fn faltas_do_dia(this: usize) {
    // (1) as ausências
    let l = read_usize(this);
    if l != 0 {
        cpw7_inicio(l);
        if read_i16(l) > 0 {
            let mut i = 0;
            loop {
                let r = cpw7_proximo(read_usize(this));
                if r == 0 {
                    return;
                }
                if cpw7_passou(r + 4) {
                    let fim: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_END_ABSENCE);
                    fim(this, r);
                    break;
                }
                i += 1;
                if i >= read_i16(read_usize(this)) as i32 {
                    break;
                }
            }
        }
    }
    // (2) as faltas ao treino
    let l = read_usize(this + 4);
    if l != 0 {
        cpw7_inicio(l);
        if read_i16(l) > 0 {
            let mut i = 0;
            loop {
                let r = cpw7_proximo(read_usize(this + 4));
                if r == 0 {
                    return;
                }
                if cpw7_semana_passou(r + 4, 1) { // cpw7: uma semana depois
                    crate::comparacao::ramo("cpw_7 falta vencida");
                    cpw7_resolver_falta(this, r);
                    break;
                }
                i += 1;
                if i >= read_i16(read_usize(this + 4)) as i32 {
                    break;
                }
            }
        }
    }
    // (3) os atrasos
    let l = read_usize(this + 8);
    if l == 0 {
        return;
    }
    cpw7_inicio(l);
    if read_i16(l) <= 0 {
        return;
    }
    let mut i = 0;
    loop {
        let r = cpw7_proximo(read_usize(this + 8));
        if r == 0 {
            return;
        }
        if cpw7_semana_passou(r + 8, 1) {
            let pessoa = read_usize(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
            let resolveu = if read_u8(r + 0x10) != 0 {
                if read_usize(pessoa + 0x61) == 0 {
                    true
                } else if cpw7_sortear(read_u8(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(read_u8(pessoa + 0x5d) as i8 as i32) == 0 {
                    cpw7_cair(pessoa + 0x5b, true);
                    cpw7_cair(pessoa + 0x5d, true);
                    true
                } else {
                    false
                }
            } else {
                let a = cpw7_sortear(read_u8(pessoa + 0x5b) as i8 as i32);
                let b = cpw7_sortear(5);
                if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
                    cpw7_cair(pessoa + 0x5b, true);
                    let j = read_usize(pessoa + 0x61);
                    if j != 0 {
                        cpw7_cair(j + 0x44, true);
                    }
                    true
                } else {
                    false
                }
            };
            if resolveu {
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
                tirar(read_usize(this + 8), r);
                return;
            }
        }
        i += 1;
        if i >= read_i16(read_usize(this + 8)) as i32 {
            return;
        }
    }
}

/// A falta ao treino `r` com uma semana passada (o meio da lista (2) de `faltas_do_dia`).
unsafe fn cpw7_resolver_falta(this: usize, r: usize) {
    let pessoa = read_usize(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
    if read_u8(r + 0xc) != 0 {
        // o caso repetido
        if read_usize(pessoa + 0x61) != 0 {
            crate::comparacao::ramo("cpw_7 caso repetido");
            if cpw7_sortear(read_u8(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(read_u8(pessoa + 0x5d) as i8 as i32) == 0 { // cpw7: os sorteios do caso repetido
                cpw7_cair(pessoa + 0x5b, true);
                cpw7_cair(pessoa + 0x5d, true);
                cpw7_tirar_falta(this, r);
                cpw7_faltou(this, pessoa);
                return;
            }
        }
        return cpw7_tirar_falta(this, r);
    }
    // o caso novo: os colegas do clube
    crate::comparacao::ramo("cpw_7 caso novo");
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let k = contrato_de(read_usize(0xae1050), pessoa);
    write_u32(0xae1080, k as u32);
    let clube = if k != 0 { read_usize(0xae23b4).wrapping_add(read_usize(k + 4).wrapping_mul(0x245)) } else { 0 };
    if clube != 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
        let pedir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_SET_FLAGS);
        for v in 0..0x32usize {
            let s = read_usize(clube + 0xd7 + v * 4);
            if s == 0 || s == pessoa || read_usize(s + 0x61) == 0 {
                continue;
            }
            let ct = contrato(read_usize(0xae1050), pessoa, clube);
            if ct == 0 {
                continue;
            }
            if marcas(ct) & 2 == 0 && read_u8(s + 0x5b) as i8 > 0xf && read_u8(s + 0x57) as i8 > 0xa {
                let a = cpw7_sortear(read_u8(s + 0x5b) as i8 as i32);
                let b = cpw7_sortear(0x1e);
                let f = read_u8(ct + 0x4f) & 0xf0;
                if a > b && (f == 0x10 || f == 0x20) && read_u8(s + 0x18) as i8 > 0x17 && read_i16(read_usize(s + 0x61) + 0xb) > 0x1482 {
                    pedir(ct, 2);
                }
            }
            if read_u8(s + 0x18) as i8 >= 0x15 || read_i16(read_usize(s + 0x61) + 0xb) <= 0x1482 {
                continue;
            }
            cpw7_cair(s + 0x5b, true);
            let j = read_usize(s + 0x61);
            if read_u8(j + 0x44) as i8 > 1 && cpw7_sortear(read_u8(s + 0x5b) as i8 as i32) == 0 {
                *((j + 0x44) as *mut i8) -= 1;
            }
            cpw7_cair(s + 0x5d, true);
        }
    }
    // o faltoso
    let j = read_usize(pessoa + 0x61);
    if j != 0 {
        let a = cpw7_sortear(read_u8(pessoa + 0x5b) as i8 as i32);
        let b = cpw7_sortear(0xf);
        if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
            cpw7_cair(pessoa + 0x5b, false);
            cpw7_cair(read_usize(pessoa + 0x61) + 0x44, false);
            cpw7_tirar_falta(this, r);
            cpw7_faltou(this, pessoa);
            if clube != 0 && read_usize(pessoa + 0x61) != 0 {
                let brigar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_MEDIA_CREATE_PLAYER_MEDIA_STORY);
                brigar(read_usize(0xb6b7c8), pessoa, clube, 8, 0, 0);
            }
            return;
        }
    }
    cpw7_tirar_falta(this, r);
}

// ponytail: sorteia, tira e libera nós das listas (0x68d1f0), dá notícia de volta (0x419710) e de falta (0x41a360): só
// a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_faltas_do_dia(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1308)(this)
}

/// { 0x418ff0 } (thiscall, ret 4, awol.cpp): `AWOL::has_missed_training(pessoa)`, pelos jogadores do dia (0x5a69b0) e
/// pelas telas. Pessoa nula ou fora da conta [0xae2364]: falso. Com o bit 2 da marca dela ([this+0xc] + id) e sem
/// contrato (0x4cf5a0, guardado em [0xae1080]), tira a falta (0x4197c0) antes. Devolve o bit 2 da marca.
unsafe extern "thiscall" fn teve_falta_ao_treino(this: usize, pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let id = cpw7_i(pessoa);
    if id >= cpw7_i(0xae2364) {
        return id as u32 & 0xffffff00;
    }
    if read_u8(read_usize(this + 0xc).wrapping_add(id as usize)) & 2 != 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let k = contrato_de(read_usize(0xae1050), pessoa);
        write_u32(0xae1080, k as u32);
        if k == 0 || read_usize(0xae23b4).wrapping_add(read_usize(k + 4).wrapping_mul(0x245)) == 0 {
            let limpar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_CLEAR_MISSED_TRAINING);
            limpar(this, pessoa);
        }
    }
    (read_u8(read_usize(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize)) & 2) as u32
}

/// O comparador da falta ao treino: lado a lado pela devolução (byte baixo) e pelo [0xae1080]; sem contrato com o bit 2,
/// só a original (a 0x4197c0 tira e libera o nó da lista).
unsafe extern "thiscall" fn comparar_teve_falta_ao_treino(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1309);
    if pessoa != 0 && cpw7_i(pessoa) < cpw7_i(0xae2364) && read_u8(read_usize(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize)) & 2 != 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let k = contrato_de(read_usize(0xae1050), pessoa);
        if k == 0 || read_usize(0xae23b4).wrapping_add(read_usize(k + 4).wrapping_mul(0x245)) == 0 {
            // ponytail: tira a falta da lista (libera o nó): só a original
            return o(this, pessoa);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(this, pessoa), || teve_falta_ao_treino(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("teve_falta_ao_treino(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// { 0x419070 } (thiscall, ret 4, awol.cpp): `AWOL::has_fitness_incident(pessoa)`, pelos jogadores do dia (0x5a69b0).
/// Pessoa nula, fora da conta [0xae2364] ou sem a lista dos atrasos ([this+8]): falso; senão percorre a lista (cursor em
/// +0xa) atrás do registro da pessoa (id em +0). Verdadeiro no byte baixo; os altos como a original deixa no registrador.
unsafe extern "thiscall" fn teve_incidente_de_forma(this: usize, pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let id = cpw7_i(pessoa);
    if id >= cpw7_i(0xae2364) {
        return id as u32 & 0xffffff00;
    }
    let l = read_usize(this + 8);
    if l == 0 {
        return 0;
    }
    cpw7_inicio(l);
    if read_i16(l) <= 0 {
        return l as u32 & 0xffffff00;
    }
    let mut i = 0;
    loop {
        let r = cpw7_proximo(read_usize(this + 8));
        if r == 0 {
            return 0;
        }
        if cpw7_i(r) == cpw7_i(pessoa) {
            return (cpw7_i(pessoa) as u32 & 0xffffff00) | 1;
        }
        i += 1;
        let n = read_i16(read_usize(this + 8)) as i32;
        if i >= n {
            return n as u32 & 0xffffff00;
        }
    }
}

/// O comparador do atraso: lado a lado pela devolução (byte baixo) e pelo cursor da lista.
unsafe extern "thiscall" fn comparar_teve_incidente_de_forma(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1310);
    let l = read_usize(this + 8);
    let regioes: Vec<(usize, usize)> = if l != 0 { vec![(l + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa), || teve_incidente_de_forma(this, pessoa));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("teve_incidente_de_forma(pessoa {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", pessoa, r, mo, n, mr));
    r
}

// ---- as originais que a faltas_do_dia chama e que liam resíduo da pilha particular (bancada da 259ª) ----

/// O ensaio dos comparadores de `marcar_falta_ao_treino`, `fim_da_ausencia` e `marcas_do_contrato`: a cópia para antes
/// das chamadas que alocam, dão notícia ou não podem rodar duas vezes e deixa em CPW7_DECISAO o que faria.
static mut CPW7_ENSAIO: bool = false;
static mut CPW7_DECISAO: u32 = 0;

/// O clube do contrato `k` (+4 id, [0xae23b4], 0x245 bytes), ou 0.
unsafe fn cpw7_clube_do_contrato(k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    read_usize(0xae23b4).wrapping_add(read_usize(k + 4).wrapping_mul(0x245))
}

/// O byte +0xc (caso repetido) do registro da falta. A 0x41a360 não o grava: o registro de 13 bytes é montado num local
/// e o +0xc fica com o que a chamada anterior deixou no dword E−0x10 da pilha (E: o esp na entrada). No caminho normal
/// da original, pela faltas_ao_treino (0x5d6aa0), a chamada anterior que alcança essa altura é a 0x54cdb0 (dias desde a
/// chegada), que guarda ali o seu `this`, a data do jogo 0xae2c90: o byte baixo é 0x90. A cópia usa esse valor (um caso
/// repetido); o comparador não o compara (o da original é resíduo de verdade).
const CPW7_BYTE_DO_CASO: u8 = 0x90;

/// { 0x41a360 } (thiscall, ret 4, awol.cpp, SEH só com a data local, sem destrutor): `AWOL::miss_training(pessoa)`,
/// pelas faltas ao treino do dia (0x5d6aa0) e pela faltas_do_dia. Jogador (+0x61) dentro da conta, com contrato
/// (0x4cf5a0, em [0xae1080]) num clube e sem a marca 2: contrato com moral (+0x44) até −100 e o bit 0x2000 (depois o
/// 0x800) em +0x45, com sorteio(profissionalismo + 10) e sorteio(+0x59 + 10) zero, some (0x419840 com 0, 0). Senão
/// marca o bit 2, notícia da falta (0x41a630, com o clube do contrato), o byte +0xa da linha de 79 bytes ([0xae2d0c])
/// soma sorteio(10), a condição (0x62c590) cai 25 × (21 − o +0x34 do jogador) + sorteio + 100 (0x62bd10), o registro
/// (pessoa, hoje, caso) entra na lista das faltas ([this+4], 0x68d0d0); clube da IA (0x600a50) com técnico (+0xcf, +0x69)
/// e sorteio(+0x14 dele) não zero: a multa (0x5a5980, prazo 1 se sorteio(nota do técnico 0x544410 com o +0x19) passar
/// de sorteio(5), senão 2; motivo 1).
unsafe extern "thiscall" fn marcar_falta_ao_treino(this: usize, pessoa: usize) {
    if pessoa == 0 || read_usize(pessoa + 0x61) == 0 || cpw7_i(pessoa) >= cpw7_i(0xae2364) {
        return;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let k = contrato_de(read_usize(0xae1050), pessoa);
    write_u32(0xae1080, k as u32);
    let clube = cpw7_clube_do_contrato(k);
    if k == 0 || clube == 0 {
        return;
    }
    let id = cpw7_i(pessoa);
    let m = read_usize(this + 0xc).wrapping_add(id as usize) as *mut u8;
    if *m & 2 != 0 {
        return;
    }
    let k2 = contrato_de(read_usize(0xae1050), pessoa);
    if k2 != 0 && read_u8(k2 + 0x44) as i8 <= -100 {
        for bit in [0x2000u32, 0x800] {
            if read_usize(k2 + 0x45) as u32 & bit != 0
                && cpw7_sortear(read_u8(pessoa + 0x5b) as i8 as i32 + 10) == 0
                && cpw7_sortear(read_u8(pessoa + 0x59) as i8 as i32 + 10) == 0
            {
                crate::comparacao::ramo("cpw_7 sumiu em vez de faltar");
                if CPW7_ENSAIO {
                    CPW7_DECISAO = 1;
                    return;
                }
                let sumir: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_START_ABSENCE);
                sumir(this, pessoa, 0, 0);
                return;
            }
        }
    }
    crate::comparacao::ramo("cpw_7 marcou a falta");
    if CPW7_ENSAIO {
        CPW7_DECISAO = 2;
        return;
    }
    *m |= 2;
    let k3 = contrato_de(read_usize(0xae1050), pessoa);
    write_u32(0xae1080, k3 as u32);
    let noticia: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(FN_AWOL_POST_MISSED_TRAINING_NEWS);
    noticia(pessoa, cpw7_clube_do_contrato(k3));
    let linha = read_usize(0xae2d0c).wrapping_add((cpw7_i(pessoa) as usize).wrapping_mul(79)).wrapping_add(0xa) as *mut u8;
    let r = cpw7_sortear(10);
    *linha = (*linha).wrapping_add(r as u8);
    let condicao: unsafe extern "thiscall" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_INJURY_GET_CONDITION);
    let c = condicao(read_usize(0xb63d28), pessoa, 0);
    // as contas da original são de 32 bits com bytes altos de outros valores no registrador, mas só a palavra baixa
    // chega (a condição é short na 0x62bd10)
    let queda = 0x15i16.wrapping_sub(read_u8(read_usize(pessoa + 0x61) + 0x34) as i8 as i16).wrapping_mul(25);
    let r = cpw7_sortear(queda as i32);
    let nova = (c as u16).wrapping_sub((queda as u16).wrapping_add(r as u16).wrapping_add(0x64));
    let gravar_condicao: unsafe extern "thiscall" fn(usize, usize, i16, i16) = core::mem::transmute(FN_INJURY_SET_MATCH_CONDITION);
    gravar_condicao(read_usize(0xb63d28), pessoa, nova as i16, 0);
    let mut reg = [0u8; 13];
    reg[..4].copy_from_slice(&cpw7_i(pessoa).to_le_bytes());
    reg[4..12].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    reg[12] = CPW7_BYTE_DO_CASO;
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    acrescentar(read_usize(this + 4), reg.as_ptr() as usize, 0xd);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    let tecnico = read_usize(clube + 0xcf);
    if tecnico == 0 {
        return;
    }
    let np = read_usize(tecnico + 0x69);
    if np == 0 || cpw7_sortear(read_u8(np + 0x14) as i8 as i32) == 0 {
        return;
    }
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let a = cpw7_sortear(nota(np, read_u8(np + 0x19) as u32) as i32);
    let b = cpw7_sortear(5);
    crate::comparacao::ramo("cpw_7 multa pela falta");
    let multar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_FINE_FINE_PLAYER);
    multar(read_usize(0xae2a3c), pessoa, if a > b { 1 } else { 2 }, 1);
}

/// O comparador da falta marcada, em ensaio: a cópia para na decisão (0 nada, 1 some, 2 marca a falta) e a original roda
/// a partir do mesmo estado (sorteio, marca da pessoa, [0xae1080]); a decisão da original sai da marca (bit 1 novo:
/// sumiu; bit 2 novo: faltou). Sem decisão, o estado inteiro tem de bater (os mesmos sorteios). O byte do caso (+0xc do
/// registro) não entra: na original é resíduo da pilha.
unsafe extern "thiscall" fn comparar_marcar_falta_ao_treino(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1311);
    if pessoa == 0 || read_usize(pessoa + 0x61) == 0 || cpw7_i(pessoa) >= cpw7_i(0xae2364) {
        return o(this, pessoa);
    }
    let m = read_usize(this + 0xc).wrapping_add(cpw7_i(pessoa) as usize);
    let mut regioes = vec![(m, 1), (0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| marcar_falta_ao_treino(this, pessoa)));
    CPW7_ENSAIO = false;
    let n = CPW7_DECISAO;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let b0 = read_u8(m);
    o(this, pessoa);
    let novo = read_u8(m) & !b0;
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let r = if novo & 2 != 0 { 2 } else if novo & 1 != 0 { 1 } else { 0 };
    let igual = (r == n || (n == 1 && b0 & 1 != 0)) && (n != 0 || mo == mr);
    PLACAR.registrar(igual, || format!("marcar_falta_ao_treino(pessoa {:#x}) original={} {:x?} rust={} {:x?}", pessoa, r, mo, n, mr));
}

/// { 0x419710 } (thiscall, ret 4, awol.cpp): `AWOL::end_absence(registro)`, pela faltas_do_dia. Registro com a marca
/// da pessoa (+0 id; [this+0xc] + id) não zero: com contrato (0x4cf5a0, em [0xae1080]) num clube, a notícia da volta
/// (0x419ad0 com o clube +0x39 da pessoa, 0 e −1); troca o bit 1 da marca (o `or 1` da original nunca dá zero) e tira o
/// registro da lista das ausências ([this], 0x68d1f0). Não lê local sem valor.
unsafe extern "thiscall" fn fim_da_ausencia(this: usize, r: usize) {
    if r == 0 {
        return;
    }
    let m = read_usize(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_7 fim da ausencia");
    let pessoa = read_usize(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let k = contrato_de(read_usize(0xae1050), pessoa);
    write_u32(0xae1080, k as u32);
    if k != 0 && cpw7_clube_do_contrato(k) != 0 {
        if CPW7_ENSAIO {
            CPW7_DECISAO = 1;
        } else {
            let p = read_usize(0xae23bc).wrapping_add((cpw7_i(r) as usize).wrapping_mul(0x6e));
            let noticia: unsafe extern "stdcall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_POST_NEWS);
            noticia(p, read_usize(p + 0x39), 0, 0xffffffff);
        }
    }
    let m = read_usize(this + 0xc).wrapping_add(cpw7_i(r) as usize) as *mut u8;
    *m ^= 1;
    if CPW7_ENSAIO {
        return;
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    tirar(read_usize(this), r);
}

/// O comparador do fim da ausência, em ensaio: a cópia sem a notícia e sem tirar da lista; a marca da pessoa e, sem
/// notícia, o [0xae1080] têm de bater com os da original.
unsafe extern "thiscall" fn comparar_fim_da_ausencia(this: usize, r: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1312);
    if r == 0 {
        return o(this, r);
    }
    let m = read_usize(this + 0xc).wrapping_add(cpw7_i(r) as usize);
    let regioes = [(m, 1), (0xae1080, 4)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| fim_da_ausencia(this, r));
    CPW7_ENSAIO = false;
    let noticia = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, r);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = mo.0[0] == mr.0[0] && (noticia || mo.0[1] == mr.0[1]);
    PLACAR.registrar(igual, || format!("fim_da_ausencia(marca {:#x}) original={:x?} rust={:x?}", m, mo, mr));
}

/// { 0x867560 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_flags(marcas)`, chamada por muitas rotinas
/// (no motor.rs pelo endereço; antes só a porta do pós-jogo a desviava, no modo comparação). Marcas acima de 0x1000: a
/// caixa "Error" e segue (aqui: a original inteira). Liga as marcas em +0x40; pessoa (+0 id) que não é das funções
/// 0xc, 0xd e 0xf, contrato com +0x44 positivo, sem +0x49, que quer sair (0x866f60), sem transferência futura
/// (0x8cca10 com 0), sem os bits 1 e 8 de +0x4e, fora da fila da transferência livre (0x8c1870) e com mais de 20 anos
/// (+0x18) ou reputação atual (+0xb do jogador) acima de 4250: o pedido de transferência (0x4d8610 em [0xae1050]).
/// Não lê local sem valor.
unsafe extern "thiscall" fn marcas_do_contrato(c: usize, marcas: u32) {
    if marcas as i32 > 0x1000 {
        // ponytail: a caixa "Error" (antes de qualquer gravação): a original inteira
        return orig::<unsafe extern "thiscall" fn(usize, u32)>(1313)(c, marcas);
    }
    write_u32(c + 0x40, read_usize(c + 0x40) as u32 | marcas);
    let pessoa = read_usize(0xae23bc).wrapping_add((cpw7_i(c) as usize).wrapping_mul(0x6e));
    let funcao = read_u8(pessoa + 0x3d);
    if funcao == 0xc || funcao == 0xd || funcao == 0xf {
        return;
    }
    let moral = read_u8(c + 0x44) as i8;
    if moral == 0 || moral < 0 || read_u8(c + 0x49) != 0 {
        return;
    }
    let quer_sair: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    if quer_sair(c) == 0 {
        return;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if futura(read_usize(0xde1f54), pessoa, 0) != 0 || read_u8(c + 0x4e) & 9 != 0 {
        return;
    }
    let na_fila: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_QUEUED_FOR_FREE_TRANSFER);
    if na_fila(read_usize(0xde1f54), pessoa) != 0 {
        return;
    }
    if read_u8(pessoa + 0x18) as i8 <= 0x14 && read_i16(read_usize(pessoa + 0x61) + 0xb) <= 0x109a {
        return;
    }
    crate::comparacao::ramo("cpw_7 pedido de saida");
    if CPW7_ENSAIO {
        CPW7_DECISAO = 1;
        return;
    }
    let pedido: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_HANDLE_TRANSFER_REQUEST);
    pedido(read_usize(0xae1050), c);
}

/// O comparador das marcas do contrato, em ensaio: a cópia sem o pedido de transferência e a original a partir do mesmo
/// estado (marcas e sorteio); sem pedido, tudo tem de bater; com pedido (que sorteia e dá notícia), só as marcas.
unsafe extern "thiscall" fn comparar_marcas_do_contrato(c: usize, marcas: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1313);
    if marcas as i32 > 0x1000 {
        return o(c, marcas);
    }
    let mut regioes = vec![(c + 0x40, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| marcas_do_contrato(c, marcas)));
    CPW7_ENSAIO = false;
    let pediu = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(c, marcas);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if pediu { mo.0[0] == mr.0[0] } else { mo == mr };
    PLACAR.registrar(igual, || format!("marcas_do_contrato({:#x}, {:#x}) pediu={} original={:x?} rust={:x?}", c, marcas, pediu, mo, mr));
}

