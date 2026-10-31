# Os eventos financeiros da semana (finance.cpp): FinanceTable::process_club_finance_events 0x59ed70 (pelo passe diário
# das finanças 0x59b9c0: na virada da semana, semana = 1, para todo clube de liga com reputação de 5000 ou nação ativa;
# nos outros dias só o clube em crise, semana = sorteio zero), FinanceTable::try_board_debt_payment 0x59dd40 e
# FinanceTable::try_cash_injection 0x59e5a0 (as duas pela 0x59ed70). Iguais no cm0102-1080.exe. Sem SEH; a 0x59ed70 tem
# quadro de 0x208 bytes (o elenco de 64 vagas da 0x541f90 e a caixa de erro do clube nulo: a original); x87 em precisão
# dupla nas outras duas (_ftol 0x945640; a 0x59e5a0 compara o valor com reputação²/4 no registrador). A 0x59ed70 chama
# originais que sorteiam e alocam (o impulso 0x59e050, a troca da diretoria 0x59e940, as notícias 0x59fc60/0x5a02f0/
# 0x59ffb0/0x59f900, os contratos 0x4d0c60); as outras duas, a notícia 0x59f300 e a verba 0x598800 só quando o evento
# acontece. Comparadores: a 0x59ed70 só a original (a cópia leva o ramo); a 0x59dd40 e a 0x59e5a0 com a cópia em ensaio
# (sem notícia nem verba) e a original a partir do mesmo estado: sem evento, registro de finanças e sorteio inteiros;
# com evento, a devolução e os campos gravados.
SAVE = False
ROTINAS = [('eventos_financeiros_da_semana', 0x59ed70, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('diretoria_paga_a_divida', 0x59dd40, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('injecao_de_dinheiro', 0x59e5a0, [0x83, 0xec, 0x0c, 0x53, 0x55])]
DOCS = ('os eventos financeiros da semana: o passe dos eventos do clube, a diretoria que paga a dívida e a injeção de dinheiro',
        'the weekly finance events: the club events pass, the board paying the debt and the cash injection',
        [('eventos_financeiros_da_semana', 'FinanceTable::process_club_finance_events', '0x59ed70', 'finance.cpp', 'thiscall, ret 8',
          'pela faixa das finanças: encerra ou abre a crise (+0x165), move o aperto (+0x166), tenta o impulso, a dívida, a injeção e a troca da diretoria, desgasta o centro de treino; com aperto acima de 50, tira o aquecimento ou fecha 10% dos lugares',
          'by the finance band: ends or opens the crisis (+0x165), moves the squeeze counter (+0x166), tries the boost, the debt, the injection and the board takeover, wears the training ground; with squeeze above 50, removes the heating or closes 10% of the places'),
         ('diretoria_paga_a_divida', 'FinanceTable::try_board_debt_payment', '0x59dd40', 'finance.cpp', 'thiscall, ret 8',
          'clube pobre (faixa −2/−1, ou 0 com saldo baixo): três sorteios; a diretoria paga 450, 300 ou 150 × reputação e manda a notícia',
          'poor club (band −2/−1, or 0 with a low balance): three random draws; the board pays 450, 300 or 150 × reputation and sends the news'),
         ('injecao_de_dinheiro', 'FinanceTable::try_cash_injection', '0x59e5a0', 'finance.cpp', 'thiscall, ret 8',
          'clube de reputação 4750 fora de Gales e das Irlandas: sorteios pela faixa e pelo estádio do clube de +0xbf; o valor pela reputação, até 10% da dívida e 4 milhões; notícia e fim da crise',
          'club of reputation 4750 outside Wales and the Irelands: draws by the band and the stadium of the +0xbf club; the amount by reputation, up to 10% of the debt and 4 million; news and end of crisis')])
CAMPOS = [('finanças do clube (0x167 bytes) +0 saldo (i64), +0xb4 e +0x154 dinheiro recebido da diretoria; valores; lê e grava (diretoria_paga_a_divida, injecao_de_dinheiro)',
           'club finance (0x167 bytes) +0 balance (i64), +0xb4 and +0x154 money received from the board; values; reads and writes (diretoria_paga_a_divida, injecao_de_dinheiro)'),
          ('finanças do clube +0x165 estado (0 normal, 1 crise, 2 aperto), +0x166 contador de aperto (i8); valores; lê e grava (eventos_financeiros_da_semana; a injeção grava 0 no estado)',
           'club finance +0x165 state (0 normal, 1 crisis, 2 squeeze), +0x166 squeeze counter (i8); values; reads and writes (eventos_financeiros_da_semana; the injection writes 0 to the state)'),
          ('clube +0x53 nação, +0x57 liga, +0x69 estádio, +0x6d byte, +0x7f centro de treino, +0x80 reputação, +0x82 marca, +0xbf clube ligado; ponteiros/valores; lê, grava +0x7f e +0x82 (eventos_financeiros_da_semana)',
           'club +0x53 nation, +0x57 league, +0x69 stadium, +0x6d byte, +0x7f training ground, +0x80 reputation, +0x82 flag, +0xbf linked club; pointers/values; reads, writes +0x7f and +0x82 (eventos_financeiros_da_semana)'),
          ('estádio +0xf nível, +0x20 nível, +0x3c lugares, +0x40 resto, +0x44 lugares fechados, +0x4d aquecimento; valores; lê e grava (eventos_financeiros_da_semana; lê +0xf/+0x20 nas outras)',
           'stadium +0xf level, +0x20 level, +0x3c places, +0x40 remainder, +0x44 closed places, +0x4d heating; values; reads and writes (eventos_financeiros_da_semana; reads +0xf/+0x20 in the others)'),
          ('jogador +0x52 valor; contrato +0x4e bit 4; [0xae1080] último contrato; valores; grava (eventos_financeiros_da_semana, crise do clube humano)',
           'player +0x52 value; contract +0x4e bit 4; [0xae1080] last contract; values; writes (eventos_financeiros_da_semana, human club crisis)'),
          ('[0xadadfc] ligas ativas (dword por id); valor; lê (eventos_financeiros_da_semana)',
           '[0xadadfc] active leagues (dword per id); value; reads (eventos_financeiros_da_semana)')]
INTERESSA = ['CMRTE', 'SGE']
