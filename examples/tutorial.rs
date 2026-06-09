//! # Derived Topos — Tutorial
//!
//! Progressive lessons covering categories, functors, topos theory, sheaves,
//! chain complexes, derived functors, and agent knowledge bases.
//!
//! Run with: `cargo run --example tutorial`

use lau_derived_topos::{
    AgentKnowledgeBase, Category, ChainComplex, DenseMatrix, DerivedFunctor, Functor,
    MorphismData, NaturalTransformation, Sheaf, SubobjectClassifier,
    Topos,
};
use std::collections::HashMap;

// ── Lesson 1: Categories, Objects, and Morphisms ────────────────────────────
//
// A Category has objects and morphisms with identity laws and composition.
// We build small finite categories and explore their properties.

fn lesson_1_categories() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 1: Categories & Morphisms");
    println!("═══════════════════════════════════════════\n");

    // Build a simple category: A → B → C
    let cat = Category::new(
        vec!["A".into(), "B".into(), "C".into()],
        vec![
            MorphismData { name: "f".into(), domain: "A".into(), codomain: "B".into() },
            MorphismData { name: "g".into(), domain: "B".into(), codomain: "C".into() },
        ],
    );

    println!("Category: A → B → C");
    println!("  Objects: {:?}", cat.objects);
    println!("  Morphisms:");
    for (i, m) in cat.morphisms.iter().enumerate() {
        println!("    [{}] {}: {} → {}", i, m.name, m.domain, m.codomain);
    }

    // Identity morphisms are auto-generated
    let id_a = cat.identity("A");
    println!("\n  id_A = morphism [{}]: {}", id_a, cat.morphisms[id_a].name);
    let id_b = cat.identity("B");
    println!("  id_B = morphism [{}]: {}", id_b, cat.morphisms[id_b].name);

    // Composition: g ∘ f
    let f = cat.morphism_index("f").unwrap();
    let g = cat.morphism_index("g").unwrap();
    let gf = cat.compose(f, g);
    println!("\n  g ∘ f = {:?}", gf.map(|idx| cat.morphisms[idx].name.clone()));

    // Hom-sets
    println!("\n  Hom(A, B) = {:?}", cat.hom_set("A", "B"));
    println!("  Hom(A, A) = {:?}", cat.hom_set("A", "A"));
    println!("  Hom(A, C) = {:?} (no direct morphism)", cat.hom_set("A", "C"));

    // Add composition g ∘ f = h explicitly
    let mut cat2 = cat.clone();
    let h_idx = cat2.add_morphism(MorphismData {
        name: "h".into(), domain: "A".into(), codomain: "C".into(),
    });
    cat2.add_composition(f, g, h_idx);
    let composed = cat2.compose(f, g);
    println!("\n  After adding h = g ∘ f:");
    println!("  g ∘ f = {:?} ({})", composed, cat2.morphisms[composed.unwrap()].name);
    println!();
}

// ── Lesson 2: Categorical Limits and Universal Properties ───────────────────
//
// Terminal objects, initial objects, products, and equalizers encode
// universal properties — the bread and butter of category theory.

