# O acesso e o rebaixamento da Belgica, os movimentos da Terceira Divisao e a divisao dela em grupos (bel_first.cpp;
# leitor cx08, rodada 5, 24/11/2026). Simulacao com a Belgica (as ligas do cx02_21: 'ENG BEL HOL GER DEN', ate
# 2002-08-01): as tres rodam na virada belga do verao de 2002 (BelFirst::start_new_season, mwc_25, 1 chamada).
# - 0x41ec90 bel_first_move_third_division_clubs thiscall() ret 0: na Terceira ([0x9cf7c4]), a tabela de times da fase 9
#   (+0xa7, entradas de 6 bytes, status +5 = 1 e divisao != +4 da Terceira): move_club_to_division 0x6830b0 (push 1;
#   push +4; push clube); as linhas k da tabela da Terceira e da fase 0 com status 3: relegate_club_to_division 0x6831a0
#   para a Quarta A ([0x9cf7d4]) e a Quarta B ([0x9cf7d8]); os quatro clubes guardados em +0xee..+0xfa da Terceira:
#   move_club_to_division para a Terceira. Prologo push ecx; mov eax, [0xadadfc] (6 bytes).
# - 0x41ee80 BelFirst::process_promotion_relegation_chain thiscall(pares) ret 4 (virtual 0xb0): as virtuais 0xa4 da
#   Primeira, Segunda, Terceira e da fase 0 da Terceira; league_promote_and_relegate 0x689c80 (thiscall, 6 args: cima,
#   baixo, 1, 1, -1, -1) tres vezes. Prologo mov eax, [0xadadfc] (5 bytes).
# - 0x41ef70 bel_first_split_third_division_groups thiscall() ret 0 (ecx nao lido): ate 32 clubes da Terceira numa lista
#   na pilha, qsort 0x945333 com 0x53fe50 (por longitude), +0x60 de cada clube = o registro da Terceira A (os 16
#   primeiros) ou B. Prologo mov eax, [0x9cf5a4] (5 bytes).
# Defeitos do original: o split percorre sempre 32 vagas da lista da pilha (com menos de 32 clubes escreve por ponteiros
# velhos; a copia para nos clubes achados); o move le a tabela da fase 0 com a contagem da Terceira.
# Comparadores: os movimentos e a cadeia so a original (movem clubes); os grupos lado a lado no +0x60 dos clubes.
# Depende do cx08_6 (comp_record_by_key): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_first_move_third_division_clubs', 0x41ec90, [0x51, 0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('bel_first_process_promotion_relegation_chain', 0x41ee80, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('bel_first_split_third_division_groups', 0x41ef70, [0xa1, 0xa4, 0xf5, 0x9c, 0x00])]
DOCS = ('o acesso e o rebaixamento na Bélgica e os grupos da Terceira Divisão', 'the Belgian promotion and relegation and the Third Division groups',
        [('bel_first_move_third_division_clubs', 'bel_first_move_third_division_clubs', '0x41ec90', 'bel_first.cpp', 'thiscall, ret 0',
          'os movimentos da Terceira Divisão belga: os vencedores da repescagem entram nela; os rebaixados do grupo A descem para a Quarta A e os do grupo B para a Quarta B; os quatro clubes guardados entram na Terceira',
          'the Belgian Third Division moves: the play-off winners come into it; the relegated clubs of group A go down to Fourth Division A and those of group B to Fourth Division B; the four stored clubs come into the Third Division'),
         ('bel_first_process_promotion_relegation_chain', 'BelFirst::process_promotion_relegation_chain', '0x41ee80', 'bel_first.cpp', 'thiscall, ret 4',
          'a cadeia de acesso e rebaixamento belga: as posições finais das três divisões ficam gravadas; depois sobe e desce entre a Primeira e a Segunda e entre a Segunda e cada grupo da Terceira',
          'the Belgian promotion and relegation chain: the final positions of the three divisions are recorded; then promotion and relegation between the First and the Second and between the Second and each Third Division group'),
         ('bel_first_split_third_division_groups', 'bel_first_split_third_division_groups', '0x41ef70', 'bel_first.cpp', 'thiscall, ret 0',
          'os 32 clubes da Terceira Divisão belga são divididos pela longitude: os 16 primeiros (oeste) no grupo A e o resto no grupo B. Defeito do original: com menos de 32 clubes ele escreve por ponteiros velhos da pilha',
          'the 32 Belgian Third Division clubs are split by longitude: the first 16 (west) to group A and the rest to group B. Defect of the original: with fewer than 32 clubs it writes through stale stack pointers')])
CAMPOS = [('terceira divisão belga +0xee..+0xfa os quatro clubes que entram (ids)', 'Belgian Third Division +0xee..+0xfa the four incoming clubs (ids)')]
INTERESSA = []
