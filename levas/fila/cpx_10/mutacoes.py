# Mutacoes (cpx_10), na noticia da pressure (0x6a0e70, por pensar em demitir o tecnico 0x69aba0, ja em Rust): a data de
# hoje nao vai para +0x18 da linha da diretoria (a 0x69aba0 le essa data nos prazos de 14 e 30 dias: a noticia, com os
# sorteios dela, volta mais cedo), o boato sem o sorteio(10) (um sorteio a menos por noticia) e o limit da primeira
# avaliacao a confianca - 400 em vez de - 500 (o sorteio(3) roda em outros clubes). As tres mudam a contagem dos sorteios
# e tudo o que sorteia depois. Escapam se a noticia da pressure nao sair ate 2002-06-01 (contadores "cpx_10 data da
# pressure" e "cpx_10 rumour draw"); a terceira tambem se nenhuma avaliacao +8 cair entre os dois limites (contador
# "cpx_10 reason by evaluation", que conta so os casos abaixo do limit original). A saida e o tecnico novo so mudam as
# caixas dos humanos: passageiras (contadores "cpx_10 departure news" e "cpx_10 new manager news").
MUTACOES = [
    ('cpx10_sem_data', '    copiar(board_row_of(this, clube) + 0x18, data.as_mut_ptr() as usize, 0xae2c90); // cpx10: a data da notícia na linha',
     '    let _ = (copiar, data); // mutacao: sem a data',
     'cpx_10 pressure date'),
    ('cpx10_boato_sem_sorteio', '    let mut boato = cpx10_sortear(10); // cpx10: o sorteio do boato',
     '    let mut boato = 0; // mutacao: sem o sorteio',
     'cpx_10 rumour draw'),
    ('cpx10_limite_400', '    let motivo: i8 = if (cpx10_w(l + 8) as i32) < cpx10_w(l + 6) as i32 - 500 /* cpx10: o limit pela avaliação +8 */ && {',
     '    let motivo: i8 = if (cpx10_w(l + 8) as i32) < cpx10_w(l + 6) as i32 - 400 /* mutacao: o limit a 400 */ && {',
     'cpx_10 reason by evaluation'),
]
