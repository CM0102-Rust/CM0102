# O grupo sul da Primeira Divisao finlandesa e a repescagem contra a Veikkausliiga (fin_first.cpp; leitor cx09, rodada
# 5, 24/11/2026). Simulacao com a Finlandia ('ENG SWE FIN NOR POR', ate 2002-08-01): o grupo sul roda na criacao do jogo
# e na virada (cobertura do mundo: 2), a repescagem uma vez, em outubro de 2001.
# - 0x594cd0 FinFirst::create_south_group_stage thiscall() ret 0: o mesmo corpo do BelThird::create_group_b_stage
#   0x422970 com a chave [0x9cf7f0] (sul), 3 turnos e as caixas 0x52f, 0x540, 0x54d, 0x558 do fin_first.cpp
#   (create_second_group_stage do cx09_13; os 22 argumentos do League_Stage conferidos, so o 4o muda). Prologo push -1;
#   push imm32.
# - 0x594610 FinFirst::create_premier_play_off_stage thiscall() ret 0: malloc(8) (caixa 0x3bb); o primeiro time da fase
#   1 (Cup_Stage, +0xa7, entradas de 6 bytes, contagem +0x3e) com +5 = 2 e a primeira linha da Veikkausliiga
#   ([[0xadadfc] + 4 x [0x9cf7e8]]) com +0x37 = 2 (sem achar: a vaga fica com o lixo do malloc, defeito); virtual 0x3c
#   (3, &rodadas, &extra, 0); operator new 0xb2 e Cup_Stage 0x522e00 (this, 2, par, rodadas, competicao, tabela, ano, 3,
#   1, extra, 0x14, 0, 0, 0, 0) na vaga 3; sem fase a caixa (0x3e0, sem liberar); senao free da tabela e do par e +0x30
#   = 3 (sem Cup::set_up_round). Prologo mov eax, fs:[0] (6 bytes).
# Fica de fora (relatorio) a 0x594080 FinFirst::create_play_off_stages (1,4 KB, x87 e quadro alinhado).
# Comparadores: so a original; o grupo conferido depois (vaga 0 e fase atual); a repescagem conferida contra o par da
# copia feito antes (em qualquer ordem).
# Iguais no cm0102-1080.exe. Depende do cx09_13 (e por ele do cx09_8 e do cx09_12), do cx09_11 e do cx09_14.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('fin_first_create_south_group_stage', 0x594cd0, [0x6a, 0xff, 0x68, 0x9e, 0x8f, 0x95, 0x00]),
           ('fin_first_create_premier_play_off_stage', 0x594610, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o grupo sul da Primeira Divisão finlandesa e a repescagem contra a Veikkausliiga',
        'the Finnish First Division south group and the play-off against the Veikkausliiga',
        [('fin_first_create_south_group_stage', 'FinFirst::create_south_group_stage', '0x594cd0', 'fin_first.cpp', 'thiscall, ret 0',
          'o grupo sul da Primeira Divisão finlandesa: os dez clubes do sul, em três turnos, com as regras da liga (fase 0)',
          'the Finnish First Division south group: the ten southern clubs, three legs, with the league rules (stage 0)'),
         ('fin_first_create_premier_play_off_stage', 'FinFirst::create_premier_play_off_stage', '0x594610', 'fin_first.cpp', 'thiscall, ret 0',
          'a repescagem entre o vencedor da repescagem de acesso da Primeira e o clube da Veikkausliiga que vai a ela (fase 3)',
          "the play-off between the First Division's promotion play-off winner and the Veikkausliiga club sent to it (stage 3)")])
CAMPOS = []
INTERESSA = []
