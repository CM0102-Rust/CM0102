*English version of [README.md](README.md). Both are kept in sync; the Portuguese file is the original.*

# Championship Manager 01/02: reconstruction

Championship Manager: Season 01/02 (Sports Interactive / Eidos, 2001), version 3.9.68, released for free by Eidos and SI.
This repository keeps it playable on Windows 11 and prepares the rewrite of the game in Rust, routine by routine, with the interface identical to the original.

| Where | What it is | In git? |
|---|---|---|
| `JOGAR.bat` | Opens the game at native 1920×1080, in a resizable window, without the CD and without changing the Windows resolution. | yes |
| `rust/` | Rust core, compiled as `ddraw.dll` (our own video layer; entry point for the hooks). | yes |
| `save/` | Save-only DLL (saves above 2 GB, up to 4 GB), for modified exes (NickPatcher, CLZ): [`docs/SAVE_2GB.en.md`](docs/SAVE_2GB.en.md). | yes |
| `jogo/` | Playable installation: the originals `cm0102.exe` / `cm0102_GDI.exe` and the patched copies `*-portatil.exe`. | no |
| `original/` | Untouched downloads (ISO, zips, 3.9.68 patch) and a backup of the two 3.9.68 exes. | no |
| `referencias/` | Third-party projects consulted (e.g. nckstwrt's `CM0102Patcher`). | no |
| `engenharia-reversa/` | Analysis tools (Python) and the generator of the portable exes. | yes |
| `docs/` | Technical documentation; start with [`DESCOMPILACAO.md`](docs/DESCOMPILACAO.en.md). To continue the rewrite: [`COMO_CONTINUAR.md`](docs/COMO_CONTINUAR.en.md). English meaning of the code's names: [`GLOSSARIO.en.md`](docs/GLOSSARIO.en.md). Record fields: [`CAMPOS.en.md`](docs/CAMPOS.en.md). | yes |
| `levas/` | One folder per batch of the rewrite: the code, the integration and the test results. | yes |
| `wiki/` | Project state, decisions and history. | yes |
| `CLAUDE.md` | Instructions for continuing the work in Claude Code. | yes |

**State:** 215 batches, 546 routines in Rust, each one compared with the original (the match engine without the screens is 100% in Rust; saves go beyond 2 GB, up to 4 GB); how much is left, module by module: [`docs/PLACAR.en.md`](docs/PLACAR.en.md).

The game files do not go into git: the game is free to download, but the code and the brand still belong to SI and Eidos.

## Building from scratch

1. Install the game from `original/CM0102.iso` and apply the 3.9.68 patch (`original/Championship-Manager-Season-0102_Patch_Win_EN_Patch-v3968/242.exe`). Put the installation in `jogo/`.
2. Generate the portable exes (Python 3 with `pefile` and `capstone`):
   ```
   cd engenharia-reversa
   python patch_portatil.py ..\jogo\cm0102.exe ..\jogo\cm0102-portatil.exe
   python patch_portatil.py ..\jogo\cm0102_GDI.exe ..\jogo\cm0102_GDI-portatil.exe
   ```
   And the 1920×1080 version (it also converts the background images; the original pictures are kept in `jogo\Pictures-800x600`):
   ```
   python resolucao.py ..\jogo\cm0102-portatil.exe ..\jogo\cm0102-1080.exe 1920 1080 ..\jogo
   ```
3. Compile the Rust core (target `i686-pc-windows-msvc`, already configured in `rust/.cargo/config.toml`) and copy the DLL to the game:
   ```
   cd rust
   cargo build --release
   copy target\i686-pc-windows-msvc\release\ddraw.dll ..\jogo\
   ```
4. Double-click `JOGAR.bat`.

Diagnostics: with `set CM_ESPIAO=1` before opening the game, the DLL passes everything through to the Windows DirectDraw and logs each call in `jogo\ddraw.log`.

What the portable exes fix, and why: [`docs/DESCOMPILACAO.md`](docs/DESCOMPILACAO.en.md), section 3.
