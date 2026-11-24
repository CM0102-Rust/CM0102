# A lista dos clubes, as posicoes previstas e os codigos de progresso da segunda divisao turca categoria B
# (tur_second_b.cpp; leitor cx14, rodada 5, 24/11/2026). Simulacao com a Turquia (as ligas e a data dos itens mwd).
# - 0x901800 TurSecondB::get_club_array thiscall(&lista, &n) ret 8: n = 50; malloc 0xc8 (falta: caixa da linha 0x4c5,
#   n = 0); o grupo 1 (a tabela da liga) em 0..9 e cada uma das fases 0..3 em club_count + i, SEM o deslocamento das
#   fases anteriores (defeito do original: as quatro fases escrevem nas mesmas vagas 10..19 e as vagas 20..49 ficam
#   como o malloc deixou). 0 chamadas na cobertura do mundo: so o comparador a prova, se rodar.
# - 0x901950 TurSecondB::set_predicted_positions thiscall() ret 0: sem estatisticas a caixa da linha 0x4ec; malloc 0xc8
#   (caixa 0x4f4); para g = -1..3 os clubes do grupo, qsort 0x945333 (cdecl) com 0x4b07d0 e
#   Comp_Stats::set_club_final_position 0x4a2540 (thiscall(clube, pos), ret 8; le so o byte) com 5k + 1 (o mesmo para
#   todos os grupos); free. 2 chamadas na cobertura do mundo (criacao e virada).
# - 0x901ac0 TurSecondB::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: a tabela de saltos
#   0x901b88 (pior - 1: 0, 1 -> 1; 2, 3, 4 -> 5) conferida nos bytes; a conta em bytes (imul de 8 bits).
# Prologos: sub esp, 0x204 (6 bytes) nas duas primeiras; mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes) na terceira.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a lista so a original e as vagas conferidas; as posicoes
# previstas lado a lado (o byte de cada entrada); o progresso lado a lado (os 3 bytes da entrada).
# Os auxiliares grouped_league_club_slots / grouped_league_club_array / grouped_league_predicted_positions servem
# tambem a Segunda Divisao sueca e a Segunda Divisao B espanhola (ver o relatorio do cx14).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_second_b_get_club_array', 0x901800, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('tur_second_b_set_predicted_positions', 0x901950, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('tur_second_b_set_club_progress', 0x901ac0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('a lista dos clubes, as posições previstas e o progresso da segunda divisão turca categoria B',
        'the club list, the predicted positions and the progress of the Turkish second division category B',
        [('tur_second_b_get_club_array', 'TurSecondB::get_club_array', '0x901800', 'tur_second_b.cpp', 'thiscall, ret 8',
          'regra: a lista dos 50 clubes da categoria B: os dez do primeiro grupo e, depois deles, os de cada um dos outros quatro grupos; defeito do original: os quatro grupos são escritos nas mesmas dez vagas (só o último fica) e as outras trinta vagas ficam sem valor',
          'rule: the list of the 50 category B clubs: the ten of the first group and, after them, those of each of the other four groups; defect of the original: the four groups are written into the same ten slots (only the last stays) and the other thirty slots are left unset'),
         ('tur_second_b_set_predicted_positions', 'TurSecondB::set_predicted_positions', '0x901950', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da categoria B: cada um dos cinco grupos de dez é ordenado pela força e o k-ésimo clube (de 0 a 9) de cada grupo recebe a posição prevista 5k + 1',
          'rule: the predicted positions of category B: each of the five groups of ten is sorted by strength and the k-th club (0 to 9) of each group gets the predicted position 5k + 1'),
         ('tur_second_b_set_club_progress', 'TurSecondB::set_club_progress', '0x901ac0', 'tur_second_b.cpp', 'thiscall, ret 0x14',
          'regra: os códigos de progresso (atual, melhor, pior) de um clube da categoria B por fase: na primeira fase atual (5 × atual − 4) × 5 + 1, melhor 1 e pior 36 (pior abaixo de 3) ou 231; no grupo de acesso atual × 5 + 1 (de 3 para cima, senão 6), melhor 1 e pior 6 (1 e 2), 26 (3 a 5) ou pior × 5 + 1; nos grupos de classificação atual × 5 + 1, 1 e pior × 5 + 1; no mata-mata cada código 1 fica 1 e os outros viram 11; depois, como vieram',
          'rule: the progress codes (current, best, worst) of a category B club by stage: in the first phase current (5 × current − 4) × 5 + 1, best 1 and worst 36 (worst below 3) or 231; in the promotion group current × 5 + 1 (from 3 up, else 6), best 1 and worst 6 (1 and 2), 26 (3 to 5) or worst × 5 + 1; in the classification groups current × 5 + 1, 1 and worst × 5 + 1; in the play-off each code 1 stays 1 and the others become 11; later, as they came')])
CAMPOS = []
INTERESSA = []
