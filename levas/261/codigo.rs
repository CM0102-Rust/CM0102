// ---------- 261a leva: as convocações para o jogo de seleções e duas notícias: escolher os elencos dos dois lados, a notícia do convocado e a do desfecho na copa (national_teams.cpp, Cup.cpp) ----------

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
static mut CPX13_ENSAIO: bool = false;
/// No ensaio: a cópia pediu um elenco (a comparação não vale).
static mut CPX13_PEDIU: bool = false;

/// O registro da nação do clube na tabela das seleções ([this+4], 0x6e bytes, pelo id da nação +0x53).
unsafe fn cpx13_registro(this: usize, clube: usize) -> usize {
    cpx13_d(this + 4).wrapping_add(cpx13_d(cpx13_d(clube + 0x53)).wrapping_mul(0x6e))
}

/// Um elenco do registro `reg` (tipo 0 principal, 1 B/olímpico, 2 sub-21: a marca de hoje em +0x32/+0x46/+0x5a, a
/// data do elenco em +0x3a/+0x4e/+0x62, a marca de convocado em +0x43/+0x57/+0x6b): com o elenco de até 14 dias (0x54cdb0
/// sobre hoje) e a marca (e `extra`), grava hoje (0x417c10) e leva a data do elenco até a data do jogo (+0x2a/+0x28)
/// se ela for anterior; senão pede o elenco (0x76c120 com a nação, o tipo, hoje, a partida e 0).
unsafe fn cpx13_elenco(this: usize, p: usize, clube: usize, reg: usize, tipo: u32, extra: impl FnOnce() -> bool) {
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
    if CPX13_ENSAIO {
        CPX13_PEDIU = true;
        return;
    }
    let escolher: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32) = core::mem::transmute(0x76c120usize);
    crate::comparacao::ramo("cpx_13 elenco pedido");
    escolher(this, cpx13_d(clube + 0x53), tipo, 0xae2c90, p, 0); // cpx13: o elenco
}

/// Os elencos B/olímpico (+0x4e da partida, bit 0x100) ou sub-21 (bit 0x200, ou o clube é seleção B 0x5409b0) do lado
/// `clube`; o sub-21 só sem convocado: competição [0x9cf7a0] ou a nação fora do vetor [this+0x36].
unsafe fn cpx13_elencos_menores(this: usize, p: usize, clube: usize, reg: usize) {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let m = cpx13_w(p + 0x4d) as u16;
    if m & 0x100 != 0 {
        cpx13_elenco(this, p, clube, reg, 1, || true);
    } else if m & 0x200 != 0 || segunda(clube) != 0 {
        cpx13_elenco(this, p, clube, reg, 2, || {
            cpx13_d(p) == cpx13_d(0x9cf7a0) || cpx13_b(cpx13_d(cpx13_d(clube + 0x53)).wrapping_add(cpx13_d(this + 0x36))) == 0
        });
    }
}

/// O que a 0x76d880 grava no registro (fora das pessoas): depois da data do sub-21 (+0x62/+0x66, 0x456740 sobre
/// hoje), zera a marca +0x6b.
unsafe fn cpx13_dispensar(this: usize, clube: usize) {
    if !CPX13_ENSAIO {
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
/// principal (cpx13_elenco tipo 0) e, sem o bit 2, a dispensa do sub-21 (0x76d880) das duas nações; os elencos
/// menores (cpx13_elencos_menores). Visitante: o mesmo sem a dispensa. Por fim apaga os repetidos (0x76c060) das duas.
unsafe extern "thiscall" fn convocar_selecoes_para_o_jogo(this: usize, p: usize) {
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
        cpx13_elenco(this, p, casa, reg, 0, || true);
        if cpx13_b(p + 0x4e) & 2 == 0 {
            cpx13_dispensar(this, cpx13_d(p + 0x1c));
            cpx13_dispensar(this, cpx13_d(p + 0x20));
        }
    }
    cpx13_elencos_menores(this, p, casa, reg);
    let reg = cpx13_registro(this, fora);
    if segunda(fora) == 0 {
        cpx13_elenco(this, p, fora, reg, 0, || true);
    }
    cpx13_elencos_menores(this, p, fora, reg);
    if !CPX13_ENSAIO {
        repetidos(cpx13_d(cpx13_d(p + 0x1c) + 0x53));
        repetidos(cpx13_d(cpx13_d(p + 0x20) + 0x53));
    }
}

/// Comparador: a original roda antes; a cópia depois, em ensaio, a partir dos mesmos registros das duas nações (+0x32
/// a +0x6c: marcas de hoje, datas dos elencos e marcas de convocado). Se a cópia pediu algum elenco (0x76c120 aloca e
/// mexe nos elencos), a comparação não vale e fica a original; senão os registros têm de bater.
unsafe extern "thiscall" fn comparar_convocar_selecoes_para_o_jogo(this: usize, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1341);
    let regioes = [(cpx13_registro(this, cpx13_d(p + 0x1c)) + 0x32, 0x3a), (cpx13_registro(this, cpx13_d(p + 0x20)) + 0x32, 0x3a)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    o(this, p);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    CPX13_ENSAIO = true;
    CPX13_PEDIU = false;
    crate::aleatorio::pausar_rastro(|| convocar_selecoes_para_o_jogo(this, p));
    CPX13_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    if !CPX13_PEDIU {
        PLACAR.registrar(mo == mr, || format!("convocar_selecoes_para_o_jogo({:x}, {:x}) original={:x?} rust={:x?}", this, p, mo, mr));
    }
}

/// O clube do contrato da pessoa (0x4cf5a0 de [0xae1050], gravado em [0xae1080]; clube +4 em [0xae23b4], 0x245
/// bytes), ou 0.
unsafe fn cpx13_clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpx13_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    cpx13_d(0xae23b4).wrapping_add(cpx13_d(c + 4).wrapping_mul(0x245))
}

/// O ensaio do comparador da 0x76fd10: a cópia roda sem entregar a notícia.
static mut CPX13_ENSAIO_NOTICIA: bool = false;

/// { 0x76fd10 } (stdcall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor):
/// `national_teams_post_player_called_up_news(jogador, nação, vaga)`: só com técnico humano (0x7875e0) no clube +0x39
/// do jogador ou no clube do contrato (que precisa existir). A notícia 0x21 (local): 0 o id, 1..3 os +0x33 de +4, +8,
/// +0xc; vaga 0: 4 a seleção principal (0x53b3d0), 5 = 0; vaga 1: a principal e 1; outra: a B (0x53b4f0) e 0.
/// Entregue ao técnico (0x7861e0) do +0x39, se houver, e ao do clube do contrato quando é outro.
unsafe extern "stdcall" fn noticia_do_convocado(jogador: usize, nacao: usize, vaga: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let c39 = cpx13_d(jogador + 0x39);
    if c39 == 0 || humano(c39) == 0 {
        if cpx13_clube_do_contrato(jogador) == 0 {
            return;
        }
        if humano(cpx13_clube_do_contrato(jogador)) == 0 {
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
    if c39 != 0 && !CPX13_ENSAIO_NOTICIA {
        ao_clube(noticias, n, c39);
    }
    if cpx13_clube_do_contrato(jogador) == cpx13_d(jogador + 0x39) {
        return;
    }
    let c = cpx13_clube_do_contrato(jogador);
    if !CPX13_ENSAIO_NOTICIA {
        ao_clube(noticias, n, c);
    }
}

/// Comparador: a cópia em ensaio (sem entregar) e a original a partir do mesmo [0xae1080] (o último contrato lido, que
/// as duas gravam).
unsafe extern "stdcall" fn comparar_noticia_do_convocado(jogador: usize, nacao: usize, vaga: u32) {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, u32)>(1342);
    let regioes = [(0xae1080usize, 4usize)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX13_ENSAIO_NOTICIA = true;
    crate::aleatorio::pausar_rastro(|| noticia_do_convocado(jogador, nacao, vaga));
    CPX13_ENSAIO_NOTICIA = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(jogador, nacao, vaga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("noticia_do_convocado({:x}, {:x}, {}) original={:x?} rust={:x?}", jogador, nacao, vaga, mo, mr));
}

/// { 0x519690 } (thiscall, ret 0x14, Cup.cpp, quadro SEH sem destrutor): `Cup::add_team_result_news(clube, desfecho,
/// a3, partida, a5)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o clube, o
/// desfecho, a3, a saída (0xff), a partida e a5; −1 dela ou sem partida: 0. A notícia 0x10 (local): 0 o clube, 1 a
/// competição, 2 o desfecho, 3 a categoria do resultado (0x4b0890), 4 o que a virtual devolveu, o jogo a partir do
/// dado 5 (0x4afeb0, que avança o cursor) e no cursor a rodada seguinte (+7 da rodada +0x34 + 1, até +0x3c − 1; 0x8c
/// ou fora: −1). Saída ≠ 0xff: a entrega pelo alcance (0x786090). Desfecho 1 na rodada 0x96 ou 3 na 0x8c (+0x30): à
/// região da competição (0x786860); senão ao técnico do clube (0x7861e0). Devolve 1.
unsafe extern "thiscall" fn noticia_do_desfecho_na_copa(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let categoria: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b0890usize);
    let gravar_jogo: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x4afeb0usize);
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
    gravar_jogo(p, n, cursor.as_mut_ptr());
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
unsafe extern "thiscall" fn comparar_noticia_do_desfecho_na_copa(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(1343)(copa, clube, desfecho, a3, p, a5)
}

// ---------- 261a leva: as vagas da lista de observação com lance, e as marcas de lista de transferências e de empréstimos do contrato (shortlist_manager.cpp) ----------

unsafe fn cpy1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy1_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0: thiscall([0xae1050], pessoa), ret 4).
/// As três marcas não guardam o contrato em [0xae1080].
unsafe fn cpy1_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpy1_d(0xae1050), pessoa)
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const CPY1_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// A entrada do clube no Shortlist_Manager (+0xdc, 9 bytes por clube: +0 as 40 vagas de 0x25 bytes, +4 o estado).
unsafe fn cpy1_entrada(this: usize, clube: usize) -> usize {
    cpy1_d(this + 0xdc).wrapping_add(cpy1_d(clube).wrapping_mul(9))
}

/// { 0x844540 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::count_slots_with_bids(clube, posicao)`,
/// pela avaliação das listas (0x84a020, com 0xb), pela avaliação do elenco (0x83cb40, com −1) e pelo laço do dia
/// (0x5cccd0, com −1). Das 40 vagas da lista do clube, conta as que têm alvo (+0xf) e lance (+0x17), as duas acima de
/// −1, e posição (+0) igual a `posicao` (0xff: qualquer). Com a lista no estado 2 (+4 da entrada) e o clube do humano
/// (0x600a50 com 1, 1): se hoje − 20 dias (0x54c8e0) passa da data de chegada do técnico (+0x3e da ficha em +0xcf do
/// clube) e o alvo não tem ofertas (0x8c5cf0), a vaga perde o alvo e o lance (−1) e não conta. A conta é um byte.
unsafe extern "thiscall" fn contar_vagas_com_lance(this: usize, clube: usize, posicao: u32) -> u8 {
    // 0x600a50: cdecl(clube, 1, 1, 0, 0) (add esp, 0x14 depois); 0x54c8e0: thiscall(0xae2c90, saída, 0x14); 0x8c5cf0:
    // thiscall([0xde1f54], pessoa, 0, 0)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let entrada = cpy1_entrada(this, clube);
    let mut n: u8 = 0;
    for i in 0..40usize {
        let v = cpy1_d(entrada).wrapping_add(i * 0x25);
        if (cpy1_d(v + 0xf) as i32) <= -1 {
            continue;
        }
        if (cpy1_d(v + 0x17) as i32) <= -1 {
            crate::comparacao::ramo("cpy_1 alvo sem lance");
            continue; // cpy1: alvo sem lance não conta
        }
        if posicao as u8 != 0xff && cpy1_b(v) != posicao as u8 {
            crate::comparacao::ramo("cpy_1 outra posicao");
            continue; // cpy1: lance de outra posição não conta
        }
        if cpy1_b(entrada + 4) == 2 && humano(clube, 1, 1, 0, 0) != 0 {
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, 0x14);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = cpy1_w(r + 2);
            let chegada = cpy1_d(clube + 0xcf).wrapping_add(0x3e);
            let ano_c = cpy1_w(chegada + 2);
            let velho = ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned(chegada as *const i16));
            if velho {
                let alvo = cpy1_d(0xae23bc).wrapping_add(cpy1_d(v + 0xf).wrapping_mul(0x6e));
                if ofertas(cpy1_d(0xde1f54), alvo, 0, 0) == 0 {
                    crate::comparacao::ramo("cpy_1 lance vencido");
                    *((v + 0xf) as *mut i32) = -1; // cpy1: a vaga perde o alvo
                    *((v + 0x17) as *mut i32) = -1;
                    continue;
                }
            }
        }
        crate::comparacao::ramo("cpy_1 vaga com lance");
        n = n.wrapping_add(1); // cpy1: conta a vaga com lance
    }
    n
}

/// Comparador: lado a lado pelas 40 vagas do clube e pelas datas estáticas, e a devolução.
unsafe extern "thiscall" fn comparar_contar_vagas_com_lance(this: usize, clube: usize, posicao: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(1344);
    let mut reg = CPY1_ESTATICAS.to_vec();
    reg.push((cpy1_d(cpy1_entrada(this, clube)), 40 * 0x25));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, posicao), || contar_vagas_com_lance(this, clube, posicao));
    PLACAR.registrar(igual && r == n, || format!("contar_vagas_com_lance(clube {:x}, posicao {}) original={} rust={} {:x?} x {:x?}", clube, posicao as i8, r, n, mo, mr));
    r
}

