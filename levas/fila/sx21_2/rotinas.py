# As regras de transferencia de base, parte 2 (transfer_offer.cpp; leitor sx21, rodada 5, 24/11/2026): quem nao pode
# jogar uma partida (slot 0x18, resposta de base de todas as nacoes; roda nas partidas da carreira ENG), os dias ate o
# fim da janela aberta (das noticias de janela do gestor de transferencias, todo dia) e o destrutor (so no fim do
# jogo ou ao trocar de jogo: nao roda na carreira de teste; o comparador so roda a original).
# Convencoes conferidas: 0x8e8a60 thiscall(jogador, partida) ret 8, al; 0x8e8ed0 thiscall ret 0, ax;
# 0x8e8540 thiscall ret 0; 0x7e6f20 e 0x5c6710 thiscall(comp) ret 4; 0x4cf5a0/0x4cf6d0 thiscall(pessoa) ret 4;
# 0x53b900 cdecl(clube); 0x40a7d0 thiscall(saida, ano) ret 8; 0x54c8e0 thiscall(saida, n) ret 8;
# 0x54cdb0 thiscall(dia+ano, bissexto) ret 8.
# Comparadores: o codigo (al) e G_CONTRACT_LOOKUP_RESULT lado a lado; os dias (ax); o destrutor so a original.
SAVE = False
ROTINAS = [
    ('transfer_rules_get_ineligibility_code', 0x8e8a60, [0x8b, 0x41, 0x15, 0x53, 0x8b, 0x5c, 0x24, 0x0c]),
    ('transfer_rules_get_days_until_window_closes', 0x8e8ed0, [0x6a, 0xff, 0x68, 0x20, 0x4a, 0x96, 0x00]),
    ('destroy_transfer_rules', 0x8e8540, [0xc7, 0x01, 0x58, 0x04, 0x97, 0x00]),
]
DOCS = ('as regras de transferência de base: quem não pode jogar, os dias até o fim da janela e o destrutor',
        "the base transfer rules: who may not play, the days to the window's close and the destructor",
        [('transfer_rules_get_ineligibility_code', 'transfer_rules_get_ineligibility_code', '0x8e8a60', 'transfer_offer.cpp', 'thiscall, ret 8',
          'regra: contrato do tipo 5 fora de amistoso e de reservas: 3 (salvo +0x15); o emprestado, quando o clube atual joga a partida (e não é jogo de seleção): 10 numa competição que não é de nenhuma das divisões do clube, amistoso ou de reservas (sem o byte +0x1c do empréstimo), 11 contra o clube dono (sem o byte +0x1d); senão 0',
          "rule: a type 5 contract outside friendlies and reserve games: 3 (unless +0x15); a loanee whose current club plays the match (not a national team match): 10 in a competition that is none of the club's divisions, friendly or reserve (without the loan's byte +0x1c), 11 against the owning club (without byte +0x1d); else 0"),
         ('transfer_rules_get_days_until_window_closes', 'transfer_rules_get_days_until_window_closes', '0x8e8ed0', 'transfer_offer.cpp', 'thiscall, ret 0',
          'regra: os dias de hoje até o fechamento da janela aberta hoje (o deste ano se ainda não passou, senão o do ano que vem); sem janela aberta, 365',
          "rule: the days from today to the close of the window open today (this year's if not yet past, else next year's); without an open window, 365"),
         ('destroy_transfer_rules', 'destroy_transfer_rules', '0x8e8540', 'transfer_offer.cpp', 'thiscall, ret 0',
          'regra: a vtable de base de volta e a tabela de janelas liberada',
          'rule: the base vtable back and the window table freed')])
CAMPOS = []
INTERESSA = []
