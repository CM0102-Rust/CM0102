// ---------- 270a leva: os olheiros do dia, os relatórios deles e a notícia do relatório (scout_manager.cpp) ----------

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
unsafe fn cqf12_depois(dia: i16, ano: u16, data: usize) -> bool {
    let (d, a) = (cqf12_w(data), cqf12_w(data + 2) as u16);
    ano > a || (ano == a && dia > d)
}

/// O ensaio da 0x7f8750: a cópia só poda os pedidos (num vetor dela) e para antes dos relatórios.
static mut CQF12_ENSAIO: bool = false;

/// A poda de um pedido (0x11 bytes: +0 o clube, +4 o jogador, +8 a data, +0x10 quantos relatórios): devolve
/// (tirar, processar). Sai o pedido de clube que não é humano, de jogador do próprio clube (o +0x39 ou o do contrato),
/// com mais de um ano, com 3 relatórios, ou (depois de 30 dias) de jogador machucado (0x62c400 = 2); entre 30 dias e um
/// ano o pedido é processado (e sai), salvo jogador sem nação ou de clube que joga nos 14 dias seguintes, que fica.
unsafe fn cqf12_julgar(pedido: usize, catorze: usize) -> (bool, bool, usize, usize, usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
    let c = contrato_de(cqf12_d(0xae1050), jogador);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    let do_contrato = if c != 0 { cqf12_clube(cqf12_d(c + 4)) } else { 0 };
    if do_contrato == clube {
        return (true, false, 0, 0, 0);
    }
    let (dia, ano) = (cqf12_w(0xae2c90), cqf12_w(0xae2c92) as u16);
    let mut t = [0u32; 2];
    let ano_depois = mais(pedido + 8, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano
    if cqf12_depois(dia, ano, ano_depois) {
        return (true, false, 0, 0, 0);
    }
    if (cqf12_b(pedido + 0x10) as i8) >= 3 { // cqf12: três relatórios
        return (true, false, 0, 0, 0);
    }
    let mut t2 = [0u32; 2];
    let mes_depois = mais(pedido + 8, t2.as_mut_ptr() as usize, 0x1e); // cqf12: trinta dias
    if !cqf12_depois(dia, ano, mes_depois) {
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
unsafe extern "thiscall" fn cqf12_olheiros_do_dia(this: usize) -> u32 {
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
        let (tirar, processar, jogador, tecnico, nacao) = cqf12_julgar(pedido, catorze);
        if processar && !CQF12_ENSAIO {
            crate::comparacao::ramo("cqf_12 pedido processado");
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
                        crate::comparacao::ramo("cqf_12 relatorio do pedido");
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
    if CQF12_ENSAIO {
        soltar(l);
        return 1;
    }
    relatorios(this, l);
    ordenar(cqf12_d(this + 0x5c), cqf12_i(this + 0x64), 0x11, 0x7fc270);
    for k in 0..0x10i32 {
        let p = cqf12_pessoa((cqf12_i(0xae2364) - k - 1) as usize);
        if cqf12_b(p + 0x5f) == 1 {
            crate::comparacao::ramo("cqf_12 busca do humano");
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
unsafe extern "thiscall" fn comparar_cqf12_olheiros_do_dia(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1626);
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
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let a1080 = cqf12_d(0xae1080);
    CQF12_ENSAIO = true;
    crate::aleatorio::sem_contar(|| cqf12_olheiros_do_dia(this));
    CQF12_ENSAIO = false;
    estado.escrever();
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
    PLACAR.registrar(r == 1 && agora == plano, || format!("cqf12_olheiros_do_dia: pedidos {} -> original {} ensaio {}; {:x?} x {:x?}", n0, n2, n1, agora, plano));
    r
}

/// O registro das notícias de relatório que a 0x7f9a60 recebe (jogador, clube, quantos), ligado só no comparador da
/// 0x7f9890 para conferir os grupos da original.
static mut CQF12_GRUPOS: Option<Vec<(u32, u32, u8)>> = None;

/// { 0x7f9890 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::send_scout_reports(lista)`, pelo dia dos olheiros
/// (0x7f8750) e pela busca (0x7f8c00). Com a lista não vazia (+0 conta): um vetor de conta × 0x67 (0x944e46; sem
/// memória, a caixa da linha 0x7dd) com cópias dos registros, ordenado (0x945333 com 0x7fc2a0); cada grupo seguido do
/// mesmo jogador (+0x2b) e do mesmo clube (+0x50) vira uma notícia de relatório (0x7f9a60 com o primeiro e quantos).
/// Libera o vetor (0x9452ca). A lista fica com o cursor no começo.
unsafe extern "thiscall" fn cqf12_relatorios_dos_olheiros(this: usize, lista: usize) {
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
        aviso_em(0xa7b094, 0x7dd);
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
        if CQF12_ENSAIO {
            if let Some(g) = &mut *core::ptr::addr_of_mut!(CQF12_GRUPOS) {
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
unsafe extern "thiscall" fn comparar_cqf12_relatorios_dos_olheiros(this: usize, lista: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1627);
    let cursor = cqf12_d(lista + 0xa);
    *core::ptr::addr_of_mut!(CQF12_GRUPOS) = Some(Vec::new());
    CQF12_ENSAIO = true;
    cqf12_relatorios_dos_olheiros(this, lista);
    CQF12_ENSAIO = false;
    let plano = (*core::ptr::addr_of_mut!(CQF12_GRUPOS)).replace(Vec::new()).unwrap_or_default();
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cursor);
    o(this, lista);
    let feito = (*core::ptr::addr_of_mut!(CQF12_GRUPOS)).take().unwrap_or_default();
    PLACAR.registrar(feito == plano, || format!("cqf12_relatorios_dos_olheiros(lista {:#x}) original {:x?} ensaio {:x?}", lista, feito, plano));
}

/// { 0x7f9a60 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da notícia local): `SCOUT_MANAGER::
/// send_scout_report_news(registro, quantos)`, pelos relatórios (0x7f9890). A notícia 0x138e (0x77bbe0): quantos (até
/// 3), +0x66, o olheiro (0x7e5160 com +0x1f), +0x59, +0x5a, o clube +0x50, os dados do jogo (0x7e4de0), +0x5e, +0x62,
/// +0x58, +0x54, de cada um dos registros (+0x37 + 0x67 × k) +0xc, +0, +4, +8 e +0x26 + 8 × +0x25, 0x420e9 e as forças
/// do jogador (+0x2b) para o clube (0x546ab0); vai ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn cqf12_noticia_do_relatorio(this: usize, r: usize, quantos: u32) {
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
unsafe extern "thiscall" fn comparar_cqf12_noticia_do_relatorio(this: usize, r: usize, quantos: u32) {
    if let Some(g) = &mut *core::ptr::addr_of_mut!(CQF12_GRUPOS) {
        g.push((cqf12_d(r + 0x2b) as u32, cqf12_d(r + 0x50) as u32, quantos as u8));
    }
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1628)(this, r, quantos);
}

// ---------- 266a leva: o dia das seleções e as buscas da sessão do técnico humano (national_teams.cpp, scout_manager.cpp) ----------

unsafe fn cqc12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc12_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A data `r` é a de hoje (o ano e o dia)?
unsafe fn cqc12_e_hoje(r: usize, hoje: usize) -> bool {
    cqc12_uw(r + 2) == cqc12_uw(hoje + 2) && cqc12_w(r) == cqc12_w(hoje)
}

/// Marca os clubes com jogo na data `d` e nos dias vizinhos (0x7775c0 na data, na véspera e no dia seguinte; a data
/// volta a ser a mesma no fim: 0x54c8e0/0x54c5b0 e 0x417c10).
unsafe fn cqc12_marcar_tres_dias(this: usize, d: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7775c0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    // 0x7775c0: push &data; mov ecx, edi -> thiscall, ret 4
    let (mut t, mut u) = ([0u8; 8], [0u8; 8]);
    let (pt, pu) = (t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    crate::comparacao::ramo("cqc_12 marca os clubes");
    marcar(this, d);
    copiar(d, pu, menos(d, pt, 1));
    marcar(this, d);
    copiar(d, pu, mais(d, pt, 2));
    marcar(this, d);
    copiar(d, pu, menos(d, pt, 1));
}

/// Os jogos da lista `l` (do dia daqui a duas semanas): para cada um, as convocações (0x76bab0) e, fora das fases
/// finais (a competição +0x14 numa das chaves 0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0,
/// 0x9cf78c, 0x9cf964), com mandante que não é a seleção B (0x5409b0) e a marca +0x40: os convocados por clube
/// (0x7771c0) das duas seleções; com o bit 0x200 de +0x4d, também os das seleções B (0x53b4f0 da nação +0x53 de cada
/// uma) e, sem ele e com o 0x100, os das duas com `b` = 1.
unsafe fn cqc12_jogos(this: usize, l: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let convocar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76bab0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7771c0usize);
    // 0x76bab0: push p; mov ecx, edi -> thiscall ret 4; 0x5409b0 e 0x53b4f0: push; add esp, 4 -> cdecl;
    // 0x7771c0: push p; push b; push time; mov ecx, edi -> thiscall ret 0xc
    cqc12_pd(l + 0xa, cqc12_d(l + 2));
    let mut p = proximo(l);
    while p != 0 {
        crate::comparacao::ramo("cqc_12 convoca");
        convocar(this, p); // cqc12: as convocações para o jogo
        let comp = cqc12_d(p + 0x14);
        let final_ = comp != 0
            && [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964]
                .iter().any(|&k| cqc12_d(comp) == cqc12_d(k));
        if !final_ && segunda(cqc12_d(p + 0x1c)) == 0 && *((p + 0x40) as *const u8) != 0 {
            registrar(this, cqc12_d(p + 0x1c), 0, p);
            registrar(this, cqc12_d(p + 0x20), 0, p);
            let f = cqc12_uw(p + 0x4d);
            if f & 0x200 != 0 {
                let b1 = selecao_b(cqc12_d(cqc12_d(p + 0x1c) + 0x53));
                registrar(this, b1, 0, p);
                let b2 = selecao_b(cqc12_d(cqc12_d(p + 0x20) + 0x53));
                registrar(this, b2, 0, p);
            } else if f & 0x100 != 0 {
                registrar(this, cqc12_d(p + 0x1c), 1, p);
                registrar(this, cqc12_d(p + 0x20), 1, p);
            }
        }
        p = proximo(l);
    }
}

/// { 0x76b100 } (thiscall, ret 4, national_teams.cpp, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::daily_update(hoje)`, pelo dia do jogo ([0xb74318], 0x5ccb07). A data daqui a duas semanas
/// (0x54c770); a tabela dos clubes com jogo (+0x24) sai (0x9452ca) e zera. Em 1º de janeiro, 15 de abril e 1º de agosto
/// (0x549f70 no ano de hoje), as reservas das seleções se refazem (0x76abf0). Nas 3 fatias do dia daqui a duas
/// semanas, as listas dos grupos 0 e 1 (0x5abf10): com jogo em alguma e sem a tabela, os clubes com jogo em volta da
/// data são marcados (cqc12_marcar_tres_dias); os jogos das duas listas (cqc12_jogos). Com a tabela, os jogos dos clubes
/// com convocados se remarcam (0x7777e0). Da lista em +0 saem (0x68d1f0) os itens do dia de hoje (+0x10). Com o ranking
/// (+0x1c), a atualização dele (0x58d460); no dia múltiplo de 70, a poda dos registros de convocação (0x7774b0); por fim
/// as notícias dos classificados (0x7779e0).
unsafe extern "thiscall" fn dia_das_selecoes(this: usize, hoje: usize) {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let refazer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x76abf0usize);
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let remarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7777e0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let ranking: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x58d460usize);
    let podar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7774b0usize);
    let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7779e0usize);
    // 0x54c770: push 2; push saida; mov ecx, hoje -> thiscall ret 8; free: add esp, 4 -> cdecl; 0x549f70: 4 pushes;
    // lea ecx, saida -> thiscall ret 0x10; 0x76abf0, 0x7774b0, 0x7779e0: mov ecx, edi sem pushes -> thiscall;
    // 0x5abf10: 4 pushes; mov ecx, [0xae2a58] -> thiscall ret 0x10; 0x7777e0: push &data; mov ecx, edi -> thiscall
    // ret 4; 0x68d1f0: push item; mov ecx, [edi] -> thiscall ret 4; 0x58d460: push hoje; mov ecx, [edi + 0x1c] ->
    // thiscall ret 4
    let mut dd = [0u8; 8];
    let d = dd.as_mut_ptr() as usize;
    semanas(hoje, d, 2);
    if cqc12_d(this + 0x24) != 0 {
        liberar(cqc12_d(this + 0x24));
        cqc12_pd(this + 0x24, 0);
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
    let fixman = cqc12_d(0xae2a58);
    for fatia in 0u32..3 {
        // os dwords da original: o ano (com os 2 bytes seguintes da data) e o dia (com o ano em cima); ponytail: a
        // fatia vai sem a sobra de pilha dos bytes de cima (a 0x5abf10 faz and 0xff)
        let l0 = lista(fixman, cqc12_d(d + 2) as u32, cqc12_d(d) as u32, fatia, 0);
        let l1 = lista(fixman, cqc12_d(d + 2) as u32, cqc12_d(d) as u32, fatia, 1);
        if ((l0 != 0 && cqc12_w(l0) != 0) || (l1 != 0 && cqc12_w(l1) != 0)) && cqc12_d(this + 0x24) == 0 {
            cqc12_marcar_tres_dias(this, d); // cqc12: os clubes com jogo em volta da data
        }
        if l0 != 0 {
            cqc12_jogos(this, l0);
        }
        if l1 != 0 {
            cqc12_jogos(this, l1);
        }
    }
    if cqc12_d(this + 0x24) != 0 {
        remarcar(this, d);
    }
    if cqc12_d(this) != 0 {
        let dia = cqc12_w(hoje);
        let l = cqc12_d(this);
        cqc12_pd(l + 0xa, cqc12_d(l + 2));
        let mut p = proximo(cqc12_d(this));
        while p != 0 {
            if cqc12_w(p + 0x10) == dia {
                tirar(cqc12_d(this), p);
            }
            p = proximo(cqc12_d(this));
        }
    }
    if cqc12_d(this + 0x1c) != 0 {
        ranking(cqc12_d(this + 0x1c), hoje);
    }
    if cqc12_w(hoje) as i32 % 0x46 == 0 {
        podar(this);
    }
    classificados(this);
}

/// Quantos itens da lista em +0 são do dia `dia` (+0x10), percorrendo os nós sem mexer no cursor.
unsafe fn cqc12_do_dia(this: usize, dia: i16) -> u32 {
    let l = cqc12_d(this);
    if l == 0 {
        return 0;
    }
    let mut n = 0;
    let mut no = cqc12_d(l + 2);
    while no != 0 {
        if cqc12_w(cqc12_d(no) + 0x10) == dia {
            n += 1;
        }
        no = cqc12_d(no + 6);
    }
    n
}

/// Comparador: a original convoca (0x76bab0: notícias, elencos), remarca jogos (0x7777e0), aloca a tabela (0x7775c0)
/// e libera itens (0x68d1f0): não pode rodar duas vezes. Só ela roda, conferindo o plano do que a própria rotina grava:
/// nenhum item do dia de hoje sobra na lista em +0.
unsafe extern "thiscall" fn comparar_dia_das_selecoes(this: usize, hoje: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1539);
    let dia = cqc12_w(hoje);
    let antes = cqc12_do_dia(this, dia);
    o(this, hoje);
    let sobra = cqc12_do_dia(this, dia);
    PLACAR.registrar(sobra == 0, || format!("dia_das_selecoes(dia {}) itens do dia antes {} depois {}", dia, antes, sobra));
}

/// O índice (0..15) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um byte da original
/// ((id − [0xae2364]) + 16 em 8 bits); Err(()) sem técnico, Ok(None) fora (a caixa).
unsafe fn cqc12_indice() -> Result<Option<usize>, ()> {
    let t = cqc12_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// O registro de busca `k` (0x2b0 bytes) da sessão `s`.
unsafe fn cqc12_busca(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// { 0x7f8420 } (thiscall, ret 0, scout_manager.cpp): `SCOUT_MANAGER::current_search_has_no_club()`, pelas telas das
/// buscas (0x482a9b). A sessão do técnico humano da vez ([this + 0xc + 4 × índice]): a busca atual (o byte com sinal
/// em +0 da sessão) sem clube (+0x191 = −2). Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x454; a
/// original) e 0.
unsafe extern "thiscall" fn busca_atual_sem_clube(this: usize) -> u32 {
    let i = match cqc12_indice() {
        Err(()) => return 0,
        Ok(None) => return orig::<unsafe extern "thiscall" fn(usize) -> u32>(1540)(this), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    (cqc12_d(cqc12_busca(s, *(s as *const i8) as i32) + 0x191) as i32 == -2) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_busca_atual_sem_clube(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1540);
    if matches!(cqc12_indice(), Ok(None)) {
        return o(this);
    }
    let r = o(this);
    let n = busca_atual_sem_clube(this);
    PLACAR.registrar(r == n, || format!("busca_atual_sem_clube() original={} rust={}", r, n));
    r
}

/// { 0x7f8500 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::is_club_in_session_searches(clube)`, pelas telas
/// (0x482aaf, 0x483040, 0x7b34e1). Alguma das 10 buscas da sessão do técnico humano da vez tem o clube (+0x191 = +0
/// do clube): 1. Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x480; a original) e 0.
unsafe extern "thiscall" fn clube_nas_buscas_da_sessao(this: usize, clube: usize) -> u32 {
    let i = match cqc12_indice() {
        Err(()) => return 0,
        Ok(None) => return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1541)(this, clube), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    let id = cqc12_d(clube);
    // ponytail: a original testa o contador contra 0xff (nunca) para usar a busca atual (+0); fica o contador
    (0..10).any(|k| cqc12_d(cqc12_busca(s, k) + 0x191) == id) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_clube_nas_buscas_da_sessao(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1541);
    if matches!(cqc12_indice(), Ok(None)) {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = clube_nas_buscas_da_sessao(this, clube);
    PLACAR.registrar(r == n, || format!("clube_nas_buscas_da_sessao(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 266a leva: os olheiros dos técnicos humanos na partida e a sessão de edição das buscas (scout_manager.cpp) ----------

unsafe fn cqc13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x7f8c00 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `SCOUT_MANAGER::scout_match(partida, resultado)`, pelo fechamento da partida do dia (0x6b5f00, Rust). Para cada um
/// dos 16 últimos registros de pessoa ([0xae23bc], 0x6e bytes, do último para trás) com +0x5f = 1 (o técnico humano):
/// num vetor zerado de 10 marcas (0x945501), os 10 observadores numa ordem sorteada (0x90cfc0 de 10 até achar um não
/// marcado); o registro de cada um (0x7f6cd0) com +0x19d, numa partida com as marcas +0x40 e +0x41 e do clube do
/// técnico (+0x39 = +0x1c ou +0x20 da partida), começa a observação (0x7ae9c0 com −1, −1, −1, 0, 1); e observa a
/// partida (0x7b5be0, os relatórios na lista local). O vetor sai (0x9452ca). Por fim os relatórios vão (0x7f9890) e a
/// lista é destruída. Devolve 1.
unsafe extern "thiscall" fn olheiros_na_partida(this: usize, p: usize, resultado: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let registro: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let comecar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x7ae9c0usize);
    let observar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x7b5be0usize);
    let enviar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f9890usize);
    // calloc: push 4; push 0xa; add esp, 8 -> cdecl (10, 4); sorteio: push 0xa; add esp, 4 -> cdecl; free: cdecl;
    // 0x7f6cd0: push pessoa; push qual; mov ecx, [this] -> thiscall ret 8; 0x7ae9c0: 5 pushes; mov ecx, registro ->
    // thiscall ret 0x14; 0x7b5be0: push &lista; push resultado; push partida; mov ecx, registro -> thiscall ret 0xc;
    // 0x7f9890: push &lista; mov ecx, this -> thiscall ret 4; 0x68d020/0x68d040: lea ecx, lista -> thiscall
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    for k in 0..16i32 {
        let idx = (cqc13_d(0xae2364) as i32).wrapping_sub(k).wrapping_sub(1);
        let pessoa = cqc13_d(0xae23bc).wrapping_add((idx as usize).wrapping_mul(0x6e));
        if *((pessoa + 0x5f) as *const u8) != 1 {
            continue;
        }
        crate::comparacao::ramo("cqc_13 olheiros do humano");
        let marcas = zerado(10, 4);
        for _ in 0..10 { // cqc13: os 10 observadores
            let q = loop {
                let q = sortear(10) as i8; // cqc13: o observador sorteado
                if cqc13_d(marcas.wrapping_add((q as isize * 4) as usize)) == 0 {
                    break q; // cqc13: um ainda não marcado
                }
            };
            core::ptr::write_unaligned(marcas.wrapping_add((q as isize * 4) as usize) as *mut u32, 1);
            // ponytail: a original empurra o dword com o byte do observador e sobra de pilha em cima; a 0x7f6cd0 lê o
            // byte
            let r = registro(this, q as u8 as u32, pessoa);
            if cqc13_d(r + 0x19d) != 0 && *((p + 0x40) as *const u8) != 0 && *((p + 0x41) as *const u8) != 0 {
                let clube = cqc13_d(pessoa + 0x39);
                if cqc13_d(p + 0x1c) == clube || cqc13_d(p + 0x20) == clube {
                    comecar(r, u32::MAX, u32::MAX, u32::MAX, 0, 1);
                }
            }
            observar(r, p, resultado, lista);
        }
        liberar(marcas);
    }
    enviar(this, lista);
    soltar(lista);
    1
}

/// Comparador: a observação (0x7b5be0, original) grava o que os olheiros sabem e os relatórios, a 0x7ae9c0 começa
/// buscas e a 0x7f9890 entrega as notícias: não pode rodar duas vezes. Só a original roda (ponytail: ela mesma só grava
/// o vetor e a lista locais, que saem); confere a devolução (1).
unsafe extern "thiscall" fn comparar_olheiros_na_partida(this: usize, p: usize, resultado: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1543)(this, p, resultado);
    PLACAR.registrar(r == 1, || format!("olheiros_na_partida(partida {:x}) original={}", p, r));
    r
}

/// O índice (0..15) do técnico `tecnico` (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um
/// byte da original ((id − [0xae2364]) + 16 em 8 bits); None nos casos da caixa (sem técnico ou fora).
unsafe fn cqc13_indice(tecnico: usize) -> Option<usize> {
    let t = if tecnico != 0 {
        tecnico
    } else {
        cqc13_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// { 0x7f6be0 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_edit_session(tecnico)`, pela montagem dos
/// conhecidos da busca (0x7afeb7). A sessão de edição do técnico humano (+0xc + 4 × índice). Sem técnico ou índice
/// fora: a caixa (linhas 0xde e 0xe8; a original) e 0.
unsafe extern "thiscall" fn sessao_de_edicao(this: usize, tecnico: usize) -> usize {
    let Some(i) = cqc13_indice(tecnico) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1544)(this, tecnico); // a caixa
    };
    cqc13_d(this + 0xc + i * 4)
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_sessao_de_edicao(this: usize, tecnico: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1544);
    if cqc13_indice(tecnico).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = sessao_de_edicao(this, tecnico);
    PLACAR.registrar(r == n, || format!("sessao_de_edicao(tecnico {:x}) original={:x} rust={:x}", tecnico, r, n));
    r
}

/// { 0x7f6af0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_edit_session(sessao, tecnico)`, pelas telas
/// das buscas (0x8085bc, 0x812d88...). Grava a sessão em +0xc + 4 × índice do técnico e devolve 1. Sem técnico ou
/// índice fora: a caixa (linhas 0xb8 e 0xc2; a original) e 0.
unsafe extern "thiscall" fn escolher_sessao_de_edicao(this: usize, sessao: usize, tecnico: usize) -> u32 {
    let Some(i) = cqc13_indice(tecnico) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1545)(this, sessao, tecnico); // a caixa
    };
    core::ptr::write_unaligned((this + 0xc + i * 4) as *mut usize, sessao);
    1
}

/// Comparador: lado a lado pelas 16 sessões e a devolução; com a caixa, só a original.
unsafe extern "thiscall" fn comparar_escolher_sessao_de_edicao(this: usize, sessao: usize, tecnico: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1545);
    if cqc13_indice(tecnico).is_none() {
        return o(this, sessao, tecnico);
    }
    let reg = [(this + 0xc, 0x40)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, sessao, tecnico),
        || escolher_sessao_de_edicao(this, sessao, tecnico));
    PLACAR.registrar(igual && r == n, || format!("escolher_sessao_de_edicao(sessao {:x}, tecnico {:x}) original={} rust={} {:x?} x {:x?}",
        sessao, tecnico, r, n, mo, mr));
    r
}

// ---------- 261a leva: os amistosos internacionais do dia e a lista escolhida do olheiro do técnico humano (friendly.cpp, scout_manager.cpp) ----------

unsafe fn cpy4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// As datas estáticas da 0x54c770 (valor e guarda) e da 0x54c5b0, 0x54c8e0 e 0x54cdb0.
const CPY4_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// O corpo da 0x5cbb00. No `ensaio` (o comparador), nada se grava e nada se chama além das consultas: devolve o par
/// (primeira entrada, quantas) que iria à 0x5cbdf0, ou None se a original não a chamaria.
unsafe fn cpy4_amistosos(this: usize, ensaio: bool) -> Option<(usize, u32)> {
    // 0x54a110: thiscall(data) (lea ecx); 0x54c770: thiscall(0xae2c90, saída, 0xc) (mov ecx, 0xae2c90; 2 push);
    // 0x68d080: thiscall(lista) (mov ecx, lista; sem push); 0x945333: cdecl(vetor, conta, tamanho, comparação) (add esp,
    // 0x10); 0x5cbdd0: thiscall(entrada, ano) (mov ecx, entrada; 1 push); 0x417c10: thiscall(destino, saída, origem)
    // (lea ecx, destino; 2 push); 0x946464: cdecl(chave, vetor, conta, 0x14, 0x5cc610) (add esp, 0x14); 0x456770 e
    // 0x456740: thiscall(data, baixo, alto) (lea ecx, data; 2 push); 0x5cbdf0: thiscall(this, primeira, quantas) (mov
    // ecx, esi; 2 push)
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let data_da_entrada: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5cbdd0usize);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let marcar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5cbdf0usize);
    // a chave da busca: a data em +3, como as entradas de 20 bytes (a original passa a data local − 3)
    let mut chave = [0u8; 0x14];
    let d = chave.as_mut_ptr() as usize + 3;
    data_padrao(d);
    let mut lim = [0u8; 8];
    semanas(0xae2c90, &mut lim, 0xc); // cpy4: hoje + 12 semanas
    let hoje = cpy4_w(0xae2c90);
    if hoje == 0 && !ensaio {
        crate::comparacao::ramo("cpy_4 virada do ano");
        let mut i = 0i32;
        while i < cpy4_d(0xae2350) as i32 {
            let l = cpy4_d(cpy4_d(this + 0x34f6).wrapping_add(i as usize * 4));
            if l != 0 {
                limpar(l);
            }
            i += 1;
        }
        ordenar(cpy4_d(this + 0x34fa), cpy4_d(this + 0x34fe) as u32, 4, 0x5cc5f0); // cpy4: as nações pela reputação
        // o primeiro ano de Copa (ano − 2 múltiplo de 4) a partir deste; a metade alta do dword que a original soma é
        // sobra de pilha: aqui, zero
        let mut ano = cpy4_w(0xae2c92) as u32;
        while ((ano & 0xffff) as i32 - 2) % 4 != 0 {
            ano = ano.wrapping_add(1);
        }
        let mut k = 0i32;
        while k < cpy4_d(this + 0x34f0) as i32 {
            let e = cpy4_d(this + 0x34ec).wrapping_add(k as usize * 0x14);
            let mut a = ((*((e + 0xb) as *const i8)) as i16 as u16 as u32).wrapping_add(ano);
            if (a & 0xffff) as i32 - cpy4_w(0xae2c92) as i32 > 3 {
                a = a.wrapping_add(0xfffc);
            }
            data_da_entrada(e, a);
            k += 1;
        }
        ordenar(cpy4_d(this + 0x34ec), cpy4_d(this + 0x34f0) as u32, 0x14, 0x5cc610);
    }
    // a data de 0xdd6ec8 depois de hoje + 12 semanas: nada
    let x_ano = cpy4_w(0xdd6eca);
    let l_ano = core::ptr::read_unaligned(lim.as_ptr().add(2) as *const u16);
    if x_ano > l_ano || (x_ano == l_ano && *(0xdd6ec8 as *const i16) > core::ptr::read_unaligned(lim.as_ptr() as *const i16)) {
        return None;
    }
    if cpy4_w(this + 0x34f4) as u32 == hoje as i16 as i32 as u32 {
        return None;
    }
    let mut lixo = [0u8; 8];
    copiar(d, &mut lixo, lim.as_ptr() as usize);
    let base = cpy4_d(this + 0x34ec);
    let n = cpy4_d(this + 0x34f0);
    let p = bsearch(chave.as_ptr() as usize, base, n as u32, 0x14, 0x5cc610);
    if p == 0 {
        return None;
    }
    let igual = |e: usize| antes(d, cpy4_d(e + 3) as u32, cpy4_d(e + 7) as u32) == 0 && depois(d, cpy4_d(e + 3) as u32, cpy4_d(e + 7) as u32) == 0;
    // volta até a primeira entrada da data
    let mut e = p;
    while e > base {
        if !igual(e - 0x14) {
            break;
        }
        e -= 0x14;
    }
    let primeira = e;
    let ultima = base.wrapping_add(n.wrapping_mul(0x14)).wrapping_sub(0x14);
    let mut quantas = 0u32;
    while e <= ultima && igual(e) {
        quantas += 1;
        e += 0x14;
    }
    if ensaio {
        return Some((primeira, quantas));
    }
    crate::comparacao::ramo("cpy_4 amistosos marcados");
    marcar(this, primeira, quantas); // cpy4: marcar os amistosos da data
    *((this + 0x34f4) as *mut u16) = cpy4_w(0xae2c90);
    Some((primeira, quantas))
}

