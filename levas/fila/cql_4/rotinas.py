# O desempate ("spareggio") da Série A e da Série B italianas no fim da temporada (Agente B da frente das competições,
# 04/11/2026). Rodam na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt: 0x642880 e 0x647050 2064
# vezes, o slot +0x14 de cada dia; 0x641dc0 5 vezes, o slot +0x44; a 0x6471a0 só roda com empate na fronteira):
# - 0x642880 (ItaSerA::update_for_date, 486 bytes) e 0x647050 (ItaSerB::update_for_date, 335 bytes): o dia de liga e,
#   no fim da temporada, a procura do empate em pontos entre faixas diferentes;
# - 0x6471a0 (ita_league_sort_clubs_level_on_points, 132 bytes): o bloco dos empatados pelo confronto direto;
# - 0x641dc0 (ItaSerA::update_staff_records_for_status, 498 bytes): o resultado do jogo de desempate na tabela.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x6847c0 (League::update_for_date): push completo; push hoje; mov ecx, esi -> thiscall, ret 8;
# - 0x905c60 (UefaSeeding::get_nation_places): push &uefa; push &cl; push nação; mov ecx, [0xde1f58] -> thiscall, ret 0xc;
# - 0x6471a0: push pontos (edx/edi: a palavra; os bits de cima são lixo de outra conta ou do `completo`, a 0x6471a0 só
#   lê a palavra: mov dx, word ptr [esp + 0x10]); mov ecx, esi -> thiscall, ret 4;
# - 0x944e46 (malloc): push 4; add esp, 4 -> cdecl;
# - as virtuais +0x28, +0x94 (mov ecx, esi; call [vt + ..] -> thiscall, ret 0) e +0x10 (push 0; push 0 -> thiscall,
#   ret 8);
# - 0x682b30 (League::build_h2h_table): push [+0xb1]; mov ecx, esi -> thiscall, ret 4;
# - 0x945333 (qsort): 4 push; add esp, 0x10 -> cdecl, com a comparação 0x683410;
# - 0x686940 (League::update_staff_records_for_status): 6 push (a6, jogo, a4, -1, situação, clube); mov ecx, ebp ->
#   thiscall, ret 0x18;
# - 0x6827d0 (League::sort_table): push 0; mov ecx, ebp -> thiscall, ret 4.
# Comparadores: os dois dias, só a original (o dia de liga e a fase nova não rodam duas vezes), conferindo que a fase
# do desempate só nasce com empate na fronteira e que o fim da temporada não fica pendente; a ordem dos empatados, lado
# a lado na tabela inteira; o resultado do desempate, só a original (base e virtuais), com a devolução −1 conferida nas
# outras situações.
# Determinismo: as faixas vêm de constantes e da 0x905c60 (que grava as duas contas antes de ler); a original monta as
# faixas num registro local de 18/20 bytes e só leria além dele com mais clubes do que a Série A/B tem; nada de pilha,
# heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('desempate_da_serie_a', 0x642880, [0x83, 0xec, 0x1c, 0x8b, 0x44, 0x24, 0x20]),
           ('desempate_da_serie_b', 0x647050, [0x83, 0xec, 0x18, 0xb0, 0x02]),
           ('ordem_dos_empatados_na_italia', 0x6471a0, [0x53, 0x56, 0x8b, 0xf1, 0x57]),
           ('resultado_do_desempate_da_serie_a', 0x641dc0, [0x8a, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x3c])]
DOCS = ('o desempate da Série A e da Série B italianas no fim da temporada (o "spareggio")',
 'the end-of-season tie-break of the Italian Serie A and Serie B (the "spareggio")',
 [('desempate_da_serie_a', 'ItaSerA::update_for_date', '0x642880', 'ita_ser_a.cpp', 'thiscall, ret 8',
   'o dia da Série A; no fim da temporada, dois clubes empatados em pontos em faixas diferentes (o título; as vagas da '
   'Liga dos Campeões da Itália; as da Copa da UEFA; as duas seguintes [inferência: a Intertoto]; os rebaixados, os '
   'últimos +0xc1 de 18) não se separam pelo saldo: o primeiro par achado de cima para baixo é ordenado pelo confronto '
   'direto e vai a um jogo extra (a fase seguinte); sem empate na fronteira, a temporada fecha',
   'the Serie A day; at season end, two clubs level on points in different bands (the title; Italy\'s Champions '
   'League places; the UEFA Cup places; the next two [inference: the Intertoto]; the relegated, the last +0xc1 of 18) '
   'are not split by goal difference: the first such pair from the top is ordered by head-to-head and goes to an extra '
   'match (the next stage); with no tie on a boundary, the season closes'),
  ('desempate_da_serie_b', 'ItaSerB::update_for_date', '0x647050', 'ita_ser_b.cpp', 'thiscall, ret 8',
   'o dia da Série B; as faixas fixas são o campeão, mais 3 vagas de acesso (4 sobem) e os 4 rebaixados de 20; cada par '
   'empatado numa fronteira é ordenado pelo confronto direto e, havendo algum, o jogo de desempate',
   'the Serie B day; the fixed bands are the champion, 3 more promotion places (4 go up) and the 4 relegated of 20; '
   'each tied pair on a boundary is ordered by head-to-head and, if any, the tie-break match'),
  ('ordem_dos_empatados_na_italia', 'ita_league_sort_clubs_level_on_points', '0x6471a0', 'ita_ser_b.cpp',
   'thiscall, ret 4',
   'os clubes com os mesmos pontos são ordenados pelo confronto direto entre eles (a minitabela 0x682b30) e as '
   'posições da tabela são renumeradas',
   'clubs level on points are ordered by the head-to-head among them (the mini-table 0x682b30) and the table positions '
   'are renumbered'),
  ('resultado_do_desempate_da_serie_a', 'ItaSerA::update_staff_records_for_status', '0x641dc0', 'ita_ser_a.cpp',
   'thiscall, ret 0x18',
   'na liga, os registros da base; no jogo de desempate, o vencedor (situação 1 [inferência]) fica à frente do outro na '
   'tabela (trocam de linha se estava abaixo) e, sem play-off pendente, a temporada fecha',
   'in the league, the base records; in the tie-break match, the winner (status 1 [inference]) is placed above the '
   'other in the table (rows swap if it was below) and, with no play-off pending, the season closes')])
CAMPOS = [('liga +0xdd fim da temporada, +0xe6 [inferência: a tabela fechada], +0x2c fases, +0xc vetor das fases, +0x4c '
           'marca do histórico; valor e ponteiro; lê e grava (desempate_da_serie_a, desempate_da_serie_b, '
           'resultado_do_desempate_da_serie_a)',
           'league +0xdd season over, +0xe6 [inference: table closed], +0x2c stages, +0xc stage vector, +0x4c history '
           'mark; value and pointer; reads and writes'),
          ('liga +0xc1 rebaixados; valor; lê (desempate_da_serie_a)', 'league +0xc1 relegated; value; reads'),
          ('linha da tabela +0x19 pontos, +4 posição, +0x37 destino, +0 clube; valor; lê e grava (as quatro)',
           'table row +0x19 points, +4 position, +0x37 fate, +0 club; value; reads and writes'),
          ('registro da competição +0x5d nação; ponteiro; lê (desempate_da_serie_a)',
           'competition record +0x5d nation; pointer; reads'),
          ('partida +0x1c e +0x20 os dois clubes; ponteiro; lê (resultado_do_desempate_da_serie_a)',
           'fixture +0x1c and +0x20 the two clubs; pointer; reads')]
INTERESSA = ['CMRTE']
