# As viradas de temporada da Supercopa Europeia, da Copa dos Campeões e da Copa da UEFA (eur_super_cup.cpp,
# european_cup.cpp, uefa_cup.cpp em eurocomp\; leitor mwg, rodada 3 do mundo inteiro, 19/11/2026). Cada uma roda uma
# vez até 2002-08-01 (cobertura_mundo.txt), na virada das competições europeias; as copas europeias existem na carreira
# só com a Inglaterra (os clubes ingleses jogam nelas): LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x57a6d0..0x57a880, 0x584d60..0x584f92, 0x902ee0..0x9030ab): as três
# thiscall, ret 0; o molde das viradas de copa (start_new_cup_season, mwf_1) com as diferenças:
# - EurSuperCup: sem o livro de inscritos (+0x14) e sem as fases; depois da árvore, Cup::set_up_round(0) (0x51c800,
#   push 0; mov ecx -> thiscall, ret 4) e só então as posições previstas (virtual +0x5c), cujo eax devolve;
# - EuropeanCup: libera também o livro de inscritos (+0x14, entre +0xa7 e +0xa3) e destrói as fases 0..=+0x30 (virtual
#   0 com 1), +0x30 = -1; a semeadura da UEFA (0x9035a0 UefaSeeding::update_season, push 0; mov ecx, [0xde1f58] ->
#   thiscall, ret 4, só com o objeto); os inscritos (0x584fa0) falham com 0 (test eax; jne), a tabela de entrada das
#   preliminares (0x584990) com negativo; caixas nas linhas 0x40c, 0x413, 0x41a e 0x421;
# - UefaCup: o livro de inscritos e a semeadura, sem as fases; os inscritos 0x902b90; caixas 0x273, 0x27a e 0x281.
# As caixas de erro (0x944cff + "v%s %s.%s %d" + 0x5e8290, 0 em 0xb67a34) = show_error_at; depois dela o original
# devolve o eax da caixa, a cópia 0 (quem chama a virada não lê).
# Comparadores: só o original roda (libera e refaz a copa); o ano tem de subir 1 (compare_cup_new_season).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('eur_super_cup_start_new_season', 0x57a6d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('european_cup_start_new_season', 0x584d60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('uefa_cup_start_new_season', 0x902ee0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas de temporada da Supercopa Europeia, da Copa dos Campeões e da Copa da UEFA',
        'the season turnovers of the European Super Cup, the European Cup and the UEFA Cup',
        [('eur_super_cup_start_new_season', 'EurSuperCup::start_new_season', '0x57a6d0', 'eur_super_cup.cpp', 'thiscall, ret 0',
          'a virada da Supercopa Europeia: as tabelas da temporada passada liberadas (times, rodadas e confrontos de cada rodada), a temporada arquivada nas estatísticas, o ano + 1, a rodada atual e o "por fases" zerados; os dois clubes escolhidos, a tabela de rodadas e a árvore de confrontos refeitas (cada uma com a sua caixa de erro, linhas 0x118, 0x11f e 0x126, se negativa), a primeira rodada montada e as posições previstas',
          "the European Super Cup's turnover: last season's tables freed (teams, rounds and each round's ties), the season archived in the statistics, the year + 1, the current round and the run-by-stages flag cleared; the two clubs picked, the round table and the tie tree rebuilt (each with its error box, lines 0x118, 0x11f and 0x126, when negative), the first round set up and the predicted positions"),
         ('european_cup_start_new_season', 'EuropeanCup::start_new_season', '0x584d60', 'european_cup.cpp', 'thiscall, ret 0',
          'a virada da Copa dos Campeões: como a da Supercopa, e também o livro de inscritos liberado e as fases destruídas (fase atual −1); a semeadura da UEFA passa para a temporada nova; os inscritos (caixa na linha 0x40c se der 0), a tabela de entrada das preliminares (0x413), a tabela de rodadas (0x41a) e a árvore (0x421), e as posições previstas',
          "the European Cup's turnover: as the Super Cup's, and also the entrant book freed and the stages destroyed (current stage −1); the UEFA seeding moves to the new season; the entrants (box at line 0x40c when 0), the qualifying entry table (0x413), the round table (0x41a) and the tree (0x421), and the predicted positions"),
         ('uefa_cup_start_new_season', 'UefaCup::start_new_season', '0x902ee0', 'uefa_cup.cpp', 'thiscall, ret 0',
          'a virada da Copa da UEFA: as tabelas e o livro de inscritos liberados, a temporada arquivada, o ano + 1, a semeadura da UEFA na temporada nova; os inscritos, a tabela de rodadas e a árvore (caixas nas linhas 0x273, 0x27a e 0x281 se negativas) e as posições previstas',
          "the UEFA Cup's turnover: the tables and the entrant book freed, the season archived, the year + 1, the UEFA seeding in the new season; the entrants, the round table and the tree (boxes at lines 0x273, 0x27a and 0x281 when negative) and the predicted positions")])
