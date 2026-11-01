# 261ª leva — as convocações para o jogo de seleções e duas notícias: escolher os elencos dos dois lados, a notícia do convocado e a do desfecho na copa; as vagas da lista de observação com lance e as marcas de lista de transferências e de empréstimos do contrato; a situação da pessoa nas listas de observação dos clubes, a marca de lista de empréstimos e o teto de preço pelo estatuto; a verba de transferências repartida pelas vagas da lista de observação, o estatuto permitido pela idade e o nome das listas do olheiro; os amistosos internacionais do dia e a lista escolhida do olheiro do técnico humano; os inscritos da Liga dos Campeões: a lista das quatro bandas, a tabela da qualificação e a fase seguinte; o jogo da Liga dos Campeões: o prêmio e o coeficiente do jogo, o progresso do clube e o conflito de datas da competição; o árbitro de cada jogo: a escolha do árbitro, as marcas de ocupado do dia e a renovação dos árbitros da liga; os árbitros do jogo novo: os árbitros sorteados de uma nação, o conserto das fichas dos árbitros e a cópia de uma ficha; o prêmio do ano no histórico (o técnico conta, o jogador premiado agradece ou renova) e os recordes de vitória e de derrota; os jogos e as faltas das permissões de trabalho e os prêmios por jogos depois da partida, o prêmio por jogos na seleção guardado e a notícia da transferência finalizada; o veto dos fisioterapeutas e da diretoria à transferência, a notícia do ultimato da diretoria e o texto dela; a pessoa liberada sem custo, as férias do técnico humano e a notícia da permissão para discutir o contrato

