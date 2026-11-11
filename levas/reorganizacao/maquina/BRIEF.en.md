# Machine-code suspects: brief for the reviewers (11/11/2026)

The readability batches 34-58 of the CM 01/02 Rust port (`D:\Sistemas\CM0102`) only rename: offsets become field
names, raw reads become typed accessors, literals become constants. Each batch passed the career proofs, but two of
them still changed behaviour in routines the careers do not reach (the match stats screen: a local closure
`byte(k, d)` taken for the memory helper; the scout report: `p + SCOUT_NAME_REF + 0xc` where `p` already included
SCOUT_NAME_REF). To find any other, the DLL was built at every batch commit and the machine code compared function by
function (`engenharia-reversa/maquina_por_funcao.py`, normalised: call targets by name, addresses inside the DLL as
`@`). The files you get list, for one batch, the functions whose **memory accesses** changed: a different displacement
on a record (any base register other than esp/ebp), a different access width, or a different sign/zero extension.

Most of these are still harmless compiler noise: a helper inlined or not (its accesses appear in or vanish from the
caller), the internal calling convention changing (`ecx/edx` vs the stack), a function now returning `this` from the
callee, a typed accessor reading the same field through a different register, `movzx` vs `movsx` where the value is
only compared for equality or masked. Your job is to tell the noise from a real change of behaviour.

For each function (`=== name`, then a unified diff of the normalised disassembly, before `-` and after `+`):
- `noise`: the same effect, with a one-line reason;
- `behaviour`: the code now reads or writes a different offset, width or sign, or compares differently, in a way that can
  change a result. Find the Rust source of the function at the batch commit (`git -C D:\Sistemas\CM0102 show COMMIT --
  rust/src/motor/...`, or `git show COMMIT~1:path` for the previous version) and give the file, the line and the fix;
- `unclear`: say what is missing.

To see more of one function: `python D:\Sistemas\CM0102\engenharia-reversa\maquina_por_funcao.py DLL_A MAP_A DLL_B MAP_B
--mostrar NAME` with the DLLs and maps in the scratchpad folder `dlls\` (named by commit; the pairs are in
`levas/reorganizacao/maquina/pares.txt`, `before after count bytes`). Do not edit project files and do not build.
Write a JSON list (`batch`, `function`, `verdict`, `reason`, and for `behaviour`: `file`, `line`, `fix`) to the path
given. Finish with a short summary in Portuguese, leading with any `behaviour` found.
