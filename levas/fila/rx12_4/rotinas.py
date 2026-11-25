# Os jogadores do mes da Alemanha (award/germany_awards.cpp; leitor rx12, rodada 6, 25/11/2026). Usa
# award_month_player e nation_award (sx03_2), ja publicados.
# Chamada pela virtual +0xc dos premios (NationAwards::daily_update 0x768310 na virada do mes) com a Alemanha
# carregada; os premios do mes guardam de agosto a maio.
# Convencoes conferidas na desmontagem: thiscall(this) ret 0; MONTH_AWARD::award_best_rated_player 0x767150
# thiscall([[+4]+0x1c], [+0xa], -1) e ([[+4]+0x2c], [+0xa], 0x14) ret 8.
# Comparador: so a original (o premio nao se da duas vezes).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('germany_awards_on_month_change', 0x5dff40, [0x56, 0x8b, 0xf1, 0x6a, 0xff])]
DOCS = ('os jogadores do mês da Alemanha',
        'the German players of the month',
        [('germany_awards_on_month_change', 'GermanyAwards::on_month_change', '0x5dff40', 'award/germany_awards.cpp', 'thiscall',
          'na virada do mês, o jogador do mês da primeira divisão pelas notas do mês, sem limite de idade, e o da segunda divisão até 20 anos',
          'at the turn of the month, the First Division player of the month from the month ratings, with no age limit, and the Second Division one up to 20')])
CAMPOS = []
INTERESSA = []
