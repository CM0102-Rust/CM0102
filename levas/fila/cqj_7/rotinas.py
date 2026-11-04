# As regras do Goiano e das duas divisões da J-League (frente das competições, 03/11/2026, agente C).
# O slot 36 (+0x8c, `set_up_rules`): grava os bytes de regra e pede o calendário ao slot 16 (+0x3c).
# - 0x433640 BraRegGoias::set_up_rules (161 bytes), vtable 0x9681d0; calendário 0x432e50.
# - 0x663620 JapJ1::set_up_rules (176 bytes), vtable 0x96ca40; calendário 0x662e30.
# - 0x665140 JapJ2::set_up_rules (161 bytes), vtable 0x96caf4; calendário 0x664780.
# Rodam 2 vezes cada na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt).
# (A 0x426230 BraChampCup::set_up_rules e a 0x427ec0 BraCup::set_up_rounds, escolhidas no começo, são de cups\: ficaram
# com o agente D; a 0x426230 está na cqi_12.)
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   as três: thiscall (`mov esi, ecx`), ret 0, devolvem 1; chamadas pelo slot 36 da vtable.
#   o calendário (slot 16, `call [edx+0x3c]`): thiscall (`mov ecx, esi`), 4 argumentos (empurrados 0, &+0x3a, &+0xa9,
#   -1), ret 0x10 nos três alvos.
# Nenhuma chamada por endereço. Globais lidas: as chaves fixas da J1 (0x9cf65c) e da J2 (0x9cf768) (docs/chaves_uso.csv).
# Comparadores: lado a lado com a cópia em ensaio (as regras sem o calendário), sobre o objeto da liga (0xee) menos as
# saídas do calendário (+0x3a, +0xa9, +0xba); os três calendários não gravam no objeto. Devolução comparada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqj7_regras_do_goiano', 0x433640, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('cqj7_regras_da_j1', 0x663620, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('cqj7_regras_da_j2', 0x665140, [0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('as regras do Goiano e das duas divisões da J-League',
        'the rules of the Goias state league and the two J-League divisions',
        [('cqj7_regras_do_goiano', 'BraRegGoias::set_up_rules', '0x433640', 'bra_reg_goias.cpp', 'thiscall',
          'Goiano: turno único; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; 8 vagas de cima '
          '(palavra de regras 8: levam à fase seguinte, o mata-mata [inferência]); 4 rebaixados, sem liga acima nem '
          'abaixo; 7 reservas, 3 substituições; partidas de categoria 2. Depois, o calendário',
          'Goias: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; 8 top places '
          '(rule word 8: they lead to the next stage, the knockout [inference]); 4 relegated, no league above or below; '
          '7 bench substitutes, 3 substitutions; category 2 matches. Then the schedule'),
         ('cqj7_regras_da_j1', 'JapJ1::set_up_rules', '0x663620', 'jap_j1.cpp', 'thiscall',
          'J1: dois turnos (os dois estágios do ano, criados pelo add_result 0x663940 [inferência]); vitória 3, empate 1; '
          'desempate pelo saldo, gols pró e gols marcados fora; nenhuma vaga de cima na tabela geral (palavra 8, sem '
          'faixa de campeão: o campeão sai da final entre os vencedores dos estágios [inferência]); 2 caem para a J2; '
          'sem liga acima; 5 reservas e 3 substituições; numeração fixa do elenco; a palavra +0xab = 0x82 copiada em '
          'cada partida. Depois, o calendário',
          'J1: double round robin (the two stages of the year, created by add_result 0x663940 [inference]); win 3, draw 1; '
          'tie-break by goal difference, goals scored and away goals; no top places in the overall table (rule word 8, '
          'no champion band: the champion comes from the final between the stage winners [inference]); 2 go down to J2; '
          'no league above; 5 bench substitutes and 3 substitutions; fixed squad numbers; the word +0xab = 0x82 copied to '
          'each match. Then the schedule'),
         ('cqj7_regras_da_j2', 'JapJ2::set_up_rules', '0x665140', 'jap_j2.cpp', 'thiscall',
          'J2: quatro turnos; vitória 3, empate 1; desempate pelo saldo, gols pró e gols fora; 2 sobem para a J1 (a '
          'palavra de regras fica a da base, 3: campeão e acesso), nenhum rebaixado, sem liga abaixo; 5 reservas, '
          '3 substituições. Depois, o calendário',
          'J2: four round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; 2 go up to J1 '
          '(the rule word stays the base one, 3: champion and promotion), no relegation, no league below; 5 bench '
          'substitutes, 3 substitutions. Then the schedule')])
CAMPOS = [('liga +0x3c: turnos; valor (palavra); grava', 'league +0x3c: round robins; value (word); writes'),
          ('liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava', 'league +0xc2 / +0xc3: points per win / per draw; value; writes'),
          ('liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava', 'league +0xc4: category copied to each match (2 = league); value; writes'),
          ('liga +0x42: tipo da competição (1 = clube nacional); valor; grava', 'league +0x42: competition type (1 = national club); value; writes'),
          ('liga +0xc5..+0xc8: desempates (1 saldo, 2 gols pró, 3 confronto direto, 4 gols fora); valor; grava', 'league +0xc5..+0xc8: tie-breaks (1 goal difference, 2 goals for, 3 head-to-head, 4 away goals); value; writes'),
          ('liga +0xbe..+0xc1: vagas de cima, play-offs de cima e de baixo, rebaixados; valor; grava', 'league +0xbe..+0xc1: top places, upper and lower play-offs, relegated; value; writes'),
          ('liga +0x1c / +0x20: liga de cima / de baixo (chave fixa, -1 = nenhuma); id; grava', 'league +0x1c / +0x20: league above / below (fixed key, -1 = none); id; writes'),
          ('liga +0xd9: palavra de bits de regra (8 = fase seguinte); valor (palavra); grava (Goiano e J1)', 'league +0xd9: rule bit word (8 = next stage); value (word); writes (Goias and J1)'),
          ('liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava', 'league +0x49 / +0x4a: bench substitutes / substitutions; value; writes'),
          ('liga +0x52: marcas da fase (2 = numeração fixa); valor; grava (J1)', 'league +0x52: stage flags (2 = fixed numbers); value; writes (J1)'),
          ('liga +0xab / +0xdb: palavras copiadas em cada partida (+0x38 e +0x36 da partida) [inferência: códigos de estágio]; valor; grava (0x82 e 0 na J-League, 0 e 0 no Goiano)',
           'league +0xab / +0xdb: words copied to each match (match +0x38 and +0x36) [inference: stage codes]; value; writes (0x82 and 0 in the J-League, 0 and 0 in Goias)'),
          ('liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (pelo slot 16)',
           'league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (through slot 16)'),
          ('[0x9cf65c] / [0x9cf768]: chaves da J1 e da J2; id; lê', '[0x9cf65c] / [0x9cf768]: J1 and J2 keys; id; reads')]
INTERESSA = ['CMRTE']
