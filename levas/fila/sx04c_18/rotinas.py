# O jogador do mes dos premios antigos da Finlandia (award/old_finland_awards.cpp; leitor sx04c, rodada 5, 23/11/2026).
# Usa nation_award e award_month_player (sx03_2): integrar depois dele.
# NAO RODA com a base do jogo (a classe antiga so com uma base sem as chaves novas; ver o sx04c_17); sem mutacoes.
# Convencao conferida: 0x79dcb0 thiscall, ret 0: 0x767150 (notas +0xa, -1) no premio +0x14 da tabela (o 5).
# Comparador: so a original.
SAVE = False
ROTINAS = [('old_finland_awards_on_month_change', 0x79dcb0, [0x8b, 0x41, 0x0a, 0x8b, 0x49, 0x04])]
DOCS = ('o jogador do mês dos prêmios antigos da Finlândia',
        'the old Finnish player of the month',
        [('old_finland_awards_on_month_change', 'OldFinlandAwards::on_month_change', '0x79dcb0', 'award/old_finland_awards.cpp', 'thiscall',
          'na virada do mês, o jogador do mês da Premier Division finlandesa pelas notas do mês, sem limite de idade',
          'at the turn of the month, the Finnish Premier Division player of the month from the month ratings, with no age limit')])
CAMPOS = []
INTERESSA = []
