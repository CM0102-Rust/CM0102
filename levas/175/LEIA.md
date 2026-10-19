# 175ª leva — a formação da tabela tática, o peso das competições do jogador e o dado da notícia

**Rotinas:**
- `registro_da_formacao()` = 0x896570 (tactics.cpp, thiscall): o registro da formação (0x18e3 bytes) nas duas tabelas;
- `peso_da_competicao()` = 0x7c6930 (player_stats.cpp, stdcall): o peso da competição pelas duas competições do clube (500/1000/300/100);
- `gravar_dado_da_noticia()` = 0x785790 (news.cpp, thiscall): grava o dado 0..49 da notícia;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (6):** 5 acusadas. `f_limite` escapou: o limite só muda para as formações além das padrão pedidas com o humano (as formações próprias de um técnico humano), que a carreira automática não usa. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 174ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 175`
