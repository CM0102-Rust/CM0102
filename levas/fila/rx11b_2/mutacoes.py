# Mutacoes (rx11b_2). As noticias e o texto nao voltam a simulacao: acusadas pelos comparadores da comparacao (o de
# add_result confere a noticia guardada na caixa do tecnico humano da carreira; o do texto, lado a lado).
# - a vaga da marca do outro lado invertida: a noticia guardada difere do plano da copia (e o texto: o nome do
#   adversario como time principal ou reserva);
# - as manchetes de empate e de derrota trocadas: o texto difere.
MUTACOES = [
    ('rx11b_2_marca_do_outro_lado',
     "        exe::newsitem_set_slot(news, k.wrapping_add(1) as i8, (other == 0) as u32); // rx11b_2: the other side's mark\n",
     "        exe::newsitem_set_slot(news, k.wrapping_add(1) as i8, (other != 0) as u32); // mutation\n",
     'rx11b_2 reserve result news'),
    ('rx11b_2_manchete_trocada',
     'let template = if gf > ga { 0xa7a49c } else if gf == ga { 0xa7a46c } else { 0xa7a43c }; // "<club> win/draw/lose"',
     'let template = if gf > ga { 0xa7a49c } else if gf == ga { 0xa7a43c } else { 0xa7a46c }; // mutation',
     'rx11b_2 reserve result headline'),
]
