# Mutacoes da disciplina (o comparando acusa pelo plano na primeira suspensao; a carreira pelas escalacoes). O jogo
# cumprido contando 2: a suspensao por jogos pula o fim (contador passa do total) ou acaba antes; o jogador volta em
# outra data e as escalacoes mudam. O orgao sem esquecer a suspensao solta: a entrada dele segue com o indice (+6), o
# jogador continua impedido (0x55d270) e a proxima suspensao estende a velha (0x556940). O jogo antes do comeco da
# suspensao contando: so muda se o jogo do cartao (ou outro da mesma data) chega antes do comeco gravado (PODE
# ESCAPAR se a carreira nunca passa pelo ramo; o contador diz).
MUTACOES = [
    ('cpm4_jogo_cumprido_dois', 'let n = (*((s + 1) as *const u8)).wrapping_add(1); // mais um jogo cumprido',
     'let n = (*((s + 1) as *const u8)).wrapping_add(2); // mutacao: dois jogos', 'cpm_4 match served'),
    ('cpm4_orgao_nao_esquece', 'esquecer(ruling_body_for_rule(this, regra), id); // o órgão esquece a suspensão',
     '// mutacao: o orgao nao esquece', 'cpm_4 release from the body'),
    ('cpm4_antes_do_inicio', 'if ano < ano0 || (ano == ano0 && dia < dia0) { // ainda não começou',
     'if ano < ano0 && dia < dia0 && false { // mutacao: conta antes do comeco', 'cpm_4 before the start'),
]
