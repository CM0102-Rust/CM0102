// ---------- %%LEVA%%a leva: a pessoa liberada sem custo, as férias do técnico humano e a notícia da permissão para discutir o contrato (transfer_manager.cpp, human_manager.cpp) ----------

unsafe fn cpy13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy13_clube(id: usize) -> usize {
    cpy13_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpy13_pessoa(id: usize) -> usize {
    cpy13_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpy13_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpy13_d(0xae1050), p)
}

/// Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): a data de hoje ([0xae2c90]) no destino.
unsafe fn cpy13_hoje_em(destino: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), 0xae2c90);
}

/// { 0x8c1af0 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::free_transfer_staff(pessoa, clube, motivo,
/// compensação, só_registro)`, pela fila das transferências livres (0x8c18c0), pelo fim dos contratos (0x4c7ac0) e pela
/// 0x8cd450. Pessoa emprestada (0x4cf6d0): nada. A transferência passada (0x25 bytes no vetor [this+0x862], capacidade
/// +0x866, conta +0x86a; cheio, cresce de 100 em 100; sem memória: a caixa 0xce9 e nada): +0x10 −1, +0x14 o clube ou −1,
/// a data de hoje em +0x1d, a pessoa (o id e as três chaves), +0x1c 5 (só registro) ou 3, +0x18 0. Só registro: acabou.
/// Senão: com ficha (+0x61), o histórico (0x7c5fe0 em [0xb74350] com o motivo); com clube, a compensação (−1: com
/// contrato do tipo 2, os dias até o fim (0x85fa00) × o salário (+0xc) / 7; senão 0) paga ao clube (0x599920), a situação
/// nas listas de observação (0x843e10, evento 4, tipo 2) e, no clube atual (+0x39), as duas escalações (0x896c60 em
/// [0xde1d08]) e o elenco (0x53c190), com a data +0x3e de hoje; as ofertas de contrato do clube do contrato (0x4cca70,
/// contrato guardado em [0xae1080]), o contrato principal (0x4d0b50), +0x52 e +0x6d zerados, as três cláusulas (0x8cb4f0,
/// 0x8cb590, 0x8cb630), a permissão de trabalho (0x8c7e20 sem clube) e 0x4196a0 em [0xad9c6c].
unsafe extern "thiscall" fn free_transfer_staff(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if ficha(cpy13_d(0xae1050), p) != 0 {
        return;
    }
    if !cpl3_crescer(this, 0x862, 4, 100, 0x25, 0xce9, 0xa8e998) {
        return;
    }
    let n = cpy13_d(this + 0x86a);
    let r = cpy13_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    core::ptr::write_unaligned((this + 0x86a) as *mut usize, n.wrapping_add(1));
    let por = |o: usize, v: u32| core::ptr::write_unaligned((r + o) as *mut u32, v);
    por(0x10, u32::MAX);
    por(0x14, if clube != 0 { cpy13_d(clube) as u32 } else { u32::MAX });
    cpy13_hoje_em(r + 0x1d);
    por(0, cpy13_d(p) as u32);
    por(4, cpy13_d(cpy13_d(p + 4) + 0x33) as u32);
    por(8, cpy13_d(cpy13_d(p + 8) + 0x33) as u32);
    por(0xc, cpy13_d(cpy13_d(p + 0xc) + 0x33) as u32);
    *((r + 0x1c) as *mut u8) = if so_registro != 0 { 5 } else { 3 };
    por(0x18, 0);
    if so_registro != 0 {
        return;
    }
    crate::comparacao::ramo("cpy_13 liberado");
    if cpy13_d(p + 0x61) != 0 {
        let historico: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32) = core::mem::transmute(0x7c5fe0usize);
        historico(cpy13_d(0xb74350), p, 0, 0, -1, motivo, 0);
    }
    if clube != 0 {
        let c = cpy13_contrato(p);
        let mut comp = compensacao;
        if comp == -1 {
            comp = if c != 0 && cpy13_b(c + 0x35) & 0x3f == 2 {
                let dias: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(0x85fa00usize);
                (dias(c) as i32).wrapping_mul(cpy13_d(c + 0xc) as i32) / 7
            } else {
                0
            };
        }
        let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
        let pagar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x599920usize);
        let fin = financas(cpy13_d(0xae2a38), clube);
        if comp != 0 {
            crate::comparacao::ramo("cpy_13 compensacao paga");
        }
        pagar(fin, comp); // cpy13: a compensação ao clube
        let listas: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
        listas(cpy13_d(this + 0x856), p, clube, 4, 2); // cpy13: a situação nas listas de observação
        if cpy13_d(p + 0x39) != 0 {
            let escalacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
            let elenco: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
            crate::comparacao::ramo("cpy_13 sai do clube atual");
            escalacao(cpy13_d(0xde1d08), cpy13_d(p + 0x39), p, 1, 1); // cpy13: as escalações
            escalacao(cpy13_d(0xde1d08), cpy13_d(p + 0x39), p, 0, 1);
            elenco(cpy13_d(p + 0x39), p, 0, 1, 0);
        }
        cpy13_hoje_em(p + 0x3e);
    }
    let c = cpy13_contrato(p);
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpy13_clube(cpy13_d(c + 4)) != 0 {
        let c = cpy13_contrato(p);
        *(0xae1080 as *mut usize) = c;
        let dono = if c == 0 { 0 } else { cpy13_clube(cpy13_d(c + 4)) };
        let tirar_ofertas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
        tirar_ofertas(cpy13_d(0xae1050), p, dono);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d0b50usize);
    tirar_contrato(cpy13_d(0xae1050), p);
    core::ptr::write_unaligned((p + 0x52) as *mut u32, 0);
    *((p + 0x6d) as *mut u8) = 0;
    let percentual: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb4f0usize);
    let bonus_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let bonus_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8c7e20usize);
    let fim: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4196a0usize);
    percentual(this, p);
    bonus_time(this, p);
    bonus_selecao(this, p);
    permissao(this, p, 0);
    fim(cpy13_d(0xad9c6c), p);
}

