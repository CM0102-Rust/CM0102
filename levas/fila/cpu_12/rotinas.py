# A ordem dos clubes da liga e os clubes possíveis de um jogo da copa (league.cpp, Cup.cpp): League::draw_team_order
# (0x686a70, 36 chamadas na cobertura de 17/10: o embaralhamento da tabela antes da grade de jogos de cada liga, pela
# 0x6835c0), Cup::get_fixture_side_teams (0x521790) e Cup::collect_possible_teams (0x5219b0, recursiva), essas duas
# só pelas telas da copa (passageiras). A 0x686a70 guarda um sorteio, semeia o rand da CRT (0x9469fa) com +0x40 +
# [0xdd7120], troca n pares, volta a semente e põe os pares do mesmo estádio, dos estádios ligados e o Celtic e o
# Rangers nas vagas k e clubes / 2 + k; na marca 0x100 de +0xd9 as vagas vêm da tabela virtual (+0xa8, +0xac). Chama
# só o sorteio, o rand e o srand da CRT, o new/delete (0x944cf1/0x944c94) e a caixa; a divisão 64 bits (0x945e80) é
# feita em i64. Sem SEH. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira (a grade de jogos muda) e a
# confirmação no jogo real comparando. Comparadores: a ordem só a original (sorteia e aloca); as da copa pelo núcleo
# recursivo sobre uma saída própria, antes da original.
SAVE = False
ROTINAS = [('sortear_ordem_dos_clubes', 0x686a70, [0x81, 0xec, 0x68, 0x02, 0x00, 0x00]),
           ('clubes_do_lado_do_jogo', 0x521790, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('clubes_possiveis_do_jogo', 0x5219b0, [0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a ordem dos clubes da liga para a tabela de jogos e os clubes possíveis de cada lado de um jogo da copa',
        "the league's team order for the fixture grid and the possible teams on each side of a cup fixture",
        [('sortear_ordem_dos_clubes', 'League::draw_team_order', '0x686a70', 'league.cpp', 'thiscall, ret 4',
          'embaralha a tabela da liga pelo rand semeado e põe os clubes do mesmo estádio, dos estádios ligados e o Celtic e o Rangers em vagas pareadas',
          'shuffles the league table with the seeded rand and puts clubs sharing a ground, clubs with linked grounds and Celtic and Rangers in paired slots'),
         ('clubes_do_lado_do_jogo', 'Cup::get_fixture_side_teams', '0x521790', 'Cup.cpp', 'thiscall, ret 0x10',
          'o clube de um lado do jogo ou, vazio, os clubes que ainda podem chegar a ele (conta no global 0xae1880)',
          'the team on one side of the fixture or, if empty, the teams that can still reach it (count in global 0xae1880)'),
         ('clubes_possiveis_do_jogo', 'Cup::collect_possible_teams', '0x5219b0', 'Cup.cpp', 'thiscall, ret 0x10',
          'acrescenta à saída os clubes que podem jogar o jogo, descendo pelos jogos de onde vêm os lados vazios',
          'appends to the output the teams that can play the fixture, walking down the fixtures the empty sides come from')])
CAMPOS = [('League +0x3e: a conta dos clubes (palavra); +0x40 a semente da liga (palavra); +0xb1 a tabela (59 bytes: +0 o clube); +0xd9 as marcas (0x100: vagas pela tabela virtual +0xa8/+0xac); +4 a nação; ponteiro e valor; lê e grava a tabela (sortear_ordem_dos_clubes)',
           'League +0x3e: the club count (word); +0x40 the league seed (word); +0xb1 the table (59 bytes: +0 the club); +0xd9 the flags (0x100: slots from virtual +0xa8/+0xac); +4 the nation; pointer and value; reads and writes the table (sortear_ordem_dos_clubes)'),
          ('clube +0x69: o estádio (+0x48 o estádio ligado); [0x9cf5dc] a Escócia; [0x9d0118]/[0x9d011c] o Celtic e o Rangers; [0xdd7120] a semente do jogo; ponteiro e id; lê (sortear_ordem_dos_clubes)',
           'club +0x69: the stadium (+0x48 the linked stadium); [0x9cf5dc] Scotland; [0x9d0118]/[0x9d011c] Celtic and Rangers; [0xdd7120] the game seed; pointer and id; reads (sortear_ordem_dos_clubes)'),
          ('Cup +0x3c: a conta das rodadas (palavra); +0xa7 os clubes; +0xad os jogos de cada rodada (0x35 bytes: +0/+2 os lados, palavra, −1 vazio; +0xe/+0xa e +0x10/+0xc a rodada e o jogo de onde vem cada lado); [0xae1880] a conta; valor e ponteiro; lê e grava a conta (clubes_do_lado_do_jogo, clubes_possiveis_do_jogo)',
           'Cup +0x3c: the round count (word); +0xa7 the clubs; +0xad the fixtures of each round (0x35 bytes: +0/+2 the sides, word, −1 empty; +0xe/+0xa and +0x10/+0xc the round and fixture each side comes from); [0xae1880] the count; value and pointer; reads and writes the count (clubes_do_lado_do_jogo, clubes_possiveis_do_jogo)')]
INTERESSA = ['GPF2']
