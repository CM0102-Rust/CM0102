# A classificação da Conmebol (conmebol_seeding.cpp), parte 2: a tabela dos clubes, a virada do ano e o campeão da Copa
# dos Campeões.
# - 0x4c6280 (collect_conmebol_clubs, 429 bytes): 2 chamadas até 2002-06-01 (a virada do ano, em 2001 e em 2002).
# - 0x4c61b0 (ConmebolSeeding::update_for_new_season, 201 bytes): 2 chamadas.
# - 0x4c6960 (ConmebolSeeding::set_bra_champ_cup_winner, 12 bytes): 1 chamada (o fim da Copa dos Campeões); o +0x71 só
#   é lido de 2003 em diante (0x4c6430): passageira para as mutações.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x4c6280: 0x4c61de (lea eax, saída; push eax; call, sem mov ecx logo antes; o ecx que sobra é o this da
#   0x4c61b0, não lido) -> stdcall(&saída), ret 4.
# - quem chama a 0x4c61b0: 0x4c2fe9 e 0x4c3a03 (mov ecx, [0xae1000]; push ebx) -> thiscall(this, pular), ret 4.
# - quem chama a 0x4c6960: 0x426a7e (mov ecx, [0xae1000]; push esi) -> thiscall(this, clube), ret 4.
# - dentro da 0x4c6280: 0x944e46 push n; add esp, 4 -> cdecl; 0x53b900 push clube; add esp, 4 -> cdecl; 0x945333 4
#   push, add esp, 0x10 -> cdecl; a caixa (aviso_em, linha 0xbb).
# - dentro da 0x4c61b0: 0x4c6280 como acima; 0x4c6430 push conta (eax inteiro: só o ax conta, a 0x4c6730 lê o short);
#   push tabela; mov ecx, esi -> thiscall, ret 8; 0x9452ca push; add esp, 4 -> cdecl; a caixa (linha 0x9b).
# Os comparadores das rotinas que chamam a 0x4afd80 (o campeão/vice pelo histórico) incluem o histórico das
# competições nas regiões (a 0x4afd80 o reordena; ver a cqj_1).
# Comparadores (todos lado a lado): 0x4c6280 com a saída de quem chama na original e uma própria na cópia (liberada), os
# clubes, a devolução e os registros das duas tabelas; 0x4c61b0 o ano gravado, os clubes e a marca da caixa (cada uma
# monta e solta a sua tabela); 0x4c6960 o +0x71.
# Determinismo: nada lê pilha, heap ou endereço como valor (a devolução da 0x4c6280 vai com os bytes altos zerados; a
# original deixa os do qsort, que ninguém lê).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('conmebol_juntar_clubes', 0x4c6280, [0xa1, 0x5c, 0x23, 0xae, 0x00]),
           ('conmebol_virada_do_ano', 0x4c61b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('conmebol_campeao_dos_campeoes', 0x4c6960, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10])]
DOCS = ('a classificação da Conmebol: a tabela dos clubes sul-americanos (e mexicanos), a virada do ano que dá as vagas da Libertadores e o campeão da Copa dos Campeões',
        'the CONMEBOL seeding: the South American (and Mexican) club table, the turn of the year that gives the Libertadores places and the Copa dos Campeões winner',
        [('conmebol_juntar_clubes', 'collect_conmebol_clubs', '0x4c6280', 'conmebol_seeding.cpp', 'stdcall, ret 4',
          'a tabela dos clubes da Conmebol: os da América do Sul e os do México (fora as seleções), cada um sem vaga continental, ordenados por nação e, dentro dela, pela divisão, pela posição e pela reputação [inferência: os campos +0x69 da divisão e +0x5f do clube]',
          'the CONMEBOL club table: South American and Mexican clubs (national teams excluded), each without a continental place, sorted by nation and, within it, by division, position and reputation [inference: the division +0x69 and club +0x5f fields]'),
         ('conmebol_virada_do_ano', 'ConmebolSeeding::update_for_new_season', '0x4c61b0', 'conmebol_seeding.cpp', 'thiscall, ret 4',
          'o calendário da Libertadores: a cada ano novo do jogo monta a tabela e dá as vagas',
          'the Libertadores calendar: every new game year it builds the table and gives the places'),
         ('conmebol_campeao_dos_campeoes', 'ConmebolSeeding::set_bra_champ_cup_winner', '0x4c6960', 'conmebol_seeding.cpp', 'thiscall, ret 4',
          'guarda o campeão da Copa dos Campeões: de 2003 em diante ele tem vaga na Libertadores',
          'stores the Copa dos Campeões winner: from 2003 on it gets a Libertadores place')])
CAMPOS = [('classificação da Conmebol +0x6f ano (short), +0x71 id do campeão da Copa dos Campeões; valor e id; lê e grava (conmebol_virada_do_ano, conmebol_campeao_dos_campeoes)',
           'CONMEBOL seeding +0x6f year (short), +0x71 Copa dos Campeões winner id; value and id; reads and writes (conmebol_virada_do_ano, conmebol_campeao_dos_campeoes)'),
          ('clube +0x1db competição continental (−1 sem vaga), +0x53 nação, +0x80 reputação, +0x5b divisão e +0x5f (zerados quando a competição da divisão não tem o +0x11c); id, ponteiro e valor; lê e grava (conmebol_juntar_clubes)',
           'club +0x1db continental competition (−1 none), +0x53 nation, +0x80 reputation, +0x5b division and +0x5f (cleared when the division\'s competition lacks +0x11c); id, pointer and value; reads and writes (conmebol_juntar_clubes)'),
          ('nação +0 id, +0x71 continente; id e ponteiro; lê (conmebol_juntar_clubes)',
           'nation +0 id, +0x71 continent; id and pointer; reads (conmebol_juntar_clubes)')]
INTERESSA = ['CMRTE']
