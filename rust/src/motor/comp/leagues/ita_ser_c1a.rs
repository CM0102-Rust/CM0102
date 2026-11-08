use super::*;



/// Club of row i of the league table (+0xb1, 0x3b bytes).
///
/// [pt] O clube da linha `i` da tabela [+0xb1] (0x3b bytes).
pub(crate) unsafe fn ita_ser_c1a_league_row_club(liga: usize, i: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(liga + 0xb1) + i * 0x3b)
}

/// League::advance_stage body: error box at the last stage; else +0x30 + 1 and, reaching stage 0, creates the
/// end-of-league stages.
///
/// [pt] A passagem de fase das três (League::advance_stage, o slot +0x28; o corpo de 0x64b9e0, 0x64fba0 e 0x578330): com a
/// fase atual (+0x30) já na última (+0x2c − 1, com sinal), a caixa do fonte `arquivo` na linha `linha`; senão +0x30 + 1
/// e, ao chegar à fase 0 (a primeira depois da liga), cria as fases (`criar`: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe fn ita_ser_c1a_advance_stage_cql5_passar_de_fase(this: usize, arquivo: usize, linha: u32, criar: usize) {
    let fase = read_u32_as_usize(this + 0x30) as i32;
    if fase >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        show_error_at(arquivo, linha);
        return;
    }
    let nova = fase.wrapping_add(1);
    write_u32_from_usize(this + 0x30, nova as usize);
    if nova == 0 {
        crate::comparacao::ramo("cql_5 fases do fim da liga");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(criar);
        f(this); // cql5: as fases do fim da liga
    }
}

/// after the league, the current stage goes up; on the first, the play-off and the play-out are created; past the last,
/// the error box
/// Original: `ItaSerC1a::advance_stage` (0x64b9e0, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64b9e0 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::advance_stage()`; cria o play-off e o play-out pela
/// 0x64ba70. Fonte 0x9ea474, linha 0x250.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_passar_de_fase(this, 0x9ea474, 0x250, 0x64ba70)
}

/// the same as C1/A
/// Original: `ItaSerC1b::advance_stage` (0x64fba0, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64fba0 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::advance_stage()`; pela 0x64fc30. Fonte 0x9ea558, linha
/// 0x255.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_advance_stage_64fba0(this: usize) {
    ita_ser_c1a_advance_stage_cql5_passar_de_fase(this, 0x9ea558, 0x255, 0x64fc30)
}

/// after the Third Division league, the current stage goes up and, on the first, the promotion play-off is created
/// (0x5783c0)
/// Original: `EngThird::advance_stage` (0x578330, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x578330 } (thiscall, ret 0, eng_third.cpp): `EngThird::advance_stage()`; cria o play-off da Third Division pela
/// 0x5783c0 (já em Rust). Fonte 0x9c4424, linha 0x29c.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_advance_stage_578330(this: usize) {
    ita_ser_c1a_advance_stage_cql5_passar_de_fase(this, 0x9c4424, 0x29c, 0x5783c0)
}

/// Comparer of advance_stage: side by side on +0x30 when no stage is created; else only the original.
///
/// [pt] O comparador das três: sem criar fase (a fase atual só sobe), lado a lado em +0x30; na caixa ou ao criar as fases
/// (alocam), só a original (a criação tem o seu comparador).
pub(crate) unsafe fn ita_ser_c1a_compare_advance_stage(nome: &str, this: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let fase = read_u32_as_usize(this + 0x30) as i32;
    if fase >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) || fase.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x30, 4)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", nome, this, mo, mr));
}




/// As linhas dos 4 clubes do play-off de acesso (2º contra 5º, 3º contra 4º: a lista é 5º, 2º, 4º, 3º) e do play-out
/// (14º contra 17º, 15º contra 16º: 17º, 14º, 16º, 15º), na ordem da lista que vai para a fase.
pub(crate) const PROMOTION_PLAYOFF_ROWS: [usize; 4] = [4, 1, 3, 2]; // cql5: as linhas do play-off de acesso
pub(crate) const PLAYOUT_ROWS: [usize; 4] = [16, 13, 15, 14]; // cql5: as linhas do play-out

