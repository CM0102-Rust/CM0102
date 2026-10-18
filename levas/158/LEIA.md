# 158ª leva — o save acima de 2 GB (o contêiner do jogo)

**Rotinas** (`si/code/zipdir.cpp`, o contêiner dos `.sav` e `.qst`):
- `abrir_no_conteiner()` = 0x9326e0 (`zopen`): abre um item para ler ou gravar;
- `buscar_no_conteiner()` = 0x933080 (`zseek`): anda dentro do item;
- `posicao_no_conteiner()` = 0x933170 (`ztell`): a posição atual no item.

**A causa do bug:** as três usam `fseek`/`ftell` de 32 bits com sinal. Acima de 2 GB, ao gravar, o `ftell` devolve −1 e o índice ganha a posição 0xffffffff; ao ler, a busca falha. O índice guarda posições de 32 bits sem sinal, que vão até 4 GB.

**Diferença de propósito em relação à original:**
- posições sem sinal, e busca e posição de 64 bits pelo Windows (`SetFilePointer`, com o mesmo cuidado da CRT com o `FILE`);
- o formato não muda;
- acima de 4 GB, o item é recusado sem estragar o arquivo.

**Como foi provado:**
- **Mutações (4):** as 4 escaparam. A carreira automática não salva nem carrega, então é uma lacuna em `docs/PENDENCIAS.md`.
- **`engenharia-reversa/save_grande.py`:** um início rápido com um enchimento de 2,2 GB gera um `.qst` de 2,38 GB, sem cliques.
  - Na original, os 45 itens além de 2 GB ficam com a posição 0xffffffff.
  - Com a 158ª, o índice sai certo.
- **Jogo do Lucas:**
  - um save de 2,55 GB, com 187 itens além de 2 GB, foi gravado e carregado com a DLL;
  - com as rotinas originais (`CM_ORIGINAL=1`), o carregamento falha com `Friendly.cpp`, no primeiro item além de 2 GB.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 157ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 158 -`
4. `python engenharia-reversa/save_grande.py --modo original`, que deve sair com o índice errado
5. `python engenharia-reversa/save_grande.py --modo rust`, que deve sair com o índice OK