/// { 0x844280 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_transfer_list(pessoa)`, pelas
/// telas de transferência (0x8f6770, 0x8f8120). Sem contrato (0x4cf5a0): 0. Com ele, o bit 0 de +0x4e (na lista de
/// transferências) sai e devolve 1 no byte baixo (os de cima são os do ponteiro do contrato, como a original).
unsafe extern "stdcall" fn tirar_da_lista_de_transferencias(pessoa: usize) -> u32 {
    let c = cpy1_contrato(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfe;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_tirar_da_lista_de_transferencias(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1345);
    let c = cpy1_contrato(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || tirar_da_lista_de_transferencias(pessoa));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_lista_de_transferencias(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x8442d0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_loan_list(pessoa)`, pela
/// renovação do contrato (0x4dc3a0) e pelas telas de transferência. Sem contrato: 0; com ele, o bit 1 de +0x4e (na
/// lista de empréstimos) sai e devolve 1 no byte baixo (os de cima, os do ponteiro do contrato).
unsafe extern "stdcall" fn tirar_da_lista_de_emprestimos(pessoa: usize) -> u32 {
    let c = cpy1_contrato(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfd;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_tirar_da_lista_de_emprestimos(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1346);
    let c = cpy1_contrato(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || tirar_da_lista_de_emprestimos(pessoa));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_lista_de_emprestimos(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ---------- 261a leva: a situação da pessoa nas listas de observação dos clubes, a marca de lista de empréstimos e o teto de preço pelo estatuto (shortlist_manager.cpp) ----------

unsafe fn cpy2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy2_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy2_clube(id: usize) -> usize {
    cpy2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A entrada do clube no Shortlist_Manager (+0xdc, 9 bytes por clube: +0 as 40 vagas de 0x25 bytes, +4 o estado, +8).
unsafe fn cpy2_entrada(this: usize, clube: usize) -> usize {
    cpy2_d(this + 0xdc).wrapping_add(cpy2_d(clube).wrapping_mul(9))
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const CPY2_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// Tira o par (pessoa, clube) de um dos vetores ordenados de pares (8 bytes) do Shortlist_Manager (`base` = +0 ou +4,
/// conta em `base` + 0x10) pela busca binária (0x946464: cdecl(chave, vetor, conta, 8, comparação), add esp, 0x14):
/// os de depois descem uma casa, o último fica zerado e a conta cai. Devolve se achou.
unsafe fn cpy2_tirar_par(base: usize, chave: &[u32; 2], comparar: usize) -> bool {
    let bsearch: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let p = bsearch(chave.as_ptr(), cpy2_d(base), cpy2_d(base + 0x10) as u32, 8, comparar);
    if p == 0 {
        return false;
    }
    let mut i = (p.wrapping_sub(cpy2_d(base)) >> 3) as i32;
    while i < cpy2_d(base + 0x10) as i32 - 1 {
        let v = cpy2_d(base);
        i += 1;
        let k = v.wrapping_add(i as usize * 8);
        cpy2_pd(k - 8, cpy2_d(k) as u32);
        cpy2_pd(k - 4, cpy2_d(k + 4) as u32);
    }
    let fim = cpy2_d(base).wrapping_add(cpy2_d(base + 0x10).wrapping_mul(8));
    cpy2_pd(fim - 4, 0);
    cpy2_pd(fim - 8, 0);
    cpy2_pd(base + 0x10, (cpy2_d(base + 0x10) as u32).wrapping_sub(1));
    true
}

/// A data de hoje mais (0x54c5b0) ou menos (0x54c8e0) `dias` (thiscall(0xae2c90, saída, dias), ret 8) copiada para a
/// data da vaga em +0x1b (0x417c10: thiscall(destino, saída, origem), ret 8).
unsafe fn cpy2_data_da_vaga(v: usize, rotina: usize, dias: u32) {
    let deslocar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(rotina);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let r = deslocar(0xae2c90, &mut a, dias);
    copiar(v + 0x1b, &mut b, r);
}

/// O par (pessoa, clube) do evento 2 ou 5: a vaga guarda a pessoa em +0x13 (bloqueada até a data de +0x1b), sem lance
/// (+0x17 = −1) e sem valores (+0xb, +7); o estatuto da vaga (bits 1..4 de +0x24) vai ao nibble alto de +0x4f do
/// contrato da pessoa no clube; a entrada fica com 100 em +8.
unsafe fn cpy2_bloquear(v: usize, entrada: usize, pessoa: usize, contrato: usize, rotina: usize, dias: u32) {
    cpy2_data_da_vaga(v, rotina, dias);
    cpy2_pd(v + 0x13, cpy2_d(pessoa) as u32);
    cpy2_pd(v + 0x17, 0xffff_ffff);
    cpy2_pd(v + 0xb, 0);
    cpy2_pd(v + 7, 0);
    if contrato != 0 {
        let c = (contrato + 0x4f) as *mut u8;
        *c = (*c & 0xf) | (((cpy2_b(v + 0x24) >> 1) & 0xf) << 4);
    }
    *((entrada + 8) as *mut u8) = 0x64;
}

/// { 0x843e10 } (thiscall, ret 0x10, shortlist_manager.cpp, quadro SEH da LLIST local): `Shortlist_Manager::
/// update_staff_status(pessoa, clube, evento, tipo)`, pelas ofertas, contratos, técnicos e regens (0x4d07e0, 0x690730,
/// 0x7a59e0, 0x8bf680, 0x8c44e0...) e por ela mesma. Com o evento 4 e o clube nulo ou o da pessoa (+0x39): cada clube
/// interessado (0x842180 com 0, 3, 0) recebe o evento 1 com o tipo 2 (recursão). Clube nulo: 1. Com o evento 5, ou 1..3
/// de outro clube: o par (pessoa, clube) sai dos dois vetores ordenados (+0/+0x10 pela 0x757ea0, +4/+0x14 pela
/// 0x844740; vetor nulo: 0); achado nos dois e o clube não humano (0x600a50 com 0, 1), o nibble baixo de +0xb do
/// registro de 79 bytes da pessoa ([0xae2d0c]) cai um. Depois, nas 40 vagas do clube (tipo 1: fora as de jogador 0xb;
/// tipo 0: só elas) com a pessoa no alvo (+0xf; no evento 4, também em +0x13): 1 = a vaga se esvazia e é preenchida de
/// novo (0x83f3a0); 2 e 5 = bloqueio de 120 e 14 dias (0x54c5b0); 3 = esvazia, data de hoje − 120 (0x54c8e0); 4 =
/// solta +0x13 e o alvo, data de hoje − 120. Devolve 1.
unsafe extern "thiscall" fn situacao_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    // 0x68d020/0x68d040/0x68d1b0: thiscall(lista); 0x842180: thiscall(this, pessoa, lista, 0, 3, 0) (5 push); a
    // recursão: thiscall(this, pessoa, clube, 1, 2) (4 push); 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14);
    // 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x83f3a0: thiscall(this, vaga, clube, pessoa) (3 push)
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x83f3a0usize);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    let ev = evento as u8;
    let id = cpy2_d(pessoa);
    if (clube == 0 || clube == cpy2_d(pessoa + 0x39)) && ev == 4 {
        interessados(this, pessoa, l, 0, 3, 0);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpy2_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            situacao(this, pessoa, cpy2_clube(cpy2_d(e + 4)), 1, 2);
            e = proximo(l);
        }
    }
    if clube == 0 {
        destruir(l);
        return 1;
    }
    if ev == 5 || (ev != 4 && clube != cpy2_d(pessoa + 0x39)) {
        if cpy2_d(this) == 0 || cpy2_d(this + 4) == 0 {
            destruir(l);
            return 0;
        }
        let chave = [id as u32, cpy2_d(clube) as u32];
        let mut faltou = !cpy2_tirar_par(this, &chave, 0x757ea0);
        faltou |= !cpy2_tirar_par(this + 4, &chave, 0x844740);
        if humano(clube, 0, 1, 0, 0) == 0 && !faltou {
            crate::comparacao::ramo("cpy_2 contagem da lista");
            let r = (cpy2_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f)) + 0xb) as *mut u8;
            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: uma lista a menos na conta da pessoa
        }
    }
    let entrada = cpy2_entrada(this, clube);
    let contrato = contrato_de(cpy2_d(0xae1050), pessoa, clube);
    for i in 0..40usize {
        let v = cpy2_d(entrada).wrapping_add(i * 0x25);
        let t = tipo as u8;
        if t == 1 {
            if cpy2_b(v) == 0xb {
                continue;
            }
        } else if t == 0 && cpy2_b(v) != 0xb {
            continue;
        }
        if cpy2_d(v + 0xf) != id && (ev != 4 || cpy2_d(v + 0x13) != id) {
            continue;
        }
        match ev {
            1 => {
                cpy2_pd(v + 0xf, 0xffff_ffff);
                cpy2_pd(v + 0x17, 0xffff_ffff);
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
                *((v + 0x24) as *mut u8) &= 0xfe;
                crate::comparacao::ramo("cpy_2 vaga refeita");
                preencher(this, v, clube, pessoa); // cpy2: a vaga é preenchida de novo
            }
            2 => cpy2_bloquear(v, entrada, pessoa, contrato, 0x54c5b0, 0x78),
            5 => {
                crate::comparacao::ramo("cpy_2 bloqueio de 14 dias");
                cpy2_bloquear(v, entrada, pessoa, contrato, 0x54c5b0, 0xe); // cpy2: o bloqueio de 14 dias
            }
            3 => {
                cpy2_pd(v + 0xf, 0xffff_ffff);
                cpy2_pd(v + 0x17, 0xffff_ffff);
                cpy2_data_da_vaga(v, 0x54c8e0, 0x78);
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
            }
            4 => {
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
                if cpy2_d(v + 0xf) == id {
                    cpy2_pd(v + 0xf, 0xffff_ffff);
                    cpy2_pd(v + 0x17, 0xffff_ffff);
                    cpy2_data_da_vaga(v, 0x54c8e0, 0x78);
                }
            }
            _ => {}
        }
    }
    destruir(l);
    1
}

/// O caso que não pode rodar duas vezes: a recursão do evento 4 pelos clubes interessados, ou o evento 1 com a pessoa no
/// alvo de alguma vaga do clube (a 0x83f3a0 preenche a vaga: sorteia, acrescenta pares com realocação e dá notícias).
unsafe fn cpy2_so_a_original(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> bool {
    let ev = evento as u8;
    if ev == 4 && (clube == 0 || clube == cpy2_d(pessoa + 0x39)) {
        return true;
    }
    if ev != 1 || clube == 0 {
        return false;
    }
    // de outro clube, o evento 1 passa pelos vetores; um deles nulo devolve 0 antes das vagas
    if clube != cpy2_d(pessoa + 0x39) && (cpy2_d(this) == 0 || cpy2_d(this + 4) == 0) {
        return false;
    }
    let entrada = cpy2_entrada(this, clube);
    (0..40usize).any(|i| {
        let v = cpy2_d(entrada).wrapping_add(i * 0x25);
        let ok = match tipo as u8 {
            1 => cpy2_b(v) != 0xb,
            0 => cpy2_b(v) == 0xb,
            _ => true,
        };
        ok && cpy2_d(v + 0xf) == cpy2_d(pessoa)
    })
}

/// Comparador: lado a lado pelos dois vetores de pares e suas contas, pelo byte da conta de listas da pessoa, pelas 40
/// vagas e o +8 da entrada do clube, pelo byte +0x4f do contrato e pelas datas estáticas, e a devolução. Na recursão do
/// evento 4 e no evento 1 que preenche a vaga, só a original (a cópia leva os ramos).
unsafe extern "thiscall" fn comparar_situacao_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(1347);
    if cpy2_so_a_original(this, pessoa, clube, evento, tipo) {
        return o(this, pessoa, clube, evento, tipo);
    }
    let mut reg = CPY2_ESTATICAS.to_vec();
    for base in [this, this + 4] {
        reg.push((base + 0x10, 4));
        if cpy2_d(base) != 0 {
            reg.push((cpy2_d(base), cpy2_d(base + 0x10).wrapping_mul(8)));
        }
    }
    reg.push((cpy2_d(0xae2d0c).wrapping_add(cpy2_d(pessoa).wrapping_mul(0x4f)) + 0xb, 1));
    if clube != 0 {
        let entrada = cpy2_entrada(this, clube);
        reg.push((cpy2_d(entrada), 40 * 0x25));
        reg.push((entrada + 8, 1));
        let contrato_de: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        let c = contrato_de(cpy2_d(0xae1050), pessoa, clube);
        if c != 0 {
            reg.push((c + 0x4f, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, clube, evento, tipo),
        || situacao_na_lista_de_observacao(this, pessoa, clube, evento, tipo));
    PLACAR.registrar(igual && r == n, || format!("situacao_na_lista_de_observacao(pessoa {:x}, clube {:x}, evento {}, tipo {}) original={} rust={} {:x?} x {:x?}",
        pessoa, clube, evento as u8, tipo as u8, r, n, mo, mr));
    r
}

/// { 0x8442b0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_add_to_loan_list(pessoa)`, pelas telas de
/// transferência (0x8f6770, 0x8f8120). Com contrato (0x4cf5a0, sem guardar em [0xae1080]), o bit 1 de +0x4e (na lista
/// de empréstimos) liga. Devolve o eax que a original deixa: o contrato.
unsafe extern "stdcall" fn por_na_lista_de_emprestimos(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy2_d(0xae1050), pessoa);
    if c != 0 {
        *((c + 0x4e) as *mut u8) |= 2;
    }
    c
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_por_na_lista_de_emprestimos(pessoa: usize) -> usize {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> usize>(1348);
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy2_d(0xae1050), pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || por_na_lista_de_emprestimos(pessoa));
    PLACAR.registrar(igual && r == n, || format!("por_na_lista_de_emprestimos(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x84b1f0 } (thiscall, ret 0xc, shortlist_manager.cpp): `shortlist_manager_max_fee_for_player_status(clube, pessoa,
/// estatuto)`, pela tela de contrato (0x4dd0f0, ecx = o Shortlist_Manager em +0x856 da gerência de transferências).
/// Sem pessoa ou sem ficha de jogador (+0x61): −1. Senão o teto da vaga (0x84b230) com as posições fortes do jogador
/// (0x5b7d20) e o estatuto.
unsafe extern "thiscall" fn teto_de_preco_pelo_estatuto(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    // 0x5b7d20: cdecl(ficha) (add esp, 4); 0x84b230: chamada com mov ecx, esi (o this) e 3 push (clube, posições,
    // estatuto), ret 0xc: thiscall daqui (a cópia stdcall do motor não lê o ecx)
    if pessoa == 0 || cpy2_d(pessoa + 0x61) == 0 {
        return -1;
    }
    let posicoes: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5b7d20usize);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x84b230usize);
    let m = posicoes(cpy2_d(pessoa + 0x61));
    teto(this, clube, m, estatuto)
}

/// Comparador: a devolução (rotina pura).
unsafe extern "thiscall" fn comparar_teto_de_preco_pelo_estatuto(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(1349);
    let r = o(this, clube, pessoa, estatuto);
    let n = teto_de_preco_pelo_estatuto(this, clube, pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("teto_de_preco_pelo_estatuto(clube {:x}, pessoa {:x}, estatuto {}) original={} rust={}", clube, pessoa, estatuto as i8, r, n));
    r
}

// ---------- 261a leva: a verba de transferências repartida pelas vagas da lista de observação, o estatuto permitido pela idade e o nome das listas do olheiro (shortlist_manager.cpp, scout_manager.cpp) ----------

unsafe fn cpy3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy3_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpy3_f64(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// A pessoa de índice `i` ([0xae23bc], 0x6e bytes; a conta da original em 32 bits, com índice negativo inclusive).
unsafe fn cpy3_pessoa(i: i32) -> usize {
    cpy3_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e))
}

/// { 0x83b6e0 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::allocate_transfer_budget(clube)`, pela
/// atualização da lista do clube (0x83a730, já em Rust), pelas listas iniciais (0x83a1b0) e pela lista refeita
/// (0x84abe0). Clube do humano (0x600a50 com 0, 1), seleção (0x53b900), sem finanças (0x5a0590) ou lista fora do estado
/// 1: nada. A verba (0x598800 com 0, 0, 0, 1, 0; negativa: 0) se reparte pelas vagas abertas (alvo −1) depois de
/// ordenar as 40 (qsort 0x83b690): entre as primeiras 8 (ou até a quarta vaga de jogador aberta), cada vaga aberta
/// recebe em +7 a verba / (abertas × fator do tipo e do estatuto); o resto da verba passa pelas vagas ainda sem alvo
/// (reduções pelos lances guardados, pelo valor de +0xb sobre a soma e pelos fatores de 0x969540, 0x968968), cada uma
/// preenchida pela 0x83f3a0; o que sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
/// (0x846920 acima de 0x968e58, sorteios de 10 e 100 dias, ofertas 0x8c5cf0).
unsafe extern "thiscall" fn distribuir_verba_de_transferencias(this: usize, clube: usize) {
    // 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14); 0x53b900: cdecl(clube) (add esp, 4); 0x5a0590:
    // thiscall([0xae2a38], clube); 0x598800: thiscall(finanças, 0, 0, 0, 1, 0) (5 push); 0x945333: cdecl(vetor, 0x28,
    // 0x25, 0x83b690) (add esp, 0x10); 0x8c61e0: thiscall([0xde1f54], lance) (1 push); 0x83f3a0: thiscall(this, vaga,
    // clube, 0) (mov ecx, [this local]; 3 push); 0x4cf5a0: thiscall([0xae1050], pessoa); 0x544590: cdecl(pessoa, 0)
    // (add esp, 8); 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x846920: thiscall(this, pessoa, clube, 1, 0,
    // estatuto, 0) (6 push, devolve em st0); 0x90cfc0: cdecl(n) (add esp, 4); 0x85fa00: thiscall(contrato) (sem push,
    // movsx ax); 0x8c5cf0: thiscall([0xde1f54], pessoa, 0, 0) (3 push); 0x54c8e0: thiscall(0xae2c90, saída, dias)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x598800usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let lance_de: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x83f3a0usize);
    if humano(clube, 0, 1, 0, 0) != 0 || selecao(clube) != 0 {
        return;
    }
    let fin = financas(cpy3_d(0xae2a38), clube);
    if fin == 0 {
        return;
    }
    let mut verba = verba_de(fin, 0, 0, 0, 1, 0) as i32;
    if verba < 0 {
        verba = 0;
    }
    let entrada = cpy3_d(this + 0xdc).wrapping_add(cpy3_d(clube).wrapping_mul(9));
    if cpy3_b(entrada + 4) != 1 {
        return;
    }
    ordenar(cpy3_d(entrada), 0x28, 0x25, 0x83b690);
    let vaga = |i: i32| cpy3_d(entrada).wrapping_add((i as usize).wrapping_mul(0x25));
    // as vagas abertas entre as primeiras 8, ou até a quarta vaga de jogador aberta
    let mut limite: i8 = 8;
    let mut abertas: i8 = 0;
    let mut jogadores: u8 = 0;
    let mut soma: i32 = 0;
    let mut i: i8 = 0;
    loop {
        let v = vaga(i as i32);
        if cpy3_i(v + 0xf) == -1 {
            if cpy3_b(v) == 0xb {
                jogadores = jogadores.wrapping_add(1);
                if jogadores == 4 {
                    limite = i.wrapping_add(1);
                }
            }
            soma = soma.wrapping_add(cpy3_i(v + 0xb));
            abertas = abertas.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        if i >= limite {
            break;
        }
    }
    let a = abertas as i32;
    let mut repartido: i32 = 0;
    if limite > 0 {
        for k in 0..limite as i32 {
            let v = vaga(k);
            if cpy3_i(v + 0xf) != -1 {
                continue;
            }
            let t = cpy3_b(v);
            let divisor = if t == 0xb {
                match ((cpy3_b(v + 0x24) >> 1) & 0xf).wrapping_sub(2) {
                    0 | 1 => {
                        crate::comparacao::ramo("cpy_3 verba da vaga de titular");
                        Some(a * 2) // cpy3: a verba da vaga de titular
                    }
                    2 => Some(a * 4),
                    3 => Some(a * 10),
                    _ => None,
                }
            } else {
                if cpy3_i(v + 0x13) != -1 {
                    continue;
                }
                let rico = cpy3_b(clube + 0x64) == 1;
                match (t as i8 as i32).wrapping_sub(6) {
                    0 | 2 => Some(if rico { a * 10 } else { 0 }),
                    1 => Some(if rico { a * 1000 } else { 0 }),
                    3 | 4 => Some(if rico { a * 20 } else { 0 }),
                    _ => None,
                }
            };
            let Some(dv) = divisor else { continue };
            let x = if dv == 0 { 0 } else { verba.wrapping_div(dv) };
            cpy3_pi(v + 7, x);
            repartido = repartido.wrapping_add(x);
        }
    }
    let mut resto = verba.wrapping_sub(repartido);
    let mut baixos: i8 = 0;
    if limite > 0 {
        for k in 0..limite as i32 {
            let v = vaga(k);
            let t = cpy3_b(v);
            if t != 0xb && cpy3_i(v + 0x13) > -1 {
                continue;
            }
            let alvo = cpy3_i(v + 0xf);
            if alvo > -1 || t == 7 {
                continue;
            }
            let lance = cpy3_i(v + 0x17);
            if lance > -1 {
                if alvo == -1 {
                    cpy3_pi(v + 0x17, -1);
                } else if t == 0xb {
                    let l = cpy3_d(lance_de(cpy3_d(0xde1f54), lance));
                    if l != 0 {
                        resto = resto.wrapping_sub(cpy3_i(l + 0x1b));
                    }
                } else {
                    resto = resto.wrapping_sub(cpy3_i(cpy3_pessoa(alvo) + 0x52));
                }
                continue;
            }
            if cpy3_b(clube + 0x64) != 1 && t != 0xb {
                cpy3_pi(v + 7, 0);
            } else {
                let base = if repartido >= 1000 { repartido } else { 1000 };
                let e = cpy3_i(v + 7);
                let x = ftol((e as f64) / (base as f64) * (resto as f64) * cpy3_f64(0x969540));
                crate::comparacao::ramo("cpy_3 reducao pela sobra");
                cpy3_pi(v + 7, e.wrapping_sub(x)); // cpy3: a redução da vaga pela sobra
            }
            let marca = cpy3_b(v + 0x24) & 0x1e;
            if marca <= 6 {
                if baixos < 3 {
                    baixos = baixos.wrapping_add(1);
                    let s = if soma >= 1 { soma } else { 1 };
                    let b = cpy3_i(v + 0xb);
                    let b = if b >= 1 { b } else { 1 };
                    let x = ftol((resto as f64) / (s as f64) * (b as f64));
                    cpy3_pi(v + 7, cpy3_i(v + 7).wrapping_add(x));
                }
            } else {
                let s = if soma >= 1 { soma } else { 1 };
                let b = cpy3_i(v + 0xb);
                let b = if b >= 1 { b } else { 1 };
                let x = ftol((resto as f64) / (s as f64) * (b as f64) * cpy3_f64(0x968968));
                cpy3_pi(v + 7, cpy3_i(v + 7).wrapping_add(x));
            }
            *((v + 0x24) as *mut u8) &= 0xfe;
            crate::comparacao::ramo("cpy_3 vaga preenchida");
            preencher(this, v, clube, 0); // cpy3: a vaga é preenchida
            let alvo = cpy3_i(v + 0xf);
            if alvo > -1 && cpy3_b(v + 0x24) & 1 == 0 {
                resto = resto.wrapping_add(ftol(cpy3_i(cpy3_pessoa(alvo) + 0x52) as f64 * cpy3_f64(0x96f878)));
            }
            soma = soma.wrapping_sub(cpy3_i(v + 0xb));
        }
    }
    // a sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
    if limite >= 0x28 {
        return;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestado: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x544590usize);
    let contrato_no: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x85fa00usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut k = limite;
    while resto > 0 {
        let v = vaga(k as i32);
        let alvo = cpy3_i(v + 0xf);
        let candidata = 'c: {
            if cpy3_b(v) != 0xb || alvo <= -1 || cpy3_i(v + 0x17) != -1 {
                break 'c false;
            }
            let p = cpy3_pessoa(alvo);
            if cpy3_d(p + 0x39) != clube {
                break 'c false;
            }
            let c = contrato_de(cpy3_d(0xae1050), p);
            *(0xae1080 as *mut usize) = c;
            let do_contrato = if c != 0 { cpy3_d(0xae23b4).wrapping_add(cpy3_d(c + 4).wrapping_mul(0x245)) } else { 0 };
            if do_contrato == clube || emprestado(p, 0) != 0 {
                break 'c false;
            }
            let c2 = contrato_no(cpy3_d(0xae1050), p, clube);
            let estatuto = if c2 != 0 { (cpy3_b(c2 + 0x4f) >> 4) as u32 } else { 4 };
            let f = interesse(this, p, clube, 1, 0, estatuto, 0);
            if !(f >= cpy3_f64(0x968e58)) {
                break 'c false;
            }
            if c2 != 0 {
                let lim = 20 - sorteio(10);
                if (fim(c2) as i16 as i32) < lim {
                    break 'c true;
                }
            }
            if ofertas(cpy3_d(0xde1f54), p, 0, 0) != 0 {
                break 'c true;
            }
            if c2 == 0 {
                break 'c false;
            }
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, (sorteio(100) + 0x1e) as u32);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = core::ptr::read_unaligned((r + 2) as *const u16);
            let ano_c = core::ptr::read_unaligned((c2 + 0x27) as *const u16);
            ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned((c2 + 0x25) as *const i16))
        };
        if candidata {
            crate::comparacao::ramo("cpy_3 sobra ao jogador do clube");
            cpy3_pi(v + 0xb, cpy3_i(v + 3));
            cpy3_pi(v + 7, resto);
            return;
        }
        k = k.wrapping_add(1);
        if k >= 0x28 {
            return;
        }
    }
}