/// Creates the end-of-league stages: champion to the Serie C1 Super Cup, play-off and play-out Cup_Stages with their
/// first-round draw.
///
/// [pt] O corpo das duas (0x64ba70 e 0x64fc30, iguais fora o fonte, as linhas e a ordem de soltar as listas no fim):
/// 1) o campeão (a linha 0) vai para a Supercopa da Série C1 (a competição [[0xadadfc] + [0x9cf938] × 4], a lista de
///    inscritos [+0xa7] de 6 bytes): na vaga 0 se vazia, senão na vaga 1;
/// 2) a lista de 4 clubes (malloc 0x944e46 de 0x10: push; add esp, 4 -> cdecl; sem memória: a caixa da linha
///    `linhas[0]` e volta) com os da faixa do play-off (PROMOTION_PLAYOFF_ROWS);
/// 3) o calendário da fase 0 (a virtual +0x3c: push 0; push &extra; push &rodadas; push 0; mov ecx, this -> thiscall,
///    ret 0x10; devolve o gabarito, `rodadas` e `extra` são palavras);
/// 4) a fase (operator new 0x944cf1 de 0xb2: push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00: 15 argumentos
///    empurrados, mov ecx, nova -> thiscall, ret 0x3c: a liga, os clubes (+0xbf, o byte com sinal), a lista, as
///    rodadas, o registro da competição (+4), o gabarito, o ano (+0x40), 0, 1, o `extra`, 0x14, 0, 0, 0 e as posições
///    dos 4 na tabela (registro local de 4 bytes: 4, 1, 3, 2)) vai para [+0xc][0]; sem fase: a caixa da linha
///    `linhas[1]` e volta; senão o sorteio da primeira rodada (Cup::set_up_round 0x51c800: push 0; mov ecx, fase ->
///    thiscall, ret 4) e solta o gabarito (0x9452ca: push; add esp, 4 -> cdecl);
/// 5) a mesma coisa com a fase 1 e os do play-out (PLAYOUT_ROWS; as posições 16, 13, 15, 14 no registro local e
///    1, 1 nos dois bytes) em [+0xc][1]; solta a lista e o gabarito (na ordem `lista_primeiro`); sem fase: a caixa da
///    linha `linhas[2]`; senão o sorteio da primeira rodada e +0x30 = 1.
/// Os bits de cima das palavras que a original empurra como dupla (os clubes, as rodadas, o ano e o `extra`) são lixo
/// da pilha ou de outros registradores; o construtor só lê a palavra (0x522e00: mov ax, word ptr [esp + ...]) e a
/// cópia passa zero neles.
pub(crate) unsafe fn create_end_of_league_stages(this: usize, arquivo: usize, linhas: [u32; 3], lista_primeiro: bool) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let fase_de_copa: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    let calendario: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    // a Supercopa da Série C1
    let copa = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_C1_SUPER_CUP) * 4);
    let inscritos = read_u32_as_usize(copa + 0xa7);
    let campeao = ita_ser_c1a_league_row_club(this, 0);
    if read_u32_as_usize(inscritos) != 0 {
        write_u32_from_usize(inscritos + 6, campeao);
    } else {
        write_u32_from_usize(inscritos, campeao);
    }
    let lista = alocar(0x10);
    if lista == 0 {
        show_error_at(arquivo, linhas[0]);
        return;
    }
    for (k, &i) in PROMOTION_PLAYOFF_ROWS.iter().enumerate() {
        write_u32_from_usize(lista + k * 4, ita_ser_c1a_league_row_club(this, i));
    }
    let clubes = *((this + 0xbf) as *const i8) as i16 as u16 as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let registro = read_u32_as_usize(this + 4);
    let (mut rodadas, mut extra) = (0u32, 0u32);
    let gabarito = calendario(this, 0, &mut rodadas, &mut extra, 0);
    let posicoes: [u8; 4] = [4, 1, 3, 2]; // cql5: as posições do play-off
    let mut fase = novo(0xb2);
    if fase != 0 {
        fase = fase_de_copa(fase, this, clubes, lista, rodadas & 0xffff, registro, gabarito, ano, 0, 1, extra & 0xffff, 0x14, 0, 0, 0, posicoes.as_ptr());
    }
    write_u32_from_usize(read_u32_as_usize(this + 0xc), fase);
    if read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        show_error_at(arquivo, linhas[1]);
        return;
    }
    sortear(read_u32_as_usize(read_u32_as_usize(this + 0xc)), 0);
    soltar(gabarito);
    let (mut rodadas, mut extra) = (0u32, 0u32);
    let calendario: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let gabarito = calendario(this, 1, &mut rodadas, &mut extra, 0);
    for (k, &i) in PLAYOUT_ROWS.iter().enumerate() {
        write_u32_from_usize(lista + k * 4, ita_ser_c1a_league_row_club(this, i));
    }
    let posicoes: [u8; 4] = [0x10, 0xd, 0xf, 0xe];
    let mut fase = novo(0xb2);
    if fase != 0 {
        fase = fase_de_copa(fase, this, clubes, lista, rodadas & 0xffff, registro, gabarito, ano, 1, 1, extra & 0xffff, 0x14, 0, 0, 0, posicoes.as_ptr());
    }
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + 4, fase);
    if lista_primeiro {
        soltar(lista);
        soltar(gabarito);
    } else {
        soltar(gabarito);
        soltar(lista);
    }
    if read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4) == 0 {
        show_error_at(arquivo, linhas[2]);
        return;
    }
    sortear(read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4), 0);
    write_u32_from_usize(this + 0x30, 1);
}

