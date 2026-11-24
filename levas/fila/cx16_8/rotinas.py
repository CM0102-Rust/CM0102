# A situação de classificação de uma seleção nas eliminatórias europeias, da CONCACAF e da Oceania (wc_europe_league.cpp,
# wc_oceania_league.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação: as virtuais +0x70/+0x74 rodam com os resultados
# das eliminatórias (a campanha de 2004 na carreira de teste): ATE = '2005-12-31' (o pedido da rodada 5).
# - 0x91b6e0 WcEuropeLeague::get_qualification_status thiscall(clube) ret 4: Comp_Stats::get_club_stat 0x4a2e10 (push
#   0x12; push clube; mov ecx, [this+8]; ret 8, float) e _ftol 0x945640; o byte com sinal < 12: -2, senão -3 (ax).
# - 0x91b710 WcEuropeLeague::get_qualification_status_change thiscall(clube) ret 4: antes = stat 0x13, agora = 0x12
#   (bytes com sinal); agora < 12: (antes < 12) - 1; senão (antes < 12). O byte em al.
# - 0x920260 WcOceaniaLeague::get_qualification_status_change thiscall(clube) ret 4 (também na vtable da CONCACAF,
#   +0x74): agora = 1: -1 se antes != 1, senão 0; senão 1 se antes = 1. O byte em al.
# Prólogos: mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes) e push ebx; push esi; push edi; mov edi, [esp+0x10] (7). Iguais
# no cm0102-1080.exe. Comparadores: lado a lado no retorno (só leem as estatísticas).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_europe_league_get_qualification_status', 0x91b6e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('wc_europe_league_get_qualification_status_change', 0x91b710, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('wc_oceania_league_get_qualification_status_change', 0x920260, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('a situação de classificação de uma seleção nas eliminatórias',
        'the qualification status of a nation in the qualifying',
        [('wc_europe_league_get_qualification_status', 'WcEuropeLeague::get_qualification_status', '0x91b6e0', 'wc_europe_league.cpp', 'thiscall, ret 4',
          'regra: nas eliminatórias europeias a situação de uma seleção está em aberto (-2) enquanto o código de progresso (estatística 0x12) for menor que 12, e decidida (-3) a partir de 12',
          'rule: in the European qualifying a nation status is open (-2) while its progress code (stat 0x12) is below 12, and decided (-3) from 12 on'),
         ('wc_europe_league_get_qualification_status_change', 'WcEuropeLeague::get_qualification_status_change', '0x91b710', 'wc_europe_league.cpp', 'thiscall, ret 4',
          'regra: nas eliminatórias europeias a situação muda para baixo (-1) quando o código cai abaixo de 12 vindo de 12 ou mais, para cima (1) quando sobe a 12 ou mais vindo de baixo, e 0 nos outros casos',
          'rule: in the European qualifying the status changes down (-1) when the code falls below 12 from 12 or more, up (1) when it rises to 12 or more from below, and 0 otherwise'),
         ('wc_oceania_league_get_qualification_status_change', 'WcOceaniaLeague::get_qualification_status_change', '0x920260', 'wc_oceania_league.cpp', 'thiscall, ret 4',
          'regra: nas eliminatórias da Oceania e da CONCACAF a situação muda para cima (1) quando o código sai de 1, para baixo (-1) quando passa a 1 vindo de outro, e 0 nos outros casos',
          'rule: in the Oceania and CONCACAF qualifying the status changes up (1) when the code leaves 1, down (-1) when it becomes 1 from another code, and 0 otherwise')])
CAMPOS = []
INTERESSA = []
