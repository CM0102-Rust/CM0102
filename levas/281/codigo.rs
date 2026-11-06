// ---------- 281a leva: o fim da temporada das ligas: a cadeia de acessos e rebaixamentos; as notícias do prêmio e da TV de uma competição (league.cpp, comp.cpp) ----------

unsafe fn cqp1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// O slot 0xa4 da vtable da liga (thiscall, sem argumentos) [inferência: fecha a classificação final da temporada e
/// marca os que sobem e descem (+0x37 dos clubes), que a 0x689c80 lê].
unsafe fn cqp1_fechar(liga: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqp1_d(cqp1_d(liga) + 0xa4));
    f(liga)
}

/// { 0x689c20 } (thiscall, ret 4, league.cpp, `League::process_promotion_relegation_chain(aos_pares)`; pelo slot do
/// fim de temporada da liga): só na liga do topo da pirâmide (+0x1c, a liga de cima, negativo): fecha a temporada
/// dela (slot 0xa4) e, descendo a cadeia (+0x20 o índice da liga de baixo na tabela [0xadadfc], até negativo), fecha a
/// de baixo e troca os que sobem e descem entre as duas (0x689c80 com contratos = 1, `aos_pares`, sem contagem
/// esperada: −1, −1; o ecx é o desta liga).
unsafe extern "thiscall" fn cadeia_de_acessos(this: usize, aos_pares: u32) {
    if (cqp1_d(this + 0x1c) as i32) >= 0 {
        return;
    }
    crate::comparacao::ramo("cqp_1 cadeia");
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x689c80usize);
    let mut acima = this;
    cqp1_fechar(this); // cqp1: fecha a liga do topo
    let mut i = cqp1_d(this + 0x20) as i32;
    while i >= 0 {
        let abaixo = cqp1_d(cqp1_d(0xadadfc) + i as usize * 4);
        cqp1_fechar(abaixo); // cqp1: fecha a de baixo
        trocar(this, acima, abaixo, 1, aos_pares, u32::MAX, u32::MAX); // cqp1: sobem e descem
        i = cqp1_d(abaixo + 0x20) as i32;
        acima = abaixo;
    }
}

// ponytail: fecha temporadas e move clubes entre as ligas (contratos, técnicos, notícias): não roda duas vezes; só a
// original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_cadeia_de_acessos(this: usize, aos_pares: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32)>(1960)(this, aos_pares)
}

/// O técnico do clube é humano? (0x7875e0, stdcall de 1, já em Rust; o ecx = [0xb7431c] da original não conta.)
unsafe fn cqp1_humano(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(clube) != 0
}

/// { 0x48e3e0 } (thiscall, ret 0x1c, comp.cpp, `Comp::send_prize_money_news(clube, valor (i64), posição, rodada, partida,
/// tipo)`; quadro SEH só pela notícia local): para o técnico humano do clube (0x7875e0), com o valor de pelo menos 50 ×
/// a reputação (+0x80), a notícia 0x1a (0x77bbe0): 0 a competição ([this+4]), 1 o valor, 2 o clube, 3 a posição, 4 a
/// rodada, 7 o tipo; com a partida, 5 o adversário e 6 se o clube venceu (+0x3d/+0x3e o vencedor nos pênaltis
/// [inferência], senão +0x4b/+0x4c, senão +0x43/+0x44 o placar); sem ela −1 e 0. Entregue a ele (0x7861e0).
unsafe extern "thiscall" fn noticia_do_premio(this: usize, clube: usize, lo: u32, hi: i32, posicao: u32, rodada: u32, partida: usize, tipo: u32) {
    if !cqp1_humano(clube) {
        return;
    }
    let minimo = *((clube + 0x80) as *const i16) as i64 * 50;
    let valor = ((hi as i64) << 32) | lo as i64;
    if valor < minimo {
        return;
    }
    crate::comparacao::ramo("cqp_1 noticia do premio");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1a, 0);
    dado(p, 0, cqp1_d(cqp1_d(this + 4)) as u32);
    dado(p, 1, lo);
    dado(p, 2, cqp1_d(clube) as u32);
    dado(p, 3, posicao as u8 as i8 as i32 as u32);
    dado(p, 4, rodada as u16 as i16 as i32 as u32);
    dado(p, 7, tipo as u8 as i8 as i32 as u32);
    let b = |a: usize| *(a as *const i8);
    let venceu = if partida != 0 {
        if cqp1_d(partida + 0x1c) == clube {
            dado(p, 5, cqp1_d(cqp1_d(partida + 0x20)) as u32);
            b(partida + 0x3d) != 0 || b(partida + 0x4b) > b(partida + 0x4c) || b(partida + 0x43) > b(partida + 0x44)
        } else {
            dado(p, 5, cqp1_d(cqp1_d(partida + 0x1c)) as u32);
            b(partida + 0x3e) != 0 || b(partida + 0x4b) < b(partida + 0x4c) || b(partida + 0x43) < b(partida + 0x44)
        }
    } else {
        dado(p, 5, u32::MAX);
        false
    };
    dado(p, 6, venceu as u32);
    entregar(cqp1_d(0xb7431c), p, clube);
}

// ponytail: entrega a notícia (aloca); só a original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_premio(this: usize, clube: usize, lo: u32, hi: i32, posicao: u32, rodada: u32, partida: usize, tipo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, usize, u32)>(1961)(this, clube, lo, hi, posicao, rodada, partida, tipo)
}

/// { 0x48ea40 } (thiscall, ret 8, comp.cpp, `Comp::send_tv_rights_news(clube, valor)`; quadro SEH só pela notícia local):
/// para o técnico humano do clube (0x7875e0), a notícia 0x1f (0x77bbe0): 0 o clube, 1 a competição ([this+4]), 2 o
/// valor, 3 o ano da competição (+0x40), entregue a ele (0x7861e0).
unsafe extern "thiscall" fn noticia_da_tv(this: usize, clube: usize, valor: u32) {
    if !cqp1_humano(clube) {
        return;
    }
    crate::comparacao::ramo("cqp_1 noticia da tv");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1f, 0);
    dado(p, 0, cqp1_d(clube) as u32);
    dado(p, 1, cqp1_d(cqp1_d(this + 4)) as u32);
    dado(p, 2, valor);
    dado(p, 3, *((this + 0x40) as *const i16) as i32 as u32);
    entregar(cqp1_d(0xb7431c), p, clube);
}

// ponytail: entrega a notícia (aloca); só a original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_tv(this: usize, clube: usize, valor: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1962)(this, clube, valor)
}

// ---------- 281a leva: o calendário de mais copas de clubes: a Copa dos Campeões, a Merconorte e as Supercopas da Europa e da Ásia (bra_champ_cup.cpp, con_merc_cup.cpp, eur_super_cup.cpp, asia_super_cup.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1963);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1964);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1965);
    cql10_comparar("cqp10_supercopa_europeia_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp10_supercopa_europeia_tabela(this, fase, r, e, s))
}

/// A Supercopa Asiática: ida e volta (14 dias entre os jogos) a partir de 4 de agosto, sorteio em 4 de julho, sem prêmios.
const CQP10_SUPERCOPA_ASIATICA: [Cql10R; 1] = [
    cql10r((4, 6, 0, 5), (4, 7, 0, 5, 2, 0), 0, (0, 0, 0x83), 6, (2, 1, 2, 0), (0, 2, 0xe), (0, 0, 0)), // cqp10: a Supercopa Asiática
];

/// { 0x410910 } (thiscall, ret 0x10, asia_super_cup.cpp; a virtual +0x3c da Supercopa Asiática): `AsiaSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x97 sem memória.
unsafe extern "thiscall" fn cqp10_supercopa_asiatica_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP10_SUPERCOPA_ASIATICA, 0x987be0, 0x97, "cqp_10 tabela da supercopa asiatica")
}