fn lesson_2_limits() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 2: Limits & Universal Properties");
    println!("═══════════════════════════════════════════\n");

    // Category with terminal object: every object has exactly one morphism to *
    let mut cat = Category::new(
        vec!["A".into(), "B".into(), "*".into()],
        vec![
            MorphismData { name: "!A".into(), domain: "A".into(), codomain: "*".into() },
            MorphismData { name: "!B".into(), domain: "B".into(), codomain: "*".into() },
        ],
    );
    println!("Category with terminal object *:");
    println!("  Terminal object: {:?}", cat.terminal_object());
    println!("  Initial object: {:?}", cat.initial_object());

    // Add product structure: A×B with projections
    let _p1 = cat.add_morphism(MorphismData { name: "π₁".into(), domain: "A×B".into(), codomain: "A".into() });
    let _p2 = cat.add_morphism(MorphismData { name: "π₂".into(), domain: "A×B".into(), codomain: "B".into() });
    cat.objects.push("A×B".into());
    println!("\nAfter adding product object A×B:");
    println!("  Has products? {}", cat.has_products());
    println!("  Cartesian closed? {}", cat.is_cartesian_closed());
    println!("  Is topos? {}", cat.is_topos());

    // Check isomorphisms
    let iso_cat = Category::new(
        vec!["X".into(), "Y".into()],
        vec![
            MorphismData { name: "f".into(), domain: "X".into(), codomain: "Y".into() },
            MorphismData { name: "g".into(), domain: "Y".into(), codomain: "X".into() },
        ],
    );
    // Need to add compositions: g∘f = id_X, f∘g = id_Y
    let mut iso_cat2 = iso_cat.clone();
    let f_idx = iso_cat.morphism_index("f").unwrap();
    let g_idx = iso_cat.morphism_index("g").unwrap();
    let id_x = iso_cat.identity("X");
    let id_y = iso_cat.identity("Y");
    iso_cat2.add_composition(f_idx, g_idx, id_x);
    iso_cat2.add_composition(g_idx, f_idx, id_y);
    println!("\nIs X ≅ Y? {}", iso_cat2.isomorphic("X", "Y"));
    println!();
}

// ── Lesson 3: Functors — Maps Between Categories ────────────────────────────
//
// A Functor maps objects to objects and morphisms to morphisms while
// preserving identity and composition.

fn lesson_3_functors() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 3: Functors — Maps Between Categories");
    println!("═══════════════════════════════════════════\n");

    // Source category: {X, Y} with f: X → Y
    let source = Category::new(
        vec!["X".into(), "Y".into()],
        vec![MorphismData { name: "f".into(), domain: "X".into(), codomain: "Y".into() }],
    );

    // Target category: {A, B} with g: A → B
    let target = Category::new(
        vec!["A".into(), "B".into()],
        vec![MorphismData { name: "g".into(), domain: "A".into(), codomain: "B".into() }],
    );

    // Define functor F: source → target
    let f_idx = source.morphism_index("f").unwrap();
    let g_idx = target.morphism_index("g").unwrap();
    let id_x = source.identity("X");
    let id_y = source.identity("Y");
    let id_a = target.identity("A");
    let id_b = target.identity("B");

    let functor = Functor {
        source: source.clone(),
        target: target.clone(),
        object_map: HashMap::from([("X".into(), "A".into()), ("Y".into(), "B".into())]),
        morphism_map: HashMap::from([(id_x, id_a), (id_y, id_b), (f_idx, g_idx)]),
    };

    println!("Functor F: {{X→Y}} → {{A→B}}");
    println!("  F(X) = {}", functor.on_objects("X"));
    println!("  F(Y) = {}", functor.on_objects("Y"));
    println!("  F(f) = {}", target.morphisms[functor.on_morphisms(f_idx)].name);
    println!("  Preserves identities? {}", functor.preserves_identities());
    println!("  Preserves composition? {}", functor.preserves_composition());
    println!("  Is faithful? {}", functor.is_faithful());
    println!("  Is full? {}", functor.is_full());
    println!("  Is essentially surjective? {}", functor.is_essentially_surjective());
    println!("  Is equivalence of categories? {}", functor.is_equivalence());
    println!();
}

// ── Lesson 4: Natural Transformations ───────────────────────────────────────
//
// A natural transformation η: F → G assigns to each object a component
// morphism η_A: F(A) → G(A) such that the naturality squares commute.

