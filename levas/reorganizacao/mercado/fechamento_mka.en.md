*English version of [fechamento_mka.md](fechamento_mka.md). Both are kept in sync; the Portuguese file is the original.*

# Closing the market map, section A §3: the asking price (0x4d1600)

`contract_manager_calc_asking_price` (0x4d1600..0x4d4257, contract_manager.cpp) disassembled in full, instruction by
instruction (`desmonta.py jogo/cm0102.exe 0x4d1600 0x4d4260`, 3135 lines), with the x87 stack operands followed by
hand and the bytes of the register forms checked (`DE F9` = `fdivp st(1)`: st1 ← st1/st0; `DE F1` = `fdivrp st(1)`:
st1 ← st0/st1; `D8 E9` = `fsubr st(1)`: st0 ← st1 − st0; `D8 E1` = `fsub st(1)`). The magic-number divisions were
checked by brute force (−40000..40000): `0x51eb851f sar 4` = /50, `0x10624dd3 sar 5` = /500, `sar 6` = /1000,
`0x66666667 sar 3` = /20, `0xc3a13de7 + add sar 9` = /670, `0xef9db22d sar 6` = /−1000. The item is `levas/fila/mka_2`
(compiled; the bench proof is left to the main session).

**Everything below becomes [proved]** (disassembly). Addresses in brackets. Stack after the prologue: seller
`[esp+0x90]` (ebp), buyer `+0x94`, player `+0x98`, reasons `+0x9c`, adjust `+0xa0` (byte), flag `+0xa4`.

## Signature

- **Before:** `thiscall(contract manager, seller, buyer, player, *reasons, adjust, flag)`, `ret 0x18`; 20 callers.
- **Now:** the same, and: `reasons` is a `ushort*` (set with `or byte [edi+1], k`, cleared with `and word [edi],
  0x800`); `adjust` is read as a byte [0x4d19c6]; `flag` as a dword [0x4d3891]. The 20 callers checked with `refs.py`
  (0x4d4429, 0x4d739c, 0x4d8705, 0x696ec5, 0x7847d4, 0x7b6df1, 0x8402fe, 0x840bfb, 0x8433a8, 0x843574, 0x843b17,
  0x84b0e0, 0x8c0adc, 0x8c1542, 0x8c8291, 0x8c8c6e, 0x8cd391, 0x8e6533, 0x8e6554, 0x8f4ba4). The routine draws nothing
  itself; it writes only `*reasons` (and `G_CONTRACT_LOOKUP_RESULT` inside `calc_player_value`).

## Step 1: early exits

- **Before:** no seller, no contract, not a type 2 contract or a Bosman leaver: 0 [proved]; a human seller on holiday
  without bit 4: the stored value [proved].
- **Now:** confirmed [0x4d160d..0x4d16e7]: `get_contract_for_club(this, player, seller)` [0x4d1663]; `+0x35 & 0x3f ==
  2` [0x4d1676]; `+0x49 == 0` [0x4d1683]; human club with (1, 1) [0x4d1697], `HUMAN_MANAGER::is_on_holiday` [0x4d16b3],
  `get_holiday_option(0, manager)` [0x4d16ce] ≠ 0 and `test [contract+0x4e], 4` [0x4d16d7] → `staff::VALUE` (+0x52).
  One more: **value (step 6) 0 → 0** [0x4d1a69].

## Step 2: the league factor

- **Before:** [inferred] "no visible later use; probably inside an `_ftol`"; formula 1 − (20 − x)² × 0.2/60.
- **Now:** L = 1 − (20 − y)² × 0.2 × (1/60) [0x4d17fc..0x4d1812], between 0.1 and 1.1 [0x4d1818..0x4d1852], kept in
  `[esp+0x64]`. x = `nation::STATE_OF_DEVELOPMENT` (+0x7e, i8), 3 outside 1..3 [0x4d170c..0x4d1728]. **With a
  division:** x = (x + 1) × 0.5, × 4 (`PROFESSIONAL_STATUS` 2) or × 7.5 (3) [0x4d172e..0x4d174e]; y = (division
  reputation² × 25 + club reputation) / (x × 1000) [0x4d1754..0x4d177a]. **Without one:** x × 2 (`fadd st0,st0`) or ×
  7.5, without the (x+1)/2, and y = (`nation::LEAGUE_STANDARD` (+0x85)² × 25 + club reputation) / (x × 1000)
  [0x4d17b3..0x4d17f2]. **No nation: y = 1.** **Use:** L multiplies the premium [0x4d23c1] and the reputation premium
  [0x4d2482] (it is used).

