# The next batches: the subjects the career runs (12/11/2026)

The CM 01/02 Rust port (`D:\Sistemas\CM0102`; read `CLAUDE.md`, the header of `rust/src/motor.rs` and
`docs/COMO_ESCREVER_UM_ITEM.en.md` first) wants to close, subject by subject, everything the automatic career runs that
is still in the original. Each reader gets a list in `levas/reorganizacao/proximas/<prefix>.txt` (address, size in
bytes, source file, name and purpose from the maxevans60 map, `referencias/cm0102-function-map`). The lists come from
the career coverage (`docs/cobertura_simulacao.txt`, ENG to 2002-06-01; `docs/cobertura_competicoes.txt`, 8 leagues to
2002-06-01): every function in them runs in the test career.

Queue order: `mke` (the rest of the market), `cqx`/`cqy`/`cqz` (competitions), `sma` (managers, discipline, injuries,
records), `nma` (news, media, the daily loop).

For each function on your list:
1. **Prove the rule from the disassembly** (`python engenharia-reversa/desmonta.py jogo/cm0102.exe START END`; the
   pseudo-C of `descompilar.py` hides `_ftol` arguments and x87 order). First check whether the function is already in
   `rust/src/motor.rs`/`rust/src/motor/` or in `levas/fila/*`: never port twice.
2. **Port** it as queue items (`levas/fila/<prefix>_<n>/`, up to 3 routines each, in the guide's current style: English
   names and headers with the `Original:` line, typed records, `exe::`, `covered!`, the comparer, `DOCS` in Portuguese
   and English, mutations that change the result, not just the timing), byte-faithful. A big routine (over ~4 KB) may be
   split into named pieces, each with its own comparer, as long as the whole routine stays hooked in a single item.
   Competition items carry `LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'` in `rotinas.py` (the others, no `LIGAS`). Group by
   source file: the goal is to close whole files and subjects.
3. **Compile only**: apply the items to your own copy of `rust/` and `cm0102-base/` in your scratchpad, as `fila.py`
   does (one `cargo build --release` at a time; delete the copy and its `target` at the end). **Do not run the game**:
   the main session proves every item in the benches (comparison, Rust ×2, mutations). Write in `PRONTO.txt` what was
   compiled and what the proof needs (leagues, date, branches only the comparison sees).

No commits; do not touch the main copy's `rust/src`, `levas/<N>`, `ordem.txt` or the benches. Other readers work at the
same time on the other prefixes: if you need a routine from another list, read it, do not port it. Short report in
Portuguese: the items, what was left out and why, defects of the original found, doubts.
