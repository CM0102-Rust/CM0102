# Os jogadores virtuais do clube que o tecnico humano ve (virtual_staff.cpp; leitor sx21b, rodada 5, 24/11/2026).
# 0 chamadas na carreira de teste (o mundo ate 2002-08-01): so a tela de um clube (0x484215) a chama. Tipo TELAS.
# Caminho no jogo: abrir a tela de elenco de um clube de fora das ligas escolhidas (com poucos jogadores, que o jogo
# completa com virtuais); depois abrir a de outro clube assim (o primeiro perde ou muda de bloco os virtuais dele);
# e abrir a de um clube que joga uma partida marcada (os virtuais ficam no bloco da partida).
# Convencoes conferidas: 0x90d4f0 thiscall(clube) ret 4. A vaga do tecnico da tela como em remove_virtual_players
# (current_viewed_club_slot, da sx21b_3: integrar esta depois dela); 0x68d1b0 get_next na lista de partidas (+0x5c; o no: partida, palavra da vaga);
# 0x90e760 get_id_block(vaga, e_partida, mandante, &staff, &jogador); 0x90e9a0 relocate_virtual_players(clube, staff,
# jogador); 0x90d860 remove_virtual_players(clube, 0); 0x90dae0 top_up_club_squad(clube, staff, jogador, 0, 0).
# Comparador: so a original (cria e move jogadores virtuais: o sorteio e os registros); confere a vaga com o clube.
# Sem mutacao: o comparador so roda a original, entao nenhuma mutacao da copia seria acusada na sessao comparando; a
# copia roda nas sessoes no modo Rust.
SAVE = False
TELAS = True
ROTINAS = [
    ('fill_viewed_club_squad', 0x90d4f0, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
]
DOCS = ('os jogadores virtuais do clube que o técnico humano vê',
        "the virtual players of the club a human manager views",
        [('fill_viewed_club_squad', 'fill_viewed_club_squad', '0x90d4f0', 'virtual_staff.cpp', 'thiscall, ret 4',
          'regra: a vaga do técnico já no clube, com virtuais dele: nada; senão o clube visto antes guarda os virtuais no bloco de uma partida que joga ou de outra vaga que o vê, ou os perde; o clube novo é completado com virtuais no bloco da vaga, que passa a ser dele',
          "rule: the manager's slot already on the club, with its own virtual players: nothing; otherwise the club viewed before keeps its virtual players in the block of a fixture it plays or of another slot viewing it, or loses them; the new club is topped up with virtual players in the slot's block, which becomes its own")])
CAMPOS = []
INTERESSA = []
