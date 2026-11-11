*English version of [fechamento_mkd1.md](fechamento_mkd1.md). Both are kept in sync; the Portuguese file is the original.*

# Closing the market map, reader D1: 0x864550 and 0x85fc50 (11/11/2026)

Routines: `STAFF_CONTRACT::default_staff_contract` (0x864550, the default wage) and
`STAFF_CONTRACT::set_to_desired_contract` (0x85fc50, the desired contract). Everything below comes from the whole
disassembly of `jogo/cm0102.exe` (`desmonta.py`, packages `pacotes/0x864550` and `pacotes/0x85fc50`), with the
instruction address; the constants were read from the exe. The map (`docs/MAPA_MERCADO.en.md`, section D, table, §2.3
and §7) had them only by structure. For each claim: the old text, the new text and the proof. The main session
replaces [inferred] by [proved].

## 1. 0x864550 `default_staff_contract` — item **mkd_2** (ported, compiles; proof in the benches)

### 1.1 Section D table

- **Old:** `| 0x864550 | ... | no | — | 0x846920 (wage gate), 0x85fc50 (×3), 0x844780, 0x842e10, 0x840084, update_wage_bill (0x598370), 0x85e960 |`
- **New:** `| 0x864550 | ... | no (item mkd_2) | mkd_2 | 0x846920 (wage gate, 0x849957), 0x85fc50 (×3: 0x860310, 0x8604ad, 0x860523), 0x844780 (0x84668f), 0x842e10 (0x84371d), 0x83f3a0 (0x840893), update_wage_bill (0x5983ee), 0x85e960 (0x85eca3) |`
- **Proof:** `refs.py jogo/cm0102.exe 0x864550` (9 relative calls); the function boundaries of `pacote.py` put the call
  at 0x840893 in 0x83f3a0 (the map said 0x840084).

### 1.2 Convention [proved]

stdcall(person, club, flag, squad status, job, kind), `ret 0x18` (0x8665b4), returns the float in st0 loaded from an
f32 ([esp+0x4c], 0x8665a9). ecx is overwritten on entry (`mov ecx, [0xae1050]`, 0x864550): the thiscall wrapper of
`exe_calls.rs` passes an ecx that does not count. The argument slots are reused as scratch ([esp+0x3c] club from
0x865bfc on; [esp+0x48] job at 0x864723/0x864ba4/0x864c73/0x86522b; [esp+0x4c] the result), always after their last
read; the end uses the club in ebx.

### 1.3 §2.3, the rule

- **Old:** "0x864550 `default_staff_contract` and 0x862720 `calc_unset_default_wage` (originals): the default wage by
  ambition, loyalty, pressure, the major nation (0x549e50), the season (0x547770) and the cap (0x5968d0). [inferred]"
