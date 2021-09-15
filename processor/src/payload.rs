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
// -----------------------------------------------------------------------------

use sawtooth_sdk::processor::handler::ApplyError;

use bond_common::proto::{
    payload, payload_bond, payload_holding, payload_order, payload_organization,
    payload_participant, payload_quote, payload_receipt, payload_settlement,
};
use protobuf;

fn unpack_data<M: protobuf::Message>(bytes: &[u8]) -> Result<M, ApplyError> {
    protobuf::parse_from_bytes(bytes).map_err(|err| {
        ApplyError::InvalidTransaction(format!("Failed to deserialize protobuf: {:?}", err))
    })
}

#[derive(Debug, Clone)]
pub enum Action {
    CreateParticipant(payload_participant::CreateParticipant),
    UpdateParticipant(payload_participant::UpdateParticipant),
    CreateOrganization(payload_organization::CreateOrganization),
    UpdateOrganization(payload_organization::UpdateOrganization),
    UpdateOrganizationAuthorization(payload_organization::UpdateOrganizationAuthorization),
    CreateOrder(payload_order::CreateOrder),
    CreateQuote(payload_quote::CreateQuote),
    CreateSettlement(payload_settlement::CreateSettlement),
    CreateReceipt(payload_receipt::CreateReceipt),
    CreateHolding(payload_holding::CreateHolding),
    DeleteHolding(payload_holding::DeleteHolding),
    CreateBond(payload_bond::CreateBond),
}

pub struct BondPayload {
    action: Action,
}

impl BondPayload {
    pub fn new(payload: &[u8]) -> Result<Option<BondPayload>, ApplyError> {
        let payload: payload::BondPayload = unpack_data(payload)?;
        let bond_action = payload.get_action();

        let action = match bond_action {
            payload::BondPayload_Action::CREATE_PARTICIPANT => {
                let create_participant: payload_participant::CreateParticipant =
                    unpack_data(payload.get_content())?;

                Action::CreateParticipant(create_participant.clone())
            }

            payload::BondPayload_Action::UPDATE_PARTICIPANT => {
                let update_participant: payload_participant::UpdateParticipant =
                    unpack_data(payload.get_content())?;

                Action::UpdateParticipant(update_participant.clone())
            }

            payload::BondPayload_Action::CREATE_ORGANIZATION => {
                let create_organization: payload_organization::CreateOrganization =
                    unpack_data(payload.get_content())?;

                Action::CreateOrganization(create_organization.clone())
            }

            payload::BondPayload_Action::UPDATE_ORGANIZATION => {
                let update_organization: payload_organization::UpdateOrganization =
                    unpack_data(payload.get_content())?;

                Action::UpdateOrganization(update_organization.clone())
            }

            payload::BondPayload_Action::UPDATE_ORGANIZATION_AUTHORIZATION => {
                let update_organization_authorization:
                    payload_organization::UpdateOrganizationAuthorization
                    = unpack_data(payload.get_content())?;

                Action::UpdateOrganizationAuthorization(update_organization_authorization.clone())
            }

            payload::BondPayload_Action::CREATE_ORDER => {
                let create_order: payload_order::CreateOrder = unpack_data(payload.get_content())?;

                Action::CreateOrder(create_order.clone())
            }

            payload::BondPayload_Action::CREATE_QUOTE => {
                let create_quote: payload_quote::CreateQuote = unpack_data(payload.get_content())?;

                Action::CreateQuote(create_quote.clone())
            }

            payload::BondPayload_Action::CREATE_SETTLEMENT => {
                let create_settlement: payload_settlement::CreateSettlement =
                    unpack_data(payload.get_content())?;

                Action::CreateSettlement(create_settlement.clone())
            }

            payload::BondPayload_Action::CREATE_RECEIPT => {
                let create_receipt: payload_receipt::CreateReceipt =
                    unpack_data(payload.get_content())?;

                Action::CreateReceipt(create_receipt.clone())
            }

            payload::BondPayload_Action::CREATE_HOLDING => {
                let create_holding: payload_holding::CreateHolding =
                    unpack_data(payload.get_content())?;

                Action::CreateHolding(create_holding.clone())
            }

            payload::BondPayload_Action::DELETE_HOLDING => {
                let delete_holding: payload_holding::DeleteHolding =
                    unpack_data(payload.get_content())?;

                Action::DeleteHolding(delete_holding.clone())
            }

            payload::BondPayload_Action::CREATE_BOND => {
                let create_bond: payload_bond::CreateBond = unpack_data(payload.get_content())?;

                Action::CreateBond(create_bond.clone())
            }
        };

        Ok(Some(BondPayload { action: action }))
    }

    pub fn get_action(&self) -> Action {
        self.action.clone()
    }
}
