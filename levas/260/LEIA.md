# 260ª leva — as faltas do dia: o dia das ausências, das faltas ao treino e dos atrasos, a falta ao treino marcada e o atraso registrado; a oferta de transferência recusada pelo clube vendedor, a retirada pelo comprador e a notícia da oferta recusada; o jogador desassossegado pelo lance de um clube maior, a notícia da oferta desfeita e a do prêmio por jogos; a transferência efetivada, com a notícia da transferência concluída e a do acordo entre os clubes; o empréstimo do arquivo de configuração, a notícia do fim do contrato e a do pedido de adiamento da transferência; a tática ativa do clube: a tática do time do clube, a vaga de tática do técnico humano e a tática ativa dele; a escalação guardada do clube: tirar o jogador da escalação, trocar duas vagas e a cópia para desfazer; as notícias do técnico: a diretoria pensa no cargo do técnico, a saída do técnico e o técnico novo; o fim da temporada da liga: os prêmios em dinheiro, os registros pela situação do clube e a notícia da situação; as estatísticas do jogo e os recordes do clube: o jogo nas estatísticas dos jogadores, os recordes de posição na liga e as notícias dos recordes

**Rotinas:**
- `faltas_do_dia()` = 0x4190d0 (awol.cpp, thiscall): acaba a primeira ausência vencida; resolve a primeira falta ao treino de uma semana atrás (caso repetido: sorteios e nova falta; caso novo: colegas que pedem a saída ou perdem profissionalismo, moral e temperamento, e o faltoso que perde profissionalismo e moral e briga com o clube); resolve o primeiro atraso vencido pelos sorteios;
- `teve_falta_ao_treino()` = 0x418ff0 (awol.cpp, thiscall, ret 4): o bit 2 da marca da pessoa; sem contrato, tira a falta antes;
- `teve_incidente_de_forma()` = 0x419070 (awol.cpp, thiscall, ret 4): verdadeiro se a pessoa tem registro na lista dos atrasos;
- `marcar_falta_ao_treino()` = 0x41a360 (awol.cpp, thiscall, ret 4): o jogador muito infeliz pode sumir (0x419840); senão marca a falta, dá a notícia, cai a condição, entra na lista das faltas (o byte do caso, sem valor na original, vai 0x90) e o técnico da IA pode multar;
- `fim_da_ausencia()` = 0x419710 (awol.cpp, thiscall, ret 4): com contrato num clube, a notícia da volta; troca o bit 1 da marca e tira o registro da lista das ausências;
- `marcas_do_contrato()` = 0x867560 (staff_contracts.cpp, thiscall, ret 4): liga as marcas em +0x40 e, se a pessoa agora quer sair e nada impede, o pedido de transferência (0x4d8610);
- `recusar_oferta_de_transferencia()` = 0x8bf680 (transfer_manager.cpp, thiscall): recusa a oferta: o jogador pode se desassossegar, estado 2, lances desmarcados, ofertas de contrato fechadas, a notícia, a volta em 25 dias para sair e a marca de oferta da pessoa;
- `retirar_oferta_de_transferencia()` = 0x8bfcb0 (transfer_manager.cpp, thiscall): retira a oferta: estado 3, ofertas de contrato guardadas reenviadas, lances desmarcados, a volta em 10 dias; no motivo 2, o desassossego; a notícia e as ofertas de contrato;
- `noticia_oferta_recusada()` = 0x8d5170 (transfer_manager.cpp, thiscall): a notícia 0xfa1 da oferta recusada ou retirada, com o motivo, aos clubes humanos interessados;
- `desassossegar_pelo_lance()` = 0x8c1290 (transfer_manager.cpp, thiscall): o sorteio contra a reputação do comprador marca o jogador desassossegado; com a marca, a nota pelos atributos e o interesse leva à queixa 0x80, à fuga e às antipatias;
- `noticia_oferta_desfeita()` = 0x8de790 (transfer_manager.cpp, thiscall): a notícia 0xfb0 da oferta desfeita, com o clube e o motivo, aos clubes humanos interessados;
- `noticia_premio_por_jogos()` = 0x8db8a0 (transfer_manager.cpp, thiscall): a notícia 0xfa6 do prêmio por jogos pago, ao clube que paga e ao clube do contrato;
- `efetivar_transferencia()` = 0x8c9390 (transfer_manager.cpp, thiscall): efetiva a transferência: confiança das diretorias, regras da nação, cláusulas, pagamento, registros, parte dos clubes anteriores, o jogador e os da troca de clube, números de camisa, estado 0x14 e a volta em 10 dias;
- `noticia_transferencia_concluida()` = 0x8dbef0 (transfer_manager.cpp, thiscall): a notícia 0xfa8 da transferência concluída, com o valor, os da troca, um sorteio e os pontos fortes do jogador;
- `noticia_transferencia_acertada()` = 0x8e05d0 (transfer_manager.cpp, thiscall): a notícia 0xfb3 do acordo entre os clubes, com o valor, os da troca e a data da conclusão;
- `emprestimo_da_configuracao()` = 0x8bf400 (transfer_manager.cpp, thiscall): o empréstimo do arquivo de configuração: as condições, o contrato estendido até o fim, o contrato de empréstimo, o elenco, os números de camisa e a permissão de trabalho;
- `noticia_fim_do_contrato()` = 0x4da590 (contract_manager.cpp, thiscall): a notícia 0xbb9 do fim do contrato, do teste ou do empréstimo, ao clube, aos interessados ou, para os famosos, aos técnicos da nação ou a todos;
- `noticia_pedido_de_adiamento()` = 0x8dda90 (transfer_manager.cpp, thiscall): a notícia 0xfab do clube que pede para adiar a transferência uma semana, aos clubes humanos interessados;
- `tatica_do_clube()` = 0x896660 (tactics.cpp, thiscall, ret 8): no clube humano (sem +0x9a01 ou com a opção 2 das férias), a vaga pelo time e o bloco ativo do técnico; nos outros, pelo índice na linha do clube, o bloco pronto (abaixo da conta das prontas) ou o próprio;
- `escolher_vaga_de_tatica()` = 0x5fbe20 (human_manager.cpp, thiscall, ret 0xc): a vaga ativa do técnico humano: 4 na segunda seleção, 3 com reserva numa seleção, 2 numa seleção, 1 com reserva, 0 no time principal;
- `tatica_ativa_do_humano()` = 0x5fbd00 (human_manager.cpp, thiscall, ret 4): o bloco de tática (0x1ecd bytes) da vaga ativa do registro do técnico humano;
- `tirar_da_escalacao()` = 0x896c60 (tactics.cpp, thiscall, ret 0x10): esvazia (−1) as vagas da escalação com a pessoa; com papéis, também as duas vagas extras e os batedores de bola parada; as cópias para desfazer do clube e do time somem; 1 se a tirou de uma vaga ou de bola parada;
- `trocar_vagas_da_escalacao()` = 0x8973e0 (tactics.cpp, thiscall, ret 0x10): troca os ids de duas vagas (0..19) da escalação e solta as cópias para desfazer do clube e do time;
- `tem_copia_da_escalacao()` = 0x897250 (tactics.cpp, thiscall, ret 8): 1 se o humano da vez tem a cópia para desfazer da escalação deste clube e time;
- `noticia_da_pressao()` = 0x6a0e70 (manager_manager.cpp, thiscall, ret 8): a notícia 0x1777 com o técnico, o clube e o presidente; o motivo pelas avaliações da diretoria abaixo da confiança − 500 (sorteios de 3 e 5) e o boato (sorteio de 10, ou 1 se o técnico conhece o clube); entrega a todos, aos do país ou aos da divisão; a data de hoje em +0x18 da linha da diretoria;
- `noticia_da_saida_do_tecnico()` = 0x6a1b00 (manager_manager.cpp, thiscall, ret 0x14): a notícia 6000 com o técnico, o tipo, o clube, o cargo, o presidente e o motivo; técnico humano com os nomes e a todos; senão pelo clube (seleção ou reputação acima de 7250 a todos, senão aos do país);
- `noticia_do_tecnico_novo()` = 0x6a2e00 (manager_manager.cpp, thiscall, ret 0x10): só clube de nação ativa com a divisão ativa, ou seleção de nação ativa: a notícia 0x1787 com o técnico, o clube anterior, o clube, o cargo e a função anterior; entrega como a da saída;
- `pagar_premios_da_liga()` = 0x682e10 (league.cpp, thiscall, ret 0): divide o fundo (+0x28) por 1 + 2 + … + n e paga (n − i) partes ao clube da linha i da tabela (0x5999a0 no registro de finanças), com a notícia do prêmio acima de 1000; com +0xe1 = 3 a competição paga pela virtual +0x6c; com +0xe1 = 2, nada;
- `registros_pela_situacao_na_liga()` = 0x686940 (league.cpp, thiscall, ret 0x18): pela situação: 5 o título (0x868c50), 0 o acesso (0x869480, peso 100), 3 o rebaixamento (0x86a1c0) nos registros das pessoas; devolve 0;
- `anunciar_situacao_na_liga()` = 0x686670 (league.cpp, thiscall, ret 0x14): fora dos amistosos, chama a virtual +0x44 da competição e monta a notícia 0xc (clube, competição, fase, situação, rodada, o que a virtual devolveu), entregue pelo alcance ou, pela situação, à nação, à divisão (e à de acesso ou rebaixamento) ou ao técnico do clube;
- `registrar_partida_nas_estatisticas()` = 0x7c0d70 (player_stats.cpp, thiscall, ret 8): pela categoria de cada lado (amistoso, reservas, tipo da competição; sorteio no jogo simulado), soma a nota e o jogo nas somas em double e cada linha que jogou nos registros das categorias (jogos, gols, placar, nota, minutos...), nos jogos e gols do jogador e no +0x3b do contrato; anda a forma meio byte;
- `recordes_de_posicao_na_liga()` = 0x447430 (club_records.cpp, thiscall, ret 8): ordena a tabela de cada fase da liga e grava a melhor e a pior posição de cada clube na temporada e, com o segundo argumento, de sempre (com a notícia na fila para clube humano ou seleção);
- `noticias_de_recordes()` = 0x448e30 (club_records.cpp, thiscall, ret 0): entrega a notícia de cada recorde da fila (clube humano ou seleção) e esvazia a fila (solta com capacidade de 100 ou mais);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (28):** 24 acusadas. `cpx1_sem_desmarcar_comprador` escapou: os lances do comprador que ficam marcados só pesam numa nova oferta dele pelo mesmo jogador, que a carreira até 2002-06-01 não fez [inferência]. `cpx2_sorteio_2000` escapou: nenhum comprador da carreira teve a reputação entre os dois limites (2000 e 4250 mais o sorteio) [inferência]. `cpx3_sem_parte_dos_anteriores` escapou: a parte dos clubes anteriores só vale com a cláusula no contrato, que nenhuma transferência da carreira teve [inferência]. `cpx10_limite_400` escapou: o limite só muda o motivo do texto da notícia, e nenhuma avaliação da diretoria caiu na faixa entre 400 e 500 abaixo da confiança [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (7da97d00).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 2e9d1c64, original 086f15e7 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 259ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 260`

**Campos tocados:**
- AWOL +0 ausências, +4 faltas ao treino, +8 atrasos (listas com conta em +0, primeiro nó em +2, cursor em +0xa), +0xc marcas por pessoa (bit 1 ausente, bit 2 faltou); ponteiros/valores; lê e grava (faltas_do_dia, teve_falta_ao_treino, teve_incidente_de_forma)
- registro da falta (13 bytes) +0 pessoa, +4 data, +0xc caso repetido; registro do atraso +0 pessoa, +8 data, +0x10 marca; id/valores; lê (faltas_do_dia)
- pessoa +0x18 idade, +0x57 atributo, +0x5b profissionalismo, +0x5d temperamento; valores; lê e grava +0x5b e +0x5d (faltas_do_dia)
- jogador (pessoa +0x61) +0xb reputação atual, +0x44 moral; valores; lê e grava +0x44 (faltas_do_dia)
- clube +0xd7 elenco (50 vagas); contrato +0x4f tipo (nibble alto); [0xae1080] último contrato; ponteiros/valores; lê, grava [0xae1080] (faltas_do_dia, teve_falta_ao_treino)
- contrato +0x40 marcas, +0x44, +0x45 bits, +0x49, +0x4e bits 1 e 8; valores; lê, grava +0x40 (marcas_do_contrato, marcar_falta_ao_treino)
- [0xae2d0c] linhas de 79 bytes por pessoa, +0xa; valor; soma sorteio(10) (marcar_falta_ao_treino); pessoa +0x3d função, +0x59 atributo; lê
- oferta +0: lista das ofertas rivais; ponteiro; lê (recusa, retirada)
- oferta +4: lista dos jogadores da troca (+4 de cada vaga: marca); ponteiro; lê e grava 0 na marca (recusa, retirada)
- oferta +8 comprador, +0xc jogador; id; lê (as três)
- oferta +0x10; valor; grava 4 e 0 (retirada), 0 (recusa)
- oferta +0x2c: estado; valor; grava 2 (recusa) e 3 (retirada); lê (a notícia)
- oferta +0x2d; valor; grava 1 com a marca (recusa, retirada)
- oferta +0x11, +0x1f, +0x20 vaga, +0x2e, +0x2f; valor; lê
- gerência de transferências +0x86e: registro de 8 bytes por pessoa, bit 0 = oferta em andamento; valor; lê e apaga (recusa, retirada)
- contrato +4 clube, +0x35 & 0x3f tipo; valor; lê (a notícia)
- [0xae1080] último contrato consultado; ponteiro; grava (as três)
- oferta +8 comprador, +0xc jogador; id; lê (desassossegar_pelo_lance, noticia_oferta_desfeita)
- oferta +0x2d: jogador desassossegado; valor; grava 1 e lê (desassossegar_pelo_lance)
- clube +0x80: reputação; valor; lê (desassossegar_pelo_lance)
- pessoa +0x18 idade, +0x57, +0x59, +0x5b, +0x5d: atributos de personalidade; valor; lê (desassossegar_pelo_lance)
- ficha +0xb: reputação atual; valor; lê (desassossegar_pelo_lance)
- contrato +0x4e & 8: listado; +0x44: força do pedido; valor; lê (desassossegar_pelo_lance)
- gerência de transferências +0x86e: registro de 8 bytes por pessoa, +1; valor; lê (desassossegar_pelo_lance)
- cláusula +0 pessoa, +4, +9 clube que paga; id; lê (noticia_premio_por_jogos)
- [0xae1080] último contrato consultado; ponteiro; grava (as três)
- oferta +4: lista dos jogadores da troca; ponteiro; lê (as três)
- oferta +8 comprador, +0xc jogador; id; lê (as três)
- oferta +0x1b: valor da transferência; valor; lê (as três)
- oferta +0x1f, +0x11, +0x2f, +0x30; valor; lê
- oferta +0x2c: estado; valor; grava 0x14 (efetivar_transferencia)
- gerência de transferências +0x8ac: as 28 regras de nação (+0xa: nação); ponteiro; lê (efetivar_transferencia)
- gerência de transferências +0x86e: registro de 8 bytes por pessoa, bit 0 = oferta em andamento; valor; lê e apaga (efetivar_transferencia)
- clube +0xd7: elenco (50 vagas); ponteiro; grava 0 na vaga do primeiro da troca (efetivar_transferencia)
- pessoa +0x39 clube, +0x69, +0x61 ficha; ponteiro; lê
- contrato +0x2d/+0x2f: fim do contrato; valor; lê (noticia_transferencia_concluida)
- [0xae1080] último contrato consultado; ponteiro; grava (as três)
- pessoa +0x46: data (fim do vínculo); valor; lê e grava o fim do empréstimo (emprestimo_da_configuracao)
- pessoa +0x6d; valor; grava 0x10 no lugar de 2 (emprestimo_da_configuracao)
- contrato +0x2d: fim do contrato; valor; grava o fim do empréstimo (emprestimo_da_configuracao)
- contrato local +0x25: começo; valor; grava (emprestimo_da_configuracao)
- pessoa +0x1a nação, +0x39 clube atual, +0x61 ficha; ficha +0xb/+0xd reputação; valor; lê (noticia_fim_do_contrato)
- registro de 79 bytes +0 (general.dat): índice do contrato; valor; lê (noticia_fim_do_contrato)
- oferta +8 comprador, +0xc jogador, +0x20 vaga, +0x2c estado; valor; lê (noticia_pedido_de_adiamento)
- [0xae1080] último contrato consultado; ponteiro; grava (as notícias)
- Tactics +0x5f1 blocos próprios, +0x5f9 blocos prontos (0x18e3 bytes), +0x5fd conta das prontas, +0x601 tabela tática (linhas de 0x91 bytes por id, +0 índice da tática, palavra com sinal); ponteiros/valores; lê (tatica_do_clube)
- HUMAN_MANAGER +0 registros (0xa6ef bytes, índice id − [0xae2364] + 16), +4 conta; registro +0x9a69 vaga ativa (byte com sinal), blocos de tática de 0x1ecd bytes; valores; lê e grava +0x9a69 (escolher_vaga_de_tatica), lê (tatica_ativa_do_humano)
- clube +0 id, +0xcf técnico; pessoa +0x39 clube; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] humano da vez, [0xb63c98] gerente dos humanos; ponteiros/ids; lê
- Tactics +0x601 tabela tática (linhas de 0x91 bytes: +2 as 20 vagas da escalação, ids, −1 vazia; +0x56/+0x5a vagas extras; +0x5e batedores); ids; lê e grava (tirar_da_escalacao, trocar_vagas_da_escalacao)
- Tactics +0x30 cópias para desfazer (16 × 0x5c bytes: +0 clube, +4 time, +8 a cópia), +0x5f0 conta (byte); ponteiros/valores; lê e grava +8 e a conta (tirar_da_escalacao, trocar_vagas_da_escalacao), lê (tem_copia_da_escalacao)
- pessoa +0 id; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] humano da vez; [0xae2364] conta de pessoas; ids; lê
- linha da diretoria (0x49 bytes por id em [MANAGER_MANAGER+0]) +6 confiança, +8/+0xa/+0xc avaliações (palavras com sinal), +0x18 data da última notícia de pressão; valores; lê, grava +0x18 (noticia_da_pressao)
- clube +0 id, +0x53 nação, +0x57 divisão, +0x80 reputação, +0xbf presidente, +0xcf técnico; pessoa +0 id, +4/+8/+0xc nomes (+0x33 chave), +0x28 e +0x3d função, +0x69 não jogador (+0x1b); ponteiros/ids/valores; lê
- [0xdd6ec8] data a partir da qual saem as notícias de técnico; nação +0x11c ativa; [0xadadfc] divisões ativas; [0x9cf764] divisão sem notícia; datas/ponteiros/ids; lê
- liga +0xe1: o modo dos prêmios (2 nenhum, 3 pela competição); valor; lê (pagar_premios_da_liga)
- liga +0x28: o fundo de prêmios; valor; lê (pagar_premios_da_liga)
- liga +0xb1 / +0x3e: a tabela (linhas de 0x3b, +0 clube) e quantas; ponteiro e valor; lê (pagar_premios_da_liga)
- finanças do clube +0 (64 bits), +0xa8, +0x148: o saldo e os acumulados de prêmios; valor; grava (pagar_premios_da_liga, pela 0x5999a0)
- liga +0x3a / +0x44 / +0xc / +0x1c: a rodada, a fase, as fases e a divisão de cima; valor e ponteiro; lê (anunciar_situacao_na_liga)
- competição (objeto de [0xadadfc]) +0x1c / +0x20: as divisões de acesso e de rebaixamento; id; lê (anunciar_situacao_na_liga)
- [0xdd7f00] os registros das pessoas; ponteiro; lê (registros_pela_situacao_na_liga)
- PlayerStats +0x10..+0x28: as somas das notas e dos jogos (double; +0x20/+0x28 com a marca +0x4c do jogo); valor; grava (registrar_partida_nas_estatisticas)
- registro de categoria (0x20 bytes) +0 jogos, +1 entradas, +7/+8 gols a favor/contra, +9/+0xa vitórias/derrotas, +0xd melhor em campo, +0xe nota, +0x1c minutos; valor; grava (registrar_partida_nas_estatisticas)
- registro do jogador (PlayerStats +0x30, 0x27 bytes) +0x20 jogos, +0x22 gols, +0x24..+0x26 a forma (meio byte por jogo); valor; grava (registrar_partida_nas_estatisticas)
- contrato +0x3b: os jogos pelo clube; valor; grava (registrar_partida_nas_estatisticas)
- linha do jogo (0x49 bytes a partir de +0x190 + 0x625 × lado) +0 pessoa, +4 jogador, +0x21..+0x2d contagens, +0x30/+0x32/+0x34 saída, entrada e minutos, +0x35 nota; valor; lê (registrar_partida_nas_estatisticas)
- recorde (temporada 0x4528d0 / sempre 0x452710) +0/+1/+5 melhor posição, competição e ano; +7/+8/+0xc a pior; valor e id; lê e grava (recordes_de_posicao_na_liga)
- Club_Records +0x24/+0x28/+0x2c: a fila das notícias de recorde (6 bytes: id, tipo, período), capacidade e conta; ponteiro e valor; lê e grava (recordes_de_posicao_na_liga, noticias_de_recordes)
- competição ([0xae23d0], 0x6b bytes) +0x69: a reputação; valor; lê (recordes_de_posicao_na_liga)

**Interessa a:** CMRTE, SGE, CMScout