**Rotinas:**
- `convocar_selecoes_para_o_jogo()` = 0x76bab0 (national_teams.cpp, thiscall, ret 4): a partir de duas semanas antes do jogo, para cada lado: elenco principal, B/olímpico ou sub-21 de até 14 dias e marcado só leva a data até o jogo; senão pede um novo (0x76c120); dispensa os acima da idade do sub-21 e apaga os repetidos;
- `noticia_do_convocado()` = 0x76fd10 (national_teams.cpp, stdcall, ret 0xc): com técnico humano no clube do jogador, a notícia 0x21 (o jogador, a seleção e a vaga) ao técnico do clube +0x39 e ao do clube do contrato;
- `noticia_do_desfecho_na_copa()` = 0x519690 (Cup.cpp, thiscall, ret 0x14): fora dos amistosos, a virtual +0x44 da competição e a notícia 0x10 (clube, competição, desfecho, categoria do resultado, o jogo, a rodada seguinte), entregue pelo alcance, à região da competição (final e semifinal) ou ao técnico do clube;
- `contar_vagas_com_lance()` = 0x844540 (shortlist_manager.cpp, thiscall): conta as vagas da lista do clube com alvo e lance na posição pedida (0xff: qualquer); com a lista no estado 2 de clube do humano, a vaga cujo técnico chegou há mais de 20 dias e cujo alvo não tem ofertas perde o alvo e o lance;
- `tirar_da_lista_de_transferencias()` = 0x844280 (shortlist_manager.cpp, stdcall): tira o bit 0 (lista de transferências) de +0x4e do contrato; 1 com contrato, 0 sem;
- `tirar_da_lista_de_emprestimos()` = 0x8442d0 (shortlist_manager.cpp, stdcall): tira o bit 1 (lista de empréstimos) de +0x4e do contrato; 1 com contrato, 0 sem;
- `situacao_na_lista_de_observacao()` = 0x843e10 (shortlist_manager.cpp, thiscall): aplica um evento de transferência da pessoa à lista do clube: no evento 4 do próprio clube, o evento 1 a cada clube interessado; o par (pessoa, clube) sai dos vetores ordenados e a conta de listas da pessoa cai; nas vagas com a pessoa, 1 refaz a vaga, 2 e 5 bloqueiam por 120 e 14 dias, 3 e 4 soltam com a data de hoje − 120;
- `por_na_lista_de_emprestimos()` = 0x8442b0 (shortlist_manager.cpp, stdcall): liga o bit 1 (lista de empréstimos) de +0x4e do contrato da pessoa;
- `teto_de_preco_pelo_estatuto()` = 0x84b1f0 (shortlist_manager.cpp, thiscall): o teto de preço do clube para o jogador no estatuto (0x84b230 com as posições fortes); −1 sem pessoa ou sem ficha de jogador;
- `distribuir_verba_de_transferencias()` = 0x83b6e0 (shortlist_manager.cpp, thiscall): reparte a verba de transferências do clube do computador pelas vagas abertas da lista (fatores do tipo e do estatuto), reduz pelos lances guardados e pela sobra, preenche cada vaga sem alvo e dá a sobra à primeira vaga de jogador do próprio clube que o técnico venderia;
- `estatuto_permitido()` = 0x84b4d0 (shortlist_manager.cpp, stdcall): os estatutos 5 e 6 (jovens) só abaixo de 24 anos; os outros, sempre;
- `nome_da_lista_do_olheiro()` = 0x7fc150 (scout_manager.cpp, cdecl): o nome traduzido da lista do olheiro: Short List, Player Search, Staff Search ou Scout #n;
- `amistosos_internacionais_do_dia()` = 0x5cbb00 (friendly.cpp, thiscall): no primeiro dia do ano esvazia as listas de nações, ordena as nações pela reputação e dá o ano às datas internacionais pelo ciclo da Copa; depois marca os amistosos das datas iguais a hoje + 12 semanas (0x5cbdf0), uma vez por dia;
- `lista_escolhida_do_tecnico()` = 0x7f6e30 (scout_manager.cpp, thiscall): a lista escolhida pelo técnico humano (nulo: o da vez);
- `escolher_lista_do_tecnico()` = 0x7f6f20 (scout_manager.cpp, thiscall): grava a lista escolhida pelo técnico humano (nulo: o da vez);
- `inscritos_da_liga_dos_campeoes()` = 0x584fa0 (european_cup.cpp, thiscall, ret 0): aloca a lista de 71 entradas de 6 bytes (+0x14) e põe nela os clubes da Liga dos Campeões (+0x1db = [0x9cf6e8]) pela banda (+0x1df): 1 até 16 (rodada 7), 2 até 18 (8), 3 até 19 (0xd), 4 até 18 (0); sem as 71, +0x36 = 0 e devolve 0;
- `tabela_da_qualificacao_da_liga_dos_campeoes()` = 0x584990 (european_cup.cpp, thiscall, ret 0): aloca as 55 linhas da qualificação (+0xa7, +0x3e = 55): a linha i leva o clube da entrada 70 − i e a rodada em que entra (8 → 2, 0xd → 1, outra → 0); sem memória, −1;
- `proxima_fase_da_liga_dos_campeoes()` = 0x5851d0 (european_cup.cpp, thiscall, ret 0): passa à fase seguinte (+0x30) e sorteia os grupos da fase 1 (fase 0), os da fase 2 (8) ou as quartas (12); na última fase, a caixa de erro;
- `processar_jogo_da_liga_dos_campeoes()` = 0x583b10 (european_cup.cpp, thiscall, ret 8): pelo placar (+0x47/+0x48, ou +0x43/+0x44), 2 pontos no coeficiente da UEFA da nação do vencedor ou 1 para cada no empate (0x9058b0); na fase de grupos (+0x42 de 0 a 11), 200 mil ao vencedor ou 100 mil a cada um, com a notícia do prêmio (0x48e3e0); depois a base da copa (0x51a150);
- `progresso_do_clube_na_liga_dos_campeoes()` = 0x586c00 (european_cup.cpp, thiscall, ret 0x14): converte os três códigos de progresso pela fase (qualificação, fase 1, fase 2) e grava em +0x73..+0x75 do registro do clube na estatística da competição;
- `conta_para_conflito_de_datas()` = 0x48e360 (comp.cpp, thiscall, ret 8): 1 se a rodada do jogo (+0x30) termina em 0 ou passa de 999, senão 0;
- `escolher_arbitro_da_partida()` = 0x79c7b0 (officials_manager.cpp, thiscall, ret 8): a faixa de árbitros pela competição (a da liga no bloco da nação, o quarto de cima de uma nação neutra do continente nas competições dos tipos 2 e 4, a nação do mandante no amistoso), um sorteado nela e o seguinte livre;
- `limpar_arbitros_ocupados()` = 0x79bf40 (officials_manager.cpp, thiscall, ret 0): zera os bytes de árbitro ocupado no dia;
- `renovar_arbitros()` = 0x79cee0 (officials_manager.cpp, thiscall, ret 4): no começo da temporada da liga, troca os árbitros da faixa com mais de 45 anos (ou mais de 40, um em três) por novos sorteados e zera a linha de estatísticas do primeiro;
- `arbitros_sorteados()` = 0x79c5c0 (officials_manager.cpp, thiscall (this sem uso), ret 8): realoca a tabela dos árbitros, gera `conta` árbitros novos da nação (nomes pelo peso, notas e atributos sorteados, nascidos nos anos 1960) e ordena a tabela por nação e qualidade;
- `validar_arbitros()` = 0x79caa0 (officials_manager.cpp, cdecl): conserta cada ficha de árbitro: a nação pelos nomes, as notas e os atributos fora da faixa sorteados de novo e a data de nascimento antiga demais;
- `copiar_ficha_de_arbitro()` = 0x79c530 (officials_manager.cpp, thiscall, ret 4): copia uma ficha de árbitro de 0x2b bytes campo a campo, a data pela cópia de data;
- `registrar_premio_do_ano()` = 0x86c390 (staff_records.cpp, thiscall, ret 0x18): a entrada do prêmio do ano no histórico da pessoa; o técnico soma o prêmio no registro dele; o jogador com contrato longo na lista do clube recebe a oferta de renovação e o primeiro colocado pode agradecer ao técnico (sorteio pela lealdade);
- `vitoria_maior()` = 0x7e4a30 (record_utils.cpp, cdecl): o registro de partida é uma vitória por diferença maior que a do outro, ou igual com mais gols feitos;
- `derrota_maior()` = 0x7e4b00 (record_utils.cpp, cdecl): o registro de partida é uma derrota por diferença maior que a do outro, ou igual com mais gols sofridos;
- `jogos_das_permissoes_e_clausulas()` = 0x8c39f0 (transfer_manager.cpp, thiscall): depois da partida, nos dois lados: paga o prêmio por jogos aos 1, 10, 20... 50 jogos (clube pelo contrato, seleção pelos jogos desde o acordo) e tira o bônus pago; no jogo que conta, soma um jogo à permissão de trabalho de quem jogou e uma falta à dos que ficaram de fora sem lesão, suspensão, convocação nem empréstimo;
- `guardar_premio_por_jogos_na_selecao()` = 0x8cad90 (transfer_manager.cpp, thiscall): acrescenta o prêmio por jogos na seleção (13 bytes) ao vetor, crescendo de 50 em 50, e reordena pela pessoa;
- `noticia_transferencia_finalizada()` = 0x8da5e0 (transfer_manager.cpp, thiscall): a notícia 0xfa5 da transferência acertada, com a data de conclusão, ao vendedor e ao comprador humanos;
- `veto_dos_fisios_e_da_diretoria()` = 0x696b10 (manager_manager.cpp, thiscall): comprando um jogador lesionado, os fisioterapeutas sorteiam o veto; num clube humano, a diretoria veta pela verba, pelo pedido ou pelo valor e perde confiança;
- `noticia_do_ultimato()` = 0x6ac700 (manager_manager.cpp, thiscall): a notícia 0x178d do ultimato antes do jogo, ao técnico do clube ou, por sorteio, aos da divisão ou do país;
- `texto_do_ultimato()` = 0x6ac880 (manager_manager.cpp, stdcall): o título ou o corpo da notícia do ultimato, com os nomes e as ligações do técnico e dos clubes;
- `liberar_sem_custo()` = 0x8c1af0 (transfer_manager.cpp, thiscall): registra a transferência livre e, fora do só registro, paga a compensação ao clube, tira a pessoa das listas, das escalações, do elenco, das ofertas e do contrato, das cláusulas e da permissão de trabalho;
- `ferias_do_humano()` = 0x5fd2f0 (human_manager.cpp, thiscall): liga ou desliga as férias do técnico humano, com a conta dos humanos de férias; ao ligar, responde as notícias pendentes;
- `noticia_conversa_de_contrato()` = 0x8d7cd0 (transfer_manager.cpp, thiscall): a notícia 0xfac da permissão para discutir o contrato ao comprador humano e, por jogador da troca, ao vendedor humano;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (37):** 34 acusadas. `cpy10_sem_sorteio_59` escapou: nenhum premiado da carreira teve o +0x59 acima de 15, onde o sorteio do agradecimento entra [inferência]. `cpy13_fica_na_escalacao` escapou: o jogador liberado sem custo que continua na escalação sai dela na próxima escolha do time da máquina, antes de jogar [inferência]. `cpy13_sem_listas` escapou: a situação do liberado nas listas de observação só pesa numa oferta por ele, que a carreira até 2002-06-01 não fez [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (0b8bc0f1).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 65356052, original 8c36a9f9 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 260ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 261`

**Campos tocados:**
- registro da nação (NationalTeams +4, 0x6e bytes) +0x32/+0x46/+0x5a: a marca de hoje dos elencos principal, B e sub-21; valor (data); grava (convocar_selecoes_para_o_jogo)
- registro da nação +0x3a/+0x4e/+0x62: a data dos elencos; valor (data); lê e grava (convocar_selecoes_para_o_jogo)
- registro da nação +0x43/+0x57/+0x6b: a marca de elenco convocado; valor; lê (convocar_selecoes_para_o_jogo)
- partida +0x28/+0x2a: a data do jogo; +0x40 a marca; +0x4d/+0x4e os bits do jogo (1 sem convocar, 2 sem dispensar, 0x100 B, 0x200 sub-21); valor; lê (convocar_selecoes_para_o_jogo)
- NationalTeams +0x36: o vetor das nações com convocados (byte por id); ponteiro; lê (convocar_selecoes_para_o_jogo)
- [0xae1080] o último contrato lido; ponteiro; grava (noticia_do_convocado)
- pessoa +0x39: o clube da seleção do jogador; ponteiro; lê (noticia_do_convocado)
- copa +0x3c / +0xa3: as rodadas e o vetor delas (0x68 bytes, +7 a rodada); valor e ponteiro; lê (noticia_do_desfecho_na_copa)
- Shortlist_Manager +0xdc: entrada de 9 bytes por clube (+0 vagas, +4 estado); ponteiro; lê (contar_vagas_com_lance)
- vaga da lista (0x25 bytes) +0 posição, +0xf alvo, +0x17 lance; valor e id; lê, grava −1 no alvo e no lance vencidos (contar_vagas_com_lance)
- clube +0xcf: técnico (+0x3e data de chegada); ponteiro; lê (contar_vagas_com_lance)
- contrato +0x4e: bit 0 lista de transferências, bit 1 lista de empréstimos; valor; grava (tirar_da_lista_de_transferencias, tirar_da_lista_de_emprestimos)
- Shortlist_Manager +0/+0x10 e +4/+0x14: vetores ordenados de pares (pessoa, clube) de 8 bytes e contas; ponteiro e valor; lê e grava (situacao_na_lista_de_observacao)
- Shortlist_Manager +0xdc: entrada de 9 bytes por clube (+0 vagas, +8 grava 100); ponteiro; lê e grava (situacao_na_lista_de_observacao)
- vaga da lista (0x25 bytes) +0 tipo, +7 e +0xb valores, +0xf alvo, +0x13 pessoa bloqueada, +0x17 lance, +0x1b data, +0x24 estatuto e marca; lê e grava (situacao_na_lista_de_observacao)
- [0xae2d0c] registro de 79 bytes por pessoa +0xb: nibble baixo = em quantas listas; valor; grava −1 (situacao_na_lista_de_observacao)
- contrato +0x4f: nibble alto = estatuto no elenco; valor; grava (situacao_na_lista_de_observacao)
- contrato +0x4e: bit 1 lista de empréstimos; valor; grava (por_na_lista_de_emprestimos)
- pessoa +0x39 clube, +0x61 ficha de jogador; ponteiro; lê (situacao_na_lista_de_observacao, teto_de_preco_pelo_estatuto)
- Shortlist_Manager +0xdc: entrada de 9 bytes por clube (+0 vagas, +4 estado 1); ponteiro; lê (distribuir_verba_de_transferencias)
- vaga da lista (0x25 bytes) +0 tipo, +3, +7 verba, +0xb valor, +0xf alvo, +0x13, +0x17 lance, +0x24 estatuto e marca; lê e grava (distribuir_verba_de_transferencias)
- clube +0x64: 1 = clube rico (vagas de não jogador com verba); valor; lê (distribuir_verba_de_transferencias)
- pessoa +0x52 valor, +0x39 clube, +0x18 idade, +0x61 ficha; lê (distribuir_verba_de_transferencias, estatuto_permitido)
- contrato +0x4f nibble alto estatuto, +0x25/+0x27 data; lê (distribuir_verba_de_transferencias)
- [0xae1080] último contrato consultado; ponteiro; grava (distribuir_verba_de_transferencias)
- Friendly +0x34ec/+0x34f0: datas internacionais (20 bytes: +0 dia da semana, +1 dia, +2 mês, +3 data, +0xb deslocamento do ano) e conta; lê e grava a data (amistosos_internacionais_do_dia)
- Friendly +0x34f4: último dia feito (palavra); valor; lê e grava (amistosos_internacionais_do_dia)
- Friendly +0x34f6 listas de nações ([0xae2350] delas), +0x34fa/+0x34fe nações e conta; lê, esvazia e ordena (amistosos_internacionais_do_dia)
- [0xdd6ec8] data-limite dos amistosos; valor; lê (amistosos_internacionais_do_dia)
- SCOUT_MANAGER +0x4c: lista escolhida por técnico humano (16 bytes); valor; lê e grava (lista_escolhida_do_tecnico, escolher_lista_do_tecnico)
- clube +0x1db: a competição europeia da temporada; id; lê (inscritos_da_liga_dos_campeoes)
- clube +0x1df: a banda de entrada na Liga dos Campeões (1 a 4); valor; lê (inscritos_da_liga_dos_campeoes)
- [0x9cf6e8] a Liga dos Campeões; id; lê (inscritos_da_liga_dos_campeoes)
- Liga dos Campeões +0x14 / +0x36 / +0x38: a lista dos inscritos (6 bytes: +0 clube, +4 rodada, +5 0) e as contas; ponteiro e valor; grava (inscritos_da_liga_dos_campeoes), lê (tabela_da_qualificacao_da_liga_dos_campeoes)
- Liga dos Campeões +0xa7 / +0x3e: a tabela da qualificação (6 bytes: +0 clube, +4 rodada de entrada 0/1/2, +5 0) e quantas linhas; ponteiro e valor; grava (tabela_da_qualificacao_da_liga_dos_campeoes)
- Liga dos Campeões +0x30 / +0x2c: a fase atual e quantas fases; valor; lê e grava a fase (proxima_fase_da_liga_dos_campeoes)
- jogo +0x47 / +0x48 (ou +0x43 / +0x44 com +0x47 = −1): o placar; valor; lê (processar_jogo_da_liga_dos_campeoes)
- jogo +0x42: a fase (0 a 11 a fase de grupos); valor; lê (processar_jogo_da_liga_dos_campeoes)
- jogo +0x30: a rodada; valor; lê (conta_para_conflito_de_datas)
- [0xde1f58] +0xf + 18 × índice: os pontos do coeficiente da UEFA da nação (o índice é +0x1df da seleção); valor; grava pela 0x9058b0 (processar_jogo_da_liga_dos_campeoes)
- finanças do clube +0 (64 bits), +0xa8, +0x148: o saldo e os acumulados de prêmios; valor; grava pela 0x5999a0 (processar_jogo_da_liga_dos_campeoes)
- estatística da competição, registro do clube +0x73 / +0x74 / +0x75: os códigos de progresso; valor; grava (progresso_do_clube_na_liga_dos_campeoes)
- Liga dos Campeões +0x30: a fase atual; valor; lê no comparador (processar_jogo_da_liga_dos_campeoes)
- OfficialsManager +0 linhas de estatísticas (17 bytes por id: +4 média, +8, +0xa, +0xc palavras, +0xf cartões), +4 conta e +8 vetor dos bytes de ocupado, +0xc blocos por nação (0x102 bytes: +4 conta, +8 primeiro, +0x10 número de faixas, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição); ponteiros/valores; lê (escolher_arbitro_da_partida, renovar_arbitros), grava os bytes de ocupado (limpar_arbitros_ocupados) e a linha de estatísticas (renovar_arbitros)
- árbitro (0x2b bytes em [0xae23e8], [0xae2390] deles) +0 id, +4 e +8 nomes, +0xc dia e +0xe ano de nascimento, +0x16 nação, +0x25 nota; ids/ponteiros/valores; lê +0 (escolher_arbitro_da_partida), lê e grava +4, +8, +0xc, +0xe, +0x25 (renovar_arbitros)
- jogo +0 (−2 tirado), +0x14 competição, +0x1c e +0x20 clubes, +0x2e árbitro (palavra, a saída); competição +0 id, +0x59 continente, +0x5d nação; objeto da competição ([0xadadfc]) +0x42 tipo; clube +0x53 nação; nação (0x122 bytes em [0xae23a8]) +0 id, +0x71 continente; ids/ponteiros; lê (escolher_arbitro_da_partida, renovar_arbitros)
- [0xae2c90] data do jogo (dia, ano); [0xae2350] nações, [0xae234c] continentes; valores; lê
- árbitro (0x2b bytes em [0xae23e8], [0xae2390] deles) +0 id, +4 e +8 nomes (registro de nome +0x37 índice da nação), +0xc dia e +0xe ano de nascimento, +0x16 nação, +0x1a, +0x1e, +0x20 e +0x22 notas (palavras; a ordem usa +0x28 × 50 + +0x22), +0x24..+0x2a atributos (bytes 1..20); ponteiros/valores; grava as fichas novas, a conta e a tabela (arbitros_sorteados), lê e grava (validar_arbitros, copiar_ficha_de_arbitro)
- nação (0x122 bytes em [0xae23a8]) +0x71 continente, +0x8e nível dos árbitros (byte); [0xae2350] nações; ponteiros/valores; lê
- entrada do histórico da pessoa (FILE_LLIST [Staff_Records], começo e fim em [+0xc] + id × 8) +0 data, +8 clube ou −1, +0xc tipo 0x34, +0xd prêmio, +0x11 a4, +0x15 a5, +0x19 crédito; valores; grava (registrar_premio_do_ano)
- registro de pontos do técnico (0x86f5b0) +0x9e prêmios (palavra); valor; lê e grava (registrar_premio_do_ano)
- pessoa +0 id, +0x28 e +0x3d cargos, +0x59 e +0x5b atributos de caráter (bytes com sinal), +0x61 ficha de jogador; contrato (0x4cf5a0/0x4cf5f0) +4 clube, +0x25/+0x29 data do fim, +0x35 tipo, +0x45 marca; [0xae1080] último contrato; valores/ponteiros; lê, grava [0xae1080] (registrar_premio_do_ano)
- registro de partida +0x19 gols feitos, +0x1a gols sofridos (bytes com sinal); valores; lê (vitoria_maior, derrota_maior)
- jogo +0 (−2), +0x14 competição, +0x1c casa, +0x20 fora, +0x40/+0x41 o lado conta; valor/ponteiro; lê (jogos_das_permissoes_e_clausulas)
- partida + lado × 0x625 + vaga × 0x49: +0x190 id, +0x1ad, +0x1c2; id/valor; lê
- gerência de transferências +0x86e: registro de 8 bytes por pessoa (bit 2 cláusula do clube, 4 da seleção, 0x20 permissão; +7 jogos pela seleção no acordo); valor; lê
- permissão de trabalho (0xe bytes em [+0x93a], conta +0x93e) +4 jogos, +6 faltas; valor; grava +1 (jogos_das_permissoes_e_clausulas)
- pessoa +0x22 jogos pela seleção, +0x61 ficha; valor/ponteiro; lê
- contrato +0x3b jogos pelo clube, +4 clube, +0x35 & 0x3f tipo; valor; lê
- gerência de transferências +0x898 prêmios na seleção (13 bytes), +0x89c capacidade, +0x8a0 conta; ponteiro/valor; grava (guardar_premio_por_jogos_na_selecao)
- oferta +4 troca, +8 comprador, +0xc pessoa, +0x11, +0x1b valor, +0x20, +0x24, +0x2c estado, +0x2f, +0x30; valor; lê (noticia_transferencia_finalizada)
- [0xae1080] último contrato consultado; ponteiro; grava
- diretoria (0x49 bytes em [gerência dos técnicos] + id × 0x49) +6, +0x14, +0x16 confianças; valor; lê e grava (veto_dos_fisios_e_da_diretoria)
- clube +0x1cf três fisioterapeutas, +0x53 nação (+0x11c), +0x57 divisão, +0x80 reputação; ponteiro/valor; lê
- lesão +0x28 gravidade; valor; lê
- pessoa +0x39 clube atual, +0x61 ficha (+0x2d); ponteiro/valor; lê
- contrato +0x35 & 0x3f tipo, +0x3d, +4 clube; valor; lê
- [0xad6c30] rand da CRT; valor; grava (srand pelos ids e pelo relógio)
- [0x9cf764] chave da divisão inferior; id; lê
- clube +0xcf técnico, +4 e +0x38 nomes, +0x37 e +0x52 gêneros; ponteiro/valor; lê (noticia_do_ultimato, texto_do_ultimato)
- competição (0x6b bytes em [0xae23d0]) +0x38 nome, +0x52 gênero; valor; lê (texto_do_ultimato)
- [0xae1080] último contrato consultado; ponteiro; grava
- gerência de transferências +0x862 transferências passadas (0x25 bytes: +0 pessoa, +4..+0xc chaves, +0x10, +0x14 clube, +0x18, +0x1c tipo, +0x1d data), +0x866 capacidade, +0x86a conta; ponteiro/valor; grava (liberar_sem_custo)
- pessoa +0x39 clube atual, +0x3e data, +0x52 valor, +0x6d, +0x61 ficha; ponteiro/valor; lê e grava (liberar_sem_custo)
- contrato +0x35 & 0x3f tipo, +0xc salário, +4 clube; valor; lê
- humanos (0xa6ef bytes em [this]) +0x9a01 férias; [this+4] quantos, [this+0xc] de férias; valor; grava (ferias_do_humano)
- [0xb74c26 + 0x300 × [0xb77c7a]] técnico humano da vez; ponteiro; lê
- oferta +4 troca, +8 comprador, +0xc pessoa, +0x20, +0x24; valor; lê (noticia_conversa_de_contrato)
- [0xae1080] último contrato consultado; ponteiro; grava

**Interessa a:** CMRTE, CMScout, SGE, GPF2
