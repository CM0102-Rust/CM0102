# Os premios do mes da Noruega (award/norway_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa nation_award,
# award_month_manager e award_month_player (sx03_2): integrar depois dele.
# Roda com a Noruega carregada na virada de cada mes (virtual +0xc, de NationAwards::daily_update); os premios so
# aceitam os meses de abril a outubro.
# Convencao conferida: 0x794430 thiscall, ret 0: 0x86f950 + 0x7671c0 nos premios 6 (Premier) e 9 (First), depois
# 0x767150 (notas +0xa, -1) nos premios 7 e 10.
# Comparador: so a original.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [('norway_awards_on_month_change', 0x794430, [0xa1, 0xbc, 0xf6, 0x9c, 0x00])]
DOCS = ('os prêmios do mês da Noruega',
        'the Norwegian month awards',
        [('norway_awards_on_month_change', 'NorwayAwards::on_month_change', '0x794430', 'award/norway_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês de cada divisão (o melhor técnico do mês, com o clube) e o jogador do mês de cada uma pelas notas do mês',
          'at the turn of the month, the manager of the month of each division (the best manager of the month, with his club) and the player of the month of each from the month ratings')])
CAMPOS = []
INTERESSA = []
