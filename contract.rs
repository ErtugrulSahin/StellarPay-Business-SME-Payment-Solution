use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct PaymentPlatform;

#[contracttype]
pub struct Invoice {
    pub business: Address,
    pub client: Address,
    pub amount: i128,
    pub currency: Symbol,
    pub status: Symbol, // "pending", "paid"
}

#[contractimpl]
impl PaymentPlatform {
    fn invoices<'a>(env: &'a Env) -> Vec<'a, Invoice> {
        env.storage().instance().get::<Vec<Invoice>>(Symbol::short("invoices")).unwrap_or(Vec::new(&env))
    }

    pub fn create_invoice(env: Env, client: Address, amount: i128, currency: Symbol) {
        let business = env.invoker();
        let mut invoices = Self::invoices(&env);
        invoices.push_back(Invoice { business, client, amount, currency, status: Symbol::short("pending") });
        env.storage().instance().set(Symbol::short("invoices"), &invoices);
    }

    pub fn pay_invoice(env: Env, index: u32) {
        let client = env.invoker();
        let mut invoices = Self::invoices(&env);
        let invoice = &mut invoices[index as usize];
        assert_eq!(invoice.client, client, "Only client can pay");
        invoice.status = Symbol::short("paid");
        env.storage().instance().set(Symbol::short("invoices"), &invoices);
    }
}
