use super::*;

/// The club's nation (+0x53).
///
/// [pt] A nação do clube (+0x53).
pub(crate) unsafe fn club_nation(club: Club) -> usize {
    club.nation().addr()
}

/// Draws the Champions League first group phase: the 32 clubs sorted by pot, same-nation partners in opposite halves,
/// the draw of the 8 groups pot by pot, the swap of same-nation clubs within a group, the coefficient, the 1,800,000
/// prize and its news, the 8 stages, and the third qualifying round losers into the UEFA Cup.
///
/// SEH frame only for the stage's `new`: no destructor on the normal path. `EuropeanCup::draw_phase1_groups()`, called
/// from the Champions League's next-stage routine (`european_cup_advance_stage`, stage 0).
///
/// 1) The list of the 32 (`exe::crt_calloc`(0x20, 9): push 9; push 0x20; add esp, 8 -> cdecl; 9 bytes: +0 the club, +4 the
/// same-nation partner, +8 the half: 1 groups A..D, 2 groups E..H; out of memory, the error box at line 0x4cc): the
/// first 16 of table `[+0x14]` (6 bytes) and the teams of table `[+0xa7]` (6 bytes) with +5 = 1 (the third qualifying
/// round winners). Fewer than 32: the error box at line 0x4ee, +0x30 = −1, the list freed and +0xb1 = 0.
///
/// 2) The list is sorted by pot (qsort `exe::crt_qsort` with `european_cup_compare_clubs_by_seed`: the club's +0x1df and the reputation) and each club
/// without a partner gets the first following club without a partner from the same nation (the two point at each
/// other).
///
/// 3) The groups (`exe::crt_calloc`(0x20, 4): 8 groups × 4 pots; out of memory, the error box at line 0x515 and the game stops):
/// for each pot of 8 (k = 0..3), 4 slots start free in each half, minus those already marked; 8 times a club of the pot
/// is drawn (`exe::utils_random`(8) + 8k, drawn again while the slot is already taken); without a half, one is drawn
/// (`exe::utils_random`(free A + free B) < free B: the second) and the partner, if it is in the same pot, is put in the other half
/// (with no slot there, it is not marked); the club enters the first group of the half with the pot free.
///
/// 4) In each half and pot, a club with another club of the same nation in its group swaps places with the same-pot
/// club of the first other group in the half without its nation.
///
/// 5) Each group: the 4 clubs earn 1 point of the nation coefficient (0x9058b0 on `[0xde1f58]`: push 1; push nation ->
/// thiscall, ret 8), the 1,800,000 prize (`get_club_finance` on `[0xae2a38]` push club -> thiscall, ret 4; `add_prize_money` push amount;
/// mov ecx, finances -> thiscall, ret 4) and the prize news (`send_prize_money_news`: push −2; push 0; push 0x449; push −1; push 0;
/// push amount; push club; mov ecx, this -> thiscall, ret 0x1c); the round table from method +0x3c (push 0; push &id;
/// push &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xee) and the League_Stage
/// constructor 0x68b100: this, 4, the group, 2, +4, the table, the rounds (word), 3, 1, 8, &{4,1,2,0}, &{2,0,0,0}, the
/// year (word), g, the id (word), 0xf, 2, 0, 8, −1, 0, 2; the original pushes the year with garbage in the high bytes
/// of edx, which the constructor does not read: the copy passes the clean year) into `[+0xc][g]` (without a stage, the
/// error box at line 0x5c9 and it returns, without freeing the table or the groups); the round table freed, +0x30 = g
/// and the group news (`send_group_draw_news`).
///
/// 6) The groups are freed; the teams of +0xa7 with +5 = 2 (the third qualifying round losers) go to the UEFA Cup
/// (`[[0xadadfc] + 4 × [0x9cf6f0]]`; without it, the error box at line 0x5d8): into the last 24 slots of its +0xa7
/// table (from +0x3e − 24), with +4 and +5 cleared, into its +0x14 table with status 5 (+0x36 goes up) and with the
/// club's European competition (+0x1db) = `[0x9cf6f0]`; then the UEFA Cup's predicted positions (its method +0x5c).
/// Original: `EuropeanCup::draw_phase1_groups` (0x585290, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x585290 } (thiscall, ret 0, european_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `EuropeanCup::draw_phase1_groups()`, da fase seguinte da Liga dos Campeões (0x5851d0, fase 0).
/// 1) A lista dos 32 (0x945501(0x20, 9) push 9; push 0x20; add esp, 8 -> cdecl; 9 bytes: +0 o clube, +4 o parceiro da
///    mesma nação, +8 a metade: 1 grupos A..D, 2 grupos E..H; sem memória, a caixa da linha 0x4cc): os 16 primeiros da
///    tabela [+0x14] (6 bytes) e os times da tabela [+0xa7] (6 bytes) com +5 = 1 (os vencedores da terceira
///    preliminar). Menos de 32: a caixa da linha 0x4ee, +0x30 = −1, a lista liberada e +0xb1 = 0.
/// 2) A lista ordenada pelo pote (qsort 0x945333 com 0x586f60: +0x1df do clube e a reputação) e cada clube sem parceiro
///    ganha o primeiro seguinte sem parceiro da mesma nação (os dois se apontam).
/// 3) Os grupos (0x945501(0x20, 4): 8 grupos × 4 potes; sem memória, a caixa da linha 0x515 e o jogo para): para cada
///    pote de 8 (k = 0..3) começam livres 4 vagas em cada metade, menos as já marcadas; 8 vezes um clube do pote
///    sorteado (0x90cfc0(8) + 8k, de novo enquanto a vaga já saiu); sem metade, sorteia uma (0x90cfc0(livres A +
///    livres B) < livres B: a segunda) e o parceiro, se é do mesmo pote, fica na outra metade (sem vaga lá, não marca);
///    entra no primeiro grupo da metade com o pote livre.
/// 4) Em cada metade e pote, um clube com outro da mesma nação no grupo troca de lugar com o do mesmo pote do primeiro
///    outro grupo da metade sem a nação dele.
/// 5) Cada grupo: os 4 clubes ganham 1 ponto do coeficiente da nação (0x9058b0 em [0xde1f58]: push 1; push nação ->
///    thiscall, ret 8), o prêmio de 1.800.000 (0x5a0590 em [0xae2a38] push clube -> thiscall, ret 4; 0x5999a0 push
///    valor; mov ecx, finanças -> thiscall, ret 4) e a notícia do prêmio (0x48e3e0: push −2; push 0; push 0x449; push
///    −1; push 0; push valor; push clube; mov ecx, this -> thiscall, ret 0x1c); a tabela das rodadas do método +0x3c
///    (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) e o
///    construtor League_Stage 0x68b100: this, 4, o grupo, 2, +4, a tabela, as rodadas (word), 3, 1, 8, &{4,1,2,0},
///    &{2,0,0,0}, o ano (word), g, o id (word), 0xf, 2, 0, 8, −1, 0, 2; a original empurra o ano com lixo nos bytes
///    altos do edx, que o construtor não lê: a cópia passa o ano limpo) em [+0xc][g] (sem fase, a caixa da linha 0x5c9
///    e volta, sem liberar a tabela nem os grupos); a tabela das rodadas liberada, +0x30 = g e a notícia do grupo
///    (0x684230).
/// 6) Os grupos liberados; os times de +0xa7 com +5 = 2 (os perdedores da terceira preliminar) vão para a Copa da UEFA
///    ([[0xadadfc] + 4 × [0x9cf6f0]]; sem ela, a caixa da linha 0x5d8): nas últimas 24 vagas da tabela +0xa7 dela (a
///    partir de +0x3e − 24), +4 e +5 zerados, na tabela +0x14 dela com a situação 5 (+0x36 sobe) e com a competição
///    europeia do clube (+0x1db) = [0x9cf6f0]; depois as posições previstas da UEFA (método +0x5c dela).
pub(crate) unsafe extern "thiscall" fn draw_phase1_groups(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rules: [u8; 4] = [4, 1, 2, 0];
    let rules_b: [u8; 4] = [2, 0, 0, 0];
    let list = exe::crt_calloc(0x20, 9);
    if list == 0 {
        show_error_at(0x9c4900, 0x4cc);
        return;
    }
    let ent = |i: i32| list.wrapping_add((i * 9) as usize);
    let mut n = 0i16;
    for k in 0..16usize {
        let e = ent(n as i32);
        write_usize(e, read_usize(read_usize(this + cup::ENTRANT_BOOK) + k * 6));
        write_usize(e + 4, 0);
        n += 1;
    }
    let mut k = 0i32;
    while k < read_i16(this + cup::TEAM_COUNT) as i32 {
        let t = read_usize(this + cup::TEAM_TABLE) + k as usize * 6;
        if read_u8(t + 5) == 1 {
            let e = ent(n as i32);
            write_usize(e, read_usize(t));
            write_usize(e + 4, 0);
            n += 1;
        }
        k += 1;
    }
    if n < 0x20 {
        show_error_at(0x9c4900, 0x4ee);
        write_usize(this + cup::CURRENT_STAGE, usize::MAX);
        exe::crt_free(list);
        write_u8(this + cup::RUN_BY_STAGES, 0);
        return;
    }
    exe::crt_qsort(list, 0x20, 9, 0x586f60);
    for j in 0..0x20i32 {
        let e = ent(j);
        if read_usize(e + 4) != 0 || j + 1 >= 0x20 {
            continue;
        }
        for i in (j + 1)..0x20 {
            let o = ent(i);
            if read_usize(o + 4) == 0 && club_nation(Club(read_usize(e))) == club_nation(Club(read_usize(o))) {
                crate::comparison::branch("cqg_4 partner from same nation");
                write_usize(e + 4, read_usize(o)); // cqg4: the partner from the same nation
                write_usize(o + 4, read_usize(e));
                break;
            }
        }
    }
    let groups = exe::crt_calloc(0x20, 4);
    if groups == 0 {
        fatal_error_at(0x9c4900, 0x515);
    }
    let slot = |g: i32, pot: i32| groups + ((pot + g * 4) * 4) as usize;
    let mut pot = 0i32;
    let mut base = 0i32;
    while base < 0x20 {
        let (mut livres_b, mut livres_a) = (4i8, 4i8); // [esp+0x12] half 2, [esp+0x13] half 1
        for i in 0..8 {
            match read_u8(ent(base + i) + 8) {
                2 => livres_b -= 1,
                1 => livres_a -= 1,
                _ => {}
            }
        }
        for _ in 0..8 {
            let mut s = exe::utils_random(8).wrapping_add(base); // cqg4: the club drawn from the pot
            while read_usize(ent(s)) == 0 {
                s = exe::utils_random(8).wrapping_add(base);
            }
            crate::comparison::branch("cqg_4 club drawn");
            let e = ent(s);
            if read_u8(e + 8) == 0 {
                crate::comparison::branch("cqg_4 half drawn");
                let b = livres_b as i32;
                if exe::utils_random(livres_a as i32 + b) < b { // cqg4: a metade sorteada
                    write_u8(e + 8, 2);
                    livres_b -= 1;
                } else {
                    write_u8(e + 8, 1);
                    livres_a -= 1;
                }
                let par = read_usize(e + 4);
                if par != 0 {
                    if let Some(i) = (0..0x20).find(|&i| read_usize(ent(i)) == par) {
                        let same_pot = i / 8 == s / 8;
                        let mark = if same_pot {
                            let m = read_u8(e + 8);
                            if (m == 2 && livres_a == 0) || (m == 1 && livres_b == 0) {
                                false
                            } else {
                                if m == 2 {
                                    livres_a -= 1;
                                } else {
                                    livres_b -= 1;
                                }
                                true
                            }
                        } else {
                            true
                        };
                        if mark {
                            write_u8(ent(i) + 8, if read_u8(e + 8) != 2 { 2 } else { 1 });
                        }
                    }
                }
            }
            let start: i32 = if read_u8(e + 8) == 1 { 0 } else { 4 };
            for g in start..start + 4 {
                if read_usize(slot(g, pot)) == 0 {
                    write_usize(slot(g, pot), read_usize(e));
                    write_usize(e, 0);
                    break;
                }
            }
        }
        base += 8;
        pot += 1;
    }
    exe::crt_free(list);
    for metade in 0..2i32 {
        let (start, fim) = (metade * 4, metade * 4 + 4);
        for p in 0..4i32 {
            let mut g = start;
            while g < fim {
                let nation = club_nation(Club(read_usize(slot(g, p))));
                let repetida = (0..4).any(|q| q != p && nation == club_nation(Club(read_usize(slot(g, q)))));
                if repetida {
                    for g2 in start..fim {
                        if g2 == g {
                            continue;
                        }
                        if (0..4).all(|q| club_nation(Club(read_usize(slot(g, p)))) != club_nation(Club(read_usize(slot(g2, q))))) {
                            crate::comparison::branch("cqg_4 swap by nation");
                            let x = read_usize(slot(g, p));
                            write_usize(slot(g, p), read_usize(slot(g2, p)));
                            write_usize(slot(g2, p), x);
                            break;
                        }
                    }
                }
                g += 1;
            }
        }
    }
    for g in 0..8i32 {
        let row = slot(g, 0);
        for q in 0..4usize {
            let c = read_usize(row + q * 4);
            exe::uefaseeding_add_nation_points(read_usize(G_UEFA_SEEDING), club_nation(Club(c)), 1);
            let value = 1_800_000i32; // cqg4: the group stage prize
            crate::comparison::branch("cqg_4 group stage prize");
            exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), value);
            exe::comp_send_prize_money_news(this, c, value, 0, -1, 0x449, 0, -2);
        }
        let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let (mut rodadas, mut id) = (0i16, 0i16);
        let table = rounds_of_cup(this, g as u32, &mut rodadas, &mut id, 0);
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(
                p,
                this,
                4,
                row,
                2,
                read_usize(this + 4),
                table,
                rodadas as u16 as u32,
                3,
                1,
                8,
                rules.as_ptr(),
                rules_b.as_ptr(),
                read_i16(this + cup::YEAR) as u16 as u32,
                g as u32,
                id as u16 as u32,
                0xf,
                2,
                0,
                8,
                -1,
                0,
                2,
            )
        } else {
            0
        };
        write_usize(read_usize(this + cup::STAGES) + g as usize * 4, stage);
        if read_usize(read_usize(this + cup::STAGES) + g as usize * 4) == 0 {
            show_error_at(0x9c4900, 0x5c9);
            return;
        }
        exe::crt_free(table);
        write_usize(this + cup::CURRENT_STAGE, g as usize);
        exe::league_send_group_draw_news(read_usize(read_usize(this + cup::STAGES) + g as usize * 4));
    }
    exe::crt_free(groups);
    let uefa = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_UEFA_CUP) * 4);
    if uefa == 0 {
        show_error_at(0x9c4900, 0x5d8);
        return;
    }
    let mut target = ((read_i16(uefa + cup::TEAM_COUNT) as i32 - 0x18) * 6) as usize;
    let mut k = 0i32;
    while k < read_i16(this + cup::TEAM_COUNT) as i32 {
        let t = read_usize(this + cup::TEAM_TABLE) + k as usize * 6;
        if read_u8(t + 5) == 2 {
            let e = read_usize(uefa + cup::TEAM_TABLE).wrapping_add(target);
            target = target.wrapping_add(6);
            write_usize(e, read_usize(t));
            write_u8(e + 4, 0);
            write_u8(e + 5, 0);
            let n = read_i16(uefa + cup::ENTRANT_COUNT) as i32;
            write_usize(read_usize(uefa + cup::ENTRANT_BOOK) + (n * 6) as usize, read_usize(e));
            let n = read_i16(uefa + cup::ENTRANT_COUNT) as i32;
            write_u8(read_usize(uefa + cup::ENTRANT_BOOK) + (n * 6) as usize + 4, 5);
            write_u16(uefa + cup::ENTRANT_COUNT, (read_i16(uefa + cup::ENTRANT_COUNT) as u16).wrapping_add(1));
            write_usize(read_usize(e) + 0x1db, read_usize(KEY_CLUB_COMP_UEFA_CUP));
        }
        k += 1;
    }
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa) + 0x5c));
    predicted(uefa);
}

