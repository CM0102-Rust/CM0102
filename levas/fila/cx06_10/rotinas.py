# Os resultados, as posicoes e o progresso das selecoes nas eliminatorias da Eurocopa (euro_champ_qual.cpp; leitor cx06,
# rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15: os jogos dos grupos vao de 2/9/2002 a 7/10/2003
# (cx06_4); na cobertura do mundo ate 2002-08-01 as tres tem 0 chamadas. O mesmo ATE do cx04_6 (a mesma leva).
# - 0x57ffd0 EuroChampQual::add_result thiscall(partida, estatisticas) ret 8 (virtual 4): partida de fase de grupo (byte
#   +0x42, com sinal, ate +0xff): uma copia (LLIST::add_tail 0x68d0d0, 0x4f bytes) na lista +0x10; depois
#   League::add_result 0x685d30 (thiscall, ret 8) e o resultado dele. Prologo push esi; push edi; mov edi, [esp+0xc]
#   (6 bytes).
# - 0x580010 EuroChampQual::update_positions_and_status thiscall() ret 0 (virtual 0x94): League::update_positions_and_status
#   0x684cd0 (ja em Rust, pelo endereco); com +0xfb zero e a virtual 0x10 (push 0; push 1 -> al) verdadeira,
#   mark_qualified_teams 0x580040 (cx06_11, pelo endereco). Prologo push esi; mov esi, ecx; call rel32 (3 bytes antes do
#   call): RELATIVAS (0x580010, 3), ver INTEGRAR_ANTES.txt.
# - 0x580980 EuroChampQual::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68): a
#   entrada do clube (Comp_Stats::find_club_entry 0x4a4850, sem conferir); fase de grupo (ate +0xff, com sinal): melhor
#   1..6 -> 1, 10, 19, 28, 37, 46 (tabela de salto 0x580b00), atual e pior 1..6 -> 1, 16, 19, 28, 37, 46 (0x580b30 e
#   0x580b18); repescagem (+0x101): 1 -> 13, 2 -> 16 nos tres; outros codigos e fases ficam; +0x74, +0x73, +0x75 nessa
#   ordem.
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: o resultado so a original (a liga nao recebe o resultado duas vezes), com o plano (a lista cresce um na
# fase de grupo); as posicoes so a original (os anuncios), com o plano (+0xfb nao volta a zero); o progresso lado a lado
# nos 3 bytes (compare_group_championship_progress do cx04_5).
# Depende do cx04_5 (compare_group_championship_progress): integrar depois dele.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('euro_champ_qual_add_result', 0x57ffd0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('euro_champ_qual_update_positions_and_status', 0x580010, [0x56, 0x8b, 0xf1, 0xe8, 0xb8, 0x4c, 0x10, 0x00]),
           ('euro_champ_qual_set_club_progress', 0x580980, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('os resultados, as posições e o progresso das seleções nas eliminatórias da Eurocopa',
        "the European Championship qualifiers' results, positions and nation progress",
        [('euro_champ_qual_add_result', 'EuroChampQual::add_result', '0x57ffd0', 'euro_champ_qual.cpp', 'thiscall, ret 8',
          'recebe um resultado das eliminatórias: um jogo de fase de grupo é guardado numa lista (para a tabela dos segundos colocados) antes de entrar na liga',
          "takes a qualifiers' result: a group stage game is kept in a list (for the runners-up table) before it goes into the league"),
         ('euro_champ_qual_update_positions_and_status', 'EuroChampQual::update_positions_and_status', '0x580010', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'atualiza as posições e as situações das eliminatórias e, quando a competição termina, marca uma vez só os classificados',
          "updates the qualifiers' positions and statuses and, once the competition is complete, marks the qualified nations once"),
         ('euro_champ_qual_set_club_progress', 'EuroChampQual::set_club_progress', '0x580980', 'euro_champ_qual.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de uma seleção nas eliminatórias: nos grupos os códigos 1 a 6 viram 1, 10 (o melhor) ou 16 (o atual e o pior), 19, 28, 37 e 46; na repescagem 1 e 2 viram 13 e 16',
          "stores a nation's progress in the qualifiers: in the groups the codes 1 to 6 become 1, 10 (the best) or 16 (the current and the worst), 19, 28, 37 and 46; in the play-off 1 and 2 become 13 and 16")])
CAMPOS = []
INTERESSA = []
