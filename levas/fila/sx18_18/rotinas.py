# O órgão da liga finlandesa e o construtor do da copa (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Finlandia carregada (ATE 2002-08-01; a temporada finlandesa vai de abril a outubro).
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('rb_finland_league_construct', 0x7dd990, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_finland_league_process_match_cards', 0x7dd9e0, [0x6a, 0xff, 0x68, 0x26, 0x00, 0x96, 0x00]),
           ('rb_finland_cup_construct', 0x7dd660, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão da liga finlandesa e o construtor do da copa',
        "the Finnish league ruling body and the Finnish cup body's constructor",
        [('rb_finland_league_construct', 'RbFinlandLeague::RbFinlandLeague', '0x7dd990', 'ruling_body/rb_finland_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga finlandesa (vtable 0x96eb74, vaga 0, regra 0xa, escala 7, Finlândia na Europa; liga 0xa, copa 0xb, confederação 1) sobre a base',
          'builds the Finnish league (vtable 0x96eb74, slot 0, rule 0xa, scale 7, Finland in Europe; league 0xa, cup 0xb, confederation 1) ruling body on the base'),
         ('rb_finland_league_process_match_cards', 'RbFinlandLeague::process_match_cards', '0x7dd9e0', 'ruling_body/rb_finland_league.cpp', 'thiscall, ret 8',
          'regras finlandesas: cada terceiro amarelo dá 1 jogo (quantidade 3); vermelho enfileira uma suspensão por pontos a partir de uma semana e dá 1 jogo; a partir de hoje',
          'Finnish rules: every third booking gives 1 match (amount 3); a red card queues a points ban from a week on and gives 1 match; from today'),
         ('rb_finland_cup_construct', 'RbFinlandCup::RbFinlandCup', '0x7dd660', 'ruling_body/rb_finland_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa finlandesa (vtable 0x96eb54, vaga 1, regra 0xb, escala 7) sobre a base',
          'builds the Finnish cup (vtable 0x96eb54, slot 1, rule 0xb, scale 7) ruling body on the base')])
CAMPOS = []
INTERESSA = []
