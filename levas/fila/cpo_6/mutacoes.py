# Mutacoes (com o ramo de cada uma), todas na resposta do tecnico da maquina (428 chamadas na carreira ate 2002-06-01;
# a resposta escolhe o efeito da historia em 0x736c00: confiancas, noticias, sorteios). Os tipos comuns (0, 2, 9, 0xc,
# 0xd, 0xe, 0x11, 0x21..0x23, 0x52) sao as historias mais frequentes. A primeira tira o sorteio que o original descarta
# (o gerador anda um a menos; o comparador lado a lado no gerador tambem acusa). A segunda muda o limit da soma
# (sorteio(0x24) -> 0x20) e a terceira a nota (x 5 -> x 4), as duas mudam a resposta. Se a carreira nao tiver historia
# de tipo comum com contrato 0x10..0x30 na lista, a primeira PODE ESCAPAR (ver o ramo).
MUTACOES = [
    ('cpo_sem_sorteio_descartado', 'sorteio(cpo6_c(tecnico + 0x59)); // o original descarta o resultado',
     '// mutacao: sem o sorteio descartado',
     'cpo_6 sorteio descartado'),
    ('cpo_tipos_comuns_limite', 'let y = sorteio(0x24);\n            if x <= y {\n                return 3;',
     'let y = sorteio(0x20); // mutacao\n            if x <= y {\n                return 3;',
     'cpo_6 tipos comuns'),
    ('cpo_nota_x4', 'nota = match_pl_ftol(nota_da_historia(this, jogador, clube, contrato, tipo, 3, 0.0) * *(0x968990 as *const f32) as f64);',
     'nota = match_pl_ftol(nota_da_historia(this, jogador, clube, contrato, tipo, 3, 0.0) * 4.0); // mutacao',
     'cpo_6 tipos comuns'),
]