// ponytail: a 0x83f3a0 (chamada por cada vaga ainda sem alvo) sorteia, acrescenta pares aos vetores com realocação e dá
// notícias: não pode rodar duas vezes. Só a original; a cópia leva os ramos e as mutações.
unsafe extern "thiscall" fn comparar_distribuir_verba_de_transferencias(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1350)(this, clube)
}

/// { 0x84b4d0 } (stdcall, ret 8, shortlist_manager.cpp; chamada com ecx = o Shortlist_Manager, que ela não lê):
/// `shortlist_manager_squad_status_allowed(pessoa, estatuto)`, pela tela de contrato (0x4dd0f0) e pela de transferência
/// (0x8f6770). Os estatutos 5 e 6 (jovens) só abaixo de 24 anos (+0x18, com sinal); os outros, 1. Sem ficha de jogador
/// (+0x61): a original (caixa "Error", 0).
unsafe extern "stdcall" fn estatuto_permitido(pessoa: usize, estatuto: u32) -> u32 {
    if cpy3_d(pessoa + 0x61) == 0 {
        return orig::<unsafe extern "stdcall" fn(usize, u32) -> u32>(1351)(pessoa, estatuto); // caixa: só a original
    }
    match estatuto as u8 {
        5 | 6 => ((cpy3_b(pessoa + 0x18) as i8) < 0x18) as u32,
        _ => 1,
    }
}

/// Comparador: a devolução (rotina pura); sem ficha de jogador, só a original (caixa).
unsafe extern "stdcall" fn comparar_estatuto_permitido(pessoa: usize, estatuto: u32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32) -> u32>(1351);
    if cpy3_d(pessoa + 0x61) == 0 {
        return o(pessoa, estatuto);
    }
    let r = o(pessoa, estatuto);
    let n = estatuto_permitido(pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("estatuto_permitido(pessoa {:x}, estatuto {}) original={} rust={}", pessoa, estatuto as i8, r, n));
    r
}

/// { 0x7fc150 } (cdecl, scout_manager.cpp): `scout_manager_write_list_name(indice, texto, tamanho)`, pelo construtor do
/// SCOUT_MANAGER (0x7f6760) e pelas telas de busca. O nome traduzido (0x66f4e0) da lista `indice`: 0 "Short List", 1
/// "Player Search", 2 "Staff Search", 3..9 "Scout #<%d - number>" com `indice` − 2. Devolve 1. Índice fora de 0..9,
/// texto nulo ou tamanho abaixo de 50: a original (caixa "Error", 0).
unsafe extern "C" fn nome_da_lista_do_olheiro(indice: u32, texto: usize, tamanho: i32) -> u32 {
    // 0x66f4e0: cdecl(texto, formato, ...) (add esp, 8 ou 0xc)
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || texto == 0 || tamanho < 0x32 {
        return orig::<unsafe extern "C" fn(u32, usize, i32) -> u32>(1352)(indice, texto, tamanho); // caixa: só a original
    }
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    match k {
        0 => traduzir(texto, 0xa7dd94),
        1 => traduzir(texto, 0xa7dd84),
        2 => traduzir(texto, 0xa7dd74),
        _ => traduzir(texto, 0xa7dd5c, k as i32 - 2),
    };
    1
}

/// Comparador: lado a lado pelo texto (até 0x100 bytes do tamanho dado), e a devolução; nos casos de erro, só a
/// original (caixa).
unsafe extern "C" fn comparar_nome_da_lista_do_olheiro(indice: u32, texto: usize, tamanho: i32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, i32) -> u32>(1352);
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || texto == 0 || tamanho < 0x32 {
        return o(indice, texto, tamanho);
    }
    let reg = [(texto, (tamanho as usize).min(0x100))];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(indice, texto, tamanho), || nome_da_lista_do_olheiro(indice, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("nome_da_lista_do_olheiro(indice {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
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

// ---------- 261a leva: os inscritos da Liga dos Campeões: a lista das quatro bandas, a tabela da qualificação e a fase seguinte (european_cup.cpp) ----------

unsafe fn cpy5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy5_alocar(n: u32) -> usize {
    let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    f(n)
}

/// Uma entrada de 6 bytes da lista dos inscritos: o clube (+0), a rodada em que entra (+4) e +5 = 0 (nesta ordem,
/// como a original).
unsafe fn cpy5_entrada(p: usize, clube: usize, rodada: u8) {
    core::ptr::write_unaligned(p as *mut usize, clube);
    *((p + 5) as *mut u8) = 0;
    *((p + 4) as *mut u8) = rodada;
}

/// O laço da 0x584fa0 sobre os clubes ([0xae23b4], 0x245 bytes, [0xae235c] deles): quem tem a competição europeia
/// (+0x1db) da Liga dos Campeões ([0x9cf6e8]) entra pela banda (+0x1df) na lista `t`: 1 em +0 (até 16, rodada 7),
/// 2 em +0x60 (até 18, rodada 8), 3 em +0xcc (até 19, rodada 0xd), 4 em +0x13e (até 18, rodada 0). Devolve a soma
/// das quatro contas.
unsafe fn cpy5_preencher(t: usize) -> i32 {
    let clubes = cpy5_i(0xae235c);
    let (mut n1, mut n2, mut n3, mut n4) = (0i32, 0i32, 0i32, 0i32);
    let (mut o1, mut o2, mut o3, mut o4) = (0usize, 0usize, 0usize, 0usize);
    if clubes <= 0 {
        return 0;
    }
    let cl = cpy5_i(0x9cf6e8);
    let mut c = cpy5_d(0xae23b4);
    let mut i: i32 = 0;
    loop {
        if cpy5_i(c + 0x1db) == cl {
            match cpy5_b(c + 0x1df) {
                1 => {
                    if o1 < 0x60 {
                        cpy5_entrada(t + o1, c, 7);
                        n1 += 1;
                        o1 += 6;
                    }
                }
                2 => {
                    if o2 < 0x6c {
                        crate::comparacao::ramo("cpy_5 entrada da banda 2");
                        cpy5_entrada(t + 0x60 + o2, c, 8); // cpy5: a banda 2 entra na rodada 8
                        n2 += 1;
                        o2 += 6;
                    }
                }
                3 => {
                    if o3 < 0x72 {
                        crate::comparacao::ramo("cpy_5 entrada da banda 3");
                        cpy5_entrada(t + 0xcc + o3, c, 0xd); // cpy5: a banda 3 entra na rodada 0xd
                        n3 += 1;
                        o3 += 6;
                    }
                }
                4 => {
                    if o4 < 0x6c {
                        crate::comparacao::ramo("cpy_5 entrada da banda 4");
                        cpy5_entrada(t + 0x13e + o4, c, 0); // cpy5: a banda 4 entra na rodada 0
                        n4 += 1;
                        o4 += 6;
                    }
                }
                _ => {}
            }
        }
        c = c.wrapping_add(0x245);
        i += 1;
        if i as i16 as i32 >= clubes {
            break;
        }
    }
    n1 + n2 + n3 + n4
}

/// { 0x584fa0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::collect_entrants()`, do construtor (0x583888, na
/// carreira nova) e do começo da temporada (0x584e5d). +0x36 e +0x38 = 71; libera a lista antiga (+0x14, free
/// 0x9452ca) e aloca 6 × 71 bytes (0x944e46; sem memória: a caixa da linha 0x44e e 0, com +0x14 nulo); preenche as
/// quatro bandas (cpy5_preencher). Sem as 71 entradas: +0x36 = 0 e 0; senão 1.
unsafe extern "thiscall" fn inscritos_da_liga_dos_campeoes(this: usize) -> u32 {
    *((this + 0x36) as *mut i16) = 0x47;
    *((this + 0x38) as *mut i16) = 0x47;
    let velha = cpy5_d(this + 0x14);
    if velha != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        liberar(velha);
    }
    let t = cpy5_alocar((cpy5_w(this + 0x36) as i32).wrapping_mul(6) as u32);
    core::ptr::write_unaligned((this + 0x14) as *mut usize, t);
    if t == 0 {
        aviso_em(0x9c4900, 0x44e);
        return 0;
    }
    if cpy5_preencher(t) != cpy5_w(this + 0x36) as i32 {
        *((this + 0x36) as *mut i16) = 0;
        return 0;
    }
    1
}

/// Comparador da 0x584fa0: a original aloca (só ela roda); o plano confere depois tudo o que ela gravou: a lista nova
/// copiada e repreenchida pela cópia (cpy5_preencher só lê os clubes, que a original não muda) tem de sair igual,
/// +0x36, +0x38 e a devolução.
unsafe extern "thiscall" fn comparar_inscritos_da_liga_dos_campeoes(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1356)(this);
    let t = cpy5_d(this + 0x14);
    if t == 0 {
        PLACAR.registrar(r == 0, || format!("inscritos_da_liga_dos_campeoes({:x}) sem lista original={}", this, r));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x47 * 6).to_vec();
    let total = cpy5_preencher(v.as_mut_ptr() as usize);
    let n = if total == 0x47 { 1 } else { 0 };
    let igual = r == n
        && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x47 * 6)
        && cpy5_w(this + 0x36) == if n == 1 { 0x47 } else { 0 }
        && cpy5_w(this + 0x38) == 0x47;
    PLACAR.registrar(igual, || format!("inscritos_da_liga_dos_campeoes({:x}) original={} +0x36={} rust={} total={}", this, r, cpy5_w(this + 0x36), n, total));
    r
}

/// O laço da 0x584990 sobre as 55 linhas (+0x3e) da tabela `t`: a linha i leva o clube da entrada (+0x36) − i − 1 da
/// lista (+0x14; lista vazia: 0) e a rodada pela entrada (+0x36) − i, byte +4 da anterior (8 → 2, 0xd → 1, outra →
/// 0); +5 = 0. Com +0x36 = 0 a original lê antes da lista (índice negativo), como aqui.
unsafe fn cpy5_qualificacao(this: usize, t: usize) {
    let mut i: i32 = 0;
    let mut k = 0usize;
    while i < cpy5_w(this + 0x3e) as i32 {
        let n = cpy5_w(this + 0x36) as i32;
        let lista = cpy5_d(this + 0x14);
        let clube = if n != 0 { cpy5_d(lista.wrapping_add((n - i - 1).wrapping_mul(6) as usize)) } else { 0 };
        core::ptr::write_unaligned((t + k) as *mut usize, clube);
        let lista = cpy5_d(this + 0x14);
        let codigo = cpy5_b(lista.wrapping_add(((n - i).wrapping_mul(6) - 2) as usize));
        *((t + k + 4) as *mut u8) = match codigo {
            8 => 2,
            0xd => 1,
            _ => 0,
        };
        *((t + k + 5) as *mut u8) = 0;
        i += 1;
        k += 6;
    }
}

/// { 0x584990 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::build_qualifying_entry_table()`, logo depois da
/// 0x584fa0 (0x58388f, 0x584e9a). +0x3e = 55; aloca 6 × 55 bytes em +0xa7 (sem memória: a caixa da linha 0x2f6 e −1)
/// e monta as linhas da qualificação de trás para a frente (cpy5_qualificacao); devolve 1.
unsafe extern "thiscall" fn tabela_da_qualificacao_da_liga_dos_campeoes(this: usize) -> i32 {
    *((this + 0x3e) as *mut i16) = 0x37;
    let t = cpy5_alocar(0x14a);
    core::ptr::write_unaligned((this + 0xa7) as *mut usize, t);
    if t == 0 {
        aviso_em(0x9c4900, 0x2f6);
        return -1;
    }
    cpy5_qualificacao(this, t);
    1
}

