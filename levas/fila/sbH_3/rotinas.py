# O que uma data guarda para um clube nos amistosos (friendly.cpp; leitor sbH, rodada 5, 24/11/2026; a sobra do sbG,
# ver levas/fila/sbG_5/FORA.md). Simulacao, carreira ENG (sem LIGAS, como o sbG_4 e o cx05_12): o dia dos amistosos
# (Friendly::create_due_planned_friendlies 0x5c2ae0, ja em Rust, pelo club_date_status: kind 0, sem datas escolhidas,
# com o estadio) pergunta por cada amistoso planejado; as telas de amistosos (0x45fcfd, 0x470870, 0x483238, 0x4834ca,
# 0x4837af) perguntam com datas escolhidas e o kind 0xb. Recursiva (0x5c7bd7).
# - 0x5c75a0 Friendly::get_club_date_status thiscall(clube, &data, &torneio, &amistoso, &pedido, datas, n (palavra),
#   kind (byte), estadio) ret 0x24 -> al (3,2 KB ate o ret 0x24 de 0x5c81c8; tabelas de salto 0x5c81cc, 0x5c81fc e
#   0x5c822c depois dele). Lida pela desmontagem com o quadro normalizado (scratchpad sbB\frame.py; o quadro +0x468 e o
#   clube, +0x46c a data, +0x470..+0x478 as saidas, +0x47c as datas, +0x480 n, +0x484 o kind, +0x488 o estadio): as
#   tres saidas zeradas; 4 se a data esta entre as n escolhidas (indice em byte com sinal); a primeira passada pelos 127
#   torneios (0x69 bytes de +0xb1, em uso pelo +0 e com o clube entre os quatro de +0x15): a janela (inicio - 1, inicio
#   + folga do kind do torneio) estrita (Date::sub_days 0x54c8e0 e add_days 0x54c5b0, a copia por Date::operator=
#   0x417c10 que copia tambem para o retorno), *torneio = vaga e 2 + (+0x65 == 0); a segunda passada com a janela (inicio
#   - folga do kind pedido - 1, inicio + folga + 1): 2; a lista de amistosos (+0x34c8, cursor rebobinado, LLIST::get_next
#   0x68d1b0): o primeiro jogo com mandante e visitante e o clube nele, a data a ate um dia (inclusive); no dia exato,
#   sem o outro livre (0x5c6b70 (data, outro, 0)), LLIST::remove 0x68d1f0 e a resposta da propria rotina sobre
#   [0xae2c74] (nao sobre this) com o ultimo argumento 1; no dia antes ou depois 2; no dia *amistoso = jogo e 5; a lista
#   +0x34cc (no do clube em +0, contagem em byte +8, vagas de 0x19 bytes de +9 com o clube que pede em +0xc): os dias de
#   data - 1 a data + 1, a primeira vaga em uso nesse dia: no proprio dia *pedido = no e 6, senao 2; kind 0xb: com
#   datas, os dias entre a data e o bloco escolhido (da data ao primeiro - 1, ou do ultimo + 1 a data + 1, estrito)
#   todos livres (0x5c6b70 (dia, clube, 0)) ou 2, uma escolhida no dia antes ou depois 2 (no dia 4), mais de 3 dias antes
#   da primeira ou depois da ultima 2; sem datas, livre (kind 0) ou 2; os outros kinds: livre (o kind) ou 2 e, com o
#   ultimo argumento e kind 0, 1 ou 2, o estadio (+0x69) ocupado (FIXTURE_MANAGER::is_stadium_in_use 0x5ae870 sobre
#   [0xae2a58], (estadio, dia, ano, -1)) 2; por fim 2 se a data esta a menos de uma semana de hoje (hoje depois de data -
#   7), senao 1. Prologo push -1; push 0x95a22f (7 bytes).
# Nota ao FORA.md do sbG: o fim e o contrario do que ele diz: 1 a partir de uma semana, 2 antes.
# Comparador: a resposta da copia primeiro (so move os cursores das listas, que a original rebobina); quando a copia
# tiraria um amistoso e perguntaria de novo, so a original; senao o valor e as tres saidas lado a lado. As folgas de um
# kind de torneio invalido mostram a caixa da linha 0xf18 (na comparacao, duas vezes).
# Iguais no cm0102-1080.exe (inteira).
SAVE = False
ROTINAS = [('friendly_get_club_date_status', 0x5c75a0, [0x6a, 0xff, 0x68, 0x2f, 0xa2, 0x95, 0x00])]
DOCS = ('o que uma data guarda para um clube nos amistosos', 'what a date holds for a club in the friendlies',
        [('friendly_get_club_date_status', 'Friendly::get_club_date_status', '0x5c75a0', 'friendly.cpp', 'thiscall, ret 0x24',
          'uma data serve para um amistoso do clube só se não for uma das datas já escolhidas (4), não cair num torneio que ele joga (2, ou 3 enquanto o torneio não foi montado) nem perto dele, não estiver a um dia de um amistoso dele (5 no próprio dia) ou de um pedido agendado (6 no próprio dia), o clube não tiver jogo nela, o estádio estiver livre (tipos 0 a 2, quando pedido) e a data estiver a pelo menos uma semana; um pedido de datas (tipo 11) exige também os dias até o bloco escolhido livres e a data perto dele; o amistoso cujo adversário não está mais livre no dia é cancelado e a pergunta refeita',
          'a date is open to a club for a friendly only if it is not one of the dates already chosen (4), not within a tournament it plays (2, or 3 while the tournament is not set up yet) nor close to one, not within a day of a friendly it plays (5 on the day itself) or of a scheduled request (6 on the day itself), the club has no match on it, its stadium is free (kinds 0 to 2, when asked) and the date is at least a week ahead; a request for dates (kind 11) also needs the days up to the chosen block free and the date near it; a friendly whose opponent is no longer free on its day is cancelled and the question asked again')])
CAMPOS = []
INTERESSA = []
