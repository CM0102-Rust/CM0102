# Mutacoes. As excursoes marcam amistosos dos clubes da IA na pre-temporada (datas, publico, renda, desgaste): a largura
# da excursao 2n + 3 em vez de 2n + 1 (48 chamadas ate 2002-06-01: a janela encolhe e o dia sorteado muda), o amistoso
# da excursao com o mando trocado (a renda e o publico vao para o outro clube) e os jogos seguintes da excursao de 3 em 3
# dias (so quando ha um adversario depois). Os ramos ficam depois de todas as condicoes, colados no efeito.
MUTACOES = [
    ('cpt12_largura', 'let largura = (n as u8).wrapping_mul(2).wrapping_add(1) as i8 as i16; // cpt12: a largura da excursão',
     'let largura = (n as u8).wrapping_mul(2).wrapping_add(3) as i8 as i16; // mutacao: mais larga',
     'cpt_12 janela da excursão'),
    ('cpt12_mando_trocado', 'let m = acrescentar(this, clube, &mut d1, outro, 0); // cpt12: o clube em casa',
     'let m = acrescentar(this, outro, &mut d1, clube, 0); // mutacao: mando trocado',
     'cpt_12 jogo da excursão marcado'),
    ('cpt12_tres_dias', 'let r = mais_dias(d1.as_ptr() as usize, &mut t, 2); // cpt12: o próximo jogo',
     'let r = mais_dias(d1.as_ptr() as usize, &mut t, 3); // mutacao: de 3 em 3 dias',
     'cpt_12 data do jogo seguinte'),
]
