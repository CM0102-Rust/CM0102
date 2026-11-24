# Os premios da Inglaterra: os do ano, os do mes e a equipe da semana (england_awards.cpp; leitor sx03, rodada 5,
# 23/11/2026). Rodam na carreira ENG: daily_update todo dia (virtual +4, de award_manager_process_date),
# on_month_change na virada do mes (virtual +0xc, de NationAwards::daily_update 0x768310), update_team_of_the_week
# (virtual +0x10, da equipe da semana). Vtable 0x969f38: init_awards, daily_update, 0x5a96b0 (notas da partida),
# on_month_change, update_team_of_the_week.
# Datas (meses de 0): 5 de abril (jogador e jovem do ano da Premier, equipes da temporada), 30 de maio (tecnicos do
# ano), o dia de fim de temporada da nacao (premios do mes viram a temporada).
# Convencoes conferidas: 0x768cf0 thiscall 10 args ret 0x28; 0x7698f0 thiscall(comp, premio) ret 8; 0x768940
# thiscall(hoje, premio, comp) ret 0xc; 0x7672e0 thiscall(ano) ret 4; 0x767150 thiscall(notas, idade) ret 8 (Rust);
# 0x7671c0 thiscall(id, clube, mes, ano) ret 0x10 (Rust); 0x86f950 thiscall(comp) ret 4; 0x769c70 thiscall(comp,
# premio) ret 8; 0x768310 thiscall(hoje) ret 4. Os bytes altos do mes e do ano de set_winner sao lixo de registrador
# no original; Staff_Records::add_month_award_record so usa o byte e a palavra baixos.
# Depende de nada de outro item; os itens sx03_4 em diante usam os auxiliares daqui (nation_award, awards_day_is...).
# Comparadores: so a original (noticias, historico e reputacao: nao rodam duas vezes).
SAVE = False
ROTINAS = [
    ('england_awards_daily_update', 0x579610, [0x83, 0xec, 0x10, 0x56, 0x57]),
    ('england_awards_on_month_change', 0x579900, [0xa1, 0xbc, 0xf5, 0x9c, 0x00]),
    ('england_awards_update_team_of_the_week', 0x579af0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
]
DOCS = ('os prêmios da Inglaterra: os do ano, os do mês e a equipe da semana',
        "England's awards: the yearly ones, the monthly ones and the team of the week",
        [('england_awards_daily_update', 'england_awards_daily_update', '0x579610', 'england_awards.cpp', 'thiscall, ret 4',
          'regra: em 5 de abril, o jogador do ano e o jovem do ano (até 21) pelas estatísticas da Premier e a equipe da temporada de cada divisão (e da Conference, com as divisões inferiores); em 30 de maio, o técnico do ano de cada divisão (e da Conference); no dia do fim de temporada da nação, os prêmios do mês passam para a temporada nova com o ano de hoje; sempre, a conferência da virada do mês da base',
          "rule: on 5 April, the player of the year and the young player of the year (up to 21) from the Premier's stats and the team of the season of each division (and the Conference's, with the lower divisions); on 30 May, the manager of the year of each division (and the Conference's); on the nation's season end day, the month awards move to the new season with today's year; always, the base month change check"),
         ('england_awards_on_month_change', 'england_awards_on_month_change', '0x579900', 'england_awards.cpp', 'thiscall, ret 0',
          'regra: na virada do mês, o técnico do mês de cada divisão (e da Conference), o de mais pontos no mês com pelo menos duas semanas no cargo, com o clube dele e o mês e o ano do registro de pessoas; depois o jogador do mês e o jovem do mês (até 20) de cada divisão pelas notas do mês',
          "rule: at the turn of the month, the manager of the month of each division (and the Conference's), the one with the most points in the month among those two weeks or more in the job, with his club and the staff records' month and year; then the player of the month and the young player of the month (up to 20) of each division from the month ratings"),
         ('england_awards_update_team_of_the_week', 'england_awards_update_team_of_the_week', '0x579af0', 'england_awards.cpp', 'thiscall, ret 4',
          'regra: monta a equipe da semana quando o prêmio é o de equipe de uma das quatro divisões (ou o da Conference, com as divisões inferiores), com a competição do prêmio',
          "rule: builds the team of the week when the award is the team award of one of the four divisions (or the Conference's, with the lower divisions), with the award's competition")])
CAMPOS = []
INTERESSA = []
