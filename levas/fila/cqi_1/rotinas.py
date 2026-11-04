# A classificação da UEFA (uefa_seeding.cpp), parte 1: os coeficientes das nações, as vagas de cada nação e o n-ésimo
# clube da nação. Rodam na virada do ano do jogo pela 0x9035a0 (UefaSeeding::update_season), uma vez na carreira até
# 2002-06-01 (o começo de 2001: docs/cobertura_competicoes.txt, 1 chamada cada), e a 0x9050d0 531 vezes (as vagas da
# Liga dos Campeões 0x904210, da Copa da UEFA 0x904580 e da Intertoto 0x904e30).
# - 0x903b50 (UefaSeeding::roll_coefficients, 364 bytes, x87): fora de 2001 as 5 temporadas de cada nação andam uma
#   casa e a nova é pontos / clubes; o coeficiente é a soma das 5 (double, arredondada para float); ordena a tabela
#   (qsort do jogo com o comparador do jogo 0x905cb0) e grava a posição no +0x1df da seleção.
# - 0x903cc0 (UefaSeeding::set_place_counts, 884 bytes): as vagas de cada nação pela posição; o fair play (2001: fixo;
#   depois: sorteio).
# - 0x9050d0 (uefa_find_nth_unplaced_club_of_nation, 345 bytes): o n-ésimo clube da nação sem vaga/sem humano.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x903b50 e a 0x903cc0: 0x9036d2 e 0x9036d9 (mov ecx, ebx; call, sem push) -> thiscall, ret 0.
# - quem chama a 0x9050d0: 0x904485 (push 0; push 0; push n; push nação; push conta; push clubes; mov ecx, ebp),
#   0x904eb0 e 0x904f4c (push 1; push 1; push 1; ...; mov ecx, esi), 0x905024 (push 0; push 1; push 1; ...; mov ecx,
#   esi), 0x9048ba e 0x90490d (push 0; push 1; push 1; ...; mov ecx, edi) -> thiscall(this, clubes, conta, nação, n,
#   sem_vaga, sem_humano), ret 0x18 (o this não é lido). O n vai de um dword com lixo nos bytes altos na 0x904210 (só o
#   byte baixo é gravado): a rotina lê só o byte.
# - dentro da 0x903b50: 0x945333 push cmp; push 0x12; push conta; push tabela; add esp, 0x10 -> cdecl (qsort);
#   0x53b3d0 push nação; add esp, 4 -> cdecl.
# - dentro da 0x903cc0: 0x90cfc0 push conta; add esp, 0xc depois de três -> cdecl; a caixa (aviso_em, linha 0x285).
# - dentro da 0x9050d0: 0x946464 push 0x905ce0; push 0xc; push conta; push tabela; push &chave; add esp, 0x14 ->
#   cdecl (bsearch); 0x600a50 push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl.
# Comparadores: lado a lado na 0x903b50 (a tabela, as temporadas de cada nação e o +0x1df das seleções) e na 0x903cc0
# (a tabela, o sorteio com a cópia sem contar, a marca da caixa); a devolução na 0x9050d0 (só lê).
# Determinismo: a chave da busca binária da original só tem a nação (+4) e o +8 = 0xff gravados; o comparador do jogo
# (0x905ce0) não lê o resto quando o +8 é 0xff; a cópia zera o resto. Nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('uefa_rodar_coeficientes', 0x903b50, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('uefa_vagas_por_nacao', 0x903cc0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('uefa_enesimo_clube_da_nacao', 0x9050d0, [0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x44, 0x24, 0x14])]
DOCS = ('a classificação da UEFA: os coeficientes das nações, as vagas de cada nação nas três copas europeias e o n-ésimo clube da nação',
        'the UEFA seeding: the nations\' coefficients, each nation\'s places in the three European cups and the nth club of a nation',
        [('uefa_rodar_coeficientes', 'UefaSeeding::roll_coefficients', '0x903b50', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'a regra do coeficiente da UEFA: a nação soma as 5 últimas temporadas (cada temporada = os pontos dos clubes dela nas copas europeias ÷ o número de clubes); na virada do ano as temporadas andam uma casa (em 2001 o jogo usa as do banco de dados, sem andar); a tabela é ordenada do maior coeficiente para o menor e a posição fica gravada na seleção da nação',
          'the UEFA coefficient rule: a nation adds up its last 5 seasons (each season = its clubs\' points in the European cups ÷ the number of clubs); at the turn of the year the seasons shift by one (in 2001 the game uses the database ones, without shifting); the table is sorted from the highest coefficient down and the position is stored in the nation\'s national team'),
         ('uefa_vagas_por_nacao', 'UefaSeeding::set_place_counts', '0x903cc0', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'as vagas pela posição no coeficiente. Liga dos Campeões: 1º a 3º 4 vagas, 4º a 6º 3, 7º a 15º 2, 16º a 48º 1. Copa da UEFA: 1º a 6º 3, 7º e 8º 4, 9º a 15º 2, 16º a 21º 3, 22º a 48º 2, 49º e 50º 1 (Liechtenstein e San Marino sem vaga). Intertoto: Moldávia, San Marino, Andorra, Bósnia e Liechtenstein 0, Escócia 1; em 2001 França 4, Alemanha 3, Itália 1, Portugal, Noruega, Espanha e Rússia 0; fora de 2001 Inglaterra 1; as outras, na ordem do coeficiente, 2 vagas até a 16ª delas, 1 até a 43ª, depois 0. Fair play: três nações ganham uma vaga a mais na Copa da UEFA (2001: Finlândia, Eslováquia e Bielorrússia; depois, três sorteadas, nunca a Itália). Só as vagas da Intertoto são lidas pela distribuição (0x904e30); as da Liga dos Campeões e da Copa da UEFA (e o fair play) ficam gravadas, mas a distribuição usa faixas fixas pela posição [inferência: servem às telas]',
          'places by coefficient position. Champions League: 1st to 3rd 4 places, 4th to 6th 3, 7th to 15th 2, 16th to 48th 1. UEFA Cup: 1st to 6th 3, 7th and 8th 4, 9th to 15th 2, 16th to 21st 3, 22nd to 48th 2, 49th and 50th 1 (Liechtenstein and San Marino none). Intertoto: Moldova, San Marino, Andorra, Bosnia and Liechtenstein 0, Scotland 1; in 2001 France 4, Germany 3, Italy 1, Portugal, Norway, Spain and Russia 0; outside 2001 England 1; the others, in coefficient order, 2 places up to the 16th of them, 1 up to the 43rd, then 0. Fair play: three nations get one extra UEFA Cup place (2001: Finland, Slovakia and Belarus; later, three drawn at random, never Italy). Only the Intertoto places are read by the distribution (0x904e30); the Champions League and UEFA Cup ones (and fair play) are stored, but the distribution uses fixed bands by position [inference: they feed the screens]'),
         ('uefa_enesimo_clube_da_nacao', 'uefa_find_nth_unplaced_club_of_nation', '0x9050d0', 'uefa_seeding.cpp', 'thiscall, ret 0x18 (this não lido)',
          'na tabela dos clubes europeus ordenada por nação, o n-ésimo clube da nação, pulando se pedido os que já têm vaga europeia e os de técnico humano; é por ela que cada vaga vai ao clube seguinte da nação (a ordem dentro da nação vem da 0x904040)',
          'in the European club table sorted by nation, the nth club of the nation, skipping on request those already holding a European place and those with a human manager; this is how each place goes to the nation\'s next club (the order within the nation comes from 0x904040)')])
