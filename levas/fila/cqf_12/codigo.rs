// ---------- %%LEVA%%a leva: os olheiros do dia, os relatórios deles e a notícia do relatório (scout_manager.cpp) ----------

unsafe fn cqf12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqf12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf12_pessoa(id: usize) -> usize {
    cqf12_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf12_clube(id: usize) -> usize {
    cqf12_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// `a` depois de `b` (datas: ano sem sinal em +2, dia com sinal em +0), como as comparações da original.
unsafe fn scout_manager_date_after(dia: i16, ano: u16, data: usize) -> bool {
    let (d, a) = (cqf12_w(data), cqf12_w(data + 2) as u16);
    ano > a || (ano == a && dia > d)
}

/// O ensaio da 0x7f8750: a cópia só poda os pedidos (num vetor dela) e para antes dos relatórios.
static mut SCOUT_DAILY_DRY_RUN: bool = false;

/// A poda de um pedido (0x11 bytes: +0 o clube, +4 o jogador, +8 a data, +0x10 quantos relatórios): devolve
/// (tirar, processar). Sai o pedido de clube que não é humano, de jogador do próprio clube (o +0x39 ou o do contrato),
/// com mais de um ano, com 3 relatórios, ou (depois de 30 dias) de jogador machucado (0x62c400 = 2); entre 30 dias e um
/// ano o pedido é processado (e sai), salvo jogador sem nação ou de clube que joga nos 14 dias seguintes, que fica.
unsafe fn judge_scout_request(pedido: usize, catorze: usize) -> (bool, bool, usize, usize, usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> i16 = core::mem::transmute(0x5acbe0usize);
    // 0x600a50: 5 pushes (clube, 1, 1, 0, 0); add esp, 0x14 -> cdecl; 0x4cf5a0: push jogador; mov ecx, [0xae1050] ->
    // thiscall, ret 4; 0x54c5b0: push n; push &saída; lea ecx, [pedido+8] -> thiscall, ret 8; 0x62c400: push jogador;
    // mov ecx, [0xb63d28] -> thiscall, ret 4 (al); 0x4cf5f0: push clube; push jogador; mov ecx, [0xae1050] -> thiscall,
    // ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24 (ax)
    let jogador = cqf12_pessoa(cqf12_d(pedido + 4));
    let clube = cqf12_clube(cqf12_d(pedido));
    if humano(clube, 1, 1, 0, 0) == 0 {
        return (true, false, 0, 0, 0);
    }
    let tecnico = cqf12_d(clube + 0xcf);
    if cqf12_d(jogador + 0x39) == clube {
        return (true, false, 0, 0, 0);
    }
    let c = contract_of(cqf12_d(0xae1050), jogador);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    let do_contrato = if c != 0 { cqf12_clube(cqf12_d(c + 4)) } else { 0 };
    if do_contrato == clube {
        return (true, false, 0, 0, 0);
    }
    let (dia, ano) = (cqf12_w(0xae2c90), cqf12_w(0xae2c92) as u16);
    let mut t = [0u32; 2];
    let ano_depois = mais(pedido + 8, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano
    if scout_manager_date_after(dia, ano, ano_depois) {
        return (true, false, 0, 0, 0);
    }
    if (cqf12_b(pedido + 0x10) as i8) >= 3 { // cqf12: três relatórios
        return (true, false, 0, 0, 0);
    }
    let mut t2 = [0u32; 2];
    let mes_depois = mais(pedido + 8, t2.as_mut_ptr() as usize, 0x1e); // cqf12: trinta dias
    if !scout_manager_date_after(dia, ano, mes_depois) {
        return (false, false, 0, 0, 0);
    }
    if lesao(cqf12_d(0xb63d28), jogador) == 2 {
        return (true, false, 0, 0, 0);
    }
    let seu = cqf12_d(jogador + 0x39);
    let mut nacao = if seu != 0 { cqf12_d(seu + 0x53) } else { 0 };
    if nacao == 0 {
        nacao = cqf12_d(jogador + 0x1a);
        if nacao == 0 {
            return (false, false, 0, 0, 0);
        }
    }
    let mut direto = false;
    if seu != 0 {
        let k = contrato_com(cqf12_d(0xae1050), jogador, seu);
        if k != 0 {
            let s = cqf12_b(k + 0x4f) & 0xf0;
            direto = s == 0x60 || s == 0x70 || s == 0;
        }
    }
    if !direto && seu != 0 {
        // o clube do jogador joga de hoje até 14 dias depois? (sem lista; a parte alta do 2º dword é sobra da pilha na
        // original e a 0x5acbe0 só lê a palavra baixa)
        let ultimo = cqf12_d(catorze) as u32;
        let ultimo_alto = core::ptr::read_unaligned((catorze + 2) as *const u32);
        let a1 = (ultimo_alto & 0xffff_0000) | dia as u16 as u32;
        let a2 = ano as u32;
        if achar(cqf12_d(0xae2a58), a1, a2, ultimo, ultimo_alto, cqf12_d(seu) as u32, 0, -1, -2, 1) != 0 {
            return (false, false, 0, 0, 0);
        }
    }
    (true, true, jogador, tecnico, nacao)
}

/// { 0x7f8750 } (thiscall, ret 0, scout_manager.cpp, quadro SEH da lista local e das datas): `SCOUT_MANAGER::
/// daily_update()`, pelo dia (só na fatia 2 do dia, [0xae2c88]; senão 0). Poda os pedidos de observação (+0x5c,
/// conta +0x64, 0x11 bytes; o tirado recebe o último), e o pedido processado vira até 3 relatórios dos olheiros do
/// técnico do clube (das 10 listas, 0x7f6cd0, a que cobre a nação do jogador, 0x7b7490): 0x7b62a0 com −1000, modo 1, e o
/// registro (0x67 bytes) entra na lista local se o byte +0x66 ficou ≥ 0. Depois os relatórios (0x7f9890), a ordem dos
/// pedidos (0x945333 com 0x7fc270) e uma busca (0x7b3e20) em cada uma das 10 listas dos técnicos humanos (as 16 últimas
/// pessoas com +0x5f = 1). Devolve 1.
unsafe extern "thiscall" fn scout_manager_daily_update(this: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // 0x68d020 / 0x68d040 / 0x54a110: lea ecx, local -> thiscall; 0x54c5b0: push 0xe; push &saída; mov ecx, 0xae2c90
    let mut d14 = [0u32; 2];
    let catorze = d14.as_mut_ptr() as usize;
    mais(0xae2c90, catorze, 0xe);
    // o registro do relatório (0x67 bytes) com as datas que a entrada da original constrói (+0, +0x2f, +0x47)
    let mut registro = [0u8; 0x68];
    let reg = registro.as_mut_ptr() as usize;
    data_padrao(reg);
    data_padrao(reg + 0x2f);
    data_padrao(reg + 0x47);
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    if cqf12_b(0xae2c88) != 2 {
        soltar(l);
        return 0;
    }
    let registro_do_olheiro: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let cobre: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7b7490usize);
    let relatorio: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, usize, usize, u32) = core::mem::transmute(0x7b62a0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let relatorios: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f9890usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let buscar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7b3e20usize);
    // 0x7f6cd0: push técnico; push k; mov ecx, this -> thiscall, ret 8; 0x7b7490: push nação; mov ecx, lista ->
    // thiscall, ret 4; 0x7b62a0: push 1; push 0; push 0; push 1; push −1000; push &registro; push jogador; mov ecx, lista
    // -> thiscall, ret 0x1c; 0x68d0d0: push 0x67; push &registro; lea ecx, local -> thiscall, ret 8; 0x7f9890: push
    // &local; mov ecx, this -> thiscall, ret 4; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x7b3e20: mov ecx,
    // lista -> thiscall, ret 0
    let mut i: i32 = 0;
    while i < cqf12_i(this + 0x64) {
        let pedido = cqf12_d(this + 0x5c) + i as usize * 0x11;
        let (tirar, processar, jogador, tecnico, nacao) = judge_scout_request(pedido, catorze);
        if processar && !SCOUT_DAILY_DRY_RUN {
            crate::comparison::branch("cqf_12 request processed");
            let mut feitos: i8 = 0;
            for k in 0..10u32 {
                if feitos >= 3 { // cqf12: até três relatórios
                    break;
                }
                let lo = registro_do_olheiro(this, k, tecnico);
                if cobre(lo, nacao) != 0 {
                    relatorio(lo, jogador, reg, -1000, 1, 0, 0, 1);
                    feitos += 1;
                    if (cqf12_b(reg + 0x66) as i8) > -1 {
                        crate::comparison::branch("cqf_12 request report");
                        acrescentar(l, reg, 0x67);
                    }
                }
            }
        }
        if tirar {
            let n = cqf12_i(this + 0x64);
            let base = cqf12_d(this + 0x5c);
            core::ptr::copy(base.wrapping_add((n as usize).wrapping_mul(0x11)).wrapping_sub(0x11) as *const u8, pedido as *mut u8, 0x11);
            core::ptr::write_unaligned((this + 0x64) as *mut i32, n - 1);
            i -= 1;
        }
        i += 1;
    }
    if SCOUT_DAILY_DRY_RUN {
        soltar(l);
        return 1;
    }
    relatorios(this, l);
    ordenar(cqf12_d(this + 0x5c), cqf12_i(this + 0x64), 0x11, 0x7fc270);
    for k in 0..0x10i32 {
        let p = cqf12_pessoa((cqf12_i(0xae2364) - k - 1) as usize);
        if cqf12_b(p + 0x5f) == 1 {
            crate::comparison::branch("cqf_12 human search");
            for j in 0..10u32 {
                buscar(registro_do_olheiro(this, j, p)); // cqf12: a busca de cada lista
            }
        }
    }
    soltar(l);
    1
}

/// Comparador da 0x7f8750: os relatórios sorteiam e alocam e as buscas mexem nas listas dos humanos: só a original
/// roda. Antes, a cópia em ensaio poda uma cópia dos pedidos (sem os relatórios) e os põe na ordem da original; os
/// pedidos que a original deixa (conta e os 0x11 bytes de cada) têm de ser esses.
unsafe extern "thiscall" fn compare_scout_manager_daily_update(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    if cqf12_b(0xae2c88) != 2 {
        return o(this);
    }
    let n0 = cqf12_i(this + 0x64).max(0) as usize;
    let base = cqf12_d(this + 0x5c);
    let mut copia: Vec<u8> = (0..n0 * 0x11).map(|k| cqf12_b(base + k)).collect();
    copia.push(0);
    // o ensaio poda a cópia: troca o vetor e a conta do `this` pelos da cópia e devolve depois
    let (b0, c0) = (cqf12_d(this + 0x5c), cqf12_i(this + 0x64));
    core::ptr::write_unaligned((this + 0x5c) as *mut usize, copia.as_mut_ptr() as usize);
    let estado = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let a1080 = cqf12_d(0xae1080);
    SCOUT_DAILY_DRY_RUN = true;
    crate::rng::without_counting(|| scout_manager_daily_update(this));
    SCOUT_DAILY_DRY_RUN = false;
    estado.write_back();
    core::ptr::write_unaligned(0xae1080 as *mut usize, a1080);
    let n1 = cqf12_i(this + 0x64).max(0) as usize;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(copia.as_mut_ptr() as usize, n1 as i32, 0x11, 0x7fc270);
    let plano: Vec<u8> = copia[..n1 * 0x11].to_vec();
    core::ptr::write_unaligned((this + 0x5c) as *mut usize, b0);
    core::ptr::write_unaligned((this + 0x64) as *mut i32, c0);
    let r = o(this);
    let n2 = cqf12_i(this + 0x64).max(0) as usize;
    let agora: Vec<u8> = (0..n2 * 0x11).map(|k| cqf12_b(cqf12_d(this + 0x5c) + k)).collect();
    SCOREBOARD.record(r == 1 && agora == plano, || format!("scout_manager_daily_update: pedidos {} -> original {} ensaio {}; {:x?} x {:x?}", n0, n2, n1, agora, plano));
    r
}

/// O registro das notícias de relatório que a 0x7f9a60 recebe (jogador, clube, quantos), ligado só no comparador da
/// 0x7f9890 para conferir os grupos da original.
static mut SCOUT_REPORT_GROUPS: Option<Vec<(u32, u32, u8)>> = None;

/// { 0x7f9890 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::send_scout_reports(lista)`, pelo dia dos olheiros
/// (0x7f8750) e pela busca (0x7f8c00). Com a lista não vazia (+0 conta): um vetor de conta × 0x67 (0x944e46; sem
/// memória, a caixa da linha 0x7dd) com cópias dos registros, ordenado (0x945333 com 0x7fc2a0); cada grupo seguido do
/// mesmo jogador (+0x2b) e do mesmo clube (+0x50) vira uma notícia de relatório (0x7f9a60 com o primeiro e quantos).
/// Libera o vetor (0x9452ca). A lista fica com o cursor no começo.
unsafe extern "thiscall" fn send_scout_reports(this: usize, lista: usize) {
    let conta = cqf12_w(lista);
    if conta == 0 {
        return;
    }
    let n = conta as i32;
    let alocar: unsafe extern "C" fn(i32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x7f9a60usize);
    // 0x944e46 (malloc): push; add esp, 4 -> cdecl; 0x9452ca (free): push; add esp, 4 -> cdecl; 0x68d1b0: mov ecx, lista
    // -> thiscall; 0x945333: 4 pushes; add esp, 0x10 -> cdecl; 0x7f9a60: push quantos; push registro; mov ecx, this ->
    // thiscall, ret 8
    let v = alocar(n.wrapping_mul(0x67));
    if v == 0 && n != 0 {
        show_error_at(0xa7b094, 0x7dd);
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqf12_d(lista + 2));
    let mut dst = v;
    let mut item = proximo(lista);
    while item != 0 {
        core::ptr::copy_nonoverlapping(item as *const u8, dst as *mut u8, 0x67);
        dst += 0x67;
        item = proximo(lista);
    }
    ordenar(v, n, 0x67, 0x7fc2a0);
    let mut i: i32 = 0;
    while i < n {
        let r = v + i as usize * 0x67;
        let clube_id = cqf12_d(cqf12_clube(cqf12_d(r + 0x50)));
        let jogador_id = cqf12_d(cqf12_pessoa(cqf12_d(r + 0x2b)));
        let inicio = i;
        let mut quantos: u8 = 1;
        let mut j = i + 1;
        while j < n {
            let s = v + j as usize * 0x67;
            if cqf12_d(s + 0x50) != clube_id || cqf12_d(s + 0x2b) != jogador_id {
                break;
            }
            quantos = quantos.wrapping_add(1);
            j += 1;
            i += 1;
        }
        let primeiro = v + inicio as usize * 0x67;
        if SCOUT_DAILY_DRY_RUN {
            if let Some(g) = &mut *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) {
                g.push((cqf12_d(primeiro + 0x2b) as u32, cqf12_d(primeiro + 0x50) as u32, quantos));
            }
        } else {
            noticia(this, primeiro, quantos as u32);
        }
        i += 1;
    }
    liberar(v);
}

