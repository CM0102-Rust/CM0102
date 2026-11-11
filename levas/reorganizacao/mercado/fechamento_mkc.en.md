*English version of [fechamento_mkc.md](fechamento_mkc.md). Both are kept in sync; the Portuguese file is the original.*

# Closing the market map, reader mkc: the loan rules of the nations outside the 8 leagues (11/11/2026)

Routines: the loan and window rules that section C of the map (`docs/MAPA_MERCADO.en.md`, §1, §3, §4) gave as "only
mapped": Finland 0x5a9a80/0x5a9af0/0x5a9ba0, Norway 0x794760, Turkey 0x902240, Northern Ireland 0x793870/0x793900,
Russia 0x7ec8e0, Wales 0x90f1a0, Germany 0x5e0210, Ireland 0x63c530/0x63c4e0. Everything below comes from the
disassembly of `jogo/cm0102.exe` (`desmonta.py`), with the instruction address. For each claim of the map: the old
text, the new text and the proof. The main session replaces [inferred] / "only mapped" with [proved] and fixes the
"Item" column of §1.

Items: **mkc_3** (0x90f1a0, 0x7ec8e0, 0x902240; the 8 leagues' LIGAS, because **they run there**), **mkc_4** (Finland),
**mkc_5** (0x793870, 0x793900, 0x794760), **mkc_6** (0x5e0210, 0x63c530, 0x63c4e0); the last three with LIGAS =
`ENG FIN NOR NIR GER IRL`. All compiled in a copy (with mkc_1 and mkc_2), mutations compiled, nothing run in the game.

## 0. The list checked against the code: the 29 rules vtables

The map's list came from the community map (`*_RULES::` names). Checked in the code: every rules vtable has 0x40a5f0 in
slot +0x14; scanning `.rdata` for that dword finds **29 vtables** (28 nations + the base), each written by a single
constructor (`refs.py`; 15 of them by the constructors already in Rust, `construct_nation_rules`, the others by the
community map's names of the constructors 0x5c1570 France, 0x7ec770 Russia, 0x555460 Denmark etc.). The loan slots (+0
`daily_update`, +8 `record_loan`, +0x20 `CAN_LOAN_PLAYER`, +0x24 `CAN_END_LOAN`, +0x28 `GET_LOAN_PERIOD`; "base" =
0x40a600 / 0x40a610 / 0x8e8bd0, "none" = 0x583470):

| Vtable | Nation | +0 | +8 | +0x20 | +0x24 | +0x28 |
|---|---|---|---|---|---|---|
| 0x9674a0 | Argentina | 0x40a710 | none | base | base | base |
| 0x967830 | Australia | none | none | 0x412dd0 | base | **0x90f1a0** |
| 0x967c10 | Belgium | none | none | base | base | 0x90f1a0 |
| 0x968838 | Brazil | none | none | base | base | **0x7ec8e0** |
| 0x969394 | Croatia | none | none | 0x5ed060 | base | 0x90f1a0 |
| 0x969860 | Denmark | none | none | 0x793900 | base | base |
| 0x969f4c | England | 0x57a020 | 0x579fd0 | 0x579d60 | base | 0x579c70 |
| 0x96a188 | Europe (generic) | none | none | base | base | base |
| 0x96a75c | Finland | none | none | 0x5a9af0 | 0x5a9ba0 | 0x5a9a80 |
| 0x96ae48 | France | 0x5c1710 | none | **0x412dd0** | 0x5c1ed0 | **0x90f1a0** |
| 0x96b3cc | Germany | none | none | base | 0x5e0210 | 0x90f1a0 |
| 0x96b784 | Greece | none | none | 0x5ed060 | base | 0x90f1a0 |
| 0x96ba88 | Holland | none | none | 0x794760 | base | 0x90f1a0 |
| 0x96c190 | Ireland | **0x57a020** | 0x63c4e0 | 0x63c3d0 | 0x63c530 | 0x90f1a0 |
| 0x96c96c | Italy | none | none | 0x661770 | 0x6618f0 | base |
| 0x96ccfc | Japan | none | none | base | base | base |
| 0x96cfec | Korea | none | none | 0x793900 | base | 0x90f1a0 |
| 0x96df9c | Northern Ireland | none | none | 0x793900 | base | 0x793870 |
| 0x96dfe4 | Norway | none | none | 0x794760 | base | base |
| 0x96e61c | Poland | none | none | 0x5ed060 | base | 0x7ec8e0 |
| 0x96e9c0 | Portugal | none | none | base | base | base |
| 0x96f340 | Russia | none | none | 0x412dd0 | base | 0x7ec8e0 |
| 0x96f838 | Scotland | none | none | 0x7f6650 | base | 0x7f6530 |
| 0x96fdec | Spain | none | none | 0x859630 | base | base |
| 0x970368 | Sweden | none | none | base | base | base |
| 0x970458 | base `TRANSFER_RULES` | none | none | base | base | base |
| 0x97075c | Turkey | none | none | 0x902240 | base | 0x90f1a0 |
| 0x970a38 | USA | none | none | **0x902240** | base | base |
| 0x970a9c | Wales | none | none | 0x793900 | base | 0x90f1a0 |

What changes: several "one nation" routines belong to several; and **three run in the 8 leagues**.

## 1. The routine table (§1 of section C)

| Map claim | New text | Proof |
|---|---|---|
| 0x5a9a80 / 0x5a9af0 / 0x5a9ba0 Finland: "out: does not run in the 8 leagues" | **[proved]**: only the Finnish vtable 0x96a75c points to them; as the borrower's rule they do not run in the 8 leagues (the `can_end_loan`, the owner's rule, only if the AI asks to borrow a player of a Finnish club). Item **mkc_4** | §0 |
| 0x794760 / 0x902240 Norway, Turkey: "out (same)" | **fixed [proved]**: 0x794760 belongs to Norway **and Holland** (item **mkc_5**); 0x902240 to Turkey **and the USA**: it **runs in the 8 leagues** (an American club borrowing). Item **mkc_3** | §0 |
| 0x793870 / 0x793900 / 0x7ec8e0 / 0x90f1a0: "out (same)" | **fixed [proved]**: 0x793870 only Northern Ireland; 0x793900 Denmark, Korea, Northern Ireland and Wales (**mkc_5**); 0x7ec8e0 **Brazil**, Poland and Russia and 0x90f1a0 **11 nations**, among them **France and Australia**: both **run in the 8 leagues** (**mkc_3**) | §0 |
| 0x5e0210 / 0x63c530 / 0x63c4e0 Germany, Ireland: "out (same)" | **[proved]**: only their own vtables (0x96b3cc, 0x96c190). Item **mkc_6** | §0 |
| (new) | 0x412dd0 (`AUSTRALIA_RULES::can_loan_player`, mkc_2) is also the `CAN_LOAN_PLAYER` of **France** and Russia: in the 8-league career it also runs for French clubs **[proved]** | vtables 0x96ae48 +0x20, 0x96f340 +0x20 |
| (new) | 0x5ed060 (Greece, also Croatia and Poland) is `mov eax, 1; ret 0x10` (8 bytes): nothing to port **[proved]** | 0x5ed060..0x5ed065 |
| (new) | Ireland uses England's 0x57a020 (already in Rust) as its `daily_update` (+0): the Irish clubs' season loan count is also zeroed on 20 June **[proved]** | vtable 0x96c190 +0 |

