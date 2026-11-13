# O gerente de prêmios: os prêmios de uma nação e os arquivos de histórico dos prêmios (AWARD_MANAGER,
# award/award_manager.cpp; leitor mwg, rodada 2 do mundo inteiro, 12/11/2026). find_nation_awards roda 8.073 vezes até
# 2002-08-01 no mundo inteiro (cobertura_mundo.txt: notícias e prêmios do mês/ano consultam os prêmios da nação);
# add_year_award_record (265, chamada por 0x930309, YEAR_AWARD) e add_team_award_record (32, por 0x8affcf, TEAM_AWARD)
# gravam o histórico dos prêmios entregues no fim da temporada. Rodam na carreira só com a Inglaterra (os prêmios da
# Inglaterra e os continentais/mundiais): LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x4142c0..0x4142d6, 0x414300..0x414317, 0x414340..0x4143e4):
# - add_year/team_award_record: thiscall(this, registro, primeiro, último), ret 0xc; repassam a
#   FILE_LLIST::add_node (0x58e340) thiscall(ecx = [this] ou [this+4]; primeiro, último, registro), ret 0xc, e
#   devolvem o eax dela;
# - find_nation_awards: thiscall(this, nação), ret 4 -> o objeto (eax inteiro); sem nação a caixa de erro (linha 0x2b3
#   de award_manager.cpp, 0x987e24) e 0.
# Comparadores: os dois repasses só a original (gravam no arquivo de histórico: não rodam duas vezes);
# find_nation_awards pelo valor (compare_value!).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('award_manager_add_year_award_record', 0x4142c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x0c]),
           ('award_manager_add_team_award_record', 0x414300, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x0c]),
           ('find_nation_awards', 0x414340, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o gerente de prêmios: os prêmios de uma nação e o histórico dos prêmios',
        "the award manager: a nation's awards and the award history",
        [('award_manager_add_year_award_record', 'AWARD_MANAGER::add_year_award_record', '0x4142c0', 'award/award_manager.cpp', 'thiscall, ret 0xc',
          'acrescenta um registro de prêmio do ano ao arquivo de histórico dos prêmios do ano (a primeira lista em arquivo, FILE_LLIST::add_node com primeiro, último e o registro)',
          'appends a year award record to the year award history file (the first file list, FILE_LLIST::add_node with first, last and the record)'),
         ('award_manager_add_team_award_record', 'AWARD_MANAGER::add_team_award_record', '0x414300', 'award/award_manager.cpp', 'thiscall, ret 0xc',
          'acrescenta um registro de prêmio de equipe ao arquivo de histórico dos prêmios de equipe (a segunda lista em arquivo)',
          'appends a team award record to the team award history file (the second file list)'),
         ('find_nation_awards', 'AWARD_MANAGER::find_nation_awards', '0x414340', 'award/award_manager.cpp', 'thiscall, ret 4',
          'o objeto de prêmios de uma nação: a primeira das 30 vagas com um objeto de tipo 1 (+0x12) daquela nação (+0xe), ou 0; sem nação, a caixa de erro (linha 0x2b3) e 0',
          "a nation's award object: the first of the 30 slots holding one of kind 1 (+0x12) for that nation (+0xe), or 0; without a nation, the error box (line 0x2b3) and 0")])
