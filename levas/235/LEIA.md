# 235ª leva — acrescentar às preferências: a pessoa favorita e a rejeitada do clube e a pessoa favorita da pessoa; o clube rejeitado da pessoa, o esfriar das relações pessoais e a lista das pessoas rejeitadas; a máscara da posição natural, quantos do elenco a têm e o companheiro favorito escolhido; as regras por nação: o clube fora da Argentina, fora da Argentina, França e Croácia, e a nação da Melanésia; a nação da Polinésia, o nome do cargo de treino e os anos entre duas datas em texto; as ordens dos clubes pelo nome curto e das pessoas pelo nome completo, e se a pessoa confere com os nomes; as ordens dos clubes pela latitude e pela longitude da cidade do estádio

**Rotinas:**
- `clube_acrescentar_pessoa_favorita()` = 0x547870 (Database.cpp, cdecl): põe a pessoa entre as três favoritas do clube (+0x9b..+0xa3): já favorita, nada; rejeitada (+0xa7..+0xaf), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3);
- `clube_acrescentar_pessoa_rejeitada()` = 0x547940 (Database.cpp, cdecl): põe a pessoa entre as três rejeitadas do clube (+0xa7..+0xaf): já rejeitada, nada; favorita (+0x9b..+0xa3), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3);
- `acrescentar_pessoa_favorita()` = 0x547a10 (Database.cpp, cdecl): põe a outra pessoa entre as três favoritas da pessoa (preferências +0x1c..+0x24): já favorita, nada; rejeitada (+0x28..+0x30), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3);
- `acrescentar_clube_rejeitado()` = 0x547e70 (Database.cpp, cdecl): põe o clube entre os três rejeitados da pessoa (preferências +0x10..+0x18): o clube já rejeita a pessoa (+0xa7..+0xaf), nada; favorito (+4..+0xc), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3);
- `esmaecer_relacoes()` = 0x547f90 (Database.cpp, cdecl): cada pessoa favorita (preferências +0x1c..+0x24) sai com o sorteio de (+0x59) + 15 dando 0; cada rejeitada (+0x28..+0x30), com o de 35 − (+0x5d);
- `pessoas_rejeitadas()` = 0x5480f0 (Database.cpp, cdecl): copia as pessoas rejeitadas presentes (preferências +0x28..+0x30) para a saída, contando no byte do terceiro argumento (zerado sempre);
- `mascara_da_posicao_natural()` = 0x5481a0 (Database.cpp, cdecl): a máscara das posições em que o jogador é natural (20): goleiro 1; laterais, zagueiros, meias, atacantes (bits 2..0xc00); sem nenhuma, a primeira posição com 15 ou mais; senão 0x10;
- `contar_elenco_com_mascara()` = 0x548150 (Database.cpp, cdecl): quantos dos 50 do elenco (+0xd7) com jogador têm a máscara da posição natural igual à dada;
- `escolher_companheiro_favorito()` = 0x548da0 (Database.cpp, cdecl): o colega do elenco que a pessoa escolhe para favorito: filtros pela pessoa (idade, +0x57, +0x59) e pelo colega (rejeitado, posição, contrato, reputação, 0x8cca10), nota pela reputação, média 0x7c1e30, já favorito, ids, nível do contrato e nacionalidade; o de nota mais alta;
- `clube_fora_da_argentina()` = 0x5487b0 (Database.cpp, cdecl): 1 se a nação do clube (+0x53) não é a Argentina (chave 0x9cf218); clube ou nação nulos: 0; o resto do eax é o do id;
- `clube_fora_de_argentina_franca_croacia()` = 0x5487d0 (Database.cpp, cdecl): 1 se a nação do clube não é Argentina, França nem Croácia (chaves 0x9cf218, 0x9cf300, 0x9cf2ac); nulos: 0;
- `nacao_da_melanesia()` = 0x53d900 (Database.cpp, cdecl): 1 se a nação é uma das cinco melanésias (chaves 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500);
- `nacao_da_polinesia()` = 0x53d940 (Database.cpp, cdecl): 1 se a nação é uma das cinco polinésias (chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514);
- `nome_do_cargo_de_treino()` = 0x53ac40 (Database.cpp, cdecl): o nome do cargo (5/12 técnico, 6/13 auxiliar, 8/15 treinador; outros: vazio) traduzido em 0xae1980; com o idioma 4 grava o gênero (4 ou 1) no segundo argumento;
- `anos_entre_datas_em_texto()` = 0x540340 (Database.cpp, cdecl): os anos completos entre as datas (ano do fim menos o do início, menos 1 se o dia do início é maior) em texto "%d" de palavra sem sinal; devolve 1;
- `ordem_dos_clubes_pelo_nome_curto()` = 0x540080 (Database.cpp, cdecl): comparação do qsort de clubes pelo nome curto (+0x38) sem maiúsculas (0x90c1d0); clube nulo: texto vazio;
- `ordem_das_pessoas_pelo_nome_completo()` = 0x540270 (Database.cpp, cdecl): comparação do qsort de pessoas pelo nome no estilo 1 (0x53dba0) sem maiúsculas; pessoa nula: texto vazio;
- `pessoa_confere_com_os_nomes()` = 0x53e990 (Database.cpp, cdecl): 1 se a pessoa (com jogador ou não jogador) tem os ids de nome, sobrenome e nome comum pedidos (nos não nulos) e, com data, a mesma data de nascimento;
- `ordem_dos_clubes_pela_latitude()` = 0x53fd40 (Database.cpp, cdecl): comparação do qsort de clubes pela latitude (+0x23, f64) da cidade (+0x38) do estádio (+0x69), a menor primeiro; sem cidade: 0;
- `ordem_dos_clubes_pela_longitude()` = 0x53fe50 (Database.cpp, cdecl): a mesma comparação pela longitude (+0x2b, f64);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (20):** 13 acusadas. `dbr_fora_da_argentina` escapou: só pesa em disciplina, técnicos e notícias de clubes da Argentina, ausentes das ligas da carreira; escapou também no jogo real comparando (0 divergências). `dbr_fora_de_arg_fra_cro` escapou: só muda algo para clubes da França (recurso de suspensão), ausentes das ligas da carreira; escapou também no jogo real comparando (0 divergências). `dbr_nome_do_cargo` escapou: o texto do cargo só aparece em notícias de aposentadoria e na regeneração, sem efeito no estado; escapou também no jogo real comparando (0 divergências). `dbr_anos_em_texto` escapou: só nas telas de recordes; escapou também no jogo real comparando (0 divergências). `dbr_pessoas_pelo_nome_completo` escapou: ordenação do gerente de transferências cujo resultado não mudou a carreira até 2002-06-01; escapou também no jogo real comparando (0 divergências). `dbr_confere_nomes` escapou: a busca de pessoa pelos nomes não teve chamada em que o sobrenome decidisse; escapou também no jogo real comparando (0 divergências). `dbr_longitude` escapou: a ordenação por longitude só é usada nos sorteios de copa de Bélgica, Holanda, Itália (C), Espanha e Turquia, fora da carreira inglesa; escapou também no jogo real comparando (0 divergências). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 234ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 235`

**Campos tocados:**
- TClub +0x9b..+0xa3 pessoas favoritas, +0xa7..+0xaf pessoas rejeitadas; lê e grava (clube_acrescentar_pessoa_favorita, clube_acrescentar_pessoa_rejeitada)
- preferências (TStaff +0x65) +0x1c..+0x24 pessoas favoritas, +0x28..+0x30 pessoas rejeitadas; lê e grava (acrescentar_pessoa_favorita)
- preferências (TStaff +0x65) +4..+0xc clubes favoritos, +0x10..+0x18 clubes rejeitados; TClub +0xa7..+0xaf pessoas rejeitadas; lê e grava (acrescentar_clube_rejeitado)
- preferências +0x1c..+0x30 pessoas favoritas e rejeitadas; TStaff +0x59, +0x5d; lê e grava (esmaecer_relacoes), lê (pessoas_rejeitadas)
- jogador (TStaff +0x61) +0xf..+0x19 posições, +5, +0xb reputação, +0x21; lê (mascara_da_posicao_natural, escolher_companheiro_favorito)
- TClub +0xd7 elenco (50), +0x53 nação, +0x80 reputação; TStaff +0x18, +0x1a nação, +0x3e/+0x42 nascimento, +0x57, +0x59, +0x5a, +0x5b; preferências +0x1c..+0x30; contrato +0x45, +0x4f; [0xae2364], [0xb74350] +0x10/+0x18; lê (contar_elenco_com_mascara, escolher_companheiro_favorito)
- TClub +0x53: nação; ponteiro; lê (clube_fora_da_argentina, clube_fora_de_argentina_franca_croacia)
- TNation +0: id; inteiro; lê (as três). Chaves 0x9cf218, 0x9cf300, 0x9cf2ac (Argentina, França, Croácia), 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (Melanésia); lê
- TNation +0: id; inteiro; lê (nacao_da_polinesia). Chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (Polinésia); lê
- 0xae1980: texto fixo do nome do cargo; grava (nome_do_cargo_de_treino)
- data +0 dia do ano, +2 ano (palavras); lê (anos_entre_datas_em_texto)
- TClub +0x38: nome curto; texto; lê (ordem_dos_clubes_pelo_nome_curto)
- TStaff +4/+8/+0xc: nome, sobrenome, nome comum (registro de nome +0x33 id); +0x10/+0x12 nascimento (dia, ano); +0x61/+0x69 jogador e não jogador; lê (pessoa_confere_com_os_nomes)
- TClub +0x69: estádio; ponteiro; estádio +0x38 cidade; cidade +0x23 latitude, +0x2b longitude (f64); lê (as duas)

**Interessa a:** SGE, CMRTE
