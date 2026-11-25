# O texto das noticias, a lista dos clubes e as posicoes previstas da Segunda Divisao B espanhola (spa_second_b.cpp;
# leitor rx09, rodada 6, 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01.
# - 0x857e20 SpaSecondB::build_fate_news_text thiscall(clube, desfecho, fase, a4..a8, noticia, texto) ret 0x28: os
#   pushes das tres chamadas conferidos (0x48c6d0 com a7 = 0, e o desfecho 0 empurrado como constante nos grupos do
#   acesso; 0x4b0b80 cdecl (clube, a5, a8, desfecho, a6, texto)); o texto com o nome da fase: 0x4b4850(buf, 0x64, a4, 0)
#   e 0x4ba8f0(a4, 0), sprintf 0x66f4e0 de 0xa829ac (generos do clube, da fase e da competicao +0x37; nome curto do
#   clube, nome da fase, nome da competicao +4), os dois vinculos 0x7d5 e 0x7d0.
# - 0x8581a0 SpaSecondB::get_club_array thiscall(&lista, &n) ret 8: n = 80, malloc 0x140 (caixa 0x4f2, n = 0); o grupo 1
#   em 0..19 e cada fase 0..2 em +0x3e + i, i < +0x3e da propria liga (o defeito do cx14_3: sem deslocamento).
# - 0x8582f0 SpaSecondB::set_predicted_positions thiscall() ret 0: sem estatisticas a caixa 0x519; malloc 4 x +0x3e
#   (caixa 0x521); g = -1..2, qsort com 0x4b07d0 e set_club_final_position 0x4a2540 com 4k + 1; free.
# Prologos: mov eax, [esp+0xc]; sub esp, 0x64 (7 bytes); sub esp, 0x204 (6 bytes) nas outras duas. Iguais no
# cm0102-1080.exe.
# Comparadores: os textos proprios lado a lado (compare_text_writer), o comum e o do mata-mata so a original; a lista so
# a original e as vagas conferidas; as posicoes previstas lado a lado.
# Depende do cx13_6 (play_off_club_text), do cx14_3 (grouped_league_club_array, grouped_league_predicted_positions e os
# comparadores) e do rx09_1 (SEGUNDA_B_SOURCE).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('spa_second_b_build_fate_news_text', 0x857e20, [0x8b, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x64]),
           ('spa_second_b_get_club_array', 0x8581a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('spa_second_b_set_predicted_positions', 0x8582f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o texto das notícias, a lista dos clubes e as posições previstas da Segunda Divisão B espanhola',
        'the fate news text, the club list and the predicted positions of the Spanish Second Division B',
        [('spa_second_b_build_fate_news_text', 'SpaSecondB::build_fate_news_text', '0x857e20', 'spa_second_b.cpp', 'thiscall, ret 0x28',
          'regra: nos grupos do acesso, quem não sobe recebe "não conseguiu o acesso no grupo tal da Segunda B" (com notícia) ou "não consegue o acesso"; na repescagem e na final, sem notícia, "escapa do rebaixamento", "fica mais perto do rebaixamento" (repescagem) ou "é rebaixado" (final); com notícia, o texto do mata-mata',
          'rule: in the promotion play-off groups, a club that does not go up gets "have failed to win promotion from group such in the Second Division B" (with a news item) or "fail to win promotion"; in the relegation play-off and its final, without a news item, "avoid relegation", "slip nearer relegation" (play-off) or "relegated" (final); with one, the cup tie text'),
         ('spa_second_b_get_club_array', 'SpaSecondB::get_club_array', '0x8581a0', 'spa_second_b.cpp', 'thiscall, ret 8',
          'regra: a lista dos 80 clubes da Segunda B: os 20 do grupo 1 e, depois deles, os de cada um dos outros três grupos; defeito do original: os três grupos são escritos nas mesmas vinte vagas (só o último fica) e as outras quarenta ficam sem valor',
          'rule: the list of the 80 Second Division B clubs: the 20 of group 1 and, after them, those of each of the other three groups; defect of the original: the three groups are written into the same twenty slots (only the last stays) and the other forty are left unset'),
         ('spa_second_b_set_predicted_positions', 'SpaSecondB::set_predicted_positions', '0x8582f0', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Segunda B: cada um dos quatro grupos é ordenado pela força e o k-ésimo clube de cada grupo recebe a posição prevista 4k + 1',
          'rule: the predicted positions of the Second Division B: each of the four groups is sorted by strength and the k-th club of each group gets the predicted position 4k + 1')])
CAMPOS = []
INTERESSA = []
