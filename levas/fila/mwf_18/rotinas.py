# A troca com a Conference quando ela não é jogada (eng_prm.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até
# 2002-08-01; docs/cobertura_mundo.txt: 1 vez, na virada inglesa de meados de 2002; a Conference não é carregada).
# - 0x574e60 (EngPrm::exchange_conference_clubs_unplayed): o clube mais forte da Conference (pela reputação, sem
#   embaralhar) sobe para a Third Division se o estádio cumpre as exigências dela (Finance::upgrade_stadium_for_league,
#   que pode ampliar o estádio), e os rebaixados da Third descem para a Conference; senão o último da Third fica (status
#   0xfe) e o clube da Conference recebe a notícia. Sem clube na Conference, a caixa de erro (linha 0x2be).
# Convenções: chamada pela virada da Premier com mov ecx, esi e sem argumentos (ret 0); dentro: 0x944e46 (cdecl),
# 0x4afbd0 (3 pushes -> cdecl), 0x5a0590 (push club; mov ecx, [0xae2a38] -> thiscall, ret 4), 0x599e00 (4 pushes;
# mov ecx, eax -> thiscall, ret 0x10), 0x6826d0 (thiscall, ret 4), 0x6830b0 e 0x6831a0 (thiscall, ret 0xc), 0x48ece0
# (4 pushes -> thiscall, ret 0x10).
# Comparador: só a original (pode ampliar o estádio, move clubes, manda notícia), conferindo depois que os rebaixados
# da Third estão na Conference (ou que o último ficou com 0xfe).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('eng_prm_exchange_conference_clubs_unplayed', 0x574e60, [0xa1, 0xfc, 0xad, 0xad, 0x00])]
DOCS = ('a troca com a Conference quando ela não é jogada',
        'the Conference exchange when the Conference is not played',
        [('eng_prm_exchange_conference_clubs_unplayed', 'EngPrm::exchange_conference_clubs_unplayed', '0x574e60', 'eng_prm.cpp', 'thiscall, ret 0',
          'regra: sem a Conference jogada, o clube mais forte dela (pela reputação) sobe para a Third Division se o estádio cumpre as exigências da Third, e os rebaixados da Third descem para a Conference; senão o último da Third fica e o clube da Conference recebe a notícia',
          "rule: without a played Conference, its strongest club (by reputation) goes up to the Third Division if its ground meets the Third Division's requirements, and the Third Division's relegated clubs go down to the Conference; otherwise the Third Division's bottom club stays up and the Conference club gets the news")])
CAMPOS = [('liga +0xe2/+0xe4 (exigências do estádio); linha da tabela +0x37 (3 = rebaixado, 0xfe = fica); clube +0x57, +0x5b; valor; lê/grava',
           'league +0xe2/+0xe4 (ground requirements); table row +0x37 (3 = relegated, 0xfe = stays); club +0x57, +0x5b; value; reads/writes')]
