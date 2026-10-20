# 192ª leva — a entrada da aposentadoria, as competições do amistoso e a vaga da notícia

**Rotinas:**
- `entrada_da_aposentadoria()` = 0x7a7ed0 (player_regen.cpp, thiscall): a entrada da pessoa na lista das aposentadorias (por lesão ou já anunciada);
- `registrar_competicoes_do_amistoso()` = 0x5ca030 (friendly.cpp, thiscall): grava os objetos dos amistosos e dos torneios na tabela das competições;
- `vaga_da_noticia()` = 0x785830 (news.cpp, thiscall): o valor de uma das 50 vagas da notícia;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `a_torneio` escapou: a vaga 0 dos torneios do amistoso nunca está ocupada na carreira. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 191ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 192`
