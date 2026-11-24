// ---------- cx11_11: the real 2001/02 Serie C2/A calendar (ita_ser_c2a.cpp) ----------

/// The 306 fixed matches of the 2001/02 Serie C2/A (day, month with 0 = January, year, home, away), with the club
/// names in the exe's `.rdata`, in the original routine's order (extracted from the disassembly and checked: the vector
/// positions go in steps of 16; the same extraction gives back `C1A_FIXTURES` exactly).
pub(crate) const SERIE_C2A_FIXTURES: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea744, 0x9ea750), // Cremonese × Poggibonsi
    (2, 8, 2001, 0x9ea730, 0x9ea738), // Legnano × Alessandria
    (2, 8, 2001, 0x9ea71c, 0x9ea724), // Meda × Viareggio
    (2, 8, 2001, 0x9ea704, 0x9ea710), // Montevarchi × Castelnuovo
    (2, 8, 2001, 0x9ea6f4, 0x9ea6fc), // Pavia × Prato
    (2, 8, 2001, 0x9ea6dc, 0x9ea6e8), // Pro Sesto × Biellese
    (2, 8, 2001, 0x9ea6bc, 0x9ea6cc), // Pro Vercelli × Sangiovannese
    (2, 8, 2001, 0x9ea6a8, 0x9ea6b4), // Rondinella × Novara
    (2, 8, 2001, 0x9ea690, 0x9ea69c), // Valenzana × Pro Patria
    (9, 8, 2001, 0x9ea738, 0x9ea6bc), // Alessandria × Pro Vercelli
    (9, 8, 2001, 0x9ea6e8, 0x9ea71c), // Biellese × Meda
    (9, 8, 2001, 0x9ea710, 0x9ea6a8), // Castelnuovo × Rondinella
    (9, 8, 2001, 0x9ea6b4, 0x9ea744), // Novara × Cremonese
    (9, 8, 2001, 0x9ea750, 0x9ea730), // Poggibonsi × Legnano
    (9, 8, 2001, 0x9ea6fc, 0x9ea6dc), // Prato × Pro Sesto
    (9, 8, 2001, 0x9ea69c, 0x9ea704), // Pro Patria × Montevarchi
    (9, 8, 2001, 0x9ea6cc, 0x9ea6f4), // Sangiovannese × Pavia
    (9, 8, 2001, 0x9ea724, 0x9ea690), // Viareggio × Valenzana
    (16, 8, 2001, 0x9ea710, 0x9ea69c), // Castelnuovo × Pro Patria
    (16, 8, 2001, 0x9ea744, 0x9ea6dc), // Cremonese × Pro Sesto
    (16, 8, 2001, 0x9ea730, 0x9ea6cc), // Legnano × Sangiovannese
    (16, 8, 2001, 0x9ea71c, 0x9ea738), // Meda × Alessandria
    (16, 8, 2001, 0x9ea704, 0x9ea750), // Montevarchi × Poggibonsi
    (16, 8, 2001, 0x9ea6f4, 0x9ea6e8), // Pavia × Biellese
    (16, 8, 2001, 0x9ea6bc, 0x9ea6fc), // Pro Vercelli × Prato
    (16, 8, 2001, 0x9ea6a8, 0x9ea724), // Rondinella × Viareggio
    (16, 8, 2001, 0x9ea690, 0x9ea6b4), // Valenzana × Novara
    (23, 8, 2001, 0x9ea738, 0x9ea6a8), // Alessandria × Rondinella
    (23, 8, 2001, 0x9ea6e8, 0x9ea690), // Biellese × Valenzana
    (23, 8, 2001, 0x9ea6b4, 0x9ea710), // Novara × Castelnuovo
    (23, 8, 2001, 0x9ea750, 0x9ea6bc), // Poggibonsi × Pro Vercelli
    (23, 8, 2001, 0x9ea6fc, 0x9ea730), // Prato × Legnano
    (23, 8, 2001, 0x9ea69c, 0x9ea71c), // Pro Patria × Meda
    (23, 8, 2001, 0x9ea6dc, 0x9ea6f4), // Pro Sesto × Pavia
    (23, 8, 2001, 0x9ea6cc, 0x9ea744), // Sangiovannese × Cremonese
    (23, 8, 2001, 0x9ea724, 0x9ea704), // Viareggio × Montevarchi
    (30, 8, 2001, 0x9ea710, 0x9ea71c), // Castelnuovo × Meda
    (30, 8, 2001, 0x9ea744, 0x9ea724), // Cremonese × Viareggio
    (30, 8, 2001, 0x9ea730, 0x9ea6e8), // Legnano × Biellese
    (30, 8, 2001, 0x9ea704, 0x9ea6b4), // Montevarchi × Novara
    (30, 8, 2001, 0x9ea6f4, 0x9ea750), // Pavia × Poggibonsi
    (30, 8, 2001, 0x9ea6fc, 0x9ea6cc), // Prato × Sangiovannese
    (30, 8, 2001, 0x9ea6bc, 0x9ea69c), // Pro Vercelli × Pro Patria
    (30, 8, 2001, 0x9ea6a8, 0x9ea6dc), // Rondinella × Pro Sesto
    (30, 8, 2001, 0x9ea690, 0x9ea738), // Valenzana × Alessandria
    (7, 9, 2001, 0x9ea738, 0x9ea710), // Alessandria × Castelnuovo
    (7, 9, 2001, 0x9ea6e8, 0x9ea6a8), // Biellese × Rondinella
    (7, 9, 2001, 0x9ea71c, 0x9ea704), // Meda × Montevarchi
    (7, 9, 2001, 0x9ea6f4, 0x9ea730), // Pavia × Legnano
    (7, 9, 2001, 0x9ea750, 0x9ea6fc), // Poggibonsi × Prato
    (7, 9, 2001, 0x9ea69c, 0x9ea744), // Pro Patria × Cremonese
    (7, 9, 2001, 0x9ea6dc, 0x9ea6bc), // Pro Sesto × Pro Vercelli
    (7, 9, 2001, 0x9ea6cc, 0x9ea690), // Sangiovannese × Valenzana
    (7, 9, 2001, 0x9ea724, 0x9ea6b4), // Viareggio × Novara
    (14, 9, 2001, 0x9ea710, 0x9ea6dc), // Castelnuovo × Pro Sesto
    (14, 9, 2001, 0x9ea744, 0x9ea71c), // Cremonese × Meda
    (14, 9, 2001, 0x9ea730, 0x9ea724), // Legnano × Viareggio
    (14, 9, 2001, 0x9ea704, 0x9ea6a8), // Montevarchi × Rondinella
    (14, 9, 2001, 0x9ea6b4, 0x9ea69c), // Novara × Pro Patria
    (14, 9, 2001, 0x9ea750, 0x9ea6cc), // Poggibonsi × Sangiovannese
    (14, 9, 2001, 0x9ea6fc, 0x9ea738), // Prato × Alessandria
    (14, 9, 2001, 0x9ea6bc, 0x9ea6e8), // Pro Vercelli × Biellese
    (14, 9, 2001, 0x9ea690, 0x9ea6f4), // Valenzana × Pavia
    (21, 9, 2001, 0x9ea6e8, 0x9ea750), // Biellese × Poggibonsi
    (21, 9, 2001, 0x9ea71c, 0x9ea6b4), // Meda × Novara
    (21, 9, 2001, 0x9ea6f4, 0x9ea6bc), // Pavia × Pro Vercelli
    (21, 9, 2001, 0x9ea69c, 0x9ea6fc), // Pro Patria × Prato
    (21, 9, 2001, 0x9ea6dc, 0x9ea738), // Pro Sesto × Alessandria
    (21, 9, 2001, 0x9ea6a8, 0x9ea730), // Rondinella × Legnano
    (21, 9, 2001, 0x9ea6cc, 0x9ea704), // Sangiovannese × Montevarchi
    (21, 9, 2001, 0x9ea690, 0x9ea744), // Valenzana × Cremonese
    (21, 9, 2001, 0x9ea724, 0x9ea710), // Viareggio × Castelnuovo
    (28, 9, 2001, 0x9ea738, 0x9ea6e8), // Alessandria × Biellese
    (28, 9, 2001, 0x9ea710, 0x9ea690), // Castelnuovo × Valenzana
    (28, 9, 2001, 0x9ea744, 0x9ea6a8), // Cremonese × Rondinella
    (28, 9, 2001, 0x9ea730, 0x9ea704), // Legnano × Montevarchi
    (28, 9, 2001, 0x9ea71c, 0x9ea6f4), // Meda × Pavia
    (28, 9, 2001, 0x9ea6b4, 0x9ea6fc), // Novara × Prato
    (28, 9, 2001, 0x9ea750, 0x9ea69c), // Poggibonsi × Pro Patria
    (28, 9, 2001, 0x9ea6cc, 0x9ea6dc), // Sangiovannese × Pro Sesto
    (28, 9, 2001, 0x9ea724, 0x9ea6bc), // Viareggio × Pro Vercelli
    (4, 10, 2001, 0x9ea6e8, 0x9ea6cc), // Biellese × Sangiovannese
    (4, 10, 2001, 0x9ea704, 0x9ea744), // Montevarchi × Cremonese
    (4, 10, 2001, 0x9ea6f4, 0x9ea724), // Pavia × Viareggio
    (4, 10, 2001, 0x9ea6fc, 0x9ea710), // Prato × Castelnuovo
    (4, 10, 2001, 0x9ea69c, 0x9ea738), // Pro Patria × Alessandria
    (4, 10, 2001, 0x9ea6dc, 0x9ea750), // Pro Sesto × Poggibonsi
    (4, 10, 2001, 0x9ea6bc, 0x9ea6b4), // Pro Vercelli × Novara
    (4, 10, 2001, 0x9ea6a8, 0x9ea71c), // Rondinella × Meda
    (4, 10, 2001, 0x9ea690, 0x9ea730), // Valenzana × Legnano
    (11, 10, 2001, 0x9ea738, 0x9ea750), // Alessandria × Poggibonsi
    (11, 10, 2001, 0x9ea710, 0x9ea6cc), // Castelnuovo × Sangiovannese
    (11, 10, 2001, 0x9ea744, 0x9ea6f4), // Cremonese × Pavia
    (11, 10, 2001, 0x9ea730, 0x9ea6bc), // Legnano × Pro Vercelli
    (11, 10, 2001, 0x9ea71c, 0x9ea6fc), // Meda × Prato
    (11, 10, 2001, 0x9ea704, 0x9ea690), // Montevarchi × Valenzana
    (11, 10, 2001, 0x9ea6b4, 0x9ea6dc), // Novara × Pro Sesto
    (11, 10, 2001, 0x9ea6a8, 0x9ea69c), // Rondinella × Pro Patria
    (11, 10, 2001, 0x9ea724, 0x9ea6e8), // Viareggio × Biellese
    (18, 10, 2001, 0x9ea744, 0x9ea710), // Cremonese × Castelnuovo
    (18, 10, 2001, 0x9ea6f4, 0x9ea704), // Pavia × Montevarchi
    (18, 10, 2001, 0x9ea750, 0x9ea6b4), // Poggibonsi × Novara
    (18, 10, 2001, 0x9ea6fc, 0x9ea6e8), // Prato × Biellese
    (18, 10, 2001, 0x9ea69c, 0x9ea724), // Pro Patria × Viareggio
    (18, 10, 2001, 0x9ea6dc, 0x9ea730), // Pro Sesto × Legnano
    (18, 10, 2001, 0x9ea6bc, 0x9ea71c), // Pro Vercelli × Meda
    (18, 10, 2001, 0x9ea6cc, 0x9ea738), // Sangiovannese × Alessandria
    (18, 10, 2001, 0x9ea690, 0x9ea6a8), // Valenzana × Rondinella
    (25, 10, 2001, 0x9ea738, 0x9ea6f4), // Alessandria × Pavia
    (25, 10, 2001, 0x9ea6e8, 0x9ea69c), // Biellese × Pro Patria
    (25, 10, 2001, 0x9ea710, 0x9ea750), // Castelnuovo × Poggibonsi
    (25, 10, 2001, 0x9ea730, 0x9ea744), // Legnano × Cremonese
    (25, 10, 2001, 0x9ea71c, 0x9ea690), // Meda × Valenzana
    (25, 10, 2001, 0x9ea704, 0x9ea6bc), // Montevarchi × Pro Vercelli
    (25, 10, 2001, 0x9ea6b4, 0x9ea6cc), // Novara × Sangiovannese
    (25, 10, 2001, 0x9ea6a8, 0x9ea6fc), // Rondinella × Prato
    (25, 10, 2001, 0x9ea724, 0x9ea6dc), // Viareggio × Pro Sesto
    (2, 11, 2001, 0x9ea738, 0x9ea724), // Alessandria × Viareggio
    (2, 11, 2001, 0x9ea6e8, 0x9ea6b4), // Biellese × Novara
    (2, 11, 2001, 0x9ea730, 0x9ea69c), // Legnano × Pro Patria
    (2, 11, 2001, 0x9ea6f4, 0x9ea710), // Pavia × Castelnuovo
    (2, 11, 2001, 0x9ea750, 0x9ea6a8), // Poggibonsi × Rondinella
    (2, 11, 2001, 0x9ea6fc, 0x9ea704), // Prato × Montevarchi
    (2, 11, 2001, 0x9ea6dc, 0x9ea690), // Pro Sesto × Valenzana
    (2, 11, 2001, 0x9ea6bc, 0x9ea744), // Pro Vercelli × Cremonese
    (2, 11, 2001, 0x9ea6cc, 0x9ea71c), // Sangiovannese × Meda
    (9, 11, 2001, 0x9ea710, 0x9ea730), // Castelnuovo × Legnano
    (9, 11, 2001, 0x9ea744, 0x9ea6e8), // Cremonese × Biellese
    (9, 11, 2001, 0x9ea71c, 0x9ea750), // Meda × Poggibonsi
    (9, 11, 2001, 0x9ea704, 0x9ea6dc), // Montevarchi × Pro Sesto
    (9, 11, 2001, 0x9ea6b4, 0x9ea738), // Novara × Alessandria
    (9, 11, 2001, 0x9ea69c, 0x9ea6f4), // Pro Patria × Pavia
    (9, 11, 2001, 0x9ea6a8, 0x9ea6bc), // Rondinella × Pro Vercelli
    (9, 11, 2001, 0x9ea690, 0x9ea6fc), // Valenzana × Prato
    (9, 11, 2001, 0x9ea724, 0x9ea6cc), // Viareggio × Sangiovannese
    (16, 11, 2001, 0x9ea738, 0x9ea704), // Alessandria × Montevarchi
    (16, 11, 2001, 0x9ea6e8, 0x9ea710), // Biellese × Castelnuovo
    (16, 11, 2001, 0x9ea730, 0x9ea71c), // Legnano × Meda
    (16, 11, 2001, 0x9ea6f4, 0x9ea6b4), // Pavia × Novara
    (16, 11, 2001, 0x9ea750, 0x9ea724), // Poggibonsi × Viareggio
    (16, 11, 2001, 0x9ea6fc, 0x9ea744), // Prato × Cremonese
    (16, 11, 2001, 0x9ea6dc, 0x9ea69c), // Pro Sesto × Pro Patria
    (16, 11, 2001, 0x9ea6bc, 0x9ea690), // Pro Vercelli × Valenzana
    (16, 11, 2001, 0x9ea6cc, 0x9ea6a8), // Sangiovannese × Rondinella
    (23, 11, 2001, 0x9ea710, 0x9ea6bc), // Castelnuovo × Pro Vercelli
    (23, 11, 2001, 0x9ea744, 0x9ea738), // Cremonese × Alessandria
    (23, 11, 2001, 0x9ea71c, 0x9ea6dc), // Meda × Pro Sesto
    (23, 11, 2001, 0x9ea704, 0x9ea6e8), // Montevarchi × Biellese
    (23, 11, 2001, 0x9ea6b4, 0x9ea730), // Novara × Legnano
    (23, 11, 2001, 0x9ea69c, 0x9ea6cc), // Pro Patria × Sangiovannese
    (23, 11, 2001, 0x9ea6a8, 0x9ea6f4), // Rondinella × Pavia
    (23, 11, 2001, 0x9ea690, 0x9ea750), // Valenzana × Poggibonsi
    (23, 11, 2001, 0x9ea724, 0x9ea6fc), // Viareggio × Prato
    (6, 0, 2002, 0x9ea750, 0x9ea744), // Poggibonsi × Cremonese
    (6, 0, 2002, 0x9ea738, 0x9ea730), // Alessandria × Legnano
    (6, 0, 2002, 0x9ea724, 0x9ea71c), // Viareggio × Meda
    (6, 0, 2002, 0x9ea710, 0x9ea704), // Castelnuovo × Montevarchi
    (6, 0, 2002, 0x9ea6fc, 0x9ea6f4), // Prato × Pavia
    (6, 0, 2002, 0x9ea6e8, 0x9ea6dc), // Biellese × Pro Sesto
    (6, 0, 2002, 0x9ea6cc, 0x9ea6bc), // Sangiovannese × Pro Vercelli
    (6, 0, 2002, 0x9ea6b4, 0x9ea6a8), // Novara × Rondinella
    (6, 0, 2002, 0x9ea69c, 0x9ea690), // Pro Patria × Valenzana
    (13, 0, 2002, 0x9ea6bc, 0x9ea738), // Pro Vercelli × Alessandria
    (13, 0, 2002, 0x9ea71c, 0x9ea6e8), // Meda × Biellese
    (13, 0, 2002, 0x9ea6a8, 0x9ea710), // Rondinella × Castelnuovo
    (13, 0, 2002, 0x9ea744, 0x9ea6b4), // Cremonese × Novara
    (13, 0, 2002, 0x9ea730, 0x9ea750), // Legnano × Poggibonsi
    (13, 0, 2002, 0x9ea6dc, 0x9ea6fc), // Pro Sesto × Prato
    (13, 0, 2002, 0x9ea704, 0x9ea69c), // Montevarchi × Pro Patria
    (13, 0, 2002, 0x9ea6f4, 0x9ea6cc), // Pavia × Sangiovannese
    (13, 0, 2002, 0x9ea690, 0x9ea724), // Valenzana × Viareggio
    (20, 0, 2002, 0x9ea69c, 0x9ea710), // Pro Patria × Castelnuovo
    (20, 0, 2002, 0x9ea6dc, 0x9ea744), // Pro Sesto × Cremonese
    (20, 0, 2002, 0x9ea6cc, 0x9ea730), // Sangiovannese × Legnano
    (20, 0, 2002, 0x9ea738, 0x9ea71c), // Alessandria × Meda
    (20, 0, 2002, 0x9ea750, 0x9ea704), // Poggibonsi × Montevarchi
    (20, 0, 2002, 0x9ea6e8, 0x9ea6f4), // Biellese × Pavia
    (20, 0, 2002, 0x9ea6fc, 0x9ea6bc), // Prato × Pro Vercelli
    (20, 0, 2002, 0x9ea724, 0x9ea6a8), // Viareggio × Rondinella
    (20, 0, 2002, 0x9ea6b4, 0x9ea690), // Novara × Valenzana
    (27, 0, 2002, 0x9ea6a8, 0x9ea738), // Rondinella × Alessandria
    (27, 0, 2002, 0x9ea690, 0x9ea6e8), // Valenzana × Biellese
    (27, 0, 2002, 0x9ea710, 0x9ea6b4), // Castelnuovo × Novara
    (27, 0, 2002, 0x9ea6bc, 0x9ea750), // Pro Vercelli × Poggibonsi
    (27, 0, 2002, 0x9ea730, 0x9ea6fc), // Legnano × Prato
    (27, 0, 2002, 0x9ea71c, 0x9ea69c), // Meda × Pro Patria
    (27, 0, 2002, 0x9ea6f4, 0x9ea6dc), // Pavia × Pro Sesto
    (27, 0, 2002, 0x9ea744, 0x9ea6cc), // Cremonese × Sangiovannese
    (27, 0, 2002, 0x9ea704, 0x9ea724), // Montevarchi × Viareggio
    (3, 1, 2002, 0x9ea71c, 0x9ea710), // Meda × Castelnuovo
    (3, 1, 2002, 0x9ea724, 0x9ea744), // Viareggio × Cremonese
    (3, 1, 2002, 0x9ea6e8, 0x9ea730), // Biellese × Legnano
    (3, 1, 2002, 0x9ea6b4, 0x9ea704), // Novara × Montevarchi
    (3, 1, 2002, 0x9ea750, 0x9ea6f4), // Poggibonsi × Pavia
    (3, 1, 2002, 0x9ea6cc, 0x9ea6fc), // Sangiovannese × Prato
    (3, 1, 2002, 0x9ea69c, 0x9ea6bc), // Pro Patria × Pro Vercelli
    (3, 1, 2002, 0x9ea6dc, 0x9ea6a8), // Pro Sesto × Rondinella
    (3, 1, 2002, 0x9ea738, 0x9ea690), // Alessandria × Valenzana
    (10, 1, 2002, 0x9ea710, 0x9ea738), // Castelnuovo × Alessandria
    (10, 1, 2002, 0x9ea6a8, 0x9ea6e8), // Rondinella × Biellese
    (10, 1, 2002, 0x9ea704, 0x9ea71c), // Montevarchi × Meda
    (10, 1, 2002, 0x9ea730, 0x9ea6f4), // Legnano × Pavia
    (10, 1, 2002, 0x9ea6fc, 0x9ea750), // Prato × Poggibonsi
    (10, 1, 2002, 0x9ea744, 0x9ea69c), // Cremonese × Pro Patria
    (10, 1, 2002, 0x9ea6bc, 0x9ea6dc), // Pro Vercelli × Pro Sesto
    (10, 1, 2002, 0x9ea690, 0x9ea6cc), // Valenzana × Sangiovannese
    (10, 1, 2002, 0x9ea6b4, 0x9ea724), // Novara × Viareggio
    (17, 1, 2002, 0x9ea6dc, 0x9ea710), // Pro Sesto × Castelnuovo
    (17, 1, 2002, 0x9ea71c, 0x9ea744), // Meda × Cremonese
    (17, 1, 2002, 0x9ea724, 0x9ea730), // Viareggio × Legnano
    (17, 1, 2002, 0x9ea6a8, 0x9ea704), // Rondinella × Montevarchi
    (17, 1, 2002, 0x9ea69c, 0x9ea6b4), // Pro Patria × Novara
    (17, 1, 2002, 0x9ea6cc, 0x9ea750), // Sangiovannese × Poggibonsi
    (17, 1, 2002, 0x9ea738, 0x9ea6fc), // Alessandria × Prato
    (17, 1, 2002, 0x9ea6e8, 0x9ea6bc), // Biellese × Pro Vercelli
    (17, 1, 2002, 0x9ea6f4, 0x9ea690), // Pavia × Valenzana
    (24, 1, 2002, 0x9ea750, 0x9ea6e8), // Poggibonsi × Biellese
    (24, 1, 2002, 0x9ea6b4, 0x9ea71c), // Novara × Meda
    (24, 1, 2002, 0x9ea6bc, 0x9ea6f4), // Pro Vercelli × Pavia
    (24, 1, 2002, 0x9ea6fc, 0x9ea69c), // Prato × Pro Patria
    (24, 1, 2002, 0x9ea738, 0x9ea6dc), // Alessandria × Pro Sesto
    (24, 1, 2002, 0x9ea730, 0x9ea6a8), // Legnano × Rondinella
    (24, 1, 2002, 0x9ea704, 0x9ea6cc), // Montevarchi × Sangiovannese
    (24, 1, 2002, 0x9ea744, 0x9ea690), // Cremonese × Valenzana
    (24, 1, 2002, 0x9ea710, 0x9ea724), // Castelnuovo × Viareggio
    (3, 2, 2002, 0x9ea6e8, 0x9ea738), // Biellese × Alessandria
    (3, 2, 2002, 0x9ea690, 0x9ea710), // Valenzana × Castelnuovo
    (3, 2, 2002, 0x9ea6a8, 0x9ea744), // Rondinella × Cremonese
    (3, 2, 2002, 0x9ea704, 0x9ea730), // Montevarchi × Legnano
    (3, 2, 2002, 0x9ea6f4, 0x9ea71c), // Pavia × Meda
    (3, 2, 2002, 0x9ea6fc, 0x9ea6b4), // Prato × Novara
    (3, 2, 2002, 0x9ea69c, 0x9ea750), // Pro Patria × Poggibonsi
    (3, 2, 2002, 0x9ea6dc, 0x9ea6cc), // Pro Sesto × Sangiovannese
    (3, 2, 2002, 0x9ea6bc, 0x9ea724), // Pro Vercelli × Viareggio
    (10, 2, 2002, 0x9ea6cc, 0x9ea6e8), // Sangiovannese × Biellese
    (10, 2, 2002, 0x9ea744, 0x9ea704), // Cremonese × Montevarchi
    (10, 2, 2002, 0x9ea724, 0x9ea6f4), // Viareggio × Pavia
    (10, 2, 2002, 0x9ea710, 0x9ea6fc), // Castelnuovo × Prato
    (10, 2, 2002, 0x9ea738, 0x9ea69c), // Alessandria × Pro Patria
    (10, 2, 2002, 0x9ea750, 0x9ea6dc), // Poggibonsi × Pro Sesto
    (10, 2, 2002, 0x9ea6b4, 0x9ea6bc), // Novara × Pro Vercelli
    (10, 2, 2002, 0x9ea71c, 0x9ea6a8), // Meda × Rondinella
    (10, 2, 2002, 0x9ea730, 0x9ea690), // Legnano × Valenzana
    (24, 2, 2002, 0x9ea750, 0x9ea738), // Poggibonsi × Alessandria
    (24, 2, 2002, 0x9ea6cc, 0x9ea710), // Sangiovannese × Castelnuovo
    (24, 2, 2002, 0x9ea6f4, 0x9ea744), // Pavia × Cremonese
    (24, 2, 2002, 0x9ea6bc, 0x9ea730), // Pro Vercelli × Legnano
    (24, 2, 2002, 0x9ea6fc, 0x9ea71c), // Prato × Meda
    (24, 2, 2002, 0x9ea690, 0x9ea704), // Valenzana × Montevarchi
    (24, 2, 2002, 0x9ea6dc, 0x9ea6b4), // Pro Sesto × Novara
    (24, 2, 2002, 0x9ea69c, 0x9ea6a8), // Pro Patria × Rondinella
    (24, 2, 2002, 0x9ea6e8, 0x9ea724), // Biellese × Viareggio
    (30, 2, 2002, 0x9ea710, 0x9ea744), // Castelnuovo × Cremonese
    (30, 2, 2002, 0x9ea704, 0x9ea6f4), // Montevarchi × Pavia
    (30, 2, 2002, 0x9ea6b4, 0x9ea750), // Novara × Poggibonsi
    (30, 2, 2002, 0x9ea6e8, 0x9ea6fc), // Biellese × Prato
    (30, 2, 2002, 0x9ea724, 0x9ea69c), // Viareggio × Pro Patria
    (30, 2, 2002, 0x9ea730, 0x9ea6dc), // Legnano × Pro Sesto
    (30, 2, 2002, 0x9ea71c, 0x9ea6bc), // Meda × Pro Vercelli
    (30, 2, 2002, 0x9ea738, 0x9ea6cc), // Alessandria × Sangiovannese
    (30, 2, 2002, 0x9ea6a8, 0x9ea690), // Rondinella × Valenzana
    (7, 3, 2002, 0x9ea6f4, 0x9ea738), // Pavia × Alessandria
    (7, 3, 2002, 0x9ea69c, 0x9ea6e8), // Pro Patria × Biellese
    (7, 3, 2002, 0x9ea750, 0x9ea710), // Poggibonsi × Castelnuovo
    (7, 3, 2002, 0x9ea744, 0x9ea730), // Cremonese × Legnano
    (7, 3, 2002, 0x9ea690, 0x9ea71c), // Valenzana × Meda
    (7, 3, 2002, 0x9ea6bc, 0x9ea704), // Pro Vercelli × Montevarchi
    (7, 3, 2002, 0x9ea6cc, 0x9ea6b4), // Sangiovannese × Novara
    (7, 3, 2002, 0x9ea6fc, 0x9ea6a8), // Prato × Rondinella
    (7, 3, 2002, 0x9ea6dc, 0x9ea724), // Pro Sesto × Viareggio
    (14, 3, 2002, 0x9ea724, 0x9ea738), // Viareggio × Alessandria
    (14, 3, 2002, 0x9ea6b4, 0x9ea6e8), // Novara × Biellese
    (14, 3, 2002, 0x9ea69c, 0x9ea730), // Pro Patria × Legnano
    (14, 3, 2002, 0x9ea710, 0x9ea6f4), // Castelnuovo × Pavia
    (14, 3, 2002, 0x9ea6a8, 0x9ea750), // Rondinella × Poggibonsi
    (14, 3, 2002, 0x9ea704, 0x9ea6fc), // Montevarchi × Prato
    (14, 3, 2002, 0x9ea690, 0x9ea6dc), // Valenzana × Pro Sesto
    (14, 3, 2002, 0x9ea744, 0x9ea6bc), // Cremonese × Pro Vercelli
    (14, 3, 2002, 0x9ea71c, 0x9ea6cc), // Meda × Sangiovannese
    (21, 3, 2002, 0x9ea730, 0x9ea710), // Legnano × Castelnuovo
    (21, 3, 2002, 0x9ea6e8, 0x9ea744), // Biellese × Cremonese
    (21, 3, 2002, 0x9ea750, 0x9ea71c), // Poggibonsi × Meda
    (21, 3, 2002, 0x9ea6dc, 0x9ea704), // Pro Sesto × Montevarchi
    (21, 3, 2002, 0x9ea738, 0x9ea6b4), // Alessandria × Novara
    (21, 3, 2002, 0x9ea6f4, 0x9ea69c), // Pavia × Pro Patria
    (21, 3, 2002, 0x9ea6bc, 0x9ea6a8), // Pro Vercelli × Rondinella
    (21, 3, 2002, 0x9ea6fc, 0x9ea690), // Prato × Valenzana
    (21, 3, 2002, 0x9ea6cc, 0x9ea724), // Sangiovannese × Viareggio
    (28, 3, 2002, 0x9ea704, 0x9ea738), // Montevarchi × Alessandria
    (28, 3, 2002, 0x9ea710, 0x9ea6e8), // Castelnuovo × Biellese
    (28, 3, 2002, 0x9ea71c, 0x9ea730), // Meda × Legnano
    (28, 3, 2002, 0x9ea6b4, 0x9ea6f4), // Novara × Pavia
    (28, 3, 2002, 0x9ea724, 0x9ea750), // Viareggio × Poggibonsi
    (28, 3, 2002, 0x9ea744, 0x9ea6fc), // Cremonese × Prato
    (28, 3, 2002, 0x9ea69c, 0x9ea6dc), // Pro Patria × Pro Sesto
    (28, 3, 2002, 0x9ea690, 0x9ea6bc), // Valenzana × Pro Vercelli
    (28, 3, 2002, 0x9ea6a8, 0x9ea6cc), // Rondinella × Sangiovannese
    (5, 4, 2002, 0x9ea6bc, 0x9ea710), // Pro Vercelli × Castelnuovo
    (5, 4, 2002, 0x9ea738, 0x9ea744), // Alessandria × Cremonese
    (5, 4, 2002, 0x9ea6dc, 0x9ea71c), // Pro Sesto × Meda
    (5, 4, 2002, 0x9ea6e8, 0x9ea704), // Biellese × Montevarchi
    (5, 4, 2002, 0x9ea730, 0x9ea6b4), // Legnano × Novara
    (5, 4, 2002, 0x9ea6cc, 0x9ea69c), // Sangiovannese × Pro Patria
    (5, 4, 2002, 0x9ea6f4, 0x9ea6a8), // Pavia × Rondinella
    (5, 4, 2002, 0x9ea750, 0x9ea690), // Poggibonsi × Valenzana
    (5, 4, 2002, 0x9ea6fc, 0x9ea724), // Prato × Viareggio
];

/// The 2001/02 Serie C2/A: the 306 matches (34 rounds of 9) fixed by club name (`set_up_fixed_fixtures_body`, Serie
/// C1's body, checked instruction by instruction: only the table, the source file and the box line 0x3ce change); with a
/// name missing from the database the list is dropped and the league draws its schedule.
/// Original: `ItaSerC2a::set_up_fixed_fixtures` (0x6545c0, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_set_up_fixed_fixtures(this: usize) {
    covered!("cx11_11 serie c2 fixed calendar");
    set_up_fixed_fixtures_body(League(this), &SERIE_C2A_FIXTURES, 0x9ea654, 0x3ce)
}

/// Comparer for `ita_ser_c2a_set_up_fixed_fixtures`: only the original runs (it allocates); afterwards the list is either dropped or has every
/// fixture with both clubs found (`fra_first_compare_fixed_fixtures`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_set_up_fixed_fixtures(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    fra_first_compare_fixed_fixtures("ita_ser_c2a_set_up_fixed_fixtures", League(this), || o(this))
}
