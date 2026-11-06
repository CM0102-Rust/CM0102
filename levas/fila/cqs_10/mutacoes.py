# Mutacoes (cqs_10), nos mata-matas da Libertadores (2001 e 2002) e da Merconorte (2001): o seguinte de cada fase de grupo
# passa no lugar do classificado (quem passa muda); o seguinte do grupo 1 passa; as fases de grupo na lista em ordem
# inversa (o sorteio da primeira rodada sai de outra lista). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqs10_o_seguinte_de_cada_fase_passa',
     'cqs10_d(linha(fase, j)) // cqs10: a posição j da fase',
     'cqs10_d(linha(fase, j + 1)) // mutacao: o seguinte passa',
     'cqs_10 classificados da libertadores'),
    ('cqs10_o_seguinte_do_grupo_um_passa',
     'cqs10_d(cqs10_d(this + 0xb1) + i * 0x3b) // cqs10: a linha do grupo 1',
     'cqs10_d(cqs10_d(this + 0xb1) + (i + 1) * 0x3b) // mutacao: o seguinte passa',
     'cqs_10 classificados da merconorte'),
    ('cqs10_fases_em_ordem_inversa',
     'for f in 0..fases {\n        let mut j = 0i32;',
     'for f in (0..fases).rev() {\n        let mut j = 0i32;',
     'cqs_10 classificados da libertadores'),
]
