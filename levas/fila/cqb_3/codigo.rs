// ---------- %%LEVA%%a leva: as datas internacionais dos amistosos: a tabela das datas, as datas no ciclo da Copa e o torneio do clube humano (friendly.cpp) ----------

unsafe fn cqb3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqb3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqb3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// As 110 datas internacionais da 0x5ca4f0, na ordem em que a original as grava: (+0 o dia da semana, +1 o dia, +2 o
/// mês a partir de 0, +0xb o ano a somar ao da Copa, o global do continente em +0xc, o global da competição em +0x10
/// ou 0 para −1). Extraídas da desmontagem (0x5ca588..0x5cb8e6); as 5 últimas vão pela 0x5cb9b0.
const CQB3_DATAS: [(u8, u8, u8, u8, usize, usize); 110] = [
    (5, 10, 9, 0, 0x9cfa08, 0),
    (2, 18, 10, 0, 0x9cfa08, 0),
    (6, 15, 11, 0, 0x9cfa08, 0),
    (6, 12, 0, 1, 0x9cfa08, 0),
    (6, 24, 1, 1, 0x9cfa08, 0),
    (2, 24, 2, 1, 0x9cfa08, 0),
    (2, 28, 3, 1, 0x9cfa08, 0),
    (2, 9, 5, 1, 0x9cfa08, 0),
    (6, 24, 6, 1, 0x9cfa08, 0),
    (2, 18, 7, 1, 0x9cfa08, 0),
    (5, 9, 9, 1, 0x9cfa08, 0),
    (5, 1, 5, 2, 0x9cfa08, 0x9cf77c),
    (5, 15, 5, 2, 0x9cfa08, 0x9cf77c),
    (6, 1, 9, 2, 0x9cfa08, 0x9cf77c),
    (6, 10, 10, 2, 0x9cfa08, 0x9cf77c),
    (6, 12, 0, 3, 0x9cfa08, 0x9cf77c),
    (6, 24, 1, 3, 0x9cfa08, 0x9cf77c),
    (5, 5, 3, 3, 0x9cfa08, 0x9cf77c),
    (6, 27, 3, 3, 0x9cfa08, 0x9cf77c),
    (5, 7, 5, 3, 0x9cfa08, 0x9cf77c),
    (6, 24, 6, 3, 0x9cfa08, 0x9cf77c),
    (6, 17, 7, 3, 0x9cfa08, 0x9cf77c),
    (6, 28, 8, 3, 0x9cfa08, 0x9cf77c),
    (5, 20, 9, 3, 0x9cfa08, 0x9cf77c),
    (2, 16, 7, 0, 0x9cfa1c, 0),
    (2, 6, 8, 0, 0x9cfa1c, 0),
    (2, 7, 9, 0, 0x9cfa1c, 0),
    (2, 15, 10, 0, 0x9cfa1c, 0),
    (2, 29, 2, 1, 0x9cfa1c, 0),
    (2, 26, 3, 1, 0x9cfa1c, 0),
    (2, 16, 7, 1, 0x9cfa1c, 0),
    (2, 6, 8, 1, 0x9cfa1c, 0),
    (2, 7, 9, 1, 0x9cfa1c, 0),
    (2, 26, 3, 2, 0x9cfa1c, 0x9cf778),
    (6, 26, 6, 2, 0x9cfa1c, 0x9cf778),
    (2, 6, 8, 2, 0x9cfa1c, 0x9cf778),
    (5, 7, 9, 2, 0x9cfa1c, 0x9cf778),
    (2, 15, 10, 2, 0x9cfa1c, 0x9cf778),
    (2, 26, 3, 3, 0x9cfa1c, 0x9cf778),
    (6, 4, 5, 3, 0x9cfa1c, 0x9cf778),
    (6, 26, 6, 3, 0x9cfa1c, 0x9cf778),
    (2, 16, 7, 3, 0x9cfa1c, 0x9cf778),
    (2, 6, 8, 3, 0x9cfa1c, 0x9cf778),
    (2, 7, 9, 3, 0x9cfa1c, 0x9cf778),
    (2, 15, 10, 3, 0x9cfa1c, 0x9cf778),
    (5, 10, 9, 0, 0x9cfa18, 0),
    (2, 18, 10, 0, 0x9cfa18, 0),
    (6, 15, 11, 0, 0x9cfa18, 0),
    (6, 10, 0, 1, 0x9cfa18, 0),
    (2, 24, 2, 1, 0x9cfa18, 0),
    (6, 25, 6, 1, 0x9cfa18, 0),
    (1, 7, 8, 1, 0x9cfa18, 0),
    (5, 9, 9, 1, 0x9cfa18, 0),
    (2, 17, 10, 1, 0x9cfa18, 0),
    (6, 10, 11, 2, 0x9cfa18, 0x9cf76c),
    (6, 1, 1, 3, 0x9cfa18, 0x9cf76c),
    (5, 10, 3, 3, 0x9cfa18, 0x9cf76c),
    (5, 15, 3, 3, 0x9cfa18, 0x9cf76c),
    (5, 10, 6, 3, 0x9cfa18, 0x9cf76c),
    (5, 17, 8, 3, 0x9cfa18, 0x9cf76c),
    (5, 29, 9, 3, 0x9cfa18, 0x9cf76c),
    (5, 29, 10, 3, 0x9cfa18, 0x9cf76c),
    (5, 24, 2, 0, 0x9cfa10, 0),
    (2, 25, 3, 0, 0x9cfa10, 0),
    (5, 2, 8, 0, 0x9cfa10, 0x9cf784),
    (2, 15, 10, 0, 0x9cfa10, 0),
    (5, 7, 9, 0, 0x9cfa10, 0x9cf784),
    (2, 15, 10, 1, 0x9cfa10, 0),
    (5, 24, 2, 1, 0x9cfa10, 0x9cf784),
    (2, 25, 3, 1, 0x9cfa10, 0x9cf784),
    (2, 6, 5, 1, 0x9cfa10, 0x9cf784),
    (5, 2, 8, 1, 0x9cfa10, 0x9cf784),
    (2, 6, 8, 1, 0x9cfa10, 0x9cf784),
    (5, 24, 2, 2, 0x9cfa10, 0),
    (2, 25, 3, 2, 0x9cfa10, 0),
    (5, 7, 9, 2, 0x9cfa10, 0x9cf784),
    (2, 15, 10, 2, 0x9cfa10, 0),
    (2, 15, 10, 3, 0x9cfa10, 0),
    (5, 7, 9, 2, 0x9cfa10, 0x9cf780),
    (5, 24, 2, 3, 0x9cfa10, 0x9cf780),
    (2, 25, 3, 3, 0x9cfa10, 0x9cf780),
    (2, 6, 5, 3, 0x9cfa10, 0x9cf780),
    (5, 2, 8, 3, 0x9cfa10, 0x9cf780),
    (2, 6, 8, 3, 0x9cfa10, 0x9cf780),
    (5, 7, 9, 3, 0x9cfa10, 0x9cf780),
    (5, 25, 7, 0, 0x9cfa14, 0),
    (2, 18, 10, 0, 0x9cfa14, 0),
    (6, 15, 11, 0, 0x9cfa14, 0),
    (6, 10, 0, 1, 0x9cfa14, 0),
    (6, 23, 1, 1, 0x9cfa14, 0),
    (2, 24, 2, 1, 0x9cfa14, 0),
    (2, 28, 3, 1, 0x9cfa14, 0),
    (1, 7, 8, 1, 0x9cfa14, 0),
    (5, 9, 9, 1, 0x9cfa14, 0),
    (5, 25, 6, 2, 0x9cfa14, 0x9cf770),
    (5, 25, 7, 2, 0x9cfa14, 0x9cf770),
    (6, 17, 10, 2, 0x9cfa14, 0x9cf770),
    (6, 26, 0, 3, 0x9cfa14, 0x9cf770),
    (6, 23, 1, 3, 0x9cfa14, 0x9cf770),
    (6, 2, 2, 3, 0x9cfa14, 0x9cf770),
    (5, 15, 2, 3, 0x9cfa14, 0x9cf770),
    (6, 20, 3, 3, 0x9cfa14, 0x9cf770),
    (6, 29, 5, 3, 0x9cfa14, 0x9cf770),
    (2, 9, 8, 3, 0x9cfa14, 0x9cf770),
    (2, 1, 9, 3, 0x9cfa14, 0x9cf770),
    (6, 16, 10, 3, 0x9cfa14, 0x9cf770),
    (2, 9, 5, 1, 0x9cfa0c, 0),
    (1, 7, 8, 1, 0x9cfa0c, 0),
    (2, 4, 5, 3, 0x9cfa0c, 0x9cf774),
    (5, 13, 8, 3, 0x9cfa0c, 0x9cf774),
];

