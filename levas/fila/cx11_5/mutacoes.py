# Mutacoes (cx11_5), com a Italia e a base NORMAL ate 2002-09-30:
# - a repescagem de acesso da C2/A recebe o calendario do play-out (1 rodada em vez de 2: sem a final, outro clube
#   sobe em maio de 2002; acusada tambem pelo comparador, lado a lado);
# - o play-out da C2/B recebe o calendario da repescagem (2 rodadas: os perdedores da primeira jogam de novo; outros
#   caem).
MUTACOES = [
    ('cx11_5_c2a_repescagem_de_uma_rodada',
     'create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea654, rows: [0xab, 0x155, 0x18f] }) // cx11_5: the C2/A schedule',
     'create_c1_schedule(League(this), if stage == 0 { 1 } else { stage }, rounds, extra, league, &C1BracketVariant { file: 0x9ea654, rows: [0xab, 0x155, 0x18f] }) // mutation: the play-out schedule',
     'cx11_5 serie c2 schedule'),
    ('cx11_5_c2b_play_out_de_duas_rodadas',
     'create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea75c, rows: [0xab, 0x155, 0x18f] }) // cx11_5: the C2/B schedule',
     'create_c1_schedule(League(this), if stage == 1 { 0 } else { stage }, rounds, extra, league, &C1BracketVariant { file: 0x9ea75c, rows: [0xab, 0x155, 0x18f] }) // mutation: the play-off schedule',
     'cx11_5 serie c2 schedule'),
]
