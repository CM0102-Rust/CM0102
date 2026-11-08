# Mutacoes (cpw_4), todas na oferta de contrato acrescentada (0x4cc490, 3032 chamadas ate 2002-06-01 na cobertura de
# 17/10; "cpq_8 loan offer" passa de 256 e cada uma chega aqui): o contador de ofertas da pessoa (+8 do
# registro de 79 bytes do general.dat, no banco) subindo 2 (e nao 1) nunca volta a zero quando as ofertas saem; sem
# marcar os termos oferecidos ao jogador da oferta de transferencia (0x8c1f80), a oferta fica no estado 6 e a
# transferencia nao anda; sem agendar a decisao (0x4cd230, com sorteio), o jogador nao responde a oferta. As tres mudam
# o banco, as transferencias e os sorteios.
MUTACOES = [
    ('cpw4_contador_2', '        *c = (*c).wrapping_add(1); // cpw4: o contador de ofertas da pessoa',
     '        *c = (*c).wrapping_add(2); // mutacao: sobe 2',
     'cpw_4 new offer'),
    ('cpw4_sem_termos', '            marcar(cpw4_d(0xde1f54), vaga); // cpw4: os termos oferecidos ao jogador',
     '            let _ = marcar; // mutacao: sem marcar os termos',
     'cpw_4 terms to the player'),
    ('cpw4_sem_decisao', '        decisao(this, cpw4_d(termos) as u32, vaga); // cpw4: a decisão agendada',
     '        let _ = decisao; // mutacao: sem a decisao',
     'cpw_4 schedule decision'),
]