/// The 32 clubs the draw puts in groups: the first 16 entrants of [+0x14] plus table entries (+0xa7) with +5 = 1,
/// sorted.
///
/// [pt] Os 32 clubes que o sorteio põe nos grupos: os 16 primeiros de [+0x14] e os de +0xa7 com +5 = 1.
pub(crate) unsafe fn group_draw_clubs(this: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..16usize).map(|k| read_usize(read_usize(this + cup::ENTRANT_BOOK) + k * 6)).collect();
    for k in 0..read_i16(this + cup::TEAM_COUNT).max(0) as usize {
        let t = read_usize(this + cup::TEAM_TABLE) + k * 6;
        if read_u8(t + 5) == 1 {
            v.push(read_usize(t));
        }
    }
    v.sort_unstable();
    v
}

/// the pot comparison: the club's European seed (+0x1df), and on a tie the higher reputation first
/// Original: `european_cup_compare_clubs_by_seed` (0x586f60, european_cup.cpp, cdecl).
///
/// [pt] { 0x586f60 } (cdecl, european_cup.cpp): `european_cup_compare_clubs_by_seed(a, b)`, a comparação da qsort dos potes
/// do sorteio (0x585290): o byte +0x1df (a semente europeia) de a menos o de b; empate: a reputação (+0x80, word) de b
/// menos a de a.
pub(crate) unsafe extern "C" fn european_cup_compare_clubs_by_seed(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    let r = read_u8(x + 0x1df) as i8 as i32 - read_u8(y + 0x1df) as i8 as i32;
    if r != 0 {
        return r;
    }
    read_i16(y + 0x80) as i32 - read_i16(x + 0x80) as i32
}

