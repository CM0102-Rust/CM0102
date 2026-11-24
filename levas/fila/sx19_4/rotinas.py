# O orgao disciplinar da Oceania (rb_oceania.cpp: o construtor e as regras dos cartoes; leitor sx19, rodada 5,
# 24/11/2026). Usa as pecas do sx18_1 (plain_card_verdict, run_card_rules, compare_card_rules) e do sx19_1
# (own_rules): integrar depois deles.
# So roda com a Australia carregada: o construtor na criacao do jogo quando o continente passa no teste 0x6699d0
# (Discipline::create_rule_sets, regra 0x23), as regras nas partidas das competicoes da Oceania com cartao.
# Convencoes conferidas na desmontagem:
# - 0x7e13d0 thiscall(arquivo) ret 4: 0x7e7760 (base), vtable 0x96ed74, +4 = 2, +5 = 0x23, +6 = 0xa, +0x17 = -1,
#   +0x1b Oceania (0x9cfa18), +0x1f/+0x20 = 0xff, +0x21 = 0x23, +0x22 = 0;
# - 0x7e1420 thiscall(linha do registro, partida) ret 8: o vermelho (+0x2d == 1) da 1 jogo (motivo 2, quantidade 1);
#   cada partida com amarelo soma 1 na entrada e a cada terceiro mais 1 jogo (motivo 1; a quantidade e o cl, que o
#   `mov ecx, 3` do idiv deixou em 3); regras ff ff 23 ff; a partir de hoje; 0x556940 add_ban e 0x557280.
# Defeito do original: os bits de lixo das marcas (+0x13), como nos outros orgaos.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_oceania_construct', 0x7e13d0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_oceania_process_match_cards', 0x7e1420, [0x6a, 0xff, 0x68, 0x98, 0x03, 0x96, 0x00])]
DOCS = ('o órgão disciplinar da Oceania',
        'the Oceanian ruling body',
        [('rb_oceania_construct', 'RbOceania::RbOceania', '0x7e13d0', 'ruling_body/rb_oceania.cpp', 'thiscall, ret 4',
          'monta o órgão da Oceania sobre a base: tabela virtual 0x96ed74, a vaga 2 do registro, a regra 0x23, a escala 10, sem nação, a Oceania e ele mesmo como regra da confederação',
          'builds the Oceanian body on the base: vtable 0x96ed74, record slot 2, rule set 0x23, scale 10, no nation, Oceania and itself as the confederation rule'),
         ('rb_oceania_process_match_cards', 'RbOceania::process_match_cards', '0x7e1420', 'ruling_body/rb_oceania.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da Oceania: um jogo pelo vermelho e um a cada terceiro amarelo (um por partida), a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Oceanian rules: one match for a red card and one for every third booking (one per match), from today, with its news')])
CAMPOS = []
INTERESSA = []
