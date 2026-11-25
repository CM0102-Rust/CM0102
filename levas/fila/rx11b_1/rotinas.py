# Os jogos da competicao reserva (record_utils.cpp; leitor rx11b, rodada 6, 25/11/2026). Simulacao, carreira ENG (sem
# LIGAS): a marcacao dos jogos reservas roda todo dia (FIXTURE_MANAGER 0x5aaf00, ja Rust); o jogo B ou sub-21 roda no
# add_fixture (0x5aa680, Rust) de cada jogo entre selecoes (amistosos e eliminatorias da carreira).
# Convencoes conferidas na desmontagem:
# - 0x7e6380 ReserveComp::schedule_reserve_fixtures thiscall(data) ret 4 (ecx = [0xb74434] no 0x5aaf00): tres LLIST
#   locais (0x68d020/0x68d040; duas nunca usadas, a terceira recebe a copia de 14 bytes do cabecalho da lista do dia e o
#   cursor = cabeca, `mov eax,[esp+0x12]; mov [esp+0x1a],eax`); 0x54a110 Date; 0x54a3f0 get_weekday (ax = 4: 0x54c5b0
#   add_days(data, tmp, 2), senao 0x54c8e0 sub_days) e 0x417c10 Date::operator=(this, saida, origem) ret 8; 35 entradas
#   de 0x48 bytes em 0xb63d60; 0x5ae6a0 thiscall([0xae2a58], data, fatia (dword com lixo em cima), grupo +0x14) ret 0xc;
#   0x4b0700 cdecl(id da competicao); 0x7e6ba0 thiscall(this, jogo, 1) ret 8; copia de 0x4f bytes (rep movsd x 0x13 +
#   movsw + movsb); 0x5ae870 thiscall([0xae2a58], estadio, dia (dword), ano (dword), -1) ret 0x10 duas vezes, com dwords
#   diferentes (o 1o com a data local, o 2o com +0x2a/+0x28 da copia); 0x5aa680 thiscall([0xae2a58], &copia, 1) ret 8.
#   A Belgica le o dword +0x4d..+0x50 (2 bytes fora da copia) e grava so a palavra.
# - 0x7e6720 ReserveComp::add_b_or_under21_fixture thiscall(jogo) ret 4 (this nao lido; ecx = [0xb74434] no 0x5aa680):
#   0x549ef0 Date(dia, ano) ret 8 e 0x54c8e0 sub_days(.., 1); 0x5c6710 thiscall([0xae2c74], competicao) ret 4; 0x549f70
#   Date(dia, mes, ano, -1) ret 0x10 (20/5/1998, 1/2/2000, 1/8/2000); 0x542740 cdecl(clube); 0x5ac7a0 thiscall([0xae2a58],
#   jogo) ret 4; 0x53b4f0 cdecl(nacao), sem checar nulo; 0x85c260 thiscall([0xdd7ef4], 0x20, &copia, -1) ret 0xc;
#   0x5aa680 como acima.
# Comparadores: so a original (os jogos entram no calendario). A marcacao: o plano da copia (os pares de clubes que ela
# mandaria ao add_fixture) e, depois da original, todo jogo reserva novo na data reserva tem de ser um deles (o
# add_fixture pode recusar ou mudar a data de um: so a inclusao e conferida). O B/sub-21: o tipo planejado e as regras
# do jogo principal guardado (so os bits 0x300 mudam, para o planejado ou nenhum).
SAVE = False
ROTINAS = [('schedule_reserve_fixtures', 0x7e6380, [0x6a, 0xff, 0x68, 0x8c, 0x07, 0x96, 0x00]),
           ('add_b_or_under21_fixture', 0x7e6720, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os jogos da competição reserva: os jogos reservas de uma data e o jogo B ou sub-21 de um jogo entre seleções',
        "the reserve competition's fixtures: the reserve matches of a date and the B or Under 21 match of a national team match",
        [('schedule_reserve_fixtures', 'ReserveComp::schedule_reserve_fixtures', '0x7e6380', 'record_utils.cpp', 'thiscall, ret 4',
          'regra: para os jogos de liga de uma data (35 nações-chave com liga escolhida, menos a França; as três fatias do dia), quando os dois clubes podem escalar os reservas, marca um jogo reserva dois dias depois (se a data é o dia da semana 4) ou dois dias antes; o jogo reserva é no campo do outro clube (fora da Turquia), no estádio reserva dele se livre, senão no estádio dele se livre, senão não há jogo; na Bélgica, 4 suplentes e 4 substituições',
          'rule: for the league fixtures of a date (35 key nations with a chosen league, France excepted; the three slices of the day), when both clubs can field reserve sides, adds a reserve match two days later (when the date is weekday 4) or two days earlier; the reserve match is at the other club\'s ground (outside Turkey), at its reserve stadium when free, else at its stadium when free, else there is none; in Belgium, 4 substitutes and 4 substitutions'),
         ('add_b_or_under21_fixture', 'ReserveComp::add_b_or_under21_fixture', '0x7e6720', 'record_utils.cpp', 'thiscall, ret 4',
          'regra: na véspera de um jogo entre duas seleções de reputação 2500 ou mais, um jogo B (amistoso, ano par, até 20 de maio, nações com 70 pessoas e os dois com time reserva; as mesmas seleções) ou senão um sub-21 (sem prefixo ou sem nome de rodada; nos anos divisíveis por 4, fora de 2/fev a 31/jul; as seleções sub-21 das duas nações); o jogo principal ganha a marca do jogo ligado; no estádio reserva livre do mandante ou num estádio do país; sem estádio, a marca sai e não há jogo',
          'rule: the day before a match between two national teams of reputation 2500 or more, a B match (a friendly, an even year, up to 20 May, nations with 70 people and both with a reserve squad; the same teams) or else an Under 21 one (no round prefix or no round name; in years divisible by 4, outside 2 February to 31 July; both nations\' Under 21 teams); the senior fixture gets the linked match mark; at the home team\'s free reserve stadium or a ground in its country; with no ground, the mark is cleared and there is no match')])
CAMPOS = []
INTERESSA = []
