use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use std::result::Result;

// use crate::{error::BalancedDollarError, network_address::NetworkAddress};

pub const CROSS_TRANSFER: &str = "CrossTransfer";
pub const CROSS_TRANSFER_REVERT: &str = "CrossTransferRevert";

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug, PartialEq, Clone)]
pub struct NetworkAddress {
    pub net: String,
    pub account: String,
}

impl NetworkAddress {
    pub fn new(net: String, account: String) -> Self {
        Self { net, account }
    }

    pub fn from_str(value: &String) -> Self {
        let mut iter = value.split('/');
        NetworkAddress {
            net: iter.next().unwrap_or("").to_string(),
            account: iter.next().unwrap_or("").to_string(),
        }
    }

    pub fn parse_networrk_address(&self) -> (&String, &String)
    {
        (&self.net, &self.account)
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", &self.net, &self.account)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug, PartialEq, Clone)]
pub struct CrossTransferMessage {
    pub method: String,
    pub from: NetworkAddress,
    pub to: NetworkAddress,
    pub value: u128,
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug, PartialEq, Clone)]
pub struct CrossTransferRevert {
    pub method: String,
    pub from: Pubkey,
    pub value: u128,
}

impl Encodable for CrossTransferMessage {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream
            .begin_list(5)
            .append(&self.method)
            .append(&self.from.to_string())
            .append(&self.to.to_string())
            .append(&self.value)
            .append(&self.data);
    }
}

impl Decodable for CrossTransferMessage {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let from: String = rlp.val_at(1)?;
        let to: String = rlp.val_at(2)?;
        Ok(Self {
            method: rlp.val_at(0)?,
            from: NetworkAddress::from_str(&from),
            to: NetworkAddress::from_str(&to),
            value: rlp.val_at(3)?,
            data: rlp.val_at(4)?,
        })
    }
}

impl Encodable for CrossTransferRevert {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream
            .begin_list(3)
            .append(&self.method)
            .append(&self.from.to_string())
            .append(&self.value);
    }
}

impl Decodable for CrossTransferRevert {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let from: String = rlp.val_at(1)?;
        Ok(Self {
            method: rlp.val_at(0)?,
            from: Pubkey::from_str(&from).unwrap(),
            value: rlp.val_at(2)?,
        })
    }
}
