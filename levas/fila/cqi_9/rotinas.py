# As regras de cinco estaduais brasileiros por três rotinas (frente das competições, 03/11/2026, agente C).
# O slot 36 (+0x8c, `set_up_rules`): grava os bytes de regra e pede o calendário ao slot 16 (+0x3c). Uma rotina
# serve a mais de uma liga (a mesma entrada em várias vtables):
# - 0x437fc0 BraRegParana::set_up_rules (157 bytes): Paranaense (vtable 0x9684a0; calendário 0x437700).
# - 0x439430 BraRegPern::set_up_rules (157 bytes): Norte (0x968338; 0x435640) e Pernambucano (0x968554; 0x438bf0).
# - 0x43a600 BraRegRio::set_up_rules (156 bytes): Mineiro (0x968284; 0x434050) e Carioca (0x968608; 0x439d40).
# Rodam 2, 4 e 4 vezes na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): 2 por liga.
# As três gravam o mesmo conjunto (dois turnos); mudam as vagas de cima (+0xbe) e os rebaixados (+0xc1): o porte é um
# ajudante comum (cqi9_regras) com os parâmetros de cada uma, conferidos byte a byte.
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   as três: thiscall (`mov esi, ecx`), ret 0, devolvem 1; chamadas pelo slot 36 da vtable.
#   o calendário (slot 16, `call [edi+0x3c]`, edi = vtable lida antes): thiscall (`mov ecx, esi`), 4 argumentos
#   (empurrados 0, &+0x3a, &+0xa9, -1), ret 0x10 nos cinco alvos.
# Nenhuma chamada por endereço; nenhuma global lida.
# Comparadores: lado a lado com a cópia em ensaio (as regras sem o calendário), sobre o objeto da liga (0xee) menos as
# saídas do calendário (+0x3a, +0xa9, +0xba); os cinco calendários não gravam no objeto. Devolução comparada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqi9_regras_do_paranaense', 0x437fc0, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('cqi9_regras_do_pernambucano', 0x439430, [0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('cqi9_regras_do_carioca', 0x43a600, [0x56, 0x8b, 0xf1, 0x33, 0xc0])]
_COMUM_PT = ('dois turnos; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na '
             'tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de '
             'regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão); 7 reservas e 3 substituições; '
             'partidas de categoria 2 (liga). Depois, o calendário da fase principal')
_COMUM_EN = ('double round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the '
             'table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the '
             'top places lead to the next stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 '
             '(league) matches. Then the main stage schedule')
DOCS = ('as regras dos estaduais Paranaense, Norte, Pernambucano, Mineiro e Carioca',
        'the rules of the Parana, North, Pernambuco, Minas Gerais and Rio state leagues',
        [('cqi9_regras_do_paranaense', 'BraRegParana::set_up_rules', '0x437fc0', 'bra_reg_parana.cpp', 'thiscall',
          'Paranaense: nenhuma vaga de cima na tabela geral (cada turno é um estágio de liga próprio, criado pelo '
          'add_result 0x4386e0, e decide o título [inferência]); 4 rebaixados; ' + _COMUM_PT,
          'Parana: no top places in the overall table (each half is a league stage of its own, created by add_result '
          '0x4386e0, and decides the title [inference]); 4 relegated; ' + _COMUM_EN),
         ('cqi9_regras_do_pernambucano', 'BraRegPern::set_up_rules', '0x439430', 'bra_reg_pern.cpp', 'thiscall',
          'Norte e Pernambucano (a mesma rotina nas duas vtables): 4 vagas de cima (a fase final de 4); 2 rebaixados; ' + _COMUM_PT,
          'North and Pernambuco (the same routine in the two vtables): 4 top places (the final stage of 4); 2 relegated; ' + _COMUM_EN),
         ('cqi9_regras_do_carioca', 'BraRegRio::set_up_rules', '0x43a600', 'bra_reg_rio.cpp', 'thiscall',
          'Mineiro e Carioca (a mesma rotina nas duas vtables): nenhuma vaga de cima na tabela geral (os turnos têm '
          'estágios próprios, criados pelo add_result 0x43af20 / 0x435130 [inferência: decidem o título]); '
          '2 rebaixados; ' + _COMUM_PT,
          'Minas Gerais and Rio (the same routine in the two vtables): no top places in the overall table (the halves '
          'have stages of their own, created by add_result 0x43af20 / 0x435130 [inference: they decide the title]); '
          '2 relegated; ' + _COMUM_EN)])
CAMPOS = [('liga +0x3c: turnos (2); valor (palavra); grava', 'league +0x3c: round robins (2); value (word); writes'),
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
