# 249ª leva — a copa: quem passa no jogo e no confronto e a ordem dos cabeças de chave no sorteio; a tabela da liga: iniciar a linha do clube, o vetor dos clubes e as posições finais; a mudança de divisão do clube e os embaralhamentos das listas de clubes; a disciplina: contar o jogo cumprido da suspensão, soltar a suspensão e apagar o histórico de suspensões da pessoa; as lesões: sortear a lesão recorrente, recomeçar o histórico de lesões e reiniciar o físico do jogador; os recordes do clube: somar o valor da transferência aos totais de venda e compra, os recordes de jogos e gols na carreira pelo clube e o recorde de nota da temporada; os amistosos: se a competição vale na data, se o clube já tem amistoso perto da data e acrescentar o amistoso planejado

**Rotinas:**
- `decidir_jogo_da_copa()` = 0x51c020 (Cup.cpp, thiscall, ret 4): quem passa num jogo de copa (1 mandante, 2 visitante, 0 ninguém) pelo placar, prorrogação, pênaltis, W.O. e pontos da regra (+0x38), gravando o código de cada lado (+0x3d/+0x3e);
- `decidir_confronto_da_copa()` = 0x51f2f0 (Cup.cpp, thiscall, ret 8): no primeiro jogo marca o vencedor e devolve 3; na volta decide pelo agregado, W.O., prorrogação, gols fora e pênaltis (1 ou 2 o lado que passa, 4 sem decisão);
- `ordem_dos_cabecas_de_chave()` = 0x51ebc0 (Cup.cpp, cdecl): a comparação do qsort das entradas de 7 bytes do sorteio: o cabeça de chave (+6) maior primeiro;
- `iniciar_linha_da_tabela()` = 0x687430 (league.cpp, thiscall, ret 0x10): põe o clube na linha i da tabela: totais zerados, situação 0xff, marca, total de rodadas (+0xa7) se ainda −1, histórico das posições e, na regra 4, os confrontos diretos (a última linha preenche os adversários de todas);
- `vetor_dos_clubes_da_liga()` = 0x687a20 (league.cpp, thiscall, ret 8): um vetor novo com os clubes da tabela e a conta; sem memória, caixa e conta 0;
- `gravar_posicoes_finais_da_liga()` = 0x68aa30 (league.cpp, thiscall): cada clube ainda na divisão da liga guarda a divisão da última temporada (+0x5b) e a posição final (+0x5f);
- `mudar_clube_de_divisao()` = 0x6830b0 (league.cpp, thiscall, ret 0xc): a divisão atual vira a da última temporada e a nova entra; limpa as cláusulas de divisão (e os números da camisa se a nova os usa), abre a vaga de técnico pedida, faz a obra do estádio mínimo da liga e passa a situação 3 a 2;
- `embaralhar_ponteiros()` = 0x4afb40 (comp_util.cpp, cdecl): embaralha um vetor de n ponteiros com 3 × n trocas de pares sorteados;
- `ordenar_e_embaralhar_os_melhores()` = 0x4afbd0 (comp_util.cpp, cdecl): ordena os clubes pela reputação (nulos no fim) e embaralha os min(3 × escolhas, n) primeiros;
- `contar_jogo_da_suspensao()` = 0x556e90 (discipline.cpp, thiscall, ret 0xc): a suspensão da pessoa no órgão da regra: por jogos, a partir do começo (+4/+6) e com conta, +1 no contador (+1); ao chegar ao total (+0), ou por dias quando hoje chega ao começo + dias (+2, 0x54c5b0), vai para o histórico (0x58e340) e é solta (0x556d00);
- `soltar_suspensao()` = 0x556d00 (discipline.cpp, thiscall, ret 8): cada órgão das 4 regras da suspensão (+0xc..+0xf) a esquece (0x7e8590); na quarta, o item da pessoa sai do índice das nações (o último no lugar, qsort); a suspensão fica livre (+0 = 0xff, +2 = −1);
- `apagar_historico_de_suspensoes()` = 0x55e1d0 (discipline.cpp, thiscall, ret 4): o último deslocamento do histórico de suspensões da pessoa (+0x18 do registro de 29 bytes) = −1 (0x58f110);
- `sortear_lesao_recorrente()` = 0x62e500 (injury.cpp, thiscall, ret 8): 1 em 20, a lesão que pode voltar fica recorrente no registro (+0x12 tipo, +0x13 = 0); dos registros com recorrente, o de maior nota (sorteio 0x65 − 5 × nível − propensão, para em 95): jogador de clube grande (reputação > 4250) sem humano e 1 em 3, lesão nova 0x4d/0x4e/0x4f (0x62d250); senão a marca dele cai; em DESLIGADAS desde 29/10/2026 (o desvio, mesmo de passagem, muda o resíduo de pilha que uma original lê depois: a carreira saía de be54a138); a cópia fica no código e não roda;
- `recomecar_historico_de_lesoes()` = 0x631f90 (injury.cpp, thiscall, ret 4): o último deslocamento do histórico de lesões da pessoa (+0x18 do registro de 31 bytes) = −1 (0x58f110);
- `reiniciar_fisico_do_jogador()` = 0x62bfa0 (injury.cpp, thiscall, ret 4): tira os itens do id da lista +0x28, termina a lesão (0x62e700), recomeça o histórico (0x631f90) e o registro de 31 bytes (data de hoje, condição e forma 10000, desgaste 0, sem lesão nem recorrente); o bit 0 do +0xc do registro de 79 bytes cai;
- `somar_totais_de_transferencia()` = 0x448ad0 (club_records.cpp, thiscall, ret 0xc): soma o valor ao recebido do vendedor e ao gasto do comprador na temporada, sobe os recordes de sempre e guarda a notícia de recorde (0x1f, 0x1e) acima de (+0x80 + 500) × 1000;
- `recordes_de_carreira_no_clube()` = 0x448cb0 (club_records.cpp, thiscall, ret 0x10): os recordes de sempre de jogos (+0x247) e gols (+0x261) pela liga do clube, com a referência do jogador (0x7e4920), quando o tipo vale para o clube (0x449590);
- `recorde_de_nota_da_temporada()` = 0x452a90 (club_records.cpp, thiscall, ret 4): o jogador do elenco com a maior média (estatística 0x11) entre os que jogaram ao menos metade dos jogos vira o recorde de nota da temporada;
- `competicao_na_data_do_amistoso()` = 0x5c29b0 (friendly.cpp, thiscall, ret 8): 1 para a competição comum; para o torneio amistoso, só com a data a menos de 7 dias da data da vaga;
- `amistoso_perto_da_data()` = 0x5c45a0 (friendly.cpp, thiscall (this sem uso), ret 8): 1 se o clube já tem amistoso planejado na data, no dia antes ou no dia depois;
- `acrescentar_amistoso_planejado()` = 0x5c4b60 (friendly.cpp, thiscall, ret 0x10): acrescenta o amistoso (data e três ids) ao vetor de 0x14 bytes, que cresce de 100 em 100;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (16):** 14 acusadas. `cpm_confronto_adversario` escapou: iniciar_linha_da_tabela 0x687430: com o próprio clube no lugar do adversário, o desempate por confronto direto (League::rank_by_head_to_head 0x682b30, que acha a vaga pelo id do adversário) não acha vaga e deixa a ordem para a regra seguinte; a mudança vale só nas tabelas montadas durante a carreira (32 ou mais últimas linhas com a regra 4, contadores na 249ª DLL) e até 2002-06-01 nenhum empate decidido por ela mudou a carreira; no jogo comparando o comparador roda só a original; a causa exata não foi isolada. `cpm_vetor_conta` escapou: vetor_dos_clubes_da_liga 0x687a20: o único chamador é o prêmio de técnico do ano da nação (0x768940, nation_awards.cpp, pela vtable +0xa0), que ordena os clubes pelos pontos do técnico (0x86fab0) e só segue com 3 candidatos; sem o último clube da tabela (8 ou mais chamadas) os candidatos são os mesmos e o prêmio não entra na carreira. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (2f65fd6b68d1027b).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (be54a1386f51073a).

