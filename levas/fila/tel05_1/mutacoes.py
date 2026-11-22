# Telas do clube (5a): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Esta prova o
# comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat: a visao com que a tela da troca abre (o comparador da
# abertura confere os params da original contra o plano). Caminho: tela de proposta por um jogador > Add Player.
# O tratador e a saida so rodam a original no modo comparando (nao ha mutacao que o comparador pegue).
MUTACOES = [
    ('tel05_1_visao_da_troca',
     'const PART_EXCHANGE_OPEN_PARAMS: [u32; 9] = [0, 0, 0, 0x41, 8, 6,',
     'const PART_EXCHANGE_OPEN_PARAMS: [u32; 9] = [0, 0, 0, 0x42, 8, 6,'),
]
