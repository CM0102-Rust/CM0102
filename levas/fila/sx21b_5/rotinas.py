# A oferta de transferencia no jogo salvo: carregar e gravar (transfer_offer.cpp; leitor sx21b, rodada 5, 24/11/2026).
# 0 chamadas na carreira (o gestor de transferencias as chama na gravacao, 0x8bd737/0x8bd81e, e na carga,
# 0x8bc2e9/0x8bc52b, de cada oferta): item do save, com a carga do FeriasTesteSave (CARGA).
# Convencoes conferidas: 0x8e5790 thiscall(fluxo, ofertas) ret 8 (quadro SEH: as listas novas); 0x8e5b30
# thiscall(fluxo) ret 4. 0x932c30 zread e 0x932e10 zwrite cdecl(buf, tamanho, quantidade, fluxo) -> quantidade;
# 0x68d020 LLIST::LLIST, 0x68d0d0 add_tail, 0x68d080 clear, 0x68d040 ~LLIST, 0x944cf1/0x944c94 new/delete.
# A caixa de erro (transfer_offer.cpp 0xa9e318) nas linhas 0x69, 0x70, 0x7a, 0x8d, 0x99, 0xbe, 0xd0 (carga) e 0xee,
# 0x102, 0x110, 0x125, 0x133 (gravacao); so a 0x99 segue adiante.
# Comparadores: so a original (leem e gravam o fluxo do save, alocam listas); a prova e a do save.
SAVE = True
CARGA = True
ROTINAS = [
    ('transfer_offer_load', 0x8e5790, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
    ('transfer_offer_save', 0x8e5b30, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
]
DOCS = ('a oferta de transferência no jogo salvo: carregar e gravar',
        'a transfer offer in the saved game: loading and saving it',
        [('transfer_offer_load', 'transfer_offer_load', '0x8e5790', 'transfer_offer.cpp', 'thiscall, ret 8',
          'regra: o registro de 0x41 bytes; a lista de ofertas rivais pelos índices gravados (um índice de oferta já carregada faz esta oferta usar a lista daquela; senão a lista lida fica sendo a dela); a lista de jogadores de troca, nós de 7 bytes',
          "rule: the 0x41-byte record; the rival offer list by the saved indices (an index of an offer already loaded makes this offer use that one's list; otherwise the list read becomes its own); the exchange player list, 7-byte nodes"),
         ('transfer_offer_save', 'transfer_offer_save', '0x8e5b30', 'transfer_offer.cpp', 'thiscall, ret 4',
          'regra: o registro de 0x41 bytes, a contagem e as entradas de 4 bytes da lista de rivais, a contagem e os nós de 7 bytes da lista de troca (sem lista, contagem 0)',
          'rule: the 0x41-byte record, the count and 4-byte entries of the rival list, the count and 7-byte nodes of the exchange list (no list, count 0)')])
CAMPOS = []
INTERESSA = []