/// Comparador da 0x584990: a original aloca (só ela roda); depois a tabela dela, copiada e remontada pela cópia,
/// tem de sair igual, com +0x3e = 55 e a devolução.
unsafe extern "thiscall" fn comparar_tabela_da_qualificacao_da_liga_dos_campeoes(this: usize) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1357)(this);
    let t = cpy5_d(this + 0xa7);
    if t == 0 || cpy5_w(this + 0x3e) != 0x37 {
        PLACAR.registrar(r == -1 && t == 0, || format!("tabela_da_qualificacao_da_liga_dos_campeoes({:x}) original={} tabela={:x} +0x3e={}", this, r, t, cpy5_w(this + 0x3e)));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x14a).to_vec();
    cpy5_qualificacao(this, v.as_mut_ptr() as usize);
    let igual = r == 1 && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x14a);
    PLACAR.registrar(igual, || format!("tabela_da_qualificacao_da_liga_dos_campeoes({:x}) original={} tabela={:x?}", this, r, &v[..]));
    r
}

/// { 0x5851d0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::advance_stage()` (a virtual +0x28, do passe da
/// copa 0x51f904). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x495. Senão +0x30 + 1 e, na fase
/// 0, o sorteio dos grupos da fase 1 (0x585290, thiscall(this)); na 8, os da fase 2 (0x585ab0, thiscall(this, 0),
/// ret 4); na 12, as quartas (0x586330, thiscall(this)). Quem chama não lê a devolução.
unsafe extern "thiscall" fn proxima_fase_da_liga_dos_campeoes(this: usize) {
    let f = cpy5_i(this + 0x30);
    if f >= cpy5_i(this + 0x2c).wrapping_sub(1) {
        return aviso_em(0x9c4900, 0x495);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    crate::comparacao::ramo("cpy_5 fase seguinte");
    match f {
        0 => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x585290usize);
            sortear(this);
        }
        8 => {
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x585ab0usize);
            sortear(this, 0);
        }
        0xc => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x586330usize);
            sortear(this);
        }
        _ => {}
    }
}

// ponytail: os sorteios sorteiam, alocam as fases e mandam notícias: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_proxima_fase_da_liga_dos_campeoes(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1358)(this)
}

// ---------- 261a leva: o jogo da Liga dos Campeões: o prêmio e o coeficiente do jogo, o progresso do clube e o conflito de datas da competição (european_cup.cpp, comp.cpp) ----------

unsafe fn cpy6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de finanças do clube (0x5a0590: thiscall(tabela [0xae2a38], clube), ret 4).
unsafe fn cpy6_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cpy6_d(0xae2a38), clube)
}

/// Soma no saldo, em +0xa8 e em +0x148 (0x5999a0: thiscall(finanças, valor), ret 4).
unsafe fn cpy6_somar(fin: usize, valor: i32) {
    let f: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    f(fin, valor)
}

/// Os pontos do coeficiente da UEFA para a nação (0x9058b0: thiscall(tabela [0xde1f58], nação, pontos), ret 8; soma
/// o byte dos pontos na palavra +0xf + 18 × índice (+0x1df da seleção 0x53b3d0 da nação)).
unsafe fn cpy6_coeficiente(nacao: usize, pontos: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x9058b0usize);
    f(cpy6_d(0xde1f58), nacao, pontos);
}

/// O ensaio do comparador da 0x583b10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca) e sem a base da copa
/// (0x51a150, que processa o jogo inteiro e não pode rodar duas vezes).
static mut CPY6_ENSAIO: bool = false;

/// A notícia do prêmio (Comp 0x48e3e0: thiscall(this, clube, valor, 0, −1, 0, jogo, −2), ret 0x1c); no ensaio, nada.
unsafe fn cpy6_noticia(this: usize, clube: usize, valor: u32, jogo: usize) {
    if CPY6_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    f(this, clube, valor, 0, -1, 0, jogo, -2);
}

/// { 0x583b10 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::process_fixture(jogo, arg)` (a virtual +4 da Liga dos
/// Campeões, a cada jogo dela). O placar: +0x47/+0x48 (com +0x47 = −1, +0x43/+0x44), com sinal. O vencedor ganha 2
/// pontos no coeficiente da sua nação (+0x53; 0x9058b0), o empate 1 para cada. Na fase de grupos (+0x42 de 0 a 11):
/// 200 mil ao vencedor ou 100 mil a cada um no empate (0x5a0590 + 0x5999a0), com a notícia do prêmio (0x48e3e0;
/// no empate, as duas somas antes das duas notícias). Depois a base da copa (0x51a150 com o jogo e `arg`).
unsafe extern "thiscall" fn processar_jogo_da_liga_dos_campeoes(this: usize, jogo: usize, arg: u32) {
    let (ga, gb) = if cpy6_b(jogo + 0x47) != 0xff {
        (cpy6_b(jogo + 0x47) as i8, cpy6_b(jogo + 0x48) as i8)
    } else {
        (cpy6_b(jogo + 0x43) as i8, cpy6_b(jogo + 0x44) as i8)
    };
    let (a, b) = (cpy6_d(jogo + 0x1c), cpy6_d(jogo + 0x20));
    let (vencedor, perdedor) = if ga > gb {
        cpy6_coeficiente(cpy6_d(a + 0x53), 2);
        (cpy6_d(jogo + 0x1c), cpy6_d(jogo + 0x20))
    } else if ga < gb {
        cpy6_coeficiente(cpy6_d(b + 0x53), 2);
        (cpy6_d(jogo + 0x20), cpy6_d(jogo + 0x1c))
    } else {
        cpy6_coeficiente(cpy6_d(a + 0x53), 1);
        cpy6_coeficiente(cpy6_d(cpy6_d(jogo + 0x20) + 0x53), 1);
        (0, 0)
    };
    let _ = perdedor;
    let fase = cpy6_b(jogo + 0x42) as i8;
    if fase > -1 && fase < 0xc {
        if vencedor != 0 {
            crate::comparacao::ramo("cpy_6 premio da vitoria");
            cpy6_somar(cpy6_financas(vencedor), 0x30d40); // cpy6: o prêmio da vitória
            cpy6_noticia(this, vencedor, 0x30d40, jogo);
        } else {
            crate::comparacao::ramo("cpy_6 premio do empate");
            cpy6_somar(cpy6_financas(cpy6_d(jogo + 0x1c)), 0x186a0); // cpy6: o prêmio do empate do mandante
            cpy6_somar(cpy6_financas(cpy6_d(jogo + 0x20)), 0x186a0);
            cpy6_noticia(this, cpy6_d(jogo + 0x1c), 0x186a0, jogo);
            cpy6_noticia(this, cpy6_d(jogo + 0x20), 0x186a0, jogo);
        }
    }
    if CPY6_ENSAIO {
        return;
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x51a150usize);
    base(this, jogo, arg);
}

/// A palavra do coeficiente da nação do clube: [0xde1f58] + 18 × (+0x1df da seleção 0x53b3d0 da nação +0x53) + 0xf,
/// como a 0x9058b0 calcula; 0 sem tabela, sem nação ou sem seleção.
unsafe fn cpy6_casa_do_coeficiente(clube: usize) -> usize {
    let tabela = cpy6_d(0xde1f58);
    let nacao = cpy6_d(clube + 0x53);
    if tabela == 0 || nacao == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let s = selecao(nacao);
    if s == 0 {
        return 0;
    }
    tabela.wrapping_add((cpy6_b(s + 0x1df) as i8 as isize * 18) as usize + 0xf)
}

/// Comparador da 0x583b10, em ensaio: a cópia sem a notícia e sem a base, e a original inteira, a partir do mesmo
/// estado (as palavras do coeficiente das duas nações e os registros de finanças dos dois clubes, restaurados depois
/// do ensaio). Confere as palavras do coeficiente, que a base não toca, salvo pelo sorteio das fases (0x585290,
/// 0x585ab0: o coeficiente dos classificados), que só roda quando a fase (+0x30) anda: aí, sem comparação. As
/// finanças não entram (a base paga o prêmio da rodada da copa nos mesmos campos, 0x51a2fe); o prêmio segue o ramo do
/// placar que os pontos do coeficiente já conferem. Sem clube, nação, seleção ou finanças: só a original.
unsafe extern "thiscall" fn comparar_processar_jogo_da_liga_dos_campeoes(this: usize, jogo: usize, arg: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1359);
    let (a, b) = if jogo != 0 { (cpy6_d(jogo + 0x1c), cpy6_d(jogo + 0x20)) } else { (0, 0) };
    if a == 0 || b == 0 {
        return o(this, jogo, arg); // ponytail: sem clube, só a original
    }
    let (ca, cb) = (cpy6_casa_do_coeficiente(a), cpy6_casa_do_coeficiente(b));
    let (fa, fb) = (cpy6_financas(a), cpy6_financas(b));
    if ca == 0 || cb == 0 || fa == 0 || fb == 0 {
        return o(this, jogo, arg); // ponytail: sem nação, seleção ou finanças, só a original (cai do mesmo jeito)
    }
    let conferir = [(ca, 2), (cb, 2)];
    let antes = crate::comparacao::Memoria::ler(&[(ca, 2), (cb, 2), (fa, 0x167), (fb, 0x167)]);
    let fase = cpy6_i(this + 0x30);
    CPY6_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| processar_jogo_da_liga_dos_campeoes(this, jogo, arg)));
    CPY6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&conferir);
    antes.escrever();
    o(this, jogo, arg);
    if cpy6_i(this + 0x30) != fase {
        return; // ponytail: a fase andou na base e o sorteio somou coeficiente: sem comparação
    }
    let mo = crate::comparacao::Memoria::ler(&conferir);
    PLACAR.registrar(mo == mr, || format!("processar_jogo_da_liga_dos_campeoes(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// { 0x586c00 } (thiscall, ret 0x14, european_cup.cpp): `EuropeanCup::set_club_progress(clube, fase, p1, p2, p3)` (a
/// virtual +0x68, da base da copa 0x521b30): converte os três códigos de progresso pela fase e grava no registro do
/// clube na estatística da competição (0x4a4850 de +8, sem conferir o nulo, como a original) +0x73 ← p1, +0x74 ← p2,
/// +0x75 ← p3. Fase −1 (a qualificação): 1 → 0x19 (p2: 1), 17 → 0x21, 33 → 0x31, 47 → 0x3f; fase abaixo de 8 (com
/// sinal): 1 e 2 → 1 (p3: 0xd), 3 → 0x11, 4 → 0x19; abaixo de 12: 1 e 2 → 1 (p3: 7), 3 → 9, 4 → 0xd; os outros
/// códigos e as fases de 12 em diante passam como estão.
unsafe extern "thiscall" fn progresso_do_clube_na_liga_dos_campeoes(this: usize, clube: usize, fase: u32, p1: u32, p2: u32, p3: u32) {
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = buscar(cpy6_d(this + 8), clube);
    let (fase, p1, p2, p3) = (fase as u8 as i8, p1 as u8, p2 as u8, p3 as u8);
    let (c73, c74, c75) = if fase == -1 {
        let m = |p: u8, um: u8| match p {
            1 => um,
            17 => 0x21,
            33 => 0x31,
            47 => 0x3f,
            _ => p,
        };
        (m(p1, 0x19), m(p2, 1), m(p3, 0x19))
    } else if fase < 8 {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 0x11,
            4 => 0x19,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 0xd))
    } else if fase < 0xc {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 9,
            4 => 0xd,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 7))
    } else {
        (p1, p2, p3)
    };
    *((e + 0x74) as *mut u8) = c74;
    *((e + 0x73) as *mut u8) = c73;
    *((e + 0x75) as *mut u8) = c75;
}

/// Comparador da 0x586c00: lado a lado nos três bytes do registro do clube (+0x73..+0x75); sem registro (a original
/// cai), só a original.
unsafe extern "thiscall" fn comparar_progresso_do_clube_na_liga_dos_campeoes(this: usize, clube: usize, fase: u32, p1: u32, p2: u32, p3: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1360);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = buscar(cpy6_d(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, p1, p2, p3); // ponytail: sem registro, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &[(e + 0x73, 3)],
        || o(this, clube, fase, p1, p2, p3),
        || progresso_do_clube_na_liga_dos_campeoes(this, clube, fase, p1, p2, p3),
    );
    PLACAR.registrar(igual, || format!("progresso_do_clube_na_liga_dos_campeoes(fase {} {} {} {}) original={:x?} rust={:x?}", fase as u8 as i8, p1 as u8, p2 as u8, p3 as u8, mo, mr));
}

/// { 0x48e360 } (thiscall, ret 8, comp.cpp): `Comp::counts_for_fixture_clash(jogo, outro)` (a virtual +0x60 das
/// competições, do conflito de datas 0x5adba0): 1 se o algarismo das unidades (0x4b0850) da rodada (+0x30, com sinal)
/// é 0 (rodada de 1000 em diante também dá 0), senão 0. `outro` não é lido.
unsafe extern "thiscall" fn conta_para_conflito_de_datas(_this: usize, jogo: usize, _outro: usize) -> u32 {
    let unidades: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    (unidades(cpy6_w(jogo + 0x30) as i32) == 0) as u32
}

unsafe extern "thiscall" fn comparar_conta_para_conflito_de_datas(this: usize, jogo: usize, outro: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1361)(this, jogo, outro);
    let n = conta_para_conflito_de_datas(this, jogo, outro);
    PLACAR.registrar(r == n, || format!("conta_para_conflito_de_datas(rodada {}) original={} rust={}", cpy6_w(jogo + 0x30), r, n));
    r
}

// ---------- 261a leva: o árbitro de cada jogo: a escolha do árbitro, as marcas de ocupado do dia e a renovação dos árbitros da liga (officials_manager.cpp) ----------

