# Os calendarios da Primeira Divisao e da Premier russas (rus_first.cpp, rus_prm.cpp; leitor sbE, rodada 5, 24/11/2026).
# Simulacao com a Russia (as ligas e a data dos itens mwd): os calendarios na criacao do jogo e na virada de novembro de
# 2001 (a temporada russa vai de marco a novembro); o play-off do titulo so com os dois primeiros empatados.
# - 0x7ea220 RusFirst::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): so -1, 36
#   rodadas de 1/abr a 1/nov (extra 0, caixa 0xaa): as 36 datas 0x68a160 e depois, num laco, a faixa vazia 0x68a1c0 de
#   cada rodada. Outra fase 0 sem escrever. Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes).
# - 0x7ead50 RusPrm::create_schedule: -1, 30 rodadas (extra 0, caixa 0xaa): com a Russia na Copa do Mundo (0x9cf79c) ou
#   na Eurocopa (0x9cf7a4) (Comp::includes_club 0x48db80 da selecao de database_get_national_team 0x53b3d0) as datas das
#   finais, senao as comuns; depois o laco das faixas vazias das 30 (a caixa 0xfc nunca e alcancada); 0, o play-off do
#   titulo (e_liga 0, 1 rodada de copa, caixa 0x10f): sorteio 13/nov, jogo 18/nov, rodada 0xa0, desempate 0x203,
#   categoria 5, 2p times, p confrontos e 2p novos (p o byte +0xee, os pares), jogo unico; outra fase 0. Prologo mov al,
#   [esp+4]; sub esp, 0x200 (10 bytes).
#   Defeito do original: as datas das finais sao 31 (a ultima em 12/nov) numa tabela de 30: a 31a grava alem do bloco
#   do malloc (estouro de heap). A copia faz o mesmo; o comparador so roda a original nesse caso.
# Tabelas tiradas executando a original no unicorn (emula.py do cx14, com a Russia nas finais simulada).
# Depende do cx08_2 (compare_staged_schedule, ScheduleTable): integrar depois dele.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: lado a lado (compare_staged_schedule; o play-off como copa).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rus_first_create_schedule', 0x7ea220, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rus_prm_create_schedule', 0x7ead50, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Primeira Divisão e da Premier russas', 'the Russian First Division and Premier League schedules',
        [('rus_first_create_schedule', 'RusFirst::create_schedule', '0x7ea220', 'rus_first.cpp', 'thiscall, ret 0x10',
          'a Primeira Divisão russa em 36 rodadas de 1º de abril a 1º de novembro, a cada seis dias mais ou menos, com muitas rodadas no meio da semana; sem faixas de TV',
          'the Russian First Division in 36 rounds from 1 April to 1 November, about every six days, with many midweek rounds; no TV slots'),
         ('rus_prm_create_schedule', 'RusPrm::create_schedule', '0x7ead50', 'rus_prm.cpp', 'thiscall, ret 0x10',
          'a Premier russa em 30 rodadas de 25 de março a 12 de novembro; com a Rússia na Copa do Mundo ou na Eurocopa, outras datas com rodadas no meio da semana e uma pausa de 31 de maio a 22 de julho; o play-off do título (dois empatados no topo) em jogo único em 18 de novembro',
          'the Russian Premier League in 30 rounds from 25 March to 12 November; with Russia at the World Cup or the European Championship, other dates with midweek rounds and a break from 31 May to 22 July; the title play-off (two clubs tied at the top) a single match on 18 November')])
CAMPOS = []
INTERESSA = []
