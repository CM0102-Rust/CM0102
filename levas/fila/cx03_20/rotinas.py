# Os elencos do Jogo das Estrelas da MLS (usa_mls_all_stars.cpp; leitor cx03, rodada 5, 24/11/2026). Carreira de 8 ligas
# (os EUA jogaveis): roda 2 vezes ate 2002-06-01 (docs/cobertura_competicoes.txt), chamada pelo dia da copa
# (usa_mls_all_stars_update_for_date, na convocacao de 26/7) e na criacao.
# - 0x90a980 UsaMlsAllStars::select_squads thiscall() ret 0 (1.328 bytes): a MLS ([0xadadfc][[0x9cf590]]) e as fases 0 e
#   1; a data da primeira rodada (+0xa3: o dia +3 e o ano relativo +5 + +0x40; Date::Date 0x54a110 vazio, 0x549ef0 (dia,
#   ano) e Date::operator= 0x417c10, que copia nos dois locais: a data final e a da rodada); as 20 mascaras de posicao
#   (palavras locais); para cada conferencia, 4 linhas x 2 passadas (o Leste: a tabela da MLS e, se a conferencia
#   0x90a000 do clube da fase 0 for 0x413, a fase 0; o Oeste: a fase 1 e, com 0x414, a fase 0), os 50 do elenco de cada
#   clube com registro de jogador (+0x61), fora de lesao (0x62c400, ecx [0xb63d28], != 2), sem convocacao na data
#   (0x7757d0, ecx [0xb74318], 5 pushes, thiscall, ret 0x14) e ainda no clube (+0x39); o clube das estrelas ([0x9d00a4]
#   / [0x9d00a8]) com o elenco zerado (rep stosd 0x32), qsort 0x945333 (4 bytes, 0x5402f0), no maximo 127 e a caixa 0x1dc
#   / 0x221 com menos de 30; 0x6e2000 (15 pushes, cdecl: lista, n, mascaras, 0x14, 0, elenco, 2, oito zeros). As duas
#   listas de 300 ficam lado a lado na pilha do original (0x268 e 0x718): a copia usa 400 cada (so muda acima de 300).
#   SEH so para o Date local (destrutor trivial). Prologo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe.
# Comparador: lado a lado nos elencos dos dois clubes das estrelas (50 vagas cada) e no indicador da caixa.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('usa_mls_all_stars_select_squads', 0x90a980, [0x6a, 0xff, 0x68, 0xd6, 0x54, 0x96, 0x00])]
DOCS = ('os elencos do Jogo das Estrelas da MLS', 'the squads of the MLS All-Star Game',
        [('usa_mls_all_stars_select_squads', 'UsaMlsAllStars::select_squads', '0x90a980', 'usa_mls_all_stars.cpp', 'thiscall, ret 0',
          'regra: as duas seleções das estrelas da MLS são convocadas dos clubes nos quatro primeiros lugares: o Leste da tabela da própria MLS e dos clubes do Leste na tabela da primeira conferência, o Oeste da tabela da segunda conferência e dos clubes do Oeste na primeira; cada seleção leva os jogadores sem lesão e sem convocação no dia do jogo, ordenados, no máximo 127, e escolhe um elenco de 20 por posição',
          'rule: the two MLS All-Star selections are called up from the clubs in the first four places: the East from the MLS\'s own table and from the East clubs in the first conference table, the West from the second conference table and from the West clubs in the first; each selection takes the players fit and free of international duty on the day of the game, sorted, at most 127, and picks a squad of 20 by position')])
CAMPOS = []
INTERESSA = []
