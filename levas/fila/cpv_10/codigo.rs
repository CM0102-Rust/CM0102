// ---------- %%LEVA%%a leva: o fim da lesão do jogador, a notícia do pedido de desculpas e a caixa de entrada esvaziada (injury.cpp, media.cpp, news.cpp) ----------

unsafe fn cpv10_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// O registro de 31 bytes da pessoa `id` em [lesoes] (o físico e a lesão atual em +0xe).
unsafe fn cpv10_registro(lesoes: usize, id: i32) -> usize {
    (cpv10_d(lesoes) as u32).wrapping_add((id as u32).wrapping_mul(31)) as usize
}

/// { 0x62e700 } (thiscall, ret 8, injury.cpp): `Injury::end_player_injury(pessoa, noticia)`, pelo físico do dia
/// (0x62c680, com notícia), pelas batidas da partida (0x62c080, sem) e pelo recomeço do físico (0x62bfa0, sem): com a
/// lesão atual (+0xe do registro, índice na tabela de 0x30 bytes em [this+4]; −1 nenhuma): fora das férias (tipo
/// 0x51), o histórico ([this+0xc], 0x58e340 com +0x14 e +0x18 do registro); com `noticia` e o clube da pessoa (+0x39)
/// humano (0x600a50), a notícia da volta (0x630ee0 com 1 e o tipo). O tipo vira 0xff e o +0xe −1. Sem pessoa: a caixa
/// de erro (linha 0xc87).
unsafe extern "thiscall" fn end_player_injury(this: usize, pessoa: usize, noticia: u32) {
    if pessoa == 0 {
        aviso_em(0x9e7f28, 0xc87);
        return;
    }
    let id = cpv10_d(pessoa);
    if id >= cpv10_d(0xae2364) { // cpv10: o id da pessoa
        return;
    }
    if id >= cpv10_d(0xae2364) / 2 {
        crate::comparacao::ramo("cpv_10 metade de cima");
    }
    let r = cpv10_registro(this, id);
    let i = cpv10_d(r + 0xe);
    if i == -1 { // cpv10: a lesão atual
        return;
    }
    if i == 0 {
        crate::comparacao::ramo("cpv_10 entrada zero");
    }
    let e = (cpv10_d(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    if *(e as *const u8) != 0x51 {
        let historico: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
        historico(cpv10_d(this + 0xc) as usize, r + 0x14, r + 0x18, e as *const u8);
    }
    let clube = cpv10_d(pessoa + 0x39) as usize;
    if noticia != 0 && clube != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) != 0 {
            let volta: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x630ee0usize);
            // ponytail: a original empurra o ecx inteiro com o tipo no byte de baixo; a 0x630ee0 lê o byte
            volta(this, pessoa, 1, *(e as *const u8) as u32, 0, 0);
        }
    }
    crate::comparacao::ramo("cpv_10 fim da lesao");
    *(e as *mut u8) = 0xff; // cpv10: a entrada fica livre
    core::ptr::write_unaligned((cpv10_registro(this, cpv10_d(pessoa)) + 0xe) as *mut i32, -1); // cpv10: sem lesão atual
}

