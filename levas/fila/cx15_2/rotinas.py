# Os pontos da temporada de uma nacao, as vagas europeias de uma nacao e os prazos de inscricao europeus (uefa_seeding.cpp;
# leitor cx15, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS): os pontos a cada resultado das copas europeias (Liga dos
# Campeoes, Copa da UEFA, Intertoto), as vagas no fim de cada liga europeia e na previsao, os prazos todo dia.
# - 0x9058b0 UefaSeeding::add_nation_points thiscall(nacao, pontos) ret 8 -> o endereco da palavra: database_get_national_team
#   0x53b3d0 (push; add esp -> cdecl) da nacao, o byte com sinal +0x1df da selecao (o lugar no ranking) indexa as
#   entradas de 0x12 bytes; +0xf += pontos (byte com sinal, movsx dx). Prologo mov eax, [esp+4]; push esi (5 bytes).
# - 0x905c60 UefaSeeding::get_nation_places thiscall(nacao, &cl, &uefa) ret 0xc: *uefa = 1 e *cl = 1 (nessa ordem);
#   cada entrada (contagem +0x438 com sinal, relida) com a nacao (+4) igual escreve +0xc e +0xd (sem parar: vale a
#   ultima). O eax de saida nao e lido. Prologo push ebp; mov ebp, [esp+0xc] (5 bytes).
# - 0x9058e0 uefa_check_registration_deadlines thiscall() ret 0 (o ecx nao e lido; o update_season o chama com ecx):
#   Date::Date 0x549f70 (thiscall(dia, mes, ano, semana), ret 0x10) de (4, 7, ano, 4), (1, 8, ano, 4) e (2, 1, ano, 1),
#   os tres antes do teste; hoje (0xae2c90) igual pelo ano e pelo dia: SQUAD_MANAGER::process_euro_registration
#   0x85a9b0 (ecx = [0xdd7ef0]; push passo -> thiscall, ret 4) com 1, 2 ou 3; o terceiro so fora de 2001. O ano vai em
#   ax com o resto do registrador nos bytes altos (Date::Date le uma palavra). O quadro SEH so cobre as datas locais.
#   Prologo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os pontos lado a lado na tabela do ranking e no retorno; as vagas
# lado a lado (a original nos bytes do chamador, a copia em locais); os prazos so a original (a inscricao manda noticias).
SAVE = False
ROTINAS = [('uefa_seeding_add_nation_points', 0x9058b0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('uefa_seeding_get_nation_places', 0x905c60, [0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('uefa_check_registration_deadlines', 0x9058e0, [0x6a, 0xff, 0x68, 0x68, 0x52, 0x96, 0x00])]
DOCS = ('o ranking da UEFA: os pontos da temporada de uma nação, as vagas europeias de uma nação e os prazos de inscrição',
        'the UEFA seeding: a nation\'s season points, its European places and the registration deadlines',
        [('uefa_seeding_add_nation_points', 'UefaSeeding::add_nation_points', '0x9058b0', 'uefa_seeding.cpp', 'thiscall, ret 8',
          'soma os pontos de um resultado europeu aos pontos da temporada da nação, na entrada do ranking indicada pela posição da seleção dela',
          'adds the points of a European result to the nation\'s season points, in the seeding entry given by its national team\'s rank'),
         ('uefa_seeding_get_nation_places', 'UefaSeeding::get_nation_places', '0x905c60', 'uefa_seeding.cpp', 'thiscall, ret 0xc',
          'dá as vagas de uma nação na Liga dos Campeões e na Copa da UEFA; fora do ranking, uma de cada',
          'gives a nation\'s Champions League and UEFA Cup places; outside the seeding, one of each'),
         ('uefa_check_registration_deadlines', 'uefa_check_registration_deadlines', '0x9058e0', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'nos prazos de inscrição europeus (4 de julho, 1 de agosto e 2 de janeiro, este fora de 2001), faz a inscrição dos jogadores daquela fase',
          'on the European registration deadlines (4 July, 1 August and 2 January, the last one outside 2001), runs the player registration of that phase')])
CAMPOS = []
INTERESSA = []
