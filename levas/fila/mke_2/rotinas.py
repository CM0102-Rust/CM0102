# O dia dos contratos (CONTRACT_MANAGER::daily_update 0x4c7ac0, contract_manager.cpp, 9.376 bytes; leitor mke do
# resto do mercado, 12/11/2026), partido em pedaços com nome (o mapa: docs/MAPA_MERCADO.md §2.4, provado na
# desmontagem 0x4c7ac0..0x4c9e2e e nas 10 tabelas de salto 0x4c9e34..0x4c9f54, conferidas uma a uma):
# contract_day_fixed_dates (fatia 0), contract_day_expire_talks, contract_day_value, contract_day_loan_end,
# contract_day_rule_change, contract_day_fifa_period, contract_day_expired, contract_day_bosman, contract_day_renewals,
# contract_day_end, contract_day_decisions; a rotina inteira fica desviada neste item só.
# Chamada pelo laço do dia (game_main_day_loop 0x5cd406: hoje, a fatia [0xae2c88], 1) a cada fatia e pela
# game_run_to_start_date (0x5cc9ef); 1548 chamadas até 2002-06-01 (cobertura de 17/10).
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(gerente de contratos, &data, fatia (byte num dword), pessoas (byte num dword)), ret 0xc,
#   quadro SEH (os locais de oferta e as datas só importam para exceção); a fatia vai inteira para 0x4cd9c0;
# - 0x549f70 lea ecx, local; push -1; push ano; push 0xb; push 0x1f -> ret 0x10 (o dia e o ano lidos em 16 bits);
#   0x54c5b0 push n; push &saída; mov ecx, data -> ret 8 (só os 16 bits de n: a conta (25 − lealdade) × 2 da original
#   leva lixo em cima, que não muda o resultado); 0x54c8e0 idem; 0x54cdb0 push +0x29; push +0x25; mov ecx, data ->
#   ret 8; 0x417c10 push origem; push &saída; mov ecx, destino -> ret 8; 0x54a110 lea ecx, +0x51 -> ret 0;
# - cdecl: 0x600a50 (5), 0x53b620 (byte do cargo), 0x5477d0 (pessoa, clube), 0x547840 (pessoa, técnico), 0x53ab90
#   (pessoa, &data; i8), 0x544670 (nação), 0x5444a0 (pessoa, 0), 0x90cfc0, 0x945333 (qsort), 0x66f4e0 (sprintf de
#   0xde1f64 com "Updating game data"), 0x53c190 (clube, pessoa, 0, 1, 0);
# - 0x802f80 mov ecx, 0xb74450; push -1; push passos; push progresso; push texto -> ret 0x10;
# - 0x4d0c60 mov ecx, this; push flag; push -1; push 0; push clube; push pessoa -> ret 0x14; 0x867b50, 0x867d30 (u8),
#   0x866f60 mov ecx, contrato; 0x868210 push 0; push &no_limite; push 0; mov ecx, contrato -> ret 0xc (sempre grava o
#   no_limite); 0x7a7eb0 push pessoa; mov ecx, [0xb7434c] -> ret 4 (u8); 0x7875e0 push clube; mov ecx, [0xb7431c] ->
#   ret 4; 0x8d0a70 push pessoa; mov ecx, [0xde1f54] -> ret 4; 0x8cca10 push 0; push pessoa -> ret 8; 0x8c5cf0 push
#   0; push 0; push pessoa -> ret 0xc; 0x8c5e40 push 0; push pessoa -> ret 8; 0x8c1870 push pessoa -> ret 4; 0x8c16b0
#   push -1; push 1; push pessoa -> ret 0xc; 0x8c1af0 push 0; push -1; push 0; push clube; push pessoa -> ret 0x14;
# - 0x842c00 push 1; push pessoa; mov ecx, [[0xde1f54]+0x856] -> ret 8; 0x849ce0 push 0; push 0; push pessoa; push
#   clube; mesmo ecx -> ret 0x10;
# - 0x4cda70 push 2; push f64 (2 dwords: 1,0 / 1,1 / 1,2 / 1,3 / 1,4 imediatos); push -1; push 0; push 0; push
#   pessoa; mov ecx, this -> ret 0x1c; 0x4d6790 (contrato, período, bosman, aviso) -> ret 0x10; 0x4da590 (pessoa,
#   tipo, largo) -> ret 0xc; 0x4d6d70 (contrato, 0) -> ret 8; 0x4d07e0 (pessoa) -> ret 4; 0x4d49a0 (oferta, 1) -> ret
#   8; 0x4cd9c0 (data, fatia) -> ret 8; todas com ecx = this;
# - 0x7c5fe0 push 0; push -1; push 0; push 0; push -1; push pessoa; mov ecx, [0xb74350] -> ret 0x18; 0x896c60 push
#   1; push 0|1; push pessoa; push clube; mov ecx, [0xde1d08] -> ret 0x10; 0x5a0590 / 0x598370 as finanças.
# Fidelidade: a oferta local da recusa (0x85e410 + a data em +0x51) tem na original os bytes +0x5d..+0x60 e depois de
# +0x61 sem valor (pilha); a cópia zera o local inteiro. A 0x4d49a0 lê +0x35, +0x4f, +0x50, +0x59 e +0x61 (conferido).
# O pedaço da lei Bosman (contract_day_bosman) devolve "próxima pessoa" nos quatro caminhos da original que pulam o
# fim do contrato (0x4c89ac, 0x4c89bb, 0x4c89d9, 0x4c89e4).
# Comparador: só a original (oferece contratos, escreve notícias, põe na fila de livres, sorteia e mexe nas datas de
# decisão; não roda duas vezes).
SAVE = False
ROTINAS = [('contract_manager_daily_update', 0x4c7ac0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8, 0x6a, 0xff])]
DOCS = ('o dia dos contratos: conversas, valores, empréstimos, renovações e o fim do contrato',
        "the contracts' day: talks, values, loans, renewals and the contract end",
        [('contract_manager_daily_update', 'CONTRACT_MANAGER::daily_update', '0x4c7ac0', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'a cada fatia do dia: na fatia 0 recalcula as datas fixas (hoje + 7, 30, 60, 91, 121, 152, 182, 365, 550, 1095); na fatia 2 as conversas de contrato vencem (25 − lealdade) × 2 dias depois da data e, com o sinal 1, passa por todas as pessoas (em 31/12 com a barra "Updating game data"): antes da carga completa só o contrato que vence hoje vira tipo 1; senão o valor (clube humano: jogador com o bit 4 e ficha reavaliado; IA: jogadores só nos dias ímpares; comissão todo dia; limitado pela multa rescisória ativa; sem contrato ou clube, 0), o fim do empréstimo no dia (notícia 6 e end_loan), antes da mudança das regras de transferência a renovação quando a mudança cai em hoje + 7/30/42/60/70/91 (contrato com mais de 180 dias, termina antes de hoje + 1095, clube da IA sem antipatias, menos de 32 anos: status 1, 2 e 3 com reputação > 6750 ou menos de 24 e potencial > 100, 5 com potencial > 100), o período protegido da FIFA (tipo 2 terminando depois de hoje + 182 sem empréstimo: a IA renova por status e o humano recebe a notícia 7), o contrato vencido (tipo 1, 31 dias depois do fim, quer sair ou Bosman, sem ofertas: cargos 12, 13 e 15 saem do elenco, os outros vão à fila de livres), a lei Bosman (tipo 2 com mais de 93 dias, dentro das faixas, reputação > 3750, mundial > 1750, lealdade < 15, 19 a 30 anos, a nação permite, não (clube > 4750 e menos de 23), e querer sair: sai de graça, vai às listas dos clubes e notícia), as listas dos clubes nas faixas ímpares (14 com sorteio de 1 em 20), as renovações da IA (550 dias status 1-3 na lista; 365 dias com notícia 6 e a recusa; faixas 3 e 5 1,2 na lista; faixa 1 1,1 na lista com notícia 5 e a recusa com o bit 0x80; faixa 7 1,3 / 1,1 com notícia 1 e a recusa com o bit 0x80; faixas 9, 11 e 13 1,4 / 1,2 por status; no fim, o tipo 1, notícia 2, a saída Bosman ou a renovação por status com índice de empréstimo, e o listado sem ofertas vai à fila) e o fim do contrato no dia (empréstimo: notícia 5, estatísticas, contrato apagado, folha salarial, fora das escalações e do clube; os outros viram tipo 1 e o Bosman sem ofertas é liberado na hora); por fim, nos dias 36 e 200, as datas de decisão processadas saem do vetor, as pendentes são reordenadas e process_due_decision_dates',
          "at every slice of the day: slice 0 recomputes the fixed dates (today + 7, 30, 60, 91, 121, 152, 182, 365, 550, 1095); slice 2 expires the contract talks (25 − loyalty) × 2 days after their date and, with flag 1, passes over every person (on 31 December with the \"Updating game data\" bar): before the full load only a contract ending today becomes type 1; otherwise the value (human club: a player with bit 4 and a sheet revalued; AI: players only on odd days; staff every day; capped by an active release fee; without a contract or club, 0), the loan end on the day (news 6 and end_loan), before the transfer rules change the renewal when the change falls on today + 7/30/42/60/70/91 (contract over 180 days old, ending before today + 1095, AI club without dislikes, under 32: status 1, 2 and 3 with reputation > 6750 or under 24 and potential > 100, 5 with potential > 100), the FIFA protected period (type 2 ending after today + 182 with no loan: the AI renews by status and the human gets news 7), the expired contract (type 1, 31 days after the end, wanting to leave or Bosman, no offers: jobs 12, 13 and 15 leave the squad, the others go to the free transfer queue), the Bosman rule (type 2 over 93 days, inside the bands, reputation > 3750, world > 1750, loyalty < 15, 19 to 30 years, the nation allows it, not (club > 4750 and under 23), and wanting to leave: he leaves for free, goes on the clubs' shortlists and the news), the clubs' shortlists on the odd bands (14 with a 1 in 20 draw), the AI renewals (550 days status 1-3 on the list; 365 days with news 6 and the refusal; bands 3 and 5 1.2 on the list; band 1 1.1 on the list with news 5 and the refusal with bit 0x80; band 7 1.3 / 1.1 with news 1 and the refusal with bit 0x80; bands 9, 11 and 13 1.4 / 1.2 by status; at the end, type 1, news 2, the Bosman exit or the renewal by status with a loan index, and the listed player with no offers goes to the queue) and the contract end on the day (loan: news 5, stats, contract erased, wage bill, out of the selections and the club; the others become type 1 and a Bosman with no offers is released at once); last, on days 36 and 200, the processed decision dates leave the array, the pending ones are sorted again and process_due_decision_dates")])
CAMPOS = [('gerente de contratos +0x28/+0x2c conversas de contrato (registros de 21 bytes: +8 pessoa, +0xc data), vencem (25 − lealdade) × 2 dias depois da data; lê e grava (daily_update)',
           "contract manager +0x28/+0x2c contract talks (21-byte records: +8 person, +0xc date), expire (25 − loyalty) × 2 days after the date; reads and writes (daily_update)"),
          ('0xae1008 hoje + 550 e 0xae1020 hoje + 1095 (fatia 0); 0xae1070, 0xae1018, 0xae1038, 0xae1028, 0xae1078, 0xae1088 hoje + 7, 30, 42, 60, 70, 91 (recalculadas por pessoa antes da mudança das regras); grava (daily_update)',
           '0xae1008 today + 550 and 0xae1020 today + 1095 (slice 0); 0xae1070, 0xae1018, 0xae1038, 0xae1028, 0xae1078, 0xae1088 today + 7, 30, 42, 60, 70, 91 (recomputed per person before the rules change); writes (daily_update)')]
INTERESSA = ['Tweaks', 'CMRTE']
