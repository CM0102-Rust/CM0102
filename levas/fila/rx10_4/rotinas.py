# As ligas divididas em secoes (comum) e o construtor, as regras e a tabela da Regionalliga alema (ger_regional.cpp;
# leitor rx10, rodada 6, 25/11/2026). Simulacao com a Alemanha e a base NORMAL (a Regionalliga e divisao inferior:
# so e carregada com a base NORMAL; 0 chamadas na cobertura do mundo ate 2002-08-01), como o sbE_11. Rodam na criacao
# do jogo (2001) e, as regras e a tabela, na virada alema no verao de 2002.
# - 0x5dcbd0 GerRegional::GerRegional thiscall(ano, registro) ret 8: construct_divided_league (o mesmo corpo do
#   PorSecondB::PorSecondB 0x7d1920, conferido instrucao a instrucao): League::League, vtable 0x96b250, disciplina 0xd,
#   0x687b10(1) termina; +0x2c = 1 e o vetor de fases (malloc 4); regras 0x5dd9f0 (caixa 0x4a), tabela 0x5dd7f0 (0x51),
#   ordem 0x6827d0(0) (0x58), estatisticas (0x60), jogos 0x6835c0 (0x67); a fase Sul 0x5ddc50(0) e as posicoes
#   previstas 0x5de130 chamadas direto. Prologo push -1; push 0x95a629 (7 bytes).
# - 0x5dd9f0 GerRegional::set_up_rules thiscall() ret 0: os campos (+0xc1 = 5 em 2000, senao 4; acima a 2. Bundesliga
#   0x9cf550; abaixo -1; +0xd9 = 0x42), a tabela de rodadas pela virtual +0x3c(-1, +0xa9, +0x3a, 0). Prologo push esi;
#   mov esi, ecx; mov eax, 2 (8 bytes).
# - 0x5dd7f0 GerRegional::create_table thiscall() ret 0: +0x3e = 19 em 2000 (senao 18), +0xa7 = -1; fill_section_table
#   (os clubes com divisao 0x9cf564 e a reserva 0x9cf55c, a Norte; o time B com o flag 0 da 0x540a50 fica com +0x37 =
#   0xfc; caixas 0x13b e 0x14c, o clube que sobra perde a divisao). Prologo sub esp, 0x208 (6 bytes).
# Auxiliares novos (pub(crate), usados pelos rx10_5 a rx10_9): DividedLeague, construct_divided_league,
# compare_divided_league_ctor, fill_section_table, compare_section_table, SectionStage, create_section_stage,
# compare_section_stage, divided_league_turnover, divided_league_club_array, compare_divided_league_club_array,
# section_predicted_positions e compare_section_predicted.
# Iguais no cm0102-1080.exe. Comparadores: o construtor e a tabela so a original com o plano; as regras em ensaio.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('construct_ger_regional', 0x5dcbd0, [0x6a, 0xff, 0x68, 0x29, 0xa6, 0x95, 0x00]),
           ('ger_regional_set_up_rules', 0x5dd9f0, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('ger_regional_create_table', 0x5dd7f0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as ligas divididas em seções e o construtor, as regras e a tabela da Regionalliga alemã',
        'the leagues divided into sections and the German Regionalliga constructor, rules and table',
        [('construct_ger_regional', 'GerRegional::GerRegional', '0x5dcbd0', 'ger_regional.cpp', 'thiscall, ret 8',
          'regra: a Regionalliga é uma liga do ano do jogo com as regras disciplinares da liga alemã, em duas seções (a Norte na tabela própria, a Sul como fase); sem estado salvo, monta as regras, a tabela Norte, a ordem, as estatísticas, os jogos, a fase Sul e as posições previstas',
          'rule: the Regionalliga is a league of the game\'s year under the German league discipline rules, in two sections (North in its own table, South as a stage); with nothing saved, it builds the rules, the North table, the order, the statistics, the fixtures, the South stage and the predicted positions'),
         ('ger_regional_set_up_rules', 'GerRegional::set_up_rules', '0x5dd9f0', 'ger_regional.cpp', 'thiscall, ret 0',
          'regra: a Regionalliga tem turno e returno, 3 pontos por vitória e 1 por empate; dois de cada seção sobem para a 2. Bundesliga e quatro caem (cinco em 2000), sem liga abaixo',
          'rule: the Regionalliga is played home and away, 3 points a win and 1 a draw; two from each section go up to the 2. Bundesliga and four go down (five in 2000), with no league below'),
         ('ger_regional_create_table', 'GerRegional::create_table', '0x5dd7f0', 'ger_regional.cpp', 'thiscall, ret 0',
          'regra: a Regionalliga Norte tem 18 clubes (19 em 2000), os da seção Norte na ordem da base; o time B cujo clube principal está acima não pode subir',
          'rule: the Regionalliga North has 18 clubs (19 in 2000), those of the North section in database order; a B team whose senior club is above cannot go up')])
CAMPOS = []
INTERESSA = []
