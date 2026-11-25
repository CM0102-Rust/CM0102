# Os premios do mes e a equipe da semana da Finlandia (award/finland_awards.cpp; leitor rx12, rodada 6, 25/11/2026).
# Usa award_month_manager, award_month_player e nation_award (sx03_2) e nation_awards_team_of_the_week_of (sx05b_5),
# ja publicados.
# on_month_change pela virtual +0xc (NationAwards::daily_update 0x768310 na virada do mes) e a equipe da semana pela
# virtual das equipes de premio, com a Finlandia carregada. Os premios do mes so guardam os meses de maio a outubro.
# Convencoes conferidas na desmontagem:
# - 0x5a9890 thiscall(this) ret 0: Staff_Records::find_manager_of_the_month 0x86f950 ([KEY_CLUB_COMP_FINNISH_PREMIER_
#   DIVISION]) e MONTH_AWARD::set_winner 0x7671c0 no premio 6 (o molde do award_month_manager), o mesmo com a First
#   Division no premio 11; MONTH_AWARD::award_best_rated_player 0x767150 ([+0xa], -1) ret 8 nos premios 5 e 10;
# - 0x5a9930 thiscall(premio) ret 4: compara com os premios 8 (+0x20) e 13 (+0x34); 0x769c70 ([[premio+9]], premio).
# Comparadores: so a original (o premio e a selecao nao se refazem).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('finland_awards_on_month_change', 0x5a9890, [0xa1, 0xe8, 0xf7, 0x9c, 0x00]),
           ('finland_awards_team_of_the_week', 0x5a9930, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('os prêmios do mês e a equipe da semana da Finlândia',
        'the Finnish month awards and team of the week',
        [('finland_awards_on_month_change', 'FinlandAwards::on_month_change', '0x5a9890', 'award/finland_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês da Premier Division e o da First Division (o melhor técnico do mês dos registros, com o clube dele), depois o jogador do mês de cada uma pelas notas do mês, sem limite de idade',
          "at the turn of the month, the Premier Division and First Division managers of the month (the staff records' best manager of the month, with his club), then each one's player of the month from the month ratings, with no age limit"),
         ('finland_awards_team_of_the_week', 'FinlandAwards::update_team_of_the_week', '0x5a9930', 'award/finland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é uma das duas equipes da semana finlandesas',
          'builds the team of the week when the award is one of the two Finnish teams of the week')])
CAMPOS = []
INTERESSA = []
