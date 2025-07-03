use crate::events::AnyContractEvent::AnyContractEvent;

pub trait TraitConvertContractToAnyEvent {
    fn convert_to_any(self) -> Result<AnyContractEvent, String>;
}