- **New [proved]** (0x862720 is not in this round):
  1. The person's contract (`get_contract` 0x864562) goes to [0xae1080] (0x864569); the current club = the contract's,
     except type 5 (0x864570..0x864591).
  2. The level: with a club, reputation / 500 + 1 in a byte (0x8645d2..0x8645eb); with a current club and professional
     status 2, minus one, within 1..4 (0x8645fe..0x864613). Without a club, the world reputation of the player sheet
     (+0xd) or of the non-player sheet (+0xc) / 500, at least 1 (0x86461e..0x86465b).
  3. The job: `job_for_club` (+0x3d) when there is a current club; else `job_for_nation` (+0x28) with a national team
     (+0x24); else +0x3d; else 11 for a player; else +0x28; else 9 (0x864669..0x8646a2). Jump table 0x8665b8 on job − 5.
  4. Staff (jobs 5..10): the level rises to the current club's or the national team's (non-player, national job ≠ 11)
     (0x8646ba..0x864714); base = the non-player current reputation (+0xa)³ × k1 or, without a non-player sheet,
     (id / staff count × 4 + 1) × k2 × 0.0001; wage = level × base × 0.25 (0x864718..0x864761). By job: 5 (k1 2.5e-8
     0x96ff38, k2 25000): at least 750, above 25000 → **150000** (0x864767..0x8649f0); 6 and 7 (1e-8, 10000): at least
     500; 8 (5e-9, 5000): 275..5000; 9 (1e-9, 1000): 250..1000; 10: 200..1000 (0x864cd2..0x864ce3). With a current club
     other than the club: jobs 6, 7 and 8 give 150000 when the current club is more than three levels above (reputation
     / 500 + 4 > level) — for 6 also when the asked job is neither 11 nor 6; without that, 6 below 2500 × 1.35
     (0x86485b..0x8648c3, 0x86498e..0x8649ba, 0x864ade..0x864b2e); 6 and 7 otherwise: above 25000 → 150000 unless the
     asked job is 11 or 6 (0x8649c7..0x8649f0). Job 14 and the others: 1000 (0x864ce3). **150000 is the "not
     available" marker**, not a wage.
  5. Players (jobs 11, 12, 13, 15): without a sheet, the "No player data" message goes to the text 0xde1f64 and the
     wage is `rand() × 4 / 32768 × 550` (0, 550, 1100 or 1650; 0x865699..0x8656e1). With a sheet:
     - status −1 with a club: 1 (current reputation > the club's, 24..33 years), 2 (> 0.75 × the club's, 21+), 4 (> 0.5
       × the club's and 17+, or 25+), 5 (potential × 50 > 0.75 × the club's), 6 (0x864d40..0x864e00); the low byte of
       the argument is rewritten and goes whole to `get_max_wage`;
     - the base: kind 3 85000; kind 1 92500 (loyalty < 15 and pressure > 15) or 75000; kind 2 75000 (23..31 years) or
       82500; asked job 13 or 15 at 36+ 10000; status 5/6: 9350 (current reputation < 2500), 11000 (< 6250); attacker
       ≥ 15: 82500 / 70000 (reputation > 6750, < 31 years, loyalty < 15 / ≥ 15) or 55000; defensive, central or
       attacking midfielder ≥ 15: 67750 / 60000 (> 7250, < 31, loyalty < 14) or 44000; goalkeeper 20: 37500 (world
       > 7250, loyalty < 3, < 34) or 22000; sweeper and defender < 15: 27500; else 50000 (> 7500, loyalty < 6, < 33) or
       27500 (0x864e04..0x864fc6). **The kind 0x14244 of the third test (0x8651bc) never holds: a defect, the 6500
       floor does not exist;**
     - the young (19..31) not tied to the club or the club's nation (club reputation < 5750 or league < 12) with world
       > (loyalty + 230) × 25 − ambition × 10 rises to 75000 / 92500 / 105000 (0x864fca..0x8650c9);
     - FIFA rules ([0xae233c] = 1), above 11000, club of a major nation (0x549e50): × 1.15 / 1.125 / 1.1 (current
       reputation > 6750, by loyalty 1 / < 6 / other) or × 1.025 (> 4250) (0x8650cd..0x865141), not rounded to f32;
     - m = the highest reputation (current, home, world); world < 1750 and m > 3250: m = 3250 − ftol((m − 3250) ×
       −0.2); kind 1 at least 7250, kind 2 at least 6750; base 75000/92500/105000 at least 7250 (0x865145..0x865201);
     - u = (current + home + world + 4m + 50 × (ability + potential)) / 9 + ftol(id / count × 8 − 4); a foreigner
       (another region, `actual_region` +0x76, than the club's and the current club's): at least 1500 or × 1.05; at
       least 1 (0x865206..0x8652c3);
     - wage = base × u² × 1e-8, at least 175 (compared before rounding, 0x8652c8..0x8652f8); × 0.75 or × 0.5 by the
       `state_of_development` (+0x7e) of the current club's nation (or 1 with world > 4250, or the person's, or 3)
       (0x865300..0x865345); × world × 0.0004 (club < 2500) or world / (level × 500), within 0.85..1.15
       (0x865349..0x8653b7);
     - level caps: < 10 → 2500 × level; < 15 → 2750 × level; else 10% above 5250 × level (England, Spain, Italy with
       PLC +0x82), 7500 × level (without PLC) or 3250 × level (others) (0x8653c1..0x865513);
     - position cap: 20% above 150000 (attacker 20), 125000 (midfielders 20), 100000 (defender 20) or 80000; × 1.1 with
       FIFA at a major nation and current > 7250 (0x865515..0x8655ae);
     - without `club_job` (+0x39), except the old (33+) international with work rate + determination ≤ 30, id % 15 ≠ 0,
       at a club ≤ 4500: × (1.375 − d × 0.0125), at least 0.2, d = days since `date_joined_club` (+0x3e, at most 250),
       + 60 in season (0x547770), 20% above 30 (0x8655b2..0x865690).
  6. With a club (0x864ceb; without one it returns right away): for a sheet holder, r = (club reputation × 0.00017 /
     0.00012 / 0.0001)⁶ (pow 0x945ff0), at most 1; by status (table 0x8665e4) the wage goes half-way up to 3 ×
     reputation × r × (1: 0.6 / 1.5 / 1.25; 2: 0.3 / 1 / 0.6; 3: 0.25 and 0.6 / 0.3; 4: 0.1 / 0.3 / 0.15; 5: 0.05; 6:
     0.02, for < 20 years / 24+ at a major nation / others) (0x864cf7..0x86598a); with the contract grievance 0x10
     (+0x45): × 0.67 / 0.67 / 0.85 / 2.5 / 3.5 / 7.5 (table 0x8665fc, 0x86598c..0x8659ec).
  7. The club's nation, when the person works there (`club_job` of the same nation, and is from there or loyalty > 3
     or ambition < 15) or is from there without a club (0x8659f0..0x865a47): Japan at least level² × 10 and 25% above
     7500; England, Italy, Spain nothing; USA × 0.75, MLS 15% above 250, natives 20% above 1250 (32+ or < 24 with world
     < 3750) or 2750, foreigners 25% above 3250; Korea, Scotland, Germany/France/Holland, Sweden, Denmark/Norway,
     Belgium, Portugal/Croatia/Poland, Brazil/Argentina × the factor (club reputation / 50 / `league_standard`) × 0.1 × k
     (k = 0.25 / 0.25 / 0.75 / 0.25 / 0.5 / 0.45 / 0.5 / 0.45), at most 1, when reputation / 50 > league × 10, and
     each one's caps; Wales/Finland/Ireland/Northern Ireland/Australia only caps (200 / 100 × 0.15); the others × 0.3
     and 25% above 15000 (0x865a4d..0x86635b).
  8. Players: at most the wage of the contract with the current club of status 7 (`get_contract_for_club`,
     0x866377..); with `flag`, or without a current club and current < 2250, at most `get_max_wage` (without finances
     it returns the value as it is, without the minimums) (0x8663af..0x866414); without `flag` and with a current club,
     the ambitious (> 10), disloyal (< 6), famous (> 6750) attacker/midfielder/attacking midfielder 20 aged 21+: × 1.15/
     1.1 (attacking midfielder) or, if wage × 1.25 < maximum, × 1.1/1.05 (midfielder) or × 1.2/1.15 (0x866428..0x8664ff).
  9. Minimum: club of status 1: 75 under 20 years, else 100; others: 75 at a club > 2250, else 50 (0x866503..0x8665a9).
