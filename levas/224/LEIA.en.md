# Batch 224 — the first and second name draw by nation and the formatted person name; the club's kit colours, the club's manager and the club's English name; the morale adjustment, the non-player attribute vector and the person's squad flags

**Routines:**
- `primeiro_nome_da_nacao()` = 0x539c60 (Database.cpp, cdecl): draws a first name for the nation (20 draws by weight when it has over 100; otherwise borrows from the language group);
- `segundo_nome_da_nacao()` = 0x53a1b0 (Database.cpp, cdecl): draws a second name for the nation, the same way;
- `formatar_nome_da_pessoa()` = 0x53dba0 (Database.cpp, cdecl): writes the person's name in one of 7 styles (first last, F.last, last, first, initials...), preferring the common name or nickname;
- `cores_do_uniforme_do_clube()` = 0x53b640 (Database.cpp, cdecl): the club's home, away or third kit colours as screen pixels, with defaults when missing;
- `tecnico_do_clube_ou_selecao()` = 0x53ca90 (Database.cpp, cdecl): the manager of the club, of its A/B partner or, for an under-21 side, the senior national team's;
- `nome_ingles_do_clube()` = 0x545880 (Database.cpp, cdecl): the club's name, or its English one from eng.lng when another language is loaded;
- `ajustar_moral_da_pessoa()` = 0x544710 (Database.cpp, cdecl): adds a delta to the player's morale, damped by a draw near the ends, clamped to 1..20; returns whether it changed;
- `vetor_de_atributos_do_nao_jogador()` = 0x53a8b0 (Database.cpp, cdecl): widens the non-player's coaching attributes (+0x0e..+0x22) into a 21-cell vector of 3 bytes, with the split at 13;
- `marcar_elenco_da_pessoa()` = 0x540980 (Database.cpp, cdecl): sets the person's club or national squad bits, keeping the other pair;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `dq8_ingles` escaped: the English club name only reads eng.lng when the game runs in another language; the career and the real game comparing run in English (language 0/1), where the routine returns the name itself. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 223's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 224`

**Fields touched:**
- nation +0x00: id; id; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- nation +0x77: language group (1..32; 0 none); value; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- nation name index ([0xae2338] + id × 36) +0x00 / +0x04: first first name / first second name; id; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- nation name index +0x18 / +0x1c: first / second name count; value; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- name row (60 bytes at [0xae23f0] first, [0xae23f4] second) +0x00 text, +0x3b weight (signed); value; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- [0xae2398] / [0xae239c] first / second name total (capped at 0x7fff in the draw); value; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- [0xae23a8] nations (0x122 bytes), [0xae2350] count; pointer; reads (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- [0xad6c30] CRT rand() state; value; reads and writes (primeiro_nome_da_nacao, segundo_nome_da_nacao)
- staff +0x04 / +0x08 / +0x0c: first name / second name / common name; pointer (text); reads (formatar_nome_da_pessoa)
- [0xb63c98] nicknames given by humans (0x600740, 0x6005e0); pointer; reads (formatar_nome_da_pessoa)
- club +0x83 / +0x87: home colours (foreground / background); pointer; reads (cores_do_uniforme_do_clube)
- club +0x8b / +0x8f: away colours; pointer; reads (cores_do_uniforme_do_clube)
- club +0x93 / +0x97: third kit colours; pointer; reads (cores_do_uniforme_do_clube)
- colour +0x37 / +0x38 / +0x39: red / green / blue; value; reads (cores_do_uniforme_do_clube)
- [0xaebdf0] / [0xae3184] default foreground / background pixels; value; reads (cores_do_uniforme_do_clube)
- club +0xcf: manager; pointer; reads (tecnico_do_clube_ou_selecao)
- club +0x00: id (≥ [0xae235c] − [0xae2350] national team); id; reads (tecnico_do_clube_ou_selecao, nome_ingles_do_clube)
- club +0x53: nation; pointer; reads (tecnico_do_clube_ou_selecao)
- club +0x244: unnamed (A/B partner flag, zeroed by 0x540a50 on refusal); value; reads and may zero (tecnico_do_clube_ou_selecao, through 0x540a50)
- club +0x04: name; text; returns its address (nome_ingles_do_clube)
- [0x9b5984] language; value; reads (nome_ingles_do_clube)
- [0xae2408] English names (+0 count, +0x20 79-byte rows), [0xae23fc] id map; pointer; reads (nome_ingles_do_clube)
- staff +0x61: player; pointer; reads (ajustar_moral_da_pessoa)
- player +0x45: morale (1..20); value; reads and writes (ajustar_moral_da_pessoa)
- player +0x2f, staff +0x5a, staff +0x58: unnamed (their sum / 3 caps the draw that damps a morale drop); value; reads (ajustar_moral_da_pessoa)
- staff +0x39: club; pointer; reads (ajustar_moral_da_pessoa, with cap)
- staff-club link (0x4cf5f0) +0x45: unnamed (non-zero stops morale rising with the cap); value; reads (ajustar_moral_da_pessoa)
- [0xde1f5c], [0xde1f60], [0xad6c30] generator state; value; reads and writes (ajustar_moral_da_pessoa, through 0x90cfc0)
- non-player +0x0e..+0x22: coaching attributes (21 signed bytes); value; reads (vetor_de_atributos_do_nao_jogador)
- staff +0x5e: squad flags (& 3 club, & 0xc national team); value; reads and writes (marcar_elenco_da_pessoa)

**Of interest to:** GPF2, CMRTE, CMScout
