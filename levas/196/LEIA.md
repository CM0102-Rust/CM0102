# 196ª leva — o confronto direto da liga e os termos oferecidos na transferência

**Rotinas:**
- `ordem_do_confronto()` = 0x6834f0 (league.cpp, cdecl): a ordem da minitabela do confronto direto: pontos e os critérios de desempate da liga;
- `atualizar_confronto_direto()` = 0x6869a0 (league.cpp, thiscall): soma um jogo ao confronto direto (jogos, resultado, gols e pontos);
- `marcar_termos_oferecidos()` = 0x8c1f80 (transfer_manager.cpp, thiscall): marca que a oferta de transferência teve os termos de contrato oferecidos ao jogador;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 1 acusadas. `c_saldo` escapou: nenhuma liga da carreira desempata pelo confronto direto com o saldo (a minitabela não decide). `c_empate` escapou: os pontos por empate do confronto direto não decidem nenhuma classificação da carreira. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 195ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 196`