fn lesson_4_natural_transformations() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 4: Natural Transformations");
    println!("═══════════════════════════════════════════\n");

    // Build two parallel functors F, G: C → D
    let cat_c = Category::new(
        vec!["A".into(), "B".into()],
        vec![MorphismData { name: "f".into(), domain: "A".into(), codomain: "B".into() }],
    );

    // Target has identity functors — F = G = identity
    let f_idx = cat_c.morphism_index("f").unwrap();
    let id_a = cat_c.identity("A");
    let id_b = cat_c.identity("B");

    let id_functor = Functor {
        source: cat_c.clone(),
        target: cat_c.clone(),
        object_map: HashMap::from([("A".into(), "A".into()), ("B".into(), "B".into())]),
        morphism_map: HashMap::from([(id_a, id_a), (id_b, id_b), (f_idx, f_idx)]),
    };

    // Natural transformation: identity → identity (components are identity morphisms)
    let nat = NaturalTransformation {
        source: id_functor.clone(),
        target: id_functor.clone(),
        components: HashMap::from([("A".into(), id_a), ("B".into(), id_b)]),
    };

    println!("Natural transformation id → id:");
    println!("  Component at A: morphism [{}] ({})", id_a, cat_c.morphisms[id_a].name);
    println!("  Component at B: morphism [{}] ({})", id_b, cat_c.morphisms[id_b].name);
    println!("  Naturality square for f? {}", nat.naturality_square(f_idx));
    println!("  Is natural isomorphism? {}", nat.is_natural_isomorphism());
    println!();
}

// ── Lesson 5: Sheaves and the Gluing Principle ──────────────────────────────
//
// A Sheaf assigns data to objects of a category with restriction maps
// along morphisms. The key axiom: compatible local data glues uniquely.

fn lesson_5_sheaves() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 5: Sheaves & the Gluing Principle");
    println!("═══════════════════════════════════════════\n");

    // Build a simple poset category as our "site": U ⊆ V
    let cat = Category::new(
        vec!["U".into(), "V".into(), "W".into()],
        vec![MorphismData { name: "incl".into(), domain: "U".into(), codomain: "V".into() }],
    );

    // A sheaf of R-values on this site
    let incl_idx = cat.morphism_index("incl").unwrap();
    let sheaf = Sheaf {
        category: cat,
        values: HashMap::from([
            ("U".into(), vec![1.0, 2.0]),
            ("V".into(), vec![3.0, 4.0]),
            ("W".into(), vec![5.0]),
        ]),
        restriction_maps: HashMap::from([
            // Restriction from V to U: linear map [[1,0],[0,1]]
            (incl_idx, vec![vec![1.0, 0.0], vec![0.0, 1.0]]),
        ]),
    };

    println!("Sheaf with stalks:");
    for (obj, vals) in &sheaf.values {
        println!("  stalk at {} = {:?}", obj, vals);
    }

    // Restrict a section
    let section_v = vec![3.0, 4.0];
    let restricted = sheaf.restriction(incl_idx, &section_v);
    println!("\nRestriction of section at V to U:");
    println!("  section at V = {:?}", section_v);
    println!("  restricted to U = {:?}", restricted);

    // Gluing
    let glued = sheaf.gluing(&[0, 1], &[vec![1.0, 2.0], vec![1.0, 2.0]]);
    println!("\nGluing compatible sections:");
    println!("  [1,2] + [1,2] (compatible) → {:?}", glued);

    let bad_glue = sheaf.gluing(&[0, 1], &[vec![1.0, 2.0], vec![3.0, 4.0]]);
    println!("  [1,2] + [3,4] (incompatible) → {:?}", bad_glue);
    println!("  is sheaf? {}", sheaf.is_sheaf());
    println!();
}

// ── Lesson 6: Chain Complexes and Homology ──────────────────────────────────
//
// A ChainComplex is a sequence of abelian groups with differentials ∂ₙ
// satisfying ∂² = 0. Homology Hₙ = ker(∂ₙ)/im(∂ₙ₊₁) measures "holes".