/// The positions that `european_cup_rank_teams` records, in the order of its calls to `set_club_final_position`: (index of the club in the sorted vector,
/// position).
///
/// `mata` is the knockout round table (0x68 bytes per round, +0x1a the tie count) with `nm` rounds; the preliminaries
/// come from [+0xa3] with +0x3c rounds.
///
/// 1) The knockout, from the final backwards: in the final, those of the first pass get position 1 (the counter goes up
/// with each one); in every round, each of the following ones gets the counter value from the start of the round (which
/// goes up with each one).
///
/// 2) The second group stage: 2 × (4 clubs with the counter; the counter goes up by 4). 3) The first group stage: 2 ×
/// (8 clubs; it goes up by 8).
///
/// 4) The preliminaries, from the last to the first: each club of the round gets the counter value from the start of
/// that round (it goes up with each one).
///
/// [pt] As posições que a 0x586900 grava, na ordem das chamadas à 0x4a2540: (índice do clube no vetor ordenado, posição).
/// `mata` é a tabela das rodadas do mata-mata (0x68 bytes por rodada, +0x1a a conta de confrontos) com `nm` rodadas;
/// as preliminares vêm de [+0xa3] com +0x3c rodadas.
/// 1) O mata-mata, da final para trás: na final, os da primeira volta ganham a posição 1 (o contador sobe a cada um);
///    em toda rodada, cada um dos seguintes ganha o contador do começo da rodada (que sobe a cada um).
/// 2) A segunda fase: 2 × (4 clubes com o contador; o contador sobe 4). 3) A primeira fase: 2 × (8 clubes; sobe 8).
/// 4) As preliminares, da última para a primeira: cada clube da rodada com o contador do começo dela (sobe a cada um).
pub(crate) unsafe fn planned_final_positions(this: usize, knockout: usize, nm: i16) -> (Vec<(i16, u8)>, usize) {
    let mut v = Vec::new();
    let mut i = 0i16;
    let mut count = 1u8;
    let mut r = nm.wrapping_sub(1);
    while r >= 0 {
        let n = read_i16(knockout + r as usize * 0x68 + 0x1a);
        if r == nm.wrapping_sub(1) {
            let mut k = 0i16;
            while k < n {
                crate::comparison::branch("cqg_4 champion");
                v.push((i, 1u8)); // cqg4: the champion
                i += 1;
                count = count.wrapping_add(1);
                k += 1;
            }
        }
        let p = count;
        let mut k = 0i16;
        while k < read_i16(knockout + r as usize * 0x68 + 0x1a) {
            v.push((i, p));
            i += 1;
            count = count.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    let n_knockout = v.len();
    for &(vezes, quantos) in &[(2, 4u8), (2, 8u8)] {
        for _ in 0..vezes {
            let p = count;
            count = count.wrapping_add(quantos);
            for _ in 0..quantos {
                v.push((i, p));
                i += 1;
            }
        }
    }
    let mut r = read_i16(this + cup::ROUND_COUNT).wrapping_sub(1);
    while r >= 0 {
        let p = count;
        let mut k = 0i16;
        while k < read_i16(read_usize(this + cup::ROUND_TABLE) + r as usize * 0x68 + 0x1a) {
            crate::comparison::branch("cqg_4 position in the preliminary");
            v.push((i, p)); // cqg4: the position in the preliminary
            i += 1;
            count = count.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    (v, n_knockout)
}

/// Sets the Champions League predicted positions: the entrants sorted by strength, each one's predicted position taken
/// from the knockout from the final back, the second and first group phases and the qualifying rounds.
///
/// Method +0x5c of the Champions League: `EuropeanCup::rank_teams()`. Without statistics (+8): the error box at line
/// 0x81b. The entrants of table `[+0x14]` (6 bytes, +0x36 of them) go into an array (`exe::crt_malloc`; out of memory, the error
/// box at line 0x823), sorted by strength (`comp_util_sort_clubs_by_strength`: push n; push array; add esp, 8 -> cdecl); the knockout table from
/// method +0x3c (push 0; push &id; push &rounds; push 0xc; mov ecx -> thiscall, ret 0x10), freed afterwards; each
/// entrant's position, in array order, through `set_club_final_position` (push position; push club; mov ecx, `[+8]` -> thiscall, ret 8;
/// the original pushes the stack dword with the position in the low byte and garbage in the others, and `set_club_final_position` writes
/// only the byte: the copy passes the clean byte) following the plan in `planned_final_positions`; the array freed.
/// Original: `EuropeanCup::rank_teams` (0x586900, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x586900 } (thiscall, ret 0, european_cup.cpp, método +0x5c da Liga dos Campeões): `EuropeanCup::rank_teams()`, as
/// posições previstas. Sem estatística (+8): a caixa da linha 0x81b. Os inscritos da tabela [+0x14] (6 bytes, +0x36
/// deles) num vetor (0x944e46; sem memória, a caixa da linha 0x823), ordenado pela força (0x4c08f0: push n; push vetor;
/// add esp, 8 -> cdecl); a tabela do mata-mata pelo método +0x3c (push 0; push &id; push &rodadas; push 0xc; mov ecx ->
/// thiscall, ret 0x10), liberada depois; a posição de cada um, na ordem do vetor, pela 0x4a2540 (push posição; push
/// clube; mov ecx, [+8] -> thiscall, ret 8; a original empurra o dword da pilha com a posição no byte de baixo e lixo
/// nos outros, e a 0x4a2540 só grava o byte: a cópia passa o byte limpo) no plano de `planned_final_positions`; o vetor liberado.
pub(crate) unsafe extern "thiscall" fn european_cup_rank_teams(this: usize) {
    if read_usize(this + 8) == 0 {
        show_error_at(0x9c4900, 0x81b);
        return;
    }
    let v = exe::crt_malloc(((read_i16(this + cup::ENTRANT_COUNT) as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0x9c4900, 0x823);
        return;
    }
    let mut i = 0i16;
    while i < read_i16(this + cup::ENTRANT_COUNT) {
        core::ptr::write_unaligned((v + i as usize * 4) as *mut u32, read_usize(read_usize(this + cup::ENTRANT_BOOK) + i as usize * 6) as u32);
        i += 1;
    }
    exe::comp_util_sort_clubs_by_strength(v, read_i16(this + cup::ENTRANT_COUNT) as i32);
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let knockout = rounds_of_cup(this, 0xc, &mut nm, &mut id, 0);
    // the knockout positions before freeing its table, the others after (like the original)
    let (plano, n_mata) = planned_final_positions(this, knockout, nm);
    for (j, &(k, p)) in plano.iter().enumerate() {
        if j == n_mata {
            exe::crt_free(knockout);
        }
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(v + k as usize * 4), p as u32);
    }
    if n_mata == plano.len() {
        exe::crt_free(knockout);
    }
    exe::crt_free(v);
}

/// Copies a 7-byte group entry (club, same-nation count, first-stage group, winner/runner-up) as the original does.
///
/// [pt] Copia a entrada de 7 bytes `de` (+0 o clube, +4 os da mesma nação, +5 o grupo da primeira fase, +6 0 vencedor / 1
/// segundo) para `para`, como a original: o dword, o word +4 e o byte +6.
pub(crate) unsafe fn copy_group_entry(para: usize, de: usize) {
    write_usize(para, read_usize(de));
    write_u16(para + 4, read_i16(de + 4) as u16);
    write_u8(para + 6, read_u8(de + 6));
}

/// Draws the Champions League second group phase: the 16 qualifiers sorted, the same-group pairs, each one's group
/// choice (shuffled order, fewest teams, no same first-phase group nor nation), the redraw up to 20 times, the
/// coefficient, the 1,200,000 prize and its news, stages 8 to 11 and the third places into the UEFA Cup.
///
/// SEH frame only for the stage's `new`: no destructor on the normal path. `EuropeanCup::draw_phase2_groups(attempt)`,
/// called from the Champions League's next-stage routine (`european_cup_advance_stage`, stage 8, attempt 0).
///
/// 1) The 16 qualifiers (`exe::crt_calloc`(0x10, 7); out of memory, the error box at line 0x61c): from each first-phase group b
/// (`[+0xc][b]`) the first and second of the table (`league_record` push 0 / push 1; mov ecx, stage -> thiscall, ret 4), with
/// group b and 0/1; +4 counts the clubs in the list from the same nation (itself included); the list is sorted (qsort
/// `exe::crt_qsort` with `european_cup_compare_phase2_draw_entries`) and each club's bookings are cleared in the competition's governing body (`exe::rulingbody_reset_bookings_for_club`, as in
/// the World Cup draw).
///
/// 2) The pairs from the same first-phase group (two cleared lists of 16; out of memory, the error boxes 0x649 and
/// 0x651): each club still in the list goes to slot k (winner) or k + 1 (runner-up) and the other club of its group to
/// the other one; k goes up by 2.
///
/// 3) Each of the 16, in pair order: the order of the 4 groups is shuffled (5 swaps of `exe::utils_random`(4) and `exe::utils_random`(4));
/// the group with fewest teams among those that have a free slot in its half (winners in slots 0/1, runners-up in 2/3)
/// and have no team from the same first-phase group nor from the same nation receives the club.
///
/// 4) If a club is left without a group: with attempt < 20, the lists are freed and the draw is run again with attempt
/// + 1 (the call nests inside this one); on the 20th, each leftover takes the first free slot (the club stays in the
/// pairs list).
///
/// 5) Each group h (0..3): the 4 clubs earn 1 coefficient point, the 1,200,000 prize and the news (`send_prize_money_news` with round
/// 0x44a); stage h + 8 (method +0x3c, `exe::crt_operator_new`(0xee) and the constructor 0x68b100 with the same arguments as the
/// first-phase draw, the list of the 4 clubs copied into a 0x40-byte array (`exe::crt_malloc`); without a stage, the error box
/// at line 0x715), +0x30 = h + 8 and the group news. The four lists are freed.
///
/// 6) The first-phase third places (`league_record` push 2) go to the UEFA Cup (`[[0xadadfc] + 4 × [0x9cf6f0]]`; without it,
/// the error box at line 0x729): into the last 8 slots of its +0xa7 table, from back to front, into the +0x14 table
/// with status 5 and with the club's European competition = the UEFA Cup; the fourth places (push 3) are left without a
/// European competition (−1); then the UEFA Cup's predicted positions (method +0x5c).
/// Original: `EuropeanCup::draw_phase2_groups` (0x585ab0, european_cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x585ab0 } (thiscall, ret 4, european_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `EuropeanCup::draw_phase2_groups(tentativa)`, da fase seguinte da Liga dos Campeões (0x5851d0, fase 8,
/// tentativa 0).
/// 1) Os 16 classificados (0x945501(0x10, 7); sem memória, a caixa da linha 0x61c): de cada grupo b da primeira fase
///    ([+0xc][b]) o primeiro e o segundo da tabela (0x6826d0 push 0 / push 1; mov ecx, fase -> thiscall, ret 4), com o
///    grupo b e 0/1; +4 conta os da lista com a mesma nação (o próprio incluído); a lista ordenada (qsort 0x945333 com
///    0x5871e0) e os cartões de cada um zerados no órgão da competição (0x7e9180, como no sorteio da Copa do Mundo).
/// 2) Os pares do mesmo grupo da primeira fase (duas listas zeradas de 16, sem memória as caixas 0x649 e 0x651): cada
///    clube ainda na lista vai para a vaga k (vencedor) ou k + 1 (segundo) e o outro do grupo dele para a outra; k sobe 2.
/// 3) Cada um dos 16, na ordem dos pares: a ordem dos 4 grupos embaralhada (5 trocas de 0x90cfc0(4) e 0x90cfc0(4));
///    o grupo com menos times entre os que têm vaga na metade dele (vencedores nas vagas 0/1, segundos nas 2/3) e não
///    têm time do mesmo grupo da primeira fase nem da mesma nação recebe o clube.
/// 4) Sobrou clube sem grupo: com a tentativa < 20, as listas liberadas e o sorteio de novo com a tentativa + 1 (a
///    chamada fica dentro desta); na 20ª, cada sobra ocupa a primeira vaga livre (o clube continua na lista dos pares).
/// 5) Cada grupo h (0..3): os 4 clubes ganham 1 ponto do coeficiente, o prêmio de 1.200.000 e a notícia (0x48e3e0 com a
///    rodada 0x44a); a fase h + 8 (o método +0x3c, 0x944cf1(0xee) e o construtor 0x68b100 com os mesmos argumentos do
///    sorteio da primeira fase, a lista dos 4 clubes copiada num vetor de 0x40 bytes (0x944e46); sem fase, a caixa da
///    linha 0x715), +0x30 = h + 8 e a notícia do grupo. As quatro listas liberadas.
/// 6) Os terceiros da primeira fase (0x6826d0 push 2) vão para a Copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]; sem ela, a
///    caixa da linha 0x729): nas últimas 8 vagas da tabela +0xa7 dela, de trás para a frente, na tabela +0x14 com a
///    situação 5 e com a competição europeia do clube = a UEFA; os quartos (push 3) ficam sem competição europeia (−1);
///    depois as posições previstas da UEFA (método +0x5c).
pub(crate) unsafe extern "thiscall" fn draw_phase2_groups(this: usize, attempt: u32) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rules: [u8; 4] = [4, 1, 2, 0];
    let rules_b: [u8; 4] = [2, 0, 0, 0];
    let w = exe::crt_calloc(0x10, 7);
    if w == 0 {
        show_error_at(0x9c4900, 0x61c);
        return;
    }
    let e7 = |l: usize, i: i32| l.wrapping_add((i * 7) as usize);
    for b in 0..8i32 {
        let stage = || read_usize(read_usize(this + cup::STAGES) + b as usize * 4);
        let e = e7(w, 2 * b);
        write_usize(e, read_usize(exe::league_get_table_row(stage(), (0) as i32)));
        write_u8(e + 6, 0);
        write_u8(e + 5, b as u8);
        write_u8(e + 4, 0);
        let e = e7(w, 2 * b + 1);
        write_usize(e, read_usize(exe::league_get_table_row(stage(), (1) as i32)));
        write_u8(e + 6, 1);
        write_u8(e + 5, b as u8);
        write_u8(e + 4, 0);
    }
    for i in 0..16 {
        for j in 0..16 {
            if club_nation(Club(read_usize(e7(w, i)))) == club_nation(Club(read_usize(e7(w, j)))) {
                write_u8(e7(w, i) + 4, read_u8(e7(w, i) + 4).wrapping_add(1));
            }
        }
    }
    exe::crt_qsort(w, 0x10, 7, 0x5871e0);
    for i in 0..16 {
        let body = read_usize(read_usize(read_usize(G_DISCIPLINE)).wrapping_add((read_u8(this + cup::DISCIPLINE_INDEX) as i8 as i32 * 4) as usize));
        exe::rulingbody_reset_bookings_for_club(body, read_usize(e7(w, i)));
    }
    let r = exe::crt_calloc(0x10, 7);
    if r == 0 {
        show_error_at(0x9c4900, 0x649);
        return;
    }
    let g = exe::crt_calloc(0x10, 7);
    if g == 0 {
        show_error_at(0x9c4900, 0x651);
        return;
    }
    let mut k = 0i8;
    for i in 0..16i32 {
        let e = e7(w, i);
        if read_usize(e) == 0 {
            continue;
        }
        let c = k as i32;
        copy_group_entry(e7(r, if read_u8(e + 6) != 0 { c + 1 } else { c }), e);
        write_usize(e, 0);
        for a in 0..16i32 {
            if i != a && read_u8(e + 5) == read_u8(e7(w, a) + 5) {
                let o = e7(w, a);
                let target = if read_usize(e7(r, c)) != 0 { e7(r, c + 1) } else { e7(r, c) };
                crate::comparison::branch("cqg_6 pair from same group");
                copy_group_entry(target, o); // cqg6: the other one in the group
                k = k.wrapping_add(2);
                write_usize(o, 0);
                break;
            }
        }
    }
    let mut order = [0u8, 1, 2, 3];
    for i in 0..16i32 {
        let p = e7(r, i);
        let mut minus = 4i8;
        let mut best = 0xffu8;
        let s: i32 = if read_u8(p + 6) != 0 { 2 } else { 0 };
        for _ in 0..5 {
            let a = exe::utils_random(4) as i8 as usize;
            let b = exe::utils_random(4) as i8 as usize; // cqg6: the swap of the group order
            crate::comparison::branch("cqg_6 order swap");
            order.swap(a, b);
        }
        for q in 0..4usize {
            let gr = order[q] as i8 as i32;
            let mut n = 0i8;
            let mut conflito = false;
            for j in 0..4i32 {
                let v = e7(g, j + gr * 4);
                let club = read_usize(v);
                if club != 0 {
                    n += 1;
                    if read_u8(v + 5) == read_u8(p + 5) || club_nation(Club(club)) == club_nation(Club(read_usize(p))) { // cqg6: same group or nation
                        conflito = true;
                        break;
                    }
                }
            }
            let vaga = read_usize(e7(g, s + gr * 4)) == 0 || read_usize(e7(g, s + 1 + gr * 4)) == 0;
            if vaga && !conflito && n < minus {
                minus = n;
                best = order[q];
            }
        }
        if best != 0xff {
            crate::comparison::branch("cqg_6 group chosen");
            let base = best as i8 as i32 * 4;
            let v = if read_usize(e7(g, s + base)) == 0 { e7(g, s + base) } else { e7(g, s + 1 + base) };
            copy_group_entry(v, p);
            write_usize(p, 0);
        }
    }
    for i in 0..16i32 {
        let p = e7(r, i);
        if read_usize(p) == 0 {
            continue;
        }
        if (attempt as u8 as i8) < 0x14 {
            crate::comparison::branch("cqg_6 redraw");
            exe::crt_free(g);
            exe::crt_free(r);
            exe::crt_free(w);
            exe::europeancup_draw_phase2_groups(this, (attempt as u8).wrapping_add(1) as u32);
            return;
        }
        if let Some(v) = (0..16).map(|j| e7(g, j)).find(|&v| read_usize(v) == 0) {
            copy_group_entry(v, p);
        }
    }
    let t = exe::crt_malloc(0x40);
    for i in 0..16usize {
        write_usize(t + i * 4, read_usize(e7(g, i as i32)));
    }
    for h in 0..4i32 {
        for q in 0..4i32 {
            let c = read_usize(e7(g, h * 4 + q));
            exe::uefaseeding_add_nation_points(read_usize(G_UEFA_SEEDING), club_nation(Club(c)), 1);
            let value = 1_200_000i32; // cqg6: the second stage prize
            crate::comparison::branch("cqg_6 second stage prize");
            exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), value);
            exe::comp_send_prize_money_news(this, c, value, 0, -1, 0x44a, 0, -2);
        }
        let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let (mut rodadas, mut id) = (0i16, 0i16);
        let table = rounds_of_cup(this, (h + 8) as u32, &mut rodadas, &mut id, 0);
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(
                p,
                this,
                4,
                t + h as usize * 16,
                2,
                read_usize(this + 4),
                table,
                rodadas as u16 as u32,
                3,
                1,
                8,
                rules.as_ptr(),
                rules_b.as_ptr(),
                read_i16(this + cup::YEAR) as u16 as u32,
                (h + 8) as u32,
                id as u16 as u32,
                0xf,
                2,
                0,
                8,
                -1,
                0,
                2,
            )
        } else {
            0
        };
        let slot = read_usize(this + cup::STAGES) + (h + 8) as usize * 4;
        write_usize(slot, stage);
        if read_usize(read_usize(this + cup::STAGES) + (h + 8) as usize * 4) == 0 {
            show_error_at(0x9c4900, 0x715);
            return;
        }
        exe::crt_free(table);
        write_usize(this + cup::CURRENT_STAGE, (h + 8) as usize);
        exe::league_send_group_draw_news(read_usize(read_usize(this + cup::STAGES) + (h + 8) as usize * 4));
    }
    exe::crt_free(w);
    exe::crt_free(g);
    exe::crt_free(r);
    exe::crt_free(t);
    let uefa = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_UEFA_CUP) * 4);
    if uefa == 0 {
        show_error_at(0x9c4900, 0x729);
        return;
    }
    let mut target = ((read_i16(uefa + cup::TEAM_COUNT) as i32 - 1) * 6) as usize;
    for b in 0..8usize {
        let e = read_usize(uefa + cup::TEAM_TABLE).wrapping_add(target);
        target = target.wrapping_sub(6);
        let third = read_usize(exe::league_get_table_row(read_usize(read_usize(this + cup::STAGES) + b * 4), (2) as i32));
        write_u8(e + 4, 0);
        write_usize(e, third);
        write_u8(e + 5, 0);
        let n = read_i16(uefa + cup::ENTRANT_COUNT) as i32;
        write_usize(read_usize(uefa + cup::ENTRANT_BOOK) + (n * 6) as usize, read_usize(e));
        let n = read_i16(uefa + cup::ENTRANT_COUNT) as i32;
        write_u8(read_usize(uefa + cup::ENTRANT_BOOK) + (n * 6) as usize + 4, 5);
        write_u16(uefa + cup::ENTRANT_COUNT, (read_i16(uefa + cup::ENTRANT_COUNT) as u16).wrapping_add(1));
        write_usize(read_usize(e) + 0x1db, read_usize(KEY_CLUB_COMP_UEFA_CUP));
        let quarto = read_usize(exe::league_get_table_row(read_usize(read_usize(this + cup::STAGES) + b * 4), (3) as i32));
        write_usize(quarto + 0x1db, usize::MAX);
    }
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa) + 0x5c));
    predicted(uefa);
}

