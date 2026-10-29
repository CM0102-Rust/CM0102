# Mutacoes (com o ramo de cada uma). A resposta a todas sem resposta e so a mais nova (o comparando roda a original: so
# a carreira acusa, pelos convites e propostas que ficam sem resposta quando o tecnico de testes sai de ferias no
# comeco da carreira; 2 chamadas: PODEM ESCAPAR se a caixa estiver vazia nessa hora). O proximo campo do texto um
# adiante (o comparando acusa pelo byte; a carreira so se a noticia de tecnico for lida pelos campos).
MUTACOES = [
    ('cpn_responder_todas', 'responder(vaga, tecnico, clube); // a resposta automática',
     '// mutacao: sem a resposta automatica',
     'cpn_2 responder todas'),
    ('cpn_responder_so_a_nova', 'k -= 1;\n        if k == cpn2_i(e + 4) - 1 {\n            return 1;',
     'k -= 1;\n        if true { // mutacao: so a mais nova\n            return 1;',
     'cpn_2 responder todas'),
    ('cpn_proximo_campo', '*proximo = p; // o próximo campo livre',
     '*proximo = p.wrapping_add(1); // mutacao: um campo adiante',
     'cpn_2 proximo campo do texto'),
]
