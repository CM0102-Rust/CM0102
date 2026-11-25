# Mutations (rx10_10): the screens batch runs no mutations in the chain (the automatic career opens no screen); these
# are for a session of Lucas with JOGAR_COMPARANDO.bat, on the nominations screen of a monthly or yearly award (open it
# from the "vote" news item):
# - the pending vote found only for another player: get_pending_vote returns 0 where the original returns the slot,
#   caught by its side-by-side comparer as soon as the screen asks about the picked player.
MUTACOES = [
    ('rx10_10_voto_de_outro',
     '    if read_usize(slot) == player {\n        covered!("rx10_10 pending vote found");',
     '    if read_usize(slot) == player.wrapping_add(1) { // mutation\n        covered!("rx10_10 pending vote found");',
     'rx10_10 pending vote found'),
]
