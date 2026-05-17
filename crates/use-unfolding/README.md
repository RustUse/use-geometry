# use-unfolding

Unfolding primitives for the RustUse geometry workspace.

`use-unfolding` provides unfold-step and plan records that can later back polyhedral-net workflows.

## Example

```rust
use use_unfolding::{UnfoldingPlan, UnfoldingStep};

let plan = UnfoldingPlan::new(vec![UnfoldingStep::new(0, 1)]);

assert_eq!(plan.step_count(), 1);
```
