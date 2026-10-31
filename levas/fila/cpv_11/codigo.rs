// ---------- %%LEVA%%a leva: o convocado machucado: tirar da seleção, a notícia ao clube dele e o pedido de substituto ao técnico da seleção (national_teams.cpp) ----------

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
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, p, dias_arg);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
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
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I1%%)(this, p, nacao, tipo)
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
unsafe extern "thiscall" fn comparar_noticia_do_pedido_de_substituto(this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    crate::comparacao::ramo("noticia_do_pedido_de_substituto");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%)(this, p, nacao, tipo, nivel)
}
