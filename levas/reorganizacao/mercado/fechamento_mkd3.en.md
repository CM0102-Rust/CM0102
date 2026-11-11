*English version of [fechamento_mkd3.md](fechamento_mkd3.md). Both are kept in sync; the Portuguese file is the original.*

# Closing the market map, reader D3: 0x862720 and 0x844780 (11/11/2026)

Routines: `staff_contract_calc_unset_default_wage` (0x862720, the default wage of the initial shortlists and of the
regens) and `staff_interest_in_job_at_club` (0x844780, the will of a person with a staff sheet to take a job). Section D
of the map (`docs/MAPA_MERCADO.en.md`) had them only by structure. Everything below comes from the disassembly of
`jogo/cm0102.exe` (`desmonta.py`), with the instruction address; the constants read from the exe. Ghidra's pseudo-C of
both is misleading: in 0x862720 it loses the FIFA-rules adjustment (0x86315d..0x8631d9) and says `return 0` where the
routine returns the value (0x864320); in 0x844780 the `_ftol` calls show no argument and the ambition weights
(0x844a9f..0x844b25), the weighted reputations and the board's confidence disappear. For each claim of the map: the old
text, the new one and the proof. The main session replaces [inferred] by [proved].

Jobs (`enums.rs`, `job_kind`): 5 manager, 6 assistant manager, 7 reserve team manager, 8 coach, 9 scout, 10 physio, 11
player, 12 player-manager, 13 player-assistant, 15 player-coach, 16 retired player, 17 media pundit.

## 1. 0x862720 `staff_contract_calc_unset_default_wage` — item **mkd_6** (ported)

