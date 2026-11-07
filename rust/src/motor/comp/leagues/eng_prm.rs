// ---------- 278a leva: os construtores das quatro divisões inglesas (a Premier, a First, a Second e a Third Division: as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------



/// O que muda de uma divisão inglesa para outra no construtor.
struct Cql6Divisao {
    /// a vtable da classe derivada
    vtabela: usize,
    /// a capacidade mínima do estádio (+0xe2) e os lugares sentados mínimos (+0xe4), quando a divisão os grava
    estadio: Option<(u16, u16)>,
    /// quantas fases além da liga (+0x2c): 1 = o play-off de acesso (o vetor [+0xc] de 1 fase, malloc de 4)
    fases: u32,
    /// set_up_rules e create_table da divisão (mov ecx, this -> thiscall, ret 0; devolvem 0 na falha)
    regras: usize,
    tabela: usize,
    /// o fonte e as linhas das cinco caixas (regras, tabela, jogos, ordem, estatísticas)
    arquivo: usize,
    linhas: [u32; 5],
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
unsafe fn cql6_construir(this: usize, ano: u32, registro: usize, p: &Cql6Divisao) -> usize {
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

const CQL6_PREMIER: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969d1c,
    estadio: Some((0x2710, 0x2710)),
    fases: 0,
    regras: 0x574b70,
    tabela: 0x574a20,
    arquivo: 0x9c43b0,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
const CQL6_FIRST: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969c68,
    estadio: None,
    fases: 1, // cql6: o play-off da First Division
    regras: 0x572d90,
    tabela: 0x572bf0,
    arquivo: 0x9c4374,
    linhas: [0x49, 0x50, 0x57, 0x60, 0x68],
};
const CQL6_SECOND: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969dd0,
    estadio: None,
    fases: 1, // cql6: o play-off da Second Division
    regras: 0x576780,
    tabela: 0x576630,
    arquivo: 0x9c43e8,
    linhas: [0x4a, 0x51, 0x58, 0x61, 0x68],
};
const CQL6_THIRD: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969e84,
    estadio: Some((0x1770, 0x3e8)),
    fases: 1,
    regras: 0x5780c0,
    tabela: 0x577f70,
    arquivo: 0x9c4424,
    linhas: [0x4e, 0x55, 0x5c, 0x65, 0x6d],
};

/// { 0x573380 } (thiscall, ret 8, eng_prm.cpp): `EngPrm::EngPrm(ano, registro)`. A Premier: sem fases além da liga
/// (+0x2c = 0, +0xc = 0) e o estádio mínimo de 10.000 lugares, todos sentados.
unsafe extern "thiscall" fn construir_premier(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_PREMIER)
}

/// { 0x571680 } (thiscall, ret 8, eng_first.cpp): `EngFirst::EngFirst(ano, registro)`. A First Division: 1 fase além da
/// liga (o play-off de acesso), sem estádio mínimo.
unsafe extern "thiscall" fn construir_first(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_FIRST)
}

/// { 0x5754a0 } (thiscall, ret 8, eng_second.cpp): `EngSecond::EngSecond(ano, registro)`. A Second Division: 1 fase além
/// da liga (o play-off), sem estádio mínimo.
unsafe extern "thiscall" fn construir_second(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_SECOND)
}

/// { 0x576dd0 } (thiscall, ret 8, eng_third.cpp): `EngThird::EngThird(ano, registro)`. A Third Division: 1 fase além da
/// liga (o play-off) e o estádio mínimo de 6.000 lugares, 1.000 sentados.
unsafe extern "thiscall" fn construir_third(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_THIRD)
}

/// O comparador dos quatro: alocam (as estatísticas, a tabela, os jogos): só a original, conferindo o que ela grava:
/// a devolução, a vtable, o ano, o registro e a regra de disciplina; na liga nova (+0x44 = 0xff), +0x2c, o vetor das
/// fases, a fase atual −1 e as estatísticas em +8.
unsafe fn cql6_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cql6Divisao, original: impl FnOnce() -> usize) -> usize {
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

unsafe extern "thiscall" fn comparar_construir_premier(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1861);
    cql6_comparar("construir_premier", this, ano, registro, &CQL6_PREMIER, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1862);
    cql6_comparar("construir_first", this, ano, registro, &CQL6_FIRST, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1863);
    cql6_comparar("construir_second", this, ano, registro, &CQL6_SECOND, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1864);
    cql6_comparar("construir_third", this, ano, registro, &CQL6_THIRD, || o(this, ano, registro))
}