/// the C1/A rule (18 clubs): the champion goes up directly and enters the Serie C1 Super Cup; 2nd v 5th and 3rd v 4th
/// play the promotion play-off (the second place); 14th v 17th and 15th v 16th play the play-out (the losers go down
/// with the 18th [inference]); each is a cup stage with the league's own schedule
/// Original: `ItaSerC1a::create_playoff_stages` (0x64ba70, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64ba70 } (thiscall, ret 0, ita_ser_c1a.cpp, quadro SEH só para o operator new): `ItaSerC1a::create_playoff_stages()`.
/// A regra da C1/A: o campeão sobe direto (e vai à Supercopa da C1); do 2º ao 5º, um play-off de acesso (2º × 5º,
/// 3º × 4º) dá a segunda vaga; do 14º ao 17º, um play-out (14º × 17º, 15º × 16º) decide quem cai junto com o 18º
/// [inferência pelas faixas da cqk_6 e pelos nomes do mapa]. Fonte 0x9ea474, linhas 0x27c, 0x290, 0x2b3.
pub(crate) unsafe extern "thiscall" fn create_playoff_stages(this: usize) {
    create_end_of_league_stages(this, 0x9ea474, [0x27c, 0x290, 0x2b3], true)
}

/// the same rule in C1/B
/// Original: `ItaSerC1b::create_play_off_stages` (0x64fc30, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64fc30 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_play_off_stages()`, a mesma regra na C1/B.
/// Fonte 0x9ea558, linhas 0x281, 0x295, 0x2b8; no fim solta o gabarito antes da lista.
pub(crate) unsafe extern "thiscall" fn create_play_off_stages(this: usize) {
    create_end_of_league_stages(this, 0x9ea558, [0x281, 0x295, 0x2b8], false)
}

/// Comparer of the end-of-league stages: runs only the original and checks the Super Cup slot, both stages and +0x30 =
/// 1.
///
/// [pt] O comparador das duas: alocam e sorteiam: só a original. O plano, feito antes: o campeão e a vaga da Supercopa. Depois:
/// a vaga com o campeão, as duas fases em [+0xc] e +0x30 = 1.
pub(crate) unsafe fn compare_end_of_league_stages(nome: &str, this: usize, original: impl FnOnce()) {
    let copa = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_C1_SUPER_CUP) * 4);
    let inscritos = read_u32_as_usize(copa + 0xa7);
    let campeao = ita_ser_c1a_league_row_club(this, 0);
    let vaga = if read_u32_as_usize(inscritos) != 0 { inscritos + 6 } else { inscritos };
    original();
    let fases = read_u32_as_usize(this + 0xc);
    let ok = read_u32_as_usize(vaga) == campeao && fases != 0 && read_u32_as_usize(fases) != 0 && read_u32_as_usize(fases + 4) != 0 && read_u32_as_usize(this + 0x30) == 1;
    PLACAR.registrar(ok, || format!("{}({:#x}): vaga da Supercopa {:#x} (campeão {:#x}), fases {:#x}, +0x30 = {}", nome, this, read_u32_as_usize(vaga), campeao, fases, read_u32_as_usize(this + 0x30) as i32));
}



