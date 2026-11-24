# Os grupos e os resultados das eliminatorias da Euro 2000 numa partida que comeca em 1999 (euro_champ_qual.cpp; leitor
# cx06, rodada 5, 24/11/2026). Simulacao com as 8 ligas, como os outros itens das eliminatorias (cx06_3..5), mas as
# duas rotinas NAO RODAM com o banco de 2001: o construtor (0x57ce10, cx06_3) so as chama quando o ano da temporada e
# maior que a edicao, o que so acontece numa partida que comeca em 1999 (edicao 1998). Entram para fechar o arquivo (o
# exe inteiro); nenhuma mutacao pode ser acusada na cadeia.
# - 0x580bd0 EuroChampQual::create_groups_and_results thiscall() ret 0 (8992 bytes, quase tudo dados): os 9 grupos da
#   Euro 2000 por chave de nacao (lidas no inicio; -1 vazio), +0xf6 = 2, +0xee = malloc(0xcc) (nulo: caixa 0x7f9), +0xf2
#   de 0 em diante; cada selecao (database_get_national_team 0x53b3d0, cdecl, duas vezes) na lista local e nos inscritos;
#   o grupo 1 na propria tabela (+0x3e, malloc n x 0x3b, nulo: caixa 0x815; League::init_table_row 0x687430), os outros
#   em create_group_stage 0x57ed10 (g - 1, a lista, n); create_runners_up_stage 0x57f920, a virtual 0x5c, free(+0xee) e
#   zera (+0xf2 fica), +0x45 = 1; depois os 220 resultados de 1998 e 1999 por add_historic_fixture (direto). Os dados
#   saem da desmontagem (EURO_2000_GROUPS, EURO_2000_RESULTS: 71 jogos de 1998 e 149 de 1999).
# - 0x582ef0 EuroChampQual::add_historic_fixture thiscall(fase, dia, mes, ano, nacao da casa, gols, nacao de fora, gols)
#   ret 0x20: sem vetor de fases, fase depois de +0x30 ou fase vazia: caixa 0x936; ano fora de 1998..1999 ou fase fora de
#   -1..7: caixa 0x93d; o registro de partida local (0x4f bytes) com os campos da copa, Date::Date 0x549f70 (dia, mes,
#   ano, -1), a fatia de horario pelo dia da semana (Date::get_weekday 0x54a3f0: 5 ou 6 -> 1, senao 2), as selecoes,
#   Stadium::set_fixture_venue 0x85c260 sobre [0xdd7ef4] (0, a partida, -1); data depois do inicio do jogo [0xdd6ec8]: os
#   gols 0xff; FIXTURE_MANAGER::add_fixture 0x5aa680 (push 1); jogada: a virtual 4 (add_result, push 0) e
#   League::sort_table 0x6827d0 do grupo; sempre update_runners_up_table 0x57fa10. Prologo mov eax, fs:[0] (6 bytes).
# Defeito do original: o registro de partida e local e nao e zerado; o arbitro (+0x2e), o campo neutro (+0x3c), as marcas
# (+0x4e) e os 4 bits altos de +0x4d (se o set_fixture_venue nao os escrever) ficam com o que estava na pilha; a copia
# comeca de zero. Vai para o DEFEITOS.
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (alocam os grupos e entram partidas), com o plano (os 9 grupos e +0x45 1).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('euro_champ_qual_create_groups_and_results', 0x580bd0, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00]),
           ('euro_champ_qual_add_historic_fixture', 0x582ef0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os grupos e os resultados das eliminatórias da Euro 2000 numa partida que começa em 1999',
        'the Euro 2000 qualifying groups and results in a game that starts in 1999',
        [('euro_champ_qual_create_groups_and_results', 'EuroChampQual::create_groups_and_results', '0x580bd0', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'numa partida que começa em 1999, monta os 9 grupos das eliminatórias da Euro 2000 (com seis seleções nos grupos 2, 4, 7 e 9) e entra os 220 jogos de 1998 e 1999, com os placares dos que aconteceram antes do início do jogo',
          'in a game that starts in 1999, builds the 9 Euro 2000 qualifying groups (with six nations in groups 2, 4, 7 and 9) and enters the 220 games of 1998 and 1999, with the scores of those played before the game starts'),
         ('euro_champ_qual_add_historic_fixture', 'EuroChampQual::add_historic_fixture', '0x582ef0', 'euro_champ_qual.cpp', 'thiscall, ret 0x20',
          'entra um jogo passado das eliminatórias: a partida no calendário (sexta e sábado no horário de fim de semana) e, se foi jogada antes do início do jogo, o placar na tabela do grupo; a tabela dos segundos colocados é refeita',
          'enters a past qualifying game: the fixture in the calendar (Friday and Saturday in the weekend slot) and, when it was played before the game starts, its score in the group table; the runners-up table is rebuilt')])
CAMPOS = []
INTERESSA = []
