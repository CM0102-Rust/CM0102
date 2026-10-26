# Batch 235 — adding to the preferences: the favoured and the disliked person of a club and the favoured person of a person; the disliked club of a person, the fading of personal relationships and the list of disliked staff; the natural position mask, how many of the squad have it and the picked favoured team mate; the nation rules: club outside Argentina, outside Argentina, France and Croatia, and Melanesian nation; the Polynesian nation, the coaching job name and the years between two dates as text; the club order by short name and the staff order by full name, and whether a person matches the names; the club order by latitude and by longitude of the stadium city

**Routines:**
- `clube_acrescentar_pessoa_favorita()` = 0x547870 (Database.cpp, cdecl): puts the person among the three favoured staff of the club (+0x9b..+0xa3): already favoured, nothing; disliked (+0xa7..+0xaf), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3);
- `clube_acrescentar_pessoa_rejeitada()` = 0x547940 (Database.cpp, cdecl): puts the person among the three disliked staff of the club (+0xa7..+0xaf): already disliked, nothing; favoured (+0x9b..+0xa3), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3);
- `acrescentar_pessoa_favorita()` = 0x547a10 (Database.cpp, cdecl): puts the other person among the three favoured staff of the person (preferences +0x1c..+0x24): already favoured, nothing; disliked (+0x28..+0x30), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3);
- `acrescentar_clube_rejeitado()` = 0x547e70 (Database.cpp, cdecl): puts the club among the three disliked clubs of the person (preferences +0x10..+0x18): the club already dislikes the person (+0xa7..+0xaf), nothing; favoured (+4..+0xc), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3);
- `esmaecer_relacoes()` = 0x547f90 (Database.cpp, cdecl): each favoured person (preferences +0x1c..+0x24) leaves when a random draw of (+0x59) + 15 gives 0; each disliked one (+0x28..+0x30), with 35 − (+0x5d);
- `pessoas_rejeitadas()` = 0x5480f0 (Database.cpp, cdecl): copies the present disliked staff (preferences +0x28..+0x30) to the output, counting in the byte of the third argument (always cleared);
- `mascara_da_posicao_natural()` = 0x5481a0 (Database.cpp, cdecl): the mask of the positions where the player is natural (20): goalkeeper 1; full backs, centre backs, midfielders, forwards (bits 2..0xc00); with none, the first position at 15 or more; otherwise 0x10;
- `contar_elenco_com_mascara()` = 0x548150 (Database.cpp, cdecl): how many of the 50 in the squad (+0xd7) with a player record have the given natural position mask;
- `escolher_companheiro_favorito()` = 0x548da0 (Database.cpp, cdecl): the squad mate the person picks as favoured: filters by the person (age, +0x57, +0x59) and by the mate (disliked, position, contract, reputation, 0x8cca10), score by reputation, the 0x7c1e30 average, already favoured, ids, contract level and nationality; the highest score;
- `clube_fora_da_argentina()` = 0x5487b0 (Database.cpp, cdecl): 1 when the club nation (+0x53) is not Argentina (key 0x9cf218); null club or nation: 0; the rest of eax comes from the id;
- `clube_fora_de_argentina_franca_croacia()` = 0x5487d0 (Database.cpp, cdecl): 1 when the club nation is not Argentina, France or Croatia (keys 0x9cf218, 0x9cf300, 0x9cf2ac); null: 0;
- `nacao_da_melanesia()` = 0x53d900 (Database.cpp, cdecl): 1 when the nation is one of the five Melanesian ones (keys 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500);
- `nacao_da_polinesia()` = 0x53d940 (Database.cpp, cdecl): 1 when the nation is one of the five Polynesian ones (keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514);
- `nome_do_cargo_de_treino()` = 0x53ac40 (Database.cpp, cdecl): the job name (5/12 manager, 6/13 assistant, 8/15 coach; others: empty) translated into 0xae1980; with language 4 writes the gender (4 or 1) into the second argument;
- `anos_entre_datas_em_texto()` = 0x540340 (Database.cpp, cdecl): the full years between the dates (end year minus start year, minus 1 when the start day is later) as "%d" text of an unsigned word; returns 1;
- `ordem_dos_clubes_pelo_nome_curto()` = 0x540080 (Database.cpp, cdecl): qsort comparison of clubs by short name (+0x38), case-insensitive (0x90c1d0); null club: empty text;
- `ordem_das_pessoas_pelo_nome_completo()` = 0x540270 (Database.cpp, cdecl): qsort comparison of staff by style-1 name (0x53dba0), case-insensitive; null person: empty text;
- `pessoa_confere_com_os_nomes()` = 0x53e990 (Database.cpp, cdecl): 1 when the person (with a player or non-player record) has the requested first, second and common name ids (on the non-null ones) and, with a date, the same birth date;
- `ordem_dos_clubes_pela_latitude()` = 0x53fd40 (Database.cpp, cdecl): qsort comparison of clubs by the latitude (+0x23, f64) of the city (+0x38) of the stadium (+0x69), lowest first; no city: 0;
- `ordem_dos_clubes_pela_longitude()` = 0x53fe50 (Database.cpp, cdecl): the same comparison by longitude (+0x2b, f64);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (20):** 13 caught. `dbr_fora_da_argentina` escaped: only matters in discipline, managers and news of Argentine clubs, absent from the career leagues; also escaped the real game comparing (0 divergences). `dbr_fora_de_arg_fra_cro` escaped: only changes something for French clubs (ban appeal), absent from the career leagues; also escaped the real game comparing (0 divergences). `dbr_nome_do_cargo` escaped: the job text only appears in retirement news and regeneration, with no effect on the state; also escaped the real game comparing (0 divergences). `dbr_anos_em_texto` escaped: only in the records screens; also escaped the real game comparing (0 divergences). `dbr_pessoas_pelo_nome_completo` escaped: transfer manager sort whose result did not change the career to 2002-06-01; also escaped the real game comparing (0 divergences). `dbr_confere_nomes` escaped: the person search by names had no call where the surname decided; also escaped the real game comparing (0 divergences). `dbr_longitude` escaped: the longitude sort is only used in the cup draws of Belgium, Netherlands, Italy (C), Spain and Turkey, outside the English career; also escaped the real game comparing (0 divergences). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 234's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 235`

