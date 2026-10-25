# 232ª leva — liberar os históricos de pessoas e de prêmios e crescer o histórico de competições; as ordens dos clubes pela reputação e dos elencos pela habilidade; a normalização física e geral dos atributos do jogador e o segundo nome pelo peso; os conversores da versão 1: clubes, não jogadores, jogadores; o conversor das competições da versão 1 e os nomes em inglês da nação e o curto do clube; a criação do jogo: os elencos dos times A e B, a comissão do time B, o índice de nomes por nação; a criação do jogo: tirar a pessoa do clube, os olheiros do clube com os do time parceiro

**Rotinas:**
- `liberar_historico_de_pessoas()` = 0x527ee0 (Database.cpp, cdecl): com a tabela do histórico de pessoas presente, solta (GlobalFree no banco compartilhado, free no outro) e zera ponteiro e contagem;
- `liberar_historico_de_premios()` = 0x527f20 (Database.cpp, cdecl): o mesmo para o histórico de prêmios (competições de pessoal);
- `crescer_historico_de_competicoes()` = 0x527f60 (Database.cpp, cdecl): realloc do histórico de competições para n linhas a mais (sem memória: 0); contagem, ponteiro das seleções e as linhas novas com o próprio índice em +0 e o resto zero; 1;
- `ordem_dos_clubes_pela_reputacao()` = 0x540020 (Database.cpp, cdecl): comparação do qsort dos ids de clube pela reputação (+0x80), a maior primeiro; id negativo = 0;
- `ordem_do_elenco_pela_habilidade()` = 0x5402f0 (Database.cpp, cdecl): comparação do qsort dos elencos: pessoa nula no fim, depois a palavra +0xb do jogador, a maior primeiro;
- `ordem_do_elenco_pela_habilidade_atual()` = 0x549c30 (Database.cpp, cdecl): comparação do qsort do elenco: sem jogador no fim, depois a habilidade atual (+5) e, empatada, a palavra +0xb, a maior primeiro;
- `normalizar_atributos_fisicos_do_jogador()` = 0x538ec0 (Database.cpp, cdecl): baixa os físicos que passam da habilidade atual (corte pela soma ponderada com as constantes do .rdata) e reequilibra o vetor de atributos sem as células 0x18 e 0x1e, alvo de −40 a 0; com +0xf = 20 tira médias de pares e trios; tira e devolve o progresso do treino;
- `segundo_nome_pelo_peso()` = 0x53a0e0 (Database.cpp, cdecl): um segundo nome: com a nação tendo mais de 10, o de maior peso entre 3 sorteios na faixa dela; senão sorteio na tabela inteira;
- `normalizar_atributos_do_jogador()` = 0x538bc0 (Database.cpp, cdecl): reequilibra o vetor de atributos do jogador com alvo −20 (12 células no limite, excesso acima de 85 repartido); com +0xf ≥ 15 tira a média de +0x2a/+0x3a; tira e devolve o progresso do treino;
- `ler_clubes_v1()` = 0x54dc90 (Db_files.cpp, cdecl): lê o club.dat versão 1 campo a campo (577 bytes no arquivo para registros de 581, buracos em +0x37, +0x52 e +0x244), a reputação num byte (0 vira 1) × 500; 0 na primeira leitura que não vem, 1 se veio tudo;
- `ler_nao_jogadores_v1()` = 0x54e9a0 (Db_files.cpp, cdecl): lê a tabela dos não jogadores (id 9; o mapa da comunidade a chama de players) versão 1 campo a campo (65 bytes no arquivo para registros de 68), as três reputações em bytes; 0 na primeira leitura que não vem, 1 se veio tudo;
- `ler_jogadores_v1()` = 0x54f150 (Db_files.cpp, cdecl): lê a tabela dos jogadores (id 10; o mapa da comunidade a chama de non_players) versão 1 campo a campo (67 bytes no arquivo para registros de 70), as três reputações em bytes; 0 na primeira leitura que não vem, 1 se veio tudo;
- `ler_competicoes_v1()` = 0x54ffd0 (Db_files.cpp, cdecl): lê o club_comp.dat / nation_comp.dat versão 1 campo a campo (104 bytes no arquivo para registros de 107, buracos em +0x37 e +0x52), a reputação num byte; 0 na primeira leitura que não vem, 1 se veio tudo;
- `nome_ingles_da_nacao()` = 0x5453f0 (Database.cpp, cdecl): o nome da nação, ou o em inglês do eng.lng (linhas de 109 bytes, pelo id) quando outro idioma está carregado;
- `nome_curto_ingles_do_clube()` = 0x545770 (Database.cpp, cdecl): o nome curto do clube, ou o em inglês do eng.lng (linhas de 79 bytes, pelo mapa de ids ou pelo id) quando outro idioma está carregado;
- `cortar_elencos_de_a_e_b()` = 0x541e60 (Database.cpp, cdecl): marca em cada clube se ele tem time parceiro (+0x244) e, em cada time A, corta o elenco conjunto A+B a 48 pela reputação, de trás para a frente, mantendo 3 goleiros;
- `juntar_o_time_b_ao_a()` = 0x544970 (Database.cpp, cdecl): em cada time A com time B: corta o elenco do B ao que couber em 50 com o do A e passa a comissão do B (técnicos, olheiros, fisioterapeutas e o treinador quando o A não tem) para o A;
- `montar_indice_de_nomes()` = 0x539750 (Database.cpp, cdecl): aloca e monta o índice por nação das tabelas de primeiros, segundos e nomes comuns (primeira linha, soma dos pesos e quantidade de cada grupo); o último grupo de cada tabela fica sem gravar, como na original;
- `tirar_pessoa_do_clube()` = 0x53c190 (Database.cpp, cdecl): tira a pessoa da casa do clube que o cargo dela indica (gravando nela a data, quando vem, e, no treinador da seleção principal, também da sub-21) e, se pedido, zera o clube na pessoa; 0 se ela não estava na casa;
- `olheiros_do_clube()` = 0x53cd60 (Database.cpp, cdecl): copia os 7 olheiros do clube para um vetor; com time parceiro (A/B), junta os dois até 7 e zera o resto;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856662 bytes).
- **Mutações (20):** 11 acusadas. `dbi_crescer_historico_de_competicoes` escapou: só roda no fim de uma competição; a prova do save só cria e grava, e a carga vai até 2001-08-28. `dbi_ordem_do_elenco_pela_habilidade_atual` escapou: só roda na força do elenco da IA durante a temporada, não na criação nem na gravação do .qst. `dbj1_clubes` escapou: a base de fábrica e o save são versão 2 e o jogo dos testes está em inglês: nem a prova do save nem a carreira passam por ela; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj1_nao_jogadores` escapou: a base de fábrica e o save são versão 2 e o jogo dos testes está em inglês: nem a prova do save nem a carreira passam por ela; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj1_jogadores` escapou: a base de fábrica e o save são versão 2 e o jogo dos testes está em inglês: nem a prova do save nem a carreira passam por ela; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj2_competicoes` escapou: a base de fábrica e o save são versão 2 e o jogo dos testes está em inglês: nem a prova do save nem a carreira passam por ela; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj2_clube_curto` escapou: a base de fábrica e o save são versão 2 e o jogo dos testes está em inglês: nem a prova do save nem a carreira passam por ela; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbk1_juntar_comissao` escapou: nenhum time B das ligas do teste parece ter técnico, olheiro ou fisioterapeuta na criação, então nada passa ao time A (a conferir pela base). `dbk2_olheiros` escapou: a criação não chama a 0x53cd60 (só a carreira); a prova do save não a vê. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 231ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 232`

