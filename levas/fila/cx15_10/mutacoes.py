# Mutacoes (cx15_10), carreira ENG ate 2002-06-01. Nenhuma abre caixa. As entradas so sao lidas quando as
# eliminatorias africanas comecam (depois da data da carreira): a carreira pode nao acusar; os comparadores lado a lado
# das duas rotinas acusam na comparacao (as duas rodam na criacao do jogo).
# - os inscritos sem ordenar pela forca: outra ordem na lista;
# - as marcas da primeira fase trocadas (a primeira metade com 2).
MUTACOES = [
    ('cx15_10_inscritos_sem_ordem',
     '    covered!("cx15_10 african entrants sorted");\n    exe::crt_qsort(list, count as u32, 4, 0x4b07d0);',
     '    covered!("cx15_10 african entrants sorted"); // mutation',
     'cx15_10 african entrants sorted'),
    ('cx15_10_marcas_trocadas',
     '{ 1 } else { 2 }); // the round the entrant starts in',
     '{ 2 } else { 1 }); // mutation',
     'cx15_10 african first round entries'),
]
