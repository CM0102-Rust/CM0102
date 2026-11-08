*English version of [COMO_ESCREVER_UM_ITEM.md](COMO_ESCREVER_UM_ITEM.md). Both are kept in sync; the Portuguese file is the original.*

# How to write a queue item

An item is a package of up to 3 ported routines, written ahead of time in `levas/fila/<prefix>_<n>/` and gathered into a batch by `fila.py` (the file format is in the header of [`engenharia-reversa/fila.py`](../engenharia-reversa/fila.py); the batch cycle, in [COMO_CONTINUAR.en.md](COMO_CONTINUAR.en.md)). This guide collects the lessons from the agents' briefs (October and November 2026), so they don't have to be rewritten every round.

## Choosing

- **Where to look:** `python engenharia-reversa/candidatos.py` (`--simulacao`, `--competicoes`), `docs/cobertura_*.txt`, `docs/MAPA_COMPETICOES.md` and `docs/PLACAR_ARQUIVOS.md`. Prefer routines that only call what is already Rust (0 or 1 missing).
- **Only what the test career exercises, with an effect up to 2002-06-01.** A routine that doesn't run (counter 0), a passing tournament (nothing read later depends on its matches) or a record nobody reads before June has no mutation that gets caught: the item stays out. One item fewer is better than an item that doesn't run.
- **No repeats:** nothing from `motor.rs` or `levas/fila/*`. Reserve by creating the folder with a minimal `rotinas.py` as soon as you choose.
- **Community map** (`referencias/cm0102-function-map`): entries marked "fragment" or "not a function" are pieces of another routine, not routines. Don't read 's NickDLL code (licence); only [.en.md](.en.md).
- **Same slot in several leagues:** putting the same slot of 3 leagues in one item is good; a shared helper with each league's parameters is fine, as long as every address has its own `Prologo` and is faithful byte by byte.

## Porting

- **Memory access:** use the functions in `rust/src/motor/mem.rs` (`read_u32`, `write_u8`...); don't create `<item>_d/_b/_w` copies in the item (since 11/07/2026).
- **Calling conventions checked at the call site** (thiscall/stdcall/cdecl, `ret N`); `engenharia-reversa/aridade.py` helps. Note them in `rotinas.py`.
- **Size of what is written:** 0x549ef0 writes 8 bytes (the date and the leap-year flag); a 4-byte local corrupts the stack. Use the size the original writes.
- **Shuffles and counts:** check the exact number of slots (one copy shuffled 9 and the original 8).
- **Order the original assumes:** the ported `init_table_row` (0x687430) assumes the rows in order; don't reverse them.
- **Field labels:** check them in [CAMPOS.en.md](CAMPOS.en.md); what isn't proven gets "[inference]".
- **DOCS:** for each routine, the rule in words (how many go up and down, dates, places, tie-break), in Portuguese and English. It is the source of the glossary and of the headers.

## Names and header (since 2026-11-08)

- Code in English: the routine with the original method name (maxevans60's map, as in `docs/NOMES.csv`), helpers with English names and no work-item prefix, constants in English UPPER_CASE.
- A bilingual header on every routine: the English summary, the line `Original: Class::method (address, file.cpp, convention)`, a `///` line and the Portuguese notes with `[pt]`. The English is the complete text (everything the `[pt]` says: conditions, offsets, defects of the original), not a summary: readers of the public code do not read Portuguese.
- Named fields on one-letter variables too: `p + staff::NATION`, not `p + 0x..`, when the variable holds a record of docs/FORMATOS.en.md for the whole function (`aplicar_tipos_variaveis.py` does it later, but the item can already come that way).
- Use what already has a name: `read_u32`/`write_u8` (`mem.rs`), `KEY_*` (`keys.rs`), `G_*` (`globals.rs`), `club::DIVISION` and `club::SIZE` (`fields.rs`), `FN_*` (`exe_fns.rs`) and `original_fn::<F>(IDX_<NAME>)`; a float constant read from the exe with its value in a comment.
- The `DOCS` in `rotinas.py` stays in Portuguese and English (it is the source of ROTINAS and the glossary).

## Comparing

- The comparer checks **only what the routine guarantees on exit**. A field that another routine rewrites later (e.g. 0x6827d0 rewrites +0xa7) gives false alarms.
- When the routine can't run twice, only the original runs and the plan is checked afterwards (the `cqt`/`cqk` model); schedules go side by side (the `cqp_11` machine: `Cqp11Op`, `cqp11_tabela`, `cqp11_comparar`).
- Asking a cup for its round table again shows the "Cup.cpp 1278" box: don't rebuild the plan during the comparison.

## Proving (in your own copy, never in the benches)

- The copy takes `rust/` and `cm0102-base/` side by side (`rust/` depends on the base through the path `../cm0102-base`).
- **Comparison** with `simular.py --modo comparar` (with `--ligas "ENG ESP ITA FRA BRA JPN AUS USA"` for competition items) up to the date the routine runs; any `DIVERGENCIA` or `EXCECAO` line blocks the item.
- **Rust careers ×2** with `--repetir 2 --detector` up to 2002-06-01: equal, and "detector de lixo: igual".
- **Mutations** through the career up to 2002-06-01, with the bid engine on (like the chain's `mutacoes.py`; use `--limite 3000`, since the 8-league one takes over 900 s):
  - change **who plays, who goes through or who gets in**, not just when;
  - a date mutation moves **7 days**: the game snaps rounds to the weekday, and 1 or 2 days land in the same place;
  - avoid mutations that hang or crash the game (they are only caught at the time limit and delay the chain);
  - a mutation on a path that doesn't run in the test leagues is no good;
  - one that escapes the career but is caught by the routine's comparer counts as caught; the others need an explanation, or the item goes out.
- Heavy tests only through `simular.py` (laboratory lock, `CM_VAGAS`); never two games in the same folder.

## Delivering

- `PRONTO.txt` with one line of proof (e.g. "comparison up to 2001-10-01 0 div; Rust x2 equal; mutations 3/3"). After it, don't touch the item again.
- An item that doesn't pass: `FORA.md` saying why (the next round reads it before reusing anything).
- Report per item: addresses, names, comparer, conventions, the rule in one line, the proof and the mutations that escaped.
- No commits; don't touch the main copy's `rust/src`, `levas/<N>`, `ordem.txt` or `copias/bancada*`. Delete the game copy and `target` when done.
