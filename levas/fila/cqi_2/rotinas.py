# A classificação da UEFA (uefa_seeding.cpp), parte 2: a tabela dos clubes europeus e as vagas da Liga dos Campeões e
# da Intertoto. Rodam uma vez na carreira até 2002-06-01 (a virada do ano do jogo no começo de 2001, pela 0x9035a0:
# docs/cobertura_competicoes.txt, 1 chamada cada); decidem quem joga a Liga dos Campeões e a Intertoto de 2001/02.
# - 0x904040 (UefaSeeding::collect_european_clubs, 464 bytes): aloca a tabela (12 bytes por clube), tira a vaga europeia
#   de cada clube da Europa, grava o registro de ordenação e ordena pela nação (qsort do jogo com o comparador do jogo 0x905ce0);
#   em 2001 a ordem do euro.cfg (0x9059e0, original).
# - 0x904210 (UefaSeeding::assign_champions_league_places, 878 bytes): o campeão nos grupos e as faixas fixas por posição
#   da nação, com as capacidades de cada fase.
# - 0x904e30 (UefaSeeding::assign_intertoto_places, 669 bytes): voltas pelas nações até 60 clubes.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x904040: 0x9036e5 (lea edx, saída; mov ecx, ebx; push edx) -> thiscall(this, &saída), ret 4; a
#   devolução é lida como short (cmp si, bp; push esi).
# - quem chama a 0x904210, a 0x904580 e a 0x904e30: 0x90375d, 0x90376a, 0x903777 (push conta (esi); push tabela; mov
#   ecx, ebx) -> thiscall(this, tabela, conta), ret 8; a conta vai inteira para a 0x9050d0, que lê o short.
# - dentro da 0x904040: 0x944e46 push n; add esp, 4 -> cdecl; 0x53b900 push clube; add esp, 4 -> cdecl; 0x945333 push
#   0x905ce0; push 0xc; push conta; push tabela; add esp, 0x10 -> cdecl; 0x9059e0 push conta; push tabela; mov ecx, ebx
#   -> thiscall, ret 8; a caixa (aviso_em, linha 0x2ed).
# - dentro da 0x904210: 0x4afd80 push 0; push linha da competição; add esp, 8 -> cdecl; 0x9050d0 push 0; push 0; push k;
#   push nação; push conta; push tabela; mov ecx, ebp -> thiscall, ret 0x18 (o k é um dword com só o byte baixo gravado:
#   a 0x9050d0 lê o byte); a caixa (linha 0x35d).
# - dentro da 0x904e30: 0x903890 mov ecx, esi, sem push -> thiscall, ret 0; 0x9050d0 (push 1; push 1; push 1 ou push
#   0; push 1; push 1; nação; conta; tabela; mov ecx, esi) -> thiscall, ret 0x18; 0x9452ca push lista; add esp, 4 ->
#   cdecl.
# Os comparadores das rotinas que chamam a 0x4afd80 (o campeão/vice pelo histórico) incluem o histórico das
# competições nas regiões (a 0x4afd80 o reordena; ver a cqj_1).
# Comparadores (todos lado a lado): 0x904040 com a saída de quem chama na original e uma própria na cópia (liberada), os
# clubes, a linha de cada clube em [0xae2d14], a devolução e os registros das duas tabelas; 0x904210 a tabela das
# nações, os clubes e a marca da caixa; 0x904e30 em 2001 sem lista de voluntários (a 0x903890 devolve 0 sem efeito; a
# lista não pode ser solta duas vezes) a tabela, os clubes e o sorteio (cópia sem contar); fora disso só a original.
# Determinismo: nada lê pilha sem valor (o k da 0x904210 é passado limpo; a original empurra o dword com lixo nos bytes
# altos, que a 0x9050d0 não lê), heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('uefa_juntar_clubes_europeus', 0x904040, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('uefa_vagas_liga_dos_campeoes', 0x904210, [0x81, 0xec, 0x64, 0x02, 0x00, 0x00]),
           ('uefa_vagas_intertoto', 0x904e30, [0x83, 0xec, 0x10, 0x53, 0x55])]
