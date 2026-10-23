# Batch 222 — the reputation scope, the reputation seen by a viewer and whether a person is foreign; the person's age, whether the nation has free transfers and whether the person is an EU national; whether the person is transfer listed and the 1 to 20 player attributes

**Routines:**
- `escopo_da_reputacao()` = 0x5408c0 (Database.cpp, cdecl): how far the viewer stands from the person (0 same club or league, 2 same nation, 3 club nation or high reputation, 4 world): picks the reputation;
- `reputacao_vista_por()` = 0x5407e0 (Database.cpp, cdecl): the person's reputation as seen by a viewer: current, home, world or a blend, by the scope; player or non-player data;
- `pessoa_e_estrangeira()` = 0x53d980 (Database.cpp, cdecl): whether the person is foreign in the nation: nationalities, groups (British Isles, Australia, [0x9cf360]) and the Portugal rule for contracts up to 1998;
- `idade_da_pessoa()` = 0x53ab90 (Database.cpp, cdecl): the person's age in whole years on a date, from the birth date;
- `nacao_com_transferencia_livre()` = 0x544670 (Database.cpp, cdecl): whether the nation has free transfers: new FIFA rules, Brazil, or Europe except Poland, Turkey, Croatia and Russia;
- `pessoa_e_comunitaria()` = 0x53d740 (Database.cpp, cdecl): whether the person counts as an EU national for a club of the nation: the nationality EU mark, Switzerland and the Germany, [0x9cf360] and Denmark exceptions;
- `pessoa_listada_para_transferencia()` = 0x544520 (Database.cpp, cdecl): whether the person's link with the club (or the contract club) is transfer listed, by the club or at request;
- `atributo_calculado_do_jogador()` = 0x543540 (Database.cpp, cdecl): a player attribute (1 to 31) as 1 to 20: the record byte or, for relative ones, the rating of 2 × byte + current ability;
- `atributo_da_pessoa()` = 0x5431e0 (Database.cpp, cdecl): a player attribute of the person, from the person's player data (0x543540);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 9 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 221's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 222`

**Fields touched:**
- staff +0x39: contracted club; pointer; reads (escopo_da_reputacao)
- club +0x53: nation; pointer; reads (escopo_da_reputacao)
- club +0x80 [2]: reputation; value; reads (escopo_da_reputacao: + player current reputation ≥ 14000)
- staff +0x1a / +0x1e: nationality and second nationality; pointer; reads (escopo_da_reputacao, pessoa_e_estrangeira)
- staff +0x00: id (the last 16 are the human managers); id; reads (escopo_da_reputacao)
- staff +0x61: player data; pointer; reads (escopo_da_reputacao, reputacao_vista_por)
- staff +0x69: non-player data; pointer; reads (reputacao_vista_por)
- player +0x09 / +0x0b / +0x0d [2]: current, home and world reputation; value; reads (escopo_da_reputacao, reputacao_vista_por)
- non-player +0x08 / +0x0a / +0x0c [2]: current, home and world reputation; value; reads (reputacao_vista_por)
- nation +0x00: id; id; reads (pessoa_e_estrangeira: compared with [0x9cf2e4] England, [0x9cf45c] Scotland, [0x9cf510] Wales, [0x9cf3e4] Northern Ireland, [0x9cf2dc] Ireland, [0x9cf224] Australia, [0x9cf3f4], [0x9cf360], [0x9cf4f0] and [0x9cf434] Portugal)
- contract (0x4cf5a0) +0x25 / +0x27 [2]: start day of year and year; value; reads (pessoa_e_estrangeira, Portugal rule: up to 1/1/1998)
- [0xae2364] staff count; value; reads (escopo_da_reputacao)
- staff +0x10 / +0x12 [2]: birth date (day of year and year); value; reads (idade_da_pessoa)
- staff +0x1a / +0x1e: nationality and second nationality; pointer; reads (pessoa_e_comunitaria)
- nation +0x7f: EU mark (2 = EU); value; reads (pessoa_e_comunitaria)
- nation +0x71: continent; pointer; reads (nacao_com_transferencia_livre, pessoa_e_comunitaria)
- nation +0x00: id; id; reads (compared with [0x9cf260] Brazil, [0x9cf430] Poland, [0x9cf4dc] Turkey, [0x9cf2ac] Croatia, [0x9cf448] Russia, [0x9cf4ac] Switzerland, [0x9cf314] Germany, [0x9cf360], [0x9cf2c0] Denmark, [0x9cf348], [0x9cf408] Norway, [0x9cf3a0])
- continent +0x00: id (Europe = [0x9cfa10]); id; reads (nacao_com_transferencia_livre)
- [0xae233c] new FIFA rules (1 = free transfers everywhere); value; reads (nacao_com_transferencia_livre)
- [0xae23a4] continent table (198 bytes); pointer; reads (pessoa_e_comunitaria)
- staff +0x61: player data; pointer; reads (atributo_da_pessoa)
- staff +0x58: attribute 10 (raw byte); value; reads (atributo_calculado_do_jogador)
- player +0x05 [2]: current ability; value; reads (atributo_calculado_do_jogador)
- player +0x0f: goalkeeper (1..20; = 20 or < 15 changes the formula); value; reads (atributo_calculado_do_jogador)
- player +0x1b..+0x20, +0x28, +0x29, +0x2e, +0x2f, +0x36, +0x3c, +0x3d, +0x3f, +0x40, +0x44: absolute attributes (acceleration, agility, ..., work rate); value; reads (atributo_calculado_do_jogador)
- player +0x1e, +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x37, +0x39, +0x3a, +0x3e, +0x43: relative attributes (anticipation, ..., tackling, creativity; rated with the current ability); value; reads (atributo_calculado_do_jogador)
- contract (0x4cf5a0) +4: club id; id; reads (pessoa_listada_para_transferencia)
- staff-club link (0x4cf5f0) +0x4e: flags (1 transfer list, 2 loan list, 8 at request); value; reads (pessoa_listada_para_transferencia)
- [0xae1080] last looked-up contract; pointer; writes (pessoa_listada_para_transferencia, without a club)

**Of interest to:** CMScout, CMRTE
