# As perguntas "mais um amarelo suspende?" dos orgaos disciplinares (vtable +0x14, is_one_booking_from_ban;
# rb_*.cpp; leitor sx19, rodada 5, 24/11/2026). Item de tela: so rodam pelas telas (0 chamadas na carreira do mundo
# ate 2002-08-01): RulingBody::check_one_booking_from_ban (0x7e8d60) as chama para a tela do clube (o elenco,
# Discipline::is_one_booking_from_ban 0x55d360 chamado de 0x484eb0) e RulingBody::get_ban_text (0x7e88d0) para a tela
# da pessoa (0x55d3c0 chamado de 0x87618a). Usa o molde do sx18_26 (one_booking_outputs, compare_one_booking):
# integrar depois dele.
# Caminho no jogo (para o Lucas provar): um jogo com a liga do pais carregada (Noruega, Polonia, Portugal, Russia,
# Escocia, Suecia, Turquia, Estados Unidos, Gales ou Espanha, que usa a de Gales): a tela do elenco de um clube dessa
# liga (a marca de "um amarelo da suspensao" na lista) e a tela de um jogador com cartoes (a linha de disciplina).
# Compartilhadas no original: 0x7e39c0 (Suecia) e a das duas suecas; 0x7e40b0 (Turquia) a das duas turcas; 0x7e4710
# (Gales) tambem a da liga espanhola.
# E a base que as chama: 0x7e8d60 RulingBody::check_one_booking_from_ban thiscall(id, &jogos, &dias, competicao) ret 0x10:
# 0x7e82e0(id, 0); sem entrada 0; senao a vtable +0x14 (entrada, &jogos, &dias, competicao).
# Fora da lista: 0x7e7850 RulingBody::is_one_booking_from_ban (xor eax, eax; ret 0x10: corpo vazio da base, 5 bytes).
# Convencoes conferidas na desmontagem: thiscall(entrada, &jogos, &dias, competicao) ret 0x10; grava jogos (1, ou na
# Escocia a soma das suspensoes por pontos + 1) e dias 0xff quando os ponteiros nao sao nulos; devolve 0/1:
# - Noruega: contagem (+4) == 2 ou > 3; Polonia: 3, 6, 9 ou impar a partir de 10; Portugal: (contagem + 1) % passo
#   == 0, passo 3 se [competicao] == [0x9cf7c0] (Second Division B portuguesa), senao 5; Russia: == 3; copa escocesa:
#   contagem + 1 >= 2; liga escocesa: jogos = (+0xa >> 3 & 7) + (+0xb & 7) + (+0xa & 7) + 1, devolve pontos (+8) + 2
#   >= 16; Suecia: par e nao zero; Turquia: (contagem + 1) % 4 == 0; EUA: pontos == 20; Gales: (contagem + 1) % 5 == 0.
# Defeito do original: o aviso russo no terceiro amarelo nunca se cumpre (o quarto nao suspende, ver sx19_9).
SAVE = False
TELAS = True
ROTINAS = [('rb_norway_league_is_one_booking_from_ban', 0x7e1390, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_poland_is_one_booking_from_ban', 0x7e1a10, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_portugal_is_one_booking_from_ban', 0x7e1db0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_russia_is_one_booking_from_ban', 0x7e2170, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_scotland_cup_is_one_booking_from_ban', 0x7e2440, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_scotland_league_is_one_booking_from_ban', 0x7e2930, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('rb_sweden_league_is_one_booking_from_ban', 0x7e39c0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_turkey_league_is_one_booking_from_ban', 0x7e40b0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_usa_is_one_booking_from_ban', 0x7e4390, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_wales_is_one_booking_from_ban', 0x7e4710, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('check_one_booking_from_ban', 0x7e8d60, [0x8b, 0x44, 0x24, 0x04, 0x56])]
_C = 'thiscall, ret 0x10'
_PT = 'diz se mais um amarelo suspende o jogador pelas regras {} ({}); grava 1 jogo e nenhum dia nas saídas'
_EN = 'tells whether one more booking bans the player under the {} rules ({}); writes 1 match and no days to the outputs'
DOCS = ('as perguntas "mais um amarelo suspende?" dos órgãos disciplinares',
        'the "does one more booking ban?" questions of the ruling bodies',
        [('rb_norway_league_is_one_booking_from_ban', 'RbNorwayLeague::is_one_booking_from_ban', '0x7e1390', 'ruling_body/rb_norway_league.cpp', _C,
          _PT.format('da liga norueguesa', 'contagem 2 ou acima de 3'), _EN.format('Norwegian league', 'count 2 or above 3')),
         ('rb_poland_is_one_booking_from_ban', 'RbPoland::is_one_booking_from_ban', '0x7e1a10', 'ruling_body/rb_poland.cpp', _C,
          _PT.format('polonesas', 'contagem 3, 6, 9 ou ímpar a partir de 10'), _EN.format('Polish', 'count 3, 6, 9 or odd from 10')),
         ('rb_portugal_is_one_booking_from_ban', 'RbPortugal::is_one_booking_from_ban', '0x7e1db0', 'ruling_body/rb_portugal.cpp', _C,
          _PT.format('portuguesas', 'contagem mais 1 múltipla de 3 na Segunda Divisão B, de 5 nas outras'), _EN.format('Portuguese', 'count plus 1 a multiple of 3 in the Second Division B, of 5 elsewhere')),
         ('rb_russia_is_one_booking_from_ban', 'RbRussia::is_one_booking_from_ban', '0x7e2170', 'ruling_body/rb_russia.cpp', _C,
          _PT.format('russas', 'contagem 3; o quarto amarelo não suspende de fato'), _EN.format('Russian', 'count 3; the fourth booking does not actually ban')),
         ('rb_scotland_cup_is_one_booking_from_ban', 'RbScotlandCup::is_one_booking_from_ban', '0x7e2440', 'ruling_body/rb_scotland_cup.cpp', _C,
          _PT.format('da copa escocesa', 'já tem um amarelo'), _EN.format('Scottish cup', 'already one booking')),
         ('rb_scotland_league_is_one_booking_from_ban', 'RbScotlandLeague::is_one_booking_from_ban', '0x7e2930', 'ruling_body/rb_scotland_league.cpp', _C,
          'diz se mais 2 pontos disciplinares levam o jogador aos 16 da suspensão escocesa e grava a duração dela (as suspensões por pontos desta temporada e das duas anteriores, mais 1) e nenhum dia nas saídas',
          'tells whether 2 more disciplinary points take the player to the Scottish 16-point ban and writes its length (the points bans of this season and the two before, plus 1) and no days to the outputs'),
         ('rb_sweden_league_is_one_booking_from_ban', 'RbSwedenLeague::is_one_booking_from_ban', '0x7e39c0', 'ruling_body/rb_sweden_league.cpp', _C,
          _PT.format('suecas (liga e copa)', 'contagem par e não zero'), _EN.format('Swedish (league and cup)', 'count even and not zero')),
         ('rb_turkey_league_is_one_booking_from_ban', 'RbTurkeyLeague::is_one_booking_from_ban', '0x7e40b0', 'ruling_body/rb_turkey_league.cpp', _C,
          _PT.format('turcas (liga e copa)', 'contagem mais 1 múltipla de 4'), _EN.format('Turkish (league and cup)', 'count plus 1 a multiple of 4')),
         ('rb_usa_is_one_booking_from_ban', 'RbUsa::is_one_booking_from_ban', '0x7e4390', 'ruling_body/rb_usa.cpp', _C,
          _PT.format('americanas', 'exatamente 20 pontos'), _EN.format('US', 'exactly 20 points')),
         ('rb_wales_is_one_booking_from_ban', 'RbWales::is_one_booking_from_ban', '0x7e4710', 'ruling_body/rb_wales.cpp', _C,
          _PT.format('galesas e da liga espanhola', 'contagem mais 1 múltipla de 5'), _EN.format('Welsh and Spanish league', 'count plus 1 a multiple of 5')),
         ('check_one_booking_from_ban', 'RulingBody::check_one_booking_from_ban', '0x7e8d60', 'ruling_body/ruling_body.cpp', 'thiscall, ret 0x10',
          'acha a entrada do jogador no órgão (sem acrescentar) e pergunta à regra do próprio órgão (vtable +0x14) se mais um amarelo o suspende; sem entrada, não',
          "finds the player's entry in the body (without adding one) and asks the body's own rule (vtable +0x14) whether one more booking bans him; without an entry, no")])
CAMPOS = []
INTERESSA = []