DOCS = ('a classificação da UEFA: a tabela dos clubes europeus e as vagas da Liga dos Campeões e da Intertoto',
        'the UEFA seeding: the European club table and the Champions League and Intertoto places',
        [('uefa_juntar_clubes_europeus', 'UefaSeeding::collect_european_clubs', '0x904040', 'uefa_seeding.cpp', 'thiscall, ret 4',
          'a tabela dos clubes da Europa (fora as seleções), cada um sem vaga europeia, ordenada por nação e, dentro dela, pela divisão, pela posição e pela reputação [inferência: os campos +0x69 da divisão e +0x5f do clube]; em 2001 o euro.cfg pode pôr clubes na frente da sua nação',
          'the table of European clubs (national teams excluded), each without a European place, sorted by nation and, within it, by division, position and reputation [inference: the division +0x69 and club +0x5f fields]; in 2001 euro.cfg can move clubs to the front of their nation'),
         ('uefa_vagas_liga_dos_campeoes', 'UefaSeeding::assign_champions_league_places', '0x904210', 'uefa_seeding.cpp', 'thiscall, ret 8',
          'a regra da Liga dos Campeões (as fases 1 a 4 do jogo lidas como grupos, terceira, segunda e primeira preliminar [inferência]): o campeão da última edição vai direto aos grupos; as nações, pela posição no coeficiente: 1ª a 3ª 2 clubes nos grupos e 2 na terceira preliminar; 4ª a 6ª 2 e 1; 7ª a 9ª 1 e 1; 10ª 1 nos grupos e 1 na segunda preliminar; 11ª a 15ª 1 na terceira e 1 na segunda; 16ª 1 na terceira; 17ª a 29ª 1 na segunda; da 30ª em diante o campeão na primeira preliminar. Os clubes vão na ordem da tabela da nação (o campeão, o vice...); cada fase tem capacidade (grupos 16, ou 15 com o campeão; terceira 18, segunda 19, primeira 18) e a vaga de uma fase cheia passa para a seguinte; o clube que já tem vaga perde a dele',
          'the Champions League rule (the game\'s rounds 1 to 4 read as groups, third, second and first qualifying round [inference]): the last winner goes straight to the groups; nations, by coefficient position: 1st to 3rd 2 clubs in the groups and 2 in the third qualifying round; 4th to 6th 2 and 1; 7th to 9th 1 and 1; 10th 1 in the groups and 1 in the second qualifying round; 11th to 15th 1 in the third and 1 in the second; 16th 1 in the third; 17th to 29th 1 in the second; 30th onwards the champion in the first qualifying round. Clubs go in the nation\'s table order (champion, runner-up...); each round has a capacity (groups 16, or 15 with the holder; third 18, second 19, first 18) and a place in a full round moves to the next one; a club that already has a place loses its own'),
         ('uefa_vagas_intertoto', 'UefaSeeding::assign_intertoto_places', '0x904e30', 'uefa_seeding.cpp', 'thiscall, ret 8',
          'a regra da Intertoto (fases 1 a 3 do jogo: a 1 a primeira rodada [inferência]): 60 clubes; dá voltas pelas nações na ordem do coeficiente, cada nação com as suas vagas (da 0x903cc0) e os primeiros clubes ainda sem vaga europeia; as 8 primeiras nações entram na fase 1, da 9ª à 12ª na 2, depois na 3 (o segundo clube de uma nação uma fase adiante). A Inglaterra põe o clube voluntário (0x903890) ou o primeiro sem vaga e sem técnico humano, e um segundo quando tem 2 vagas',
          'the Intertoto rule: 60 clubs; it cycles through the nations in coefficient order, each nation with its places (from 0x903cc0) and its first clubs still without a European place; the first 8 nations enter round 1, the 9th to 12th round 2, then round 3 (a nation\'s second club one round later). England puts in the volunteer club (0x903890) or the first one without a place and without a human manager, and a second one when it has 2 places')])
CAMPOS = [('classificação da UEFA +0 tabela das nações (+0 id da nação, +0xe vagas na Intertoto, +0x11 clubes nas copas), +0x438 número de nações, +0x439 ano (short), +0x43b lista dos voluntários ingleses, +0x43f a conta dela; id, ponteiro e valor; lê e grava (uefa_vagas_liga_dos_campeoes, uefa_vagas_intertoto), lê o ano (uefa_juntar_clubes_europeus)',
           'UEFA seeding +0 nation table (+0 nation id, +0xe Intertoto places, +0x11 clubs in the cups), +0x438 number of nations, +0x439 year (short), +0x43b English volunteer list, +0x43f its count; id, pointer and value; reads and writes (uefa_vagas_liga_dos_campeoes, uefa_vagas_intertoto), reads the year (uefa_juntar_clubes_europeus)'),
          ('clube +0x1db competição europeia (−1 sem vaga), +0x1df fase de entrada; id e valor; grava (as três); +0x53 nação, +0x80 reputação, +0x5b divisão e +0x5f (zerados quando a competição da divisão não tem o +0x11c); ponteiro e valor; lê, grava +0x5b/+0x5f (uefa_juntar_clubes_europeus)',
           'club +0x1db European competition (−1 none), +0x1df entry round; id and value; writes (all three); +0x53 nation, +0x80 reputation, +0x5b division and +0x5f (cleared when the division\'s competition lacks +0x11c); pointer and value; reads, writes +0x5b/+0x5f (uefa_juntar_clubes_europeus)'),
          ('[0xae2d14] linha de 6 bytes por clube, o dword +0 = −1; valor; grava (uefa_juntar_clubes_europeus)',
           '[0xae2d14] 6-byte row per club, dword +0 = −1; value; writes (uefa_juntar_clubes_europeus)'),
          ('nação +0 id, +0x71 continente; id e ponteiro; lê (uefa_juntar_clubes_europeus, uefa_vagas_liga_dos_campeoes)',
           'nation +0 id, +0x71 continent; id and pointer; reads (uefa_juntar_clubes_europeus, uefa_vagas_liga_dos_campeoes)')]
INTERESSA = ['CMRTE']
