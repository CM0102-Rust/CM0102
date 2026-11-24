# Os construtores da Libertadores e da Mercosul e as regras da Mercosul (conmebol_liber.cpp, conmebol_merc.cpp; leitor
# cx03, rodada 5, 24/11/2026). Carreira de 8 ligas (o Brasil jogavel): os construtores rodam na criacao do jogo
# (setup.rs, create_game: as duas com o ano corrente).
# - 0x4c2f50 ConmebolLiber::ConmebolLiber e 0x4c46c0 ConmebolMerc::ConmebolMerc thiscall(ano, registro) ret 8 -> this: o
#   mesmo corpo (conferido instrucao a instrucao): League::League 0x682200 (thiscall, ret 0); +4 = registro, a vtable
#   (0x968bf4 / 0x968ca8), +0x40 = ano, +0x50 = 2, +0xee = 0, +0x51 = 0xf; League::load_from_temp_file 0x687b10 (push 1 ->
#   thiscall, ret 4) != 0: this. Novo: +0xd9 = 0x28, +0x44 = 0xff, +0x30 = -1, +0x2c = 8 / 5 e +0xc = malloc(0x20 /
#   0x14); so na Libertadores ConmebolSeeding::update_for_new_season 0x4c61b0 (mov ecx, [0xae1000]; push 0 -> thiscall,
#   ret 4); as regras (0x4c3890 / 0x4c4f50, thiscall, ret 0; 0: a caixa 0x52 / 0x4a e this); +0x3e = 0, +0xb1 = 0,
#   +0x45 = 0; Comp_Stats (0x944cf1 de 0x5ce; 0x49ee70 push this -> thiscall) em +8 (sem: a caixa 0x60 / 0x58). SEH so
#   para o new. Prologo push -1; push imm32 (7 bytes).
# - 0x4c4f50 ConmebolMerc::set_up_rules thiscall() ret 0 -> 1: os campos (+0xc2, +0xc7, +0x4a = 3; +0x3c = 2; +0xc6, +0x42
#   = 2; +0xc3 = 1; +0xc4 = 7; +0xc5 = 1; +0xbe = 1; +0xbf, +0xc0, +0xc1 = 0; +0x1c, +0x20 = -1; +0x49 = 7), Date(0xf, 5,
#   ano, -1) (0x549f70, thiscall, ret 0x10) com a palavra em +0xb6 e +0xb8 = 0, +0x28 = 2.000.000 e a tabela pela virtual
#   0x3c (push 0; &+0x3a; &+0xa9; -1) em +0xba: o molde apply_cup_rules (HEAD), como a Libertadores e a Merconorte. SEH
#   sem destrutor no caminho normal.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original (alocam e leem o save; as regras alocam a tabela), com o plano (cup_constructor_plan_mismatch
# e check_cup_rules).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_conmebol_liber', 0x4c2f50, [0x6a, 0xff, 0x68, 0x39, 0x6a, 0x95, 0x00]),
           ('construct_conmebol_merc', 0x4c46c0, [0x6a, 0xff, 0x68, 0xe9, 0x6a, 0x95, 0x00]),
           ('conmebol_merc_set_up_rules', 0x4c4f50, [0x6a, 0xff, 0x68, 0x28, 0x6b, 0x95, 0x00])]
DOCS = ('os construtores da Libertadores e da Mercosul e as regras da Mercosul',
        'the constructors of the Copa Libertadores and the Copa Mercosur, and the Mercosur rules',
        [('construct_conmebol_liber', 'ConmebolLiber::ConmebolLiber', '0x4c2f50', 'conmebol_liber.cpp', 'thiscall, ret 8',
          'a Libertadores na criação da carreira: regra disciplinar 2 (a sul-americana) e importância 15; uma copa nova tem oito fases, a nova temporada do sorteio da Conmebol, as regras e as estatísticas',
          'the Copa Libertadores at career creation: discipline rule 2 (the South American one) and importance 15; a new cup has eight stages, the Conmebol seeding of the new season, the rules and the statistics'),
         ('construct_conmebol_merc', 'ConmebolMerc::ConmebolMerc', '0x4c46c0', 'conmebol_merc.cpp', 'thiscall, ret 8',
          'a Mercosul na criação da carreira, como a Libertadores, com cinco fases e sem o passo do sorteio',
          'the Copa Mercosur at career creation, as the Libertadores, with five stages and no seeding step'),
         ('conmebol_merc_set_up_rules', 'ConmebolMerc::set_up_rules', '0x4c4f50', 'conmebol_merc.cpp', 'thiscall, ret 0',
          'regra: os grupos da Mercosul são de ida e volta, 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2 e 3, um classificado direto por grupo, clubes de várias nações, categoria 7, 7 reservas e 3 substituições; a temporada começa no dia 15 do mês 5 do ano da competição [inferência: 15 de junho]; prêmio total de 2.000.000',
          'rule: the Copa Mercosur groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3, one club going through directly from each group, clubs from several nations, category 7, 7 substitutes named and 3 changes; the season starts on day 15 of month 5 of the competition year [inference: 15 June]; prize fund of 2,000,000')])
CAMPOS = []
INTERESSA = []
