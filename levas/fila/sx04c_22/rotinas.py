# Os tecnicos do mes dos premios antigos da Irlanda (award/old_ireland_awards.cpp; leitor sx04c, rodada 5,
# 23/11/2026). Usa nation_award e award_month_manager (sx03_2): integrar depois dele.
# NAO RODA com a base do jogo (a classe antiga so com uma base sem as chaves novas; ver o sx04c_21); sem mutacoes.
# Convencao conferida: 0x79f490 thiscall, ret 0: 0x86f950 + 0x7671c0 nos premios 9 e 16.
# Comparador: so a original.
SAVE = False
ROTINAS = [('old_ireland_awards_on_month_change', 0x79f490, [0xa1, 0x0c, 0xf8, 0x9c, 0x00])]
DOCS = ('os técnicos do mês dos prêmios antigos da Irlanda',
        'the old Irish managers of the month',
        [('old_ireland_awards_on_month_change', 'OldIrelandAwards::on_month_change', '0x79f490', 'award/old_ireland_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês de cada divisão irlandesa, o melhor técnico do mês da divisão, com o clube',
          'at the turn of the month, the manager of the month of each Irish division, the best manager of the month of the division, with his club')])
CAMPOS = []
INTERESSA = []
