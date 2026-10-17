# 154ª leva — os eventos gravados e o passo em rede (0x6d98b0, 0x6d9c40, 0x6b55c0)

**Rotinas:**
- `gravar_eventos()` = 0x6d98b0 (match_events.cpp): grava os eventos da partida nos arquivos "events1/2.tmp" de CM3_TEMP (humano ou computador), recriando o mais antigo quando os dois enchem.
- `ler_eventos()` = 0x6d9c40 (match_events.cpp): lê os eventos de volta (para rever os lances).
- `passo_em_rede()` = 0x6b55c0 (match_day.cpp): um passo da partida do humano em rede, pelo relógio e pelo placar dos humanos.

**Como foi comparada:** sem comparação própria, pela carreira (Rust × original), com contadores de ramo.

**Resultado** (`cadeia.txt`):
- Mutações (6): as 6 escaparam. A gravação roda na carreira, mas os arquivos não entram nas impressões; a leitura e o passo em rede não rodam. Duas lacunas em `docs/PENDENCIAS.md`; o teste é no jogo (rever os lances de uma partida assistida).
- Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3).
- Carreiras Rust ×2 e original até 2002-06-01: as três iguais (034ff3eb3e55dfd3).

**Refazer:** sobre o commit da 153ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 154 -`
