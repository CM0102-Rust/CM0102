# As posições previstas da copa (Cup.cpp, comp_util.cpp, comp_stats.cpp): Cup::rank_teams (0x5223a0, 12 chamadas na
# cobertura de 17/10: o início de cada copa), comp_util_sort_clubs_by_strength (0x4c08f0, 17: a copa e as posições
# previstas das ligas, 0x68a850 e as europeias) e Comp_Stats::set_club_final_position (0x4a2540: chamada por todas as
# posições previstas, das copas e das ligas). A posição prevista (+0x72 do registro do clube na estatística) entra na
# importância da partida (0x4b02e0 e as de liga/copa no motor.rs: quem está acima, a força do adversário na copa).
# A 0x5223a0 empurra a posição como dword com só o byte de baixo gravado (sobra de pilha em cima); a 0x4a2540 lê o
# byte (a cópia passa o byte estendido). Sem SEH. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Comparadores: a ordenação e as posições pelo plano feito antes sobre uma cópia
# do vetor (as chaves num Vec, sem alocar no jogo); gravar a posição pelo registro visto antes e o byte +0x72.
SAVE = False
ROTINAS = [('posicoes_previstas_da_copa', 0x5223a0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('ordenar_clubes_pela_forca', 0x4c08f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gravar_posicao_do_clube', 0x4a2540, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas da copa: as posições pela rodada de saída, ordenar os clubes pela força e gravar a posição do clube na estatística',
        "the cup's predicted positions: positions by exit round, sorting the clubs by strength and storing the club's position in the statistics",
        [('posicoes_previstas_da_copa', 'Cup::rank_teams', '0x5223a0', 'Cup.cpp', 'thiscall, ret 0',
          'ordena os clubes da copa pela força e dá a cada um a posição prevista pela rodada em que sai, na estatística da copa',
          'sorts the cup clubs by strength and gives each one the predicted position of the round it goes out in, in the cup statistics'),
         ('ordenar_clubes_pela_forca', 'comp_util_sort_clubs_by_strength', '0x4c08f0', 'comp_util.cpp', 'cdecl',
          'ordena um vetor de clubes pela força (0x4c0720), a maior primeiro',
          'sorts an array of clubs by strength (0x4c0720), strongest first'),
         ('gravar_posicao_do_clube', 'Comp_Stats::set_club_final_position', '0x4a2540', 'comp_stats.cpp', 'thiscall, ret 8',
          'acha ou acrescenta o registro do clube na estatística da competição e grava a posição em +0x72',
          "finds or adds the club's record in the competition statistics and stores the position at +0x72")])
CAMPOS = [('Cup +8: a estatística (Comp_Stats); +0x3c a conta das rodadas (palavra); +0x3e a conta dos clubes (palavra); +0xa3 as rodadas (0x68 bytes, +0x1a quantos saem nela); +0xa7 os clubes (6 bytes, +0 o clube); ponteiro e valor; lê (posicoes_previstas_da_copa)',
           'Cup +8: the statistics (Comp_Stats); +0x3c the round count (word); +0x3e the club count (word); +0xa3 the rounds (0x68 bytes, +0x1a how many go out in it); +0xa7 the clubs (6 bytes, +0 the club); pointer and value; reads (posicoes_previstas_da_copa)'),
          ('registro do clube na estatística da competição +0x72: a posição prevista; valor (byte); grava (gravar_posicao_do_clube)',
           "club record in the competition statistics +0x72: the predicted position; value (byte); writes (gravar_posicao_do_clube)")]
INTERESSA = ['CMRTE']