/// { 0x5cbb00 } (thiscall, ret 0, friendly.cpp, quadro SEH da data local de destrutor vazio): `Friendly::
/// update_international_friendlies()`, pelo dia dos amistosos (0x5c2460, já em Rust). No primeiro dia do ano: as listas
/// de nações ([+0x34f6], [0xae2350] delas) se esvaziam (0x68d080), a lista em +0x34fa (conta +0x34fe) se ordena pela
/// reputação (0x5cc5f0) e cada data internacional (+0x34ec, 20 bytes, conta +0x34f0) ganha o ano pelo ciclo da Copa
/// (ano − 2 múltiplo de 4, mais o deslocamento +0xb, menos 4 se passar de 3 anos à frente; 0x5cbdd0), reordenadas pela
/// data (0x5cc610). Depois, se a data de 0xdd6ec8 não passa de hoje + 12 semanas (0x54c770) e o dia não foi feito
/// (+0x34f4), as entradas com a data de hoje + 12 semanas (bsearch e as vizinhas iguais) vão à 0x5cbdf0 e o dia fica
/// marcado.
unsafe extern "thiscall" fn amistosos_internacionais_do_dia(this: usize) {
    cpy4_amistosos(this, false);
}

/// Comparador: no primeiro dia do ano (as listas se esvaziam, liberando) e quando há amistosos a marcar (a 0x5cbdf0
/// cria jogos e sorteia), só a original, conferindo o plano: o dia marcado em +0x34f4. Nos outros dias, lado a lado
/// pelo dia marcado e pelas datas estáticas.
unsafe extern "thiscall" fn comparar_amistosos_internacionais_do_dia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1353);
    if cpy4_w(0xae2c90) == 0 {
        return o(this);
    }
    if let Some((primeira, quantas)) = cpy4_amistosos(this, true) {
        o(this);
        let dia = cpy4_w(this + 0x34f4);
        PLACAR.registrar(dia == cpy4_w(0xae2c90), || format!("amistosos_internacionais_do_dia(primeira {:x}, quantas {}) dia marcado={} hoje={}", primeira, quantas, dia, cpy4_w(0xae2c90)));
        return;
    }
    let mut reg = CPY4_ESTATICAS.to_vec();
    reg.push((this + 0x34f4, 2));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || amistosos_internacionais_do_dia(this));
    PLACAR.registrar(igual, || format!("amistosos_internacionais_do_dia(sem amistosos) {:x?} x {:x?}", mo, mr));
}

