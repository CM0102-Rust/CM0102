# 252ª leva — as datas de decisão e as ofertas do clube: a ordem das datas, decidir as datas vencidas e as ofertas de contrato do clube; os empréstimos e o lugar no elenco: assinar o contrato de empréstimo, encerrar o empréstimo e trocar o lugar no elenco; a diretoria e o técnico: a diretoria do técnico novo, o motivo da saída do técnico e o registro do técnico; quem serve para o cargo e a moral depois da partida ruim: o candidato a técnico, o cargo que a pessoa pode assumir e baixar a moral pela partida; a lista de alvos dos clubes: as vagas da formação, refazer a lista do clube e suspender os clubes que miram o jogador; pôr o jogador à venda, pô-lo nas listas de alvos dos clubes e pagar o prêmio por jogos da transferência; as ofertas de transferência: o clube recusa a oferta, soltar os jogadores da oferta e montar a oferta em dinheiro; as doenças e a pré-temporada: acrescentar o surto de doença no clube, espalhar os surtos pelos elencos e o jogador fora da temporada; a criação do jogo: a forma inicial dos elencos, os registros de lesão com as lesões iniciais e as listas de clubes por nação dos jovens; o fim da temporada e as transferências nos registros: o jogador do ano da torcida, arquivar as estatísticas da nação e os recordes de valor de transferência; os amistosos internacionais: o par de nações que não joga amistoso, a ordem das nações pela reputação e a dos amistosos pela data; as seleções e o calendário dos clubes: se o jogo do clube pode ser marcado com os convocados, a ordem das convocações pela data e a nação classificada para a fase final; as copas e a divisão dos clubes: a ordem dos cabeças de chave do sorteio (decrescente e crescente) e o teto da divisão dos clubes no fim da temporada

