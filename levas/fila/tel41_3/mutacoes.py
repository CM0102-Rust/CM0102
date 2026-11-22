# Mutations (tel41_3). The season end of the competitions that finish before 2002-06-01 (the league in May, the cups'
# finals) runs these in the ENG career; the records and their news do not flow back into the simulation, so the
# comparers catch them (update_season_end_records side by side over the whole object; archive and send_record_news by
# their plans):
# - the season's most points never beats the all-time record;
# - the runs are not kept when the season closes (the plan expects one saved run per club);
# - the record flag is not cleared after its news (the plan expects every flag cleared).
MUTACOES = [
    ('tel41_3_most_points',
     "        // tel41_3: the season's most points beats the all-time record\n        if season > -1 && (all_time == -1 || season > all_time) {",
     "        // mutation: never beaten\n        if season > 0x7fff_fffe && (all_time == -1 || season > all_time) {"),
    ('tel41_3_runs_kept',
     '    comp_stats_save_team_runs(stats); // tel41_3: the runs kept for the next season',
     '    // mutation: the runs are not kept'),
    ('tel41_3_flag_cleared',
     '        write_u32(flag, 0); // tel41_3: the flag cleared',
     '        // mutation: the flag stays'),
]
