# Example

Suppose you want to read a PADMet file `metabolism.padmet`. You could use the following code snippet:

```rust
use std::collections::HashMap;
use padmet::spec::PadmetSpec;

# fn main() {
    // Read the PADMet file and load it in the padmet_object struct
    let padmet_file: PathBuf =
        PathBuf::from("metabolism.padmet");
    let padmet_object: PadmetSpec = PadmetSpec::from_file(padmet_file).unwrap();
    let pathways = padmet_object.get_pathways();
    // Get the the sets of reactions
    let pathway_reactions: HashMap<String, HashSet<String>> = padmet_object.get_pathways_reactions();
    assert_eq!(pathways.len(), pathway_reactions.len());
    assert!(
        pathway_reactions
            .get("FAO-PWY")
            .expect("pathway FAO-PWY should exist in the test padmet file")
            .len()
            > 0
    );

# }
```
