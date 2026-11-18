# Mutacoes (mwf_17), na virada italiana de meados de 2002. Sem a mutacao do reagrupamento: sem ele a C1 A ficaria com
# mais clubes do que a tabela dela (a caixa de erro). A do ano + 2 saiu em 18/11/2026: o ano so e lido quando a Serie A
# 2002/03 comeca, depois da data da cadeia (ver docs/COMO_ESCREVER_UM_ITEM.md). Esta tira o acesso e o rebaixamento:
# os clubes ficam na divisao antiga, o que muda o mercado e as financas logo depois da virada (como mwe15_sem_troca).
MUTACOES = [
    ('mwf17_sem_acesso_e_rebaixamento',
     'chain(this.addr(), 1); // mwf17: the Italian promotion and relegation',
     '// mutation: no promotion and relegation',
     'mwf_17 serie a rollover'),
]