unsafe fn cpy8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O bloco de árbitros da nação `nacao` no gerenciador (0x102 bytes em [this+0xc]: +4 conta, +8 primeiro índice, +0x10
/// palavra com o número de faixas de liga, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição).
unsafe fn cpy8_bloco(this: usize, nacao: i32) -> usize {
    cpy8_d(this + 0xc).wrapping_add(nacao.wrapping_mul(0x102) as usize)
}

/// O id da nação do clube (+0x53, ponteiro) ou, sem nação, `sem`.
unsafe fn cpy8_nacao_do_clube(clube: usize) -> Option<i32> {
    let n = cpy8_d(clube + 0x53);
    if n != 0 { Some(cpy8_i(n)) } else { None }
}

/// { 0x79c7b0 } (thiscall, ret 8, officials_manager.cpp): `OfficialsManager::choose_match_official(jogo, saida)`, pela
/// montagem do dia (0x6b3c10, saída em +0x2e do jogo). Jogo tirado (−2): 0 (eax 0xffffff00, como a original). A faixa
/// de árbitros: sem competição, a nação do mandante (+0x1c, +0x53) ou uma sorteada; amistoso (0x5c6710 em [0xae2c74]),
/// a do mandante; competição sem objeto ([0xadadfc]), como sem competição; competição dos tipos 2 e 4 (+0x42), o quarto
/// de cima (ftol de 0,75 e 0,25 da conta) da primeira nação, a partir de uma sorteada, do continente (+0x59 da
/// competição, ou +0x71 da nação do mandante, do visitante, ou um sorteado) que não seja a de nenhum dos dois clubes;
/// nas outras, a faixa da competição no bloco da nação dela (+0x5d; ou do mandante, do visitante, ou sorteada), a última
/// que bater, ou o bloco inteiro. Conta 1: o primeiro; senão primeiro + sorteio(conta − 1). Ocupado (o byte do id do
/// árbitro em [this+8]): o seguinte da faixa, dando a volta; todos ocupados, o sorteado. Grava o índice (palavra) e
/// devolve 1 no al (o resto do eax com o índice, como a original).
unsafe extern "thiscall" fn escolher_arbitro_da_partida(this: usize, jogo: usize, saida: usize) -> u32 {
    if cpy8_i(jogo) == -2 {
        return 0xffff_ff00;
    }
    let nacoes = cpy8_i(0xae2350);
    let mandante = cpy8_d(jogo + 0x1c);
    let visitante = cpy8_d(jogo + 0x20);
    let comp = cpy8_d(jogo + 0x14);
    let faixa_da_nacao = |nacao: i32| {
        let b = cpy8_bloco(this, nacao);
        (cpy8_i(b + 8), cpy8_i(b + 4))
    };
    let (ini, n) = if comp == 0 {
        faixa_da_nacao(cpy8_nacao_do_clube(mandante).unwrap_or_else(|| sortear(nacoes)))
    } else {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistoso(cpy8_d(0xae2c74), comp) != 0 {
            faixa_da_nacao(cpy8_i(cpy8_d(mandante + 0x53))) // como a original, sem conferir a nação
        } else {
            let c = cpy8_d(cpy8_d(0xadadfc).wrapping_add(cpy8_i(comp).wrapping_mul(4) as usize));
            if c == 0 {
                faixa_da_nacao(cpy8_nacao_do_clube(mandante).unwrap_or_else(|| sortear(nacoes)))
            } else {
                let tipo = cpy8_b(c + 0x42) as i8;
                if tipo == 2 || tipo == 4 {
                    let continente_do = |clube: usize| {
                        let n = cpy8_d(clube + 0x53);
                        if n != 0 && cpy8_d(n + 0x71) != 0 { Some(cpy8_i(cpy8_d(n + 0x71))) } else { None }
                    };
                    let continente = if cpy8_d(comp + 0x59) != 0 {
                        cpy8_i(cpy8_d(comp + 0x59))
                    } else if let Some(k) = continente_do(mandante) {
                        k
                    } else if let Some(k) = continente_do(visitante) {
                        k
                    } else {
                        sortear(cpy8_i(0xae234c))
                    };
                    let inicio = sortear(nacoes);
                    let mut k = inicio;
                    loop {
                        let nac = cpy8_d(0xae23a8).wrapping_add(k.wrapping_mul(0x122) as usize);
                        let cont = cpy8_d(nac + 0x71);
                        if cont != 0 && cpy8_i(cont) == continente
                            && cpy8_nacao_do_clube(mandante) != Some(cpy8_i(nac))
                            && cpy8_nacao_do_clube(visitante) != Some(cpy8_i(nac))
                        {
                            break;
                        }
                        k += 1;
                        if k >= cpy8_i(0xae2350) {
                            k = 0;
                        }
                        if k == inicio {
                            break;
                        }
                    }
                    let b = cpy8_bloco(this, k);
                    let conta = cpy8_i(b + 4) as f64;
                    crate::comparacao::ramo("cpy_8 arbitro neutro");
                    (cpy8_i(b + 8).wrapping_sub(ftol(conta * -0.75)), ftol(conta * 0.25)) // cpy8: o quarto de cima
                } else {
                    let nacao = if cpy8_d(comp + 0x5d) != 0 {
                        cpy8_i(cpy8_d(comp + 0x5d))
                    } else if let Some(k) = cpy8_nacao_do_clube(mandante) {
                        k
                    } else if let Some(k) = cpy8_nacao_do_clube(visitante) {
                        k
                    } else {
                        sortear(nacoes)
                    };
                    let b = cpy8_bloco(this, nacao);
                    let id = cpy8_i(comp);
                    let mut achou = None;
                    for f in 0..cpy8_w(b + 0x10).max(0) as usize {
                        let e = b + 0x12 + f * 0x10;
                        if cpy8_i(e + 0xc) == id {
                            achou = Some((cpy8_i(e), cpy8_i(e + 4)));
                        }
                    }
                    match achou {
                        Some(faixa) => {
                            crate::comparacao::ramo("cpy_8 faixa da liga");
                            faixa // cpy8: a faixa da liga
                        }
                        None => (cpy8_i(b + 8), cpy8_i(b + 4)),
                    }
                }
            }
        }
    };
    let sorteado = if n == 1 {
        ini
    } else {
        crate::comparacao::ramo("cpy_8 arbitro sorteado");
        ini.wrapping_add(sortear(n.wrapping_sub(1))) // cpy8: o sorteio na faixa
    };
    let ocupado = |k: i32| {
        let ficha = cpy8_d(0xae23e8).wrapping_add(k.wrapping_mul(0x2b) as usize);
        cpy8_b(cpy8_d(this + 8).wrapping_add(cpy8_d(ficha))) != 0
    };
    let mut k = sorteado;
    if ocupado(k) {
        let fim = n.wrapping_add(ini);
        loop {
            k += 1;
            if k >= fim {
                k = ini;
            }
            if k == sorteado || !ocupado(k) {
                break;
            }
        }
    }
    core::ptr::write_unaligned(saida as *mut i16, k as i16);
    (k as u32 & 0xffff_ff00) | 1
}

/// Comparador: lado a lado pela palavra de saída e pelo estado do sorteio, e pela devolução.
unsafe extern "thiscall" fn comparar_escolher_arbitro_da_partida(this: usize, jogo: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1362);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 2));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida),
        || crate::aleatorio::sem_contar(|| escolher_arbitro_da_partida(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("escolher_arbitro_da_partida(jogo {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", jogo, r, mo, n, mr));
    r
}

/// { 0x79bf40 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::clear_busy_flags`, no laço do dia
/// (0x5cccd0) e na corrida até a data de início (0x5cc850): zera os [this+4] bytes de ocupado em [this+8] (a conta e o
/// vetor relidos a cada passo). Devolve a conta (0 se não positiva), como o eax da original.
unsafe extern "thiscall" fn limpar_arbitros_ocupados(this: usize) -> u32 {
    let mut i = 0i32;
    while i < cpy8_i(this + 4) {
        *(cpy8_d(this + 8).wrapping_add(i as usize) as *mut u8) = 0;
        i += 1;
    }
    i as u32
}

/// Comparador: lado a lado pelos bytes de ocupado e pela devolução.
unsafe extern "thiscall" fn comparar_limpar_arbitros_ocupados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1363);
    let n = cpy8_i(this + 4);
    let reg = if n > 0 && cpy8_d(this + 8) != 0 { vec![(cpy8_d(this + 8), n as usize)] } else { Vec::new() };
    let (r, m, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || limpar_arbitros_ocupados(this));
    PLACAR.registrar(igual && r == m, || format!("limpar_arbitros_ocupados(conta {}) original={} rust={} memoria igual {}", n, r, m, igual));
    r
}

/// { 0x79cee0 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::renew_officials(comp)`, no começo da
/// temporada de cada liga (83 chamadores). Sem competição, sem objeto ([0xadadfc]), dos tipos 4 e 2 (+0x42) ou sem
/// conta (0x4b0700): 0. A faixa da competição no bloco da nação dela (+0x5d, ou uma sorteada), a primeira que bater,
/// ou o bloco inteiro. Cada árbitro da faixa (fichas de 0x2b bytes em [0xae23e8]; ficha nula: a caixa "Error"): com
/// mais de 45 anos (o ano em byte, menos 1 antes do dia dele), ou mais de 40 e sorteio(3) zero, vira outro: dois nomes
/// da nação dele (0x539c60, +4 e +8), +0x25 sorteio(16) + 5 (ou, com sorteio(10) zero, sorteio(11) + sorteio(11)), o
/// ano (+0xe) ano do jogo − 27 + sorteio(5) e o dia (+0xc) sorteio(340). Depois zera a linha de estatísticas (17 bytes
/// em [this]) do primeiro da faixa (+4, +8, +0xa, +0xc, +0xf; a original repete a mesma linha `conta` vezes). Devolve
/// 1 (só o al vale).
unsafe extern "thiscall" fn renovar_arbitros(this: usize, comp: usize) -> u8 {
    if comp == 0 {
        return 0;
    }
    let c = cpy8_d(cpy8_d(0xadadfc).wrapping_add(cpy8_i(comp).wrapping_mul(4) as usize));
    if c == 0 || cpy8_b(c + 0x42) == 4 || cpy8_b(c + 0x42) == 2 {
        return 0;
    }
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(cpy8_i(comp)) == 0 {
        return 0;
    }
    let nacao = if cpy8_d(comp + 0x5d) != 0 { cpy8_i(cpy8_d(comp + 0x5d)) } else { sortear(cpy8_i(0xae2350)) };
    let b = cpy8_bloco(this, nacao);
    let mut faixa = None;
    for f in 0..cpy8_w(b + 0x10).max(0) as usize {
        let e = b + 0x12 + f * 0x10;
        if cpy8_i(e + 0xc) == cpy8_i(comp) {
            faixa = Some((cpy8_i(e), cpy8_i(e + 4)));
            break;
        }
    }
    let (ini, n) = faixa.unwrap_or((cpy8_i(b + 8), cpy8_i(b + 4)));
    let nome: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    for k in 0..n.max(0) {
        let ficha = cpy8_d(0xae23e8).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x2b) as usize);
        if ficha == 0 {
            aviso_em(0xa6a22c, 0xa61);
            continue;
        }
        let ano = cpy8_b(0xae2c92);
        let dif = (cpy8_w(ficha + 0xc) as i32).wrapping_sub(cpy8_w(0xae2c90) as i32);
        let idade = if dif > 0 { ano.wrapping_sub(cpy8_b(ficha + 0xe)).wrapping_sub(1) } else { ano.wrapping_sub(cpy8_b(ficha + 0xe)) } as i8;
        if idade <= 0x28 || (idade <= 0x2d && sortear(3) != 0) {
            continue;
        }
        crate::comparacao::ramo("cpy_8 arbitro renovado");
        let nac = cpy8_d(ficha + 0x16);
        let n1 = nome(nac);
        core::ptr::write_unaligned((ficha + 4) as *mut usize, n1);
        let n2 = nome(cpy8_d(ficha + 0x16));
        core::ptr::write_unaligned((ficha + 8) as *mut usize, n2);
        *((ficha + 0x25) as *mut u8) = if sortear(10) != 0 {
            (sortear(0x10) as u8).wrapping_add(5)
        } else {
            let a = sortear(0xb) as u8;
            a.wrapping_add(sortear(0xb) as u8)
        };
        let r5 = sortear(5);
        let base = (cpy8_w(0xae2c92) as u16).wrapping_sub(0x1b);
        core::ptr::write_unaligned((ficha + 0xe) as *mut u16, (r5 as u16).wrapping_add(base));
        core::ptr::write_unaligned((ficha + 0xc) as *mut u16, sortear(0x154) as u16);
    }
    if n > 0 {
        let linha = cpy8_d(this).wrapping_add(ini.wrapping_mul(0x11) as usize);
        *((linha + 0xf) as *mut u8) = 0;
        core::ptr::write_unaligned((linha + 8) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xa) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 4) as *mut u32, 0);
    }
    1
}

/// Comparador: lado a lado pelas fichas de todos os árbitros ([0xae2390] × 0x2b), pelas linhas de estatísticas
/// ([0xae2390] × 17), pelo estado do sorteio e por [0xb67a34], e pela devolução. Sem tabela de árbitros (a caixa a cada
/// ficha): só a original.
unsafe extern "thiscall" fn comparar_renovar_arbitros(this: usize, comp: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1364);
    let total = cpy8_i(0xae2390).max(0) as usize;
    if cpy8_d(0xae23e8) == 0 || cpy8_d(this) == 0 {
        return o(this, comp);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(cpy8_d(0xae23e8), total * 0x2b), (cpy8_d(this), total * 0x11), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, comp),
        || crate::aleatorio::sem_contar(|| renovar_arbitros(this, comp)));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("renovar_arbitros(comp {:#x}) original={} rust={}, diferencas em {:x?}", comp, r, n, dif)
    });
    r
}

// ---------- 261a leva: os árbitros do jogo novo: os árbitros sorteados de uma nação, o conserto das fichas dos árbitros e a cópia de uma ficha (officials_manager.cpp) ----------

unsafe fn cpy9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy9_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpy9_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpy9_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// Um byte sorteado: sorteio(n) + k no al.
unsafe fn cpy9_s(n: i32, k: u8) -> u8 {
    (sortear(n) as u8).wrapping_add(k)
}

/// Uma ficha de árbitro nova em `r` (0x2b bytes) da nação `nacao`, na ordem da 0x79c6aa: primeiro e segundo nome pelo
/// peso (0x539b90, 0x53a0e0), a nação (+0x16), as palavras +0x22 sorteio(80) + 1, +0x1e e +0x20 sorteio(80) + 40 (o
/// byte estendido), os bytes +0x24..+0x26 sorteio(10) + 5 e +0x27..+0x2a sorteio(10) + 7, +0x1a zero, o ano (+0xe)
/// 1960 + sorteio(10) e o dia (+0xc) sorteio(300) + 1. Não grava +0, +0x10..+0x15 (a original deixa o que o realloc
/// trouxe).
unsafe fn cpy9_gerar(r: usize, nacao: usize) {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539b90usize);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a0e0usize);
    cpy9_pd(r + 4, primeiro(nacao));
    cpy9_pd(r + 8, segundo(nacao));
    cpy9_pd(r + 0x16, nacao);
    cpy9_pw(r + 0x22, cpy9_s(0x50, 1) as i8 as i16); // cpy9: a nota +0x22
    cpy9_pw(r + 0x1e, cpy9_s(0x50, 0x28) as i8 as i16);
    cpy9_pw(r + 0x20, cpy9_s(0x50, 0x28) as i8 as i16);
    for o in [0x24usize, 0x25, 0x26] {
        cpy9_pb(r + o, cpy9_s(10, 5));
    }
    cpy9_pb(r + 0x27, cpy9_s(10, 7));
    cpy9_pb(r + 0x28, cpy9_s(10, 7)); // cpy9: o atributo +0x28
    cpy9_pb(r + 0x29, cpy9_s(10, 7));
    cpy9_pb(r + 0x2a, cpy9_s(10, 7));
    cpy9_pd(r + 0x1a, 0);
    cpy9_pw(r + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
    cpy9_pw(r + 0xc, sortear(0x12c).wrapping_add(1) as i16);
}

/// { 0x79c5c0 } (thiscall com o this sem uso, ret 8, officials_manager.cpp): `officials_manager_add_random_officials(
/// conta, nacao)`, pela montagem dos árbitros do jogo novo (0x79bf60, nas nações com poucos árbitros). Nação sem
/// continente (+0x71): nada. Realoca a tabela ([0xae23e8], 0x945667) para [0xae2390] + conta fichas (sem memória: erro
/// e o jogo para), gera `conta` fichas novas no fim (cpy9_gerar), soma a conta em [0xae2390] e ordena a tabela inteira
/// (qsort 0x945333 com 0x79d480: nação, +0x28 × 50 + +0x22, id).
unsafe extern "thiscall" fn arbitros_sorteados(_this: usize, conta: i32, nacao: usize) {
    if nacao != 0 && cpy9_d(nacao + 0x71) == 0 {
        return;
    }
    let realocar: unsafe extern "C" fn(usize, i32) -> usize = core::mem::transmute(0x945667usize);
    let antes = cpy9_i(0xae2390);
    let t = realocar(cpy9_d(0xae23e8), antes.wrapping_add(conta).wrapping_mul(0x2b));
    *(0xae23e8 as *mut usize) = t;
    if t == 0 {
        erro_em(0xa6a22c, 0x342);
    }
    let mut r = t.wrapping_add(antes.wrapping_mul(0x2b) as usize);
    for _ in 0..conta.max(0) {
        crate::comparacao::ramo("cpy_9 arbitro novo");
        cpy9_gerar(r, nacao);
        r += 0x2b;
    }
    let n = cpy9_i(0xae2390).wrapping_add(conta);
    *(0xae2390 as *mut i32) = n;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparacao::ramo("cpy_9 ordenar");
    ordenar(cpy9_d(0xae23e8), n, 0x2b, 0x79d480); // cpy9: ordenar a tabela
}

/// A ficha `g` (gerada no ensaio) bate com a `f` da tabela nos campos que a 0x79c5c0 grava (+4..+0x10, +0x16..+0x2b)?
unsafe fn cpy9_mesma_ficha(g: &[u8; 0x2b], f: usize) -> bool {
    let t = core::slice::from_raw_parts(f as *const u8, 0x2b);
    g[4..0x10] == t[4..0x10] && g[0x16..] == t[0x16..]
}

/// Comparador: realoca e ordena (não pode rodar duas vezes): só a original, depois de um ensaio. O ensaio gera as
/// `conta` fichas da cópia (cpy9_gerar, sem contar os sorteios) numa área própria a partir do mesmo estado do sorteio,
/// que é reposto; depois roda a original. Confere o estado final do sorteio, a conta nova ([0xae2390] + conta) e que
/// cada ficha do ensaio esteja na tabela ordenada (campos que a original grava). Conta não positiva ou nação sem
/// continente: só a original.
unsafe extern "thiscall" fn comparar_arbitros_sorteados(this: usize, conta: i32, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize)>(1365);
    if conta <= 0 || (nacao != 0 && cpy9_d(nacao + 0x71) == 0) {
        return o(this, conta, nacao);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let mut novas = vec![[0u8; 0x2b]; conta as usize];
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        for g in novas.iter_mut() {
            cpy9_gerar(g.as_mut_ptr() as usize, nacao);
        }
    }));
    let estado_rust = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    let antes = cpy9_i(0xae2390);
    o(this, conta, nacao);
    let estado_orig = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let total = cpy9_i(0xae2390);
    let t = cpy9_d(0xae23e8);
    let mut usada = vec![false; total.max(0) as usize];
    let mut faltam = Vec::new();
    for (k, g) in novas.iter().enumerate() {
        match (0..usada.len()).find(|&j| !usada[j] && cpy9_mesma_ficha(g, t + j * 0x2b)) {
            Some(j) => usada[j] = true,
            None => faltam.push(k),
        }
    }
    let igual = estado_orig == estado_rust && total == antes.wrapping_add(conta) && faltam.is_empty();
    PLACAR.registrar(igual, || format!("arbitros_sorteados(conta {}, nacao {:#x}) total {} (antes {}), sorteio igual {}, fichas da copia ausentes {:?}",
        conta, nacao, total, antes, estado_orig == estado_rust, faltam));
}