Convention: stdcall(person, club, check, status, job, floor), `ret 0x18` (0x8644e7), float in st0 (the f32 at
[esp+0x14]); the first instruction overwrites ecx (both callers, thiscall, pass the contract in it and it is not read).
Callers (`refs.py`): `set_default_wage` 0x8626f6 (person, `club_job`, 1, −1, 11, 0) and `create_initial_shortlists`
0x83a2c3 (person, club, 1, the high nibble of the contract's +0x4f with the rest of edx, 11, 0), the latter for the
contracts with wage −1 (0x83a2a8). Same in cm0102-1080.exe (0x1e30 bytes).

Constants (read from the exe): 750 = 0x96ff30, 25000 = 0x96ff34 (f32); 500 = 0x96d1a8, 10000 = 0x96d1b0, 275 = 0x96ff20,
5000 = 0x968db0, 250 = 0x96d210, 1000 = 0x96d2dc, 200 = 0x96d834, 2500 = 0x968dac (f32); 2.5e-8 = 0x96ff38, 1e-8 =
0x968eb8, 5e-9 = 0x96ff28, 1e-9 = 0x96ff18, 0.0001 = 0x96a5c0, 1.35 = 0x969588 (f64); the bases 50000 / 55000 / 65000 =
0x96ff04 / 0x96ff08 / 0x96ff0c, 40000 / 45000 / 52500 = 0x96fef0 / 0x96fef4 / 0x96fef8, 20000 = 0x96ff00, 30000 =
0x96fefc, 8500 = 0x96d4e0, 11000 = 0x96feec, 175 = 0x96fee8, 550 = 0x96fed8, 1.375 = 0x96fedc, 0.0125 = 0x96fee0,
0.0004 = 0x96fee4, 0.2 = 0x96b028 (f32); 0.00017 = 0x96fed0, 0.00012 = 0x96ff10, 6 = 0x969000, 1.125 = 0x96a5a8, 2750 =
0x96fec8, 3250 = 0x96fec0, 1500 = 0x96feb8, 350 = 0x96f870 (f64). The immediates 150000 (0x48127c00), 85000
(0x47a60400), 75000 (0x47927c00), 65000 (0x477de800), 50000 (0x47435000), 40000 (0x471c4000), 75 (0x42960000), 50
(0x42480000) and 100 (0x42c80000) are not in `.rdata`.

| Map claim (§1, §2.3) | New text | Proof |
|---|---|---|
| Callers: `set_default_wage` (0x8626e0), `create_initial_shortlists` (0x83a1b0) | **[proved]**, with the arguments: both pass check = 1 and job = 11; 0x83a1b0 only for contracts with wage −1, with the contract's status | 0x8626e0..0x8626f6, 0x83a2a8..0x83a2c3 |
| "The default wage by ambition, loyalty, pressure, the major nation (0x549e50), the season (0x547770) and the cap (0x5968d0)" [inferred] | **corrected [proved]**: it reads neither pressure nor professionalism; it reads loyalty (base and market), ambition (market and bonus), determination and work rate (the veteran), international caps, age, the three reputations, ability and potential; it uses 0x549e50, 0x547770 (with today's date) and 0x5968d0 with the status dword | below |
| (new) The contract and the current club | **[proved]**: `get_contract` (stored in [0xae1080]); the contract's club, none for a loan (type 5) | 0x862720..0x86276d |
| (new) The level | **[proved]**: with a club, reputation / 500 + 1 in byte arithmetic; with a current club and a club of professional status 2, one less, within 1..4; without a club, the world reputation of the player (or staff) sheet / 500, at least 1 | 0x86279c..0x862838 |
| (new) The job computed | **[proved]**: the club job (with a current club), else the national job, else the club job, else 11 with a player sheet, else the national job or 9; the `job` argument only matters for jobs 6 and 8 (6 or 11 offered) and for coaches of 36 or more (13, 15) | 0x86283c..0x86288e, table 0x8644ec |
| (new) Staff (5 to 10) | **[proved]**: the level rises to the current club's reputation / 500 + 1 (or the national side's, outside job 11); wage = level × (current reputation³ × k with a staff sheet, else (id / number of people × 4 + 1) × m × 0.0001) × 0.25; manager: k 2.5e-8, m 25000, at least 750, above 25000 it becomes 150000; assistant and reserve manager: 1e-8 / 10000, at least 500; coach: 5e-9 / 5000, 275..5000; scout: 1e-9 / 1000, 250..1000; physio: 200..1000. At another club: the assistant is worth 150000 (except job 6/11 offered with the level reaching the current reputation / 500 + 4: below 2500, × 1.35); reserve manager and coach are worth 150000 when the current club's reputation / 500 + 4 is above the level; otherwise above 25000 becomes 150000 (job offered other than 6 and 11) | 0x862895..0x862f06 |
| (new) Player (11, 12, 13, 15) without a sheet | **[proved]**: writes "No player data..." into 0xde1f64 (sprintf with dwords +4, +8 and the id) and returns `rand()` × 4 / 32768 × 550 | 0x863660..0x8636b0 |
| (new) Player: status −1 | **[proved]**: with a club: 1 (current reputation above the club's, 24..33 years), 2 (above 0.75 × the club's, 21+), 4 (above 0.5 × and 17+, or 25+), 5 (potential × 50 above 0.75 × the club's), 6; the argument's byte is rewritten and goes whole to `get_max_wage` | 0x862f5b..0x86302f |
| (new) Player: the base | **[proved]**: job offered 13/15 at 36+: 10000; status 5/6: 8500 (current reputation < 2500) or 10000 (< 6250); attacker ≥ 15: 65000 / 55000 (reputation > 6750, loyalty < 15 / ≥ 15) or 50000; defensive, central or attacking midfielder ≥ 15: 52500 / 45000 (> 7250, loyalty < 14) or 40000; goalkeeper 20: 25000 (world > 7750, loyalty < 3) or 20000; sweeper < 15 and defender 0: 25000; else 30000 (world > 7500, loyalty < 6) or 25000. With the FIFA rules ([0xae233c] = 1), a base above 11000 and a club of a major nation: × 1.15 / 1.125 / 1.1 (reputation > 6750, loyalty 1 / < 6 / other) or × 1.025 (> 4250), not rounded to f32 | 0x86303d..0x8631d9 |
| (new) Player: the score and the wage | **[proved]**: m = the highest reputation (current, home, world), floor 7250 (floor 1) or 6750 (floor 2); s = (current + home + world + 4m + 50 × (ability + potential)) / 9 + ftol(id / count × 8 − 4); a foreigner (region +0x76 other than the club's and the current club's): at least 1500, otherwise × 1.05; at least 1; wage = base × s² × 1e-8, at least 175 (compared before rounding); × 0.75 or × 0.5 by the development of the current club's (or the person's) nation; × current reputation × 0.0004 (club < 2500) or / (level × 500), within 0.85..1.15 | 0x8631d9..0x8633b7 |
| (new) Player: the caps | **[proved]**: level < 10: 2500 × level; < 15: 2750 × level; else 3250 × level, or 10% above (soft) 3250 × level (England, Spain, Italy with PLC) or 5000 × level (without PLC); position: 20% above 85000 (attacker 20), 75000 (central or attacking midfielder 20), 65000 (defensive midfielder 20), 50000 (defender 20) or 40000, × 1.1 with FIFA, a major nation and current > 7250 | 0x8633bb..0x8635a9 |
| (new) Player without a club | **[proved]**: without `club_job`, except the veteran (33+, international, work rate + determination ≤ 30, id % 15 ≠ 0, club ≤ 4500): × (1.375 − d × 0.0125), at least 0.2, d = days since +0x3e (no cap), + 60 in season (0x547770) | 0x8635ad..0x86365b |
| (new) With a club: the offer by status | **[proved]**: with a player sheet, r = (club reputation × 0.00017 (amateur) / 0.00012 (> 2250) / 0.0001)⁶ (pow 0x945ff0), at most 1; by status (table 0x864518) the wage rises to the average with 3 × reputation × r × (1: 0.6 / 1.25 / 1.5; 2: 0.3 / 0.6 / 1; 3: 0.25 and then 0.3 / 0.6; 4: 0.1 / 0.15 / 0.3; 5: 0.05; 6: 0.02, for < 20 years / < 24 or outside a major nation / others); with the contract's grievance 0x10: × 0.67 / 0.67 / 0.85 / 2.5 / 3.5 / 7.5 (table 0x864530) | 0x862f12..0x8639b7 |
| (new) The market of the club's nation | **[proved]**, the same as step 7 of 0x864550 in the map: when the person works there (`club_job` of the same nation and is from there or loyalty > 3 or ambition < 15) or is from there without a club; Japan floor level² × 10 and 25% above 7500; England, Italy, Spain nothing; USA × 0.75, MLS 15% above 250, natives 20% above 1250 (32+, or < 24 with world < 3750) or 2750, foreigners 25% above 3250; Korea, Scotland, Germany/France/Holland, Sweden, Denmark/Norway/Poland, Belgium, Portugal/Croatia, Brazil/Argentina × the league-standard factor (reputation / 50 / standard × 0.1 × k, at most 1, when reputation / 50 > standard × 10) and each one's caps; Wales, Finland, Ireland, Northern Ireland, Australia only caps; the others × 0.3 and 25% above 15000 (compared before rounding) | 0x8639bb..0x864291 |
| (new) The final limits | **[proved]**: player: at most the wage of the current contract of status 7; when checking, or without a current club with reputation < 2250: at most `get_max_wage` (without finances it returns the value as it is, without the floors); without checking and with a current club, the ambitious (> 10), disloyal (< 6), famous (> 6750) attacker/midfielder/attacking midfielder 20 aged 21+: × 1.1 (attacking midfielder) / 1.05 (midfielder) / 1.15 if wage × 1.25 < cap; floors: club of status 1: 75 under 20 years, else 100; others: 75 at a club > 2250, else 50; without a club: the value without floors | 0x864295..0x8644e7 |
| §2.3 "0x862720 still inferred" | **[proved]**, and the differences from 0x864550 (map, item mkd_2): lower bases (without kinds 1..3 of 75000..105000, without the young rising, without the world < 1750 cut, attacker 50000..65000 instead of 55000..82500); level caps 3250 / 5000 instead of 5250 / 7500; position caps 40000..85000 instead of 80000..150000; the ratio by the **current** reputation (0x864550 uses the world one); the development without the "1 with world > 4250"; the free agent without the day cap nor the 20% above 30; staff with the 150000 | comparison with the 0x864550 text in the map |

### Defects and quirks (0x862720)

1. **150000 is a marker**, like the 250000 of 0x85fc50: the manager above 25000 and the assistant / coach of a stronger
   club are worth 150000, not a wage **[proved]** 0x862bea, 0x862a80, 0x862bb4, 0x862d33.
2. The defender test is `!= 0` (0x86310f): every player has a defender rating, so the "sweeper < 15 and defender 0"
   branch never holds and the base stays at 25000 / 30000 **[proved]**; the same result in practice.
3. Without a club and with level 15 or more (world ≥ 7500), a player reads +0x53 of the null club (0x863403): an access
   fault **[proved]**; both callers pass a club (a regen has one) **[inferred]**.
4. Without a player sheet in a player job, the sprintf uses `%s` with the person's dwords +4 and +8 (0x863660)
   **[proved]**; the wage comes from the CRT `rand()`, outside the game's generator.

### Extension points

- **0x862720 (mkd_6, Rust once integrated)**: the wage of the regens (`set_default_wage`) and of the initial shortlists;
  the bases by position, the s² score and the market by nation are in one function. For the Tweaks, a detour on the
  way out that scales the value for the regens only changes their wage without touching the desired contract
  (0x85fc50).

## 2. 0x844780 `staff_interest_in_job_at_club` — item **mkd_7** (ported)

Convention: thiscall(shortlist manager, person, club, check, job, &reasons), `ret 0x14` (0x84680b), SEH frame, f64 in
st0; ecx is only read for the clubs' shortlists (+0xdc, 0x845809, 0x84600a). Callers (`refs.py`): 0x4ce92c
(`resolve_contract_offers`, check 0), 0x69fc3d (`is_staff_suitable_for_job` 0x69f810: job 5, check 0), 0x840682
(0x840084, the AI offers: check 1), 0x860339 (`set_to_desired_contract` 0x85fc50) and 0x807ebc (search filter
0x807450, check 1). Same in cm0102-1080.exe (0x21a0 bytes).

Constants (read from the exe): 0.025 = 0x967878, 1.4 = 0x968dc0, 0.02 = 0x968a98, 0.8 = 0x9689b0, 0.05 = 0x967888, 0.5 =
0x968968, 0 = 0x9689a0, 3.5 = 0x968e38, 900 = 0x96f958, 12 = 0x96d6a0, 0.33 = 0x968f78, 0.85 = 0x969520, 3 = 0x968930,
1/60 = 0x968958, 0.75 = 0x969040, 0.7 = 0x968e40, 0.1 = 0x967880, 0.0025 = 0x969028, 1.5 = 0x968940, 0.01 = 0x967898,
0.45 = 0x96a6a8, 0.65 = 0x96a5d0, 2.2 = 0x96d7d0, 10 = 0x9688b0, 1/0.85 = 0x96f950, 1/0.45 = 0x96f948, 0.4 = 0x968e00,
0.25 = 0x9689e0, 0.2857 = 0x96d5e0, 7000 = 0x96d470, 0.67 = 0x968f80, 0.0008 = 0x96a618, 5 = 0x9688a0, 1000 =
0x968908, 0.0001 = 0x96a5c0, 1.25 = 0x968928, 1.45 = 0x96f940, 0.6 = 0x968e48, 5000 = 0x9688e8, 1/15000 = 0x968d78,
12000 = 0x96f938, 200 = 0x96a648, 0.2 = 0x968918, 2.5 = 0x968e18, 0.000125 = 0x96a6b8, 0.0877 = 0x96f930, 1.1 =
0x9689c0, 100 = 0x9688a8, 150 = 0x968f50, 1/150 = 0x96d1d0, 0.04 = 0x968a90, 1/4500 = 0x96f928, 0.001 = 0x96a350, 1.2
= 0x968978, 0.9 = 0x9689d8, 1.75 = 0x968f98, 1.35 = 0x969588, 0.3 = 0x968e80, 0.225 = 0x96d6d8 (f64); 1 = 0x968e70
(f32). Immediates: 2.25 (0x40020000), 1.5 (0x3ff80000), 3.5 (0x400c0000), 0.5 (0x3f000000, f32), 0.001
(0x3f50624d_d2f1a9fc), 0.9 (0x3feccccc_cccccccd). Names: "Dario Gradi" 0xa82620, "Pat Nevin" 0xa82614, "Kevin Keegan"
0xa82604.

| Map claim (§1) | New text | Proof |
|---|---|---|
| "Callers: 0x4ce4e0, 0x85fc50, `is_staff_suitable_for_job` (0x69f810), 0x840084, filters" | **[proved]**, with the sites and arguments: 0x4ce92c, 0x860339, 0x69fc3d (job 5, check 0), 0x840682 (check 1), 0x807ebc (search_filters.cpp, check 1) | `refs.py`; 0x69fc2a..0x69fc3d, 0x84066f..0x840682, 0x807ea5..0x807ebc |
| (new) The filters that give 0 | **[proved]**: no staff sheet, no club or person nation; **Dario Gradi, Pat Nevin and Kevin Keegan with a club job** (by the formatted name, 0x53dba0); club or national job 17; outside job 5, the last 16 records (id ≥ count − 16); a national manager (job 5 for the nation); with a job offered ≠ 0: a manager (5/12 at club or nation) with current reputation > 4250 or ability > 100 wanting a job other than 5; a physio (10) asked for another job, or someone with another job asked to be physio; a player (11) with a club asked by another club (outside jobs 5, 12, 15); a manager whose current club has a division of reputation greater or equal to the new one's, or a reputation above the new one's − 750; a club below 2750 of the current club's nation or that finds him foreign (0x53d980) | 0x8447d2..0x844a84 |
| (new) The weights | **[proved]**: w_stadium = 1.4 − ambition × 0.025 − professionalism × 0.025; w_division = ambition × 0.02 + 0.8; w_competition = ambition × 0.05 + 0.5; w_manager = 0 for jobs 5 and 12, else 1; denominator = the sum + 1 (0.5 if ≤ 0) | 0x844a87..0x844b25 |
| (new) The current club | **[proved]**: the `club_job`; without one, or when the club offered is a national side (0x53b900), the national side (except job 11 there); a current club without a nation: returns 3.5 | 0x844b30..0x844b8c |
| (new) The base | **[proved]**: 3.5 at his own club; otherwise 0 with a current contract of the same language and grievance 0x20; 2.25 (jobs 5, 12) or 1.5, plus, without a club, (days since the later of the game's start [0xdd6ec8] and joining the club, at most 500)² / (current reputation + 900), at most 12 | 0x844b97..0x844d22 |
| (new) Likes, rivals, managers | **[proved]**: dislikes the club × 0.33 (2); the current one × 2 (4); favours the current one × 0.85 (current amateur and new professional) or × 0.5 (8 with a contract without Bosman); favours the new one × 3 (0x10); rival or league clash (0x4b02e0): Celtic × Rangers 0.5 − loyalty × 0.025, Italy 1 − loyalty / 60, a rival of the same division 0.75 − loyalty × 0.025 (difference < 1250) or 0.8 − loyalty / 60, else 0.85 − loyalty × 0.025 (0x20 below 0.7); without a rival, Croatia × Yugoslavia × 0.05 / 0.1 (2); the new club's manager favours × 2 (0x40), dislikes × 0.33 (0x80), outside jobs 5 and 12; the current manager dislikes × 2 (0x100), favours × 0.5 (0x200); with a future transfer: 0 (0x10000000) | 0x844d34..0x8450d3 |
| (new) Morale and jobs | **[proved]**: the current contract's morale: at his own club × (1 + morale × 0.0025), else × (1 − morale × 0.0025) (0x400 above 1.5, 0x800 below 0.5); at his own club the routine jumps from here to the end. By the current job (table 0x846810): chairman × 0.01; directors × 0.01 (× 0.5 if offered manager at a club ≥ 6000); manager × 0.45 (0x4000000) or × 0.65 without a club; assistant × 1.5, reserves × 2.2, coach × (ambition × 0.05 + 0.8) (0x8000000 if offered manager); 16 (retired) × 10 / ambition; player and others × 10 (0x8000000). By the one offered (table 0x846850): 0 → 0; chairman × 10; directors × 1/0.85; manager × 1/0.45 (× 1.5 if already a manager); assistant × 0.5; reserves and coach × 0.4; scout and physio × 0.4 (already one of them) or 0.25, × 0.25 with a player sheet; player and others × 0.1; a current manager × 0.4 in the other jobs. A manager with the board's confidence (+6 + +8 of the 0xb67a44 row + 500 × (loyalty + determination)) below 8000: × 1.4 | 0x845127..0x845438 |
| (new) The weighted reputations | **[proved]**: for the new club, e = ftol(1.5 × (w_comp × competition + w_div × division + w_man × manager + w_stad × stadium + shortlists) / den − 5000), with the stadium and the division of 0x846920, competition = 600 × the continental one's reputation (16 bits) or 0.8 × division up to 7000, the manager = his reputation as the person sees it (0x5407e0) weighed by the board's confidence (0.67 × +6 + 0.33 × +8), shortlists = 0.4 × the shortlists' reputation + 0.6 × the reputation by wealth band (0.7 / 0.85 / 1.25 min. 4250 / 1.5 min. 5750), × 1.45 at a national side of his nation; a non-professional club: min(e/2, e − 5 × (international caps² + 400)), status 3 again min(e/2, e − 2000). For the current club the same with other constants: the manager by his current reputation (div × 0.75 without a manager), bands 0.75 / 0.9 / 1.25 min. 4750 / 1.5, national side × 1.35, the Intertoto rule. Without a current club: the average of the sheet's three reputations | 0x84543c..0x845955, 0x845bd8..0x846122 |
| (new) Language, foreigner, time | **[proved]**: × max(F × 0.1, (e + 5000)² / 15000 / (12000 − 200 F)) + 0.2 (F = fluency, 10 without a player sheet) at the new club (0x10000 below 0.6), ÷ the same at the current one; a foreigner (no club, or foreign in the club's nation with a nation other than the current club's): × 0.85 / 0.75 / 0.65 / 0.5 / 0.01 by job when the club's manager is his compatriot, else × 0.25 / 0.3 / 0.225 / 0.2 / 0.001; without a club × 2.5; with a club, 0 below (loyalty + 8) × 15 days at the club, and t = min(days² / 20 / (loyalty + 6), 400) × 0.0025 | 0x845955..0x845bd0, 0x846122..0x8461ce |
| (new) The scales' difference | **[proved]**: a null division or the key division 0x9cf764: × 0.1; g = 1 + (club reputation − the sheet's average) × 0.000125; y = (e_new − e_current) × (ambition × 0.0877 + 0.75) going up, × 1.5 going down (jobs other than 5/12), or for a manager × min(max(1.1 − will × 0.01, 0.1), 1) with a will > 10; y > 0 with t < 1: × ((150 − min(y × 0.02, 100)) × t + y × 0.02) / 150 (0x100000 below 0.6); a manager offer to a non-manager, y < 0: y × max(1 − will × 0.04, 0.4) (coach, assistant) or max(1 − will × 0.1, 0.2); × max(1 + y / 4500, 0.001) (0x400000 / 0x800000 / 0x200000 by ambition and the competitions); the nations' development: equal, a European going outside Europe × 0.4; different, × (dev_current² × div_new) / (max(div_current, 500) × dev_new²) within 0.01..10, × 0.2 if from another continent and < 1 (0x2000000); × g (0x800000 below 0.6) | 0x8461d7..0x846628 |
| (new) The wage | **[proved]**: when checking with a will ≥ 1.75: no finances, 0; q = max(3.5 / max(will, 0.1), 0.9); asked = q × the default wage (0x864550 with a local contract, check 0, status −1); above the staff cap (0x597db0): will × cap / asked and the asked wage becomes the cap; with a current contract: ratio = asked / max(wage, 10); below 0.1 or below 1, the will; above 10, × 10 / q; otherwise × ratio / q | 0x84662e..0x8467ec |
| §2.1 "the will 0x846920; non-player 0x844780 at 0x840682" | **[proved]**: 0x840682 calls 0x844780 with (person, club, 1, the vacancy's job, 0) | 0x84066f..0x840682 |

### Defects and quirks (0x844780)

1. **Three fixed names**: Dario Gradi, Pat Nevin and Kevin Keegan never leave a club job (will 0); the comparison is by
   the formatted name, so anyone with that name too **[proved]** 0x8447fb..0x8448f5.
2. **The last 16 person records** (id ≥ [0xae2364] − 16) only accept the manager's job **[proved]** 0x84491a;
   [inferred] they are the regens' spare records.
3. **Job 0 only at his own club**: the job offered 0 gives 0 at another club (0x84532a), but at his own club the routine
   skips the job tables and computes the will **[proved]** 0x8451ca.
4. **The manager's asymmetry**: a new club without a manager weighs 0 on the scale; the current one without a manager
   weighs 0.75 × the division **[proved]** 0x8455f4 against 0x845ded; the wealth bands, the national side factor (1.45
   against 1.35) and the Intertoto rule (only the current club) differ too **[proved]**.
5. **No finances, 0**: when checking with a will ≥ 1.75, a club without a finance record gives will 0, even a positive
   one **[proved]** 0x846671..0x846673.

### Extension points

- **0x844780 (mkd_7, Rust once integrated)**: the whole staff movement (managers, assistants, scouts) in one function:
  the base 1.5 / 2.25, the job tables (× 0.01 to × 10), the minimum time at the club ((loyalty + 8) × 15 days), the
  board's confidence (8000) and the wage gate (1.75). For the Tweaks: changing how much managers change club by groups
  (reputation, age) without touching the players' will (0x846920).
