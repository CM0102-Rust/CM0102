// ---------- 283a leva: os construtores dos estaduais: Goiano, Mineiro e Norte (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt2Estadual {
    vtabela: usize,
    fases: u32,
    regras: usize,
    tabela: usize,
    previstas: usize,
    arquivo: usize,
    linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
unsafe fn cqt2_construir(this: usize, ano: u32, registro: usize, e: &Cqt2Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
unsafe fn cqt2_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt2Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + 0x2c) == e.fases as usize && read_usize(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_usize(this), read_usize(this + 0x2c), read_usize(this + 0xc), read_usize(this + 8)));
    r
}

const CQT2_GOIANO: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x9681d0,
    fases: 1,
    regras: 0x433640,
    tabela: 0x433540,
    previstas: 0x433b40,
    arquivo: 0x98ae90,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

const CQT2_MINEIRO: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x968284,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x434810,
    previstas: 0x434f80,
    arquivo: 0x98aed0,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const CQT2_NORTE: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x968338,
    fases: 1,
    regras: 0x439430,
    tabela: 0x435d80,
    previstas: 0x436280,
    arquivo: 0x98af14,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x432b50 } (thiscall, ret 8, bra_reg_goias.cpp): `BraRegGoias::BraRegGoias(ano, registro)`: o Goiano (uma fase além da liga).
unsafe extern "thiscall" fn cqt2_construir_goiano(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_GOIANO, 7, true, "cqt_2 goiano novo") // cqt2: a disciplina e as posições previstas do Goiano
}

/// { 0x433d50 } (thiscall, ret 8, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::BraRegMinasGerais(ano, registro)`: o Mineiro (três fases além da liga).
unsafe extern "thiscall" fn cqt2_construir_mineiro(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_MINEIRO, 7, true, "cqt_2 mineiro novo") // cqt2: a disciplina e as posições previstas do Mineiro
}

/// { 0x435340 } (thiscall, ret 8, bra_reg_north.cpp): `BraRegNorth::BraRegNorth(ano, registro)`: o Norte (uma fase além da liga).
unsafe extern "thiscall" fn cqt2_construir_norte(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_NORTE, 7, true, "cqt_2 norte novo") // cqt2: a disciplina e as posições previstas do Norte
}

unsafe extern "thiscall" fn comparar_cqt2_construir_goiano(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2049);
    cqt2_comparar("cqt2_construir_goiano", this, ano, registro, &CQT2_GOIANO, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt2_construir_mineiro(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2050);
    cqt2_comparar("cqt2_construir_mineiro", this, ano, registro, &CQT2_MINEIRO, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt2_construir_norte(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2051);
    cqt2_comparar("cqt2_construir_norte", this, ano, registro, &CQT2_NORTE, || o(this, ano, registro))
}

// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Goiano, Norte e Nordeste (bra_reg_goias.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp) ----------


/// A virada da temporada de um estadual (`start_new_season`, slot 3, +0x08; o mesmo molde nos nove estaduais de pontos
/// corridos com mata-mata: só mudam o fonte, as linhas das caixas e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o ano (+0x40, word) sobe um (`ano`; o contador `ramo`) e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras da temporada
/// (virtual +0x8c, set_up_rules), a tabela com os clubes do estadual (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c) e a renovação dos árbitros do estadual
/// (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe fn cqp8_virada(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = read_usize(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        arquivar(estatisticas, 1);
    }
    if (read_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > read_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(0xb74340), read_usize(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp8_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this);
    }
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqp8_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn cqp8_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = read_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x4336f0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::start_new_season()`, a virada da temporada do Goiano (a tabela:
/// 0x433540; caixas nas linhas 0x1c3, 0x1ca, 0x1d0 e 0x1d7).
unsafe extern "thiscall" fn cqp8_virada_do_goiano(this: usize) -> u32 {
    cqp8_virada(this, 0x433540, 0x98ae90, [0x1c3, 0x1ca, 0x1d0, 0x1d7], 1, "cqp_8 virada do goiano") // cqp8: o ano seguinte do Goiano
}

/// { 0x4338a0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::advance_stage()`, a fase seguinte do Goiano: no fim dos pontos corridos,
/// o mata-mata de 8 (0x433930); a caixa da linha 0x1f1.
unsafe extern "thiscall" fn cqp8_fase_seguinte_do_goiano(this: usize) {
    cqp8_fase_seguinte(this, 0x98ae90, 0x1f1, 0x433930);
}

/// { 0x435e80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::start_new_season()`, a virada da temporada do Norte (a tabela:
/// 0x435d80; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp8_virada_do_norte(this: usize) -> u32 {
    cqp8_virada(this, 0x435d80, 0x98af14, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_8 virada do norte") // cqp8: o ano seguinte do Norte
}

/// { 0x436030 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::advance_stage()`, a fase seguinte do Norte: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4360c0); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp8_fase_seguinte_do_norte(this: usize) {
    cqp8_fase_seguinte(this, 0x98af14, 0x1f8, 0x4360c0);
}

/// { 0x436e90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::start_new_season()`, a virada da temporada do Nordeste (a tabela:
/// 0x436d90; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
unsafe extern "thiscall" fn cqp8_virada_do_nordeste(this: usize) -> u32 {
    cqp8_virada(this, 0x436d90, 0x98af54, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste") // cqp8: o ano seguinte do Nordeste
}

/// { 0x437040 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::advance_stage()`, a fase seguinte do Nordeste: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4370d0); a caixa da linha 0x1ea.
unsafe extern "thiscall" fn cqp8_fase_seguinte_do_nordeste(this: usize) {
    cqp8_fase_seguinte(this, 0x98af54, 0x1ea, 0x4370d0);
}

unsafe extern "thiscall" fn comparar_cqp8_virada_do_goiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1932);
    cqp8_comparar_virada("cqp8_virada_do_goiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_goiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1933);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_goiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_virada_do_norte(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1934);
    cqp8_comparar_virada("cqp8_virada_do_norte", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_norte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1935);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_norte", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_virada_do_nordeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1936);
    cqp8_comparar_virada("cqp8_virada_do_nordeste", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_nordeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1937);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_nordeste", this, || o(this))
}

