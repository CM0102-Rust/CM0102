# Os prêmios europeus: o objeto, os seus quatro prêmios e o dia dos prêmios (award/european_awards.cpp; leitor rx13,
# rodada 6, 25/11/2026). Simulação: o construtor e o init na criação do jogo (o AWARD_MANAGER monta os prêmios), o
# daily_update todo dia; os prêmios saem em 16 de junho de 2002 (2001 é pulado): ATE = '2002-08-01'. Usa o molde comum
# do sx05b_5 (já no motor) e o awards_day_is / FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS do international_awards.rs.
# - 0x583400 EuropeanAwards::EuropeanAwards thiscall(arquivo) ret 4: NationAwards 0x768290, vtable 0x96a1bc, +0xe = 0,
#   +0x13 = 2, +0x12 = 2; com arquivo, NationAwards::load 0x76a280 (lido: fim); senão init_awards 0x583480(arquivo).
#   Sem notas do mês. Prólogo push -1; push imm32 (7 bytes).
# - 0x583480 EuropeanAwards::init_awards thiscall(arquivo) ret 4: +8 = 4, malloc(0x10) (nulo: caixa 0x60); quatro
#   YEAR_AWARD 0x92fe80 (new 0x2b) com (id +0x13, vaga, [0x9cfd6c..0x9cfd78], 8000, 7000, -1, 1, 0, 0, arquivo, 1),
#   cada push conferido. O índice só anda quando o new dá certo (a cópia fixa as vagas). Prólogo mov eax, fs:[0] (6).
# - 0x5836a0 EuropeanAwards::daily_update thiscall(hoje) ret 4: ano 2001 nada; Date 0x549f70(0x10, 5, ano, -1) contra
#   hoje (as duas palavras); LLIST local 0x68d020; quatro 0x768720 thiscall(hoje, prêmio k, [[0xad9c60]+0x10],
#   [0x9cfa10], máscara 1/6/0x38/0x40, 1, &lista) ret 0x1c; ~LLIST 0x68d040. Prólogo mov eax, fs:[0] (6).
# Iguais no cm0102-1080.exe. Comparadores: só a original (alocam; os prêmios saem uma vez), com o plano do molde no
# construtor (vtable, id 2, tipo 2, contagem 4) e no init (cada prêmio com o id e a vaga).
SAVE = False
ATE = '2002-08-01'
ROTINAS = [('european_awards_construct', 0x583400, [0x6a, 0xff, 0x68, 0xd8, 0x89, 0x95, 0x00]),
           ('european_awards_init', 0x583480, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('european_awards_daily_update', 0x5836a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os prêmios europeus: o objeto, os seus quatro prêmios e o dia dos prêmios',
        'the European awards: the object, its four awards and the award day',
        [('european_awards_construct', 'EuropeanAwards::EuropeanAwards', '0x583400', 'award/european_awards.cpp', 'thiscall, ret 4',
          'regra: os prêmios europeus não são de nenhuma nação; de um jogo salvo são lidos, senão criados',
          'rule: the European awards object belongs to no nation; from a saved game it loads its awards, otherwise it creates them'),
         ('european_awards_init', 'EuropeanAwards::init_awards', '0x583480', 'award/european_awards.cpp', 'thiscall, ret 4',
          'regra: a Europa tem quatro prêmios do ano: goleiro, defensor, meio-campista e atacante do ano, cada um com reputação nacional 8000 e mundial 7000, para jogadores, sem competição de clubes',
          'rule: Europe has four awards of the year, the goalkeeper, the defender, the midfielder and the striker of the year, each with national reputation 8000 and world reputation 7000, for players, with no club competition'),
         ('european_awards_daily_update', 'EuropeanAwards::daily_update', '0x5836a0', 'award/european_awards.cpp', 'thiscall, ret 4',
          'regra: em 16 de junho de cada temporada, menos 2001, os três melhores jogadores europeus do ano por posição (goleiros, defensores, meio-campistas e atacantes), pelas notas mundiais do ano; uma lista só para os quatro',
          'rule: on 16 June of every season but 2001, the best three European players of the year by position (goalkeepers, defenders, midfielders and strikers) from the world year ratings; one list for the four')])
CAMPOS = []
INTERESSA = []
