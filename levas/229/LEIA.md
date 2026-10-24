# 229ª leva — a leitura dos .dat: árbitros, cores, nomes; a leitura dos .dat com conversor: clubes, competições, não jogadores; a leitura dos .dat: jogadores, pessoas 110, preferências

**Rotinas:**
- `ler_arbitros()` = 0x5505c0 (Db_files.cpp, cdecl): lê a tabela officials.dat (id 7) de registros de 43 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_cores()` = 0x550710 (Db_files.cpp, cdecl): lê a tabela colour.dat (id 2) de registros de 58 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_nomes()` = 0x550860 (Db_files.cpp, cdecl): lê as tabelas first_names.dat, second_names.dat e common_names.dat (ids 13, 14, 15) de registros de 60 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;
- `ler_clubes()` = 0x54db50 (Db_files.cpp, cdecl): lê a tabela club.dat e nat_club.dat (ids 0, 1) de registros de 581 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2, e troca a reputação 0 por 1; versão 1: o conversor 0x54dc90 (outra: caixa e saída); 1 se veio tudo;
- `ler_competicoes()` = 0x54feb0 (Db_files.cpp, cdecl): lê a tabela club_comp.dat e nation_comp.dat (ids 12, 16) de registros de 107 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2; versão 1: o conversor 0x54ffd0 (outra: caixa e saída); 1 se veio tudo;
- `ler_nao_jogadores()` = 0x54e880 (Db_files.cpp, cdecl): lê a tabela dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players) de registros de 68 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2; versão 1: o conversor 0x54e9a0 (outra: caixa e saída); 1 se veio tudo;
- `ler_jogadores()` = 0x54f030 (Db_files.cpp, cdecl): lê a tabela dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players) de registros de 70 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2; versão 1: o conversor 0x54f150 (outra: caixa e saída); 1 se veio tudo;
- `ler_pessoas_110()` = 0x54e630 (Db_files.cpp, cdecl): lê a tabela staff.dat (pessoas, versão de arquivo de 110 bytes: o save e as bases da comunidade) (id 6) de registros de 110 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2 (outra: caixa e saída); 1 se veio tudo;
- `ler_preferencias()` = 0x54fc10 (Db_files.cpp, cdecl): lê a tabela das preferências das pessoas (id 22: o save e as bases da comunidade) de registros de 52 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL (16856662 bytes); carga IGUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutações (9):** 9 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 228ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 229`

**Campos tocados:**
- index.dat entrada +0x37: quantidade de registros; valor; lê (as três)
- index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)
- TOfficials (43 bytes, [0xae23e8]): +0x0 id; +0x4 primeiro nome e +0x8 sobrenome (ids); +0xc nascimento; +0x14 ano; +0x16 nação; +0x1a cidade; +0x1e capacidade atual; +0x20 potencial; +0x22 reputação; +0x24..+0x2a notas (deixar correr, disciplina, jogos importantes, pressão, arbitragem, bandeira, cronometragem); registro inteiro; grava (ler_arbitros)
- TColour (58 bytes, [0xae23ec]): +0x0 id; +0x4 nome (51); +0x37 vermelho; +0x38 verde; +0x39 azul; registro inteiro; grava (ler_cores)
- TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): +0x0 nome (51); +0x33 id; +0x37 nação; +0x3b contagem (peso); registro inteiro; grava (ler_nomes)
- index.dat entrada +0x37: quantidade de registros; valor; lê (as três)
- index.dat entrada +0x3f: versão da tabela; valor; lê (as três; 1 vai ao conversor; fora de 1 e 2, a original abre a caixa)
- TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x0 id; +0x4 nome (51, a chave por nome); +0x53 nação; +0x65 caixa; +0x69 estádio; +0x73..+0x7b público; +0x80 reputação (u16; 0 vira 1); +0x83..+0x97 cores; +0xb3..+0xbb rivais; +0xd7 elenco (50 ids); registro inteiro; grava (ler_clubes)
- TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x0 id; +0x4 nome (51, a chave por nome); +0x38 nome curto; +0x53 sigla; +0x57 escopo; +0x58 selecionada; +0x59 continente; +0x5d nação; +0x61/+0x65 cores; +0x69 reputação; registro inteiro; grava (ler_competicoes)
- TNonPlayer (68 bytes, [0xae23c0]): +0x0 id; +0x4 capacidade atual; +0x6 potencial; +0x8/+0xa/+0xc reputações (casa, atual, mundial); +0xe.. atributos de técnico (+0xe ataque, +0xf negócios, +0x10 treino...); registro inteiro; grava (ler_nao_jogadores)
- index.dat entrada +0x37: quantidade de registros; valor; lê (as três)
- index.dat entrada +0x3f: versão da tabela; valor; lê (as três; jogadores na versão 1 vão ao conversor; fora da versão aceita, a original abre a caixa)
- TPlayer (70 bytes, [0xae23c4]): +0x0 id; +0x4 número; +0x5 capacidade atual; +0x7 potencial; +0x9/+0xb/+0xd reputações (casa, atual, mundial); +0xf.. posições (+0xf goleiro, +0x10 líbero, +0x11 zagueiro...); atributos; registro inteiro; grava (ler_jogadores)
- TStaff (110 bytes, [0xae23bc]): +0x0 id; +0x4/+0x8/+0xc nomes (ids); +0x10 nascimento; +0x18 ano; +0x1a nação; +0x39 clube; +0x3e/+0x46 contrato; +0x4e salário; +0x52 valor; +0x56..+0x5d traços; +0x61 jogador, +0x65 preferências, +0x69 não jogador (índices); registro inteiro; grava (ler_pessoas_110)
- TPreferences (52 bytes, [0xae23c8]): +0x0 id; +0x4..+0xc clubes favoritos; +0x10..+0x18 clubes detestados; +0x1c..+0x24 pessoas favoritas; +0x28..+0x30 pessoas detestadas (ids); registro inteiro; grava (ler_preferencias)

**Interessa a:** SGE, CMRTE
