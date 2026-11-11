# Market, section C: loans, windows and limits (reader C, 11/11/2026)

*English version of [secao_c.md](secao_c.md); the Portuguese file is the original.*

Part of the market map asked for by Tweaks 1.5 ([BRIEF.en.md](BRIEF.en.md)). **[proved]** = read in the disassembly or
in Rust already compared (0 divergences); **[inferred]** = a conclusion from the code, not reproduced in the game.

## 1. The routines

| Address | Name | In Rust | Item |
|---|---|---|---|
| 0x842740 | `Shortlist_Manager::try_loan_offer_for_player` | yes (`try_loan_offer_for_player`) | — |
| 0x8d0590 | `TRANSFER_MANAGER::get_loan_period` | yes (`transfer_manager_get_loan_period`) | — |
| 0x8d0040 | `TRANSFER_MANAGER::can_loan_player` | yes (`transfer_manager_can_loan_player`) | — |
| 0x8d0530 | `TRANSFER_MANAGER::can_end_loan` | yes (`can_end_loan`) | — |
| 0x8c63a0 | `TRANSFER_MANAGER::process_due_offer` | yes (`process_due_offer`) | — |
| 0x8c5880 | `TRANSFER_MANAGER::daily_update` (the weekly sweep of the offers) | yes | — |
| 0x8c2180 | `TRANSFER_MANAGER::handle_player_accepted_terms` (the loan completes here) | yes | — |
| 0x8cb810 | `TRANSFER_MANAGER::add_past_transfer` | yes (`add_past_transfer`) | — |
| 0x8cd450 | `TRANSFER_MANAGER::check_squad_limits` | yes | — |
| 0x8c6bc0 | `TRANSFER_MANAGER::update_transfer_window_news` | yes | — |
| 0x8cf470 | `TRANSFER_MANAGER::find_next_window_open_date` | yes | — |
| 0x8c4e10 | `TRANSFER_MANAGER::hold_offers_at_window_close` | yes | — |
| 0x4d01e0 / 0x4d07e0 / 0x4ccca0 | `CONTRACT_MANAGER::sign_loan_contract` / `end_loan` / `schedule_loan_end` | yes | — |
| 0x8e8bd0 | `TRANSFER_RULES::get_loan_period` (the default period) | no | **mkc_1** |
| 0x8e8e50 | `TRANSFER_RULES::find_open_window` | no | **mkc_1** |
| 0x8e8c40 | `transfer_window_contains_date` | no | **mkc_1** |
| 0x5c1ed0 | `FRANCE_RULES::can_end_loan` | no | **mkc_2** (8 leagues) |
| 0x6618f0 | `ITALY_RULES::can_end_loan` | no | **mkc_2** (8 leagues) |
| 0x412dd0 | `AUSTRALIA_RULES::can_loan_player` | no | **mkc_2** (8 leagues) |
| 0x579c70 / 0x579d60 / 0x579fd0 / 0x57a020 | England: period, `can_loan_player`, `record_loan`, `daily_update` | yes | — |
| 0x7f6530 / 0x7f6650 | Scotland: period, `can_loan_player` | yes | — |
| 0x661770 / 0x63c3d0 / 0x859630 | `can_loan_player` of Italy, Ireland, Spain | yes | — |
| 0x40a600 / 0x40a610 | base `can_loan_player` / `can_end_loan` (`mov eax, 1; ret`) | no | cannot be detoured (2 bytes) |
| 0x5a9a80 / 0x5a9af0 / 0x5a9ba0 | Finland: period, `can_loan_player`, `can_end_loan` | no | out: does not run in the 8 leagues |
| 0x794760 / 0x902240 | Norway, Turkey `can_loan_player` | no | out (same) |
| 0x793870 / 0x793900 / 0x7ec8e0 / 0x90f1a0 | Northern Ireland (period, `can_loan_player`), Russia and Wales (period) | no | out (same) |
| 0x5e0210 / 0x63c530 / 0x63c4e0 | Germany `can_end_loan`, Ireland `can_end_loan` and `record_loan` | no | out (same) |
| 0x8e8560 / 0x8e8800 | base `can_sign_player` / `can_make_offer` (foreign player limits) | no | mapped here, not ported (they belong to signings, reader B) |
| 0x784100 / 0x7874b0 / 0x5fd2f0 | `NewsItem::auto_respond`, `News::auto_respond_all`, `HUMAN_MANAGER::set_on_holiday` | no | mapped (the holiday defect) |

## 2. The AI loan request (0x842740) [proved, Rust]