fn lesson_6_chain_complexes() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 6: Chain Complexes & Homology");
    println!("═══════════════════════════════════════════\n");

    // Simple chain complex: 0 → Z² → Z² → 0
    // ∂₁ = [[1, 0], [0, 0]] — maps (a,b) to (a, 0)
    let d1 = DenseMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 0.0]]);

    let complex = ChainComplex::new(
        vec![vec![0.0], vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0]],
        vec![d1.clone()],
    );

    println!("Chain complex: 0 → Z² --∂₁--> Z² → 0");
    println!("  ∂₁ = {:?}", d1.data);
    println!("  ∂² = 0? {}", complex.verify_boundary_squared_zero());

    // Compute kernel (cycles) and image (boundaries)
    let cycles = complex.cycle(0);
    let boundaries = complex.boundary_group(0);
    println!("  Cycles (ker ∂₀): {} basis vectors", cycles.len());
    for (i, c) in cycles.iter().enumerate() {
        println!("    basis {}: {:?}", i, c);
    }
    println!("  Boundaries (im ∂₁): {} basis vectors", boundaries.len());

    // Homology: dim(ker/im)
    println!("  H₀ dimension = {}", complex.homology(0));

    // Exact complex example
    let d_exact = DenseMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let exact_complex = ChainComplex::new(
        vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]],
        vec![d_exact.clone()],
    );
    println!("\nExact chain complex: 0 → Z → Z² → Z → 0");
    println!("  ∂₁ = {:?}", d_exact.data);
    println!("  ∂² = 0? {}", exact_complex.verify_boundary_squared_zero());
    println!("  is exact? {}", exact_complex.is_exact());
    println!("  H₀ = {}, H₁ = {}", exact_complex.homology(0), exact_complex.homology(1));
    println!();
}

// ── Lesson 7: Topos Theory and Internal Logic ───────────────────────────────
//
// A topos is a category with a subobject classifier Ω, finite limits,
// and power objects. Its internal logic supports intuitionistic reasoning.

fn lesson_7_topos_and_logic() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 7: Topos Theory & Internal Logic");
    println!("═══════════════════════════════════════════\n");

    // Build a small topos: Sets-like category with 1, Ω
    let cat = Category::new(
        vec!["1".into(), "Ω".into(), "A".into()],
        vec![
            MorphismData { name: "true".into(), domain: "1".into(), codomain: "Ω".into() },
            MorphismData { name: "false".into(), domain: "1".into(), codomain: "Ω".into() },
            MorphismData { name: "!A".into(), domain: "A".into(), codomain: "1".into() },
        ],
    );

    let true_idx = cat.morphism_index("true").unwrap();
    let false_idx = cat.morphism_index("false").unwrap();

    let subobject_classifier = SubobjectClassifier {
        omega: "Ω".into(),
        true_morphism: true_idx,
        characteristic_map: vec![],
        category: cat.clone(),
    };

    let topos = Topos {
        category: cat.clone(),
        subobject_classifier: subobject_classifier.clone(),
        exponentials: HashMap::new(),
        power_objects: HashMap::new(),
    };

    println!("Topos structure:");
    println!("  Terminal object: {:?}", cat.terminal_object());
    println!("  Subobject classifier Ω = {}", topos.subobject_classifier.omega);
    println!("  Truth values: {:?}", subobject_classifier.truth_values());
    println!("  Has terminal? {}", cat.has_terminal());

    // Internal logic
    let logic = topos.internal_logic();
    println!("\nInternal logic of the topos:");
    println!("  True morphism: [{}]", logic.true_idx);
    println!("  Truth value indices: {:?}", logic.truth_value_indices);

    // Logical operations
    println!("  true AND true = {} (true? {})", logic.conjunction(true_idx, true_idx),
        logic.conjunction(true_idx, true_idx) == true_idx);
    println!("  true AND false = {} (false? {})", logic.conjunction(true_idx, false_idx),
        logic.conjunction(true_idx, false_idx) == false_idx);
    println!("  true OR false = {} (true? {})", logic.disjunction(true_idx, false_idx),
        logic.disjunction(true_idx, false_idx) == true_idx);
    println!("  true → false = {} (false? {})", logic.implication(true_idx, false_idx),
        logic.implication(true_idx, false_idx) == false_idx);

    println!("  Law of excluded middle? {}", logic.law_of_excluded_middle());
    println!("  Is intuitionistic? {}", logic.is_intuitionistic());
    println!();
}

