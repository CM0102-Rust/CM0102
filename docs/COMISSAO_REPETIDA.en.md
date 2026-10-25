# The same person repeated in a club's staff slots (`club.dat` +415)

Investigation of 2026-10-25, read-only and disassembly only (the game was not run), requested for warning N6 of the CM3SaveEditor validator. [observed] = seen in the binary or in the save; [inference] = a conclusion without direct proof.

## The slots

The club record (581 bytes) has 15 consecutive slots at +415 (0x19f): 5 coaches (+415..+431), 7 scouts (+435..+459) and 3 physios (+463..+471). In the save an empty slot is **−1** and anything else is the person's id: `save_database` 0x52dad0 writes a null pointer as −1, and `resolve_database_indexes` 0x5315a0 turns a negative id back into null [observed]. **0 is not empty: it is the person with id 0.**

## The cases in the saves

| Save | Repeats at +415 | Where |
|---|---|---|
| `clz_teste\Valencia.sav` (same file as CMExplorer's, MD5 67e35978…) | 6, in 2 clubs | club 894 (Real Avilés Industrial): `[10026, 1379, 10026, 10026, 10026]`; club 2057 (Cusco FC): `[30189, 28530, 30189, 30189, 30189]` |
| `jogo\FeriasTesteSave.sav` | 0 | — |
| CLZ database (`clz_teste\Data\club.dat`) | 0 | — |

[observed] Only the coach slots; never scouts or physios. In `staff.dat` the repeated person has club = that same club and job 8 (coach), has a non-player record and no player record. The 6 repeats sit at +423/+427/+431 (the first occurrence, at +415, does not count as a repeat). Of the 239 N6 hits in Valencia, the other 233 are in other lists: rivals (+179, 58) and preferences (175). The examples in the request (club 498 with id 97604, club 2681 with 132058) **do not occur** in this save: it has 62,487 people, and club 2681 has all 15 slots empty.

## 1. Who writes it: the regen queue (a bug)

`PLAYER_REGEN::process_regen_queue` 0x7a65d0 gives a non-player record to older players at clubs without a human manager. When the club counts fewer than 5 coaches (`database_count_club_coaches` 0x53c8d0) and the ability fits the reputation, the player becomes a **player-coach** (job 15) [observed, 0x7a67b3..0x7a67e8]:

```
7a67ca  mov eax,[esi+0x39]            ; the club
7a67cd  mov ebp,[ecx+eax+0x19f]       ; coach slot j
7a67db  test ebp,ebp / jne 7a67e4
7a67df  mov [eax],esi                 ; write the person
7a67e1  mov byte [esi+0x3d],bl        ; job 15
7a67e4  add ecx,4 / dec edx / jne 7a67ca   ; keeps going: no exit after the first
```

The loop does not stop after the first empty slot, so the person goes into **every** empty coach slot. That is the `[X, Y, X, X, X]` pattern: Y was already in slot 1 and X took the other 4. Later, when he stops playing, `PLAYER_REGEN::retire_from_playing_staff` 0x7aace0 changes job 15 to 8 and removes him only from the squad (+0xd7). The copies in the coach slots stay [observed in code]. This matches what the save holds: job 8, no player record [inference: the sequence was not watched happening].

It is a bug (a missing `break`), not one person holding several roles. Roles live in separate fields (manager +0xcf, assistant +0xd3, squad +0xd7), and no other routine repeats anyone on purpose [observed]. The other routines that write these slots rule out the other hypotheses:

- `database_add_staff_to_club` 0x53bd10 fills only the first empty slot.
- `database_generate_extra_non_playing_staff` 0x527240 and `database_merge_b_team_staff` 0x544970, at game creation, place one new person per slot.
- Loading (0x5315a0) and saving (0x52dad0) only convert id ↔ pointer.
- The CLZ database arrives with no repeats.

Another loophole, with no case in the save: 0x53bd10 does not check whether the person is already in the list, and `database_club_has_room_for_job` 0x53b9a0 treats "already here" as room. That path would also produce a repeat if something re-added a person who is already listed [observed in code].

## 2. Effect in the game

- **Count:** 0x53c8d0 counts every slot, so club 894 has 5 coaches instead of 2. In the regen itself (`> 4`), the club gets no more player-coaches [observed].
- **Free slots:** with all 5 slots taken, 0x53b9a0 reports no room for another coach and 0x53bd10 fails. Hiring through `CONTRACT_MANAGER::sign_contract` 0x4cf720 and the vacant-job filler 0x8cb6d0 cannot place a coach at that club [observed in code].
- **Departure:** `database_remove_staff_from_club` 0x53c190, used in transfers and sackings, clears only the **first** occurrence. When the person leaves, the other copies stay behind as a "ghost" coach pointing at someone now at another club. `MANAGER_MANAGER::remove_staff_from_club` 0x690890 and retirement 0x7aaeb0 clear every occurrence [observed].
- **Training:** `database_get_club_coaches` 0x53cc30 returns the same person several times, but only the training assignment for human clubs (0x8b2880) uses that list. Both cases are computer clubs [observed; effect on human training: inference].
- **Wages:** they come from the contract, one per person, not from the slots. The repeat does not multiply the wage [inference].

**Emptying the repeats is safe.** The result is exactly the state a normal add produces: the person in one slot, with that club and job 8 on his record. Everything that removes the person from the club (0x53c190, 0x690890) still finds him. The club gets its slots back.

## 3. Recommendation for the validator

**Fix it without asking**, only in the coach range (+415..+431):

1. Keep the **first** occurrence and write **−1** in the later ones (never 0, which is the person with id 0). This is the same thing the original validator did.
2. Leave `staff.dat` alone: the club (+0x39) and the job (8, or 15 if he still plays) are already right.
3. No need to compact: the game accepts holes and fills the first empty slot.

Suggested text: *"club N: coach X appears in K slots. This is a game bug (the regen puts a player-coach into every empty slot). The repeats were emptied and the club has coach slots free again."*

A repeat among scouts or physios has no known cause in the game: fix it the same way (−1 in the later ones), but with the generic repeated-list warning. Rivals and preferences are a separate matter and were not investigated here.
