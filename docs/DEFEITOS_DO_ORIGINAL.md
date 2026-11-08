# Defeitos do original

A lista dos defeitos (bugs) do CM 01/02 3.9.68 que achamos ao portar, ou que a comunidade documentou e conferimos no binário. A correção entra no exe todo em Rust, no fim da migração.

**Regra.** Enquanto o exe original roda junto, a cópia Rust reproduz o defeito: o critério dos testes é o resultado que o jogo produz, e a comparação rotina a rotina não aceita divergência. Exceção: as linhas marcadas "já corrigido", que o Lucas aprovou corrigir já. Cada leva que achar um defeito acrescenta uma linha aqui e no `DEFEITOS_DO_ORIGINAL.en.md`.

**Situação:**
- **reproduzido 1:1:** a cópia Rust faz o mesmo que a original.
- **reproduzido (cópia usa 0):** a original lê sobra de pilha; a cópia Rust não tem como repetir esse valor e usa 0, um valor que o jogo também produz.
- **na original:** a rotina ainda não foi portada.
- **já corrigido:** a cópia Rust já corrige, com a aprovação do Lucas.
- **corrigir no fim, aprovado:** o Lucas já decidiu a correção, que entra no exe todo em Rust.
- Nas outras linhas a correção se decide no fim.

[inferência] marca o que não foi provado no binário ou no jogo. "Comunidade" é defeito publicado no fórum ou corrigido pelo `CM0102Patcher` (`referencias/CM0102Patcher`, só citado, sem código copiado).

## Nomes e textos

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x79cee0 (a chamada em 0x79d098) | `renovar_arbitros`, officials_manager.cpp | Ao renovar um árbitro velho, sorteia o nome e o sobrenome com a função do primeiro nome (0x539c60) duas vezes. A do sobrenome é a 0x53a1b0. | Os árbitros renovados ficam com dois primeiros nomes. É o defeito dos "sobrenomes trocados" dos árbitros. | reproduzido 1:1 (261ª) | `docs/CAMPOS.md` (comunidade, conferido); `motor.rs` `renovar_arbitros`. O `CM0102Patcher` não corrige: o RefereePatcher dele só mexe no `officials.dat`. |
| 0x539750 | `montar_indice_de_nomes`, Database.cpp | Monta o índice de nomes por nação e não grava o último grupo de cada tabela (primeiros, segundos e comuns). | [inferência] A nação da última linha de cada tabela fica sem índice de nomes, e as pessoas geradas para ela não tiram nome da própria lista. | reproduzido 1:1 | `motor.rs` `montar_indice_de_nomes` |
| — (passe das pessoas da carga, `load_database`) | Database.cpp | Pessoa com nome nulo recebe o primeiro nome comum vazio, e isso vale para os três nomes: o nome e o sobrenome nulos também vão para a tabela de nomes comuns. | [inferência] A pessoa sem nome na base aparece com o nome errado ou vazio. | na original | `docs/MAPA_DATABASE.md` §2 e a tabela de riscos |

## Contas e arredondamentos

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x868830 | `resultado_nos_registros_dos_tecnicos`, staff_records.cpp | No visitante, +0xbe/+0xba recebem o [esp+0x10] da rotina, que é a nota do mandante (ou o bônus dele, sem registro). | Os pontos do técnico visitante (na seleção ou fora dela) saem com a nota do outro técnico. | reproduzido 1:1 (263ª) | `motor.rs` `resultado_nos_registros_dos_tecnicos` |
| — (carga do banco) | Database.cpp | A reputação do clube (+0x80) é gravada como `(char)(sorteio(4000)+1)`: um sorteio de até 4000 truncado a 8 bits. | [inferência] O clube que cai nesse caminho fica com reputação entre −128 e 127 em vez de até 4000. | na original | `docs/MAPA_DATABASE.md` (tabela de riscos, "erros do original a copiar") |
| 0x5986b0 (o ponto 0x5986e3) | `faixa_de_riqueza`, finance.cpp | Comunidade: o patch "EnsureCashDoesNotResetToZero" desvia em 0x5986e3 e repõe 2 bilhões quando o saldo cai abaixo de −1,5 bilhão. | [inferência, pelo patch] O saldo de 32 bits do clube muito rico dá a volta e fica negativo. | reproduzido 1:1 | `CM0102Patcher` (Patcher.cs, "bugfixes"; deslocamento 0x1986e3 no arquivo) |

