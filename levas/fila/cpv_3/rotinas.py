# As ofertas em espera no fechamento da janela e duas notícias (transfer_manager.cpp, manager_manager.cpp): quando a
# janela de transferências de uma nação fecha, completa as ofertas aprovadas e põe em espera as outras dessa nação
# (TRANSFER_MANAGER::hold_offers_at_window_close 0x8c4e10, pelo dia das janelas: as regras da nação, a retirada ou a
# recusa das ofertas do computador e o reagendamento para o dia seguinte), a notícia das expectativas da federação
# para a seleção (MANAGER_MANAGER::post_fa_expectations_news 0x6a44a0) e a do protesto da torcida contra o técnico
# (post_supporter_protest_news 0x6a5d10). Notícias montadas na pilha, de volta pelo critério novo (31/10/2026).
# Chamadas originais pelo endereço: 0x53b3d0 (seleção), a virtual +0xc das regras da nação, 0x8c34e0, 0x4cf5a0,
# 0x4cca70, 0x4ccb30, 0x68d1b0, 0x7875e0, 0x8e1150, 0x8bfcb0, 0x8bf680, 0x600a50, 0x8c6220, 0x77bbe0, 0x785790,
# 0x786090. Iguais no cm0102-1080.exe. Comparadores: só a original nas três (mexem nas ofertas, liberam, alocam
# notícias); as cópias levam os ramos e se provam pela carreira e pelas mutações.
SAVE = False
ROTINAS = [('ofertas_em_espera_na_janela', 0x8c4e10, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x0c]),
           ('noticia_das_expectativas_da_federacao', 0x6a44a0, [0x6a, 0xff, 0x68, 0x2b, 0xce, 0x95, 0x00]),
           ('noticia_do_protesto_da_torcida', 0x6a5d10, [0x6a, 0xff, 0x68, 0xeb, 0xce, 0x95, 0x00])]
DOCS = ('as ofertas em espera no fechamento da janela, a notícia das expectativas da federação e a do protesto da torcida',
        'the offers on hold when the window closes, the FA expectations news and the supporter protest news',
        [('ofertas_em_espera_na_janela', 'hold_offers_at_window_close', '0x8c4e10', 'transfer_manager.cpp', 'thiscall',
          'na janela que fecha, completa as ofertas aprovadas e põe em espera as da nação: notícia se há humano, senão retirada ou recusada; do computador, status 3 e de volta no dia seguinte',
          'when the window closes, completes the approved offers and puts the nation\'s offers on hold: news if a human is involved, otherwise withdrawn or rejected; for the computer, status 3 and back the next day'),
         ('noticia_das_expectativas_da_federacao', 'post_fa_expectations_news', '0x6a44a0', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x177e da expectativa da federação para a seleção numa competição',
          'the 0x177e news of the FA\'s expectation for the national team in a competition'),
         ('noticia_do_protesto_da_torcida', 'post_supporter_protest_news', '0x6a5d10', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x1782 do protesto da torcida contra o técnico',
          'the 0x1782 news of the supporters protesting against the manager')])
CAMPOS = [('[this+0x8ac] as 28 regras de transferência por nação (+0xa: nação; virtual +0xc: segurar a oferta); ponteiro; lê (ofertas_em_espera_na_janela)',
           '[this+0x8ac] the 28 transfer rules per nation (+0xa: nation; virtual +0xc: hold the offer); pointer; reads (ofertas_em_espera_na_janela)'),
          ('oferta +0x2c: status (0xe completar, 0x13 fora, 1 em espera, 3 reagendada); valor; lê e grava (ofertas_em_espera_na_janela)',
           'offer +0x2c: status (0xe complete, 0x13 out, 1 on hold, 3 rescheduled); value; reads and writes (ofertas_em_espera_na_janela)'),
          ('oferta +0x30: marcas (bit 0 em espera, bits 1..3 limpos); +0x10: termos (zerados); valor; grava (ofertas_em_espera_na_janela)',
           'offer +0x30: flags (bit 0 on hold, bits 1..3 cleared); +0x10: terms (cleared); value; writes (ofertas_em_espera_na_janela)'),
          ('oferta +4: lista da troca (item +4: estado, zerado); +0x20: vaga; lê e grava (ofertas_em_espera_na_janela)',
           'offer +4: exchange list (item +4: state, cleared); +0x20: slot; reads and writes (ofertas_em_espera_na_janela)'),
          ('clube +0x53: nação; ponteiro; lê (ofertas_em_espera_na_janela)',
           'club +0x53: nation; pointer; reads (ofertas_em_espera_na_janela)'),
          ('pessoa +0x61: ficha de jogador; ponteiro; lê (ofertas_em_espera_na_janela)',
           'person +0x61: player record; pointer; reads (ofertas_em_espera_na_janela)'),
          ('clube +0xcf: técnico; ponteiro; lê (noticia_do_protesto_da_torcida)',
           'club +0xcf: manager; pointer; reads (noticia_do_protesto_da_torcida)'),
          ('[0xdd6ec8] data a partir da qual saem as notícias; valor; lê (noticia_do_protesto_da_torcida)',
           '[0xdd6ec8] date from which the news are posted; value; reads (noticia_do_protesto_da_torcida)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (ofertas_em_espera_na_janela)',
           '[0xae1080] the last contract asked for; pointer; writes (ofertas_em_espera_na_janela)')]
INTERESSA = ['CMRTE']