## Step 3: age

- **Before:** above 28, a power factor (0x945ff0).
- **Now:** A = (1 − (age − 28) × 0.1)^2.0 [0x4d1865..0x4d188a] (`_CIpow` with the base in st1, the exponent 2.0
  [0x968950] in st0), `[esp+0x74]`; it multiplies the premium [0x4d23c5] and the reputation premium [0x4d2486]. Age =
  byte `staff+0x18`.

## Step 4: happiness and role

- **Before:** the shortfall up to 75/50/40/25/0 for roles 1, 2 and 5, 3, 4, 6; role 7 → happiness −100. Absent without
  leave: the seller manager's discipline zeroes the terms.
- **Now:** role = `SQUAD_STATUS` >> 4 (jump table 0x4d4258) [0x4d188e..0x4d1932]: 1 → max(75 − h, 0); 2 and 5 → max(50
  − h, 0); 3 → max(40 − h, 0); 4 → max(25 − h, 0); **6 → max(−h, 0)** (not 0); 7 → h = −100 and shortfall 0; 0 and 8+ →
  shortfall 0. h = `contract::HAPPINESS` (+0x3d, i8; handled as 16 bits). **Absent** (`AWOL::is_player_awol(player, 1)`
  [0x4d1944]): shortfall 0, the **adjust switched off** (bl = 0) and h ≤ −5 × the seller manager's
  `non_player::DISCIPLINE` (+0x14) (h = −100 without a manager) [0x4d194d..0x4d19c4]. Otherwise the adjust comes from the
  5th argument [0x4d19c6].

## Step 5: morale

- **Before:** morale × 0.00746 (negative) or 0.00333; × 0.8333 **without** grievances or 1.2 **with**.
- **Now (corrected):** M = 1 + morale × 1/300 [0x968e60] (positive) or × 1/134 [0x969090] (zero or negative); **× 1.2
  without grievances** (`GRIEVANCES` +0x45 = 0) [0x968978] and **× 0.8333 with them** [0x969088]
  [0x4d19cd..0x4d1a1a]. The map had it reversed.

## Step 6: the value (0x4d1a5a)

- **Before:** the value = `calc_player_value(player, seller, 0, ability seen by the manager (0x6e86d0), 0)`; 0 → 0.
- **Now:** the "seen ability" is the seen **potential**: `perceived_ability(the seller's manager (+0xcf), player,
  seller, 1, rep)` [0x4d1a43], kind 1 (potential; 2 would be current) and rep = `player_sheet::CURRENT_REPUTATION`
  (+0xb) / 2 + `POTENTIAL_ABILITY` (+7) × 25 in 16 bits (`imul dx, dx, 0x19`) [0x4d1a21..0x4d1a35]; cdecl, `add esp,
  0x14`. Then `calc_player_value(this, player, seller, 0, seen, 0)` [0x4d1a5a] (ecx is read from `[esp+0x40]` before
  the `add esp`, = `this`); only the low 16 bits of seen are valid (`mov ax, ..` on return) and `calc_player_value`
  reads only 16 bits. V in `[esp+0x14]` and `[esp+0x1c]`. **Extension point confirmed:** the price is V + premium(V);
  scaling V scales everything, and "seen" is the only place where the seller's manager (his judgement) enters.

## Step 7: "contract running out" → **recent signing**

- **Before:** [inferred] with `adjust`, computer seller, not listed: a factor by role (7.5; 6.0; 5.5; 3.5) minus days to
  the end (121 to 365) / (365 / (f − 1)); above 2 sets 0x100.
