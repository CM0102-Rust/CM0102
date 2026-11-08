# Mutacoes. A projecao entra na confianca semanal da diretoria de cada clube (0x697c40 pela 0x4ae300): posicoes
# projetadas diferentes mudam a confianca, as demissoes de tecnico e o que vem delas. A ordem pela injury_word_0c invertida
# (so na escala do primeiro terco da temporada, que a carreira atravessa a partir de julho de 2001: o ramo diz); a
# ordem pelos pontos invertida (as tres posicoes saem do fim da tabela); e a posicao na forma normal uma abaixo (o ramo
# "cpr_11 projection" prova que a rotina roda na carreira). As ordens sao quentes: sem ramo.
MUTACOES = [
    ('cpr11_reputacao_invertida', 'rb - ra // o clube de maior reputação primeiro',
     'ra - rb // mutacao: a menor primeiro',
     'cpr_11 projection by injury_word_0c'),
    ('cpr11_pontos_invertidos', 'cpr11_w(b + 0xa) as i32 - cpr11_w(a + 0xa) as i32 // os mais pontos primeiro',
     'cpr11_w(a + 0xa) as i32 - cpr11_w(b + 0xa) as i32 // mutacao: os menos primeiro'),
    ('cpr11_normal_acima', 'normal = (normal & !0xff) | (k as u8).wrapping_add(1) as u32; // a posição na forma normal',
     'normal = (normal & !0xff) | (k as u8).wrapping_add(2) as u32; // mutacao: uma abaixo',
     'cpr_11 projection'),
]