/// O técnico (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) e o índice dele entre os 16 humanos (id −
/// [0xae2364] + 16), ou None nos casos de erro da original (sem técnico ou fora de 0..15).
unsafe fn cpy4_indice(tecnico: usize) -> Option<usize> {
    let t = if tecnico != 0 {
        tecnico
    } else {
        cpy4_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (cpy4_d(t) as i32).wrapping_sub(cpy4_d(0xae2364) as i32).wrapping_add(0x10);
    if !(0..=0xf).contains(&i) {
        return None;
    }
    Some(i as usize)
}

/// { 0x7f6e30 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_selected_list(tecnico)`, pelas telas de
/// busca (0x804248). A lista escolhida pelo técnico humano (byte em +0x4c + índice). Sem técnico ou fora de 0..15: a
/// original (caixa "Error", 0xff).
unsafe extern "thiscall" fn lista_escolhida_do_tecnico(this: usize, tecnico: usize) -> u8 {
    let Some(i) = cpy4_indice(tecnico) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1354)(this, tecnico); // caixa: só a original
    };
    *((this + 0x4c + i) as *const u8)
}

/// Comparador: a devolução (rotina pura); nos casos de erro, só a original (caixa).
unsafe extern "thiscall" fn comparar_lista_escolhida_do_tecnico(this: usize, tecnico: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1354);
    if cpy4_indice(tecnico).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = lista_escolhida_do_tecnico(this, tecnico);
    PLACAR.registrar(r == n, || format!("lista_escolhida_do_tecnico(tecnico {:x}) original={} rust={}", tecnico, r, n));
    r
}

