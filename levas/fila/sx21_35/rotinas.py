# A oferta de transferencia: refeita como lance em dinheiro (transfer_offer.cpp; leitor sx21, rodada 5, 24/11/2026).
# Roda nas ofertas da carreira ENG (22.895 chamadas no mundo ate 2002-08-01).
# Convencoes conferidas: 0x8e78f0 thiscall(jogador, clube, taxa) ret 0xc; sem jogador ou clube, a caixa da linha 0x4b1
# de transfer_offer.cpp; 0x4cf5a0 thiscall(pessoa) ret 4 (guardado em 0xae1080, duas vezes); 0x90cfc0 cdecl(20).
# Comparadores: os 0x41 bytes da oferta, G_CONTRACT_LOOKUP_RESULT e o estado do sorteio, lado a lado.
SAVE = False
ROTINAS = [
    ('init_transfer_offer', 0x8e78f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
]
DOCS = ('a oferta de transferência: refeita como lance em dinheiro',
        'a transfer offer: reset as a cash bid',
        [('init_transfer_offer', 'init_transfer_offer', '0x8e78f0', 'transfer_offer.cpp', 'thiscall, ret 0xc',
          'regra: entre clubes da mesma divisão, o jogador famoso (reputação atual acima de 7000) fica sabendo do lance com uma chance em 20 (+0x2d); a oferta fica com o comprador, o alvo, a taxa, as cláusulas zeradas com 10 jogos cada, cargo 0xb, sem listas e sem situação',
          'rule: between clubs of the same division, a famous player (current reputation above 7000) hears of the bid with one chance in 20 (+0x2d); the offer gets the buyer, the target, the fee, the clauses cleared with 10 games each, job 0xb, no lists and no status')])
CAMPOS = []
INTERESSA = []
