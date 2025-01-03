/// A marker for generic types that specifies a variant of something being not specified.
///
/// This struct is mostly used with typed builder pattern.
///
/// # Examples
///
/// ```rust
/// use mavio::protocol::Unset;
///
/// trait MaybeSomething {}
///
/// struct Something {
///     /* fields */
/// }
///
/// impl MaybeSomething for Something {}
///
/// impl MaybeSomething for Unset {}
///
/// struct Container<T: MaybeSomething> (T);
///
/// impl Container<Unset> {
///     fn new() -> Self {
///         Self(Unset)
///     }
///
///     fn add_something(something: Something) -> Container<Something> {
///         Container(something)
///     }
/// }
///
/// impl Container<Something> {
///     fn do_something(&self) {
///         /* logic specific to something */
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unset;
