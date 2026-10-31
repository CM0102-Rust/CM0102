# 254ª leva — os limites da escalação: a tabela de restrições da partida, o limite do banco do dia e quem precisa de permissão de trabalho; quem pode negociar: se o clube pode chegar ao jogador, os termos mandados ao jogador da troca e a fila da transferência livre do clube; as transferências futuras: a do jogador, tirar a da oferta e acrescentar a da oferta acertada; as ofertas abertas e as desfeitas: as ofertas pelos jogadores do clube, desfazer as ofertas pelo jogador e guardar o percentual da próxima venda; as diretorias no início da carreira, as apelações de suspensão e a partida que o humano pode ver; as aposentadorias: quem se aposenta no começo do jogo, quem se aposenta pela idade e a lista inicial; a busca de novas aposentadorias, a saída dos cargos de quem se aposenta e o registro da liberação; o estado da lesão do jogador, a data da decisão sobre as ofertas e os candidatos a técnico do ano; a projeção da liga para a diretoria: as posições projetadas do clube e as duas ordens da projeção; o calendário dos jogos: achar os jogos de um clube ou competição entre duas datas, os jogos da competição num dia e se a competição tem jogo na data; o clube livre para o amistoso, o clube na copa e a nação parceira do calendário

**Rotinas:**
- `restricoes_da_partida()` = 0x8cf6b0 (transfer_manager.cpp, thiscall, ret 0xc): enche a tabela de 42 bytes com 0xff e, pelo alcance das regras da partida (0x8d2fa0), deixa a regra da nação ou do continente enchê-la (virtual +0x1c); com o pedido, fica só o primeiro limite (+5/+3, +2/+0 ou +0x16/+0x17);
- `limite_do_banco_do_dia()` = 0x8cf880 (transfer_manager.cpp, thiscall (this em ecx), ret 0x18): conta, pelas vagas do banco do dia (jogadores dados ou a tática 0x8968f0), os não comunitários, os estrangeiros ou os acima da idade do limite da partida (0x8cf7e0); passando, 1 e o aviso;
- `precisa_de_permissao_de_trabalho()` = 0x8c7560 (transfer_manager.cpp, thiscall, ret 0xc): 1 se o clube é de nação com permissão de trabalho (+0x76 = 0xe), o jogador não é da UE e não tem a permissão dada (contrato do tipo 2, marca 0x20, busca no vetor +0x93a);
- `clube_pode_chegar_ao_jogador()` = 0x8ce990 (transfer_manager.cpp, thiscall, ret 0x10): não pode (0 e o motivo): nação que não deixa sair (0x27), contrato acertado (0x21), trocas abertas (7), fila livre (1), oferta em negociação (0x12, marca +0x2d) ou transferência futura (0x20), as regras do clube basco (0x19), da nação [0x9cf4dc] (0x28) e dos clubes [0x9d0128]/[0x9d01a0] (0x1a); senão 1;
- `termos_ao_jogador_da_troca()` = 0x8c2060 (transfer_manager.cpp, thiscall, ret 8): marca o jogador da troca a quem foram mandados os termos; nenhum marcado: a oferta vai ao estado 8 (de 7) ou 6;
- `fila_livre_do_clube()` = 0x8c1780 (transfer_manager.cpp, thiscall, ret 4): conta as pessoas da fila da transferência livre do clube ou do parceiro dele (0x540a50);
- `transferencia_futura_do_jogador()` = 0x8cca10 (transfer_manager.cpp, thiscall, ret 8): com a marca 0x10, procura nas ofertas pelo jogador (0x8c5cf0) e nas trocas que o oferecem (0x8c5e40) a de estado 0x13; achando, o registro do vetor +0x920 e 1; senão tira a marca e 0;
- `tirar_transferencia_futura()` = 0x8cbf70 (transfer_manager.cpp, thiscall, ret 4): tira a marca 0x10 do jogador e dos da troca e o registro da oferta do vetor +0x920 (os seguintes descem, 0x8cbf40);
- `acrescentar_transferencia_futura()` = 0x8cb9a0 (transfer_manager.cpp, thiscall, ret 8): acrescenta (data, oferta) ao vetor +0x920 e reordena; a oferta vai ao estado 0x13; suspende as listas de alvos pelo jogador e pelos da troca (0x842b00) e liga a marca 0x10 deles; o contrato marcado guarda a oferta;
- `ofertas_abertas_pelos_jogadores_do_clube()` = 0x8c6100 (transfer_manager.cpp, thiscall, ret 8): conta (e lista os índices de) as ofertas abertas por jogadores com contrato no clube, fora dos estados 2, 3, 4, 0x10, 0x14, das desfeitas e dos empréstimos;
- `desfazer_ofertas_pelo_jogador()` = 0x8c47d0 (transfer_manager.cpp, thiscall, ret 4): cada oferta pelo jogador perde a transferência futura (0x8cbf70), fica desfeita (0xf, +0x2e = 3), solta os lances do comprador e do vendedor (0x84aeb0) e é agendada (0x8c6220, 10 dias);
- `guardar_percentual_da_proxima_venda()` = 0x8caaf0 (transfer_manager.cpp, thiscall, ret 4): acrescenta os 13 bytes do percentual da próxima venda ao vetor +0x880 (cresce de 50 em 50) e reordena (0x8e54d0);
- `iniciar_diretorias_dos_clubes()` = 0x68f130 (manager_manager.cpp, thiscall, ret 0): arruma as funções das pessoas com clube (técnico em +0xcf) e monta a linha da diretoria de cada clube: datas, reputação², as quatro confianças por sorteio a partir da expectativa (nota da diretoria ou reputação) e da reputação, presas a 5750/5500..10000, ou 5000 sem técnico; as expectativas da temporada (0x6a3770) e, sem técnico, a vaga (0x69ba50);
- `apelacoes_de_suspensao()` = 0x6abef0 (manager_manager.cpp, thiscall, ret 0): para cada apelação (+0x14) cuja data chegou: o resultado nas até 4 regras da disciplina (0x557000), a notícia (0x6ac060) e a apelação sai da lista;
- `pode_ver_a_partida()` = 0x5fe700 (human_manager.cpp, thiscall, ret 8): 1 se a partida tem +0x43 negativo e nem ela nem as do mesmo dia (cada grupo das nações-chave) têm um clube do humano; senão 0;
- `aposentadoria_no_inicio()` = 0x7a79c0 (player_regen.cpp, thiscall, ret 8): no jogo novo, a pessoa com clube se aposenta? Sem ficha de jogador pelo limite de idade do cargo; jogador pelo limite da ficha, a habilidade, a competição do clube e o estatuto do contrato; a saída (id, clube, motivo) e as datas do anúncio e da aposentadoria pela nação-chave;
- `aposentadoria_por_idade()` = 0x7a7310 (player_regen.cpp, thiscall, ret 8): na busca de novas aposentadorias, a pessoa se aposenta? Como a do começo, com o limite ajustado pelo contrato e pelo salário, o desemprego, o fim do contrato e o potencial; sem nação-chave, datas sorteadas perto de hoje;
- `lista_inicial_de_aposentadorias()` = 0x7a58f0 (player_regen.cpp, thiscall): no jogo novo, testa cada pessoa elegível (0x7a79c0), acrescenta quem se aposenta e ordena a lista;
- `novas_aposentadorias()` = 0x7a5790 (player_regen.cpp, thiscall): testa cada pessoa elegível (fora os jovens de competição do jogo, os marcados, os já na lista e os de contrato marcado) pela idade (0x7a7310) ou pelo desemprego (0x7a8050), acrescenta quem se aposenta e ordena a lista;
- `sair_dos_cargos()` = 0x7aaeb0 (player_regen.cpp, stdcall, ret 4): tira a pessoa das vagas do cargo no clube e na seleção (presidente, diretores, técnico com a vaga aberta, auxiliar, treinadores, olheiros, fisioterapeutas) e zera o clube, a seleção e os tipos de cargo;
- `registrar_a_liberacao()` = 0x86d800 (staff_records.cpp, thiscall, ret 4): os históricos da liberação sem custo: o do técnico do clube (tipo 0x68) e o da pessoa (tipo 0x67);
- `estado_da_lesao()` = 0x62c400 (injury.cpp, thiscall, ret 4): atualiza a forma do dia (0x62c680) e devolve 2 enquanto a lesão dura, 1 enquanto recupera a forma e 0 pronto, pela diferença da data do jogo para a da lesão;
- `marcar_decisao_da_oferta()` = 0x4cd230 (contract_manager.cpp, thiscall, ret 8): acha ou cria a entrada de decisão de tipo 3 da pessoa e marca o dia e a fatia em que ela responde: pelo prazo da oferta, ou sem oferta pela data de hoje e pelo fim do contrato; no máximo daqui a 14 dias;
- `candidatos_a_tecnico_do_ano()` = 0x86fab0 (staff_records.cpp, thiscall, ret 8): ordena os clubes e guarda até três cujo técnico está no cargo há 28 semanas e tem jogos no registro;
- `projetar_posicoes_da_liga()` = 0x68ac40 (league.cpp, thiscall, ret 4): projeta os pontos de cada clube da tabela no fim da temporada (no primeiro terço, por uma escala pela reputação), ordena e entrega à virtual +0x68 as posições do clube na forma normal, melhor (× 1,5) e pior (× 0,444);
- `ordem_da_projecao_pela_reputacao()` = 0x68afb0 (league.cpp, cdecl): comparador do qsort das entradas da projeção pela reputação do clube (+0x80), a maior primeiro;
- `ordem_da_projecao_pelos_pontos()` = 0x68afd0 (league.cpp, cdecl): comparador do qsort das entradas da projeção pelos pontos projetados (+0xa), o maior primeiro;
- `achar_jogos_do_calendario()` = 0x5acbe0 (fix_man.cpp, thiscall, ret 0x24): conta (e copia para a lista, se dada) os jogos entre duas datas nas listas das nações-chave do clube ou da competição, com os filtros de tipo, lado do clube e competição;
- `jogos_da_competicao_no_dia()` = 0x5abd90 (fix_man.cpp, thiscall, ret 0x1c): copia para a lista os jogos de um dia e fatia da competição, rodada e tipo pedidos e devolve quantos;
- `competicao_tem_jogo_na_data()` = 0x5ae6d0 (fix_man.cpp, stdcall, ret 8): diz se a competição tem jogo na data em alguma das três fatias do calendário;
- `clube_livre_na_data()` = 0x5c6b70 (friendly.cpp, stdcall, ret 0xc): diz se o clube está livre para um amistoso na data: sem jogo da véspera até a folga do tipo e, fora das seleções, fora da temporada da nação-chave;
- `copa_tem_o_clube()` = 0x522910 (Cup.cpp, thiscall, ret 4): diz se o clube está na tabela de times da copa;
- `nacao_parceira_do_clube()` = 0x5af5b0 (fix_man.cpp, cdecl): para clube inglês ou galês, o índice da nação-chave ativa da outra nação; senão 0xff;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (27):** 24 acusadas. `cpr9_vaga_nao_abre` escapou: o ramo do técnico rodou 2 vezes na carreira, mas a vaga só abre quando a 0x544820 deixa o clube procurar técnico [inferência: não deixou nas duas]; o comparando também não divergiu. `cpr12_qualquer_rodada` escapou: nos dias e competições pedidos com rodada fixa (a mudança das rodadas das copas para as datas fixas) todos os jogos da lista já são da rodada pedida, e o filtro não tira nenhum [inferência]; o comparando também não divergiu. `cpr13_copa_sem_o_clube` escapou: o ramo rodou 32 vezes (projeção da diretoria, 0x4ae300), mas a projeção da copa não muda nada que a carreira até 2002-06-01 use [inferência]; o comparando também não divergiu. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (8c95873456684fd1).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (7f4ecded7ef233a0).

