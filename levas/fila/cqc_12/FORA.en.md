# cqc_12 out of the 266th batch (2026-11-01)

With `dia_das_selecoes` (0x76b100) in Rust, the two Rust careers to 2002-06-01 come out different (from 2002-148 on). The cause is not the port: it is a defect in the original.

- The squad selection (0x76c120, original) only fills the position codes for squads of 22, 24 or 26 players.
- On 2002-05-29 the 23-man World Cup squads arrive, and the code of the 23rd slot keeps whatever was on the stack. That value goes to the line-up (0x6e2000).
- With 0x76b100 in Rust, the call-up (0x76bab0) runs deeper in the private stack, and the leftover there changes on every run: low halves of heap pointers, such as 0x48c8, 0x4078 and 0x4408.
- Proof: filling the stack with a fixed value before the call-up makes the careers equal (diagnosis only). Such a stack zeroing has been forbidden since 10/31.

Ways out, to be decided with Lucas:
- **(b)** A fix to the original in 0x76c120: give codes to squads of 23 and 25, porting the routine.
- **(c)** Make the DLL's own allocations leave no heap pointers on the private stack (`simulacao.rs`/`alocador.rs`).

With either one, the item goes back to the queue unchanged.

## Resolution: way out (b), approved by Lucas (2026-11-01)

0x76c120 joined the item as its 4th routine (`escolher_elenco_da_selecao`), faithful to the original plus the fix: in 23- and 25-man squads the slots from the 22nd on get, in order, 0x202, 0x88 and 0x808, the slots the 24 table has beyond the 22 table. The first 22 stay the original's (see `cqc12_codigos`).

- The comparator runs only the original, with the tape open: the slot assignment (0x6e2000) is recorded as a unit with the list and the codes. The Rust copy builds the list and the codes from the same state and checks them. In 23- and 25-man squads the fixed slots are left out (branch).
- Proof on 89ce07b + 266th + the item (4 routines): two Rust careers to 2002-06-01 equal (`f8972df1c83e582c`, also in 3 more single runs). The 23-man fix runs on 2001-170..172, 2002-026 and 2002-148 (2002-05-29).
- Game comparing to 2002-01-01: 0 divergences in cqc_12. 4 divergences remain in other routines: `veto_dos_fisios_e_da_diretoria` ×3 (2001-198..201, one draw more or less with the same return) and `amistoso_enviar_pedido` ×1 (2001-198). The base without the item has 0 up to 2002-01-01; with the item the comparing mode follows another path and reaches cases those two comparisons did not cover.