// ponytail: paga, tira de listas, escalações, elenco e contrato (não pode rodar duas vezes): só a original, conferida
// pelo plano: emprestada, a conta das transferências passadas fica; senão sobe 1 com o registro novo (a pessoa, o
// clube, −1 e o tipo 5 ou 3) e, fora do só registro, sem contrato principal, com +0x52 e +0x6d zerados.
unsafe extern "thiscall" fn comparar_free_transfer_staff(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let emprestada = ficha(cpy13_d(0xae1050), p) != 0;
    let n = cpy13_d(this + 0x86a);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32)>(%%I0%%)(this, p, clube, motivo, compensacao, so_registro);
    if emprestada {
        PLACAR.registrar(cpy13_d(this + 0x86a) == n, || format!("free_transfer_staff(pessoa {}) emprestada: conta {} -> {}", cpy13_d(p), n, cpy13_d(this + 0x86a)));
        return;
    }
    if cpy13_d(this + 0x862) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let r = cpy13_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let mut ok = cpy13_d(this + 0x86a) == n.wrapping_add(1)
        && cpy13_d(r) == cpy13_d(p)
        && cpy13_d(r + 0x10) as u32 == u32::MAX
        && cpy13_d(r + 0x14) as u32 == if clube != 0 { cpy13_d(clube) as u32 } else { u32::MAX }
        && cpy13_b(r + 0x1c) == if so_registro != 0 { 5 } else { 3 };
    if so_registro == 0 {
        ok = ok && cpy13_contrato(p) == 0 && cpy13_d(p + 0x52) == 0 && cpy13_b(p + 0x6d) == 0;
    }
    PLACAR.registrar(ok, || format!("free_transfer_staff(pessoa {}, clube {:x}, só registro {}): conta {} -> {}, registro {:x?}", cpy13_d(p), clube, so_registro,
        n, cpy13_d(this + 0x86a), core::slice::from_raw_parts(r as *const u8, 0x1d)));
}

/// { 0x5fd2f0 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::set_on_holiday(férias, técnico)`: sem técnico, o
/// humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); nenhum: a caixa da linha 0x41c. O índice do técnico (id − [0xae2364]
/// + 16) fora de 0..[this+4]: a caixa da linha 0x428. A marca de férias (+0x9a01 do registro de 0xa6ef bytes em [this])
/// passa a `férias`, com a conta dos humanos de férias (+0xc) subindo ao ligar e descendo ao desligar; ao ligar, as
/// notícias pendentes do técnico com o clube (+0x39) e com a seleção (+0x24) são respondidas (0x7874b0); com o dia em
/// [0xb67a50], o interesse do dia (0x6b5040).
unsafe extern "thiscall" fn set_on_holiday(this: usize, ferias: u32, tecnico: usize) {
    let mut t = tecnico;
    if t == 0 {
        t = cpy13_d(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize));
        if t == 0 {
            aviso_em(0x9cf11c, 0x41c);
            return;
        }
    }
    let i = (cpy13_d(t) as i32).wrapping_sub(cpy13_d(0xae2364) as i32).wrapping_add(0x10);
    if i < 0 || i > (cpy13_d(this + 4) as i32).wrapping_sub(1) {
        aviso_em(0x9cf11c, 0x428);
        return;
    }
    let marca = cpy13_d(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let conta = (this + 0xc) as *mut i32;
    let antes = cpy13_d(marca);
    if ferias != 0 {
        if antes == 0 {
            *conta = (*conta).wrapping_add(1);
        }
    } else if antes != 0 {
        *conta = (*conta).wrapping_sub(1);
    }
    core::ptr::write_unaligned(marca as *mut u32, ferias);
    crate::comparacao::ramo("cpy_13 ferias do humano");
    if ferias != 0 {
        let respond: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7874b0usize);
        if cpy13_d(t + 0x39) != 0 {
            respond(cpy13_d(0xb7431c), t, cpy13_d(t + 0x39));
        }
        if cpy13_d(t + 0x24) != 0 {
            respond(cpy13_d(0xb7431c), t, cpy13_d(t + 0x24));
        }
    }
    let dia = cpy13_d(0xb67a50);
    if dia != 0 {
        let interesse: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6b5040usize);
        interesse(dia);
    }
}

