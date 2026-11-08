use super::*;

/// Constructs a cup knockout stage: the round table copied from the cup's template (dates, draws), the year, the stage,
/// the id and the parent's rules (+0x49, +0x4a), the team list with pot seeds, and the fixture tree.
///
/// `Cup_Stage::Cup_Stage(parent, n, teams, rounds, a5, template, year, stage, b2, id, b3, tree, by_order, a14, seeds)`;
/// SEH frame with no destructor on the normal path. The base (Cup::Cup `exe::new_cup`, mov ecx -> thiscall, ret 0); +4 = a5;
/// +0x40 = the year (word); +0x44 = the stage, +0x42 = b2 (bytes); +0x30 = +0x2c = −1; +0x51 = b3; the vtable 0x969468;
/// +0x43 = 1; +0xb1 = +0xab = +0xac = 0; +0x3c = the rounds and +0x3a = the id (words); +0x49 and +0x4a copied from the
/// parent; +0x28 = a14. The round table (`exe::crt_malloc` of rounds × 0x68 at +0xa3; out of memory, the box for line 0x48) is
/// copied from the template; +0x3e = n and the team list (calloc `exe::crt_calloc`(n, 6) at +0xa7; out of memory, the box for
/// line 0x53): each team gets status (+5) 0 and its pot seed (+4): the one from the `seeds` vector (given by the cup),
/// or, without it, the position in the list if `by_order`, or 0. Finally the fixture tree, built at once: with `tree`,
/// Cup_Stage::map_fixture_tree (0x5241e0, which links the winners and places the seeds in the first round), otherwise
/// Cup::map_fixture_tree (`map_fixture_tree`, which links at random), both thiscall, ret 0 (they draw); if negative, the box for
/// line 0x62 or 0x6a. Returns this.
/// Original: `Cup_Stage::Cup_Stage` (0x522e00, cup_stage.cpp, thiscall, ret 0x3c).
///
/// [pt] { 0x522e00 } (thiscall, ret 0x3c, cup_stage.cpp; quadro SEH sem destrutor no caminho normal): `Cup_Stage::Cup_Stage(pai,
/// n, times, rodadas, a5, modelo, ano, fase, b2, id, b3, arvore, por_ordem, a14, sementes)`: a base (Cup::Cup 0x518640,
/// mov ecx -> thiscall, ret 0); +4 = a5; +0x40 = o ano (word); +0x44 = a fase, +0x42 = b2 (bytes); +0x30 = +0x2c = −1;
/// +0x51 = b3; a vtable 0x969468; +0x43 = 1; +0xb1 = +0xab = +0xac = 0; +0x3c = as rodadas e +0x3a = o id (words);
/// +0x49 e +0x4a copiados do pai; +0x28 = a14. A tabela das rodadas (0x944e46 de rodadas × 0x68 em +0xa3; sem memória, a
/// caixa da linha 0x48) copiada do modelo; +0x3e = n e a lista dos times (calloc 0x945501(n, 6) em +0xa7; sem memória, a
/// caixa da linha 0x53): cada time com a situação (+5) 0 e a semente (+4): a do vetor `sementes`, ou, sem ele, a posição
/// na lista se `por_ordem`, ou 0. Por fim a árvore dos confrontos: com `arvore`, Cup_Stage::map_fixture_tree (0x5241e0),
/// senão Cup::map_fixture_tree (0x518790), as duas thiscall, ret 0 (sorteiam); negativa, a caixa da linha 0x62 ou 0x6a.
/// Devolve this.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn construct_cup_stage(this: usize, parent: usize, n: u32, teams: usize, rounds: u32, a5: u32, template: usize, year: u32, stage: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, by_order: u32, a14: u32, seeds: usize) -> usize {
    const F: usize = 0x9b5944;
    exe::new_cup(this);
    write_usize(this + 4, a5 as usize);
    write_u16(this + cup::YEAR, year as u16); // cqs7: the stage year
    write_u8(this + cup::CLUB_SIDES, b2 as u8);
    write_u8(this + cup::STAGE_INDEX, stage as u8);
    write_usize(this + cup::CURRENT_STAGE, usize::MAX);
    write_usize(this + cup::STAGE_COUNT, usize::MAX);
    write_u8(this + cup::IMPORTANCE_BIAS, b3 as u8);
    write_usize(this, 0x969468);
    write_u8(this + 0x43, 1);
    write_u8(this + cup::RUN_BY_STAGES, 0);
    write_u8(this + cup::CURRENT_ROUND, 0);
    write_u8(this + cup::NEXT_DRAW_ROUND, 0);
    write_u16(this + cup::ROUND_COUNT, rounds as u16);
    write_u16(this + cup::STAGE_LABEL, id as u16);
    write_u8(this + cup::SUBSTITUTES_NAMED, *((parent + 0x49) as *const u8));
    write_u8(this + cup::SUBSTITUTIONS_ALLOWED, *((parent + 0x4a) as *const u8));
    write_usize(this + cup::PRIZE_FUND, a14 as usize);
    let size = (rounds as u16 as i16 as i32 * 0x68) as u32;
    let t = exe::crt_malloc(size);
    write_usize(this + cup::ROUND_TABLE, t);
    if t == 0 {
        show_error_at(F, 0x48);
        return this;
    }
    core::ptr::copy_nonoverlapping(template as *const u8, t as *mut u8, size as usize);
    write_u16(this + cup::TEAM_COUNT, n as u16);
    let l = exe::crt_calloc(n as u16 as i16 as i32 as u32, 6);
    write_usize(this + cup::TEAM_TABLE, l);
    if l == 0 {
        show_error_at(F, 0x53);
        return this;
    }
    crate::comparison::branch("cqs_7 fase de copa");
    let m = read_i16(this + cup::TEAM_COUNT) as i32;
    let mut i = 0i32;
    while i < m {
        let e = i as usize * 6;
        write_usize(read_usize(this + cup::TEAM_TABLE) + e, read_usize(teams + i as usize * 4)); // cqs7: o time i
        write_u8(read_usize(this + cup::TEAM_TABLE) + e + 5, 0);
        let s = if seeds != 0 {
            *((seeds + i as usize) as *const u8) // cqs7: the vector seed
        } else if by_order != 0 {
            i as u8
        } else {
            0
        };
        write_u8(read_usize(this + cup::TEAM_TABLE) + e + 4, s);
        i += 1;
    }
    let (map, linha) = if arvore != 0 { (0x5241e0usize, 0x62) } else { (0x518790usize, 0x6a) };
    let map: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(map);
    if map(this) < 0 {
        show_error_at(F, linha);
    }
    this
}

