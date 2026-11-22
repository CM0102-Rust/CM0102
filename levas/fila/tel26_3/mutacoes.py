# Mutations (tel26_3). The automatic career opens no screen: the chain runs none. The Job Information comparer
# (display list side by side, with the list built) catches both in a session of Lucas's: open Job Information
# (manager menu) and change the page or come back to it (the rebuild with the list already made is compared).
# - the "Secure" threshold moved (some rows read Slightly Insecure / Secure differently);
# - the club column with the nation's button id.
MUTACOES = [
    ('tel26_3_secure_threshold', '} else if s < 0x2134 {', '} else if s < 0x1f00 {'),
    ('tel26_3_club_link', 'job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d5, club as u32]', 'job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d6, club as u32]'),
]
