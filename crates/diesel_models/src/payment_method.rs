use std::collections::HashMap;

use common_enums::MerchantStorageScheme;
use common_utils::{encryption::Encryption, pii};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
use masking::Secret;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
use crate::{enums as storage_enums, schema::payment_methods};
#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
use crate::{enums as storage_enums, schema_v2::payment_methods};

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
#[derive(
    Clone, Debug, Eq, PartialEq, Identifiable, Queryable, Selectable, Serialize, Deserialize,
)]
#[diesel(table_name = payment_methods, primary_key(payment_method_id), check_for_backend(diesel::pg::Pg))]
pub struct PaymentMethod {
    pub customer_id: common_utils::id_type::CustomerId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub payment_method_id: String,
    #[diesel(deserialize_as = super::OptionalDieselArray<storage_enums::Currency>)]
    pub accepted_currency: Option<Vec<storage_enums::Currency>>,
    pub scheme: Option<String>,
    pub token: Option<String>,
    pub cardholder_name: Option<Secret<String>>,
    pub issuer_name: Option<String>,
    pub issuer_country: Option<String>,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub payer_country: Option<Vec<String>>,
    pub is_stored: Option<bool>,
    pub swift_code: Option<String>,
    pub direct_debit_token: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub last_modified: PrimitiveDateTime,
    pub payment_method: Option<storage_enums::PaymentMethod>,
    pub payment_method_type: Option<storage_enums::PaymentMethodType>,
    pub payment_method_issuer: Option<String>,
    pub payment_method_issuer_code: Option<storage_enums::PaymentMethodIssuerCode>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub payment_method_data: Option<Encryption>,
    pub locker_id: Option<String>,
    pub last_used_at: PrimitiveDateTime,
    pub connector_mandate_details: Option<serde_json::Value>,
    pub customer_acceptance: Option<pii::SecretSerdeValue>,
    pub status: storage_enums::PaymentMethodStatus,
    pub network_transaction_id: Option<String>,
    pub client_secret: Option<String>,
    pub payment_method_billing_address: Option<Encryption>,
    pub updated_by: Option<String>,
    pub version: common_enums::ApiVersion,
    pub network_token_requestor_reference_id: Option<String>,
    pub network_token_locker_id: Option<String>,
    pub network_token_payment_method_data: Option<Encryption>,
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
#[derive(Clone, Debug, Identifiable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = payment_methods, primary_key(id), check_for_backend(diesel::pg::Pg))]
pub struct PaymentMethod {
    pub customer_id: common_utils::id_type::CustomerId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub created_at: PrimitiveDateTime,
    pub last_modified: PrimitiveDateTime,
    pub payment_method_data: Option<Encryption>,
    pub locker_id: Option<String>,
    pub last_used_at: PrimitiveDateTime,
    pub connector_mandate_details: Option<PaymentsMandateReference>,
    pub customer_acceptance: Option<pii::SecretSerdeValue>,
    pub status: storage_enums::PaymentMethodStatus,
    pub network_transaction_id: Option<String>,
    pub client_secret: Option<String>,
    pub payment_method_billing_address: Option<Encryption>,
    pub updated_by: Option<String>,
    pub locker_fingerprint_id: Option<String>,
    pub payment_method_type_v2: Option<storage_enums::PaymentMethod>,
    pub payment_method_subtype: Option<storage_enums::PaymentMethodType>,
    pub id: common_utils::id_type::GlobalPaymentMethodId,
    pub version: common_enums::ApiVersion,
    pub network_token_requestor_reference_id: Option<String>,
    pub network_token_locker_id: Option<String>,
    pub network_token_payment_method_data: Option<Encryption>,
}

impl PaymentMethod {
    #[cfg(all(
        any(feature = "v1", feature = "v2"),
        not(feature = "payment_methods_v2")
    ))]
    pub fn get_id(&self) -> &String {
        &self.payment_method_id
    }

    #[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
    pub fn get_id(&self) -> &common_utils::id_type::GlobalPaymentMethodId {
        &self.id
    }
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
#[derive(
    Clone, Debug, Eq, PartialEq, Insertable, router_derive::DebugAsDisplay, Serialize, Deserialize,
)]
#[diesel(table_name = payment_methods)]
pub struct PaymentMethodNew {
    pub customer_id: common_utils::id_type::CustomerId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub payment_method_id: String,
    pub payment_method: Option<storage_enums::PaymentMethod>,
    pub payment_method_type: Option<storage_enums::PaymentMethodType>,
    pub payment_method_issuer: Option<String>,
    pub payment_method_issuer_code: Option<storage_enums::PaymentMethodIssuerCode>,
    pub accepted_currency: Option<Vec<storage_enums::Currency>>,
    pub scheme: Option<String>,
    pub token: Option<String>,
    pub cardholder_name: Option<Secret<String>>,
    pub issuer_name: Option<String>,
    pub issuer_country: Option<String>,
    pub payer_country: Option<Vec<String>>,
    pub is_stored: Option<bool>,
    pub swift_code: Option<String>,
    pub direct_debit_token: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub last_modified: PrimitiveDateTime,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub payment_method_data: Option<Encryption>,
    pub locker_id: Option<String>,
    pub last_used_at: PrimitiveDateTime,
    pub connector_mandate_details: Option<serde_json::Value>,
    pub customer_acceptance: Option<pii::SecretSerdeValue>,
    pub status: storage_enums::PaymentMethodStatus,
    pub network_transaction_id: Option<String>,
    pub client_secret: Option<String>,
    pub payment_method_billing_address: Option<Encryption>,
    pub updated_by: Option<String>,
    pub version: common_enums::ApiVersion,
    pub network_token_requestor_reference_id: Option<String>,
    pub network_token_locker_id: Option<String>,
    pub network_token_payment_method_data: Option<Encryption>,
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
#[derive(Clone, Debug, Insertable, router_derive::DebugAsDisplay, Serialize, Deserialize)]
#[diesel(table_name = payment_methods)]
pub struct PaymentMethodNew {
    pub customer_id: common_utils::id_type::CustomerId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub created_at: PrimitiveDateTime,
    pub last_modified: PrimitiveDateTime,
    pub payment_method_data: Option<Encryption>,
    pub locker_id: Option<String>,
    pub last_used_at: PrimitiveDateTime,
    pub connector_mandate_details: Option<PaymentsMandateReference>,
    pub customer_acceptance: Option<pii::SecretSerdeValue>,
    pub status: storage_enums::PaymentMethodStatus,
    pub network_transaction_id: Option<String>,
    pub client_secret: Option<String>,
    pub payment_method_billing_address: Option<Encryption>,
    pub updated_by: Option<String>,
    pub locker_fingerprint_id: Option<String>,
    pub payment_method_type_v2: Option<storage_enums::PaymentMethod>,
    pub payment_method_subtype: Option<storage_enums::PaymentMethodType>,
    pub id: common_utils::id_type::GlobalPaymentMethodId,
    pub version: common_enums::ApiVersion,
    pub network_token_requestor_reference_id: Option<String>,
    pub network_token_locker_id: Option<String>,
    pub network_token_payment_method_data: Option<Encryption>,
}

impl PaymentMethodNew {
    pub fn update_storage_scheme(&mut self, storage_scheme: MerchantStorageScheme) {
        self.updated_by = Some(storage_scheme.to_string());
    }