/// { 0x7f6f20 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_selected_list(lista, tecnico)`, pelas telas
/// de busca (0x804460). Grava a lista escolhida (byte) em +0x4c + índice do técnico humano e devolve 1. Sem técnico ou
/// fora de 0..15: a original (caixa "Error", 0).
unsafe extern "thiscall" fn escolher_lista_do_tecnico(this: usize, lista: u32, tecnico: usize) -> u32 {
    let Some(i) = cpy4_indice(tecnico) else {
        return orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(1355)(this, lista, tecnico); // caixa: só a original
    };
    *((this + 0x4c + i) as *mut u8) = lista as u8;
    1
}

/// Comparador: lado a lado pelos 16 bytes das listas escolhidas, e a devolução; nos casos de erro, só a original.
unsafe extern "thiscall" fn comparar_escolher_lista_do_tecnico(this: usize, lista: u32, tecnico: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(1355);
    if cpy4_indice(tecnico).is_none() {
        return o(this, lista, tecnico);
    }
    let reg = [(this + 0x4c, 0x10)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, lista, tecnico), || escolher_lista_do_tecnico(this, lista, tecnico));
    PLACAR.registrar(igual && r == n, || format!("escolher_lista_do_tecnico(lista {}, tecnico {:x}) original={} rust={} {:x?} x {:x?}", lista as u8, tecnico, r, n, mo, mr));
    r
}

