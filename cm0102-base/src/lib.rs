//! Shared base of the CM 01/02 3.9.68 DLLs (the project's `ddraw.dll`, `save/`; later Tweaks and CLZ-DLL).
//! Nothing here depends on the match engine or the simulation: the two links to them are hooks (`ganchos`).
//!
//! [pt] Base comum das DLLs do CM 01/02 3.9.68 (a `ddraw.dll` do projeto, `save/`; depois o Tweaks e a CLZ-DLL).
//! Nada aqui depende do motor nem da simulação: as duas ligações com eles são ganchos (`ganchos`).

pub mod apresentacao;
pub mod conteiner;
pub mod ganchos;
pub mod reducao;
pub mod registro;
pub mod video;
pub mod win;
