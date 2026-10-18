# Batch 162 — the tactics table slots

**Routines:**
- `gravar_vaga_tatica()` = 0x896a90 (tactics.cpp, thiscall): stores the person in slot 0..19 of the club row (0x8982b0) and clears the club's pending instructions;
- `pessoa_da_vaga_tatica()` = 0x8968f0 (tactics.cpp, thiscall): the slot's person; without a record or from another nation in a national team, the slot becomes −1;
- `gravar_formacao_tatica()` = 0x8967d0 (tactics.cpp, thiscall): stores the formation in the club row;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 2 caught. `g_pendente` escaped: pending instructions (+0x5f0) only exist with a human manager. `p_ficha` escaped: in the automatic career the tactics table slots are empty (−1): not even the inverted condition (`p_ficha_b`, `mutacoes/leva162b.py`) is caught; the part with a person only runs on the human manager's tactics. `p_nacao` escaped: likewise (the inverted `p_nacao_b` also escapes). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 161's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 162`
