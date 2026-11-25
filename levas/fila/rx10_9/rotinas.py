# As posicoes previstas e a virada da Segunda Divisao B portuguesa (por_second_b.cpp; leitor rx10, rodada 6,
# 25/11/2026). Simulacao com Portugal e a base NORMAL (ver rx10_7). As posicoes rodam na criacao do jogo e na virada;
# a virada no verao de 2002 (pela cadeia da Liga de Honra).
# - 0x7d2cd0 PorSecondB::set_predicted_positions thiscall() ret 0: section_predicted_positions (rx10_4) com duas zonas
#   e passo 3 (a posicao 3 x linha + 1, o byte de imul); caixas 0x2b7 e 0x2bf. Prologo sub esp, 0x204 (6 bytes).
# - 0x7d26b0 PorSecondB::start_new_season thiscall() ret 0: divided_league_turnover com SEGUNDA_B (rx10_7) e as caixas
#   0x1fe, 0x205, 0x20c, 0x212; as zonas 0x7d2870(0) e (1). Prologo sub esp, 0x200 (6 bytes).
# Depende do rx10_4 e do rx10_7 (SEGUNDA_B). Iguais no cm0102-1080.exe. Comparadores: as posicoes lado a lado; a virada
# so a original (o ano sobe um).
# Defeito do original: as tres zonas recebem as mesmas posicoes (1, 4, 7...), nao intercaladas.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('por_second_b_set_predicted_positions', 0x7d2cd0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('por_second_b_start_new_season', 0x7d26b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas e a virada da Segunda Divisão B portuguesa',
        'the Portuguese Segunda B predicted positions and season turnover',
        [('por_second_b_set_predicted_positions', 'PorSecondB::set_predicted_positions', '0x7d2cd0', 'por_second_b.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Segunda B: os clubes de cada zona pela reputação, o melhor primeiro, nas posições 1, 4, 7... (as três zonas recebem as mesmas posições)',
          'rule: the Segunda B predicted positions: each zone\'s clubs by reputation, the best first, in places 1, 4, 7... (the three zones get the same places)'),
         ('por_second_b_start_new_season', 'PorSecondB::start_new_season', '0x7d26b0', 'por_second_b.cpp', 'thiscall, ret 0',
          'regra: na virada a Segunda B arquiva a temporada, limpa a tabela Norte, as datas e as zonas, passa o ano e refaz as regras, a tabela Norte, a ordem, os jogos e as duas zonas; depois as posições previstas e a renovação dos árbitros',
          'rule: at the season turn the Segunda B archives the season, clears the North table, the dates and the zones, moves the year on and rebuilds the rules, the North table, the order, the fixtures and the two zones; then the predicted positions and the referee renewal')])
CAMPOS = []
INTERESSA = []
