use super::*;

// ---------- sexagesima nona leva: clube na competicao ----------

/// { 0x48db80 } (comp.cpp): o clube esta na competicao? Antes, pela nacao: selecao (id a partir
/// de clubes - nacoes) de outro continente que o da competicao (+4 -> +0x59): nao. Clube da nacao
/// da competicao (+4 -> +0x5d) e da propria divisao (+0x57): sim; de nacao de outro continente: nao.
/// Depois percorre a competicao (-1) e as fases (+0xc, ate +0x30): a ultima de tipo 2 (+0x43, com
/// tabela de classificacao em +0xb1, linhas de 0x3b) ou, sem nenhuma ainda, a fase da vez (lista
/// de participantes em +0xa7, 6 bytes cada); quantos em +0x3e.
pub(crate) unsafe extern "thiscall" fn clube_na_competicao(this: usize, clube: usize) -> u8 {
    let base = *((this + 4) as *const usize);
    let d = |p: usize, o: usize| *((p + o) as *const usize);
    let limite = (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32));
    let nacao = d(clube, 0x53);
    if *(clube as *const i32) >= limite {
        let continente = d(base, 0x59);
        if continente != 0 && (nacao == 0 || d(nacao, 0x71) != continente) {
            return 0;
        }
    } else if nacao != 0 {
        if nacao == d(base, 0x5d) && d(clube, 0x57) != 0 && d(clube, 0x57) == base {
            return 1;
        }
        if d(nacao, 0x71) != d(base, 0x59) {
            return 0;
        }
    }
    let n = *((this + 0x30) as *const i32);
    let (mut grupo, mut outra) = (0usize, 0usize);
    let mut i = -1i32;
    while i <= n {
        let fase = if i == -1 { this } else { d(d(this, 0xc), 4 * i as usize) };
        if byte(fase, 0x43) == 2 {
            grupo = fase;
        } else {
            outra = fase;
        }
        let (f, tabela, passo) = if grupo != 0 { (grupo, 0xb1, 0x3b) } else { (outra, 0xa7, 6) };
        let quantos = *((f + 0x3e) as *const i16);
        let lista = d(f, tabela);
        let mut k = 0i16;
        while k < quantos {
            if d(lista, passo * k as usize) == clube {
                return 1;
            }
            k += 1;
        }
        i += 1;
    }
    0
}

/// { 0x76d940 } (ret 4): o clube e um dos nove guardados em 0x9cf78c, 0x9cf790, 0x9cf79c..0x9cf7ac,
/// 0x9cf8a0 e 0x9cf964 (pelo id)?
pub(crate) unsafe extern "stdcall" fn clube_especial(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let id = *(clube as *const u32);
    [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
        .iter()
        .any(|&a| *(a as *const u32) == id) as u32
}

// ---------- 281a leva: o fim da temporada das ligas: a cadeia de acessos e rebaixamentos; as notícias do prêmio e da TV de uma competição (league.cpp, comp.cpp) ----------


/// O slot 0xa4 da vtable da liga (thiscall, sem argumentos) [inferência: fecha a classificação final da temporada e
/// marca os que sobem e descem (+0x37 dos clubes), que a 0x689c80 lê].
pub(crate) unsafe fn cqp1_fechar(liga: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(liga) + 0xa4));
    f(liga)
}


// ponytail: fecha temporadas e move clubes entre as ligas (contratos, técnicos, notícias): não roda duas vezes; só a
// original. A cópia leva o ramo.

/// O técnico do clube é humano? (0x7875e0, stdcall de 1, já em Rust; o ecx = [0xb7431c] da original não conta.)
pub(crate) unsafe fn cqp1_humano(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    f(clube) != 0
}

/// the prize money news to the club's human manager, only when the prize exceeds 50 × the club reputation: the
/// competition, the amount, the position, the round, the kind and, with the fixture, the opponent and whether the club
/// won
/// Original: `Comp::send_prize_money_news` (0x48e3e0, comp.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x48e3e0 } (thiscall, ret 0x1c, comp.cpp, `Comp::send_prize_money_news(clube, valor (i64), posição, rodada, partida,
/// tipo)`; quadro SEH só pela notícia local): para o técnico humano do clube (0x7875e0), com o valor de pelo menos 50 ×
/// a reputação (+0x80), a notícia 0x1a (0x77bbe0): 0 a competição ([this+4]), 1 o valor, 2 o clube, 3 a posição, 4 a
/// rodada, 7 o tipo; com a partida, 5 o adversário e 6 se o clube venceu (+0x3d/+0x3e o vencedor nos pênaltis
/// [inferência], senão +0x4b/+0x4c, senão +0x43/+0x44 o placar); sem ela −1 e 0. Entregue a ele (0x7861e0).
pub(crate) unsafe extern "thiscall" fn noticia_do_premio(this: usize, clube: usize, lo: u32, hi: i32, posicao: u32, rodada: u32, partida: usize, tipo: u32) {
    if !cqp1_humano(clube) {
        return;
    }
    let minimo = *((clube + 0x80) as *const i16) as i64 * 50;
    let valor = ((hi as i64) << 32) | lo as i64;
    if valor < minimo {
        return;
    }
    crate::comparacao::ramo("cqp_1 noticia do premio");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1a, 0);
    dado(p, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
    dado(p, 1, lo);
    dado(p, 2, read_u32_as_usize(clube) as u32);
    dado(p, 3, posicao as u8 as i8 as i32 as u32);
    dado(p, 4, rodada as u16 as i16 as i32 as u32);
    dado(p, 7, tipo as u8 as i8 as i32 as u32);
    let b = |a: usize| *(a as *const i8);
    let venceu = if partida != 0 {
        if read_u32_as_usize(partida + 0x1c) == clube {
            dado(p, 5, read_u32_as_usize(read_u32_as_usize(partida + 0x20)) as u32);
            b(partida + 0x3d) != 0 || b(partida + 0x4b) > b(partida + 0x4c) || b(partida + 0x43) > b(partida + 0x44)
        } else {
            dado(p, 5, read_u32_as_usize(read_u32_as_usize(partida + 0x1c)) as u32);
            b(partida + 0x3e) != 0 || b(partida + 0x4b) < b(partida + 0x4c) || b(partida + 0x43) < b(partida + 0x44)
        }
    } else {
        dado(p, 5, u32::MAX);
        false
    };
    dado(p, 6, venceu as u32);
    entregar(read_u32_as_usize(0xb7431c), p, clube);
}

