# Mutacoes (cx01_11), carreira de 8 ligas ate 2002-06-01 (os comparadores so rodam a original: precisa mudar a
# carreira). Nenhuma abre caixa.
# - as linhas da preliminar com a marca 1 e nao 0 (League::init_table_row): o lado dos jogos da preliminar muda;
# - a preliminar comecada sem a copa ativa: a atualizacao diaria e o fechamento da copa a pulam, e os grupos nao vem.
# A fase seguinte fica sem mutacao propria: as variantes (pular os grupos ou as semifinais, passar da ultima fase)
# deixam a copa com uma fase sem objeto e derrubam o jogo; os grupos e as semifinais sao provados pelo cx01_12.
MUTACOES = [
    ('cx01_11_linhas_marca_1',
     '        exe::league_init_table_row(this, row as u8 as u32, read_usize(entry), 0, 0);',
     '        exe::league_init_table_row(this, row as u8 as u32, read_usize(entry), 1, 0); // mutation'),
    ('cx01_11_copa_inativa',
     '    covered!("cx01_11 preliminary started");\n    cup.set_active(1);',
     '    covered!("cx01_11 preliminary started");\n    cup.set_active(0); // mutation'),
]
