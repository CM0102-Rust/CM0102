# O progresso dos clubes da Premier Division norueguesa e os dois passos da virada da Segunda Divisao (nor_prm.cpp;
# leitor cx12, rodada 5, 24/11/2026). Simulacao com a Noruega (LIGAS e ATE dos itens mwe): o progresso a cada
# atualizacao de status; os dois passos uma vez na virada do fim de 2001 (o NorPrm::start_new_season ja portado os chama
# por endereco, call_on_league, thiscall com ecx = a Premier, ret 0).
# - 0x792d40 NorPrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: o corpo do NirPrm (cx12_10)
#   com a vaga 12 -> 11 (melhor) e 14 (pior): premier_play_off_progress. Prologo mov eax, [esp+4]; mov ecx, [ecx+8].
# - 0x792960 nor_prm_swap_first_and_second_division_clubs: a lista de G_CLUB_COUNT ponteiros (malloc; sem memoria a
#   caixa da linha 0x24b), os clubes com divisao e nacao, a divisao num dos 8 grupos da Segunda ([0x9cf6c4..0x9cf6e0]) e
#   a nacao Noruega ([0x9cf408]); com algum, comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (cdecl: lista, n, 4), os 4
#   primeiros sobem (League::move_club_to_division 0x6830b0, thiscall(clube, registro da Primeira, 1), ret 0xc) e as
#   linhas da Primeira com situacao 3 descem ao Grupo 1 (League::get_table_row 0x6826d0 e
#   League::relegate_club_to_division 0x6831a0, as mesmas convencoes); free. DEFEITO: com menos de 4 candidatos, le
#   entradas nunca escritas da lista. Prologo sub esp, 0x208 (6 bytes).
# - 0x792b40 nor_prm_regroup_second_division_clubs: ate 96 clubes cuja divisao e um dos 8 registros dos grupos (o
#   indice uma palavra, a contagem um byte com sinal), qsort 0x945333 pela latitude da cidade (0x53fd40, o comparador do
#   reagrupamento da C1, mwf_16) e os grupos em blocos de 12, sem noticia. Prologo sub esp, 0x19c (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o progresso lado a lado; a troca so a original (embaralha); o
# reagrupamento so a original contra o plano (a mesma lista ordenada antes).
# Depende do cx12_10 (premier_play_off_progress): integrar depois dele.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('nor_prm_set_club_progress', 0x792d40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('nor_prm_swap_first_and_second_division_clubs', 0x792960, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('nor_prm_regroup_second_division_clubs', 0x792b40, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00])]
DOCS = ('o progresso dos clubes da Premier norueguesa e os passos da virada da Segunda Divisão',
        'the Norwegian Premier Division club progress and the Second Division turnover steps',
        [('nor_prm_set_club_progress', 'NorPrm::set_club_progress', '0x792d40', 'nor_prm.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação na Premier norueguesa: a vaga da repescagem (12) vira 11 como a melhor e 14 como a pior; a atual como vem',
          'the place forecast in the Norwegian Premier: the play-off place (12) becomes 11 as the best and 14 as the worst; the current one as it comes'),
         ('nor_prm_swap_first_and_second_division_clubs', 'nor_prm_swap_first_and_second_division_clubs', '0x792960', 'nor_prm.cpp', 'thiscall, ret 0',
          'regra: na virada, quatro clubes noruegueses da Segunda Divisão, sorteados, sobem à Primeira, e os rebaixados da Primeira vão ao Grupo 1 da Segunda',
          'rule: at the turnover, four Norwegian Second Division clubs, drawn, go up to the First Division, and the First Division\'s relegated clubs go to Second Division Group 1'),
         ('nor_prm_regroup_second_division_clubs', 'nor_prm_regroup_second_division_clubs', '0x792b40', 'nor_prm.cpp', 'thiscall, ret 0',
          'regra: na virada, os clubes da Segunda Divisão norueguesa (até 96) são ordenados do norte para o sul [inferência: pela latitude da cidade] e repartidos pelos oito grupos em blocos de doze',
          'rule: at the turnover, the Norwegian Second Division clubs (up to 96) are sorted from north to south [inference: by the city latitude] and split into the eight groups in blocks of twelve')])
CAMPOS = []
INTERESSA = []
