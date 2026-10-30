# 251ª leva — a confiança da diretoria: o negócio da transferência, a confiança depois da transferência e o ultimato; os contratos: tirar o contrato principal, reenviar as ofertas guardadas e o mês do fim da temporada do clube; os pontos dos técnicos: a virada do mês e do ano, o fim da temporada da competição e o técnico do mês; as saídas dos clubes: as saídas pendentes dos técnicos, tirar o jogador do elenco e tirar a pessoa do cargo; a notícia nova e a confiança da torcida e da diretoria depois de uma história da mídia; a resposta do técnico da máquina a uma história da mídia, marcar a notícia lida e achar a notícia pelo id; as notícias do jogador depois de uma história da mídia: gratidão ao técnico, pedido de apoio e reação à crítica; as listas de posição da lista de observação: montar as listas, ordená-las a cada dia e os clubes interessados no jogador; a lista de observação e o empréstimo: acrescentar o par na lista, prever a resposta do clube vendedor e o período do empréstimo; as transferências: o prazo da resposta à oferta, pagar as prestações do mês e tirar a oferta; as suspensões na troca de clube: acrescentar a suspensão, soltar as vagas que não valem no clube novo e devolver as suspensões guardadas no fim do empréstimo; os idiomas do jogador (acrescentar e melhorar) e zerar as estatísticas do jogador regenerado; as seleções: a competição atual da nação e os clubes com jogo na data; a disciplina: tirar o jogador regenerado

