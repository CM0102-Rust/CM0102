# A situação de classificação de uma seleção na Copa do Mundo e a mudança dela (world_cup.cpp; leitor cx17, rodada 5,
# 24/11/2026). Simulação: as virtuais +0x70/+0x74 rodam com os resultados da Copa de 2002 (31 de maio a 30 de junho):
# LIGAS = 'ENG' e ATE = '2002-08-01', como o cx17_5.
# - 0x92fae0 WorldCup::get_qualification_status thiscall(clube) ret 4: Comp_Stats::get_club_stat 0x4a2e10 (push 0x12;
#   push clube; mov ecx, [this+8]; ret 8, float) e _ftol 0x945640; o byte: 1 -> -1, 2 -> 0x96, < 5 -> 0x82, < 9 ->
#   0x78, < 0x11 -> 0x14, senão -4 (ax; comparações com sinal).
# - 0x92fb40 WorldCup::get_qualification_status_change thiscall(clube) ret 4: antes = stat 0x13, agora = 0x12 (bytes com
#   sinal); faixas do agora < 3, < 9, < 0x11, resto; do antes < 5, < 9, < 0x11, resto; a tabela de 4 x 4 (0 na mesma
#   faixa). O byte em al. Defeito do original [inferência]: a primeira faixa do agora acaba em 3 e a do antes em 5; um
#   código que fica em 3 ou 4 (semifinalista) dá mudança 2.
# Prólogos: mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes) e push ebx; push esi; push edi; mov edi, [esp+0x10] (7). Iguais
# no cm0102-1080.exe (inteiras). Comparadores: lado a lado no retorno (só leem as estatísticas).
# Usa o qualifying_stat_byte do cx16_8: integrar depois dele (INTEGRAR_ANTES.txt).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'  # 26/11/2026: in the night round with cx16_7/cx16_8 (its dependencies)
ROTINAS = [('world_cup_get_qualification_status', 0x92fae0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('world_cup_get_qualification_status_change', 0x92fb40, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('a situação de classificação na Copa do Mundo e a mudança dela',
        'the World Cup qualification status and its change',
        [('world_cup_get_qualification_status', 'WorldCup::get_qualification_status', '0x92fae0', 'world_cup.cpp', 'thiscall, ret 4',
          'regra: a situação de uma seleção na Copa do Mundo pelo código de progresso: -1 para o campeão, 150 para o vice, 130 para os semifinalistas, 120 para os das quartas, 20 para os das oitavas e -4 para os que caíram nos grupos',
          'rule: a nation status in the World Cup by its progress code: -1 for the champion, 150 for the runner-up, 130 for the semi-finalists, 120 for the quarter-finalists, 20 for the last 16 and -4 for those out in the groups'),
         ('world_cup_get_qualification_status_change', 'WorldCup::get_qualification_status_change', '0x92fb40', 'world_cup.cpp', 'thiscall, ret 4',
          'regra: a mudança da situação na Copa do Mundo entre o código anterior e o atual: 0 na mesma faixa, positiva quando desce de faixa e negativa quando sobe, pelo número de faixas (as faixas do atual acabam em 3, 9 e 17; as do anterior em 5, 9 e 17)',
          'rule: the World Cup status change between the previous code and the current one: 0 in the same band, positive going down the bands and negative going up, by the number of bands (the current code bands end at 3, 9 and 17; the previous one at 5, 9 and 17)')])
CAMPOS = []
INTERESSA = []
