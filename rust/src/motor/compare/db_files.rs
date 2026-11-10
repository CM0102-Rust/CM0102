use super::*;

// The comparers of the routines in ../db_files.rs.
// [pt] Os comparadores das rotinas de ../db_files.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `conversor_v1`.
pub(crate) unsafe fn compare_conversor_v1(name: &str, o: V1Converter, rust: V1Converter, stream: usize, use_zip: i32,
                                records: usize, n: u32, size: usize) -> i32 {
    if use_zip != 0 {
        return o(stream, use_zip, records, n);
    }
    let pos = exe::crt_ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side(&[(records, n as usize * size)],
        || {
            let r = o(stream, use_zip, records, n);
            fim_o = exe::crt_ftell(stream);
            r
        },
        || {
            exe::crt_fseek(stream, pos, (0) as u32);
            let r = rust(stream, use_zip, records, n);
            fim_r = exe::crt_ftell(stream);
            r
        });
    SCOREBOARD.record(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("{}(n {}) original={} rust={} file em {} x {} (de {}), 1º byte different {:?}", name, n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}

/// Comparer for `db_files_read_clubs_v1`.
pub(crate) unsafe extern "C" fn compare_db_files_read_clubs_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    compare_conversor_v1("db_files_read_clubs_v1", original_fn::<V1Converter>(IDX_DB_FILES_READ_CLUBS_V1), db_files_read_clubs_v1, stream, use_zip, records, n, 581)
}

/// Comparer for `db_files_read_players_v1`.
pub(crate) unsafe extern "C" fn compare_db_files_read_players_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    compare_conversor_v1("db_files_read_players_v1", original_fn::<V1Converter>(IDX_DB_FILES_READ_PLAYERS_V1), db_files_read_players_v1, stream, use_zip, records, n, 68)
}

/// Comparer for `db_files_read_non_players_v1`.
pub(crate) unsafe extern "C" fn compare_db_files_read_non_players_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    compare_conversor_v1("db_files_read_non_players_v1", original_fn::<V1Converter>(IDX_DB_FILES_READ_NON_PLAYERS_V1), db_files_read_non_players_v1, stream, use_zip, records, n, 70)
}

/// Comparer for `db_files_write_staff_history`.
pub(crate) unsafe extern "C" fn compare_db_files_write_staff_history(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF_HISTORY)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_comps`.
pub(crate) unsafe extern "C" fn compare_db_files_write_comps(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_COMPS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_comp_history`.
pub(crate) unsafe extern "C" fn compare_db_files_write_comp_history(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_COMP_HISTORY)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_staff_comps`.
pub(crate) unsafe extern "C" fn compare_db_files_write_staff_comps(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF_COMPS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_officials`.
pub(crate) unsafe extern "C" fn compare_db_files_write_officials(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_OFFICIALS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_records_0x3a`.
pub(crate) unsafe extern "C" fn compare_db_files_write_records_0x3a(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_RECORDS_0X3A)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_or_read_records_0x3c`.
pub(crate) unsafe extern "C" fn compare_db_files_write_or_read_records_0x3c(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_OR_READ_RECORDS_0X3C)(stream, use_zip, records, n)
}

/// Comparer for `db_files_open_for_read`.
///
/// [pt] Comparador: abrir duas vezes deixaria outro FILE aberto (ou alocaria um lugar novo na tabela de FILE da
/// CRT, que fica). Roda a original e compara só o caminho (sem efeito colateral).
pub(crate) unsafe extern "C" fn compare_db_files_open_for_read(folder: usize, name: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_DB_FILES_OPEN_FOR_READ)(folder, name);
    check_database_path("db_files_open_for_read", folder, name);
    r
}

/// Comparer for `db_files_open_for_write`.
///
/// [pt] Comparador: criar duas vezes truncaria o arquivo com dois FILE abertos. Roda a original e compara só o
/// caminho.
pub(crate) unsafe extern "C" fn compare_db_files_open_for_write(folder: usize, name: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_DB_FILES_OPEN_FOR_WRITE)(folder, name);
    check_database_path("db_files_open_for_write", folder, name);
    r
}

