# Mutacoes (sbA_8): a varredura roda uma vez ate 2002-06-01 (cobertura); o comparador (a lista temporaria e a posicao
# lado a lado) as acusa na comparacao; na carreira podem escapar se o chamador so olhar a contagem.
MUTACOES = [
    ('sbA_8_sem_filtro_de_rodada',
     '        } else if s.round_min != round {',
     '        } else if s.round_min != round && false { // mutation'),
    ('sbA_8_volta_um_ano_a_mais',
     '            if years >= 5 || (max == -1 && years > 1) {\n                stop(year, 0, slice as u8);',
     '            if years >= 6 || (max == -1 && years > 2) { // mutation\n                stop(year, 0, slice as u8);'),
]
