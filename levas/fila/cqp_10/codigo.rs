// ---------- %%LEVA%%a leva: o calendário de mais copas de clubes: a Copa dos Campeões, a Merconorte e as Supercopas da Europa e da Ásia (bra_champ_cup.cpp, con_merc_cup.cpp, eur_super_cup.cpp, asia_super_cup.cpp) ----------

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
type Cqp10G = (u32, u32, u32, u32);

/// O que a virtual +0x3c monta numa fase: nada (devolve 0 sem gravar), uma fase de grupos (as rodadas, o extra, a linha
/// da caixa) ou o mata-mata (as rodadas, as palavras a mais (rodada, deslocamento, valor), a linha da caixa).
#[derive(Clone, Copy)]
enum Cqp10F {
    Nada,
    Grupos(&'static [Cqp10G], i16, u32, &'static str),
    Mata(&'static [Cql10R], &'static [(usize, usize, u16)], u32, &'static str),
}

/// A virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10) pela fase `f` escolhida. Nos
/// grupos: é_liga (se pedido) = 1, rodadas, extra; a tabela de 0x41 bytes por rodada (0x944e46; sem memória, a caixa e
/// 0) com as datas, o ano +0x40 em 16 bits. No mata-mata: é_liga = 0, rodadas, extra = 0; a tabela de 0x68 bytes por
/// rodada (sem memória, a caixa e 0), as rodadas pela `cql10_rodada` com o ano +0x40 com sinal e as palavras a mais.
unsafe fn cqp10_tabela(this: usize, f: Cqp10F, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    match f {
        Cqp10F::Nada => 0,
        Cqp10F::Grupos(grupos, id, linha, nome) => {
            let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
            let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = grupos.len() as i16;
            *extra = id;
            let t = alocar(grupos.len() as u32 * 0x41);
            if t == 0 {
                aviso_em(arquivo, linha);
                return 0;
            }
            crate::comparacao::ramo(nome);
            // o ano vai com lixo nos bytes altos na original (o registrador de antes com a palavra +0x40 por cima); a
            // 0x68a160 só usa os 16 bits de baixo: a cópia passa o ano limpo
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
                data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, ano, 0);
                jogo(t, r as i32, 0, -1, -1, -1, 0);
            }
            t
        }
        Cqp10F::Mata(mata, a_mais, linha, nome) => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = mata.len() as i16;
            *extra = 0;
            let t = alocar(mata.len() as u32 * 0x68);
            if t == 0 {
                aviso_em(arquivo, linha);
                return 0;
            }
            crate::comparacao::ramo(nome);
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            for (i, r) in mata.iter().enumerate() {
                cql10_rodada(t, i as u32, r, ano);
            }
            for &(r, o, v) in a_mais {
                core::ptr::write_unaligned((t + r * 0x68 + o) as *mut u16, v);
            }
            t
        }
    }
}

/// Os bytes que a original grava na tabela de `n` rodadas da fase `f`: nos grupos (0x41 bytes), +0..+0xb e
/// +0x3d..+0x40 de cada rodada; no mata-mata, os da cql10_gravados e as palavras a mais.
unsafe fn cqp10_gravados(f: Cqp10F, t: usize, n: i16) -> Vec<u8> {
    match f {
        Cqp10F::Nada => Vec::new(),
        Cqp10F::Grupos(..) => {
            let mut v = Vec::new();
            for r in 0..n.max(0) as usize {
                let e = t + r * 0x41;
                v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
                v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
            }
            v
        }
        Cqp10F::Mata(_, a_mais, ..) => {
            let mut v = cql10_gravados(t, n);
            for &(r, o, _) in a_mais {
                v.extend_from_slice(core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2));
            }
            v
        }
    }
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn cqp10_comparar(nome: &str, fase: u32, f: Cqp10F, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if let Cqp10F::Nada = f {
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqp10_gravados(f, a, *rodadas) == cqp10_gravados(f, b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa dos Campeões, a fase preliminar (fase −1): três rodadas às segundas [inferência: dia da semana 1], 19 e 26 de
/// junho e 3 de julho.
const CQP10_CAMPEOES_PRELIMINAR: [Cqp10G; 3] = [(0x13, 5, 1, 2), (0x1a, 5, 1, 2), (3, 6, 1, 2)]; // cqp10: a preliminar da Copa dos Campeões

/// A Copa dos Campeões, os grupos (fases 0 e 1): três rodadas, 9, 16 e 20 de julho.
const CQP10_CAMPEOES_GRUPOS: [Cqp10G; 3] = [(9, 6, 0, 2), (0x10, 6, 0, 2), (0x14, 6, 4, 2)];

/// A Copa dos Campeões, o mata-mata (fase 2 em diante): semifinal e final em ida e volta no fim de julho.
const CQP10_CAMPEOES_MATA: [Cql10R; 2] = [
    cql10r((0x15, 6, 0, 5), (0x17, 6, 0, 0, 2, 0), 0x82, (0, 0x200, 1), 6, (4, 2, 4, 0), (0, 2, 2), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 6, 0, 3), (0x1b, 6, 0, 4, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 3), (0, 0xc3500, 0x61a80)),
];

/// A fase da Copa dos Campeões pelo byte com sinal: −1 a preliminar (extra 0xe6, caixa 0xaa), abaixo de 2 os grupos
/// (extra = fase + 0x3fd, caixa 0xcd), 2 em diante o mata-mata (caixa 0xed).
fn cqp10_campeoes_fase(fase: u32) -> Cqp10F {
    let f = fase as u8 as i8;
    if f == -1 {
        Cqp10F::Grupos(&CQP10_CAMPEOES_PRELIMINAR, 0xe6, 0xaa, "cqp_10 preliminar da copa dos campeoes")
    } else if f < 2 {
        Cqp10F::Grupos(&CQP10_CAMPEOES_GRUPOS, (f as i16).wrapping_add(0x3fd), 0xcd, "cqp_10 grupos da copa dos campeoes")
    } else {
        Cqp10F::Mata(&CQP10_CAMPEOES_MATA, &[], 0xed, "cqp_10 mata-mata da copa dos campeoes")
    }
}

/// { 0x425c10 } (thiscall, ret 0x10, bra_champ_cup.cpp; a virtual +0x3c da Copa dos Campeões): `BraChampCup::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a preliminar, os grupos ou o mata-mata (`cqp10_campeoes_fase`).
unsafe extern "thiscall" fn cqp10_copa_dos_campeoes_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp10_tabela(this, cqp10_campeoes_fase(fase), rodadas, extra, liga, 0x98a1e8)
}

unsafe extern "thiscall" fn comparar_cqp10_copa_dos_campeoes_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cqp10_comparar("cqp10_copa_dos_campeoes_tabela", fase, cqp10_campeoes_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp10_copa_dos_campeoes_tabela(this, fase, r, e, l))
}

