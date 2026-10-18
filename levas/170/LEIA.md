# 170ª leva — o fechamento do motor (4): as ordens das partidas e dos jogadores e a variante do evento

**Rotinas:**
- `ordem_das_partidas_do_dia()` = 0x6b6510 (match_eng.cpp, cdecl): a ordem das partidas do dia (qsort): competição, campos do registro e o nome do clube;
- `ordem_de_acao()` = 0x6bd840 (match_eng.cpp, cdecl): a ordem dos jogadores na vez do motor (qsort): os marcados primeiro, depois pela força;
- `variante_do_evento()` = 0x6d5d80 (match_events.cpp, stdcall): troca o evento da narração por uma variante, por sorteio e pelo tipo do lance;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 4 acusadas. `o_campos` escapou: na liga inglesa as partidas do dia empatadas até +0x36 nunca divergem em sinal entre +0x38 e +0x34 (falta de dados; vai para a carreira ampla). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 169ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 170`
