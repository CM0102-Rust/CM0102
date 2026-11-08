# Mutacoes. Cada clube que poe o jogador na lista conta dois (o limit de 5 chega antes: menos clubes miram cada
# jogador, as ofertas mudam). O contrato nao fica a venda (o +0x4f segue sem o 7: os clubes nao o tratam como listado;
# comparando, o plano acusa). O clube antigo nao recebe o premio por jogos (o dinheiro dele muda; so se houver premio
# pago ate 2002-06-01: PODE ESCAPAR, ver o ramo).
MUTACOES = [
    ('cpp_listas_conta', 'conta = conta.wrapping_add(1); // mais um clube com ele na lista',
     'conta = conta.wrapping_add(2); // mutacao: conta dois',
     'cpp_6 clube acrescentou'),
    ('cpp_a_venda', '*((c + 0x4f) as *mut u8) = (a & 0xf) | 0x70; // à venda',
     'let _ = a; // mutacao: sem o estado a venda',
     'cpp_6 por a venda'),
    ('cpp_premio', 'receber(f_antigo, valor as u32, (valor >> 31) as u32); // o clube antigo recebe',
     'let _ = (receber, f_antigo); // mutacao: o clube antigo nao recebe',
     'cpp_6 premio por jogos'),
]
