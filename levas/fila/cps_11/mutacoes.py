# Mutacoes. O calendario guardado e o das partidas que o jogo marca, adia e remarca (amistosos, TV, copas): a busca sem
# achar a copia guardada (330 chamadas ate 2002-06-01: quem remarca ou confere o jogo passa a trata-lo como novo), guardar
# sem acrescentar a copia (123: o jogo some do calendario e a partida nao acontece) e tirar sem tirar (468: o jogo
# remarcado fica tambem na data antiga). Os ramos ficam depois de todas as condicoes, colados no efeito.
MUTACOES = [
    ('cps11_nao_acha', 'return item; // cps11: a cópia guardada',
     'return 0; // mutacao: nao acha',
     'cps_11 achou o jogo guardado'),
    ('cps11_nao_guarda', 'if copiar(l, f, 0x4f) == 0 { // cps11: guardar',
     'if false { // mutacao: nao guarda',
     'cps_11 jogo guardado'),
    ('cps11_nao_tira', 'let r = tirar(fixture_list_in_block(b, f), f); // cps11: tirar',
     'let r = 0u32; // mutacao: nao tira',
     'cps_11 jogo tirado'),
]
