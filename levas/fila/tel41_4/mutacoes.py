# Mutations (tel41_4):
# - the archive key starts at 0 instead of -1 (every competition is built at the start of the career; the plan of the
#   constructor's comparer);
# - the instance count does not go down (the destructor's plan, when a competition object is destroyed);
# - the projected best and worst positions swapped, in the projection and in the fallback (the board's weekly
#   confidence 0x697c40 and the manager's exit reason 0x69b350 read them: the ENG career, and the comparer side by
#   side).
MUTACOES = [
    ('tel41_4_archive_key',
     'write_i32(stats + comp_stats_obj::ARCHIVE_KEY1, -1); // tel41_4: no archived season yet',
     'write_i32(stats + comp_stats_obj::ARCHIVE_KEY1, 0); // mutation: key 0'),
    ('tel41_4_instances',
     '    write_i32(COMP_STATS_INSTANCES, read_i32(COMP_STATS_INSTANCES).wrapping_sub(1)); // tel41_4: one instance fewer',
     '    // mutation: the count stays'),
    ('tel41_4_projected_best',
     'comp_stats_club::BEST_POSITION); // tel41_4: projected best',
     'comp_stats_club::WORST_POSITION); // mutation: the worst as best'),
    ('tel41_4_fallback_best',
     '*best = read_u8(entry + comp_stats_club::BEST_POSITION); // tel41_4: fallback best',
     '*best = read_u8(entry + comp_stats_club::WORST_POSITION); // mutation: the worst as best'),
]
