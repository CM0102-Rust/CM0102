# Batch 316 — the injury type names and the return to training news text; the manager departure news text

**Routines:**
- `injury_get_type_name()` = 0x62ef30 (injury.cpp, thiscall, ret 0x10): the translated name of an injury type (0 to 0x51), with the indefinite article ("a cold") or without it; in language 3 always without; another article value outside language 3 writes nothing; a buffer under 30 bytes or a type outside the table: the error box;
- `injury_write_recovery_news_text()` = 0x631580 (injury.cpp, thiscall, ret 0xc): the headline or body of the news of the return to light or full training: the headline with the name; the body by the type's category: treatments and holidays (12) by the type, the outcome and slot 10; illnesses (0) by the type; the others with the injury's name (the category, or the type in category 6) and gender; in light training, the expected time to full training (approximate); the person's link when the ids and the birth date still match;
- `news_manager_departure_text()` = 0x6a1d60 (manager_manager.cpp, stdcall, ret 0xc): the headline or body of the news that a club or national team manager was sacked, resigned or retired: the headline by how he left and by the job (manager, player-manager, or with a reason), with the club's short name; the body, for a national team, by the first or second national team, how he left and the reason, with the nation's adjective; for a club, sacked as manager or player-manager with or without a second name, or with one of 16 reasons from two tables; resigned by the job; the other ways with a single text; the links of the person (when the ids match), of the club and of the adjective (when they appear in the text);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (1):** 1 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (e39bebee).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 e9db46f8, original 60d7b94b (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 315's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 316`
