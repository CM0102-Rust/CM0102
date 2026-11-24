# A tabela de rodadas e os inscritos do FA Trophy (eng_fa_trophy.cpp; leitor sbF, rodada 5, 24/11/2026). Carreira ENG
# com a base NORMAL: o FA Trophy so e criado quando a divisao extra (a Conference) e carregada (create_english_comps,
# a entrada com true no setup.rs), e o simular.py fixa MINIMUM (como os itens de Serie C2 do cx11: BASE='NORMAL', que
# nenhuma ferramenta le ainda). Com a base padrao as duas rotinas nao rodam.
# - 0x570ec0 EngFaTrophy::create_round_table thiscall(fase, &rodadas, &extra, &saida) ret 0x10: so a fase -1; o molde
#   eng_cc_cup_create_single_stage_round_table (5 rodadas, caixa 0xa4) e a palavra +0x11 = 3 nas rodadas 0 a 3 (o
#   original a grava entre os campos da rodada; o cql10r nao a tem). Tirada por emulacao (unicorn) nas fases
#   -128..127, ano impar e par: as saidas, o tamanho e cada byte.
# - 0x5712b0 EngFaTrophy::select_teams thiscall() ret 0 -> 1/0/-1: 32 vagas (malloc 0xc0, caixa 0x126 e -1); um pool
#   local de ate 100 (byte com sinal) com os clubes da nacao da competicao (+0x5d do registro) fora das quatro
#   divisoes, da Conference e da amadora; comp_util_sort_and_shuffle_top_clubs(pool, n, 10) (o n vai em cx, com lixo
#   nos 16 bits altos; a rotina le si); os 10 primeiros; a Conference do inscrito 10 em diante ate encher; 1 se
#   encheu.
# Prologos: mov al, [esp+4]; sub esp, 0x200 (10 bytes); sub esp, 0x390 (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: a tabela lado a lado (com a palavra +0x11); os inscritos so a original, com o plano (a Conference).
SAVE = False
BASE = 'NORMAL'
ROTINAS = [('eng_fa_trophy_create_round_table', 0x570ec0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_fa_trophy_select_teams', 0x5712b0, [0x81, 0xec, 0x90, 0x03, 0x00, 0x00])]
DOCS = ('a tabela de rodadas e os inscritos do FA Trophy',
        'the FA Trophy round table and entrants',
        [('eng_fa_trophy_create_round_table', 'EngFaTrophy::create_round_table', '0x570ec0', 'eng_fa_trophy.cpp', 'thiscall, ret 0x10',
          'regra: o FA Trophy (clubes de fora da liga) tem cinco rodadas: 32 clubes na 3ª rodada no fim de novembro (com replay), a 4ª em janeiro, a 5ª em fevereiro, as semifinais em ida e volta em abril e a final em 15 de maio em campo neutro [inferência: Wembley]; categoria 4 até a 5ª rodada e 6 depois; sem prêmio por rodada',
          'rule: the FA Trophy (non-league clubs) has five rounds: 32 clubs in the 3rd round in late November (with replays), the 4th in January, the 5th in February, two-legged semi-finals in April and the final on 15 May at a neutral ground [inference: Wembley]; category 4 up to the 5th round and 6 after; no prize per round'),
         ('eng_fa_trophy_select_teams', 'EngFaTrophy::select_teams', '0x5712b0', 'eng_fa_trophy.cpp', 'thiscall, ret 0',
          'regra: o FA Trophy tem 32 clubes: os dez melhores das divisões de fora da liga abaixo da Conference (os melhores embaralhados entre si) e os clubes da Conference; falha (devolve 0) quando não enchem as 32 vagas',
          'rule: the FA Trophy takes 32 clubs: the ten best clubs of the non-league divisions below the Conference (the best shuffled among them) and the Conference clubs; it fails (returns 0) when they do not fill the 32 places')])
CAMPOS = []
INTERESSA = []
