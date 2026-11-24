# A equipe da semana de uma competicao (award/nation_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa
# team_award_obj e award_base (sx05b_5), team_award_clear_selection e team_award_add_candidate (sx05_6),
# team_award_set_dates (sx05b_2) e MATCH_TEAM_BLOCK_DESTRUCTOR (club_screens.rs): integrar depois deles e ANTES do
# sx04c_23 (os comparadores das nacoes leem TEAM_OF_THE_WEEK_BUILDS daqui).
# RODA SO PELAS TELAS: chamada pela virtual update_team_of_the_week (+0x10) de cada nacao, que so a tela dos premios
# chama (0 chamadas na cobertura do mundo ate 2002-08-01). Item TELAS. Caminho para o Lucas, jogando comparando: com um
# jogo em andamento ha algumas semanas, abrir os premios da Inglaterra (ou de outra nacao carregada) e escolher a
# equipe da semana de uma divisao; repetir em outra divisao e numa semana sem jogo.
# Convencoes conferidas na desmontagem (o quadro de 0x101c bytes por _chkstk, os enderecos de pilha normalizados por
# script e conferidos a mao):
# - 0x769c70 thiscall(id da competicao, premio) ret 8, this sem uso; SEH (estados 1..0xa so do desenrolar);
# - inicio = Date::sub_days 0x54c8e0(data atual, saida, 100); semana = copia de 0xae2c90; tres Date::Date 0x54a110;
#   LLIST::LLIST 0x68d020; o registro da partida (0x11f + 2 x 0x625 bytes) com os dois blocos pelo vector
#   constructor iterator 0x9463af (0x453c30, 0x453c10); inicio = a data de atualizacao do premio (+0x1d) se for
#   maior (ano word sem sinal, dia word com sinal); Date::get_nearest_weekday 0x54a2d0(0, a propria semana);
# - enquanto inicio < semana: Date::sub_days(semana, 3) e FIXTURE_MANAGER::find_fixtures 0x5acbe0 (dwords das datas
#   em +0 e +2, -1, lista, competicao, -2, -1) -> word; com 2 ou mais jogos (`cmp ax, 1; jbe`), cada jogo: o registro
#   (+8; -1 ou Match_Stats::read_match_record 0x729600 falhando: a contagem volta a 0 e a semana anterior); a data do
#   1o jogo (Date::Date 0x549ef0(dia +0x2a, ano +0x28): so as words) e a do ultimo de outro dia; os 20 jogadores de
#   cada lado (reserva so com +0x32 != 0xff; id e jogador != -1; id <= [0xae2364] com sinal); o vetor cresce de 200
#   (realloc 0x945667 / malloc 0x944e46; nulo: caixa linha 0x7a8 e a limpeza); mais de 20: para; senao LLIST::clear
#   0x68d080 e Date::sub_weeks 0x54cab0(1);
# - com candidatos: qsort 0x945333(vetor, n, 0xb, 0x76a460), TEAM_AWARD::clear_selection 0x8afd90, reservas =
#   [comp+0x49] (3 sem competicao), add_candidate 0x8afa40 (pessoa, clube, nota em float por fild, posicao word) de
#   todos, set_dates 0x8aff00(primeiro dia, ultimo dia, semana), free 0x9452ca; sempre o vector destructor 0x9462bb e
#   ~LLIST 0x68d040.
# Defeitos do original: semana com um jogo so e ignorada; id igual a contagem de pessoas entra (jg); o vetor alocado
# antes de um jogo nao jogado zerar a contagem vaza quando nada mais entra; sem memoria, o vetor antigo vaza.
# Comparador: lado a lado sobre os 0x25d bytes do premio (selecao, reservas, datas); conta as montagens para o sx04c_23.
TELAS = True
SAVE = False
ROTINAS = [('nation_awards_build_team_of_the_week', 0x769c70, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a equipe da semana de uma competição',
        "a competition's team of the week",
        [('nation_awards_build_team_of_the_week', 'nation_awards_build_team_of_the_week', '0x769c70', 'award/nation_awards.cpp', 'thiscall, ret 8',
          'volta semana a semana desde hoje enquanto a semana for posterior a 100 dias atrás e à última atualização do prêmio: com dois ou mais jogos da competição na semana, junta os jogadores que jogaram (titulares e reservas que entraram) com o clube, a nota e a posição; um jogo ainda não jogado zera a lista; para com mais de 20; depois ordena pela nota e oferece todos ao prêmio, com as reservas da competição e as datas do período',
          'goes back week by week from today while the week is later than 100 days ago and the award\'s last update: with two or more of the competition\'s matches in the week, gathers the players who played (starters and substitutes who came on) with club, grade and position; a match not played yet empties the list; stops with more than 20; then sorts by grade and offers them all to the award, with the competition\'s substitutes and the period dates')])
CAMPOS = []
INTERESSA = []
