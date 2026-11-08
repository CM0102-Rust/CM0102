// ---------- %%LEVA%%a leva: o fim da falta ao treino e da ausência sem licença e a notícia da falta ao treino (awol.cpp) ----------

unsafe fn cqd11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os itens da lista (o ponteiro em +0 de cada nó, da cabeça +2 seguindo o +6), sem mexer no cursor.
unsafe fn llist_items(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqd11_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqd11_d(no));
        no = cqd11_d(no + 6);
    }
    v
}

/// O corpo comum de { 0x4196a0 } e { 0x4197c0 }: com a marca `bit` da pessoa ([this+0xc] + id), a marca troca (o
/// `or bit` da original nunca dá zero) e, com a lista ([this + `lista`]), o cursor volta à cabeça e, entre os primeiros
/// `conta` (+0, palavra com sinal) itens, o primeiro com o id da pessoa em +0 sai (0x68d1f0, que libera o nó). Uma lista
/// que acaba antes da conta encerra.
unsafe fn toggle_mark_and_unlist(this: usize, pessoa: usize, bit: u8, lista: usize, falta: bool) {
    if pessoa == 0 {
        return;
    }
    let marcas = cqd11_d(this + 0xc);
    if marcas == 0 {
        return;
    }
    let p = marcas.wrapping_add(cqd11_d(pessoa));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return;
    }
    if falta {
        crate::comparison::branch("cqd_11 falta tirada");
    }
    *(p as *mut u8) = m ^ bit;
    let l = cqd11_d(this + lista);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqd11_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    // 0x68d1b0: mov ecx, [edi + lista] -> thiscall; 0x68d1f0: push item; mov ecx, [edi + lista]; ret 4 -> thiscall
    let mut i: i32 = 0;
    if cqd11_w(l) as i32 <= 0 {
        return;
    }
    loop {
        let item = proximo(cqd11_d(this + lista));
        if item == 0 {
            return;
        }
        if cqd11_d(item) == cqd11_d(pessoa) {
            if falta {
                crate::comparison::branch("cqd_11 registro da falta tirado");
            }
            tirar(cqd11_d(this + lista), item);
            return;
        }
        i += 1;
        if i >= cqd11_w(cqd11_d(this + lista)) as i32 {
            return;
        }
    }
}

/// { 0x4196a0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_awol(pessoa)`, pelo fim da ausência sem contrato (0x418f60),
/// pelas aposentadorias (0x7a59e0) e pelas transferências (0x8c1af0, 0x8c2180, 0x8c9720). Pessoa e tabela de marcas
/// ([this+0xc]) não nulas e o bit 1 da marca: troca o bit e tira o registro da pessoa da lista das ausências ([this]).
unsafe extern "thiscall" fn clear_awol(this: usize, pessoa: usize) {
    toggle_mark_and_unlist(this, pessoa, 1, 0, false);
}

/// { 0x4197c0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_missed_training(pessoa)`, pela falta ao treino de quem ficou
/// sem contrato (0x418ff0, que os jogadores do dia consultam) e pelas aposentadorias (0x7a59e0). Pessoa e tabela de
/// marcas não nulas e o bit 2 da marca: troca o bit e tira o registro da pessoa da lista das faltas ([this+4]).
unsafe extern "thiscall" fn clear_missed_training(this: usize, pessoa: usize) {
    toggle_mark_and_unlist(this, pessoa, 2, 4, true);
}

/// O plano das duas: a marca depois e os itens que ficam na lista (o primeiro com o id da pessoa entre os `conta`
/// primeiros sai). Sem nada a fazer: o estado de antes.
unsafe fn plan_toggle_mark_and_unlist(this: usize, pessoa: usize, bit: u8, lista: usize) -> (Option<(usize, u8)>, usize, Vec<usize>) {
    let l = cqd11_d(this + lista);
    let itens = llist_items(l);
    if pessoa == 0 || cqd11_d(this + 0xc) == 0 {
        return (None, l, itens);
    }
    let p = cqd11_d(this + 0xc).wrapping_add(cqd11_d(pessoa));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return (Some((p, m)), l, itens);
    }
    if l == 0 {
        return (Some((p, m ^ bit)), l, itens);
    }
    let conta = (cqd11_w(l) as i32).max(0) as usize;
    let mut fica = itens.clone();
    if let Some(k) = itens.iter().take(conta).position(|&x| cqd11_d(x) == cqd11_d(pessoa)) {
        fica.remove(k);
    }
    (Some((p, m ^ bit)), l, fica)
}