// ---------- 255a leva: o jogo com jogador observado, a lista do olheiro do humano e o registro da primeira convocação (scout_manager.cpp, staff_records.cpp) ----------

unsafe fn cps8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7f9720 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::fixture_has_scouted_player(partida)` (pela
/// partida_sem_humano 0x6b66a0): competição (+0) negativa, 0. Um clube da lista vigiada (LLIST estática 0xb74438,
/// cursor reposto; +0 do item) igual a +0xc ou +0x10 da partida, 1. Depois os pedidos de observação ([this+0x5c],
/// 0x11 bytes, a pessoa em +4, conta [this+0x64], relidos a cada passo): a pessoa cujo clube (+0x39) ou seleção (+0x24)
/// é um dos dois da partida (+0x1c primeiro, depois +0x20), que não está cedida (0x8c7560, fora de seleção 0x53b900),
/// nem lesionada (0x62c400 = 2), nem suspensa (0x55d270), nem impedida (0x7757d0), e que ainda não jogou (0x8cf5e0
/// = 0), dá 1. Senão 0.
unsafe extern "thiscall" fn jogo_com_observado(this: usize, partida: usize) -> u32 {
    let (c1, c2) = (cps8_d(partida + 0x1c), cps8_d(partida + 0x20));
    if cps8_i(partida) < 0 {
        crate::comparacao::ramo("cps_8 competicao negativa");
        return 0; // cps8: competição negativa
    }
    core::ptr::write_unaligned(0xb74442 as *mut u32, cps8_d(0xb7443a) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(0xb74438);
    while e != 0 {
        let v = cps8_d(e);
        if cps8_d(partida + 0xc) == v || cps8_d(partida + 0x10) == v {
            return 1;
        }
        e = proximo(0xb74438);
    }
    if cps8_d(this + 0x5c) == 0 || cps8_i(this + 0x64) <= 0 {
        crate::comparacao::ramo("cps_8 sem pedidos");
        return 0; // cps8: sem pedidos de observação
    }
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x53b900usize);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7757d0usize);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    let mut k = 0i32;
    while k < cps8_i(this + 0x64) {
        let id = cps8_i(cps8_d(this + 0x5c).wrapping_add(k as usize * 0x11) + 4);
        k += 1;
        if id <= -1 {
            continue;
        }
        let p = cps8_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        let (clube, sel) = (cps8_d(p + 0x39), cps8_d(p + 0x24));
        let x = if c1 == clube || c1 == sel {
            c1
        } else if c2 == clube || c2 == sel {
            c2
        } else {
            continue;
        };
        if selecao(x) == 0 && cedido(cps8_d(0xde1f54), p, x, 0) != 0 {
            continue;
        }
        if lesao(cps8_d(0xb63d28), p) == 2 {
            continue;
        }
        if suspenso(cps8_d(0xae28f0), p, cps8_d(partida + 0x14), 1, 0) != 0 {
            continue;
        }
        if impedido(cps8_d(0xb74318), p, 0xae2c90, x, 0, 0) != 0 {
            continue;
        }
        if ja_jogou(cps8_d(0xde1f54), p, partida) == 0 {
            return 1;
        }
    }
    0
}

unsafe extern "thiscall" fn comparar_jogo_com_observado(this: usize, partida: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1137)(this, partida);
    let n = jogo_com_observado(this, partida);
    PLACAR.registrar(r == n, || format!("jogo_com_observado(partida {:x}) original={} rust={}", partida, r, n));
    r
}

/// O gerente do humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
unsafe fn cps8_gerente_da_vez() -> usize {
    let k = core::ptr::read_unaligned(0xb77c7a as *const i16) as isize;
    cps8_d((0xb74c26isize + 0x300 * k) as usize)
}

/// { 0x7f7490 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::is_player_shortlisted(pessoa, gerente)` (pelos
/// clubes interessados 0x842180, pelo estatuto 0x849d60, pela 0x8cde70 e por telas): pessoa nula, a caixa "Error"
/// (linha 0x231) e 0; sem gerente, o do humano da vez (sem ele, a caixa da linha 0x23b e 0). A vaga do humano (byte
/// +0 do gerente − byte de [0xae2364] + 16, com sinal, 0..15): com a edição aberta ([this + vaga × 4 + 0xc]), a
/// resposta dela (0x804540); senão a primeira das 10 buscas (0x7f6cd0) de tipo 1 (+0xa) responde (0x7af600). Sem ela, 0.
unsafe extern "thiscall" fn na_lista_do_olheiro(this: usize, pessoa: usize, gerente: usize) -> u32 {
    if pessoa == 0 {
        aviso_em(0xa7b094, 0x231);
        return 0;
    }
    let m = if gerente != 0 { gerente } else { cps8_gerente_da_vez() };
    if m == 0 {
        aviso_em(0xa7b094, 0x23b);
        return 0;
    }
    let vaga = (*(m as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        return 0;
    }
    let s = cps8_d(this + vaga as usize * 4 + 0xc);
    if s != 0 {
        let da_edicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x804540usize);
        return da_edicao(s, pessoa);
    }
    // ponytail: a original empurra um dword local com o índice só no byte baixo; a 0x7f6cd0 lê só esse byte
    let busca: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let tem: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7af600usize);
    for i in 0..10u32 {
        let l = busca(this, i, m);
        if l != 0 && *((l + 0xa) as *const u8) == 1 {
            let r = tem(l, pessoa);
            crate::comparacao::ramo("cps_8 lista do olheiro");
            return r; // cps8: a resposta da lista do olheiro
        }
    }
    0
}

