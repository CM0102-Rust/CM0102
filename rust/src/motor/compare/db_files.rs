use super::*;

// The comparers of the routines in ../db_files.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../db_files.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `conversor_v1` (comparison mode: see the header of motor.rs).
pub(crate) unsafe fn comparar_conversor_v1(nome: &str, o: V1Converter, rust: V1Converter, stream: usize, use_zip: i32,
                                registros: usize, n: u32, tamanho: usize) -> i32 {
    if use_zip != 0 {
        return o(stream, use_zip, registros, n);
    }
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FTELL);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let pos = ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(registros, n as usize * tamanho)],
        || {
            let r = o(stream, use_zip, registros, n);
            fim_o = ftell(stream);
            r
        },
        || {
            fseek(stream, pos, 0);
            let r = rust(stream, use_zip, registros, n);
            fim_r = ftell(stream);
            r
        });
    PLACAR.registrar(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("{}(n {}) original={} rust={} arquivo em {} x {} (de {}), 1º byte diferente {:?}", nome, n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}

/// Comparer for `db_files_read_clubs_v1` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_clubs_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("db_files_read_clubs_v1", original_fn::<V1Converter>(IDX_DB_FILES_READ_CLUBS_V1), db_files_read_clubs_v1, stream, use_zip, registros, n, 581)
}

/// Comparer for `db_files_read_players_v1` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_players_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("db_files_read_players_v1", original_fn::<V1Converter>(IDX_DB_FILES_READ_PLAYERS_V1), db_files_read_players_v1, stream, use_zip, registros, n, 68)
}

/// Comparer for `db_files_read_non_players_v1` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_non_players_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("db_files_read_non_players_v1", original_fn::<V1Converter>(IDX_DB_FILES_READ_NON_PLAYERS_V1), db_files_read_non_players_v1, stream, use_zip, registros, n, 70)
}

/// Comparer for `db_files_write_staff_history` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_staff_history(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF_HISTORY)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_comps` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_comps(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_COMPS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_comp_history` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_comp_history(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_COMP_HISTORY)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_staff_comps` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_staff_comps(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF_COMPS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_officials` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_officials(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_OFFICIALS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_records_0x3a` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_records_0x3a(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_RECORDS_0X3A)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_or_read_records_0x3c` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_or_read_records_0x3c(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_OR_READ_RECORDS_0X3C)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_open_for_read` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: abrir duas vezes deixaria outro FILE aberto (ou alocaria um lugar novo na tabela de FILE da
/// CRT, que fica). Roda a original e compara só o caminho (sem efeito colateral).
pub(crate) unsafe extern "C" fn comparar_db_files_open_for_read(pasta: usize, nome: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_DB_FILES_OPEN_FOR_READ)(pasta, nome);
    check_database_path("db_files_open_for_read", pasta, nome);
    r
}

/// Comparer for `db_files_open_for_write` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: criar duas vezes truncaria o arquivo com dois FILE abertos. Roda a original e compara só o
/// caminho.
pub(crate) unsafe extern "C" fn comparar_db_files_open_for_write(pasta: usize, nome: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_DB_FILES_OPEN_FOR_WRITE)(pasta, nome);
    check_database_path("db_files_open_for_write", pasta, nome);
    r
}

/// Comparer for `db_files_write_continents` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_continents(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_CONTINENTS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_nations` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_nations(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_NATIONS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_cities` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_cities(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_CITIES)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_stadiums` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_stadiums(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STADIUMS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_clubs` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_clubs(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_CLUBS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_staff(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_players(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_PLAYERS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_non_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_non_players(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_NON_PLAYERS)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_write_staff_extra` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_write_staff_extra(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF_EXTRA)(stream, use_zip, registros, n)
}

/// Comparer for `db_files_read_officials` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_officials(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_OFFICIALS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_colours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_colours(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COLOURS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_names` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_names(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NAMES)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_clubs` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_clubs(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CLUBS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_comps` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_comps(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMPS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_players(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_PLAYERS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_non_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_non_players(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NON_PLAYERS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_staff(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_staff_extra` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_staff_extra(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_EXTRA)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_continents` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_continents(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CONTINENTS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_nations` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_nations(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NATIONS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_cities` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_cities(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CITIES)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_stadiums` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_stadiums(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STADIUMS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_staff_v1` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_staff_v1(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_V1)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_staff_history` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_staff_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_HISTORY)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_comp_history` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_comp_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMP_HISTORY)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_staff_comps` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_staff_comps(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMPS)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_staff_comp_history` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_staff_comp_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMP_HISTORY)(stream, use_zip, registros, entrada)
}

/// Comparer for `db_files_read_comps_v1` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_db_files_read_comps_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_READ_COMPS_V1);
    if use_zip != 0 {
        return o(stream, use_zip, registros, n);
    }
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FTELL);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let pos = ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(registros, n as usize * 107)],
        || {
            let r = o(stream, use_zip, registros, n);
            fim_o = ftell(stream);
            r
        },
        || {
            fseek(stream, pos, 0);
            let r = db_files_read_comps_v1(stream, use_zip, registros, n);
            fim_r = ftell(stream);
            r
        });
    PLACAR.registrar(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("db_files_read_comps_v1(n {}) original={} rust={} arquivo em {} x {} (de {}), 1º byte diferente {:?}", n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}
