# O progresso na Copa da Premier galesa e o mata-mata das duas copas galesas (wel_prm_cup.cpp, wel_lge_cup.cpp; leitor
# cx03, rodada 5, 24/11/2026). Competicoes de Gales: LIGAS do grupo ENG SCO WAL, ate 2002-08-01 (os dois mata-matas
# rodam uma vez cada no mundo ate essa data).
# - 0x9290b0 WelPrmCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: fase < 1: atual e
#   melhor 1..3 -> 1, senao 9; pior 1, 2 -> 5, 3 -> 9, senao 13; senao como vem (store_club_progress do cx03_9).
# - 0x926d00 WelLgeCup::create_knockout_stage thiscall() ret 0 (a fase seguinte, virtual 0x28): malloc 0x20 (a caixa
#   da linha 0x27e); os 6 primeiros (a tabela propria +0xb1 e as fases 0..4: 0x6826d0 push 0 -> thiscall, ret 4); o
#   ranking dos segundos num League_Stage temporario (0x944cf1 de 0xee; 0x68b100 com 22 argumentos, thiscall, ret 0x58:
#   a copa, 6, 0, 0, +4, 0, 0, os bytes +0xc2 e +0xc3, 0, &+0xc5, &+0xbe, o ano, -2, 0, 0, 1, 0, 2, -1, 0, 2), a linha
#   2 de cada grupo copiada (0x3b bytes, rep movsd/movsw/movsb) com +9 e +5 zerados, League::sort_table 0x6827d0 (push
#   0 -> thiscall, ret 4); os 2 primeiros do ranking; a virtual 0x3c (fase 5), o Cup_Stage (0x944cf1 de 0xb2; 0x522e00
#   com 15 argumentos, thiscall, ret 0x3c: a copa, 8, o vetor, rodadas, +4, a tabela, o ano, 5, 1, extra, 0x14, 0, 0,
#   0, 0) em [+0xc][5]; free do vetor e da tabela; o ranking destruido (virtual 0 com 1) ANTES de conferir a fase (por
#   isso nao usa create_knockout_stage do ita_cup: a ordem das liberacoes muda o heap); sem fase a caixa da linha 0x2b2;
#   com ela Cup::draw_round 0x51c410 (push 0 -> thiscall, ret 4).
# - 0x928650 WelPrmCup::create_knockout_stage thiscall() ret 0: malloc 0x20 (a caixa 0x27c); a tabela propria (linha 1),
#   os inscritos 8..11 de +0xee e as linhas 1 e 2 da fase 0, intercalados; as sementes 2,1,2,1,2,1,2,1 (bytes locais);
#   create_knockout_stage (ita_cup) com a fase 1 e a caixa 0x297 (a mesma sequencia, conferida).
# Usa compare_club_progress do cx02_4 e store_club_progress do cx03_9 (integrar depois deles).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o progresso lado a lado; os mata-matas so a original (alocam e sorteiam), conferindo a fase no slot.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_prm_cup_set_club_progress', 0x9290b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('wel_lge_cup_create_knockout_stage', 0x926d00, [0x6a, 0xff, 0x68, 0xac, 0x5d, 0x96, 0x00]),
           ('wel_prm_cup_create_knockout_stage', 0x928650, [0x6a, 0xff, 0x68, 0x3e, 0x5e, 0x96, 0x00])]
DOCS = ('o progresso na Copa da Premier galesa e o mata-mata das duas copas galesas',
        'the Welsh Premier Cup progress and the knockout stages of the two Welsh league cups',
        [('wel_prm_cup_set_club_progress', 'WelPrmCup::set_club_progress', '0x9290b0', 'wel_prm_cup.cpp', 'thiscall, ret 0x14',
          'regra: nos grupos da Copa da Premier galesa (fases abaixo de 1) os lugares viram códigos de progresso: 1º a 3º dão 1 e o resto 9 no atual e no melhor; no pior 1º e 2º dão 5, 3º dá 9 e o resto 13; do mata-mata em diante os códigos ficam como vêm',
          'rule: in the Welsh Premier Cup groups (stages below 1) the places become progress codes: places 1 to 3 give 1 and anything else 9 for the current and best codes; for the worst places 1 and 2 give 5, place 3 gives 9 and anything else 13; from the knockout on the codes stay as they come'),
         ('wel_lge_cup_create_knockout_stage', 'WelLgeCup::create_knockout_stage', '0x926d00', 'wel_lge_cup.cpp', 'thiscall',
          'regra: o mata-mata da Copa da Liga galesa (fase 5) recebe os seis campeões de grupo e os dois melhores segundos (classificados numa tabela própria, pelos pontos e pelos critérios de desempate da copa) e sorteia as quartas',
          'rule: the Welsh League Cup knockout (stage 5) takes the six group winners and the two best runners-up (ranked in a table of their own, by points and the cup\'s tie-breaks) and draws the quarter-finals'),
         ('wel_prm_cup_create_knockout_stage', 'WelPrmCup::create_knockout_stage', '0x928650', 'wel_prm_cup.cpp', 'thiscall',
          'regra: o mata-mata da Copa da Premier galesa (fase 1) tem oito clubes: os dois primeiros dos dois grupos e os quatro que entram ali (os inscritos 8 a 11), com sementes alternadas 2 e 1, e sorteia as quartas',
          'rule: the Welsh Premier Cup knockout (stage 1) has eight clubs: the top two of the two groups and the four clubs entering there (entrants 8 to 11), seeded alternately 2 and 1, and draws the quarter-finals')])
CAMPOS = []
INTERESSA = []
