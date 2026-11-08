*English version of [EXTENSAO.md](EXTENSAO.md).*

# Extension point: an external DLL that reads the engine

Created on 2026-10-17 for Lucas's button football game (a 2D engine in Rust that draws the matches), as separate work, on the `jogo-de-botao` branch.

## The contract (version 1)

- The `ddraw.dll` (the core, `rust/src/extension.rs`) loads, if present, `botao.dll` from the game folder, at the first match instant.
- The DLL exports, in C:
  - `extern "C" fn cm_instante(motor: *const u8)`: called at the start of each instant of the engine loop (0x6b98c0, `laco_do_motor` in `motor.rs`);
  - `extern "C" fn cm_versao() -> u32` (optional): the contract version it expects; if it differs from `extensao::VERSAO`, the DLL is not used.
- Without the DLL, nothing changes: the reference career is the same (proven with the Rust career to 2002-06-01).

## Rules

- **Read only.** The extension does not write to the engine, does not call game routines and does not draw random numbers: the career must be identical with or without it.
- What is in the engine (player positions, ball, clock, score, commentary) is in `motor.rs` and `ROTINAS.md`; the extension reads the fields by the documented offsets.
- A contract change (another call, another argument) raises `VERSAO` on `main` and on the extension side together.

## How to work without disturbing `main`

- Branch `jogo-de-botao`, in its own folder (worktree), never `git checkout` in the main folder `D:\Sistemas\CM0102`, where the batches run.
- The extension lives in its own crate (outside `rust/src`), built as `botao.dll`.
- Its own game folder for testing (a copy of `jogo/`), because `jogo/ddraw.dll` is swapped on every batch.
- Heavy tests respect the lab lock (`engenharia-reversa/laboratorio.py`); the machine is shared.
