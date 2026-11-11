# A virada do futebol brasileiro e da J2 (bra_nat_first.cpp, jap_j2.cpp; agente C da frente das competições,
# 06/11/2026). Rodam uma vez cada na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), na virada da
# temporada:
# - 0x42b130 bra_nat_first_exchange_state_league_clubs (1.001 bytes): o acesso e o rebaixamento entre os doze
#   estaduais e as divisões de baixo de cada um;
# - 0x42b520 bra_nat_first_refresh_third_division_clubs (576 bytes): os 16 brasileiros sem divisão que entram na Série
#   C e os eliminados da primeira fase que saem dela;
# - 0x6651f0 JapJ2::start_new_season (397 bytes): a virada da J2 (o molde das viradas do Carioca e da MLS, cqr_2/cqs_2,
#   sem as fases além da liga).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x42b520 e 0x42b130, da virada da Série A (0x42a662, 0x42a669): mov ecx, esi; call; sem push nem add esp -> cdecl
#   sem argumentos (o ecx não é lido), ret 0;
# - 0x6651f0: o slot da virada (call [vtable + ...]; termina em ret) -> thiscall, ret 0;
# - 0x944e46 (malloc): push 0x320 / 0x280; call; add esp, 4 -> cdecl; 0x9452ca (free) -> cdecl;
# - 0x4afbd0 (comp_util_sort_and_shuffle_top_clubs): push 0x19 / 0x10; push n; push lista; call; add esp, 0xc -> cdecl;
# - 0x69ba50 (MANAGER_MANAGER::add_job_vacancy): push 0; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8;
# - 0x6826d0 (o registro i da tabela): push i; mov ecx, liga -> thiscall, ret 4;
# - 0x544820 (o clube pode ser dirigido): push clube; call; add esp, 4 -> cdecl, devolve al;
# - 0x600a50 (técnico humano decide pelo clube): push 0; push 0; push 1; push 1; push clube; call; add esp, 0x14 -> cdecl;
# - 0x69b790 (MANAGER_MANAGER::remove_club_manager): push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] ->
#   thiscall, ret 0x10;
# - 0x4cd030 (CONTRACT_MANAGER::trigger_relegation_clauses): push registro; push clube; mov ecx, [0xae1050] -> thiscall,
#   ret 8;
# - 0x687970 (League::free_table): push 0; mov ecx, esi -> thiscall, ret 4; 0x4a1c50 (Comp_Stats::archive): push 1; mov
#   ecx, [esi + 8] -> thiscall, ret 4; as virtuais +0x8c (regras) e +0x5c (previstas), 0x665030 (tabela), 0x6835c0
#   (jogos), 0x68aa80 (expectativas): mov ecx, esi -> thiscall, ret 0; 0x6827d0 (ordem): push 0 -> thiscall, ret 4;
#   0x79cee0 (renovar os árbitros): push +4; mov ecx, [0xb74340] -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original nas três (movem clubes, sorteiam o embaralhamento, mexem em técnicos e contratos, alocam
# tabelas), com o plano: no acesso, quantos de cada divisão sobem (min(vagas, os dela)) e os rebaixados na divisão de
# baixo; na Série C, os 16 que entram e os eliminados fora; na J2, o ano + 1 e a tabela nova.
# Determinismo: as listas vêm da ordem do banco e do embaralhamento da 0x4afbd0 (o sorteio do jogo).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_nat_first_exchange_state_league_clubs', 0x42b130, [0x81, 0xec, 0x70, 0x02, 0x00, 0x00]),
           ('bra_nat_first_refresh_third_division_clubs', 0x42b520, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('jap_j2_start_new_season', 0x6651f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada do futebol brasileiro e da J2: o acesso e o rebaixamento entre os estaduais e as divisões de baixo, os 16 que entram na Série C e a virada da J2',
        'the Brazilian and J2 season turnover: promotion and relegation between the state leagues and their lower divisions, the 16 clubs entering the third division and the J2 turnover',
        [('bra_nat_first_exchange_state_league_clubs', 'bra_nat_first_exchange_state_league_clubs', '0x42b130', 'bra_nat_first.cpp', 'cdecl',
          'Na virada do futebol brasileiro, o acesso e o rebaixamento entre os doze estaduais e as divisões de baixo de cada um: de cada divisão de baixo sobem tantos clubes quantas são as vagas de rebaixamento do estadual, escolhidos pela reputação com um embaralhamento entre os 75 melhores (até 200 clubes no todo); os que sobem e estão sem técnico abrem a vaga; os das últimas linhas de cada estadual descem para a divisão de baixo, perdem o técnico humano que não pode mais dirigi-los e acionam as cláusulas de rebaixamento dos contratos',
          'At the Brazilian season turnover, promotion and relegation between the twelve state leagues and their lower divisions: from each lower division as many clubs go up as the state league has relegation places, chosen by reputation with a shuffle among the top 75 (up to 200 clubs overall); promoted clubs without a manager open the job; the bottom clubs of each state league go down to the lower division, lose the human manager who can no longer run them and trigger the relegation clauses of the contracts'),
         ('bra_nat_first_refresh_third_division_clubs', 'bra_nat_first_refresh_third_division_clubs', '0x42b520', 'bra_nat_first.cpp', 'cdecl',
          'Na virada do futebol brasileiro, a renovação da Série C: se há pelo menos 16 clubes brasileiros sem divisão nacional (até 160, na ordem do banco), os 16 primeiros pela reputação, com um embaralhamento entre os 48 melhores, entram na Série C (os sem técnico abrem a vaga); os clubes marcados como eliminados (+0x37 = 3) na tabela da Série C e nas dos sete grupos saem da divisão nacional, perdem o técnico humano que não pode mais dirigi-los e acionam as cláusulas de rebaixamento',
          'At the Brazilian season turnover, the third division renewal: if there are at least 16 Brazilian clubs without a national division (up to 160, in database order), the top 16 by reputation, with a shuffle among the top 48, enter the third division (those without a manager open the job); the clubs marked as eliminated (+0x37 = 3) in the third division table and in the seven group tables leave the national divisions, lose the human manager who can no longer run them and trigger the relegation clauses'),
         ('jap_j2_start_new_season', 'JapJ2::start_new_season', '0x6651f0', 'jap_j2.cpp', 'thiscall, ret 0',
          'A virada da J2: a tabela e o calendário da temporada que acabou liberados, as estatísticas arquivadas, o ano seguinte, as regras, a tabela, os jogos e a ordem de novo (uma caixa e para se algum falhar), as posições previstas, as expectativas das diretorias e a renovação dos árbitros; a J2 é só de pontos corridos',
          'The J2 season turnover: the table and schedule of the finished season freed, the statistics archived, the next year, the rules, the table, the fixtures and the order again (a message box and stop if any fails), the predicted positions, the board expectations and the referee renewal; J2 is a plain round robin')])
CAMPOS = [('clube +0x60: a divisão regional (o registro do estadual ou da divisão de baixo); ponteiro; grava', 'club +0x60: the regional division (the state league or lower division record); pointer; writes'),
          ('clube +0x57: a divisão nacional (o registro da Série C, ou 0); ponteiro; grava', 'club +0x57: the national division (the third division record, or 0); pointer; writes'),
          ('liga +0xc1: as vagas de rebaixamento; valor com sinal; lê', 'league +0xc1: the relegation places; signed value; reads'),
          ('linha da tabela +0x37: [inferência] a situação do clube na fase (3: eliminado); valor; lê', 'table row +0x37: [inference] the club status in the stage (3: eliminated); value; reads'),
          ('liga +0x40: o ano da temporada; valor; grava + 1', 'league +0x40: the season year; value; writes + 1')]
INTERESSA = ['CMRTE']
