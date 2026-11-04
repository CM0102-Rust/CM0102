# A classificação da UEFA (uefa_seeding.cpp), parte 3: as vagas da Copa da UEFA, o voluntário inglês da Intertoto e a
# virada do ano que refaz tudo.
# - 0x904580 (UefaSeeding::assign_uefa_cup_places, 2223 bytes): o campeão, o fair play sorteado, as faixas por posição e
#   a ordem dos clubes de cada nação (campeão da copa, vice, a final guardada, campeão da copa da liga, a tabela). Roda
#   uma vez na carreira até 2002-06-01 (o começo de 2001; docs/cobertura_competicoes.txt) e decide a Copa da UEFA de
#   2001/02.
# - 0x903890 (UefaSeeding::pick_english_intertoto_club, 689 bytes, quadro SEH pela notícia local sem destrutor no caminho
#   normal): passageira; em 2001 (o único ano que roda até 2002-06-01) devolve 0 sem efeito.
# - 0x9035a0 (UefaSeeding::update_season, 522 bytes, quadro SEH pela notícia local sem destrutor no caminho normal):
#   chamada pelas copas europeias todo dia (519 chamadas); a virada do ano refaz a classificação (uma vez, no começo de
#   2001); o dia 100 entrega o pedido de voluntários aos técnicos humanos da Premier.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x904580: 0x90376a (push conta; push tabela; mov ecx, ebx) -> thiscall, ret 8.
# - quem chama a 0x903890: 0x904e92 e 0x904f30 (mov ecx, esi; sem push) -> thiscall, ret 0.
# - quem chama a 0x9035a0: as copas europeias (0x584ae0 e outras: push diário; mov ecx, seeding) -> thiscall(this,
#   diário), ret 4.
# - dentro da 0x904580: 0x4afd80 push lugar; push linha da competição; add esp, 8 -> cdecl; 0x90cfc0 push n; add esp
#   (0xc/8) -> cdecl; 0x669340 push nação; add esp, 4 -> cdecl (devolve char); 0x9050d0 push 0; push 1; push 1; push
#   nação; push conta; push tabela; mov ecx, edi -> thiscall, ret 0x18; a caixa (linha 0x418).
# - dentro da 0x903890: 0x90cfc0 cdecl; 0x600a50 5 push, add esp, 0x18 (com o do sorteio) -> cdecl; 0x944e46 push 0x78;
#   add esp, 4 -> cdecl; a caixa (linha 0xf9); 0x77bbe0 push 0; push 0x17; lea ecx, notícia -> thiscall, ret 8;
#   0x785790 push id; push 0; lea ecx, notícia -> thiscall, ret 8; 0x7862d0 push divisão; push &notícia; mov ecx,
#   [0xb7431c] -> thiscall, ret 8.
# - dentro da 0x9035a0: 0x903b50, 0x903cc0, 0x905f00 e 0x9058e0 mov ecx, ebx, sem push -> thiscall, ret 0; 0x904040
#   push &saída; mov ecx, ebx -> thiscall, ret 4; 0x904210/0x904580/0x904e30 push conta; push tabela; mov ecx, ebx ->
#   thiscall, ret 8; 0x9452ca push; add esp, 4 -> cdecl; a caixa (linha 0x2b7); 0x77bbe0 push 0; push 0x16; lea ecx ->
#   thiscall, ret 8; 0x785790 (push valor; push campo; lea ecx, notícia) -> thiscall, ret 8; 0x7875e0 push clube; mov
#   ecx, [0xb7431c] -> thiscall, ret 4 (a cópia em Rust dela não lê o this); 0x7861e0 push clube; push &notícia; mov
#   ecx, [0xb7431c] -> thiscall, ret 8.
# Os comparadores das rotinas que chamam a 0x4afd80 (o campeão/vice pelo histórico) incluem o histórico das
# competições nas regiões (a 0x4afd80 o reordena; ver a cqj_1).
# Comparadores: 0x904580 lado a lado (a classificação com as finais em +0x440, os clubes, o sorteio com a cópia sem
# contar, a marca da caixa); 0x903890 em 2001 a devolução das duas (sem efeito), fora disso só a original (sorteia, aloca,
# notícia); 0x9035a0 só a original (refaz a classificação pelas rotinas de dentro e entrega notícias; a cópia leva o
# contador "cqi_3 virada da classificacao").
# Determinismo: as notícias locais começam zeradas na cópia e o construtor (0x77bbe0) as preenche; o [S+0x1a] da original
# (a última fase com vaga) só é lido depois de gravado na mesma nação ou na anterior (começa 0 na cópia). Nada lê pilha,
# heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('uefa_vagas_copa_da_uefa', 0x904580, [0x81, 0xec, 0x48, 0x02, 0x00, 0x00]),
           ('uefa_voluntario_ingles', 0x903890, [0x6a, 0xff, 0x68, 0x3b, 0x52, 0x96, 0x00]),
           ('uefa_virada_da_classificacao', 0x9035a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a classificação da UEFA: as vagas da Copa da UEFA, o voluntário inglês da Intertoto e a virada do ano que refaz as vagas',
        'the UEFA seeding: the UEFA Cup places, the English Intertoto volunteer and the turn of the year that redoes the places',
        [('uefa_vagas_copa_da_uefa', 'UefaSeeding::assign_uefa_cup_places', '0x904580', 'uefa_seeding.cpp', 'thiscall, ret 8',
          'a regra da Copa da UEFA (fase 1 do jogo = a primeira fase, fase 2 = a preliminar [inferência]): o campeão da última edição entra na primeira fase e a nação dele perde uma vaga nessa fase; três nações sorteadas (o fair play) ganham uma vaga na preliminar; pela posição no coeficiente, vagas na primeira fase e na preliminar: 1ª a 6ª 3 e 0, 7ª e 8ª 4 e 0, 9ª a 15ª 1 e 1, 16ª e 17ª 1 e 2, 18ª a 21ª 0 e 3, 22ª a 48ª 0 e 2, da 49ª em diante 0 e 1. A primeira vaga da nação vai ao campeão da copa nacional (ou ao vice, se o campeão já está na Liga dos Campeões; com os dois já classificados, aos finalistas da copa ainda por jogar guardados na classificação, fora Espanha e Bélgica), a segunda ao campeão da copa da liga, as outras aos primeiros da tabela ainda sem vaga',
          'the UEFA Cup rule (the game\'s round 1 = the first round, round 2 = the qualifying round [inference]): the last winner enters the first round and its nation loses one place in that round; three nations drawn at random (fair play) get one place in the qualifying round; by coefficient position, places in the first round and the qualifying round: 1st to 6th 3 and 0, 7th and 8th 4 and 0, 9th to 15th 1 and 1, 16th and 17th 1 and 2, 18th to 21st 0 and 3, 22nd to 48th 0 and 2, 49th onwards 0 and 1. A nation\'s first place goes to the national cup winner (or the runner-up if the winner is already in the Champions League; with both already qualified, to the finalists of the cup still to be played stored in the seeding, except Spain and Belgium), the second to the league cup winner, the others to the top clubs of the table still without a place'),
         ('uefa_voluntario_ingles', 'UefaSeeding::pick_english_intertoto_club', '0x903890', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'o clube inglês da Intertoto: fora de 2001, até 6 voluntários sorteados entre os clubes da Premier sem técnico humano e sem vaga europeia; vai o voluntário da divisão mais alta (o mais bem colocado [inferência: o +0x5f]); sem voluntário, um clube sorteado sem vaga; com a notícia à divisão. Em 2001 a Inglaterra usa a tabela (a 0x904e30)',
          'England\'s Intertoto club: outside 2001, up to 6 volunteers drawn among Premier clubs without a human manager or a European place; the volunteer from the highest division goes (the best placed [inference: +0x5f]); without volunteers, a random club without a place; with the news to the division. In 2001 England uses the table (0x904e30)'),
         ('uefa_virada_da_classificacao', 'UefaSeeding::update_season', '0x9035a0', 'uefa_seeding.cpp', 'thiscall, ret 4',
          'o calendário da classificação: a cada ano novo do jogo refaz os coeficientes, as vagas por nação e as vagas da Liga dos Campeões, da Copa da UEFA e da Intertoto, e entrega a notícia dos classificados; no dia 100 do ano pede voluntários para a Intertoto aos técnicos humanos da Premier; todo dia, o prazo das inscrições europeias',
          'the seeding calendar: every new game year it redoes the coefficients, the places per nation and the Champions League, UEFA Cup and Intertoto places, and posts the qualifiers news; on day 100 of the year it asks Premier human managers for Intertoto volunteers; every day, the European registration deadline')])
