# 157ª leva — a narração no painel (0x6d7b00)

**Rotina:** `narrar_no_painel()` = 0x6d7b00 (match_events.cpp, ~4,4 KB): durante a partida assistida, escreve o lance no painel de narração com as cores dos clubes, quebrando ou cortando a linha pela largura, e toca o som do lance e da torcida.

**Marco:** com ela, **o motor de partidas sem as telas está 100% em Rust** (321 de 321 KB). O que falta do motor são as telas (`match_screens.cpp`), etapa própria.

**Como foi comparada:** sem comparação própria (desenha e toca som), pela carreira (Rust × original).

**Resultado** (`cadeia.txt`):
- Mutações (2): as 2 escaparam; a rotina só roda com a tela da partida (contador em zero), lacuna em `docs/PENDENCIAS.md`. O teste é assistir partidas no `JOGAR.bat`.
- Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3).
- Carreiras Rust ×2 e original até 2002-06-01: as três iguais (034ff3eb3e55dfd3).

**Refazer:** sobre o commit da 156ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 157 -`
