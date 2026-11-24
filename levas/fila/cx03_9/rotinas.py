# As noticias do destino dos clubes nas duas copas galesas e o progresso na Copa da Liga galesa (wel_lge_cup.cpp,
# wel_prm_cup.cpp; leitor cx03, rodada 5, 24/11/2026). Competicoes de Gales: LIGAS do grupo ENG SCO WAL, ate 2002-08-01.
# - 0x9265e0 WelLgeCup::build_fate_news_text thiscall(clube, destino, fase, a4..a8, noticia, texto) ret 0x28 e 0x927f40
#   WelPrmCup::build_fate_news_text: o mesmo corpo (conferido instrucao a instrucao, e tambem igual ao das tres
#   continentais americanas do cx03_14), com o mata-mata na fase 5 e na fase 1; o texto 0x9a7e9c "through to quarter
#   finals". Dentro: 0x4b4590 (6 pushes), 0x4b0b80 (6 pushes), 0x4b4420 (4 pushes), 0x66f4e0 (5 pushes), todos cdecl;
#   0x4ae660 (thiscall, ret 4) e 0x4ae8a0 (thiscall, ret 0xc). Prologo mov al, [esp+0xc]; cmp al, k (6 bytes).
# - 0x927760 WelLgeCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (a virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (push clube; mov ecx, [this+8] -> thiscall, ret 4); fase < 5: atual e melhor
#   1, 2 -> 1 e 3 -> 9 (0 e acima de 3 ficam), pior 1 -> 5 e 2, 3 -> 13; senao como vem; +0x73 atual, +0x74 melhor,
#   +0x75 pior. Prologo mov eax, [esp+4]; mov ecx, [ecx+8].
# Usa compare_club_progress do cx02_4 (integrar depois dele). O molde das noticias (write_group_cup_fate_news) e o
# gravador do progresso (store_club_progress) servem aos cx03_10, cx03_12, cx03_14, cx03_15 e cx03_16.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o texto "through to quarter finals" lado a lado (compare_text_writer, modo CM_TEXTO_NOTICIAS); os outros
# caminhos passam o objeto da noticia adiante: so a original. O progresso lado a lado nos tres bytes da entrada.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_lge_cup_build_fate_news_text', 0x9265e0, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x05]),
           ('wel_prm_cup_build_fate_news_text', 0x927f40, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x01]),
           ('wel_lge_cup_set_club_progress', 0x927760, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as notícias do destino dos clubes nas duas copas galesas e o progresso na Copa da Liga galesa',
        "the fate news of the two Welsh league cups and the Welsh League Cup progress",
        [('wel_lge_cup_build_fate_news_text', 'WelLgeCup::build_fate_news_text', '0x9265e0', 'wel_lge_cup.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Copa da Liga galesa: nos grupos (fases abaixo de 5) a classificação e a eliminação pela frase genérica dos grupos, ou "{clube} through to quarter finals" para quem passa sem objeto de notícia; no mata-mata (fase 5) o resultado do confronto',
          "rule: the Welsh League Cup news of a club's fate: in the groups (stages below 5) qualification and elimination through the generic group sentence, or \"{club} through to quarter finals\" for a club going through with no news object; in the knockout (stage 5) the tie result"),
         ('wel_prm_cup_build_fate_news_text', 'WelPrmCup::build_fate_news_text', '0x927f40', 'wel_prm_cup.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Copa da Premier galesa, como a da Copa da Liga, com o mata-mata na fase 1',
          "rule: the Welsh Premier Cup news of a club's fate, as the League Cup's, with the knockout at stage 1"),
         ('wel_lge_cup_set_club_progress', 'WelLgeCup::set_club_progress', '0x927760', 'wel_lge_cup.cpp', 'thiscall, ret 0x14',
          'regra: nos grupos da Copa da Liga galesa (fases abaixo de 5) os lugares no grupo viram códigos de progresso na copa: 1º e 2º dão 1 e 3º dá 9 no atual e no melhor, e no pior 1º dá 5 e 2º e 3º dão 13; os outros códigos ficam; do mata-mata em diante os códigos ficam como vêm',
          'rule: in the Welsh League Cup groups (stages below 5) the group places become cup progress codes: places 1 and 2 give 1 and place 3 gives 9 for the current and best codes, and for the worst place 1 gives 5 and places 2 and 3 give 13; other codes stay; from the knockout on the codes stay as they come')])
CAMPOS = []
INTERESSA = []
