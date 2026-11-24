# A lista de inscritos das copas com grupos no jogo salvo, leitura (ita_cup.cpp, kor_league_cup.cpp, nir_lge_cup.cpp;
# leitor cx02, rodada 5, 24/11/2026). Item do SAVE (CARGA), como o cx02_14 (usa load_group_cup_entrants e
# compare_group_cup_entrants_load dele: integrar depois dele). So com um save que tenha a Italia, a Coreia ou a Irlanda
# do Norte; o FeriasTesteSave nao as chama.
# - 0x640050 ItaCup::load_extra_data, 0x66d1e0 KorLeagueCup::load_extra_data e 0x78db10 NirLgeCup::load_extra_data
#   thiscall(arquivo) ret 4 -> 0/1: iguais a GreCup::load_extra_data menos o arquivo-fonte (0x9ea23c, 0x9eb15c, 0xa69698)
#   e as linhas das caixas. Prologo sub esp, 0x200.
# Iguais no cm0102-1080.exe.
# Comparadores: como no cx02_14.
SAVE = True
CARGA = True
ROTINAS = [('ita_cup_load_extra_data', 0x640050, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_league_cup_load_extra_data', 0x66d1e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nir_lge_cup_load_extra_data', 0x78db10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a lista de inscritos das copas com grupos no jogo salvo (leitura, 2)', "the group cups' entrant list in the saved game (load, 2)",
        [('ita_cup_load_extra_data', 'ItaCup::load_extra_data', '0x640050', 'ita_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da Itália o número de inscritos e os ids dos clubes, que voltam a ser clubes (−1, vaga vazia); leitura curta ou falta de memória: a caixa de erro e 0',
          "reads from the Coppa Italia temporary file the entrant count and the clubs' ids, which become clubs again (−1, an empty slot); a short read or no memory: the error box and 0"),
         ('kor_league_cup_load_extra_data', 'KorLeagueCup::load_extra_data', '0x66d1e0', 'kor_league_cup.cpp', 'thiscall, ret 4',
          'o mesmo para a Copa da Liga coreana',
          'the same for the Korean League Cup'),
         ('nir_lge_cup_load_extra_data', 'NirLgeCup::load_extra_data', '0x78db10', 'nir_lge_cup.cpp', 'thiscall, ret 4',
          'o mesmo para a Copa da Liga da Irlanda do Norte',
          'the same for the Northern Irish League Cup')])
CAMPOS = []
INTERESSA = []
