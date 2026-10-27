# Table dump right after -load (CMRTE regens)

CMRTE request (2026-10-26): prove that the GPF2 Save built **from memory** gives the same regen list as the one built
**from the file** (canonical output SHA `514961fe…`). This document describes the raw memory dump made for that.

## What was dumped

In `despejo_regens\` (outside git): one `.bin` per table, memory as it is (pointers already resolved), in the game's
vector order, and `despejo.json` (addresses, counts, date, header, MD5s). Non-players not included.

| File | Pointer (where the game reads it) | Base | Count (at) | Record | Bytes | MD5 |
|---|---|---|---|---|---|---|
| `pessoas.bin` (people) | `[0xae23bc]` (0xae234c+0x70) | 0x1b4183b0 | 62,487 `[0xae2364]` | 0x6e | 6,873,570 | fa9be884cd181e414d94452b22d243a7 |
| `jogadores.bin` (players) | `[0xae23c4]` | 0x1c17c3c0 | 42,553 `[0xae236c]` | 0x46 | 2,978,710 | c360b48aa0ab1ecf61732563c9b85d02 |
| `first_names.bin` | `[0xae23f0]` | 0x1a1c7834 | 51,901 `[0xae2398]` | 0x3c | 3,114,060 | 46cb637ece22661aeb5ab05b93914f43 |
| `second_names.bin` | `[0xae23f4]` | 0x1a4bfc80 | 98,912 `[0xae239c]` | 0x3c | 5,934,720 | b4713a3be656cecd5f87f4cf3eb22034 |
| `common_names.bin` | `[0xae23f8]` | 0x1aa68b00 | 4,786 `[0xae23a0]` | 0x3c | 287,160 | 8cebd1b903efb6f26a09cf045f0957bf |
| `nacoes.bin` (nations) | `[0xae23a8]` | 0x1ab27e42 | 231 `[0xae2350]` | 0x122 | 66,990 | 036d37ee71ca58ba06f1e22aacaf6451 |
| `clubes.bin` (clubs) | `[0xae23b4]` | 0x1abe63a0 | 7,394 `[0xae235c]` (clubs + national teams) | 0x245 | 4,295,914 | b72015e6e5dea2fdf733eab6f084cdf3 |

Addresses checked against `docs/MAPA_DATABASE.md` (the 19-table list) and `TABELAS_DO_BANCO_DBN2` in `motor.rs`. Sizes
match the save items: `staff.dat`, `player.dat`, the three name tables and `nation.dat` have exactly the same byte
counts; the clubs are `club.dat` (6,932) + `nat_club.dat` (462), contiguous.

Also in the JSON: the 8 bytes at `0xae2c90` (`4901ec0701000000` = day 329 of 2028) and the 0x80 bytes at `0xae234c`
(the table counts and pointers).

## Source

- **Save:** `clz_teste\Valencia.sav` (only read and copied), MD5 `67e35978592908917184a0fc7e9f09d3`, 278,992,303 bytes,
  untouched (checked afterwards). It is **from the CLZ database**: 231 nations (the factory one has 213), same as CLZ's
  `auto setup game.sav`. So it was loaded in the CLZ copy.
- **Exe:** `%TEMP%\clz_compat4\cm0102.exe` (3.9.68 with the CLZ 25-26 patch and NickDLL), MD5
  `b9630dc08e96accf7683e3b517208424`, run as a renamed copy (`cm0102_despejo.exe`, deleted at the end).

## The exact moment

At the **entry of 0x8387e0** (Setup.cpp, restore humans). In the `-load` handler (0x81bd70) the order is:
`load_game_data` 0x82a9c0 (0x81bee7) → `Config::apply_and_save` 0x5d8480 → human check 0x838d10 → `call 0x8387e0`
(0x81c03d). No day has passed (no `CM_FERIAS`, nothing runs before the dump); the process is terminated right there.
Caveat: 0x5d8480 reloads the translated names (clubs, nations) if the `game.cfg` language differs from the names'
language; the `.bin` files hold what the game has in memory at that point.

## How to reproduce

1. Diagnostic DLL: `copias\regens\rust` (HEAD's `rust\` + `src\despejar.rs` + the hook in `simulacao.rs`; outside git).
   `cargo build --release` in that folder.
2. `python copias\regens\despejar.py`: inside the lab lock (the `simular.py` bench pointed at the CLZ copy; the copy's
   `game.cfg`, `*.tmp`, `ddraw.dll`, `hall_of_fame.bin` and `ddraw.log` restored), copies the save to
   `%TEMP%\cm0102_regens`, runs `cm0102_despejo.exe -windowed -seed 12345 -load <copy>` with `CM_PARAR_EM=2099-12-31`
   (simulation mode, needed for the hook), `CM_ORIGINAL=1`, `CM_JANELA=960x540`, `CM_OCULTO=1` and
   `CM_DESPEJAR=D:\Sistemas\CM0102\despejo_regens`, no clicks (~5 s), and adds the MD5s of the exe, the save and the
   `.bin` files to the JSON. The run's `ddraw.log` is kept alongside.