- **Now (corrected):** the days are **since the player joined the club** (the player's `staff::DATE_JOINED_CLUB`
  +0x3e/+0x40, not the contract): d = 365 × (current year − year joined) + current day − day joined
  [0x4d1aff..0x4d1b25], between 121 and 365 (negative → 121) [0x4d1b2b..0x4d1b43]. f by role (table 0x4d4274, f32): 2 →
  6.0 [0x969084]; 3 → 5.5 [0x96907c]; 4 and 5 → 3.5 [0x969080]; the others → 7.5 [0x969078]; roles 7 and 8 out
  [0x4d1abc..0x4d1ac8]. C = f − d / (365 / (f − 1)) [0x4d1b4b..0x4d1b63]: f up to 4 months after joining, 1 after a
  year. Above 2 sets 0x100 [0x4d1b6b..0x4d1b7c]. Conditions: adjust on, a computer seller (`is_human_club(.., 0, 1)` =
  0), the player neither listed nor requesting (0x544520). C enters Q, the D²C cap, the premium (× min(C², 25)) and the
  3V × D × C cap.

## Step 8: high happiness

- **Before:** (shortfall/2 + happiness) × 0.01167 + 1.4165, and above 49 at a computer club × (… × 0.006 + 0.7);
  reasons 0x200 (≥ 61) and 0x400 (21..60).
- **Now:** H = (shortfall/2 + h) × 0.01167 + 1.4165 [0x4d1b80..0x4d1bae]; with h ≥ 50 and a computer seller: H =
  ((shortfall/2 + h)² / 50 × 0.006 + 0.7) × H [0x4d1bb4..0x4d1bf7]. Reasons: h > 60 → 0x200, else h > 20 → 0x400
  [0x4d1bfb..0x4d1c11] (only with `reasons`).

## Step 9: the seller's finances

- **Before:** the balance against the table 0x9a7f48 by reputation / 500 (50 thousand to 25 million): rich, balance /
  table; poor, damped; added to happiness × 0.002 and the observation nibble² × 0.05; ≤ 1 → × 0.2125 + 0.7875, else ×
  0.25 + 0.75; at least happiness × 0.0025 + 0.5, at most 2.5.
- **Now:** the table read from the exe (21 doubles): 50k, 100k, 200k, 300k, 400k, 500k, 750k, 1, 1.25, 1.5, 2, 4, 6, 8,
  10, 12.5, 15, 17.5, 20, 22.5, 25 million. B = `finance::BALANCE` (i64, 0 without finances) [0x4d1c15..0x4d1c3a]. B ≥
  T: B / T [0x4d1cbc]; B < T: ((B − T)/T + x) / (x + 1), x = B × 1.25e-6 [0x969050] between 0 and 10
  [0x4d1c64..0x4d1cba]. G = (that + n² × 0.05) + h × 0.002 [0x4d1cbe..0x4d1d03], n = the low nibble of
  `staff_state::STATUS_FLAGS` (+0xb of the 79-byte record) [inferred: the knowledge/scouting level]. G > 1 → × 0.25 +
  0.75; else × 0.2125 + 0.7875 [0x4d1d05..0x4d1d2c]. Floor h × 0.0025 + 0.5 (when the floor wins there is **no** cap);
  else cap 2.5 [0x4d1d30..0x4d1d6c]. G also caps Q (≤ 6G).

## Step 10: the buyer

- **Before:** Celtic × Rangers: 2.3 + …; a derby or rivals, same stadium city or not: 1.3 to 2; same division (outside
  0x9cf764): 1 + … × 0.005 (reasons 8 and 0x10).