**Rotinas:**
- `comparar_datas_de_decisao()` = 0x4dc910 (contract_manager.cpp, cdecl): a ordem do qsort das datas de decisão: pela data e, no mesmo dia, pela fatia do dia (com sinal);
- `decidir_as_datas_vencidas()` = 0x4cd9c0 (contract_manager.cpp, thiscall, ret 8): consome as datas de decisão vencidas até a data e a fatia dadas: cada uma fica com −1 e, sendo do tipo 3, a pessoa decide as ofertas de contrato; em DESLIGADAS desde 30/10/2026: só com o desvio (sem rodar a cópia) a carreira no modo original saía da referência be54a138 (bissecção das 39 rotinas da 252ª): o desvio muda o resíduo de pilha que uma original lê depois; a cópia fica no código e não roda;
- `ofertas_de_contrato_do_clube()` = 0x4ca300 (contract_manager.cpp, thiscall, ret 8): copia as ofertas de contrato abertas do clube (fora dos estados 5 e 6) para um vetor novo de 111 bytes por oferta e devolve quantas;
- `assinar_contrato_de_emprestimo()` = 0x4d01e0 (contract_manager.cpp, thiscall, ret 4): passa a pessoa ao clube do empréstimo (listas, táticas, elenco como jogador), marca o reserva no contrato principal e guarda o contrato de empréstimo na vaga dela (a que tem, uma livre ou no fim do vetor); refaz as folhas;
- `encerrar_o_emprestimo()` = 0x4d07e0 (contract_manager.cpp, thiscall, ret 4): tira a pessoa do clube do empréstimo (listas, disputas, táticas, elenco, lista de observação), libera o contrato de empréstimo, refaz as folhas e a devolve ao clube dono com a marca de elenco;
- `trocar_lugar_no_elenco()` = 0x4cbd20 (contract_manager.cpp, thiscall, ret 0xc): troca a pessoa entre o principal e os reservas do clube ou entre o clube e o parceiro (time A/B), com o contrato de empréstimo pesando; 0 sem vaga no destino;
- `diretoria_do_tecnico_novo()` = 0x694c80 (manager_manager.cpp, thiscall, ret 4): recalcula as expectativas, aproxima a reputação do técnico da do clube e sorteia a confiança e os humores da diretoria a partir da reputação; zera as mudanças e as marcas, registra o técnico e guarda a faixa de riqueza; em DESLIGADAS desde 30/10/2026: com o desvio a carreira no modo Rust saía da referência be54a138 (bissecção das 39 rotinas da 252ª): a causa (resíduo de pilha ou diferença da cópia) ainda não foi isolada; a cópia fica no código e não roda;
- `motivo_da_saida_do_tecnico()` = 0x69b350 (manager_manager.cpp, thiscall, ret 4): o motivo (0 a 11) da saída do técnico: jogadores insatisfeitos, posição na liga, dívida, mudanças e humores da diretoria (com sorteios) e o tempo no cargo;
- `registro_do_tecnico()` = 0x86f5b0 (staff_records.cpp, thiscall, ret 8): acha (ou cria, ou toma de outro) o registro de 210 bytes do técnico e atualiza as nações, a passagem atual e as passagens mais longa e mais curta;
- `serve_para_o_cargo()` = 0x69f810 (manager_manager.cpp, thiscall, ret 0x10): se a pessoa é candidata realista ao cargo de técnico do clube: vetos e aceites pela reputação, nação, contrato e tempo no cargo; senão o interesse dela no cargo contra a nota mínima (double);
- `pode_assumir_o_cargo()` = 0x4cb990 (contract_manager.cpp, stdcall, ret 8): se a pessoa serve para o cargo (6 a 15) pela idade, pelo cargo atual e pelos registros de jogador e de não jogador; pessoa nula: 0;
- `baixar_moral_pela_partida()` = 0x4cb250 (contract_manager.cpp, thiscall, ret 0xc): baixa a moral do contrato do jogador depois da partida ruim (pela nota, reputação, tempo no clube, idade e faixa do contrato) e, com a moral muito baixa, pode dar a queixa de querer sair ou de falta de jogos; machucado, emprestado ou aposentando apaga as queixas;
- `montar_vagas_da_formacao()` = 0x83c090 (shortlist_manager.cpp, stdcall, ret 4): dá o papel das 40 vagas da entrada do clube (25 de jogador, 15 de comissão) e grava a máscara de posição das 25 vagas de jogador pela formação (+7);
- `refazer_lista_do_clube()` = 0x83a730 (shortlist_manager.cpp, thiscall, ret 4): solta os alvos velhos das vagas do clube, reavalia o elenco (0x83cb40) e redistribui a verba de transferências (0x83b6e0);
- `suspender_clubes_que_miram_o_jogador()` = 0x842b00 (shortlist_manager.cpp, thiscall, ret 8): põe em 100 o estado da entrada de cada clube que mira o jogador (0x842180) e, com limpar, tira a marca de lance das vagas com ele;
- `por_jogador_nas_listas_dos_clubes()` = 0x842c00 (shortlist_manager.cpp, thiscall, ret 8): com os filtros do jogador (ficha, regen marcado, cargo, fim de carreira, nação, intocável, chegada há menos de 200 dias sem estar à venda), tenta pô-lo na lista de cada clube do vetor (0x842e10) até 5 clubes, fora seleções, o clube do contrato e o dele; em DESLIGADAS desde 30/10/2026: só com o desvio (sem rodar a cópia) a carreira no modo original saía da referência be54a138 (bissecção das 39 rotinas da 252ª): o desvio muda o resíduo de pilha que uma original lê depois; a cópia fica no código e não roda;
- `por_jogador_a_venda()` = 0x842320 (shortlist_manager.cpp, thiscall, ret 8): marca o contrato à venda; o jogador pode passar a desgostar do técnico (gosta do clube ou lealdade contra dois sorteios de 100); com notícia, põe nas listas dos clubes e dá a notícia;
- `pagar_premio_por_jogos()` = 0x8cb340 (transfer_manager.cpp, thiscall, ret 0x10): acha o registro do prêmio por jogos (clube ou seleção) e, quando os jogos batem, dá a notícia, soma aos totais e o clube antigo recebe o valor que o atual paga; devolve 1 se pagou;
- `clube_recusa_a_oferta()` = 0x8c36b0 (transfer_manager.cpp, thiscall, ret 0xc): conta a tentativa da oferta; pelo lado que recusa, um sorteio(2 × tentativas) rejeita ou retira a oferta, senão ela é reagendada com a notícia de adiamento;
- `soltar_jogadores_da_oferta()` = 0x8c6ae0 (transfer_manager.cpp, thiscall, ret 4): tira o jogador da oferta e cada jogador de troca das outras ofertas e desfaz as ofertas por cada jogador de troca;
- `montar_oferta_em_dinheiro()` = 0x8c8740 (transfer_manager.cpp, thiscall, ret 0x14): aloca e inicia uma oferta em dinheiro pelo jogador com o valor dado (0x8e78f0), guarda na saída, atração 50 e devolve 1;
- `acrescentar_surto_de_doenca()` = 0x62e820 (injury.cpp, thiscall, ret 0x10): tipo 4 nunca; tipo 3 com 1 em 15, os outros com 3 em 4; fora das seleções, grava o surto (data, tipo, clube, partida) no vetor (que cresce de 30 em 30) e reordena pela data;
- `espalhar_surtos_de_doenca()` = 0x62e9e0 (injury.cpp, thiscall): até o primeiro surto depois de hoje: o de mais de cinco dias sai; os outros adoecem jogadores sorteados do elenco do clube (tipo 3 só no dia, 1 a 4; os outros 0 a 2; até 20 tentativas) pela lesão nova 0x62d250;
- `jogador_fora_da_temporada()` = 0x62ce30 (injury.cpp, stdcall, ret 4): 1 sem pessoa; 1 se o clube (sem marca +0x1db) está fora da temporada da nação e faltam menos de 40 dias, sem convocação futura da seleção; senão 0;
- `forma_inicial_dos_elencos()` = 0x62a920 (injury.cpp, thiscall): para cada clube, os dias de pré-temporada (0 dentro da temporada; até 40); a forma de cada jogador do elenco = 10000 − 75 × (dias − forma natural/2 − +0x5b/4, entre 1 e 40); quem não é jogador sai da vaga (e do clube, cargo 0xb);
- `registros_iniciais_de_lesao()` = 0x62cf70 (injury.cpp, thiscall): aloca o registro de 31 bytes de cada pessoa em forma plena; jogador: 1 em 10 de lesão recorrente do tipo sorteado e 1 em 100 de começar lesionado (0x62d140, 0x62d250); os totais em 0xde1f64;
- `listas_de_clubes_por_nacao()` = 0x7acfc0 (player_regen.cpp, thiscall): uma lista por nação; cada clube com divisão entra na da nação dele (nação com +0x11c: só com +0x80 até 2), fora as seleções e os clubes de 0x9d00a4/0x9d00a8;
- `jogador_do_ano_da_torcida()` = 0x452da0 (club_records.cpp, thiscall, ret 8): do elenco do clube, com jogos acima do mínimo (fator 0,6 caindo de 0,1 até 0,3), o de maior nota (média + 0,025 × +2 + 1/60 × +6, gostos do clube e do jogador); a referência dele em +0x371 do registro e 1, ou 0;
- `arquivar_estatisticas_da_nacao()` = 0x7c1350 (player_stats.cpp, thiscall, ret 4): para os clubes da nação (até a primeira seleção), arquiva (0x7c5fe0) as estatísticas de cada jogador do elenco que não está emprestado a outro clube; 1 (0 e a caixa "Error" sem nação);
- `recordes_de_valor_de_transferencia()` = 0x448760 (club_records.cpp, thiscall, ret 4): o recorde pago do comprador e o recebido do vendedor, da temporada e de sempre (valor, referência, outro clube, data, empréstimo); o de sempre acima de 1000 × (reputação + 300) vai à fila de notícias e, na compra, o técnico passa a gostar do jogador;
- `par_de_nacoes_sem_amistoso()` = 0x5cc3e0 (friendly.cpp, thiscall, ret 8): devolve 1 quando (a, b), (a, −1) ou (−1, b) está nos pares proibidos (+0x3502, conta +0x3506), senão 0;
- `ordem_das_nacoes_pela_reputacao()` = 0x5cc5f0 (friendly.cpp, cdecl): comparador do qsort dos ponteiros de nação pela reputação (+0x8e), a maior primeiro;
- `ordem_dos_amistosos_pela_data()` = 0x5cc610 (friendly.cpp, cdecl): comparador do qsort dos registros de amistoso pela data em +3 (0x456770/0x456740), a mais cedo primeiro;
- `pode_marcar_jogo_do_clube()` = 0x776fc0 (national_teams.cpp, thiscall, ret 4): jogo amistoso: 1; senão 0 quando o mandante ou o visitante (clube comum) tem mais de 2 convocados entre um dia antes e um depois da data do jogo, senão 1; em DESLIGADAS desde 30/10/2026: só com o desvio (sem rodar a cópia) a carreira no modo original saía da referência be54a138 (bissecção das 39 rotinas da 252ª): o desvio muda o resíduo de pilha que uma original lê depois; a cópia fica no código e não roda;
- `ordem_das_convocacoes_pela_data()` = 0x778e90 (national_teams.cpp, cdecl): comparador do qsort dos registros pela data em +0 (0x456770/0x456740), a mais cedo primeiro;
- `acrescentar_nacao_classificada()` = 0x7779b0 (national_teams.cpp, thiscall, ret 8): com a nação e a fase final não nulas, acrescenta o par à lista das novas classificadas (+0x28);
- `ordem_dos_cabecas_de_chave_decrescente()` = 0x522190 (Cup.cpp, cdecl): comparador do qsort das entradas da copa pelo nível (+4), o maior primeiro; no empate pela reputação do clube (+0x80), a menor primeiro;
- `ordem_dos_cabecas_de_chave_crescente()` = 0x5221c0 (Cup.cpp, cdecl): comparador do qsort das entradas da copa pelo nível (+4), o menor primeiro; no empate pela reputação do clube (+0x80), a menor primeiro;
- `teto_da_divisao_dos_clubes()` = 0x68a980 (league.cpp, thiscall, ret 0x10): para cada linha da tabela, leva o byte +0x64 do clube ao valor (para baixo ou para cima) quando a situação da linha não é a de fora e quando é a de dentro (0xfd = nenhuma);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (29):** 25 acusadas. `cpp_a_venda` escapou: quem lê se o jogador está à venda é o bit 1 do +0x4e do contrato (0x5444a0), que a mutação mantém; o nibble alto do +0x4f (7 = à venda) não é lido por nada que a carreira automática percorre [inferência]. `cpp_soltar` escapou: o ramo rodou, mas o jogador da oferta concluída não tinha outra oferta aberta que mudasse a carreira: as que sobram caem depois pelo fim da transferência [inferência]; o comparando também não viu diferença. `cpp10_compra_grande` escapou: nenhuma compra da carreira caiu entre 1000 × (reputação + 200) e 1000 × (reputação + 300), a faixa que a mutação acrescenta; o comparador usa a mesma regra no plano e não acusa por construção. `cpp10_gostar_forcado` escapou: forçar só importa quando o técnico comprador não tem vaga livre na lista de preferidos (0x547a10 sorteia quem sai); com as DESLIGADAS da 252ª isso não aconteceu até 2002-06-01 (na primeira cadeia, antes delas, a mutação foi acusada em 2002-002). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (2f65fd6b68d1027b).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (be54a1386f51073a).

