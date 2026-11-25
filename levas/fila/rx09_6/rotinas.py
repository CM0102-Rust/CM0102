# Os codigos de progresso da Segunda Divisao B espanhola (spa_second_b.cpp; leitor rx09, rodada 6, 25/11/2026). So
# existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01 (o fim de cada fase).
# - 0x858460 SpaSecondB::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: Comp_Stats::
#   find_club_entry 0x4a4850 sem conferir; fase (byte com sinal) < 3: melhor 1..4 -> 1, pior 1..4 -> 4, os tres x 4 - 3;
#   3..6: as tres tabelas de saltos 0x858678/0x858688/0x858698 conferidas nos bytes (1, 5, 9, 0xd; fora de 1..4 fica);
#   7 e 8: o registrador dl (o melhor) recebe 0x3b/0x3f do melhor, do pior e do atual (1 e 3 na fase 7, 1 e 2 na 8) e o
#   atual e o pior ficam como vieram (defeito do original); depois, como vieram.
# Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes); igual no cm0102-1080.exe.
# Comparador: os 3 bytes da entrada lado a lado.
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('spa_second_b_set_club_progress', 0x858460, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os códigos de progresso da Segunda Divisão B espanhola', 'the progress codes of the Spanish Second Division B',
        [('spa_second_b_set_club_progress', 'SpaSecondB::set_club_progress', '0x858460', 'spa_second_b.cpp', 'thiscall, ret 0x14',
          'regra: os códigos de progresso (atual, melhor, pior) de um clube da Segunda B por fase: nos grupos melhor 1 e pior 4 para quem está entre os quatro primeiros, e cada código vira 4v − 3; nos grupos do acesso os códigos 1 a 4 viram 1, 5, 9 e 13; na repescagem e na final o vencedor vira 0x3b e o perdedor 0x3f, só no código melhor (defeito do original: o atual e o pior ficam como vieram)',
          'rule: the progress codes (current, best, worst) of a Second Division B club by stage: in the groups best 1 and worst 4 for the top four, and each code becomes 4v − 3; in the promotion play-off groups the codes 1 to 4 become 1, 5, 9 and 13; in the relegation play-off and its final the winner becomes 0x3b and the loser 0x3f, only in the best code (defect of the original: the current and worst codes stay as they came)')])
CAMPOS = []
INTERESSA = []