## Atributos e jogadores

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x6ec010 (o salto em 0x6eda39) | `atualizar_atributos`, match_pl.cpp | Com CA de 125 ou mais, o salto em 0x6eda39 pula o cálculo da criatividade, que fica com o valor do atributo anterior. | A criatividade do jogador bom na partida sai errada. | reproduzido 1:1 | `docs/ENDERECOS_COMUNIDADE.md` (MadScientist); `CM0102Patcher` "Unlock your creativity" (0x2eda3a no arquivo) |
| 0x6ef644, 0x6ef692 (na rotina 0x6ef372) | match_pl.cpp [inferência] | Comunidade: o patch "MarkingAndPositionClampTo100" limita a 100 dois valores de marcação e posicionamento. | [inferência, pelo nome do patch] Os valores passam de 100 e distorcem a marcação e o posicionamento. | na original | `CM0102Patcher` (Patcher.cs, "bugfixes") |
| 0x6d1cc0 | `fechamento`, match_eng.cpp | Lado sem minutos jogados fica com a média de notas do lado anterior; no primeiro lado, a média é sobra de pilha. | A reputação (0x6d3610, 0x6d3de0) recebe uma média que não é a do time. | reproduzido (cópia usa 0) | `motor.rs` `fechamento_com`; `ZERADAS` (histórico) |
| 0x7a65d0 | `fila_de_regens`, player_regen.cpp | O jogador que vira jogador-treinador entra em todas as vagas vazias de treinador do clube: falta um `break` no laço. | O clube conta o mesmo treinador várias vezes, fica sem vaga para contratar treinador e não recebe outro jogador-treinador. | reproduzido 1:1 | `docs/COMISSAO_REPETIDA.md` §1 e §2 |
| 0x53c190, 0x53bd10, 0x53b9a0 | Database.cpp | A saída do clube (0x53c190) tira só a primeira ocorrência da pessoa. A inclusão (0x53bd10) não confere se a pessoa já está na lista, e 0x53b9a0 trata "já está aqui" como vaga. | Junto com a linha acima: o treinador que sai deixa cópias "fantasmas" nas outras vagas. | na original | `docs/COMISSAO_REPETIDA.md` §1 e §2 |
| — | — | Comunidade: o convocado fica preso na seleção e não volta ao clube. | O jogador some do clube. | na original | `CM0102Patcher` (ferramenta GoHome) |

