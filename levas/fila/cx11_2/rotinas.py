# Os construtores da Serie C2 italiana, grupos A e B (ita_ser_c2a.cpp, ita_ser_c2b.cpp; leitor cx11, rodada 5,
# 24/11/2026). Simulacao com a Italia e as divisoes inferiores carregadas: rodam uma vez cada na criacao do jogo.
# ATENCAO, a base: a Serie C2 so existe quando a criacao do jogo tem a base NORMAL ou MAXIMA. O setup (0x8274e1..)
# grava em nation+0x11c ((~[0xae3138+4] & 2) | 1) << 1, e o DATABASE= do auto_setup grava [0xae3138+4] (0x8272c0:
# NORMAL 1, MINIMUM 2, MAXIMUM 4); com MINIMUM (a do simular.py) o bit 4 (LOWER_DIVISIONS_LOADED) fica apagado e o
# setup da Italia (0x83562f) nao cria as tres C2. Por isso 0 chamadas na cobertura do mundo. A prova pede uma carreira
# com DATABASE=NORMAL (BASE abaixo; o simular.py e o fila.py ainda nao leem essa chave).
# - 0x652e90 ItaSerC2a::ItaSerC2a e 0x657350 ItaSerC2b::ItaSerC2b thiscall(ano, registro) ret 8: o corpo do
#   ItaSerC1a::ItaSerC1a (conferido instrucao a instrucao: so a vtable 0x96c69c / 0x96c750, as regras 0x653b90 /
#   0x658050, a tabela 0x653a90 / 0x657f50, o calendario real 0x6545c0 / 0x658750 e o fonte 0x9ea654 / 0x9ea75c mudam;
#   as linhas das caixas 0x49, 0x50, 0x5a, 0x63, 0x6a sao as da C1/A nas duas): construct_italian_division com
#   SERIE_C2A_DIVISION / SERIE_C2B_DIVISION. Chamados pelo setup da Italia (0x835678 e 0x8356ce: push registro; push
#   ano; mov ecx, novo -> thiscall, ret 8). Prologo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe (inteiros). Comparadores: so a original (aloca), compare_italian_constructor (a devolucao,
# a vtable, o ano, o registro, a disciplina e, na liga nova, as 2 fases).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('construct_ita_ser_c2a', 0x652e90, [0x6a, 0xff, 0x68, 0xd9, 0xbb, 0x95, 0x00]),
           ('construct_ita_ser_c2b', 0x657350, [0x6a, 0xff, 0x68, 0x79, 0xbc, 0x95, 0x00])]
DOCS = ('os construtores da Série C2 italiana, grupos A e B',
        'the constructors of the Italian Serie C2 groups A and B',
        [('construct_ita_ser_c2a', 'ItaSerC2a::ItaSerC2a', '0x652e90', 'ita_ser_c2a.cpp', 'thiscall, ret 8',
          'monta a Série C2/A italiana na criação do jogo (só com a base normal ou máxima): regra de disciplina 0x11 (a das ligas italianas), a liga e duas fases (a repescagem de acesso e o play-out do rebaixamento); sem nada salvo, as regras, a tabela, o calendário real de 2001/02 pelos nomes dos clubes, os jogos, a ordem, as estatísticas e as posições previstas',
          'builds the Italian Serie C2/A at game creation (only with the normal or maximum database): discipline rule 0x11 (the Italian leagues\' one), the league and two stages (the promotion play-off and the relegation play-out); with nothing saved, the rules, the table, the real 2001/02 calendar by club names, the fixtures, the order, the statistics and the predicted positions'),
         ('construct_ita_ser_c2b', 'ItaSerC2b::ItaSerC2b', '0x657350', 'ita_ser_c2b.cpp', 'thiscall, ret 8',
          'monta a Série C2/B italiana, como a C2/A',
          'builds the Italian Serie C2/B, as C2/A')])
CAMPOS = []
INTERESSA = []
