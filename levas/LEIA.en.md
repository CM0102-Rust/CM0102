# Engine batches: how each one was made and tested

Each batch ports match-engine routines to Rust (`rust/src/motor.rs`) and only goes in after passing the test chain. From batch 143 (2026-10-16) on, each one has a folder here with what was used and what came out, for anyone who wants to redo, check or continue the work. Earlier batches are described in `docs/ROTINAS.md`, with their mutations in `engenharia-reversa/mutacoes/`.

## What is in `levas/NNN/`

| File | What it is |
|---|---|
| `LEIA.md` / `LEIA.en.md` | the batch on one page: routines, decisions, what went wrong along the way and the result |
| `codigo.rs` | the Rust code the batch added to `motor.rs` (copies, Rust-mode dispatch and comparators) |
| `integrar.py` | how `codigo.rs` went into `motor.rs`: the `PROLOGOS` table, function lists, recording ports removed, `ZERADAS` |
| `docs.py` | generates the batch's documentation (`docs/ROTINAS`, `PROJETO`, `wiki`, glossary) with `engenharia-reversa/gen_docs.py` |
| `cadeia.txt` | the full output of the test chain (below) |

The mutations live in `engenharia-reversa/mutacoes/levaNNN.py`, alongside the older batches'.

## The cycle of a batch

1. **Choose** the routines: `python engenharia-reversa/fronteiras.py --candidatas` shows the scoreboard and the engine routines whose calls are already ported. The published scoreboard is `docs/PLACAR.en.md` (updated at each batch).
2. **Read** the original: `python engenharia-reversa/pacote.py 0xADDR` gathers in one folder (`pacotes/`, outside git) the disassembly, the pseudocode, callers and callees (with the Rust name of each routine already ported), the floating-point constants with their values, the jump tables, the fields it uses with the routines across the whole exe that read or write them (from the OllyDbg text brought from the forum, `referencias/cm0102_3.9.68_text.zip`) and where the routine already appears in the notes. Underneath: `descompilar.py` (Ghidra pseudocode) for the structure and `desmonta.py` (assembly) for the floating-point arithmetic and the details the pseudocode hides.
3. **Write** the Rust copy with the calls that touch things outside the engine injected (`chamar`), the Rust-mode dispatch and the comparator (side by side, or through the tape when there are calls with effects).
4. **Compare** in a game to 2001-08-20 (`simular.py --modo comparar`): zero divergences.
5. **Mutations**: each one changes a detail of the copy; the career must change (or the comparison must flag it). One that is not flagged is a gap (goes to `docs/PENDENCIAS.md`) or an equivalent mutation (redone).
6. **Chain**: `bash engenharia-reversa/cadeia.sh NNN 2001-10-01` runs the mutations, a real game comparing to 2002-01-01 and the Rust ×2 and original careers to 2002-06-01. All three careers must give the reference fingerprint.
7. **Document** (`levas/NNN/docs.py`) and publish.

Details, pitfalls and the reason for each step: `docs/COMO_CONTINUAR.md`. Nothing from the game goes here: the chain only records numbers, branch names and fingerprints.