/// { 0x5ca4f0 } (thiscall, ret 0, friendly.cpp): `Friendly::build_international_date_table()`, pelo construtor da
/// competição de amistosos (0x5c1f80, na criação do jogo). Conta 0xc8 em +0x34f0 e a tabela de 0xfa0 bytes (malloc
/// 0x944e46: push n; add esp, 4 -> cdecl) em +0x34ec (sem memória: a caixa da linha 0x15d5, conta 0: a original);
/// grava as 110 datas de 20 bytes (as 105 primeiras direto, as 5 últimas pela 0x5cb9b0: mov ecx, entrada; 6 pushes;
/// sem add esp -> thiscall, ret 0x18); com a conta abaixo de 110 (nunca: é a 0xc8 de cima) a caixa da linha 0x15d8;
/// conta 110, a tabela reduzida a 0x898 bytes (realloc 0x945667: 2 pushes; add esp, 8 -> cdecl), as datas no ciclo
/// da Copa (0x5cc510: mov ecx, esi; sem push -> thiscall) e o último dia feito (+0x34f4) = 400.
unsafe extern "thiscall" fn build_international_date_table(this: usize) {
    *((this + 0x34f0) as *mut i32) = 0xc8;
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = malloc(0xfa0);
    *((this + 0x34ec) as *mut usize) = t;
    if t == 0 {
        // ponytail: sem memória, a caixa; a original refaz a conta e cai nela (raro demais para copiar a caixa)
        *((this + 0x34f0) as *mut i32) = 0xc8;
        return original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    }
    let preencher: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x5cb9b0usize);
    for (k, &(semana, dia, mes, ano, continente, competicao)) in CQB3_DATAS.iter().enumerate() {
        let e = cqb3_d(this + 0x34ec) + k * 0x14;
        let comp = if competicao == 0 { -1 } else { cqb3_i(competicao) };
        if k >= 105 {
            // ponytail: a original empurra registradores inteiros (o ecx, o edx, o ebx) com o byte certo embaixo; a 0x5cb9b0
            // só lê os bytes: aqui, os bytes limpos
            preencher(e, semana as u32, dia as u32, mes as u32, ano as u32, cqb3_i(continente) as u32, comp as u32);
            continue;
        }
        *(e as *mut u8) = semana;
        *((e + 1) as *mut u8) = dia;
        *((e + 2) as *mut u8) = mes;
        *((e + 0xb) as *mut u8) = ano; // cqb3: o ano a somar ao da Copa
        core::ptr::write_unaligned((e + 0xc) as *mut i32, cqb3_i(continente));
        core::ptr::write_unaligned((e + 0x10) as *mut i32, comp);
    }
    if cqb3_i(this + 0x34f0) < 0x6e {
        return; // ponytail: inalcançável (a conta é a 0xc8 gravada acima); a original dá a caixa da linha 0x15d8
    }
    crate::comparacao::ramo("cqb_3 tabela montada");
    *((this + 0x34f0) as *mut i32) = 0x6e;
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    *((this + 0x34ec) as *mut usize) = realloc(cqb3_d(this + 0x34ec), 0x898);
    let resolver: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5cc510usize);
    resolver(this);
    *((this + 0x34f4) as *mut u16) = 0x190;
}

