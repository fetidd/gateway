use super::*;
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Default)]
pub struct TransactionBuilder<T, A, P, Acc, M, B, C> {
    transaction_type: Option<TransactionType>,
    amount: Option<Amount>,
    payment: Option<Payment>,
    billing: Option<Billing>,
    merchant: Option<Merchant>,
    account: Option<Account>,
    customer: Option<Customer>,
    currency: Option<Currency>,
    _t: PhantomData<T>,
    _a: PhantomData<A>,
    _p: PhantomData<P>,
    _acc: PhantomData<Acc>,
    _m: PhantomData<M>,
    _b: PhantomData<B>,
    _c: PhantomData<C>,
}

impl TransactionBuilder<NoType, NoAmount, NoPayment, NoAccount, NoMerchant, NoBilling, NoCurrency> {
    pub fn new(
    ) -> TransactionBuilder<NoType, NoAmount, NoPayment, NoAccount, NoMerchant, NoBilling, NoCurrency>
    {
        TransactionBuilder::default()
    }
}

#[derive(Default)]
pub struct HasType;
#[derive(Default)]
pub struct NoType;
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
#[derive(Default)]
pub struct HasCurrency;
#[derive(Default)]
pub struct NoCurrency;

impl
    TransactionBuilder<
        HasType,
        HasAmount,
        HasPayment,
        HasAccount,
        HasMerchant,
        HasBilling,
        HasCurrency,
    >
{
    pub fn build(self) -> Transaction {
        Transaction {
            r#type: self.transaction_type.unwrap(),
            amount: self.amount.unwrap(),
            payment: self.payment.unwrap(),
            billing: self.billing.unwrap(),
            merchant: self.merchant.unwrap(),
            account: self.account.unwrap(),
            customer: self.customer,
            status: TransactionStatus::Success,
            reference: Uuid::new_v4().to_string(),
            currency: self.currency.unwrap(),
        }
    }
}

impl<T: Default, A: Default, P: Default, Acc: Default, M: Default, B: Default, C: Default>
    TransactionBuilder<T, A, P, Acc, M, B, C>
{
    pub fn transaction_type(
        self,
        t_type: TransactionType,
    ) -> TransactionBuilder<HasType, A, P, Acc, M, B, C> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: Some(t_type),
            account: self.account,
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            currency: self.currency,
            ..Default::default()
        }
    }

    pub fn amount<I: Into<Amount>>(
        self,
        amount: I,
    ) -> TransactionBuilder<T, HasAmount, P, Acc, M, B, C> {
        TransactionBuilder {
            amount: Some(amount.into()),
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            currency: self.currency,
            ..Default::default()
        }
    }

    pub fn payment(self, payment: Payment) -> TransactionBuilder<T, A, HasPayment, Acc, M, B, C> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: Some(payment),
            currency: self.currency,
            ..Default::default()
        }
    }

    pub fn account(self, account: Account) -> TransactionBuilder<T, A, P, HasAccount, M, B, C> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: Some(account),
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            currency: self.currency,
            ..Default::default()
        }
    }

    pub fn merchant(
        self,
        merchant: Merchant,
    ) -> TransactionBuilder<T, A, P, Acc, HasMerchant, B, C> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: Some(merchant),
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            currency: self.currency,
            ..Default::default()
        }
    }

    pub fn billing(self, billing: Billing) -> TransactionBuilder<T, A, P, Acc, M, HasBilling, C> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: Some(billing),
            customer: self.customer,
            payment: self.payment,
            currency: self.currency,
            ..Default::default()
        }
    }

    pub fn currency(
        self,
        currency: Currency,
    ) -> TransactionBuilder<T, A, P, Acc, M, B, HasCurrency> {
        TransactionBuilder {
            amount: self.amount,
            transaction_type: self.transaction_type,
            account: self.account,
            merchant: self.merchant,
            billing: self.billing,
            customer: self.customer,
            payment: self.payment,
            currency: Some(currency),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{account::BankOneAccount, card_scheme::CardScheme, currency::Currency};

    use super::*;
    use rstest::*;

    #[rstest]
    fn build_test_1() {
        let acct = Account::BankOne(BankOneAccount {
            merchant_identification_value: "12345678".into(),
        });
        let mer = Merchant::default();
        let card = Payment::Card {
            scheme: CardScheme::Visa,
            expiry_date: (2021, 3),
            pan: "4000111122223333".into(),
            security_code: "123".into(),
        };
        let billing = Billing::default();
        let trx = {
            let trx = TransactionBuilder::new()
                .transaction_type(TransactionType::Auth)
                .amount(12345)
                .payment(card)
                .account(acct)
                .billing(billing)
                .currency(Currency::GBP)
                .merchant(mer);
            trx.build()
        };
        assert_eq!(
            trx,
            Transaction {
                amount: Amount::Base {
                    val: 12345,
                    cur: Currency::GBP,
                },
                payment: Payment::Card {
                    scheme: CardScheme::Visa,
                    expiry_date: (2021, 3),
                    security_code: "123".into(),
                    pan: "4000111122223333".into()
                },
                r#type: TransactionType::Auth,
                billing: Billing::default(),
                merchant: Merchant::default(),
                account: Account::BankOne(BankOneAccount {
                    merchant_identification_value: "12345678".into()
                }),
                customer: None,
                status: TransactionStatus::Success,
                reference: trx.reference.clone(),
                currency: Currency::GBP
            }
        )
    }
}