unsafe extern "thiscall" fn comparar_cqp10_supercopa_asiatica_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1966);
    cql10_comparar("cqp10_supercopa_asiatica_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp10_supercopa_asiatica_tabela(this, fase, r, e, s))
}

// ---------- 281a leva: o calendário das copas de clubes da Ásia e do Troféu da Liga inglesa: a Liga dos Campeões e a Recopa asiáticas e o LDV Vans Trophy (asia_club_champ.cpp, asia_cup_winner.cpp, eng_auto_cup.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum Cqp11Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use Cqp11Op::{B as Cqp11B, D as Cqp11D, E as Cqp11E, J as Cqp11J, L as Cqp11L, S as Cqp11S, W as Cqp11W, X as Cqp11X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações e o nome do contador. `None`: a fase devolve 0 sem gravar nada.
#[derive(Clone, Copy)]
struct Cqp11Fase {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [Cqp11Op],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn cqp11_tabela(this: usize, f: Option<Cqp11Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(f.tamanho);
    if t == 0 {
        aviso_em(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops {
        match *op {
            Cqp11D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp11L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqp11S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqp11J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp11E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp11B(o, v) => *((t + o) as *mut u8) = v,
            Cqp11W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqp11X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn cqp11_gravados(ops: &[Cqp11Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops {
        match *op {
            Cqp11D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp11L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp11S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp11J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp11E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp11B(o, _) => pega(o, 1),
            Cqp11W(o, _) => pega(o, 2),
            Cqp11X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn cqp11_comparar(nome: &str, fase: u32, f: Option<Cqp11Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqp11_gravados(f.ops, a) == cqp11_gravados(f.ops, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Liga dos Campeões asiática, eliminatórias de cada zona: 1ª rodada sorteada em 1/8 e jogada em 23/8, 2ª em 14/9 e 20/9, ida e volta.
const CQP11_ASIA_CLUBES_MATA: [Cqp11Op; 36] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0),
];
/// A Liga dos Campeões asiática, grupos finais: 3 rodadas (21, 23 e 25 de março do ano seguinte, valor 1) com os jogos de cada rodada.
const CQP11_ASIA_CLUBES_GRUPOS: [Cqp11Op; 12] = [
    Cqp11D(0, 0x15, 2, 1, 2, 1, 1), Cqp11L(0, 0, 2, 2, 3, 0x10), Cqp11L(0, 1, 2, 1, 3, 0x10),
    Cqp11L(0, 2, -1, -1, -1, 0), Cqp11D(1, 0x17, 2, 1, 4, 1, 1), Cqp11L(1, 0, 4, 2, 3, 0x10),
    Cqp11L(1, 1, 4, 1, 3, 0x10), Cqp11L(1, 2, -1, -1, -1, 0), Cqp11D(2, 0x19, 2, 1, 6, 1, 1),
    Cqp11L(2, 0, 6, 2, 3, 0x10), Cqp11L(2, 1, 6, 1, 3, 0x10), Cqp11L(2, 2, -1, -1, -1, 0),
];
/// A Liga dos Campeões asiática, fase final em maio do ano seguinte: semifinal (com os confrontos 0x521f10), terceiro lugar e final.
const CQP11_ASIA_CLUBES_FINAL: [Cqp11Op; 57] = [
    Cqp11S(0, 0x1a, 2, 1, 0), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10),
    Cqp11E(0, 1, 3, 2, 3, 0x10), Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x383), Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2),
    Cqp11W(0xea, 1), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, eliminatórias: 1ª rodada em 23/8, 2ª em 20/9, quartas em 24 de março do ano seguinte, ida e volta.
const CQP11_ASIA_RECOPA_MATA: [Cqp11Op; 54] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x1c, 8, 0, 3), Cqp11J(2, 0x18, 2, 1, 2, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11W(0xdf, 0x83), Cqp11B(0xe7, 4), Cqp11W(0xe8, 4),
    Cqp11W(0xea, 2), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, fase final em maio do ano seguinte: semifinal, terceiro lugar e final (e a cauda comum +0x12c..+0x137).
const CQP11_ASIA_RECOPA_FINAL: [Cqp11Op; 57] = [
    Cqp11S(0, 0xf, 3, 1, 6), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10), Cqp11E(0, 1, 3, 2, 3, 0x10),
    Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x383),
    Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0), Cqp11W(0x1e, 0),
    Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0), Cqp11X(0x64, 0),
    Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1), Cqp11W(0x73, 0),
    Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1), Cqp11W(0x84, 0),
    Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0),
    Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c), Cqp11W(0xd9, 2),
    Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0), Cqp11X(0x12c, 0),
    Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// O LDV Vans Trophy, cada chave: 5 rodadas de dezembro a março do ano seguinte, a última (a final da chave) em ida e volta.
const CQP11_TROFEU_CHAVES: [Cqp11Op; 90] = [
    Cqp11S(0, 7, 0xa, 0, 5), Cqp11J(0, 0xa, 0xb, 0, 1, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 4), Cqp11W(0x18, 0x18), Cqp11W(0x1a, 0xc), Cqp11W(0x1c, 0x18), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xb, 0xb, 0, 2), Cqp11J(1, 0xe, 0, 1, 1, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 4), Cqp11W(0x80, 0x10), Cqp11W(0x82, 8),
    Cqp11W(0x84, 4), Cqp11W(0x86, 0x18), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0xf, 0, 1, 2), Cqp11J(2, 4, 1, 1, 1, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 4), Cqp11W(0xe8, 8),
    Cqp11W(0xea, 4), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0), Cqp11S(3, 5, 1, 1, 2), Cqp11J(3, 0x12, 1, 1, 1, 2, 0),
    Cqp11W(0x13f, 0x82), Cqp11W(0x141, 3), Cqp11W(0x143, 0), Cqp11W(0x145, 0x83), Cqp11W(0x147, 0), Cqp11B(0x14f, 4),
    Cqp11W(0x150, 4), Cqp11W(0x152, 2), Cqp11W(0x154, 0), Cqp11W(0x156, 0), Cqp11B(0x158, 0), Cqp11B(0x159, 1),
    Cqp11B(0x15a, 0), Cqp11X(0x194, 0), Cqp11X(0x198, 0), Cqp11X(0x19c, 0), Cqp11S(4, 0x13, 1, 1, 2),
    Cqp11J(4, 4, 2, 1, 1, 2, 0), Cqp11W(0x1a7, 0x96), Cqp11W(0x1a9, 4), Cqp11W(0x1ab, 0), Cqp11W(0x1ad, 0),
    Cqp11W(0x1af, 0x83), Cqp11B(0x1b7, 6), Cqp11W(0x1b8, 2), Cqp11W(0x1ba, 1), Cqp11W(0x1bc, 0), Cqp11W(0x1be, 0),
    Cqp11B(0x1c0, 0), Cqp11B(0x1c1, 2), Cqp11B(0x1c2, 7), Cqp11X(0x1fc, 0), Cqp11X(0x200, 0), Cqp11X(0x204, 0),
];
/// O LDV Vans Trophy, a final: sorteio em 12/3 e jogo em 20/4 do ano seguinte, mando 4 (estádio neutro [inferência]).
const CQP11_TROFEU_FINAL: [Cqp11Op; 18] = [
    Cqp11S(0, 0xc, 2, 1, 3), Cqp11J(0, 0x14, 3, 1, 6, 1, 4), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 6), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0xffff_ffff),
    Cqp11X(0x64, 0),
];

