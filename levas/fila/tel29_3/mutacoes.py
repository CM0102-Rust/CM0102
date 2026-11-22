# Mutacoes (tel29_3): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva; estas provam
# os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - o voto do computador vale dois (o comparador lado a lado de pick_winners pega na votacao do premio de fim de
#   temporada, de maio/junho);
# - o voto do tecnico humano vale dois (o comparador lado a lado de cast_vote pega ao votar na tela de indicados).
MUTACOES = [
    ('tel29_3_voto_duplo_computador',
     "write_i16(votes, read_i16(votes).wrapping_add(1)); // tel29_3: a computer club's vote",
     "write_i16(votes, read_i16(votes).wrapping_add(2)); // mutation: a computer club's vote counts two",
     'tel29_3 winners picked'),
    ('tel29_3_voto_duplo_humano',
     "write_i16(votes, read_i16(votes).wrapping_add(1)); // tel29_3: the human's vote counts one",
     "write_i16(votes, read_i16(votes).wrapping_add(2)); // mutation: the human's vote counts two",
     'tel29_3 human vote'),
]
