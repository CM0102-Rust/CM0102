# As regras dos cartões sul-coreanas e asiáticas (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa KOREA_RULING_BODY do sx18_22 e ASIA_RULING_BODY do sx18_2. As regras asiaticas so rodam em competicao asiatica de clubes (com a Coreia ou o Japao): podem nao rodar ate 2002-08-01 (so o comparador as prova).
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('rb_korea_process_match_cards', 0x7e0780, [0x6a, 0xff, 0x68, 0x26, 0x03, 0x96, 0x00]),
           ('rb_asia_process_match_cards', 0x7db3b0, [0x6a, 0xff, 0x68, 0xa8, 0xfd, 0x95, 0x00])]
DOCS = ('as regras dos cartões sul-coreanas e asiáticas',
        'the South Korean and Asian card rules',
        [('rb_korea_process_match_cards', 'RbKorea::process_match_cards', '0x7e0780', 'ruling_body/rb_korea.cpp', 'thiscall, ret 8',
          'regras sul-coreanas: vermelho enfileira uma suspensão por pontos a partir de amanhã e dá 1 jogo; os amarelos da partida somam e o 3º e cada ímpar depois dão 1 jogo (quantidade 3 no terceiro, depois 2); a partir de hoje',
          "South Korean rules: a red card queues a points ban from tomorrow and gives 1 match; the match's bookings add up and the 3rd and each odd count after give 1 match (amount 3 at the third, then 2); from today"),
         ('rb_asia_process_match_cards', 'RbAsia::process_match_cards', '0x7db3b0', 'ruling_body/rb_asia.cpp', 'thiscall, ret 8',
          'regras asiáticas: vermelho dá 1 jogo; cada segundo amarelo (um por partida) dá 1 jogo (quantidade 2), na regra asiática da vaga da confederação; a partir de hoje',
          'Asian rules: a red card gives 1 match; every second booking (one per match) gives 1 match (amount 2), under the Asian rule set in the confederation slot; from today')])
CAMPOS = []
INTERESSA = []