// ---------- 275a leva: as regras das quatro divisões inglesas: turnos, pontos, vagas de acesso, mata-mata e rebaixamento, desempates e a tabela de rodadas (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------


/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQI4_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqi4_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
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
fn cqi4_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0x52, 1, 2)]
}

/// { 0x574b70 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::set_up_rules()`. A Premier: as comuns, +0xab e +0xdb = 0
/// (as palavras copiadas em cada partida), ninguém sobe (+0xbe = 0, +0xbf = 0, +0xc0 = 0), 3 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1) e a First abaixo (+0x20 = [0x9cf5c0]).
unsafe extern "thiscall" fn regras_da_premier(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da premier");
    cqi4_regras(this, &cqi4_premier())
}

unsafe fn cqi4_premier() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Premier, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32)]);
    v
}

/// { 0x572d90 } (thiscall, ret 0, eng_first.cpp): `EngFirst::set_up_rules()`. A First: as comuns, 2 sobem direto
/// (+0xbe), 4 vão ao mata-mata de acesso (+0xbf, do 3º ao 6º), nenhum play-off de permanência (+0xc0), 3 caem
/// (+0xc1), a Premier acima (+0x1c = [0x9cf5bc]) e a Second abaixo (+0x20 = [0x9cf5c4]), 5 reservas no banco
/// (+0x49) e 3 substituições (+0x4a).
unsafe extern "thiscall" fn regras_da_first(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da first");
    cqi4_regras(this, &cqi4_first())
}

unsafe fn cqi4_first() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x576780 } (thiscall, ret 0, eng_second.cpp): `EngSecond::set_up_rules()`. A Second: como a First, mas 4
/// caem (+0xc1); a First acima ([0x9cf5c0]) e a Third abaixo ([0x9cf5c8]).
unsafe extern "thiscall" fn regras_da_second(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da second");
    cqi4_regras(this, &cqi4_second())
}

unsafe fn cqi4_second() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Second dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x5780c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::set_up_rules()`. A Third: as comuns, 3 sobem direto
/// (+0xbe), 4 ao mata-mata (+0xbf, do 4º ao 7º), 1 cai (+0xc1), a Second acima ([0x9cf5c4]) e abaixo a Conference
/// ([0x9cf69c]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da nação, [[+4]+0x5d]
/// [inferência]), senão −1; 5 reservas e 3 substituições.
unsafe extern "thiscall" fn regras_da_third(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da third");
    cqi4_regras(this, &cqi4_third(this))
}

unsafe fn cqi4_third(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Third, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xbe, 1, 3), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32)]);
    let nacao = read_usize(read_usize(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x4a, 1, 3), (0x49, 1, 5)]);
    v
}

/// O comparador das quatro: a original monta a tabela de rodadas (aloca; não roda duas vezes), então é ensaio. A
/// original roda inteira; a faixa +0x1c..+0xec volta como estava e a cópia grava as regras sem a virtual +0x3c; os
/// campos que a cópia grava têm de sair iguais aos da original, e a devolução 1; no fim fica a faixa da original
/// (com a tabela de rodadas dela em +0xba, +0xa9 e +0x3a).
unsafe fn cqi4_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
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

unsafe extern "thiscall" fn comparar_regras_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1740);
    cqi4_comparar("regras_da_premier", this, cqi4_premier(), || o(this), || regras_da_premier(this))
}

unsafe extern "thiscall" fn comparar_regras_da_first(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1741);
    cqi4_comparar("regras_da_first", this, cqi4_first(), || o(this), || regras_da_first(this))
}

unsafe extern "thiscall" fn comparar_regras_da_second(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1742);
    cqi4_comparar("regras_da_second", this, cqi4_second(), || o(this), || regras_da_second(this))
}

unsafe extern "thiscall" fn comparar_regras_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1743);
    cqi4_comparar("regras_da_third", this, cqi4_third(this), || o(this), || regras_da_third(this))
}

// ---------- 275a leva: a tabela das quatro divisões inglesas: os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV no início da temporada (eng_prm.cpp, eng_second.cpp, eng_third.cpp, eng_first.cpp) ----------



/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqi5_registro(g: usize) -> usize {
    read_usize(0xae23d0).wrapping_add((read_usize(g) as i32 as usize).wrapping_mul(0x6b))
}

/// O ano do jogo ([0xae2c92]) passou de 1999 (o dinheiro da TV maior).
unsafe fn cqi5_depois_de_1999() -> bool {
    core::ptr::read_unaligned(0xae2c92 as *const u16) > 0x7cf
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8): o valor a cada clube da tabela, ou só aos
/// que vieram da competição `filtro` (+0x5b do clube).
unsafe fn cqi5_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, valor, filtro);
}

/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Sem encher: 0. Cheia: o fundo de prêmios
/// +0x28 (League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4), +0xe1 = 1, o dinheiro da TV (`pagar`) e 1.
unsafe fn cqi5_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, cota: i32, pagar: impl FnOnce()) -> u32 {
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
    let mut clube = read_usize(0xae23b4);
    while (i as i32) < read_usize(0xae235c) as i32 {
        let comp = read_usize(clube + 0x57);
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

/// { 0x574a20 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::create_table()`: 20 clubes ([0x9cf5bc]), cota 250.000 do
/// fundo de prêmios e a TV: 12.000.000 a cada clube depois de 1999 (5.000.000 antes). Fonte 0x9c43b0, linha 0x1ee.
unsafe extern "thiscall" fn tabela_da_premier(this: usize) -> u32 {
    cqi5_tabela(this, 0x14, 0x9c43b0, 0x1ee, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || {
        let valor = if cqi5_depois_de_1999() { 0xb71b00 } else { 0x4c4b40 }; // cqi5: a TV da Premier
        cqi5_pagar(this, valor, 0)
    })
}

