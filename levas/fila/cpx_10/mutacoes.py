# Mutacoes (cpx_10), na noticia da pressao (0x6a0e70, por pensar em demitir o tecnico 0x69aba0, ja em Rust): a data de
# hoje nao vai para +0x18 da linha da diretoria (a 0x69aba0 le essa data nos prazos de 14 e 30 dias: a noticia, com os
# sorteios dela, volta mais cedo), o boato sem o sorteio(10) (um sorteio a menos por noticia) e o limite da primeira
# avaliacao a confianca - 400 em vez de - 500 (o sorteio(3) roda em outros clubes). As tres mudam a contagem dos sorteios
# e tudo o que sorteia depois. Escapam se a noticia da pressao nao sair ate 2002-06-01 (contadores "cpx_10 data da
# pressao" e "cpx_10 sorteio do boato"); a terceira tambem se nenhuma avaliacao +8 cair entre os dois limites (contador
# "cpx_10 motivo pela avaliacao", que conta so os casos abaixo do limite original). A saida e o tecnico novo so mudam as
# caixas dos humanos: passageiras (contadores "cpx_10 noticia da saida" e "cpx_10 noticia do tecnico novo").
MUTACOES = [
    ('cpx10_sem_data', '    copiar(cpx10_linha(this, clube) + 0x18, data.as_mut_ptr() as usize, 0xae2c90); // cpx10: a data da notícia na linha',
     '    let _ = (copiar, data); // mutacao: sem a data',
     'cpx_10 data da pressao'),
    ('cpx10_boato_sem_sorteio', '    let mut boato = cpx10_sortear(10); // cpx10: o sorteio do boato',
     '    let mut boato = 0; // mutacao: sem o sorteio',
     'cpx_10 sorteio do boato'),
    ('cpx10_limite_400', '    let motivo: i8 = if (cpx10_w(l + 8) as i32) < cpx10_w(l + 6) as i32 - 500 /* cpx10: o limite pela avaliação +8 */ && {',
     '    let motivo: i8 = if (cpx10_w(l + 8) as i32) < cpx10_w(l + 6) as i32 - 400 /* mutacao: o limite a 400 */ && {',
     'cpx_10 motivo pela avaliacao'),
]
