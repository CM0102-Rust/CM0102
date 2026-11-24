# Mutacoes (sx18_11), pela carreira com LIGAS 'ENG ESP ITA FRA BRA JPN AUS USA' ate 2002-06-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_11_terceiro_amarelo_copa',
     'japan_card_verdict(MatchRecordRow(row), e, 2, 0,',
     'japan_card_verdict(MatchRecordRow(row), e, 3, 0,',
     'sx18_11 japanese cup cards'),
    ('sx18_11_quarto_australiano',
     'if count == 5 || (count >= 7',
     'if count == 4 || (count >= 7 // mutation\n           ',
     'sx18_11 australian booking ban'),
]
