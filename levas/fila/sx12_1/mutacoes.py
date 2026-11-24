# Mutacoes (sx12_1). Os comparadores (retorno e tabela lado a lado) acusam as tres na primeira chamada:
# - a lesao atual nunca achada (o lesionado passa por apto na escalacao, na escolha da lesao e na partida);
# - nenhum continente com nacao escolhida (premios, disciplina e copas continentais);
# - as linhas inativas com o indice 0x7f (o original nao as toca): so o comparador acusa, na criacao do jogo, sem mudar
#   a carreira (uma contagem errada quebraria as listas de partidas).
MUTACOES = [
    ('sx12_lesao_atual',
     "read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add((k as usize).wrapping_mul(0x30)) // sx12_1: the injury entry",
     "0 // mutation: never injured",
     'sx12_1 current injury'),
    ('sx12_continente_escolhido',
     'return 1; // sx12_1: a chosen nation of the continent',
     'return 0; // mutation: no chosen nation',
     'sx12_1 continent has a selected nation'),
    ('sx12_linhas_inativas_numeradas',
     "let n = Nation(read_usize(row.addr())); // sx12_1: the row's nation",
     'let n = Nation(read_usize(row.addr())); if !(n.is_null() || n.league_selected() != 0) { row.set_active_index(0x7f); } // mutation: inactive rows marked',
     'sx12_1 active key nation row'),
]