// ponytail: entrega a notícia (aloca); só a original. A cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_do_premio(this: usize, clube: usize, lo: u32, hi: i32, posicao: u32, rodada: u32, partida: usize, tipo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, usize, u32)>(1961)(this, clube, lo, hi, posicao, rodada, partida, tipo)
}

/// the TV rights news to the club's human manager: the club, the competition, the amount and the year
/// Original: `Comp::send_tv_rights_news` (0x48ea40, comp.cpp, thiscall, ret 8).
///
/// [pt] { 0x48ea40 } (thiscall, ret 8, comp.cpp, `Comp::send_tv_rights_news(clube, valor)`; quadro SEH só pela notícia local):
/// para o técnico humano do clube (0x7875e0), a notícia 0x1f (0x77bbe0): 0 o clube, 1 a competição ([this+4]), 2 o
/// valor, 3 o ano da competição (+0x40), entregue a ele (0x7861e0).
pub(crate) unsafe extern "thiscall" fn noticia_da_tv(this: usize, clube: usize, valor: u32) {
    if !cqp1_humano(clube) {
        return;
    }
    crate::comparacao::ramo("cqp_1 noticia da tv");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1f, 0);
    dado(p, 0, read_u32_as_usize(clube) as u32);
    dado(p, 1, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
    dado(p, 2, valor);
    dado(p, 3, *((this + 0x40) as *const i16) as i32 as u32);
    entregar(read_u32_as_usize(0xb7431c), p, clube);
}

// ponytail: entrega a notícia (aloca); só a original. A cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_tv(this: usize, clube: usize, valor: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1962)(this, clube, valor)
}

// ---------- 272a leva: a competição: todas as fases completas, a lista dos clubes e o progresso do clube (comp.cpp) ----------




/// in the top competition, whether it and every one down the chain are complete
/// Original: `Comp::all_stages_complete` (0x48ce10, comp.cpp, thiscall, ret 0).
///
/// [pt] { 0x48ce10 } (thiscall, ret 0, comp.cpp, método +0xc das competições): `Comp::all_stages_complete()`: só na
/// competição do topo (+0x1c ≤ −1), completa (método +0x10 com 1, 1: push 1; push 1; mov ecx -> thiscall, ret 8) ela e
/// cada uma da cadeia de baixo (+0x20, o índice em [0xadadfc]); devolve 1 ou 0 (no al).
pub(crate) unsafe extern "thiscall" fn competicao_todas_completas(this: usize) -> u8 {
    if (read_u32_as_usize(this + 0x1c) as i32) > -1 {
        return 0;
    }
    let completa = |c: usize| -> u8 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(c) + 0x10));
        f(c, 1, 1)
    };
    if completa(this) == 0 {
        return 0;
    }
    let mut i = read_u32_as_usize(this + 0x20) as i32;
    while i >= 0 {
        let c = read_u32_as_usize(read_u32_as_usize(0xadadfc) + i as usize * 4);
        if completa(c) == 0 {
            return 0;
        }
        crate::comparacao::ramo("cqh_6 liga de baixo completa");
        i = read_u32_as_usize(c + 0x20) as i32; // cqh6: a liga de baixo
    }
    1
}

/// Só a original (o método +0x10 marca a liga completa, grava recordes e paga prêmios: não roda duas vezes): a cópia
/// leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_competicao_todas_completas(this: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize) -> u8>(1655)(this)
}

