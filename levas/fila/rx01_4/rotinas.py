# O interpretador TrueType da T2K: SFVTPV, ISECT e os opcodes que gravam o estado grafico (SMD, SCVTCI, SSWCI, SSW,
# SRP0-2/SLOOP/POP) (library_without_asserts; leitor rx01, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao rodam (so numa fonte com instrucoes desenhada com ajuste: ver o rx01_1). Caminho no jogo:
# abrir o jogo comparando.
# Convencoes conferidas: cdecl, pela tabela dos opcodes (estado local).
# - 0x9384f0 tt_op_isect: lida instrucao a instrucao (as cinco posicoes da pilha, as zonas 0, 1 e 2, os ramos da reta
#   horizontal/vertical, da inclinacao maior, do paralelo e do quociente invertido com fixed_div).
# - 0x9387f0 tt_op_ssw: a escala pelo campo +0x98 do estado global (cdecl: estado global, valor).
# - 0x938820 tt_op_srp_sloop_pop: a tabela de salto 0x938854 (8 entradas, opcodes 0x10-0x17: 0x10-0x12 os pontos de
#   referencia, 0x13-0x16 nada, 0x17 o laco); 0x21 (POP) cai fora da tabela.
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (mudam o estado). Prova sem o jogo: o emulador (rascunho rx01/check_rx01.py).
# Depende do rx01_1 e do rx01_3 (TT_ONE, TT_MOVE_FREE).
SAVE = False
TELAS = True
ROTINAS = [('tt_op_sfvtpv', 0x9384d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x14]),
           ('tt_op_isect', 0x9384f0, [0x83, 0xec, 0x20, 0x8b, 0x4c, 0x24, 0x24]),
           ('tt_op_smd', 0x938790, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_scvtci', 0x9387b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_sswci', 0x9387d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_ssw', 0x9387f0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_srp_sloop_pop', 0x938820, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: SFVTPV, ISECT e os que gravam o estado gráfico',
        'the T2K TrueType interpreter: SFVTPV, ISECT and the graphics state setters',
        [('tt_op_sfvtpv', 'tt_op_sfvtpv', '0x9384d0', _A, 'cdecl',
          'regra: SFVTPV (0x0e): a liberdade passa a ser a projeção (produto 1)',
          'rule: SFVTPV (0x0e): the freedom becomes the projection (product 1)'),
         ('tt_op_isect', 'tt_op_isect', '0x9384f0', _A, 'cdecl',
          'regra: ISECT (0x0f): tira a reta a (zona 0), a reta b (zona 1) e o ponto (zona 2) da pilha e põe o ponto no cruzamento das retas (a reta horizontal com a vertical direto; paralelas: o meio dos pontos médios); o ponto fica tocado nos dois eixos',
          'rule: ISECT (0x0f): pops line a (zone 0), line b (zone 1) and the point (zone 2) and puts the point where the lines cross (a horizontal with a vertical directly; parallel lines: the middle of their midpoints); the point is touched in both axes'),
         ('tt_op_smd', 'tt_op_smd', '0x938790', _A, 'cdecl',
          'regra: SMD (0x1a): a distância mínima tirada da pilha', 'rule: SMD (0x1a): the minimum distance popped from the stack'),
         ('tt_op_scvtci', 'tt_op_scvtci', '0x9387b0', _A, 'cdecl',
          'regra: SCVTCI (0x1d): o corte dos valores de controle tirado da pilha', 'rule: SCVTCI (0x1d): the control value cut-in popped from the stack'),
         ('tt_op_sswci', 'tt_op_sswci', '0x9387d0', _A, 'cdecl',
          'regra: SSWCI (0x1e): o corte da largura única tirado da pilha', 'rule: SSWCI (0x1e): the single width cut-in popped from the stack'),
         ('tt_op_ssw', 'tt_op_ssw', '0x9387f0', _A, 'cdecl',
          'regra: SSW (0x1f): a largura única em unidades da fonte, tirada da pilha e guardada também em pixels pela escala',
          'rule: SSW (0x1f): the single width in font units, popped and also stored in pixels through the scaler'),
         ('tt_op_srp_sloop_pop', 'tt_op_srp_sloop_pop', '0x938820', _A, 'cdecl',
          'regra: SRP0-2 (0x10-0x12) guardam o valor tirado da pilha no ponto de referência, SLOOP (0x17) no laço (valor − 1), POP (0x21) só tira',
          'rule: SRP0-2 (0x10-0x12) store the popped value in the reference point, SLOOP (0x17) in the loop (value − 1), POP (0x21) only pops')])
CAMPOS = []
INTERESSA = []