/// Comparador: lado a lado na entrada (tipo) e no +0xe do registro quando nada sai da rotina (férias e sem notícia);
/// com o histórico (arquivo) ou a notícia: só a original, conferindo o plano (tipo 0xff e +0xe −1). Sem pessoa: a
/// original (caixa).
unsafe extern "thiscall" fn comparar_end_player_injury(this: usize, pessoa: usize, noticia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    if pessoa == 0 || cpv10_d(pessoa) >= cpv10_d(0xae2364) {
        return o(this, pessoa, noticia);
    }
    let r = cpv10_registro(this, cpv10_d(pessoa));
    let i = cpv10_d(r + 0xe);
    if i == -1 {
        return o(this, pessoa, noticia);
    }
    let e = (cpv10_d(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let clube = cpv10_d(pessoa + 0x39) as usize;
    let fora = *(e as *const u8) != 0x51 || (noticia != 0 && clube != 0 && humano(clube, 0, 1, 0, 0) != 0);
    if fora {
        o(this, pessoa, noticia);
        let ok = *(e as *const u8) == 0xff && cpv10_d(cpv10_registro(this, cpv10_d(pessoa)) + 0xe) == -1;
        PLACAR.registrar(ok, || format!("end_player_injury({:x}, {}) plano: tipo {:x} e +0xe {}", pessoa, noticia, *(e as *const u8), cpv10_d(cpv10_registro(this, cpv10_d(pessoa)) + 0xe)));
        return;
    }
    let reg = [(e, 1), (r + 0xe, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, noticia), || end_player_injury(this, pessoa, noticia));
    PLACAR.registrar(igual, || format!("end_player_injury({:x}, {}) {:x?} x {:x?}", pessoa, noticia, mo, mr));
}

/// { 0x72bbf0 } (stdcall, ret 0xc, media.cpp, quadro SEH sem destrutor): `media_queue_player_apology_news(jogador,
/// clube, historia)`, pelos efeitos da história (0x736c00) e pela resposta à história do jogador (0x741020): com o
/// jogador, a ficha (+0x61) e o clube, a notícia 0xbdb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os
/// nomes (+0x33 de +4, +8 e +0xc; sem o terceiro, −1), 4 = id do clube, 5 = a história, 6 = 0 e 7 = a variante:
/// sorteio(4) nas histórias 0, 2 a 5, 13, 14, 17 e 35 (tabela 0x72bd2c), senão 0; vai para o técnico do clube
/// (0x7861e0).
unsafe extern "stdcall" fn media_queue_player_apology_news(jogador: usize, clube: usize, historia: i16) {
    if jogador == 0 || cpv10_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    crate::comparacao::ramo("media_queue_player_apology_news");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdb, 0);
    gravar(n, 0, cpv10_d(jogador) as u32);
    gravar(n, 1, cpv10_d(cpv10_d(jogador + 4) as usize + 0x33) as u32);
    gravar(n, 2, cpv10_d(cpv10_d(jogador + 8) as usize + 0x33) as u32);
    let terceiro = cpv10_d(jogador + 0xc) as usize;
    gravar(n, 3, if terceiro == 0 { u32::MAX } else { cpv10_d(terceiro + 0x33) as u32 });
    gravar(n, 4, cpv10_d(clube) as u32);
    let h = historia as i32;
    gravar(n, 5, h as u32);
    gravar(n, 6, 0);
    let v = if (h as u32) <= 0x23 && *((0x72bd2c + h as usize) as *const u8) == 0 { sorteio(4) } else { 0 };
    gravar(n, 7, v as u32);
    ao_tecnico(cpv10_d(0xb7431c) as usize, n, clube);
}

/// Comparador: a notícia sorteia e vai para a caixa do técnico: só a original (a prova é a carreira e o modo Rust).
unsafe extern "stdcall" fn comparar_media_queue_player_apology_news(jogador: usize, clube: usize, historia: i16) {
    orig::<unsafe extern "stdcall" fn(usize, usize, i16)>(%%I1%%)(jogador, clube, historia)
}

/// { 0x787630 } (thiscall, ret 4, news.cpp): `News::clear_inbox(tecnico)`, pela 0x6b28b0 e pela carga do jogo
/// (0x82a9c0): as respostas padrão das notícias do clube (+0x39) e da seleção (+0x24) do técnico (0x7874b0); depois a
/// caixa do humano (registro de 12 bytes em [this+8], pelo id − [0xae2364] + 16): +4 = +0 e +8 = 0; com a primeira
/// notícia (+0, −1 nenhuma), as 100 notícias de 0xdf bytes a partir dela ([this]) com +0xd6 = 0 e +0xda = 1. Sem técnico
/// ou com índice negativo: a caixa de erro (linhas 0x24c1 e 0x24cc).
unsafe extern "thiscall" fn clear_inbox(this: usize, tecnico: usize) {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x24c1);
        return;
    }
    let h = cpv10_d(tecnico).wrapping_sub(cpv10_d(0xae2364)).wrapping_add(0x10);
    if h < 0 {
        aviso_em(0xa693ec, 0x24cc);
        return;
    }
    let caixa = (cpv10_d(this + 8) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    let respond: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7874b0usize);
    for o in [0x39usize, 0x24] {
        let c = cpv10_d(tecnico + o) as usize;
        if c != 0 {
            respond(this, tecnico, c);
        }
    }
    let primeira = cpv10_d(caixa);
    core::ptr::write_unaligned((caixa + 8) as *mut i32, 0);
    core::ptr::write_unaligned((caixa + 4) as *mut i32, primeira);
    if primeira != -1 {
        crate::comparacao::ramo("clear_inbox");
        for k in 0..0x64u32 {
            let n = (cpv10_d(this) as u32).wrapping_add(k.wrapping_add(cpv10_d(caixa) as u32).wrapping_mul(0xdf)) as usize;
            core::ptr::write_unaligned((n + 0xd6) as *mut i32, 0);
            core::ptr::write_unaligned((n + 0xda) as *mut i32, 1);
        }
    }
}

/// As regiões que a 0x787630 grava (a caixa e, com a primeira notícia, +0xd6..+0xde das 100 notícias).
unsafe fn cpv10_regioes_da_caixa(this: usize, caixa: usize) -> Vec<(usize, usize)> {
    let mut reg = vec![(caixa, 12)];
    let primeira = cpv10_d(caixa);
    if primeira != -1 {
        for k in 0..0x64u32 {
            reg.push(((cpv10_d(this) as u32).wrapping_add(k.wrapping_add(primeira as u32).wrapping_mul(0xdf)) as usize + 0xd6, 8));
        }
    }
    reg
}

/// Comparador: lado a lado na caixa e nas 100 notícias; com resposta padrão a dar (clube ou seleção), só a original,
/// conferindo o plano depois dela (+4 = +0, +8 = 0 e os dois campos das 100 notícias). Erros: a original.
unsafe extern "thiscall" fn comparar_clear_inbox(this: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    if tecnico == 0 {
        return o(this, tecnico);
    }
    let h = cpv10_d(tecnico).wrapping_sub(cpv10_d(0xae2364)).wrapping_add(0x10);
    if h < 0 {
        return o(this, tecnico);
    }
    let caixa = (cpv10_d(this + 8) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    if cpv10_d(tecnico + 0x39) != 0 || cpv10_d(tecnico + 0x24) != 0 {
        o(this, tecnico);
        let primeira = cpv10_d(caixa);
        let noticias = cpv10_regioes_da_caixa(this, caixa);
        let ok = cpv10_d(caixa + 4) == primeira && cpv10_d(caixa + 8) == 0
            && noticias[1..].iter().all(|&(n, _)| cpv10_d(n) == 0 && cpv10_d(n + 4) == 1);
        PLACAR.registrar(ok, || format!("clear_inbox({:x}) plano: caixa {:x?}", tecnico, (primeira, cpv10_d(caixa + 4), cpv10_d(caixa + 8))));
        return;
    }
    let reg = cpv10_regioes_da_caixa(this, caixa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico), || clear_inbox(this, tecnico));
    PLACAR.registrar(igual, || format!("clear_inbox({:x}) {:x?} x {:x?}", tecnico, mo, mr));
}
