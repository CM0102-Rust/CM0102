# A varredura do calendario de jogos a partir de uma data (fix_man.cpp; leitor sbA, rodada 5, 24/11/2026). Carreira ENG
# (sem LIGAS; a cobertura conta 1 chamada ate 2002-06-01).
# - 0x5acee0 FIXTURE_MANAGER::scan_fixtures thiscall(&ano, &dia, &faixa, para frente, competicao, rodada minima,
#   rodada maxima, clube A, clube B, a10, a11, a12 (nao lidos), maximo (palavra; -1 sem, -2 o primeiro dia), lista,
#   jogado, fase, lado, com_estranhos) ret 0x48 -> contagem (palavra): as nacoes-chave pela competicao
#   (comp_active_key_nation_of_fixture 0x48dd70 do registro 0x6b) ou pelos clubes (0x48dee0, 0x5af5b0 e as entradas
#   continentais da tabela 0xb63d60), sem repetidas; as listas de cada dia e faixa (get_fixture_list 0x5abf10); os
#   filtros; LLIST::add_tail 0x68d0d0 (0x4f); a posicao escrita de volta.
# Convencoes conferidas: get_fixture_list thiscall(ano, dia, faixa, nacao) ret 0x10; os 18 argumentos pelos
# deslocamentos da pilha (0x23c..0x280 depois dos 4 push). Prologo sub esp, 0x228 (6 bytes). Sem relativa. Igual no
# cm0102-1080.exe.
# Comparador: a original na lista e na posicao de verdade; a copia numa lista temporaria (run_with_temp_list) e em
# copias da posicao: a contagem, os jogos copiados e a posicao escrita.
SAVE = False
ROTINAS = [('scan_fixtures', 0x5acee0, [0x81, 0xec, 0x28, 0x02, 0x00, 0x00])]
DOCS = ('a varredura do calendário de jogos a partir de uma data',
        'scanning the fixture calendar from a date',
        [('scan_fixtures', 'FIXTURE_MANAGER::scan_fixtures', '0x5acee0', 'fix_man.cpp', 'thiscall, ret 0x48',
          'percorre o calendário a partir de uma data e faixa, para frente ou para trás, copiando para a lista os jogos que passam nos filtros (competição, rodadas, clubes, lados, jogado ou não); para no máximo pedido, no primeiro dia com algo (−2) ou depois de um ano (−1) ou cinco, e devolve onde parou',
          'walks the calendar from a date and slice, forwards or backwards, copying into the list the fixtures that pass the filters (competition, rounds, clubs, sides, played or not); stops at the maximum asked, at the first day with something (−2) or after one year (−1) or five, and gives back where it stopped')])
CAMPOS = []
INTERESSA = []
