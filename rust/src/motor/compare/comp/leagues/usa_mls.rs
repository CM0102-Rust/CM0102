use super::*;

// The comparers of the routines in ../comp/leagues/usa_mls.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/usa_mls.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `usa_mls_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    let esperado = if ultima { antes } else if antes == 1 || antes == 2 { antes + 1 } else { antes };
    PLACAR.registrar(depois == esperado, || format!("usa_mls_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// Comparer for `usa_mls_create_playoff_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela cópia da escolha (os mesmos registros e o mesmo qsort): a fase da terceira vaga existe, com +0x3e = 8, +4
/// = o +4 da liga e os oito na lista dela (+0xa7, 6 bytes; como conjunto).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_create_playoff_stage(this: usize) {
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let (a, b) = (read_usize(read_usize(this + 0xc)), read_usize(read_usize(this + 0xc) + 4));
    let mut plano = [0usize; 8];
    let mut r = [0u8; 54];
    put_club_points_record(&mut r, 0, table_row_club_points(this, 0));
    put_club_points_record(&mut r, 1, table_row_club_points(a, 0));
    put_club_points_record(&mut r, 2, table_row_club_points(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    for k in 0..3 {
        plano[k] = club_of_record(&r, k);
    }
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            put_club_points_record(&mut r, k, table_row_club_points(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    for k in 0..5 {
        plano[3 + k] = club_of_record(&r, k);
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_CREATE_PLAYOFF_STAGE)(this);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    let igual = fase != 0 && read_i16(fase + 0x3e) == 8 && read_usize(fase + 4) == read_usize(this + 4) && {
        let mut nela = [0usize; 8];
        for (i, c) in nela.iter_mut().enumerate() {
            *c = read_usize(read_usize(fase + 0xa7) + 6 * i);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    PLACAR.registrar(igual, || format!("usa_mls_create_playoff_stage({:#x}) fase={:#x} plano={:x?}", this, fase, plano));
}

/// Comparer for `usa_mls_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os dois primeiros vencedores do play-off. Depois, a fase da quarta vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (como conjunto).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_create_final_stage(this: usize) {
    let po = read_usize(read_usize(this + 0xc) + 8);
    let mut v = [0usize; 2];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < read_i16(po + 0x3e) as i32 && n < 2 {
        let e = read_usize(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_CREATE_FINAL_STAGE)(this);
    let fase = read_usize(read_usize(this + 0xc) + 0xc);
    // simplification: com menos de dois vencedores a original lê vagas da lista sem valor; aí só a fase é conferida.
    let igual = fase != 0 && n < 2 || fase != 0 && {
        let l = read_usize(fase + 0xa7);
        let (x, y) = (read_usize(l), read_usize(l + 6));
        read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (v[0], v[1]) || (x, y) == (v[1], v[0]))
    };
    PLACAR.registrar(igual, || format!("usa_mls_create_final_stage({:#x}) finalistas={:x?} n={} fase={:#x}", this, v, n, fase));
}

/// Comparer for `usa_mls_create_table` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da tabela: só a original (simplification: aloca a tabela e as listas das linhas), com o plano feito antes:
/// os clubes da competição com o código 0x413, na ordem do banco, até 4. Depois, a devolução, +0x3e = 4 e as linhas.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_create_table(this: usize) -> u32 {
    let mut plano = [0usize; 4];
    let mut n = 0usize;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 4 {
        let c = base + i as u16 as usize * club::SIZE;
        if is_in_mls_division(this, c, 0x413) {
            plano[n] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_CREATE_TABLE)(this);
    let t = read_usize(this + 0xb1);
    let igual = r == (n == 4) as u32 && read_i16(this + 0x3e) == 4 && t != 0 && (0..n).all(|k| read_usize(t + k * 0x3b) == plano[k]);
    PLACAR.registrar(igual, || format!("usa_mls_create_table({:#x}) r={} n={} plano={:x?}", this, r, n, plano));
    r
}

/// Comparer for `create_second_division_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_second_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_SECOND_DIVISION_STAGE);
    compare_division_stage("create_second_division_stage", this, 0x47b, 0, || o(this))
}

/// Comparer for `create_third_division_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_third_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_THIRD_DIVISION_STAGE);
    compare_division_stage("create_third_division_stage", this, 0x414, 1, || o(this))
}

/// Comparer for `usa_mls_start_new_season` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da virada: só a original (simplification: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1 e as três divisões montadas
/// (+0x3e = 4 e as vagas 0 e 1 das fases).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_start_new_season(this: usize) -> u32 {
    let antes = read_i16(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_START_NEW_SEASON)(this);
    let depois = read_i16(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && read_i16(this + 0x3e) == 4 && read_usize(read_usize(this + 0xc)) != 0 && read_usize(read_usize(this + 0xc) + 4) != 0;
    PLACAR.registrar(igual, || format!("usa_mls_start_new_season({:#x}) ano antes={} depois={} +0x3e={}", this, antes, depois, read_i16(this + 0x3e)));
    r
}

/// Comparer for `usa_mls_set_predicted_positions` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador das posições previstas: só a original (simplification: a 0x4a2540 acrescenta o registro de um clube que não
/// tem; não pode rodar duas vezes), com o plano feito antes: os doze clubes das três tabelas, cada bloco ordenado pela
/// mesma qsort numa cópia; depois, a posição (+0x72 do registro achado por 0x4a4850) de cada um.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_SET_PREDICTED_POSITIONS);
    let n = read_i16(this + 0x3e) as usize;
    if read_usize(this + 8) == 0 || n != 4 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut v = [0usize; 12];
    for k in 0..4 {
        v[k] = aus_nsl_table_row_club(this, k);
        v[4 + k] = aus_nsl_table_row_club(read_usize(read_usize(this + 0xc)), k);
        v[8 + k] = aus_nsl_table_row_club(read_usize(read_usize(this + 0xc) + 4), k);
    }
    for b in 0..3 {
        ordenar(v[4 * b..].as_mut_ptr(), 4, 4, 0x4b07d0);
    }
    o(this);
    const LUGARES: [u8; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10];
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(read_usize(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) != LUGARES[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("usa_mls_set_predicted_positions({:#x}): a posição da vaga {:?} não bate ({:x?})", this, errado, v));
}

/// Comparer for `usa_mls_get_club_array` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da lista: só a original (aloca a lista, que fica com quem chamou), com o plano: n = 12 e os doze clubes
/// das três tabelas na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_get_club_array(this: usize, lista: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_USA_MLS_GET_CLUB_ARRAY)(this, lista, n);
    let m = read_i16(this + 0x3e) as usize;
    let igual = *n == 0xc && *lista != 0 && (0..m).all(|i| {
        read_usize(*lista + 4 * i) == aus_nsl_table_row_club(this, i)
            && read_usize(*lista + 4 * (m + i)) == aus_nsl_table_row_club(read_usize(read_usize(this + 0xc)), i)
            && read_usize(*lista + 4 * (2 * m + i)) == aus_nsl_table_row_club(read_usize(read_usize(this + 0xc) + 4), i)
    });
    PLACAR.registrar(igual, || format!("usa_mls_get_club_array({:#x}) n={}", this, *n));
}

/// Comparer for `usa_mls_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com a
/// devolução prevista pelo ensaio.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let n = mls_stage_outcome(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_USA_MLS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, situacao, fase, a4, jogo, a6);
    PLACAR.registrar(r == n, || format!("usa_mls_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}

/// Comparer for `usa_mls_add_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só a original (simplification: o resultado entra nas tabelas; não roda duas vezes), com o plano: a devolução
/// 1, a marca +0x42 do jogo (1 no estágio 1; senão -1 se estava abaixo de 2) e o bloco +8 devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_add_result(this: usize, jogo: usize, estat: u32) -> u32 {
    let marca = *((jogo + 0x42) as *const i8);
    let bloco = read_u32_as_usize(this + 8);
    let esperada = if read_u32_as_usize(this + 0x30) == 1 { 1 } else if marca < 2 { -1 } else { marca };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_USA_MLS_ADD_RESULT)(this, jogo, estat);
    let (m, b) = (*((jogo + 0x42) as *const i8), read_u32_as_usize(this + 8));
    PLACAR.registrar(r == 1 && m == esperada && b == bloco, || format!("usa_mls_add_result({:x}, {:x}) devolveu {} marca {} x {} bloco {:x} x {:x}", this, jogo, r, m, esperada, b, bloco));
    r
}

/// Comparer for `usa_mls_get_club_all_star_conference` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só lê; o caso de erro fica com a original sozinha).
pub(crate) unsafe extern "C" fn comparar_usa_mls_get_club_all_star_conference(clube: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_ALL_STAR_CONFERENCE)(clube);
    let nacao = read_u32_as_usize(clube + club::DIVISION);
    if nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && r == 0 {
        return r; // o caso de erro (a caixa já saiu na original)
    }
    let n = usa_mls_get_club_all_star_conference(clube);
    PLACAR.registrar(r == n, || format!("usa_mls_get_club_all_star_conference({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// Comparer for `usa_mls_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado sobre o objeto da liga (0xee; só grava nele); a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_set_up_rules(this: usize) -> u32 {
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xee)], || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_SET_UP_RULES)(this), || usa_mls_set_up_rules(this));
    PLACAR.registrar(igual && r == n, || format!("usa_mls_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparador: como o do Mineiro.
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_USA_MLS_SET_CLUB_PROGRESS)(this, clube, fase, a, b, c), || usa_mls_set_club_progress(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("usa_mls_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}
