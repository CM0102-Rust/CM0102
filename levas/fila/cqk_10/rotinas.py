# As regras dos torneios de seleções: a Copa América, a Copa Africana de Nações e a eliminatória da Oceania (o mesmo
# slot, a virtual +0x8c set_up_rules; goldcup.cpp, african_nations.cpp, wc_oceania_league.cpp). Frente das
# competições (Agente D, 03/11/2026), carreira de 8 ligas.
# - 0x5e12d0 (GoldCup::set_up_rules, 213 bytes; 2 chamadas até 2002-06-01 com as 8 ligas): os grupos da Copa América
#   (goldcup.cpp; o mapa da comunidade chama a classe de Copa América). Uma mutação.
# - 0x401ab0 (AfricanNations::set_up_rules, 209 bytes; 1 chamada): os grupos da Copa Africana. Duas mutações.
# - 0x91ecf0 (WcOceaniaLeague::set_up_rules, 205 bytes; 1 chamada): os grupos da eliminatória da Oceania.
# As três têm quadro SEH só pela Date local (sem destrutor): a cópia não monta o quadro. Ajudantes comuns
# (cqk10_campos, cqk10_data, cqk10_fim) com a tabela de campos de cada uma; diferenças byte a byte conferidas: a Copa
# América grava +0xc8 = 0, +0xbe = 2 e a data (20, 2, ano); a Africana não grava +0xc8, tem a data (5, 9, ano − 1) e
# +0xb8 = −1; a da Oceania tem +0xbe = 1 e a data (15, 0, ano).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x8c (mov ecx, liga; sem push -> thiscall, ret 0;
# devolvem 1). Dentro: 0x549f70 (Date::Date: push semana; push ano; push mês; push dia; lea ecx, local -> thiscall, ret
# 0x10; o ano num registrador com lixo nos 16 bits altos, lido como palavra: a cópia passa limpo) e a virtual +0x3c (push
# 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10).
# Comparadores: só a original (a virtual +0x3c aloca a tabela das rodadas), com o plano: os campos, a data em +0xb6
# (calculada antes pela 0x549f70 numa Date local), +0xb8 e a devolução 1.
# Determinismo: a Date local começa zerada na cópia; o construtor grava a palavra lida.
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: a da Copa Africana e a da Copa América ACUSADAS; a da Oceania ESCAPA (a data dos grupos da Oceania não
#   chega a jogo da janela). A vitória valendo 2 escapou nas duas (os grupos são estágios próprios).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqk10_copa_america_regras', 0x5e12d0, [0x6a, 0xff, 0x68, 0x68, 0xa8, 0x95, 0x00]),
           ('cqk10_copa_africana_regras', 0x401ab0, [0x6a, 0xff, 0x68, 0xd8, 0x34, 0x95, 0x00]),
           ('cqk10_eliminatoria_oceania_regras', 0x91ecf0, [0x6a, 0xff, 0x68, 0xf8, 0x5a, 0x96, 0x00])]
DOCS = ('as regras dos grupos da Copa América, da Copa Africana de Nações e da eliminatória da Oceania',
        'the group rules of the Copa America, the African Nations Cup and the Oceania qualifiers',
        [('cqk10_copa_america_regras', 'GoldCup::set_up_rules', '0x5e12d0', 'goldcup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa América são de turno único, 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2 e 3 [inferência: saldo, gols pró, vitórias]; dois classificados por grupo; seleções; categoria 10 em cada jogo; 7 reservas e 3 substituições; a data de 20 do mês 2 do ano da competição num dia da semana 5 [inferência: a data do sorteio]',
          'rule: the Copa America groups are single round robin, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3 [inference: goal difference, goals for, wins]; two qualifiers per group; national teams; category 10 on each match; 7 substitutes and 3 changes; the date of 20 of month 2 of the competition year on weekday 5 [inference: the draw date]'),
         ('cqk10_copa_africana_regras', 'AfricanNations::set_up_rules', '0x401ab0', 'african_nations.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa Africana de Nações são de turno único, 3/1 pontos, desempate 1, 2, 3, dois classificados por grupo, categoria 10, 7 reservas e 3 substituições; a data de 5 do mês 9 do ano anterior ao do torneio num dia da semana 5 [inferência: o sorteio no ano anterior] e +0xb8 = −1',
          'rule: the African Nations Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group, category 10, 7 substitutes and 3 changes; the date of 5 of month 9 of the year before the tournament on weekday 5 [inference: the draw in the previous year] and +0xb8 = −1'),
         ('cqk10_eliminatoria_oceania_regras', 'WcOceaniaLeague::set_up_rules', '0x91ecf0', 'wc_oceania_league.cpp', 'thiscall, ret 0',
          'regra: os grupos da eliminatória da Oceania são de turno único, 3/1 pontos, desempate 1, 2, 3, um classificado por grupo (para a final da Oceania), categoria 10, 7 reservas e 3 substituições, começo em 15 do mês 0 num dia da semana 5',
          'rule: the Oceania qualifier groups are single round robin, 3/1 points, tiebreak 1, 2, 3, one qualifier per group (to the Oceania final), category 10, 7 substitutes and 3 changes, starting on 15 of month 0 on weekday 5')])
CAMPOS = [('liga do grupo +0xc2/+0xc3 pontos, +0xc4 categoria, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas, +0x3c turnos, +0x43 (2 seleções), +0x49/+0x4a banco e substituições, +0xb6 a data, +0xb8, +0xba calendário; valor/ponteiro; grava (as três)',
           'group league +0xc2/+0xc3 points, +0xc4 category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places, +0x3c rounds, +0x43 (2 national teams), +0x49/+0x4a bench and changes, +0xb6 date, +0xb8, +0xba schedule; value/pointer; writes (all three)'),
          ('competição +0x40 ano; valor; lê (as três)', 'competition +0x40 year; value; reads (all three)')]
INTERESSA = ['CMRTE']
