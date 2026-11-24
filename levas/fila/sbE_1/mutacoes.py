# Mutacoes (sbE_1), com a Finlandia ate 2002-08-01 (a virada finlandesa no fim de 2001):
# - os grupos trocados (os dez do sul no grupo Norte): acusada pelo comparador lado a lado;
# - os vencedores da repescagem ficam onde estao (outra Ykkonen em 2002: a carreira);
# - a cadeia sem a troca com o grupo Sul (os clubes da fase 0 nao sobem nem descem: a carreira).
MUTACOES = [
    ('sbE_1_grupos_trocados',
     'let group = if k < 10 { south } else { north }; // sbE_1: the ten southernmost',
     'let group = if k < 10 { north } else { south }; // mutation: the groups swapped',
     'sbE_1 finnish groups by latitude'),
    ('sbE_1_repescagem_parada',
     'if read_u8(entry + 5) == 1 && read_usize(club + club::DIVISION) != first.competition().addr() {',
     'if read_u8(entry + 5) == 7 && read_usize(club + club::DIVISION) != first.competition().addr() { // mutation',
     'sbE_1 finnish play-off winner up'),
    ('sbE_1_cadeia_sem_sul',
     'exe::league_promote_and_relegate(this, premier(), read_usize(League(first()).stages()), 1, 1, -1, -1)\n}',
     '{ let _ = first; 0 } // mutation: the South group does not move\n}',
     'sbE_1 finnish promotion and relegation'),
]