/// the vector of the distinct clubs of all stages (up to 200) and the count
/// Original: `Comp::collect_club_list` (0x48e1c0, comp.cpp, thiscall, ret 8).
///
/// [pt] { 0x48e1c0 } (thiscall, ret 8, comp.cpp, método +0x58 das competições): `Comp::collect_club_list(&lista, _)`: um
/// vetor de 200 ponteiros (0x944e46(0x320); sem memória, a caixa da linha 0x681, a lista nula e 0) com os clubes
/// distintos de todas as fases (−1 a própria até +0x30; as outras em [+0xc]): numa liga (+0x43 = 2) os da tabela +0xb1
/// (linhas de 0x3b bytes), noutra os de +0xa7 (6 bytes), sem os nulos e os repetidos, até 200; grava o vetor em
/// `lista` e devolve a conta (word, no ax).
pub(crate) unsafe extern "thiscall" fn competicao_lista_de_clubes(this: usize, lista: *mut usize, _x: u32) -> u16 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alocar(0x320);
    if v == 0 {
        aviso_em(0x99b80c, 0x681);
        *lista = 0;
        return 0;
    }
    let mut n = 0i16;
    let mut s = -1i8;
    while s as i32 <= read_u32_as_usize(this + 0x30) as i32 {
        let f = if s == -1 { this } else { read_u32_as_usize(read_u32_as_usize(this + 0xc) + (s as i32 * 4) as usize) };
        let liga = read_u8(f + 0x43) == 2;
        let mut i = 0i16;
        while i < read_i16(f + 0x3e) {
            let clube = if liga { read_u32_as_usize(read_u32_as_usize(f + 0xb1) + i as usize * 0x3b) } else { read_u32_as_usize(read_u32_as_usize(f + 0xa7) + i as usize * 6) };
            if clube != 0 && !(0..n).any(|k| read_u32_as_usize(v + k as usize * 4) == clube) && n < 0xc8 { // cqh6: até 200 clubes
                crate::comparacao::ramo("cqh_6 clube na lista");
                core::ptr::write_unaligned((v + n as usize * 4) as *mut u32, clube as u32); // cqh6: o clube na lista
                n += 1;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    *lista = v;
    n as u16
}

/// Lado a lado: a original com a saída de quem chama, a cópia com saída própria, a partir do mesmo [0xb67a34]; a conta
/// e os clubes das duas listas iguais (a da cópia é liberada).
pub(crate) unsafe extern "thiscall" fn comparar_competicao_lista_de_clubes(this: usize, lista: *mut usize, x: u32) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(1656);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut l2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, lista, x), || competicao_lista_de_clubes(this, &mut l2, x));
    let mesmas = a == b && (*lista == 0) == (l2 == 0) && (*lista == 0 || core::slice::from_raw_parts(*lista as *const u32, a as usize) == core::slice::from_raw_parts(l2 as *const u32, b as usize));
    if l2 != 0 {
        soltar(l2);
    }
    PLACAR.registrar(igual && mesmas, || format!("competicao_lista_de_clubes: conta original={} rust={}", a, b));
    a
}
/// the stage id, the status and the round the club reached, from the last stage back
/// Original: `Comp::find_club_progress` (0x48cc70, comp.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x48cc70 } (thiscall, ret 0x10, comp.cpp): `Comp::find_club_progress(clube, &fase, &rodada, &situação)`: das
/// fases da última (+0x30) para a primeira (−1 a própria; as outras em [+0xc]), pulando as de id 0x3fc: numa liga
/// (+0x43 = 2), a linha da tabela do clube (0x6826d0 push i; mov ecx, fase -> thiscall, ret 4): fase = o id (+0x3a) e
/// situação = +0x37 da linha, e volta; numa copa, o último time da tabela +0xa7 (6 bytes) que é o clube dá fase = o id
/// e situação = +5, e a rodada é a da primeira rodada (de +0xab − 1 para trás, +0xa3 com 0x68 bytes, os confrontos de
/// +0xad com 0x35 bytes) com um confronto em que o time joga (+0 ou +2): rodada = +7 da rodada, e volta.
pub(crate) unsafe extern "thiscall" fn competicao_progresso_do_clube(this: usize, clube: usize, fase_id: *mut i16, rodada: *mut i16, situacao: *mut u8) {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let mut f = read_u32_as_usize(this + 0x30) as i32;
    while f >= -1 {
        let s = if f > -1 { read_u32_as_usize(read_u32_as_usize(this + 0xc) + f as usize * 4) } else { this };
        if read_i16(s + 0x3a) != 0x3fc {
            if read_u8(s + 0x43) == 2 {
                let mut i = 0i8;
                while (i as i16) < read_i16(s + 0x3e) {
                    if read_u32_as_usize(linha(s, i as i32)) == clube {
                        *fase_id = read_i16(s + 0x3a);
                        *situacao = read_u8(linha(s, i as i32) + 0x37);
                        return;
                    }
                    i = i.wrapping_add(1);
                }
            } else {
                let mut time: i16 = -1;
                let mut k = 0i16;
                while k < read_i16(s + 0x3e) {
                    let t = read_u32_as_usize(s + 0xa7) + k as usize * 6;
                    if read_u32_as_usize(t) == clube {
                        *fase_id = read_i16(s + 0x3a);
                        crate::comparacao::ramo("cqh_6 progresso na copa");
                        *situacao = read_u8(t + 5); // cqh6: a situação do time
                        time = k;
                    }
                    k += 1;
                }
                if time != -1 {
                    let mut r = (read_u8(s + 0xab) as i8 as i16).wrapping_sub(1);
                    while r >= 0 {
                        let n = read_i16(read_u32_as_usize(s + 0xa3) + r as usize * 0x68 + 0x1a);
                        let confrontos = read_u32_as_usize(read_u32_as_usize(s + 0xad) + r as usize * 4);
                        let mut j = 0i16;
                        while j < n {
                            let c = confrontos.wrapping_add((j as i32 * 0x35) as usize);
                            if read_i16(c) == time || read_i16(c + 2) == time {
                                *rodada = read_i16(read_u32_as_usize(s + 0xa3) + r as usize * 0x68 + 7);
                                return;
                            }
                            j += 1;
                        }
                        r -= 1;
                    }
                }
            }
        }
        f -= 1;
    }
}

