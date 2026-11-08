use super::*;

// ---------- 278a leva: os construtores das quatro divisões inglesas (a Premier, a First, a Second e a Third Division: as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------



/// O que muda de uma divisão inglesa para outra no construtor.
pub(crate) struct Cql6Divisao {
    /// a vtable da classe derivada
    pub(crate) vtabela: usize,
    /// a capacidade mínima do estádio (+0xe2) e os lugares sentados mínimos (+0xe4), quando a divisão os grava
    pub(crate) estadio: Option<(u16, u16)>,
    /// quantas fases além da liga (+0x2c): 1 = o play-off de acesso (o vetor [+0xc] de 1 fase, malloc de 4)
    pub(crate) fases: u32,
    /// set_up_rules e create_table da divisão (mov ecx, this -> thiscall, ret 0; devolvem 0 na falha)
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    /// o fonte e as linhas das cinco caixas (regras, tabela, jogos, ordem, estatísticas)
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o ano (+0x40, palavra), o registro da competição (+4), a vtable,
/// a regra de disciplina 9 (+0x50) e o estádio mínimo; tenta carregar o estado do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova:
/// fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c fases (e o vetor [+0xc]), as regras, a tabela, os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0), a ordem da tabela (League::sort_table 0x6827d0:
/// push 0; mov ecx -> thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl;
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (League::set_predicted_positions 0x68a850: mov ecx -> thiscall, ret 0). Cada passo que falha mostra a caixa da
/// sua linha e encerra. Devolve this.
pub(crate) unsafe fn cql6_construir(this: usize, ano: u32, registro: usize, p: &Cql6Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_PREDICTED_POSITIONS);
    base(this);
    write_u32_from_usize(this + 4, registro);
    write_u32_from_usize(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 9; // cql6: a regra de disciplina das ligas inglesas
    if let Some((capacidade, sentados)) = p.estadio {
        core::ptr::write_unaligned((this + 0xe2) as *mut u16, capacidade);
        core::ptr::write_unaligned((this + 0xe4) as *mut u16, sentados);
    }
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cql_6 divisao inglesa nova");
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    write_u32_from_usize(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(4);
        write_u32_from_usize(this + 0xc, v);
    } else {
        write_u32_from_usize(this + 0xc, 0);
    }
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    write_u32_from_usize(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    this
}

pub(crate) const CQL6_PREMIER: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969d1c,
    estadio: Some((0x2710, 0x2710)),
    fases: 0,
    regras: 0x574b70,
    tabela: 0x574a20,
    arquivo: 0x9c43b0,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
pub(crate) const CQL6_FIRST: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969c68,
    estadio: None,
    fases: 1, // cql6: o play-off da First Division
    regras: 0x572d90,
    tabela: 0x572bf0,
    arquivo: 0x9c4374,
    linhas: [0x49, 0x50, 0x57, 0x60, 0x68],
};
pub(crate) const CQL6_SECOND: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969dd0,
    estadio: None,
    fases: 1, // cql6: o play-off da Second Division
    regras: 0x576780,
    tabela: 0x576630,
    arquivo: 0x9c43e8,
    linhas: [0x4a, 0x51, 0x58, 0x61, 0x68],
};
pub(crate) const CQL6_THIRD: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969e84,
    estadio: Some((0x1770, 0x3e8)),
    fases: 1,
    regras: 0x5780c0,
    tabela: 0x577f70,
    arquivo: 0x9c4424,
    linhas: [0x4e, 0x55, 0x5c, 0x65, 0x6d],
};

/// the Premiership: discipline rule 9 (the English leagues one), minimum ground of 10,000 all seated, only the league
/// (no play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted
/// positions
/// Original: `EngPrm::EngPrm` (0x573380, eng_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x573380 } (thiscall, ret 8, eng_prm.cpp): `EngPrm::EngPrm(ano, registro)`. A Premier: sem fases além da liga
/// (+0x2c = 0, +0xc = 0) e o estádio mínimo de 10.000 lugares, todos sentados.
pub(crate) unsafe extern "thiscall" fn construir_premier(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_PREMIER)
}



/// the Third Division: discipline 9, minimum ground of 6,000 with 1,000 seated, the league and the play-off
/// Original: `EngThird::EngThird` (0x576dd0, eng_third.cpp, thiscall, ret 8).
///
/// [pt] { 0x576dd0 } (thiscall, ret 8, eng_third.cpp): `EngThird::EngThird(ano, registro)`. A Third Division: 1 fase além da
/// liga (o play-off) e o estádio mínimo de 6.000 lugares, 1.000 sentados.
pub(crate) unsafe extern "thiscall" fn construir_third(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_THIRD)
}

