// ---------- cx12_6: the real 2001/02 Serie C2/C calendar (ita_ser_c2c.cpp) ----------

/// The 306 fixed matches of the 2001/02 Serie C2/C (day, month with 0 = January, year, home, away), with the club
/// names in the exe's `.rdata`, in the original routine's order (extracted from the disassembly and checked: the vector
/// positions go in steps of 16; the same extraction gives back `SERIE_C2B_FIXTURES` exactly).
pub(crate) const SERIE_C2C_FIXTURES: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea918, 0x9ea924), // Acireale × Fidelis Andria
    (2, 8, 2001, 0x9ea900, 0x9ea90c), // Campobasso × Igea Virtus
    (2, 8, 2001, 0x9ea8e8, 0x9ea8f4), // Catanzaro × Giugliano
    (2, 8, 2001, 0x9ea8d8, 0x9ea8e0), // Cavese × Martina
    (2, 8, 2001, 0x9ea8c4, 0x9ea8cc), // Foggia × Puteolana
    (2, 8, 2001, 0x9ea8b0, 0x9ea8bc), // Frosinone × Paternò
    (2, 8, 2001, 0x9ea8a0, 0x9ea8a8), // Gela × Fasano
    (2, 8, 2001, 0x9ea890, 0x9ea898), // Nardò × Palmese
    (2, 8, 2001, 0x9ea878, 0x9ea880), // Tricase × Sant'Anastasia
    (9, 8, 2001, 0x9ea8a8, 0x9ea890), // Fasano × Nardò
    (9, 8, 2001, 0x9ea924, 0x9ea8b0), // Fidelis Andria × Frosinone
    (9, 8, 2001, 0x9ea8f4, 0x9ea918), // Giugliano × Acireale
    (9, 8, 2001, 0x9ea90c, 0x9ea8c4), // Igea Virtus × Foggia
    (9, 8, 2001, 0x9ea8e0, 0x9ea8a0), // Martina × Gela
    (9, 8, 2001, 0x9ea898, 0x9ea8e8), // Palmese × Catanzaro
    (9, 8, 2001, 0x9ea8bc, 0x9ea900), // Paternò × Campobasso
    (9, 8, 2001, 0x9ea8cc, 0x9ea878), // Puteolana × Tricase
    (9, 8, 2001, 0x9ea880, 0x9ea8d8), // Sant'Anastasia × Cavese
    (16, 8, 2001, 0x9ea918, 0x9ea8bc), // Acireale × Paternò
    (16, 8, 2001, 0x9ea900, 0x9ea8cc), // Campobasso × Puteolana
    (16, 8, 2001, 0x9ea8e8, 0x9ea8a8), // Catanzaro × Fasano
    (16, 8, 2001, 0x9ea924, 0x9ea8e0), // Fidelis Andria × Martina
    (16, 8, 2001, 0x9ea8c4, 0x9ea8f4), // Foggia × Giugliano
    (16, 8, 2001, 0x9ea8b0, 0x9ea90c), // Frosinone × Igea Virtus
    (16, 8, 2001, 0x9ea8a0, 0x9ea880), // Gela × Sant'Anastasia
    (16, 8, 2001, 0x9ea890, 0x9ea8d8), // Nardò × Cavese
    (16, 8, 2001, 0x9ea878, 0x9ea898), // Tricase × Palmese
    (23, 8, 2001, 0x9ea8d8, 0x9ea8a0), // Cavese × Gela
    (23, 8, 2001, 0x9ea8a8, 0x9ea878), // Fasano × Tricase
    (23, 8, 2001, 0x9ea8f4, 0x9ea8b0), // Giugliano × Frosinone
    (23, 8, 2001, 0x9ea90c, 0x9ea924), // Igea Virtus × Fidelis Andria
    (23, 8, 2001, 0x9ea8e0, 0x9ea8e8), // Martina × Catanzaro
    (23, 8, 2001, 0x9ea898, 0x9ea900), // Palmese × Campobasso
    (23, 8, 2001, 0x9ea8bc, 0x9ea8c4), // Paternò × Foggia
    (23, 8, 2001, 0x9ea8cc, 0x9ea918), // Puteolana × Acireale
    (23, 8, 2001, 0x9ea880, 0x9ea890), // Sant'Anastasia × Nardò
    (30, 8, 2001, 0x9ea918, 0x9ea880), // Acireale × Sant'Anastasia
    (30, 8, 2001, 0x9ea900, 0x9ea8a8), // Campobasso × Fasano
    (30, 8, 2001, 0x9ea8e8, 0x9ea8a0), // Catanzaro × Gela
    (30, 8, 2001, 0x9ea924, 0x9ea8f4), // Fidelis Andria × Giugliano
    (30, 8, 2001, 0x9ea8c4, 0x9ea898), // Foggia × Palmese
    (30, 8, 2001, 0x9ea8b0, 0x9ea878), // Frosinone × Tricase
    (30, 8, 2001, 0x9ea890, 0x9ea8e0), // Nardò × Martina
    (30, 8, 2001, 0x9ea8bc, 0x9ea90c), // Paternò × Igea Virtus
    (30, 8, 2001, 0x9ea8cc, 0x9ea8d8), // Puteolana × Cavese
    (7, 9, 2001, 0x9ea8d8, 0x9ea900), // Cavese × Campobasso
    (7, 9, 2001, 0x9ea8a8, 0x9ea8b0), // Fasano × Frosinone
    (7, 9, 2001, 0x9ea8a0, 0x9ea8c4), // Gela × Foggia
    (7, 9, 2001, 0x9ea90c, 0x9ea8f4), // Igea Virtus × Giugliano
    (7, 9, 2001, 0x9ea8e0, 0x9ea8bc), // Martina × Paternò
    (7, 9, 2001, 0x9ea890, 0x9ea8e8), // Nardò × Catanzaro
    (7, 9, 2001, 0x9ea898, 0x9ea924), // Palmese × Fidelis Andria
    (7, 9, 2001, 0x9ea880, 0x9ea8cc), // Sant'Anastasia × Puteolana
    (7, 9, 2001, 0x9ea878, 0x9ea918), // Tricase × Acireale
    (14, 9, 2001, 0x9ea918, 0x9ea8a0), // Acireale × Gela
    (14, 9, 2001, 0x9ea900, 0x9ea890), // Campobasso × Nardò
    (14, 9, 2001, 0x9ea924, 0x9ea8c4), // Fidelis Andria × Foggia
    (14, 9, 2001, 0x9ea8b0, 0x9ea8e8), // Frosinone × Catanzaro
    (14, 9, 2001, 0x9ea8f4, 0x9ea8cc), // Giugliano × Puteolana
    (14, 9, 2001, 0x9ea90c, 0x9ea880), // Igea Virtus × Sant'Anastasia
    (14, 9, 2001, 0x9ea898, 0x9ea8d8), // Palmese × Cavese
    (14, 9, 2001, 0x9ea8bc, 0x9ea8a8), // Paternò × Fasano
    (14, 9, 2001, 0x9ea878, 0x9ea8e0), // Tricase × Martina
    (21, 9, 2001, 0x9ea8e8, 0x9ea918), // Catanzaro × Acireale
    (21, 9, 2001, 0x9ea8d8, 0x9ea878), // Cavese × Tricase
    (21, 9, 2001, 0x9ea8a8, 0x9ea8f4), // Fasano × Giugliano
    (21, 9, 2001, 0x9ea8c4, 0x9ea900), // Foggia × Campobasso
    (21, 9, 2001, 0x9ea8a0, 0x9ea8bc), // Gela × Paternò
    (21, 9, 2001, 0x9ea8e0, 0x9ea90c), // Martina × Igea Virtus
    (21, 9, 2001, 0x9ea890, 0x9ea924), // Nardò × Fidelis Andria
    (21, 9, 2001, 0x9ea8cc, 0x9ea8b0), // Puteolana × Frosinone
    (21, 9, 2001, 0x9ea880, 0x9ea898), // Sant'Anastasia × Palmese
    (28, 9, 2001, 0x9ea918, 0x9ea8c4), // Acireale × Foggia
    (28, 9, 2001, 0x9ea924, 0x9ea900), // Fidelis Andria × Campobasso
    (28, 9, 2001, 0x9ea8b0, 0x9ea890), // Frosinone × Nardò
    (28, 9, 2001, 0x9ea8f4, 0x9ea8a0), // Giugliano × Gela
    (28, 9, 2001, 0x9ea90c, 0x9ea8d8), // Igea Virtus × Cavese
    (28, 9, 2001, 0x9ea898, 0x9ea8a8), // Palmese × Fasano
    (28, 9, 2001, 0x9ea8bc, 0x9ea8cc), // Paternò × Puteolana
    (28, 9, 2001, 0x9ea880, 0x9ea8e0), // Sant'Anastasia × Martina
    (28, 9, 2001, 0x9ea878, 0x9ea8e8), // Tricase × Catanzaro
    (4, 10, 2001, 0x9ea900, 0x9ea918), // Campobasso × Acireale
    (4, 10, 2001, 0x9ea8e8, 0x9ea8bc), // Catanzaro × Paternò
    (4, 10, 2001, 0x9ea8d8, 0x9ea8f4), // Cavese × Giugliano
    (4, 10, 2001, 0x9ea8a8, 0x9ea880), // Fasano × Sant'Anastasia
    (4, 10, 2001, 0x9ea8c4, 0x9ea8b0), // Foggia × Frosinone
    (4, 10, 2001, 0x9ea8a0, 0x9ea878), // Gela × Tricase
    (4, 10, 2001, 0x9ea8e0, 0x9ea898), // Martina × Palmese
    (4, 10, 2001, 0x9ea890, 0x9ea90c), // Nardò × Igea Virtus
    (4, 10, 2001, 0x9ea8cc, 0x9ea924), // Puteolana × Fidelis Andria
    (11, 10, 2001, 0x9ea8d8, 0x9ea8a8), // Cavese × Fasano
    (11, 10, 2001, 0x9ea8b0, 0x9ea918), // Frosinone × Acireale
    (11, 10, 2001, 0x9ea8f4, 0x9ea900), // Giugliano × Campobasso
    (11, 10, 2001, 0x9ea90c, 0x9ea8cc), // Igea Virtus × Puteolana
    (11, 10, 2001, 0x9ea8e0, 0x9ea8c4), // Martina × Foggia
    (11, 10, 2001, 0x9ea898, 0x9ea8a0), // Palmese × Gela
    (11, 10, 2001, 0x9ea8bc, 0x9ea924), // Paternò × Fidelis Andria
    (11, 10, 2001, 0x9ea880, 0x9ea8e8), // Sant'Anastasia × Catanzaro
    (11, 10, 2001, 0x9ea878, 0x9ea890), // Tricase × Nardò
    (18, 10, 2001, 0x9ea918, 0x9ea90c), // Acireale × Igea Virtus
    (18, 10, 2001, 0x9ea900, 0x9ea8b0), // Campobasso × Frosinone
    (18, 10, 2001, 0x9ea8e8, 0x9ea8d8), // Catanzaro × Cavese
    (18, 10, 2001, 0x9ea8a8, 0x9ea8e0), // Fasano × Martina
    (18, 10, 2001, 0x9ea924, 0x9ea880), // Fidelis Andria × Sant'Anastasia
    (18, 10, 2001, 0x9ea8c4, 0x9ea878), // Foggia × Tricase
    (18, 10, 2001, 0x9ea8a0, 0x9ea890), // Gela × Nardò
    (18, 10, 2001, 0x9ea8f4, 0x9ea8bc), // Giugliano × Paternò
    (18, 10, 2001, 0x9ea8cc, 0x9ea898), // Puteolana × Palmese
    (25, 10, 2001, 0x9ea8e8, 0x9ea900), // Catanzaro × Campobasso
    (25, 10, 2001, 0x9ea8d8, 0x9ea8bc), // Cavese × Paternò
    (25, 10, 2001, 0x9ea8a8, 0x9ea918), // Fasano × Acireale
    (25, 10, 2001, 0x9ea8a0, 0x9ea8b0), // Gela × Frosinone
    (25, 10, 2001, 0x9ea8e0, 0x9ea8cc), // Martina × Puteolana
    (25, 10, 2001, 0x9ea890, 0x9ea8c4), // Nardò × Foggia
    (25, 10, 2001, 0x9ea898, 0x9ea90c), // Palmese × Igea Virtus
    (25, 10, 2001, 0x9ea880, 0x9ea8f4), // Sant'Anastasia × Giugliano
    (25, 10, 2001, 0x9ea878, 0x9ea924), // Tricase × Fidelis Andria
    (2, 11, 2001, 0x9ea918, 0x9ea8e0), // Acireale × Martina
    (2, 11, 2001, 0x9ea900, 0x9ea8a0), // Campobasso × Gela
    (2, 11, 2001, 0x9ea924, 0x9ea8a8), // Fidelis Andria × Fasano
    (2, 11, 2001, 0x9ea8c4, 0x9ea8e8), // Foggia × Catanzaro
    (2, 11, 2001, 0x9ea8b0, 0x9ea8d8), // Frosinone × Cavese
    (2, 11, 2001, 0x9ea8f4, 0x9ea898), // Giugliano × Palmese
    (2, 11, 2001, 0x9ea90c, 0x9ea878), // Igea Virtus × Tricase
    (2, 11, 2001, 0x9ea8bc, 0x9ea880), // Paternò × Sant'Anastasia
    (2, 11, 2001, 0x9ea8cc, 0x9ea890), // Puteolana × Nardò
    (9, 11, 2001, 0x9ea8e8, 0x9ea8cc), // Catanzaro × Puteolana
    (9, 11, 2001, 0x9ea8d8, 0x9ea924), // Cavese × Fidelis Andria
    (9, 11, 2001, 0x9ea8a8, 0x9ea8c4), // Fasano × Foggia
    (9, 11, 2001, 0x9ea8a0, 0x9ea90c), // Gela × Igea Virtus
    (9, 11, 2001, 0x9ea8e0, 0x9ea8b0), // Martina × Frosinone
    (9, 11, 2001, 0x9ea890, 0x9ea918), // Nardò × Acireale
    (9, 11, 2001, 0x9ea898, 0x9ea8bc), // Palmese × Paternò
    (9, 11, 2001, 0x9ea880, 0x9ea900), // Sant'Anastasia × Campobasso
    (9, 11, 2001, 0x9ea878, 0x9ea8f4), // Tricase × Giugliano
    (16, 11, 2001, 0x9ea918, 0x9ea898), // Acireale × Palmese
    (16, 11, 2001, 0x9ea900, 0x9ea878), // Campobasso × Tricase
    (16, 11, 2001, 0x9ea924, 0x9ea8e8), // Fidelis Andria × Catanzaro
    (16, 11, 2001, 0x9ea8c4, 0x9ea8d8), // Foggia × Cavese
    (16, 11, 2001, 0x9ea8b0, 0x9ea880), // Frosinone × Sant'Anastasia
    (16, 11, 2001, 0x9ea8f4, 0x9ea8e0), // Giugliano × Martina
    (16, 11, 2001, 0x9ea90c, 0x9ea8a8), // Igea Virtus × Fasano
    (16, 11, 2001, 0x9ea8bc, 0x9ea890), // Paternò × Nardò
    (16, 11, 2001, 0x9ea8cc, 0x9ea8a0), // Puteolana × Gela
    (23, 11, 2001, 0x9ea8e8, 0x9ea90c), // Catanzaro × Igea Virtus
    (23, 11, 2001, 0x9ea8d8, 0x9ea918), // Cavese × Acireale
    (23, 11, 2001, 0x9ea8a8, 0x9ea8cc), // Fasano × Puteolana
    (23, 11, 2001, 0x9ea8a0, 0x9ea924), // Gela × Fidelis Andria
    (23, 11, 2001, 0x9ea8e0, 0x9ea900), // Martina × Campobasso
    (23, 11, 2001, 0x9ea890, 0x9ea8f4), // Nardò × Giugliano
    (23, 11, 2001, 0x9ea898, 0x9ea8b0), // Palmese × Frosinone
    (23, 11, 2001, 0x9ea880, 0x9ea8c4), // Sant'Anastasia × Foggia
    (23, 11, 2001, 0x9ea878, 0x9ea8bc), // Tricase × Paternò
    (6, 0, 2002, 0x9ea924, 0x9ea918), // Fidelis Andria × Acireale
    (6, 0, 2002, 0x9ea90c, 0x9ea900), // Igea Virtus × Campobasso
    (6, 0, 2002, 0x9ea8f4, 0x9ea8e8), // Giugliano × Catanzaro
    (6, 0, 2002, 0x9ea8e0, 0x9ea8d8), // Martina × Cavese
    (6, 0, 2002, 0x9ea8cc, 0x9ea8c4), // Puteolana × Foggia
    (6, 0, 2002, 0x9ea8bc, 0x9ea8b0), // Paternò × Frosinone
    (6, 0, 2002, 0x9ea8a8, 0x9ea8a0), // Fasano × Gela
    (6, 0, 2002, 0x9ea898, 0x9ea890), // Palmese × Nardò
    (6, 0, 2002, 0x9ea880, 0x9ea878), // Sant'Anastasia × Tricase
    (13, 0, 2002, 0x9ea890, 0x9ea8a8), // Nardò × Fasano
    (13, 0, 2002, 0x9ea8b0, 0x9ea924), // Frosinone × Fidelis Andria
    (13, 0, 2002, 0x9ea918, 0x9ea8f4), // Acireale × Giugliano
    (13, 0, 2002, 0x9ea8c4, 0x9ea90c), // Foggia × Igea Virtus
    (13, 0, 2002, 0x9ea8a0, 0x9ea8e0), // Gela × Martina
    (13, 0, 2002, 0x9ea8e8, 0x9ea898), // Catanzaro × Palmese
    (13, 0, 2002, 0x9ea900, 0x9ea8bc), // Campobasso × Paternò
    (13, 0, 2002, 0x9ea878, 0x9ea8cc), // Tricase × Puteolana
    (13, 0, 2002, 0x9ea8d8, 0x9ea880), // Cavese × Sant'Anastasia
    (20, 0, 2002, 0x9ea8bc, 0x9ea918), // Paternò × Acireale
    (20, 0, 2002, 0x9ea8cc, 0x9ea900), // Puteolana × Campobasso
    (20, 0, 2002, 0x9ea8a8, 0x9ea8e8), // Fasano × Catanzaro
    (20, 0, 2002, 0x9ea8e0, 0x9ea924), // Martina × Fidelis Andria
    (20, 0, 2002, 0x9ea8f4, 0x9ea8c4), // Giugliano × Foggia
    (20, 0, 2002, 0x9ea90c, 0x9ea8b0), // Igea Virtus × Frosinone
    (20, 0, 2002, 0x9ea880, 0x9ea8a0), // Sant'Anastasia × Gela
    (20, 0, 2002, 0x9ea8d8, 0x9ea890), // Cavese × Nardò
    (20, 0, 2002, 0x9ea898, 0x9ea878), // Palmese × Tricase
    (27, 0, 2002, 0x9ea8a0, 0x9ea8d8), // Gela × Cavese
    (27, 0, 2002, 0x9ea878, 0x9ea8a8), // Tricase × Fasano
    (27, 0, 2002, 0x9ea8b0, 0x9ea8f4), // Frosinone × Giugliano
    (27, 0, 2002, 0x9ea924, 0x9ea90c), // Fidelis Andria × Igea Virtus
    (27, 0, 2002, 0x9ea8e8, 0x9ea8e0), // Catanzaro × Martina
    (27, 0, 2002, 0x9ea900, 0x9ea898), // Campobasso × Palmese
    (27, 0, 2002, 0x9ea8c4, 0x9ea8bc), // Foggia × Paternò
    (27, 0, 2002, 0x9ea918, 0x9ea8cc), // Acireale × Puteolana
    (27, 0, 2002, 0x9ea890, 0x9ea880), // Nardò × Sant'Anastasia
    (3, 1, 2002, 0x9ea880, 0x9ea918), // Sant'Anastasia × Acireale
    (3, 1, 2002, 0x9ea8a8, 0x9ea900), // Fasano × Campobasso
    (3, 1, 2002, 0x9ea8a0, 0x9ea8e8), // Gela × Catanzaro
    (3, 1, 2002, 0x9ea8f4, 0x9ea924), // Giugliano × Fidelis Andria
    (3, 1, 2002, 0x9ea898, 0x9ea8c4), // Palmese × Foggia
    (3, 1, 2002, 0x9ea878, 0x9ea8b0), // Tricase × Frosinone
    (3, 1, 2002, 0x9ea8e0, 0x9ea890), // Martina × Nardò
    (3, 1, 2002, 0x9ea90c, 0x9ea8bc), // Igea Virtus × Paternò
    (3, 1, 2002, 0x9ea8d8, 0x9ea8cc), // Cavese × Puteolana
    (10, 1, 2002, 0x9ea900, 0x9ea8d8), // Campobasso × Cavese
    (10, 1, 2002, 0x9ea8b0, 0x9ea8a8), // Frosinone × Fasano
    (10, 1, 2002, 0x9ea8c4, 0x9ea8a0), // Foggia × Gela
    (10, 1, 2002, 0x9ea8f4, 0x9ea90c), // Giugliano × Igea Virtus
    (10, 1, 2002, 0x9ea8bc, 0x9ea8e0), // Paternò × Martina
    (10, 1, 2002, 0x9ea8e8, 0x9ea890), // Catanzaro × Nardò
    (10, 1, 2002, 0x9ea924, 0x9ea898), // Fidelis Andria × Palmese
    (10, 1, 2002, 0x9ea8cc, 0x9ea880), // Puteolana × Sant'Anastasia
    (10, 1, 2002, 0x9ea918, 0x9ea878), // Acireale × Tricase
    (17, 1, 2002, 0x9ea8a0, 0x9ea918), // Gela × Acireale
    (17, 1, 2002, 0x9ea890, 0x9ea900), // Nardò × Campobasso
    (17, 1, 2002, 0x9ea8c4, 0x9ea924), // Foggia × Fidelis Andria
    (17, 1, 2002, 0x9ea8e8, 0x9ea8b0), // Catanzaro × Frosinone
    (17, 1, 2002, 0x9ea8cc, 0x9ea8f4), // Puteolana × Giugliano
    (17, 1, 2002, 0x9ea880, 0x9ea90c), // Sant'Anastasia × Igea Virtus
    (17, 1, 2002, 0x9ea8d8, 0x9ea898), // Cavese × Palmese
    (17, 1, 2002, 0x9ea8a8, 0x9ea8bc), // Fasano × Paternò
    (17, 1, 2002, 0x9ea8e0, 0x9ea878), // Martina × Tricase
    (24, 1, 2002, 0x9ea918, 0x9ea8e8), // Acireale × Catanzaro
    (24, 1, 2002, 0x9ea878, 0x9ea8d8), // Tricase × Cavese
    (24, 1, 2002, 0x9ea8f4, 0x9ea8a8), // Giugliano × Fasano
    (24, 1, 2002, 0x9ea900, 0x9ea8c4), // Campobasso × Foggia
    (24, 1, 2002, 0x9ea8bc, 0x9ea8a0), // Paternò × Gela
    (24, 1, 2002, 0x9ea90c, 0x9ea8e0), // Igea Virtus × Martina
    (24, 1, 2002, 0x9ea924, 0x9ea890), // Fidelis Andria × Nardò
    (24, 1, 2002, 0x9ea8b0, 0x9ea8cc), // Frosinone × Puteolana
    (24, 1, 2002, 0x9ea898, 0x9ea880), // Palmese × Sant'Anastasia
    (3, 2, 2002, 0x9ea8c4, 0x9ea918), // Foggia × Acireale
    (3, 2, 2002, 0x9ea900, 0x9ea924), // Campobasso × Fidelis Andria
    (3, 2, 2002, 0x9ea890, 0x9ea8b0), // Nardò × Frosinone
    (3, 2, 2002, 0x9ea8a0, 0x9ea8f4), // Gela × Giugliano
    (3, 2, 2002, 0x9ea8d8, 0x9ea90c), // Cavese × Igea Virtus
    (3, 2, 2002, 0x9ea8a8, 0x9ea898), // Fasano × Palmese
    (3, 2, 2002, 0x9ea8cc, 0x9ea8bc), // Puteolana × Paternò
    (3, 2, 2002, 0x9ea8e0, 0x9ea880), // Martina × Sant'Anastasia
    (3, 2, 2002, 0x9ea8e8, 0x9ea878), // Catanzaro × Tricase
    (10, 2, 2002, 0x9ea918, 0x9ea900), // Acireale × Campobasso
    (10, 2, 2002, 0x9ea8bc, 0x9ea8e8), // Paternò × Catanzaro
    (10, 2, 2002, 0x9ea8f4, 0x9ea8d8), // Giugliano × Cavese
    (10, 2, 2002, 0x9ea880, 0x9ea8a8), // Sant'Anastasia × Fasano
    (10, 2, 2002, 0x9ea8b0, 0x9ea8c4), // Frosinone × Foggia
    (10, 2, 2002, 0x9ea878, 0x9ea8a0), // Tricase × Gela
    (10, 2, 2002, 0x9ea898, 0x9ea8e0), // Palmese × Martina
    (10, 2, 2002, 0x9ea90c, 0x9ea890), // Igea Virtus × Nardò
    (10, 2, 2002, 0x9ea924, 0x9ea8cc), // Fidelis Andria × Puteolana
    (24, 2, 2002, 0x9ea8a8, 0x9ea8d8), // Fasano × Cavese
    (24, 2, 2002, 0x9ea918, 0x9ea8b0), // Acireale × Frosinone
    (24, 2, 2002, 0x9ea900, 0x9ea8f4), // Campobasso × Giugliano
    (24, 2, 2002, 0x9ea8cc, 0x9ea90c), // Puteolana × Igea Virtus
    (24, 2, 2002, 0x9ea8c4, 0x9ea8e0), // Foggia × Martina
    (24, 2, 2002, 0x9ea8a0, 0x9ea898), // Gela × Palmese
    (24, 2, 2002, 0x9ea924, 0x9ea8bc), // Fidelis Andria × Paternò
    (24, 2, 2002, 0x9ea8e8, 0x9ea880), // Catanzaro × Sant'Anastasia
    (24, 2, 2002, 0x9ea890, 0x9ea878), // Nardò × Tricase
    (30, 2, 2002, 0x9ea90c, 0x9ea918), // Igea Virtus × Acireale
    (30, 2, 2002, 0x9ea8b0, 0x9ea900), // Frosinone × Campobasso
    (30, 2, 2002, 0x9ea8d8, 0x9ea8e8), // Cavese × Catanzaro
    (30, 2, 2002, 0x9ea8e0, 0x9ea8a8), // Martina × Fasano
    (30, 2, 2002, 0x9ea880, 0x9ea924), // Sant'Anastasia × Fidelis Andria
    (30, 2, 2002, 0x9ea878, 0x9ea8c4), // Tricase × Foggia
    (30, 2, 2002, 0x9ea890, 0x9ea8a0), // Nardò × Gela
    (30, 2, 2002, 0x9ea8bc, 0x9ea8f4), // Paternò × Giugliano
    (30, 2, 2002, 0x9ea898, 0x9ea8cc), // Palmese × Puteolana
    (7, 3, 2002, 0x9ea900, 0x9ea8e8), // Campobasso × Catanzaro
    (7, 3, 2002, 0x9ea8bc, 0x9ea8d8), // Paternò × Cavese
    (7, 3, 2002, 0x9ea918, 0x9ea8a8), // Acireale × Fasano
    (7, 3, 2002, 0x9ea8b0, 0x9ea8a0), // Frosinone × Gela
    (7, 3, 2002, 0x9ea8cc, 0x9ea8e0), // Puteolana × Martina
    (7, 3, 2002, 0x9ea8c4, 0x9ea890), // Foggia × Nardò
    (7, 3, 2002, 0x9ea90c, 0x9ea898), // Igea Virtus × Palmese
    (7, 3, 2002, 0x9ea8f4, 0x9ea880), // Giugliano × Sant'Anastasia
    (7, 3, 2002, 0x9ea924, 0x9ea878), // Fidelis Andria × Tricase
    (14, 3, 2002, 0x9ea8e0, 0x9ea918), // Martina × Acireale
    (14, 3, 2002, 0x9ea8a0, 0x9ea900), // Gela × Campobasso
    (14, 3, 2002, 0x9ea8a8, 0x9ea924), // Fasano × Fidelis Andria
    (14, 3, 2002, 0x9ea8e8, 0x9ea8c4), // Catanzaro × Foggia
    (14, 3, 2002, 0x9ea8d8, 0x9ea8b0), // Cavese × Frosinone
    (14, 3, 2002, 0x9ea898, 0x9ea8f4), // Palmese × Giugliano
    (14, 3, 2002, 0x9ea878, 0x9ea90c), // Tricase × Igea Virtus
    (14, 3, 2002, 0x9ea880, 0x9ea8bc), // Sant'Anastasia × Paternò
    (14, 3, 2002, 0x9ea890, 0x9ea8cc), // Nardò × Puteolana
    (21, 3, 2002, 0x9ea8cc, 0x9ea8e8), // Puteolana × Catanzaro
    (21, 3, 2002, 0x9ea924, 0x9ea8d8), // Fidelis Andria × Cavese
    (21, 3, 2002, 0x9ea8c4, 0x9ea8a8), // Foggia × Fasano
    (21, 3, 2002, 0x9ea90c, 0x9ea8a0), // Igea Virtus × Gela
    (21, 3, 2002, 0x9ea8b0, 0x9ea8e0), // Frosinone × Martina
    (21, 3, 2002, 0x9ea918, 0x9ea890), // Acireale × Nardò
    (21, 3, 2002, 0x9ea8bc, 0x9ea898), // Paternò × Palmese
    (21, 3, 2002, 0x9ea900, 0x9ea880), // Campobasso × Sant'Anastasia
    (21, 3, 2002, 0x9ea8f4, 0x9ea878), // Giugliano × Tricase
    (28, 3, 2002, 0x9ea898, 0x9ea918), // Palmese × Acireale
    (28, 3, 2002, 0x9ea878, 0x9ea900), // Tricase × Campobasso
    (28, 3, 2002, 0x9ea8e8, 0x9ea924), // Catanzaro × Fidelis Andria
    (28, 3, 2002, 0x9ea8d8, 0x9ea8c4), // Cavese × Foggia
    (28, 3, 2002, 0x9ea880, 0x9ea8b0), // Sant'Anastasia × Frosinone
    (28, 3, 2002, 0x9ea8e0, 0x9ea8f4), // Martina × Giugliano
    (28, 3, 2002, 0x9ea8a8, 0x9ea90c), // Fasano × Igea Virtus
    (28, 3, 2002, 0x9ea890, 0x9ea8bc), // Nardò × Paternò
    (28, 3, 2002, 0x9ea8a0, 0x9ea8cc), // Gela × Puteolana
    (5, 4, 2002, 0x9ea90c, 0x9ea8e8), // Igea Virtus × Catanzaro
    (5, 4, 2002, 0x9ea918, 0x9ea8d8), // Acireale × Cavese
    (5, 4, 2002, 0x9ea8cc, 0x9ea8a8), // Puteolana × Fasano
    (5, 4, 2002, 0x9ea924, 0x9ea8a0), // Fidelis Andria × Gela
    (5, 4, 2002, 0x9ea900, 0x9ea8e0), // Campobasso × Martina
    (5, 4, 2002, 0x9ea8f4, 0x9ea890), // Giugliano × Nardò
    (5, 4, 2002, 0x9ea8b0, 0x9ea898), // Frosinone × Palmese
    (5, 4, 2002, 0x9ea8c4, 0x9ea880), // Foggia × Sant'Anastasia
    (5, 4, 2002, 0x9ea8bc, 0x9ea878), // Paternò × Tricase
];

/// The 2001/02 Serie C2/C: the 306 matches (34 rounds of 9) fixed by club name (`set_up_fixed_fixtures_body`, Serie
/// C1's body, checked instruction by instruction: only the table, the source file and the box line 0x3cd change); with
/// a name missing from the database the list is dropped and the league draws its schedule.
/// Original: `ItaSerC2c::load_fixed_fixtures` (0x65c8e0, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2c_set_up_fixed_fixtures(this: usize) {
    covered!("cx12_6 serie c2 c fixed calendar");
    set_up_fixed_fixtures_body(League(this), &SERIE_C2C_FIXTURES, 0x9ea83c, 0x3cd)
}

/// Comparer for `ita_ser_c2c_set_up_fixed_fixtures`: only the original runs (it allocates); afterwards the list is
/// either dropped or has every fixture with both clubs found (`fra_first_compare_fixed_fixtures`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2c_set_up_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    fra_first_compare_fixed_fixtures("ita_ser_c2c_set_up_fixed_fixtures", League(this), || o(this))
}
