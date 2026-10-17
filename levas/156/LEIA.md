# 156ª leva — o texto dos eventos e o começo da partida em rede (0x6d8d10, 0x6b5260)

**Rotinas:**
- `texto_do_evento()` = 0x6d8d10 (match_events.cpp): monta a linha de narração do evento, trocando as marcas do `events_*.cfg` (jogador, clube, competição, fase, número, árbitro) com prefixo, sufixo e maiúsculas.
- `confirmar_partida_em_rede()` = 0x6b5260 (match_day.cpp): cada humano confirma a partida em rede; com todos, o motor é criado e preparado para ver.

**Como foi comparada:** o texto do evento ganhou comparação de verdade (só escreve nos buffers de texto): no `JOGAR_COMPARANDO`, cada linha de narração é montada pelas duas versões e conferida. A confirmação em rede, pela carreira.

**Resultado** (`cadeia.txt`):
- Mutações (3): as 3 escaparam; as duas rotinas só rodam com a tela da partida (contadores em zero), lacuna em `docs/PENDENCIAS.md`. O teste é no jogo do Lucas.
- Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3).
- Carreiras Rust ×2 e original até 2002-06-01: as três iguais (034ff3eb3e55dfd3).

**Refazer:** sobre o commit da 155ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 156 -`
