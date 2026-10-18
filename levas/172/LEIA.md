# 172ª leva — o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista

**Rotinas:**
- `proximo_evento()` = 0x6d78b0 (match_events.cpp, thiscall): o próximo evento da narração a mostrar, parte por parte do texto (separadas por |), avançando a leitura da fila;
- `nome_do_jogador_na_narracao()` = 0x6db2d0 (match_events.cpp, cdecl): o nome do jogador na narração: a forma curta, ou a longa quando outro jogador dos dois clubes tem o mesmo nome curto;
- `habilidade_vista()` = 0x6e86d0 (match_man.cpp, cdecl): a habilidade atual ou potencial do jogador como um observador a vê (reputação, ruído e conhecimento do observador);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (6):** 3 acusadas. `p_parte` escapou: a 0x6d78b0 só roda com a narração na tela, que a carreira automática não mostra (vai para a partida assistida do Lucas). `p_anel` escapou: a 0x6d78b0 só roda com a narração na tela, que a carreira automática não mostra. `n_longa` escapou: a 0x6db2d0 só roda com a narração na tela, que a carreira automática não mostra. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 171ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 172`
