ROTINAS = [('disciplina_temporada_nova', 0x556210, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10]),
           ('disciplina_do_dia', 0x556240, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('disciplina_gravar_vaga', 0x55cb20, [0x0f, 0xbe, 0x44, 0x24, 0x08])]
DOCS = ('a disciplina: a temporada nova, o dia e a vaga da tabela',
        'discipline: the new season, the day and the table slot',
        [('disciplina_temporada_nova', 'discipline_new_season', '0x556210', 'discipline.cpp', 'thiscall',
          'começa a temporada nova das regras de disciplina da competição (método 0 da tabela virtual)',
          'starts the new season of the competition\'s discipline rules (virtual method 0)'),
         ('disciplina_do_dia', 'discipline_daily_update', '0x556240', 'discipline.cpp', 'thiscall',
          'o dia de cada uma das 43 regras de disciplina (método 1 da tabela virtual)',
          'the day of each of the 43 discipline rules (virtual method 1)'),
         ('disciplina_gravar_vaga', 'discipline_set_slot', '0x55cb20', 'discipline.cpp', 'thiscall',
          'grava o tipo e o valor de uma vaga da pessoa na tabela da disciplina',
          'writes the type and value of one of the person\'s slots in the discipline table')])
