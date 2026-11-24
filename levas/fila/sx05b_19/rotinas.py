# O tecnico do mes e a equipe da semana de Gales (award/wales_awards.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa
# award_month_manager e nation_award (sx03_2) e nation_awards_team_of_the_week_of (sx05b_5): integrar depois deles.
# on_month_change pela virtual +0xc (NationAwards::daily_update na virada do mes) e a equipe da semana pela virtual
# das equipes de premio, com Gales carregado: LIGAS = 'ENG SCO WAL'.
# Convencoes conferidas na desmontagem:
# - 0x90f020 thiscall(this) ret 0: Staff_Records::find_manager_of_the_month 0x86f950 thiscall([0xdd7f00],
#   [KEY_CLUB_COMP_WELSH_PREMIER_DIVISION]) ret 4; sem tecnico, nada; MONTH_AWARD::set_winner 0x7671c0 thiscall([[+4]
#   +0x14], id, id do clube (lido sem conferir), mes (byte +0x14 do Staff_Records, com os bytes altos do ponteiro no
#   original), ano (word +0x16)) ret 0x10: o mesmo do award_month_manager do sx03_2;
# - 0x90f060 thiscall(premio) ret 4: compara com [[+4]] (premio 0); 0x769c70 thiscall([[premio+9]], premio) ret 8.
# Comparadores: so a original (o premio e a selecao nao se refazem).
SAVE = False
LIGAS = 'ENG SCO WAL'
ROTINAS = [('wales_awards_on_month_change', 0x90f020, [0xa1, 0x4c, 0xf9, 0x9c, 0x00]),
           ('wales_awards_team_of_the_week', 0x90f060, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('o técnico do mês e a equipe da semana de Gales',
        'the Welsh manager of the month and team of the week',
        [('wales_awards_on_month_change', 'WalesAwards::on_month_change', '0x90f020', 'award/wales_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês da Premier Division galesa: o melhor técnico do mês dos registros fica com o prêmio do mês, com o clube dele',
          "at the turn of the month, the Welsh Premier Division manager of the month: the staff records' best manager of the month takes the month award, with his club"),
         ('wales_awards_team_of_the_week', 'WalesAwards::update_team_of_the_week', '0x90f060', 'award/wales_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana de um prêmio galês: só para a equipe da semana (prêmio 0)',
          'builds the team of the week of a Welsh award: only for the team of the week (award 0)')])
CAMPOS = []
INTERESSA = []
