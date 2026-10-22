// ---------- 213a leva: o fim do empréstimo, a contratação pelas regras da nação e as trocas que oferecem o jogador (transfer_manager.cpp) ----------

/// As regras de transferência da nação: das 28 em [gerente+0x8ac] (ponteiros), a de nação (+0xa) igual;
/// 0 se nenhuma.
unsafe fn regras_de_transferencia_da_nacao(gerente: usize, nacao: usize) -> usize {
    (0..28)
        .map(|i| *((gerente + 0x8ac + 4 * i) as *const usize))
        .find(|&r| core::ptr::read_unaligned((r + 0xa) as *const usize) == nacao)
        .unwrap_or(0)
}

/// { 0x8ced40 } (thiscall, ret 0x18, transfer_manager.cpp): se o `clube` pode contratar o `jogador`,
/// pelas regras da nação do clube (método +0x10 com clube, jogador, a4, a2 e o motivo); sem regras, 1. O
/// motivo (palavra) vai para `saida` (zerada antes). Recusado com `a5`, clube humano (0x600a50) e motivo
/// de estrangeiros (0x11, 0x14, 0x15, 0x1f): pergunta de novo com a nação do jogador (+0x1a) trocada pela
/// do clube (e devolvida depois); aceito assim, 1.
unsafe extern "thiscall" fn pode_contratar_jogador(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let regras = regras_de_transferencia_da_nacao(gerente, core::ptr::read_unaligned((clube + 0x53) as *const usize));
    if !saida.is_null() {
        *saida = 0;
    }
    if regras == 0 {
        return 1;
    }
    let consultar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut u32) -> u32 =
        core::mem::transmute(*((*(regras as *const usize) + 0x10) as *const usize));
    // o motivo é a vaga do argumento `clube` na pilha da original: sem resposta das regras fica o clube
    let mut motivo = clube as u32;
    let r = consultar(regras, clube, jogador, a4, a2, &mut motivo);
    if !saida.is_null() {
        *saida = motivo as u16;
    }
    if r != 0 {
        return r;
    }
    if a5 == 0 {
        return 0; // recusado, sem segunda pergunta
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) == 0 || !matches!(motivo as u16, 0x11 | 0x14 | 0x15 | 0x1f) {
        if !saida.is_null() {
            *saida = motivo as u16;
        }
        return 0;
    }
    let nacao = (jogador + 0x1a) as *mut u32;
    let guardada = core::ptr::read_unaligned(nacao);
    core::ptr::write_unaligned(nacao, core::ptr::read_unaligned((clube + 0x53) as *const u32));
    let r = consultar(regras, clube, jogador, a4, a2, &mut motivo);
    core::ptr::write_unaligned(nacao, guardada);
    if r != 0 {
        return 1;
    }
    if !saida.is_null() {
        *saida = motivo as u16;
    }
    0
}

