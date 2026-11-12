*English version of [FORA.md](FORA.md). Both are kept in sync; the Portuguese file is the original.*

# nma_7 left out (2026-11-12): NewsItem::write_text (0x77bc80, news.cpp) too big for one session

The routine was not ported. There is no partial copy (neither hooked nor stored): the hook must cover the whole routine in a single `Prologue`, and a faithful port of all of it does not fit in one session with the checking the item needs. This is the map measured on the disassembly (`desmonta.py jogo/cm0102.exe 0x77bc80 0x782b70`, 8,440 instructions, 599 calls), so the next round can split it into sessions.

## Signature and frame

- thiscall, `ret 0x10` (4 arguments): ecx = the news item; arg1 (`[esp+0x13c8]` after the 4 pushes) must not be 0; arg2 = the `Text` object (ebp), cleared by 0x4afa40 (comp_text.cpp) at the start; arg3 = headline (≠0) or body; arg4 is passed on. Returns 1 (written) or 0 (error).
- 0x13a8-byte frame (`__chkstk` 0x945610) with SEH (0x95e1cb): several local `Text` objects destroyed on the error path and on exit.
- Null arg1 or arg2: the "Error" box with `news.cpp` lines 0x9f / 0xa7 (0x944cff, 0x90d130, 0x944c9f, 0x5e8290), clears 0xb67a34 and returns 0.
- The category comes from 0x785670 (thiscall, the news item; returns the group in al); the type is the dword `[item+0]`.

## The pieces (by category)

| Category (0x785670) | Types that only dispatch to the exe's writer | Own body (the bulk of the routine) |
|---|---|---|
| 2 | 0x1b–0x25, 0x29, 0x2a, 0x2b, 0x2c, 0x2e, 0x2f, 2, 0x18, 0x19, 0x1a (21 writers: 0x414420, 0x76e3b0, ..., 0x6aadb0) | **types 1, 3–0xd, 0x10–0x17, 0x26, 0x27, 0x28: 0x77bfe2–0x7801e9, ~16.9 KB**; 0x15 (0x77f3e4, 0x9a4 bytes) has a sub-switch (0x41c, 0xa, 0x14, 0x82, 0x449, 0x44a) |
| 7 | 0x3e8 (0x44c650) | 0x3e9: 0x780229–0x7802cd |
| 3 | 0x7d1, 0x7d2, 0x7d3 (0x631580, 0x5576e0, 0x55b8b0) | **0x7d0: 0x7802e5–0x78164b, ~4.9 KB**, with jump tables (0x782ae8, 0x782afc, 0x782b3c, 0x782b50) |
| 6 | 0x1770–0x1790, 31 writers (0x59f3e0 ... 0x6a3060) | — |
| 4 | 0xbb8–0xbea, 41 writers (0x7aa4c0 ... 0x738d50; includes 0x72d270 and 0x744f80, being ported by other agents) | — |
| 5 | 0xfa0–0xfc3, 36 writers (0x8d3520 ... 0x8e51d0; includes 0x755860, being ported) | — |
| 1 | 0x1389–0x1394 (12 writers) | 0x1388: 0x782848–0x782a02, builds a local `Text` and **calls 0x77bc80 itself** on another news item (recursion; 0x786b70, 0x785a20, 0x53dba0) |

Unknown type: falls to 0x782a68 (returns 0 without writing).

The dispatch (~150 types) is mechanical. What did not fit is the ~22 KB of own body: texts built with `langlib_sprintf` (0x66f4e0, 101 calls), `Text::add_link` (0x4ae8a0, 86), `Text::set_string` (0x4ae660, 52), dates (0x549ef0, 0x54bef0, 0x54a3f0, 0x54a110), competition names (0x4bfa30, 0x4bff70, with the "f"/"r" article swaps by string comparison), injuries (0x6306e0 ×22, 0x62ec00, 0x62ef30), club/person/competition tables read by raw field, and the local `Text` objects with destructors.

## How to do it (proposal)

1. One session per own-body group, in a staging folder **without** a `Prologue` (e.g. `levas/fila/nma_7/pedacos/`): (a) category 2, types 1–0xd; (b) category 2, types 0x10–0x17, 0x26–0x28 (0x15 with the sub-switch); (c) category 3, 0x7d0 with the jump tables; (d) category 7 0x3e9 and category 1 0x1388 (the recursion calls 0x77bc80 through the exe, which is the hook).
2. Then one session joins the dispatch, the assertions and the pieces, hooking the whole routine, with the text comparer (original into `out`, the copy into a local `Text` inside `side_by_side` with `STATE_REGIONS` and `without_counting`, comparing string and links) and each piece's `covered!` (`nma_7 cat2 type 6`, ...), naming the piece of the divergence.
3. Find out first which types the ENG career runs (only the function's count is in `docs/cobertura_simulacao.txt`: 4,568 calls to 2002-06-01), to put the mutations in the pieces that run.

Defects of the original: none found in this reading (only the map was made).
