# officials_manager.cpp (leitor sx16, rodada 5, 24/11/2026): as linhas de estatisticas dos arbitros lidas pelas telas.
# Todas thiscall com this = G_OFFICIALS_MANAGER (0xb74340); a linha de 0x11 bytes em [this+0] + id x 0x11.
# - 0x79d190 reset_official_stats(&arbitro) ret 4: zera +0xf, +8, +0xa, +0xc (palavras) e +4 (dword); a +0x10 fica.
# - 0x79d1f0 copy_comp_official_stats(competicao, saida) ret 8 -> ax; 0x79d300 get_comp_official_count(competicao)
#   ret 4 -> ax (no maximo 100): o objeto da competicao ([0xadadfc][id]), tipo +0x42 nem 4 nem 2, 0x4b0700
#   comp_util_is_league_division cdecl(id); a nacao +0x5d da competicao ou 0x90cfc0(G_NATION_COUNT); o bloco da nacao
#   ([this+0xc] + nacao x 0x102): a primeira das +0x10 vagas (de 0x10 bytes a partir de +0x12) com o id da competicao
#   em +0xc (+0 primeiro, +4 quantos), senao o bloco inteiro (+8 primeiro, +4 quantos).
# - 0x79d3b0 get_last_match_rating(&arbitro) ret 4 -> al; 0x79d3e0 get_average_rating(&arbitro) ret 4 -> st(0)
#   (6.0 de 0x969084 sem arbitro).
# Quem chama: show_comp_official_stats_screen 0x79a530 (ja Rust, a tela Official Stats: count e copy) e o pedido a
# federacao sobre o arbitro (manager_manager.cpp, 0x6aa566..0x6aa733: nota media, ultima nota, reset), so pela tela.
# Defeitos do original: copy_comp_official_stats nao limita a 100 como get_comp_official_count (a tela aloca pela
# contagem limitada: uma competicao com mais de 100 arbitros escreveria alem do buffer); uma competicao sem nacao
# sorteia uma nacao a cada chamada (count e copy podem pegar blocos diferentes).
# Caminho no jogo: Competicoes -> Premier -> Official Stats (count e copy); o pedido a federacao sobre o arbitro depois
# de uma partida (as outras tres).
TELAS = True
ROTINAS = [('reset_official_stats', 0x79d190, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('copy_comp_official_stats', 0x79d1f0, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10]),
           ('get_comp_official_count', 0x79d300, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('official_last_match_rating', 0x79d3b0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('official_average_rating', 0x79d3e0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('as linhas de estatísticas dos árbitros',
        "the officials' statistics rows",
        [('reset_official_stats', 'OfficialsManager::reset_official_stats', '0x79d190', 'officials_manager.cpp', 'thiscall, ret 4',
          'zera a linha de estatísticas de um árbitro (cartões, faltas, soma das notas, jogos, nota média); a última nota fica',
          "clears an official's statistics row (cards, fouls, grade sum, matches, average grade); the last grade stays"),
         ('copy_comp_official_stats', 'OfficialsManager::copy_comp_official_stats', '0x79d1f0', 'officials_manager.cpp', 'thiscall, ret 8',
          'copia as linhas dos árbitros de uma competição de liga (a vaga da competição no bloco da nação dela, ou o bloco inteiro; sem nação, uma nação sorteada) e devolve quantas; 0 para copa, competição de tipo 2 ou 4 ou que não é divisão de liga',
          "copies the rows of a league competition's officials (the competition's slot in its nation's block, or the whole block; without a nation, a drawn nation) and returns how many; 0 for a cup, a type 2 or 4 competition or one that is not a league division"),
         ('get_comp_official_count', 'OfficialsManager::get_comp_official_count', '0x79d300', 'officials_manager.cpp', 'thiscall, ret 4',
          'quantos árbitros apitam a competição de liga, no máximo 100',
          'how many officials referee the league competition, at most 100'),
         ('official_last_match_rating', 'OfficialsManager::get_last_match_rating', '0x79d3b0', 'officials_manager.cpp', 'thiscall, ret 4',
          'a última nota do árbitro; 7 sem nota registrada, 6 sem árbitro',
          "the official's last grade; 7 with none recorded, 6 without an official"),
         ('official_average_rating', 'OfficialsManager::get_average_rating', '0x79d3e0', 'officials_manager.cpp', 'thiscall, ret 4',
          'a nota média do árbitro; 6,0 sem árbitro',
          "the official's average grade; 6.0 without an official")])
