# Closing the market map: brief (11/11/2026)

`docs/MAPA_MERCADO.en.md` (from `levas/reorganizacao/mercado/secao_*.en.md`) still has parts marked **[inferred]** or
"only mapped". Lucas wants every one of them disassembled and proved, so the Rust Tweaks design rests on proved facts.
Read first `levas/reorganizacao/mercado/BRIEF.en.md` (the first brief: map and port), `docs/COMO_ESCREVER_UM_ITEM.en.md`
and the relevant section of the map.

For your routines:
1. **Prove the rules from the disassembly** (`python engenharia-reversa/desmonta.py jogo/cm0102.exe START END`; never
   rely on the pseudo-C alone: it hides `_ftol` arguments and x87 order). Every step: the instruction addresses, the
   fields with offsets, the constants with address and value (read the doubles from the exe), the callers.
2. **Port** the routine as a queue item (`levas/fila/<prefix>_<n>/`, current style, comparer, DOCS pt+en, mutations that
   change the result), byte-faithful. Check `rust/src/motor.rs` and `levas/fila/*` first.
3. **Compile-check** only: apply the item to a scratch copy of `rust/` and `cm0102-base/` as `fila.py` does (one `cargo
   build --release` at a time; delete the copy and its `target` after). **Do not run the game**: the main session
   proves every item in its benches (comparison, Rust ×2, mutations). Write in `PRONTO.txt` what was compiled.
4. Write `levas/reorganizacao/mercado/fechamento_<prefix>.md` and `.en.md`: for each claim of the map about your
   routines, the old text, the new text and the proof (instruction addresses), so the main session can replace
   [inferred] by [proved] in the map. Do not edit `docs/MAPA_MERCADO*.md` yourself (other readers work at the same time).

No commits; do not touch the main `rust/src`, `levas/<N>`, `ordem.txt` or the benches. Report in Portuguese: what is
now proved, what changed from the map, the item, any defect found.