- **Now:** D = 1 without a buyer [0x4d2002]. h' = h, or ftol(h × 1.5) when h > 20 without grievances
  [0x4d1d83..0x4d1da7].
  - Celtic × Rangers (both clubs among `G_KEY_CLUB_GLASGOW_CELTIC`/`RANGERS`): D = max(h' × 0.04 + 2.3, 2.3)
    [0x4d1fb5..0x4d1fe7];
  - a league clash (`comp_util_get_league_clash_type(buyer, seller)` ≠ 0 [0x4d1dd4]): same city (`stadium::CITY` +0x38
    of both grounds): max(h'/30 + 2, 2); else max(h' × 0.02 + 1.6, 1.3) [0x4d1de4..0x4d1e62];
  - a rival of the seller (`club::RIVAL1..3` = the buyer): same city max(h' × 0.02 + 1.6, 1.3); else max(h' × 0.01 +
    1.3, 1.3) [0x4d1e88..0x4d1f9a];
  - these three set **0x800** with D ≥ 1.3 [0x4d1e6e, 0x4d1fa2, 0x4d1fef];
  - same division (same pointer, and the buyer's division id ≠ `KEY_CLUB_COMP_A_LOWER_DIVISION`): max(h' × 0.005 + 1,
    1), sets **0x1000** with D > 1.3 [0x4d1ea8..0x4d1f1b].
  (The reasons are 0x800 and 0x1000, not 8 and 0x10: the `or` is on the high byte.)

## Step 11: squad importance

- **Before:** ability above reputation/50 × 17/20, age < 25, role < 7: 1 + … × 0.04 (reason 0x20).
- **Now:** k = reputation/50 + 1 (i16), threshold = k × 17 / 20 [0x4d2006..0x4d203d]. With `reasons`, role 5 sets
  **0x2000** and skips the rest [0x4d2043..0x4d2052] (without `reasons` it goes on). Seen > threshold, age < 25, role < 7
  [0x4d205b..0x4d2085]: with `reasons`, role 6 sets 0x2000 and skips; else S = 1 + ftol((seen − threshold) × k × 0.005)
  × 0.04, at least 1 [0x4d20a2..0x4d20f5]; **role 4 also multiplies H by S** [0x4d20f9..0x4d2107]; S > 1.6 sets 0x2000
  [0x4d210f..0x4d211c].

## Step 12: the price

- **Before:** price = value + value × (the product of the factors, capped 4.25..10 and 6 × the finance factor), between
  −3 and 50 million; minimum 1000. Work permit and listed: the reasons keep only 0x800. A rich club with reputation >
  4250 and age > 18: adjusted by the wealth band.
- **Now:**
  - Q = G × H × M × S × D × C (in this order) [0x4d215c..0x4d2177], at most clamp(D² × C, **4.25 is the floor**
    [0x969008], 10) [0x4d2124..0x4d218a] and at most 6G [0x4d219e..0x4d21b5]. With D = C = 1 the cap is 4.25.
  - R = ftol((current reputation + `HOME_REPUTATION`) × 0.3 + `WORLD_REPUTATION` × 0.4) [0x4d21bd..0x4d21fb]; e = (club
    rep + R − 4000) × 225 between 150 thousand and 2 million [0x4d2200..0x4d223e]; E = (((1 + (R − (1.2 rep − 2000)) ×
    R × 5e-8) × u + 100) − u) × 0.01, u = h × 0.5 + 50 [0x4d2246..0x4d228c]; above 1, E⁵ (`_CIpow`)
    [0x4d22a3..0x4d22b2]; E between 1 and 3 [0x4d22b6..0x4d22f2].
  - raw premium p = ftol(V × Q − V) [0x4d2307..0x4d231b]. **Negative:** max(p, −0.4 V) [0x968fe8], and at least −3
    million [0x4d232e..0x4d236e]. **Positive:** min(p × C, ftol(e × Q)) × min(C², 25) × L × A [0x4d2373..0x4d23d2];
    halved when listed or requesting [0x4d23d8..0x4d2401]; up to 50 million [0x4d2403].
  - price = V + premium; with E > 1, + ftol(L × A × t), t = ftol(price × E − price), above 5 million 0.625 t + 1.875
    million, above 10 million 0.125 t + 8.75 million [0x4d2410..0x4d2495]; at least 1000 [0x4d2499].
  - **Work permit** (`player_needs_work_permit(player, seller, 0)` [0x4d24bb]): at most V (reasons unchanged)
    [0x4d24c4..0x4d24d0]. **Listed or requesting:** the reasons keep only 0x800 and the price ≤ `staff::VALUE` × D
    [0x4d24d5..0x4d257a]. **A club with a linked B team** (`club_get_linked_b_team(the player's club, &b, 1)` ≠ 0 and b
    = 0, i.e. the A side) and role not 1, 5, 2, 3: the same [0x4d250f..0x4d257a]. (`b` is not initialized in the
    original; the callee writes it whenever it returns a club [0x5412a7, 0x54187c → 0x541e42].)
  - **The rich club's floor (corrected):** a computer seller, reason 0x800 already set, reputation > 4250, age > 18,
    role ≠ 7, wealth band ≠ −1 and −2: if the price < (seller rep + 25000) × 100, the price becomes (rep **of the
    player's club** (`staff::CLUB_JOB`) + 25000) × 100 [0x4d2585..0x4d261b]. Not an "adjustment by the wealth band": the
    band only filters.
  - **New:** at most max(3V, V + 5 million) × D × C [0x4d261f..0x4d267a].

## Step 13: grievances

- **Before:** grievances (& 0x6b or 0x100) with less than a year of contract: the price goes down towards the value.
- **Now:** price > V and `GRIEVANCES` & 0x16b [0x4d267e..0x4d2692]: days = days to the end of the contract
  (`CONTRACT_EXPIRES` +0x2d/+0x2f) [0x4d2698..0x4d26bf]; V' = V (≥ 365 days) or ftol(days/365 × V); w = days × 0.0027 −
  0.485 between 0 and 1; p' = ftol((1 − w) V' + w × price) [0x4d26e7..0x4d274d]; ratio = (V' + p') / max(2 × B × D, 1)
  between 1 and 3; y = 1.375 − 0.375 ratio; price = ftol((1 − y) V' + y p') [0x4d2754..0x4d27e3]. It also applies with
  more than a year of contract (then only the balance pulls).

## Step 14: the big nations → ladders A and B and the pull to the value

- **Before:** [inferred] big nations with reputation ≥ 4001: multipliers by role (1.25; 1.1; 1.4; 1.15; 1.5; 1.75; ×2 up
  to +2M; ×5 up to +5M) and 1.025..1.1 more in the same division; the contract and staff flags.
- **Now:** the **seller's** shortlist entry for the player (`find_club_shortlist_entry(seller, player, 0, 1)`
  [0x4d27ff]). "Big nation" [0x4d2806..0x4d28e1]: reputation > 4000, nation in {England, Italy, Spain, France, Germany,
  Scotland, the USA, Japan, Holland, Portugal}, (`LEAGUE_STANDARD` > 12 or reputation ≥ 7000), finances with a wealth
  band ≠ −1/−2, the player neither listed (0x5444a0) nor listed by request (0x544600).
  - **Ladder A** (an entry targeting the player, `TARGET_STAFF_ID`, or `staff::SQUAD_SELECTED_FOR` = 2 with a
    continental competition) [0x4d28e7..0x4d2905]: the youth step (below 25, role not 6/7, seen × 50 > reputation):
    min(seen / (125 − morale/4) × price, price − |age − 20| × 1,333,333 + 5 million) [0x4d290b..0x4d29b0]. Small club
    (rep ≤ 6750 and division reputation ≤ 15 or no division): with grievances and morale ≤ 0, only with grievances 0x101
    and more than 425 contract days (`Date::days_since`): role 1 +375k/×1.25 (same division +50k/×1.05), role 2
    +200k/×1.2 (+37.5k/×1.025); without: role 1 +750k/×1.75 (+100k/×1.1), 2 +400k/×1.4 (+75k/×1.075), 3 +150k/×1.15
    (+25k/×1.025) [0x4d29b4..0x4d2cf0]. Big club: with grievances and morale ≤ −50: (0x101 and > 425 days) role 1
    min(+2.5M, ×2) (+1M/×2), 2 +1.5M/×1.5 (+750k/×1.75), 3 +1M/×1.25 (+400k/×1.25); else, with a buyer, > 225 days, role
    ≠ 7, professionalism and loyalty > 5, current reputation > 6250, happiness > −loyalty, aged 23 to 30: +1M/×1.25 and,
    in the same division with a league clash (`clash_type(seller, buyer)`), +400k/×1.25; without serious grievances:
    role 1 min(+5M, ×5) (+1M/×2), 2 +2.5M/×2.5 (+750k/×1.75), 3 min(+1.5M, ×2) (+400k/×1.4) [0x4d2cf5..0x4d3135]. Each
    step is min(price + K, price × f).
  - **Ladder B** (an empty entry, target −1) [0x4d313a..0x4d3824]: the youth step below 22; small club without
    (grievances and morale ≤ 0): role 1 +250k/×1.25 (+50k/×1.05); role 2, or another with happiness > 40 − loyalty,
    +100k/×1.1 (+25k/×1.025). Big club: with grievances and morale ≤ 0: (0x101, > 425 days) role 1 +500k/×1.25
    (+250k/×1.1), 2 +1M/×1.2 (+200k/×1.05), 3 +750k/×1.15 (+150k/×1.02); else (> 225 days, role ≠ 7, loyalty +
    professionalism > 10, current reputation > 5250, happiness > 10 − loyalty, aged 23 to 30) +750k/×1.15
    (+150k/×1.02); without: role 1 min(+2M, ×2) (+100k/×1.1), 2 +1.5M/×1.5 (+75k/×1.075), 3 +1M/×1.5 (+20k/×1.02).
  - A big nation with an entry targeting someone else: nothing.
  - **New, important:** **without a shortlist entry** and outside the big-nation conditions (or a big nation without an
    entry that does not fall in ladder A), price > V: h < 60 → 0.67 V + 0.33 price; else t = 1.27 − h × 0.01 and (1 − t)
    price + t V [0x4d3826..0x4d387e]. It is the common case outside the big leagues: the price comes back two thirds of
    the way to the value.
  - The contract flags (0x8676f0), loyalty, professionalism and morale are **not in the ladders**: they are in the
    youngster floor (new step 15).

