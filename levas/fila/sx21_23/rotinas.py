# A Argentina: de que oferta um clube argentino pode participar (leitor sx21, rodada 5, 24/11/2026): roda nas
# transferencias dos clubes argentinos, so com a Argentina carregada. O texto da noticia do prazo foi para a sx21_31
# (tipo TELAS).
# Convencoes conferidas: 0x40a900 thiscall(oferta, clube, data, motivo) ret 0x10 (guarda o contrato em 0xae1080);
# 0x8e8e50 thiscall(data, saida) ret 8.
# Depende da sx21_1 (transfer_rules_can_make_offer, compare_rule_check): integrar junto ou depois dela.
# Comparadores: a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT, lado a lado.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ROTINAS = [
    ('argentina_rules_can_make_offer', 0x40a900, [0x51, 0x53, 0x55, 0x56, 0x57]),
]
DOCS = ('a Argentina: de que oferta um clube argentino pode participar',
        'Argentina: which offers an Argentine club may take part in',
        [('argentina_rules_can_make_offer', 'argentina_rules_can_make_offer', '0x40a900', 'argentina_rules.cpp', 'thiscall, ret 0x10',
          'regra: na segunda janela, a transferência não pode levar o clube além de duas contratações (0xe; o vendedor conta os jogadores de troca); fora das janelas 9; senão a regra de base',
          'rule: in the second window, the move may not take the club past two signings (0xe; the seller counts the exchange players); outside the windows 9; else the base rule')])
CAMPOS = []
INTERESSA = []