## 2. The period (§3)

Common signature (0x8d0590's): thiscall(rules, club, a2, &start_month, &start_year, &end_month, &end_year, player),
`ret 0x1c`. Common part: start = today's month (0x54cfb0 on 0xae2c90) + k; year = [0xae2c92]; start > 11 (signed): −12
and year + 1; end month = `contract_manager_season_end_month(club)` (0x4d45f0 with ecx = [0xae1050]); start < end
(signed): end year = start year, otherwise today's year + 1. `a2` and `player` are not read.

| Map claim | New text | Proof |
|---|---|---|
| Finland 0x5a9a80: "as the base" | **[proved]**: k = 1, the same computation as 0x8e8bd0 (which zeroes the month instead of subtracting 12: same result, the maximum is 12) | 0x5a9a80..0x5a9aeb |
| Russia 0x7ec8e0: "start in 3 months, end at season end" | **[proved]**: k = 3; it serves **Brazil**, Poland and Russia. **Latent defect [proved in the code]**: with the start wrapping into next year (today October to December) and the season end month ≤ the start month (a season ending in January or February), the end lands in the start's year and before it (or on it). Whether this happens with these nations' data was not checked | 0x7ec8f4 (`add al, 3`), 0x7ec924..0x7ec947 |
| Northern Ireland 0x793870: "the same, and the start never passes the end" | **[proved]**: k = 3 and then, a start after the end (year unsigned, then the month signed) becomes the end. It is Russia's routine with the fix of the defect above. Only Northern Ireland | 0x793885, 0x7938d7..0x7938f3 |
| Wales 0x90f1a0: "start = end (season end)" | **[proved]**: k = 1 and then start = end (month and year). It serves **11 nations**: Australia, Belgium, Croatia, France, Germany, Greece, Holland, Ireland, Korea, Turkey and Wales. Consequence **[proved]**: in `can_loan_player` (0x8d0040, §4 step 5) start = end returns 1 at once, without asking the owner's `CAN_END_LOAN`, unless 0x8d0590 changes the end through the contract (§3 steps 2–4); in the AI (0x842740) the months to the start and to the end are the same | 0x90f1b4, 0x90f1ff..0x90f206 |

## 3. Can he be borrowed? (§4 step 3)

Signature: thiscall(rules, club, player, message, &reason), `ret 0x10`.

| Map claim | New text | Proof |
|---|---|---|
| "base 0x40a600 / Greece 0x5ed060: 1; Northern Ireland 0x793900: 1 (writes [0xae1080])" | **[proved]**, with the nations: base for Argentina, Belgium, Brazil, Europe, Germany, Japan, Portugal, Sweden; 0x5ed060 for Croatia, Greece, Poland; 0x793900 for **Denmark, Korea, Northern Ireland and Wales**: looks up the player's main contract (0x4cf5a0), keeps it in [0xae1080], computes its club and does not use it; 1 | 0x793900..0x793934 |
| "Australia 0x412dd0, Norway 0x794760, Turkey 0x902240: outside the window, 9" | **[proved]**: all three: `find_open_window(today, 0)` (0x8e8e50); closed: with a message the text with the adjective of the rules' nation (+0xa, form 2), reason 9 (even without a message) and 0; open: 1. Differences: the Norwegian one (also **Holland**) uses the text 0x9cf088 (example "Dutch", same format); the Turkish one (also the **USA**), with the window open, looks up the contract and writes [0xae1080] before the 1 | 0x794760..0x7947cb; 0x902240..0x9022da (0x9022a2..0x9022b4) |
| "Finland 0x5a9af0: the same and, for an AI club, only between days 90 and 300 of the year" | **[proved]**: closed window as above (text 0x987d38). Open: with a club that is not human (`human_manager_is_human_club(club, 1, 1, 0, 0)` = 0, cdecl), today's day of the year (signed word at 0xae2c90; 0 = 1 January, table 0x9b8448) > 300 or < 90: **0 with no text and no reason**. 90 = 1 April, 300 = 28 October (31 March and 27 October in a leap year). With the window (16/11–15/8), the Finnish AI only borrows from 1 April to 15 August | 0x5a9b52..0x5a9b89 |

## 4. Can it end? (`CAN_END_LOAN`, §4 step 5)

Signature: thiscall(rules, club, player, message, date, flag), `ret 0x14`. The callers of 0x8d0530 checked: screen
0x8fa341 (flag 1, no message), the end-loan screen 0x8fbcf9 (**flag 0**, message 0xde1f64, today's date, club = the
player's `club_job`), AI 0x8429ae (flag 1); 0x8d0040 always with flag 1.

| Map claim | New text | Proof |
|---|---|---|
| "Ireland 0x63c530: the date inside a window of the nation" | **[proved]**: a null date = today; `find_open_window(date, 0)` with the entry's ecx; closed: 0 and, with a message, the Irish adjective ([0x9cf2dc], form 10) in 0x9cd030 ("sent back", when `club` is the player's `club_job`) or 0x9ccfbc ("recalled"). The same code as France's (0x5c1ed0) with the Irish key. The Irish window: 10 May to 31 January | 0x63c530..0x63c5d9 |
| "Finland 0x5a9ba0: refuses between 18 August and 15 November" | **[proved]**, precisely: refuses with 18/8 of the date's year **before** the date and the date **before** 15/11 (strict at both ends; year unsigned, then the day signed; dates by 0x549f70(18, 7, year, −1) and (15, 10, year, −1)). Texts 0x9cc1a4 ("cannot be sent back until 15th November") / 0x9cc170 ("recalled"), no adjective. The date is **not checked** (a null one would crash; no caller passes null). The flag is not read | 0x5a9bb9..0x5a9ca5 |
| "Germany 0x5e0210: without the flag, refuses ending loans between German clubs (with the flag, 1)" | **[proved]**, precisely: flag ≠ 0: 1. Otherwise refuses when the player's club (`club_job`, +0x39) is German ([0x9cf314]) **or has no club or nation**; another country: 1. With a message, the German adjective (form 2) in 0x9cd5cc ("Players on loan from other German clubs cannot be sent back until the end of the loan.", when `club` = `club_job`, read again) or 0x9cd574 ("...cannot be recalled."). Since 0x8d0530 only calls the rule with the owner and the club in the same nation, it is "between German clubs". **Only the end-loan screen (flag 0) refuses**: in the loan check (0x8d0040) and the AI request Germany never refuses. The date is not read | 0x5e0210..0x5e02bf; flag 0 at 0x8fbce2 |
| (new) Ireland 0x63c4e0 `record_loan` | **[proved]**: the offer's club (id at +8 in the table [0xae23b4], 0x245 bytes) with a nation of id [0x9cf2dc] (Ireland): byte 0 of the 16-byte row `[[0xde1f54]+0x91c] + id·16` + 1. The same code as England's 0x579fd0 with the Irish key; it is the count the Irish `can_loan_player` (0x63c3d0) limits to 5, zeroed on 20 June by 0x57a020 (the Irish `daily_update`) | 0x63c4e0..0x63c521 |

## 5. §7 (the end-of-contract loan defect)

| Map claim | New text | Proof |
|---|---|---|
| "With a French/Italian/Irish/Finnish owner outside the window, 'This loan deal cannot go ahead'" | **fixed [proved]**: the Finnish one is not a window, it is the autumn (18/8–15/11, §4); the German one never refuses there (flag 1). And for borrowers under the Welsh period (11 nations, France and Australia included) the start comes equal to the end and the owner is only asked if 0x8d0590 moves the end through the contract | §2, §4 |

## 6. §11 (extension points)

Add to the row "rule vtables (+0x20, +0x24, +0x28)": the full table of §0 (the 29, with the nation). Swapping the slot
of a shared routine's pointer in one **vtable** changes only that nation (swapping +0x28 in France's vtable changes
only France); detouring the shared routine (0x90f1a0) changes all 11. The Irish season count lives in the same table
as the English one (`[[0xde1f54]+0x91c] + id·16`).

## 7. Defects found

1. **Russian period (Brazil, Poland, Russia)** — with the start in next year and the season ending in January or
   February, the end falls before the start (or on it). The Northern Irish copy of the same routine fixes it with "the
   start becomes the end"; the Russian one does not. Latent: it depends on these nations' season end month (not
   checked). **[proved in the code]**
2. **Germany without a club** — a player without `club_job` (or whose club has no nation) falls into the German refusal
   with the "recalled" text. In practice 0x8d0530 does not call the rule then (it requires the owner in the same
   nation). **[proved in the code]**
3. Not a defect, but a surprise: the Welsh period gives start = end for 11 nations, so for France and Australia (and
   the others) `can_loan_player` accepts without the owner's rule. **[proved]**
