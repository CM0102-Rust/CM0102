# Batch 265 — saving the finances and the squad manager: writing and reading finance.dat and writing squad_manager.dat; the saved game temporary files: the UEFA and CONMEBOL seedings and the host countries; saving the histories: the club history, the officials statistics and the simulated statistics; saving the fixture calendar: writing and reading the club dates and writing the leg checks

**Routines:**
- `gravar_financas()` = 0x59b3b0 (fin_prm.cpp, thiscall): writes the finances of every club into the finance.dat item (0x167 bytes × [0xae235c] from [this]); no container, nothing; on error the "Error" box (finance.CPP 0x115e, 0x1166) without closing the item;
- `carregar_financas()` = 0x59b2a0 (fin_prm.cpp, thiscall): reads the finances of every club from the finance.dat item into [this] and returns 1; no container 0; on error the "Error" box (0x1137, 0x113f) and 0 (the constructor rebuilds the finances from scratch);
- `gravar_gerente_de_elencos()` = 0x859cc0 (squad_manager.cpp, thiscall): writes the u16 at +0, widened to 4 bytes, into the squad_manager.dat item and returns 1; on error the "Error" box (squad_manager.cpp 0x68, 0x6f) and 0;
- `gravar_sementes_da_uefa()` = 0x905510 (uefa_seeding.cpp, thiscall): writes CM3_TEMP\uefa_seeding.tmp: the country count (+0x438), 0x12 bytes per country from +0, +0x439 (u16), as many u32 as +0x43f from [+0x43b] and 50 records of 0xc from +0x440; on error the "Error" box without closing the file;
- `gravar_sementes_da_conmebol()` = 0x4c6b70 (conmebol_seeding.cpp, thiscall): writes CM3_TEMP\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0, +0x6f (u16) and +0x71 (u32); on error the "Error" box without closing the file;
- `gravar_paises_anfitrioes()` = 0x5fb320 (host_country.cpp, thiscall): writes CM3_TEMP\HostCountry.tmp: the count (+4, u16) and 0x22 bytes per host from [+0], without checking the writes; on open error the "Error" box;
- `gravar_historico_dos_clubes()` = 0x443f50 (club_history.cpp, thiscall): writes the count (+0xc) and the 0x10-byte records from [+8] into the "club history.dat" item and returns 1; no container the "Error" box and 0; on open or write error the box and the game stops;
- `gravar_estatisticas_dos_arbitros()` = 0x79bb80 (officials_manager.cpp, thiscall): writes [0xae2390] officials (0x11 bytes from [+0]), [0xae2350] nations (0x102 bytes from [+0xc]) and one byte per official from [+8] into the off.dat item; returns 1 in the low byte; with no officials or on error the "Error" box, closes the item and 0;
- `gravar_estatisticas_simuladas()` = 0x84c680 (simulated_stats.cpp, thiscall): writes the count (+0) and the 0x44-byte records from [+4] into the simulated_stats.dat item; on error the "Error" box without closing the item;
- `gravar_datas_dos_clubes()` = 0x5af040 (fix_man.cpp, cdecl): writes the [0xae2a5c] table, one word per club ([0xae235c]), into CM3_TEMP\club_dates.tmp; without the table or on a short write, the "Error" box (fix_man.cpp 0x1011, 0x101b);
- `carregar_datas_dos_clubes()` = 0x5aee90 (fix_man.cpp, cdecl): allocates the [0xae2a5c] table (one word per club) and reads it from CM3_TEMP\club_dates.tmp, or zeroes it without the file; already allocated, out of memory or on a short read, the "Error" box (0xfe4, 0xfec, 0xff6);
- `gravar_conferencia_das_pernas()` = 0x5af4a0 (fix_man.cpp, cdecl): with the [0xae2a48] list not empty, writes the count (2 bytes) and the 15-byte items into CM3_TEMP\check_fixture_legs.tmp; on a short item write, the "Error" box (0x10e0);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL fora dos campos ignorados; carga EQUAL (parou em 2001-239, 729127830 sorteios, impressao 57d1b826c3d85ea4).
- **Mutations (12):** 12 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (0bb74e04).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6299f556, original da4329bf (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 264's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 265`

**Fields touched:**
- FinanceTable +0: the club finances (0x167 bytes per club, [0xae235c] clubs, nat_club included); pointer; reads and writes into the save (gravar_financas), writes from the save (carregar_financas)
- SQUAD_MANAGER +0: the squad manager value (u16; 0 in a new game); value; reads and writes into the save as 4 bytes (gravar_gerente_de_elencos)
- [0xae235c] club count; value; reads (gravar_financas, carregar_financas)
- UefaSeeding +0: countries (0x12 bytes each); +0x438 count (i8); +0x439 (u16); +0x43b pointer to the u32s and +0x43f count (i8); +0x440 50 records of 0xc; bytes; reads and writes into uefa_seeding.tmp (gravar_sementes_da_uefa)
- ConmebolSeeding +0: countries (0xa bytes each); +0x6e count (i8); +0x6f (u16); +0x71 (u32); bytes; reads and writes into conmebol_seeding.tmp (gravar_sementes_da_conmebol)
- HostCountry +0: hosts (0x22 bytes each); pointer; +4 count (i16); reads and writes into HostCountry.tmp (gravar_paises_anfitrioes)
- Club_History +8: the club league and cup history records (0x10 bytes each); pointer; +0xc count (u32); reads and writes into the save (gravar_historico_dos_clubes)
- OfficialsManager +0: per-official stats (0x11 bytes); +0xc per nation (0x102 bytes); +8 one byte per official; pointers; [0xae2390] officials and [0xae2350] nations; reads and writes into off.dat (gravar_estatisticas_dos_arbitros)
- SimulatedStats +0: count (u32); +4 the 0x44-byte records; pointer; reads and writes into the save (gravar_estatisticas_simuladas)
- [0xae2a5c] the club dates (one word per club: bit 0 the first year, the rest the last); pointer; reads and writes into the save (gravar_datas_dos_clubes), allocates and writes from the load (carregar_datas_dos_clubes)
- [0xae2a48] the fixture leg check list (LLIST: +0 count, +2 head, +0xa cursor; 15-byte items); reads and writes the cursor (gravar_conferencia_das_pernas)
- [0xae235c] club count; value; reads (gravar_datas_dos_clubes, carregar_datas_dos_clubes)

**Of interest to:** SGE, CMRTE