CAMPOS = [('classificação da UEFA +0 tabela das nações (registros de 0x12: +0 id da nação, +4 a nação, +8 coeficiente float, +0xc vagas na Liga dos Campeões, +0xd na Copa da UEFA, +0xe na Intertoto, +0xf pontos da temporada short, +0x11 clubes nas copas char), +0x438 número de nações; id, ponteiro e valor; lê e grava (uefa_rodar_coeficientes, uefa_vagas_por_nacao)',
           'UEFA seeding +0 nation table (0x12-byte records: +0 nation id, +4 nation, +8 float coefficient, +0xc Champions League places, +0xd UEFA Cup, +0xe Intertoto, +0xf season points short, +0x11 clubs in the cups char), +0x438 number of nations; id, pointer and value; reads and writes (uefa_rodar_coeficientes, uefa_vagas_por_nacao)'),
          ('nação +0xe0..+0x108 as 5 temporadas do coeficiente (double); valor; lê e grava (uefa_rodar_coeficientes)',
           'nation +0xe0..+0x108 the 5 coefficient seasons (double); value; reads and writes (uefa_rodar_coeficientes)'),
          ('clube +0x1df (na seleção: a posição da nação no coeficiente); valor; grava (uefa_rodar_coeficientes); +0x1db competição europeia do clube (−1 sem vaga); id; lê (uefa_enesimo_clube_da_nacao)',
           'club +0x1df (on the national team: the nation\'s coefficient position); value; writes (uefa_rodar_coeficientes); +0x1db club\'s European competition (−1 none); id; reads (uefa_enesimo_clube_da_nacao)'),
          ('[0xae2c92] ano do jogo; valor; lê (uefa_rodar_coeficientes, uefa_vagas_por_nacao)',
           '[0xae2c92] game year; value; reads (uefa_rodar_coeficientes, uefa_vagas_por_nacao)')]
INTERESSA = ['CMRTE']