// ponytail: responde notícias e mexe no interesse do dia (não pode rodar duas vezes): só a original, conferida pelo
// plano: a marca de férias passa a `férias` e a conta muda de ±1 só quando a marca muda.
unsafe extern "thiscall" fn comparar_set_on_holiday(this: usize, ferias: u32, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I1%%);
    let t = if tecnico != 0 { tecnico } else { cpy13_d(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize)) };
    let i = if t == 0 { -1 } else { (cpy13_d(t) as i32).wrapping_sub(cpy13_d(0xae2364) as i32).wrapping_add(0x10) };
    if i < 0 || i > (cpy13_d(this + 4) as i32).wrapping_sub(1) {
        return o(this, ferias, tecnico); // a caixa: só a original
    }
    let marca = cpy13_d(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let (antes, conta) = (cpy13_d(marca), cpy13_d(this + 0xc) as i32);
    let esperada = conta + if ferias != 0 && antes == 0 { 1 } else if ferias == 0 && antes != 0 { -1 } else { 0 };
    o(this, ferias, tecnico);
    let (depois, conta2) = (cpy13_d(marca) as u32, cpy13_d(this + 0xc) as i32);
    PLACAR.registrar(depois == ferias && conta2 == esperada, || format!("set_on_holiday({}, técnico {:x}): marca {:x} -> {:x}, conta {} -> {} (esperava {})", ferias, t, antes, depois, conta, conta2, esperada));
}

/// { 0x8d7cd0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH das notícias locais): `TRANSFER_MANAGER::
/// post_contract_talks_news(oferta)`, pela oferta aceita (0x8c0080). O vendedor: o clube do contrato da pessoa (0x4cf5a0,
/// guardado em [0xae1080]) fora do tipo 5. Comprador humano (0x7875e0): a notícia 0xfac com a pessoa (o id e as três
/// chaves), o vendedor (ou −1), o comprador, +0x20, +0x24, 0 e 0. Vendedor humano com troca (+4): uma 0xfac por jogador
/// da troca, com ele, o comprador, o vendedor, +0x20, +0x24, 0 e 1, ao vendedor.
unsafe extern "thiscall" fn post_contract_talks_news(_this: usize, o: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let p = cpy13_pessoa(cpy13_d(o + 0xc));
    let c = cpy13_contrato(p);
    *(0xae1080 as *mut usize) = c;
    let vendedor = if c == 0 || cpy13_b(c + 0x35) & 0x3f == 5 { 0 } else { cpy13_clube(cpy13_d(c + 4)) };
    let comprador = cpy13_clube(cpy13_d(o + 8));
    let chaves = |nt: usize, q: usize| {
        gravar(nt, 0, cpy13_d(q) as u32);
        gravar(nt, 1, cpy13_d(cpy13_d(q + 4) + 0x33) as u32);
        gravar(nt, 2, cpy13_d(cpy13_d(q + 8) + 0x33) as u32);
        gravar(nt, 3, cpy13_d(cpy13_d(q + 0xc) + 0x33) as u32);
    };
    if humano(cpy13_d(0xb7431c), comprador) != 0 {
        crate::comparacao::ramo("cpy_13 noticia ao comprador");
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        construir(nt, 0xfac, 0);
        chaves(nt, p);
        gravar(nt, 4, if vendedor != 0 { cpy13_d(vendedor) as u32 } else { u32::MAX });
        gravar(nt, 5, cpy13_d(o + 8) as u32);
        gravar(nt, 7, cpy13_d(o + 0x20) as u32);
        gravar(nt, 8, cpy13_d(o + 0x24) as u32);
        gravar(nt, 9, 0);
        gravar(nt, 0xa, 0);
        entregar(cpy13_d(0xb7431c), nt, comprador);
    }
    if vendedor == 0 || humano(cpy13_d(0xb7431c), vendedor) == 0 || cpy13_d(o + 4) == 0 {
        return;
    }
    let troca = cpy13_d(o + 4);
    core::ptr::write_unaligned((troca + 0xa) as *mut usize, cpy13_d(troca + 2));
    let mut e = proximo(cpy13_d(o + 4));
    let mut local = [0u32; 0x38]; // a segunda notícia local
    let nt = local.as_mut_ptr() as usize;
    while e != 0 {
        let q = cpy13_pessoa(cpy13_d(e));
        crate::comparacao::ramo("cpy_13 noticia ao vendedor");
        construir(nt, 0xfac, 0);
        chaves(nt, q);
        gravar(nt, 4, cpy13_d(o + 8) as u32);
        gravar(nt, 5, cpy13_d(vendedor) as u32);
        gravar(nt, 7, cpy13_d(o + 0x20) as u32);
        gravar(nt, 8, cpy13_d(o + 0x24) as u32);
        gravar(nt, 9, 0);
        gravar(nt, 0xa, 1);
        entregar(cpy13_d(0xb7431c), nt, vendedor);
        e = proximo(cpy13_d(o + 4));
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_post_contract_talks_news(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, o)
}
