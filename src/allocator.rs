use bumpalo::Bump;
thread_local!(static IR_ARENA: Bump = Bump::new());
pub fn ir_arena() -> &'static Bump { return IR_ARENA.with(|a| a) }
thread_local!(static AA_ARENA: Bump = Bump::new());
pub fn aa_arena() -> &'static Bump { return AA_ARENA.with(|a| a) }