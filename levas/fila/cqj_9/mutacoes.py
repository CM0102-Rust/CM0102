# Mutacoes (cqj_9): o destino de cada clube no fim das fases (slot 18), que roda no fim de cada fase das ligas
# brasileiras (Serie A e C ate dezembro de 2001, estaduais ate maio de 2002). A linha da tabela (+0x37) e os registros
# de carreira do tecnico (o historico e os pontos dele) mudam, e a memoria do jogo (impressao diaria) com eles.
# - Serie A: o campeao do mata-mata nao ganha o registro de campeao (historico e pontos do tecnico). (O destino 0 em
#   vez de 5 na linha escapou na carreira de 8 ligas ate 2002-06-01, 04/11: nada le o destino do campeao antes da
#   virada.)
# - Serie C: quem sobe fica com o destino 1 (play-off de cima) em vez de 0.
# - Estaduais com o slot do Baiano (sete): o rebaixado nao ganha o registro de rebaixamento.
MUTACOES = [
    ('cqj9_campeao_serie_a', 'add_champions_record(clube, comp); // cqj9: o registro do campeão da Série A',
     '// mutacao: sem o registro do campeão da Série A', 'cqj_9 serie a champion'),
    ('cqj9_promovido_serie_c', '*((achada + 0x37) as *mut u8) = 0; // cqj9: o destino de quem sobe da Série C',
     '*((achada + 0x37) as *mut u8) = 1; // mutacao: o destino de quem sobe da Série C', 'cqj_9 serie c promoted'),
    ('cqj9_rebaixado_baiano', 'bra_nat_first_add_relegation_record(clube, comp); // cqj9: o rebaixado do Baiano',
     '// mutacao: sem o registro do rebaixado do Baiano', 'cqj_9 baiano relegated'),
]
