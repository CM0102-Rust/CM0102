# A semeadura e o sorteio dos grupos da Eurocopa (euro_champ.cpp; leitor cx05, rodada 5, 24/11/2026). Simulacao com
# as 8 ligas e ATE 2004-07-15: a Eurocopa do jogo e a de 2004; o dia (EuroChamp::update_for_date, ja em Rust) chama a
# semeadura em 10 de janeiro de 2004 e o sorteio em 10 de fevereiro de 2004; os jogos de 9 a 30 de junho. 0 chamadas
# na cobertura do mundo ate 2002-08-01: sem a data longa o item nao prova nada.
# - 0x57c710 EuroChamp::seed_teams thiscall() ret 0: malloc 0x944e46 e free 0x9452ca (cdecl), qsort 0x945333 com
#   0x4b07b0 (compare_clubs_by_reputation_desc, entradas de 6 bytes; cdecl), HostCountry::find_hosts 0x5fa730 (5 pushes;
#   mov ecx, [0xb5e134] -> thiscall, ret 0x14), database_get_national_team 0x53b3d0 (cdecl), comp_util_get_latest_
#   placed_club 0x4afd80 (push 0; push registro; add esp, 8 -> cdecl). Prologo sub esp, 0x228 (6 bytes).
# - 0x57c320 EuroChamp::draw_groups thiscall() ret 0 -> lista: calloc 0x945501 (cdecl), utils_random 0x90cfc0 (cdecl).
#   Prologo sub esp, 0x20c.
# - 0x57c1d0 EuroChamp::run_group_draw thiscall() ret 0: draw_groups (call direto), create_group_table 0x57b530
#   (push lista -> thiscall, ret 4), League::create_fixtures 0x6835c0 (thiscall), League::sort_table 0x6827d0 (push 0
#   -> thiscall, ret 4), create_group_stage 0x57b910 (push lista; push g -> thiscall, ret 8), as virtuais +0x5c e +0x8c.
#   Prologo sub esp, 0x200.
# Defeitos do original: (1) os dois indices de sede da semeadura sao locais nao iniciados (o find_hosts grava os dois
# quando acha a edicao; sem edicao o lixo da pilha seria lido como indice): a copia inicia com -1 (sem sede); (2) o laco
# dos cabecas de chave percorre 16 entradas da copia mesmo com menos inscritas (le alem do bloco); (3) o sorteio solta o
# calendario +0xba sem zerar e, nos retornos de erro, deixa a lista do sorteio alocada.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: a semeadura lado a lado nos 80 bytes da lista semeada (+0xee); o sorteio e a montagem so a original (o
# sorteio e aleatorio), com o plano: os quatro cabecas nos grupos 1 a 4 e quatro times por grupo; a tabela do grupo 1,
# as tres fases e +0x45 = 1.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-07-15'
ROTINAS = [('euro_champ_seed_teams', 0x57c710, [0x81, 0xec, 0x28, 0x02, 0x00, 0x00]),
           ('euro_champ_draw_groups', 0x57c320, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('euro_champ_run_group_draw', 0x57c1d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a semeadura e o sorteio da Eurocopa', 'the European Championship seeding and draw',
        [('euro_champ_seed_teams', 'EuroChamp::seed_teams', '0x57c710', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: as 16 seleções da Eurocopa vão para quatro potes de quatro por reputação; a primeira sede, a segunda sede e o detentor do título ficam à frente (as vagas 1, 2 e 3) e o resto se ordena depois deles',
          'rule: the 16 European Championship nations go into four pots of four by reputation; the first host, the second host and the holder go in front (places 1, 2 and 3) and the rest are sorted after them'),
         ('euro_champ_draw_groups', 'EuroChamp::draw_groups', '0x57c320', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: os quatro primeiros cabeças de chave vão para os grupos 1 a 4; as outras doze seleções, na ordem dos potes, vão cada uma para um grupo ao acaso que ainda não tenha time daquele pote',
          'rule: the four top seeds go into groups 1 to 4; the other twelve nations, in pot order, each go into a random group that has no team of that pot yet'),
         ('euro_champ_run_group_draw', 'EuroChamp::run_group_draw', '0x57c1d0', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: no dia do sorteio a Eurocopa sorteia os grupos, monta o grupo 1 na própria tabela com os jogos, os grupos 2 a 4 como fases, as posições previstas e as regras de novo',
          'rule: on the draw day the European Championship draws the groups, builds group 1 in its own table with the fixtures, groups 2 to 4 as stages, the predicted positions and the rules again')])
CAMPOS = [('Eurocopa +0xee lista semeada: 16 entradas de 5 bytes dentro do objeto (id da seleção, pote); lê e grava', 'European Championship +0xee seeded list: 16 entries of 5 bytes inside the object (national team id, pot); reads and writes')]
INTERESSA = []
