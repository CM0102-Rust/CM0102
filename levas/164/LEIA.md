# 164ª leva — o calendário das ligas

**Rotinas:**
- `girar_rodada()` = 0x68a030 (league.cpp, cdecl): a próxima posição no rodízio do calendário (liga especial, saltos de 3 por sorteio, voltas em n);
- `registrar_jogo_da_rodada()` = 0x68a1c0 (league.cpp, cdecl): grava o jogo da rodada (três bytes e um dword);
- `marcar_ano_no_calendario()` = 0x5af180 (fix_man.cpp, cdecl): marca o ano do jogo nas entradas dos dois clubes (ano × 2 + bit, regra de 2001);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 5 acusadas. `g_meio` escapou: a carreira automática só joga ligas inglesas, sem a marca 0x80 do rodízio; nem refeita mais forte (`g_meio_b`, `mutacoes/leva164b.py`) é acusada. `g_volta` escapou: a volta no fim do rodízio não acontece na carreira; nem refeita mais forte (`g_volta_b`) é acusada. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 163ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 164`
