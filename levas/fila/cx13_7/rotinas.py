# A fase seguinte e a repescagem de acesso da Segunda Divisao polonesa (pol_second.cpp; leitor cx13, rodada 5,
# 24/11/2026). Simulacao com a Polonia (as ligas e a data dos itens mwd): a repescagem e criada quando a Primeira
# acaba (junho de 2002; 1 chamada de cada na cobertura do mundo ate 2002-08-01).
# - 0x7cc1c0 PolSecond::advance_stage thiscall() ret 0: na ultima fase a caixa da linha 0x234; senao, com a Primeira
#   completa (virtual +0x10 com 0, 1: push 1; push 0), a fase final da Primeira (fases[1]) atualiza posicoes e status
#   (virtual +0x94), a fase atual sobe 1 e, ao chegar a 0, create_play_off_stage (call direto 0x7cc290: a copia chama
#   pelo endereco). Prologo sub esp, 0x200 (6 bytes).
# - 0x7cc290 PolSecond::create_play_off_stage thiscall() ret 0: calloc 0x945501 (4, 4; cdecl) da lista (sem memoria,
#   caixa da linha 0x260); as linhas da Segunda com status 1 nas vagas 0 e 2, as da fase final da Primeira com status 2
#   (League::get_table_row 0x6826d0, thiscall(k), ret 4) nas vagas 1 e 3; o calendario da fase 0 (virtual +0x3c); a fase
#   (operator new 0xb2, Cup_Stage::Cup_Stage 0x522e00 com 15 argumentos, ret 0x3c) na vaga 0 das fases; a lista e o
#   calendario liberados; sem fase a caixa da linha 0x28b, senao Cup::set_up_round 0x51c800 com 0. O quadro SEH so
#   cobre o construtor. Prologo mov eax, fs:[0] (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original nas duas, com o plano (a fase sobe 0 ou 1; a fase
# criada com os 4 clubes do plano, como conjunto).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_second_advance_stage', 0x7cc1c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_second_create_play_off_stage', 0x7cc290, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a fase seguinte e a repescagem de acesso da Segunda Divisão polonesa',
        'the Polish Second Division\'s next stage and promotion play-off',
        [('pol_second_advance_stage', 'PolSecond::advance_stage', '0x7cc1c0', 'pol_second.cpp', 'thiscall, ret 0',
          'espera a Primeira Divisão acabar; então a fase final da Primeira fecha as posições, a fase sobe uma e, na fase 0, monta a repescagem',
          'waits for the First Division to finish; then the First Division\'s final stage settles its positions, the stage goes up by one and, at stage 0, the play-off is built'),
         ('pol_second_create_play_off_stage', 'PolSecond::create_play_off_stage', '0x7cc290', 'pol_second.cpp', 'thiscall, ret 0',
          'a repescagem em mata-mata: os dois clubes da Segunda com vaga na repescagem contra os dois da Primeira marcados para ela (Segunda × Primeira em cada confronto)',
          'the knockout play-off: the two Second Division clubs with a play-off place against the two First Division clubs marked for it (Second v First in each tie)')])
CAMPOS = []
INTERESSA = []
