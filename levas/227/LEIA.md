# 227ª leva — as chaves fixas de clube e de nação pelo nome e a troca da chave de clube depois do corte da base; as chaves fixas das competições de clube e das competições e prêmios de pessoa pelo nome

**Rotinas:**
- `casar_chave_de_clube()` = 0x6146b0 (index.cpp, cdecl): casa o nome do clube da base com as 313 chaves fixas de clube (372 nomes em 4 cadeias, _stricmp): o primeiro acerto de cada cadeia grava a linha na global da chave;
- `remapear_chave_de_clube()` = 0x60ba80 (index.cpp, cdecl): depois do corte da tabela de clubes, a primeira chave de clube que guarda o id antigo passa a guardar o novo (devolve 1; nenhuma: 0);
- `casar_chave_de_nacao()` = 0x60d810 (index.cpp, cdecl): casa o nome da nação da base com as 209 chaves fixas de nação (217 nomes em 2 cadeias, _stricmp);
- `casar_chave_de_competicao_de_clube()` = 0x60efd0 (index.cpp, cdecl): casa o nome longo da competição de clube com as 301 chaves fixas (351 nomes em 3 cadeias, _stricmp; as duas divisões croatas gravam também 0x9cf8a4);
- `casar_chave_de_competicao_de_pessoa()` = 0x6115e0 (index.cpp, cdecl): casa o nome da competição ou prêmio de pessoa com as 417 chaves fixas (442 nomes em 5 cadeias, _stricmp);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856662 bytes).
- **Mutações (6):** 4 acusadas. `ix2_comp_pessoa` escapou: as chaves de competição de pessoa (0x9cfa20..0x9d00a0) são globais do exe, não vão para o .qst, e o início rápido não as lê antes de gravar; a prova do save (que avalia as mutações das levas do save) não as vê. `ix2_devolucao` escapou: quem chama (set_key_ids 0x601a40) ignora a devolução das rotinas de casar a chave (o resultado do último _stricmp). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 226ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 227`

**Campos tocados:**
- [0x9d00a4..0x9d0524] e [0xb63c9c..0xb63d00]: as 313 chaves de clube (nomes reais na tabela CHAVES_DE_CLUBE do codigo.rs: MLS All Stars East/West, River Plate, Vélez Sarsfield, Everton, Tottenham Hotspur, Chelsea, Valencia C.F., FC St. Pauli, Arminia Bielefeld...; grafias diferentes dão na mesma chave); id (linha da tabela de clubes); grava (casar_chave_de_clube), lê e grava (remapear_chave_de_clube)
- [0x9cf200..0x9cf544]: as 209 chaves de nação (0x9cf200 Afghanistan, 0x9cf204 Albania...; nomes em CHAVES_DE_NACAO, 217 com as grafias); id (linha da tabela de nações); grava (casar_chave_de_nacao)
- clube/nação +nome: o nome que a base dá (argumento das rotinas de casar); texto; lê (por _stricmp 0x951c50, sem distinguir maiúsculas; com o locale em [0xde3040+8])
- [0x9cf548..0x9cfa04]: as 301 chaves de competição de clube (nomes reais em CHAVES_DE_COMPETICAO_DE_CLUBE do codigo.rs: French Lower Division, German First Division, World Club Cup, UEFA Champions League, turkish premier division...; croatian first division e croatian a1 division gravam 0x9cf87c e 0x9cf8a4); id (linha da tabela de competições de clube); grava (casar_chave_de_competicao_de_clube)
- [0x9cfa20..0x9d00a0]: as 417 chaves de competição e prêmio de pessoa (nomes em CHAVES_DE_COMPETICAO_DE_PESSOA: FIFA World Player of the Year, European Footballer of the Year, Italian Serie A Team of the Year, Brazilian 1st Division Top Goalscorer...); id (linha da tabela de competições de pessoa); grava (casar_chave_de_competicao_de_pessoa)
- competição +nome longo / competição de pessoa +nome: o nome que a base dá (argumento); texto; lê (por _stricmp 0x951c50, sem distinguir maiúsculas)

**Interessa a:** CMRTE, SGE
