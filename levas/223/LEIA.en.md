# Batch 223 — the competition history order, the player attribute vector and whether the nation is British or Ireland; whether the club is Basque and whether the person is on the club transfer list or the loan list; whether the person is listed at request, the rating of a relative attribute and whether the club may have a reserve side

**Routines:**
- `ordem_do_historico_de_competicao()` = 0x540780 (Database.cpp, cdecl): qsort comparison of the competition history: rows without a competition last, by competition id and by season, newest first;
- `vetor_de_atributos_do_jogador()` = 0x53a610 (Database.cpp, cdecl): widens the player's positions, sides and attributes into a 63 cell vector (value and weight) and returns the group ends (9, 0xc, 0x20);
- `nacao_britanica_ou_irlanda()` = 0x5446d0 (Database.cpp, cdecl): whether the nation is England, Scotland, Ireland, Wales or Northern Ireland;
- `clube_basco()` = 0x53d7f0 (Database.cpp, cdecl): whether the club is Spanish and one of the 23 Basque clubs named in the code;
- `pessoa_listada_pelo_clube()` = 0x5444a0 (Database.cpp, cdecl): whether the person's link with the club (or the contract club) is transfer listed by the club; job 0xf at a club without a human manager does not count;
- `pessoa_listada_para_emprestimo()` = 0x544590 (Database.cpp, cdecl): whether the person's link with the club (or the contract club) is loan listed;
- `pessoa_listada_a_pedido()` = 0x544600 (Database.cpp, cdecl): whether the person's link with the club (or the contract club) is transfer listed at request;
- `nota_relativa_do_atributo()` = 0x544380 (Database.cpp, cdecl): the 1 to 20 rating of 2 × the attribute byte + the player's current ability;
- `clube_pode_ter_reservas()` = 0x542740 (Database.cpp, cdecl): whether the club may have a reserve side: not national teams, A/B pairs, Spanish, German and Portuguese clubs and some second national sides;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 9 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 222's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 223`

**Fields touched:**
- competition history +0x04: competition; pointer; reads (ordem_do_historico_de_competicao)
- competition history +0x08 [2]: season (unnamed; −1 = none); value; reads (ordem_do_historico_de_competicao)
- competition +0x00: id; id; reads (ordem_do_historico_de_competicao)
- player +0x0f..+0x1a: positions and sides (unnamed in CAMPOS.md; +0x0f goalkeeper); value; reads (vetor_de_atributos_do_jogador)
- player +0x1b..+0x44: attributes (acceleration +0x1b, anticipation +0x1e, consistency +0x21, dribbling +0x26, heading +0x2b, jumping +0x2e, leadership +0x2f, left foot +0x30, long shots +0x31, marking +0x32, off the ball +0x33, one on ones +0x35, pace +0x36, passing +0x37, right foot +0x3b, stamina +0x3c, tackling +0x3e, teamwork +0x3f, technique +0x40, creativity +0x43, work rate +0x44; the others unnamed); value; reads (vetor_de_atributos_do_jogador)
- staff +0x58: unnamed (attribute 10 of 0x543540); value; reads (vetor_de_atributos_do_jogador)
- nation +0x00: id; id; reads (nacao_britanica_ou_irlanda: compared with [0x9cf2e4] England, [0x9cf45c] Scotland, [0x9cf2dc] Ireland, [0x9cf510] Wales, [0x9cf3e4] Northern Ireland)
- club +0x00: id; id; reads (clube_basco: compared with 23 clubs at [0x9d0120]..[0x9d0338])
- club +0x53: nation; pointer; reads (clube_basco)
- nation +0x00: id; id; reads (clube_basco: compared with [0x9cf488] Spain)
- staff +0x3d: job at the club (unnamed in CAMPOS.md; 0xf counts only at a human manager club); value; reads (pessoa_listada_pelo_clube)
- contract (0x4cf5a0) +4: club id; id; reads (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo)
- staff-club link (0x4cf5f0) +0x4e: flags (1 transfer list, 2 loan list, 8 at request); value; reads (pessoa_listada_pelo_clube: 1; pessoa_listada_para_emprestimo: 2)
- [0xae1080] last looked-up contract; pointer; writes (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo, without a club)
- contract (0x4cf5a0) +4: club id; id; reads (pessoa_listada_a_pedido)
- staff-club link (0x4cf5f0) +0x4e: flags (8 at request); value; reads (pessoa_listada_a_pedido)
- [0xae1080] last looked-up contract; pointer; writes (pessoa_listada_a_pedido, without a club)
- player +0x05 [2]: current ability; value; reads (nota_relativa_do_atributo)
- club +0x00: id (≥ [0xae235c] − [0xae2350] national team; ≥ [0xae235c] − 2 × [0xae2350] second national team); id; reads (clube_pode_ter_reservas)
- club +0x53: nation; pointer; reads (clube_pode_ter_reservas)
- club +0x244: unnamed (A/B partner flag, zeroed by 0x540a50 on refusal); value; reads and may zero (clube_pode_ter_reservas, through 0x540a50)
- nation +0x00: id; id; reads (clube_pode_ter_reservas: compared with [0x9cf488] Spain, [0x9cf314] Germany, [0x9cf434] Portugal, [0x9cf408] Norway, [0x9cf360], [0x9cf260] Brazil, [0x9cf218] Argentina)
- nation +0x71: continent; pointer; reads (clube_pode_ter_reservas: [0x9cfa18] and [0x9cfa08])
- [0xae235c] / [0xae2350] club total / nation count; value; reads (clube_pode_ter_reservas)

**Of interest to:** CMScout, CMRTE
