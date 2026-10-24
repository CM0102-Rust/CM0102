# Batch 227 — the fixed club and nation keys by name and the club key remap after the database trim; the fixed club competition and staff competition/award keys by name

**Routines:**
- `casar_chave_de_clube()` = 0x6146b0 (index.cpp, cdecl): matches the database club name with the 313 fixed club keys (372 names in 4 chains, _stricmp): the first hit of each chain stores the row in the key global;
- `remapear_chave_de_clube()` = 0x60ba80 (index.cpp, cdecl): after the club table trim, the first club key holding the old id gets the new one (returns 1; none: 0);
- `casar_chave_de_nacao()` = 0x60d810 (index.cpp, cdecl): matches the database nation name with the 209 fixed nation keys (217 names in 2 chains, _stricmp);
- `casar_chave_de_competicao_de_clube()` = 0x60efd0 (index.cpp, cdecl): matches the club competition long name with the 301 fixed keys (351 names in 3 chains, _stricmp; the two Croatian divisions also store 0x9cf8a4);
- `casar_chave_de_competicao_de_pessoa()` = 0x6115e0 (index.cpp, cdecl): matches the staff competition or award name with the 417 fixed keys (442 names in 5 chains, _stricmp);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856662 bytes).
- **Mutations (6):** 4 caught. `ix2_comp_pessoa` escaped: the person competition keys (0x9cfa20..0x9d00a0) are exe globals, not written into the .qst, and the quick start does not read them before saving; the save proof (which evaluates the save batch mutations) does not see them. `ix2_devolucao` escaped: the caller (set_key_ids 0x601a40) ignores the return value of the key matching routines (the result of the last _stricmp). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 226's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 227`

**Fields touched:**
- [0x9d00a4..0x9d0524] and [0xb63c9c..0xb63d00]: the 313 club keys (real names in the CHAVES_DE_CLUBE table of codigo.rs: MLS All Stars East/West, River Plate, Vélez Sarsfield, Everton, Tottenham Hotspur, Chelsea, Valencia C.F., FC St. Pauli, Arminia Bielefeld...; different spellings map to the same key); id (club table row); writes (casar_chave_de_clube), reads and writes (remapear_chave_de_clube)
- [0x9cf200..0x9cf544]: the 209 nation keys (0x9cf200 Afghanistan, 0x9cf204 Albania...; names in CHAVES_DE_NACAO, 217 with the spellings); id (nation table row); writes (casar_chave_de_nacao)
- club/nation +name: the name the database gives (argument of the match routines); text; reads (through _stricmp 0x951c50, case-insensitive; with the locale at [0xde3040+8])
- [0x9cf548..0x9cfa04]: the 301 club competition keys (real names in CHAVES_DE_COMPETICAO_DE_CLUBE of codigo.rs: French Lower Division, German First Division, World Club Cup, UEFA Champions League, turkish premier division...; croatian first division and croatian a1 division store 0x9cf87c and 0x9cf8a4); id (club competition table row); writes (casar_chave_de_competicao_de_clube)
- [0x9cfa20..0x9d00a0]: the 417 staff competition and award keys (names in CHAVES_DE_COMPETICAO_DE_PESSOA: FIFA World Player of the Year, European Footballer of the Year, Italian Serie A Team of the Year, Brazilian 1st Division Top Goalscorer...); id (staff competition table row); writes (casar_chave_de_competicao_de_pessoa)
- competition +long name / staff competition +name: the name the database gives (argument); text; reads (through _stricmp 0x951c50, case-insensitive)

**Of interest to:** CMRTE, SGE
