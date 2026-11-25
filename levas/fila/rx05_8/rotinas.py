# Os textos das notícias da permissão de trabalho concedida, da cláusula de rescisão ativada e do jogador da troca sem
# permissão (transfer_manager.cpp; leitor rx05, rodada 6, 25/11/2026). As três inteiras (0x8e32c0..0x8e359d,
# 0x8e3f90..0x8e4285, 0x8e3780..0x8e3b75), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01.
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc (0x8e32c0 sem quadro SEH;
# 0x8e3f90 e 0x8e3780 com quadro SEH só para o estado da quantia); 0x6181d0 nation_get_nationality_adjective cdecl
# (nação do clube +0x53, buf, 0x64, 8); 0x43f460/0x441c10 (a quantia: 200 bytes na 0x8e3f90, 100 na 0x8e3780); na
# 0x8e3f90 o nome da pessoa vai em 200 bytes (0x53e140 com 0xc8, não 0x12c) e a competição pelo nome longo (+4, gênero
# +0x37), com vínculo 0x7d0 só se [0xadadfc][comp] não for nulo. Na 0x8e3780 as vagas 6..9 são lidas primeiro e a
# taxa nova é comparada com sinal (jle) com a antiga.
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_work_permit_granted_news_text', 0x8e32c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('write_release_clause_activated_news_text', 0x8e3f90, [0x6a, 0xff, 0x68, 0x4b, 0x48, 0x96, 0x00]),
           ('write_exchange_player_permit_refused_news_text', 0x8e3780, [0x6a, 0xff, 0x68, 0x0b, 0x48, 0x96, 0x00])]
DOCS = ('os textos das notícias da permissão de trabalho concedida, da cláusula de rescisão ativada e do jogador da troca sem permissão',
        'the news texts of the work permit granted, of the release clause activated and of the exchange player without a permit',
        [('write_work_permit_granted_news_text', 'TRANSFER_MANAGER::write_work_permit_granted_news_text', '0x8e32c0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "permissão concedida" ou o corpo "(o jogador do clube) recebeu uma permissão condicional <nacionalidade> para ir ao <clube>"; os vínculos aos clubes e à pessoa',
          'the headline "granted work permit" or the body "(the club\'s) player has been granted a conditional <nationality> work permit for his move to <club>"; the links to the clubs and the person'),
         ('write_release_clause_activated_news_text', 'TRANSFER_MANAGER::write_release_clause_activated_news_text', '0x8e3f90', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "cláusula de rescisão ativada" ou o corpo "pode deixar o clube por <valor> pelo rebaixamento (tipo 3) ou pela falta de acesso na <competição>"; os vínculos à pessoa, ao clube e à competição (se ela tiver objeto)',
          'the headline "release clause activated" or the body "can now leave the club for <fee> due to its relegation (kind 3) or to it not achieving promotion from the <competition>"; the links to the person, the club and the competition (when it has an object)'),
         ('write_exchange_player_permit_refused_news_text', 'TRANSFER_MANAGER::write_exchange_player_permit_refused_news_text', '0x8e3780', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "não obtém permissão de trabalho" ou o corpo "o jogador saiu da troca pelo <alvo> sem a permissão <nacionalidade>", com o valor novo da transferência quando ele passa o antigo; os vínculos aos clubes, ao alvo e ao jogador',
          'the headline "fails to get work permit" or the body "the player has been removed from the part exchange offer for <target> as he failed to get a <nationality> work permit", with the new fee when it is above the old one; the links to the clubs, the target and the player')])