/// O comparador dos quatro: alocam (as estatísticas, a tabela, os jogos): só a original, conferindo o que ela grava:
/// a devolução, a vtable, o ano, o registro e a regra de disciplina; na liga nova (+0x44 = 0xff), +0x2c, o vetor das
/// fases, a fase atual −1 e as estatísticas em +8.
pub(crate) unsafe fn cql6_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cql6Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_u32_as_usize(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_u32_as_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 9;
    if let Some((c, s)) = p.estadio {
        ok = ok && core::ptr::read_unaligned((this + 0xe2) as *const u16) == c && core::ptr::read_unaligned((this + 0xe4) as *const u16) == s;
    }
    if *((this + 0x44) as *const u8) == 0xff && read_u32_as_usize(this + 8) != 0 {
        ok = ok && read_u32_as_usize(this + 0x2c) == p.fases as usize && (read_u32_as_usize(this + 0xc) != 0) == (p.fases != 0);
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_u32_as_usize(this), read_u32_as_usize(this + 0x2c), read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 8)));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_premier(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1861);
    cql6_comparar("construir_premier", this, ano, registro, &CQL6_PREMIER, || o(this, ano, registro))
}



pub(crate) unsafe extern "thiscall" fn comparar_construir_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1864);
    cql6_comparar("construir_third", this, ano, registro, &CQL6_THIRD, || o(this, ano, registro))
}

// ---------- 275a leva: as regras das quatro divisões inglesas: turnos, pontos, vagas de acesso, mata-mata e rebaixamento, desempates e a tabela de rodadas (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------


/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static mut CQI4_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn cqi4_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI4_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às quatro divisões (os turnos +0x3c e os pontos da vitória +0xc2 vão em cada uma): 1 ponto pelo empate (+0xc3),
/// categoria 2 nas partidas (+0xc4, liga), clubes nacionais (+0x42 = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7: saldo,
/// gols pró e a posição [inferência pelos códigos da Inglaterra]) e +0x52 = 2 [sem nome].
pub(crate) fn cqi4_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0x52, 1, 2)]
}

/// the Premier (20 clubs): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go straight down
/// to the First; tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; builds the round
/// schedule
/// Original: `EngPrm::set_up_rules` (0x574b70, eng_prm.cpp, thiscall, ret 0).
///
/// [pt] { 0x574b70 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::set_up_rules()`. A Premier: as comuns, +0xab e +0xdb = 0
/// (as palavras copiadas em cada partida), ninguém sobe (+0xbe = 0, +0xbf = 0, +0xc0 = 0), 3 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1) e a First abaixo (+0x20 = [0x9cf5c0]).
pub(crate) unsafe extern "thiscall" fn regras_da_premier(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da premier");
    cqi4_regras(this, &cqi4_premier())
}

pub(crate) unsafe fn cqi4_premier() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Premier, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32)]);
    v
}


pub(crate) unsafe fn cqi4_first() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32), (0x49, 1, 5)]);
    v
}


pub(crate) unsafe fn cqi4_second() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Second dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) as u32), (0x49, 1, 5)]);
    v
}

/// the Third (24): 3 go up directly, 4th to 7th to the play-offs, the bottom club goes down to the Conference when the
/// nation has its lower divisions loaded [inference from nation bit 4] (otherwise there is no league below)
/// Original: `EngThird::set_up_rules` (0x5780c0, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5780c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::set_up_rules()`. A Third: as comuns, 3 sobem direto
/// (+0xbe), 4 ao mata-mata (+0xbf, do 4º ao 7º), 1 cai (+0xc1), a Second acima ([0x9cf5c4]) e abaixo a Conference
/// ([0x9cf69c]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da nação, [[+4]+0x5d]
/// [inferência]), senão −1; 5 reservas e 3 substituições.
pub(crate) unsafe extern "thiscall" fn regras_da_third(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da third");
    cqi4_regras(this, &cqi4_third(this))
}

pub(crate) unsafe fn cqi4_third(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Third, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xbe, 1, 3), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32)]);
    let nacao = read_usize(read_usize(this + 4) + 0x5d);
    let abaixo = if *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) & 4 != 0 { read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x4a, 1, 3), (0x49, 1, 5)]);
    v
}

/// O comparador das quatro: a original monta a tabela de rodadas (aloca; não roda duas vezes), então é ensaio. A
/// original roda inteira; a faixa +0x1c..+0xec volta como estava e a cópia grava as regras sem a virtual +0x3c; os
/// campos que a cópia grava têm de sair iguais aos da original, e a devolução 1; no fim fica a faixa da original
/// (com a tabela de rodadas dela em +0xba, +0xa9 e +0x3a).
pub(crate) unsafe fn cqi4_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQI4_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1740);
    cqi4_comparar("regras_da_premier", this, cqi4_premier(), || o(this), || regras_da_premier(this))
}



