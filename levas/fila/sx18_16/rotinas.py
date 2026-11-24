# O órgão irlandês e o construtor do norte-irlandês (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Irlanda e a Irlanda do Norte carregadas (ATE 2002-08-01).
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
ROTINAS = [('rb_ireland_construct', 0x7df480, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_ireland_process_match_cards', 0x7df4d0, [0x6a, 0xff, 0x68, 0x07, 0x02, 0x96, 0x00]),
           ('rb_northern_ireland_construct', 0x7e0af0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão irlandês e o construtor do norte-irlandês',
        "the Irish ruling body and the Northern Irish body's constructor",
        [('rb_ireland_construct', 'RbIreland::RbIreland', '0x7df480', 'ruling_body/rb_ireland.cpp', 'thiscall, ret 4',
          'monta o órgão irlandês (vtable 0x96ec54, vaga 0, regra 0x10, escala 10, Irlanda na Europa; liga 0x10, confederação 1) sobre a base',
          'builds the Irish (vtable 0x96ec54, slot 0, rule 0x10, scale 10, Ireland in Europe; league 0x10, confederation 1) ruling body on the base'),
         ('rb_ireland_process_match_cards', 'RbIreland::process_match_cards', '0x7df4d0', 'ruling_body/rb_ireland.cpp', 'thiscall, ret 8',
          'regras irlandesas, para todo jogador da partida: um histórico de cinco partidas com cartão (bits 0..4 de +0xa da entrada); partida limpa depois de cinco limpas tira um amarelo da conta; cada amarelo soma 1: o 5º e o 8º dão 1 jogo, o 11º 2, e cada terceiro a partir do 14º 3; vermelho enfileira uma suspensão por pontos a partir do primeiro dia da semana 3 depois de hoje e dá 1 jogo; começa em 14 dias',
          "Irish rules, for every player of the match: a five-match card history (bits 0..4 of the entry's +0xa); a clean match after five clean ones takes a booking off the count; each booking adds 1: the 5th and 8th give 1 match, the 11th 2, and every third from the 14th 3; a red card queues a points ban from the first weekday 3 after today and gives 1 match; it starts in 14 days"),
         ('rb_northern_ireland_construct', 'RbNorthernIreland::RbNorthernIreland', '0x7e0af0', 'ruling_body/rb_northern_ireland.cpp', 'thiscall, ret 4',
          'monta o órgão norte-irlandês (vtable 0x96ed14, vaga 0, regra 0x24, escala 5, Irlanda do Norte na Europa; liga 0x24, confederação 1) sobre a base',
          'builds the Northern Irish (vtable 0x96ed14, slot 0, rule 0x24, scale 5, Northern Ireland in Europe; league 0x24, confederation 1) ruling body on the base')])
CAMPOS = []
INTERESSA = []
