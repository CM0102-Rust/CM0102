# Os 58 premios do Brasil (award/brazil_awards.cpp; leitor rx09, rodada 6, 25/11/2026). Rotina grande sozinha (5,7 KB,
# 0x43d340..0x43e990, ret 4; sem tabela de saltos nem desvio: 58 operator new e 58 construtores em linha). Simulacao
# com as 8 ligas: roda na criacao do jogo (o construtor do rx09_13).
# - 0x43d340 BrazilAwards::init_awards thiscall(arquivo) ret 4: +8 = 0x3a, malloc 0xe8 (caixa 0xa3); cada premio por
#   operator new 0x944cf1 (0x25d equipe, 0x2b ano) e o construtor 0x8af520 (10 argumentos, ret 0x28) ou 0x92fe80 (11,
#   ret 0x2c). Os argumentos de todas as 58 chamadas tirados pela emulacao (unicorn, rx09\emul_awards.py, com cada
#   chave valendo o proprio endereco) e conferidos na desmontagem: 16 de equipe e 42 do ano, tipo 0, escopo 0, para
#   jogadores; as equipes do ano com o periodo 0 e as da semana com 1.
# Prologo mov eax, fs:[0] (6 bytes); igual no cm0102-1080.exe.
# Comparador: so a original (aloca os premios); cada premio com a nacao e o indice da vaga (compare_nation_awards_init).
# Depende do sx05b_5 (AwardSpec, nation_awards_build, compare_nation_awards_init) e do rx09_13 (BRAZIL_STATES,
# BRAZIL_AWARDS_CPP). As chaves dos estaduais de 0x9d0000 em diante nao tem nome no keys.rs (ficam pela conta).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('brazil_awards_init_awards', 0x43d340, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os 58 prêmios do Brasil', 'the 58 Brazilian awards',
        [('brazil_awards_init_awards', 'BrazilAwards::init_awards', '0x43d340', 'award/brazil_awards.cpp', 'thiscall, ret 4',
          'regra: a Primeira e a Segunda Divisão têm a equipe da semana, o jogador do ano, o artilheiro, o jovem do ano e a equipe do ano; cada um dos doze estaduais tem a equipe da semana, o melhor jogador, o artilheiro e o melhor goleiro; as reputações dos prêmios caem da Primeira para a Segunda e para os estaduais',
          'rule: the First and Second Divisions have the team of the week, the player of the year, the top goalscorer, the young player of the year and the team of the year; each of the twelve state championships has the team of the week, the best player, the top goalscorer and the best goalkeeper; the awards\' reputations fall from the First Division to the Second and to the state championships')])
CAMPOS = []
INTERESSA = []
