# Mutacoes (cx13_6), com a Polonia ate 2002-08-01 (a repescagem em junho de 2002):
# - o clube da Primeira que vence a repescagem cai assim mesmo (status 3: quem cai muda; acusada tambem pelo
#   comparador, o status da linha contra o plano);
# - a faixa da repescagem com o melhor 1 (acusada pelo comparador, lado a lado);
# - os textos da repescagem trocados (acusada pelo comparador, compare_text_writer).
MUTACOES = [
    ('cx13_6_primeira_cai_mesmo_vencendo',
     'return PlayOffRecordPlan { record: None, row: Some((row, 0xfe)), result }; // cx13_6: the upper division club stays up',
     'return PlayOffRecordPlan { record: None, row: Some((row, 3)), result }; // mutation: relegated anyway',
     'cx13_6 play-off status'),
    ('cx13_6_faixa_da_repescagem_em_1',
     'b = 2; // cx13_6: the play-off band',
     'b = 1; // mutation: the title band',
     'cx13_6 polish season progress'),
    ('cx13_6_textos_trocados',
     'return play_off_club_text(club_id, if first { 0x987594 } else { 0x989c18 }, text); // cx13_6: avoid relegation / win promotion',
     'return play_off_club_text(club_id, if first { 0x989c18 } else { 0x987594 }, text); // mutation: swapped',
     'cx13_6 polish play-off text'),
]