/// The 16 clubs that go to the second stage (top two of each first-stage group), sorted.
///
/// [pt] Os 16 clubes que vão para a segunda fase (o primeiro e o segundo de cada grupo da primeira).
pub(crate) unsafe fn second_stage_clubs(this: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(16);
    for b in 0..8usize {
        let f = read_usize(read_usize(this + cup::STAGES) + b * 4);
        v.push(read_usize(exe::league_get_table_row(f, (0) as i32)));
        v.push(read_usize(exe::league_get_table_row(f, (1) as i32)));
    }
    v.sort_unstable();
    v
}

/// The comparison for the second-phase qualifiers: more same-nation clubs first, then the higher nation word +0x8e,
/// then the group winner before the runner-up.
///
/// `european_cup_compare_phase2_draw_entries(a, b)`, the qsort comparison of the qualifiers (`draw_phase2_groups`): b's same-nation
/// count (+4) minus a's; on a tie: the nation's word +0x8e (the club's +0x53) of b minus a's; on a tie: a's +6 (the
/// runner-up flag) minus b's.
/// Original: `european_cup_compare_phase2_draw_entries` (0x5871e0, european_cup.cpp, cdecl).
///
/// [pt] { 0x5871e0 } (cdecl, european_cup.cpp): `european_cup_compare_phase2_draw_entries(a, b)`, a comparação da qsort dos
/// classificados (0x585ab0): os da mesma nação (+4) de b menos os de a; empate: o word +0x8e da nação (+0x53 do clube) de
/// b menos o de a; empate: +6 (o segundo) de a menos o de b.
pub(crate) unsafe extern "C" fn european_cup_compare_phase2_draw_entries(a: usize, b: usize) -> i32 {
    let r = read_u8(b + 4) as i8 as i32 - read_u8(a + 4) as i8 as i32;
    if r != 0 {
        return r;
    }
    let r = read_i16(club_nation(Club(read_usize(b))) + 0x8e) as i32 - read_i16(club_nation(Club(read_usize(a))) + 0x8e) as i32;
    if r != 0 {
        return r;
    }
    read_u8(a + 6) as i8 as i32 - read_u8(b + 6) as i8 as i32
}

/// comp_util_scale_by_round (`comp_util_scale_by_round`): a value between low and high by round, 0..100.
///
/// [pt] comp_util_scale_by_round (0x4bf850, cdecl(baixo, alto, rodada, rodadas)): a faixa pela rodada, de 0 a 100.
pub(crate) unsafe fn scale_by_round(low: u32, high: u32, round: i16, rounds: u32) -> u32 {
    // simplification: the original pushes the round in a register with garbage in the high 16 bits; 0x4bf850 only reads the word.
    (exe::comp_util_scale_by_round(low, high, round as u16 as u32, rounds) as u32)
}

/// Staff_Records add_qualified_for_round_record for the club.
///
/// [pt] Staff_Records::add_qualified_for_round_record (0x86bdd0: thiscall(registros, clube, comp, fase, rodada, peso),
/// ret 0x14).
pub(crate) unsafe fn european_cup_add_qualified_record(club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    exe::staff_records_add_qualified_for_round_record(eng_second_staff_records(), club, comp, stage, round, weight);
}