/// Lado a lado nas três saídas (a original e a cópia com saídas próprias, começando dos valores de quem chama).
pub(crate) unsafe extern "thiscall" fn comparar_competicao_progresso_do_clube(this: usize, clube: usize, fase_id: *mut i16, rodada: *mut i16, situacao: *mut u8) {
    let (mut f2, mut r2, mut s2) = (*fase_id, *rodada, *situacao);
    orig::<unsafe extern "thiscall" fn(usize, usize, *mut i16, *mut i16, *mut u8)>(1657)(this, clube, fase_id, rodada, situacao);
    competicao_progresso_do_clube(this, clube, &mut f2, &mut r2, &mut s2);
    let ok = f2 == *fase_id && r2 == *rodada && s2 == *situacao;
    PLACAR.registrar(ok, || format!("competicao_progresso_do_clube: original ({}, {}, {}) rust ({}, {}, {})", *fase_id, *rodada, *situacao, f2, r2, s2));
}

// ---------- 267a leva: a copa e as fases: a árvore dos confrontos da copa, a primeira fase do grupo e a fase atual (Cup.cpp, comp.cpp) ----------







/// O confronto `p` (0x35 bytes) da rodada `r` da copa (vetores em [+0xad]).
pub(crate) unsafe fn cqd7_confronto(copa: usize, r: i32, p: i32) -> usize {
    read_u32_as_usize(read_u32_as_usize(copa + 0xad) + (r as isize * 4) as usize).wrapping_add((p as isize * 0x35) as usize)
}

/// A rodada `r` da copa (0x68 bytes na tabela [+0xa3]).
pub(crate) unsafe fn cqd7_rodada(copa: usize, r: i32) -> usize {
    read_u32_as_usize(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}



/// A fase `k` da competição como a original a lê no meio das rotinas: −1 a própria, 0..+0x30 em [+0xc]; fora disso a
/// caixa da linha 0x36e de comp.cpp e 0.
pub(crate) unsafe fn cqd7_fase(comp: usize, k: i32) -> usize {
    if k < -1 || k > read_u32_as_usize(comp + 0x30) as i32 {
        aviso_em(0x99b80c, 0x36e);
        return 0;
    }
    if k == -1 { comp } else { read_u32_as_usize(read_u32_as_usize(comp + 0xc) + k as usize * 4) }
}

/// from the given stage backwards, the index of the first stage of the same group
/// Original: `Comp::get_first_stage_of_group` (0x48d2b0, comp.cpp, thiscall, ret 4).
///
/// [pt] { 0x48d2b0 } (thiscall, ret 4, comp.cpp): `Comp::get_first_stage_of_group(k)`, das telas das competições
/// (0x4947a0): da fase `k` para trás, enquanto as fases têm o mesmo grupo (0x4b01d0 do id +0x3a: push id; add esp, 4 ->
/// cdecl) e o índice é maior que −1, o índice da primeira delas (o próprio k sem grupo).
pub(crate) unsafe extern "thiscall" fn competicao_primeira_fase_do_grupo(this: usize, k: i32) -> i32 {
    let grupo: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_COMP_UTIL_GET_PARENT_STAGE_ID);
    let mut e = k;
    let g = grupo(read_i16(cqd7_fase(this, e) + 0x3a) as i32);
    if g > -1 && e > -1 {
        loop {
            let f = cqd7_fase(this, e - 1);
            if grupo(read_i16(f + 0x3a) as i32) != g {
                break;
            }
            e -= 1;
            if e <= -1 {
                break;
            }
        }
    }
    e
}

/// O comparador da primeira fase do grupo: com o índice válido (−1..+0x30), as duas (só leem) e o mesmo índice; fora
/// dele só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_competicao_primeira_fase_do_grupo(this: usize, k: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> i32>(1559);
    if k < -1 || k > read_u32_as_usize(this + 0x30) as i32 {
        return o(this, k);
    }
    let r = o(this, k);
    let n = competicao_primeira_fase_do_grupo(this, k);
    PLACAR.registrar(r == n, || format!("competicao_primeira_fase_do_grupo({}): original={} rust={}", k, r, n));
    r
}

