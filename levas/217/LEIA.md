# 217ª leva — o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor

**Rotinas:**
- `montar_estatisticas_da_partida()` = 0x729000 (match_stats.cpp, thiscall): confere o CD do CM01/02 e monta as estatísticas: carrega do save ou cria o índice, os baldes e o arquivo temporário zerado;
- `desmontar_estatisticas_da_partida()` = 0x7293c0 (match_stats.cpp, thiscall): libera o índice dos registros e zera o ponteiro e a quantidade;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 216ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 217`

**Campos tocados:**
- Match_Stats +0: índice dos registros (n × 12 bytes); ponteiro; grava (montar_estatisticas_da_partida); lê, libera e grava 0 (desmontar_estatisticas_da_partida)
- Match_Stats +4: capacidade do índice (n); valor; grava (montar_estatisticas_da_partida); grava 0 (desmontar_estatisticas_da_partida)
- Match_Stats +8..+0x14: cursores dos quatro baldes (k × n / 4); valor; grava (montar_estatisticas_da_partida)
- Match_Stats +0x18: sem nome (zerado depois de gravar o arquivo temporário); valor; grava 0 (montar_estatisticas_da_partida)
- Match_Stats +0x1c: quantidade de registros (n); valor; grava (montar_estatisticas_da_partida)
- match_stats_matches.tmp (CM3_TEMP): n registros de estatística de 0xd7a bytes zerados; arquivo; grava (montar_estatisticas_da_partida)
- [0x729089]/[0x7290cb] bytes do código da conferência do CD (nop/xor no exe de trabalho); valor; lê (montar_estatisticas_da_partida)

**Interessa a:** SGE