/// Destroys a knockout stage: frees the team list, the round table and each round's fixtures, then destroys the base.
///
/// `Cup_Stage::~Cup_Stage()`, the destructor, called by the virtual destructor. Sets the vtable 0x969468; the team list
/// (+0xa7), the round table (+0xa3) and, if there are fixtures (+0xad), each fixture vector of the +0x3c rounds and the
/// vector itself are freed (`exe::crt_free`, cdecl; the pointers are left in place); finally the base (Cup::~Cup `exe::drop_cup`, mov
/// ecx -> thiscall, ret 0), whose eax it returns.
/// Original: `Cup_Stage::~Cup_Stage` (0x523300, cup_stage.cpp, thiscall, ret 0).
///
/// [pt] { 0x523300 } (thiscall, ret 0, cup_stage.cpp; o destrutor, chamado pelo destrutor virtual): `Cup_Stage::~Cup_Stage()`:
/// a vtable 0x969468; a lista dos times (+0xa7), a tabela das rodadas (+0xa3) e, com os confrontos (+0xad), cada vetor de
/// confrontos das +0x3c rodadas e o próprio vetor liberados (0x9452ca, cdecl; os ponteiros ficam); por fim a base
/// (Cup::~Cup 0x518690, mov ecx -> thiscall, ret 0), cujo eax ela devolve.
pub(crate) unsafe extern "thiscall" fn destroy_cup_stage(this: usize) -> usize {
    write_usize(this, 0x969468);
    if read_usize(this + cup::TEAM_TABLE) != 0 {
        exe::crt_free(read_usize(this + cup::TEAM_TABLE));
    }
    if read_usize(this + cup::ROUND_TABLE) != 0 {
        exe::crt_free(read_usize(this + cup::ROUND_TABLE));
    }
    if read_usize(this + cup::TIE_TABLES) != 0 {
        let mut i = 0i32;
        while i < read_i16(this + cup::ROUND_COUNT) as i32 {
            let p = read_usize(read_usize(this + cup::TIE_TABLES) + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
            }
            i += 1;
        }
        exe::crt_free(read_usize(this + cup::TIE_TABLES));
    }
    crate::comparison::branch("cqs_7 fim da fase de copa");
    exe::drop_cup(this)
}

// simplification: frees the tables and destroys the database; cannot run twice: original only

