# Does renaming a fixed key stop the save from loading?

Test of 24/10/2026, with the original executable (original mode of the simulation). Tool: `engenharia-reversa/renomear_no_save.py`.

## Short answer

- **[observed]** Yes, but only when you change the **English** name of a key, which is in the save's `eng.lng` item. Loading stops at the box "Unable to find the F_C__BARCELONA index" (or "...BRAZIL index"), `v3.9.68 index.cpp 5809`, which waits for a click.
- **[observed]** Renaming the same club or nation in the save's `club.dat`/`nation.dat` does **not** stop loading in this save (translated names). Changing the short name does not either.
- **[observed, see "Matrix by language"]** In a save with English names (language 0), the key is looked up directly in the `club.dat`/`nation.dat` name. There, renaming a listed key stops the load, with the game in English or in Portuguese. **Renaming the full name of a listed club or nation only in `club.dat`/`nation.dat` is not safe in every language.**

## Why (disassembly)

- `set_key_ids` 0x601a40 has a single call, at 0x52d8fa, at the end of `load_database`. It is the same path for creating a game from `Data\` and for loading a save (`load_game_data` → `load_database(…, 1)`). **[observed]**: the box shows up on load.
- With the name language `[0x9b5984] ≠ 0` (it comes from the tail of the save's `index.dat`; 9 in this save), the game opens `CM3_TEMP\eng.lng` (0x603c5e). The load extracts that file from the save itself. The keys are matched against the English names: clubs through the list 0xae23fc (save row → `eng.lng` row) at 0x603e6f/0x603e7e, nations at 0x604081. Without `eng.lng`, or with language 0, the game uses the name at +4 of the nation record (0x604226) and of the club record (0x604391).
- Only the long name is compared (`_stricmp` 0x951c50, case-insensitive). The short name never is.
- The CMRTE session also saw in the disassembly **[observed]** that `set_key_ids` passes 0x6146b0 only the club's full name (`[0xae23b4]+i+4`), without the short name, and that its editor does not change nation names. This matches the language-0 branch above. In the translated branch, what 0x6146b0 receives is the `eng.lng` row.
- Every key left at −1 calls 0x60d780: it writes "Unable to find the %s index" to 0xde1f64 and opens the box 0x5e8290 with "v3.9.68 index.cpp 5809" (0xae24d0). That box waits for a click.

## `.lng` layout

8 counts (0x20 bytes). Then, in sequence: clubs (0x4f bytes each; 13,640 rows, the whole database and not just the cut one), continents (0xba), nation competitions (0x53) and nations (0x6d). The name is at +0 (51 bytes) and the short name at +0x34 (26 bytes).

## Copies and result

Base: a copy of `jogo\FeriasTesteSave.sav` (type 3 container, 142 items), MD5 `33fda465193b5e8e6d7ace93ecd44559`, the same before and after everything. The copies are in `%TEMP%\cm0102_teste_chave\`. Each load used `-load`, `CM_FERIAS` and `--ate 2001-07-31`, in original mode. A watcher read the game's memory (0xae24d0, 0xde1f64), without clicking, and closed the game when the box appeared.

| Copy | Change | MD5 | Result |
|---|---|---|---|
| `base.sav` | none | 33fda465193b5e8e6d7ace93ecd44559 | loads (2001-211, fingerprint d7abb94be53efcc4) |
| `ida_volta.sav` | rewritten without changes | same as the base, byte for byte | — |
| `a_clubdat_barcelona.sav` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" (club **on the list**) | e42bc66df2017586ca9cd3f760ac0101 | **loads**, same fingerprint |
| `a2_englng_barcelona.sav` | `eng.lng` (clubs): the same name | b85c1b7092a4706cb7e005de379649ee | **box**: "Unable to find the F_C__BARCELONA index", index.cpp 5809; stops within 5 s |
| `b_controle_villarreal2.sav` | "Villarreal C.F." → "Villarreal C.X." in `eng.lng` **and** in `club.dat` (not on the list) | fa973efd94bf1e68590ced241bdf041a | loads, same fingerprint |
| `c_nationdat_brasil.sav` | `nation.dat`: "Brasil" → "Brasix" (nation on the list, translated name) | 3f586e8f7686962dc6ad2f65d8171b0f | **loads**, same fingerprint |
| `c2_englng_brazil.sav` | `eng.lng` (nations): "Brazil" → "Brazix" | 8c4e3554a98cb64c48c454d45809fcde | **box**: "Unable to find the BRAZIL index", index.cpp 5809 |
| `d_curto_barcelona2.sav` | short name "Barcelona" → "Barcelonx" in `club.dat` **and** in `eng.lng` | 46057c6d63911c8350bf79c3cd5946d3 | loads, same fingerprint |

The copies that loaded reached the same date, with the same draws and the same fingerprint as the base. Only the `banco` fingerprint changed, because it covers the renamed names.

## What about the `.qst`?

**[inference]** The same applies. The `.qst` is the same container, with the same `index.dat` (tail and language) and the same `*.lng` files, and its `-load` goes through the same `load_database`, which calls `set_key_ids` 0x601a40 at the end (the single call, 0x52d8fa). A `.qst` that is already created does not carry the keys ready-made: they are globals (0x9cf178..), they do not go into the save, and they are rebuilt from the names on every load. I did not test by loading a `.qst`, because `-load` of a `.qst` stops at the manager-name screen (`docs/COMO_CONTINUAR.md`, "Prova do save").

## The tool

```
python engenharia-reversa/renomear_no_save.py IN OUT --item club.dat --de "F.C. Barcelona" --para "F.C. Barcelonx" [--curto]
python engenharia-reversa/renomear_no_save.py IN OUT --item eng.lng --tabela nacao --de Brazil --para Brazix
python engenharia-reversa/renomear_no_save.py IN OUT          (round trip)
python engenharia-reversa/renomear_no_save.py --carregar COPY.sav|COPY.qst [--idioma 0|9] [--modo original|rust] [--ate YYYY-MM-DD]
python engenharia-reversa/renomear_no_save.py --gerar-qst OUT.qst [--language ENG_UK|POR]
```

- The tool changes only the field, writing the new name and a zero. For type 3, the change is in place. For type 4, it re-encodes the item as valid RLE (b ≤ 0x80 literal, b > 0x80 becomes `0x81 b`) and shifts the following positions in the index.
- At the end, it checks that every decompressed item of the output equals the input's, except the changed item, which must come out as expected.
- Type 4 was checked only by that check, on a copy of `TesteTelasComprimido.sav`. The game did not load any rewritten type 4 save.
- `--carregar` uses the load from `prova_save.py` (bench: laboratory lock, `game.cfg` and `*.tmp` restored) plus the box watcher.
- `--idioma N` sets the game language in the test `game.cfg` for that load only (the bench restores the copy).
- The load extracts the save's `*.lng` into the test game folder, and their content goes into the career fingerprint. So the tool backs up the `*.lng` in that folder first and restores them afterwards.
- Every 2 s, the watcher reads from memory `[0x9b5984]`, the game language `[[0xae3138]+8]`, the Barcelona (0x9d0138) and Brazil (0x9cf260) keys, and the in-memory name of the club behind the Barcelona key.
- With a `.qst`, the load stops at the manager-name screen. The watcher counts the load as done once the key leaves −1 and 15 s pass with no box (`set_key_ids` opens the box right away). It then closes the game.
- `--gerar-qst` writes a `.qst` through the quick start (the one in `prova_save.gravar`) with the requested `LANGUAGE=` and reports the language in the `index.dat` tail.

## Where the language is stored

Found from the disassembly alone, plus a read-only look at `FeriasTesteSave.sav` (MD5 `33fda465193b5e8e6d7ace93ecd44559`, unchanged before and after). The game was not opened. Lucas kept the key-name block for every language, so the language-0 case was not loaded in the game.

- **In memory:** the dword `[0x9b5984]` (`.data`). Loading resets it to −1 at 0x52851c before reading the save. The game's language switch (0x5339c0) writes the new language there at 0x535806, and the getter is 0x535920. **[observed in the disassembly]**
- **In the file:** it is the **last 4 bytes of the `index.dat` item** (decompressed). The writer (0x531400..0x5314b7) writes these in order after the 0x16 records of 0x43 bytes: a flag byte (=1), 0x1cc bytes, the u32 count `[0xae235c]` (−1 if the list `[0xae23fc]` does not exist), the list (count × 4 bytes: save row → `eng.lng` row) and the u32 language. The reader (0x52881a..0x528974) follows the same order. **[observed in the disassembly]**
- **In `FeriasTesteSave.sav`:** `index.dat` is 30,067 bytes, the tail starts at +0x5ca, the count is 7,029 and the language, at **+0x756f** (the last 4 bytes), is **9** (POR). The `eng.lng` item is in the save. **[observed in the file]**
- **Flag 0:** if the flag byte is 0 (a save without the tail), loading sets `[0x9b5984] = 0` (0x528a30) and reads nothing more. **[observed in the disassembly]**
- **Language 0 in a save:** **[observed in the quick-start `.qst` with `LANGUAGE=ENG_UK`; see "Matrix by language"]** it is 0 when the career was created in English (`LANGUAGE=ENG_UK` in the auto setup, or the game running in English) and nobody switched the language afterwards. In that case the key is matched directly against the +4 name in `club.dat`/`nation.dat` (see "Why") and `eng.lng` is not opened. To find a save's language, read the last u32 of `index.dat`. Having an `eng.lng` item is only a hint, because the game opens that file only when the language is ≠ 0.

## Matrix by language

Test of 24/10/2026, requested by Lucas. The question: if the **full name** of a club from the key list (`levas/227/codigo.rs`) is renamed **only in `club.dat`**, as the editor would do (it never touches `eng.lng`), does the save still load under every language setting? Conditions: original simulation mode, test game, hidden 960×540 window, no clicks. The `.sav` loads run until 2001-07-31.

### How the language is chosen when loading (disassembly and observed)

1. **[disassembly]** The `-load` switch (0x81bd70) applies the preferences (0x5d8480 at 0x81be9e) **before** loading. No database is loaded at that point (`[0xae23a8]` = 0), so the language switch 0x5339a0 does nothing.
2. **[disassembly]** `load_database` (0x528540) extracts the save's `*.lng` into `CM3_TEMP` (0x933320). If the save has none, or the game-language `.lng` is missing (0x535930), it sets the game language to English (0x5d8020(0)). That affects only the configuration and the UI, not the names.
3. **[disassembly]** The name language `[0x9b5984]` is read **from the save**: it is the last u32 of `index.dat` (0x52896b). `set_key_ids` (0x601a40, called at 0x52d8fa at the end of `load_database`) matches the keys in that language. If the language is ≠ 0, it matches through the extracted `eng.lng`. If it is 0, it matches through the +4 name in `club.dat`/`nation.dat`. **The game configuration plays no part in this.**
4. **[disassembly]** After the load, `-load` applies the preferences again (0x5d8480 at 0x81c002). If the game language differs from `[0x9b5984]`, it calls the language switch (0x5d85a8..0x5d85bd → 0x5339a0). The switch first goes through English (it applies `eng.lng`) and then applies the new language's `.lng`, which rewrites the database names.
5. **[observed]** At the end of a load, `[0x9b5984]` always equals the game language: a POR save loaded in English ends with 0, and an ENG save loaded in Portuguese ends with 9. When the box shows up, though, `[0x9b5984]` still holds the save's value (0 for the ENG save with the game in POR). So the box comes before the switch.
6. **[observed]** The switch **erases the edit**. For the POR save loaded in English, the club behind the Barcelona key reads "F.C. Barcelona" in memory, not "F.C. Barcelonx". For the POR save loaded in Portuguese, the edit stays ("F.C. Barcelonx").

### Copies

The copies are in `%TEMP%\cm0102_teste_chave\`. `FeriasTesteSave.sav` had MD5 `33fda465193b5e8e6d7ace93ecd44559` before and after everything. The test `game.cfg` was restored after every load, and no `quick_start.cfg`, `auto_setup.cfg` or `PROVA.cfg` was left behind.

| Copy | Source | Change | MD5 |
|---|---|---|---|
| `base.sav` | copy of `FeriasTesteSave.sav` (language 9) | none | 33fda465193b5e8e6d7ace93ecd44559 |
| `a_clubdat_barcelona.sav` | `base.sav` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" (**listed**) | e42bc66df2017586ca9cd3f760ac0101 |
| `e_controle_villarreal_clubdat.sav` | `base.sav` | `club.dat`: "Villarreal C.F." → "Villarreal C.X." (control, not listed) | 0d38b78986479826ed3eba66b3d7bf5a |
| `c_nationdat_brasil.sav` | `base.sav` | `nation.dat`: "Brasil" → "Brasix" (**listed**) | 3f586e8f7686962dc6ad2f65d8171b0f |
| `eng_base.qst` | written by the game: quick start with `LANGUAGE=ENG_UK` (ENG, minimum database); language in `index.dat` = **0**; has the 10 `*.lng` | none | 4c1c985e16fcbda8d4262a5be420772d |
| `eng_clubdat_barcelona.qst` | `eng_base.qst` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" | c01c36f0ac702cdabc6b997ba891b137 |
| `eng_controle_villarreal_clubdat.qst` | `eng_base.qst` | `club.dat`: "Villarreal C.F." → "Villarreal C.X." | 0edf092c83dd5b34beb09e73107b813a |
| `eng_nationdat_brasil.qst` | `eng_base.qst` | `nation.dat`: "Brazil" → "Brazix" | 4d0a618b34ecb07f9803cf4c4a89cef0 |
| `por_base.qst` | the same quick start with `LANGUAGE=POR`; language = **9** (control for the `.qst` method) | none | a08de2630963aac61f3dd02813c682e2 |
| `por_clubdat_barcelona.qst` | `por_base.qst` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" | fac6bf6746062283f073dd5cf4ca01a2 |
| `por_controle_villarreal_clubdat.qst` | `por_base.qst` | `club.dat`: "Villarreal C.F." → "Villarreal C.X." | e07460980ed67da8a3706cf8baed52b0 |
| `por_nationdat_brasil.qst` | `por_base.qst` | `nation.dat`: "Brasil" → "Brasix" | 4f81161cd417025a28cae863f89d8fa9 |

**Why a `.qst` for language 0.** The game does not save while the manager is on holiday, and the only `.sav` with a manager is Lucas's, which is in POR. A genuine English `.sav` would need clicks (the Save menu). The quick-start `.qst` is written by the game itself, with the same container, the same `index.dat` tail and the same `*.lng`. Its `-load` goes through the same `load_database`/`set_key_ids` and only then stops at the manager-name screen, which is enough to answer the question. The `por_*.qst` copies give the same results as the POR `.sav`, which confirms the method.

### Result [observed]

The game language comes from `--idioma` (+8 of the test `game.cfg`; 0 = English, 9 = Portuguese). "Loads" means: the `.sav` reached 2001-07-31 (2001-211); the `.qst` had its keys set and went 15 s with no box. "Box" is "v3.9.68 index.cpp 5809", which waits for a click.

| Save (name language) | Change | Game in Portuguese (9) | Game in English (0) |
|---|---|---|---|
| POR `.sav` (9) | none | loads (previous test: d7abb94be53efcc4) | loads, fingerprint d6c1ca019c200c59, `[0x9b5984]`=0 at the end |
| POR `.sav` (9) | Barcelona in `club.dat` | **loads**, d7abb94be53efcc4; "F.C. Barcelonx" in memory | **loads**, d6c1ca019c200c59; "F.C. Barcelona" in memory (the switch erased the edit) |
| POR `.sav` (9) | Villarreal control in `club.dat` | loads, d7abb94be53efcc4 | loads, d6c1ca019c200c59 |
| POR `.sav` (9) | Brasil in `nation.dat` | loads (previous test) | **loads**, d6c1ca019c200c59 |
| ENG `.qst` (0) | none | loads; `[0x9b5984]`=9 at the end | loads; `[0x9b5984]`=0 |
| ENG `.qst` (0) | Barcelona in `club.dat` | **BOX**: "Unable to find the F_C__BARCELONA index"; `[0x9b5984]`=0, key −1 | **BOX**: "Unable to find the F_C__BARCELONA index" |
| ENG `.qst` (0) | Villarreal control in `club.dat` | loads | loads |
| ENG `.qst` (0) | Brazil in `nation.dat` | **BOX**: "Unable to find the BRAZIL index" | **BOX**: "Unable to find the BRAZIL index" |
| POR `.qst` (9) | none / Barcelona / control / Brasil | loads (all 4); "F.C. Barcelonx" in memory | loads (all 4); "F.C. Barcelona" in memory |

The `.sav` loads with the game in Portuguese repeat the earlier test, this time with the memory snapshot. The `.sav` loads with the game in English all give the same fingerprint as the English base. That fingerprint differs from the Portuguese one because the names differ. Loads from 15:34 on used `ddraw.dll` d9d0817d0d42a834c67a0dc1fc4b377d (the published 228 one), and earlier loads may have used the previous DLL, which was replaced at 15:31. This does not affect the result: everything ran in original mode, and the `.sav` gave the same fingerprint in every load with the same language.

### Conclusion

- **[observed]** **It is not safe in every language.** In a save whose names are in English (last u32 of `index.dat` = 0), renaming the full name of a listed club or nation only in `club.dat`/`nation.dat` **stops the load**, with the game in English **and** in Portuguese. The load stops at the box, which waits for a click. Under Lucas's rule, editing the name stays **blocked**.
- **[observed]** What decides is the language **stored in the save**, not the game configuration: the box comes before the switch to the game language.
- **[observed]** Saves in another language (≠ 0, with `eng.lng`) load under both settings. But the edit disappears whenever the game switches the names, either because the game runs in another language or because of a switch in the options: `eng.lng` and the target `.lng` rewrite the name.
- **[inference]** A save's language changes when it is saved again with the game in another language. At the end of a load `[0x9b5984]` equals the game language, and the writer (0x5314b3) stores that value. A POR save loaded and saved with the game in English therefore becomes language 0, and from then on the same edit stops the load. For that reason, allowing the edit only when the save language is ≠ 0 would not be safe over the save's lifetime either.
