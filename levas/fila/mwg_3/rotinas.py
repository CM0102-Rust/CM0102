# A criação de um jogador virtual (VIRTUAL_STAFF::create_virtual_player 0x90e0a0, virtual_staff.cpp, 1.728 bytes;
# leitor mwg, rodada 2 do mundo inteiro, 12/11/2026). Chamada pela completação dos elencos (top_up_club_squad, mwg_2)
# em toda partida: 5,1 milhões de chamadas até 2002-08-01 no mundo inteiro (cobertura_mundo.txt). Roda também na
# carreira só com a Inglaterra (jovens virtuais dos clubes ingleses nos grupos abaixo do mínimo; elencos inteiros dos
# estrangeiros nas copas europeias e amistosos). Por isso LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x90e0a0..0x90e73d; a tabela de saltos dos grupos em 0x90e740: 0 goleiro,
# 1 esquerda, 2 direita (+0x17), 3 centro (+0x19), 4..6 o mesmo no meio, 7 atacante):
# - a rotina: thiscall(this, clube, grupo (byte), staff, jogador, jovem, reserva, número), ret 0x1c, quadro SEH sem
#   destrutor (as datas não têm);
# - sorteios: 0x946a04 rand × n / 0x8000 em 64 bits (_allmul 0x945e40, _alldiv 0x945e80) = scaled_crt_rand;
# - cdecl: 0x539c60/0x53a1b0 (nação) nomes, 0x53ab90 idade (pessoa, &data) -> al (movsx para a palavra +0x18),
#   0x53b900 database_is_national_team;
# - thiscall: 0x549ef0 Date::Date(dia, ano) -> eax (o this), 0x54a110 Date::Date() -> eax, 0x417c10
#   Date::operator=(push origem; push temporário; ecx = destino), 0x85add0 ([0xdd7ef0]; clube, 0);
# - x87: fild da reputação (movsx), fmul 0.016 (0x970a80) / 0.052 (0x970a78), fadd 20 (0x968ea8) / fsub 150
#   (0x968f50), _ftol (0x945640).
# Fidelidade: o ano-base é o ano atual (palavra sem sinal) arredondado para baixo ao múltiplo de 3; o jovem nasce em
# base − 17, os outros em base − 17 − sorteio(14); entrada no clube em base − 1 e contrato até base + 6; o molde
# vem dos primeiros min(contagem de jogadores, 30000) registros, até 1000 sorteios (fica o último); a ordem dos
# sorteios (nomes sem nação, nascimento, entrada, contrato, adaptabilidade, moldes, jovem, reserva) é a da original.
# Comparador: lado a lado (sorteio e semente da CRT, o registro de staff, o de jogador e as 50 vagas do elenco).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('create_virtual_player', 0x90e0a0, [0x6a, 0xff, 0x68, 0xa0, 0x55, 0x96, 0x00])]
DOCS = ('a criação de um jogador virtual',
        'creating one virtual player',
        [('create_virtual_player', 'VIRTUAL_STAFF::create_virtual_player', '0x90e0a0', 'virtual_staff.cpp', 'thiscall, ret 0x1c',
          'cria um jogador virtual do clube nos registros de staff e de jogador dados e o põe no elenco: nome e sobrenome sorteados para a nação do clube (ou das tabelas inteiras sem nação) e o primeiro apelido; nascido num dia sorteado do ano atual arredondado ao múltiplo de 3 menos 17 (jovem) ou menos 17 a 30; a idade, a nação do clube, a segunda nação basca nos clubes bascos (Athletic e Real Sociedad com os times B, Alavés); numa seleção o cargo 11 na seleção, senão o cargo 11 no clube; entrada no clube no ano anterior e contrato por seis anos (dias sorteados); salário = reputação do clube / 5, sem valor, adaptabilidade 1 + sorteio(10), as outras notas de personalidade 10, classificação 2, marca de elenco 1 (reserva 2; seleção 4 e 8); a ficha de jogador é a cópia de um molde sorteado entre os primeiros 30000 com as posições do grupo (goleiro com nota exatamente 20; os outros 15 ou mais; até 1000 sorteios); capacidade atual e potencial pela reputação do clube: reputação / 25 − 100 (acima de 5000, × 0,016 + 20); numa seleção × 0,052 − 150 (acima de 5000, / 100 + 40); jovem e reserva perdem cada um 30 + sorteio(41); sem reputações, o número de camisa se o clube usa números; entra na primeira vaga livre das 50 do elenco',
          "creates a virtual player of the club at the given staff and player records and puts him in the squad: first and second names drawn for the club's nation (or from the whole tables without a nation) and the first common name; born on a drawn day of the current year rounded down to a multiple of 3 less 17 (youngster) or less 17 to 30; the age, the club's nation, the Basque second nation at the Basque clubs (Athletic and Real Sociedad with their B sides, Alavés); at a national team job 11 there, otherwise job 11 at the club; joined the club the year before with a six-year contract (drawn days); wage = the club's reputation / 5, no value, adaptability 1 + random(10), the other personality ratings 10, classification 2, squad mark 1 (reserve 2; national team 4 and 8); the player record is a copy of a template drawn from the first 30000 with the group's positions (goalkeeper rated exactly 20; the others 15 or more; up to 1000 draws); current and potential ability from the club's reputation: reputation / 25 − 100 (above 5000, × 0.016 + 20); at a national team × 0.052 − 150 (above 5000, / 100 + 40); a youngster and a reserve each lose 30 + random(41); no reputations, the squad number when the club uses numbers; he takes the first free of the 50 squad slots")])
INTERESSA = ['Tweaks', 'CMRTE']
