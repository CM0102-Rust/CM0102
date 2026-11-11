use super::*;

/// A version-1 database converter (Db_files.cpp): for each of the `n` records of `tamanho` bytes, the fields in file
/// order, `(deslocamento, tamanho, quantos)`, each read on its own (`tamanho`, 1) by `zread` `zipdir_zread` (use_zip ≠ 0) or
/// `fread` `exe::crt_fread`.
///
/// Size 0 = one byte from the file that becomes a u16 (`reputacao`) at offset + 2k (the original reads the byte into an
/// argument on its own stack, already used, and writes the u16 before the next read).
///
/// The first read that does not come back gives 0 (what has already been read stays in the record); all of them, 1. The
/// bytes the file does not provide (holes in the record) stay as they were.
pub(crate) unsafe fn convert_v1_records(stream: usize, use_zip: i32, records: usize, n: u32, size: usize,
                       fields: &[(usize, u32, usize)], injury_word_0c: fn(u8) -> u16) -> i32 {
    let read: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = records + i * size;
        for &(o, t, q) in fields {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if read(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o + 2 * k) as *mut u16).write_unaligned(injury_word_0c(b));
                } else if read(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

/// Bytes of one version-1 record in the file, from its field list.
pub(crate) fn v1_record_size(fields: &[(usize, u32, usize)]) -> usize {
    fields.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum()
}

/// TClub (581 bytes) no club.dat versão 1 (577 bytes no arquivo): buracos em +0x37, +0x52 e +0x244; a
/// reputação vem num byte (0 vira 1) × 500 em +0x80.
pub(crate) const CLUB_V1_LAYOUT: [(usize, u32, usize); 18] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 3), (0x5f, 1, 1),
    (0x60, 4, 1), (0x64, 1, 1), (0x65, 4, 2), (0x6d, 1, 1), (0x6e, 4, 1), (0x72, 1, 1), (0x73, 4, 3), (0x7f, 1, 1),
    (0x80, 0, 1), (0x82, 1, 1), (0x83, 4, 87), (0x1df, 1, 1), (0x1e0, 4, 25)];

/// Não jogador (68 bytes, id 9) versão 1 (65 no arquivo): as três reputações (+8, +0xa, +0xc) vêm em bytes.
pub(crate) const NON_PLAYER_V1_LAYOUT: [(usize, u32, usize); 6] = [(0, 4, 1), (4, 2, 2), (8, 0, 3), (0xe, 1, 21), (0x23, 4, 8), (0x43, 1, 1)];

/// Jogador (70 bytes, id 10) versão 1 (67 no arquivo): as três reputações (+9, +0xb, +0xd) vêm em bytes.
pub(crate) const PLAYER_V1_FIELDS: [(usize, u32, usize); 5] = [(0, 4, 1), (4, 1, 1), (5, 2, 2), (9, 0, 3), (0xf, 1, 55)];

/// reads version 1 club.dat field by field (577 bytes on file for 581-byte records, holes at +0x37, +0x52 and +0x244),
/// the reputation in one byte (0 becomes 1) × 500; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_clubs_v1` (0x54dc90, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_clubs_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database read clubs v1");
    convert_v1_records(stream, use_zip, records, n, 581, &CLUB_V1_LAYOUT, |b| (b.max(1) as u16).wrapping_mul(500))
}

/// reads the version 1 non-player table (id 9; the community map calls it players) field by field (65 bytes on file for
/// 68-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_players_v1` (0x54e9a0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_players_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database read non-players v1");
    convert_v1_records(stream, use_zip, records, n, 68, &NON_PLAYER_V1_LAYOUT, |b| b as u16)
}

/// reads the version 1 player table (id 10; the community map calls it non_players) field by field (67 bytes on file
/// for 70-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_non_players_v1` (0x54f150, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_non_players_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database read players v1");
    convert_v1_records(stream, use_zip, records, n, 70, &PLAYER_V1_FIELDS, |b| b as u16)
}

pub(crate) type V1Converter = unsafe extern "C" fn(usize, i32, usize, u32) -> i32;

/// Comparer of a converter.
///
/// With `fread` (use_zip = 0), the original reads, the file is rewound (`fseek` `exe::crt_fseek` to the position from `ftell`
/// `exe::crt_ftell`) and the Rust one reads again; it compares the records, the answer and where the file stopped. The Rust
/// one's position is kept (the same, if equal).
// simplification: with zread (the container) original only; the save is version 2 and does not get here.

