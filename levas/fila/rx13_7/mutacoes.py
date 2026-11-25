# Mutacoes (rx13_7), pela carreira ENG ate 2002-06-01 (as queixas e as marcas mudam a moral, os pedidos de
# transferencia e as noticias; no modo de comparacao a porta do depois do jogo fica com os enderecos, entao a prova e a
# carreira Rust):
# - as queixas nunca apagadas (o pedido de transferencia e os vinculos seguem);
# - as marcas de evento nunca apagadas.
MUTACOES = [
    ('rx13_7_queixas_ficam',
     'contract.set_grievances(old & !bits); // rx13_7: the bits cleared',
     'contract.set_grievances(old); // mutation: nothing cleared',
     'rx13_7 grievances cleared'),
    ('rx13_7_marcas_ficam',
     'contract.set_flags(contract.flags() & !bits); // rx13_7: the flag bits cleared',
     'contract.set_flags(contract.flags()); // mutation: nothing cleared',
     'rx13_7 flags cleared'),
]