/// { 0x8c5e40 } (thiscall, ret 8, transfer_manager.cpp): quantas ofertas abertas do clube do contrato do
/// `jogador` (0x4cf5a0) o oferecem na troca (lista +4 da oferta, nó com o id dele), anotando o índice
/// (+0x20) em `saida` (0x68d0d0) se houver. Ofertas no estado (+0x2c) 2, 3, 4, 0x10, 0x14 ou 0xf com
/// +0x2e ≥ 3 não contam. Para ao chegar ao número de trocas marcado ([+0x86e] + 8 × id + 2); marcado 0
/// ou id fora da faixa, 0.
unsafe extern "thiscall" fn trocas_oferecendo_o_jogador(gerente: usize, jogador: usize, saida: usize) -> u32 {
    if jogador != 0 && *(jogador as *const i32) >= *(0xae2364 as *const i32) {
        return 0;
    }
    let limite = || *((*((gerente + 0x86e) as *const usize) as isize + *(jogador as *const i32) as isize * 8 + 2) as *const u8) as u32;
    if limite() == 0 {
        return 0; // sem trocas marcadas
    }
    if clube_do_contrato(jogador) == 0 {
        return 0;
    }
    let clube = *(clube_do_contrato(jogador) as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let mut conta = 0u32;
    let mut i = 0;
    while i < *((gerente + 4) as *const i32) {
        let vaga = (*(gerente as *const usize) + 4 * i as usize) as *const usize;
        i += 1;
        let oferta = *vaga;
        if oferta == 0 || *((oferta + 8) as *const u32) != clube {
            continue;
        }
        let estado = *((oferta + 0x2c) as *const u8);
        if matches!(estado, 2 | 3 | 4 | 0x10 | 0x14) || (estado == 0xf && *((oferta + 0x2e) as *const u8) >= 3) {
            continue;
        }
        let l = *((oferta + 4) as *const usize);
        if l != 0 {
            core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
            loop {
                let no = prox(*((*vaga + 4) as *const usize));
                if no == 0 {
                    break;
                }
                if *(no as *const u32) == *(jogador as *const u32) {
                    if saida != 0 {
                        anexar(saida, *vaga + 0x20, 4);
                    }
                    conta += 1; // trocas
                    break;
                }
            }
        }
        if conta == limite() {
            break;
        }
    }
    conta
}


unsafe extern "thiscall" fn comparar_pode_contratar_jogador(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, *mut u16) -> u32>(526);
    let mut regioes = vec![];
    if !saida.is_null() {
        regioes.push((saida as usize, 2));
    }
    if jogador != 0 {
        regioes.push((jogador + 0x1a, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, clube, a2, jogador, a4, a5, saida),
        || pode_contratar_jogador(gerente, clube, a2, jogador, a4, a5, saida));
    PLACAR.registrar(igual && r == n, || format!("pode_contratar_jogador(a5 {}) original={} rust={} {:x?} x {:x?}", a5, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_trocas_oferecendo_o_jogador(gerente: usize, jogador: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(527);
    if saida != 0 {
        // ponytail: com lista de saída a original anexa (aloca); só a original
        return o(gerente, jogador, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if jogador != 0 && *(jogador as *const i32) < *(0xae2364 as *const i32) {
        let guardado = *(0xae1080 as *const usize);
        let p = clube_do_contrato(jogador);
        *(0xae1080 as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das ofertas do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            for i in 0..(*((gerente + 4) as *const i32)).max(0) as usize {
                let oferta = *((*(gerente as *const usize) + 4 * i) as *const usize);
                if oferta != 0 && *((oferta + 8) as *const u32) == clube && *((oferta + 4) as *const usize) != 0 {
                    regioes.push((*((oferta + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(gerente, jogador, saida),
        || trocas_oferecendo_o_jogador(gerente, jogador, saida));
    PLACAR.registrar(igual && r == n, || format!("trocas_oferecendo_o_jogador original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 213a leva: as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano (news.cpp) ----------

/// A entrada (12 bytes) da caixa de notícias do técnico humano: [noticias+8] + 12 × (id − [0xae2364] +
/// 0x10); None quando o índice fica negativo (a original abre a caixa de erro).
unsafe fn entrada_da_caixa_do_tecnico(noticias: usize, tecnico: usize) -> Option<usize> {
    let i = (*(tecnico as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 {
        return None;
    }
    Some((*((noticias + 8) as *const usize)).wrapping_add((i as usize).wrapping_mul(12)))
}

/// A notícia `k` da caixa que começa em `inicio`: [noticias] + 0xdf × (inicio + k).
unsafe fn noticia_da_caixa(noticias: usize, inicio: i32, k: i32) -> usize {
    (*(noticias as *const usize)).wrapping_add((inicio.wrapping_add(k) as usize).wrapping_mul(0xdf))
}

/// { 0x786fe0 } (thiscall, ret 8, news.cpp): as notícias não lidas do técnico humano. Sem categoria (0):
/// o total da entrada (+8, negativo vira 0). Com categoria: das 100 notícias da caixa (início na entrada
/// +0; −1 = vazia, 0), as ocupadas (+0xd6) da categoria (0x785670) ainda não lidas (+0xda = 0). Técnico
/// nulo ou fora da faixa: a original (caixa de erro).
unsafe extern "thiscall" fn noticias_nao_lidas(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let entrada = match if tecnico == 0 { None } else { entrada_da_caixa_do_tecnico(noticias, tecnico) } {
        Some(e) => e,
        None => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(528)(noticias, tecnico, categoria),
    };
    let total = (entrada + 8) as *mut i32;
    if *total < 0 {
        *total = 0;
    }
    if categoria as u8 == 0 {
        return *total; // total da caixa
    }
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return 0;
    }
    let categoria_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
    let mut conta = 0;
    for k in 0..100 {
        let n = noticia_da_caixa(noticias, inicio, k);
        if core::ptr::read_unaligned((n + 0xd6) as *const u32) != 0
            && categoria_de(n) as u8 == categoria as u8
            && core::ptr::read_unaligned((n + 0xda) as *const u32) == 0
        {
            conta += 1;
        }
    }
    conta
}

/// Onde a 0x786d30 grava: Err nos casos da caixa de erro (técnico nulo, id 0, vaga fora de 0..49, cópia
/// de outro id, índice negativo, id fora das 100 da caixa); Ok(None) com a caixa vazia (início −1);
/// Ok(Some(notícia)) achada.
unsafe fn alvo_da_marca(noticias: usize, tecnico: usize, id: u32, vaga: i32, copia: usize) -> Result<Option<usize>, ()> {
    if tecnico == 0 || id == 0 || !(0..0x32).contains(&vaga) {
        return Err(());
    }
    if copia != 0 && core::ptr::read_unaligned((copia + 0xd6) as *const u32) != id {
        return Err(());
    }
    let entrada = entrada_da_caixa_do_tecnico(noticias, tecnico).ok_or(())?;
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return Ok(None);
    }
    (0..100)
        .map(|k| noticia_da_caixa(noticias, inicio, k))
        .find(|&n| core::ptr::read_unaligned((n + 0xd6) as *const u32) == id)
        .map(Some)
        .ok_or(())
}

/// { 0x786d30 } (thiscall, ret 0x14, news.cpp): grava `valor` na vaga `vaga` (dword +5 + 4 × vaga) da
/// notícia `id` (+0xd6) da caixa do técnico humano, primeiro na `copia` (se houver) e depois na da caixa;
/// 1 se achou. Caixa vazia: só a cópia, 0. Casos de erro: a original.
unsafe extern "thiscall" fn gravar_marca_da_noticia(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let alvo = match alvo_da_marca(noticias, tecnico, id, vaga, copia) {
        Ok(a) => a,
        Err(()) => return orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(529)(noticias, tecnico, id, vaga, valor, copia),
    };
    if copia != 0 {
        core::ptr::write_unaligned((copia + 5 + 4 * vaga as usize) as *mut u32, valor);
    }
    match alvo {
        Some(n) => {
            core::ptr::write_unaligned((n + 5 + 4 * vaga as usize) as *mut u32, valor); // marca na caixa
            1
        }
        None => 0,
    }
}

/// { 0x7875e0 } (stdcall, ret 4, news.cpp): 1 se um humano dirige o `clube` (0x600a50 com férias
/// ignoradas); antes de devolver, consulta as férias (0x5fd210 em [0xb63c98]) do técnico do clube
/// (0x53ca90), sem usar a resposta. Clube nulo: 0.
unsafe extern "stdcall" fn clube_de_tecnico_humano(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let r = humano(clube, 1, 1, 0, 0);
    if r == 0 {
        return r;
    }
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let t = tecnico(clube);
    ferias(*(0xb63c98 as *const usize), t);
    1 // humano
}

unsafe extern "thiscall" fn comparar_noticias_nao_lidas(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(528);
    let entrada = if tecnico == 0 { None } else { entrada_da_caixa_do_tecnico(noticias, tecnico) };
    let Some(entrada) = entrada else {
        return o(noticias, tecnico, categoria);
    };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(entrada + 8, 4)], || o(noticias, tecnico, categoria),
        || noticias_nao_lidas(noticias, tecnico, categoria));
    PLACAR.registrar(igual && r == n, || format!("noticias_nao_lidas(categoria {}) original={} rust={} {:x?} x {:x?}", categoria as u8, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_gravar_marca_da_noticia(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(529);
    let Ok(alvo) = alvo_da_marca(noticias, tecnico, id, vaga, copia) else {
        return o(noticias, tecnico, id, vaga, valor, copia);
    };
    let mut regioes = vec![];
    if copia != 0 {
        regioes.push((copia + 5 + 4 * vaga as usize, 4));
    }
    if let Some(a) = alvo {
        regioes.push((a + 5 + 4 * vaga as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(noticias, tecnico, id, vaga, valor, copia),
        || gravar_marca_da_noticia(noticias, tecnico, id, vaga, valor, copia));
    PLACAR.registrar(igual && r == n, || format!("gravar_marca_da_noticia(vaga {}) original={} rust={} {:x?} x {:x?}", vaga, r, n, mo, mr));
    r
}

unsafe extern "stdcall" fn comparar_clube_de_tecnico_humano(clube: usize) -> u32 {
    // só consultas: as duas rodam de novo sem efeito
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(530)(clube);
    let n = clube_de_tecnico_humano(clube);
    PLACAR.registrar(r == n, || format!("clube_de_tecnico_humano original={} rust={}", r, n));
    r
}

// ---------- 213a leva: as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados (Cup.cpp, friendly.cpp) ----------

/// A data (Date::Date, 0x549f70, num Date local de 8 bytes) de `dia`/`mes`/`ano`, levada ao dia da
/// semana `semana` (−1 = qualquer): (palavra +0 = dia do ano, byte baixo do ano +2).
unsafe fn data_da_rodada(dia: u32, mes: u32, ano: u32, semana: u32) -> (u16, u8) {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    (d[0] as u16, (d[0] >> 16) as u8)
}

/// O registro de 104 bytes da rodada `rodada` (palavra com sinal) a partir de `rodadas`.
fn registro_da_rodada(rodadas: usize, rodada: u32) -> usize {
    rodadas.wrapping_add((rodada as i16 as isize * 104) as usize)
}

/// { 0x521e60 } (cdecl, Cup.cpp): a data do sorteio da rodada: dia do ano na palavra +0 e ano − ano-base
/// (byte) em +2, da data de `dia`/`mes`/ano-base + `anos` levada ao dia da semana. Devolve o registro.
unsafe extern "C" fn data_do_sorteio_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let (d, a) = data_da_rodada(dia, mes, base.wrapping_add(anos), semana);
    let r = registro_da_rodada(rodadas, rodada);
    core::ptr::write_unaligned(r as *mut u16, d);
    *((r + 2) as *mut u8) = a.wrapping_sub(base as u8); // ano do sorteio
    r
}

/// { 0x521eb0 } (cdecl, Cup.cpp): a data do jogo da rodada, como a do sorteio, em +3 (dia) e +5 (ano −
/// ano-base); o horário (byte) em +6 e as marcas do mando em +0x13. Devolve o registro.
unsafe extern "C" fn data_do_jogo_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let (d, a) = data_da_rodada(dia, mes, base.wrapping_add(anos), semana);
    let r = registro_da_rodada(rodadas, rodada);
    core::ptr::write_unaligned((r + 3) as *mut u16, d); // dia do jogo
    *((r + 5) as *mut u8) = a.wrapping_sub(base as u8);
    *((r + 6) as *mut u8) = hora as u8;
    core::ptr::write_unaligned((r + 0x13) as *mut u32, mando);
    r
}

/// { 0x5c64e0 } (thiscall, ret 4, friendly.cpp): tira o `clube` dos amistosos planejados: no vetor
/// [+0x34d8] ([+0x34e0] registros de 0x14 bytes) o id do clube em +8 vira −1; na lista [+0x34c8]
/// (0x68d1b0), o ponteiro do clube em +0xc vira 0.
unsafe extern "thiscall" fn tirar_clube_dos_amistosos(amistosos: usize, clube: usize) {
    let vetor = (amistosos + 0x34d8) as *const usize;
    let n = (amistosos + 0x34e0) as *const i32;
    if *vetor != 0 {
        let mut i = 0;
        while i < *n {
            let id = (*vetor + 8 + 0x14 * i as usize) as *mut u32;
            if *id == *(clube as *const u32) {
                *id = 0xffff_ffff;
            }
            i += 1;
        }
    }
    let lista = (amistosos + 0x34c8) as *const usize;
    if *lista == 0 {
        return;
    }
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((*lista + 0xa) as *mut u32, core::ptr::read_unaligned((*lista + 2) as *const u32));
    loop {
        let e = prox(*lista);
        if e == 0 {
            return;
        }
        if *((e + 0xc) as *const usize) == clube {
            *((e + 0xc) as *mut usize) = 0; // sai da lista
        }
    }
}

unsafe extern "C" fn comparar_data_do_sorteio_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(531);
    let r = registro_da_rodada(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 3)], || o(rodadas, rodada, dia, mes, anos, semana, base),
        || data_do_sorteio_da_rodada(rodadas, rodada, dia, mes, anos, semana, base));
    PLACAR.registrar(igual && a == b, || format!("data_do_sorteio_da_rodada({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "C" fn comparar_data_do_jogo_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(532);
    let r = registro_da_rodada(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 3, 0x14)], || o(rodadas, rodada, dia, mes, anos, semana, hora, base, mando),
        || data_do_jogo_da_rodada(rodadas, rodada, dia, mes, anos, semana, hora, base, mando));
    PLACAR.registrar(igual && a == b, || format!("data_do_jogo_da_rodada({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "thiscall" fn comparar_tirar_clube_dos_amistosos(amistosos: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(533);
    let mut regioes = vec![];
    let vetor = *((amistosos + 0x34d8) as *const usize);
    let n = *((amistosos + 0x34e0) as *const i32);
    if vetor != 0 && n > 0 {
        regioes.push((vetor, n as usize * 0x14));
    }
    let lista = *((amistosos + 0x34c8) as *const usize);
    if lista != 0 {
        // o cursor e o ponteiro +0xc de cada amistoso da lista (percorrida aqui antes; as duas
        // rotinas reiniciam o cursor)
        regioes.push((lista + 0xa, 4));
        let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
        loop {
            let e = prox(lista);
            if e == 0 {
                break;
            }
            regioes.push((e + 0xc, 4));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(amistosos, clube),
        || tirar_clube_dos_amistosos(amistosos, clube));
    PLACAR.registrar(igual, || format!("tirar_clube_dos_amistosos({:#x}) {:x?} x {:x?}", clube, mo, mr));
}
