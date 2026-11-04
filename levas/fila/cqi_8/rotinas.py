# As regras de seis estaduais brasileiros por três rotinas (frente das competições, 03/11/2026, agente C).
# O slot 36 (+0x8c, `set_up_rules`): grava os bytes de regra e pede o calendário ao slot 16 (+0x3c). Uma rotina
# serve a mais de uma liga (a mesma entrada em várias vtables):
# - 0x430ff0 BraRegCentral::set_up_rules (160 bytes): Centro-Oeste (vtable 0x968068; calendário 0x4307b0).
# - 0x4322d0 BraRegGaucho::set_up_rules (155 bytes): Gaúcho (0x96811c; 0x431ac0), Nordeste (0x9683ec; 0x4366f0) e
#   Paulista (0x968770; 0x43c5a0).
# - 0x43bc70 BraRegSanta::set_up_rules (158 bytes): Baiano (0x967fb4; 0x42f5d0) e Catarinense (0x9686bc; 0x43b430).
# Rodam 2, 6 e 4 vezes na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): 2 por liga.
# As três gravam o mesmo conjunto de campos; mudam os turnos (+0x3c), as vagas de cima (+0xbe) e os rebaixados
# (+0xc1): o porte é um ajudante comum (cqi8_regras) com os parâmetros de cada uma, conferidos byte a byte.
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   as três: thiscall (`mov esi, ecx`), ret 0, devolvem 1; chamadas pelo slot 36 da vtable.
#   o calendário (slot 16, `call [edi+0x3c]`, edi = vtable lida antes): thiscall (`mov ecx, esi`), 4 argumentos
#   (empurrados 0, &+0x3a, &+0xa9, -1), ret 0x10 nos seis alvos.
# Nenhuma chamada por endereço; nenhuma global lida.
# Comparadores: lado a lado com a cópia em ensaio (as regras sem o calendário), sobre o objeto da liga (0xee) menos as
# saídas do calendário (+0x3a, +0xa9, +0xba); os seis calendários não gravam no objeto. Devolução comparada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqi8_regras_do_centro_oeste', 0x430ff0, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('cqi8_regras_do_gaucho', 0x4322d0, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('cqi8_regras_do_catarinense', 0x43bc70, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00])]
_COMUM_PT = ('vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem '
             'liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 '
             '(as vagas de cima levam à fase seguinte; sem faixa de campeão: o campeão sai da fase final); 7 reservas e '
             '3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal')
_COMUM_EN = ('win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or '
             'below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next '
             'stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3 substitutions; '
             'category 2 (league) matches. Then the main stage schedule')
DOCS = ('as regras dos estaduais do Centro-Oeste, Gaúcho, Nordeste, Paulista, Baiano e Catarinense',
        'the rules of the Centre-West, Gaucho, Northeast, Paulista, Bahia and Santa Catarina state leagues',
        [('cqi8_regras_do_centro_oeste', 'BraRegCentral::set_up_rules', '0x430ff0', 'bra_reg_central.cpp', 'thiscall',
          'Centro-Oeste: turno único; 8 vagas de cima (o mata-mata de 8 [inferência: a fase do slot 11]); 3 rebaixados; ' + _COMUM_PT,
          'Centre-West: single round robin; 8 top places (the 8-club knockout [inference: the slot 11 stage]); 3 relegated; ' + _COMUM_EN),
         ('cqi8_regras_do_gaucho', 'BraRegGaucho::set_up_rules', '0x4322d0', 'bra_reg_gaucho.cpp', 'thiscall',
          'Gaúcho, Copa do Nordeste e Paulista (a mesma rotina nas três vtables): turno único; 4 vagas de cima (o mata-mata '
          'de 4); 2 rebaixados; ' + _COMUM_PT,
          'Gaucho, Northeast Cup and Paulista (the same routine in the three vtables): single round robin; 4 top places '
          '(the 4-club knockout); 2 relegated; ' + _COMUM_EN),
         ('cqi8_regras_do_catarinense', 'BraRegSanta::set_up_rules', '0x43bc70', 'bra_reg_santa.cpp', 'thiscall',
          'Baiano e Catarinense (a mesma rotina nas duas vtables): dois turnos; 4 vagas de cima (a semifinal de 4); '
          '1 rebaixado; ' + _COMUM_PT,
          'Bahia and Santa Catarina (the same routine in the two vtables): double round robin; 4 top places (the 4-club '
          'semi-final); 1 relegated; ' + _COMUM_EN)])
CAMPOS = [('liga +0x3c: turnos; valor (palavra); grava', 'league +0x3c: round robins; value (word); writes'),
          ('liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava', 'league +0xc2 / +0xc3: points per win / per draw; value; writes'),
          ('liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava', 'league +0xc4: category copied to each match (2 = league); value; writes'),
          ('liga +0x42: tipo da competição (1 = clube nacional); valor; grava', 'league +0x42: competition type (1 = national club); value; writes'),
          ('liga +0xc5..+0xc7: desempates (3 confronto direto, 1 saldo, 0 nenhum); valor; grava', 'league +0xc5..+0xc7: tie-breaks (3 head-to-head, 1 goal difference, 0 none); value; writes'),
          ('liga +0xbe..+0xc1: vagas de cima, play-offs de cima e de baixo (0), rebaixados; valor; grava', 'league +0xbe..+0xc1: top places, upper and lower play-offs (0), relegated; value; writes'),
          ('liga +0x1c / +0x20: liga de cima / de baixo (-1 = nenhuma); id; grava', 'league +0x1c / +0x20: league above / below (-1 = none); id; writes'),
          ('liga +0xd9: palavra de bits de regra (8 = fase seguinte); valor (palavra); grava', 'league +0xd9: rule bit word (8 = next stage); value (word); writes'),
          ('liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava', 'league +0x49 / +0x4a: bench substitutes / substitutions; value; writes'),
          ('liga +0xab / +0xdb: palavras copiadas em cada partida (+0x38 e +0x36 da partida) [inferência: códigos de estágio]; valor; grava 0',
           'league +0xab / +0xdb: words copied to each match (match +0x38 and +0x36) [inference: stage codes]; value; writes 0'),
          ('liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (pelo slot 16)',
           'league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (through slot 16)')]
INTERESSA = ['CMRTE']