**Refazer**, sobre o commit da 251ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 252`

**Campos tocados:**
- CONTRACT_MANAGER +0xc, +0x14, +0x18: o vetor das datas de decisão (14 bytes: +0 id da pessoa ou −1, +4 data, +0xc fatia, +0xd tipo), a conta e o próximo índice; ponteiro e valores; lê e grava −1 e o índice (decidir_as_datas_vencidas), lê (comparar_datas_de_decisao)
- CONTRACT_MANAGER +0x34: a lista das ofertas de contrato (111 bytes: +4 id do clube, +0x35 estado nos 6 bits baixos); ponteiro; lê (ofertas_de_contrato_do_clube)
- [0xae2d0c] registro de 79 bytes da pessoa +0, +4: os contratos principal e de empréstimo; índice; lê e grava +4 (as três)
- CONTRACT_MANAGER +0, +4, +8: o vetor dos contratos (0x50 bytes), a capacidade e a conta; ponteiro e valores; lê e grava (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)
- contrato +0x35 bit 0x40: o reserva; valor; lê e grava (as três)
- [0xae1080] a ficha global do contrato; ponteiro; grava (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)
- pessoa +0x39 (clube atual), +0x3e (data de entrada), +0x5e (marcas de elenco), +0x6d (situação, 0x10 vira 4); ponteiro e valores; lê e grava (encerrar_o_emprestimo), lê +0x39/+0x5e e devolve +0x3e (assinar_contrato_de_emprestimo)
- MANAGER_MANAGER [this] linha da diretoria (0x49 bytes por clube) +1, +2, +3, +4, +5, +6, +8, +0xa, +0xc, +0x10..+0x16, +0x28, +0x30, +0x38, +0x40; valores; grava (diretoria_do_tecnico_novo), lê +2, +6..+0x16, +0x30, +0x38 (motivo_da_saida_do_tecnico)
- ficha de não jogador +8, +0xa, +0xc: as reputações doméstica, atual e de outra nação; valor; lê e grava (diretoria_do_tecnico_novo)
- Shortlist_Manager +0xdc (9 bytes por clube) +4; valor; grava 2 (diretoria_do_tecnico_novo)
- contrato +0x45 bit 0x10000000: a queixa; valor; lê (motivo_da_saida_do_tecnico)
- Staff_Records +8: os registros de técnico (210 bytes: +0x4a..+0x7e as passagens, +0x86 dono, +0xba/+0xbe pontos, +0xc2/+0xc6 nações); ponteiro; lê e grava (registro_do_tecnico)
- [0xdd6ec8] a data a partir da qual as passagens contam; valor; lê (registro_do_tecnico)
- pessoa +0x18 (idade), +0x1a (nação), +0x22, +0x24/+0x28 (seleção e cargo nela), +0x39 (clube), +0x3d (cargo), +0x3e/+0x40 (data de entrada), +0x57, +0x59, +0x61 (jogador), +0x65, +0x69 (não jogador); valores e ponteiros; lê (as três)
- contrato +0x2d/+0x2f (data), +0x3d, +0x3e, +0x3f, +0x44 (moral), +0x45 (queixas), +0x49, +0x4f (faixa); valores; lê e grava +0x44 (baixar_moral_pela_partida), lê (serve_para_o_cargo)
- clube +0x53 (nação, +0x85 o peso), +0x57 (liga, +0x69 o nível), +0x80 (reputação), +0xcf/+0xd3; lê (serve_para_o_cargo, baixar_moral_pela_partida)
- [0xb63d8a] tabela das janelas de transferência (72 bytes por nação-chave, as datas em +0 e +0xf); lê (baixar_moral_pela_partida)
- [0xae1080] a ficha global do contrato; ponteiro; grava (baixar_moral_pela_partida)
- entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; id × 9) +0: as 40 vagas de 0x25 bytes; ponteiro; lê (as três)
- entrada do clube +7: a formação (1..9; fora, a padrão); valor; lê (montar_vagas_da_formacao)
- entrada do clube +8: o estado (100 = suspenso para o jogador); valor; grava 100 (suspender_clubes_que_miram_o_jogador)
- vaga de 0x25 bytes +0: o papel (0xb jogador; 6, 7, 8, 10, 9 comissão); valor; grava (montar_vagas_da_formacao)
- vaga +1: a máscara de posição (16 bits); valor; grava (montar_vagas_da_formacao)
- vaga +0x24: bits 2..4 = 0 e bit 1 nas vagas de comissão; valor; lê e grava (montar_vagas_da_formacao)
- vaga +0xf: o id do jogador alvo; id; lê (suspender_clubes_que_miram_o_jogador)
- vaga +0x17: a marca de lance (−1 sem); valor; lê e grava −1 (suspender_clubes_que_miram_o_jogador)
- jogador +0x61: a ficha de posições (+0xb nota, 16 bits com sinal); ponteiro; lê (por_jogador_nas_listas_dos_clubes)
- [0xae2d0c] registro de 79 bytes +0xb: bits 0x30 (0x10 marca) e o nibble baixo (quantos clubes já o têm na lista); valor; lê (por_jogador_nas_listas_dos_clubes)
- jogador +0x3d, +0x24, +0x28, +0x1a, +0x39, +0x3e/+0x40: cargo, fim de carreira, nação, clube, data de chegada; valor e ponteiro; lê (por_jogador_nas_listas_dos_clubes)
- [0xae2d1c] e [0xae235c]: o vetor dos ids dos clubes e a conta; id; lê (por_jogador_nas_listas_dos_clubes)
- contrato +0x4e bit 1 e +0x4f nibble alto (7 = à venda); valor; lê e grava (por_jogador_a_venda), lê (por_jogador_nas_listas_dos_clubes)
- contrato +0x44: a lealdade (byte com sinal); valor; lê (por_jogador_a_venda); +0x45; valor; lê (por_jogador_nas_listas_dos_clubes)
- [0xae1080] o último contrato achado; ponteiro; grava (por_jogador_nas_listas_dos_clubes, pagar_premio_por_jogos)
- TRANSFER_MANAGER +0x88c/+0x894 e +0x898/+0x8a0: os registros de prêmio por jogos (0xd bytes: +0 pessoa, +4 valor, +8 jogos, +9 clube antigo) de clube e de seleção, e as contas; ponteiro e valor; lê (pagar_premio_por_jogos)
- oferta (0x8e5680, 0x41 bytes) +0x2c: o estado (0x11 recusada pelo outro lado, 0x12 pelo vendedor); valor; lê e grava (clube_recusa_a_oferta)
- oferta +0x2e: as tentativas; valor; lê e grava (clube_recusa_a_oferta)
- oferta +8 e +0xc: o id do clube vendedor e o do jogador; id; lê (clube_recusa_a_oferta, soltar_jogadores_da_oferta)
- oferta +4: a lista dos jogadores de troca (itens: id); ponteiro; lê (soltar_jogadores_da_oferta)
- oferta +0x1b: o valor; valor; grava pela 0x8e78f0 (montar_oferta_em_dinheiro)
- contrato +0x35 (& 0x3f; 5 fora); valor; lê (clube_recusa_a_oferta); [0xae1080] o último contrato; ponteiro; grava
- Injury +0x1c, +0x20, +0x24: o vetor de surtos (17 bytes), a capacidade e a conta; ponteiro e valor; lê e grava (acrescentar_surto_de_doenca, espalhar_surtos_de_doenca)
- surto +0, +8, +9, +0xd: a data, o tipo da doença, o índice do clube e o id da partida (−1 sem); valor e id; grava (acrescentar_surto_de_doenca), lê (espalhar_surtos_de_doenca)
- Injury +0: os registros de 31 bytes por jogador; +0x1d a doença atual; ponteiro e valor; lê (espalhar_surtos_de_doenca)
- clube +0xd7: as 50 vagas do elenco; ponteiro; lê (espalhar_surtos_de_doenca)
- clube +0x1db: marca (−1 = sem); valor; lê (jogador_fora_da_temporada)
- pessoa +0x24, +0x39, +0x5e (bit 3): seleção, clube e a classe da convocação; ponteiro e valor; lê (jogador_fora_da_temporada)
- [0xae2c90] a data do jogo; valor; lê
- Injury +0: os registros de 31 bytes por pessoa ([0xae2364]); ponteiro; grava (registros_iniciais_de_lesao), lê (forma_inicial_dos_elencos)
- registro de lesão +0, +8, +0xa, +0xc, +0xe, +0x12, +0x14, +0x18, +0x1d, +0x1e: data, forma, condição, desgaste, lesão, recorrente, histórico, doença, 50; valor; grava (registros_iniciais_de_lesao; +8 forma_inicial_dos_elencos)
- jogador +0x34: forma natural; valor com sinal; lê (forma_inicial_dos_elencos)
- pessoa +0x5b, +0x61, +0x39, +0x3d: atributo da pessoa, jogador, clube e cargo; valor e ponteiro; lê (as três), grava +0x39/+0x3d (forma_inicial_dos_elencos)
- clube +0xd7: as 50 vagas do elenco; ponteiro; lê e grava 0 (forma_inicial_dos_elencos)
- clube +0x53, +0x57, +0x80: nação, divisão e reputação; ponteiro e valor; lê (listas_de_clubes_por_nacao)
- nação +0x71, +0x11c: as marcas que põem os clubes na lista; valor; lê (listas_de_clubes_por_nacao)
- PLAYER_REGEN +0: as listas de clubes por nação (id da nação); ponteiro; grava (listas_de_clubes_por_nacao)
- Club_Records +0x1c: os registros de 106 bytes por clube; +0x10 os jogos (com sinal); ponteiro e valor; lê (jogador_do_ano_da_torcida)
- registro de recordes +0x371: a referência do jogador do ano da torcida (0x18 bytes); valor e id; grava (jogador_do_ano_da_torcida)
- registro de recordes +0x313..+0x33b e +0x33c..+0x364: recorde pago e recebido (valor, referência, clube, data, empréstimo); valor e id; lê e grava (recordes_de_valor_de_transferencia)
- Club_Records +0x24, +0x2c: a fila de notícias de recorde (6 bytes: id do clube, tipo) e a conta; ponteiro e valor; lê (recordes_de_valor_de_transferencia)
- estatística (0x7c3920, categoria 6) +0, +2, +6, +0xe: jogos, +2, +6 e a soma das notas; valor; lê (jogador_do_ano_da_torcida)
- transferência +4, +8, +0xc, +0x1b: empréstimo, índice do comprador, id da pessoa e valor; valor e id; lê (recordes_de_valor_de_transferencia)
- clube +0x80, +0xbf, +0xd7: reputação, técnico e as 50 vagas; valor e ponteiro; lê
- [0xae1080] o contrato da última transferência; ponteiro; grava (recordes_de_valor_de_transferencia)
- amistosos +0x3502: o vetor dos pares proibidos (2 ids de nação por par, −1 = qualquer); ponteiro; lê; +0x3506 a conta (byte com sinal); valor; lê (par_de_nacoes_sem_amistoso)
- nação +0x8e: a reputação; valor; lê (ordem_das_nacoes_pela_reputacao)
- registro de amistoso (20 bytes) +3: a data (dia, ano); valor; lê (ordem_dos_amistosos_pela_data)
- jogo +0x28/+0x2a: a data (ano, dia); +0x14 a competição; +0x1c, +0x20 mandante e visitante; valor, id e ponteiro; lê (pode_marcar_jogo_do_clube)
- NationalTeams +0x20: as convocações por clube (6 bytes pelo id: +0 registros de 9 bytes, +5 a conta); ponteiro; lê (pode_marcar_jogo_do_clube)
- convocação (9 bytes) +0: a data; +8 quantos convocados; valor; lê (pode_marcar_jogo_do_clube, ordem_das_convocacoes_pela_data)
- [0xae235c] clubes, [0xae2350] nações: o limite dos clubes comuns (sem as seleções e as sub-21); valor; lê (pode_marcar_jogo_do_clube)
- NationalTeams +0x28: a lista (LLIST) das novas classificadas (nação, fase final); ponteiro; grava (acrescentar_nacao_classificada)
- entrada de copa +0: o clube; ponteiro; +4 o nível; valor; lê (as duas ordens)
- clube +0x80: a reputação; valor; lê (as duas ordens)
- liga +0x3e: a conta das linhas da tabela; +0xb1 a tabela (0x3b bytes: +0 o clube, +0x37 a situação no fim da temporada); valor e ponteiro; lê (teto_da_divisao_dos_clubes)
- clube +0x64: o teto da divisão (byte com sinal); valor; lê e grava (teto_da_divisao_dos_clubes)

**Interessa a:** CMRTE, SGE, GPF2, CMScout
