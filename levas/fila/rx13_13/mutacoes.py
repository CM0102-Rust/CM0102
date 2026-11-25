# Mutacoes (rx13_13), pela carreira com as 8 ligas ate 2002-06-01 (a edicao de 2001 do Campeonato de Clubes da Oceania
# e jogada no segundo semestre de 2001):
# - a regra de disciplina inglesa (9) no lugar da 0x23 (outras suspensoes nos jogos da Oceania e depois nas ligas).
# O comparador tambem acusa (a disciplina contra a original), mas so roda a original; a mutacao vale pela carreira Rust.
MUTACOES = [
    ('rx13_13_disciplina_inglesa',
     'l.set_discipline_rule(0x23); // rx13_13: the discipline rule',
     'l.set_discipline_rule(9); // mutation: the English rule',
     'rx13_13 oceania club championship new'),
]
