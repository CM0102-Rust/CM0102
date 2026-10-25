# A comissão do clube junto com a do time parceiro A/B (0x540a50): o auxiliar (0x53cb80), os 5 treinadores
# (0x53cc30, irmã da 0x53cd60 dos olheiros do dbk_2) e a contagem dos treinadores válidos (0x53c8d0). Rodam na
# carreira: a distribuição dos treinadores (TRAINING_MANAGER::update_coach_assignments 0x8b2880, a cada contratação
# de comissão e transferência, e pelo MANAGER_MANAGER::daily_update 0x68f9c0 → 0x8b2470 quando um técnico assume) e a
# regeneração (0x7a65d0). Na criação do .qst só passam se algum técnico assumir ou alguma pessoa da comissão for
# contratada nos dias até a data de início (game_run_to_start_date chama o 0x68f9c0): as mutações PODEM ESCAPAR da
# prova do save; os comparadores rodam no jogo real comparando da cadeia. Sem SEH, sem sorteio; a local de 0x200
# bytes só serve à mensagem da caixa (clube nulo, que fica com a original).
SAVE = True
ROTINAS = [('auxiliar_do_clube', 0x53cb80, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('treinadores_do_clube', 0x53cc30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('contar_treinadores_do_clube', 0x53c8d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a comissão do clube com a do time parceiro: o auxiliar, os treinadores e a contagem dos treinadores',
        'the club staff with the partner team ones: the assistant, the coaches and the coach count',
        [('auxiliar_do_clube', 'database_get_club_assistant', '0x53cb80', 'Database.cpp', 'cdecl',
          'o auxiliar (+0xd3) do clube; sem ele, o do time parceiro A/B; nenhum: 0',
          'the club assistant (+0xd3); without one, the partner (senior/B) team one; none: 0'),
         ('treinadores_do_clube', 'database_get_club_coaches', '0x53cc30', 'Database.cpp', 'cdecl',
          'copia os 5 treinadores do clube para um vetor; com time parceiro (A/B), junta os dois até 5 e zera o resto',
          'copies the 5 club coaches into an array; with a partner team (senior/B), merges both up to 5 and clears the rest'),
         ('contar_treinadores_do_clube', 'database_count_club_coaches', '0x53c8d0', 'Database.cpp', 'cdecl',
          'quantos treinadores válidos (não nulos, id < [0xae2364]) o clube e o time parceiro têm juntos',
          'how many valid coaches (non-null, id < [0xae2364]) the club and the partner team have together')])
CAMPOS = [('TClub +0xd3: auxiliar; ponteiro; lê (auxiliar_do_clube)', 'TClub +0xd3: assistant; pointer; reads (auxiliar_do_clube)'),
          ('TClub +0x19f (5): treinadores; ponteiros; lê (treinadores_do_clube, contar_treinadores_do_clube)', 'TClub +0x19f (5): coaches; pointers; reads (treinadores_do_clube, contar_treinadores_do_clube)'),
          ('TClub +0x244: tem time parceiro; byte; zerado pela 0x540a50 quando recusa o parceiro (as três)', 'TClub +0x244: has partner team; byte; cleared by 0x540a50 when it refuses the partner (all three)')]
INTERESSA = ['SGE', 'CMRTE']
