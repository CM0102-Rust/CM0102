# A Copa Intertoto e a Copa da UEFA: os três vencedores do Intertoto na Copa da UEFA e o destino de cada clube nas duas
# (intertoto_cup.cpp, uefa_cup.cpp). Frente das competições (Agente D, 03/11/2026), carreira de 8 ligas.
# - 0x633d40 (IntertotoCup::update_for_date, 355 bytes, a virtual +0x14 do Intertoto; 2064 chamadas até 2002-06-01 com
#   as 8 ligas): uma vez, quando o Intertoto acaba, os vencedores das finais vão para as três últimas vagas da Copa da
#   UEFA e para o livro de inscritos dela; depois a atualização diária da copa (0x51f890, Rust).
# - 0x633c10 (IntertotoCup::update_staff_records_for_status, 290 bytes, virtual +0x44; 114 chamadas): os registros de
#   cada clube pelo desfecho e a marca da competição europeia (+0x1db): o vencedor passa à Copa da UEFA, quem cai sai.
#   É a rotina das mutações (duas).
# - 0x9030e0 (UefaCup::update_staff_records_for_status, 62 bytes, virtual +0x44; 288 chamadas): quem termina na UEFA
#   (campeão, vice ou eliminado) sai da competição europeia; depois os registros da base (0x519890, Rust). Uma mutação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   As próprias (virtuais; quem chama: 0x51f890 e as fases, pela vtable): 0x633d40 thiscall ret 8 (push fase; push
#   hoje; mov ecx, copa); 0x633c10 e 0x9030e0 thiscall ret 0x18 (6 pushes; mov ecx, copa).
#   Na 0x633d40: virtual +0x10 (push 1; push 0; mov ecx, this -> thiscall, ret 8), virtual +0x5c da UEFA (mov ecx,
#   uefa; sem push -> thiscall, ret 0), 0x51f890 (push fase; push hoje; mov ecx, this -> thiscall, ret 8), a caixa
#   (aviso_em: 0x944cff, 0x90d130, 0x944c9f, 0x5e8290, cdecl).
#   Na 0x633c10: 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), 0x86bdd0 (5 pushes; mov ecx, [0xdd7f00] -> thiscall,
#   ret 0x14), 0x86c000 (5 pushes; mov ecx, [0xdd7f00] -> thiscall, ret 0x14).
#   Na 0x9030e0: 0x519890 (6 pushes; ecx = this sem mudar -> thiscall, ret 0x18).
#   Palavras empurradas em registradores com lixo nos 16 bits altos (a rodada do jogo para a 0x4bf850, +0x30 para a
#   0x86c000): os chamados só leem a palavra; a cópia passa o valor limpo (como a cpz_4).
# Comparadores: as três só a original (a atualização diária joga e sorteia; os registros vão ao histórico e mexem na
# reputação e no moral), com o plano feito antes: o livro da UEFA ganha os vencedores na 0x633d40; o +0x1db do clube
# nas outras duas.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01, impressão contra a base): as três ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqi10_intertoto_dia', 0x633d40, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqi10_intertoto_registros', 0x633c10, [0x8a, 0x44, 0x24, 0x08, 0x53]),
           ('cqi10_uefa_registros', 0x9030e0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04])]
