
use ::attribute::BaseAttribute;

pub trait ProvideBaseAttributes {
    fn base_attributes() -> Vec<BaseAttribute>;
}
