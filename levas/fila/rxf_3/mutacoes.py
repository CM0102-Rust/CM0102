# The destroyed object is never read again, so the career cannot catch a mutation here: the comparer (side by side
# over the vtable dword, jogo real) does.
# - rxf_3_vtable_errada: ~AWARD puts back the next dword of the vtable: the comparer diverges at the first award
#   destroyed (free_game_data in the creation of the career).
# The scalar deleting destructor has no mutation: it does not run (no object of the plain base class).
MUTACOES = [
    ('rxf_3_vtable_errada',
     '    covered!("rxf_3 award destructor");\n    write_usize(this, AWARD_VTABLE);',
     '    covered!("rxf_3 award destructor");\n    write_usize(this, AWARD_VTABLE + 4); // rxf_3 mutation',
     'rxf_3 award destructor'),
]
