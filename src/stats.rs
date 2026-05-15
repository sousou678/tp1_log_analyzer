use crate::parser::FailedLogin;
use std::collections::HashMap;

pub fn count_by_ip(events: &[FailedLogin]) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    for event in events {
        *counts.entry(event.ip.clone()).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1)); // Trier par score descendant
    sorted
}

pub fn count_by_user(events: &[FailedLogin]) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    for event in events {
        *counts.entry(event.user.clone()).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}