**Refazer**, sobre o commit da 253ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 254`

**Campos tocados:**
- TRANSFER_MANAGER +0x8ac: as 28 regras de transferência das nações (+0xa nação, +0xe continente, +0x12 tipo, virtual +0x1c enche a tabela); ponteiro; lê (restricoes_da_partida)
- tabela de restrições (42 bytes): +0/+2 estrangeiros, +3/+5 não comunitários, +0x16 idade e +0x17 quantidade; valor; grava (restricoes_da_partida)
- partida +0x1c/+0x20: mandante e visitante; +0x40/+0x41: lado do computador; +0x4d & 0xf: reservas além dos 11; ponteiro e valor; lê (limite_do_banco_do_dia)
- limite de escalação (6 bytes): +0 tipo, +1 idade, +2 titulares, +3 reservas, +4 total, +5 quantidade; valor; grava (limite_do_banco_do_dia)
- staff +0x18: idade; valor; lê (limite_do_banco_do_dia)
- clube +0x53: a nação; nação +0x76: a marca 0xe (permissão de trabalho); ponteiro e valor; lê
- contrato +0x35 & 0x3f: o tipo (2); valor; lê (precisa_de_permissao_de_trabalho)
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x20; +0x93a/+0x93e: as permissões (14 bytes) e a conta; valor; lê (precisa_de_permissao_de_trabalho)
- staff +0x1a / +0x1e: as nacionalidades; +0x39: o clube do contrato; +0x61: a ficha de jogador; ponteiro; lê (clube_pode_chegar_ao_jogador, fila_livre_do_clube)
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x40 (contrato acertado); valor; lê e grava (clube_pode_chegar_ao_jogador)
- TRANSFER_MANAGER +0x92c: a fila da transferência livre (LLIST de ids; cursor +0x936); id; lê (clube_pode_chegar_ao_jogador, fila_livre_do_clube)
- oferta de transferência +4: a lista da troca (itens: +0 id, +4 termos mandados); +0x2c: o estado; +0x2d: a marca; valor; lê e grava (clube_pode_chegar_ao_jogador, termos_ao_jogador_da_troca)
- clube +0x53 / +0x57: a nação e a região; ponteiro; lê (clube_pode_chegar_ao_jogador)
- [0x9cf458], [0x9cf424], [0x9cf4dc], [0x9cf8d8], [0x9cf488], [0x9d0128], [0x9d01a0]: os ids das nações e dos clubes das regras; id; lê (clube_pode_chegar_ao_jogador)
- TRANSFER_MANAGER +0x920 / +0x924 / +0x928: as transferências futuras (12 bytes: data +0, oferta +8), a capacidade e a conta; ponteiro, valor e id; lê e grava
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x10 (transferência futura); valor; lê e grava
- oferta de transferência +4: a lista da troca; +0xc: o jogador; +0x20: o número da oferta; +0x2c: o estado (0x13); +0x2d: a marca; id e valor; lê e grava +0x2c/+0x2d (acrescentar_transferencia_futura)
- contrato +0x49 / +0x4a: a marca e a oferta da transferência futura; valor e id; lê e grava (acrescentar_transferencia_futura)
- TRANSFER_MANAGER +0x856: as listas de alvos dos clubes (Shortlist_Manager); ponteiro; lê (acrescentar_transferencia_futura)
- oferta de transferência +4: a lista da troca; +8: o comprador; +0xc: o jogador; +0x2c: o estado; +0x2e: o motivo do fim (3 = desfeita); id e valor; lê e grava +0x2c/+0x2e (desfazer_ofertas_pelo_jogador)
- contrato +4: o clube; +0x35 & 0x3f: o tipo (5 = empréstimo); id e valor; lê (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)
- [0xae1080] o último contrato achado; ponteiro; grava (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, byte +1 (ofertas pela pessoa); valor; lê (desfazer_ofertas_pelo_jogador)
- TRANSFER_MANAGER +0x880 / +0x884 / +0x888: os percentuais da próxima venda (13 bytes), a capacidade e a conta; ponteiro e valor; grava (guardar_percentual_da_proxima_venda)
- pessoa +0x24 clube, +0x28 função, +0x3d função de origem, +0x39 contrato, +0x61 ficha de jogador; ponteiro/valor; lê e grava +0x24, +0x28, +0x3d (iniciar_diretorias_dos_clubes)
- clube +0xcf técnico; ponteiro; grava a pessoa de função 5/12, zera na segunda seleção (iniciar_diretorias_dos_clubes)
- clube +0x80 reputação; valor; lê, presa a 200 × nação +0x8e na segunda seleção (iniciar_diretorias_dos_clubes)
- clube +0xbf, +0xc3..+0xcf, +0xd3, +0x19f..+0x1db; ponteiros; zera na segunda seleção (iniciar_diretorias_dos_clubes)
- linha da diretoria [this] + 73 × i: +0 , +2..+5 marcas, +6/+8/+0xa/+0xc confianças, +0xe, +0x10..+0x16, +0x18 data (hoje), +0x20 data, +0x28, +0x2c, +0x30 reputação² × 0,25 (double), +0x38, +0x3c, +0x40, +0x41 data; valores; grava (iniciar_diretorias_dos_clubes)
- preferências do técnico (pessoa +0x69) +0xa nota da diretoria; valor; lê e baixa a 0,75 × reputação (iniciar_diretorias_dos_clubes)
- MANAGER_MANAGER +0x14: a lista das apelações (+0 id da pessoa, +4 resultado, +5 data, +0xd..+0x10 regras, +0x11..+0x18 campos do resultado); ponteiro; lê e tira (apelacoes_de_suspensao)
- partida +0x1c/+0x20 clubes, +0x28/+0x2a data, +0x3f fatia, +0x40/+0x41 marcas, +0x43; valores; lê (pode_ver_a_partida)
- [0xb64738] grupos das nações-chave; valor; lê (pode_ver_a_partida)
- pessoa +0x18, +0x3d, +0x28, +0x57, +0x59: idade, tipos do cargo no clube e na seleção, dois bytes com sinal; valor; lê (as duas decisões)
- pessoa +0x1a, +0x24, +0x39, +0x61, +0x69: nação, seleção, clube, ficha de jogador e ficha de técnico; ponteiro; lê (as duas decisões)
- pessoa +0x3e: data desde a qual está sem clube; valor; lê (aposentadoria_por_idade)
- jogador +5, +7, +0xb, +0xf, +0x14, +0x15, +0x3c: habilidade atual e potencial, a habilidade das contas, bytes da ficha; valor; lê (as duas decisões)
- contrato com o clube +0xc, +0x2f, +0x3d, +0x3e, +0x45, +0x4f: salário, ano do fim, estatuto, um byte, um campo e o nibble alto; valor; lê (as duas decisões)
- contrato +0x49: marca que exclui da lista inicial; valor; lê (lista_inicial_de_aposentadorias)
- clube +0x57 e [[0xadadfc] + id × 4]: competição e se ela roda no jogo; ponteiro e valor; lê; nação +0x11c; valor; lê
- 0xb63d60 + k × 72: +0x15 dia do fim de temporada, +0x1d um ano a mais; valor; lê (as duas decisões)
- saída (26 bytes) +0 id, +4 clube, +8 anúncio, +0x10 aposentadoria, +0x18 motivo, +0x19 zero; id e valor; grava (as duas decisões)
- [0xae1080] o contrato atual; ponteiro; grava (as duas decisões)
- [0xae2d0c] registro de 79 bytes por pessoa, +0xb bits 0x30; valor; lê (lista_inicial_de_aposentadorias)
- PLAYER_REGEN +4 lista de aposentadorias (26 bytes), +8 conta; ponteiro e valor; acrescenta e ordena (lista_inicial_de_aposentadorias)
- pessoa +0x18: idade; valor; lê (novas_aposentadorias)
- pessoa +0x39 e +0x24: clube e seleção; ponteiro; lê e grava 0 (sair_dos_cargos); lê (novas_aposentadorias)
- pessoa +0x3d e +0x28: tipos do cargo no clube e na seleção; valor; lê e grava 0 (sair_dos_cargos)
- clube +0xbf, +0xc3 (3), +0xcf, +0xd3, +0x19f (5), +0x1b3 (7), +0x1cf (3): presidente, diretores, técnico, auxiliar, treinadores, olheiros, fisioterapeutas; ponteiro; lê e grava 0 (sair_dos_cargos)
- clube +0x57 e [[0xadadfc] + id × 4]: competição e se ela roda no jogo; ponteiro e valor; lê (novas_aposentadorias)
- contrato +0x49: marca que exclui da busca; valor; lê (novas_aposentadorias)
- [0xae2d0c] registro de 79 bytes por pessoa, +0xb bits 0x30; valor; lê (novas_aposentadorias)
- PLAYER_REGEN +4 lista de aposentadorias (26 bytes, id em +0), +8 conta; ponteiro e valor; lê, acrescenta e ordena (novas_aposentadorias)
- [0xae1080] o contrato atual; ponteiro; grava (registrar_a_liberacao)
- Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_a_liberacao)
- Injury +0: índice da lesão atual por pessoa (+0xe do registro de 31 bytes, −1 sem); +4: lesões de 48 bytes (+0x20 data, +0x28 e +0x2a dias de recuperação e de lesão); id e valor; lê (estado_da_lesao)
- CONTRACT_MANAGER +0xc decisões de 14 bytes (+0 id, +4 data, +0xc fatia, +0xd tipo), +0x14 conta, +0x18 começo, +0x1c contador; valor; lê e grava (marcar_decisao_da_oferta)
- contrato (80 bytes em CONTRACT_MANAGER +0) +0x2d: data do fim; valor; lê (marcar_decisao_da_oferta)
- oferta +0x2f: marca de prazo curto; valor; lê (marcar_decisao_da_oferta)
- [0xae2c88] fatia do dia; valor; lê (marcar_decisao_da_oferta)
- clube +0xcf: técnico; ponteiro; lê; técnico +0x3e: data de chegada; valor; lê; registro do técnico +0x8e: jogos; valor; lê (candidatos_a_tecnico_do_ano)
- liga +0x3e: a conta das linhas da tabela; +0xb1 a tabela (0x3b bytes: +0 o clube, +0x11 os jogos, +0x19 os pontos); valor e ponteiro; lê (projetar_posicoes_da_liga)
- liga +0xa7: os jogos da temporada; +0xc2 os pontos por jogo da escala (byte); +0x44 a competição; valor; lê (projetar_posicoes_da_liga)
- liga +8: o Comp_Stats; ponteiro; lê (projetar_posicoes_da_liga, comparador)
- clube +0x80: a reputação; valor; lê (ordem_da_projecao_pela_reputacao)
- registro do clube no Comp_Stats +0x73..+0x75: as posições projetadas (normal, melhor, pior); valor; grava (pela virtual +0x68 da projetar_posicoes_da_liga)
- jogo (0x4f bytes) +0: a competição; +0xc o clube mandante; +0x10 o visitante; +0x34 a rodada (palavra); +0x40/+0x41 o lado do mandante e do visitante; +0x42 o tipo; id e valor; lê (as três)
- lista (LLIST) +0: a conta (palavra); +2 o começo; +6 a cauda; +0xa o cursor; valor e ponteiro; lê, anda o cursor e acrescenta (as três)
- [0xae23b4] os clubes (0x245 bytes); [0xae23d0] as competições (0x6b bytes); [0xae2a58] o calendário; ponteiro; lê (achar_jogos_do_calendario, competicao_tem_jogo_na_data)
- clube +0: o id; +0x53 a nação; id e ponteiro; lê (clube_livre_na_data, nacao_parceira_do_clube)
- nação +0: o id; +0x11c ativa (byte); id e valor; lê (clube_livre_na_data, nacao_parceira_do_clube)
- nação-chave (72 bytes em 0xb63d60) +0: a nação; +0x14 o índice; +0x2a o início da temporada; +0x39 o fim; ponteiro, valor e data; lê (clube_livre_na_data, nacao_parceira_do_clube)
- [0x9cf2e4] a Inglaterra; [0x9cf510] o País de Gales; [0xae23a8] as nações (0x122 bytes); id e ponteiro; lê (nacao_parceira_do_clube)
- copa +0x3e: a conta dos times; +0xa7 a tabela (6 bytes: +0 o clube); valor e ponteiro; lê (copa_tem_o_clube)

**Interessa a:** CMRTE, SGE, GPF2, CMScout