## Lixo de pilha / variável lida antes de escrita

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x697c40 (leituras em 0x6984ad, 0x6986ff, 0x6987d6) | `MANAGER_MANAGER::weekly_board_confidence_update` | A quantidade de clubes da liga (N, [esp+0x2b]) é lida antes de ser escrita (0x6989d0). O 1º clube lê lixo; os seguintes, o N da liga do clube anterior. | N é a régua de posição da diretoria (pressão se posição ≥ N/4; bônus abaixo de N/5 ou 3N/10). A confiança da diretoria fica enviesada. | corrigir no fim, aprovado (26/10): usar o N da liga do próprio clube | `docs/COMO_CONTINUAR.md`; memória `correcoes-do-original` |
| 0x76c120 | `escolher_elenco_da_selecao`, national_teams.cpp | Só há códigos de posição para elencos de 22, 24 e 26. Nos de 23 (Copa de 2002) e 25, as vagas a partir da 22ª ficam com lixo de pilha, que vai à escala (0x6e2000). | A escalação da seleção na Copa depende de lixo. | já corrigido (266ª, aprovado em 01/11): as vagas recebem 0x202, 0x88 e 0x808 | `levas/fila/cqc_12/FORA.md`; `docs/COMO_CONTINUAR.md` |
| 0x6c9cd0 (decisão em 0x6e9820) | `disputa`, match_eng.cpp | A decisão do juiz sai antes de escrever a vantagem em vários caminhos; a disputa lê o que sobrou na pilha ([esp+0x16]). | Em partida assistida com narração, o lixo decide a narração 0x2168 e um sorteio. | já corrigido (cópia usa 0; decisão do Lucas de 14/10) | `docs/COMO_CONTINUAR.md`; `wiki/log.md` (14/10) |
| 0x6dbc50 | `escalar_automatico`, match_man.cpp | No modo 2, nunca escreve o curto da 1ª vaga extra e o passa à escala (0x6e2000 → 0x6e2f20). | Bônus da escalação somados por bits de lixo. | reproduzido (cópia usa 0) | `docs/COMO_CONTINUAR.md` |
| 0x84be20 (lida pela 0x6beeb0) | simulated_stats.cpp | A ficha do jogo simulado é montada na pilha e a palavra +0x38 nunca é gravada. A simulação rápida a lê como regra de prorrogação e pênaltis. | A regra de desempate do jogo simulado vem do lixo. | na original | `docs/COMO_CONTINUAR.md` (212ª, histórico) |
| 0x70c040 | `desfecho_do_chute`, match_pl.cpp | Com o próprio goleiro chutando, lê o rebote e a saída da defesa (b13/b16) sem valor. | O desfecho do lance vem do lixo. | reproduzido (cópia usa 0) | `motor.rs` `desfecho_do_chute` |
| 0x7c1e30 | `estatistica_da_categoria`, player_stats.cpp | Nas categorias combinadas (100 a 102), sem o registro 4 ou 5 lê locais sem valor. | Entra na escalação (0x6e2f20), no MATCH_PLAYER::setup e na reputação do clube. | reproduzido (cópia usa 0; o comparador deixa esses casos com a original) | `motor.rs` `estatistica_da_categoria` |
| 0x62c080 | `efeitos_fisicos_da_partida`, injury.cpp | A gravidade é uma local que passa de uma batida para a outra; sem batida acima de 51 antes, lê sobra de pilha. | A gravidade da lesão vem do lixo. | reproduzido (cópia usa 0) | `motor.rs` `efeitos_fisicos_da_partida` |
| 0x7a79c0 | `aposentadoria_no_inicio`, player_regen.cpp | Sem cargo (+0x3d) e sem +0x28, o limite de idade de quem não é jogador usa o byte baixo do ponteiro da saída, lido da vaga errada da pilha. | Idade de aposentadoria arbitrária. | reproduzido 1:1 | `motor.rs` `cpr8_limite_sem_jogador` |
| 0x85a850 | `gravar_numero_de_camisa`, squad_manager.cpp | Sem nenhuma posição boa na ficha, lê a lista de números preferidos de 4 bytes sem valor na pilha. | Número de camisa preferido arbitrário. | reproduzido (cópia usa a lista vazia) | `motor.rs` `cql2_preferidos` |
| 0x69ba50 | `vaga_de_tecnico`, manager_manager.cpp | A nota anterior (+0x12) fica sem valor quando o escolhido é o auxiliar ou o de +0x19f. | [inferência] Grava uma nota arbitrária. | reproduzido (cópia usa 0) | `motor.rs` `vaga_de_tecnico` |
| 0x754a30 | `boatos_de_transferencia`, media.cpp | Com jogador sem contrato, o técnico do dono fica o da volta anterior; na primeira volta é sobra de pilha. | [inferência] Boato com o técnico errado. | reproduzido (cópia usa 0) | `motor.rs` `boatos_de_transferencia` |
| 0x6ab2d0 | `texto_da_diretoria_sobre_os_jogadores`, manager_manager.cpp | Com o 1º id −1 e outro válido, lê o lugar 0 sem valor. | [inferência] Texto da diretoria com jogador errado. | reproduzido 1:1 (esse caso roda a original) | `motor.rs` `texto_da_diretoria_sobre_os_jogadores` |
| 0x8bed60 | `fazer_oferta_de_emprestimo` | Bytes da oferta (0x8e5680) e do contrato (0x85e410) locais que os construtores não preenchem vão para a oferta guardada. | Quem lê não foi mapeado. | portada (253ª); o que a cópia grava nesses bytes não foi conferido aqui | `docs/PENDENCIAS.md` |