/// the first stage not finished (in a cup not yet started, the start of its group); all finished, the start of the last
/// one's group
/// Original: `Comp::get_current_stage` (0x48cec0, comp.cpp, thiscall, ret 0).
///
/// [pt] { 0x48cec0 } (thiscall, ret 0, comp.cpp): `Comp::get_current_stage()`, das telas das competições (0x48f8e0): a
/// primeira fase (−1 a própria, até +0x30) que não acabou (método 0x10 com (0, 1) = 0); se é uma copa (+0x43 = 1) que
/// não começou (+0xab = 0), volta ao começo do grupo dela pela fase anterior (0x4b01d0 e 0x48d200: push k; mov ecx,
/// this -> thiscall, ret 4). Todas acabadas: o começo do grupo da última. Fase inválida no fim: a caixa da linha 0x36e
/// e a própria competição; fase nula: a própria.
pub(crate) unsafe extern "thiscall" fn competicao_fase_atual(this: usize) -> usize {
    let grupo: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_COMP_UTIL_GET_PARENT_STAGE_ID);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_COMP_GET_STAGE);
    let voltar = |s0: i32| -> i32 {
        let mut s = s0;
        let g = grupo(read_i16(cqd7_fase(this, s) + 0x3a) as i32);
        if g > -1 && s > -1 {
            loop {
                let f = fase(this, s - 1);
                if grupo(read_i16(f + 0x3a) as i32) != g {
                    break;
                }
                s -= 1;
                if s <= -1 {
                    break;
                }
            }
        }
        s
    };
    let fim = |s: i32| -> usize {
        if s < -1 || s > read_u32_as_usize(this + 0x30) as i32 {
            aviso_em(0x99b80c, 0x36e);
            return this;
        }
        let f = if s == -1 { this } else { read_u32_as_usize(read_u32_as_usize(this + 0xc) + s as usize * 4) };
        if f != 0 { f } else { this }
    };
    let mut e: i32 = -1;
    if read_u32_as_usize(this + 0x30) as i32 >= -1 {
        loop {
            let f = cqd7_fase(this, e);
            let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(f) + 0x10));
            if acabou(f, 0, 1) as u8 == 0 {
                if f == 0 {
                    break;
                }
                if read_u8(f + 0x43) == 1 && read_u8(f + 0xab) == 0 && e > -1 {
                    return fim(voltar(e - 1));
                }
                return f;
            }
            e += 1;
            if e > read_u32_as_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    fim(voltar(read_u32_as_usize(this + 0x30) as i32))
}

/// O comparador da fase atual: só a original (ponytail: o método 0x10 das fases pode refazer a tabela; não roda duas
/// vezes); a cópia se prova no modo Rust pelas telas.
pub(crate) unsafe extern "thiscall" fn comparar_competicao_fase_atual(this: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(1560)(this)
}

// ---------- 264a leva: as nações-chave das competições: a do clube, a do estádio e a entrada da tabela pelo índice ativo (comp.cpp) ----------

/// A entrada `k` (72 bytes) da tabela das 35 nações-chave em 0xb63d60: +0 a nação, +0x14 o índice ativo.
pub(crate) fn cqb12_entrada(k: i8) -> usize {
    0xb63d60 + k as i32 as usize * 72
}

/// O índice `k` da nação `nacao` na tabela das nações-chave (entrada com nação ativa, +0x11c da nação), ou None.
pub(crate) unsafe fn cqb12_achar(nacao: usize) -> Option<i8> {
    let mut k: i8 = 0;
    while k < 0x23 {
        let e = *(cqb12_entrada(k) as *const usize);
        if e != 0 && *((e + 0x11c) as *const u8) != 0 && e == nacao {
            return Some(k);
        }
        k += 1;
    }
    None
}

/// the active index of the club's nation among the active key nations; otherwise 0 for a national team and 3 for a club
/// (the type used by the fixture search and the date clash check)
/// Original: `comp_active_key_nation_of_club` (0x48dee0, comp.cpp, cdecl).
///
/// [pt] { 0x48dee0 } (cdecl, comp.cpp): `comp_active_key_nation_of_club(clube)`: o índice ativo (+0x14 da entrada) da
/// nação do clube (+0x53) entre as nações-chave ativas; sem nação ou fora da tabela, 0 para uma seleção (0x53b900) e 3
/// para um clube. Devolve só `al` (os chamadores gravam o byte).
pub(crate) unsafe extern "C" fn nacao_chave_do_clube(clube: usize) -> u8 {
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao != 0 {
        if let Some(k) = cqb12_achar(nacao) {
            crate::comparacao::ramo("cqb_12 nacao chave do clube");
            return *((cqb12_entrada(k) + 0x14) as *const u8); // cqb12: o índice ativo da nação do clube
        }
    }
    // 0x53b900: push clube; add esp, 4 -> cdecl
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqb_12 clube sem nacao chave");
    3 // cqb12: clube fora das nações-chave
}

