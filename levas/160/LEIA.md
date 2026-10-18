# 160ª leva — estatísticas do jogador e zagueiros na formação

**Rotinas:**
- `estatistica_do_jogador()` = 0x7c2150 (player_stats.cpp, stdcall): a estatística `qual` 1..22 do registro (bytes, porcentagens ×100.0f, médias por jogo, as duas notas completadas com menos de 20 jogos ×0.05f), em x87 como a original;
- `media_das_notas()` = 0x7c1950 (player_stats.cpp, thiscall): a média das cinco notas de 4 bits do registro (zero conta 27), em 16 bits dividida por 4;
- `zagueiros_sem_conflito()` = 0x5b8dc0 (formation.cpp, thiscall): 0 se alguma vaga com lado aberto é de zagueiro ou volante;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 7 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 159ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 160`
