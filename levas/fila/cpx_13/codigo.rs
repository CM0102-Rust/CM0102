// ---------- %%LEVA%%a leva: as convocações para o jogo de seleções e duas notícias: escolher os elencos dos dois lados, a notícia do convocado e a do desfecho na copa (national_teams.cpp, Cup.cpp) ----------

unsafe fn cpx13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O ensaio do comparador da 0x76bab0: a cópia roda sem escolher elenco (0x76c120), sem dispensar os acima da idade do
/// sub-21 (0x76d880: no ensaio, só o byte +0x6b que ela zera depois da data) e sem apagar repetidos (0x76c060).
static mut SELECT_SQUADS_DRY_RUN: bool = false;
/// No ensaio: a cópia pediu um elenco (a comparação não vale).
static mut SELECT_SQUADS_REQUESTED_SQUAD: bool = false;

/// O registro da nação do clube na tabela das seleções ([this+4], 0x6e bytes, pelo id da nação +0x53).
unsafe fn cpx13_registro(this: usize, clube: usize) -> usize {
    cpx13_d(this + 4).wrapping_add(cpx13_d(cpx13_d(clube + 0x53)).wrapping_mul(0x6e))
}

/// Um elenco do registro `reg` (tipo 0 principal, 1 B/olímpico, 2 sub-21: a marca de hoje em +0x32/+0x46/+0x5a, a
/// data do elenco em +0x3a/+0x4e/+0x62, a marca de convocado em +0x43/+0x57/+0x6b): com o elenco de até 14 dias (0x54cdb0
/// sobre hoje) e a marca (e `extra`), grava hoje (0x417c10) e leva a data do elenco até a data do jogo (+0x2a/+0x28)
/// se ela for anterior; senão pede o elenco (0x76c120 com a nação, o tipo, hoje, a partida e 0).
unsafe fn refresh_or_select_squad(this: usize, p: usize, clube: usize, reg: usize, tipo: u32, extra: impl FnOnce() -> bool) {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let base = reg + [0x32usize, 0x46, 0x5a][tipo as usize];
    let data = base + 8;
    let d = dias(0xae2c90, cpx13_d(data) as u32, cpx13_d(data + 4) as u32);
    if d <= 0xe && cpx13_b(base + 0x11) != 0 && extra() {
        let mut r = [0u8; 8];
        copia(base, r.as_mut_ptr(), 0xae2c90);
        let mut jogo = [0u8; 8];
        data_nova(jogo.as_mut_ptr(), cpx13_w(p + 0x2a) as u16 as u32, cpx13_w(p + 0x28) as u16 as u32);
        let (ra, rd) = (cpx13_w(data + 2) as u16, cpx13_w(data) as i16);
        let (ja, jd) = (u16::from_le_bytes([jogo[2], jogo[3]]), i16::from_le_bytes([jogo[0], jogo[1]]));
        if ra < ja || (ra == ja && rd < jd) {
            core::ptr::copy_nonoverlapping(data as *const u8, r.as_mut_ptr(), 8);
            copia(data, r.as_mut_ptr(), jogo.as_ptr() as usize);
        }
        return;
    }
    if SELECT_SQUADS_DRY_RUN {
        SELECT_SQUADS_REQUESTED_SQUAD = true;
        return;
    }
    let escolher: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32) = core::mem::transmute(0x76c120usize);
    crate::comparacao::ramo("cpx_13 elenco pedido");
    escolher(this, cpx13_d(clube + 0x53), tipo, 0xae2c90, p, 0); // cpx13: o elenco
}

/// Os elencos B/olímpico (+0x4e da partida, bit 0x100) ou sub-21 (bit 0x200, ou o clube é seleção B 0x5409b0) do lado
/// `clube`; o sub-21 só sem convocado: competição [0x9cf7a0] ou a nação fora do vetor [this+0x36].
unsafe fn refresh_minor_squads(this: usize, p: usize, clube: usize, reg: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let m = cpx13_w(p + 0x4d) as u16;
    if m & 0x100 != 0 {
        refresh_or_select_squad(this, p, clube, reg, 1, || true);
    } else if m & 0x200 != 0 || segunda(clube) != 0 {
        refresh_or_select_squad(this, p, clube, reg, 2, || {
            cpx13_d(p) == cpx13_d(0x9cf7a0) || cpx13_b(cpx13_d(cpx13_d(clube + 0x53)).wrapping_add(cpx13_d(this + 0x36))) == 0
        });
    }
}