/// Self-test of the version 1 converters (`CM_AUTOTESTE=1`, comparison mode: the factory database and the save are
/// version 2 and the career does not go through them).
///
/// For each converter, 64 cases with fixed seeds: from 1 to 6 records, a temporary file (`autoteste_banco_v1.tmp` in
/// the game folder, deleted at the end) with random bytes (1 case in 4 all zeros: the club's reputation 0), whole (1 in
/// 3) or cut at a random point (the read fails in the middle of a field), opened by the game's `fopen` `0x945b00`; the
/// buffer starts random (the record's holes are included in the comparison).
pub unsafe fn autoteste_dbj_1() {
    const ARQ: &str = "autoteste_banco_v1.tmp";
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let (t0, d0) = SCOREBOARD.counts();
    let mut s = TestRng(0x5631_4442_4a31);
    let casos: [(V1Converter, &[(usize, u32, usize)], usize); 3] = [(compare_db_files_read_clubs_v1, &CLUB_V1_LAYOUT, 581),
        (compare_db_files_read_players_v1, &NON_PLAYER_V1_LAYOUT, 68), (compare_db_files_read_non_players_v1, &PLAYER_V1_FIELDS, 70)];
    for (comparar, campos, tamanho) in casos {
        for caso in 0..64u64 {
            let n = 1 + (s.next_u64() % 6) as usize;
            let total = n * v1_record_size(campos);
            let corte = if caso % 3 == 0 { total } else { (s.next_u64() % (total as u64 + 1)) as usize };
            let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.next_u64() as u8 }).collect();
            if std::fs::write(ARQ, &bytes).is_err() {
                continue;
            }
            let f = fopen(b"autoteste_banco_v1.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
            if f == 0 {
                continue;
            }
            let mut buf: Vec<u8> = (0..n * tamanho).map(|_| s.next_u64() as u8).collect();
            comparar(f, 0, buf.as_mut_ptr() as usize, n as u32);
            fclose(f);
        }
    }
    let _ = std::fs::remove_file(ARQ);
    let (t1, d1) = SCOREBOARD.counts();
    linha(&format!("database v1 (converters): SELFTEST {} cases, {} divergences", t1 - t0, d1 - d0));
}

/// Writes n 17-byte records (staff_history.dat, id 17) in one go, through zwrite (save) or fwrite (Data folder); 1 if
/// everything was written.
///
/// `db_files_write_staff_history(stream, use_zip, records, n)`: writes `n` 17-byte records (staff_history.dat, id 17)
/// in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save) or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything
/// was written. No stack frame: nothing from the stack goes into the file.
/// Original: `db_files_write_staff_history` (0x54fe50, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_staff_history(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write people history");
    let written = if use_zip != 0 {
        exe::zwrite(records, 17, n, stream)
    } else {
        exe::crt_fwrite(records, 17, n, stream)
    };
    (written == n) as i32 // write people’s history
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// writes n 107-byte records (club_comp.dat, in two calls: the competitions and the 127 reserve ones; nation_comp.dat)
/// in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went
/// Original: `db_files_write_comps` (0x5501d0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_comps(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write competitions");
    let written = if use_zip != 0 {
        exe::zwrite(records, 107, n, stream)
    } else {
        exe::crt_fwrite(records, 107, n, stream)
    };
    (written == n) as i32 // save competitions
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// Writes n 26-byte records (club_comp_history.dat and nation_comp_history.dat, ids 19 and 20) in one go, through
/// zwrite (save) or fwrite (Data folder); 1 if everything was written.
///
/// `db_files_write_comp_history(stream, use_zip, records, n)`: writes `n` 26-byte records (club_comp_history.dat and
/// nation_comp_history.dat, ids 19 and 20) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save) or `fwrite`
/// `exe::crt_fwrite` (the Data folder); 1 if everything was written. No stack frame: nothing from the stack goes into the file.
/// Original: `db_files_write_comp_history` (0x550320, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_comp_history(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write competition histories");
    let written = if use_zip != 0 {
        exe::zwrite(records, 26, n, stream)
    } else {
        exe::crt_fwrite(records, 26, n, stream)
    };
    (written == n) as i32 // write competition histories
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// Writes n 101-byte records (staff_comp.dat, id 11) in one go, through zwrite (save) or fwrite (Data folder); 1 if
/// everything was written.
///
/// `db_files_write_staff_comps(stream, use_zip, records, n)`: writes `n` 101-byte records (staff_comp.dat, id 11) in
/// one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save) or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was
/// written. No stack frame: nothing from the stack goes into the file.
/// Original: `db_files_write_staff_comps` (0x550470, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_staff_comps(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write staff competitions");
    let written = if use_zip != 0 {
        exe::zwrite(records, 101, n, stream)
    } else {
        exe::crt_fwrite(records, 101, n, stream)
    };
    (written == n) as i32 // save staff competitions
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// Writes n 43-byte records (officials.dat, id 7) in one go, through zwrite (save) or fwrite (Data folder); 1 if
/// everything was written.
///
/// `db_files_write_officials(stream, use_zip, records, n)`: writes `n` 43-byte records (officials.dat, id 7) in one go
/// through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save) or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was
/// written. No stack frame: nothing from the stack goes into the file.
/// Original: `db_files_write_officials` (0x5506b0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_officials(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write referees");
    let written = if use_zip != 0 {
        exe::zwrite(records, 43, n, stream)
    } else {
        exe::crt_fwrite(records, 43, n, stream)
    };
    (written == n) as i32 // save referees
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// Writes n 58-byte records (colour.dat, id 2, and staff_comp_history.dat, id 18: the same routine for both) in one go,
/// through zwrite (save) or fwrite (Data folder); 1 if everything was written.
///
/// `db_files_write_records_0x3a(stream, use_zip, records, n)`: writes `n` 58-byte records (colour.dat, id 2, and
/// staff_comp_history.dat, id 18: the same routine for both) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the
/// save) or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was written. No stack frame: nothing from the stack
/// goes into the file.
/// Original: `db_files_write_records_0x3a` (0x550800, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_records_0x3a(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write 58-byte records");
    let written = if use_zip != 0 {
        exe::zwrite(records, 58, n, stream)
    } else {
        exe::crt_fwrite(records, 58, n, stream)
    };
    (written == n) as i32 // save 58-byte records
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// writes n 60-byte records (first_names.dat, second_names.dat and common_names.dat, ids 13 to 15) in one go, through
/// zwrite (save) or fwrite (Data folder); 1 if everything went (it only writes, despite the community name)
/// Original: `db_files_write_or_read_records_0x3c` (0x550950, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_or_read_records_0x3c(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write names");
    let written = if use_zip != 0 {
        exe::zwrite(records, 60, n, stream)
    } else {
        exe::crt_fwrite(records, 60, n, stream)
    };
    (written == n) as i32 // gravar nomes
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: original only; the proof is the .qst (saving the game) and the copy’s branch in Rust mode.

