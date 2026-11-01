# Mutacoes (cpx_8), na tatica de cada jogo (0x896660 pela formacao 0x6e0230, todo jogo dos clubes da maquina): o bloco
# pronto da tatica vizinha (os clubes com tatica pronta jogam com outra formacao e instrucoes), o bloco proprio de outro
# clube (indice/2: os clubes de tatica propria a partir do indice 2 jogam com a de outro) e, no clube humano das ferias,
# a vaga do time trocada (0 <-> 1, principal e reserva). As duas primeiras mudam o jogo e tudo o que sorteia depois; a
# terceira pode escapar se o humano das ferias nao passar pelo ramo (contador "cpx_8 tatica do humano") ou se as
# vagas 0 e 1 tiverem o mesmo bloco (o reset do registro poe a pronta 0 em todas). Os contadores "cpx_8 tatica pronta"
# e "cpx_8 tatica propria" mostram qual ramo da maquina roda.
MUTACOES = [
    ('cpx8_pronta_vizinha', '        cpx8_d(this + 0x5f9).wrapping_add(indice.wrapping_mul(0x18e3) as usize) // cpx8: o bloco pronto',
     '        cpx8_d(this + 0x5f9).wrapping_add(((indice + 1) % prontas).wrapping_mul(0x18e3) as usize) // mutacao: a pronta vizinha',
     'cpx_8 tatica pronta'),
    ('cpx8_propria_de_outro', '        cpx8_d(this + 0x5f1).wrapping_add(indice.wrapping_sub(prontas).wrapping_mul(0x18e3) as usize) // cpx8: o bloco próprio',
     '        cpx8_d(this + 0x5f1).wrapping_add((indice.wrapping_sub(prontas) / 2).wrapping_mul(0x18e3) as usize) // mutacao: a propria de outro',
     'cpx_8 tatica propria'),
    ('cpx8_vaga_trocada', '            escolher(gerente, time, clube, cpx8_d(clube + 0xcf)); // cpx8: a vaga pelo time',
     '            escolher(gerente, time ^ 1, clube, cpx8_d(clube + 0xcf)); // mutacao: a vaga trocada',
     'cpx_8 tatica do humano'),
]
