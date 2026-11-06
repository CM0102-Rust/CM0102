# Os clubes das supercopas: a Supercopa Europeia, a Supercopa Asiática e a Supercopa da Série C1 (eur_super_cup.cpp,
# asia_super_cup.cpp em eurocomp\, ita_c1_super.cpp em cups\). Frente das competições (Agente D, 06/11/2026), carreira
# de 8 ligas (começa em janeiro de 2001).
# - 0x57a500 (EurSuperCup::select_teams, 456 bytes; 1 chamada até 2002-06-01 com as 8 ligas): o campeão da Liga dos
#   Campeões e o da Copa da UEFA. Duas mutações.
# - 0x410a50 (AsiaSuperCup::select_teams, 497 bytes; 1 chamada): os campeões asiáticos. Uma mutação.
# - 0x63c9d0 (ItaC1Super::init_team_list, 203 bytes; 1 chamada): as duas vagas vazias (passageira).
# Reaproveita do motor.rs os ajudantes das supercopas da 276a (cqm12_lista, cqm12_colocado, cqm12_vaga, cqm12_fim,
# cqm12_comparar).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, copa; call; sem push), do construtor e da
# virada. Dentro: 0x944e46 (push 0xc; add esp, 4 -> cdecl), 0x4afd80 (push lugar; push competição do banco; add esp, 8
# -> cdecl, pela cqm12_colocado) e a caixa (aviso_em).
# Comparadores: lado a lado (cqm12_comparar): o +0x3e e o [0xb67a34]; a lista da cópia comparada nos 12 bytes e
# liberada.
# Determinismo: nada lê pilha, heap ou endereço como valor (a 0x4afd80 ordena o histórico da competição; a segunda
# passada acha o histórico já ordenado e escolhe o mesmo clube).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq11_europeia_clubes', 0x57a500, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqq11_asiatica_clubes', 0x410a50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqq11_c1_vagas', 0x63c9d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os clubes das supercopas: a Supercopa Europeia, a Supercopa Asiática e a Supercopa da Série C1',
        'the super cups\' clubs: the European Super Cup, the Asian Super Cup and the Serie C1 Super Cup',
        [('cqq11_europeia_clubes', 'EurSuperCup::select_teams', '0x57a500', 'eur_super_cup.cpp', 'thiscall, ret 0',
          'regra: a Supercopa Europeia é entre o campeão da Liga dos Campeões e o da Recopa na edição de 1999 (a última Recopa) e, nas outras, o da Copa da UEFA; sem um dos dois, a caixa de erro',
          'rule: the European Super Cup is between the Champions League winner and the Cup Winners\' Cup winner in the 1999 edition (the last Cup Winners\' Cup) and, in the others, the UEFA Cup winner; without either, the error box'),
         ('cqq11_asiatica_clubes', 'AsiaSuperCup::select_teams', '0x410a50', 'asia_super_cup.cpp', 'thiscall, ret 0',
          'regra: a Supercopa Asiática é entre o campeão da Liga dos Campeões asiática e o da Recopa asiática; se o mesmo clube ganhou as duas, entra o vice da Liga dos Campeões',
          'rule: the Asian Super Cup is between the Asian Club Championship winner and the Asian Cup Winners\' Cup winner; if the same club won both, the Club Championship runner-up plays'),
         ('cqq11_c1_vagas', 'ItaC1Super::init_team_list', '0x63c9d0', 'ita_c1_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Série C1 começa com as duas vagas vazias; os clubes entram no fim da temporada [inferência: os campeões das duas chaves da C1]',
          'rule: the Serie C1 Super Cup starts with both places empty; the clubs enter at the end of the season [inference: the champions of the two C1 groups]')])
CAMPOS = [('copa +0x3e inscritos, +0xa7 lista (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (as três)',
           'cup +0x3e entrants, +0xa7 list (6 bytes: club, +4 class, +5 status); value/pointer; writes (all three)'),
          ('copa +0x40 ano; valor; lê (Supercopa Europeia)', 'cup +0x40 year; value; reads (European Super Cup)'),
          ('[0x9cf6e8] Liga dos Campeões, [0x9cf6ec] Recopa, [0x9cf6f0] Copa da UEFA, [0x9cf64c]/[0x9cf650] as copas asiáticas; [0xae23d0] competições do banco; id; lê',
           '[0x9cf6e8] Champions League, [0x9cf6ec] Cup Winners\' Cup, [0x9cf6f0] UEFA Cup, [0x9cf64c]/[0x9cf650] the Asian cups; [0xae23d0] database competitions; id; reads')]
INTERESSA = ['CMRTE']