/// Builds a .dat path with _makepath(dest, 0, folder, name, 0) (FN_CRT_MAKEPATH).
pub(crate) unsafe fn database_path(target: usize, folder: usize, name: usize) {
    exe::crt_makepath((target) as *mut u8, 0, folder, name, 0);
}

/// Compares the path the original built at 0xde1f64 with the copy's, rebuilt in a local buffer.
pub(crate) unsafe fn check_database_path(routine: &str, folder: usize, name: usize) {
    let mut c = [0u8; 0x400];
    database_path(c.as_mut_ptr() as usize, folder, name);
    let o = core::slice::from_raw_parts(0xde1f64 as *const u8, 0x400);
    let end = |s: &[u8]| s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let (a, b) = (&o[..end(o)], &c[..end(&c)]);
    SCOREBOARD.record(a == b, || format!("{} path {:?} x {:?}", routine, String::from_utf8_lossy(a), String::from_utf8_lossy(b)));
}

/// Builds folder + name at 0xde1f64 (_makepath) and returns exe::crt_fopen(path, "rb"); the comparer checks the path.
///
/// `db_files_open_for_read(folder, name)`: builds the path at 0xde1f64 (`_makepath`, see `database_path`) and returns
/// `exe::crt_fopen(0xde1f64, "rb")` (`exe::crt_fopen`; "rb" at 0x98f928). When the game is created, `load_database` opens index.dat and
/// each table of the Data folder this way.
/// Original: `db_files_open_for_read` (0x54d580, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_open_for_read(folder: usize, name: usize) -> usize {
    covered!("database open .dat");
    database_path(0xde1f64, folder, name); // abrir (rb)
    exe::crt_fopen(0xde1f64, 0x98f928)
}

/// builds folder + name at 0xde1f64 (_makepath) and returns exe::crt_fopen(path, "wb"); only when writing the database to the
/// folder; the comparator checks the path
/// Original: `db_files_open_for_write` (0x54d5b0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_open_for_write(folder: usize, name: usize) -> usize {
    covered!("database create .dat");
    database_path(0xde1f64, folder, name);
    exe::crt_fopen(0xde1f64, 0x9a7f28) // criar (wb)
}

