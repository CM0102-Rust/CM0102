# Batch 226 — the adjective form per language, the person's coaching attribute and the rating with the reduced ability; the first name by weight, the competition's colours and whether the club has room in the job; adding a person to a club

**Routines:**
- `forma_do_adjetivo()` = 0x545a60 (Database.cpp, cdecl): the adjective form (0..4) for index 1..35 in the inflecting languages 3..10 (jump tables); any other language or index: 0xff;
- `atributo_do_nao_jogador()` = 0x543f30 (Database.cpp, cdecl): the person's coaching attribute 1..12: 0x543fd0 with their non-player record (+0x69);
- `nota_relativa_com_habilidade_reduzida()` = 0x5442f0 (Database.cpp, cdecl): the 1-to-20 rating of 2 × value + current ability × 0.1 (0x544380 with the ability scaled down);
- `primeiro_nome_pelo_peso()` = 0x539b90 (Database.cpp, cdecl): a first name: nation with more than 15 names, the heaviest of 3 draws in its range; otherwise a draw over the whole table;
- `cores_da_competicao()` = 0x53b7c0 (Database.cpp, cdecl): the competition's colours (foreground and background) in pixels, from the +0x5d record or the competition itself;
- `clube_tem_vaga_para_cargo()` = 0x53b9a0 (Database.cpp, cdecl): whether the club has room for the person in job 1..15 (an empty slot or one already theirs; 12, 13 and 15 also in the squad);
- `incluir_pessoa_no_clube()` = 0x53bd10 (Database.cpp, cdecl): puts the person in the job slot at the club (jobs 1..15; a player above 85 without a human manager moves from 6/8 to 13/15), copies the date and writes club and job into the person;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `dq13_reduzida` escaped: the score with reduced ability (0x5442f0) decides nothing in the career up to 2002-06-01, and in the real game comparing (to 2002-01-01) there was no call where ×0.2 and ×0.1 gave different scores after clamping to 1..20. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 225's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 226`

**Fields touched:**
- [0x545b2c..0x545db4] adjective form jump tables; value; reads (forma_do_adjetivo)
- staff +0x69: non-player record; pointer; reads (atributo_do_nao_jogador)
- player +0x05: current ability; value; reads (nota_relativa_com_habilidade_reduzida)
- [0x967880] 0.1, [0x967888] 0.05, [0x9688b0] 10, [0x968a80] 1/3, [0x968968] 0.5, [0x967890] 1, [0x968ea8] 20; value; reads (nota_relativa_com_habilidade_reduzida)
- [0xae2338] per-nation name index (+0 first row, +0x18 first-name count); value; reads (primeiro_nome_pelo_peso)
- [0xae23f0] first-name table (60 bytes; +0x3b weight); pointer; reads (primeiro_nome_pelo_peso)
- [0xae2398] first-name total; value; reads (primeiro_nome_pelo_peso)
- [0xad6c30] CRT rand() state; value; reads and writes (primeiro_nome_pelo_peso)
- competition +0x5d: record holding the colours (+0x90, +0x94); pointer; reads (cores_da_competicao)
- competition +0x61, +0x65: colours; pointer; reads (cores_da_competicao)
- colour +0x37, +0x38, +0x39: red, green, blue; value; reads (cores_da_competicao)
- [0xae3184], [0xaebe04] default colours; value; reads (cores_da_competicao)
- club +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): job and squad slots; pointer; reads (clube_tem_vaga_para_cargo)
- club +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): job and squad slots; pointer; reads and writes (incluir_pessoa_no_clube)
- staff +0x24, +0x28: national team and job there; pointer, value; writes (incluir_pessoa_no_clube)
- staff +0x29, +0x3e: date joined national team / club (8 bytes); value; writes (incluir_pessoa_no_clube)
- staff +0x39, +0x3d: club and job there; pointer, value; writes (incluir_pessoa_no_clube)
- staff +0x61: player record; pointer; reads (incluir_pessoa_no_clube)
- player +0x05: current ability (> 85 raises the job); value; reads (incluir_pessoa_no_clube)
- [0xae2364] staff total (id ≥ total − 16: job 5 only); value; reads (incluir_pessoa_no_clube)
- [0xae235c], [0xae2350] club and nation totals (national team range); value; reads (incluir_pessoa_no_clube)

**Of interest to:** CMRTE, CMScout, GPF2, SGE
