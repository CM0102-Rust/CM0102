*English version of [CAMPOS.md](CAMPOS.md). Both are kept in sync; the Portuguese file is the original.*

# Fields of the game's records used by the routines

Names of the database fields that the rewritten routines read, cross-referenced with the community index **"[DOCUMENTATION] Offset Index: Decoding OllyDbg"** (champman0102.co.uk forum, topic 2802, maintained by Xeno, with findings by John Locke, Tapani, Saturn and others; updated on 2023-02-16): <https://champman0102.co.uk/viewtopic.php?t=2802>. The English names in parentheses are the ones from that index; most of them match SI's original header (`database.h`, in `referencias/`, outside git), which gives the same field order.

Offsets are in hexadecimal from the start of each record; the size in bytes is in brackets when it is not 1.

## Player (player record, `CM3_PLAYERS`)

In the in-match player object (block of 20 per side in the engine), the pointer at **+0x6d** points to this record. In the routines, the variable is called `pessoa`.

| Offset | Field | Routines |
|---|---|---|
| +0x05 [2] | current ability (currentAbility) | `forca_do_time` |
| +0x1b | acceleration | `forca_do_time` (added to pace) |
| +0x1e | anticipation | `forca_do_time` |
| +0x21 | consistency | `sorte_do_dia` in `forca_do_time` (draw(consistency) against draw(20)), `escolher_por_papel_c`, `escolher_por_papel_e` |
| +0x26 | dribbling | `forca_do_time`, `pontuar_casa` |
| +0x2b | heading | `forca_do_time` |
| +0x2e | jumping | `forca_do_time` |
| +0x2f | leadership | `forca_do_time` |
| +0x30 | left foot (leftFoot) | `apontar`, `classificar_lance` |
| +0x31 | long shots (longShots) | `forca_do_time` |
| +0x32 | marking | `forca_do_time` |
| +0x33 | off the ball (offTheBall) | `forca_do_time` |
| +0x35 | one on ones (oneOnOnes) | `forca_do_time` |
| +0x36 | pace | `forca_do_time` |
| +0x37 | passing | `forca_do_time` |
| +0x3b | right foot (rightFoot) | `apontar` (the direction gets 2 × (right foot − left foot)), `classificar_lance` |
| +0x3c | stamina | `forca_do_time` (fatigue uses 30 − stamina) |
| +0x3e | tackling | `forca_do_time` |
| +0x3f | teamwork (teamWork) | `forca_do_time` |
| +0x40 | technique | `forca_do_time` |
| +0x43 | creativity ("Vision" in SI's header) | `forca_do_time` |
| +0x44 | work rate (workrate) | `forca_do_time`, `escolher_por_papel_b` |

## Person (person record, `CM3_STAFF`)

In the in-match player object, the pointer at **+0x69** (the "ficha", record).

| Offset | Field | Routines |
|---|---|---|
| +0x5d | temperament | `aplicar_fator` and `atualizar_jogador`: condition (+0x1a2) does not drop below 2 × (25 − temperament) |
| +0x65 | preferences (pointer): +0x1c, +0x20, +0x24 people liked; +0x28, +0x2c, +0x30 people disliked | `gosta_de`, `nao_gosta_de` |

## Referee (official record, `CM3_OFFICIALS`)

In the match refereeing object (`match_official.cpp`), the pointer at **+0x04**.

| Offset | Field | Routines |
|---|---|---|
| +0x24 | allowing flow (allowingFlow) | `cartao_pela_gravidade` |
| +0x25 | discipline | `cartao_pela_gravidade` |
| +0x28 | refereeing | `decidir_falta`: the referee lets the foul go when draw(refereeing + 15) = 0 (the higher the attribute, the rarer the mistake) |
| +0x29 | running the line (runningTheLine) | `procurar_impedimento`: the offside goes unflagged when draw(running the line + 25) = 0 |

## Club (club record, `CM3_CLUBS`)

| Offset | Field | Routines |
|---|---|---|
| +0x53 | nation (pointer) | `forca_do_time` (national league style, see Nations) |
| +0x80 [2] | reputation | `forca_do_time` (home × away and the 2500 limit) |

## Global variables

| Address | What it is | Where it appears |
|---|---|---|
| 0xAE234C..0xAE2394 | counts of each table (continents, nations, cities, stadiums, clubs, people, players, competitions...) | — |
| 0xAE2378 | number of club competitions (the last 127 are friendlies) | `e_amistoso` (0x5c6710), called by `forca_do_time` |
| 0xAE235C / 0xAE2350 | total clubs (domestic + 2 × nations) / nations | `e_selecao`, `e_segunda_selecao` |
| 0xAE2364 | number of people (the last 16 ids are the human managers) | `contrato_de`, `tecnico_9a01`, `tecnico_9a05` |
| 0xAE2D0C | contract index table per person (79 bytes per person) | `contrato_de` |
| 0xAE23A4..0xAE23EC | pointers to the start of each table (clubs at 0xAE23B4, people at 0xAE23BC, players at 0xAE23C4...) | — |
| 0xAE2C90 / 0xAE2C92 [2] | current day of the year and year | the simulation stops at the requested date by reading it here (`simulacao.rs`) |
| 0xDE1F58 | pointer to the UEFA country rankings | next to the random generator state (0xDE1F5C, 8 bytes), which the comparison checks |
| 0xDE4400 | the game's `HeapCreate` handle | replaced by the simulation's fixed allocator (`alocador.rs`) |
| 0xDE4404 | 1 if the game recognised the Windows version | — |

## Match (match object, `MATCH_DATA`)

In the engine, the pointer at **+0x4792**. Fields from the structure davdav published in topic 10794 (the unknown ones left out), checked against what `forca_do_time` reads:

| Offset | Field | Routines |
|---|---|---|
| +0x00 | competition id | — |
| +0x0C / +0x10 | home / away ids | — |
| +0x14 | competition (pointer) | `forca_do_time` (competition nation at +0x5d; 0x5c6710) |
| +0x1C / +0x20 | home / away (club pointers) | `forca_do_time` (nation and reputation; side 0 is the home team) |

The same object holds the dates (year/day words), the stages and the score; according to the topic, also the stadium, and the engine reads it a lot.

## Nations (constants at 0x9CF200..)

Each nation in the database has a fixed global pointer in this area (0x9CF200 Afghanistan ... 0x9CF4A8 Sweden; continents at 0x9CFA08..). Source: NickDLL's `Helpers/9cf_constants`, maintained by davdav (dm-carmo, <https://github.com/dm-carmo/NickDLL>; the name comes from Nick+Co, who had the initial idea, a different person; no license declared: facts only, no code copied).

In `forca_do_time`, in a domestic competition (the competition's nation, +0x5d, is the home club's nation, +0x53), word 0x1de of the strength is multiplied by the national league's style:

| Nations | Factor |
|---|---|
| Germany (0x9CF314), Scotland (0x9CF45C), Belgium (0x9CF244) | 1.15 |
| Sweden (0x9CF4A8) | 1.25 |
| Norway (0x9CF408), England (0x9CF2E4) | 1.05 |
| Netherlands (0x9CF338), Denmark (0x9CF2C0), Portugal (0x9CF434), France (0x9CF300) | 1.10 |

### The whole 0x9CF*** area (topic 2801)

The topic "[DOCUMENTATION] The [9CF***] Thread" (champman0102.co.uk forum, topic 2801, John Locke and contributors, updated on 2022-01-15) lists the whole area of fixed pointers the game fills in when it loads the database. Ranges (check each address in our disassembly before using it):

| Range | What it is |
|---|---|
| 0x9CF178..0x9CF1FC | colours (34) |
| 0x9CF200..0x9CF544 | nations (Brazil 0x9CF260, Argentina 0x9CF218, England 0x9CF2E4...) |
| 0x9CF548..0x9CFA04 | competitions (e.g. Brazilian first division 0x9CF9EC, Libertadores 0x9CF63C, friendly 0x9CF668, World Cup 0x9CF79C) |
| 0x9CFA08..0x9CFA1C | continents |
| 0x9CFA20..0x9D00A0 | awards |
| 0x9D00A4..0x9D0524 and 0xB63C9C..0xB63D00 | clubs named in the code (e.g. Flamengo 0x9D0358); outside the area: Falkirk 0xA8EB48 and four Irish clubs at 0x9E9FC0..0x9E9FF4 |

The same topic lists the decimal competition ids (0 = Belgian First Division ... 432 = Belgian Fourth Division D); sometimes the executable uses the id instead of the pointer.

Use already seen in the match engine, not yet rewritten: 0x6c6a30 (match_eng.cpp) compares the nation linked to the ball holder's record (+0x69, +0x1a) and picks a float factor: 1.0 for England, Scotland, Wales, Northern Ireland, Ireland and Norway; 0.6 for Sweden, Denmark and Germany; 0.2 for the others.

## Tactics: squares, formation and preferences (topic 2799)

Source: topic "About AI Manager preferred formations & AI tactic selection" (champman0102.co.uk forum, topic 2799, Xeno, MadScientist and John Locke, 2021). Checked against what we have already rewritten.

**Match-engine tables A and B** (`tabela_a` 0x6C38C0 and `tabela_b` 0x6C38F0, by tactical position 0..10 at motor + 0x8EBC/0x8EC4 + 4 × side): A tells the player's square in the tactic, B where his arrow points (same as A with no arrow). Bits: sector 1 goalkeeper, 2 sweeper, 4 defence, 8 defensive midfield, 0x10 midfield, 0x20 attacking midfield, 0x40 attack; side 0x80 left, 0x100 centre-left, 0x200 centre, 0x400 centre-right, 0x800 right. It matches what we already use (`marcador_direto` mirrors sector and side; `escolher_cobrador` tests 0x40; `marcado_designado` compares A with B).

**Formation mask** (side's formation + 0x187B, 32 bits): one bit per square of the pitch, 31 squares: 0 goalkeeper; 1..5 sweeper line; 6..10 defence; 11..15 defensive midfield; 16..20 midfield; 21..25 attacking midfield; 26..30 attack (in each row, right to left, seen from the own goal). E.g. 0x281B06C1 = 4-4-2. In `marcado_designado`, bit 0x10000000 (square 28: centre forward) of the opposing formation is one of the tests. 0x5B59A0 (formation.cpp) builds the mask from the 11 squares (0x5B5A60, `bit_da_casa`) and replaces some masks with the nearest preferred formation's (4-5-1, 3-4-3, 5-3-2, 3-5-2).

**Managers' preferred formations** (0x896184..0x8961BC, nine constants in the order of the editor's list): 5-3-2 with sweeper 0x280E8A89, 5-3-2 0x280E8B81, 4-4-2 0x281B06C1 (also the default), 4-3-3 0x381506C1, 4-2-4 0x2A2A06C1, 3-5-2 0x281B2381, 4-1-2-1-2 0x288A26C1, 4-5-1 0x101F06C1, 3-4-3 0x381B0381. The computer's tactic `.pct` files are loaded at 0x893DC0 (28 plus 17, six repeated).

## Competitions (for the competitions stage)

Not yet used by rewritten routines; recorded for stage 2 (competitions, tables, calendar). Source: the "Modern Champions League format for CM 01/02" topic (champman0102.co.uk forum, topic 10794, tafo and davdav, July 2026); check each one against our own disassembly before use.

| Where | What it is |
|---|---|
| stage +0xB1 | league table: one 0x3B-byte row per team (+0x00 club, +0x04 position, +0x11 played, +0x15/+0x17 goals for/against, +0x19 points, +0x37 fate, +0x39/+0x3A best and worst reachable position) |
| stage +0xA7 [2] | games needed per team (computed once and stored in the save) |
| stage +0xC5..+0xC8 | tiebreak rules |
| stage +0xC9 | stage end date (the stage ends by date, compared with 0xAE2C90/0xAE2C92) |
| stage +0xDD | stage complete |
| 0x6827D0 | sorts the stage table with its tiebreakers |
| 0x51C410 / 0x522610 | cup round setup / seeded draw |
| 0x6844B0 | builds the fixture plan of a round-robin stage: per team and matchday, a signed code (opponent 1..N; positive at home, negative away) |
| 0x683AD3 / 0x683E20 | number of matchdays derived from the team count (N − 1) |
| 0x9086E0 | hand-built fixtures (MLS); 0x68B100 creates the stage, 0x85C260 and 0x5AA680 register each match |
| 0xADADFC | competition by id (id × 4) |
| stage +0x52 | stage flags; bit 2 turns on fixed squad numbers (tested in squad_manager.cpp, 0x859DF4); each competition stores its value when it is built (e.g. 2 for the Brazilian league, 0x42A621). Source: topic 1540, page 2 |

## Finances and club facilities (for future stages)

Source: topic "Miscellaneous CM0102 Patches & Fixes by melanzalko" (champman0102.co.uk forum, topic 10877, melanzalko and davdav, August 2026). The topic mixes file offsets and VAs; below, everything is a VA, checked in our 3.9.68 disassembly.

| VA | What it is (original) |
|---|---|
| 0x461531, 0x461551, 0x461571, 0x461591 | thresholds of the training facilities text: 400, 600, 800, 1000 ("Good facilities" up to 400, then "Adequate", "Need some", "Need major investment") |
| 0x5986B0 | financial status for the board: "Rich" if the balance exceeds (25 × rep + 50,000) × 100 (lea with 6000 and 2000 at 0x5986EB/0x5986F2); "Secure" with 10,500 instead of 6000 (0x598713) |
| 0x59714D, 0x597A8E | "Dividends" brackets (in practice a tax on revenue): 7750 (0x1E46) and 7250 (0x1C52), compared with word +0xB |
| 0x8EDB67 | cap of the next-sale percentage in negotiations: 25 (byte 0x19) |
| 0x98F9B0 | text "  Dividends" on the finance screen |

**Competition prize and TV money** (topic "Offsets", champman0102.co.uk forum, topic 1540, 2020): the amounts in pounds are immediate constants in each competition's code; the topic lists the file offsets (VA = offset + 0x400000) for leagues, cups and European tournaments. Checked: Brazilian first division, TV 1,000,000 at 0x42A461; second division, TV 500,000 at 0x42CA51; Premier League, prize 250,000 at 0x574B1D and TV 12,000,000 at 0x574B40; Champions League, winner 4,000,000 at 0x584539.

Not used yet: the match engine does not touch finances. They are for the stage of the simulation outside the match.
