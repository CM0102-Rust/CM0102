*English version of [VERSAO_SEM_TELA.md](VERSAO_SEM_TELA.md). Both are kept in sync; the Portuguese file is the original.*

# Headless version: what the game itself already offers

Study of 2026-10-05 (`Setup.cpp`, function 0x827110 and the game start at 0x81b890).

SI left in the executable a production line for games without an interface, used to generate the quick starts (`qstart\*.qst`) and, by all indications, for internal testing. It works in 3.9.68 with our installation.

## 1. Automatic configuration file

Text format, one key per line; spaces in the line are removed before reading and **order matters** (the line is applied when read):

| Key | Values | Effect |
|---|---|---|
| `LANGUAGE=` | `ENG_UK`, `ENG_US`, `FRA`, `DEU`, `ITA`, `ESP`, `DAN`, `NED`, `NOR`, `POR`, `SVE`, `JAP`, `KOR`, `POL`, `RUS`, `CHI_SIM`, `CHI_TRA`, `POR_BRA`, `ESP_MEX` | game language (table at 0x9eb244) |
| `DATABASE=` | `NORMAL`, `MINIMUM`, `MAXIMUM` | database size |
| `MATCH_STATS=` | `NORMAL`, `SMALL`, `LARGE` | matches stored |
| `FOREGROUND_LEAGUES=` | **3-letter codes** of the nations: `ENG`, `BRA`, `ARG`, `GER`... | playable leagues |
| `BACKGROUND_LEAGUES=` | same | background leagues |
| `ENABLE_MATCH_ENGINE=` | `YES`/`NO` | with `NO` the matches are resolved without the match engine (much faster) |
| `ENABLE_COMPUTER_BIDS=` | `YES`/`NO` | AI bids |
| `ON_HOLIDAY=` | `YES`/`NO` | manager "on holiday": the game advances on its own |

Leagues are recognized by the nation code (`+0x53` in the 0x122-byte record of `nation.dat`). The full name (`+4`) comes in the loaded language ("Inglaterra" in Portuguese) and does not work. To generate, at least one foreground league is required.

## 2. Two modes of use

