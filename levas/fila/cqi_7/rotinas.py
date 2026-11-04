# As regras das três divisões nacionais do Brasil (frente das competições, 03/11/2026, agente C: ligas de fora da
# Europa). O mesmo slot (36, +0x8c da vtable, `set_up_rules`) em três ligas: cada uma grava os bytes de regra do objeto
# da liga e pede o calendário da fase principal ao slot 16 (+0x3c, `create_schedule`).
# - 0x42a5a0 BraNatFirst::set_up_rules (Série A, 151 bytes), vtable 0x967d98; calendário 0x429b50.
# - 0x42cb70 BraNatSecond::set_up_rules (Série B, 156 bytes), vtable 0x967e4c; calendário 0x42c140.
# - 0x42de50 BraNatThird::set_up_rules (Série C, 154 bytes), vtable 0x967f00; calendário 0x42d620.
# Rodam 2 vezes cada na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): a montagem da carreira e
# [inferência] a virada da temporada de ano civil.
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma, conferido byte a byte).
# Convenções conferidas:
#   as três: thiscall (this em ecx: `mov esi, ecx`), ret 0, devolvem 1; chamadas pelo slot 36 da vtable.
#   o calendário (slot 16, chamada indireta `call [reg+0x3c]`): thiscall (`mov ecx, esi` antes do call), 4 argumentos
#   empurrados (0, &+0x3a, &+0xa9, -1: na ordem Rust -1, &+0xa9, &+0x3a, 0), ret 0x10 nos três alvos (0x429b50,
#   0x42c140, 0x42d620).
# Nenhuma chamada por endereço. Globais lidas: as chaves fixas das divisões (0x9cf9ec Série A, 0x9cf9e0 Série B,
# 0x9cf9ac Série C; docs/chaves_uso.csv).
# Comparadores: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca as rodadas), sobre o objeto
# da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba); os três calendários não gravam no objeto
# (nenhum `mov [edi+..]` neles). Devolução comparada.
# Códigos de desempate (lidos na 0x6834f0, a comparação da 0x6827d0, tabela de saltos 0x6835a4): 1 saldo, 2 gols pró,
# 3 confronto direto (a 0x683410 com a 0x682b30, quando é o primeiro), 4 gols marcados fora (+0x31 da linha),
# 5 vitórias.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqi7_regras_da_serie_a', 0x42a5a0, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('cqi7_regras_da_serie_b', 0x42cb70, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('cqi7_regras_da_serie_c', 0x42de50, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras das três divisões nacionais do Brasil (Séries A, B e C)',
        'the rules of the three Brazilian national divisions (Series A, B and C)',
        [('cqi7_regras_da_serie_a', 'BraNatFirst::set_up_rules', '0x42a5a0', 'bra_nat_first.cpp', 'thiscall',
          'Série A: turno único; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem liga acima, '
          'cai para a Série B; 4 vagas de cima que, pela palavra de regras 8, levam à fase seguinte (os play-offs; sem '
          'acesso e sem faixa de campeão: o campeão sai do mata-mata) [inferência: a fase seguinte é a do slot 11]; '
          '4 rebaixados; 7 reservas e 3 substituições; numeração fixa do elenco; partidas de categoria 2 (liga). Depois, '
          'o calendário da fase principal',
          'Series A: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no league above, '
          'drops to Series B; 4 top places that, by the rule word 8, lead to the next stage (the play-offs; no promotion '
          'and no champion band: the champion comes from the knockout) [inference: the next stage is the slot 11 one]; '
          '4 relegated; 7 substitutes on the bench and 3 substitutions; fixed squad numbers; category 2 (league) matches. '
          'Then the main stage schedule'),
         ('cqi7_regras_da_serie_b', 'BraNatSecond::set_up_rules', '0x42cb70', 'bra_nat_second.cpp', 'thiscall',
          'Série B: turno único; vitória 3, empate 1; confronto direto e saldo; sobe para a Série A e cai para a Série C; '
          '4 sobem (palavra de regras 2: as vagas de cima são de acesso) e 4 caem; 7 reservas, 3 substituições, '
          'numeração fixa. Depois, o calendário',
          'Series B: single round robin; win 3, draw 1; head-to-head and goal difference; promotes to Series A and '
          'relegates to Series C; 4 go up (rule word 2: the top places are promotion) and 4 go down; 7 substitutes, '
          '3 substitutions, fixed numbers. Then the schedule'),
         ('cqi7_regras_da_serie_c', 'BraNatThird::set_up_rules', '0x42de50', 'bra_nat_third.cpp', 'thiscall',
          'Série C: dois turnos (grupos de ida e volta [inferência: os grupos de 7 do calendário 0x42d620]); vitória 3, '
          'empate 1; desempate pelo saldo, gols pró e gols marcados fora; sobe para a Série B, sem liga abaixo; 2 vagas '
          'de cima que levam à fase seguinte (palavra 8) e 2 de baixo [inferência: sem efeito, não há liga abaixo]; '
          '7 reservas, 3 substituições, numeração fixa. Depois, o calendário',
          'Series C: double round robin (home-and-away groups [inference: the groups of 7 of the 0x42d620 schedule]); '
          'win 3, draw 1; tie-break by goal difference, goals scored and away goals; promotes to Series B, no league '
          'below; 2 top places leading to the next stage (rule word 8) and 2 bottom ones [inference: no effect, there is '
          'no league below]; 7 substitutes, 3 substitutions, fixed numbers. Then the schedule')])
