# As notícias do empréstimo cancelado, da espera das permissões de trabalho e da permissão concedida
# (transfer_manager.cpp; leitor rx05, rodada 6, 25/11/2026). As três inteiras (0x8e23d0..0x8e2560,
# 0x8e2a00..0x8e2cc2, 0x8e3130..0x8e32b1), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01: só rodam com um humano na oferta.
# Usa os auxiliares do rx05_1 (integrar junto com ele ou depois).
# Convenções conferidas na desmontagem e nos pontos de chamada (todas thiscall com quadro SEH sem destrutor no caminho
# normal):
# - 0x8e23d0 thiscall(this, oferta, motivo) ret 8; chamada por 0x8c2180 (em Rust). 0x8d1130 com 0xf; o motivo lido
#   como word com sinal; a vaga 7 é o id da divisão do comprador (+0x57) ou −1.
# - 0x8e2a00 thiscall(this, oferta) ret 4; chamada por 0x8c2f50 (em Rust). 0x8c7560 thiscall(this, pessoa, clube, 0)
#   ret 0xc: o alvo no comprador e cada jogador da troca no vendedor (o clube do contrato do alvo, nulo no tipo 5).
# - 0x8e3130 thiscall(this, pessoa, clube, oferta) ret 0xc; chamada por 0x8c7690 (em Rust). 0x8d1130 com 3.
# Comparadores: só a original (a notícia vai às caixas dos técnicos e aloca).
SAVE = False
ROTINAS = [('post_loan_cancelled_news', 0x8e23d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_work_permit_delay_news', 0x8e2a00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_work_permit_granted_news', 0x8e3130, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as notícias do empréstimo cancelado, da espera das permissões de trabalho e da permissão concedida',
        'the news of the cancelled loan, of the wait for work permits and of the permit granted',
        [('post_loan_cancelled_news', 'TRANSFER_MANAGER::post_loan_cancelled_news', '0x8e23d0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xfb7 do empréstimo cancelado: o jogador, o comprador, o clube do contrato, o motivo e a divisão do comprador (ou −1); vai aos destinatários da oferta (nenhum, nada)',
          "news 0xfb7 of the cancelled loan: the player, the buyer, the contract club, the reason and the buyer's division (or −1); it goes to the offer's recipients (none, nothing)"),
         ('post_work_permit_delay_news', 'TRANSFER_MANAGER::post_work_permit_delay_news', '0x8e2a00', 'transfer_manager.cpp', 'thiscall, ret 4',
          'a notícia 0xfb9 da transferência à espera das permissões: o alvo, o comprador, o clube do contrato (ou −1), se o alvo precisa de permissão no comprador, quantos jogadores da troca precisam no vendedor e o primeiro deles; vai aos destinatários da oferta (nenhum, nada)',
          "news 0xfb9 of the transfer waiting for permits: the target, the buyer, the contract club (or −1), whether the target needs a permit at the buyer, how many exchange players need one at the seller and the first of them; it goes to the offer's recipients (none, nothing)"),
         ('post_work_permit_granted_news', 'TRANSFER_MANAGER::post_work_permit_granted_news', '0x8e3130', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a notícia 0xfba da permissão de trabalho concedida: a pessoa, o clube de destino e o clube do contrato dela (ou −1); vai aos destinatários da oferta (nenhum, nada)',
          "news 0xfba of the work permit granted: the person, the destination club and his contract club (or −1); it goes to the offer's recipients (none, nothing)")])
