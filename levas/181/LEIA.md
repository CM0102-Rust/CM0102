# 181ª leva — as notícias do jogador na mídia e a importância da partida de copa

**Rotinas:**
- `noticia_com_resposta_2()` = 0x735780 (media.cpp, thiscall): o jogador tem uma notícia com a resposta 2 no registro da mídia;
- `tem_noticia()` = 0x735840 (media.cpp, thiscall): o jogador tem alguma notícia no registro da mídia;
- `importancia_da_partida()` = 0x5221f0 (Cup.cpp, thiscall): a importância da partida da copa pela fase e a diferença do placar da ida;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (4):** 2 acusadas. `m_alguma` escapou: as notícias do registro da mídia consultadas na carreira nunca têm a resposta 0. `c_fase` escapou: mutação equivalente: com a fase 1 trocar por 1 não muda nada (não é lacuna do teste). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 180ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 181`
