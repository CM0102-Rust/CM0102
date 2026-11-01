# 263ª leva — a rodada da copa: o prêmio da rodada, os mandantes da rodada e o progresso do time; os amistosos dos clubes: a resposta ao pedido de amistoso, o pedido enviado e a lista das nações para os amistosos internacionais; o sorteio da rodada da copa, o nome da fase e da rodada e o clube da divisão mais alta; a copa pelo desfecho e no fim da temporada: os registros pelo desfecho, as honras da temporada e os semifinalistas para a UEFA; a vaga no elenco para a oferta (o limite de 50 jogadores e das vagas da comissão, com a liberação do mais barato), a oferta de empréstimo recusada pelo clube e a cláusula de percentual da próxima venda achada; a lista de observação do técnico humano: o jogador na busca, as listas dos olheiros acertadas com os olheiros do clube e a vaga do jogador no registro do clube observado; o valor do jogador para o clube (a nota pela vaga da lista de observação e o preço que o clube pagaria), a entrada do jogador na busca do clube observado e o prêmio por jogos na seleção achado; os contratos depois do jogo (os bônus, a felicidade, a moral e a evolução de quem ficou de fora), a moral da ficha e o texto dela; o valor do jogador, a comparação da moral de dois jogadores e as marcas de elenco da tela do clube; a diretoria e os jogadores depois do jogo: os jogadores que ela cita (com a confiança), o veredito da nota na partida e o texto dela; os registros dos técnicos: o resultado de cada jogo, a posição no fim da liga e o jogador do ano da torcida; os registros das copas e da troca de técnico: o vice, o terceiro e a passagem do técnico pelo clube; as reações dos elencos à transferência: os contratos dos dois elencos, a comemoração da contratação e o companheiro desassossegado pela venda