/// O ajuste de um atributo em byte (+0x24..+0x2a) fora de 1..20: o valor sorteado por `f` (lê a ficha antes).
unsafe fn cpy9_atributo(e: usize, o: usize, f: impl FnOnce() -> u8) {
    let v = cpy9_b(e + o) as i8;
    if v <= 0 || v > 0x14 {
        cpy9_pb(e + o, f());
    }
}

/// { 0x79caa0 } (cdecl, officials_manager.cpp): `officials_manager_validate_officials()`, no começo da montagem dos
/// árbitros do jogo novo (0x79bf60). Para cada uma das [0xae2390] fichas (0x2b bytes em [0xae23e8]): sem nação (+0x16),
/// a da tabela das nações pelo índice (+0x37) do primeiro nome (+4), do segundo (+8) ou sorteado; as palavras +0x22,
/// +0x1e e +0x20 fora de 1..200: sorteio(80) + 1 mais o byte +0x8e da nação (+0x22) ou 4 vezes ele (+0x1e, +0x20; a
/// +0x20 refeita fica no mínimo na +0x1e); os bytes +0x24..+0x2a fora de 1..20 sorteados pelas faixas de +0x1e (acima de
/// 120, de 80); nascido antes da data 0x549f70(1, 1, 1910, −1) (ano sem sinal, depois o dia com sinal), ano 1960 +
/// sorteio(10) e dia sorteio(300) + 1.
unsafe extern "C" fn validar_arbitros() {
    let base = cpy9_d(0xae23e8);
    let mut i = 0i32;
    while i < cpy9_i(0xae2390) {
        let e = base.wrapping_add((i as usize).wrapping_mul(0x2b));
        if cpy9_d(e + 0x16) == 0 {
            let (n1, n2) = (cpy9_d(e + 4), cpy9_d(e + 8));
            let k = if cpy9_i(n1 + 0x37) >= 0 {
                cpy9_i(n1 + 0x37)
            } else if cpy9_i(n2 + 0x37) >= 0 {
                cpy9_i(n2 + 0x37)
            } else {
                sortear(cpy9_i(0xae2350))
            };
            cpy9_pd(e + 0x16, cpy9_d(0xae23a8).wrapping_add(k.wrapping_mul(0x122) as usize));
        }
        let fora = |o: usize| {
            let v = cpy9_w(e + o);
            v <= 0 || v > 0xc8
        };
        if fora(0x22) {
            let nac = cpy9_d(e + 0x16);
            let v = if nac != 0 {
                let s = sortear(0x50) as u8;
                s.wrapping_add(cpy9_b(cpy9_d(e + 0x16) + 0x8e)).wrapping_add(1)
            } else {
                (sortear(0x50) as u8).wrapping_add(1)
            };
            cpy9_pw(e + 0x22, v as i8 as i16);
        }
        for o in [0x1eusize, 0x20] {
            if fora(o) {
                crate::comparacao::ramo("cpy_9 nota consertada");
                let nac = cpy9_d(e + 0x16);
                let v = if nac != 0 {
                    let s = sortear(0x50) as u8;
                    s.wrapping_add(cpy9_b(cpy9_d(e + 0x16) + 0x8e) << 2).wrapping_add(1)
                } else {
                    (sortear(0x50) as u8).wrapping_add(1)
                };
                cpy9_pw(e + o, v as i8 as i16);
                if o == 0x20 && cpy9_w(e + 0x20) < cpy9_w(e + 0x1e) {
                    cpy9_pw(e + 0x20, cpy9_w(e + 0x1e));
                }
            }
        }
        let faixa = |alto: (i32, u8), medio: (i32, u8), baixo: (i32, u8)| {
            let w = cpy9_w(e + 0x1e);
            let (n, k) = if w > 0x78 { alto } else if w > 0x50 { medio } else { baixo };
            cpy9_s(n, k)
        };
        cpy9_atributo(e, 0x24, || if cpy9_w(e + 0x1e) > 0x64 { cpy9_s(0xc, 7) } else { cpy9_s(0xa, 5) });
        cpy9_atributo(e, 0x25, || faixa((0xe, 7), (0xf, 5), (0xa, 5)));
        cpy9_atributo(e, 0x26, || faixa((0xb, 0xa), (0xf, 5), (0xa, 5)));
        cpy9_atributo(e, 0x27, || if cpy9_w(e + 0x1e) > 0x50 { cpy9_s(0xf, 6) } else { cpy9_s(0x14, 1) });
        cpy9_atributo(e, 0x28, || faixa((0xa, 0xb), (0xf, 5), (0xa, 7)));
        cpy9_atributo(e, 0x29, || faixa((0xd, 8), (0xe, 7), (0xf, 6)));
        cpy9_atributo(e, 0x2a, || if sortear(0x64) != 0 { cpy9_s(0xa, 0xb) } else { cpy9_s(0xa, 5) });
        let mut limite = [0u8; 8];
        let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> *const u8 = core::mem::transmute(0x549f70usize);
        let d = data(limite.as_mut_ptr(), 1, 1, 0x776, -1);
        let (dia, ano) = (cpy9_w(d as usize), cpy9_w(d as usize + 2) as u16);
        let ano_e = cpy9_w(e + 0xe) as u16;
        if ano > ano_e || (ano == ano_e && dia > cpy9_w(e + 0xc)) {
            cpy9_pw(e + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
            cpy9_pw(e + 0xc, sortear(0x12c).wrapping_add(1) as i16);
        }
        i += 1;
    }
}

/// Comparador: lado a lado pelas fichas de todos os árbitros e pelo estado do sorteio.
unsafe extern "C" fn comparar_validar_arbitros() {
    let o = orig::<unsafe extern "C" fn()>(1366);
    let total = cpy9_i(0xae2390).max(0) as usize;
    if cpy9_d(0xae23e8) == 0 {
        return o();
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((cpy9_d(0xae23e8), total * 0x2b));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| validar_arbitros()));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("validar_arbitros ({} fichas) diferencas em {:x?}", total, dif)
    });
}

/// { 0x79c530 } (thiscall, ret 4, officials_manager.cpp): `Official::operator=(src)`, pela montagem dos árbitros do
/// jogo novo (0x79bf60, as fichas que ficam). Copia a ficha de 0x2b bytes campo a campo (+0, +4, +8; a data de +0xc
/// pela 0x417c10; +0x14, +0x16, +0x1a, +0x1e, +0x20, +0x22, os bytes +0x24..+0x2a). Devolve `this`.
unsafe extern "thiscall" fn copiar_ficha_de_arbitro(this: usize, src: usize) -> usize {
    for o in [0usize, 4, 8] {
        cpy9_pd(this + o, cpy9_d(src + o));
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(this + 0xc, t.as_mut_ptr(), src + 0xc);
    cpy9_pw(this + 0x14, cpy9_w(src + 0x14));
    cpy9_pd(this + 0x16, cpy9_d(src + 0x16));
    cpy9_pd(this + 0x1a, cpy9_d(src + 0x1a));
    for o in [0x1eusize, 0x20, 0x22] {
        cpy9_pw(this + o, cpy9_w(src + o));
    }
    for o in 0x24usize..0x2b {
        cpy9_pb(this + o, cpy9_b(src + o));
    }
    this
}

/// Comparador: lado a lado pela ficha de destino e pela devolução.
unsafe extern "thiscall" fn comparar_copiar_ficha_de_arbitro(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1367);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x2b)], || o(this, src), || copiar_ficha_de_arbitro(this, src));
    PLACAR.registrar(igual && r == n, || format!("copiar_ficha_de_arbitro(src {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", src, r, mo, n, mr));
    r
}

// ---------- 261a leva: o prêmio do ano no histórico (o técnico conta, o jogador premiado agradece ou renova) e os recordes de vitória e de derrota (staff_records.cpp, record_utils.cpp) ----------

unsafe fn cpy10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O corpo da 0x86c390; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
unsafe fn cpy10_premio(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32, gravar: bool) {
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x34;
    cpq4_por(&mut e, 8, if clube != 0 { cpy10_d(clube) as i32 } else { -1 });
    cpq4_por(&mut e, 0xd, cpy10_d(premio) as i32);
    cpq4_por(&mut e, 0x11, (a4 & 0xffff) as i32);
    cpq4_por(&mut e, 0x15, a5 as u8 as i8 as i32);
    cpq4_por(&mut e, 0x19, credito as u8 as i8 as i32);
    cpq4_historico(gravar, this, cpy10_d(p), &e);
    let (t3d, t28) = (cpy10_b(p + 0x3d), cpy10_b(p + 0x28));
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = cpq4_registro(this, p);
        if r != 0 {
            crate::comparacao::ramo("cpy_10 prêmio do técnico");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
        }
        return;
    }
    if cpy10_d(p + 0x61) == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpy10_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 || cpy10_b(c + 0x35) & 0x3f == 5 {
        return;
    }
    if cpy10_d(0xae23b4).wrapping_add(cpy10_d(c + 4).wrapping_mul(0x245)) == 0 {
        return;
    }
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = do_clube(cpy10_d(0xae1050), p, cpv13_clube_do_contrato(p));
    if k == 0 {
        return;
    }
    if cpy10_d(0xde1f54) != 0 {
        let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, cpy10_d(k + 0x25), cpy10_d(k + 0x29)) > 0x12c { // cpy10: mais de 300 dias
            let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
            let dono = cpv13_clube_do_contrato(p);
            let x = entrada(cpy10_d(cpy10_d(0xde1f54) + 0x856), dono, p, 0, 0);
            if x != 0 && cpy10_d(x + 0xf) == cpy10_d(p) {
                crate::comparacao::ramo("cpy_10 renovação");
                cpq3_renovar(p);
            }
        }
    }
    if credito as u8 != 1 {
        crate::comparacao::ramo("cpy_10 sem crédito");
        return; // cpy10: só o creditado
    }
    if cpy10_d(k + 0x45) != 0 {
        return;
    }
    let v59 = cpy10_b(p + 0x59) as i8;
    if v59 <= 0xf { // cpy10: o +0x59
        return;
    }
    if cpy10_b(p + 0x5b) as i8 <= 0xa {
        return;
    }
    crate::comparacao::ramo("cpy_10 sorteio do agradecimento");
    if sortear(v59 as i32) <= 5 {
        return;
    }
    let agradece: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x58a070usize);
    agradece(p, clube, premio);
}

/// { 0x86c390 } (thiscall, ret 0x18, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_year_award_record(
/// p, clube, premio, a4, a5, credito)`, pelo registro dos vencedores do prêmio do ano (YEAR_AWARD::record_winners
/// 0x9302f0: o primeiro, o segundo e o terceiro). A entrada 0x34 do histórico da pessoa (data do jogo, clube ou −1, o id
/// do prêmio, a4 na palavra, a5 e o crédito em bytes com sinal; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na
/// palavra +0x9e do registro dele (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o
/// clube (0x4cf5f0): com o gerente de transferências, contrato de mais de 300 dias e o jogador na lista do clube
/// (0x849ce0), a renovação (0x4cda70); depois, só com o crédito 1, sem a marca +0x45 do contrato, com +0x59 acima de 15
/// e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do agradecimento ao técnico (0x58a070). Sem pessoa ou sem
/// prêmio: a original (caixa "Error").
unsafe extern "thiscall" fn registrar_premio_do_ano(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32) {
    if p == 0 || premio == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(1368)(this, p, clube, premio, a4, a5, credito);
    }
    cpy10_premio(this, p, clube, premio, a4, a5, credito, true)
}

/// Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
unsafe extern "thiscall" fn comparar_registrar_premio_do_ano(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, credito: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(1368);
    if p == 0 || premio == 0 {
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let (t3d, t28) = (cpy10_b(p + 0x3d), cpy10_b(p + 0x28));
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparacao::ramo("registrar_premio_do_ano jogador");
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let r = cpq4_registro(this, p);
    if r == 0 {
        return o(this, p, clube, premio, a4, a5, credito);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, p, clube, premio, a4, a5, credito),
        || cpy10_premio(this, p, clube, premio, a4, a5, credito, false));
    PLACAR.registrar(igual, || format!("registrar_premio_do_ano({:x}, prêmio {:x}) {:x?} x {:x?}", p, cpy10_d(premio), mo, mr));
}

/// { 0x7e4a30 } (cdecl, record_utils.cpp): `match_record_is_bigger_win(a, b)`, pelos recordes do clube depois de cada
/// jogo (0x444930) e pelos da competição (0x49fb93): 1 se `a` é vitória (gols +0x19 > +0x1a, bytes com sinal) por
/// diferença maior que a de `b`, ou igual com mais gols feitos. Registro nulo: a original (caixa "Error", 0).
unsafe extern "C" fn vitoria_maior(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return orig::<unsafe extern "C" fn(usize, usize) -> u32>(1369)(a, b);
    }
    let (fa, sa, fb, sb) = (cpy10_b(a + 0x19) as i8, cpy10_b(a + 0x1a) as i8, cpy10_b(b + 0x19) as i8, cpy10_b(b + 0x1a) as i8);
    if fa <= sa {
        return 0;
    }
    let (da, db) = (fa as i32 - sa as i32, fb as i32 - sb as i32);
    (da > db || (da == db && fa > fb)) as u32
}