**Campos tocados:**
- [0xae23cc] / [0xae2374]: tabela e contagem do histórico de pessoas (17 bytes); ponteiro; solta e zera (liberar_historico_de_pessoas)
- [0xae23e4] / [0xae238c]: tabela e contagem do histórico de prêmios (58 bytes); ponteiro; solta e zera (liberar_historico_de_premios)
- [0xae2340]: banco no bloco compartilhado (GlobalAlloc); valor; lê (as duas liberações)
- [0xae23d4] / [0xae237c] / [0xae23dc]: histórico de competições (26 bytes), contagem e parte das seleções (a partir da linha [0xae2384]); tabela; escreve (crescer_historico_de_competicoes)
- clube +0x80: reputação; palavra com sinal; lê (ordem_dos_clubes_pela_reputacao)
- pessoa +0x61: jogador; ponteiro; lê. Jogador +5: habilidade atual; +0xb: a palavra do desempate; palavras com sinal; lê (as duas ordens de elenco)
- pessoa +0x61: jogador; ponteiro; lê. Jogador (70 bytes) +0x0f..+0x44: atributos; bytes com sinal; lê e escreve (as duas normalizações, pelo 0x53a610/0x53aa10 e direto: +0x1b, +0x36, +0x40, +0x44, +0x2a, +0x3a, +0x1e, +0x24, +0x39)
- jogador +5: habilidade atual; palavra com sinal; lê (normalizar_atributos_fisicos_do_jogador)
- pessoa +0x58: célula 54 do vetor; byte; escreve (pelo 0x53aa10)
- [0xde1f44]: TRAINING_MANAGER; ponteiro; lê (as duas normalizações: tirar e devolver o progresso, 0x8b3640)
- índice de nomes da nação ([0xae2338] + id × 36) +4 / +0x1c: primeiro e quantos segundos nomes; valor; lê. [0xae23f4] (60 bytes) +0x3b: peso; byte com sinal; lê (segundo_nome_pelo_peso)
- TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x80 reputação (u16; versão 1: byte, 0 vira 1, × 500); registro inteiro menos +0x37, +0x52 e +0x244; grava (ler_clubes_v1)
- TNonPlayer (68 bytes, [0xae23c0]): +0x8/+0xa/+0xc reputações (u16; versão 1: bytes); registro inteiro; grava (ler_nao_jogadores_v1)
- TPlayer (70 bytes, [0xae23c4]): +0x9/+0xb/+0xd reputações (u16; versão 1: bytes); registro inteiro; grava (ler_jogadores_v1)
- TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x69 reputação (u16; versão 1: byte); registro inteiro menos +0x37 e +0x52; grava (ler_competicoes_v1)
- 0x9b5984 idioma dos nomes; valor; lê (nome_ingles_da_nacao, nome_curto_ingles_do_clube)
- 0xae2408 tabela do eng.lng (+0 quantos clubes, +0x20 clubes em linhas de 79, +0x2c nações em linhas de 109); ponteiro; lê (as duas)
- 0xae23fc mapa de ids dos clubes do eng.lng; ponteiro; lê (nome_curto_ingles_do_clube)
- TClub +0x244: tem time parceiro; byte; grava (cortar_elencos_de_a_e_b), lê (juntar_o_time_b_ao_a)
- TClub +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas do clube; ponteiros; grava (as duas, por 0x53c190 e 0x53bd10)
- TStaff +0x39 clube, +0x3d cargo, +0x3e data de entrada; grava (as duas, por 0x53c190 e 0x53bd10)
- TNames (60 bytes): +0x37 nação, +0x3b peso; lê (montar_indice_de_nomes)
- índice de nomes [0xae2338] (0x24 bytes por nação): +0/+4/+8 primeira linha, +0xc/+0x10/+0x14 soma dos pesos, +0x18/+0x1c/+0x20 quantidade (primeiros, segundos, comuns); grava (montar_indice_de_nomes)
- TClub +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas do clube; ponteiros; grava (tirar_pessoa_do_clube)
- TClub +0x53 nação; lê (tirar_pessoa_do_clube)
- TClub +0x1b3 (7) olheiros; ponteiros; lê (olheiros_do_clube)
- TStaff +0x24 seleção, +0x28 cargo na seleção, +0x29 data na seleção, +0x39 clube, +0x3d cargo, +0x3e data no clube; grava (tirar_pessoa_do_clube)

**Interessa a:** SGE, CMRTE