CAMPOS = [('classificação da UEFA +0 tabela das nações (+0 id, +4 a nação, +0x11 clubes nas copas), +0x438 número de nações, +0x439 ano (short), +0x43b lista dos voluntários (ids), +0x43f a conta, +0x440 50 finais de copa guardadas (12 bytes: id da copa, id do campeão, id do vice); id, ponteiro e valor; lê e grava (as três)',
           'UEFA seeding +0 nation table (+0 id, +4 nation, +0x11 clubs in the cups), +0x438 number of nations, +0x439 year (short), +0x43b volunteer list (ids), +0x43f its count, +0x440 50 stored cup finals (12 bytes: cup id, winner id, runner-up id); id, pointer and value; reads and writes (all three)'),
          ('clube +0x1db competição europeia (−1 sem vaga), +0x1df fase de entrada, +0x53 nação, +0x5b divisão, +0x5f [inferência: posição]; id, ponteiro e valor; lê e grava +0x1db/+0x1df (uefa_vagas_copa_da_uefa), lê (uefa_voluntario_ingles)',
           'club +0x1db European competition (−1 none), +0x1df entry round, +0x53 nation, +0x5b division, +0x5f [inference: position]; id, pointer and value; reads and writes +0x1db/+0x1df (uefa_vagas_copa_da_uefa), reads (uefa_voluntario_ingles)'),
          ('nação +0 id, +0x11c tem competições no jogo; 0xb63d7b/+0x7e/+0x82 a linha de 0x48 bytes da nação jogável (o ano, a copa nacional, a copa da liga); competição +0x40 ano; id, ponteiro e valor; lê (uefa_vagas_copa_da_uefa)',
           'nation +0 id, +0x11c has competitions in the game; 0xb63d7b/+0x7e/+0x82 the playable nation\'s 0x48-byte row (the year, the national cup, the league cup); competition +0x40 year; id, pointer and value; reads (uefa_vagas_copa_da_uefa)'),
          ('[0xae2c92] ano do jogo, [0xae2c90] dia do ano; valor; lê (uefa_voluntario_ingles, uefa_virada_da_classificacao)',
           '[0xae2c92] game year, [0xae2c90] day of the year; value; reads (uefa_voluntario_ingles, uefa_virada_da_classificacao)')]
INTERESSA = ['CMRTE']
