# Mutacoes (cqh_7): as ordens dos arbitros (uma vez por carreira, na criacao do jogo da carreira automatica; milhares
# de comparacoes). A ordem decrescente decide os arbitros que ficam por nacao (0x79bf60); a crescente, os indices
# depois dos sorteados. A tabela dos arbitros e o banco do jogo (impressao diaria) mudam.
# - O peso do atributo +0x28 na qualidade 40 em vez de 50 (as duas ordens).
# - O empate da ordem decrescente pelo indice de a menos o de b.
# - A ordem crescente invertida pela qualidade.
MUTACOES = [
    ('cqh7_peso_40', '(*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(cqh7_w(r + 0x22)) // cqh7: o peso da qualidade',
     '(*((r + 0x28) as *const i8) as i16).wrapping_mul(0x28).wrapping_add(cqh7_w(r + 0x22)) // mutacao: o peso da qualidade', 'cqh_7 ordem decrescente'),
    ('cqh7_empate_invertido', 'return cqh7_i(b).wrapping_sub(cqh7_i(a)); // cqh7: o empate pelo índice',
     'return cqh7_i(a).wrapping_sub(cqh7_i(b)); // mutacao: o empate pelo índice', 'cqh_7 ordem decrescente'),
    ('cqh7_crescente_invertida', 'ka as i32 - kb as i32 // cqh7: a ordem crescente',
     'kb as i32 - ka as i32 // mutacao: a ordem crescente', 'cqh_7 ordem crescente'),
]
