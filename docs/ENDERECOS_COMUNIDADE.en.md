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

## Portuguese texts (language.ldb and statistics)

Source: the same Danek1985 thread, page 2 (t=10790&start=25, 2026-07-18), with notes from tafo.

- **Preferred foot in Portuguese:** `Data\language.ldb` (204 MB) stores the translated labels. At 0x070B0198 in the file is "Esquerdo"; changing it to "Só Esque" fixes the wrongly displayed "Left only". According to tafo, each label may appear twice in the file, a few KB apart.
- **Blank "Key" column in the match statistics in Portuguese:** VA 0xa161cc (file 0x6161cc), in the "Won", "Tck", "Key", "Cmp" abbreviation list in `.data`. The patch changes "Key" to "Imp".
- **For us:** when the interface is ours, the translated labels and abbreviations come from `language.ldb`, and the English abbreviations serve as keys.

## maxevans60's function and global map (2026-10-18)

The public map [cm0102-function-map](https://github.com/maxevans60/cm0102-function-map) (maxevans60, CC BY 4.0 licence, with extra permission for AI use) gives a name, source file, convention, purpose and confidence to the 6,863 functions of `cm0102.exe` 3.9.68, and a name, type and readers/writers to the 4,428 globals. It was made for the "Bosman Edition", a recompiled port of the game to Mac, Windows and Linux.

- Cloned into `referencias/cm0102-function-map` (outside git, like the other references).
- `engenharia-reversa/pacote.py` puts in each package's `LEIA.md` the routine's name, convention and purpose in the map and the globals it uses.
- Check on 2026-10-18: the 391 routine starts already ported here are function starts in the map; of the 6,071 starts shared with `docs/mapa_funcoes.csv`, only 44 have a different file. The names of the routines from the night of 10/17 match ours (for example 0x5b7d20 `formation_get_position_mask`, 0x9326e0 `zopen`).
- Medium or low confidence names are a hint, not proof: the criterion is still the comparison with the original.
- Thanks to maxevans60, and to those he credits (John Locke, Tapani, Saturn and the Offset Index thread).

## Old forum threads (nic.opik.net archive, read on 2026-10-20)

The [OldThreads](https://nic.opik.net/open/cm0102/OldThreads/) archive keeps the old champman0102.co.uk threads as PDFs. These were read in full: the **Offsets** thread (2,357 posts, 2012–2020, "New Rip" version), **Nicks CM0102Patcher** (New Rip, 1,681 posts), **Save Game Queries** (1,329 posts) and 14 smaller threads (Database Structure, CM Secrets, Adding space to an exe, Transfer Window, Resetting regens, Using Development Benchmark Mode, Add Fonts, among others). The PDFs, the extracted text and the per-part reports live in `referencias/forum_antigo/` (outside git; `achados/` holds the reports). The addresses below are VAs checked against the original `cm0102.exe`; those valid only in patched exes were left out.

**Command line and environment** (scan of the switches tested on `g_command_line_text` 0xae2c98 through 0x945a60):
- `-load <save>` (0x81bdb1, near `setup_run_front_end`) opens a save with no clicks. Other switches: `-verifypics` (0x41b4b3, 0x8282fb), `-verifyhist`, `-verifyplcfg`, `-verifyeurocfg` (0x828316, 0x828331, 0x828356), `-verifysounds` (0x6da68c), `norealplhist`, `checkgks` (0x7c0292, 0x7c0429), `-media` (0x7da35c), `-nonetwork` (0x81b9bf), `-dontcheckram` (0x81dff0), `-ignoreenv` (0x90bc57), `-defaultfont` (0x5d79ba), `nomemcheck` (strcmp at 0x5cc6d0).
- Environment variables read at 0x90bc50: `SI_DATA`, `CM3_DATA`, `CM3_TEMP`, `CM3_DEBUG`, `CM3_PREFS` (folder of `game.cfg`), `CM3_TACTICS`, `CM3_HISTORY`, `CM3_PICTURES`, `CM3_SOUNDS`, `CM3_QSTART`, `CM3_HELP`, `CM3_TRAINING`.
- 0x68d3b6 (`jne`): the single-instance lock. 0x9472cd (`push 0x2710`): the main loop wait.

**Competitions** (stage and cup structure):
- 0x48c580: default league stage constructor (+0x1c/+0x20 = −1, +0x49 = 5 substitutes named, +0x4a = 3 substitutions, +0x42, +0x51, +0x45, +0x30).
- League stage: +0xc2 points per win, +0xc3 per draw, +0xc7 non-EU, +0xbf promotion play-off, +0xc0 relegation play-off (with +0xbe and +0xc1), +0x40 season year.
- Competition: +0x2c = stages − 1 (league 0, league with play-off 1, World Cup 8, Champions League 0x0d); +0x42 type (4 national teams, 2 international clubs, 1 domestic); +0x50 suspension group (9 England, 0xff none); +0x13 foreign signing limit; +0x4c Serie A "spareggio"; +0xab league tie-break.
- Cup round (0x68 record): +0x0b draw type (1/3 lower side at home, 4 seeded, 8 better-placed side at home); +0x0d/+0x0f/+0x11 tie-break (low byte 0x83 golden goal, 0x03 extra time and penalties; high byte 0x02 away goals); cup stage +0x229 legs and +0x22a days to the second leg; cup +0xf2 number of clubs.
- Cup.cpp: 0x521e60 draw date, 0x521eb0 match date (day, month 0–11, weekday, slot, season year 0/1), 0x521f10.
- Setup.cpp 0x830520: one `je` per competition (114, from 0x8322a2 to 0x838672) before its constructor; competition vector 0xae23d0 (0x6c bytes).
- Hosts (host_country.cpp 0x5f9970): World Cup 0x5f99e9.., Euro 0x5f9c47.. (year at 0x5f9c0a), African Cup 0x5f9eea.., Olympics 0x5fa030.., Copa América 0x5fa1b5.., Asian Cup 0x5fa563...
- Club World Championship: year 0x9291b2; places through 0x92ae60 (reputation cut 5000; league selected through nation +0x11c).
- Globals filled by name: nations at 0x605144.. (inside 0x601a40), clubs at 0x60a973.. (inside 0x606060); nation region (+0x75: 1 Africa … 24 Spain) and continents (0 Africa … 5 South America).

**Players and transfers:**
- Internal attribute → 1..20 rating: `t = attribute/10 + CA/k + 10`, `rating = t²/30 + t/3 + 0.5`, clamped to 1..20 (0x5442f0, 0x543540).
- 0x4d0c60 player value: base value by position (striker 13 M, peak at 28; goalkeeper 6 M, peak at 33; the others 8 to 11 M), reputation cuts 5000/8000, table 0x9a7f30.
- 0x8c5880 second nationality: 3 years (Portugal and Spain), 6 (Europe), 11 (Italy, Finland, Russia), never outside Europe. 0x53d740 "is EU?" (nation +0x7f = 2; exceptions for Switzerland, Germany, Italy and Denmark).
- Loans: 0x579c70 four-month limit (byte +0x18 < 24, date 19 May); England 0x579e58 (5 per season) and 0x579f16 (at once). Foreigner rules per country (42-byte record filled with −1: England 0x579c50, France 0x5c1a30, Belgium 0x424f80, Italy 0x6619e0, Portugal 0x7d43c0, Mexico/Turkey 0x902180); a nation with no rules uses the English ones.
- Regens: 0x7abac0 (CA by position, aged 15 to 20), nationality swap only with a nation under 100 people (0x7abcd7), promotion blocked with 50 or more in the squad (0x7ad1bd); pregens 1.035 (0x524de9).
- Training: 0x8b2be0 writes the attributes; 0x8b4e50 the default schedule (levels 0, 0x0a, 0x19, 0x32).

**Match engine:**
- Shot: 0x702050 calls 0x700bd0 (shot) and 0x700420 (goalkeeper); `100·shot/(shot+keeper)` compared with 71.0 (0x96d808). Golden goal at 0x6d4250. Event codes = the `events_eng.cfg` code + 8000.
- In `atualizar_atributos` (0x6ec010, already in Rust), with CA of 125 or more the jump at 0x6eda39 leaves creativity with the previous attribute's value. The community (MadScientist) treats it as a bug; our copy reproduces the original (left for modernisation).
- Generator: srand 0x9469fa, rand 0x946a04 (seed at 0xad6c30). With a fixed seed, loops such as `while(rand(10)!=5)` hang.

**Save** (confirms `SAVE_2GB.en.md`): the container index seen by Hugo9 in hex (`injury_history`: position 0xec20fb, size 0x17d90); items `injury_history.tmp`, `club cup history.tmp`, `club league history.tmp`, `club records history.tmp`, `comp records history.tmp`, `ban_history.tmp`. Safe saving in `save_game_data` (0x82e430): temporary file, delete the old one, rename; the failure is error Setup.cpp 11431 (0x82f626). Setup.cpp 9090 (0x82ce4a) is the failure to unpack the `.tmp` files. The most common cause of a "missing" save is Windows VirtualStore.

**Free space in the exe:** end of `.text` (0x9660e5..0x966fff); `cc` at 0x953472.. and 0x953fb3..; "Tapani Space" (0x601a4d, compacted by the patches); the NickPatcher extra section (2 MB at 0xde7000), which is why addresses from patched exes do not match.

Thanks to the thread authors: Tapani, John Locke, saturn, Golly, Hugo9, MadScientist, Nick (nckstwrt), Craig Forrest, djole2mcloud, giovanisantana29, xeno, Patinoz, Zozoa, Dmitri, Fiestita, Kingsley and the others; and to whoever keeps the archive at nic.opik.net.
