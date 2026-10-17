# 153ª leva — a montagem do dia de jogos (0x6b3c10, 0x714d30, 0x6d8c70)

**Rotinas:**
- `montar_o_dia()` = 0x6b3c10 (match_day.cpp): monta o dia de jogos (partidas pendentes de cada lista, agrupadas por lista e competição, a lista global e o interesse do dia).
- `lado_do_jogador()` = 0x714d30 (match_pl.cpp): o lado do jogador (+0x27). Com ela, `match_pl.cpp` fica sem rotinas reais faltando (as outras 2 do placar são fronteiras falsas no meio da 0x7109f0).
- `sons_terminados()` = 0x6d8c70 (match_events.cpp): os sons dos eventos marcados já tocaram?

**Como foi comparada:** sem comparação própria, pela carreira (Rust × original), com contadores de ramo.

**Teste junto com outro projeto:** a trava do `mutacoes.py` passou a olhar só os `cm0102.exe` desta pasta (outro projeto da máquina, o CMRTE, tem um exe de teste com o mesmo nome). A cadeia rodou com ele aberto e os tempos ficaram iguais aos das levas anteriores (mutações 137–161 s, jogo real 1755 s, carreiras 140–160 s).

**Resultado** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutações (8): 4 acusadas e 4 escaparam; pelos contadores de ramo, 3 lacunas em `docs/PENDENCIAS.md` (a 1ª volta do dia, as partidas já marcadas -2, os sons dos eventos).
- Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3).
- Carreiras Rust ×2 e original até 2002-06-01: as três iguais (034ff3eb3e55dfd3).

**Refazer:** sobre o commit da 152ª (e o do ponto de extensão):
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 153 -`
