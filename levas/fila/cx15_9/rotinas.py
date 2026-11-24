# O calendario da MLS (o slot create_schedule, a virtual +0x3c, de usa_mls.cpp; leitor cx15, rodada 5, 24/11/2026).
# Competicoes com as 8 ligas: 12 chamadas na carreira de 8 ligas ate 2002-06-01 (docs/cobertura_competicoes.txt): a
# temporada regular e as fases das divisoes na criacao e na virada, os play-offs e a final quando a fase chega.
# - 0x9072d0 UsaMls::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10: o molde dos calendarios da
#   cqp_11 (ScheduleStage, create_round_table, asia_club_champ_compare_round_table, ja no motor). Fases (extraidas
#   seguindo o fluxo da original com cada fase de -128 a 127, o fases.py do agente C): -3 liga de 12 rodadas (0x30c
#   bytes, caixa da linha 0xb3), -2 de 22 (0x596, 0xeb), -1 de 32 (0x820, 0x141), 2 play-offs de 2 rodadas (0xd0,
#   0x1b6, extra 0xa0) e 3 a final de 1 rodada (0x68, 0x1f2, extra 0x417); e_liga so com o ponteiro nao nulo; o extra
#   sem conferir; as outras fases devolvem 0 sem gravar.
#   Convencoes conferidas: 0x944e46 (malloc) cdecl; 0x68a160 (a data da rodada) e 0x68a1c0 (o jogo da rodada), 0x521e60
#   (o sorteio) e 0x521eb0 (o jogo da copa): 9, 7, 7 e 9 pushes, cdecl (o ano: a palavra +0x40 nas da liga, com sinal
#   nas de copa); a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290) pelo create_round_table.
#   Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes).
# Igual no cm0102-1080.exe (inteira). Comparador: lado a lado (asia_club_champ_compare_round_table): as saidas e os bytes
# gravados das duas tabelas iguais, a partir do mesmo [0xb67a34]; a tabela da copia e liberada; na fase sem tabela, so
# a original (tem de devolver 0).
# Ja tentada na cqu_4 (06/11/2026): comparada sem divergencia, mas duas mutacoes de data escaparam da carreira (os jogos
# da MLS sao montados a mao). Aqui as mutacoes contam pelo comparador da rotina.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('usa_mls_create_schedule', 0x9072d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da MLS',
        'the MLS schedule',
        [('usa_mls_create_schedule', 'UsaMls::create_schedule', '0x9072d0', 'usa_mls.cpp', 'thiscall, ret 0x10',
          'O calendário da MLS: a liga de 12, 22 ou 32 rodadas (fases −3, −2 e −1; a temporada regular de 22/fev a 6/ago), os play-offs de duas rodadas (fase 2: sorteios em 7 e 21/ago, jogos de 16 e 30/ago) e a final da MLS Cup (fase 3: sorteio em 8/set, jogo em 15/set); cada rodada com a data e os jogos; as outras fases não têm tabela',
          'The MLS schedule: the league of 12, 22 or 32 rounds (stages −3, −2 and −1; the regular season from 22 Feb to 6 Aug), the two-round play-offs (stage 2: draws on 7 and 21 Aug, games from 16 and 30 Aug) and the MLS Cup final (stage 3: draw on 8 Sep, game on 15 Sep); each round with its date and games; the other stages have no table')])
CAMPOS = []
INTERESSA = []