## Step 15 (new): the youngster floor and the manager's Youngsters (0x4d3ccb)

- **Before:** only mentioned in the request ("the role of the manager's Youngsters at 0x4d3ccb").
- **Now:** with the flag (6th argument), age < 24, seen > 110, a nation and role ≠ 7 [0x4d3886..0x4d38cd]: si = seen;
  outside goalkeepers (`GOALKEEPER` = 20): age factor a = 2.6 − 0.07 age; physical = ftol(((stamina × 1.2 + strength) ×
  a + teamwork × 0.8) + work rate × 1.2 + jumping × 0.75); dribbling and finishing = `staff_get_player_attribute(11 /
  12)` × a, up to 20 [0x4d38d3..0x4d39d7]; by pace + acceleration: 40 → +15/+10/+8/+5/+3 by physical (85/75/65/50/35)
  and + (dribbling − 13)², + (finishing − 14)²; > 34 → +8/+5/+3/+1, (dribbling − 15)², 0.8 × (finishing − 15)²; > 25 →
  +5/+3/+1, −3 with physical < 35, 0.8 × (dribbling − 15)², 0.5 × (finishing − 15)²; < 15 → −5/−8/−10
  [0x4d39db..0x4d3ba2]. With the entry targeting the player: + rep/1000 [0x4d3ba5..0x4d3bd6]. Role 1: + rep/500 (under
  19 at least 121); 2 and 3: + rep/670 (19+) or rep/500; 4 under 19: + rep/670 [0x4d3be1..0x4d3c59, 0x4d3d61..0x4d3d83];
  above 20 with current ability < 45: − rep/1000 [0x4d3c5b..0x4d3c83]; seen > 120 and `club::TRAINING` > 10: + training
  − 10 [0x4d3c85..0x4d3c98].
  **The Youngsters [0x4d3c9c..0x4d3cd0]:** with a manager (`club::MANAGER`) and a non-player sheet, at a **computer
  club** (`is_human_club(seller, 1, 1)` = 0): si += `non_player::YOUNGSTERS` (+0x22) − 10 (in 16 bits). It is the only
  place the manager enters this routine besides judging the seen potential. Then, if the player joined the club after the
  manager (`Date::operator>` of the dates joined) and si < potential: + half the difference [0x4d3cd4..0x4d3d15].
  si > 120 [0x4d3d17]: si ≤ clamp(rep × 0.0167 + 100, 150, 220) [0x4d3d21..0x4d3da9]; bonus (si > 150, entry, rep ≥
  2500, league standard > 10) = (min(4 si − 600, 200))² × 250 × rep / 6000 [0x4d3dab..0x4d3e0b]; Y = ftol((si² − 14400) ×
  250 × 0.6153846 + bonus) [0x4d3e13..0x4d3e3b]; standard < 13 or status 3: × r/5250 with r the reputation (× 0.75 at
  status 3 below 5250; standard < 5 above 4000: 4000 + (r − 4000)/5) [0x4d3e40..0x4d3ee4]; grievances: × 0.75 (morale <
  −50), × 0.8 (0x800000), × 0.95 (contract flag 8), × 0.9 (1), × 0.9 (0x40) [0x4d3ee8..0x4d3f88]; loyalty < 6 and
  professionalism < 10: × 0.9 (morale < 0) and × 0.9 (buyer 750 above); loyalty > 15 without negative morale: × 1.1
  [0x4d3f8c..0x4d4013]. The price is at least Y, with reason 0x2000 [0x4d4015..0x4d402a].

