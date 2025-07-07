use crate::indexing::HasTable;
use crate::indexing::IndexLookup;
use crate::indexing::IndexMultiLookup;

pub type DynLookup<V> = dyn IndexLookup<Record = V>;
pub type DynMultiLookup<V> = dyn IndexMultiLookup<Record = V>;

/// A composable, recursive query structure used to express logical operations over indexed fields.
///
/// This enum represents a logical tree of operations that can be evaluated against a record table,
/// using field-level indexes to retrieve matches. Queries can be composed using boolean logic and 
/// evaluated in a streaming or in-memory context.
///
/// # Examples
///
/// ```rust
/// use atlatl::querying::Query;
/// use atlatl::indexing::{Habitat, Species};
///
/// // Simple indexed lookups
/// let habitat_query = Query::lookup(Habitat("Tide Pool".to_string()));
/// let species_query = Query::lookup(Species("Mantis Shrimp".to_string()));
///
/// // A composite query: find records in a Tide Pool habitat AND NOT Mantis Shrimp
/// let complex_query = Query::and(habitat_query, Query::not(species_query));
/// ```
pub enum Query<V: HasTable> {
    // Atomic lookup -------------------------------------------------------------------------------

    /// Lookup starts a query by matching an index (e.g., Habitat("Tide Pool")), returning a KeySet
    /// of keys.
    ///
    /// A base query that performs a direct lookup using a field index.
    ///
    /// This is the atomic unit of a query: a single field match against an index. It may correspond
    /// to a unique or non-unique key. All higher-order queries are composed from these.
    ///
    /// Values must implement the [`IndexLookup`] trait and will be boxed at runtime.
    ///
    /// Note: this operation returns the *set of matching record keys*, not the record itself.
    Lookup(Box<DynLookup<V>>),

    /// Performs a logical `NOT` over a subquery.
    ///
    /// All records **not** matching the subquery will be returned.
    ///
    /// This is equivalent to set negation or exclusion.
    Not(Box<DynLookup<V>>),

    // Binary set operations -----------------------------------------------------------------------

    /// Performs a logical `AND` between two subqueries.
    ///
    /// Only records that satisfy *both* subqueries will be matched.
    ///
    /// This is equivalent to set intersection in terms of result evaluation.
    ///
    /// # Performance
    ///
    /// Prefer placing the more selective query on the left-hand side, as it will be evaluated 
    /// first.
    And(Box<Query<V>>, Box<DynLookup<V>>),

    /// Set difference (e.g., Tide Pool WITHOUT Mantis Shrimp).
    Difference(Box<Query<V>>, Box<DynLookup<V>>),

    /// Performs a logical `OR` between two subqueries.
    ///
    /// Records matching *either* query will be returned.
    ///
    /// This is equivalent to set union in terms of result evaluation.
    Or(Box<Query<V>>, Box<DynLookup<V>>),

    /// Performs a logical `XOR` between two subqueries.
    Xor(Box<Query<V>>, Box<DynLookup<V>>),

    // Grouping ------------------------------------------------------------------------------------

    /// Groups a query for precedence control during recursive traversal.
    ///
    /// Though functionally identical to the inner query, this variant allows explicit grouping in 
    /// visual renderings, logical simplification, and future optimizations.
    ///
    /// # Example
    ///
    /// This makes sense when building visual or human-readable representations:
    ///
    /// ```text
    /// A AND (B OR C)
    /// ```
    Group(Box<Query<V>>),

    // Multi-value lookups -------------------------------------------------------------------------

    /// Internal: Multi-value IN lookup (e.g., Species IN ["Clownfish", "Parrotfish"]). Use
    /// `Query::any_of`.
    AnyOf(Box<DynMultiLookup<V>>),

    /// Internal: Multi-value NOT IN lookup. Use `Query::not_in`.
    NotIn(Box<DynMultiLookup<V>>),

    // Custom predicate ----------------------------------------------------------------------------

