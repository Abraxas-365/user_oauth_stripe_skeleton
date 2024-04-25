use std::sync::Arc;

use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, Currency, Customer, CustomerId, IdOrCreate,
    Price, Product, ProductId,
};

use crate::{
    modules::user::{self, User, UserError},
    utils::Config,
};

use super::{ports::Repository, Payment, PaymentError, PaymentStatus};

pub struct Service {
    repository: Arc<dyn Repository>,
    user_service: Arc<user::Service>,
    stripe_client: Client,
    config: Config,
}

impl Service {
    pub fn new(repository: Arc<dyn Repository>, user_service: Arc<user::Service>) -> Self {
        let config = Config::from_env();
        let stripe_client = Client::new(&config.strip_secret);
        Self {
            repository,
            user_service,
            stripe_client,
            config,
        }
    }
}

//Product stripe
impl Service {
    pub async fn get_product(&self, product_id: &str) -> Result<Product, PaymentError> {
        let product_id = product_id.parse::<ProductId>()?;
        let product = Product::retrieve(&self.stripe_client, &product_id, &[]).await?;
        Ok(product)
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, PaymentError> {
        let products = Product::list(&self.stripe_client, &Default::default()).await?;
        Ok(products.data)
    }

    pub async fn get_product_price(&self, product_id: &str) -> Result<Price, PaymentError> {
        let product_id = product_id.parse::<ProductId>()?;
        let price = stripe::Price::list(
            &self.stripe_client,
            &stripe::ListPrices {
                product: Some(IdOrCreate::Id(&product_id)),
                currency: Some(Currency::USD),
                active: Some(true),
                ..Default::default()
            },
        )
        .await?
        .data
        .pop()
        .ok_or(PaymentError::NotFound)?;
        Ok(price)
    }
}

//Customer stripe
impl Service {
    pub async fn get_customer(&self, user: &User) -> Result<Customer, PaymentError> {
        if let Some(stripe_customer_id) = &user.stripe_customer_id {
            let stripe_customer_id = stripe_customer_id.parse::<CustomerId>()?;
            let customer = Customer::retrieve(&self.stripe_client, &stripe_customer_id, &[])
                .await
                .map_err(|_| PaymentError::NotFound)?;
            return Ok(customer);
        }

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

        //save user with new customer id
        let updated_user = User {
            stripe_customer_id: Some(customer.id.to_string()),
            ..user.clone()
        };
        self.user_service.update_user(&updated_user).await?;

        Ok(customer)
    }
}

//Checkout stripe
impl Service {
    pub async fn create_checkout(
        &self,
        user_id: i32,
        product_id: &str,
    ) -> Result<String, PaymentError> {
        let user = self
            .user_service
            .get_user_by_id(user_id)
            .await?
            .ok_or(UserError::NotFound)?;

        let customer = self.get_customer(&user).await?;

        let price = self.get_product_price(product_id).await?;

        let checkout_session = {
            let mut params = CreateCheckoutSession::new();
            params.cancel_url = Some(&self.config.stripe_checkout_cancel_url);
            params.success_url = Some(&self.config.stripe_checkout_success_url);
            params.customer = Some(customer.id);
            params.mode = Some(CheckoutSessionMode::Payment);
            params.line_items = Some(vec![CreateCheckoutSessionLineItems {
                quantity: Some(1),
                price: Some(price.id.to_string()),
                ..Default::default()
            }]);
            params.expand = &[];

            CheckoutSession::create(&self.stripe_client, params).await?
        };

        checkout_session
            .url
            .ok_or(PaymentError::CreateCheckoutError)
    }

    pub async fn get_checkout_session_by_id(
        &self,
        session_id: &str,
    ) -> Result<CheckoutSession, PaymentError> {
        let session_id = session_id.parse::<stripe::CheckoutSessionId>()?;
        let session = CheckoutSession::retrieve(
            &self.stripe_client,
            &session_id,
            &[
                "line_items",
                "line_items.data.price.product",
                "payment_intent",
            ],
        )
        .await?;
        Ok(session)
    }
}

//Payment
impl Service {
    pub async fn create_payment(&self, checkout_session_id: &str) -> Result<(), PaymentError> {
        let checkout_session = self.get_checkout_session_by_id(checkout_session_id).await?;

        let payment_intent = checkout_session
            .payment_intent
            .as_ref()
            .ok_or(PaymentError::ItemNotFound)?
            .id();

        let line_items = checkout_session
            .line_items
            .clone()
            .data
            .get(0)
            .ok_or(PaymentError::ItemNotFound)?
            .clone();

        let product_id = line_items
            .price
            .ok_or(PaymentError::ItemNotFound)?
            .product
            .ok_or(PaymentError::ItemNotFound)?
            .id();

        let customer_id = checkout_session
            .customer
            .ok_or(PaymentError::ItemNotFound)?
            .id();

        let user = self
            .user_service
            .get_user_by_customer_id(&customer_id)
            .await?
            .ok_or(UserError::NotFound)?;

        let pament = Payment::new(user.id, payment_intent.as_str(), product_id.as_str());

        self.repository.create_payment(&pament).await?;

        Ok(())
    }

    pub async fn update_payment_status(
        &self,
        checkout_session_id: &str,
        status: PaymentStatus,
    ) -> Result<(), PaymentError> {
        let checkout_session = self.get_checkout_session_by_id(checkout_session_id).await?;

        let payment_intent = checkout_session
            .payment_intent
            .as_ref()
            .ok_or(PaymentError::ItemNotFound)?
            .id();

        self.repository
            .update_payment_status(payment_intent.as_str(), status)
            .await?;
        Ok(())
    }
}