**Rotinas:**
- `copa_premio_da_rodada()` = 0x522240 (Cup.cpp, thiscall, ret 0x10): paga o prêmio da rodada (+0x60 ao vencedor, +0x64 ao perdedor; com −1, do fundo +0x28 pelas rodadas que faltam e pelos confrontos, 3/4 e 1/4 quando os dois são −1): o vencedor com a notícia (0x48e3e0) e o dinheiro (0x5999a0), o perdedor só o dinheiro; o fundo perde os dois;
- `copa_mandantes_da_rodada()` = 0x51e870 (Cup.cpp, thiscall, ret 8): inverte mandante e visitante num confronto ou em todos os da rodada pela regra da rodada: o clube de divisão mais alta (virtual +0x90), o nível do time ou o estádio da copa;
- `copa_progresso_do_time()` = 0x521b30 (Cup.cpp, thiscall, ret 8): quanto o time foi na copa, pelo desfecho e pela rodada (com ou sem a rodada do terceiro lugar), entregue à virtual +0x68 (set_club_progress) com a competição +0x44;
- `amistoso_responder_pedido()` = 0x5c5f40 (friendly.cpp, thiscall, ret 8): o convidado (sem técnico humano, ou com forcar) aceita ou recusa pela reputação, com o fator das nações, regiões, continentes e da distância das cidades dos estádios; avisa quem pede (notícia 7); o aceito vai ao calendário (0x5aa680) e marca a vaga do amistoso; com 3 ou 4 o pedido sai (0x5c87b0); noutra situação, só sai da lista das respostas;
- `amistoso_enviar_pedido()` = 0x5c5570 (friendly.cpp, thiscall, ret 4): guarda o pedido na lista das respostas (+0x34d4, criada se falta); convidado de técnico humano: situação 2 e a notícia 6;
- `amistoso_lista_das_nacoes()` = 0x5ca3e0 (friendly.cpp, thiscall, ret 0): aloca a lista (+0x34fa) das nações com continente, conta em +0x34fe e ordena pela reputação (0x5cc5f0);
- `copa_sortear_rodada()` = 0x51c410 (Cup.cpp, thiscall, ret 4): com algum time em aberto adia o sorteio 1 dia (e as rodadas seguintes para no mínimo essa data); senão sorteia pela regra (semente 0x51d2d0, nações separadas 0x51d9f0 ou livre 0x51e220), acerta os mandantes (0x51e870) e o estádio compartilhado (0x522610), marca os confrontos sorteados e manda a notícia do sorteio aos times de técnico humano; devolve 1;
- `nome_da_fase_e_rodada()` = 0x4bffc0 (comp_util.cpp, cdecl): escreve o nome da fase e o da rodada (com os gêneros, pelo modelo 0x9a7cfc), ou só o de uma delas; sem saída ou com menos de 200 bytes, a caixa e 0;
- `clube_da_divisao_mais_alta()` = 0x522360 (Cup.cpp, thiscall, ret 8): devolve o clube de nível de divisão menor (+0x57 → +0x69), ou 0 no empate;
- `copa_registros_pelo_desfecho()` = 0x519890 (Cup.cpp, thiscall, ret 0x18): pelo desfecho do clube: o registro de campeão (e a vaga na Copa da UEFA pela Copa da Liga inglesa ou pela copa principal de uma nação europeia), de vice, de terceiro, a classificação para a rodada seguinte ou a eliminação, com o peso pela rodada; devolve 0;
- `copa_honras_da_temporada()` = 0x519a90 (Cup.cpp, thiscall, ret 0): acha o campeão, o vice, o terceiro (na copa galesa, pela estatística) e a seleção anfitriã, grava a linha do histórico e, na copa principal da nação, passa os semifinalistas à UEFA;
- `copa_semifinalistas_para_a_uefa()` = 0x519fc0 (Cup.cpp, thiscall, ret 0): acha as semifinais (0x82) e a final (0x96) e manda à semeadura da UEFA os dois semifinalistas que não estão na final;
- `recusar_oferta_de_emprestimo()` = 0x8bfa30 (transfer_manager.cpp, thiscall, ret 0xc): o clube recusa a oferta de empréstimo: estado 2, os lances desmarcados, a notícia, a oferta volta em 10 dias para sair, a marca de oferta da pessoa sai sem rival em aberto, as ofertas de contrato fecham e a situação na lista de observação do comprador;
- `vaga_no_elenco_para_a_oferta()` = 0x8cd450 (transfer_manager.cpp, thiscall, ret 0x10): confere se a oferta deixa o clube (com o parceiro) dentro de 50 jogadores e das vagas de treinadores (5), fisioterapeutas (3), olheiros (7) e auxiliar; o comprador do computador sem troca libera de graça o jogador mais barato que pode sair; senão o motivo 4 ou 0x22 (mais 1 pelo vendedor) e 0;
- `percentual_da_proxima_venda_achado()` = 0x8d0dd0 (transfer_manager.cpp, thiscall, ret 4): com a marca 8 da pessoa, procura pela busca binária a cláusula de percentual da próxima venda da pessoa com o clube do contrato; sem achar, tira a marca;
- `jogador_na_busca()` = 0x7af600 (player_search.cpp, thiscall, ret 4): procura o jogador nos resultados da busca (busca binária pelo id) e devolve 1 se achado e não marcado como removido;
- `sincronizar_listas_dos_olheiros()` = 0x7f7270 (scout_manager.cpp, thiscall, ret 4): tira das listas de olheiro do técnico os olheiros que não são mais do clube e dá a cada olheiro do clube sem lista a primeira lista livre; com a edição aberta, ela refaz os olheiros;
- `vaga_do_jogador_observado()` = 0x7bd700 (player_search.cpp, thiscall, ret 8): a vaga do jogador entre as 50 do registro do clube observado; reservando, a primeira livre recebe o id e zera os contadores;
- `valor_do_jogador_para_o_clube()` = 0x840d80 (shortlist_manager.cpp, thiscall, ret 0x18): a nota do jogador para a vaga (pelo melhor observador do clube, pela lista do olheiro no clube humano, pela névoa) e o preço que o clube pagaria: o valor base, a idade, a habilidade vista dos jovens, a consistência, os tetos pela posição, pela reputação, pela ficha, pela idade, pela divisão e pela nação do clube;
- `entrada_do_jogador_na_busca_do_clube()` = 0x7f8610 (scout_manager.cpp, thiscall, ret 4): a vaga do jogador no registro da busca do humano que observa o clube dele, ou 0;
- `premio_de_jogos_na_selecao_achado()` = 0x8d0f00 (transfer_manager.cpp, thiscall, ret 4): com a marca 4 da pessoa, procura pela busca binária a cláusula de prêmio por jogos na seleção; sem achar, tira a marca;
- `contratos_depois_do_jogo()` = 0x4ca6d0 (contract_manager.cpp, thiscall): depois de cada jogo: os bônus dos contratos, a felicidade pelos minutos, a moral pelo sorteio e pelo resultado, e a evolução e a felicidade de quem ficou de fora;
- `moral_da_ficha()` = 0x4cc2d0 (contract_manager.cpp, stdcall): a moral da ficha do jogador (+0x45), pelas telas;
- `texto_da_moral()` = 0x4cc2e0 (contract_manager.cpp, stdcall): a palavra da moral (de "Very Low" a "Superb") num texto, pelas telas;
- `valor_do_jogador()` = 0x4d0c60 (contract_manager.cpp, thiscall): o valor do jogador pelo auge e pela idade, pelas notas da ficha, pela habilidade vista pelo técnico do clube, pela nação do clube, pelos dias de contrato e pela cláusula de rescisão;
- `diferenca_de_moral_em_texto()` = 0x4cc3a0 (contract_manager.cpp, stdcall): a diferença de moral de dois jogadores em palavra ("Same", "Marginal", "Better", "Superior"), pela tela de comparação;
- `marcas_de_elenco_para_a_tela()` = 0x4cc130 (contract_manager.cpp, thiscall): as marcas de elenco que a tela do clube mostra, pelo bit de reserva do contrato quando o jogador está emprestado a outro clube;
- `jogadores_que_a_diretoria_cita()` = 0x6aaee0 (manager_manager.cpp, thiscall): depois do jogo, por sorteio, a posição na liga para a notícia e os jogadores preferidos ou rejeitados do técnico que jogaram mal, com a confiança da diretoria subindo ou caindo por eles;
- `veredito_da_nota_na_partida()` = 0x6ab670 (manager_manager.cpp, thiscall): acha o jogador entre os titulares do registro da partida: −1 com nota abaixo de 7, 1 acima, senão 0;
- `texto_da_diretoria_sobre_os_jogadores()` = 0x6ab2d0 (manager_manager.cpp, stdcall): a frase da diretoria elogiando ou culpando o jogador citado, pela variante;
- `resultado_nos_registros_dos_tecnicos()` = 0x868830 (staff_records.cpp, thiscall): nos registros dos dois técnicos: o jogo, os gols, a vitória, o empate ou a derrota e a nota do jogo (importância × reputação do adversário × peso × pontos) no mês, no ano, na liga do clube e na temporada do clube ou da seleção; o visitante recebe na temporada a nota do mandante (defeito da original);
- `posicoes_no_fim_da_liga()` = 0x86a070 (staff_records.cpp, thiscall): no fim da liga, a posição final de cada clube no histórico do técnico dele;
- `jogador_do_ano_da_torcida_no_registro()` = 0x86dc30 (staff_records.cpp, thiscall): o jogador do ano da torcida no histórico dele e, pelo sorteio, o vínculo de favoritos entre ele e o clube;
- `vice_da_copa_nos_registros()` = 0x86b0b0 (staff_records.cpp, thiscall): o vice da copa: a reputação do clube sobe até peso × 500 (no máximo 350 de uma vez), as três notas do lado dele na partida pelo peso, e os históricos do técnico, do presidente, da escalação e do clube;
- `terceiro_da_copa_nos_registros()` = 0x86b710 (staff_records.cpp, thiscall): o terceiro da copa: o mesmo com peso × 200 (no máximo 200) e notas menores; acaba as convocações nas competições guardadas;
- `troca_de_tecnico_nos_registros()` = 0x869aa0 (staff_records.cpp, thiscall): fecha a passagem atual do técnico (a mais curta e a mais longa), abre a nova quando contratado e conta as passagens; os históricos do técnico, do presidente e do clube;
- `reacoes_dos_elencos_a_transferencia()` = 0x758200 (media.cpp, thiscall): depois da transferência de um jogador de 19 anos ou mais com reputação 4250: no elenco novo, as marcas e queixas dos contratos pela posição e pela força do elenco, a torcida (+500) e a comemoração da contratação de um craque; no elenco antigo, as queixas pela saída do companheiro, a moral −25, as marcas tiradas e a notícia do companheiro desassossegado;
- `noticia_da_comemoracao_da_contratacao()` = 0x75b6e0 (media.cpp, thiscall): a notícia 0xbe5 da torcida ou do porta-voz do elenco comemorando a contratação, ao técnico do clube;
- `noticia_do_companheiro_desassossegado()` = 0x75cfc0 (media.cpp, thiscall): a notícia 0xbe4 do companheiro insatisfeito com a venda, ao técnico do clube antigo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (33):** 31 acusadas. `cqa2_sem_o_fator_da_distancia` escapou: o fator da distância só pesa na nota do convite, e nos 256 convites com cidade do estádio a decisão (aceita ou recusa) saiu a mesma; a comparação rotina a rotina também não acusou [inferência]. `cqa3_sorteio_adiado_dois_dias` escapou: o adiamento só roda com um confronto anterior em aberto (4 vezes, replays da FA Cup), e sortear dois dias depois não mudou os confrontos nem o consumo do sorteio até 2002-06-01 [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (4307c8ab).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6299f556, original da4329bf (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 262ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 263`

**Campos tocados:**
- Cup +0x28: o fundo de prêmios; valor; lê e grava (copa_premio_da_rodada)
- Cup +0x3c / +0xa3: as rodadas e a tabela delas (0x68 bytes: +7 tipo, +0xb regra, +0x1a confrontos, +0x5b progresso, +0x60/+0x64 prêmios); valor e ponteiro; lê (as três)
- Cup +0xa7: os times (6 bytes: +0 clube, +4 nível, +5 desfecho); ponteiro; lê (as três)
- Cup +0xad: os confrontos de cada rodada (0x35 bytes: +0/+2 times, +0xa/+0xc, +0xe/+0x10); ponteiro; lê e grava (copa_mandantes_da_rodada)
- Cup +0x24 / +0x44: o estádio da copa (o id, comparado com o do estádio +0x69 do clube) e a competição passada à +0x68; id; lê (copa_mandantes_da_rodada, copa_progresso_do_time)
- partida +0x30: a rodada da partida (0x136 troca o desfecho 1 por 3 na notícia); valor; lê (copa_premio_da_rodada)
- finanças do clube (0x5a0590): saldo, +0xa8, +0x148; valor; soma pela 0x5999a0 (copa_premio_da_rodada)
- pedido de amistoso (0x19 bytes): +0/+2 dia e ano, +0xc quem pede, +0x10/+0x14 os clubes do jogo, +0x18 situação (1 pendente, 2 com o humano, 3 aceito, 4 recusado); lê, grava +0x18 (amistoso_responder_pedido, amistoso_enviar_pedido)
- clube +0x53 / +0x69 / +0x77 / +0x80: a nação, o estádio (+0x38 a cidade, +0x23/+0x2b as coordenadas), o inteiro +0x77 e a reputação; ponteiro e valor; lê (amistoso_responder_pedido)
- nação +0x71 / +0x76: o continente e a região; ponteiro e valor; lê (amistoso_responder_pedido, amistoso_lista_das_nacoes)
- Friendly +0x34d4: a lista das respostas dos amistosos; ponteiro; lê e grava (amistoso_responder_pedido, amistoso_enviar_pedido)
- Friendly +0x34fa / +0x34fe: a lista das nações e a conta; ponteiro e valor; grava (amistoso_lista_das_nacoes)
- vaga do amistoso marcado (0x5c6740) +0x10 / +0x18: o adversário e a marca; ponteiro e valor; grava (amistoso_responder_pedido)
- [0xae2c92] o ano de hoje; [0xae2350] / [0xae23a8] a conta e a tabela das nações (0x122 bytes); valor e ponteiro; lê
- Cup +0x45 / +0x3c / +0x40 / +0x3a / +0x44 / +0xab / +0xac: a marca de pronta, as rodadas, o ano base, o id, a competição, a rodada atual e a conta das notícias de sorteio; valor; lê, grava +0xac (copa_sortear_rodada)
- rodada da copa (0x68 bytes) +0 / +2 / +7 / +0xb / +0xd / +0x1a / +0x1c / +0x21: a data do sorteio (dia e ano), o tipo, as regras, os confrontos, as entradas e o estádio; valor; lê, grava a data (copa_sortear_rodada)
- confronto da copa (0x35 bytes) +0 / +2 / +0x34: os times e a marca de sorteado; valor; lê, grava +0x34 (copa_sortear_rodada)
- clube +0x57 → divisão +0x69: o nível da divisão; ponteiro e valor; lê (clube_da_divisao_mais_alta)
- Cup +0x2c / +0x44 / +0x3c / +0x3e / +0x40: as fases, a competição, as rodadas, os times e o ano; valor; lê (as três)
- Cup +0xa3 / +0xa7 / +0xad: as rodadas (+7 tipo, +0x1a confrontos), os times (+5 desfecho) e os confrontos; ponteiro; lê (as três)
- estado do clube [0xae2d14] + id × 6: a competição europeia da temporada; id; lê e grava a chave da Copa da UEFA (copa_registros_pelo_desfecho)
- jogo +0x30 / +0x32 / +0x34 / +0x3a: as palavras da rodada e a marca; valor; lê (copa_registros_pelo_desfecho)
- histórico de competições [0xae23d4] (26 bytes por linha): +4 competição, +8 ano, +0xa..+0x16 os quatro clubes; grava pela 0x4afce0 (copa_honras_da_temporada)
- semeadura da UEFA [0xde1f58] +0x440: 50 vagas de 12 bytes (competição, os dois clubes); grava pela 0x905790 (copa_semifinalistas_para_a_uefa)
- oferta +4 / +8 / +0xc / +0x11: a troca (LLIST), o comprador, a pessoa e o tipo da oferta; id e valor; lê (vaga_no_elenco_para_a_oferta, recusar_oferta_de_emprestimo)
- oferta +0x2c / +0x2d / +0x10: o estado (2), a marca e a etapa; valor; grava (recusar_oferta_de_emprestimo)
- clube +0xd7: os 50 do elenco; ponteiro; lê (vaga_no_elenco_para_a_oferta)
- clube +0x19f / +0x1b3 / +0x1cf / +0xd3: treinadores (5), olheiros (7), fisioterapeutas (3) e auxiliar; ponteiro; lê (vaga_no_elenco_para_a_oferta)
- pessoa +0x3d / +0x52: o tipo e o valor; valor; lê (vaga_no_elenco_para_a_oferta)
- contrato +0x3d / +0x35: o estatuto no elenco (com sinal) e o tipo (5 = empréstimo); valor; lê (vaga_no_elenco_para_a_oferta)
- [this+0x86e] + id × 8: as marcas da pessoa (bit 0 oferta em andamento, bit 3 percentual da próxima venda); valor; lê e grava (recusar_oferta_de_emprestimo, percentual_da_proxima_venda_achado)
- [this+0x880] / [this+0x888]: as cláusulas de percentual da próxima venda (0xd bytes: +1 clube, +9 pessoa) e a conta; ponteiro e valor; lê (percentual_da_proxima_venda_achado)
- [0xde1f54] + 0x92c: a fila da transferência livre (LLIST de ids); id; lê (vaga_no_elenco_para_a_oferta)
- [0xae1080]: o último contrato consultado; ponteiro; grava (as três)
- busca +0x1ee / +0x1f6: os resultados (0x2d bytes: +0xc o id, +0x20 bit 0 removido) e a conta; ponteiro e valor; lê (jogador_na_busca)
- busca +0xa: o tipo da lista (4 = olheiro); valor; lê (sincronizar_listas_dos_olheiros)
- busca +0x174..+0x195: o olheiro (ids, índice +0x180, data) e as missões; id e valor; grava pela 0x7ae8a0 (sincronizar_listas_dos_olheiros)
- pessoa +0x39 / +0x3d: o clube e o tipo (9 = olheiro); ponteiro e valor; lê (sincronizar_listas_dos_olheiros)
- [gerência de olheiros + vaga × 4 + 0xc]: a edição aberta do técnico; ponteiro; lê (sincronizar_listas_dos_olheiros)
- registro do clube observado +4: 50 vagas de 0x1a2 bytes (+0 id, −1 livre; +4 palavra, +0x1a0, +0x1a1 contadores); id e valor; lê e grava (vaga_do_jogador_observado)
- vaga da lista +1 / +7 / +0xb / +0xf / +0x24: a posição, o teto do preço, o valor de referência, a pessoa e o tipo (bit 0 empréstimo, bits 1..4 o tipo); valor e id; lê (valor_do_jogador_para_o_clube)
- clube +0xcf / +0x1b3 / +0x80 / +0x53 / +0x57: o técnico, os 7 olheiros, a reputação, a nação e a divisão (+0x69); ponteiro e valor; lê (valor_do_jogador_para_o_clube)
- pessoa +0x69: a ficha de conhecimento do observador; ponteiro; lê e grava uma sorteada quando falta (valor_do_jogador_para_o_clube)
- pessoa +0x18 / +0x39 / +0x61: a idade, o clube e a ficha de jogador (+0xb, +0xd, +0x2d); ponteiro e valor; lê (valor_do_jogador_para_o_clube)
- [0xae2d0c] + id × 0x4f + 0xb: os 4 bits da névoa da pessoa; valor; lê (valor_do_jogador_para_o_clube)
- [this+0xdc] + id do clube × 9 + 5: a reputação do clube na tabela das listas; valor; lê (valor_do_jogador_para_o_clube)
- [0x9cf434] / [0x9cf300]: as duas nações com regra própria de preço; id; lê (valor_do_jogador_para_o_clube)
- busca +0x170 / +0x190: o registro e o clube observados; ponteiro e id; lê (entrada_do_jogador_na_busca_do_clube)
- [this+0x86e] + id × 8 bit 2 e [this+0x898] / [this+0x8a0]: a marca e as cláusulas de prêmio por jogos na seleção (0xd bytes, +0 a pessoa); valor e ponteiro; lê e grava a marca (premio_de_jogos_na_selecao_achado)
- jogo +0x40, +0x41 marcas; +0x43/+0x44 e +0x47/+0x48 placar; +0x14 competição; +0x1c/+0x20 clubes; +0x3a reputação conta; valor/ponteiro; lê (contratos_depois_do_jogo)
- resumo da partida + lado × 0x625: +0x190 vagas de 0x49 bytes (+0/+4 ids, +0x1f, +0x20, +0x21, +0x22, +0x27, +0x30, +0x32 minutos), +0x154 palavras; valor; lê
- contrato +0x10, +0x14, +0x18 bônus; +0x35 & 0x3f tipo; valor; lê
- contrato +0x3d felicidade, +0x3e e +0x3f contas de jogos, +0x44 moral; valor; lê e grava
- registro de 79 bytes em [0xae2d0c] +9 (seleção) e +4 (segundo contrato); valor/índice; lê e grava (+9)
- clube +0xd7 elenco (50), +0x80 reputação; ponteiro/valor; lê
- pessoa +0x57, +0x61 ficha; valor/ponteiro; lê
- ficha +0x45 moral; valor; lê (moral_da_ficha, texto_da_moral)
- [0xb74350] +0x10/+0x18 notas do campeonato; valor; lê
- ficha +0xf, +0x12..+0x15 posições; +0x27, +0x28, +0x2d, +0x2f, +0x36, +0x43 notas; +9 reputação; +0xb atual; +0xd potencial; valor; lê (valor_do_jogador)
- pessoa +0x18 idade, +0x22 e +0x1a (+0x8e) experiência, +0x61 ficha, +0x5e marcas de elenco; valor/ponteiro; lê
- clube +0xcf técnico, +0x53 nação (+0x7e, +0x85), +0x57 divisão (+0x69), +0x64, +0x80 reputação; ponteiro/valor; lê
- contrato +0x2d/+0x2f fim, +0x35 tipo e bit 0x40, +0x3d felicidade, +0x21 cláusula, +4 clube; valor; lê
- [0xae1080] contrato consultado; ponteiro; grava
- [0x9a7f30] tabela de 20 bytes pela idade; valor; lê
- ficha +0x45 moral; valor; lê (diferenca_de_moral_em_texto)
- diretoria (0x49 bytes em [gerência dos técnicos] + id × 0x49) +6 confiança; valor; lê e grava (jogadores_que_a_diretoria_cita); +0x2c; valor; lê
- clube +0xbf técnico (+0x57, +0x69 ficha +0x16), +0x57 divisão, +0x80 reputação; ponteiro/valor; lê
- linha da tabela da liga +4 jogos, +0x19, +0x2d, +0x73, +0x74; valor; lê
- pessoa +0x18 idade, +0x3d cargo, +0x61 ficha (+0xb); valor/ponteiro; lê
- registro da partida (0xd7a bytes): lados de 0x625 em +0x11f, vagas em +0x71 (0x49 bytes: +0 id, +0x35 nota); valor; lê (veredito_da_nota_na_partida)
- [0xae1080] contrato consultado; ponteiro; grava
- registro do técnico (0xd2 bytes em [0xdd7f00]+8) +0x8e jogos, +0x90 vitórias, +0x92 empates, +0x94 derrotas, +0x96 gols pró, +0x9a gols contra; valores; grava (resultado_nos_registros_dos_tecnicos)
- registro do técnico +0xb6 pontos do ano, +0xca pontos, +0xce pontos do mês (liga do clube), +0xba temporada no clube, +0xbe temporada na seleção; valores; grava (resultado_nos_registros_dos_tecnicos)
- jogo +0x14 competição, +0x1c/+0x20 clubes, +0x3a não disputado, +0x43/+0x44 e +0x47/+0x48 placares; ponteiros/valores; lê (resultado_nos_registros_dos_tecnicos)
- partida +0x11f/+0x127/+0x128/+0x129 e +0x744/+0x74c/+0x74d/+0x74e contagens de cada lado; valores; lê (resultado_nos_registros_dos_tecnicos)
- clube +0xcf técnico, +0x80 reputação, +0x57/+0x60 ligas, +0x9b..+0xb3 pessoas favoritas e rejeitadas; ponteiros/valores; lê (resultado_nos_registros_dos_tecnicos, posicoes_no_fim_da_liga), grava as favoritas (jogador_do_ano_da_torcida_no_registro)
- preferências da pessoa (+0x65) +4..+0x18 clubes favoritos e rejeitados; ponteiros; grava (jogador_do_ano_da_torcida_no_registro)
- liga +4 competição, +0x3a ano, +0x3e linhas da tabela; valores; lê (posicoes_no_fim_da_liga)
- clube +0x80 reputação; valor; lê e grava (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)
- clube +0xcf técnico, +0xbf presidente, +0x53 nação; ponteiros; lê (os três)
- partida +0x188/+0x18a/+0x18c notas do lado (lado × 0x625), +0x190 escalação (20 × 0x49 bytes, id ou −1); valores/ids; grava as notas, lê a escalação (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)
- competição +0x69 peso, +0x42 fase; valores; lê (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)
- registro do técnico +0x4a clube atual, +0x4e começo, +0x56 data, +0x5e/+0x62/+0x6a passagem mais longa, +0x72/+0x76/+0x7e mais curta, +0xb4 passagens, +0xb5 seleções; ids/datas/valores; grava (troca_de_tecnico_nos_registros)
- [0xdd6ec8] começo do jogo, [0xae2c90] hoje, [0xae2470] data estática da 0x54cdb0; datas; lê, grava a estática (troca_de_tecnico_nos_registros)
- contrato (0x50 bytes) +0x3d, +0x40 marcas de evento, +0x44 moral, +0x45 queixas; valores; lê e grava (reacoes_dos_elencos_a_transferencia)
- [[0xb67a44]] + 0x49 × id do clube, +8: a torcida (1..10000); valor; grava +500 (reacoes_dos_elencos_a_transferencia)
- pessoa +0x18 idade, +0x3e data de chegada, +0x57/+0x58/+0x59/+0x5a/+0x5b atributos, +0x61 ficha (+5, +0xb reputação, +0xd potencial); valores; lê (reacoes_dos_elencos_a_transferencia)
- clube +0xd7 elenco (50), +0x80 reputação, +0x53 nação, +0x57 liga; ponteiros/valores; lê (reacoes_dos_elencos_a_transferencia)
- Media +0 fila das histórias (+8 pessoa, +0x10 tipo, +0x11, +0x13 notícia), +8 conta; valores; lê (reacoes_dos_elencos_a_transferencia)

**Interessa a:** CMRTE, SGE, CMScout
