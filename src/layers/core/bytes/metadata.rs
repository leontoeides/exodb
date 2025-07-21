#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Metadata {
	pub corrector: Option<crate::layers::correctors::Metadata>,
}