# 230ª leva — a gravação dos .dat: continentes, nações, cidades; a gravação dos .dat: estádios, clubes, pessoas 110; a gravação dos .dat: não jogadores, jogadores, preferências

**Rotinas:**
- `gravar_continentes()` = 0x54d6e0 (Db_files.cpp, cdecl): grava a tabela continent.dat (id 3) de registros de 198 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_nacoes()` = 0x54d840 (Db_files.cpp, cdecl): grava a tabela nation.dat (id 4) de registros de 290 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_cidades()` = 0x54d9a0 (Db_files.cpp, cdecl): grava a tabela city.dat (id 21) de registros de 56 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_estadios()` = 0x54daf0 (Db_files.cpp, cdecl): grava a tabela stadium.dat de registros de 78 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_clubes()` = 0x54e5d0 (Db_files.cpp, cdecl): grava a tabela club.dat e nat_club.dat (ids 0, 1) de registros de 581 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_pessoas_110()` = 0x54e720 (Db_files.cpp, cdecl): grava a tabela staff.dat (pessoas, 110 bytes; id 6) de registros de 110 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_nao_jogadores()` = 0x54efd0 (Db_files.cpp, cdecl): grava a tabela dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players) de registros de 68 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_jogadores()` = 0x54fbb0 (Db_files.cpp, cdecl): grava a tabela dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players) de registros de 70 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;
- `gravar_preferencias()` = 0x54fd00 (Db_files.cpp, cdecl): grava a tabela das preferências das pessoas (id 22) de registros de 52 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856662 bytes).
- **Mutações (9):** 9 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 229ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 230`

**Campos tocados:**
- TContinent (198 bytes, [0xae23a4]): registro inteiro (+0xbe força regional, double); lê (gravar_continentes: vai para o arquivo)
- TNation (290 bytes, [0xae23a8]): registro inteiro (os rivais já trocados por índices pelo save_database); lê (gravar_nacoes)
- TCities (56 bytes, [0xae23ac]): registro inteiro; lê (gravar_cidades)
- TStadium (78 bytes): registro inteiro (+0x3c capacidade); lê (gravar_estadios)
- TClub (581 bytes, [0xae23b4 / 0xae23b8]): registro inteiro (ponteiros já trocados por índices); lê (gravar_clubes)
- TStaff (110 bytes, [0xae23bc]): registro inteiro (ponteiros já trocados por índices); lê (gravar_pessoas_110)
- TNonPlayer (68 bytes, [0xae23c0]): registro inteiro; lê (gravar_nao_jogadores)
- TPlayer (70 bytes, [0xae23c4]): registro inteiro; lê (gravar_jogadores)
- TPreferences (52 bytes, [0xae23c8]): registro inteiro; lê (gravar_preferencias)

**Interessa a:** SGE, CMRTE
