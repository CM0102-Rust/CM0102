# Batch 219 — the saved game: the file box filters (saved game, quick start and version); the saved game: the FIFA transfer rules (save and load) and the save summary; the saved game: the database links (save and load) and freeing the game data

**Routines:**
- `filtro_do_jogo_salvo()` = 0x838e10 (Setup.cpp, cdecl): file box filter for saved games: 1 if the container has id 0x16 (.sav), else 2 (ax only); a null path gives the error box;
- `filtro_do_inicio_rapido()` = 0x839290 (Setup.cpp, cdecl): file box filter for quick starts: 1 if the container has id 0x17 (.qst) and the saved version is the exe one, else 2 (ax only);
- `versao_do_jogo_salvo_valida()` = 0x838eb0 (Setup.cpp, cdecl): opens the container general.dat, reads the version and the summary (0xf70 or 0xf74 bytes) and returns 1 if the summary version text is the exe one; on success it does not close the item (as the original);
- `gravar_regras_fifa()` = 0x8395b0 (Setup.cpp, cdecl): writes the last saved game item, fifa_transfer_rules.zip: 2 bytes ([0xae233c] and the rule in force [0xa83608]); errors give the "Error" box;
- `carregar_regras_fifa()` = 0x839760 (Setup.cpp, cdecl): reads the 2 bytes of fifa_transfer_rules.zip; if the first is 1, the rule in force becomes 5;
- `montar_resumo_do_save()` = 0x591f80 (file_screens.cpp, cdecl): builds the general.dat summary: the active nations (two groups), the human managers with club and nation, the game date, the version, the play time and time();
- `gravar_banco_no_save()` = 0x524d30 (Database.cpp, cdecl): save_game_data step 1: save_database(save, 1, a4, type), the database tables into the container;
- `carregar_banco_do_save()` = 0x524d10 (Database.cpp, cdecl): load_game_data step 2: load_database(save, shared, 1), the database read from the container;
- `liberar_dados_do_jogo()` = 0x82ff40 (Setup.cpp, cdecl): releases the screens, destroys the subsystems held in globals (destructor + delete, free, virtual destructor) in the original order, deletes the *.tmp files, frees the database tables, resets the game date and clears three flags; returns 1;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL fora dos campos ignorados.
- **Mutations (4):** 3 caught. `sv2_banco_a4` escaped: the 0x524d30 copy is switched off (DESLIGADAS: the save_database it calls writes 408 bytes of unset locals into index.dat and, called from the copy on the private stack, the garbage changes), so the mutation does not run. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 218's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 219`

**Fields touched:**
- container (file) +4: archive id (0x16 .sav, 0x17 .qst); value; reads, through 0x9331c0 (filtro_do_jogo_salvo, filtro_do_inicio_rapido)
- general.dat +0: save version; value (|v| < 0x3e027 = old 0xf70-byte summary); reads (versao_do_jogo_salvo_valida)
- general.dat +4: save summary (0xf70 or 0xf74 bytes; the build_save_game_summary one); bytes; reads (versao_do_jogo_salvo_valida)
- save summary +0x7d0: version text of the exe that saved; text; reads and compares with 0x90d130 (versao_do_jogo_salvo_valida)
- [0xae24d0], [0xae28bc], [0xae26c4] error box texts; text; writes (versao_do_jogo_salvo_valida, on errors)
- [0xae233c] first FIFA rules byte (1 = rule 5 on load); value; reads (gravar_regras_fifa) and writes (carregar_regras_fifa)
- [0xa83608] FIFA transfer rule in force (10 in the exe); value; reads (gravar_regras_fifa) and writes, 5 if [0xae233c] = 1 (carregar_regras_fifa)
- fifa_transfer_rules.zip (saved game item): the 2 bytes above; bytes; writes (gravar_regras_fifa) and reads (carregar_regras_fifa)
- league table [0xb63d60] (0x23 items of 0x48) +0: nation; pointer; reads (montar_resumo_do_save)
- nation +0x11c: bits 2 and 1 (the two summary nation groups); value; reads (montar_resumo_do_save)
- nation/club +0x38: name; text; reads (montar_resumo_do_save)
- staff (0x6e) +0x5f: human manager (1); value; reads (montar_resumo_do_save)
- staff +0x39: club; pointer; reads (montar_resumo_do_save)
- staff +0x24: national team (nation); pointer; reads (montar_resumo_do_save)
- [0xae2364] staff count, [0xae23bc] staff table; value, pointer; reads (montar_resumo_do_save)
- [0xae2c90] game date; value; reads (montar_resumo_do_save)
- [0xdd7140] accumulated play time, [0xdd7144] last time(); value; reads and writes (montar_resumo_do_save)
- save summary +0 / +0x3e8: nations (bit 2 / bit 1); text; writes (montar_resumo_do_save)
- save summary +0x7d0: exe version; text; writes (montar_resumo_do_save)
- save summary +0x7e4 + 0x78 times i (i < 16): human manager name; text; writes (montar_resumo_do_save)
- save summary +0x820 + 0x78 times i: manager "club / nation"; text; writes, human only (montar_resumo_do_save)
- save summary +0xf64: game date; value; writes (montar_resumo_do_save)
- save summary +0xf6c / +0xf70: play time and time(); value (clock); writes (montar_resumo_do_save)
- game subsystems; pointer; reads, destroys and writes 0 (liberar_dados_do_jogo)
- [0xae2c74] (friendlies), [0xb74434] objects with a virtual destructor; pointer; reads, destroys and writes 0 (liberar_dados_do_jogo)
- [0xadadfc] (and [0xadadf8]), [0xae2d0c], [0xae2d14], [0xae2d1c], [0xae2d20], [0xae2d24], [0xae2338], [0xdd7ef0] loose arrays; pointer; reads, frees and writes 0 (liberar_dados_do_jogo)
- [0xae2c90] game date; value; writes (0x54a130 with 1, 0, 2001, -1) (liberar_dados_do_jogo)
- [0xae2c88] (byte), [0xdd711c], [0xdd7130] game flags; value; writes 0 (liberar_dados_do_jogo)

**Of interest to:** SGE, CMRTE