**Fields touched:**
- TClub +0x9b..+0xa3 favoured staff, +0xa7..+0xaf disliked staff; reads and writes (clube_acrescentar_pessoa_favorita, clube_acrescentar_pessoa_rejeitada)
- preferences (TStaff +0x65) +0x1c..+0x24 favoured staff, +0x28..+0x30 disliked staff; reads and writes (acrescentar_pessoa_favorita)
- preferences (TStaff +0x65) +4..+0xc favoured clubs, +0x10..+0x18 disliked clubs; TClub +0xa7..+0xaf disliked staff; reads and writes (acrescentar_clube_rejeitado)
- preferences +0x1c..+0x30 favoured and disliked staff; TStaff +0x59, +0x5d; reads and writes (esmaecer_relacoes), reads (pessoas_rejeitadas)
- player (TStaff +0x61) +0xf..+0x19 positions, +5, +0xb reputation, +0x21; reads (mascara_da_posicao_natural, escolher_companheiro_favorito)
- TClub +0xd7 squad (50), +0x53 nation, +0x80 reputation; TStaff +0x18, +0x1a nation, +0x3e/+0x42 birth date, +0x57, +0x59, +0x5a, +0x5b; preferences +0x1c..+0x30; contract +0x45, +0x4f; [0xae2364], [0xb74350] +0x10/+0x18; reads (contar_elenco_com_mascara, escolher_companheiro_favorito)
- TClub +0x53: nation; pointer; reads (clube_fora_da_argentina, clube_fora_de_argentina_franca_croacia)
- TNation +0: id; integer; reads (all three). Keys 0x9cf218, 0x9cf300, 0x9cf2ac (Argentina, France, Croatia), 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (Melanesia); reads
- TNation +0: id; integer; reads (nacao_da_polinesia). Keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (Polynesia); reads
- 0xae1980: static text of the job name; writes (nome_do_cargo_de_treino)
- date +0 day of year, +2 year (words); reads (anos_entre_datas_em_texto)
- TClub +0x38: short name; text; reads (ordem_dos_clubes_pelo_nome_curto)
- TStaff +4/+8/+0xc: first, second, common name (name record +0x33 id); +0x10/+0x12 birth (day, year); +0x61/+0x69 player and non-player; reads (pessoa_confere_com_os_nomes)
- TClub +0x69: stadium; pointer; stadium +0x38 city; city +0x23 latitude, +0x2b longitude (f64); reads (both)

**Of interest to:** SGE, CMRTE