/// Comparador das duas: a original tira e libera o nó da lista (não pode rodar duas vezes): só ela roda, com o plano
/// montado antes (a marca e os itens que ficam, sem ler o item tirado depois).
unsafe fn compare_toggle_mark_and_unlist(this: usize, pessoa: usize, bit: u8, lista: usize, o: unsafe extern "thiscall" fn(usize, usize), nome: &str) {
    let (marca, l, fica) = plan_toggle_mark_and_unlist(this, pessoa, bit, lista);
    o(this, pessoa);
    let depois = llist_items(l);
    let marca_ok = marca.map_or(true, |(p, m)| *(p as *const u8) == m);
    SCOREBOARD.record(marca_ok && depois == fica, || format!("{}(pessoa {:#x}) marca plano {:x?} agora {:x?}; lista plano {:x?} agora {:x?}",
        nome, pessoa, marca, marca.map(|(p, _)| *(p as *const u8)), fica, depois));
}

unsafe extern "thiscall" fn compare_clear_awol(this: usize, pessoa: usize) {
    compare_toggle_mark_and_unlist(this, pessoa, 1, 0, original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%), "clear_awol");
}

unsafe extern "thiscall" fn compare_clear_missed_training(this: usize, pessoa: usize) {
    compare_toggle_mark_and_unlist(this, pessoa, 2, 4, original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%), "clear_missed_training");
}

/// O ensaio da notícia: os sorteios vêm da fita da original (n e resposta) e, em vez de entregar, a cópia anota o
/// destino (1 o clube do jogador, 2 a nação do clube, 0 nenhum). A fita fica com o próximo e se algum n não bateu.
static mut AWOL_NEWS_TAPE: Option<(Vec<(i32, i32)>, usize, bool)> = None;
static mut AWOL_NEWS_DESTINATION: (u32, usize) = (0, 0);

unsafe fn taped_random(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE);
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

