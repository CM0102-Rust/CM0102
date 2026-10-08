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
| +0x04 | stadium (id; pointer at +0x18) | — |
| +0x43 / +0x44 | home / away goals at the end (0x6BDA86 copies them from the engine's 0xF5BD/0xF5F3) | — |
| +0x30 / +0x32 | round name / prefix (codes of comp_util.cpp's name table, 0x4B4850) | `placar_agregado` (units digit of +0x30), `nova_reputacao` and `reputacao_do_jogador` (larger weights in quarter finals, semi finals, finals and groups A to H) |

The same object holds the dates (year/day words), the stages and the score; according to the topic, also the stadium, and the engine reads it a lot.

**Round codes** (topic 1540, page 3, Saturn's explanation of the cups' round blocks; checked in the name table 0x4B4850). Each cup round has a 0x68-byte record (+0x07 name, +0x09 number, +0x0B draw type, +0x0D/+0x0F/+0x11 tiebreak of game 1/2/3, +0x18 teams, +0x1A ties, +0x1C new teams, +0x20 replays, +0x21 legs, +0x22 days between them, +0x5C/+0x60/+0x64 prize money; the rounds follow each other every 0x68, as the French Cup at 0x5BA4EA..0x5BA6C9 shows, page 16). The name is base + variant: base 0x0A first round, 0x14 second... 0x78 quarter final, 0x82 semi final, 0x8C third place, 0x96 final, 0xA0 playoff, 0xDC qualifying round; variant 0 single game, 1 replay, 2 second replay, 3 first leg, 4 second leg, 5 decider. Prefixes: 0x41F..0x426 = groups A to H (0x418..0x41E: playoff groups, autumn and spring seasons, "Group Stage", northern and southern sections).

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

## Rules (for future stages)

Source: topic 1540, page 5 (Xeno, giovanisantana29 and others). Checked in 3.9.68:

| VA | Rule |
|---|---|
| 0x7DD212 (and 0x7DF766, 0x7DB019) | a suspension starts 14 days later (`push 0xE`): England, Ireland and obscene gestures |
| 0x8C31D2 | UK work permit: 18 + a draw of days (`add al, 0x12`, 18 to 25) |
| 0x8C7BE4.. | UK work permit rule (topic 1540, page 10; checked): granted with the player's world reputation (+0x0D) above 8500, the contract's weekly wage (+0x0C) above 34,999, under 24 (person +0x18) with potential (+0x07) above 190, more than 20 caps (person +0x22) with the nation (person +0x1A) of reputation (+0x8E) above 3000 and under 32, or more than 70% of the games (0x8C7D9E); for non-players, world reputation above 160 (0x8C7A94) |
| 0x579C63 | limit of 3 non-EU foreigners in England (`mov byte [edx+5], 3`) |
| 0x96D808 | constant 71.0 (float) compared in match_pl.cpp (0x706D1B, 0x706DEE, routine not yet rewritten); according to the community, the percentage of shots on target that do not go in (the more-goals patch changes it to 65); topic 1540, pages 21 and 22 |
| 0x708A31 | the goal goes into the score: adds 1 to the side's goals byte (motor + 0xF5BD + 54 × side, the same one `placar_agregado` reads); page 24. The community also mentions a "brake" after 5 goals, not located yet |
| 0x6BD98F | length of the first half on the engine clock (0x8ED4): 0x1EF = 495; the match ends at 0x3DE = 990 (the two limits `momento_marcado` and `placar_agregado` test); topic 1540, page 25 |
| 0x6B5B41.. | the match day looks for the next match in the list at [0xB67A50] + 0x19 (0x69-byte records; the match object starts at +4) |
| 0x4CF5F0 | 80-byte (person, club) record, by the index at [0xAE2D0C] + 79 × person id, in [0xAE1050]; used by the engine (e.g. the captain's influence) |
| stage +0xA7 (cups) | list of entering teams, 6-byte slots (+0 club, +4, +5); the Northern Irish Cup setup (0x78AC90) first places 20 first- and second-division teams and then 12 from the lower ones (page 26) |
| 0x54CEDB | leap year (divisible by 4, except centuries not divisible by 400) |
| 0x642A70.. | real first-season fixtures for the Italian divisions (A 0x642A70, B 0x647230, C1 and C2 up to 0x65C8E0) |
| 0x43F7F0 | currencies: per currency, the rate against the pound (double at 0xAD9CB8), the transfer fee list multiplier (0xAD9DA0) and the +/− button step (0xAD9CC0) |
| 0x5999A0 / 0x5A0590 | gives money to a club (thiscall on the club's finances, argument the amount); 0x5A0590 finds the club's finances (e.g. 1,800,000 for reaching the Champions League group stage, 0x585803) |
| [0xDE1F58] | UEFA coefficients: per nation, pointer, coefficient (float) and the Champions League, UEFA Cup and Intertoto places (page 28) |
| 0x54AC4F | weekday from the year (0xAE2C92), the day of the year (0xAE2C90) and the leap year flag (0xAE2C94) |
| 0x8CFA1E | limit of non-EU players in a match (the jump that applies it) |
| 0x8C34E0 | carries out a transfer: in the record received, +8 buying club id, +0xC player id; club table at 0xAE23B4 and person table at 0xAE23BC (page 23) |
| 0x43F2C8 | foreigners in the Brazilian league: 2 per game (`mov byte [edx+2], 2`; topic 1540, page 18) |
| 0x533080 | without real names, players swap clubs in drawn pairs (16 × the count, factor at 0x969508); some stay where they were (page 19) |
| stage +0x3E | number of teams in the league (e.g. 18 in the Welsh league, 0x925B42; 14 in the Greek one, 0x5EAD72, which reserves 59 bytes per team, the table row; pages 19 and 20) |
| 0x53D900 | the nations that enter the Oceania cups (New Zealand, Papua New Guinea, Fiji, Solomon Islands, Vanuatu; Australia and New Zealand are added separately); page 20 |
| 0x868307 | a contract becomes unprotected after 2 years (+1 under 28) (`add ebx, 2`) |
| 0x76D4B0.. | national team squads: up to 23 in official tournaments (senior and under-21), 26/22 in the other case, 26 and minimum 22 in friendlies and qualifiers (topic 1540, page 16) |
| 0x76CC1F.. | call-ups: friendlies, B team and under-21 follow their own rules; players outside their nation's continent (+0x71) are only called if their club's league reputation (+0x69) exceeds a fraction (0x968E40) of the competition's reputation (topic 1540, page 29) |
| stage +0x3C | number of league rounds (read at 0x683E0E) |
| 0x85EB91 | cap on a contract wage (the topic gives the file offset, 0x45EB91): topic 1540, page 13, speaks of current reputation × 750; the code compares with a value × 15 at 0x85EB84 (another scale, to be checked). The same page shows swapping Northern Ireland for Austria: dozens of places in the code with the nation pointer (0x9CF3E4) and each league id written by hand, an example of why competitions are changed by patch today |
| 0x60E100.. | the nations' 0x9CF*** pointers are filled by name when the database is loaded (e.g. "Holland" → 0x9CF338); topic 1540, page 14 |
| 0x92EB46.. | 2002 World Cup (`cmp word [edi+0x40], 0x7D2`) with the 32 teams and the groups written in the code, nation by nation |
| 0x7ABCDF | generated player (regen): 1 chance in 4 of changing nationality, drawing among the nations (count at 0xAE2350, array at 0xAE23A8, 290-byte records) and only accepting the drawn one if it has fewer than 100 people (nation +0x88); topic 1540, page 12 |
| 0x5265F8 / 0x536A1D | a new player becomes a "wonderkid" with 1 chance in 100 (potential −2, 0xFFFE at +0x07); potential −2 (and −1) is resolved at 0x536A1D when the game is created |
| 0x53D7F0 / 0x8CEBB5 | does the club only sign Basques? / the nation used in that restriction (0x9CF424, Basque Country); topic 1540, page 8 |
| 0x83A1D5 | computer clubs' shortlists at the start of the game (skipping the jump, none until the season update) |
| 0x5E069A | Copa América by the parity of (year − 1996); the year of the next edition is at competition +0x40 |

## Other points (topic 1540, pages 6 and 7)

Checked in 3.9.68 where stated:

| Where | What it is |
|---|---|
| competition/stage +0x49, +0x4A | named substitutes and substitutions allowed (4 bits each); a competition method (around 0x48F2FA, with a competition-specific version in vtable +0x7C) copies both into word +0x4D of the object it receives (probably the match). The French Cup stores its own at 0x5BA19B; the Italian Serie A, 7 and 3 (0x6416D6); topic 1540, page 11, lists these values for each competition. Checked |
| transfer rules (e.g. italy_rules.cpp, 0x6613F0) | windows in 6-byte records: type (0x0E), ?, day of the week (0xFF = the date itself), day, month (0 = January), ?; topic 1540, page 11 |
| 0xA158B0 | text "Only %d home players" of match_eng.cpp (lines 612 and 652): the match flags a team short of players |
| 0x68A160, 0x68A1C0 | league calendar. 0x68A160(stage, round, day, month, season year 0/1, weekday 0 = Monday, time 0 morning/1 afternoon/2 evening, ?, venue 0 = home team's ground): the round falls on the weekday nearest the date (2001-02 calendar). 0x68A1C0(stage, round, TV game number, weekday, time, ...) marks the televised games; the round's last call gives how many. E.g. Italian Serie B at 0x646574, Premier League at 0x57379D; topic 1540, page 17. Checked |
| 0x585498..0x5857C6 | Champions League group draw: 32 nine-byte "pairs" (club, club from the same country or 0, drawn half 0/1/2); pot by pot, it draws the pair (`draw(8)`), the half (A–D or E–H, by the slots left) and the first free group; the same-country partner goes to the other half; at the end it swaps clubs between groups of the same half until no group has two from the same country (+0x53). Result: 32 pointers, 4 per group (page 27) |
| match venue (last argument of 0x68A160) | 0 first drawn team's ground, 1 and 0x10 venue unknown, 4 national stadium (or the country's largest), 5 largest stadium, 8 neutral, 0x2A halfway between the two (30 to 50 thousand), 0x40..0x4000 the 1st to 9th largest stadium, 0x8000 Stade Louis II (community observations; e.g. German Cup final at 0x5D92B9) |
| 0x5836C4, 0x632E30 | start year (2001, `cmp ax, 0x7D1`) for the previous year's international and European awards. Checked |
| 0x5E4800 | builds an interface colour from three components (blue, green, red pushed before the call); the interface has dozens of these calls. Checked |
| nation +0x11C | league selected (`NationLeagueSelected` in `database.h`); 0xAE2D04 seems to be the number of selected leagues (not checked) |

## Known defects of the original (topic 1540, page 9)

Reproduced on purpose while the goal is fidelity; candidates for fixing in the modernization stage. Checked in 3.9.68:

| Where | Defect |
|---|---|
| 0x411D82 (Australian league) | the competition is not reset at the end of the season (−1 is never written to stage +0x30): the same champion every year. And the opening/closing split comes out 36/5 games instead of 24/10 (stage +0x3C) |
| 0x797474, 0x40AEDE, 0x40CFCE | Oceania, Asian and African Champions League (or Asian Cup Winners' Cup) with a third-place playoff (3 final rounds), sometimes scheduled after the final, which can stall the competition |
