# A carga do premio de equipe, do premio do ano e da tabela das notas do ano (award/team_award.cpp, year_award.cpp,
# year_ratings.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa team_award_obj e year_award_obj (sx05b_5): integrar
# depois dele. Define save_load_error_box e memory_error_box_of (as caixas de erro de carga com o arquivo e a mensagem),
# que o sx05b_8 usa.
# Rodam ao carregar um jogo salvo: os construtores dos premios (0x8af60c, 0x92ff0a) e o do AWARD_MANAGER (0x931381,
# pelo construtor das notas) leem do arquivo do save. Item de carga: SAVE e CARGA.
# Convencoes conferidas na desmontagem:
# - 0x8af660 thiscall(arquivo) ret 4: zread 0x932c30 cdecl(destino, tamanho, 1, arquivo) da data (+0x1d, 8), do
#   registro (+0x4e, 0x205), dos reservas (+0x253, 1) e dos deslocamentos (+0x254, +0x258, 4); a caixa "Load Game
#   Error" (0x90d130 com a mensagem, sprintf "v%s %s.%s %d (%s)", 0x66f4e0 0x9af3e0 e 0x9af334, 0x5e8290) sem zerar o
#   botao do mouse; linhas 0x69, 0x70, 0x77, 0x7e, 0x85;
# - 0x92fff0 thiscall(escopo, arquivo) ret 8: os deslocamentos +0x23 e +0x27 (linhas 0x70 e 0x77); com escopo,
#   operator new 0x944cf1(0x40) e AWARD_SHORTLIST 0x417440 (escopo, arquivo) ret 8 em +0x1f; sem memoria, a caixa
#   "Memory Allocation Error" (0x9a8108, 0x9a7ff8) da linha 0x81;
# - 0x931390 thiscall(arquivo) ret 4: a conta em +4; com entradas, +8 = a conta e malloc 0x944e46(21 x conta) em +0;
#   zread das entradas de 0x15 bytes contra a conta; as caixas "Error" (show_error_at) das linhas 0x60, 0x6c, 0x73.
# Comparadores: so a original (a leitura do save nao se repete) e o objeto conferido depois.
SAVE = True
CARGA = True  # a prova do save carrega o FeriasTesteSave com e sem as rotinas (prova_save.py)
ROTINAS = [('team_award_load_from_disk', 0x8af660, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('year_award_load_from_disk', 0x92fff0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('year_ratings_load_from_disk', 0x931390, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a carga dos prêmios de equipe e do ano e das notas do ano',
        'loading the team award, the year award and the year ratings',
        [('team_award_load_from_disk', 'TEAM_AWARD::load_from_disk', '0x8af660', 'award/team_award.cpp', 'thiscall, ret 4',
          'lê do save a data da atualização, o registro da seleção (0x205 bytes), a conta de reservas e os dois deslocamentos do histórico; 1, ou 0 com a caixa "Load Game Error" da primeira leitura curta',
          'reads from the save the update date, the selection record (0x205 bytes), the substitutes count and the two history offsets; 1, or 0 with the "Load Game Error" box of the first short read'),
         ('year_award_load_from_disk', 'YEAR_AWARD::load_from_disk', '0x92fff0', 'award/year_award.cpp', 'thiscall, ret 8',
          'lê do save os dois deslocamentos do histórico e, com escopo, monta a lista curta lida do save (sem memória, a caixa "Memory Allocation Error" e 0); 1, ou 0 com a caixa da leitura curta',
          'reads from the save the two history offsets and, with a scope, builds the shortlist read from the save (out of memory, the "Memory Allocation Error" box and 0); 1, or 0 with the box of the short read'),
         ('year_ratings_load_from_disk', 'YEAR_RATINGS::load_from_disk', '0x931390', 'award/year_ratings.cpp', 'thiscall, ret 4',
          'lê do save a conta das entradas e, com entradas, a capacidade igual à conta e as entradas de 21 bytes de uma vez; 1, ou 0 com a caixa "Error"',
          'reads from the save the entry count and, with entries, the capacity equal to the count and the 21-byte entries in one go; 1, or 0 with the "Error" box')])
CAMPOS = []
INTERESSA = ['SGE', 'CMRTE']
