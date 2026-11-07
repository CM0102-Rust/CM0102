// ---------- 283a leva: os construtores dos estaduais: Carioca, Catarinense e Paulista (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) e a média de público dos clubes no fim da temporada (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp, game.cpp) ----------

unsafe fn cqt4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt4Estadual {
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
unsafe fn cqt4_construir(this: usize, ano: u32, registro: usize, e: &Cqt4Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
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
unsafe fn cqt4_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt4Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt4_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt4_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt4_d(this + 8) != 0 {
        ok = ok && cqt4_d(this + 0x2c) == e.fases as usize && cqt4_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt4_d(this), cqt4_d(this + 0x2c), cqt4_d(this + 0xc), cqt4_d(this + 8)));
    r
}

const CQT4_CARIOCA: Cqt4Estadual = Cqt4Estadual {
    vtabela: 0x968608,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x43a500,
    previstas: 0x43add0,
    arquivo: 0x98b014,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const CQT4_CATARINENSE: Cqt4Estadual = Cqt4Estadual {
    vtabela: 0x9686bc,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x43bb70,
    previstas: 0x43c110,
    arquivo: 0x98b050,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const CQT4_PAULISTA: Cqt4Estadual = Cqt4Estadual {
    vtabela: 0x968770,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x43cc40,
    previstas: 0x43d140,
    arquivo: 0x98b090,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x439a40 } (thiscall, ret 8, bra_reg_rio.cpp): `BraRegRio::BraRegRio(ano, registro)`: o Carioca (três fases além da liga).
unsafe extern "thiscall" fn cqt4_construir_carioca(this: usize, ano: u32, registro: usize) -> usize {
    cqt4_construir(this, ano, registro, &CQT4_CARIOCA, 7, true, "cqt_4 carioca novo") // cqt4: a disciplina e as posições previstas do Carioca
}

/// { 0x43b130 } (thiscall, ret 8, bra_reg_santa.cpp): `BraRegSanta::BraRegSanta(ano, registro)`: o Catarinense (uma fase além da liga).
unsafe extern "thiscall" fn cqt4_construir_catarinense(this: usize, ano: u32, registro: usize) -> usize {
    cqt4_construir(this, ano, registro, &CQT4_CATARINENSE, 7, true, "cqt_4 catarinense novo") // cqt4: a disciplina e as posições previstas do Catarinense
}

/// { 0x43c280 } (thiscall, ret 8, bra_reg_sp.cpp): `BraRegSp::BraRegSp(ano, registro)`: o Paulista (uma fase além da liga).
unsafe extern "thiscall" fn cqt4_construir_paulista(this: usize, ano: u32, registro: usize) -> usize {
    cqt4_construir(this, ano, registro, &CQT4_PAULISTA, 7, true, "cqt_4 paulista novo") // cqt4: a disciplina e as posições previstas do Paulista
}

unsafe extern "thiscall" fn comparar_cqt4_construir_carioca(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2055);
    cqt4_comparar("cqt4_construir_carioca", this, ano, registro, &CQT4_CARIOCA, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt4_construir_catarinense(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2056);
    cqt4_comparar("cqt4_construir_catarinense", this, ano, registro, &CQT4_CATARINENSE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt4_construir_paulista(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2057);
    cqt4_comparar("cqt4_construir_paulista", this, ano, registro, &CQT4_PAULISTA, || o(this, ano, registro))
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
unsafe fn cqt4_rep(liga: usize) -> i32 {
    cqt4_w(cqt4_d(liga + 4) + 0x69) as i32
}

/// O piso que sobe com a reputação `r` (dos ramos que puxam o público para cima): com r < 5, 250 × r × f; senão
/// 750 × (r − 4) × f + 1000 (a original compara com a parte inteira + 1000 e grava a conta com 1000 somado antes do
/// truncamento). Abaixo dele, o público vai a ele e o recorde (+0x7b) sobe até ele.
unsafe fn cqt4_piso_pela_reputacao(c: usize, r: i32, f: f64) {
    let lim = if r < 5 {
        let lim = ftol((r.wrapping_mul(250)) as f64 * f);
        if cqt4_i(c + 0x73) >= lim {
            return;
        }
        lim
    } else {
        let x = (r.wrapping_sub(4).wrapping_mul(750)) as f64 * f;
        if cqt4_i(c + 0x73) >= ftol(x).wrapping_add(1000) {
            return;
        }
        ftol(x + 1000.0)
    };
    cqt4_pi(c + 0x73, lim);
    if lim > cqt4_i(c + 0x7b) {
        cqt4_pi(c + 0x7b, lim);
    }
}

/// O teto `lim` do público: acima dele, o público cai a ele e o mínimo (+0x77) desce até ele.
unsafe fn cqt4_teto(c: usize, lim: i32) {
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
            let k = cqt4_rep(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(200) } else { k.wrapping_mul(100) };
            somar(ftol(val as f64 * f));
        } else if acesso() == 0 && pos < 3 {
            let k = cqt4_rep(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(125) } else { k.wrapping_mul(65) };
            somar(ftol(val as f64 * f));
        } else if acesso() != 0 && pos as i32 <= *((liga + 0xbe) as *const i8) as i32 {
            if cima != 0 {
                let mut a = cqt4_w(cqt4_d(cima + 4) + 0x69).wrapping_sub(cqt4_w(cqt4_d(liga + 4) + 0x69));
                if a < 1 {
                    a = 1;
                }
                let v = ftol(cqt4_w(c + 0x80) as f64 * a as f64 * f * 0.1).max(100);
                if dono {
                    let r = sorteio(v);
                    somar(r.wrapping_add(v));
                } else {
                    somar(v);
                }
                cqt4_piso_pela_reputacao(c, cqt4_rep(cima), f);
                crate::comparacao::ramo("cqt_4 publico de quem subiu");
            }
        } else if *((liga + 0xbf) as *const i8) != 0 && pos as i32 <= *((liga + 0xbf) as *const i8) as i32 {
            let a = (prevista as i16).wrapping_sub(pos as i16);
            if a > 0 {
                let s = a as i32;
                if dono {
                    let v = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    let r = sorteio(v);
                    let w = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    somar(r.wrapping_add(w));
                } else {
                    let v = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(25) as f64 * f);
                    let r = sorteio(v);
                    let w = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(10) as f64 * f);
                    somar(r.wrapping_add(w));
                }
            } else {
                let r = cqt4_rep(liga);
                let val = if dono { r.wrapping_mul(50) } else { r.wrapping_mul(25) };
                let v = ftol(val as f64 * f);
                somar(sorteio(v));
            }
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (cqt4_i(e + 6) / jogos as i32).wrapping_add(cqt4_i(c + 0x73).wrapping_mul(4));
                cqt4_pi(c + 0x73, ftol(val as f64 * f * 0.2));
            }
        } else if rebaixamento() != 0
            && pos as i32 > cqt4_w(liga + 0x3e) as i32 - *((liga + 0xc1) as *const i8) as i32
        {
            if baixo != 0 {
                let mut a = cqt4_w(cqt4_d(baixo + 4) + 0x69).wrapping_sub(cqt4_w(cqt4_d(liga + 4) + 0x69));
                if a > -1 {
                    a = -1;
                }
                let mut v = ftol(cqt4_w(c + 0x80) as f64 * a as f64 * f * 0.2).min(-100);
                if cqt4_i(baixo + 0x20) == -1 {
                    let r = sorteio(0x2ee);
                    v = v.wrapping_mul(2).wrapping_sub(0xfa).wrapping_sub(r);
                }
                let r = sorteio(v);
                somar(r.wrapping_add(v));
                let rb = cqt4_rep(baixo);
                let lim = if rb > 7 { ftol(rb.wrapping_mul(3000) as f64 * f) } else { ftol(rb.wrapping_mul(2000) as f64 * f) };
                cqt4_teto(c, lim);
                crate::comparacao::ramo("cqt_4 publico de quem caiu");
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
            let v = ftol(val as f64 * f);
            somar(sorteio(v));
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (cqt4_i(e + 6) / jogos as i32).wrapping_add(cqt4_i(c + 0x73).wrapping_mul(4));
                cqt4_pi(c + 0x73, ftol(val as f64 * 0.2)); // cqt4: a mistura com o público real
            }
            crate::comparacao::ramo("cqt_4 publico do meio da tabela");
            cqt4_piso_pela_reputacao(c, cqt4_rep(liga), f);
            let r = cqt4_rep(liga);
            let teto = if r > 12 { r.wrapping_mul(5000) } else if r > 7 { r.wrapping_mul(3000) } else { r.wrapping_mul(2000) };
            cqt4_teto(c, teto);
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
unsafe extern "C" fn comparar_cqt4_atualizar_media_de_publico(liga: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(2058);
    let mut regioes = vec![(cqt4_d(0xae23b4), cqt4_d(0xae235c) * 0x245)];
    regioes.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(liga), || crate::aleatorio::sem_contar(|| cqt4_atualizar_media_de_publico(liga)));
    PLACAR.registrar(igual, || format!("cqt4_atualizar_media_de_publico({:#x}): o público dos clubes não bate", liga));
}

// ---------- 282a leva: os turnos do Carioca, do Mineiro, do Paranaense e da J1: a fase de pontos corridos de cada turno (bra_reg_rio.cpp, bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, jap_j1.cpp) ----------

unsafe fn cqr1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqr1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O turno `k` de uma liga de dois turnos (o slot create_half_season_stage; o mesmo molde nos três estaduais e na J1,
/// conferido instrução a instrução: só mudam o fonte, a linha da caixa e o 20º argumento do construtor, 0xb nos
/// estaduais e 0xf na J1). Chamado do resultado de cada jogo (0x43af20 e as irmãs: push dl = +0x30 + 1; mov ecx ->
/// thiscall, ret 4; a original lê o argumento inteiro, usa o byte baixo e o empurra inteiro ao construtor).
/// 1) Os clubes da tabela da liga (+0xb1, linhas de 0x3b bytes, as +0x3e linhas) num vetor (malloc 0x944e46 de
///    4 × +0x3e; sem memória: a caixa e volta).
/// 2) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, 1, +4, 0, 0, os pontos por vitória +0xc2 (`pv`), por empate +0xc3, a
///    categoria dos jogos +0xc4, &+0xc5 (os critérios de desempate), &{1,0,0,0} (uma vaga de cima: o campeão do
///    turno), o ano +0x40 (word), k, o id 0x439 (primeiro turno) ou 0x43a (segundo), 0, 1, 0, 8, `vinte`, 0, 2; a
///    original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores
///    limpos; o k vai inteiro, como chegou). Sem memória para a fase: 0 na vaga, sem caixa.
/// 3) [+0xc][k] = a fase, +0x30 = k (o byte com sinal) e o vetor liberado (free 0x9452ca). O quadro SEH da original só
///    cobre o construtor (sem destrutor no caminho normal).
unsafe fn cqr1_turno(this: usize, k: u32, arquivo: usize, linha: u32, vinte: u32, pv: usize, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, u32, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let vagas: [u8; 4] = [1, 0, 0, 0];
    let n = cqr1_w(this + 0x3e);
    let clubes = alocar(((n as i32) << 2) as u32);
    if clubes == 0 {
        return aviso_em(arquivo, linha);
    }
    let mut i: i16 = 0;
    while i < cqr1_w(this + 0x3e) {
        core::ptr::write_unaligned((clubes + 4 * i as usize) as *mut usize, cqr1_d(cqr1_d(this + 0xb1) + i as usize * 0x3b));
        i += 1;
    }
    let id = 0x439 + (k as u8 != 0) as u32;
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, cqr1_w(this + 0x3e) as u16 as u32, clubes, 1, cqr1_d(this + 4), 0, 0,
                  cqr1_b(this + pv) as u32,
                  cqr1_b(this + 0xc3) as u32, cqr1_b(this + 0xc4) as u32, this + 0xc5, vagas.as_ptr(),
                  cqr1_w(this + 0x40) as u16 as u32, k, id, 0, 1, 0, 8, vinte, 0, 2)
    } else {
        0
    };
    let k = k as u8 as i8 as i32;
    core::ptr::write_unaligned((cqr1_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
    crate::comparacao::ramo(ramo);
    soltar(clubes);
}

/// O comparador de um turno: só a original (ponytail: aloca o vetor e a fase, que monta a tabela do turno), com o plano:
/// depois, a vaga k tem a fase (o operator new não falha na carreira), +0x30 = k e a fase tem os +0x3e clubes da liga.
unsafe fn cqr1_comparar(nome: &str, this: usize, k: u32, original: impl FnOnce()) {
    original();
    let kk = k as u8 as i8 as i32;
    let fase = cqr1_d(cqr1_d(this + 0xc) + 4 * kk as usize);
    let igual = fase != 0 && cqr1_d(this + 0x30) as i32 == kk && cqr1_w(fase + 0x3e) == cqr1_w(this + 0x3e);
    PLACAR.registrar(igual, || format!("{}({:#x}, {}) fase={:#x} +0x30={}", nome, this, kk, fase, cqr1_d(this + 0x30) as i32));
}

/// { 0x43afa0 } (thiscall, ret 4, bra_reg_rio.cpp): `BraRegRio::create_half_season_stage(k)`, um turno do Carioca (os
/// doze clubes; a caixa da linha 0x3d5).
unsafe extern "thiscall" fn cqr1_turno_do_carioca(this: usize, k: u32) {
    cqr1_turno(this, k, 0x98b014, 0x3d5, 0xb, 0xc2, "cqr_1 turno do carioca") // cqr1: os pontos por vitória no Carioca
}

/// { 0x4351b0 } (thiscall, ret 4, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_half_season_stage(k)`, um turno
/// do Mineiro (a caixa da linha 0x3d5).
unsafe extern "thiscall" fn cqr1_turno_do_mineiro(this: usize, k: u32) {
    cqr1_turno(this, k, 0x98aed0, 0x3d5, 0xb, 0xc2, "cqr_1 turno do mineiro") // cqr1: os pontos por vitória no Mineiro
}

/// { 0x438760 } (thiscall, ret 4, bra_reg_parana.cpp): `BraRegParana::create_half_stage(k)`, um turno do Paranaense (a
/// caixa da linha 0x3d5).
unsafe extern "thiscall" fn cqr1_turno_do_paranaense(this: usize, k: u32) {
    cqr1_turno(this, k, 0x98af98, 0x3d5, 0xb, 0xc2, "cqr_1 turno do paranaense") // cqr1: os pontos por vitória no Paranaense
}

/// { 0x663c20 } (thiscall, ret 4, jap_j1.cpp): `JapJ1::create_half_season_stage(k)`, um turno da J1 (a caixa da linha
/// 0x278; o 20º argumento do construtor é 0xf).
unsafe extern "thiscall" fn cqr1_turno_da_j1(this: usize, k: u32) {
    cqr1_turno(this, k, 0x9eabc8, 0x278, 0xf, 0xc2, "cqr_1 turno da j1") // cqr1: os pontos por vitória na J1
}

unsafe extern "thiscall" fn comparar_cqr1_turno_do_carioca(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1986);
    cqr1_comparar("cqr1_turno_do_carioca", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_cqr1_turno_do_mineiro(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1987);
    cqr1_comparar("cqr1_turno_do_mineiro", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_cqr1_turno_do_paranaense(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1988);
    cqr1_comparar("cqr1_turno_do_paranaense", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_cqr1_turno_da_j1(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1989);
    cqr1_comparar("cqr1_turno_da_j1", this, k, || o(this, k))
}

// ---------- 282a leva: a virada da temporada do Carioca, do Mineiro e do Paranaense (bra_reg_rio.cpp, bra_reg_minas_gerais.cpp, bra_reg_parana.cpp) ----------

unsafe fn cqr2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A virada da temporada de um estadual de dois turnos e final (`start_new_season`, slot 3, +0x08; o mesmo molde no
/// Carioca, no Mineiro e no Paranaense: só mudam o fonte e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o vetor das fases liberado (free) e trocado por um novo de três vagas (+0x2c = 3, malloc 0x944e46 de 0xc, sem
/// conferir nem zerar: os turnos e a final o preenchem); o ano (+0x40, word) sobe `ano` (o contador `ramo`) e
/// +0x30 = −1; então, cada um com a sua caixa (linhas 0x1d4, 0x1db, 0x1e1 e 0x1e8) se devolver 0: as regras da
/// temporada (virtual +0x8c), a tabela com os doze clubes (`tabela`, thiscall, ret 0), os jogos (League::create_fixtures
/// 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as
/// posições previstas (virtual +0x5c) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0,
/// thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com caixa, a cópia devolve 0 (a original devolve o eax da
/// caixa, que a virada do Brasil não lê).
unsafe fn cqr2_virada(this: usize, tabela: usize, arquivo: usize, ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqr2_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqr2_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqr2_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqr2_d(cqr2_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqr2_d(cqr2_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqr2_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    liberar(cqr2_d(this + 0xc));
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, 3);
    let v = alocar(0xc);
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqr2_d(cqr2_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, 0x1d4);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, 0x1db);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, 0x1e1);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, 0x1e8);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqr2_d(cqr2_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqr2_d(0xb74340), cqr2_d(this + 4))
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um, +0x2c = 3 e +0x30 = −1.
unsafe fn cqr2_comparar(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let igual = depois == antes.wrapping_add(1) && cqr2_d(this + 0x2c) == 3 && cqr2_d(this + 0x30) as i32 == -1;
    PLACAR.registrar(igual, || format!("{}({:#x}) ano antes={} depois={} +0x2c={} +0x30={}", nome, this, antes, depois, cqr2_d(this + 0x2c), cqr2_d(this + 0x30) as i32));
    r
}

/// { 0x43a6a0 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::start_new_season()`, a virada do Carioca (a tabela: 0x43a500).
unsafe extern "thiscall" fn cqr2_virada_do_carioca(this: usize) -> u32 {
    cqr2_virada(this, 0x43a500, 0x98b014, 1, "cqr_2 virada do carioca") // cqr2: o ano seguinte do Carioca
}

/// { 0x434910 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::start_new_season()`, a virada do Mineiro (a tabela: 0x434810).
unsafe extern "thiscall" fn cqr2_virada_do_mineiro(this: usize) -> u32 {
    cqr2_virada(this, 0x434810, 0x98aed0, 1, "cqr_2 virada do mineiro") // cqr2: o ano seguinte do Mineiro
}

/// { 0x438060 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::start_new_season()`, a virada do Paranaense (a tabela: 0x437ec0).
unsafe extern "thiscall" fn cqr2_virada_do_paranaense(this: usize) -> u32 {
    cqr2_virada(this, 0x437ec0, 0x98af98, 1, "cqr_2 virada do paranaense") // cqr2: o ano seguinte do Paranaense
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_carioca(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1990);
    cqr2_comparar("cqr2_virada_do_carioca", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_mineiro(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1991);
    cqr2_comparar("cqr2_virada_do_mineiro", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_paranaense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1992);
    cqr2_comparar("cqr2_virada_do_paranaense", this, || o(this))
}

// ---------- 279a leva: o Carioca: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_rio.cpp) ----------

unsafe fn cqo9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqo9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqo9_campeao_do_turno(this: usize, k: usize) -> usize {
    cqo9_d(cqo9_d(cqo9_d(cqo9_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x43a870 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::advance_stage()`, a fase seguinte do Carioca (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x43a900, call direto com o ecx da liga).
unsafe extern "thiscall" fn cqo9_fase_seguinte_do_carioca(this: usize) {
    let fase = cqo9_d(this + 0x30) as i32;
    if fase >= (cqo9_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x98b014, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x43a900usize);
        final_(this); // cqo9: a final depois do segundo turno
    }
}

/// { 0x43a900 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe extern "thiscall" fn cqo9_final_do_carioca(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x98b014, 0x226);
    }
    let a = cqo9_campeao_do_turno(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = cqo9_campeao_do_turno(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_9 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((cqo9_d(cqo9_d(cqo9_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo9: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < cqo9_w(this + 0x3e) {
            let linha = cqo9_d(this + 0xb1) + i as usize * 0x3b;
            if cqo9_d(linha) == cqo9_d(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqo9_d(cqo9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqo9_d(this + 4), t, cqo9_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((cqo9_d(this + 0xc) + 8) as *mut usize, fase);
    let fase = cqo9_d(cqo9_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0x98b014, 0x250);
    }
    crate::comparacao::ramo("cqo_9 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x43ad20 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::record_season_honours()` (slot 13), o fim da temporada do Carioca.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 675.000 ao campeão (primeira linha da tabela +0xb1) e 250.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
unsafe extern "thiscall" fn cqo9_fim_da_temporada_do_carioca(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    if cqo9_d(this + 0x30) == 2 {
        let f = cqo9_d(cqo9_d(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqo9_d(cqo9_d(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_9 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684640usize);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    for (linha, valor, posicao) in [(0usize, 675000i32, 1i32), (0x3b, 250000, 2)] {
        let c = cqo9_d(cqo9_d(this + 0xb1) + linha);
        premio(financas(cqo9_d(0xae2a38), c), valor);
        noticia(this, cqo9_d(cqo9_d(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_9 premios");
    campeoes()
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
unsafe extern "thiscall" fn comparar_cqo9_fase_seguinte_do_carioca(this: usize) {
    let antes = cqo9_d(this + 0x30) as i32;
    let ultima = antes >= (cqo9_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1905)(this);
    let depois = cqo9_d(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("cqo9_fase_seguinte_do_carioca({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqo9_final_do_carioca(this: usize) {
    let (a, b) = (cqo9_campeao_do_turno(this, 0), cqo9_campeao_do_turno(this, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(1906)(this);
    let igual = if a == b {
        let marca = |k: usize| *((cqo9_d(cqo9_d(cqo9_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..cqo9_w(this + 0x3e).max(0) as usize).all(|i| {
            let linha = cqo9_d(this + 0xb1) + i * 0x3b;
            cqo9_d(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        cqo9_d(this + 0x30) == 1 && cqo9_d(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = cqo9_d(cqo9_d(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = cqo9_d(fase + 0xa7);
            let (x, y) = (cqo9_d(l), cqo9_d(l + 6));
            cqo9_w(fase + 0x3e) == 2 && cqo9_d(fase + 4) == cqo9_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqo9_d(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("cqo9_final_do_carioca({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, cqo9_d(this + 0x30), cqo9_d(this + 0x2c)));
}

/// Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_9 historico da final" e "cqo_9 premios") e é provada pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqo9_fim_da_temporada_do_carioca(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1907)(this)
}

// ---------- 277a leva: a tabela dos estaduais: Carioca, Catarinense e Paulista (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqm8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqm8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cqm8_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cqm8_d(clube + 0x60);
    comp != 0 && cqm8_d(comp) == cqm8_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn cqm8_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
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
    let mut clube = cqm8_d(0xae23b4);
    while (i as i32) < cqm8_d(0xae235c) as i32 {
        if cqm8_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqm8_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqm8_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x43a500 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_table()`: o Carioca tem 12 clubes (divisão reserva +0x60 = [0x9cf9dc]).
/// Fonte 0x98b014, linha 0x169.
unsafe extern "thiscall" fn cqm8_tabela_do_carioca(this: usize) -> u32 {
    cqm8_tabela(this, 0xc, 0x98b014, 0x169, 0x9cf9dc, 0, "cqm_8 carioca cheio")
}

/// { 0x43bb70 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_table()`: o Catarinense tem 10 clubes (divisão reserva +0x60 = [0x9cf9b0]).
/// Fonte 0x98b050, linha 0x167.
unsafe extern "thiscall" fn cqm8_tabela_do_catarinense(this: usize) -> u32 {
    cqm8_tabela(this, 0xa, 0x98b050, 0x167, 0x9cf9b0, 0, "cqm_8 catarinense cheio")
}

/// { 0x43cc40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_table()`: o Paulista tem 16 clubes (divisão reserva +0x60 = [0x9cf9e4]).
/// Fonte 0x98b090, linha 0x159.
unsafe extern "thiscall" fn cqm8_tabela_do_paulista(this: usize) -> u32 {
    cqm8_tabela(this, 0x10, 0x98b090, 0x159, 0x9cf9e4, 0, "cqm_8 paulista cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cqm8_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqm8_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqm8_d(0xae23b4) + (i as usize) * 0x245;
        if cqm8_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqm8_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cqm8_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqm8_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cqm8_tabela_do_carioca(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1837);
    cqm8_comparar("cqm8_tabela_do_carioca", this, 0xc, 0x9cf9dc, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm8_tabela_do_catarinense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1838);
    cqm8_comparar("cqm8_tabela_do_catarinense", this, 0xa, 0x9cf9b0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm8_tabela_do_paulista(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1839);
    cqm8_comparar("cqm8_tabela_do_paulista", this, 0x10, 0x9cf9e4, || o(this))
}