/// Writes the continent.dat table (id 3) of 198-byte records in one go, through zwrite (save) or fwrite (Data); 1 if
/// everything was written.
///
/// `db_files_write_continents(stream, use_zip, records, count)`: writes `count` 198-byte records (continent.dat, id 3)
/// in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address) or `fwrite` `exe::crt_fwrite`
/// (the Data folder); 1 if everything was written. No frame, no SEH, no box. Only save_database `exe::save_database` calls it (with
/// the pointers already replaced by indices).
/// Original: `db_files_write_continents` (0x54d6e0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_continents(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write continents");
    let written = if use_zip != 0 {
        exe::zwrite(records, 198, n, stream)
    } else {
        exe::crt_fwrite(records, 198, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the nation.dat table (id 4) of 290-byte records in one go, through zwrite (save) or fwrite (Data); 1 if
/// everything was written.
///
/// `db_files_write_nations(stream, use_zip, records, count)`: writes `count` 290-byte records (nation.dat, id 4) in one
/// go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address) or `fwrite` `exe::crt_fwrite` (the
/// Data folder); 1 if everything was written. No frame, no SEH, no box. Only save_database `exe::save_database` calls it (with the
/// pointers already replaced by indices).
/// Original: `db_files_write_nations` (0x54d840, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_nations(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write nations");
    let written = if use_zip != 0 {
        exe::zwrite(records, 290, n, stream)
    } else {
        exe::crt_fwrite(records, 290, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the city.dat table (id 21) of 56-byte records in one go, through zwrite (save) or fwrite (Data); 1 if
/// everything was written.
///
/// `db_files_write_cities(stream, use_zip, records, count)`: writes `count` 56-byte records (city.dat, id 21) in one go
/// through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address) or `fwrite` `exe::crt_fwrite` (the Data
/// folder); 1 if everything was written. No frame, no SEH, no box. Only save_database `exe::save_database` calls it (with the
/// pointers already replaced by indices).
/// Original: `db_files_write_cities` (0x54d9a0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_cities(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write cities");
    let written = if use_zip != 0 {
        exe::zwrite(records, 56, n, stream)
    } else {
        exe::crt_fwrite(records, 56, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the stadium.dat table of 78-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything
/// was written.
///
/// `db_files_write_stadiums(stream, use_zip, records, count)`: writes `count` 78-byte records (stadium.dat) in one go
/// through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address) or `fwrite` `exe::crt_fwrite` (the Data
/// folder); 1 if everything was written. No frame, no SEH, no box. Only save_database `exe::save_database` calls it (with the
/// pointers already replaced by indices).
/// Original: `db_files_write_stadiums` (0x54daf0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_stadiums(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write stadiums");
    let written = if use_zip != 0 {
        exe::zwrite(records, 78, n, stream)
    } else {
        exe::crt_fwrite(records, 78, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the club.dat and nat_club.dat table (ids 0, 1) of 581-byte records in one go, through zwrite (save) or fwrite
/// (Data); 1 if everything was written.
///
/// `db_files_write_clubs(stream, use_zip, records, count)`: writes `count` 581-byte records (club.dat and nat_club.dat,
/// ids 0, 1) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address) or
/// `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was written. No frame, no SEH, no box. Only save_database
/// `exe::save_database` calls it (with the pointers already replaced by indices).
/// Original: `db_files_write_clubs` (0x54e5d0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_clubs(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write clubs");
    let written = if use_zip != 0 {
        exe::zwrite(records, 581, n, stream)
    } else {
        exe::crt_fwrite(records, 581, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the staff.dat table (staff, 110 bytes; id 6) of 110-byte records in one go, through zwrite (save) or fwrite
/// (Data); 1 if everything was written.
///
/// `db_files_write_staff(stream, use_zip, records, count)`: writes `count` 110-byte records (staff.dat: persons, 110
/// bytes; id 6) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address) or
/// `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was written. No frame, no SEH, no box. Only save_database
/// `exe::save_database` calls it (with the pointers already replaced by indices).
/// Original: `db_files_write_staff` (0x54e720, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_staff(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write people 110");
    let written = if use_zip != 0 {
        exe::zwrite(records, 110, n, stream)
    } else {
        exe::crt_fwrite(records, 110, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the staff.dat non-player table (id 9; the community map calls it players) of 68-byte records in one go,
/// through zwrite (save) or fwrite (Data); 1 if everything was written.
///
/// `db_files_write_players(stream, use_zip, records, count)`: writes `count` 68-byte records (of the staff.dat
/// non-players, id 9; the community map calls it players) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save;
/// already in Rust, called by address) or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was written. No frame,
/// no SEH, no box. Only save_database `exe::save_database` calls it (with the pointers already replaced by indices).
/// Original: `db_files_write_players` (0x54efd0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_players(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write non-players");
    let written = if use_zip != 0 {
        exe::zwrite(records, 68, n, stream)
    } else {
        exe::crt_fwrite(records, 68, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the staff.dat player table (id 10; the community map calls it non_players) of 70-byte records in one go,
/// through zwrite (save) or fwrite (Data); 1 if everything was written.
///
/// `db_files_write_non_players(stream, use_zip, records, count)`: writes `count` 70-byte records (of the staff.dat
/// players, id 10; the community map calls it non_players) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save;
/// already in Rust, called by address) or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was written. No frame,
/// no SEH, no box. Only save_database `exe::save_database` calls it (with the pointers already replaced by indices).
/// Original: `db_files_write_non_players` (0x54fbb0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_non_players(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write players");
    let written = if use_zip != 0 {
        exe::zwrite(records, 70, n, stream)
    } else {
        exe::crt_fwrite(records, 70, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Writes the staff preferences table (id 22) of 52-byte records in one go, through zwrite (save) or fwrite (Data); 1
/// if everything was written.
///
/// `db_files_write_staff_extra(stream, use_zip, records, count)`: writes `count` 52-byte records (of the persons'
/// preferences, id 22) in one go through `zwrite` `zipdir_zwrite` (use_zip ≠ 0: the save; already in Rust, called by address)
/// or `fwrite` `exe::crt_fwrite` (the Data folder); 1 if everything was written. No frame, no SEH, no box. Only save_database
/// `exe::save_database` calls it (with the pointers already replaced by indices).
/// Original: `db_files_write_staff_extra` (0x54fd00, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_write_staff_extra(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database write preferences");
    let written = if use_zip != 0 {
        exe::zwrite(records, 52, n, stream)
    } else {
        exe::crt_fwrite(records, 52, n, stream)
    };
    (written == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: original only; the proof is the .qst (the written table) and the copy’s branch in Rust mode.

/// Reads the officials.dat table (id 7) of 43-byte records in one go, through zread (save) or fread (Data), with entry
/// version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_officials(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads
/// in one go the count (+0x37) of 43-byte records (officials.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or
/// `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and
/// the game exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath):
/// nothing from the stack goes into the table.
/// Original: `db_files_read_officials` (0x5505c0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_officials(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_OFFICIALS)(stream, use_zip, records, entry);
    }
    covered!("database read referees");
    let read = if use_zip != 0 {
        exe::zread(records, 43, n, stream)
    } else {
        exe::crt_fread(records, 43, n, stream)
    };
    (read == n) as i32 // referees
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the colour.dat table (id 2) of 58-byte records in one go, through zread (save) or fread (Data), with entry
/// version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_colours(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads in
/// one go the count (+0x37) of 58-byte records (colour.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or `fread`
/// `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and the game
/// exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath): nothing from
/// the stack goes into the table.
/// Original: `db_files_read_colours` (0x550710, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_colours(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COLOURS)(stream, use_zip, records, entry);
    }
    covered!("database read colours");
    let read = if use_zip != 0 {
        exe::zread(records, 58, n, stream)
    } else {
        exe::crt_fread(records, 58, n, stream)
    };
    (read == n) as i32 // cores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the first_names.dat, second_names.dat and common_names.dat tables (ids 13, 14, 15) of 60-byte records in one
/// go, through zread (save) or fread (Data), with entry version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_names(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads in
/// one go the count (+0x37) of 60-byte records (first_names.dat, second_names.dat and common_names.dat: the same
/// routine for the three) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if
/// everything was read. Another version: the "Error" box (Db_files.cpp) and the game exits; the copy calls the
/// original. The original's 0x200 frame only serves the box path (_splitpath): nothing from the stack goes into the
/// table.
/// Original: `db_files_read_names` (0x550860, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_names(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NAMES)(stream, use_zip, records, entry);
    }
    covered!("database read names");
    let read = if use_zip != 0 {
        exe::zread(records, 60, n, stream)
    } else {
        exe::crt_fread(records, 60, n, stream)
    };
    (read == n) as i32 // nomes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the club.dat and nat_club.dat table (ids 0, 1) of 581-byte records in one go, through zread (save) or fread
/// (Data), with entry version 2, and turns reputation 0 into 1; version 1: the `db_files_read_clubs_v1` converter (other: box and
/// exit); 1 if everything was read.
///
/// `db_files_read_clubs(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 2, reads in
/// one go the count (+0x37) of 581-byte records (club.dat and nat_club.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the
/// save) or `fread` `exe::crt_fread` (the Data folder); if not everything was read, 0; if it was, replaces reputation (+0x80,
/// u16) 0 with 1 in all of them and returns 1. Version 1: the field-by-field converter `db_files_read_clubs_v1` (stream, use_zip,
/// records, count), called by address, and its answer (without the reputation replacement: the converter does its own,
/// ×500). Another version: the "Error" box (Db_files.cpp) and the game exits; the copy calls the original. The
/// original's 0x200 frame only serves the box path.
/// Original: `db_files_read_clubs` (0x54db50, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_clubs(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version == 1 {
        covered!("database read clubs version 1");
        return exe::db_files_read_clubs_v1(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CLUBS)(stream, use_zip, records, entry);
    }
    covered!("database read clubs");
    let read = if use_zip != 0 {
        exe::zread(records, 581, n, stream)
    } else {
        exe::crt_fread(records, 581, n, stream)
    };
    if read != n {
        return 0;
    }
    for i in 0..n as usize {
        let p = (records + i * club::SIZE + club::REPUTATION) as *mut u16;
        if p.read_unaligned() == 0 {
            p.write_unaligned(1);
        }
    }
    1 // clubes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the club_comp.dat and nation_comp.dat table (ids 12, 16) of 107-byte records in one go, through zread (save)
/// or fread (Data), with entry version 2; version 1: the `db_files_read_comps_v1` converter (other: box and exit); 1 if everything was
/// read.
///
/// `db_files_read_comps(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 2, reads in
/// one go the count (+0x37) of 107-byte records (club_comp.dat and nation_comp.dat) through `zread` `zipdir_zread` (use_zip ≠
/// 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Version 1: the field-by-field
/// converter `db_files_read_comps_v1` (stream, use_zip, records, count), called by address, and its answer. Another version: the
/// "Error" box (Db_files.cpp) and the game exits; the copy calls the original. The original's 0x200 frame only serves
/// the box path.
/// Original: `db_files_read_comps` (0x54feb0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_comps(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version == 1 {
        covered!("database read competitions version 1");
        return exe::db_files_read_comps_v1(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMPS)(stream, use_zip, records, entry);
    }
    covered!("database read competitions");
    let read = if use_zip != 0 {
        exe::zread(records, 107, n, stream)
    } else {
        exe::crt_fread(records, 107, n, stream)
    };
    (read == n) as i32 // club competitions
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the non-player table of staff.dat (id 9; the community map calls it players) of 68-byte records in one go,
/// through zread (save) or fread (Data), with entry version 2; version 1: the `db_files_read_players_v1` converter (other: box and exit);
/// 1 if everything was read.
///
/// `db_files_read_players(stream, use_zip, records, entry)` (the community map swaps the names: it is table id 9, the
/// non-players, 0xae23c0): with the index entry version (+0x3f) equal to 2, reads in one go the count (+0x37) of
/// 68-byte records through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if
/// everything was read. Version 1: the field-by-field converter `db_files_read_players_v1` (stream, use_zip, records, count), called by
/// address, and its answer. Another version: the "Error" box (Db_files.cpp) and the game exits; the copy calls the
/// original. The original's 0x200 frame only serves the box path.
/// Original: `db_files_read_players` (0x54e880, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_players(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version == 1 {
        covered!("database read non-players version 1");
        return exe::db_files_read_players_v1(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_PLAYERS)(stream, use_zip, records, entry);
    }
    covered!("database read non-players");
    let read = if use_zip != 0 {
        exe::zread(records, 68, n, stream)
    } else {
        exe::crt_fread(records, 68, n, stream)
    };
    (read == n) as i32 // non-players
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the player table of staff.dat (id 10; the community map calls it non_players) of 70-byte records in one go,
/// through zread (save) or fread (Data), with entry version 2; version 1: the `db_files_read_non_players_v1` converter (other: box and exit);
/// 1 if everything was read.
///
/// `db_files_read_non_players(stream, use_zip, records, entry)` (the community map swaps the names: it is table id 10,
/// the players, 0xae23c4): with the index entry version (+0x3f) equal to 2, reads in one go the count (+0x37) of
/// 70-byte records through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if
/// everything was read. Version 1: the field-by-field converter `db_files_read_non_players_v1` (stream, use_zip, records, count), called by
/// address, and its answer. Another version: the "Error" box (Db_files.cpp) and the game exits; the copy calls the
/// original. The original's 0x200 frame only serves the box path.
/// Original: `db_files_read_non_players` (0x54f030, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_non_players(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version == 1 {
        covered!("database read players version 1");
        return exe::db_files_read_non_players_v1(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NON_PLAYERS)(stream, use_zip, records, entry);
    }
    covered!("database read players");
    let read = if use_zip != 0 {
        exe::zread(records, 70, n, stream)
    } else {
        exe::crt_fread(records, 70, n, stream)
    };
    (read == n) as i32 // jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the staff.dat table (id 6; staff, the 110-byte file version: the save and the community databases) of 110-byte
/// records in one go, through zread (save) or fread (Data), with entry version 2 (other: box and exit); 1 if everything
/// was read.
///
/// `db_files_read_staff(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 2, reads in
/// one go the count (+0x37) of 110-byte records (staff.dat in version 2: the save and the community databases; the
/// stock database is version 1, `db_files_read_staff_v1`) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or `fread` `exe::crt_fread` (the
/// Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and the game exits; the copy
/// calls the original. The original's 0x200 frame only serves the box path (_splitpath): nothing from the stack goes
/// into the table.
/// Original: `db_files_read_staff` (0x54e630, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_staff(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF)(stream, use_zip, records, entry);
    }
    covered!("database read people 110");
    let read = if use_zip != 0 {
        exe::zread(records, 110, n, stream)
    } else {
        exe::crt_fread(records, 110, n, stream)
    };
    (read == n) as i32 // pessoas 110
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is loading the save (the table read becomes the game database) and the copy’s branch.

/// Reads the staff preferences table (id 22: the save and the community databases) of 52-byte records in one go,
/// through zread (save) or fread (Data), with entry version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_staff_extra(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads
/// in one go the count (+0x37) of 52-byte records (the preferences, id 22: the save and the community databases; in the
/// stock database they come out of the conversion of the 157-byte persons) through `zread` `zipdir_zread` (use_zip ≠ 0: the
/// save) or `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box
/// (Db_files.cpp) and the game exits; the copy calls the original. The original's 0x200 frame only serves the box path
/// (_splitpath): nothing from the stack goes into the table.
/// Original: `db_files_read_staff_extra` (0x54fc10, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_staff_extra(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_EXTRA)(stream, use_zip, records, entry);
    }
    covered!("database read preferences");
    let read = if use_zip != 0 {
        exe::zread(records, 52, n, stream)
    } else {
        exe::crt_fread(records, 52, n, stream)
    };
    (read == n) as i32 // preferences
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is loading the save (the table read becomes the game database) and the copy’s branch.

/// Reads the continent.dat table (id 3) of 198-byte records in one go, through zread (save) or fread (Data), with entry
/// version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_continents(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads
/// in one go the count (+0x37) of 198-byte records (continent.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or
/// `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and
/// the game exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath):
/// nothing from the stack goes into the table.
/// Original: `db_files_read_continents` (0x54d5e0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_continents(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CONTINENTS)(stream, use_zip, records, entry);
    }
    covered!("database read continents");
    let read = if use_zip != 0 {
        exe::zread(records, 198, n, stream)
    } else {
        exe::crt_fread(records, 198, n, stream)
    };
    (read == n) as i32 // continentes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the nation.dat table (id 4) of 290-byte records in one go, through zread (save) or fread (Data), with entry
/// version 2 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_nations(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 2, reads in
/// one go the count (+0x37) of 290-byte records (nation.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or
/// `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and
/// the game exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath):
/// nothing from the stack goes into the table.
/// Original: `db_files_read_nations` (0x54d740, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_nations(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NATIONS)(stream, use_zip, records, entry);
    }
    covered!("database read nations");
    let read = if use_zip != 0 {
        exe::zread(records, 290, n, stream)
    } else {
        exe::crt_fread(records, 290, n, stream)
    };
    (read == n) as i32 // nations
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the city.dat table (id 21) of 56-byte records in one go, through zread (save) or fread (Data), with entry
/// version 1 to 2 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_cities(stream, use_zip, records, entry)`: with the index entry version (+0x3f) from 1 to 2 (signed),
/// reads in one go the count (+0x37) of 56-byte records (city.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or
/// `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and
/// the game exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath):
/// nothing from the stack goes into the table.
/// Original: `db_files_read_cities` (0x54d8a0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_cities(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version <= 0 || version > 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CITIES)(stream, use_zip, records, entry);
    }
    covered!("database read cities");
    let read = if use_zip != 0 {
        exe::zread(records, 56, n, stream)
    } else {
        exe::crt_fread(records, 56, n, stream)
    };
    (read == n) as i32 // cidades
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the stadium.dat table (id 5) of 78-byte records in one go, through zread (save) or fread (Data), with entry
/// version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_stadiums(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads in
/// one go the count (+0x37) of 78-byte records (stadium.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or
/// `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and
/// the game exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath):
/// nothing from the stack goes into the table.
/// Original: `db_files_read_stadiums` (0x54da00, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_stadiums(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STADIUMS)(stream, use_zip, records, entry);
    }
    covered!("database read stadiums");
    let read = if use_zip != 0 {
        exe::zread(records, 78, n, stream)
    } else {
        exe::crt_fread(records, 78, n, stream)
    };
    (read == n) as i32 // stadiums
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the staff.dat table (id 6; staff, the 157-byte file version) of 157-byte records in one go, through zread
/// (save) or fread (Data), with entry version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_staff_v1(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1, reads in
/// one go the count (+0x37) of 157-byte records (staff.dat: persons, the 157-byte file version) through `zread`
/// `zipdir_zread` (use_zip ≠ 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version:
/// the "Error" box (Db_files.cpp) and the game exits; the copy calls the original. The original's 0x200 frame only
/// serves the box path (_splitpath): nothing from the stack goes into the table.
/// Original: `db_files_read_staff_v1` (0x54e780, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_staff_v1(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_V1)(stream, use_zip, records, entry);
    }
    covered!("database read people 157");
    let read = if use_zip != 0 {
        exe::zread(records, 157, n, stream)
    } else {
        exe::crt_fread(records, 157, n, stream)
    };
    (read == n) as i32 // pessoas 157
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the staff_history.dat table (id 17) of 17-byte records in one go, through zread (save) or fread (Data), with
/// entry version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_staff_history(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1,
/// reads in one go the count (+0x37) of 17-byte records (staff_history.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the
/// save) or `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box
/// (Db_files.cpp) and the game exits; the copy calls the original. The original's 0x200 frame only serves the box path
/// (_splitpath): nothing from the stack goes into the table.
/// Original: `db_files_read_staff_history` (0x54fd60, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_staff_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_HISTORY)(stream, use_zip, records, entry);
    }
    covered!("database read people history");
    let read = if use_zip != 0 {
        exe::zread(records, 17, n, stream)
    } else {
        exe::crt_fread(records, 17, n, stream)
    };
    (read == n) as i32 // people history
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the club_comp_history.dat and nation_comp_history.dat table (ids 19, 20) of 26-byte records in one go, through
/// zread (save) or fread (Data), with entry version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_comp_history(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 1,
/// reads in one go the count (+0x37) of 26-byte records (club_comp_history.dat and nation_comp_history.dat) through
/// `zread` `zipdir_zread` (use_zip ≠ 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another
/// version: the "Error" box (Db_files.cpp) and the game exits; the copy calls the original. The original's 0x200 frame
/// only serves the box path (_splitpath): nothing from the stack goes into the table.
/// Original: `db_files_read_comp_history` (0x550230, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_comp_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMP_HISTORY)(stream, use_zip, records, entry);
    }
    covered!("database read competition histories");
    let read = if use_zip != 0 {
        exe::zread(records, 26, n, stream)
    } else {
        exe::crt_fread(records, 26, n, stream)
    };
    (read == n) as i32 // competition histories
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the staff_comp.dat table (id 11) of 101-byte records in one go, through zread (save) or fread (Data), with
/// entry version 2 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_staff_comps(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to 2, reads
/// in one go the count (+0x37) of 101-byte records (staff_comp.dat) through `zread` `zipdir_zread` (use_zip ≠ 0: the save) or
/// `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box (Db_files.cpp) and
/// the game exits; the copy calls the original. The original's 0x200 frame only serves the box path (_splitpath):
/// nothing from the stack goes into the table.
/// Original: `db_files_read_staff_comps` (0x550380, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_staff_comps(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMPS)(stream, use_zip, records, entry);
    }
    covered!("database read staff competitions");
    let read = if use_zip != 0 {
        exe::zread(records, 101, n, stream)
    } else {
        exe::crt_fread(records, 101, n, stream)
    };
    (read == n) as i32 // staff competitions
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads the staff_comp_history.dat table (id 18) of 58-byte records in one go, through zread (save) or fread (Data),
/// with entry version 1 (other: box and exit); 1 if everything was read.
///
/// `db_files_read_staff_comp_history(stream, use_zip, records, entry)`: with the index entry version (+0x3f) equal to
/// 1, reads in one go the count (+0x37) of 58-byte records (staff_comp_history.dat) through `zread` `zipdir_zread` (use_zip ≠
/// 0: the save) or `fread` `exe::crt_fread` (the Data folder); 1 if everything was read. Another version: the "Error" box
/// (Db_files.cpp) and the game exits; the copy calls the original. The original's 0x200 frame only serves the box path
/// (_splitpath): nothing from the stack goes into the table.
/// Original: `db_files_read_staff_comp_history` (0x5504d0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_staff_comp_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = read_u32(entry + db_index_entry::RECORD_COUNT);
    let version = read_i32(entry + db_index_entry::VERSION);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMP_HISTORY)(stream, use_zip, records, entry);
    }
    covered!("database read staff competition history");
    let read = if use_zip != 0 {
        exe::zread(records, 58, n, stream)
    } else {
        exe::crt_fread(records, 58, n, stream)
    };
    (read == n) as i32 // staff competition history
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: original only; the proof is the .qst (the table read goes into the save) and the copy’s branch in Rust mode.

