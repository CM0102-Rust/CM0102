// ---------- cx11_12: the real 2001/02 Serie C2/B calendar (ita_ser_c2b.cpp) ----------

/// The 306 fixed matches of the 2001/02 Serie C2/B (day, month with 0 = January, year, home, away), with the club
/// names in the exe's `.rdata`, in the original routine's order (extracted from the disassembly and checked: the vector
/// positions go in steps of 16; the same extraction gives back `C1A_FIXTURES` exactly).
pub(crate) const SERIE_C2B_FIXTURES: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea82c, 0x9ea834), // Gubbio × Trento
    (2, 8, 2001, 0x9ea818, 0x9ea820), // Imolese × Fiorenzuola
    (2, 8, 2001, 0x9ea804, 0x9ea80c), // Mantova × Brescello
    (2, 8, 2001, 0x9ea7fc, 0x9d9860), // Mestre × San Marino
    (2, 8, 2001, 0x9ea7ec, 0x9ea7f4), // Rimini × Gualdo
    (2, 8, 2001, 0x9ea7d4, 0x9ea7e4), // Sambenedettese × Teramo
    (2, 8, 2001, 0x9ea7bc, 0x9ea7c8), // Sassuolo × Montichiari
    (2, 8, 2001, 0x9ea7a8, 0x9ea7b4), // Alto Adige × Poggese
    (2, 8, 2001, 0x9ea798, 0x9ea7a0), // Thiene × Faenza
    (9, 8, 2001, 0x9ea80c, 0x9ea7d4), // Brescello × Sambenedettese
    (9, 8, 2001, 0x9ea7a0, 0x9ea804), // Faenza × Mantova
    (9, 8, 2001, 0x9ea820, 0x9ea7ec), // Fiorenzuola × Rimini
    (9, 8, 2001, 0x9ea7f4, 0x9ea82c), // Gualdo × Gubbio
    (9, 8, 2001, 0x9ea7c8, 0x9ea7fc), // Montichiari × Mestre
    (9, 8, 2001, 0x9ea7b4, 0x9ea818), // Poggese × Imolese
    (9, 8, 2001, 0x9d9860, 0x9ea7bc), // San Marino × Sassuolo
    (9, 8, 2001, 0x9ea7e4, 0x9ea798), // Teramo × Thiene
    (9, 8, 2001, 0x9ea834, 0x9ea7a8), // Trento × Alto Adige
    (16, 8, 2001, 0x9ea82c, 0x9ea7a0), // Gubbio × Faenza
    (16, 8, 2001, 0x9ea818, 0x9ea7ec), // Imolese × Rimini
    (16, 8, 2001, 0x9ea804, 0x9ea7e4), // Mantova × Teramo
    (16, 8, 2001, 0x9ea7fc, 0x9ea80c), // Mestre × Brescello
    (16, 8, 2001, 0x9ea7d4, 0x9ea7b4), // Sambenedettese × Poggese
    (16, 8, 2001, 0x9d9860, 0x9ea7f4), // San Marino × Gualdo
    (16, 8, 2001, 0x9ea7bc, 0x9ea820), // Sassuolo × Fiorenzuola
    (16, 8, 2001, 0x9ea7a8, 0x9ea7c8), // Alto Adige × Montichiari
    (16, 8, 2001, 0x9ea798, 0x9ea834), // Thiene × Trento
    (23, 8, 2001, 0x9ea80c, 0x9ea798), // Brescello × Thiene
    (23, 8, 2001, 0x9ea7a0, 0x9d9860), // Faenza × San Marino
    (23, 8, 2001, 0x9ea820, 0x9ea7a8), // Fiorenzuola × Alto Adige
    (23, 8, 2001, 0x9ea7f4, 0x9ea7d4), // Gualdo × Sambenedettese
    (23, 8, 2001, 0x9ea7c8, 0x9ea804), // Montichiari × Mantova
    (23, 8, 2001, 0x9ea7b4, 0x9ea82c), // Poggese × Gubbio
    (23, 8, 2001, 0x9ea7ec, 0x9ea7bc), // Rimini × Sassuolo
    (23, 8, 2001, 0x9ea7e4, 0x9ea7fc), // Teramo × Mestre
    (23, 8, 2001, 0x9ea834, 0x9ea818), // Trento × Imolese
    (30, 8, 2001, 0x9ea7a0, 0x9ea7f4), // Faenza × Gualdo
    (30, 8, 2001, 0x9ea820, 0x9ea7e4), // Fiorenzuola × Teramo
    (30, 8, 2001, 0x9ea82c, 0x9ea7c8), // Gubbio × Montichiari
    (30, 8, 2001, 0x9ea7b4, 0x9ea80c), // Poggese × Brescello
    (30, 8, 2001, 0x9d9860, 0x9ea7d4), // San Marino × Sambenedettese
    (30, 8, 2001, 0x9ea7bc, 0x9ea818), // Sassuolo × Imolese
    (30, 8, 2001, 0x9ea7a8, 0x9ea7ec), // Alto Adige × Rimini
    (30, 8, 2001, 0x9ea798, 0x9ea804), // Thiene × Mantova
    (30, 8, 2001, 0x9ea834, 0x9ea7fc), // Trento × Mestre
    (7, 9, 2001, 0x9ea80c, 0x9ea7a0), // Brescello × Faenza
    (7, 9, 2001, 0x9ea7f4, 0x9ea834), // Gualdo × Trento
    (7, 9, 2001, 0x9ea818, 0x9ea7a8), // Imolese × Alto Adige
    (7, 9, 2001, 0x9ea804, 0x9d9860), // Mantova × San Marino
    (7, 9, 2001, 0x9ea7fc, 0x9ea82c), // Mestre × Gubbio
    (7, 9, 2001, 0x9ea7c8, 0x9ea820), // Montichiari × Fiorenzuola
    (7, 9, 2001, 0x9ea7ec, 0x9ea7b4), // Rimini × Poggese
    (7, 9, 2001, 0x9ea7d4, 0x9ea798), // Sambenedettese × Thiene
    (7, 9, 2001, 0x9ea7e4, 0x9ea7bc), // Teramo × Sassuolo
    (14, 9, 2001, 0x9ea7a0, 0x9ea7d4), // Faenza × Sambenedettese
    (14, 9, 2001, 0x9ea820, 0x9ea804), // Fiorenzuola × Mantova
    (14, 9, 2001, 0x9ea7f4, 0x9ea7fc), // Gualdo × Mestre
    (14, 9, 2001, 0x9ea82c, 0x9ea7ec), // Gubbio × Rimini
    (14, 9, 2001, 0x9ea7b4, 0x9ea7bc), // Poggese × Sassuolo
    (14, 9, 2001, 0x9d9860, 0x9ea80c), // San Marino × Brescello
    (14, 9, 2001, 0x9ea7a8, 0x9ea7e4), // Alto Adige × Teramo
    (14, 9, 2001, 0x9ea798, 0x9ea818), // Thiene × Imolese
    (14, 9, 2001, 0x9ea834, 0x9ea7c8), // Trento × Montichiari
    (21, 9, 2001, 0x9ea80c, 0x9ea7c8), // Brescello × Montichiari
    (21, 9, 2001, 0x9ea818, 0x9ea82c), // Imolese × Gubbio
    (21, 9, 2001, 0x9ea804, 0x9ea7f4), // Mantova × Gualdo
    (21, 9, 2001, 0x9ea7fc, 0x9ea7a0), // Mestre × Faenza
    (21, 9, 2001, 0x9ea7ec, 0x9d9860), // Rimini × San Marino
    (21, 9, 2001, 0x9ea7d4, 0x9ea7a8), // Sambenedettese × Alto Adige
    (21, 9, 2001, 0x9ea7bc, 0x9ea834), // Sassuolo × Trento
    (21, 9, 2001, 0x9ea7e4, 0x9ea7b4), // Teramo × Poggese
    (21, 9, 2001, 0x9ea798, 0x9ea820), // Thiene × Fiorenzuola
    (28, 9, 2001, 0x9ea7a0, 0x9ea7ec), // Faenza × Rimini
    (28, 9, 2001, 0x9ea7f4, 0x9ea80c), // Gualdo × Brescello
    (28, 9, 2001, 0x9ea82c, 0x9ea7bc), // Gubbio × Sassuolo
    (28, 9, 2001, 0x9ea7c8, 0x9ea7e4), // Montichiari × Teramo
    (28, 9, 2001, 0x9ea7b4, 0x9ea804), // Poggese × Mantova
    (28, 9, 2001, 0x9ea7d4, 0x9ea818), // Sambenedettese × Imolese
    (28, 9, 2001, 0x9d9860, 0x9ea798), // San Marino × Thiene
    (28, 9, 2001, 0x9ea7a8, 0x9ea7fc), // Alto Adige × Mestre
    (28, 9, 2001, 0x9ea834, 0x9ea820), // Trento × Fiorenzuola
    (4, 10, 2001, 0x9ea80c, 0x9ea7a8), // Brescello × Alto Adige
    (4, 10, 2001, 0x9ea820, 0x9ea7b4), // Fiorenzuola × Poggese
    (4, 10, 2001, 0x9ea818, 0x9ea7c8), // Imolese × Montichiari
    (4, 10, 2001, 0x9ea804, 0x9ea82c), // Mantova × Gubbio
    (4, 10, 2001, 0x9ea7fc, 0x9ea7d4), // Mestre × Sambenedettese
    (4, 10, 2001, 0x9ea7ec, 0x9ea834), // Rimini × Trento
    (4, 10, 2001, 0x9ea7bc, 0x9ea7a0), // Sassuolo × Faenza
    (4, 10, 2001, 0x9ea7e4, 0x9d9860), // Teramo × San Marino
    (4, 10, 2001, 0x9ea798, 0x9ea7f4), // Thiene × Gualdo
    (11, 10, 2001, 0x9ea7a0, 0x9ea7e4), // Faenza × Teramo
    (11, 10, 2001, 0x9ea7f4, 0x9ea820), // Gualdo × Fiorenzuola
    (11, 10, 2001, 0x9ea82c, 0x9ea80c), // Gubbio × Brescello
    (11, 10, 2001, 0x9ea7fc, 0x9ea7ec), // Mestre × Rimini
    (11, 10, 2001, 0x9ea7c8, 0x9ea798), // Montichiari × Thiene
    (11, 10, 2001, 0x9ea7b4, 0x9ea834), // Poggese × Trento
    (11, 10, 2001, 0x9ea7d4, 0x9ea804), // Sambenedettese × Mantova
    (11, 10, 2001, 0x9d9860, 0x9ea818), // San Marino × Imolese
    (11, 10, 2001, 0x9ea7a8, 0x9ea7bc), // Alto Adige × Sassuolo
    (18, 10, 2001, 0x9ea820, 0x9ea7d4), // Fiorenzuola × Sambenedettese
    (18, 10, 2001, 0x9ea818, 0x9ea7a0), // Imolese × Faenza
    (18, 10, 2001, 0x9ea804, 0x9ea7fc), // Mantova × Mestre
    (18, 10, 2001, 0x9ea7ec, 0x9ea7c8), // Rimini × Montichiari
    (18, 10, 2001, 0x9ea7bc, 0x9ea7f4), // Sassuolo × Gualdo
    (18, 10, 2001, 0x9ea7a8, 0x9ea82c), // Alto Adige × Gubbio
    (18, 10, 2001, 0x9ea7e4, 0x9ea80c), // Teramo × Brescello
    (18, 10, 2001, 0x9ea798, 0x9ea7b4), // Thiene × Poggese
    (18, 10, 2001, 0x9ea834, 0x9d9860), // Trento × San Marino
    (25, 10, 2001, 0x9ea80c, 0x9ea7bc), // Brescello × Sassuolo
    (25, 10, 2001, 0x9ea7a0, 0x9ea834), // Faenza × Trento
    (25, 10, 2001, 0x9ea7f4, 0x9ea7e4), // Gualdo × Teramo
    (25, 10, 2001, 0x9ea82c, 0x9ea820), // Gubbio × Fiorenzuola
    (25, 10, 2001, 0x9ea804, 0x9ea818), // Mantova × Imolese
    (25, 10, 2001, 0x9ea7fc, 0x9ea798), // Mestre × Thiene
    (25, 10, 2001, 0x9ea7c8, 0x9ea7b4), // Montichiari × Poggese
    (25, 10, 2001, 0x9ea7d4, 0x9ea7ec), // Sambenedettese × Rimini
    (25, 10, 2001, 0x9d9860, 0x9ea7a8), // San Marino × Alto Adige
    (2, 11, 2001, 0x9ea820, 0x9ea80c), // Fiorenzuola × Brescello
    (2, 11, 2001, 0x9ea82c, 0x9ea7e4), // Gubbio × Teramo
    (2, 11, 2001, 0x9ea818, 0x9ea7fc), // Imolese × Mestre
    (2, 11, 2001, 0x9ea7c8, 0x9ea7a0), // Montichiari × Faenza
    (2, 11, 2001, 0x9ea7b4, 0x9d9860), // Poggese × San Marino
    (2, 11, 2001, 0x9ea7ec, 0x9ea798), // Rimini × Thiene
    (2, 11, 2001, 0x9ea7bc, 0x9ea804), // Sassuolo × Mantova
    (2, 11, 2001, 0x9ea7a8, 0x9ea7f4), // Alto Adige × Gualdo
    (2, 11, 2001, 0x9ea834, 0x9ea7d4), // Trento × Sambenedettese
    (9, 11, 2001, 0x9ea80c, 0x9ea834), // Brescello × Trento
    (9, 11, 2001, 0x9ea7a0, 0x9ea7a8), // Faenza × Alto Adige
    (9, 11, 2001, 0x9ea7f4, 0x9ea7c8), // Gualdo × Montichiari
    (9, 11, 2001, 0x9ea804, 0x9ea7ec), // Mantova × Rimini
    (9, 11, 2001, 0x9ea7fc, 0x9ea7b4), // Mestre × Poggese
    (9, 11, 2001, 0x9ea7d4, 0x9ea82c), // Sambenedettese × Gubbio
    (9, 11, 2001, 0x9d9860, 0x9ea820), // San Marino × Fiorenzuola
    (9, 11, 2001, 0x9ea7e4, 0x9ea818), // Teramo × Imolese
    (9, 11, 2001, 0x9ea798, 0x9ea7bc), // Thiene × Sassuolo
    (16, 11, 2001, 0x9ea820, 0x9ea7a0), // Fiorenzuola × Faenza
    (16, 11, 2001, 0x9ea82c, 0x9d9860), // Gubbio × San Marino
    (16, 11, 2001, 0x9ea818, 0x9ea80c), // Imolese × Brescello
    (16, 11, 2001, 0x9ea7c8, 0x9ea7d4), // Montichiari × Sambenedettese
    (16, 11, 2001, 0x9ea7b4, 0x9ea7f4), // Poggese × Gualdo
    (16, 11, 2001, 0x9ea7ec, 0x9ea7e4), // Rimini × Teramo
    (16, 11, 2001, 0x9ea7bc, 0x9ea7fc), // Sassuolo × Mestre
    (16, 11, 2001, 0x9ea7a8, 0x9ea798), // Alto Adige × Thiene
    (16, 11, 2001, 0x9ea834, 0x9ea804), // Trento × Mantova
    (23, 11, 2001, 0x9ea80c, 0x9ea7ec), // Brescello × Rimini
    (23, 11, 2001, 0x9ea7a0, 0x9ea7b4), // Faenza × Poggese
    (23, 11, 2001, 0x9ea7f4, 0x9ea818), // Gualdo × Imolese
    (23, 11, 2001, 0x9ea804, 0x9ea7a8), // Mantova × Alto Adige
    (23, 11, 2001, 0x9ea7fc, 0x9ea820), // Mestre × Fiorenzuola
    (23, 11, 2001, 0x9ea7d4, 0x9ea7bc), // Sambenedettese × Sassuolo
    (23, 11, 2001, 0x9d9860, 0x9ea7c8), // San Marino × Montichiari
    (23, 11, 2001, 0x9ea7e4, 0x9ea834), // Teramo × Trento
    (23, 11, 2001, 0x9ea798, 0x9ea82c), // Thiene × Gubbio
    (6, 0, 2002, 0x9ea834, 0x9ea82c), // Trento × Gubbio
    (6, 0, 2002, 0x9ea820, 0x9ea818), // Fiorenzuola × Imolese
    (6, 0, 2002, 0x9ea80c, 0x9ea804), // Brescello × Mantova
    (6, 0, 2002, 0x9d9860, 0x9ea7fc), // San Marino × Mestre
    (6, 0, 2002, 0x9ea7f4, 0x9ea7ec), // Gualdo × Rimini
    (6, 0, 2002, 0x9ea7e4, 0x9ea7d4), // Teramo × Sambenedettese
    (6, 0, 2002, 0x9ea7c8, 0x9ea7bc), // Montichiari × Sassuolo
    (6, 0, 2002, 0x9ea7b4, 0x9ea7a8), // Poggese × Alto Adige
    (6, 0, 2002, 0x9ea7a0, 0x9ea798), // Faenza × Thiene
    (13, 0, 2002, 0x9ea7d4, 0x9ea80c), // Sambenedettese × Brescello
    (13, 0, 2002, 0x9ea804, 0x9ea7a0), // Mantova × Faenza
    (13, 0, 2002, 0x9ea7ec, 0x9ea820), // Rimini × Fiorenzuola
    (13, 0, 2002, 0x9ea82c, 0x9ea7f4), // Gubbio × Gualdo
    (13, 0, 2002, 0x9ea7fc, 0x9ea7c8), // Mestre × Montichiari
    (13, 0, 2002, 0x9ea818, 0x9ea7b4), // Imolese × Poggese
    (13, 0, 2002, 0x9ea7bc, 0x9d9860), // Sassuolo × San Marino
    (13, 0, 2002, 0x9ea798, 0x9ea7e4), // Thiene × Teramo
    (13, 0, 2002, 0x9ea7a8, 0x9ea834), // Alto Adige × Trento
    (20, 0, 2002, 0x9ea7a0, 0x9ea82c), // Faenza × Gubbio
    (20, 0, 2002, 0x9ea7ec, 0x9ea818), // Rimini × Imolese
    (20, 0, 2002, 0x9ea7e4, 0x9ea804), // Teramo × Mantova
    (20, 0, 2002, 0x9ea80c, 0x9ea7fc), // Brescello × Mestre
    (20, 0, 2002, 0x9ea7b4, 0x9ea7d4), // Poggese × Sambenedettese
    (20, 0, 2002, 0x9ea7f4, 0x9d9860), // Gualdo × San Marino
    (20, 0, 2002, 0x9ea820, 0x9ea7bc), // Fiorenzuola × Sassuolo
    (20, 0, 2002, 0x9ea7c8, 0x9ea7a8), // Montichiari × Alto Adige
    (20, 0, 2002, 0x9ea834, 0x9ea798), // Trento × Thiene
    (27, 0, 2002, 0x9ea798, 0x9ea80c), // Thiene × Brescello
    (27, 0, 2002, 0x9d9860, 0x9ea7a0), // San Marino × Faenza
    (27, 0, 2002, 0x9ea7a8, 0x9ea820), // Alto Adige × Fiorenzuola
    (27, 0, 2002, 0x9ea7d4, 0x9ea7f4), // Sambenedettese × Gualdo
    (27, 0, 2002, 0x9ea804, 0x9ea7c8), // Mantova × Montichiari
    (27, 0, 2002, 0x9ea82c, 0x9ea7b4), // Gubbio × Poggese
    (27, 0, 2002, 0x9ea7bc, 0x9ea7ec), // Sassuolo × Rimini
    (27, 0, 2002, 0x9ea7fc, 0x9ea7e4), // Mestre × Teramo
    (27, 0, 2002, 0x9ea818, 0x9ea834), // Imolese × Trento
    (3, 1, 2002, 0x9ea7f4, 0x9ea7a0), // Gualdo × Faenza
    (3, 1, 2002, 0x9ea7e4, 0x9ea820), // Teramo × Fiorenzuola
    (3, 1, 2002, 0x9ea7c8, 0x9ea82c), // Montichiari × Gubbio
    (3, 1, 2002, 0x9ea80c, 0x9ea7b4), // Brescello × Poggese
    (3, 1, 2002, 0x9ea7d4, 0x9d9860), // Sambenedettese × San Marino
    (3, 1, 2002, 0x9ea818, 0x9ea7bc), // Imolese × Sassuolo
    (3, 1, 2002, 0x9ea7ec, 0x9ea7a8), // Rimini × Alto Adige
    (3, 1, 2002, 0x9ea804, 0x9ea798), // Mantova × Thiene
    (3, 1, 2002, 0x9ea7fc, 0x9ea834), // Mestre × Trento
    (10, 1, 2002, 0x9ea7a0, 0x9ea80c), // Faenza × Brescello
    (10, 1, 2002, 0x9ea834, 0x9ea7f4), // Trento × Gualdo
    (10, 1, 2002, 0x9ea7a8, 0x9ea818), // Alto Adige × Imolese
    (10, 1, 2002, 0x9d9860, 0x9ea804), // San Marino × Mantova
    (10, 1, 2002, 0x9ea82c, 0x9ea7fc), // Gubbio × Mestre
    (10, 1, 2002, 0x9ea820, 0x9ea7c8), // Fiorenzuola × Montichiari
    (10, 1, 2002, 0x9ea7b4, 0x9ea7ec), // Poggese × Rimini
    (10, 1, 2002, 0x9ea798, 0x9ea7d4), // Thiene × Sambenedettese
    (10, 1, 2002, 0x9ea7bc, 0x9ea7e4), // Sassuolo × Teramo
    (17, 1, 2002, 0x9ea7d4, 0x9ea7a0), // Sambenedettese × Faenza
    (17, 1, 2002, 0x9ea804, 0x9ea820), // Mantova × Fiorenzuola
    (17, 1, 2002, 0x9ea7fc, 0x9ea7f4), // Mestre × Gualdo
    (17, 1, 2002, 0x9ea7ec, 0x9ea82c), // Rimini × Gubbio
    (17, 1, 2002, 0x9ea7bc, 0x9ea7b4), // Sassuolo × Poggese
    (17, 1, 2002, 0x9ea80c, 0x9d9860), // Brescello × San Marino
    (17, 1, 2002, 0x9ea7e4, 0x9ea7a8), // Teramo × Alto Adige
    (17, 1, 2002, 0x9ea818, 0x9ea798), // Imolese × Thiene
    (17, 1, 2002, 0x9ea7c8, 0x9ea834), // Montichiari × Trento
    (24, 1, 2002, 0x9ea7c8, 0x9ea80c), // Montichiari × Brescello
    (24, 1, 2002, 0x9ea82c, 0x9ea818), // Gubbio × Imolese
    (24, 1, 2002, 0x9ea7f4, 0x9ea804), // Gualdo × Mantova
    (24, 1, 2002, 0x9ea7a0, 0x9ea7fc), // Faenza × Mestre
    (24, 1, 2002, 0x9d9860, 0x9ea7ec), // San Marino × Rimini
    (24, 1, 2002, 0x9ea7a8, 0x9ea7d4), // Alto Adige × Sambenedettese
    (24, 1, 2002, 0x9ea834, 0x9ea7bc), // Trento × Sassuolo
    (24, 1, 2002, 0x9ea7b4, 0x9ea7e4), // Poggese × Teramo
    (24, 1, 2002, 0x9ea820, 0x9ea798), // Fiorenzuola × Thiene
    (3, 2, 2002, 0x9ea7ec, 0x9ea7a0), // Rimini × Faenza
    (3, 2, 2002, 0x9ea80c, 0x9ea7f4), // Brescello × Gualdo
    (3, 2, 2002, 0x9ea7bc, 0x9ea82c), // Sassuolo × Gubbio
    (3, 2, 2002, 0x9ea7e4, 0x9ea7c8), // Teramo × Montichiari
    (3, 2, 2002, 0x9ea804, 0x9ea7b4), // Mantova × Poggese
    (3, 2, 2002, 0x9ea818, 0x9ea7d4), // Imolese × Sambenedettese
    (3, 2, 2002, 0x9ea798, 0x9d9860), // Thiene × San Marino
    (3, 2, 2002, 0x9ea7fc, 0x9ea7a8), // Mestre × Alto Adige
    (3, 2, 2002, 0x9ea820, 0x9ea834), // Fiorenzuola × Trento
    (10, 2, 2002, 0x9ea7a8, 0x9ea80c), // Alto Adige × Brescello
    (10, 2, 2002, 0x9ea7b4, 0x9ea820), // Poggese × Fiorenzuola
    (10, 2, 2002, 0x9ea7c8, 0x9ea818), // Montichiari × Imolese
    (10, 2, 2002, 0x9ea82c, 0x9ea804), // Gubbio × Mantova
    (10, 2, 2002, 0x9ea7d4, 0x9ea7fc), // Sambenedettese × Mestre
    (10, 2, 2002, 0x9ea834, 0x9ea7ec), // Trento × Rimini
    (10, 2, 2002, 0x9ea7a0, 0x9ea7bc), // Faenza × Sassuolo
    (10, 2, 2002, 0x9d9860, 0x9ea7e4), // San Marino × Teramo
    (10, 2, 2002, 0x9ea7f4, 0x9ea798), // Gualdo × Thiene
    (24, 2, 2002, 0x9ea7e4, 0x9ea7a0), // Teramo × Faenza
    (24, 2, 2002, 0x9ea820, 0x9ea7f4), // Fiorenzuola × Gualdo
    (24, 2, 2002, 0x9ea80c, 0x9ea82c), // Brescello × Gubbio
    (24, 2, 2002, 0x9ea7ec, 0x9ea7fc), // Rimini × Mestre
    (24, 2, 2002, 0x9ea798, 0x9ea7c8), // Thiene × Montichiari
    (24, 2, 2002, 0x9ea834, 0x9ea7b4), // Trento × Poggese
    (24, 2, 2002, 0x9ea804, 0x9ea7d4), // Mantova × Sambenedettese
    (24, 2, 2002, 0x9ea818, 0x9d9860), // Imolese × San Marino
    (24, 2, 2002, 0x9ea7bc, 0x9ea7a8), // Sassuolo × Alto Adige
    (30, 2, 2002, 0x9ea7d4, 0x9ea820), // Sambenedettese × Fiorenzuola
    (30, 2, 2002, 0x9ea7a0, 0x9ea818), // Faenza × Imolese
    (30, 2, 2002, 0x9ea7fc, 0x9ea804), // Mestre × Mantova
    (30, 2, 2002, 0x9ea7c8, 0x9ea7ec), // Montichiari × Rimini
    (30, 2, 2002, 0x9ea7f4, 0x9ea7bc), // Gualdo × Sassuolo
    (30, 2, 2002, 0x9ea82c, 0x9ea7a8), // Gubbio × Alto Adige
    (30, 2, 2002, 0x9ea80c, 0x9ea7e4), // Brescello × Teramo
    (30, 2, 2002, 0x9ea7b4, 0x9ea798), // Poggese × Thiene
    (30, 2, 2002, 0x9d9860, 0x9ea834), // San Marino × Trento
    (7, 3, 2002, 0x9ea7bc, 0x9ea80c), // Sassuolo × Brescello
    (7, 3, 2002, 0x9ea834, 0x9ea7a0), // Trento × Faenza
    (7, 3, 2002, 0x9ea7e4, 0x9ea7f4), // Teramo × Gualdo
    (7, 3, 2002, 0x9ea820, 0x9ea82c), // Fiorenzuola × Gubbio
    (7, 3, 2002, 0x9ea818, 0x9ea804), // Imolese × Mantova
    (7, 3, 2002, 0x9ea798, 0x9ea7fc), // Thiene × Mestre
    (7, 3, 2002, 0x9ea7b4, 0x9ea7c8), // Poggese × Montichiari
    (7, 3, 2002, 0x9ea7ec, 0x9ea7d4), // Rimini × Sambenedettese
    (7, 3, 2002, 0x9ea7a8, 0x9d9860), // Alto Adige × San Marino
    (14, 3, 2002, 0x9ea80c, 0x9ea820), // Brescello × Fiorenzuola
    (14, 3, 2002, 0x9ea7e4, 0x9ea82c), // Teramo × Gubbio
    (14, 3, 2002, 0x9ea7fc, 0x9ea818), // Mestre × Imolese
    (14, 3, 2002, 0x9ea7a0, 0x9ea7c8), // Faenza × Montichiari
    (14, 3, 2002, 0x9d9860, 0x9ea7b4), // San Marino × Poggese
    (14, 3, 2002, 0x9ea798, 0x9ea7ec), // Thiene × Rimini
    (14, 3, 2002, 0x9ea804, 0x9ea7bc), // Mantova × Sassuolo
    (14, 3, 2002, 0x9ea7f4, 0x9ea7a8), // Gualdo × Alto Adige
    (14, 3, 2002, 0x9ea7d4, 0x9ea834), // Sambenedettese × Trento
    (21, 3, 2002, 0x9ea834, 0x9ea80c), // Trento × Brescello
    (21, 3, 2002, 0x9ea7a8, 0x9ea7a0), // Alto Adige × Faenza
    (21, 3, 2002, 0x9ea7c8, 0x9ea7f4), // Montichiari × Gualdo
    (21, 3, 2002, 0x9ea7ec, 0x9ea804), // Rimini × Mantova
    (21, 3, 2002, 0x9ea7b4, 0x9ea7fc), // Poggese × Mestre
    (21, 3, 2002, 0x9ea82c, 0x9ea7d4), // Gubbio × Sambenedettese
    (21, 3, 2002, 0x9ea820, 0x9d9860), // Fiorenzuola × San Marino
    (21, 3, 2002, 0x9ea818, 0x9ea7e4), // Imolese × Teramo
    (21, 3, 2002, 0x9ea7bc, 0x9ea798), // Sassuolo × Thiene
    (28, 3, 2002, 0x9ea7a0, 0x9ea820), // Faenza × Fiorenzuola
    (28, 3, 2002, 0x9d9860, 0x9ea82c), // San Marino × Gubbio
    (28, 3, 2002, 0x9ea80c, 0x9ea818), // Brescello × Imolese
    (28, 3, 2002, 0x9ea7d4, 0x9ea7c8), // Sambenedettese × Montichiari
    (28, 3, 2002, 0x9ea7f4, 0x9ea7b4), // Gualdo × Poggese
    (28, 3, 2002, 0x9ea7e4, 0x9ea7ec), // Teramo × Rimini
    (28, 3, 2002, 0x9ea7fc, 0x9ea7bc), // Mestre × Sassuolo
    (28, 3, 2002, 0x9ea798, 0x9ea7a8), // Thiene × Alto Adige
    (28, 3, 2002, 0x9ea804, 0x9ea834), // Mantova × Trento
    (5, 4, 2002, 0x9ea7ec, 0x9ea80c), // Rimini × Brescello
    (5, 4, 2002, 0x9ea7b4, 0x9ea7a0), // Poggese × Faenza
    (5, 4, 2002, 0x9ea818, 0x9ea7f4), // Imolese × Gualdo
    (5, 4, 2002, 0x9ea7a8, 0x9ea804), // Alto Adige × Mantova
    (5, 4, 2002, 0x9ea820, 0x9ea7fc), // Fiorenzuola × Mestre
    (5, 4, 2002, 0x9ea7bc, 0x9ea7d4), // Sassuolo × Sambenedettese
    (5, 4, 2002, 0x9ea7c8, 0x9d9860), // Montichiari × San Marino
    (5, 4, 2002, 0x9ea834, 0x9ea7e4), // Trento × Teramo
    (5, 4, 2002, 0x9ea82c, 0x9ea798), // Gubbio × Thiene
];

/// The 2001/02 Serie C2/B: the 306 matches (34 rounds of 9) fixed by club name (`set_up_fixed_fixtures_body`, Serie
/// C1's body, checked instruction by instruction: only the table, the source file and the box line 0x3d3 change); with a
/// name missing from the database the list is dropped and the league draws its schedule.
/// Original: `ItaSerC2b::load_fixed_fixtures` (0x658750, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_set_up_fixed_fixtures(this: usize) {
    covered!("cx11_12 serie c2 fixed calendar");
    set_up_fixed_fixtures_body(League(this), &SERIE_C2B_FIXTURES, 0x9ea75c, 0x3d3)
}

/// Comparer for `ita_ser_c2b_set_up_fixed_fixtures`: only the original runs (it allocates); afterwards the list is either dropped or has every
/// fixture with both clubs found (`fra_first_compare_fixed_fixtures`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_set_up_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    fra_first_compare_fixed_fixtures("ita_ser_c2b_set_up_fixed_fixtures", League(this), || o(this))
}