/// Comparador: rotina pura; compara o byte devolvido.
pub(crate) unsafe extern "C" fn comparar_nacao_chave_do_clube(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1491)(clube);
    let n = nacao_chave_do_clube(clube);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_clube({:#x}) original={} rust={}", clube, r, n));
    r
}

/// the active index of the stadium city's nation among the active key nations, or 3
/// Original: `comp_active_key_nation_of_stadium` (0x48df40, comp.cpp, cdecl).
///
/// [pt] { 0x48df40 } (cdecl, comp.cpp): `comp_active_key_nation_of_stadium(estadio)`: o índice ativo da nação da cidade
/// do estádio (+0x38 a cidade, +0x1f a nação) entre as nações-chave ativas; sem estádio, cidade, nação ou fora da
/// tabela, 3. Devolve só `al`.
pub(crate) unsafe extern "C" fn nacao_chave_do_estadio(estadio: usize) -> u8 {
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    if estadio == 0 || u(estadio + 0x38) == 0 || u(u(estadio + 0x38) + 0x1f) == 0 {
        return 3;
    }
    match cqb12_achar(u(u(estadio + 0x38) + 0x1f)) {
        Some(k) => {
            crate::comparacao::ramo("cqb_12 nacao chave do estadio");
            *((cqb12_entrada(k) + 0x14) as *const u8) // cqb12: o índice ativo da nação do estádio
        }
        None => 3,
    }
}

/// Comparador: rotina pura; compara o byte devolvido.
pub(crate) unsafe extern "C" fn comparar_nacao_chave_do_estadio(estadio: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1492)(estadio);
    let n = nacao_chave_do_estadio(estadio);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_estadio({:#x}) original={} rust={}", estadio, r, n));
    r
}

/// A entrada da tabela das nações-chave cujo índice ativo (+0x14) é `ativo`, se houver.
pub(crate) unsafe fn cqb12_pelo_ativo(ativo: i8) -> Option<i8> {
    (0..0x23i8).find(|&k| *((cqb12_entrada(k) + 0x14) as *const i8) == ativo)
}

/// the first key nation table entry with that active index; none, the error box and 3
/// Original: `comp_key_nation_entry_from_active_index` (0x48dcd0, comp.cpp, cdecl).
///
/// [pt] { 0x48dcd0 } (cdecl, comp.cpp): `comp_key_nation_entry_from_active_index(ativo)`: a primeira entrada da tabela das
/// nações-chave com aquele índice ativo; nenhuma, a caixa da linha 0x543 de comp.cpp (o jogo segue) e 3.
pub(crate) unsafe extern "C" fn entrada_da_nacao_chave_ativa(ativo: i8) -> i32 {
    match cqb12_pelo_ativo(ativo) {
        Some(k) => k as i32,
        None => {
            aviso_em(0x99b80c, 0x543); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp.cpp"
            3
        }
    }
}

/// Comparador: rotina pura; compara a entrada devolvida. Sem entrada (a caixa): só a original.
pub(crate) unsafe extern "C" fn comparar_entrada_da_nacao_chave_ativa(ativo: i8) -> i32 {
    let o = orig::<unsafe extern "C" fn(i8) -> i32>(1493);
    if cqb12_pelo_ativo(ativo).is_none() {
        return o(ativo); // ponytail: a caixa não roda duas vezes
    }
    let r = o(ativo);
    let n = entrada_da_nacao_chave_ativa(ativo);
    PLACAR.registrar(r == n, || format!("entrada_da_nacao_chave_ativa({}) original={} rust={}", ativo, r, n));
    r
}

// ---------- 259a leva: os convocados por clube na data do jogo da seleção, a importância do jogo e a fase da competição (national_teams.cpp, comp.cpp) ----------


/// O registro de convocações do clube `c` em [this + 0x20] (6 bytes: +0 vetor de entradas de 9 bytes, +4 capacidade,
/// +5 quantas), se o clube conta: id dentro dos [0xae235c] − 2 × [0xae2350] registros, com nação (+0x53) marcada
/// (+0x11c) e que não é a de [0x9cf260].
pub(crate) unsafe fn cpw13_registro(this: usize, c: usize) -> Option<usize> {
    let n = (read_usize(0xae235c) as i32).wrapping_sub((read_usize(0xae2350) as i32).wrapping_mul(2));
    let id = read_usize(c) as i32;
    if id >= n || id < 0 {
        return None;
    }
    let nacao = read_usize(c + 0x53);
    if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 || read_usize(nacao) == read_usize(KEY_NATION_BRAZIL) {
        return None;
    }
    Some(read_usize(this + 0x20).wrapping_add(6 * id as usize))
}

/// Soma `conta` convocados do clube `c` na data `d` (dia +0, ano +2): na entrada da mesma data, se houver; senão uma
/// entrada nova (com o vetor cheio, mais 3 vagas por 0x945667) com a data e a conta, e o vetor reordenado pela data
/// (qsort 0x945333 com 0x778e90). Sem memória: a caixa (linha 0x21c8) e false (a rotina para).
pub(crate) unsafe fn cpw13_somar(this: usize, c: usize, conta: u8, d: *mut u8) -> bool {
    let rec = match cpw13_registro(this, c) {
        Some(r) => r,
        None => return true,
    };
    let quantas = *((rec + 5) as *const i8);
    let (dia, ano) = (core::ptr::read_unaligned(d as *const u16), core::ptr::read_unaligned((d as usize + 2) as *const u16));
    let mut k: i8 = 0;
    while k < quantas {
        let e = read_usize(rec).wrapping_add(9 * k as i32 as usize);
        if core::ptr::read_unaligned((e + 2) as *const u16) == ano && core::ptr::read_unaligned(e as *const u16) == dia {
            crate::comparacao::ramo("cpw_13 soma na data");
            *((e + 8) as *mut u8) = (*((e + 8) as *const u8)).wrapping_add(conta); // cpw13: soma na data
            return true;
        }
        k += 1;
    }
    if quantas == *((rec + 4) as *const i8) {
        let cap = (*((rec + 4) as *const u8)).wrapping_add(3);
        *((rec + 4) as *mut u8) = cap;
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let v = realocar(read_usize(rec), (cap as i8 as i32 * 9) as u32);
        core::ptr::write_unaligned(rec as *mut usize, v);
        if v == 0 {
            aviso_em(0xa5b2a4, 0x21c8);
            return false;
        }
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let q = *((rec + 5) as *const i8) as i32;
    let e = read_usize(rec).wrapping_add(9 * q as usize);
    let mut t = [0u8; 8];
    copia(e, t.as_mut_ptr(), d);
    crate::comparacao::ramo("cpw_13 registro novo");
    *((e + 8) as *mut u8) = conta; // cpw13: a conta do registro novo
    let q = (*((rec + 5) as *const i8)).wrapping_add(1);
    *((rec + 5) as *mut i8) = q;
    qsort(read_usize(rec), q as i32 as u32, 9, 0x778e90);
    true
}


/// O registro de algum clube que a 0x7771c0 pode somar está com o vetor cheio (capacidade = quantas, inclusive sem
/// vetor)? Então ela pode realocar.
pub(crate) unsafe fn cpw13_pode_realocar(this: usize, time: usize) -> bool {
    (0..0x32usize).any(|i| {
        let s = read_usize(time + 0xd7 + 4 * i);
        if s == 0 || read_usize(s) as i32 >= read_usize(0xae2364) as i32 || read_usize(s + 0x39) == 0 {
            return false;
        }
        match cpw13_registro(this, read_usize(s + 0x39)) {
            Some(rec) => *((rec + 4) as *const i8) == *((rec + 5) as *const i8),
            None => false,
        }
    })
}


/// forwards the match importance query to the fixture's stage (virtual +0x54)
/// Original: `Comp::get_fixture_match_importance` (0x48e190, comp.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x48e190 } (thiscall, ret 0xc, comp.cpp): `Comp::get_fixture_match_importance(partida, a, b)`: a fase da partida
/// (+0x42; 0xff = a própria competição, senão [+0xc] + 4 × fase) responde pela virtual +0x54 com os mesmos argumentos.
pub(crate) unsafe extern "thiscall" fn importancia_do_jogo_na_competicao(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let fase = *((p + 0x42) as *const u8);
    let c = if fase == 0xff { this } else { read_usize(read_usize(this + 0xc).wrapping_add(4 * (fase as i8 as i32) as usize)) };
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(c) + 0x54));
    f(c, p, a, b)
}

/// Comparador: pelo valor (a virtual da fase só lê; roda duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_importancia_do_jogo_na_competicao(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1306)(this, p, a, b);
    let n = importancia_do_jogo_na_competicao(this, p, a, b);
    PLACAR.registrar(r == n, || format!("importancia_do_jogo_na_competicao({:x}, partida {:x}, {}, {}) original={} rust={}", this, p, a, b, r, n));
    r
}

