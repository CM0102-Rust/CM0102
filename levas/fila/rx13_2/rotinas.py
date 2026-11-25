# Os prêmios internacionais: o objeto e os seus seis prêmios (award/international_awards.cpp; leitor rx13, rodada 6,
# 25/11/2026). Simulação: rodam na criação do jogo (o AWARD_MANAGER monta os prêmios); os prêmios saem em 6 de maio,
# 6 de junho e 16 de junho de 2002 (o daily_update 0x632e20 já é Rust): ATE = '2002-08-01', como o rx13_1. Usa o molde
# comum do sx05b_5 (já no motor).
# - 0x632ac0 InternationalAwards::InternationalAwards thiscall(arquivo) ret 4: NationAwards 0x768290, vtable 0x96bb78,
#   +0xe = 0, +0x13 = 1, +0x12 = 4; com arquivo, NationAwards::load 0x76a280 (lido: fim); senão init_awards
#   0x632b30(arquivo). Sem notas do mês. Prólogo push -1; push imm32 (7 bytes).
# - 0x632b30 InternationalAwards::init_awards thiscall(arquivo) ret 4: +8 = 6, malloc(0x18) (nulo: caixa 0x63); seis
#   YEAR_AWARD 0x92fe80 (new 0x2b): (id, vaga, chave, rep. nacional, mundial, -1, 1, escopo, 0, arquivo, 1), cada push
#   conferido; só o primeiro (o jogador do ano da FIFA) com escopo: o endereço de um local de 14 bytes (5, três dwords
#   -1, 0xa) que o construtor passa à lista de indicados 0x417440. O índice só anda quando o new dá certo (a cópia fixa
#   as vagas). Prólogo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe. Comparadores: só a original (alocam), com o plano do molde: vtable, id 1, tipo 4, contagem
# 6; cada prêmio com o id e a vaga.
SAVE = False
ATE = '2002-08-01'
ROTINAS = [('international_awards_construct', 0x632ac0, [0x6a, 0xff, 0x68, 0x58, 0xb2, 0x95, 0x00]),
           ('international_awards_init', 0x632b30, [0x6a, 0xff, 0x68, 0xc4, 0xb2, 0x95, 0x00])]
DOCS = ('os prêmios internacionais: o objeto e os seus seis prêmios',
        'the international awards: the object and its six awards',
        [('international_awards_construct', 'InternationalAwards::InternationalAwards', '0x632ac0', 'award/international_awards.cpp', 'thiscall, ret 4',
          'regra: os prêmios internacionais não são de nenhuma nação; de um jogo salvo são lidos, senão criados',
          'rule: the international awards object belongs to no nation; from a saved game it loads its awards, otherwise it creates them'),
         ('international_awards_init', 'InternationalAwards::init_awards', '0x632b30', 'award/international_awards.cpp', 'thiscall, ret 4',
          'regra: os seis prêmios internacionais do ano são o jogador do mundo da FIFA (com lista de indicados), o futebolista do mundo, os futebolistas africano, europeu e sul-americano e o jogador da Oceania do ano, para jogadores, sem competição de clubes; reputações 9000/8000 (FIFA, mundo, Europa), 7000/5250 (África), 7000/6000 (América do Sul) e 5000/3000 (Oceania)',
          'rule: the six international awards of the year are the FIFA world player (with a shortlist), the world footballer, the African, European and South American footballers and the Oceania player of the year, for players, with no club competition; reputations 9000/8000 (FIFA, world, Europe), 7000/5250 (Africa), 7000/6000 (South America) and 5000/3000 (Oceania)')])
CAMPOS = []
INTERESSA = []
