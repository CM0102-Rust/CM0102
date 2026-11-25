# Os premios do mes da Australia (award/australia_awards.cpp; leitor rx12, rodada 6, 25/11/2026). Usa
# award_month_manager, award_month_player e nation_award (sx03_2), ja publicados.
# Chamada pela virtual +0xc dos premios (NationAwards::daily_update na virada do mes) com a Australia carregada; os
# premios do mes guardam de outubro a maio.
# Convencoes conferidas na desmontagem: thiscall(this) ret 0; 0x86f950 ([KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_
# LEAGUE]) e 0x7671c0 no premio 1 (o molde do award_month_manager); 0x767150 ([[+4]+8], [+0xa], -1) e ([[+4]+0xc],
# [+0xa], 0x15) ret 8.
# Comparador: so a original (o premio nao se da duas vezes).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
ROTINAS = [('australia_awards_on_month_change', 0x412c20, [0xa1, 0x9c, 0xf8, 0x9c, 0x00])]
DOCS = ('os prêmios do mês da Austrália',
        'the Australian month awards',
        [('australia_awards_on_month_change', 'AustraliaAwards::on_month_change', '0x412c20', 'award/australia_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês da NSL (o melhor técnico do mês dos registros, com o clube dele), depois o jogador do mês, sem limite de idade, e o jovem do mês, até 21 anos, pelas notas do mês',
          "at the turn of the month, the NSL manager of the month (the staff records' best manager of the month, with his club), then the player of the month, with no age limit, and the young player of the month, up to 21, from the month ratings")])
CAMPOS = []
INTERESSA = []