**Rotinas:**
- `avaliar_o_negocio_da_transferencia()` = 0x696570 (manager_manager.cpp, thiscall, ret 0xc): marca o negócio para o comprador e o vendedor pela razão taxa/valor do jogador (faixas por valor acima ou até 250.000, taxa até (reputação × 0,2)² é justa), soma e tira o valor do gasto líquido de cada um e marca o jogador favorito ou rejeitado do clube e do presidente;
- `confianca_da_diretoria_na_transferencia()` = 0x6978f0 (manager_manager.cpp, thiscall, ret 4): muda a confiança (+6, 1..10000) e os humores (+0x14, +0x16) da diretoria do comprador e do vendedor pelas marcas do negócio e tira o jogador das pessoas favoritas do presidente do vendedor;
- `resolver_o_ultimato_da_diretoria()` = 0x6ac470 (manager_manager.cpp, thiscall, ret 4): tira o clube da lista dos ultimatos e, se o técnico está há mais de 100 dias no cargo, põe a saída dele na lista (motivo 2, ou 7 sem jogos na tabela) e devolve 1;
- `tirar_o_contrato_principal()` = 0x4d0b50 (contract_manager.cpp, thiscall, ret 4): libera a entrada do contrato principal da pessoa (−1), põe −1 na vaga dela no registro de 79 bytes e refaz a folha de salários do clube do contrato;
- `reenviar_ofertas_guardadas()` = 0x4cf4a0 (contract_manager.cpp, thiscall, ret 4): reenvia como ofertas vivas as ofertas guardadas da pessoa (com a transferência do mesmo clube, estado 0xf vira 8; sem transferência, com −1), tira cada uma da lista e devolve quantas;
- `mes_do_fim_da_temporada_do_clube()` = 0x4d45f0 (contract_manager.cpp, stdcall, ret 4): o mês (0..11) da data dez dias depois do fim da temporada da nação do clube;
- `pontos_dos_tecnicos_no_dia()` = 0x86dda0 (staff_records.cpp, thiscall, ret 4): na virada do mês ou do ano zera os pontos do mês de todos os técnicos e guarda o mês e o ano; no primeiro dia do ano passa os pontos do ano ao hall da fama e os zera;
- `pontos_dos_tecnicos_no_fim_da_competicao()` = 0x86d6e0 (staff_records.cpp, thiscall, ret 4): no fim da temporada da competição passa ao hall da fama os pontos de liga (com os da copa da mesma competição) e de copa dos técnicos dela e os zera;
- `tecnico_do_mes_na_liga()` = 0x86f950 (staff_records.cpp, thiscall, ret 4): o técnico da liga com mais pontos no mês (acima de 0) entre os que estão no cargo há duas semanas; liga sem tabela: a caixa "Error" e 0;
- `saidas_pendentes_dos_tecnicos()` = 0x69b650 (manager_manager.cpp, thiscall, ret 0): percorre a lista das saídas pendentes e, em cada clube com técnico, faz a saída dele e, se o clube abre vaga, a vaga; tira cada item da lista;
- `tirar_jogador_do_elenco_do_clube()` = 0x690730 (manager_manager.cpp, thiscall, ret 8): tira o jogador do clube (ou da seleção): a função de jogador-técnico volta à de técnico (senão 0xb e ele sai), das listas, da mídia, das táticas e das 50 vagas do elenco; o empréstimo fica com o estado 4;
- `tirar_pessoa_do_cargo_no_clube()` = 0x690890 (manager_manager.cpp, thiscall, ret 8): tira o contrato e a pessoa da vaga do cargo no clube ou na seleção (presidente, diretor, técnico, auxiliar, treinador, olheiro, fisioterapeuta; as funções de jogador viram 0xb), da mídia e, se humana, das listas do humano;
- `construir_noticia()` = 0x77bbe0 (news.cpp, thiscall, ret 8): monta a notícia: tipo (+0), byte importante (+4), os 50 dados zerados, a data de hoje (+0xcd), o byte [0xae2c88] em +0xd5, id e +0xda zerados; em DESLIGADAS desde 29/10/2026: com o desvio a carreira no modo Rust saía da referência be54a138 (bissecção das 39 rotinas da 251ª; com a 0x77bbe0, que constrói a notícia, o jogo caía depois na 0x7c6650 com um índice estragado); a causa (resíduo de pilha ou diferença da cópia) ainda não foi isolada; a cópia fica no código e não roda;
- `ajustar_confianca_da_torcida()` = 0x738340 (media.cpp, thiscall, ret 0x14): soma na confiança da torcida (+8 da linha do clube em [[0xb67a44]], 1..10000) o alvo menos a confiança atual, × 0,75, amortecido pelo técnico favorito ou rejeitado e acima de ±1000; fora de ±500, a notícia 0xbe0 ao técnico;
- `ajustar_confianca_da_diretoria()` = 0x7386f0 (media.cpp, thiscall, ret 0x18): soma na confiança da diretoria (+6 da linha do clube) (confiança − 6000)/20 + alvo, × 0,75, amortecido pelo presidente que gosta ou não do técnico e acima de ±1000; fora de ±500, a notícia 0xbe0 ao técnico com as marcas do sujeito;
- `resposta_do_tecnico_da_maquina()` = 0x735870 (media.cpp, thiscall, ret 0xc): a resposta 1, 2 ou 3 do técnico da máquina à história do tipo sobre o jogador: pelo contrato, pela nota do jogador na história (× 5), pelas preferências do técnico e por cadeias de sorteios por família de tipos; alguns tipos põem o jogador entre os rejeitados do técnico;
- `marcar_noticia_lida()` = 0x787140 (news.cpp, thiscall, ret 0xc): marca lida a primeira notícia do id ainda não lida no anel do humano, se ela não pede resposta (o botão 0x782b70), tira da conta e, a pedido, repete para os humanos da mesma máquina da rede;
- `achar_noticia_pelo_id()` = 0x786b70 (news.cpp, thiscall, ret 0xc): copia na saída a notícia do id: no anel de 100 vagas do técnico humano ou, sem técnico, entre todas as vagas;
- `noticia_de_gratidao_do_jogador()` = 0x732c80 (media.cpp, thiscall, ret 0x10): a notícia 0xbda ao técnico: o jogador, o clube, o tipo (0x12/0x21 por sorteio para jogador na lista da mídia ou com contrato marcado, 0xc lesionado), a variante, o sorteio do tipo (que pode subir +0x44 da ficha), o técnico e o auxiliar;
- `noticia_de_pedido_de_apoio()` = 0x743370 (media.cpp, thiscall, ret 0x10): para clube humano, a notícia 0xbcf ao técnico: o jogador, o clube, a faixa de reputação e o grau do pedido pelos atributos, o tipo, o quarto argumento e se está listado para transferência;
- `noticia_de_reacao_a_critica()` = 0x743a50 (media.cpp, thiscall, ret 0x10): a notícia 0xbdc: o jogador, o clube, a reação (0..4 pela reputação, a categoria do contrato, a idade, os atributos e a resposta), a resposta, a confiança da diretoria abaixo de 4000, o tópico e o técnico, entregue aos técnicos dos clubes interessados (0x8d1da0);
- `montar_listas_de_posicao()` = 0x839e60 (shortlist_manager.cpp, thiscall, ret 4): refaz as 32 listas (16 vagas × 2) com 12 bytes por entrada: o tamanho é o maior número de pessoas que servem para uma vaga (0x844300); cada pessoa fora dos humanos entra em cada vaga que serve com o id, o id da nação (sorteada se faltar) e o valor ou o +0x3d do contrato;
- `ordenar_listas_de_posicao()` = 0x83a780 (shortlist_manager.cpp, thiscall, ret 0): refaz a chave das entradas (o valor com contrato de faixa 0, menos 1.500.000 no jogador listado de clube humano; na segunda lista −1000, 2000 ou o +0x3d do contrato) e ordena os dois vetores de cada vaga (0x945333 com 0x83b670);
- `clubes_interessados_no_jogador()` = 0x842180 (shortlist_manager.cpp, thiscall, ret 0x14): esvazia a lista de saída e põe os pares (jogador, clube): os humanos que têm o jogador na lista do olheiro e os clubes do vetor pela pessoa não suspensos, que não são o dele e põem nele mais que o valor;
- `acrescentar_na_lista_de_observacao()` = 0x8423f0 (shortlist_manager.cpp, thiscall, ret 8): cria os dois vetores de pares (100 de capacidade), e o par ainda não estando: cresce de 100 em 100, acrescenta e ordena (pela pessoa, pelo clube) e, fora de clube humano, soma 1 no nibble baixo do +0xb do registro de 79 bytes da pessoa;
- `prever_resposta_do_vendedor()` = 0x84af00 (shortlist_manager.cpp, stdcall, ret 0x20): 0 aceita, 1 negocia, 2 recusa: pelo contrato com o vendedor, o empréstimo (listado, reputação, estatuto), o preço (humano, cláusula, férias, pedido ou 0x4d1600) contra a oferta;
- `periodo_do_emprestimo()` = 0x8d0590 (transfer_manager.cpp, thiscall, ret 0x1c): o período pelas regras da nação do clube e, com contrato de tipo 2, recua o fim mês a mês até não passar do fim do contrato (o início acompanha); sem contrato, vencido, longe demais ou antes deste ano: o ano padrão 1972; em DESLIGADAS desde 29/10/2026: com o desvio a carreira no modo Rust saía da referência be54a138 (bissecção das 39 rotinas da 251ª; com a 0x77bbe0, que constrói a notícia, o jogo caía depois na 0x7c6650 com um índice estragado); a causa (resíduo de pilha ou diferença da cópia) ainda não foi isolada; a cópia fica no código e não roda;
- `prazo_da_resposta_a_oferta()` = 0x8cc5b0 (transfer_manager.cpp, thiscall, ret 0xc): pelo prazo até fechar a janela (as regras das nações do comprador e do clube do jogador): 0 dia, 1 ou 2 dias com fatias sorteadas, ou um valor pelo interesse do jogador (0x846920) ou sorteado pelo estatuto; dias = valor / 3, fatias = valor % 3;
- `pagar_prestacoes_das_transferencias()` = 0x8caee0 (transfer_manager.cpp, thiscall, ret 0): paga cada prestação (finanças do pagador e do recebedor), soma as frações na figura das prestações (+0x1c) nas 8 últimas, tira as acabadas e zera a figura dos clubes sem prestação no sentido dela;
- `tirar_oferta_de_transferencia()` = 0x8d0f60 (transfer_manager.cpp, thiscall, ret 4): destrói a oferta ligada, tira o par da lista de empréstimos e as propostas de contrato da troca e do jogador, baixa as contas de ofertas do jogador (+0x86e) e do gerente, a menor vaga livre acompanha e a oferta é destruída;
- `acrescentar_suspensao()` = 0x556940 (discipline.cpp, thiscall, ret 8): com suspensão da pessoa numa das 4 regras, estende a velha (jogos ou dias) e marca as duas; senão copia a nova na primeira vaga livre do vetor (ou no fim, crescendo 100 até 0x7d00), liga cada regra a ela (0x7e8510) e, com a quarta regra, põe {id, nação} no índice das nações (qsort);
- `tirar_suspensoes_de_fora_do_clube()` = 0x556270 (discipline.cpp, thiscall, ret 0xc): na troca de clube, libera cada vaga das 4 do registro da pessoa cujo órgão (nação +0x17 ou continente +0x1b) não vale para o clube novo: solta a suspensão, vaga e entrada livres; com suspensões guardadas, a caixa "Error";
- `restaurar_suspensoes_guardadas()` = 0x556570 (discipline.cpp, thiscall, ret 4): no fim do empréstimo, devolve às vagas as suspensões guardadas da pessoa (a vaga ocupada é liberada antes; o item sai da lista) e libera as vagas 0..2 não devolvidas cujo órgão não vale para o clube do contrato;
- `acrescentar_idioma()` = 0x7c6a30 (player_stats.cpp, thiscall, ret 0x10): põe no fim da tabela dos idiomas (6 bytes: id, idioma, fluência até 10; nasce com 100, cresce 100) o registro novo e, se pedido, ordena (qsort);
- `melhorar_idioma()` = 0x7c6ca0 (player_stats.cpp, thiscall, ret 0x10): acha o registro da pessoa no idioma (bsearch) e soma o aumento à fluência, até 10; sem ele, acrescenta o idioma com a fluência do aumento;
- `zerar_estatisticas_do_jogador()` = 0x7c6650 (player_stats.cpp, thiscall, ret 4): libera os 6 registros de categoria do jogador (+0xe = −1, +1 nos livres), zera os totais da liga e as notas recentes e esvazia o histórico; em DESLIGADAS desde 29/10/2026: com o desvio a carreira no modo Rust saía da referência be54a138 (bissecção das 39 rotinas da 251ª; com a 0x77bbe0, que constrói a notícia, o jogo caía depois na 0x7c6650 com um índice estragado); a causa (resíduo de pilha ou diferença da cópia) ainda não foi isolada; a cópia fica no código e não roda;
- `definir_competicao_da_selecao()` = 0x775220 (national_teams.cpp, thiscall, ret 8): a competição vira a atual da nação; com uma fase final grande em curso, a nova vai para a fila; com a nova sendo fase final grande, a atual vai para a fila; avisa o gerenciador dos técnicos (0x6a3770);
- `marcar_clubes_com_jogo()` = 0x7775c0 (national_teams.cpp, thiscall, ret 4): na tabela de um byte por clube, marca os clubes dos jogos não amistosos da data: 1 jogo da liga do clube, 2 outro, 3 clube de nação marcada (entrada sem nação); em DESLIGADAS desde 29/10/2026: com o desvio a carreira no modo Rust saía da referência be54a138 (bissecção das 39 rotinas da 251ª; com a 0x77bbe0, que constrói a notícia, o jogo caía depois na 0x7c6650 com um índice estragado); a causa (resíduo de pilha ou diferença da cópia) ainda não foi isolada; a cópia fica no código e não roda;
- `tirar_jogador_da_disciplina()` = 0x556830 (discipline.cpp, thiscall, ret 4): apaga o histórico de suspensões, solta as suspensões e libera as 4 vagas, tira as suspensões guardadas da lista e deixa o registro de 29 bytes livre (0xff, conta 0);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (33):** 31 acusadas. `cpo_tecnico_recente` escapou: o ramo do técnico chegado há menos de duas semanas rodou na base, mas os pontos do mês dele nunca passaram os do melhor da liga, então tirar a regra não muda o escolhido; o comparador (mesmo retorno da original) e a carreira confirmam. `cpo_tirar_ofertas_pessoa` escapou: o byte +1 do registro de 8 bytes da pessoa ([+0x86e]) só serve de atalho na busca da maior oferta (0x8cc930): sobrando maior que zero, a busca percorre o vetor, não acha oferta pela pessoa e devolve 0 como o atalho; a carreira não muda. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (2f65fd6b68d1027b).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (be54a1386f51073a).

