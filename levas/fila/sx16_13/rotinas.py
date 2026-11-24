# physio.cpp, parte 1 (leitor sx16, rodada 5, 24/11/2026): o objeto estatico e dois juizos do fisioterapeuta.
# - 0x7a1a70 cdecl: push 0x7a1a80; call 0x94627a atexit; pop; ret. Inicializador da CRT (roda na partida do exe, depois
#   da carga da ddraw.dll, que ja instalou os desvios; o precedente sao os setup_static_init_*). Prologo `push imm32`
#   (5 bytes, absoluto).
# - 0x7a1a80 cdecl: ret (o destrutor vazio, chamado na saida). Prologo `ret` + 4 nop de enchimento, como o
#   match_stats_static_date_atexit (0x72aa70, ja desviado do mesmo jeito).
# - 0x7a25c0 thiscall(fisio, pessoa) ret 4 -> al (ecx carregado pelo chamador e nao usado): o pico pela posicao (+0xf
#   goleiro 20: 33; +0x11/+0x12: 31; +0x13/+0x14: 29; senao 28), x87 fild da idade e do pico x 1,1 (0x9689c0), fcompp.
#   O retorno 2 sai do truque setge/dec/and 0xfa/add 8 com o ah do fnstsw (al = 2). Chamada por
#   physio_file_report_news (0x7a1b26).
# - 0x7a2900 thiscall(fisio, fisioterapeuta, jogador) ret 8 -> eax: as caixas do physio.cpp (0x355, 0x35a) e 180;
#   (id + id) % 35 por idiv com sinal. Chamada por physio_assess_player_fitness (0x7a281d).
# Ficam para outro item do physio.cpp (nao feitos nesta rodada): physio_file_report_news 0x7a1a90, o texto do
# relatorio 0x7a1cd0 e physio_assess_player_fitness 0x7a2650.
# Caminho no jogo: o perfil de um jogador do proprio clube -> pedir o relatorio do fisioterapeuta (a tela do relatorio
# 0x7a2f10); abrir a noticia do relatorio na caixa de entrada.
TELAS = True
ROTINAS = [('physio_static_init', 0x7a1a70, [0x68, 0x80, 0x1a, 0x7a, 0x00]),
           ('physio_static_object_destructor', 0x7a1a80, [0xc3, 0x90, 0x90, 0x90, 0x90]),
           ('physio_get_age_stage', 0x7a25c0, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9]),
           ('physio_get_days_to_judge', 0x7a2900, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o fisioterapeuta (1): o objeto estático e os juízos de idade e de tempo',
        'the physio (1): the static object and the age and time judgements',
        [('physio_static_init', 'physio_static_init', '0x7a1a70', 'physio.cpp', 'cdecl',
          'registra no atexit o destrutor vazio do objeto estático do physio.cpp',
          "registers with atexit the empty destructor of physio.cpp's static object"),
         ('physio_static_object_destructor', 'physio_static_object_destructor', '0x7a1a80', 'physio.cpp', 'cdecl',
          'não faz nada (o destrutor do objeto estático)',
          'does nothing (the static object\'s destructor)'),
         ('physio_get_age_stage', 'physio_get_age_stage', '0x7a25c0', 'physio.cpp', 'thiscall, ret 4',
          'a fase da carreira do jogador para o relatório: o pico pela posição principal (goleiro 33, defensor 31, meio-campo 29, os outros 28); passou de 1,1 × o pico e tem mais de 32 anos: 4; menos de 18: 1; de 18 a 23: 2; senão 8; sem pessoa ou ficha de jogador, 0',
          'the player\'s career stage for the report: the peak by the main position (goalkeeper 33, defender 31, midfielder 29, others 28); past 1.1 × the peak and older than 32: 4; under 18: 1; 18 to 23: 2; otherwise 8; without a person or player sheet, 0'),
         ('physio_get_days_to_judge', 'physio_get_days_to_judge', '0x7a2900', 'physio.cpp', 'thiscall, ret 8',
          'os dias que o fisioterapeuta precisa com o jogador antes de julgá-lo: 0 para um jogador de ponta (capacidade acima de 160, ou potencial acima de 160 com capacidade acima de 100); senão (soma dos ids módulo 35) + 30 + 25, 40 ou 55 pela capacidade do fisioterapeuta (acima de 140, de 80, ou menos); sem fisioterapeuta ou jogador, a caixa de erro e 180',
          'the days the physio needs with the player before judging him: 0 for a top player (ability above 160, or potential above 160 with ability above 100); otherwise (the ids summed modulo 35) + 30 + 25, 40 or 55 by the physio\'s ability (above 140, above 80, or less); without the physio or the player, the error box and 180')])