// ── Lesson 8: Agent Knowledge Bases ─────────────────────────────────────────
//
// AgentKnowledgeBase models multi-agent knowledge within a topos.
// Agents hold subobjects as knowledge; merging is colimit (union),
// common knowledge is limit (intersection).

fn lesson_8_agent_knowledge() {
    println!("═══════════════════════════════════════════");
    println!("  Lesson 8: Agent Knowledge Bases");
    println!("═══════════════════════════════════════════\n");

    // Set up a simple topos for the knowledge base
    let cat = Category::new(
        vec!["1".into(), "Ω".into(), "K".into()],
        vec![
            MorphismData { name: "true".into(), domain: "1".into(), codomain: "Ω".into() },
            MorphismData { name: "!K".into(), domain: "K".into(), codomain: "1".into() },
        ],
    );

    let true_idx = cat.morphism_index("true").unwrap();
    let subobject_classifier = SubobjectClassifier {
        omega: "Ω".into(),
        true_morphism: true_idx,
        characteristic_map: vec![],
        category: cat.clone(),
    };

    let topos = Topos {
        category: cat,
        subobject_classifier,
        exponentials: HashMap::new(),
        power_objects: HashMap::new(),
    };

    // Build knowledge base with 3 agents
    let mut knowledge = HashMap::new();
    knowledge.insert("alice".into(), vec![0, 1, 2]);    // knows facts 0, 1, 2
    knowledge.insert("bob".into(), vec![1, 2, 3]);       // knows facts 1, 2, 3
    knowledge.insert("carol".into(), vec![0, 2, 4]);     // knows facts 0, 2, 4

    let kb = AgentKnowledgeBase {
        topos,
        agents: vec!["alice".into(), "bob".into(), "carol".into()],
        knowledge,
    };

    println!("Agent knowledge base:");
    for agent in &kb.agents {
        println!("  {} knows: {:?}", agent, kb.knowledge_state(agent));
        println!("    consistent? {}", kb.consistent(agent));
    }

    // Merge knowledge (colimit: union)
    let merged = kb.merge_knowledge(&["alice", "bob"]);
    println!("\nMerged knowledge (alice + bob): {:?}", merged);

    let all_merged = kb.merge_knowledge(&["alice", "bob", "carol"]);
    println!("Merged knowledge (all three): {:?}", all_merged);

    // Common knowledge (limit: intersection)
    let common = kb.common_knowledge();
    println!("Common knowledge (intersection): {:?}", common);
    println!("  (fact 2 is known by everyone!)");

    // Derived functors
    let id_functor = Functor {
        source: Category::new(vec!["A".into()], vec![]),
        target: Category::new(vec!["A".into()], vec![]),
        object_map: HashMap::from([("A".into(), "A".into())]),
        morphism_map: HashMap::new(),
    };
    let derived = DerivedFunctor {
        functor: id_functor,
        derived_order: 0,
    };
    println!("\nDerived functor L₀F:");
    println!("  L₀F ≅ F? {}", derived.zeroth_is_original());

    let resolution = ChainComplex::new(
        vec![vec![1.0], vec![1.0, 0.0], vec![0.0]],
        vec![DenseMatrix::new(vec![vec![1.0, 0.0]])],
    );
    let ln = derived.compute_ln("A", &resolution);
    println!("  L₀(A) via resolution: {:?}", ln);
    println!();
}

fn main() {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║  Derived Topos — Interactive Tutorial              ║");
    println!("║  From categories to topos theory & agent knowledge ║");
    println!("╚════════════════════════════════════════════════════╝\n");

    lesson_1_categories();
    lesson_2_limits();
    lesson_3_functors();
    lesson_4_natural_transformations();
    lesson_5_sheaves();
    lesson_6_chain_complexes();
    lesson_7_topos_and_logic();
    lesson_8_agent_knowledge();

    println!("═════════════════════════════════════════════════");
    println!("  ✓ Tutorial complete — all 8 lessons done!");
    println!("═════════════════════════════════════════════════");
}
