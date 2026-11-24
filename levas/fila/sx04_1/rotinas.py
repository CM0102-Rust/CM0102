# A virada do mes dos premios de uma nacao, a nova temporada do premio do mes e a ordem das notas do mes
# (nation_awards.cpp, month_award.cpp, month_ratings.cpp; leitor sx04, rodada 5, 23/11/2026). Rodam na carreira ENG:
# NationAwards::daily_update 0x768310 todo dia, no fim da diaria de cada nacao (a da Inglaterra 0x5798e5); a nova
# temporada 0x7672e0 no dia de inicio da temporada da Inglaterra (0x5797f1.., uma vez por premio do mes); a ordem
# 0x768200 pelo qsort da MONTH_RATINGS::find_best_player (0x768130, ja Rust), a cada virada do mes.
# Convencoes conferidas na desmontagem:
# - 0x768310: thiscall(hoje), ret 4 (ponto de chamada `push edi; mov ecx, esi; call` 0x5798e5); quadro SEH sem
#   objeto com destrutor; Date::add_days 0x54c5b0 thiscall(hoje, &local, 1); Date::get_month 0x54cfb0 thiscall -> al;
#   a virtual +0xc sem argumentos; MONTH_RATINGS::reset 0x767f60 thiscall(mes, dword do ano), ret 8, com o ponteiro
#   das notas lido de novo depois da virtual.
# - 0x7672e0: thiscall(ano em word), ret 4; a conta (+0xa1, i8) relida a cada volta.
# - 0x768200: cdecl(a, b) do qsort; escreve nas vagas dos proprios argumentos (rascunho do fild), sem efeito.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('nation_awards_daily_update', 0x768310, [0x6a, 0xff, 0x68, 0xc8, 0xdb, 0x95, 0x00]),
           ('month_award_new_season', 0x7672e0, [0x53, 0x8a, 0x99, 0xa1, 0x00, 0x00, 0x00]),
           ('month_ratings_compare_average', 0x768200, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a virada do mês dos prêmios de uma nação, a nova temporada do prêmio do mês e a ordem das notas do mês',
        "a nation's awards month change, the month award's new season and the month ratings order",
        [('nation_awards_daily_update', 'NationAwards::daily_update', '0x768310', 'nation_awards.cpp', 'thiscall, ret 4',
          'regra: no último dia do mês (amanhã cai noutro mês ou ano que o das notas do mês da nação), dá os prêmios do mês da nação (a virtual on_month_change: jogador, jovem e técnico do mês) e depois esvazia as notas do mês para o mês e o ano de amanhã. Nação sem notas do mês: nada',
          "rule: on the month's last day (tomorrow falls in another month or year than the nation's month ratings), gives the nation's month awards (the virtual on_month_change: player, young player and manager of the month) and then empties the month ratings for tomorrow's month and year. A nation without month ratings: nothing"),
         ('month_award_new_season', 'MONTH_AWARD::start_new_season', '0x7672e0', 'month_award.cpp', 'thiscall, ret 4',
          'regra: no dia de início da temporada, se a primeira vaga do mês ainda não é do ano dado, cada vaga em uso passa um ano adiante e perde o vencedor e o clube',
          'rule: on the season start day, unless the first month slot is already in the given year, every used slot moves one year on and loses its winner and club'),
         ('month_ratings_compare_average', 'month_ratings_compare_average', '0x768200', 'month_ratings.cpp', 'cdecl',
          'regra: a ordem das notas do mês: a maior média primeiro; quem tem menos de 3 jogos fica no fim',
          'rule: the month ratings order: the best average first; those with fewer than 3 matches go last')])
CAMPOS = [('prêmios da nação (NationAwards) +4 tabela dos prêmios, +8 conta (i16), +0xa notas do mês; ponteiros; lê (nation_awards_daily_update)',
           'nation awards (NationAwards) +4 award table, +8 count (i16), +0xa month ratings; pointers; reads (nation_awards_daily_update)'),
          ('prêmio do mês +0xa1 vagas em uso (i8); valor; lê (month_award_new_season)',
           'month award +0xa1 slots in use (i8); value; reads (month_award_new_season)')]
INTERESSA = []
