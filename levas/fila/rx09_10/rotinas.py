# Os registros dos tecnicos, o texto das noticias e a lista dos clubes da Segunda Divisao sueca (swe_second.cpp; leitor
# rx09, rodada 6, 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01 (o fim
# das secoes, o acesso e a repescagem em novembro de 2001).
# - 0x892ee0 SweSecond::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: fase < 5:
#   status 1 -> 0x86bdd0(clube, registro, 0x458, 0, 0x28); 5: sem reserva 0; a secao pela reserva (caixa 0x38b e -1);
#   League::find_club_row 0x68ab70(secao, clube, 0) (0 -> -1); status 1: +0x37 = 0 e 0x869480(.., 0x64); senao 0xfe;
#   6: sem reserva ou divisao (+0x57) diferente do registro da liga -1; secao (caixa 0x3bb); linha; status 1: 0xfe;
#   senao 3 e 0x86a1c0; depois 0.
# - 0x893160 SweSecond::build_fate_news_text thiscall(..) ret 0x28: os pushes das chamadas conferidos (0x48c6d0 com a7 =
#   0; 0x4b0b80 (clube, a5, a8, desfecho, a6, texto); 0x4b4420 (clube, a5, desfecho, texto)); os textos 0x989c18,
#   0x987528, 0x987594 e 0x987564 com o genero curto e o nome curto do clube e o vinculo 0x7d5.
# - 0x893320 SweSecond::get_club_array thiscall(&lista, &n) ret 8: n = 72, malloc 0x120 (caixa 0x464, n = 0); a secao
#   Norte e cada fase 0..4 em +0x3e + i, i < +0x3e da fase (o defeito do cx14_3: sem deslocamento).
# Prologos: mov al, [esp+0xc]; sub esp, 0x200 (10 bytes); mov eax, [esp+0xc]; push esi (5 bytes); sub esp, 0x204 (6).
# Iguais no cm0102-1080.exe.
# Comparadores: os registros so a original e o plano (o resultado e o status da linha); os textos proprios lado a
# lado (compare_text_writer); a lista so a original e as vagas.
# Depende do cx14_4 (CategoryBRecord, CategoryBRecordPlan), do cx13_6 (play_off_club_text), do cx14_3
# (grouped_league_club_array), do rx09_8 (SWEDISH_SECOND_SECTIONS) e do rx09_7 (SWEDISH_SECOND_SOURCE).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('swe_second_update_staff_records_for_status', 0x892ee0, [0x8a, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_second_build_fate_news_text', 0x893160, [0x8b, 0x44, 0x24, 0x0c, 0x56]),
           ('swe_second_get_club_array', 0x893320, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('os registros dos técnicos, o texto das notícias e a lista dos clubes da Segunda Divisão sueca',
        'the staff records, the fate news text and the club list of the Swedish Second Division',
        [('swe_second_update_staff_records_for_status', 'SweSecond::update_staff_records_for_status', '0x892ee0', 'swe_second.cpp', 'thiscall, ret 0x18',
          'regra: nas seções, o campeão ganha o registro de classificação; no acesso, o vencedor ganha o de acesso e a linha dele na seção fica 0, o perdedor 0xfe; na repescagem, quem escapa fica 0xfe e quem cai fica 3, com o registro de rebaixamento',
          'rule: in the sections, the winner gets the qualified record; in the promotion play-off the winner gets the promotion record and its section row becomes 0, the loser 0xfe; in the relegation play-off a club that stays up becomes 0xfe and one that goes down 3, with the relegation record'),
         ('swe_second_build_fate_news_text', 'SweSecond::build_fate_news_text', '0x893160', 'swe_second.cpp', 'thiscall, ret 0x28',
          'regra: no acesso, sem notícia, "sobe" ou "fica sem o acesso"; na repescagem, sem notícia, "escapa do rebaixamento" ou "é rebaixado", e a notícia comum de vitória ou derrota no mata-mata nos outros casos; com notícia, o texto do mata-mata',
          'rule: in the promotion play-off, without a news item, "win promotion" or "miss out on promotion"; in the relegation play-off, without one, "avoid relegation" or "relegated", and the common cup win or defeat news otherwise; with a news item, the cup tie text'),
         ('swe_second_get_club_array', 'SweSecond::get_club_array', '0x893320', 'swe_second.cpp', 'thiscall, ret 8',
          'regra: a lista dos 72 clubes da Segunda sueca: os 12 da seção Norte e, depois deles, os de cada uma das outras cinco seções; defeito do original: as cinco seções são escritas nas mesmas doze vagas (só a última fica) e as outras 48 ficam sem valor',
          'rule: the list of the 72 Swedish Second Division clubs: the 12 of the North section and, after them, those of each of the other five sections; defect of the original: the five sections are written into the same twelve slots (only the last stays) and the other 48 are left unset')])
CAMPOS = []
INTERESSA = []
