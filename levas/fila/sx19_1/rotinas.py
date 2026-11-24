# O orgao disciplinar sul-americano (rb_south_america.cpp: o construtor e as regras dos cartoes) e as regras
# "escalonadas" que Portugal, Espanha, Turquia e Pais de Gales repetem (stepped_card_verdict, own_rules; leitor sx19,
# rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, ruling_body_class_construct, CardVerdict,
# ImmediateBan, plain_card_verdict, pending_points_ban, run_card_rules, compare_card_rules,
# compare_ruling_body_class_built): integrar depois dele. Os outros itens sx19_* de simulacao usam as pecas deste:
# integrar este antes deles.
# Roda na carreira ENG: o construtor sempre (Discipline::create_rule_sets, regra 2, Always); as regras nas partidas
# das competicoes sul-americanas de selecoes (eliminatorias da Copa de 2002 ate novembro de 2001) e de clubes.
# Convencoes conferidas na desmontagem:
# - 0x7e29e0 thiscall(arquivo) ret 4: 0x7e7760 (base, construct_ruling_body), vtable 0x96ee34, +4 = 2, +5 = 2, +6 = 0xa,
#   +0x17 = -1, +0x1b America do Sul (0x9cfa1c), +0x1f/+0x20 = 0xff, +0x21 = 2, +0x22 = 0;
# - 0x7e2a30 thiscall(linha do registro, partida) ret 8: o vermelho (+0x2d == 1) da 1 jogo (motivo 2, quantidade 1);
#   cada partida com amarelo soma 1 na entrada e a cada segundo da mais 1 jogo (motivo 1, quantidade 2); regras
#   ff ff 02 ff; comeca hoje; 0x7e82e0 find_or_add thiscall(id, 1) ret 8; 0x556940 add_ban; 0x557280 post_ban_news.
# Defeito do original (o mesmo de todos os orgaos): o byte de marcas da suspensao (+0x13) sai com os bits 0, 1 e 3..7
# do lixo da pilha; a copia grava 0 e o comparador olha so o bit 2.
SAVE = False
ROTINAS = [('rb_south_america_construct', 0x7e29e0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_south_america_process_match_cards', 0x7e2a30, [0x6a, 0xff, 0x68, 0xe8, 0x04, 0x96, 0x00])]
DOCS = ('o órgão disciplinar sul-americano e as regras escalonadas dos cartões',
        'the South American ruling body and the stepped card rules',
        [('rb_south_america_construct', 'RbSouthAmerica::RbSouthAmerica', '0x7e29e0', 'ruling_body/rb_south_america.cpp', 'thiscall, ret 4',
          'monta o órgão sul-americano sobre a base: tabela virtual 0x96ee34, a vaga 2 do registro, a regra 2, a escala 10, sem nação, a América do Sul e ele mesmo como regra da confederação',
          'builds the South American body on the base: vtable 0x96ee34, record slot 2, rule set 2, scale 10, no nation, South America and itself as the confederation rule'),
         ('rb_south_america_process_match_cards', 'RbSouthAmerica::process_match_cards', '0x7e2a30', 'ruling_body/rb_south_america.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras sul-americanas: um jogo pelo vermelho e um a cada segundo amarelo (um por partida), a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the South American rules: one match for a red card and one for every second booking (one per match), from today, with its news')])
CAMPOS = []
INTERESSA = []
