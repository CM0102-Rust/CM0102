# Os construtores das quatro divisões inglesas (Agente B da frente das competições, 04/11/2026). Rodam uma vez cada na
# criação da carreira de 8 ligas (docs/cobertura_competicoes.txt: 0x573380 1, os outros 1 ou 2 com a virada):
# - 0x573380 (EngPrm::EngPrm, 545 bytes), 0x571680 (EngFirst::EngFirst, 540), 0x5754a0 (EngSecond::EngSecond, 540) e
#   0x576dd0 (EngThird::EngThird, 558), todos com quadro SEH só para o operator new das estatísticas.
# Iguais no cm0102-1080.exe (inteiros). Os quatro diferem só na vtable, no estádio mínimo (+0xe2/+0xe4: a Premier
# 10.000/10.000, a Third 6.000/1.000, as outras nada), nas fases (+0x2c: 0 na Premier; 1 e o vetor [+0xc] = malloc de 4
# nas outras), nas regras e na tabela da divisão, no fonte e nas linhas das caixas (conferido pela desmontagem).
# Convenções conferidas nos pontos de chamada da original:
# - 0x682200 (League::League): mov ecx, esi -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x944e46 (malloc): push 4; add esp, 4 -> cdecl;
# - as regras (0x574b70, 0x572d90, 0x576780, 0x5780c0) e as tabelas (0x574a20, 0x572bf0, 0x576630, 0x577f70): mov ecx,
#   esi -> thiscall, ret 0 (a 275ª leva);
# - 0x6835c0 (League::create_fixtures): mov ecx, esi -> thiscall, ret 0;
# - 0x6827d0 (League::sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x944cf1 (operator new): push 0x5ce; add esp, 4 -> cdecl;
# - 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov ecx, novo -> thiscall, ret 4;
# - 0x68a850 (League::set_predicted_positions): mov ecx, esi -> thiscall, ret 0;
# - a caixa: aviso_em.
# Comparador: só a original (aloca), conferindo a devolução, a vtable, o ano, o registro, a disciplina, o estádio e, na
# liga nova, as fases e as estatísticas.
# Determinismo: só constantes e os argumentos.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construir_premier', 0x573380, [0x6a, 0xff, 0x68, 0xc9, 0x83, 0x95, 0x00]),
           ('construir_first', 0x571680, [0x6a, 0xff, 0x68, 0x59, 0x83, 0x95, 0x00]),
           ('construir_second', 0x5754a0, [0x6a, 0xff, 0x68, 0x19, 0x84, 0x95, 0x00]),
           ('construir_third', 0x576dd0, [0x6a, 0xff, 0x68, 0x89, 0x84, 0x95, 0x00])]
DOCS = ('os construtores das quatro divisões inglesas', 'the constructors of the four English divisions',
 [('construir_premier', 'EngPrm::EngPrm', '0x573380', 'eng_prm.cpp', 'thiscall, ret 8',
   'a Premier: a regra de disciplina 9 (a das ligas inglesas), estádio mínimo de 10.000 lugares todos sentados, só a '
   'liga (sem play-off); sem estado salvo, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições '
   'previstas',
   'the Premiership: discipline rule 9 (the English leagues one), minimum ground of 10,000 all seated, only the league '
   '(no play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted '
   'positions'),
  ('construir_first', 'EngFirst::EngFirst', '0x571680', 'eng_first.cpp', 'thiscall, ret 8',
   'a First Division: disciplina 9, sem estádio mínimo, a liga e 1 fase (o play-off de acesso)',
   'the First Division: discipline 9, no minimum ground, the league and 1 stage (the promotion play-off)'),
  ('construir_second', 'EngSecond::EngSecond', '0x5754a0', 'eng_second.cpp', 'thiscall, ret 8',
   'a Second Division: disciplina 9, sem estádio mínimo, a liga e o play-off',
   'the Second Division: discipline 9, no minimum ground, the league and the play-off'),
  ('construir_third', 'EngThird::EngThird', '0x576dd0', 'eng_third.cpp', 'thiscall, ret 8',
   'a Third Division: disciplina 9, estádio mínimo de 6.000 lugares com 1.000 sentados, a liga e o play-off',
   'the Third Division: discipline 9, minimum ground of 6,000 with 1,000 seated, the league and the play-off')])
CAMPOS = [('liga +0 vtable, +4 registro da competição, +0x40 ano, +0x50 regra de disciplina, +0xe2/+0xe4 estádio mínimo '
           '(capacidade, sentados); valor e ponteiro; grava (os quatro)',
           'league +0 vtable, +4 competition record, +0x40 year, +0x50 discipline rule, +0xe2/+0xe4 minimum ground '
           '(capacity, seated); value and pointer; writes'),
          ('liga +0x44 fase (0xff = a liga), +0x30 fase atual, +0x2c fases, +0xc vetor das fases, +8 Comp_Stats; valor e '
           'ponteiro; grava (os quatro)',
           'league +0x44 stage (0xff = the league), +0x30 current stage, +0x2c stages, +0xc stage vector, +8 Comp_Stats; '
           'value and pointer; writes')]
INTERESSA = ['CMRTE']
