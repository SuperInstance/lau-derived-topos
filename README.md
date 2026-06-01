# lau-derived-topos

> Elementary topos theory + derived functors — a computational framework for agent knowledge systems.

[![tests](https://img.shields.io/badge/tests-81-green)]()
[![license](https://img.shields.io/badge/license-MIT-blue)]()

## What This Does

This crate implements the foundational structures of **category theory** as concrete, serializable Rust types — with a specific goal: modeling **agent knowledge** as a topos.

The core journey is:

1. **Build a category** with objects, morphisms, and composition
2. **Upgrade it to a topos** by adding a subobject classifier (Ω), exponentials, and power objects
3. **Run the internal logic** (conjunction, disjunction, implication, negation) on the topos's truth values
4. **Model agents** whose knowledge states are subobjects, with operations for merging (colimit), common knowledge (limit), and consistency checking
5. **Compute homology** of chain complexes and take **derived functors** — the algebraic topology layer that measures what's lost when a functor isn't exact

Every structure is `Serialize`/`Deserialize` via `serde`. No external math dependencies — linear algebra is done with a built-in `DenseMatrix` type.

## The Key Idea

An **elementary topos** is a category that behaves enough like **Set** to support intuitionistic logic internally. Formally, it has:

| Structure | Implementation |
|-----------|----------------|
| **Finite limits** | Terminal object + products (detected via hom-sets) |
| **Subobject classifier** Ω | `SubobjectClassifier` — an object with a "true" morphism `1 → Ω` and characteristic functions for every subobject |
| **Power objects** `P(A)` | `HashMap` from object names — `P(A)` represents the "set of all subobjects of A" |
| **Exponentials** `B^A` | `HashMap` keyed `"B,A"` — internal hom-objects |

The internal logic of a topos is **intuitionistic** in general (law of excluded middle may fail). In the test suite, we construct a mini `Set` topos where `Ω = {true, false}` and the logic *is* classical — verifying the well-known theorem that **Set is a Boolean topos**.

For **agent knowledge**, each agent is modeled as knowing a set of subobject indices. Operations correspond to categorical constructions:

- **Merge knowledge** = colimit (union over agents)
- **Common knowledge** = limit (intersection over agents)
- **Consistency** = no agent knows a subobject and its negation

## Install

```bash
cargo add lau-derived-topos
```

## Quick Start

```rust
use lau_derived_topos::*;

// 1. Create a category with objects and morphisms
let mut cat = category::Category::new(
    vec!["empty".into(), "unit".into(), "bool".into()],
    vec![],
);

// Add morphisms: true and false as global elements 1 → Ω
let true_m = cat.add_morphism(types::MorphismData {
    name: "true".into(),
    domain: "unit".into(),
    codomain: "bool".into(),
});
let false_m = cat.add_morphism(types::MorphismData {
    name: "false".into(),
    domain: "unit".into(),
    codomain: "bool".into(),
});

// 2. Wrap in a topos with subobject classifier
let topos = topos::Topos {
    category: cat,
    subobject_classifier: subobject_classifier::SubobjectClassifier {
        omega: "bool".into(),
        true_morphism: true_m,
        characteristic_map: vec![],
        category: /* ... */,
    },
    exponentials: HashMap::new(),
    power_objects: HashMap::new(),
};

// 3. Query the internal logic
let logic = topos.internal_logic();
assert!(logic.law_of_excluded_middle()); // Classical in Set

// 4. Build an agent knowledge base
let mut kb = agent_knowledge::AgentKnowledgeBase {
    topos,
    agents: vec!["alice".into(), "bob".into()],
    knowledge: HashMap::new(),
};
kb.knowledge.insert("alice".into(), vec![0, 1, 2]);
kb.knowledge.insert("bob".into(), vec![1, 3]);

// Merge (colimit): union of all knowledge
let merged = kb.merge_knowledge(&["alice", "bob"]);
assert_eq!(merged, vec![0, 1, 2, 3]);

// Common knowledge (limit): intersection
let common = kb.common_knowledge();
assert_eq!(common, vec![1]);
```

## API Reference

### `category::Category`

The foundational type. A small category with named objects and morphisms, explicit identity morphisms, and a composition table.

| Method | Description |
|--------|-------------|
| `new(objects, morphisms)` | Create a category; identity morphisms are added automatically |
| `identity(obj)` | Index of the identity morphism for an object |
| `compose(f, g)` | Compose morphisms `g ∘ f`; returns `None` if codomain/domain mismatch |
| `hom_set(from, to)` | All morphism indices from one object to another |
| `isomorphic(a, b)` | Check if two objects are isomorphic |
| `initial_object()` | Find an initial object (unique morphism to every object) |
| `terminal_object()` | Find a terminal object (unique morphism from every object) |
| `products()` | Detect candidate product objects |
| `equalizers()` | Detect candidate equalizers |
| `is_cartesian_closed()` | Check for terminal + products (proxy) |
| `is_topos()` | Preliminary check (would need subobject classifier too) |

### `morphism::Morphism`

Wrapper with properties:

- `is_mono(cat, f_idx)` — left-cancellative
- `is_epi(cat, f_idx)` — right-cancellative
- `is_iso(cat, f_idx)` — both mono and epi

### `functor::Functor`

A mapping between categories (object map + morphism map).

| Method | Description |
|--------|-------------|
| `on_objects(obj)` | Map an object through the functor |
| `on_morphisms(f)` | Map a morphism index through the functor |
| `preserves_identities()` | Check `F(id_X) = id_{F(X)}` |
| `preserves_composition()` | Check `F(g ∘ f) = F(g) ∘ F(f)` |
| `is_full()` | Surjective on each hom-set |
| `is_faithful()` | Injective on each hom-set |
| `is_essentially_surjective()` | Every target object is iso to some `F(X)` |
| `is_equivalence()` | Full + faithful + essentially surjective |

### `natural_transformation::NaturalTransformation`

A family of morphisms `β_A: F(A) → G(A)` indexed by objects.

| Method | Description |
|--------|-------------|
| `naturality_square(f)` | Check `β_B ∘ F(f) = G(f) ∘ β_A` for a given morphism |
| `is_natural_isomorphism()` | Every component is an isomorphism |

### `subobject_classifier::SubobjectClassifier`

The "truth-value object" Ω of a topos.

| Method | Description |
|--------|-------------|
| `characteristic(subobject)` | Get the characteristic morphism for a subobject |
| `pullback(chi)` | Inverse: recover the subobject from its characteristic morphism |
| `truth_values()` | Global elements `1 → Ω` (the truth values) |

### `topos::Topos`

An elementary topos: a category with a subobject classifier, exponentials, and power objects.

| Method | Description |
|--------|-------------|
| `power_object(obj)` | Get `P(A)` for an object |
| `exponential(a, b)` | Get `B^A` for objects |
| `internal_logic()` | Extract the `InternalLogic` of this topos |

### `internal_logic::InternalLogic`

The logical calculus inside a topos. Operates on truth value indices.

| Method | Formula |
|--------|---------|
| `conjunction(p, q)` | `p ∧ q` |
| `disjunction(p, q)` | `p ∨ q` |
| `implication(p, q)` | `p ⇒ q` |
| `negation(p)` | `¬p` |
| `universal(var, prop)` | `∀var. prop` |
| `existential(var, prop)` | `∃var. prop` |
| `law_of_excluded_middle()` | `∀p. p ∨ ¬p`? |
| `is_intuitionistic()` | `¬(∀p. p ∨ ¬p)`? |

### `sheaf::Sheaf`

A presheaf with values in `Vec<f64>` and linear restriction maps (matrices).

| Method | Description |
|--------|-------------|
| `restriction(morphism, value)` | Apply the restriction map for a morphism |
| `gluing(cover, values)` | Attempt to glue compatible local sections |
| `is_sheaf()` | Checks the sheaf condition (trivially true in this implementation) |

### `dense_matrix::DenseMatrix`

A simple `rows × cols` matrix of `f64` with Gaussian elimination.

| Method | Description |
|--------|-------------|
| `new(data)` | Construct from row vectors |
| `zeros(rows, cols)` | Zero matrix |
| `identity(n)` | `n × n` identity |
| `multiply(&other)` | Matrix multiplication |
| `transpose()` | Transpose |
| `rank()` | Rank via row echelon form |
| `nullity()` | `cols - rank` |
| `kernel_basis()` | Basis for `ker(A)` via Gaussian elimination |
| `image_basis()` | Basis for `im(A)` via column independence |

### `chain_complex::ChainComplex`

A sequence of groups and differentials `... → C_{n+1} → C_n → C_{n-1} → ...`.

| Method | Description |
|--------|-------------|
| `boundary(degree)` | The differential `∂_n` |
| `cycle(degree)` | `ker(∂_n)` — the cycle group basis |
| `boundary_group(degree)` | `im(∂_{n+1})` — the boundary group basis |
| `homology(degree)` | `dim(ker(∂_n)) - dim(im(∂_{n+1}))` |
| `is_exact()` | All homology groups are trivial |
| `verify_boundary_squared_zero()` | Check `∂² = 0` |

### `derived_functor::DerivedFunctor`

Left/right derived functors computed via projective/injective resolutions.

| Method | Description |
|--------|-------------|
| `compute_ln(object, resolution)` | `L_nF(A)` via projective resolution |
| `compute_rn(object, resolution)` | `R^nF(A)` via injective resolution |
| `zeroth_is_original()` | `L_0F ≅ F` (true iff `derived_order == 0`) |

### `agent_knowledge::AgentKnowledgeBase`

Multi-agent knowledge modeled as subobjects in a topos.

| Method | Categorical meaning | Description |
|--------|--------------------|-------------|
| `knowledge_state(agent)` | — | Get an agent's known subobject indices |
| `merge_knowledge(&[agents])` | Colimit (union) | Combine knowledge from multiple agents |
| `common_knowledge()` | Limit (intersection) | Knowledge shared by all agents |
| `consistent(agent)` | — | No contradictions (no duplicate knowledge entries) |

## How It Works

### Architecture

The crate is organized as layered mathematical structures, each building on the previous:

```
types.rs          →  MorphismData, DenseMatrix (shared types)
    ↓
category.rs       →  Category (objects, morphisms, composition, limits)
    ↓
morphism.rs       →  Morphism (mono, epi, iso properties)
functor.rs        →  Functor (structure-preserving maps between categories)
natural_transformation.rs → NaturalTransformation (maps between functors)
    ↓
subobject_classifier.rs → SubobjectClassifier (Ω, truth values)
topos.rs          →  Topos (category + Ω + exponentials + power objects)
internal_logic.rs →  InternalLogic (∧, ∨, ⇒, ¬, ∀, ∃)
    ↓
sheaf.rs          →  Sheaf (presheaf with restriction maps + gluing)
chain_complex.rs  →  ChainComplex (differentials, homology, exactness)
dense_matrix.rs   →  DenseMatrix (linear algebra for homology computations)
derived_functor.rs → DerivedFunctor (L_nF, R^nF via resolutions)
    ↓
agent_knowledge.rs → AgentKnowledgeBase (multi-agent knowledge in a topos)
```

### Serde Serialization

Every type derives `Serialize` and `Deserialize`. You can serialize an entire `AgentKnowledgeBase` (including its topos, category, and all structure) to JSON and reconstruct it:

```rust
let json = serde_json::to_string_pretty(&kb)?;
let kb2: AgentKnowledgeBase = serde_json::from_str(&json)?;
```

### Linear Algebra

The built-in `DenseMatrix` type provides:

- **Matrix multiplication** and **transpose** for differential composition
- **Kernel basis** via Gaussian elimination with partial pivoting — used for cycle groups
- **Image basis** via transpose + kernel + column independence testing — used for boundary groups
- **Rank** and **nullity** computed from row echelon form

This is deliberately kept minimal — no BLAS, no external crates. All arithmetic is `f64`.

### No External Dependencies

The only dependencies are `serde` and `serde_json`. All mathematics is implemented from scratch.

## The Math

### Categories

A **category** C consists of:
- A collection of **objects** `Ob(C)`
- For each pair `A, B ∈ Ob(C)`, a set of **morphisms** `Hom(A, B)`
- **Composition**: `g ∘ f ∈ Hom(A, C)` for `f ∈ Hom(A, B)`, `g ∈ Hom(B, C)`
- **Identity**: `id_A ∈ Hom(A, A)` for each object
- Associativity and unit laws

This crate represents morphisms as named entries in a `Vec<MorphismData>`, with composition stored as a lookup table `((f, g), h)` meaning `h = g ∘ f`.

### Topos Theory

An **elementary topos** is a category E with:

1. **Finite limits** — terminal object `1` and pullbacks (equivalently, `1` + products + equalizers)
2. **Subobject classifier** — an object `Ω` with a morphism `true: 1 → Ω` such that every subobject `m: A' ↪ A` has a unique **characteristic morphism** `χ_m: A → Ω` making a pullback square
3. **Power objects** — for each `A`, an object `P(A)` representing `Ω^A` (subobjects of A)

The **internal logic** of a topos is a type of intuitionistic higher-order logic. The truth values are the global elements `1 → Ω`, and logical connectives are defined via the structure of Ω.

### Sheaf Theory

A **presheaf** on a category C is a functor `F: C^op → Set`. A **sheaf** additionally satisfies:

- **Locality**: if two sections agree on all overlaps, they're equal
- **Gluing**: compatible local sections can be uniquely glued to a global section

This crate models sheaves with `Vec<f64>` stalks and matrix-valued restriction maps.

### Homological Algebra

A **chain complex** is a sequence of abelian groups and differentials:

```
... → C_{n+1} --∂_{n+1}--> C_n --∂_n--> C_{n-1} → ...
```

where `∂_n ∘ ∂_{n+1} = 0` (boundary of a boundary is zero).

The **nth homology** is `H_n = ker(∂_n) / im(∂_{n+1})`. It measures the failure of exactness at `C_n`.

A **derived functor** `L_nF` (left) or `R^nF` (right) measures the failure of a functor `F` to preserve exact sequences. Key theorem: `L_0F ≅ F` (the zeroth derived functor recovers the original).

### Yoneda Lemma

The **Yoneda lemma** states that for any functor `F: C → Set` and object `A ∈ C`:

```
Nat(h^A, F) ≅ F(A)
```

where `h^A = Hom(A, -)` is the representable functor. This means natural transformations from the representable functor are in bijection with elements of `F(A)`.

The test suite verifies a simplified instance of this theorem for single-object categories.

### Agent Knowledge as Categorical Constructions

| Operation | Category theory | Implementation |
|-----------|----------------|----------------|
| Merge knowledge | **Colimit** (coproduct/union) | `HashSet` union of subobject indices |
| Common knowledge | **Limit** (product/intersection) | `HashSet` intersection |
| Consistency | No contradictory subobjects | No duplicate entries |
| Knowledge state | Subobject selection | `Vec<usize>` of known indices |

Merge is **associative** (verified in tests): `(A ∪ B) ∪ C = A ∪ (B ∪ C)`.

## Testing

81 tests covering all modules. Run with:

```bash
cargo test
```

Test categories include:

- **Category operations**: identity, composition, isomorphisms, initial/terminal objects
- **Morphism properties**: mono, epi, iso
- **Functor laws**: preserves identities, preserves composition, equivalence detection
- **Natural transformations**: naturality squares, natural isomorphisms
- **Subobject classifier**: truth values, characteristic functions, pullbacks
- **Topos structure**: power objects, exponentials, cartesian closedness
- **Internal logic**: conjunction, disjunction, implication, negation, LEM
- **Sheaves**: restriction maps, gluing, sheaf condition
- **DenseMatrix**: multiply, transpose, kernel basis, image basis, rank, nullity
- **Chain complexes**: boundary² = 0, homology computation, exactness
- **Derived functors**: `L_0F ≅ F`, computation via resolutions
- **Agent knowledge**: knowledge states, merge, common knowledge, consistency, associativity
- **Serde round-trips**: every major type serializes and deserializes correctly
- **Yoneda lemma**: simplified verification for single-object categories

## License

MIT
