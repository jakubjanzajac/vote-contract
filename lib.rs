#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod voter {
    use ink_storage::Mapping;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Voter {
        voted_received: Mapping<u128, u128>,
        feedbacks: Vec<String>
    }

    impl Voter {
        #[ink(constructor)]
        pub fn default() -> Self {
            // Self { feedbacks: Vec::new(), voted_received: Mapping::default() }
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.voted_received = Mapping::default();
                contract.feedbacks = Vec::new();
            })
        }

        #[ink(message)]
        pub fn add_feedback(&mut self, feedback: String) {
            self.feedbacks.push(feedback);
        }

        #[ink(message)]
        pub fn add_vote(&mut self, feedback_index: u128) {
            let mut votes = self.voted_received.get(feedback_index).unwrap_or_default();
            votes = votes + 1;
            self.voted_received.insert(feedback_index, &votes);
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let voter = Voter::default();
            assert_eq!(voter.feedbacks.len(), 0);
        }

        #[ink::test]
        fn add_feedback() {
            let mut voter = Voter::default();
            voter.add_feedback(String::from("test feedback"));
            assert_eq!(voter.feedbacks.len(), 1);
        }

        #[ink::test]
        fn add_vote() {
            let mut voter = Voter::default();
            voter.add_feedback(String::from("test feedback"));
            voter.add_vote(0);
            let feedback_votes = voter.voted_received.get(0).unwrap();
            assert_eq!(feedback_votes, 1);
        }
    }
}
