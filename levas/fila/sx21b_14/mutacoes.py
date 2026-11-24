# Mutacoes (sx21b_14). A troca de buffers roda nas ordenacoes da carreira; cai pelo comparador lado a lado (e pela
# carreira, quando a ordem muda uma escalacao). A versao so aparece nas caixas de erro: sem mutacao. A copia de arquivo:
# so a original no comparador, sem mutacao.
MUTACOES = [
    ('sx21b_14_um_byte_a_menos',
     '    for k in 0..count as usize { // sx21b_14: every byte',
     '    for k in 0..(count as usize).saturating_sub(1) { // mutation: one byte fewer',
     'sx21b_14 bytes swapped'),
]
