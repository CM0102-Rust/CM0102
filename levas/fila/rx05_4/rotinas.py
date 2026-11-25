# As notícias do jogador da troca sem permissão de trabalho, da cláusula de rescisão ativada e do contrato sem a
# permissão revista (transfer_manager.cpp; leitor rx05, rodada 6, 25/11/2026). As três inteiras (0x8e35a0..0x8e3776,
# 0x8e3db0..0x8e3f7e, 0x8e4290..0x8e4459), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01.
# Usa os auxiliares do rx05_1 (integrar junto com ele ou depois).
# Convenções conferidas na desmontagem e nos pontos de chamada (todas thiscall com quadro SEH sem destrutor no caminho
# normal):
# - 0x8e35a0 thiscall(this, oferta, pessoa, valor) ret 0xc; chamada por 0x8c7690 (em Rust). 0x7875e0 thiscall([0xb7431c],
#   clube) ret 4, primeiro o comprador e, sem humano nele, o vendedor.
# - 0x8e3db0 thiscall(this, pessoa, competição, tipo, contrato) ret 0x10; chamada por 0x4cced0 (em Rust, com 3).
#   0x7a7eb0 thiscall([0xb7434c], pessoa) ret 4 (al = 1: aposentando); 0x8cca10 thiscall(this, pessoa, 0) ret 8;
#   0x8d1da0 thiscall(this, pessoa, 0xa, lista, 0x14) ret 0x10. A vaga 7 é o dword +0x21 do contrato (release_fee).
# - 0x8e4290 thiscall(this, pessoa, proposta de contrato) ret 8; chamada por 0x8c38b0 (em Rust). Sem destinatários
#   calculados: o clube do contrato da pessoa, procurado de novo antes de cada uso (sem filtro do tipo 5).
# Defeito do original (0x8e4290): a segunda entrega compara o clube do contrato com o clube da proposta e, sendo
# outro, manda de novo ao clube do contrato; o clube da proposta nunca recebe a notícia (a cópia mantém).
# Comparadores: só a original (a notícia vai às caixas dos técnicos e aloca).
SAVE = False
ROTINAS = [('post_exchange_player_permit_refused_news', 0x8e35a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_release_clause_activated_news', 0x8e3db0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_contract_permit_refused_news', 0x8e4290, [0x6a, 0xff, 0x68, 0x6b, 0x48, 0x96, 0x00])]
DOCS = ('as notícias do jogador da troca sem permissão de trabalho, da cláusula de rescisão ativada e do contrato sem a permissão revista',
        'the news of the exchange player without a work permit, of the release clause activated and of the contract without the revised permit',
        [('post_exchange_player_permit_refused_news', 'TRANSFER_MANAGER::post_exchange_player_permit_refused_news', '0x8e35a0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a notícia 0xfbb de que o jogador da troca não obteve a permissão e saiu da oferta: o clube vendedor, o comprador, o alvo, o jogador da troca, o valor e o valor da oferta; vai ao comprador e ao vendedor (se houver), e só quando um dos dois é humano',
          'news 0xfbb that the exchange player got no permit and left the offer: the selling club, the buyer, the target, the exchange player, the value and the offer fee; it goes to the buyer and to the seller (if any), only when one of them is human'),
         ('post_release_clause_activated_news', 'TRANSFER_MANAGER::post_release_clause_activated_news', '0x8e3db0', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'a notícia 0xfbf da cláusula de rescisão ativada pelo resultado do clube: a pessoa, o clube do contrato, a competição, o tipo e o valor da cláusula; nada sem pessoa, competição ou contrato, para quem se aposenta, para quem tem transferência futura ou sem destinatários (até 20)',
          "news 0xfbf of the release clause activated by the club's result: the person, the contract club, the competition, the kind and the clause fee; nothing without a person, competition or contract, for a retiring player, for one with a future transfer or without recipients (up to 20)"),
         ('post_contract_permit_refused_news', 'TRANSFER_MANAGER::post_contract_permit_refused_news', '0x8e4290', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xbcc de que a proposta de contrato caiu por falta da permissão de trabalho revista: a pessoa e o clube da proposta; vai ao clube do contrato da pessoa e, se ele não for o da proposta, de novo a ele (defeito: o clube da proposta nunca recebe)',
          "news 0xbcc that the contract offer fell for lack of the revised work permit: the person and the offering club; it goes to the person's contract club and, when that is not the offering club, to it again (defect: the offering club never gets it)")])
