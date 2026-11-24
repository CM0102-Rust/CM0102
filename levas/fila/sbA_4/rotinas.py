# O construtor do ranking da FIFA e o resultado internacional (fifa_rankings.cpp; leitor sbA, rodada 5, 24/11/2026).
# Carreira ENG (sem LIGAS).
# - 0x58cea0 FIFA_RANKINGS::FIFA_RANKINGS thiscall(arquivo do save) ret 4 -> this: roda na criacao (arquivo 0) e na
#   carga. Date::Date 0x54a110 em +5, o mes da data de inicio (Date::get_month 0x54cfb0 de 0xdd6ec8) em +4,
#   Date::operator= 0x417c10 (thiscall(&copia, &inicio)), a carga FIFA_RANKINGS::load_dat 0x58d7a0 (cx04_3, chamada pelo
#   endereco) e, sem ela, create_default_table 0x58dd50. SEH so para o desenrolar.
# - 0x58cf70 FIFA_RANKINGS::record_match_result thiscall(jogo) ret 4: centenas de vezes na carreira (cada jogo
#   internacional terminado, pelo 0x76b600). x87 com tabela de saltos 0x58d444 (5 casos pelo codigo das faixas),
#   constantes f32 0x96a324..0x96a340 e f64 0x968e80 (0,3), 0x968918 (0,2), 0x968e18 (2,5), 0x968908 (1000); o peso
#   do continente (+0xbe, double) quando as duas nacoes sao do mesmo; os gols da prorrogacao (+0x47/+0x48) quando ha.
#   Competicao desconhecida: a caixa de erro (fifa_rankings.cpp 0xde) e nada.
# Convencoes conferidas: get_nation_strength_tier 0x58e190 (thiscall, ret 4, al), comp_active_key_nation_of_fixture
# 0x48dd70 (cdecl(comp, 0), al), national_teams_is_finals_comp 0x76d940 (ecx = [0xb74318]), is_friendly_competition
# 0x5c6710 (ecx = [0xae2c74]), _ftol 0x945640, append_result 0x58dea0 (thiscall(nacao, pontos, jogo), ret 0xc).
# Prologos: push -1; push imm32 (7 bytes); sub esp, 0x220 (6 bytes). Sem relativas. Iguais no cm0102-1080.exe.
# Comparadores: o construtor so a original (aloca ou carrega a tabela), conferindo o mes e a data que deixa; o resultado
# planejado pela copia antes (sem acrescentar), a original roda e a ultima entrada de cada nacao tem de ter os pontos
# planejados (o historico pode ser realocado: nao roda duas vezes).
SAVE = False
ROTINAS = [('new_fifa_rankings', 0x58cea0, [0x6a, 0xff, 0x68, 0x0b, 0x8e, 0x95, 0x00]),
           ('fifa_record_match_result', 0x58cf70, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('o construtor do ranking da FIFA e o resultado internacional',
        "the FIFA ranking's constructor and an international result",
        [('new_fifa_rankings', 'FIFA_RANKINGS::FIFA_RANKINGS', '0x58cea0', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'monta o ranking da FIFA: a data da última atualização e o mês da virada pela data de início do jogo; carrega a tabela do jogo salvo ou monta a tabela padrão',
          'builds the FIFA ranking: the last update date and the rollover month from the game start date; loads the table from the saved game or makes the default one'),
         ('fifa_record_match_result', 'FIFA_RANKINGS::record_match_result', '0x58cf70', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'pontua um jogo internacional terminado para as duas seleções: peso da competição (Copa do Mundo 1,5; finais 1,25 ou 1,1; amistoso 1) vezes o peso do continente quando são do mesmo; base pela diferença das faixas de força e pelo resultado, mais 6% da base por gol (até 5); 0,3 a mais para o visitante jogando no país do mandante; seleção de reputação abaixo de 3500 limita os próprios pontos a 3 e garante 2,5 ao adversário; × 1000 no histórico',
          'scores a finished international for both national sides: competition weight (World Cup 1.5; finals 1.25 or 1.1; friendly 1) times the continent weight when both are in the same one; the base by the strength tier gap and the result, plus 6% of the base per goal (up to 5); 0.3 more for the away side playing in the home nation; a side of reputation under 3500 caps its own points at 3 and guarantees 2.5 to the opponent; × 1000 into the history')])
CAMPOS = [('continente +0xbe (double): o peso do continente no ranking da FIFA [inferencia]; le (fifa_record_match_result)',
           'continent +0xbe (double): the continent weight in the FIFA ranking [inference]; reads (fifa_record_match_result)')]
INTERESSA = []
