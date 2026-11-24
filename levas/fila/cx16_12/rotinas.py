# A atualização da tabela das eliminatórias europeias e os classificados da repescagem (wc_europe_league.cpp; leitor
# cx16, rodada 5, 24/11/2026). Simulação: a virtual +0x94 roda com os resultados das eliminatórias europeias (a campanha
# de 2004: os grupos de setembro de 2004 a outubro de 2005, a repescagem em novembro de 2005): ATE = '2005-12-31' (o
# pedido da rodada 5). Os classificados só são marcados quando a seção acaba (novembro de 2005).
# - 0x91a7b0 WcEuropeLeague::update_positions_and_status thiscall ret 0 (virtual +0x94): League::update_positions_and_status
#   0x684cd0 (chamada direta, já em Rust); com +0xfb = 0 e a virtual +0x10 (1, 0) com al != 0, 0x91a7e0. Prólogo push esi;
#   mov esi, ecx; call rel32: entra pelas RELATIVAS (0x91a7b0, 3), ver INTEGRAR_ANTES.txt.
# - 0x91a7e0 WcEuropeLeague::mark_play_off_qualifiers thiscall ret 0: só com +0xfb = 0, que vira 1; sem repescagem
#   (+0x101 = -1): o primeiro da tabela da fase dos segundos (+0xc[+0x100], linha 0) procurado nos grupos -1..+0xff;
#   achado, a linha com situação 7 (+0x37) e a virtual +0x44 (clube, 7, grupo, 0, 0, 0; ret 0x18); não achado, caixa
#   0x65f. Com repescagem: para cada grupo, cada linha da tabela de times da repescagem (+0xa7, linhas de 6, contagem
#   +0x3e, relidas a cada volta) com situação 1 tem a sua linha no grupo (League::get_table_row 0x6826d0, ret 4) com
#   situação 7. Prólogo sub esp, 0x20c (6 bytes).
# Iguais no cm0102-1080.exe. Comparadores: a atualização só a original; os classificados só a original contra o plano (a
# marca +0xfb e, com repescagem, cada vencedor com situação 7 no seu grupo).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_europe_league_update_positions_and_status', 0x91a7b0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0xa5, 0xd6, 0xff]),
           ('wc_europe_league_mark_play_off_qualifiers', 0x91a7e0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('a atualização da tabela das eliminatórias europeias e os classificados da repescagem',
        'the European qualifying table update and the play-off qualifiers',
        [('wc_europe_league_update_positions_and_status', 'WcEuropeLeague::update_positions_and_status', '0x91a7b0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: a atualização da tabela das eliminatórias europeias é a da liga e, quando todas as eliminatórias acabam, os vencedores da repescagem são marcados como classificados (uma vez)',
          'rule: the European qualifying table update is the league one and, once the whole qualifying is complete, the play-off winners are marked as qualified (once)'),
         ('wc_europe_league_mark_play_off_qualifiers', 'WcEuropeLeague::mark_play_off_qualifiers', '0x91a7e0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: quando as eliminatórias europeias acabam, os vencedores da repescagem ficam com a situação de classificado (7) nos seus grupos; sem repescagem, o melhor segundo colocado é que se classifica, com o seu recorde',
          'rule: when the European qualifying ends, the play-off winners get the qualified status (7) in their groups; without a play-off the best runner-up qualifies instead, with its record')])
CAMPOS = [('eliminatórias da Europa +0xfb marca de classificados já marcados (dword) [inferência pelo uso]; valor; lê e grava',
           'European qualifying +0xfb qualifiers-marked flag (dword) [inference from the use]; value; reads and writes')]
INTERESSA = []