CAMPOS = [('liga +0x3c: turnos; valor (palavra); grava (as três)', 'league +0x3c: rounds robin; value (word); writes (all three)'),
          ('liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava', 'league +0xc2 / +0xc3: points per win / per draw; value; writes'),
          ('liga +0xc4: categoria copiada em cada partida (2 = liga, vai para o +0x3a da partida); valor; grava',
           'league +0xc4: category copied to each match (2 = league, goes to the match +0x3a); value; writes'),
          ('liga +0x42: tipo da competição (1 = clube nacional); valor; grava', 'league +0x42: competition type (1 = national club); value; writes'),
          ('liga +0xc5..+0xc7: desempates (1 saldo, 2 gols pró, 3 confronto direto, 4 gols fora, 5 vitórias); valor; grava',
           'league +0xc5..+0xc7: tie-breaks (1 goal difference, 2 goals for, 3 head-to-head, 4 away goals, 5 wins); value; writes'),
          ('liga +0xbe / +0xbf / +0xc0 / +0xc1: vagas de cima, play-off de cima, play-off de baixo, rebaixados; valor; grava',
           'league +0xbe / +0xbf / +0xc0 / +0xc1: top places, upper play-off, lower play-off, relegated; value; writes'),
          ('liga +0x1c / +0x20: liga de cima / de baixo; id (chave fixa, -1 = nenhuma); grava', 'league +0x1c / +0x20: league above / below; id (fixed key, -1 = none); writes'),
          ('liga +0xd9: palavra de bits de regra (2 acesso, 8 fase seguinte); valor (palavra); grava', 'league +0xd9: rule bit word (2 promotion, 8 next stage); value (word); writes'),
          ('liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava', 'league +0x49 / +0x4a: bench substitutes / substitutions; value; writes'),
          ('liga +0x52: marcas da fase (2 = numeração fixa do elenco); valor; grava', 'league +0x52: stage flags (2 = fixed squad numbers); value; writes'),
          ('liga +0xa9 / +0x3a: saídas do calendário (rodadas, código do estágio); valor (palavra); grava (pelo slot 16)',
           'league +0xa9 / +0x3a: schedule outputs (rounds, stage code); value (word); writes (through slot 16)'),
          ('liga +0xba: calendário (rodadas de 0x41 bytes); ponteiro; grava', 'league +0xba: schedule (0x41-byte rounds); pointer; writes'),
          ('[0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: chaves das Séries A, B e C; id; lê', '[0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: Series A, B and C keys; id; reads')]
INTERESSA = ['CMRTE']
