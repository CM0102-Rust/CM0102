# As fases da Segunda Divisao argentina e as tres zonas (arg_second.cpp; leitor cx08, rodada 5, 24/11/2026). Simulacao com
# a Argentina (as ligas do cx01_8, ate 2002-08-01): o advance_stage roda no fim de cada fase (dezembro de 2001 para as
# zonas, abril e maio de 2002 para a copa de acesso e as repescagens); as zonas no fim da primeira fase.
# - 0x4080c0 ArgSecond::advance_stage thiscall() ret 0 (virtual 0x28): na ultima fase a caixa 0x2eb; na fase 3 espera a
#   fase 0 da Primeira acabar (o al da sua virtual 0xc, Comp::all_stages_complete na vtable da League); +0x30 + 1 e, pela
#   nova fase, 0x4081b0 (0), 0x408630 (3) ou 0x408910 (4). Prologo sub esp, 0x200.
# - 0x4081b0 ArgSecond::create_zone_stages thiscall() ret 0: malloc 0x60 (caixa 0x31b) com os clubes das linhas de cada
#   zona (A: 1, 10-15, 24; B: 2, 7-9, 16-18, 23; C: 3-6, 19-22); para g = 0..2 a virtual 0x3c (g, &rodadas, &rotulo, 0; o
#   push 0 do ultimo argumento vem antes, logo depois do malloc), operator new 0xee e League_Stage 0x68b100 (8 clubes,
#   2 turnos, lugares 1,0,0,0, bits 8, modo 2) na vaga g, free da tabela; as tres criadas: +0x30 = 2, senao a caixa
#   0x359. Prologo push -1; push imm32.
# Defeito do original: a lista das zonas nunca e liberada (96 bytes por temporada).
# Comparadores: so a original; o avanco pela fase atual (igual ou + 1); as zonas pelos clubes de cada fase contra o plano
# e +0x30 = 2.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_advance_stage', 0x4080c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('arg_second_create_zone_stages', 0x4081b0, [0x6a, 0xff, 0x68, 0x4a, 0x36, 0x95, 0x00])]
DOCS = ('as fases e as zonas da Segunda Divisão argentina', 'the Argentine Second Division stages and zones',
        [('arg_second_advance_stage', 'ArgSecond::advance_stage', '0x4080c0', 'arg_second.cpp', 'thiscall, ret 0',
          'a Segunda Divisão argentina passa pelas fases: as três zonas depois da primeira fase, a copa de acesso depois das zonas (só quando o Clausura da Primeira acabou) e as repescagens de acesso',
          "the Argentine Second Division moves through its stages: the three zones after the first phase, the promotion cup after the zones (only once the Premier Division's Clausura is over) and the promotion play-offs"),
         ('arg_second_create_zone_stages', 'ArgSecond::create_zone_stages', '0x4081b0', 'arg_second.cpp', 'thiscall, ret 0',
          'depois da primeira fase os 24 clubes abaixo do líder vão para três zonas de oito pela colocação (A: 2º, 11º a 16º e 25º; B: 3º, 8º a 10º, 17º a 19º e 24º; C: 4º a 7º e 20º a 23º), cada uma em ida e volta. Defeito do original: a lista das zonas não é liberada',
          'after the first phase the 24 clubs below the leader go into three zones of eight by their place (A: 2nd, 11th to 16th and 25th; B: 3rd, 8th to 10th, 17th to 19th and 24th; C: 4th to 7th and 20th to 23rd), each home and away. Defect of the original: the zone list is never freed')])
CAMPOS = []
INTERESSA = []
