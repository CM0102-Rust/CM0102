# Mutacoes (sx18_1), pela carreira ENG ate 2002-06-01 (cartoes em toda rodada; as suspensoes tiram jogadores das
# escalacoes seguintes):
# - a escala do orgao ingles 5 -> 8 (as regras inglesas nao usam a escala: acusada pelo comparador do construtor, que
#   confere os campos que a original grava contra a classe);
# - o primeiro limite de amarelos 5 -> 4;
# - a suspensao inglesa comecando hoje (e nao em 14 dias);
# - o europeu suspendendo a cada segundo amarelo (copas europeias).
MUTACOES = [
    ('sx18_1_escala',
     'scale: 5,\n    nation: Some(KEY_NATION_ENGLAND)',
     'scale: 8, // mutation\n    nation: Some(KEY_NATION_ENGLAND)',
     'sx18_1 english ruling body built'),
    ('sx18_1_quarto_amarelo',
     '0..=4 => 5, // sx18_1: the first booking ban',
     '0..=3 => 4, 4 => 5, // mutation: the fourth booking bans',
     'sx18_1 english booking ban'),
    ('sx18_1_comeca_hoje',
     'start: ban_start_in(14) }); // sx18_1: the English ban starts in two weeks',
     'start: ban_start_in(0) }); // mutation: from today',
     'sx18_1 english red card'),
    ('sx18_1_europeu_segundo',
     'plain_card_verdict(row, entry, false, 3, [0xff, 0xff, 1, 0xff]) // sx18_1: every third European booking',
     'plain_card_verdict(row, entry, false, 2, [0xff, 0xff, 1, 0xff]) // mutation: every second',
     'sx18_1 european cards'),
]
