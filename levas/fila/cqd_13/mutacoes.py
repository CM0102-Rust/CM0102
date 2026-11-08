# Mutacoes (cqd_13): as tres na 0x868c50 (4 chamadas ate 2002-06-01, os campeoes das ligas em maio). Nas carreiras de
# 01/11/2026 dezenas de jogadores passaram do limiar (contador "cqd_13 jogador destacado do campeao"); a injury_word_0c do
# campeao so subiu numa delas, por isso nao serve. Com todos acima do limiar, os de media baixa ganham o historico, moral
# +20 em vez de +10 e +100 em vez de +50 no contrato (e a renovacao); o moral da ficha (+0x45, 0x544710) pesa nas
# partidas e o delta 0 nao sorteia (a contagem de sorteios muda); o moral do contrato (+0x44) pesa na felicidade e nos
# pedidos de saida. Na 0x86a800 (as finais das copas) o laco do elenco rodou pouco (para quando a injury_word_0c do campeao
# passa de peso x 400) e a injury_word_0c subiu uma ou duas vezes: por isso as mutacoes ficaram na liga. Podem escapar se o
# efeito de maio nao mudar nada ate 2002-06-01 (o moral do contrato e o mais fraco dos tres).
MUTACOES = [
    ('cqd13_todos_destacados', '        if f > limiar as f64 { // cqd13: acima do limiar',
     '        if f >= 0.0 { // mutacao: todos acima do limiar',
     'cqd_13 jogador destacado do campeao'),
    ('cqd13_moral_zero', '            moral(p, 0x14, 1); // cqd13: o moral do jogador destacado',
     '            moral(p, 0, 1); // mutacao: moral sem delta',
     'cqd_13 jogador destacado do campeao'),
    ('cqd13_contrato_50', '            add_contract_morale(k, 100); // cqd13: o contrato do jogador destacado',
     '            add_contract_morale(k, 50); // mutacao: +50 no contrato',
     'cqd_13 contrato do destacado'),
]