/// A Liga dos Campeões asiática [inferência: o Campeonato Asiático de Clubes], as fases: −1 e 0 as eliminatórias de
/// cada zona (2 rodadas de ida e volta em agosto e setembro), 1 e 2 os grupos finais (3 rodadas em março), 3 a fase
/// final (semifinal, terceiro lugar e final em maio).
fn cqp11_asia_clubes_fase(fase: u32) -> Option<Cqp11Fase> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(Cqp11Fase { liga: 0, rodadas: 2, extra: 0x480 + (f != -1) as i16, extra_checa: false, tamanho: 0xd0, linha: 0xc5,
            ops: &CQP11_ASIA_CLUBES_MATA, nome: "cqp_11 eliminatorias da liga dos campeoes asiatica" }),
        1 | 2 => Some(Cqp11Fase { liga: 1, rodadas: 3, extra: 0x482 + (f != 1) as i16, extra_checa: false, tamanho: 0xc3, linha: 0x103,
            ops: &CQP11_ASIA_CLUBES_GRUPOS, nome: "cqp_11 grupos da liga dos campeoes asiatica" }),
        3 => Some(Cqp11Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: true, tamanho: 0x138, linha: 0x125,
            ops: &CQP11_ASIA_CLUBES_FINAL, nome: "cqp_11 final da liga dos campeoes asiatica" }),
        _ => None,
    }
}

/// { 0x40ae90 } (thiscall, ret 0x10, asia_club_champ.cpp; a virtual +0x3c): `AsiaClubChamp::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqp11_asia_clubes_fase`.
unsafe extern "thiscall" fn cqp11_asia_clubes_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqp11_asia_clubes_fase(fase), rodadas, extra, liga, 0x987a40)
}

unsafe extern "thiscall" fn comparar_cqp11_asia_clubes_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1967);
    cqp11_comparar("cqp11_asia_clubes_tabela", fase, cqp11_asia_clubes_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp11_asia_clubes_tabela(this, fase, r, e, l))
}

/// A Recopa asiática, as fases: −1 e 0 as eliminatórias de cada zona (3 rodadas de agosto a março), 1 a fase final
/// (semifinal, terceiro lugar e final em maio).
fn cqp11_asia_recopa_fase(fase: u32) -> Option<Cqp11Fase> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(Cqp11Fase { liga: 0, rodadas: 3, extra: 0x480 + (f != -1) as i16, extra_checa: false, tamanho: 0x138, linha: 0xc6,
            ops: &CQP11_ASIA_RECOPA_MATA, nome: "cqp_11 eliminatorias da recopa asiatica" }),
        1 => Some(Cqp11Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: true, tamanho: 0x138, linha: 0x112,
            ops: &CQP11_ASIA_RECOPA_FINAL, nome: "cqp_11 final da recopa asiatica" }),
        _ => None,
    }
}

/// { 0x40cf90 } (thiscall, ret 0x10, asia_cup_winner.cpp; a virtual +0x3c): `AsiaCupWinner::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqp11_asia_recopa_fase`.
unsafe extern "thiscall" fn cqp11_asia_recopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqp11_asia_recopa_fase(fase), rodadas, extra, liga, 0x987b60)
}

unsafe extern "thiscall" fn comparar_cqp11_asia_recopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1968);
    cqp11_comparar("cqp11_asia_recopa_tabela", fase, cqp11_asia_recopa_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp11_asia_recopa_tabela(this, fase, r, e, l))
}

/// O LDV Vans Trophy (o troféu da liga inglesa de baixo), as fases: −1 e 0 as chaves do Norte e do Sul (5 rodadas em
/// jogo único de dezembro a fevereiro e a final da chave em ida e volta em março), 1 a final (abril, mando 4).
fn cqp11_trofeu_fase(fase: u32) -> Option<Cqp11Fase> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(Cqp11Fase { liga: 0, rodadas: 5, extra: 0x41d + (f != -1) as i16, extra_checa: false, tamanho: 0x208, linha: 0xca,
            ops: &CQP11_TROFEU_CHAVES, nome: "cqp_11 chaves do trofeu" }),
        1 => Some(Cqp11Fase { liga: 0, rodadas: 1, extra: 0, extra_checa: false, tamanho: 0x68, linha: 0x13d,
            ops: &CQP11_TROFEU_FINAL, nome: "cqp_11 final do trofeu" }),
        _ => None,
    }
}

/// { 0x56ade0 } (thiscall, ret 0x10, eng_auto_cup.cpp; a virtual +0x3c do LDV Vans Trophy): `EngAutoCup::
/// create_round_table(fase, &rodadas, &extra, &é_liga)`: a fase de `cqp11_trofeu_fase`.
unsafe extern "thiscall" fn cqp11_trofeu_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqp11_trofeu_fase(fase), rodadas, extra, liga, 0x9c4180)
}

unsafe extern "thiscall" fn comparar_cqp11_trofeu_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1969);
    cqp11_comparar("cqp11_trofeu_tabela", fase, cqp11_trofeu_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp11_trofeu_tabela(this, fase, r, e, l))
}

// ---------- 281a leva: o calendário dos torneios de seleções: a Copa América, a Copa Africana de Nações e a Copa das Confederações (goldcup.cpp, african_nations.cpp, fifa_confed.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum Cqp12Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use Cqp12Op::{B as Cqp12B, D as Cqp12D, E as Cqp12E, J as Cqp12J, L as Cqp12L, S as Cqp12S, W as Cqp12W, X as Cqp12X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
struct Cqp12Fase {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [Cqp12Op],
    ops2: &'static [Cqp12Op],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn cqp12_tabela(this: usize, f: Option<Cqp12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(f.tamanho);
    if t == 0 {
        aviso_em(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqp12D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp12L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqp12S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqp12J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp12E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp12B(o, v) => *((t + o) as *mut u8) = v,
            Cqp12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqp12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn cqp12_gravados(ops: &[Cqp12Op], ops2: &[Cqp12Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqp12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp12B(o, _) => pega(o, 1),
            Cqp12W(o, _) => pega(o, 2),
            Cqp12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn cqp12_comparar(nome: &str, fase: u32, f: Option<Cqp12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqp12_gravados(f.ops, f.ops2, a) == cqp12_gravados(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa América, as três rodadas de cada grupo (6, 10 e 13 de julho, código 1, valor 1) com o jogo 2 vazio.
const CQP12_COPA_AMERICA_GRUPOS: [Cqp12Op; 6] = [
    Cqp12D(0, 6, 6, 0, 3, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xa, 6, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xd, 6, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa América, os jogos do grupo da fase −1.
const CQP12_COPA_AMERICA_GRUPO_A: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 2, 2, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 0x100), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x80),
];
/// A Copa América, os jogos do grupo da fase 0.
const CQP12_COPA_AMERICA_GRUPO_B: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 4, 1, 3, 0x40), Cqp12L(0, 1, 4, 2, 3, 0x100), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x80),
    Cqp12L(2, 0, 3, 1, 3, 0x40), Cqp12L(2, 1, 3, 2, 3, 0x100),
];
/// A Copa América, os jogos do grupo da fase 1.
const CQP12_COPA_AMERICA_GRUPO_C: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 1, 1, 3, 0x100), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x80),
];
/// A Copa América, o mata-mata em jogo único: quartas em 16/7 (com os confrontos fixos), semifinal em 19/7, final em 23/7 e terceiro lugar em 22/7.
const CQP12_COPA_AMERICA_MATA: [Cqp12Op; 80] = [
    Cqp12S(0, 0xf, 6, 0, 5), Cqp12J(0, 0x10, 6, 0, 6, 1, 1), Cqp12E(0, 0, 6, 2, 3, 4), Cqp12E(0, 1, 6, 1, 3, 0x40),
    Cqp12E(0, 2, 0, 1, 3, 0x100), Cqp12E(0, 3, 0, 2, 3, 0x80), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x103), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x12, 6, 0, 1), Cqp12J(1, 0x13, 6, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 0x40), Cqp12E(1, 1, 2, 2, 3, 4), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x103), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x14, 6, 0, 3), Cqp12J(2, 0x17, 6, 0, 6, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x14, 6, 0, 3),
    Cqp12J(3, 0x16, 6, 0, 5, 1, 0x80), Cqp12W(0x150, 2), Cqp12W(0x141, 3), Cqp12W(0x145, 3), Cqp12W(0x13f, 0x8c),
    Cqp12W(0x143, 0), Cqp12B(0x14f, 0xa), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa Africana, as três rodadas de cada grupo (8, 12 e 16 de fevereiro) com o jogo 2 vazio.
