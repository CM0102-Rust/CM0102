# As viradas das eliminatórias da Copa do Mundo da Ásia, da América do Sul e da Oceania (wc_asia_league.cpp,
# wc_south_american_league.cpp, wc_oceania_league.cpp em worldcup\; leitor mwg, rodada 3 do mundo inteiro, 19/11/2026).
# Cada uma roda uma vez até 2002-08-01 (cobertura_mundo.txt), quando a campanha de 2002 termina; as seções das
# eliminatórias existem em qualquer carreira (as seleções estão no banco inteiro; os construtores, cqz_4): LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x913360..0x913481, 0x921470..0x92158e, 0x91ef40..0x91f0c4): as três
# thiscall, ret 0, sobre League:
# - todas: +0x4c = 0, League::free_table(0) (0x687970, push 0 -> thiscall, ret 4), +0xba liberado e zerado, o arquivo
#   da temporada (0x4a1c50, push 1; mov ecx, [+8] -> thiscall, ret 4), o ano +0x40 (movsx) + 4 enquanto abaixo de
#   [0xae2c92] (palavra sem sinal, relida), +0x45 = 0; os erros por eax == 0 (test; jne), não negativo;
# - Ásia: +0xef (a lista de seleções) liberada sem zerar, as fases 0..=+0x30 destruídas (virtual 0 com 1), +0x30 = -1
#   antes do arquivo e de novo depois do ano; 0x913f60 build_entrant_list (retorno não lido); a virtual +0x8c (caixa
#   0x45e), cujo eax devolve;
# - América do Sul: sem lista e sem fases; 0x920d10 count_exempt_nations (retorno não lido), +0x8c (caixa 0x2e1) e
#   0x920eb0 build_team_slots (caixa 0x2e8), cujo eax devolve;
# - Oceania: como a Ásia, e o vetor de fases +0xc liberado (sem zerar) depois das fases; 0x91fa80 build_entrant_pool,
#   a virtual +0x5c, +0x2c = 3 e +0xc = malloc(0xc) (0x944e46, cdecl), +0x8c (caixa 0x305) e 0x91ec00 init_group_table
#   (caixa 0x30c), cujo eax devolve.
# Depois de uma caixa o original devolve o eax dela, a cópia 0 (quem chama a virada não lê).
# Comparadores: só o original roda (libera e refaz); o plano: o ano subiu um múltiplo de 4 e não está abaixo do ano do
# jogo, e +0x4c = 0.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('wc_asia_league_start_new_season', 0x913360, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_south_american_league_start_new_season', 0x921470, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_oceania_league_start_new_season', 0x91ef40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas das eliminatórias da Copa do Mundo da Ásia, da América do Sul e da Oceania',
        'the World Cup qualifying turnovers of Asia, South America and Oceania',
        [('wc_asia_league_start_new_season', 'WcAsiaLeague::start_new_season', '0x913360', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'a virada das eliminatórias asiáticas: a tabela, as datas, a lista de seleções e as fases da campanha passada liberadas, a temporada arquivada, o ano sobe de 4 em 4 até alcançar o do jogo, o sorteio por fazer e a fase −1; as seleções de novo e as regras (caixa na linha 0x45e se falhar)',
          "the Asian qualifying turnover: last campaign's table, dates, national-team list and stages freed, the season archived, the year goes up 4 at a time until it reaches the game's, the draw still to do and the stage −1; the national teams again and the rules (box at line 0x45e on failure)"),
         ('wc_south_american_league_start_new_season', 'WcSouthAmericanLeague::start_new_season', '0x921470', 'wc_south_american_league.cpp', 'thiscall, ret 0',
          'a virada das eliminatórias sul-americanas (uma liga só, sem fases): a tabela e as datas liberadas, a temporada arquivada, o ano de 4 em 4 até o do jogo, o sorteio por fazer; as isentas contadas de novo, as regras (caixa 0x2e1) e as vagas das seleções (caixa 0x2e8)',
          "the South American qualifying turnover (one league, no stages): the table and dates freed, the season archived, the year 4 at a time up to the game's, the draw still to do; the exempt nations counted again, the rules (box 0x2e1) and the team slots (box 0x2e8)"),
         ('wc_oceania_league_start_new_season', 'WcOceaniaLeague::start_new_season', '0x91ef40', 'wc_oceania_league.cpp', 'thiscall, ret 0',
          'a virada das eliminatórias da Oceania: como a asiática, e o vetor de fases liberado; o grupo de seleções e as posições previstas de novo, 3 fases num vetor novo, as regras (caixa 0x305) e a tabela do grupo (caixa 0x30c)',
          "the Oceania qualifying turnover: as the Asian one, and the stage vector freed; the national-team pool and the predicted positions again, 3 stages in a new vector, the rules (box 0x305) and the group table (box 0x30c)")])
