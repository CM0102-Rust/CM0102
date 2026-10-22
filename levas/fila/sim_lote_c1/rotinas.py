ROTINAS = [('liga_tem_rebaixamento', 0x6847b0, [0x8a, 0x91, 0xc1, 0x00, 0x00, 0x00]),
           ('data_da_rodada_da_liga', 0x68a160, [0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18]),
           ('importancia_do_jogo_da_liga', 0x68a3e0, [0x51, 0x8b, 0x44, 0x24, 0x10])]
DOCS = ('se a liga tem rebaixamento, a data da rodada do modelo da liga e a importância do jogo da liga',
        'whether the league has relegation, the league schedule round date and the league match importance',
        [('liga_tem_rebaixamento', 'League::has_relegation', '0x6847b0', 'league.cpp', 'thiscall',
          'se a liga tem vagas de rebaixamento no fim da tabela',
          'whether the league has relegation places at the bottom of the table'),
         ('data_da_rodada_da_liga', 'league_round_set_date', '0x68a160', 'league.cpp', 'cdecl',
          'grava num registro de rodada do modelo da liga a data com o ano relativo, o código e um valor',
          'stores in a league schedule round record the date with a relative year, the code and a value'),
         ('importancia_do_jogo_da_liga', 'League::get_match_importance', '0x68a3e0', 'league.cpp', 'thiscall',
          'a importância do jogo da liga para o clube: a base da liga, +1 no começo do campeonato ou com a margem do último dia',
          'the league match importance for the club: the league base, +1 early in the season or with the final-day margin')])