const CQP12_AFRICANA_GRUPOS: [Cqp12Op; 6] = [
    Cqp12D(0, 8, 1, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xc, 1, 0, 3, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x10, 1, 0, 0, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa Africana, os jogos do grupo da fase −1.
const CQP12_AFRICANA_GRUPO_A: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 6, 1, 3, 0x40), Cqp12L(1, 0, 2, 1, 3, 4), Cqp12L(1, 1, 2, 2, 3, 0x40),
    Cqp12L(2, 0, 6, 1, 3, 4), Cqp12L(2, 1, 6, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 0.
const CQP12_AFRICANA_GRUPO_B: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x40), Cqp12L(1, 0, 3, 1, 3, 4), Cqp12L(1, 1, 3, 2, 3, 0x40),
    Cqp12L(2, 0, 0, 1, 3, 4), Cqp12L(2, 1, 0, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 1.
const CQP12_AFRICANA_GRUPO_C: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 0x80), Cqp12L(0, 1, 6, 2, 3, 0x80), Cqp12L(1, 0, 2, 2, 3, 0x80),
    Cqp12L(1, 1, 3, 2, 3, 0x80), Cqp12L(2, 0, 0, 2, 3, 0x80), Cqp12L(2, 1, 0, 2, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 2.
const CQP12_AFRICANA_GRUPO_D: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 0, 2, 3, 0x80), Cqp12L(0, 1, 1, 2, 3, 0x80), Cqp12L(1, 0, 4, 1, 3, 0x80),
    Cqp12L(1, 1, 4, 2, 3, 0x80), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// A Copa Africana, o mata-mata em jogo único: quartas em 21/2, semifinal em 25/2, final em 28/2 e terceiro lugar em 27/2.
const CQP12_AFRICANA_MATA: [Cqp12Op; 80] = [
    Cqp12S(0, 0x12, 1, 0, 2), Cqp12J(0, 0x15, 1, 0, 5, 1, 1), Cqp12E(0, 0, 4, 2, 3, 0x80), Cqp12E(0, 1, 5, 1, 3, 4),
    Cqp12E(0, 2, 5, 2, 3, 0x40), Cqp12E(0, 3, 6, 1, 3, 0x40), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x17, 1, 0, 0), Cqp12J(1, 0x19, 1, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 4), Cqp12E(1, 1, 2, 2, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x303), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x1a, 1, 0, 3), Cqp12J(2, 0x1c, 1, 0, 5, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x203), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x1a, 1, 0, 3),
    Cqp12J(3, 0x1b, 1, 0, 4, 2, 0x40), Cqp12W(0x13f, 0x8c), Cqp12W(0x141, 3), Cqp12W(0x143, 0), Cqp12W(0x145, 0x203),
    Cqp12B(0x14f, 0xa), Cqp12W(0x150, 2), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa das Confederações, as três rodadas de cada grupo (29 de julho, 1º e 4 de agosto) com o jogo 2 vazio.
const CQP12_CONFED_GRUPOS: [Cqp12Op; 6] = [
    Cqp12D(0, 0x1d, 6, 0, 5, 2, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 1, 7, 0, 1, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 4, 7, 0, 4, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa das Confederações, os jogos do grupo da fase −1.
const CQP12_CONFED_GRUPO_A: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 4), Cqp12L(0, 1, 6, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];
/// A Copa das Confederações, os jogos do grupo da fase 0.
const CQP12_CONFED_GRUPO_B: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x40), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(1, 1, 2, 2, 3, 4),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];
/// A Copa das Confederações, o mata-mata em jogo único: semifinal em 6/8, final em 9/8 e terceiro lugar em 8/8.
const CQP12_CONFED_MATA: [Cqp12Op; 57] = [
    Cqp12S(0, 5, 7, 0, 5), Cqp12J(0, 6, 7, 0, 6, 1, 1), Cqp12E(0, 0, 6, 1, 3, 0x40), Cqp12E(0, 1, 6, 1, 3, 4),
    Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x383),
    Cqp12B(0x17, 0xa), Cqp12W(0x18, 4), Cqp12W(0x1a, 2), Cqp12W(0x1c, 4), Cqp12W(0xf, 0), Cqp12W(0x1e, 0),
    Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
    Cqp12S(1, 7, 7, 0, 0), Cqp12J(1, 9, 7, 0, 2, 1, 4), Cqp12W(0x6f, 0x96), Cqp12W(0x71, 1), Cqp12W(0x73, 0),
    Cqp12W(0x75, 0x83), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 2), Cqp12W(0x82, 1), Cqp12W(0x84, 0), Cqp12W(0x77, 0),
    Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0), Cqp12X(0xc4, 0), Cqp12X(0xc8, 0),
    Cqp12X(0xcc, 0), Cqp12S(2, 7, 7, 0, 0), Cqp12J(2, 8, 7, 0, 1, 1, 0x40), Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 2),
    Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x83), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2), Cqp12W(0xea, 1), Cqp12W(0xec, 0),
    Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1), Cqp12B(0xf2, 0), Cqp12X(0x12c, 0),
    Cqp12X(0x130, 0), Cqp12X(0x134, 0),
];

/// A fase da Copa América pelo byte com sinal: abaixo de 2 os grupos (id 0x420 + fase; os jogos de cada grupo para −1,
/// 0 e 1), 2 o mata-mata, acima nada.
fn cqp12_copa_america_fase(fase: u32) -> Option<Cqp12Fase> {
    let f = fase as u8 as i8;
    if f < 2 {
        let jogos: &'static [Cqp12Op] = match f {
            -1 => &CQP12_COPA_AMERICA_GRUPO_A,
            0 => &CQP12_COPA_AMERICA_GRUPO_B,
            1 => &CQP12_COPA_AMERICA_GRUPO_C,
            _ => &[],
        };
        Some(Cqp12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x420), extra_checa: false, tamanho: 0xc3, linha: 0x9c,
            ops: &CQP12_COPA_AMERICA_GRUPOS, ops2: jogos, nome: "cqp_12 grupos da copa america" })
    } else if f == 2 {
        Some(Cqp12Fase { liga: 0, rodadas: 4, extra: 0, extra_checa: false, tamanho: 0x1a0, linha: 0xd6,
            ops: &CQP12_COPA_AMERICA_MATA, ops2: &[], nome: "cqp_12 mata-mata da copa america" })
    } else {
        None
    }
}

/// { 0x5e0900 } (thiscall, ret 0x10, goldcup.cpp; a virtual +0x3c da Copa América): `GoldCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqp12_copa_america_fase`.
unsafe extern "thiscall" fn cqp12_copa_america_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp12_tabela(this, cqp12_copa_america_fase(fase), rodadas, extra, liga, 0x9cd640)
}

unsafe extern "thiscall" fn comparar_cqp12_copa_america_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1970);
    cqp12_comparar("cqp12_copa_america_tabela", fase, cqp12_copa_america_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp12_copa_america_tabela(this, fase, r, e, l))
}

/// A fase da Copa Africana: abaixo de 3 os grupos (id 0x3e9 + fase; os jogos de cada grupo para −1, 0, 1 e 2), 3 o
/// mata-mata, acima nada.
fn cqp12_africana_fase(fase: u32) -> Option<Cqp12Fase> {
    let f = fase as u8 as i8;
    if f < 3 {
        let jogos: &'static [Cqp12Op] = match f {
            -1 => &CQP12_AFRICANA_GRUPO_A,
            0 => &CQP12_AFRICANA_GRUPO_B,
            1 => &CQP12_AFRICANA_GRUPO_C,
            2 => &CQP12_AFRICANA_GRUPO_D,
            _ => &[],
        };
        Some(Cqp12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x3e9), extra_checa: false, tamanho: 0xc3, linha: 0x9e,
            ops: &CQP12_AFRICANA_GRUPOS, ops2: jogos, nome: "cqp_12 grupos da copa africana" })
    } else if f == 3 {
        Some(Cqp12Fase { liga: 0, rodadas: 4, extra: 0, extra_checa: false, tamanho: 0x1a0, linha: 0xe0,
            ops: &CQP12_AFRICANA_MATA, ops2: &[], nome: "cqp_12 mata-mata da copa africana" })
    } else {
        None
    }
}

