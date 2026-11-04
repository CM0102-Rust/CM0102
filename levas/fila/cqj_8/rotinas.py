# As regras da NSL australiana e da MLS, e o histórico da temporada dos estaduais e das Séries A e B (frente das
# competições, 03/11/2026, agente C). Duas rotinas de regra (slot 36) e duas passageiras pequenas do slot 13:
# - 0x411ce0 AusNsl::set_up_rules (164 bytes), vtable 0x967768; calendário 0x4111a0. 2 chamadas.
# - 0x908660 UsaMls::set_up_rules (123 bytes), vtable 0x970844; sem calendário (os jogos da MLS são montados à mão pela
#   0x9086e0). 1 chamada.
# - 0x432970 BraRegGaucho::record_season_honours (15 bytes), o slot 13 (+0x30) de nove estaduais (vtables 0x967fb4,
#   0x968068, 0x96811c, 0x9681d0, 0x968338, 0x9683ec, 0x968554, 0x9686bc, 0x968770). 9 chamadas.
# - 0x408eb0 ArgSecond::record_season_honours (10 bytes), o slot 13 da segunda divisão argentina (0x9673d8) e das
#   Séries A (0x967d98) e B (0x967e4c). 2 chamadas.
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   0x411ce0, 0x908660: thiscall (`mov esi, ecx` / campos em [ecx+..]), ret 0, devolvem 1; chamadas pelo slot 36.
#   0x432970, 0x408eb0: thiscall (lêem [ecx+0xc]), ret 0 (salto final), chamadas pelo slot 13; devolvem o eax do
#   chamado.
#   o calendário da NSL (slot 16, `call [edx+0x3c]`): thiscall (`mov ecx, esi`), 4 argumentos (empurrados 0, &+0x3a,
#   &+0xa9, -1), ret 0x10 na 0x4111a0.
#   o histórico do estágio (slot 13 do estágio [[this+0xc]], `call [edx+0x30]` / `jmp [edx+0x30]`): thiscall (ecx = o
#   estágio), sem argumentos.
#   0x42bba0 bra_nat_first_fill_champions_cup_entrants (por endereço, salto final da 0x432970): cdecl sem argumentos
#   (`sub esp, 0xa0`, não lê argumento nem ecx; ret 0).
# Comparadores: NSL lado a lado com a cópia em ensaio (as regras sem o calendário) sobre o objeto menos as saídas do
# calendário (+0x3a, +0xa9, +0xba; a 0x4111a0 não grava no objeto); MLS lado a lado sobre o objeto (0xee); os dois
# históricos, só a original (gravam o histórico e alocam o registro; não podem rodar duas vezes).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqj8_regras_da_nsl', 0x411ce0, [0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('cqj8_regras_da_mls', 0x908660, [0xb2, 0x02, 0xb8, 0x01, 0x00, 0x00, 0x00]),
           ('cqj8_historico_do_estadual', 0x432970, [0x8b, 0x41, 0x0c, 0x8b, 0x08]),
           ('cqj8_historico_do_primeiro_estagio', 0x408eb0, [0x8b, 0x41, 0x0c, 0x8b, 0x08])]
