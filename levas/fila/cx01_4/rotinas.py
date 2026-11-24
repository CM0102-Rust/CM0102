# Um clube ainda na disputa, a primeira data de uma competicao e os jogos extras de uma rodada de copa (comp.cpp,
# Cup.cpp; leitor cx01, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS).
# - 0x48f150 Comp::is_club_still_in_contention thiscall(clube) ret 4 -> 0/1: chamada pela escolha do time e pelas
#   substituicoes (MATCH_MAN), pelo MATCH_PLAYER::setup, pelo motor (comentario) e pela diretoria depois da partida.
#   Dentro: comp_is_reserve_or_minor_comp (mov ecx, [0xb74434]; push comp) ret 4; Friendly::is_friendly_competition
#   (mov ecx, [0xae2c74]) ret 4; Comp_Stats::find_club_entry (mov ecx, [+8]; push clube) ret 4.
# - 0x48f340 Comp::get_first_schedule_date thiscall(saida) ret 4 -> saida: chamada pela atualizacao diaria do pessoal
#   (game.cpp) e pela confianca semanal da diretoria. Dentro: Date::Date 0x54a110 thiscall(); o slot 0x3c
#   (create_schedule: push &liga; push &extra; push &rodadas; push -1) ret 0x10; Date::Date 0x549ef0 thiscall(dia, ano)
#   ret 8; Date::operator= thiscall(rascunho, fonte) ret 8; free cdecl.
# - 0x522c70 Cup::label_round_extra_fixtures thiscall(rodada, mandante, visitante) ret 0xc: chamada pela atualizacao
#   diaria do FIXTURE_MANAGER (a lista dos confrontos de ida e volta). Dentro: Date::sub_weeks thiscall(rascunho, 2)
#   ret 8; FIXTURE_MANAGER::find_next_fixture thiscall(&ano, &dia, &fatia, 1, comp, rodada, mandante, visitante, -1, -1,
#   -1) ret 0x2c (a fatia e o proprio dword do argumento da rodada, zerado: a copia usa um local); Date::add_days
#   thiscall(rascunho, 1) ret 8.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: a disputa pelo valor; a primeira data lado a lado (a original na saida, a copia num local; numa copa so
# a original: pedir a tabela de rodadas de novo abre a caixa do Cup.cpp); os jogos extras pelo plano (a copia acha os
# dois sem gravar, a original grava; os nomes conferidos).
SAVE = False
ROTINAS = [('comp_is_club_still_in_contention', 0x48f150, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('comp_get_first_schedule_date', 0x48f340, [0x6a, 0xff, 0x68, 0x08, 0x61, 0x95, 0x00]),
           ('cup_label_round_extra_fixtures', 0x522c70, [0x6a, 0xff, 0x68, 0x18, 0x78, 0x95, 0x00])]
DOCS = ('um clube ainda na disputa, a primeira data de uma competição e os jogos extras de uma rodada de copa',
        "a club still in contention, a competition's first date and a cup round's extra fixtures",
        [('comp_is_club_still_in_contention', 'Comp::is_club_still_in_contention', '0x48f150', 'comp.cpp', 'thiscall, ret 4',
          'diz se o clube ainda disputa algo na competição: nunca numa competição reserva, menor ou amistosa; uma etapa diz que sim; com a posição já decidida nas estatísticas, não; senão procura nas fases, da atual para trás: numa copa, estar entre os times é disputar; numa liga, com a situação decidida (0xfe, 7, 6 ou 5) não, senão disputa enquanto a melhor e a pior posição possíveis diferem; não achado, sim',
          'tells whether the club still contends for something in the competition: never in a reserve, minor or friendly competition; a stage says yes; with the place already decided in the statistics, no; otherwise searches the stages, from the current one back: in a cup, being among the teams is contending; in a league, with the status decided (0xfe, 7, 6 or 5) no, otherwise it contends while the best and worst possible positions differ; not found, yes'),
         ('comp_get_first_schedule_date', 'Comp::get_first_schedule_date', '0x48f340', 'comp.cpp', 'thiscall, ret 4',
          'a data da primeira entrada do calendário da competição (o calendário inteiro, pedido e liberado), com o ano relativo ao da competição; sem calendário, a data padrão',
          "the date of the competition's first schedule entry (the whole schedule, asked for and freed), with the year relative to the competition's; without a schedule, the default date"),
         ('cup_label_round_extra_fixtures', 'Cup::label_round_extra_fixtures', '0x522c70', 'Cup.cpp', 'thiscall, ret 0xc',
          'a partir de duas semanas antes da data da rodada, o próximo jogo desta competição e rodada entre os dois clubes recebe o código da rodada + 3 e, a partir do dia seguinte a ele, o próximo recebe o código + 4',
          "from two weeks before the round's date, the next fixture of this competition and round between the two clubs gets the round's code + 3 and, from the day after it, the next one gets the code + 4")])
CAMPOS = []
INTERESSA = []
