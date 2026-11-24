# O progresso das selecoes na Copa Africana de Nacoes (african_nations.cpp; leitor cx05, rodada 5, 24/11/2026).
# Simulacao com as 8 ligas, sem ATE: a Copa Africana do jogo e a de 2002 (janeiro e fevereiro de 2002, dentro da
# janela ate 2002-06-01); a virtual roda a cada jogo da copa.
# - 0x4029c0 AfricanNations::set_club_progress thiscall(selecao, fase, atual, melhor, pior) ret 0x14 (virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (push selecao; mov ecx, [this+8] -> thiscall, ret 4; sem conferir nulo); fase
#   < 3 (com sinal): as tabelas de salto 0x402a88, 0x402a98 e 0x402aa8 (1 a 4); grava +0x74 (melhor), +0x73 (atual),
#   +0x75 (pior). O molde do Campeonato da Oceania (group_championship_progress do cx04_5). Prologo mov eax, [esp+4];
#   mov ecx, [ecx+8] (7 bytes).
# Igual no cm0102-1080.exe (inteira).
# Depende do cx04_5 (group_championship_progress, compare_group_championship_progress): integrar depois dele.
# Comparador: lado a lado nos 3 bytes da entrada (so a original sem entrada).
# Duvida para a prova: conferir na cobertura que a Copa Africana de 2002 roda na carreira de 8 ligas (o construtor e o
# dia ja sao Rust); se nao rodar, o item vai para uma carreira com as ligas africanas (nao ha nenhuma no jogo) ou fica
# so com o comparador.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('african_nations_set_club_progress', 0x4029c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o progresso das seleções na Copa Africana', 'the nations\' progress in the African Nations Cup',
        [('african_nations_set_club_progress', 'AfricanNations::set_club_progress', '0x4029c0', 'african_nations.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de uma seleção nas estatísticas da Copa Africana: nos grupos as colocações 1 a 4 viram os códigos da copa (atual 5, 9, 9, 13; melhor 1, 1, 9, 13; pior 5, 9, 9, 13); depois, como vêm',
          'stores a nation\'s progress in the African Nations Cup statistics: in the groups places 1 to 4 become the cup\'s codes (current 5, 9, 9, 13; best 1, 1, 9, 13; worst 5, 9, 9, 13); afterwards, as they come')])
CAMPOS = []
INTERESSA = []