/// Staff_Records add_failed_to_qualify_record for the club.
///
/// [pt] Staff_Records::add_failed_to_qualify_record (0x86c1d0: thiscall(registros, clube, comp, fase, _), ret 0x10).
pub(crate) unsafe fn european_cup_add_failed_to_qualify_record(club: usize, comp: usize, stage: u32, x: u32) {
    exe::staff_records_add_failed_to_qualify_record(eng_second_staff_records(), club, comp, stage, x);
}

/// Staff_Records add_knocked_out_record for the club.
///
/// [pt] Staff_Records::add_knocked_out_record (0x86c000: thiscall(registros, clube, comp, fase, rodada, _), ret 0x14).
pub(crate) unsafe fn european_cup_add_knocked_out_record(club: usize, comp: usize, stage: u32, round: u32, x: u32) {
    exe::staff_records_add_knocked_out_record(eng_second_staff_records(), club, comp, stage, round, x);
}

/// Removes the club from its European competition this season (EURO_FLAG +0x1db = -1).
///
/// [pt] O clube sai da competição europeia da temporada (+0x1db = −1).
pub(crate) unsafe fn leave_euro_competition(club: Club) {
    core::ptr::write_unaligned((club.addr() + club::CONTINENTAL_COMP_ID) as *mut i32, -1);
}

/// Updates the staff records by outcome and stage: in qualifying, the qualified (`add_qualified_for_round_record`) or knocked-out (`add_knocked_out_record`)
/// record weighted by round (`comp_util_scale_by_round`); in the group stages, qualified (6) or failed to qualify (0xfe, `add_failed_to_qualify_record`); in
/// the knockout, winners (`add_cup_winners_record`), runners-up (`add_cup_runners_up_record`), qualified or knocked out; whoever leaves gets +0x1db = -1;
/// returns 0.
///
/// `EuropeanCup::update_staff_records_for_status(club, outcome, stage, _, fixture, final)`, the Champions League's
/// virtual +0x44. Returns 0. Comp = +4.
///
/// - In qualifying (stage 0xff): outcome 0 or 1, the qualification by the fixture's round (+0x34): in round 0, with
/// stage +0x3a and the +0x6f of the first round (+0xa3) and the weight `comp_util_scale_by_round`(1, 0x14, 0, 2); in any other, 0x41c, 0
/// and `comp_util_scale_by_round`(1, 0x14, round, 2). Outcome 0xff or 2: the elimination (`add_knocked_out_record`) with the fixture's +0x32 and +0x30
/// and the weight `comp_util_scale_by_round`(1, 0x14, round, 2), and the club leaves (+0x1db = −1).
///
/// - In phase 1 (stage < 8): 6, the qualification (0x44a, −1, 0x32); 0xfe, the non-qualification (0x449, 0x32) and the
/// club leaves.
///
/// - In phase 2 (8 to 11): 6, the qualification (0, 0x78, 0x32); 0xfe, the non-qualification (0x44a, 0x32) and the club
/// leaves.
///
/// - In the knockout (12): 1, the title (Staff_Records::add_cup_winners_record `add_cup_winners_record`, thiscall(records, club,
/// fixture, final), ret 0xc) and the club leaves; 2, runner-up (`add_cup_runners_up_record`, likewise) and the club leaves; 0, the
/// qualification (0, the +0x6f of the fixture's round in the knockout stage's round table (`[+0xc]` + 0x30, +0xa3, 0x68
/// bytes), `comp_util_scale_by_round`(0x4b, 0x64, round, 2)); 0xff, the elimination (0, the fixture's +0x30, `comp_util_scale_by_round`(0x4b, 0x64, round,
/// 2)) and the club leaves.
/// Original: `EuropeanCup::update_staff_records_for_status` (0x584ae0, european_cup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x584ae0 } (thiscall, ret 0x18, european_cup.cpp): `EuropeanCup::update_staff_records_for_status(clube,
/// desfecho, fase, _, jogo, final)`, a virtual +0x44 da Liga dos Campeões. Devolve 0. Comp = +4.
/// - Na qualificação (fase 0xff): desfecho 0 ou 1, a classificação pela rodada do jogo (+0x34): na rodada 0, com a
///   fase +0x3a e o +0x6f da primeira rodada (+0xa3) e o peso 0x4bf850(1, 0x14, 0, 2); noutra, 0x41c, 0 e
///   0x4bf850(1, 0x14, rodada, 2). Desfecho 0xff ou 2: a eliminação (0x86c000) com +0x32 e +0x30 do jogo e o peso
///   0x4bf850(1, 0x14, rodada, 2), e o clube sai (+0x1db = −1).
/// - Na fase 1 (fase < 8): 6, a classificação (0x44a, −1, 0x32); 0xfe, a não classificação (0x449, 0x32) e sai.
/// - Na fase 2 (8 a 11): 6, a classificação (0, 0x78, 0x32); 0xfe, a não classificação (0x44a, 0x32) e sai.
/// - No mata-mata (12): 1, o título (Staff_Records::add_cup_winners_record 0x86a800, thiscall(registros, clube, jogo,
///   final), ret 0xc) e sai; 2, o vice (0x86b0b0, idem) e sai; 0, a classificação (0, o +0x6f da rodada do jogo na
///   tabela de rodadas da fase do mata-mata ([+0xc] + 0x30, +0xa3, 0x68 bytes), 0x4bf850(0x4b, 0x64, rodada, 2));
///   0xff, a eliminação (0, +0x30 do jogo, 0x4bf850(0x4b, 0x64, rodada, 2)) e sai.
pub(crate) unsafe extern "thiscall" fn european_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a: u32, game: usize, end: u32) -> u32 {
    let d = outcome as u8;
    let f = stage as u8;
    let comp = read_usize(this + 4);
    if f == 0xff {
        match d {
            0 | 1 => {
                let r = read_i16(game + 0x34);
                if r == 0 {
                    let weight = scale_by_round(1, 0x14, 0, 2);
                    let round = read_i16(read_usize(this + cup::ROUND_TABLE) + 0x6f) as u16 as u32;
                    // simplification: the stage +0x3a goes in a register with garbage in the high 16 bits; 0x86bdd0 only uses the word.
                    european_cup_add_qualified_record(club, comp, read_i16(this + cup::STAGE_LABEL) as u16 as u32, round, weight);
                } else {
                    let weight = scale_by_round(1, 0x14, r, 2);
                    european_cup_add_qualified_record(club, comp, 0x41c, 0, weight);
                }
            }
            0xff | 2 => {
                let weight = scale_by_round(1, 0x14, read_i16(game + 0x34), 2);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words.
                european_cup_add_knocked_out_record(club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
                crate::comparison::branch("cpz_4 out in qualifying");
                leave_euro_competition(Club(club)); // cpz4: whoever is knocked out in qualifying leaves the European competition
            }
            _ => {}
        }
    } else if (f as i8) < 8 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0x44a, 0xffff_ffff, 0x32);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x449, 0x32);
            crate::comparison::branch("cpz_4 out in stage 1");
            leave_euro_competition(Club(club)); // cpz4: whoever is knocked out in stage 1 leaves the European competition
        }
    } else if (f as i8) < 0xc {
        if d == 6 {
            crate::comparison::branch("cpz_4 past stage 2");
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x32); // cpz4: passar da fase 2 pesa 0x32
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x44a, 0x32);
            leave_euro_competition(Club(club));
        }
    } else if f == 0xc {
        match d {
            1 => {
                exe::staff_records_add_cup_winners_record(eng_second_staff_records(), club, game, end);
                leave_euro_competition(Club(club));
            }
            2 => {
                exe::staff_records_add_cup_runners_up_record(eng_second_staff_records(), club, game, end);
                leave_euro_competition(Club(club));
            }
            0 => {
                let weight = scale_by_round(0x4b, 0x64, read_i16(game + 0x34), 2);
                let knockout = read_usize(read_usize(this + cup::STAGES) + 0x30);
                let rounds = read_usize(knockout + cup::ROUND_TABLE);
                let r = read_i16(game + 0x34) as i32;
                // the upper bits of the register the original pushes are those of the stages pointer (+0xc)
                let round = (read_usize(this + cup::STAGES) as u32 & 0xffff_0000) | read_i16(rounds.wrapping_add((r as usize).wrapping_mul(0x68)) + 0x6f) as u16 as u32;
                european_cup_add_qualified_record(club, comp, 0, round, weight);
            }
            0xff => {
                let weight = scale_by_round(0x4b, 0x64, read_i16(game + 0x34), 2);
                european_cup_add_knocked_out_record(club, comp, 0, read_i16(game + 0x30) as u16 as u32, weight);
                leave_euro_competition(Club(club));
            }
            _ => {}
        }
    }
    0
}

// simplification: the records go to the file, touch the reputations and morale (they roll) and the champion and runner-up write the
// final squad: they do not run twice. Only the original runs; the plan checks what the routine itself writes (+0x1db of the
// club: −1 in the cases where it leaves, untouched in the others).

/// Draws four distinct group indices (rand 4 each until unused).
///
/// [pt] Os quatro sorteados da fase 2 (os índices dos grupos, todos diferentes): a, b ≠ a, c ≠ a, b e d ≠ a, b, c, cada um
/// com sorteio(4) (0x90cfc0) até sair um que serve (a d sorteia pelo menos uma vez: começa igual à a).
pub(crate) unsafe fn draw_four_groups() -> [i8; 4] {
    let a = random(4) as i8;
    let mut b = random(4) as i8;
    while b == a {
        b = random(4) as i8;
    }
    let mut c = random(4) as i8;
    while c == a || c == b {
        c = random(4) as i8;
    }
    let mut d = a;
    while d == a || d == b || d == c {
        d = random(4) as i8;
    }
    [a, b, c, d]
}

/// Club in row k of second-stage group g (League get_table_row).
///
/// [pt] O clube da linha `k` do grupo `g` da fase 2 (a fase [+0xc] + 0x20 + 4 × g; League::get_table_row 0x6826d0,
/// thiscall(grupo, k), ret 4).
pub(crate) unsafe fn second_stage_group_club(this: usize, g: i8, k: u32) -> usize {
    read_usize(exe::league_get_table_row(read_usize(read_usize(this + cup::STAGES) + 0x20 + 4 * (g as i32 as usize)), (k) as i32))
}

