# O jogador do mes da Coreia do Sul (award/korea_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa nation_award e
# award_month_player (sx03_2): integrar depois dele.
# Roda com a Coreia carregada na virada de cada mes (virtual +0xc, de NationAwards::daily_update 0x768310: a Coreia
# tem notas do mes); o premio so aceita os meses de marco a novembro (o MONTH_AWARD decide).
# Convencao conferida: 0x66e610 thiscall, ret 0: MONTH_AWARD::award_best_rated_player 0x767150 (notas +0xa, -1) no
# premio +4 da tabela (o premio 1).
# Comparador: so a original (o premio nao se da duas vezes).
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ROTINAS = [('korea_awards_on_month_change', 0x66e610, [0x8b, 0x41, 0x0a, 0x8b, 0x49, 0x04])]
DOCS = ('o jogador do mês da Coreia do Sul',
        'the South Korean player of the month',
        [('korea_awards_on_month_change', 'KoreaAwards::on_month_change', '0x66e610', 'award/korea_awards.cpp', 'thiscall',
          'na virada do mês, o jogador do mês coreano pelas notas do mês, sem limite de idade',
          'at the turn of the month, the Korean player of the month from the month ratings, with no age limit')])
CAMPOS = []
INTERESSA = []
