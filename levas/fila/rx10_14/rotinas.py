# O segundo e o terceiro grupos do Mundial de Clubes (world_club_champ.cpp; leitor rx10, rodada 6, 25/11/2026).
# Simulacao com as 8 ligas e ATE 2003-08-15 (ver rx10_11): roda no sorteio de 5/fev/2003 (update_for_date, ja em Rust,
# chama 0x929e60 pelo endereco) depois do primeiro grupo.
# - 0x929e60 WorldClubChamp::create_second_and_third_groups thiscall() ret 0: a lista de 4 (malloc 0x10, caixa 0x302 e
#   0); os inscritos 3k+1 da lista +0xee; a tabela de rodadas pela virtual +0x3c(0, &rodadas, &extra, 0); um
#   League_Stage (new 0xee; 0x68b100 com o campeonato, 4, a lista, 1 turno, a competicao, a tabela, as rodadas, vitoria,
#   empate, categoria, +0xc5, +0xbe, o ano, fase 0, o extra, a importancia, 2, 0, +0xd9, -1, 0, 2) na vaga 0; a tabela
#   liberada; sem fase a caixa 0x318 e 0 (a lista vaza); +0x30 = 0 e League::send_group_draw_news 0x684230 na fase.
#   Depois o mesmo com os inscritos 3k+2 na vaga 1 (caixa 0x332), +0x30 = 1, a noticia, a lista liberada e 1. Prologo
#   mov eax, fs:[0] (6 bytes; o quadro SEH so cobre os new).
# Iguais no cm0102-1080.exe. Comparador: so a original (aloca e manda noticias), com o plano (as duas fases com os
# clubes certos, em qualquer ordem, e a fase atual 1).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('world_club_champ_create_second_and_third_groups', 0x929e60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o segundo e o terceiro grupos do Mundial de Clubes', 'the Club World Championship second and third groups',
        [('world_club_champ_create_second_and_third_groups', 'WorldClubChamp::create_second_and_third_groups', '0x929e60', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: o segundo e o terceiro grupos do Mundial de Clubes são o segundo e o terceiro clube de cada um dos quatro potes (os inscritos 2, 5, 8, 11 e 3, 6, 9, 12), turno único, cada um com a notícia do sorteio',
          'rule: the Club World Championship\'s second and third groups are the second and the third club of each of the four pots (entrants 2, 5, 8, 11 and 3, 6, 9, 12), single round robin, each with its draw news')])
CAMPOS = []
INTERESSA = []
