# NickPatcher: patches that fix errors or crashes of the original

Inventory for the all-Rust exe. Sources read as data (no code copied): the `patches` dictionary of `referencias/CM0102Patcher/Patcher.cs`, the texts of `PatcherForm.Designer.cs` and the `.patch`/`.txt` files of `MiscPatches.zip` (read with `ler_catalogo`/`opcoes_do_patcher` from the Tweaks). Addresses as VA: the catalogue offsets are file offsets (VA = offset + 0x400000); the new code of the patches goes into the section added by `addextraspaceheader` (VA = file + 0x70b000). Routine: `docs/mapa_funcoes.csv` and the community name (`referencias/cm0102-function-map/map/functions.tsv`). "Rust" = the copy in `rust/src/motor/` (found by the `Original: ... (0x...)` header). Disassembly checked against `jogo/cm0102.exe` (3.9.68) with capstone. 08/11/2026.

**Criterion.** Table 1: the target of the patch is a defect of the 3.9.68 code (crash, null pointer, arithmetic that wraps around, competition that hangs, wasted CPU). Table 2: whatever changes a rule, limit, calendar, database or taste; whatever fixes a defect introduced by another patch (Tapani, Saturn, the patcher itself); whatever only serves the start-year change; and whatever only silences the error box.

## Table 1: fixes for defects of the original

