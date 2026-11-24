# As regras dos cartões do órgão francês (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa FRANCE_RULING_BODY do sx18_7. Roda para toda linha da partida (com ou sem cartao) nas competicoes francesas. Defeito do original: o no pendente da terceira partida com cartao sai com as marcas (+0x13) e o byte +0x6f do lixo da pilha (ou do no do vermelho na mesma chamada); a copia grava 0.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_france_process_match_cards', 0x7ddd10, [0x6a, 0xff, 0x68, 0x92, 0x00, 0x96, 0x00])]
DOCS = ('as regras dos cartões do órgão francês',
        'the French card rules',
        [('rb_france_process_match_cards', 'RbFrance::process_match_cards', '0x7ddd10', 'ruling_body/rb_france.cpp', 'thiscall, ret 8',
          'regras francesas, para todo jogador da partida: uma janela de dez partidas com cartão (bits de +0xa, +0xb e +0xc da entrada) anda uma partida; vermelho dá 1 jogo e uma suspensão por pontos pendente a partir do primeiro dia da semana 3 depois de hoje; amarelo marca a partida e, na terceira partida com cartão da janela, enfileira uma suspensão de 1 jogo (motivo 1, quantidade 3) a valer no primeiro dia da semana 3 depois de hoje e esvazia a janela',
          "French rules, for every player of the match: a ten-match booking window (bits of the entry's +0xa, +0xb and +0xc) moves one match; a red card gives 1 match and a pending points ban from the first weekday 3 after today; a booking marks the match and, at the third booked match in the window, queues a one-match ban (reason 1, amount 3) effective on the first weekday 3 after today and empties the window")])
CAMPOS = []
INTERESSA = []
