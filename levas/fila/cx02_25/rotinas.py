# A fase seguinte da Copa da Grecia (gre_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Competicoes da Grecia; ATE
# 2002-08-01 (1 chamada no mundo: o fim dos grupos, quando o mata-mata e montado).
# - 0x5e95c0 GreCup::advance_stage thiscall() ret 0 (a virtual 0x28): o molde da WelPrmCup::advance_stage
#   (welsh_cup_advance_stage) com a linha 0x24a e o mata-mata na fase 10 (GreCup::create_knockout_stage 0x5e9850, mov ecx
#   -> thiscall, ret 0; ainda original, chamada pelo endereco). Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6
#   bytes).
# Igual no cm0102-1080.exe.
# Comparador: so a original (o mata-mata monta uma fase), conferindo a fase atual (compare_state_league_advance_stage).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_cup_advance_stage', 0x5e95c0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a fase seguinte da Copa da Grécia', 'the Greek Cup next stage',
        [('gre_cup_advance_stage', 'GreCup::advance_stage', '0x5e95c0', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Grécia passa à fase seguinte; ao chegar à fase 10 monta o mata-mata dos 22 classificados dos grupos; depois da última fase, a caixa de erro',
          'rule: the Greek Cup moves on to the next stage; on reaching stage 10 it builds the knockout of the 22 group qualifiers; past the last stage, the error box')])
CAMPOS = []
INTERESSA = []
