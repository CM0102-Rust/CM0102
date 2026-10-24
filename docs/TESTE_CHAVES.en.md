# Does renaming a fixed key stop the save from loading?

Test of 24/10/2026, with the original executable (original mode of the simulation). Tool: `engenharia-reversa/renomear_no_save.py`.

## Short answer

- **[observed]** Yes, but only when you change the **English** name of a key, which is in the save's `eng.lng` item. Loading stops at the box "Unable to find the F_C__BARCELONA index" (or "...BRAZIL index"), `v3.9.68 index.cpp 5809`, which waits for a click.
- **[observed]** Renaming the same club or nation in the save's `club.dat`/`nation.dat` does **not** stop loading in this save (translated names). Changing the short name does not either.
- **[inference, from the disassembly]** In a save with English names (language 0), the key is looked up directly in the `club.dat`/`nation.dat` name. There, renaming a listed key stops loading.

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
python engenharia-reversa/renomear_no_save.py --carregar COPY.sav [--modo original|rust] [--ate YYYY-MM-DD]
```

- The tool changes only the field, writing the new name and a zero. For type 3, the change is in place. For type 4, it re-encodes the item as valid RLE (b ≤ 0x80 literal, b > 0x80 becomes `0x81 b`) and shifts the following positions in the index.
- At the end, it checks that every decompressed item of the output equals the input's, except the changed item, which must come out as expected.
- Type 4 was checked only by that check, on a copy of `TesteTelasComprimido.sav`. The game did not load any rewritten type 4 save.
- `--carregar` uses the load from `prova_save.py` (bench: laboratory lock, `game.cfg` and `*.tmp` restored) plus the box watcher.

## Where the language is stored

Found from the disassembly alone, plus a read-only look at `FeriasTesteSave.sav` (MD5 `33fda465193b5e8e6d7ace93ecd44559`, unchanged before and after). The game was not opened. Lucas kept the key-name block for every language, so the language-0 case was not loaded in the game.

- **In memory:** the dword `[0x9b5984]` (`.data`). Loading resets it to −1 at 0x52851c before reading the save. The game's language switch (0x5339c0) writes the new language there at 0x535806, and the getter is 0x535920. **[observed in the disassembly]**
- **In the file:** it is the **last 4 bytes of the `index.dat` item** (decompressed). The writer (0x531400..0x5314b7) writes these in order after the 0x16 records of 0x43 bytes: a flag byte (=1), 0x1cc bytes, the u32 count `[0xae235c]` (−1 if the list `[0xae23fc]` does not exist), the list (count × 4 bytes: save row → `eng.lng` row) and the u32 language. The reader (0x52881a..0x528974) follows the same order. **[observed in the disassembly]**
- **In `FeriasTesteSave.sav`:** `index.dat` is 30,067 bytes, the tail starts at +0x5ca, the count is 7,029 and the language, at **+0x756f** (the last 4 bytes), is **9** (POR). The `eng.lng` item is in the save. **[observed in the file]**
- **Flag 0:** if the flag byte is 0 (a save without the tail), loading sets `[0x9b5984] = 0` (0x528a30) and reads nothing more. **[observed in the disassembly]**
- **Language 0 in a save:** **[inference]** it is 0 when the career was created in English (`LANGUAGE=ENG_UK` in the auto setup, or the game running in English) and nobody switched the language afterwards. In that case the key is matched directly against the +4 name in `club.dat`/`nation.dat` (see "Why") and `eng.lng` is not opened. To find a save's language, read the last u32 of `index.dat`. Having an `eng.lng` item is only a hint, because the game opens that file only when the language is ≠ 0.
