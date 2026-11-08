# Mutacoes (com o ramo de cada uma). As listas de posicao dao os candidatos das listas dos clubes da IA (0x83a730 ->
# 0x83b6e0): mudar quem entra ou a ordem muda os alvos e as transferencias da carreira. Montar com a vaga vizinha
# (as listas ficam com jogadores de outra posicao; o comparando confere a vaga e acusa). Ordenar sem a band do
# vencimento (o jogador de contrato acabando passa a valer o valor e sobe na ordem). Os clubes interessados com a
# comparacao do valor invertida (o comparando acusa pela lista; a carreira pelas ofertas).
MUTACOES = [
    ('cpo_montar_vaga', 'if serve(p, vaga) == 0 {',
     'if serve(p, vaga ^ 1) == 0 { // mutacao: a vaga vizinha',
     'cpo_8 build lists'),
    ('cpo_ordenar_faixa', 'if k != 0 && band(k) == 0 {',
     'if k != 0 { // mutacao: sem a band do vencimento',
     'cpo_8 contract band'),
    ('cpo_interessados_valor', '&& valor > cpo8_i(jogador + 0x52) {',
     '&& valor <= cpo8_i(jogador + 0x52) { // mutacao: invertida',
     'cpo_8 interested club'),
]
