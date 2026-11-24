# Mutacoes (sx21_3). Os textos sao pegos pelo comparador (compare_text_writer); o texto nao volta a simulacao, entao a
# carreira nao os ve. A noticia (so a original no comparador, ela so vai para os tecnicos) nao tem mutacao que a
# cadeia pegue: fica sem.
# - a materia de uma semana com o formato de hoje;
# - a manchete de uma semana com a de hoje.
MUTACOES = [
    ('sx21_3_materia_semana',
     '2 => 0x989e70, // sx21_3: the story a week before',
     '2 => 0x989f20, // mutation: the story of the day',
     'sx21_3 deadline story'),
    ('sx21_3_manchete_semana',
     '2 => 0xa9e368, // sx21_3: the headline a week before',
     '2 => 0xa9e388, // mutation: the headline of the day',
     'sx21_3 deadline headline'),
]