/// Comparador da 0x5ca4f0: a cópia primeiro (aloca a tabela dela, que sai copiada e liberada: free 0x9452ca, cdecl),
/// depois a original a partir dos mesmos +0x34ec..+0x34f5; confere a conta, o último dia feito e as 110 datas. Roda
/// uma vez, no construtor dos amistosos.
unsafe extern "thiscall" fn comparar_build_international_date_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let campos = crate::comparacao::Memoria::ler(&[(this + 0x34ec, 10)]);
    build_international_date_table(this);
    let (t, n) = (cqb3_d(this + 0x34ec), cqb3_i(this + 0x34f0));
    if t == 0 || n != 0x6e {
        campos.escrever();
        return o(this); // ponytail: sem memória na cópia, só a original
    }
    let mr = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    free(t);
    campos.escrever();
    o(this);
    let t = cqb3_d(this + 0x34ec);
    if t == 0 {
        return PLACAR.registrar(false, || "build_international_date_table: a original ficou sem tabela".to_string());
    }
    let mo = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    PLACAR.registrar(mo.0[0].1 == mr.0[0].1 && mo.0[1].1 == mr.0[1].1, || format!("build_international_date_table original={:x?} rust={:x?}", mo, mr));
}

/// { 0x5cc510 } (thiscall, ret 0, friendly.cpp): `Friendly::resolve_international_dates()`, pela tabela (0x5ca4f0). O
/// ano da Copa: o primeiro a partir do de hoje ([0xae2c92]) com (ano − 2) múltiplo de 4. Cada data (+0x34ec, conta
/// +0x34f0) ganha a data em +3 pela 0x54a130 (lea ecx, entrada + 3; 4 pushes; sem add esp -> thiscall(data, dia, mês,
/// ano, dia da semana), ret 0x10) com o dia (+1) e o dia da semana (+0) em byte com sinal, o mês (+2) e o ano da Copa
/// mais o +0xb (com sinal), menos 4 se pass_ball de 3 anos à frente de hoje (palavras). Depois o qsort pela data (0x945333:
/// 4 pushes; add esp, 0x10 -> cdecl(tabela, conta, 0x14, 0x5cc610)).
unsafe extern "thiscall" fn resolve_international_dates(this: usize) {
    let hoje = cqb3_w(0xae2c92);
    let mut ciclo = hoje as u32;
    while ((ciclo & 0xffff) as i32 - 2) % 4 != 0 {
        crate::comparacao::ramo("cqb_3 ano da copa adiante");
        ciclo = ciclo.wrapping_add(1); // cqb3: o ano da Copa
    }
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x54a130usize);
    let mut k = 0i32;
    while k < cqb3_i(this + 0x34f0) {
        let e = cqb3_d(this + 0x34ec).wrapping_add((k as usize).wrapping_mul(0x14));
        let mut ano = (cqb3_b(e + 0xb) as i8 as i16 as u16).wrapping_add(ciclo as u16);
        if ano as i32 - hoje as i32 > 3 {
            crate::comparacao::ramo("cqb_3 ano para tras");
            ano = ano.wrapping_add(0xfffc); // cqb3: menos 4 se pass_ball de 3 anos à frente
        }
        let com_sinal = |o: usize| cqb3_b(e + o) as i8 as i16 as u16 as u32;
        // ponytail: o dia e o dia da semana vão sobre o edx de metade alta zerada (como a original); o mês e o ano vão
        // sobre o eax, de metade alta lixo, e a 0x54a130 só lê o byte do mês e a palavra do ano: aqui, limpos
        acertar(e + 3, com_sinal(1), cqb3_b(e + 2) as u32, ano as u32, com_sinal(0));
        k += 1;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cqb3_d(this + 0x34ec), cqb3_i(this + 0x34f0), 0x14, 0x5cc610);
}