/// Comparer for `db_files_write_continents`.
pub(crate) unsafe extern "C" fn compare_db_files_write_continents(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_CONTINENTS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_nations`.
pub(crate) unsafe extern "C" fn compare_db_files_write_nations(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_NATIONS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_cities`.
pub(crate) unsafe extern "C" fn compare_db_files_write_cities(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_CITIES)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_stadiums`.
pub(crate) unsafe extern "C" fn compare_db_files_write_stadiums(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STADIUMS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_clubs`.
pub(crate) unsafe extern "C" fn compare_db_files_write_clubs(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_CLUBS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_staff`.
pub(crate) unsafe extern "C" fn compare_db_files_write_staff(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_players`.
pub(crate) unsafe extern "C" fn compare_db_files_write_players(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_PLAYERS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_non_players`.
pub(crate) unsafe extern "C" fn compare_db_files_write_non_players(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_NON_PLAYERS)(stream, use_zip, records, n)
}

/// Comparer for `db_files_write_staff_extra`.
pub(crate) unsafe extern "C" fn compare_db_files_write_staff_extra(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_WRITE_STAFF_EXTRA)(stream, use_zip, records, n)
}

/// Comparer for `db_files_read_officials`.
pub(crate) unsafe extern "C" fn compare_db_files_read_officials(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_OFFICIALS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_colours`.
pub(crate) unsafe extern "C" fn compare_db_files_read_colours(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COLOURS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_names`.
pub(crate) unsafe extern "C" fn compare_db_files_read_names(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NAMES)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_clubs`.
pub(crate) unsafe extern "C" fn compare_db_files_read_clubs(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CLUBS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_comps`.
pub(crate) unsafe extern "C" fn compare_db_files_read_comps(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMPS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_players`.
pub(crate) unsafe extern "C" fn compare_db_files_read_players(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_PLAYERS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_non_players`.
pub(crate) unsafe extern "C" fn compare_db_files_read_non_players(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NON_PLAYERS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_staff`.
pub(crate) unsafe extern "C" fn compare_db_files_read_staff(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_staff_extra`.
pub(crate) unsafe extern "C" fn compare_db_files_read_staff_extra(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_EXTRA)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_continents`.
pub(crate) unsafe extern "C" fn compare_db_files_read_continents(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CONTINENTS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_nations`.
pub(crate) unsafe extern "C" fn compare_db_files_read_nations(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NATIONS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_cities`.
pub(crate) unsafe extern "C" fn compare_db_files_read_cities(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CITIES)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_stadiums`.
pub(crate) unsafe extern "C" fn compare_db_files_read_stadiums(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STADIUMS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_staff_v1`.
pub(crate) unsafe extern "C" fn compare_db_files_read_staff_v1(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_V1)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_staff_history`.
pub(crate) unsafe extern "C" fn compare_db_files_read_staff_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_HISTORY)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_comp_history`.
pub(crate) unsafe extern "C" fn compare_db_files_read_comp_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMP_HISTORY)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_staff_comps`.
pub(crate) unsafe extern "C" fn compare_db_files_read_staff_comps(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMPS)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_staff_comp_history`.
pub(crate) unsafe extern "C" fn compare_db_files_read_staff_comp_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMP_HISTORY)(stream, use_zip, records, entry)
}

/// Comparer for `db_files_read_comps_v1`.
pub(crate) unsafe extern "C" fn compare_db_files_read_comps_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(IDX_DB_FILES_READ_COMPS_V1);
    if use_zip != 0 {
        return o(stream, use_zip, records, n);
    }
    let pos = exe::crt_ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side(&[(records, n as usize * 107)],
        || {
            let r = o(stream, use_zip, records, n);
            fim_o = exe::crt_ftell(stream);
            r
        },
        || {
            exe::crt_fseek(stream, pos, (0) as u32);
            let r = db_files_read_comps_v1(stream, use_zip, records, n);
            fim_r = exe::crt_ftell(stream);
            r
        });
    SCOREBOARD.record(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("db_files_read_comps_v1(n {}) original={} rust={} file em {} x {} (de {}), 1º byte different {:?}", n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}