- **Extension point:** the default wage is a float returned in st0: a detour that calls the original and multiplies
  the result changes all callers (the demand, the will gate and the wage bill) in one place. With the Rust copy of
  mkd_2, the fine-tuning points are the base by position (step 5), u² and the caps by nation (step 7).

### 1.4 Defects of the original (0x864550)

- `cmp edx, 0x14244` (0x8651bf) compares the kind byte with a number that does not fit in a byte: the 6500 floor never
  applies.
- Without a club, a player with a nation reads the club through a null pointer at 0x865270 (and at level ≥ 15 at
  0x86541b); no caller passes a null club for a player.

## 2. 0x85fc50 `set_to_desired_contract` — item **mkd_3** out (FORA.md); rules proved

### 2.1 Section D table

- **Old:** `| 0x85fc50 | ... | no | — | ... |`
- **New:** `| 0x85fc50 | ... | no (mkd_3 out: size, see FORA.md) | — | ... |` (the map's callers match the package:
  0x47c0b0, 0x4cda70, 0x4ce450, 0x4ce4e0, 0x4de5d0, 0x4e2280, 0x84a020, 0x866620, 0x866830, 0x8bf060, 0x8c0080,
  0x8c9720).

### 2.2 Convention [proved]

thiscall(contract, person, club, offer, squad status, job, kind, &will), `ret 0x1c` (0x862688), returns an int (the
contract value). The arguments at [esp+0x278..0x290] after the prologue (0x85fc7e, 0x85fd66, 0x85ff4a, 0x8602f8,
0x8609d4, 0x8609f5). Frame with the C++ handler 0x962a76.

### 2.3 §2.3, the rule

- **Old:** "0x85fc50 (original, 10,896 bytes): the contract the person wants at the club. Reads ambition, loyalty,
  pressure, professionalism, int_apps; the player value (calc_player_value 0x4d0c60, ×7...); the default wage
  (0x864550, ×3); the will (0x846920 and 0x844780); the cap (get_max_wage, ×4); the window (0x8cf470, 0x8cf0e0); the
  month of the season end (0x4d45f0); a draw. Writes type 2 at +0x35, the status at +0x4f (high nibble), the
  wage/value at +8/+0xc, clauses and the end date. [inferred] An absolute cap of 250,000 in the staff branch
  (fVar27 = 250000.0)."
