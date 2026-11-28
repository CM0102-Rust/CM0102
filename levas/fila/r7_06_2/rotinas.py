# A campanha europeia de 2002 das eliminatorias (wc_europe_league.cpp; leitor r7_06, rodada 7, 28/11/2026). Estava
# fora (rx08_11/FORA.md, sobras.md) por ser codigo morto; o motivo continua valendo, conferido na desmontagem: so o
# construtor 0x918200 (ja em Rust, construct_wc_europe_league) a chama (0x918477), quando o ano do jogo nao e anterior
# ao da campanha, que nunca e antes de 2004 (wc_europe_campaign_year); o construtor so e chamado pelo
# setup_set_game_start_date (0x830990) com o ano da data de inicio. So um exe com o ano de inicio mudado chega aqui (o
# YearChanger do CM0102Patcher em anos 4k+1; nos anos 4k ele pula a chamada: 0x918473 vira jmp). Escrita porque a
# rodada 5 pede o exe inteiro (destino: o exe independente).
# Tipo: TELAS, so para entrar numa leva sem mutacoes pela carreira (nada a exercita: a comparacao nunca dispara). A
# prova e a emulacao (ver PRONTO.txt). Se a sessao principal preferir, segurar no SEGURADOS.txt.
# - 0x91b760 WcEuropeLeague::load_2002_campaign thiscall ret 0, quadro de 0x224 bytes sem SEH: a tabela local dos 9
#   grupos (6 chaves cada, -1 nas vagas vazias dos 4 ultimos), +0xf6 = 1, o pool +0xee (free 0x9452ca se havia; +0xf2 =
#   0x32; malloc 0x944e46 de 0xc8; nulo: caixa 0x8ea), +0xf2 = 0; por grupo, por nacao, database_get_national_team
#   0x53b3d0 duas vezes (a lista do grupo e o pool); grupo 0: +0x3e = tamanho, +0xb1 = malloc(0x3b x tamanho) (nulo:
#   caixa 0x906, sem liberar o pool) e League::init_table_row 0x687430 (linha, selecao, 0, 0); grupos 1..8:
#   WcEuropeLeague::create_group_stage 0x9196d0 (k - 1, lista, tamanho); depois create_runners_up_stage 0x91a140, a
#   virtual +0x5c, free do pool, +0xee = +0xf2 = 0, +0x45 = 1 e as 230 chamadas a 0x91db60 (a tabela tirada da
#   desmontagem por script: rascunho r7_06\extrai_r7_06.py). O original passa a linha e o tamanho com lixo de pilha nos
#   bytes de cima (movsx dx, bl / o byte em [esp+0x14]); as chamadas so leem o byte e a palavra: a copia passa limpos.
# Chama a 0x91db60 pelo endereco (o desvio do r7_06_1, se integrado; senao a original): nao depende de nome do r7_06_1.
# Prologo: sub esp, 0x224 (6 bytes, sem relativo). Igual no cm0102-1080.exe (inteira).
TELAS = True
ROTINAS = [('wc_europe_league_load_2002_campaign', 0x91b760, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00])]
DOCS = ('a campanha europeia de 2002 das eliminatorias', 'the European 2002 qualifying campaign',
        [('wc_europe_league_load_2002_campaign', 'WcEuropeLeague::load_2002_campaign', '0x91b760', 'wc_europe_league.cpp', 'thiscall',
          'regra: as eliminatorias europeias de um jogo cujo ano ja chegou ao da campanha carregam a campanha real de 2002: uma selecao isenta, os nove grupos reais (o da propria secao e oito fases de grupo; seis selecoes nos cinco primeiros e cinco nos quatro ultimos), a fase dos segundos colocados, as posicoes previstas, a secao ativa e os 230 jogos de setembro de 2000 a outubro de 2001 com os placares conhecidos quando o jogo foi feito (os 23 da ultima rodada sem placar). Codigo morto no jogo como ele e: a campanha nunca e antes de 2004 e o jogo comeca em 2001',
          "rule: the European qualifying of a game whose year has reached the campaign's loads the real 2002 campaign: one exempt nation, the nine real groups (the section's own and eight group stages; six nations in the first five and five in the last four), the runners-up stage, the predicted positions, the section active and the 230 matches of September 2000 to October 2001 with the scores known when the game was made (the 23 of the last round without a score). Dead code in the game as it is: the campaign is never before 2004 and the game starts in 2001")])