## Competições e calendário

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x411ce0 (o ponto 0x411d82) | `cqj8_regras_da_nsl`, aus_nsl.cpp | A competição não é zerada no fim da temporada (falta gravar −1 em +0x30 da fase), e a divisão abertura/encerramento sai 36/5 jogos em vez de 24/10 (+0x3c). | Liga australiana com o mesmo campeão todo ano. | reproduzido 1:1 | `docs/CAMPOS.md` (comunidade, tópico 1540 p. 9, conferido) |
| 0x797474, 0x40aede, 0x40cfce | oceania_club_champ.cpp, asia_club_champ.cpp, asia_cup_winner.cpp | Disputa de terceiro lugar (3 rodadas finais), às vezes marcada depois da final. | Pode travar a competição. | na original | `docs/CAMPOS.md` (comunidade, conferido) |
| 0x585a5a (conferido), 0x5862a4, 0x633e03 | european_cup.cpp (0x585a5a, 0x5862a4); 0x633e03 [arquivo não conferido] | Acrescentam inscritos à copa no meio da temporada sem conferir a capacidade do livro (+0x38). | [inferência] Escrita além do livro de inscritos. | na original | `docs/CAMPOS.md` (comunidade) |
| 0x91df0e (anos em 0x911c4d...0x92036e) | eliminatórias da Copa | Em bancos com início antecipado, a Ásia sorteia antes da Europa e 0x91df0e lê um grupo vazio. | Só com bancos editados; o `CM0102Patcher` tem o "fixworldcuppre2000". | na original | `docs/CAMPOS.md` (tópico 10790) |
| 0x584990 | `tabela_da_qualificacao_da_liga_dos_campeoes`, european_cup.cpp | Com +0x36 = 0, lê antes da lista (índice negativo). | [inferência] Clube lixo na qualificação, se o caso ocorrer. | reproduzido 1:1 | `motor.rs` `cpy5_qualificacao` |
| 0x641dc0 | `resultado_do_desempate_da_serie_a`, ita_ser_a.cpp | Lê o clube da linha seguinte mesmo na última linha (4 bytes além da tabela). | [inferência] Comparação com um valor de fora da tabela. | reproduzido 1:1 | `motor.rs` (comentário na rotina) |
| 0x6624f0 | `cqn12_copa_do_imperador_inscritos`, jap_emp_cup.cpp | Sem clubes, lê o vetor no índice −1. | [inferência] Inscrito lixo. | a cópia usa 0 e cai na caixa de erro | `motor.rs` (comentário na rotina) |
| 0x4c11a0 | `cqi11_concacaf_classificados` | Com clubes de menos numa lista (mexicanos, centro-americanos, caribenhos, MLS), lê sobra de pilha. | [inferência] Classificado lixo, só em banco com poucos clubes. | reproduzido (cópia usa 0) | `motor.rs` `compare_cqi11_concacaf_classificados` |
| 0x5b91c0, 0x5bd600 | `construir_cfa`, `construir_divisao_de_baixo_francesa` | A lista local de 72 ou 40 ponteiros não é zerada; sem clubes bastantes, lê lixo. | [inferência] Só em banco com poucos clubes. | reproduzido (cópia usa 0) | `motor.rs` `cqn4_construir` |

