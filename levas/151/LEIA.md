# 151ª leva — o andamento do dia de jogos (0x6b5a70, 0x6b5ce0, 0x6b5f00)

**Rotinas:**
- `iniciar_proxima_partida()` = 0x6b5a70 (match_day.cpp): começa a próxima partida do dia (cria e prepara o motor); na 1ª volta sem nenhuma, passa à 2ª.
- `andar_partida_do_dia()` = 0x6b5ce0 (match_day.cpp): um passo do motor da partida atual, ou até o fim; ao terminar, cria o resumo, registra e libera o motor.
- `fechar_partida_do_dia()` = 0x6b5f00 (match_day.cpp): copia o resultado para a partida, registra na competição e roda o pós-jogo (jogadores, eventos, contratos, suspensões, estados, história, clubes).

**Como foi comparada:** sem comparação própria (criam e liberam motores e resumos), pela carreira (Rust × original), como as das levas 148 a 150.

**Contadores de ramo desde o início** (ideia do Lucas, a partir desta leva): cada ramo mutado tem um `ramo("...")`; a rodada base mostra quais rodam. Aqui mostrou que o passo a passo e a rodada do humano não rodam na carreira automática.

**Resultado** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutações (14): 6 acusadas e 8 escaparam. Duas eram fracas e, refeitas mais fortes, foram acusadas. Uma é equivalente (teste redundante). As outras 5 viraram 4 lacunas em `docs/PENDENCIAS.md`.
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.

**Refazer:** sobre o commit da 150ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 151 -`
