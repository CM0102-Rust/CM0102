# O jogador do mes da MLS (award/usa_awards.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa award_month_player e
# nation_award (sx03_2): integrar depois dele.
# Chamada pela virtual +0xc dos premios (NationAwards::daily_update 0x768310 na virada do mes) com os EUA carregados:
# LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'. O premio so guarda os meses de marco a outubro.
# Convencoes conferidas na desmontagem: thiscall(this) ret 0; MONTH_AWARD::award_best_rated_player 0x767150
# thiscall([[+4]], [+0xa], -1) ret 8.
# Comparador: so a original (o premio nao se da duas vezes).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('usa_awards_on_month_change', 0x906f60, [0x8b, 0x41, 0x0a, 0x8b, 0x49, 0x04])]
DOCS = ('o jogador do mês da MLS',
        'the MLS player of the month',
        [('usa_awards_on_month_change', 'UsaAwards::on_month_change', '0x906f60', 'award/usa_awards.cpp', 'thiscall',
          'na virada do mês, o jogador do mês da MLS pelas notas do mês dos EUA, sem limite de idade',
          "at the turn of the month, the MLS player of the month from the USA's month ratings, with no age limit")])
CAMPOS = []
INTERESSA = []