- **Generate a quick start:** `quick_start.cfg` in the game folder with `AUTO_SETUP=<file or pattern>`. For each file found without a corresponding `.qst` in `qstart\`, the game creates the career and writes `qstart\<name>.qst`, then closes. Tested: England, minimum database, **~15 s**, without a single click. The `.qst` comes out uncompressed (flag 3, 67 MB; the originals have flag 4, ~20 MB).
- **Run on its own:** `auto_setup.cfg` in the game folder. The game creates the career with an SI test manager ("Andy Ryan") and, with `ON_HOLIDAY=YES`, advances the calendar on its own. Tested with `ENABLE_MATCH_ENGINE=NO`: from 2001-07-28 to 2001-09-08 in ~90 s, with the rounds being played.

The `.qst` files have the same container as the saves (`.sav`): compression flag, version, number of blocks and a directory of 268-byte entries (`continent.dat`, `officials.dat`...).

## 3. Precautions

- **Never leave `auto_setup.cfg` or `quick_start.cfg` in the game folder**: Lucas's next launch would start the automatic career.
- **Copy `game.cfg` before and restore it afterwards.** When the game exits normally, it saves the preferences with what the configuration file ordered (language, statistics). It happened in the first test and was fixed by hand: language back to 9 (POR), statistics NORMAL and the following byte back to 18.
- The game creates working `*.tmp` files in the folder; if it is force-closed, they remain (they are recreated in the next game).

## 4. Path to the headless version

1. **Already available:** automatic career + holiday + match engine off = batch simulation. What is missing is controlling when to stop and reading the results (through the saves, whose format `CM0102Patcher` already describes, or through the process memory).
2. **With our DLL:** hide the window and draw nothing (the video layer is ours), pin the seed (`-seed`) for reproducible simulations, and set from the command line the stop date and the output save.
3. **With the engine in Rust:** as the routines migrate, the simulation stops depending on the original exe.

## 5. Reproducible simulation (`simular.py`)

`engenharia-reversa/simular.py --ate AAAA-MM-DD [--semente N] [--repetir N] [--modo rust|original|comparar]` sets up the automatic career, opens the game off screen with `-seed N` and the DLL ends the process on the first call to the random draw after the requested date, writing the fingerprint (number of draws + complete generator state). Same seed ⇒ same fingerprint: it is the reference for proving rewritten routines over the whole career.

With a fixed seed the game still varied between runs. Accidental sources, all neutralized **only in simulation mode** (`CM_PARAR_EM`); the normal game does not change:

| Source | Symptom | Neutralization (`simulation.rs`, `allocator.rs`) |
|---|---|---|
| `time()` | `srand(time)` in manager_manager.cpp (0x696d4b) | fixed instant |
| `clock()` (31 calls, 13 routines) | match-day waits (match_day.cpp 0x6b55c0) depended on how many times menus and screens queried the clock | one clock per routine, +1 ms per query from it |
| Windows heap (LFH, random address) | tie-breaks by pointer order: different lineup on 2001-04-04 with the identical generator | the exe's `Heap*` (IAT) in a deterministic allocator in a fixed region (0x18000000, 1 GB) |
| Memory outside the heap | pointers to the database (`GlobalAlloc` in Database.cpp 0x529cdf) and to DLL memory (surfaces) | `GlobalAlloc`/`GlobalFree` in the same allocator; video-layer objects and pixels in their own region (0x10000000); the exe's `VirtualAlloc(NULL)` in a fixed range |
| Garbage on the stack | the game reads uninitialized local variables, inheriting whatever the previously called routine left; original, Rust and probes leave different scratch data (the career changed with the mode and even with the trace on) | every hooked routine runs on a private stack at a fixed address (`detours::isolate`); in original mode the same routines are hooked to the original itself (`PASSAGEM`) |
| The DLL's own memory | the DLL's `Vec`, `String`, `format!` (log, daily diary, comparisons) went to the process heap, whose address changes on each run; the pointers stayed on the private stack and an original routine that reads a variable with no value found them (item cqc_6: two different Rust careers, 2026-11-01) | the DLL's global allocator uses a third fixed region (0x0b000000, 64 MB; `alocador::REGIAO_DLL`), only for the game's thread and outside compare mode (which stays on the Windows heap); nothing is written on the stack, only the address becomes fixed. The hooks' trampolines and stubs come from 2 MB executable blocks (`desvios::executavel`), not from one 64 KB `VirtualAlloc(NULL)` each |
| Environment variables | the CRT copies the environment to the heap at startup; different sizes shifted everything | `simular.py` passes a fixed minimal environment; the `CM_*` ones are always passed, with `0`/`1` of the same size |
| Windows garbage on the stack | the code of the Windows functions (files, fonts, messages) writes values onto the game's stack that change on each run (process heap addresses, handles) and the game reads them as uninitialized variables: with the engine on, half of the careers changed on 2001-12-26 (training), with the generator identical up to that point; probes that made the day slower hid the problem | all IAT entries (139) go through the private stack (`isolar_importacoes`), except the exception and termination ones; the game's `PeekMessageA` empties the queue and always answers "none" |
| `*.tmp` files | the game reuses the working files left by the previous run (force-terminated) | `simular.py` deletes them before each run (and gives back the ones that were already in the folder) |

Also pinned: total/free memory reported to the game (0x7a16d0, 0x7a1710). Window: created and kept off screen (`CreateWindowExA`/`MoveWindow`/`ShowWindow` in the IAT), without flickering the screen of whoever is using the PC.

Divergence diagnosis: `CM_RASTRO=AAAA-MM-DD` writes to `jogo\rastro.log` every draw of the day (request, address of the caller, generator state), every `qsort` (comparator, hash of the input and of the output) and every clock query (the caller's address comes from the private stack's entry stub); the `ddraw.log` journal has one line per day (draws, generator, hash of the people table, clocks). Method: find the first differing day in the journal, trace that day, find the first differing line and climb up through the code.

### Audit of the imports (what comes into the game from outside)

The exe imports 144 functions. Everything it knows about the world comes in through them, through memory (addresses, garbage on the stack) or through files. Classification for the simulation:

| Input | Where | Status |
|---|---|---|
| Clock: `GetLocalTime`, `GetSystemTime`, `GetSystemTimeAsFileTime` | only in the CRT's `time()` and `clock()` | pinned |
| `GlobalMemoryStatus` | 0x7a16d0, 0x7a1710 | pinned |
| Disk space: `GetDiskFreeSpaceA` (and `…ExA` via `GetProcAddress`) | os.cpp 0x7a1820, 136 calls (menus, day, Setup) | pinned at 4096 MB (the maximum the routine returns) |
| Drives: `GetLogicalDriveStringsA`, `GetDriveTypeA` | the 33 CD checks (35 calls) | fixed list `C:\` |
| Environment, `GetCommandLineA` | CRT at startup | pinned by `simular.py` |
| Memory: `Heap*`, `GlobalAlloc`, `VirtualAlloc` | CRT, Database.cpp | fixed regions |
| Folder listing: `FindFirstFileA` + file dates | utils.cpp 0x90bf90 (files screen, configuration, menus, Setup) | **not pinned**: the contents of the game folders go into the career; a new save can change the fingerprint. Compare runs made with the folder in the same state |
| Language files `jogo\*.lng` | Database.cpp 0x5339a0 (language switch: writes `<language>.lng` into `CM3_TEMP`, which without the variable is the game folder) | **rewritten by the game itself** in some automatic careers. The content goes into the fingerprint (the reference changed from `c76ad0ff6ce3daa6` to `cd2c213e0ddda4d1` on 2026-10-10 at 13:04, when the content was replaced); the date does not (an identical rewrite at 15:01 changed nothing) |
| Paths: `GetCurrentDirectoryA`, `GetFullPathNameA`, `GetModuleFileNameA` | CRT, Setup | constant as long as the game does not change folder |
| Registry: `RegQueryValueExA` | main.cpp 0x68d370 (startup) | constant on the machine |
| Screen and fonts: `GetSystemMetrics`, `GetDeviceCaps`, `GetTextMetricsA` | game_config.cpp, display.cpp | constant on the machine (changing the Windows resolution can change the fingerprint) |
| Messages: `PeekMessageA`, `GetMessageA` | main loop | no measured effect (per-routine clock) |
| Network (`WSOCK32`), printing, `CreateProcessA`, `WinExec`, `MessageBoxA` | network game, printing, opening programs, errors | outside the simulation path |
| `GetVersionExA`, `GetSystemInfo`, `GetKeyboardLayout`, `GetACP`/`GetOEMCP` | startup, os.cpp | constant on the machine |

### Diagnostic tools

- **Coverage:** `cobertura.py <módulos>` writes `jogo\cobertura.txt` (only proven routine starts: instruction boundary in the linear sweep of `.text` and target of a real `call` or a pointer in the data; the map has misaligned chunks and a counter in the middle of an instruction crashes the game). `simular.py --cobertura` counts the calls (`inc [contador]; jmp original`, does not change the career) and summarizes by file; the complete list goes to `ddraw.log`. Engine on, 2 months: 210 of 229 engine functions exercised.
- **Exceptions:** in the simulation, every exception goes to `ddraw.log` (code, address, registers, stack), because Windows does not always record the crash of a hidden game.
- **Game boxes** (2026-11-02): every game message box (`gui_show_message_box` 0x5e8290, drawn by the game, invisible when it is hidden) becomes the line `msgbox YYYY-DDD title | text` in `ddraw.log` before it appears (`desvios::instalar_caixa`; in every mode, observation only: the career does not change). Line breaks and tabs become spaces; the text is the game's ANSI, byte by byte. The original entry (9 bytes, `sub esp,0x6c; mov ecx,[0xb67a30]`) goes to a trampoline; if another DLL has already replaced the entry with a `jmp` (CLZ's LaMOHatesCD), the DLL chains to its target; anything else, it leaves it alone and says `msgbox: caixas do jogo fora do log`. On a patched exe it comes with the deferred installation. `simular.py` reads the log every 5 s and, on seeing the line, ends the game with `caixa do jogo: title | text` instead of waiting for `--limite`.
- **Fingerprint and environment:** the fingerprint depends on the exact set of environment variables (the CRT copies them to the heap). A new variable in `simular.py` changes the fingerprints; always compare runs of the same version of `simular.py` and of the DLL.
- **Garbage detector** (`simular.py --detector`, 2026-11-01): after the runs, one more with `jogo\veneno.txt`; the DLL fills the whole private stack with the poison (`5a5a5a5a`) on each entry coming from outside (`detours::POISON`). Output: `detector de lixo: igual` or `DIFERENTE (...)`. Record only: it does not change the exit code. With the DLL memory at a fixed address, a new garbage read no longer shows up as "Rust ×2 different"; the detector is the alarm that remains. A file and not an environment variable, so as not to shift the game heap.
- **Engine on:** `--motor sim` is also deterministic and Rust = original; two months of season in ~40 s (600 million draws).
- **Memory map:** with `CM_RASTRO` on, the journal gains the hash of the game's whole memory and `jogo\memoria.log` the hash of each block (4 KB of `.data`, 64 KB of the heap) per day; on the trace day the whole `dados.bin` and `heap.bin` are dumped. Comparing good runs separates the noise (font handles, interface state, time base of the original `clock`) from the first data that diverges. Warning: these probes make each day slower and have already hidden a time-dependent divergence; the final proof is always without probes.
