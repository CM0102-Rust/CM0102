# Os registros do tecnico pelo destino, a passagem de fase e a final do titulo da Terceira Divisao belga
# (bel_third.cpp; leitor cx09, rodada 5, 24/11/2026). Simulacao com a Belgica ('ENG BEL HOL GER DEN', ate 2002-08-01;
# na cobertura do mundo a 0x4230d0 roda 55 vezes, a 0x423640 e a 0x423710 uma vez, na primavera de 2002).
# - 0x4230d0 BelThird::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18 -> eax
#   (virtual 0x44): fase 0xff ou 0: status 0 Staff_Records::add_promotion_record 0x869480 (0x64), 1
#   add_qualified_for_round_record 0x86bdd0 (0, 0x458, 0x2d), 3 add_relegation_record 0x86a1c0; 0. Fase com sinal < 7:
#   -1. Fase 7: a linha (League::find_club_row 0x68ab70 (clube, 0) no grupo pelo +0x60 do clube = [0x9cf5a4]); status 1:
#   +0x37 = 5 e add_league_champions_record 0x868c50, 0; senao -1. Fase 8: clube da divisao (+0x57 = competicao) no
#   grupo, outro na Segunda ([[0xadadfc] + 4 x [0x9cf5a0]]); status 1: nosso 0 e promocao, o outro 0xfe; 2 ou 0xff:
#   nosso 0xfe, o outro 3 e rebaixamento; devolve o id da divisao do clube. Fase 9: so clube da divisao (senao -1);
#   status 1 0xfe; 2/0xff 3 e rebaixamento; 0. Acima de 9: 0. Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x423640 BelThird::advance_stage thiscall() ret 0 (virtual 0x28): +0x30 >= +0x2c - 1: a caixa (linha 0x567);
#   senao, com a Segunda completa (virtual 0x10 (0, 1), al), +0x30 + 1 e na 7: 0x423710, 0x423880, 0x423ba0 (pelos
#   desvios). Prologo sub esp, 0x200 (6 bytes).
# - 0x423710 BelThird::create_title_decider_stage thiscall() ret 0: os lideres dos grupos A e B, virtual 0x3c (7,
#   &rodadas, &extra, 0), operator new 0xb2 e Cup_Stage 0x522e00 (15 argumentos conferidos, ret 0x3c) na vaga 7; sem
#   fase a caixa (linha 0x59a, e a tabela de rodadas nao e liberada: vazamento do original); com ela Cup::set_up_round
#   0x51c800 (0), free da tabela, +0x30 = 7. Prologo push -1; push imm32.
# Comparadores: o dos registros so a original, conferida contra o plano da copia feito antes (retorno e status da
# linha); a passagem so a original; a final so a original, conferida depois (a vaga 7 e a fase atual).
# Iguais no cm0102-1080.exe. Depende do cx09_8 (BEL_THIRD_SOURCE); comp_object_of_key publicado no bra_nat_first.rs.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_update_staff_records_for_status', 0x4230d0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('bel_third_advance_stage', 0x423640, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bel_third_create_title_decider_stage', 0x423710, [0x6a, 0xff, 0x68, 0x7e, 0x41, 0x95, 0x00])]
DOCS = ('os registros pelo destino, a passagem de fase e a final do título da Terceira Divisão belga',
        'the Belgian Third Division staff records by outcome, the stage advance and the title decider',
        [('bel_third_update_staff_records_for_status', 'BelThird::update_staff_records_for_status', '0x4230d0', 'bel_third.cpp', 'thiscall, ret 0x18',
          'na liga e nos grupos: acesso, vaga na repescagem ou rebaixamento vão aos registros do técnico; na final do título o vencedor é campeão; na repescagem de acesso o clube da Terceira que vence sobe e o da Segunda que perde cai; na de rebaixamento quem perde cai',
          "in the league and the groups: promotion, a play-off place or relegation go to the manager's records; in the title decider the winner is champion; in the promotion play-off the Third Division club that wins goes up and the Second Division club that loses goes down; in the relegation play-off the loser goes down"),
         ('bel_third_advance_stage', 'BelThird::advance_stage', '0x423640', 'bel_third.cpp', 'thiscall, ret 0',
          'a Terceira Divisão belga só passa de fase depois que a Segunda termina; ao chegar à fase 7 monta a final do título e as duas repescagens',
          'the Belgian Third Division only moves on once the Second Division is over; on reaching stage 7 it builds the title decider and both play-offs'),
         ('bel_third_create_title_decider_stage', 'BelThird::create_title_decider_stage', '0x423710', 'bel_third.cpp', 'thiscall, ret 0',
          'a final do título: o líder do grupo A contra o do grupo B, em mata-mata (fase 7)',
          'the title decider: the group A leader against the group B leader, a knockout tie (stage 7)')])
CAMPOS = []
INTERESSA = []