/// Comparador da 0x7f9890: a original entrega notícias (aloca nas caixas): só ela roda, com o registro das chamadas à
/// 0x7f9a60 ligado; antes, a cópia em ensaio (sem a 0x7f9a60) diz os grupos (jogador, clube, quantos) em ordem, e as
/// chamadas da original têm de ser essas.
unsafe extern "thiscall" fn compare_send_scout_reports(this: usize, lista: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let cursor = cqf12_d(lista + 0xa);
    *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) = Some(Vec::new());
    SCOUT_DAILY_DRY_RUN = true;
    send_scout_reports(this, lista);
    SCOUT_DAILY_DRY_RUN = false;
    let plano = (*core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS)).replace(Vec::new()).unwrap_or_default();
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cursor);
    o(this, lista);
    let feito = (*core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS)).take().unwrap_or_default();
    SCOREBOARD.record(feito == plano, || format!("send_scout_reports(lista {:#x}) original {:x?} ensaio {:x?}", lista, feito, plano));
}

/// { 0x7f9a60 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da notícia local): `SCOUT_MANAGER::
/// send_scout_report_news(registro, quantos)`, pelos relatórios (0x7f9890). A notícia 0x138e (0x77bbe0): quantos (até
/// 3), +0x66, o olheiro (0x7e5160 com +0x1f), +0x59, +0x5a, o clube +0x50, os dados do jogo (0x7e4de0), +0x5e, +0x62,
/// +0x58, +0x54, de cada um dos registros (+0x37 + 0x67 × k) +0xc, +0, +4, +8 e +0x26 + 8 × +0x25, 0x420e9 e as forças
/// do jogador (+0x2b) para o clube (0x546ab0); vai ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn send_scout_report_news(this: usize, r: usize, quantos: u32) {
    let _ = this;
    let n = (quantos as u8 as i8).min(3);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let olheiro: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e5160usize);
    let dados: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e4de0usize);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    // 0x77bbe0: push 0; push 0x138e; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8; 0x7e5160
    // e 0x7e4de0: push &índice; push &notícia; push registro; add esp, 0xc -> cdecl; 0x546ab0: push clube; push
    // jogador; add esp, 8 -> cdecl; 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
    let mut noticia = [0u32; 0x38];
    let nt = noticia.as_mut_ptr() as usize;
    construir(nt, 0x138e, 0);
    gravar(nt, 0, n as i32 as u32);
    let mut idx: u8 = 1;
    let mut pr = |idx: &mut u8, v: u32| {
        gravar(nt, *idx as u32, v);
        *idx = idx.wrapping_add(1);
    };
    pr(&mut idx, cqf12_b(r + 0x66) as i8 as i32 as u32);
    olheiro(r + 0x1f, nt, &mut idx);
    pr(&mut idx, cqf12_b(r + 0x59) as i8 as i32 as u32);
    pr(&mut idx, cqf12_w(r + 0x5a) as i32 as u32);
    pr(&mut idx, cqf12_d(r + 0x50) as u32);
    dados(r, nt, &mut idx);
    pr(&mut idx, cqf12_d(r + 0x5e) as u32);
    pr(&mut idx, cqf12_d(r + 0x62) as u32);
    pr(&mut idx, cqf12_b(r + 0x58) as i8 as i32 as u32);
    pr(&mut idx, cqf12_d(r + 0x54) as u32);
    for k in 0..n.max(0) as usize {
        let p = r + 0x37 + k * 0x67;
        pr(&mut idx, cqf12_d(p + 0xc) as u32);
        pr(&mut idx, cqf12_d(p) as u32);
        pr(&mut idx, cqf12_d(p + 4) as u32);
        pr(&mut idx, cqf12_d(p + 8) as u32);
        pr(&mut idx, ((cqf12_b(p + 0x26) as i8 as i32) + (cqf12_b(p + 0x25) as i8 as i32) * 8) as u32);
    }
    pr(&mut idx, 0x420e9);
    let c = cqf12_i(r + 0x50);
    let clube = if c > -1 { cqf12_clube(c as usize) } else { 0 };
    let k = idx;
    idx = idx.wrapping_add(1);
    let _ = idx;
    gravar(nt, k as u32, forcas(cqf12_pessoa(cqf12_d(r + 0x2b)), clube));
    entregar(cqf12_d(0xb7431c), nt, cqf12_clube(cqf12_d(r + 0x50)));
}

/// Comparador da 0x7f9a60: a notícia é entregue (aloca na caixa do humano): só a original roda (ponytail: o efeito é a
/// caixa de entrada; os grupos que chegam aqui são conferidos pela 0x7f9890). Com o registro ligado, anota a chamada.
unsafe extern "thiscall" fn compare_send_scout_report_news(this: usize, r: usize, quantos: u32) {
    if let Some(g) = &mut *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) {
        g.push((cqf12_d(r + 0x2b) as u32, cqf12_d(r + 0x50) as u32, quantos as u8));
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, r, quantos);
}