/// { 0x4012e0 } (thiscall, ret 0x10, african_nations.cpp; a virtual +0x3c da Copa Africana): `AfricanNations::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `cqp12_africana_fase`.
unsafe extern "thiscall" fn cqp12_africana_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp12_tabela(this, cqp12_africana_fase(fase), rodadas, extra, liga, 0x9870f8)
}

unsafe extern "thiscall" fn comparar_cqp12_africana_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1971);
    cqp12_comparar("cqp12_africana_tabela", fase, cqp12_africana_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp12_africana_tabela(this, fase, r, e, l))
}

/// A fase da Copa das Confederações: abaixo de 1 os grupos (id 0x3e9 + fase; os jogos de cada grupo para −1 e 0), 1 o
/// mata-mata, acima nada.
fn cqp12_confed_fase(fase: u32) -> Option<Cqp12Fase> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [Cqp12Op] = match f {
            -1 => &CQP12_CONFED_GRUPO_A,
            0 => &CQP12_CONFED_GRUPO_B,
            _ => &[],
        };
        Some(Cqp12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x3e9), extra_checa: false, tamanho: 0xc3, linha: 0x95,
            ops: &CQP12_CONFED_GRUPOS, ops2: jogos, nome: "cqp_12 grupos da copa das confederacoes" })
    } else if f == 1 {
        Some(Cqp12Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: false, tamanho: 0x138, linha: 0xc7,
            ops: &CQP12_CONFED_MATA, ops2: &[], nome: "cqp_12 mata-mata da copa das confederacoes" })
    } else {
        None
    }
}

/// { 0x58b560 } (thiscall, ret 0x10, fifa_confed.cpp; a virtual +0x3c da Copa das Confederações): `FifaConfed::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `cqp12_confed_fase`.
unsafe extern "thiscall" fn cqp12_confed_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp12_tabela(this, cqp12_confed_fase(fase), rodadas, extra, liga, 0x9c7474)
}

unsafe extern "thiscall" fn comparar_cqp12_confed_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1972);
    cqp12_comparar("cqp12_confed_tabela", fase, cqp12_confed_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp12_confed_tabela(this, fase, r, e, l))
}

// ---------- 281a leva: o calendário das copas intercontinentais e da Intertoto: a Intertoto, a Copa Intercontinental, a Copa Interamericana e a Copa dos Campeões da CONCACAF (intertoto_cup.cpp, world_club_cup.cpp, inter_amer_cup.cpp, con_champ.cpp) ----------

/// A Intertoto: cinco rodadas em ida e volta de junho a agosto (a 1ª com 40 times; 12 novos na 2ª, 8 na 3ª).
const CQP13_INTERTOTO: [Cql10R; 5] = [
    cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: a 1ª rodada da Intertoto
    cql10r((1, 6, 0, 3), (3, 6, 0, 5, 1, 0), 0x14, (0, 0, 3), 7, (0x20, 0x10, 0xc, 0x28), (0, 2, 7), (0, 0, 0)),
    cql10r((0xb, 6, 0, 6), (0x11, 6, 0, 5, 1, 0), 0x1e, (0, 0, 3), 7, (0x18, 0xc, 8, 0x34), (0, 2, 7), (0, 0, 0)),
    cql10r((0x19, 6, 0, 6), (0x1c, 6, 0, 2, 2, 0), 0x82, (0, 0, 3), 8, (0xc, 6, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((5, 7, 0, 3), (0xa, 7, 0, 1, 2, 0), 0x96, (0, 0, 3), 8, (6, 3, 0, 0), (0, 2, 0xe), (0, 0, 0)),
];

/// { 0x633300 } (thiscall, ret 0x10, intertoto_cup.cpp; a virtual +0x3c): `IntertotoCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0xe4 sem memória.
unsafe extern "thiscall" fn cqp13_intertoto_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERTOTO, 0x9e9e14, 0xe4, "cqp_13 tabela da intertoto")
}

unsafe extern "thiscall" fn comparar_cqp13_intertoto_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1973);
    cql10_comparar("cqp13_intertoto_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_intertoto_tabela(this, fase, r, e, s))
}

/// A Copa Intercontinental: jogo único em 1º de dezembro (sorteio em 10 de outubro), mando 4 (Tóquio [inferência]).
const CQP13_INTERCONTINENTAL: [Cql10R; 1] = [
    cql10r((0xa, 9, 0, 5), (1, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp13: a Copa Intercontinental
];

/// { 0x92b770 } (thiscall, ret 0x10, world_club_cup.cpp; a virtual +0x3c): `WorldClubCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
unsafe extern "thiscall" fn cqp13_intercontinental_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERCONTINENTAL, 0xad5848, 0x9a, "cqp_13 tabela da intercontinental")
}

unsafe extern "thiscall" fn comparar_cqp13_intercontinental_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1974);
    cql10_comparar("cqp13_intercontinental_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_intercontinental_tabela(this, fase, r, e, s))
}

/// A Copa Interamericana: ida e volta a partir de 26 de janeiro do ano seguinte (sorteio em 10 de outubro), 7 dias.
const CQP13_INTERAMERICANA: [Cql10R; 1] = [
    cql10r((0xa, 9, 0, 5), (0x1a, 0, 1, 2, 2, 0), 0, (0, 0, 3), 8, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)),
];

/// { 0x632340 } (thiscall, ret 0x10, inter_amer_cup.cpp; a virtual +0x3c): `InterAmerCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
unsafe extern "thiscall" fn cqp13_interamericana_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERAMERICANA, 0x9e9d94, 0x9a, "cqp_13 tabela da interamericana")
}

unsafe extern "thiscall" fn comparar_cqp13_interamericana_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1975);
    cql10_comparar("cqp13_interamericana_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_interamericana_tabela(this, fase, r, e, s))
}

