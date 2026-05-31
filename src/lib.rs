pub mod types;
pub mod category;
pub mod morphism;
pub mod functor;
pub mod natural_transformation;
pub mod subobject_classifier;
pub mod topos;
pub mod internal_logic;
pub mod sheaf;
pub mod chain_complex;
pub mod dense_matrix;
pub mod derived_functor;
pub mod agent_knowledge;

// Re-export key types for convenience (avoid ambiguous glob re-exports)
pub use category::Category;
pub use morphism::Morphism;
pub use functor::Functor;
pub use natural_transformation::NaturalTransformation;
pub use subobject_classifier::SubobjectClassifier;
pub use topos::Topos;
pub use internal_logic::InternalLogic;
pub use sheaf::Sheaf;
pub use chain_complex::ChainComplex;
pub use dense_matrix::DenseMatrix;
pub use derived_functor::DerivedFunctor;
pub use agent_knowledge::AgentKnowledgeBase;
pub use types::MorphismData;
