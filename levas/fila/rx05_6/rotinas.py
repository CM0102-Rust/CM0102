# O texto da notícia da oferta recusada, rejeitada como inaceitável, cancelada ou retirada
# (TRANSFER_MANAGER::write_offer_rejected_news_text 0x8d5420, transfer_manager.cpp, 6.096 bytes, 0x8d5420..0x8d6be2;
# leitor rx05, rodada 6, 25/11/2026). Igual no cm0102-1080.exe. Contagem 0 na carreira ENG; 34 chamadas no mundo até
# 2002-08-01 (docs/cobertura_mundo.txt).
# Os 54 modelos e a ordem dos argumentos de cada um foram tirados da desmontagem por um passeio simbólico de todos os
# caminhos (rascunho rx05\sym.py): offer_rejected_template e offer_withdrawn_template.
# Convenções conferidas na desmontagem: thiscall(this, notícia, corpo, Text) ret 0xc, quadro SEH só para o estado da
# quantia; a quantia (vaga 10) por 0x43f460/0x441c10 com 200 bytes (não os 0x32 do news_text_money); as razões em 5
# buffers de 250 bytes e o texto delas em 0x418; "%s" pelo sprintf da CRT (0x944c9f) = cópia; a competição da vaga 11
# na tabela [0xae23d0] (0x6b bytes), vínculo 0x7d0. As comparações da vaga 8 são de word (test ax / cmp ax), as das
# vagas 7 e 9 de byte.
# Defeitos do original (a cópia mantém): no atraso recusado de uma oferta retirada (tipo 6) e no da troca, os clubes
# saem trocados com os gêneros na ordem das vagas; um resultado fora de 2 e 3 no título, ou um tipo de retirada sem
# texto, não escreve nada e o Text recebe o que sobrou no buffer comum.
# Comparador: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_offer_rejected_news_text', 0x8d5420, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto da notícia da oferta recusada ou retirada',
        'the news text of a rejected or withdrawn offer',
        [('write_offer_rejected_news_text', 'TRANSFER_MANAGER::write_offer_rejected_news_text', '0x8d5420', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título (lance de empréstimo ou de transferência recusado ou retirado, o clube que desiste do jogador, a transferência cancelada) ou o corpo: na recusa, pelo tipo (simples, sem lugar no elenco, sem poder contratar, atraso recusado, inaceitável, ou com até duas razões: indispensável, valioso, comprado há pouco, não vende a rival ou a outro clube da competição, importante para o futuro), pela troca, pelo empréstimo, pela quantia e por "a"/"uma"; na retirada, pelo motivo (termos pessoais, sem dinheiro, sem lugar, sem poder contratar, atraso); os vínculos ao vendedor, ao comprador, à pessoa e à competição',
          'the headline (loan or transfer bid rejected or withdrawn, the club giving up on the player, the transfer cancelled) or the body: for a rejection, by the kind (plain, no squad space, no more staff allowed, delay refused, unacceptable, or with up to two reasons: indispensable, valuable, recently bought, will not sell to a rival or to another club of the competition, important for the future), the part exchange, the loan, the cash and "the"/"a"; for a withdrawal, by the reason (personal terms, cannot afford, no space, no more staff, delay); the links to the seller, the bidder, the person and the competition')])
