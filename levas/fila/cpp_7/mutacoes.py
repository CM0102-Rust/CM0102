# Mutacoes. Sem contar a tentativa da oferta recusada (o sorteio(2 x tentativas) muda de faixa e o "mais de uma
# tentativa" demora: ofertas que seriam rejeitadas sao reagendadas; muda a carreira). O jogador da oferta continua nas
# outras ofertas (depois da transferencia, outros clubes seguem com oferta por ele). A oferta em dinheiro com 1000 a
# mais (o valor da transferencia muda o dinheiro dos dois clubes). As tres so no modo Rust e na carreira: os
# comparadores rodam so a original.
MUTACOES = [
    ('cpp_tentativa', '*((oferta + 0x2e) as *mut u8) = cpp7_b(oferta + 0x2e).wrapping_add(1); // mais uma tentativa',
     '// mutacao: sem contar a tentativa',
     'cpp_7 recusa'),
    ('cpp_soltar', 'tirar(this, cpp7_pessoa(cpp7_d(oferta + 0xc)), oferta, 0, 0); // o jogador da oferta',
     '// mutacao: o jogador segue nas outras ofertas',
     'cpp_7 soltar jogadores'),
    ('cpp_valor', 'iniciar(oferta, jogador, clube, valor); // o valor da oferta',
     'iniciar(oferta, jogador, clube, valor.wrapping_add(1000)); // mutacao: 1000 a mais',
     'cpp_7 oferta em dinheiro'),
]
