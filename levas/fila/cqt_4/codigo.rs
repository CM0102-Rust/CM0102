// ---------- %%LEVA%%a leva: os construtores dos estaduais: Carioca, Catarinense e Paulista (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) e a média de público dos clubes no fim da temporada (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp, game.cpp) ----------

unsafe fn cqt4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct BraRegRioStateChampionship {
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
unsafe fn bra_reg_rio_construct_state_league(this: usize, ano: u32, registro: usize, e: &BraRegRioStateChampionship, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
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
    crate::comparison::branch(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        show_error_at(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        show_error_at(e.arquivo, e.linhas[4]);
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
unsafe fn bra_reg_rio_compare_state_league_ctor(nome: &str, this: usize, ano: u32, registro: usize, e: &BraRegRioStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt4_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt4_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt4_d(this + 8) != 0 {
        ok = ok && cqt4_d(this + 0x2c) == e.fases as usize && cqt4_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt4_d(this), cqt4_d(this + 0x2c), cqt4_d(this + 0xc), cqt4_d(this + 8)));
    r
}

const CARIOCA_CHAMPIONSHIP: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x968608,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x43a500,
    previstas: 0x43add0,
    arquivo: 0x98b014,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const CATARINENSE_STATE_CHAMPIONSHIP: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x9686bc,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x43bb70,
    previstas: 0x43c110,
    arquivo: 0x98b050,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const PAULISTA_STATE_LEAGUE: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x968770,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x43cc40,
    previstas: 0x43d140,
    arquivo: 0x98b090,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x439a40 } (thiscall, ret 8, bra_reg_rio.cpp): `BraRegRio::BraRegRio(ano, registro)`: o Carioca (três fases além da liga).
unsafe extern "thiscall" fn construct_bra_reg_rio(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_rio_construct_state_league(this, ano, registro, &CARIOCA_CHAMPIONSHIP, 7, true, "cqt_4 carioca novo") // cqt4: a disciplina e as posições previstas do Carioca
}

/// { 0x43b130 } (thiscall, ret 8, bra_reg_santa.cpp): `BraRegSanta::BraRegSanta(ano, registro)`: o Catarinense (uma fase além da liga).
unsafe extern "thiscall" fn construct_bra_reg_santa(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_rio_construct_state_league(this, ano, registro, &CATARINENSE_STATE_CHAMPIONSHIP, 7, true, "cqt_4 catarinense novo") // cqt4: a disciplina e as posições previstas do Catarinense
}

/// { 0x43c280 } (thiscall, ret 8, bra_reg_sp.cpp): `BraRegSp::BraRegSp(ano, registro)`: o Paulista (uma fase além da liga).
unsafe extern "thiscall" fn construct_bra_reg_sp(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_rio_construct_state_league(this, ano, registro, &PAULISTA_STATE_LEAGUE, 7, true, "cqt_4 paulista novo") // cqt4: a disciplina e as posições previstas do Paulista
}

unsafe extern "thiscall" fn compare_construct_bra_reg_rio(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_rio", this, ano, registro, &CARIOCA_CHAMPIONSHIP, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_construct_bra_reg_santa(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_santa", this, ano, registro, &CATARINENSE_STATE_CHAMPIONSHIP, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_construct_bra_reg_sp(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_sp", this, ano, registro, &PAULISTA_STATE_LEAGUE, || o(this, ano, registro))
}

unsafe fn cqt4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqt4_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v)
}

/// A reputação da competição de uma liga (+0x69 do registro +4, word).
unsafe fn league_comp_reputation(liga: usize) -> i32 {
    cqt4_w(cqt4_d(liga + 4) + 0x69) as i32
}

/// O piso que sobe com a reputação `r` (dos ramos que puxam o público para cima): com r < 5, 250 × r × f; senão
/// 750 × (r − 4) × f + 1000 (a original compara com a parte inteira + 1000 e grava a conta com 1000 somado antes do
/// truncamento). Abaixo dele, o público vai a ele e o recorde (+0x7b) sobe até ele.
unsafe fn raise_attendance_floor(c: usize, r: i32, f: f64) {
    let lim = if r < 5 {
        let lim = match_pl_ftol((r.wrapping_mul(250)) as f64 * f);
        if cqt4_i(c + 0x73) >= lim {
            return;
        }
        lim
    } else {
        let x = (r.wrapping_sub(4).wrapping_mul(750)) as f64 * f;
        if cqt4_i(c + 0x73) >= match_pl_ftol(x).wrapping_add(1000) {
            return;
        }
        match_pl_ftol(x + 1000.0)
    };
    cqt4_pi(c + 0x73, lim);
    if lim > cqt4_i(c + 0x7b) {
        cqt4_pi(c + 0x7b, lim);
    }
}

/// O teto `lim` do público: acima dele, o público cai a ele e o mínimo (+0x77) desce até ele.
unsafe fn cap_attendance(c: usize, lim: i32) {
    if cqt4_i(c + 0x73) > lim {
        cqt4_pi(c + 0x73, lim);
        if lim < cqt4_i(c + 0x77) {
            cqt4_pi(c + 0x77, lim);
        }
    }
}

/// { 0x5d6280 } (cdecl, game.cpp, sem nome no mapa): `atualizar_media_de_publico(liga)`, do laço das temporadas
/// (0x5d5ae0, em 0x5d5cfb: push liga; call; add esp, 4), no fim da temporada de cada liga da nação (a de cima e as de
/// baixo, pela +0x20). A média de público do clube (+0x73, Attendance) anda com o resultado da temporada, entre o
/// mínimo (+0x77) e o recorde (+0x7b).
/// O peso `f` vem da nação da competição (o primeiro dword de [+4]+0x5d): 0,1 em [0x9cf510], [0x9cf2dc], [0x9cf3e4],
/// [0x9cf224]; 0,25 em [0x9cf2c0], [0x9cf2fc], [0x9cf408], [0x9cf4a8], [0x9cf4f8]; 0,5 em [0x9cf45c]; 1 nas outras.
/// `r` = a reputação da competição (+0x69 do registro, word). A tabela final vem das estatísticas (Comp_Stats::
/// get_sorted_team_table 0x4a4b20, thiscall(+8, 1, &n, 1000), ret 0xc: registros de 0x76 bytes, +0 o id do clube,
/// +0x72 a posição prevista, +0x73 a final, +5 os jogos em casa [inferência], +6 o público somado [inferência]);
/// para cada clube (os ramos na ordem da original, com as mesmas chamadas às virtuais +0x98 "tem acesso" e +0x9c "tem
/// rebaixamento"; d = prevista − final + 1, word):
/// - campeão sem acesso: + (r + max(d, 0)) × 200 × f (dono do estádio, +0x6d) ou × 100 × f;
/// - 2º (ou posição 0) sem acesso: + (r + max(d, 0)) × 125 × f ou × 65 × f;
/// - subiu (posição ≤ +0xbe): pela liga de cima ([0xadadfc][+0x1c]; sem ela, nada): v = +0x80 do clube (a reputação,
///   word) × max(r de cima − r, 1) × f × 0,1, no mínimo 100; + v + sorteio(v) (dono) ou + v; e o piso pela reputação
///   da liga de cima;
/// - play-off de acesso (+0xbf ≠ 0 e posição ≤ +0xbf): com d − 1 > 0: + sorteio(r(d − 1) × 75 × f) + r(d − 1) × 75 × f
///   (dono) ou + sorteio(r(d − 1) × 25 × f) + r(d − 1) × 10 × f; senão + sorteio(r × 50 × f ou r × 25 × f); com
///   jogos em casa, a mistura com o público real: (público somado / jogos + 4 × média) × f × 0,2;
/// - caiu (posição > +0x3e − +0xc1): pela liga de baixo (sem ela, nada): v = +0x80 × min(r de baixo − r, −1) × f × 0,2,
///   no máximo −100 (sem rebaixamento na de baixo, +0x20 = −1: v = 2v − 250 − sorteio(750)); + v + sorteio(v); e o
///   teto de 2000 (r de baixo ≤ 7) ou 3000 vezes a r de baixo, × f;
/// - os outros: + sorteio(r × d × 5 × f), × 10 para o dono com d > 0 ou com +0x80 acima de 500 × r; a mistura com o
///   público real (sem o peso f); o piso pela reputação; e o teto de 2000 r (r ≤ 7), 3000 r (r ≤ 12) ou 5000 r.
/// No fim de cada um: no mínimo 500, e dentro de [+0x77, +0x7b]. A tabela é liberada (free 0x9452ca).
unsafe extern "C" fn cqt4_atualizar_media_de_publico(liga: usize) {
    let tabela_c = cqt4_d(0xadadfc);
    let cima = if cqt4_i(liga + 0x1c) == -1 { 0 } else { cqt4_d(tabela_c + 4 * cqt4_d(liga + 0x1c)) };
    let baixo = if cqt4_i(liga + 0x20) == -1 { 0 } else { cqt4_d(tabela_c + 4 * cqt4_d(liga + 0x20)) };
    let mut f = 1.0f64;
    let reg = cqt4_d(liga + 4);
    if reg != 0 && cqt4_d(reg + 0x5d) != 0 {
        let n = cqt4_d(cqt4_d(reg + 0x5d));
        let em = |gs: &[usize]| gs.iter().any(|&g| n == cqt4_d(g));
        if em(&[0x9cf510, 0x9cf2dc, 0x9cf3e4, 0x9cf224]) {
            f = f64::from_bits(0x3fb9_9999_9999_999a);
        } else if em(&[0x9cf2c0, 0x9cf2fc, 0x9cf408, 0x9cf4a8, 0x9cf4f8]) {
            f = 0.25;
        } else if n == cqt4_d(0x9cf45c) {
            f = 0.5;
        }
    }
    let ordenada: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32) -> usize = core::mem::transmute(0x4a4b20usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let acesso = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqt4_d(cqt4_d(liga) + 0x98));
        v(liga)
    };
    let rebaixamento = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqt4_d(cqt4_d(liga) + 0x9c));
        v(liga)
    };
    let mut n = 0i32;
    let t = ordenada(cqt4_d(liga + 8), 1, &mut n, 1000);
    if t == 0 {
        return;
    }
    let mut i = 0i32;
    while i < n {
        let e = t + 0x76 * i as usize;
        let c = cqt4_d(0xae23b4) + 0x245 * cqt4_d(e);
        let pos = *((e + 0x73) as *const u8);
        let prevista = *((e + 0x72) as *const u8);
        let dono = *((c + 0x6d) as *const u8) != 0;
        let somar = |v: i32| cqt4_pi(c + 0x73, cqt4_i(c + 0x73).wrapping_add(v));
        let d = (prevista as i16).wrapping_sub(pos as i16).wrapping_add(1);
        if pos == 1 && acesso() == 0 {
            let k = league_comp_reputation(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(200) } else { k.wrapping_mul(100) };
            somar(match_pl_ftol(val as f64 * f));
        } else if acesso() == 0 && pos < 3 {
            let k = league_comp_reputation(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(125) } else { k.wrapping_mul(65) };
            somar(match_pl_ftol(val as f64 * f));
        } else if acesso() != 0 && pos as i32 <= *((liga + 0xbe) as *const i8) as i32 {
            if cima != 0 {
                let mut a = cqt4_w(cqt4_d(cima + 4) + 0x69).wrapping_sub(cqt4_w(cqt4_d(liga + 4) + 0x69));
                if a < 1 {
                    a = 1;
                }
                let v = match_pl_ftol(cqt4_w(c + 0x80) as f64 * a as f64 * f * 0.1).max(100);
                if dono {
                    let r = sorteio(v);
                    somar(r.wrapping_add(v));
                } else {
                    somar(v);
                }
                raise_attendance_floor(c, league_comp_reputation(cima), f);
                crate::comparison::branch("cqt_4 attendance of promoted");
            }
        } else if *((liga + 0xbf) as *const i8) != 0 && pos as i32 <= *((liga + 0xbf) as *const i8) as i32 {
            let a = (prevista as i16).wrapping_sub(pos as i16);
            if a > 0 {
                let s = a as i32;
                if dono {
                    let v = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    let r = sorteio(v);
                    let w = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    somar(r.wrapping_add(w));
                } else {
                    let v = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(25) as f64 * f);
                    let r = sorteio(v);
                    let w = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(10) as f64 * f);
                    somar(r.wrapping_add(w));
                }
            } else {
                let r = league_comp_reputation(liga);
                let val = if dono { r.wrapping_mul(50) } else { r.wrapping_mul(25) };
                let v = match_pl_ftol(val as f64 * f);
                somar(sorteio(v));
            }
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (cqt4_i(e + 6) / jogos as i32).wrapping_add(cqt4_i(c + 0x73).wrapping_mul(4));
                cqt4_pi(c + 0x73, match_pl_ftol(val as f64 * f * 0.2));
            }
        } else if rebaixamento() != 0
            && pos as i32 > cqt4_w(liga + 0x3e) as i32 - *((liga + 0xc1) as *const i8) as i32
        {
            if baixo != 0 {
                let mut a = cqt4_w(cqt4_d(baixo + 4) + 0x69).wrapping_sub(cqt4_w(cqt4_d(liga + 4) + 0x69));
                if a > -1 {
                    a = -1;
                }
                let mut v = match_pl_ftol(cqt4_w(c + 0x80) as f64 * a as f64 * f * 0.2).min(-100);
                if cqt4_i(baixo + 0x20) == -1 {
                    let r = sorteio(0x2ee);
                    v = v.wrapping_mul(2).wrapping_sub(0xfa).wrapping_sub(r);
                }
                let r = sorteio(v);
                somar(r.wrapping_add(v));
                let rb = league_comp_reputation(baixo);
                let lim = if rb > 7 { match_pl_ftol(rb.wrapping_mul(3000) as f64 * f) } else { match_pl_ftol(rb.wrapping_mul(2000) as f64 * f) };
                cap_attendance(c, lim);
                crate::comparison::branch("cqt_4 attendance of relegated");
            }
        } else {
            let r16 = cqt4_w(cqt4_d(liga + 4) + 0x69);
            let r = r16 as i32;
            let val = if d <= 0 && cqt4_w(c + 0x80) <= r16.wrapping_mul(500) {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            } else if dono {
                r.wrapping_mul(d as i32).wrapping_mul(10)
            } else {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            };
            let v = match_pl_ftol(val as f64 * f);
            somar(sorteio(v));
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (cqt4_i(e + 6) / jogos as i32).wrapping_add(cqt4_i(c + 0x73).wrapping_mul(4));
                cqt4_pi(c + 0x73, match_pl_ftol(val as f64 * 0.2)); // cqt4: a mistura com o público real
            }
            crate::comparison::branch("cqt_4 attendance of mid-table");
            raise_attendance_floor(c, league_comp_reputation(liga), f);
            let r = league_comp_reputation(liga);
            let teto = if r > 12 { r.wrapping_mul(5000) } else if r > 7 { r.wrapping_mul(3000) } else { r.wrapping_mul(2000) };
            cap_attendance(c, teto);
        }
        if cqt4_i(c + 0x73) < 500 {
            cqt4_pi(c + 0x73, 500);
        }
        let a = cqt4_i(c + 0x73);
        if a < cqt4_i(c + 0x77) {
            cqt4_pi(c + 0x73, cqt4_i(c + 0x77));
        } else if a > cqt4_i(c + 0x7b) {
            cqt4_pi(c + 0x73, cqt4_i(c + 0x7b));
        }
        i += 1;
    }
    soltar(t);
}

/// O comparador da média de público: lado a lado (a cópia sem contar os sorteios, com o estado do sorteador
/// restaurado entre as duas) sobre a tabela dos clubes inteira (onde ficam +0x73, +0x77 e +0x7b) e o estado do
/// sorteador; a tabela final (0x4a4b20) é alocada e liberada nas duas.
unsafe extern "C" fn compare_cqt4_atualizar_media_de_publico(liga: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I3%%);
    let mut regioes = vec![(cqt4_d(0xae23b4), cqt4_d(0xae235c) * 0x245)];
    regioes.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&regioes, || o(liga), || crate::rng::without_counting(|| cqt4_atualizar_media_de_publico(liga)));
    SCOREBOARD.record(igual, || format!("cqt4_atualizar_media_de_publico({:#x}): o público dos clubes não bate", liga));
}