    /// A custom predicate-based query over records.
    ///
    /// This variant is only available if the `custom-queries` feature is enabled. It allows 
    /// arbitrary logic over deserialized records and is useful when indexes cannot express the full
    /// query.
    ///
    /// **Warning**: This query cannot be accelerated by index traversal and will be applied *after* 
    /// indexed filtering or on a full scan. Use with care.
    ///
    /// # Example
    ///
    /// ```rust
    /// let predicate = |record: &User| record.age > 30 && record.name.starts_with("A");
    /// let custom_query = Query::Custom(predicate);
    /// ```    
    #[cfg(feature = "custom-queries")]
    Custom(fn(&V) -> bool),
}

impl<T, V> From<T> for Query<V>
where
    T: IndexLookup<Record = V> + 'static,
    V: HasTable,
{
    fn from(index_lookup: T) -> Self {
        Self::Lookup(Box::new(index_lookup))
    }
}
/*
impl<V: HasTable> std::fmt::Display for Query<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::Lookup(index) => write!(f, "{:?}", index),
            Query::And(lhs, rhs) => write!(f, "({} AND {:?})", lhs, rhs),
            Query::Or(lhs, rhs) => write!(f, "({} OR {:?})", lhs, rhs),
            Query::Xor(lhs, rhs) => write!(f, "({} XOR {:?})", lhs, rhs),
            Query::Difference(lhs, rhs) => write!(f, "({} WITHOUT {:?})", lhs, rhs),
            Query::Not(inner) => write!(f, "(NOT {:?})", inner),
            Query::Group(inner) => write!(f, "({})", inner),
            Query::AnyOf(multi) => write!(f, "(ANY_OF {:?})", multi),
            Query::NotIn(multi) => write!(f, "(NOT_IN {:?})", multi),
            #[cfg(feature = "custom-queries")]
            Query::Custom(_) => write!(f, "(CUSTOM PREDICATE)"),
        }
    }
}
*/

/// A composable query expression tree for filtering records of type `V`.
///
/// This enum represents logical expressions that can be evaluated to filter records from a table.
/// Queries can be composed using standard logic operators (`AND`, `OR`, `NOT`) as well as grouped
/// for controlling precedence. The base unit is an indexed lookup, allowing queries to operate
/// efficiently using pre-built indices.
impl<V: HasTable> Query<V> {
    // Constructor ---------------------------------------------------------------------------------

    /// Constructs a base query using a single indexed field.
    ///
    /// This is typically the entry point for queries, specifying the initial field lookup. For
    /// example `Habitat("Desert")`, `Species("Mantis Shrimp")`.
    ///
    /// The `IndexLookup` trait defines how to locate the index table and serialize the key.
    fn lookup<I>(index_lookup: I) -> Self 
    where 
        I: IndexLookup<Record = V> + 'static
    {
        index_lookup.into()
    }

    // Chainable binary operations -----------------------------------------------------------------

    /// Combines two queries with a logical `AND`.
    ///
    /// Both `self` and `filter` must evaluate to `true` for the overall query to match a record.
    /// Use this to narrow results by requiring multiple conditions.
    fn and<I>(self, filter: I) -> Self
    where 
        I: IndexLookup<Record = V> + 'static
    {    
        Query::And(Box::new(self), Box::new(filter))
    }    

    /// Combines two queries with a logical `OR`.
    ///
    /// Either `self` or `extender` must evaluate to `true` for the overall query to match a record.
    /// Use this to broaden the result set by accepting multiple possibilities.
    fn or_else<I>(self, extender: I) -> Self
    where
        I: IndexLookup<Record = V> + 'static
    {
        Query::Or(Box::new(self), Box::new(extender))
    }

    /// Excludes records matching `filter` from those matching `self`.
    ///
    /// Only records satisfying `self` but not `filter` are included. Use this to filter out
    /// specific conditions from a broader query result.
    fn without<I>(self, filter: I) -> Self
    where
        I: IndexLookup<Record = V> + 'static
    {
        Query::Difference(Box::new(self), Box::new(filter))
    }

    /// Combines two queries with a logical `XOR`.
    ///
    /// Records matching exactly one of `self` or `filter`, but not both, are included. Use this to
    /// find records exclusive to one condition or the other.
    fn xor<I>(self, filter: I) -> Self
    where
        I: IndexLookup<Record = V> + 'static
    {
        Query::Xor(Box::new(self), Box::new(filter))
    }
}


impl<V: HasTable> Query<V> {
    // Static binary operations --------------------------------------------------------------------

