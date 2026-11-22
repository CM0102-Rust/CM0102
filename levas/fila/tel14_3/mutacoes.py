# Mutations (tel14_3). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat), on the club's screen > Tactics (both side by side):
# - a tab click keeps the new view as the previous one (click With Ball / Without Ball);
# - a zone click leaves the hover zone at the new zone (With Ball tab, click another of the 12 zones).
MUTACOES = [
    ('tel14_3_tab_previous', 'write_u8(ts + f::PREVIOUS_VIEW, view);', 'write_u8(ts + f::PREVIOUS_VIEW, tab);'),
    ('tel14_3_zone_hover', 'write_u32(ts + t::HOVER_ZONE, old);', 'write_u32(ts + t::HOVER_ZONE, read_u32(ts + t::ZONE));'),
]
