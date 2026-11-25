# Os registros de historico, a ordem dos cabecas de chave e o primeiro grupo do Mundial de Clubes (world_club_champ.cpp;
# leitor rx10, rodada 6, 25/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15 (ver rx10_11): o sorteio em 5/fev/2003
# (WorldClubChamp::update_for_date, ja em Rust: os inscritos, as posicoes previstas, o primeiro grupo 0x929aa0), os
# registros ao fim de cada fase (julho e agosto de 2003).
# - 0x929c70 WorldClubChamp::update_staff_records_for_status thiscall(clube, desfecho, fase, _, jogo, final) ret 0x18:
#   fase < 2: 6 -> 0x86bdd0(clube, comp, 0, 0x96, 0x19), 0xfe -> 0x86c1d0(clube, comp, 0x41c, 0x19), 0; fase 2: 1/2/3
#   -> 0x86a800/0x86b0b0/0x86b710(clube, jogo, final) e 0, outro desfecho -1; fase > 2: 0. Prologo mov al, [esp+0xc];
#   cmp al, 2 (6 bytes).
# - 0x92b400 compare_entrants_by_continent_strength cdecl(&a, &b) (a comparacao do qsort): sem nacao nos dois 0, so em
#   a -1, so em b 1; a forca do continente (f64 em +0xbe do registro +0x71 da nacao) de b menos a de a em x87 contra
#   0.0 (0x9689a0); empate: reputacao da nacao (+0x8e) de b - a, id da nacao de a - b, reputacao do clube (+0x80) de b
#   - a. Prologo mov eax, [esp+4]; mov ecx, [esp+8] (8 bytes).
# - 0x929aa0 WorldClubChamp::draw_first_group thiscall() ret 0: +0x3e = 4, malloc 0xec (caixa 0x220 e 0), qsort da
#   lista +0xee (o dword +0xf2) com 0x92b400, as linhas com os inscritos 0, 3, 6 e 9, League::send_group_draw_news
#   0x684230; 1. Prologo sub esp, 0x204 (6 bytes).
# Iguais no cm0102-1080.exe. Comparadores: os registros so a original (o retorno pelo plano); a comparacao lado a lado;
# o grupo so a original com o plano.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('world_club_champ_update_staff_records_for_status', 0x929c70, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x02]),
           ('compare_entrants_by_continent_strength', 0x92b400, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('world_club_champ_draw_first_group', 0x929aa0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('os registros, a ordem dos cabeças de chave e o primeiro grupo do Mundial de Clubes',
        'the Club World Championship records, seeding order and first group',
        [('world_club_champ_update_staff_records_for_status', 'WorldClubChamp::update_staff_records_for_status', '0x929c70', 'world_club_champ.cpp', 'thiscall, ret 0x18',
          'regra: nas fases de grupos quem passa ganha o registro da rodada 0x96 e quem sai o de eliminado; no mata-mata o campeão, o vice e o terceiro ganham os registros da final',
          'rule: in the group rounds the club going through gets the record of round 0x96 and the club going out the failed record; in the knockout the winner, the runner-up and the third get the final records'),
         ('compare_entrants_by_continent_strength', 'compare_entrants_by_continent_strength', '0x92b400', 'world_club_champ.cpp', 'cdecl',
          'regra: os cabeças de chave do Mundial de Clubes pela força do continente, o mais forte primeiro; depois pela reputação da nação, pelo id da nação (o menor primeiro) e pela reputação do clube; um clube sem nação vem antes',
          'rule: the Club World Championship seeds by the continent\'s strength, the stronger first; then by the nation\'s reputation, the nation id (lower first) and the club\'s reputation; a club without a nation comes first'),
         ('world_club_champ_draw_first_group', 'WorldClubChamp::draw_first_group', '0x929aa0', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: o primeiro grupo do Mundial de Clubes é o primeiro clube de cada um dos quatro potes de três (os doze ordenados pela força do continente: o 1º, 4º, 7º e 10º), com a notícia do sorteio',
          'rule: the Club World Championship\'s first group is the top club of each of the four pots of three (the twelve seeded by continent strength: the 1st, 4th, 7th and 10th), with the draw news')])
CAMPOS = []
INTERESSA = []
