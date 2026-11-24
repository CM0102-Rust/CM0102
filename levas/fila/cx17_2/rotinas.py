# A situação de classificação e os registros de carreira de uma seleção nas eliminatórias sul-americanas
# (wc_south_american_league.cpp; leitor cx17, rodada 5, 24/11/2026). Simulação: na carreira de teste a seção
# sul-americana é a campanha de 2004 (o construtor, já em Rust, leva o ano ao ciclo, nunca antes de 2004; a liga de 18
# rodadas vai de 29 de março de 2004 a 15 de novembro de 2005): as virtuais +0x70/+0x74 rodam com os jogos dela e a +0x44
# no fim da liga. Daí LIGAS = 'ENG' e ATE = '2005-12-31', como os cx16_7..9 (o pedido da rodada 5). A cobertura do mundo
# até 2002-08-01 não as viu (0 chamadas).
# - 0x921630 WcSouthAmericanLeague::get_qualification_status thiscall(clube) ret 4: Comp_Stats::get_club_stat 0x4a2e10
#   (push 0x12; push clube; mov ecx, [this+8]; ret 8, float) e _ftol 0x945640; o byte com sinal < 6 - (+0xee, byte com
#   sinal): -2, senão -3 (ax).
# - 0x921670 WcSouthAmericanLeague::get_qualification_status_change thiscall(clube) ret 4: antes = stat 0x13, agora =
#   0x12, vagas = 6 - (+0xee); agora < vagas: (antes < vagas) - 1; senão (antes < vagas). O byte em al.
# - 0x920a50 WcSouthAmericanLeague::update_staff_records_for_status thiscall(clube, desfecho, fase, saida, jogo, a6) ret
#   0x18 (virtual +0x44): 7: Staff_Records::add_qualified_for_round_record 0x86bdd0 (thiscall em [0xdd7f00]; clube, o
#   registro da Copa do Mundo [0xae23d0] + 0x6b x [0x9cf79c], 0, 0, 0x64), WorldCup::add_qualified_team 0x92e740 (push
#   clube; ecx = [0xadadfc + 4 x [0x9cf79c]]), *saida = 4, NationalTeams::add_qualified_nation 0x7779b0 (push registro;
#   push clube; mov ecx, [0xb74318]; ret 8), devolve -1; 1: o registro de classificação para a seção da Oceania
#   ([0x9cf76c]; 0xa0, 0, 0x46), 0; 0xfe: Staff_Records::add_failed_to_qualify_record 0x86c1d0 (clube, [this+4], 0, 0x64)
#   e NationalTeams::clear_current_competition 0x775000 (push [clube+0x53]), 0; outro, 0.
# Prólogos: mov eax, [esp+4]; push esi (5 bytes), push ebx; push esi; push edi; mov edi, [esp+0x10] (7) e mov al,
# [esp+8]; push esi (5). Iguais no cm0102-1080.exe (inteiras). Comparadores: as duas situações lado a lado no retorno; os
# registros só a original (não rodam duas vezes), com o plano (o retorno, *saida = 4 e a Copa do Mundo com mais uma).
# Usa o qualifying_stat_byte do cx16_8: integrar depois dele (INTEGRAR_ANTES.txt).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_south_american_league_get_qualification_status', 0x921630, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('wc_south_american_league_get_qualification_status_change', 0x921670, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('wc_south_american_league_update_staff_records_for_status', 0x920a50, [0x8a, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a situação de classificação e os registros de carreira nas eliminatórias sul-americanas',
        'the qualification status and the staff records in the South American qualifying',
        [('wc_south_american_league_get_qualification_status', 'WcSouthAmericanLeague::get_qualification_status', '0x921630', 'wc_south_american_league.cpp', 'thiscall, ret 4',
          'regra: nas eliminatórias sul-americanas uma seleção está classificada (-2) enquanto a sua posição na tabela (estatística 0x12) estiver dentro das vagas (seis menos as isentas), e fora (-3) nas outras',
          'rule: in the South American qualifying a nation is qualified (-2) while its table position (stat 0x12) is inside the places (six less the exempt nations), and out (-3) otherwise'),
         ('wc_south_american_league_get_qualification_status_change', 'WcSouthAmericanLeague::get_qualification_status_change', '0x921670', 'wc_south_american_league.cpp', 'thiscall, ret 4',
          'regra: nas eliminatórias sul-americanas a situação muda para baixo (-1) quando a seleção sai das vagas, para cima (1) quando entra nelas, e 0 nos outros casos',
          'rule: in the South American qualifying the status changes down (-1) when the nation leaves the places, up (1) when it enters them, and 0 otherwise'),
         ('wc_south_american_league_update_staff_records_for_status', 'WcSouthAmericanLeague::update_staff_records_for_status', '0x920a50', 'wc_south_american_league.cpp', 'thiscall, ret 0x18',
          'regra: no fim das eliminatórias sul-americanas a classificada ganha o registro da Copa do Mundo (peso 100), entra na Copa e na lista das classificadas e fica com a situação 4; a que vai à repescagem com a Oceania ganha o registro da seção da Oceania (peso 70); a eliminada ganha o registro de não classificada (peso 100) e encerra a campanha',
          'rule: when the South American qualifying ends a qualified nation gets the World Cup record (weight 100), enters the World Cup and the list of qualified nations and its status becomes 4; one going to the play-off with Oceania gets the Oceania section record (weight 70); an eliminated one gets the failed-to-qualify record (weight 100) and ends its campaign')])
CAMPOS = []
INTERESSA = []
