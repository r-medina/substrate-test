#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::collections::HashMap;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        numbers: HashMap<AccountId, u32>, // <---------- where does AccountId come from???
    }

    impl Incrementer {
        /// Constructor that allocates the storage.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                numbers: Default::default(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// A message that can be called on instantiated contracts.
        /// This one increases the value of the stored
        #[ink(message)]
        pub fn increment(&mut self, account_id: AccountId) {
            let num = self.numbers.get(&account_id).cloned().unwrap_or(0);
            self.numbers.insert(account_id, num + 1);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self, account_id: AccountId) -> u32 {
            *self.numbers.get(&account_id).unwrap_or(&0)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        // use super::*;
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            // let incrementer = Incrementer::default();
            // let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // incrementer.increment(account_id);
            // assert_eq!(incrementer.get(), 1);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            // let mut incrementer = Incrementer::new();
            // assert_eq!(incrementer.get(), 1);
            // assert_eq!(incrementer.get(), 1);
        }
    }
}