/// returns stage k of the competition (-1 = itself); out of range, the error box and 0
/// Original: `Comp::get_stage` (0x48d200, comp.cpp, thiscall, ret 4).
///
/// [pt] { 0x48d200 } (thiscall, ret 4, comp.cpp): `Comp::get_stage(k)`: −1 a própria competição, 0..+0x30 a fase
/// [+0xc] + 4 × k; fora disso a caixa (linha 0x36e) e 0, pela original.
pub(crate) unsafe extern "thiscall" fn fase_da_competicao(this: usize, k: i32) -> usize {
    if k < -1 || k > read_usize(this + 0x30) as i32 {
        return orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(1307)(this, k);
    }
    if k == -1 {
        return this;
    }
    read_usize(read_usize(this + 0xc).wrapping_add(4 * k as usize))
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_fase_da_competicao(this: usize, k: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(1307);
    if k < -1 || k > read_usize(this + 0x30) as i32 {
        return o(this, k); // ponytail: a caixa, só a original
    }
    let r = o(this, k);
    let n = fase_da_competicao(this, k);
    PLACAR.registrar(r == n, || format!("fase_da_competicao({:x}, {}) original={:x} rust={:x}", this, k, r, n));
    r
}

/// the active key of the key nation that owns the fixture: by the competition nation, by the nations of the friendly
/// clubs, 0 for the World Cup, 1 for national side tournaments
/// Original: `comp_active_key_nation_of_fixture` (0x48dd70, comp.cpp, cdecl).
///
/// [pt] { 0x48dd70 } (cdecl, comp.cpp): `comp_active_key_nation_of_fixture(competição, partida)`, a chave das listas de jogos
/// do FIXTURE_MANAGER (add_fixture 0x5aa680, find_fixtures 0x5acbe0, ...), da simulação rápida (0x6b66a0) e do peso do
/// ranking da FIFA (0x58cf70). Amistoso (0x5c6710 sobre [0xae2c74]) ou reservas ([0x9cf684]) com a partida dada e a
/// competição dela amistosa ([0xae2c74]+4) ou de reservas: com uma das duas seleções (0x53b900 dos clubes +0x1c/+0x20),
/// 1; nações (+0x53) diferentes ou fora da escolha (+0x11c), 3; a nação na tabela das 35 nações-chave (0xb63d60, 72
/// bytes cada, escolhida em +0x11c), o byte +0x14 da linha; senão 0 com o mandante seleção e 3 sem. Senão, competição de
/// nação (+0x5d): o byte +0x14 da linha dela, ou 3. Sem nação: Copa do Mundo e eliminatórias (0x669a10), 0; torneio
/// internacional (0x669ae0), 1; senão o índice da nação-chave (0x669470). Chamada a cada jogo procurado: sem contador.
pub(crate) unsafe extern "C" fn nacao_chave_do_jogo(comp: usize, partida: usize) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha = |nacao: usize| (0..0x23usize).find(|&k| {
        let n = read_usize(0xb63d60 + 72 * k);
        n != 0 && *((n + 0x11c) as *const u8) != 0 && n == nacao
    });
    let nacao = read_usize(comp + 0x5d);
    let reservas = read_usize(KEY_CLUB_COMP_RESERVE);
    if (amistoso(read_usize(0xae2c74), comp) != 0 || read_usize(comp) == reservas) && partida != 0 {
        let pc = read_usize(partida + 0x14);
        if pc == read_usize(read_usize(0xae2c74) + 4) || read_usize(pc) == reservas {
            if selecao(read_usize(partida + 0x1c)) != 0 || selecao(read_usize(partida + 0x20)) != 0 {
                return 1;
            }
            let casa = read_usize(partida + 0x1c);
            let a = read_usize(casa + 0x53);
            if a != read_usize(read_usize(partida + 0x20) + 0x53) || *((a + 0x11c) as *const u8) == 0 {
                return 3;
            }
            if let Some(k) = linha(a) {
                return *((0xb63d74 + 72 * k) as *const u8); // cpt3: a nacao do amistoso
            }
            return if selecao(casa) != 0 { 0 } else { 3 };
        }
    }
    if read_usize(comp + 0x5d) != 0 {
        return match linha(nacao) {
            Some(k) => *((0xb63d74 + 72 * k) as *const u8), // cpt3: a nacao-chave da competicao
            None => 3,
        };
    }
    let copa: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_IS_WORLD_CUP_COMP);
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_IS_INTERNATIONAL_COMP);
    let indice: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_COMP);
    if copa(comp) != 0 {
        return 0; // cpt3: Copa do Mundo e eliminatorias
    }
    if internacional(comp) != 0 {
        return 1; // cpt3: torneio internacional
    }
    indice(comp)
}

