# As regras dos cartões croatas (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa CROATIA_RULING_BODY do sx18_20.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rb_croatia_process_match_cards', 0x7dc750, [0x6a, 0xff, 0x68, 0x0c, 0xff, 0x95, 0x00])]
DOCS = ('as regras dos cartões croatas',
        'the Croatian card rules',
        [('rb_croatia_process_match_cards', 'RbCroatia::process_match_cards', '0x7dc750', 'ruling_body/rb_croatia.cpp', 'thiscall, ret 8',
          'regras croatas: cada amarelo soma 1 e cada terceiro dá 1 jogo; vermelho enfileira uma suspensão por pontos a partir do primeiro dia da semana 3 depois de hoje e dá 1 jogo; a partir de hoje',
          'Croatian rules: each booking adds 1 and every third gives 1 match; a red card queues a points ban from the first weekday 3 after today and gives 1 match; from today')])
CAMPOS = []
INTERESSA = []
