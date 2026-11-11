# The transfer market: map and Rust items (11/11/2026)

The CM 01/02 Rust port (`D:\Sistemas\CM0102`; read `CLAUDE.md`, the header of `rust/src/motor.rs` and
`docs/COMO_ESCREVER_UM_ITEM.en.md` first) is asked by the sister project (the community DLL "Rust Tweaks", release 1.5)
for a map of the transfer market, loans included, to rebalance it: make the market easier for some groups of players,
harder for others, without the selling side accepting anything; the chairman's and the staff's attributes as fine
tuning. You do two things for your part of the market:

1. **Map it.** For each routine: the rule in words, with the order of the conditions; every field it reads or writes of
   person (staff), contract, club, chairman, directors, manager and scouts, with the offsets and the names in
   `rust/src/motor/fields.rs` / `records.rs`; the constants with address and value (`.rdata` doubles: read them from the
   exe, `jogo/cm0102.exe`, VA − 0x400000 = file offset); who calls it and when (daily, monthly, window), from the call
   sites (`engenharia-reversa/refs.py jogo/cm0102.exe ADDR`) and the community map (`docs/mapa_funcoes.csv`,
   `referencias/cm0102-function-map`); defects of the original; **extension points** (where a small hook changes the
   policy without rewriting a whole routine). Mark each claim **[proved]** (disassembly or a test) or **[inferred]**.
   Tools: `python engenharia-reversa/desmonta.py jogo/cm0102.exe START END`, the pseudo-C of
   `engenharia-reversa/descompilar.py`, `pacotes/` (reading packages), `docs/FORMATOS.en.md`, `docs/CAMPOS.md`,
   `docs/MAPA_*.md`. Routines already in Rust (`rust/src/motor.rs`, the `routines!` table) are read from the Rust code
   and its header.
2. **Port what is not in Rust yet** as queue items (`levas/fila/<prefix>_<n>/`, up to 3 routines each, in the current
   style exactly as the guide says: English names and headers with the `Original:` line, typed records, `exe::`,
   `covered!`, the comparer, `DOCS` in Portuguese and English, mutations that change who signs, who goes, how much is
   paid, not just when). Check `rust/src/motor.rs` and `levas/fila/*` first: never port a routine that is already
   there. Prove each item in your own copy as the guide's "Proving" section says (comparison 0 divergences, Rust ×2
   equal with the detector, mutations caught), through `simular.py`, which takes the shared laboratory lock: never in
   `copias/bancada*`, never two games in one folder, and stop if the memory guard refuses. An item that does not pass
   gets `FORA.md`. If proving is not possible in your time, deliver the item with `PRONTO.txt` saying what was proved
   and what was not.

Your output:
- your section of the map in `levas/reorganizacao/mercado/secao_<letter>.md` (Portuguese) and `secao_<letter>.en.md`
  (English), with the routine table (address, name, in Rust or not, item), the rules, fields, constants, callers,
  defects, extension points;
- your queue items, with your prefix, not added to `levas/fila/ordem.txt` (the main session orders them);
- a short report in Portuguese: what you mapped, the items and their proof, the defects found, the doubts.

Do not commit, do not touch `rust/src` of the main copy, `levas/<N>`, `ordem.txt` or the benches. Other readers map
the other parts at the same time; if you need a routine of theirs, read it, do not port it.
