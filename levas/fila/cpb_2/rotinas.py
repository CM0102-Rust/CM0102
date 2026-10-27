# Bloco novo: competições (Cup.cpp). O progresso do time na copa (0x521b30: 10 chamadas diretas no processamento da
# partida 0x51a150; 1.148 na carreira) e a copa terminada (0x521e00, posição 0x10 das vtables das copas; 4.365).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem
# sorteio, sem variável local. As duas chamam pela vtable: a 0x521b30 a posição 0x68 da própria copa
# (set_club_progress, que grava os bytes de progresso na estatística do clube), a 0x521e00 a posição 0x10 de cada etapa
# (League::check_complete paga os prêmios e marca a liga terminada: não pode rodar duas vezes). O comparador escuta
# essas posições por uma cópia da vtable do objeto (escuta do item cpb_1, que vem antes na fila). A cópia chama pelo
# mesmo ponteiro, como a original. A 0x521b30 empurra bytes com os bits altos do registrador (o ebx de quem chamou):
# vão sem eles e a fita compara só o byte (as 70 versões de set_club_progress leem bytes). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('copa_progresso_do_time', 0x521b30, [0x66, 0x8b, 0x41, 0x3c, 0x53]),
           ('copa_terminou', 0x521e00, [0x56, 0x8b, 0xf1, 0x57, 0x8a, 0x86, 0xb1, 0x00, 0x00, 0x00])]
DOCS = ('a copa: o progresso do time e a copa terminada',
        'the cup: the team progress and the finished cup',
        [('copa_progresso_do_time', 'Cup::record_team_progress', '0x521b30', 'Cup.cpp', 'thiscall',
          'guarda até onde o time foi (campeão, vice, terceiro, quarto ou a rodada em que saiu) pela posição 0x68 da vtable (set_club_progress)',
          'stores how far the team went (winner, runner-up, third, fourth or the round it went out) through vtable slot 0x68 (set_club_progress)'),
         ('copa_terminou', 'Cup::check_complete', '0x521e00', 'Cup.cpp', 'thiscall',
          '1 se a copa acabou e, pedindo as etapas numa copa de etapas, todas terminaram e a última foi alcançada',
          '1 when the cup is over and, when the stages are asked for in a staged cup, all of them are complete and the last one was reached')])
CAMPOS = [('Cup +0x3c (rodadas), +0x44 (etapa), +0xa3 (rodadas: +7 tipo, +0x1a confrontos, +0x5b código), +0xa7 (times: +5 desfecho); lê (copa_progresso_do_time)',
           'Cup +0x3c (rounds), +0x44 (stage), +0xa3 (rounds: +7 type, +0x1a ties, +0x5b code), +0xa7 (teams: +5 outcome); reads (copa_progresso_do_time)'),
          ('Comp +0xc (etapas), +0x2c (número de etapas), +0x30 (etapa atual), +0x44, +0xb1 (acabou); lê (copa_terminou)',
           'Comp +0xc (stages), +0x2c (stage count), +0x30 (current stage), +0x44, +0xb1 (finished); reads (copa_terminou)')]
INTERESSA = ['SGE', 'CMRTE']
