ROTINAS = [('tirar_jogador_da_selecao', 0x775ba0, [0x83, 0xec, 0x08, 0x53, 0x56, 0x57]),
           ('faixa_do_horario', 0x5af2f0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('teto_da_observacao', 0x84b230, [0x0f, 0xbe, 0x44, 0x24, 0x0c])]
DOCS = ('o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados',
        'the player leaving the national side, the fixture time slice and the shortlist fee ceiling',
        [('tirar_jogador_da_selecao', 'NationalTeams::remove_player', '0x775ba0', 'national_teams.cpp', 'thiscall',
          'tira o jogador da reserva do nível dele na nação e da convocação, limpando os campos de seleção',
          "takes the player out of his level pool in the nation and out of the squad, clearing his national team fields"),
         ('faixa_do_horario', 'FIXTURE_MANAGER::get_time_slice', '0x5af2f0', 'fix_man.cpp', 'thiscall',
          'a faixa de horário do jogo (1 fim de semana, 2 os outros) pela nação da competição e o dia',
          'the fixture time slice (1 weekend, 2 otherwise) by competition nation and weekday'),
         ('teto_da_observacao', 'shortlist_manager_max_fee_for_squad_status', '0x84b230', 'shortlist_manager.cpp', 'stdcall',
          'o teto de preço da vaga na lista de observados: reputação do clube vezes um fator (ou -1)',
          'the shortlist slot fee ceiling: club reputation times a factor (or -1)')])
