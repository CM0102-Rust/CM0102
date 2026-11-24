# O acesso e o rebaixamento da Argentina, o troca-troca da Segunda com a Terceira e a noticia do rebaixamento por
# promedio (arg_prm.cpp; leitor cx08, rodada 5, 24/11/2026). Simulacao com a Argentina (as ligas do cx01_8, ate
# 2002-08-01): as duas primeiras rodam na virada de 2002 (ArgPrm::start_new_season, cx08_4); o texto no fim do Clausura,
# com a noticia dos rebaixados.
# - 0x4067f0 ArgPrm::process_promotion_relegation_chain thiscall(pares) ret 4 (virtual 0xb0; pares nao lido): as
#   virtuais 0xa4 desta e da Segunda ([[0xadadfc] + [0x9cf658] x 4]); na tabela da Segunda status 0 ou 5:
#   League::move_club_to_division 0x6830b0 (thiscall, push 0; push +4; push clube), senao
#   CONTRACT_MANAGER::trigger_non_promotion_clauses 0x4cced0 (ecx = [0xae1050]; push +4 da Segunda; push clube); na
#   tabela desta, status 3: League::relegate_club_to_division 0x6831a0 (push 0; push [0xae23d0] + id x 0x6b; push clube).
#   Prologo mov eax, [0xadadfc] (5 bytes, endereco absoluto).
# - 0x4068d0 arg_prm_swap_second_and_third_division_clubs thiscall() ret 0: a virtual 0x34 da Segunda (push 1); malloc
#   0xa0 (caixa 0x4b8); ate 40 clubes da Terceira ([0x9cf688]) com +0x57, +0x53 e +0x60 nao nulos; nenhum: free e fim;
#   comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (cdecl; lista, n, 6); ate 4 com status 3 na Segunda descem
#   (relegate_club_to_division nesta liga, push 1) e o mesmo numero da lista sobe (move_club_to_division, push 1), cada
#   vaga zerada. Prologo sub esp, 0x20c.
# - 0x406aa0 ArgPrm::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, corpo, Text) ret 0x28
#   (virtual 0x48): fase 0xff, desfecho 3 e corpo: o texto 0x987420 ("{clube} have been relegated from the {competicao}
#   as a result of their poor average of points over the last three seasons."), o Text e os vinculos 0x7d5 e 0x7d0;
#   senao Comp::build_fate_news_text 0x48c6d0 (a7 = 0). Prologo mov eax, [esp+0xc]; push ebx (5 bytes).
# Defeito do original: o troca-troca nao libera a lista dos clubes da Terceira depois de subir (160 bytes por virada).
# Os auxiliares comp_record_by_key e relegated_by_average_text ficam aqui (o cx08_10 os usa).
# Comparadores: a cadeia e o troca-troca so a original (movem clubes, sorteiam); o texto lado a lado (compare_text_writer,
# sem partida; com o modo CM_TEXTO_NOTICIAS).
# Depende do cx08_4 (update_relegation_averages_virtual): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_prm_process_promotion_relegation_chain', 0x4067f0, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('arg_prm_swap_second_and_third_division_clubs', 0x4068d0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('arg_prm_build_fate_news_text', 0x406aa0, [0x8b, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('o acesso e o rebaixamento na Argentina e a notícia do rebaixamento por promédio', 'the Argentine promotion and relegation and the relegation-by-average news',
        [('arg_prm_process_promotion_relegation_chain', 'ArgPrm::process_promotion_relegation_chain', '0x4067f0', 'arg_prm.cpp', 'thiscall, ret 4',
          'o acesso e o rebaixamento argentinos: as posições finais da Primeira e da Segunda ficam gravadas; os clubes da Segunda marcados 0 (campeão) ou 5 sobem para a Primeira e os outros têm as cláusulas de não acesso disparadas; os da Primeira marcados como rebaixados descem para a Segunda',
          'the Argentine promotion and relegation: the final positions of the Premier and the Second Division are recorded; the Second Division clubs marked 0 (champion) or 5 go up to the Premier Division and the others have their non-promotion clauses triggered; the Premier Division clubs marked relegated go down to the Second Division'),
         ('arg_prm_swap_second_and_third_division_clubs', 'arg_prm_swap_second_and_third_division_clubs', '0x4068d0', 'arg_prm.cpp', 'thiscall, ret 0',
          'até quatro clubes da Segunda Divisão marcados como rebaixados descem para a Terceira e o mesmo número de clubes da Terceira sobe, escolhidos entre os mais fortes (ordenados e embaralhados entre os 6 primeiros). Defeito do original: a lista dos clubes não é liberada',
          'up to four Second Division clubs marked relegated go down to the Third Division and as many Third Division clubs come up, chosen among the strongest (sorted and shuffled among the top 6). Defect of the original: the club list is never freed'),
         ('arg_prm_build_fate_news_text', 'ArgPrm::build_fate_news_text', '0x406aa0', 'arg_prm.cpp', 'thiscall, ret 0x28',
          'a notícia de um clube rebaixado da Primeira Divisão argentina pelo promédio: "{clube} foi rebaixado da {competição} por causa do seu mau promédio de pontos nas últimas três temporadas"; qualquer outro destino, o texto comum das competições',
          'the news of a club relegated from the Argentine Premier Division by its average: "{club} have been relegated from the {competition} as a result of their poor average of points over the last three seasons"; any other fate, the competitions\' common text')])
CAMPOS = []
INTERESSA = []
