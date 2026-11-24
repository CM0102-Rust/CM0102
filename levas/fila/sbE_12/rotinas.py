# Os conflitos dos times B na virada espanhola (spa_first.cpp; leitor sbE, rodada 5, 24/11/2026). So roda com a Segunda B
# carregada (SpaFirst::start_new_season 0x84feb0: com o objeto de 0x9cf7b8), o que pede a base NORMAL (como os grupos da
# Segunda B do cx14; o simular.py fixa MINIMUM). BASE = 'NORMAL', que nenhuma ferramenta le ainda: passar a chave ao
# fila.py, ao cadeia.sh e ao simular.py antes de integrar. As ligas e a data do mwf_14 (a Espanha, a virada no verao).
# - 0x850970 SpaFirst::resolve_b_team_clashes thiscall() ret 0 (chamada direta, ecx = a Primeira): as linhas da
#   Primeira com parceiro (0x540a50, a marca diferente de 0): na Segunda (a linha por League::find_club_row 0x68ab70,
#   caixa 0x4c1), promovido conta uma vaga; com o clube principal rebaixado (3) a linha 0xfb, a noticia (0x48ece0 com
#   ecx = a Segunda) e um salvo; senao 0xfe; na Segunda B (os grupos 0x9cf62c..0x9cf638: a propria Segunda B e as fases
#   0 a 2) com o principal rebaixado, a linha no grupo (caixa 0x4fc; sem grupo fica a linha da busca anterior, no comeco
#   o proprio objeto da fase 2: defeito do original), promovido conta a vaga do grupo, a linha 0xfe. As linhas da
#   Segunda: parceiro na Segunda B (caixa 0x52e): promovido conta; com o principal rebaixado a linha 3 e a noticia (ecx =
#   a Segunda B), senao 0xfe. Depois as vagas e os salvos nas linhas da Segunda (0xff -> 0; 3 -> 0xfe), as vagas de cada
#   grupo (0xff -> 0) e o 0xfb da Segunda vira 3. Prologo sub esp, 0x428 (6 bytes).
# O corpo (resolve_b_team_clashes, com a BTeamLimits) serve tambem ao PorPrm::apply_b_team_promotion_limits 0x7d0320
# (sbE_13), conferido instrucao a instrucao. Sem dependencias da fila.
# Igual no cm0102-1080.exe (inteira). Comparador: so a original (marca linhas e manda noticias); nenhum 0xfb sobrando.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('spa_first_resolve_b_team_clashes', 0x850970, [0x81, 0xec, 0x28, 0x04, 0x00, 0x00])]
DOCS = ('os conflitos dos times B na virada espanhola', 'the Spanish B team clashes at the turnover',
        [('spa_first_resolve_b_team_clashes', 'SpaFirst::resolve_b_team_clashes', '0x850970', 'spa_first.cpp', 'thiscall, ret 0',
          'com a Segunda B, nenhum time B termina na mesma divisão do clube principal: o time B da Segunda cujo principal cai da Primeira cai junto (com a notícia) e salva um rebaixado; o que subiria é barrado e a vaga passa ao próximo indefinido; o mesmo na Segunda B, grupo a grupo',
          'with Segunda B, no B team ends in the same division as its senior club: a Second Division B team whose senior club goes down from the First goes down too (with the news) and saves a relegated club; one that would go up is held back and the place goes to the next undecided club; the same in Segunda B, group by group')])
CAMPOS = []
INTERESSA = []
