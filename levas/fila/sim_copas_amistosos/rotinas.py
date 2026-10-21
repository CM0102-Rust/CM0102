ROTINAS = [('regras_da_rodada_no_jogo', 0x521d60, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('chave_do_jogo_na_copa', 0x519c90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pais_da_excursao', 0x5c4680, [0x51, 0x8b, 0x44, 0x24, 0x08])]
DOCS = ('as regras da rodada no jogo da copa, a chave do jogo e o país da excursão',
        'the round rules on the cup match, the match tie and the tour nation',
        [('regras_da_rodada_no_jogo', 'Cup::apply_round_rules_to_match', '0x521d60', 'Cup.cpp', 'thiscall',
          'passa as regras da rodada para o jogo e, com decisão no empate, marca quem passa pelo nível dos dois lados',
          "copies the round rules onto the match and, with a decision on a draw, marks who goes through by both sides' levels"),
         ('chave_do_jogo_na_copa', 'Cup::find_tree_slot_for_fixture', '0x519c90', 'Cup.cpp', 'thiscall',
          'a chave da rodada em que está o mandante do jogo (-1 se nenhuma)',
          "the round tie that holds the match's home club (-1 if none)"),
         ('pais_da_excursao', 'Friendly::pick_tour_nation', '0x5c4680', 'friendly.cpp', 'thiscall',
          'sorteia o país da excursão do clube, mais perto quanto menor a reputação',
          'draws the nation for a club tour, nearer the lower the reputation')])