/// A Copa dos Campeões da CONCACAF: quartas (28/8), semifinal (1/9) e final (4/9) em jogo único, 8 times.
const CQP13_CONCACAF: [Cql10R; 3] = [
    cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: as quartas da CONCACAF
    cql10r((0x1d, 8, 0, 4), (1, 9, 0, 6, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((2, 9, 0, 0), (4, 9, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x4c0f00 } (thiscall, ret 0x10, con_champ.cpp; a virtual +0x3c): `ConChamp::create_round_table(fase, &rodadas,
/// &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 3, extra = 0; a tabela (0x944e46 de 0x138; sem
/// memória, a caixa da linha 0x97 e 0) com as 3 rodadas e, nas duas primeiras, o dia 0 do confronto vazio (0x521f10
/// com −1, −1, −1 e mando 0; a original o chama entre o jogo e os campos, que não se sobrepõem). Fora da −1, 0.
unsafe extern "thiscall" fn cqp13_concacaf_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 3;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x138);
    if t == 0 {
        aviso_em(0x9a7d30, 0x97);
        return 0;
    }
    crate::comparacao::ramo("cqp_13 tabela da concacaf");
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in CQP13_CONCACAF.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
        if i < 2 {
            elo(t, i as i32, 0, -1, -1, -1, 0);
        }
    }
    t
}

unsafe extern "thiscall" fn comparar_cqp13_concacaf_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1976);
    let a = cql10_comparar("cqp13_concacaf_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_concacaf_tabela(this, fase, r, e, s));
    // ponytail: a cql10_comparar não olha os dias dos confrontos (+0x23..); o dia 0 das duas primeiras rodadas é
    // conferido aqui contra o valor fixo que a cópia grava
    if a != 0 {
        let ok = (0..2usize).all(|r| {
            let e = a + r * 0x68 + 0x23;
            core::ptr::read_unaligned(e as *const u32) == 0 && core::slice::from_raw_parts((e + 4) as *const u8, 3) == [0xff, 0xff, 0xff]
        });
        PLACAR.registrar(ok, || format!("cqp13_concacaf_tabela: o dia 0 dos confrontos da original difere"));
    }
    a
}

// ---------- 281a leva: a Copa da UEFA e a Intertoto: as rodadas da Copa da UEFA, os inscritos da Intertoto e a lista dos seus clubes (uefa_cup.cpp, intertoto_cup.cpp) ----------

/// As rodadas fixas da Copa da UEFA (a preliminar e os campos que dependem de +0xb2 vêm à parte, em
/// `cqq10_uefa_rodadas`): a 1ª rodada de qualificação em agosto, a 2ª no fim de agosto, a 1ª rodada em setembro, a 2ª
/// em novembro, a 3ª em dezembro, as oitavas em fevereiro, as quartas e a semifinal em março e abril e a final em
/// maio, todas em ida e volta menos a final.
const CQQ10_UEFA: [Cql10R; 8] = [
    cql10r((0x1c, 6, 0, 4), (0xa, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqq10: a 1ª rodada de qualificação da UEFA
    cql10r((0x19, 7, 0, 4), (0xe, 8, 0, 3, 2, 0), 0xa, (4, 0x400, 3), 7, (0x60, 0x30, 0x37, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 8, 0, 4), (0x13, 9, 0, 3, 2, 0), 0x14, (4, 0, 3), 7, (0x30, 0x18, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xa, 0, 4), (0x17, 0xa, 0, 3, 2, 0), 0x1e, (0, 0, 3), 8, (0x20, 0x10, 8, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 0xb, 0, 4), (1, 2, 1, 3, 2, 0), 0x28, (0, 0, 3), 8, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 2, 1, 4), (0xf, 2, 1, 3, 2, 0), 0x78, (0, 0, 3), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 2, 1, 4), (5, 3, 1, 3, 2, 0), 0x82, (0, 0, 3), 8, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 4), (0x10, 4, 1, 2, 2, 4), 0x96, (0, 0x83, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0x16e360, 0)),
];

/// A preliminar (só com `b` = +0xb2 > 0): sorteio em 28/6, jogo em 6/7; b times, b/2 jogos, b novos.
const CQQ10_UEFA_PRELIMINAR: Cql10R =
    cql10r((0x1c, 5, 0, 2), (6, 6, 0, 3, 2, 0), 0xa0, (0, 0x200, 3), 7, (0, 0, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0));

/// As rodadas da Copa da UEFA com `b` = +0xb2 (byte com sinal; as metades truncam como a original, cdq/sar): com b > 0
/// a preliminar (b times, b/2 jogos, b novos) e depois as oito; na 1ª rodada de qualificação 0x52 − b/2 novos e b já
/// inscritos, na 2ª b/2 + 0x52 inscritos, na 1ª rodada b/2 + 0x89 inscritos.
fn cqq10_uefa_rodadas(b: i8) -> ([Cql10R; 9], usize) {
    let mut t = [CQQ10_UEFA_PRELIMINAR; 9];
    let (b, m) = (b as i32, b as i32 / 2);
    let mut u = CQQ10_UEFA;
    u[0].times.2 = (0x52 - m) as u16;
    u[0].times.3 = b as u16;
    u[1].times.3 = (m + 0x52) as u16;
    u[3].times.3 = (m + 0x89) as u16;
    let mut n = 0;
    if b > 0 {
        crate::comparacao::ramo("cqq_10 preliminar da uefa");
        t[0].times = (b as u16, m as u16, b as u16, 0);
        n = 1;
    }
    for r in u {
        t[n] = r;
        n += 1;
    }
    (t, n)
}

/// { 0x9025c0 } (thiscall, ret 0x10, uefa_cup.cpp; a virtual +0x3c da Copa da UEFA): `UefaCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 9, extra = 0; a tabela (0x944e46 de 0x3a8;
/// sem memória, a caixa da linha 0xe0 e 0) com as rodadas de `cqq10_uefa_rodadas` e rodadas = as gravadas. Fora da −1,
/// 0.
unsafe extern "thiscall" fn cqq10_uefa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 9;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x3a8);
    if t == 0 {
        aviso_em(0xaa19e0, 0xe0);
        return 0;
    }
    crate::comparacao::ramo("cqq_10 tabela da uefa");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let (tab, n) = cqq10_uefa_rodadas(*((this + 0xb2) as *const i8));
    for (i, r) in tab[..n].iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    *rodadas = n as i16;
    t
}

unsafe extern "thiscall" fn comparar_cqq10_uefa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1977);
    cql10_comparar("cqq10_uefa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqq10_uefa_tabela(this, fase, r, e, s))
}

unsafe fn cqq10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqq10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x6336c0 } (thiscall, ret 0, intertoto_cup.cpp): `IntertotoCup::collect_entrants()`: +0x3e = 60; um vetor local de
/// 60 ponteiros (0x944e46 de 0xf0; sem memória, a caixa da linha 0x161 e 0) com os clubes da tabela ([0xae23b4], até
/// [0xae235c]) cuja copa europeia (+0x1db) é a Intertoto ([0x9cf6f8]), até 60; faltando, a caixa da linha 0x172 e 0 (o
/// vetor fica alocado, como na original). A lista (0x944e46 de 6 × 60 em +0xa7; sem memória, a caixa 0x17c e −1) leva
/// cada clube com a classe = o seu coeficiente (+0x1df) e a situação 0, vai ao qsort (0x522190) e o vetor é liberado.
/// Devolve 1 com os 60.
unsafe fn cqq10_intertoto(this: usize) -> (i32, i32) {
    const F: usize = 0x9e9e14;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x3c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0xf0);
    if v == 0 {
        aviso_em(F, 0x161);
        return (0, 0);
    }
    let mut k = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < cqq10_d(0xae235c) as i32 {
        if k >= cqq10_w(this + 0x3e) as i32 {
            break;
        }
        let c = cqq10_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize);
        if cqq10_d(c + 0x1db) == cqq10_d(0x9cf6f8) {
            core::ptr::write_unaligned((v + k as usize * 4) as *mut u32, c as u32);
            k += 1;
        }
        i = i.wrapping_add(1);
    }
    let n = cqq10_w(this + 0x3e) as i32;
    if k < n {
        aviso_em(F, 0x172);
        return (0, 0);
    }
    let l = alocar((n * 6) as u32);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(F, 0x17c);
        return (-1, 0);
    }
    crate::comparacao::ramo("cqq_10 inscritos da intertoto");
    let mut j: i16 = 0;
    while j < cqq10_w(this + 0x3e) {
        let c = cqq10_d(v + j as usize * 4);
        let e = cqq10_d(this + 0xa7) + j as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = *((c + 0x1df) as *const u8); // cqq10: a classe pelo coeficiente
        *((e + 5) as *mut u8) = 0;
        j = j.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqq10_d(this + 0xa7), cqq10_w(this + 0x3e) as i32 as u32, 6, 0x522190);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    soltar(v);
    ((k == cqq10_w(this + 0x3e) as i32) as i32, cqq10_w(this + 0x3e) as i32)
}