/// Reads version 1 club_comp.dat / nation_comp.dat field by field (104 bytes on file for 107-byte records, holes at
/// +0x37 and +0x52), the reputation in one byte; 0 at the first read that fails, 1 if everything was read.
///
/// `db_files_read_comps_v1(stream, use_zip, records, n)`: for each of the `n` 107-byte records, the fields of
/// `COMPETITION_V1_LAYOUT`, each read on its own (size, 1) through `zread` `zipdir_zread` (use_zip ≠ 0) or `fread` `exe::crt_fread`;
/// the reputation, read into one read_i8(the original reads it into an argument slot on its own stack + (already used)),
/// becomes a u16 with no arithmetic. The first read that fails gives 0 (what was already read stays); everything, 1.
/// Called by `db_files_read_comps` `db_files_read_comps` with the entry version equal to 1; the stock database and the save are
/// version 2. No SEH and no local variable read without a value.
/// Original: `db_files_read_comps_v1` (0x54ffd0, Db_files.cpp, cdecl).
pub(crate) unsafe extern "C" fn db_files_read_comps_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    covered!("database read competitions v1");
    let read: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = records + i * 107;
        for &(o, t, q) in &COMPETITION_V1_LAYOUT {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if read(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o) as *mut u16).write_unaligned(b as u16); // competition reputation
                } else if read(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

