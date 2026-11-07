# FORA (agente C, 07/11/2026)

As posições previstas dos doze estaduais (o slot `set_predicted_positions` de `bra_reg_*.cpp`: a antiga `cqu_4` com
Baiano, Centro-Oeste, Gaúcho e Goiano, esta `cqv_1` com Mineiro, Norte, Nordeste e Paranaense e a `cqv_2` com
Pernambucano, Carioca, Catarinense e Paulista) rodam na carreira de 8 ligas (na criação e na virada) e passaram na
comparação de 8 ligas até 2002-04-30 sem divergência, mas as mutações escaparam todas: o 1º e o 2º trocados (Baiano),
sem lugares divididos (Centro-Oeste) e até sem ordenar pela força (Gaúcho) deram a mesma carreira até 2002-06-01
(impressão f29c85195c2a534f). Quem lê a posição prevista (+0x72 do registro na Comp_Stats) é a média de público do fim
da temporada (0x5d6280, cqt_4), que nos estaduais só roda depois de junho: é o caso "registros que ninguém lê até junho
escapam". Ficam fora até haver uma prova que alcance esse efeito (uma carreira além do fim dos estaduais de 2002). O
código continua aqui como estava; a `cqu_4` antiga se regenera com o gerador do agente C (gera_prev.py).
