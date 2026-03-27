#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, Address};

    fn setup() -> (Env, Address, Address) {
        let env = Env::default();
        let owner = Address::generate(&env);
        let user = Address::generate(&env);

        (env, owner, user)
    }

    #[test]
    fn test_init() {
        let (env, owner, _) = setup();

        PiggyBank::init(env.clone(), owner.clone());

        let stored: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .unwrap();

        assert_eq!(stored, owner);
    }

    #[test]
    #[should_panic]
    fn test_double_init() {
        let (env, owner, _) = setup();

        PiggyBank::init(env.clone(), owner.clone());
        PiggyBank::init(env.clone(), owner.clone());
    }

    #[test]
    fn test_deposit() {
        let (env, owner, user) = setup();

        PiggyBank::init(env.clone(), owner);

        PiggyBank::deposit(env.clone(), user.clone(), 100);

        let bal = PiggyBank::get_balance(env.clone());
        assert_eq!(bal, 100);
    }

    #[test]
    fn test_multiple_deposit() {
        let (env, owner, user) = setup();

        PiggyBank::init(env.clone(), owner);

        PiggyBank::deposit(env.clone(), user.clone(), 100);
        PiggyBank::deposit(env.clone(), user.clone(), 50);

        let bal = PiggyBank::get_balance(env.clone());
        assert_eq!(bal, 150);
    }

    #[test]
    fn test_withdraw_by_owner() {
        let (env, owner, user) = setup();

        PiggyBank::init(env.clone(), owner.clone());

        PiggyBank::deposit(env.clone(), user, 100);

        PiggyBank::withdraw(env.clone(), owner.clone(), 40);

        let bal = PiggyBank::get_balance(env.clone());
        assert_eq!(bal, 60);
    }

    #[test]
    #[should_panic]
    fn test_withdraw_insufficient() {
        let (env, owner, user) = setup();

        PiggyBank::init(env.clone(), owner.clone());

        PiggyBank::deposit(env.clone(), user, 50);

        PiggyBank::withdraw(env.clone(), owner, 100);
    }

    #[test]
    #[should_panic]
    fn test_withdraw_not_owner() {
        let (env, owner, user) = setup();

        PiggyBank::init(env.clone(), owner);

        PiggyBank::deposit(env.clone(), user.clone(), 100);

        PiggyBank::withdraw(env.clone(), user, 50);
    }
}