# 228ª leva — a leitura dos .dat: continentes, nações, cidades; a leitura dos .dat: estádios, pessoas 157, histórico de pessoas; a leitura dos .dat: históricos de competição, competições de pessoal, histórico de competições de pessoal

**Rotinas:**
- `ler_continentes()` = 0x54d5e0 (Db_files.cpp, cdecl): lê a tabela continent.dat (id 3) de registros de 198 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_nacoes()` = 0x54d740 (Db_files.cpp, cdecl): lê a tabela nation.dat (id 4) de registros de 290 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2 (outra: caixa e saída); 1 se veio tudo;
- `ler_cidades()` = 0x54d8a0 (Db_files.cpp, cdecl): lê a tabela city.dat (id 21) de registros de 56 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada de 1 a 2 (com sinal) (outra: caixa e saída); 1 se veio tudo;
- `ler_estadios()` = 0x54da00 (Db_files.cpp, cdecl): lê a tabela stadium.dat (id 5) de registros de 78 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_pessoas_157()` = 0x54e780 (Db_files.cpp, cdecl): lê a tabela staff.dat (pessoas, versão de arquivo de 157 bytes) (id 6) de registros de 157 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_historico_de_pessoas()` = 0x54fd60 (Db_files.cpp, cdecl): lê a tabela staff_history.dat (id 17) de registros de 17 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_historicos_de_competicao()` = 0x550230 (Db_files.cpp, cdecl): lê a tabela club_comp_history.dat e nation_comp_history.dat (id 19, 20) de registros de 26 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_competicoes_de_pessoal()` = 0x550380 (Db_files.cpp, cdecl): lê a tabela staff_comp.dat (id 11) de registros de 101 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2 (outra: caixa e saída); 1 se veio tudo;
- `ler_historico_de_competicoes_de_pessoal()` = 0x5504d0 (Db_files.cpp, cdecl): lê a tabela staff_comp_history.dat (id 18) de registros de 58 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856662 bytes).
- **Mutações (9):** 8 acusadas. `dbf_historico_de_competicoes_de_pessoal` escapou: a tabela do histórico de competições de pessoal não é item do .qst (não há staff_comp_history.dat no contêiner) e o início rápido não grava nada derivado dela; a prova do save não a vê. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 227ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 228`

**Campos tocados:**
- index.dat entrada +0x37: quantidade de registros; valor; lê (as três)
- index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)
- TContinent (198 bytes, [0xae23a4]): +0x0 id; +0x4 nome (26); +0x1f sigla; +0x23 continentalidade; +0x3d federação (101); +0xa3 federação curta; +0xbe força regional (double); registro inteiro; grava (ler_continentes)
- TNation (290 bytes, [0xae23a8]): +0x0 id; +0x4 nome (51, a chave por nome); +0x57 nacionalidade; +0x71 continente; +0x7a capital; +0x80 estádio; +0x8e reputação; +0x90..+0xa4 cores; +0xa8..+0x108 coeficientes FIFA/UEFA (double); +0x110..+0x118 rivais (ids que viram ponteiro no resolve); registro inteiro; grava (ler_nacoes)
- TCities (56 bytes, [0xae23ac]): +0x0 id; +0x4 nome (26); +0x1f nação; +0x23 latitude e +0x2b longitude (double); +0x33 atração; +0x34 clima; registro inteiro; grava (ler_cidades)
- index.dat entrada +0x37: quantidade de registros; valor; lê (as três)
- index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)
- TStadiums (78 bytes, [0xae23b0]): +0x0 id; +0x4 nome (51); +0x38 cidade; +0x3c capacidade; +0x40 sentados; +0x44 expansão; +0x48 estádio vizinho; +0x4c coberto; +0x4d aquecimento; registro inteiro; grava (ler_estadios)
- TStaff v1 (157 bytes, [0xae2404 (temporária; convertida para 110 bytes em 0xae23bc)]): 157 bytes; os primeiros campos coincidem com o TStaff de 110 (nascimento +0x10, ano +0x18, contrato +0x3e/+0x46); o layout inteiro está no CM0102-Editor (P8); registro inteiro; grava (ler_pessoas_157)
- TStaffHistory (17 bytes, [0xae23cc]): +0x0 id; +0x4 pessoa; +0x8 ano; +0xa clube; +0xe emprestado; +0xf jogos; +0x10 gols; registro inteiro; grava (ler_historico_de_pessoas)
- index.dat entrada +0x37: quantidade de registros; valor; lê (as três)
- index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)
- TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): +0x0 id; +0x4 competição; +0x8 ano; +0xa campeão; +0xe vice; +0x12 terceiro; +0x16 sede; registro inteiro; grava (ler_historicos_de_competicao)
- TStaffComp (101 bytes, [0xae23e0]): +0x0 id; +0x4 nome (51, a chave por nome); +0x38 nome curto; +0x53 continente; +0x57 nação; +0x5b/+0x5f cores; +0x63 reputação; registro inteiro; grava (ler_competicoes_de_pessoal)
- TStaffCompHistory (58 bytes, [0xae23e4]): +0x0 id; +0x4 competição; +0x8 ano; +0xa..+0x36 campeão, vice e terceiro (primeiro nome, sobrenome, pessoa, info); registro inteiro; grava (ler_historico_de_competicoes_de_pessoal)

**Interessa a:** SGE, CMRTE
