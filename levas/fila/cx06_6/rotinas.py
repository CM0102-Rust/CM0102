# As selecoes e as posicoes previstas da Copa das Confederacoes (fifa_confed.cpp; leitor cx06, rodada 5, 24/11/2026).
# Simulacao com as 8 ligas e ATE 2003-08-15: o construtor (ja em Rust, 0x58b310) leva a edicao a 2003; o dia
# (FifaConfed::update_for_date 0x58c4b0, ja em Rust, FIFA_CONFED_DAY) sorteia as selecoes (select_teams), monta o grupo
# A (create_group_table, cx06_7) e a fase do grupo B (create_group_stage, cx06_7) na data do sorteio de 2003, e os jogos
# vao de 29/07 a 09/08/2003 (create_schedule ja em Rust). Na cobertura do mundo ate 2002-08-01 as tres tem 0 chamadas:
# sem a data longa o item nao prova nada (o mesmo ATE do cx04_6, a mesma leva).
# - 0x58c6b0 FifaConfed::select_teams thiscall() ret 0 -> 0/1: +0xf2 = 8, +0xee = malloc(0x20) (nulo: caixa 0x35f e 0);
#   0: a selecao da sede (HostCountry::find_hosts 0x5fa730 sobre [0xb5e134], id da competicao, ano, as duas sedes, 1;
#   nenhuma: caixa 0x366 e uma nacao sorteada, utils_random 0x90cfc0 cdecl de [0xae2350]); 1: o ultimo campeao
#   (comp_util_get_latest_placed_club 0x4afd80 cdecl, 0; nenhum: caixa 0x371 e um clube sorteado de [0xae235c] -
#   2 x [0xae2350]); se for a sede, o vice (1; nenhum: caixa 0x37c e um clube sorteado); 2 a 7: pick_continental_champion
#   com Eurocopa [0x9cf7a4], CONCACAF (-1), Copa America [0x9cf7a8], Copa Africana [0x9cf7ac], Oceania [0x9cf8a0] e Asia
#   [0x9cf790]; 1.
# - 0x58ccb0 FifaConfed::pick_continental_champion thiscall(competicao, lista, n) ret 0xc -> clube: -1 e a CONCACAF (antes
#   de 2002 o Canada se livre; senao sorteio de 4 entre EUA, Canada, Mexico e Jamaica ate um livre); outra competicao: o
#   ultimo campeao, se ja estiver na lista o vice, se tambem o terceiro (sem conferir); sem nenhum, a primeira livre de
#   Brasil, Italia, Espanha, Argentina, EUA, Australia, Franca e Alemanha (ou nulo).
# - 0x58ca40 FifaConfed::set_predicted_positions thiscall() ret 0 (virtual 0x5c): sem estatisticas, caixa 0x39c; qsort
#   pela reputacao (0x4b07d0); posicoes 1, 2, 3, 4 e quatro 5 (Comp_Stats::set_club_final_position 0x4a2540);
#   comp_util_shuffle_pointer_array 0x4afb40 (cdecl) com a palavra da contagem.
# Defeito do original: o terceiro colocado de uma competicao continental e escolhido sem conferir se ja esta na lista
# (a mesma selecao pode entrar duas vezes).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: os tres lado a lado a partir do mesmo estado do gerador (a copia sem contar os sorteios): a escolha e o
# gerador; as 8 selecoes (cada lado na sua lista; a da copia e liberada) e o gerador; a lista ordenada e embaralhada e
# o gerador, mais as posicoes pela reputacao.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('fifa_confed_pick_continental_champion', 0x58ccb0, [0x83, 0xec, 0x20, 0xa1, 0x60, 0xf2, 0x9c, 0x00]),
           ('fifa_confed_select_teams', 0x58c6b0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('fifa_confed_set_predicted_positions', 0x58ca40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as seleções e as posições previstas da Copa das Confederações', "the Confederations Cup's teams and predicted positions",
        [('fifa_confed_pick_continental_champion', 'FifaConfed::pick_continental_champion', '0x58ccb0', 'fifa_confed.cpp', 'thiscall, ret 0xc',
          'escolhe a seleção de uma vaga continental da Copa das Confederações que ainda não esteja na lista: da CONCACAF, antes de 2002 o Canadá, senão uma sorteada entre Estados Unidos, Canadá, México e Jamaica; das outras, o último campeão do continente, ou o vice, ou o terceiro (sem conferir), e sem nenhum a primeira livre entre Brasil, Itália, Espanha, Argentina, Estados Unidos, Austrália, França e Alemanha',
          'picks the nation of a Confederations Cup continental place not yet on the list: for CONCACAF, before 2002 Canada, otherwise one drawn among the United States, Canada, Mexico and Jamaica; for the others, the continent\'s latest winner, or the runner-up, or the third (unchecked), and with none the first free of Brazil, Italy, Spain, Argentina, the United States, Australia, France and Germany'),
         ('fifa_confed_select_teams', 'FifaConfed::select_teams', '0x58c6b0', 'fifa_confed.cpp', 'thiscall, ret 0',
          'escolhe as 8 seleções da Copa das Confederações: a sede, o último campeão (o vice quando o campeão é a sede) e os campeões da Europa, da CONCACAF, da América do Sul, da África, da Oceania e da Ásia',
          'selects the 8 Confederations Cup nations: the host, the holders (the runners-up when the holders are the host) and the champions of Europe, CONCACAF, South America, Africa, Oceania and Asia'),
         ('fifa_confed_set_predicted_positions', 'FifaConfed::set_predicted_positions', '0x58ca40', 'fifa_confed.cpp', 'thiscall, ret 0',
          'as posições previstas da Copa das Confederações pela reputação: 1, 2, 3, 4 e quatro quintos; depois a lista é embaralhada',
          'the Confederations Cup\'s predicted positions by reputation: 1, 2, 3, 4 and four fifths; then the list is shuffled')])
CAMPOS = []
INTERESSA = []
