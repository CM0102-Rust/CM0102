ROTINAS = [('ordem_das_linhas_da_tabela', 0x683210, [0x51, 0x53, 0x8b, 0x1d, 0xfc, 0x79, 0xb6, 0x00]),
           ('montar_grade_de_turnos', 0x6844b0, [0x8b, 0x4c, 0x24, 0x08, 0x83, 0xec, 0x10]),
           ('margem_do_ultimo_dia', 0x68a4d0, [0x0f, 0xbf, 0x81, 0xa9, 0x00, 0x00, 0x00])]
DOCS = ('a ordem das linhas da tabela, a grade de turnos da liga e a margem de gols do último dia',
        'the league table row order, the league round-robin grid and the last-day goal margin',
        [('ordem_das_linhas_da_tabela', 'league_compare_table_rows', '0x683210', 'league.cpp', 'cdecl',
          'a comparação do qsort da tabela: pontos, os critérios de desempate da liga e o id do clube',
          'the table qsort comparison: points, the league tiebreak rules and the club id'),
         ('montar_grade_de_turnos', 'league_build_round_robin_grid', '0x6844b0', 'league.cpp', 'stdcall',
          'monta a grade em que cada clube enfrenta todos uma vez, com o mando equilibrado',
          'builds the grid where every club meets each other once, with balanced home and away games'),
         ('margem_do_ultimo_dia', 'League::final_day_margin_needed', '0x68a4d0', 'league.cpp', 'thiscall',
          'no último dia, a margem de gols que o clube precisa para o título, as vagas de cima ou fugir da queda, e a marca de jogo decisivo',
          'on the last matchday, the goal margin a club needs for the title, the top places or survival, and the deciding-match flag')])
