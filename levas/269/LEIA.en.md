# Batch 269 — the league: the highest-rated fixtures moved to the TV day and slot, the group draw news and the opening fixtures news

**Routines:**
- `liga_jogos_para_a_tv()` = 0x685640 (league.cpp, thiscall, ret 4): on the TV table dates, each slot takes the highest-rated fixture (a draw by reputation, zeroed when either club plays 2..5 days before or after; 100/50 in national team slots) to the weekday and time slot of the slot, with the news, the stadium and the calendar;
- `liga_noticia_dos_grupos()` = 0x684230 (league.cpp, thiscall, ret 0): group news 0x15 (the competition, the first date, the opponents) to each human-managed club;
- `liga_noticia_da_tabela()` = 0x683ee0 (league.cpp, thiscall, ret 4): news 0xd with the first three fixtures (date, opponent, venue) to each human-managed club not yet told, and the same in the following stages;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (8845ebc9).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 151bdf85, original 385e2e8b (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 268's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 269`

**Fields touched:**
- league +0xba (TV date table, 0x41 bytes per entry; count +0xa9) +0 day, +2 year − the league one, +4 slot, +5.. 7-byte slots (+0 stadium, +4 weekday, +5 TV slot, +6 type: 0xff = empty); value; reads and writes (liga_jogos_para_a_tv: the used slot becomes 0xff, the order by 0x945333), reads (liga_noticia_dos_grupos, liga_noticia_da_tabela)
- league +0x40 year, +0x44 type/stage (0xff first stage), +0x3e teams, +0xb1 0x3b-byte teams (+0 club), +0x24, +0x3a, +0xc/+0x30 the stages (+0x43); pointer and value; reads (liga_jogos_para_a_tv, liga_noticia_dos_grupos, liga_noticia_da_tabela)
- fixture (0x4f bytes in the calendar) +0xc/+0x10 ids, +0x18 stadium, +0x1c/+0x20 clubs, +0x28 year, +0x2a day, +0x3b type, +0x3f slot, +0x42 league type; value; reads and writes (liga_jogos_para_a_tv), reads (liga_noticia_da_tabela)
- club +0x80 reputation, +0x53 nation; value; reads (liga_jogos_para_a_tv)

**Of interest to:** CMRTE
