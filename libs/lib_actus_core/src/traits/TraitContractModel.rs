
pub trait TraitContractModel {
    /// Access a contract attribute as a reference to `Any`
    ///
    /// The caller can downcast the returned reference to the appropriate cont_type.
    ///
    /// # Parameters
    /// - `name`: The name of the attribute to retrieve.
    ///
    /// # Returns
    /// A reference to `Any`, allowing for downcasting by the caller.
    ///
    /// # Panics
    /// This method will panic if the attribute is not found.
    fn cm(&self) -> String;

    
    //fn get_as(&self, name: &str) -> Option<&dyn Any>;
}