**Refazer**, sobre o commit da 250ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 251`

**Campos tocados:**
- MANAGER_MANAGER +0: a tabela da diretoria por clube (0x49 bytes pelo id); ponteiro; lê (as três)
- diretoria +2: marcas (bit 2 = ultimato cumprido); valor; grava (resolver_o_ultimato_da_diretoria)
- diretoria +6: confiança (1..10000); valor; lê e grava (confianca_da_diretoria_na_transferencia)
- diretoria +0x14, +0x16: humores (+0x16 não abaixo de −10000); valor; lê e grava (confianca_da_diretoria_na_transferencia)
- diretoria +0x38: gasto líquido em transferências; double; lê e grava (avaliar_o_negocio_da_transferencia)
- MANAGER_MANAGER +4, +0x10: as listas das saídas pendentes (6 bytes) e dos ultimatos; ponteiro; lê e grava (resolver_o_ultimato_da_diretoria)
- transferência +8, +0xc, +0x1f: clube comprador, jogador, já avaliada; id e valor; lê
- pessoa +0x39, +0x61, +0x3e/+0x42: clube, ficha do jogador, data de entrada no cargo; ponteiro e valor; lê
- clube +0x80, +0xbf, +0xcf, +0x57: reputação, presidente, técnico, divisão; valor e ponteiro; lê
- finanças +0x165: clube sem confiança da diretoria (1); valor; lê (confianca_da_diretoria_na_transferencia)
- [0xae2d0c] registro de 79 bytes da pessoa +0: o índice do contrato principal (−1 sem); valor; lê e grava −1 (tirar_o_contrato_principal)
- CONTRACT_MANAGER +0: o vetor dos contratos (0x50 bytes; +0 −1 livre, +4 id do clube); ponteiro; lê e grava (tirar_o_contrato_principal)
- CONTRACT_MANAGER +0x38: a lista das ofertas guardadas (+0 id da pessoa, +0x59 transferência, +0x5d clube, +0x61 byte); ponteiro; lê e grava (reenviar_ofertas_guardadas)
- transferência (0x8c61e0) +0x24, +0x2c: clube e estado (0xf vira 8); id e valor; lê e grava (reenviar_ofertas_guardadas)
- clube: a data do fim da temporada pela nação (0x669bd0); valor; lê (mes_do_fim_da_temporada_do_clube)
- Staff_Records +8: os registros de pontos (0xd2 bytes, [0xae2368] + 16); ponteiro; lê (as três)
- Staff_Records +0x14, +0x16: o mês e o ano guardados; valor; lê e grava (pontos_dos_tecnicos_no_dia)
- registro +0x86: id da pessoa (−1 vazio); id; lê
- registro +0xb6: pontos do ano; valor; lê e grava 0 (pontos_dos_tecnicos_no_dia)
- registro +0xba/+0xbe e +0xc2/+0xc6: pontos de liga e copa e as competições; valor e id; lê e grava (pontos_dos_tecnicos_no_fim_da_competicao)
- registro +0xca/+0xce: pontos do mês; valor; lê e grava 0 (pontos_dos_tecnicos_no_dia, tecnico_do_mes_na_liga)
- pessoa +0x39, +0x3e/+0x40: clube e data de entrada no cargo (dia com sinal, ano sem); ponteiro e valor; lê (tecnico_do_mes_na_liga)
- liga +0x43, +0x3e, +0xb1: tipo (2 = tabela), conta e vetor da tabela (0x3b bytes); valor e ponteiro; lê (tecnico_do_mes_na_liga)
- competição +0x11c: guarda no hall da fama; valor; lê (pontos_dos_tecnicos_no_fim_da_competicao)
- MANAGER_MANAGER +4: a lista das saídas pendentes (6 bytes: dois bytes e o id do clube em +2); ponteiro; lê e grava (saidas_pendentes_dos_tecnicos)
- pessoa +0x39, +0x3d: clube e função no clube; ponteiro e valor; lê e grava (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)
- pessoa +0x24, +0x28: seleção e função na seleção; ponteiro e valor; lê e grava (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)
- clube +0xbf, +0xc3 (3), +0xcf, +0xd3: presidente, diretores, técnico, auxiliar; ponteiro; lê e grava 0 (tirar_pessoa_do_cargo_no_clube)
- clube +0x19f (5), +0x1b3 (7), +0x1cf (3): treinadores, olheiros, fisioterapeutas; ponteiro; lê e grava 0 (tirar_pessoa_do_cargo_no_clube)
- clube +0xd7 (50): as vagas do elenco; ponteiro; lê e grava 0 (tirar_jogador_do_elenco_do_clube)
- contrato de empréstimo (0x4cf5a0) +0x4e: estado; valor; grava 4 (tirar_jogador_do_elenco_do_clube)
- linha do clube em [[0xb67a44]] (id × 0x49) +6: confiança da diretoria (1..10000); valor; lê e grava (ajustar_confianca_da_diretoria)
- linha do clube em [[0xb67a44]] +8: confiança da torcida (1..10000); valor; lê e grava (ajustar_confianca_da_torcida)
- linha do clube em [[0xb67a44]] +0xe: palavra levada na notícia da diretoria; valor; lê (ajustar_confianca_da_diretoria)
- clube +0, +0xbf, +0xcf: id, o presidente e o técnico; id e ponteiro; lê (os ajustes)
- pessoa +0x18, +0x3e/+0x42, +0x59, +0x5b, +0x61: idade, a data de chegada, dois atributos e a ficha (+5 reputação); valor e ponteiro; lê (ajustar_confianca_da_diretoria)
- notícia +0, +4, +5..+0xcc, +0xcd, +0xd5, +0xd6, +0xda: tipo, importante, os 50 dados, a data, o byte de [0xae2c88], id, lida; valor; grava (construir_noticia)
- [0xae2c90] a data de hoje; [0xb74350] +0x10/+0x18 as notas da liga; valor; lê
- pessoa (técnico) +0x24, +0x39, +0x57, +0x59..+0x5d, +0x65, +0x69: seleção, clube, atributos, as preferências e a ficha de técnico (+0x14, +0x19, +0x1b, +0x1d, +0x1f); ponteiro e valor; lê, e grava +0x65 +0x28..+0x30 pela 0x547af0 (resposta_do_tecnico_da_maquina)
- pessoa (jogador) +0, +0x5a, +0x5b, +0x61 (+0xb reputação, +0x25, +0x44, +0x45): id, atributos e a ficha; valor; lê (resposta_do_tecnico_da_maquina)
- contrato +0x3d, +0x3f, +0x49, +0x4f: valores e o tipo (nibble alto); valor; lê (resposta_do_tecnico_da_maquina)
- [0xae2d0c] registro de 0x4f bytes da pessoa +0xa, +0xb (bit 7); valor; lê (resposta_do_tecnico_da_maquina)
- News +0/+4/+8: o vetor das notícias, a conta de vagas e as entradas dos 16 humanos (início do anel, próxima, conta); ponteiro e valor; lê e grava a conta (marcar_noticia_lida, achar_noticia_pelo_id)
- notícia +0xd6, +0xda: id e lida; valor; lê e grava +0xda (marcar_noticia_lida)
- pessoa +0x5f: humano ativo (1); valor; lê (marcar_noticia_lida)
- ficha do jogador (pessoa +0x61) +0x44: sobe 1 (até 20) na gratidão quando o sorteio(5) dá 0 ou 1; valor; lê e grava (noticia_de_gratidao_do_jogador)
- ficha do jogador +0xb: reputação; valor; lê (pedido de apoio, reação à crítica)
- pessoa +0, +4/+8/+0xc (+0x33), +0x18, +0x3e/+0x42, +0x57, +0x59, +0x5b, +0x5d: id, as chaves, idade, a chegada e atributos; valor; lê (as três)
- contrato +0x45: marcas 0x2000, 8 e 0x181; valor; lê (gratidão, reação à crítica)
- Media +0/+8: a lista da mídia (0x17 bytes: +8 id da pessoa, +0x10 marca 2) e a conta; ponteiro e valor; lê (noticia_de_gratidao_do_jogador)
- clube +0, +0x80, +0xcf, +0xd3: id, reputação, técnico e auxiliar; linha em [[0xb67a44]] +6 (confiança da diretoria); valor e ponteiro; lê
- Shortlist_Manager +0x18/+0x58: os 16 + 16 vetores das listas de posição (12 bytes: id da pessoa, id da nação, chave); ponteiro e valor; grava (montar_listas_de_posicao, ordenar_listas_de_posicao)
- Shortlist_Manager +0x98: as 16 contas; +0xd8: o tamanho dos vetores; valor; grava (montar_listas_de_posicao), lê (ordenar_listas_de_posicao)
- Shortlist_Manager +0/+0x10: o vetor de pares (pessoa, clube) ordenado pela pessoa e a conta; +0xdc: as entradas de 9 bytes dos clubes (+0 as 40 vagas, +8 o estado: 100 suspenso); ponteiro e valor; lê (clubes_interessados_no_jogador)
- vaga da lista do clube (0x25 bytes) +7: quanto o clube põe no jogador; +0xf: o jogador; valor e id; lê (clubes_interessados_no_jogador)
- staff +0x1a: a nação; ponteiro; lê e grava a sorteada se nula (montar_listas_de_posicao)
- staff +0x39, +0x52, +0x5f, +0x61: o clube, o valor, o humano ativo, a ficha de jogador; ponteiro e valor; lê
- contrato +4: o clube; +0x3d: o estatuto; +0x4e: bits 0..3; valor; lê (montar_listas_de_posicao, ordenar_listas_de_posicao)
- clube +0x57: a divisão (marca em [0xadadfc]); +0x80: a reputação; ponteiro e valor; lê (ordenar_listas_de_posicao)
- [0xae1080] o último contrato buscado; ponteiro; grava
- Shortlist_Manager +0/+4: os vetores de pares (id da pessoa, id do clube) pela pessoa e pelo clube; +8/+0xc as capacidades; +0x10/+0x14 as contas; ponteiro e valor; lê e grava (acrescentar_na_lista_de_observacao)
- registro de 79 bytes da pessoa ([0xae2d0c]) +0xb: nibble baixo, em quantas listas da IA ela está; valor; lê e grava (acrescentar_na_lista_de_observacao)
- contrato +0x2d/+0x2f: o fim (dia do ano, ano); +0x35: o tipo; +0x3d, +0x49, +0x4f: estatuto e marcas; valor; lê (prever_resposta_do_vendedor, periodo_do_emprestimo)
- clube +0x53: a nação (regras de transferência); +0x80: a reputação; +0xcf: o técnico; +0x1db; ponteiro e valor; lê
- ficha de jogador (staff +0x61) +0xb, +0xf; staff +0x52: o valor; valor; lê (prever_resposta_do_vendedor)
- TRANSFER_MANAGER +0x8ac: as 28 regras de transferência (+0xa a nação; método +0x28 o período do empréstimo); ponteiro; lê (periodo_do_emprestimo)
- [0xae2c90] a data de hoje (dia do ano, ano); valor; lê (periodo_do_emprestimo)
- oferta de transferência +4: a lista dos jogadores da troca; +8: o clube comprador; +0xc: o jogador; +0x11: o cargo; +0x1f: empréstimo; +0x20: a vaga; +0x28: a oferta ligada; ponteiro, id e valor; lê (prazo_da_resposta_a_oferta, tirar_oferta_de_transferencia)
- TRANSFER_MANAGER +0/+8: as ofertas e a conta; +0xc/+0x14: as ligadas e a conta; +0x85e: a menor vaga livre; +0x86e: o registro de 8 bytes por pessoa (+1 ofertas, +2 da troca); +0x946: a lista de empréstimos; ponteiro e valor; lê e grava (tirar_oferta_de_transferencia)
- TRANSFER_MANAGER +0x872: a lista das prestações (+0 valor, +4 clube que paga, +8 clube que recebe, +0xc quantas faltam); ponteiro e valor; lê e grava (pagar_prestacoes_das_transferencias)
- finanças do clube (0x5a0590) +0x1c: a figura das prestações; valor; lê e grava (pagar_prestacoes_das_transferencias)
- TRANSFER_MANAGER +0x8ac: as 28 regras (+0xa a nação); +0x856: o gerente da lista de observação; ponteiro; lê (prazo_da_resposta_a_oferta)
- staff +0x18: a idade; ficha +7: a habilidade; registro de 79 bytes +0xb (nibble baixo): em quantas listas; contrato +0x3d: o estatuto; clube +0x80: a reputação; valor; lê (prazo_da_resposta_a_oferta)
- [0xae1080] o último contrato buscado; ponteiro; grava
- disciplina +4/+8/+0xa: vetor das suspensões (20 bytes), capacidade e conta (palavras); ponteiro e valor; lê e grava (acrescentar_suspensao)
- suspensão +0 jogos (0xff por dias), +2 dias, +0xc..+0xf as 4 regras, +0x13 bit 0 estendida; valor; lê e grava (acrescentar_suspensao)
- disciplina +0xc/+0x10/+0x12: índice das nações (8 bytes: id, nação), capacidade e conta (palavras); ponteiro e valor; lê e grava (acrescentar_suspensao)
- disciplina +0x14: registro de 29 bytes por pessoa (4 vagas de 5 bytes: regra e índice da entrada; +0x1c guardadas, com sinal); lê e grava (tirar_suspensoes_de_fora_do_clube, restaurar_suspensoes_guardadas)
- disciplina +0x1c: lista das suspensões guardadas (item: id, +4 vaga, +5 os 5 bytes da vaga); lê e tira (restaurar_suspensoes_guardadas)
- órgão disciplinar +0x17 nação, +0x1b continente (−1 sem); id; lê; entrada de 14 bytes da pessoa (+0 = −1 livre, +6 índice da suspensão); grava −1 em +0
- clube +0x53 nação, nação +0x71 continente; ponteiro; lê. pessoa +0x1a nação; lê. [0xae1080] o contrato; grava (restaurar_suspensoes_guardadas)
- estatísticas +0x3c/+0x40/+0x44: a tabela dos idiomas (6 bytes: +0 id da pessoa, +4 idioma, +5 fluência 0..10), capacidade e conta; ponteiro e valor; lê e grava (acrescentar_idioma, melhorar_idioma)
- estatísticas +0x30: registro de 39 bytes por pessoa (+0..+0x14 os 6 índices de categoria, +0x18/+0x1c histórico, +0x20/+0x22 totais da liga, +0x24..+0x26 notas recentes); lê e grava (zerar_estatisticas_do_jogador)
- estatísticas +4: registros de categoria de 32 bytes (+0xe = −1 livre); +0x38 quantos livres; grava (zerar_estatisticas_do_jogador)
- pessoa +0 id, +0x61 jogador; lê
- seleções +4: registros de 110 bytes por nação (+4 a competição atual, −1 sem); id; lê e grava (definir_competicao_da_selecao)
- seleções +8: a competição na fila por nação (4 bytes); id; grava (definir_competicao_da_selecao)
- [0xae23d0] competições (107 bytes, +0 id); [0x9cf78c..0x9cf964] as fases finais grandes; id; lê
- seleções +0x24: tabela de um byte por clube (0, 1 jogo da liga, 2 outro, 3 nação marcada; n = [0xae235c] − 2 × [0xae2350]); ponteiro e valor; grava (marcar_clubes_com_jogo)
- 0xb63df0: 0x21 entradas de 0x48 bytes (+0 nação, +0x14 grupo das listas); nação +0x11c marcada; jogo +0x14 competição, +0x1c/+0x20 clubes; clube +0x53 nação, +0x57 liga, +0x60; lê (marcar_clubes_com_jogo)
- disciplina +0x14: registro de 29 bytes da pessoa (4 vagas, +0x1c guardadas); grava 0xff e conta 0; +0x1c lista das guardadas; tira (tirar_jogador_da_disciplina)

**Interessa a:** CMRTE, SGE, GPF2, CMScout
