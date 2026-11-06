# O Mineiro (Minas Gerais): a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_minas_gerais.cpp; agente C da
# frente das competições, 05/11/2026). O mesmo molde no Mineiro, no Paranaense e no Carioca (cqo_7, cqo_8, cqo_9):
# conferido instrução a instrução (só mudam o fonte e os prêmios). Cada uma roda uma vez na carreira de 8 ligas até
# 2002-06-01 (docs/cobertura_competicoes.txt), no fim do estadual de 2001.
# - 0x434ae0 BraRegMinasGerais::advance_stage: a fase seguinte; ao chegar a 2 monta a final.
# - 0x434b70 BraRegMinasGerais::create_final_stage: a final entre os campeões dos turnos (ou o campeão sem final).
# - 0x434ed0 BraRegMinasGerais::record_season_honours: o histórico e os prêmios (350.000 e 100.000) sem final.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x434b70 da fase seguinte: call direto com o ecx da liga, sem push -> thiscall, ret 0;
# - 0x944e46 (malloc): push 8; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xb2; call; add esp, 4 -> cdecl;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 2; mov ecx, esi -> thiscall, ret 0x10;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (o mesmo construtor da cqn_9, aqui
#   com n = 2 e o primeiro byte 2 em vez de 0; a cópia passa as words limpas);
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, fase -> thiscall, ret 4;
# - 0x9452ca (free): push t; call; push lista; call; add esp, 8 -> cdecl;
# - a virtual +0x30 da fase final (record_season_honours): mov ecx, fase; call [edx+0x30], sem push -> thiscall, ret 0;
# - 0x42bba0: call sem push nem add esp -> cdecl sem argumentos (mapa: cdecl(void));
# - 0x684640 (League::record_season_honours): mov ecx, esi; call -> thiscall, ret 0;
# - 0x5a0590 (FinanceTable::get_club_finance): push clube; mov ecx, [0xae2a38]; call -> thiscall, ret 4 (o valor
#   empurrado antes fica na pilha para a 0x5999a0: push valor; push clube; ... call 0x5a0590; mov ecx, eax; call 0x5999a0
#   -> 0x5999a0 thiscall(finanças, valor), ret 4, como no mapa);
# - 0x48e3e0 (Comp::send_prize_money_news): push −2; push 0; push 0; push posição; push 0; push valor; push clube;
#   mov ecx, esi -> thiscall, ret 0x1c (o valor em 64 bits);
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: a fase seguinte e a final, só a original com o plano (a final aloca e sorteia); o fim da temporada,
# só a original (histórico, prêmios e notícia).
# Determinismo: a lista e as sementes vêm das tabelas; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqo7_fase_seguinte_do_mineiro', 0x434ae0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqo7_final_do_mineiro', 0x434b70, [0x6a, 0xff, 0x68, 0xee, 0x47, 0x95, 0x00]),
           ('cqo7_fim_da_temporada_do_mineiro', 0x434ed0, [0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02])]
DOCS = ('o Mineiro: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada',
        'the Minas Gerais state league: the next stage, the final between the two half-season winners and the end of the season',
        [('cqo7_fase_seguinte_do_mineiro', 'BraRegMinasGerais::advance_stage', '0x434ae0', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 0',
          'O Mineiro tem dois turnos (as fases 0 e 1, cada uma uma liga de pontos corridos) e, se precisar, a final (fase 2): a fase seguinte sobe a fase atual e, no fim do segundo turno, monta a final; depois da última fase é erro (a caixa)',
          'The Minas Gerais state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final (stage 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the last stage it is an error (the box)'),
         ('cqo7_final_do_mineiro', 'BraRegMinasGerais::create_final_stage', '0x434b70', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 0',
          'A final do Mineiro: os campeões do primeiro e do segundo turno se enfrentam em dois jogos [inferência: ida e volta], o campeão do primeiro turno como semente 0; se o mesmo clube vence os dois turnos, é campeão sem final (a temporada acaba com as duas fases, e ele fica com a situação 5, campeão, nas três tabelas)',
          'The Minas Gerais final: the winners of the first and second halves meet over two games [inference: home and away], the first-half winner as seed 0; if the same club wins both halves, it is champion without a final (the season ends with the two stages and it gets status 5, champion, in the three tables)'),
         ('cqo7_fim_da_temporada_do_mineiro', 'BraRegMinasGerais::record_season_honours', '0x434ed0', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 0',
          'O fim da temporada do Mineiro: com final, o histórico vem da final; sem ela, do campeonato, e o campeão ganha 350.000 e o vice 100.000 de prêmio (no saldo, com a notícia do prêmio). Depois, quando os doze estaduais acabaram, os campeões vão para a Copa dos Campeões',
          'The end of the Minas Gerais season: with a final, the honours come from the final; without one, from the league, and the champion gets 350,000 and the runner-up 100,000 in prize money (to the balance, with the prize money news). Then, once the twelve state leagues are over, the champions go into the Champions Cup')])
CAMPOS = [('liga +0x2c: o número de fases; valor; lê (fase seguinte) e grava 2 (final, o mesmo campeão)', 'league +0x2c: the number of stages; value; reads (next stage) and writes 2 (final, same winner)'),
          ('liga +0x30: a fase atual; valor; lê e grava (fase seguinte), grava 1 ou 2 (final), lê (fim da temporada)', 'league +0x30: the current stage; value; reads and writes (next stage), writes 1 or 2 (final), reads (end of season)'),
          ('liga +0xc: as fases (vetor de ponteiros: os dois turnos e a final); ponteiro; lê as duas primeiras, grava a terceira', 'league +0xc: the stages (pointer array: the two halves and the final); pointer; reads the first two, writes the third'),
          ('fase de liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube, +0x37 a situação); ponteiro; lê o clube e grava a situação 5 da primeira linha', 'league stage +0xb1: the table (0x3b-byte rows, +0 the club, +0x37 the status); pointer; reads the club and writes status 5 of the first row'),
          ('liga +0xb1 / +0x3e: a tabela da liga e o número de linhas; ponteiro / valor; lê, grava a situação 5 do campeão', 'league +0xb1 / +0x3e: the league table and its row count; pointer / value; reads, writes status 5 of the champion'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads'),
          ('[0xae2a38] a tabela das finanças; ponteiro; lê (prêmios)', '[0xae2a38] the finance table; pointer; reads (prize money)'),
          ('fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)', 'cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)')]
INTERESSA = []