/// Draws the order of the 4 phase 2 groups (`exe::utils_random`) and pairs the quarter-finals (1st of a v 2nd of b, 2nd of a v 1st
/// of b, same for c and d), resets the 8 clubs' bookings (`exe::rulingbody_reset_bookings_for_club`), creates the knockout stage (`construct_cup_stage`) in slot
/// +0x30, drops the 3rd and 4th of each group (+0x1db = -1), sets up the round (`set_up_round`) and sets +0x30 = 12.
///
/// `EuropeanCup::draw_quarter_finals()`, called from the next-stage routine `european_cup_advance_stage` at stage 12. The list of the 8
/// (calloc `exe::crt_calloc` of 8 × 4; out of memory: the error box at line 0x762) with the drawn groups a, b, c and d: 1st of a
/// × 2nd of b, 2nd of a × 1st of b, 1st of c × 2nd of d and 2nd of c × 1st of d. Each one goes through `exe::rulingbody_reset_bookings_for_club`
/// (thiscall(the object `[[0xae28f0]]` + 4 × +0x50, club), ret 4). The virtual +0x3c (create_round_table: stage 12) and
/// the knockout stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `construct_cup_stage` with the cup, 8, the list, the
/// rounds, +4, the table, +0x40, 0xc, 2, the extra, 0x14, 1, 0, 0, 0) into slot `[+0xc]` + 0x30; the list and the table
/// freed (`exe::crt_free`). Without a stage: the error box at line 0x78a. Otherwise the 3rd and 4th of each phase 2 group
/// leave (+0x1db = −1), the round is set up (Cup::set_up_round `set_up_round`, thiscall(stage, 0)) and +0x30 = 12.
/// Original: `EuropeanCup::draw_quarter_finals` (0x586330, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x586330 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::draw_quarter_finals()`, da fase seguinte 0x5851d0
/// na fase 12. A lista das 8 (calloc 0x945501 de 8 × 4; sem memória: a caixa da linha 0x762) com os grupos sorteados
/// a, b, c e d: o 1º de a × o 2º de b, o 2º de a × o 1º de b, o 1º de c × o 2º de d e o 2º de c × o 1º de d. Cada um
/// passa pela 0x7e9180 (thiscall(o objeto [[0xae28f0]] + 4 × +0x50, clube), ret 4). A virtual +0x3c (create_round_table:
/// fase 12) e a fase do mata-mata (operator new 0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00 com a copa, 8, a lista,
/// as rodadas, +4, a tabela, +0x40, 0xc, 2, o extra, 0x14, 1, 0, 0, 0) na vaga [+0xc] + 0x30; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa da linha 0x78a. Senão o 3º e o 4º de cada grupo da fase 2 saem (+0x1db =
/// −1), a rodada montada (Cup::set_up_round 0x51c800, thiscall(fase, 0)) e +0x30 = 12.
pub(crate) unsafe extern "thiscall" fn draw_quarter_finals(this: usize) {
    let list = exe::crt_calloc(8, 4);
    if list == 0 {
        return show_error_at(0x9c4900, 0x762);
    }
    crate::comparison::branch("cpz_4 quarter-final draw");
    let [a, b, c, d] = draw_four_groups();
    let order: [(i8, u32); 8] = [(a, 0), (b, 1), (a, 1), (b, 0), (c, 0), (d, 1), (c, 1), (d, 0)];
    for (k, &(g, l)) in order.iter().enumerate() {
        core::ptr::write_unaligned((list + 4 * k) as *mut usize, second_stage_group_club(this, g, l));
    }
    for k in 0..8 {
        let obj = read_usize(read_usize(read_usize(G_DISCIPLINE)) + 4 * (read_u8(this + cup::DISCIPLINE_INDEX) as i8 as i32 as usize));
        exe::rulingbody_reset_bookings_for_club(obj, read_usize(list + 4 * k));
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 0xc, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the rounds, the extra and the word +0x40 go in registers with garbage in the high 16 bits; the constructor
    // only reads the words.
    let stage = if p != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 8, list, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + cup::YEAR) as u16 as u32, 0xc, 2, extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + cup::STAGES) + 0x30) as *mut usize, stage);
    exe::crt_free(list);
    exe::crt_free(t);
    let stage = read_usize(read_usize(this + cup::STAGES) + 0x30);
    if stage == 0 {
        return show_error_at(0x9c4900, 0x78a);
    }
    for g in 0..4i8 {
        leave_euro_competition(Club(second_stage_group_club(this, g, 2)));
        leave_euro_competition(Club(second_stage_group_club(this, g, 3)));
    }
    exe::cup_set_up_round(stage, 0);
    core::ptr::write_unaligned((this + cup::CURRENT_STAGE) as *mut i32, 0xc);
}

// simplification: the draw rolls, 0x7e9180 and the constructor allocate and the round creates the matches: original only. The plan,
// done before: the 8 group winners and runners-up and the 3rd and 4th of each; then +0x30 = 12, the knockout stage
// with 8 clubs that are the first and second places (as a set) and the 3rd and 4th of each group with +0x1db = −1.

/// Entrants (+0x14, 6 bytes each) still in the competition (+0x1db = the cup id), optionally excluding entry round 7;
/// writes them to v and returns the count.
///
/// [pt] A lista dos inscritos (+0x14, 6 bytes: +0 o clube, +4 a rodada de entrada; +0x38 deles) que seguem na competição
/// (+0x1db = o id da copa, [+4]); com `sem_7`, fora os de rodada de entrada 7. Grava em `v` (até 71) e devolve a conta.
pub(crate) unsafe fn active_entrants(this: usize, sem_7: bool, v: *mut usize) -> i16 {
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while i < read_i16(this + cup::ENTRANT_CAPACITY) {
        let e = read_usize(this + cup::ENTRANT_BOOK).wrapping_add((i as i32 as usize).wrapping_mul(6));
        let c = read_usize(e);
        if core::ptr::read_unaligned((c + 0x1db) as *const i32) == core::ptr::read_unaligned(read_usize(this + 4) as *const i32) && !(sem_7 && read_u8(e + 4) == 7) {
            core::ptr::write_unaligned(v.add(n as i32 as usize), c);
            n = n.wrapping_add(1);
        }
        i += 1;
    }
    n
}

/// Allocates the list (0x11c bytes) of the entrants (+0x14) still in the cup (+0x1db = its id): kind 1 without the
/// round 7 entries, kind 2 all, kind 3 at stage 11; otherwise the competition's own (`comp_collect_club_list`); returns the count.
///
/// `EuropeanCup::collect_club_list(&list, kind)`, the Champions League's virtual +0x58. Kind 1: the entrants still in,
/// except those of round 7; kind 2: all those still in; kind 3 at stage 11: those still in; in any other case the
/// competition's one (Comp::collect_club_list `comp_collect_club_list`, thiscall(this, &list, 0), ret 8). The list (malloc `exe::crt_malloc` of
/// 0x11c bytes; out of memory: the error box at line 0x7b5, 0x7cf or 0x7e9, null list and 0) goes into `list`; returns
/// the count in the low word (the high word is the list pointer's).
/// Original: `EuropeanCup::collect_club_list` (0x586660, european_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x586660 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::collect_club_list(&lista, tipo)`, a virtual +0x58
/// da Liga dos Campeões. Tipo 1: os inscritos que seguem, fora os da rodada 7; tipo 2: todos os que seguem; tipo 3 na
/// fase 11: os que seguem; noutro caso a da competição (Comp::collect_club_list 0x48e1c0, thiscall(this, &lista, 0),
/// ret 8). A lista (malloc 0x944e46 de 0x11c bytes; sem memória: a caixa das linhas 0x7b5, 0x7cf ou 0x7e9, lista
/// nula e 0) vai em `lista`; devolve a conta na palavra de baixo (a de cima é a do ponteiro da lista).
pub(crate) unsafe extern "thiscall" fn european_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let t = kind as u8;
    let (linha, sem_7) = match t {
        1 => (0x7b5, true),
        2 => (0x7cf, false),
        3 if core::ptr::read_unaligned((this + cup::CURRENT_STAGE) as *const i32) == 0xb => (0x7e9, false),
        _ => {
            return (exe::comp_collect_club_list(this, out, 0) as u32);
        }
    };
    let v = exe::crt_malloc(0x11c);
    if v == 0 {
        show_error_at(0x9c4900, linha);
        core::ptr::write_unaligned(out as *mut usize, 0);
        return 0;
    }
    crate::comparison::branch("cpz_4 list of entrants");
    let n = active_entrants(this, sem_7, v as *mut usize);
    core::ptr::write_unaligned(out as *mut usize, v);
    (v as u32 & 0xffff_0000) | n as u16 as u32
}

/// CRT malloc(n).
pub(crate) unsafe fn european_cup_malloc(n: u32) -> usize {
    exe::crt_malloc(n)
}

/// Writes a 6-byte entrant record: club (+0), +5 = 0, entry round (+4), in the original's order.
///
/// [pt] Uma entrada de 6 bytes da lista dos inscritos: o clube (+0), a rodada em que entra (+4) e +5 = 0 (nesta ordem,
/// como a original).
pub(crate) unsafe fn european_cup_write_entrant(p: usize, club: usize, round: u8) {
    core::ptr::write_unaligned(p as *mut usize, club);
    *((p + 5) as *mut u8) = 0;
    *((p + 4) as *mut u8) = round;
}

/// Loop of `european_cup_collect_entrants` over the clubs ([0xae23b4], 0x245 bytes, [0xae235c] of them): clubs entered in the Champions League
/// go to entrant list `t` by band; returns the total count.
///
/// Clubs whose European competition (+0x1db) is the Champions League ([0x9cf6e8]) enter list `t` by band (+0x1df): 1 at
/// +0 (up to 16, round 7), 2 at +0x60 (up to 18, round 8), 3 at +0xcc (up to 19, round 0xd), 4 at +0x13e (up to 18,
/// round 0). Returns the sum of the four counts.
///
/// [pt] O laço da 0x584fa0 sobre os clubes ([0xae23b4], 0x245 bytes, [0xae235c] deles): quem tem a competição europeia
/// (+0x1db) da Liga dos Campeões ([0x9cf6e8]) entra pela banda (+0x1df) na lista `t`: 1 em +0 (até 16, rodada 7),
/// 2 em +0x60 (até 18, rodada 8), 3 em +0xcc (até 19, rodada 0xd), 4 em +0x13e (até 18, rodada 0). Devolve a soma
/// das quatro contas.
pub(crate) unsafe fn fill_champions_league_entrants(t: usize) -> i32 {
    let clubs = read_i32(G_CLUB_COUNT);
    let (mut n1, mut n2, mut n3, mut n4) = (0i32, 0i32, 0i32, 0i32);
    let (mut o1, mut o2, mut o3, mut o4) = (0usize, 0usize, 0usize, 0usize);
    if clubs <= 0 {
        return 0;
    }
    let cl = read_i32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    let mut c = read_usize(G_CLUB_TABLE);
    let mut i: i32 = 0;
    loop {
        if read_i32(c + 0x1db) == cl {
            match read_u8(c + 0x1df) {
                1 => {
                    if o1 < 0x60 {
                        european_cup_write_entrant(t + o1, c, 7);
                        n1 += 1;
                        o1 += 6;
                    }
                }
                2 => {
                    if o2 < 0x6c {
                        crate::comparison::branch("cpy_5 band 2 entry");
                        european_cup_write_entrant(t + 0x60 + o2, c, 8); // cpy5: band 2 enters in round 8
                        n2 += 1;
                        o2 += 6;
                    }
                }
                3 => {
                    if o3 < 0x72 {
                        crate::comparison::branch("cpy_5 band 3 entry");
                        european_cup_write_entrant(t + 0xcc + o3, c, 0xd); // cpy5: band 3 enters in round 0xd
                        n3 += 1;
                        o3 += 6;
                    }
                }
                4 => {
                    if o4 < 0x6c {
                        crate::comparison::branch("cpy_5 band 4 entry");
                        european_cup_write_entrant(t + 0x13e + o4, c, 0); // cpy5: band 4 enters in round 0
                        n4 += 1;
                        o4 += 6;
                    }
                }
                _ => {}
            }
        }
        c = c.wrapping_add(0x245);
        i += 1;
        if i as i16 as i32 >= clubs {
            break;
        }
    }
    n1 + n2 + n3 + n4
}