/// { 0x576630 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_table()`: 24 clubes ([0x9cf5c4]), cota 20.000
/// e a TV: 2.500.000 depois de 1999 (300.000 antes). Fonte 0x9c43e8, linha 0x1f5.
unsafe extern "thiscall" fn tabela_da_second(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c43e8, 0x1f5, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0x2625a0 } else { 0x493e0 }, 0))
}

/// { 0x577f70 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_table()`: 24 clubes ([0x9cf5c8]), cota 10.000
/// e a TV: 1.000.000 depois de 1999 (200.000 antes). Fonte 0x9c4424, linha 0x1fa.
unsafe extern "thiscall" fn tabela_da_third(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4424, 0x1fa, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0xf4240 } else { 0x30d40 }, 0))
}

/// { 0x572bf0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_table()`: 24 clubes ([0x9cf5c0]), cota 70.000
/// e a TV pela divisão de onde o clube veio (+0x5b): depois de 1999, 5.877.777 aos que caíram da Premier (o registro
/// de [0x9cf5bc]), 4.099.999 aos que já estavam na First (o próprio registro, +4) e 4.099.999 aos que subiram da
/// Second ([0x9cf5c4]); antes, 1.000.000 a todos. Fonte 0x9c4374, linha 0x22d.
unsafe extern "thiscall" fn tabela_da_first(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4374, 0x22d, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, 0x11170, || {
        if cqi5_depois_de_1999() {
            cqi5_pagar(this, 0x59b011, cqi5_registro(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION)); // cqi5: a TV de quem caiu da Premier
            cqi5_pagar(this, 0x3e8fa0, read_usize(this + 4));
            cqi5_pagar(this, 0x3e8fa0, cqi5_registro(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION));
        } else {
            cqi5_pagar(this, 0xf4240, 0);
        }
    })
}

/// O comparador das quatro: a original aloca a tabela e paga os clubes (não roda duas vezes): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu
/// (senão 0) e as linhas da tabela com os clubes do plano, na ordem; cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqi5_comparar(nome: &str, this: usize, n: i16, g: usize, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(0xae23b4) + (i as usize) * 0x245;
        let comp = read_usize(clube + 0x57);
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

unsafe extern "thiscall" fn comparar_tabela_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1744);
    cqi5_comparar("tabela_da_premier", this, 0x14, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_second(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1745);
    cqi5_comparar("tabela_da_second", this, 0x18, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1746);
    cqi5_comparar("tabela_da_third", this, 0x18, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_first(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1747);
    cqi5_comparar("tabela_da_first", this, 0x18, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, 0x11170, || o(this))
}

