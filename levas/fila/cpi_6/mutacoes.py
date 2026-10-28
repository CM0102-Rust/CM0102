# Mutacoes dos diversos. A noticia segue depois da recusa (o comparando roda a original: so o modo Rust e a carreira;
# noticias quase nao entram na impressao: PODE ESCAPAR). A missao sem a competicao (o comparando acusa quando so a
# competicao esta marcada; olheiros quase nao decidem: PODE ESCAPAR da carreira). A copa com uma fase incompleta
# aceita: o comparando acusa no primeiro fim de fase; a carreira, se a copa terminar antes.
MUTACOES = [
    ('cpi_noticia_recusa', 'break; // a entrega recusada encerra',
     'continue; // mutacao: segue'),
    ('cpi_missao_comp', '|| cpi6_d(this + 0x195) as i32 > -1 // a competição',
     '// mutacao: sem a competicao'),
    ('cpi_copa_fase', 'return 0; // uma fase incompleta',
     '{} // mutacao: aceita a incompleta'),
]