**Refazer**, sobre o commit da 248ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 249`

**Campos tocados:**
- jogo +0x38: a regra (0x10 pontos, 2 prorrogação, 0x21 pênaltis, 0x800/0x1000 W.O., 0x2000 códigos −3, 0x4000 e 4 gols fora); valor; lê (decidir_jogo_da_copa, decidir_confronto_da_copa)
- jogo +0x3d/+0x3e: o código do resultado de cada lado (1, 2 venceu, 3 prorrogação, 4/5 pênaltis, 6 gols fora, 8 W.O.); valor; lê e grava (decidir_jogo_da_copa, decidir_confronto_da_copa)
- jogo +0x43/+0x44, +0x47/+0x48, +0x49/+0x4a: o placar, a prorrogação e os pênaltis; valor; lê (decidir_jogo_da_copa)
- confronto +0x14..+0x1b e +0x1e..+0x25: placar, prorrogação e pênaltis da volta e da ida; +0x33 o jogo (1 = volta); valor; lê (decidir_confronto_da_copa)
- copa +0xa7/+0x3e: as entradas de 6 bytes dos times (+0 clube, +4 cabeça de chave) e a conta; ponteiro e valor; lê (decidir_jogo_da_copa)
- entrada do sorteio +6: o cabeça de chave; valor; lê (ordem_dos_cabecas_de_chave)
- liga +0xa7: o total de rodadas (−1 até a primeira linha); valor; lê e grava (iniciar_linha_da_tabela)
- liga +0xc5: as 4 regras de desempate (4 = confrontos diretos); valor; lê (iniciar_linha_da_tabela)
- linha da tabela (0x3b bytes) +0, +4, +5, +9, +0xd..+0x36, +0x37..+0x3a: clube, posição, confrontos diretos, histórico, totais, situação, marca, 1, clubes; ponteiro e valor; grava (iniciar_linha_da_tabela)
- clube +0x64: situação (3 passa a 2 ao entrar na tabela); valor; lê e grava (iniciar_linha_da_tabela)
- clube +0x57/+0x5b/+0x5f: divisão, divisão da última temporada e posição final; ponteiro e valor; lê e grava (gravar_posicoes_finais_da_liga)
- clube +0x57/+0x5b: divisão e divisão da última temporada; ponteiro; lê e grava (mudar_clube_de_divisao)
- clube +0x64: situação (3 passa a 2); valor; lê e grava (mudar_clube_de_divisao)
- clube +0xcf, +0x69: o técnico e a ficha do estádio; ponteiro; lê (mudar_clube_de_divisao)
- competição +0x43, +0xe2, +0xe4: o tipo (2 = liga) e o estádio mínimo (lugares, capacidade); valor; lê (mudar_clube_de_divisao)
- [0xadadfc] as competições carregadas pelo id; ponteiro; lê (mudar_clube_de_divisao)
- clube +0x80: reputação (a ordem do qsort); valor; lê (ordenar_e_embaralhar_os_melhores, pela 0x4b07d0)
- disciplina +4: vetor das suspensões (20 bytes: +0 jogos ou 0xff por dias, +1 jogos cumpridos, +2 dias, +4 dia e +6 ano do começo, +0xc..+0xf as 4 regras); ponteiro; lê e grava (contar_jogo_da_suspensao, soltar_suspensao)
- disciplina +0xc/+0x12: índice das nações (8 bytes: id, nação), conta (palavra); lê e grava (soltar_suspensao)
- disciplina +0x14: registros de 29 bytes por pessoa (+0x14 começo e +0x18 último deslocamento do histórico); lê e grava −1 em +0x18 (apagar_historico_de_suspensoes); +0x18 arquivo do histórico (contar_jogo_da_suspensao)
- órgão disciplinar: entrada de 14 bytes da pessoa (+6 índice da suspensão, −1 sem); lê (contar_jogo_da_suspensao); grava −1 pela 0x7e8590 (soltar_suspensao)
- pessoa +0x1a nação, +0x1e segunda nação, +0x61 jogador; [0xae2c90] data do jogo; lê
- físico (INJURY [this]): registro de 31 bytes por pessoa: +0 data, +8 condição, +0xa forma, +0xc desgaste, +0xe índice da lesão (−1 sem), +0x12 tipo recorrente (0xff sem), +0x13 nível, +0x14/+0x18 histórico, +0x1d; valores; lê e grava (sortear_lesao_recorrente, recomecar_historico_de_lesoes, reiniciar_fisico_do_jogador)
- INJURY +0x28: lista de itens por id; lê e tira (reiniciar_fisico_do_jogador); +0xc arquivo do histórico (recomecar_historico_de_lesoes)
- [0x9e7ae0 + 13 × tipo]: a lesão pode voltar; valor; lê (sortear_lesao_recorrente)
- pessoa +0x39 clube, +0x61 jogador; jogador +0x2d propensão a lesões; clube +0x80 reputação; lê (sortear_lesao_recorrente)
- [0xae2d0c] registro de 79 bytes +0xc: bit 0; grava 0 (reiniciar_fisico_do_jogador)
- Club_Records registro da temporada e de sempre +0x365 gasto em transferências, +0x369 ano, +0x36b recebido, +0x36f ano; valor; lê e grava (somar_totais_de_transferencia)
- Club_Records +0x24 fila de notícias de recorde (6 bytes: id do clube, tipo), +0x28 capacidade, +0x2c conta; lê (somar_totais_de_transferencia)
- clube +0x80 base do mínimo da notícia de recorde (palavra); valor; lê (somar_totais_de_transferencia)
- Club_Records registro de sempre +0x247 recorde de jogos na liga, +0x249 referência do jogador (0x18), +0x261 recorde de gols, +0x263 referência; valor; lê e grava (recordes_de_carreira_no_clube)
- Club_Records +0x1c entrada do clube (0x6a bytes) +0x10 jogos da temporada; valor; lê (recorde_de_nota_da_temporada)
- Club_Records registro da temporada +0x1f1 nota (float), +0x1f5 referência do jogador, +0x20d jogos, +0x20e zerado; grava (recorde_de_nota_da_temporada)
- clube +0xd7 elenco (50 ponteiros); pessoa +0 id, +0x39 clube, +0x61 estatísticas; [0xae2364] primeiro id de técnico humano; [0xb74350] PlayerStats; lê (recorde_de_nota_da_temporada)
- Friendly +4 competição do amistoso; ponteiro; +0xaf primeiro id (byte); +0xb1 vagas do torneio (0x69 bytes: +0 ocupada, +4 data); lê (competicao_na_data_do_amistoso)
- [0xae2378] número de competições; valor; lê (competicao_na_data_do_amistoso)
- candidato a amistoso +0x12 lista de amistosos planejados (itens: dia +0, ano +2); lê, grava o cursor +0xa (amistoso_perto_da_data)
- Friendly +0x34d8 amistosos planejados (0x14 bytes: data +0, ids +8, +0xc, +0x10), +0x34e0 conta, +0x34e4 capacidade; grava (acrescentar_amistoso_planejado)

**Interessa a:** CMRTE, SGE, GPF2, CMScout
