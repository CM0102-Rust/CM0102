# A fase final da Copa do Mundo: o construtor e a lista de seleções (world_cup.cpp). Leitor cqz (12/11/2026), lista de 8
# ligas.
# - 0x92bf50 (WorldCup::WorldCup), 0x92da40 (WorldCup::set_up_team_list).
# O construtor: a edição no ciclo de 4 anos a partir de 1994 começando no ano seguinte ao do jogo (2002 num jogo de
# 2001), o livro de 32 vagas (calloc) ANTES da carga, a lista e as regras; no fim, a Copa do próprio ano tem as 32,
# senão +0x45 = 0; +0xb1 e +0x3e zerados. A lista: o campeão (status 2), as sedes (status 1; a sede campeã conta uma
# vez), com uma sede só a China na vaga 2, sem sedes Islândia e Hungria e contagem 3; as vagas 3 a 31 com uma lista
# fixa de 29 seleções; em 2002, as classificadas da CONCACAF e da África (status 6) na contagem, por cima da lista
# fixa: Costa Rica, um sorteio do par (1 em 3 Estados Unidos e Honduras; senão metade Estados Unidos e México, metade
# Honduras e México), Camarões, Nigéria, Senegal, Tunísia e África do Sul (caixa da linha 0x456 com o livro cheio).
# Convenções conferidas: o construtor thiscall, ret 8; a lista thiscall, ret 0; 0x945501, 0x4afd80, 0x53b3d0, 0x90cfc0
# (cdecl), 0x5fa730 (5 pushes; mov ecx -> ret 0x14; a primeira sede na primeira saída).
# Comparadores: só a original (alocam; a lista sorteia), com o plano.
# Cuidado na integração: usa compare_nations_tournament_ctor (motor.rs, já em Rust).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_world_cup', 0x92bf50, [0x6a, 0xff, 0x68, 0x89, 0x5f, 0x96, 0x00]),
           ('world_cup_set_up_team_list', 0x92da40, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a fase final da Copa do Mundo: o construtor e a lista de seleções',
        'the World Cup finals: the constructor and the nation list',
        [('construct_world_cup', 'WorldCup::WorldCup', '0x92bf50', 'world_cup.cpp', 'thiscall, ret 8', 'regra: a Copa do Mundo é de quatro em quatro anos a partir de 1994; a edição do jogo é a seguinte ao ano do jogo (num jogo de 2001, a de 2002); 32 vagas, 8 fases, a lista de seleções, as regras e as estatísticas; a Copa do próprio ano já tem as 32', "rule: the World Cup is every four years from 1994; the game's edition is the one after the game's year (in a 2001 game, 2002); 32 places, 8 stages, the nation list, the rules and the statistics; the World Cup of the game's own year already has its 32"),
         ('world_cup_set_up_team_list', 'WorldCup::set_up_team_list', '0x92da40', 'world_cup.cpp', 'thiscall, ret 0', 'regra: o campeão primeiro, depois as sedes (a sede campeã conta uma vez), depois uma lista fixa de 29 seleções; na Copa de 2002 entram já classificadas as da CONCACAF e da África: Costa Rica, duas entre Estados Unidos, Honduras e México (sorteadas: 1 em 3 Estados Unidos e Honduras, senão metade Estados Unidos e México, metade Honduras e México), Camarões, Nigéria, Senegal, Tunísia e África do Sul', 'rule: the holder first, then the hosts (a host that is the holder counts once), then a fixed list of 29 nations; for the 2002 World Cup the CONCACAF and African qualifiers come in already qualified: Costa Rica, two of the United States, Honduras and Mexico (drawn: one in three the United States and Honduras, else half the United States and Mexico, half Honduras and Mexico), Cameroon, Nigeria, Senegal, Tunisia and South Africa')])
CAMPOS = [('Copa do Mundo +0x14 livro de inscritas (6 bytes por vaga: +0 seleção, +4 status 1 sede, 2 campeão, 6 classificada), +0x36/+0x38 inscritas/vagas, +0x4b, +0x45, +0xb1, +0x3e; valor/ponteiro; grava', 'World Cup +0x14 entrant book (6 bytes per slot: +0 team, +4 status 1 host, 2 holder, 6 qualified), +0x36/+0x38 entrants/places, +0x4b, +0x45, +0xb1, +0x3e; value/pointer; writes')]
