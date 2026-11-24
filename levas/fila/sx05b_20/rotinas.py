# Os premios da Copa do Mundo (award/world_cup_awards.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa os construtores
# comuns dos premios das nacoes (sx05b_5) e nation_award (sx03_2): integrar depois deles.
# O construtor e o init na criacao do jogo (AWARD_MANAGER 0x4135f7, o grupo internacional), o daily_update todo dia;
# os premios so em 13 de julho de ano de Copa (2002-07-13): LIGAS = 'ENG' e ATE = '2002-08-01', como os premios do fim
# da temporada inglesa (sx05_2, sx04_2).
# Convencoes conferidas na desmontagem:
# - 0x92fc10 thiscall(arquivo) ret 4: NationAwards::NationAwards 0x768290; vtable 0x9713a4; +0xe = 0; +0x13 = 0;
#   +0x12 = 4; com arquivo, 0x76a280; senao (ou falhando) 0x92fc80(arquivo);
# - 0x92fc80 thiscall(arquivo) ret 4: +8 = 2, malloc(8) (nulo: show_error_at linha 0x5d); YEAR_AWARD 0x92fe80 e
#   TEAM_AWARD 0x8af520 (argumentos tirados por script e conferidos a mao);
# - 0x92fdf0 thiscall(hoje) ret 4: (ano - 0x7ce) com resto com sinal por 4 (and 0x80000003 e o ajuste do sinal);
#   Date::Date 0x549f70 (0xd, 6, ano, 0) (o ultimo argumento 0, nao -1); dia e ano comparados como words;
#   nation_awards_award_tournament_player 0x7690c0 (comp, hoje, premio, -1, 0, 2) ret 0x18 (o ecx e o this, nao lido)
#   e NationAwards::award_tournament_team 0x769af0 thiscall(comp, premio) ret 8; sem a atualizacao da base 0x768310.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('world_cup_awards_construct', 0x92fc10, [0x6a, 0xff, 0x68, 0x28, 0x60, 0x96, 0x00]),
           ('world_cup_awards_init', 0x92fc80, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('world_cup_awards_daily_update', 0x92fdf0, [0x83, 0xec, 0x10, 0x56, 0x8b, 0x74, 0x24, 0x18])]
DOCS = ('os prêmios da Copa do Mundo',
        'the World Cup awards',
        [('world_cup_awards_construct', 'WorldCupAwards::WorldCupAwards', '0x92fc10', 'award/world_cup_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Copa: a base com a tabela virtual da classe, sem nação, o id 0 e o tipo 4 (internacional); com arquivo, os salvos; senão os dois prêmios',
          'builds the World Cup awards: the base with the class vtable, no nation, id 0 and type 4 (international); with a file, the saved ones; otherwise the two awards'),
         ('world_cup_awards_init', 'WorldCupAwards::init_awards', '0x92fc80', 'award/world_cup_awards.cpp', 'thiscall, ret 4',
          'cria os dois prêmios da Copa: a chuteira de ouro (reputações 9000 e 7000) e a seleção da Copa (8500 e 6750)',
          'creates the two World Cup awards: the golden boot (reputations 9000 and 7000) and the dream team (8500 and 6750)'),
         ('world_cup_awards_daily_update', 'WorldCupAwards::daily_update', '0x92fdf0', 'award/world_cup_awards.cpp', 'thiscall, ret 4',
          'em ano de Copa (o ano menos 1998 divisível por 4), em 13 de julho, a chuteira de ouro (os três melhores da competição pela estatística 2) e a seleção da Copa; sem a atualização da base',
          "in a World Cup year (the year minus 1998 divisible by 4), on 13 July, the golden boot (the competition's best three by statistic 2) and the dream team; no base daily update")])
CAMPOS = []
INTERESSA = []