| # | Patch (source) | VA changed | Routine | The defect | How the patch fixes it | Rust | In DEFEITOS_DO_ORIGINAL.md |
|---|---|---|---|---|---|---|---|
| 1 | `UnlockYourCreativity` (zip, Misc; also in the `bugfixes` of Patcher.cs, "Apply known bug fixes") | 0x6eda3a (1 byte) | 0x6ec010 `MATCH_PLAYER::calc_match_attributes` (our `update_attributes`), match_pl.cpp | With CA ≥ 125 and intrinsic creativity > 10, a jump skips the calculation and creativity keeps the previous value (the reflexes one). | Changes the jump target so it lands on the creativity calculation. | ported, reproduces: `rust/src/motor/match_pl.rs:9334-9341` (with CA ≥ 125 `l28` is not recalculated) | yes (0x6ec010) |
| 2 | `MarkingAndPositionClampTo100` (zip, Misc; also in `bugfixes`); variant `MarkingAndPositionClampTo75` | 0x6ef644, 0x6ef692 (detours) + new routine in the added section | inside 0x6eecb0 `MATCH_PLAYER::calc_ratings` (the local map splits it at 0x6ef372, without a file), match_pl.cpp | Marking and positioning computed above the ceiling are stored in a byte (+0x110, +0x111) and wrap around to a low value. | Before storing, clamps the value to 100 (or 75). | in the original (0x6eecb0 not ported) | yes (0x6ef644/0x6ef692), without the routine name |
| 3 | `EnsureCashDoesNotResetToZero` (zip, Misc; also in `bugfixes`) | 0x5986e3 (detour) + code at 0x602f05 (space from `tapanispacemaker` in `set_key_ids`) | 0x5986b0 `Finance::get_wealth_band`, finance.cpp | The balance of a very rich club goes past ~2 billion and wraps around. | There, if the low word of the balance fell below −1.5 billion, it restores 2 billion. | ported, reproduces (does not restore): `rust/src/motor/comp/leagues/fin_prm.rs:3112` | yes (0x5986b0) |
| 4 | `memorycheckfix` (Patcher.cs, part of "Make executable portable") | 0x7a1737..0x7a1744 | 0x7a1710 `os_get_free_memory_mb`, os.cpp | Adds free physical memory + free swap file (GlobalMemoryStatus) in 32 bits before dividing by 1 MB: with 4 GB or more the sum wraps around and the game thinks it has little memory (sets [0xdd7138]). | Divides each term by 1 MB before adding. | in the original; the DLL only pins 2047 MB in simulation mode (`rust/src/simulation.rs:380-412`) | no |
| 5 | `idlesensitivity` + `idlesensitivitytransferscreen`, `idlesensitivitytransferstatusscreen`, `idlesensitivitydisciplinescreen` (Patcher.cs, "Enable idle sensitivity") | 0x4e243a, 0x9472d5, 0x8ec743, 0x8f76fe, 0x5a8a10 + routines in the gaps 0x966120.. and in the added section | interface wait loop and screens (0x4e2280 contract_screens.cpp, 0x8ec420 transfer_screens.cpp, among others) | When idle, the game spins the loop without waiting: CPU at 100%. | Adds a wait when there is no event and adjusts the screens that depended on the fast spin. | in the original (screens) | no |
| 6 | `protect_against_virtual_staff_195_error` (zip, Protection) | 0x85d891..0x85d89d, 0x85d8b8 | 0x85d8a0 `Stadium::get_travel_distance_factor`, stadium.cpp | Reads [away team's nation +0x7e] without checking; club without a nation → null → crash ("virtual staff 195"). | A null nation counts as 0. | ported, reproduces: `rust/src/motor/comp/stadium.rs:1034` | yes (0x85d8a0) |
| 7 | `StupidNULLPointerCrash` (zip, Protection) | 0x8c3caa, 0x8c3e13, 0x8c3e4b + code at 0x60301e (Tapani's space) | 0x8c39f0 `TRANSFER_MANAGER::update_appearance_clauses_after_match`, transfer_manager.cpp | Adds match (+4) and absence (+6) to the work permit record found by bsearch (0x946464) without checking whether it was found: null → crash (common with all leagues). | Only adds if the record exists. | ported and **already protected** (differs from the original only in the crash case): `rust/src/motor/transfer_manager.rs:3782-3786` | no |
| 8 | `danny_bhoy67_crash_squad_manager_protection_patch` (zip, Protection) | 0x85a556 + new code | 0x85a470 `SQUAD_MANAGER::allocate_squad_number`, squad_manager.cpp | Reads the person's player record (+0x61) and their positions without checking for null. | Null record: returns without assigning a number. | ported, reproduces: `rust/src/motor/comp/squad_manager.rs:307` | no |
| 9 | `luke_transfer_offer_protection_patch` (zip, Protection) | 0x8e8827 + new code | 0x8e8800 `TRANSFER_RULES::can_make_offer`, transfer_offer.cpp | Dereferences a pointer (eax) without checking for null when comparing the club of the offer. | Null: takes the "not the same" branch. | in the original (the copy only calls it: `rules_can_make_offer`, `transfer_manager.rs:7533`) | no |
| 10 | `NullNewsItemProtection` (zip, Protection) | 0x4aeb0a + new code | 0x4aeaf0 `Text::draw_wrapped`, comp_text.cpp | A null news text goes into `strlen` (repne scasb): crash. The cause of the null is not known. | A null text becomes an empty text. | in the original (interface) | no |
| 11 | `Teebo78_ProtectionPatch` (zip, Protection) | 0x48ce46 + new code | 0x48ce10 `Comp::all_stages_complete`, comp.cpp | Walks the chain of divisions (+0x20) through the table [0xadadfc] and calls method +0x10 of an entry that may be null (competition not loaded). | Null entry: treats the chain as complete. | ported, reproduces: `rust/src/motor/comp/comp.rs:198-199` | no |
| 12 | `transfer_manager_protection` (zip, Protection) | 0x84aebb + gap 0x84aea5 | 0x84aeb0 `Shortlist_Manager::clear_bid_markers_for_player`, shortlist_manager.cpp | Reads the club id through a null club pointer. | Null club: returns. | ported, reproduces: `rust/src/motor/shortlist_manager.rs:4206` | no |
| 13 | `Transfer_Offer_Protection_Patch` (zip, Protection) | 0x8e7bef + new code | 0x8e7b50 `TRANSFER_OFFER::get_weighted_value`, transfer_offer.cpp | Reads player data (+0x61) of a person without a player record. | No record: skips that part of the value. | in the original | yes (line 0x8e7bec in "Save and database", without the patch name or the routine name) |
| 14 | `vascobenny_staff_records_protection_patch` (zip, Protection) | 0x86d606, 0x8c94c5 + new code | 0x86cd70 `Staff_Records::add_transfer_records` and 0x8c9390 `TRANSFER_MANAGER::finalise_transfer` | A player given in exchange whose contract ends before the arrival is left without a club (+0x39 null) and both routines read [club]. | Null club: skips the history and the removal from the squad for that player. | ported, reproduce: `rust/src/motor/staff_records.rs:1772` and `rust/src/motor/transfer_manager.rs:5079-5081` | no |
| 15 | `woerd86_netherlands_protection_patch` (zip, Protection) | 0x5f74ac + new code | 0x5f73d0 `HolFirst::set_club_progress`, hol_first.cpp | The lookup of the Comp_Stats row (0x4a4850, KPN Dutch cup) returns 0 and the code carries on without checking. | Zero: returns from the routine. | in the original | no |
| 16 | `WorldCup_NULL_Crash_Protection` (zip, Protection) | 0x4b07b0 + new code | 0x4b07b0 `compare_clubs_by_reputation_desc` (qsort), comp_util.cpp | The comparator reads +0x80 of a null club pointer in the World Cup list (rare). | Null: copies the other club in its place (duplicates a team instead of crashing). | ported (detoured), reproduces: `rust/src/motor/comp/comp_util.rs:44-46` (`order_by_80`) | no |
| 17 | `ArgentinaFixes` (Protection) and `Possible Argentinian Aug 1st Points Issue` (Misc): the same byte | 0x409d7e (jle → jmp) | 0x409c20 `ArgSecond::update_relegation_averages`, arg_second.cpp | In the averages table of the Argentine 2nd division, it looks for the club by reading [club pointer] of each row; a row without a club crashes the game (on 1 August). [inference about the null row] | Skips the search and goes straight to the empty slot (−1). | in the original | no |
| 18 | `Australian League Bug Fix` (zip, 2021; applied by the "All Tested 2021..2026") | 0x411cd5..0x411cdc, 0x411d24, 0x411d82..0x411d8a | 0x411ce0 `AusNsl::set_up_rules`, aus_nsl.cpp | The NSL is not reset at the end of the season (missing −1 at +0x30) and writes 3 at +0x3c. | Writes 2 at +0x3c and −1 at +0x30 before returning. | ported, reproduces: `rust/src/motor/comp/leagues/aus_nsl.rs:172` (+0x3c = 3) and `:219-226` (no +0x30) | yes (0x411ce0; the cited source is the forum) |
| 19 | 3rd place cancelled (Xeno): loose lines of the "All Tested 2021..2026 + Saturn Patches" | 0x797477, 0x40aee1, 0x40cfd1 (3 → 2) | 0x79746c oceania_club_champ.cpp; 0x40ae90 `AsiaClubChamp::create_round_table`; 0x40cf90 `AsiaCupWinner::create_round_table` (the patch comment calls the 3rd one "CAF Champions League") | The final phase has 3 rounds (semi, 3rd place, final) and the 3rd place sometimes falls after the final and hangs the competition. | Reduces the final phase to 2 rounds, without a 3rd place. | Oceania in the original; the Asian ones ported, reproduce: `rust/src/motor/comp/eurocomp/asia_club_champ.rs:508` and `:537` (`rodadas: 3`) | yes (0x797474...), but the status "in the original" is outdated for the two Asian ones |
| 20 | `Stack Increase` (zip, Misc, no .txt). Generic mitigation | PE header (0x400150 checksum, 0x40015a SizeOfStackReserve) | — | [inference] Overflow of the 1 MB stack (large databases). | Stack reserve from 1 MB → 20 MB. | not applicable; the detoured routines already run on their own stack (`rust/src/detours.rs`) | no |
| 21 | `DoubleHeapMemory` (zip, 2021, no .txt). Generic mitigation | 0x944ee1 (CRT malloc) + new code | 0x944e84 (CRT, heap allocation) | [inference] Writes past the allocated block crash the game. | Doubles the size of every allocation. | in the original (CRT) | no |

**Count.** 19 fixes of localised defects (24 patch names: `bugfixes`, the 75 variant, the Argentine pair, the 4 idle options and the 3 lines of the 3rd place) + 2 generic mitigations. Already in `DEFEITOS_DO_ORIGINAL.md`: 7 (rows 1, 2, 3, 6, 13, 18, 19). Missing 12 (4, 5, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17) and the 2 mitigations. In the Rust copy: 10 ported that reproduce the fault (1, 3, 6, 8, 11, 12, 14, 16, 18 and the Asian ones of 19), 1 already protected (7), the rest still in the original.

**Suggestions for `DEFEITOS_DO_ORIGINAL.md`** (nothing changed): the 3rd place line says "in the original", but 0x40ae90 and 0x40cf90 are already Rust; the 0x6ef644 line can get the name `MATCH_PLAYER::calc_ratings` (0x6eecb0); the 0x8e7bec line is the `Transfer_Offer_Protection_Patch` in `TRANSFER_OFFER::get_weighted_value`; the NSL one is the `Australian League Bug Fix`; row 7 is already fixed in the copy without an "already fixed" line (the comment of `increment_permit_counter` says the original would crash).

## Table 2: rule, limit, calendar, database, taste, or fix of another patch

### `Patcher.cs` options

| Option | Classification |
|---|---|
| `colouredattributes` | taste (coloured attributes) |
| `disablecdremove`, `disablesplashscreen` | taste (CD warning, splash screen) |
| `disableunprotectedcontracts`, `hideprivatebids` | rule |
| `sevensubs`, `remove3playerlimit`, `manageanyteam`, `jobsabroadboost`, `transferwindowpatch` | rule |
| `showstarplayers`, `positionintacticsview`, `addadditionalcolumns`, `addadditionalcolumns_italy`, `changenamecolour` | taste (interface) |
| `allowclosewindow` | taste (the original disables "close" on purpose); `CleanUpOnCloseWindow` fixes the side effect |
| `forceloadallplayers`, `findallplayers` | feature/database |
| `regenfixes` | [debatable] the text says "fix", but it is a rule: limit from 50 → 45 players to receive a regen (0x7ad1bf in `pick_best_club_for_player`), removes two conditions from 0x7acfc0 (`build_nation_club_lists`) and the reputation ≥ 350 branch at 0x7abeab (`regenerate_as_youth_player`, the origin of the old regens of small nations) |
| `fixsuperkeepers` | [debatable] outside the interface; changes the constant 71.0 → 69.0 at 0x96d808 (.rdata), the goalkeeper save threshold in `take_shot` (0x706d1d, 0x706dee): balance. The Rust copy reads the constant from the exe (`match_eng.rs:4867`) and follows a modified exe |
| `to1280x800` | taste (resolution) |
| `tapanispacemaker`, `addextraspaceheader` | infrastructure (space in the exe) |
| `tapaninewregencode`, `makeyourpotential200` | rule (regens, potential) |
| `englishleaguenorthpatch`, `englishleaguenorthpatchrelegation`, `englishleaguenorthawards`, `englishleaguesouthawards`, `chinapatch` | database/competitions |
| `restricttactics`, `changegeneraldat` | rule (restricted mode) |
| `changeregistrylocation`, `removemutexcheck` | infrastructure ("portable") |
| `datecalcpatch`, `datecalcpatchjumps`, `comphistory_datecalcpatch`, `tapanieurofix`, `fixworldcuppre2000` | only for the year changer (they fix what the year change breaks); `fixworldcuppre2000` already cited in DEFEITOS (0x91df0e) |


### `MiscPatches.zip` patches outside table 1 (folders Misc, Protection, fairedinkum, hodgy, 3.9.60, 3.9.65)

| Folder | Patch | Classification / first line of the .txt |
|---|---|---|
| 3.9.60 | `ColoredAttributes_3_9_60` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.60 | `HidePrivateBids_DisableUnprotectedContracts_3_9_60` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_AllowClosingGameWindow` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_ColorAttribs` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_ForceLoadAllPlayers` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_HidePrivateBids` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_IdleSensitivity` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_SpeedHack_x4` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| 3.9.65 | `3_9_65_UnprotectedContracts` | another exe version (3.9.60/3.9.65): port of a Patcher.cs option |
| fairedinkum | `AllowInstantSquadStatusBiddingReset` | rule/taste: "What this patch does is allow you to change a players squad status - https://ibb.co/2ccLPHy - during contract talks, th |
| fairedinkum | `EPLFixtureFixes` | rule/taste: "Here's another patch that people might find handy (and Nick himself might want to include in his patcher), it's an upda |
| fairedinkum | `SaturnPatchUpdateEuroBidsHostsFor2019` | rule/taste: From fairedinkum: |
| fairedinkum | `SaturnPatchUpdateWCHostsFor2019` | rule/taste: Here's another patch that people might find handy (and Nick himself might want to include in his patcher), it's an updat |
| hodgy | `CombineApps` | rule/taste: Will combine appearances on the Player History screen, so instead of 10(10) apps, it will simply show 20. |
| hodgy | `CombineAppsLeagueOnly` | rule/taste: Will do same as CombineApps.patch but show league appearances only like the game stores old histories (full details are  |
| Misc | `95SubsForAllApartFromEngland` | rule/taste: This patch should make all competitions have 9 Named Subs and 5 Usable Subs apart from the English competitions. |
| Misc | `9and5SubsForEverything` | rule/taste: Patch that makes every competition have 9 subs on the bench and 5 that can be used. |
| Misc | `All Tested 2020 + Saturn Patches` | rule/taste (no .txt; by the name) |
| Misc | `AllAvailableJobsInOnePlace` | rule/taste: All available jobs listed in one place. |
| Misc | `AllowFutureTransfersToBeFarOut` | rule/taste (no .txt; by the name) |
| Misc | `AllowJanuaryTransfersInChampionsLeague` | rule/taste: Normally players bought in January would not be eligble to play in Champions League games that season. |
| Misc | `AllowLoansToPlayInEurope` | rule/taste: Taken from a Tapani patch, this allows your loans to register to play in Europe. |
| Misc | `AllowNonEULoans` | rule/taste (no .txt; by the name) |
| Misc | `AllowVeryYoungPlayers` | rule/taste (no .txt; by the name) |
| Misc | `AllowYoungAndOld` | rule/taste: Some managers/non-players you might want to include from real-life, but the game might consider them too old or too youn |
| Misc | `AlwaysRegenToNewGoodCountry` | rule/taste: This checks if a regen player's potential is 120 or higher and if so changes their nation to a different nation (but one |
| Misc | `Android11Patch` | platform compatibility |
| Misc | `Asia Cup add only reputable teams` | rule/taste (no .txt; by the name) |
| Misc | `AttributeMaskingOffByDefault` | rule/taste (no .txt; by the name) |
| Misc | `Australia to Asia V2` | rule/taste: However, this patch actually does |
| Misc | `BrazilSubsTo9` | rule/taste: Changes Brazil's subs from 7 to 9. Taken from the Tapani patch. |
| Misc | `BTeamsIncreaseTo50Players` | rule/taste: Teams that have B teams only have 48 players taken from the database by default. |
| Misc | `CleanUpOnCloseWindow` | fix of another patch: side effect of the Tapani/patcher "close the window" (leftover .tmp files) |
| Misc | `CopaLibertadores9Subs` | rule/taste (no .txt; by the name) |
| Misc | `Coup de France and Trophee des Champions Finals 95 Subs` | rule/taste: Makes the finals of the Coup de France and Trophee des Champions have 9+5 Subs |
| Misc | `Disable virtual_staff.cpp 195 error` | only silences the error box (0x90d4bc, virtual_staff.cpp); does not fix |
| Misc | `DontAskAboutPlayerComparison` | rule/taste: When trying to select a 1st and 2nd player for comparison CM0102 would confirm you are sure you want to set that player  |
| Misc | `DontRejectOffersV2` | rule/taste: Fed up of your board cancelling your transfers at the last minute because they deem they are unrealistic? |
| Misc | `EnableSeasonTicketMoneyOnFirstYear` | [debatable] the 1st year without season tickets comes from the "passed" flag of 0x59b4c0 (ported, `fin_prm.rs:2009`); looks intentional |
| Misc | `English Subs to 9 Named 5 Used` | rule/taste (no .txt; by the name) |
| Misc | `FindAllPlayers` | feature (the search finds everyone) |
| Misc | `FitnessPatch` | rule/taste: Fitness Patch - changes the dates when players have slacking vs training portions of the year. |
| Misc | `FixGoalieStatsV3` | [debatable] experimental, balance (handling x reflexes at load, Database.cpp); source bytes do not match plain 3.9.68 |
| Misc | `FixNetworkGames` | fix of another patch: crash introduced by the Saturn v6+ patches (network in Win95 mode) |
| Misc | `FixSaturnFrenchJokerWindowPatch` | fix of another patch: memory corruption from Saturn v6 (French joker window) |
| Misc | `GiveMoreOptionsInOfferDropDown` | rule/taste: When offering to buy players this gives more options in the dropdown so you can quickly get to the value you want to off |
| Misc | `GiveMoreOptionsInOfferDropDownV2` | rule/taste: When offering to buy players this gives more options in the dropdown so you can quickly get to the value you want to off |
| Misc | `GoldCupCrashFixV2` | fix of another patch: freezes the Gold Cup that the patcher itself (Oct/2021) added |
| Misc | `HalveSigningOnFees` | rule/taste: Xeno pointed out that clubs were spending a significant amount of revenue just on signing on fees, especially for contra |
| Misc | `HideAttributesPlayerScreen68TapaniSaturn` | rule/taste: Use this patch if you want to play blind mode: It hides all attributes in player screen. |
| Misc | `hodgy Replace Northern Ireland With Austria` | rule/taste: Load Austrian leagues instead of Northern Ireland |
| Misc | `Human Manager Attributes` | rule/taste: "Human manager attributes. If you apply below patch form, you will see "your" attributes in the game." --Xeno |
| Misc | `IgnorePlayerHistoriesOnLoad` | performance (skips the history at creation) |
| Misc | `Include SanMarino in Euro Qualifiers (Tapani)` | rule/taste (no .txt; by the name) |
| Misc | `IncreaseLoans` | rule/taste: Increases the number of loans from 2 -> 9 (at least for English clubs) |
| Misc | `IncreasePregensMultiplierTo1.30` | rule/taste: Discovered by Saturn, this will MILDY increase the number of "pregens" added when you start a new game. |
| Misc | `IncreasePregensMultiplierTo1.50` | rule/taste: Discovered by Saturn, this will SIGNIFICANTLY increase the number of "pregens" added when you start a new game. |
| Misc | `IncreasePregensMultiplierTo1.90` | rule/taste: Discovered by Saturn, this will GREATLY increase the number of "pregens" added when you start a new game. |
| Misc | `IncreasePromotionToDivision3To2Places` | rule/taste: This patch changes the promotions to the English Division 3 from the Conference to 2. |
| Misc | `Increase_Font_Size_By_1` | rule/taste: This patch will increase the size of all fonts by 1 |
| Misc | `Italy Subs to 9 Named 5 Used` | rule/taste (no .txt; by the name) |
| Misc | `JL_MakeAttributesShowAsStars` | rule/taste: From John Locke: |
| Misc | `LoansUpdate` | rule/taste: This patches the number of loans England, Scotland, France and Ireland can do to 9 (rather than 2 by default) |
| Misc | `MakeExeRequireAdmin` | infrastructure (manifest) |
| Misc | `MakeFutureLineProperlyTransparent` | fix of another patch: graphical defect "that does not exist in the original" |
| Misc | `match_eng_1225_error_turn_off` | only silences the error box (0x6dd41c, in 0x6dbc50 match_man.cpp); does not fix: the cause is the person repeated in the squad (0x53bd10/0x4cbd20 do not check presence), recorded in [DEFEITOS_DO_ORIGINAL.en.md](DEFEITOS_DO_ORIGINAL.en.md), Others section |
| Misc | `NicksFitnessPatchV3` | rule/taste: This is like Fiestita's fitness patch but instead removes the idea of a "slacking" portion altogether. |
| Misc | `No Complaining About Training New Positions` | rule/taste: Without this patch players complain constantly about training new positions. |
| Misc | `No MLS Foreign Restrictions` | rule/taste: Removes the foreign player restrictions in the US Major League Soccer league |
| Misc | `NoForeignRestrictionsForAll` | rule/taste: Removes the foreign player restrictions from ALL teams. |
| Misc | `NoForeignRestrictionsForEU` | rule/taste: Removes the foreign player restrictions from all EU teams. |
| Misc | `NoMoreRetirements` | rule/taste (no .txt; by the name) |
| Misc | `Northern Ireland and Interamerican Renaming Fix` | fix of another patch: indices broken by the patcher's renamer |
| Misc | `NoTransferFundCountryBiasV2` | rule/taste: This patch ensures the percentage of bank balance available for transfer funds for each nation is equal. |
| Misc | `NoWorkPermits` | rule/taste: Removes the need for any work permits when buying players. |
| Misc | `PageDownPatch` | rule/taste (no .txt; by the name) |
| Misc | `PauseOnMinorInjury` | rule/taste: The game will stop to allow substitutions if your players get an injury, but not if the player picks up a minor knock. |
| Misc | `PlayersAlwaysAskForWageTheyWant` | rule/taste (no .txt; by the name) |
| Misc | `PolishFACupRemove2ndLegs` | rule/taste: Removes all the 2nd leg games from the Polish FA Cup |
| Misc | `Portugal_No_Restrictions` | rule/taste (no .txt; by the name) |
| Misc | `raise_retirement_of_employed_managers` | rule/taste (no .txt; by the name) |
| Misc | `raise_retirement_of_unemployed_managers` | rule/taste (no .txt; by the name) |
| Misc | `ReduceFAAppealTime` | rule/taste: As requested by Offside Trap - this reduces the number of days it takes for the FA to reply to 2 or less |
| Misc | `Remove4MonthLoanLimit` | rule/taste: Loans for players over 18 years old are limited to 4 months. |
| Misc | `RemoveBasqueRestrictions` | rule/taste: Some teams in the Spanish league were limited to only hiring Basque players. This patch removes that limitation. |
| Misc | `RemoveEnglandForeignPlayerRestrictions` | rule/taste (no .txt; by the name) |
| Misc | `RemoveEUOnlyLoanRestriction` | rule/taste: Playing as an EU country you can only loan EU players. This patch removes that restriction. |
| Misc | `RemoveLoanProtection` | rule/taste: As of Saturn v5 and onwards, Saturn has made his releases so that players that are on loan cannot be bought or sold. Thi |
| Misc | `RemoveSecondNameRequirement` | rule/taste: When starting a new game you can now just enter a first name and leave second name blank. |
| Misc | `RetirePatchV4` | rule/taste: This patch ensures that players will not retire until they are at least 34 |
| Misc | `SaturnSpacePatch` | infrastructure (space in the exe) |
| Misc | `Saturn_v8_2019_PointsDeduction` | rule/taste (no .txt; by the name) |
| Misc | `ScoreboardPatch` | rule/taste: This changes the way the scoreboard looks. Take a look here for the difference: |
| Misc | `Secret Attributes Tighter Spacing For Italian Language` | rule/taste: This patch makes the Show Secret Attributes option display all the text when using the Italian language option |
| Misc | `SeeAnyTeamsFinancesV2` | rule/taste (no .txt; by the name) |
| Misc | `ShowPrivateBids` | rule/taste: Tapani's Patcher and Mine can Hide Non-Public Bids. |
| Misc | `StopYoungstersAlwaysRejectingOffers` | [debatable] rule of the under-18 who rejects everything (0x847ef8, shortlist_manager.cpp); design behaviour |
| Misc | `SwapNationalLeagueNorthForSouthernPremierOnSaturn` | rule/taste: This is only for Saturn patched EXEs |
| Misc | `TapaniHighlightPlayersInSearch` | rule/taste: This patch is from Tapani's 2.21 patcher. |
| Misc | `TapaniSpacePatch` | infrastructure (space in the exe) |
| Misc | `Tapani_Relegation_Patch` | rule/taste: This is from the Tapani patch and adds the relegation to leagues that did not have it. |
| Misc | `TurkishSubsForeignRestrictionsPatch` | rule/taste (no .txt; by the name) |
| Misc | `TurnOffForcedAdminRequirement` | infrastructure (manifest) |
| Misc | `TurnOffTapaniRegens` | rule/taste (no .txt; by the name) |
| Misc | `Uncap20s` | rule/taste: Some intrinsic values are capped at 20, even if they would normally be higher. |
| Misc | `UseCommonNamesForBrazilRegens` | rule/taste: This makes Brazil regens always use Brazil Common Names for the regen |
| Misc | `WalesPatch` | rule/taste: This adds Relegation + Foreign Player Restrictions to the Welsh League |
| Misc | `World Cup first year add reputable teams` | rule/taste: If the World Cup is seeded on the first year of play it will add additional teams in alphabetical order. |
| Misc | `Xeno National Squad Increase` | rule/taste (no .txt; by the name) |
| Misc | `Xeno Polish Foreign Restrictions Change` | rule/taste: Poland [5 foreign -> 17 foreign; squad 5 foreign -> no restrictions] |
| Misc | `Xeno Register 30 Players for Euro Comps` | rule/taste (no .txt; by the name) |
| Misc | `Yugoslavia_To_Serbia` | rule/taste: Should remove references to Yugoslavia (and replace with Serbia) |
| Misc | `[HIDDEN} 3.9.68 to Saturn 9 - FA Cup Changes` | rule/taste (no .txt; by the name) |
| Protection | `NoNorthernLeagueJumpPatch` | fix of another patch: removes a Tapani jump (at 0x689c6e plain 3.9.68 already has the bytes the patch writes) |

### Saturn and per-year folders (grouped)

They change competitions, calendar, database, rules or names to update the game, or fix what another community patch introduced. Those named "fix", "crash" or "error" are explained in the following table.

| Folder | Qty | Patches |
|---|---|---|
| 2020 Patches | 20 | `All Tested 2020 + Saturn Patches`, `Champions League Subs Increase`, `Copa America`, `English Fixtures Update`, `FIFA World Club Cup`, `France Restructure`, `French Subs Increase`, `German Subs Increase`, `German Super Cup`, `Greek Subs Increase`, `International Tournament Hosts Update`, `Italy Restructure`, `PlayerRestrictions`, `Recopa Sudamericana`, `Remove Mexico From Copa Libertadores`, `RenameIrishTeamsToOriginalNames`, `ShefWeds Minus 6 Points`, `Squad numbers`, `The African Cup of Nations`, `UpdateGoldenGoals` |
| 2021 Patches | 26 | `African Cup of Nations - 2021`, `All Tested 2021 + Saturn Patches`, `Allow Italian loans + January loans until the end of the season`, `Allow loans between Premier League teams`, `Allow Uefa Super Cup at major stadiums`, `Baseline`, `CONCACAF Gold Cup`, `CONCACAF Tapani Parts`, `Copa America 2024 Start`, `Derby County Minus 21 Points - Reading Minus 6 points`, `DontLoadCPUInf32`, `fairedinkum - International Hosts 2021`, `fairedinkum_ACON_Asian_Cup_Hosts`, `fairedinkum_MiscellaneousPatch`, `fairedinkum_Push Euros_Back_2_Weeks`, `FIFA World Cup 2022 groups for start year 2021 - Xeno`, `France Restructure 2021`, `Irish Leinster Senior Cup fix V2`, `Italy Restructure 2021`, `Oceania OFC Tapani Patch`, `ReplaceLimerickWithTreaty`, `ReplaceMerthyrTydfilWithTown`, `saturn_Correct_South_American_World_Cup_qualifying_schedule`, `World Cup first year add reputable teams`, `WorldCupAdjustments`, `xeno_fairedinkum_FCFWFix` |
| 2022 Patches | 10 | `All Tested 2022 + Saturn Patches`, `France Restructure 2022`, `French Ligue 1 to 4 Relegations`, `Italy Restructure 2022`, `Leinster Cup Add Aditional Leinster League Team`, `Move World Cup to 2023`, `Reverse FA Cup Changes`, `Set Euro Host 2024 To Germany`, `wc_south_american_league_744_crash`, `World Cup 2022` |
| 2023 Patches | 8 | `All Tested 2023 + Saturn Patches`, `Freeze France To 18 Teams`, `Italy Restructure 2023`, `October 2023 Update Patch`, `Set Euro Host 2024 To Germany`, `Set World Cup Host 2026 to Mexico`, `Swedish League Restructure`, `Tapani_Euro_Patch` |
| 2024 Patches | 1 | `All Tested 2024 + Saturn Patches` |
| 2025 Patches | 3 | `2025 Italy Restructure`, `All Tested 2025 + Saturn Patches`, `davdav World Cup Winners` |
| 2026 Patches | 2 | `2026 Italy Restructure`, `All Tested 2026 + Saturn Patches` |
| Saturn v1 | 32 | `2015 International Reschedule`, `Argentinian money`, `Argentinian squad restrictions`, `Australia restructure`, `Background Hires`, `CONCACAF Competitions Reactivation`, `Continental auto load`, `Disable defunct competitions`, `Display Your Star Players`, `English Loans`, `French Leagues Restructure`, `German Super Cup`, `Greek National A + B Division restructure`, `Italian Serie A + B restructure`, `Manage Any Team`, `Olympic Games 2016 entrants`, `Player Restrictions`, `Polish Second Division restructure`, `Recopa Sudamericana rename`, `Recopa Sudamericana`, `Rename 'National Leagues'`, `Republic Of Ireland fixes`, `Reserve Sides Enabled`, `Russian squad restrictions`, `Scout Role Offer`, `Second nationalities in non-European leagues`, `South Korean start date`, `Transfer Windows`, `Wembley fix`, `World Club Championship`, `World Club Cup hosting`, `Younger Regens` |
| Saturn v2 | 17 | `B teams`, `Champions League Fixtures`, `Copa América tiebreaks`, `Copa Argentina`, `Currency`, `English FA Cup Quarter Final`, `Europa League Fixtures`, `French loans`, `International Reschedule`, `Irish loans`, `Other`, `Round Name Changes`, `Turkish Foreign Players`, `USA Clubs Renaming`, `World Cup Qualifying`, `x English loans - reverted from +v1`, `x Work permits - reverted from +v1` |
| Saturn v3 | 37 | `Bosman countries`, `Belgian FA Cup`, `Brazilian Cup`, `Copa Sudamericana`, `Croatian Cup`, `English FA Trophy`, `English Vans Trophy`, `Greek Cup`, `Irish League Cup`, `Italian Cup`, `Italian Serie C Cup`, `Northern Irish Charity Shield`, `Northern Irish Cup`, `Northern Irish League Cup`, `Norwegian Cup`, `Polish Cup`, `Portuguese Cup`, `Portuguese Super Cup`, `Scottish Cup`, `Swedish Cup`, `Turkish Cup`, `USA MLS`, `Welsh League Cup`, `_Competition changes_`, `Competition changes`, `Dutch Eerste Divisie relegation place removal`, `English National League North suspensions`, `English suspensions minor edit`, `English Third Division relegations`, `Golden goals tweak (normal)`, `Job application nudge`, `MLS clubs name changes`, `Real Sociedad Spanish signings`, `Scottish playoffs`, `Squad numbers`, `Transfer windows edit`, `x Irish Leagues Restructure - reverted from +v1` |
| Saturn v4 | 33 | `2017 International competitions`, `African Cup of Nations reschedule`, `Australia to Asia`, `B Teams`, `Canadian Green Card`, `England restructure`, `English loan limit ASCII`, `English National League North rename`, `English National League North substitutes named`, `European competitions' round names`, `Eurox2`, `FAI Cup fixes`, `Fixture fixes`, `French Championnat National fix`, `French League Cup restructure`, `Host nation updates`, `International B Teams removal`, `Job application nudge edit`, `Jobs abroad boost`, `Loans boost`, `Obscene gesture suspensions`, `Portuguese ProLiga restructure`, `Puchar Polski restructure`, `Republic of Ireland loans`, `S.D. Éibar Basque removal`, `Scottish loans`, `Select Leagues ASCII`, `Spelling fixes`, `Taça de Portugal restructure`, `Tottenham Hotspur stadium move`, `Values`, `Version name`, `World Cup 2018 Qualifiers` |
| Saturn v5 | 20 | `2018 World Cup Group Stage`, `AI Loan Players Register`, `Awards fix`, `B Teams`, `CONCACAF Gold Cup`, `Copa América entrants`, `English FA Trophy team selection`, `English National League North awards`, `English National League North relegations`, `FAI Cup restructure`, `FIFA World Club Cup`, `Irish Leinster Senior Cup fix`, `Irish Munster Senior Cup fix`, `Loan_players_purchase_protection`, `Polish match restrictions`, `Regen tweak`, `Register for Europe age increase`, `Republic Of Ireland restructuring`, `Spelling fixes reversed from +v4`, `Version update` |
| Saturn v6 | 45 | `Argentina foreign players allowed`, `Brazilian Champions Cup disable`, `Champions League Saturday final`, `Confederations Cup fixes`, `Copa Libertadores tiebreaks`, `EFL Cup tiebreaks`, `French fixtures reschedule`, `French joker window removal`, `German DFB-Pokal player restriction`, `Girona FC B Team`, `Greek Cup fixtures reschedule`, `International call-ups edit`, `African friendlies`, `Asian friendlies`, `European friendlies`, `North American friendlies`, `Oceanic friendlies`, `South American friendlies`, `International friendlies`, `Ireland - Russia swap`, `FAI Cup fixtures`, `Irish Leinster Senior Cup fixtures`, `Irish Munster Senior Cup fixtures`, `League Of Ireland Cup fixtures`, `League Of Ireland First Division fixtures`, `League Of Ireland Premier Division fixtures`, `Republic Of Ireland & Russia swap`, `Republic Of Ireland game start, training & holiday`, `Russian Cup fixtures`, `Russian Football National League fixtures & restructure`, `Russian Premier Division fixtures`, `Setup`, `Irish Munster Senior Cup team selection`, `Italian Serie A & B playoffs disable`, `Japanese Emperors Cup fixture reschedule`, `Japanese J1 League Final reschedule`, `Portuguese ProLiga restructure`, `Regen CA level reset`, `Regen tweak 2`, `Russian second nationality`, `Scottish Premiership stadium requirements`, `Taça de Portugal restructure`, `Turkish competitions' substitutes`, `Under-21 caps non-binding`, `Version name update` |
| Saturn v7 | 17 | `Atlanta United nation edit`, `Awards fix update`, `EFL Cup fix`, `English FA Cup Fifth Round`, `English transfer window update`, `Goalkeeper training boost`, `Italian transfer window update`, `MLS clubs update`, `North American friendlies edit`, `Norwegian 1. Divisjon changes`, `Norwegian Eliteserien restructure`, `Republic Of Ireland contracts`, `Russian contracts`, `Saudi Arabia restriction removal`, `South American continental finals`, `Turkish TFF 2. Lig fixture changes`, `Version name update` |
| Saturn v8 | 69 | `Asian Cup fixture reschedule`, `Awards fix update`, `B Teams update`, `China & South Korea swap`, `Confederations Cup disable`, `Copa América changes`, `Coupe de Belgique replays removal`, `Croatian league play-offs disable`, `Danish transfer window`, `English competitions fixtures reschedule`, `European Championship Qualifying Playoff nations`, `European competitions fixture reschedule`, `FIFA Club World Cup fix`, `Find Players fix`, `English FA Community Shield fixture reschedule`, `English FA Cup fixture reschedule`, `English FA Trophy fixture reschedule`, `English Football Championship fixture reschedule`, `English Football League Cup fixture reschedule`, `English Football League One fixture reschedule`, `English Football League Trophy fixture reschedule`, `English Football League Two fixture reschedule`, `English National League fixture reschedule`, `English Premier League fixture reschedule`, `UEFA Champions League fixture reschedule`, `UEFA Europa League fixture reschedule`, `UEFA Europa League Qualifying fixture reschedule`, `UEFA Super Cup fixture reschedule`, `Coppa Italia fixture reschedule`, `Coppa Italia team selection adjustment`, `Italian Serie A & B playoffs disable adjustment`, `Italian Serie A fixture reschedule`, `Italian Serie B fixture reschedule & 20 team changes`, `Supercoppa Italia fixture reschedule`, `Spanish Copa del Rey fixture reschedule`, `Spanish La Liga fixture reschedule`, `Spanish Segunda División B minor fixture reschedule`, `Spanish Segunda División fixture reschedule`, `Supercopa de España fixture reschedule`, `African World Cup Qualifying fixture reschedule`, `Asian World Cup Qualifying fixture reschedule`, `Error message suppression`, `North American World Cup Qualifying fixture reschedule`, `Oceanic World Cup Qualifying fixture reschedule`, `South American World Cup Qualifying fixture reschedule`, `Greece restructuring`, `International hosts update`, `Italy changes`, `match_eng error information`, `Physical training fix`, `Polish Ekstraklasa points halving removal`, `Regens mega boost`, `Republic Of Ireland awards date sync`, `Republic Of Ireland fixtures reschedule`, `Russian awards date sync`, `Russian new game start date`, `Spanish fixture reschedule`, `Spanish new game start date change`, `Spanish Segunda División B non-EU players`, `Swedish transfer window`, `Transfer offer button sensitivity fix`, `Turkish reserve matches`, `Turkish Süper Lig minor fixture reschedule`, `Turkish TFF Lig subs`, `Turkish training period extension`, `Türkiye Kupasi update`, `UEFA Super Cup subs`, `Version name update`, `World Cup Qualifying fixture reschedule` |
| Saturn v9 | 4 | `Irish Munster Senior Cup fix`, `MLS clubs update`, `Toronto FC nation edit`, `Version name update` |

### Those named "fix", "crash" or "error" in the Saturn and per-year folders, and why they are left out

| Patch | Why |
|---|---|
| Saturn v4 `Fixture fixes` (+ the loose ones in `Fixture Fixes/`), `FAI Cup fixes`, `French Championnat National fix` | dates and format in the updated calendar/database (rule) |
| Saturn v5/v7/v8 `Awards fix`, `Awards fix update` | awards before the 1st year when the start is not 2001 (only with a year change) |
| Saturn v5/v9 `Irish Leinster/Munster Senior Cup fix`, 2021 `Irish Leinster Senior Cup fix V2` | club names of the updated database |
| Saturn v6 `Confederations Cup fixes` | year and host (rule) |
| Saturn v7 `EFL Cup fix` | defect of Saturn v6 itself (3rd match in the semi) |
| Saturn v8 `FIFA Club World Cup fix` | crash in the draw of the Club World Cup rewritten by Saturn v5 over the code of the old German League Cup (0x5dc532); the source byte does not match 3.9.68 |
| Saturn v8 `Find Players fix` | 3.9.68 already has `mov ecx, 0xb74450` at 0x5a4207: it fixes Saturn |
| Saturn v8 `Physical training fix` | 0 of 87 source bytes match 3.9.68 (0x8b3ce3.., training_manager.cpp): it fixes Saturn code |
| Saturn v8 `Transfer offer button sensitivity fix` | calls 0x966170, a routine placed by `idlesensitivity`: it fixes the coexistence with idle |
| Saturn v8 `match_eng error information`, `Error message suppression` | diagnostics / silenced error box |
| Saturn v5 `Loan_players_purchase_protection` | rule (do not buy loaned players) |
| Saturn v1 `Wembley fix`, `Republic Of Ireland fixes` | stadium/updated database |
| Saturn v2 `Other` + Saturn v5 `Regen tweak` | rule: limit 50 → 45 for regens and `jne` → `jl` at 0x7a6f48; in 3.9.68 the loop of 0x7a6f10 stops at the limit, so `==` and `<` give the same result (there is no defect) |
| 2021 `xeno_fairedinkum_FCFWFix`, `Copa America 2024 Start`, `WorldCupAdjustments`; 2022 `wc_south_american_league_744_crash`; 2025 `davdav World Cup Winners` | years, hosts and slots of the international competitions with the start year changed or the database updated (the 744 one replaces 10 − x with 8 slots in the South American qualifiers, 0x920f1d) |
| 2020..2022 `France/Italy Restructure`, 2021 `African Cup of Nations - 2021` | restructuring; they silence error boxes (match_eng 612, club_records 794, player_stats 1753, world_cup 1438) that the change itself causes |
| line "Portugal Cup would crash if not enough teams" of the "All Tested" | database data (the Taça needs 184 Portuguese clubs); reactivates clubs, no byte in the exe |
