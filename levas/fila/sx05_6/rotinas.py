# A selecao do premio de equipe (TEAM_AWARD, award/team_award.cpp; leitor sx05, rodada 5, 23/11/2026): o candidato
# oferecido a selecao (add_candidate 0x8afa40, chamado pelas selecoes da semana e da temporada das nacoes,
# 0x769a10/0x769c04/0x76a1ff), o esvaziamento das 20 vagas (clear_selection 0x8afd90) e as mascaras de posicao das
# vagas pela formacao (init_slot_positions 0x8afdc0, chamado pelo construtor 0x8af600, no inicio do jogo).
# Convencoes conferidas na desmontagem:
# - 0x8afa40 thiscall(this, pessoa, clube, nota (f32), mascara (word, 0 = a da ficha)) ret 0x10, devolve eax 0/1 (o
#   chamador 0x769a10 empilha 0, a nota por fstp, o clube e a pessoa); 0x5b7d20 formation_get_position_mask
#   cdecl(ficha); 0x85add0 SQUAD_MANAGER::club_uses_squad_numbers thiscall([0xdd7ef0], clube, 0) ret 8; 0x85a760
#   get_squad_number thiscall([0xdd7ef0], pessoa, clube) ret 8 (o numero em al, com sinal, guardado em modulo);
# - as vagas: 0x19 bytes a partir de +0x4e (id, tres ids de nome em +0x33 dos registros de nome da pessoa, id do
#   clube, numero, nota f32), as mascaras em +0x26 (word por vaga), 11 + [+0x253] vagas, [+0x252] preenchidas, o modo
#   da formacao em +0x25c; a troca copia as 25 bytes da vaga (rep movsd 6 + movsb);
# - 0x8afd90 e 0x8afdc0 thiscall(this), sem argumentos.
# Os premios de equipe sao das nacoes com premio (a Inglaterra tem a selecao da semana e da temporada): LIGAS = 'ENG'
# e ATE = '2002-08-01', como os outros itens de premio (sx05_2..4) e o mwg_5.
# Comparadores: lado a lado sobre as mascaras, as vagas e a conta (+0x26..+0x253) e o retorno.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('team_award_add_candidate', 0x8afa40, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('team_award_clear_selection', 0x8afd90, [0x56, 0x8d, 0x41, 0x52, 0xbe, 0x14, 0x00, 0x00, 0x00]),
           ('team_award_init_slot_positions', 0x8afdc0, [0x8a, 0x81, 0x5c, 0x02, 0x00, 0x00])]
DOCS = ('a seleção do prêmio de equipe',
        'the team award selection',
        [('team_award_add_candidate', 'TEAM_AWARD::add_candidate', '0x8afa40', 'award/team_award.cpp', 'thiscall, ret 0x10',
          'oferece um jogador à seleção: sem pessoa, ficha ou clube, nada; já escolhido, a nota sobe se for maior; senão as posições (a máscara dada ou a da ficha) são alargadas (2 dá também 4; 8 ou 0x20 dão 0x10; 0x40 com 0x80 ou 0x800 dá 0x200) e ele ocupa a primeira vaga vazia que lhe serve; sem nenhuma, a primeira vaga ocupada que lhe serve cujo ocupante caiba numa vaga vazia: o ocupante muda para ela e ele fica com a vaga. O número da camisa (em módulo) vale quando o clube usa números fixos. Devolve 1 com a seleção completa',
          'offers a player to the selection: without a person, a sheet or a club, nothing; already chosen, his rating rises when higher; otherwise his positions (the given mask or his sheet\'s) are widened (2 also gives 4; 8 or 0x20 give 0x10; 0x40 with 0x80 or 0x800 gives 0x200) and he takes the first empty slot that fits him; with none, the first filled slot that fits him whose occupant fits an empty slot: the occupant moves there and he takes the slot. His squad number (absolute) counts when the club uses squad numbers. Returns 1 when the selection is complete'),
         ('team_award_clear_selection', 'TEAM_AWARD::clear_selection', '0x8afd90', 'award/team_award.cpp', 'thiscall',
          'esvazia as 20 vagas (id, ids de nome e clube −1, nota 0; o número fica) e zera a conta das preenchidas',
          'empties the 20 slots (id, name ids and club −1, rating 0; the number stays) and zeroes the filled count'),
         ('team_award_init_slot_positions', 'TEAM_AWARD::init_slot_positions', '0x8afdc0', 'award/team_award.cpp', 'thiscall',
          'as máscaras de posição das vagas: os onze pela formação (modos 0, 1 e 2 os seus, qualquer outro o quarto) e os nove reservas sempre goleiro e três vezes (0x40, 4, 0x10)',
          'the slots\' position masks: the eleven by the formation (modes 0, 1 and 2 their own, any other the fourth) and the nine substitutes always goalkeeper then three times (0x40, 4, 0x10)')])
CAMPOS = []
INTERESSA = []
