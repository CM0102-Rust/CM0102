# Mutacoes do acrescentar as preferencias: cada uma desloca a lista em uma vaga (a pessoa entra na vaga seguinte; a
# terceira cai na primeira da lista oposta). A da 0x547a10 (relacoes do elenco, diaria) a carreira ate 2002-06-01 deve
# acusar. As do clube dependem de a 0x547870 (registros de clubes e de pessoas) e a 0x547940 (disputas, midia, 0x7daf50)
# rodarem na carreira: PODEM NAO acusar (o comparando acusa na primeira chamada).
MUTACOES = [
    ('dbq_clube_favorita', '    dbq1_acrescentar(clube + 0x9b, clube + 0xa7, pessoa, dbq1_tem(clube + 0x9b, pessoa), forcar) // as favoritas do clube',
     '    dbq1_acrescentar(clube + 0x9f, clube + 0xa7, pessoa, dbq1_tem(clube + 0x9b, pessoa), forcar) // mutacao: uma vaga adiante'),
    ('dbq_clube_rejeitada', '    dbq1_acrescentar(clube + 0xa7, clube + 0x9b, pessoa, dbq1_tem(clube + 0xa7, pessoa), forcar) // as rejeitadas do clube',
     '    dbq1_acrescentar(clube + 0xab, clube + 0x9b, pessoa, dbq1_tem(clube + 0xa7, pessoa), forcar) // mutacao: uma vaga adiante'),
    ('dbq_pessoa_favorita', '    dbq1_acrescentar(pref + 0x1c, pref + 0x28, outra, dbq1_tem(pref + 0x1c, outra), forcar) // as favoritas da pessoa',
     '    dbq1_acrescentar(pref + 0x20, pref + 0x28, outra, dbq1_tem(pref + 0x1c, outra), forcar) // mutacao: uma vaga adiante'),
]
