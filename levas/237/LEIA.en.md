# Batch 237 — the name from strings, the job name and the language name; the national team name, the player attribute name and the positions text; the region name and the continent adjective

**Routines:**
- `nome_pelos_textos()` = 0x53e2e0 (Database.cpp, cdecl): the person name by format (1..7) from the first, second and common names: "%s %s", "%c.%s", "%s, %s", "%s, %c", one of them or the initials;
- `nome_do_cargo()` = 0x53ad00 (Database.cpp, cdecl): the translated job name (0..0x11), short or full, lower or upper case, in [0xae19b4]; the gender with language 4;
- `nome_do_idioma()` = 0x53cee0 (Database.cpp, cdecl): the translated language name (1..64, else "Unknown") in [0xae21d4];
- `nome_da_selecao()` = 0x542100 (Database.cpp, cdecl): the national team club name by style (1..4): the name, "Reserves", "B"/"'B'", "U21"/"Under 21"/"Under 21's"/"Under 21 Squad" (or 23) and "Squad"/"Main Squad";
- `nome_do_atributo_do_jogador()` = 0x5428a0 (Database.cpp, cdecl): the translated name of the 31 player attributes, short or full; returns 1;
- `texto_das_posicoes()` = 0x53ee50 (Database.cpp, cdecl): the positions text from the ratings (15 to 10) and the sides (15), short ("D/DM RC"), full or as letters;
- `nome_da_regiao()` = 0x616f20 (index.cpp, cdecl): the region name (1..14): from the language 4, 5 and 9 tables with the adjective form, else the translated adjective or noun by form;
- `adjetivo_do_continente()` = 0x6177f0 (index.cpp, cdecl): the continent adjective: the record one (+0x23) or, in languages 9, 5 and 4, the text by continent and adjective form;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (8):** 1 caught. `dbu_cargo` escaped: screen/news text that does not reach the fingerprint (media, transfers, person, club and contract screens); also escaped the real game comparing (0 divergences). `dbu_idioma` escaped: screen/news text that does not reach the fingerprint (search and person screen); also escaped the real game comparing (0 divergences). `dbu_selecao` escaped: screen/news text that does not reach the fingerprint (national and youth team screens, club screen, line-up, hall of fame); also escaped the real game comparing (0 divergences). `dbu_atributo` escaped: screen/news text that does not reach the fingerprint (player, club, search and training screens); also escaped the real game comparing (0 divergences). `dbu_posicoes` escaped: screen/news text that does not reach the fingerprint (person screen with a wide player, search, tactics, training); also escaped the real game comparing (0 divergences). `dbu_regiao` escaped: screen/news text that does not reach the fingerprint (region filter of the player search); also escaped the real game comparing (0 divergences). `dbu_continente` escaped: screen/news text that does not reach the fingerprint (search screens, menu bar, matches of the day); also escaped the real game comparing (0 divergences). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 236's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 237`

**Fields touched:**
- [0xae19b4]: job name text; writes (nome_do_cargo); [0xae21d4]: language name text (100 bytes); writes (nome_do_idioma)
- TClub +0 id, +4 long name, +0x37 and +0x52 gender, +0x38 name, +0x53 nation; [0xae2350] nations, [0xae235c] clubs, [0xb74318] +0x36 under-23 by nation; reads (nome_da_selecao)
- player +0xf..+0x1a position ratings; reads; +0x13 midfielder: writes 15 (texto_das_posicoes); [0xde1f64] global text: writes
- [0x9d0518], [0x9d06d4], [0x9d05f8]: region name tables by language; [0x9eb2dc] language; reads (nome_da_regiao)
- continent +0 id, +0x23 adjective; [0x9cfa08..0x9cfa18] continent keys; reads (adjetivo_do_continente)

**Of interest to:** SGE, CMRTE