/// O que a 0x76d880 grava no registro (fora das pessoas): depois da data do sub-21 (+0x62/+0x66, 0x456740 sobre
/// hoje), zera a marca +0x6b.
unsafe fn release_overage_u21(this: usize, clube: usize) {
    if !SELECT_SQUADS_DRY_RUN {
        let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76d880usize);
        return dispensar(this, cpx13_d(clube + 0x53));
    }
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let reg = cpx13_registro(this, clube);
    if depois(0xae2c90, cpx13_d(reg + 0x62) as u32, cpx13_d(reg + 0x66) as u32) != 0 {
        *((reg + 0x6b) as *mut u8) = 0;
    }
}

/// { 0x76bab0 } (thiscall, ret 4, national_teams.cpp, quadro SEH sem destrutor): `NationalTeams::select_squads_for_fixture
/// (partida)`: sem a marca +0x40 com o bit 1 de +0x4e, ou mandante seleção B (0x5409b0) com o bit 2, nada. Só a
/// partir de duas semanas antes do jogo (0x54cab0 sobre a data +0x2a/+0x28). Mandante: fora de seleção B, o elenco
/// principal (refresh_or_select_squad tipo 0) e, sem o bit 2, a dispensa do sub-21 (0x76d880) das duas nações; os elencos
/// menores (refresh_minor_squads). Visitante: o mesmo sem a dispensa. Por fim apaga os repetidos (0x76c060) das duas.
unsafe extern "thiscall" fn select_squads_for_fixture(this: usize, p: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let semanas_antes: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x76c060usize);
    if cpx13_b(p + 0x40) == 0 && cpx13_b(p + 0x4e) & 1 != 0 {
        return;
    }
    if segunda(cpx13_d(p + 0x1c)) != 0 && cpx13_b(p + 0x4e) & 2 != 0 {
        return;
    }
    let mut jogo = [0u8; 8];
    data_nova(jogo.as_mut_ptr(), cpx13_w(p + 0x2a) as u16 as u32, cpx13_w(p + 0x28) as u16 as u32);
    let (hd, ha) = (cpx13_w(0xae2c90), cpx13_w(0xae2c92) as u16);
    let mut r = [0u8; 8];
    let q = semanas_antes(jogo.as_mut_ptr(), r.as_mut_ptr(), 2); // cpx13: a janela
    let (qa, qd) = (cpx13_w(q as usize + 2) as u16, cpx13_w(q as usize));
    if ha < qa || (ha == qa && hd < qd) {
        return;
    }
    let (casa, fora) = (cpx13_d(p + 0x1c), cpx13_d(p + 0x20));
    let reg = cpx13_registro(this, casa);
    if segunda(casa) == 0 {
        refresh_or_select_squad(this, p, casa, reg, 0, || true);
        if cpx13_b(p + 0x4e) & 2 == 0 {
            release_overage_u21(this, cpx13_d(p + 0x1c));
            release_overage_u21(this, cpx13_d(p + 0x20));
        }
    }
    refresh_minor_squads(this, p, casa, reg);
    let reg = cpx13_registro(this, fora);
    if segunda(fora) == 0 {
        refresh_or_select_squad(this, p, fora, reg, 0, || true);
    }
    refresh_minor_squads(this, p, fora, reg);
    if !SELECT_SQUADS_DRY_RUN {
        repetidos(cpx13_d(cpx13_d(p + 0x1c) + 0x53));
        repetidos(cpx13_d(cpx13_d(p + 0x20) + 0x53));
    }
}

