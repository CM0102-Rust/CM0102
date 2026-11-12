# Batch 289 — the Finnish loan rules: the period, the deadline and the autumn without returns; the Northern Irish loan period and loan rule (also Denmark, Korea and Wales); the Norwegian loan deadline (also Holland); ending a loan in Germany and Ireland; the Irish season loan count

**Routines:**
- `finland_rules_get_loan_period()` = 0x5a9a80 (finland_rules.cpp, thiscall, ret 0x1c): rule: the loan to a Finnish club runs from next month to the season end month of the club's nation, this year if that month is still ahead of the start, otherwise next year (the same result as the base rule);
- `finland_rules_can_loan_player()` = 0x5a9af0 (finland_rules.cpp, thiscall, ret 0x10): rule: a Finnish club only takes a player on loan with the Finnish window open today (16 November to 15 August; outside it, reason 9 and "The deadline for loans to Finnish clubs has passed."); an AI club, moreover, only between days 90 and 300 of the year (1 April to 28 October), refused with no reason and no text;
- `finland_rules_can_end_loan()` = 0x5a9ba0 (finland_rules.cpp, thiscall, ret 0x14): rule: a loan with a Finnish club does not end (neither sent back nor recalled) on a date strictly between 18 August and 15 November of the date's year: "This player cannot be sent back/recalled until 15th November";
- `northern_ireland_rules_get_loan_period()` = 0x793870 (northern_ireland_rules.cpp, thiscall, ret 0x1c): rule: the loan to a Northern Irish club runs from today's month + 3 to the season end month of the club's nation (in the start's year if still ahead of it, otherwise the year after today); a start after the end becomes the end;
- `northern_ireland_rules_can_loan_player()` = 0x793900 (northern_ireland_rules.cpp, thiscall, ret 0x10): rule: a club of Denmark, Korea, Northern Ireland or Wales can always take a player on loan (no window, no limit); it only keeps the player's main contract;
- `norway_rules_can_loan_player()` = 0x794760 (norway_rules.cpp, thiscall, ret 0x10): rule: a club of Norway or Holland only takes a player on loan with a window of its nation open today; outside it, reason 9 and "The deadline for loans to ... clubs has passed.";
- `germany_rules_can_end_loan()` = 0x5e0210 (germany_rules.cpp, thiscall, ret 0x14): rule: a loan between German clubs cannot end early through the screen (flag 0): "Players on loan from other German clubs cannot be sent back until the end of the loan." for the borrower, "Players on loan to other German clubs cannot be recalled." for the owner; with the flag (the loan check and the AI request), it always can;
- `ireland_rules_can_end_loan()` = 0x63c530 (ireland_rules.cpp, thiscall, ret 0x14): rule: a loan with an Irish club only ends (sent back or recalled) on a date inside the Irish window (10 May to 31 January); outside it, "Players cannot be sent back/recalled ... until after the Irish transfer deadline";
- `ireland_rules_record_loan()` = 0x63c4e0 (ireland_rules.cpp, thiscall, ret 4): rule: each loan signed by an Irish club adds 1 to its season count (the Irish rule's limit of 5; zeroed on 20 June);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (7):** 7 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 cc7f6ccb, original d77c6c14 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 288's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 289`

**Fields touched:**
- transfer rules +0xa (NATION_ID): the rules' nation, for the adjective of the text; pointer; reads
- staff +0x39 (club_job): the player's club, to choose "send back" or "recall"; pointer; reads
- [0xae2c90] (G_CURRENT_DATE): today's day of the year (signed word, 0 = 1 January); reads
- transfer rules +0xa (NATION_ID): the rules' nation, for the adjective of the text; pointer; reads
- [0xae1080] (G_CONTRACT_LOOKUP_RESULT): the player's main contract, kept by the Northern Irish rule; writes
- staff +0x39 (club_job): the player's club, its nation (the German rule) and "send back" or "recall"; pointer; reads
- [[0xde1f54]+0x91c] + id·16 (SEASON_LOAN_COUNTS): byte 0, the club's loans this season; adds 1

**Of interest to:** Tweaks
