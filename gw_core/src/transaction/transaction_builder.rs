use std::marker::PhantomData;

use super::*;

/// Anything optional here but not in Transaction will be required when `build` is called.
#[derive(Default)]
pub struct TransactionBuilder<A, P, Acc, M, B> {
    transaction_type: Option<TransactionType>,
    amount: Option<Amount>,
    payment: Option<Payment>,
    billing: Option<Billing>,
    merchant: Option<Merchant>,
    account: Option<Account>,
    customer: Option<Customer>,
    _a: PhantomData<A>,
    _p: PhantomData<P>,
    _acc: PhantomData<Acc>,
    _m: PhantomData<M>,
    _b: PhantomData<B>,
}

impl TransactionBuilder<NoAmount, NoPayment, NoAccount, NoMerchant, NoBilling> {
    pub fn auth() -> TransactionBuilder<NoAmount, NoPayment, NoAccount, NoMerchant, NoBilling> {
        TransactionBuilder {
            transaction_type: Some(TransactionType::Auth),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct HasAmount;
#[derive(Default)]
pub struct NoAmount;
#[derive(Default)]
pub struct HasPayment;
#[derive(Default)]
pub struct NoPayment;
#[derive(Default)]
pub struct HasAccount;
#[derive(Default)]
pub struct NoAccount;
#[derive(Default)]
pub struct HasMerchant;
#[derive(Default)]
pub struct NoMerchant;
#[derive(Default)]
pub struct HasBilling;
#[derive(Default)]
pub struct NoBilling;

//------------------------------------------------

impl TransactionBuilder<HasAmount, HasPayment, HasAccount, HasMerchant, HasBilling> {
    pub fn build(self) -> Transaction {
        Transaction {
            r#type: self.transaction_type.unwrap(),
            amount: self.amount.unwrap(),
            payment: self.payment.unwrap(),
            billing: self.billing.unwrap(),
            merchant: self.merchant.unwrap(),
            account: self.account.unwrap(),
            customer: self.customer,
        }
    }
}

impl<A: Default, P: Default, Acc: Default, M: Default, B: Default>
    TransactionBuilder<A, P, Acc, M, B>
{
    pub fn amount<T: Into<Amount>>(self, amount: T) -> TransactionBuilder<HasAmount, P, Acc, M, B> {
        TransactionBuilder {
            amount: Some(amount.into()),
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            ..Default::default()
        }
    }

    pub fn payment(self, payment: Payment) -> TransactionBuilder<A, HasPayment, Acc, M, B> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: Some(payment),
            ..Default::default()
        }
    }

    pub fn account(self, account: Account) -> TransactionBuilder<A, P, HasAccount, M, B> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: Some(account),
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            ..Default::default()
        }
    }

    pub fn merchant(self, merchant: Merchant) -> TransactionBuilder<A, P, Acc, HasMerchant, B> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: Some(merchant),
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            ..Default::default()
        }
    }

    pub fn billing(self, billing: Billing) -> TransactionBuilder<A, P, Acc, M, HasBilling> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: Some(billing),
            customer: self.customer,
            payment: self.payment,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card_scheme::CardScheme;

    use super::*;
    use rstest::*;

    #[rstest]
    fn build_test_1() {
        let acct = Account::BankA {};
        let mer = Merchant {};
        let card = Payment::Card {
            scheme: CardScheme::Visa,
            expiry_date: (2021, 3),
            pan: "4000111122223333".into(),
            security_code: "123".into(),
        };
        let billing = Billing::default();
        let trx = TransactionBuilder::auth()
            .amount(12345)
            .payment(card)
            .account(acct)
            .billing(billing)
            .merchant(mer);
        let trx = trx.build();
        assert_eq!(
            trx,
            Transaction {
                amount: Amount::Base {
                    val: 12345,
                    cur: crate::currency::Currency::GBP,
                },
                payment: Payment::Card {
                    scheme: CardScheme::Visa,
                    expiry_date: (2021, 3),
                    security_code: "123".into(),
                    pan: "4000111122223333".into()
                },
                r#type: TransactionType::Auth,
                billing: Billing::default(),
                merchant: Merchant {},
                account: Account::BankA {},
                customer: None
            }
        )
    }
}