/// allocates the 71-entry list of 6-byte entries (+0x14) and fills it with the Champions League clubs (+0x1db =
/// [0x9cf6e8]) by band (+0x1df): 1 up to 16 (round 7), 2 up to 18 (8), 3 up to 19 (0xd), 4 up to 18 (0); short of 71,
/// +0x36 = 0 and returns 0
/// Original: `EuropeanCup::collect_entrants` (0x584fa0, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x584fa0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::collect_entrants()`, do construtor (0x583888, na
/// carreira nova) e do começo da temporada (0x584e5d). +0x36 e +0x38 = 71; libera a lista antiga (+0x14, free
/// 0x9452ca) e aloca 6 × 71 bytes (0x944e46; sem memória: a caixa da linha 0x44e e 0, com +0x14 nulo); preenche as
/// quatro bandas (fill_champions_league_entrants). Sem as 71 entradas: +0x36 = 0 e 0; senão 1.
pub(crate) unsafe extern "thiscall" fn european_cup_collect_entrants(this: usize) -> u32 {
    *((this + cup::ENTRANT_COUNT) as *mut i16) = 0x47;
    *((this + cup::ENTRANT_CAPACITY) as *mut i16) = 0x47;
    let old = read_usize(this + cup::ENTRANT_BOOK);
    if old != 0 {
        exe::crt_free(old);
    }
    let t = european_cup_malloc((read_i16(this + cup::ENTRANT_COUNT) as i32).wrapping_mul(6) as u32);
    core::ptr::write_unaligned((this + cup::ENTRANT_BOOK) as *mut usize, t);
    if t == 0 {
        show_error_at(0x9c4900, 0x44e);
        return 0;
    }
    if fill_champions_league_entrants(t) != read_i16(this + cup::ENTRANT_COUNT) as i32 {
        *((this + cup::ENTRANT_COUNT) as *mut i16) = 0;
        return 0;
    }
    1
}

/// Loop of `build_qualifying_entry_table` over the 55 table rows: each row gets an entrant from the list (reverse order) and a code from the
/// entry round (8 -> 2, 0xd -> 1, else 0).
///
/// [pt] O laço da 0x584990 sobre as 55 linhas (+0x3e) da tabela `t`: a linha i leva o clube da entrada (+0x36) − i − 1 da
/// lista (+0x14; lista vazia: 0) e a rodada pela entrada (+0x36) − i, byte +4 da anterior (8 → 2, 0xd → 1, outra →
/// 0); +5 = 0. Com +0x36 = 0 a original lê antes da lista (índice negativo), como aqui.
pub(crate) unsafe fn fill_qualifying_table(this: usize, t: usize) {
    let mut i: i32 = 0;
    let mut k = 0usize;
    while i < read_i16(this + cup::TEAM_COUNT) as i32 {
        let n = read_i16(this + cup::ENTRANT_COUNT) as i32;
        let list = read_usize(this + cup::ENTRANT_BOOK);
        let club = if n != 0 { read_usize(list.wrapping_add((n - i - 1).wrapping_mul(6) as usize)) } else { 0 };
        core::ptr::write_unaligned((t + k) as *mut usize, club);
        let list = read_usize(this + cup::ENTRANT_BOOK);
        let code = read_u8(list.wrapping_add(((n - i).wrapping_mul(6) - 2) as usize));
        *((t + k + 4) as *mut u8) = match code {
            8 => 2,
            0xd => 1,
            _ => 0,
        };
        *((t + k + 5) as *mut u8) = 0;
        i += 1;
        k += 6;
    }
}

/// allocates the 55 qualifying rows (+0xa7, +0x3e = 55): row i takes the club of entry 70 - i and its entry round (8 ->
/// 2, 0xd -> 1, other -> 0); out of memory, -1
/// Original: `EuropeanCup::build_qualifying_entry_table` (0x584990, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x584990 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::build_qualifying_entry_table()`, logo depois da
/// 0x584fa0 (0x58388f, 0x584e9a). +0x3e = 55; aloca 6 × 55 bytes em +0xa7 (sem memória: a caixa da linha 0x2f6 e −1)
/// e monta as linhas da qualificação de trás para a frente (fill_qualifying_table); devolve 1.
pub(crate) unsafe extern "thiscall" fn build_qualifying_entry_table(this: usize) -> i32 {
    *((this + cup::TEAM_COUNT) as *mut i16) = 0x37;
    let t = european_cup_malloc(0x14a);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut usize, t);
    if t == 0 {
        show_error_at(0x9c4900, 0x2f6);
        return -1;
    }
    fill_qualifying_table(this, t);
    1
}

/// Moves to the next stage (+0x30) and draws the phase 1 groups (stage 0), the phase 2 groups (8) or the quarter-finals
/// (12); at the last stage, the error box.
///
/// `EuropeanCup::advance_stage()` (the virtual +0x28, from the cup pass 0x51f904). At the last stage (+0x30 ≥ +0x2c −
/// 1, signed): the error box at line 0x495. Otherwise +0x30 + 1 and, at stage 0, the phase 1 group draw (`draw_phase1_groups`,
/// thiscall(this)); at 8, the phase 2 draw (`draw_phase2_groups`, thiscall(this, 0), ret 4); at 12, the quarter-finals (`draw_quarter_finals`,
/// thiscall(this)). The caller does not read the return value.
/// Original: `EuropeanCup::advance_stage` (0x5851d0, european_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5851d0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::advance_stage()` (a virtual +0x28, do passe da
/// copa 0x51f904). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x495. Senão +0x30 + 1 e, na fase
/// 0, o sorteio dos grupos da fase 1 (0x585290, thiscall(this)); na 8, os da fase 2 (0x585ab0, thiscall(this, 0),
/// ret 4); na 12, as quartas (0x586330, thiscall(this)). Quem chama não lê a devolução.
pub(crate) unsafe extern "thiscall" fn european_cup_advance_stage(this: usize) {
    let f = read_i32(this + cup::CURRENT_STAGE);
    if f >= read_i32(this + cup::STAGE_COUNT).wrapping_sub(1) {
        return show_error_at(0x9c4900, 0x495);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + cup::CURRENT_STAGE) as *mut i32, f);
    crate::comparison::branch("cpy_5 next stage");
    match f {
        0 => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE1_GROUPS);
            sortear(this);
        }
        8 => {
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE2_GROUPS);
            sortear(this, 0);
        }
        0xc => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROPEANCUP_DRAW_QUARTER_FINALS);
            sortear(this);
        }
        _ => {}
    }
}

// simplification: the draws roll, allocate the stages and send news: original only; the copy takes the branch.

/// Club's finance record (FINANCETABLE get_club_finance on G_FINANCE_TABLE).
///
/// [pt] O registro de finanças do clube (0x5a0590: thiscall(tabela [0xae2a38], clube), ret 4).
pub(crate) unsafe fn european_cup_club_finance(club: usize) -> usize {
    exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club)
}

/// Adds prize money to the finance record (balance, +0xa8 and +0x148).
///
/// [pt] Soma no saldo, em +0xa8 e em +0x148 (0x5999a0: thiscall(finanças, valor), ret 4).
pub(crate) unsafe fn european_cup_add_prize_money(fin: usize, value: i32) {
    exe::finance_add_prize_money(fin, value)
}

/// Adds coefficient points for the nation (UEFASEEDING add_nation_points).
///
/// [pt] Os pontos do coeficiente da UEFA para a nação (0x9058b0: thiscall(tabela [0xde1f58], nação, pontos), ret 8; soma
/// o byte dos pontos na palavra +0xf + 18 × índice (+0x1df da seleção 0x53b3d0 da nação)).
pub(crate) unsafe fn add_uefa_coefficient(nation: usize, points: u32) {
    exe::uefaseeding_add_nation_points(read_usize(G_UEFA_SEEDING), nation, points);
}

/// The dry run of the comparer of `0x583b10`.
///
/// The copy runs without the prize news (`0x48e3e0`, which allocates) and without the cup base (`0x51a150`, which
/// processes the whole match and cannot run twice).
///
/// [pt] O ensaio do comparador da 0x583b10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca) e sem a base da copa
/// (0x51a150, que processa o jogo inteiro e não pode rodar duas vezes).
pub(crate) static mut EUROPEAN_CUP_FIXTURE_DRY_RUN: bool = false;

/// Sends the prize money news (Comp `send_prize_money_news`); nothing in dry run.
///
/// [pt] A notícia do prêmio (Comp 0x48e3e0: thiscall(this, clube, valor, 0, −1, 0, jogo, −2), ret 0x1c); no ensaio, nada.
pub(crate) unsafe fn european_cup_send_prize_money_news(this: usize, club: usize, value: u32, game: usize) {
    if EUROPEAN_CUP_FIXTURE_DRY_RUN {
        return;
    }
    exe::comp_send_prize_money_news(this, club, (value) as i32, (0) as i32, -1, 0, game, -2);
}

