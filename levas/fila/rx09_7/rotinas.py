# A Segunda Divisao sueca na criacao do jogo, as regras e as tabelas de rodadas (swe_second.cpp; leitor rx09, rodada 6,
# 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01 (a temporada sueca de
# 2001 termina em novembro e a de 2002 comeca em abril).
# - 0x8916f0 SweSecond::SweSecond thiscall(ano, registro) ret 8: o molde do rx09_1 (construct_grouped_league) com
#   vtable 0x9702a0, disciplina 0x1c, +0x2c = 7, malloc 0x1c; as regras escritas no proprio construtor (as mesmas do
#   set_up_rules, com a virtual +0x3c em +0xba, sem caixa); tabela 0x8921b0 (0x50), sort (0x57), Comp_Stats (0x5f),
#   fixtures (0x68); create_section_stage(k) 0x892670 para k = 0..4 e set_predicted_positions 0x893470.
# - 0x892380 SweSecond::set_up_rules thiscall() ret 0 -> 1: as escritas conferidas (sem +0x49/+0x4a) e +0xba.
# - 0x891a60 SweSecond::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10: fase < 5: 22 rodadas,
#   malloc 0x596 (caixa 0xb9), o extra pela tabela de saltos 0x892198 so para -1..4 (fase + 0x46d); 5: uma rodada de
#   copa, extra 0x438, malloc 0x68 (0x12b); 6: duas rodadas, extra 0x452, malloc 0xd0 (0x153); depois 0. Datas pela
#   emulacao (cx14\emula.py) e as escritas das rodadas de copa conferidas uma a uma.
# Prologos: push -1; push imm32 (7 bytes); push ebx; push esi; mov esi, ecx; mov eax, 2 (9 bytes); mov al, [esp+4];
# sub esp, 0x200 (10 bytes). Iguais no cm0102-1080.exe.
# Comparadores: o construtor so a original; as regras pelo ensaio; as tabelas lado a lado.
# Depende do rx09_1 (construct_grouped_league, GroupedLeagueCtor, league_round_table, cup_round_table,
# compare_league_or_cup_schedule).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('construct_swe_second', 0x8916f0, [0x6a, 0xff, 0x68, 0xb9, 0x31, 0x96, 0x00]),
           ('swe_second_set_up_rules', 0x892380, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('swe_second_create_schedule', 0x891a60, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Segunda Divisão sueca na criação do jogo, as regras e as tabelas de rodadas',
        'the Swedish Second Division at game creation, its rules and its round tables',
        [('construct_swe_second', 'SweSecond::SweSecond', '0x8916f0', 'swe_second.cpp', 'thiscall, ret 8',
          'regra: a Segunda sueca é criada com sete fases além da seção Norte (as outras cinco seções, o acesso e a repescagem do rebaixamento); sem estado salvo, as regras, a tabela da seção Norte, a ordem, as estatísticas, os jogos, as outras cinco seções e as posições previstas',
          'rule: the Swedish Second Division is built with seven stages besides the North section (the other five sections, the promotion play-off and the relegation play-off); with no saved state, the rules, the North table, the order, the statistics, the fixtures, the other five sections and the predicted positions'),
         ('swe_second_set_up_rules', 'SweSecond::set_up_rules', '0x892380', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: dois turnos, 3 pontos por vitória e 1 por empate; desempate pelo saldo e pelos gols marcados; o campeão de cada seção vai ao acesso, o vice à repescagem do rebaixamento, os dois últimos caem; a Primeira acima',
          'rule: two legs, 3 points a win and 1 a draw; tie-breaks goal difference, goals for; each section winner goes to the promotion play-off, the runner-up to the relegation play-off, the last two down; the First Division above'),
         ('swe_second_create_schedule', 'SweSecond::create_schedule', '0x891a60', 'swe_second.cpp', 'thiscall, ret 0x10',
          'regra: as seções jogam 22 rodadas aos domingos de 6 de abril a 8 de novembro, com a pausa de 7 de junho a 19 de julho; o acesso, ida e volta a partir de 15 de novembro (sorteio em 9 de novembro); a repescagem do rebaixamento em duas rodadas de ida e volta, 15 e 29 de novembro, os vices entrando na segunda',
          'rule: the sections play 22 rounds on Sundays from 6 April to 8 November, with the break from 7 June to 19 July; the promotion play-off, two legs from 15 November (drawn on 9 November); the relegation play-off in two two-leg rounds, 15 and 29 November, the runners-up coming in at the second')])
CAMPOS = []
INTERESSA = []
