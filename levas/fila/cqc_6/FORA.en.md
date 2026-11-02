# cqc_6 out of the 266th batch (2026-11-01)

With `liga_jogos_para_a_tv` (0x685640, `League::move_fixtures_for_tv`) in Rust, the two Rust careers come out different, already on 2001-036 in the short window (12,135 fewer random draws that day). In the career to 2002-06-01, with 0x76c120 fixed, two states remain: 96d1c4fee745c48a and f8972df1c83e582c.

- **The Rust copy is right:** its inputs and decisions (dates, slots, matches, scores) are identical on every run.
- **The cause is a leftover on the private stack.** The routine is called every day and almost always returns early. What changes on every run is the process heap (0xeb0000 to 0x11d0000), where the DLL's own allocations go (Vec, String, format!): the simulation mode's fixed allocator only covers the game's. The base of `ddraw.dll` does not change between runs of the same file (option (c) study, 11/01).
- **Proof:** filling the stack below it with a fixed value gives 10 out of 10 equal (diagnosis only; stack zeroing has been forbidden since 10/31).
- **The original routine that reads this garbage was not found:** the spot moves with the heap layout.

Way out approved by Lucas (11/01): option (c). The DLL's allocations go to a fixed region in simulation mode; in the experiment, that gave 6 out of 6 equal careers to 2001-03-01 and 3 out of 3 to 2002-06-01. Once it is in, the item goes back to the queue unchanged.

## Resolution (2026-11-02)

Option (c) went into the commit after the 267th batch: the DLL's allocations go to a fixed region (0x0b000000..0x0f000000) in simulation mode, and the trampolines sit in a single block. With it, the 266th batch plus this item gave two equal Rust careers and an equal garbage detector run. The item goes back to the queue unchanged.
