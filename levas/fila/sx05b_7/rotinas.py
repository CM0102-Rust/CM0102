# A gravacao do premio de equipe, do premio do ano e da tabela das notas do ano (award/team_award.cpp, year_award.cpp,
# year_ratings.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa team_award_obj, year_award_obj (sx05b_5) e TEAM_AWARD_CPP,
# YEAR_AWARD_CPP (sx05b_6): integrar depois deles.
# Rodam ao gravar o jogo (as virtuais de gravacao dos premios, chamadas pelo AWARD_MANAGER; 0x9314e0 por 0x414292 e
# 0x4142a3). Item de save (SAVE): a prova grava o .qst pela original e pela Rust e compara.
# Convencoes conferidas na desmontagem: thiscall(arquivo) ret 4; zwrite 0x932e10 cdecl(origem, tamanho, n, arquivo)
# contra 1 (e contra a conta nas entradas das notas); a caixa "Error" (0x90d130, "v%s %s.%s %d", 0x5e8290 e o botao do
# mouse zerado) nas linhas 0x96, 0x9d, 0xa4, 0xab, 0xb2 (equipe), 0x94, 0x9b (ano) e 0x86, 0x8f (notas); o premio do
# ano grava a lista curta por 0x417720 thiscall(arquivo) ret 4 quando ha. O eax de saida nao e lido pelos chamadores
# (virtual sem retorno; a copia devolve nada).
# Comparadores: so a original (o arquivo do save nao se grava duas vezes); a prova do save compara os .qst.
SAVE = True
ROTINAS = [('team_award_save_to_disk', 0x8af870, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('year_award_save_to_disk', 0x9301f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('year_ratings_save_to_disk', 0x9314e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a gravação dos prêmios de equipe e do ano e das notas do ano',
        'saving the team award, the year award and the year ratings',
        [('team_award_save_to_disk', 'TEAM_AWARD::save_to_disk', '0x8af870', 'award/team_award.cpp', 'thiscall, ret 4',
          'grava o prêmio de equipe como a carga o lê: a data da atualização, o registro da seleção, a conta de reservas e os dois deslocamentos do histórico; a gravação curta mostra a caixa "Error" e para',
          'saves the team award as the load reads it: the update date, the selection record, the substitutes count and the two history offsets; a short write shows the "Error" box and stops'),
         ('year_award_save_to_disk', 'YEAR_AWARD::save_to_disk', '0x9301f0', 'award/year_award.cpp', 'thiscall, ret 4',
          'grava os dois deslocamentos do histórico e a lista curta, quando há',
          'saves the two history offsets and the shortlist, when there is one'),
         ('year_ratings_save_to_disk', 'YEAR_RATINGS::save_to_disk', '0x9314e0', 'award/year_ratings.cpp', 'thiscall, ret 4',
          'grava a conta das entradas e, com entradas, as entradas de 21 bytes de uma vez',
          'saves the entry count and, with entries, the 21-byte entries in one go')])
CAMPOS = []
INTERESSA = ['SGE', 'CMRTE']
