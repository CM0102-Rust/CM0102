# A copa (Cup.cpp): quem passa num jogo (Cup::decide_fixture_winner 0x51c020, 353 chamadas na carreira até 2002-06-01)
# e num confronto de ida e volta (Cup::decide_tie_winner 0x51f2f0, 514), as duas pela vtable das 63 copas, e a ordem
# dos cabeças de chave no sorteio (cup_compare_draw_entries_by_seed 0x51ebc0, a comparação do qsort da 0x51d2d0 e da
# 0x51d9f0). Gravam só os códigos do resultado (+0x3d/+0x3e do jogo). Sem SEH, sem sorteio, sem leitura de lixo (os
# bits altos do registrador da regra não são lidos). A caixa "Error" só no empate também nos pênaltis com a regra deles
# (a cópia chama a original; nada foi gravado antes, fora o +0x3d/+0x3e = 1 da regra 0x10, que a original refaz). A
# 0x521b30 (Cup) fica de fora: passa bytes com os bits altos de lixo ao método. Iguais no cm0102-1080.exe.
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: lado a lado pelos códigos
# e pelo lado devolvido (a comparação do sorteio pelo valor).
SAVE = False
ROTINAS = [('decidir_jogo_da_copa', 0x51c020, [0x81, 0xec, 0xe8, 0x05, 0x00, 0x00]),
           ('decidir_confronto_da_copa', 0x51f2f0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('ordem_dos_cabecas_de_chave', 0x51ebc0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('a copa: quem passa no jogo e no confronto e a ordem dos cabeças de chave no sorteio',
        'the cup: who goes through in the match and in the tie and the seed order in the draw',
        [('decidir_jogo_da_copa', 'Cup::decide_fixture_winner', '0x51c020', 'Cup.cpp', 'thiscall, ret 4',
          'quem passa num jogo de copa (1 mandante, 2 visitante, 0 ninguém) pelo placar, prorrogação, pênaltis, W.O. e pontos da regra (+0x38), gravando o código de cada lado (+0x3d/+0x3e)',
          'who goes through in a cup match (1 home, 2 away, 0 nobody) by the score, extra time, penalties, walkover and points of the rule (+0x38), storing each side\'s code (+0x3d/+0x3e)'),
         ('decidir_confronto_da_copa', 'Cup::decide_tie_winner', '0x51f2f0', 'Cup.cpp', 'thiscall, ret 8',
          'no primeiro jogo marca o vencedor e devolve 3; na volta decide pelo agregado, W.O., prorrogação, gols fora e pênaltis (1 ou 2 o lado que passa, 4 sem decisão)',
          'in the first leg marks the winner and returns 3; in the second leg decides by aggregate, walkover, extra time, away goals and penalties (1 or 2 the side through, 4 undecided)'),
         ('ordem_dos_cabecas_de_chave', 'cup_compare_draw_entries_by_seed', '0x51ebc0', 'Cup.cpp', 'cdecl',
          'a comparação do qsort das entradas de 7 bytes do sorteio: o cabeça de chave (+6) maior primeiro',
          'the qsort comparison of the 7-byte draw entries: the higher seed (+6) first')])
CAMPOS = [('jogo +0x38: a regra (0x10 pontos, 2 prorrogação, 0x21 pênaltis, 0x800/0x1000 W.O., 0x2000 códigos −3, 0x4000 e 4 gols fora); valor; lê (decidir_jogo_da_copa, decidir_confronto_da_copa)',
           'match +0x38: the rule (0x10 points, 2 extra time, 0x21 penalties, 0x800/0x1000 walkover, 0x2000 −3 codes, 0x4000 and 4 away goals); value; reads (decidir_jogo_da_copa, decidir_confronto_da_copa)'),
          ('jogo +0x3d/+0x3e: o código do resultado de cada lado (1, 2 venceu, 3 prorrogação, 4/5 pênaltis, 6 gols fora, 8 W.O.); valor; lê e grava (decidir_jogo_da_copa, decidir_confronto_da_copa)',
           'match +0x3d/+0x3e: each side\'s result code (1, 2 won, 3 extra time, 4/5 penalties, 6 away goals, 8 walkover); value; reads and writes (decidir_jogo_da_copa, decidir_confronto_da_copa)'),
          ('jogo +0x43/+0x44, +0x47/+0x48, +0x49/+0x4a: o placar, a prorrogação e os pênaltis; valor; lê (decidir_jogo_da_copa)',
           'match +0x43/+0x44, +0x47/+0x48, +0x49/+0x4a: score, extra time and penalties; value; reads (decidir_jogo_da_copa)'),
          ('confronto +0x14..+0x1b e +0x1e..+0x25: placar, prorrogação e pênaltis da volta e da ida; +0x33 o jogo (1 = volta); valor; lê (decidir_confronto_da_copa)',
           'tie +0x14..+0x1b and +0x1e..+0x25: score, extra time and penalties of the second and first legs; +0x33 the leg (1 = second); value; reads (decidir_confronto_da_copa)'),
          ('copa +0xa7/+0x3e: as entradas de 6 bytes dos times (+0 clube, +4 cabeça de chave) e a conta; ponteiro e valor; lê (decidir_jogo_da_copa)',
           'cup +0xa7/+0x3e: the 6-byte team entries (+0 club, +4 seed) and the count; pointer and value; reads (decidir_jogo_da_copa)'),
          ('entrada do sorteio +6: o cabeça de chave; valor; lê (ordem_dos_cabecas_de_chave)',
           'draw entry +6: the seed; value; reads (ordem_dos_cabecas_de_chave)')]
INTERESSA = ['CMRTE']
