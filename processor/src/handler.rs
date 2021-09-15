// Copyright 2018 Bitwise IO
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

use bond_common::addressing::*;

use bond_common::proto::{
    payload_bond, payload_holding, payload_order, payload_organization, payload_participant,
    payload_quote, payload_receipt, payload_settlement,
};

use payload::{Action, BondPayload};
use state::BondState;

pub struct BondTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl BondTransactionHandler {
    pub fn new() -> BondTransactionHandler {
        BondTransactionHandler {
            family_name: "bond".to_string(),
            family_versions: vec!["0.1".to_string()],
            namespaces: vec![get_bond_namespace()],
        }
    }

    fn _create_participant(
        &self,
        _payload: payload_participant::CreateParticipant,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _update_participant(
        &self,
        _payload: payload_participant::UpdateParticipant,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_organization(
        &self,
        _payload: payload_organization::CreateOrganization,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _update_organization(
        &self,
        _payload: payload_organization::UpdateOrganization,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _update_organization_authorization(
        &self,
        _payload: payload_organization::UpdateOrganizationAuthorization,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_order(
        &self,
        _payload: payload_order::CreateOrder,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_quote(
        &self,
        _payload: payload_quote::CreateQuote,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_holding(
        &self,
        _payload: payload_holding::CreateHolding,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _delete_holding(
        &self,
        _payload: payload_holding::DeleteHolding,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_settlement(
        &self,
        _payload: payload_settlement::CreateSettlement,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_receipt(
        &self,
        _payload: payload_receipt::CreateReceipt,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }

    fn _create_bond(
        &self,
        _payload: payload_bond::CreateBond,
        mut _state: BondState,
        _signer: &str,
    ) -> Result<(), ApplyError> {
        unimplemented!();
    }
}

impl TransactionHandler for BondTransactionHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut TransactionContext,
    ) -> Result<(), ApplyError> {
        let payload = BondPayload::new(request.get_payload());

        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };

        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )))
            }
        };

        let signer = request.get_header().get_signer_public_key();
        let state = BondState::new(context);

        info!(
            "Payload: {:?} {} {}",
            payload.get_action(),
            request.get_header().get_inputs()[0],
            request.get_header().get_outputs()[0]
        );

        match payload.get_action() {
            Action::CreateParticipant(create_participant_payload) => {
                self._create_participant(create_participant_payload, state, signer)?
            }

            Action::UpdateParticipant(update_participant_payload) => {
                self._update_participant(update_participant_payload, state, signer)?
            }

            Action::CreateOrganization(create_organization_payload) => {
                self._create_organization(create_organization_payload, state, signer)?
            }

            Action::UpdateOrganization(update_organization_payload) => {
                self._update_organization(update_organization_payload, state, signer)?
            }

            Action::UpdateOrganizationAuthorization(update_organization_authorization_payload) => {
                self._update_organization_authorization(
                    update_organization_authorization_payload,
                    state,
                    signer,
                )?
            }

            Action::CreateOrder(create_order_payload) => {
                self._create_order(create_order_payload, state, signer)?
            }

            Action::CreateQuote(create_quote_payload) => {
                self._create_quote(create_quote_payload, state, signer)?
            }

            Action::CreateSettlement(create_settlement_payload) => {
                self._create_settlement(create_settlement_payload, state, signer)?
            }

            Action::CreateReceipt(create_receipt_payload) => {
                self._create_receipt(create_receipt_payload, state, signer)?
            }

            Action::CreateHolding(create_holding_payload) => {
                self._create_holding(create_holding_payload, state, signer)?
            }

            Action::DeleteHolding(delete_holding_payload) => {
                self._delete_holding(delete_holding_payload, state, signer)?
            }

            Action::CreateBond(create_bond_payload) => {
                self._create_bond(create_bond_payload, state, signer)?
            }
        }

        Ok(())
    }
}