unsafe extern "thiscall" fn comparar_na_lista_do_olheiro(this: usize, pessoa: usize, gerente: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1138);
    if pessoa == 0 || (gerente == 0 && cps8_gerente_da_vez() == 0) {
        return o(this, pessoa, gerente);
    }
    let r = o(this, pessoa, gerente);
    let n = na_lista_do_olheiro(this, pessoa, gerente);
    PLACAR.registrar(r == n, || format!("na_lista_do_olheiro(pessoa {:x}, gerente {:x}) original={} rust={}", pessoa, gerente, r, n));
    r
}

/// { 0x86cc50 } (thiscall, ret 8, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_first_cap_record(pessoa, adversário)` (pelo resultado internacional 0x76b600): o histórico da
/// pessoa (tipo 6: o clube contratante +0x39 ou −1, o id do adversário na partida e a idade +0x18 com sinal), no arquivo (0x58e340).
/// O clube é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn registrar_a_primeira_convocacao(this: usize, pessoa: usize, adversario: usize) {
    crate::comparacao::ramo("cps_8 primeira convocacao");
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 6;
    cpq4_por(&mut e, 8, cpq4_clube_de(pessoa));
    cpq4_por(&mut e, 0xd, cps8_i(adversario));
    cpq4_por(&mut e, 0x11, *((pessoa + 0x18) as *const i8) as i32);
    if pessoa != 0 {
        cpq4_historico(true, this, cps8_d(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_a_primeira_convocacao(this: usize, pessoa: usize, adversario: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1139)(this, pessoa, adversario)
}

// ---------- centesima sexagesima sexta leva: o observador do humano, o humano no clube e a categoria do contrato ----------

/// { 0x7f6cd0 } (thiscall, ret 8, scout_manager.cpp): o registro de observação `qual` 0..9 do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]): o índice (id −
/// [0xae2364] + 16) × 10 + `qual`, até [this+8] − 1, com 0x2b0 bytes cada a partir de [this].
/// `qual` fora, sem técnico ou índice fora: a original (caixa de erro).
unsafe extern "thiscall" fn registro_do_observador(t: usize, qual: i8, tecnico: *const i32) -> usize {
    let mut p = tecnico;
    if (0..=9).contains(&qual) && p.is_null() {
        let vez = *(0xb77c7a as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    if !(0..=9).contains(&qual) || p.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(370)(t, qual, tecnico);
    }
    let i = ((*p).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10)).wrapping_mul(10).wrapping_add(qual as i32);
    if i < 0 || i > (*((t + 8) as *const i32)).wrapping_sub(1) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(370)(t, qual, tecnico);
    }
    crate::comparacao::ramo("registro do observador");
    (*(t as *const usize)).wrapping_add(i as usize * 0x2b0)
}

/// { 0x600c90 } (cdecl, human_manager.cpp): o técnico humano da vez manda no `clube` (ou, nas
/// segundas seleções, na seleção principal da nação, 0x53b3d0)? Precisa ser o técnico do clube
/// (+0xcf); sem `qualquer`, não pode ter as marcas +0x9ace (0x5fe040) nem +0x9a01 no registro de
/// humanos [0xb63c98]; e então basta `forcar`, o clube ser seleção (0x53b900) ou a marca +0x9a05
/// (0x5fd480). Clube nulo: a original (caixa de erro e 0).
unsafe extern "C" fn humano_manda_no_clube(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let tecnico_9ace: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fe040usize);
    let tecnico_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let e_segunda_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao_principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let mut c = clube;
    loop {
        if c == 0 {
            return orig::<unsafe extern "C" fn(usize, u32, i8) -> u32>(371)(0, qualquer, forcar);
        }
        let vez = *(0xb77c7a as *const i16) as isize;
        let humano = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const usize);
        let tecnico = core::ptr::read_unaligned((c + 0xcf) as *const usize);
        'proximo: {
            if tecnico == 0 || humano != tecnico {
                break 'proximo;
            }
            let humanos = *(0xb63c98 as *const usize);
            if qualquer == 0 {
                if tecnico_9ace(humanos, humano) != 0 {
                    break 'proximo;
                }
                let i = (*(humano as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
                if i >= 0
                    && i <= (*((humanos + 4) as *const i32)).wrapping_sub(1)
                    && core::ptr::read_unaligned((*(humanos as *const usize) + i as usize * 0xa6ef + 0x9a01) as *const u32) != 0
                {
                    break 'proximo;
                }
            }
            if forcar != 0 || e_selecao(c) != 0 || tecnico_9a05(humanos, humano) != 0 {
                crate::comparacao::ramo("humano manda no clube");
                return 1;
            }
        }
        if e_segunda_selecao(c) == 0 {
            return 0;
        }
        c = selecao_principal(core::ptr::read_unaligned((c + 0x53) as *const usize));
    }
}

