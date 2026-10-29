# A disciplina (discipline.cpp): contar o jogo cumprido da suspensão (Discipline::update_ban_progress 0x556e90, ~273
# mil chamadas na carreira, pelos órgãos 0x7e7b80/0x7e7ff0/0x7e85b0/0x7e88d0 e pela 0x557120), soltar a suspensão
# (Discipline::release_ban 0x556d00, ~300) e apagar o histórico de suspensões da pessoa (Discipline::clear_ban_history
# 0x55e1d0, ~500, pelos regens 0x7a59e0/0x7abac0 e pela 0x556830). Trocas: das candidatas da família só a 0x55e1d0 e a
# 0x7a8350 (1 chamada) estavam livres na lista; a 0x556830 (remove_player) fica para depois, as três daqui têm efeito
# lido pelas escalações. 0x7e82e0 (sem criar), 0x7e8590, 0x58e340 (histórico no arquivo), 0x54c5b0 e 0x58f110
# originais; qsort 0x945333 e bsearch 0x946464 com a comparação 0x55e5f0. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Sem SEH, sem sorteio; caixa: na 0x556d00 sem o item no índice das nações (segue,
# como a original), na 0x55e1d0 pessoa nula ou não jogadora (fatal, a original). Comparadores: a 0x556e90 pelo plano
# (grava no arquivo); a 0x556d00 e a 0x55e1d0 lado a lado.
SAVE = False
ROTINAS = [('contar_jogo_da_suspensao', 0x556e90, [0x83, 0xec, 0x10, 0x0f, 0xbe, 0x44, 0x24, 0x18]),
           ('soltar_suspensao', 0x556d00, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('apagar_historico_de_suspensoes', 0x55e1d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a disciplina: contar o jogo cumprido da suspensão, soltar a suspensão e apagar o histórico de suspensões da pessoa',
        "discipline: counting a served match of a ban, releasing a ban and clearing a person's ban history",
        [('contar_jogo_da_suspensao', 'Discipline::update_ban_progress', '0x556e90', 'discipline.cpp', 'thiscall, ret 0xc',
          'a suspensão da pessoa no órgão da regra: por jogos, a partir do começo (+4/+6) e com conta, +1 no contador (+1); ao chegar ao total (+0), ou por dias quando hoje chega ao começo + dias (+2, 0x54c5b0), vai para o histórico (0x58e340) e é solta (0x556d00)',
          "the person's ban in the rule's ruling body: match bans, from the start date (+4/+6) and when counting, +1 to the counter (+1); on reaching the total (+0), or day bans once today reaches start + days (+2, 0x54c5b0), it goes to the history (0x58e340) and is released (0x556d00)"),
         ('soltar_suspensao', 'Discipline::release_ban', '0x556d00', 'discipline.cpp', 'thiscall, ret 8',
          'cada órgão das 4 regras da suspensão (+0xc..+0xf) a esquece (0x7e8590); na quarta, o item da pessoa sai do índice das nações (o último no lugar, qsort); a suspensão fica livre (+0 = 0xff, +2 = −1)',
          'each ruling body of the ban\'s 4 rules (+0xc..+0xf) forgets it (0x7e8590); on the fourth, the person\'s item leaves the nation index (last one moved in, qsort); the ban becomes free (+0 = 0xff, +2 = −1)'),
         ('apagar_historico_de_suspensoes', 'Discipline::clear_ban_history', '0x55e1d0', 'discipline.cpp', 'thiscall, ret 4',
          'o último deslocamento do histórico de suspensões da pessoa (+0x18 do registro de 29 bytes) = −1 (0x58f110)',
          "the last offset of the person's ban history (+0x18 of the 29-byte record) = −1 (0x58f110)")])
CAMPOS = [('disciplina +4: vetor das suspensões (20 bytes: +0 jogos ou 0xff por dias, +1 jogos cumpridos, +2 dias, +4 dia e +6 ano do começo, +0xc..+0xf as 4 regras); ponteiro; lê e grava (contar_jogo_da_suspensao, soltar_suspensao)',
           'discipline +4: ban vector (20 bytes: +0 matches or 0xff for days, +1 matches served, +2 days, +4 day and +6 year of the start, +0xc..+0xf the 4 rules); pointer; reads and writes (contar_jogo_da_suspensao, soltar_suspensao)'),
          ('disciplina +0xc/+0x12: índice das nações (8 bytes: id, nação), conta (palavra); lê e grava (soltar_suspensao)',
           'discipline +0xc/+0x12: nation index (8 bytes: id, nation), count (word); reads and writes (soltar_suspensao)'),
          ('disciplina +0x14: registros de 29 bytes por pessoa (+0x14 começo e +0x18 último deslocamento do histórico); lê e grava −1 em +0x18 (apagar_historico_de_suspensoes); +0x18 arquivo do histórico (contar_jogo_da_suspensao)',
           'discipline +0x14: 29-byte records per person (+0x14 start and +0x18 last offset of the history); reads and writes −1 to +0x18 (apagar_historico_de_suspensoes); +0x18 history file (contar_jogo_da_suspensao)'),
          ('órgão disciplinar: entrada de 14 bytes da pessoa (+6 índice da suspensão, −1 sem); lê (contar_jogo_da_suspensao); grava −1 pela 0x7e8590 (soltar_suspensao)',
           "ruling body: the person's 14-byte entry (+6 ban index, −1 none); reads (contar_jogo_da_suspensao); writes −1 via 0x7e8590 (soltar_suspensao)"),
          ('pessoa +0x1a nação, +0x1e segunda nação, +0x61 jogador; [0xae2c90] data do jogo; lê',
           'person +0x1a nation, +0x1e second nation, +0x61 player; [0xae2c90] game date; reads')]
INTERESSA = ['CMRTE', 'SGE']
