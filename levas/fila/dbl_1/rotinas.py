# Três consultas do Database.cpp usadas pelo contrato: na criação, CONTRACT_MANAGER::initialise_all_staff_contracts
# (0x4c7560, chamada por create_game 0x829809) gera o contrato de cada pessoa (0x85e960 → contrato padrão 0x864550,
# que pergunta se o clube está em temporada, 0x547770, e se é de uma das seis grandes nações, 0x549e50), e o
# game_run_to_start_date monta as listas iniciais (0x83a1b0 → salário padrão 0x862720, as mesmas duas). Os contratos
# vão para o .qst da prova do save. A 0x549ea0 (a mesma regra sobre a nação) só roda na regeneração dos jovens, na
# temporada. Sem SEH, sem sorteio, sem variável local; a 0x669bd0 chamada pela 0x547770 tem SEH, mas é chamada pelo
# endereço do jogo (a cópia não a reescreve).
SAVE = True
ROTINAS = [('clube_em_temporada', 0x547770, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('clube_em_nacao_grande', 0x549e50, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('nacao_grande', 0x549ea0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('as regras do contrato: o clube em temporada e o clube e a nação entre as seis grandes',
        'the contract rules: club in season and club and nation among the six major ones',
        [('clube_em_temporada', 'club_is_in_season', '0x547770', 'Database.cpp', 'cdecl',
          'clube nulo: 0; senão key_nation_club_in_season 0x669bd0(clube, nulo, nulo): 1 se hoje está dentro da temporada da nação do clube (o eax inteiro)',
          'null club: 0; otherwise key_nation_club_in_season 0x669bd0(club, null, null): 1 when today is inside the season of the club nation (the whole eax)'),
         ('clube_em_nacao_grande', 'club_is_in_major_nation', '0x549e50', 'Database.cpp', 'cdecl',
          '1 se a nação do clube (+0x53) é Inglaterra, França, Alemanha, Itália, Espanha ou Holanda (as chaves do index.cpp); clube ou nação nulos: 0',
          '1 when the club nation (+0x53) is England, France, Germany, Italy, Spain or Holland (the index.cpp keys); null club or nation: 0'),
         ('nacao_grande', 'nation_is_major_nation', '0x549ea0', 'Database.cpp', 'cdecl',
          'o mesmo teste das seis chaves sobre a própria nação; nula: 0',
          'the same six-key test on the nation itself; null: 0')])
CAMPOS = [('TClub +0x53: nação; ponteiro; lê (clube_em_nacao_grande)', 'TClub +0x53: nation; pointer; reads (clube_em_nacao_grande)'),
          ('TNation +0: id; inteiro; lê (clube_em_nacao_grande, nacao_grande). Chaves 0x9cf2e4, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338 (Inglaterra, França, Alemanha, Itália, Espanha, Holanda); lê',
           'TNation +0: id; integer; reads (clube_em_nacao_grande, nacao_grande). Keys 0x9cf2e4, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338 (England, France, Germany, Italy, Spain, Holland); reads')]
INTERESSA = ['SGE', 'CMRTE']
