# Os registros de carreira, os codigos de progresso e o banco da Copa da Grecia (gre_cup.cpp; leitor cx02, rodada 5,
# 24/11/2026). Competicoes da Grecia; ATE 2002-08-01.
# - 0x5e9ca0 GreCup::update_staff_records_for_status thiscall(clube, desfecho, fase, _, partida, final) ret 0x18 (a
#   virtual 0x44; 86 chamadas no mundo): o molde da ItaCup (italian_cup_staff_records) com o mata-mata na fase 10 e a
#   rodada de classificacao 0x14. Prologo mov al, [esp+0xc]; push esi.
# - 0x5ea2d0 GreCup::set_club_progress (a virtual 0x68): set_group_cup_club_progress do cx02_11 com as tabelas lidas do
#   exe (melhor 1,1,21,30; pior 17,17,21,30; atual 17,17,21,30) e os grupos abaixo da fase 10; compare_club_progress do
#   cx02_4.
# - 0x5ea3d0 GreCup::set_fixture_substitutes thiscall(partida) ret 4 (a virtual 0x7c): o byte baixo de +0x4d vira 0x77
#   quando o nome da rodada (+0x30) e 0xa, senao 0x37. Prologo mov eax, [esp+4]; cmp word [eax+0x30], 0xa (9 bytes).
# Integrar depois do cx02_4 e do cx02_11. Iguais no cm0102-1080.exe.
# Comparadores: os registros so a original (gravam os historicos), o retorno 0; o progresso lado a lado nos tres bytes;
# o banco lado a lado na palavra +0x4d.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_cup_update_staff_records_for_status', 0x5e9ca0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('gre_cup_set_club_progress', 0x5ea2d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('gre_cup_set_fixture_substitutes', 0x5ea3d0, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x83, 0x78, 0x30, 0x0a])]
DOCS = ('os registros de carreira, os códigos de progresso e o banco da Copa da Grécia', 'the Greek Cup staff records, progress codes and bench',
        [('gre_cup_update_staff_records_for_status', 'GreCup::update_staff_records_for_status', '0x5e9ca0', 'gre_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Grécia, quem passa de um grupo (fases 0 a 9) ganha o registro de classificação (rodada 0x14, peso 40) e quem cai o de não classificado; no mata-mata (fase 10) a eliminação e a passagem pesam pela rodada (50 a 100 em 5 rodadas), e o campeão e o vice ganham os registros da final',
          'rule: in the Greek Cup a club going through a group (stages 0 to 9) gets the qualified record (round 0x14, weight 40) and one going out the failed record; in the knockout (stage 10) elimination and progress weigh by round (50 to 100 over 5 rounds), and the winner and runner-up get the final records'),
         ('gre_cup_set_club_progress', 'GreCup::set_club_progress', '0x5ea2d0', 'gre_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Grécia, nos dez grupos, o 1º e o 2º lugares viram, nas estatísticas, a posição 17 (o melhor, 1), o 3º 21 e o 4º 30; no mata-mata, como vêm',
          'rule: in the Greek Cup, in the ten groups, 1st and 2nd place become position 17 in the statistics (the best one, 1), 3rd 21 and 4th 30; in the knockout, as they come'),
         ('gre_cup_set_fixture_substitutes', 'GreCup::set_fixture_substitutes', '0x5ea3d0', 'gre_cup.cpp', 'thiscall, ret 4',
          'regra: na Copa da Grécia as partidas da rodada 0xa têm 7 reservas e 7 substituições; as outras, 7 reservas e 3 substituições',
          'rule: in the Greek Cup the matches of round 0xa have 7 substitutes named and 7 substitutions; the others, 7 named and 3 substitutions')])
CAMPOS = []
INTERESSA = []
