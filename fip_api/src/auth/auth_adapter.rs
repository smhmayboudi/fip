pub struct Adapter {}

/// TODO: documentation
impl Adapter {
    // Model represents the whole access control model.
    type Model map[string]AssertionMap

    // AssertionMap is the collection of assertions, can be "r", "p", "g", "e", "m".
    type AssertionMap map[string]*Assertion

    /// load_policy loads all policy rules from the storage.
/// TODO: documentation
    pub fn load_policy(&self, model:model.Model)-> Rsult<(),std::error::Error> {
        return errors.New("not implemented")
    }

    /// save_policy saves all policy rules to the storage.
/// TODO: documentation
    pub fn save_policy(&self, model:model.Model)-> Rsult<(),std::error::Error> {
        return errors.New("not implemented")
    }

    /// add_policy adds a policy rule to the storage.
/// TODO: documentation
    pub fn add_policy(&self, sec:String, ptype:String, rule: [String])-> Rsult<(),std::error::Error> {
        return errors.New("not implemented")
    }

    /// remove_policy removes a policy rule from the storage.
/// TODO: documentation
    pub fn remove_policy(&self, sec:String, ptype:String, rule:[String])-> Rsult<(),std::error::Error> {
        return errors.New("not implemented")
    }

    /// remove_filtered_policy removes policy rules that match the filter from the storage.
/// TODO: documentation
    pub fn remove_filtered_policy(&self, sec:String, ptype:String, fieldIndex:u32, fieldValues:...String)-> Rsult<(),std::error::Error> {
        return errors.New("not implemented")
    }
}
