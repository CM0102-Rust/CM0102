# Mutations through the load of the holiday save (prova_save.py --leva N with CARGA; the impression up to 2001-08-28):
# - rxf_5_pares_sem_ordem: the club-keyed pairs are not sorted: the lookups by club (bsearch with 0x844740) miss, so
#   the computer clubs' shortlists and bids after the load change.
# - rxf_5_reputacao_velha: the clubs' reputation copies are not raised to the current reputation.
MUTACOES = [
    ('rxf_5_pares_sem_ordem',
     '    exe::crt_qsort(read_usize(this + f::PAIRS_BY_CLUB), read_u32(this + f::PAIRS_BY_CLUB_COUNT), 8, SHORTLIST_PAIR_COMPARE); // rxf_5: the club-keyed pairs',
     '    let _ = SHORTLIST_PAIR_COMPARE; // rxf_5: the club-keyed pairs (mutation)',
     'rxf_5 shortlist pairs'),
    ('rxf_5_reputacao_velha',
     '            if read_i16(entry + 5) < reputation { // +5: the club reputation copy',
     '            if false && read_i16(entry + 5) < reputation { // +5: the club reputation copy (rxf_5 mutation)',
     'rxf_5 shortlist pairs'),
]
