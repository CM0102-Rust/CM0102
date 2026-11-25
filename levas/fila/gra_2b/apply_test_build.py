"""Scratch test build for check_player.py: copy this file and pedacos.rs next to a `git archive HEAD rust cm0102-base`
extracted in ./b, run it, then build with /MAP. It puts pedacos.rs (and a test entry) after the integrar anchor of the
copy's motor.rs and exports `gra2b_player_day` (lib.rs wrapper and ddraw.def). Never on the main tree."""
import os
S = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(S, 'b', 'rust', 'src', 'motor.rs')
orig = os.path.join(S, 'motor.rs.orig')
if not os.path.exists(orig):
    open(orig, 'w', encoding='utf-8', newline='\n').write(open(p, encoding='utf-8').read())
s = open(orig, encoding='utf-8').read()
anc = 'compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);\n'
assert s.count(anc) == 1
cod = open(os.path.join(S, 'pedacos.rs'), encoding='utf-8').read()
test = '''
/// Emulator entry (scratch build only): one contracted player's day, 0x5ce4b6..0x5d352d.
pub(crate) unsafe fn gra2b_player_day_entry(p: usize, pd: usize, club: usize, contract: usize, b_team: u32, slot70: *mut u32) {
    let mut d = StaffDay { p, state_off: read_u32(p) as usize * staff_state::SIZE, pd, club, contract, mate: 0, away_from_job: false, odds: 0, b_team };
    let mut run = StaffDayRun { slot70: *slot70 };
    staff_day_player(&mut d, &mut run);
    *slot70 = run.slot70;
}
'''
s = s.replace(anc, anc + '\n' + cod + test)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
l0 = s[:s.index(anc)].count('\n') + 2
open(os.path.join(S, 'region.txt'), 'w').write('%d %d' % (l0, l0 + cod.count('\n') + test.count('\n')))
lib = os.path.join(S, 'b', 'rust', 'src', 'lib.rs')
lo = lib + '.orig'
if not os.path.exists(lo):
    open(lo, 'w', encoding='utf-8', newline='\n').write(open(lib, encoding='utf-8').read())
open(lib, 'w', encoding='utf-8', newline='\n').write(open(lo, encoding='utf-8').read() + '\n#[no_mangle]\npub unsafe extern "C" fn gra2b_player_day(p: usize, pd: usize, club: usize, contract: usize, b_team: u32, slot70: *mut u32) {\n    motor::gra2b_player_day_entry(p, pd, club, contract, b_team, slot70)\n}\n')
df = os.path.join(S, 'b', 'rust', 'ddraw.def')
t = open(df, encoding='utf-8').read()
if 'gra2b_player_day' not in t:
    open(df, 'w', encoding='utf-8', newline='\n').write(t.rstrip('\n') + '\n    gra2b_player_day\n')
print('ok', l0)
