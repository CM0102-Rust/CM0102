# Batch 210 — the finable media story, the drawn injury type and the news category; the repeated national squad entries, the selected player slot and the former international pundit; the round rules on the cup match, the match tie and the tour nation

**Routines:**
- `historia_punivel_do_jogador()` = 0x7357c0 (media.cpp, thiscall): the kind of the media story about the player when it is a misbehaviour kind, else 0;
- `escolher_tipo_de_lesao()` = 0x62d140 (injury.cpp, thiscall): draws the injury type for the body part and severity, keeping the current injury or preferring the recurring one;
- `categoria_da_noticia()` = 0x785670 (news.cpp, thiscall): the news category by the thousands range of its type;
- `apagar_repetidos_das_convocacoes()` = 0x76c060 (national_teams.cpp, stdcall): clears repeated squad entries from the senior and B national sides of a nation;
- `vaga_do_escalado()` = 0x8972c0 (tactics.cpp, thiscall): the club selection slot (0..19) holding the player, or 0xff;
- `escolher_comentarista()` = 0x72b810 (media.cpp, thiscall): draws a former international (37+, 50+ caps) as pundit, weighted by nation reputation and caps;
- `regras_da_rodada_no_jogo()` = 0x521d60 (Cup.cpp, thiscall): copies the round rules onto the match and, with a decision on a draw, marks who goes through by both sides' levels;
- `chave_do_jogo_na_copa()` = 0x519c90 (Cup.cpp, thiscall): the round tie that holds the match's home club (-1 if none);
- `pais_da_excursao()` = 0x5c4680 (friendly.cpp, thiscall): draws the nation for a club tour, nearer the lower the reputation;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `m_dobro` escaped: doubling the weight (or adding 1) only decides when candidates from the subject nation compete with others; in the career the other-nation ones hardly ever pass the cut (reputation above 6500 + draw and record ≥ 6000), and among same-nation ones 2a and a + 1 keep the same order. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 209's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 210`