/// Comparador: pura; registro nulo (a caixa), só a original.
unsafe extern "C" fn comparar_vitoria_maior(a: usize, b: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(1369)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = vitoria_maior(a, b);
    PLACAR.registrar(r == n, || format!("vitoria_maior({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x7e4b00 } (cdecl, record_utils.cpp): `match_record_is_bigger_defeat(a, b)`, pelos recordes do clube depois de cada
/// jogo (0x444930): 1 se `a` é derrota (+0x19 < +0x1a) por diferença maior que a de `b`, ou igual com mais gols
/// sofridos. Registro nulo: a original (caixa "Error", 0).
unsafe extern "C" fn derrota_maior(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return orig::<unsafe extern "C" fn(usize, usize) -> u32>(1370)(a, b);
    }
    let (fa, sa, fb, sb) = (cpy10_b(a + 0x19) as i8, cpy10_b(a + 0x1a) as i8, cpy10_b(b + 0x19) as i8, cpy10_b(b + 0x1a) as i8);
    if fa >= sa {
        return 0;
    }
    let (da, db) = (sa as i32 - fa as i32, sb as i32 - fb as i32);
    (da > db || (da == db && sa > sb)) as u32
}

/// Comparador: pura; registro nulo (a caixa), só a original.
unsafe extern "C" fn comparar_derrota_maior(a: usize, b: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(1370)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = derrota_maior(a, b);
    PLACAR.registrar(r == n, || format!("derrota_maior({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 261a leva: os jogos e as faltas das permissões de trabalho e os prêmios por jogos depois da partida, o prêmio por jogos na seleção guardado e a notícia da transferência finalizada (transfer_manager.cpp) ----------

unsafe fn cpy11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy11_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy11_clube(id: usize) -> usize {
    cpy11_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpy11_pessoa(id: usize) -> usize {
    cpy11_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O byte das marcas da pessoa no registro de 8 bytes da gerência de transferências ([this+0x86e] + id × 8).
unsafe fn cpy11_marca(this: usize, p: usize) -> usize {
    cpy11_d(this + 0x86e).wrapping_add(cpy11_d(p).wrapping_mul(8))
}

/// A permissão de trabalho da pessoa (0xe bytes, [this+0x93a], conta +0x93e), como a original procura em linha: id fora
/// dos [0xae2364] (com sinal) ou sem a marca 0x20, 0; senão a busca binária (bsearch 0x946464 com 0x8e55c0).
unsafe fn cpy11_permissao(this: usize, p: usize) -> usize {
    if p != 0 && cpy11_d(p) as i32 >= cpy11_d(0xae2364) as i32 {
        return 0;
    }
    if cpy11_b(cpy11_marca(this, p)) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpy11_d(p) as u32];
    bsearch(chave.as_ptr(), cpy11_d(this + 0x93a), cpy11_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// Soma 1 à palavra `o` da permissão (+4 jogos, +6 faltas). A original soma sem conferir a busca (a marca garante a
/// permissão); sem ela, a original cairia: aqui nada.
unsafe fn cpy11_contar(r: usize, o: usize) {
    if r != 0 {
        let a = (r + o) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
    }
}

/// O ensaio do comparador: a cópia roda sem pagar os prêmios por jogos (0x8cb340 dá notícia, aloca e paga), como se
/// nenhum pagasse (sem tirar o bônus); a contagem das permissões não depende deles.
static mut CPY11_ENSAIO: bool = false;

/// TRANSFER_MANAGER::pay_appearance_money (0x8cb340: thiscall(this, pessoa, jogos, tipo, 0), ret 0x10); no ensaio, 0.
unsafe fn cpy11_pagar(this: usize, p: usize, jogos: i32, tipo: u32) -> u32 {
    if CPY11_ENSAIO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32 = core::mem::transmute(0x8cb340usize);
    f(this, p, jogos, tipo, 0)
}

/// { 0x8c39f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::update_appearance_clauses_after_match(jogo,
/// partida)`, pelo fim de cada partida (0x6b5f00 e as estatísticas simuladas 0x84be20). Para os dois lados (+0x1c casa,
/// +0x20 fora): o jogo conta para as permissões de trabalho com competição (+0x14) que não é de seleção (casa pela
/// 0x53b900) nem amistoso (0x5c6710) e o byte do lado (+0x40/+0x41) ligado, ou com +0 = −2. Clube que não é seleção: o
/// elenco (0x541f90, 50 vagas; seleção: o vetor fica como estava) e, contando, os jogadores dele com a marca 0x20 (o
/// registro de 8 bytes em +0x86e). Nas 20 vagas da partida (+0x190 o id, +0x1ad ≠ −5, reservas com +0x1c2 ≠ −1; sem
/// ficha de jogador +0x61, fim das vagas): fora do empréstimo (0x4cf6d0, só no clube) e com a marca da cláusula (2
/// clube, 4 seleção), os jogos (seleção: +0x22 menos o byte +7 do registro; clube: +0x3b do contrato 0x4cf5a0) múltiplos
/// de 10 até 50, ou 1, pagam o prêmio (0x8cb340 com o tipo 5 ou 2) e, pago, o bônus sai (0x8cb630 ou 0x8cb590). Contando,
/// o jogador marcado soma um jogo (+4 da permissão) e sai da lista; os que sobram, sem lesão (0x62c400), suspensão
/// (0x55d270), convocação (0x7757d0) nem inelegibilidade (0x8cf5e0), e não emprestados por outro clube (0x4cf6d0 com o
/// contrato guardado em [0xae1080]), somam uma falta (+6).
unsafe extern "thiscall" fn jogos_das_permissoes_e_clausulas(this: usize, jogo: usize, partida: usize) {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    let tirar_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x7757d0usize);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    // ponytail: o elenco local da original (esp+0x110) não é preenchido no lado da seleção e fica com o que a pilha tinha
    // (o do outro lado ou lixo); aqui começa zerado (vaga vazia, que o jogo também produz) e passa de um lado ao outro
    let mut elenco = [0usize; 50];
    let mut lado: i8 = 0;
    while lado < 2 {
        let clube = cpy11_d(jogo + if lado == 0 { 0x1c } else { 0x20 });
        let comp = cpy11_d(jogo + 0x14);
        let fora = comp == 0 || e_selecao(cpy11_d(jogo + 0x1c)) != 0 || amistoso(cpy11_d(0xae2c74), comp) != 0;
        let contando = (!fora && cpy11_b(jogo + 0x40 + lado as usize) != 0) || cpy11_d(jogo) as i32 == -2;
        let mascara: u8 = if e_selecao(clube) != 0 {
            4
        } else {
            reunir(clube, elenco.as_mut_ptr(), 0);
            2
        };
        let mut seguidos = [0usize; 50];
        let mut n: i8 = 0;
        if contando {
            for &p in elenco.iter() {
                if p != 0 && (cpy11_d(p) as i32) < cpy11_d(0xae2364) as i32 && cpy11_b(cpy11_marca(this, p)) & 0x20 != 0 {
                    seguidos[n as usize] = p;
                    n += 1;
                }
            }
        }
        let base = partida + lado as usize * 0x625;
        let mut k: i8 = 0;
        while k < 0x14 {
            let v = base + k as usize * 0x49;
            let id = cpy11_d(v + 0x190) as i32;
            if id != -1 && cpy11_b(v + 0x1ad) != 0xfb && (k < 0xb || cpy11_b(v + 0x1c2) != 0xff) {
                let p = cpy11_pessoa(id as usize);
                if cpy11_d(p + 0x61) == 0 {
                    break;
                }
                if (p == 0 || (cpy11_d(p) as i32) < cpy11_d(0xae2364) as i32) && !(mascara == 2 && ficha(cpy11_d(0xae1050), p) != 0) {
                    let reg = cpy11_marca(this, p);
                    if cpy11_b(reg) & mascara != 0 {
                        let (jogos, tipo): (i16, u8) = if mascara == 4 {
                            ((cpy11_b(p + 0x22) as u16).wrapping_sub(cpy11_b(reg + 7) as u16) as i16, 5)
                        } else {
                            let mut j = 0i16;
                            if contrato(cpy11_d(0xae1050), p) != 0 {
                                j = cpy11_w(contrato(cpy11_d(0xae1050), p) + 0x3b);
                            }
                            (j, 2)
                        };
                        // ponytail: a original empurra o dword do local do tipo, com os 3 bytes altos de lixo da pilha;
                        // a 0x8cb340 e a notícia só leem o byte: vai limpo
                        if (jogos as i32 % 10 == 0 && jogos <= 0x32) || jogos == 1 { // cpy11: os jogos do prêmio
                            if cpy11_pagar(this, p, jogos as i32, tipo as u32) != 0 {
                                crate::comparacao::ramo("cpy_11 premio pago");
                                if tipo == 5 {
                                    tirar_selecao(this, p);
                                } else {
                                    tirar_time(this, p);
                                }
                            }
                        }
                    }
                    if contando && cpy11_b(cpy11_marca(this, p)) & 0x20 != 0 {
                        let r = cpy11_permissao(this, p);
                        crate::comparacao::ramo("cpy_11 jogo contado");
                        cpy11_contar(r, 4); // cpy11: o jogo conta na permissão
                        let mut i: i8 = 0;
                        while i < n {
                            if seguidos[i as usize] == p {
                                seguidos[i as usize] = 0;
                                break;
                            }
                            i += 1;
                        }
                    }
                }
            }
            k += 1;
        }
        if contando && n > 0 {
            for i in 0..n as usize {
                let p = seguidos[i];
                if p == 0 || lesao(cpy11_d(0xb63d28), p) != 0 {
                    continue;
                }
                let comp = cpy11_d(jogo + 0x14);
                if comp != 0 && suspenso(cpy11_d(0xae28f0), p, comp, 1, 0) != 0 {
                    continue;
                }
                if convocado(cpy11_d(0xb74318), p, 0xae2c90, clube, 0, 0) != 0 {
                    continue;
                }
                if cpy11_d(jogo + 0x14) != 0 && inelegivel(this, p, jogo) != 0 {
                    continue;
                }
                if ficha(cpy11_d(0xae1050), p) != 0 {
                    let c = contrato(cpy11_d(0xae1050), p);
                    *(0xae1080 as *mut usize) = c;
                    let dono = if c == 0 { 0 } else { cpy11_clube(cpy11_d(c + 4)) };
                    if dono == clube {
                        continue;
                    }
                }
                let r = cpy11_permissao(this, p);
                crate::comparacao::ramo("cpy_11 falta contada");
                cpy11_contar(r, 6); // cpy11: a falta conta na permissão
            }
        }
        lado += 1;
    }
}

/// Comparador: a cópia em ensaio (sem pagar os prêmios) e a original a partir do mesmo estado do vetor das permissões
/// de trabalho; os jogos (+4) e as faltas (+6) têm de bater. Seleção em casa: o elenco local da original começa com o
/// lixo da pilha, só a original roda. Os prêmios se provam pela própria 0x8cb340 (comparada) e pela carreira.
unsafe extern "thiscall" fn comparar_jogos_das_permissoes_e_clausulas(this: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1371);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let v = cpy11_d(this + 0x93a);
    let n = cpy11_d(this + 0x93e) as i32;
    if e_selecao(cpy11_d(jogo + 0x1c)) != 0 || v == 0 || n <= 0 {
        return o(this, jogo, partida); // ponytail: só a original (lixo da pilha ou sem permissões)
    }
    let regioes = [(v, n as usize * 0xe)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPY11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| jogos_das_permissoes_e_clausulas(this, jogo, partida)));
    CPY11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, jogo, partida);
    // a original não realoca o vetor (os prêmios mexem em outros vetores)
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("jogos_das_permissoes_e_clausulas(jogo {:x}) permissões diferentes nos bytes {:x?}", jogo, dif)
    });
}

/// { 0x8cad90 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_international_team_appearance_money(
/// registro)`: acrescenta o prêmio por jogos na seleção (13 bytes copiados de `registro`) ao vetor [this+0x898]
/// (capacidade +0x89c, conta +0x8a0); cheio, cresce de 50 em 50 (sem memória: caixa 0x25af e nada). Reordena pela pessoa
/// (qsort 0x945333 com 0x8b4e00). A gêmea do time principal é a 0x8cac40.
unsafe extern "thiscall" fn guardar_premio_por_jogos_na_selecao(this: usize, registro: usize) {
    crate::comparacao::ramo("cpy_11 premio na selecao guardado");
    if !cpl3_crescer(this, 0x898, 4, 50, 0xd, 0x25af, 0xa8ebe0) {
        return;
    }
    let n = cpy11_d(this + 0x8a0);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpy11_d(this + 0x898).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    core::ptr::write_unaligned((this + 0x8a0) as *mut usize, n.wrapping_add(1));
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpy11_d(this + 0x898), cpy11_d(this + 0x8a0) as u32, 0xd, 0x8b4e00);
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta e os 13
/// bytes), a original roda; confere a conta e que o registro está no vetor (reordenado).
unsafe extern "thiscall" fn comparar_guardar_premio_por_jogos_na_selecao(this: usize, registro: usize) {
    let n = cpy11_d(this + 0x8a0);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1372)(this, registro);
    if cpy11_d(this + 0x898) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpy11_d(this + 0x8a0);
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpy11_d(this + 0x898), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("guardar_premio_por_jogos_na_selecao: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// { 0x8da5e0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da data e da notícia locais): `TRANSFER_MANAGER::
/// post_transfer_finalised_news(oferta)`, pela transferência acertada (0x8c2f50). O vendedor: o clube do contrato da
/// pessoa (0x4cf5a0, guardado em [0xae1080]) fora do tipo 5 (pedido de novo, sem guardar). A notícia 0xfa5: a pessoa (o
/// id e as três chaves), o comprador, o vendedor (ou −1), +0x20, +0x24, o valor +0x1b, o estado +0x2c, o total
/// (0x8e7a70 com 0, 1, 0, 1), +0x2f, +0x11; a data de conclusão (0x8cee70) depois de hoje vai nos campos 0xe (dia), 0xf
/// (ano) e 0xd (o nibble baixo de +0x30, com sinal), senão 0, 0, 0; a conta da troca (+4) no 0xb. Ao vendedor humano
/// (0x7875e0), fora do estado 0xc e sem troca pendente (+0x2f com nibble +0x30), e ao comprador humano, cada um no
/// campo 0x10.
unsafe extern "thiscall" fn noticia_transferencia_finalizada(this: usize, o: usize) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u16 = core::mem::transmute(0x8cee70usize);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpy11_pessoa(cpy11_d(o + 0xc));
    let c = contrato(cpy11_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let mut vendedor = if c == 0 || cpy11_b(c + 0x35) & 0x3f == 5 { 0 } else { cpy11_clube(cpy11_d(c + 4)) };
    let comprador = cpy11_clube(cpy11_d(o + 8));
    let mut data = [0u8; 8];
    data_padrao(data.as_mut_ptr());
    if vendedor != 0 && cpy11_b(contrato(cpy11_d(0xae1050), p) + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    crate::comparacao::ramo("cpy_11 noticia da transferencia finalizada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa5, 0);
    gravar(nt, 0, cpy11_d(p) as u32);
    gravar(nt, 1, cpy11_d(cpy11_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpy11_d(cpy11_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpy11_d(cpy11_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpy11_d(o + 8) as u32);
    gravar(nt, 5, if vendedor != 0 { cpy11_d(vendedor) as u32 } else { u32::MAX });
    gravar(nt, 6, cpy11_d(o + 0x20) as u32);
    gravar(nt, 7, cpy11_d(o + 0x24) as u32);
    gravar(nt, 8, cpy11_d(o + 0x1b) as u32);
    gravar(nt, 9, cpy11_b(o + 0x2c) as i8 as u32);
    gravar(nt, 0xa, total(o, 0, 1, 0, 1));
    gravar(nt, 0xc, cpy11_b(o + 0x2f) as i8 as u32);
    gravar(nt, 0x11, cpy11_b(o + 0x11) as i8 as u32);
    conclusao(this, o, data.as_mut_ptr());
    let (dia, ano) = (i16::from_le_bytes([data[0], data[1]]), u16::from_le_bytes([data[2], data[3]]));
    let (hoje_dia, hoje_ano) = (cpy11_w(0xae2c90), cpy11_w(0xae2c92) as u16);
    if hoje_ano < ano || (hoje_ano == ano && hoje_dia < dia) {
        gravar(nt, 0xe, dia as i32 as u32);
        gravar(nt, 0xf, ano as u32);
        gravar(nt, 0xd, (((cpy11_b(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    } else {
        gravar(nt, 0xe, 0);
        gravar(nt, 0xf, 0);
        gravar(nt, 0xd, 0);
    }
    let troca = cpy11_d(o + 4);
    gravar(nt, 0xb, if troca != 0 { cpy11_b(troca) as i8 as u32 } else { 0 });
    let jornal = cpy11_d(0xb7431c);
    if vendedor != 0 && humano(jornal, vendedor) != 0 && cpy11_b(o + 0x2c) != 0xc && (cpy11_b(o + 0x2f) == 0 || cpy11_b(o + 0x30) & 0xf == 0) {
        gravar(nt, 0x10, cpy11_d(vendedor) as u32);
        entregar(cpy11_d(0xb7431c), nt, vendedor);
    }
    if humano(cpy11_d(0xb7431c), comprador) != 0 {
        gravar(nt, 0x10, cpy11_d(comprador) as u32);
        entregar(cpy11_d(0xb7431c), nt, comprador);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_transferencia_finalizada(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1373)(this, o)
}

// ---------- 261a leva: o veto dos fisioterapeutas e da diretoria à transferência, a notícia do ultimato da diretoria e o texto dela (manager_manager.cpp) ----------

unsafe fn cpy12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy12_clube(id: usize) -> usize {
    cpy12_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O registro da diretoria do clube na gerência dos técnicos ([this] + id × 0x49).
unsafe fn cpy12_diretoria(this: usize, clube: usize) -> usize {
    cpy12_d(this).wrapping_add(cpy12_d(clube).wrapping_mul(0x49))
}

/// Soma `delta` à palavra `o` (+6, +0x14, +0x16: as confianças) da diretoria do clube.
unsafe fn cpy12_confianca(this: usize, clube: usize, o: usize, delta: i16) {
    let a = (cpy12_diretoria(this, clube) + o) as *mut i16;
    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(delta));
}

/// O piso da confiança +6 em 1 e o veto 0x800.
unsafe fn cpy12_piso(this: usize, clube: usize) -> u32 {
    let a = (cpy12_diretoria(this, clube) + 6) as *mut i16;
    if core::ptr::read_unaligned(a) < 1 {
        core::ptr::write_unaligned(a, 1);
    }
    0x800
}

/// O valor do jogador (CONTRACT_MANAGER::calc_player_value 0x4d0c60 em [0xae1050] com 0, −1 e `modo`).
unsafe fn cpy12_valor(j: usize, clube: usize, modo: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    f(cpy12_d(0xae1050), j, clube, 0, -1, modo)
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]) ou 0.
unsafe fn cpy12_clube_do_contrato(j: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy12_d(0xae1050), j);
    *(0xae1080 as *mut usize) = c;
    if c == 0 { 0 } else { cpy12_clube(cpy12_d(c + 4)) }
}

/// Se a cópia semeou o rand da CRT pelo relógio (os fisioterapeutas avaliaram): o comparador não confere o estado do
/// rand (o relógio anda entre a original e a cópia no jogo real).
static mut CPY12_SEMEOU: bool = false;

/// { 0x696b10 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::check_board_transfer_veto(jogador, clube,
/// valor, comprando)`, pela 0x8cdb80 (oferta sem troca). Comprando um jogador com lesão (0x62ed70) de gravidade +0x28 > 20,
/// cada um dos três fisioterapeutas do clube (+0x1cf, com ficha +0x69) semeia o rand da CRT pelos dois ids e sorteia:
/// aposentadoria anunciada pela lesão (0x7a7ed0) contra a nota dele (0x544410 do byte +0x1e), a gravidade contra
/// sorteio(200) + 25 e (41 − nota) × 5, a recaída (0x62ece0), a nota³ + 100 (pow 0x945ff0) e a nota contra a do jogador
/// (+0x61 +0x2d): qualquer um veta (0x2000); no fim o rand volta ao relógio (0x946ebb). Depois, só com contrato fora dos
/// tipos 1 e 3, clube atual (+0x39) e clube humano (0x600a50): o limite (reputação +0x80 × 0,2)² entre 150 mil e 2
/// milhões. Vendendo: liga com divisão e elenco abaixo de 15, a diretoria perde 1250/1250/500 (+6/+0x14/+0x16, piso 1 em
/// +6) e veta 0x200; senão o pedido (0x4d1600, no máximo o valor pelo clube do contrato) acima do limite e o valor pelo
/// clube atual não abaixo do `valor`: pedido > 2 × valor com +0x3d do contrato > 25, ou pedido > 5 milhões com +0x3d > 0 e
/// > 2,5 × valor, perdem 1250/1250/500; > 6 × valor, 500/500/250; veto 0x800. Comprando: valor acima de 1,2 × a verba
/// (0x598800) veta 0x800; acima do limite, o menor valor (pelo clube e pelo do contrato) + 0,65 × limite + a faixa de
/// riqueza (0x5986b0: 1,25 milhão, 650 mil ou 65 mil) truncado abaixo de 0,275 × valor perde 250/500/500 e veta 0x800.
/// Senão 0.
unsafe extern "thiscall" fn veto_dos_fisios_e_da_diretoria(this: usize, j: usize, clube: usize, valor: i32, comprando: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(0x9469fausize);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(0x946ebbusize);
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    let aposentadoria: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7a7ed0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let recaida: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ece0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    CPY12_SEMEOU = false;
    if comprando as u8 != 0 {
        let l = lesao(cpy12_d(0xb63d28), j);
        if l != 0 && cpy12_w(l + 0x28) > 0x14 {
            let mut semeou = false;
            for k in 0..3usize {
                let f = cpy12_d(clube + 0x1cf + k * 4);
                if f == 0 || cpy12_d(f + 0x69) == 0 {
                    continue;
                }
                semear((cpy12_d(j) as u32).wrapping_add(1).wrapping_mul((cpy12_d(f) as u32).wrapping_add(1)));
                semeou = true;
                CPY12_SEMEOU = true;
                crate::comparacao::ramo("cpy_12 fisioterapeuta avalia");
                let np = cpy12_d(f + 0x69);
                // a original empurra o registrador inteiro com o byte +0x1e no baixo; a 0x544410 só lê o byte
                let n = || nota(np, cpy12_b(np + 0x1e) as u32) as i32;
                if aposentadoria(cpy12_d(0xb7434c), j, 1) != 0 {
                    if sorteio(n()) != 0 {
                        return 0x2000;
                    }
                    if aposentadoria(cpy12_d(0xb7434c), j, 0) != 0 {
                        return 0x2000;
                    }
                }
                let r = sorteio(200); // cpy12: o sorteio da gravidade
                if cpy12_w(l + 0x28) as i32 > r.wrapping_add(0x19) {
                    let a = sorteio((0x29 - n()) * 5);
                    let b = sorteio(cpy12_w(l + 0x28) as i32);
                    if a < b {
                        return 0x2000;
                    }
                }
                if recaida(cpy12_d(0xb63d28), j) != 0 {
                    let c = n();
                    let rr = recaida(cpy12_d(0xb63d28), j) as i8 as i32;
                    let a = sorteio(c + rr);
                    let b = sorteio(10);
                    if a > b {
                        return 0x2000;
                    }
                }
                let c = n();
                if sorteio(ftol(pow_do_jogo(c as f64, f64_em(0x968930)) + f64_em(0x9688a8))) == 0 {
                    return 0x2000;
                }
                let a = sorteio(n());
                let b = sorteio(200);
                if a > b {
                    let a = sorteio(cpy12_b(cpy12_d(j + 0x61) + 0x2d) as i8 as i32);
                    let b = sorteio(200);
                    if a > b {
                        return 0x2000;
                    }
                }
            }
            if semeou {
                let mut t = 0i32;
                semear(relogio(&mut t) as u32);
            }
        }
    }
    let c = contrato(cpy12_d(0xae1050), j);
    if c == 0 {
        return 0;
    }
    let tipo = cpy12_b(c + 0x35) & 0x3f;
    if tipo == 1 || tipo == 3 || cpy12_d(j + 0x39) == 0 || humano(clube, 1, 1, 0, 0) == 0 {
        return 0;
    }
    let x = cpy12_w(clube + 0x80) as f64 * f64_em(0x968918);
    let mut limite = ftol(x * x);
    if limite < 0x249f0 {
        limite = 0x249f0;
    } else if limite > 0x1e8480 {
        limite = 0x1e8480;
    }
    if comprando as u8 == 0 {
        let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
        let nacao = cpy12_d(clube + 0x53);
        let divisao = cpy12_d(clube + 0x57);
        if nacao != 0 && cpy12_b(nacao + 0x11c) != 0 && divisao != 0 && cpy12_d(divisao) != cpy12_d(0x9cf764) && (contar(clube, 0, 0) as i8) < 0xf {
            cpy12_confianca(this, clube, 6, -0x4e2);
            cpy12_confianca(this, clube, 0x14, -0x4e2);
            cpy12_confianca(this, clube, 0x16, -500);
            cpy12_piso(this, clube);
            return 0x200;
        }
        let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        let mut pedido = preco(cpy12_d(0xae1050), cpy12_d(j + 0x39), clube, j, 0, 0, 1);
        let pelo_contrato = cpy12_valor(j, cpy12_clube_do_contrato(j), 1);
        if pedido > pelo_contrato {
            pedido = pelo_contrato;
        }
        let pelo_atual = cpy12_valor(j, cpy12_d(j + 0x39), 0);
        if pedido <= limite || pelo_atual < valor {
            return 0;
        }
        let (p, v) = (pedido as f64, valor as f64);
        let mais = cpy12_b(c + 0x3d) as i8;
        crate::comparacao::ramo("cpy_12 diretoria vendendo");
        if p > v + v && mais > 0x19 {
            cpy12_confianca(this, clube, 6, -0x4e2);
            cpy12_confianca(this, clube, 0x14, -0x4e2);
        } else if pedido > 0x4c4b40 && mais > 0 && p > v * f64_em(0x968e18) {
            cpy12_confianca(this, clube, 6, -0x4e2);
            cpy12_confianca(this, clube, 0x14, -0x4e2);
        } else if p > v * f64_em(0x969000) {
            cpy12_confianca(this, clube, 6, -500);
            cpy12_confianca(this, clube, 0x14, -500);
            cpy12_confianca(this, clube, 0x16, -0xfa);
            return cpy12_piso(this, clube);
        } else {
            return 0;
        }
        cpy12_confianca(this, clube, 0x16, -500);
        return cpy12_piso(this, clube);
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let orcamento = verba(financas(cpy12_d(0xae2a38), clube), j, 0, 0, 1, 0);
    if valor as i64 > orcamento.wrapping_mul(12) / 10 { // cpy12: a verba da diretoria
        crate::comparacao::ramo("cpy_12 diretoria sem verba");
        return 0x800;
    }
    if valor <= limite {
        return 0;
    }
    let pelo_clube = cpy12_valor(j, clube, 0);
    let pelo_contrato = cpy12_valor(j, cpy12_clube_do_contrato(j), 1);
    let menor = if pelo_clube > pelo_contrato { pelo_contrato } else { pelo_clube };
    let fin = financas(cpy12_d(0xae2a38), clube);
    if fin == 0 {
        return 0;
    }
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let k = match faixa(fin) {
        2 => f64_em(0x96d1f8),
        1 => f64_em(0x96d1f0),
        _ => f64_em(0x968ea0),
    };
    let teto = ftol(limite as f64 * f64_em(0x96a5d0) + menor as f64 + k);
    if !(valor as f64 * f64_em(0x96d1e8) > teto as f64) {
        return 0;
    }
    crate::comparacao::ramo("cpy_12 diretoria acha caro");
    cpy12_confianca(this, clube, 6, -0xfa);
    cpy12_confianca(this, clube, 0x14, -500);
    cpy12_confianca(this, clube, 0x16, -500);
    cpy12_piso(this, clube)
}

/// Comparador: lado a lado a partir do mesmo estado do registro da diretoria do clube, do contrato guardado em
/// [0xae1080] e do sorteio; a devolução e tudo isso têm de bater (o rand da CRT, só sem a semente do relógio).
unsafe extern "thiscall" fn comparar_veto_dos_fisios_e_da_diretoria(this: usize, j: usize, clube: usize, valor: i32, comprando: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32>(1374);
    let regioes = [(cpy12_diretoria(this, clube), 0x49), (0xae1080, 4), (0xde1f5c, 8), (0xad6c30, 4)];
    let (r, n, _, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, j, clube, valor, comprando),
        || crate::aleatorio::sem_contar(|| veto_dos_fisios_e_da_diretoria(this, j, clube, valor, comprando)));
    let partes = if CPY12_SEMEOU { 3 } else { 4 };
    let igual = (0..partes).all(|i| mo.0[i] == mr.0[i]);
    PLACAR.registrar(igual && r == n, || format!("veto_dos_fisios_e_da_diretoria(clube {:x}, valor {}, comprando {}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, valor, comprando as u8, r, n, mo, mr));
    r
}

/// { 0x6ac700 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH da notícia local): `MANAGER_MANAGER::
/// post_board_ultimatum_news(clube, adversário, competição)`, pelo ultimato da diretoria (0x6ac560). Clube com divisão
/// (+0x57), técnico (+0xcf) e a competição: a notícia 0x178d com o técnico (o id e as três chaves), o clube, o adversário,
/// a competição e um sorteio(5); clube de reputação (+0x80) abaixo de 5750, ao técnico do clube (0x7861e0); senão, por
/// sorteio(3), aos técnicos da divisão (0x7862d0) ou aos do país (+0x53, 0x7863f0).
unsafe extern "thiscall" fn noticia_do_ultimato(_this: usize, clube: usize, adversario: usize, comp: usize) {
    if clube == 0 || cpy12_d(clube + 0x57) == 0 || cpy12_d(clube + 0xcf) == 0 || comp == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let ao_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x178d, 0);
    let t = cpy12_d(clube + 0xcf);
    gravar(nt, 0, cpy12_d(t) as u32);
    gravar(nt, 1, cpy12_d(cpy12_d(t + 4) + 0x33) as u32);
    gravar(nt, 2, cpy12_d(cpy12_d(t + 8) + 0x33) as u32);
    gravar(nt, 3, cpy12_d(cpy12_d(t + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpy12_d(clube) as u32);
    gravar(nt, 5, cpy12_d(adversario) as u32);
    gravar(nt, 6, cpy12_d(comp) as u32);
    crate::comparacao::ramo("cpy_12 noticia do ultimato");
    gravar(nt, 7, sorteio(5) as u32); // cpy12: o sorteio do ultimato
    if cpy12_w(clube + 0x80) < 0x1676 {
        ao_clube(cpy12_d(0xb7431c), nt, clube);
    } else if sorteio(3) != 0 {
        a_divisao(cpy12_d(0xb7431c), nt, cpy12_d(clube + 0x57));
    } else {
        ao_pais(cpy12_d(0xb7431c), nt, cpy12_d(clube + 0x53));
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca) e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_ultimato(this: usize, clube: usize, adversario: usize, comp: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1375)(this, clube, adversario, comp)
}

/// { 0x6ac880 } (stdcall, ret 0xc, manager_manager.cpp): `news_board_ultimatum_text(notícia, corpo, saída)`: lê os
/// campos 0 a 7 da notícia (0x785830: o técnico, as três chaves, o clube, o adversário, a competição); o nome do técnico
/// (0x53dba0, 500, estilo 1). Corpo: o texto 0xa12dac em 0xde1f64 (0x66f4e0) com os gêneros (+0x37 dos clubes, +0x52 da
/// competição), os nomes (+4 dos clubes, +0x38 da competição) e o do técnico, na `saída` (0x4ae660), e as ligações
/// (0x4ae8a0): o técnico (0x7da, se os nomes conferem pela 0x53e990), o clube e o adversário (0x7d5). Título: 0xa12d64
/// com o gênero (+0x52) e o nome curto (+0x38) do clube.
unsafe extern "stdcall" fn texto_do_ultimato(noticia: usize, corpo: u32, saida: usize) {
    let campo: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(0x785830usize);
    let nome_da_pessoa: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x53e990usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let por_texto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ae660usize);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x4ae8a0usize);
    crate::comparacao::ramo("cpy_12 texto do ultimato");
    let v: [u32; 7] = core::array::from_fn(|k| campo(noticia, k as i8));
    campo(noticia, 7);
    let pessoa = cpy12_d(0xae23bc).wrapping_add((v[0] as usize).wrapping_mul(0x6e));
    let clube = cpy12_clube(v[4] as usize);
    let adversario = cpy12_clube(v[5] as usize);
    let comp = cpy12_d(0xae23d0).wrapping_add((v[6] as usize).wrapping_mul(0x6b));
    let mut nome = [0u8; 0x200];
    let buf = nome.as_mut_ptr() as usize;
    nome_da_pessoa(pessoa, buf, 0x1f4, 1);
    if corpo != 0 {
        let gc = cpy12_b(comp + 0x52) as i8 as i32;
        let ga = cpy12_b(adversario + 0x37) as i8 as i32;
        let gl = cpy12_b(clube + 0x37) as i8 as i32;
        formatar(0xde1f64, 0xa12dac, gl, gl, ga, ga, gc, gc, clube + 4, buf, adversario + 4, comp + 0x38);
        por_texto(saida, 0xde1f64);
        if confere(pessoa, v[1], v[2], v[3], 0) != 0 {
            ligar(saida, buf, 0x7da, pessoa);
        }
        ligar(saida, clube + 4, 0x7d5, clube);
        ligar(saida, adversario + 4, 0x7d5, adversario);
    } else {
        let g = cpy12_b(clube + 0x52) as i8 as i32;
        formatar(0xde1f64, 0xa12d64, g, g, clube + 0x38); // "... manager given ultimatum by board"
        por_texto(saida, 0xde1f64);
    }
}

/// Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original.
unsafe extern "stdcall" fn comparar_texto_do_ultimato(noticia: usize, corpo: u32, saida: usize) {
    // ponytail: o objeto de texto não pode receber o texto duas vezes; só a original roda aqui
    orig::<unsafe extern "stdcall" fn(usize, u32, usize)>(1376)(noticia, corpo, saida);
}

// ---------- 261a leva: a pessoa liberada sem custo, as férias do técnico humano e a notícia da permissão para discutir o contrato (transfer_manager.cpp, human_manager.cpp) ----------

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
unsafe extern "thiscall" fn liberar_sem_custo(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
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
unsafe extern "thiscall" fn comparar_liberar_sem_custo(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let emprestada = ficha(cpy13_d(0xae1050), p) != 0;
    let n = cpy13_d(this + 0x86a);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32)>(1377)(this, p, clube, motivo, compensacao, so_registro);
    if emprestada {
        PLACAR.registrar(cpy13_d(this + 0x86a) == n, || format!("liberar_sem_custo(pessoa {}) emprestada: conta {} -> {}", cpy13_d(p), n, cpy13_d(this + 0x86a)));
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
    PLACAR.registrar(ok, || format!("liberar_sem_custo(pessoa {}, clube {:x}, só registro {}): conta {} -> {}, registro {:x?}", cpy13_d(p), clube, so_registro,
        n, cpy13_d(this + 0x86a), core::slice::from_raw_parts(r as *const u8, 0x1d)));
}

/// { 0x5fd2f0 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::set_on_holiday(férias, técnico)`: sem técnico, o
/// humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); nenhum: a caixa da linha 0x41c. O índice do técnico (id − [0xae2364]
/// + 16) fora de 0..[this+4]: a caixa da linha 0x428. A marca de férias (+0x9a01 do registro de 0xa6ef bytes em [this])
/// passa a `férias`, com a conta dos humanos de férias (+0xc) subindo ao ligar e descendo ao desligar; ao ligar, as
/// notícias pendentes do técnico com o clube (+0x39) e com a seleção (+0x24) são respondidas (0x7874b0); com o dia em
/// [0xb67a50], o interesse do dia (0x6b5040).
unsafe extern "thiscall" fn ferias_do_humano(this: usize, ferias: u32, tecnico: usize) {
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
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7874b0usize);
        if cpy13_d(t + 0x39) != 0 {
            responder(cpy13_d(0xb7431c), t, cpy13_d(t + 0x39));
        }
        if cpy13_d(t + 0x24) != 0 {
            responder(cpy13_d(0xb7431c), t, cpy13_d(t + 0x24));
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
unsafe extern "thiscall" fn comparar_ferias_do_humano(this: usize, ferias: u32, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(1378);
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
    PLACAR.registrar(depois == ferias && conta2 == esperada, || format!("ferias_do_humano({}, técnico {:x}): marca {:x} -> {:x}, conta {} -> {} (esperava {})", ferias, t, antes, depois, conta, conta2, esperada));
}

/// { 0x8d7cd0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH das notícias locais): `TRANSFER_MANAGER::
/// post_contract_talks_news(oferta)`, pela oferta aceita (0x8c0080). O vendedor: o clube do contrato da pessoa (0x4cf5a0,
/// guardado em [0xae1080]) fora do tipo 5. Comprador humano (0x7875e0): a notícia 0xfac com a pessoa (o id e as três
/// chaves), o vendedor (ou −1), o comprador, +0x20, +0x24, 0 e 0. Vendedor humano com troca (+4): uma 0xfac por jogador
/// da troca, com ele, o comprador, o vendedor, +0x20, +0x24, 0 e 1, ao vendedor.
unsafe extern "thiscall" fn noticia_conversa_de_contrato(_this: usize, o: usize) {
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
unsafe extern "thiscall" fn comparar_noticia_conversa_de_contrato(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1379)(this, o)
}
