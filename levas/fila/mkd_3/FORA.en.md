# mkd_3: 0x85fc50 `STAFF_CONTRACT::set_to_desired_contract` stays out (11/11/2026)

*English version of [FORA.md](FORA.md); the Portuguese file is the original.*

Reserved for 0x85fc50 (the desired contract: wage demand, signing-on value, clauses, bonuses and length). The whole
disassembly was read and the rules proved (see `levas/reorganizacao/mercado/fechamento_mkd1.en.md`), but the routine
**was not ported** in this round:

- 3,109 instructions in a frame with a C++ exception handler (0x962a76, two local Dates with a destructor, `trylevel`
  0 → 1), 0x258 bytes of locals;
- it calls three originals outside the CRT: `staff_interest_in_job_at_club` (0x844780, the staff branch), and on the
  error path 0x944cff/0x90d130/0x5e8290 (the "Error" box of staff_contracts.cpp line 3150);
- it calls competition virtual methods (`[0xadadfc][div]`, slots +0x98 and +0x9c, with a league of type 2) and
  `get_max_wage` up to four times in a row in the same computation;
- it draws (`utils_random(0x1c)` 0x90cfc0, the day of the contract end), so the comparer must restore the game's
  generator and the whole contract state (0x50 bytes), [0xae1080], the text 0xde1f64 and the person (+0x3d).

A byte-faithful port of that needs a round of its own (two or three helper routines: the value of the clauses, the
length, the staff branch), with the comparer writing the whole contract side by side. Defects found that the port must
keep: the `&&` instead of `||` at 0x861d6b..0x861d70 (the error-box path reads address 0) and the reputation variable
at [esp+0x48] read without a value when the person has a player sheet, a staff job and no non-player sheet (see the
closing file).

Extension point that makes the port unnecessary (proved in the disassembly): the wage demand comes out in `contract
+0xc` = ftol of the final value (0x862483..0x862493) and the signing-on value in `contract +8` (0x85ffe2..0x86016b); a
detour that calls the original and adjusts both on the way out changes the demand without rewriting the routine; the
return value (int) is the contract value (adjusted wage + the lower of the bonus-and-clause sum and 10% of the value),
which the callers compare.
