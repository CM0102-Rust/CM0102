MOTOR = True
ROTINAS = [('cumprir_regra_do_elenco', 0x6e8910, [0x81, 0xec, 0x70, 0x04, 0x00, 0x00]),
           ('nota_fora_da_partida', 0x6e6340, [0x83, 0xec, 0x50, 0x53, 0x55]),
           ('nota_para_a_vaga', 0x6e2f20, [0x83, 0xec, 0x50, 0x53, 0x8b, 0x5c, 0x24, 0x58])]
DOCS = ('o fechamento do motor (7): a regra do elenco e as notas do jogador',
        'closing the engine (7): the squad rule and the player scores',
        [('cumprir_regra_do_elenco', 'apply_squad_rule', '0x6e8910', 'match_man.cpp', 'cdecl',
          'cumpre a regra do elenco (estrangeiros, extracomunitários ou idade): troca os restritos pelos candidatos de menor perda na nota',
          'applies the squad rule (foreigners, non-EU or age): swaps restricted players for the candidates with the smallest score loss'),
         ('nota_fora_da_partida', 'off_match_score', '0x6e6340', 'match_man.cpp', 'cdecl',
          'a nota do jogador para uma posição fora da partida (transferências e observação): posição, forma, habilidade vista, contrato, idade, conhecimento do observador',
          'the player\'s score for a position outside a match (transfers and scouting): position, form, perceived ability, contract, age, scout knowledge'),
         ('nota_para_a_vaga', 'lineup_slot_score', '0x6e2f20', 'match_man.cpp', 'cdecl',
          'a nota do jogador para uma vaga da escalação: como a nota fora da partida, mais a segunda posição, o fôlego na partida, a tática, os veteranos e as férias',
          'the player\'s score for a lineup slot: like the off-match score, plus the second position, in-match stamina, tactics, veterans and holidays')])
