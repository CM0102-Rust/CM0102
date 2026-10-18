"""Documentacao da 158a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva158.py` (4, pela carreira até 2002-06-01): as 4 escaparam; a carreira automática não salva nem carrega (lacuna em `PENDENCIAS.md`). "
     "A prova é o `save_grande.py`: um início rápido com um enchimento de 2,2 GB gera um `.qst` de 2,38 GB; na original os 45 itens além de 2 GB ficam com posição 0xffffffff, com a 158ª o índice sai certo. "
     "No jogo do Lucas: save de 2,55 GB (187 itens além de 2 GB) gravado e carregado com a nossa DLL; com as originais (`CM_ORIGINAL=1`) o carregamento falha (`Friendly.cpp`, primeiro item além de 2 GB). "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva158.py` (4, by the career to 2002-06-01): all 4 escaped; the automatic career neither saves nor loads (gap in `PENDENCIAS.en.md`). "
      "The proof is `save_grande.py`: a quick start with a 2.2 GB filler makes a 2.38 GB `.qst`; in the original the 45 items beyond 2 GB get position 0xffffffff, with batch 158 the index is right. "
      "In Lucas's game: a 2.55 GB save (187 items beyond 2 GB) saved and loaded with our DLL; with the originals (`CM_ORIGINAL=1`) loading fails (`Friendly.cpp`, first item beyond 2 GB). "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(158, 347, 350, 'o save acima de 2 GB (o contêiner do jogo)', 'saves above 2 GB (the game container)',
      ["| `abrir_no_conteiner()` | 0x9326e0 | cdecl | zipdir.cpp (`zopen`): abre o item `nome` do contêiner (cabeçalho de 12 bytes em +0x19, índice de 1000 itens de 0x10c bytes em +0x25: posição, tamanho, nome); grava acrescentando no fim (máx. 999) ou lê indo à posição. **Diferença de propósito:** posição sem sinal e busca/posição em 64 bits (a original usa `fseek`/`ftell` de 32 bits com sinal: acima de 2 GB o índice ganha 0xffffffff e a leitura falha); acima de 4 GB recusa o item sem estragar o arquivo |",
       "| `buscar_no_conteiner()` | 0x933080 | cdecl | zipdir.cpp (`zseek`): anda no item aberto para leitura (origem 0/1/2); fora do item, -1; tipo 4 (RLE) volta ao começo e relê até o ponto, tipo 3 busca direto. Posições sem sinal, busca em 64 bits |",
       "| `posicao_no_conteiner()` | 0x933170 | cdecl | zipdir.cpp (`ztell`): tipo 4 devolve posição do item + z+8; tipo 3, o `ftell` (aqui em 64 bits, descontando o buffer como a 0x946aae) menos a posição do item |"],
      ["| `abrir_no_conteiner()` | 0x9326e0 | cdecl | zipdir.cpp (`zopen`): opens item `nome` of the container (12-byte header at +0x19, index of 1000 items of 0x10c bytes at +0x25: position, size, name); writes appending at the end (max. 999) or reads seeking to the position. **Deliberate difference:** unsigned position and 64-bit seek/tell (the original uses signed 32-bit `fseek`/`ftell`: above 2 GB the index gets 0xffffffff and reading fails); above 4 GB it refuses the item without damaging the file |",
       "| `buscar_no_conteiner()` | 0x933080 | cdecl | zipdir.cpp (`zseek`): moves within the item open for reading (origin 0/1/2); outside the item, -1; type 4 (RLE) goes back to the start and re-reads up to the point, type 3 seeks directly. Unsigned positions, 64-bit seek |",
       "| `posicao_no_conteiner()` | 0x933170 | cdecl | zipdir.cpp (`ztell`): type 4 returns item position + z+8; type 3, `ftell` (64-bit here, discounting the buffer like 0x946aae) minus the item position |"],
      M, ME,
      [('abrir_no_conteiner', 'open_in_container'), ('buscar_no_conteiner', 'seek_in_container'), ('posicao_no_conteiner', 'position_in_container')],
      "'narrar_no_painel': 'commentate_on_panel',",
      ("O contêiner (`zipdir.cpp`) só roda ao salvar, ao carregar e no início rápido: a carreira automática não o exercita (mutações escapam). Prova: `engenharia-reversa/save_grande.py --modo rust|original` (um `.qst` acima de 2 GB, sem cliques). O jogo apaga os `*.tmp` ao criar a carreira: o enchimento espera fora da pasta e entra por renomeação quando surge o primeiro `.tmp`. Um `ftell` próprio tem de descontar o buffer do `FILE` (`_cnt` em +4 na leitura, `_ptr`−`_base` na escrita).",
       "The container (`zipdir.cpp`) only runs when saving, loading and in the quick start: the automatic career does not exercise it (mutations escape). Proof: `engenharia-reversa/save_grande.py --modo rust|original` (a `.qst` above 2 GB, no clicks). The game deletes the `*.tmp` files when creating the career: the filler waits outside the folder and comes in by rename when the first `.tmp` appears. A home-made `ftell` must discount the `FILE` buffer (`_cnt` at +4 when reading, `_ptr`−`_base` when writing)."),
      secao=('Save', 'Save'))