DOCS = ('as regras da NSL australiana e da MLS, e o histórico da temporada dos estaduais e das Séries A e B',
        'the rules of the Australian NSL and the MLS, and the season honours of the state leagues and Series A and B',
        [('cqj8_regras_da_nsl', 'AusNsl::set_up_rules', '0x411ce0', 'aus_nsl.cpp', 'thiscall',
          'NSL: três turnos; vitória 3, empate 1; desempate pelo saldo, gols pró e gols marcados fora; os 6 primeiros vão '
          'para a fase final (palavra de regras 0x28: 0x8 fase seguinte, sem faixa de campeão, e 0x20, os sem chance '
          'marcados como eliminados); ninguém sobe nem cai; 4 reservas e 3 substituições; numeração fixa do elenco. '
          'Depois, o calendário',
          'NSL: three round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; the top 6 '
          'go to the finals series (rule word 0x28: 0x8 next stage, no champion band, and 0x20, clubs without a chance '
          'marked eliminated); nobody goes up or down; 4 bench substitutes and 3 substitutions; fixed squad numbers. '
          'Then the schedule'),
         ('cqj8_regras_da_mls', 'UsaMls::set_up_rules', '0x908660', 'usa_mls.cpp', 'thiscall',
          'MLS: 32 jogos por time (+0xa7; quatro turnos na conta de +0x3c); vitória 3, empate 1; desempate pelo saldo, '
          'gols pró e confronto direto; 1 vaga de cima na tabela geral (palavra 0x48: 0x8 fase seguinte, os play-offs; '
          '0x40 rodadas em ordem fixa); ninguém sobe nem cai; a palavra +0xab = 0xc2 copiada em cada partida. Sem '
          'calendário aqui: os jogos são montados à mão (0x9086e0)',
          'MLS: 32 games per team (+0xa7; four round robins in +0x3c); win 3, draw 1; tie-break by goal difference, '
          'goals scored and head-to-head; 1 top place in the overall table (rule word 0x48: 0x8 next stage, the '
          'play-offs; 0x40 fixed round order); nobody goes up or down; the word +0xab = 0xc2 copied to each match. No '
          'schedule here: the games are built by hand (0x9086e0)'),
         ('cqj8_historico_do_estadual', 'BraRegGaucho::record_season_honours', '0x432970', 'bra_reg_gaucho.cpp', 'thiscall',
          'fim de um estadual (nove deles): grava o histórico do primeiro estágio (a tabela) e chama a 0x42bba0, que, '
          'quando os doze estaduais terminaram, põe os campeões na Copa dos Campeões',
          'end of a state league (nine of them): records the honours of the first stage (the table) and calls 0x42bba0, '
          'which, once the twelve state leagues are over, puts the champions into the Champions Cup'),
         ('cqj8_historico_do_primeiro_estagio', 'ArgSecond::record_season_honours', '0x408eb0', 'arg_second.cpp', 'thiscall',
          'fim da segunda divisão argentina e das Séries A e B: o histórico vem do primeiro estágio, não da tabela geral',
          'end of the Argentine second division and Series A and B: the honours come from the first stage, not the '
          'overall table')])
CAMPOS = [('liga +0x3c: turnos; valor (palavra); grava', 'league +0x3c: round robins; value (word); writes'),
          ('liga +0xa7: jogos por time na temporada; valor (palavra); grava (MLS: 32)', 'league +0xa7: games per team in the season; value (word); writes (MLS: 32)'),
          ('liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava', 'league +0xc2 / +0xc3: points per win / per draw; value; writes'),
          ('liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava', 'league +0xc4: category copied to each match (2 = league); value; writes'),
          ('liga +0x42: tipo da competição (1 = clube nacional); valor; grava', 'league +0x42: competition type (1 = national club); value; writes'),
          ('liga +0xc5..+0xc8: desempates (1 saldo, 2 gols pró, 3 confronto direto, 4 gols fora); valor; grava', 'league +0xc5..+0xc8: tie-breaks (1 goal difference, 2 goals for, 3 head-to-head, 4 away goals); value; writes'),
          ('liga +0xbe..+0xc1: vagas de cima, play-offs, rebaixados; valor; grava', 'league +0xbe..+0xc1: top places, play-offs, relegated; value; writes'),
          ('liga +0x1c / +0x20: liga de cima / de baixo (-1 = nenhuma); id; grava', 'league +0x1c / +0x20: league above / below (-1 = none); id; writes'),
          ('liga +0xd9: palavra de bits de regra (0x8 fase seguinte, 0x20 eliminados, 0x40 ordem fixa); valor (palavra); grava', 'league +0xd9: rule bit word (0x8 next stage, 0x20 eliminated, 0x40 fixed order); value (word); writes'),
          ('liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava (NSL)', 'league +0x49 / +0x4a: bench substitutes / substitutions; value; writes (NSL)'),
          ('liga +0x52: marcas da fase (2 = numeração fixa); valor; grava (NSL)', 'league +0x52: stage flags (2 = fixed numbers); value; writes (NSL)'),
          ('liga +0xab: palavra copiada em cada partida (+0x38 da partida) [inferência: código de estágio]; valor; grava (0 na NSL, 0xc2 na MLS)',
           'league +0xab: word copied to each match (match +0x38) [inference: stage code]; value; writes (0 in the NSL, 0xc2 in the MLS)'),
          ('liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (NSL, pelo slot 16)',
           'league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (NSL, through slot 16)'),
          ('liga +0xc: vetor de estágios; ponteiro; lê o primeiro (os dois históricos)', 'league +0xc: stage vector; pointer; reads the first (both honours routines)')]
INTERESSA = ['CMRTE']
