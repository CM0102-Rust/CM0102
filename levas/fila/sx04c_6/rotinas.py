# Os tecnicos do mes da Irlanda do Norte (award/northern_ireland_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa
# nation_award e award_month_manager (sx03_2): integrar depois dele.
# Roda com a Irlanda do Norte carregada na virada de cada mes (virtual +0xc, de NationAwards::daily_update).
# Convencao conferida: 0x793630 thiscall, ret 0: Staff_Records::find_manager_of_the_month 0x86f950 thiscall(comp) ret
# 4 e MONTH_AWARD::set_winner 0x7671c0 (id, clube, mes byte +0x14, ano word +0x16 do Staff_Records) nos premios 6 e 12.
# Comparador: so a original.
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [('northern_ireland_awards_on_month_change', 0x793630, [0xa1, 0xb0, 0xf8, 0x9c, 0x00])]
DOCS = ('os técnicos do mês da Irlanda do Norte',
        'the Northern Irish managers of the month',
        [('northern_ireland_awards_on_month_change', 'NorthernIrelandAwards::on_month_change', '0x793630', 'award/northern_ireland_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês da Premier Division e o da First Division, cada um o melhor técnico do mês da sua divisão, com o clube dele',
          'at the turn of the month, the manager of the month of the Premier Division and of the First Division, each the best manager of the month of his division, with his club')])
CAMPOS = []
INTERESSA = []