Called by `make_club_bids` (0x84a020, the AI's pass over its 40 shortlist slots), with `can_loan_player` checked
before by `fill_shortlist_slot` (0x83f3a0) and by `make_club_bids` itself (0x84aa2b). Order:

1. No owner (main contract 0x4cf5a0 null): the shortlist slot goes back to state 1; −1.
2. The player's interest in the club (`staff_interest_in_club`, 0x846920, with the slot's state) below **3.5**
   (`.rdata` 0x968e38): −1.
3. The period (`get_loan_period`, §3): months to the end `(end_year − year)·12 − month + end_month`, to the start the
   same, at least 1; start after the end: −1. **This is where the end-of-contract loan dies for the AI** (§7).
4. A club of Norway (`KEY_NATION_NORWAY`) with today's month in November to March: −1.
5. Interest without wage ≥ **2.75** (0x9690a8) and the club's nation not Spain, Portugal or Italy: counts the months to
   the start; otherwise to the end.
6. The return date: today's day (at most 28) in month today + months. `can_end_loan` (0x8d0530, flag 1) = 0: −1.
   Return after the end of the main contract (+0x2d/+0x2f): −1. Return − 4 weeks before the window date of the club's
   key nation (`[0xb63d8a + 72·key_nation_index_of_club]`): −1. Return before today + 3 weeks: −1.
7. Wage paid: 100% if the club's reputation (`club.reputation()`) beats the owner's by more than 2500, 0% if it is 1500
   or more below, otherwise 50%. The offer: `make_loan_offer` (0x8bed60).

The owner's answer is decided by 0x84af00 (`predict_selling_club_response`, another reader's section).

## 3. The loan period (0x8d0590) [proved, Rust]

