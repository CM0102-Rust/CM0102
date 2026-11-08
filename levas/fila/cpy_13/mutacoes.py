# Mutacoes (cpy_13), na liberacao sem custo (0x8c1af0; pela fila das transferencias livres, "cps_3 released" e
# "cps_3 compensation" 8 na base). Sem a compensacao (0x599920 com 0) o clube nao paga e o saldo dele muda (a
# cps3_compensacao_em_dobro, na mesma conta, foi acusada em 2001-282). O jogador liberado que fica na escalacao do time
# principal do clube atual (0x896c60 com 1, 1) muda as escalacoes seguintes. Sem a situacao nas listas de observacao
# (0x843e10 evento 4) os clubes que observavam o jogador seguem com a entrada antiga e as ofertas mudam.
MUTACOES = [
    ('cpy13_sem_compensacao', '        pagar(fin, comp); // cpy13: a compensação ao clube',
     '        pagar(fin, 0); // mutacao: sem a compensação',
     'cpy_13 compensation paid'),
    ('cpy13_fica_na_escalacao', '            escalacao(cpy13_d(0xde1d08), cpy13_d(p + 0x39), p, 1, 1); // cpy13: as escalações',
     '            let _ = escalacao; // mutacao: fica na escalação do time principal',
     'cpy_13 leaves the current club'),
    ('cpy13_sem_listas', '        listas(cpy13_d(this + 0x856), p, clube, 4, 2); // cpy13: a situação nas listas de observação',
     '        let _ = listas; // mutacao: sem a situação nas listas',
     'cpy_13 released'),
]
