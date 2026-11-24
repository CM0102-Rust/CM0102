# Os textos das noticias da oferta de cargo e da reacao da diretoria a uma liberacao (manager_manager.cpp; leitor sx14,
# rodada 5, 23/11/2026). So escritos quando a noticia e aberta na caixa do tecnico humano (0 chamadas na carreira de
# teste): item de TELAS. Chamados pelas noticias (0x77bc80, Global(0xb67a44, ..)): thiscall(MANAGER_MANAGER, noticia,
# corpo, Text), ret 0xc; o this nao e lido.
# Convencoes conferidas na desmontagem: 0x785830 thiscall ret 4 (os campos lidos na ordem do original, inclusive os
# que so a manchete ou so o corpo leem); 0x53e140 cdecl(n1, n2, n3, buf, 0x190, modo, id) (modo 5 na manchete, 1 no
# corpo); 0x53e990 cdecl(pessoa, n1, n2, n3, 0); 0x66f4e0 cdecl, os argumentos de cada formato conferidos push a push.
# Prologos `mov eax, [esp+8]; sub esp, 0x19c` (10 bytes, fronteira de instrucao, sem relativo).
# Comparador: compare_text_writer.
# Caminho no jogo (para o Lucas provar comparando), a caixa de noticias do tecnico humano:
# - 0x1775 oferta de cargo: quando um clube oferece o cargo de tecnico a alguem do clube do humano (o assistente ou um
#   jogador veterano dele); aparece em Noticias;
# - 0x1778 reacao a liberacao: liberar um jogador valorizado (perfil do jogador > "Release"/"Free transfer"), com a
#   diretoria contrariada ou bloqueando.
# 24/11/2026: provado pela carreira automatica com o modo de textos (CM_TEXTOS=1 na cadeia); deixou de ser TELAS.
ROTINAS = [('news_job_offer_text', 0x6a0690, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('news_board_release_reaction_text', 0x6a0ad0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x9c, 0x01, 0x00, 0x00])]
DOCS = ('os textos das notícias da oferta de cargo e da reação da diretoria a uma liberação',
        "the news texts of a job offer and of the board's reaction to a release",
        [('news_job_offer_text', 'news_job_offer_text', '0x6a0690', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ("<clube> oferece a <nome> o cargo de técnico") ou o corpo da notícia de que um clube ofereceu o cargo a alguém (um modelo para jogador, cargos 0xb, 0xd e 0xf, e outro para os demais), com os vínculos da pessoa (se o nome ainda é o dela) e dos clubes; defeito: a manchete junta o gênero do nome completo do clube ao nome curto',
          'the headline ("<club> offer <name> the position of manager") or body of the news that a club offered someone its manager\'s job (one template for a player, jobs 0xb, 0xd and 0xf, another for the others), with the links to the person (when the name is still his) and to the clubs; defect: the headline pairs the gender of the club\'s full name with its short name'),
         ('news_board_release_reaction_text', 'news_board_release_reaction_text', '0x6a0ad0', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete (bloqueio, choque, irritação ou surpresa, pelas marcas 0x800, 0x400 e 1) ou o corpo (pelas marcas 0x800 com ou sem 0x200, 0x400, 1, 0x200 ou nenhuma) da reação da diretoria à liberação de um jogador ou ao bloqueio da transferência livre, com os vínculos do jogador e, no corpo, do clube',
          "the headline (block, shock, upset or surprise, by the flags 0x800, 0x400 and 1) or body (by the flags 0x800 with or without 0x200, 0x400, 1, 0x200 or none) of the board's reaction to a player's release or to the blocked free transfer, with the links to the player and, in the body, to the club")])