/// { 0x41a630 } (thiscall, ret 8, awol.cpp, SEH da notícia local, sem destrutor): `awol_post_missed_training_news(jogador,
/// clube)`, pela falta ao treino (0x41a360). Jogador com ficha (+0x61) e clube com técnico (+0xcf): a notícia 0xbc9
/// (0x77bbe0) com o jogador (id e as chaves +0x33 de +4, +8, +0xc), o id do clube, sorteio(3), o clube de humano que
/// decide (0x600a50 com 0, 1) e quem relata: o técnico, se o clube tem técnico e não é de humano (0x600a50 com 1, 1;
/// id e chaves); senão, com sorteio(2), um dos 5 treinadores (+0x19f, sorteio(5) até achar ou mil voltas), sem
/// sorteio o auxiliar (+0xd3); sem ninguém (ou id −1), −1 nos quatro. Clube de humano (0x7875e0): ao clube do jogador
/// (+0x39, 0x7861e0); senão, com nação (+0x53) e reputação do jogador (+0x61, +0xb) acima de 7250, às caixas da nação
/// (0x7863f0).
unsafe extern "thiscall" fn awol_post_missed_training_news(this: usize, jogador: usize, clube: usize) {
    let _ = this;
    if jogador == 0 || cqd11_d(jogador + 0x61) == 0 || clube == 0 || cqd11_d(clube + 0xcf) == 0 { // cqd11: sem jogador ou técnico
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    // 0x77bbe0: push 0; push 0xbc9; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall; 0x600a50: 5
    // pushes; add esp, 0x14 -> cdecl (clube, 0, 1, 0, 0) e (clube, 1, 1, 0, 0)
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbc9, 0);
    gravar(nt, 0, cqd11_d(jogador) as u32);
    gravar(nt, 1, cqd11_d(cqd11_d(jogador + 4) + 0x33) as u32);
    gravar(nt, 2, cqd11_d(cqd11_d(jogador + 8) + 0x33) as u32);
    gravar(nt, 3, cqd11_d(cqd11_d(jogador + 0xc) + 0x33) as u32);
    gravar(nt, 4, cqd11_d(clube) as u32);
    crate::comparison::branch("cqd_11 noticia da falta");
    let r = taped_random(3); // cqd11: o sorteio da notícia
    gravar(nt, 5, r as u32);
    gravar(nt, 6, humano(clube, 0, 1, 0, 0));
    let tecnico = cqd11_d(clube + 0xcf);
    if tecnico != 0 && humano(clube, 1, 1, 0, 0) == 0 { // cqd11: o técnico da IA relata
        crate::comparison::branch("cqd_11 relator tecnico");
        let t = cqd11_d(clube + 0xcf);
        gravar(nt, 7, cqd11_d(t) as u32);
        gravar(nt, 8, cqd11_d(cqd11_d(t + 4) + 0x33) as u32);
        gravar(nt, 9, cqd11_d(cqd11_d(t + 8) + 0x33) as u32);
        gravar(nt, 0xa, cqd11_d(cqd11_d(t + 0xc) + 0x33) as u32);
    } else {
        let quem = if taped_random(2) != 0 {
            let mut k;
            let mut voltas = 0;
            loop {
                k = taped_random(5);
                voltas += 1;
                if voltas >= 1000 || cqd11_d(clube.wrapping_add(k as usize * 4) + 0x19f) != 0 {
                    break;
                }
            }
            cqd11_d(clube.wrapping_add(k as usize * 4) + 0x19f)
        } else {
            cqd11_d(clube + 0xd3)
        };
        let id = if quem != 0 { cqd11_d(quem) as i32 } else { -1 };
        if id == -1 {
            for k in 7..=0xa {
                gravar(nt, k, u32::MAX);
            }
        } else {
            let p = cqd11_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            gravar(nt, 7, id as u32);
            gravar(nt, 8, cqd11_d(cqd11_d(p + 4) + 0x33) as u32);
            gravar(nt, 9, cqd11_d(cqd11_d(p + 8) + 0x33) as u32);
            gravar(nt, 0xa, cqd11_d(cqd11_d(p + 0xc) + 0x33) as u32);
        }
    }
    let noticias = cqd11_d(0xb7431c);
    let de_humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    // 0x7875e0: mov ecx, [0xb7431c]; push clube (ret 4); 0x7861e0: push clube +0x39; push notícia; mov ecx, [0xb7431c];
    // 0x7863f0: push nação; push notícia; mov ecx, [0xb7431c] -> thiscall nas três
    let ensaio = (*core::ptr::addr_of!(AWOL_NEWS_TAPE)).is_some();
    if de_humano(noticias, clube) != 0 {
        if ensaio {
            AWOL_NEWS_DESTINATION = (1, cqd11_d(jogador + 0x39));
        } else {
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
            entregar(noticias, nt, cqd11_d(jogador + 0x39));
        }
        return;
    }
    let nacao = cqd11_d(clube + 0x53);
    if nacao == 0 || cqd11_w(cqd11_d(jogador + 0x61) + 0xb) <= 0x1c52 {
        return;
    }
    if ensaio {
        AWOL_NEWS_DESTINATION = (2, nacao);
    } else {
        let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
        a_nacao(noticias, nt, nacao);
    }
}

/// Comparador: a original monta a notícia e a entrega (aloca na caixa): só ela roda, com os sorteios rastreados; depois
/// a cópia em ensaio refaz o caminho com os sorteios da fita, sem entregar. Confere que os sorteios do ensaio são o
/// começo da fita da original (o resto é da entrega) com os mesmos n, e anota o destino (a original não o expõe).
unsafe extern "thiscall" fn compare_awol_post_missed_training_news(this: usize, jogador: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    let ((), tape_lock) = crate::rng::trace(|| o(this, jogador, clube));
    let n = tape_lock.len();
    *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE) = Some((tape_lock, 0, false));
    AWOL_NEWS_DESTINATION = (0, 0);
    crate::rng::without_counting(|| awol_post_missed_training_news(this, jogador, clube));
    let (k, falhou) = match &*core::ptr::addr_of!(AWOL_NEWS_TAPE) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE) = None;
    let destino = AWOL_NEWS_DESTINATION;
    SCOREBOARD.record(!falhou && k <= n, || format!("awol_post_missed_training_news(jogador {:#x}, clube {:#x}) tape_lock de {} sorteios, o ensaio usou {} (n diferente: {}), destino {:x?}",
        jogador, clube, n, k, falhou, destino));
}
