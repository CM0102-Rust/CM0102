# A final da repescagem, o dia e os registros dos tecnicos da Segunda Divisao B espanhola (spa_second_b.cpp; leitor rx09,
# rodada 6, 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01 (a repescagem
# e a final em maio e junho de 2002; os registros no fim de cada fase).
# - 0x857a00 SpaSecondB::create_relegation_playoff_final thiscall() ret 0: as duas primeiras entradas de status 2 (+5)
#   da tabela de times (+0xa7, 6 bytes; +0x3e relido) da fase 7 num local de 2; a virtual +0x3c(8, ...); Cup_Stage
#   0x522e00 (this, 2, lista, rodadas, registro, tabela, ano, 8, 1, extra, 0x14, 0, 0, 0, 0) em +0xc[8]; sem ela, caixa
#   0x371; senao free da tabela, Cup::set_up_round(0) 0x51c800 e +0x30 = 8.
# - 0x858160 SpaSecondB::update_for_date thiscall(hoje, completo) ret 8: com +0x30 == 7 e a virtual +0x10(0, 0) da fase
#   7 verdadeira (al), a virtual +0x28 (advance_stage); depois League::update_for_date 0x6847c0 com os dois argumentos.
# - 0x857b70 SpaSecondB::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18 -> 0: as
#   quatro ramificacoes conferidas (fase < 3; 3..6; 7; 8); o grupo pela reserva (+0x60) do clube; para um id fora dos
#   quatro grupos o original usa o proprio status como o grupo (defeito: le [status + 0x3e]); a linha pelo clube no
#   inicio dela; +0x37 da linha = 0, 0xfe ou 3; Staff_Records 0x86bdd0 (clube, registro, 0x438, 0x41c, 0x28), 0x869480
#   (clube, registro, 0x64) e 0x86a1c0 (clube, registro) no G_STAFF_RECORDS.
# Prologos: mov eax, fs:[0] (6 bytes); push esi; mov esi, ecx; cmp dword [esi+0x30], 7 (7 bytes); mov al, [esp+0xc];
# push ebx (5 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original nas tres; os planos conferidos depois (os clubes da final, a fase 8 depois da
# repescagem, o status da linha e o resultado 0).
# Depende do rx09_3 (league_cup_stage, compare_league_cup_stage, segunda_b_group), do rx09_2 (SEGUNDA_B_GROUPS) e do
# cx14_6 (stage_round_table).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('spa_second_b_create_relegation_playoff_final', 0x857a00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('spa_second_b_update_for_date', 0x858160, [0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x07]),
           ('spa_second_b_update_staff_records_for_status', 0x857b70, [0x8a, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('a final da repescagem, o dia e os registros dos técnicos da Segunda Divisão B espanhola',
        'the relegation play-off final, the daily update and the staff records of the Spanish Second Division B',
        [('spa_second_b_create_relegation_playoff_final', 'SpaSecondB::create_relegation_playoff_final', '0x857a00', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: os dois perdedores da repescagem do rebaixamento jogam a final dela, ida e volta; o perdedor cai',
          'rule: the two losers of the relegation play-off play its final, two legs; the loser goes down'),
         ('spa_second_b_update_for_date', 'SpaSecondB::update_for_date', '0x858160', 'spa_second_b.cpp', 'thiscall, ret 8',
          'regra: quando a repescagem do rebaixamento termina, a Segunda B passa à final dela; depois o dia comum da liga',
          'rule: when the relegation play-off is over the Second Division B moves on to its final; then the common league day'),
         ('spa_second_b_update_staff_records_for_status', 'SpaSecondB::update_staff_records_for_status', '0x857b70', 'spa_second_b.cpp', 'thiscall, ret 0x18',
          'regra: nos grupos, quem vai ao acesso ganha o registro de classificação; nos grupos do acesso o vencedor ganha o de acesso e a linha dele no grupo fica 0, os outros 0xfe; na repescagem quem escapa fica 0xfe; na final o que escapa fica 0xfe e o que cai fica 3, com o registro de rebaixamento',
          'rule: in the groups, a club through to the play-off gets the qualified record; in the play-off groups the winner gets the promotion record and its group row becomes 0, the others 0xfe; in the relegation play-off a club that stays up becomes 0xfe; in its final the one that stays up becomes 0xfe and the one that goes down 3, with the relegation record')])
CAMPOS = []
INTERESSA = []