- **New [proved]:** the counts match (calc_player_value at 0x85ffc8, 0x8615e5, 0x8617c5, 0x8618cd, 0x8619ed, 0x861af2,
  0x861c53; the default wage at 0x860310, 0x8604ad, 0x860523; get_max_wage at 0x861274, 0x8612a2, 0x8612d4, 0x861302).
  The order:
  1. Clears +0x21 and +0x20; without a club, the club becomes the current one (or the linked B team,
     `club_get_linked_b_team` 0x540a50) (0x85fcb0..0x85fd18); the contract with the current club
     (`get_contract_for_club`, 0x85fd2b) is "the current contract".
  2. +0x35 = (+0x35 & 0x40) | 2 (type 2), +0x49 = 0, +0x4a = −1, +0x3b = 0, +0 = id, +0x3a = 0 (0x85fd34..0x85fd56).
  3. Status −1 with a sheet: the deserved one (`get_deserved_squad_status` 0x4ca470) against the current contract
     (grievance 0x10 and not 1 → 2) or `get_squad_status_for_club` (0x849f00); goes to the high nibble of +0x4f
     (0x85fd73..0x85fe40).
  4. Level (byte): with a club, reputation / 500 + 1 (professional status 2: −1, within 1..4), +4 = the club id;
     without a club, the national team's / 500 + 1 (at least 3) or the world reputation / 500; + ftol(league × 0.25 ×
     int_apps × 0.125) (1..4) for internationals (0x85fe3e..0x85ff42).
  5. A player's signing-on value (+8): `calc_player_value` / 5 (0x85ffc8..0x85ffe2), × 0.25 (no club_job, current
     < 2750, club < 2000), × 0.5 (< 4250) or × 0.75 (< 4750); renewal: 0 (< 19 years and < 3750), × 0.1/0.25/0.5 under
     21, × 0.5 at a club < 3500; × 0.9 / 0.75 / 0.75 / 0.5 / 0.25 by the deserved status (table 0x86268c); new club:
     × 0.5 (current < 4250) and × 0.5 (league < 10) (0x85ffe5..0x86016b). Staff: 0 (0x860177).
  6. Without `job_for_club`: the "has no job for club" message and +0x3d = 11 (with a sheet) or 9
     (0x86017e..0x8601a6, 0x8602f4).
  7. Staff (0x8602f8): w = the default wage (flag 0, kind 0); will = `staff_interest_in_job_at_club` (0x844780),
     written to *&will; w + 2500 with will < 1.75; value = w × max(0.9; 3.5 / max(will; 0.1)) (0x861333..0x86137e); job
     6: 250000 if > 2 × reputation + 1200 and > 4100; 8: > reputation + 500 and > 3200; 9: > 0.75 × reputation + 250 and
     > 2100; 10: the same with 1500; and always at most 250000 (0x861375..0x861492). **The "250,000 cap" is the "not
     available" marker (0x48742400 = 250000), not a wage cap.**
  8. Player, renewal at the own club (0x8601e2..0x86045a): the default wage with flag 1 (at most the club's cap) when
     the renewal is "cheap" (no grievance, without the 0xf mark of the 79-byte record 0xae2d0c, current < 7250, fewer
     than loyalty × 12 + 182 days to the end and the Bosman exit marked; or pressure < 6, < 24 years, no future
     transfer, not listed, < 3250 and below the club's reputation + 250, < 200 − pressure days; or a grievance without
     the 0x161a8 bits and few days; or loyalty > 18 / current < 5250 without the mark, with world < 3500); else flag 0.
     New club (0x86045f..0x860528): kind 3 with current > 6750, else 0 (the tests ambition + loyalty > 80 and > 100,
     which would give kind 1/2 and status 1/2, never hold with attributes 1..20).
  9. Player: a demand above the current wage: at most max(2 × current; current + 8000), and for the famous (world >
     6750, > 10000) above that, the cap 75000 (England/Spain/Italy) or 50000, 25% above (0x86052c..0x860699). The
     ambition: t = world + 25 × (ambition − loyalty) + 10 × pressure (16 bits); the boost at a major nation for 19..31
     years (the finances' wage budget +0x10, up to 0.0001 × t × 85000/105000 in England/Spain/Italy, goalkeeper up to
     60000) × (0.9..1.0 + adjustments by age, position and (ambition − professionalism − loyalty + 4) × 0.0075 within
     −0.25..0.15) (0x8606c7..0x8609cc).
  10. Will = `staff_interest_in_joining_club` (0x846920), written to *&will; q = 3.5 / max(will; 0.1); w + 5000 with
      will < 1.75 (no current contract, or a grievance and young/listed/...); q at least 0.9; on renewal q at most
      1.25/1.75/1.5/4.0 by grievance (2 / 1 / 0x8400 / others), 1.25/1.5/2.0/1.35/1.15 by age and deserved status
      (tables 0x86269c, 0x8626b0), 1.1 for the young (< 20, ability < 125 − pressure − ambition, current < 4250, < 120 −
      pressure − ambition days), 3.0 without a contract (0x8609ce..0x860e27); value = q × w (0x860e2f).
  11. At least the current wage × (club reputation / the current club's, at least 0.5) when the club is weaker
      (0x860e4b..0x860ed6); with grievance 0x1000000, up to 95% of the wage budget (0x860ed8..0x860f0c).
  12. "Not available" markers (250000): value > 200000; > 35000 at 35+; > 75000 at 32+; goalkeeper 20 > 105000; status
      7 > 5000, 6 > 8250, 5 > 17500, 4 > 35000, 3 > 57500 (world < 6750; human club `human_manager_is_human_club`
      0x600a50, or outside the 85000/105000/125000 bands), 2 > 125000 (world < 7250) (0x860f10..0x8610f0).
  13. A dearer current contract at another club: value = ((20 − a) × current wage + a × min(w; max(get_max_wage; w ×
      0.5))) × 0.05, a within 0..20 (grievance 0x100 +10, listed +10, wage > 40000, age 34+, happiness and morale)
      (0x86110b..0x86132a).
  14. Clauses (only a player with job 11): +0x1d=0/+0x20=1 for the veteran; non-promotion (+0x1c), relegation (+0x1f),
      minimum fee (+0x1d), non-playing (+0x1e), with the fee (+0x21) by `calc_player_value` and minimums 150000 / 50000
      / 150000 / 1500000; the league ones through the competition virtuals +0x98/+0x9c (type 2) (0x86149a..0x861d1b).
  15. Dates (0x861d1f..0x862371): `date_started` (+0x25) by the window (0x8cf470, or 0x8cf0e0 by the offer) and right
      away rewritten with today (0x861e14); end = day `utils_random(0x1c)` + 1 of the season-end month (0x4d45f0) of the
      year (the next one, by the table 0xb63d77 of the key nation 0x669360) + ftol(L + 1) years; L by age (3; 4;
      3/2/1.5 by position; 1 or 4), × 0.75 with a clause, by level and loyalty, by status (table 0x8626c4), by
      reputation, Finland/Turkey × 0.5 (at most 3), wage < 500/1000/2000 → at most 2/3/4, at most [0xa83608] − 1 = 9, at
      least 1; type 2 with bit 0x80 (no `club_job`, level < 2 and ftol(fixed draw × 4, or × 5 below level × 500 + 1250
      of reputation) = 0); in that last case, up to 20 years, type 1 (+0x35 & 0xc1 | 1) with no extra years; on renewal
      the end never moves back (`date_expires_club` +0x46).
  16. Bonuses (+0x10/+0x14/+0x18) = ftol(value × fixed draw × 0.025 + value × 0.025 + 25) for attacker / midfield /
      goalkeeper 20, else −1; +0x35 |= 0x80 with value < 125 at a club of status 2, cleared above 1250; **+0xc =
      ftol(value)** (0x86237a..0x862493).
  17. Return: ftol(wage (× 0.5 for type 1, × 1.05 with 0x80) + min(sum of bonuses/5 and of clauses × 0.013..0.1;
      value × 0.1)) (0x862496..0x86266d).
- **Extension point (§7, item 2):** confirmed [proved]: +0xc is written once, at the end (0x862493), and +8 before
  (0x85ffe2..0x86016b); the return value does not reread +0xc. A detour that calls the original and adjusts +0xc/+8 on
  the way out is enough. Note: the return value (the contract value) is computed with the wage from before the
  adjustment; a caller that uses the return instead of +0xc does not see the adjustment (check caller by caller before
  detouring).

### 2.4 Defects of the original (0x85fc50)

- 0x861d64..0x861d70: `if (!slot && !*slot)` instead of `||`: with an offer without a slot (0x8c61e0 returns 0) it
  reads address 0 before the error box; with a slot and `*slot == 0` it passes 0 to 0x8cf0e0.
- [esp+0x48] (the reputation used for the clauses and the length) is only written with a sheet + a player job or with
  a non-player sheet (0x85ff46..0x85ff7c); a person with a sheet, a staff job and no non-player sheet reads stack
  garbage (which becomes 1 if it is 0).
- The tests (100 − ambition − loyalty) × 125000 < 0 and (80 − ...) < 0 (0x86045f..0x8604df) never hold with attributes
  1..20: kind 1/2 and status 1/2 of that branch never happen.
- `date_started` computed from the window is overwritten with today right after (0x861e14).