/// { 0x4ca470 } (stdcall, ret 8, contract_manager.cpp): a categoria 1..8 do jogador `p` no contrato
/// `c`: sem ficha (+0x61), 7; sem contrato, 4; contrato de tipo 5 (+0x35 & 0x3f), 8; senão pela
/// situação (+0x4f >> 4, 0..6; acima, 7), a satisfação (+0x3d, com sinal), a idade (+0x18), a
/// ambição (+0x57) e a reputação da ficha (+0xb, +0xd).
unsafe extern "stdcall" fn categoria_do_contrato(p: usize, c: usize) -> u8 {
    if p == 0 {
        return 7;
    }
    let ficha = core::ptr::read_unaligned((p + 0x61) as *const usize);
    if ficha == 0 {
        return 7;
    }
    if c == 0 {
        return 4;
    }
    if *((c + 0x35) as *const u8) & 0x3f == 5 {
        return 8;
    }
    let k = *((c + 0x4f) as *const u8) >> 4;
    let v = *((c + 0x3d) as *const i8);
    let idade = *((p + 0x18) as *const i8);
    let ambicao = *((p + 0x57) as *const i8);
    let rep = |o: usize| core::ptr::read_unaligned((ficha + o) as *const i16);
    let baixo = |limite: i8| if v <= limite { 7 } else { 4 };
    match k {
        0 => {
            if idade > 0x15 {
                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { baixo(-30) }
            } else if v > 0 {
                5
            } else if v > -30 || ambicao > 5 {
                6
            } else {
                7
            }
        }
        1 => if v > 0x2d { 1 } else { (v <= 0x19) as u8 + 2 },
        2 => {
            let nota = ambicao as i32 * 25 - *((p + 0x59) as *const i8) as i32 * 10 + rep(0xd) as i32;
            if v > 0x4b && nota > 0x186a && idade > 0x17 && idade < 0x22 {
                1
            } else if v > 0x19 {
                2
            } else {
                (v <= 0) as u8 + 3
            }
        }
        3 => if v > 0x37 && idade > 0x17 && idade < 0x22 { 2 } else { (v <= 0) as u8 + 3 },
        4 => {
            if v > 0x41 && idade > 0x17 {
                (idade < 0x22) as u8 + 2
            } else if v > -30 {
                4
            } else if idade >= 0x18 {
                7
            } else {
                5
            }
        }
        5 => {
            if idade > 0x18 {
                if idade > 0x22 { 4 } else if v > 0x4b { 2 } else if v > 0 { 3 } else { baixo(-30) }
            } else if ambicao > 5 && rep(0xb) > 0x4e2 {
                5
            } else {
                6
            }
        }
        6 => {
            if idade > 0x17 {
                if v > 0x4b { 1 } else if v > 0x1e { 2 } else if v > 0 { 3 } else { baixo(-50) }
            } else if rep(0xb) > 0x4e2 && ambicao > 0xf {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

unsafe extern "thiscall" fn comparar_registro_do_observador(t: usize, qual: i8, tecnico: *const i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(370)(t, qual, tecnico);
    let n = registro_do_observador(t, qual, tecnico);
    PLACAR.registrar(r == n, || format!("registro_do_observador({:#x}, {}, {:p}) original={:#x} rust={:#x}", t, qual, tecnico, r, n));
    r
}

unsafe extern "C" fn comparar_humano_manda_no_clube(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32, i8) -> u32>(371)(clube, qualquer, forcar);
    let n = humano_manda_no_clube(clube, qualquer, forcar);
    PLACAR.registrar(r == n, || format!("humano_manda_no_clube({:#x}, {}, {}) original={} rust={}", clube, qualquer, forcar, r, n));
    r
}

unsafe extern "stdcall" fn comparar_categoria_do_contrato(p: usize, c: usize) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u8>(372)(p, c);
    let n = categoria_do_contrato(p, c);
    PLACAR.registrar(r == n, || format!("categoria_do_contrato({:#x}, {:#x}) original={} rust={}", p, c, r, n));
    r
}

