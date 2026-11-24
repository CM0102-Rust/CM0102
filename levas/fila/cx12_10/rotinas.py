# O progresso dos clubes da Premier Division norte-irlandesa (nir_prm.cpp; leitor cx12, rodada 5, 24/11/2026).
# Simulacao com a Irlanda do Norte (LIGAS e ATE dos itens mwa): a virtual 0x68 a cada linha da tabela em cada
# atualizacao de status (a temporada de agosto de 2001 a abril de 2002).
# - 0x78efe0 NirPrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: Comp_Stats::find_club_entry
#   0x4a4850 (push clube; mov ecx, [ecx+8] -> thiscall, ret 4; sem teste), a melhor 9 vira 8 e a pior 9 vira 10 (9 =
#   a vaga da repescagem), a atual como vem; a fase nao e lida; grava +0x73, +0x74, +0x75: premier_play_off_progress
#   (aqui; o cx12_12 usa na Noruega, 12 -> 11 e 14). Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Igual no cm0102-1080.exe (inteira). Comparador: lado a lado nos 3 bytes da entrada (compare_c1_club_progress).
SAVE = False
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
ROTINAS = [('nir_prm_set_club_progress', 0x78efe0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o progresso dos clubes da Premier Division norte-irlandesa', 'the Northern Irish Premier Division club progress',
        [('nir_prm_set_club_progress', 'NirPrm::set_club_progress', '0x78efe0', 'nir_prm.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação na Premier norte-irlandesa: a vaga da repescagem (9) vira 8 como a melhor e 10 como a pior; a atual como vem',
          'the place forecast in the Northern Irish Premier: the play-off place (9) becomes 8 as the best and 10 as the worst; the current one as it comes')])
CAMPOS = []
INTERESSA = []