    #[cfg(all(
        any(feature = "v1", feature = "v2"),
        not(feature = "payment_methods_v2")
    ))]
    pub fn get_id(&self) -> &String {
        &self.payment_method_id
    }

    #[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
    pub fn get_id(&self) -> &common_utils::id_type::GlobalPaymentMethodId {
        &self.id
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TokenizeCoreWorkflow {
    pub lookup_key: String,
    pub pm: storage_enums::PaymentMethod,
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentMethodUpdate {
    MetadataUpdateAndLastUsed {
        metadata: Option<serde_json::Value>,
        last_used_at: PrimitiveDateTime,
    },
    UpdatePaymentMethodDataAndLastUsed {
        payment_method_data: Option<Encryption>,
        last_used_at: PrimitiveDateTime,
    },
    PaymentMethodDataUpdate {
        payment_method_data: Option<Encryption>,
    },
    LastUsedUpdate {
        last_used_at: PrimitiveDateTime,
    },
    NetworkTransactionIdAndStatusUpdate {
        network_transaction_id: Option<String>,
        status: Option<storage_enums::PaymentMethodStatus>,
    },
    StatusUpdate {
        status: Option<storage_enums::PaymentMethodStatus>,
    },
    AdditionalDataUpdate {
        payment_method_data: Option<Encryption>,
        status: Option<storage_enums::PaymentMethodStatus>,
        locker_id: Option<String>,
        payment_method: Option<storage_enums::PaymentMethod>,
        payment_method_type: Option<storage_enums::PaymentMethodType>,
        payment_method_issuer: Option<String>,
        network_token_requestor_reference_id: Option<String>,
        network_token_locker_id: Option<String>,
        network_token_payment_method_data: Option<Encryption>,
    },
    ConnectorMandateDetailsUpdate {
        connector_mandate_details: Option<serde_json::Value>,
    },
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentMethodUpdate {
    UpdatePaymentMethodDataAndLastUsed {
        payment_method_data: Option<Encryption>,
        last_used_at: PrimitiveDateTime,
    },
    PaymentMethodDataUpdate {
        payment_method_data: Option<Encryption>,
    },
    LastUsedUpdate {
        last_used_at: PrimitiveDateTime,
    },
    NetworkTransactionIdAndStatusUpdate {
        network_transaction_id: Option<String>,
        status: Option<storage_enums::PaymentMethodStatus>,
    },
    StatusUpdate {
        status: Option<storage_enums::PaymentMethodStatus>,
    },
    AdditionalDataUpdate {
        payment_method_data: Option<Encryption>,
        status: Option<storage_enums::PaymentMethodStatus>,
        locker_id: Option<String>,
        payment_method_type_v2: Option<storage_enums::PaymentMethod>,
        payment_method_subtype: Option<storage_enums::PaymentMethodType>,
        network_token_requestor_reference_id: Option<String>,
        network_token_locker_id: Option<String>,
        network_token_payment_method_data: Option<Encryption>,
    },
    ConnectorMandateDetailsUpdate {
        connector_mandate_details: Option<PaymentsMandateReference>,
    },
}

impl PaymentMethodUpdate {
    pub fn convert_to_payment_method_update(
        self,
        storage_scheme: MerchantStorageScheme,
    ) -> PaymentMethodUpdateInternal {
        let mut update_internal: PaymentMethodUpdateInternal = self.into();
        update_internal.updated_by = Some(storage_scheme.to_string());
        update_internal
    }
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay, Serialize, Deserialize)]
#[diesel(table_name = payment_methods)]
pub struct PaymentMethodUpdateInternal {
    payment_method_data: Option<Encryption>,
    last_used_at: Option<PrimitiveDateTime>,
    network_transaction_id: Option<String>,
    status: Option<storage_enums::PaymentMethodStatus>,
    locker_id: Option<String>,
    payment_method_type_v2: Option<storage_enums::PaymentMethod>,
    connector_mandate_details: Option<PaymentsMandateReference>,
    updated_by: Option<String>,
    payment_method_subtype: Option<storage_enums::PaymentMethodType>,
    last_modified: PrimitiveDateTime,
    network_token_requestor_reference_id: Option<String>,
    network_token_locker_id: Option<String>,
    network_token_payment_method_data: Option<Encryption>,
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
impl PaymentMethodUpdateInternal {
    pub fn apply_changeset(self, source: PaymentMethod) -> PaymentMethod {
        let Self {
            payment_method_data,
            last_used_at,
            network_transaction_id,
            status,
            locker_id,
            payment_method_type_v2,
            connector_mandate_details,
            updated_by,
            payment_method_subtype,
            last_modified,
            network_token_requestor_reference_id,
            network_token_locker_id,
            network_token_payment_method_data,
        } = self;

        PaymentMethod {
            customer_id: source.customer_id,
            merchant_id: source.merchant_id,
            created_at: source.created_at,
            last_modified,
            payment_method_data: payment_method_data.or(source.payment_method_data),
            locker_id: locker_id.or(source.locker_id),
            last_used_at: last_used_at.unwrap_or(source.last_used_at),
            connector_mandate_details: connector_mandate_details
                .or(source.connector_mandate_details),
            customer_acceptance: source.customer_acceptance,
            status: status.unwrap_or(source.status),
            network_transaction_id: network_transaction_id.or(source.network_transaction_id),
            client_secret: source.client_secret,
            payment_method_billing_address: source.payment_method_billing_address,
            updated_by: updated_by.or(source.updated_by),
            locker_fingerprint_id: source.locker_fingerprint_id,
            payment_method_type_v2: payment_method_type_v2.or(source.payment_method_type_v2),
            payment_method_subtype: payment_method_subtype.or(source.payment_method_subtype),
            id: source.id,
            version: source.version,
            network_token_requestor_reference_id: network_token_requestor_reference_id
                .or(source.network_token_requestor_reference_id),
            network_token_locker_id: network_token_locker_id.or(source.network_token_locker_id),
            network_token_payment_method_data: network_token_payment_method_data
                .or(source.network_token_payment_method_data),
        }
    }
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay, Serialize, Deserialize)]
#[diesel(table_name = payment_methods)]
pub struct PaymentMethodUpdateInternal {
    metadata: Option<serde_json::Value>,
    payment_method_data: Option<Encryption>,
    last_used_at: Option<PrimitiveDateTime>,
    network_transaction_id: Option<String>,
    status: Option<storage_enums::PaymentMethodStatus>,
    locker_id: Option<String>,
    network_token_requestor_reference_id: Option<String>,
    payment_method: Option<storage_enums::PaymentMethod>,
    connector_mandate_details: Option<serde_json::Value>,
    updated_by: Option<String>,
    payment_method_type: Option<storage_enums::PaymentMethodType>,
    payment_method_issuer: Option<String>,
    last_modified: PrimitiveDateTime,
    network_token_locker_id: Option<String>,
    network_token_payment_method_data: Option<Encryption>,
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
impl PaymentMethodUpdateInternal {
    pub fn apply_changeset(self, source: PaymentMethod) -> PaymentMethod {
        let Self {
            metadata,
            payment_method_data,
            last_used_at,
            network_transaction_id,
            status,
            locker_id,
            network_token_requestor_reference_id,
            payment_method,
            connector_mandate_details,
            updated_by,
            payment_method_type,
            payment_method_issuer,
            last_modified,
            network_token_locker_id,
            network_token_payment_method_data,
        } = self;

        PaymentMethod {
            customer_id: source.customer_id,
            merchant_id: source.merchant_id,
            payment_method_id: source.payment_method_id,
            accepted_currency: source.accepted_currency,
            scheme: source.scheme,
            token: source.token,
            cardholder_name: source.cardholder_name,
            issuer_name: source.issuer_name,
            issuer_country: source.issuer_country,
            payer_country: source.payer_country,
            is_stored: source.is_stored,
            swift_code: source.swift_code,
            direct_debit_token: source.direct_debit_token,
            created_at: source.created_at,
            last_modified,
            payment_method: payment_method.or(source.payment_method),
            payment_method_type: payment_method_type.or(source.payment_method_type),
            payment_method_issuer: payment_method_issuer.or(source.payment_method_issuer),
            payment_method_issuer_code: source.payment_method_issuer_code,
            metadata: metadata.map_or(source.metadata, |v| Some(v.into())),
            payment_method_data: payment_method_data.or(source.payment_method_data),
            locker_id: locker_id.or(source.locker_id),
            last_used_at: last_used_at.unwrap_or(source.last_used_at),
            connector_mandate_details: connector_mandate_details
                .or(source.connector_mandate_details),
            customer_acceptance: source.customer_acceptance,
            status: status.unwrap_or(source.status),
            network_transaction_id: network_transaction_id.or(source.network_transaction_id),
            client_secret: source.client_secret,
            payment_method_billing_address: source.payment_method_billing_address,
            updated_by: updated_by.or(source.updated_by),
            version: source.version,
            network_token_requestor_reference_id: network_token_requestor_reference_id
                .or(source.network_token_requestor_reference_id),
            network_token_locker_id: network_token_locker_id.or(source.network_token_locker_id),
            network_token_payment_method_data: network_token_payment_method_data
                .or(source.network_token_payment_method_data),
        }
    }
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
impl From<PaymentMethodUpdate> for PaymentMethodUpdateInternal {
    fn from(payment_method_update: PaymentMethodUpdate) -> Self {
        match payment_method_update {
            PaymentMethodUpdate::MetadataUpdateAndLastUsed {
                metadata,
                last_used_at,
            } => Self {
                metadata,
                payment_method_data: None,
                last_used_at: Some(last_used_at),
                network_transaction_id: None,
                status: None,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::PaymentMethodDataUpdate {
                payment_method_data,
            } => Self {
                metadata: None,
                payment_method_data,
                last_used_at: None,
                network_transaction_id: None,
                status: None,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::LastUsedUpdate { last_used_at } => Self {
                metadata: None,
                payment_method_data: None,
                last_used_at: Some(last_used_at),
                network_transaction_id: None,
                status: None,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::UpdatePaymentMethodDataAndLastUsed {
                payment_method_data,
                last_used_at,
            } => Self {
                metadata: None,
                payment_method_data,
                last_used_at: Some(last_used_at),
                network_transaction_id: None,
                status: None,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::NetworkTransactionIdAndStatusUpdate {
                network_transaction_id,
                status,
            } => Self {
                metadata: None,
                payment_method_data: None,
                last_used_at: None,
                network_transaction_id,
                status,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::StatusUpdate { status } => Self {
                metadata: None,
                payment_method_data: None,
                last_used_at: None,
                network_transaction_id: None,
                status,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::AdditionalDataUpdate {
                payment_method_data,
                status,
                locker_id,
                network_token_requestor_reference_id,
                payment_method,
                payment_method_type,
                payment_method_issuer,
                network_token_locker_id,
                network_token_payment_method_data,
            } => Self {
                metadata: None,
                payment_method_data,
                last_used_at: None,
                network_transaction_id: None,
                status,
                locker_id,
                network_token_requestor_reference_id,
                payment_method,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_issuer,
                payment_method_type,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id,
                network_token_payment_method_data,
            },
            PaymentMethodUpdate::ConnectorMandateDetailsUpdate {
                connector_mandate_details,
            } => Self {
                metadata: None,
                payment_method_data: None,
                last_used_at: None,
                status: None,
                locker_id: None,
                network_token_requestor_reference_id: None,
                payment_method: None,
                connector_mandate_details,
                network_transaction_id: None,
                updated_by: None,
                payment_method_issuer: None,
                payment_method_type: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_payment_method_data: None,
            },
        }
    }
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
impl From<PaymentMethodUpdate> for PaymentMethodUpdateInternal {
    fn from(payment_method_update: PaymentMethodUpdate) -> Self {
        match payment_method_update {
            PaymentMethodUpdate::PaymentMethodDataUpdate {
                payment_method_data,
            } => Self {
                payment_method_data,
                last_used_at: None,
                network_transaction_id: None,
                status: None,
                locker_id: None,
                payment_method_type_v2: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_subtype: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_requestor_reference_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::LastUsedUpdate { last_used_at } => Self {
                payment_method_data: None,
                last_used_at: Some(last_used_at),
                network_transaction_id: None,
                status: None,
                locker_id: None,
                payment_method_type_v2: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_subtype: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_requestor_reference_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::UpdatePaymentMethodDataAndLastUsed {
                payment_method_data,
                last_used_at,
            } => Self {
                payment_method_data,
                last_used_at: Some(last_used_at),
                network_transaction_id: None,
                status: None,
                locker_id: None,
                payment_method_type_v2: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_subtype: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_requestor_reference_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::NetworkTransactionIdAndStatusUpdate {
                network_transaction_id,
                status,
            } => Self {
                payment_method_data: None,
                last_used_at: None,
                network_transaction_id,
                status,
                locker_id: None,
                payment_method_type_v2: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_subtype: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_requestor_reference_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::StatusUpdate { status } => Self {
                payment_method_data: None,
                last_used_at: None,
                network_transaction_id: None,
                status,
                locker_id: None,
                payment_method_type_v2: None,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_subtype: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_requestor_reference_id: None,
                network_token_payment_method_data: None,
            },
            PaymentMethodUpdate::AdditionalDataUpdate {
                payment_method_data,
                status,
                locker_id,
                payment_method_type_v2,
                payment_method_subtype,
                network_token_requestor_reference_id,
                network_token_locker_id,
                network_token_payment_method_data,
            } => Self {
                payment_method_data,
                last_used_at: None,
                network_transaction_id: None,
                status,
                locker_id,
                payment_method_type_v2,
                connector_mandate_details: None,
                updated_by: None,
                payment_method_subtype,
                last_modified: common_utils::date_time::now(),
                network_token_requestor_reference_id,
                network_token_locker_id,
                network_token_payment_method_data,
            },
            PaymentMethodUpdate::ConnectorMandateDetailsUpdate {
                connector_mandate_details,
            } => Self {
                payment_method_data: None,
                last_used_at: None,
                status: None,
                locker_id: None,
                payment_method_type_v2: None,
                connector_mandate_details,
                network_transaction_id: None,
                updated_by: None,
                payment_method_subtype: None,
                last_modified: common_utils::date_time::now(),
                network_token_locker_id: None,
                network_token_requestor_reference_id: None,
                network_token_payment_method_data: None,
            },
        }
    }
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
impl From<&PaymentMethodNew> for PaymentMethod {
    fn from(payment_method_new: &PaymentMethodNew) -> Self {
        Self {
            customer_id: payment_method_new.customer_id.clone(),
            merchant_id: payment_method_new.merchant_id.clone(),
            payment_method_id: payment_method_new.payment_method_id.clone(),
            locker_id: payment_method_new.locker_id.clone(),
            network_token_requestor_reference_id: payment_method_new
                .network_token_requestor_reference_id
                .clone(),
            accepted_currency: payment_method_new.accepted_currency.clone(),
            scheme: payment_method_new.scheme.clone(),
            token: payment_method_new.token.clone(),
            cardholder_name: payment_method_new.cardholder_name.clone(),
            issuer_name: payment_method_new.issuer_name.clone(),
            issuer_country: payment_method_new.issuer_country.clone(),
            payer_country: payment_method_new.payer_country.clone(),
            is_stored: payment_method_new.is_stored,
            swift_code: payment_method_new.swift_code.clone(),
            direct_debit_token: payment_method_new.direct_debit_token.clone(),
            created_at: payment_method_new.created_at,
            last_modified: payment_method_new.last_modified,
            payment_method: payment_method_new.payment_method,
            payment_method_type: payment_method_new.payment_method_type,
            payment_method_issuer: payment_method_new.payment_method_issuer.clone(),
            payment_method_issuer_code: payment_method_new.payment_method_issuer_code,
            metadata: payment_method_new.metadata.clone(),
            payment_method_data: payment_method_new.payment_method_data.clone(),
            last_used_at: payment_method_new.last_used_at,
            connector_mandate_details: payment_method_new.connector_mandate_details.clone(),
            customer_acceptance: payment_method_new.customer_acceptance.clone(),
            status: payment_method_new.status,
            network_transaction_id: payment_method_new.network_transaction_id.clone(),
            client_secret: payment_method_new.client_secret.clone(),
            updated_by: payment_method_new.updated_by.clone(),
            payment_method_billing_address: payment_method_new
                .payment_method_billing_address
                .clone(),
            version: payment_method_new.version,
            network_token_locker_id: payment_method_new.network_token_locker_id.clone(),
            network_token_payment_method_data: payment_method_new
                .network_token_payment_method_data
                .clone(),
        }
    }
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
impl From<&PaymentMethodNew> for PaymentMethod {
    fn from(payment_method_new: &PaymentMethodNew) -> Self {
        Self {
            customer_id: payment_method_new.customer_id.clone(),
            merchant_id: payment_method_new.merchant_id.clone(),
            locker_id: payment_method_new.locker_id.clone(),
            created_at: payment_method_new.created_at,
            last_modified: payment_method_new.last_modified,
            payment_method_data: payment_method_new.payment_method_data.clone(),
            last_used_at: payment_method_new.last_used_at,
            connector_mandate_details: payment_method_new.connector_mandate_details.clone(),
            customer_acceptance: payment_method_new.customer_acceptance.clone(),
            status: payment_method_new.status,
            network_transaction_id: payment_method_new.network_transaction_id.clone(),
            client_secret: payment_method_new.client_secret.clone(),
            updated_by: payment_method_new.updated_by.clone(),
            payment_method_billing_address: payment_method_new
                .payment_method_billing_address
                .clone(),
            locker_fingerprint_id: payment_method_new.locker_fingerprint_id.clone(),
            payment_method_type_v2: payment_method_new.payment_method_type_v2,
            payment_method_subtype: payment_method_new.payment_method_subtype,
            id: payment_method_new.id.clone(),
            version: payment_method_new.version,
            network_token_requestor_reference_id: payment_method_new
                .network_token_requestor_reference_id
                .clone(),
            network_token_locker_id: payment_method_new.network_token_locker_id.clone(),
            network_token_payment_method_data: payment_method_new
                .network_token_payment_method_data
                .clone(),
        }
    }
}

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "payment_methods_v2")
))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentsMandateReferenceRecord {
    pub connector_mandate_id: String,
    pub payment_method_type: Option<common_enums::PaymentMethodType>,
    pub original_payment_authorized_amount: Option<i64>,
    pub original_payment_authorized_currency: Option<common_enums::Currency>,
    pub mandate_metadata: Option<pii::SecretSerdeValue>,
    pub connector_mandate_status: Option<common_enums::ConnectorMandateStatus>,
    pub connector_mandate_request_reference_id: Option<String>,
}

#[cfg(all(feature = "v2", feature = "payment_methods_v2"))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentsMandateReferenceRecord {
    pub connector_mandate_id: String,
    pub payment_method_subtype: Option<common_enums::PaymentMethodType>,
    pub original_payment_authorized_amount: Option<i64>,
    pub original_payment_authorized_currency: Option<common_enums::Currency>,
    pub mandate_metadata: Option<pii::SecretSerdeValue>,
    pub connector_mandate_status: Option<common_enums::ConnectorMandateStatus>,
    pub connector_mandate_request_reference_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct PaymentsMandateReference(
    pub HashMap<common_utils::id_type::MerchantConnectorAccountId, PaymentsMandateReferenceRecord>,
);

impl std::ops::Deref for PaymentsMandateReference {
    type Target =
        HashMap<common_utils::id_type::MerchantConnectorAccountId, PaymentsMandateReferenceRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PaymentsMandateReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

common_utils::impl_to_sql_from_sql_json!(PaymentsMandateReference);
