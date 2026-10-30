# Mutacoes. O surto novo com 1 em 5 de ficar de fora em vez de 1 em 4 (o sorteio muda de faixa: outros surtos entram;
# comparando, o plano usa a mesma regra mas a original nao: a conta difere). Os surtos espalhando 0 a 1 doente em vez
# de 0 a 2 e os surtos saindo com seis dias em vez de cinco (so o modo Rust e a carreira: o comparador roda a
# original; doentes a mais ou a menos mudam as escalacoes e as partidas). Se nenhum surto acontecer ate 2002-06-01,
# os ramos dizem e as mutacoes sao puladas.
MUTACOES = [
    ('cpp8_surto_novo', '} else if cpp8_sorteio(4) == 0 { // 1 em 4 fica de fora',
     '} else if cpp8_sorteio(5) == 0 { // mutacao: 1 em 5 fica de fora',
     'cpp_8 surto novo'),
    ('cpp8_espalhar', 'Some(cpp8_sorteio(3) as u8) // 0 a 2 doentes',
     'Some(cpp8_sorteio(2) as u8) // mutacao: 0 a 1 doente',
     'cpp_8 espalhar'),
    ('cpp8_surto_antigo', 'datar(0xae2c90, &mut d, 5); // cinco dias atrás',
     'datar(0xae2c90, &mut d, 6); // mutacao: seis dias',
     'cpp_8 surto antigo'),
]
