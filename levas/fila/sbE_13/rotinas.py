# Os limites dos times B na virada portuguesa (por_prm.cpp; leitor sbE, rodada 5, 24/11/2026). So roda com a Segunda
# Divisao B carregada (PorPrm::start_new_season 0x7cf9e0, mwe_15: com o objeto de 0x9cf7c0), o que pede a base NORMAL
# (como a Segunda B portuguesa do cx13). BASE = 'NORMAL', que nenhuma ferramenta le ainda: passar a chave ao fila.py, ao
# cadeia.sh e ao simular.py antes de integrar. As ligas e a data do mwe_15.
# - 0x7d0320 PorPrm::apply_b_team_promotion_limits thiscall() ret 0 (chamada direta, ecx = a Primeira): o mesmo corpo
#   do SpaFirst::resolve_b_team_clashes (sbE_12, conferido instrucao a instrucao) com a Liga de Honra (0x9cf610), a
#   Segunda B (0x9cf7c0) nas zonas Norte (a propria), Centro (a fase 0) e Sul (a fase 1) e as caixas 0x390, 0x3c9 e
#   0x3f7 de por_prm.cpp; a linha velha da busca comeca no objeto da fase 1 (defeito do original). Prologo sub esp,
#   0x420 (6 bytes).
# Depende do sbE_12 (BTeamLimits, resolve_b_team_clashes, compare_b_team_clashes): integrar depois dele.
# Igual no cm0102-1080.exe (inteira). Comparador: so a original (marca linhas e manda noticias); nenhum 0xfb sobrando.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('por_prm_apply_b_team_promotion_limits', 0x7d0320, [0x81, 0xec, 0x20, 0x04, 0x00, 0x00])]
DOCS = ('os limites dos times B na virada portuguesa', 'the Portuguese B team limits at the turnover',
        [('por_prm_apply_b_team_promotion_limits', 'PorPrm::apply_b_team_promotion_limits', '0x7d0320', 'por_prm.cpp', 'thiscall, ret 0',
          'com a Segunda Divisão B, nenhum time B termina na mesma divisão do clube principal: o da Liga de Honra cujo principal cai da Primeira cai junto (com a notícia) e salva um rebaixado; o que subiria é barrado e a vaga passa ao próximo indefinido; o mesmo nas zonas da Segunda B',
          'with the Second Division B, no B team ends in the same division as its senior club: a Liga de Honra one whose senior club goes down from the Premier Division goes down too (with the news) and saves a relegated club; one that would go up is held back and the place goes to the next undecided club; the same in the Second Division B zones')])
CAMPOS = []
INTERESSA = []
