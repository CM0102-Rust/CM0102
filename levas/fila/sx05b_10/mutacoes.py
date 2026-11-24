# Mutacoes (sx05b_10, telas: a cadeia nao as roda; para provar o comparador numa sessao do Lucas).
# - o destrutor do premio de equipe sem o destrutor da base (a tabela virtual fica a da classe: o comparador lado a
#   lado acusa). Os outros destrutores liberam memoria e o comparador roda so a original: sem mutacao que ele acuse.
MUTACOES = [
    ('sx05b_10_sem_base',
     '    write_usize(this, team_award_obj::VTABLE);\n    award_destruct_base(this);',
     '    write_usize(this, team_award_obj::VTABLE); // mutation: no base destructor'),
]
