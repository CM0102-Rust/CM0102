# 238ª leva — as chaves fixas do jogo: os ids pela base, as cores e os continentes, e o relato das que faltam

**Rotinas:**
- `definir_chaves_fixas()` = 0x601a40 (index.cpp, cdecl): as chaves fixas (cores, continentes, nações, competições, prêmios, clubes) em -1 e depois a linha da base pelo nome (pelo eng.lng ou pela base); as que ficam sem valor vão à caixa "Unable to find the %s index" (7 fatais pela nação); no cm0102-1080.exe pela entrada 0x603834, com o -1 nas faixas inteiras;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL (16856662 bytes); carga IGUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutações (5):** 4 acusadas. `dbw_continentes` escapou: as chaves de continente (0x9cfa08..0x9cfa1c) não decidem nada na criação do .qst nem na carga até 2001-08-28; Ásia e Europa trocadas não mudaram o save. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 237ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 238`

**Campos tocados:**
- [0x9cf178..0x9d0527], [0xb63c9c..0xb63d03]: as chaves fixas (-1 e a linha da base); grava (definir_chaves_fixas)
- [0x9b5984] idioma; CM3_TEMP\eng.lng (8 contagens e as tabelas); [0xae23fc] lista de clubes; lê (definir_chaves_fixas)

**Interessa a:** SGE, CMRTE, CMScout