/// A Merconorte, grupos (fases abaixo de 3): seis rodadas às terças de 15 em 15 dias, de 9 de setembro a 18 de novembro.
const CQP10_MERCONORTE_GRUPOS: [Cqp10G; 6] = [(9, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // cqp10: os grupos da Merconorte

/// A Merconorte, o mata-mata (fase 3): semifinal em ida e volta (7 dias) e final em jogo único em dezembro.
const CQP10_MERCONORTE_MATA: [Cql10R; 2] = [
    cql10r((0x13, 0xa, 0, 3), (0x19, 0xa, 0, 2, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 4, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xb, 0, 3), (9, 0xb, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// A fase da Merconorte: abaixo de 3 os grupos (extra = fase + 0x3e9, caixa 0x9b), 3 o mata-mata (caixa 0xc1), acima
/// nada.
fn cqp10_merconorte_fase(fase: u32) -> Cqp10F {
    let f = fase as u8 as i8;
    if f < 3 {
        Cqp10F::Grupos(&CQP10_MERCONORTE_GRUPOS, (f as i16).wrapping_add(0x3e9), 0x9b, "cqp_10 grupos da merconorte")
    } else if f == 3 {
        Cqp10F::Mata(&CQP10_MERCONORTE_MATA, &[], 0xc1, "cqp_10 mata-mata da merconorte")
    } else {
        Cqp10F::Nada
    }
}

/// { 0x4c1960 } (thiscall, ret 0x10, con_merc_cup.cpp; a virtual +0x3c da Merconorte): `ConMercCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: os grupos ou o mata-mata (`cqp10_merconorte_fase`).
unsafe extern "thiscall" fn cqp10_merconorte_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp10_tabela(this, cqp10_merconorte_fase(fase), rodadas, extra, liga, 0x9a7d6c)
}

unsafe extern "thiscall" fn comparar_cqp10_merconorte_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    cqp10_comparar("cqp10_merconorte_tabela", fase, cqp10_merconorte_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp10_merconorte_tabela(this, fase, r, e, l))
}

/// A Supercopa Europeia: jogo único em 24 de agosto (sorteio em 4 de julho) em campo neutro (mando 0x8000), sem prêmios.
const CQP10_SUPERCOPA_EUROPEIA: [Cql10R; 1] = [
    cql10r((4, 6, 0, 5), (0x18, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp10: a Supercopa Europeia
];

/// { 0x57a3c0 } (thiscall, ret 0x10, eur_super_cup.cpp; a virtual +0x3c da Supercopa Europeia): `EurSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
unsafe extern "thiscall" fn cqp10_supercopa_europeia_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP10_SUPERCOPA_EUROPEIA, 0x9c45f8, 0x98, "cqp_10 tabela da supercopa europeia")
}

unsafe extern "thiscall" fn comparar_cqp10_supercopa_europeia_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    cql10_comparar("cqp10_supercopa_europeia_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp10_supercopa_europeia_tabela(this, fase, r, e, s))
}

/// A Supercopa Asiática: ida e volta (14 dias entre os jogos) a partir de 4 de agosto, sorteio em 4 de julho, sem prêmios.
const CQP10_SUPERCOPA_ASIATICA: [Cql10R; 1] = [
    cql10r((4, 6, 0, 5), (4, 7, 0, 5, 2, 0), 0, (0, 0, 0x83), 6, (2, 1, 2, 0), (0, 2, 0xe), (0, 0, 0)),
];

/// { 0x410910 } (thiscall, ret 0x10, asia_super_cup.cpp; a virtual +0x3c da Supercopa Asiática): `AsiaSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x97 sem memória.
unsafe extern "thiscall" fn cqp10_supercopa_asiatica_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP10_SUPERCOPA_ASIATICA, 0x987be0, 0x97, "cqp_10 tabela da supercopa asiatica")
}

unsafe extern "thiscall" fn comparar_cqp10_supercopa_asiatica_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I3%%);
    cql10_comparar("cqp10_supercopa_asiatica_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp10_supercopa_asiatica_tabela(this, fase, r, e, s))
}