/// Comparador: a original roda antes; a cópia depois, em ensaio, a partir dos mesmos registros das duas nações (+0x32
/// a +0x6c: marcas de hoje, datas dos elencos e marcas de convocado). Se a cópia pediu algum elenco (0x76c120 aloca e
/// mexe nos elencos), a comparação não vale e fica a original; senão os registros têm de bater.
unsafe extern "thiscall" fn comparar_select_squads_for_fixture(this: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let regioes = [(cpx13_registro(this, cpx13_d(p + 0x1c)) + 0x32, 0x3a), (cpx13_registro(this, cpx13_d(p + 0x20)) + 0x32, 0x3a)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    o(this, p);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    SELECT_SQUADS_DRY_RUN = true;
    SELECT_SQUADS_REQUESTED_SQUAD = false;
    crate::aleatorio::pausar_rastro(|| select_squads_for_fixture(this, p));
    SELECT_SQUADS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    if !SELECT_SQUADS_REQUESTED_SQUAD {
        PLACAR.registrar(mo == mr, || format!("select_squads_for_fixture({:x}, {:x}) original={:x?} rust={:x?}", this, p, mo, mr));
    }
}

/// O clube do contrato da pessoa (0x4cf5a0 de [0xae1050], gravado em [0xae1080]; clube +4 em [0xae23b4], 0x245
/// bytes), ou 0.
unsafe fn club_of_person_contract(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpx13_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    cpx13_d(0xae23b4).wrapping_add(cpx13_d(c + 4).wrapping_mul(0x245))
}

/// O ensaio do comparador da 0x76fd10: a cópia roda sem entregar a notícia.
static mut CALLED_UP_NEWS_DRY_RUN: bool = false;

/// { 0x76fd10 } (stdcall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor):
/// `national_teams_post_player_called_up_news(jogador, nação, vaga)`: só com técnico humano (0x7875e0) no clube +0x39
/// do jogador ou no clube do contrato (que precisa existir). A notícia 0x21 (local): 0 o id, 1..3 os +0x33 de +4, +8,
/// +0xc; vaga 0: 4 a seleção principal (0x53b3d0), 5 = 0; vaga 1: a principal e 1; outra: a B (0x53b4f0) e 0.
/// Entregue ao técnico (0x7861e0) do +0x39, se houver, e ao do clube do contrato quando é outro.
unsafe extern "stdcall" fn national_teams_post_player_called_up_news(jogador: usize, nacao: usize, vaga: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let c39 = cpx13_d(jogador + 0x39);
    if c39 == 0 || humano(c39) == 0 {
        if club_of_person_contract(jogador) == 0 {
            return;
        }
        if humano(club_of_person_contract(jogador)) == 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpx_13 noticia do convocado");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x21, 0);
    gravar(n, 0, cpx13_d(jogador) as u32);
    gravar(n, 1, cpx13_d(cpx13_d(jogador + 4) + 0x33) as u32);
    gravar(n, 2, cpx13_d(cpx13_d(jogador + 8) + 0x33) as u32);
    gravar(n, 3, cpx13_d(cpx13_d(jogador + 0xc) + 0x33) as u32);
    match vaga as u8 {
        0 => {
            gravar(n, 4, cpx13_d(principal(nacao)) as u32);
            gravar(n, 5, 0);
        }
        1 => {
            gravar(n, 4, cpx13_d(principal(nacao)) as u32);
            gravar(n, 5, 1);
        }
        _ => {
            gravar(n, 4, cpx13_d(b(nacao)) as u32);
            gravar(n, 5, 0);
        }
    }
    let noticias = cpx13_d(0xb7431c);
    let c39 = cpx13_d(jogador + 0x39);
    if c39 != 0 && !CALLED_UP_NEWS_DRY_RUN {
        ao_clube(noticias, n, c39);
    }
    if club_of_person_contract(jogador) == cpx13_d(jogador + 0x39) {
        return;
    }
    let c = club_of_person_contract(jogador);
    if !CALLED_UP_NEWS_DRY_RUN {
        ao_clube(noticias, n, c);
    }
}

/// Comparador: a cópia em ensaio (sem entregar) e a original a partir do mesmo [0xae1080] (o último contrato lido, que
/// as duas gravam).
unsafe extern "stdcall" fn comparar_national_teams_post_player_called_up_news(jogador: usize, nacao: usize, vaga: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(%%I1%%);
    let regioes = [(0xae1080usize, 4usize)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CALLED_UP_NEWS_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| national_teams_post_player_called_up_news(jogador, nacao, vaga));
    CALLED_UP_NEWS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(jogador, nacao, vaga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("national_teams_post_player_called_up_news({:x}, {:x}, {}) original={:x?} rust={:x?}", jogador, nacao, vaga, mo, mr));
}

/// { 0x519690 } (thiscall, ret 0x14, Cup.cpp, quadro SEH sem destrutor): `Cup::add_team_result_news(clube, desfecho,
/// a3, partida, a5)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o clube, o
/// desfecho, a3, a saída (0xff), a partida e a5; −1 dela ou sem partida: 0. A notícia 0x10 (local): 0 o clube, 1 a
/// competição, 2 o desfecho, 3 a categoria do resultado (0x4b0890), 4 o que a virtual devolveu, o jogo a partir do
/// dado 5 (0x4afeb0, que avança o cursor) e no cursor a rodada seguinte (+7 da rodada +0x34 + 1, até +0x3c − 1; 0x8c
/// ou fora: −1). Saída ≠ 0xff: a entrega pelo alcance (0x786090). Desfecho 1 na rodada 0x96 ou 3 na 0x8c (+0x30): à
/// região da competição (0x786860); senão ao técnico do clube (0x7861e0). Devolve 1.
unsafe extern "thiscall" fn add_team_result_news(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let categoria: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b0890usize);
    let save_game_data: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x4afeb0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_regiao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x786860usize);
    let pelo_alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    // ponytail: a original grava só o byte da saída num dword da pilha e empurra o dword; a 0x786090 lê o byte
    let mut saida: u32 = 0xff;
    if amistoso(cpx13_d(0xae2c74), cpx13_d(copa + 4)) != 0 {
        return 0;
    }
    let comp = cpx13_d(cpx13_d(0xadadfc).wrapping_add(cpx13_d(cpx13_d(copa + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, usize, u32) -> i32 = core::mem::transmute(cpx13_d(cpx13_d(comp) + 0x44));
    let r = virtual44(comp, clube, desfecho, a3, &mut saida, p, a5);
    if r == -1 || p == 0 {
        return 0;
    }
    crate::comparacao::ramo("cpx_13 noticia do desfecho");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x10, 0);
    gravar(n, 0, cpx13_d(clube) as u32);
    gravar(n, 1, cpx13_d(cpx13_d(copa + 4)) as u32);
    gravar(n, 2, desfecho as u8 as i8 as i32 as u32);
    gravar(n, 3, categoria(clube, p) as i8 as i32 as u32);
    gravar(n, 4, r as u32);
    // ponytail: o cursor é um dword da pilha com 5 no byte baixo (o resto é sobra); só o byte baixo conta
    let mut cursor = [5u8, 0, 0, 0];
    save_game_data(p, n, cursor.as_mut_ptr());
    let k = cpx13_w(p + 0x34) as i32;
    let proxima = if k < cpx13_w(copa + 0x3c) as i32 - 1 {
        let v = cpx13_w(cpx13_d(copa + 0xa3).wrapping_add((k as usize).wrapping_mul(0x68)) + 0x6f);
        if v != 0x8c { v as i32 } else { -1 }
    } else {
        -1
    };
    gravar(n, cursor[0] as i8, proxima as u32);
    let noticias = cpx13_d(0xb7431c);
    if saida as u8 != 0xff {
        let nacao = cpx13_d(clube + 0x53);
        let continente = if nacao != 0 { cpx13_d(nacao + 0x71) } else { 0 };
        pelo_alcance(noticias, n, clube, cpx13_d(copa + 4), nacao, continente, saida);
        return 1;
    }
    let rodada = cpx13_w(p + 0x30);
    if (desfecho as u8 == 1 && rodada == 0x96) || (desfecho as u8 == 3 && rodada == 0x8c) {
        a_regiao(noticias, n, cpx13_d(copa + 4));
    } else {
        ao_clube(noticias, n, clube);
    }
    1
}

// ponytail: a virtual +0x44 da copa acrescenta registros às pessoas e a notícia vai às caixas dos técnicos (alocam): só
// a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_add_team_result_news(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(%%I2%%)(copa, clube, desfecho, a3, p, a5)
}
