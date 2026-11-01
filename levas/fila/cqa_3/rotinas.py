# O sorteio da rodada da copa e dois auxiliares (Cup.cpp e comp_util.cpp, fora da fila): Cup::draw_round (0x51c410,
# 1.106 chamadas na carreira até 2002-06-01, pela atualização diária da copa 0x51f890 e pela montagem da rodada 0x51c800,
# as duas em Rust, e pelas copas de outras nações), comp_util_format_stage_round_name (0x4bffc0, 218, o nome da fase e
# da rodada nos textos das competições; passageira: só texto) e Cup::club_in_higher_division (0x522360, a virtual +0x90
# da copa base, consultada pelos mandantes da rodada 0x51e870 com a regra 1; passageira: não roda na carreira de
# 17/10). O comp_util.cpp que resta é só texto (a 0x4b4850 tem 19,7 KB; a 0x4b4420 entrega o texto à notícia pela
# 0x4ae660, que aloca): a rotina com efeito do item é a 0x51c410. Quadro SEH na 0x51c410 só para as Datas e a notícia
# no desenrolar (sem destrutor no caminho normal). Iguais no cm0102-1080.exe (corpo inteiro).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x51d2d0, 0x51d9f0, 0x51e220, 0x522610 (os sorteios e o estádio compartilhado): push rodada; ecx = a copa; sem add
#   esp -> thiscall(copa, rodada), ret 4.
# - 0x51e870 (mandantes): push −1; push rodada; ecx = a copa; sem add esp -> thiscall, ret 8.
# - 0x549ef0 (Data::Data): push ano; push dia; ecx = a local; sem add esp -> thiscall, ret 8 (dia e ano em
#   registradores com lixo nos 16 bits altos; a Rust só lê as palavras).
# - 0x54c5b0 (mais): push 1; push &b; ecx = &a; sem add esp -> thiscall, ret 8, devolve &b.
# - 0x417c10 (copia): push &b (o devolvido); push &c; ecx = &a; sem add esp -> thiscall, ret 8.
# - 0x76d940 (competição especial das seleções): push comp; ecx = [0xb74318]; sem add esp -> ret 4; a cópia Rust é
#   stdcall(comp) (o this não é lido), chamada assim.
# - 0x5c6710 (competição amistosa): push comp; ecx = [0xae2c74]; sem add esp -> thiscall, ret 4.
# - 0x77bbe0 / 0x785790 / 0x7861e0 (a notícia): thiscall, ret 8 as três (push 0; push 0x27 / push valor; push k / push
#   clube; push &local com ecx = [0xb7431c]).
# - 0x7875e0 (clube de técnico humano): push clube; ecx = [0xb7431c] -> ret 4; a cópia Rust é stdcall(clube).
# - 0x4b4850 (nome da fase): push estilo; push id; push tamanho; push saída; add esp (0x30 junto) -> cdecl de 4.
# - 0x4ba8f0 (gênero do nome): push estilo; push id; add esp -> cdecl de 2.
# - 0x66f4e0 (texto traduzido): 8 pushes; add esp, 0x20 -> cdecl (saída, modelo, ga, ga, gb, gb, &a, &b).
# Comparadores: a 0x51c410 só a original (sorteia e aloca a notícia), com o plano feito antes (as datas adiadas; ou as
# marcas +0x34 de todos os confrontos e a soma em +0xac); a 0x4bffc0 lado a lado na saída e na devolução; a 0x522360
# pelo valor.
SAVE = False
ROTINAS = [('copa_sortear_rodada', 0x51c410, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('nome_da_fase_e_rodada', 0x4bffc0, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0xc8, 0x02, 0x00, 0x00]),
           ('clube_da_divisao_mais_alta', 0x522360, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x57])]
DOCS = ('o sorteio da rodada da copa, o nome da fase e da rodada e o clube da divisão mais alta',
        'the cup round draw, the stage and round name and the club in the higher division',
        [('copa_sortear_rodada', 'Cup::draw_round', '0x51c410', 'Cup.cpp', 'thiscall, ret 4',
          'com algum time em aberto adia o sorteio 1 dia (e as rodadas seguintes para no mínimo essa data); senão sorteia pela regra (semente 0x51d2d0, nações separadas 0x51d9f0 ou livre 0x51e220), acerta os mandantes (0x51e870) e o estádio compartilhado (0x522610), marca os confrontos sorteados e manda a notícia do sorteio aos times de técnico humano; devolve 1',
          'with some team still open it puts the draw off by 1 day (and the following rounds to at least that date); otherwise draws by the rule (seeded 0x51d2d0, nations apart 0x51d9f0 or open 0x51e220), sets the home teams (0x51e870) and the shared stadium (0x522610), marks the drawn ties and sends the draw news to the human managed teams; returns 1'),
         ('nome_da_fase_e_rodada', 'comp_util_format_stage_round_name', '0x4bffc0', 'comp_util.cpp', 'cdecl',
          'escreve o nome da fase e o da rodada (com os gêneros, pelo modelo 0x9a7cfc), ou só o de uma delas; sem saída ou com menos de 200 bytes, a caixa e 0',
          'writes the stage and round names (with their genders, through template 0x9a7cfc), or just one of them; without an output or under 200 bytes, the error box and 0'),
         ('clube_da_divisao_mais_alta', 'Cup::club_in_higher_division', '0x522360', 'Cup.cpp', 'thiscall, ret 8',
          'devolve o clube de nível de divisão menor (+0x57 → +0x69), ou 0 no empate',
          'returns the club whose division level (+0x57 -> +0x69) is lower, or 0 when level')])
CAMPOS = [('Cup +0x45 / +0x3c / +0x40 / +0x3a / +0x44 / +0xab / +0xac: a marca de pronta, as rodadas, o ano base, o id, a competição, a rodada atual e a conta das notícias de sorteio; valor; lê, grava +0xac (copa_sortear_rodada)',
           'Cup +0x45 / +0x3c / +0x40 / +0x3a / +0x44 / +0xab / +0xac: ready flag, rounds, base year, id, competition, current round and draw news count; value; reads, writes +0xac (copa_sortear_rodada)'),
          ('rodada da copa (0x68 bytes) +0 / +2 / +7 / +0xb / +0xd / +0x1a / +0x1c / +0x21: a data do sorteio (dia e ano), o tipo, as regras, os confrontos, as entradas e o estádio; valor; lê, grava a data (copa_sortear_rodada)',
           'cup round (0x68 bytes) +0 / +2 / +7 / +0xb / +0xd / +0x1a / +0x1c / +0x21: draw date (day and year), type, rules, ties, entries and stadium; value; reads, writes the date (copa_sortear_rodada)'),
          ('confronto da copa (0x35 bytes) +0 / +2 / +0x34: os times e a marca de sorteado; valor; lê, grava +0x34 (copa_sortear_rodada)',
           'cup tie (0x35 bytes) +0 / +2 / +0x34: teams and drawn flag; value; reads, writes +0x34 (copa_sortear_rodada)'),
          ('clube +0x57 → divisão +0x69: o nível da divisão; ponteiro e valor; lê (clube_da_divisao_mais_alta)',
           'club +0x57 -> division +0x69: the division level; pointer and value; reads (clube_da_divisao_mais_alta)')]
INTERESSA = []
