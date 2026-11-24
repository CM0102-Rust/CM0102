# Os registros pela situacao, o texto da noticia do destino e o progresso dos clubes da J1 League japonesa (jap_j1.cpp;
# leitor cx12, rodada 5, 24/11/2026). Simulacao com as 8 ligas ate 2002-06-01: a J1 de 2001 tem a segunda fase de
# agosto a novembro e a final do campeonato em dezembro de 2001 (os registros das fases 1 e 2); a primeira fase de
# 2002 so acaba depois de junho. O texto so com o modo CM_TEXTO_NOTICIAS e um tecnico humano como destinatario.
# - 0x663db0 JapJ1::update_staff_records_for_status thiscall(clube, situacao, fase, &marca, a5, a6) ret 0x18 (6 args:
#   mov al, [esp+0xc] = a fase; [esp+0x20] depois de 4 push = o ponteiro da marca, um byte que recebe 1): fase 0,
#   situacao 6: registro de fase vencida (Staff_Records::add_qualified_for_round_record 0x86bdd0 com 0, 0x96, 0x28), a
#   marca e a linha do clube na tabela com situacao 6; 0. Fase 1 (qualquer situacao): se o clube tambem foi o primeiro
#   da primeira fase ([[+0xc]].tabela[0]), campeao sem final: a linha e as linhas nas tabelas das duas fases com 5,
#   add_league_champions_record 0x868c50, +0x2c (STAGE_COUNT) = 2 e a marca; 1. Senao, registro de fase vencida, a
#   marca e a linha com a situacao passada; 2. Fase 2: situacao 1, campeao (as linhas com 5, o registro, a marca); 0;
#   situacao 2, add_knocked_out_record 0x86c000 (0, 0x96, 0x50); 0. Resto 0. Prologo mov al, [esp+0xc]; push ebx (5).
# - 0x6640a0 JapJ1::build_fate_news_text thiscall(clube, destino, fase, a4, jogo, a6, resultado, a8, noticia, texto)
#   ret 0x28 (o a7 e o retorno do update_staff_records_for_status): fase 0xff, o texto base Comp::build_fate_news_text
#   0x48c6d0 com -1 e a7 = 0; fase 0 destino 6 e fase 1 destino 6 (resultado 1 ou 2), os textos proprios (com a noticia o
#   longo, com a liga; sem ela o curto); fase 2 com a noticia o texto base (destino 1 literal quando e 1, fase 2, a7 = 0),
#   sem ela "win {liga}" (destino 1) ou "lose {liga} final". langlib_sprintf 0x66f4e0 em 0xde1f64, Text::set_string
#   0x4ae660 e os dois vinculos Text::add_link 0x4ae8a0 (clube 0x7d5, competicao 0x7d0) sempre. Prologo como o anterior.
# - 0x664400 JapJ1::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (sem teste), a situacao da linha do clube; na fase 0xff, situacao 1 nao grava;
#   senao melhor 2..5 -> 2 e pior 2..5 -> 5; grava +0x73, +0x74, +0x75. DEFEITO: a situacao fica no byte baixo do
#   argumento clube na pilha; sem linha, vale o byte baixo do ponteiro do clube. Prologo 4 push; mov edi, [esp+0x14] (8).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os registros so a original contra o plano feito antes (retorno,
# situacao da linha, STAGE_COUNT e a marca); o texto lado a lado (compare_text_writer) nos caminhos so de texto; o
# progresso lado a lado nos 3 bytes (compare_c1_club_progress).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('jap_j1_update_staff_records_for_status', 0x663db0, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('jap_j1_build_fate_news_text', 0x6640a0, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('jap_j1_set_club_progress', 0x664400, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14])]
DOCS = ('os registros pela situação, o texto da notícia do destino e o progresso dos clubes da J1 League japonesa',
        'the J1 League staff records by outcome, fate news text and club progress',
        [('jap_j1_update_staff_records_for_status', 'JapJ1::update_staff_records_for_status', '0x663db0', 'jap_j1.cpp', 'thiscall, ret 0x18',
          'regra: quem vence a primeira fase ganha o registro de fase vencida; quem vence a segunda e também venceu a primeira é campeão sem final (as linhas das três tabelas com 5, o registro de campeão e só duas fases); senão ganha o registro de fase vencida e vai à final; na final, o vencedor é campeão e o perdedor ganha o registro de eliminado',
          'rule: the first stage winner gets the stage won record; the second stage winner that also won the first is champion with no final (the rows of the three tables set to 5, the champions record and only two stages); otherwise it gets the stage won record and goes to the final; in the final, the winner is champion and the loser gets the knocked out record'),
         ('jap_j1_build_fate_news_text', 'JapJ1::build_fate_news_text', '0x6640a0', 'jap_j1.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na J1: venceu a primeira fase; venceu as duas fases (campeão); venceu a segunda fase e vai à final; venceu ou perdeu a final; na temporada inteira e na final com a notícia completa, o texto base',
          'the fate news text in the J1: won the first stage; won both stages (champions); won the second stage and goes to the final; won or lost the final; over the whole season and in the final with the full news, the base text'),
         ('jap_j1_set_club_progress', 'JapJ1::set_club_progress', '0x664400', 'jap_j1.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação na J1: na temporada inteira, o clube marcado com 1 na tabela não muda; senão a melhor colocação entre 2 e 5 vira 2 e a pior entre 2 e 5 vira 5; nas outras fases, como vêm',
          "the place forecast in the J1: over the whole season, a club marked 1 in the table keeps its codes; otherwise the best place 2 to 5 becomes 2 and the worst 2 to 5 becomes 5; in the other stages, as they come")])
CAMPOS = [('liga +0x2c (STAGE_COUNT = 2 quando o mesmo clube vence as duas fases); linha +0x37; entrada das estatisticas +0x73..+0x75; grava',
           'league +0x2c (STAGE_COUNT = 2 when the same club wins both stages); row +0x37; statistics entry +0x73..+0x75; writes')]
INTERESSA = []
