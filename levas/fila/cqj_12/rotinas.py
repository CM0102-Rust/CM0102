# As regras das eliminatórias da Copa do Mundo: Europa, América do Sul e Ásia (o mesmo slot, a virtual +0x8c
# set_up_rules; wc_europe_league.cpp, wc_south_american_league.cpp, wc_asia_league.cpp). Frente das competições
# (Agente D, 03/11/2026), carreira de 8 ligas.
# - 0x9191e0 (WcEuropeLeague::set_up_rules, 214 bytes; 1 chamada até 2002-06-01 com as 8 ligas): os grupos europeus.
#   Rotina de duas mutações.
# - 0x921040 (WcSouthAmericanLeague::set_up_rules, 281 bytes; 1 chamada): a liga única sul-americana, com as vagas
#   diretas descontando as nações isentas. Uma mutação.
# - 0x912eb0 (WcAsiaLeague::set_up_rules, 201 bytes; 1 chamada): os grupos asiáticos.
# As três têm quadro SEH só pela Date local (sem destrutor): a cópia não monta o quadro. Ajudantes comuns
# (cqj12_campos, cqj12_data, cqj12_fim) com a tabela de campos de cada uma; diferenças byte a byte conferidas: a
# Europa grava +0xc7 = 4, +0xc8 = 5, +0x49 = 7 e +0x4a = 3; a da América do Sul copia +0xef de [0x9cf76c] + 0xee, grava
# +0xbe = 4 − +0xef − +0xee, +0xbf = 1, +0xa7 = −1, +0xab = +0xdb = 0 e a data no mês 0; a da Ásia não grava banco nem
# substituições e põe +0xb8 = 1.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x8c (mov ecx, liga; sem push -> thiscall, ret 0;
# devolvem 1). Dentro: 0x549f70 (Date::Date: push semana; push ano; push mês; push dia; lea ecx, local -> thiscall, ret
# 0x10; o ano vem de +0x40 num registrador com lixo nos 16 bits altos: o construtor lê a palavra, a cópia passa limpo) e
# a virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10).
# Comparadores: só a original (a virtual +0x3c aloca a tabela das rodadas), com o plano: os campos, a data em +0xb6
# (calculada antes pela 0x549f70 numa Date local), +0xb8 e a devolução 1; na da América do Sul também +0xef e as vagas.
# Determinismo: a Date local começa zerada na cópia; o construtor grava a palavra lida.
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: as três ESCAPAM (e escaparam também a vitória valendo 2 e a categoria 9 na Europa e a vitória na
#   América do Sul): nesta carreira as regras e a data da liga-mãe das eliminatórias não chegam a nenhum jogo da
#   janela (os grupos são estágios próprios, e as eliminatórias da Copa de 2002 já vêm montadas/encerradas)
#   [inferência]. O item não passa nas mutações sozinho: só como passageiro de outro item com efeito.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqj12_eliminatoria_europa_regras', 0x9191e0, [0x6a, 0xff, 0x68, 0x98, 0x59, 0x96, 0x00]),
           ('cqj12_eliminatoria_sul_americana_regras', 0x921040, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqj12_eliminatoria_asia_regras', 0x912eb0, [0x6a, 0xff, 0x68, 0x48, 0x57, 0x96, 0x00])]
DOCS = ('as regras das eliminatórias da Copa do Mundo na Europa, na América do Sul e na Ásia',
        'the World Cup qualifier rules in Europe, South America and Asia',
        [('cqj12_eliminatoria_europa_regras', 'WcEuropeLeague::set_up_rules', '0x9191e0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: os grupos europeus são de ida e volta, 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2, 4 e 5 [inferência: saldo, gols pró, confronto direto, gols fora]; o primeiro de cada grupo vai direto; seleções (+0x43 = 2); categoria 10 (seleções) em cada jogo; 7 reservas e 3 substituições; começa em 25 do mês 2 [inferência: 25 de março] levado a um dia da semana 5',
          'rule: the European groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2, 4 and 5 [inference: goal difference, goals for, head-to-head, away goals]; each group winner goes straight through; national teams (+0x43 = 2); category 10 (national teams) on each match; 7 substitutes and 3 changes; starts on 25 of month 2 [inference: 25 March] moved to weekday 5'),
         ('cqj12_eliminatoria_sul_americana_regras', 'WcSouthAmericanLeague::set_up_rules', '0x921040', 'wc_south_american_league.cpp', 'thiscall, ret 0',
          'regra: a eliminatória sul-americana é uma liga única de ida e volta, 3/1 pontos, desempate 1, 2 e 3; as vagas diretas são quatro menos as nações isentas (a campeã do mundo e as anfitriãs sul-americanas, contadas em +0xee e +0xef [inferência]); o quinto vai à repescagem (+0xbf = 1); categoria 10, 7 reservas e 3 substituições; começa em 25 do mês 0 num dia da semana 5',
          'rule: the South American qualifier is a single home and away league, 3/1 points, tiebreak 1, 2 and 3; the direct places are four minus the exempt nations (the world champion and South American hosts, counted in +0xee and +0xef [inference]); the fifth goes to the play-off (+0xbf = 1); category 10, 7 substitutes and 3 changes; starts on 25 of month 0 on weekday 5'),
         ('cqj12_eliminatoria_asia_regras', 'WcAsiaLeague::set_up_rules', '0x912eb0', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: os grupos asiáticos são de ida e volta, 3/1 pontos, desempate 1, 2 e 3, o primeiro de cada grupo segue, categoria 10, começo em 25 do mês 2 num dia da semana 5, +0xb8 = 1 [inferência: a fase começa no ano seguinte]',
          'rule: the Asian groups are home and away, 3/1 points, tiebreak 1, 2 and 3, each group winner goes on, category 10, starts on 25 of month 2 on weekday 5, +0xb8 = 1 [inference: the stage starts the following year]')])
CAMPOS = [('liga da eliminatória +0xc2/+0xc3 pontos, +0xc4 categoria, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas, +0x3c turnos, +0x43, +0x49/+0x4a banco e substituições, +0xa7, +0xab, +0xdb, +0xb6 a data de começo, +0xb8, +0xba calendário; valor/ponteiro; grava (as três)',
           'qualifier league +0xc2/+0xc3 points, +0xc4 category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places, +0x3c rounds, +0x43, +0x49/+0x4a bench and changes, +0xa7, +0xab, +0xdb, +0xb6 start date, +0xb8, +0xba schedule; value/pointer; writes (all three)'),
          ('eliminatória sul-americana +0xee/+0xef nações isentas [inferência]; [0x9cf76c] a competição de onde vem o +0xef; valor/id; lê e grava +0xef (cqj12_eliminatoria_sul_americana_regras)',
           'South American qualifier +0xee/+0xef exempt nations [inference]; [0x9cf76c] the competition +0xef comes from; value/id; reads and writes +0xef (cqj12_eliminatoria_sul_americana_regras)')]
INTERESSA = ['CMRTE']
