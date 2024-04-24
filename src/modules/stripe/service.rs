use std::sync::Arc;

use stripe::{Client, CreateCustomer, CreatePaymentIntent, Currency, Customer, PaymentIntent};

use crate::{
    modules::user::{self, UserError},
    utils::Config,
};

use super::{ports::Repository, Payment, PaymentError, PaymentStatus};

pub struct Service {
    repository: Arc<dyn Repository>,
    user_service: Arc<user::Service>,
    stripe_client: Client,
}
impl Service {
    pub fn new(repository: Arc<dyn Repository>, user_service: Arc<user::Service>) -> Self {
        let config = Config::from_env();
        let stripe_client = Client::new(config.strip_secret);
        Self {
            repository,
            user_service,
            stripe_client,
        }
    }

    pub async fn create_stripe_user(&self, user_id: i32) -> Result<(), PaymentError> {
        //Get user
        let mut user = self
            .user_service
            .get_user_by_id(user_id)
            .await?
            .ok_or(UserError::NotFound)?;

        //Create strip customer
        let customer = Customer::create(
            &self.stripe_client,
            CreateCustomer {
                name: Some(&user.name),
                email: Some(&user.email),
                metadata: Some(std::collections::HashMap::from([(
                    String::from("async-stripe"),
                    String::from("true"),
                )])),

                ..Default::default()
            },
        )
        .await?;

        //Update customer with stripe customer id
        user.stripe_customer_id = Some(customer.id.to_string());
        self.user_service.update_user(&user).await?;

        Ok(())
    }

    pub async fn create_payment_intent(
        &self,
        user_id: i32,
        product_id: i32,
    ) -> Result<Payment, PaymentError> {
        //agregar traer producto
        let payment_intent = {
            let mut create_intent = CreatePaymentIntent::new(1000, Currency::USD);
            create_intent.payment_method_types = Some(vec!["card".to_string()]);

            PaymentIntent::create(&self.stripe_client, create_intent).await?
        };
        let payment = Payment::new(user_id, &payment_intent.id.to_string());
        self.repository.create_payment(&payment).await
    }

    pub async fn payment_status(&self, user_id: i32) -> Result<PaymentStatus, PaymentError> {
        match self.repository.get_payment_by_user(user_id).await {
            Ok(Some(payment)) => Ok(payment.payment_status),
            Ok(None) => Err(PaymentError::NotFound),
            Err(e) => Err(e),
        }
    }

    pub async fn update_payment_status(
        &self,
        stripe_payment_id: &str,
        new_status: PaymentStatus,
    ) -> Result<(), PaymentError> {
        self.repository
            .update_payment_status(stripe_payment_id, new_status)
            .await
    }
}
