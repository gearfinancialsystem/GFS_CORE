use std::collections::HashMap;
use crate::attributes::ContractReference::Object;
use crate::util_tests::ObservedDataSet::ObservedDataSet;
use crate::util_tests::ResultSet::ResultSet;

#[derive(PartialEq, Debug, Clone)]
pub struct TestData {
    identifier: String,
    terms: HashMap<String, Object>,
    to: String,
    data_observed: HashMap<String, ObservedDataSet>,
    events_observed: ObservedDataSet,
    results: Vec<ResultSet>
}

impl TestData {
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }
    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = identifier;
    }
    pub fn get_terms(&self) -> &HashMap<String, Object> {
        &self.terms
    }
    pub fn set_terms(&mut self, terms: HashMap<String, Object>) {
        self.terms = terms;
    }
    pub fn get_to(&self) -> String {
        self.to.clone()
    }
    pub fn set_to(&mut self, to: String) {
        self.to = to;
    }
    pub fn get_data_observed(&self) -> &HashMap<String, ObservedDataSet> {
        &self.data_observed
    }
    pub fn set_data_observed(&mut self, data_observed: HashMap<String, ObservedDataSet>) {
        self.data_observed = data_observed;
    }
    pub fn get_events_observed(&self) -> &ObservedDataSet {
        &self.events_observed
    }
    pub fn set_events_observed(&mut self, events_observed: ObservedDataSet) {
        self.events_observed = events_observed;
    }
    pub fn get_results(&self) -> &Vec<ResultSet> {
        &self.results
    }
    pub fn set_results(&mut self, results: Vec<ResultSet>) {
        self.results = results;
    }
}