`TRANSFER_MANAGER::get_loan_period(club, contract_club, player, &start_month, &start_year, &end_month, &end_year)`,
thiscall ret 0x1c. Callers: 0x842740 (AI), 0x8d0040 (`can_loan_player`), 0x8f9690 (the human's loan screen), 0x8f1c30
(answering a loan bid), 0x888260/0x889c30 (profile and comparison). 203,886 calls up to 2002-06-01 in the English
career (counters of item cqe_6).

1. The rule of the nation of the **borrowing club** (of the 28 at `TRANSFER_RULES` +0x8ac; without one, the first)
   gives the period through virtual +0x28 (`GET_LOAN_PERIOD`):
   - base 0x8e8bd0 (**mkc_1**): from next month to the season end month of the club's nation (0x4d45f0), this year if
     still ahead of the start, otherwise next year;
   - England 0x579c70: from next month to 3 months later; after 19 June and a player under 24, the base;
   - Finland 0x5a9a80: as the base; Russia 0x7ec8e0: start in 3 months, end at season end; Northern Ireland 0x793870:
     the same, and the start never passes the end; Wales 0x90f1a0: start = end (season end).
2. The player's main contract (0x4cf5a0). **No contract, a type (+0x35 & 0x3f) other than 2, or today after the
   contract end: the end year becomes 1972** (the year of the default date 0x54a110), an "impossible" sentinel.
   The types (screen 0x87cee0, jump table 0x87dd58) [proved]: 0 invalid, 1 monthly, **2 normal contract**, 5 trial,
   6/7 loan; bit 0x80 = part time.
3. End more than 5 years ahead: 1972.
4. While the date (today's day of the month, end month, end year) passes the contract end (+0x2d day of the year,
   +0x2f year), the end steps back one month and the start follows if it is after the end; the end year below the
   current one: 1972.

## 4. Can the club take him on loan? (0x8d0040) [proved, Rust]

`can_loan_player(club, player, message)`, thiscall ret 0xc, 1 = yes. Callers: 0x83f3a0 and 0x84a020 (AI, no message),
0x8f9690 (the human's screen, message at 0xde1f64).

1. No current club (contract other than type 5): 0. Already on loan (loan contract 0x4cf6d0): 0. Not a player
   (`job_for_club() != PLAYER`): 0.
2. `can_club_bid_for_player` (reasons 0x21 under negotiation, 0x27, 7, 1, 0x12/0x13, 0x19/0x28/0x1a nation): 0 with
   the text.
3. The rule of the **borrowing club's** nation, virtual +0x20 (`CAN_LOAN_PLAYER`) = 0: 0. By nation:
   - base 0x40a600 / Greece 0x5ed060: 1; Northern Ireland 0x793900: 1 (writes `[0xae1080]`);
   - England 0x579d60: same division (Premier only) 0x1d; **5 a season** (byte 0 of `[[0xde1f54]+0x91c] + id·16`,
     zeroed on 20 June by 0x57a020, counted by 0x579fd0) 0x1e; **more than 1** loanee from elsewhere already in the
     squad (the 3rd is refused) 0x26; outside the window (0x8e8e50) 9;
   - Australia 0x412dd0 (**mkc_2**), Norway 0x794760, Turkey 0x902240: outside the window, 9 ("The deadline for loans
     to ... clubs has passed."); Finland 0x5a9af0: the same and, for an AI club, only between days 90 and 300 of the
     year;
   - Italy 0x661770, Spain 0x859630, Scotland 0x7f6650, Ireland 0x63c3d0: in Rust (windows and division limits).
4. The rule of the **current club's** nation (the lender): none, 1.
5. The period (§3). **Start = end: 1** (nothing else). Otherwise, month by month from the start to the end
   (inclusive), the owner rule's virtual +0x24 (`CAN_END_LOAN`, flag 1); the first month that lets the loan end gives
   1; past the end: 0, "This loan deal cannot go ahead". By nation (+0x24):
   - base 0x40a610: 1 (England, Scotland, Spain...: the loop answers 1 on the first month);
   - France 0x5c1ed0 (**mkc_2**) and Ireland 0x63c530: the date inside a window of the nation;
   - Italy 0x6618f0 (**mkc_2**): with the flag, June; otherwise the date in the 1st or 2nd Italian window;
   - Finland 0x5a9ba0: refuses between 18 August and 15 November;
   - Germany 0x5e0210: without the flag, refuses ending loans between German clubs (with the flag, 1).

## 5. Windows [proved]

Transfer rules: +4 the window table (12 bytes: +0 opening, +6 closing; each date 6 bytes: +0 rule, +1 number, +2 the
weekday it moves to (0xff none) [inferred], +3 day, +4 month (0 = January), +5 1 opens / 0 closes), +8 how many, +0xa
the nation, +0x13 foreign player limit, +0x14 non-EU limit.

- `transfer_window_contains_date` 0x8e8c40 (**mkc_1**): the two dates in the date's year, both ends included; a window
  that wraps the year (opening month > closing month) is open from the opening to December and from January to the
  closing.
- `find_open_window` 0x8e8e50 (**mkc_1**): the first window with the date; **a nation without any window (Denmark) is
  always open**; with windows and none open, 0.
- Dates (constructors, Rust): England 2 June (Monday) to 26 March (Thursday); USA 7 January to 15 August; Australia 11
  June to 17 April; France 30/5–31/8, 1/9–20/12, 21/12–10/1; Finland 16/11–15/8; Croatia 1/6–30/3; Denmark none.
- Notices (0x8c6bc0, every day, slice 0): on the day, the opening/closing; the day after the opening, the offers that
  waited for that nation's window (0x8c4e10); a week before.

## 6. Limits that block a signing [proved]

- **Squad** (`check_squad_limits` 0x8cd450, Rust, from the offer restrictions 0x8cfc90 for the buyer and the seller):
  50 players (club + its B team), 5 coaches, 3 physios, 7 scouts and the assistant; above, an AI buyer without an
  exchange releases for free the cheapest player who can leave; otherwise reason 4 or 0x22 (+1 for the seller). For a
  loan, the completion (0x8c2180) refuses if the borrower's squad + 1 passes 50 (reason 4) and asks the borrower
  nation's `CAN_LOAN_PLAYER` again.
- **Foreigners** (base `can_sign_player` 0x8e8560, virtual +0x10, not ported): counts the foreigners in the squad (50
  slots +0xd7, `database_is_staff_foreign` 0x53d980) up to limit +0x13 (default 50; 0xff no limit): reached, reason
  0x14. A Turkish club outside the Turkish Premier: limit 0 (no foreigner). With the rules' nation in Europe
  (`KEY_CONTINENT_EUROPE`), the non-EU players (0x53d740) up to +0x14 (default 30): reason 0x15; Spain: 4 in the First
  Division, 3 in the Second, 0 elsewhere. Then the window: closed, 9. `can_make_offer` 0x8e8800 (+0xc) does the same
  count with the exchange players and only checks the window when the decider is the buyer. Overridden by: Japan (5
  foreigners), Portugal, Spain, Italy, Germany, Belgium, France, Argentina.

## 7. Suspected defect 1: the loan at the end of the contract

**The path [proved]:** §3 steps 2–4 and §4 step 5, with §2 step 3.

- A contract **not of type 2** (monthly 1, trial 5) or already expired: the period comes out with year 1972.
  - The AI (0x842740) computes negative months to the end: never asks.
  - `can_loan_player` **does not recognise the sentinel**: start ≠ end, so it tests the start month with the owner's
    `CAN_END_LOAN`; with an English owner (base = 1) **it returns 1**. The human's screen (0x8f9690) opens with fields
    5/6 (end) = the month and **1972** [proved: the values go to fields 3–6 and 0xc/0xd of the screen]. With a French/
    Italian/Irish/Finnish owner outside the window, "This loan deal cannot go ahead".
- A type 2 contract that **ends in the current month**: the end steps back to this month and the start follows (start
  = end):
  - `can_loan_player` returns 1 (start = end);
  - the AI computes 0 months to the end < 1: never asks;
  - the human gets a zero-month period (return this month) [what the screen does with it: inferred].
- A contract ending **next month**: a one-month loan; the AI asks if the return is 3 weeks away or more.

**A defect of the original?** Yes, of consistency [proved]: `get_loan_period` uses 1972 to say "impossible", but
`can_loan_player` only blocks through the owner's end rule and accepts start = end, so the human is allowed to open a
proposal with an impossible (or zero-month) end, while the AI silently refuses. That this is exactly the forum's "I
can't loan players at the end of their contract" (58.2) is **inferred** (not reproduced in the game).

**Small fix:** a detour on 0x8d0040 (Rust here already; in the Tweaks, prologue `6a ff 68 1b 40 96 00`, 7 bytes) that,
after the period, returns 0 with a clear message when the end year is the default date's (1972) or the end is before
the start; or, as a more generous policy, a detour on 0x8d0590 (prologue `83 ec 10 32 d2`) that calls the original and,
when the sentinel came from the contract (type ≠ 2 or stepping back below this year), sets the end to the contract end
month and the start to the month after today, if it fits.

## 8. Suspected defect 2: offers stuck while the manager is on holiday

**What the code says [proved]:**

- On holiday, `human_manager_is_human_club(club, 0, 1, 0, 0)` (0x600a50, checking holidays) says **no**: new offers
  take the AI branches of `process_due_offer` (0 → AI reply; 1 → `submit_loan_contract_offer`; 5/6 → contract to the
  player). State 0x11/0x12 uses `news_club_has_human_manager` (0x7875e0, **ignores** holidays): sends news 0xfa0 and
  waits 28 days.
- News delivered while the manager is away is answered at once (`News::deliver_to_manager` 0x785d40 →
  `NewsItem::auto_respond` 0x784100). On **starting** the holiday, `set_on_holiday` (0x5fd2f0, at 0x5fd3d9/0x5fd3ed)
  answers everything in the inbox (`auto_respond_all` 0x7874b0): only the items still in the 100-slot ring.
- `auto_respond` covers: 0xfa0 (offer; loan → 0x84af00 → accept 0x8c0660 / refuse 0x8bfa30), 0xfa2, 0xfa4 (only sets
  `RESPONSE_ROUND` = 3), 0xfa5 (confirmation → 0x8cfc90 → `club_rejects_offer`/`confirm_offer_by_club`), 0xfa8, 0xfab,
  0xfac (contract talks → `offer_contract_to_offer_player`), 0xfb4, 0xfb5 (loan request), 0xfb6, 0xfc0 (loan
  negotiation → `submit_loan_contract_offer`).
- `process_due_offer` **has no branch** for 8, 9, 0xb, 0xc, 0xd and 0x13 (0x13 belongs to the future transfers; 8 to
  the contract manager; 0xa to the permits). The weekly sweep of `daily_update` (day a multiple of 7) reschedules for
  tomorrow every overdue offer **except** 0x13, 7, 8 and 0xa: 9, 0xb, 0xc and 0xd come back every day and nothing
  happens.

**The defect [inferred, not reproduced]:** an offer that was waiting for the human (9: the human buyer must offer the
contract; 0xb–0xd: the clubs' confirmations) when he goes on holiday only moves if its news item is still in the
inbox; if it left the 100-slot ring (or he deleted it), it stays put for the whole holiday, and the player stays
"under offer" (bit 0 of the 8-byte record in `PERSON_FLAGS`), which blocks other offers for him. It is a defect of the
original: the dispatch depends on the news item, not on the offer's state.

**And loans?** A loan never goes through 9 or 0xb–0xd: it completes the moment the player accepts (0x8c2180). The loan
states that wait for someone are 0 (human owner: 14 days and it lapses), 1 (negotiation with a human borrower: lapses)
and 8 (the player's decision, in the contract manager 0x4cc930/0x4cd9c0). The only one that **does not lapse** is 8,
and the weekly sweep leaves it out; a loan whose player decision never comes stays stuck. That this is xeno's item 88
is **inferred**; the exact path that loses the decision was not found.

**Small fix:** in `process_due_offer` (Rust here; in the Tweaks, prologue `6a ff 68 00 3d 96 00`), for state
9/0xb/0xc/0xd with the human involved on holiday (`news_club_has_human_manager` ≠ 0 and
`human_manager_is_human_club(…, 0, 1, 0, 0)` = 0), do what `auto_respond` would: 9 → `offer_contract_to_offer_player`;
0xb–0xd → `check_offer_restrictions` and `confirm_offer_by_club` or `club_rejects_offer`. For 8, a deadline: a loan offer
in 8 for more than N days (`DUE_DATE` a week overdue) → `reject_loan_offer`.

## 9. How a loan ends and returns [proved, Rust]

- **Signing:** `handle_player_accepted_terms` (0x8c2180) → `sign_loan_contract` (0x4d01e0): the contract goes into the
  loan slot (+4 of the 79-byte record), the player joins the borrower's squad (job 0xb), bit 0x40 of the main contract
  = reserves; history `add_loan_records`; `record_loan` of the borrower's nation (England counts the season);
  `add_past_transfer` type 2; offer 0x14, removed in 5 days.
- **Natural end:** `CONTRACT_MANAGER::daily_update` (0x4c8114) calls `end_loan` (0x4d07e0) on the end date [inferred
  from the call site].
- **Early end:** the screen (0x8fbcc0) asks `can_end_loan` (0x8d0530: only when the player's owning club is in the same
  nation as the one asking; otherwise 1) → `schedule_loan_end` (0x4ccca0: the loan contract ends tomorrow, type 7,
  news 0xfb8 if a human is involved) → `end_loan` the next day.
- **Permanent transfer:** `move_player_to_club` (0x8c9d5d) ends the loan first.
- **`end_loan`:** takes him out of the borrower's squad and tactics, frees the loan contract, redoes the wage bills,
  returns him to the owner with the squad mark (with a reserve team: 2 if the main contract has bit 0x40, otherwise
  1), European registration 0x10 → 4.

## 10. The transfer history (0x8cb810) [proved]

A 37-byte record (0x25): +0 person, +4/+8/+0xc name ids, +0x10 destination club, +0x14 origin club, +0x18 fee, **+0x1c
the type**, +0x1d the date. The types, by their writers: **0 paid**, **1 with exchange players** (`move_player_to_club`
0x8c99a5: `exchange_players` ≠ 0), **2 loan** (0x8c2180), **3 release** (`free_transfer_staff` 0x8c1af0), **4 free**
(pre-contract with fee 0 and completion 3 or 4, 0x8c9996), **5 a non-player** (manager/staff: `move_player_to_club`
0x8c99bf and `MANAGER_MANAGER::daily_update` 0x690432; or record only in `free_transfer_staff`). Vector with capacity
+0x866, count `past_transfer_count()`, grows by 100.

## 11. Extension points (for the Tweaks)

| Where | What it changes | How |
|---|---|---|
| 0x8d0590 `get_loan_period` | default length; the end-of-contract sentinel | detour, call the original and adjust the 4 outputs |
| 0x8d0040 `can_loan_player` | who can borrow | detour: original + the new rule (e.g. refuse the sentinel; allow outside the window) |
| rule vtables (+0x20, +0x24, +0x28) | one nation's rule | swap the slot pointer in the nation's table (England 0x969f4c, Scotland 0x96f838, Italy 0x96c96c, Ireland 0x96c190, Spain 0x96fdec) |
| rules +4/+8 (memory) | the window dates | data, no code: rewrite the table after the constructors |
| rules +0x13/+0x14 (memory) | foreign/non-EU limits | data |
| `[[0xde1f54]+0x91c] + id·16` | the English season loan count | data (the limits 5 and 2 are immediates in 0x579d60) |
| 0x842740 (3.5 / 2.75 / 2500 / −1500 / 3 weeks) | how much the AI borrows | detour; do not touch the `.rdata` doubles (shared) |
| 0x8cb810 `add_past_transfer` | measuring | every completed move passes here with its type |
| 0x8c63a0 `process_due_offer` | offers stuck on holiday | detour (§8) |

## 12. Doubts

- The exact point of the human's screen that turns the 1972 end (or the zero-month period) into a refusal.
- Which state traps the loan on holiday (8 is the candidate; the path that loses the decision was not found).
- Do contract types 3 and 4 (N/A on the screen) exist in the data?
- The loop of `ENGLAND_RULES::can_loan_player` tests the id of the target `player`, not of the squad player, to skip
  invalid slots (the Rust reproduces it; no effect in practice) [inferred].
