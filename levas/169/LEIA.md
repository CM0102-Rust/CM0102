# 169ª leva — o fechamento do motor (3): a fila da narração

**Rotinas:**
- `evento_na_frente()` = 0x6d5d00 (match_events.cpp, thiscall): põe um evento na frente da fila da narração (recua a posição no anel de 12, tira o repetido) e enfileira pela 0x6d6f70;
- `construir_fila_da_narracao()` = 0x6d5bf0 (match_events.cpp, thiscall): o construtor da fila da narração: zera a fila, lê os eventos e, se pedido, toca o loop da torcida;
- `liberar_tabelas_de_eventos()` = 0x6d5b50 (match_events.cpp, cdecl): libera as tabelas de eventos carregadas e os textos de cada registro;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (4):** 2 acusadas. `c_tempo` escapou: mutação equivalente: o +0x2db0 inicial (125) só é comparado com 20 e 101 e as telas da partida o regravam. `l_fim` escapou: mutação equivalente: a 0x6d9e50, que chama a liberação, regrava 0xa1599c (o idioma dos eventos) logo depois. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 168ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 169`
