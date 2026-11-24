# Mutacoes (sx09b_8). O texto nao volta para a simulacao: acusada pelo comparador de texto quando a noticia 0x1784 e
# escrita; na carreira de ferias ninguem multa, entao ela provavelmente escapa (a rotina nao roda).
# - o primeiro aviso vira o ultimo.
MUTACOES = [
    ('sx09b_aviso_da_diretoria',
     '} else if exe::newsitem_get_slot(news, 0) != 0 { // sx09b_8: the first warning',
     '} else if exe::newsitem_get_slot(news, 0) == 0x7fff_ffff { // mutation',
     'sx09b_8 board fine warning, the first'),
]
