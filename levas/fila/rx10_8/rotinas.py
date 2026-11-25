# O calendario, as fases das zonas e a lista de clubes da Segunda Divisao B portuguesa (por_second_b.cpp; leitor rx10,
# rodada 6, 25/11/2026). Simulacao com Portugal e a base NORMAL (ver rx10_7). O calendario e as zonas rodam na criacao
# do jogo e na virada portuguesa de 2002; a lista de clubes quando o jogo a pede (virtual).
# - 0x7d1c40 PorSecondB::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10: fase >= 2 (com sinal): 0;
#   senao liga = 1, 38 rodadas, extra 0x453 (-1), 0x457 (0) ou 0x456 (outras), antes do malloc 0x9a6 (caixa 0xb9);
#   as mesmas 38 datas em qualquer fase, cada uma com o seu jogo vazio (o molde eng_second_set_league_rounds). Tiradas
#   por emulacao (unicorn) nas fases -1, 0, 1 e 2 e nos anos 1999 a 2003 (iguais). Prologo mov al, [esp+4]; sub esp,
#   0x200 (10 bytes).
# - 0x7d2870 PorSecondB::create_zone_stage thiscall(fase) ret 4: create_section_stage (rx10_4) com 20 clubes da
#   Segunda B 0x9cf7c0 (id), a zona Centro 0x9cf618 na fase 0 e a Sul 0x9cf61c nas outras, as vagas da propria liga
#   (+0xbe), sem marcar times B; caixas 0x246, 0x257, 0x264 e 0x26f. Prologo push -1; push 0x95fb0e (7 bytes).
# - 0x7d2b80 PorSecondB::get_club_array thiscall(&lista, &n) ret 8: n = 60, malloc 0xf0 (caixa 0x290); a Norte e as
#   duas zonas, mas as duas a partir do indice 20 (o laco usa o numero de clubes da propria liga e nao soma a zona).
#   Prologo sub esp, 0x204 (6 bytes).
# Depende do rx10_4 e do rx10_5 (compare_section_schedule). Iguais no cm0102-1080.exe. Comparadores: o calendario e a
# lista lado a lado (a lista nos 40 indices gravados); as zonas so a original com o plano.
# Defeito do original: a lista de 60 clubes perde a zona Centro (a Sul a sobrescreve nos indices 20 a 39) e os indices
# 40 a 59 ficam sem gravar (o lixo do malloc).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('por_second_b_create_schedule', 0x7d1c40, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_second_b_create_zone_stage', 0x7d2870, [0x6a, 0xff, 0x68, 0x0e, 0xfb, 0x95, 0x00]),
           ('por_second_b_get_club_array', 0x7d2b80, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o calendário, as zonas e a lista de clubes da Segunda Divisão B portuguesa',
        'the Portuguese Segunda B schedule, zone stages and club array',
        [('por_second_b_create_schedule', 'PorSecondB::create_schedule', '0x7d1c40', 'por_second_b.cpp', 'thiscall, ret 0x10',
          'regra: as três zonas da Segunda B têm o mesmo calendário: 38 rodadas de 29 de agosto a 11 de junho, quase todas no dia 6 da semana [inferência: domingo], algumas no dia 5 e duas no meio da semana',
          'rule: the three Segunda B zones share one schedule: 38 rounds from 29 August to 11 June, mostly on weekday 6 [inference: Sunday], some on weekday 5 and two midweek'),
         ('por_second_b_create_zone_stage', 'PorSecondB::create_zone_stage', '0x7d2870', 'por_second_b.cpp', 'thiscall, ret 4',
          'regra: as zonas Centro (fase 0) e Sul (fase 1) da Segunda B são fases próprias, 20 clubes cada, turno e returno, com as vagas da liga (um sobe, quatro caem)',
          'rule: the Segunda B Central (stage 0) and South (stage 1) zones are stages of their own, 20 clubs each, home and away, with the league\'s places (one up, four down)'),
         ('por_second_b_get_club_array', 'PorSecondB::get_club_array', '0x7d2b80', 'por_second_b.cpp', 'thiscall, ret 8',
          'regra: a lista de clubes da Segunda B tem 60 lugares, os 20 da Norte primeiro; mas as duas zonas são copiadas a partir do lugar 20 (a Sul por cima da Centro), e os últimos 20 lugares nunca são gravados (defeito do original)',
          'rule: the Segunda B club array has 60 places, the North\'s 20 first; but both zones are copied from place 20 (the South over the Central), and the last 20 places are never written (a defect of the original)')])
CAMPOS = []
INTERESSA = []
