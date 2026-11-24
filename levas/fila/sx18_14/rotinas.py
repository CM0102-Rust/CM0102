# O órgão da liga alemã e o construtor do da copa (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Alemanha carregada.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('rb_germany_league_construct', 0x7de620, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_germany_league_process_match_cards', 0x7de670, [0x6a, 0xff, 0x68, 0xf6, 0x00, 0x96, 0x00]),
           ('rb_germany_cup_construct', 0x7de2e0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão da liga alemã e o construtor do da copa',
        "the German league ruling body and the German cup body's constructor",
        [('rb_germany_league_construct', 'RbGermanyLeague::RbGermanyLeague', '0x7de620', 'ruling_body/rb_germany_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga alemã (vtable 0x96ebd4, vaga 0, regra 0xd, escala 7, Alemanha na Europa; liga 0xd, copa 0xe, confederação 1) sobre a base',
          'builds the German league (vtable 0x96ebd4, slot 0, rule 0xd, scale 7, Germany in Europe; league 0xd, cup 0xe, confederation 1) ruling body on the base'),
         ('rb_germany_league_process_match_cards', 'RbGermanyLeague::process_match_cards', '0x7de670', 'ruling_body/rb_germany_league.cpp', 'thiscall, ret 8',
          'regras alemãs da liga: cada quinto amarelo dá 1 jogo (quantidade 5); vermelho enfileira uma suspensão por pontos a partir de uma semana e dá 1 jogo; o vermelho direto vale também na copa; a partir de hoje',
          'German league rules: every fifth booking gives 1 match (amount 5); a red card queues a points ban from a week on and gives 1 match; a straight red card also counts in the cup; from today'),
         ('rb_germany_cup_construct', 'RbGermanyCup::RbGermanyCup', '0x7de2e0', 'ruling_body/rb_germany_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa alemã (vtable 0x96ebb4, vaga 1, regra 0xe, escala 10) sobre a base',
          'builds the German cup (vtable 0x96ebb4, slot 1, rule 0xe, scale 10) ruling body on the base')])
CAMPOS = []
INTERESSA = []
