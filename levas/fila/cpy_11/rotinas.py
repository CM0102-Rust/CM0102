# Os jogos e as faltas das permissoes de trabalho e os premios por jogos depois de cada partida, o premio por jogos na
# selecao guardado e a noticia da transferencia finalizada (transfer_manager.cpp).
# - 0x8c39f0 (update_appearance_clauses_after_match, 19414 chamadas ate 2002-06-01 na cobertura de 17/10): conta os
#   jogos (+4) e as faltas (+6) das permissoes de trabalho, que a 0x8c7950 le (jogos > 3 x faltas ou faltas < 4) em
#   toda avaliacao de permissao; paga os premios por jogos (0x8cb340) aos 1, 10, 20... 50 jogos.
# - 0x8cad90 (add_international_team_appearance_money, 1 chamada): a gemea da 0x8cac40 (cpl_3), no vetor +0x898.
# - 0x8da5e0 (post_transfer_finalised_news, 5 chamadas): a noticia 0xfa5.
# Convencoes conferidas nos pontos de chamada da original (lição da 258a):
#   0x53b900 e_selecao: cdecl (add esp, 4), 1 arg. 0x5c6710 e_amistoso: thiscall (mov ecx, [0xae2c74]), 1 arg.
#   0x541f90 reunir_elenco: cdecl (add esp, 0xc), 3 args. 0x4cf6d0 ficha_de_contrato e 0x4cf5a0 contrato_de: thiscall
#   (mov ecx, [0xae1050]), 1 arg. 0x8cb340 pagar_premio: thiscall (mov ecx, edi = this), 4 args. 0x8cb630 / 0x8cb590:
#   thiscall (mov ecx, edi), 1 arg. 0x946464 bsearch: cdecl (add esp, 0x14), 5 args. 0x62c400 estado_da_lesao: thiscall
#   (mov ecx, [0xb63d28]), 1 arg. 0x55d270 esta_suspenso: thiscall (mov ecx, [0xae28f0]), 4 args. 0x7757d0 convocacao:
#   thiscall (mov ecx, [0xb74318]), 5 args. 0x8cf5e0 inelegibilidade: thiscall (mov ecx, edi), 2 args.
#   0x944e46 malloc / 0x945667 realloc / 0x945333 qsort: cdecl (add esp 4/8/0x10) (pelo cpl3_crescer e em linha).
#   0x54a110 data_padrao: thiscall (lea ecx, [esp+0x14]), 0 args. 0x77bbe0 construir: thiscall (lea ecx, local), 2 args.
#   0x785790 gravar: thiscall (lea ecx, local), 2 args. 0x8e7a70 total: thiscall (mov ecx, esi = oferta), 4 args.
#   0x8cee70 conclusao: thiscall (mov ecx, ebx = this), 2 args. 0x7875e0 humano: a original poe ecx = [0xb7431c] e
#   empurra 1 arg; a rotina e stdcall (ret 4) e nao le ecx: declarada thiscall(jornal, clube), mesma pilha.
#   0x7861e0 entregar: thiscall (mov ecx, [0xb7431c]), 2 args.
# Comparadores: a 0x8c39f0 em ensaio (sem pagar os premios) contra a original a partir do mesmo vetor de permissoes
# (selecao em casa: so a original, pelo elenco local com lixo); a 0x8cad90 pelo plano (conta e registro); a noticia so a
# original (aloca). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('jogos_das_permissoes_e_clausulas', 0x8c39f0, [0x81, 0xec, 0xc8, 0x01, 0x00, 0x00]),
           ('guardar_premio_por_jogos_na_selecao', 0x8cad90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_transferencia_finalizada', 0x8da5e0, [0x6a, 0xff, 0x68, 0x96, 0x44, 0x96, 0x00])]
DOCS = ('os jogos e as faltas das permissões de trabalho e os prêmios por jogos depois da partida, o prêmio por jogos na seleção guardado e a notícia da transferência finalizada',
        'the appearances and absences of work permits and the appearance money after the match, the international appearance money stored and the news of the finalised transfer',
        [('jogos_das_permissoes_e_clausulas', 'update_appearance_clauses_after_match', '0x8c39f0', 'transfer_manager.cpp', 'thiscall',
          'depois da partida, nos dois lados: paga o prêmio por jogos aos 1, 10, 20... 50 jogos (clube pelo contrato, seleção pelos jogos desde o acordo) e tira o bônus pago; no jogo que conta, soma um jogo à permissão de trabalho de quem jogou e uma falta à dos que ficaram de fora sem lesão, suspensão, convocação nem empréstimo',
          'after the match, on both sides: pays the appearance money at 1, 10, 20... 50 games (club by contract, national team by caps since the deal) and removes the paid bonus; in a counting match, adds an appearance to the work permit of who played and an absence to those left out without injury, ban, call-up or loan'),
         ('guardar_premio_por_jogos_na_selecao', 'add_international_team_appearance_money', '0x8cad90', 'transfer_manager.cpp', 'thiscall',
          'acrescenta o prêmio por jogos na seleção (13 bytes) ao vetor, crescendo de 50 em 50, e reordena pela pessoa',
          'appends the international appearance money (13 bytes) to the array, growing by 50, and sorts it by person'),
         ('noticia_transferencia_finalizada', 'post_transfer_finalised_news', '0x8da5e0', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfa5 da transferência acertada, com a data de conclusão, ao vendedor e ao comprador humanos',
          'the 0xfa5 news of the agreed transfer, with the completion date, to the human seller and buyer')])
CAMPOS = [('jogo +0 (−2), +0x14 competição, +0x1c casa, +0x20 fora, +0x40/+0x41 o lado conta; valor/ponteiro; lê (jogos_das_permissoes_e_clausulas)',
           'fixture +0 (−2), +0x14 competition, +0x1c home, +0x20 away, +0x40/+0x41 side counts; value/pointer; reads (jogos_das_permissoes_e_clausulas)'),
          ('partida + lado × 0x625 + vaga × 0x49: +0x190 id, +0x1ad, +0x1c2; id/valor; lê', 'match + side × 0x625 + slot × 0x49: +0x190 id, +0x1ad, +0x1c2; id/value; reads'),
          ('gerência de transferências +0x86e: registro de 8 bytes por pessoa (bit 2 cláusula do clube, 4 da seleção, 0x20 permissão; +7 jogos pela seleção no acordo); valor; lê',
           'transfer manager +0x86e: 8-byte record per person (bit 2 club clause, 4 national, 0x20 permit; +7 caps at the deal); value; reads'),
          ('permissão de trabalho (0xe bytes em [+0x93a], conta +0x93e) +4 jogos, +6 faltas; valor; grava +1 (jogos_das_permissoes_e_clausulas)',
           'work permit (0xe bytes at [+0x93a], count +0x93e) +4 appearances, +6 absences; value; writes +1 (jogos_das_permissoes_e_clausulas)'),
          ('pessoa +0x22 jogos pela seleção, +0x61 ficha; valor/ponteiro; lê', 'person +0x22 caps, +0x61 player record; value/pointer; reads'),
          ('contrato +0x3b jogos pelo clube, +4 clube, +0x35 & 0x3f tipo; valor; lê', 'contract +0x3b club appearances, +4 club, +0x35 & 0x3f type; value; reads'),
          ('gerência de transferências +0x898 prêmios na seleção (13 bytes), +0x89c capacidade, +0x8a0 conta; ponteiro/valor; grava (guardar_premio_por_jogos_na_selecao)',
           'transfer manager +0x898 international appearance money (13 bytes), +0x89c capacity, +0x8a0 count; pointer/value; writes (guardar_premio_por_jogos_na_selecao)'),
          ('oferta +4 troca, +8 comprador, +0xc pessoa, +0x11, +0x1b valor, +0x20, +0x24, +0x2c estado, +0x2f, +0x30; valor; lê (noticia_transferencia_finalizada)',
           'offer +4 exchange, +8 buyer, +0xc person, +0x11, +0x1b fee, +0x20, +0x24, +0x2c state, +0x2f, +0x30; value; reads (noticia_transferencia_finalizada)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava', '[0xae1080] last contract looked up; pointer; writes')]
INTERESSA = ['CMRTE']
