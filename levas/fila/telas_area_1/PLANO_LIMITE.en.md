# Lifting the 30-row limit of the areas (second stage)

## The problem

The match report builds the list (0x55fba0 → `Area::init` 0x402ac0) with `n = panel height / font 2 height` rows and 2 columns, with no row weights. At 800x600, `n` ≤ 30. At 1080, `n` goes past 30 and:

1. `Area::init` fills the row weights (+0xbcb, a 30-byte vector) with `n` bytes of 1. From 31 on it overruns the vector: it hits +0xbe9 (open) and, at 35 or more, +0xbed (has scrollbar), which wrongly becomes "true". Past 38 it leaves the area (0xbf1 bytes) and runs into the next area (or into the widget table, if this is the last slot).
2. `Area::layout` 0x403350 checks +0xbac > 30 and shows the **Area.cpp 507** box (497 is the one for columns).

## The structure cannot grow

The areas live inside the Display: 249 slots of 0xbf1 bytes from Display+4, followed right away by the widget table (Display+0xba95e). The object cannot be enlarged. The vectors stay where they are (their first 30 entries stay the same as in the original) and the Rust copies get a **side table** keyed by area address: weights [u8; 255] and column/row start/end [i32; 255]. The u8 counts cap it at 255.

## Who uses the vectors (search by offset over the whole .text)

- weights +0xbad/+0xbcb: only `Area::init` 0x402ac0 and `Area::layout` 0x403350;
- grids +0x1c/+0x94/+0x10c/+0x184: only `Area::layout` (writes), `Area::draw` 0x402e70 and `Area::scroll_rows` 0x403ee0 (read);
- counts +0xbab/+0xbac (the count only, not the vectors): `Area::set_scroll_row` 0x403320, `setup_scrollbar` 0x403600, `Guio::init` 0x5ee150 (adds a row to pop-up areas, with no limit), Display 0x560490, 0x560c30, 0x561210 and 0x561f10 (+0xbb0 from the Display). These stay correct with more than 30 rows.

## Order

1. This batch: `layout`, `init` and `draw` in Rust, identical to the original (with the limit and the box).
2. Next batch: `scroll_rows` 0x403ee0 (the report scrolls).
3. Lift it: once all four are in Rust, `init` and `layout` accept up to 255 rows through the side table (without overrunning the vector), and `draw`/`scroll_rows` read from it. With 30 rows or fewer everything stays the same as the original and the comparators still hold. Above 30 only the copy runs (the original would show the box), and the proof is the report opened in Lucas's game at 1080.

## Risks

- Vector accesses through a computed pointer do not show up in the offset search. Before lifting, check with `CM_ESPIAO` or a page guard on the areas at 1080.
- The side table must be cleared when the area is reinitialised (`init`) and when the Display is cleared (0x55f7c0 `Display::reset`). A stale entry would draw widgets in the wrong place.
- The remote Display (network, 0x569540) sends the weights with `extra_len`. That is not the report's case (null pointer), but a screen that sends more than 30 weights over the network still breaks on the other side.
- `init` differs between the exes (0x31f/0x257 → 0x77f/0x437): the copy reads the immediates from the running exe.