pub(crate) unsafe extern "C" fn comparar_nacao_chave_do_jogo(comp: usize, partida: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(1161)(comp, partida);
    let n = nacao_chave_do_jogo(comp, partida);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_jogo(competição {}, partida {:x}) original={} rust={}", read_usize(comp) as i32, partida, r, n));
    r
}

/// the stage is a league, or the round is before the current one, or is the current last round with code 0x8c
/// Original: `Comp::are_round_fixtures_known` (0x48d410, comp.cpp, thiscall, ret 8).
///
/// [pt] { 0x48d410 } (thiscall, ret 8, comp.cpp): `Comp::are_round_fixtures_known(fase, rodada)`: a fase (−1 = a própria
/// competição; acima de +0x30: 0; abaixo de −1: a caixa da linha 0x36e, pela original) é liga (+0x43 = 2): 1. Senão 1
/// se a rodada é antes da atual (+0xab, byte) ou é a atual, a última (+0x3c − 1) e com o código 0x8c (+7 do registro
/// de 0x68 bytes dela em +0xa3); senão 0.
pub(crate) unsafe extern "thiscall" fn rodada_com_jogos_conhecidos(this: usize, fase: i32, rodada: i32) -> u32 {
    if fase > read_usize(this + 0x30) as i32 {
        return 0;
    }
    if fase < -1 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1304)(this, fase, rodada);
    }
    let c = if fase == -1 { this } else { read_usize(read_usize(this + 0xc).wrapping_add(4 * fase as usize)) };
    if *((c + 0x43) as *const u8) == 2 {
        return 1;
    }
    let atual = *((c + 0xab) as *const i8) as i32;
    if rodada < atual {
        return 1;
    }
    if rodada != atual {
        return 0;
    }
    let n = core::ptr::read_unaligned((c + 0x3c) as *const i16) as i32;
    if atual != n - 1 {
        return 0;
    }
    let cod = core::ptr::read_unaligned((read_usize(c + 0xa3).wrapping_add((0x68 * (n - 1)) as usize) + 7) as *const i16);
    (cod == 0x8c) as u32
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_rodada_com_jogos_conhecidos(this: usize, fase: i32, rodada: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1304);
    if fase < -1 && fase <= read_usize(this + 0x30) as i32 {
        return o(this, fase, rodada); // ponytail: a caixa, só a original
    }
    let r = o(this, fase, rodada);
    let n = rodada_com_jogos_conhecidos(this, fase, rodada);
    PLACAR.registrar(r == n, || format!("rodada_com_jogos_conhecidos({:x}, {}, {}) original={} rust={}", this, fase, rodada, r, n));
    r
}

/// 1 when the match round (+0x30) ends in 0 or is above 999, else 0
/// Original: `Comp::counts_for_fixture_clash` (0x48e360, comp.cpp, thiscall, ret 8).
///
/// [pt] { 0x48e360 } (thiscall, ret 8, comp.cpp): `Comp::counts_for_fixture_clash(jogo, outro)` (a virtual +0x60 das
/// competições, do conflito de datas 0x5adba0): 1 se o algarismo das unidades (0x4b0850) da rodada (+0x30, com sinal)
/// é 0 (rodada de 1000 em diante também dá 0), senão 0. `outro` não é lido.
pub(crate) unsafe extern "thiscall" fn conta_para_conflito_de_datas(_this: usize, jogo: usize, _outro: usize) -> u32 {
    let unidades: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_ROUND_VARIANT);
    (unidades(read_i16(jogo + 0x30) as i32) == 0) as u32
}

pub(crate) unsafe extern "thiscall" fn comparar_conta_para_conflito_de_datas(this: usize, jogo: usize, outro: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1361)(this, jogo, outro);
    let n = conta_para_conflito_de_datas(this, jogo, outro);
    PLACAR.registrar(r == n, || format!("conta_para_conflito_de_datas(rodada {}) original={} rust={}", read_i16(jogo + 0x30), r, n));
    r
}
