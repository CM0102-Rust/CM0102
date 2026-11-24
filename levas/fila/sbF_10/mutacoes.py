# Mutacoes (sbF_10), carreira ENG com a base NORMAL ate 2002-06-01. Nenhuma abre caixa.
# - o premio do FA Trophy de 200.000 e nao 300.000: o comparador do construtor acusa (confere o premio contra a
#   constante; a EngFaTrophy::set_up_rounds do cx02_4 grava 300.000 de novo, entao a carreira nao muda).
# A virada (start_new_season) e o molde provado das copas (start_new_cup_season): sem mutacao propria, o que ela monta
# so e usado na edicao seguinte, depois da data da cadeia.
MUTACOES = [
    ('sbF_10_premio_200_mil',
     'entrants: 0x5712b0, fails_when_negative: false, prize: Some(0x493e0), rank: true,',
     'entrants: 0x5712b0, fails_when_negative: false, prize: Some(0x30d40), rank: true, // mutation\n   ',
     'sbF_10 fa trophy new'),
]
