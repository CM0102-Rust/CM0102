# A fila dos regens (PLAYER_REGEN::process_regen_queue 0x7a65d0, pelo laco do dia 0x5cccd0, todo dia: os regens jovens
# da fila que vencem, pela 0x7abac0, centenas por carreira (cpv_9 "regen contratado"), e os tecnicos de reserva refeitos,
# raros), o veiculo da noticia pelo clube (media_pick_outlet_type_for_club 0x757f30, pelas noticias do trofeu e da
# venda) e pela pessoa (media_pick_outlet_type_for_person 0x757fa0, pelas historias do jogador e pela comemoracao da
# contratacao 0x75b6e0 nas transferencias). Chamadas originais pelo endereco: 0x456770 (datas), 0x7abac0 (o regen jovem),
# 0x944c9f e a caixa de erro (aviso_em), 0x945ff0 (pow, pow_do_jogo) e _ftol; o resto ja em Rust. Iguais no
# cm0102-1080.exe. Trocas: das candidatas de player_regen.cpp, a regenerate_as_non_player 0x7ab340 (lida e deixada de
# fora: nada nas cadeias mostra que ela rode ate 2002-06-01; o contador da 0x4cb990 com pessoa nula, que so ela usa,
# nunca apareceu) e as de noticia e de arquivo; a 0x7a59e0 e a 0x7abac0 sao grandes. Simulacao: as mutacoes pela
# carreira ate 2002-06-01, todas na fila dos regens jovens. Comparadores: a fila so a original com o plano (as k vencidas
# saem, as seguintes sobem, a conta cai k; sem vencidas, lado a lado no cabecalho; com reservas a refazer, so a
# original); os dois veiculos lado a lado no sorteio e na devolucao.
SAVE = False
ROTINAS = [('fila_de_regens', 0x7a65d0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('veiculo_pelo_clube', 0x757f30, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x81, 0xb8, 0x80, 0x00, 0x00, 0x00, 0x6a, 0x18]),
           ('veiculo_pela_pessoa', 0x757fa0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61])]
DOCS = ('a fila dos regens: os regens jovens que vencem e os técnicos de reserva refeitos; o veículo da notícia pelo clube e pela pessoa',
        'the regen queue: youth regens that fall due and rebuilt reserve coaches; the news outlet by club and by person',
        [('fila_de_regens', 'PLAYER_REGEN::process_regen_queue', '0x7a65d0', 'player_regen.cpp', 'thiscall, ret 0',
          'da frente da fila dos regens jovens (12 bytes: id e data), enquanto a data for antes de hoje, o regen jovem (0x7abac0) e a entrada sai; com mais de 2 reservas de técnico, refaz conta − 2 delas: a pessoa que empresta os dados (0x7a6e60) ganha o registro de não jogador, o cargo no clube da máquina e as notas pela reputação (pow e faixas) ou sorteadas; a reserva sai (0x7a82b0)',
          'from the front of the youth regen queue (12 bytes: id and date), while the date is before today, the youth regen (0x7abac0) and the entry leaves; with more than 2 reserve coaches, rebuilds count − 2 of them: the person lending the data (0x7a6e60) gets the non-player record, the job at the AI club and the ratings from reputation (pow and bands) or drawn; the reserve leaves (0x7a82b0)'),
         ('veiculo_pelo_clube', 'media_pick_outlet_type_for_club', '0x757f30', 'media.cpp', 'cdecl',
          'reputação do clube acima de 6250: o veículo 0 a 6 pelo sorteio(100); senão 2 ou 4 pelo sorteio(4)',
          "club reputation above 6250: outlet 0 to 6 from random(100); otherwise 2 or 4 from random(4)"),
         ('veiculo_pela_pessoa', 'media_pick_outlet_type_for_person', '0x757fa0', 'media.cpp', 'cdecl',
          'o mesmo pela reputação do jogador (+0xb da ficha)',
          "the same from the player's reputation (player data +0xb)")])
CAMPOS = [('PLAYER_REGEN +0x10: fila dos regens jovens (12 bytes: +0 id, +4 data), +0x14 conta, +0x40 liga; id e valor; lê, tira da frente (fila_de_regens)',
           'PLAYER_REGEN +0x10: youth regen queue (12 bytes: +0 id, +4 date), +0x14 count, +0x40 switch; id and value; reads, removes from the front (fila_de_regens)'),
          ('PLAYER_REGEN +0x1c: reservas de técnico (+0 id, +4 nação), +0x20 conta; id; lê (fila_de_regens)',
           'PLAYER_REGEN +0x1c: reserve coaches (+0 id, +4 nation), +0x20 count; id; reads (fila_de_regens)'),
          ('pessoa +0x69 registro de não jogador ([0xae23c0] + 0x44 × id: +8, +0xa, +0xc notas, +0x10/+0x11); +0x3d cargo; +0x22; +0x57; ponteiro e valor; grava (fila_de_regens)',
           'person +0x69 non-player record ([0xae23c0] + 0x44 × id: +8, +0xa, +0xc ratings, +0x10/+0x11); +0x3d job; +0x22; +0x57; pointer and value; writes (fila_de_regens)'),
          ('clube +0x19f: as 5 vagas de treinador; +0xd3 auxiliar; +0x80 reputação; ponteiro e valor; lê e grava (fila_de_regens), lê a reputação (veiculo_pelo_clube)',
           'club +0x19f: the 5 coach slots; +0xd3 assistant; +0x80 reputation; pointer and value; reads and writes (fila_de_regens), reads reputation (veiculo_pelo_clube)'),
          ('ficha +0xb reputação, +9, +0xd, +0xf; valor; lê (fila_de_regens, veiculo_pela_pessoa)',
           'player data +0xb reputation, +9, +0xd, +0xf; value; reads (fila_de_regens, veiculo_pela_pessoa)')]
INTERESSA = ['CMRTE', 'GPF2']
