# Batch 307: two divergences that depend on the memory layout (18/11/2026)

The batch passed the mutations (**10 of 10 caught**) and the careers (Rust ×2 equal, `31731bcff4c2f351`, garbage
detector equal). The real game (ENG KOR ARG ITA FRA ESP AUS until 2002-08-01) ran three times with the batch's code:

| Run | Bench | DLL | Result |
|---|---|---|---|
| 1 | 3 | the batch's | 1 divergence, 2002-182: `post_team_media_story(club 0x1bf176c7, kind 0x5b)` |
| 2 | 4 (the chain's) | the same | 1 divergence: `contract_manager_offer_contract(person 0x1c5c53ce, ...)` |
| 3 | 3 | the batch's + the byte diagnostic in the comparer | **0 divergences** (fingerprint `570739b553297b4b`) |

Neither routine belongs to this batch: `post_team_media_story` is from batch 294, `contract_manager_offer_contract`
from batch 270.

## What is known

- **`post_team_media_story`:** the copy's draws are a prefix of the original's and the log entry matches; only the
  content of the news item delivered to the human differs. The holiday auto-response (`0x784100`) does not handle type
  `0xbdf` and is ruled out. The comparer now writes the differing bytes (offset, delivered, built), so the next
  occurrence names the field.
- **`contract_manager_offer_contract`:** the same divergence as batch 302 (see `levas/302/correcoes.md`, in Portuguese), there
  blamed on a diagnostic probe that consumed draws. The probe is gone and the divergence came back: that explanation
  is not enough.
- The same code in different folders (the memory layout changes with the folder) gives a different divergence or
  none. The hypothesis is a common cause: stack or memory residue read by some routine, original or port.

## What stays open

Both divergences are **unexplained**. The batch publishes on the third run, like batch 302, and the investigation
continues with the stack probe (`sonda_pilha.py`) and the comparer's byte diagnostic.