pub(crate) unsafe extern "thiscall" fn comparar_regras_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1743);
    cqi4_comparar("regras_da_third", this, cqi4_third(this), || o(this), || regras_da_third(this))
}

// ---------- 275a leva: a tabela das quatro divisões inglesas: os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV no início da temporada (eng_prm.cpp, eng_second.cpp, eng_third.cpp, eng_first.cpp) ----------



/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
pub(crate) unsafe fn cqi5_registro(g: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_usize(g) as i32 as usize).wrapping_mul(0x6b))
}

/// O ano do jogo ([0xae2c92]) passou de 1999 (o dinheiro da TV maior).
pub(crate) unsafe fn cqi5_depois_de_1999() -> bool {
    core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16) > 0x7cf
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8): o valor a cada clube da tabela, ou só aos
/// que vieram da competição `filtro` (+0x5b do clube).
pub(crate) unsafe fn cqi5_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, valor, filtro);
}

/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Sem encher: 0. Cheia: o fundo de prêmios
/// +0x28 (League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4), +0xe1 = 1, o dinheiro da TV (`pagar`) e 1.
pub(crate) unsafe fn cqi5_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, cota: i32, pagar: impl FnOnce()) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(clube + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let linha = k; // cqi5: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != read_i16(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo("cqi_5 tabela cheia");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar();
    1
}

/// the Premier has 20 clubs; the prize fund starts at 250,000 per place; TV pays 12,000,000 to each club (5,000,000 up
/// to 1999)
/// Original: `EngPrm::create_table` (0x574a20, eng_prm.cpp, thiscall, ret 0).
///
/// [pt] { 0x574a20 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::create_table()`: 20 clubes ([0x9cf5bc]), cota 250.000 do
/// fundo de prêmios e a TV: 12.000.000 a cada clube depois de 1999 (5.000.000 antes). Fonte 0x9c43b0, linha 0x1ee.
pub(crate) unsafe extern "thiscall" fn tabela_da_premier(this: usize) -> u32 {
    cqi5_tabela(this, 0x14, 0x9c43b0, 0x1ee, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || {
        let valor = if cqi5_depois_de_1999() { 0xb71b00 } else { 0x4c4b40 }; // cqi5: a TV da Premier
        cqi5_pagar(this, valor, 0)
    })
}


/// the Third has 24 clubs; prize fund of 10,000 per place; TV pays 1,000,000 to each club (200,000 up to 1999)
/// Original: `EngThird::create_table` (0x577f70, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x577f70 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_table()`: 24 clubes ([0x9cf5c8]), cota 10.000
/// e a TV: 1.000.000 depois de 1999 (200.000 antes). Fonte 0x9c4424, linha 0x1fa.
pub(crate) unsafe extern "thiscall" fn tabela_da_third(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4424, 0x1fa, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0xf4240 } else { 0x30d40 }, 0))
}


/// O comparador das quatro: a original aloca a tabela e paga os clubes (não roda duas vezes): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu
/// (senão 0) e as linhas da tabela com os clubes do plano, na ordem; cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn cqi5_comparar(nome: &str, this: usize, n: i16, g: usize, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * 0x245;
        let comp = read_usize(clube + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1744);
    cqi5_comparar("tabela_da_premier", this, 0x14, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || o(this))
}


pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1746);
    cqi5_comparar("tabela_da_third", this, 0x18, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || o(this))
}

/// the Premiership schedule: 38 rounds (20 clubs), from 19 August to 20 May; Christmas (four fixed-date rounds) by the
/// day the 16 December round falls on; no play-offs (main stage only)
/// Original: `EngPrm::create_schedule` (0x573660, eng_prm.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x573660 } (thiscall, ret 0x10, eng_prm.cpp, 5.019 bytes, método +0x3c): `EngPrm::create_schedule(fase, &rodadas,
/// &extra, &é_liga)`: só a fase −1, 38 rodadas (`CQP5_PREMIERSHIP_NATAL`); as outras devolvem 0. Fonte 0x9c43b0.
pub(crate) unsafe extern "thiscall" fn calendario_da_premiership(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 38;
            *extra = 0;
            let t = alocar(0x9a6);
            if t == 0 {
                aviso_em(0x9c43b0, 0xa7);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da premiership");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            cqp5_natal(t, ano, &CQP5_PREMIERSHIP_NATAL, 0x9c43b0);
            t
        }
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_premiership(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1924);
    cqp5_comparar("calendario_da_premiership", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_premiership(this, fase, r, e, l))
}