/// Comparador da 0x5cc510: lado a lado nas datas da tabela (só grava nelas; o qsort e a 0x54a130 não sorteiam).
unsafe extern "thiscall" fn comparar_resolve_international_dates(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (t, n) = (cqb3_d(this + 0x34ec), cqb3_i(this + 0x34f0));
    let reg = if t != 0 && n > 0 { vec![(t, n as usize * 0x14)] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || resolve_international_dates(this));
    PLACAR.registrar(igual, || format!("resolve_international_dates(conta {}) original={:x?} rust={:x?}", n, mo, mr));
}

/// { 0x5c6830 } (thiscall, ret 0, friendly.cpp): `Friendly::find_human_club_tournament()`, pelas telas dos amistosos. O
/// clube (+0x39) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); entre os 127 torneios (0x69 bytes a partir
/// de +0xb1), o primeiro com esse anfitrião (+0) e em uso (+0x29 ≠ 0): o ponteiro dele; senão 0.
unsafe extern "thiscall" fn find_human_club_tournament(this: usize) -> usize {
    let tecnico = cqb3_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if tecnico == 0 {
        return 0;
    }
    let clube = cqb3_d(tecnico + 0x39);
    if clube == 0 {
        return 0;
    }
    for i in 0..0x7fusize {
        let t = this + 0x69 * i + 0xb1;
        let anfitriao = cqb3_d(t);
        if anfitriao != 0 && anfitriao == clube && cqb3_d(t + 0x29) != 0 {
            return t;
        }
    }
    0
}

comparar_valor!(comparar_find_human_club_tournament, find_human_club_tournament, %%I2%%, () -> usize);