/// by the score (+0x47/+0x48, or +0x43/+0x44), 2 UEFA coefficient points to the winner's nation or 1 each on a draw
/// (0x9058b0); in the group stage (+0x42 from 0 to 11), 200 thousand to the winner or 100 thousand each, with the prize
/// news item (`send_prize_money_news`); then the cup base (`exe::cup_process_fixture`)
/// Original: `EuropeanCup::process_fixture` (0x583b10, european_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x583b10 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::process_fixture(jogo, arg)` (a virtual +4 da Liga dos
/// Campeões, a cada jogo dela). O placar: +0x47/+0x48 (com +0x47 = −1, +0x43/+0x44), com sinal. O vencedor ganha 2
/// pontos no coeficiente da sua nação (+0x53; 0x9058b0), o empate 1 para cada. Na fase de grupos (+0x42 de 0 a 11):
/// 200 mil ao vencedor ou 100 mil a cada um no empate (0x5a0590 + 0x5999a0), com a notícia do prêmio (0x48e3e0;
/// no empate, as duas somas antes das duas notícias). Depois a base da copa (0x51a150 com o jogo e `arg`).
pub(crate) unsafe extern "thiscall" fn process_fixture(this: usize, game: usize, arg: u32) {
    let (ga, gb) = if read_u8(game + fixture::HOME_GOALS_EXTRA_TIME) != 0xff {
        (read_u8(game + fixture::HOME_GOALS_EXTRA_TIME) as i8, read_u8(game + fixture::AWAY_GOALS_EXTRA_TIME) as i8)
    } else {
        (read_u8(game + fixture::HOME_GOALS) as i8, read_u8(game + fixture::AWAY_GOALS) as i8)
    };
    let (a, b) = (read_usize(game + fixture::HOME_CLUB), read_usize(game + fixture::AWAY_CLUB));
    let (vencedor, perdedor) = if ga > gb {
        add_uefa_coefficient(read_usize(a + 0x53), 2);
        (read_usize(game + fixture::HOME_CLUB), read_usize(game + fixture::AWAY_CLUB))
    } else if ga < gb {
        add_uefa_coefficient(read_usize(b + 0x53), 2);
        (read_usize(game + fixture::AWAY_CLUB), read_usize(game + fixture::HOME_CLUB))
    } else {
        add_uefa_coefficient(read_usize(a + 0x53), 1);
        add_uefa_coefficient(read_usize(read_usize(game + fixture::AWAY_CLUB) + 0x53), 1);
        (0, 0)
    };
    let _ = perdedor;
    let stage = read_u8(game + fixture::STAGE_INDEX) as i8;
    if stage > -1 && stage < 0xc {
        if vencedor != 0 {
            crate::comparison::branch("cpy_6 win bonus");
            european_cup_add_prize_money(european_cup_club_finance(vencedor), 0x30d40); // cpy6: the win prize
            european_cup_send_prize_money_news(this, vencedor, 0x30d40, game);
        } else {
            crate::comparison::branch("cpy_6 draw bonus");
            european_cup_add_prize_money(european_cup_club_finance(read_usize(game + fixture::HOME_CLUB)), 0x186a0); // cpy6: the home side’s draw prize
            european_cup_add_prize_money(european_cup_club_finance(read_usize(game + fixture::AWAY_CLUB)), 0x186a0);
            european_cup_send_prize_money_news(this, read_usize(game + fixture::HOME_CLUB), 0x186a0, game);
            european_cup_send_prize_money_news(this, read_usize(game + fixture::AWAY_CLUB), 0x186a0, game);
        }
    }
    if EUROPEAN_CUP_FIXTURE_DRY_RUN {
        return;
    }
    exe::cup_process_fixture(this, game, arg);
}

/// Address of the club nation's UEFA coefficient word ([G_UEFA_SEEDING] + 18 x national-team +0x1df + 0xf); 0 if
/// missing.
///
/// [pt] A palavra do coeficiente da nação do clube: [0xde1f58] + 18 × (+0x1df da seleção 0x53b3d0 da nação +0x53) + 0xf,
/// como a 0x9058b0 calcula; 0 sem tabela, sem nação ou sem seleção.
pub(crate) unsafe fn nation_coefficient_word(club: Club) -> usize {
    let table = read_usize(G_UEFA_SEEDING);
    let nation = club.nation().addr();
    if table == 0 || nation == 0 {
        return 0;
    }
    let s = exe::database_get_national_team(nation);
    if s == 0 {
        return 0;
    }
    table.wrapping_add((read_u8(s + 0x1df) as i8 as isize * 18) as usize + 0xf)
}

/// Converts the three progress codes by stage (qualifying, phase 1, phase 2) and writes them to +0x73..+0x75 of the
/// club's competition stats entry.
///
/// `EuropeanCup::set_club_progress(club, stage, p1, p2, p3)` (the virtual +0x68, from the cup base `record_team_progress`): converts
/// the three progress codes by stage and writes them into the club's record in the competition statistics (`exe::comp_stats_find_club_entry` on
/// +8, without checking for null, as the original) +0x73 ← p1, +0x74 ← p2, +0x75 ← p3. Stage −1 (qualifying): 1 → 0x19
/// (p2: 1), 17 → 0x21, 33 → 0x31, 47 → 0x3f; stage below 8 (signed): 1 and 2 → 1 (p3: 0xd), 3 → 0x11, 4 → 0x19; below
/// 12: 1 and 2 → 1 (p3: 7), 3 → 9, 4 → 0xd; the other codes and the stages from 12 on pass through unchanged.
/// Original: `EuropeanCup::set_club_progress` (0x586c00, european_cup.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x586c00 } (thiscall, ret 0x14, european_cup.cpp): `EuropeanCup::set_club_progress(clube, fase, p1, p2, p3)` (a
/// virtual +0x68, da base da copa 0x521b30): converte os três códigos de progresso pela fase e grava no registro do
/// clube na estatística da competição (0x4a4850 de +8, sem conferir o nulo, como a original) +0x73 ← p1, +0x74 ← p2,
/// +0x75 ← p3. Fase −1 (a qualificação): 1 → 0x19 (p2: 1), 17 → 0x21, 33 → 0x31, 47 → 0x3f; fase abaixo de 8 (com
/// sinal): 1 e 2 → 1 (p3: 0xd), 3 → 0x11, 4 → 0x19; abaixo de 12: 1 e 2 → 1 (p3: 7), 3 → 9, 4 → 0xd; os outros
/// códigos e as fases de 12 em diante passam como estão.
pub(crate) unsafe extern "thiscall" fn european_cup_set_club_progress(this: usize, club: usize, stage: u32, p1: u32, p2: u32, p3: u32) {
    let e = exe::comp_stats_find_club_entry(read_usize(this + 8), club);
    let (stage, p1, p2, p3) = (stage as u8 as i8, p1 as u8, p2 as u8, p3 as u8);
    let (c73, c74, c75) = if stage == -1 {
        let m = |p: u8, um: u8| match p {
            1 => um,
            17 => 0x21,
            33 => 0x31,
            47 => 0x3f,
            _ => p,
        };
        (m(p1, 0x19), m(p2, 1), m(p3, 0x19))
    } else if stage < 8 {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 0x11,
            4 => 0x19,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 0xd))
    } else if stage < 0xc {
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

/// Builds the round table: in the group stages, the 6-round schedule (phase 1 dates in September and October and phase
/// 2 from November to March, by half) with fixture 0 of each round empty; the qualifying rounds and the knockout stay
/// with the original.
///
/// Method +0x3c of the Champions League, SEH frame for the local dates with no destructor on the normal path:
/// `EuropeanCup::create_round_table(stage, &rounds, &id, &is_league)`, called from the group draws (`draw_phase1_groups`, `draw_phase2_groups`)
/// and the predicted positions (`european_cup_rank_teams`). In the group stages (char 0..11, and the negatives below −1, which fall into
/// the same branch): is_league (if requested) = 1, rounds = 6, id = 0x43d + stage; the table of 6 rounds of 0x41 bytes
/// (`exe::crt_malloc`(0x186); out of memory, the error box at line 0x17a or 0x1aa and it returns 0); in the second phase,
/// beforehand, the date of 1 March of the following year moved to the Tuesday (`exe::date_date_549f70`), one day earlier (0x54c8e0,
/// which writes its static date) and the copy (`exe::date_operator_3d`), unused afterwards (the copy calls them the same way); the
/// dates of `GROUP_STAGE_DATES` (`league_round_set_date` with the cup's year +0x40) and fixture 0 of each round empty (`store_round_match` with
/// −1). The qualifying rounds (−1) and the knockout (12 onwards): the original through the trampoline. Returns the
/// table.
/// Original: `EuropeanCup::create_round_table` (0x583c50, european_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x583c50 } (thiscall, ret 0x10, european_cup.cpp, método +0x3c da Liga dos Campeões, quadro SEH pelas datas locais
/// sem destrutor no caminho normal): `EuropeanCup::create_round_table(fase, &rodadas, &id, &é_liga)`, dos sorteios dos
/// grupos (0x585290, 0x585ab0) e das posições previstas (0x586900). Nas fases de grupos (char 0..11, e as negativas
/// abaixo de −1, que caem no mesmo ramo): é_liga (se pedido) = 1, rodadas = 6, id = 0x43d + fase; a tabela de 6 rodadas
/// de 0x41 bytes (0x944e46(0x186); sem memória, a caixa da linha 0x17a ou 0x1aa e devolve 0); na segunda fase, antes, a
/// data de 1º de março do ano seguinte levada à terça (0x549f70), um dia antes (0x54c8e0, que grava a data estática dela)
/// e a cópia (0x417c10), sem uso depois (a cópia as chama do mesmo jeito); as datas de `GROUP_STAGE_DATES` (0x68a160 com o ano
/// +0x40 da copa) e o jogo 0 de cada rodada vazio (0x68a1c0 com −1). As preliminares (−1) e o mata-mata (12 em diante):
/// a original pelo trampolim. Devolve a tabela.
pub(crate) unsafe extern "thiscall" fn european_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, id: *mut i16, league: *mut i32) -> usize {
    let f = stage as u8 as i8;
    if f == -1 || f >= 0xc {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUROPEAN_CUP_CREATE_ROUND_TABLE)(this, stage, rounds, id, league);
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 6;
    *id = (f as i16).wrapping_add(0x43d);
    let t = exe::crt_malloc(0x186);
    if t == 0 {
        show_error_at(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
        return 0;
    }
    let year = core::ptr::read_unaligned((this + cup::YEAR) as *const i16) as u16 as u32;
    let half = if f < 8 {
        if f < 4 { 0 } else { 1 }
    } else {
        let copy: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let (mut d, mut t1, mut t2) = ([0u16; 4], [0u16; 4], [0u16; 4]);
        // simplification: the year + 1 goes in ax with the rest of eax in the high bytes; 0x549f70 reads the year as a word
        exe::date_date_549f70(&mut d, 1, 2, (year as u16).wrapping_add(1) as u32, 2);
        let r = exe::date_sub_days((&mut d) as *mut [u16; 4] as usize, (&mut t1) as *mut [u16; 4] as usize, 1);
        copy(&mut d, &mut t2, r);
        if f < 10 { 2 } else { 3 }
    };
    crate::comparison::branch("cqh_4 round fixtures");
    for &(i, dia, mes, anos, semana) in GROUP_STAGE_DATES[half].iter() {
        exe::league_round_set_date(t, i, dia, mes, anos, semana, 2, year, 0);
    }
    for r in 0..6 {
        exe::league_round_set_match_slot(t, r, 0, -1, -1, -1, 0);
    }
    t
}

