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
| +0x53 | nation (pointer) | `forca_do_time` (adjustment by competition) |
| +0x80 [2] | reputation | `forca_do_time` (home × away and the 2500 limit) |

## Global variables

| Address | What it is | Where it appears |
|---|---|---|
| 0xAE234C..0xAE2394 | counts of each table (continents, nations, cities, stadiums, clubs, people, players, competitions...) | — |
| 0xAE2378 | number of club competitions | 0x5c6710, called by `forca_do_time` |
| 0xAE23A4..0xAE23EC | pointers to the start of each table (clubs at 0xAE23B4, people at 0xAE23BC, players at 0xAE23C4...) | — |
| 0xAE2C90 / 0xAE2C92 [2] | current day of the year and year | the simulation stops at the requested date by reading it here (`simulacao.rs`) |
| 0xDE1F58 | pointer to the UEFA country rankings | next to the random generator state (0xDE1F5C, 8 bytes), which the comparison checks |
| 0xDE4400 | the game's `HeapCreate` handle | replaced by the simulation's fixed allocator (`alocador.rs`) |
| 0xDE4404 | 1 if the game recognised the Windows version | — |
