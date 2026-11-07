// ---------- 283a leva: os construtores dos estaduais: Baiano, Centro-Oeste e Gaúcho (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cqt1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt1Estadual {
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
unsafe fn cqt1_construir(this: usize, ano: u32, registro: usize, e: &Cqt1Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
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
unsafe fn cqt1_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt1Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt1_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt1_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt1_d(this + 8) != 0 {
        ok = ok && cqt1_d(this + 0x2c) == e.fases as usize && cqt1_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt1_d(this), cqt1_d(this + 0x2c), cqt1_d(this + 0xc), cqt1_d(this + 8)));
    r
}

const CQT1_BAIANO: Cqt1Estadual = Cqt1Estadual {
    vtabela: 0x967fb4,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x42fd10,
    previstas: 0x430340,
    arquivo: 0x98acac,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const CQT1_CENTRO_OESTE: Cqt1Estadual = Cqt1Estadual {
    vtabela: 0x968068,
    fases: 1,
    regras: 0x430ff0,
    tabela: 0x430ef0,
    previstas: 0x431610,
    arquivo: 0x98ae10,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

const CQT1_GAUCHO: Cqt1Estadual = Cqt1Estadual {
    vtabela: 0x96811c,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x4321d0,
    previstas: 0x432980,
    arquivo: 0x98ae50,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x42f2d0 } (thiscall, ret 8, bra_reg_bahia.cpp): `BraRegBahia::BraRegBahia(ano, registro)`: o Baiano (uma fase além da liga).
unsafe extern "thiscall" fn cqt1_construir_baiano(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_BAIANO, 7, true, "cqt_1 baiano novo") // cqt1: a disciplina e as posições previstas do Baiano
}

/// { 0x4304b0 } (thiscall, ret 8, bra_reg_central.cpp): `BraRegCentral::BraRegCentral(ano, registro)`: o Centro-Oeste (uma fase além da liga).
unsafe extern "thiscall" fn cqt1_construir_centro_oeste(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_CENTRO_OESTE, 7, true, "cqt_1 centro_oeste novo") // cqt1: a disciplina e as posições previstas do Centro-Oeste
}

/// { 0x4317c0 } (thiscall, ret 8, bra_reg_gaucho.cpp): `BraRegGaucho::BraRegGaucho(ano, registro)`: o Gaúcho (uma fase além da liga).
unsafe extern "thiscall" fn cqt1_construir_gaucho(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_GAUCHO, 7, true, "cqt_1 gaucho novo") // cqt1: a disciplina e as posições previstas do Gaúcho
}

