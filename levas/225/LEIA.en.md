# Batch 225 — the club's squad count and squad list and whether the club can be managed; the attribute vectors back into the non-player and the player and the rebalancing of the non-player's attributes; the colour family, whether two colours clash and the non-player's computed attribute

**Routines:**
- `contar_elenco_do_clube()` = 0x53c590 (Database.cpp, cdecl): how many valid people are in the club's squad (with the A/B partner, the reserve squad flags and the loanee filter);
- `reunir_elenco_do_clube()` = 0x541f90 (Database.cpp, cdecl): copies the club's squad (and the A/B partner's, up to 50 or 100) into a vector, padding with zeros;
- `clube_pode_ser_dirigido()` = 0x544820 (Database.cpp, cdecl): whether the club can be managed: senior national team of a nation with +0x88 > 150, or a club that is not a B team, with an active competition and a flagged nation;
- `vetor_para_o_nao_jogador()` = 0x53a980 (Database.cpp, cdecl): writes the low byte of the vector's 21 cells back to the non-player's coaching attributes (+0x0e..+0x22);
- `vetor_para_o_jogador()` = 0x53aa10 (Database.cpp, cdecl): writes the low byte of the vector's cells 0..53 back to the player's attributes and cell 54 to staff +0x58;
- `normalizar_atributos_do_nao_jogador()` = 0x539490 (Database.cpp, cdecl): rebalances the non-player's attributes in two passes: mean of the near-zero cells pulled to −5, positive excess trimmed, cap −80 for job 10;
- `familia_da_cor()` = 0x540520 (Database.cpp, cdecl): the colour's family (1..10) from the index.cpp colour keys;
- `cores_se_confundem()` = 0x540490 (Database.cpp, cdecl): whether two colours clash: same family or the pairs 8-9, 8-7, 6-7 and family 6 against the colour keyed at [0x9cf1dc];
- `atributo_calculado_do_nao_jogador()` = 0x543fd0 (Database.cpp, cdecl): the coaching attribute 1..12: four straight from the record, eight through the 1-to-20 formula with the non-player word +4;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `dq10_reunir` escaped: the 0x541f90 copy is switched off (DESLIGADAS: even the original through the passthrough hook changes the career, because the 0x540a50 it calls leaves its residue on the private stack), so the mutation does not run. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 224's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 225`

**Fields touched:**
- club +0xd7: squad (50 person pointers); pointer; reads (contar_elenco_do_clube, reunir_elenco_do_clube)
- staff +0x00: id (valid if < [0xae2364]); id; reads (contar_elenco_do_clube, reunir_elenco_do_clube)
- staff +0x5e: squad flags (reserve squad filter); value; reads (contar_elenco_do_clube)
- staff +0x39: club; pointer; reads (contar_elenco_do_clube)
- contract (0x4cf5a0) +0x04: contract club id; id; reads (contar_elenco_do_clube)
- [0xae1080] last contract looked up; pointer; writes (contar_elenco_do_clube)
- club +0x244: unnamed (A/B partner flag, zeroed by 0x540a50 on refusal); value; reads and may zero (all three, through 0x540a50)
- club +0x00: id (national teams from [0xae235c] − 2 × [0xae2350]; senior ones below [0xae235c] − [0xae2350]); id; reads (clube_pode_ser_dirigido)
- club +0x53: nation; pointer; reads (clube_pode_ser_dirigido)
- club +0x57: competition; pointer; reads (clube_pode_ser_dirigido)
- nation +0x88: unnamed (> 150 lets the national team be managed); value; reads (clube_pode_ser_dirigido)
- nation +0x11c: flags (bit 2 required); value; reads (clube_pode_ser_dirigido)
- [0xadadfc] competition objects (by id); pointer; reads (clube_pode_ser_dirigido)
- non-player +0x0e..+0x22: coaching attributes (21 signed bytes); value; reads and writes (vetor_para_o_nao_jogador writes, normalizar_atributos_do_nao_jogador reads and writes)
- player +0x0f..+0x44: attributes and positions (54 bytes of CELULAS_DO_VETOR); value; writes (vetor_para_o_jogador)
- staff +0x58: unnamed (cell 54 of the player vector); value; writes (vetor_para_o_jogador)
- staff +0x69: non-player; pointer; reads (normalizar_atributos_do_nao_jogador)
- staff +0x28 / +0x3d: unnamed (jobs; 10 caps the attributes at −80); value; reads (normalizar_atributos_do_nao_jogador)
- colour +0x00: id (compared with the keys 0x9cf178..0x9cf1fc); id; reads (familia_da_cor, cores_se_confundem)
- [0x9cf178..0x9cf1fc] index.cpp colour keys; id; reads (familia_da_cor, cores_se_confundem)
- staff +0x56, +0x58: coaching attributes 1 and 4; value; reads (atributo_calculado_do_nao_jogador)
- non-player +0x04: unnamed (signed word of the formula); value; reads (atributo_calculado_do_nao_jogador)
- non-player +0x10, +0x11, +0x14, +0x17, +0x18, +0x19, +0x1b, +0x1e, +0x21, +0x22: coaching attributes; value; reads (atributo_calculado_do_nao_jogador)

**Of interest to:** CMRTE, CMScout, GPF2