## Old step 15 (age cap) → step 16

- **Before:** [proved in the pseudo-C] 29 (26 for a goalkeeper): value + 5M ... 38 to 45: the value.
- **Now (goalkeeper corrected):** [0x4d402e..0x4d4153], table 0x4d4288: a goalkeeper (`GOALKEEPER` = 20) counts 3 years
  less, so his cap starts at **32** (not 26); 29 +5M, 30 +4M, 31 +3.25M, 32 +2.5M, 33 +1.75M, 34 +1M, 35 +500k, 36
  +200k, 37 +100k, 38..45 V; above 45, nothing.

## Old step 16 (Portugal, Korea) → step 17

- **Before:** Portugal above the value: < 6M → × (price × 1.667e−7 × 0.6 + 0.3); else × 0.9. Korea: an adjustment.
- **Now:** Portugal, price > V: below 6 million, x = (price × 1.667e−7 × 0.6 + 0.3) × price; else 0.9 × price; price =
  max(V, x) [0x4d416b..0x4d41eb]. South Korea, price > V and > 50000: (price − 50000) × 0.15 + 50000
  [0x4d41ed..0x4d421d].

## Old step 17 (release fee) → step 18

- **Before:** [proved] the active fee caps it.
- **Now:** confirmed [0x4d422a..0x4d423e]: `has_active_release_clause` and price ≥ `RELEASE_FEE` (+0x21) → the fee.

