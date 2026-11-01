# Os eventos raros das finanças (fin_prm.cpp; os asserts citam finance.CPP), as duas originais que a cpv_6 deixou
# chamando pelo endereço no passe dos eventos da semana (0x59ed70): FinanceTable::try_finance_boost_event (0x59e050,
# 0x544 bytes com x87, tentado toda semana para os clubes da faixa 0 e nos outros estados; "cpv_6 impulso na faixa 0"
# passou de 2048 na base) e FinanceTable::try_board_takeover (0x59e940, 0x42c bytes com x87, clubes endividados);
# passageira: Finance::pay_other_costs (0x5999e0, 0x34 bytes, pela 0x7e5b10). Sem SEH. Iguais no cm0102-1080.exe
# (corpo inteiro das três). Usa os ajudantes da cpv_6 (cpv6_sortear, cpv6_faixa, cpv6_noticia, cpv6_verba,
# cpv6_somar, cpv6_somar_saldo e o CPV6_ENSAIO).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x5986b0 (Finance::get_wealth_band): mov ecx, finanças; sem push; sem add esp -> thiscall(fin).
# - 0x90cfc0 (sorteio): push n; call; add esp, 4 (ou 8 depois de dois) -> cdecl(n).
# - 0x945640 (_ftol) e 0x945ff0 (pow do x87: fild +0x20; fld 2,0) -> `ftol` e o quadrado em f64.
# - 0x598800 (Finance::get_transfer_budget): 5 pushes (0, 1, 0, 0, 0 -> argumentos 0, 0, 0, 1, 0); ecx = finanças; sem
#   add esp -> thiscall, ret 0x14 (cpv6_verba).
# - 0x59f300 (finance_post_finance_boost_news): 4 pushes (tipo, verba, valor, clube); ecx = a tabela (guardada na
#   entrada); sem add esp -> thiscall, ret 0x10 (cpv6_noticia).
# - 0x539c60, 0x53a5b0, 0x53a1b0 (nomes do presidente novo): push nação; call; add esp, 0xc depois das três -> cdecl.
# Comparadores: as duas em ensaio (modelo CPV6_ENSAIO: a cópia sem a notícia nem a verba, e sem o presidente novo na
# 0x59e940; o registro de finanças, o sorteio, a marca +0x82 e o +0x20 da ficha restaurados): sem evento, tudo igual
# (inclusive o sorteio); com evento, a devolução, o saldo, +0xb4, +0x154, +0x165 e o +0x20 da ficha (o tipo da
# notícia, a marca +0x82 e os sorteios depois da verba dependem dela). A 0x5999e0 lado a lado nas finanças.
SAVE = False
ROTINAS = [('impulso_das_financas', 0x59e050, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('troca_do_presidente', 0x59e940, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('pagar_outros_custos', 0x5999e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x20, 0x01, 0x00, 0x00])]
DOCS = ('os eventos raros das finanças: o impulso das finanças, a troca do presidente e os outros custos',
        'the rare finance events: the finance boost, the board takeover and the other costs',
        [('impulso_das_financas', 'FinanceTable::try_finance_boost_event', '0x59e050', 'fin_prm.cpp', 'thiscall, ret 8',
          'fora de sete nações e das marcas +0x64 2 e 3, o divisor pela faixa de riqueza e pela reputação; com a ficha do presidente, três sorteios (contra o +0xf, a dívida e 750 × +0x20) dão (3 × (+0x20)² + 750) × reputação + 1 milhão mais o saldo; sem ela, dois sorteios dão (reputação + 1000) × 1000; um terço em três nações; o dinheiro, a notícia pelo tipo e o fim da crise',
          'outside seven nations and flags +0x64 2 and 3, the divisor by wealth band and reputation; with the chairman record, three draws (against +0xf, the debt and 750 × +0x20) give (3 × (+0x20)² + 750) × reputation + 1 million plus the balance; without it, two draws give (reputation + 1000) × 1000; a third in three nations; the money, the news by type and the end of the crisis'),
         ('troca_do_presidente', 'FinanceTable::try_board_takeover', '0x59e940', 'fin_prm.cpp', 'thiscall, ret 8',
          'o divisor pela faixa e o fator pela reputação e pela liga; sorteios pela ficha do presidente e por 15000 / divisor; (reputação + 1000) × 1000 menos o saldo (ou só o que zera o saldo); com a ficha, o presidente novo (nação, nomes e ficha sorteados); a notícia (tipo 2) e o fim da crise',
          'the divisor by band and the factor by reputation and league; draws by the chairman record and by 15000 / divisor; (reputation + 1000) × 1000 less the balance (or just what clears the balance); with the record, the new chairman (drawn nation, names and record); the news (type 2) and the end of the crisis'),
         ('pagar_outros_custos', 'Finance::pay_other_costs', '0x5999e0', 'fin_prm.cpp', 'thiscall, ret 4',
          'soma o valor aos outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo',
          'adds the amount to the season (+0x120) and month (+0x80) other costs and takes it from the balance')])
CAMPOS = [('finanças do clube +0 o saldo (i64), +0xb4 / +0x154 os investimentos do mês e do ano, +0x165 o estado de crise; valor; lê e grava (impulso_das_financas, troca_do_presidente)',
           'club finances +0 the balance (i64), +0xb4 / +0x154 month and season investment, +0x165 the crisis state; value; reads and writes (impulso_das_financas, troca_do_presidente)'),
          ('finanças +0x120 / +0x80: os outros custos do ano e do mês; valor; soma (pagar_outros_custos)',
           'finances +0x120 / +0x80: season and month other costs; value; adds (pagar_outros_custos)'),
          ('clube +0x53 a nação, +0x57 a liga (+0x69 palavra), +0x64 a marca, +0x80 a reputação, +0x82 a marca do impulso (tipo 7); ponteiro e valor; lê e grava +0x82 (impulso_das_financas, troca_do_presidente)',
           'club +0x53 the nation, +0x57 the league (+0x69 word), +0x64 the flag, +0x80 the reputation, +0x82 the boost flag (type 7); pointer and value; reads and writes +0x82 (impulso_das_financas, troca_do_presidente)'),
          ('clube +0xbf o presidente: +0x1a a nação, +4 / +8 / +0xc os nomes, +0x57 byte, +0x69 a ficha (+0xf, +0x16, +0x1d, +0x20); ponteiro e valor; lê e grava (impulso_das_financas: +0x20 cai 1; troca_do_presidente: tudo sorteado)',
           'club +0xbf the chairman: +0x1a the nation, +4 / +8 / +0xc the names, +0x57 byte, +0x69 the record (+0xf, +0x16, +0x1d, +0x20); pointer and value; reads and writes (impulso_das_financas: +0x20 drops 1; troca_do_presidente: all drawn)'),
          ('[0xae2350] / [0xae23a8] a conta e a tabela das nações (0x122 bytes); valor e ponteiro; lê (troca_do_presidente)',
           '[0xae2350] / [0xae23a8] the nation count and table (0x122 bytes); value and pointer; reads (troca_do_presidente)')]
INTERESSA = ['CMRTE']