DOCS = ('a Copa Intertoto e a Copa da UEFA: os vencedores do Intertoto na UEFA e o destino de cada clube',
        'the Intertoto Cup and the UEFA Cup: the Intertoto winners into the UEFA Cup and each club\'s fate',
        [('cqi10_intertoto_dia', 'IntertotoCup::update_for_date', '0x633d40', 'intertoto_cup.cpp', 'thiscall, ret 8',
          'regra: os vencedores das finais do Intertoto (até três, em agosto) entram na Copa da UEFA, nas três últimas vagas da tabela dela (do tamanho dela menos 27 em diante) e no livro de inscritos com a situação 4 [inferência: entrada pelo Intertoto]; depois as posições previstas da UEFA. Uma vez por temporada, quando o Intertoto acaba; sem a Copa da UEFA, a caixa de erro. Todo dia, a atualização da copa',
          'rule: the winners of the Intertoto finals (up to three, in August) enter the UEFA Cup, in the last three places of its table (from its size minus 27 on) and in its entry book with status 4 [inference: entry through the Intertoto]; then the UEFA predicted positions. Once a season, when the Intertoto ends; without the UEFA Cup, the error box. Every day, the cup update'),
         ('cqi10_intertoto_registros', 'IntertotoCup::update_staff_records_for_status', '0x633c10', 'intertoto_cup.cpp', 'thiscall, ret 0x18',
          'regra: o vencedor de uma final do Intertoto ganha o registro de classificação para a Copa da UEFA (rodada 0xa, peso de 60 a 100 pela rodada) e passa a disputar a UEFA; quem passa de rodada ganha o registro da rodada seguinte (peso rodada × 99 / (rodadas − 1) + 1); quem cai (ou perde a final) ganha o registro de eliminação (peso de 20 a 80) e sai da competição europeia',
          'rule: the winner of an Intertoto final gets the qualification record for the UEFA Cup (round 0xa, weight 60 to 100 by round) and moves to the UEFA Cup; a club going through gets the next round record (weight round × 99 / (rounds − 1) + 1); a club knocked out (or losing the final) gets the knocked-out record (weight 20 to 80) and leaves European competition'),
         ('cqi10_uefa_registros', 'UefaCup::update_staff_records_for_status', '0x9030e0', 'uefa_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da UEFA, o campeão, o vice e cada eliminado saem da competição europeia (a marca do clube volta a −1); os registros de carreira são os da copa base (campeão, vice, classificação e eliminação pela rodada)',
          'rule: in the UEFA Cup, the winner, the runner-up and every knocked-out club leave European competition (the club mark goes back to −1); the career records are the base cup ones (winner, runner-up, qualification and knock-out by round)')])
CAMPOS = [('clube +0x1db: a competição europeia da temporada (id; −1 nenhuma); valor; grava (cqi10_intertoto_registros, cqi10_uefa_registros)',
           'club +0x1db: the European competition of the season (id; −1 none); value; writes (cqi10_intertoto_registros, cqi10_uefa_registros)'),
          ('copa +0xa7 times (6 bytes: +0 clube, +4, +5 desfecho: 1 venceu), +0x3e times, +0x14 livro de inscritos (6 bytes: +0 clube, +4 situação), +0x36 inscritos no livro, +0xa3 rodadas (0x68 bytes, +7 código), +0x3c rodadas; ponteiro/valor; lê e grava (cqi10_intertoto_dia)',
           'cup +0xa7 teams (6 bytes: +0 club, +4, +5 outcome: 1 won), +0x3e teams, +0x14 entry book (6 bytes: +0 club, +4 status), +0x36 book entries, +0xa3 rounds (0x68 bytes, +7 code), +0x3c rounds; pointer/value; reads and writes (cqi10_intertoto_dia)'),
          ('Intertoto +0xb2: os vencedores já passaram à UEFA; valor; lê e grava 1 (cqi10_intertoto_dia)',
           'Intertoto +0xb2: winners already moved to the UEFA Cup; value; reads and writes 1 (cqi10_intertoto_dia)'),
          ('jogo da copa +0x30 rodada (código), +0x32 fase, +0x34 índice da rodada; valor; lê (cqi10_intertoto_registros)',
           'cup tie +0x30 round (code), +0x32 stage, +0x34 round index; value; reads (cqi10_intertoto_registros)'),
          ('[0x9cf6f0] id da Copa da UEFA; [0xadadfc] objetos das competições; [0xae23d0] competições do banco (0x6b bytes); [0xdd7f00] registros de carreira; id/ponteiro; lê',
           '[0x9cf6f0] UEFA Cup id; [0xadadfc] competition objects; [0xae23d0] database competitions (0x6b bytes); [0xdd7f00] career records; id/pointer; reads')]
INTERESSA = ['CMRTE']