// ---------- 277a leva: a tabela dos estaduais: Goiano, Mineiro e Norte (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------



/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cql9_do_estadual(clube: usize, g: usize) -> bool {
    let comp = read_usize(clube + 0x60);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cql9_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
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
        if cql9_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
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
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x433540 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_table()`: o Goiano tem 14 clubes (divisão reserva +0x60 = [0x9cf9b4]).
/// Fonte 0x98ae90, linha 0x160.
unsafe extern "thiscall" fn cql9_tabela_do_goiano(this: usize) -> u32 {
    cql9_tabela(this, 0xe, 0x98ae90, 0x160, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0, "cql_9 goiano cheio")
}

/// { 0x434810 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_table()`: o Mineiro tem 12 clubes (divisão reserva +0x60 = [0x9cf9d4]).
/// Fonte 0x98aed0, linha 0x169.
unsafe extern "thiscall" fn cql9_tabela_do_mineiro(this: usize) -> u32 {
    cql9_tabela(this, 0xc, 0x98aed0, 0x169, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0, "cql_9 mineiro cheio")
}

/// { 0x435d80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_table()`: o Norte tem 10 clubes (divisão reserva +0x60 = [0x9cf994]).
/// Fonte 0x98af14, linha 0x167.
unsafe extern "thiscall" fn cql9_tabela_do_norte(this: usize) -> u32 {
    cql9_tabela(this, 0xa, 0x98af14, 0x167, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0, "cql_9 norte cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cql9_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(0xae23b4) + (i as usize) * 0x245;
        if cql9_do_estadual(clube, g) {
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
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cql9_tabela_do_goiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1831);
    cql9_comparar("cql9_tabela_do_goiano", this, 0xe, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, || o(this))
}

unsafe extern "thiscall" fn comparar_cql9_tabela_do_mineiro(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1832);
    cql9_comparar("cql9_tabela_do_mineiro", this, 0xc, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, || o(this))
}

unsafe extern "thiscall" fn comparar_cql9_tabela_do_norte(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1833);
    cql9_comparar("cql9_tabela_do_norte", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, || o(this))
}

// ---------- 274a leva: as regras do Goiano e das duas divisões da J-League (bra_reg_goias.cpp, jap_j1.cpp, jap_j2.cpp) ----------




/// O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x432e50, 0x662e30, 0x664780): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqj7_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    write_u32_at(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os três calendários não gravam
/// nada no objeto além delas (nenhum `mov [edi+..]`).
fn cqj7_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// As regras do Goiano (o que a 0x433640 grava antes do calendário).
unsafe fn cqj7_regras_goiano(o: usize) {
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3); // cqj7: o primeiro desempate do Goiano
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xbe, 8); // os 8 classificados
    write_u16_at(o, 0xd9, 8);
    write_u16_at(o, 0x3c, 1);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc1, 4);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
}

/// As regras da J1 (0x663620).
unsafe fn cqj7_regras_j1(o: usize) {
    write_u16_at(o, 0x3c, 2);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1); // cqj7: os pontos por empate da J1
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0x82);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora, nenhum
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbe, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 2); // dois caem para a J2
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u8_at(o, 0x52, 2);
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2 as *const u32)); // cai para a J2
    write_u16_at(o, 0xd9, 8);
    write_u8_at(o, 0x49, 5);
}

/// As regras da J2 (0x665140): não grava +0x52 nem a palavra +0xd9 (fica a de `League::League`).
unsafe fn cqj7_regras_j2(o: usize) {
    write_u16_at(o, 0x3c, 4); // quatro turnos
    write_u8_at(o, 0xc2, 3); // cqj7: os pontos por vitória da J2
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0x82);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbe, 2); // dois sobem para a J1
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1 as *const u32)); // sobe para a J1
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 5);
}

/// { 0x433640 } (thiscall, bra_reg_goias.cpp): `BraRegGoias::set_up_rules` (slot 36), as regras do Goiano e o
/// calendário; devolve 1.
unsafe extern "thiscall" fn cqj7_regras_do_goiano(this: usize) -> u32 {
    cqj7_regras_goiano(this);
    crate::comparacao::ramo("cqj_7 regras do goiano");
    cqj7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqj7_regras_do_goiano(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1721)(this), || {
        cqj7_regras_goiano(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_do_goiano({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x663620 } (thiscall, jap_j1.cpp): `JapJ1::set_up_rules`, as regras da J1 e o calendário; devolve 1.
unsafe extern "thiscall" fn cqj7_regras_da_j1(this: usize) -> u32 {
    cqj7_regras_j1(this);
    crate::comparacao::ramo("cqj_7 regras da j1");
    cqj7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
unsafe extern "thiscall" fn comparar_cqj7_regras_da_j1(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1722)(this), || {
        cqj7_regras_j1(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_da_j1({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x665140 } (thiscall, jap_j2.cpp): `JapJ2::set_up_rules`, as regras da J2 e o calendário; devolve 1.
unsafe extern "thiscall" fn cqj7_regras_da_j2(this: usize) -> u32 {
    cqj7_regras_j2(this);
    crate::comparacao::ramo("cqj_7 regras da j2");
    cqj7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
unsafe extern "thiscall" fn comparar_cqj7_regras_da_j2(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1723)(this), || {
        cqj7_regras_j2(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_da_j2({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

