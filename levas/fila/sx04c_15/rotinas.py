# Os premios do mes da Escocia (award/scotland_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa nation_award e
# award_month_manager (sx03_2), award_month_best_player (sx04c_1) e SCOTLAND_DIVISIONS/scotland_block (sx04c_14):
# integrar depois deles.
# Roda com a Escocia carregada na virada de cada mes (virtual +0xc, de NationAwards::daily_update); os premios so
# aceitam os meses de agosto a maio.
# Convencao conferida: 0x7f6010 thiscall, ret 0: 0x86f950 + 0x7671c0 nos premios 4, 10, 16, 22; depois, por divisao,
# MONTH_RATINGS::find_best_player 0x768130 ([chave], -1 e 0x15) e MONTH_AWARD::set_winner (id, id do clube de
# [pessoa+0x39] ou 0, mes +0xc e ano +0xd das notas) nos premios 5/6, 11/12, 17/18, 23/24 (as notas relidas a cada vez).
# Comparador: so a original.
SAVE = False
LIGAS = 'ENG SCO WAL'
ROTINAS = [('scotland_awards_on_month_change', 0x7f6010, [0xa1, 0xdc, 0xf5, 0x9c, 0x00])]
DOCS = ('os prêmios do mês da Escócia',
        'the Scottish month awards',
        [('scotland_awards_on_month_change', 'ScotlandAwards::on_month_change', '0x7f6010', 'award/scotland_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês de cada divisão e, divisão por divisão, o jogador do mês e o jovem do mês (até 21), os melhores das notas do mês, com o clube',
          'at the turn of the month, the manager of the month of each division and, division by division, the player of the month and the young player of the month (up to 21), the best of the month ratings, with their club')])
CAMPOS = []
INTERESSA = []
