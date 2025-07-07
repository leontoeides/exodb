#[derive(Clone, Debug, Default)]
pub struct Metadata {
	pub corrector: Option<crate::layers::correctors::Metadata>,
}