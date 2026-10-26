# 240ª leva — o adjetivo de nacionalidade pelo idioma e pela forma

**Rotinas:**
- `adjetivo_da_nacao()` = 0x6181d0 (index.cpp, cdecl): o adjetivo da nação no texto (tamanho >= 50): com gênero 0 ou fora dos idiomas 9, 4, 5, 10 e 6 o nome da nação; nesses, a forma da 0x545a60 e as cadeias de 937 nações (geradas do exe por adjetivos_nacao.py, textos pelo endereço), com o nome se o texto ficar vazio (menos nos idiomas 4 e 5); devolve 1 (0 com nação ou texto nulo ou tamanho curto);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 3 acusadas. `dbx_forma` escapou: a forma 5 (min(5) → min(4)) só aparece nas tabelas de italiano e espanhol, que o jogo em inglês não usa; escapou também no jogo real comparando (0 divergências). `dbx_vazio` escapou: o nome no fim só muda algo nos idiomas 4 e 5 (italiano, espanhol); escapou também no jogo real comparando (0 divergências). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 239ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 240`

**Campos tocados:**
- TNation +0 id, +0x57 nome; [0x9cf178..0x9d0527] chaves das nações; [0x9eb2dc] idioma; lê (adjetivo_da_nacao)

**Interessa a:** SGE, CMRTE, CMScout