/// the Serie C1 schedule (each group of 18): 34 rounds from 5 September to 7 May, nearly all on the same weekday (6)
/// and two midweek; no TV slots. The promotion play-offs (stage 0): two-legged semi-finals on 21 and 28 May (4 clubs,
/// no draw) and the final on 4 and 11 June; the play-outs (stage 1) [inference: relegation]: 2 two-legged ties on the
/// semi-final dates, no draw
/// Original: `ItaSerC1a::create_schedule` (0x64ad80, ita_ser_c1a.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x64ad80 } (thiscall, ret 0x10, ita_ser_c1a.cpp, 2.279 bytes, método +0x3c): `ItaSerC1a::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a chave A da Série C1. Fonte 0x9ea474, linhas 0xab, 0x155, 0x18f.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_c1_schedule(this, fase, rodadas, extra, liga, &C1BracketVariant { arquivo: 0x9ea474, linhas: [0xab, 0x155, 0x18f] })
}


/// the club's place forecast in C1/A: in the league, best and worst place become bands (2 to 5 = the promotion
/// play-off: best 2, worst 5; 14 to 17 = the play-out: best 14, worst 17); in the play-off, places + 1; in the
/// play-out, the winner stays at 14 and the loser at 16
/// Original: `ItaSerC1a::set_club_progress` (0x64bd80, ita_ser_c1a.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x64bd80 } (thiscall, ret 0x14, ita_ser_c1a.cpp): `ItaSerC1a::set_club_progress(clube, fase, atual, melhor,
/// pior)`, a virtual +0x68 da 0x684cd0 a cada linha da tabela.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    set_c1_club_progress(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8)
}


/// the 2001/02 C1/A: the 306 matches (34 rounds of 9) fixed by club name; with a name missing from the database, the
/// list is dropped and the league draws its schedule
/// Original: `ItaSerC1a::set_up_fixed_fixtures` (0x64be90, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64be90 } (thiscall, ret 0, ita_ser_c1a.cpp, 11.650 bytes): `ItaSerC1a::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/A 2001/02 (34 rodadas de 9). Fonte 0x9ea474, linha 0x3fd.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_set_up_fixed_fixtures(this: usize) {
    set_up_fixed_fixtures_body(this, &C1A_FIXTURES, 0x9ea474, 0x3fd)
}


/// C1/A: discipline 0x11, the league and 2 stages (the promotion play-off and the play-out), the real 2001/02 fixture
/// list
/// Original: `ItaSerC1a::ItaSerC1a` (0x64aa70, ita_ser_c1a.cpp, thiscall, ret 8).
///
/// [pt] { 0x64aa70 } (thiscall, ret 8, ita_ser_c1a.cpp): `ItaSerC1a::ItaSerC1a(ano, registro)`: a C1/A, a liga e 2 fases
/// (o play-off de acesso e o play-out).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_c1a(this: usize, ano: u32, registro: usize) -> usize {
    construct_italian_division(this, ano, registro, &SERIE_C1A_DIVISION)
}


/// Serie C1/A (18): the champion goes up to B, 2nd to 5th go to the promotion play-off, 14th to 17th to the relegation
/// play-out and the last goes down; tie-breaks 1 and 2; the league below stored is C1/B (as in the original)
/// Original: `ItaSerC1a::set_up_rules` (0x64b770, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64b770 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::set_up_rules()`. A C1/A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 0 (+0xc7), o campeão sobe direto (+0xbe = 1), do 2º ao 5º vão ao play-off de acesso
/// (+0xbf = 4), do 14º ao 17º ao play-out de permanência (+0xc0 = 4), o último cai (+0xc1 = 1), a Série B acima
/// ([0x9cf574]) e abaixo o id de [0x9cf57c] (o da C1/B, o outro grupo: assim no original).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1a");
    ita_ser_a_apply_league_rules(this, &serie_c1a_rules(this))
}


/// Serie C1/A has 18 clubs; no prize fund and no TV money
/// Original: `ItaSerC1a::create_table` (0x64b670, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64b670 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::create_table()`: 18 clubes ([0x9cf578]), sem fundo
/// nem TV; devolve se encheu. Fonte 0x9ea474, linha 0x1bd.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1a_create_table(this: usize) -> u32 {
    ita_ser_a_fill_league_table(this, 0x12, 0x9ea474, 0x1bd, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A).unwrap_or(false) as u32
}