unsafe extern "thiscall" fn cqq10_intertoto_inscritos(this: usize) -> i32 {
    cqq10_intertoto(this).0
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a lista dela em +0xa7, a cópia faz a
/// sua, comparada nos inscritos (com a devolução 1) e liberada.
unsafe extern "thiscall" fn comparar_cqq10_intertoto_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1978);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let antes = cqq10_d(this + 0xa7);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqq10_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, antes as u32);
        r
    }, || {
        let (r, n) = cqq10_intertoto(this);
        lr.set(cqq10_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { nr.get().max(0) as usize * 6 } else { 0 };
    let lista = if lo != antes && lr != antes && lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == lr || k == 0
    };
    if lr != 0 && lr != lo && lr != antes {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqq10_intertoto_inscritos({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x633f70 } (thiscall, ret 8, intertoto_cup.cpp; uma virtual): `IntertotoCup::collect_club_list(&saida, ativos)`:
/// uma lista de ponteiros (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2fb, saida = 0 e 0) com os clubes da
/// lista +0xa7 (não nulos, até +0x3e): com `ativos`, só os de situação (+5) 0 ou 1. Grava a lista em saida e devolve a
/// conta (palavra).
unsafe fn cqq10_lista(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let n = cqq10_w(this + 0x3e);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar((n as i32 * 4) as u32);
    if l == 0 {
        aviso_em(0x9e9e14, 0x2fb);
        *saida = 0;
        return 0;
    }
    crate::comparacao::ramo("cqq_10 lista da intertoto");
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    while i < cqq10_w(this + 0x3e) {
        let e = cqq10_d(this + 0xa7) + i as usize * 6;
        let s = *((e + 5) as *const u8);
        let c = cqq10_d(e);
        if (ativos as u8 == 0 || s == 1 || s == 0) && c != 0 && k < n { // cqq10: os clubes da lista
            core::ptr::write_unaligned((l + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    *saida = l;
    k as u16
}

unsafe extern "thiscall" fn cqq10_intertoto_lista(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    cqq10_lista(this, saida, ativos)
}

/// Lado a lado: a original grava a lista dela em `saida` (de quem chama, que a libera); a cópia faz a sua numa saída
/// própria, comparada na conta e nos ponteiros, e a libera.
unsafe extern "thiscall" fn comparar_cqq10_intertoto_lista(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(1979);
    let mut s2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, saida, ativos), || cqq10_lista(this, &mut s2, ativos));
    let lo = *saida;
    let lista = if lo != 0 && s2 != 0 {
        core::slice::from_raw_parts(lo as *const u8, a as usize * 4) == core::slice::from_raw_parts(s2 as *const u8, b as usize * 4)
    } else {
        lo == 0 && s2 == 0
    };
    if s2 != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(s2);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqq10_intertoto_lista({:#x}, {}) devolveu {} x {} lista {}", this, ativos as u8, a, b, lista));
    a
}

// ---------- 281a leva: os clubes das supercopas: a Supercopa Europeia, a Supercopa Asiática e a Supercopa da Série C1 (eur_super_cup.cpp, asia_super_cup.cpp, ita_c1_super.cpp) ----------

/// { 0x57a500 } (thiscall, ret 0, eur_super_cup.cpp): `EurSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc1 e −1). O campeão da Liga dos Campeões ([0x9cf6e8]; sem ele, a caixa 0xca e 0) e o da Recopa
/// ([0x9cf6ec]) na copa de 1999, nas outras o da Copa da UEFA ([0x9cf6f0]); sem ele, 0xd5 e 0. Devolve 1.
unsafe fn cqq11_europeia(this: usize) -> (i32, i32) {
    const F: usize = 0x9c45f8;
    let l = match cqm12_lista(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf6e8, 0); // cqq11: o campeão da Liga dos Campeões
    if !cqm12_vaga(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    let copa = if core::ptr::read_unaligned((this + 0x40) as *const i16) == 0x7cf { 0x9cf6ec } else { 0x9cf6f0 };
    let b = cqm12_colocado(copa, 0); // cqq11: o campeão da Copa da UEFA
    if !cqm12_vaga(l, 1, b, F, 0xd5) {
        return (0, 0);
    }
    crate::comparacao::ramo("cqq_11 clubes da supercopa europeia");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqq11_europeia_clubes(this: usize) -> i32 {
    cqq11_europeia(this).0
}

unsafe extern "thiscall" fn comparar_cqq11_europeia_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1980);
    cqm12_comparar("cqq11_europeia_clubes", this, || o(this), || cqq11_europeia(this))
}

/// { 0x410a50 } (thiscall, ret 0, asia_super_cup.cpp): `AsiaSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc0 e −1). O campeão da Liga dos Campeões asiática ([0x9cf64c]; sem ele, 0xc9 e 0) e o da Recopa
/// asiática ([0x9cf650]; sem ele, 0xd1 e 0); se é o mesmo clube, o primeiro vira o vice da Liga dos Campeões (sem
/// conferir, como a original). Devolve 1.
unsafe fn cqq11_asiatica(this: usize) -> (i32, i32) {
    const F: usize = 0x987be0;
    let l = match cqm12_lista(this, F, 0xc0) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf64c, 0);
    if !cqm12_vaga(l, 0, a, F, 0xc9) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf650, 0); // cqq11: o campeão da Recopa asiática
    if !cqm12_vaga(l, 1, b, F, 0xd1) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqq_11 dobradinha asiatica");
        core::ptr::write_unaligned(l as *mut u32, cqm12_colocado(0x9cf64c, 1) as u32);
    }
    crate::comparacao::ramo("cqq_11 clubes da supercopa asiatica");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqq11_asiatica_clubes(this: usize) -> i32 {
    cqq11_asiatica(this).0
}

unsafe extern "thiscall" fn comparar_cqq11_asiatica_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1981);
    cqm12_comparar("cqq11_asiatica_clubes", this, || o(this), || cqq11_asiatica(this))
}

/// { 0x63c9d0 } (thiscall, ret 0, ita_c1_super.cpp): `ItaC1Super::init_team_list()`: a lista de dois (sem memória, a
/// caixa da linha 0xc2 e −1) com as duas vagas vazias (os clubes entram depois, com os campeões das duas chaves da C1
/// [inferência]); classe e situação 0. Devolve 1.
unsafe fn cqq11_c1(this: usize) -> (i32, i32) {
    let l = match cqm12_lista(this, 0x9ea1c8, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    crate::comparacao::ramo("cqq_11 vagas da supercopa c1");
    core::ptr::write_unaligned(l as *mut u32, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, 0);
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqq11_c1_vagas(this: usize) -> i32 {
    cqq11_c1(this).0
}

unsafe extern "thiscall" fn comparar_cqq11_c1_vagas(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1982);
    cqm12_comparar("cqq11_c1_vagas", this, || o(this), || cqq11_c1(this))
}

// ---------- 281a leva: as copas de clubes da Oceania e da Ásia: o calendário do Campeonato de Clubes da Oceania e os times da Liga dos Campeões e da Recopa asiáticas (oceania_club_champ.cpp, asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum Cqq12Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use Cqq12Op::{B as Cqq12B, D as Cqq12D, E as Cqq12E, J as Cqq12J, L as Cqq12L, S as Cqq12S, W as Cqq12W, X as Cqq12X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
struct Cqq12Fase {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [Cqq12Op],
    ops2: &'static [Cqq12Op],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn cqq12_tabela(this: usize, f: Option<Cqq12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(f.tamanho);
    if t == 0 {
        aviso_em(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqq12D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqq12L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqq12S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqq12J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqq12E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqq12B(o, v) => *((t + o) as *mut u8) = v,
            Cqq12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqq12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn cqq12_gravados(ops: &[Cqq12Op], ops2: &[Cqq12Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqq12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqq12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqq12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqq12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqq12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqq12B(o, _) => pega(o, 1),
            Cqq12W(o, _) => pega(o, 2),
            Cqq12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn cqq12_comparar(nome: &str, fase: u32, f: Option<Cqq12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqq12_gravados(f.ops, f.ops2, a) == cqq12_gravados(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// O Campeonato de Clubes da Oceania, as três rodadas de cada grupo (10, 12 e 14 de janeiro do ano seguinte, valor 1) com o jogo 2 vazio.
const CQQ12_OCEANIA_GRUPOS: [Cqq12Op; 6] = [
    Cqq12D(0, 0xa, 0, 1, 4, 2, 1), Cqq12L(0, 2, -1, -1, -1, 0), Cqq12D(1, 0xc, 0, 1, 6, 1, 1),
    Cqq12L(1, 2, -1, -1, -1, 0), Cqq12D(2, 0xe, 0, 1, 1, 2, 1), Cqq12L(2, 2, -1, -1, -1, 0),
];
/// Os jogos do grupo da fase −1.
const CQQ12_OCEANIA_GRUPO_A: [Cqq12Op; 6] = [
    Cqq12L(0, 0, 4, 1, 3, 4), Cqq12L(0, 1, 4, 2, 3, 0x40), Cqq12L(1, 0, 6, 1, 3, 4), Cqq12L(1, 1, 6, 2, 3, 0x40),
    Cqq12L(2, 0, 1, 1, 3, 4), Cqq12L(2, 1, 1, 1, 3, 0x40),
];
/// Os jogos do grupo da fase 0.
const CQQ12_OCEANIA_GRUPO_B: [Cqq12Op; 6] = [
    Cqq12L(0, 0, 5, 1, 3, 0x40), Cqq12L(0, 1, 5, 2, 3, 4), Cqq12L(1, 0, 0, 1, 3, 0x40), Cqq12L(1, 1, 0, 2, 3, 4),
    Cqq12L(2, 0, 2, 1, 3, 4), Cqq12L(2, 1, 2, 1, 3, 0x40),
];
/// O mata-mata em jogo único: semifinal em 17/1, final e terceiro lugar em 19/1 do ano seguinte.
const CQQ12_OCEANIA_MATA: [Cqq12Op; 54] = [
    Cqq12S(0, 0x10, 0, 1, 3), Cqq12J(0, 0x11, 0, 1, 4, 2, 0x40), Cqq12W(0x7, 0x82), Cqq12W(0x9, 0), Cqq12W(0xb, 0),
    Cqq12W(0xd, 0x283), Cqq12B(0x17, 8), Cqq12W(0x18, 4), Cqq12W(0x1a, 2), Cqq12W(0x1c, 4), Cqq12W(0xf, 0),
    Cqq12W(0x1e, 0), Cqq12B(0x20, 0), Cqq12B(0x21, 1), Cqq12B(0x22, 0), Cqq12X(0x5c, 0), Cqq12X(0x60, 0),
    Cqq12X(0x64, 0), Cqq12S(1, 0x12, 0, 1, 5), Cqq12J(1, 0x13, 0, 1, 6, 1, 4), Cqq12W(0x6f, 0x96), Cqq12W(0x71, 1),
    Cqq12W(0x73, 0), Cqq12W(0x75, 0x83), Cqq12B(0x7f, 8), Cqq12W(0x80, 2), Cqq12W(0x82, 1), Cqq12W(0x84, 0),
    Cqq12W(0x77, 0), Cqq12W(0x86, 0), Cqq12B(0x88, 0), Cqq12B(0x89, 1), Cqq12B(0x8a, 0), Cqq12X(0xc4, 0),
    Cqq12X(0xc8, 0), Cqq12X(0xcc, 0), Cqq12S(2, 0x12, 0, 1, 5), Cqq12J(2, 0x13, 0, 1, 6, 1, 0x40), Cqq12W(0xd7, 0x8c),
    Cqq12W(0xd9, 2), Cqq12W(0xdb, 0), Cqq12W(0xdd, 0x83), Cqq12B(0xe7, 8), Cqq12W(0xe8, 2), Cqq12W(0xea, 1),
    Cqq12W(0xec, 0), Cqq12W(0xdf, 0), Cqq12W(0xee, 0), Cqq12B(0xf0, 0), Cqq12B(0xf1, 1), Cqq12B(0xf2, 0),
    Cqq12X(0x12c, 0), Cqq12X(0x130, 0), Cqq12X(0x134, 0),
];

/// A fase do Campeonato de Clubes da Oceania: abaixo de 1 os grupos (id 0x420 + fase; os jogos de cada grupo para −1
/// e 0), 1 o mata-mata, acima nada.
fn cqq12_oceania_fase(fase: u32) -> Option<Cqq12Fase> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [Cqq12Op] = match f {
            -1 => &CQQ12_OCEANIA_GRUPO_A,
            0 => &CQQ12_OCEANIA_GRUPO_B,
            _ => &[],
        };
        Some(Cqq12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x420), extra_checa: false, tamanho: 0xc3, linha: 0x93,
            ops: &CQQ12_OCEANIA_GRUPOS, ops2: jogos, nome: "cqq_12 grupos da oceania" })
    } else if f == 1 {
        Some(Cqq12Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: false, tamanho: 0x138, linha: 0xc5,
            ops: &CQQ12_OCEANIA_MATA, ops2: &[], nome: "cqq_12 mata-mata da oceania" })
    } else {
        None
    }
}

/// { 0x797230 } (thiscall, ret 0x10, oceania_club_champ.cpp; a virtual +0x3c): `OceaniaClubChamp::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqq12_oceania_fase`.
unsafe extern "thiscall" fn cqq12_oceania_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqq12_tabela(this, cqq12_oceania_fase(fase), rodadas, extra, liga, 0xa6a068)
}

unsafe extern "thiscall" fn comparar_cqq12_oceania_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1983);
    cqq12_comparar("cqq12_oceania_tabela", fase, cqq12_oceania_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqq12_oceania_tabela(this, fase, r, e, l))
}

unsafe fn cqq12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// A tabela dos 16 times das copas asiáticas (AsiaClubChamp::build_team_table 0x40b620 e AsiaCupWinner::
/// create_team_entries 0x40d4d0, o mesmo corpo): +0x3e = 16 e a tabela (0x944e46 de 0x60 em +0xa7; sem memória, a caixa
/// `linha` de `arquivo` e −1); a vaga i recebe o clube i dos classificados (+0xb2, ponteiros; `ordem` dá o índice) com
/// classe e situação 0. Devolve 1.
unsafe fn cqq12_times(this: usize, arquivo: usize, linha: u32, nome: &'static str, ordem: impl Fn(i16) -> i16) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x10);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x60);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return -1;
    }
    crate::comparacao::ramo(nome);
    let mut i: i16 = 0;
    while i < core::ptr::read_unaligned((this + 0x3e) as *const i16) {
        let c = cqq12_d(cqq12_d(this + 0xb2) + ordem(i) as usize * 4);
        let e = cqq12_d(this + 0xa7) + i as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = 0;
        *((e + 5) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 16 vagas (com a devolução 1) e liberada.
unsafe fn cqq12_comparar_times(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqq12_d(this + 0xa7));
        r
    }, || {
        let r = c();
        lr.set(cqq12_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 0x60 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", nome, this, a, b, igual, lista));
    a
}

/// { 0x40b620 } (thiscall, ret 0, asia_club_champ.cpp): `AsiaClubChamp::build_team_table()`: os 16 times (a caixa da
/// linha 0x1cd sem memória).
unsafe extern "thiscall" fn cqq12_asia_clubes_times(this: usize) -> i32 {
    cqq12_times(this, 0x987a40, 0x1cd, "cqq_12 times da liga dos campeoes asiatica", |i| i) // cqq12: os times da Liga dos Campeões asiática
}

unsafe extern "thiscall" fn comparar_cqq12_asia_clubes_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1984);
    cqq12_comparar_times("cqq12_asia_clubes_times", this, || o(this), || cqq12_asia_clubes_times(this))
}

/// { 0x40d4d0 } (thiscall, ret 0, asia_cup_winner.cpp): `AsiaCupWinner::create_team_entries()`: os 16 times (a caixa
/// da linha 0x185 sem memória).
unsafe extern "thiscall" fn cqq12_asia_recopa_times(this: usize) -> i32 {
    cqq12_times(this, 0x987b60, 0x185, "cqq_12 times da recopa asiatica", |i| i) // cqq12: os times da Recopa asiática
}

unsafe extern "thiscall" fn comparar_cqq12_asia_recopa_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1985);
    cqq12_comparar_times("cqq12_asia_recopa_times", this, || o(this), || cqq12_asia_recopa_times(this))
}