    /// Combines two queries with a logical `AND`.
    ///
    /// Both `a` and `b` must evaluate to `true` for the overall query to match a record. Use this
    /// to narrow results by requiring multiple conditions.
    fn and_with<Q, I>(a: Q, b: I) -> Self
    where
        Q: Into<Query<V>>,
        I: IndexLookup<Record = V> + 'static
    {
        Query::And(Box::new(a.into()), Box::new(b))
    }

    /// Excludes records matching `b` from those matching `a`.
    ///
    /// Only records satisfying `a` but not `b` are included. Use this to filter out specific
    /// conditions from a broader query result.
    fn difference<Q, I>(a: Q, b: I) -> Self
    where
        Q: Into<Query<V>>,
        I: IndexLookup<Record = V> + 'static
    {
        Query::Difference(Box::new(a.into()), Box::new(b))
    }

    /// Combines two queries with a logical `OR`.
    ///
    /// Either `a` or `b` must evaluate to `true` for the overall query to match a record. Use this
    /// to broaden the result set by accepting multiple possibilities.
    fn or_else_with<Q, I>(a: Q, b: I) -> Self
    where
        Q: Into<Query<V>>,
        I: IndexLookup<Record = V> + 'static
    {
        Query::Or(Box::new(a.into()), Box::new(b))
    }

    /// Combines two queries with a logical `XOR`.
    ///
    /// Records matching exactly one of `a` or `b`, but not both, are included. Use this to find
    /// records exclusive to one condition or the other.
    fn xor_with<Q, I>(a: Q, b: I) -> Self
    where
        Q: Into<Query<V>>,
        I: IndexLookup<Record = V> + 'static
    {
        Query::Xor(Box::new(a.into()), Box::new(b))
    }
}

impl<V: HasTable> Query<V> {
    // Unary and grouping --------------------------------------------------------------------------

    /// Negates a table using logical `NOT`.
    ///
    /// If the inner query matches a record, this query will exclude it. Useful for excluding 
    /// specific matches from a broader set.
    fn negate<I>(index_lookup: I) -> Self
    where 
        I: IndexLookup<Record = V> + 'static
    {
        index_lookup.into()
    }

    /// Groups a subquery to explicitly define precedence.
    ///
    /// While the `Query` structure already defines a tree of operations, this method allows 
    /// semantically grouping a subexpression. This is useful for debugging, display, or evaluation 
    /// strategies that care about grouping.
    fn group<Q>(query: Q) -> Self
    where 
        Q: Into<Query<V>>
    {    
        Query::Group(Box::new(query.into()))
    }    

    // Multi-value lookups -------------------------------------------------------------------------

    /// Matches records where a field equals any of multiple values.
    ///
    /// Records with a field value in the provided set (e.g., a list of `Species`) are included.
    /// Use this to query multiple possible matches efficiently.
    pub fn any_of<I, M>(multi: I) -> Self
    where    
        I: Into<M>,
        M: IndexMultiLookup<Record = V> + 'static
    {
        Self::AnyOf(Box::new(multi.into()))
    }

    /// Excludes records where a field equals any of multiple values.
    ///
    /// Records with a field value not in the provided set (e.g., a list of `Species`) are included.
    /// Use this to filter out multiple specific values.
    pub fn not_in<I, M>(multi: I) -> Self
    where    
        I: Into<M>,
        M: IndexMultiLookup<Record = V> + 'static
    {
        Self::NotIn(Box::new(multi.into()))
    }

    // Custom predicate ----------------------------------------------------------------------------

    /// Creates a custom query using a raw function that evaluates a record.
    ///
    /// This allows for arbitrary user-defined logic, typically used when no index is available or 
    /// when a more complex in-memory filter is required.
    ///
    /// This variant bypasses the index and is evaluated *after* data is loaded, so use it sparingly 
    /// for performance-critical paths.
    #[cfg(feature = "custom-queries")]
    fn custom(f: fn(&V) -> bool) -> Self {
        Query::Custom(f)
    }
}

fn example() {
    use crate::indexing::Habitat;
    use crate::indexing::Species;

    let habitat_query = Query::lookup(Habitat("Tide Pool".to_string()));
    let complex_query = Query::and(habitat_query, Species("Mantis Shrimp".to_string()));
} 