## Save e banco

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x5315a0 | `resolver_indices_do_banco`, Database.cpp | Com a última divisão do clube (+0x5b) acima da quantidade, zera +0x60 (que vira a competição 0) e deixa +0x5b com o id cru. | O clube fica ligado à competição 0 e com um campo que não é ponteiro. | reproduzido 1:1 | `motor.rs` (comentário "erro do original"); `docs/MAPA_DATABASE.md` |
| 0x5315a0 | `resolver_indices_do_banco`, Database.cpp | Com o estádio vizinho (+0x48) acima da quantidade, zera a cidade (+0x38) e deixa +0x48 com o id cru. | O save lê `[ptr]` de todo campo não nulo: [inferência] desreferencia o id como endereço e cai. | reproduzido 1:1 | `docs/MEMORIA_PREFERENCIAS_ESTADIO.md` |
| 0x8e7bec | — | Lê os dados de jogador (+0x61) de uma pessoa com clube sem testar nulo. | Banco com pessoa sem esses dados derruba o jogo. | na original | `docs/CAMPOS.md` (proteção da comunidade, conferido) |
| 0x52dad0 | `save_database` | Grava no `index.dat` 408 bytes de locais sem valor (lixo de ponteiro, a partir de +0x5cb). | Sem efeito no jogo; os saves não saem iguais. | na original | `motor.rs` (`save_database_for_game`); `docs/PENDENCIAS.md` |
| 0x82e430 | `gravar_jogo` (`save_game_data`), Setup.cpp | O que a 0x591f80 não escreve no resumo (o clube dos 16 técnicos não humanos, o resto dos textos) é sobra de pilha gravada no `general.dat`. | Sem efeito no jogo; 13 bytes do `general.dat` mudam a cada gravação. | reproduzido (cópia usa 0) | `motor.rs` `gravar_jogo`; `docs/COMO_CONTINUAR.md` |
| 0x5c5f40, 0x51a150, 0x51cb90 | amistoso e jogo de copa | O registro de partida é montado num local de 0x4f sem gravar tudo; a sobra vai para o `fixtures_<ano>.tmp`. | Sem efeito no jogo. | na original | `docs/MAPA_SAVE.md` |
| 0x828548..0x828561 | `create_game` | Cada registro de pessoa começa com `+0xb = lixo & 0x80`. | [inferência] O bit 0x80 de +0xb começa arbitrário; o bit 0x40 (marca do regen) começa zerado. | na original | `docs/MARCA_DO_REGEN.md` |
| 0x5fb810 | `HUMAN_MANAGER::reset_human_record` | O `STAFF_CONTRACT` temporário (o construtor 0x85e410 escreve só parte) é copiado inteiro para o rascunho de oferta do técnico humano. | Sem efeito no jogo (a tela regrava o rascunho); a sobra vai para o `human_manager.dat`. | na original | `docs/COMO_CONTINUAR.md` |
| 0x933320 | `desempacotar_do_conteiner`, zipdir.cpp | Nos caminhos de erro (sem memória, contêiner que não abre ou de tipo estranho), fecha o item de uma local nunca escrita. | [inferência] Pode fechar um item inválido ao carregar um save estragado. | reproduzido (cópia usa 0) | `motor.rs` `desempacotar_do_conteiner` |

## Outros

| Endereço (VA) | Rotina/arquivo | O defeito | Efeito no jogo | Situação | Fonte |
|---|---|---|---|---|---|
| 0x8ca8a0 | `pagar_transferencia_e_clausulas`, transfer_manager.cpp | Sem vendedor, lê as finanças de um lugar sem valor e cai adiante em [vendedor]. | [inferência] Queda do jogo; o caso não aparece nas carreiras. | a cópia Rust não paga ninguém e não cai | `motor.rs` (comentário na rotina) |
| 0x71f500 | `painel_das_zonas_de_acao`, match_screens.cpp | Com a vista fora de 0 a 4, passa o texto de uma local sem valor. | Título com lixo na tela, fora do uso normal. | reproduzido (cópia usa texto vazio) | `motor.rs` (comentário na rotina) |
| 0x721cd0 | `preencher_estatisticas_do_time`, match_screens.cpp | Com o parâmetro 0x1c fora de 1 e 2, lê a tabela sem valor. | Fora do uso normal. | reproduzido 1:1 (esse caso roda a original) | `motor.rs` (comentário na rotina) |
