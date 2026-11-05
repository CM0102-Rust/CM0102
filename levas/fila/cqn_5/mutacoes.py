# Mutacoes (cqn_5), no calendario real de 2001/02 da Serie A e da Serie B (a criacao da carreira de 8 ligas).
# Na carreira de 8 ligas as duas listas sempre caem: metade dos nomes de 2001/02 nao existe no banco minimo (provado
# pelo agente, 05/11: Serie A 306 jogos com clubes sem achar ja nos jogos 2, 4, 6...; Serie B nos 0, 2, 3...). Assim,
# mexer nos jogos (o mando, as datas) ou derrubar a lista escapa (provado: "lista derrubada" e "mando invertido"
# ficaram IGUAIS A BASE). A unica mutacao que a carreira ve e a que mantem a lista com clubes sem achar: a montagem dos
# jogos (0x6835c0) usa a lista e o jogo muda (ou cai) no primeiro dia.
MUTACOES = [
    ('cqn5_lista_nunca_cai',
     'if cqn5_d(f + 8) == 0xffff_ffff || cqn5_d(f + 0xc) == 0xffff_ffff { // cqn5: um clube sem achar derruba a lista',
     'if false { // mutacao: a lista nunca cai',
     'cqn_5 calendario fixo'),
]
