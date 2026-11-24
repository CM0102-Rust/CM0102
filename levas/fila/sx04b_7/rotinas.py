# A gravacao das notas do mes e os premios de uma nacao no save (month_ratings.cpp, nation_awards.cpp; leitor sx04b,
# rodada 5, 23/11/2026). Rodam na gravacao do jogo (o AWARD_MANAGER grava cada nacao: NationAwards::save 0x76a380, que
# grava cada premio pela virtual +4, a marca das notas do mes e as notas, 0x767d60) e na carga (o construtor de cada
# nacao com arquivo chama NationAwards::load 0x76a280: a virtual init_awards(arquivo), a marca e as notas do mes).
# Convencoes conferidas na desmontagem: as tres thiscall(arquivo), ret 4; zread 0x932c30 / zwrite 0x932e10 cdecl; a
# 0x76a280 com quadro SEH (o novo MONTH_RATINGS, operator new 0x944cf1(0xf), construtor 0x767af0(arquivo) ret 4) e a
# virtual 0 da nacao thiscall(arquivo) (a da Inglaterra e 0x5788c0, init_awards); a marca de 4 bytes lida num local nao
# iniciado; a 0x76a380 toma a marca antes de gravar os premios (virtual +4 de cada premio, thiscall(arquivo)), a
# contagem (i16) relida a cada volta. Caixa "Error": month_ratings.cpp 138/147/155/162, nation_awards.cpp 2059/2091.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe (0x76a280 comeca com mov eax, fs:[0], 6 bytes).
# Depende do sx04b_6 (MONTH_RATINGS_*, MONTH_RATINGS_CPP), do sx04_2 (NATION_AWARDS_CPP) e do sx05b_5
# (nation_awards_fields, FN_MONTH_RATINGS_CONSTRUCT): integrar depois deles.
# Comparadores: so a original (leem ou gravam o arquivo uma vez).
SAVE = True
CARGA = True  # a 0x76a280 so roda na carga
ROTINAS = [('month_ratings_save_to_disk', 0x767d60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nation_awards_load', 0x76a280, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('nation_awards_save', 0x76a380, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a gravação das notas do mês e os prêmios de uma nação no save',
        "the month ratings' save and a nation's awards in the saved game",
        [('month_ratings_save_to_disk', 'MONTH_RATINGS::save_to_disk', '0x767d60', 'month_ratings.cpp', 'thiscall, ret 4',
          'regra: grava no save a contagem das notas, as notas (quando há), o mês e o ano; uma gravação curta mostra a caixa "Error" e para',
          'rule: writes the ratings count, the entries (when there are any), the month and the year into the save; a short write shows the "Error" box and stops'),
         ('nation_awards_load', 'NationAwards::load', '0x76a280', 'nation_awards.cpp', 'thiscall, ret 4',
          'regra: a nação monta os seus prêmios lendo-os do save (a virtual init_awards); depois lê a marca das notas do mês e, com ela, cria as notas lendo-as do save; devolve 1, ou 0 depois da caixa "Error" se a marca não puder ser lida',
          "rule: the nation builds its awards reading them from the save (the virtual init_awards); then reads the month ratings flag and, with it, creates the ratings reading them from the save; returns 1, or 0 after the \"Error\" box when the flag cannot be read"),
         ('nation_awards_save', 'NationAwards::save', '0x76a380', 'nation_awards.cpp', 'thiscall, ret 4',
          'regra: grava cada prêmio da nação, a marca de se a nação tem notas do mês e, com elas, as notas; uma gravação curta da marca mostra a caixa "Error" e para',
          "rule: writes each of the nation's awards, the flag of whether the nation has month ratings and, with them, the ratings; a short write of the flag shows the \"Error\" box and stops")])
CAMPOS = []
INTERESSA = []
