# 231ª leva — a gravação dos .dat: histórico de pessoas, competições, históricos de competição; a gravação dos .dat: competições de pessoal, árbitros, cores; a gravação dos .dat: nomes; abrir e criar os .dat

**Rotinas:**
- `gravar_historico_de_pessoas()` = 0x54fe50 (Db_files.cpp, cdecl): grava de uma vez n registros de 17 bytes (staff_history.dat, id 17) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo;
- `gravar_competicoes()` = 0x5501d0 (Db_files.cpp, cdecl): grava de uma vez n registros de 107 bytes (club_comp.dat, em duas chamadas: as competições e as 127 de reserva; nation_comp.dat) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo;
- `gravar_historicos_de_competicao()` = 0x550320 (Db_files.cpp, cdecl): grava de uma vez n registros de 26 bytes (club_comp_history.dat e nation_comp_history.dat, ids 19 e 20) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo;
- `gravar_competicoes_de_pessoal()` = 0x550470 (Db_files.cpp, cdecl): grava de uma vez n registros de 101 bytes (staff_comp.dat, id 11) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo;
- `gravar_arbitros()` = 0x5506b0 (Db_files.cpp, cdecl): grava de uma vez n registros de 43 bytes (officials.dat, id 7) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo;
- `gravar_registros_58()` = 0x550800 (Db_files.cpp, cdecl): grava de uma vez n registros de 58 bytes (colour.dat, id 2, e staff_comp_history.dat, id 18: a mesma rotina para os dois) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo;
- `gravar_nomes()` = 0x550950 (Db_files.cpp, cdecl): grava de uma vez n registros de 60 bytes (first_names.dat, second_names.dat e common_names.dat, ids 13 a 15) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo (só grava, apesar do nome da comunidade);
- `abrir_arquivo_do_banco()` = 0x54d580 (Db_files.cpp, cdecl): monta pasta + nome em 0xde1f64 (_makepath) e devolve fopen(caminho, "rb"); o comparador confere o caminho;
- `criar_arquivo_do_banco()` = 0x54d5b0 (Db_files.cpp, cdecl): monta pasta + nome em 0xde1f64 (_makepath) e devolve fopen(caminho, "wb"); só ao gravar a base na pasta; o comparador confere o caminho;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856662 bytes).
- **Mutações (9):** 7 acusadas. `dbh_gravar_historico_de_pessoas` escapou: o save_database só chama a 0x54fe50 com [0xae23cc] ≠ 0 (0x5311ac); no jogo o histórico de pessoas já virou staff history.tmp, então a gravação só roda ao gravar a base na pasta Data, nunca no .qst. `dbh_criar` escapou: a 0x54d5b0 só é chamada em save_database(…, 0, …), a gravação da base na pasta Data; o .qst é aberto pelo zopen 0x9326e0. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 230ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 231`

**Campos tocados:**
- TStaffHistory (17 bytes, [0xae23cc]): registro inteiro; lê (gravar_historico_de_pessoas)
- TComp (107 bytes, [0xae23d0 / 0xae23d8]): registro inteiro; lê (gravar_competicoes)
- TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): registro inteiro; lê (gravar_historicos_de_competicao)
- TStaffComp (101 bytes, [0xae23e0]): registro inteiro; lê (gravar_competicoes_de_pessoal)
- TOfficials (43 bytes, [0xae23e8]): registro inteiro; lê (gravar_arbitros)
- TColour (58 bytes, [0xae23ec]) e TStaffCompHistory (58 bytes, [0xae23e4]): registro inteiro; lê (gravar_registros_58)
- TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): registro inteiro; lê (gravar_nomes)
- 0xde1f64 (caminho do .dat, 260): texto; grava (abrir_arquivo_do_banco, criar_arquivo_do_banco)

**Interessa a:** SGE, CMRTE
