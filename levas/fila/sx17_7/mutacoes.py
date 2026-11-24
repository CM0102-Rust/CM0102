# Mutacoes (sx17_7), item de TELAS: a carreira automatica nao passa por estas rotinas; servem para provar o comparador
# numa sessao do Lucas jogando comparando (o relatorio do treinador na tela de um jogador):
# - o limiar dos pontos fortes 12 + desvio em vez de 14 (o comparador acusa as palavras de pontos fortes);
# - o jovem sem a barra mais alta (o comparador do judge_notices acusa com jogadores abaixo de 22 anos);
# - a frase de bolas paradas com maus pontos trocada pela de so maus pontos (o comparador do texto acusa).
MUTACOES = [
    ('sx17_7_limiar_forte',
     'let high = offset + 14; // sx17_7: the strength threshold',
     'let high = offset + 12; // mutation: a lower threshold'),
    ('sx17_7_jovem_facil',
     'bar += (22 - age as i32) * 2; // sx17_7: the young player is harder to read',
     'bar += 0; // mutation: the young player is as easy to read'),
    ('sx17_7_frase_trocada',
     '(false, true, true) => story_lang!(out, 0xa750b0, named, bad), // sx17_7: the set pieces and the bad points',
     '(false, true, true) => story_lang!(out, 0xa75048, scout, bad), // mutation: the wrong sentence'),
]
