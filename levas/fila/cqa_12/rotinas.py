# Os registros das copas e da troca de tecnico (staff_records.cpp):
# - 0x86b0b0 (Staff_Records::add_cup_runners_up_record): pela final das copas (Cup 0x519890 / copa_registros_pelo_desfecho,
#   Liga dos Campeoes 0x584ae0 e as copas das nacoes): a reputacao do vice (+0x80, no banco) sobe.
# - 0x86b710 (Staff_Records::add_cup_third_place_record): pelas copas com disputa de terceiro (selecoes, mundiais): pode
#   nao rodar ate 2002-06-01 (passageira).
# - 0x869aa0 (Staff_Records::record_manager_job_change): pela saida do tecnico 0x69b790 ("cpq_1 remove the manager" 16) e
#   pelo tecnico novo 0x694c80 ("cpp_3 open spell" 128).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x53d980 pessoa_e_estrangeira: cdecl (add esp, 0xc em 0x86b1eb e 0x86b846), 3 args (push 1, push nacao +0x53, push
#   tecnico). 0x945640 _ftol: x87, pelo ftol (fild do produto em 32 bits, fmul qword [0x96ff88]). 0x76d940
#   clube_especial: o chamador poe ecx = [0xb74318] (0x86bc53), 1 arg (push competicao), a rotina nao le o ecx
#   (stdcall no motor.rs). 0x774f90 encerrar_convocacoes_da_nacao: thiscall (mov ecx, [0xb74318] em 0x86bc68 e
#   0x86bc7c), 2 args (push competicao, push clube). 0x58e340 (historico): thiscall (ecx = [this] ou [this+4]), 3 args,
#   pelo cpq4_historico e pelo cqa12_historico_do_clube. 0x54a110/0x417c10: thiscall (a data), pelo cpq4_entrada,
#   cpq4_datar e cqa12_copiar (dst no ecx, push src, push &local). 0x53b900 e_selecao: cdecl (add esp, 4 em 0x869ae9),
#   1 arg. 0x86f5b0 registro_do_tecnico: thiscall (mov ecx, edi = this em 0x869af7 e 0x869d3e), 2 args (push 1, push
#   tecnico). 0x54cdb0 dias_ate: thiscall (mov ecx, 0xae2c90 em 0x869ba5/0x869c23; mov ecx, ebx = registro + 0x7e /
#   + 0x6a em 0x869bc3/0x869c41), 2 args (a data por valor: push [+4], push [+0]), ret 8. 0x549f70 data_dma: thiscall
#   (lea ecx, local em 0x869d12), 4 args (push -1, push 0x7b7, push 0xb, push 0xd), ret 0x10, devolve o this.
#   As proprias: thiscall, ret 0xc (os chamadores poem ecx = [0xdd7f00]).
# Comparadores: os tres em ensaio (as copias sem os historicos em arquivo e, no terceiro, sem acabar as convocacoes);
# os colocados conferem a reputacao do clube e as tres notas do lado dele na partida; a troca de tecnico confere o
# registro do tecnico (criado antes pela 0x86f5b0, como a original faria) e a data estatica 0xae2470 da 0x54cdb0. Sem
# sorteio. Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('vice_da_copa_nos_registros', 0x86b0b0, [0x6a, 0xff, 0x68, 0x46, 0x2c, 0x96, 0x00]),
           ('terceiro_da_copa_nos_registros', 0x86b710, [0x6a, 0xff, 0x68, 0x66, 0x2c, 0x96, 0x00]),
           ('troca_de_tecnico_nos_registros', 0x869aa0, [0x6a, 0xff, 0x68, 0xc1, 0x2b, 0x96, 0x00])]
DOCS = ('os registros das copas e da troca de técnico: o vice, o terceiro e a passagem do técnico pelo clube',
        "the cup and manager change records: the runner-up, the third place and the manager's spell at the club",
        [('vice_da_copa_nos_registros', 'Staff_Records::add_cup_runners_up_record', '0x86b0b0', 'staff_records.cpp', 'thiscall',
          'o vice da copa: a reputação do clube sobe até peso × 500 (no máximo 350 de uma vez), as três notas do lado dele na partida pelo peso, e os históricos do técnico, do presidente, da escalação e do clube',
          "the cup runner-up: the club reputation rises up to weight × 500 (at most 350 at once), its side's three match ratings by the weight, and the manager, chairman, line-up and club histories"),
         ('terceiro_da_copa_nos_registros', 'Staff_Records::add_cup_third_place_record', '0x86b710', 'staff_records.cpp', 'thiscall',
          'o terceiro da copa: o mesmo com peso × 200 (no máximo 200) e notas menores; acaba as convocações nas competições guardadas',
          'the cup third place: the same with weight × 200 (at most 200) and smaller ratings; ends the call-ups in the stored competitions'),
         ('troca_de_tecnico_nos_registros', 'Staff_Records::record_manager_job_change', '0x869aa0', 'staff_records.cpp', 'thiscall',
          'fecha a passagem atual do técnico (a mais curta e a mais longa), abre a nova quando contratado e conta as passagens; os históricos do técnico, do presidente e do clube',
          "closes the manager's current spell (shortest and longest), opens the new one when hired and counts the spells; the manager, chairman and club histories")])
CAMPOS = [('clube +0x80 reputação; valor; lê e grava (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)',
           'club +0x80 reputation; value; reads and writes (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)'),
          ('clube +0xcf técnico, +0xbf presidente, +0x53 nação; ponteiros; lê (os três)', 'club +0xcf manager, +0xbf chairman, +0x53 nation; pointers; reads (all three)'),
          ('partida +0x188/+0x18a/+0x18c notas do lado (lado × 0x625), +0x190 escalação (20 × 0x49 bytes, id ou −1); valores/ids; grava as notas, lê a escalação (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)',
           'match data +0x188/+0x18a/+0x18c side ratings (side × 0x625), +0x190 line-up (20 × 0x49 bytes, id or −1); values/ids; writes the ratings, reads the line-up (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)'),
          ('competição +0x69 peso, +0x42 fase; valores; lê (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)', 'competition +0x69 weight, +0x42 stage; values; reads (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)'),
          ('registro do técnico +0x4a clube atual, +0x4e começo, +0x56 data, +0x5e/+0x62/+0x6a passagem mais longa, +0x72/+0x76/+0x7e mais curta, +0xb4 passagens, +0xb5 seleções; ids/datas/valores; grava (troca_de_tecnico_nos_registros)',
           'manager record +0x4a current club, +0x4e start, +0x56 date, +0x5e/+0x62/+0x6a longest spell, +0x72/+0x76/+0x7e shortest, +0xb4 spells, +0xb5 national jobs; ids/dates/values; writes (troca_de_tecnico_nos_registros)'),
          ('[0xdd6ec8] começo do jogo, [0xae2c90] hoje, [0xae2470] data estática da 0x54cdb0; datas; lê, grava a estática (troca_de_tecnico_nos_registros)',
           '[0xdd6ec8] game start, [0xae2c90] today, [0xae2470] static date of 0x54cdb0; dates; reads, writes the static (troca_de_tecnico_nos_registros)')]
INTERESSA = ['CMRTE']
