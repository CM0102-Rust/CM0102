# Mutacoes (gra_1). O texto nao volta para a simulacao: acusadas pela comparacao (compare_text_writer compara o texto,
# os vinculos e o gerador), como as de texto aceitas em 12/11/2026.
# - a lesao num jogo (sem o registro do jogo): a duracao e o nome da lesao trocados de lugar no texto;
# - a lesao no treino (clube do tecnico humano ou sem clube): idem;
# - o sorteio da copa: o vinculo do clube sorteado aponta para o adversario;
# - o sorteio dos grupos: o alvo do vinculo da fase com a competicao deslocada de 10 bits (e nao 11).
MUTACOES = [
    ('gra1_lesao_no_jogo_duracao',
     '0x64, np, dp, ip, comp + competition::NAME,',
     '0x64, np, ip, dp, comp + competition::NAME,',
     'gra_1 injury in a match'),
    ('gra1_lesao_no_treino_duracao',
     'f(NEWS_TEXT_BUFFER, 0xa654a4, b, a, gwd, 0x64, np, dp, ip, wd);',
     'f(NEWS_TEXT_BUFFER, 0xa654a4, b, a, gwd, 0x64, np, ip, dp, wd);',
     'gra_1 injury in training'),
    ('gra1_sorteio_vinculo',
     'news_link(out, drawn + club::SHORT_NAME, 0x7d5, drawn);',
     'news_link(out, drawn + club::SHORT_NAME, 0x7d5, opponent);',
     'gra_1 cup draw body'),
    ('gra1_grupos_vinculo_da_fase',
     '(ci << 11)) << 11)',
     '(ci << 10)) << 11)',
     'gra_1 group draw body'),
]
