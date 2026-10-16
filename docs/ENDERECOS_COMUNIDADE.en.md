# Community addresses (text, finance and screen patches)

Points in `cm0102.exe` 3.9.68 that the community has already identified with byte patches. They are not part of the match engine, but they show where rules and texts live that we may want to change after the migration.

**Source:**
- melanzalko, with contributions from davdav, in the "Miscellaneous CM0102 Patches & Fixes" thread on the champman0102.co.uk forum (viewtopic.php?t=10877, pages 1 to 3, August to October 2026).
- Thanks to both.

**The posts' convention:**
- The numbers in the posts are **file offsets**. Here they are converted to VA (VA = file + 0x400000 in `.text`, `.rdata` and `.data`).
- Each "old" byte was checked against the original exe on 2026-10-16.
- The routine is the one containing the address in `docs/mapa_funcoes.csv` (with the module, when the map has it).

## Game rules

### Training facilities
- **VA 0x461532, 0x461552, 0x461572, 0x461592** — routine 0x460590 (`club_screens.cpp`).
- Thresholds of the training facilities description: 0x190 / 0x258 / 0x320 / 0x3e8, that is good / adequate / needs some / needs major.
- The patch lowers them to 285 / 435 / 585 / 950.
- It only changes the description, not the level.

### Financial status with the board
- **VA 0x5986ee (rich, multiplier 0x1770) and VA 0x598716 (secure, multiplier 0x2904)** — routine 0x5986b0.
- These are the multipliers by club reputation.
- The patch changes them to 0x9398 and 0x9538, so that not every club becomes "rich".

### Dividends (in practice a tax on revenue)
They are in routine 0x59cfa0, which has no module in the map yet.

| VA | What it is | Patch |
|---|---|---|
| 0x59db0c | `je` of the chairman's random decision | `jmp` removes the draw |
| 0x59dbf7 | `add ecx, 0x7d0` | — |
| 0x59dca3 | `add ecx, 0x5dc` | — |
| 0x59dc66 / 0x59dcc9 | `cmp word [edi+0x80], 0xfa0` (reputation 4000) | — |
| 0x59dcbb | `shl eax, 3`, the entry threshold's scale | becomes `imul eax, eax, n`, with n from 1 to 8 |
| 0x59dcc1 / 0x59dcc7 | the two cash guards | with NOPs, every club pays |

- The payment adds to +0x94 and +0x134 of the financial record and is subtracted from the cash (64 bits at +0 and +4).
- Then it calls 0x59fe10.
- The tax bands: **VA 0x59714d and VA 0x597a92**, constants 0x1e46 and 0x1c52 in routine 0x5968d0. davdav suggests 3500 and 2500.

### Transfers and contracts
- **VA 0x8edb67** — routine 0x8e9550 (`transfer_screens.cpp`): the cap of 25 (0x19) on the percentage of next sale. davdav: 0x32.
- **VA 0x867abe** — routine 0x867aa0 (`staff_contracts.cpp`): the clean sheet bonus is restricted to goalkeepers. The patch makes the routine always return 1.

### Tours
- **VA 0x471ef1** — routine 0x4719a0 (`club_screens.cpp`).
- It is the reputation threshold for tours to any continent: the byte 4 multiplies 1000.

## Screens and interface

| Area | VA | Routine | What it is |
|---|---|---|---|
| Next match | 0x45e8c3, 0x45e8ce, 0x45e920 | 0x45ccf0 (`club_screens.cpp`) | pitch condition 13 days ahead |
| Club screen | 0x456415 | 0x455942 (`club_screens.cpp`) | PRINT button |
| Competition screen | 0x490219, 0x490220 | 0x48f8e0 (`comp_screens.cpp`) | PRINT button |
| Club screen | 0x456f85 | 0x456900 (`club_screens.cpp`) | pointer to the "Stats" text on the match screen |
| Club fixtures | 0x483f55 (home), 0x483f7f (away), 0x483fa0 (neutral) | 0x483a90 (`club_screens.cpp`) | colors of the H/A/N letters: `mov reg, [color]` becomes `mov reg, immediate` |
| **Tactics screen** | 0x899f00, 0x899efb, 0x899ef7 | 0x899e50 (`tactics_screens.cpp`) | R, G, B of goalkeeper kit 1 |
| **Tactics screen** | 0x89e4ad | 0x899e50 | the jump to the code that paints goalkeeper kit 2 |
| Manager menu | 0x75e8e5, 0x75ebd4 | 0x75db00 (`Menubar.cpp`) | "control all national teams" and "control reserves" options |
| Requests to the board | 0x4254fa | 0x425390 | `push` of the "Meet with the Board" text |

The tactics screen matters for the new graphic element Lucas has considered: 0x899e50 draws the kits, with the color in RGB.

## Texts (in `.data`)

| VA | Text |
|---|---|
| 0x98f9b0 | "Dividends" (finances) |
| 0x9c9720 | "Shareholder dividend payout" |
| 0x9c96b8 | "The board have announced…" |
| 0x98fa94 | "Season Tickets" |
| 0x98ee70 | "Stats" |
| 0x98ff14 | "Non-Playing Staff" |
| 0xaa0dec | "Set Staff Transfer Status" |
| 0x98f8d0, 0x990d54, 0x98f834, 0x990dc4, 0x9908c3, 0x9908d3, 0x99091b | "Tour" and tour phrases |
| 0x98a1a8 | instruction for requests to the board |
| 0x9c740c | instruction for requests to the football association |
| 0xa0fd54, 0xa0fd7b, 0xa0fea6 | investment options and news |

## Areas used as "caves" (depends on the exe)
- **VA 0x58ff74 (file 0x18ff74):** NOPs inside routine 0x58f500 (`file_screens.cpp`). The "Stats" patch writes a text there.
- **File 0x5e8200 and 0x6dd254:** they do not match the original exe. The first is not zero in `.data`; the second lies past the end of the file. They only hold in an exe already expanded by CM0102Patcher. Do not use without checking.

## World Cup qualifiers (the crash of early-start databases)

Source: Danek1985, in the "I've been studying the match engine, happy to share" thread on the same forum (viewtopic.php?t=10790, 2026-07-12).

- **The year in which each confederation draws its qualifiers** is a word (`cmp`/`mov` with the year) inside each qualifying league's routines:
  - Asia: VA 0x911c4d, 0x911c88, 0x911ca4 and 0x911cb8, in routine 0x911bf0 (`wc_asia_league.cpp`);
  - Europe: VA 0x9182dc, in routine 0x918200 (`wc_europe_league.cpp`);
  - South America: VA 0x92036e, in routine 0x9202b0 (`wc_south_american_league.cpp`).
- **In the original 3.9.68 exe** they are 2000 (0x7d0) at the first three Asian sites and 2004 (0x7d4) at the others. The post fixes an exe whose start was moved to 1994, where they stay at 1994 (0x7ca); the fix sets 1996 (0x7cc).
- **The cause, according to him:** the draws are staggered (Asia in 1995, Europe in 1996). In the gap, the Asian qualifying code looks up the still-empty European group and reads a null pointer: a read of 0x3e at VA 0x91df0e, in routine 0x91def0. It only takes effect in a new game.
- **For us:** a real case of cross-reference between competitions, to remember when we port the qualifiers or change the start year.