unsafe extern "thiscall" fn comparar_cqt1_construir_baiano(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2046);
    cqt1_comparar("cqt1_construir_baiano", this, ano, registro, &CQT1_BAIANO, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt1_construir_centro_oeste(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2047);
    cqt1_comparar("cqt1_construir_centro_oeste", this, ano, registro, &CQT1_CENTRO_OESTE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt1_construir_gaucho(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2048);
    cqt1_comparar("cqt1_construir_gaucho", this, ano, registro, &CQT1_GAUCHO, || o(this, ano, registro))
}

// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Baiano, Centro-Oeste e Gaúcho (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cqp7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

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
unsafe fn cqp7_virada(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqp7_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqp7_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqp7_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqp7_d(cqp7_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqp7_d(cqp7_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqp7_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp7_d(cqp7_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp7_d(cqp7_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqp7_d(0xb74340), cqp7_d(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp7_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = cqp7_d(this + 0x30) as i32;
    if fase >= (cqp7_d(this + 0x2c) as i32).wrapping_sub(1) {
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
unsafe fn cqp7_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn cqp7_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqp7_d(this + 0x30) as i32;
    let ultima = antes >= (cqp7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqp7_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x42fe10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::start_new_season()`, a virada da temporada do Baiano (a tabela:
/// 0x42fd10; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp7_virada_do_baiano(this: usize) -> u32 {
    cqp7_virada(this, 0x42fd10, 0x98acac, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_7 virada do baiano") // cqp7: o ano seguinte do Baiano
}

/// { 0x42ffc0 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::advance_stage()`, a fase seguinte do Baiano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x430050); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp7_fase_seguinte_do_baiano(this: usize) {
    cqp7_fase_seguinte(this, 0x98acac, 0x1f8, 0x430050);
}

/// { 0x431090 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::start_new_season()`, a virada da temporada do Centro-Oeste (a tabela:
/// 0x430ef0; caixas nas linhas 0x1cb, 0x1d2, 0x1d8 e 0x1df).
unsafe extern "thiscall" fn cqp7_virada_do_centro_oeste(this: usize) -> u32 {
    cqp7_virada(this, 0x430ef0, 0x98ae10, [0x1cb, 0x1d2, 0x1d8, 0x1df], 1, "cqp_7 virada do centro_oeste") // cqp7: o ano seguinte do Centro-Oeste
}

/// { 0x431240 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::advance_stage()`, a fase seguinte do Centro-Oeste: no fim dos pontos corridos,
/// o mata-mata de 8 (0x4312d0); a caixa da linha 0x1f9.
unsafe extern "thiscall" fn cqp7_fase_seguinte_do_centro_oeste(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae10, 0x1f9, 0x4312d0);
}

/// { 0x432370 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::start_new_season()`, a virada da temporada do Gaúcho (a tabela:
/// 0x4321d0; caixas nas linhas 0x1c6, 0x1cd, 0x1d3 e 0x1da).
unsafe extern "thiscall" fn cqp7_virada_do_gaucho(this: usize) -> u32 {
    cqp7_virada(this, 0x4321d0, 0x98ae50, [0x1c6, 0x1cd, 0x1d3, 0x1da], 1, "cqp_7 virada do gaucho") // cqp7: o ano seguinte do Gaúcho
}

/// { 0x432520 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::advance_stage()`, a fase seguinte do Gaúcho: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4325b0); a caixa da linha 0x1f4.
unsafe extern "thiscall" fn cqp7_fase_seguinte_do_gaucho(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae50, 0x1f4, 0x4325b0);
}

unsafe extern "thiscall" fn comparar_cqp7_virada_do_baiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1926);
    cqp7_comparar_virada("cqp7_virada_do_baiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_baiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1927);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_baiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_virada_do_centro_oeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1928);
    cqp7_comparar_virada("cqp7_virada_do_centro_oeste", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_centro_oeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1929);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_centro_oeste", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_virada_do_gaucho(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1930);
    cqp7_comparar_virada("cqp7_virada_do_gaucho", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_gaucho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1931);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_gaucho", this, || o(this))
}

// ---------- 277a leva: a tabela dos estaduais: Baiano, Centro-Oeste e Gaúcho (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cql8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cql8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cql8_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cql8_d(clube + 0x60);
    comp != 0 && cql8_d(comp) == cql8_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cql8_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cql8_d(0xae23b4);
    while (i as i32) < cql8_d(0xae235c) as i32 {
        if cql8_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cql8_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cql8_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x42fd10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_table()`: o Baiano tem 10 clubes (divisão reserva +0x60 = [0x9cf9c0]).
/// Fonte 0x98acac, linha 0x167.
unsafe extern "thiscall" fn cql8_tabela_do_baiano(this: usize) -> u32 {
    cql8_tabela(this, 0xa, 0x98acac, 0x167, 0x9cf9c0, 0, "cql_8 baiano cheio")
}

/// { 0x430ef0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::load_region_clubs()`: o Centro-Oeste tem 15 clubes (divisão reserva +0x60 = [0x9cf998]).
/// Fonte 0x98ae10, linha 0x168.
unsafe extern "thiscall" fn cql8_tabela_do_centro_oeste(this: usize) -> u32 {
    cql8_tabela(this, 0xf, 0x98ae10, 0x168, 0x9cf998, 0, "cql_8 centro_oeste cheio")
}

/// { 0x4321d0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::load_region_clubs()`: o Gaúcho tem 17 clubes (divisão reserva +0x60 = [0x9cf9c8]).
/// Fonte 0x98ae50, linha 0x163.
unsafe extern "thiscall" fn cql8_tabela_do_gaucho(this: usize) -> u32 {
    cql8_tabela(this, 0x11, 0x98ae50, 0x163, 0x9cf9c8, 0, "cql_8 gaucho cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cql8_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cql8_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cql8_d(0xae23b4) + (i as usize) * 0x245;
        if cql8_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cql8_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cql8_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cql8_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cql8_tabela_do_baiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1828);
    cql8_comparar("cql8_tabela_do_baiano", this, 0xa, 0x9cf9c0, || o(this))
}

unsafe extern "thiscall" fn comparar_cql8_tabela_do_centro_oeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1829);
    cql8_comparar("cql8_tabela_do_centro_oeste", this, 0xf, 0x9cf998, || o(this))
}

unsafe extern "thiscall" fn comparar_cql8_tabela_do_gaucho(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1830);
    cql8_comparar("cql8_tabela_do_gaucho", this, 0x11, 0x9cf9c8, || o(this))
}

// ---------- 277a leva: as semifinais dos estaduais e regionais: Baiano, Norte e Nordeste (bra_reg_bahia.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp) ----------

unsafe fn cqn7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqn7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqn7_clube(liga: usize, l: usize) -> usize {
    cqn7_d(cqn7_d(liga + 0xb1) + l * 0x3b)
}

/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqn7_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn7_clube(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqn7_d(cqn7_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, cqn7_d(this + 4), t, cqn7_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(cqn7_d(this + 0xc) as *mut usize, fase);
    let fase = cqn7_d(cqn7_d(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// { 0x430050 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98acac, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn7_mata_mata_do_baiano(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // cqn7: 1º × 4º, 2º × 3º Baiano
}

/// { 0x4360c0 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af14, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn7_mata_mata_do_norte(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af14, 0x21b, 0x22f, "cqn_7 norte"); // cqn7: 1º × 4º, 2º × 3º Norte
}

/// { 0x4370d0 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af54, linhas 0x20d (sem lista) e 0x221 (sem fase).
unsafe extern "thiscall" fn cqn7_mata_mata_do_nordeste(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
unsafe fn cqn7_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn7_clube(this, l)).collect();
    original();
    let fase = cqn7_d(cqn7_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqn7_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| cqn7_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqn7_w(fase + 0x3e) as usize == linhas.len() && cqn7_d(fase + 4) == cqn7_d(this + 4) && cqn7_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqn7_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_baiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1843);
    cqn7_comparar("cqn7_mata_mata_do_baiano", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_norte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1844);
    cqn7_comparar("cqn7_mata_mata_do_norte", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_nordeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1845);
    cqn7_comparar("cqn7_mata_mata_do_nordeste", this, &[0, 3, 1, 2], || o(this))
}