## Extension points (revised)

- The return of 0x4d1600 (20 callers) and `*reasons` (0x100 recent, 0x200/0x400 happiness, 0x800 rival/derby, 0x1000
  same division, 0x2000 important/young).
- The value call [0x4d1a5a]: V enters linearly (V + V × (Q − 1) × ...).
- The 4.25 floor of the cap [0x969008] and the 6G: they control the whole premium (with D = C = 1, Q ≤ 4.25: the
  premium reaches 3.25 V).
- The pull to the value without an entry [0x968f80 = 0.67, 0x968f78 = 0.33]: the rule that weighs most outside the big
  leagues.
- The Youngsters [0x4d3ccb]: `movsx cx, [eax+0x22]` and `lea esi, [esi+ecx−0xa]` (the −10 is an immediate).
- The wealth table 0x9a7f48 (21 doubles) and the age cap (immediates 0x4d4067..0x4d4133).

## Defects of the original

- **[proved]** In step 4 with absence without leave, the high 16 bits of `[esp+0x24]` hold garbage (the `movsx ax` over
  the manager pointer); only the low 16 are read: no effect.
- **[proved]** The rich club's floor compares with the seller's reputation and writes with that of **the player's club**
  (`staff::CLUB_JOB`), normally the seller itself; for a loaned player or one of another club, the two differ
  [0x4d25ea / 0x4d2603].
- **[proved]** The B-team `b` is not initialized; no effect because the callee writes it whenever it returns a club.
